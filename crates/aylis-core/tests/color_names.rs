//! A cor escrita pelo nome (`accent`) acompanha o tema, vai junto na cópia e deixa de
//! valer quando a cor é trocada direto no componente.

mod common;

use common::boxy;
use aylis_core::*;

fn accent(theme: &Theme) -> PropValue {
    PropValue::Color(theme.resolve_color("accent").unwrap())
}

#[test]
fn a_named_color_follows_the_theme_and_the_copy() {
    let mut ui = Ui::new();
    ui.register_class::<common::Boxy>("Test");
    ui.set_theme(Theme::light());
    let form = ui.add_root(boxy(400.0, 300.0));
    let panel = ui.add(boxy(10.0, 10.0), form);
    ui.set_prop(panel, "fill", "accent").unwrap();
    assert_eq!(ui.color_name(panel, "fill"), Some("accent"));

    // A cópia (o colar do designer e o modelo da VirtualList usam o clone) leva o nome.
    let copy = ui.clone_component(panel, Some(form)).unwrap();
    assert_eq!(ui.color_name(copy, "fill"), Some("accent"));

    ui.set_theme(Theme::dark());
    let dark = accent(&Theme::dark());
    let raw = |ui: &Ui, id| component::read_prop(ui.component(id).unwrap(), "fill").unwrap();
    assert_eq!(raw(&ui, panel), dark, "o original troca com o tema");
    assert_eq!(raw(&ui, copy), dark, "a cópia também");
}

#[test]
fn a_color_changed_directly_stays_after_a_theme_change() {
    let mut ui = Ui::new();
    ui.set_theme(Theme::light());
    let form = ui.add_root(boxy(400.0, 300.0));
    let panel = ui.add(boxy(10.0, 10.0), form);
    ui.set_prop(panel, "fill", "accent").unwrap();

    // Direto no componente, sem passar pelo `Ui` (antes, a troca de tema a desfazia).
    let red = Color::rgb(200, 0, 0);
    ui.control_mut(panel).unwrap().set_prop("fill", PropValue::Color(red)).unwrap();
    assert_eq!(ui.color_name(panel, "fill"), None);
    ui.set_theme(Theme::dark());
    assert_eq!(ui.get_prop(panel, "fill"), Ok(PropValue::Color(red)));
}

#[test]
fn mixing_from_transparent_does_not_go_through_black() {
    // O hover de um botão transparente para um cinza-claro: no meio, o cinza-claro meio
    // opaco, nunca um cinza-escuro.
    let light = Color::rgba(244, 244, 245, 255);
    let half = Color::rgba(0, 0, 0, 0).lerp(light, 0.5);
    assert_eq!((half.r, half.g, half.b), (244, 244, 245));
    assert!((126..=129).contains(&half.a), "{half:?}");
    // Entre cores opacas, a mistura de sempre.
    assert_eq!(Color::rgba(0, 0, 0, 255).lerp(Color::rgba(200, 100, 50, 255), 0.5), Color::rgba(100, 50, 25, 255));
    assert_eq!(Color::rgba(0, 0, 0, 0).lerp(Color::rgba(0, 0, 0, 0), 0.5), Color::rgba(0, 0, 0, 0));
}
