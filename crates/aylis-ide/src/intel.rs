//! A inteligência do editor de código: o servidor de linguagem do projeto (o mesmo que o VS
//! Code usa, [`aylis_tools::lsp`]) e, sem ele (ou quando ele não responde), o índice próprio
//! dos fontes ([`aylis_project::symbols`]).
//!
//! A janela de código manda os pedidos ([`LspEvent`]: abriu, mudou, sugestões, dica,
//! definição...) e esta parte responde chamando as funções da janela (a lista de sugestões,
//! a dica, os erros, as referências, as trocas de texto). O servidor abre numa thread (o Java
//! e o Rust demoram a indexar) e as respostas chegam no `poll`, a cada volta da IDE.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::Arc;

use aylis_controls::Suggestion;
use aylis_core::Ui;
use aylis_project::Language;
use aylis_tools::lsp::{self, Client, Incoming, Pos};
use serde_json::{Value, json};

use crate::codewindow::{self, LspEvent};
use crate::state::Shared;

/// O que um pedido em andamento vai fazer com a resposta.
enum Ask {
    Complete { file: PathBuf },
    Hover { file: PathBuf, index: usize },
    /// Servidor sem dica (o pasls): a linha da declaração faz o papel.
    HoverDeclaration { file: PathBuf, index: usize },
    Signature { file: PathBuf },
    Definition { word: String },
    References { word: String },
    Rename,
    /// O renomear do projeto (o componente, o form ou o handler renomeado no designer): a
    /// resposta vai para a prévia (`crate::rename`), não direto para os arquivos.
    ProjectRename { batch: u64 },
    Format { file: PathBuf },
    Symbols { file: PathBuf },
    QuickFix,
}

/// O servidor do projeto e o que está pendente.
#[derive(Default)]
pub struct Intel {
    client: Option<Client>,
    starting: Option<Receiver<Result<Client, String>>>,
    /// A linguagem e a pasta do servidor aberto (trocou: abre outro).
    key: Option<(Language, PathBuf)>,
    pending: HashMap<i64, Ask>,
    /// O último pedido de cada tipo que se repete (as respostas velhas são jogadas fora).
    latest: HashMap<&'static str, i64>,
    /// Os erros e avisos de cada arquivo (o painel Problemas).
    pub diagnostics: BTreeMap<PathBuf, Vec<lsp::Diag>>,
    /// O que mostrar sobre o servidor ("rust-analyzer", "iniciando…", "sem servidor").
    pub server_status: String,
    /// O detalhe (a dica do rótulo): o que o servidor ainda está fazendo; vazio: o padrão.
    pub server_hint: String,
    /// O servidor do projeto (o programa), achado ao abrir; `None` sem servidor instalado.
    server: Option<lsp::Server>,
    /// Por que ele não abriu (ou que fechou); vazio sem erro.
    error: String,
}

impl Intel {
    /// O servidor está aberto e respondendo.
    pub fn ready(&self) -> bool {
        self.client.is_some()
    }
}

/// A linguagem e a pasta do código do projeto aberto.
/// No modo restrito (projeto de fora), nenhuma: o servidor de linguagem roda código do
/// projeto (o `build.rs`, as macros, o MSBuild, o Gradle). Confiar muda a chave, e ele abre.
fn project_key(state: &Shared) -> Option<(Language, PathBuf)> {
    let st = state.borrow();
    if st.restricted {
        return None;
    }
    let p = st.project.as_ref()?;
    Some((p.language, crate::debugger::norm(&p.code_dir())))
}

