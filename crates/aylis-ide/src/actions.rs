//! Os comandos da IDE (menus, barra de ferramentas, atalhos).

use std::future::Future;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Wake};

use aylis_core::{ComponentId, ModalResult, PropValue, Ui};
use aylis_project::{Language, Project, codegen};
use serde_json::Value;

use crate::app::{Ids, refresh};
use crate::dialogs::{self, FieldSpec};
use crate::document::{AlignOp, Document};
use crate::state::Shared;

// ===========================================================================
// Tarefas (diálogos do sistema) e a bomba de mensagens
// ===========================================================================

/// Roda `task` na thread da interface, sem travá-la: os diálogos do sistema são
/// assíncronos (no Windows o rfd abre cada um em outra thread). Um diálogo modal aberto
/// dentro do clique trava a janela: ela não consegue se redesenhar enquanto ele está
/// aberto, e o Windows repete o pedido de pintura sem parar (100% de CPU).
///
/// Um diálogo por vez: com outro aberto, o pedido é ignorado.
pub fn spawn(state: &Shared, task: impl Future<Output = ()> + 'static) {
    let mut st = state.borrow_mut();
    if !st.tasks.is_empty() {
        st.status = "Já há um diálogo aberto".into();
        return;
    }
    st.tasks.push(Box::pin(task));
    if let Some(waker) = &st.waker {
        waker.wake_by_ref();
    }
}

/// Acorda o loop do aylis (que então chama a bomba e avança as tarefas).
struct Wakeup(aylis_platform_winit::Waker);

impl Wake for Wakeup {
    fn wake(self: Arc<Self>) {
        self.0.wake();
    }
}

/// A bomba de mensagens da IDE: avança as tarefas, recebe a saída do programa em
/// execução e, quando algo muda, refaz os painéis. Entregue a
/// [`aylis_platform_winit::run_with_pump`].
pub fn pump(state: Shared, ids: Rc<Ids>, waker: aylis_platform_winit::Waker) -> aylis_platform_winit::Pump {
    let aylis_waker = waker.clone();
    let waker = std::task::Waker::from(Arc::new(Wakeup(waker)));
    state.borrow_mut().waker = Some(waker.clone());
    state.borrow_mut().output_waker = Some(aylis_waker);
    // O refresh refaz os painéis da IDE: com um trabalho mandando linhas (compilar, o app
    // rodando), uma vez por linha deixava a IDE presa nele (a rolagem engasgava). No
    // máximo a cada REFRESH_EVERY; o que chega no meio espera a próxima.
    // O relógio do `Ui` (no navegador não há o `Instant` do Rust).
    const REFRESH_EVERY: f64 = 0.12;
    let last = Rc::new(std::cell::Cell::new(None::<f64>));
    let pending = Rc::new(std::cell::Cell::new(false));
    Box::new(move |ui| {
        let mut changed = false;
        // Fora do `borrow`: as tarefas mexem no estado quando terminam.
        let mut tasks = std::mem::take(&mut state.borrow_mut().tasks);
        if !tasks.is_empty() {
            let before = tasks.len();
            let mut cx = Context::from_waker(&waker);
            tasks.retain_mut(|task| task.as_mut().poll(&mut cx).is_pending());
            changed |= tasks.len() < before;
            let mut st = state.borrow_mut();
            tasks.append(&mut st.tasks);
            st.tasks = tasks;
        }
        let after = state.borrow_mut().after_task.take();
        if let Some(f) = after {
            f(ui);
            changed = true;
        }
        // Compilar, depurar, os aparelhos e o servidor de linguagem: só onde a IDE faz isso.
        if crate::lite::can(crate::lite::Capability::Build) {
            changed |= crate::jobs::drain(ui, &state, &ids);
            changed |= crate::debugger::poll(ui, &state);
            changed |= crate::devices::poll(&state);
            // O servidor de linguagem só mexe no editor (sugestões, erros, dicas): basta
            // repintar, sem refazer os painéis da IDE (o que custava uns 20 ms a cada
            // resposta, ou seja, a cada tecla).
            if crate::intel::poll(ui, &state) {
                ui.request_repaint();
            }
        }
        if !changed || pending.get() {
            return;
        }
        let now = ui.time();
        let wait = last.get().map_or(0.0, |t| (REFRESH_EVERY - (now - t)).max(0.0));
        if wait <= 0.0 {
            last.set(Some(now));
            refresh(ui, &state, &ids);
            return;
        }
        pending.set(true);
        let (state, ids, last, pending) = (state.clone(), ids.clone(), last.clone(), pending.clone());
        ui.set_timeout((wait * 1000.0).ceil() as u64, move |ui| {
            pending.set(false);
            last.set(Some(ui.time()));
            refresh(ui, &state, &ids);
        });
    })
}

/// Pergunta sim/não com a caixa de mensagem do aylis: um componente `MessageDialog` no form
/// da IDE, liberado ao fechar. `on_yes` roda só com "Sim"; depois a IDE se redesenha.
pub fn confirm(ui: &mut Ui, state: &Shared, ids: &Ids, question: &str, on_yes: impl FnOnce(&mut Ui) + 'static) {
    let dialog = aylis_controls::MessageDialog::new(ui.tr(question), aylis_controls::DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No])
        .with_title("Aylis IDE");
    let id = ui.add(dialog, ids.form);
    let (state, ids) = (state.clone(), *ids);
    aylis_controls::MessageDialog::execute_then(ui, id, move |ui, result| {
        ui.free(id);
        if result == ModalResult::Yes {
            on_yes(ui);
            refresh(ui, &state, &ids);
        }
    });
}

// ===========================================================================
// Configuração (recentes, janela e painéis)
// ===========================================================================

/// Uma configuração à parte (os recentes, o layout) em `dir`, em vez da de quem usa a IDE: o
/// gerador da documentação abre a IDE sem janela para desenhar as telas dos tutoriais e não
/// pode mexer na lista de recentes de ninguém.
pub fn use_private_config(dir: &Path) {
    let _ = PRIVATE_CONFIG.set(dir.join("config.json"));
}

static PRIVATE_CONFIG: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

fn config_path() -> Option<PathBuf> {
    if let Some(path) = PRIVATE_CONFIG.get() {
        return Some(path.clone());
    }
    // Os testes (o executável deles fica em `target/.../deps`) não mexem na configuração de
    // quem usa a IDE (os recentes, o layout): cada execução de teste tem a sua.
    let exe = std::env::current_exe().ok();
    if exe.as_ref().and_then(|e| e.parent()).and_then(|d| d.file_name()).is_some_and(|n| n == "deps") {
        return Some(std::env::temp_dir().join(format!("aylis-ide-test-{}", std::process::id())).join("config.json"));
    }
    let base = std::env::var_os("APPDATA").or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config").into_os_string()))?;
    Some(PathBuf::from(base).join("aylis-ide").join("config.json"))
}

/// Lê os arquivos recentes guardados.
pub fn load_config(state: &Shared) {
    let Some(path) = config_path() else { return };
    let Ok(text) = std::fs::read_to_string(path) else { return };
    let Ok(value) = serde_json::from_str::<Value>(&text) else { return };
    if let Some(list) = value["recent"].as_array() {
        state.borrow_mut().recent = list.iter().filter_map(Value::as_str).map(PathBuf::from).filter(|p| p.exists()).collect();
    }
    if let Some(language) = value["language"].as_str() {
        state.borrow_mut().language = language.to_string();
    }
}

/// Liga ou desliga o servidor MCP (agentes de IA comandando a IDE); `remember` guarda a
/// escolha para a próxima vez (o menu guarda; `AYLIS_IDE_MCP` não). `port` 0: a padrão.
pub fn set_mcp(ui: &mut Ui, state: &Shared, on: bool, port: u16, remember: bool) {
    let mut st = state.borrow_mut();
    if let Some(server) = st.mcp.take() {
        server.stop();
    }
    if on {
        // A porta e o token de %LOCALAPPDATA%\aylis\mcp.json (o token não aparece na tela).
        let started = aylis_tools::mcp::access().and_then(|access| {
            let port = if port == 0 { access.port } else { port };
            // Outra IDE já usa a porta: esta fica numa livre (a ponte `aylis-tool mcp` acha pelo
            // registro das IDEs abertas).
            crate::mcp::start(ui.poster(), port, access.token.clone()).or_else(|_| crate::mcp::start(ui.poster(), 0, access.token))
        });
        match started {
            Ok(server) => {
                crate::instance::set_mcp_port(Some(server.port()));
                st.status = format!("Servidor MCP em {} · para conectar um agente: Ferramentas → Opções da IDE → MCP", server.url());
                st.mcp = Some(server);
            }
            Err(e) => st.status = format!("⚠ {e}"),
        }
    } else {
        crate::instance::set_mcp_port(None);
        st.status = "Servidor MCP desligado".into();
    }
    if remember {
        update_config("mcp", Value::from(st.mcp.is_some()));
    }
    drop(st);
    install_edit_guard(state);
}

/// Com o MCP ligado e um projeto aberto, o gancho que impede o agente de gravar direto nos
/// arquivos que a IDE guarda (`edit_guard`). Gravado no `.claude/settings.local.json` do
/// projeto, só quando falta ou mudou (a IDE noutro lugar).
pub fn install_edit_guard(state: &Shared) {
    #[cfg(feature = "desktop")]
    {
        let (dir, on) = {
            let st = state.borrow();
            (st.project.as_ref().map(|p| p.dir()), st.mcp.is_some())
        };
        let (Some(dir), true, Some(command)) = (dir, on, crate::edit_guard::command()) else { return };
        if let Err(e) = crate::edit_guard::install(&dir, &command) {
            state.borrow_mut().status = format!("⚠ gancho do agente: {e}");
        }
    }
    #[cfg(not(feature = "desktop"))]
    let _ = state;
}

/// O servidor MCP estava ligado da última vez?
pub fn mcp_was_on() -> bool {
    let Some(path) = config_path() else { return false };
    std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).is_some_and(|v| v["mcp"] == true)
}

/// Troca o idioma da IDE (`""`: o do sistema) e guarda a escolha.
pub fn set_language(ui: &mut Ui, state: &Shared, code: &str) {
    state.borrow_mut().language = code.to_string();
    update_config("language", Value::from(code));
    crate::lang::apply(ui, crate::lang::resolve(code));
}

fn save_config(state: &Shared) {
    let recent: Vec<String> = state.borrow().recent.iter().map(|p| p.display().to_string()).collect();
    update_config("recent", serde_json::json!(recent));
}

/// Uma opção liga/desliga do arquivo de configuração (`default` se não houver).
/// Um texto do arquivo de configuração (vazio se não houver).
pub fn config_text(key: &str) -> String {
    let Some(path) = config_path() else { return String::new() };
    std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v[key].as_str().map(str::to_string)).unwrap_or_default()
}

pub fn config_flag(key: &str, default: bool) -> bool {
    let Some(path) = config_path() else { return default };
    std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v[key].as_bool()).unwrap_or(default)
}

/// Uma chave do arquivo de configuração (`Null` se não houver).
pub(crate) fn config_value(key: &str) -> Value {
    let Some(path) = config_path() else { return Value::Null };
    std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|mut v| v[key].take()).unwrap_or(Value::Null)
}

