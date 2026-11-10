//! O designer mostra, apagado, o que passa da borda do form, e só pinta essa camada
//! quando há algo para fora (pintar tudo de novo custava um quadro inteiro, e um Model3D
//! duas vezes).

use aylis_controls::{Button, Form, Panel};
use aylis_core::pages::RecordingCanvas;
use aylis_core::*;

#[test]
fn only_what_sticks_out_is_repainted() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    ui.control_mut(form).unwrap().set_rect(120.0, 80.0, 400.0, 300.0);
    let inside = ui.add(Button::new("Dentro"), form);
    ui.control_mut(inside).unwrap().set_rect(10.0, 10.0, 100.0, 30.0);
    let panel = ui.add(Panel::new(200.0, 100.0), form);
    ui.control_mut(panel).unwrap().set_rect(50.0, 100.0, 200.0, 100.0);
    ui.layout();
    assert!(!ui.overflows(form), "tudo dentro: sem a camada");

    // Um filho do painel que sai pela direita do form.
    let outside = ui.add(Button::new("Fora"), panel);
    ui.control_mut(outside).unwrap().set_rect(150.0, 10.0, 300.0, 30.0);
    ui.layout();
    assert!(ui.overflows(form));
    let mut canvas = RecordingCanvas::new(None);
    ui.paint_overflow(form, &mut canvas);
    let texts: Vec<String> = canvas.ops.iter().filter_map(|op| if let aylis_core::pages::Op::Text { text, .. } = op { Some(text.clone()) } else { None }).collect();
    assert!(texts.contains(&"Fora".to_string()), "{texts:?}");
    assert!(!texts.contains(&"Dentro".to_string()), "o que está dentro não é pintado de novo: {texts:?}");
}

/// O balão do tempo da barra de mídia sai por cima dela e fica por cima do painel de cima
/// (que é pintado depois): desenhado no fim do quadro.
#[test]
fn the_media_bar_bubble_stays_above_the_siblings() {
    use aylis_controls::SeekBar;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let bar = ui.add(SeekBar::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(30.0);
    let panel = ui.add(Panel::new(100.0, 100.0), form);
    ui.control_mut(panel).unwrap().set_align(Align::Client);
    ui.layout();
    let r = ui.absolute_rect(bar).unwrap();
    let p = PointF::new(r.x + r.width / 2.0, r.y + 15.0);
    ui.pointer_move(form, PointerEvent { position: p, ..Default::default() });
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(form, &mut canvas);
    // O texto do balão ("0:50", metade de 100 s) é a última coisa pintada.
    let last_text = canvas.ops.iter().rev().find_map(|op| if let aylis_core::pages::Op::Text { text, rect, .. } = op { Some((text.clone(), *rect)) } else { None });
    let (text, rect) = last_text.expect("o balão");
    assert_eq!(text, "0:50");
    assert!(rect.y < r.y, "acima da barra: {rect:?}");
}

/// Um player atualiza a posição da barra a cada instante (timer). Se isso cai entre apertar
/// e soltar o botão, o `on_seek` tem de levar o ponto clicado, não a posição de fora.
#[test]
fn the_media_bar_seeks_to_the_clicked_point_even_with_the_timer_in_between() {
    use aylis_controls::SeekBar;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 100.0));
    let bar = ui.add(SeekBar::default(), form);
    ui.control_mut(bar).unwrap().set_rect(0.0, 30.0, 400.0, 30.0);
    ui.set_prop(bar, "duration", 200.0).unwrap();
    ui.layout();
    let order = std::rc::Rc::new(std::cell::Cell::new(-1.0));
    let p = order.clone();
    ui.on(bar, "on_seek", move |_, _, args| {
        if let EventArgs::Value(v) = args {
            p.set(v.to_f64().unwrap());
        }
    });
    let middle = PointF::new(200.0, 45.0);
    ui.pointer_down(form, PointerEvent { position: middle, ..Default::default() });
    ui.set_prop(bar, "position", 3.0).unwrap(); // o timer do player
    ui.pointer_up(form, PointerEvent { position: middle, ..Default::default() });
    assert!((order.get() - 100.0).abs() < 1.0, "on_seek com o ponto clicado: {}", order.get());
    // Solto, a posição de fora volta a valer.
    ui.set_prop(bar, "position", 7.0).unwrap();
    assert_eq!(ui.get_prop(bar, "position").unwrap(), PropValue::Float(7.0));
}
