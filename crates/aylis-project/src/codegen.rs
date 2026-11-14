//! Código dos eventos (o "duplo clique no OnClick" do Delphi), em Rust ou Python.
//!
//! Num projeto ([`generate`]):
//!
//! ```text
//! Rust (uma pasta por dono)                       Python
//! Cargo.toml             (se não existir)         src/main.py           (se não existir)
//! build.rs               (se não existir)         src/aylis_client.py    (se não existir)
//! src/lib.rs             (do usuário; criado uma vez)
//! src/main.rs            (do usuário; criado uma vez)
//! src/forms/<form>.rs    (do usuário: só ganha   src/forms/__init__.py (sempre refeito)
//!                         os handlers novos)     src/forms/<form>.py   (atualizado)
//! generated/mod.rs       (sempre refeito: o registro e os arquivos dos forms)
//! generated/components.rs (sempre refeito: a struct de cada form e o acesso tipado)
//! ```
//!
//! Regerar é seguro: o arquivo de cada form em Rust nunca é refeito (sem bloco marcado; os
//! handlers são métodos da struct do form, e a geração só acrescenta os novos no fim do
//! `impl`); o que é da IDE fica em `generated/`.
//!
//! Os handlers são registrados com o nome do form na frente (`frmClientes.btn_salvar_click`):
//! dois forms podem ter handlers com o mesmo nome.

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::{Language, Project, file_stem_for};

const BEGIN: &str = "aylis-ide:register-begin";
const END: &str = "aylis-ide:register-end";

/// Pasta do aylis (para o `Cargo.toml` dos projetos em Rust apontar para ele).
///
/// Procura, nesta ordem: a pasta `aylis/` ao lado do executável (a
/// IDE instalada, veja `aylis-tool installer-ide`); a pasta onde este crate foi compilado
/// (rodando de dentro do repositório). A instalação sem o suporte a Rust só tem os
/// `resources/`: os projetos em Rust precisam também dos `crates/`.
pub fn aylis_root() -> String {
    static ROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    ROOT.get_or_init(|| {
        let beside_exe = || {
            let dir = std::env::current_exe().ok()?.parent()?.join("aylis");
            dir.join("resources").is_dir().then_some(dir)
        };
        let root = beside_exe().unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
        let root = root.canonicalize().unwrap_or(root);
        // Sem o prefixo \\?\ do Windows e com barras normais (o TOML aceita nos dois sistemas).
        root.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/")
    })
    .clone()
}

/// O caminho do aylis para o `Cargo.toml` do crate em `crate_dir`, sempre relativo: dentro do
/// próprio aylis, até a raiz dele (os exemplos do repositório valem em qualquer clone); num
/// projeto do usuário, até `<projeto>/.aylis/sdk`, a junção que o aylis mantém até a instalação
/// desta máquina (o `Cargo.toml` vai para o git sem o caminho de ninguém).
pub fn aylis_path_for(crate_dir: &Path) -> String {
    let root = aylis_root();
    // A pasta pode ainda não existir (a casca em `.aylis/shell`): a parte que existe é
    // resolvida pelo sistema, o resto entra como está.
    let normalize = |p: &Path| {
        let mut base = p.to_path_buf();
        let mut rest = Vec::new();
        let p = loop {
            if let Ok(c) = base.canonicalize() {
                break rest.iter().rev().fold(c, |acc: PathBuf, part| acc.join(part));
            }
            match (base.file_name().map(|n| n.to_os_string()), base.parent()) {
                (Some(name), Some(parent)) => {
                    rest.push(name);
                    base = parent.to_path_buf();
                }
                _ => break p.to_path_buf(),
            }
        };
        p.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").trim_end_matches('/').to_string()
    };
    let dir = normalize(crate_dir);
    let inside = if cfg!(windows) { dir.to_lowercase().strip_prefix(&root.to_lowercase()).map(str::to_string) } else { dir.strip_prefix(&root).map(str::to_string) };
    match inside {
        Some(rest) if rest.starts_with('/') => {
            let levels = rest.split('/').filter(|s| !s.is_empty()).count();
            vec![".."; levels].join("/")
        }
        // Fora do repositório do aylis (um projeto do usuário): o `Cargo.toml` vai para o git e
        // vale em qualquer máquina, então aponta para `<projeto>/.aylis/sdk`, sempre o mesmo, e o
        // aylis mantém ali uma junção até a instalação desta máquina (fora do git). Antes ia o
        // caminho absoluto daqui, com o nome do usuário.
        _ => match project_dir_of(&dir) {
            Some(project) => {
                let link = format!("{project}/.aylis/sdk");
                ensure_sdk_link(&project, &root);
                relative_path(&dir, &link).unwrap_or(link)
            }
            None => relative_path(&dir, &root).unwrap_or(root),
        },
    }
}

/// De onde vem o Aylis no `Cargo.toml` de um projeto (a chave `sdk`; G11/G03 do Tabernáculo).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AylisSource {
    /// Uma pasta com o código do Aylis (`<pasta>/crates/aylis`), relativa ao `Cargo.toml`.
    Path(String),
    /// A versão publicada no crates.io.
    Version(String),
}

impl AylisSource {
    /// O lugar da dependência `krate` (`aylis`, `aylis-build`) no `Cargo.toml`: o
    /// `path = ".../crates/aylis"` ou o `version = "..."`.
    pub fn spec(&self, krate: &str) -> String {
        match self {
            AylisSource::Path(root) => format!("path = \"{root}/crates/{krate}\""),
            AylisSource::Version(v) => format!("version = \"{v}\""),
        }
    }

    /// Fora do repositório do Aylis (o perfil de otimização das dependências vale): uma
    /// versão, ou um caminho que não é só `..`.
    fn outside(&self) -> bool {
        match self {
            AylisSource::Path(root) => !root.split('/').all(|p| p == ".."),
            AylisSource::Version(_) => true,
        }
    }
}

/// De onde vem o Aylis para o crate em `crate_dir` do projeto: o crates.io (a versão desta
/// IDE), a pasta fixa do `sdk`, ou a junção até o Aylis desta máquina ([`aylis_path_for`]).
pub fn aylis_source(project: &Project, crate_dir: &Path) -> AylisSource {
    match project.sdk.trim() {
        crate::SDK_CRATES_IO => AylisSource::Version(env!("CARGO_PKG_VERSION").to_string()),
        "" | "local" => AylisSource::Path(aylis_path_for(crate_dir)),
        dir => {
            let path = Path::new(dir);
            let abs = if path.is_absolute() { path.to_path_buf() } else { project.dir().join(path) };
            let abs = abs.to_string_lossy().replace('\\', "/").trim_end_matches('/').to_string();
            let from = crate_dir.to_string_lossy().replace('\\', "/");
            AylisSource::Path(relative_path(&from, &abs).unwrap_or(abs))
        }
    }
}

/// A pasta do projeto (a que tem o `.aylisproj`) que contém `dir`, subindo até três níveis
/// (`<projeto>/rust`, `<projeto>/.aylis/shell`).
fn project_dir_of(dir: &str) -> Option<String> {
    let mut current = Some(Path::new(dir));
    for _ in 0..4 {
        let d = current?;
        // A pasta pode ainda não existir (a casca): segue subindo.
        let has_project = std::fs::read_dir(d).is_ok_and(|entries| entries.flatten().any(|e| e.path().extension().is_some_and(|x| x == "aylisproj")));
        if has_project {
            return Some(d.to_string_lossy().replace('\\', "/"));
        }
        current = d.parent();
    }
    None
}

/// Deixa `<projeto>/.aylis/sdk` apontando para o aylis em `root` (a junção do Windows, sem
/// administrador; o link simbólico nos outros sistemas), e o `.aylis/.gitignore` com ela de
/// fora do git. Uma pasta de verdade no lugar não é mexida.
fn ensure_sdk_link(project: &str, root: &str) {
    let aylis_dir = Path::new(project).join(".aylis");
    let link = aylis_dir.join("sdk");
    let target = Path::new(root);
    let canon = |p: &Path| p.canonicalize().ok();
    if canon(&link).is_some_and(|l| Some(l) == canon(target)) {
        return;
    }
    crate::local_dir_ignored(&aylis_dir);
    // Uma junção ou link velho (outra instalação) sai; uma pasta de verdade fica.
    if let Ok(meta) = std::fs::symlink_metadata(&link) {
        if !meta.file_type().is_symlink() && !is_junction(&link) {
            return;
        }
        let _ = std::fs::remove_dir(&link).or_else(|_| std::fs::remove_file(&link));
    }
    #[cfg(windows)]
    let _ = junction::create(target, &link);
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(target, &link);
}

#[cfg(windows)]
fn is_junction(path: &Path) -> bool {
    junction::exists(path).unwrap_or(false)
}

#[cfg(not(windows))]
fn is_junction(_path: &Path) -> bool {
    false
}

/// O caminho de `from` até `to` (os dois absolutos, com `/`), se começam no mesmo disco.
fn relative_path(from: &str, to: &str) -> Option<String> {
    let parts = |p: &str| p.split('/').filter(|s| !s.is_empty()).map(str::to_string).collect::<Vec<_>>();
    let (a, b) = (parts(from), parts(to));
    let same = |x: &str, y: &str| if cfg!(windows) { x.eq_ignore_ascii_case(y) } else { x == y };
    // No Windows, discos diferentes (`C:` e `D:`) não têm caminho relativo.
    if cfg!(windows) && !a.first().zip(b.first()).is_some_and(|(x, y)| same(x, y)) {
        return None;
    }
    let common = a.iter().zip(&b).take_while(|(x, y)| same(x, y)).count();
    let mut out: Vec<String> = vec!["..".to_string(); a.len() - common];
    out.extend(b[common..].iter().cloned());
    Some(if out.is_empty() { ".".to_string() } else { out.join("/") })
}

