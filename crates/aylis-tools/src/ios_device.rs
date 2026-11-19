//! O app de iOS num iPhone de verdade, sem projeto do Xcode: o certificado do desenvolvedor,
//! o perfil de provisionamento, os direitos (entitlements), o `.ipa` e o aparelho conectado.
//!
//! O simulador aceita a assinatura "ad hoc" (`codesign --sign -`); o iPhone exige três coisas
//! da Apple, todas da máquina e nunca do projeto:
//!
//! - **o certificado** ("Apple Development: ..."), no Keychain. O Xcode cria com qualquer
//!   Apple ID, gratuito inclusive (Xcode → Settings → Accounts);
//! - **o perfil** (`.mobileprovision`): diz que equipe, que bundle id e que aparelhos. Com a
//!   conta gratuita, o Xcode só gera o perfil a partir de um projeto dele com o mesmo bundle
//!   id rodado uma vez no iPhone; com a conta paga, baixa-se do site da Apple;
//! - **o aparelho**: no perfil de desenvolvimento, o iPhone tem de estar na lista dele.
//!
//! Sem nada configurado, o Aylis acha o perfil nas pastas onde o Xcode grava
//! ([`profile_dirs`]), pelo bundle id, pelo aparelho e pela validade, e o certificado pelo
//! próprio perfil (a impressão SHA-1 dos certificados que ele aceita). As Opções podem fixar
//! os dois: [`PROFILE`] e [`IDENTITY`] no `tools.json` ([`crate::settings`]) ou as variáveis
//! `AYLIS_IOS_PROFILE` e `AYLIS_IOS_IDENTITY`.
//!
//! O que conversa com o macOS (`security`, `codesign`, `xcrun devicectl`) só roda num Mac; o
//! resto (ler o perfil, escolher, escrever os direitos e o `.ipa`) é testado em qualquer
//! sistema.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

/// O perfil de provisionamento fixado nas Opções (o caminho do `.mobileprovision`).
pub const PROFILE: &str = "ios_profile";
/// O certificado fixado nas Opções: o nome ("Apple Development: Fulano (X1Y2Z3)") ou a
/// impressão SHA-1, como o `security find-identity` mostra.
pub const IDENTITY: &str = "ios_sign_identity";
/// A versão mínima do iOS dos apps do Aylis (o `MinimumOSVersion` e o
/// `IPHONEOS_DEPLOYMENT_TARGET` da compilação).
pub const MINIMUM_IOS: &str = "14.0";

/// Um perfil de provisionamento lido.
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub path: PathBuf,
    pub name: String,
    pub uuid: String,
    /// A equipe (o prefixo do `application-identifier`).
    pub team: String,
    /// O bundle id que o perfil aceita, sem a equipe: `com.empresa.app` ou `com.empresa.*`/`*`.
    pub app_id: String,
    /// O fim da validade, em ISO 8601 (`2026-11-25T10:00:00Z`).
    pub expires: String,
    /// Os UDIDs dos aparelhos (vazio com `all_devices`, nos perfis de empresa e da App Store).
    pub devices: Vec<String>,
    pub all_devices: bool,
    /// O perfil de desenvolvimento (`get-task-allow`: o depurador pode se anexar).
    pub development: bool,
    /// O `aps-environment` do perfil (`development`/`production`), se ele tem o Push.
    pub push: Option<String>,
    /// As impressões SHA-1 (hexadecimal maiúsculo) dos certificados que o perfil aceita.
    pub certificates: Vec<String>,
}

/// Um certificado de assinatura do Keychain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    /// A impressão SHA-1, em hexadecimal maiúsculo.
    pub sha1: String,
    pub name: String,
}

/// Um aparelho conectado (pelo cabo ou pela rede), como o `devicectl` lista.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// O identificador do `devicectl` (`--device`).
    pub id: String,
    /// O UDID (o que o perfil lista).
    pub udid: String,
    pub name: String,
    /// O aparelho já foi pareado com este Mac (o "Confiar neste computador?").
    pub paired: bool,
}

/// Tudo o que a assinatura para o iPhone precisa.
#[derive(Clone, Debug)]
pub struct Signing {
    pub profile: Profile,
    pub identity: Identity,
}

// ---------------------------------------------------------------------------
// O perfil

