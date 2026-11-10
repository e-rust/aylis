//! Acessibilidade (AccessKit): leitores de tela (Narrador, NVDA, VoiceOver, Orca) veem os
//! controles do aylis como controles nativos — papel, nome, valor, estado e posição — e
//! podem focar, clicar e mudar valores.
//!
//! A árvore vem dos componentes: o papel pela classe (e pelas classes de base, então um
//! `CellEditor` da grade é um campo de texto), o nome pelas propriedades de texto. Só é montada quando
//! uma tecnologia assistiva pede (sem leitor de tela, nada é gasto).

use accesskit::{Action, ActionData, ActionHandler, ActionRequest, ActivationHandler, DeactivationHandler, Node, NodeId, Rect, Role, Toggled, TreeId, TreeInfo, TreeUpdate};
use aylis_core::{ComponentId, Poster, PropValue, Ui};
#[cfg(not(web))]
use winit::event_loop::ActiveEventLoop;
#[cfg(not(web))]
use winit::window::Window;

/// Pede a primeira árvore: avisa a interface (que manda a árvore inteira em seguida).
#[cfg_attr(web, allow(dead_code))]
struct Activation(Poster);

impl ActivationHandler for Activation {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        self.0.post(|ui| ui.request_repaint());
        None
    }
}

/// Ações do leitor de tela, entregues na thread da interface.
#[cfg_attr(web, allow(dead_code))]
struct Actions(Poster);

impl ActionHandler for Actions {
    fn do_action(&mut self, request: ActionRequest) {
        self.0.post(move |ui| perform(ui, request));
    }
}

#[cfg_attr(web, allow(dead_code))]
struct Deactivation;

impl DeactivationHandler for Deactivation {
    fn deactivate_accessibility(&mut self) {}
}

/// O adaptador de uma janela. No desktop e no iOS é o do `accesskit_winit` (a janela
/// precisa ser criada invisível e mostrada depois). No Android, a árvore vira nós virtuais
/// da vista raiz da janela (`DecorView`) pelo `accesskit_android`: um por processo, porque
/// a janela do winit é recriada a cada volta do segundo plano e a vista raiz é a mesma.
#[cfg(not(web))]
pub(crate) struct Adapter {
    #[cfg(not(target_os = "android"))]
    inner: accesskit_winit::Adapter,
}

#[cfg(not(web))]
impl Adapter {
    pub(crate) fn new(event_loop: &ActiveEventLoop, window: &Window, poster: Poster) -> Self {
        #[cfg(not(target_os = "android"))]
        {
            Self { inner: accesskit_winit::Adapter::with_direct_handlers(event_loop, window, Activation(poster.clone()), Actions(poster), Deactivation) }
        }
        #[cfg(target_os = "android")]
        {
            let _ = (window, &Deactivation);
            android::init(event_loop, poster);
            Self {}
        }
    }

    pub(crate) fn update_if_active(&mut self, update: impl FnOnce() -> TreeUpdate) {
        #[cfg(not(target_os = "android"))]
        self.inner.update_if_active(update);
        #[cfg(target_os = "android")]
        android::update_if_active(update);
    }

    pub(crate) fn process_event(&mut self, window: &Window, event: &winit::event::WindowEvent) {
        #[cfg(not(target_os = "android"))]
        self.inner.process_event(window, event);
        #[cfg(target_os = "android")]
        let _ = (window, event);
    }
}

#[cfg(target_os = "android")]
mod android {
    use std::cell::RefCell;

    use accesskit::TreeUpdate;
    use accesskit_android::InjectingAdapter;
    use accesskit_android::jni::{JavaVM, objects::JObject};
    use aylis_core::Poster;
    use winit::event_loop::ActiveEventLoop;
    use winit::platform::android::ActiveEventLoopExtAndroid;

    thread_local! {
        static ADAPTER: RefCell<Option<InjectingAdapter>> = const { RefCell::new(None) };
    }

    pub(super) fn init(event_loop: &ActiveEventLoop, poster: Poster) {
        if ADAPTER.with(|a| a.borrow().is_some()) {
            return;
        }
        let app = event_loop.android_app();
        // SAFETY: a VM e a Activity do android-activity, válidas enquanto o app existir.
        let Ok(vm) = (unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }) else { return };
        let Ok(mut env) = vm.attach_current_thread() else { return };
        let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
        let decor = env
            .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])
            .and_then(|w| w.l())
            .and_then(|w| env.call_method(&w, "getDecorView", "()Landroid/view/View;", &[]))
            .and_then(|d| d.l());
        let Ok(decor) = decor else {
            let _ = env.exception_clear();
            return;
        };
        let adapter = InjectingAdapter::new(&mut env, &decor, super::Activation(poster.clone()), super::Actions(poster));
        ADAPTER.with(|a| *a.borrow_mut() = Some(adapter));
    }

    pub(super) fn update_if_active(update: impl FnOnce() -> TreeUpdate) {
        ADAPTER.with(|a| {
            if let Some(a) = a.borrow_mut().as_mut() {
                a.update_if_active(update);
            }
        });
    }
}

