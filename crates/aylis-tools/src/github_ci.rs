//! A compilação do app de iOS no GitHub Actions, sem Mac: `aylis-tool ci-ios <projeto>` grava
//! `.github/workflows/ios-<app>.yml` no repositório do projeto.
//!
//! A máquina macOS do GitHub já tem o Xcode e o Rust. O workflow baixa o projeto e o Aylis
//! (na versão deste `aylis-tool`), compila o `aylis-tool`, gera o código do projeto (o que
//! liga o `<projeto>/.aylis/sdk` ao Aylis baixado) e publica o `.ipa`, que fica nos
//! resultados da execução (Actions → a execução → Artifacts).
//!
//! - Sem assinatura (o padrão): o `.ipa` sai sem a assinatura da Apple, e o Sideloadly, no
//!   Windows, assina com o Apple ID de quem instala (a conta gratuita serve; o app vale 7
//!   dias).
//! - Com `--signed`: o certificado (`.p12`, em base64), a senha dele e o perfil
//!   (`.mobileprovision`, em base64) vêm dos segredos do repositório ([`SECRETS`]) para um
//!   Keychain temporário, e o `.ipa` sai assinado. Nunca no arquivo do workflow nem no
//!   repositório.
//!
//! Roda só quando pedido (Actions → o workflow → Run workflow): em repositório privado, o
//! minuto de macOS conta dez vezes.

use std::path::{Path, PathBuf};

use aylis_project::{Language, Project};

/// Os segredos do workflow assinado: (nome, o que é).
pub const SECRETS: &[(&str, &str)] = &[
    ("IOS_CERTIFICATE_P12", "o certificado exportado do Keychain (.p12), em base64"),
    ("IOS_CERTIFICATE_PASSWORD", "a senha do .p12"),
    ("IOS_PROFILE", "o perfil de provisionamento (.mobileprovision), em base64"),
];

/// O repositório do Aylis que o workflow baixa.
pub const AYLIS_REPOSITORY: &str = "e-rust/aylis";

/// A pasta do repositório git que contém `dir` (a que tem o `.git`).
pub fn git_root(dir: &Path) -> Option<PathBuf> {
    dir.ancestors().find(|d| d.join(".git").exists()).map(Path::to_path_buf)
}

/// A tag do Aylis que o workflow baixa: a desta versão.
pub fn aylis_ref() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

/// Onde fica o workflow do projeto (existindo ou não); `None` fora de um repositório git.
pub fn workflow_path(project: &Project) -> Option<PathBuf> {
    let root = git_root(&project.dir())?;
    Some(root.join(".github").join("workflows").join(format!("ios-{}.yml", project.package_name())))
}

/// O workflow gravado assina com os segredos (o `--signed`).
pub fn is_signed(text: &str) -> bool {
    text.contains("secrets.IOS_CERTIFICATE_P12")
}

/// A tag do Aylis que um workflow gravado baixa (`v0.7.0`).
pub fn workflow_aylis_ref(text: &str) -> Option<String> {
    text.lines().find_map(|l| l.trim().strip_prefix("ref: ")).map(|r| r.trim().to_string())
}

/// O repositório do GitHub (`dono/nome`) do `origin` do repositório em `root`, lido do
/// `.git/config` (sem rodar o git).
pub fn github_repository(root: &Path) -> Option<String> {
    let config = std::fs::read_to_string(root.join(".git").join("config")).ok()?;
    github_repository_in(&config)
}

