//! O editor de código da IDE: uma janela só, com uma aba por arquivo, e o
//! que o VS Code tem: realce, números de linha, desfazer, localizar e substituir, as
//! operações de linha, os pares que fecham sozinhos, a paleta de comandos e, pelo servidor de
//! linguagem ([`crate::intel`]), as sugestões, a dica, a ajuda dos parâmetros, ir para a
//! definição, as referências, renomear, formatar, os símbolos, as correções e os erros
//! enquanto digita (o painel Problemas).
//!
//! Depurando, o painel da direita mostra os botões, a pilha, as variáveis e o "Avaliar"; a
//! linha onde o programa parou abre na aba do arquivo dela.
//!
//! O editor fica embutido na janela da IDE, no lugar do designer
//! ([`set_host`]; os botões "Design | Código" alternam). Uma aba pode ir para outra janela
//! (Arquivo → Abrir em outra janela, ou arrastando a aba para fora da faixa), que junta
//! quantas abas forem para lá; arrastar uma aba para fora da janela de fora a traz de
//! volta, e fechar essa janela devolve as abas dela à IDE. Cada editor (o embutido e o de
//! fora) tem as suas abas; o "atual" é o do último evento.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, Diagnostic, Edit, FlowLayout, Form, Label, Layout, ListBox, MainMenu, Memo, MenuItem, Panel, PopupMenu, Splitter, Suggestion, TabStrip, TreeNode, TreeView};
use aylis_tools::lsp::Pos;
use aylis_core::{Align, ComponentId, Ui};

use crate::widgets::muted;

/// Um arquivo aberto (uma aba).
struct Tab {
    file: PathBuf,
    memo: ComponentId,
    modified: bool,
    /// A marca do arquivo quando foi lido ou salvo ([`crate::disk`]).
    disk: Option<aylis_project::FileStamp>,
    /// Já avisou que fechar descarta as alterações.
    warned: bool,
}

/// Um editor: o embutido na IDE ou uma janela de código.
struct Editor {
    form: ComponentId,
    /// Embutido na janela da IDE (senão, uma janela própria).
    embedded: bool,
    tab_bar: ComponentId,
    save: ComponentId,
    status: ComponentId,
    activity: ComponentId,
    /// O painel de depuração (escondido fora da depuração) e o divisor dele.
    panel: ComponentId,
    splitter: ComponentId,
    tabs: Vec<Tab>,
    active: usize,
    /// Fechar a janela com alterações: avisa uma vez.
    warned: bool,
    /// Localizar e substituir (Ctrl+F, Ctrl+H).
    find: Find,
    /// O painel de baixo (Problemas ou Referências): o título e a lista.
    bottom: ComponentId,
    bottom_splitter: ComponentId,
    bottom_title: ComponentId,
    bottom_list: ComponentId,
    /// O botão dos erros e avisos (abre o painel Problemas) e o nome do servidor.
    problems: ComponentId,
    server: ComponentId,
    /// O tamanho da fonte do código (Ctrl+= e Ctrl+-; 0 = o do tema).
    font_size: f32,
    /// O minimapa à direita dos editores (Exibir → Minimapa).
    minimap: bool,
    /// O tema de cores do código (Exibir → Tema do editor; vazio: o da IDE).
    color_theme: String,
    /// Onde ficam a trilha e os editores (à direita dos fontes).
    area: ComponentId,
    /// A trilha de cima (arquivo › tipo › função onde o cursor está).
    breadcrumb: ComponentId,
    /// A faixa do arquivo gerado pela IDE (somente leitura: a geração o refaz).
    generated_banner: ComponentId,
}

/// A barra de localizar e substituir.
#[derive(Clone, Copy)]
struct Find {
    bar: ComponentId,
    replace_row: ComponentId,
    query: ComponentId,
    replace: ComponentId,
    case: ComponentId,
    whole: ComponentId,
    /// Expressão regular, só na seleção e preservar maiúsculas na substituição.
    regex: ComponentId,
    in_selection: ComponentId,
    keep_case: ComponentId,
    count: ComponentId,
}

thread_local! {
    static EDITORS: RefCell<Vec<Editor>> = const { RefCell::new(Vec::new()) };
    /// O form do editor atual (o do último evento).
    static CURRENT: std::cell::Cell<Option<ComponentId>> = const { std::cell::Cell::new(None) };
    /// Onde o editor embutido fica na IDE (`None`: só janelas).
    static HOST: std::cell::Cell<Option<ComponentId>> = const { std::cell::Cell::new(None) };
    static HOOKS: RefCell<Option<Rc<Hooks>>> = const { RefCell::new(None) };
}

/// Onde o editor embutido fica na janela da IDE (o centro, no lugar do designer). Sem isto,
/// o código abre numa janela própria.
pub fn set_host(host: Option<ComponentId>) {
    HOST.with(|h| h.set(host));
}

/// O que a janela de código pede à depuração (a IDE liga; sem ganchos, só edita).
pub struct Hooks {
    /// Liga/desliga o ponto de parada da linha (a partir de 1).
    pub toggle: Box<dyn Fn(&mut Ui, &Path, usize)>,
    pub command: Box<dyn Fn(&mut Ui, crate::debugger::Cmd)>,
    /// As linhas com ponto de parada do arquivo.
    pub breakpoints: Box<dyn Fn(&Path) -> Vec<usize>>,
    /// A condição (e a contagem de passagens) do ponto de parada da linha.
    pub condition: Box<dyn Fn(&Path, usize) -> (String, String)>,
    pub set_condition: Box<dyn Fn(&mut Ui, &Path, usize, &str, &str)>,
    /// Executar até o cursor (F4).
    pub run_to: Box<dyn Fn(&mut Ui, &Path, usize)>,
    /// Os fontes do projeto: a pasta do código e os arquivos (o painel da esquerda e o
    /// Ctrl+F12).
    pub sources: Box<dyn Fn() -> (Option<PathBuf>, Vec<PathBuf>)>,
    /// O servidor de linguagem ([`crate::intel`]): o editor avisa o que mudou e pede.
    pub lsp: Box<dyn Fn(&mut Ui, LspEvent)>,
    /// Mostra o código embutido (`true`) ou volta ao designer (`false`) no centro da IDE.
    pub view: Box<dyn Fn(&mut Ui, bool)>,
    /// Pesquisar no projeto (Ctrl+Shift+F), com o texto selecionado.
    pub search: Box<dyn Fn(&mut Ui, &str)>,
    /// As abas do editor embutido mudaram (abriu, fechou, trocou a ativa, alterou ou salvou):
    /// a IDE refaz a faixa de abas dela, que junta forms e código.
    pub tabs: Box<dyn Fn(&mut Ui)>,
    /// O fonte foi salvo pelo editor (Ctrl+S, o botão): a IDE salva junto o form dele, se
    /// alterado (conferência de 08/11).
    pub saved: Box<dyn Fn(&mut Ui, &Path)>,
}

/// Avisa a IDE de que as abas do código mudaram.
fn notify_tabs(ui: &mut Ui) {
    if let Some(h) = hooks() {
        (h.tabs)(ui);
    }
}

/// O form do editor embutido, se existe.
fn embedded_form(ui: &Ui) -> Option<ComponentId> {
    with_editors(ui, |list| list.iter().find(|ed| ed.embedded).map(|ed| ed.form)).flatten()
}

/// As abas do editor embutido: o arquivo e se há alteração não salva.
pub fn embedded_tabs(ui: &Ui) -> Vec<(PathBuf, bool)> {
    with_editors(ui, |list| list.iter().find(|ed| ed.embedded).map(|ed| ed.tabs.iter().map(|t| (t.file.clone(), t.modified)).collect())).flatten().unwrap_or_default()
}

/// O arquivo da aba ativa do editor embutido.
pub fn embedded_active(ui: &Ui) -> Option<PathBuf> {
    with_editors(ui, |list| list.iter().find(|ed| ed.embedded).and_then(|ed| ed.tabs.get(ed.active).map(|t| t.file.clone()))).flatten()
}

/// Fecha a aba de `file` no editor embutido (com alteração não salva, avisa uma vez, como o
/// Ctrl+F4).
pub fn close_embedded(ui: &mut Ui, file: &Path) {
    let key = crate::debugger::norm(file);
    let Some(form) = embedded_form(ui) else { return };
    let Some(index) = with_editor_at(ui, form, |ed| ed.tabs.iter().position(|t| t.file == key)).flatten() else { return };
    CURRENT.with(|c| c.set(Some(form)));
    activate(ui, index);
    close_active(ui);
}

/// Salva os fontes alterados de todos os editores (o embutido e as janelas de código). Antes de
/// compilar, executar, depurar e publicar: a compilação lê o disco, e o código alterado no editor
/// ficava de fora (o app rodava com o código velho; conferência de 08/11).
pub fn save_all(ui: &mut Ui) -> usize {
    // O campo do Object Inspector em edição grava antes (o form entra junto na compilação).
    crate::inspector::commit_pending(ui);
    let memos: Vec<(ComponentId, ComponentId)> =
        with_editors(ui, |list| list.iter().flat_map(|ed| ed.tabs.iter().filter(|t| t.modified).map(move |t| (ed.form, t.memo))).collect()).unwrap_or_default();
    for &(form, memo) in &memos {
        CURRENT.with(|c| c.set(Some(form)));
        save_memo(ui, memo);
    }
    memos.len()
}

/// Salva a aba ativa do editor embutido, ou (`all`) todas as alteradas. Devolve quantas.
pub fn save_embedded(ui: &mut Ui, all: bool) -> usize {
    let Some(form) = embedded_form(ui) else { return 0 };
    let memos: Vec<ComponentId> = with_editor_at(ui, form, |ed| {
        if all {
            ed.tabs.iter().filter(|t| t.modified).map(|t| t.memo).collect()
        } else {
            ed.tabs.get(ed.active).map(|t| t.memo).into_iter().collect()
        }
    })
    .unwrap_or_default();
    CURRENT.with(|c| c.set(Some(form)));
    for &memo in &memos {
        save_memo(ui, memo);
    }
    memos.len()
}

/// O que o editor pede ao servidor de linguagem (os índices em caracteres do texto).
pub enum LspEvent {
    Opened { file: PathBuf, text: String },
    Changed { file: PathBuf, text: String },
    Saved { file: PathBuf, text: String },
    Closed { file: PathBuf },
    /// As sugestões no cursor (digitando ou Ctrl+Espaço).
    Complete { file: PathBuf, index: usize },
    /// A ajuda dos parâmetros (abriu "(" ou Ctrl+Shift+Espaço).
    Signature { file: PathBuf, index: usize },
    /// O mouse parou numa palavra.
    Hover { file: PathBuf, index: usize },
    /// F12 ou Ctrl+clique.
    Definition { file: PathBuf, index: usize, word: String },
    /// Shift+F12.
    References { file: PathBuf, index: usize, word: String },
    /// F2.
    Rename { file: PathBuf, index: usize, name: String },
    /// Shift+Alt+F.
    Format { file: PathBuf },
    /// Ctrl+Shift+O.
    Symbols { file: PathBuf },
    /// Ctrl+.
    QuickFix { file: PathBuf, index: usize },
}

/// Manda um pedido ao servidor (sem ganchos, nada).
fn lsp_event(ui: &mut Ui, event: LspEvent) {
    if let Some(h) = hooks() {
        (h.lsp)(ui, event);
    }
}

pub fn set_hooks(hooks: Hooks) {
    HOOKS.with(|h| *h.borrow_mut() = Some(Rc::new(hooks)));
}

fn hooks() -> Option<Rc<Hooks>> {
    HOOKS.with(|h| h.borrow().clone())
}

/// Lê o editor atual (o do último evento; sem ele, o embutido ou o primeiro). Não chame o
/// `ui` de dentro de `f`: um evento disparado ali voltaria aqui com o editor emprestado.
fn with_editor<R>(ui: &Ui, f: impl FnOnce(&mut Editor) -> R) -> Option<R> {
    let current = CURRENT.with(|c| c.get());
    with_editors(ui, |list| {
        let i = list.iter().position(|ed| Some(ed.form) == current).or_else(|| list.iter().position(|ed| ed.embedded)).or((!list.is_empty()).then_some(0))?;
        Some(f(&mut list[i]))
    })
    .flatten()
}

/// Lê o editor do form `form`.
fn with_editor_at<R>(ui: &Ui, form: ComponentId, f: impl FnOnce(&mut Editor) -> R) -> Option<R> {
    with_editors(ui, |list| list.iter_mut().find(|ed| ed.form == form).map(f)).flatten()
}

/// Todos os editores vivos (os de janelas fechadas saem da lista).
fn with_editors<R>(ui: &Ui, f: impl FnOnce(&mut Vec<Editor>) -> R) -> Option<R> {
    EDITORS.with(|e| {
        let mut list = e.borrow_mut();
        list.retain(|ed| ui.contains(ed.form));
        Some(f(&mut list))
    })
}

/// O editor de `id` (o form dele ou qualquer controle dentro) passa a ser o atual. Cada
/// evento do editor chama isto primeiro: os comandos valem para a janela onde aconteceram.
fn select_of(ui: &Ui, id: ComponentId) {
    let forms: Vec<ComponentId> = with_editors(ui, |list| list.iter().map(|ed| ed.form).collect()).unwrap_or_default();
    let mut current = Some(id);
    while let Some(c) = current {
        if forms.contains(&c) {
            CURRENT.with(|cur| cur.set(Some(c)));
            return;
        }
        current = ui.parent(c);
    }
}

/// O form do editor embutido (ou, sem lugar na IDE, o de uma janela), criando se preciso.
fn primary(ui: &mut Ui) -> ComponentId {
    let host = HOST.with(|h| h.get()).filter(|&h| ui.contains(h));
    let found = with_editors(ui, |list| list.iter().find(|ed| ed.embedded == host.is_some()).map(|ed| ed.form)).flatten();
    found.unwrap_or_else(|| create(ui, host))
}

/// A janela de código de fora (criada se não houver).
fn external(ui: &mut Ui) -> ComponentId {
    let found = with_editors(ui, |list| list.iter().find(|ed| !ed.embedded).map(|ed| ed.form)).flatten();
    found.unwrap_or_else(|| create(ui, None))
}

/// Mostra o editor do form: o embutido troca o centro da IDE para o código; a janela vem
/// para a frente.
fn reveal(ui: &mut Ui, form: ComponentId) {
    if with_editor_at(ui, form, |ed| ed.embedded).unwrap_or(false) {
        if let Some(h) = hooks() {
            (h.view)(ui, true);
        }
    } else {
        let _ = ui.show(form);
    }
}

/// O editor do arquivo (o form) e o índice da aba, se ele está aberto em algum.
fn find_file(ui: &Ui, key: &Path) -> Option<(ComponentId, usize)> {
    with_editors(ui, |list| list.iter().find_map(|ed| ed.tabs.iter().position(|t| t.file == key).map(|i| (ed.form, i)))).flatten()
}

/// Há código alterado sem salvar em algum editor.
pub fn has_unsaved(ui: &Ui) -> bool {
    with_editors(ui, |list| list.iter().any(|ed| ed.tabs.iter().any(|t| t.modified))).unwrap_or(false)
}

/// Fechar a IDE com código não salvo no editor embutido: avisa uma vez (mostra o código);
/// `true` se pode fechar.
pub fn confirm_close(ui: &mut Ui) -> bool {
    let embedded = with_editors(ui, |list| list.iter().find(|ed| ed.embedded && ed.tabs.iter().any(|t| t.modified) && !ed.warned).map(|ed| ed.form)).flatten();
    let Some(form) = embedded else { return true };
    with_editor_at(ui, form, |ed| ed.warned = true);
    CURRENT.with(|c| c.set(Some(form)));
    reveal(ui, form);
    set_status(ui, "Há código alterado sem salvar: Ctrl+S salva; feche de novo para descartar");
    false
}

