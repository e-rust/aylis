//! O editor de uma lista de registros (`PropKind::RecordList`: os itens do ListView, os nós do
//! TreeView, as colunas do DataGrid, as séries do Chart...), como o editor de coleção do
//! Delphi: os itens à esquerda e, à direita, os campos do item escolhido, cada um com o editor
//! do tipo dele. Uma lista dentro do item (os filhos de um nó) abre o mesmo editor por cima.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, ComboBox, Edit, Layout, ListBox, ScrollBox};
use aylis_core::{Align, ComponentId, PropKind, RecordInfo, Ui};
use serde_json::{Map, Value};

use crate::dialogs::{new_dialog, run};
use crate::widgets::label;

/// Um campo do item na tela: o editor e o que ele guarda (a lista de dentro fica no item).
enum FieldEditor {
    Text(ComponentId),
    Check(ComponentId),
    Choice(ComponentId),
    /// A lista (ou o registro) de dentro: editada no editor de cima, já guardada no item.
    Nested,
}

struct State {
    items: Vec<Map<String, Value>>,
    selected: Option<usize>,
    editors: Vec<(&'static str, PropKind, FieldEditor)>,
}

type Shared = Rc<RefCell<State>>;
type Rebuild = Rc<RefCell<Option<Box<dyn Fn(&mut Ui)>>>>;

/// Os itens do JSON (lista de objetos; um objeto só vira um item).
fn parse(json: &str) -> Vec<Map<String, Value>> {
    match serde_json::from_str::<Value>(json.trim()) {
        Ok(Value::Array(items)) => items.into_iter().filter_map(|v| if let Value::Object(m) = v { Some(m) } else { None }).collect(),
        Ok(Value::Object(m)) => vec![m],
        _ => Vec::new(),
    }
}

/// O nome do item na lista: o primeiro campo de texto preenchido.
fn caption(record: &RecordInfo, item: &Map<String, Value>, index: usize, unnamed: &str) -> String {
    let text = record.fields.iter().filter(|(_, k)| *k == PropKind::String).find_map(|(f, _)| item.get(*f).and_then(Value::as_str).filter(|s| !s.trim().is_empty()));
    // O `;` separa os itens do ListBox.
    text.map_or_else(|| unnamed.replace("{}", &(index + 1).to_string()), |t| t.replace(';', ","))
}

/// O valor do campo como o editor de texto mostra (a lista: os itens separados por `; `).
fn field_text(kind: PropKind, value: Option<&Value>) -> String {
    match (kind, value) {
        (_, None | Some(Value::Null)) => String::new(),
        (PropKind::List | PropKind::NumberList, Some(Value::Array(a))) => a.iter().map(|x| x.as_str().map_or_else(|| x.to_string(), str::to_string)).collect::<Vec<_>>().join("; "),
        (_, Some(Value::String(s))) => s.clone(),
        (_, Some(v)) => v.to_string(),
    }
}

/// O texto do editor como o valor do campo (no tipo dele; vazio: sem o campo).
fn text_value(kind: PropKind, text: &str) -> Option<Value> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    Some(match kind {
        PropKind::Int => t.parse::<i64>().map(Value::from).unwrap_or_else(|_| Value::from(t)),
        PropKind::Float => t.replace(',', ".").parse::<f64>().map(Value::from).unwrap_or_else(|_| Value::from(t)),
        PropKind::List => Value::Array(t.split(';').map(str::trim).filter(|s| !s.is_empty()).map(Value::from).collect()),
        PropKind::NumberList => Value::Array(t.split(';').filter_map(|s| s.trim().replace(',', ".").parse::<f64>().ok()).map(Value::from).collect()),
        _ => Value::from(t),
    })
}

