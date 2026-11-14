//! O editor de texto (Memo, e o editor de código da IDE) rola com a roda do mouse mesmo com
//! o foco: a rolagem só vai atrás do cursor quando o cursor anda ou o texto muda.

use aylis_controls::{Form, Memo};
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::*;

/// A primeira linha desenhada.
fn first_line(ui: &mut Ui, form: ComponentId) -> String {
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    canvas.ops.iter().find_map(|op| if let Op::Text { text, .. } = op { text.contains("linha").then(|| text.clone()) } else { None }).unwrap_or_default()
}

#[test]
fn the_wheel_scrolls_a_focused_editor_with_the_caret_still() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let text: Vec<String> = (1..=200).map(|i| format!("linha {i}")).collect();
    let memo = ui.add(Memo::new(text.join("\n")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.layout();
    ui.set_focus(memo);
    assert_eq!(first_line(&mut ui, form), "linha 1");

    let wheel = |ui: &mut Ui, dy: f32| ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, dy), ..Default::default() });
    wheel(&mut ui, -40.0);
    wheel(&mut ui, -40.0);
    let after = first_line(&mut ui, form);
    assert_ne!(after, "linha 1", "a roda rolou");
    // Repintar de novo (o cursor piscando, o mouse passando) não volta para o cursor.
    assert_eq!(first_line(&mut ui, form), after);
    assert_eq!(first_line(&mut ui, form), after);

    // Digitar (o cursor anda) traz a linha do cursor de volta à vista.
    ui.text_input(form, "x");
    assert_eq!(first_line(&mut ui, form), "xlinha 1");
}

#[test]
fn the_scrollbar_appears_and_dragging_the_thumb_scrolls() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let text: Vec<String> = (1..=200).map(|i| format!("linha {i}")).collect();
    let memo = ui.add(Memo::new(text.join("\n")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.layout();
    assert_eq!(first_line(&mut ui, form), "linha 1");
    // O polegar: um retângulo arredondado estreito na borda direita, perto do topo.
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    let thumb = canvas
        .ops
        .iter()
        .rev()
        .find_map(|op| if let Op::FillRect { rect, .. } = op { (rect.width <= 10.0 && rect.height >= 20.0).then_some(*rect) } else { None })
        .expect("a barra de rolagem");
    assert!(thumb.x > 360.0, "na borda direita: {thumb:?}");

    // Arrastar o polegar até embaixo: aparece o fim do texto, e o cursor não sai do lugar.
    let start = PointF::new(thumb.x + thumb.width / 2.0, thumb.y + 5.0);
    let ev = |p: PointF| PointerEvent { position: p, ..Default::default() };
    ui.pointer_down(form, ev(start));
    ui.pointer_move(form, ev(PointF::new(start.x, 400.0)));
    ui.pointer_up(form, ev(PointF::new(start.x, 400.0)));
    let end = first_line(&mut ui, form);
    let n: usize = end.trim_start_matches("linha ").parse().unwrap();
    assert!(n > 180, "rolou até o fim: {end}");
    assert_eq!(first_line(&mut ui, form), end, "repintar não volta ao cursor");
    assert_eq!(ui.get_prop(memo, "caret_line").unwrap(), PropValue::Int(1), "o clique na barra não mexe no cursor");

    // Clicar no trilho, no meio: vai para o meio do texto.
    ui.pointer_down(form, ev(PointF::new(start.x, 150.0)));
    ui.pointer_up(form, ev(PointF::new(start.x, 150.0)));
    let middle: usize = first_line(&mut ui, form).trim_start_matches("linha ").parse().unwrap();
    assert!((70..130).contains(&middle), "o meio do texto: {middle}");
}

/// O x do começo da linha `linha 1` (negativo quando rolou de lado).
fn x_of_the_first(ui: &mut Ui, form: ComponentId) -> f32 {
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    canvas.ops.iter().find_map(|op| if let Op::Text { text, rect, .. } = op { text.contains("linha 1 ").then_some(rect.x) } else { None }).expect("a linha 1")
}

#[test]
fn the_horizontal_bar_scrolls_long_lines() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let long = "x".repeat(300);
    let memo = ui.add(Memo::new(format!("linha 1 {long}\nlinha 2")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.set_prop(memo, "trimming", "none").unwrap();
    ui.layout();
    let before = x_of_the_first(&mut ui, form);

    // A barra horizontal: o polegar largo e baixinho embaixo.
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    let thumb = canvas
        .ops
        .iter()
        .rev()
        .find_map(|op| if let Op::FillRect { rect, .. } = op { (rect.height <= 10.0 && rect.width >= 20.0 && rect.y > 250.0).then_some(*rect) } else { None })
        .expect("a barra horizontal");
    // Arrastar o polegar até a direita: as linhas andam para a esquerda.
    let start = PointF::new(thumb.x + 5.0, thumb.y + thumb.height / 2.0);
    let ev = |p: PointF| PointerEvent { position: p, ..Default::default() };
    ui.pointer_down(form, ev(start));
    ui.pointer_move(form, ev(PointF::new(390.0, start.y)));
    ui.pointer_up(form, ev(PointF::new(390.0, start.y)));
    let after = x_of_the_first(&mut ui, form);
    assert!(after < before - 100.0, "rolou de lado: {before} -> {after}");
    assert_eq!(x_of_the_first(&mut ui, form), after, "e fica");

    // Shift+roda volta para a esquerda.
    for _ in 0..40 {
        ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, 40.0), modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() });
    }
    assert_eq!(x_of_the_first(&mut ui, form), before, "Shift+roda voltou ao começo");
}