/// Fecha as abas de código de todos os editores (o projeto fechou ou trocou: as dele não
/// ficam abertas no outro, conferência de 08/11). Com código não salvo, avisa uma vez (como
/// ao fechar a IDE) e devolve `false`, sem fechar nada.
pub fn close_all_tabs(ui: &mut Ui) -> bool {
    if has_unsaved(ui) && !confirm_close(ui) {
        return false;
    }
    let files: Vec<(ComponentId, PathBuf)> = with_editors(ui, |list| list.iter().flat_map(|ed| ed.tabs.iter().map(move |t| (ed.form, t.file.clone()))).collect()).unwrap_or_default();
    for (form, file) in files {
        let Some(index) = with_editor_at(ui, form, |ed| ed.tabs.iter().position(|t| t.file == file)).flatten() else { continue };
        // Sem perguntar de novo: o aviso já foi dado (ou não havia o que salvar).
        with_editor_at(ui, form, |ed| {
            if let Some(t) = ed.tabs.get_mut(index) {
                t.modified = false;
            }
        });
        CURRENT.with(|c| c.set(Some(form)));
        activate(ui, index);
        close_active(ui);
    }
    true
}

/// O código embutido tem abas (sem nenhuma, o centro da IDE fica no designer).
pub fn embedded_has_tabs(ui: &Ui) -> bool {
    with_editors(ui, |list| list.iter().any(|ed| ed.embedded && !ed.tabs.is_empty())).unwrap_or(false)
}

/// O arquivo e o editor da aba ativa.
fn active(ui: &Ui) -> Option<(PathBuf, ComponentId)> {
    with_editor(ui, |ed| ed.tabs.get(ed.active).map(|t| (t.file.clone(), t.memo))).flatten()
}

/// O arquivo da aba ativa e a linha do cursor (a partir de 1).
pub fn active_location(ui: &Ui) -> Option<(PathBuf, usize)> {
    let (file, memo) = active(ui)?;
    let line = ui.get::<Memo>(memo).map_or(0, |m| m.caret_position().0) + 1;
    Some((file, line))
}

/// Mostra na janela de código o que a IDE está fazendo (vazio: nada em andamento). Quem
/// aperta F9 na janela de código não vê a janela principal, que costuma ficar atrás.
pub fn show_activity(ui: &mut Ui, text: &str) {
    let Some(label) = with_editor(ui, |ed| ed.activity) else { return };
    if ui.get::<Label>(label).is_some_and(|l| l.text != text) {
        let _ = ui.set_prop(label, "text", text);
    }
}

/// Mexe no editor da aba de `file`, se ela estiver aberta.
pub fn update(ui: &mut Ui, file: &Path, f: impl FnOnce(&mut Memo)) {
    let Some(memo) = memo_of(ui, file) else { return };
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        f(m);
    }
    ui.request_repaint();
}

/// O painel de depuração (no editor embutido; sem lugar na IDE, na janela de código), que
/// é criado vazio se for preciso, e mostra ou esconde.
pub fn debug_panel(ui: &mut Ui, show: bool) -> Option<ComponentId> {
    if !show && with_editors(ui, |list| list.is_empty()).unwrap_or(true) {
        return None;
    }
    let form = primary(ui);
    let (panel, splitter) = with_editor_at(ui, form, |ed| (ed.panel, ed.splitter))?;
    for id in [panel, splitter] {
        if let Some(c) = ui.control_mut(id) {
            c.set_visible(show);
        }
    }
    if show {
        reveal(ui, form);
    }
    ui.request_repaint();
    Some(panel)
}

/// A linguagem do realce pela extensão do arquivo.
pub fn syntax_of(file: &Path) -> String {
    let ext = file.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase);
    // Uma linguagem importada do VS Code com essa extensão.
    if let Some(lang) = ext.as_deref().and_then(aylis_controls::textmate::language_for_extension) {
        return lang;
    }
    match ext.as_deref() {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("json") | Some("aylisproj") => "json",
        Some("pas" | "pp" | "lpr" | "dpr" | "inc") => "pascal",
        Some("ts" | "tsx") => "typescript",
        Some("js" | "jsx" | "mjs" | "cjs") => "javascript",
        Some("cs") => "csharp",
        Some("java") => "java",
        Some("go") => "go",
        Some("c") => "c",
        Some("cpp" | "cc" | "cxx" | "h" | "hpp" | "hh") => "cpp",
        _ => "",
    }
    .to_string()
}

/// Os controles da janela de código com a aba de um arquivo (para os testes).
#[derive(Clone, Copy, Debug)]
pub struct CodeWindow {
    pub form: ComponentId,
    pub memo: ComponentId,
    pub save: ComponentId,
    pub status: ComponentId,
}

/// Abre `file` no editor de código (na aba que já tem o arquivo, em qualquer editor; senão
/// numa aba nova do embutido na IDE), com o cursor no começo da linha `line` (a partir de
/// 1). Linha 0: a aba fica como estava.
pub fn open(ui: &mut Ui, file: &Path, line: usize) -> Result<CodeWindow, String> {
    let key = crate::debugger::norm(file);
    let form = match find_file(ui, &key) {
        Some((form, _)) => form,
        None => primary(ui),
    };
    open_in(ui, form, &key, line)
}

/// Como [`open`], mas um arquivo novo abre no editor atual (o da janela onde o usuário está:
/// Ctrl+P, ir para a definição, a lista dos fontes).
fn open_here(ui: &mut Ui, file: &Path, line: usize) -> Result<CodeWindow, String> {
    let key = crate::debugger::norm(file);
    let form = match find_file(ui, &key) {
        Some((form, _)) => form,
        None => match with_editor(ui, |ed| ed.form) {
            Some(form) => form,
            None => primary(ui),
        },
    };
    open_in(ui, form, &key, line)
}

/// Um lugar do código: o arquivo e a posição do cursor (em caracteres).
type Place = (PathBuf, usize);

/// Por onde o editor passou, para voltar e avançar (os botões extras do mouse, Alt+← e
/// Alt+→), como no VS Code: cada salto (abrir um arquivo, ir para a declaração, um achado da
/// pesquisa) guarda o lugar de onde saiu.
#[derive(Default)]
struct Trail {
    back: Vec<Place>,
    forward: Vec<Place>,
    /// Voltando ou avançando: o salto não entra no histórico.
    moving: bool,
}

/// Quantos lugares o histórico guarda.
const TRAIL_MAX: usize = 100;

thread_local! {
    static TRAIL: RefCell<Trail> = RefCell::new(Trail::default());
}

/// Onde o cursor está no editor atual.
fn here(ui: &Ui) -> Option<Place> {
    let (file, memo) = with_editor(ui, |ed| ed.tabs.get(ed.active).map(|t| (t.file.clone(), t.memo))).flatten()?;
    let caret = ui.get::<Memo>(memo)?.selection().0;
    Some((file, caret))
}

/// Guarda o lugar de agora antes de um salto (fora de uma volta ou avanço).
fn remember_place(ui: &Ui) {
    if TRAIL.with(|t| t.borrow().moving) {
        return;
    }
    let Some(place) = here(ui) else { return };
    TRAIL.with(|t| {
        let mut t = t.borrow_mut();
        if t.back.last() != Some(&place) {
            t.back.push(place);
            if t.back.len() > TRAIL_MAX {
                t.back.remove(0);
            }
        }
        t.forward.clear();
    });
}

/// Volta (ou avança) um lugar no histórico do editor. Devolve se havia para onde ir.
pub fn navigate(ui: &mut Ui, forward: bool) -> bool {
    let Some(target) = TRAIL.with(|t| {
        let mut t = t.borrow_mut();
        if forward {
            t.forward.pop()
        } else {
            t.back.pop()
        }
    }) else {
        return false;
    };
    if let Some(now) = here(ui) {
        TRAIL.with(|t| {
            let mut t = t.borrow_mut();
            if forward { t.back.push(now) } else { t.forward.push(now) }
        });
    }
    TRAIL.with(|t| t.borrow_mut().moving = true);
    // Abre na linha do lugar (rola até ela) e põe o cursor na coluna.
    let line = text_of(ui, &target.0).map_or(0, |text| text.chars().take(target.1).filter(|&c| c == '\n').count());
    let opened = open_here(ui, &target.0, line + 1);
    TRAIL.with(|t| t.borrow_mut().moving = false);
    let Ok(w) = opened else { return false };
    if let Some(m) = ui.get_mut::<Memo>(w.memo) {
        let index = target.1.min(m.text.chars().count());
        m.select(index, index);
    }
    show_status(ui);
    true
}

/// Abre `key` (já normalizado) no editor do form `form`.
fn open_in(ui: &mut Ui, form: ComponentId, key: &Path, line: usize) -> Result<CodeWindow, String> {
    remember_place(ui);
    CURRENT.with(|c| c.set(Some(form)));
    let existing = with_editor_at(ui, form, |ed| ed.tabs.iter().position(|t| t.file == key)).flatten();
    let index = match existing {
        Some(i) => i,
        None => add_tab(ui, key)?,
    };
    activate(ui, index);
    let (memo, save, status) = with_editor_at(ui, form, |ed| (ed.tabs[index].memo, ed.save, ed.status)).ok_or("janela de código fechada")?;
    if line > 0 {
        if let Some(m) = ui.get_mut::<Memo>(memo) {
            m.go_to_line(line - 1);
        }
    }
    show_status(ui);
    crate::lang::translate(ui);
    reveal(ui, form);
    ui.set_focus(memo);
    Ok(CodeWindow { form, memo, save, status })
}

/// Leva a aba `index` do editor `from` para o editor `to` (no fim das abas dele), com o
/// texto, o cursor e o desfazer como estão.
fn move_tab(ui: &mut Ui, from: ComponentId, index: usize, to: ComponentId) {
    if from == to {
        return;
    }
    let Some((tab, strip)) = with_editor_at(ui, from, |ed| (index < ed.tabs.len()).then(|| (ed.tabs.remove(index), ed.tab_bar))).flatten() else { return };
    if let Some(s) = ui.get_mut::<TabStrip>(strip) {
        s.remove(index);
    }
    let Some((area, to_strip)) = with_editor_at(ui, to, |ed| (ed.area, ed.tab_bar)) else { return };
    let _ = ui.set_parent(tab.memo, Some(area));
    if let Some(s) = ui.get_mut::<TabStrip>(to_strip) {
        let i = s.add(file_name(&tab.file), "file");
        if let Some(t) = s.tab_mut(i) {
            t.hint = tab.file.display().to_string();
        }
    }
    let (memo, modified) = (tab.memo, tab.modified);
    let new_index = with_editor_at(ui, to, |ed| {
        ed.tabs.push(tab);
        ed.tabs.len() - 1
    })
    .unwrap_or(0);
    // O que ficou para trás: a aba vizinha, ou nada (o embutido volta ao designer; a
    // janela de fora fecha).
    let (left, embedded) = with_editor_at(ui, from, |ed| (ed.tabs.len(), ed.embedded)).unwrap_or((0, false));
    CURRENT.with(|c| c.set(Some(from)));
    if left > 0 {
        activate(ui, index.min(left - 1));
    } else if embedded {
        if let Some(h) = hooks() {
            (h.view)(ui, false);
        }
    } else {
        ui.close(from);
    }
    CURRENT.with(|c| c.set(Some(to)));
    activate(ui, new_index);
    if modified {
        show_status(ui);
    }
    reveal(ui, to);
    ui.set_focus(memo);
}

/// A aba ativa vai para a outra janela: do embutido para a janela de fora (criada se
/// preciso), e da janela de fora de volta à IDE.
pub fn detach_active(ui: &mut Ui) {
    let Some((form, index, embedded)) = with_editor(ui, |ed| (ed.form, ed.active, ed.embedded)) else { return };
    if with_editor_at(ui, form, |ed| ed.tabs.is_empty()).unwrap_or(true) {
        return;
    }
    let target = if embedded { external(ui) } else { primary(ui) };
    move_tab(ui, form, index, target);
}

