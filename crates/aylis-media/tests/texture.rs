//! O quadro de vídeo como textura (o relatório do Tabernáculo, W15): o Media Foundation
//! decodifica na placa e o quadro fica numa textura compartilhada que o Direct2D compõe
//! direto; o Skia e o navegador recebem os pixels copiados, iguais aos do caminho pela CPU.
#![cfg(windows)]

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::*;
use aylis_media::*;

fn sample() -> MediaSource {
    MediaSource::File(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.mp4"))
}

fn bgra(image: &Image) -> (u32, u32, Vec<u8>) {
    match image.source() {
        ImageSource::Bgra { width, height, pixels } => (*width, *height, pixels.clone()),
        other => panic!("esperado BGRA: {other:?}"),
    }
}

/// Diferença média por canal entre duas imagens BGRA do mesmo tamanho.
fn mean_difference(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let sum: u64 = a.chunks_exact(4).zip(b.chunks_exact(4)).map(|(p, q)| (0..3).map(|i| (p[i] as i32 - q[i] as i32).unsigned_abs() as u64).sum::<u64>()).sum();
    sum as f64 / (a.len() / 4 * 3) as f64
}

/// A placa de vídeo do canvas (a mesma em que os vídeos decodificam), ou o teste pula.
fn adapter() -> Option<GpuDevice> {
    match mf::video_adapter() {
        Some(adapter_luid) => Some(GpuDevice::Direct3D11 { adapter_luid }),
        None => {
            eprintln!("sem placa com decodificação: teste pulado");
            None
        }
    }
}

#[test]
fn frames_come_as_textures_and_match_the_pixel_path() {
    let Some(gpu) = adapter() else { return };
    let mut cpu = open_video(&sample()).unwrap();
    let mut tex = open_video(&sample()).unwrap();
    tex.set_gpu_device(Some(gpu));
    assert!(tex.max_queued().is_some(), "a fila é limitada pelas texturas");

    let a = cpu.next_frame().expect("quadro pela CPU");
    let b = tex.next_frame().expect("quadro na placa");
    assert!(a.gpu.is_none());
    let surface = b.gpu.as_ref().expect("textura").surface;
    let GpuSurface::D3D12 { texture, width, height, ready, ready_value, release, release_value, .. } = surface else { panic!("{surface:?}") };
    assert!(texture != 0 && ready != 0 && release != 0);
    assert_eq!((width, height), (320, 240));
    assert_eq!(ready_value, release_value);
    assert_eq!(b.size(), Some(SizeF::new(320.0, 240.0)));
    assert!((a.time - b.time).abs() < 1e-6);

    // A reserva (Skia, navegador): os pixels copiados da textura, iguais aos do caminho pela CPU.
    let (w, h, pa) = bgra(&a.image);
    let (w2, h2, pb) = bgra(&b.to_image().expect("cópia para a CPU"));
    assert_eq!((w, h), (w2, h2));
    let d = mean_difference(&pa, &pb);
    assert!(d < 3.0, "diferença média {d:.2}");
    // A cópia é feita uma vez por quadro.
    assert_eq!(b.to_image().unwrap().id(), b.to_image().unwrap().id());

    // Depois de uma busca, também.
    cpu.seek(1.5);
    tex.seek(1.5);
    let a = cpu.next_frame().unwrap();
    let b = tex.next_frame().unwrap();
    assert!((a.time - b.time).abs() < 1e-6);
    let d = mean_difference(&bgra(&a.image).2, &bgra(&b.to_image().unwrap()).2);
    assert!(d < 3.0, "diferença média depois da busca {d:.2}");

    // As texturas são reaproveitadas (os quadros soltos voltam ao conjunto): decodifica até o fim.
    let mut n = 1;
    let started = std::time::Instant::now();
    while let Some(f) = tex.next_frame() {
        assert!(f.gpu.is_some());
        n += 1;
    }
    assert!((70..=80).contains(&n), "quadros depois de 1,5 s: {n}");
    assert!(started.elapsed().as_secs_f64() < 5.0, "sem esperar texturas: {:?}", started.elapsed());

    // No tamanho em que aparece, e de volta aos pixels.
    tex.seek(0.0);
    tex.set_output_size(160, 120);
    let f = tex.next_frame().unwrap();
    assert_eq!(f.size(), Some(SizeF::new(160.0, 120.0)));
    tex.set_gpu_device(None);
    assert!(tex.max_queued().is_none());
    let f = tex.next_frame().unwrap();
    assert!(f.gpu.is_none(), "quadros na CPU de novo");
    assert_eq!(f.image.known_size(), Some(SizeF::new(160.0, 120.0)));
}

/// Canvas que finge compor texturas na placa `gpu` (ou não sabe, com `None`).
struct Probe {
    gpu: Option<GpuDevice>,
    surfaces: Vec<GpuSurface>,
    images: Vec<Image>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, _: &str, _: &TextStyle) -> SizeF {
        SizeF::new(0.0, 0.0)
    }
    fn draw_image(&mut self, image: &Image, _: RectF, _: RectF, _: f32) {
        self.images.push(image.clone());
    }
    fn gpu_device(&self) -> Option<GpuDevice> {
        self.gpu
    }
    fn draw_gpu_surface(&mut self, _: RectF, surface: &GpuSurface) -> bool {
        if self.gpu.is_some() {
            self.surfaces.push(*surface);
        }
        self.gpu.is_some()
    }
}

