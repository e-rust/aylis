//! A janela de campos do ClientDataSet: fica aberta ao
//! lado da IDE, lista os campos na ordem e, ao clicar num, seleciona-o — o Object
//! Inspector passa a mostrar todas as propriedades dele. Adicionar, excluir, subir e
//! descer; "Editar todos…" abre a tabela com nome, tipo, tamanho e rótulo de uma vez.

use std::cell::RefCell;

use aylis_controls::{Button, Form, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, PropValue, Ui};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{clear, muted, row};

/// A janela aberta: o form dela, a lista e o ClientDataSet que ela mostra.
#[derive(Clone, Copy)]
struct Open {
    form: ComponentId,
    list: ComponentId,
    cds: ComponentId,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

/// Abre (ou traz para a frente, já mostrando este ClientDataSet) a janela de campos.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids, cds: ComponentId) {
    if let Some(open) = OPEN.with(|o| *o.borrow()).filter(|o| ui.contains(o.form) && ui.is_open(o.form)) {
        OPEN.with(|o| *o.borrow_mut() = Some(Open { cds, ..open }));
        let _ = ui.show(open.form);
        sync(ui, state, ids);
        return;
    }
    let form = ui.add_root(Form::new("Campos", 340.0, 460.0));
    if let Some(cb) = ui.control_mut(form) {
        cb.set_padding(12.0);
    }
    let _ = ui.set_prop(form, "topmost", true);

    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(34.0).set_margins([0.0, 10.0, 0.0, 0.0]);
    let all = ui.add(Button::new("Editar todos…"), bar);
    ui.control_mut(all).unwrap().set_align(Align::Right).set_width(118.0).set_style("flat");
    let tools = ui.add(Layout::default(), form);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let mut button = |text: &str, width: f32, hint: &str| {
        let b = ui.add(Button::new(text), tools);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 6.0, 0.0]).set_style("flat");
        let _ = ui.set_prop(b, "hint", hint);
        b
    };
    let add = button("+ Adicionar", 104.0, "Novo campo no ClientDataSet");
    let remove = button("Excluir", 74.0, "Exclui o campo selecionado");
    let up = button("↑", 36.0, "Sobe o campo");
    let down = button("↓", 36.0, "Desce o campo");
    // Onde ficam as propriedades: como no Fields Editor, o campo escolhido vai para o
    // Object Inspector (os eventos `on_get_text` e `on_set_text` na aba Eventos).
    let tip = ui.add(Label::new("Clique num campo: o Object Inspector mostra as propriedades dele; os eventos (on_get_text, on_set_text, on_validate) ficam na aba Eventos."), form);
    ui.control_mut(tip).unwrap().set_align(Align::Bottom).set_height(48.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let _ = ui.set_prop(tip, "wrap", true);
    let _ = ui.set_prop(tip, "font_size", 11.0);
    muted(ui, tip);
    let list = ui.add(ScrollBox::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    OPEN.with(|o| *o.borrow_mut() = Some(Open { form, list, cds }));
    let current = || OPEN.with(|o| o.borrow().map(|o| o.cds));
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(add, move |ui, _| {
            let Some(cds) = current() else { return };
            {
                let mut st = state.borrow_mut();
                st.doc.select(Some(cds));
                st.status = match st.doc.add("Field", None) {
                    Ok(id) => format!("{} adicionado", st.doc.ui.name(id).unwrap_or_default()),
                    Err(e) => format!("⚠ {e}"),
                };
            }
            refresh(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(remove, move |ui, _| {
            if selected_field(&state).is_some() {
                state.borrow_mut().doc.delete_selected();
                refresh(ui, &state, &ids);
            }
        });
    }
    for (b, delta) in [(up, -1), (down, 1)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(b, move |ui, _| {
            if let Some(field) = selected_field(&state) {
                state.borrow_mut().doc.move_among_siblings(field, delta);
                refresh(ui, &state, &ids);
            }
        });
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(all, move |ui, _| {
            if let Some(cds) = current() {
                crate::actions::fields_editor(ui, &state, &ids, cds);
            }
        });
    }
    ui.on(form, "on_close", move |ui, form, _| {
        OPEN.with(|o| *o.borrow_mut() = None);
        // Liberado depois do evento (o form ainda está em uso agora).
        ui.poster().post(move |ui| {
            ui.free(form);
        });
    });
    crate::lang::translate(ui);
    let _ = ui.show(form);
    sync(ui, state, ids);
}

/// O campo selecionado no documento, se for do ClientDataSet da janela.
fn selected_field(state: &Shared) -> Option<ComponentId> {
    let cds = OPEN.with(|o| o.borrow().map(|o| o.cds))?;
    let st = state.borrow();
    st.doc.selection.filter(|&s| st.doc.ui.parent(s) == Some(cds) && st.doc.ui.class_name(s) == Some("Field"))
}

/// Refaz a lista (chamado no refresh da IDE: um campo renomeado no Object Inspector, um
/// campo novo, outra aba...). Sem janela aberta, não faz nada.
pub fn sync(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(open) = OPEN.with(|o| *o.borrow()) else { return };
    if !ui.contains(open.form) {
        OPEN.with(|o| *o.borrow_mut() = None);
        return;
    }
    let (title, fields, selected) = {
        let st = state.borrow();
        let doc = &st.doc;
        if !doc.ui.contains(open.cds) {
            drop(st);
            clear(ui, open.list);
            let l = ui.add(Label::new("O ClientDataSet não está no form aberto."), open.list);
            ui.control_mut(l).unwrap().set_align(Align::Top).set_height(28.0);
            muted(ui, l);
            return;
        }
        let name = doc.ui.name(open.cds).unwrap_or("ClientDataSet").to_string();
        let fields: Vec<(ComponentId, String)> = doc
            .ui
            .children(open.cds)
            .iter()
            .copied()
            .filter(|&f| doc.ui.class_name(f) == Some("Field"))
            .map(|f| {
                let kind = match doc.ui.get_prop(f, "type") {
                    Ok(PropValue::Enum(t)) => t,
                    _ => String::new(),
                };
                (f, format!("{}   ·   {kind}", doc.ui.name(f).unwrap_or_default()))
            })
            .collect();
        (format!("Campos de {name}"), fields, doc.selection)
    };
    let _ = ui.set_prop(open.form, "caption", ui.tr(&title));
    clear(ui, open.list);
    if fields.is_empty() {
        let l = ui.add(Label::new("Nenhum campo. Use + Adicionar."), open.list);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(28.0);
        muted(ui, l);
    }
    for (field, text) in fields {
        let r = row(ui, open.list, &text, 0.0, selected == Some(field), 28.0);
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(r, move |ui, _| {
            // O Object Inspector passa a mostrar o campo (todas as propriedades dele).
            {
                let mut st = state.borrow_mut();
                st.doc.select(Some(field));
                st.events_tab = false;
            }
            refresh(ui, &state, &ids);
        });
    }
}
