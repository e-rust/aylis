//! A escolha da placa de vídeo: como o projeto grava e como vale para o processo.

mod common;

use aylis_core::gpu::{self, GpuChoice};

#[test]
fn the_choice_is_read_back_as_the_project_saves_it() {
    for choice in [GpuChoice::Auto, GpuChoice::HighPerformance, GpuChoice::LowPower, GpuChoice::FollowMonitor, GpuChoice::Named("Radeon RX".into())] {
        assert_eq!(GpuChoice::parse(&choice.to_text()), choice);
    }
    assert_eq!(GpuChoice::parse("high-performance"), GpuChoice::HighPerformance);
    assert_eq!(GpuChoice::parse(" Economia "), GpuChoice::LowPower);
    let named = GpuChoice::parse("radeon");
    assert!(named.matches_name("AMD Radeon RX 9070 XT"));
    assert!(!named.matches_name("Intel(R) UHD Graphics"));
    assert!(!GpuChoice::HighPerformance.matches_name("AMD Radeon"));

    // Gravada pela plataforma; o AYLIS_GPU (testes) passa na frente.
    if std::env::var_os("AYLIS_GPU").is_none() {
        gpu::set_choice(GpuChoice::LowPower);
        assert_eq!(gpu::choice(), GpuChoice::LowPower);
        gpu::set_choice(GpuChoice::Auto);
    }
}

#[test]
fn a_form_choice_overrides_the_project_one() {
    assert_eq!(GpuChoice::parse("follow-monitor"), GpuChoice::FollowMonitor);
    assert_eq!(gpu::choice_for("monitor"), GpuChoice::FollowMonitor);
    assert_eq!(gpu::choice_for("economia"), GpuChoice::LowPower);
    // Vazia: a do projeto.
    assert_eq!(gpu::choice_for("  "), gpu::choice());
}

#[test]
fn the_form_reports_a_change_of_adapter() {
    use aylis_core::Ui;
    let mut ui = Ui::new();
    let form = ui.add_root(common::boxy(100.0, 100.0));
    assert_eq!(ui.form_gpu(form), None);
    ui.set_form_gpu(form, Some("Intel(R) UHD Graphics".into()));
    assert_eq!(ui.form_gpu(form), Some("Intel(R) UHD Graphics"));
    ui.set_form_gpu(form, Some("AMD Radeon RX 9070 XT".into()));
    assert_eq!(ui.form_gpu(form), Some("AMD Radeon RX 9070 XT"));
    gpu::set_adapter_list(|| vec!["A".into(), "B".into()]);
    assert_eq!(gpu::adapters().len(), 2);
}