/// Abre o servidor do projeto se ainda não estiver aberto (ou se o projeto trocou).
/// `true`: trocou (a barra da janela precisa mostrar o novo).
fn ensure(state: &Shared) -> bool {
    let key = project_key(state);
    let same = state.borrow().intel.key == key;
    if same {
        return false;
    }
    let mut st = state.borrow_mut();
    if let Some(mut c) = st.intel.client.take() {
        c.stop();
    }
    st.intel.starting = None;
    st.intel.pending.clear();
    st.intel.diagnostics.clear();
    st.intel.key = key.clone();
    st.intel.server_hint.clear();
    st.intel.server = None;
    st.intel.error.clear();
    let Some((language, dir)) = key else {
        st.intel.server_status = if st.restricted { "modo restrito: sem servidor de linguagem".into() } else { String::new() };
        return true;
    };
    let Some(server) = lsp::server(language, &dir) else {
        st.intel.server_status = "sem servidor de linguagem (Verificar ambiente instala)".into();
        return true;
    };
    st.intel.server = Some(server.clone());
    st.intel.server_status = format!("{}: iniciando…", server.name);
    st.intel.server_hint = format!("O {} está abrindo (no primeiro uso de um projeto pode levar um pouco): ir para a declaração, as dicas e as sugestões funcionam quando ele estiver pronto.", server.name);
    let waker = st.output_waker.clone();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        lsp::prepare(language, &dir);
        let wake: Option<lsp::Wake> = waker.map(|w| Arc::new(move || w.wake()) as lsp::Wake);
        let result = Client::start(&server, &dir, wake.clone());
        let _ = tx.send(result);
        if let Some(w) = wake {
            w();
        }
    });
    st.intel.starting = Some(rx);
    true
}

/// Um pedido da janela de código.
pub fn handle(ui: &mut Ui, state: &Shared, event: LspEvent) {
    if ensure(state) {
        show_server_status(ui, state);
        codewindow::show_problems(ui, &Default::default());
    }
    // Sem palavra no cursor, não há o que procurar.
    if let LspEvent::Definition { word, .. } | LspEvent::References { word, .. } = &event {
        if word.trim().is_empty() {
            codewindow::set_status(ui, "Ponha o cursor num nome");
            return;
        }
    }
    match event {
        LspEvent::Opened { file, text } | LspEvent::Changed { file, text } => {
            let mut st = state.borrow_mut();
            if let Some(c) = st.intel.client.as_mut() {
                let _ = c.change(&file, &text);
            }
        }
        LspEvent::Saved { file, text } => {
            let mut st = state.borrow_mut();
            if let Some(c) = st.intel.client.as_mut() {
                let _ = c.save(&file, &text);
            }
        }
        LspEvent::Closed { file } => {
            let mut st = state.borrow_mut();
            if let Some(c) = st.intel.client.as_mut() {
                let _ = c.close(&file);
            }
        }
        LspEvent::Complete { file, index } => {
            if !request(ui, state, "textDocument/completion", &file, index, Ask::Complete { file: file.clone() }, "completionProvider", Some("complete")) {
                needs_server(ui, state, "As sugestões");
            }
        }
        LspEvent::Signature { file, index } => {
            request(ui, state, "textDocument/signatureHelp", &file, index, Ask::Signature { file: file.clone() }, "signatureHelpProvider", Some("signature"));
        }
        LspEvent::Hover { file, index } => {
            let asked = request(ui, state, "textDocument/hover", &file, index, Ask::Hover { file: file.clone(), index }, "hoverProvider", Some("hover"))
                || request(ui, state, "textDocument/definition", &file, index, Ask::HoverDeclaration { file: file.clone(), index }, "definitionProvider", Some("hover"));
            // Sem servidor, sem dica (o servidor de linguagem é obrigatório: veja `needs_server`).
            let _ = asked;
        }
        LspEvent::Definition { file, index, word } => {
            if !request(ui, state, "textDocument/definition", &file, index, Ask::Definition { word: word.clone() }, "definitionProvider", None) {
                needs_server(ui, state, "Ir para a declaração");
            }
        }
        LspEvent::References { file, index, word } => {
            let Some(text) = codewindow::text_of(ui, &file) else { return };
            let mut params = lsp::at(&file, lsp::position_in(&text, index));
            params["context"] = json!({ "includeDeclaration": true });
            if !request_raw(state, "textDocument/references", params, Ask::References { word: word.clone() }, "referencesProvider", None) {
                needs_server(ui, state, "Procurar as referências");
            }
        }
        LspEvent::Rename { file, index, name } => {
            let Some(text) = codewindow::text_of(ui, &file) else { return };
            let mut params = lsp::at(&file, lsp::position_in(&text, index));
            params["newName"] = Value::String(name);
            if !request_raw(state, "textDocument/rename", params, Ask::Rename, "renameProvider", None) {
                needs_server(ui, state, "Renomear");
            }
        }
        LspEvent::Format { file } => {
            let params = json!({ "textDocument": { "uri": lsp::uri(&file) }, "options": { "tabSize": 4, "insertSpaces": true } });
            if !request_raw(state, "textDocument/formatting", params, Ask::Format { file: file.clone() }, "documentFormattingProvider", None) {
                codewindow::set_status(ui, "Formatar precisa do servidor de linguagem (este não formata)");
            }
        }
        LspEvent::Symbols { file } => {
            let params = json!({ "textDocument": { "uri": lsp::uri(&file) } });
            if !request_raw(state, "textDocument/documentSymbol", params, Ask::Symbols { file: file.clone() }, "documentSymbolProvider", None) {
                needs_server(ui, state, "Ir para o símbolo");
            }
        }
        LspEvent::QuickFix { file, index } => {
            let Some(text) = codewindow::text_of(ui, &file) else { return };
            let p = lsp::position_in(&text, index);
            let diags: Vec<Value> = state
                .borrow()
                .intel
                .diagnostics
                .get(&crate::debugger::norm(&file))
                .map(|list| {
                    list.iter()
                        .filter(|d| d.start.0 <= p.0 && p.0 <= d.end.0)
                        .map(|d| json!({ "range": range_json(d.start, d.end), "severity": d.severity, "message": d.message, "source": d.source }))
                        .collect()
                })
                .unwrap_or_default();
            let params = json!({ "textDocument": { "uri": lsp::uri(&file) }, "range": range_json(p, p), "context": { "diagnostics": diags } });
            if !request_raw(state, "textDocument/codeAction", params, Ask::QuickFix, "codeActionProvider", None) {
                codewindow::set_status(ui, "Sem correções (precisa do servidor de linguagem)");
            }
        }
    }
}