/// Lê um `.mobileprovision`. O arquivo é uma assinatura CMS com o plist dentro, em texto: o
/// XML sai direto dos bytes, sem o `security cms` do macOS.
pub fn read_profile(path: &Path) -> Result<Profile, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut profile = parse_profile(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    profile.path = path.to_path_buf();
    Ok(profile)
}

/// O perfil a partir do conteúdo do `.mobileprovision` (ou do plist já extraído).
pub fn parse_profile(bytes: &[u8]) -> Result<Profile, String> {
    let start = find(bytes, b"<?xml").ok_or("não é um perfil de provisionamento (sem o plist)")?;
    let end = find(&bytes[start..], b"</plist>").ok_or("o plist do perfil está incompleto")? + start + b"</plist>".len();
    let text = std::str::from_utf8(&bytes[start..end]).map_err(|_| "o plist do perfil não é UTF-8")?;
    let plist = parse_plist(text)?;
    let s = |key: &str| plist.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let entitlements = plist.get("Entitlements").and_then(Value::as_object).cloned().unwrap_or_default();
    let application = entitlements.get("application-identifier").and_then(Value::as_str).unwrap_or_default();
    let team = plist
        .get("TeamIdentifier")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| application.split_once('.').map(|(t, _)| t.to_string()))
        .ok_or("o perfil não diz a equipe (TeamIdentifier)")?;
    let app_id = application.strip_prefix(&format!("{team}.")).unwrap_or(application).to_string();
    if app_id.is_empty() {
        return Err("o perfil não diz o bundle id (application-identifier)".into());
    }
    let list = |key: &str| -> Vec<String> { plist.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default() };
    let certificates = list("DeveloperCertificates").iter().filter_map(|b64| certificate_sha1(b64)).collect();
    Ok(Profile {
        path: PathBuf::new(),
        name: s("Name"),
        uuid: s("UUID"),
        team,
        app_id,
        expires: s("ExpirationDate"),
        devices: list("ProvisionedDevices"),
        all_devices: plist.get("ProvisionsAllDevices").and_then(Value::as_bool).unwrap_or(false),
        development: entitlements.get("get-task-allow").and_then(Value::as_bool).unwrap_or(false),
        push: entitlements.get("aps-environment").and_then(Value::as_str).map(str::to_string),
        certificates,
    })
}

/// A impressão SHA-1 de um certificado em base64 (o `<data>` do perfil).
fn certificate_sha1(b64: &str) -> Option<String> {
    use base64::Engine;
    use sha1::Digest;
    let clean: String = b64.chars().filter(|c| !c.is_whitespace()).collect();
    let der = base64::engine::general_purpose::STANDARD.decode(clean).ok()?;
    Some(sha1::Sha1::digest(&der).iter().map(|b| format!("{b:02X}")).collect())
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// O plist XML como JSON: `<dict>` vira objeto, `<array>` lista, `<date>` e `<data>` texto.
pub fn parse_plist(text: &str) -> Result<Map<String, Value>, String> {
    let doc = roxmltree::Document::parse_with_options(text, roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() }).map_err(|e| format!("plist inválido: {e}"))?;
    let root = doc.root_element().children().find(|n| n.is_element()).ok_or("plist vazio")?;
    match plist_value(root) {
        Value::Object(map) => Ok(map),
        _ => Err("o plist não começa por um <dict>".into()),
    }
}

fn plist_value(node: roxmltree::Node) -> Value {
    let elements = || node.children().filter(|n| n.is_element());
    match node.tag_name().name() {
        "dict" => {
            let mut map = Map::new();
            let mut key = None;
            for child in elements() {
                if child.tag_name().name() == "key" {
                    key = Some(child.text().unwrap_or_default().to_string());
                } else if let Some(k) = key.take() {
                    map.insert(k, plist_value(child));
                }
            }
            Value::Object(map)
        }
        "array" => Value::Array(elements().map(plist_value).collect()),
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "integer" => node.text().and_then(|t| t.trim().parse::<i64>().ok()).map(Value::from).unwrap_or(Value::Null),
        "real" => node.text().and_then(|t| t.trim().parse::<f64>().ok()).map(Value::from).unwrap_or(Value::Null),
        _ => Value::String(node.text().unwrap_or_default().trim().to_string()),
    }
}

/// O perfil aceita o bundle id: igual, ou pelo curinga do fim (`com.empresa.*`, `*`).
pub fn accepts_bundle(app_id: &str, bundle: &str) -> bool {
    match app_id.strip_suffix('*') {
        Some(prefix) => bundle.starts_with(prefix),
        None => app_id == bundle,
    }
}

