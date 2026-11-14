//! O form em vários monitores (`span_monitors`): os pedaços que a plataforma informa, onde os
//! menus abrem e a pintura de um pedaço com o deslocamento da janela dele.

mod common;

use aylis_core::*;
use common::{RecordingCanvas, boxy};

#[test]
fn a_popup_stays_in_the_piece_where_it_opens() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(1000.0, 500.0));
    // Uma janela só: o form inteiro.
    assert_eq!(ui.popup_area(form, PointF::new(900.0, 10.0)), RectF::new(0.0, 0.0, 1000.0, 500.0));
    let pieces = vec![RectF::new(0.0, 0.0, 420.0, 500.0), RectF::new(420.0, 0.0, 580.0, 300.0)];
    ui.set_form_pieces(form, pieces.clone());
    assert_eq!(ui.form_pieces(form), &pieces[..]);
    assert_eq!(ui.popup_area(form, PointF::new(100.0, 10.0)), pieces[0]);
    assert_eq!(ui.popup_area(form, PointF::new(900.0, 10.0)), pieces[1]);
    // No vão (abaixo do segundo monitor, que é mais baixo): o pedaço mais perto.
    assert_eq!(ui.popup_area(form, PointF::new(900.0, 400.0)), pieces[1]);
    // De volta a uma janela.
    ui.set_form_pieces(form, Vec::new());
    assert!(ui.form_pieces(form).is_empty());
}

#[test]
fn a_piece_paints_only_its_controls_moved_to_the_window() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(1000.0, 500.0));
    let left = ui.add(boxy(100.0, 50.0), form);
    ui.control_mut(left).unwrap().set_rect(10.0, 10.0, 100.0, 50.0);
    let right = ui.add(boxy(100.0, 50.0), form);
    ui.control_mut(right).unwrap().set_rect(600.0, 20.0, 100.0, 50.0);
    ui.layout();

    // O pedaço da direita começa em x = 420 do form: o controle em 600 sai em 180 na janela.
    let piece = RectF::new(420.0, 0.0, 580.0, 500.0);
    let mut canvas = RecordingCanvas::default();
    ui.paint_piece(form, &mut canvas, piece, None, true);
    let fills = canvas.fills();
    assert!(fills.contains(&RectF::new(180.0, 20.0, 100.0, 50.0)), "{fills:?}");
    assert!(!fills.iter().any(|r| r.width == 100.0 && r.x < 0.0), "o controle da esquerda não é pintado: {fills:?}");
    assert!(canvas.balanced());

    // Uma área (nas coordenadas da janela) fora de tudo o que muda: nada além do fundo.
    let mut canvas = RecordingCanvas::default();
    ui.paint_piece(form, &mut canvas, piece, Some(&[RectF::new(170.0, 10.0, 20.0, 20.0)]), false);
    assert!(canvas.fills().contains(&RectF::new(180.0, 20.0, 100.0, 50.0)));
    // Uma área que não cai no pedaço: não pinta.
    let mut canvas = RecordingCanvas::default();
    ui.paint_piece(form, &mut canvas, piece, Some(&[RectF::new(-300.0, 0.0, 10.0, 10.0)]), false);
    assert!(canvas.ops.is_empty());
}
