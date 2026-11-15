//! `Ui::render_rgba`: um retângulo do form em pixels RGBA, sem PNG (o editor de print do
//! NoOBS, item 4). O recorte sai igual ao mesmo pedaço da imagem do form inteiro, com a imagem
//! de fundo, o desenho (`on_paint`) e um Memo por cima: pixel por pixel no Direct2D; no Skia, a
//! mesma coisa a menos da beirada de uma curva que o retângulo corta. E a decisão de trazer a
//! janela para a frente pelo caminho forçado (`force_foreground`, item 6), que não precisa de
//! janela.

use aylis_controls::{Form, ImageView};
use aylis_core::platform::OffscreenService;
use aylis_core::*;

const W: u32 = 640;
const H: u32 = 400;

/// Quantos tons a beirada anti-aliasing de uma curva cortada pelo retângulo pode mudar no Skia.
/// Ele refaz a curva que passa da borda da imagem (troca o pedaço de fora por retas e
/// subdivide o resto de outro jeito): uma faixa de 1 pixel que corta uma elipse na vertical
/// mudava 5 tons, mesmo com os 16 pixels de moldura do `render_rgba` (sem a moldura, 9
/// pixels mudavam mais). É o mesmo efeito da pintura por parte da janela no Skia (o
/// `partial_paint` aceita 48); o resto (a imagem, o texto, os retângulos) sai igual.
const SKIA_CURVE_TONES: u8 = 8;

/// O desenho fora da tela do Skia (o mesmo que a plataforma põe: `SkiaOffscreen`).
struct SkiaOffscreen;

impl OffscreenService for SkiaOffscreen {
    fn render_png(&self, _: u32, _: u32, _: f32, _: Color, _: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        None
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        aylis_skia::SkiaRenderer::new().render_rgba(width, height, scale, background, |c| paint(c))
    }
}

/// O do Direct2D (o `D2dOffscreen` da plataforma).
#[cfg(windows)]
struct D2dOffscreen;

#[cfg(windows)]
impl OffscreenService for D2dOffscreen {
    fn render_png(&self, _: u32, _: u32, _: f32, _: Color, _: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        None
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        aylis_d2d::D2dRenderer::new().ok()?.render_rgba(width, height, scale, background, |c| paint(c))
    }
}

