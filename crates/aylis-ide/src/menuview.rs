//! O editor de menu: uma janela ao lado da IDE com a
//! árvore de itens de um `PopupMenu` ou `MainMenu` (submenus recuados, divisores como
//! linha). Clicar num item seleciona-o — o Object Inspector mostra texto, ícone, atalho,
//! `checked`, `auto_check` e o `on_click` dele. Adicionar item, subitem e divisor,
//! excluir, subir e descer.

use std::cell::RefCell;

use aylis_controls::{Button, Form, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, PropValue, Ui};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{clear, muted, row};

#[derive(Clone, Copy)]
struct Open {
    form: ComponentId,
    list: ComponentId,
    menu: ComponentId,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

/// O que um botão da janela adiciona.
#[derive(Clone, Copy, PartialEq)]
enum Add {
    /// Ao lado do item selecionado (ou no fim do menu).
    Item,
    /// Dentro do item selecionado (submenu).
    Sub,
    /// Um divisor ao lado do selecionado.
    Divider,
}

/// Abre (ou traz para a frente, mostrando este menu) o editor de menu.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids, menu: ComponentId) {
    if let Some(open) = OPEN.with(|o| *o.borrow()).filter(|o| ui.contains(o.form) && ui.is_open(o.form)) {
        OPEN.with(|o| *o.borrow_mut() = Some(Open { menu, ..open }));
        let _ = ui.show(open.form);
        sync(ui, state, ids);
        return;
    }
    let form = ui.add_root(Form::new("Menu", 380.0, 480.0));
    if let Some(cb) = ui.control_mut(form) {
        cb.set_padding(12.0);
    }
    let _ = ui.set_prop(form, "topmost", true);
    let tools = ui.add(Layout::default(), form);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 4.0]);
    let tools2 = ui.add(Layout::default(), form);
    ui.control_mut(tools2).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let mut button = |parent: ComponentId, text: &str, width: f32, hint: &str| {
        let b = ui.add(Button::new(text), parent);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 6.0, 0.0]).set_style("flat");
        let _ = ui.set_prop(b, "hint", hint);
        b
    };
    let add_item = button(tools, "+ Item", 72.0, "Novo item ao lado do selecionado");
    let add_sub = button(tools, "+ Subitem", 96.0, "Novo item dentro do selecionado (submenu)");
    let add_div = button(tools, "+ Divisor", 92.0, "Uma linha separando os itens");
    let remove = button(tools2, "Excluir", 74.0, "Exclui o item selecionado (e os subitens)");
    let up = button(tools2, "↑", 36.0, "Sobe o item");
    let down = button(tools2, "↓", 36.0, "Desce o item");
    let list = ui.add(ScrollBox::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    OPEN.with(|o| *o.borrow_mut() = Some(Open { form, list, menu }));
    for (b, kind) in [(add_item, Add::Item), (add_sub, Add::Sub), (add_div, Add::Divider)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(b, move |ui, _| {
            add(&state, kind);
            refresh(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(remove, move |ui, _| {
            if selected_item(&state).is_some() {
                state.borrow_mut().doc.delete_selected();
                refresh(ui, &state, &ids);
            }
        });
    }
    for (b, delta) in [(up, -1), (down, 1)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(b, move |ui, _| {
            if let Some(item) = selected_item(&state) {
                state.borrow_mut().doc.move_among_siblings(item, delta);
                refresh(ui, &state, &ids);
            }
        });
    }
    ui.on(form, "on_close", move |ui, form, _| {
        OPEN.with(|o| *o.borrow_mut() = None);
        ui.poster().post(move |ui| {
            ui.free(form);
        });
    });
    crate::lang::translate(ui);
    let _ = ui.show(form);
    sync(ui, state, ids);
}

/// O item selecionado no documento, se for deste menu.
fn selected_item(state: &Shared) -> Option<ComponentId> {
    let menu = OPEN.with(|o| o.borrow().map(|o| o.menu))?;
    let st = state.borrow();
    let doc = &st.doc;
    let item = doc.selection.filter(|&s| doc.ui.class_name(s) == Some("MenuItem"))?;
    let mut current = doc.ui.parent(item);
    while let Some(p) = current {
        if p == menu {
            return Some(item);
        }
        current = doc.ui.parent(p);
    }
    None
}

/// Cria o item (ou subitem, ou divisor) e o deixa selecionado, logo depois do selecionado.
fn add(state: &Shared, kind: Add) {
    let Some(menu) = OPEN.with(|o| o.borrow().map(|o| o.menu)) else { return };
    let selected = selected_item(state);
    let mut st = state.borrow_mut();
    let doc = &mut st.doc;
    // Onde entra: dentro do selecionado (subitem), ao lado dele, ou no fim do menu.
    let parent = match (kind, selected) {
        (Add::Sub, Some(item)) => item,
        (_, Some(item)) => doc.ui.parent(item).unwrap_or(menu),
        (_, None) => menu,
    };
    doc.select(Some(parent));
    let created = match doc.add("MenuItem", None) {
        Ok(id) => id,
        Err(e) => {
            st.status = format!("⚠ {e}");
            return;
        }
    };
    if kind == Add::Divider {
        let _ = doc.set_prop(created, "text", &serde_json::json!("-"));
    }
    // Logo depois do selecionado (e não no fim).
    if let (Some(item), true) = (selected, kind != Add::Sub) {
        let siblings = doc.ui.children(parent).to_vec();
        if let (Some(at), Some(now)) = (siblings.iter().position(|&c| c == item), siblings.iter().position(|&c| c == created)) {
            let steps = now as i32 - (at as i32 + 1);
            if steps > 0 {
                let _ = doc.ui.set_child_index(created, at + 1);
            }
        }
    }
    doc.select(Some(created));
}

/// Refaz a árvore (chamado no refresh da IDE). Sem janela aberta, não faz nada.
pub fn sync(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(open) = OPEN.with(|o| *o.borrow()) else { return };
    if !ui.contains(open.form) {
        OPEN.with(|o| *o.borrow_mut() = None);
        return;
    }
    let (title, rows, selected) = {
        let st = state.borrow();
        let doc = &st.doc;
        if !doc.ui.contains(open.menu) {
            drop(st);
            clear(ui, open.list);
            let l = ui.add(Label::new("O menu não está no form aberto."), open.list);
            ui.control_mut(l).unwrap().set_align(Align::Top).set_height(28.0);
            muted(ui, l);
            return;
        }
        // A árvore: (item, profundidade, texto da linha).
        fn walk(doc: &crate::document::Document, parent: ComponentId, depth: usize, out: &mut Vec<(ComponentId, usize, String)>) {
            for &c in doc.ui.children(parent) {
                if doc.ui.class_name(c) != Some("MenuItem") {
                    continue;
                }
                let text = match doc.ui.get_prop(c, "text") {
                    Ok(PropValue::String(s)) => s,
                    _ => String::new(),
                };
                let line = if text == "-" {
                    "──────────".to_string()
                } else {
                    let mut l = text;
                    if let Ok(PropValue::String(icon)) = doc.ui.get_prop(c, "icon") {
                        if !icon.is_empty() {
                            l = format!("[{icon}]  {l}");
                        }
                    }
                    if matches!(doc.ui.get_prop(c, "checked"), Ok(PropValue::Bool(true))) {
                        l = format!("✓ {l}");
                    }
                    if let Ok(PropValue::String(sc)) = doc.ui.get_prop(c, "shortcut") {
                        if !sc.is_empty() {
                            l = format!("{l}    {sc}");
                        }
                    }
                    if !doc.ui.children(c).is_empty() {
                        l.push_str("  ›");
                    }
                    l
                };
                out.push((c, depth, line));
                walk(doc, c, depth + 1, out);
            }
        }
        let mut rows = Vec::new();
        walk(doc, open.menu, 0, &mut rows);
        let name = doc.ui.name(open.menu).unwrap_or("Menu").to_string();
        (format!("Menu {name}"), rows, doc.selection)
    };
    let _ = ui.set_prop(open.form, "caption", title);
    clear(ui, open.list);
    if rows.is_empty() {
        let l = ui.add(Label::new("Nenhum item. Use + Item."), open.list);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(28.0);
        muted(ui, l);
    }
    for (item, depth, text) in rows {
        let r = row(ui, open.list, &text, depth as f32 * 22.0, selected == Some(item), 28.0);
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(r, move |ui, _| {
            {
                let mut st = state.borrow_mut();
                st.doc.select(Some(item));
                st.events_tab = false;
            }
            refresh(ui, &state, &ids);
        });
    }
}
