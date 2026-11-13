//! De onde vêm os arquivos dos pacotes: o cache, as pastas de pacotes e os espelhos (na
//! ordem), e o download (prazo, proxy, o SHA-256 conferido no caminho, o Google Drive).

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::{FileSpec, Progress, hex, sha256_file};
use crate::process::Control;

/// O endereço de download direto do Google Drive.
const DRIVE: &str = "https://drive.usercontent.google.com/download";
/// A vista embutida de uma pasta pública do Drive: uma página simples com o nome e o id de cada
/// arquivo, sem chave de API.
const DRIVE_FOLDER: &str = "https://drive.google.com/embeddedfolderview?id=";

/// O id da pasta num link de pasta do Drive (`https://drive.google.com/drive/folders/<id>`,
/// também com `/u/0/` e `?usp=sharing`). Outro endereço: `None` (um `pacotes.json`).
pub(crate) fn drive_folder_id(url: &str) -> Option<String> {
    let rest = url.strip_prefix("https://drive.google.com/")?;
    let id = rest.split("/folders/").nth(1)?;
    let id: String = id.chars().take_while(|c| c.is_ascii_alphanumeric() || "-_".contains(*c)).collect();
    (!id.is_empty()).then_some(id)
}

/// O índice de uma pasta do Drive pela página da vista embutida: `{"files": {"<nome>":
/// {"drive": "<id>"}}}`. Só os arquivos (as subpastas não entram); o hash que vale continua o
/// do manifesto.
pub(crate) fn drive_folder_index(page: &str) -> Value {
    const LINK: &str = "https://drive.google.com/file/d/";
    const TITLE: &str = "class=\"flip-entry-title\">";
    let mut files = serde_json::Map::new();
    for part in page.split(LINK).skip(1) {
        let id: String = part.chars().take_while(|c| c.is_ascii_alphanumeric() || "-_".contains(*c)).collect();
        let Some(at) = part.find(TITLE) else { continue };
        let name = &part[at + TITLE.len()..];
        let name = name[..name.find('<').unwrap_or(name.len())].trim();
        let name = name.replace("&amp;", "&").replace("&#39;", "'").replace("&quot;", "\"").replace("&lt;", "<").replace("&gt;", ">");
        if !id.is_empty() && !name.is_empty() {
            files.insert(name, serde_json::json!({ "drive": id }));
        }
    }
    serde_json::json!({ "aylis": 1, "files": files })
}

/// As origens dos arquivos, fora o cache (que vem sempre primeiro).
#[derive(Clone, Debug)]
pub struct Sources {
    /// Pastas com os arquivos pelo nome (a `pacotes` ao lado do executável, as das Opções,
    /// uma pasta de rede, a do Google Drive para desktop).
    pub folders: Vec<PathBuf>,
    /// Endereços HTTPS de um `pacotes.json` (o índice diz o lugar de cada arquivo).
    /// Não há a URL oficial do manifesto como última origem (decisão do autor): só o `pack` e
    /// o `update` a usam.
    pub mirrors: Vec<String>,
    /// O download direto do Drive (os testes trocam por um servidor local).
    pub(crate) drive: String,
    /// Os índices já lidos, por espelho (uma ida por espelho, não por arquivo).
    pub(crate) indexes: Arc<Mutex<HashMap<String, Result<Value, String>>>>,
}

impl Default for Sources {
    fn default() -> Self {
        Sources { folders: Vec::new(), mirrors: Vec::new(), drive: DRIVE.into(), indexes: Arc::default() }
    }
}

impl Sources {
    /// As de sempre: a pasta de `AYLIS_PACKAGE_DIR` (o instalador da IDE), a pasta `pacotes`
    /// ao lado do executável (a IDE), as pastas e os espelhos das Opções (`settings`:
    /// `package_dirs`, `package_mirrors`; sem nada gravado, o espelho padrão).
    pub fn configured() -> Sources {
        let mut s = Sources::default();
        // A pasta `pacotes` ao lado do instalador da IDE (o kit offline): o instalador a passa
        // ao `aylis-tool setup` por esta variável, sem gravá-la nas Opções.
        s.folders.extend(std::env::var_os("AYLIS_PACKAGE_DIR").filter(|d| !d.is_empty()).map(PathBuf::from));
        s.folders.extend(Sources::beside_exe());
        s.folders.extend(crate::settings::list(crate::settings::PACKAGE_DIRS).into_iter().map(PathBuf::from));
        s.mirrors.extend(crate::settings::list(crate::settings::PACKAGE_MIRRORS));
        s
    }

