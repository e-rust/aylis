//! Recorte, máscara e padrão do SVG: o leitor do Aylis desenha com as formas, os recortes e as
//! máscaras do `Canvas`, e o resultado é o mesmo no Skia e no Direct2D (o Canvas 2D usa os
//! mesmos passos; a conferência no navegador está no roteiro).

use aylis_core::{Canvas, Color, PointF, RectF, SizeF, TextStyle};
use aylis_vector::SvgImage;

/// Quatro quadrantes: recorte por um círculo; recorte por dois retângulos (a união); padrão de
/// listras com uma máscara de luminância (o furo preto no meio); máscara de alfa.
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

/// Uma máscara cinza (`#808080`, brilho ~0,5) sobre preto: metade.
const GRAY_MASK: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
  <mask id="m"><rect width="10" height="10" fill="#808080"/></mask>
  <rect width="10" height="10" fill="#000" mask="url(#m)"/>
</svg>"##;

const SIZE: u32 = 200;

fn skia(svg: &SvgImage, size: u32) -> Vec<u8> {
    let r = aylis_skia::SkiaRenderer::new();
    r.render_rgba(size, size, 1.0, Color::WHITE, |c| svg.render(c, RectF::new(0.0, 0.0, size as f32, size as f32), Color::BLACK)).unwrap()
}

#[cfg(windows)]
fn d2d(svg: &SvgImage, size: u32) -> Vec<u8> {
    let r = aylis_d2d::D2dRenderer::new().unwrap();
    r.render_rgba(size, size, 1.0, Color::WHITE, |c| svg.render(c, RectF::new(0.0, 0.0, size as f32, size as f32), Color::BLACK)).unwrap()
}

/// O pixel no ponto (x, y) do `viewBox` 0..100.
fn at(px: &[u8], x: f32, y: f32) -> [u8; 3] {
    let (i, j) = ((x * SIZE as f32 / 100.0) as usize, (y * SIZE as f32 / 100.0) as usize);
    let k = (j * SIZE as usize + i) * 4;
    [px[k], px[k + 1], px[k + 2]]
}

fn near(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 12)
}

/// Os pontos e as cores esperadas de [`EFFECTS`].
fn check(name: &str, px: &[u8]) {
    let (white, red, green, blue, magenta) = ([255, 255, 255], [255, 0, 0], [0, 160, 0], [0, 0, 255], [255, 0, 255]);
    let expected = [
        ((25.0, 25.0), red, "dentro do círculo do recorte"),
        ((7.0, 7.0), white, "fora do círculo"),
        ((62.0, 25.0), green, "no primeiro retângulo do recorte"),
        ((75.0, 25.0), white, "entre os dois retângulos"),
        ((87.0, 25.0), green, "no segundo retângulo (a união)"),
        ((12.0, 57.0), blue, "listra do padrão"),
        ((17.0, 57.0), white, "entre as listras"),
        ((2.0, 92.0), blue, "listra do padrão, embaixo"),
        ((22.0, 75.0), white, "o furo preto da máscara de luminância"),
        ((60.0, 75.0), magenta, "máscara de alfa opaca"),
        ((90.0, 75.0), white, "fora da máscara de alfa"),
    ];
    for ((x, y), color, what) in expected {
        let got = at(px, x, y);
        assert!(near(got, color), "{name}: ({x}, {y}) {what}: {got:?}, esperado {color:?}");
    }
}

#[test]
fn clip_mask_and_pattern_in_skia() {
    let svg = SvgImage::parse(EFFECTS).unwrap();
    assert!(svg.has_effects());
    let px = skia(&svg, SIZE);
    check("Skia", &px);
    if let Ok(dir) = std::env::var("AYLIS_SHOTS") {
        std::fs::write(std::path::Path::new(&dir).join("svg-efeitos-skia.png"), aylis_skia::encode_png(&px, SIZE, SIZE).unwrap()).unwrap();
    }
    let gray = skia(&SvgImage::parse(GRAY_MASK).unwrap(), 20);
    assert!(gray[40 * 10 + 40].abs_diff(127) <= 4, "máscara cinza = metade: {}", gray[40 * 10 + 40]);
}

#[cfg(windows)]
#[test]
fn clip_mask_and_pattern_in_direct2d_like_skia() {
    let svg = SvgImage::parse(EFFECTS).unwrap();
    let (a, b) = (d2d(&svg, SIZE), skia(&svg, SIZE));
    check("Direct2D", &a);
    if let Ok(dir) = std::env::var("AYLIS_SHOTS") {
        std::fs::write(std::path::Path::new(&dir).join("svg-efeitos-d2d.png"), aylis_skia::encode_png(&a, SIZE, SIZE).unwrap()).unwrap();
    }
    // Fora as bordas (o antisserrilhado de cada um), os mesmos pixels.
    let differ = a.chunks(4).zip(b.chunks(4)).filter(|(p, q)| p.iter().zip(q.iter()).any(|(x, y)| x.abs_diff(*y) > 48)).count();
    assert!(differ < (SIZE * SIZE / 100) as usize, "Direct2D × Skia: {differ} pixels diferentes");
    let gray = d2d(&SvgImage::parse(GRAY_MASK).unwrap(), 20);
    assert!(gray[40 * 10 + 40].abs_diff(127) <= 4, "máscara cinza = metade: {}", gray[40 * 10 + 40]);
}