/// Por que o perfil não serve para este app neste aparelho (`None`: serve).
pub fn profile_problem(profile: &Profile, bundle: &str, udid: Option<&str>, push: bool, now: &str) -> Option<String> {
    if !accepts_bundle(&profile.app_id, bundle) {
        return Some(format!("é do bundle id {}, e o app é {bundle}", profile.app_id));
    }
    if !profile.expires.is_empty() && profile.expires.as_str() <= now {
        return Some(format!("venceu em {}", profile.expires));
    }
    if let Some(udid) = udid {
        if !profile.all_devices && !profile.devices.iter().any(|d| d.eq_ignore_ascii_case(udid)) {
            return Some(format!("não tem o aparelho {udid}"));
        }
    }
    if push && profile.push.is_none() {
        return Some("não tem o Push (o app usa o PushNotifications)".into());
    }
    None
}

/// O melhor perfil para o app: entre os que servem, o do bundle id exato antes do curinga,
/// o de desenvolvimento antes dos outros (o Executar), e o que vence mais tarde. Sem nenhum,
/// o erro diz por que cada um ficou de fora.
pub fn choose_profile<'a>(profiles: &'a [Profile], bundle: &str, udid: Option<&str>, push: bool, now: &str) -> Result<&'a Profile, String> {
    let mut why = Vec::new();
    let mut best: Option<&Profile> = None;
    for p in profiles {
        if let Some(problem) = profile_problem(p, bundle, udid, push, now) {
            why.push(format!("  {} ({}): {problem}", p.name, p.path.display()));
            continue;
        }
        let rank = |p: &Profile| (!p.app_id.ends_with('*'), p.development, p.expires.clone());
        if best.is_none_or(|b| rank(p) > rank(b)) {
            best = Some(p);
        }
    }
    best.ok_or_else(|| {
        let mut text = format!("nenhum perfil de provisionamento serve para {bundle}");
        if why.is_empty() {
            text.push_str(" (nenhum foi achado)");
        } else {
            text.push_str(":\n");
            text.push_str(&why.join("\n"));
        }
        text.push_str(&format!("\n{}", PROFILE_HELP));
        text
    })
}

/// Como conseguir o perfil (a mensagem de quando falta).
pub const PROFILE_HELP: &str = "Para conseguir um: no Xcode, crie um projeto de app com o mesmo bundle id, escolha a sua equipe em Signing & Capabilities e rode uma vez no iPhone (com a conta paga, dá também para baixar do site da Apple). Ou aponte o arquivo em ios_profile, nas Opções (ou AYLIS_IOS_PROFILE).";

/// As pastas onde o Xcode grava os perfis (a do Xcode 16 em diante e a de antes).
pub fn profile_dirs(home: &Path) -> Vec<PathBuf> {
    vec![home.join("Library/Developer/Xcode/UserData/Provisioning Profiles"), home.join("Library/MobileDevice/Provisioning Profiles")]
}

/// Os perfis da máquina: o fixado nas Opções (só ele) ou os das pastas do Xcode.
pub fn machine_profiles() -> Result<Vec<Profile>, String> {
    if let Some(path) = setting(PROFILE, "AYLIS_IOS_PROFILE") {
        return read_profile(Path::new(&path)).map(|p| vec![p]);
    }
    let home = std::env::var_os("HOME").map(PathBuf::from).ok_or("sem a pasta do usuário (HOME)")?;
    let mut out = Vec::new();
    for dir in profile_dirs(&home) {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "mobileprovision") {
                if let Ok(p) = read_profile(&path) {
                    out.push(p);
                }
            }
        }
    }
    Ok(out)
}