    /// A pasta `pacotes` ao lado do executável (o instalador ou a IDE), exista ou não.
    pub fn beside_exe() -> Option<PathBuf> {
        std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.join("pacotes")))
    }

    /// Há de onde obter (uma pasta que existe ou um espelho)? Sem nenhuma, só o cache.
    pub fn has_any(&self) -> bool {
        self.folders.iter().any(|f| f.is_dir()) || !self.mirrors.is_empty()
    }

    /// Só estas pastas, sem espelho nem rede.
    pub fn local(folders: Vec<PathBuf>) -> Sources {
        Sources { folders, ..Sources::default() }
    }

    /// As origens em texto, na ordem (o `aylis-tool packages sources` e a tela das Opções).
    pub fn describe(&self, cache: &Path) -> Vec<String> {
        let mut out = vec![format!("cache: {}", cache.display())];
        for f in &self.folders {
            out.push(format!("pasta: {}{}", f.display(), if f.is_dir() { "" } else { " (não existe)" }));
        }
        for m in &self.mirrors {
            out.push(format!("espelho: {m}"));
        }
        out
    }

    /// Quantos de `files` cada origem tem (o "Testar as origens" das Opções): numa pasta, os
    /// arquivos com o nome e o tamanho fixados; num espelho, os que o índice dele lista. Não
    /// baixa os arquivos nem confere o hash (a instalação confere).
    pub fn probe(&self, files: &[FileSpec], control: &Control) -> Vec<Probe> {
        let total = files.len();
        let mut out = Vec::new();
        for folder in &self.folders {
            let place = folder.display().to_string();
            if !folder.is_dir() {
                out.push(Probe { mirror: false, place, found: 0, total, error: Some("a pasta não existe".into()) });
                continue;
            }
            let found = files.iter().filter(|f| std::fs::metadata(folder.join(&f.name)).is_ok_and(|m| m.is_file() && (f.size == 0 || m.len() == f.size))).count();
            out.push(Probe { mirror: false, place, found, total, error: None });
        }
        for mirror in &self.mirrors {
            let probe = match self.index(mirror, control) {
                Ok(index) => {
                    let found = files.iter().filter(|f| index["files"].get(&f.name).is_some_and(|e| e["url"].is_string() || e["drive"].is_string())).count();
                    Probe { mirror: true, place: mirror.clone(), found, total, error: None }
                }
                Err(e) => Probe { mirror: true, place: mirror.clone(), found: 0, total, error: Some(e) },
            };
            out.push(probe);
        }
        out
    }
}

/// O que uma origem tem ([`Sources::probe`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Probe {
    /// Um espelho (senão, uma pasta).
    pub mirror: bool,
    /// A pasta ou o endereço.
    pub place: String,
    pub found: usize,
    pub total: usize,
    /// Por que não deu para ler (a pasta não existe, o índice não respondeu).
    pub error: Option<String>,
}

impl Sources {

    /// O índice de um espelho (lido uma vez).
    fn index(&self, mirror: &str, control: &Control) -> Result<Value, String> {
        if let Some(v) = self.indexes.lock().ok().and_then(|m| m.get(mirror).cloned()) {
            return v;
        }
        let v = match drive_folder_id(mirror) {
            // O link de uma pasta compartilhada do Drive: a lista dela vira o índice (o nome de
            // cada arquivo e o id dele), sem precisar de um `pacotes.json` com os ids.
            Some(folder) => get_text(&format!("{DRIVE_FOLDER}{folder}"), control).and_then(|page| {
                let index = drive_folder_index(&page);
                if index["files"].as_object().is_some_and(|f| !f.is_empty()) {
                    Ok(index)
                } else {
                    Err("a pasta do Drive não lista nenhum arquivo (está compartilhada como \"qualquer pessoa com o link\"?)".into())
                }
            }),
            None => get_text(mirror, control).and_then(|t| serde_json::from_str::<Value>(&t).map_err(|e| format!("o índice não é JSON: {e}"))),
        };
        if let Ok(mut m) = self.indexes.lock() {
            m.insert(mirror.to_string(), v.clone());
        }
        v
    }
}

