//! Pesquisar no projeto (Ctrl+Shift+F), como no VS Code: o texto (ou a expressão regular) em
//! todos os fontes do projeto, com as opções do localizar, os arquivos a incluir e a excluir
//! (`src/**/*.rs`, `*.json`) e, à parte, nos forms (o nome dos componentes, os textos, os
//! handlers e os valores das propriedades). Os resultados vêm numa árvore por arquivo; o clique
//! abre o código na linha ou o form no designer com o componente escolhido e a propriedade em
//! destaque. Substituir troca só os marcados (☑, o espaço alterna); nos arquivos abertos vai pelo
//! desfazer do editor ou do designer, nos fechados o "Desfazer a substituição" volta o conteúdo.
//!
//! A procura roda numa thread (não trava a IDE); `target`, `node_modules`, `dist`, `.git` e o
//! que o `.gitignore` do projeto lista ficam de fora.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use aylis_controls::search::SearchOptions;
use aylis_controls::{Button, CheckBox, Edit, Form, Label, Layout, TreeNode, TreeView};
use aylis_core::{Align, ComponentId, EventArgs, Key, Ui};
use serde_json::Value;

use crate::app::Ids;
use crate::state::Shared;
use crate::widgets::muted;

/// Pastas que nunca entram (compilação, dependências, ferramentas).
const SKIP: [&str; 12] = ["target", "node_modules", "bin", "obj", "build", "dist", "out", ".aylis", "__pycache__", ".git", ".vs", ".idea"];
/// Maior arquivo lido (os maiores são dados, não código).
const MAX_FILE: u64 = 2 * 1024 * 1024;

/// Uma ocorrência num fonte.
#[derive(Clone, Debug, PartialEq)]
pub struct TextHit {
    pub file: PathBuf,
    /// Linha (a partir de 0) e o texto dela.
    pub line: usize,
    pub text: String,
    /// Onde, em bytes do arquivo (para substituir).
    pub byte_start: usize,
    pub byte_end: usize,
    /// A troca já decidida (a do servidor de linguagem, no renomear): o "Substituir" põe este
    /// texto no trecho, sem procurar de novo. `None`: a troca da busca.
    pub replace_with: Option<String>,
}

/// Uma ocorrência num form: o componente, a propriedade e o valor.
#[derive(Clone, Debug, PartialEq)]
pub struct FormHit {
    pub form: String,
    pub file: PathBuf,
    pub component: String,
    /// `name` (o próprio nome), uma propriedade ou um evento (`on_click`).
    pub property: String,
    pub value: String,
}

/// O que a procura achou.
#[derive(Clone, Debug, Default)]
pub struct Results {
    pub text: Vec<TextHit>,
    pub forms: Vec<FormHit>,
    pub files: usize,
}

/// Um padrão de arquivos (`*.rs`, `src/**/*.ts`, `forms/`) como expressão: sem `/` vale para o
/// nome ou qualquer pasta do caminho.
fn glob_matches(pattern: &str, rel: &str) -> bool {
    let pattern = pattern.trim().trim_start_matches("./").trim_end_matches('/');
    if pattern.is_empty() {
        return false;
    }
    let mut re = String::from("^");
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' if chars.peek() == Some(&'*') => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    chars.next();
                    re.push_str("(?:.*/)?");
                } else {
                    re.push_str(".*");
                }
            }
            '*' => re.push_str("[^/]*"),
            '?' => re.push_str("[^/]"),
            c => re.push_str(&regex_escape(c)),
        }
    }
    re.push('$');
    let Ok(re) = regex_lite(&re) else { return false };
    if pattern.contains('/') {
        // Também vale para o que está dentro (`src` pega `src/main.rs`).
        re.is_match(rel) || rel.split('/').scan(String::new(), |acc, part| {
            if !acc.is_empty() {
                acc.push('/');
            }
            acc.push_str(part);
            Some(acc.clone())
        }).any(|prefix| re.is_match(&prefix))
    } else {
        rel.split('/').any(|part| re.is_match(part))
    }
}

fn regex_escape(c: char) -> String {
    let mut s = String::new();
    if "\\.+()[]{}^$|".contains(c) {
        s.push('\\');
    }
    s.push(c);
    s
}

fn regex_lite(pattern: &str) -> Result<RegexMatcher, String> {
    // O mesmo regex do localizar (sem diferenciar maiúsculas: Windows e macOS não diferenciam
    // nomes de arquivo).
    let o = SearchOptions { query: pattern.to_string(), regex: true, ..Default::default() };
    o.find_all("").map(|_| RegexMatcher(o))
}

struct RegexMatcher(SearchOptions);