fn node_id(id: ComponentId) -> NodeId {
    NodeId(id.to_u64())
}

/// Os nós dos itens desenhados pelos controles (abas, linhas, itens de menu, células) têm
/// este bit; o resto é o índice do componente e o do item. O mapa leva de volta ao
/// componente (refeito a cada árvore).
const ITEM_BIT: u64 = 1 << 63;

thread_local! {
    static ITEMS: std::cell::RefCell<std::collections::HashMap<u64, (ComponentId, usize)>> = Default::default();
}

fn item_node_id(owner: ComponentId, index: usize) -> NodeId {
    NodeId(ITEM_BIT | (owner.to_u64() & 0xFFFF_FFFF) << 24 | index as u64 & 0xFF_FFFF)
}

#[cfg_attr(web, allow(dead_code))]
fn perform(ui: &mut Ui, request: ActionRequest) {
    // Um item de um controle: clicar (ou selecionar) pelo próprio controle.
    if request.target_node.0 & ITEM_BIT != 0 {
        // `try_with`: um pedido do leitor de tela pode chegar com o app já fechando.
        let Some((owner, index)) = ITEMS.try_with(|m| m.borrow().get(&request.target_node.0).copied()).ok().flatten() else { return };
        use aylis_core::automation::ItemAction;
        let action = match request.action {
            Action::Click | Action::Focus => Some(ItemAction::Click),
            Action::Expand => Some(ItemAction::Expand),
            Action::Collapse => Some(ItemAction::Collapse),
            _ => None,
        };
        if let Some(action) = action.filter(|_| ui.contains(owner)) {
            ui.access_action(owner, index, action);
            ui.request_repaint();
        }
        return;
    }
    let id = ComponentId::from_u64(request.target_node.0);
    if !ui.contains(id) {
        return;
    }
    match request.action {
        Action::Focus => {
            ui.set_focus(id);
        }
        Action::Click => {
            ui.set_focus(id);
            ui.click(id);
        }
        Action::SetValue => match request.data {
            Some(ActionData::Value(text)) => {
                let _ = ui.set_prop(id, "text", PropValue::String(text.to_string()));
            }
            Some(ActionData::NumericValue(v)) => {
                let _ = ui.set_prop(id, "value", PropValue::Float(v));
            }
            _ => {}
        },
        _ => {}
    }
    ui.request_repaint();
}

/// Papel pela classe (a primeira da cadeia que for conhecida).
fn role(ui: &Ui, id: ComponentId) -> Role {
    let mut chain = Vec::new();
    if let Some(c) = ui.component(id) {
        c.class_chain(&mut chain);
    }
    for class in chain {
        let role = match class {
            "Form" => Role::Window,
            "Button" => Role::Button,
            "Edit" | "NumberBox" | "DateEdit" | "TimeEdit" => Role::TextInput,
            "Memo" => Role::MultilineTextInput,
            "Label" => Role::Label,
            "CheckBox" => Role::CheckBox,
            "Switch" => Role::Switch,
            "RadioButton" => Role::RadioButton,
            "RadioGroup" => Role::RadioGroup,
            "ComboBox" => Role::ComboBox,
            "ListBox" => Role::ListBox,
            "ListView" => Role::List,
            "MenuOverlay" => Role::Menu,
            "TreeView" => Role::Tree,
            "TrackBar" => Role::Slider,
            "ProgressBar" => Role::ProgressIndicator,
            "TabControl" => Role::TabList,
            "TabItem" => Role::TabPanel,
            "Image" => Role::Image,
            "DataGrid" => Role::Grid,
            "ToolBar" => Role::Toolbar,
            "StatusBar" => Role::Status,
            "GroupBox" | "Expander" | "Panel" | "Calendar" | "Chart" => Role::Group,
            "ScrollBox" => Role::ScrollView,
            "MainMenu" => Role::MenuBar,
            _ => continue,
        };
        return role;
    }
    Role::GenericContainer
}