/// Troca uma chave do arquivo de configuração, mantendo as outras.
/// As pastas da árvore do projeto que o usuário abriu (`project_trees` da configuração, por
/// projeto). Sem nada guardado, tudo fechado.
pub fn project_tree_of(project: &Path) -> std::collections::HashSet<String> {
    let Some(path) = config_path() else { return Default::default() };
    let config = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).unwrap_or_default();
    config["project_trees"][project.display().to_string()].as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

/// Guarda as pastas abertas da árvore do projeto `project`.
pub fn remember_project_tree(project: &Path, open: &std::collections::HashSet<String>) {
    let Some(path) = config_path() else { return };
    let mut trees = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|c| c["project_trees"].clone()).filter(Value::is_object).unwrap_or_else(|| serde_json::json!({}));
    let mut list: Vec<&String> = open.iter().collect();
    list.sort();
    trees[project.display().to_string()] = serde_json::json!(list);
    update_config("project_trees", trees);
}

pub(crate) fn update_config(key: &str, value: Value) {
    let Some(path) = config_path() else { return };
    let mut config = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).filter(Value::is_object).unwrap_or_else(|| serde_json::json!({}));
    config[key] = value;
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, serde_json::to_string_pretty(&config).unwrap_or_default());
}

/// Os painéis que o usuário redimensiona: (nome, controle, usa a largura).
fn layout_panels(ui: &Ui, ids: &Ids) -> Vec<(&'static str, ComponentId, bool)> {
    let mut out = Vec::new();
    // Nomes novos desde que o inspector foi para a esquerda (os tamanhos antigos não servem).
    if let Some(left) = ui.parent(ids.inspector) {
        out.push(("inspector_side", left, true));
    }
    if let Some(right) = ui.parent(ids.palette) {
        out.push(("palette_side", right, true));
    }
    out.push(("structure", ids.structure, false));
    out.push(("project", ids.project, false));
    out.push(("messages", ids.messages_panel, false));
    out
}

/// A janela da IDE e o tamanho dos painéis, para guardar.
pub fn layout_json(ui: &Ui, ids: &Ids) -> Value {
    let panels: serde_json::Map<String, Value> =
        layout_panels(ui, ids).into_iter().filter_map(|(name, id, width)| ui.control(id).map(|cb| (name.to_string(), Value::from(if width { cb.width() } else { cb.height() } as f64)))).collect();
    let mut out = serde_json::json!({ "panels": panels });
    if let Some(p) = ui.window_placement(ids.form) {
        out["window"] = serde_json::json!({ "x": p.x, "y": p.y, "width": p.width, "height": p.height, "maximized": p.maximized });
    }
    out
}

/// Devolve a janela e os painéis como estavam (de [`layout_json`]). Tamanhos absurdos
/// (de outra tela) são ignorados.
pub fn apply_layout(ui: &mut Ui, ids: &Ids, layout: &Value) {
    if let Some(w) = layout.get("window") {
        let int = |k: &str| w.get(k).and_then(Value::as_i64);
        if let (Some(x), Some(y), Some(width), Some(height)) = (int("x"), int("y"), int("width"), int("height")) {
            let maximized = w.get("maximized").and_then(Value::as_bool).unwrap_or(false);
            let p = aylis_core::WindowPlacement { x: x as i32, y: y as i32, width: width.clamp(0, 16384) as u32, height: height.clamp(0, 16384) as u32, maximized };
            ui.set_window_placement(ids.form, p);
        }
    }
    let Some(panels) = layout.get("panels") else { return };
    for (name, id, width) in layout_panels(ui, ids) {
        let Some(size) = panels.get(name).and_then(Value::as_f64).filter(|s| (40.0..4000.0).contains(s)) else { continue };
        if let Some(cb) = ui.control_mut(id) {
            if width {
                cb.set_width(size as f32);
            } else {
                cb.set_height(size as f32);
            }
        }
    }
    ui.invalidate();
}

/// Lê a janela e os painéis guardados e aplica (antes de abrir a janela).
pub fn restore_layout(ui: &mut Ui, ids: &Ids) {
    let Some(path) = config_path() else { return };
    let Some(config) = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else { return };
    if let Some(layout) = config.get("layout") {
        apply_layout(ui, ids, layout);
    }
}

/// Guarda a janela e os painéis (ao fechar a IDE).
pub fn save_layout(ui: &Ui, ids: &Ids) {
    update_config("layout", layout_json(ui, ids));
}

// ===========================================================================
// Projeto
// ===========================================================================

/// Os temas próprios do projeto (`themes/*.json`, com a mistura por classe dele): o claro e
/// o escuro de uma família que não é do aylis, para o "Form claro / Form escuro".
/// O tema já guardado igual a `theme`, ou ele mesmo guardado: as abas do mesmo projeto ficam com
/// um tema só (cada um tem ~0,2 MB; com os 1009 forms do ERP abertos, eram 200 MB de cópias).
/// Um tema mudado (o editor de temas) é outro, e entra no lugar do mais antigo.
fn shared_theme(theme: aylis_core::Theme) -> std::sync::Arc<aylis_core::Theme> {
    thread_local! {
        static THEMES: std::cell::RefCell<Vec<std::sync::Arc<aylis_core::Theme>>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    THEMES.with(|themes| {
        let mut themes = themes.borrow_mut();
        if let Some(t) = themes.iter().find(|t| ***t == theme) {
            return t.clone();
        }
        if themes.len() >= 4 {
            themes.remove(0);
        }
        let shared = std::sync::Arc::new(theme);
        themes.push(shared.clone());
        shared
    })
}

fn project_themes(project: &Project) -> Vec<aylis_core::Theme> {
    let files = aylis_project::Files::Dir(project.dir());
    files
        .list("themes/")
        .into_iter()
        .filter(|f| f.ends_with(".json"))
        .filter_map(|file| {
            let mut p = project.clone();
            p.theme = file;
            aylis_project::resolve_theme(&p, &files).ok()
        })
        .collect()
}

/// Regrava os forms do projeto no formato que a IDE salva (a ordem e o formato fixos do
/// item 22), sem mudar o conteúdo: o que abrir e salvar daria. Para passar uma vez nos forms
/// escritos à mão ou por outra ferramenta; depois, abrir e salvar não muda nada. Devolve os
/// arquivos regravados (`aylis-ide --format-forms projeto.aylisproj`).
pub fn format_forms(project: &Project) -> Result<Vec<PathBuf>, String> {
    let mut changed = Vec::new();
    for form in &project.forms {
        let path = project.file(&form.file);
        let original = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let doc = project_document(project, &form.name, None).map_err(|e| format!("{}: {e}", path.display()))?;
        let saved = doc.to_json()?;
        if saved != original.replace("\r\n", "\n") {
            std::fs::write(&path, saved).map_err(|e| format!("{}: {e}", path.display()))?;
            changed.push(path);
        }
    }
    Ok(changed)
}

/// Documento de um form do projeto (com acesso às imagens de `assets/`).
pub fn project_document(project: &Project, form: &str, variant: Option<&str>) -> Result<Document, String> {
    let entry = project.form(form).ok_or_else(|| format!("o projeto não tem o form '{form}'"))?;
    let path = project.file(&variant.map_or_else(|| entry.file.clone(), |v| aylis_project::variant_file(&entry.file, v)));
    let theme = aylis_project::resolve_theme(project, &aylis_project::Files::Dir(project.dir())).ok().map(shared_theme);
    // Os frames mostram o form deles, lido do arquivo do projeto.
    let dir = project.clone();
    let frames: crate::document::FrameSource = std::rc::Rc::new(move |name: &str| {
        // Um pacote: o form dele e o manifesto (o designer aplica os valores expostos).
        if let Some(package) = aylis_project::packages::package_of(name) {
            let files = aylis_project::Files::Dir(dir.dir());
            let manifest = aylis_project::packages::manifest(&files, package)?;
            let mut layout = aylis_project::packages::form_layout(&files, package, package).ok()?;
            layout["manifest"] = serde_json::to_value(manifest).ok()?;
            return Some(layout);
        }
        let entry = dir.form(name)?;
        serde_json::from_str(&std::fs::read_to_string(dir.file(&entry.file)).ok()?).ok()
    });
    let mut doc = Document::open_with_modules(&path, theme.clone(), Some(frames), |uses| module_layouts(project, uses))?;
    doc.palette = project.palette();
    doc.project_themes = project_themes(project);
    doc.ui.services_mut().resources = Some(Box::new(crate::assetdialogs::IdeResources(project.dir())));
    // Fontes de assets/fonts e os ícones de pacote que o projeto cita.
    aylis_project::install_assets(&mut doc.ui, &aylis_project::Files::Dir(project.dir()));
    aylis_project::install_font_names(&mut doc.ui, project);
    crate::assetdialogs::register_used_icons(&mut doc.ui, project, Some(&path));
    // O designer mostra o form com a aparência do projeto.
    if let Some(theme) = theme {
        doc.set_shared_theme(theme);
    }
    // O aparelho do projeto (o modelo Smartphone vem com um): o form já abre nele, com a
    // moldura.
    if let Some(device) = aylis_core::devices::parse(&project.device) {
        doc.show_device(Some(device));
    }
    // O designer precisa mostrar as imagens: recarrega o layout já com o acesso aos arquivos.
    doc.ui.request_repaint();
    Ok(doc)
}

/// O aparelho escolhido no designer vai para o projeto (`device`): o próximo form e a próxima
/// vez que o projeto abrir mostram nele, e o Executar abre com a moldura dele. Só grava quando
/// muda.
pub fn sync_project_device(state: &Shared) {
    let mut st = state.borrow_mut();
    if st.no_form {
        return;
    }
    let now = st.doc.device.map_or(String::new(), |(d, landscape)| if landscape { format!("{};deitado", d.name) } else { d.name.to_string() });
    let Some(project) = st.project.as_mut() else { return };
    if project.device != now {
        project.device = now;
        let _ = project.save();
    }
}

pub fn new_project(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let ids = *ids;
    let start = state.borrow().project.as_ref().and_then(|p| p.dir().parent().map(Path::to_path_buf)).or_else(|| {
        std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(|h| PathBuf::from(h).join("Documents"))
    });
    let start = start.unwrap_or_else(|| PathBuf::from("."));
    let state2 = state.clone();
    crate::wizard::new_project(ui, state, &ids, start, move |ui, name, dir, language, mut options| {
        // O idioma do app começa como o da IDE (o escolhido em Exibir → Idioma, ou o do sistema).
        options.locale = crate::lang::app_locale(crate::lang::resolve(&state2.borrow().language));
        let created = aylis_project::new_project::create_with(&dir, &name, language, &options);
        match created {
            Ok(project) => {
                // Criado aqui: confiável sem perguntar.
                let _ = aylis_tools::trust::trust(&project.dir());
                open_project(ui, &state2, &ids, &project.path)
            }
            Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
        }
        refresh(ui, &state2, &ids);
    });
}

pub fn open_project_dialog(state: &Shared, ids: &Ids) {
    let (state2, ids) = (state.clone(), *ids);
    spawn(state, async move {
        let file = crate::files::AsyncFileDialog::new().add_filter("Projeto do Aylis", &[aylis_project::EXTENSION]).pick_file().await;
        if let Some(file) = file {
            let path = file.path().to_path_buf();
            let state3 = state2.clone();
            state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| open_project(ui, &state3, &ids, &path)));
        }
    });
}

