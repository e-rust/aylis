//! Acessibilidade no navegador: o canvas não diz nada ao leitor de tela, então a mesma
//! árvore do AccessKit (papel, nome, valor, estado, posição) vira elementos HTML com ARIA,
//! invisíveis e por cima do canvas, sem pegar o mouse. O leitor de tela (NVDA, VoiceOver,
//! TalkBack no Chrome) navega por eles; ativar um botão clica no controle do aylis.
//!
//! O foco do teclado continua no canvas (o aylis recebe as teclas); os elementos só são a
//! "descrição" da tela. A árvore é refeita no máximo 4 vezes por segundo, e só se mudou.

use std::collections::HashMap;

use accesskit::{Node, NodeId, Role, Toggled, TreeUpdate};
use aylis_core::{ComponentId, Poster};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlCanvasElement, HtmlElement};

pub(crate) struct WebAccess {
    root: HtmlElement,
    canvas: HtmlCanvasElement,
    poster: Poster,
    /// A assinatura da última árvore (texto): igual, nada a refazer.
    last: String,
    last_time: f64,
    placed: String,
    _clicks: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

/// O papel ARIA de cada papel do AccessKit (`None`: só texto, ou contêiner sem papel).
fn aria(role: Role) -> Option<&'static str> {
    Some(match role {
        Role::Window => "region",
        Role::Button => "button",
        Role::TextInput | Role::MultilineTextInput => "textbox",
        Role::CheckBox => "checkbox",
        Role::Switch => "switch",
        Role::RadioButton => "radio",
        Role::RadioGroup => "radiogroup",
        Role::ComboBox => "combobox",
        Role::List => "list",
        Role::Tree => "tree",
        Role::Slider => "slider",
        Role::ProgressIndicator => "progressbar",
        Role::TabList => "tablist",
        Role::TabPanel => "tabpanel",
        Role::Image => "img",
        Role::Grid => "grid",
        Role::Toolbar => "toolbar",
        Role::Status => "status",
        Role::Group => "group",
        Role::ScrollView => "group",
        Role::MenuBar => "menubar",
        _ => return None,
    })
}

fn signature(update: &TreeUpdate) -> String {
    let mut s = String::new();
    for (id, n) in &update.nodes {
        let b = n.bounds().map(|b| (b.x0 as i32, b.y0 as i32, b.x1 as i32, b.y1 as i32));
        s.push_str(&format!(
            "{}|{:?}|{:?}|{:?}|{:?}|{:?}|{}|{:?}\n",
            id.0,
            n.role(),
            n.label(),
            n.value(),
            b,
            n.toggled(),
            n.is_disabled(),
            n.numeric_value()
        ));
    }
    s
}

impl WebAccess {
    pub(crate) fn new(canvas: HtmlCanvasElement, poster: Poster) -> Option<Self> {
        let document = web_sys::window()?.document()?;
        let root: HtmlElement = document.create_element("div").ok()?.dyn_into().ok()?;
        let style = root.style();
        for (k, v) in [("position", "absolute"), ("pointer-events", "none"), ("opacity", "0"), ("overflow", "hidden"), ("z-index", "0")] {
            let _ = style.set_property(k, v);
        }
        let _ = root.set_attribute("data-aylis", "acessibilidade");
        document.body()?.append_child(&root).ok()?;
        Some(Self { root, canvas, poster, last: String::new(), last_time: -1.0, placed: String::new(), _clicks: Vec::new() })
    }

    /// Refaz os elementos se a árvore mudou (no máximo a cada 250 ms). `now` em segundos.
    pub(crate) fn update(&mut self, now: f64, tree: impl FnOnce() -> TreeUpdate) {
        if now - self.last_time < 0.25 && self.last_time >= 0.0 {
            return;
        }
        self.last_time = now;
        self.place();
        let update = tree();
        let sig = signature(&update);
        if sig == self.last {
            return;
        }
        self.last = sig;
        self.root.set_inner_html("");
        self._clicks.clear();
        let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
        let nodes: HashMap<NodeId, &Node> = update.nodes.iter().map(|(id, n)| (*id, n)).collect();
        for (id, node) in &update.nodes {
            let Some(element) = self.element(&document, *id, node, &nodes) else { continue };
            let _ = self.root.append_child(&element);
        }
    }