impl RegexMatcher {
    fn is_match(&self, text: &str) -> bool {
        // Ancorado nas duas pontas (^...$): achou = casou tudo.
        self.0.find_all(text).is_ok_and(|f| f.iter().any(|m| m.byte_start == 0 && m.byte_end == text.len()))
    }
}

/// Os padrões de uma lista separada por vírgulas.
fn patterns(text: &str) -> Vec<String> {
    text.split(',').map(str::trim).filter(|p| !p.is_empty()).map(str::to_string).collect()
}

/// As linhas do `.gitignore` do projeto que dá para seguir (sem `!` e sem comentários).
fn gitignore(root: &Path) -> Vec<String> {
    std::fs::read_to_string(root.join(".gitignore"))
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('!'))
        .map(|l| l.trim_start_matches('/').to_string())
        .collect()
}

/// Os fontes do projeto a procurar: os arquivos de texto, fora as pastas puladas, o
/// `.gitignore`, os forms (que entram à parte) e o que `exclude` pede; só o que `include` pede,
/// se pede algo.
pub fn files(root: &Path, include: &str, exclude: &str, forms: &[PathBuf]) -> Vec<PathBuf> {
    let (include, mut exclude) = (patterns(include), patterns(exclude));
    exclude.extend(gitignore(root));
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            let path = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            let rel = path.strip_prefix(root).map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();
            if exclude.iter().any(|p| glob_matches(p, &rel)) {
                continue;
            }
            if path.is_dir() {
                if !SKIP.contains(&name.as_str()) {
                    stack.push(path);
                }
                continue;
            }
            // O `code-sync.json` é da própria IDE (o que falta portar entre as linguagens): um nome
            // de componente nele não é um uso (aparecia na pesquisa depois de um renomear).
            if forms.contains(&path) || name == aylis_project::sync::FILE || e.metadata().is_ok_and(|m| m.len() > MAX_FILE) {
                continue;
            }
            if !include.is_empty() && !include.iter().any(|p| glob_matches(p, &rel)) {
                continue;
            }
            out.push(path);
        }
    }
    out.sort();
    out
}

/// Procura nos fontes e nos forms. Erro: a expressão não é válida.
pub fn search(options: &SearchOptions, files: &[PathBuf], forms: &[(String, PathBuf)]) -> Result<Results, String> {
    search_with(options, files, forms, &std::collections::HashMap::new())
}

/// Como [`search`], com o texto dos forms abertos na IDE (`open`, pelo arquivo): o que está na
/// memória vale, não o disco (depois de um renomear, o form não salvo ainda tinha o nome antigo
/// no disco, e a pesquisa mostrava achados que não existiam; conferência de 08/11).
pub fn search_with(options: &SearchOptions, files: &[PathBuf], forms: &[(String, PathBuf)], open: &std::collections::HashMap<PathBuf, String>) -> Result<Results, String> {
    options.find_all("")?; // a expressão vale?
    let mut results = Results { files: files.len(), ..Default::default() };
    for file in files {
        let Ok(bytes) = std::fs::read(file) else { continue };
        // Binário (um NUL no começo): não é fonte.
        if bytes.iter().take(8000).any(|&b| b == 0) {
            continue;
        }
        let Ok(text) = String::from_utf8(bytes) else { continue };
        let found = options.find_all(&text)?;
        if found.is_empty() {
            continue;
        }
        // A linha de cada ocorrência, andando junto.
        let mut line = 0;
        let mut at = 0;
        for f in found {
            line += text[at..f.byte_start].matches('\n').count();
            at = f.byte_start;
            let start = text[..f.byte_start].rfind('\n').map_or(0, |i| i + 1);
            let end = text[f.byte_start..].find('\n').map_or(text.len(), |i| f.byte_start + i);
            results.text.push(TextHit { file: file.clone(), line, text: text[start..end].trim_end_matches('\r').to_string(), byte_start: f.byte_start, byte_end: f.byte_end, replace_with: None });
        }
    }
    for (form, file) in forms {
        let Some(text) = open.get(file).cloned().or_else(|| std::fs::read_to_string(file).ok()) else { continue };
        let Ok(json) = serde_json::from_str::<Value>(&text) else { continue };
        walk_form(&json["root"], form, file, options, &mut results.forms);
    }
    Ok(results)
}

/// Os componentes do form e o que cada um tem que casa (o nome, os textos e os eventos).
fn walk_form(node: &Value, form: &str, file: &Path, options: &SearchOptions, out: &mut Vec<FormHit>) {
    let Some(map) = node.as_object() else { return };
    let component = map.get("name").and_then(Value::as_str).unwrap_or(form).to_string();
    for (key, value) in map {
        if key == "children" || key == "class" {
            continue;
        }
        let shown = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            _ => continue,
        };
        if options.find_all(&shown).is_ok_and(|f| !f.is_empty()) {
            out.push(FormHit { form: form.to_string(), file: file.to_path_buf(), component: component.clone(), property: key.clone(), value: shown });
        }
    }
    if let Some(children) = map.get("children").and_then(Value::as_array) {
        for child in children {
            walk_form(child, form, file, options, out);
        }
    }
}