/// Cria um editor sem abas: embutido em `host` (o centro da IDE) ou numa janela.
fn create(ui: &mut Ui, host: Option<ComponentId>) -> ComponentId {
    let embedded = host.is_some();
    let form = ui.add_root(Form::new("Código", 1180.0, 760.0));
    if let Some(host) = host {
        let _ = ui.set_parent(form, Some(host));
        ui.control_mut(form).unwrap().set_align(Align::Client);
    }

    // Arquivo: Salvar (Ctrl+S) e Fechar arquivo (Ctrl+F4) valem para a aba ativa. Embutido,
    // não há barra de menu nem botão Salvar: os mesmos itens ficam no menu do
    // botão direito, com os atalhos valendo do mesmo jeito, e o Salvar é o da IDE.
    let menu = if embedded { ui.add(PopupMenu::default(), form) } else { ui.add(MainMenu::default(), form) };
    if embedded {
        let _ = ui.set_prop(form, "popup_menu", aylis_core::PropValue::Component(Some(menu)));
    }
    // No menu do botão direito, os clássicos primeiro (como em todo editor).
    if embedded {
        clipboard_items(ui, menu);
        menu_separator(ui, menu);
    }
    let file_menu = ui.add(MenuItem::new("Arquivo"), menu);
    let mi_save = ui.add(MenuItem::new("Salvar").with_shortcut("Ctrl+S"), file_menu);
    let mi_close = ui.add(MenuItem::new("Fechar arquivo").with_shortcut("Ctrl+F4"), file_menu);
    // A aba vai para a janela de fora (ou, de lá, volta para a IDE).
    let mi_detach = ui.add(MenuItem::new(if embedded { "Abrir em outra janela" } else { "Voltar para a IDE" }).with_shortcut("Ctrl+Shift+D"), file_menu);
    ui.on_click(mi_detach, |ui, sender| {
        select_of(ui, sender);
        detach_active(ui);
    });

    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(if embedded { 28.0 } else { 40.0 }).set_padding(if embedded { [6.0, 2.0, 6.0, 2.0] } else { [6.0; 4] });
    let save = ui.add(Button::new("Salvar"), bar);
    ui.control_mut(save).unwrap().set_align(Align::Left).set_width(90.0).set_visible(!embedded);
    let status = ui.add(Label::new(""), bar);
    ui.control_mut(status).unwrap().set_align(Align::Right).set_width(220.0).set_margins([12.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(status, "h_align", "trailing");
    muted(ui, status);
    // Os erros e avisos (clique abre o painel Problemas) e o servidor de linguagem.
    let problems = ui.add(Button::new("⊗ 0  ⚠ 0"), bar);
    ui.control_mut(problems).unwrap().set_align(Align::Right).set_width(96.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let _ = ui.set_prop(problems, "hint", "Problemas (Ctrl+Shift+M)");
    ui.on_click(problems, |ui, sender| {
        select_of(ui, sender);
        toggle_bottom(ui, Bottom::Problems);
    });
    let server = ui.add(Label::new(""), bar);
    // O rótulo recebe o mouse (o `Label` não recebe): senão a dica com o
    // detalhe do servidor nunca aparecia (roteiro 58).
    ui.control_mut(server).unwrap().set_align(Align::Right).set_width(280.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_hit_test(true);
    let _ = ui.set_prop(server, "h_align", "trailing");
    muted(ui, server);
    let activity = ui.add(Label::new(""), bar);
    // Embutido, o que a IDE está fazendo já aparece na barra de status dela.
    ui.control_mut(activity).unwrap().set_align(Align::Client).set_margins([12.0, 0.0, 0.0, 0.0]).set_visible(!embedded);

    // As abas, uma por arquivo (como as do navegador): o × fecha (avisando da alteração não
    // salva), o "+" abre outro fonte, arrastar troca a ordem. Embutido, as abas são as da
    // IDE (form e código juntos: `panels::tabs`) e esta faixa fica escondida.
    let mut strip = TabStrip::new();
    strip.auto_close = false;
    let tab_bar = ui.add(strip, form);
    ui.control_mut(tab_bar).unwrap().set_align(Align::Top).set_height(38.0).set_visible(!embedded);
    ui.on(tab_bar, "on_change", |ui, sender, args| {
        select_of(ui, sender);
        if let aylis_core::EventArgs::Value(v) = args {
            if let Ok(i) = v.to_i64() {
                activate(ui, i.max(0) as usize);
                if let Some((_, memo)) = active(ui) {
                    ui.set_focus(memo);
                }
            }
        }
    });
    ui.on(tab_bar, "on_close", |ui, sender, args| {
        select_of(ui, sender);
        if let aylis_core::EventArgs::Value(v) = args {
            if let Ok(i) = v.to_i64() {
                activate(ui, i.max(0) as usize);
                close_active(ui);
            }
        }
    });
    ui.on(tab_bar, "on_add", |ui, sender, _| {
        select_of(ui, sender);
        open_source_dialog(ui);
    });
    // A aba arrastada para fora da faixa vai para a outra janela.
    ui.on(tab_bar, "on_tear_off", |ui, sender, args| {
        select_of(ui, sender);
        if let Some(i) = event_index(args) {
            activate(ui, i);
            detach_active(ui);
        }
    });
    ui.on(tab_bar, "on_reorder", move |ui, sender, args| {
        select_of(ui, sender);
        let aylis_core::EventArgs::Text(t) = args else { return };
        let mut parts = t.split(',').filter_map(|p| p.trim().parse::<usize>().ok());
        let (Some(from), Some(to)) = (parts.next(), parts.next()) else { return };
        let active_now = ui.get::<TabStrip>(tab_bar).map(|s| s.tab_index.max(0) as usize).unwrap_or(0);
        with_editor(ui, |ed| {
            if from < ed.tabs.len() && to < ed.tabs.len() {
                let tab = ed.tabs.remove(from);
                ed.tabs.insert(to, tab);
                ed.active = active_now;
            }
        });
    });
    let find = create_find_bar(ui, form);

    // Sem lista de fontes aqui: na IDE quem navega é a árvore do Projeto; em qualquer
    // janela, o Ctrl+F12 (Abrir fonte) acha pelo nome.

    // O painel de depuração à direita (o depurador o preenche quando o programa para).
    let panel = ui.add(Panel::default(), form);
    ui.control_mut(panel).unwrap().set_align(Align::Right).set_width(440.0).set_padding(6.0).set_visible(false);
    let splitter = ui.add(Splitter::new(Align::Right), form);
    ui.control_mut(splitter).unwrap().set_visible(false);

    // O painel de baixo: Problemas (Ctrl+Shift+M) e Referências (Shift+F12).
    let bottom = ui.add(Panel::default(), form);
    ui.control_mut(bottom).unwrap().set_align(Align::Bottom).set_height(200.0).set_padding(6.0).set_visible(false);
    let bottom_splitter = ui.add(Splitter::new(Align::Bottom), form);
    ui.control_mut(bottom_splitter).unwrap().set_visible(false);
    let head = ui.add(Layout::default(), bottom);
    ui.control_mut(head).unwrap().set_align(Align::Top).set_height(26.0);
    let close_bottom = ui.add(Button::new("×"), head);
    ui.control_mut(close_bottom).unwrap().set_align(Align::Right).set_width(30.0).set_style("flat");
    ui.on_click(close_bottom, |ui, sender| {
        select_of(ui, sender);
        show_bottom(ui, None);
    });
    let bottom_title = ui.add(Label::new(""), head);
    ui.control_mut(bottom_title).unwrap().set_align(Align::Client);
    let bottom_list = ui.add(TreeView::default(), bottom);
    ui.control_mut(bottom_list).unwrap().set_align(Align::Client);
    // Clique numa linha: vai para o lugar ("arquivo|linha|coluna" no dado do nó).
    ui.on(bottom_list, "on_change", |ui, sender, _| {
        select_of(ui, sender);
        let data = ui.get::<TreeView>(sender).and_then(|t| t.selected()).map(|n| n.data.clone()).unwrap_or_default();
        let mut parts = data.rsplitn(3, '|');
        let (col, line, file) = (parts.next(), parts.next(), parts.next());
        if let (Some(file), Some(line), Some(col)) = (file, line.and_then(|l| l.parse::<usize>().ok()), col.and_then(|c| c.parse::<usize>().ok())) {
            go_to_position(ui, Path::new(file), (line, col));
        }
    });

    create_menus(ui, menu, embedded);

    if let Some(h) = hooks() {
        let debug_menu = ui.add(MenuItem::new("Depurar"), menu);
        let toggle_item = ui.add(MenuItem::new("Ponto de parada").with_shortcut("F5"), debug_menu);
        {
            let h = h.clone();
            ui.on_click(toggle_item, move |ui, sender| {
                select_of(ui, sender);
                let Some((file, memo)) = active(ui) else { return };
                let line = ui.get::<Memo>(memo).map_or(0, |m| m.caret_position().0) + 1;
                (h.toggle)(ui, &file, line);
            });
        }
        let cond_item = ui.add(MenuItem::new("Condição do ponto de parada…"), debug_menu);
        {
            let h = h.clone();
            ui.on_click(cond_item, move |ui, sender| {
                select_of(ui, sender);
                let Some((file, memo)) = active(ui) else { return };
                let line = ui.get::<Memo>(memo).map_or(0, |m| m.caret_position().0) + 1;
                condition_dialog(ui, h.clone(), file, line);
            });
        }
        let run_item = ui.add(MenuItem::new("Executar até o cursor").with_shortcut("F4"), debug_menu);
        {
            let h = h.clone();
            ui.on_click(run_item, move |ui, sender| {
                select_of(ui, sender);
                let Some((file, memo)) = active(ui) else { return };
                let line = ui.get::<Memo>(memo).map_or(0, |m| m.caret_position().0) + 1;
                (h.run_to)(ui, &file, line);
            });
        }
        for (cmd, text, shortcut) in crate::debugger::Cmd::ALL {
            let item = ui.add(MenuItem::new(text).with_shortcut(shortcut), debug_menu);
            let h = h.clone();
            ui.on_click(item, move |ui, sender| {
                select_of(ui, sender);
                (h.command)(ui, cmd);
            });
        }
    }

    for b in [save, mi_save] {
        ui.on_click(b, |ui, sender| {
            select_of(ui, sender);
            crate::inspector::commit_pending(ui);
            let file = active(ui).map(|(f, _)| f);
            save_active(ui);
            if let (Some(h), Some(file)) = (hooks(), file) {
                (h.saved)(ui, &file);
            }
        });
    }
    ui.on_click(mi_close, |ui, sender| {
        select_of(ui, sender);
        close_active(ui);
    });
    // Fechar a janela de fora: as abas dela voltam para a IDE (nada se perde). Sem a IDE
    // por perto, com alteração não salva avisa uma vez; fechar de novo descarta.
    ui.on(form, "on_close_query", move |ui, _, _| {
        let host = HOST.with(|h| h.get()).filter(|&h| ui.contains(h));
        if host.is_some() {
            let count = with_editor_at(ui, form, |ed| ed.tabs.len()).unwrap_or(0);
            if count > 0 {
                let target = primary(ui);
                for _ in 0..count {
                    move_tab(ui, form, 0, target);
                }
            }
            return;
        }
        let warn = with_editor_at(ui, form, |ed| {
            let pending = ed.tabs.iter().any(|t| t.modified) && !ed.warned;
            ed.warned |= pending;
            pending
        });
        if warn == Some(true) {
            ui.cancel_close();
            let _ = ui.set_prop(status, "text", "Há alterações não salvas: Ctrl+S salva; feche de novo para descartar");
        }
    });
    ui.on(form, "on_close", move |_, _, _| EDITORS.with(|e| e.borrow_mut().retain(|ed| ed.form != form)));
    // Os botões extras do mouse, Alt+← e Alt+→: os lugares por onde o editor passou.
    ui.on(form, "on_navigate", |ui, _, args| {
        navigate(ui, args.text() == Some("forward"));
    });

    // A trilha (arquivo › tipo › função do cursor), em cima do editor; clicar mostra os
    // símbolos do arquivo (Ctrl+Shift+O).
    // A área dos editores (a trilha em cima e o editor da aba ativa).
    let area = ui.add(Layout::default(), form);
    ui.control_mut(area).unwrap().set_align(Align::Client);
    // O arquivo que a geração refaz abre somente leitura, com a faixa dizendo de onde ele vem.
    let generated_banner = ui.add(Label::new("Gerado pela IDE do Aylis a cada \"Gerar código\": somente leitura (mude o form ou o seu código, e gere de novo)"), area);
    ui.control_mut(generated_banner).unwrap().set_align(Align::Top).set_height(26.0).set_padding([8.0, 0.0, 8.0, 0.0]).set_visible(false);
    let _ = ui.set_prop(generated_banner, "style", "warning");
    let breadcrumb = ui.add(Label::new(""), area);
    ui.control_mut(breadcrumb).unwrap().set_align(Align::Top).set_height(24.0).set_margins([6.0, 2.0, 6.0, 0.0]).set_cursor(aylis_core::CursorShape::Pointer);
    let _ = ui.set_prop(breadcrumb, "font_size", 12);
    muted(ui, breadcrumb);
    let _ = ui.set_prop(breadcrumb, "hint", "Onde o cursor está; clique para ir para outro símbolo (Ctrl+Shift+O)");
    ui.on_click(breadcrumb, |ui, sender| {
        select_of(ui, sender);
        if let Some((file, _)) = active(ui) {
            lsp_event(ui, LspEvent::Symbols { file });
        }
    });

    EDITORS.with(|e| {
        e.borrow_mut().push(Editor {
            form,
            embedded,
            tab_bar,
            save,
            status,
            activity,
            panel,
            splitter,
            tabs: Vec::new(),
            active: 0,
            warned: false,
            find,
            bottom,
            bottom_splitter,
            bottom_title,
            bottom_list,
            problems,
            server,
            font_size: 0.0,
            minimap: true,
            color_theme: crate::actions::config_text("code_theme"),
            breadcrumb,
            generated_banner,
            area,
        })
    });
    CURRENT.with(|c| c.set(Some(form)));
    form
}

/// Uma aba nova com o arquivo (ainda não ativa). Devolve o índice.
fn add_tab(ui: &mut Ui, file: &Path) -> Result<usize, String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let (form, tab_bar) = with_editor(ui, |ed| (ed.form, ed.tab_bar)).ok_or("janela de código fechada")?;
    // O CodeEditor (o Memo de dentro tem o texto, o cursor e os pontos de parada).
    let mut editor = aylis_controls::CodeEditor::new(text).with_syntax(&syntax_of(file));
    // Código não termina linha em "…": a linha longa rola de lado (a barra horizontal).
    editor.memo.trimming = aylis_core::Trimming::None;
    editor.set_minimap(with_editor(ui, |ed| ed.minimap).unwrap_or(true));
    editor.set_color_theme(with_editor(ui, |ed| ed.color_theme.clone()).unwrap_or_default());
    if let Some(h) = hooks() {
        editor.memo.set_breakpoints(&(h.breakpoints)(file));
    }
    // Opções da IDE: o código digitado entra nas gravações de uso só mascarado.
    editor.memo.sensitive = crate::actions::config_flag("mask_editor_text", false);
    // O que a IDE refaz a cada geração: somente leitura (a faixa explica).
    if is_generated(file, &editor.memo.text) {
        editor.memo.read_only = true;
    }
    let opened_text = editor.memo.text.clone();
    let area = with_editor(ui, |ed| ed.area).unwrap_or(form);
    let memo = ui.add(editor, area);
    ui.control_mut(memo).unwrap().set_align(Align::Client).set_visible(false);
    attach_bookmarks(ui, memo, file);
    if let Some(size) = with_editor(ui, |ed| ed.font_size).filter(|s| *s > 0.0) {
        let _ = ui.set_prop(memo, "font_size", size);
    }
    let name = file_name(file);
    if let Some(strip) = ui.get_mut::<TabStrip>(tab_bar) {
        let i = strip.add(name, "file");
        if let Some(t) = strip.tab_mut(i) {
            t.hint = file.display().to_string();
        }
    }
    ui.request_repaint();

    // Clique na calha liga o ponto de parada da linha deste arquivo.
    if let Some(h) = hooks() {
        let file = file.to_path_buf();
        ui.on(memo, "on_gutter_click", move |ui, _, args| {
            select_of(ui, memo);
            if let aylis_core::EventArgs::Value(v) = args {
                if let Ok(line) = v.to_i64() {
                    (h.toggle)(ui, &file, line.max(1) as usize);
                }
            }
        });
    }
    {
        let file = file.to_path_buf();
        ui.on(memo, "on_change", move |ui, _, _| {
            select_of(ui, memo);
            crate::hovertip::hide_for(ui, memo);
            with_editor(ui, |ed| {
                if let Some(t) = ed.tabs.iter_mut().find(|t| t.memo == memo) {
                    t.modified = true;
                    t.warned = false;
                }
                ed.warned = false;
            });
            show_status(ui);
            let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
            lsp_event(ui, LspEvent::Changed { file: file.clone(), text });
            refresh_find(ui);
        });
    }
    {
        let file = file.to_path_buf();
        ui.on(memo, "on_complete_request", move |ui, _, args| {
            select_of(ui, memo);
            if let Some(index) = event_index(args) {
                lsp_event(ui, LspEvent::Complete { file: file.clone(), index });
            }
        });
    }
    {
        let file = file.to_path_buf();
        ui.on(memo, "on_signature_request", move |ui, _, args| {
            select_of(ui, memo);
            if let Some(index) = event_index(args) {
                lsp_event(ui, LspEvent::Signature { file: file.clone(), index });
            }
        });
    }
    {
        let file = file.to_path_buf();
        // A dica do servidor (o tipo, a documentação) com o mesmo atraso das dicas da tela: o
        // pedido fica agendado e sai se o mouse continuar na palavra (antes aparecia na hora;
        // relatado pelo autor em 07/11). Mudou de palavra ou saiu: o agendado é cancelado.
        thread_local! {
            static HOVER_PENDING: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
        }
        ui.on(memo, "on_hover_index", move |ui, _, args| {
            if let Some(pending) = HOVER_PENDING.with(|p| p.take()) {
                ui.clear_timeout(pending);
            }
            // Saiu da palavra: a dica aberta fecha daqui a pouco, a não ser que o mouse esteja
            // indo para ela (para rolar ou selecionar).
            crate::hovertip::hide_later(ui);
            if let Some(index) = event_index(args) {
                let file = file.clone();
                let delay = (ui.hint_delay() * 1000.0) as u64;
                let id = ui.set_timeout(delay, move |ui| {
                    HOVER_PENDING.with(|p| p.set(None));
                    select_of(ui, memo);
                    lsp_event(ui, LspEvent::Hover { file, index });
                });
                HOVER_PENDING.with(|p| p.set(Some(id)));
            }
        });
    }
    ui.on(memo, "on_caret_move", move |ui, _, _| {
        select_of(ui, memo);
        crate::hovertip::hide_for(ui, memo);
        show_status(ui);
    });
    // A dica do código fecha com uma tecla (o Esc também), um clique ou a roda no editor,
    // como no VS Code (só o Ctrl, o Shift e o Alt sozinhos não: o Ctrl+mouse mostra o link).
    ui.on(memo, "on_key_down", move |ui, _, args| {
        let modifier = matches!(args, aylis_core::EventArgs::Key(e) if matches!(e.key, aylis_core::Key::Control | aylis_core::Key::Shift | aylis_core::Key::Alt | aylis_core::Key::Meta));
        if !modifier {
            crate::hovertip::hide_for(ui, memo);
        }
    });
    for event in ["on_wheel", "on_pointer_down"] {
        ui.on(memo, event, move |ui, _, _| crate::hovertip::hide_for(ui, memo));
    }
    {
        // Ctrl+clique: a definição (pelo servidor; sem ele, o índice).
        let file = file.to_path_buf();
        ui.on(memo, "on_word_click", move |ui, _, args| {
            select_of(ui, memo);
            if let aylis_core::EventArgs::Text(word) = args {
                let word = word.clone();
                let index = ui.get::<Memo>(memo).map_or(0, |m| m.caret_index());
                lsp_event(ui, LspEvent::Definition { file: file.clone(), index, word });
            }
        });
    }
    lsp_event(ui, LspEvent::Opened { file: file.to_path_buf(), text: opened_text });
    let index = with_editor(ui, |ed| {
        ed.tabs.push(Tab { file: file.to_path_buf(), memo, modified: false, warned: false, disk: aylis_project::FileStamp::read(file) });
        ed.tabs.len() - 1
    })
    .ok_or("janela de código fechada")?;
    Ok(index)
}

fn file_name(file: &Path) -> String {
    file.file_name().and_then(|n| n.to_str()).unwrap_or("código").to_string()
}

/// Mostra a aba `index` (o editor dela) e esconde as outras.
fn activate(ui: &mut Ui, index: usize) {
    // A dica é da palavra de uma aba: trocar de aba a fecha.
    crate::hovertip::hide(ui);
    let Some((form, strip, current, tabs)) = with_editor(ui, |ed| {
        ed.active = index.min(ed.tabs.len().saturating_sub(1));
        (ed.form, ed.tab_bar, ed.active, ed.tabs.iter().enumerate().map(|(i, t)| (i == ed.active, t.memo, t.file.clone())).collect::<Vec<_>>())
    }) else {
        return;
    };
    if let Some(s) = ui.get_mut::<TabStrip>(strip) {
        s.tab_index = current as i64;
    }
    for (on, memo, file) in tabs {
        if let Some(c) = ui.control_mut(memo) {
            c.set_visible(on);
        }
        if on {
            let _ = ui.set_prop(form, "caption", format!("Código — {}", file_name(&file)));
            let banner = with_editor(ui, |ed| ed.generated_banner);
            let generated = ui.get::<Memo>(memo).is_some_and(|m| m.read_only) && std::fs::read_to_string(&file).is_ok_and(|t| is_generated(&file, &t));
            if let Some(c) = banner.and_then(|b| ui.control_mut(b)) {
                c.set_visible(generated);
            }
        }
    }
    show_status(ui);
    notify_tabs(ui);
    ui.request_repaint();
}

/// A situação da aba ativa: linha e coluna, e se há alteração não salva (também na aba).
fn show_status(ui: &mut Ui) {
    let Some((status, strip, tabs, active)) =
        with_editor(ui, |ed| (ed.status, ed.tab_bar, ed.tabs.iter().map(|t| (t.memo, t.modified)).collect::<Vec<_>>(), ed.active))
    else {
        return;
    };
    // O ponto de não salvo na aba (no lugar do ×).
    let marks_changed = ui.get::<TabStrip>(strip).is_some_and(|s| s.tabs().iter().zip(&tabs).any(|(t, (_, m))| t.modified != *m));
    if marks_changed {
        if let Some(s) = ui.get_mut::<TabStrip>(strip) {
            for (i, (_, modified)) in tabs.iter().enumerate() {
                s.set_modified(i, *modified);
            }
        }
        notify_tabs(ui);
    }
    for (i, (memo, modified)) in tabs.iter().enumerate() {
        if i == active {
            let (l, c) = ui.get::<Memo>(*memo).map_or((0, 0), |m| m.caret_position());
            let mark = if *modified { "● não salvo   " } else { "" };
            let _ = ui.set_prop(status, "text", format!("{mark}Ln {}, Col {}", l + 1, c + 1));
        }
    }
    update_breadcrumb(ui);
}

/// A trilha: o arquivo, o tipo (classe, struct, record...) e a função onde o cursor está,
/// pelas declarações do índice próprio (as de antes da linha do cursor).
fn update_breadcrumb(ui: &mut Ui) {
    let Some((file, memo)) = active(ui) else { return };
    let Some(label) = with_editor(ui, |ed| ed.breadcrumb) else { return };
    let Some((text, (line, _))) = ui.get::<Memo>(memo).map(|m| (m.text.clone(), m.caret_position())) else { return };
    let language = aylis_project::Language::ALL
        .into_iter()
        .find(|l| aylis_project::symbols::extensions(*l).iter().any(|e| file.extension().is_some_and(|x| x.eq_ignore_ascii_case(e))));
    let mut crumbs = vec![file_name(&file)];
    if let Some(language) = language {
        let lines: Vec<&str> = text.split('\n').collect();
        // A declaração vale se o bloco dela (pelo recuo, como no dobrar) vai até o cursor.
        let encloses = |d: &aylis_project::symbols::Declaration| {
            let start = d.line.saturating_sub(1);
            start == line || aylis_controls::block_end(&lines, start).is_some_and(|end| line <= end + 1)
        };
        let decls: Vec<_> = aylis_project::symbols::declarations(language, &text).into_iter().filter(|d| d.line <= line + 1 && encloses(d)).collect();
        let is_type = |k: &str| matches!(k, "class" | "struct" | "record" | "interface" | "trait" | "enum" | "type" | "object" | "namespace" | "union" | "impl");
        let ty = decls.iter().rev().find(|d| is_type(d.kind));
        let func = decls.iter().rev().find(|d| matches!(d.kind, "fn" | "const fn" | "function" | "procedure" | "func" | "def" | "method" | "constructor" | "destructor" | "operator"));
        if let Some(t) = ty {
            crumbs.push(t.name.clone());
        }
        if let Some(f) = func {
            crumbs.push(format!("{}()", f.name));
        }
    }
    let text = crumbs.join("  ›  ");
    if ui.get::<Label>(label).is_some_and(|l| l.text != text) {
        let _ = ui.set_prop(label, "text", text);
    }
}

/// Salva a aba ativa.
fn save_active(ui: &mut Ui) {
    if let Some((_, memo)) = active(ui) {
        save_memo(ui, memo);
    }
}

/// Salva a aba do editor `memo` (no editor atual).
fn save_memo(ui: &mut Ui, memo: ComponentId) {
    let Some(file) = with_editor(ui, |ed| ed.tabs.iter().find(|t| t.memo == memo).map(|t| t.file.clone())).flatten() else { return };
    let status = with_editor(ui, |ed| ed.status);
    let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
    match std::fs::write(&file, &text) {
        Ok(()) => {
            let stamp = aylis_project::FileStamp::read(&file);
            with_editor(ui, |ed| {
                if let Some(t) = ed.tabs.iter_mut().find(|t| t.memo == memo) {
                    t.modified = false;
                    t.disk = stamp;
                }
            });
            show_status(ui);
            // As linhas dos marcadores como ficaram no arquivo salvo.
            save_bookmarks_of(ui, memo);
            lsp_event(ui, LspEvent::Saved { file, text });
        }
        Err(e) => {
            if let Some(status) = status {
                let _ = ui.set_prop(status, "text", format!("⚠ não consegui salvar: {e}"));
            }
        }
    }
}

/// Fecha a aba ativa (com alteração: avisa uma vez); sem abas, fecha a janela.
fn close_active(ui: &mut Ui) {
    enum Next {
        Warn(ComponentId),
        Remove(ComponentId, usize, PathBuf),
    }
    let next = with_editor(ui, |ed| {
        let i = ed.active;
        let t = ed.tabs.get_mut(i)?;
        if t.modified && !t.warned {
            t.warned = true;
            return Some(Next::Warn(ed.status));
        }
        let t = ed.tabs.remove(i);
        Some(Next::Remove(t.memo, i, t.file))
    })
    .flatten();
    match next {
        Some(Next::Warn(status)) => {
            let _ = ui.set_prop(status, "text", "Há alterações não salvas: Ctrl+S salva; Ctrl+F4 de novo descarta");
        }
        Some(Next::Remove(memo, index, file)) => {
            ui.free(memo);
            let strip = with_editor(ui, |ed| ed.tab_bar);
            if let Some(s) = strip.and_then(|s| ui.get_mut::<TabStrip>(s)) {
                s.remove(index);
            }
            let left = with_editor(ui, |ed| ed.tabs.len()).unwrap_or(0);
            lsp_event(ui, LspEvent::Closed { file });
            notify_tabs(ui);
            if left == 0 {
                // O embutido sem abas volta ao designer; a janela de fora fecha.
                match with_editor(ui, |ed| (ed.form, ed.embedded)) {
                    Some((_, true)) => {
                        if let Some(h) = hooks() {
                            (h.view)(ui, false);
                        }
                    }
                    Some((form, false)) => {
                        ui.close(form);
                    }
                    None => {}
                }
            } else {
                let i = with_editor(ui, |ed| ed.active.min(left - 1)).unwrap_or(0);
                activate(ui, i);
            }
        }
        None => {}
    }
}

/// O caminho para mostrar: relativo à pasta do código, com `/`.
fn relative(root: Option<&Path>, file: &Path) -> String {
    let rel = root.and_then(|r| file.strip_prefix(r).ok()).unwrap_or(file);
    rel.to_string_lossy().replace('\\', "/")
}

/// Abrir fonte (Ctrl+F12): a lista dos fontes do projeto com um
/// filtro; OK (ou Enter) abre na janela de código.
pub fn open_source_dialog(ui: &mut Ui) {
    use crate::dialogs::{edit, field, new_dialog, run, text_of};
    let Some(h) = hooks() else { return };
    let (root, files) = (h.sources)();
    let d = new_dialog(ui, "Abrir fonte", 560.0, 520.0);
    let line = field(ui, d.body, "Filtro");
    let filter = edit(ui, line, "");
    let list = ui.add(ListBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client).set_margins([0.0, 8.0, 0.0, 0.0]);
    let all: Vec<(String, PathBuf)> = files.iter().map(|f| (relative(root.as_deref(), f), f.clone())).collect();
    let shown: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
    let fill = {
        let shown = shown.clone();
        move |ui: &mut Ui| {
            let query = text_of(ui, filter).to_lowercase();
            let rows: Vec<&(String, PathBuf)> = all.iter().filter(|(rel, _)| rel.to_lowercase().contains(&query)).collect();
            *shown.borrow_mut() = rows.iter().map(|(_, f)| f.clone()).collect();
            let _ = ui.set_prop(list, "items", rows.iter().map(|(rel, _)| rel.as_str()).collect::<Vec<_>>().join(";"));
            let _ = ui.set_prop(list, "item_index", if rows.is_empty() { -1i64 } else { 0 });
        }
    };
    fill(ui);
    ui.on(filter, "on_change", move |ui, _, _| fill(ui));
    let pick = {
        let shown = shown.clone();
        move |ui: &Ui| {
            let i = ui.get_prop(list, "item_index").ok().and_then(|v| v.to_i64().ok()).unwrap_or(-1);
            usize::try_from(i).ok().and_then(|i| shown.borrow().get(i).cloned())
        }
    };
    let pick2 = pick.clone();
    run(
        ui,
        d,
        move |ui| if pick(ui).is_some() { Ok(()) } else { Err("escolha um fonte".into()) },
        move |ui| {
            if let Some(file) = pick2(ui) {
                let _ = open_here(ui, &file, 0);
            }
        },
    );
    ui.set_focus(filter);
}

/// A caixa da condição do ponto de parada.
fn condition_dialog(ui: &mut Ui, h: Rc<Hooks>, file: PathBuf, line: usize) {
    use aylis_controls::Edit;
    let (cond, hits) = (h.condition)(&file, line);
    let form = ui.add_root(Form::new(format!("Ponto de parada — linha {line}"), 420.0, 230.0));
    let _ = ui.set_prop(form, "border_style", aylis_core::PropValue::Enum("single".into()));
    ui.control_mut(form).unwrap().set_padding(14.0);
    let add_label = |ui: &mut Ui, text: &str| {
        let l = ui.add(Label::new(text), form);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(22.0);
        muted(ui, l);
    };
    add_label(ui, "Condição (só para quando for verdadeira; ex.: i == 3)");
    let cond_edit = ui.add(Edit::new(cond), form);
    ui.control_mut(cond_edit).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    add_label(ui, "Passagens (5 = na quinta; >= 5 = da quinta em diante)");
    let hits_edit = ui.add(Edit::new(hits), form);
    ui.control_mut(hits_edit).unwrap().set_align(Align::Top).set_height(32.0);
    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(34.0);
    let cancel = ui.add(Button::new("Cancelar"), bar);
    ui.control_mut(cancel).unwrap().set_align(Align::Right).set_width(100.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(cancel, "modal_result", aylis_core::PropValue::Enum("cancel".into()));
    let ok = ui.add(Button::new("OK"), bar);
    ui.control_mut(ok).unwrap().set_align(Align::Right).set_width(100.0);
    let _ = ui.set_prop(ok, "modal_result", aylis_core::PropValue::Enum("ok".into()));
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, result| {
        if result == aylis_core::ModalResult::Ok {
            let text = |id| ui.get::<Edit>(id).map(|e| e.text.clone()).unwrap_or_default();
            let (c, n) = (text(cond_edit), text(hits_edit));
            (h.set_condition)(ui, &file, line, &c, &n);
        }
    });
    ui.set_focus(cond_edit);
}

/// Um FlowLayout para os botões do painel de depuração (quebram linha se não couberem).
pub fn flow(ui: &mut Ui, parent: ComponentId) -> ComponentId {
    let f = ui.add(FlowLayout::default(), parent);
    ui.control_mut(f).unwrap().set_align(Align::Top).set_height(76.0);
    f
}

// ---------------------------------------------------------------------------
// Os menus (o que o VS Code tem), a paleta de comandos e os atalhos

/// O índice que veio num evento do editor (`-1`: nenhum).
fn event_index(args: &aylis_core::EventArgs) -> Option<usize> {
    match args {
        aylis_core::EventArgs::Value(v) => v.to_i64().ok().filter(|i| *i >= 0).map(|i| i as usize),
        _ => None,
    }
}

/// Mexe no editor da aba ativa (com a hora, para o desfazer) e, se o texto mudou, avisa
/// como se tivesse digitado (o servidor, o "não salvo", o localizar).
fn with_active_memo(ui: &mut Ui, f: impl FnOnce(&mut Memo, f64) -> bool) {
    let Some((_, memo)) = active(ui) else { return };
    let now = ui.time();
    let before = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
    let changed = ui.get_mut::<Memo>(memo).is_some_and(|m| f(m, now));
    let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
    if changed && text != before {
        ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.into()));
    }
    ui.set_focus(memo);
    ui.request_repaint();
}

/// O arquivo da aba ativa, o índice do cursor e a palavra sob ele.
fn caret_word(ui: &Ui) -> Option<(PathBuf, usize, String)> {
    let (file, memo) = active(ui)?;
    let m = ui.get::<Memo>(memo)?;
    Some((file, m.caret_index(), m.word_at_caret()))
}

fn add_menu_item(ui: &mut Ui, parent: ComponentId, text: &str, shortcut: &str, action: impl Fn(&mut Ui) + 'static) -> ComponentId {
    let item = ui.add(MenuItem::new(text).with_shortcut(shortcut), parent);
    ui.on_click(item, move |ui, sender| {
        select_of(ui, sender);
        action(ui);
    });
    item
}

fn menu_separator(ui: &mut Ui, parent: ComponentId) {
    ui.add(MenuItem::new("-"), parent);
}

thread_local! {
    /// Um item de Recortar/Copiar/Colar mandando a tecla ao editor: a tecla que o editor não
    /// tratar (Colar num editor só de leitura) cai nos atalhos e voltaria ao mesmo item.
    static SENDING_KEY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Ctrl+`key` no editor da aba ativa, pelo mesmo caminho do teclado (os vários cursores, o
/// desfazer, o `on_change` e a área de transferência do Aylis).
fn send_ctrl_key(ui: &mut Ui, key: char) {
    let Some((_, memo)) = active(ui) else { return };
    if SENDING_KEY.with(|s| s.replace(true)) {
        return;
    }
    ui.set_focus(memo);
    let modifiers = aylis_core::Modifiers { ctrl: true, ..Default::default() };
    let root = ui.root_of(memo);
    ui.key_down(root, aylis_core::KeyEvent { key: aylis_core::Key::Char(key), modifiers, repeat: false });
    SENDING_KEY.with(|s| s.set(false));
    ui.request_repaint();
}

/// Recortar, Copiar, Colar e Selecionar tudo (os clássicos do menu do botão direito).
fn clipboard_items(ui: &mut Ui, parent: ComponentId) {
    add_menu_item(ui, parent, "Recortar", "Ctrl+X", |ui| send_ctrl_key(ui, 'x'));
    add_menu_item(ui, parent, "Copiar", "Ctrl+C", |ui| send_ctrl_key(ui, 'c'));
    add_menu_item(ui, parent, "Colar", "Ctrl+V", |ui| send_ctrl_key(ui, 'v'));
    menu_separator(ui, parent);
    add_menu_item(ui, parent, "Selecionar tudo", "Ctrl+A", |ui| send_ctrl_key(ui, 'a'));
}

/// Editar, Navegar e Exibir.
fn create_menus(ui: &mut Ui, menu: ComponentId, embedded: bool) {
    let edit = ui.add(MenuItem::new("Editar"), menu);
    add_menu_item(ui, edit, "Desfazer", "Ctrl+Z", |ui| with_active_memo(ui, |m, _| m.undo()));
    add_menu_item(ui, edit, "Refazer", "Ctrl+Y", |ui| with_active_memo(ui, |m, _| m.redo()));
    menu_separator(ui, edit);
    // No menu do botão direito eles já estão no topo.
    if !embedded {
        clipboard_items(ui, edit);
        menu_separator(ui, edit);
    }
    add_menu_item(ui, edit, "Localizar", "Ctrl+F", |ui| show_find(ui, false));
    add_menu_item(ui, edit, "Substituir", "Ctrl+H", |ui| show_find(ui, true));
    add_menu_item(ui, edit, "Pesquisar no projeto…", "Ctrl+Shift+F", |ui| {
        let selected = active(ui).and_then(|(_, memo)| ui.get::<Memo>(memo).map(|m| m.sel_text())).filter(|s| !s.contains('\n')).unwrap_or_default();
        if let Some(h) = hooks() {
            (h.search)(ui, &selected);
        }
    });
    add_menu_item(ui, edit, "Localizar próximo", "F3", |ui| find_step(ui, true));
    add_menu_item(ui, edit, "Localizar anterior", "Shift+F3", |ui| find_step(ui, false));
    add_menu_item(ui, edit, "Ir para a linha…", "Ctrl+G", go_to_line_dialog);
    menu_separator(ui, edit);
    add_menu_item(ui, edit, "Comentar ou descomentar", "Ctrl+/", |ui| with_active_memo(ui, |m, now| m.toggle_comment(now)));
    add_menu_item(ui, edit, "Mover a linha para cima", "Alt+Up", |ui| with_active_memo(ui, |m, now| m.move_lines(true, now)));
    add_menu_item(ui, edit, "Mover a linha para baixo", "Alt+Down", |ui| with_active_memo(ui, |m, now| m.move_lines(false, now)));
    add_menu_item(ui, edit, "Copiar a linha para cima", "Shift+Alt+Up", |ui| with_active_memo(ui, |m, now| m.duplicate_lines(false, now)));
    add_menu_item(ui, edit, "Copiar a linha para baixo", "Shift+Alt+Down", |ui| with_active_memo(ui, |m, now| m.duplicate_lines(true, now)));
    add_menu_item(ui, edit, "Apagar a linha", "Ctrl+Shift+K", |ui| with_active_memo(ui, |m, now| m.delete_lines(now)));
    add_menu_item(ui, edit, "Linha nova abaixo", "Ctrl+Enter", |ui| with_active_memo(ui, |m, now| m.insert_line(true, now)));
    add_menu_item(ui, edit, "Aumentar o recuo", "Ctrl+]", |ui| with_active_memo(ui, |m, now| m.indent_lines(false, now)));
    add_menu_item(ui, edit, "Diminuir o recuo", "Ctrl+[", |ui| with_active_memo(ui, |m, now| m.indent_lines(true, now)));
    menu_separator(ui, edit);
    add_menu_item(ui, edit, "Selecionar a linha", "Ctrl+L", |ui| with_active_memo(ui, |m, _| {
        m.select_line();
        false
    }));
    add_menu_item(ui, edit, "Selecionar a próxima ocorrência", "Ctrl+D", |ui| with_active_memo(ui, |m, _| {
        m.add_next_occurrence();
        false
    }));
    add_menu_item(ui, edit, "Selecionar todas as ocorrências", "Ctrl+Shift+L", |ui| with_active_memo(ui, |m, _| {
        m.select_all_occurrences();
        false
    }));
    add_menu_item(ui, edit, "Adicionar cursor acima", "Ctrl+Alt+Up", |ui| with_active_memo(ui, |m, _| {
        m.add_cursor_vertical(false);
        false
    }));
    add_menu_item(ui, edit, "Adicionar cursor abaixo", "Ctrl+Alt+Down", |ui| with_active_memo(ui, |m, _| {
        m.add_cursor_vertical(true);
        false
    }));
    add_menu_item(ui, edit, "Ir para o par do colchete", "Ctrl+Shift+\\", |ui| with_active_memo(ui, |m, _| {
        m.jump_to_bracket();
        false
    }));
    menu_separator(ui, edit);
    add_menu_item(ui, edit, "Formatar o documento", "Shift+Alt+F", |ui| {
        if let Some((file, _)) = active(ui) {
            lsp_event(ui, LspEvent::Format { file });
        }
    });

    let nav = ui.add(MenuItem::new("Navegar"), menu);
    add_menu_item(ui, nav, "Abrir fonte…", "Ctrl+F12", open_source_dialog);
    add_menu_item(ui, nav, "Ir para o arquivo…", "Ctrl+P", open_source_dialog);
    add_menu_item(ui, nav, "Ir para o símbolo…", "Ctrl+Shift+O", |ui| {
        if let Some((file, _)) = active(ui) {
            lsp_event(ui, LspEvent::Symbols { file });
        }
    });
    add_menu_item(ui, nav, "Símbolo no projeto…", "Ctrl+T", project_symbols_dialog);
    menu_separator(ui, nav);
    // Embutido na IDE, o F12 volta ao form e a definição fica no Alt+F12 (e
    // no Ctrl+clique).
    if embedded {
        add_menu_item(ui, nav, "Ir para o Design", "F12", |ui| {
            if let Some(h) = hooks() {
                (h.view)(ui, false);
            }
        });
    }
    add_menu_item(ui, nav, "Ir para a definição", if embedded { "Alt+F12" } else { "F12" }, |ui| {
        if let Some((file, index, word)) = caret_word(ui) {
            lsp_event(ui, LspEvent::Definition { file, index, word });
        }
    });
    add_menu_item(ui, nav, "Ir para as referências", "Shift+F12", |ui| {
        if let Some((file, index, word)) = caret_word(ui) {
            lsp_event(ui, LspEvent::References { file, index, word });
        }
    });
    add_menu_item(ui, nav, "Renomear o símbolo…", "F2", rename_dialog);
    add_menu_item(ui, nav, "Correção rápida…", "Ctrl+.", |ui| {
        if let Some((file, index, _)) = caret_word(ui) {
            lsp_event(ui, LspEvent::QuickFix { file, index });
        }
    });
    menu_separator(ui, nav);
    add_menu_item(ui, nav, "Sugestões", "Ctrl+Space", |ui| {
        if let Some((file, index, _)) = caret_word(ui) {
            lsp_event(ui, LspEvent::Complete { file, index });
        }
    });
    add_menu_item(ui, nav, "Ajuda dos parâmetros", "Ctrl+Shift+Space", |ui| {
        if let Some((file, index, _)) = caret_word(ui) {
            lsp_event(ui, LspEvent::Signature { file, index });
        }
    });

    let view = ui.add(MenuItem::new("Exibir"), menu);
    add_menu_item(ui, view, "Paleta de comandos…", "Ctrl+Shift+P", command_palette);
    add_menu_item(ui, view, "Problemas", "Ctrl+Shift+M", |ui| toggle_bottom(ui, Bottom::Problems));
    add_menu_item(ui, view, "Minimapa", "", toggle_minimap);
    add_menu_item(ui, view, "Tema do editor…", "", choose_color_theme);
    add_menu_item(ui, view, "Importar tema…", "", |ui| crate::grammars::import_dialog(ui, true));
    add_menu_item(ui, view, "Importar linguagem…", "", |ui| crate::grammars::import_dialog(ui, false));
    menu_separator(ui, view);
    add_menu_item(ui, view, "Dobrar", "Ctrl+Shift+[", |ui| with_active_memo(ui, |m, _| {
        m.fold_at_caret();
        false
    }));
    add_menu_item(ui, view, "Desdobrar", "Ctrl+Shift+]", |ui| with_active_memo(ui, |m, _| {
        m.unfold_at_caret();
        false
    }));
    add_menu_item(ui, view, "Dobrar tudo", "Ctrl+Shift+-", |ui| with_active_memo(ui, |m, _| {
        m.fold_all();
        false
    }));
    add_menu_item(ui, view, "Desdobrar tudo", "Ctrl+Shift+=", |ui| with_active_memo(ui, |m, _| {
        m.unfold_all();
        false
    }));
    // Até o nível N: 1 deixa à vista só as funções (e as regions) do arquivo, 2 os blocos de
    // dentro delas...
    for level in 1..=3 {
        add_menu_item(ui, view, &format!("Dobrar o nível {level}"), "", move |ui| with_active_memo(ui, |m, _| {
            m.fold_level(level);
            false
        }));
    }
    menu_separator(ui, view);
    add_menu_item(ui, view, "Aumentar a fonte", "Ctrl+=", |ui| zoom(ui, 1.0));
    add_menu_item(ui, view, "Diminuir a fonte", "Ctrl+-", |ui| zoom(ui, -1.0));
    // (Ctrl+0 é o marcador 0.)
    add_menu_item(ui, view, "Fonte padrão", "Ctrl+Alt+0", |ui| zoom(ui, 0.0));
    menu_separator(ui, view);
    add_menu_item(ui, view, "Marcadores…", "Ctrl+Alt+B", bookmarks_list);
    add_menu_item(ui, view, "Marcar a linha", "Ctrl+Alt+K", |ui| {
        let memo = active(ui).map(|(_, m)| m);
        with_active_memo(ui, |m, _| {
            m.toggle_bookmark();
            false
        });
        if let Some(memo) = memo {
            ui.fire(memo, "on_bookmarks_change", &aylis_core::EventArgs::None);
        }
    });
    add_menu_item(ui, view, "Marcar com o próximo número", "Ctrl+B", |ui| {
        let memo = active(ui).map(|(_, m)| m);
        with_active_memo(ui, |m, _| {
            m.toggle_next_bookmark();
            false
        });
        if let Some(memo) = memo {
            ui.fire(memo, "on_bookmarks_change", &aylis_core::EventArgs::None);
        }
    });
    add_menu_item(ui, view, "Nome do marcador…", "", rename_bookmark);
}

/// Exibir → Tema do editor: as cores do código (como os temas do VS Code), em todos os
/// editores; a escolha fica guardada.
fn choose_color_theme(ui: &mut Ui) {
    let mut names = vec!["Automático (as cores da IDE)".to_string()];
    let themes = aylis_controls::syntax::code_themes();
    names.extend(themes.iter().map(|t| format!("{}   — {}", t.name, if t.dark { "escuro" } else { "claro" })));
    quick_pick(ui, "Tema do editor", names, move |ui, i| {
        let name = if i == 0 { String::new() } else { themes[i - 1].name.to_string() };
        set_color_theme(ui, &name);
    });
}

/// Troca o tema de cores do código em todos os editores e guarda.
pub fn set_color_theme(ui: &mut Ui, name: &str) {
    let Some(memos) = with_editor(ui, |ed| {
        ed.color_theme = name.to_string();
        ed.tabs.iter().map(|t| t.memo).collect::<Vec<_>>()
    }) else {
        return;
    };
    for memo in memos {
        let _ = ui.set_prop(memo, "color_theme", name);
    }
    crate::actions::update_config("code_theme", serde_json::Value::from(name));
    set_status(ui, &if name.is_empty() { "Tema do editor: as cores da IDE".to_string() } else { format!("Tema do editor: {name}") });
    ui.request_repaint();
}

/// Exibir → Minimapa: liga ou desliga em todos os editores.
fn toggle_minimap(ui: &mut Ui) {
    let Some((on, memos)) = with_editor(ui, |ed| {
        ed.minimap = !ed.minimap;
        (ed.minimap, ed.tabs.iter().map(|t| t.memo).collect::<Vec<_>>())
    }) else {
        return;
    };
    for memo in memos {
        let _ = ui.set_prop(memo, "minimap", on);
    }
    set_status(ui, if on { "Minimapa ligado" } else { "Minimapa desligado" });
}

/// Ctrl+= e Ctrl+-: o tamanho da fonte de todos os editores (0: o padrão).
fn zoom(ui: &mut Ui, step: f32) {
    let Some((size, memos)) = with_editor(ui, |ed| {
        let base = if ed.font_size > 0.0 { ed.font_size } else { 14.0 };
        ed.font_size = if step == 0.0 { 0.0 } else { (base + step).clamp(8.0, 40.0) };
        (ed.font_size, ed.tabs.iter().map(|t| t.memo).collect::<Vec<_>>())
    }) else {
        return;
    };
    for memo in memos {
        let value = if size > 0.0 { aylis_core::PropValue::Float(f64::from(size)) } else { aylis_core::PropValue::Null };
        let _ = ui.set_prop(memo, "font_size", value);
    }
    set_status(ui, &if size > 0.0 { format!("Fonte do código: {size}") } else { "Fonte do código: a padrão".to_string() });
}

/// A paleta de comandos (Ctrl+Shift+P): todos os itens dos menus da janela, com o atalho.
fn command_palette(ui: &mut Ui) {
    let Some(form) = with_editor(ui, |ed| ed.form) else { return };
    let Some(menu) = ui.children(form).iter().copied().find(|&c| ui.get::<MainMenu>(c).is_some() || ui.get::<PopupMenu>(c).is_some()) else { return };
    let mut items: Vec<(String, ComponentId)> = Vec::new();
    for &top in ui.children(menu) {
        let group = ui.get::<MenuItem>(top).map(|m| m.text.clone()).unwrap_or_default();
        for &item in ui.children(top) {
            let Some(m) = ui.get::<MenuItem>(item) else { continue };
            if m.text == "-" || !ui.children(item).is_empty() {
                continue;
            }
            let shortcut = if m.shortcut.is_empty() { String::new() } else { format!("   ({})", m.shortcut) };
            items.push((format!("{}: {}{shortcut}", ui.tr(&group), ui.tr(&m.text)), item));
        }
    }
    let titles = items.iter().map(|(t, _)| t.clone()).collect();
    quick_pick(ui, "Paleta de comandos", titles, move |ui, i| {
        ui.click(items[i].1);
    });
}

// ---------------------------------------------------------------------------
// Localizar e substituir

fn create_find_bar(ui: &mut Ui, form: ComponentId) -> Find {
    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(36.0).set_padding([6.0, 2.0, 6.0, 2.0]).set_visible(false);
    let close = ui.add(Button::new("×"), bar);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(30.0).set_style("flat");
    ui.on_click(close, |ui, sender| {
        select_of(ui, sender);
        hide_find(ui);
    });
    let next = ui.add(Button::new("↓"), bar);
    ui.control_mut(next).unwrap().set_align(Align::Right).set_width(34.0).set_style("flat");
    let _ = ui.set_prop(next, "hint", "Próximo (Enter, F3)");
    ui.on_click(next, |ui, sender| {
        select_of(ui, sender);
        find_step(ui, true);
    });
    let prev = ui.add(Button::new("↑"), bar);
    ui.control_mut(prev).unwrap().set_align(Align::Right).set_width(34.0).set_style("flat");
    let _ = ui.set_prop(prev, "hint", "Anterior (Shift+Enter, Shift+F3)");
    ui.on_click(prev, |ui, sender| {
        select_of(ui, sender);
        find_step(ui, false);
    });
    let count = ui.add(Label::new(""), bar);
    ui.control_mut(count).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    muted(ui, count);
    let whole = ui.add(CheckBox::new("Palavra inteira"), bar);
    ui.control_mut(whole).unwrap().set_align(Align::Right).set_width(130.0);
    let in_selection = ui.add(CheckBox::new("Na seleção"), bar);
    ui.control_mut(in_selection).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(in_selection, "hint", "Só no trecho selecionado ao marcar");
    let regex = ui.add(CheckBox::new(".*"), bar);
    ui.control_mut(regex).unwrap().set_align(Align::Right).set_width(52.0);
    let _ = ui.set_prop(regex, "hint", "Expressão regular (na substituição, $1 é o primeiro grupo)");
    let case = ui.add(CheckBox::new("Aa"), bar);
    ui.control_mut(case).unwrap().set_align(Align::Right).set_width(56.0);
    let _ = ui.set_prop(case, "hint", "Diferenciar maiúsculas");
    let query = ui.add(Edit::new(""), bar);
    ui.control_mut(query).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(query, "placeholder", "Localizar");

    let replace_row = ui.add(Layout::default(), form);
    ui.control_mut(replace_row).unwrap().set_align(Align::Top).set_height(36.0).set_padding([6.0, 2.0, 6.0, 2.0]).set_visible(false);
    let all = ui.add(Button::new("Substituir todos"), replace_row);
    ui.control_mut(all).unwrap().set_align(Align::Right).set_width(140.0).set_style("flat");
    ui.on_click(all, |ui, sender| {
        select_of(ui, sender);
        replace_all(ui);
    });
    let one = ui.add(Button::new("Substituir"), replace_row);
    ui.control_mut(one).unwrap().set_align(Align::Right).set_width(110.0).set_style("flat");
    ui.on_click(one, |ui, sender| {
        select_of(ui, sender);
        replace_one(ui);
    });
    let keep_case = ui.add(CheckBox::new("AB"), replace_row);
    ui.control_mut(keep_case).unwrap().set_align(Align::Right).set_width(56.0);
    let _ = ui.set_prop(keep_case, "hint", "Preservar maiúsculas (\"Nome\" vira \"Valor\", \"NOME\" vira \"VALOR\")");
    let replace = ui.add(Edit::new(""), replace_row);
    ui.control_mut(replace).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(replace, "placeholder", "Substituir por");

    for c in [query, case, whole, regex, in_selection] {
        ui.on(c, "on_change", move |ui, sender, _| {
            select_of(ui, sender);
            if c == in_selection {
                remember_find_scope(ui);
            }
            refresh_find(ui);
            find_from_caret(ui);
        });
    }
    // Enter: próximo; Shift+Enter: anterior; Esc fecha.
    for edit in [query, replace] {
        ui.on(edit, "on_key_down", move |ui, sender, args| {
            select_of(ui, sender);
            let aylis_core::EventArgs::Key(k) = args else { return };
            match k.key {
                aylis_core::Key::Enter if edit == replace => replace_one(ui),
                aylis_core::Key::Enter => find_step(ui, !k.modifiers.shift),
                aylis_core::Key::Escape => hide_find(ui),
                _ => {}
            }
        });
    }
    Find { bar, replace_row, query, replace, case, whole, regex, in_selection, keep_case, count }
}

thread_local! {
    /// O trecho do "Na seleção" (em caracteres), guardado ao marcar a caixa: a seleção muda ao
    /// achar, o trecho não.
    static FIND_SCOPE: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) };
}

/// Marcou ou desmarcou o "Na seleção": guarda a seleção de agora como o trecho.
fn remember_find_scope(ui: &mut Ui) {
    let Some(f) = with_editor(ui, |ed| ed.find) else { return };
    let on = ui.get_prop(f.in_selection, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false);
    let scope = active(ui).and_then(|(_, memo)| ui.get::<Memo>(memo).map(|m| m.selection())).filter(|(a, b)| a != b);
    FIND_SCOPE.with(|s| s.set(if on { scope } else { None }));
}

/// O que procurar (com as opções) e o trecho, se for só na seleção.
fn find_options(ui: &Ui) -> Option<(aylis_controls::search::SearchOptions, Option<(usize, usize)>)> {
    let f = with_editor(ui, |ed| ed.find)?;
    let visible = ui.control(f.bar).is_some_and(|c| c.visible());
    let checked = |c: ComponentId| ui.get_prop(c, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false);
    let query = ui.get::<Edit>(f.query).map(|e| e.text.clone()).unwrap_or_default();
    let options = aylis_controls::search::SearchOptions { query, case: checked(f.case), whole: checked(f.whole), regex: checked(f.regex), preserve_case: checked(f.keep_case) };
    let scope = FIND_SCOPE.with(|s| s.get()).filter(|_| checked(f.in_selection));
    (visible && !options.query.is_empty()).then_some((options, scope))
}

/// Ctrl+F (e Ctrl+H com a linha do substituir): a seleção vira o procurado.
fn show_find(ui: &mut Ui, replace: bool) {
    let Some(f) = with_editor(ui, |ed| ed.find) else { return };
    if let Some((_, memo)) = active(ui) {
        let selected = ui.get::<Memo>(memo).map(|m| m.sel_text()).unwrap_or_default();
        if !selected.is_empty() && !selected.contains('\n') {
            let _ = ui.set_prop(f.query, "text", selected);
        }
    }
    if let Some(c) = ui.control_mut(f.bar) {
        c.set_visible(true);
    }
    if let Some(c) = ui.control_mut(f.replace_row) {
        c.set_visible(replace);
    }
    ui.set_focus(f.query);
    refresh_find(ui);
    ui.request_repaint();
}

fn hide_find(ui: &mut Ui) {
    let Some(f) = with_editor(ui, |ed| ed.find) else { return };
    for id in [f.bar, f.replace_row] {
        if let Some(c) = ui.control_mut(id) {
            c.set_visible(false);
        }
    }
    let memos: Vec<ComponentId> = with_editor(ui, |ed| ed.tabs.iter().map(|t| t.memo).collect()).unwrap_or_default();
    for memo in memos {
        if let Some(m) = ui.get_mut::<Memo>(memo) {
            m.set_highlights(Vec::new());
        }
    }
    if let Some((_, memo)) = active(ui) {
        ui.set_focus(memo);
    }
    ui.request_repaint();
}

/// Realça as ocorrências e mostra quantas são ("3 de 12").
fn refresh_find(ui: &mut Ui) {
    let Some(f) = with_editor(ui, |ed| ed.find) else { return };
    let Some((_, memo)) = active(ui) else { return };
    let options = find_options(ui);
    let mut error = None;
    let (all, sel) = match (&options, ui.get::<Memo>(memo)) {
        (Some((o, scope)), Some(m)) => (m.find_with(o, *scope).unwrap_or_else(|e| { error = Some(e); Vec::new() }), m.selection()),
        _ => (Vec::new(), (0, 0)),
    };
    let text = match (&options, all.iter().position(|&r| r == sel)) {
        (None, _) => String::new(),
        (Some(_), _) if error.is_some() => format!("⚠ {}", error.unwrap_or_default()),
        (Some(_), _) if all.is_empty() => "Nenhum".to_string(),
        (Some(_), Some(i)) => format!("{} de {}", i + 1, all.len()),
        (Some(_), None) => format!("{} resultados", all.len()),
    };
    let _ = ui.set_prop(f.count, "text", text);
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        m.set_highlights(all);
    }
    ui.request_repaint();
}

