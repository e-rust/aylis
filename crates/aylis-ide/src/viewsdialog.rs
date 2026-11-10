//! O editor das vistas do form ("Editar vistas…" no seletor da barra): criar, renomear e apagar
//! vistas e mudar as condições (largura, altura, orientação, plataforma) e as escalas de
//! fonte e de espaços de cada uma. O que cada vista muda nos componentes continua sendo
//! editado no designer, com a vista à mostra.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, ComboBox, Edit, Layout, ScrollBox};
use aylis_core::views::PLATFORMS;
use aylis_core::{Align, ComponentId, FormView, PropEnum, Ui, ViewOrientation};

use crate::app::{Ids, refresh};
use crate::dialogs::{field, new_dialog, run};
use crate::state::Shared;
use crate::widgets::{clear, label, muted, tool};

const ORIENTATIONS: [&str; 3] = ["Qualquer", "Retrato", "Paisagem"];
const PLATFORM_LABELS: [&str; 6] = ["Windows", "Linux", "macOS", "Android", "iOS", "Web"];

/// Os editores de uma vista na lista.
struct Row {
    name: ComponentId,
    min_width: ComponentId,
    max_width: ComponentId,
    min_height: ComponentId,
    max_height: ComponentId,
    orientation: ComponentId,
    platforms: Vec<ComponentId>,
    font: ComponentId,
    spacing: ComponentId,
}

/// Uma vista no editor: o nome que ela tinha no form (`None`: nova) e como está agora.
type Item = (Option<String>, FormView);