/// Nome sugerido para o handler: `btnSalvar` + `on_click` → `btn_salvar_click`.
pub fn default_handler_name(component: &str, event: &str) -> String {
    format!("{}_{}", snake(component), event.trim_start_matches("on_"))
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            // Sigla junta: `IBGE` → `ibge`, `HTTPServer` → `http_server`.
            let prev_lower = i > 0 && (chars[i - 1].is_lowercase() || chars[i - 1].is_ascii_digit());
            let starts_word = i > 0 && chars[i - 1].is_uppercase() && chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            if (prev_lower || starts_word) && !out.ends_with('_') {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else if c.is_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    out
}

/// A chave do layout é um evento? `on_*` e os do Delphi `after_*` e `before_*` (os do
/// dataset: `after_scroll`, `before_post`...).
pub fn is_event_key(key: &str) -> bool {
    key.starts_with("on_") || key.starts_with("after_") || key.starts_with("before_")
}

/// Nome do form e handlers usados no layout (os valores de `on_*`, `after_*` e `before_*`), sem repetir.
pub fn form_handlers(layout: &str) -> Result<(String, Vec<String>), String> {
    let doc: Value = serde_json::from_str(layout).map_err(|e| e.to_string())?;
    let root = doc.get("root").unwrap_or(&doc);
    let form = root.get("name").and_then(Value::as_str).unwrap_or("Form").to_string();
    let mut handlers = Vec::new();
    collect(root, &mut handlers);
    // Os dois viram código: um arquivo editado fora da IDE não pode injetar nada nele.
    if !crate::is_identifier(&form) {
        return Err(format!("nome de form inválido '{form}' (letras, dígitos e _, sem começar por dígito)"));
    }
    if let Some(bad) = handlers.iter().find(|h| !crate::is_identifier(h)) {
        return Err(format!("nome de handler inválido '{bad}' no form {form} (letras, dígitos e _, sem começar por dígito)"));
    }
    Ok((form, handlers))
}

thread_local! {
    /// Os handlers ligados a eventos que perguntam no form que está sendo gerado ((form,
    /// handler) → evento): o esqueleto sai com a assinatura certa (o `var` do Pascal, o `Result` do
    /// Rust).
    static QUESTIONS: std::cell::RefCell<std::collections::HashMap<(String, String), String>> = std::cell::RefCell::new(Default::default());
}

/// Os handlers ligados a eventos que perguntam (`on_close_query`, `before_post`...) no layout:
/// (handler, evento).
pub fn question_handlers(layout: &str) -> Vec<(String, String)> {
    fn walk(node: &Value, out: &mut Vec<(String, String)>) {
        let Some(obj) = node.as_object() else { return };
        for (key, value) in obj {
            if let (Some(_), Some(h)) = (aylis_core::question::question_fields(key), value.as_str()) {
                if !h.is_empty() && !out.iter().any(|(o, _)| o == h) {
                    out.push((h.to_string(), key.clone()));
                }
            }
        }
        for child in obj.get("children").and_then(Value::as_array).into_iter().flatten() {
            walk(child, out);
        }
    }
    let mut out = Vec::new();
    if let Ok(doc) = serde_json::from_str::<Value>(layout) {
        walk(doc.get("root").unwrap_or(&doc), &mut out);
    }
    out
}

/// O evento-pergunta a que o handler `handler` do form `form` está ligado (no que está sendo
/// gerado), se for um.
pub(crate) fn question_event(form: &str, handler: &str) -> Option<String> {
    QUESTIONS.with(|q| q.borrow().get(&(form.to_string(), handler.to_string())).cloned())
}

fn collect(node: &Value, out: &mut Vec<String>) {
    let Some(obj) = node.as_object() else { return };
    for (key, value) in obj {
        if let (true, Some(handler)) = (is_event_key(key), value.as_str()) {
            if !handler.is_empty() && !out.iter().any(|h| h == handler) {
                out.push(handler.to_string());
            }
        }
    }
    if let Some(children) = obj.get("children").and_then(Value::as_array) {
        for child in children {
            collect(child, out);
        }
    }
}

/// Gera (ou atualiza) o código do projeto: o da linguagem ativa e, nas outras linguagens que
/// já têm pasta, os handlers novos (vazios) e os renomeados — nenhuma fica sem a função que
/// o form chama. Depois anota o estado de cada handler ([`crate::sync`]). Devolve os
/// arquivos escritos.
///
/// Só os forms que mudaram desde a última geração são lidos (os de pedaço do acesso tipado
/// para trás, [`crate::bindings::stale_forms`]); o registro também olha os forms cujo código o
/// usuário mudou depois dele ([`crate::sync::changed_since_record`]).
/// Os forms que mudaram desde a última geração e os de unit mexida pelo usuário depois dela
/// (`None`: todos). Quem gera e verifica olha antes de gerar (a geração põe tudo em dia).
pub fn changed_forms(project: &Project) -> Option<Vec<String>> {
    let mut stale = crate::bindings::stale_forms(project)?;
    let touched = crate::sync::changed_since_record(project)?;
    stale.extend(touched.into_iter().filter(|n| !stale.contains(n)).collect::<Vec<_>>());
    Some(stale)
}

pub fn generate(project: &Project) -> Result<Vec<PathBuf>, String> {
    project.migrate_layout()?;
    let mut languages = project.code_languages();
    if !languages.contains(&project.language) {
        languages.push(project.language);
    }
    // O que mudou, olhado antes de gerar (a geração põe os pedaços em dia): os forms e as units
    // que o usuário mexeu depois da última geração (uma assinatura de antes é atualizada).
    let stale = crate::bindings::stale_forms(project);
    let touched = crate::sync::changed_since_record(project);
    let union = |a: Option<Vec<String>>, b: &Option<Vec<String>>| match (a, b) {
        (Some(mut a), Some(b)) => {
            a.extend(b.iter().filter(|n| !a.contains(n)).cloned().collect::<Vec<_>>());
            Some(a)
        }
        _ => None,
    };
    let mut written = Vec::new();
    let mut cited = Vec::new();
    for language in languages {
        let p = project.with_language(language);
        let own = if language == project.language { stale.clone() } else { crate::bindings::stale_forms(&p) };
        let changed = union(own, &touched);
        crate::sync::apply_renames_in(&p, changed.as_deref())?;
        let (files, cites) = generate_language(&p, changed.as_deref())?;
        written.extend(files);
        if language == project.language {
            cited = cites;
        }
    }
    let record = union(stale, &touched);
    crate::sync::record_in(project, record.as_deref())?;
    // Só os ícones e as animações citados vão para o app (uma vez, para todas as linguagens):
    // os dos forms da lista de cada um, e o código do usuário lido numa passada.
    written.extend(sync_assets_from(project, &cited)?);
    Ok(written)
}

/// Só o código do form `form` (o F12, abrir o código dele): a unit com os handlers novos e os
/// renomeados, e o acesso tipado aos componentes. O que é de compilar (o `Cargo.toml` e as
/// features, os ícones, o registro dos handlers) fica para o "Gerar código" e o Executar. Um
/// form sem unit ainda, ou outra linguagem que não Rust ou Python (um arquivo para todos os
/// forms), gera tudo.
pub fn generate_form(project: &Project, form: &str) -> Result<Vec<PathBuf>, String> {
    let Some(entry) = project.form(form) else { return generate(project) };
    let forms_dir = project.code_dir().join("src/forms");
    let unit_exists = form_source(project, form).is_some_and(|p| p.is_file());
    if !matches!(project.language, Language::Rust | Language::Python) || !unit_exists {
        return generate(project);
    }
    project.migrate_layout()?;
    let only = [entry.name.clone()];
    crate::sync::apply_renames_in(project, Some(&only))?;
    let code = read_form_code(project, entry, &mut crate::FormFiles::default(), &asset_packs())?;
    // O índice dos forms (os handlers de cada um) acompanha: o acesso tipado abaixo marca o form
    // como em dia, e a geração completa reaproveitaria o índice velho (depois de renomear um
    // handler e dar F12, o registro ficava com o nome antigo e o projeto não compilava;
    // conferência de 08/11).
    let index_path = forms_index(project);
    let mut index: std::collections::BTreeMap<String, FormCode> = read(&index_path).and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let indexed = FormCode { bindings: Vec::new(), ..code.clone() };
    if index.get(&module_name(&entry.file)) != Some(&indexed) {
        index.insert(module_name(&entry.file), indexed);
        if let Some(parent) = index_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        write(&index_path, &serde_json::to_string(&index).map_err(|e| e.to_string())?)?;
    }
    let (path, _) = write_unit(project, entry, &forms_dir, &code)?;
    let mut written = vec![path];
    written.extend(crate::bindings::generate_in(project, Some(&entry.name))?);
    crate::sync::record_in(project, Some(&only))?;
    Ok(written)
}

/// O que a geração guarda de cada form, em `generated/components/forms.json`: o nome (o da
/// raiz do layout), os handlers e os eventos-pergunta deles. Os arquivos que juntam todos os
/// forms (o registro dos handlers) usam isto em vez de reler os forms que não mudaram.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct FormCode {
    form: String,
    handlers: Vec<String>,
    #[serde(default)]
    questions: Vec<(String, String)>,
    /// As classes do form (as features do `Cargo.toml` saem daqui, sem reler os forms).
    #[serde(default)]
    classes: Vec<String>,
    /// Os ícones e as animações que ele cita (`pacote:nome`); `None` numa lista de antes.
    #[serde(default)]
    cited: Option<Vec<String>>,
    /// `componente.evento` → handler, lido agora do form (não vai para o índice).
    #[serde(skip)]
    bindings: Vec<(String, String)>,
}

fn forms_index(project: &Project) -> PathBuf {
    project.code_dir().join("generated/components/forms.json")
}

/// O form lido do disco: o padrão e as variantes por plataforma (os mesmos handlers servem a
/// todas).
fn read_form_code(project: &Project, entry: &crate::FormEntry, files: &mut crate::FormFiles, packs: &[String]) -> Result<FormCode, String> {
    let mut code = FormCode { form: entry.name.clone(), cited: Some(Vec::new()), ..Default::default() };
    for (_, layout) in project.read_entry_variants(entry, files)? {
        let (name, found) = form_handlers(&layout)?;
        code.form = name;
        for h in found {
            if !code.handlers.contains(&h) {
                code.handlers.push(h);
            }
        }
        for q in question_handlers(&layout) {
            if !code.questions.contains(&q) {
                code.questions.push(q);
            }
        }
        if let Ok(value) = serde_json::from_str::<Value>(layout.trim_start_matches('\u{feff}')) {
            classes_in(&value, &mut code.classes);
            crate::sync::walk(value.get("root").unwrap_or(&value), &mut code.bindings, &mut Vec::new());
        }
        if let Some(cited) = code.cited.as_mut() {
            for c in cited_in(&layout, packs) {
                if !cited.contains(&c) {
                    cited.push(c);
                }
            }
        }
    }
    Ok(code)
}

/// As classes do layout (a base: `Button.flat` é `Button`).
fn classes_in(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(class) = map.get("class").and_then(Value::as_str) {
                let base = class.split('.').next().unwrap_or(class).to_string();
                if !out.contains(&base) {
                    out.push(base);
                }
            }
            map.values().for_each(|v| classes_in(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| classes_in(v, out)),
        _ => {}
    }
}

/// O fonte de um form numa subpasta que ainda está no lugar de antes (direto em `src/forms/`,
/// com o nome do arquivo): vai para a subpasta, se lá não há outro e se nenhum form da raiz
/// usa aquele nome.
fn move_flat_source(flat: &Path, new: &Path, project: &Project, entry: &crate::FormEntry) {
    if flat == new || new.exists() || !flat.is_file() {
        return;
    }
    let stem = module_stem(&entry.file);
    if project.forms.iter().any(|f| f.name != entry.name && module_folders(&f.file).is_empty() && module_stem(&f.file) == stem) {
        return;
    }
    if let Some(dir) = new.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::rename(flat, new);
}

/// A unit de um form (Rust ou Python), que é do usuário: só ganha os handlers novos (não os
/// que o usuário apagou: [`crate::sync::removed_handlers`]).
/// Atualiza o fonte do form; devolve o caminho e se o texto mudou (o "Gerar código" lista só
/// os que mudaram: antes listava todos os `src/forms/*.rs`, que são do usuário; G17).
fn write_unit(project: &Project, entry: &crate::FormEntry, forms_dir: &Path, code: &FormCode) -> Result<(PathBuf, bool), String> {
    remember(code);
    let removed = crate::sync::removed_handlers(&crate::sync::load(project), project.language, &code.form, &code.bindings);
    let handlers: Vec<String> = code.handlers.iter().filter(|h| !removed.contains(h)).cloned().collect();
    match project.language {
        Language::Rust => {
            // Os componentes de cada plataforma ficam na struct gerada, como `Option`.
            let path = entry_source(project, entry);
            move_flat_source(&forms_dir.join(format!("{}.rs", module_stem(&entry.file))), &path, project, entry);
            let old = read(&path);
            let mut text = update_rust_unit(old.clone(), &handlers, &code.form);
            drop_unused_stubs(&mut text, &code.handlers, Language::Rust);
            let changed = old.as_deref() != Some(text.as_str());
            if changed {
                write(&path, &text)?;
            }
            Ok((path, changed))
        }
        Language::Python => {
            let path = entry_source(project, entry);
            move_flat_source(&forms_dir.join(format!("{}.py", module_stem(&entry.file))), &path, project, entry);
            // A subpasta é um pacote do Python.
            if let Some(dir) = path.parent().filter(|d| *d != forms_dir) {
                let init = dir.join("__init__.py");
                if !init.exists() {
                    write(&init, "")?;
                }
            }
            let old = read(&path);
            let mut text = update_python(old.clone(), &handlers, &code.form);
            drop_unused_stubs(&mut text, &code.handlers, Language::Python);
            let changed = old.as_deref() != Some(text.as_str());
            if changed {
                write(&path, &text)?;
            }
            Ok((path, changed))
        }
        _ => unreachable!("só Rust e Python têm uma unit por form"),
    }
}

/// Os eventos-pergunta do form, para a assinatura dos handlers (`var CanClose` no Pascal...).
fn remember(code: &FormCode) {
    QUESTIONS.with(|q| {
        let mut q = q.borrow_mut();
        for (h, e) in &code.questions {
            q.insert((code.form.clone(), h.clone()), e.clone());
        }
    });
}

/// O código de uma linguagem, na pasta dela (`project.language`). Só os forms em `changed`
/// (`None`: todos) são lidos e têm a unit atualizada; dos outros vale o que a última geração
/// guardou.
fn generate_language(project: &Project, changed: Option<&[String]>) -> Result<(Vec<PathBuf>, Vec<String>), String> {
    let dir = project.code_dir();
    let forms_dir = dir.join("src/forms");
    std::fs::create_dir_all(&forms_dir).map_err(|e| e.to_string())?;
    let mut written = Vec::new();
    let index_path = forms_index(project);
    let old: std::collections::BTreeMap<String, FormCode> = read(&index_path).and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let mut index = std::collections::BTreeMap::new();
    let mut files = crate::FormFiles::default();
    let units = matches!(project.language, Language::Rust | Language::Python);
    let mut list = Vec::new();
    let mut classes: Vec<String> = Vec::new();
    let mut cited: Vec<String> = Vec::new();
    let packs = asset_packs();
    for entry in &project.forms {
        let module = module_name(&entry.file);
        let in_changed = changed.is_none_or(|c| c.iter().any(|n| n.eq_ignore_ascii_case(&entry.name)));
        let unit_missing = units && !files.exists(&forms_dir.join(format!("{module}.{}", if project.language == Language::Rust { "rs" } else { "py" })));
        // Sem classes: a lista é de antes delas (todo form tem ao menos a da raiz).
        let reread = in_changed || unit_missing || old.get(&module).is_none_or(|c| c.classes.is_empty() || c.cited.is_none());
        let code = if reread { read_form_code(project, entry, &mut files, &packs)? } else { old[&module].clone() };
        for c in code.cited.iter().flatten() {
            if !cited.contains(c) {
                cited.push(c.clone());
            }
        }
        remember(&code);
        if reread && units {
            if let (path, true) = write_unit(project, entry, &forms_dir, &code)? {
                written.push(path);
            }
        }
        list.push((module.clone(), code.form.clone(), code.handlers.clone()));
        for c in &code.classes {
            if !classes.contains(c) {
                classes.push(c.clone());
            }
        }
        // As ligações (lidas agora do form) não vão para o índice: comparado com o do disco, que
        // não as tem, ele parecia sempre mudado.
        index.insert(module, FormCode { bindings: Vec::new(), ..code });
    }
    if index != old {
        if let Some(parent) = index_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        write(&index_path, &serde_json::to_string(&index).map_err(|e| e.to_string())?)?;
    }
    // TypeScript, C#, Java, Go, C++ e Free Pascal: o gerador delas.
    if !units {
        written.extend(crate::hostlang::generate(&dir, &project.name, project.language, &list, changed)?);
    } else {
        written.extend(match project.language {
            Language::Rust => rust_project(project, &list, &classes)?,
            _ => python_project(project, &list)?,
        });
    }
    // O acesso tipado aos componentes (frmX.componente.Propriedade).
    written.extend(crate::bindings::generate(project)?);
    Ok((written, cited))
}

