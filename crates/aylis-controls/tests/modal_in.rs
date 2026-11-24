//! O modal de uma parte da tela (`Ui::show_modal_in`): só o contêiner fica bloqueado, o foco
//! fica no form, o Esc cancela e o form de fora fechando cancela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

struct Setup {
    ui: Ui,
    main: ComponentId,
    page: ComponentId,
    page_edit: ComponentId,
    other_edit: ComponentId,
    dialog: ComponentId,
    dialog_edit: ComponentId,
    ok: ComponentId,
}

fn setup() -> Setup {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.add_root(Form::new("Principal", 800.0, 600.0));
    // Duas "abas": a página com o modal e a outra.
    let page = ui.add(Panel::default(), main);
    ui.control_mut(page).unwrap().set_align(Align::Left).set_width(400.0);
    let page_edit = ui.add(Edit::new(""), page);
    let other = ui.add(Panel::default(), main);
    ui.control_mut(other).unwrap().set_align(Align::Client);
    let other_edit = ui.add(Edit::new(""), other);
    // O form da pergunta, solto.
    let dialog = ui.add_root(Form::new("Pergunta", 300.0, 160.0));
    let dialog_edit = ui.add(Edit::new(""), dialog);
    let ok = ui.add(Button::new("OK"), dialog);
    ui.set_prop(ok, "modal_result", "ok").unwrap();
    ui.show(main).unwrap();
    ui.layout();
    ui.set_focus(page_edit);
    Setup { ui, main, page, page_edit, other_edit, dialog, dialog_edit, ok }
}

fn key(ui: &mut Ui, root: ComponentId, key: Key, shift: bool) {
    let mut event = KeyEvent { key, ..Default::default() };
    event.modifiers.shift = shift;
    ui.key_down(root, event);
    ui.key_up(root, event);
}

fn result_log() -> (Rc<RefCell<Vec<ModalResult>>>, impl FnOnce(&mut Ui, ModalResult) + 'static) {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    (log, move |_: &mut Ui, r| l.borrow_mut().push(r))
}

#[test]
fn the_modal_blocks_only_its_container() {
    let Setup { mut ui, main, page, page_edit, other_edit, dialog, dialog_edit, ok } = setup();
    let (log, on_close) = result_log();
    ui.show_modal_in(dialog, page, on_close).unwrap();
    ui.layout();
    assert_eq!(ui.root_of(dialog), main, "o form fica por cima da página, na mesma janela");
    assert_eq!(ui.focused(main), Some(dialog_edit), "o foco vai para o form");
    assert!(!ui.is_open(dialog) && !ui.input_blocked(main), "a janela não fica bloqueada");
    // O que está atrás, na página, não recebe o foco; o da outra aba, sim.
    assert!(!ui.set_focus(page_edit));
    assert!(ui.set_focus(other_edit));
    ui.set_focus(dialog_edit);
    // O Tab dá a volta no form.
    key(&mut ui, main, Key::Tab, false);
    assert!(ui.focused(main).is_some_and(|f| f == dialog_edit || ui.is_ancestor(dialog, f)), "o Tab fica no form");
    // O mouse na página cai no véu, não no campo de trás.
    let rect = ui.absolute_rect(page_edit).unwrap();
    let hit = ui.hit_test(main, PointF::new(rect.x + 2.0, rect.y + 2.0));
    assert_ne!(hit, Some(page_edit));
    // O mesmo retorno do show_modal.
    ui.click(ok);
    assert_eq!(*log.borrow(), [ModalResult::Ok]);
    assert_eq!(ui.parent(dialog), None, "fechado, volta a ser um form solto");
    assert!(ui.set_focus(page_edit), "a página volta a responder");
    assert!(ui.contains(dialog), "não é liberado");
}

#[test]
fn esc_cancels_and_closing_the_outer_form_cancels_too() {
    let Setup { mut ui, main, page, dialog, .. } = setup();
    let (log, on_close) = result_log();
    ui.show_modal_in(dialog, page, on_close).unwrap();
    key(&mut ui, main, Key::Escape, false);
    assert_eq!(*log.borrow(), [ModalResult::Cancel], "o Esc cancela");

    let (log, on_close) = result_log();
    ui.show_modal_in(dialog, page, on_close).unwrap();
    // Um segundo form aberto para não encerrar o app ao fechar o principal.
    let other = ui.add_root(Form::new("Outra", 200.0, 100.0));
    ui.show(other).unwrap();
    ui.set_main_form(other);
    ui.close(main);
    assert_eq!(*log.borrow(), [ModalResult::Cancel], "o form de fora fechou: cancelado");
    assert_eq!(ui.parent(dialog), None);
}

#[test]
fn the_modal_goes_along_with_its_container() {
    let Setup { mut ui, main, page, dialog, .. } = setup();
    let (log, on_close) = result_log();
    ui.show_modal_in(dialog, page, on_close).unwrap();
    // A página arrancada para outra janela leva o modal.
    let window = ui.add_root(Form::new("Janela", 600.0, 400.0));
    ui.set_parent(page, Some(window)).unwrap();
    ui.show(window).unwrap();
    assert_eq!(ui.root_of(dialog), window);
    let other = ui.add_root(Form::new("Outra", 200.0, 100.0));
    ui.show(other).unwrap();
    ui.set_main_form(other);
    ui.close(main);
    assert!(log.borrow().is_empty(), "o form de onde a página saiu fechou: o modal continua");
    ui.close(window);
    assert_eq!(*log.borrow(), [ModalResult::Cancel]);
}

#[test]
fn show_modal_in_refuses_a_form_that_is_open_or_inside_the_container() {
    let Setup { mut ui, main, page, page_edit, .. } = setup();
    assert!(ui.show_modal_in(main, page, |_, _| {}).is_err(), "o form aberto");
    assert!(ui.show_modal_in(page_edit, page, |_, _| {}).is_err(), "um controle de dentro");
}
