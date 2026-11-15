//! A orientação da tela no celular (pedido do autor em 14/11): a do app (o projeto), a do form à
//! frente e a de desenvolvimento, por cima de tudo. O núcleo calcula o pedido; a plataforma
//! aplica quando ele muda.

use aylis_core::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.create("Form", None).unwrap();
    ui.show(main).unwrap();
    (ui, main)
}

#[test]
fn project_uses_the_app_orientation() {
    let (mut ui, main) = setup();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Any, "sem nada, gira com o aparelho");
    assert_eq!(ui.get_prop(main, "orientation").unwrap(), PropValue::Enum("project".into()), "o padrão do form é o do projeto");
    ui.set_app_orientation(ScreenOrientation::Portrait);
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Portrait);
}

#[test]
fn the_form_in_front_asks_for_its_own_and_closing_gives_back_the_one_below() {
    let (mut ui, main) = setup();
    ui.set_app_orientation(ScreenOrientation::Portrait);
    let video = ui.create("Form", None).unwrap();
    ui.set_prop(video, "orientation", "sensor_landscape").unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Portrait, "fechado, não conta");
    ui.show(video).unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::SensorLandscape, "à frente, a dele");
    // Um form `project` por cima: a do app, não a do de baixo.
    let about = ui.create("Form", None).unwrap();
    ui.show(about).unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Portrait);
    ui.close(about);
    assert_eq!(ui.requested_orientation(), ScreenOrientation::SensorLandscape, "fechou: a do form de baixo");
    ui.close(video);
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Portrait, "fechou: a do app");
    // Um form com a sua no lugar do principal.
    ui.set_prop(main, "orientation", "landscape").unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Landscape);
}

#[test]
fn changing_while_running_changes_the_request() {
    let (mut ui, main) = setup();
    ui.set_prop(main, "orientation", "portrait").unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Portrait);
    ui.set_prop(main, "orientation", "landscape").unwrap();
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Landscape, "a propriedade muda o pedido na hora");
    ui.set_prop(main, "orientation", "project").unwrap();
    // O app todo em execução (a do projeto): o `set_orientation` do `Screen`, que vale em
    // todas as linguagens.
    let screen = ui.create("Screen", Some(main)).unwrap();
    ui.invoke(screen, "set_orientation", &[PropValue::String("sensor_portrait".into())]).unwrap();
    assert_eq!(ui.app_orientation(), ScreenOrientation::SensorPortrait);
    assert_eq!(ui.invoke(screen, "orientation", &[]).unwrap(), PropValue::String("sensor_portrait".into()));
    assert!(ui.invoke(screen, "set_orientation", &[PropValue::String("sideways".into())]).is_err(), "nome errado é erro");
    assert!(ui.set_prop(main, "orientation", "sideways").is_err());
}

#[test]
fn the_development_override_wins_over_project_and_form() {
    let (mut ui, main) = setup();
    ui.set_app_orientation(ScreenOrientation::Portrait);
    ui.set_prop(main, "orientation", "sensor_portrait").unwrap();
    ui.set_orientation_override(Some(ScreenOrientation::Landscape));
    assert_eq!(ui.requested_orientation(), ScreenOrientation::Landscape);
    ui.set_orientation_override(None);
    assert_eq!(ui.requested_orientation(), ScreenOrientation::SensorPortrait);
}