/// Um número digitado (vazio: `default`; aceita a vírgula decimal).
pub fn number(text: &str, default: f32) -> Option<f32> {
    let text = text.trim();
    if text.is_empty() {
        return Some(default);
    }
    text.replace(',', ".").parse::<f32>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

/// O texto de um número no editor (0 de limite fica vazio: sem limite).
fn shown(value: f32, empty: f32) -> String {
    if value == empty { String::new() } else { format!("{value}") }
}

/// A vista como está nos editores de `row` (por cima de `view`, para o que não é editado).
fn row_view(ui: &Ui, row: &Row, view: &FormView) -> Result<FormView, String> {
    let text = |id: ComponentId| ui.get::<Edit>(id).map(|e| e.text.clone()).unwrap_or_default();
    let mut v = view.clone();
    v.name = text(row.name).trim().to_string();
    let label = if v.name.is_empty() { ui.tr("(sem nome)") } else { v.name.clone() };
    if v.name.is_empty() {
        return Err(ui.tr("a vista precisa de um nome"));
    }
    if v.name.contains(';') {
        return Err(format!("{label}: {}", ui.tr("o nome não pode ter ponto e vírgula")));
    }
    let limit = |id: ComponentId| number(&text(id), 0.0).ok_or_else(|| format!("{label}: {} \"{}\"", ui.tr("número inválido"), text(id)));
    (v.min_width, v.max_width, v.min_height, v.max_height) = (limit(row.min_width)?, limit(row.max_width)?, limit(row.min_height)?, limit(row.max_height)?);
    if (v.max_width > 0.0 && v.min_width > v.max_width) || (v.max_height > 0.0 && v.min_height > v.max_height) {
        return Err(format!("{label}: {}", ui.tr("o mínimo passa do máximo")));
    }
    let scale = |id: ComponentId| {
        number(&text(id), 1.0).filter(|s| (0.25..=4.0).contains(s)).ok_or_else(|| format!("{label}: {}", ui.tr("a escala vai de 0,25 a 4")))
    };
    (v.font_scale, v.spacing_scale) = (scale(row.font)?, scale(row.spacing)?);
    let index = ui.get::<ComboBox>(row.orientation).map_or(0, |c| c.item_index.max(0) as usize);
    v.orientation = ViewOrientation::from_index(index).unwrap_or_default();
    v.platforms = PLATFORMS
        .iter()
        .zip(&row.platforms)
        .filter(|(_, c)| ui.get::<CheckBox>(**c).is_some_and(|c| c.checked))
        .map(|(p, _)| p.to_string())
        .collect();
    Ok(v)
}

/// Um campo de número pequeno numa linha.
fn small_edit(ui: &mut Ui, line: ComponentId, before: &str, value: String, width: f32) -> ComponentId {
    if !before.is_empty() {
        let l = label(ui, line, before, Align::Left, 0.0);
        ui.control_mut(l).unwrap().set_width(before.chars().count() as f32 * 7.5 + 12.0).set_margins([8.0, 0.0, 4.0, 0.0]);
        muted(ui, l);
    }
    let e = ui.add(Edit::new(value), line);
    ui.control_mut(e).unwrap().set_align(Align::Left).set_width(width);
    e
}

/// Abre o editor das vistas do form aberto; com OK, troca as vistas (um passo do desfazer).
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    // Da mais larga para a mais estreita, como no seletor.
    let items: Rc<RefCell<Vec<Item>>> = Rc::new(RefCell::new(state.borrow().doc.views().into_iter().rev().map(|v| (Some(v.name.clone()), v)).collect()));
    let d = new_dialog(ui, "Vistas do form", 760.0, 640.0);
    let intro = label(
        ui,
        d.body,
        "Cada vista muda o form quando as condições dela valem (vazio: sem limite; nenhuma plataforma marcada: todas). Vale uma por vez: a primeira, da mais estreita para a mais larga, cujas condições todas valem. A escala da fonte e a dos espaços (padding, margens e espaço das grades) valem para o form inteiro. O que a vista muda nos componentes se edita no designer, com ela escolhida.",
        Align::Top,
        78.0,
    );
    let _ = ui.set_prop(intro, "wrap", true);
    muted(ui, intro);
    let tools = ui.add(Layout::default(), d.body);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 8.0, 0.0, 8.0]);
    let add = tool(ui, tools, "+ Vista", 100.0, "");
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let rows: Rc<RefCell<Vec<Row>>> = Rc::default();

    // Lê o que foi digitado antes de remontar a lista (o que estiver inválido fica como era).
    let read = {
        let (items, rows) = (items.clone(), rows.clone());
        move |ui: &Ui| {
            let mut items = items.borrow_mut();
            for (item, row) in items.iter_mut().zip(rows.borrow().iter()) {
                if let Ok(v) = row_view(ui, row, &item.1) {
                    item.1 = v;
                }
            }
        }
    };
    type Rebuild = Rc<RefCell<Option<Box<dyn Fn(&mut Ui)>>>>;
    let rebuild: Rebuild = Rc::default();
    {
        let (items, rows, rebuild2, read) = (items.clone(), rows.clone(), rebuild.clone(), read.clone());
        *rebuild.borrow_mut() = Some(Box::new(move |ui: &mut Ui| {
            clear(ui, list);
            rows.borrow_mut().clear();
            let count = items.borrow().len();
            if count == 0 {
                let empty = label(ui, list, "Sem vistas: o form é o mesmo em qualquer tamanho.", Align::Top, 32.0);
                muted(ui, empty);
            }
            for i in 0..count {
                let v = items.borrow()[i].1.clone();
                let block = ui.add(Layout::default(), list);
                ui.control_mut(block).unwrap().set_align(Align::Top).set_height(4.0 * 42.0 + 8.0).set_margins([0.0, 0.0, 12.0, 12.0]);

                let line = field(ui, block, "Nome");
                let remove = ui.add(Button::new("✕"), line);
                ui.control_mut(remove).unwrap().set_align(Align::Right).set_width(32.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
                let _ = ui.set_prop(remove, "hint", "Apagar a vista (com o que ela muda nos componentes)");
                let name = ui.add(Edit::new(v.name.clone()), line);
                ui.control_mut(name).unwrap().set_align(Align::Client);
                {
                    let (items, rebuild, read) = (items.clone(), rebuild2.clone(), read.clone());
                    ui.on_click(remove, move |ui, _| {
                        read(ui);
                        items.borrow_mut().remove(i);
                        if let Some(f) = rebuild.borrow().as_ref() {
                            f(ui);
                        }
                    });
                }

                let line = field(ui, block, "Largura");
                let min_width = small_edit(ui, line, "de", shown(v.min_width, 0.0), 70.0);
                let max_width = small_edit(ui, line, "até", shown(v.max_width, 0.0), 70.0);
                let h = label(ui, line, "Altura", Align::Left, 0.0);
                ui.control_mut(h).unwrap().set_width(70.0).set_margins([24.0, 0.0, 0.0, 0.0]);
                muted(ui, h);
                let min_height = small_edit(ui, line, "de", shown(v.min_height, 0.0), 70.0);
                let max_height = small_edit(ui, line, "até", shown(v.max_height, 0.0), 70.0);

                let line = field(ui, block, "Orientação");
                let orientation = ui.add(ComboBox::new(&ORIENTATIONS), line);
                ui.control_mut(orientation).unwrap().set_align(Align::Left).set_width(130.0);
                let _ = ui.set_prop(orientation, "item_index", v.orientation.index() as i64);
                let font = small_edit(ui, line, "Fonte ×", shown(v.font_scale, 1.0), 60.0);
                let _ = ui.set_prop(font, "placeholder", "1");
                let spacing = small_edit(ui, line, "Espaços ×", shown(v.spacing_scale, 1.0), 60.0);
                let _ = ui.set_prop(spacing, "placeholder", "1");

                let line = field(ui, block, "Plataformas");
                let platforms = PLATFORMS
                    .iter()
                    .zip(PLATFORM_LABELS)
                    .map(|(p, text)| {
                        let c = ui.add(CheckBox::new(text), line);
                        ui.control_mut(c).unwrap().set_align(Align::Left).set_width(100.0);
                        if let Some(c) = ui.get_mut::<CheckBox>(c) {
                            c.checked = v.platforms.iter().any(|x| x == p);
                        }
                        c
                    })
                    .collect();
                rows.borrow_mut().push(Row { name, min_width, max_width, min_height, max_height, orientation, platforms, font, spacing });
            }
            crate::lang::translate(ui);
        }));
    }
    if let Some(f) = rebuild.borrow().as_ref() {
        f(ui);
    }
    {
        let (items, rebuild, read) = (items.clone(), rebuild.clone(), read.clone());
        ui.on_click(add, move |ui, _| {
            read(ui);
            let name = (1..).map(|n| format!("vista{n}")).find(|n| !items.borrow().iter().any(|(_, v)| v.name == *n)).unwrap_or_default();
            items.borrow_mut().insert(0, (None, FormView::new(name, 0.0)));
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    let (state, ids) = (state.clone(), ids.clone());
    let (items_v, rows_v) = (items.clone(), rows.clone());
    run(
        ui,
        d,
        move |ui| {
            let mut names = Vec::new();
            for (item, row) in items_v.borrow().iter().zip(rows_v.borrow().iter()) {
                let v = row_view(ui, row, &item.1)?;
                if names.contains(&v.name) {
                    return Err(format!("{}: {}", v.name, ui.tr("já existe uma vista com esse nome")));
                }
                names.push(v.name);
            }
            Ok(())
        },
        move |ui| {
            read(ui);
            let list: Vec<Item> = items.borrow().clone();
            // A lista guarda a função que remonta a si mesma: quebra o ciclo.
            rebuild.borrow_mut().take();
            let result = state.borrow_mut().doc.set_views(list);
            if let Err(message) = result {
                aylis_controls::message_dlg(ui, &message, aylis_controls::DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
            }
            refresh(ui, &state, &ids);
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_take_the_decimal_comma_and_empty_is_the_default() {
        assert_eq!(number("", 0.0), Some(0.0));
        assert_eq!(number("   ", 1.0), Some(1.0));
        assert_eq!(number("599", 0.0), Some(599.0));
        assert_eq!(number(" 1,25 ", 1.0), Some(1.25));
        assert_eq!(number("0.5", 1.0), Some(0.5));
        assert_eq!(number("-3", 0.0), None, "negativo não");
        assert_eq!(number("abc", 0.0), None);
        assert_eq!(number("inf", 0.0), None);
        assert_eq!(number("1,2,3", 0.0), None);
    }

    #[test]
    fn a_zero_limit_and_a_scale_of_one_show_empty() {
        assert_eq!(shown(0.0, 0.0), "");
        assert_eq!(shown(1.0, 1.0), "");
        assert_eq!(shown(599.0, 0.0), "599");
        assert_eq!(shown(1.25, 1.0), "1.25");
    }
}
