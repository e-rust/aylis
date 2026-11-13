//! O modo de mistura (`blend_mode`): o controle se junta ao que está por baixo, igual no Skia
//! e no Direct2D. O caso que motivou: a marca d'água branca em `difference` sobre um vídeo
//! sai em negativo (o vermelho vira ciano, o preto vira branco).

use aylis::headless::Headless;
use aylis::platform::Renderer;

/// Um form 200x100: à esquerda vermelho, à direita preto (o "vídeo"), e por cima, no meio,
/// um retângulo branco com o modo `mode` (a marca d'água).
const FORM: &str = r##"{"aylis": 1, "root": {"class": "Form", "name": "frmMarca", "width": 200, "height": 100, "fill": "#808080", "children": [
    {"class": "Rectangle", "name": "recVermelho", "left": 0, "top": 0, "width": 100, "height": 100, "fill": "#FF0000", "stroke_width": 0, "radius": 0},
    {"class": "Rectangle", "name": "recPreto", "left": 100, "top": 0, "width": 100, "height": 100, "fill": "#000000", "stroke_width": 0, "radius": 0},
    {"class": "Rectangle", "name": "recMarca", "left": 50, "top": 25, "width": 100, "height": 50, "fill": "#FFFFFF", "stroke_width": 0, "radius": 0, "blend_mode": "MODE", "opacity": OPACITY}
]}}"##;

/// A cor em (x, y) com o modo `mode` no renderizador `renderer`.
fn colors(renderer: Renderer, mode: &str, opacity: f32, points: &[(u32, u32)]) -> Vec<[u8; 3]> {
    colors_with(renderer, mode, opacity, "#FFFFFF", points)
}

/// Como [`colors`], com a marca na cor `mark`.
fn colors_with(renderer: Renderer, mode: &str, opacity: f32, mark: &str, points: &[(u32, u32)]) -> Vec<[u8; 3]> {
    let mut ui = aylis::core::Ui::new();
    aylis::register_all(&mut ui);
    let mut h = Headless::from_ui(ui, renderer);
    h.load_form(&FORM.replace("MODE", mode).replace("OPACITY", &opacity.to_string()).replace("\"fill\": \"#FFFFFF\"", &format!("\"fill\": \"{mark}\""))).expect("o form carrega");
    let (pixels, w, _) = h.render_rgba().expect("o renderizador fora da tela");
    points
        .iter()
        .map(|&(x, y)| {
            let i = ((y * w + x) * 4) as usize;
            [pixels[i], pixels[i + 1], pixels[i + 2]]
        })
        .collect()
}

fn near(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 3)
}

/// Os pontos: a marca sobre o vermelho, a marca sobre o preto, o vermelho fora da marca.
const POINTS: [(u32, u32); 3] = [(75, 50), (125, 50), (20, 10)];

fn check(renderer: Renderer) {
    // `difference` com branco: o negativo (vermelho → ciano, preto → branco); fora, nada muda.
    let got = colors(renderer, "difference", 1.0, &POINTS);
    assert!(near(got[0], [0, 255, 255]), "{renderer}: sobre o vermelho {:?}", got[0]);
    assert!(near(got[1], [255, 255, 255]), "{renderer}: sobre o preto {:?}", got[1]);
    assert!(near(got[2], [255, 0, 0]), "{renderer}: fora da marca {:?}", got[2]);
    // `multiply` com branco não muda o fundo; `normal` cobre.
    let got = colors(renderer, "multiply", 1.0, &POINTS);
    assert!(near(got[0], [255, 0, 0]) && near(got[1], [0, 0, 0]), "{renderer}: multiply {got:?}");
    let got = colors(renderer, "normal", 1.0, &POINTS);
    assert!(near(got[0], [255, 255, 255]) && near(got[1], [255, 255, 255]), "{renderer}: normal {got:?}");
    // A opacidade vale junto: metade da diferença.
    let got = colors(renderer, "difference", 0.5, &POINTS);
    assert!(near(got[1], [128, 128, 128]), "{renderer}: difference a 50% sobre o preto {:?}", got[1]);
}

#[test]
fn the_blend_mode_mixes_with_what_is_below_in_skia() {
    check(Renderer::Cpu);
}

#[test]
#[cfg(all(windows, feature = "d2d"))]
fn the_blend_mode_mixes_with_what_is_below_in_direct2d() {
    check(Renderer::D2d);
}

/// Os dois renderizadores dão a mesma cor em todos os modos (o desenho igual nos três).
#[test]
#[cfg(all(windows, feature = "d2d"))]
fn skia_and_direct2d_agree_on_every_mode() {
    use aylis::core::PropEnum;
    // Com branco e com uma cor do meio (com branco, vários modos dão o mesmo).
    for mark in ["#FFFFFF", "#4080C0"] {
        for mode in aylis::core::BlendMode::NAMES {
            let skia = colors_with(Renderer::Cpu, mode, 1.0, mark, &POINTS);
            let d2d = colors_with(Renderer::D2d, mode, 1.0, mark, &POINTS);
            for (s, d) in skia.iter().zip(&d2d) {
                assert!(s.iter().zip(d).all(|(a, b)| a.abs_diff(*b) <= 8), "{mode} com {mark}: Skia {skia:?} × Direct2D {d2d:?}");
            }
        }
    }
}

/// Repintar só uma parte (o vídeo trocando de quadro embaixo) dá o mesmo que a pintura
/// inteira, com a marca misturada no lugar.
#[test]
fn a_blended_watermark_passes_the_partial_paint_check() {
    let mut ui = aylis::core::Ui::new();
    aylis::register_all(&mut ui);
    let mut h = Headless::from_ui(ui, Renderer::Cpu);
    h.load_form(&FORM.replace("MODE", "difference").replace("OPACITY", "1")).unwrap();
    h.check_partial_paint("recVermelho", |ui, id| {
        let _ = ui.set_prop(id, "fill", "#00FF00");
    })
    .expect("a pintura por parte com a marca por cima");
}
