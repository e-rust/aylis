//! Desenho de verdade na GPU (pula se a máquina não tiver uma).

use glam::{Mat4, Vec3};
use aylis_3d::{Engine, Frame, Model, Output, sample};

fn frame(model: &Model, w: u32, h: u32, background: [f32; 4]) -> Frame {
    let (lo, hi) = model.bounds;
    let center = (lo + hi) / 2.0;
    let radius = (hi - lo).length() / 2.0;
    let eye = center + Vec3::new(0.0, 0.3, 1.0).normalize() * radius * 3.0;
    Frame {
        width: w,
        height: h,
        view: glam::camera::rh::view::look_at_mat4(eye, center, Vec3::Y),
        proj: glam::camera::rh::proj::directx::perspective(40f32.to_radians(), w as f32 / h as f32, 0.05, 100.0),
        eye,
        background,
        light_dir: Vec3::new(-0.4, -1.0, -0.6),
        file_lights: true,
        shadows: true,
        ray_traced: false,
        sample: 0,
        glow: 0.0,
    }
}

fn pixels(engine: &mut Engine, model: &Model, clip_time: Option<f32>, background: [f32; 4]) -> (u32, u32, Vec<u8>) {
    let mut gpu = engine.upload(model);
    let mut view = engine.view();
    let mut pose = model.rest_pose();
    if let Some(t) = clip_time {
        model.sample(0, t, &mut pose);
    }
    let world = model.world(&pose, Mat4::IDENTITY);
    match engine.render(model, &mut gpu, &mut view, &world, &pose.weights, &frame(model, 160, 120, background), false) {
        Some(Output::Pixels { width, height, rgba }) => (width, height, rgba),
        _ => panic!("sem pixels"),
    }
}

#[test]
fn draws_the_model_with_transparent_background() {
    let Ok(mut engine) = Engine::new(None) else {
        eprintln!("sem GPU: teste pulado");
        return;
    };
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let (w, h, rgba) = pixels(&mut engine, &model, None, [0.0; 4]);
    assert_eq!((w, h), (160, 120));
    let at = |x: u32, y: u32| {
        let i = ((y * w + x) * 4) as usize;
        [rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]
    };
    assert_eq!(at(2, 2)[3], 0, "o canto é transparente");
    // Em algum lugar do meio, o tubo laranja e opaco.
    let orange = (0..w).flat_map(|x| (0..h).map(move |y| (x, y))).filter(|&(x, y)| {
        let p = at(x, y);
        p[3] == 255 && p[0] > p[1] && p[1] > p[2]
    });
    assert!(orange.count() > 200, "o tubo aparece");
}

/// Com o brilho, a luz que o material emite espalha em volta dele: o fundo transparente junto
/// do tubo acende (e fica transparente sem o brilho, ou com um material que não emite).
#[test]
fn glow_spreads_emitted_light_around_the_model() {
    let Ok(mut engine) = Engine::new(None) else {
        eprintln!("sem GPU: teste pulado");
        return;
    };
    let mut model = Model::from_glb(&sample::tube_glb()).unwrap();
    // Só o tubo (o material 0) emite; o chão não.
    model.materials[0].emissive = [3.0, 1.2, 0.2];
    // Grande o bastante para o brilho (que cresce com a imagem) caber em volta.
    const W: i32 = 320;
    const H: i32 = 240;
    let render = |engine: &mut Engine, model: &Model, glow: f32| {
        let mut gpu = engine.upload(model);
        let mut view = engine.view();
        let pose = model.rest_pose();
        let world = model.world(&pose, Mat4::IDENTITY);
        let f = Frame { glow, ..frame(model, W as u32, H as u32, [0.0; 4]) };
        match engine.render(model, &mut gpu, &mut view, &world, &pose.weights, &f, false) {
            Some(Output::Pixels { rgba, .. }) => rgba,
            _ => panic!("sem pixels"),
        }
    };
    let plain = render(&mut engine, &model, 0.0);
    let glowing = render(&mut engine, &model, 1.5);
    // Os pixels de fundo (transparentes sem o brilho) que ficam perto do tubo acendem.
    let background: Vec<usize> = (0..plain.len() / 4).filter(|&p| plain[p * 4 + 3] == 0).collect();
    let lit = background.iter().filter(|&&p| glowing[p * 4 + 3] > 8 && glowing[p * 4] > glowing[p * 4 + 2]).count();
    assert!(lit > 1200, "o brilho acende o fundo em volta ({lit} pixels)");
    // Longe do modelo (a mais de um quarto da largura de qualquer parte dele), o fundo
    // continua transparente: o brilho cai rápido (o alcance acompanha o tamanho da imagem).
    let (w, far_from) = (W, W / 4);
    let opaque: Vec<(i32, i32)> = (0..plain.len() / 4).filter(|&p| plain[p * 4 + 3] > 0).map(|p| (p as i32 % w, p as i32 / w)).collect();
    let far: Vec<usize> = background
        .iter()
        .copied()
        .filter(|&p| {
            let (x, y) = (p as i32 % w, p as i32 / w);
            opaque.iter().all(|&(ox, oy)| (ox - x).pow(2) + (oy - y).pow(2) > far_from * far_from)
        })
        .collect();
    let brightest = far.iter().map(|&p| glowing[p * 4 + 3]).max().unwrap_or(0);
    assert!(!far.is_empty() && brightest < 8, "longe do modelo o fundo continua transparente ({} pixels longe, alfa até {brightest})", far.len());

    // Sem emissão, o brilho não acende nada.
    for m in &mut model.materials {
        m.emissive = [0.0; 3];
    }
    let dark = render(&mut engine, &model, 1.5);
    let lit = background.iter().filter(|&&p| dark[p * 4 + 3] > 8).count();
    assert!(lit < 20, "sem emissão, sem brilho ({lit} pixels)");
}

