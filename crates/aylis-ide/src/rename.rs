//! Renomear em todo o projeto (item 19). Renomear no designer já acerta o próprio form (as
//! referências são pelo componente; as vistas, pelo nome, seguem em `rename_component`). O
//! que fica para trás é o resto do projeto:
//!
//! - **componente**: as referências de outros forms (`frmClientes.cdsCliente`) e os usos no
//!   código (`self.cdsCliente`, `frmClientes.cdsCliente`, nas 8 linguagens);
//! - **form**: o mesmo, e o tipo dele no código (`frmClientes` → `FrmClientes`);
//! - **handler** (um evento ligado a um nome novo que ainda não existe no código, com o antigo
//!   no código): o método no código e os outros eventos ligados a ele.
//!
//! No código, quem acha os usos é o servidor de linguagem (`textDocument/rename`, decidido pelo
//! autor em 04/11 e confirmado em 07/11): só o que é de fato aquele símbolo, não a mesma palavra
//! num comentário ou noutro escopo. Sem o servidor pronto (abrindo, sem servidor instalado, ou
//! se ele não acha o símbolo), as ocorrências por texto (a palavra inteira), com o aviso. Nos
//! forms, pelo JSON (não é código).
//!
//! Com o servidor de linguagem, a troca é direta, como no Delphi (decidido pelo autor em 08/11,
//! no lugar da prévia a cada renomear): a situação diz quantas, e o Ctrl+Z desfaz nos arquivos
//! abertos e nos forms. Só por texto (sem o servidor pronto), a prévia abre: a janela da
//! pesquisa com cada ocorrência marcada, e o "Substituir" aplica as marcadas. O código gerado
//! pela IDE fica de fora: a próxima geração refaz com o nome novo.

use std::path::PathBuf;

use aylis_controls::search::SearchOptions;
use aylis_core::Ui;
use aylis_project::Project;

use crate::app::Ids;
use crate::projectsearch::{FormHit, Results, TextHit};
use crate::state::Shared;

/// O que foi renomeado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Renamed {
    Component,
    Form,
    Handler,
}

/// A busca da troca: a palavra inteira; no form, sem diferenciar a maiúscula e preservando
/// a do achado (`frmX` e `FrmX`).
pub fn options(kind: Renamed, old: &str) -> SearchOptions {
    let form = kind == Renamed::Form;
    SearchOptions { query: old.to_string(), case: !form, whole: true, regex: false, preserve_case: form }
}

/// O que trocar no projeto ao renomear `old` (no form `form`, que já está certo na memória):
/// as ocorrências no código do usuário e nos outros forms.
pub fn plan(project: &Project, form: &str, kind: Renamed, old: &str) -> Result<Results, String> {
    let options = options(kind, old);
    let forms: Vec<(String, PathBuf)> = project.forms.iter().map(|f| (f.name.clone(), project.file(&f.file))).collect();
    let form_files: Vec<PathBuf> = forms.iter().map(|(_, f)| f.clone()).collect();
    let code = crate::projectsearch::files(&project.dir(), "", "*.json", &form_files);
    let mut results = crate::projectsearch::search(&options, &code, &forms)?;
    // O código gerado pela IDE se refaz com o nome novo.
    results.text.retain(|h: &TextHit| !h.file.components().any(|c| c.as_os_str() == "generated"));
    // O form renomeado já está certo na memória (as referências dele são pelo componente);
    // de um handler, valem também os outros eventos dele, só que apenas os eventos.
    results.forms.retain(|h: &FormHit| match kind {
        Renamed::Handler => is_event(&h.property) && h.value == old,
        _ => h.form != form && h.property != "name" && !h.property.starts_with("on_") && !h.property.starts_with("before_") && !h.property.starts_with("after_"),
    });
    Ok(results)
}

fn is_event(property: &str) -> bool {
    property.starts_with("on_") || property.starts_with("before_") || property.starts_with("after_")
}

/// Os símbolos que o renomear troca no código: o nome; no form, também o tipo dele
/// (`frmX` → `FrmX`, o nome da classe gerada).
pub fn symbols(kind: Renamed, old: &str, new: &str) -> Vec<(String, String)> {
    let mut list = vec![(old.to_string(), new.to_string())];
    if kind == Renamed::Form {
        let (a, b) = (aylis_project::bindings::form_struct(old), aylis_project::bindings::form_struct(new));
        if a != old {
            list.insert(0, (a, b));
        }
    }
    list
}

