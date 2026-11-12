//! O documento aberto no designer: um form num `Ui` próprio (separado do `Ui` da IDE),
//! a seleção (um ou vários componentes) e as operações de edição com desfazer/refazer.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use aylis_core::platform::LocaleInfo;
use aylis_core::{Align, ComponentId, PointF, PropValue, RectF, Theme, Ui};
use serde_json::{Map, Value, json};

/// Grade do designer, em unidades lógicas.
pub const GRID: f32 = 8.0;

/// Distância em que o arrasto "gruda" numa guia (borda ou centro de um vizinho).
const GUIDE_SNAP: f32 = 5.0;

/// Classes que aceitam controles filhos ao soltar da paleta.
const CONTAINERS: &[&str] = &["Form", "Panel", "Layout", "Rectangle", "ScrollBox", "Report", "ReportBand", "Card", "Parallax"];

/// Alça de redimensionamento.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Handle {
    pub const ALL: [Handle; 8] = [
        Handle::TopLeft,
        Handle::Top,
        Handle::TopRight,
        Handle::Right,
        Handle::BottomRight,
        Handle::Bottom,
        Handle::BottomLeft,
        Handle::Left,
    ];

    /// A alça do outro lado (a que fica parada quando esta é arrastada).
    pub fn opposite(self) -> Handle {
        match self {
            Handle::TopLeft => Handle::BottomRight,
            Handle::Top => Handle::Bottom,
            Handle::TopRight => Handle::BottomLeft,
            Handle::Right => Handle::Left,
            Handle::BottomRight => Handle::TopLeft,
            Handle::Bottom => Handle::Top,
            Handle::BottomLeft => Handle::TopRight,
            Handle::Left => Handle::Right,
        }
    }

    /// Para onde a alça aponta, em graus na tela sem giro (0 = direita, 90 = baixo).
    pub fn angle(self) -> f32 {
        match self {
            Handle::Right => 0.0,
            Handle::BottomRight => 45.0,
            Handle::Bottom => 90.0,
            Handle::BottomLeft => 135.0,
            Handle::Left => 180.0,
            Handle::TopLeft => 225.0,
            Handle::Top => 270.0,
            Handle::TopRight => 315.0,
        }
    }

    fn sides(self) -> (bool, bool, bool, bool) {
        // (esquerda, cima, direita, baixo)
        match self {
            Handle::TopLeft => (true, true, false, false),
            Handle::Top => (false, true, false, false),
            Handle::TopRight => (false, true, true, false),
            Handle::Right => (false, false, true, false),
            Handle::BottomRight => (false, false, true, true),
            Handle::Bottom => (false, false, false, true),
            Handle::BottomLeft => (true, false, false, true),
            Handle::Left => (true, false, false, false),
        }
    }

    /// Ponto da alça num retângulo.
    pub fn point(self, r: RectF) -> PointF {
        let (l, t, rt, b) = self.sides();
        let x = if l { r.x } else if rt { r.right() } else { r.center().x };
        let y = if t { r.y } else if b { r.bottom() } else { r.center().y };
        PointF::new(x, y)
    }
}

/// Como alinhar a seleção (menu Editar > Alinhar).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignOp {
    Left,
    CenterH,
    Right,
    Top,
    CenterV,
    Bottom,
    DistributeH,
    DistributeV,
    SameWidth,
    SameHeight,
    SameSize,
}

/// Uma guia mostrada durante o arrasto: linha vertical (`x`) ou horizontal (`y`), em
/// coordenadas do form.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Guide {
    Vertical { x: f32, from: f32, to: f32 },
    Horizontal { y: f32, from: f32, to: f32 },
}

/// Gira `p` em torno de `center` (graus, sentido horário na tela, como o `Canvas::rotate`).
pub fn rotate_about(p: PointF, center: PointF, degrees: f32) -> PointF {
    let (sin, cos) = degrees.to_radians().sin_cos();
    let (dx, dy) = (p.x - center.x, p.y - center.y);
    PointF::new(center.x + dx * cos - dy * sin, center.y + dx * sin + dy * cos)
}

fn snap(v: f32) -> f32 {
    (v / GRID).round() * GRID
}

/// Cria um `Ui` de documento com todas as classes padrão (em modo de design).
/// O "Aylis Light" ou o "Aylis Dark" de fábrica, um só para todos os documentos (cada tema tem
/// ~0,2 MB, e cada aba de form é um documento).
fn builtin_theme(dark: bool) -> std::sync::Arc<Theme> {
    static LIGHT: std::sync::OnceLock<std::sync::Arc<Theme>> = std::sync::OnceLock::new();
    static DARK: std::sync::OnceLock<std::sync::Arc<Theme>> = std::sync::OnceLock::new();
    let cell = if dark { &DARK } else { &LIGHT };
    cell.get_or_init(|| std::sync::Arc::new(if dark { Theme::dark() } else { Theme::light() })).clone()
}

pub fn new_document_ui(dark: bool) -> Ui {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    // O 3D fica fora do registro remoto (que também roda no navegador).
    aylis_3d::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    if dark {
        ui.set_shared_theme(builtin_theme(true));
    }
    ui.set_design_mode(true);
    ui
}

/// O layout (`.json`) de um form do projeto pelo nome, para os frames.
pub type FrameSource = Rc<dyn Fn(&str) -> Option<Value>>;

/// Põe em cada `Frame` de `root` o form que ele cita (`source`), ocupando a área toda. É só
/// para ver: o designer não seleciona nem salva o que está dentro de um frame. `loading`
/// são os forms já abertos um dentro do outro (um frame que usa o próprio form fica vazio).
fn embed_frames(ui: &mut Ui, root: ComponentId, source: &dyn Fn(&str) -> Option<Value>, loading: &mut Vec<String>) {
    let mut frames = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if ui.class_name(id) == Some("Frame") {
            let form = match ui.get_prop(id, "form") {
                Ok(PropValue::String(f)) => f,
                _ => String::new(),
            };
            frames.push((id, form));
        } else {
            stack.extend(ui.children(id).iter().copied());
        }
    }
    for (frame, name) in frames {
        let children = ui.children(frame).to_vec();
        ui.free_many(&children);
        if name.is_empty() || loading.iter().any(|l| l.eq_ignore_ascii_case(&name)) {
            continue;
        }
        let Some(mut layout) = source(&name) else { continue };
        // Um pacote (`pkg:Nome`): o manifesto vem junto; cada uso é um form à parte.
        let manifest = layout.as_object_mut().and_then(|o| o.remove("manifest")).and_then(|m| serde_json::from_value::<aylis_project::packages::Manifest>(m).ok());
        if let Some(package) = aylis_project::packages::package_of(&name) {
            layout["root"]["name"] = Value::String(aylis_project::packages::inner_name(ui, frame, package));
        }
        let Ok(form) = aylis_json::load_value(ui, &layout, None) else { continue };
        if let Some(m) = &manifest {
            let values = match ui.get_prop(frame, "values") {
                Ok(PropValue::String(v)) => v,
                _ => String::new(),
            };
            aylis_project::packages::apply_values(ui, form, m, &values);
        }
        if ui.set_parent(form, Some(frame)).is_err() {
            ui.free(form);
            continue;
        }
        let _ = ui.set_prop(form, "align", "client");
        if matches!(ui.get_prop(form, "fill"), Ok(PropValue::Null)) {
            let _ = ui.set_prop(form, "fill", aylis_core::Color::TRANSPARENT);
        }
        if let Some(cb) = ui.control_mut(form) {
            cb.set_visible(true);
        }
        loading.push(name);
        embed_frames(ui, form, source, loading);
        loading.pop();
    }
}

/// Uma entrada do histórico: o documento inteiro em JSON e o que estava selecionado.
#[derive(Clone, Debug)]
struct Snapshot {
    json: Value,
    selected: Vec<String>,
}

pub struct Document {
    pub ui: Ui,
    pub root: ComponentId,
    pub path: Option<PathBuf>,
    /// A marca do arquivo quando foi lido ou salvo: diferente da de agora, ele mudou por fora
    /// ([`crate::disk`]).
    pub disk: Option<aylis_project::FileStamp>,
    /// O selecionado principal (o que o Object Inspector mostra).
    pub selection: Option<ComponentId>,
    /// Todos os selecionados (inclui o principal).
    pub selected: Vec<ComponentId>,
    pub modified: bool,
    pub dark: bool,
    /// O tema com que o form é desenhado (o do projeto, com a mistura por classe).
    /// Compartilhado com o `Ui` do form e com as outras abas do mesmo tema (`Arc`).
    pub theme: std::sync::Arc<Theme>,
    /// As cores do projeto (`colors` do `.aylisproj`): a versão clara ou escura do form
    /// ([`set_dark`](Self::set_dark)) as aplica de novo, no modo dela.
    pub palette: aylis_core::palette::Palette,
    /// Os temas próprios do projeto (`themes/*.json`): o claro e o escuro de uma família que
    /// não é do aylis (NoOBS Light e NoOBS Dark) para o [`set_dark`](Self::set_dark).
    pub project_themes: Vec<Theme>,
    /// O layout de um form do projeto pelo nome: o conteúdo que cada `Frame` mostra.
    pub frame_source: Option<FrameSource>,
    /// Exibir → Mostrar componentes escondidos: os `visible = false` aparecem todos
    /// (translúcidos); senão só os selecionados (e os que os contêm).
    pub show_hidden: bool,
    /// "Travar controles" (Editar): não deixa mover nem redimensionar com o mouse.
    pub locked: bool,
    /// Guias do arrasto em andamento.
    pub guides: Vec<Guide>,
    /// Os DataModules que o form usa (`"uses"` no layout).
    pub uses: Vec<String>,
    /// Os layouts desses DataModules, carregados no mesmo `Ui` (sem desenhar) para as
    /// referências `dmDados.x` existirem no designer.
    modules: Vec<Value>,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    clipboard: Vec<Value>,
    /// Editando numa vista por largura (Celular, Tablet...): veja [`Document::enter_view`].
    pub view_mode: Option<ViewMode>,
    /// O aparelho em que o form é visto no designer (a moldura, o tamanho e a área segura) e
    /// se está deitado.
    pub device: Option<(&'static aylis_core::devices::Device, bool)>,
}

/// A vista em edição no designer: o nome, a largura da prévia e os valores base das
/// propriedades acompanhadas (o que mudar deles vira mudança da vista).
pub struct ViewMode {
    pub name: String,
    pub width: f32,
    /// A altura da prévia (a do aparelho escolhido); `None`: a do form.
    pub height: Option<f32>,
    /// O valor base de cada propriedade dos componentes com nome (e do form), para saber o
    /// que a edição na vista mudou.
    base: Vec<(ComponentId, String, String, PropValue)>,
    /// Os filhos de cada contêiner no base, para saber quem mudou de contêiner ou de ordem.
    base_children: Vec<(ComponentId, Vec<ComponentId>)>,
    root_width: PropValue,
    root_height: PropValue,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// Documento novo com um `Form1` vazio.
    pub fn new() -> Self {
        Self::new_named("Form1", "Form1")
    }

