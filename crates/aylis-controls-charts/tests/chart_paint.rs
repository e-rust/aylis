//! A pintura do `Chart` em dois defeitos: o `border_color` que apagava a grade e a coluna
//! empilhada com cada pedaço arredondado.

use aylis_controls_charts::{Chart, ChartKind};
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::*;

fn chart(kind: ChartKind) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    let c = ui.add_root(Chart::default());
    ui.control_mut(c).unwrap().set_rect(0.0, 0.0, 600.0, 400.0);
    ui.set_prop(c, "kind", PropValue::enum_value(kind)).unwrap();
    ui.set_prop(c, "labels", "Jan;Fev;Mar").unwrap();
    ui.set_prop(c, "series", r#"[{"name": "A", "values": [3, 5, 2]}, {"name": "B", "values": [4, 1, 6]}]"#).unwrap();
    ui.set_prop(c, "animate", false).unwrap();
    ui.layout();
    (ui, c)
}

fn paint(ui: &mut Ui, c: ComponentId) -> Vec<Op> {
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(c, &mut canvas);
    canvas.ops
}

/// As linhas horizontais visíveis (a grade do eixo Y).
fn grid_lines(ops: &[Op]) -> usize {
    ops.iter().filter(|op| matches!(op, Op::Line { from, to, color, .. } if from.y == to.y && color.a > 0)).count()
}

/// Item 67: `border_color: transparent` tirava a moldura e também a grade (o `ColorOverride`
/// recolore o token `border` do tema do controle, e o `Chart` pintava a grade com ele).
#[test]
fn a_transparent_border_keeps_the_grid() {
    let (mut ui, c) = chart(ChartKind::Line);
    let before = paint(&mut ui, c);
    let lines = grid_lines(&before);
    assert!(lines > 0, "o gráfico tem grade");
    ui.set_prop(c, "border_color", Color::TRANSPARENT).unwrap();
    let after = paint(&mut ui, c);
    assert_eq!(grid_lines(&after), lines, "a grade fica");
    // A moldura sim: nenhum contorno visível do tamanho do controle.
    let frame = after.iter().any(|op| matches!(op, Op::StrokeRect { rect, color, .. } if rect.width >= 590.0 && color.a > 0));
    assert!(!frame, "a moldura some");
    // Uma cor de borda muda a moldura e não a grade.
    let red = Color::rgb(255, 0, 0);
    ui.set_prop(c, "border_color", red).unwrap();
    let ops = paint(&mut ui, c);
    assert!(ops.iter().any(|op| matches!(op, Op::StrokeRect { color, .. } if *color == red)), "a moldura na cor pedida");
    assert!(!ops.iter().any(|op| matches!(op, Op::Line { color, .. } if color.r == 255 && color.g == 0 && color.b == 0)), "a grade na cor do tema");
}

/// Item 68: a coluna empilhada arredondava os quatro cantos de cada pedaço, e o fundo aparecia
/// entre as cores. A barra deitada empilhada já pintava sem raio.
#[test]
fn stacked_columns_have_no_gaps_between_the_pieces() {
    let (mut ui, c) = chart(ChartKind::StackedBar);
    let ops = paint(&mut ui, c);
    let pieces: Vec<(RectF, f32)> = ops
        .iter()
        .filter_map(|op| match op {
            Op::FillRect { rect, radius, .. } if rect.width < 200.0 && rect.width > 12.0 && rect.height > 12.0 => Some((*rect, *radius)),
            _ => None,
        })
        .collect();
    assert!(pieces.len() >= 6, "dois pedaços por coluna: {pieces:?}");
    assert!(pieces.iter().all(|(_, r)| *r == 0.0), "sem raio nos pedaços: {pieces:?}");
}