/// Abre um projeto: fecha as abas, abre o form principal.
pub fn open_project(ui: &mut Ui, state: &Shared, ids: &Ids, path: &Path) {
    // O caminho sem "..": os pontos de parada e as janelas de código comparam por ele.
    let path = &crate::debugger::norm(path);
    let project = match Project::open(path) {
        Ok(p) => p,
        // De um aylis anterior: a IDE não abre (só o próprio formato), mas oferece converter
        // com o aylis-migrate (que faz uma cópia antes) e abre em seguida.
        Err(e) if e.contains(aylis_project::MIGRATE_HINT) => {
            state.borrow_mut().status = format!("⚠ {e}");
            let (state2, ids2, path2) = (state.clone(), *ids, path.to_path_buf());
            confirm(ui, state, ids, "O projeto é de uma versão anterior do Aylis. Converter agora? (uma cópia é feita antes)", move |ui| {
                match crate::jobs::aylis_migrate(&path2) {
                    Ok(report) => {
                        state2.borrow_mut().status = report.lines().next().unwrap_or_default().to_string();
                        open_project(ui, &state2, &ids2, &path2);
                    }
                    Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
                }
            });
            return;
        }
        Err(e) => {
            state.borrow_mut().status = format!("⚠ {e}");
            return;
        }
    };
    // Outro projeto: o que era do anterior sai (as abas de código, as mensagens, o servidor de
    // linguagem); o mesmo projeto de novo fica como está.
    let other = state.borrow().project.as_ref().is_some_and(|p| !aylis_tools::instances::same_path(&p.path, path));
    if other && !leave_project(ui, state) {
        return;
    }
    crate::host::prefetch_gpu_adapters();
    let main = project.main_form.clone();
    let doc = project_document(&project, &main, None);
    let mut st = state.borrow_mut();
    // O tema do projeto (o do app rodando) manda; num tema próprio, fica a escolha de antes.
    let dark = match project.theme.as_str() {
        "Aylis Dark" => true,
        "Aylis Light" => false,
        _ => st.doc.dark,
    };
    st.close_all();
    match doc {
        Ok(mut doc) => {
            // Um tema próprio (`themes/noobs-dark.json`) já diz se é claro ou escuro: o form
            // abre como o app roda (antes, a escolha anterior da IDE trocava pelo Aylis Light).
            let dark = if project.theme.ends_with(".json") { doc.theme.dark } else { dark };
            doc.set_dark(dark);
            st.doc = doc;
        }
        Err(e) => st.status = format!("⚠ {e}"),
    }
    st.status = format!("Projeto {} aberto ({})", project.name, project.language.name());
    st.remember(path.to_path_buf());
    let dir = project.dir();
    // A árvore do projeto como o usuário deixou (I10 do Tabernáculo).
    st.project_tree_open = project_tree_of(&project.path);
    st.project = Some(project);
    drop(st);
    save_config(state);
    install_edit_guard(state);
    // Um projeto de fora pergunta antes de rodar qualquer coisa dele.
    crate::trust::check(ui, state, ids, &dir);
}

/// Antes de fechar ou trocar de projeto: fecha as abas de código, limpa as MENSAGENS e para o
/// servidor de linguagem do projeto que sai (ficavam do anterior, até um `.py` num projeto em
/// Rust e o rust-analyzer do ERP rodando com o Relatórios aberto; conferência de 08/11).
/// `false`: há código não salvo (o aviso foi dado), e o projeto fica.
pub fn leave_project(ui: &mut Ui, state: &Shared) -> bool {
    // Forms alterados: avisa uma vez, como com o código (antes eram descartados sem perguntar;
    // o relatório do Tabernáculo, I07).
    {
        let mut st = state.borrow_mut();
        if st.any_modified() && !st.leave_warned {
            st.leave_warned = true;
            st.status = "Há forms alterados sem salvar: Ctrl+Shift+S salva todos; repita para descartar".into();
            return false;
        }
    }
    if !crate::codewindow::close_all_tabs(ui) {
        state.borrow_mut().status = "Há código alterado sem salvar: Ctrl+S salva; repita para descartar".into();
        return false;
    }
    state.borrow_mut().leave_warned = false;
    crate::intel::stop(state);
    let mut st = state.borrow_mut();
    st.output.clear();
    st.code_view = false;
    true
}

/// Fecha o projeto. `false`: há algo alterado sem salvar (o aviso está na barra de estado).
pub fn close_project(ui: &mut Ui, state: &Shared) -> bool {
    if !leave_project(ui, state) {
        return false;
    }
    let mut st = state.borrow_mut();
    st.project = None;
    st.restricted = false;
    st.close_all();
    st.home = true;
    st.status = "Projeto fechado".into();
    true
}

/// Abre um form do projeto (ou vai para a aba dele).
pub fn open_project_form(state: &Shared, form: &str) {
    open_project_variant(state, form, None);
}

/// Abre a variante `variant` do form (`None`: o layout padrão), cada uma na sua aba.
pub fn open_project_variant(state: &Shared, form: &str, variant: Option<&str>) {
    let mut st = state.borrow_mut();
    // Abrir um form mostra o designer (o código embutido fica nas abas dele).
    st.code_view = false;
    let Some(project) = st.project.clone() else { return };
    let path = project.form(form).map(|e| project.file(&variant.map_or_else(|| e.file.clone(), |v| aylis_project::variant_file(&e.file, v))));
    if let Some(i) = path.as_ref().and_then(|p| st.find_tab_path(p)) {
        st.switch_to(i);
        return;
    }
    match project_document(&project, form, variant) {
        Ok(mut doc) => {
            doc.set_dark(st.doc.dark);
            st.open_tab(doc);
        }
        Err(e) => st.status = format!("⚠ {e}"),
    }
}

/// Projeto → Nova variante do form: copia o layout padrão do form aberto para a variante
/// de uma plataforma (`principal.android.json`), que depois fica com o visual que quiser.
pub fn new_variant(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (project, form) = {
        let st = state.borrow();
        (st.project.clone(), st.doc.form_name())
    };
    let Some(project) = project else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    if project.form(&form).is_none() || project.is_data_module(&form) {
        state.borrow_mut().status = "Abra um form do projeto".into();
        return;
    }
    let existing = project.variants_of(&form);
    let free: Vec<&str> = aylis_project::VARIANTS.iter().copied().filter(|v| !existing.iter().any(|e| e == v)).collect();
    if free.is_empty() {
        state.borrow_mut().status = format!("{form} já tem todas as variantes");
        return;
    }
    let dialog = dialogs::new_dialog(ui, &format!("Nova variante de {form}"), 420.0, 190.0);
    dialogs::field(ui, dialog.body, "Plataforma");
    let combo = dialogs::combo(ui, dialog.body, &free, free[0]);
    let (state2, ids) = (state.clone(), *ids);
    dialogs::run(ui, dialog, |_| Ok(()), move |ui| {
        let variant = dialogs::combo_text(ui, combo);
        match project.add_variant(&form, &variant) {
            Ok(file) => {
                open_project_variant(&state2, &form, Some(&variant));
                state2.borrow_mut().status = format!("Variante criada: {file}");
            }
            Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
        }
        refresh(ui, &state2, &ids);
    });
}

/// Os layouts dos DataModules `uses` do projeto (os que não existem ficam de fora).
fn module_layouts(project: &Project, uses: &[String]) -> Vec<Value> {
    uses.iter()
        .filter_map(|name| project.form(name))
        .filter_map(|entry| std::fs::read_to_string(project.file(&entry.file)).ok())
        .filter_map(|text| serde_json::from_str::<Value>(&text).ok())
        .filter(|v| v["root"]["class"] == "DataModule")
        .collect()
}

/// Projeto → Novo DataModule: um contêiner de componentes não visuais, criado no início.
/// Move ou renomeia o arquivo de um form do projeto (`aylis_project::Project::move_form`): a aba
/// aberta dele passa a gravar no lugar novo, e o código é gerado de novo (o módulo pode mudar).
/// Salva o form antes, se ele tem alterações.
pub fn move_form(state: &Shared, name: &str, new_file: &str) -> Result<String, String> {
    let index = state.borrow().find_tab(name);
    if index.is_some_and(|i| state.borrow().tab(i).is_some_and(|d| d.modified)) {
        if let Some(i) = index {
            state.borrow_mut().switch_to(i);
        }
        save(state, false);
    }
    let mut st = state.borrow_mut();
    let project = st.project.as_mut().ok_or("abra um projeto antes")?;
    project.move_form(name, new_file)?;
    let path = project.file(&project.form(name).map(|f| f.file.clone()).unwrap_or_default());
    let project = project.clone();
    if let Some(i) = st.find_tab(name) {
        if i == st.active {
            st.doc.path = Some(path.clone());
        } else if let Some(Some(doc)) = st.tabs.get_mut(i) {
            doc.path = Some(path.clone());
        }
    }
    drop(st);
    codegen::generate(&project)?;
    Ok(format!("{name} agora em {}", path.display()))
}

/// O caminho de um fonte novo dentro da pasta do código (`src/nucleo.rs`): relativo, sem `..`,
/// fora do `generated/`, e que ainda não existe.
pub fn new_source_path(project: &aylis_project::Project, relative: &str) -> Result<PathBuf, String> {
    let relative = relative.trim().replace('\\', "/");
    let path = Path::new(&relative);
    if relative.is_empty() || path.is_absolute() || path.components().any(|c| !matches!(c, std::path::Component::Normal(_))) {
        return Err("use um caminho dentro da pasta do código, sem .. (ex.: src/nucleo.rs)".into());
    }
    if relative.split('/').next() == Some("generated") {
        return Err("generated/ é do gerador (somente leitura)".into());
    }
    let full = project.code_dir().join(path);
    if full.exists() {
        return Err(format!("{relative} já existe"));
    }
    Ok(full)
}

/// Cria um fonte novo na pasta do código, com `text` (relatório do RTSPlayer, 10/11: não havia
/// como criar um arquivo por assunto). Em Rust, o `mod` dele no `lib.rs` fica com o usuário: o
/// código dele não é mexido pela IDE.
pub fn new_source(project: &aylis_project::Project, relative: &str, text: &str) -> Result<PathBuf, String> {
    let full = new_source_path(project, relative)?;
    if let Some(dir) = full.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("não consegui criar {}: {e}", dir.display()))?;
    }
    std::fs::write(&full, text).map_err(|e| format!("não consegui gravar {}: {e}", full.display()))?;
    Ok(full)
}

