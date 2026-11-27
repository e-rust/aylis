//! Limite e medida de quadros por segundo.

use aylis_core::*;

#[test]
fn fps_counts_the_last_second_and_the_limit_spaces_frames() {
    let mut ui = Ui::new();
    assert_eq!(ui.next_frame_at(), None, "sem limite");
    for i in 0..60 {
        ui.set_time(10.0 + i as f64 / 60.0);
        ui.frame_drawn();
    }
    assert!((ui.fps() - 60.0).abs() <= 1.0, "{}", ui.fps());
    // Um segundo e meio depois, sem quadros: 0.
    ui.set_time(12.5);
    assert_eq!(ui.fps(), 0.0);

    ui.set_max_fps(30.0);
    ui.set_time(20.0);
    ui.frame_drawn();
    let next = ui.next_frame_at().unwrap();
    assert!((next - (20.0 + 1.0 / 30.0)).abs() < 1e-9);
    ui.set_max_fps(0.0);
    assert_eq!(ui.next_frame_at(), None);
}

/// A latência da entrada ao quadro: desligada, nada; ligada, uma amostra por quadro que vem
/// depois de entradas (várias entradas antes do mesmo quadro contam da primeira).
#[test]
fn the_input_latency_counts_from_the_first_input_to_the_next_frame() {
    let mut ui = Ui::new();
    ui.note_input();
    ui.frame_drawn();
    assert_eq!(ui.input_latency(), None, "desligada");
    ui.set_measure_input_latency(true);
    ui.frame_drawn();
    assert_eq!(ui.input_latency(), None, "quadro sem entrada não conta");
    ui.note_input();
    std::thread::sleep(std::time::Duration::from_millis(5));
    ui.note_input();
    ui.frame_drawn();
    ui.frame_drawn();
    let latency = ui.input_latency().unwrap();
    assert_eq!(latency.count, 1);
    assert!(latency.p50_ms >= 4.0, "{latency:?}");
    ui.set_measure_input_latency(false);
    assert_eq!(ui.input_latency(), None);
}