/// Guarda no item escolhido o que está nos editores.
fn read(ui: &Ui, state: &Shared) {
    let mut st = state.borrow_mut();
    let Some(i) = st.selected else { return };
    let values: Vec<(&'static str, Option<Value>)> = st
        .editors
        .iter()
        .filter_map(|(name, kind, editor)| {
            let v = match editor {
                FieldEditor::Text(e) => text_value(*kind, &ui.get::<Edit>(*e).map(|e| e.text.clone()).unwrap_or_default()),
                // Desmarcado é o padrão: sai do JSON, como o texto vazio.
                FieldEditor::Check(c) => ui.get::<CheckBox>(*c).is_some_and(|c| c.checked).then_some(Value::Bool(true)),
                FieldEditor::Choice(c) => Some(Value::from(ui.get::<ComboBox>(*c).map(|c| c.text().to_string()).unwrap_or_default())).filter(|v| v.as_str() != Some("")),
                FieldEditor::Nested => return None,
            };
            Some((*name, v))
        })
        .collect();
    let Some(item) = st.items.get_mut(i) else { return };
    for (name, v) in values {
        match v {
            Some(v) => {
                item.insert(name.to_string(), v);
            }
            None => {
                item.remove(name);
            }
        }
    }
}

/// Abre o editor da lista `json` de registros `record`; `single`: um registro só (sem
/// acrescentar nem tirar). `on_ok` recebe o JSON novo.
pub fn records_editor(ui: &mut Ui, title: &str, record: &'static RecordInfo, json: &str, single: bool, on_ok: impl FnOnce(&mut Ui, Value) + 'static) {
    open(ui, title, record, json, single, Box::new(on_ok));
}

/// O editor em si (o retorno numa caixa: o editor dos filhos abre o mesmo, sem um tipo novo a
/// cada nível).
fn open(ui: &mut Ui, title: &str, record: &'static RecordInfo, json: &str, single: bool, on_ok: Box<dyn FnOnce(&mut Ui, Value)>) {
    let d = new_dialog(ui, title, 760.0, 540.0);
    let mut items = parse(json);
    if single && items.is_empty() {
        items.push(Map::new());
    }
    let selected = (!items.is_empty()).then_some(0);
    let state: Shared = Rc::new(RefCell::new(State { items, selected, editors: Vec::new() }));

    let tools = ui.add(Layout::default(), d.body);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let mut buttons = Vec::new();
    if !single {
        for (text, action) in [("+ Item", 0), ("✕", 1), ("↑", 2), ("↓", 3)] {
            let b = crate::widgets::tool(ui, tools, text, if action == 0 { 90.0 } else { 40.0 }, if action == 0 { "" } else { "flat" });
            buttons.push((b, action));
        }
    }
    let list = ui.add(ListBox::new(&[]), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Left).set_width(220.0).set_margins([0.0, 0.0, 8.0, 0.0]).set_visible(!single);
    let fields = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(fields).unwrap().set_align(Align::Client);

    let rebuild: Rebuild = Rc::default();
    {
        let (state, rebuild2) = (state.clone(), rebuild.clone());
        *rebuild.borrow_mut() = Some(Box::new(move |ui: &mut Ui| {
            // A lista da esquerda.
            let unnamed = ui.tr("(item {})");
            let (captions, selected) = {
                let st = state.borrow();
                (st.items.iter().enumerate().map(|(i, it)| caption(record, it, i, &unnamed)).collect::<Vec<_>>(), st.selected)
            };
            if let Some(l) = ui.get_mut::<ListBox>(list) {
                l.set_items(&captions);
                l.item_index = selected.map_or(-1, |i| i as i64);
            }
            ui.invalidate();
            // Os campos do item escolhido.
            crate::widgets::clear(ui, fields);
            state.borrow_mut().editors.clear();
            let Some(i) = selected else { return };
            let item = state.borrow().items.get(i).cloned().unwrap_or_default();
            for (name, kind) in record.fields {
                let row = ui.add(Layout::default(), fields);
                ui.control_mut(row).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 6.0]);
                let caption = if crate::lang::has_property_label(name) { crate::lang::property_label(name) } else { name.to_string() };
                label(ui, row, &caption, Align::Left, 150.0);
                let value = item.get(*name);
                let editor = match kind {
                    PropKind::Bool => {
                        let c = ui.add(CheckBox::new(""), row);
                        ui.control_mut(c).unwrap().set_align(Align::Client);
                        if let Some(cb) = ui.get_mut::<CheckBox>(c) {
                            cb.checked = value.and_then(Value::as_bool).unwrap_or(false);
                        }
                        FieldEditor::Check(c)
                    }
                    PropKind::Enum(names) => {
                        let mut options: Vec<&str> = vec![""];
                        options.extend(names.iter().copied());
                        let c = ui.add(ComboBox::new(&options), row);
                        ui.control_mut(c).unwrap().set_align(Align::Client);
                        let current = value.and_then(Value::as_str).unwrap_or_default();
                        let index = options.iter().position(|o| *o == current).unwrap_or(0);
                        let _ = ui.set_prop(c, "item_index", index as i64);
                        FieldEditor::Choice(c)
                    }
                    PropKind::Record(inner) | PropKind::RecordList(inner) => {
                        let list_kind = matches!(kind, PropKind::RecordList(_));
                        let count = match value {
                            Some(Value::Array(a)) => a.len(),
                            Some(Value::Object(_)) => 1,
                            _ => 0,
                        };
                        let text = if list_kind { ui.tr("Editar… ({})").replace("{}", &count.to_string()) } else { ui.tr("Editar…") };
                        let b = ui.add(Button::new(text), row);
                        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(140.0);
                        let (state, rebuild, field) = (state.clone(), rebuild2.clone(), *name);
                        ui.on_click(b, move |ui, _| {
                            read(ui, &state);
                            let current = state.borrow().items.get(i).and_then(|it| it.get(field)).cloned().unwrap_or(Value::Null);
                            let (state, rebuild) = (state.clone(), rebuild.clone());
                            open(ui, field, inner, &current.to_string(), !list_kind, Box::new(move |ui: &mut Ui, value: Value| {
                                let value = if list_kind { value } else { value.as_array().and_then(|a| a.first()).cloned().unwrap_or(Value::Null) };
                                if let Some(item) = state.borrow_mut().items.get_mut(i) {
                                    item.insert(field.to_string(), value);
                                }
                                if let Some(f) = rebuild.borrow().as_ref() {
                                    f(ui);
                                }
                            }));
                        });
                        FieldEditor::Nested
                    }
                    k => {
                        let e = ui.add(Edit::new(field_text(*k, value)), row);
                        ui.control_mut(e).unwrap().set_align(Align::Client);
                        FieldEditor::Text(e)
                    }
                };
                state.borrow_mut().editors.push((*name, *kind, editor));
            }
        }));
    }
    if let Some(f) = rebuild.borrow().as_ref() {
        f(ui);
    }
    // Trocar de item: guarda o que foi digitado no de antes.
    {
        let (state, rebuild) = (state.clone(), rebuild.clone());
        ui.on(list, "on_change", move |ui, _, _| {
            let index = ui.get::<ListBox>(list).map_or(-1, |l| l.item_index);
            let index = usize::try_from(index).ok();
            if index == state.borrow().selected {
                return;
            }
            read(ui, &state);
            state.borrow_mut().selected = index;
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    for (b, action) in buttons {
        let (state, rebuild) = (state.clone(), rebuild.clone());
        ui.on_click(b, move |ui, _| {
            read(ui, &state);
            {
                let mut st = state.borrow_mut();
                let len = st.items.len();
                match (action, st.selected) {
                    (0, _) => {
                        st.items.push(Map::new());
                        st.selected = Some(st.items.len() - 1);
                    }
                    (1, Some(i)) => {
                        st.items.remove(i);
                        st.selected = if st.items.is_empty() { None } else { Some(i.min(st.items.len() - 1)) };
                    }
                    (2, Some(i)) if i > 0 => {
                        st.items.swap(i, i - 1);
                        st.selected = Some(i - 1);
                    }
                    (3, Some(i)) if i + 1 < len => {
                        st.items.swap(i, i + 1);
                        st.selected = Some(i + 1);
                    }
                    _ => {}
                }
            }
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    run(ui, d, |_| Ok(()), move |ui| {
        read(ui, &state);
        let items: Vec<Value> = state.borrow().items.iter().cloned().map(Value::Object).collect();
        // O editor guarda a função que se remonta: quebra o ciclo.
        rebuild.borrow_mut().take();
        on_ok(ui, Value::Array(items));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_of_a_field_goes_back_in_its_type() {
        assert_eq!(text_value(PropKind::Float, "2,5"), Some(Value::from(2.5)));
        assert_eq!(text_value(PropKind::Int, "44"), Some(Value::from(44)));
        assert_eq!(text_value(PropKind::NumberList, "1; 2,5"), Some(serde_json::json!([1.0, 2.5])));
        assert_eq!(text_value(PropKind::List, "a; b"), Some(serde_json::json!(["a", "b"])));
        assert_eq!(text_value(PropKind::String, "  "), None);
        assert_eq!(field_text(PropKind::NumberList, Some(&serde_json::json!([1, 2.5]))), "1; 2.5");
    }

    #[test]
    fn the_items_come_from_a_list_or_one_object() {
        assert_eq!(parse(r#"[{"title": "Ana"}, {"title": "Bia"}]"#).len(), 2);
        assert_eq!(parse(r#"{"width": 1}"#).len(), 1);
        assert!(parse("").is_empty());
    }
}