/// Projeto → Novo fonte…: pede o caminho e abre o arquivo criado no editor.
pub fn new_source_dialog(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let suggestion = match project.language {
        Language::Rust => "src/novo.rs",
        Language::Python => "src/novo.py",
        _ => "novo",
    };
    let (check, state2) = (project.clone(), state.clone());
    dialogs::input_text(
        ui,
        "Novo fonte",
        "Caminho (na pasta do código)",
        suggestion,
        move |name| new_source_path(&check, name).map(|_| ()),
        move |ui, name| match new_source(&project, &name, "") {
            Ok(file) => {
                if let Err(e) = crate::codewindow::open(ui, &file, 1) {
                    state2.borrow_mut().status = format!("⚠ {e}");
                } else {
                    state2.borrow_mut().status = if project.language == Language::Rust {
                        format!("Fonte criado: {} (declare o mod dele no lib.rs)", name)
                    } else {
                        format!("Fonte criado: {}", name)
                    };
                }
            }
            Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
        },
    );
}

pub fn new_data_module(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let existing: Vec<String> = project.forms.iter().map(|f| f.name.clone()).collect();
    let suggestion = std::iter::once("dmDados".to_string())
        .chain((2..).map(|n| format!("dmDados{n}")))
        .find(|n| !existing.iter().any(|e| e.eq_ignore_ascii_case(n)))
        .unwrap_or_default();
    let (state2, ids) = (state.clone(), *ids);
    dialogs::input_text(
        ui,
        "Novo DataModule",
        "Nome do DataModule",
        &suggestion,
        move |name| {
            if !aylis_project::is_identifier(name) {
                return Err("use letras, números e _ (começando com letra)".into());
            }
            if existing.iter().any(|e| e.eq_ignore_ascii_case(name)) {
                return Err(format!("o projeto já tem um form ou DataModule '{name}'"));
            }
            Ok(())
        },
        move |ui, name| {
            let result = {
                let mut st = state2.borrow_mut();
                // O projeto pode ter sido fechado com o diálogo aberto (na IDE ou pelo MCP).
                match st.project.as_mut() {
                    Some(project) => project.add_data_module(&name).and_then(|_| project.save()),
                    None => Err("o projeto foi fechado".into()),
                }
            };
            match result {
                Ok(()) => {
                    open_project_form(&state2, &name);
                    state2.borrow_mut().status = format!("DataModule {name} criado");
                }
                Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
            }
            refresh(ui, &state2, &ids);
        },
    );
}

/// Os DataModules que o form aberto pode usar e os que ele usa. `Err`: a mensagem para o
/// status (sem projeto, o aberto é um DataModule, o projeto não tem nenhum).
pub fn uses_choices(state: &Shared) -> Result<(Vec<String>, Vec<String>), String> {
    let st = state.borrow();
    let project = st.project.as_ref().ok_or("Abra um projeto primeiro")?;
    if st.doc.is_data_module() {
        return Err("Um DataModule não usa outros: abra um form".into());
    }
    let modules: Vec<String> = project.forms.iter().map(|f| f.name.clone()).filter(|n| project.is_data_module(n)).collect();
    if modules.is_empty() {
        return Err("O projeto não tem DataModules (Projeto → Novo DataModule…)".into());
    }
    Ok((modules, st.doc.uses.clone()))
}

/// Troca os DataModules que o form aberto usa (recarrega o form com eles).
pub fn set_uses(state: &Shared, uses: Vec<String>) {
    let mut st = state.borrow_mut();
    let Some(project) = st.project.clone() else { return };
    let form = st.doc.form_name();
    let layouts = module_layouts(&project, &uses);
    st.doc.begin_change();
    st.status = match st.doc.set_modules(uses.clone(), layouts) {
        Ok(()) if uses.is_empty() => format!("{form} não usa DataModules"),
        Ok(()) => format!("{form} usa {}", uses.join(", ")),
        Err(e) => format!("⚠ {e}"),
    };
}

/// Projeto → Opções do projeto → DataModules usados: marca os DataModules que o form aberto usa (`uses`).
pub fn edit_uses(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (modules, current) = match uses_choices(state) {
        Ok(c) => c,
        Err(e) => {
            state.borrow_mut().status = e;
            return;
        }
    };
    let form = state.borrow().doc.form_name();
    let dialog = dialogs::new_dialog(ui, &format!("DataModules usados por {form}"), 420.0, 120.0 + 34.0 * modules.len() as f32);
    let mut checks = Vec::new();
    for name in &modules {
        let c = ui.add(aylis_controls::CheckBox::new(name), dialog.body);
        ui.control_mut(c).unwrap().set_align(aylis_core::Align::Top).set_height(30.0);
        let _ = ui.set_prop(c, "checked", current.iter().any(|u| u.eq_ignore_ascii_case(name)));
        checks.push((c, name.clone()));
    }
    let (state2, ids) = (state.clone(), *ids);
    dialogs::run(ui, dialog, |_| Ok(()), move |ui| {
        let uses: Vec<String> =
            checks.iter().filter(|(c, _)| matches!(ui.get_prop(*c, "checked"), Ok(aylis_core::PropValue::Bool(true)))).map(|(_, n)| n.clone()).collect();
        set_uses(&state2, uses);
        refresh(ui, &state2, &ids);
    });
}

pub fn new_form(ui: &mut Ui, state: &Shared, ids: &Ids) {
    if state.borrow().project.is_none() {
        // Saindo da tela inicial: o form vazio dela já serve.
        if crate::home::visible(state) {
            state.borrow_mut().home = false;
            return;
        }
        let mut st = state.borrow_mut();
        let dark = st.doc.dark;
        let mut doc = Document::new();
        doc.set_dark(dark);
        st.open_tab(doc);
        return;
    }
    let existing: Vec<String> = state.borrow().project.as_ref().map(|p| p.forms.iter().map(|f| f.name.clone()).collect()).unwrap_or_default();
    let suggestion = (2..).map(|n| format!("frmForm{n}")).find(|n| !existing.iter().any(|e| e.eq_ignore_ascii_case(n))).unwrap_or_default();
    let (state2, ids) = (state.clone(), *ids);
    dialogs::input_text(
        ui,
        "Novo form",
        "Nome do form",
        &suggestion,
        move |name| {
            if !aylis_project::is_identifier(name) {
                return Err("use letras, números e _ (começando com letra)".into());
            }
            if existing.iter().any(|e| e.eq_ignore_ascii_case(name)) {
                return Err(format!("o projeto já tem um form '{name}'"));
            }
            Ok(())
        },
        move |ui, name| {
            let result = {
                let mut st = state2.borrow_mut();
                // O projeto pode ter sido fechado com o diálogo aberto (na IDE ou pelo MCP).
                match st.project.as_mut() {
                    Some(project) => project.add_form(&name, &name).and_then(|_| project.save()),
                    None => Err("o projeto foi fechado".into()),
                }
            };
            match result {
                Ok(()) => {
                    open_project_form(&state2, &name);
                    state2.borrow_mut().status = format!("Form {name} criado");
                }
                Err(e) => state2.borrow_mut().status = format!("⚠ {e}"),
            }
            refresh(ui, &state2, &ids);
        },
    );
}

pub fn add_existing_form(state: &Shared, ids: &Ids) {
    if state.borrow().project.is_none() {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    }
    let (state2, _ids) = (state.clone(), *ids);
    spawn(state, async move {
        let Some(file) = crate::files::AsyncFileDialog::new().add_filter("Layout do Aylis", &["json"]).pick_file().await else { return };
        let mut st = state2.borrow_mut();
        // O projeto pode ter sido fechado enquanto o diálogo de arquivo estava aberto.
        let Some(project) = st.project.as_mut() else {
            st.status = "⚠ O projeto foi fechado".into();
            return;
        };
        let result = project.add_existing_form(file.path()).and_then(|entry| project.save().map(|_| entry));
        match result {
            Ok(entry) => {
                st.status = format!("Form {} adicionado", entry.name);
                drop(st);
                open_project_form(&state2, &entry.name);
            }
            Err(e) => st.status = format!("⚠ {e}"),
        }
    });
}

pub fn remove_form(ui: &mut Ui, state: &Shared, ids: &Ids, form: &str) {
    let form = form.to_string();
    let state2 = state.clone();
    let question = format!("Tirar o form {form} do projeto? (o arquivo continua no disco)");
    confirm(ui, state, ids, &question, move |_| {
        let mut st = state2.borrow_mut();
        let result = st.project.as_mut().map(|p| p.remove_form(&form).and_then(|_| p.save()));
        match result {
            Some(Ok(())) => {
                if let Some(i) = st.find_tab(&form) {
                    st.close_tab(i);
                }
                st.status = format!("Form {form} removido do projeto");
            }
            Some(Err(e)) => st.status = format!("⚠ {e}"),
            None => {}
        }
    });
}

/// Abre a pasta do projeto atual no gerenciador de arquivos do sistema.
pub fn open_project_folder(state: &Shared) {
    let dir = state.borrow().project.as_ref().map(|p| p.dir().to_path_buf());
    let mut st = state.borrow_mut();
    st.status = match dir {
        Some(dir) => match crate::jobs::open_folder(&dir) {
            Ok(()) => format!("Pasta aberta: {}", dir.display()),
            Err(e) => format!("⚠ {e}"),
        },
        None => "Nenhum projeto aberto".to_string(),
    };
}

pub fn set_main_form(state: &Shared, form: &str) {
    let mut st = state.borrow_mut();
    let Some(project) = st.project.as_mut() else { return };
    project.main_form = form.to_string();
    if let Some(entry) = project.forms.iter_mut().find(|f| f.name == form) {
        entry.auto_create = true;
    }
    st.status = match project.save() {
        Ok(()) => format!("{form} é o form principal"),
        Err(e) => format!("⚠ {e}"),
    };
}

pub fn toggle_auto_create(state: &Shared, form: &str) {
    let mut st = state.borrow_mut();
    let Some(project) = st.project.as_mut() else { return };
    if project.main_form == form {
        st.status = "O form principal é sempre criado no início".into();
        return;
    }
    if let Some(entry) = project.forms.iter_mut().find(|f| f.name == form) {
        entry.auto_create = !entry.auto_create;
    }
    if let Err(e) = project.save() {
        st.status = format!("⚠ {e}");
    }
}

pub fn project_options(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let (state2, ids) = (state.clone(), *ids);
    let (old_language, old_render) = (project.language, project.render());
    let ide_locale = crate::lang::app_locale(crate::lang::resolve(&state.borrow().language));
    dialogs::project_options(ui, &project, &ide_locale, move |ui, updated| {
        let render_changed = updated.render() != old_render;
        if updated.language == old_language {
            save_project_options(ui, &state2, &ids, updated, false, render_changed);
            return;
        }
        // Cada linguagem tem a pasta dela: voltar para uma que já existe é só trocar.
        if updated.code_dir().is_dir() {
            save_project_options(ui, &state2, &ids, updated, true, render_changed);
            return;
        }
        // Linguagem nova: o código não é traduzido, os handlers nascem vazios. Confirma antes.
        let text = format!(
            "Trocar a linguagem do projeto de {old} para {new}?\n\nO projeto ainda não tem código em {new}: a pasta {folder}/ é criada com os handlers vazios, e o app não faz nada (os botões não respondem) até você escrever a lógica neles. O visual é o mesmo, e o código em {old} continua na pasta {old_folder}/, intacto: voltar para {old} traz tudo de volta.\n\nProjeto → Pendências entre linguagens mostra o que falta portar.",
            old = old_language.name(),
            new = updated.language.name(),
            folder = updated.language.folder(),
            old_folder = old_language.folder()
        );
        let state3 = state2.clone();
        aylis_controls::message_dlg(ui, &text, aylis_controls::DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], move |ui, result| {
            if result == ModalResult::Yes {
                save_project_options(ui, &state3, &ids, updated.clone(), true, render_changed);
            } else {
                state3.borrow_mut().status = "Linguagem mantida; as outras opções não foram salvas".into();
            }
        });
    });
}

