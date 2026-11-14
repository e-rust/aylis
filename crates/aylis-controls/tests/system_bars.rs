//! O `SystemBars` (decidido em 14/11): o pedido das barras do sistema chega ao `Ui` depois de o
//! form carregar, com as cores, os ícones e o que aparece; onde a barra de status não tem cor
//! própria (o iOS), o núcleo pinta a faixa dela por cima do form principal.

use aylis_core::*;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", 360.0).unwrap();
    ui.set_prop(form, "height", 640.0).unwrap();
    ui.show(form).unwrap();
    let bars = ui.create("SystemBars", Some(form)).unwrap();
    (ui, form, bars)
}

#[test]
fn the_request_reaches_the_ui_after_the_form_loads() {
    let (mut ui, _, bars) = setup();
    assert!(ui.system_bars().is_none(), "sem propriedade mudada, nada pedido");
    ui.set_prop(bars, "status_bar_color", "#123456").unwrap();
    ui.set_prop(bars, "icons", "light").unwrap();
    ui.set_prop(bars, "navigation_bar_visible", false).unwrap();
    // Ainda não: vai inteiro, depois das propriedades.
    assert!(ui.system_bars().is_none());
    ui.advance(0.01);
    let style = ui.system_bars().cloned().expect("pedido");
    assert_eq!(style.status_color, Some(Color::rgba(0x12, 0x34, 0x56, 255)));
    assert_eq!(style.navigation_color, None, "a de navegação segue a de status");
    assert_eq!(style.icons, BarIcons::Light);
    assert!(style.status_visible && !style.navigation_visible);
    // Auto: os ícones pela claridade da barra.
    let auto = SystemBarsStyle { icons: BarIcons::Auto, ..style.clone() };
    assert!(!auto.dark_icons(Color::rgba(0x12, 0x34, 0x56, 255)) && auto.dark_icons(Color::rgba(250, 250, 250, 255)));
    let version = ui.system_bars_version();
    ui.set_prop(bars, "status_bar_visible", false).unwrap();
    ui.advance(0.01);
    assert!(ui.system_bars_version() > version && !ui.system_bars().unwrap().status_visible);
}

#[test]
fn the_status_strip_is_painted_only_where_the_platform_asks() {
    let (mut ui, form, bars) = setup();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.set_prop(form, "fill", "#FFFFFF").unwrap();
    ui.set_prop(bars, "status_bar_color", "#FF0000").unwrap();
    ui.advance(0.01);
    ui.set_safe_area(form, Bounds::new(0.0, 40.0, 0.0, 0.0));
    ui.layout();
    let pixel = |ui: &mut Ui, x: usize, y: usize| {
        let mut pixels = vec![0u32; 360 * 640];
        renderer.render_area_to_pixels(&mut pixels, 360, 640, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
        pixels[y * 360 + x] & 0x00FF_FFFF
    };
    assert_eq!(pixel(&mut ui, 180, 20), 0x00FF_FFFF, "sem a plataforma pedir, nada de faixa");
    ui.set_paints_status_strip(true);
    assert_eq!(pixel(&mut ui, 180, 20), 0x00FF_0000, "a faixa vermelha na área de cima");
    assert_eq!(pixel(&mut ui, 180, 60), 0x00FF_FFFF, "abaixo da área segura, o form");
}