/// Digitando o procurado: vai para a primeira ocorrência a partir do cursor.
fn find_from_caret(ui: &mut Ui) {
    let (Some((o, scope)), Some((_, memo))) = (find_options(ui), active(ui)) else { return };
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        let (a, _) = m.selection();
        m.select(a, a);
        m.find_next_with(&o, scope, true);
    }
    refresh_find(ui);
}

/// F3 / Shift+F3 (e Enter no campo): a próxima ou a anterior.
fn find_step(ui: &mut Ui, forward: bool) {
    let Some((o, scope)) = find_options(ui) else {
        show_find(ui, false);
        return;
    };
    let Some((_, memo)) = active(ui) else { return };
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        m.find_next_with(&o, scope, forward);
    }
    refresh_find(ui);
}

fn replace_text(ui: &Ui) -> String {
    with_editor(ui, |ed| ed.find.replace).and_then(|r| ui.get::<Edit>(r).map(|e| e.text.clone())).unwrap_or_default()
}

fn replace_one(ui: &mut Ui) {
    let Some((o, scope)) = find_options(ui) else { return };
    let with = replace_text(ui);
    with_active_memo(ui, |m, now| m.replace_with(&o, scope, &with, now));
    refresh_find(ui);
}

fn replace_all(ui: &mut Ui) {
    let Some((o, scope)) = find_options(ui) else { return };
    let with = replace_text(ui);
    let mut n = 0;
    with_active_memo(ui, |m, now| {
        n = m.replace_all_with(&o, scope, &with, now);
        n > 0
    });
    refresh_find(ui);
    set_status(ui, &format!("{n} substituições"));
}