fn save_project_options(ui: &mut Ui, state2: &Shared, ids: &Ids, updated: Project, language_changed: bool, render_changed: bool) {
    {
        let mut st = state2.borrow_mut();
        // O renderizador vira features do Cargo.toml (projetos em Rust).
        let saved = updated.save().and_then(|()| {
            if render_changed && updated.language == Language::Rust {
                aylis_project::codegen::sync_cargo(&updated).map(|_| ())
            } else {
                Ok(())
            }
        });
        // Outra linguagem: o código dela em dia com o visual (a pasta nasce, se for nova).
        let saved = saved.and_then(|()| if language_changed { codegen::generate(&updated).map(|_| ()) } else { Ok(()) });
        st.status = match saved {
            Ok(()) if render_changed => "Opções salvas; renderizador atualizado no Cargo.toml (vale na próxima compilação)".into(),
            Ok(()) if language_changed => format!("Linguagem: {} (código em {}/)", updated.language.name(), updated.language.folder()),
            Ok(()) => "Opções do projeto salvas".into(),
            Err(e) => format!("⚠ {e}"),
        };
        st.project = Some(updated);
    }
    refresh(ui, state2, ids);
    // O que mudou nas outras linguagens e falta nesta.
    if language_changed {
        crate::code_sync::open(ui, state2, true);
    }
}

/// O tema do projeto de novo em todos os forms abertos (depois de trocar a fonte, por
/// exemplo).
fn reapply_theme(st: &mut crate::state::State, project: &Project) -> Result<(), String> {
    let theme = aylis_project::resolve_theme(project, &aylis_project::Files::Dir(project.dir()))?;
    let palette = project.palette();
    st.doc.palette = palette.clone();
    st.doc.set_theme(theme.clone());
    for d in st.tabs.iter_mut().flatten() {
        d.palette = palette.clone();
        d.set_theme(theme.clone());
    }
    Ok(())
}

/// "Projeto → Opções do projeto → Fontes": o catálogo offline do Google Fonts; as marcadas vão para
/// `assets/fonts/` (com a licença) e uma delas pode ser a fonte do projeto.
/// Projeto → Opções do projeto → Memória: os limites do cache de imagens (`image_cache`).
pub fn project_memory(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let state2 = state.clone();
    let current = project.clone();
    dialogs::project_memory(ui, &current, move |_, image_cache| {
        let mut updated = project;
        updated.image_cache = image_cache;
        let mut st = state2.borrow_mut();
        match updated.save() {
            Ok(()) => {
                st.status = "Limites de memória salvos (valem na próxima execução do app)".into();
                st.project = Some(updated);
            }
            Err(e) => st.status = format!("Erro ao salvar o projeto: {e}"),
        }
    });
}

pub fn project_fonts(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let (state2, ids) = (state.clone(), *ids);
    let current = project.clone();
    crate::assetdialogs::fonts(ui, &current, move |ui, result| {
        let mut updated = project;
        let before: std::collections::BTreeSet<String> = aylis_project::assets::project_fonts(&updated).into_iter().collect();
        let catalog = aylis_project::assets::font_catalog();
        let mut errors = Vec::new();
        for folder in result.folders.difference(&before) {
            if let Some(entry) = catalog.iter().find(|e| e.folder == *folder) {
                if let Err(e) = aylis_project::assets::add_font(&updated, entry) {
                    errors.push(e);
                }
            }
        }
        for folder in before.difference(&result.folders) {
            if let Err(e) = aylis_project::assets::remove_font(&updated, folder) {
                errors.push(e);
            }
        }
        updated.font = result.font;
        updated.fonts = result.names.iter().cloned().collect();
        // Troca em massa: os arquivos do projeto (forms e temas) e a fonte do projeto...
        let mut replaced = 0;
        for (from, to) in &result.replacements {
            match aylis_project::assets::replace_font(&mut updated, from, to) {
                Ok(n) => replaced += n,
                Err(e) => errors.push(e),
            }
        }
        if let Err(e) = updated.save() {
            errors.push(e);
        }
        // As fontes novas entram no registro (o designer as mostra já).
        let mut st = state2.borrow_mut();
        // ... e os forms abertos no designer (que estão na memória).
        if !result.replacements.is_empty() {
            let st = &mut *st;
            for doc in std::iter::once(&mut st.doc).chain(st.tabs.iter_mut().flatten()) {
                let ids: Vec<ComponentId> = doc.ui.components().collect();
                for id in ids {
                    let current = match doc.ui.get_prop(id, "font_family") {
                        Ok(aylis_core::PropValue::String(f)) => f,
                        _ => continue,
                    };
                    if let Some((_, to)) = result.replacements.iter().find(|(from, _)| *from == current) {
                        let _ = doc.ui.set_prop(id, "font_family", to.as_str());
                    }
                }
            }
        }
        aylis_project::install_assets(&mut st.doc.ui, &aylis_project::Files::Dir(updated.dir()));
        aylis_project::install_font_names(&mut st.doc.ui, &updated);
        for d in st.tabs.iter_mut().flatten() {
            aylis_project::install_font_names(&mut d.ui, &updated);
        }
        if let Err(e) = reapply_theme(&mut st, &updated) {
            errors.push(e);
        }
        st.status = match errors.first() {
            None if replaced > 0 => format!("Fontes: {replaced} lugar(es) trocado(s) · {} no projeto", result.folders.len()),
            None => format!(
                "Fontes: {} no projeto · fonte {}",
                result.folders.len(),
                if updated.font.is_empty() { "do tema" } else { &updated.font }
            ),
            Some(e) => format!("⚠ {e}"),
        };
        st.project = Some(updated);
        drop(st);
        refresh(ui, &state2, &ids);
    });
}

/// "Projeto → Ícones": os pacotes offline (Material Symbols, Fluent, os do aylis e os do
/// app). Com um componente que tem `icon` selecionado, o OK põe o ícone nele.
pub fn project_icons(ui: &mut Ui, state: &Shared, ids: &Ids) {
    if state.borrow().project.is_none() {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    }
    let target = {
        let st = state.borrow();
        st.doc.selection.filter(|id| st.doc.ui.get_prop(*id, "icon").is_ok())
    };
    pick_icon(ui, state, ids, target, "icon");
}

/// O seletor de ícones para a propriedade `prop` (`icon`, `checked_icon`...) de `target`:
/// começa no ícone atual; o OK grava o escolhido ("Sem ícone" limpa). Sem projeto também
/// funciona (os pacotes do aylis, sem o do app).
pub fn pick_icon(ui: &mut Ui, state: &Shared, ids: &Ids, target: Option<ComponentId>, prop: impl Into<String>) {
    let prop: String = prop.into();
    let (project, label, initial) = {
        let st = state.borrow();
        let label = target.map(|id| st.doc.ui.name(id).unwrap_or("componente").to_string());
        let initial = match target.map(|id| st.doc.ui.get_prop(id, &prop)) {
            Some(Ok(PropValue::String(s))) => s,
            _ => String::new(),
        };
        (st.project.clone(), label, initial)
    };
    let (state2, ids) = (state.clone(), *ids);
    let opened = project.clone();
    crate::assetdialogs::icons(ui, opened.as_ref(), label, &initial, move |ui, chosen| {
        let Some(name) = chosen else { return };
        let mut st = state2.borrow_mut();
        if let Some(project) = &project {
            let form = st.doc.path.clone();
            crate::assetdialogs::register_used_icons(&mut st.doc.ui, project, form.as_deref());
        }
        // O ícone pode ser de um pacote ainda não citado: registra-o no designer.
        if let Some((pack, icon)) = name.split_once(':') {
            if st.doc.ui.icon(&name).is_none() {
                if let Some(p) = aylis_project::assets::icon_pack_text(pack).and_then(|t| aylis_core::icons::parse_pack(&t).ok()) {
                    if let Some(i) = p.icon(icon) {
                        st.doc.ui.register_icon(&name, i);
                    }
                }
            }
        }
        st.status = match target {
            Some(id) => {
                st.doc.begin_change();
                match st.doc.set_prop(id, &prop, &Value::String(name.clone())) {
                    Ok(()) => {
                        st.doc.modified = true;
                        if name.is_empty() { "Ícone removido".to_string() } else { format!("Ícone {name} aplicado") }
                    }
                    Err(e) => format!("⚠ {e}"),
                }
            }
            None => format!("Ícone escolhido: {name} (copie o nome ou selecione um componente com ícone)"),
        };
        drop(st);
        refresh(ui, &state2, &ids);
    });
}

/// "Projeto → Animações": os pacotes de animação (Noto do Google). Com um `Lottie`
/// selecionado, o OK põe a animação nele.
pub fn project_animations(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (target, label) = {
        let st = state.borrow();
        let target = st.doc.selection.filter(|id| st.doc.ui.class_name(*id) == Some("Lottie"));
        (target, target.map(|id| st.doc.ui.name(id).unwrap_or("Lottie").to_string()))
    };
    let (state2, ids) = (state.clone(), *ids);
    crate::assetdialogs::animations(ui, label, move |ui, chosen| {
        let Some(name) = chosen else { return };
        let mut st = state2.borrow_mut();
        st.status = match target {
            Some(id) => {
                st.doc.begin_change();
                match st.doc.set_prop(id, "source", &Value::String(name.clone())) {
                    Ok(()) => {
                        st.doc.modified = true;
                        format!("Animação {name} aplicada")
                    }
                    Err(e) => format!("⚠ {e}"),
                }
            }
            None => format!("Animação escolhida: {name} (copie o nome ou selecione um Lottie)"),
        };
        drop(st);
        refresh(ui, &state2, &ids);
    });
}

