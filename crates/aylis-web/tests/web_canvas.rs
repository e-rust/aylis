//! O Canvas 2D no navegador: o recorte, a máscara e o padrão do SVG, o espaço entre letras, a
//! medida com quebra e os emoji sem cor, com as mesmas contas dos testes do Direct2D e do Skia.
//!
//! Rodar (abre um servidor; o resultado aparece na página):
//! `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner NO_HEADLESS=1 cargo test
//! -p aylis-web --target wasm32-unknown-unknown`, e abrir o endereço que ele mostra.
#![cfg(target_arch = "wasm32")]

use aylis_core::{Canvas, Color, RectF, TextAlign, TextStyle};
use aylis_web::Canvas2d;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

wasm_bindgen_test_configure!(run_in_browser);

static NOTO_EMOJI: &[u8] = include_bytes!("../../../resources/fonts/notoemoji/NotoEmoji[wght].ttf");

fn context(w: u32, h: u32) -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into().unwrap();
    canvas.set_width(w);
    canvas.set_height(h);
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
    ctx.set_fill_style_str("#fff");
    ctx.fill_rect(0.0, 0.0, w as f64, h as f64);
    ctx
}

fn pixels(ctx: &CanvasRenderingContext2d, w: u32, h: u32) -> Vec<u8> {
    ctx.get_image_data(0.0, 0.0, w as f64, h as f64).unwrap().data().0
}

fn draw(w: u32, h: u32, paint: impl FnOnce(&mut dyn Canvas)) -> Vec<u8> {
    let ctx = context(w, h);
    let mut canvas = Canvas2d::new(&ctx);
    paint(&mut canvas);
    drop(canvas);
    pixels(&ctx, w, h)
}

const EFFECTS: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <defs>
    <clipPath id="c"><circle cx="25" cy="25" r="20"/></clipPath>
    <clipPath id="two"><rect x="55" y="5" width="15" height="40"/><rect x="80" y="5" width="15" height="40"/></clipPath>
    <mask id="k"><rect x="0" y="50" width="50" height="50" fill="white"/><circle cx="25" cy="75" r="10" fill="black"/></mask>
    <mask id="a" mask-type="alpha"><rect x="50" y="50" width="25" height="50" fill="#000"/></mask>
    <pattern id="p" width="10" height="10" patternUnits="userSpaceOnUse"><rect width="5" height="10" fill="#0000ff"/></pattern>
  </defs>
  <rect width="50" height="50" fill="#ff0000" clip-path="url(#c)"/>
  <rect x="50" width="50" height="50" fill="#00a000" clip-path="url(#two)"/>
  <rect y="50" width="50" height="50" fill="url(#p)" mask="url(#k)"/>
  <rect x="50" y="50" width="50" height="50" fill="#ff00ff" mask="url(#a)"/>
</svg>"##;

/// Os mesmos pontos do teste do Skia e do Direct2D (`aylis-vector`, `svg_effects`).
#[wasm_bindgen_test]
fn clip_mask_and_pattern_like_skia() {
    let svg = aylis_vector::SvgImage::parse(EFFECTS).unwrap();
    let px = draw(200, 200, |c| svg.render(c, RectF::new(0.0, 0.0, 200.0, 200.0), Color::BLACK));
    let at = |x: f32, y: f32| {
        let k = ((y * 2.0) as usize * 200 + (x * 2.0) as usize) * 4;
        [px[k], px[k + 1], px[k + 2]]
    };
    let near = |a: [u8; 3], b: [u8; 3]| a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 12);
    let (white, red, green, blue, magenta) = ([255, 255, 255], [255, 0, 0], [0, 160, 0], [0, 0, 255], [255, 0, 255]);
    for ((x, y), color, what) in [
        ((25.0, 25.0), red, "dentro do círculo do recorte"),
        ((7.0, 7.0), white, "fora do círculo"),
        ((62.0, 25.0), green, "no primeiro retângulo do recorte"),
        ((75.0, 25.0), white, "entre os dois retângulos"),
        ((87.0, 25.0), green, "no segundo retângulo (a união)"),
        ((12.0, 57.0), blue, "listra do padrão"),
        ((17.0, 57.0), white, "entre as listras"),
        ((22.0, 75.0), white, "o furo preto da máscara de luminância"),
        ((60.0, 75.0), magenta, "máscara de alfa opaca"),
        ((90.0, 75.0), white, "fora da máscara de alfa"),
    ] {
        let got = at(x, y);
        assert!(near(got, color), "({x}, {y}) {what}: {got:?}, esperado {color:?}");
    }
    // A máscara cinza (`#808080`): metade.
    let gray = aylis_vector::SvgImage::parse(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><mask id="m"><rect width="10" height="10" fill="#808080"/></mask><rect width="10" height="10" fill="#000" mask="url(#m)"/></svg>"##,
    )
    .unwrap();
    let px = draw(20, 20, |c| gray.render(c, RectF::new(0.0, 0.0, 20.0, 20.0), Color::BLACK));
    assert!(px[440].abs_diff(127) <= 4, "máscara cinza = metade: {}", px[440]);
}