/// `forms/clientes.json` → `clientes` (nome de módulo válido em Rust e Python). Numa subpasta,
/// a pasta entra no nome (`forms/painel/inicio.json` → `painel_inicio`): dois `inicio.json` em
/// pastas diferentes não colidem.
pub fn module_name(file: &str) -> String {
    let name = module_folders(file).iter().map(|f| snake(f)).chain(std::iter::once(module_stem(file))).collect::<Vec<_>>().join("_");
    if name.starts_with(|c: char| c.is_ascii_digit()) { format!("form_{name}") } else { name }
}

/// O nome do arquivo do form em minúsculas com `_` (`Inicio.json` → `inicio`).
fn module_stem(file: &str) -> String {
    let stem = Path::new(file).file_stem().and_then(|s| s.to_str()).unwrap_or("form");
    let name = snake(stem);
    if name.starts_with(|c: char| c.is_ascii_digit()) { format!("form_{name}") } else { name }
}

/// As subpastas do form dentro de `forms/` (`forms/painel/inicio.json` → `["painel"]`).
fn module_folders(file: &str) -> Vec<String> {
    let relative = file.replace('\\', "/");
    let relative = relative.strip_prefix("forms/").unwrap_or(&relative);
    let mut parts: Vec<String> = relative.split('/').map(str::to_string).collect();
    parts.pop();
    parts.into_iter().filter(|p| !p.is_empty() && p != "." && p != "..").collect()
}

/// O caminho do fonte do form dentro de `src/forms/`, sem a extensão: a subpasta do form se
/// repete no código (`forms/painel/inicio.json` → `painel/inicio`), como o Delphi mantém a unit
/// junto do form (pedido do autor em 11/11). Vale para Rust e Python.
pub fn module_path(file: &str) -> String {
    module_folders(file).iter().map(|f| snake(f)).chain(std::iter::once(module_stem(file))).collect::<Vec<_>>().join("/")
}

/// Nome do pacote do Cargo para um projeto (`Vendas Web` → `vendas_web`).
pub fn package_name(project_name: &str) -> String {
    let name = snake(project_name).replace(['-', ' '], "_");
    let name = name.trim_matches('_').to_string();
    if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) { format!("app_{name}") } else { name }
}

/// O `main.rs` que as versões antigas da IDE geravam (o projeto lia a pasta em tempo de
/// execução). Se ainda estiver igual, é trocado pelo novo (com `lib.rs`).
fn old_main_rs(name: &str) -> String {
    format!(
        "//! {name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n\n\
         mod forms;\n\n\
         fn main() -> Result<(), Box<dyn std::error::Error>> {{\n    \
         // Abre o projeto: tema, idioma, assets e os forms marcados como \"criar no início\".\n    \
         let mut app = aylis::Application::open(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{name}.aylisproj\"))?;\n    \
         forms::register(&mut app.ui);\n    \
         app.run()\n}}\n"
    )
}

/// As features do crate `aylis` que os forms do projeto precisam (`data`, `dbcontrols`,
/// `media`), pelas classes usadas. Partes não usadas nem são compiladas. As mesmas classes que
/// o `aylis-build` registra: as dos forms, as dos pacotes e as citadas pelo nome no código Rust
/// (`ui.create("Column", ...)`); sem a feature, o registro citaria uma classe que não compila.
pub fn aylis_features(project: &Project) -> Vec<&'static str> {
    features_for(project, aylis_build::used_classes(&project.dir().join("forms")))
}

/// As features com as classes dos forms já conhecidas (a geração guarda as de cada form).
fn features_for(project: &Project, mut classes: Vec<String>) -> Vec<&'static str> {
    classes.extend(aylis_build::used_classes(&project.dir().join("packages")));
    classes.extend(aylis_build::code_classes(&project.code_dir_of(Language::Rust).join("src")));
    let mut features: Vec<&'static str> = classes
        .iter()
        .filter_map(|c| aylis_build::feature_of(c))
        .filter(|f| !f.is_empty())
        .collect();
    // O fuso do projeto pelo nome precisa da tabela de fusos.
    if !project.time_zone.is_empty() {
        features.push("time-zones");
    }
    // Fontes do projeto: o núcleo mede e desenha o texto com elas (o `harfrust`). Sem fontes,
    // o texto é todo do renderizador e o app não leva o motor (~520 KB de wasm).
    if has_fonts(project) {
        features.push("text-shaping");
    }
    // As que o código usa por conta própria (`features` no `.aylisproj`), entre as do `aylis`
    // que um app escolhe (os renderizadores e o `dev` a IDE põe sozinha).
    features.extend(project.features.iter().filter_map(|f| APP_FEATURES.iter().find(|k| k.eq_ignore_ascii_case(f.trim())).copied()));
    features.sort_unstable();
    features.dedup();
    features
}

/// O projeto tem fontes em `assets/fonts` (as que o app registra: TTF, OTF, TTC).
fn has_fonts(project: &Project) -> bool {
    std::fs::read_dir(project.dir().join("assets").join("fonts")).is_ok_and(|entries| {
        entries.flatten().any(|e| {
            let name = e.file_name().to_string_lossy().to_lowercase();
            name.ends_with(".ttf") || name.ends_with(".otf") || name.ends_with(".ttc")
        })
    })
}

/// As features do crate `aylis` que o `.aylisproj` pode pedir (`features`).
pub const APP_FEATURES: &[&str] =
    &["data", "dbcontrols", "providers", "media", "markdown", "tray", "nfc", "bluetooth", "docview", "webview", "vector", "3d", "remote", "mcp", "time-zones", "text-shaping"];

/// A linha `features = [...]` da dependência do aylis.
fn features_list(features: &[&str]) -> String {
    let items: Vec<String> = features.iter().map(|f| format!("\"{f}\"")).collect();
    format!("[{}]", items.join(", "))
}

fn cargo_toml(project: &Project) -> String {
    let package = project.package_name();
    let crate_dir = project.code_dir_of(Language::Rust);
    let source = aylis_source(project, &crate_dir);
    format!(
        "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\npublish = false\n\n\
         # lib: o aplicativo (forms, handlers, entradas do Android e da web).\n\
         # bin: o executável do desktop (e do iOS).\n\
         [lib]\npath = \"src/lib.rs\"\n\n\
         [[bin]]\nname = \"{package}\"\npath = \"src/main.rs\"\n\n\
         # De onde vem o aylis: a chave `sdk` do projeto (o crates.io, ou uma pasta). As features\n\
         # (dados, mídia...) são as que os forms usam; a IDE atualiza a lista a cada \"Gerar código\".\n\
         [dependencies]\naylis = {{ {aylis}, default-features = false, features = {features} }}\n\n\
         [features]\n\
         # Ligada pela IDE ao executar: o canal cifrado com a IDE (inspetor, hot reload,\n\
         # rastro). Fica fora do app publicado.\n\
         dev = [\"aylis/dev\"]\n\n\
         # Embute forms, temas e assets no executável (e o ícone no .exe do Windows).\n\
         [build-dependencies]\naylis-build = {{ {build} }}\n\n{profile}{render}",
        version = project.version(),
        aylis = source.spec("aylis"),
        build = source.spec("aylis-build"),
        features = features_list(&aylis_features(project)),
        // Dentro do repositório do aylis o caminho é só `..` (`../../..`); de fora, termina no
        // nome da pasta (`../../../aylis`) ou é uma versão: o projeto de fora ganha o perfil.
        profile = if source.outside() { DEV_PROFILE } else { "" },
        render = render_block(project, &crate_dir),
    )
}

/// O Aylis e as outras dependências otimizados no modo de depuração (a tela acompanha o
/// monitor); o código do app continua depurável. Fora do repositório do Aylis: lá dentro o
/// espaço de trabalho dele já faz isso (e o perfil de um membro seria ignorado).
const DEV_PROFILE: &str = "# O Aylis e as outras dependências otimizados mesmo no modo de depuração: a rolagem e as\n\
                           # animações acompanham o monitor. O código do app continua depurável.\n\
                           [profile.dev.package.\"*\"]\nopt-level = 2\ndebug = false\n\n";

const RENDER_BEGIN: &str = "# aylis:render";
const RENDER_END: &str = "# aylis:render-end";

/// O bloco do renderizador de cada plataforma (Projeto → Opções), entre marcadores que a
/// IDE reescreve. Cada plataforma só compila o renderizador dela: no Windows com Direct2D,
/// o Skia nem entra no executável. `crate_dir` é a pasta do `Cargo.toml` (veja
/// [`aylis_path_for`]).
pub fn render_block(project: &Project, crate_dir: &Path) -> String {
    let source = aylis_source(project, crate_dir);
    let render = project.render();
    let mut out = format!(
        "{RENDER_BEGIN}: o renderizador de cada plataforma (Projeto → Opções; a IDE reescreve este bloco)\n"
    );
    for platform in crate::Platform::ALL {
        let choice = render.get(platform);
        out.push_str(&format!(
            "# {}: {}\n[target.'{}'.dependencies]\naylis = {{ {}, default-features = false, features = {} }}\n\n",
            platform.title(),
            choice.title(),
            platform.cfg(),
            source.spec("aylis"),
            features_list(choice.features()),
        ));
    }
    out.push_str(RENDER_END);
    out.push('\n');
    out
}

/// Onde está o bloco do renderizador: do começo da linha que abre até o fim da que fecha. Os
/// marcadores valem só em linha própria, e a abertura é a última antes do fim; entre os dois,
/// só o que o bloco escreve. Um comentário do usuário que cite o marcador não conta: antes, o
/// primeiro `aylis:render` em qualquer lugar abria o bloco, e a geração apagava as
/// dependências entre o comentário e o fim (relatório do Tabernáculo, G01).
fn render_span(cargo: &str) -> Option<(usize, usize)> {
    let mut lines = Vec::new();
    let mut at = 0;
    for line in cargo.split_inclusive('\n') {
        lines.push((at, line));
        at += line.len();
    }
    let end = lines.iter().position(|(_, l)| l.trim() == RENDER_END)?;
    let begin = lines[..end].iter().rposition(|(_, l)| {
        let l = l.trim_end();
        l == RENDER_BEGIN || l.starts_with(&format!("{RENDER_BEGIN}: "))
    })?;
    // Outra coisa entre os dois é do usuário: não mexe.
    let ours = lines[begin + 1..end].iter().all(|(_, l)| {
        let l = l.trim();
        l.is_empty() || l.starts_with('#') || l.starts_with("[target.") || l.starts_with("aylis = ")
    });
    ours.then(|| (lines[begin].0, lines[end].0 + lines[end].1.len()))
}

/// Põe o bloco do renderizador em dia com o projeto: troca o que está entre os marcadores
/// ou, num projeto gerado antes desse bloco existir (com `default-features = false`),
/// acrescenta no fim. Projetos com o aylis completo (sem `default-features = false`) já
/// têm o Skia e ficam como estão. Devolve o texto novo, se mudou.
pub fn sync_render(cargo: &str, project: &Project) -> Option<String> {
    let block = render_block(project, &project.code_dir_of(Language::Rust));
    let text = match render_span(cargo) {
        Some((a, end)) => format!("{}{}{}", &cargo[..a], block, &cargo[end..]),
        _ if cargo.lines().any(|l| l.trim_start().starts_with("aylis =") && l.contains("default-features = false")) => {
            let sep = if cargo.ends_with("\n\n") { "" } else if cargo.ends_with('\n') { "\n" } else { "\n\n" };
            format!("{cargo}{sep}{block}")
        }
        _ => return None,
    };
    (text != cargo).then_some(text)
}

/// Onde a IDE guarda os recursos offline (fontes e pacotes de ícones).
pub fn resources_dir() -> PathBuf {
    PathBuf::from(aylis_root()).join("resources")
}