/// "Projeto → Opções do projeto → Aparência": tema do projeto, mistura por classe e quem usa qual tema.
pub fn project_appearance(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return;
    };
    let (classes, rows) = {
        let st = state.borrow();
        let doc = &st.doc;
        let mut classes: Vec<String> =
            doc.ui.registry().iter().map(|c| c.name.to_string()).filter(|n| n != "Form" && doc.class_is_visual(n)).collect();
        classes.sort();
        let rows = doc
            .tree()
            .into_iter()
            .filter(|(id, _)| doc.is_visual(*id))
            .map(|(id, depth)| {
                let name = doc.ui.name(id).unwrap_or_default().to_string();
                let class = doc.ui.class_name(id).unwrap_or_default();
                let (theme, source) = doc.ui.theme_source(id);
                let source = match source {
                    aylis_core::ThemeSource::App => "do projeto".to_string(),
                    aylis_core::ThemeSource::Own => "próprio".to_string(),
                    aylis_core::ThemeSource::Inherited(p) => format!("herdado de {}", doc.ui.name(p).unwrap_or("?")),
                    aylis_core::ThemeSource::Class(c) => format!("mistura da classe {c}"),
                };
                let own = doc.ui.control(id).map(|c| c.theme_name().to_string()).unwrap_or_default();
                let indent = "   ".repeat(depth);
                dialogs::UsageRow { id, label: format!("{indent}{name}: {class}"), theme, source, own }
            })
            .collect();
        (classes, rows)
    };
    let (state2, ids) = (state.clone(), *ids);
    let original = project.clone();
    dialogs::appearance(ui, &project, classes, rows, move |ui, result| {
        let mut st = state2.borrow_mut();
        let mut updated = original;
        updated.theme = result.theme;
        updated.theme_mix = result.mix;
        let theme = aylis_project::resolve_theme(&updated, &aylis_project::Files::Dir(updated.dir()));
        // No Android e no navegador, o tema novo vai para o app aberto.
        if updated.theme.ends_with(".json") {
            let file = updated.file(&updated.theme);
            if let Some(Err(e)) = push_reload(&st, "theme", "", &file) {
                eprintln!("aylis-ide: tema no app: {e}");
            }
        }
        st.status = match (updated.save(), theme) {
            (Ok(()), Ok(theme)) => {
                let palette = updated.palette();
                st.doc.palette = palette.clone();
                st.doc.set_theme(theme.clone());
                for d in st.tabs.iter_mut().flatten() {
                    d.palette = palette.clone();
                    d.set_theme(theme.clone());
                }
                if !result.own.is_empty() {
                    st.doc.begin_change();
                    for (id, name) in &result.own {
                        let _ = st.doc.ui.set_prop(*id, "theme", name.as_str());
                    }
                    st.doc.modified = true;
                }
                format!("Aparência: {}", if updated.theme.is_empty() { "Aylis Light" } else { &updated.theme })
            }
            (Err(e), _) | (_, Err(e)) => format!("⚠ {e}"),
        };
        st.project = Some(updated);
        drop(st);
        refresh(ui, &state2, &ids);
    });
}

// ===========================================================================
// Abrir e salvar forms
// ===========================================================================

pub fn open_form_dialog(state: &Shared) {
    let state2 = state.clone();
    spawn(state, async move {
        let Some(file) = crate::files::AsyncFileDialog::new().add_filter("Layout do Aylis", &["json"]).pick_file().await else { return };
        open_form_path(&state2, file.path());
    });
}

/// Abre um layout solto (ou o form do projeto, se ele for de lá).
pub fn open_form_path(state: &Shared, path: &Path) {
    let mut st = state.borrow_mut();
    let already = (0..st.tab_count()).find(|&i| st.tab(i).and_then(|d| d.path.as_deref()) == Some(path));
    if let Some(i) = already {
        st.switch_to(i);
        return;
    }
    match Document::open(path) {
        Ok(mut doc) => {
            doc.set_dark(st.doc.dark);
            if let Some(project) = &st.project {
                doc.ui.services_mut().resources = Some(Box::new(crate::assetdialogs::IdeResources(project.dir())));
                aylis_project::install_assets(&mut doc.ui, &aylis_project::Files::Dir(project.dir()));
                aylis_project::install_font_names(&mut doc.ui, project);
                crate::assetdialogs::register_used_icons(&mut doc.ui, project, Some(path));
            }
            st.open_tab(doc);
            st.status = format!("Aberto: {}", path.display());
            st.remember(path.to_path_buf());
        }
        Err(e) => st.status = format!("⚠ {e}"),
    }
}

/// Abre um recente (projeto ou form).
pub fn open_recent(ui: &mut Ui, state: &Shared, ids: &Ids, path: &Path) {
    // Uma gravação de uso (dois cliques num `.aylisrec`) abre no visualizador.
    if path.extension().is_some_and(|e| e.eq_ignore_ascii_case(aylis_controls::RECORDING_EXTENSION)) {
        crate::recordings::show(ui, path);
        return;
    }
    // Um relato de problema (dois cliques num `.aylisreport`) abre no visualizador.
    if path.extension().is_some_and(|e| e.eq_ignore_ascii_case(aylis_tools::report::EXTENSION)) {
        crate::reportview::show_file(ui, path);
        return;
    }
    // Um pacote de componente (dois cliques num `.aylispkg`): instala, com a pergunta.
    if path.extension().is_some_and(|e| e.eq_ignore_ascii_case(aylis_tools::packages::EXTENSION)) {
        crate::packagesview::install_file(ui, state, ids, path);
        return;
    }
    if path.extension().is_some_and(|e| e == aylis_project::EXTENSION) {
        open_project(ui, state, ids, path);
    } else {
        open_form_path(state, path);
    }
    save_config(state);
}

/// Salva a aba ativa (pergunta o arquivo se for nova).
/// Arquivo → Salvar (e o botão): no código embutido salva o arquivo da aba ativa (e o form,
/// se ele também mudou, como o Delphi salva a unit e o form juntos); no designer, o form.
pub fn save_current(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    crate::inspector::commit_pending(ui);
    if state.borrow().code_view && crate::codewindow::embedded_has_tabs(ui) {
        let file = crate::codewindow::embedded_active(ui);
        crate::codewindow::save_embedded(ui, false);
        let mut st = state.borrow_mut();
        if st.no_form || !st.doc.modified {
            if let Some(file) = file {
                st.status = format!("Salvo: {}", file.display());
            }
            return;
        }
    }
    save(state, false);
}

pub fn save(state: &Shared, ask: bool) {
    if state.borrow().no_form {
        state.borrow_mut().status = "Nenhum form aberto".into();
        return;
    }
    // A lite do navegador: o form vai para o armazenamento dele (não há disco).
    if !crate::lite::can(crate::lite::Capability::Disk) {
        crate::lite::save(state);
        return;
    }
    let (known, name) = {
        let st = state.borrow();
        (st.doc.path.clone().filter(|_| !ask), format!("{}.json", aylis_project::file_stem_for(&st.doc.form_name())))
    };
    if let Some(path) = known {
        save_active_to(state, &path);
        return;
    }
    let state2 = state.clone();
    let start = state.borrow().project.as_ref().map(|p| p.dir().join("forms"));
    spawn(state, async move {
        let mut dialog = crate::files::AsyncFileDialog::new().add_filter("Layout do Aylis", &["json"]).set_file_name(name);
        if let Some(dir) = start {
            dialog = dialog.set_directory(dir);
        }
        if let Some(file) = dialog.save_file().await {
            save_active_to(&state2, file.path());
        }
    });
}

fn save_active_to(state: &Shared, path: &Path) {
    let mut st = state.borrow_mut();
    let result = st.doc.save_to(path);
    st.status = match result {
        Ok(()) => {
            sync_form_name(&mut st);
            refresh_frames(&mut st);
            let name = st.doc.form_name();
            // O que o `.aylisproj` anota do form e os ícones e animações que ele cita.
            if let Some(project) = st.project.as_mut() {
                project.refresh_form(&name);
                let _ = project.save();
                let _ = codegen::add_assets_cited_in(project, &std::fs::read_to_string(path).unwrap_or_default());
            }
            match push_reload(&st, "form", &name, path) {
                Some(Ok(done)) => format!("Salvo: {} — no app: {done}", path.display()),
                Some(Err(e)) => format!("Salvo: {} — ⚠ no app: {e}", path.display()),
                None => format!("Salvo: {}", path.display()),
            }
        }
        Err(e) => format!("⚠ {e}"),
    };
}

/// Um form salvo pode ser o de um frame: os forms abertos mostram o conteúdo novo.
fn refresh_frames(st: &mut crate::state::State) {
    st.doc.refresh_frames();
    for d in st.tabs.iter_mut().flatten() {
        d.refresh_frames();
    }
}

/// Hot reload no Android, no iOS e no navegador: com o app conectado no canal, manda o
/// arquivo salvo (no desktop o app lê do disco sozinho). `None` = não há app conectado.
pub fn push_reload(st: &crate::state::State, kind: &str, name: &str, path: &Path) -> Option<Result<String, String>> {
    if matches!(st.run_target, aylis_tools::Target::Desktop | aylis_tools::Target::Linux) {
        return None;
    }
    let dev = st.dev.as_ref().filter(|d| d.connected())?;
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => return Some(Err(e.to_string())),
    };
    Some(dev.reload(kind, name, &text))
}

/// Se o form da aba ativa foi renomeado no designer, o projeto acompanha.
fn sync_form_name(st: &mut crate::state::State) {
    let Some(path) = st.doc.path.clone() else { return };
    let name = st.doc.form_name();
    let Some(project) = st.project.as_mut() else { return };
    let dir = project.dir();
    let entry = project.forms.iter().find(|f| dir.join(&f.file) == path).map(|f| f.name.clone());
    if let Some(old) = entry.filter(|old| *old != name) {
        project.rename_form(&old, &name);
        let _ = project.save();
    }
}

/// Salva todas as abas com arquivo (e o projeto). Devolve quantas ficaram sem salvar
/// (forms novos sem arquivo).
pub fn save_all(state: &Shared) -> usize {
    save_forms(state, true)
}

/// Salva só os forms modificados (para quem lê os forms do disco, como as Traduções); o
/// `.aylisproj`, os ícones e as animações só se algum foi salvo.
pub fn save_modified_forms(state: &Shared) -> usize {
    save_forms(state, false)
}

/// Salva as abas modificadas. Com `project`, também o projeto, os ícones e as animações;
/// sem, só se algum form foi salvo (o F12 não regrava o `.aylisproj` à toa).
fn save_forms(state: &Shared, project: bool) -> usize {
    let mut st = state.borrow_mut();
    let mut unsaved = 0;
    let mut saved = 0;
    let mut saved_files = Vec::new();
    let mut errors = Vec::new();
    for i in 0..st.tab_count() {
        let path = st.tab(i).and_then(|d| d.path.clone());
        let modified = st.tab(i).is_some_and(|d| d.modified);
        if !modified {
            continue;
        }
        let Some(path) = path else {
            unsaved += 1;
            continue;
        };
        let result = if i == st.active {
            st.doc.save_to(&path)
        } else {
            st.tabs[i].as_mut().map_or(Ok(()), |d| d.save_to(&path))
        };
        match result {
            Err(e) => errors.push(e),
            Ok(()) => {
                saved += 1;
                saved_files.push(path.clone());
                let name = st.tab(i).map(|d| d.form_name()).unwrap_or_default();
                // O que o `.aylisproj` anota do form (DataModule, `uses`, a hora do arquivo).
                if let Some(p) = st.project.as_mut() {
                    p.refresh_form(&name);
                }
                if let Some(Err(e)) = push_reload(&st, "form", &name, &path) {
                    errors.push(format!("no app: {e}"));
                }
            }
        }
    }
    sync_form_name(&mut st);
    refresh_frames(&mut st);
    if let Some(project) = st.project.as_ref().filter(|_| project || saved > 0) {
        if let Err(e) = project.save() {
            errors.push(e);
        }
        // Os ícones e as animações que os forms salvos citam vão para `assets/` (só eles: tirar
        // os que ninguém usa é do Executar e do Publicar, que leem o projeto inteiro).
        for file in &saved_files {
            let text = std::fs::read_to_string(file).unwrap_or_default();
            if let Err(e) = codegen::add_assets_cited_in(project, &text) {
                errors.push(e);
            }
        }
    }
    st.status = match (errors.first(), unsaved) {
        (Some(e), _) => format!("⚠ {e}"),
        (None, 0) => "Tudo salvo".into(),
        (None, n) => format!("Salvo; {n} form(s) novo(s) ainda sem arquivo (use Salvar)"),
    };
    unsaved
}