fn style(spacing: f32) -> TextStyle {
    let mut s = TextStyle::default();
    s.font.size = 20.0;
    s.letter_spacing = spacing;
    s.color = Color::BLACK;
    s
}

/// A primeira e a última coluna com tinta.
fn ink_columns(px: &[u8], w: usize) -> (usize, usize) {
    let dark = |x: usize| px.chunks(4).skip(x).step_by(w).any(|p| p[0] < 100);
    ((0..w).find(|&x| dark(x)).unwrap_or(0), (0..w).rev().find(|&x| dark(x)).unwrap_or(0))
}

/// O `letter_spacing`: `espaço × (letras - 1)` na medida e no desenho, também centrado.
#[wasm_bindgen_test]
fn letter_spacing_like_the_other_renderers() {
    let ctx = context(4, 4);
    let mut c = Canvas2d::new(&ctx);
    let (w0, w4) = (c.measure_text("HELLO", &style(0.0)).width, c.measure_text("HELLO", &style(4.0)).width);
    assert!((w4 - w0 - 16.0).abs() < 0.6, "{w0} → {w4}");
    drop(c);
    for align in [TextAlign::Leading, TextAlign::Center, TextAlign::Trailing] {
        let paint = |spacing: f32| {
            move |c: &mut dyn Canvas| {
                let mut s = style(spacing);
                s.h_align = align;
                c.draw_text("HELLO", RectF::new(10.0, 0.0, 220.0, 80.0), &s);
            }
        };
        let (a, b) = (ink_columns(&draw(240, 80, paint(0.0)), 240), ink_columns(&draw(240, 80, paint(6.0)), 240));
        let grown = (b.1 - b.0) as f32 - (a.1 - a.0) as f32;
        assert!((grown - 24.0).abs() <= 2.0, "{align:?}: {a:?} → {b:?}");
        match align {
            TextAlign::Leading => assert!(a.0.abs_diff(b.0) <= 1, "à esquerda, a primeira letra no lugar: {a:?} → {b:?}"),
            TextAlign::Center => assert!(((a.0 + a.1) as f32 / 2.0 - (b.0 + b.1) as f32 / 2.0).abs() <= 1.5, "no meio: {a:?} → {b:?}"),
            _ => assert!(a.1.abs_diff(b.1) <= 1, "à direita, a última letra no lugar: {a:?} → {b:?}"),
        }
    }
}

/// O `measure_text_wrapped` dá o tamanho que o `draw_text` com quebra ocupa.
#[wasm_bindgen_test]
fn measure_text_wrapped_is_the_size_draw_text_takes() {
    const LONG: &str = "Um texto comprido que quebra em várias linhas, medido antes de desenhar";
    for spacing in [0.0, 1.5] {
        let mut s = style(spacing);
        s.wrap = true;
        s.v_align = TextAlign::Leading;
        let (mut size, mut line) = (Default::default(), 0.0);
        let px = draw(240, 200, |c| {
            size = c.measure_text_wrapped(LONG, &s, 150.0);
            assert_eq!(c.measure_text_wrapped(LONG, &s, 150.0), size, "o guardado é o mesmo");
            line = c.measure_text("Ag", &s).height;
            c.draw_text(LONG, RectF::new(0.0, 0.0, 150.0, size.height), &s);
        });
        let lines = (size.height / line).round();
        assert!(lines >= 3.0, "{size:?} em linhas de {line}");
        let (_, right) = ink_columns(&px, 240);
        assert!(right as f32 <= size.width + 1.0, "a tinta vai até {right}, medido {}", size.width);
        let bottom = px.chunks(4 * 240).rposition(|row| row.chunks(4).any(|p| p[0] < 100)).unwrap_or(0) as f32;
        assert!(bottom < size.height && bottom > size.height - line, "a tinta acaba em {bottom}, medido {}", size.height);
    }
}