// ---------------------------------------------------------------------------
// O painel de baixo: Problemas e Referências

#[derive(Clone, Copy, PartialEq, Eq)]
enum Bottom {
    Problems,
    References,
}

thread_local! {
    /// O que o painel de baixo mostra agora (e os problemas, para mostrar de novo).
    static BOTTOM: RefCell<(Option<Bottom>, Vec<TreeNode>, String)> = const { RefCell::new((None, Vec::new(), String::new())) };
}

/// Mostra o painel com `what` (ou esconde, com `None`).
fn show_bottom(ui: &mut Ui, what: Option<Bottom>) {
    let Some((bottom, splitter, title, list)) = with_editor(ui, |ed| (ed.bottom, ed.bottom_splitter, ed.bottom_title, ed.bottom_list)) else { return };
    for id in [bottom, splitter] {
        if let Some(c) = ui.control_mut(id) {
            c.set_visible(what.is_some());
        }
    }
    BOTTOM.with(|b| b.borrow_mut().0 = what);
    if what == Some(Bottom::Problems) {
        let (nodes, text) = BOTTOM.with(|b| (b.borrow().1.clone(), b.borrow().2.clone()));
        let _ = ui.set_prop(title, "text", text);
        if let Some(t) = ui.get_mut::<TreeView>(list) {
            t.set_nodes(nodes);
        }
    }
    ui.request_repaint();
}

