//! Inspetor do app em execução: fala com o aplicativo que o F9 abriu pelo canal de
//! desenvolvimento (veja `devlink.rs`), mostra a árvore de componentes e as propriedades do
//! escolhido; mudar um valor muda no app na hora. "Destacar" pisca uma moldura em volta do controle no app.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use aylis_controls::{Button, Edit, Form, Label, Layout, ScrollBox, Splitter, TreeNode, TreeView};
use aylis_core::{Align, ComponentId, EventArgs, Ui};
use serde_json::{Value, json};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{caption, label, muted};

struct View {
    link: Arc<crate::devlink::DevServer>,
    tree: ComponentId,
    props: ComponentId,
    title: ComponentId,
    status: ComponentId,
    selected: Option<String>,
}

type Live = Rc<RefCell<View>>;

impl View {
    fn call(&mut self, command: Value) -> Result<Value, String> {
        self.link.call(command)
    }
}

/// Abre o inspetor (o app precisa estar em execução pelo F9 e conectado no canal).
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let dev = state.borrow().dev.clone().filter(|d| d.connected());
    let Some(link) = dev else {
        state.borrow_mut().status = "Execute o projeto (F9) e espere o app abrir para inspecioná-lo".into();
        return;
    };
    let form = ui.add_root(Form::new("Inspetor do app em execução", 900.0, 640.0));
    let _ = ui.set_name(form, "frmInspetorApp");
    ui.control_mut(form).unwrap().set_padding(12.0);
    let top = ui.add(Layout::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let refresh_b = ui.add(Button::new("Atualizar"), top);
    ui.control_mut(refresh_b).unwrap().set_align(Align::Left).set_width(100.0);
    let highlight_b = ui.add(Button::new("Destacar no app"), top);
    ui.control_mut(highlight_b).unwrap().set_align(Align::Left).set_width(140.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let close_b = ui.add(Button::new("Fechar"), top);
    ui.control_mut(close_b).unwrap().set_align(Align::Right).set_width(90.0).set_style("flat");
    let _ = ui.set_prop(close_b, "modal_result", "ok");
    let status = label(ui, top, "", Align::Client, 0.0);
    ui.control_mut(status).unwrap().set_margins([12.0, 0.0, 0.0, 0.0]);
    muted(ui, status);

    let left = ui.add(Layout::default(), form);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_width(330.0);
    caption(ui, left, "COMPONENTES");
    let tree = ui.add(TreeView::default(), left);
    ui.control_mut(tree).unwrap().set_align(Align::Client);
    ui.add(Splitter::new(Align::Left), form);
    let right = ui.add(Layout::default(), form);
    ui.control_mut(right).unwrap().set_align(Align::Client).set_margins([8.0, 0.0, 0.0, 0.0]);
    let title = caption(ui, right, "PROPRIEDADES (mudar aqui muda no app)");
    let props = ui.add(ScrollBox::default(), right);
    ui.control_mut(props).unwrap().set_align(Align::Client);

    let live: Live = Rc::new(RefCell::new(View { link, tree, props, title, status, selected: None }));
    {
        let live = live.clone();
        ui.on_click(refresh_b, move |ui, _| load_tree(ui, &live));
    }
    {
        let live = live.clone();
        ui.on_click(highlight_b, move |ui, _| {
            let mut v = live.borrow_mut();
            let Some(path) = v.selected.clone() else { return };
            let result = v.call(json!({ "cmd": "highlight", "target": path }));
            let status = v.status;
            drop(v);
            let _ = ui.set_prop(status, "text", result.err().map(|e| format!("⚠ {e}")).unwrap_or_default());
        });
    }
    {
        let live = live.clone();
        ui.on(tree, "on_change", move |ui, sender, _| {
            let path = ui.get::<TreeView>(sender).and_then(|t| t.selected()).map(|n| n.data.clone());
            live.borrow_mut().selected = path;
            load_props(ui, &live);
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, _| {
        ui.free(form);
        refresh(ui, &state2, &ids2);
    });
    load_tree(ui, &live);
}

fn load_tree(ui: &mut Ui, live: &Live) {
    let (result, tree, status) = {
        let mut v = live.borrow_mut();
        (v.call(json!({ "cmd": "tree" })), v.tree, v.status)
    };
    match result {
        Ok(Value::Array(forms)) => {
            fn node(v: &Value) -> TreeNode {
                let name = v["name"].as_str().unwrap_or_default();
                let class = v["class"].as_str().unwrap_or_default();
                let text = if name.is_empty() { class.to_string() } else { format!("{name}  ·  {class}") };
                let mut n = TreeNode::new(text);
                n.data = v["path"].as_str().unwrap_or_default().to_string();
                n.children = v["children"].as_array().map(|c| c.iter().filter(|x| x["class"] != "InspectorHighlight").map(node).collect()).unwrap_or_default();
                n
            }
            let mut nodes: Vec<TreeNode> = forms.iter().map(node).collect();
            for n in &mut nodes {
                n.expanded = true;
            }
            let count = forms.len();
            if let Some(t) = ui.get_mut::<TreeView>(tree) {
                t.set_nodes(nodes);
                t.expand_all(true);
            }
            let _ = ui.set_prop(status, "text", format!("Conectado ({}) · {count} form(s)", live.borrow().link.url()));
        }
        Ok(_) => {}
        Err(e) => {
            let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
        }
    }
    ui.invalidate();
}

/// Valor digitado → JSON do tipo da propriedade.
fn parse(kind: &str, text: &str) -> Value {
    let t = text.trim();
    match kind {
        "bool" => json!(matches!(t.to_lowercase().as_str(), "true" | "1" | "sim" | "s")),
        "int" => t.parse::<i64>().map(Value::from).unwrap_or(Value::Null),
        "float" => t.replace(',', ".").parse::<f64>().map(Value::from).unwrap_or(Value::Null),
        "set" => Value::Array(t.split(',').map(|s| s.trim().trim_matches('"')).filter(|s| !s.is_empty()).map(|s| Value::String(s.to_string())).collect()),
        "bounds" => Value::Array(t.split([',', ' ']).filter(|s| !s.is_empty()).filter_map(|n| n.replace(',', ".").parse::<f64>().ok()).map(Value::from).collect()),
        _ => Value::String(text.to_string()),
    }
}

fn shown(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(|v| v.as_str().map_or_else(|| v.to_string(), str::to_string)).collect::<Vec<_>>().join(", "),
        other => other.to_string(),
    }
}

fn load_props(ui: &mut Ui, live: &Live) {
    let (path, props, title, status) = {
        let v = live.borrow();
        (v.selected.clone(), v.props, v.title, v.status)
    };
    for child in ui.children(props).to_vec() {
        ui.free(child);
    }
    let Some(path) = path else { return };
    let _ = ui.set_prop(title, "text", format!("PROPRIEDADES DE {}", path.to_uppercase()));
    let result = live.borrow_mut().call(json!({ "cmd": "props", "target": path }));
    let mut list = match result {
        Ok(Value::Array(list)) => list,
        Ok(_) => Vec::new(),
        Err(e) => {
            let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
            return;
        }
    };
    // Agrupa por categoria (na ordem em que cada uma aparece primeiro).
    let mut order: Vec<String> = Vec::new();
    for p in &list {
        let c = p["category"].as_str().unwrap_or_default().to_string();
        if !order.contains(&c) {
            order.push(c);
        }
    }
    list.sort_by_key(|p| order.iter().position(|c| p["category"].as_str() == Some(c.as_str())));
    let mut category = String::new();
    for p in list {
        let name = p["name"].as_str().unwrap_or_default().to_string();
        let kind = p["kind"].as_str().unwrap_or_default().to_string();
        let cat = p["category"].as_str().unwrap_or_default().to_string();
        if cat != category {
            caption(ui, props, &cat.to_uppercase());
            category = cat;
        }
        let row = ui.add(Layout::default(), props);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 2.0]);
        let l = ui.add(Label::new(name.as_str()), row);
        ui.control_mut(l).unwrap().set_align(Align::Left).set_width(170.0);
        let e = ui.add(Edit::new(shown(&p["value"])), row);
        ui.control_mut(e).unwrap().set_align(Align::Client);
        let read_only = p["read_only"].as_bool().unwrap_or(false) || kind == "event";
        let _ = ui.set_prop(e, "read_only", read_only);
        if let Some(options) = p["options"].as_array() {
            let hint = options.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" | ");
            let _ = ui.set_prop(e, "placeholder", hint);
        }
        if read_only {
            continue;
        }
        let (live, path) = (live.clone(), path.clone());
        ui.on(e, "on_change", move |ui, _, args| {
            let EventArgs::Value(aylis_core::PropValue::String(text)) = args else { return };
            let value = parse(&kind, text);
            if value.is_null() {
                return; // número pela metade
            }
            let mut v = live.borrow_mut();
            let result = v.call(json!({ "cmd": "set", "target": path, "props": { name.clone(): value } }));
            let status = v.status;
            drop(v);
            let text = match result {
                Ok(_) => format!("{path}.{name} alterado no app"),
                Err(e) => format!("⚠ {e}"),
            };
            let _ = ui.set_prop(status, "text", text);
        });
    }
    ui.invalidate();
}
