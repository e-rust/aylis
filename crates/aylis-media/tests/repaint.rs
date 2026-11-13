//! Quadros de câmera e a repintura (relatado em 13/11 por um app com várias câmeras e duas
//! janelas): cada quadro empurrado pedia a repintura inteira de todas as janelas, com uma
//! mensagem por quadro para a thread da interface. Agora é só a área do player, na janela
//! dele, e um aviso até a tela atender.

use aylis_controls::{Form, Label};
use aylis_core::*;
use aylis_media::*;

/// Dois forms abertos (duas janelas): A com um `VideoPlayer`, B com um rótulo. Os dois já
/// foram pintados uma vez (nada pendente).
fn two_windows() -> (Ui, ComponentId, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    let a = ui.add_root(Form::new("Câmeras", 800.0, 600.0));
    let player = ui.add(VideoPlayer::default(), a);
    ui.control_mut(player).unwrap().set_size(320.0, 180.0);
    let b = ui.add_root(Form::new("Revisão", 400.0, 300.0));
    ui.add(Label::new("ao lado"), b);
    ui.show(a).unwrap();
    ui.show(b).unwrap();
    ui.update_layout();
    let _ = ui.take_frame(a);
    let _ = ui.take_frame(b);
    (ui, a, b, player)
}

fn frame(t: f64) -> Frame {
    Frame::bgra(4, 4, vec![128; 4 * 4 * 4], t)
}

#[test]
fn camera_frames_repaint_only_the_player_in_its_window() {
    let (mut ui, a, b, player) = two_windows();
    let sink = VideoPlayer::sink(&ui, player).unwrap();
    assert!(ui.forms_to_paint(&[a, b]).is_empty(), "nada pendente");
    // 30 quadros chegam antes de a tela rodar as mensagens (como 2 s de uma câmera a 15 fps
    // contra uma tela ocupada).
    for i in 0..30 {
        sink.push_frame(frame(i as f64 / 15.0));
    }
    let messages = ui.process_posted();
    assert_eq!(messages, 1, "um aviso até a tela atender, não um por quadro");
    assert!(ui.needs_repaint());
    assert_eq!(ui.forms_to_paint(&[a, b]), vec![a], "só a janela do player");
    match ui.take_frame(a) {
        FrameDamage::Parts { areas, .. } => {
            let area = areas.into_iter().reduce(|x, y| x.union(y)).unwrap();
            assert!(area.width <= 330.0 && area.height <= 190.0, "só a área do player: {area:?}");
        }
        other => panic!("a janela inteira: {other:?}"),
    }
    // Atendido, o próximo quadro pede de novo.
    sink.push_frame(frame(3.0));
    assert_eq!(ui.process_posted(), 1);
    assert_eq!(ui.forms_to_paint(&[a, b]), vec![a]);
}

#[test]
fn a_repaint_without_place_still_paints_every_window() {
    let (mut ui, a, b, _) = two_windows();
    ui.request_repaint();
    assert_eq!(ui.forms_to_paint(&[a, b]), vec![a, b], "sem saber onde: todas, como antes");
}

#[test]
fn a_mark_in_a_form_without_window_paints_every_window() {
    let (mut ui, a, b, _) = two_windows();
    // Um form que não é janela (o do designer da IDE, pintado dentro dela).
    let designer = ui.add_root(Form::new("Designer", 300.0, 200.0));
    let label = ui.add(Label::new("x"), designer);
    ui.update_layout();
    let _ = ui.take_frame(a);
    let _ = ui.take_frame(b);
    ui.damage(label);
    assert_eq!(ui.forms_to_paint(&[a, b]), vec![a, b]);
}

/// Um controle que pede um quadro numa hora (o vídeo tocando, o cursor piscando): na hora,
/// só ele é marcado, e só a janela dele pinta (antes, todas, e cada uma inteira).
struct Blinker {
    base: ControlBase,
}

impl Component for Blinker {
    fn class_name(&self) -> &'static str {
        "Blinker"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }
    fn paint(&self, _canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        ctx.request_frame_at(ctx.time + 0.5);
    }
}

#[test]
fn a_timed_frame_marks_only_the_control_that_asked() {
    let (mut ui, a, b, _) = two_windows();
    let blinker = ui.add(Blinker { base: ControlBase::new(20.0, 20.0) }, b);
    ui.update_layout();
    let mut canvas = aylis_core::pages::RecordingCanvas::new(None);
    ui.paint(b, &mut canvas);
    let _ = ui.take_frame(a);
    let _ = ui.take_frame(b);
    let at = ui.next_paint_wake().expect("o pedido do controle");
    assert!(!ui.take_due_frames(at - 0.1), "antes da hora, nada");
    assert!(ui.take_due_frames(at));
    assert_eq!(ui.forms_to_paint(&[a, b]), vec![b], "só a janela de quem pediu");
    match ui.take_frame(b) {
        FrameDamage::Parts { areas, .. } => assert!(areas.iter().all(|r| r.width <= 30.0 && r.height <= 30.0), "{areas:?}"),
        other => panic!("a janela inteira: {other:?}"),
    }
    let _ = blinker;
}