/// A janela da pesquisa (uma só; Ctrl+Shift+F de novo traz ela para a frente).
struct Window {
    form: ComponentId,
    query: ComponentId,
    replace: ComponentId,
    case: ComponentId,
    whole: ComponentId,
    regex: ComponentId,
    keep_case: ComponentId,
    include: ComponentId,
    exclude: ComponentId,
    status: ComponentId,
    tree: ComponentId,
}

#[derive(Default)]
struct Session {
    window: Option<Window>,
    results: Results,
    options: SearchOptions,
    /// Os resultados desmarcados (☐): `t3` (fonte) e `f2` (form).
    unchecked: std::collections::BTreeSet<String>,
    /// A procura mais nova: uma que termina depois de outra começar é descartada.
    generation: u64,
    /// O conteúdo dos arquivos fechados antes da última substituição (o "Desfazer").
    backup: Vec<(PathBuf, String)>,
}

thread_local! {
    static SESSION: RefCell<Session> = RefCell::new(Session::default());
}

fn checked(ui: &Ui, id: ComponentId) -> bool {
    ui.get_prop(id, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false)
}

fn text_of(ui: &Ui, id: ComponentId) -> String {
    ui.get::<Edit>(id).map(|e| e.text.clone()).unwrap_or_default()
}

/// Abre a pesquisa (ou traz para a frente); `initial` vai para o campo (a seleção do editor).
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids, initial: &str) {
    let existing = SESSION.with(|s| s.borrow().window.as_ref().map(|w| (w.form, w.query)));
    if let Some((form, query)) = existing.filter(|(f, _)| ui.is_open(*f)) {
        if !initial.is_empty() {
            let _ = ui.set_prop(query, "text", initial);
        }
        let _ = ui.show(form);
        ui.set_focus(query);
        return;
    }
    let form = ui.add_root(Form::new("Pesquisar no projeto", 760.0, 600.0));
    ui.control_mut(form).unwrap().set_padding(12.0);
    let row = |ui: &mut Ui| {
        let r = ui.add(Layout::default(), form);
        ui.control_mut(r).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 6.0]);
        r
    };
    let check = |ui: &mut Ui, parent: ComponentId, text: &str, width: f32, hint: &str| {
        let c = ui.add(CheckBox::new(text), parent);
        ui.control_mut(c).unwrap().set_align(Align::Right).set_width(width);
        let _ = ui.set_prop(c, "hint", hint);
        c
    };
    let edit = |ui: &mut Ui, parent: ComponentId, placeholder: &str| {
        let e = ui.add(Edit::new(""), parent);
        ui.control_mut(e).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(e, "placeholder", placeholder);
        e
    };
    let r1 = row(ui);
    let regex = check(ui, r1, ".*", 52.0, "Expressão regular");
    let whole = check(ui, r1, "Palavra inteira", 130.0, "");
    let case = check(ui, r1, "Aa", 56.0, "Diferenciar maiúsculas");
    let query = edit(ui, r1, "Pesquisar (Enter)");
    let r2 = row(ui);
    let undo = ui.add(Button::new("Desfazer a substituição"), r2);
    ui.control_mut(undo).unwrap().set_align(Align::Right).set_width(180.0).set_style("flat").set_margins([6.0, 0.0, 0.0, 0.0]);
    let apply = ui.add(Button::new("Substituir marcados"), r2);
    ui.control_mut(apply).unwrap().set_align(Align::Right).set_width(160.0).set_margins([6.0, 0.0, 0.0, 0.0]);
    let keep_case = check(ui, r2, "AB", 56.0, "Preservar maiúsculas");
    let replace = edit(ui, r2, "Substituir por (com .*: $1 é o primeiro grupo)");
    let r3 = row(ui);
    let include = edit(ui, r3, "Incluir: *.rs, src/**");
    let exclude = ui.add(Edit::new(""), r3);
    ui.control_mut(exclude).unwrap().set_align(Align::Right).set_width(340.0).set_margins([6.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(exclude, "placeholder", "Excluir: testes/**, *.lock");
    let r4 = row(ui);
    let toggle_all = ui.add(Button::new("Marcar todos"), r4);
    ui.control_mut(toggle_all).unwrap().set_align(Align::Right).set_width(130.0).set_style("flat");
    let status = ui.add(Label::new("Digite e pressione Enter. Espaço marca ou desmarca um resultado."), r4);
    ui.control_mut(status).unwrap().set_align(Align::Client);
    muted(ui, status);
    let tree = ui.add(TreeView::default(), form);
    ui.control_mut(tree).unwrap().set_align(Align::Client);

    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        s.window = Some(Window { form, query, replace, case, whole, regex, keep_case, include, exclude, status, tree });
        s.results = Results::default();
        s.unchecked.clear();
    });

    let ids = *ids;
    for e in [query, include, exclude] {
        let (state, ids) = (state.clone(), ids);
        ui.on(e, "on_key_down", move |ui, _, args| {
            if let EventArgs::Key(k) = args
                && k.key == Key::Enter
            {
                run(ui, &state, &ids);
            }
        });
    }
    for c in [case, whole, regex] {
        let (state, ids) = (state.clone(), ids);
        ui.on(c, "on_change", move |ui, _, _| run(ui, &state, &ids));
    }
    {
        let (state, ids) = (state.clone(), ids);
        ui.on(tree, "on_change", move |ui, _, _| go_to_selected(ui, &state, &ids));
    }
    // O clique (também num resultado já escolhido) leva ao lugar; o duplo clique, também, e
    // traz a IDE para a frente, por cima desta janela.
    {
        let (state, ids) = (state.clone(), ids);
        ui.on(tree, "on_click", move |ui, _, _| go_to_selected(ui, &state, &ids));
    }
    {
        let (state, ids) = (state.clone(), ids);
        ui.on(tree, "on_dbl_click", move |ui, _, _| {
            go_to_selected(ui, &state, &ids);
            ui.window_command(ids.form, aylis_core::WindowCommand::Show);
        });
    }
    ui.on(tree, "on_key_down", move |ui, _, args| {
        if let EventArgs::Key(k) = args
            && k.key == Key::Space
        {
            toggle_selected(ui);
        }
    });
    ui.on_click(toggle_all, |ui, _| {
        SESSION.with(|s| {
            let mut s = s.borrow_mut();
            if s.unchecked.is_empty() {
                let all: Vec<String> = (0..s.results.text.len()).map(|i| format!("t{i}")).chain((0..s.results.forms.len()).map(|i| format!("f{i}"))).collect();
                s.unchecked.extend(all);
            } else {
                s.unchecked.clear();
            }
        });
        show(ui);
    });
    {
        let (state, ids) = (state.clone(), ids);
        ui.on_click(apply, move |ui, _| replace_checked(ui, &state, &ids));
    }
    {
        let (state, ids) = (state.clone(), ids);
        ui.on_click(undo, move |ui, _| undo_replace(ui, &state, &ids));
    }
    crate::lang::translate(ui);
    let _ = ui.show(form);
    if !initial.is_empty() {
        let _ = ui.set_prop(query, "text", initial);
        run(ui, state, &ids);
    }
    ui.set_focus(query);
}