#[test]
fn animation_changes_the_image() {
    let Ok(mut engine) = Engine::new(None) else { return };
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let (_, _, a) = pixels(&mut engine, &model, Some(0.0), [1.0, 1.0, 1.0, 1.0]);
    let (_, _, b) = pixels(&mut engine, &model, Some(0.5), [1.0, 1.0, 1.0, 1.0]);
    let diff = a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, q)| p != q).count();
    assert!(diff > 100, "o balanço mexe o tubo ({diff} pixels)");
}

#[cfg(windows)]
#[test]
fn shares_the_frame_without_copy_on_direct3d12() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::DX12, ..wgpu::InstanceDescriptor::new_without_display_handle() });
    let Ok(adapter) = pollster::block_on(instance.request_adapter(&Default::default())) else { return };
    let mut engine = Engine::with_adapter(adapter).unwrap();
    assert!(engine.can_share(), "Direct3D 12: textura e cercas compartilhadas");
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let mut gpu = engine.upload(&model);
    let mut view = engine.view();
    let pose = model.rest_pose();
    let world = model.world(&pose, Mat4::IDENTITY);
    let f = frame(&model, 64, 48, [0.0; 4]);
    let mut ready = Vec::new();
    for _ in 0..3 {
        match engine.render(&model, &mut gpu, &mut view, &world, &pose.weights, &f, true) {
            Some(Output::Surface(aylis_core::GpuSurface::D3D12 { id, texture, width, height, ready_value, release_value, .. })) => {
                assert_ne!(texture, 0);
                assert_eq!((width, height), (64, 48));
                assert_eq!(ready_value, release_value);
                ready.push((id, ready_value));
                // Sem o Direct2D do outro lado: a CPU faz o papel dele e libera.
                engine.release_on_cpu(release_value);
            }
            _ => panic!("esperava a textura compartilhada"),
        }
    }
    // Duas texturas se alternando, e a cerca sempre subindo.
    assert_eq!(ready[0].0, ready[2].0);
    assert_ne!(ready[0].0, ready[1].0);
    assert!(ready[0].1 < ready[1].1 && ready[1].1 < ready[2].1);
}

#[cfg(windows)]
#[test]
fn never_waits_on_the_gpu_while_a_texture_is_free() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::DX12, ..wgpu::InstanceDescriptor::new_without_display_handle() });
    let Ok(adapter) = pollster::block_on(instance.request_adapter(&Default::default())) else { return };
    let mut engine = Engine::with_adapter(adapter).unwrap();
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let mut gpu = engine.upload(&model);
    let mut view = engine.view();
    let pose = model.rest_pose();
    let world = model.world(&pose, Mat4::IDENTITY);
    let f = frame(&model, 32, 24, [0.0; 4]);
    // Ninguém libera: as quatro primeiras saem em texturas diferentes, sem espera na GPU.
    let mut ids = Vec::new();
    for _ in 0..4 {
        if let Some(Output::Surface(aylis_core::GpuSurface::D3D12 { id, .. })) = engine.render(&model, &mut gpu, &mut view, &world, &pose.weights, &f, true) {
            ids.push(id);
        }
    }
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 4, "quatro texturas, nenhuma reaproveitada antes de liberada");
    // Libera tudo: o próximo quadro volta para uma das existentes.
    engine.release_on_cpu(4);
    let Some(Output::Surface(aylis_core::GpuSurface::D3D12 { id, .. })) = engine.render(&model, &mut gpu, &mut view, &world, &pose.weights, &f, true) else { panic!() };
    assert!(ids.contains(&id));
}

#[test]
fn the_engine_opens_the_gpu_chosen_by_name() {
    let adapters = aylis_3d::adapters();
    eprintln!("placas: {adapters:?}");
    for name in &adapters {
        aylis_core::gpu::set_choice(aylis_core::gpu::GpuChoice::Named(name.clone()));
        match Engine::new(None) {
            Ok(engine) => assert_eq!(&engine.info.name, name),
            Err(e) => eprintln!("{name}: {e}"),
        }
    }
    aylis_core::gpu::set_choice(aylis_core::gpu::GpuChoice::Auto);
}