    fn element(&mut self, document: &web_sys::Document, id: NodeId, node: &Node, _all: &HashMap<NodeId, &Node>) -> Option<HtmlElement> {
        let role = node.role();
        let label = node.label().map(str::to_string);
        let value = node.value().map(str::to_string);
        let aria_role = aria(role);
        if aria_role.is_none() && label.is_none() && value.is_none() {
            return None; // contêiner sem nada a dizer
        }
        let e: HtmlElement = document.create_element("div").ok()?.dyn_into().ok()?;
        let style = e.style();
        if let Some(b) = node.bounds() {
            let _ = style.set_property("position", "absolute");
            let _ = style.set_property("left", &format!("{}px", b.x0));
            let _ = style.set_property("top", &format!("{}px", b.y0));
            let _ = style.set_property("width", &format!("{}px", (b.x1 - b.x0).max(1.0)));
            let _ = style.set_property("height", &format!("{}px", (b.y1 - b.y0).max(1.0)));
        }
        if let Some(r) = aria_role {
            let _ = e.set_attribute("role", r);
        }
        let _ = e.set_attribute("data-aylis-id", &id.0.to_string());
        match role {
            // Texto estático: o próprio conteúdo.
            Role::Label => e.set_text_content(value.as_deref().or(label.as_deref())),
            Role::TextInput | Role::MultilineTextInput => {
                if let Some(l) = &label {
                    let _ = e.set_attribute("aria-label", l);
                }
                if role == Role::MultilineTextInput {
                    let _ = e.set_attribute("aria-multiline", "true");
                }
                e.set_text_content(value.as_deref());
            }
            Role::Slider | Role::ProgressIndicator => {
                for (attr, v) in [("aria-valuenow", node.numeric_value()), ("aria-valuemin", node.min_numeric_value()), ("aria-valuemax", node.max_numeric_value())] {
                    if let Some(v) = v {
                        let _ = e.set_attribute(attr, &v.to_string());
                    }
                }
                if let Some(l) = &label {
                    let _ = e.set_attribute("aria-label", l);
                }
            }
            _ => {
                if let Some(l) = label.as_deref().or(value.as_deref()) {
                    let _ = e.set_attribute("aria-label", l);
                }
            }
        }
        match node.toggled() {
            Some(Toggled::True) => {
                let _ = e.set_attribute("aria-checked", "true");
            }
            Some(Toggled::False) => {
                let _ = e.set_attribute("aria-checked", "false");
            }
            _ => {}
        }
        if node.is_disabled() {
            let _ = e.set_attribute("aria-disabled", "true");
        }
        // Ativar pelo leitor de tela (Enter/toque duplo) dispara "click" no elemento.
        if matches!(role, Role::Button | Role::CheckBox | Role::Switch | Role::RadioButton | Role::ComboBox | Role::TextInput | Role::MultilineTextInput) {
            e.set_tab_index(0);
            let poster = self.poster.clone();
            let target = ComponentId::from_u64(id.0);
            let is_text = matches!(role, Role::TextInput | Role::MultilineTextInput);
            let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                poster.post(move |ui| {
                    if !ui.contains(target) {
                        return;
                    }
                    ui.set_focus(target);
                    if !is_text {
                        ui.click(target);
                    }
                    ui.request_repaint();
                });
            });
            let _ = e.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());
            self._clicks.push(click);
        }
        Some(e)
    }

    /// A camada acompanha o canvas na página.
    fn place(&mut self) {
        let (Some(window), b) = (web_sys::window(), self.canvas.get_bounding_client_rect()) else { return };
        let x = b.left() + window.scroll_x().unwrap_or(0.0);
        let y = b.top() + window.scroll_y().unwrap_or(0.0);
        let key = format!("{x};{y};{};{}", b.width(), b.height());
        if self.placed == key {
            return;
        }
        let style = self.root.style();
        let _ = style.set_property("left", &format!("{x}px"));
        let _ = style.set_property("top", &format!("{y}px"));
        let _ = style.set_property("width", &format!("{}px", b.width()));
        let _ = style.set_property("height", &format!("{}px", b.height()));
        self.placed = key;
    }
}
