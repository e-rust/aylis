//! O suporte a traçado de raios das placas daqui (só informa; não falha sem suporte).

#[test]
fn lists_ray_tracing_support() {
    for s in aylis_3d::ray_tracing_support() {
        println!("{} ({}): ray query {}, pipelines {}", s.adapter, s.backend, s.ray_query, s.pipelines);
    }
}

use glam::{Mat4, Vec3};
use aylis_3d::{Engine, Frame, Model, Output, sample};

fn frame(model: &Model, ray_traced: bool, sample: u32) -> Frame {
    let (lo, hi) = model.bounds;
    let center = (lo + hi) / 2.0;
    let radius = (hi - lo).length() / 2.0;
    let eye = center + Vec3::new(0.3, 1.0, 0.5).normalize() * radius * 2.4;
    Frame {
        width: 160,
        height: 160,
        view: glam::camera::rh::view::look_at_mat4(eye, center, Vec3::Y),
        proj: glam::camera::rh::proj::directx::perspective(40f32.to_radians(), 1.0, 0.05, 100.0),
        eye,
        background: [1.0, 1.0, 1.0, 1.0],
        // A luz de lado: a sombra do tubo cai bem visível no chão.
        light_dir: Vec3::new(1.0, -0.8, 0.0),
        file_lights: false,
        shadows: false,
        ray_traced,
        sample,
        glow: 0.0,
    }
}

fn luma(engine: &mut Engine, gpu: &mut aylis_3d::GpuModel, view: &mut aylis_3d::View, model: &Model, f: &Frame) -> Vec<f32> {
    let pose = model.rest_pose();
    let world = model.world(&pose, Mat4::IDENTITY);
    match engine.render(model, gpu, view, &world, &pose.weights, f, false) {
        Some(Output::Pixels { rgba, .. }) => rgba.chunks_exact(4).map(|p| 0.3 * p[0] as f32 + 0.59 * p[1] as f32 + 0.11 * p[2] as f32).collect(),
        _ => panic!("sem pixels"),
    }
}

#[test]
fn rays_cast_shadows_without_the_shadow_map_and_accumulate() {
    let Ok(mut engine) = Engine::new(None) else {
        eprintln!("sem GPU: teste pulado");
        return;
    };
    if !engine.ray_tracing() {
        eprintln!("{}: sem traçado de raios (ray query ou o compilador DXC): teste pulado", engine.info.name);
        return;
    }
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let mut gpu = engine.upload(&model);
    let mut view = engine.view();
    // Sem sombra do mapa (shadows = false): só os raios escurecem o chão.
    let plain = luma(&mut engine, &mut gpu, &mut view, &model, &frame(&model, false, 0));
    let traced = luma(&mut engine, &mut gpu, &mut view, &model, &frame(&model, true, 0));
    let darker = plain.iter().zip(&traced).filter(|(p, t)| **t + 20.0 < **p).count();
    assert!(darker > 150, "os raios fazem sombra ({darker} pixels mais escuros)");
    // Acumulando: as amostras se juntam numa média que muda pouco de uma para a outra.
    let mut last = Vec::new();
    for s in 1..=8 {
        last = luma(&mut engine, &mut gpu, &mut view, &model, &frame(&model, true, s));
    }
    let next = luma(&mut engine, &mut gpu, &mut view, &model, &frame(&model, true, 9));
    let change = last.iter().zip(&next).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    assert!(change < 40.0, "a nona amostra muda no máximo pouco a média ({change})");
    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;
    assert!((mean(&next) - mean(&traced)).abs() < 25.0, "a média acumulada é parecida com o quadro sem acumular");
}
