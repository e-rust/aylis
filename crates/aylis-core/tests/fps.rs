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