fn range_json(a: Pos, b: Pos) -> Value {
    json!({ "start": { "line": a.0, "character": a.1 }, "end": { "line": b.0, "character": b.1 } })
}

/// Um pedido num ponto do arquivo. `false` se não há servidor (ou ele não faz isso).
fn request(ui: &Ui, state: &Shared, method: &str, file: &Path, index: usize, ask: Ask, capability: &str, kind: Option<&'static str>) -> bool {
    let Some(text) = codewindow::text_of(ui, file) else { return false };
    request_raw(state, method, lsp::at(file, lsp::position_in(&text, index)), ask, capability, kind)
}

fn request_raw(state: &Shared, method: &str, params: Value, ask: Ask, capability: &str, kind: Option<&'static str>) -> bool {
    let mut st = state.borrow_mut();
    let Some(c) = st.intel.client.as_mut().filter(|c| c.supports(capability)) else { return false };
    let Ok(id) = c.request(method, params) else { return false };
    st.intel.pending.insert(id, ask);
    if let Some(k) = kind {
        st.intel.latest.insert(k, id);
    }
    true
}

/// A cada volta da IDE: o servidor que terminou de abrir e as respostas. `true` se algo mudou.
pub fn poll(ui: &mut Ui, state: &Shared) -> bool {
    let mut changed = false;
    // Terminou de abrir: manda os arquivos abertos.
    let started = {
        let mut st = state.borrow_mut();
        match st.intel.starting.as_ref().map(Receiver::try_recv) {
            Some(Ok(result)) => {
                st.intel.starting = None;
                Some(result)
            }
            Some(Err(TryRecvError::Disconnected)) => {
                st.intel.starting = None;
                None
            }
            _ => None,
        }
    };
    if let Some(result) = started {
        changed = true;
        match result {
            Ok(mut client) => {
                for (file, text) in codewindow::open_files(ui) {
                    let _ = client.open(&file, &text);
                }
                let mut st = state.borrow_mut();
                st.intel.server_status = client.server.name.to_string();
                st.intel.server_hint.clear();
                st.intel.client = Some(client);
            }
            Err(e) => {
                let mut st = state.borrow_mut();
                st.intel.server_status = format!("⚠ {e}");
                st.intel.server_hint.clear();
                st.intel.error = e;
            }
        }
        show_server_status(ui, state);
    }
    let incoming = {
        let mut st = state.borrow_mut();
        let Some(c) = st.intel.client.as_mut() else { return changed };
        if !c.alive() {
            let name = c.server.name;
            st.intel.client = None;
            st.intel.server_status = format!("⚠ o {name} fechou");
            st.intel.error = format!("o {name} fechou");
            drop(st);
            show_server_status(ui, state);
            return true;
        }
        c.poll()
    };
    for msg in incoming {
        changed = true;
        match msg {
            Incoming::Diagnostics { file, items } => {
                let file = crate::debugger::norm(&file);
                codewindow::set_diagnostics(ui, &file, &items);
                let mut st = state.borrow_mut();
                if items.is_empty() {
                    st.intel.diagnostics.remove(&file);
                } else {
                    st.intel.diagnostics.insert(file, items);
                }
                let all = st.intel.diagnostics.clone();
                drop(st);
                codewindow::show_problems(ui, &all);
            }
            Incoming::Message(text) => codewindow::set_status(ui, &text),
            // Indexando, lendo o projeto: o rótulo do servidor diz o quê e quanto falta.
            Incoming::Progress => {
                {
                    let mut st = state.borrow_mut();
                    let Some(c) = st.intel.client.as_ref() else { continue };
                    let (status, hint) = lsp::work_text(c.server.name, &c.work());
                    st.intel.server_status = status;
                    st.intel.server_hint = hint;
                }
                show_server_status(ui, state);
            }
            Incoming::Response { id, result } => {
                let Some(ask) = state.borrow_mut().intel.pending.remove(&id) else { continue };
                // As respostas atrasadas (já houve outro pedido do mesmo tipo) não valem.
                let stale = |kind: &str| state.borrow().intel.latest.get(kind).is_some_and(|&last| last != id);
                let value = match result {
                    Ok(v) => v,
                    Err(e) => {
                        if matches!(ask, Ask::Rename | Ask::Format { .. }) {
                            codewindow::set_status(ui, &format!("⚠ {e}"));
                        }
                        Value::Null
                    }
                };
                answer(ui, state, ask, &value, &stale);
            }
        }
    }
    changed
}