fn text_prop(ui: &Ui, id: ComponentId, name: &str) -> Option<String> {
    match ui.get_prop(id, name) {
        Ok(PropValue::String(s)) if !s.is_empty() => Some(s),
        _ => None,
    }
}

/// A árvore inteira da janela do form `form` (`scale`: pixels físicos por unidade).
pub(crate) fn tree(ui: &Ui, form: ComponentId, scale: f64) -> TreeUpdate {
    let mut nodes = Vec::new();
    let _ = ITEMS.try_with(|m| m.borrow_mut().clear());
    build(ui, form, scale, &mut nodes);
    let root = ui.root_of(form);
    // O foco só aponta para um nó da árvore (um controle escondido não está nela): o
    // AccessKit derruba o app com um foco fora da lista.
    let focus = ui.focused(root).filter(|&f| ui.contains(f)).map(node_id).filter(|f| nodes.iter().any(|(n, _)| n == f)).unwrap_or(node_id(form));
    let mut info = TreeInfo::new(node_id(form));
    info.toolkit_name = Some("aylis".into());
    TreeUpdate { nodes, tree: Some(info), tree_id: TreeId::ROOT, focus }
}

fn build(ui: &Ui, id: ComponentId, scale: f64, out: &mut Vec<(NodeId, Node)>) {
    let role = role(ui, id);
    let mut node = Node::new(role);
    if let Some(r) = ui.absolute_rect(id) {
        let (x, y) = (r.x as f64 * scale, r.y as f64 * scale);
        node.set_bounds(Rect { x0: x, y0: y, x1: x + r.width as f64 * scale, y1: y + r.height as f64 * scale });
    }
    // Nome: o texto que a pessoa vê (ou a dica do campo).
    let label = ["caption", "text", "title", "placeholder"].iter().find_map(|p| text_prop(ui, id, p));
    match role {
        Role::TextInput | Role::MultilineTextInput => {
            if let Some(v) = text_prop(ui, id, "text").or_else(|| text_prop(ui, id, "date")).or_else(|| text_prop(ui, id, "time")) {
                node.set_value(v);
            }
            if let Some(p) = text_prop(ui, id, "placeholder") {
                node.set_label(p);
            }
            node.add_action(Action::SetValue);
        }
        Role::Slider | Role::ProgressIndicator => {
            if let Ok(PropValue::Float(v)) = ui.get_prop(id, "value") {
                node.set_numeric_value(v);
            }
            if let (Ok(PropValue::Float(min)), Ok(PropValue::Float(max))) = (ui.get_prop(id, "min"), ui.get_prop(id, "max")) {
                node.set_min_numeric_value(min);
                node.set_max_numeric_value(max);
            }
            if let Some(l) = label {
                node.set_label(l);
            }
        }
        // Texto estático: o AccessKit lê o conteúdo do valor.
        Role::Label => {
            if let Some(l) = label {
                node.set_value(l);
            }
        }
        _ => {
            if let Some(l) = label {
                node.set_label(l);
            }
        }
    }
    // Marcado/desmarcado só onde isso é o controle (um botão comum também tem `checked`,
    // para grupos de botões, mas só vira "alternar" quando está marcado).
    if let Ok(PropValue::Bool(checked)) = ui.get_prop(id, "checked") {
        if role != Role::Button || checked {
            node.set_toggled(if checked { Toggled::True } else { Toggled::False });
        }
    }
    if matches!(role, Role::Button | Role::CheckBox | Role::Switch | Role::RadioButton | Role::ComboBox) {
        node.add_action(Action::Click);
    }
    let control = ui.control(id);
    if control.is_some_and(|c| c.can_focus()) {
        node.add_action(Action::Focus);
    }
    if control.is_some_and(|c| !c.enabled()) {
        node.set_disabled();
    }
    // O nome do componente (`btnSalvar`) é o AutomationId: a automação acha o controle por
    // ele, sem depender do texto (que muda com o idioma).
    if let Some(name) = ui.name(id).filter(|n| !n.is_empty()) {
        node.set_author_id(name.to_string());
    }
    let children: Vec<ComponentId> =
        ui.children(id).iter().copied().filter(|&c| ui.control(c).is_some_and(|cb| cb.visible())).collect();
    let mut child_ids: Vec<NodeId> = children.iter().map(|&c| node_id(c)).collect();
    // Os itens que o controle desenha (abas, linhas, itens de menu, células): filhos dele
    // (as células, filhas da linha).
    let items = ui.access_items(id);
    if !items.is_empty() {
        let origin = ui.absolute_rect(id).map_or((0.0, 0.0), |r| (r.x as f64, r.y as f64));
        let mut item_children: Vec<Vec<NodeId>> = vec![Vec::new(); items.len()];
        for (i, it) in items.iter().enumerate() {
            match it.parent {
                Some(p) if p < items.len() => item_children[p].push(item_node_id(id, i)),
                _ => child_ids.push(item_node_id(id, i)),
            }
        }
        for (i, it) in items.iter().enumerate() {
            let nid = item_node_id(id, i);
            ITEMS.with(|m| m.borrow_mut().insert(nid.0, (id, i)));
            out.push((nid, item_node(it, origin, scale, std::mem::take(&mut item_children[i]))));
        }
    }
    node.set_children(child_ids);
    out.push((node_id(id), node));
    for c in children {
        build(ui, c, scale, out);
    }
}