    /// Documento novo com um form vazio chamado `name`.
    pub fn new_named(name: &str, caption: &str) -> Self {
        let mut ui = new_document_ui(false);
        let root = ui.create("Form", None).expect("Form registrado");
        let _ = ui.set_name(root, name);
        let _ = ui.set_prop(root, "caption", caption);
        let _ = ui.set_prop(root, "padding", 8);
        ui.layout();
        Self {
            ui,
            root,
            path: None,
            disk: None,
            selection: Some(root),
            selected: vec![root],
            modified: false,
            dark: false,
            theme: builtin_theme(false),
            palette: Vec::new(),
            project_themes: Vec::new(),
            frame_source: None,
            show_hidden: false,
            locked: false,
            guides: Vec::new(),
            uses: Vec::new(),
            modules: Vec::new(),
            undo: Vec::new(),
            redo: Vec::new(),
            clipboard: Vec::new(),
            view_mode: None,
            device: None,
        }
    }

    /// Lê um documento JSON (layout do aylis).
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON inválido: {e}"))?;
        let mut doc = Self::new();
        doc.uses = aylis_project::uses_in(&value["root"]);
        doc.replace(&value)?;
        doc.modified = false;
        Ok(doc)
    }

    /// Documento novo com um DataModule vazio chamado `name`.
    pub fn new_data_module(name: &str) -> Self {
        let mut doc = Self::new();
        let value = serde_json::json!({ "aylis": 1, "root": { "class": "DataModule", "name": name } });
        doc.replace(&value).expect("DataModule registrado");
        doc.modified = false;
        doc
    }

    /// A raiz é um DataModule (só componentes não visuais)?
    pub fn is_data_module(&self) -> bool {
        self.ui.class_name(self.root) == Some("DataModule")
    }

    /// Troca os DataModules usados (os layouts vêm do projeto): recarrega o documento com
    /// eles, para as referências aparecerem no Object Inspector.
    pub fn set_modules(&mut self, uses: Vec<String>, modules: Vec<Value>) -> Result<(), String> {
        let current = aylis_json::save_value(&self.ui, self.root).map_err(|e| e.to_string())?;
        let changed = uses != self.uses;
        self.uses = uses;
        self.modules = modules;
        self.replace(&current)?;
        if changed {
            self.modified = true;
        }
        Ok(())
    }

