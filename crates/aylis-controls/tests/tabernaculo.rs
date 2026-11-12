//! O relatório do Tabernáculo (11/11): o arquivo solto (R04: F01, F02, F04), os atalhos do
//! menu de contexto como no Delphi (J01) e o padding do Label (C05).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            other => format!("{other:?}"),
        })
    });
    l
}

/// F01: soltar sobre a página à vista não manda o arquivo à zona de uma página escondida.
#[test]
fn a_dropped_file_does_not_go_to_a_hidden_zone() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 640.0, 400.0));
    let visible = ui.add(Panel::default(), form);
    let hidden = ui.add(Panel::default(), form);
    ui.control_mut(visible).unwrap().set_align(Align::Client);
    ui.control_mut(hidden).unwrap().set_align(Align::Client).set_visible(false);
    let zone = ui.add(FileDropZone::default(), hidden);
    let changes = log(&mut ui, zone, "on_change");
    ui.layout();
    assert!(!ui.drop_files(form, PointF::new(320.0, 200.0), vec!["C:\\fotos\\a.jpg".into()]));
    assert!(changes.borrow().is_empty());
}

/// F02 e F04: o "Todos os arquivos" (`*.*`) aceita qualquer arquivo; o que o filtro recusa
/// dispara `on_reject`; o `on_change` leva só os que entraram.
#[test]
fn the_drop_zone_filter_rejects_with_an_event_and_reports_only_new_files() {
    assert_eq!(parse_filter("Imagens|*.png|Todos|*.*")[1], ("Todos".to_string(), vec!["*".to_string()]));
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 640.0, 400.0));
    let zone = ui.add(FileDropZone::default(), form);
    ui.control_mut(zone).unwrap().set_align(Align::Client);
    ui.set_prop(zone, "filter", "Imagens|*.png").unwrap();
    let changes = log(&mut ui, zone, "on_change");
    let rejects = log(&mut ui, zone, "on_reject");
    ui.layout();
    let at = PointF::new(100.0, 100.0);
    ui.drop_files(form, at, vec!["C:\\a.png".into()]);
    ui.drop_files(form, at, vec!["C:\\b.png".into(), "C:\\notas.pdf".into()]);
    assert_eq!(*changes.borrow(), vec!["C:\\a.png".to_string(), "C:\\b.png".to_string()], "só os novos");
    assert_eq!(*rejects.borrow(), vec!["C:\\notas.pdf".to_string()]);
    assert_eq!(ui.get_prop(zone, "files").unwrap(), PropValue::String("C:\\a.png;C:\\b.png".into()), "a lista inteira fica em `files`");
    // Com o "Todos os arquivos", o PDF entra.
    ui.set_prop(zone, "filter", "Imagens|*.png|Todos os arquivos|*.*").unwrap();
    ui.drop_files(form, at, vec!["C:\\notas.pdf".into()]);
    assert_eq!(changes.borrow().last().map(String::as_str), Some("C:\\notas.pdf"));
}

/// J01: os atalhos de um PopupMenu valem só com ele ligado (`popup_menu`) ao controle com o
/// foco ou a um pai dele, como o `IsMenuKey` do VCL.
#[test]
fn popup_menu_shortcuts_follow_the_focused_control() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 400.0, 300.0));
    let edit_a = ui.add(Edit::default(), form);
    let edit_b = ui.add(Edit::default(), form);
    let menu = ui.add(PopupMenu::default(), form);
    let item = ui.add(MenuItem::new("Copiar linha"), menu);
    ui.set_prop(item, "shortcut", "Ctrl+L").unwrap();
    ui.set_prop(edit_a, "popup_menu", PropValue::Component(Some(menu))).unwrap();
    let clicks = log(&mut ui, item, "on_click");
    ui.show(form).unwrap();
    ui.layout();
    let ctrl_l = KeyEvent { key: Key::Char('l'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() };
    ui.set_focus(edit_b);
    ui.key_down(form, ctrl_l);
    ui.process_events();
    assert!(clicks.borrow().is_empty(), "o menu é do outro campo");
    ui.set_focus(edit_a);
    ui.key_down(form, ctrl_l);
    ui.process_events();
    assert_eq!(clicks.borrow().len(), 1, "o menu do campo com o foco");
}

/// C05: o `padding` do Label afasta o texto (antes só o do estilo valia).
#[test]
fn the_label_padding_moves_the_text() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 300.0, 100.0));
    let label = ui.add(Label::new("Texto"), form);
    ui.control_mut(label).unwrap().set_size(200.0, 40.0);
    ui.set_prop(label, "padding", 20.0).unwrap();
    ui.layout();
    let mut canvas = aylis_core::pages::RecordingCanvas::new(None);
    ui.paint(form, &mut canvas);
    let text = canvas.ops.iter().find_map(|op| match op {
        aylis_core::pages::Op::Text { rect, text, .. } if text == "Texto" => Some(*rect),
        _ => None,
    });
    let rect = text.expect("o texto foi desenhado");
    assert!(rect.x >= 20.0 && rect.y >= 20.0, "{rect:?}");
}