/// O arquivo no cache, conferido: do cache, de uma pasta ou de um espelho (nunca da URL
/// oficial: decisão do autor). Um erro diz o arquivo e o que cada origem respondeu.
pub(crate) fn obtain(file: &FileSpec, sources: &Sources, cache: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<PathBuf, String> {
    if !super::is_sha256(&file.sha256) {
        return Err(format!("{}: o manifesto não tem o SHA-256 (rode aylis-tool packages update)", file.name));
    }
    std::fs::create_dir_all(cache).map_err(|e| format!("{}: {e}", cache.display()))?;
    let dest = cache.join(&file.name);
    let mut tried = Vec::new();
    if dest.is_file() {
        match sha256_file(&dest) {
            Ok(h) if h == file.sha256 => return Ok(dest),
            Ok(h) => {
                tried.push(format!("cache: o arquivo que estava lá não confere (SHA-256 {h}); foi apagado"));
                let _ = std::fs::remove_file(&dest);
            }
            Err(e) => tried.push(format!("cache: {e}")),
        }
    }
    for folder in &sources.folders {
        control.check()?;
        let candidate = folder.join(&file.name);
        if !candidate.is_file() {
            tried.push(format!("pasta {}: não está", folder.display()));
            continue;
        }
        let from = candidate.display().to_string();
        progress(Progress::Step(format!("Copiando {} de {}…", file.name, folder.display())));
        match File::open(&candidate).map_err(|e| e.to_string()).and_then(|f| save_verified(f, file, &dest, &from, control, progress)) {
            Ok(()) => return Ok(dest),
            Err(e) if e == crate::process::CANCELLED => return Err(e),
            Err(e) => tried.push(format!("pasta {}: {e}", folder.display())),
        }
    }
    for mirror in &sources.mirrors {
        control.check()?;
        match from_mirror(mirror, file, sources, &dest, control, progress) {
            Ok(()) => return Ok(dest),
            Err(e) if e == crate::process::CANCELLED => return Err(e),
            Err(e) => tried.push(format!("espelho {mirror}: {e}")),
        }
    }
    let hint = if !sources.has_any() {
        "\n  Nenhuma origem configurada: indique uma pasta de pacotes ou um espelho (Opções da IDE → Pacotes, ou aylis-tool packages sources)."
    } else {
        ""
    };
    // Sem nada tentado (nenhuma origem e nada no cache), sem a lista vazia.
    let tried = if tried.is_empty() { String::new() } else { format!(":\n  - {}", tried.join("\n  - ")) };
    Err(format!("{}: não achei o arquivo com o SHA-256 fixado em nenhuma origem{tried}{hint}", file.name))
}

/// O arquivo pelo índice de um espelho: `{"files": {"<nome>": {"url": "..."} | {"drive": "<id>"}}}`.
fn from_mirror(mirror: &str, file: &FileSpec, sources: &Sources, dest: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    let index = sources.index(mirror, control)?;
    let entry = &index["files"][&file.name];
    if let Some(id) = entry["drive"].as_str() {
        if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || "-_".contains(c)) {
            return Err(format!("id do Drive inválido para {}: {id:?}", file.name));
        }
        progress(Progress::Step(format!("Baixando {} ({} MB) do Google Drive…", file.name, file.megabytes())));
        return drive_download(&sources.drive, id, file, dest, control, progress);
    }
    let Some(url) = entry["url"].as_str() else { return Err("o índice não tem este arquivo".into()) };
    let url = resolve(mirror, url);
    progress(Progress::Step(format!("Baixando {} ({} MB) de {url}…", file.name, file.megabytes())));
    download_verified(&url, file, dest, control, progress)
}

/// Um endereço do índice relativo ao próprio índice (`jdk.zip` ao lado do `pacotes.json`).
fn resolve(index: &str, url: &str) -> String {
    if url.contains("://") {
        return url.to_string();
    }
    let base = index.rsplit_once('/').map(|(b, _)| b).unwrap_or(index);
    format!("{base}/{}", url.trim_start_matches('/'))
}

/// HTTPS; HTTP só para esta máquina (um servidor local, os testes).
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn allowed_url(url: &str) -> bool {
    url.starts_with("https://") || ["http://127.0.0.1:", "http://127.0.0.1/", "http://localhost:", "http://localhost/", "http://[::1]:"].iter().any(|p| url.starts_with(p))
}

/// Lê `reader` para `dest` (pelo `.part`), conferindo o SHA-256 e o tamanho no caminho: o que
/// não confere é apagado e não é usado. Para quando passa do tamanho fixado (um espelho que
/// manda sem fim não enche o disco).
pub(crate) fn save_verified(mut reader: impl Read, file: &FileSpec, dest: &Path, from: &str, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    let got = save_hashing(&mut reader, file, dest, from, control, progress)?;
    if got.0 != file.sha256 {
        let _ = std::fs::remove_file(dest);
        return Err(format!("não confere (SHA-256 {}, o fixado é {}): não foi usado", got.0, file.sha256));
    }
    Ok(())
}

