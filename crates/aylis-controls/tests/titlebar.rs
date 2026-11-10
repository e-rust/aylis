//! TitleBar: os botões e a barra viram pedidos à janela (a plataforma executa).

use aylis_controls::*;
use aylis_core::*;

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32, count: u8) {
    let e = PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, click_count: count, ..Default::default() };
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
    ui.process_posted();
}

#[test]
fn buttons_and_drag_become_window_commands() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("Meu app", 600.0, 400.0));
    let _ = ui.set_prop(form, "border_style", PropValue::Enum("custom".into()));
    let bar = ui.add(TitleBar::default(), form);
    ui.update_layout();
    assert_eq!(ui.bounds(bar).unwrap().height, 36.0);

    // Da direita para a esquerda: fechar, maximizar, minimizar (46 de largura cada).
    click(&mut ui, form, 600.0 - 23.0 - 46.0 * 2.0, 18.0, 1);
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::Minimize)]);
    click(&mut ui, form, 600.0 - 23.0 - 46.0, 18.0, 1);
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::ToggleMaximize)]);

    // Na barra: arrastar; duplo clique: maximizar.
    let e = PointerEvent { position: PointF::new(200.0, 18.0), button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(form, e);
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::StartDrag)]);
    ui.pointer_up(form, e);
    click(&mut ui, form, 200.0, 18.0, 2);
    assert!(ui.take_window_commands().contains(&(form, WindowCommand::ToggleMaximize)));

    // Fechar o form principal encerra o app.
    ui.show(form).unwrap();
    click(&mut ui, form, 600.0 - 23.0, 18.0, 1);
    assert!(ui.close_requested());
}

#[test]
fn own_buttons_stay_left_of_the_standard_ones() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("x", 600.0, 400.0));
    let bar = ui.add(TitleBar::default(), form);
    let mine = ui.add(Button::new("☰"), bar);
    ui.control_mut(mine).unwrap().set_align(Align::Right).set_width(40.0);
    ui.update_layout();
    let r = ui.absolute_rect(mine).unwrap();
    assert!(r.right() <= 600.0 - 46.0 * 3.0, "{r:?}");
}