/// Trata a resposta de um pedido.
fn answer(ui: &mut Ui, state: &Shared, ask: Ask, value: &Value, stale: &dyn Fn(&str) -> bool) {
    match ask {
        Ask::Complete { file } => {
            if stale("complete") {
                return;
            }
            let items: Vec<lsp::Completion> = lsp::completions(value).into_iter().take(300).collect();
            let start = items.iter().find_map(|c| c.range.map(|r| r.0));
            let list = items
                .into_iter()
                .map(|c| Suggestion { label: c.label, detail: c.detail, insert: c.insert, kind: c.kind.to_string() })
                .collect();
            codewindow::show_suggestions(ui, &file, list, start);
        }
        Ask::Hover { file, index } => {
            if stale("hover") {
                return;
            }
            // Em Markdown, como veio: a dica mostra o código com as cores do editor.
            let markdown = lsp::hover_markdown(value);
            codewindow::show_hover(ui, &file, index, &markdown);
        }
        Ask::HoverDeclaration { file, index } => {
            if stale("hover") {
                return;
            }
            // A declaração como código (na linguagem do arquivo) e, embaixo, onde ela está.
            let text = lsp::locations(value).first().map(declaration_line).unwrap_or_default();
            let markdown = match text.rsplit_once('\n') {
                Some((code, place)) if !code.trim().is_empty() => format!("```\n{code}\n```\n\n{place}"),
                _ => text,
            };
            codewindow::show_hover(ui, &file, index, &markdown);
        }
        Ask::Signature { file } => {
            if stale("signature") {
                return;
            }
            codewindow::show_signature(ui, &file, &lsp::signature_text(value));
        }
        Ask::Definition { word } => match lsp::locations(value).first() {
            Some(l) => codewindow::go_to_position(ui, &l.file, l.start),
            None => codewindow::set_status(ui, &format!("Declaração de {word} não encontrada{}", still_preparing(state))),
        },
        Ask::References { word } => {
            let list = lsp::locations(value);
            if list.is_empty() {
                codewindow::set_status(ui, &format!("Nenhuma referência a {word}{}", still_preparing(state)));
                return;
            }
            let rows = list.iter().map(|l| (l.file.clone(), l.start, line_text(ui, &l.file, l.start.0))).collect();
            codewindow::show_references(ui, &word, rows);
        }
        Ask::ProjectRename { batch } => {
            crate::rename::lsp_answer(ui, state, batch, lsp::workspace_edit(value));
        }
        Ask::Rename => {
            let edits = lsp::workspace_edit(value);
            let n: usize = edits.iter().map(|(_, e)| e.len()).sum();
            apply_workspace_edit(ui, edits);
            codewindow::set_status(ui, &format!("Renomeado: {n} trocas"));
        }
        Ask::Format { file } => {
            let edits = lsp::text_edits(value);
            if edits.is_empty() {
                codewindow::set_status(ui, "Já está formatado");
            } else {
                apply_workspace_edit(ui, vec![(file, edits)]);
            }
        }
        Ask::Symbols { file } => {
            let list = lsp::symbols(value);
            if list.is_empty() {
                codewindow::set_status(ui, "Nenhum símbolo neste arquivo");
                return;
            }
            let rows = list.into_iter().map(|s| (format!("{}{}", "  ".repeat(s.depth), s.name), s.kind.to_string(), s.file.unwrap_or_else(|| file.clone()), s.start)).collect();
            codewindow::symbols_dialog(ui, "Ir para o símbolo", rows);
        }
        Ask::QuickFix => {
            let actions = lsp::actions(value);
            if actions.is_empty() {
                codewindow::set_status(ui, "Sem correções aqui");
                return;
            }
            let titles: Vec<String> = actions.iter().map(|a| a.title.clone()).collect();
            let state = state.clone();
            codewindow::quick_pick(ui, "Correção rápida", titles, move |ui, i| {
                let action = actions[i].clone();
                apply_workspace_edit(ui, action.edits);
                if let Some(cmd) = action.command {
                    let mut st = state.borrow_mut();
                    if let Some(c) = st.intel.client.as_mut() {
                        let _ = c.request("workspace/executeCommand", json!({ "command": cmd["command"], "arguments": cmd["arguments"] }));
                    }
                }
            });
        }
    }
}

