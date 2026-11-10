//! A escolha da placa de vídeo: como o projeto grava e como vale para o processo.

use aylis_core::gpu::{self, GpuChoice};

#[test]
fn the_choice_is_read_back_as_the_project_saves_it() {
    for choice in [GpuChoice::Auto, GpuChoice::HighPerformance, GpuChoice::LowPower, GpuChoice::Named("Radeon RX".into())] {
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