fn toggle_bottom(ui: &mut Ui, what: Bottom) {
    let now = BOTTOM.with(|b| b.borrow().0);
    show_bottom(ui, if now == Some(what) { None } else { Some(what) });
}

/// Um nó "arquivo|linha|coluna" (o clique vai para lá).
fn place_node(text: String, file: &Path, (line, col): Pos) -> TreeNode {
    let mut n = TreeNode::new(text);
    n.data = format!("{}|{line}|{col}", file.display());
    n
}

/// Os problemas de todos os arquivos (o painel e o contador da barra de cima).
pub fn show_problems(ui: &mut Ui, all: &std::collections::BTreeMap<PathBuf, Vec<aylis_tools::lsp::Diag>>) {
    let (mut errors, mut warnings) = (0, 0);
    let mut nodes = Vec::new();
    for (file, list) in all {
        let mut children = Vec::new();
        for d in list {
            match d.severity {
                1 => errors += 1,
                2 => warnings += 1,
                _ => {}
            }
            let icon = match d.severity {
                1 => "⊗",
                2 => "⚠",
                _ => "ℹ",
            };
            let source = if d.source.is_empty() { String::new() } else { format!("  [{}]", d.source) };
            children.push(place_node(format!("{icon} {}  (linha {}){source}", d.message.lines().next().unwrap_or(""), d.start.0 + 1), file, d.start));
        }
        if !children.is_empty() {
            let mut n = TreeNode::new(format!("{} ({})", file_name(file), children.len())).with_children(children);
            n.expanded = true;
            nodes.push(n);
        }
    }
    let title = format!("Problemas: {errors} erro(s), {warnings} aviso(s)");
    BOTTOM.with(|b| {
        let mut b = b.borrow_mut();
        b.1 = nodes;
        b.2 = title;
    });
    if let Some(button) = with_editor(ui, |ed| ed.problems) {
        let _ = ui.set_prop(button, "text", format!("⊗ {errors}  ⚠ {warnings}"));
    }
    if BOTTOM.with(|b| b.borrow().0) == Some(Bottom::Problems) {
        show_bottom(ui, Some(Bottom::Problems));
    }
}

/// As referências (Shift+F12): por arquivo, cada uma com a linha.
pub fn show_references(ui: &mut Ui, word: &str, rows: Vec<(PathBuf, Pos, String)>) {
    let Some((title, list)) = with_editor(ui, |ed| (ed.bottom_title, ed.bottom_list)) else { return };
    let mut groups: Vec<(PathBuf, Vec<TreeNode>)> = Vec::new();
    let total = rows.len();
    for (file, p, text) in rows {
        let node = place_node(format!("{}: {text}", p.0 + 1), &file, p);
        match groups.iter_mut().find(|(f, _)| *f == file) {
            Some((_, l)) => l.push(node),
            None => groups.push((file, vec![node])),
        }
    }
    let nodes = groups
        .into_iter()
        .map(|(f, children)| {
            let mut n = TreeNode::new(format!("{} ({})", file_name(&f), children.len())).with_children(children);
            n.expanded = true;
            n
        })
        .collect();
    show_bottom(ui, Some(Bottom::References));
    let _ = ui.set_prop(title, "text", format!("Referências de '{word}': {total}"));
    if let Some(t) = ui.get_mut::<TreeView>(list) {
        t.set_nodes(nodes);
    }
    ui.request_repaint();
}

// ---------------------------------------------------------------------------
// O que o servidor de linguagem responde

/// O texto do arquivo, se ele está aberto numa aba.
pub fn text_of(ui: &Ui, file: &Path) -> Option<String> {
    let memo = memo_of(ui, file)?;
    ui.get::<Memo>(memo).map(|m| m.text.clone())
}

/// Liga ou desliga o `sensitive` (gravações mascaradas) em todos os editores abertos.
pub fn set_sensitive(ui: &mut Ui, on: bool) {
    let memos: Vec<ComponentId> = with_editors(ui, |list| list.iter().flat_map(|ed| ed.tabs.iter().map(|t| t.memo)).collect()).unwrap_or_default();
    for memo in memos {
        let _ = ui.set_prop(memo, "sensitive", on);
    }
}

/// Os arquivos abertos e o texto de cada um (o servidor recebe ao abrir).
pub fn open_files(ui: &Ui) -> Vec<(PathBuf, String)> {
    let tabs: Vec<(PathBuf, ComponentId)> = with_editors(ui, |list| list.iter().flat_map(|ed| ed.tabs.iter().map(|t| (t.file.clone(), t.memo))).collect()).unwrap_or_default();
    tabs.into_iter().filter_map(|(f, m)| ui.get::<Memo>(m).map(|m| (f, m.text.clone()))).collect()
}

/// O editor da aba do arquivo.
fn memo_of(ui: &Ui, file: &Path) -> Option<ComponentId> {
    let key = crate::debugger::norm(file);
    with_editors(ui, |list| list.iter().find_map(|ed| ed.tabs.iter().find(|t| t.file == key).map(|t| t.memo))).flatten()
}

/// A lista de sugestões no editor (se ele ainda é o ativo e o cursor não saiu da palavra).
/// `start`: onde começa o trecho que elas trocam; sem ele, o começo da palavra.
pub fn show_suggestions(ui: &mut Ui, file: &Path, items: Vec<Suggestion>, start: Option<Pos>) {
    let Some(memo) = memo_of(ui, file).filter(|&m| active(ui).is_some_and(|(_, a)| a == m)) else { return };
    let Some(m) = ui.get_mut::<Memo>(memo) else { return };
    if items.is_empty() {
        m.hide_suggestions();
        return;
    }
    let caret = m.caret_index();
    let start = match start {
        Some(p) => m.index_of_position(p.0, p.1).min(caret),
        None => {
            let chars: Vec<char> = m.text.chars().collect();
            let mut i = caret.min(chars.len());
            while i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
                i -= 1;
            }
            i
        }
    };
    m.show_suggestions(items, start);
    ui.request_repaint();
}

/// A dica do mouse: os erros daquele ponto e o que o servidor disse (`markdown`, como ele
/// mandou: os blocos de código cercados e a documentação), numa caixa por cima do editor
/// ([`crate::hovertip`]).
pub fn show_hover(ui: &mut Ui, file: &Path, index: usize, markdown: &str) {
    let Some(memo) = memo_of(ui, file) else { return };
    let Some(m) = ui.get::<Memo>(memo) else { return };
    let mut parts: Vec<crate::hovertip::HoverPart> = m.diagnostics_at(index).iter().map(|d| crate::hovertip::problem(d.severity, &d.message)).collect();
    let said = crate::hovertip::parts(markdown);
    if !parts.is_empty() && !said.is_empty() {
        parts.push(crate::hovertip::HoverPart::Separator);
    }
    parts.extend(said);
    let theme = with_editor(ui, |ed| ed.color_theme.clone()).unwrap_or_default();
    crate::hovertip::show(ui, memo, index, &parts, &syntax_of(file), &theme);
}

/// A ajuda dos parâmetros acima do cursor.
pub fn show_signature(ui: &mut Ui, file: &Path, text: &str) {
    let Some(memo) = memo_of(ui, file) else { return };
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        m.show_signature(text);
    }
    ui.request_repaint();
}