pub fn close_tab(ui: &mut Ui, state: &Shared, ids: &Ids, index: usize) {
    let modified = state.borrow().tab(index).is_some_and(|d| d.modified);
    if !modified {
        state.borrow_mut().close_tab(index);
        return;
    }
    let state2 = state.clone();
    let name = state.borrow().tab(index).map(Document::form_name).unwrap_or_default();
    confirm(ui, state, ids, &format!("Descartar as alterações de {name}?"), move |_| {
        state2.borrow_mut().close_tab(index);
    });
}

// ===========================================================================
// Edição
// ===========================================================================

pub fn edit(state: &Shared, op: &str) {
    let mut st = state.borrow_mut();
    let doc = &mut st.doc;
    let message = match op {
        "undo" => (!doc.undo()).then_some("Nada para desfazer"),
        "redo" => (!doc.redo()).then_some("Nada para refazer"),
        "cut" => (!doc.cut_selected()).then_some("Selecione um componente (o form não é recortado)"),
        "copy" => (!doc.copy_selected()).then_some("Selecione um componente (o form não é copiado)"),
        "paste" => {
            if let Err(e) = doc.paste() {
                st.status = e;
            }
            return;
        }
        "delete" => (!doc.delete_selected()).then_some("Selecione um componente (o form não é excluído)"),
        "select_all" => {
            doc.select_all();
            None
        }
        "front" => (!doc.bring_to_front(true)).then_some("Selecione um componente"),
        "back" => (!doc.bring_to_front(false)).then_some("Selecione um componente"),
        "lock" => {
            doc.locked = !doc.locked;
            Some(if doc.locked { "Controles travados" } else { "Controles destravados" })
        }
        "parent" => {
            let parent = doc.selection.and_then(|s| doc.ui.parent(s));
            if parent.is_some() {
                doc.select(parent);
            }
            None
        }
        _ => None,
    };
    st.status = message.unwrap_or_default().to_string();
}

pub fn align(state: &Shared, op: AlignOp) {
    let mut st = state.borrow_mut();
    if let Err(e) = st.doc.align_selection(op) {
        st.status = format!("⚠ {e}");
    }
}

pub fn zoom(state: &Shared, factor: Option<f32>) {
    let mut st = state.borrow_mut();
    st.zoom = match factor {
        Some(f) => (st.zoom * f).clamp(0.25, 4.0),
        None => 1.0,
    };
    st.status = format!("Zoom {:.0}%", st.zoom * 100.0);
}

pub fn set_dark(state: &Shared, dark: bool) {
    let mut st = state.borrow_mut();
    st.doc.set_dark(dark);
    for d in st.tabs.iter_mut().flatten() {
        d.set_dark(dark);
    }
}

/// O evento padrão do componente (o do duplo clique, como no Delphi): o que a classe
/// declara ([`aylis_core::Component::default_event`]); senão um evento próprio dela, na
/// ordem `on_change`, `on_timer`, `on_select`, `on_close`; senão o `on_click`. No form, o
/// `on_show` (o `FormShow`).
pub fn default_event_name(doc: &Document, id: ComponentId) -> Option<String> {
    let has = |name: &str| doc.ui.prop_info(id, name).is_some();
    if id == doc.root {
        return ["on_show", "on_activate"].into_iter().find(|e| has(e)).map(str::to_string);
    }
    let component = doc.ui.component(id)?;
    if let Some(e) = component.default_event().filter(|e| has(e)) {
        return Some(e.to_string());
    }
    // Os eventos declarados pela classe (e pelas camadas de dentro), sem os comuns.
    let mut own = Vec::new();
    let mut layer = Some(component);
    while let Some(c) = layer {
        own.extend(c.props().iter().filter(|p| p.kind == aylis_core::PropKind::Event).map(|p| p.name));
        layer = c.inner();
    }
    ["on_change", "on_timer", "on_select", "on_close"]
        .into_iter()
        .find(|e| own.contains(e))
        .or_else(|| has("on_click").then_some("on_click"))
        .or_else(|| own.first().copied())
        .map(str::to_string)
}

/// Liga o handler padrão (`btn_salvar_click`) ao evento `event` do componente, se ainda
/// não houver um, e devolve o nome do handler.
pub fn ensure_handler(state: &Shared, id: ComponentId, event: &str) -> Option<String> {
    let mut st = state.borrow_mut();
    if let Ok(PropValue::Event(Some(h))) = st.doc.ui.get_prop(id, event) {
        return Some(h);
    }
    let name = codegen::default_handler_name(st.doc.ui.name(id).unwrap_or("componente"), event);
    st.doc.set_prop(id, event, &Value::String(name.clone())).ok()?;
    st.status = format!("Evento {event} → {name}");
    Some(name)
}

/// Cria (se preciso) o handler do evento padrão do selecionado e mostra a aba de eventos.
/// Devolve o nome do handler.
/// "Onde é usado…" (item 20): os usos do componente escolhido no designer (um `Field` vira o
/// campo do dataset dele) no projeto inteiro, na janela da pesquisa.
pub fn show_usages(ui: &mut Ui, state: &Shared, ids: &crate::app::Ids) {
    let target = {
        let st = state.borrow();
        let Some(id) = st.doc.selection else { return };
        let form = st.doc.form_name();
        let name = st.doc.ui.name(id).unwrap_or_default().to_string();
        let layout: serde_json::Value = st.doc.to_json().ok().and_then(|j| serde_json::from_str(&j).ok()).unwrap_or_default();
        crate::usages::target_of(&layout, &form, &name)
    };
    crate::projectsearch::show_usages(ui, state, ids, target);
}

pub fn default_event(state: &Shared) -> Option<String> {
    let (id, event) = {
        let mut st = state.borrow_mut();
        st.events_tab = true;
        let id = st.doc.selection?;
        (id, default_event_name(&st.doc, id)?)
    };
    ensure_handler(state, id, &event)
}

/// Duplo clique no designer (como no Delphi): o editor do componente, se a classe tiver
/// (campos do ClientDataSet, testar o MessageDialog); senão o handler do evento padrão,
/// criado se preciso, aberto no código.
pub fn designer_dbl_click(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some((id, class)) = ({
        let st = state.borrow();
        st.doc.selection.map(|id| (id, st.doc.ui.class_name(id).unwrap_or_default().to_string()))
    }) else {
        return;
    };
    match class.as_str() {
        "ClientDataSet" => crate::fieldsview::open(ui, state, ids, id),
        "DataGrid" => crate::columnsview::open(ui, state, ids, id),
        "PopupMenu" | "MainMenu" => crate::menuview::open(ui, state, ids, id),
        "MessageDialog" => test_message_dialog(ui, state, ids, id),
        _ => {
            if let Some(handler) = default_event(state) {
                open_code(ui, state, Some(&handler));
            }
        }
    }
}

/// Mostra a caixa do `MessageDialog` como ela vai aparecer (título, texto, tipo, botões).
fn test_message_dialog(ui: &mut Ui, state: &Shared, ids: &Ids, id: ComponentId) {
    let props: Vec<(&str, PropValue)> = {
        let st = state.borrow();
        ["title", "text", "kind", "buttons"].into_iter().filter_map(|p| Some((p, st.doc.ui.get_prop(id, p).ok()?))).collect()
    };
    let probe = ui.add(aylis_controls::MessageDialog::default(), ids.form);
    for (name, value) in props {
        let _ = ui.set_prop(probe, name, value);
    }
    let state = state.clone();
    aylis_controls::MessageDialog::execute_then(ui, probe, move |ui, result| {
        ui.free(probe);
        state.borrow_mut().status = format!("MessageDialog testado: o botão escolhido foi \"{}\"", aylis_core::PropEnum::name(result));
    });
}

/// Editor de campos do ClientDataSet selecionado.
pub fn fields_editor(ui: &mut Ui, state: &Shared, ids: &Ids, cds: ComponentId) {
    let (specs, title) = {
        let st = state.borrow();
        let doc = &st.doc;
        let specs: Vec<FieldSpec> = doc
            .ui
            .children(cds)
            .iter()
            .copied()
            .filter(|&f| doc.ui.class_name(f) == Some("Field"))
            .map(|f| FieldSpec {
                id: Some(f),
                name: doc.ui.name(f).unwrap_or_default().to_string(),
                kind: dialogs::prop_text(&doc.ui, f, "type"),
                size: match doc.ui.get_prop(f, "size") {
                    Ok(PropValue::Int(0)) => String::new(),
                    Ok(PropValue::Int(n)) => n.to_string(),
                    _ => String::new(),
                },
                label: dialogs::prop_text(&doc.ui, f, "display_label"),
            })
            .collect();
        (specs, format!("Campos de {}", doc.ui.name(cds).unwrap_or("ClientDataSet")))
    };
    let types = <aylis_data::FieldType as aylis_core::PropEnum>::NAMES;
    let (state2, ids) = (state.clone(), *ids);
    dialogs::fields_editor(ui, &title, specs, types, move |ui, specs| {
        {
            let mut st = state2.borrow_mut();
            let mut error = None;
            let doc = &mut st.doc;
            doc.begin_change();
            let keep: Vec<ComponentId> = specs.iter().filter_map(|s| s.id).collect();
            let existing: Vec<ComponentId> = doc.ui.children(cds).iter().copied().filter(|&f| doc.ui.class_name(f) == Some("Field")).collect();
            for f in existing.into_iter().filter(|f| !keep.contains(f)) {
                doc.ui.free(f);
            }
            for (index, spec) in specs.iter().enumerate() {
                let id = match spec.id.filter(|&i| doc.ui.contains(i)) {
                    Some(id) => id,
                    None => match doc.ui.create("Field", Some(cds)) {
                        Ok(id) => id,
                        Err(e) => {
                            error = Some(format!("⚠ {e}"));
                            break;
                        }
                    },
                };
                let _ = doc.ui.set_name(id, &spec.name);
                let _ = doc.ui.set_prop(id, "type", spec.kind.as_str());
                let size: i64 = spec.size.parse().unwrap_or(0);
                let _ = doc.ui.set_prop(id, "size", size);
                let _ = doc.ui.set_prop(id, "display_label", spec.label.as_str());
                let _ = doc.ui.set_child_index(id, index);
            }
            doc.ui.layout();
            if let Some(e) = error {
                st.status = e;
            }
        }
        refresh(ui, &state2, &ids);
    });
}

// ===========================================================================
// Código e execução
// ===========================================================================

/// Gera o código dos eventos (do projeto inteiro ou do form solto); os editores abertos
/// acompanham o que a geração puser neles.
pub fn generate_code(ui: &mut Ui, state: &Shared) -> Result<Vec<PathBuf>, String> {
    let before = crate::codewindow::snapshot(&crate::codewindow::open_paths());
    let result = generate_code_on_disk(state);
    follow_generation(ui, state, before);
    result
}