/// Um renomear esperando as respostas do servidor de linguagem.
struct Pending {
    batch: u64,
    /// Quantas respostas faltam.
    waiting: usize,
    kind: Renamed,
    old: String,
    new: String,
    ids: Ids,
    /// O que vale sem o servidor: as ocorrências por texto e as dos outros forms.
    textual: Results,
    /// As trocas que o servidor deu, por arquivo.
    edits: Vec<(PathBuf, Vec<aylis_tools::lsp::TextEdit>)>,
}

thread_local! {
    static PENDING: std::cell::RefCell<Option<Pending>> = const { std::cell::RefCell::new(None) };
    static NEXT_BATCH: std::cell::Cell<u64> = const { std::cell::Cell::new(1) };
}

fn what(kind: Renamed) -> &'static str {
    match kind {
        Renamed::Component => "o componente",
        Renamed::Form => "o form",
        Renamed::Handler => "o handler",
    }
}

/// Depois de renomear no designer: se o resto do projeto fala do nome antigo, abre a prévia
/// da troca. No código, quem acha as ocorrências é o servidor de linguagem (`textDocument/
/// rename`: só o que é de fato aquele símbolo, nas 8 linguagens), perguntado na posição do
/// primeiro uso no código do usuário; a prévia abre quando ele responde. Sem o servidor
/// pronto (abrindo, sem servidor instalado, ou se ele não acha o símbolo), as ocorrências por
/// texto, com o aviso. Devolve se começou.
pub fn after_rename(ui: &mut Ui, state: &Shared, ids: &Ids, kind: Renamed, old: &str, new: &str) -> bool {
    if old.is_empty() || new.is_empty() || old == new {
        return false;
    }
    let Some(project) = state.borrow().project.clone() else { return false };
    let form = state.borrow().doc.form_name();
    // O form renomeado: o projeto já trocou o nome do form; os outros forms e o código o
    // chamam pelo antigo.
    let textual = match plan(&project, &form, kind, old) {
        Ok(r) => r,
        Err(e) => {
            state.borrow_mut().status = format!("⚠ {e}");
            return false;
        }
    };
    if textual.text.is_empty() && textual.forms.is_empty() {
        return false;
    }
    let batch = NEXT_BATCH.with(|n| {
        let b = n.get();
        n.set(b + 1);
        b
    });
    // Onde perguntar: o primeiro uso de cada símbolo no código do usuário.
    let mut asked = 0;
    if !textual.text.is_empty() && crate::intel::can_rename(state) {
        for (from, to) in symbols(kind, old, new) {
            let Ok(found) = plan(&project, &form, Renamed::Component, &from) else { continue };
            let Some(hit) = found.text.first() else { continue };
            let text = crate::codewindow::text_of(ui, &hit.file).or_else(|| std::fs::read_to_string(&hit.file).ok()).unwrap_or_default();
            let index = text.get(..hit.byte_start).map_or(0, |t| t.chars().count());
            if crate::intel::project_rename(state, &hit.file, aylis_tools::lsp::position_in(&text, index), &to, batch) {
                asked += 1;
            }
        }
    }
    let pending = Pending { batch, waiting: asked, kind, old: old.to_string(), new: new.to_string(), ids: *ids, textual, edits: Vec::new() };
    if asked == 0 {
        // Sem servidor de linguagem para o projeto: só aqui a prévia (por texto) abre.
        show(ui, state, pending, Some("sem servidor de linguagem, as ocorrências do código são por texto"));
    } else {
        state.borrow_mut().status = format!("Renomeado {} {old} → {new}: perguntando ao servidor de linguagem onde trocar no código…", what(kind));
        PENDING.with(|p| *p.borrow_mut() = Some(pending));
    }
    true
}

/// Renomear agora deixaria o código para trás: o servidor de linguagem do projeto ainda não
/// abriu, e o nome antigo aparece no código do usuário. O nome não muda, e a mensagem diz por
/// quê (decidido pelo autor em 08/11: avisar, não esperar nem cair na prévia). Aberto, mesmo
/// preparando o projeto, ele é perguntado: muitas vezes já acha os usos (o renomear volta só se
/// ele responder vazio, em [`lsp_answer`]).
pub fn refused_while_loading(state: &Shared, kind: Renamed, old: &str) -> Option<String> {
    if old.is_empty() || !crate::intel::not_started(state) {
        return None;
    }
    let project = state.borrow().project.clone()?;
    let form = state.borrow().doc.form_name();
    let used = plan(&project, &form, kind, old).is_ok_and(|r| !r.text.is_empty());
    used.then(|| format!("O servidor de linguagem ainda está carregando o projeto, e {} {old} é usado no código: renomeie daqui a pouco, quando ele terminar (o rodapé do editor mostra o estado).", what(kind)))
}