/// Os erros e avisos de um arquivo aberto (os sublinhados).
pub fn set_diagnostics(ui: &mut Ui, file: &Path, items: &[aylis_tools::lsp::Diag]) {
    let Some(memo) = memo_of(ui, file) else { return };
    let Some(m) = ui.get_mut::<Memo>(memo) else { return };
    let list = items
        .iter()
        .map(|d| Diagnostic { start: m.index_of_position(d.start.0, d.start.1), end: m.index_of_position(d.end.0, d.end.1), severity: d.severity, message: d.message.clone() })
        .collect();
    m.set_diagnostics(list);
    ui.request_repaint();
}

/// Troca trechos de um arquivo aberto (renomear, formatar, correção). `false` se ele não
/// está aberto (quem chamou mexe no disco).
pub fn apply_file_edits(ui: &mut Ui, file: &Path, edits: &[(Pos, Pos, String)]) -> bool {
    let Some(memo) = memo_of(ui, file) else { return false };
    let now = ui.time();
    let changed = ui.get_mut::<Memo>(memo).is_some_and(|m| {
        let list = edits.iter().map(|(a, b, t)| (m.index_of_position(a.0, a.1), m.index_of_position(b.0, b.1), t.clone())).collect();
        m.apply_edits(list, now)
    });
    if changed {
        let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
        ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.into()));
    }
    true
}

/// Salva o arquivo aberto numa aba (`false`: não está aberto).
pub fn save_file(ui: &mut Ui, file: &Path) -> bool {
    let key = crate::debugger::norm(file);
    let Some((form, i)) = find_file(ui, &key) else { return false };
    let Some(memo) = with_editor_at(ui, form, |ed| ed.tabs.get(i).map(|t| t.memo)).flatten() else { return false };
    CURRENT.with(|c| c.set(Some(form)));
    save_memo(ui, memo);
    true
}

/// O `edit_code` do MCP: troca `old` por `new` no texto do editor do arquivo (aberto antes,
/// se preciso), como uma edição dele: o Ctrl+Z desfaz, o servidor de linguagem fica sabendo e o
/// arquivo fica para salvar. `old` tem de aparecer uma vez só, a não ser com `all`. Devolve
/// quantas trocou e a linha (a partir de 1) da primeira.
pub fn edit_text(ui: &mut Ui, file: &Path, old: &str, new: &str, all: bool) -> Result<(usize, usize), String> {
    if old.is_empty() {
        return Err("`old` vazio: diga o trecho a trocar (com linhas em volta, para ser único)".into());
    }
    if memo_of(ui, file).is_none() {
        open(ui, file, 1)?;
    }
    let memo = memo_of(ui, file).ok_or_else(|| format!("não consegui abrir {}", file.display()))?;
    let now = ui.time();
    let result = {
        let m = ui.get_mut::<Memo>(memo).ok_or("o editor sumiu")?;
        if m.read_only {
            return Err(format!("{} é somente leitura (código gerado pela IDE: mude o form, e o Gerar código refaz)", file.display()));
        }
        let starts: Vec<usize> = m.text.match_indices(old).map(|(i, _)| i).collect();
        match (starts.len(), all) {
            (0, _) => return Err("`old` não aparece no arquivo (o texto do editor, com o que ainda não foi salvo; leia com read_code)".into()),
            (n, false) if n > 1 => return Err(format!("`old` aparece {n} vezes: ponha mais linhas em volta para ser único, ou use `all`")),
            _ => {}
        }
        let chars = |byte: usize| m.text[..byte].chars().count();
        let old_chars = old.chars().count();
        let line = m.text[..starts[0]].matches('\n').count() + 1;
        let edits: Vec<(usize, usize, String)> = starts.iter().map(|&b| (chars(b), chars(b) + old_chars, new.to_string())).collect();
        let count = edits.len();
        m.apply_edits(edits, now);
        (count, line)
    };
    let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
    ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.into()));
    ui.request_repaint();
    Ok(result)
}

/// Abre o arquivo com o cursor numa posição do LSP (linha a partir de 0, coluna UTF-16).
pub fn go_to_position(ui: &mut Ui, file: &Path, (line, col): Pos) {
    let Ok(w) = open_here(ui, file, line + 1) else { return };
    if let Some(m) = ui.get_mut::<Memo>(w.memo) {
        let index = m.index_of_position(line, col);
        m.select(index, index);
    }
    show_status(ui);
}

/// Uma mensagem na barra de cima.
pub fn set_status(ui: &mut Ui, text: &str) {
    if let Some(status) = with_editor(ui, |ed| ed.activity) {
        let _ = ui.set_prop(status, "text", text);
    }
}

/// O nome do servidor de linguagem (ou que está iniciando, ou que falta) na barra de cima.
/// `hint`: o detalhe (o que ele ainda está fazendo); vazio, a explicação de sempre.
pub fn refresh_server_status(ui: &mut Ui, text: &str, hint: &str) {
    if let Some(label) = with_editor(ui, |ed| ed.server) {
        let _ = ui.set_prop(label, "text", text);
        let hint = if hint.is_empty() { "O servidor de linguagem (sugestões, definição, erros). Ferramentas → Verificar ambiente instala." } else { hint };
        let _ = ui.set_prop(label, "hint", hint);
    }
}

// ---------------------------------------------------------------------------
// Os diálogos rápidos (o "Quick Pick" do VS Code)

/// Uma lista com filtro: Enter (ou OK) escolhe.
pub fn quick_pick(ui: &mut Ui, title: &str, items: Vec<String>, on_pick: impl FnOnce(&mut Ui, usize) + 'static) {
    use crate::dialogs::{edit, field, new_dialog, run, text_of as dialog_text};
    let d = new_dialog(ui, title, 640.0, 520.0);
    let line = field(ui, d.body, "Filtro");
    let filter = edit(ui, line, "");
    let list = ui.add(ListBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client).set_margins([0.0, 8.0, 0.0, 0.0]);
    let shown: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::new()));
    let fill = {
        let shown = shown.clone();
        move |ui: &mut Ui| {
            let query = dialog_text(ui, filter).to_lowercase();
            let rows: Vec<usize> = items.iter().enumerate().filter(|(_, t)| query.split_whitespace().all(|w| t.to_lowercase().contains(w))).map(|(i, _)| i).collect();
            let _ = ui.set_prop(list, "items", rows.iter().map(|&i| items[i].replace(';', ",")).collect::<Vec<_>>().join(";"));
            let _ = ui.set_prop(list, "item_index", if rows.is_empty() { -1i64 } else { 0 });
            *shown.borrow_mut() = rows;
        }
    };
    fill(ui);
    ui.on(filter, "on_change", move |ui, _, _| fill(ui));
    // ↑/↓ no filtro andam na lista (como no VS Code).
    {
        let shown = shown.clone();
        ui.on(filter, "on_key_down", move |ui, _, args| {
            let aylis_core::EventArgs::Key(event) = args else { return };
            let step = match event.key {
                aylis_core::Key::Down => 1,
                aylis_core::Key::Up => -1,
                _ => return,
            };
            let count = shown.borrow().len() as i64;
            let i = ui.get_prop(list, "item_index").ok().and_then(|v| v.to_i64().ok()).unwrap_or(-1);
            if count > 0 {
                let _ = ui.set_prop(list, "item_index", (i + step).clamp(0, count - 1));
            }
        });
    }
    let pick = move |ui: &Ui| {
        let i = ui.get_prop(list, "item_index").ok().and_then(|v| v.to_i64().ok()).unwrap_or(-1);
        usize::try_from(i).ok().and_then(|i| shown.borrow().get(i).copied())
    };
    let pick2 = pick.clone();
    run(ui, d, move |ui| if pick(ui).is_some() { Ok(()) } else { Err("escolha um item".into()) }, move |ui| {
        if let Some(i) = pick2(ui) {
            on_pick(ui, i);
        }
    });
    ui.set_focus(filter);
}

/// Os símbolos (do arquivo ou do projeto): nome, tipo e onde.
pub fn symbols_dialog(ui: &mut Ui, title: &str, rows: Vec<(String, String, PathBuf, Pos)>) {
    if rows.is_empty() {
        set_status(ui, "Nenhum símbolo");
        return;
    }
    let titles = rows.iter().map(|(n, k, f, p)| format!("{n}   — {k}, {}:{}", file_name(f), p.0 + 1)).collect();
    quick_pick(ui, title, titles, move |ui, i| {
        let (_, _, file, p) = rows[i].clone();
        go_to_position(ui, &file, p);
    });
}

/// Símbolo no projeto (Ctrl+T): as declarações de todos os fontes (o índice próprio).
fn project_symbols_dialog(ui: &mut Ui) {
    let Some(h) = hooks() else { return };
    let (_, files) = (h.sources)();
    let mut rows = Vec::new();
    for file in files {
        let language = aylis_project::Language::ALL.into_iter().find(|l| aylis_project::symbols::extensions(*l).iter().any(|e| file.extension().is_some_and(|x| x.eq_ignore_ascii_case(e))));
        let (Some(language), Ok(text)) = (language, std::fs::read_to_string(&file)) else { continue };
        for d in aylis_project::symbols::declarations(language, &text) {
            rows.push((d.name, d.kind.to_string(), file.clone(), (d.line - 1, 0)));
        }
    }
    symbols_dialog(ui, "Símbolo no projeto", rows);
}

/// Ctrl+G: ir para a linha.
fn go_to_line_dialog(ui: &mut Ui) {
    use crate::dialogs::{edit, field, new_dialog, run, text_of as dialog_text};
    let Some((file, memo)) = active(ui) else { return };
    let total = ui.get::<Memo>(memo).map_or(1, |m| m.text.split('\n').count());
    let d = new_dialog(ui, "Ir para a linha", 360.0, 170.0);
    let line = field(ui, d.body, &format!("Linha (1 a {total})"));
    let number = edit(ui, line, "");
    run(
        ui,
        d,
        move |ui| match dialog_text(ui, number).trim().parse::<usize>() {
            Ok(n) if n >= 1 => Ok(()),
            _ => Err("digite o número da linha".into()),
        },
        move |ui| {
            if let Ok(n) = dialog_text(ui, number).trim().parse::<usize>() {
                let _ = open_here(ui, &file, n.min(total));
            }
        },
    );
    ui.set_focus(number);
}

/// F2: renomear o símbolo do cursor (o servidor troca em todos os fontes).
fn rename_dialog(ui: &mut Ui) {
    use crate::dialogs::{edit, field, new_dialog, run, text_of as dialog_text};
    let Some((file, index, word)) = caret_word(ui) else { return };
    if word.is_empty() {
        return;
    }
    let d = new_dialog(ui, "Renomear o símbolo", 420.0, 170.0);
    let line = field(ui, d.body, "Novo nome");
    let name = edit(ui, line, &word);
    run(
        ui,
        d,
        move |ui| if dialog_text(ui, name).trim().is_empty() { Err("digite o nome".into()) } else { Ok(()) },
        move |ui| {
            let name = dialog_text(ui, name).trim().to_string();
            lsp_event(ui, LspEvent::Rename { file, index, name });
        },
    );
    ui.set_focus(name);
}

// ---------------------------------------------------------------------------
// Para quem comanda a IDE de fora (o MCP): o que o usuário faz no editor e o que ele vê

/// A janela do editor atual (a da IDE, se ele está embutido; `None`: nenhum editor), a que
/// recebe as teclas e o texto.
pub fn code_form(ui: &Ui) -> Option<ComponentId> {
    with_editor(ui, |ed| ed.form).map(|f| ui.root_of(f))
}

/// O mouse parado na linha e coluna (a partir de 1) do editor ativo: pede a dica.
pub fn hover_at(ui: &mut Ui, line: usize, column: usize) -> bool {
    let Some((file, memo)) = active(ui) else { return false };
    let Some(index) = ui.get::<Memo>(memo).map(|m| m.index_of_position(line.saturating_sub(1), column.saturating_sub(1))) else { return false };
    lsp_event(ui, LspEvent::Hover { file, index });
    true
}

/// O que o editor ativo mostra: o arquivo, o cursor, a seleção, a lista de sugestões, a
/// dica, a ajuda dos parâmetros, os erros e o painel de baixo.
pub fn editor_state(ui: &Ui) -> serde_json::Value {
    use serde_json::json;
    let Some((file, memo)) = active(ui) else { return json!({ "open": false }) };
    let Some(m) = ui.get::<Memo>(memo) else { return json!({ "open": false }) };
    let (line, col) = m.caret_position();
    let shown = m.visible_suggestions();
    let bottom = with_editor(ui, |ed| ui.get::<Label>(ed.bottom_title).map(|l| l.text.clone()).unwrap_or_default()).unwrap_or_default();
    let bottom_open = with_editor(ui, |ed| ui.control(ed.bottom).is_some_and(|c| c.visible())).unwrap_or(false);
    let server = with_editor(ui, |ed| ui.get::<Label>(ed.server).map(|l| l.text.clone()).unwrap_or_default()).unwrap_or_default();
    let line_text = m.text.split('\n').nth(line).unwrap_or_default().to_string();
    json!({
        "open": true,
        "file": file.display().to_string(),
        "line": line + 1,
        "column": col + 1,
        "line_text": line_text,
        "selection": m.sel_text(),
        "cursors": m.cursors().len(),
        "folded": m.folded_lines().iter().map(|l| l + 1).collect::<Vec<_>>(),
        "breadcrumb": with_editor(ui, |ed| ui.get::<Label>(ed.breadcrumb).map(|l| l.text.clone()).unwrap_or_default()).unwrap_or_default(),
        "suggestions": shown.iter().take(15).map(|s| format!("{} {}", s.label, s.detail).trim().to_string()).collect::<Vec<_>>(),
        "suggestion_count": shown.len(),
        "tip": crate::hovertip::text_of(memo),
        "signature": m.signature(),
        "diagnostics": m.diagnostics().iter().map(|d| json!({ "severity": d.severity, "message": d.message, "line": m.position_of_index(d.start).0 + 1 })).collect::<Vec<_>>(),
        "bottom_panel": if bottom_open { bottom } else { String::new() },
        "server": server,
        "can_undo": m.can_undo(),
    })
}

// ---------------------------------------------------------------------------
// Marcadores

/// Onde ficam os marcadores do arquivo: `.aylis/bookmarks.json` na pasta do projeto (a que tem o
/// `.aylisproj`), fora do git (a pasta `.aylis` ganha um `.gitignore`). `None` fora de um projeto.
fn bookmarks_store(file: &Path) -> Option<(PathBuf, String)> {
    let root = file.ancestors().skip(1).find(|dir| {
        std::fs::read_dir(dir).is_ok_and(|mut entries| entries.any(|e| e.is_ok_and(|e| e.path().extension().is_some_and(|x| x == "aylisproj"))))
    })?;
    let key = file.strip_prefix(root).ok()?.to_string_lossy().replace('\\', "/");
    Some((root.join(".aylis").join("bookmarks.json"), key))
}