/// Um valor das Opções, com a variável de ambiente na frente.
fn setting(key: &str, env: &str) -> Option<String> {
    std::env::var(env).ok().or_else(|| Some(crate::settings::tool_text(key))).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

// ---------------------------------------------------------------------------
// O certificado

/// Os certificados da saída do `security find-identity -v -p codesigning`:
/// `  1) 0123...CDEF "Apple Development: Fulano (X1Y2Z3)"`.
pub fn parse_identities(text: &str) -> Vec<Identity> {
    text.lines()
        .filter_map(|line| {
            let (_, rest) = line.trim().split_once(") ")?;
            let (sha1, name) = rest.split_once(' ')?;
            let name = name.trim().trim_matches('"');
            (sha1.len() == 40 && sha1.chars().all(|c| c.is_ascii_hexdigit())).then(|| Identity { sha1: sha1.to_ascii_uppercase(), name: name.to_string() })
        })
        .collect()
}

/// O certificado para o perfil: o fixado nas Opções, se ele está no Keychain, ou o do
/// Keychain que o perfil aceita (o mais novo, o último da lista do `security`).
pub fn choose_identity(identities: &[Identity], profile: &Profile, fixed: Option<&str>) -> Result<Identity, String> {
    let accepted = |i: &Identity| profile.certificates.is_empty() || profile.certificates.contains(&i.sha1);
    if let Some(fixed) = fixed {
        let found = identities.iter().find(|i| i.sha1.eq_ignore_ascii_case(fixed) || i.name == fixed).ok_or_else(|| format!("o certificado {fixed} (ios_sign_identity) não está no Keychain"))?;
        if !accepted(found) {
            return Err(format!("o perfil {} não aceita o certificado {} (gere o perfil de novo com ele)", profile.name, found.name));
        }
        return Ok(found.clone());
    }
    identities.iter().rev().find(|i| accepted(i)).cloned().ok_or_else(|| {
        if identities.is_empty() {
            "não há certificado de desenvolvedor no Keychain: no Xcode, Settings → Accounts, entre com o seu Apple ID e crie um em Manage Certificates".to_string()
        } else {
            let names: Vec<_> = identities.iter().map(|i| i.name.as_str()).collect();
            format!("o perfil {} não aceita nenhum certificado deste Mac ({}): gere o perfil de novo com o certificado desta máquina", profile.name, names.join(", "))
        }
    })
}

/// O certificado e o perfil para assinar este app (num Mac).
pub fn signing(bundle: &str, udid: Option<&str>, push: bool) -> Result<Signing, String> {
    let profiles = machine_profiles()?;
    let profile = choose_profile(&profiles, bundle, udid, push, &now_iso())?.clone();
    let out = std::process::Command::new("security").args(["find-identity", "-v", "-p", "codesigning"]).output().map_err(|e| format!("security: {e}"))?;
    let identities = parse_identities(&String::from_utf8_lossy(&out.stdout));
    let identity = choose_identity(&identities, &profile, setting(IDENTITY, "AYLIS_IOS_IDENTITY").as_deref())?;
    Ok(Signing { profile, identity })
}

// ---------------------------------------------------------------------------
// Os direitos e o pacote

/// Os direitos (entitlements) do app assinado com o perfil: os que o iOS confere contra ele
/// (a equipe, o app, o depurador) e o Push, se o app usa.
pub fn entitlements(profile: &Profile, bundle: &str, push: bool) -> String {
    let team = &profile.team;
    let mut body = format!(
        "  <key>application-identifier</key><string>{team}.{bundle}</string>\n  <key>com.apple.developer.team-identifier</key><string>{team}</string>\n  <key>get-task-allow</key><{}/>\n",
        profile.development
    );
    if push {
        if let Some(env) = &profile.push {
            body.push_str(&format!("  <key>aps-environment</key><string>{env}</string>\n"));
        }
    }
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n{body}</dict>\n</plist>\n")
}

/// O `.ipa`: a pasta `Payload/` com o `.app`, num zip. É o que o Apple Configurator, o
/// Transporter (o TestFlight) e as ferramentas de instalação aceitam. O executável vai com a
/// permissão de execução, senão o iOS não abre o app.
pub fn write_ipa(app: &Path, executable: &str, ipa: &Path) -> Result<(), String> {
    use std::io::Write;
    let app_name = app.file_name().ok_or("o .app sem nome")?.to_string_lossy().to_string();
    let file = std::fs::File::create(ipa).map_err(|e| format!("{}: {e}", ipa.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    // Marcado como Unix: gravado no Windows, o zip seria do DOS e a permissão se perderia.
    let deflate = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).system(zip::System::Unix);
    let mut stack = vec![app.to_path_buf()];
    let mut files = Vec::new();
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    for path in files {
        let relative = path.strip_prefix(app).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
        let mode = if relative == executable { 0o755 } else { 0o644 };
        zip.start_file(format!("Payload/{app_name}/{relative}"), deflate.unix_permissions(mode)).map_err(|e| e.to_string())?;
        zip.write_all(&std::fs::read(&path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// O aparelho

/// Os aparelhos do JSON do `xcrun devicectl list devices --json-output`. Só os iPhones e
/// iPads (o relógio e a TV ficam de fora).
pub fn parse_devices(json: &str) -> Vec<Device> {
    let Ok(v) = serde_json::from_str::<Value>(json) else { return Vec::new() };
    let devices = v.pointer("/result/devices").and_then(Value::as_array).cloned().unwrap_or_default();
    devices
        .iter()
        .filter(|d| d.pointer("/hardwareProperties/platform").and_then(Value::as_str).is_none_or(|p| p == "iOS"))
        .filter_map(|d| {
            let text = |p: &str| d.pointer(p).and_then(Value::as_str).unwrap_or_default().to_string();
            let udid = text("/hardwareProperties/udid");
            let id = Some(text("/identifier")).filter(|s| !s.is_empty()).unwrap_or_else(|| udid.clone());
            (!udid.is_empty()).then(|| Device { id, udid, name: text("/deviceProperties/name"), paired: text("/connectionProperties/pairingState") == "paired" })
        })
        .collect()
}

/// Os aparelhos conectados a este Mac (o `devicectl` do Xcode 15 em diante).
pub fn devices() -> Result<Vec<Device>, String> {
    let file = std::env::temp_dir().join(format!("aylis-devicectl-{}.json", std::process::id()));
    let out = std::process::Command::new("xcrun").args(["devicectl", "list", "devices", "--quiet", "--json-output"]).arg(&file).output().map_err(|e| format!("xcrun devicectl: {e}"))?;
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    let _ = std::fs::remove_file(&file);
    if !out.status.success() && text.is_empty() {
        return Err(format!("xcrun devicectl: {} (precisa do Xcode 15 ou mais novo)", String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(parse_devices(&text))
}

/// O aparelho pedido (pelo UDID, pelo identificador ou pelo nome) ou, sem pedido, o único
/// pareado.
pub fn choose_device(devices: &[Device], wanted: Option<&str>) -> Result<Device, String> {
    let wanted = wanted.map(str::trim).filter(|w| !w.is_empty());
    if let Some(w) = wanted {
        return devices
            .iter()
            .find(|d| d.udid.eq_ignore_ascii_case(w) || d.id.eq_ignore_ascii_case(w) || d.name == w)
            .cloned()
            .ok_or_else(|| format!("o aparelho {w} não está conectado"));
    }
    let paired: Vec<_> = devices.iter().filter(|d| d.paired).collect();
    match paired.as_slice() {
        [one] => Ok((*one).clone()),
        [] if devices.is_empty() => Err("nenhum iPhone conectado: ligue pelo cabo, desbloqueie e toque em \"Confiar\"".into()),
        [] => Err(format!("o {} não foi pareado com este Mac: desbloqueie e toque em \"Confiar\" (e ligue o Modo de Desenvolvedor em Ajustes → Privacidade e Segurança)", devices[0].name)),
        many => Err(format!("há mais de um aparelho ({}): escolha com --serial <udid>", many.iter().map(|d| d.name.as_str()).collect::<Vec<_>>().join(", "))),
    }
}

// ---------------------------------------------------------------------------

/// Agora, em ISO 8601 UTC (`2026-11-18T13:05:00Z`), para comparar com a validade do perfil.
pub fn now_iso() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    iso_from_unix(secs)
}

/// Segundos desde 1970 em ISO 8601 UTC (o calendário civil de Howard Hinnant).
pub fn iso_from_unix(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rest = secs % 86_400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", rest / 3600, rest / 60 % 60, rest % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Um perfil de desenvolvimento como o do Xcode, embrulhado em bytes que não são texto
    /// (como na assinatura CMS de verdade).
    fn profile_bytes(app: &str, expires: &str, devices: &[&str], push: bool, cert: &[u8]) -> Vec<u8> {
        use base64::Engine;
        let devices: String = devices.iter().map(|d| format!("<string>{d}</string>")).collect();
        let push = if push { "<key>aps-environment</key><string>development</string>" } else { "" };
        let cert = base64::engine::general_purpose::STANDARD.encode(cert);
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>AppIDName</key><string>XC com empresa app</string>
  <key>CreationDate</key><date>2026-11-01T10:00:00Z</date>
  <key>DeveloperCertificates</key><array><data>{cert}</data></array>
  <key>Entitlements</key>
  <dict>
    <key>application-identifier</key><string>AB12CD34EF.{app}</string>
    <key>get-task-allow</key><true/>
    <key>keychain-access-groups</key><array><string>AB12CD34EF.*</string></array>
    {push}
  </dict>
  <key>ExpirationDate</key><date>{expires}</date>
  <key>Name</key><string>iOS Team Provisioning Profile: {app}</string>
  <key>ProvisionedDevices</key><array>{devices}</array>
  <key>TeamIdentifier</key><array><string>AB12CD34EF</string></array>
  <key>TimeToLive</key><integer>7</integer>
  <key>UUID</key><string>1111-2222</string>
</dict>
</plist>"#
        );
        let mut bytes = vec![0x30, 0x82, 0x1f, 0x00, 0x06, 0x09, 0xff];
        bytes.extend_from_slice(xml.as_bytes());
        bytes.extend_from_slice(&[0xa0, 0x82, 0x00, 0x01]);
        bytes
    }

    #[test]
    fn the_profile_is_read_from_the_signed_file() {
        let p = parse_profile(&profile_bytes("com.empresa.app", "2026-11-25T10:00:00Z", &["00008110-000A1B2C3D4E"], true, b"certificado")).unwrap();
        assert_eq!(p.team, "AB12CD34EF");
        assert_eq!(p.app_id, "com.empresa.app");
        assert_eq!(p.expires, "2026-11-25T10:00:00Z");
        assert_eq!(p.devices, ["00008110-000A1B2C3D4E"]);
        assert!(p.development && !p.all_devices);
        assert_eq!(p.push.as_deref(), Some("development"));
        assert_eq!(p.uuid, "1111-2222");
        // O SHA-1 de "certificado".
        use sha1::Digest;
        let expected: String = sha1::Sha1::digest(b"certificado").iter().map(|b| format!("{b:02X}")).collect();
        assert_eq!(p.certificates, [expected]);
    }

    #[test]
    fn the_profile_is_chosen_by_bundle_device_and_validity() {
        let now = "2026-11-18T12:00:00Z";
        let udid = "00008110-000A1B2C3D4E";
        let with = |app: &str, expires: &str, devices: &[&str], name: &str| {
            let mut p = parse_profile(&profile_bytes(app, expires, devices, false, b"c")).unwrap();
            p.name = name.into();
            p
        };
        let profiles = vec![
            with("com.empresa.outro", "2027-01-01T00:00:00Z", &[udid], "outro app"),
            with("com.empresa.app", "2026-11-10T00:00:00Z", &[udid], "vencido"),
            with("com.empresa.*", "2027-06-01T00:00:00Z", &[udid], "curinga"),
            with("com.empresa.app", "2026-12-01T00:00:00Z", &[udid], "exato"),
            with("com.empresa.app", "2027-12-01T00:00:00Z", &["OUTRO"], "outro aparelho"),
        ];
        // O exato antes do curinga, mesmo vencendo antes; o vencido e o de outro aparelho, fora.
        assert_eq!(choose_profile(&profiles, "com.empresa.app", Some(udid), false, now).unwrap().name, "exato");
        // Sem aparelho (o publish sem o iPhone ligado), o de outro aparelho serve e vence depois.
        assert_eq!(choose_profile(&profiles, "com.empresa.app", None, false, now).unwrap().name, "outro aparelho");
        // Só o curinga aceita outro app da empresa.
        assert_eq!(choose_profile(&profiles, "com.empresa.novo", Some(udid), false, now).unwrap().name, "curinga");
        // Push: nenhum tem; o erro diz o motivo de cada um e como conseguir.
        let err = choose_profile(&profiles, "com.empresa.app", Some(udid), true, now).unwrap_err();
        assert!(err.contains("vencido") && err.contains("venceu em 2026-11-10"), "{err}");
        assert!(err.contains("não tem o Push") && err.contains("Xcode"), "{err}");
        assert!(accepts_bundle("*", "qualquer.coisa"));
        assert!(!accepts_bundle("com.empresa.app", "com.empresa.app2"));
    }

    #[test]
    fn the_identity_is_the_one_the_profile_accepts() {
        let text = "  1) 1111111111111111111111111111111111111111 \"Apple Development: Fulano (X1)\"\n  2) aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \"Apple Development: Fulano (X2)\"\n     2 valid identities found\n";
        let ids = parse_identities(text);
        assert_eq!(ids.len(), 2);
        assert_eq!(ids[1].sha1, "A".repeat(40));
        let mut profile = parse_profile(&profile_bytes("com.empresa.app", "2027-01-01T00:00:00Z", &[], false, b"c")).unwrap();
        profile.certificates = vec!["1".repeat(40)];
        assert_eq!(choose_identity(&ids, &profile, None).unwrap().name, "Apple Development: Fulano (X1)");
        // O fixado nas Opções que o perfil não aceita é um erro (e não outro em silêncio).
        assert!(choose_identity(&ids, &profile, Some("Apple Development: Fulano (X2)")).unwrap_err().contains("não aceita"));
        assert!(choose_identity(&[], &profile, None).unwrap_err().contains("Keychain"));
    }

    #[test]
    fn the_entitlements_follow_the_profile() {
        let profile = parse_profile(&profile_bytes("com.empresa.*", "2027-01-01T00:00:00Z", &[], true, b"c")).unwrap();
        let text = entitlements(&profile, "com.empresa.app", true);
        assert!(text.contains("<string>AB12CD34EF.com.empresa.app</string>"), "{text}");
        assert!(text.contains("<key>get-task-allow</key><true/>"));
        assert!(text.contains("<key>aps-environment</key><string>development</string>"));
        assert!(!entitlements(&profile, "com.empresa.app", false).contains("aps-environment"));
        // O que sai é um plist que o próprio leitor entende.
        let back = parse_plist(&text).unwrap();
        assert_eq!(back["com.apple.developer.team-identifier"], "AB12CD34EF");
    }

    #[test]
    fn the_device_comes_from_devicectl() {
        let json = r#"{"result":{"devices":[
            {"identifier":"AAAA-1","deviceProperties":{"name":"iPhone do Eduardo"},"hardwareProperties":{"platform":"iOS","udid":"00008110-000A"},"connectionProperties":{"pairingState":"paired"}},
            {"identifier":"BBBB-2","deviceProperties":{"name":"Relógio"},"hardwareProperties":{"platform":"watchOS","udid":"00008301-000B"},"connectionProperties":{"pairingState":"paired"}},
            {"identifier":"CCCC-3","deviceProperties":{"name":"iPad"},"hardwareProperties":{"platform":"iOS","udid":"00008027-000C"},"connectionProperties":{"pairingState":"unpaired"}}
        ]}}"#;
        let devices = parse_devices(json);
        assert_eq!(devices.len(), 2);
        assert_eq!(choose_device(&devices, None).unwrap().id, "AAAA-1");
        assert_eq!(choose_device(&devices, Some("00008027-000c")).unwrap().name, "iPad");
        assert!(choose_device(&[], None).unwrap_err().contains("Confiar"));
        assert!(choose_device(&devices[1..], None).unwrap_err().contains("pareado"));
    }

    #[test]
    fn the_ipa_has_the_app_in_payload_with_the_executable_runnable() {
        let dir = std::env::temp_dir().join(format!("aylis-ipa-test-{}", std::process::id()));
        let app = dir.join("Meu App.app");
        std::fs::create_dir_all(app.join("Assets")).unwrap();
        std::fs::write(app.join("meu_app"), b"\xcf\xfa\xed\xfe").unwrap();
        std::fs::write(app.join("Info.plist"), b"<plist/>").unwrap();
        std::fs::write(app.join("Assets/x.png"), b"png").unwrap();
        let ipa = dir.join("Meu App.ipa");
        write_ipa(&app, "meu_app", &ipa).unwrap();
        let mut zip = zip::ZipArchive::new(std::fs::File::open(&ipa).unwrap()).unwrap();
        let mut names: Vec<String> = (0..zip.len()).map(|i| zip.by_index(i).unwrap().name().unwrap().to_string()).collect();
        names.sort();
        assert_eq!(names, ["Payload/Meu App.app/Assets/x.png", "Payload/Meu App.app/Info.plist", "Payload/Meu App.app/meu_app"]);
        assert_eq!(zip.by_name("Payload/Meu App.app/meu_app").unwrap().unix_mode().map(|m| m & 0o777), Some(0o755));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_date_is_iso_utc() {
        assert_eq!(iso_from_unix(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_from_unix(1_795_000_000), "2026-11-18T11:06:40Z");
        assert_eq!(iso_from_unix(951_782_400), "2000-02-29T00:00:00Z");
    }
}