/// A linha de um arquivo (da aba aberta, se houver; senão do disco).
fn line_text(ui: &Ui, file: &Path, line: usize) -> String {
    let text = codewindow::text_of(ui, file).or_else(|| std::fs::read_to_string(file).ok()).unwrap_or_default();
    text.lines().nth(line).unwrap_or_default().trim().to_string()
}

/// A linha da declaração (o que a dica mostra quando o servidor não tem dica).
fn declaration_line(l: &lsp::Location) -> String {
    let text = std::fs::read_to_string(&l.file).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    let Some(first) = lines.get(l.start.0) else { return String::new() };
    // A declaração pode continuar nas linhas de baixo (parâmetros), até o ";" ou o "{".
    let mut out = vec![first.trim().to_string()];
    for next in lines.iter().skip(l.start.0 + 1).take(4) {
        let last = out.last().map(String::as_str).unwrap_or_default();
        if last.ends_with([';', '{', ')']) || last.ends_with(':') {
            break;
        }
        out.push(next.trim().to_string());
    }
    let name = l.file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    format!("{}\n{name}:{}", out.join(" "), l.start.0 + 1)
}

/// Aplica as trocas de vários arquivos: nos abertos, pelo editor (com desfazer); nos outros,
/// direto no disco.
pub fn apply_workspace_edit(ui: &mut Ui, edits: Vec<(PathBuf, Vec<lsp::TextEdit>)>) {
    for (file, list) in edits {
        let ranges: Vec<(Pos, Pos, String)> = list.into_iter().map(|e| (e.start, e.end, e.text)).collect();
        if codewindow::apply_file_edits(ui, &file, &ranges) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        let mut pieces: Vec<(usize, usize, String)> = ranges.into_iter().map(|(a, b, t)| (lsp::index_in(&text, a), lsp::index_in(&text, b), t)).collect();
        pieces.sort_by(|a, b| b.0.cmp(&a.0));
        let mut chars: Vec<char> = text.chars().collect();
        for (a, b, t) in pieces {
            let b = b.max(a).min(chars.len());
            chars.splice(a.min(chars.len())..b, t.chars());
        }
        let _ = std::fs::write(&file, chars.into_iter().collect::<String>());
    }
}