/// O nó de um item desenhado (`origin`: o canto do controle no form).
fn item_node(it: &aylis_core::automation::AccessItem, origin: (f64, f64), scale: f64, children: Vec<NodeId>) -> Node {
    use aylis_core::automation::ItemRole;
    let role = match it.role {
        ItemRole::Tab => Role::Tab,
        ItemRole::ListItem => Role::ListBoxOption,
        ItemRole::MenuItem => Role::MenuItem,
        ItemRole::MenuItemCheck => Role::MenuItemCheckBox,
        ItemRole::Row => Role::Row,
        ItemRole::Cell => Role::Cell,
        ItemRole::ColumnHeader => Role::ColumnHeader,
        ItemRole::TreeItem => Role::TreeItem,
        ItemRole::Radio => Role::RadioButton,
        ItemRole::Button => Role::Button,
    };
    let mut node = Node::new(role);
    node.set_label(it.name.clone());
    if let Some(r) = it.rect {
        let (x, y) = ((origin.0 + r.x as f64) * scale, (origin.1 + r.y as f64) * scale);
        node.set_bounds(Rect { x0: x, y0: y, x1: x + r.width as f64 * scale, y1: y + r.height as f64 * scale });
    }
    if matches!(it.role, ItemRole::Tab | ItemRole::ListItem | ItemRole::Row | ItemRole::Cell | ItemRole::TreeItem) {
        node.set_selected(it.selected);
    }
    if let Some(level) = it.level {
        node.set_level(level + 1);
    }
    // Nó com filhos: abrir e fechar (o ExpandCollapse do UI Automation).
    if let Some(open) = it.expanded {
        node.set_expanded(open);
        node.add_action(if open { Action::Collapse } else { Action::Expand });
    }
    if let Some(c) = it.checked {
        node.set_toggled(if c { Toggled::True } else { Toggled::False });
    }
    if it.has_popup {
        node.set_has_popup(accesskit::HasPopup::Menu);
    }
    if let (Some(row), Some(col)) = (it.row, it.column) {
        node.set_row_index(row);
        node.set_column_index(col);
    }
    if it.disabled {
        node.set_disabled();
    } else {
        node.add_action(Action::Click);
    }
    node.set_children(children);
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use aylis_controls::{Button, Form, TabControl};

    /// A árvore leva o nome do componente como AutomationId e os itens desenhados como nós;
    /// um clique da automação num item chega ao controle.
    #[test]
    fn automation_ids_and_drawn_items_reach_the_tree() {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        let form = ui.add_root(Form::new("Pedidos", 600.0, 400.0));
        let button = ui.add(Button::new("Salvar"), form);
        ui.set_name(button, "btnSalvar").unwrap();
        let tabs = ui.add(TabControl::default(), form);
        TabControl::add_tab(&mut ui, tabs, "Dados");
        TabControl::add_tab(&mut ui, tabs, "Clientes");
        ui.layout();

        let update = tree(&ui, form, 1.0);
        let node = |id: NodeId| update.nodes.iter().find(|(n, _)| *n == id).map(|(_, node)| node).unwrap();
        assert_eq!(node(node_id(button)).author_id(), Some("btnSalvar"));
        let tab_nodes: Vec<NodeId> = node(node_id(tabs)).children().iter().copied().filter(|c| c.0 & ITEM_BIT != 0).collect();
        assert_eq!(tab_nodes.len(), 2);
        let second = node(tab_nodes[1]);
        assert_eq!((second.role(), second.label()), (Role::Tab, Some("Clientes")));
        assert_eq!(second.is_selected(), Some(false));

        perform(&mut ui, ActionRequest { action: Action::Click, target_tree: TreeId::ROOT, target_node: tab_nodes[1], data: None });
        assert_eq!(ui.get::<TabControl>(tabs).unwrap().tab_index, 1);
    }
}
