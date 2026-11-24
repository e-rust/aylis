//! A suavização do texto: em cinza por padrão (com o desenho simétrico) e o ClearType do
//! projeto (`text_antialias`) só sobre fundo opaco. Um teste só: a opção é do processo.
#![cfg(windows)]

use aylis_core::canvas::{set_text_antialias, text_antialias};
use aylis_core::{Canvas, Color, RectF, TextAntialias, TextStyle};

/// Pixels com cor (o subpixel deixa franjas coloridas na borda das letras pretas).
fn colored(px: &[u8]) -> usize {
    px.chunks(4)
        .filter(|p| p[3] > 0)
        .filter(|p| {
            let (r, g, b) = (p[0] as i32, p[1] as i32, p[2] as i32);
            (r - g).abs() + (g - b).abs() + (r - b).abs() > 30
        })
        .count()
}

fn style() -> TextStyle {
    let mut s = TextStyle::default();
    s.color = Color::BLACK;
    s.font.size = 14.0;
    s
}

const TEXT: &str = "Aylis wavy diagonals 123";

/// O texto, direto (`how = 0`), numa camada guardada (`1`) ou num recorte arredondado (`2`).
fn render(r: &aylis_d2d::D2dRenderer, background: Color, how: u8) -> Vec<u8> {
    r.render_rgba(240, 40, 1.0, background, |c| {
        let rect = RectF::new(4.0, 4.0, 232.0, 32.0);
        match how {
            1 => {
                if !c.begin_cached(7, RectF::new(0.0, 0.0, 240.0, 40.0)) {
                    c.draw_text(TEXT, rect, &style());
                }
                c.end_cached();
            }
            2 => {
                c.save();
                c.clip_round_rect(RectF::new(0.0, 0.0, 240.0, 40.0), 6.0);
                c.draw_text(TEXT, rect, &style());
                c.restore();
            }
            _ => c.draw_text(TEXT, rect, &style()),
        }
    })
    .unwrap()
}

fn width(r: &aylis_d2d::D2dRenderer) -> f32 {
    let mut w = 0.0;
    r.render_rgba(8, 8, 1.0, Color::WHITE, |c| w = c.measure_text(TEXT, &style()).width).unwrap();
    w
}

#[test]
fn cleartype_only_where_the_project_asks_and_the_background_is_opaque() {
    let r = aylis_d2d::D2dRenderer::new().unwrap();
    assert_eq!(text_antialias(), TextAntialias::Grayscale, "o padrão é o cinza");
    let gray = render(&r, Color::WHITE, 0);
    assert_eq!(colored(&gray), 0, "em cinza, sem franjas");
    assert!(gray.chunks(4).any(|p| p[0] < 100), "o texto aparece");
    let gray_width = width(&r);

    set_text_antialias(TextAntialias::ClearType);
    let clear = render(&r, Color::WHITE, 0);
    assert!(colored(&clear) > 20, "o ClearType sobre o branco: {} pixels com franja", colored(&clear));
    assert_eq!(width(&r), gray_width, "o modo não muda o tamanho medido");
    assert!(colored(&render(&r, Color::WHITE, 2)) > 20, "o recorte leva o fundo para a camada: ClearType também");
    assert_eq!(colored(&render(&r, Color::WHITE, 1)), 0, "a camada guardada é transparente: em cinza");
    assert_eq!(colored(&render(&r, Color::TRANSPARENT, 0)), 0, "o fundo transparente: em cinza");
    // Por partes (duas áreas: a camada com a máscara da união), igual à pintura inteira.
    let previous = vec![255u8; clear.len()];
    let areas = [RectF::new(0.0, 0.0, 120.0, 40.0), RectF::new(120.0, 0.0, 120.0, 40.0)];
    let parts = r.render_parts_rgba(240, 40, 1.0, Color::WHITE, &previous, &[], &areas, |c, _| c.draw_text(TEXT, RectF::new(4.0, 4.0, 232.0, 32.0), &style())).unwrap();
    assert_eq!(parts, clear, "por partes, o mesmo ClearType da pintura inteira");
    set_text_antialias(TextAntialias::Grayscale);
    assert_eq!(render(&r, Color::WHITE, 0), gray, "voltando ao cinza, o mesmo desenho");
}