#[test]
fn a_wheel_notch_glides_frame_by_frame_in_the_editor() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let text: Vec<String> = (1..=200).map(|i| format!("linha {i}")).collect();
    let memo = ui.add(Memo::new(text.join("\n")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.layout();
    assert_eq!(first_line(&mut ui, form), "linha 1");
    ui.set_time(1.0);
    let notch = |ui: &mut Ui| ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
    notch(&mut ui);
    notch(&mut ui);
    assert_eq!(first_line(&mut ui, form), "linha 1", "o deslize começa no quadro, não salta");
    assert!(ui.is_animating(), "o deslize pede os quadros");
    ui.set_time(1.04);
    ui.tick();
    let mid = first_line(&mut ui, form);
    ui.set_time(1.5);
    ui.tick();
    // Dois dentes de 8 linhas (o `wheel_lines`): a 17ª no alto.
    assert_eq!(first_line(&mut ui, form), "linha 17", "no meio estava em {mid}");
    assert!(!ui.is_animating());
}

#[test]
fn with_slow_frames_the_wheel_jumps_right_away() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let text: Vec<String> = (1..=200).map(|i| format!("linha {i}")).collect();
    let memo = ui.add(Memo::new(text.join("\n")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.layout();
    assert_eq!(first_line(&mut ui, form), "linha 1");
    // Um build de depuração: 60 ms por quadro.
    ui.frame_cost_measured(0.060);
    assert!(!ui.smooth_scrolling());
    ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
    assert_eq!(first_line(&mut ui, form), "linha 9", "salta na hora, sem esperar os quadros");
    // Com 2 linhas por dente.
    ui.set_prop(memo, "wheel_lines", 2u32).unwrap();
    ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
    assert_eq!(first_line(&mut ui, form), "linha 11");
}

#[test]
fn the_code_editor_scrolls_until_the_last_line_is_at_the_top() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let editor = ui.create("CodeEditor", Some(form)).unwrap();
    ui.control_mut(editor).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    let text: Vec<String> = (1..=40).map(|i| format!("linha {i}")).collect();
    ui.set_prop(editor, "text", text.join("\n")).unwrap();
    ui.layout();
    ui.frame_cost_measured(0.060);
    for _ in 0..30 {
        ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
        let _ = first_line(&mut ui, form);
    }
    assert_eq!(first_line(&mut ui, form), "linha 40", "a última linha no alto");
}

#[test]
fn ctrl_and_the_wheel_scroll_a_page_per_notch() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let text: Vec<String> = (1..=200).map(|i| format!("linha {i}")).collect();
    let memo = ui.add(Memo::new(text.join("\n")), form);
    ui.control_mut(memo).unwrap().set_rect(10.0, 10.0, 380.0, 280.0);
    ui.layout();
    let _ = first_line(&mut ui, form);
    ui.frame_cost_measured(0.060);
    let ctrl = Modifiers { ctrl: true, ..Default::default() };
    ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -100.0), lines: true, modifiers: ctrl, ..Default::default() });
    let top: usize = first_line(&mut ui, form).trim_start_matches("linha ").parse().unwrap();
    assert!(top > 9, "uma página (mais que os 8 do dente comum): começou na linha {top}");
}

/// Num fonte comprido (o `mod.rs` gerado de um ERP tem milhares de linhas), arrastar o
/// minimapa até embaixo chega à última linha: a escala do minimapa diminui até o arquivo
/// inteiro caber (com um piso, o fim ficava fora da faixa e o arrasto parava no meio).
#[test]
fn dragging_the_minimap_to_the_bottom_reaches_the_last_line() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 900.0, 600.0));
    let editor = ui.create("CodeEditor", Some(form)).unwrap();
    let text: Vec<String> = (1..=20_000).map(|i| format!("linha {i}")).collect();
    ui.set_prop(editor, "text", text.join("\n")).unwrap();
    ui.control_mut(editor).unwrap().set_rect(0.0, 0.0, 900.0, 600.0);
    ui.layout();
    let _ = first_line(&mut ui, form);
    // O minimapa fica à direita, antes da barra de rolagem.
    let x = 840.0;
    ui.pointer_down(form, PointerEvent { position: PointF::new(x, 10.0), ..Default::default() });
    for y in [100.0, 300.0, 500.0, 598.0] {
        ui.pointer_move(form, PointerEvent { position: PointF::new(x, y), ..Default::default() });
    }
    ui.pointer_up(form, PointerEvent { position: PointF::new(x, 598.0), ..Default::default() });
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    let shown: Vec<String> = canvas.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { text.starts_with("linha").then(|| text.clone()) } else { None }).collect();
    assert!(shown.iter().any(|t| t == "linha 20000"), "o fim não apareceu; à vista: {:?}", shown.first());
}
