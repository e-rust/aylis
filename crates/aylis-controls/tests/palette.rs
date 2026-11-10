//! Cores com nome: as do tema e as da paleta do projeto, em propriedades de cor
//! (`brand`, `text/85`), recalculadas ao trocar de tema e salvas pelo nome.

use aylis_controls::Form;
use aylis_core::palette::{Palette, PaletteColor};
use aylis_core::*;

fn fill(ui: &Ui, id: ComponentId) -> Color {
    ui.get::<aylis_controls::Rectangle>(id).and_then(|r| r.fill).unwrap_or(Color::TRANSPARENT)
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 300.0, 200.0));
    let r = ui.create("Rectangle", Some(form)).unwrap();
    (ui, r)
}

#[test]
fn theme_names_and_opacity_resolve_and_are_kept() {
    let (mut ui, r) = setup();
    ui.set_theme(Theme::light());
    let primary = ui.theme().color("primary");
    ui.set_prop(r, "fill", "primary").unwrap();
    assert_eq!(fill(&ui, r), primary);
    // O form salvo guarda o nome, não o hex.
    assert_eq!(ui.get_prop(r, "fill").unwrap(), PropValue::String("primary".into()));

    ui.set_prop(r, "fill", "primary/50").unwrap();
    assert_eq!(fill(&ui, r), primary.with_opacity(0.5));
    ui.set_prop(r, "fill", "white/12").unwrap();
    assert_eq!(fill(&ui, r), Color::WHITE.with_opacity(0.12));

    // Um hex desfaz o nome.
    ui.set_prop(r, "fill", "#112233").unwrap();
    assert_eq!(ui.get_prop(r, "fill").unwrap(), PropValue::Color(Color::rgb(0x11, 0x22, 0x33)));
    assert!(ui.color_name(r, "fill").is_none());

    assert!(ui.set_prop(r, "fill", "nao_existe").is_err());
}

#[test]
fn changing_theme_recolors_named_properties() {
    let (mut ui, r) = setup();
    ui.set_theme(Theme::light());
    ui.set_prop(r, "fill", "text").unwrap();
    ui.set_prop(r, "text_color", "primary").unwrap();
    let light = fill(&ui, r);
    ui.set_theme(Theme::dark());
    assert_eq!(fill(&ui, r), ui.theme().color("text"));
    assert_ne!(fill(&ui, r), light);
    assert_eq!(ui.get_prop(r, "text_color").unwrap(), PropValue::String("primary".into()));
}

#[test]
fn project_palette_by_mode_and_references() {
    let (mut ui, r) = setup();
    let palette: Palette = vec![
        ("brand_soft".into(), PaletteColor::same("brand/20")),
        ("brand".into(), PaletteColor::same("#10B981")),
        ("panel".into(), PaletteColor::by_mode("#0F0F0F", "#FFFFFF")),
    ];
    ui.set_palette(palette);
    ui.set_theme(Theme::dark());
    ui.set_prop(r, "fill", "panel").unwrap();
    assert_eq!(fill(&ui, r), Color::rgb(0x0F, 0x0F, 0x0F));
    // A paleta vale nos estilos (pelo tema) e pode citar uma entrada que vem depois.
    assert_eq!(ui.theme().color("brand_soft"), Color::rgb(0x10, 0xB9, 0x81).with_opacity(0.2));
    ui.set_theme(Theme::light());
    assert_eq!(fill(&ui, r), Color::WHITE);
}