/// Um canvas que conta as formas e guarda as chaves do `begin_cached`.
#[derive(Default)]
struct Counting {
    shapes: usize,
    cached: Vec<u64>,
}

impl Canvas for Counting {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &aylis_core::Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &aylis_core::Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &aylis_core::Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &aylis_core::Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &aylis_core::Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, _: &str, _: &TextStyle) -> SizeF {
        SizeF::ZERO
    }
    fn fill_path(&mut self, _: &aylis_core::Path, _: &aylis_core::Brush) {
        self.shapes += 1;
    }
    fn begin_cached(&mut self, key: u64, _: RectF) -> bool {
        if self.cached.contains(&key) {
            return true;
        }
        self.cached.push(key);
        false
    }
}

/// O desenho com máscara é guardado enquanto o SVG, o tamanho e as cores não mudam (a segunda
/// pintura não desenha nada); um SVG sem efeitos não passa pela imagem guardada.
#[test]
fn an_svg_with_effects_is_drawn_once_while_it_does_not_change() {
    let svg = SvgImage::parse(EFFECTS).unwrap();
    let mut c = Counting::default();
    let dest = RectF::new(0.0, 0.0, 100.0, 100.0);
    svg.render(&mut c, dest, Color::BLACK);
    let first = c.shapes;
    assert!(first > 0);
    svg.render(&mut c, dest, Color::BLACK);
    assert_eq!(c.shapes, first, "a segunda pintura vem da imagem guardada");
    svg.render(&mut c, dest, Color::rgb(255, 0, 0));
    assert!(c.shapes > first, "outra cor do currentColor: desenha de novo");
    let plain = SvgImage::parse(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="5" height="5"/></svg>"#).unwrap();
    let before = c.cached.len();
    plain.render(&mut c, dest, Color::BLACK);
    assert_eq!(c.cached.len(), before, "sem efeitos, sem imagem guardada");
}

/// Um ícone de 24 com máscara e recorte (o caso de muitos ícones na tela).
const ICON: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
  <defs>
    <mask id="m"><rect width="24" height="24" fill="white"/><circle cx="16" cy="8" r="5" fill="black"/></mask>
    <clipPath id="c"><rect x="2" y="2" width="20" height="20" rx="4"/></clipPath>
  </defs>
  <g clip-path="url(#c)"><path d="M2 20 L12 4 L22 20 Z" fill="currentColor" mask="url(#m)"/></g>
</svg>"##;

const ICON_PLAIN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M2 20 L12 4 L22 20 Z" fill="currentColor"/></svg>"##;

/// O custo da máscara com muitos ícones (Skia na CPU e Direct2D): 600 ícones de 32 px numa tela
/// de 1280 × 800, sem efeito, com efeito na primeira pintura (sem imagem guardada) e nas
/// seguintes. `cargo test --release -p aylis-vector --test svg_effects -- --ignored --nocapture`.
#[test]
#[ignore]
fn the_cost_of_many_masked_icons() {
    use std::time::Instant;
    let (plain, masked) = (SvgImage::parse(ICON_PLAIN).unwrap(), SvgImage::parse(ICON).unwrap());
    let place = |i: usize| RectF::new((i % 30) as f32 * 42.0 + 4.0, (i / 30) as f32 * 40.0 + 4.0, 32.0, 32.0);
    const ICONS: usize = 600;
    let draw = |c: &mut dyn Canvas, svg: &SvgImage| {
        for i in 0..ICONS {
            svg.render(c, place(i), Color::rgb(40, 40, 40));
        }
    };
    let skia = aylis_skia::SkiaRenderer::new();
    let time = |f: &mut dyn FnMut()| {
        let t = Instant::now();
        f();
        t.elapsed().as_secs_f64() * 1000.0
    };
    let plain_ms = time(&mut || drop(skia.render_rgba(1280, 800, 1.0, Color::WHITE, |c| draw(c, &plain))));
    let first_ms = time(&mut || drop(skia.render_rgba(1280, 800, 1.0, Color::WHITE, |c| draw(c, &masked))));
    let mut cached_ms = f64::MAX;
    for _ in 0..5 {
        cached_ms = cached_ms.min(time(&mut || drop(skia.render_rgba(1280, 800, 1.0, Color::WHITE, |c| draw(c, &masked)))));
    }
    println!("Skia CPU, {ICONS} ícones: sem efeito {plain_ms:.1} ms; com máscara e recorte {first_ms:.1} ms na primeira pintura, {cached_ms:.1} ms guardado");
    #[cfg(windows)]
    {
        let d2d = aylis_d2d::D2dRenderer::new().unwrap();
        let best = |svg: &SvgImage| {
            let mut ms = f64::MAX;
            for _ in 0..5 {
                ms = ms.min(time(&mut || drop(d2d.render_rgba(1280, 800, 1.0, Color::WHITE, |c| draw(c, svg)))));
            }
            ms
        };
        let (plain_ms, masked_ms) = (best(&plain), best(&masked));
        println!("Direct2D, {ICONS} ícones: sem efeito {plain_ms:.1} ms; com máscara e recorte {masked_ms:.1} ms (o melhor de 5, com a cópia da GPU para a CPU)");
    }
}
