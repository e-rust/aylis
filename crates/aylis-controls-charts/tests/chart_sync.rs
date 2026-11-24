//! O `Chart`: o `border_width` e o `padding`, o `highlight_index`, o `on_hover` e o `ChartSync`
//! com o cursor e a janela pelo valor do X.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls_charts::{Chart, ChartKind, ChartSync, SyncEvent, SyncKind};
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::*;

fn chart(ui: &mut Ui, kind: ChartKind, labels: &str, values: &str) -> ComponentId {
    let c = ui.add_root(Chart::default());
    ui.control_mut(c).unwrap().set_rect(0.0, 0.0, 600.0, 400.0);
    ui.set_prop(c, "kind", PropValue::enum_value(kind)).unwrap();
    ui.set_prop(c, "labels", labels).unwrap();
    ui.set_prop(c, "series", format!(r#"[{{"name": "A", "values": [{values}]}}]"#)).unwrap();
    ui.set_prop(c, "animate", false).unwrap();
    ui.set_prop(c, "legend", false).unwrap();
    ui.layout();
    c
}

fn paint(ui: &mut Ui, c: ComponentId) -> Vec<Op> {
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(c, &mut canvas);
    canvas.ops
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

/// O centro da coluna `i` de `n` no gráfico (pelo plot pintado: a primeira e a última linha da
/// grade dão as bordas).
fn column_x(ops: &[Op], i: usize, n: usize) -> f32 {
    let (left, right) = ops
        .iter()
        .filter_map(|op| match op {
            Op::Line { from, to, .. } if from.y == to.y && to.x > from.x + 100.0 => Some((from.x, to.x)),
            _ => None,
        })
        .next()
        .expect("a grade");
    left + (right - left) * (i as f32 + 0.5) / n as f32
}

/// A coluna destacada (o fundo do `hover_overlay` da altura do plot).
fn highlighted(ui: &Ui, c: ComponentId, ops: &[Op]) -> Option<f32> {
    let overlay = ui.theme().color("hover_overlay");
    ops.iter().find_map(|op| match op {
        Op::FillRect { rect, color, .. } if *color == overlay && rect.height > 100.0 => Some(rect.x + rect.width / 2.0),
        _ => None,
    }).filter(|_| ui.contains(c))
}

fn has_text(ops: &[Op], text: &str) -> bool {
    ops.iter().any(|op| matches!(op, Op::Text { text: t, .. } if t == text))
}

/// O cursor pintado: a linha tracejada de cima a baixo do plot.
fn cursor_line(ops: &[Op]) -> Option<f32> {
    ops.iter().find_map(|op| match op {
        Op::Line { from, to, dash, .. } if from.x == to.x && !dash.is_empty() && to.y - from.y > 100.0 => Some(from.x),
        _ => None,
    })
}

#[test]
fn border_width_and_padding_are_chart_properties_with_the_old_defaults() {
    let mut ui = Ui::new();
    let c = chart(&mut ui, ChartKind::Line, "Jan;Fev;Mar", "3, 5, 2");
    // Os padrões: a moldura do estilo (1) e o espaço de antes.
    assert_eq!(ui.get_prop(c, "border_width").unwrap(), PropValue::Null);
    assert_eq!(ui.control(c).unwrap().padding(), Bounds::new(12.0, 12.0, 12.0, 10.0));
    let before = paint(&mut ui, c);
    assert!(before.iter().any(|op| matches!(op, Op::StrokeRect { rect, width, .. } if rect.width >= 590.0 && *width == 1.0)));
    let left = column_x(&before, 0, 3);
    ui.set_prop(c, "border_width", 3.0).unwrap();
    let ops = paint(&mut ui, c);
    assert!(ops.iter().any(|op| matches!(op, Op::StrokeRect { rect, width, .. } if rect.width >= 590.0 && *width == 3.0)));
    ui.set_prop(c, "border_width", 0.0).unwrap();
    let ops = paint(&mut ui, c);
    assert!(!ops.iter().any(|op| matches!(op, Op::StrokeRect { rect, .. } if rect.width >= 590.0)), "sem moldura");
    // Sem espaço de dentro, o gráfico chega mais perto da borda.
    ui.set_prop(c, "padding", Bounds::all(0.0)).unwrap();
    let ops = paint(&mut ui, c);
    assert!(column_x(&ops, 0, 3) < left - 5.0);
}

#[test]
fn highlight_index_marks_the_point_with_its_hint() {
    let mut ui = Ui::new();
    let c = chart(&mut ui, ChartKind::Line, "Jan;Fev;Mar", "3, 5, 2");
    assert_eq!(ui.get_prop(c, "highlight_index").unwrap(), PropValue::Int(-1));
    let ops = paint(&mut ui, c);
    assert!(highlighted(&ui, c, &ops).is_none());
    ui.set_prop(c, "highlight_index", 1).unwrap();
    let ops = paint(&mut ui, c);
    let x = highlighted(&ui, c, &ops).expect("a coluna destacada");
    assert!((x - column_x(&ops, 1, 3)).abs() < 1.0);
    // A dica do ponto (o rótulo e o valor) sai sozinha.
    paint(&mut ui, c);
    let ops = paint(&mut ui, c);
    assert!(has_text(&ops, "A: 5"), "a dica do ponto destacado");
}

#[test]
fn on_hover_reports_the_point_under_the_mouse() {
    let mut ui = Ui::new();
    let c = chart(&mut ui, ChartKind::Bar, "Jan;Fev;Mar", "3, 5, 2");
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.on(c, "on_hover", move |_, _, args| s.borrow_mut().push(<i64 as EventValue>::from_args(args).unwrap()));
    let ops = paint(&mut ui, c);
    let x2 = column_x(&ops, 2, 3);
    ui.pointer_move(c, at(x2, 200.0));
    ui.pointer_move(c, at(x2 + 1.0, 201.0));
    ui.pointer_move(c, at(column_x(&ops, 0, 3), 200.0));
    ui.pointer_leave(c);
    assert_eq!(*seen.borrow(), [2, 0, -1]);
}

/// Dois gráficos com pontos diferentes (o segundo tem um horário a mais) acham o mesmo horário
/// pelo rótulo, não pelo índice.
#[test]
fn chart_sync_shares_the_cursor_by_the_x_value() {
    let mut ui = Ui::new();
    let sync = ui.add_root(ChartSync::new());
    ui.set_name(sync, "sync").unwrap();
    let a = chart(&mut ui, ChartKind::Line, "10:00;11:00;12:00", "3, 5, 2");
    ui.set_name(a, "chtCpu").unwrap();
    let b = chart(&mut ui, ChartKind::Bar, "09:00;10:00;11:00;12:00", "1, 2, 3, 4");
    for c in [a, b] {
        ui.set_prop(c, "sync", PropValue::Component(Some(sync))).unwrap();
    }
    let events = Rc::new(RefCell::new(Vec::<SyncEvent>::new()));
    let e = events.clone();
    ui.on(sync, "on_sync", move |_, _, args| e.borrow_mut().push(<SyncEvent as EventValue>::from_args(args).unwrap()));
    let ops_a = paint(&mut ui, a);
    let ops_b = paint(&mut ui, b);
    assert!(cursor_line(&ops_b).is_none());
    // O mouse em "11:00" no primeiro (índice 1)...
    ui.pointer_move(a, at(column_x(&ops_a, 1, 3), 200.0));
    ui.process_posted();
    assert_eq!(ui.get::<ChartSync>(sync).unwrap().cursor(), Some("11:00"));
    assert_eq!(ui.get_prop(sync, "cursor").unwrap(), PropValue::String("11:00".into()));
    // ... destaca "11:00" no segundo (índice 2), com a linha e a dica.
    let ops = paint(&mut ui, b);
    let x = highlighted(&ui, b, &ops).expect("o mesmo horário destacado");
    assert!((x - column_x(&ops_b, 2, 4)).abs() < 1.0);
    let line = cursor_line(&ops).expect("a linha do cursor");
    assert!((line - column_x(&ops_b, 2, 4)).abs() < 1.0);
    let ops = paint(&mut ui, b);
    assert!(has_text(&ops, "A: 3"), "a dica no segundo gráfico");
    assert_eq!(*events.borrow(), [SyncEvent { kind: SyncKind::Cursor, cursor: "11:00".into(), source: "chtCpu".into(), ..Default::default() }]);
    // O mouse sai: o cursor some de todos.
    ui.pointer_leave(a);
    ui.process_posted();
    assert_eq!(ui.get::<ChartSync>(sync).unwrap().cursor(), None);
    assert!(cursor_line(&paint(&mut ui, b)).is_none());
    assert_eq!(events.borrow().len(), 2);
    // O app também muda (sem `on_sync`: o evento é do usuário).
    ChartSync::set_cursor(&mut ui, sync, Some("12:00"));
    let ops = paint(&mut ui, a);
    assert!((highlighted(&ui, a, &ops).unwrap() - column_x(&ops_a, 2, 3)).abs() < 1.0);
    ui.process_posted();
    assert_eq!(events.borrow().len(), 2);
}

#[test]
fn chart_sync_shares_the_zoom_window() {
    let mut ui = Ui::new();
    let sync = ui.add_root(ChartSync::new());
    let a = chart(&mut ui, ChartKind::Line, "Jan;Fev;Mar;Abr;Mai;Jun", "1, 2, 3, 4, 5, 6");
    let b = chart(&mut ui, ChartKind::Line, "Fev;Mar;Abr", "7, 8, 9");
    ui.set_prop(a, "select_range", true).unwrap();
    for c in [a, b] {
        ui.set_prop(c, "sync", PropValue::Component(Some(sync))).unwrap();
    }
    let ops = paint(&mut ui, a);
    paint(&mut ui, b);
    // Arrastar de "Mar" a "Abr" no primeiro seleciona a faixa e aproxima nela.
    let (x0, x1) = (column_x(&ops, 2, 6), column_x(&ops, 3, 6));
    ui.pointer_down(a, at(x0, 200.0));
    ui.pointer_move(a, at((x0 + x1) / 2.0, 200.0));
    assert!(paint(&mut ui, a).iter().any(|op| matches!(op, Op::FillRect { rect, .. } if rect.height > 100.0 && rect.x == x0)), "a faixa pintada");
    ui.pointer_move(a, at(x1, 200.0));
    ui.pointer_up(a, at(x1, 200.0));
    ui.process_posted();
    assert_eq!(ui.get::<ChartSync>(sync).unwrap().range(), Some(("Mar", "Abr")));
    let wa = ui.get::<Chart>(a).unwrap().zoom_window();
    assert!(wa.0 > 0.3 && wa.1 < 0.75, "{wa:?}");
    // O segundo mostra só "Mar" e "Abr" (as posições 1 e 2 dos três dele).
    paint(&mut ui, b);
    let wb = ui.get::<Chart>(b).unwrap().zoom_window();
    assert!((wb.0 - 1.0 / 3.0).abs() < 1e-9 && (wb.1 - 1.0).abs() < 1e-9, "{wb:?}");
    // Pelo código: o eixo todo.
    ChartSync::set_range(&mut ui, sync, None);
    paint(&mut ui, a);
    paint(&mut ui, b);
    assert_eq!(ui.get::<Chart>(a).unwrap().zoom_window(), (0.0, 1.0));
    assert_eq!(ui.get::<Chart>(b).unwrap().zoom_window(), (0.0, 1.0));
    // Pelo nome (o protocolo): a janela.
    ui.set_prop(sync, "range", r#"{"from": "Abr", "to": "Abr"}"#).unwrap();
    ui.process_posted();
    paint(&mut ui, b);
    let wb = ui.get::<Chart>(b).unwrap().zoom_window();
    assert!((wb.0 - 2.0 / 3.0).abs() < 1e-9, "{wb:?}");
}

/// Um gráfico com outro tipo de eixo X ignora o que veio de um eixo de categorias.
#[test]
fn a_chart_with_another_x_axis_kind_ignores_the_sync() {
    let mut ui = Ui::new();
    let sync = ui.add_root(ChartSync::new());
    let a = chart(&mut ui, ChartKind::Line, "Jan;Fev;Mar", "1, 2, 3");
    let t = chart(&mut ui, ChartKind::Line, "2026-01-01;2026-02-01;2026-03-01", "1, 2, 3");
    ui.set_prop(t, "time_axis", true).unwrap();
    for c in [a, t] {
        ui.set_prop(c, "sync", PropValue::Component(Some(sync))).unwrap();
    }
    let ops = paint(&mut ui, a);
    paint(&mut ui, t);
    ui.pointer_move(a, at(column_x(&ops, 1, 3), 200.0));
    let ops_t = paint(&mut ui, t);
    assert!(cursor_line(&ops_t).is_none() && highlighted(&ui, t, &ops_t).is_none());
    // O app (sem tipo de eixo) vale para o de tempo: a data mais perto.
    ChartSync::set_cursor(&mut ui, sync, Some("2026-02-03"));
    let ops_t = paint(&mut ui, t);
    assert!(cursor_line(&ops_t).is_some());
    assert!(highlighted(&ui, t, &ops_t).is_some());
}