    /// Abre um form do projeto: os DataModules do `uses` (`modules` devolve os layouts
    /// deles) entram antes, para as referências `dmDados.x` existirem.
    ///
    /// `theme` é o do projeto: o form carrega já com ele, porque as cores pelo nome (as do
    /// tema e as do projeto, como `"fill": "green"`) se resolvem ao carregar.
    pub fn open_with_modules(path: &Path, theme: Option<std::sync::Arc<Theme>>, frames: Option<FrameSource>, modules: impl Fn(&[String]) -> Vec<Value>) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
        let value: Value = serde_json::from_str(&text).map_err(|e| format!("JSON inválido: {e}"))?;
        let mut doc = Self::new();
        if let Some(theme) = theme {
            doc.set_shared_theme(theme);
        }
        doc.frame_source = frames;
        doc.uses = aylis_project::uses_in(&value["root"]);
        doc.modules = modules(&doc.uses);
        doc.replace(&value)?;
        doc.modified = false;
        doc.path = Some(path.to_path_buf());
        doc.disk = aylis_project::FileStamp::read(path);
        Ok(doc)
    }

    pub fn open(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
        let mut doc = Self::from_json(&text)?;
        doc.path = Some(path.to_path_buf());
        Ok(doc)
    }

    /// Troca o conteúdo pelo layout JSON dado (desfazível).
    pub fn replace_json(&mut self, json: &str) -> Result<(), String> {
        let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON inválido: {e}"))?;
        self.begin_change();
        if let Err(e) = self.replace(&value) {
            self.undo();
            return Err(e);
        }
        self.uses = aylis_project::uses_in(&value["root"]);
        self.modified = true;
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, String> {
        let mut value = aylis_json::save_value(&self.ui, self.root).map_err(|e| e.to_string())?;
        if !self.uses.is_empty() {
            // `uses` logo depois de `class` e `name`, antes das propriedades.
            if let Value::Object(root) = value["root"].take() {
                let mut ordered = serde_json::Map::new();
                for key in ["class", "name"] {
                    if let Some(v) = root.get(key) {
                        ordered.insert(key.into(), v.clone());
                    }
                }
                ordered.insert("uses".into(), Value::from(self.uses.clone()));
                for (k, v) in root {
                    ordered.entry(k).or_insert(v);
                }
                value["root"] = Value::Object(ordered);
            }
        }
        Ok(aylis_json::to_form_text(&value))
    }

    pub fn save_to(&mut self, path: &Path) -> Result<(), String> {
        let json = self.to_json()?;
        std::fs::write(path, json).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
        self.path = Some(path.to_path_buf());
        self.modified = false;
        self.disk = aylis_project::FileStamp::read(path);
        Ok(())
    }

    /// Nome do form (a raiz).
    pub fn form_name(&self) -> String {
        self.ui.name(self.root).unwrap_or("Form1").to_string()
    }

    /// Nome para mostrar (arquivo ou "sem título").
    pub fn title(&self) -> String {
        let name = self.path.as_ref().and_then(|p| p.file_name()).map_or("sem título".into(), |n| n.to_string_lossy().into_owned());
        if self.modified { format!("{name} •") } else { name }
    }

    /// Troca todo o conteúdo por um documento JSON.
    fn replace(&mut self, document: &Value) -> Result<(), String> {
        let mut ui = new_document_ui(self.dark);
        ui.set_shared_theme(self.theme.clone());
        // Mantém o acesso aos arquivos do projeto (imagens) ao recarregar.
        if let Some(resources) = self.ui.services_mut().resources.take() {
            ui.services_mut().resources = Some(resources);
        }
        // Os DataModules antes: as referências do form apontam para eles.
        for module in &self.modules {
            if let Err(e) = aylis_json::load_value(&mut ui, module, None) {
                eprintln!("aylis-ide: DataModule: {e}");
            }
        }
        let root = aylis_json::load_value(&mut ui, document, None).map_err(|e| e.to_string())?;
        // O designer mostra o base (as vistas por largura só pelo seletor de vista).
        ui.force_view(root, Some(""));
        if let Some(source) = self.frame_source.clone() {
            let mut loading = vec![ui.name(root).unwrap_or_default().to_string()];
            embed_frames(&mut ui, root, &*source, &mut loading);
        }
        ui.layout();
        self.ui = ui;
        self.root = root;
        self.selection = Some(root);
        self.selected = vec![root];
        self.sync_reveal();
        Ok(())
    }

    /// Os componentes escondidos que aparecem no designer: os selecionados (e os que os
    /// contêm) ou, com [`show_hidden`](Self::show_hidden), todos.
    pub fn sync_reveal(&mut self) {
        let selected = self.selected.clone();
        self.ui.set_design_reveal(&selected, self.show_hidden);
        self.ui.layout();
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            json: aylis_json::save_value(&self.ui, self.root).unwrap_or(Value::Null),
            selected: self.selected.iter().filter_map(|&s| self.ui.name(s)).map(str::to_string).collect(),
        }
    }

    /// Guarda o estado atual no histórico, antes de uma alteração.
    pub fn begin_change(&mut self) {
        let snapshot = self.snapshot();
        self.undo.push(snapshot);
        if self.undo.len() > 200 {
            self.undo.remove(0);
        }
        self.redo.clear();
        self.modified = true;
    }

    fn restore(&mut self, snapshot: Snapshot) {
        if self.replace(&snapshot.json).is_ok() {
            let ids: Vec<ComponentId> = snapshot.selected.iter().filter_map(|n| self.ui.find_in(self.root, n)).collect();
            self.select_many(ids);
            self.modified = true;
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.pop() else { return false };
        let current = self.snapshot();
        self.redo.push(current);
        self.restore_keeping_view(previous);
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(next) = self.redo.pop() else { return false };
        let current = self.snapshot();
        self.undo.push(current);
        self.restore_keeping_view(next);
        true
    }

    /// Volta a um estado guardado; editando numa vista, continua nela (o form é refeito do
    /// JSON, com o base e as vistas).
    fn restore_keeping_view(&mut self, snapshot: Snapshot) {
        let view = self.view_mode.take().map(|m| (m.name, m.width, m.height));
        self.restore(snapshot);
        match view {
            Some((name, width, height)) => self.enter_view_sized(Some(&name), width, height),
            None => {
                self.ui.force_view(self.root, Some(""));
                self.ui.layout();
            }
        }
    }

    /// Versão clara ou escura do tema (da mesma família: Material 3 Light → Dark),
    /// mantendo a mistura por classe.
    pub fn set_dark(&mut self, dark: bool) {
        // Já no modo pedido: nada a trocar (trocar perderia um tema próprio do projeto).
        if self.theme.dark == dark {
            self.dark = dark;
            return;
        }
        // A outra versão da família: a do projeto (`themes/*.json`), a do aylis ou a padrão.
        let family = self.theme.family_name().to_string();
        let mut theme = self
            .project_themes
            .iter()
            .find(|t| t.family_name() == family && t.dark == dark)
            .or_else(|| aylis_core::themes::of_family(&family, dark).filter(|t| t.dark == dark))
            .cloned()
            .unwrap_or_else(|| if dark { Theme::dark() } else { Theme::light() });
        theme.components = self.theme.components.clone();
        // As cores e fontes que só o tema do projeto tem (as do arquivo do tema, a fonte
        // do projeto) continuam; as cores do projeto voltam no modo novo.
        for (name, color) in &self.theme.colors {
            theme.colors.entry(name.clone()).or_insert(*color);
        }
        for (name, font) in &self.theme.fonts {
            theme.fonts.entry(name.clone()).or_insert_with(|| font.clone());
        }
        theme.apply_palette(&self.palette);
        self.set_theme(theme);
        self.dark = dark;
    }

    /// Troca o tema do form desenhado (a aparência do projeto).
    pub fn set_theme(&mut self, theme: Theme) {
        self.set_shared_theme(std::sync::Arc::new(theme));
    }

    /// O mesmo, sem copiar: as abas do mesmo projeto guardam o tema dele uma vez só.
    pub fn set_shared_theme(&mut self, theme: std::sync::Arc<Theme>) {
        self.dark = theme.dark;
        self.theme = theme.clone();
        self.ui.set_shared_theme(theme);
    }

    // ----- seleção -----

    /// Seleciona só `id` (ou nada).
    pub fn select(&mut self, id: Option<ComponentId>) {
        let id = id.filter(|&i| self.ui.contains(i));
        self.selection = id;
        self.selected = id.into_iter().collect();
        self.sync_reveal();
    }

    /// Seleciona vários (o último é o principal). O form sai se houver outros.
    pub fn select_many(&mut self, ids: Vec<ComponentId>) {
        let mut ids: Vec<ComponentId> = ids.into_iter().filter(|&i| self.ui.contains(i)).collect();
        ids.dedup();
        if ids.len() > 1 {
            ids.retain(|&i| i != self.root);
        }
        if ids.is_empty() {
            ids.push(self.root);
        }
        self.selection = ids.last().copied();
        self.selected = ids;
        self.sync_reveal();
    }

    /// Ctrl/Shift + clique: põe ou tira da seleção.
    pub fn toggle(&mut self, id: ComponentId) {
        let mut ids = self.selected.clone();
        if let Some(pos) = ids.iter().position(|&s| s == id) {
            ids.remove(pos);
        } else {
            ids.push(id);
        }
        self.select_many(ids);
    }

    pub fn is_selected(&self, id: ComponentId) -> bool {
        self.selected.contains(&id)
    }

    /// Selecionados que podem ser editados juntos (sem o form).
    fn editable_selection(&self) -> Vec<ComponentId> {
        self.selected.iter().copied().filter(|&s| s != self.root && self.ui.contains(s)).collect()
    }

    /// Ctrl+A: todos os controles do mesmo pai do selecionado (ou do form).
    pub fn select_all(&mut self) {
        let parent = self.selection.filter(|&s| s != self.root).and_then(|s| self.ui.parent(s)).unwrap_or(self.root);
        let ids: Vec<ComponentId> = self.ui.children(parent).iter().copied().filter(|&c| self.is_visual(c)).collect();
        self.select_many(ids);
    }

    // ----- consultas -----

    pub fn is_visual(&self, id: ComponentId) -> bool {
        self.ui.control(id).is_some()
    }

    /// A classe cria controles visuais?
    pub fn class_is_visual(&self, class: &str) -> bool {
        let class = aylis_core::split_variant(class).0;
        self.ui.registry().get(class).is_some_and(|c| (c.create)().control().is_some())
    }

    /// Retângulo do controle em coordenadas do form.
    pub fn rect_of(&self, id: ComponentId) -> Option<RectF> {
        if id == self.root {
            return self.ui.laid_out_bounds(id).map(|b| RectF::from_size(b.size()));
        }
        self.ui.absolute_rect(id)
    }

    /// Onde `id` pode aparecer: a interseção dos contêineres acima dele que recortam o conteúdo
    /// (um `ScrollBox` rolado esconde o que saiu dele). `None`: nenhum recorta (o form não
    /// conta: o que foi arrastado para fora dele aparece apagado). Um retângulo vazio: está
    /// todo escondido.
    pub fn visible_area(&self, id: ComponentId) -> Option<RectF> {
        let mut area: Option<RectF> = None;
        let mut current = self.ui.parent(id);
        while let Some(p) = current.filter(|&p| p != self.root) {
            if self.ui.control(p).is_some_and(|cb| cb.clip_children()) {
                if let Some(r) = self.rect_of(p) {
                    area = Some(area.map_or(r, |a| a.intersect(r).unwrap_or_default()));
                }
            }
            current = self.ui.parent(p);
        }
        area
    }

    /// Giro total do controle (o dele mais o dos contêineres), em graus.
    pub fn total_rotation(&self, id: ComponentId) -> f32 {
        let mut total = 0.0;
        let mut current = Some(id);
        while let Some(n) = current.filter(|&n| n != self.root) {
            total += self.ui.control(n).map_or(0.0, |cb| cb.rotation());
            current = self.ui.parent(n);
        }
        total
    }

    /// Onde um ponto do retângulo do controle (sem giro, nas coordenadas de `rect_of`)
    /// aparece no form: com o giro (em torno do centro) e o deslocamento (`translate`) do
    /// controle e dos contêineres dele, como a pintura faz.
    pub fn rotated_point(&self, id: ComponentId, p: PointF) -> PointF {
        let mut p = p;
        let mut current = Some(id);
        while let Some(n) = current.filter(|&n| n != self.root) {
            if let (Some(cb), Some(r)) = (self.ui.control(n), self.rect_of(n)) {
                if cb.rotation() != 0.0 {
                    p = rotate_about(p, r.center(), cb.rotation());
                }
                p = p + cb.translate();
            }
            current = self.ui.parent(n);
        }
        p
    }

    /// Onde o ponto `p` de um controle aparece no form se o retângulo dele (sem giro) fosse
    /// `rect`: o giro dele em torno do centro de `rect`, o `translate` e os contêineres.
    fn place_point(&self, id: ComponentId, rect: RectF, p: PointF) -> PointF {
        let Some(cb) = self.ui.control(id) else { return p };
        let mut p = if cb.rotation() != 0.0 { rotate_about(p, rect.center(), cb.rotation()) } else { p };
        p = p + cb.translate();
        match self.ui.parent(id) {
            Some(parent) if parent != self.root => self.rotated_point(parent, p),
            _ => p,
        }
    }

    /// Redimensiona pela alça com o arrasto `delta` na tela (coordenadas do form). Girado,
    /// o arrasto vale no eixo do controle e o lado oposto à alça fica parado na tela (o
    /// giro é em torno do centro: sem compensar, esticar um lado arrastaria o outro).
    pub fn resize_rotated(&mut self, id: ComponentId, handle: Handle, start: RectF, delta: PointF) {
        let turn = self.total_rotation(id);
        if turn == 0.0 {
            self.resize_from(id, handle, start, delta);
            return;
        }
        // `start` é relativo ao pai; o giro é calculado nas coordenadas do form.
        let parent_origin = self.ui.parent(id).and_then(|p| self.rect_of(p)).map_or(PointF::ZERO, |r| r.origin());
        let start_abs = RectF::new(start.x + parent_origin.x, start.y + parent_origin.y, start.width, start.height);
        let anchor = handle.opposite();
        let before = self.place_point(id, start_abs, anchor.point(start_abs));
        self.resize_with(id, handle, start, rotate_about(delta, PointF::ZERO, -turn), false);
        let Some(now) = self.rect_of(id) else { return };
        let after = self.place_point(id, now, anchor.point(now));
        // O que andou na tela, no sistema do pai (sem o giro dos contêineres).
        let parent_turn = self.ui.parent(id).filter(|&p| p != self.root).map_or(0.0, |p| self.total_rotation(p));
        let fix = rotate_about(before - after, PointF::ZERO, -parent_turn);
        if let Some(cb) = self.ui.control_mut(id) {
            let b = cb.bounds();
            cb.set_position(b.x + fix.x, b.y + fix.y);
        }
        self.ui.layout();
    }

    /// Componente mais de cima no ponto (coordenadas do form). Diferente do hit test da
    /// execução, aqui `hit_test = false` não impede de selecionar (um Label é clicável).
    pub fn component_at(&self, p: PointF) -> Option<ComponentId> {
        self.hit(self.root, p)
    }

    fn hit(&self, id: ComponentId, pos_in_parent: PointF) -> Option<ComponentId> {
        // O que está dentro de um frame não se seleciona aqui: é o próprio frame.
        let frame = self.ui.class_name(id) == Some("Frame");
        let cb = self.ui.control(id)?;
        if !cb.visible() {
            return None;
        }
        let b = if id == self.root { RectF::from_size(cb.bounds().size()) } else { cb.bounds() };
        let mut local = pos_in_parent - b.origin();
        // Girado (ou deslocado): o clique vale onde o controle aparece, não no retângulo
        // sem giro.
        if id != self.root {
            local = local - cb.translate();
            if cb.rotation() != 0.0 {
                let center = PointF::new(b.width / 2.0, b.height / 2.0);
                local = rotate_about(local, center, -cb.rotation());
            }
        }
        let inside = local.x >= 0.0 && local.y >= 0.0 && local.x < b.width && local.y < b.height;
        // Fora do form também vale: o que foi arrastado para fora aparece apagado e dá para
        // pegar de volta.
        if cb.clip_children() && !inside && id != self.root {
            return None;
        }
        // Os menus abertos não existem no designer; a camada do menu nunca aparece aqui.
        if !frame {
            if let Some(hit) = self.ui.children(id).iter().rev().find_map(|&c| self.hit(c, local)) {
                return Some(hit);
            }
        }
        inside.then_some(id)
    }

    /// Controles visuais (não o form) que o retângulo toca (seleção por área, como no Delphi),
    /// a partir do contêiner onde ele começou. Um contêiner coberto inteiro entra como um só; o
    /// que ele só corta dá lugar aos de dentro que o retângulo toca (no NoOBS, o campo de busca
    /// e os botões de uma barra da largura toda, com um retângulo fino passando por eles;
    /// relatado pelo autor em 07/11) e entra ele mesmo se não tocar nenhum. Escondidos, os que a
    /// rolagem tirou da vista e o miolo de um frame ficam de fora.
    pub fn components_in(&self, area: RectF) -> Vec<ComponentId> {
        let parent = self.component_at(area.origin()).map_or(self.root, |c| self.container_of(c));
        let mut found = Vec::new();
        self.collect_in(parent, area, &mut found);
        found
    }

    fn collect_in(&self, parent: ComponentId, area: RectF, found: &mut Vec<ComponentId>) {
        for &c in self.ui.children(parent) {
            if !self.is_visual(c) || !self.ui.control(c).is_some_and(|cb| cb.visible()) {
                continue;
            }
            let Some(r) = self.rect_of(c) else { continue };
            // Só a parte que aparece conta (o resto de um ScrollBox rolado não se toca).
            let r = match self.visible_area(c) {
                Some(a) => match a.intersect(r) {
                    Some(v) if !v.is_empty() => v,
                    _ => continue,
                },
                None => r,
            };
            if !area.intersects(r) {
                continue;
            }
            let container = self.ui.class_name(c).is_some_and(|n| CONTAINERS.contains(&n));
            if area.contains_rect(r) || !container {
                found.push(c);
                continue;
            }
            let before = found.len();
            self.collect_in(c, area, found);
            if found.len() == before {
                found.push(c);
            }
        }
    }

    fn container_of(&self, id: ComponentId) -> ComponentId {
        let mut current = Some(id);
        while let Some(c) = current {
            if self.ui.class_name(c).is_some_and(|n| CONTAINERS.contains(&n)) {
                return c;
            }
            current = self.ui.parent(c);
        }
        self.root
    }

    /// Onde um controle novo deve entrar: o contêiner sob o ponto (ou o form).
    pub fn container_at(&self, p: PointF) -> ComponentId {
        self.component_at(p).map_or(self.root, |c| self.container_of(c))
    }

    /// Componentes não visuais do form (vão para a "bandeja" do designer). Os que têm
    /// dono (Field no ClientDataSet, animação no controle, item de menu) aparecem só na
    /// estrutura.
    pub fn non_visual(&self) -> Vec<ComponentId> {
        self.ui.children(self.root).iter().copied().filter(|&id| !self.is_visual(id)).collect()
    }

    /// A árvore inteira com a profundidade de cada componente.
    pub fn tree(&self) -> Vec<(ComponentId, usize)> {
        let mut out = Vec::new();
        self.walk(self.root, 0, &mut out);
        out
    }

    fn walk(&self, id: ComponentId, depth: usize, out: &mut Vec<(ComponentId, usize)>) {
        out.push((id, depth));
        // O conteúdo de um frame é do form dele.
        if self.ui.class_name(id) == Some("Frame") {
            return;
        }
        for &c in self.ui.children(id) {
            self.walk(c, depth + 1, out);
        }
    }

    /// `Button1`, `Button2`... (como o Delphi).
    pub fn unique_name(&self, class: &str) -> String {
        (1..).map(|n| format!("{class}{n}")).find(|name| self.ui.find_in(self.root, name).is_none()).expect("sempre há um número livre")
    }

    /// Eventos ligados a handlers por nome: (componente, evento, handler).
    pub fn handlers(&self) -> Vec<(String, String, String)> {
        let mut out = Vec::new();
        for (id, _) in self.tree() {
            for info in self.ui.prop_infos(id) {
                if info.kind == aylis_core::PropKind::Event {
                    if let Ok(PropValue::Event(Some(handler))) = self.ui.get_prop(id, info.name) {
                        out.push((self.ui.name(id).unwrap_or_default().to_string(), info.name.to_string(), handler));
                    }
                }
            }
        }
        out
    }

    /// Nomes de handler já usados no form (para escolher um existente no inspector).
    pub fn handler_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.handlers().into_iter().map(|(_, _, h)| h).collect();
        names.sort();
        names.dedup();
        names
    }

    /// Os candidatos de uma propriedade que aponta para outro componente: só os do tipo que
    /// ela aceita (`data_set` lista os datasets, `popup_menu` os `PopupMenu`), nunca o form nem
    /// o próprio componente. Uma propriedade desconhecida aceita qualquer componente.
    pub fn reference_candidates_for(&self, from: ComponentId, prop: &str) -> Vec<String> {
        let is_a = |id: ComponentId, class: &str| {
            let mut chain = Vec::new();
            if let Some(c) = self.ui.component(id) {
                c.class_chain(&mut chain);
            }
            chain.contains(&class)
        };
        let accepts = |id: ComponentId| -> bool {
            if id == from || self.ui.parent(id).is_none() {
                return false;
            }
            match prop {
                "popup_menu" => is_a(id, "PopupMenu"),
                "data_source" | "master_source" | "list_source" => is_a(id, "DataSource"),
                "data_set" | "lookup_data_set" => is_a(id, "ClientDataSet"),
                "report" => is_a(id, "Report"),
                "active_control" => self.ui.control(id).is_some_and(|c| c.can_focus()),
                "target" => self.is_visual(id),
                _ => true,
            }
        };
        self.reference_candidates(from).into_iter().filter(|name| self.ui.find_path(name).is_some_and(accepts)).collect()
    }

    /// Componentes com nome que podem ser alvo de uma referência (todos, menos `from`).
    pub fn reference_candidates(&self, from: ComponentId) -> Vec<String> {
        let mut out: Vec<String> =
            self.tree().into_iter().filter(|&(id, _)| id != from).filter_map(|(id, _)| self.ui.name(id).map(str::to_string)).filter(|n| !n.is_empty()).collect();
        // Os componentes dos DataModules usados, pelo caminho (`dmDados.cdsClientes`).
        for &module in self.ui.roots().iter().filter(|&&r| r != self.root) {
            let mut stack: Vec<ComponentId> = self.ui.children(module).to_vec();
            while let Some(id) = stack.pop() {
                if self.ui.name(id).is_some_and(|n| !n.is_empty()) {
                    out.push(self.ui.path_of(id));
                }
                stack.extend(self.ui.children(id).iter().copied());
            }
        }
        out
    }

    // ----- edição -----

    /// Cria um componente da paleta. Visuais entram no contêiner sob `at` (coordenadas do
    /// form), na grade; não visuais entram no form — ou no componente selecionado, se ele
    /// fizer sentido como dono (um `Field` num `ClientDataSet`, uma animação num controle,
    /// um item num menu).
    /// `class` pode ser um subtipo (`"Button.icon"`).
    /// As propriedades do form que a prévia da vista muda (não entram nas mudanças).
    const PREVIEW_PROPS: [&'static str; 4] = ["left", "top", "width", "height"];

    /// A largura máxima padrão de uma vista nova pelo nome.
    pub fn default_view_width(name: &str) -> f32 {
        match name {
            "phone" => 599.0,
            "tablet" => 1023.0,
            _ => 1279.0,
        }
    }

    /// O tamanho da prévia de uma vista no designer: o de um aparelho típico no celular (390)
    /// e no tablet (820); nas outras, um tamanho dentro das condições dela (a altura `None`:
    /// a do form).
    pub fn preview_size(&self, view: &aylis_core::FormView) -> (f32, Option<f32>) {
        let form = self.ui.control(self.root).map_or((1280.0, 800.0), |cb| (cb.width(), cb.height()));
        let width = match view.name.as_str() {
            "phone" if view.max_width >= 390.0 || view.max_width <= 0.0 => 390.0,
            "tablet" if view.max_width >= 820.0 || view.max_width <= 0.0 => 820.0,
            _ if view.max_width > 0.0 => view.max_width.max(view.min_width),
            _ if view.min_width > 0.0 => view.min_width.max(form.0),
            _ => form.0,
        };
        let mut height = match (view.min_height > 0.0, view.max_height > 0.0) {
            (_, true) => Some(view.max_height.max(view.min_height)),
            (true, false) => Some(view.min_height.max(form.1)),
            _ => None,
        };
        let h = height.unwrap_or(form.1);
        match view.orientation {
            aylis_core::ViewOrientation::Portrait if h <= width => height = Some((width * 16.0 / 9.0).round()),
            aylis_core::ViewOrientation::Landscape if h > width => height = Some((width * 9.0 / 16.0).round()),
            _ => {}
        }
        (width, height)
    }

    /// Mostra e edita o form na vista `name` (vazio: o base), no tamanho da prévia dela.
    pub fn enter_named_view(&mut self, name: &str) {
        if name.is_empty() {
            self.leave_view();
            return;
        }
        // Celular e Tablet existem antes de mexer: são criados ao entrar.
        let view = self.views().into_iter().find(|v| v.name == name).unwrap_or_else(|| aylis_core::FormView::new(name, Self::default_view_width(name)));
        let (width, height) = self.preview_size(&view);
        self.enter_view_sized(Some(name), width, height);
    }

    /// Mostra e edita o form numa vista (`Some("phone")`, com a prévia em `width` pontos) ou
    /// volta ao base (`None`). O que mudar de posição, tamanho e visibilidade enquanto isso
    /// vira mudança da vista; o form continua com os valores base.
    pub fn enter_view(&mut self, name: Option<&str>, width: f32) {
        self.enter_view_sized(name, width, None);
    }

    /// Mostra o form no aparelho `device` (deitado ou não): a vista pela largura dele (até
    /// 599 o celular, até 1023 o tablet), no tamanho dele, com a área segura simulada. Numa
    /// tela de computador, o form fica como uma janela nela. `None`: sem aparelho.
    pub fn show_device(&mut self, device: Option<(&'static aylis_core::devices::Device, bool)>) {
        self.device = device;
        let root = self.root;
        let Some((d, landscape)) = device else {
            self.ui.set_safe_area(root, aylis_core::Bounds::default());
            self.leave_view();
            return;
        };
        let size = d.size(landscape);
        let view = match size.width {
            w if w <= Self::default_view_width("phone") => Some("phone"),
            w if w <= Self::default_view_width("tablet") => Some("tablet"),
            _ => None,
        };
        match view.filter(|_| d.kind != aylis_core::devices::DeviceKind::Desktop) {
            Some(view) => self.enter_view_sized(Some(view), size.width, Some(size.height)),
            None => self.leave_view(),
        }
        self.ui.set_safe_area(root, d.safe_area(landscape));
        self.ui.layout();
    }

    /// Como [`enter_view`](Self::enter_view), com a altura da prévia (a do aparelho).
    pub fn enter_view_sized(&mut self, name: Option<&str>, width: f32, height: Option<f32>) {
        self.leave_view();
        let Some(name) = name.filter(|n| !n.is_empty()) else { return };
        let root = self.root;
        let views = self.ui.views(root).to_vec();
        if !views.iter().any(|v| v.name == name) {
            let mut views = views;
            views.push(aylis_core::FormView::new(name, Self::default_view_width(name)));
            self.ui.set_views(root, views);
        }
        // Os valores base (com o base à mostra) de todas as propriedades gravadas e os filhos
        // de cada contêiner: o que ficar diferente na vista vira mudança dela.
        self.ui.force_view(root, Some(""));
        self.ui.layout();
        let mut base = Vec::new();
        let mut base_children = Vec::new();
        for id in self.ui.components().filter(|&c| self.ui.form_of(c) == root).collect::<Vec<_>>() {
            if !self.ui.children(id).is_empty() {
                base_children.push((id, self.ui.children(id).to_vec()));
            }
            let component = if id == root { String::new() } else { self.ui.name(id).unwrap_or("").to_string() };
            if id != root && component.is_empty() {
                continue;
            }
            for info in self.ui.prop_infos(id) {
                if Self::view_skips(&self.ui, id, &info) {
                    continue;
                }
                if let Ok(v) = self.ui.get_prop(id, info.name) {
                    base.push((id, component.clone(), info.name.to_string(), v));
                }
            }
        }
        let root_width = self.ui.get_prop(root, "width").unwrap_or(PropValue::Float(0.0));
        let root_height = self.ui.get_prop(root, "height").unwrap_or(PropValue::Float(0.0));
        let _ = self.ui.set_prop(root, "width", width);
        if let Some(h) = height {
            let _ = self.ui.set_prop(root, "height", h);
        }
        self.ui.force_view(root, Some(name));
        self.ui.layout();
        self.view_mode = Some(ViewMode { name: name.to_string(), width, height, base, base_children, root_width, root_height });
        self.sync_view();
    }

    /// As propriedades que a vista não acompanha: as que não são gravadas, os eventos, as
    /// referências a componentes, as ligadas a dados e, no form, a posição e o tamanho (os da
    /// prévia).
    fn view_skips(ui: &Ui, id: ComponentId, info: &aylis_core::PropInfo) -> bool {
        info.read_only
            || info.runtime
            || matches!(info.kind, aylis_core::PropKind::Event | aylis_core::PropKind::Component)
            || info.name == "name"
            || ui.binding(id, info.name).is_some()
            || (ui.parent(id).is_none() && Self::PREVIEW_PROPS.contains(&info.name))
    }

    /// Guarda na vista em edição o que está diferente do base (chamado depois de cada mudança
    /// no designer): as propriedades, e o contêiner e a ordem de quem mudou de lugar.
    pub fn sync_view(&mut self) {
        let Some(mode) = &self.view_mode else { return };
        let root = self.root;
        let mut changes = Vec::new();
        let mut base = vec![(root, "width".to_string(), mode.root_width.clone()), (root, "height".to_string(), mode.root_height.clone())];
        for (id, component, prop, value) in &mode.base {
            let Ok(now) = self.ui.get_prop(*id, prop) else { continue };
            if &now != value {
                changes.push(aylis_core::ViewChange { component: component.clone(), property: prop.clone(), value: now });
                base.push((*id, prop.clone(), value.clone()));
            }
        }
        let moves = self.view_moves(&mut changes);
        self.ui.update_active_view(root, changes, base, moves);
    }

    /// Quem mudou de contêiner ou de ordem em relação ao base: acrescenta as mudanças
    /// (`parent` e `index`) e devolve os lugares de antes, numa ordem que, desfeita do fim
    /// para o começo, volta ao base (veja `aylis_core::views`).
    fn view_moves(&self, changes: &mut Vec<aylis_core::ViewChange>) -> Vec<(ComponentId, ComponentId, usize)> {
        use aylis_core::views::{INDEX, PARENT};
        let Some(mode) = &self.view_mode else { return Vec::new() };
        let ui = &self.ui;
        let base_parent = |id: ComponentId| mode.base_children.iter().find(|(_, list)| list.contains(&id)).map(|(p, _)| *p);
        // Os que mudaram de contêiner e os que ficaram mas mudaram de ordem entre os que ficaram.
        let mut moved: Vec<ComponentId> = Vec::new();
        for (parent, list) in &mode.base_children {
            if !ui.contains(*parent) {
                continue;
            }
            let stayed_base: Vec<ComponentId> = list.iter().copied().filter(|&c| ui.parent(c) == Some(*parent)).collect();
            let stayed_now: Vec<ComponentId> = ui.children(*parent).iter().copied().filter(|&c| base_parent(c) == Some(*parent)).collect();
            for &c in list {
                if ui.contains(c) && ui.parent(c) != Some(*parent) && ui.form_of(c) == self.root {
                    moved.push(c);
                }
            }
            if stayed_base != stayed_now {
                moved.extend(stayed_now.iter().zip(&stayed_base).filter(|(a, b)| a != b).map(|(a, _)| *a));
            }
        }
        // Só quem tem nome entra na vista (a mudança é pelo nome).
        moved.retain(|&c| ui.name(c).is_some_and(|n| !n.is_empty()));
        let index_now = |c: ComponentId| ui.parent(c).and_then(|p| ui.children(p).iter().position(|&x| x == c)).unwrap_or(0);
        moved.sort_by_key(|&c| (ui.parent(c), index_now(c)));
        moved.dedup();
        // Simula do base até agora, anotando o lugar de antes de cada passo.
        let mut lists: Vec<(ComponentId, Vec<ComponentId>)> = mode.base_children.clone();
        let mut moves = Vec::new();
        for &c in &moved {
            let Some(to) = ui.parent(c) else { continue };
            let Some((from, at)) = lists.iter().find_map(|(p, l)| l.iter().position(|&x| x == c).map(|i| (*p, i))) else { continue };
            moves.push((c, from, at));
            if let Some((_, l)) = lists.iter_mut().find(|(p, _)| *p == from) {
                l.remove(at);
            }
            let index = index_now(c);
            match lists.iter_mut().find(|(p, _)| *p == to) {
                Some((_, l)) => l.insert(index.min(l.len()), c),
                None => lists.push((to, vec![c])),
            }
            let name = ui.name(c).unwrap_or("").to_string();
            if base_parent(c) != Some(to) {
                let parent_name = if to == self.root { String::new() } else { ui.name(to).unwrap_or("").to_string() };
                changes.push(aylis_core::ViewChange { component: name.clone(), property: PARENT.into(), value: PropValue::String(parent_name) });
            }
            changes.push(aylis_core::ViewChange { component: name, property: INDEX.into(), value: PropValue::Int(index as i64) });
        }
        moves
    }

    /// Volta ao base: o form mostra os valores base, na largura dele.
    pub fn leave_view(&mut self) {
        if self.view_mode.is_none() {
            return;
        }
        self.sync_view();
        self.view_mode = None;
        // Sem a vista forçada pelo designer, o base à mostra (a largura de volta).
        self.ui.force_view(self.root, Some(""));
        self.ui.layout();
    }

    /// Passa `id` para dentro de `parent`, na posição `index` entre os filhos (`None`: no fim),
    /// num passo do desfazer (M16 do Tabernáculo: trocar de pai sem refazer o layout inteiro).
    pub fn move_component(&mut self, id: ComponentId, parent: ComponentId, index: Option<usize>) -> Result<(), String> {
        if id == self.root {
            return Err("o form não muda de pai".into());
        }
        let mut up = Some(parent);
        while let Some(p) = up {
            if p == id {
                return Err("um componente não vai para dentro dele mesmo".into());
            }
            up = self.ui.parent(p);
        }
        self.begin_change();
        if self.ui.parent(id) != Some(parent) {
            self.ui.set_parent(id, Some(parent)).map_err(|e| e.to_string())?;
        }
        if let Some(i) = index {
            let _ = self.ui.set_child_index(id, i);
        }
        self.ui.layout();
        Ok(())
    }

    /// Tira as mudanças de um componente da vista em edição: ele volta ao base nela.
    pub fn view_back_to_base(&mut self, id: ComponentId) -> bool {
        let Some(mode) = &self.view_mode else { return false };
        let values: Vec<(String, PropValue)> = mode.base.iter().filter(|(c, ..)| *c == id).map(|(_, _, p, v)| (p.clone(), v.clone())).collect();
        // O lugar no base (o contêiner e a posição entre os irmãos que estão lá agora).
        let place = mode.base_children.iter().find(|(_, l)| l.contains(&id)).map(|(p, l)| {
            let before = l.iter().take_while(|&&c| c != id).filter(|&&c| self.ui.parent(c) == Some(*p)).count();
            (*p, before)
        });
        if values.is_empty() && place.is_none() {
            return false;
        }
        self.begin_change();
        for (prop, value) in values {
            let _ = self.ui.set_prop(id, &prop, value);
        }
        if let Some((parent, index)) = place {
            if self.ui.parent(id) != Some(parent) {
                let _ = self.ui.set_parent(id, Some(parent));
            }
            let _ = self.ui.set_child_index(id, index);
        }
        self.ui.layout();
        self.sync_view();
        true
    }

    /// As vistas do form aberto (na ordem de escolha).
    pub fn views(&self) -> Vec<aylis_core::FormView> {
        self.ui.views(self.root).to_vec()
    }

    /// Troca as vistas do form (um passo do desfazer): cada uma com o nome que tinha (`None`:
    /// nova), as condições e as escalas; as mudanças vêm da vista de antes com aquele nome. A
    /// que ficou de fora é apagada. A vista em edição continua (pelo novo nome, se foi
    /// renomeada; apagada, volta ao base).
    pub fn set_views(&mut self, list: Vec<(Option<String>, aylis_core::FormView)>) -> Result<(), String> {
        let editing = self.view_mode.as_ref().map(|m| (m.name.clone(), m.width, m.height));
        let renames: Vec<(String, String)> = list.iter().filter_map(|(old, v)| old.clone().map(|o| (o, v.name.trim().to_string()))).collect();
        let mut views: Vec<aylis_core::FormView> = list.into_iter().map(|(_, v)| v).collect();
        for v in views.iter_mut() {
            v.name = v.name.trim().to_string();
        }
        let mut names: Vec<&str> = views.iter().map(|v| v.name.as_str()).collect();
        if names.iter().any(|n| n.is_empty()) {
            return Err("a vista precisa de um nome".into());
        }
        names.sort_unstable();
        if names.windows(2).any(|w| w[0] == w[1]) {
            return Err("já existe uma vista com esse nome".into());
        }
        // Fora da vista (as mudanças dela guardadas), muda a lista e volta a ela.
        self.leave_view();
        // As mudanças de cada vista são as de agora (as da vista em edição acabaram de ser
        // guardadas); uma nova começa sem nenhuma.
        let current = self.ui.views(self.root).to_vec();
        for v in views.iter_mut() {
            v.changes = renames
                .iter()
                .find(|(_, new)| *new == v.name)
                .and_then(|(old, _)| current.iter().find(|c| c.name == *old))
                .map(|c| c.changes.clone())
                .unwrap_or_default();
        }
        self.begin_change();
        self.ui.set_views(self.root, views);
        self.ui.force_view(self.root, Some(""));
        self.ui.layout();
        if let Some((name, width, height)) = editing {
            let name = renames.iter().find(|(old, _)| *old == name).map_or(name, |(_, new)| new.clone());
            if self.ui.views(self.root).iter().any(|v| v.name == name) {
                self.enter_view_sized(Some(&name), width, height);
            }
        }
        Ok(())
    }

    /// Liga dois componentes com um `Pipe` ou um `Wire` (`class`), arrastado no designer de
    /// `from` (apertado em `from_point`) até `to` (solto em `to_point`): cada ponta no lado do
    /// componente mais perto do ponto. Um passo do desfazer; devolve a ligação.
    pub fn connect(&mut self, class: &str, from: ComponentId, from_point: PointF, to: ComponentId, to_point: PointF) -> Result<ComponentId, String> {
        if from == to {
            return Err("solte sobre outro componente".into());
        }
        let side = |rect: Option<RectF>, p: PointF| {
            let Some(r) = rect else { return "right" };
            let (x, y) = ((p.x - r.x) / r.width.max(1.0), (p.y - r.y) / r.height.max(1.0));
            [(x, "left"), (1.0 - x, "right"), (y, "top"), (1.0 - y, "bottom")].into_iter().min_by(|a, b| a.0.total_cmp(&b.0)).map_or("right", |s| s.1)
        };
        let (from_port, to_port) = (side(self.rect_of(from), from_point), side(self.rect_of(to), to_point));
        // No mesmo pai do componente de partida (o caminho é calculado nas coordenadas dele).
        let parent = self.ui.parent(from).unwrap_or(self.root);
        self.begin_change();
        let id = self.ui.create(class, Some(parent)).map_err(|e| e.to_string())?;
        let name = self.unique_name(class);
        let _ = self.ui.set_name(id, &name);
        for (prop, value) in [("from", PropValue::Component(Some(from))), ("to", PropValue::Component(Some(to))), ("from_port", PropValue::Enum(from_port.into())), ("to_port", PropValue::Enum(to_port.into()))] {
            self.ui.set_prop(id, prop, value).map_err(|e| e.to_string())?;
        }
        self.ui.layout();
        self.select(Some(id));
        Ok(id)
    }

    pub fn add(&mut self, class: &str, at: Option<PointF>) -> Result<ComponentId, String> {
        self.add_in(class, at, None)
    }

    /// Como [`Document::add`], com o contêiner escolhido (`parent`) para os visuais, em vez do
    /// que está sob o ponto: o canto do pai podia cair num filho dele (o relatório do
    /// Tabernáculo, M22).
    pub fn add_in(&mut self, class: &str, at: Option<PointF>, parent: Option<ComponentId>) -> Result<ComponentId, String> {
        let forced = parent;
        // Um pacote (`pkg:Termometro`): um Frame com o form do pacote, nomeado pelo pacote.
        if let Some(package) = aylis_project::packages::package_of(class) {
            let package = package.to_string();
            let id = self.add_in("Frame", at, forced)?;
            let name = self.unique_name(&package);
            let _ = self.ui.set_name(id, &name);
            let _ = self.ui.set_prop(id, "form", class);
            self.refresh_frames();
            return Ok(id);
        }
        // `Classe@Estilo` (a paleta de uma plataforma): o componente fica com o tema dela.
        let (class, look) = match class.split_once('@') {
            Some((c, t)) => (c, Some(t)),
            None => (class, None),
        };
        let visual = self.class_is_visual(class);
        if visual && self.is_data_module() {
            return Err("um DataModule só aceita componentes não visuais".into());
        }
        let full = class;
        let class = aylis_core::split_variant(class).0;
        let parent = if visual {
            forced.unwrap_or_else(|| at.map_or(self.root, |p| self.container_at(p)))
        } else {
            let selected = self.selection.unwrap_or(self.root);
            let selected_class = self.ui.class_name(selected).unwrap_or_default();
            match class {
                "Field" if selected_class == "ClientDataSet" => selected,
                "Field" => return Err("selecione um ClientDataSet antes de adicionar um Field".into()),
                "Column" if selected_class == "DataGrid" => selected,
                "Column" => return Err("selecione um DataGrid antes de adicionar uma Column".into()),
                "FloatAnimation" | "ColorAnimation" if self.is_visual(selected) && selected != self.root => selected,
                "FloatAnimation" | "ColorAnimation" => return Err("selecione o controle que a animação vai animar".into()),
                "MenuItem" if matches!(selected_class, "MainMenu" | "PopupMenu" | "MenuItem") => selected,
                "MenuItem" => return Err("selecione um MainMenu, PopupMenu ou MenuItem antes de adicionar um item".into()),
                _ => self.root,
            }
        };
        self.begin_change();
        let id = self.ui.create(full, Some(parent)).map_err(|e| e.to_string())?;
        let name = self.unique_name(class);
        let _ = self.ui.set_name(id, &name);
        if let Some(look) = look {
            let _ = self.ui.set_prop(id, "theme", look);
        }
        // Texto inicial = nome (como o Caption no Delphi), menos quando o subtipo já
        // define o texto (o botão de ícone fica sem texto).
        let (_, variant) = aylis_core::split_variant(full);
        let preset: Vec<String> = variant
            .and_then(|v| self.ui.registry().variant(class, v))
            .map(|v| v.props.iter().map(|(k, _)| k.clone()).collect())
            .unwrap_or_default();
        for prop in ["text", "caption"] {
            if preset.iter().any(|p| p == prop) {
                continue;
            }
            if let Ok(PropValue::String(current)) = self.ui.get_prop(id, prop) {
                if current.is_empty() {
                    let _ = self.ui.set_prop(id, prop, name.as_str());
                }
            }
        }
        if let (true, Some(p)) = (visual, at) {
            let origin = self.rect_of(parent).map_or(PointF::ZERO, |r| r.origin());
            if let Some(cb) = self.ui.control_mut(id) {
                if cb.align() == Align::None {
                    cb.set_position(snap(p.x - origin.x), snap(p.y - origin.y));
                }
            }
        }
        self.ui.layout();
        self.select(Some(id));
        Ok(id)
    }

    /// Exclui os selecionados (o form não pode ser excluído).
    pub fn delete_selected(&mut self) -> bool {
        let ids = self.editable_selection();
        if ids.is_empty() {
            return false;
        }
        self.begin_change();
        let parent = self.ui.parent(ids[0]);
        for id in ids {
            self.ui.free(id);
        }
        self.ui.layout();
        self.select(parent.or(Some(self.root)));
        true
    }

    /// Grava uma propriedade a partir de um valor JSON (como digitado no Object Inspector).
    /// Liga a propriedade a dados ("Ligar a campo…"), ou desliga com `path` vazio, num passo do
    /// desfazer. `source`: o nome do `DataSource`.
    pub fn set_binding(&mut self, id: ComponentId, property: &str, source: &str, path: &str) -> Result<(), String> {
        let source = if path.trim().is_empty() {
            None
        } else {
            Some(self.ui.resolve_reference(id, source).ok_or_else(|| format!("'{source}' não existe no form"))?)
        };
        self.begin_change();
        self.ui.set_binding(id, property, source, path);
        Ok(())
    }

    /// Os `DataSource` do form (os nomes).
    pub fn data_sources(&self) -> Vec<String> {
        self.tree()
            .into_iter()
            .filter(|(id, _)| self.ui.class_name(*id) == Some("DataSource"))
            .filter_map(|(id, _)| self.ui.name(id).map(str::to_string))
            .filter(|n| !n.is_empty())
            .collect()
    }

    /// Os campos do dataset de um `DataSource` (pelo nome).
    pub fn fields_of(&self, source: &str) -> Vec<String> {
        let Some(ds) = self.ui.find_in(self.root, source) else { return Vec::new() };
        let Ok(PropValue::Component(Some(cds))) = self.ui.get_prop(ds, "data_set") else { return Vec::new() };
        self.ui
            .children(cds)
            .iter()
            .filter(|&&c| self.ui.class_name(c) == Some("Field"))
            .filter_map(|&c| match self.ui.get_prop(c, "field_name") {
                Ok(PropValue::String(n)) if !n.is_empty() => Some(n),
                _ => None,
            })
            .collect()
    }

    pub fn set_prop(&mut self, id: ComponentId, name: &str, value: &Value) -> Result<(), String> {
        self.set_prop_many(&[id], name, value)
    }

    /// Grava a mesma propriedade em vários componentes (seleção múltipla), num passo só
    /// do desfazer.
    pub fn set_prop_many(&mut self, ids: &[ComponentId], name: &str, value: &Value) -> Result<(), String> {
        let mut changes = Vec::new();
        for &id in ids {
            // "margins.left": um lado das bordas, sempre um número.
            let new = if name.contains('.') {
                PropValue::Float(value.as_f64().ok_or("esperado um número")?)
            } else {
                aylis_json::prop_from_json(&self.ui, id, name, value)?
            };
            let current = self.ui.get_prop(id, name).ok();
            let same = match (&current, &new) {
                (Some(PropValue::Component(Some(c))), PropValue::String(n)) => self.ui.resolve_reference(id, n) == Some(*c),
                (Some(a), b) => a == b,
                _ => false,
            };
            if !same {
                changes.push((id, new));
            }
        }
        if changes.is_empty() {
            return Ok(());
        }
        self.begin_change();
        // Um texto corrigido leva as traduções dele (O02 do Tabernáculo).
        let texts: Vec<(String, String)> = if aylis_core::i18n::TRANSLATABLE.contains(&name) && !matches!(name, "items" | "labels" | "display_values") {
            changes.iter().filter_map(|(id, new)| match (self.ui.get_prop(*id, name), new) {
                (Ok(PropValue::String(old)), PropValue::String(new)) => Some((old, new.clone())),
                _ => None,
            }).collect()
        } else {
            Vec::new()
        };
        for (id, new) in changes {
            if let Err(e) = self.ui.set_prop(id, name, new) {
                // Desfaz: volta ao estado de antes da alteração.
                if let Some(previous) = self.undo.pop() {
                    self.restore(previous);
                    self.modified = !self.undo.is_empty();
                }
                return Err(e.to_string());
            }
        }
        // Trocou o form de um Frame: mostra o conteúdo novo.
        if name == "form" {
            self.refresh_frames();
        }
        if let Some(dir) = self.project_dir() {
            for (old, new) in &texts {
                aylis_project::i18n::follow_text_change(&dir, old, new);
            }
        }
        self.ui.layout();
        Ok(())
    }

    /// A pasta do projeto do form (a que tem o `.aylisproj`, subindo a partir do arquivo).
    fn project_dir(&self) -> Option<PathBuf> {
        let file = self.path.as_ref()?;
        file.ancestors().skip(1).take(4).find(|d| std::fs::read_dir(d).is_ok_and(|mut e| e.any(|f| f.ok().is_some_and(|f| f.path().extension().is_some_and(|x| x == "aylisproj"))))).map(Path::to_path_buf)
    }

    /// Mostra de novo o conteúdo dos frames (o form de um frame mudou ou foi salvo).
    pub fn refresh_frames(&mut self) {
        let Some(source) = self.frame_source.clone() else { return };
        let mut loading = vec![self.ui.name(self.root).unwrap_or_default().to_string()];
        embed_frames(&mut self.ui, self.root, &*source, &mut loading);
        self.ui.layout();
    }

    /// Renomeia um componente (pelo Object Inspector). As referências dentro do form seguem
    /// sozinhas (são pelo componente); as mudanças das vistas, que guardam o nome, também.
    /// As de outros forms e do código: [`crate::rename`].
    pub fn rename_component(&mut self, id: ComponentId, name: &str) -> Result<(), String> {
        let old = self.ui.name(id).unwrap_or_default().to_string();
        if old == name {
            return Ok(());
        }
        self.begin_change();
        if let Err(e) = self.ui.set_name(id, name) {
            self.undo.pop();
            return Err(e.to_string());
        }
        let root = self.root;
        let mut views = self.ui.views(root).to_vec();
        let mut changed = false;
        for change in views.iter_mut().flat_map(|v| v.changes.iter_mut()) {
            if !old.is_empty() && change.component == old {
                change.component = name.to_string();
                changed = true;
            }
        }
        if changed {
            self.ui.set_views(root, views);
        }
        Ok(())
    }

    /// Posição e tamanho pedidos (left, top, width, height).
    fn requested(&self, id: ComponentId) -> Option<RectF> {
        let cb = self.ui.control(id)?;
        Some(RectF::new(cb.left(), cb.top(), cb.width(), cb.height()))
    }

    /// O controle pode ser arrastado (só `align: none`, e com os controles destravados).
    pub fn movable(&self, id: ComponentId) -> bool {
        !self.locked && id != self.root && self.ui.control(id).is_some_and(|cb| cb.align() == Align::None)
    }

    /// Move a partir da posição `start` (pedida) somando `delta`, na grade.
    pub fn move_from(&mut self, id: ComponentId, start: RectF, delta: PointF) {
        self.move_group(&[(id, start)], delta);
    }

    /// Move vários juntos (cada um a partir da sua posição inicial). O primeiro manda: ele
    /// encaixa na grade e nas guias dos vizinhos, e os outros andam o mesmo tanto.
    pub fn move_group(&mut self, items: &[(ComponentId, RectF)], delta: PointF) {
        let items: Vec<(ComponentId, RectF)> = items.iter().copied().filter(|(id, _)| self.movable(*id)).collect();
        let Some(&(lead, lead_start)) = items.first() else { return };
        let mut x = snap(lead_start.x + delta.x);
        let mut y = snap(lead_start.y + delta.y);
        // Guias: bordas e centros dos irmãos que não estão sendo movidos.
        self.guides.clear();
        if let Some(parent) = self.ui.parent(lead) {
            let moving: Vec<ComponentId> = items.iter().map(|(id, _)| *id).collect();
            let siblings: Vec<RectF> = self
                .ui
                .children(parent)
                .iter()
                .copied()
                .filter(|c| !moving.contains(c) && self.is_visual(*c))
                .filter_map(|c| self.ui.laid_out_bounds(c))
                .collect();
            let parent_size = self.ui.bounds(parent).map(|b| b.size()).unwrap_or_default();
            let origin = self.rect_of(parent).map_or(PointF::ZERO, |r| r.origin());
            let (w, h) = (lead_start.width, lead_start.height);
            let raw = PointF::new(lead_start.x + delta.x, lead_start.y + delta.y);
            let mut xs: Vec<f32> = vec![0.0, parent_size.width / 2.0, parent_size.width];
            let mut ys: Vec<f32> = vec![0.0, parent_size.height / 2.0, parent_size.height];
            for r in &siblings {
                xs.extend([r.x, r.center().x, r.right()]);
                ys.extend([r.y, r.center().y, r.bottom()]);
            }
            // Para cada borda/centro do controle, a guia mais próxima dentro do alcance.
            let best = |targets: &[f32], edges: [f32; 3]| -> Option<(f32, f32)> {
                let mut best: Option<(f32, f32, f32)> = None;
                for (i, &e) in edges.iter().enumerate() {
                    for &t in targets {
                        let d = (t - e).abs();
                        if d <= GUIDE_SNAP && best.is_none_or(|(bd, _, _)| d < bd) {
                            let offset = [0.0, 0.5, 1.0][i];
                            best = Some((d, t, offset));
                        }
                    }
                }
                best.map(|(_, t, offset)| (t, offset))
            };
            if let Some((t, offset)) = best(&xs, [raw.x, raw.x + w / 2.0, raw.x + w]) {
                x = t - w * offset;
                self.guides.push(Guide::Vertical { x: origin.x + t, from: origin.y, to: origin.y + parent_size.height });
            }
            if let Some((t, offset)) = best(&ys, [raw.y, raw.y + h / 2.0, raw.y + h]) {
                y = t - h * offset;
                self.guides.push(Guide::Horizontal { y: origin.y + t, from: origin.x, to: origin.x + parent_size.width });
            }
        }
        let (dx, dy) = (x - lead_start.x, y - lead_start.y);
        for (id, start) in items {
            if let Some(cb) = self.ui.control_mut(id) {
                cb.set_position(start.x + dx, start.y + dy);
            }
        }
        self.ui.layout();
    }

    /// Redimensiona a partir do retângulo `start` (pedido) puxando a alça. Controles
    /// alinhados só mudam a medida que o alinhamento permite (altura de um `top`...).
    pub fn resize_from(&mut self, id: ComponentId, handle: Handle, start: RectF, delta: PointF) {
        self.resize_with(id, handle, start, delta, true);
    }

    /// `grid`: as bordas vão para a grade (num controle girado a grade sem giro não vale).
    fn resize_with(&mut self, id: ComponentId, handle: Handle, start: RectF, delta: PointF, grid: bool) {
        let snap = |v: f32| if grid { snap(v) } else { v };
        if self.locked && id != self.root {
            return;
        }
        let Some(align) = self.ui.control(id).map(|cb| cb.align()) else { return };
        let (mut l, mut t, mut r, mut b) = handle.sides();
        let root = id == self.root;
        match align {
            _ if root => {
                l = false;
                t = false;
            }
            Align::Top | Align::MostTop => (l, t, r) = (false, false, false),
            Align::Bottom | Align::MostBottom => (l, r, b) = (false, false, false),
            Align::Left | Align::MostLeft => (l, t, b) = (false, false, false),
            Align::Right | Align::MostRight => (t, r, b) = (false, false, false),
            Align::Client | Align::Contents | Align::Fit => return,
            Align::Horizontal => (l, r) = (false, false),
            Align::Vertical => (t, b) = (false, false),
            _ => {}
        }
        let (mut x, mut y, mut w, mut h) = (start.x, start.y, start.width, start.height);
        if l {
            x = snap(start.x + delta.x);
            w = start.right() - x;
        }
        if r {
            w = snap(start.right() + delta.x) - start.x;
        }
        if t {
            y = snap(start.y + delta.y);
            h = start.bottom() - y;
        }
        if b {
            h = snap(start.bottom() + delta.y) - start.y;
        }
        if let Some(cb) = self.ui.control_mut(id) {
            cb.set_rect(x, y, w.max(GRID), h.max(GRID));
        }
        self.ui.layout();
    }

    /// Retângulo pedido atual (para começar um arrasto).
    pub fn start_rect(&self, id: ComponentId) -> Option<RectF> {
        self.requested(id)
    }

    /// Ajusta a posição dos selecionados com o teclado (setas), sem a grade.
    pub fn nudge(&mut self, id: ComponentId, dx: f32, dy: f32) {
        let ids: Vec<ComponentId> = if self.is_selected(id) { self.editable_selection() } else { vec![id] };
        let ids: Vec<ComponentId> = ids.into_iter().filter(|&i| self.movable(i)).collect();
        if ids.is_empty() {
            return;
        }
        self.begin_change();
        for id in ids {
            if let Some(cb) = self.ui.control_mut(id) {
                let (x, y) = (cb.left() + dx, cb.top() + dy);
                cb.set_position(x, y);
            }
        }
        self.ui.layout();
    }

    /// Muda o tamanho dos escolhidos (Shift+seta no designer, como no Delphi), um passo de
    /// `dw`×`dh`; o tamanho não fica abaixo de 1. O que o alinhamento estica no outro eixo
    /// muda só no eixo livre (um `align: top` muda só a altura).
    pub fn resize_by(&mut self, id: ComponentId, dw: f32, dh: f32) {
        let ids: Vec<ComponentId> = if self.is_selected(id) { self.editable_selection() } else { vec![id] };
        let ids: Vec<ComponentId> = ids.into_iter().filter(|&i| i != self.root && self.ui.control(i).is_some()).collect();
        if ids.is_empty() {
            return;
        }
        self.begin_change();
        for id in ids {
            if let Some(cb) = self.ui.control_mut(id) {
                let (w, h) = ((cb.width() + dw).max(1.0), (cb.height() + dh).max(1.0));
                cb.set_size(w, h);
            }
        }
        self.ui.layout();
    }

    /// Muda a ordem entre os irmãos (que é também a ordem de alinhamento).
    pub fn move_in_order(&mut self, id: ComponentId, up: bool) -> bool {
        let Some(parent) = self.ui.parent(id) else { return false };
        let siblings = self.ui.children(parent);
        let Some(index) = siblings.iter().position(|&c| c == id) else { return false };
        let target = if up { index.checked_sub(1) } else { (index + 1 < siblings.len()).then_some(index + 1) };
        let Some(target) = target else { return false };
        self.begin_change();
        let _ = self.ui.set_child_index(id, target);
        self.ui.layout();
        true
    }

    /// Sobe (`-1`) ou desce (`+1`) o componente entre os irmãos (a ordem dos campos de um
    /// ClientDataSet, dos itens de um menu...). Devolve se mudou.
    pub fn move_among_siblings(&mut self, id: ComponentId, delta: i32) -> bool {
        let Some(parent) = self.ui.parent(id) else { return false };
        let siblings = self.ui.children(parent);
        let Some(index) = siblings.iter().position(|&c| c == id) else { return false };
        let target = index as i64 + i64::from(delta);
        if target < 0 || target >= siblings.len() as i64 {
            return false;
        }
        self.begin_change();
        let _ = self.ui.set_child_index(id, target as usize);
        self.ui.layout();
        true
    }

    /// Trazer para frente (último entre os irmãos: pintado por cima) ou enviar para trás.
    pub fn bring_to_front(&mut self, front: bool) -> bool {
        let ids = self.editable_selection();
        if ids.is_empty() {
            return false;
        }
        self.begin_change();
        for id in ids {
            let index = if front { usize::MAX } else { 0 };
            let _ = self.ui.set_child_index(id, index);
        }
        self.ui.layout();
        true
    }

    /// Alinhar, distribuir ou igualar o tamanho da seleção (referência: o principal).
    pub fn align_selection(&mut self, op: AlignOp) -> Result<(), String> {
        let ids: Vec<ComponentId> = self.editable_selection().into_iter().filter(|&i| self.movable(i) || matches!(op, AlignOp::SameWidth | AlignOp::SameHeight | AlignOp::SameSize)).collect();
        let needed = if matches!(op, AlignOp::DistributeH | AlignOp::DistributeV) { 3 } else { 2 };
        if ids.len() < needed {
            return Err(format!("selecione pelo menos {needed} controles soltos (align: none)"));
        }
        let reference = self.selection.filter(|s| ids.contains(s)).unwrap_or(ids[0]);
        let r = self.requested(reference).ok_or("seleção inválida")?;
        self.begin_change();
        let mut rects: Vec<(ComponentId, RectF)> = ids.iter().filter_map(|&i| self.requested(i).map(|q| (i, q))).collect();
        match op {
            AlignOp::DistributeH | AlignOp::DistributeV => {
                let horizontal = op == AlignOp::DistributeH;
                rects.sort_by(|a, b| {
                    let (pa, pb) = if horizontal { (a.1.x, b.1.x) } else { (a.1.y, b.1.y) };
                    pa.total_cmp(&pb)
                });
                let first = rects[0].1;
                let last = rects[rects.len() - 1].1;
                let total: f32 = rects.iter().map(|(_, q)| if horizontal { q.width } else { q.height }).sum();
                let span = if horizontal { last.right() - first.x } else { last.bottom() - first.y };
                let gap = (span - total) / (rects.len() - 1) as f32;
                let mut pos = if horizontal { first.x } else { first.y };
                for (id, q) in &rects {
                    if let Some(cb) = self.ui.control_mut(*id) {
                        if horizontal { cb.set_position(pos, q.y) } else { cb.set_position(q.x, pos) };
                    }
                    pos += (if horizontal { q.width } else { q.height }) + gap;
                }
            }
            _ => {
                for (id, q) in rects {
                    let n = match op {
                        AlignOp::Left => RectF::new(r.x, q.y, q.width, q.height),
                        AlignOp::CenterH => RectF::new(r.center().x - q.width / 2.0, q.y, q.width, q.height),
                        AlignOp::Right => RectF::new(r.right() - q.width, q.y, q.width, q.height),
                        AlignOp::Top => RectF::new(q.x, r.y, q.width, q.height),
                        AlignOp::CenterV => RectF::new(q.x, r.center().y - q.height / 2.0, q.width, q.height),
                        AlignOp::Bottom => RectF::new(q.x, r.bottom() - q.height, q.width, q.height),
                        AlignOp::SameWidth => RectF::new(q.x, q.y, r.width, q.height),
                        AlignOp::SameHeight => RectF::new(q.x, q.y, q.width, r.height),
                        AlignOp::SameSize => RectF::new(q.x, q.y, r.width, r.height),
                        AlignOp::DistributeH | AlignOp::DistributeV => q,
                    };
                    if let Some(cb) = self.ui.control_mut(id) {
                        cb.set_rect(n.x, n.y, n.width, n.height);
                    }
                }
            }
        }
        self.ui.layout();
        Ok(())
    }

    // ----- copiar e colar -----

    pub fn copy_selected(&mut self) -> bool {
        let ids = self.editable_selection();
        let copied: Vec<Value> = ids.iter().filter_map(|&id| aylis_json::save_value(&self.ui, id).ok()).map(|d| d["root"].clone()).collect();
        if copied.is_empty() {
            return false;
        }
        self.clipboard = copied;
        true
    }

    /// Recortar: copia e exclui.
    pub fn cut_selected(&mut self) -> bool {
        self.copy_selected() && self.delete_selected()
    }

    pub fn has_clipboard(&self) -> bool {
        !self.clipboard.is_empty()
    }

    /// Cola dentro do selecionado (se for contêiner) ou ao lado dele, com nomes novos e
    /// deslocado para não ficar exatamente por cima.
    pub fn paste(&mut self) -> Result<ComponentId, String> {
        if self.clipboard.is_empty() {
            return Err("nada copiado".into());
        }
        let selected = self.selection.unwrap_or(self.root);
        let parent = if self.ui.class_name(selected).is_some_and(|n| CONTAINERS.contains(&n)) {
            selected
        } else {
            self.ui.parent(selected).unwrap_or(self.root)
        };
        self.begin_change();
        let mut taken = Vec::new();
        let mut pasted = Vec::new();
        for node in self.clipboard.clone() {
            let mut node = node;
            self.rename(&mut node, &mut taken);
            if let Some(obj) = node.as_object_mut() {
                for key in ["left", "top"] {
                    let v = obj.get(key).and_then(Value::as_f64).unwrap_or(0.0);
                    obj.insert(key.into(), json!(v + 2.0 * GRID as f64));
                }
            }
            let id = aylis_json::load_value(&mut self.ui, &json!({ "root": node }), Some(parent)).map_err(|e| e.to_string())?;
            pasted.push(id);
        }
        self.ui.layout();
        let last = *pasted.last().expect("pelo menos um");
        self.select_many(pasted);
        Ok(last)
    }

    fn rename(&self, node: &mut Value, taken: &mut Vec<String>) {
        let Some(obj) = node.as_object_mut() else { return };
        let class = obj.get("class").and_then(Value::as_str).unwrap_or("Componente").to_string();
        if obj.get("name").is_some() {
            let name = (1..)
                .map(|n| format!("{class}{n}"))
                .find(|n| self.ui.find_in(self.root, n).is_none() && !taken.contains(n))
                .expect("sempre há um número livre");
            taken.push(name.clone());
            obj.insert("name".into(), Value::String(name));
        }
        if let Some(Value::Array(children)) = obj.get_mut("children") {
            for child in children {
                self.rename(child, taken);
            }
        }
    }

    /// As propriedades do componente como JSON (para o Object Inspector).
    pub fn props_json(&self, id: ComponentId) -> Map<String, Value> {
        let mut out = Map::new();
        for info in self.ui.prop_infos(id) {
            if let Ok(value) = self.ui.get_prop(id, info.name) {
                let json = match &value {
                    // No inspector, a referência aparece como o usuário escreveria nesse form.
                    PropValue::Component(Some(target)) => self.ui.reference_name(id, *target).map_or(Value::Null, Value::String),
                    _ => aylis_json::prop_to_json(&self.ui, &value),
                };
                out.insert(info.name.to_string(), json);
            }
        }
        out
    }
}
