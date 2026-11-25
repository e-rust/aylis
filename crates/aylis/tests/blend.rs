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

/// Muitas marcas seguidas, cada uma sobre um fundo de outra cor (a grade de vídeos com marca
/// d'água): o Direct2D reaproveita os alvos e o fundo de uma camada para a outra, e cada célula
/// tem de sair com o negativo do próprio fundo, não o da célula anterior. Sem janela o
/// Direct2D desenha num bitmap da CPU, então isto confere a mistura e a ordem; a fila da placa
/// (o alvo reaproveitado só depois do `Flush`) só aparece na janela.
#[test]
#[cfg(all(windows, feature = "d2d"))]
fn many_blended_layers_in_a_row_keep_each_its_own_background() {
    let palette = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#00FFFF", "#FF00FF", "#000000", "#808080"];
    let (columns, rows, cell) = (8usize, 6usize, 40usize);
    let mut children = Vec::new();
    for i in 0..columns * rows {
        let (x, y) = ((i % columns) * cell, (i / columns) * cell);
        let fill = palette[(i * 5 + i / columns) % palette.len()];
        children.push(format!(
            r##"{{"class": "Layout", "name": "layCelula{i}", "left": {x}, "top": {y}, "width": {cell}, "height": {cell}, "clip_children": true, "children": [
                {{"class": "Rectangle", "name": "recFundo{i}", "align": "client", "fill": "{fill}", "stroke_width": 0, "radius": 0}},
                {{"class": "Layout", "name": "layMarca{i}", "align": "client", "blend_mode": "difference", "children": [
                    {{"class": "Rectangle", "name": "recMarca{i}", "left": 10, "top": 10, "width": 20, "height": 20, "fill": "#FFFFFF", "stroke_width": 0, "radius": 0}}
                ]}}
            ]}}"##
        ));
    }
    let form = format!(
        r##"{{"aylis": 1, "root": {{"class": "Form", "name": "frmGrade", "width": {}, "height": {}, "fill": "#808080", "children": [{}]}}}}"##,
        columns * cell,
        rows * cell,
        children.join(",")
    );
    let render = |renderer: Renderer| {
        let mut ui = aylis::core::Ui::new();
        aylis::register_all(&mut ui);
        let mut h = Headless::from_ui(ui, renderer);
        h.load_form(&form).expect("o form carrega");
        // Duas pinturas: a segunda já reaproveita o que a primeira guardou.
        let _ = h.render_rgba();
        h.render_rgba().expect("o renderizador fora da tela")
    };
    let (skia, w, _) = render(Renderer::Cpu);
    let (d2d, _, _) = render(Renderer::D2d);
    for i in 0..columns * rows {
        // O meio da marca e um canto fora dela, em cada célula.
        for (dx, dy) in [(20, 20), (4, 4)] {
            let (x, y) = ((i % columns) * cell + dx, (i / columns) * cell + dy);
            let at = (y * w as usize + x) * 4;
            let (s, d) = (&skia[at..at + 3], &d2d[at..at + 3]);
            assert!(s.iter().zip(d).all(|(a, b)| a.abs_diff(*b) <= 8), "célula {i} em ({x}, {y}): Skia {s:?} × Direct2D {d:?}");
        }
    }
}
