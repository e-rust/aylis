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
    // A montagem (a `border_style` pede os cantos à janela, que ainda não existe).
    let _ = ui.take_window_commands();

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

/// Os cantos da janela (`window_corners`): o padrão, e mudar com o app rodando pede à janela
/// que acerte os cantos; no designer não há janela.
#[test]
fn changing_the_window_corners_asks_the_window() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("Meu app", 600.0, 400.0));
    assert_eq!(ui.get_prop(form, "window_corners").unwrap(), PropValue::Enum("default".into()));
    let _ = ui.take_window_commands();
    ui.set_prop(form, "window_corners", PropValue::Enum("small".into())).unwrap();
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::UpdateCorners)]);
    // A `border_style` muda o padrão dos cantos (a barra própria arredonda).
    ui.set_prop(form, "border_style", PropValue::Enum("custom".into())).unwrap();
    assert!(ui.take_window_commands().contains(&(form, WindowCommand::UpdateCorners)));
    assert!(ui.set_prop(form, "window_corners", PropValue::Enum("oval".into())).is_err());
    ui.set_design_mode(true);
    ui.set_prop(form, "window_corners", PropValue::Enum("square".into())).unwrap();
    assert!(ui.take_window_commands().is_empty());
}

/// `topmost` mudado com a janela aberta: a janela vem para a frente (ou sai); antes só valia na
/// criação (relatório do RTSPlayer de 13/11, item 4).
#[test]
fn changing_topmost_asks_the_window() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("Meu app", 600.0, 400.0));
    let _ = ui.take_window_commands();
    ui.set_prop(form, "topmost", true).unwrap();
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::SetTopmost(true))]);
    ui.set_prop(form, "topmost", false).unwrap();
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::SetTopmost(false))]);
    ui.set_design_mode(true);
    ui.set_prop(form, "topmost", true).unwrap();
    assert!(ui.take_window_commands().is_empty());
}

/// O botão maximizar da barra própria vai para o `Ui` (a plataforma o entrega ao sistema, que
/// no Windows 11 mostra o menu de layouts com o mouse parado nele).
#[test]
fn the_maximize_button_is_told_to_the_window() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("Meu app", 600.0, 400.0));
    let _ = ui.set_prop(form, "border_style", PropValue::Enum("custom".into()));
    let top = ui.add(Panel::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(10.0);
    let bar = ui.add(TitleBar::default(), form);
    ui.update_layout();
    // Fechar, maximizar, minimizar da direita para a esquerda (46 cada), abaixo do painel.
    assert_eq!(ui.maximize_button(form), Some(RectF::new(600.0 - 46.0 * 2.0, 10.0, 46.0, 36.0)));
    // A janela muda de largura: o botão acompanha.
    ui.control_mut(form).unwrap().set_width(800.0);
    ui.update_layout();
    assert_eq!(ui.maximize_button(form), Some(RectF::new(800.0 - 46.0 * 2.0, 10.0, 46.0, 36.0)));
    // Sem o botão, escondida ou liberada: nada para o sistema.
    ui.set_prop(bar, "show_maximize", false).unwrap();
    assert_eq!(ui.maximize_button(form), None);
    ui.set_prop(bar, "show_maximize", true).unwrap();
    assert!(ui.maximize_button(form).is_some());
    ui.control_mut(bar).unwrap().set_visible(false);
    assert_eq!(ui.maximize_button(form), None);
    ui.control_mut(bar).unwrap().set_visible(true);
    ui.free(bar);
    assert_eq!(ui.maximize_button(form), None);
}