/// O `dono/nome` do `origin` num `.git/config`: `https://github.com/dono/nome(.git)`,
/// `https://usuario@github.com/...` ou `git@github.com:dono/nome.git`.
pub fn github_repository_in(config: &str) -> Option<String> {
    let mut in_origin = false;
    for line in config.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_origin = line == "[remote \"origin\"]";
            continue;
        }
        if !in_origin {
            continue;
        }
        let Some(url) = line.strip_prefix("url").map(str::trim_start).and_then(|l| l.strip_prefix('=')).map(str::trim) else { continue };
        let rest = url.split_once("github.com").map(|(_, r)| r)?;
        let rest = rest.trim_start_matches([':', '/']).trim_end_matches('/').trim_end_matches(".git");
        let mut parts = rest.split('/');
        let (owner, name) = (parts.next()?, parts.next()?);
        return (!owner.is_empty() && !name.is_empty()).then(|| format!("{owner}/{name}"));
    }
    None
}

/// A página das execuções do workflow no GitHub (o "Run workflow" fica nela).
pub fn actions_url(repository: &str, workflow: &Path) -> String {
    let file = workflow.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    format!("https://github.com/{repository}/actions/workflows/{file}")
}

/// A página dos segredos do repositório.
pub fn secrets_url(repository: &str) -> String {
    format!("https://github.com/{repository}/settings/secrets/actions")
}