/// Ícones de pacote citados no projeto: textos `"pacote:nome"` nos forms e no código
/// (`src/`), para os pacotes que existem em `resources/icons`.
pub fn used_icons(project: &Project) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    citations(project, &icon_packs())
}

/// Os pacotes de ícones da IDE (`resources/icons`).
fn icon_packs() -> Vec<String> {
    std::fs::read_dir(resources_dir().join("icons"))
        .map(|d| d.flatten().filter_map(|e| e.file_name().to_string_lossy().strip_suffix(".txt").map(str::to_string)).collect())
        .unwrap_or_default()
}

/// Animações de pacote citadas (`"noto:fogo"`), para os pacotes de `resources/lottie`.
/// Os pacotes de animação da IDE (`resources/lottie`).
fn lottie_packs() -> Vec<String> {
    std::fs::read_dir(resources_dir().join("lottie"))
        .map(|d| d.flatten().filter(|e| e.file_type().is_ok_and(|t| t.is_dir())).map(|e| e.file_name().to_string_lossy().to_string()).collect())
        .unwrap_or_default()
}

pub fn used_animations(project: &Project) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    citations(project, &lottie_packs())
}

/// Copia para `assets/lottie/<pacote>/` as animações citadas que o pacote tem (um "noto:" solto
/// num texto não vira arquivo), com a licença do pacote. Devolve os arquivos gravados.
fn copy_animations(project: &Project, used: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>) -> Result<Vec<PathBuf>, String> {
    let base = project.dir().join("assets/lottie");
    let packs = resources_dir().join("lottie");
    let mut written = Vec::new();
    for (pack, names) in used {
        let present: Vec<&String> = names.iter().filter(|n| packs.join(pack).join(format!("{n}.json")).is_file()).collect();
        if present.is_empty() {
            continue;
        }
        let dir = base.join(pack);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for name in present.iter().map(|n| format!("{n}.json")).chain(["LICENSE.txt".to_string()]) {
            let (from, to) = (packs.join(pack).join(&name), dir.join(&name));
            let Ok(data) = std::fs::read(&from) else { continue };
            if std::fs::read(&to).ok().as_deref() != Some(data.as_slice()) {
                std::fs::write(&to, &data).map_err(|e| format!("{}: {e}", to.display()))?;
                written.push(to);
            }
        }
    }
    Ok(written)
}

/// Põe em `assets/lottie/<pacote>/` as animações citadas (com a licença do pacote) e tira
/// as que deixaram de ser. Devolve os arquivos gravados.
pub fn sync_animations(project: &Project) -> Result<Vec<PathBuf>, String> {
    sync_animations_used(project, used_animations(project))
}

fn sync_animations_used(project: &Project, mut used: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>) -> Result<Vec<PathBuf>, String> {
    let packs = resources_dir().join("lottie");
    // Só as que o pacote tem (um "noto:" solto num texto não vira arquivo).
    for (pack, names) in used.iter_mut() {
        names.retain(|n| packs.join(pack).join(format!("{n}.json")).is_file());
    }
    used.retain(|_, names| !names.is_empty());
    let base = project.dir().join("assets/lottie");
    if let Ok(entries) = std::fs::read_dir(&base) {
        for e in entries.flatten().filter(|e| e.path().is_dir()) {
            let pack = e.file_name().to_string_lossy().to_string();
            // Só mexe nos pacotes da IDE: animações próprias do app ficam.
            if !packs.join(&pack).is_dir() {
                continue;
            }
            let names = used.get(&pack);
            for f in std::fs::read_dir(e.path()).into_iter().flatten().flatten() {
                let file = f.file_name().to_string_lossy().to_string();
                if let Some(stem) = file.strip_suffix(".json") {
                    if names.is_none_or(|n| !n.contains(stem)) {
                        let _ = std::fs::remove_file(f.path());
                    }
                }
            }
            if names.is_none() {
                let _ = std::fs::remove_dir_all(e.path());
            }
        }
    }
    copy_animations(project, &used)
}

/// Textos `"pacote:nome"` nos forms e no código (`src/`), para os pacotes dados. O código
/// gerado fica de fora: ele não cita ícones (no ERP de 1009 forms, era um arquivo de 10 MB).
fn citations(project: &Project, packs: &[String]) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    let mut used = form_citations(project, packs);
    for (pack, names) in code_citations(project, packs) {
        used.entry(pack).or_default().extend(names);
    }
    used
}

/// As citações de um arquivo, guardadas pela marca dele (a hora e o tamanho) e pelos pacotes
/// procurados: abrir um form na IDE registra os ícones citados no projeto, e reler os 1009
/// forms e o código do ERP a cada form aberto levava 320 ms (medido em 07/11; `erp_ide_cost`).
fn scan_file_citations(path: &Path, packs: &[String], used: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>) {
    type Cited = Vec<(String, String)>;
    static CACHE: std::sync::Mutex<Option<std::collections::HashMap<(PathBuf, String), (crate::FileStamp, Cited)>>> = std::sync::Mutex::new(None);
    let Some(stamp) = crate::FileStamp::read(path) else { return };
    let key = (path.to_path_buf(), packs.join(","));
    if let Some((cached, cited)) = CACHE.lock().ok().and_then(|c| c.as_ref().and_then(|c| c.get(&key).cloned())) {
        if cached == stamp {
            for (pack, name) in cited {
                used.entry(pack).or_default().insert(name);
            }
            return;
        }
    }
    let Ok(text) = std::fs::read_to_string(path) else { return };
    let mut found = Default::default();
    scan_citations(&text, packs, &mut found);
    let cited: Cited = found.iter().flat_map(|(pack, names)| names.iter().map(move |n| (pack.clone(), n.clone()))).collect();
    for (pack, names) in found {
        used.entry(pack).or_default().extend(names);
    }
    if let Ok(mut cache) = CACHE.lock() {
        cache.get_or_insert_with(Default::default).insert(key, (stamp, cited));
    }
}

/// As citações nos forms (todos, lidos do disco).
fn form_citations(project: &Project, packs: &[String]) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    let mut used = Default::default();
    let dir = project.dir();
    for name in crate::Files::Dir(dir.clone()).list("forms/") {
        if name.ends_with(".json") {
            scan_file_citations(&dir.join(&name), packs, &mut used);
        }
    }
    used
}

/// As citações no código do usuário de todas as linguagens do projeto (e o `src/` da raiz, do
/// layout antigo): um ícone citado em qualquer uma vai para o app.
fn code_citations(project: &Project, packs: &[String]) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    let mut used = Default::default();
    let dir = project.dir();
    let files = crate::Files::Dir(dir.clone());
    let code: Vec<String> = project.code_languages().into_iter().flat_map(|l| files.list(&format!("{}/src/", l.folder()))).collect();
    for name in files.list("src/").into_iter().chain(code) {
        let code_ext = [".rs", ".py", ".ts", ".cs", ".java", ".go", ".cpp", ".hpp", ".pas"];
        if code_ext.iter().any(|e| name.ends_with(e)) && !is_generated(project, &name) {
            scan_file_citations(&dir.join(&name), packs, &mut used);
        }
    }
    used
}

/// Os pacotes de ícones e de animações da IDE, juntos (uma passada procura os dois).
fn asset_packs() -> Vec<String> {
    icon_packs().into_iter().chain(lottie_packs()).collect()
}

/// As citações de um texto como `pacote:nome`, para os pacotes dados.
fn cited_in(text: &str, packs: &[String]) -> Vec<String> {
    let mut used = Default::default();
    scan_citations(text, packs, &mut used);
    used.into_iter().flat_map(|(pack, names)| names.into_iter().map(move |n| format!("{pack}:{n}"))).collect()
}

/// A sincronização da geração: as citações dos forms vêm da lista de cada form (`forms`, que a
/// geração guarda), e só o código do usuário é lido, numa passada para os ícones e as
/// animações.
fn sync_assets_from(project: &Project, forms: &[String]) -> Result<Vec<PathBuf>, String> {
    let (icons, lottie) = (icon_packs(), lottie_packs());
    let mut used = code_citations(project, &asset_packs());
    for c in forms {
        if let Some((pack, name)) = c.split_once(':') {
            used.entry(pack.to_string()).or_default().insert(name.to_string());
        }
    }
    let (mut for_icons, mut for_lottie) = (std::collections::BTreeMap::new(), std::collections::BTreeMap::new());
    for (pack, names) in used {
        if icons.contains(&pack) {
            for_icons.insert(pack.clone(), names.clone());
        }
        if lottie.contains(&pack) {
            for_lottie.insert(pack, names);
        }
    }
    let mut written = sync_icons_used(project, for_icons)?;
    written.extend(sync_animations_used(project, for_lottie)?);
    Ok(written)
}

/// O arquivo (relativo ao projeto) é refeito pela geração (o acesso tipado, a pasta
/// `generated/`).
fn is_generated(project: &Project, name: &str) -> bool {
    let name = name.replace('\\', "/");
    project.code_languages().into_iter().any(|l| {
        let base = format!("{}/", l.folder());
        name.starts_with(&format!("{base}generated/")) || name == format!("{base}{}", crate::bindings::file(l))
    })
}

/// Ao salvar um form: acrescenta em `assets/` os ícones e as animações que o texto dele cita, sem
/// ler os outros forms. Tirar os que ninguém mais usa é do [`sync_icons`] e do
/// [`sync_animations`], que o Executar e o Publicar rodam (eles leem o projeto inteiro de
/// qualquer jeito). Devolve os arquivos gravados.
pub fn add_assets_cited_in(project: &Project, text: &str) -> Result<Vec<PathBuf>, String> {
    let mut written = Vec::new();
    let out_dir = project.dir().join("assets/icons");
    for (pack, names) in icons_cited_in(text) {
        let path = out_dir.join(format!("{pack}.txt"));
        let current = read(&path).and_then(|t| aylis_core::icons::parse_pack(&t).ok());
        let mut all: std::collections::BTreeSet<String> = current.as_ref().map(|p| p.names().map(str::to_string).collect()).unwrap_or_default();
        if names.iter().all(|n| all.contains(n)) {
            continue;
        }
        all.extend(names);
        let Ok(source) = std::fs::read_to_string(resources_dir().join("icons").join(format!("{pack}.txt"))) else { continue };
        let parsed = aylis_core::icons::parse_pack(&source)?;
        let list: Vec<&str> = all.iter().map(String::as_str).collect();
        std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
        write(&path, &parsed.subset(&list))?;
        written.push(path);
    }
    let packs = lottie_packs();
    let mut used = Default::default();
    scan_citations(text, &packs, &mut used);
    written.extend(copy_animations(project, &used)?);
    Ok(written)
}

/// Os ícones de pacote citados num texto (o form da prévia da IDE): `"material:home"`,
/// para os pacotes que existem em `resources/icons`.
pub fn icons_cited_in(text: &str) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    let packs: Vec<String> = std::fs::read_dir(resources_dir().join("icons"))
        .map(|d| d.flatten().filter_map(|e| e.file_name().to_string_lossy().strip_suffix(".txt").map(str::to_string)).collect())
        .unwrap_or_default();
    let mut used = Default::default();
    scan_citations(text, &packs, &mut used);
    used
}

fn scan_citations(text: &str, packs: &[String], used: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>) {
    for pack in packs {
        let needle = format!("{pack}:");
        let mut rest = text;
        while let Some(i) = rest.find(&needle) {
            // O prefixo precisa começar a palavra (aspas, espaço, ';').
            let before = rest[..i].chars().last();
            let after = &rest[i + needle.len()..];
            let name: String = after.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
            let len = name.len();
            if !name.is_empty() && before.is_none_or(|c| !c.is_ascii_alphanumeric() && c != '-') {
                used.entry(pack.clone()).or_default().insert(name);
            }
            rest = &after[len.min(after.len())..];
        }
    }
}

/// Põe em `assets/icons/` um pacote por pacote usado, só com os ícones citados (o app não
/// leva os milhares de ícones da IDE). Devolve os arquivos gravados.
pub fn sync_icons(project: &Project) -> Result<Vec<PathBuf>, String> {
    sync_icons_used(project, used_icons(project))
}

fn sync_icons_used(project: &Project, used: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>) -> Result<Vec<PathBuf>, String> {
    let out_dir = project.dir().join("assets/icons");
    let mut written = Vec::new();
    // Pacotes que deixaram de ser usados saem (os do app, importados, ficam).
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if let Some(stem) = name.strip_suffix(".txt") {
                let from_ide = resources_dir().join("icons").join(&name).exists();
                if from_ide && !used.contains_key(stem) {
                    let _ = std::fs::remove_file(e.path());
                }
            }
        }
    }
    for (pack, names) in &used {
        let source = resources_dir().join("icons").join(format!("{pack}.txt"));
        let Ok(text) = std::fs::read_to_string(&source) else { continue };
        let parsed = aylis_core::icons::parse_pack(&text)?;
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        let subset = parsed.subset(&names);
        let path = out_dir.join(format!("{pack}.txt"));
        if read(&path).as_deref() != Some(subset.as_str()) {
            std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
            write(&path, &subset)?;
            written.push(path);
        }
    }
    Ok(written)
}

