//! A janela de colunas do DataGrid: fica aberta ao lado
//! da IDE, lista as colunas (componentes `Column` filhos da grade) na ordem e, ao clicar
//! numa, seleciona-a — o Object Inspector mostra o campo, o título, a largura, o
//! alinhamento, as cores. Adicionar, todos os campos, excluir, subir e descer.

use std::cell::RefCell;

use aylis_controls::{Button, Form, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, PropValue, Ui};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{clear, muted, row};

/// A janela aberta: o form dela, a lista e a grade que ela mostra.
#[derive(Clone, Copy)]
struct Open {
    form: ComponentId,
    list: ComponentId,
    grid: ComponentId,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

/// Abre (ou traz para a frente, já mostrando esta grade) a janela de colunas.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids, grid: ComponentId) {
    if let Some(open) = OPEN.with(|o| *o.borrow()).filter(|o| ui.contains(o.form) && ui.is_open(o.form)) {
        OPEN.with(|o| *o.borrow_mut() = Some(Open { grid, ..open }));
        let _ = ui.show(open.form);
        sync(ui, state, ids);
        return;
    }
    let form = ui.add_root(Form::new("Colunas", 380.0, 480.0));
    if let Some(cb) = ui.control_mut(form) {
        cb.set_padding(12.0);
    }
    let _ = ui.set_prop(form, "topmost", true);