/// Mostra um resultado já pronto para substituir (o renomear do item 19): a busca `options`
/// e o substituto `with` nos campos, as ocorrências marcadas e o aviso `status`. O
/// "Substituir" aplica as marcadas; desmarcar tira uma da troca (a prévia).
pub fn show_replace(ui: &mut Ui, state: &Shared, ids: &Ids, options: SearchOptions, with: &str, results: Results, status: &str) {
    open(ui, state, ids, "");
    let Some(w) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| (w.query, w.replace, w.case, w.whole, w.regex, w.keep_case, w.status))) else { return };
    let (query, replace, case, whole, regex, keep_case, label) = w;
    let _ = ui.set_prop(query, "text", options.query.clone());
    let _ = ui.set_prop(replace, "text", with);
    let _ = ui.set_prop(case, "checked", options.case);
    let _ = ui.set_prop(whole, "checked", options.whole);
    let _ = ui.set_prop(regex, "checked", options.regex);
    let _ = ui.set_prop(keep_case, "checked", options.preserve_case);
    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        s.generation += 1;
        s.options = options;
        s.results = results;
        s.unchecked.clear();
    });
    show(ui);
    let _ = ui.set_prop(label, "text", status);
}

/// "Onde é usado" (item 20): procura os usos de `target` numa thread e mostra na janela da
/// pesquisa, com o nome no campo de busca (palavra inteira, para o Substituir valer).
pub fn show_usages(ui: &mut Ui, state: &Shared, ids: &Ids, target: crate::usages::Target) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para ver onde o componente é usado.".into();
        return;
    };
    open(ui, state, ids, "");
    let Some((query, whole, case, status)) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| (w.query, w.whole, w.case, w.status))) else { return };
    let name = match &target {
        crate::usages::Target::Component { name, .. } | crate::usages::Target::Handler { name } => name.clone(),
        crate::usages::Target::Field { field, .. } => field.clone(),
    };
    let _ = ui.set_prop(query, "text", name);
    let _ = ui.set_prop(whole, "checked", true);
    let _ = ui.set_prop(case, "checked", !matches!(target, crate::usages::Target::Field { .. }));
    let _ = ui.set_prop(status, "text", format!("Procurando os usos de {}…", target.label()));
    let generation = SESSION.with(|s| {
        let mut s = s.borrow_mut();
        s.generation += 1;
        s.generation
    });
    let poster = ui.poster();
    std::thread::spawn(move || {
        let result = crate::usages::find(&project, &target);
        poster.post(move |ui| {
            if SESSION.with(|s| s.borrow().generation != generation) {
                return;
            }
            match result {
                Ok(results) => {
                    SESSION.with(|s| {
                        let mut s = s.borrow_mut();
                        s.results = results;
                        s.unchecked.clear();
                    });
                    show(ui);
                }
                Err(e) => {
                    let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
                }
            }
        });
    });
}