/// Atualiza o `Cargo.toml` de um projeto feito por uma versão antiga da IDE, mantendo o
/// que o usuário mudou (comentários, outras dependências): o caminho do aylis (se a pasta
/// mudou ou não existe mais), o `aylis-build` (o `build.rs` gerado precisa dele) e a feature
/// `inspect` (o Inspetor do app). `root` vem de [`aylis_path_for`] e `crate_dir` é a pasta do
/// `Cargo.toml` (onde os caminhos relativos começam). Devolve o texto novo e o que mudou,
/// se algo mudou.
pub fn upgrade_cargo(cargo: &str, root: &str, crate_dir: &Path) -> Option<(String, Vec<&'static str>)> {
    use toml_edit::{DocumentMut, InlineTable, Item, Table, Value, value};
    let mut doc: DocumentMut = cargo.parse().ok()?;
    let mut notes = Vec::new();
    let crate_path = |name: &str| format!("{root}/crates/{name}");
    // Um `path` que não é o esperado e que não serve mais: de outra instalação, de uma pasta
    // que sumiu, ou absoluto num crate de dentro do aylis (que passa a ser relativo).
    let stale = |path: &str, wanted: &str| {
        let norm = |p: &str| p.replace('\\', "/").trim_end_matches('/').to_lowercase();
        let resolved = crate_dir.join(path).join("Cargo.toml");
        norm(path) != norm(wanted) && (wanted.starts_with("..") || !resolved.exists())
    };
    if let Some(dep) = doc.get_mut("dependencies").and_then(|d| d.get_mut("aylis")).and_then(Item::as_inline_table_mut) {
        let wanted = crate_path("aylis");
        if dep.get("path").and_then(Value::as_str).is_some_and(|p| stale(p, &wanted)) {
            dep.insert("path", wanted.into());
            notes.push("caminho do aylis");
        }
    }
    let wanted = crate_path("aylis-build");
    let build = doc.entry("build-dependencies").or_insert(Item::Table(Table::new()));
    if let Some(table) = build.as_table_like_mut() {
        if table.get("aylis-build").is_none() {
            let mut dep = InlineTable::new();
            dep.insert("path", wanted.into());
            table.insert("aylis-build", Item::Value(Value::InlineTable(dep)));
            notes.push("aylis-build (para o build.rs)");
        } else if let Some(dep) = table.get_mut("aylis-build").and_then(Item::as_inline_table_mut) {
            if dep.get("path").and_then(Value::as_str).is_some_and(|p| stale(p, &wanted)) {
                dep.insert("path", wanted.into());
                notes.push("caminho do aylis-build");
            }
        }
    }
    let features = doc.entry("features").or_insert(Item::Table(Table::new()));
    if let Some(table) = features.as_table_like_mut() {
        // A antiga `inspect` (o inspetor sem cifra) virou a `dev`.
        if table.remove("inspect").is_some() {
            notes.push("feature inspect trocada pela dev");
        }
        if table.get("dev").is_none() {
            let mut list = toml_edit::Array::new();
            list.push("aylis/dev");
            table.insert("dev", value(list));
            notes.push("feature dev");
        }
    }
    let text = doc.to_string();
    (!notes.is_empty() && text != cargo).then_some((text, notes))
}

/// O `Cargo.toml` (`cargo`) do projeto em `dir` precisa de um `[workspace]` próprio: não
/// tem, e uma pasta acima é um workspace que não o lista como membro (o cargo usa o primeiro
/// workspace que acha subindo as pastas). Excluído de lá também precisa: o `aylis`, pelo
/// atalho `.aylis/sdk` dentro da mesma árvore, herdava o `rust-version` do workspace de cima
/// e o build falhava (relatório do RTSPlayer, 10/11).
pub fn needs_own_workspace(cargo: &str, dir: &Path) -> bool {
    use toml_edit::{DocumentMut, Item};
    if cargo.parse::<DocumentMut>().is_ok_and(|d| d.get("workspace").is_some()) {
        return false;
    }
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    for root in dir.ancestors().skip(1) {
        let Some(text) = read(&root.join("Cargo.toml")) else { continue };
        let Ok(doc) = text.parse::<DocumentMut>() else { continue };
        let Some(ws) = doc.get("workspace").and_then(Item::as_table_like) else { continue };
        let relative = dir.strip_prefix(root).map(|r| r.to_string_lossy().replace('\\', "/")).unwrap_or_default();
        let list = |key: &str| -> Vec<String> {
            ws.get(key).and_then(Item::as_array).map(|a| a.iter().filter_map(|v| v.as_str()).map(|s| s.trim_end_matches('/').to_string()).collect()).unwrap_or_default()
        };
        let member = list("members").iter().any(|m| glob_path(m, &relative));
        return !member;
    }
    false
}

/// O crate em `dir` é a raiz do workspace dele: tem `[workspace]`, ou nenhuma pasta de cima é
/// um workspace (o perfil dele vale).
fn workspace_root(cargo: &str, dir: &Path) -> bool {
    use toml_edit::DocumentMut;
    if cargo.parse::<DocumentMut>().is_ok_and(|d| d.get("workspace").is_some()) {
        return true;
    }
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    !dir.ancestors().skip(1).any(|root| read(&root.join("Cargo.toml")).and_then(|t| t.parse::<DocumentMut>().ok()).is_some_and(|d| d.get("workspace").is_some()))
}

/// `examples/*` casa com `examples/web` (o `*` vale um trecho do caminho, como no cargo).
fn glob_path(pattern: &str, path: &str) -> bool {
    fn segment(p: &str, s: &str) -> bool {
        match p.split_once('*') {
            None => p.eq_ignore_ascii_case(s),
            Some((before, after)) => {
                s.len() >= before.len() + after.len()
                    && s.get(..before.len()).is_some_and(|h| h.eq_ignore_ascii_case(before))
                    && (0..=s.len() - before.len()).any(|i| s.is_char_boundary(before.len() + i) && segment(after, &s[before.len() + i..]))
            }
        }
    }
    let (p, s): (Vec<&str>, Vec<&str>) = (pattern.split('/').collect(), path.split('/').collect());
    p.len() == s.len() && p.iter().zip(&s).all(|(p, s)| segment(p, s))
}

/// O `Cargo.toml` do projeto em dia com os forms (features) e com o renderizador de cada
/// plataforma. Chamado no "Gerar código" e ao salvar as Opções do projeto.
pub fn sync_cargo(project: &Project) -> Result<bool, String> {
    sync_cargo_with(project, &aylis_features(project))
}

fn sync_cargo_with(project: &Project, features: &[&str]) -> Result<bool, String> {
    let path = project.code_dir().join("Cargo.toml");
    let Some(original) = read(&path) else { return Ok(false) };
    let mut text = original.clone();
    if let Some(t) = sync_aylis_features(&text, features) {
        text = t;
    }
    if let Some(t) = sync_render(&text, project) {
        text = t;
    }
    let crates: Vec<String> = code_packages(project).iter().map(|m| crate::packages::code_dependency(m, &project.dir(), &project.code_dir())).collect();
    if let Some(t) = sync_package_crates(&text, &crates) {
        text = t;
    }
    let source = aylis_source(project, &project.code_dir());
    if let Some(t) = sync_aylis_source(&text, &source) {
        text = t;
    }
    // O perfil que otimiza as dependências também nos `Cargo.toml` de antes dele (B09 do
    // Tabernáculo: sem ele, a tela fica de 10 a 50 vezes mais lenta). Só onde vale: no crate que
    // é a raiz do próprio workspace (num membro de outro, o cargo ignora o perfil).
    if source.outside() && !text.contains("[profile.dev.package.\"*\"]") && workspace_root(&text, &project.code_dir()) {
        let sep = if text.ends_with("\n\n") { "" } else if text.ends_with('\n') { "\n" } else { "\n\n" };
        text = format!("{text}{sep}{}", DEV_PROFILE.trim_end_matches('\n'));
        text.push('\n');
    }
    if text == original {
        return Ok(false);
    }
    write(&path, &text)?;
    Ok(true)
}

/// Toda dependência do aylis no `Cargo.toml` aponta para a mesma pasta (`root`): um bloco escrito
/// à mão (o 3D da Landing só fora do navegador) ficava com o caminho do repositório, e a cópia
/// do projeto noutro lugar não compilava ("Dependency 'aylis' has different source paths";
/// conferência de 08/11).
#[cfg(test)]
fn sync_aylis_paths(text: &str, root: &str) -> Option<String> {
    sync_aylis_source(text, &AylisSource::Path(root.to_string()))
}

/// Toda dependência do aylis (`aylis` e `aylis-build`) com a origem `source`: o caminho de
/// pasta ou a versão do crates.io (trocar o `sdk` do projeto troca todas; G11).
fn sync_aylis_source(text: &str, source: &AylisSource) -> Option<String> {
    let mut changed = false;
    let lines: Vec<String> = text
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let krate = if trimmed.starts_with("aylis = {") {
                "aylis"
            } else if trimmed.starts_with("aylis-build = {") {
                "aylis-build"
            } else {
                return line.to_string();
            };
            let wanted = source.spec(krate);
            // O `path = "..."` (do aylis: termina em `/crates/<krate>`) ou o `version = "..."`.
            let found = ["path = \"", "version = \""].iter().find_map(|key| {
                let start = line.find(key)?;
                let len = line[start + key.len()..].find('"')?;
                let current = &line[start..start + key.len() + len + 1];
                let ours = key.starts_with("version") || current.ends_with(&format!("/crates/{krate}\""));
                ours.then_some((start, current.len(), current.to_string()))
            });
            let Some((start, len, current)) = found else { return line.to_string() };
            if current == wanted {
                return line.to_string();
            }
            changed = true;
            format!("{}{wanted}{}", &line[..start], &line[start + len..])
        })
        .collect();
    changed.then(|| lines.join("\n") + if text.ends_with('\n') { "\n" } else { "" })
}

/// Os pacotes de código (tipo B) que o projeto usa.
fn code_packages(project: &Project) -> Vec<crate::packages::Manifest> {
    crate::packages::used(&crate::Files::Dir(project.dir())).into_iter().filter(|m| m.is_code()).collect()
}

/// O que marca, no `Cargo.toml`, as dependências que a IDE mantém (os pacotes de código). É
/// marcador de formato: fica como está nos projetos já gerados.
const PACKAGE_MARK: &str = "# pacote do aylis";

/// Põe na seção `[dependencies]` uma linha por pacote de código (marcadas com
/// [`PACKAGE_MARK`]) e tira as dos que saíram. Devolve o texto novo, se mudou.
pub fn sync_package_crates(cargo: &str, lines: &[String]) -> Option<String> {
    let mut out = Vec::new();
    let mut section = String::new();
    let mut placed = false;
    for line in cargo.lines() {
        let t = line.trim_start();
        if t.starts_with('[') {
            section = t.trim_end().to_string();
        }
        if line.trim_end().ends_with(PACKAGE_MARK) {
            continue;
        }
        out.push(line.to_string());
        if section == "[dependencies]" && t.starts_with("[dependencies]") && !placed {
            out.extend(lines.iter().map(|l| format!("{l}  {PACKAGE_MARK}")));
            placed = true;
        }
    }
    let text = out.join("\n") + if cargo.ends_with('\n') { "\n" } else { "" };
    (text != cargo).then_some(text)
}

/// Atualiza as features do aylis na seção `[dependencies]` do `Cargo.toml` (só nos que
/// já usam `default-features = false`, como os gerados pela IDE; as dependências por
/// plataforma, do renderizador, ficam com [`sync_render`]). Devolve o texto novo, se mudou.
pub fn sync_aylis_features(cargo: &str, features: &[&str]) -> Option<String> {
    let mut changed = false;
    let mut section = String::new();
    let lines: Vec<String> = cargo
        .lines()
        .map(|line| {
            let t = line.trim_start();
            if t.starts_with('[') {
                section = t.trim_end().to_string();
            }
            if section == "[dependencies]" && t.starts_with("aylis =") && line.contains("default-features = false") {
                if let (Some(a), Some(b)) = (line.find("features = ["), line.rfind(']')) {
                    // O `features = [` de `default-features = false` não tem colchete.
                    let new = format!("{}features = {}{}", &line[..a], features_list(features), &line[b + 1..]);
                    if new != line {
                        changed = true;
                        return new;
                    }
                }
            }
            line.to_string()
        })
        .collect();
    changed.then(|| lines.join("\n") + if cargo.ends_with('\n') { "\n" } else { "" })
}

const BUILD_RS: &str = "//! Embute o projeto (forms, temas, assets) no executável. Gerado pela IDE do Aylis.\n\n\
                        fn main() {\n    aylis_build::project();\n}\n";

