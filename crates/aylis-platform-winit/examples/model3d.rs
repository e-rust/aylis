//! Modelo 3D animado com controles do aylis por cima, e os modos de medição.
//!
//! ```text
//! cargo run --release -p aylis-platform-winit --features d2d --example model3d
//! cargo run --release -p aylis-platform-winit --features d2d --example model3d -- arquivo.glb
//! ```
//!
//! Medição (`AYLIS_BENCH=600`, como nos outros exemplos):
//! - padrão: aylis + 3D + controles, a imagem do 3D passa direto na GPU;
//! - `AYLIS_3D_COPY=1`: igual, mas copiando cada quadro do 3D pela CPU;
//! - `--no-3d`: só os controles (o 3D escondido);
//! - `--plain`: só o 3D, com o `wgpu` desenhando direto na janela, sem o aylis;
//! - `--no-shadow`: com qualquer um dos outros, desliga a sombra (para medir o custo dela).

use aylis_3d::{Model, Model3D, sample};
use aylis_controls::*;
use aylis_core::*;

const W: f32 = 1100.0;
const H: f32 = 700.0;

struct Log;

impl log::Log for Log {
    fn enabled(&self, m: &log::Metadata) -> bool {
        m.level() <= log::Level::Info
    }
    fn log(&self, r: &log::Record) {
        if self.enabled(r.metadata()) {
            eprintln!("[{}] {}: {}", r.level(), r.target(), r.args());
        }
    }
    fn flush(&self) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AYLIS_3D_LOG=1: as mensagens do wgpu (diagnóstico).
    if std::env::var("AYLIS_3D_LOG").is_ok() {
        let _ = log::set_logger(&Log).map(|_| log::set_max_level(log::LevelFilter::Info));
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bytes = match args.iter().find(|a| !a.starts_with("--")) {
        Some(path) => std::fs::read(path)?,
        None => sample::scene_glb(),
    };
    let model = Model::from_glb(&bytes)?;
    let shadows = !args.iter().any(|a| a == "--no-shadow");
    if args.iter().any(|a| a == "--plain") {
        return plain::run(model, shadows);
    }

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_3d::register(&mut ui);
    let clips: Vec<String> = model.clips.iter().map(|c| c.name.clone()).collect();
    let form = ui.add_root(Form::new("aylis — 3D com controles por cima", W, H));

    let view = ui.add(Model3D::default().with_model(model), form);
    ui.control_mut(view).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(view, "background", Color::rgb(0xE9, 0xEE, 0xF5));
    if args.iter().any(|a| a == "--no-3d") {
        ui.control_mut(view).unwrap().set_visible(false);
    }
    let _ = ui.set_prop(view, "shadows", shadows);
    let _ = ui.set_prop(view, "ray_traced", args.iter().any(|a| a == "--rays"));

    // Os controles ficam por cima do 3D (são filhos do form, depois dele).
    let bar = ui.add(Panel::default(), form);
    ui.control_mut(bar).unwrap().set_rect(24.0, H - 96.0, W - 48.0, 72.0).set_padding(14.0);
    let _ = ui.set_prop(bar, "style", "glass");
    let mut x = 0.0;
    for (i, name) in clips.iter().enumerate() {
        let b = ui.add(Button::new(name.as_str()), bar);
        ui.control_mut(b).unwrap().set_rect(x, 0.0, 120.0, 44.0);
        if i > 0 {
            let _ = ui.set_prop(b, "style", "tonal");
        }
        let name = name.clone();
        ui.on_click(b, move |ui, _| {
            let _ = ui.set_prop(view, "animation", name.clone());
        });
        x += 130.0;
    }
    let speed = ui.add(TrackBar::default(), bar);
    ui.control_mut(speed).unwrap().set_rect(x + 20.0, 8.0, 220.0, 28.0);
    let _ = ui.set_prop(speed, "min", 0.0);
    let _ = ui.set_prop(speed, "max", 3.0);
    let _ = ui.set_prop(speed, "value", 1.0);
    ui.on(speed, "on_change", move |ui, sender, _| {
        let v = ui.get_prop(sender, "value").ok().and_then(|v| v.to_f64().ok()).unwrap_or(1.0);
        let _ = ui.set_prop(view, "speed", v);
    });
    let spin = ui.add(CheckBox::new("Girar o modelo"), bar);
    ui.control_mut(spin).unwrap().set_rect(x + 270.0, 8.0, 170.0, 28.0);
    ui.on(spin, "on_change", move |ui, _, args| {
        let on = matches!(args, EventArgs::Value(PropValue::Bool(true)));
        let _ = ui.set_prop(view, "auto_rotate", if on { 30.0 } else { 0.0 });
    });
    let rays = ui.add(CheckBox::new("Traçado de raios"), bar);
    ui.control_mut(rays).unwrap().set_rect(x + 450.0, 8.0, 180.0, 28.0);
    let _ = ui.set_prop(rays, "checked", args.iter().any(|a| a == "--rays"));
    ui.on(rays, "on_change", move |ui, _, args| {
        let on = matches!(args, EventArgs::Value(PropValue::Bool(true)));
        let _ = ui.set_prop(view, "ray_traced", on);
    });

    let card = ui.add(Panel::default(), form);
    ui.control_mut(card).unwrap().set_rect(24.0, 24.0, 300.0, 120.0).set_padding(16.0);
    let _ = ui.set_prop(card, "style", "card");
    let title = ui.add(Label::new("Modelo 3D"), card);
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(28.0);
    let _ = ui.set_prop(title, "font_size", 18);
    let hint = ui.add(Label::new("Arraste para girar a câmera, roda para aproximar. Os botões trocam o clipe com transição."), card);
    ui.control_mut(hint).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(hint, "word_wrap", true);
    let combo = ui.add(ComboBox::new(&clips.iter().map(String::as_str).collect::<Vec<_>>()), form);
    ui.control_mut(combo).unwrap().set_rect(W - 244.0, 24.0, 220.0, 36.0);
    ui.on(combo, "on_change", move |ui, _, args| {
        if let EventArgs::Text(t) = args {
            let _ = ui.set_prop(view, "animation", t.clone());
        }
    });

    aylis_platform_winit::run(ui, form)?;
    Ok(())
}

/// Só o 3D: `wgpu` desenhando direto na janela (a referência de desempenho do 3D).
mod plain {
    use std::sync::Arc;
    use std::time::Instant;