/// Começa a procura numa thread; o resultado volta pela fila da interface.
fn run(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    let Some((options, include, exclude, status)) = SESSION.with(|s| {
        let s = s.borrow();
        let w = s.window.as_ref()?;
        let options = SearchOptions { query: text_of(ui, w.query), case: checked(ui, w.case), whole: checked(ui, w.whole), regex: checked(ui, w.regex), preserve_case: checked(ui, w.keep_case) };
        Some((options, text_of(ui, w.include), text_of(ui, w.exclude), w.status))
    }) else { return };
    let Some(project) = state.borrow().project.clone() else {
        let _ = ui.set_prop(status, "text", "Abra um projeto para pesquisar nele.");
        return;
    };
    if options.query.is_empty() {
        return;
    }
    if let Err(e) = options.find_all("") {
        let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
        return;
    }
    let root = project.dir().to_path_buf();
    let forms: Vec<(String, PathBuf)> = project.forms.iter().map(|f| (f.name.clone(), project.file(&f.file))).collect();
    // Os forms abertos e alterados: o texto da memória.
    let open: std::collections::HashMap<PathBuf, String> = {
        let st = state.borrow();
        std::iter::once(&st.doc)
            .chain(st.tabs.iter().flatten())
            .filter(|d| d.modified)
            .filter_map(|d| Some((d.path.clone()?, d.to_json().ok()?)))
            .collect()
    };
    let generation = SESSION.with(|s| {
        let mut s = s.borrow_mut();
        s.generation += 1;
        s.options = options.clone();
        s.generation
    });
    let _ = ui.set_prop(status, "text", "Pesquisando…");
    let poster = ui.poster();
    std::thread::spawn(move || {
        let form_files: Vec<PathBuf> = forms.iter().map(|(_, f)| f.clone()).collect();
        let list = files(&root, &include, &exclude, &form_files);
        let result = search_with(&options, &list, &forms, &open);
        poster.post(move |ui| {
            let current = SESSION.with(|s| s.borrow().generation == generation);
            if !current {
                return;
            }
            match result {
                Ok(results) => {
                    SESSION.with(|s| {
                        let mut s = s.borrow_mut();
                        s.results = results;
                        s.unchecked.clear();
                    });
                    show(ui);
                }
                Err(e) => {
                    let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
                }
            }
        });
    });
}

/// Monta a árvore com os resultados (por arquivo; os forms por form) e o resumo.
fn show(ui: &mut Ui) {
    let Some((tree, status)) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| (w.tree, w.status))) else { return };
    let (nodes, summary) = SESSION.with(|s| {
        let s = s.borrow();
        let mark = |key: &str| if s.unchecked.contains(key) { "☐ " } else { "☑ " };
        let mut nodes: Vec<TreeNode> = Vec::new();
        let mut current: Option<(PathBuf, Vec<TreeNode>)> = None;
        let flush = |current: &mut Option<(PathBuf, Vec<TreeNode>)>, nodes: &mut Vec<TreeNode>| {
            if let Some((file, children)) = current.take() {
                let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let mut node = TreeNode::new(format!("{name}  ({})", children.len())).with_children(children);
                node.expanded = true;
                node.data = format!("file:{}", file.display());
                nodes.push(node);
            }
        };
        for (i, hit) in s.results.text.iter().enumerate() {
            if current.as_ref().is_none_or(|(f, _)| *f != hit.file) {
                flush(&mut current, &mut nodes);
                current = Some((hit.file.clone(), Vec::new()));
            }
            let key = format!("t{i}");
            let mut node = TreeNode::new(format!("{}{}:  {}", mark(&key), hit.line + 1, hit.text.trim()));
            node.data = key;
            current.as_mut().unwrap().1.push(node);
        }
        flush(&mut current, &mut nodes);
        if !s.results.forms.is_empty() {
            let mut children = Vec::new();
            for (i, hit) in s.results.forms.iter().enumerate() {
                let key = format!("f{i}");
                let mut node = TreeNode::new(format!("{}{} › {}.{} = {}", mark(&key), hit.form, hit.component, hit.property, hit.value));
                node.data = key;
                children.push(node);
            }
            let mut forms = TreeNode::new(format!("Forms  ({})", children.len())).with_children(children);
            forms.expanded = true;
            nodes.push(forms);
        }
        let files: std::collections::BTreeSet<&PathBuf> = s.results.text.iter().map(|h| &h.file).collect();
        let summary = if s.results.text.is_empty() && s.results.forms.is_empty() {
            format!("Nada encontrado em {} arquivos e nos forms.", s.results.files)
        } else {
            format!("{} no código ({} arquivos) e {} nos forms. Espaço marca ou desmarca.", s.results.text.len(), files.len(), s.results.forms.len())
        };
        (nodes, summary)
    });
    if let Some(t) = ui.get_mut::<TreeView>(tree) {
        t.set_nodes(nodes);
    }
    let _ = ui.set_prop(status, "text", summary);
    ui.request_repaint();
}