/// A resposta do servidor a um pedido do renomear (`edits` vazio: ele não achou ou falhou).
pub fn lsp_answer(ui: &mut Ui, state: &Shared, batch: u64, edits: Vec<(PathBuf, Vec<aylis_tools::lsp::TextEdit>)>) {
    let done = PENDING.with(|p| {
        let mut p = p.borrow_mut();
        let pending = p.as_mut().filter(|x| x.batch == batch)?;
        pending.edits.extend(edits);
        pending.waiting = pending.waiting.saturating_sub(1);
        if pending.waiting == 0 { p.take() } else { None }
    });
    if let Some(pending) = done {
        // Vazio enquanto ele ainda prepara o projeto, com o nome usado no código: o renomear volta
        // e a caixa de erro diz por quê (senão o form e o código ficavam diferentes).
        if pending.edits.iter().all(|(_, e)| e.is_empty()) && !pending.textual.text.is_empty() && crate::intel::preparing(state) {
            revert(state, pending.kind, &pending.old, &pending.new);
            state.borrow_mut().inspector_key = 0;
            let message = format!("O servidor de linguagem ainda está carregando o projeto, e {} {} é usado no código: renomeie daqui a pouco, quando ele terminar (o rodapé do editor mostra o estado).", what(pending.kind), pending.old);
            let text = ui.tr(&message);
            aylis_controls::message_dlg(ui, &text, aylis_controls::DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
            state.borrow_mut().status = String::new();
            crate::app::refresh(ui, state, &pending.ids);
            return;
        }
        // Com servidor de linguagem, sem a prévia (só sem servidor ela abre, decidido pelo autor em
        // 08/11): ele não achou o símbolo no código, então só os outros forms trocam.
        show(ui, state, pending, None);
    }
}

/// Desfaz o renomear no form aberto (o servidor não pôde trocar o código): o componente ou o
/// form volta ao nome antigo; um handler, os eventos ligados ao novo voltam ao antigo.
fn revert(state: &Shared, kind: Renamed, old: &str, new: &str) {
    let mut st = state.borrow_mut();
    let doc = &mut st.doc;
    match kind {
        Renamed::Component | Renamed::Form => {
            let root = doc.root;
            let id = if doc.ui.name(root) == Some(new) { Some(root) } else { doc.ui.find_in(root, new) };
            if let Some(id) = id {
                let _ = doc.rename_component(id, old);
            }
        }
        Renamed::Handler => {
            let ids: Vec<aylis_core::ComponentId> = doc.tree().into_iter().map(|(id, _)| id).collect();
            for id in ids {
                let events: Vec<&'static str> = doc.ui.prop_infos(id).into_iter().filter(|i| i.kind == aylis_core::PropKind::Event).map(|i| i.name).collect();
                for event in events {
                    let bound = matches!(doc.ui.get_prop(id, event), Ok(aylis_core::PropValue::String(h)) if h == new);
                    if bound {
                        let _ = doc.set_prop(id, event, &serde_json::Value::String(old.to_string()));
                    }
                }
            }
        }
    }
}

/// As trocas do servidor como ocorrências da prévia (o código gerado fica de fora: a geração
/// o refaz com o nome novo).
pub fn hits_of(ui: &Ui, edits: &[(PathBuf, Vec<aylis_tools::lsp::TextEdit>)]) -> Vec<TextHit> {
    let mut hits = Vec::new();
    for (file, list) in edits {
        if file.components().any(|c| c.as_os_str() == "generated") {
            continue;
        }
        let Some(text) = crate::codewindow::text_of(ui, file).or_else(|| std::fs::read_to_string(file).ok()) else { continue };
        let byte = |p: aylis_tools::lsp::Pos| {
            let index = aylis_tools::lsp::index_in(&text, p);
            text.char_indices().nth(index).map_or(text.len(), |(b, _)| b)
        };
        let mut sorted: Vec<&aylis_tools::lsp::TextEdit> = list.iter().collect();
        sorted.sort_by_key(|e| e.start);
        for e in sorted {
            let (start, end) = (byte(e.start), byte(e.end));
            let line_start = text[..start].rfind('\n').map_or(0, |i| i + 1);
            let line_end = text[start..].find('\n').map_or(text.len(), |i| start + i);
            hits.push(TextHit { file: file.clone(), line: e.start.0, text: text[line_start..line_end].trim_end_matches('\r').to_string(), byte_start: start, byte_end: end, replace_with: Some(e.text.clone()) });
        }
    }
    hits
}

/// Abre a prévia: as ocorrências do código (as do servidor ou, com `note`, as por texto) e as
/// dos outros forms.
fn show(ui: &mut Ui, state: &Shared, pending: Pending, note: Option<&str>) {
    let mut results = pending.textual;
    if note.is_none() {
        results.text = hits_of(ui, &pending.edits);
    }
    let (kind, old, new) = (pending.kind, pending.old.as_str(), pending.new.as_str());
    let source = note.map_or_else(|| "pelo servidor de linguagem".to_string(), |n| format!("({n})"));
    let status = format!(
        "Renomeado {} {old} → {new}. Faltam {} no código {source} e {} em outros forms: confira e clique em Substituir (desmarque o que não for).",
        what(kind),
        results.text.len(),
        results.forms.len()
    );
    if results.text.is_empty() && results.forms.is_empty() {
        state.borrow_mut().status = format!("Renomeado {} {old} → {new} (o servidor de linguagem não achou usos no código)", what(kind));
        return;
    }
    // Com o servidor de linguagem (só o que é de fato aquele símbolo), troca direto, como o Delphi
    // (pedido do autor em 08/11: confirmar a cada renomear incomodava); o Ctrl+Z desfaz nos
    // abertos. Só por texto (sem o servidor), a mesma palavra num comentário ou noutro escopo
    // também casaria: aí a prévia continua.
    if note.is_none() {
        let form = state.borrow().doc.form_name();
        let (count, _) = crate::projectsearch::apply(ui, state, &options(kind, old), new, &results, &Default::default());
        // A troca nos outros forms abre cada um: volta ao form onde o renomear foi feito.
        if state.borrow().doc.form_name() != form {
            crate::actions::open_project_form(state, &form);
        }
        state.borrow_mut().status = format!("Renomeado {} {old} → {new}: {count} troca(s) no código e nos outros forms (Ctrl+Z desfaz nos abertos)", what(kind));
        crate::app::refresh(ui, state, &pending.ids);
        return;
    }
    crate::projectsearch::show_replace(ui, state, &pending.ids, options(kind, old), new, results, &status);
}

/// Um handler foi trocado num evento: é renomear se o antigo existe no código e o novo
/// ainda não (senão é só ligar o evento a outro handler).
pub fn handler_renamed(project: &Project, old: &str, new: &str) -> bool {
    if old.is_empty() || new.is_empty() || old == new || !aylis_project::is_identifier(new) {
        return false;
    }
    let forms: Vec<PathBuf> = project.forms.iter().map(|f| project.file(&f.file)).collect();
    let code: Vec<PathBuf> = crate::projectsearch::files(&project.dir(), "", "*.json", &forms).into_iter().filter(|f| !f.components().any(|c| c.as_os_str() == "generated")).collect();
    let found = |name: &str| crate::projectsearch::search(&options(Renamed::Handler, name), &code, &[]).is_ok_and(|r| !r.text.is_empty());
    found(old) && !found(new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_form_rename_keeps_the_capital_of_the_type_name() {
        let o = options(Renamed::Form, "frmClientes");
        let text = "impl FrmClientes { fn x() { frmClientes::abrir(); frmClientesX(); } }";
        let found = o.find_all(text).unwrap();
        assert_eq!(found.len(), 2, "a palavra inteira, das duas caixas");
        let (out, n) = o.replace_all(text, "frmPessoas").unwrap();
        assert_eq!(n, 2);
        assert_eq!(out, "impl FrmPessoas { fn x() { frmPessoas::abrir(); frmClientesX(); } }");
        assert_eq!(symbols(Renamed::Form, "frmClientes", "frmPessoas"), [("FrmClientes".to_string(), "FrmPessoas".to_string()), ("frmClientes".to_string(), "frmPessoas".to_string())], "o form: o tipo e o nome, pelo servidor");
        assert_eq!(symbols(Renamed::Component, "edtNome", "edtCliente").len(), 1);
        let c = options(Renamed::Component, "edtNome");
        assert_eq!(c.find_all("self.edtNome; self.EdtNome; edtNomeX").unwrap().len(), 1, "componente: a caixa vale");
    }
}