    let tools = ui.add(Layout::default(), form);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let mut button = |text: &str, width: f32, hint: &str| {
        let b = ui.add(Button::new(text), tools);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 6.0, 0.0]).set_style("flat");
        let _ = ui.set_prop(b, "hint", hint);
        b
    };
    let add = button("+ Adicionar", 100.0, "Nova coluna, com o próximo campo que a grade ainda não mostra");
    let all = button("Todos os campos", 124.0, "Uma coluna para cada campo visível que ainda não tem");
    let remove = button("Excluir", 70.0, "Exclui a coluna selecionada");
    let tools2 = ui.add(Layout::default(), form);
    ui.control_mut(tools2).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let up = ui.add(Button::new("↑ Subir"), tools2);
    ui.control_mut(up).unwrap().set_align(Align::Left).set_width(80.0).set_margins([0.0, 0.0, 6.0, 0.0]).set_style("flat");
    let down = ui.add(Button::new("↓ Descer"), tools2);
    ui.control_mut(down).unwrap().set_align(Align::Left).set_width(80.0).set_style("flat");
    let tip = ui.add(
        Label::new("Clique numa coluna: o Object Inspector mostra as propriedades dela (campo, título, largura, alinhamento, cores). A largura também muda arrastando o separador do cabeçalho no designer."),
        form,
    );
    ui.control_mut(tip).unwrap().set_align(Align::Bottom).set_height(62.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let _ = ui.set_prop(tip, "wrap", true);
    let _ = ui.set_prop(tip, "font_size", 11.0);
    muted(ui, tip);
    let list = ui.add(ScrollBox::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    OPEN.with(|o| *o.borrow_mut() = Some(Open { form, list, grid }));
    let current = || OPEN.with(|o| o.borrow().map(|o| o.grid));
    for (b, every) in [(add, false), (all, true)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(b, move |ui, _| {
            let Some(grid) = current() else { return };
            {
                let mut st = state.borrow_mut();
                st.status = match add_columns(&mut st.doc, grid, every) {
                    Ok(0) => "Todos os campos já têm coluna".to_string(),
                    Ok(1) => "Coluna adicionada".to_string(),
                    Ok(n) => format!("{n} colunas adicionadas"),
                    Err(e) => format!("⚠ {e}"),
                };
            }
            refresh(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(remove, move |ui, _| {
            if selected_column(&state).is_some() {
                state.borrow_mut().doc.delete_selected();
                refresh(ui, &state, &ids);
            }
        });
    }
    for (b, delta) in [(up, -1), (down, 1)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(b, move |ui, _| {
            if let Some(column) = selected_column(&state) {
                state.borrow_mut().doc.move_among_siblings(column, delta);
                refresh(ui, &state, &ids);
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

/// Os campos do dataset ligado à grade (o `field_name` de cada `Field`), na ordem, com o
/// `visible` de cada um.
fn grid_fields(ui: &Ui, grid: ComponentId) -> Vec<(String, bool)> {
    let Ok(PropValue::Component(Some(ds))) = ui.get_prop(grid, "data_source") else { return Vec::new() };
    let Ok(PropValue::Component(Some(cds))) = ui.get_prop(ds, "data_set") else { return Vec::new() };
    ui.children(cds)
        .iter()
        .filter(|&&f| ui.class_name(f) == Some("Field"))
        .filter_map(|&f| {
            let name = match ui.get_prop(f, "field_name") {
                Ok(PropValue::String(n)) if !n.is_empty() => n,
                _ => ui.name(f)?.to_string(),
            };
            let visible = !matches!(ui.get_prop(f, "visible"), Ok(PropValue::Bool(false)));
            Some((name, visible))
        })
        .collect()
}

fn columns_of(ui: &Ui, grid: ComponentId) -> Vec<ComponentId> {
    ui.children(grid).iter().copied().filter(|&c| ui.class_name(c) == Some("Column")).collect()
}

/// Adiciona colunas à grade, num passo só do desfazer: a do próximo campo sem coluna ou,
/// com `every`, a de cada campo visível sem coluna. Sem colunas ainda, a lista `columns`
/// da grade (com as larguras) vira as primeiras colunas, e a lista fica vazia.
fn add_columns(doc: &mut crate::document::Document, grid: ComponentId, every: bool) -> Result<usize, String> {
    let fields = grid_fields(&doc.ui, grid);
    if fields.is_empty() {
        return Err("ligue a grade a um DataSource com campos antes".into());
    }
    let existing = columns_of(&doc.ui, grid);
    let mut wanted: Vec<(String, f32)> = Vec::new();
    if existing.is_empty() {
        if let Ok(PropValue::String(list)) = doc.ui.get_prop(grid, "columns") {
            for item in list.split(';').map(str::trim).filter(|s| !s.is_empty()) {
                let (name, width) = match item.split_once(':') {
                    Some((n, w)) => (n.trim().to_string(), w.trim().parse().unwrap_or(0.0)),
                    None => (item.to_string(), 0.0),
                };
                wanted.push((name, width));
            }
        }
    }
    let used: Vec<String> = existing
        .iter()
        .filter_map(|&c| match doc.ui.get_prop(c, "field_name") {
            Ok(PropValue::String(n)) => Some(n),
            _ => None,
        })
        .chain(wanted.iter().map(|(n, _)| n.clone()))
        .collect();
    let free: Vec<String> = fields.iter().filter(|(n, visible)| *visible && !used.contains(n)).map(|(n, _)| n.clone()).collect();
    if every || wanted.is_empty() {
        let take = if every { free.len() } else { free.len().min(1) };
        wanted.extend(free.into_iter().take(take).map(|n| (n, 0.0)));
    }
    if wanted.is_empty() {
        return Ok(0);
    }
    doc.begin_change();
    let grid_name = doc.ui.name(grid).unwrap_or("grid").to_string();
    let mut last = None;
    for (field, width) in &wanted {
        let id = doc.ui.create("Column", Some(grid)).map_err(|e| e.to_string())?;
        let _ = doc.ui.set_name(id, &column_name(doc, &grid_name, field));
        let _ = doc.ui.set_prop(id, "field_name", field.as_str());
        if *width > 0.0 {
            let _ = doc.ui.set_prop(id, "width", *width);
        }
        last = Some(id);
    }
    let _ = doc.ui.set_prop(grid, "columns", "");
    doc.select(last);
    Ok(wanted.len())
}

/// `grdEstados` + `sigla` = `grdEstadosSigla` (com número no fim se já existir).
fn column_name(doc: &crate::document::Document, grid: &str, field: &str) -> String {
    let pascal: String = field
        .split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        })
        .collect();
    let base = format!("{grid}{pascal}");
    if doc.ui.find_in(doc.root, &base).is_none() {
        return base;
    }
    (2..).map(|i| format!("{base}{i}")).find(|n| doc.ui.find_in(doc.root, n).is_none()).unwrap_or(base)
}

/// A coluna selecionada no documento, se for da grade da janela.
fn selected_column(state: &Shared) -> Option<ComponentId> {
    let grid = OPEN.with(|o| o.borrow().map(|o| o.grid))?;
    let st = state.borrow();
    st.doc.selection.filter(|&s| st.doc.ui.parent(s) == Some(grid) && st.doc.ui.class_name(s) == Some("Column"))
}

/// Refaz a lista (chamado no refresh da IDE). Sem janela aberta, não faz nada.
pub fn sync(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(open) = OPEN.with(|o| *o.borrow()) else { return };
    if !ui.contains(open.form) {
        OPEN.with(|o| *o.borrow_mut() = None);
        return;
    }
    let (title, columns, selected) = {
        let st = state.borrow();
        let doc = &st.doc;
        if !doc.ui.contains(open.grid) {
            drop(st);
            clear(ui, open.list);
            let l = ui.add(Label::new("O DataGrid não está no form aberto."), open.list);
            ui.control_mut(l).unwrap().set_align(Align::Top).set_height(28.0);
            muted(ui, l);
            return;
        }
        let name = doc.ui.name(open.grid).unwrap_or("DataGrid").to_string();
        let text = |c: ComponentId, prop: &str| match doc.ui.get_prop(c, prop) {
            Ok(PropValue::String(s)) => s,
            _ => String::new(),
        };
        let columns: Vec<(ComponentId, String)> = columns_of(&doc.ui, open.grid)
            .into_iter()
            .map(|c| {
                let field = text(c, "field_name");
                let field = if field.is_empty() { "(sem campo)".to_string() } else { field };
                let width = match doc.ui.get_prop(c, "width") {
                    Ok(PropValue::Float(w)) if w > 0.0 => format!("   ·   {w:.0} px"),
                    _ => String::new(),
                };
                let hidden = if matches!(doc.ui.get_prop(c, "visible"), Ok(PropValue::Bool(false))) { "   ·   oculta" } else { "" };
                (c, format!("{}   ·   {field}{width}{hidden}", doc.ui.name(c).unwrap_or_default()))
            })
            .collect();
        (format!("Colunas de {name}"), columns, doc.selection)
    };
    let _ = ui.set_prop(open.form, "caption", ui.tr(&title));
    clear(ui, open.list);
    if columns.is_empty() {
        let l = ui.add(Label::new("Nenhuma coluna: a grade mostra os campos da lista columns (ou todos). Use + Adicionar ou Todos os campos."), open.list);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(48.0);
        let _ = ui.set_prop(l, "wrap", true);
        muted(ui, l);
    }
    for (column, text) in columns {
        let r = row(ui, open.list, &text, 0.0, selected == Some(column), 28.0);
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(r, move |ui, _| {
            {
                let mut st = state.borrow_mut();
                st.doc.select(Some(column));
                st.events_tab = false;
            }
            refresh(ui, &state, &ids);
        });
    }
}