// ---------------------------------------------------------------------------
// Sem servidor

/// O que pede o servidor de linguagem: sem ele, a janela de código diz como instalar. O servidor
/// é obrigatório (decidido em 04/11): a IDE não procura nos fontes por conta própria, que num
/// projeto grande era ler todos os arquivos a cada pedido.
/// Abrindo, o pedido não é "falta instalar": a mensagem diz que ele ainda não está pronto.
fn needs_server(ui: &mut Ui, state: &Shared, what: &str) {
    let starting = state.borrow().intel.starting.is_some().then(|| state.borrow().intel.server_status.clone());
    let text = match starting {
        Some(status) => format!("{what}: o servidor de linguagem ainda está abrindo ({status}); tente de novo em instantes"),
        None => format!("{what} precisa do servidor de linguagem: instale em Executar → Configurar linguagens"),
    };
    codewindow::set_status(ui, &text);
}

/// O rótulo do servidor na janela de código: o estado e, na dica, o detalhe.
fn show_server_status(ui: &mut Ui, state: &Shared) {
    let (status, hint) = {
        let st = state.borrow();
        (st.intel.server_status.clone(), server_detail(ui, &st))
    };
    codewindow::refresh_server_status(ui, &status, &hint);
}

/// O que o servidor sabe fazer, pelo `capabilities` dele (na ordem do menu do editor).
const ABILITIES: [(&str, &str); 9] = [
    ("completionProvider", "sugestões"),
    ("hoverProvider", "dica"),
    ("signatureHelpProvider", "ajuda dos parâmetros"),
    ("definitionProvider", "ir para a definição"),
    ("referencesProvider", "referências"),
    ("renameProvider", "renomear"),
    ("documentFormattingProvider", "formatar"),
    ("documentSymbolProvider", "símbolos"),
    ("codeActionProvider", "correções"),
];

/// A dica do rótulo do servidor (o mouse em cima dele, no rodapé do editor): qual é e a
/// versão, o estado (iniciando, preparando o projeto, pronto, erro com a mensagem), o projeto
/// e a pasta, o programa e o que ele faz. Cada linha passa pela tradução (os modelos com
/// `{}`); vazio, a explicação de sempre.
fn server_detail(ui: &Ui, st: &crate::state::State) -> String {
    let intel = &st.intel;
    if st.restricted {
        return ui.tr("Modo restrito: o servidor de linguagem não abre num projeto que ainda não é confiável (ele executa código do projeto ao ler). Confiar no projeto o abre.");
    }
    let Some((language, dir)) = intel.key.as_ref() else { return String::new() };
    let mut lines = Vec::new();
    let Some(server) = intel.server.as_ref() else {
        lines.push(ui.tr(&format!("Sem servidor de linguagem para {}", language.name())));
        lines.push(ui.tr("Ferramentas → Verificar ambiente instala."));
        return lines.join("\n");
    };
    let version = intel.client.as_ref().map(|c| c.version.trim()).filter(|v| !v.is_empty());
    lines.push(match version {
        Some(v) => ui.tr(&format!("Servidor de linguagem: {} {v}", server.name)),
        None => ui.tr(&format!("Servidor de linguagem: {}", server.name)),
    });
    let busy = intel.client.as_ref().is_some_and(|c| !c.work().is_empty());
    let state_text = if intel.starting.is_some() {
        ui.tr("iniciando…")
    } else if !intel.error.is_empty() {
        format!("{} {}", ui.tr("erro:"), intel.error)
    } else if busy {
        ui.tr("preparando o projeto")
    } else if intel.client.is_some() {
        ui.tr("pronto")
    } else {
        ui.tr("parado")
    };
    lines.push(ui.tr(&format!("Estado: {state_text}")));
    if !intel.server_hint.is_empty() {
        lines.push(ui.tr(&intel.server_hint));
    }
    if let Some(p) = st.project.as_ref() {
        lines.push(ui.tr(&format!("Projeto: {} ({})", p.name, language.name())));
    }
    lines.push(ui.tr(&format!("Pasta: {}", dir.display())));
    lines.push(ui.tr(&format!("Programa: {}", server.program.display())));
    // O que faz: pelo que ele disse ao abrir; antes de pronto, o que o editor pede a ele.
    let abilities: Vec<String> = ABILITIES
        .iter()
        .filter(|(cap, _)| intel.client.as_ref().is_none_or(|c| c.supports(cap)))
        .map(|(_, label)| ui.tr(label))
        .chain(std::iter::once(ui.tr("os erros enquanto digita")))
        .collect();
    lines.push(ui.tr(&format!("Faz: {}", abilities.join(", "))));
    lines.join("\n")
}