/// O editor acompanha a geração; o que não deu para juntar vira aviso na situação.
fn follow_generation(ui: &mut Ui, state: &Shared, before: Vec<(PathBuf, String)>) {
    let failed = crate::codewindow::follow_generation(ui, before);
    if let Some(file) = failed.first() {
        state.borrow_mut().status = format!("⚠ A geração acrescentou código em {}, que tem alterações não salvas no editor: salve e gere de novo", file.display());
    }
}

fn generate_code_on_disk(state: &Shared) -> Result<Vec<PathBuf>, String> {
    if state.borrow().project.is_some() {
        if save_all(state) > 0 {
            return Err("salve os forms novos antes (Salvar)".into());
        }
        let st = state.borrow();
        return codegen::generate(st.project.as_ref().expect("projeto aberto"));
    }
    let path = state.borrow().doc.path.clone().ok_or("salve o layout antes de gerar o código (os arquivos ficam ao lado dele)")?;
    save_active_to(state, &path);
    codegen::generate_standalone(&path)
}

pub fn generate_code_command(state: &Shared) {
    // Com projeto, no trabalho (fora da thread da tela): gerar relê e reescreve o código de todos
    // os forms (1,7 s no ERP de 1009 forms).
    if state.borrow().project.is_some() {
        if save_all(state) > 0 {
            state.borrow_mut().status = "⚠ salve os forms novos antes (Salvar)".into();
            return;
        }
        let project = state.borrow().project.clone().expect("projeto aberto");
        // O editor acompanha o que a geração puser nos arquivos abertos.
        let open_files = crate::codewindow::open_paths();
        crate::jobs::start(state, crate::jobs::Slot::Main, "Gerando o código", move |_, sender| {
            let before = crate::codewindow::snapshot(&open_files);
            let files = codegen::generate(&project)?;
            sender.send(crate::jobs::Msg::Generated(before));
            // A classe com nome errado (G02): o componente ficou fora do acesso tipado.
            let suspicious = aylis_project::bindings::suspicious_classes(&project);
            let warning = if suspicious.is_empty() { String::new() } else { format!(" · ⚠ {}", suspicious.join("; ")) };
            Ok(format!("Código gerado: {} arquivo(s){warning}", files.len()))
        });
        return;
    }
    let result = generate_code_on_disk(state);
    let mut st = state.borrow_mut();
    st.status = match result {
        Ok(files) => format!("Código gerado: {} arquivo(s)", files.len()),
        Err(e) => format!("⚠ {e}"),
    };
}

/// Abre o código do form (F12), no handler `handler` se informado.
/// Abre o código do form na janela de código da IDE (no handler, se dado).
/// O arquivo de código do form `doc` (a unit dele, como no Delphi): o fonte do form no
/// projeto, ou o `<form>_handlers.rs` ao lado do form solto. `None`: form ainda sem arquivo.
pub fn code_file_of(st: &crate::state::State, doc: &Document) -> Option<PathBuf> {
    match &st.project {
        Some(project) => codegen::form_source(project, &doc.form_name()),
        None => doc.path.as_ref().map(|p| {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("tela");
            p.with_file_name(format!("{stem}_handlers.rs"))
        }),
    }
}

/// A aba do form cujo código está ativo no editor embutido (para voltar ao Design nele). O
/// form do projeto ainda sem aba é aberto: antes, o Design voltava ao último form mostrado,
/// que não era o deste código, e o F12 dizia que o fonte não tinha form.
/// O fonte à vista no editor é o de um form (o Design tem para onde ir; I05 do Tabernáculo). A
/// resposta fica guardada pelo arquivo: o refresh da IDE pergunta a cada mudança, e procurar
/// entre todos os forms do projeto a cada vez custaria no ERP de 1009 forms.
pub fn active_code_has_form(ui: &Ui, state: &Shared) -> bool {
    thread_local! {
        static LAST: std::cell::RefCell<Option<(PathBuf, bool)>> = const { std::cell::RefCell::new(None) };
    }
    let Some(file) = crate::codewindow::embedded_active(ui) else { return true };
    if let Some(known) = LAST.with(|l| l.borrow().as_ref().filter(|(f, _)| *f == file).map(|(_, h)| *h)) {
        return known;
    }
    let Ok(st) = state.try_borrow() else { return true };
    let open = (0..st.tab_count()).any(|i| st.tab(i).and_then(|d| code_file_of(&st, d)).is_some_and(|f| crate::debugger::norm(&f) == file));
    let has = open || st.project.as_ref().is_some_and(|p| p.forms.iter().any(|e| crate::debugger::norm(&codegen::entry_source(p, e)) == file));
    LAST.with(|l| *l.borrow_mut() = Some((file, has)));
    has
}

pub fn form_of_active_code(ui: &Ui, state: &Shared) -> Option<usize> {
    let file = crate::codewindow::embedded_active(ui)?;
    let unopened = {
        let st = state.try_borrow().ok()?;
        let tab = (0..st.tab_count()).find(|&i| st.tab(i).and_then(|d| code_file_of(&st, d)).is_some_and(|f| crate::debugger::norm(&f) == file));
        if tab.is_some() && !st.no_form {
            return tab;
        }
        let project = st.project.as_ref()?;
        project.forms.iter().find(|e| crate::debugger::norm(&codegen::entry_source(project, e)) == file)?.name.clone()
    };
    open_project_form(state, &unopened);
    let st = state.try_borrow().ok()?;
    // Só a aba do form deste fonte (o form que não abriu deixa o aviso no rodapé).
    (!st.no_form && code_file_of(&st, &st.doc).is_some_and(|f| crate::debugger::norm(&f) == file)).then_some(st.active)
}

pub fn open_code(ui: &mut Ui, state: &Shared, handler: Option<&str>) {
    // Só o código deste form (a unit e o acesso tipado): o projeto inteiro fica para o
    // "Gerar código" e o Executar.
    let generated = if state.borrow().project.is_some() {
        if save_forms(state, false) > 0 {
            Err("salve os forms novos antes (Salvar)".to_string())
        } else {
            let before = crate::codewindow::snapshot(&crate::codewindow::open_paths());
            let result = {
                let st = state.borrow();
                codegen::generate_form(st.project.as_ref().expect("projeto aberto"), &st.doc.form_name())
            };
            follow_generation(ui, state, before);
            result
        }
    } else {
        generate_code(ui, state)
    };
    let target = {
        let st = state.borrow();
        code_file_of(&st, &st.doc)
    };
    let mut st = state.borrow_mut();
    if let Err(e) = generated {
        st.status = format!("⚠ {e}");
        return;
    }
    let Some(file) = target else { return };
    let line = handler
        .and_then(|h| match &st.project {
            // Em qualquer linguagem do projeto (a definição muda de uma para outra).
            Some(project) => aylis_project::sync::handler_location(project, project.language, &st.doc.form_name(), h).map(|(_, line)| line),
            None => {
                let text = std::fs::read_to_string(&file).ok()?;
                let at = text.find(&format!("fn {h}(")).or_else(|| text.find(&format!("def {h}(")))?;
                Some(text[..at].matches('\n').count() + 1)
            }
        })
        // Sem handler, o F12 volta ao código onde ele estava (o cursor, a seleção e a rolagem,
        // como no Delphi; ia ao topo do arquivo, conferência de 08/11); a primeira vez, no topo.
        .unwrap_or(if handler.is_none() && crate::codewindow::text_of(ui, &file).is_some() { 0 } else { 1 });
    drop(st);
    let status = match crate::codewindow::open(ui, &file, line) {
        Ok(_) if line == 0 => format!("Código: {}", file.display()),
        Ok(_) => format!("Código: {}:{line}", file.display()),
        Err(e) => format!("⚠ não consegui abrir {}: {e}", file.display()),
    };
    state.borrow_mut().status = status;
}

/// Abre o arquivo no VS Code na linha e coluna, ou no programa padrão do sistema.
pub fn open_in_editor_at(file: &Path, line: usize, column: usize) -> Result<&'static str, String> {
    // Sem `cmd /C`: um nome de arquivo com `&` não vira comando.
    let target = format!("{}:{line}:{column}", file.display());
    if aylis_tools::shell::run_vscode(["-g", target.as_str()]).is_ok() {
        return Ok("VS Code");
    }
    aylis_tools::shell::open(&file.display().to_string()).map(|_| "editor padrão")
}

/// Executa o projeto (F9) na plataforma escolhida, verificando o ambiente antes; sem
/// projeto, abre a prévia do form.
pub fn run_project(ui: &mut Ui, state: &Shared) {
    // O código alterado no editor entra na compilação (ela lê o disco).
    crate::codewindow::save_all(ui);
    if state.borrow().project.is_none() {
        run_preview(state);
        return;
    }
    // Gerar o código e verificar o projeto rodam no trabalho (`jobs::prepare`), fora da thread
    // da tela; aqui só o que precisa dela: gravar os forms.
    if save_all(state) > 0 {
        state.borrow_mut().status = "⚠ salve os forms novos antes (Salvar)".into();
        return;
    }
    crate::jobs::run_project(state);
}

/// Compila otimizado (release) e executa: para ver o app como vai ser publicado.
pub fn run_release(ui: &mut Ui, state: &Shared) {
    crate::codewindow::save_all(ui);
    if state.borrow().project.is_none() {
        state.borrow_mut().status = "Abra um projeto para compilar em release".into();
        return;
    }
    if save_all(state) > 0 {
        state.borrow_mut().status = "⚠ salve os forms novos antes (Salvar)".into();
        return;
    }
    crate::jobs::run_release(state);
}

/// Para o programa em execução (ou a compilação).
pub fn stop(state: &Shared) {
    crate::jobs::stop(state);
}

/// Abre o form numa janela separada (outro processo), com os eventos no console.
pub fn run_preview(state: &Shared) {
    let mut st = state.borrow_mut();
    let path: PathBuf = std::env::temp_dir().join("aylis_ide_preview.json");
    let result = st
        .doc
        .to_json()
        .and_then(|json| std::fs::write(&path, json).map_err(|e| e.to_string()))
        .and_then(|_| std::env::current_exe().map_err(|e| e.to_string()))
        .and_then(|exe| {
            let mut cmd = Command::new(exe);
            cmd.arg("--preview").arg(&path);
            if st.doc.dark {
                cmd.arg("--dark");
            }
            if let Some(project) = &st.project {
                cmd.arg("--assets").arg(project.dir());
            }
            cmd.spawn().map_err(|e| e.to_string())
        });
    st.status = match result {
        Ok(_) => "Prévia do form numa janela separada".into(),
        Err(e) => format!("Não consegui executar: {e}"),
    };
}

/// Sair: pergunta se há alterações não salvas.
pub fn quit(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let form = ids.form;
    if !state.borrow().any_modified() {
        crate::jobs::stop_all(state);
        state.borrow_mut().quit_confirmed = true;
        ui.close(form);
        return;
    }
    let state2 = state.clone();
    confirm(ui, state, ids, "Há forms com alterações não salvas. Sair mesmo assim?", move |ui| {
        state2.borrow_mut().quit_confirmed = true;
        crate::jobs::stop_all(&state2);
        ui.close(form);
    });
}
