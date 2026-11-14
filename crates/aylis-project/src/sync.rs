//! Várias linguagens no mesmo projeto: o visual (forms, temas, assets) é um só e cada
//! linguagem tem o código dela numa pasta (`rust/`, `python/`...). Este módulo mantém as
//! pastas em dia com o visual e diz o que falta portar de uma para outra.
//!
//! - **Visual:** a cada geração, um handler renomeado no form (o mesmo componente e evento,
//!   outro nome) é renomeado no código de todas as linguagens, com o corpo; os novos entram
//!   vazios ([`crate::codegen::generate`]); os que saíram do form ficam no código e aparecem
//!   como "sem uso".
//! - **Lógica:** não dá para converter. Cada handler tem uma revisão: quando o corpo dele
//!   muda numa linguagem que estava em dia, a revisão sobe; as outras linguagens ficam
//!   pendentes até o corpo delas mudar também (foi portado) ou alguém marcar como portado.
//! - **Componentes:** um nome que saiu do form e ainda aparece no código (`"edtNome"`).
//!
//! O estado fica em `code-sync.json`, na pasta do projeto (vai junto para o controle de
//! versão: as pendências valem em qualquer máquina).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::codegen::{form_handlers, is_event_key, module_name};
use crate::{Language, Project};

/// O arquivo do estado, na pasta do projeto.
pub const FILE: &str = "code-sync.json";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SyncState {
    /// Por handler (`frmCalculadora.tecla_click`): quantas vezes a lógica mudou.
    #[serde(default)]
    pub revisions: BTreeMap<String, u32>,
    /// Por linguagem (a pasta: `rust`, `python`...).
    #[serde(default)]
    pub languages: BTreeMap<String, LanguageState>,
    /// Os nomes de componente que os forms já tiveram (para achar no código os que saíram).
    #[serde(default)]
    pub components: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LanguageState {
    /// `frmCalculadora/btn7.on_click` → `tecla_click`, como estava na última geração.
    #[serde(default)]
    pub bindings: BTreeMap<String, String>,
    /// Por handler: a impressão do corpo e a revisão que esta linguagem já tem.
    #[serde(default)]
    pub handlers: BTreeMap<String, HandlerState>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HandlerState {
    pub hash: String,
    pub seen: u32,
}

/// O que falta numa linguagem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pending {
    /// A lógica mudou em `by` (as linguagens em dia) e esta ainda não acompanhou; `empty`:
    /// aqui ainda é o handler vazio que a geração criou.
    Changed { form: String, handler: String, by: Vec<Language>, empty: bool },
    /// O handler saiu do form, mas a função continua no código.
    Unused { form: String, handler: String },
    /// Um componente que saiu do form e o código ainda cita.
    MissingComponent { name: String, file: PathBuf, line: usize },
}

impl Pending {
    /// Uma linha para listas (`frmCalculadora.tecla_click — alterado em Rust`).
    pub fn describe(&self) -> String {
        match self {
            Pending::Changed { form, handler, by, empty } => {
                let by: Vec<&str> = by.iter().map(|l| l.name()).collect();
                let how = if *empty { "vazio aqui" } else { "desatualizado aqui" };
                format!("{form}.{handler} — alterado em {} ({how})", by.join(", "))
            }
            Pending::Unused { form, handler } => format!("{form}.{handler} — saiu do form (a função continua no código)"),
            Pending::MissingComponent { name, file, line } => format!("{name} — não existe mais no form ({}:{line})", file.display()),
        }
    }
}

pub fn load(project: &Project) -> SyncState {
    std::fs::read_to_string(project.dir().join(FILE)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn save(project: &Project, state: &SyncState) -> Result<(), String> {
    let text = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    let path = project.dir().join(FILE);
    if std::fs::read_to_string(&path).is_ok_and(|t| t.trim_end() == text) {
        return Ok(());
    }
    std::fs::write(&path, text + "\n").map_err(|e| format!("não consegui gravar {}: {e}", path.display()))
}

/// Um form do projeto como o código o vê.
struct FormInfo {
    /// O nome (`frmCalculadora`) e o módulo (`calculadora`).
    form: String,
    module: String,
    handlers: Vec<String>,
    /// `componente.evento` → handler (o form inteiro é `frmX.on_show`).
    bindings: Vec<(String, String)>,
    components: Vec<String>,
}

fn forms(project: &Project) -> Result<Vec<FormInfo>, String> {
    forms_in(project, None)
}

/// Os forms do projeto, ou só `only` (o F12 de um form não relê os outros: no ERP de 1009 forms,
/// ler todos custava meio segundo por passo).
fn forms_in(project: &Project, only: Option<&[String]>) -> Result<Vec<FormInfo>, String> {
    let mut out = Vec::new();
    let mut files = crate::FormFiles::default();
    for entry in project.forms.iter().filter(|e| only.is_none_or(|o| o.iter().any(|n| e.name.eq_ignore_ascii_case(n)))) {
        let mut info = FormInfo { form: entry.name.clone(), module: module_name(&entry.file), handlers: Vec::new(), bindings: Vec::new(), components: Vec::new() };
        for (_, layout) in project.read_entry_variants(entry, &mut files)? {
            let (form, handlers) = form_handlers(&layout)?;
            info.form = form;
            for h in handlers {
                if !info.handlers.contains(&h) {
                    info.handlers.push(h);
                }
            }
            let doc: Value = serde_json::from_str(layout.trim_start_matches('\u{feff}')).map_err(|e| e.to_string())?;
            walk(doc.get("root").unwrap_or(&doc), &mut info.bindings, &mut info.components);
        }
        out.push(info);
    }
    Ok(out)
}

pub(crate) fn walk(node: &Value, bindings: &mut Vec<(String, String)>, components: &mut Vec<String>) {
    let Some(obj) = node.as_object() else { return };
    let name = obj.get("name").and_then(Value::as_str).unwrap_or_default();
    if !name.is_empty() && !components.iter().any(|c| c == name) {
        components.push(name.to_string());
    }
    for (key, value) in obj {
        if let (true, Some(handler)) = (is_event_key(key), value.as_str()) {
            if !handler.is_empty() && !name.is_empty() {
                let k = format!("{name}.{key}");
                if !bindings.iter().any(|(b, _)| *b == k) {
                    bindings.push((k, handler.to_string()));
                }
            }
        }
    }
    if let Some(children) = obj.get("children").and_then(Value::as_array) {
        for child in children {
            walk(child, bindings, components);
        }
    }
}

/// Os handlers do form `form` que a geração não deve recriar nesta linguagem: os que já
/// estavam no código dela na última geração (o usuário apagou a função) e cujas ligações no
/// form são as mesmas de então. Um evento ligado agora (uma ligação nova) ganha a função vazia;
/// um handler citado e apagado é erro da verificação, que leva ao lugar (decidido pelo autor em
/// 08/11: avisar, não recriar).
pub fn removed_handlers(state: &SyncState, language: Language, form: &str, bindings: &[(String, String)]) -> Vec<String> {
    let Some(lang) = state.languages.get(language.folder()) else { return Vec::new() };
    let mut out: Vec<String> = Vec::new();
    for (_, handler) in bindings {
        if out.contains(handler) || !lang.handlers.contains_key(&key(form, handler)) {
            continue;
        }
        let linked_before = bindings.iter().filter(|(_, h)| h == handler).all(|(b, h)| lang.bindings.get(&format!("{form}/{b}")) == Some(h));
        if linked_before {
            out.push(handler.clone());
        }
    }
    out
}

/// O arquivo de eventos de um form numa linguagem.
fn source(project: &Project, language: Language, form: &str) -> Option<PathBuf> {
    crate::codegen::form_source(&project.with_language(language), form)
}

/// O último registro: a marca ([`crate::FileStamp`]) do arquivo de código de cada form em cada
/// linguagem, como estava ao registrar.
fn record_marker(project: &Project) -> PathBuf {
    project.code_dir().join("generated/components/recorded")
}

/// A chave de um form numa linguagem no [`record_marker`].
fn record_key(language: Language, form: &str) -> String {
    format!("{}/{form}", language.folder())
}

/// As marcas de agora dos arquivos de código de todos os forms, em todas as linguagens.
fn source_stamps(project: &Project) -> BTreeMap<String, crate::FileStamp> {
    let mut files = crate::FormFiles::default();
    let mut out = BTreeMap::new();
    for language in project.code_languages() {
        let p = project.with_language(language);
        for entry in &project.forms {
            if let Some(stamp) = files.stamp(&crate::codegen::entry_source(&p, entry)) {
                out.insert(record_key(language, &entry.name), stamp);
            }
        }
    }
    out
}

/// Os forms cujo código, em alguma linguagem, mudou depois do último registro: o registro só
/// precisa relê-los (e aos forms que mudaram). `None`: sem registro ainda, todos. "Mudou" é a
/// marca ser outra que a registrada (gravado no mesmo milissegundo do registro também conta).
pub fn changed_since_record(project: &Project) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(record_marker(project)).ok()?;
    let recorded: BTreeMap<String, crate::FileStamp> = serde_json::from_str(&text).ok()?;
    let now = source_stamps(project);
    let mut out: Vec<String> = Vec::new();
    for language in project.code_languages() {
        for entry in &project.forms {
            let key = record_key(language, &entry.name);
            if now.get(&key) != recorded.get(&key) && !out.contains(&entry.name) {
                out.push(entry.name.clone());
            }
        }
    }
    Some(out)
}

/// O texto que marca a definição de um handler no arquivo e o handler vazio que a geração
/// cria, numa linguagem.
fn stub(language: Language, module: &str, form: &str, handler: &str) -> (String, String) {
    match language {
        Language::Rust => (format!("fn {handler}("), format!("pub fn {handler}(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {{\n{}}}\n", crate::codegen::rust_stub_body(handler))),
        Language::Python => (format!("def {handler}("), format!("def {handler}(aylis, event):\n    pass\n")),
        other => crate::hostlang::stub(other, module, form, handler),
    }
}

/// Onde está a definição de um handler (`marker`, como `fn salvar_click(`) no texto do
/// arquivo: só a de verdade, e não uma cópia comentada, num texto ou no meio de outro nome
/// (G22 do Tabernáculo). Nas linguagens de chaves, fora de comentários e strings; em Python,
/// no começo da linha; em Pascal, depois do `procedure`/`function`. No C++ a primeira
/// ocorrência é a declaração (no bloco de registro): vale a última.
pub fn find_marker(language: Language, text: &str, marker: &str) -> Option<usize> {
    let marker = marker.trim_end_matches(" {").trim_end();
    if marker.is_empty() {
        return None;
    }
    match language {
        Language::Python | Language::Pascal => {
            let ident = |c: char| c.is_alphanumeric() || c == '_';
            let mut from = 0;
            while let Some(i) = text[from..].find(marker).map(|i| from + i) {
                let line = text[text[..i].rfind('\n').map_or(0, |n| n + 1)..i].trim().to_ascii_lowercase();
                let starts = match language {
                    Language::Python => line.is_empty() || line == "async",
                    _ => ["procedure", "function", "constructor", "destructor", "class procedure", "class function"].contains(&line.as_str()),
                };
                if starts && !text[..i].chars().next_back().is_some_and(ident) {
                    return Some(i);
                }
                from = i + marker.len();
            }
            None
        }
        Language::Cpp => {
            let mut last = None;
            while let Some(i) = crate::codegen::find_in_code(text, marker, last.map_or(0, |l| l + marker.len())) {
                last = Some(i);
            }
            last
        }
        _ => crate::codegen::find_in_code(text, marker, 0),
    }
}

/// O corpo de um handler no texto do arquivo (sem a linha da definição), ou `None` se ele
/// não está lá. As linguagens de chaves vão até a chave que fecha (as de dentro de strings,
/// caracteres e comentários não contam); Python, até a próxima linha sem recuo; Pascal, até a
/// próxima rotina.
pub fn body(language: Language, text: &str, marker: &str) -> Option<String> {
    body_at(language, &text[find_marker(language, text, marker)?..])
}

/// O corpo do handler cuja definição começa `rest`.
fn body_at(language: Language, rest: &str) -> Option<String> {
    match language {
        Language::Python => {
            let mut lines = rest.lines();
            lines.next();
            let body: Vec<&str> = lines.take_while(|l| l.trim().is_empty() || l.starts_with([' ', '\t'])).collect();
            Some(body.join("\n"))
        }
        Language::Pascal => {
            let mut lines = rest.lines();
            lines.next();
            let ends = ["procedure ", "function ", "constructor ", "destructor ", "end.", "initialization", "finalization"];
            let body: Vec<&str> = lines.take_while(|l| !ends.iter().any(|e| l.to_ascii_lowercase().starts_with(e))).collect();
            Some(body.join("\n"))
        }
        _ => {
            // Pula a lista de parâmetros (sem `{` nas linguagens geradas) até a chave.
            let open = crate::codegen::find_in_code(rest, "{", 0)?;
            match crate::codegen::matching_brace(rest, open) {
                Some(close) => Some(rest[open + 1..close].to_string()),
                None => Some(rest[open + 1..].to_string()),
            }
        }
    }
}

/// O começo das impressões de agora: as de antes (só os 16 dígitos) são conferidas pela regra
/// de antes ([`legacy_fingerprint`]).
const FINGERPRINT_V2: &str = "v2:";

/// O que conta como lógica num corpo: sem espaços, sem comentários e sem a vírgula antes do
/// fecha (mudar o recuo, quebrar a linha ou passar o `cargo fmt` não é mudar a lógica; G22 do
/// Tabernáculo). O texto entre aspas fica como está.
fn logic(language: Language, body: &str) -> String {
    let bytes = body.as_bytes();
    let mut out = String::with_capacity(body.len());
    let mut comma = false;
    let push = |out: &mut String, piece: &str, comma: &mut bool| {
        if std::mem::take(comma) && !piece.starts_with([')', ']', '}']) {
            out.push(',');
        }
        if piece == "," {
            *comma = true;
        } else {
            out.push_str(piece);
        }
    };
    let mut i = 0;
    while i < bytes.len() {
        // Um comentário (pulado) ou um texto entre aspas (como está), até o último byte dele.
        let (end, comment) = match language {
            Language::Python => match bytes[i] {
                b'#' => (body[i..].find('\n').map_or(bytes.len(), |n| i + n), true),
                q @ (b'"' | b'\'') => {
                    let triple = [q; 3];
                    if bytes[i..].starts_with(&triple) {
                        (body[i + 3..].find(std::str::from_utf8(&triple).unwrap_or("\"\"\"")).map_or(bytes.len(), |n| i + 3 + n + 2), false)
                    } else {
                        let mut j = i + 1;
                        while j < bytes.len() && bytes[j] != q && bytes[j] != b'\n' {
                            j += if bytes[j] == b'\\' { 2 } else { 1 };
                        }
                        (j.min(bytes.len()), false)
                    }
                }
                _ => (usize::MAX, false),
            },
            Language::Pascal => match bytes[i] {
                b'/' if bytes.get(i + 1) == Some(&b'/') => (body[i..].find('\n').map_or(bytes.len(), |n| i + n), true),
                b'{' => (body[i..].find('}').map_or(bytes.len(), |n| i + n), true),
                b'(' if bytes.get(i + 1) == Some(&b'*') => (body[i + 2..].find("*)").map_or(bytes.len(), |n| i + 2 + n + 1), true),
                b'\'' => (body[i + 1..].find('\'').map_or(bytes.len(), |n| i + 1 + n), false),
                _ => (usize::MAX, false),
            },
            _ => match crate::codegen::skip_literal(body, i) {
                Some(end) => (end, bytes[i] == b'/'),
                None => (usize::MAX, false),
            },
        };
        if end != usize::MAX {
            let end = end.min(bytes.len() - 1);
            if !comment {
                push(&mut out, &body[i..=end], &mut comma);
            }
            i = end + 1;
            continue;
        }
        let c = body[i..].chars().next().unwrap_or(' ');
        if !c.is_whitespace() {
            push(&mut out, &body[i..i + c.len_utf8()], &mut comma);
        }
        i += c.len_utf8();
    }
    if comma {
        out.push(',');
    }
    out
}

/// FNV-1a, que não muda com a versão do Rust.
fn fnv(bytes: impl Iterator<Item = u8>) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// A impressão de um corpo: a da [`logic`] dele.
fn fingerprint(language: Language, body: &str) -> String {
    format!("{FINGERPRINT_V2}{}", fnv(logic(language, body).bytes()))
}

/// A impressão de antes (até o G22): todos os bytes sem espaço, comentários e vírgulas
/// inclusive, do corpo achado pela primeira ocorrência do marcador e contando todas as chaves.
/// Só para reconhecer, num `code-sync.json` de antes, o corpo que não mudou.
fn legacy_fingerprint(language: Language, text: &str, marker: &str) -> Option<String> {
    let at = if language == Language::Cpp { text.rfind(marker.trim_end_matches(" {").trim_end())? } else { text.find(marker)? };
    let rest = &text[at..];
    let body = match language {
        Language::Python | Language::Pascal => body_at(language, rest)?,
        _ => {
            let open = rest.find('{')?;
            let mut depth = 0usize;
            let mut close = rest.len();
            for (i, c) in rest[open..].char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            close = open + i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            rest[open + 1..close].to_string()
        }
    };
    Some(fnv(body.bytes().filter(|b| !b.is_ascii_whitespace())))
}

/// A impressão guardada `stored` é a do corpo de agora? Uma de antes do G22 é conferida pela
/// regra de antes: o código que não mudou continua em dia depois de atualizar o Aylis.
fn same_body(language: Language, stored: &str, now: &str, text: &str, marker: &str) -> bool {
    stored == now || (!stored.starts_with(FINGERPRINT_V2) && legacy_fingerprint(language, text, marker).as_deref() == Some(stored))
}

/// A impressão do handler vazio da geração, numa linguagem.
fn empty_fingerprint(language: Language, module: &str, form: &str, handler: &str) -> String {
    let (marker, stub) = stub(language, module, form, handler);
    fingerprint(language, &body(language, &stub, &marker).unwrap_or_default())
}

fn key(form: &str, handler: &str) -> String {
    format!("{form}.{handler}")
}

/// Troca as ocorrências do identificador `old` por `new` (palavra inteira).
fn rename_identifier(text: &str, old: &str, new: &str) -> String {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find(old) {
        let before = rest[..i].chars().next_back();
        let after = rest[i + old.len()..].chars().next();
        out.push_str(&rest[..i]);
        if before.is_some_and(ident) || after.is_some_and(ident) {
            out.push_str(old);
        } else {
            out.push_str(new);
        }
        rest = &rest[i + old.len()..];
    }
    out.push_str(rest);
    out
}

/// Antes de gerar uma linguagem (`project.language`): os handlers renomeados no form (o
/// mesmo componente e evento com outro nome) são renomeados no código dela, com o corpo.
pub fn apply_renames(project: &Project) -> Result<(), String> {
    apply_renames_in(project, None)
}

/// O [`apply_renames`] só dos forms `only` (`None`: todos): os que mudaram desde a última
/// geração (renomear muda o form).
pub fn apply_renames_in(project: &Project, only: Option<&[String]>) -> Result<(), String> {
    let language = project.language;
    let mut state = load(project);
    let Some(previous) = state.languages.get(language.folder()).map(|l| l.bindings.clone()) else { return Ok(()) };
    let mut changed = false;
    for info in forms_in(project, only)? {
        let Some(path) = source(project, language, &info.form) else { continue };
        let Ok(mut text) = std::fs::read_to_string(&path) else { continue };
        let original = text.clone();
        for (binding, new) in &info.bindings {
            let Some(old) = previous.get(&format!("{}/{binding}", info.form)) else { continue };
            // Outro componente ainda usa o nome antigo: não é renomeação.
            if old == new || info.handlers.contains(old) {
                continue;
            }
            let (old_marker, _) = stub(language, &info.module, &info.form, old);
            let (new_marker, _) = stub(language, &info.module, &info.form, new);
            let (has_old, has_new) = (find_marker(language, &text, &old_marker).is_some(), find_marker(language, &text, &new_marker).is_some());
            let (old_key, new_key) = (key(&info.form, old), key(&info.form, new));
            match (has_old, has_new) {
                (true, false) => text = rename_identifier(&text, old, new),
                // Já renomeada à mão no fonte (e depois no form): o texto fica, e o histórico vai
                // junto do mesmo jeito (antes ficava a sobra com o nome antigo; G22).
                (false, true) if !state.revisions.contains_key(&new_key) => {}
                _ => continue,
            }
            // O histórico do handler vai junto, em todas as linguagens.
            if let Some(r) = state.revisions.remove(&old_key) {
                state.revisions.insert(new_key.clone(), r);
            }
            for l in state.languages.values_mut() {
                if let Some(h) = l.handlers.remove(&old_key) {
                    l.handlers.insert(new_key.clone(), h);
                }
            }
            changed = true;
        }
        if text != original {
            std::fs::write(&path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
        }
    }
    if changed {
        save(project, &state)?;
    }
    Ok(())
}

/// Depois de gerar: anota, em cada linguagem com código, a impressão de cada handler. Um
/// corpo que mudou numa linguagem em dia sobe a revisão (as outras ficam pendentes); numa
/// linguagem atrasada, conta como portado.
pub fn record(project: &Project) -> Result<(), String> {
    record_in(project, None)
}

/// O [`record`] só dos forms `only` (`None`: todos): as ligações dos outros forms ficam como
/// estão.
pub fn record_in(project: &Project, only: Option<&[String]>) -> Result<(), String> {
    let mut state = load(project);
    let forms = forms_in(project, only)?;
    for info in &forms {
        state.components.extend(info.components.iter().cloned());
    }
    for language in project.code_languages() {
        // Só alguns forms: as ligações dos outros ficam; as deles são refeitas.
        let mut bindings = match only {
            Some(_) => {
                let prefixes: Vec<String> = forms.iter().map(|f| format!("{}/", f.form)).collect();
                let mut kept = state.languages.get(language.folder()).map(|l| l.bindings.clone()).unwrap_or_default();
                kept.retain(|k, _| !prefixes.iter().any(|p| k.starts_with(p)));
                kept
            }
            None => BTreeMap::new(),
        };
        for info in &forms {
            for (b, h) in &info.bindings {
                bindings.insert(format!("{}/{b}", info.form), h.clone());
            }
            let Some(text) = source(project, language, &info.form).and_then(|p| std::fs::read_to_string(p).ok()) else { continue };
            for handler in &info.handlers {
                let (marker, _) = stub(language, &info.module, &info.form, handler);
                let Some(b) = body(language, &text, &marker) else { continue };
                let hash = fingerprint(language, &b);
                let empty = hash == empty_fingerprint(language, &info.module, &info.form, handler);
                let k = key(&info.form, handler);
                let revision = state.revisions.get(&k).copied().unwrap_or(0);
                let known = state.languages.get(language.folder()).and_then(|l| l.handlers.get(&k)).cloned();
                let seen = match known {
                    // A primeira vez: com código, é a referência (revisão 1 no mínimo); vazio,
                    // ainda não tem nada.
                    None if empty => 0,
                    None => {
                        let r = revision.max(1);
                        state.revisions.insert(k.clone(), r);
                        r
                    }
                    // O mesmo corpo (a impressão de antes do G22 conferida pela regra dela): a
                    // impressão é refeita pelo fonte, sem revisão nova.
                    Some(old) if same_body(language, &old.hash, &hash, &text, &marker) => old.seen,
                    // Uma impressão sem versão que nem a regra de antes reconhece é de uma regra
                    // ainda mais velha (a de 09/11 em Python, TypeScript, C#, Java e Pascal):
                    // não dá para saber se o corpo mudou, então ela só é refeita pelo fonte, sem
                    // revisão nova nem porte. Tomá-la por edição subia a revisão e deixava as
                    // outras linguagens pendentes sem nada para portar.
                    Some(old) if !old.hash.starts_with(FINGERPRINT_V2) => old.seen,
                    // Mudou numa linguagem atrasada: é o porte.
                    Some(old) if old.seen < revision => revision,
                    // Mudou numa linguagem em dia: lógica nova, as outras ficam pendentes.
                    Some(_) => {
                        state.revisions.insert(k.clone(), revision + 1);
                        revision + 1
                    }
                };
                state.languages.entry(language.folder().to_string()).or_default().handlers.insert(k, HandlerState { hash, seen });
            }
            // O que saiu do form e também do código (a função apagada à mão): nada mais a
            // acompanhar. A que saiu do form e continua no código fica (aparece "sem uso").
            if let Some(lang) = state.languages.get_mut(language.folder()) {
                let prefix = format!("{}.", info.form);
                lang.handlers.retain(|k, _| {
                    let Some(handler) = k.strip_prefix(&prefix) else { return true };
                    info.handlers.iter().any(|h| h == handler) || body(language, &text, &stub(language, &info.module, &info.form, handler).0).is_some()
                });
            }
        }
        state.languages.entry(language.folder().to_string()).or_default().bindings = bindings;
    }
    // A revisão de um handler que nenhuma linguagem acompanha mais.
    let followed: BTreeSet<String> = state.languages.values().flat_map(|l| l.handlers.keys().cloned()).collect();
    state.revisions.retain(|k, _| followed.contains(k));
    save(project, &state)?;
    let marker = record_marker(project);
    if let Some(dir) = marker.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    // As marcas depois do registro (o que a geração escreveu já está nelas). Só alguns forms
    // (o F12): só as deles mudam; um arquivo de outro form mexido continua aparecendo.
    let now = source_stamps(project);
    let stamps = match only {
        None => now,
        Some(names) => {
            let mut kept: BTreeMap<String, crate::FileStamp> =
                std::fs::read_to_string(&marker).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
            for language in project.code_languages() {
                for name in names {
                    let key = record_key(language, name);
                    match now.get(&key) {
                        Some(stamp) => kept.insert(key, *stamp),
                        None => kept.remove(&key),
                    };
                }
            }
            kept
        }
    };
    let stamps = serde_json::to_string_pretty(&stamps).map_err(|e| e.to_string())?;
    // Igual ao que está gravado: não regrava (cada geração mudava o arquivo; G17).
    if std::fs::read_to_string(&marker).is_ok_and(|t| t == stamps) {
        return Ok(());
    }
    std::fs::write(&marker, stamps).map_err(|e| format!("não consegui gravar {}: {e}", marker.display()))
}

/// O que falta na linguagem `language` (o projeto na pasta dela).
pub fn pending(project: &Project, language: Language) -> Result<Vec<Pending>, String> {
    let state = load(project);
    let forms = forms(project)?;
    let mut out = Vec::new();
    let Some(mine) = state.languages.get(language.folder()) else { return Ok(out) };
    let mut current = BTreeSet::new();
    for info in &forms {
        let text = source(project, language, &info.form).and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
        for handler in &info.handlers {
            let k = key(&info.form, handler);
            current.insert(k.clone());
            let revision = state.revisions.get(&k).copied().unwrap_or(0);
            let seen = mine.handlers.get(&k).map_or(0, |h| h.seen);
            if seen >= revision {
                continue;
            }
            let by: Vec<Language> = project
                .code_languages()
                .into_iter()
                .filter(|l| *l != language && state.languages.get(l.folder()).and_then(|s| s.handlers.get(&k)).is_some_and(|h| h.seen == revision))
                .collect();
            let (marker, _) = stub(language, &info.module, &info.form, handler);
            let empty = body(language, &text, &marker).is_none_or(|b| fingerprint(language, &b) == empty_fingerprint(language, &info.module, &info.form, handler));
            out.push(Pending::Changed { form: info.form.clone(), handler: handler.clone(), by, empty });
        }
        // As funções que saíram do form (anotadas antes, ainda no código).
        for k in mine.handlers.keys() {
            let Some(handler) = k.strip_prefix(&format!("{}.", info.form)) else { continue };
            if current.contains(k) || info.handlers.iter().any(|h| h == handler) {
                continue;
            }
            let (marker, _) = stub(language, &info.module, &info.form, handler);
            if find_marker(language, &text, &marker).is_some() {
                out.push(Pending::Unused { form: info.form.clone(), handler: handler.to_string() });
            }
        }
    }
    // Componentes que saíram de todos os forms e o código ainda cita entre aspas.
    let alive: BTreeSet<&String> = forms.iter().flat_map(|f| f.components.iter()).collect();
    let gone: Vec<&String> = state.components.iter().filter(|c| !alive.contains(c)).collect();
    if !gone.is_empty() {
        for file in code_files(&project.code_dir_of(language)) {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            for (n, line) in text.lines().enumerate() {
                for name in &gone {
                    if line.contains(&format!("\"{name}\"")) || line.contains(&format!("'{name}'")) {
                        out.push(Pending::MissingComponent { name: (*name).clone(), file: file.clone(), line: n + 1 });
                    }
                }
            }
        }
    }
    Ok(out)
}

/// Onde está a função de um handler numa linguagem: o arquivo e a linha (1 se a função
/// ainda não existe no arquivo).
pub fn handler_location(project: &Project, language: Language, form: &str, handler: &str) -> Option<(PathBuf, usize)> {
    let entry = project.form(form)?;
    let file = source(project, language, form)?;
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    let (marker, _) = stub(language, &module_name(&entry.file), form, handler);
    // A função de verdade, e não uma cópia comentada acima dela.
    let at = find_marker(language, &text, &marker);
    let line = at.map_or(1, |i| text[..i].matches('\n').count() + 1);
    Some((file, line))
}

/// O [`handler_location`] de vários handlers do mesmo form, lendo o arquivo uma vez: o arquivo
/// e a linha de cada um (`None`: não está no código).
pub fn handler_lines(project: &Project, language: Language, form: &str, handlers: &[String]) -> Option<(PathBuf, Vec<Option<usize>>)> {
    let entry = project.form(form)?;
    let file = source(project, language, form)?;
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    let module = module_name(&entry.file);
    let lines = handlers
        .iter()
        .map(|handler| {
            let (marker, _) = stub(language, &module, form, handler);
            find_marker(language, &text, &marker).map(|i| text[..i].matches('\n').count() + 1)
        })
        .collect();
    Some((file, lines))
}

/// O handler tem função no código do form numa linguagem? `None` se o arquivo do form não
/// existe nela.
/// O handler está no código `text` (o arquivo de eventos do form `entry` na linguagem): quem
/// confere vários lê o arquivo uma vez.
pub fn handler_in(language: Language, entry: &crate::FormEntry, handler: &str, text: &str) -> bool {
    let (marker, _) = stub(language, &module_name(&entry.file), &entry.name, handler);
    body(language, text, &marker).is_some()
}

pub fn handler_defined(project: &Project, language: Language, form: &str, handler: &str) -> Option<bool> {
    let entry = project.form(form)?;
    let text = std::fs::read_to_string(source(project, language, form)?).ok()?;
    let (marker, _) = stub(language, &module_name(&entry.file), form, handler);
    Some(body(language, &text, &marker).is_some())
}

/// Marca um handler como portado na linguagem (a lógica dela está em dia).
pub fn mark_ported(project: &Project, language: Language, form: &str, handler: &str) -> Result<(), String> {
    let mut state = load(project);
    let k = key(form, handler);
    let revision = state.revisions.get(&k).copied().unwrap_or(0);
    let entry = state.languages.entry(language.folder().to_string()).or_default().handlers.entry(k).or_default();
    entry.seen = revision;
    save(project, &state)
}

/// Os arquivos de código de uma pasta de linguagem (sem as saídas de compilação).
fn code_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if !["target", "bin", "obj", "node_modules", ".aylis", "out"].contains(&name.as_str()) {
                out.extend(code_files(&p));
            }
        } else if ["rs", "py", "ts", "cs", "java", "go", "cpp", "hpp", "h", "pas", "lpr"].iter().any(|x| p.extension().is_some_and(|e| e == *x)) {
            out.push(p);
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodies_in_each_language() {
        let rust = "pub fn a(ui: &mut Ui) {\n    if x { y(); }\n}\n\npub fn b() {\n}\n";
        assert_eq!(body(Language::Rust, rust, "fn a(").unwrap().trim(), "if x { y(); }");
        assert_eq!(body(Language::Rust, rust, "fn b(").unwrap().trim(), "");
        let py = "def a(aylis, event):\n    x = 1\n\n    return x\n\n\ndef b(aylis, event):\n    pass\n";
        assert_eq!(fingerprint(Language::Python, &body(Language::Python, py, "def a(").unwrap()), fingerprint(Language::Python, "x = 1 return x"));
        let pas = "procedure TFrm.a(Aylis: TAylis);\nbegin\n  X := 1;\nend;\n\nprocedure TFrm.b(Aylis: TAylis);\nbegin\nend;\n\nend.\n";
        assert_eq!(fingerprint(Language::Pascal, &body(Language::Pascal, pas, "TFrm.a(").unwrap()), fingerprint(Language::Pascal, "begin X := 1; end;"));
        assert_eq!(rename_identifier("salvar_click(); salvar_click_x(); a.salvar_click", "salvar_click", "gravar_click"), "gravar_click(); salvar_click_x(); a.gravar_click");
    }
}