/// O resultado escolhido na árvore: abre o código na linha ou o form com o componente.
fn go_to_selected(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(tree) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| w.tree)) else { return };
    let Some(data) = ui.get::<TreeView>(tree).and_then(|t| t.selected().map(|n| n.data.clone())) else { return };
    let hit = SESSION.with(|s| {
        let s = s.borrow();
        let index = |prefix: char| data.strip_prefix(prefix).and_then(|i| i.parse::<usize>().ok());
        (index('t').and_then(|i| s.results.text.get(i).cloned()), index('f').and_then(|i| s.results.forms.get(i).cloned()))
    });
    match hit {
        (Some(t), _) => {
            let _ = crate::codewindow::open(ui, &t.file, t.line + 1);
        }
        (_, Some(f)) => {
            crate::actions::open_project_form(state, &f.form);
            {
                let mut st = state.borrow_mut();
                let root = st.doc.root;
                let id = st.doc.ui.find_in(root, &f.component);
                st.doc.select(id.or(Some(root)));
                // A propriedade em destaque no inspector (e a aba certa, se for evento).
                st.events_tab = f.property.starts_with("on_");
                st.inspector_filter = if f.property == "name" { String::new() } else { f.property.clone() };
            }
            crate::app::refresh(ui, state, ids);
        }
        _ => {}
    }
}

/// O espaço na árvore: marca ou desmarca o resultado escolhido (ou todos os do arquivo).
fn toggle_selected(ui: &mut Ui) {
    let Some(tree) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| w.tree)) else { return };
    let Some(node) = ui.get::<TreeView>(tree).and_then(|t| t.selected().cloned()) else { return };
    let keys: Vec<String> = if node.children.is_empty() { vec![node.data.clone()] } else { node.children.iter().map(|c| c.data.clone()).collect() };
    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        let all_checked = keys.iter().all(|k| !s.unchecked.contains(k));
        for k in keys {
            if all_checked {
                s.unchecked.insert(k);
            } else {
                s.unchecked.remove(&k);
            }
        }
    });
    show(ui);
}

/// Substitui os marcados: nos fontes abertos pelo editor (Ctrl+Z desfaz lá), nos fechados no
/// arquivo (com cópia para o "Desfazer a substituição") e nos forms pelo designer.
fn replace_checked(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(replace) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| w.replace)) else { return };
    let template = text_of(ui, replace);
    let (results, unchecked, options) = SESSION.with(|s| {
        let s = s.borrow();
        (s.results.clone(), s.unchecked.clone(), s.options.clone())
    });
    let (count, backup) = apply(ui, state, &options, &template, &results, &unchecked);
    SESSION.with(|s| s.borrow_mut().backup = backup);
    crate::app::refresh(ui, state, ids);
    if let Some(status) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| w.status)) {
        let _ = ui.set_prop(status, "text", format!("{count} substituições. Os arquivos abertos e os forms desfazem com Ctrl+Z; os fechados, com \"Desfazer a substituição\"."));
    }
    run(ui, state, ids);
}