/// O form do print: a imagem de fundo (um degradê com detalhe em cada pixel), as anotações
/// (`on_paint`: um retângulo de seleção, um traço e uma elipse) e um Memo.
fn scene(text: Box<dyn TextService>, offscreen: Box<dyn OffscreenService>) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(text);
    ui.services_mut().offscreen = Some(offscreen);
    let form = ui.add_root(Form::new("print", W as f32, H as f32));
    ui.show(form).unwrap();
    let background = ui.add(ImageView::default(), form);
    ui.set_prop(background, "align", "client").unwrap();
    let pixels: Vec<u8> = (0..W * H).flat_map(|i| {
        let (x, y) = (i % W, i / W);
        [(x * 255 / W) as u8, (y * 255 / H) as u8, ((x ^ y) & 0xFF) as u8, 255]
    }).collect();
    ui.get_mut::<ImageView>(background).unwrap().set_picture(Some(Image::rgba(W, H, pixels)));
    let notes = ui.create("Panel", Some(form)).unwrap();
    ui.set_prop(notes, "align", "client").unwrap();
    ui.on_paint(notes, |canvas, _, _| {
        let red = Color::rgb(230, 30, 30);
        canvas.stroke_rect(RectF::new(60.5, 40.5, 300.0, 180.0), 0.0, &Stroke::new(red, 3.0));
        let mut path = Path::new();
        path.move_to(80.0, 300.0).cubic_to(150.0, 200.0, 260.0, 390.0, 420.0, 260.0);
        canvas.stroke_path(&path, &Stroke::new(red, 6.0).round());
        canvas.fill_ellipse(RectF::new(470.0, 60.0, 120.0, 80.0), &Brush::Solid(Color::rgba(20, 120, 230, 160)));
    });
    let memo = ui.create("Memo", Some(notes)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 330.0.into()), ("top", 230.0.into()), ("width", 260.0.into()), ("height", 140.0.into()), ("text", "Uma anotação\ncom duas linhas e acentuação".into())] {
        ui.set_prop(memo, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    (ui, form)
}

/// O pedaço `rect` (em pixels) de uma imagem RGBA de largura `width`.
fn crop(full: &[u8], width: u32, rect: (u32, u32, u32, u32)) -> Vec<u8> {
    let (x, y, w, h) = rect;
    (y..y + h).flat_map(|row| {
        let start = ((row * width + x) * 4) as usize;
        full[start..start + (w * 4) as usize].iter().copied()
    }).collect()
}

fn compare(text: Box<dyn TextService>, offscreen: Box<dyn OffscreenService>, name: &str, tolerance: u8) {
    let (mut ui, form) = scene(text, offscreen);
    for scale in [1.0f32, 2.0] {
        let (fw, fh, full) = ui.render_rgba(form, RectF::new(0.0, 0.0, W as f32, H as f32), scale).unwrap_or_else(|| panic!("{name}: o form inteiro"));
        assert_eq!((fw, fh), ((W as f32 * scale) as u32, (H as f32 * scale) as u32));
        // A seleção, o Memo (cortado ao meio), um canto e faixas finas que cortam as curvas (a
        // elipse e o traço): o corte não muda a beirada delas.
        for rect in [(60, 40, 302, 182), (400, 250, 150, 100), (0, 0, 17, 9), (0, 120, W, 1), (0, 70, W, 3), (530, 0, 1, H), (200, 230, 7, 90)] {
            let logical = RectF::new(rect.0 as f32, rect.1 as f32, rect.2 as f32, rect.3 as f32);
            let (w, h, part) = ui.render_rgba(form, logical, scale).unwrap_or_else(|| panic!("{name}: o recorte {rect:?}"));
            let s = |v: u32| (v as f32 * scale) as u32;
            assert_eq!((w, h), (s(rect.2), s(rect.3)), "{name}: o tamanho do recorte {rect:?} × {scale}");
            let expected = crop(&full, fw, (s(rect.0), s(rect.1), w, h));
            let diff = part.iter().zip(&expected).filter(|(a, b)| a.abs_diff(**b) > tolerance).count();
            let tones = part.iter().zip(&expected).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
            assert_eq!(diff, 0, "{name}: o recorte {rect:?} × {scale} saiu diferente do mesmo pedaço do form inteiro ({diff} bytes; até {tones} tons, limite {tolerance})");
        }
    }
    // Pelo nome (o protocolo): os mesmos pixels, em base64.
    let (_, _, part) = ui.render_rgba(form, RectF::new(10.0, 20.0, 30.0, 40.0), 1.0).unwrap();
    let args: Vec<PropValue> = [10.0, 20.0, 30.0, 40.0].into_iter().map(PropValue::Float).collect();
    assert_eq!(ui.invoke(form, "render_rgba", &args), Ok(PropValue::String(image::encode_base64(&part))), "{name}: pelo invoke");
    assert!(ui.invoke(form, "render_rgba", &[PropValue::Float(1.0)]).is_err(), "{name}: sem os argumentos");
    // O retângulo vazio não desenha.
    assert!(ui.render_rgba(form, RectF::new(5.0, 5.0, 0.0, 10.0), 1.0).is_none());
}

#[test]
fn a_rectangle_of_the_form_comes_out_like_the_same_piece_of_the_whole_in_skia() {
    let renderer = aylis_skia::SkiaRenderer::new();
    compare(Box::new(renderer.text_service()), Box::new(SkiaOffscreen), "Skia", SKIA_CURVE_TONES);
}

#[cfg(windows)]
#[test]
fn a_rectangle_of_the_form_comes_out_like_the_same_piece_of_the_whole_in_direct2d() {
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    compare(Box::new(renderer.text_service()), Box::new(D2dOffscreen), "Direct2D", 0);
}

#[test]
fn base64_matches_the_standard_alphabet() {
    assert_eq!(image::encode_base64(b""), "");
    assert_eq!(image::encode_base64(b"f"), "Zg==");
    assert_eq!(image::encode_base64(b"fo"), "Zm8=");
    assert_eq!(image::encode_base64(b"foobar"), "Zm9vYmFy");
    assert_eq!(image::encode_base64(&[255, 0, 128, 7]), "/wCABw==");
}

/// Item 6: só o form com `force_foreground` (e nunca no designer) vem para a frente pelo
/// caminho forçado; um controle de dentro responde pelo form dele.
#[test]
fn only_a_form_with_force_foreground_takes_the_forced_path() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("print", 300.0, 200.0));
    let button = ui.create("Button", Some(form)).unwrap();
    assert!(!ui.forces_foreground(form), "desligado por padrão");
    ui.set_prop(form, "force_foreground", true).unwrap();
    assert!(ui.forces_foreground(form));
    assert!(ui.forces_foreground(button), "o controle responde pelo form");
    ui.set_design_mode(true);
    assert!(!ui.forces_foreground(form), "no designer, nunca");
    ui.set_design_mode(false);
    ui.set_prop(form, "force_foreground", false).unwrap();
    assert!(!ui.forces_foreground(form));
}