/// Os marcadores guardados do arquivo, com os nomes.
fn load_bookmarks(file: &Path) -> Vec<(aylis_controls::Bookmark, String)> {
    let Some((path, key)) = bookmarks_store(file) else { return Vec::new() };
    let all: serde_json::Value = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    all[&key]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|m| {
                    let line = m["line"].as_u64()? as usize;
                    let number = m["number"].as_u64().map(|n| n.min(9) as u8);
                    Some((aylis_controls::Bookmark { line, number }, m["name"].as_str().unwrap_or_default().to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Guarda os marcadores do arquivo; os nomes seguem os marcadores na ordem (as linhas andam
/// juntas com o texto, a ordem não muda).
fn save_bookmarks(file: &Path, marks: &[aylis_controls::Bookmark], names: &[String]) {
    let Some((path, key)) = bookmarks_store(file) else { return };
    let mut all: serde_json::Value = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_else(|| serde_json::json!({}));
    let items: Vec<serde_json::Value> = marks
        .iter()
        .enumerate()
        .map(|(i, m)| serde_json::json!({ "line": m.line, "number": m.number, "name": names.get(i).cloned().unwrap_or_default() }))
        .collect();
    if items.is_empty() {
        if let Some(map) = all.as_object_mut() {
            map.remove(&key);
        }
    } else {
        all[key] = serde_json::Value::Array(items);
    }
    if let Some(dir) = path.parent() {
        // A mesma regra que a geração e as horas dos forms usam (o arquivo não muda mais).
        aylis_project::local_dir_ignored(dir);
    }
    let _ = std::fs::write(&path, serde_json::to_string_pretty(&all).unwrap_or_default());
}

/// Liga os marcadores do editor `memo` do arquivo: carrega os guardados e guarda a cada mudança.
fn attach_bookmarks(ui: &mut Ui, memo: ComponentId, file: &Path) {
    let saved = load_bookmarks(file);
    let names: Vec<String> = saved.iter().map(|(_, n)| n.clone()).collect();
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        m.set_bookmarks(saved.iter().map(|(b, _)| *b).collect());
    }
    let names = Rc::new(RefCell::new(names));
    let save = {
        let (file, names) = (file.to_path_buf(), names.clone());
        move |ui: &mut Ui| {
            let marks = ui.get::<Memo>(memo).map(|m| m.bookmarks().to_vec()).unwrap_or_default();
            let mut n = names.borrow_mut();
            n.resize(marks.len(), String::new());
            save_bookmarks(&file, &marks, &n);
        }
    };
    let save = Rc::new(save);
    {
        let save = save.clone();
        ui.on(memo, "on_bookmarks_change", move |ui, _, _| save(ui));
    }
    // As linhas andam com o texto: guarda ao salvar o arquivo (o `on_change` é a cada tecla).
    BOOKMARK_SAVERS.with(|s| s.borrow_mut().push((memo, file.to_path_buf(), names, save)));
}

type BookmarkSaver = (ComponentId, PathBuf, Rc<RefCell<Vec<String>>>, Rc<dyn Fn(&mut Ui)>);

thread_local! {
    /// Os editores com marcadores ligados: o arquivo, os nomes e quem guarda.
    static BOOKMARK_SAVERS: RefCell<Vec<BookmarkSaver>> = const { RefCell::new(Vec::new()) };
}

/// Guarda os marcadores do editor `memo` (depois de salvar o arquivo, com as linhas atuais).
fn save_bookmarks_of(ui: &mut Ui, memo: ComponentId) {
    let save = BOOKMARK_SAVERS.with(|s| s.borrow().iter().find(|(m, ..)| *m == memo).map(|(.., f)| f.clone()));
    if let Some(save) = save {
        save(ui);
    }
}

/// Exibir → Marcadores (Ctrl+Alt+B): os de todos os arquivos abertos, com o nome, o arquivo, a
/// linha e o texto dela; escolher vai até ele.
fn bookmarks_list(ui: &mut Ui) {
    let savers = BOOKMARK_SAVERS.with(|s| s.borrow().iter().map(|(m, f, n, _)| (*m, f.clone(), n.clone())).collect::<Vec<_>>());
    let mut entries: Vec<(PathBuf, usize, String)> = Vec::new();
    for (memo, file, names) in savers {
        let Some(m) = ui.get::<Memo>(memo) else { continue };
        let lines: Vec<&str> = m.text.split('\n').collect();
        for (i, b) in m.bookmarks().iter().enumerate() {
            let name = names.borrow().get(i).cloned().unwrap_or_default();
            let number = b.number.map(|n| format!("[{n}] ")).unwrap_or_default();
            let label = if name.is_empty() { String::new() } else { format!("{name} · ") };
            let text = lines.get(b.line).map(|t| t.trim()).unwrap_or_default();
            entries.push((file.clone(), b.line, format!("{number}{label}{}:{} — {text}", file_name(&file), b.line + 1)));
        }
    }
    if entries.is_empty() {
        aylis_controls::show_message(ui, "Nenhum marcador. Ctrl+Shift+número ou Ctrl+Alt+K marcam a linha do cursor.");
        return;
    }
    let items = entries.iter().map(|(.., t)| t.clone()).collect();
    quick_pick(ui, "Marcadores", items, move |ui, i| {
        if let Some((file, line, _)) = entries.get(i) {
            let _ = open(ui, file, line + 1);
        }
    });
}

/// Dá um nome ao marcador da linha do cursor (aparece na lista dos marcadores).
fn rename_bookmark(ui: &mut Ui) {
    let Some((_, memo)) = active(ui) else { return };
    let Some(m) = ui.get::<Memo>(memo) else { return };
    let line = m.caret_line() - 1;
    let Some(index) = m.bookmarks().iter().position(|b| b.line == line) else {
        aylis_controls::show_message(ui, "A linha do cursor não tem marcador.");
        return;
    };
    let saver = BOOKMARK_SAVERS.with(|s| s.borrow().iter().find(|(id, ..)| *id == memo).map(|(_, _, n, f)| (n.clone(), f.clone())));
    let Some((names, save)) = saver else { return };
    let current = names.borrow().get(index).cloned().unwrap_or_default();
    crate::dialogs::input_text(ui, "Nome do marcador", "Nome", &current, |_| Ok(()), move |ui, name| {
        let mut n = names.borrow_mut();
        if n.len() <= index {
            n.resize(index + 1, String::new());
        }
        n[index] = name;
        drop(n);
        save(ui);
    });
}

#[cfg(test)]
mod generation_tests {
    use super::*;

    fn apply(text: &str, (a, b, with): (usize, usize, String)) -> String {
        let chars: Vec<char> = text.chars().collect();
        chars[..a].iter().collect::<String>() + &with + &chars[b..].iter().collect::<String>()
    }

    /// A geração acrescentou um handler no fim do `impl`: o editor recebe o mesmo, esteja ele
    /// igual ao disco ou com alterações não salvas noutro lugar.
    #[test]
    fn the_editor_follows_what_the_generation_added() {
        let old = "impl FrmPrincipal {\n    fn salvar(&mut self) {}\n}\n";
        let new = "impl FrmPrincipal {\n    fn salvar(&mut self) {}\n\n    fn btn_ok_click(&mut self) {}\n}\n";
        assert_eq!(apply(old, merge_generated(old, new, old).unwrap()), new);
        let edited = "// meu comentário, não salvo\nimpl FrmPrincipal {\n    fn salvar(&mut self) {}\n}\n";
        let merged = apply(edited, merge_generated(old, new, edited).unwrap());
        assert_eq!(merged, format!("// meu comentário, não salvo\n{new}"));
        // O lugar não dá para achar (o editor mexeu bem ali): não junta.
        let moved = "impl FrmPrincipal {\n    fn salvar(&mut self) { gravar(); }\n}\n";
        assert_eq!(merge_generated(old, new, moved), None);
        // Acentos: as posições são em caracteres.
        let (o, n) = ("é\nfim\n", "é\nnovo\nfim\n");
        assert_eq!(apply("ç é\nfim\n", merge_generated(o, n, "ç é\nfim\n").unwrap()), "ç é\nnovo\nfim\n");
    }
}

#[cfg(test)]
mod bookmark_tests {
    use super::*;

    /// Os marcadores vão para `.aylis/bookmarks.json` do projeto (com o `.gitignore`) e voltam
    /// com os nomes; sem marcadores o arquivo sai da lista.
    #[test]
    fn bookmarks_are_saved_in_the_project_and_loaded_back() {
        let dir = std::env::temp_dir().join(format!("aylis-marcadores-{}", std::process::id()));
        let src = dir.join("rust").join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(dir.join("App.aylisproj"), "{}").unwrap();
        let file = src.join("main.rs");
        std::fs::write(&file, "a\nb\nc").unwrap();
        let marks = [aylis_controls::Bookmark { line: 1, number: Some(4) }, aylis_controls::Bookmark { line: 2, number: None }];
        save_bookmarks(&file, &marks, &["cálculo".to_string(), String::new()]);
        assert!(dir.join(".aylis").join(".gitignore").exists());
        let back = load_bookmarks(&file);
        assert_eq!(back.len(), 2);
        assert_eq!(back[0], (marks[0], "cálculo".to_string()));
        save_bookmarks(&file, &[], &[]);
        assert!(load_bookmarks(&file).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// A pesquisa no projeto substitui num arquivo: se ele está aberto num editor, a troca vai
/// pelo editor (o Ctrl+Z dele desfaz; o arquivo fica para salvar) e devolve `true`. `edits` são
/// trechos em bytes de `disk` (o conteúdo no disco); com o editor diferente do disco (mudanças
/// não salvas), não troca nada e devolve `true` (o arquivo não é sobrescrito).
pub fn apply_to_open_file(ui: &mut Ui, file: &Path, disk: &str, edits: &[(usize, usize, String)]) -> bool {
    let key = file.to_path_buf();
    let Some((form, i)) = find_file(ui, &key) else { return false };
    let Some(memo) = with_editor_at(ui, form, |ed| ed.tabs.get(i).map(|t| t.memo)).flatten() else { return false };
    let now = ui.time();
    let changed = ui.get_mut::<Memo>(memo).is_some_and(|m| {
        if m.text != disk {
            return false;
        }
        let chars = |byte: usize| disk[..byte].chars().count();
        m.apply_edits(edits.iter().map(|(a, b, t)| (chars(*a), chars(*b), t.clone())).collect(), now)
    });
    if changed {
        let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
        ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.into()));
        ui.request_repaint();
    }
    true
}

/// Os fontes abertos que mudaram no disco desde que foram lidos ou salvos: o arquivo, se o
/// editor tem alteração não salva e o texto do editor ([`crate::disk`]).
pub fn disk_changes(ui: &Ui) -> Vec<(PathBuf, bool, String)> {
    let tabs: Vec<(PathBuf, ComponentId, bool, Option<aylis_project::FileStamp>)> =
        with_editors(ui, |list| list.iter().flat_map(|ed| ed.tabs.iter().map(|t| (t.file.clone(), t.memo, t.modified, t.disk))).collect()).unwrap_or_default();
    tabs.into_iter()
        .filter(|(file, _, _, disk)| {
            let now = aylis_project::FileStamp::read(file);
            now.is_some() && now != *disk
        })
        .filter_map(|(file, memo, modified, _)| Some((file, modified, ui.get::<Memo>(memo)?.text.clone())))
        .collect()
}

/// O fonte está aberto num editor.
pub fn is_open(ui: &Ui, file: &Path) -> bool {
    find_file(ui, file).is_some()
}

/// Fica o texto do editor: a marca passa a ser a de agora (não pergunta de novo até o disco
/// mudar outra vez).
pub fn keep_editor(ui: &mut Ui, file: &Path) {
    let stamp = aylis_project::FileStamp::read(file);
    let Some((form, i)) = find_file(ui, file) else { return };
    with_editor_at(ui, form, |ed| {
        if let Some(t) = ed.tabs.get_mut(i) {
            t.disk = stamp;
        }
    });
}

/// O editor passa a ter o texto do disco (como uma edição: o Ctrl+Z volta) e fica salvo.
pub fn reload_from_disk(ui: &mut Ui, file: &Path, text: &str) {
    let stamp = aylis_project::FileStamp::read(file);
    let Some((form, i)) = find_file(ui, file) else { return };
    let Some(memo) = with_editor_at(ui, form, |ed| ed.tabs.get(i).map(|t| t.memo)).flatten() else { return };
    let now = ui.time();
    if let Some(m) = ui.get_mut::<Memo>(memo) {
        // O gerado (somente leitura) também acompanha o disco.
        let read_only = std::mem::replace(&mut m.read_only, false);
        let all = m.text.chars().count();
        m.apply_edits(vec![(0, all, text.to_string())], now);
        m.read_only = read_only;
    }
    ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.to_string().into()));
    with_editor_at(ui, form, |ed| {
        if let Some(t) = ed.tabs.get_mut(i) {
            t.modified = false;
            t.disk = stamp;
        }
    });
    CURRENT.with(|c| c.set(Some(form)));
    show_status(ui);
}

/// Os arquivos abertos nos editores (sem o `Ui`: quem prepara um trabalho guarda o texto deles
/// no disco antes de gerar o código, para o [`follow_generation`] depois).
pub fn open_paths() -> Vec<PathBuf> {
    EDITORS.with(|e| e.borrow().iter().flat_map(|ed| ed.tabs.iter().map(|t| t.file.clone())).collect())
}

/// O texto no disco dos arquivos abertos, antes de a geração mexer.
pub fn snapshot(files: &[PathBuf]) -> Vec<(PathBuf, String)> {
    files.iter().filter_map(|f| std::fs::read_to_string(f).ok().map(|t| (f.clone(), t))).collect()
}

/// A geração de código mexeu no disco de arquivos abertos (`before`: o texto deles antes): o
/// editor acompanha, como uma edição dele (o Ctrl+Z desfaz). Sem alteração no editor, ele fica
/// igual ao disco e continua "salvo"; com alteração não salva, o que a geração pôs entra no
/// meio do texto do editor, no mesmo lugar (senão salvar o editor apagava os handlers novos,
/// achado na conferência de 08/11). Devolve os arquivos em que não deu para juntar.
pub fn follow_generation(ui: &mut Ui, before: Vec<(PathBuf, String)>) -> Vec<PathBuf> {
    let mut failed = Vec::new();
    for (file, old) in before {
        let Ok(new) = std::fs::read_to_string(&file) else { continue };
        if new == old {
            continue;
        }
        let Some((form, i)) = find_file(ui, &file) else { continue };
        let Some((memo, modified)) = with_editor_at(ui, form, |ed| ed.tabs.get(i).map(|t| (t.memo, t.modified))).flatten() else { continue };
        let Some(text) = ui.get::<Memo>(memo).map(|m| m.text.clone()) else { continue };
        let Some(edit) = merge_generated(&old, &new, &text) else {
            failed.push(file);
            continue;
        };
        let now = ui.time();
        if let Some(m) = ui.get_mut::<Memo>(memo) {
            m.apply_edits(vec![edit], now);
        }
        let text = ui.get::<Memo>(memo).map(|m| m.text.clone()).unwrap_or_default();
        ui.fire(memo, "on_change", &aylis_core::EventArgs::Value(text.into()));
        // O editor já tem o que a geração gravou: a marca do disco é a nova (senão parecia
        // mudado por fora). Igual ao disco, continua salvo.
        let stamp = aylis_project::FileStamp::read(&file);
        with_editor_at(ui, form, |ed| {
            if let Some(t) = ed.tabs.get_mut(i) {
                t.disk = stamp;
                if !modified {
                    t.modified = false;
                }
            }
        });
        if !modified {
            CURRENT.with(|c| c.set(Some(form)));
            show_status(ui);
        }
        ui.request_repaint();
    }
    failed
}

/// A troca (em caracteres de `text`, o do editor) que leva ao editor o que a geração mudou de
/// `old` para `new` no disco. O trecho que mudou é um só (a geração acrescenta os handlers no
/// fim do `impl`); com o editor igual a `old`, vale direto; com alterações dele, o trecho é
/// achado pelo texto em volta, que tem de aparecer uma vez só. `None`: não dá para juntar.
fn merge_generated(old: &str, new: &str, text: &str) -> Option<(usize, usize, String)> {
    let (o, n): (Vec<char>, Vec<char>) = (old.chars().collect(), new.chars().collect());
    let prefix = o.iter().zip(&n).take_while(|(a, b)| a == b).count();
    let max_suffix = o.len().min(n.len()) - prefix;
    let suffix = o.iter().rev().zip(n.iter().rev()).take(max_suffix).take_while(|(a, b)| a == b).count();
    let removed: String = o[prefix..o.len() - suffix].iter().collect();
    let added: String = n[prefix..n.len() - suffix].iter().collect();
    if text == old {
        return Some((prefix, o.len() - suffix, added));
    }
    // O texto em volta do trecho (até 200 caracteres de cada lado), no editor.
    const CONTEXT: usize = 200;
    let before: String = o[prefix.saturating_sub(CONTEXT)..prefix].iter().collect();
    let after: String = o[o.len() - suffix..(o.len() - suffix + CONTEXT).min(o.len())].iter().collect();
    let needle = format!("{before}{removed}{after}");
    let mut found = text.match_indices(&needle);
    let (at, _) = found.next()?;
    if found.next().is_some() {
        return None;
    }
    let start = text[..at].chars().count() + before.chars().count();
    Some((start, start + removed.chars().count(), added))
}

/// O arquivo é dos que a IDE refaz a cada geração (a pasta `generated/` do Rust, o acesso
/// tipado e o registro das outras linguagens, os que só apontam para o cliente do aylis).
fn is_generated(file: &Path, text: &str) -> bool {
    let in_generated_dir = file.components().any(|c| c.as_os_str() == "generated");
    let head: String = text.chars().take(600).collect();
    in_generated_dir || head.contains("\"Gerar código\": não edite")
}