/// Troca os achados (menos os de `unchecked`): nos fontes abertos pelo editor (o Ctrl+Z desfaz
/// lá), nos fechados no arquivo, e nos forms pelo designer. Devolve quantas trocou e o texto de
/// antes dos arquivos fechados (para o "Desfazer a substituição"). O renomear chama direto
/// (sem a janela da pesquisa) quando o servidor de linguagem achou os usos.
pub fn apply(ui: &mut Ui, state: &Shared, options: &SearchOptions, template: &str, results: &Results, unchecked: &std::collections::BTreeSet<String>) -> (usize, Vec<(PathBuf, String)>) {
    // Os fontes, um arquivo de cada vez (as ocorrências de trás para a frente).
    let mut by_file: std::collections::BTreeMap<PathBuf, Vec<&TextHit>> = Default::default();
    for (i, hit) in results.text.iter().enumerate() {
        if !unchecked.contains(&format!("t{i}")) {
            by_file.entry(hit.file.clone()).or_default().push(hit);
        }
    }
    let mut backup = Vec::new();
    let mut count = 0;
    for (file, hits) in by_file {
        let Ok(disk) = std::fs::read_to_string(&file) else { continue };
        // Aberto num editor, a troca parte do texto dele (com o que ainda não foi salvo): partindo
        // do disco, um arquivo alterado por um renomear anterior ficava sem a troca, em silêncio
        // (o segundo renomear deixou `self.cdsItens` no código; conferência de 08/11).
        let editor = crate::codewindow::text_of(ui, &file);
        let text = editor.clone().unwrap_or_else(|| disk.clone());
        let found = options.find_all(&text).unwrap_or_default();
        // Os achados por texto vêm do disco: no editor, a mesma ocorrência pela ordem.
        let on_disk = if text == disk { Vec::new() } else { options.find_all(&disk).unwrap_or_default() };
        let edits: Vec<(usize, usize, String)> = hits
            .iter()
            .filter_map(|h| match &h.replace_with {
                // A troca do servidor de linguagem: o trecho exato que ele deu (no texto do editor,
                // de onde ele leu).
                Some(with) => text.get(h.byte_start..h.byte_end).is_some().then(|| (h.byte_start, h.byte_end, with.clone())),
                None if on_disk.is_empty() => found.iter().find(|f| f.byte_start == h.byte_start && f.byte_end == h.byte_end).map(|f| (f.byte_start, f.byte_end, options.replacement(&text, f, template))),
                None => {
                    let k = on_disk.iter().position(|f| f.byte_start == h.byte_start && f.byte_end == h.byte_end)?;
                    found.get(k).map(|f| (f.byte_start, f.byte_end, options.replacement(&text, f, template)))
                }
            })
            .collect();
        count += edits.len();
        if editor.is_some() && crate::codewindow::apply_to_open_file(ui, &file, &text, &edits) {
            continue;
        }
        let text = disk;
        let mut out = text.clone();
        for (a, b, with) in edits.iter().rev() {
            out.replace_range(*a..*b, with);
        }
        if std::fs::write(&file, &out).is_ok() {
            backup.push((file, text));
        }
    }
    // Os forms: pelo designer (o Ctrl+Z dele desfaz).
    for (i, hit) in results.forms.iter().enumerate() {
        if unchecked.contains(&format!("f{i}")) {
            continue;
        }
        let (new_value, _) = options.replace_all(&hit.value, template).unwrap_or((hit.value.clone(), 0));
        crate::actions::open_project_form(state, &hit.form);
        let mut st = state.borrow_mut();
        let Some(id) = hit_target(&st.doc.ui, st.doc.root, &hit.component) else { continue };
        st.doc.begin_change();
        let value = match st.doc.ui.get_prop(id, &hit.property) {
            Ok(aylis_core::PropValue::Int(_)) => new_value.parse::<i64>().map(Value::from).unwrap_or(Value::from(new_value.clone())),
            Ok(aylis_core::PropValue::Float(_)) => new_value.parse::<f64>().map(Value::from).unwrap_or(Value::from(new_value.clone())),
            Ok(aylis_core::PropValue::Bool(_)) => new_value.parse::<bool>().map(Value::from).unwrap_or(Value::from(new_value.clone())),
            _ => Value::from(new_value.clone()),
        };
        let done = if hit.property == "name" { st.doc.rename_component(id, &new_value) } else { st.doc.set_prop(id, &hit.property, &value) };
        if done.is_ok() {
            count += 1;
        }
    }
    (count, backup)
}

/// O componente de um achado no form aberto: a raiz só quando o achado é dela (o nome dela).
/// Um componente que não está mais no form (o achado veio do disco, e o form na memória já
/// o renomeou) não troca nada: cair na raiz renomeava o form junto (conferência de 08/11).
fn hit_target(ui: &Ui, root: ComponentId, component: &str) -> Option<ComponentId> {
    if ui.name(root) == Some(component) {
        return Some(root);
    }
    ui.find_in(root, component)
}