fn lib_rs(project: &Project) -> String {
    format!(
        "//! {name}: o aplicativo. Criado pela IDE do Aylis (o código é seu; o que a IDE refaz fica\n\
         //! em `generated/`).\n\n\
         /// O que a IDE gera: as structs dos forms, o acesso tipado e o registro (e os arquivos\n\
         /// dos forms, em `src/forms/`).\n\
         #[rustfmt::skip]\n\
         #[path = \"../generated/mod.rs\"]\n\
         pub mod generated;\n\n\
         /// Cria o aplicativo: tema, idioma, assets e os forms marcados como \"criar no início\",\n\
         /// com os handlers de todos os forms registrados.\n\
         ///\n\
         /// No desenvolvimento os arquivos vêm da pasta do projeto; no aplicativo publicado,\n\
         /// de dentro do executável.\n\
         pub fn app() -> Result<aylis::Application, aylis::Error> {{\n    \
         let mut app = aylis::Application::bundled(aylis::include_project!())?;\n    \
         generated::register(&mut app.ui);\n    \
         Ok(app)\n}}\n\n\
         /// Android: a NativeActivity carrega a biblioteca e chama esta função.\n\
         #[cfg(target_os = \"android\")]\n\
         #[unsafe(no_mangle)]\n\
         fn android_main(android: aylis::platform::android::AndroidApp) {{\n    \
         aylis::run_android(android, app);\n}}\n\n\
         /// Navegador: a página chama esta função depois de carregar o WebAssembly.\n\
         #[cfg(target_arch = \"wasm32\")]\n\
         #[unsafe(no_mangle)]\n\
         pub extern \"C\" fn aylis_start() {{\n    \
         aylis::run_web(app);\n}}\n",
        name = project.name
    )
}

/// A unidade sem tela do projeto (o `BackgroundService`), criada uma vez.
const BACKGROUND_RS: &str = "//! A lógica sem tela do app: roda no serviço em segundo plano (o componente\n\
//! BackgroundService), numa thread que continua com a tela fechada. Gerado pela IDE do Aylis\n\
//! (o corpo é seu).\n\n\
use std::time::Duration;\n\n\
/// Recebe as mensagens da tela (o `send` do BackgroundService) e manda as suas (que chegam no\n\
/// `on_message` dele). Termina quando o serviço é desligado.\n\
pub fn run(service: aylis::background::Service) {\n    \
while !service.stopped() {\n        \
if let Some(message) = service.recv(Duration::from_secs(1)) {\n            \
service.send(format!(\"recebi: {message}\"));\n        \
}\n    \
}\n}\n";

/// O que o `lib.rs` ganha com o `BackgroundService` (uma vez): a unidade e a entrada que o
/// serviço do Android chama quando sobe sem a tela (no boot ou ao reiniciar).
const BACKGROUND_LIB: &str = "\n// O serviço em segundo plano (acrescentado pela IDE ao usar o BackgroundService).\n\
pub mod background;\n\n\
/// Android: o serviço em primeiro plano (também sozinho, no boot ou quando o sistema o\n\
/// reinicia) começa a lógica sem tela.\n\
#[cfg(target_os = \"android\")]\n\
#[unsafe(no_mangle)]\n\
pub extern \"system\" fn Java_dev_aylis_AylisActivity_00024Background_nativeStart(_env: *mut std::ffi::c_void, _class: *mut std::ffi::c_void) {\n    \
aylis::background::start(Some(background::run));\n}\n";

/// Com um `BackgroundService` nos forms: cria `src/background.rs` e liga no `lib.rs` (a
/// unidade, a entrada do Android e o `set_worker` no `app()`).
fn sync_background(dir: &Path, classes: &[String]) -> Result<Vec<PathBuf>, String> {
    if !classes.iter().any(|c| c == "BackgroundService") {
        return Ok(Vec::new());
    }
    let mut written = Vec::new();
    let unit = dir.join("src/background.rs");
    if !unit.exists() {
        write(&unit, BACKGROUND_RS)?;
        written.push(unit);
    }
    let lib = dir.join("src/lib.rs");
    if let Some(mut text) = read(&lib).filter(|t| !t.contains("pub mod background;")) {
        text = text.replacen("generated::register(&mut app.ui);", "generated::register(&mut app.ui);\n    aylis::background::set_worker(background::run);", 1);
        text.push_str(BACKGROUND_LIB);
        write(&lib, &text)?;
        written.push(lib);
    }
    Ok(written)
}

fn main_rs(project: &Project) -> String {
    format!(
        "//! {name}: o ponto de entrada do desktop. Gerado pela IDE do Aylis (pode editar).\n\
         // Sem console no Windows quando publicado (em desenvolvimento, a saída vai para a IDE).\n\
         #![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]\n\n\
         fn main() -> Result<(), aylis::Error> {{\n    \
         {package}::app()?.run()\n}}\n",
        name = project.name,
        // O crate se chama pelo nome da biblioteca, com `_` no lugar do `-` (o relatório do
        // Tabernáculo, G06: `meu-app::app()` não compila).
        package = project.lib_name()
    )
}

fn rust_project(project: &Project, forms: &[(String, String, Vec<String>)], classes: &[String]) -> Result<Vec<PathBuf>, String> {
    let dir = project.code_dir();
    let mut written = Vec::new();
    let cargo = dir.join("Cargo.toml");
    let main = dir.join("src/main.rs");
    let lib = dir.join("src/lib.rs");
    // Projeto de uma versão antiga, ainda com os arquivos gerados intocados: atualiza.
    let old = !lib.exists() && read(&main).is_some_and(|m| m == old_main_rs(&project.name));
    if old {
        let _ = std::fs::remove_file(&main);
        if read(&cargo).is_some_and(|c| !c.contains("[lib]") && !c.contains("[build-dependencies]")) {
            let _ = std::fs::remove_file(&cargo);
        }
    }
    // Montados só se faltarem: o `Cargo.toml` lê os fontes e os forms (as features), e isso
    // a cada "Gerar código" (e F12) era tempo jogado fora.
    // O `src/main.rs` só num crate que o usa: o `Cargo.toml` nasce agora, ou cita o arquivo, ou
    // não diz os binários (e o cargo pega o `src/main.rs`). Num crate só de biblioteca (com
    // `autobins = false`, ou com os `[[bin]]` noutros arquivos), não (G13 do Tabernáculo).
    let wants_main = read(&cargo).is_none_or(|c| c.contains("src/main.rs") || (!c.contains("[[bin]]") && !c.contains("autobins = false")));
    let main_path = main.clone();
    let files: [(PathBuf, fn(&Project) -> String); 4] =
        [(cargo, cargo_toml), (dir.join("build.rs"), |_| BUILD_RS.to_string()), (lib, lib_rs), (main, main_rs)];
    for (path, make) in files {
        if path == main_path && !wants_main {
            continue;
        }
        if !path.exists() {
            write(&path, &make(project))?;
            written.push(path);
        }
    }
    // Projeto de uma versão antiga com o Cargo.toml mexido: só o que falta.
    let cargo = dir.join("Cargo.toml");
    let source = aylis_source(project, &dir);
    let upgraded = match &source {
        AylisSource::Path(root) => read(&cargo).and_then(|c| upgrade_cargo(&c, root, &dir)),
        // Pelo crates.io: o que faltar entra com a versão (o `aylis-build`, a feature `dev`).
        AylisSource::Version(_) => read(&cargo).and_then(|c| upgrade_cargo(&c, "", &dir)).map(|(t, n)| (sync_aylis_source(&t, &source).unwrap_or(t), n)),
    };
    if let Some((text, _)) = upgraded {
        if write_changed(&cargo, &text)? && !written.contains(&cargo) {
            written.push(cargo.clone());
        }
    }
    // Dentro da pasta de outro workspace (um projeto salvo dentro de um repositório Rust) o
    // cargo se recusa a compilar: o projeto vira o próprio workspace.
    if let Some(text) = read(&cargo).filter(|c| needs_own_workspace(c, &dir)) {
        write(&cargo, &format!("{}\n# Fora do workspace da pasta acima: o projeto compila sozinho.\n[workspace]\n", text.trim_end()))?;
        if !written.contains(&cargo) {
            written.push(cargo.clone());
        }
    }
    written.extend(sync_background(&dir, classes)?);
    // Os forms mudaram de componentes? E o renderizador? Atualiza o Cargo.toml.
    if sync_cargo_with(project, &features_for(project, classes.to_vec()))? && !written.contains(&cargo) {
        written.push(cargo);
    }
    let path = dir.join("generated/mod.rs");
    std::fs::create_dir_all(dir.join("generated")).map_err(|e| e.to_string())?;
    if write_changed(&path, &generated_mod(project, forms))? {
        written.push(path);
    }
    Ok(written)
}

/// O `generated/mod.rs`: o acesso tipado e as structs dos forms (`components`), os arquivos
/// dos forms do usuário (`src/forms/`, montados aqui para a pasta deles não precisar de um
/// `mod.rs` que a IDE teria de mexer) e o registro das classes dos forms e dos handlers.
fn generated_mod(project: &Project, forms: &[(String, String, Vec<String>)]) -> String {
    let mut text = String::from(
        "//! O que a IDE do Aylis refaz a cada \"Gerar código\" (não edite): o acesso tipado aos\n\
         //! componentes, a struct de cada form e o registro. O código dos forms é seu, em\n\
         //! `src/forms/`.\n\n\
         use aylis::prelude::Ui;\n\n\
         pub mod components;\n\
         pub use components::*;\n",
    );
    for (module, form, _) in forms {
        // A subpasta do form se repete no código (`src/forms/painel/inicio.rs`).
        let path = project.form(form).map_or_else(|| module.clone(), |e| module_path(&e.file));
        text.push_str(&format!("\n#[path = \"../src/forms/{path}.rs\"]\npub mod {module};\n"));
    }
    text.push_str("\n/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e\n/// as classes dos pacotes de código.\npub fn register(ui: &mut Ui) {\n");
    for m in code_packages(project) {
        text.push_str(&format!("    {}::register(ui);\n", m.crate_ident()));
    }
    for (_, form, handlers) in forms {
        let ty = crate::bindings::form_struct(form);
        text.push_str(&format!("    ui.register_form_class(\"{form}\", {ty}::bind);\n"));
        for h in handlers {
            // O caminho inteiro: um `use` sairia sem uso num projeto sem handler (G07).
            text.push_str(&format!("    ui.register_handler(\"{form}.{h}\", aylis::prelude::instances::form_handler({ty}::{h}));\n"));
        }
    }
    text.push_str("}\n");
    text
}

/// O app casca de um projeto que não é em Rust, para o celular e o navegador: um crate em
/// `.aylis/shell` que leva os forms do projeto e liga a tela ao backend em `backend_url`
/// (o código do projeto roda num servidor, com `aylis-host --bridge`). Devolve a pasta.
pub fn remote_shell(project: &Project, backend_url: &str) -> Result<PathBuf, String> {
    let dir = project.dir();
    let shell = dir.join(".aylis").join("shell");
    std::fs::create_dir_all(shell.join("src")).map_err(|e| e.to_string())?;
    let package = project.package_name();
    let source = aylis_source(project, &shell);
    let mut features = aylis_features(project);
    if !features.contains(&"remote") {
        features.push("remote");
    }
    let cargo = format!(
        "# O app casca de {name} ({language}): os forms do projeto e a conexão com o servidor.\n\
         # Gerado pelo aylis a cada publicação: não edite.\n\
         [package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n\
         [lib]\npath = \"src/lib.rs\"\n\n\
         [dependencies]\naylis = {{ {aylis}, default-features = false, features = {features} }}\n\n\
         [build-dependencies]\naylis-build = {{ {build} }}\n\n\
         # Fora de qualquer workspace.\n[workspace]\n\n{render}",
        aylis = source.spec("aylis"),
        build = source.spec("aylis-build"),
        name = project.name,
        language = project.language.name(),
        version = project.version(),
        features = features_list(&features),
        render = render_block(project, &shell),
    );
    write(&shell.join("Cargo.toml"), &cargo)?;
    let dir_text = dir.display().to_string().replace('\\', "/");
    write(
        &shell.join("build.rs"),
        &format!("//! Embute o projeto (forms, temas, assets) da pasta dele. Gerado pelo aylis.\n\nfn main() {{\n    aylis_build::project_at(std::path::Path::new(\"{dir_text}\"));\n}}\n"),
    )?;
    let url = backend_url.replace('\\', "").replace('"', "");
    write(
        &shell.join("src/lib.rs"),
        &format!(
            "//! {name}: a tela no aparelho, o código no servidor ({url}). Gerado pelo aylis.\n\n\
             /// Onde o código do projeto roda (aylis-host --bridge no servidor).\n\
             pub const BACKEND: &str = \"{url}\";\n\n\
             pub fn app() -> Result<aylis::Application, aylis::Error> {{\n    \
             let mut app = aylis::Application::bundled(aylis::include_project!())?;\n    \
             if let Err(e) = aylis::connect_backend(&mut app.ui, BACKEND) {{\n        \
             aylis::controls::toast(&mut app.ui, &format!(\"Sem conexão com o servidor: {{e}}\"));\n    \
             }}\n    \
             Ok(app)\n}}\n\n\
             #[cfg(target_os = \"android\")]\n#[unsafe(no_mangle)]\n\
             fn android_main(android: aylis::platform::android::AndroidApp) {{\n    aylis::run_android(android, app);\n}}\n\n\
             #[cfg(target_arch = \"wasm32\")]\n#[unsafe(no_mangle)]\n\
             pub extern \"C\" fn aylis_start() {{\n    aylis::run_web(app);\n}}\n",
            name = project.name
        ),
    )?;
    Ok(shell)
}

