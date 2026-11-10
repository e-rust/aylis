//! Os modos de texto (item 10): no `layout` e no `core` o núcleo mede (as mesmas larguras e
//! quebras com qualquer renderizador); no `core` ele também desenha as letras, e a imagem sai
//! igual no Skia e no Direct2D. No `native`, cada renderizador mede e desenha (o de sempre).

use aylis_controls::{Form, Label};
use aylis_core::*;

/// Os textos que a fonte embutida (a Inter) tem por inteiro.
const TEXTS: &[&str] = &[
    "Ação de configuração: não há saída",
    "Pedido nº 1.234,56 — São João",
    "0123456789 R$ 1.000.000,00",
    "Um parágrafo comprido o bastante para quebrar em várias linhas na largura dada, com palavras de tamanhos diferentes e acentuação.",
];

fn register_font() -> bool {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/fonts/inter/Inter[opsz,wght].ttf");
    std::fs::read(path).map(|data| aylis_core::fonts::register(data).is_some()).unwrap_or(false)
}

fn style(wrap: bool) -> TextStyle {
    TextStyle { font: FontSpec { family: "Inter".into(), size: 14.0, ..Default::default() }, wrap, ..Default::default() }
}

/// O que um `Ui` mede para cada texto (numa linha e quebrando em 240).
fn measures(ui: &Ui) -> Vec<(SizeF, SizeF)> {
    TEXTS.iter().map(|t| (ui.measure_text(t, &style(false)), ui.measure_text_wrapped(t, &style(true), 240.0))).collect()
}

#[test]
fn layout_and_core_measure_with_the_engine_whatever_the_renderer() {
    if !register_font() {
        return eprintln!("sem resources/fonts (rode aylis-tool resources): teste pulado");
    }
    let mut ui = Ui::new();
    ui.services_mut().text = Some(Box::new(aylis_skia::SkiaRenderer::new().text_service()));
    ui.set_text_mode(TextMode::Layout);
    let skia = measures(&ui);
    // Sem nenhum serviço (como num teste) o núcleo mede igual: a medida é dele.
    ui.services_mut().text = None;
    assert_eq!(measures(&ui), skia);
    #[cfg(windows)]
    {
        ui.services_mut().text = Some(Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D").text_service()));
        assert_eq!(measures(&ui), skia, "o DirectWrite mede igual ao Skia no modo layout");
    }
    // O parágrafo quebra em várias linhas, e o núcleo diz quantas.
    let (one, wrapped) = skia[3];
    assert!(one.width > 240.0 && wrapped.height > one.height * 2.5, "{one:?} {wrapped:?}");
    // Um texto que a fonte não tem (emoji) fica com o renderizador.
    ui.services_mut().text = None;
    let emoji = ui.measure_text("👋", &style(false));
    assert_eq!(emoji, SizeF::new(14.0 * 0.55, 14.0 * 1.3), "a estimativa sem serviço");
}

/// Um form com os textos em rótulos (a Inter), pintado inteiro.
fn screen(mode: TextMode) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_text_mode(mode);
    let form = ui.add_root(Form::new("t", 640.0, 360.0));
    for text in TEXTS {
        let l = ui.add(Label::new(*text), form);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(60.0);
        let _ = ui.set_prop(l, "font_family", "Inter");
        let _ = ui.set_prop(l, "wrap", true);
    }
    ui.layout();
    (ui, form)
}

#[cfg(windows)]
#[test]
fn core_draws_the_same_letters_on_skia_and_direct2d() {
    if !register_font() {
        return eprintln!("sem resources/fonts: teste pulado");
    }
    let (w, h) = (640u32, 360u32);
    let rgb = |rgba: &[u8]| -> Vec<[u8; 3]> { rgba.chunks_exact(4).map(|p| [p[0], p[1], p[2]]).collect() };
    let differ = |a: &[[u8; 3]], b: &[[u8; 3]]| a.iter().zip(b).filter(|(x, y)| x.iter().zip(y.iter()).any(|(p, q)| p.abs_diff(*q) > 48)).count() as f64 / a.len() as f64;
    let mut shares = Vec::new();
    for mode in [TextMode::Native, TextMode::Core] {
        let skia = aylis_skia::SkiaRenderer::new();
        let (mut ui, form) = screen(mode);
        ui.services_mut().text = Some(Box::new(skia.text_service()));
        ui.layout();
        let png = skia.render_png(w, h, 1.0, Color::WHITE, |c| ui.paint(form, c)).unwrap();
        let a = rgb(&aylis_skia::decode_rgba(&png).unwrap().0);
        let d2d = aylis_d2d::D2dRenderer::new().expect("Direct2D");
        let (mut ui, form) = screen(mode);
        ui.services_mut().text = Some(Box::new(d2d.text_service()));
        ui.layout();
        let b = rgb(&d2d.render_area_rgba(w, h, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None)).unwrap());
        shares.push(differ(&a, &b));
    }
    eprintln!("pixels diferentes entre o Skia e o Direct2D: native {:.2}%, core {:.2}%", shares[0] * 100.0, shares[1] * 100.0);
    assert!(shares[1] < 0.003, "no core, o desenho das letras é do núcleo: {:.2}% diferentes", shares[1] * 100.0);
    assert!(shares[1] <= shares[0], "o core não pode diferir mais que o native");
}

#[test]
fn layout_and_core_paint_without_a_platform_text_service() {
    // Sem fonte registrada nem serviço, os modos caem no renderizador (nada quebra).
    for mode in [TextMode::Layout, TextMode::Core] {
        let skia = aylis_skia::SkiaRenderer::new();
        let (mut ui, form) = screen(mode);
        let png = skia.render_png(320, 180, 1.0, Color::WHITE, |c| ui.paint(form, c));
        assert!(png.is_some(), "{mode:?}");
    }
}

/// Grava as imagens dos três modos para conferir a olho (`AYLIS_TEXT_MODES=pasta`).
#[test]
fn save_the_modes_for_a_look() {
    let Some(dir) = std::env::var_os("AYLIS_TEXT_MODES") else { return };
    if !register_font() {
        return;
    }
    for mode in [TextMode::Native, TextMode::Layout, TextMode::Core] {
        let skia = aylis_skia::SkiaRenderer::new();
        let (mut ui, form) = screen(mode);
        ui.services_mut().text = Some(Box::new(skia.text_service()));
        ui.layout();
        let png = skia.render_png(640, 360, 1.0, Color::WHITE, |c| ui.paint(form, c)).unwrap();
        std::fs::write(std::path::Path::new(&dir).join(format!("{mode:?}.png")), png).unwrap();
    }
}

/// A fonte variável (a Inter) no peso pedido: o negrito mede mais que o normal.
#[test]
fn the_engine_follows_the_weight_of_a_variable_font() {
    if !register_font() {
        return;
    }
    let mut ui = Ui::new();
    ui.set_text_mode(TextMode::Core);
    let mut bold = style(false);
    bold.font.weight = FontWeight::Bold;
    let regular = ui.measure_text("Pedido confirmado", &style(false)).width;
    let heavy = ui.measure_text("Pedido confirmado", &bold).width;
    assert!(heavy > regular * 1.02, "negrito {heavy} × normal {regular}");
}