/// Pixels com cor de verdade.
fn colored(px: &[u8]) -> usize {
    px.chunks(4)
        .filter(|p| {
            let (r, g, b) = (p[0] as i32, p[1] as i32, p[2] as i32);
            (r - g).abs() + (g - b).abs() + (r - b).abs() > 60
        })
        .count()
}

/// `color_glyphs: false`: os emoji na cor do texto, pela Noto Emoji (registrada pelo app, só
/// nos trechos de emoji); com a cor (o padrão), coloridos.
#[wasm_bindgen_test]
async fn emoji_without_color() {
    assert!(aylis_core::fonts::register(NOTO_EMOJI.to_vec()).is_some());
    let mut mono = style(0.0);
    mono.font.size = 32.0;
    mono.color_glyphs = false;
    // A fonte entra no documento no primeiro desenho e carrega depois.
    let _ = draw(4, 4, |c| c.draw_text("\u{1F600}", RectF::new(0.0, 0.0, 4.0, 4.0), &mono));
    let fonts = js_sys::Reflect::get(&web_sys::window().unwrap().document().unwrap(), &"fonts".into()).unwrap();
    let ready: js_sys::Promise = js_sys::Reflect::get(&fonts, &"ready".into()).unwrap().dyn_into().unwrap();
    wasm_bindgen_futures::JsFuture::from(ready).await.unwrap();
    for text in ["\u{1F600}", "\u{2705} ok", "A \u{1F44D}\u{1F3FD} 1", "\u{2764}\u{FE0F}"] {
        let px = draw(240, 60, |c| c.draw_text(text, RectF::new(4.0, 0.0, 232.0, 60.0), &mono));
        assert_eq!(colored(&px), 0, "{text:?} sem cor");
        assert!(px.chunks(4).filter(|p| p[0] < 128).count() > 40, "{text:?} aparece");
    }
    let mut color = mono.clone();
    color.color_glyphs = true;
    let px = draw(240, 60, |c| c.draw_text("\u{1F600}", RectF::new(4.0, 0.0, 232.0, 60.0), &color));
    assert!(colored(&px) > 100, "o emoji colorido");
}

/// Item 23: o texto do `PaintCanvas` vira pixel também no navegador, pelo desenho fora da tela
/// do Canvas 2D (`WebOffscreen`: um `<canvas>` solto e o `getImageData`).
#[wasm_bindgen_test]
fn the_paint_canvas_text_turns_into_pixels() {
    use aylis_core::{PropValue, Ui};
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(aylis_web::WebTextService::new().unwrap()));
    ui.services_mut().offscreen = Some(Box::new(aylis_web::WebOffscreen));
    let f = ui.add_root(aylis_controls::Form::new("teste", 400.0, 300.0));
    let p = ui.create("PaintCanvas", Some(f)).unwrap();
    ui.invoke(p, "new", &[PropValue::Int(200), PropValue::Int(120)]).unwrap();
    ui.set_prop(p, "color", Color::rgb(0, 0, 255)).unwrap();
    let before = ui.invoke(p, "checksum", &[]).unwrap();
    assert_eq!(ui.invoke(p, "draw_text", &[PropValue::Int(10), PropValue::Int(10), PropValue::String("Câmera 3".into())]).unwrap(), PropValue::Bool(true));
    assert_ne!(ui.invoke(p, "checksum", &[]).unwrap(), before);
    let bitmap = ui.get::<aylis_controls::PaintCanvas>(p).unwrap().bitmap().clone();
    let blue = (8..40).flat_map(|y| (8..120).map(move |x| (x, y))).filter(|&(x, y)| bitmap.get(x, y).is_some_and(|c| c.b > 200 && c.r < 80)).count();
    assert!(blue > 20, "pixels do texto na imagem: {blue}");
    // O `render_png` também: um PNG de verdade.
    let png = aylis_core::platform::OffscreenService::render_png(&aylis_web::WebOffscreen, 10, 10, 1.0, Color::WHITE, &mut |_| {}).unwrap();
    assert_eq!(&png[1..4], b"PNG");
}