fn python_project(project: &Project, forms: &[(String, String, Vec<String>)]) -> Result<Vec<PathBuf>, String> {
    let dir = project.code_dir();
    let mut written = Vec::new();
    let main = dir.join("src/main.py");
    if !main.exists() {
        let text = format!(
            "\"\"\"{name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n\n\
             Executado pelo aylis-host: aylis-host --project ../{name}.aylisproj -- python src/main.py\n\"\"\"\n\n\
             import os\nimport sys\n\n\
             sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))\n\n\
             from aylis_client import Aylis  # noqa: E402\n\
             import forms  # noqa: E402\n\n\
             aylis = Aylis()\nforms.register(aylis)\naylis.run()  # abre o form principal do projeto\n",
            name = project.name
        );
        write(&main, &text)?;
        written.push(main);
    }
    // O cliente vem do aylis (item 12): o `src/aylis_client.py` só carrega o da instalação (ou do
    // repositório), sem cópia no projeto. Refeito a cada geração (a cópia antiga sai).
    let client = dir.join("src/aylis_client.py");
    let from = format!("{}/clients/python/aylis_client.py", aylis_path_for(&dir.join("src")));
    let loader = format!(
        "\"\"\"O cliente do protocolo vem do aylis (a instalação, ou o repositório): este arquivo só o\ncarrega. Gerado pela IDE do Aylis a cada \"Gerar código\": não edite.\"\"\"\n\n\
         import importlib.util as _util\nimport os as _os\nimport sys as _sys\n\n\
         _path = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), \"{from}\")\n\
         _spec = _util.spec_from_file_location(\"aylis_client\", _path)\n\
         _module = _util.module_from_spec(_spec)\n\
         _sys.modules[\"aylis_client\"] = _module\n\
         _spec.loader.exec_module(_module)\n"
    );
    if read(&client).as_deref() != Some(loader.as_str()) {
        write(&client, &loader)?;
        written.push(client);
    }
    let mut text = String::from("\"\"\"Forms do projeto. Gerado pela IDE do Aylis a cada \"Gerar código\": não edite.\"\"\"\n\n");
    // As classes dos forms se registram ao importar o components; aqui, os handlers.
    text.push_str("from . import components  # noqa: F401\n");
    for (m, form, _) in forms {
        // Numa subpasta, o pacote dela (`from .painel import inicio as painel_inicio`).
        let path = project.form(form).map_or_else(|| m.clone(), |e| module_path(&e.file));
        match path.rsplit_once('/') {
            Some((package, leaf)) => text.push_str(&format!("from .{} import {leaf} as {m}\n", package.replace('/', "."))),
            None => text.push_str(&format!("from . import {m}\n")),
        }
    }
    text.push_str("\n\ndef register(aylis):\n");
    for (m, form, handlers) in forms {
        for h in handlers {
            text.push_str(&format!("    aylis.on(\"{form}.{h}\")(lambda event: {m}.{h}(aylis, event))\n"));
        }
    }
    if forms.iter().all(|(_, _, h)| h.is_empty()) {
        text.push_str("    pass\n");
    }
    let path = dir.join("src/forms/__init__.py");
    write(&path, &text)?;
    written.push(path);
    Ok(written)
}

/// Gera os handlers de um layout solto (fora de projeto): `<layout>_handlers.rs` e
/// `<layout>_handlers.py` ao lado dele.
pub fn generate_standalone(layout: &Path) -> Result<Vec<PathBuf>, String> {
    let text = std::fs::read_to_string(layout).map_err(|e| format!("não consegui ler {}: {e}", layout.display()))?;
    let (form, handlers) = form_handlers(&text)?;
    let stem = layout.file_stem().and_then(|s| s.to_str()).unwrap_or("tela");
    let dir = layout.parent().unwrap_or(Path::new("."));
    let rust = dir.join(format!("{stem}_handlers.rs"));
    write(&rust, &update_rust(read(&rust), &handlers, &form, "use aylis_core::{ComponentId, EventArgs, Ui};"))?;
    let python = dir.join(format!("{stem}_handlers.py"));
    write(&python, &update_python(read(&python), &handlers, &form))?;
    Ok(vec![rust, python])
}

pub(crate) fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

pub(crate) fn write(path: &Path, text: &str) -> Result<(), String> {
    write_changed(path, text).map(|_| ())
}