    use glam::{Mat4, Vec3};
    use aylis_3d::{Engine, Frame, Model};
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoop};
    use winit::window::{Window, WindowId};

    struct App {
        model: Model,
        shadows: bool,
        state: Option<State>,
        bench: Option<(usize, Vec<f64>)>,
        start: Instant,
    }

    struct State {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        config: wgpu::SurfaceConfiguration,
        gpu: aylis_3d::GpuModel,
        view: aylis_3d::View,
        // Por último: o dispositivo sai depois dos recursos dele.
        engine: Engine,
    }

    pub fn run(model: Model, shadows: bool) -> Result<(), Box<dyn std::error::Error>> {
        let bench = std::env::var("AYLIS_BENCH").ok().and_then(|v| v.parse().ok()).map(|n: usize| (n, Vec::new()));
        let event_loop = EventLoop::new()?;
        let mut app = App { model, shadows, state: None, bench, start: Instant::now() };
        event_loop.run_app(&mut app)?;
        Ok(())
    }

    impl ApplicationHandler for App {
        fn resumed(&mut self, el: &ActiveEventLoop) {
            if self.state.is_some() {
                return;
            }
            let attrs = Window::default_attributes().with_title("aylis — 3D puro (wgpu)").with_inner_size(winit::dpi::LogicalSize::new(super::W, super::H));
            let window = Arc::new(el.create_window(attrs).unwrap());
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::DX12, ..wgpu::InstanceDescriptor::new_without_display_handle() });
            let surface = instance.create_surface(window.clone()).unwrap();
            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { compatible_surface: Some(&surface), ..Default::default() })).unwrap();
            let engine = Engine::with_adapter(adapter.clone()).unwrap();
            let size = window.inner_size();
            let bench = self.bench.is_some();
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: size.width.max(1),
                height: size.height.max(1),
                present_mode: if bench { wgpu::PresentMode::Immediate } else { wgpu::PresentMode::Fifo },
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                color_space: Default::default(),
            };
            surface.configure(&engine.device, &config);
            let gpu = engine.upload(&self.model);
            let view = engine.view();
            self.state = Some(State { window, surface, config, engine, gpu, view });
            self.start = Instant::now();
        }

        fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
            let Some(st) = self.state.as_mut() else { return };
            match event {
                WindowEvent::CloseRequested => el.exit(),
                WindowEvent::Resized(size) => {
                    st.config.width = size.width.max(1);
                    st.config.height = size.height.max(1);
                    st.surface.configure(&st.engine.device, &st.config);
                }
                WindowEvent::RedrawRequested => {
                    let t0 = Instant::now();
                    let frame = match st.surface.get_current_texture() {
                        wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
                        _ => {
                            st.window.request_redraw();
                            return;
                        }
                    };
                    let target = frame.texture.create_view(&Default::default());
                    // A mesma câmera e animação do Model3D (yaw 30°, pitch 15°).
                    let now = self.start.elapsed().as_secs_f32();
                    let mut pose = self.model.rest_pose();
                    if !self.model.clips.is_empty() {
                        self.model.sample(0, now % self.model.clips[0].duration.max(1e-6), &mut pose);
                    }
                    let (lo, hi) = self.model.bounds;
                    let center = (lo + hi) / 2.0;
                    let radius = ((hi - lo).length() / 2.0).max(1e-3);
                    let fov = 40f32.to_radians();
                    let distance = radius / (fov / 2.0).sin() * 1.05;
                    let (yaw, pitch) = (30f32.to_radians(), 15f32.to_radians());
                    let eye = center + Vec3::new(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos()) * distance;
                    let (w, h) = (st.config.width, st.config.height);
                    let f = Frame {
                        width: w,
                        height: h,
                        view: glam::camera::rh::view::look_at_mat4(eye, center, Vec3::Y),
                        proj: glam::camera::rh::proj::directx::perspective(fov, w as f32 / h as f32, distance * 0.01, distance * 10.0 + radius * 4.0),
                        eye,
                        background: [0.914, 0.933, 0.961, 1.0],
                        light_dir: Vec3::new(-0.4, -1.0, -0.6),
                        file_lights: true,
                        shadows: self.shadows,
                        ray_traced: false,
                        sample: 0,
                        glow: 0.0,
                    };
                    let world = self.model.world(&pose, Mat4::IDENTITY);
                    st.engine.render_to(&self.model, &mut st.gpu, &mut st.view, &world, &pose.weights, &f, &target);
                    st.engine.queue.present(frame);
                    if let Some((n, times)) = self.bench.as_mut() {
                        times.push(t0.elapsed().as_secs_f64() * 1000.0);
                        if times.len() >= *n + 10 {
                            let mut t: Vec<f64> = times[10..].to_vec();
                            t.sort_by(|a, b| a.total_cmp(b));
                            let avg = t.iter().sum::<f64>() / t.len() as f64;
                            let at = |q: f64| t[((t.len() as f64 - 1.0) * q).round() as usize];
                            let (private, working) = aylis_platform_winit::process_memory();
                            println!(
                                "AYLIS_BENCH renderizador=wgpu-puro quadros={} media_ms={avg:.3} p50_ms={:.3} p95_ms={:.3} fps={:.0} memoria_privada_mb={:.1} working_set_mb={:.1}",
                                t.len(),
                                at(0.5),
                                at(0.95),
                                1000.0 / avg,
                                private as f64 / 1048576.0,
                                working as f64 / 1048576.0
                            );
                            el.exit();
                            return;
                        }
                    }
                    st.window.request_redraw();
                }
                _ => {}
            }
        }
    }
}