/// Grava `reader` em `dest` (pelo `.part`) e devolve o SHA-256 e o tamanho.
pub(crate) fn save_hashing(reader: &mut dyn Read, file: &FileSpec, dest: &Path, from: &str, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(String, u64), String> {
    let part = dest.with_file_name(format!("{}.part-{}", file.name, std::process::id()));
    let result = (|| {
        let mut out = File::create(&part).map_err(|e| format!("{}: {e}", part.display()))?;
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 1 << 16];
        let mut done = 0u64;
        let mut reported = 0u64;
        loop {
            control.check()?;
            let n = reader.read(&mut buf).map_err(|e| format!("a leitura parou: {e}"))?;
            if n == 0 {
                break;
            }
            done += n as u64;
            if file.size > 0 && done > file.size && super::is_sha256(&file.sha256) {
                return Err(format!("maior que os {} bytes fixados: interrompido", file.size));
            }
            hasher.update(&buf[..n]);
            out.write_all(&buf[..n]).map_err(|e| format!("{}: {e}", part.display()))?;
            if done - reported >= 4 << 20 {
                reported = done;
                progress(Progress::Bytes { file: file.name.clone(), from: from.to_string(), done, total: file.size });
            }
        }
        out.flush().map_err(|e| e.to_string())?;
        progress(Progress::Bytes { file: file.name.clone(), from: from.to_string(), done, total: file.size.max(done) });
        Ok((hex(&hasher.finalize()), done))
    })();
    match result {
        Ok(got) => {
            if got.0 == file.sha256 || !super::is_sha256(&file.sha256) {
                std::fs::rename(&part, dest).map_err(|e| format!("{}: {e}", dest.display()))?;
            } else {
                let _ = std::fs::remove_file(&part);
            }
            Ok(got)
        }
        Err(e) => {
            let _ = std::fs::remove_file(&part);
            Err(e)
        }
    }
}

/// Baixa `url` em `dest` conferindo o hash.
pub(crate) fn download_verified(url: &str, file: &FileSpec, dest: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    let body = get(url)?;
    save_verified(body, file, dest, url, control, progress)
}

/// O download direto do Drive. Num arquivo grande ele responde uma página ("não deu para
/// verificar se há vírus") com um formulário; o Aylis manda o mesmo formulário (o `confirm`
/// e os campos escondidos) para o mesmo endereço do Drive, e só para ele.
fn drive_download(base: &str, id: &str, file: &FileSpec, dest: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    let first = format!("{base}?id={id}&export=download");
    let response = get_response(&first)?;
    if !response.html {
        return save_verified(response.body, file, dest, "Google Drive", control, progress);
    }
    let page = read_limited(response.body, 1 << 20)?;
    let (action, fields) = drive_form(&page).ok_or("o Drive respondeu uma página sem o arquivo (ele está compartilhado como \"qualquer pessoa com o link\"?)")?;
    if !same_origin(&action, base) {
        return Err(format!("o Drive mandou confirmar em outro endereço ({action}): recusado"));
    }
    let query: Vec<String> = fields.iter().map(|(k, v)| format!("{}={}", encode(k), encode(v))).collect();
    let second = format!("{action}?{}", query.join("&"));
    let response = get_response(&second)?;
    if response.html {
        return Err("o Drive pediu a confirmação de novo (cota de downloads do arquivo esgotada? tente mais tarde)".into());
    }
    save_verified(response.body, file, dest, "Google Drive", control, progress)
}

/// O formulário de confirmação do Drive: o `action` e os campos escondidos.
pub(crate) fn drive_form(page: &str) -> Option<(String, Vec<(String, String)>)> {
    let start = page.find("<form")?;
    let form = &page[start..page[start..].find("</form>").map_or(page.len(), |e| start + e)];
    let tag_end = form.find('>')?;
    let action = attr(&form[..tag_end], "action")?.replace("&amp;", "&");
    let mut fields = Vec::new();
    let mut rest = form;
    while let Some(i) = rest.find("<input") {
        let tag = &rest[i..];
        let end = tag.find('>').unwrap_or(tag.len());
        let tag = &tag[..end];
        if attr(tag, "type").as_deref() == Some("hidden")
            && let Some(name) = attr(tag, "name")
        {
            fields.push((name, attr(tag, "value").unwrap_or_default().replace("&amp;", "&")));
        }
        rest = &rest[i + end..];
    }
    fields.iter().any(|(k, _)| k == "id").then_some((action, fields))
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let key = format!("{name}=\"");
    let i = tag.find(&key)? + key.len();
    Some(tag[i..].split('"').next()?.to_string())
}