/// Grava só se o texto mudou (`true` se gravou): um arquivo igual não é regravado, e o
/// controle de versão e quem vigia o disco não veem mudança que não houve (I06 do Tabernáculo).
pub(crate) fn write_changed(path: &Path, text: &str) -> Result<bool, String> {
    if read(path).is_some_and(|old| old == text) {
        return Ok(false);
    }
    // A pasta pode ainda não existir (o fonte de um form numa subpasta).
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty() && !d.exists()) {
        std::fs::create_dir_all(dir).map_err(|e| format!("não consegui criar {}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
    Ok(true)
}

/// Troca o que está entre os marcadores.
fn replace_block(text: &str, block: &str) -> String {
    match (text.find(BEGIN), text.find(END)) {
        (Some(b), Some(e)) if b < e => {
            let start = text[..b].rfind('\n').map_or(0, |i| i + 1);
            let end = text[e..].find('\n').map_or(text.len(), |i| e + i + 1);
            format!("{}{}{}", &text[..start], block, &text[end..])
        }
        _ => text.to_string(),
    }
}

const COMPONENTS_BEGIN: &str = "aylis-ide:components-begin";
const COMPONENTS_END: &str = "aylis-ide:components-end";

/// Os nomes dos componentes do layout (sem a raiz), na ordem em que aparecem.
pub fn component_names(layout: &str) -> Vec<String> {
    fn walk(node: &Value, out: &mut Vec<String>) {
        for child in node.get("children").and_then(Value::as_array).into_iter().flatten() {
            if let Some(name) = child.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                out.push(name.to_string());
            }
            walk(child, out);
        }
    }
    let doc: Value = serde_json::from_str(layout).unwrap_or(Value::Null);
    let mut out = Vec::new();
    walk(doc.get("root").unwrap_or(&doc), &mut out);
    out
}

/// O bloco `Componentes` de um form com variantes: um campo por componente; os que não
/// existem em todas as variantes são `Option` (o código confere antes de usar).
pub fn update_components(text: &str, form: &str, variants: &[(String, Vec<String>)]) -> String {
    let mut all: Vec<&String> = Vec::new();
    for (_, names) in variants {
        for n in names {
            if !all.contains(&n) {
                all.push(n);
            }
        }
    }
    let field = |n: &str| {
        let s = snake(n);
        if s.starts_with(|c: char| c.is_ascii_digit()) { format!("c_{s}") } else { s }
    };
    let mut fields = String::new();
    let mut inits = String::new();
    for n in &all {
        let missing: Vec<&str> = variants.iter().filter(|(_, names)| !names.contains(n)).map(|(v, _)| v.as_str()).collect();
        let f = field(n);
        if missing.is_empty() {
            fields.push_str(&format!("    pub {f}: ComponentId,\n"));
            inits.push_str(&format!("            {f}: ui.find_in(form, \"{n}\")?,\n"));
        } else {
            fields.push_str(&format!("    /// Não existe em: {}.\n    pub {f}: Option<ComponentId>,\n", missing.join(", ")));
            inits.push_str(&format!("            {f}: ui.find_in(form, \"{n}\"),\n"));
        }
    }
    let names: Vec<&str> = variants.iter().map(|(v, _)| v.as_str()).collect();
    let block = format!(
        "// {COMPONENTS_BEGIN} (gerado pela IDE; não edite este bloco)\n\
         /// Os componentes de {form} nas variantes ({}). Os `Option` só existem em algumas.\n\
         pub struct Componentes {{\n{fields}}}\n\n\
         impl Componentes {{\n    \
             /// Os componentes do form aberto (a variante desta plataforma).\n    \
             pub fn de(ui: &Ui) -> Option<Self> {{\n        \
                 let form = ui.find_path(\"{form}\")?;\n        \
                 Some(Self {{\n{inits}        }})\n    \
             }}\n\
         }}\n\
         // {COMPONENTS_END}\n",
        names.join(", ")
    );
    match (text.find(COMPONENTS_BEGIN), text.find(COMPONENTS_END)) {
        (Some(b), Some(e)) if b < e => {
            let start = text[..b].rfind('\n').map_or(0, |i| i + 1);
            let end = text[e..].find('\n').map_or(text.len(), |i| e + i + 1);
            format!("{}{}{}", &text[..start], block, &text[end..])
        }
        _ => format!("{}\n{block}", text.trim_end_matches('\n').to_string() + "\n"),
    }
}

/// Atualiza (ou cria) o arquivo de eventos de um form em Rust. `uses` é a linha de `use`
/// do arquivo novo.
pub fn update_rust(existing: Option<String>, handlers: &[String], form: &str, uses: &str) -> String {
    let mut block = format!("    // {BEGIN} (gerado pela IDE; não edite este bloco)\n");
    for h in handlers {
        block.push_str(&format!("    ui.register_handler(\"{form}.{h}\", {h});\n"));
    }
    if handlers.is_empty() {
        block.push_str("    let _ = ui;\n");
    }
    block.push_str(&format!("    // {END}\n"));

    let mut text = match existing {
        Some(text) => replace_block(&text, &block),
        None => format!(
            "//! Eventos do form {form}. Gerado pela IDE do Aylis: o corpo das funções é seu;\n\
             //! a IDE só refaz o bloco de registro e acrescenta funções novas.\n\n\
             {uses}\n\n\
             /// Liga os handlers aos nomes usados no layout.\n\
             pub fn register(ui: &mut Ui) {{\n{block}}}\n"
        ),
    };
    for h in handlers {
        if !text.contains(&format!("fn {h}(")) {
            text.push_str(&format!("\npub fn {h}(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {{\n{}}}\n", rust_stub_body(h)));
        }
    }
    text
}

/// O corpo inicial de um handler. Os de texto de um `Field` (pelo nome padrão,
/// `campo_get_text` e `campo_set_text`) já vêm com a resposta, como o `OnGetText` e o
/// `OnSetText` do Delphi.
/// O arquivo do form em Rust num projeto (`src/forms/x.rs`): é do usuário e nunca é refeito.
/// Novo, nasce com o estado do form (`Data`) e o `impl` da struct gerada; depois a geração só
/// acrescenta, no fim do `impl`, o handler novo que o form cita e que ainda não existe.
pub fn update_rust_unit(existing: Option<String>, handlers: &[String], form: &str) -> String {
    let ty = crate::bindings::form_struct(form);
    let mut text = existing.unwrap_or_else(|| {
        format!(
            "//! Os eventos do form {form}: o código é seu. A IDE só acrescenta, no fim do `impl`,\n\
             //! os handlers novos (os componentes e o registro ficam em `generated/`).\n\n\
             #[allow(unused_imports)]\n\
             use aylis::prelude::*;\n\n\
             use crate::generated::{ty};\n\n\
             /// O estado do form: cada janela aberta dele tem o seu (`self.data`).\n\
             #[derive(Default)]\n\
             pub struct Data {{}}\n\n\
             // Os handlers têm o nome do componente (`btnSalvar_click`).\n\
             #[allow(non_snake_case)]\n\
             impl {ty} {{\n}}\n"
        )
    });
    // Só a função de verdade conta: a versão antiga num comentário não impede o handler novo.
    let missing: Vec<&String> = handlers.iter().filter(|h| crate::sync::find_marker(Language::Rust, &text, &format!("fn {h}(")).is_none()).collect();
    if missing.is_empty() {
        return text;
    }
    // Um handler com maiúscula (`btnSalvar_click`) dava o aviso `non_snake_case` (conferência de
    // 08/11): o `impl` do form ganha o `allow`, também num arquivo de antes.
    if missing.iter().any(|h| h.chars().any(char::is_uppercase)) {
        allow_non_snake_case(&mut text, &ty);
    }
    let mut stubs = String::new();
    for h in missing {
        // Um evento que pergunta (o `before_post`, o `on_close_query`): `Err` recusa, com a
        // mensagem na tela, como a exceção no handler do Delphi.
        if let Some(event) = question_event(form, h) {
            stubs.push_str(&format!(
                "\n    /// `{event}`: `Err(mensagem)` recusa (o `?` repassa).\n    pub fn {h}(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) -> Result<(), String> {{\n        let _ = (ui, sender, args);\n        Ok(())\n    }}\n"
            ));
            continue;
        }
        stubs.push_str(&format!("\n    pub fn {h}(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {{\n{}    }}\n", indent(rust_stub_body(h))));
    }
    // No fim do `impl` da struct do form; sem ele, um `impl` novo no fim do arquivo.
    match impl_end(&text, &ty) {
        Some(at) => {
            let stubs = if text[..at].trim_end().ends_with('{') { stubs.trim_start_matches('\n').to_string() } else { stubs };
            let line_start = text[..at].rfind('\n').map_or(0, |n| n + 1);
            if text[line_start..at].trim().is_empty() {
                // A chave sozinha na linha: os handlers entram antes dela.
                text.insert_str(line_start, &stubs);
            } else {
                // A chave junto de outro texto (`impl FrmX {}` depois do `cargo fmt`): os
                // handlers entram logo antes dela, que vai para a linha de baixo. Antes iam para
                // o começo da linha, fora do `impl` (relatório do Tabernáculo, G08).
                text.insert_str(at, &format!("\n{stubs}"));
            }
        }
        None => text.push_str(&format!("\n#[allow(non_snake_case)]\nimpl {ty} {{{stubs}}}\n")),
    }
    text
}

/// Põe `#[allow(non_snake_case)]` antes do `impl Tipo {`, se ele ainda não tem.
fn allow_non_snake_case(text: &mut String, ty: &str) {
    let Some(start) = text.find(&format!("impl {ty} {{")) else { return };
    let line_start = text[..start].rfind('\n').map_or(0, |n| n + 1);
    if text[..line_start].trim_end().ends_with("#[allow(non_snake_case)]") {
        return;
    }
    text.insert_str(line_start, "#[allow(non_snake_case)]\n");
}

/// Onde fica a chave que fecha o `impl Tipo {`.
fn impl_end(text: &str, ty: &str) -> Option<usize> {
    let start = text.find(&format!("impl {ty} {{"))?;
    let open = start + text[start..].find('{')?;
    matching_brace(text, open)
}

/// A chave que fecha a que abre em `open`, num texto em Rust: as chaves dentro de strings,
/// caracteres e comentários não contam (vale também nas outras linguagens de chaves).
pub(crate) fn matching_brace(text: &str, open: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        if let Some(end) = skip_literal(text, i) {
            i = end + 1;
            continue;
        }
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Onde `needle` aparece em código (fora de comentários, strings e caracteres), a partir de
/// `from`; um `needle` que começa com letra não vale no meio de outro nome (`fn x(` não acha
/// `fn ax(`). A versão antiga comentada de um handler não é o handler (G22 do Tabernáculo).
pub(crate) fn find_in_code(text: &str, needle: &str, from: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let word = needle.bytes().next().is_some_and(ident);
    let mut i = from;
    while i < bytes.len() {
        if let Some(end) = skip_literal(text, i) {
            i = end + 1;
            continue;
        }
        if bytes[i..].starts_with(needle.as_bytes()) && !(word && i > 0 && ident(bytes[i - 1])) {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Se em `i` começa um comentário, uma string ou um caractere (de Rust e das outras
/// linguagens de chaves), o último byte dele; `None` se ali é código.
pub(crate) fn skip_literal(text: &str, i: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let end = bytes.len();
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    match bytes[i] {
        b'/' if bytes.get(i + 1) == Some(&b'/') => Some(text[i..].find('\n').map_or(end, |n| i + n)),
        b'/' if bytes.get(i + 1) == Some(&b'*') => Some(text[i + 2..].find("*/").map_or(end, |n| i + 2 + n + 1)),
        // r"..." e r#"..."# (e br"..."): até as aspas com o mesmo número de #.
        b'r' if matches!(bytes.get(i + 1), Some(b'"' | b'#')) && (i == 0 || !ident(bytes[i - 1]) || (bytes[i - 1] == b'b' && (i == 1 || !ident(bytes[i - 2])))) => {
            let hashes = bytes[i + 1..].iter().take_while(|b| **b == b'#').count();
            if bytes.get(i + 1 + hashes) != Some(&b'"') {
                return None;
            }
            let close = format!("\"{}", "#".repeat(hashes));
            Some(text[i + 2 + hashes..].find(&close).map_or(end, |n| i + 2 + hashes + n + close.len() - 1))
        }
        // As strings de crase (TypeScript, Go) vão até a outra crase, sem escape.
        q @ (b'"' | b'`') => {
            let mut j = i + 1;
            while j < end && bytes[j] != q {
                j += if q == b'"' && bytes[j] == b'\\' { 2 } else { 1 };
            }
            Some(j.min(end))
        }
        // Um caractere ('{', '\n', '\'', '\u{7b}'); um tempo de vida ('a) é código.
        b'\'' => {
            if bytes.get(i + 1) == Some(&b'\\') {
                return Some(text.get(i + 3..).and_then(|t| t.find('\'')).map_or(end, |n| i + 3 + n));
            }
            let c = text[i + 1..].chars().next()?;
            text[i + 1 + c.len_utf8()..].starts_with('\'').then_some(i + 1 + c.len_utf8())
        }
        _ => None,
    }
}

/// Quatro espaços a mais em cada linha (o corpo de um handler dentro do `impl`).
fn indent(body: &str) -> String {
    body.lines().map(|l| if l.is_empty() { "\n".to_string() } else { format!("    {l}\n") }).collect()
}

pub(crate) fn rust_stub_body(handler: &str) -> &'static str {
    if handler.ends_with("_get_text") {
        "    // O texto mostrado para cada valor do campo (grid, Edit ligado...).\n    \
         aylis::data::get_text(ui, sender, args, |value| value.to_string());\n"
    } else if handler.ends_with("_set_text") {
        "    // O texto digitado vira o valor do campo; Err(mensagem) recusa.\n    \
         aylis::data::set_text(ui, sender, args, |text| Ok(aylis::data::Value::String(text.to_string())));\n"
    } else {
        "    let _ = (ui, sender, args);\n"
    }
}

/// Tira do arquivo do form os handlers que nenhum evento usa mais (`bound`: os ligados no form)
/// e que continuam **exatamente** como a IDE os gerou, vazios, e que nada mais no arquivo chama.
/// Um handler com qualquer código do usuário fica (decidido pelo autor em 11/11: "remover o
/// handler somente se estiver vazio", como o Delphi faz ao salvar). Devolve os tirados.
pub fn drop_unused_stubs(text: &mut String, bound: &[String], language: Language) -> Vec<String> {
    let declaration = match language {
        Language::Rust => "pub fn ",
        Language::Python => "def ",
        _ => return Vec::new(),
    };
    let mut names = Vec::new();
    let mut from = 0;
    while let Some(i) = text[from..].find(declaration) {
        let start = from + i + declaration.len();
        if let Some(end) = text[start..].find('(').map(|e| start + e) {
            let name = &text[start..end];
            if crate::is_identifier(name) {
                names.push(name.to_string());
            }
        }
        from = start;
    }
    let mut removed = Vec::new();
    for name in names {
        if bound.iter().any(|b| b == &name) || text.matches(&format!("{name}(")).count() > 1 {
            continue;
        }
        let stubs = match language {
            Language::Rust => {
                let body = format!("    pub fn {name}(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {{\n        let _ = (ui, sender, args);\n    }}\n");
                vec![format!("\n{body}"), body]
            }
            _ => vec![format!("\n\ndef {name}(aylis, event):\n    pass\n")],
        };
        if let Some((at, len)) = stubs.iter().find_map(|s| text.find(s.as_str()).map(|at| (at, s.len()))) {
            text.replace_range(at..at + len, "");
            removed.push(name);
        }
    }
    removed
}

/// Atualiza (ou cria) o arquivo de eventos de um form em Python.
pub fn update_python(existing: Option<String>, handlers: &[String], form: &str) -> String {
    // O arquivo é do usuário: os handlers (o registro fica no forms/__init__.py gerado);
    // dentro deles, frmX é a janela do evento.
    let mut text = existing.unwrap_or_else(|| {
        format!(
            "\"\"\"Eventos do form {form}: o código é seu (o registro dos handlers fica no\n\
             forms/__init__.py gerado; a IDE só acrescenta os handlers novos). Dentro de um handler,\n\
             {form} é a janela do evento.\"\"\"\n"
        )
    });
    for h in handlers {
        if crate::sync::find_marker(Language::Python, &text, &format!("def {h}(")).is_none() {
            text.push_str(&format!("\n\ndef {h}(aylis, event):\n    pass\n"));
        }
    }
    text
}

/// Arquivo de eventos do form (para a IDE abrir no editor).
pub fn form_source(project: &Project, form: &str) -> Option<PathBuf> {
    project.form(form).map(|entry| entry_source(project, entry))
}

/// O [`form_source`] de uma entrada já achada (quem percorre todos os forms não procura cada
/// um pelo nome: com 1000 forms, isso era 1 milhão de comparações).
pub fn entry_source(project: &Project, entry: &crate::FormEntry) -> PathBuf {
    let module = module_name(&entry.file);
    let path = module_path(&entry.file);
    let file = match project.language {
        Language::Rust => format!("src/forms/{path}.rs"),
        Language::Python => format!("src/forms/{path}.py"),
        other => crate::hostlang::form_file(other, &module, &entry.name),
    };
    project.code_dir().join(file)
}

/// Nome do arquivo de form sugerido (usado ao criar forms): `frmClientes` → `clientes`.
pub fn suggested_file(form: &str) -> String {
    format!("forms/{}.json", file_stem_for(form))
}

#[cfg(test)]
mod rust_unit_tests {
    use super::*;

    /// O `impl` do form em Rust leva o `allow(non_snake_case)` (os handlers têm o nome do
    /// componente): no arquivo novo e num de antes que ganha um handler com maiúscula.
    #[test]
    fn the_form_impl_allows_the_component_names() {
        let new = update_rust_unit(None, &["btnSalvar_click".to_string()], "frmPrincipal");
        assert_eq!(new.matches("#[allow(non_snake_case)]").count(), 1, "{new}");
        assert!(new.contains("pub fn btnSalvar_click("));
        let old = "use crate::generated::FrmPrincipal;\n\nimpl FrmPrincipal {\n}\n".to_string();
        let updated = update_rust_unit(Some(old), &["btnOk_click".to_string()], "frmPrincipal");
        assert!(updated.contains("#[allow(non_snake_case)]\nimpl FrmPrincipal {"), "{updated}");
        let again = update_rust_unit(Some(updated), &["btnOk_click".to_string(), "btnNovo_click".to_string()], "frmPrincipal");
        assert_eq!(again.matches("#[allow(non_snake_case)]").count(), 1, "{again}");
    }

    /// O evento tirado do form leva o handler junto só se ele está vazio, como a IDE gerou
    /// (decidido pelo autor em 11/11); com código do usuário, ou chamado por outro método, fica.
    #[test]
    fn only_empty_unused_handlers_leave() {
        let all = ["btnA_click".to_string(), "btnB_click".to_string(), "btnC_click".to_string(), "btnD_click".to_string()];
        let mut text = update_rust_unit(None, &all, "frmPrincipal");
        // O B ganha código; o D é chamado pelo C.
        text = text.replace("pub fn btnB_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {\n        let _ = (ui, sender, args);", "pub fn btnB_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {\n        let _ = (ui, sender, args);\n        println!(\"salvar\");");
        text = text.replace("pub fn btnC_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {\n        let _ = (ui, sender, args);", "pub fn btnC_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {\n        self.btnD_click(ui, sender, args);");
        // Só o C continua ligado a um evento.
        let removed = drop_unused_stubs(&mut text, &["btnC_click".to_string()], Language::Rust);
        assert_eq!(removed, vec!["btnA_click".to_string()], "{text}");
        assert!(!text.contains("fn btnA_click(") && text.contains("fn btnB_click(") && text.contains("fn btnD_click("), "{text}");

        let mut py = update_python(None, &all, "frmPrincipal");
        assert_eq!(drop_unused_stubs(&mut py, &all[..1], Language::Python).len(), 3, "{py}");
        assert!(py.contains("def btnA_click("));
    }
}

#[cfg(test)]
mod aylis_path_tests {
    use super::*;

    /// Um bloco do aylis escrito à mão (o 3D fora do navegador) vai para a mesma pasta da
    /// dependência principal.
    #[test]
    fn every_aylis_dependency_points_to_the_same_folder() {
        let text = "[dependencies]\naylis = { path = \"C:/aylis/crates/aylis\", features = [] }\n\n[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]\naylis = { path = \"../../../crates/aylis\", default-features = false, features = [\"3d\"] }\n";
        let out = sync_aylis_paths(text, "C:/aylis").unwrap();
        assert_eq!(out.matches("path = \"C:/aylis/crates/aylis\"").count(), 2, "{out}");
        assert!(out.contains("features = [\"3d\"]"));
        assert!(sync_aylis_paths(&out, "C:/aylis").is_none(), "já certo: não muda");
    }
}