/// Um arquivo em base64 (o valor dos segredos `IOS_CERTIFICATE_P12` e `IOS_PROFILE`).
pub fn base64_file(path: &Path) -> Result<String, String> {
    use base64::Engine;
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

/// Grava o workflow do projeto e devolve o caminho dele.
pub fn write_ios_workflow(project: &Project, signed: bool) -> Result<PathBuf, String> {
    if project.language != Language::Rust {
        return Err(format!("projetos em {} ainda não publicam para o iOS", project.language.name()));
    }
    let dir = project.dir();
    let root = git_root(&dir).ok_or_else(|| format!("{} não está num repositório git (o workflow fica no repositório)", dir.display()))?;
    let relative = project.path.strip_prefix(&root).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
    let package = project.package_name();
    let text = ios_workflow(&relative, project.app_name(), &package, &aylis_ref(), signed);
    let path = workflow_path(project).unwrap_or_else(|| root.join(".github").join("workflows").join(format!("ios-{package}.yml")));
    std::fs::create_dir_all(path.parent().unwrap_or(&root)).map_err(|e| e.to_string())?;
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

/// O texto do workflow: `project` é o `.aylisproj` relativo à raiz do repositório,
/// `aylis_ref` a tag do Aylis (`v0.7.0`).
pub fn ios_workflow(project: &str, app_name: &str, package: &str, aylis_ref: &str, signed: bool) -> String {
    let project_dir = project.rsplit_once('/').map(|(d, _)| d).unwrap_or(".");
    let dist = if project_dir == "." { "app/dist/ios".to_string() } else { format!("app/{project_dir}/dist/ios") };
    let title = app_name.replace(['"', '\\'], "");
    let mut steps = String::new();
    if signed {
        steps.push_str(SIGNING_STEP);
    }
    let sign_flag = if signed { "" } else { " --no-sign" };
    let header = if signed {
        "# O .ipa sai assinado com o certificado e o perfil dos segredos do repositório\n# (Settings → Secrets and variables → Actions): IOS_CERTIFICATE_P12 (o .p12 em base64),\n# IOS_CERTIFICATE_PASSWORD e IOS_PROFILE (o .mobileprovision em base64).\n"
    } else {
        "# O .ipa sai sem a assinatura da Apple: no Windows, o Sideloadly assina com o Apple ID de\n# quem instala (a conta gratuita serve; o app vale 7 dias) e instala no iPhone pelo cabo.\n"
    };
    TEMPLATE
        .replace("{header}", header)
        .replace("{title}", &title)
        .replace("{aylis_ref}", aylis_ref)
        .replace("{aylis_repository}", AYLIS_REPOSITORY)
        .replace("{signing}", &steps)
        .replace("{project}", project)
        .replace("{sign_flag}", sign_flag)
        .replace("{package}", package)
        .replace("{dist}", &dist)
}

const TEMPLATE: &str = r#"# Gerado pelo Aylis (aylis-tool ci-ios): o app de iOS compilado numa máquina macOS do GitHub.
# Rode em Actions → este workflow → Run workflow; o .ipa fica nos Artifacts da execução.
{header}#
# Em repositório privado, o minuto de macOS conta dez vezes no plano: por isso roda só quando
# pedido. Para rodar a cada tag de versão, acrescente em `on:`
#   push:
#     tags: ["v*"]
name: "iOS: {title}"

on:
  workflow_dispatch:

env:
  CARGO_TARGET_DIR: ${{ github.workspace }}/target
  CARGO_TERM_COLOR: always

jobs:
  ios:
    runs-on: macos-latest
    timeout-minutes: 90
    steps:
      - name: O projeto
        uses: actions/checkout@v4
        with:
          path: app

      - name: O Aylis ({aylis_ref})
        uses: actions/checkout@v4
        with:
          repository: {aylis_repository}
          ref: {aylis_ref}
          path: aylis

      - name: O Rust para o iPhone
        run: rustup target add aarch64-apple-ios

      - name: O cache da compilação
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: aylis-ios-{aylis_ref}-${{ hashFiles('app/**/Cargo.lock') }}
          restore-keys: aylis-ios-{aylis_ref}-

      - name: O aylis-tool
        working-directory: aylis
        run: cargo build -p aylis-tools --bin aylis-tool
{signing}
      - name: Gerar o código e publicar
        run: |
          "$CARGO_TARGET_DIR/debug/aylis-tool" generate "app/{project}"
          "$CARGO_TARGET_DIR/debug/aylis-tool" publish "app/{project}" --target ios --device{sign_flag}

      - name: O .ipa
        uses: actions/upload-artifact@v4
        with:
          name: {package}-ios
          path: {dist}/*.ipa
          if-no-files-found: error
"#;

/// O certificado e o perfil dos segredos para um Keychain temporário (apagado com a máquina).
const SIGNING_STEP: &str = r#"
      - name: O certificado e o perfil (dos segredos)
        env:
          P12: ${{ secrets.IOS_CERTIFICATE_P12 }}
          P12_PASSWORD: ${{ secrets.IOS_CERTIFICATE_PASSWORD }}
          PROFILE: ${{ secrets.IOS_PROFILE }}
        run: |
          KEYCHAIN="$RUNNER_TEMP/aylis.keychain-db"
          KEYCHAIN_PASSWORD="$(uuidgen)"
          security create-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
          security set-keychain-settings -lut 21600 "$KEYCHAIN"
          security unlock-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
          echo "$P12" | base64 --decode > "$RUNNER_TEMP/cert.p12"
          security import "$RUNNER_TEMP/cert.p12" -P "$P12_PASSWORD" -A -t cert -f pkcs12 -k "$KEYCHAIN"
          rm "$RUNNER_TEMP/cert.p12"
          security set-key-partition-list -S apple-tool:,apple: -k "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
          security list-keychains -d user -s "$KEYCHAIN" $(security list-keychains -d user | tr -d '"')
          echo "$PROFILE" | base64 --decode > "$RUNNER_TEMP/app.mobileprovision"
          echo "AYLIS_IOS_PROFILE=$RUNNER_TEMP/app.mobileprovision" >> "$GITHUB_ENV"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unsigned_workflow_builds_the_ipa_for_sideloadly() {
        let text = ios_workflow("apps/Loja/Loja.aylisproj", "Loja \"Central\"", "loja", "v0.7.0", false);
        assert!(text.contains("name: \"iOS: Loja Central\""), "{text}");
        assert!(text.contains("repository: e-rust/aylis\n          ref: v0.7.0"));
        assert!(text.contains("generate \"app/apps/Loja/Loja.aylisproj\""));
        assert!(text.contains("publish \"app/apps/Loja/Loja.aylisproj\" --target ios --device --no-sign"));
        assert!(text.contains("path: app/apps/Loja/dist/ios/*.ipa"));
        assert!(!text.contains("secrets."), "sem segredos no workflow sem assinatura");
        for marker in ["{project}", "{header}", "{title}", "{aylis_ref}", "{signing}", "{sign_flag}", "{package}", "{dist}"] {
            assert!(!text.contains(marker), "{marker} sobrando");
        }
        assert!(text.contains("workflow_dispatch"));
    }

    #[test]
    fn the_signed_workflow_takes_the_certificate_from_the_secrets() {
        let text = ios_workflow("Loja.aylisproj", "Loja", "loja", "v0.7.0", true);
        for (secret, _) in SECRETS {
            assert!(text.contains(&format!("${{{{ secrets.{secret} }}}}")), "{secret}");
        }
        assert!(text.contains("AYLIS_IOS_PROFILE="));
        assert!(text.contains("--target ios --device\n"), "assinado: sem --no-sign");
        assert!(text.contains("path: app/dist/ios/*.ipa"), "o projeto na raiz do repositório");
        // A ordem: o Keychain antes de publicar.
        assert!(text.find("security import").unwrap() < text.find("publish \"app/").unwrap());
    }

    #[test]
    fn the_github_repository_comes_from_the_origin() {
        let config = "[core]\n\trepositoryformatversion = 0\n[remote \"upstream\"]\n\turl = https://github.com/outro/coisa.git\n[remote \"origin\"]\n\turl = https://fulano@github.com/e-rust/loja.git\n\tfetch = +refs/heads/*:refs/remotes/origin/*\n";
        assert_eq!(github_repository_in(config).as_deref(), Some("e-rust/loja"));
        assert_eq!(github_repository_in("[remote \"origin\"]\n url = git@github.com:dono/app.git\n").as_deref(), Some("dono/app"));
        assert_eq!(github_repository_in("[remote \"origin\"]\n url = https://github.com/dono/app/\n").as_deref(), Some("dono/app"));
        assert_eq!(github_repository_in("[remote \"origin\"]\n url = https://gitlab.com/dono/app.git\n"), None);
        assert_eq!(github_repository_in("[core]\n"), None);
        assert_eq!(actions_url("dono/app", Path::new("x/.github/workflows/ios-loja.yml")), "https://github.com/dono/app/actions/workflows/ios-loja.yml");
        assert_eq!(secrets_url("dono/app"), "https://github.com/dono/app/settings/secrets/actions");
        let text = ios_workflow("Loja.aylisproj", "Loja", "loja", "v9.9.9", true);
        assert!(is_signed(&text) && !is_signed(&ios_workflow("Loja.aylisproj", "Loja", "loja", "v9.9.9", false)));
        assert_eq!(workflow_aylis_ref(&text).as_deref(), Some("v9.9.9"));
    }

    #[test]
    fn the_workflow_goes_to_the_repository_of_the_project() {
        let dir = std::env::temp_dir().join(format!("aylis-ci-ios-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".git")).unwrap();
        let project = aylis_project::create_at(&dir.join("apps").join("Loja"), "Loja", Language::Rust).unwrap();
        let path = write_ios_workflow(&project, false).unwrap();
        assert!(path.starts_with(dir.join(".github").join("workflows")), "{}", path.display());
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\"app/apps/Loja/Loja.aylisproj\""), "{text}");
        let _ = std::fs::remove_dir_all(&dir);
        let outside = std::env::temp_dir().join(format!("aylis-ci-ios-fora-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&outside);
        let project = aylis_project::create_at(&outside, "Loja", Language::Rust).unwrap();
        // A pasta temporária pode estar dentro de um repositório git; fora dele, é erro.
        if git_root(&outside).is_none() {
            assert!(write_ios_workflow(&project, false).unwrap_err().contains("git"));
        }
        let _ = std::fs::remove_dir_all(&outside);
    }
}