fn same_origin(a: &str, b: &str) -> bool {
    let origin = |u: &str| -> Option<String> {
        let (scheme, rest) = u.split_once("://")?;
        Some(format!("{scheme}://{}", rest.split('/').next()?))
    };
    origin(a).is_some() && origin(a) == origin(b)
}

fn encode(s: &str) -> String {
    s.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") }).collect()
}

fn read_limited(mut r: impl Read, limit: u64) -> Result<String, String> {
    let mut s = String::new();
    r.by_ref().take(limit).read_to_string(&mut s).map_err(|e| e.to_string())?;
    Ok(s)
}

fn get_text(url: &str, _control: &Control) -> Result<String, String> {
    read_limited(get(url)?, 16 << 20)
}

// ---------------------------------------------------------------------------
// HTTP (no navegador, a IDE lite, não se baixa nada).

struct Response {
    body: Box<dyn Read>,
    html: bool,
}

pub(super) fn get(url: &str) -> Result<Box<dyn Read>, String> {
    get_response(url).map(|r| r.body)
}

#[cfg(target_arch = "wasm32")]
fn get_response(url: &str) -> Result<Response, String> {
    Err(format!("{url}: baixar não existe no navegador"))
}

#[cfg(not(target_arch = "wasm32"))]
fn get_response(url: &str) -> Result<Response, String> {
    if !allowed_url(url) {
        return Err(format!("{url}: só https (ou http nesta máquina)"));
    }
    let local = !url.starts_with("https://");
    let response = agent(local).get(url).call().map_err(|e| describe(&e))?;
    let html = response.headers().get("content-type").and_then(|v| v.to_str().ok()).is_some_and(|v| v.starts_with("text/html"));
    Ok(Response { body: Box::new(response.into_body().into_reader()), html })
}

/// O erro do `ureq` em português, sem perder o detalhe.
#[cfg(not(target_arch = "wasm32"))]
fn describe(e: &ureq::Error) -> String {
    match e {
        ureq::Error::StatusCode(404) => "não encontrado (HTTP 404)".into(),
        ureq::Error::StatusCode(403) => "acesso negado (HTTP 403)".into(),
        ureq::Error::StatusCode(c) => format!("o servidor respondeu HTTP {c}"),
        ureq::Error::Timeout(t) => format!("passou do prazo ({t})"),
        ureq::Error::HostNotFound => "endereço não encontrado (sem rede ou DNS?)".into(),
        ureq::Error::ConnectionFailed => "não conectou (sem rede, firewall ou proxy?)".into(),
        other => other.to_string(),
    }
}

/// Prazos: 20 s para conectar, 60 s para a resposta e para cada leitura (um arquivo grande
/// leva o tempo que for, mas uma conexão parada não segura para sempre). O proxy: o das
/// variáveis (`HTTPS_PROXY`, `ALL_PROXY`, `NO_PROXY`; o ureq lê) ou, no Windows, o das
/// configurações de Internet do usuário.
#[cfg(not(target_arch = "wasm32"))]
fn agent(local: bool) -> ureq::Agent {
    use std::time::Duration;
    let mut config = ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .timeout_per_read(Some(Duration::from_secs(60)))
        .user_agent("aylis-tool");
    if local {
        config = config.proxy(None);
    } else if ureq::Proxy::try_from_env().is_none()
        && let Some(proxy) = system_proxy()
    {
        config = config.proxy(Some(proxy));
    }
    config.build().into()
}

/// O proxy das configurações de Internet do Windows (`ProxyEnable`/`ProxyServer`).
#[cfg(not(target_arch = "wasm32"))]
fn system_proxy() -> Option<ureq::Proxy> {
    if !cfg!(windows) {
        return None;
    }
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    let value = |name: &str| -> Option<String> {
        let out = crate::process::quiet(&mut std::process::Command::new("reg")).args(["query", key, "/v", name]).output().ok()?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        text.lines().find_map(|l| {
            let mut parts = l.split_whitespace();
            (parts.next()? == name).then(|| parts.skip(1).collect::<Vec<_>>().join(" "))
        })
    };
    if value("ProxyEnable").is_none_or(|v| v.trim() == "0x0") {
        return None;
    }
    let server = value("ProxyServer")?;
    // `host:porta` ou `http=host:porta;https=host:porta`.
    let chosen = if server.contains('=') {
        server.split(';').find_map(|p| p.strip_prefix("https=")).or_else(|| server.split(';').find_map(|p| p.strip_prefix("http=")))?.to_string()
    } else {
        server
    };
    ureq::Proxy::new(&format!("http://{}", chosen.trim())).ok()
}