/// Volta os arquivos fechados ao que eram antes da última substituição.
fn undo_replace(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let backup = SESSION.with(|s| std::mem::take(&mut s.borrow_mut().backup));
    let n = backup.iter().filter(|(file, text)| std::fs::write(file, text).is_ok()).count();
    if let Some(status) = SESSION.with(|s| s.borrow().window.as_ref().map(|w| w.status)) {
        let _ = ui.set_prop(status, "text", format!("{n} arquivos voltaram ao que eram."));
    }
    run(ui, state, ids);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Renomear `cdsPedidos` → `cdsVendas` e substituir de novo: o achado do disco
    /// (`cdsPedidos`, que o form na memória já não tem) não pode cair na raiz.
    #[test]
    fn a_hit_of_a_component_that_is_gone_does_not_touch_the_form() {
        let mut ui = Ui::new();
        let root = ui.add_root(Layout::default());
        ui.set_name(root, "frmPrincipal").unwrap();
        let cds = ui.add(Layout::default(), root);
        ui.set_name(cds, "cdsVendas").unwrap();
        assert_eq!(hit_target(&ui, root, "frmPrincipal"), Some(root));
        assert_eq!(hit_target(&ui, root, "cdsVendas"), Some(cds));
        assert_eq!(hit_target(&ui, root, "cdsPedidos"), None, "o renomeado não cai na raiz");
    }

    /// O form aberto vale pela memória (o disco ainda tem o nome antigo antes de salvar), e o
    /// `code-sync.json` da IDE não entra na pesquisa.
    #[test]
    fn open_forms_are_searched_in_memory_and_the_sync_file_is_skipped() {
        let dir = std::env::temp_dir().join(format!("aylis-pesquisa-memoria-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("forms")).unwrap();
        let form = dir.join("forms/principal.json");
        std::fs::write(&form, r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal", "children": [ { "class": "ClientDataSet", "name": "cdsPedidos" } ] } }"#).unwrap();
        std::fs::write(dir.join("code-sync.json"), r#"{ "components": ["cdsPedidos"] }"#).unwrap();
        let options = SearchOptions { query: "cdsPedidos".into(), case: true, whole: true, regex: false, preserve_case: false };
        let forms = vec![("frmPrincipal".to_string(), form.clone())];
        let list = files(&dir, "", "", &[form.clone()]);
        assert!(list.iter().all(|f| !f.ends_with("code-sync.json")), "{list:?}");
        assert_eq!(search(&options, &list, &forms).unwrap().forms.len(), 1, "do disco");
        let open = std::collections::HashMap::from([(form.clone(), r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal", "children": [ { "class": "ClientDataSet", "name": "cdsVendas" } ] } }"#.to_string())]);
        assert!(search_with(&options, &list, &forms, &open).unwrap().forms.is_empty(), "da memória: já renomeado");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn glob_patterns() {
        assert!(glob_matches("*.rs", "src/main.rs"));
        assert!(!glob_matches("*.rs", "src/main.ts"));
        assert!(glob_matches("src/**", "src/forms/a.rs"));
        assert!(glob_matches("src/**/*.rs", "src/forms/a.rs"));
        assert!(glob_matches("src/**/*.rs", "src/a.rs"));
        assert!(glob_matches("target", "rust/target/debug/x"));
        assert!(glob_matches("testes/", "testes/a.rs"));
        assert!(!glob_matches("src/**/*.rs", "lib/a.rs"));
    }

    /// No código (fora `target` e o `.gitignore`) e nos forms: o nome, o texto e o handler.
    #[test]
    fn searches_sources_and_forms() {
        let dir = std::env::temp_dir().join(format!("aylis-pesquisa-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("target")).unwrap();
        std::fs::create_dir_all(dir.join("forms")).unwrap();
        std::fs::create_dir_all(dir.join("gerado")).unwrap();
        std::fs::write(dir.join("src").join("main.rs"), "fn salvar() {}\n// Salvar tudo\nfn outro() {}\n").unwrap();
        std::fs::write(dir.join("target").join("x.rs"), "salvar").unwrap();
        std::fs::write(dir.join("gerado").join("y.rs"), "salvar").unwrap();
        std::fs::write(dir.join(".gitignore"), "gerado/\n").unwrap();
        let form = dir.join("forms").join("principal.json");
        std::fs::write(&form, r#"{"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Button","name":"btnSalvar","text":"Salvar","on_click":"salvar_click"}]}}"#).unwrap();
        let list = files(&dir, "", "", std::slice::from_ref(&form));
        // O `.gitignore` também é texto do projeto (como no VS Code); `target` e `gerado/`, não.
        assert_eq!(list, vec![dir.join(".gitignore"), dir.join("src").join("main.rs")]);
        let r = search(&SearchOptions::new("salvar"), &list, &[("frmPrincipal".into(), form.clone())]).unwrap();
        assert_eq!(r.text.iter().map(|h| h.line).collect::<Vec<_>>(), vec![0, 1]);
        let mut props: Vec<&str> = r.forms.iter().map(|h| h.property.as_str()).collect();
        props.sort_unstable();
        assert_eq!(props, vec!["name", "on_click", "text"]);
        assert!(files(&dir, "*.ts", "", &[]).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