/// Pinta até `done` dizer que basta (no máximo 3 s), com o relógio da interface andando um
/// quadro por vez.
fn paint_until(ui: &mut Ui, form: ComponentId, probe: &mut Probe, done: impl Fn(&Ui, &Probe) -> bool) {
    for _ in 0..300 {
        ui.process_posted();
        ui.set_time(ui.time() + 1.0 / 30.0);
        ui.request_repaint();
        ui.paint(form, probe);
        if done(ui, probe) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn the_player_composes_textures_where_the_canvas_can_and_pixels_elsewhere() {
    let Some(gpu) = adapter() else { return };
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("v", 320.0, 240.0));
    let MediaSource::File(path) = sample() else { unreachable!() };
    let player = ui.add(VideoPlayer::new(path.to_string_lossy()), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 320.0, 240.0);
    ui.set_prop(player, "muted", true).unwrap();
    ui.set_prop(player, "looping", true).unwrap();
    ui.set_time(10.0);
    let errors = Rc::new(RefCell::new(Vec::<String>::new()));
    let er = errors.clone();
    ui.on(player, "on_error", move |_, _, args| er.borrow_mut().push(args.text().unwrap_or_default().to_string()));
    VideoPlayer::play(&mut ui, player).unwrap();

    // O Direct2D na mesma placa: o player compõe a textura, sem pixels.
    let mut d2d = Probe { gpu: Some(gpu), surfaces: Vec::new(), images: Vec::new() };
    paint_until(&mut ui, form, &mut d2d, |_, p| p.surfaces.len() >= 3);
    assert!(errors.borrow().is_empty(), "{:?}", errors.borrow());
    assert!(d2d.surfaces.len() >= 3, "texturas compostas: {}", d2d.surfaces.len());
    let frame = VideoPlayer::current_video_frame(&ui, player).expect("quadro atual");
    assert!(frame.gpu.is_some());

    // Um canvas que não sabe (o Skia, o navegador): os pixels copiados, e o decodificador
    // volta a entregar pixels.
    let mut skia = Probe { gpu: None, surfaces: Vec::new(), images: Vec::new() };
    paint_until(&mut ui, form, &mut skia, |ui, p| p.images.len() >= 3 && VideoPlayer::current_video_frame(ui, player).is_some_and(|f| f.gpu.is_none()));
    assert!(skia.surfaces.is_empty());
    assert!(skia.images.iter().all(|i| i.known_size() == Some(SizeF::new(320.0, 240.0))), "os pixels de cada quadro");
    assert!(VideoPlayer::current_video_frame(&ui, player).is_some_and(|f| f.gpu.is_none()), "o decodificador voltou aos pixels");

    // De volta ao Direct2D; e o app que pede os pixels (`current_frame`) fica com eles.
    paint_until(&mut ui, form, &mut d2d, |ui, _| VideoPlayer::current_video_frame(ui, player).is_some_and(|f| f.gpu.is_some()));
    assert!(VideoPlayer::current_video_frame(&ui, player).is_some_and(|f| f.gpu.is_some()), "texturas de novo");
    assert!(VideoPlayer::current_frame(&ui, player).is_some_and(|i| i.known_size() == Some(SizeF::new(320.0, 240.0))));
    paint_until(&mut ui, form, &mut d2d, |ui, _| VideoPlayer::current_video_frame(ui, player).is_some_and(|f| f.gpu.is_none()));
    assert!(VideoPlayer::current_video_frame(&ui, player).is_some_and(|f| f.gpu.is_none()), "o app pediu os pixels");
}
