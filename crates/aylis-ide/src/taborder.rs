//! Editar → Ordem de tabulação… (como o "Tab Order" do Delphi, mas para o form inteiro):
//! a lista dos controles que recebem o foco, na ordem do Tab, de qualquer nível (um campo
//! num painel, outro num grupo). ↑/↓ mudam a ordem; "Começar por este" marca o primeiro
//! foco ao abrir o form (`active_control`, o ★). OK grava `tab_order` e `active_control`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, PropValue, Ui};
use serde_json::Value;

use crate::app::{Ids, refresh};
use crate::dialogs::{new_dialog, run};
use crate::state::Shared;
use crate::widgets::{clear, muted, row};

/// O que o diálogo está editando.
struct Model {
    /// (nome, classe) na ordem do Tab.
    items: Vec<(String, String)>,
    selected: Option<usize>,
    /// O nome do primeiro foco.
    first: Option<String>,
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (items, first) = {
        let st = state.borrow();
        let doc = &st.doc;
        let items: Vec<(String, String)> = doc
            .ui
            .tab_order(doc.root)
            .into_iter()
            .filter_map(|id| Some((doc.ui.name(id)?.to_string(), doc.ui.class_name(id).unwrap_or_default().to_string())))
            .filter(|(n, _)| !n.is_empty())
            .collect();
        let first = match doc.ui.get_prop(doc.root, "active_control") {
            Ok(PropValue::Component(Some(c))) => doc.ui.name(c).map(str::to_string),
            _ => None,
        };
        (items, first)
    };
    let d = new_dialog(ui, "Ordem de tabulação", 460.0, 560.0);
    let about = ui.add(Label::new("A ordem em que o Tab passa pelos controles do form, de qualquer nível. ★ = o foco ao abrir o form."), d.body);
    ui.control_mut(about).unwrap().set_align(Align::Top).set_height(40.0);
    let _ = ui.set_prop(about, "wrap", true);
    muted(ui, about);
    let tools = ui.add(Layout::default(), d.body);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 6.0, 0.0, 8.0]);
    let mut button = |text: &str, width: f32| {
        let b = ui.add(Button::new(text), tools);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 6.0, 0.0]).set_style("flat");
        b
    };
    let up = button("↑", 36.0);
    let down = button("↓", 36.0);
    let first_button = button("★ Começar por este", 150.0);
    let natural = button("Ordem natural", 120.0);
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    let model = Rc::new(RefCell::new(Model { items, selected: None, first }));
    let fill: Rc<dyn Fn(&mut Ui)> = {
        let model = model.clone();
        Rc::new(move |ui: &mut Ui| draw(ui, list, &model))
    };
    // As linhas chamam `fill` de novo ao serem clicadas: guardado onde elas acham.
    FILL.with(|f| *f.borrow_mut() = Some(fill.clone()));
    fill(ui);
    let moves: [(ComponentId, i32); 2] = [(up, -1), (down, 1)];
    for (b, delta) in moves {
        let (model, fill) = (model.clone(), fill.clone());
        ui.on_click(b, move |ui, _| {
            {
                let mut m = model.borrow_mut();
                let Some(i) = m.selected else { return };
                let j = i as i64 + i64::from(delta);
                if j < 0 || j >= m.items.len() as i64 {
                    return;
                }
                m.items.swap(i, j as usize);
                m.selected = Some(j as usize);
            }
            fill(ui);
        });
    }
    {
        let (model, fill) = (model.clone(), fill.clone());
        ui.on_click(first_button, move |ui, _| {
            {
                let mut m = model.borrow_mut();
                let Some(i) = m.selected else { return };
                m.first = Some(m.items[i].0.clone());
            }
            fill(ui);
        });
    }
    {
        // A ordem da árvore (sem lista): refaz a partir do form, sem a lista gravada.
        let (model, fill, state) = (model.clone(), fill.clone(), state.clone());
        ui.on_click(natural, move |ui, _| {
            {
                let st = state.borrow();
                let doc = &st.doc;
                let current = doc.ui.get_prop(doc.root, "tab_order").ok();
                drop(st);
                let mut st = state.borrow_mut();
                let root = st.doc.root;
                let _ = st.doc.ui.set_prop(root, "tab_order", "");
                let order: Vec<(String, String)> = st
                    .doc
                    .ui
                    .tab_order(st.doc.root)
                    .into_iter()
                    .filter_map(|id| Some((st.doc.ui.name(id)?.to_string(), st.doc.ui.class_name(id).unwrap_or_default().to_string())))
                    .filter(|(n, _)| !n.is_empty())
                    .collect();
                if let Some(v) = current {
                    let root = st.doc.root;
                    let _ = st.doc.ui.set_prop(root, "tab_order", v);
                }
                let mut m = model.borrow_mut();
                m.items = order;
                m.selected = None;
            }
            fill(ui);
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    run(ui, d, |_| Ok(()), move |ui| {
        let m = model.borrow();
        let order = m.items.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join("; ");
        {
            let mut st = state2.borrow_mut();
            let root = st.doc.root;
            let _ = st.doc.set_prop(root, "tab_order", &Value::String(order));
            let first = m.first.clone().map_or(Value::Null, Value::String);
            let _ = st.doc.set_prop(root, "active_control", &first);
            st.status = "Ordem de tabulação gravada".into();
        }
        FILL.with(|f| *f.borrow_mut() = None);
        refresh(ui, &state2, &ids2);
    });
}

thread_local! {
    static FILL: RefCell<Option<Rc<dyn Fn(&mut Ui)>>> = const { RefCell::new(None) };
}

fn draw(ui: &mut Ui, list: ComponentId, model: &Rc<RefCell<Model>>) {
    clear(ui, list);
    let m = model.borrow();
    for (i, (name, class)) in m.items.iter().enumerate() {
        let star = if m.first.as_deref() == Some(name.as_str()) { "★ " } else { "    " };
        let r = row(ui, list, &format!("{:>2}.  {star}{name}   ·   {class}", i + 1), 0.0, m.selected == Some(i), 28.0);
        let model = model.clone();
        ui.on_click(r, move |ui, _| {
            model.borrow_mut().selected = Some(i);
            if let Some(fill) = FILL.with(|f| f.borrow().clone()) {
                fill(ui);
            }
        });
    }
}