/// O servidor do projeto está abrindo ou preparando o projeto (indexando): ainda não responde
/// direito (um renomear volta vazio). `false` sem servidor nenhum.
pub fn preparing(state: &Shared) -> bool {
    // Abre o servidor se ainda não abriu: logo depois de abrir o projeto ele nem tinha começado,
    // e o renomear achava que não havia servidor (abria a prévia por texto; conferência de 08/11).
    ensure(state);
    let st = state.borrow();
    // Há servidor para a linguagem, mas ele está abrindo, ainda não abriu ou está preparando. Um
    // que falhou ao abrir (o erro) conta como sem servidor: senão ficaria "carregando" para sempre.
    st.intel.server.is_some()
        && st.intel.error.is_empty()
        && (st.intel.starting.is_some() || st.intel.client.as_ref().is_none_or(|c| !c.work().is_empty()))
}

/// Há servidor para a linguagem do projeto, mas ele ainda não abriu (está abrindo): não dá para
/// perguntar nada a ele. Abre o servidor se ainda não abriu.
pub fn not_started(state: &Shared) -> bool {
    ensure(state);
    let st = state.borrow();
    st.intel.server.is_some() && st.intel.error.is_empty() && (st.intel.starting.is_some() || st.intel.client.is_none())
}

/// O servidor ainda está preparando o projeto (indexando): uma resposta vazia pode ser só
/// isso, e a mensagem avisa.
fn still_preparing(state: &Shared) -> &'static str {
    let busy = state.borrow().intel.client.as_ref().is_some_and(|c| !c.work().is_empty());
    if busy { " (o servidor de linguagem ainda está preparando o projeto)" } else { "" }
}

/// O servidor de linguagem do projeto está pronto e sabe renomear (abre o servidor se ainda
/// não estiver aberto: a próxima vez pode estar pronto).
pub fn can_rename(state: &Shared) -> bool {
    ensure(state);
    state.borrow().intel.client.as_ref().is_some_and(|c| c.supports("renameProvider"))
}

/// Pede ao servidor o renomear do símbolo em `file`, na posição `at` (linha e coluna do LSP),
/// para `new_name`; a resposta vai para `crate::rename::lsp_answer` com o `batch`. O arquivo
/// é aberto no servidor se ainda não estiver (alguns só renomeiam o que conhecem aberto).
pub fn project_rename(state: &Shared, file: &Path, at: Pos, new_name: &str, batch: u64) -> bool {
    {
        let mut st = state.borrow_mut();
        let Some(c) = st.intel.client.as_mut() else { return false };
        if !c.is_open(file) {
            let Ok(text) = std::fs::read_to_string(file) else { return false };
            let _ = c.open(file, &text);
        }
    }
    let mut params = lsp::at(file, at);
    params["newName"] = Value::String(new_name.to_string());
    request_raw(state, "textDocument/rename", params, Ask::ProjectRename { batch }, "renameProvider", None)
}

/// O projeto fechou ou trocou: fecha o servidor.
pub fn stop(state: &Shared) {
    let mut st = state.borrow_mut();
    if let Some(mut c) = st.intel.client.take() {
        c.stop();
    }
    st.intel.key = None;
    st.intel.starting = None;
}
