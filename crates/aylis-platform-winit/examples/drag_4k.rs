//! Mede o arrasto num form 4K (3840 × 2160) com uma imagem de fundo do tamanho da tela: o
//! editor de print do NoOBS (Print Screen → área → anotar), item 5 do pedido de 14/11/2026.
//!
//! ```text
//! cargo run --release -p aylis-platform-winit --example drag_4k --features d2d [quadros]
//! ```
//!
//! Cenários, cada um `quadros` vezes (padrão 120), no Skia na CPU (o buffer de 4K da janela,
//! sem a apresentação) e, no Windows, no Direct2D (uma janela escondida com a swap chain da
//! placa, como no app, sem esperar o monitor):
//!
//! - `selecao`: o retângulo de seleção anda (com o escurecido em volta e as alças), desenhado
//!   num `on_paint` por cima da imagem;
//! - `traco`: um traço à mão livre cresce um ponto por quadro, no mesmo `on_paint`;
//! - `paintcanvas`: o lápis do `PaintCanvas` com uma imagem 4K, pelo mouse.
//!
//! Os dois primeiros em duas versões: `inteiro` (o app avisa com `ui.damage`, o controle todo)
//! e `parte` (`ui.damage_part`, só o que mudou: o retângulo de antes e o novo, o último pedaço
//! do traço). Para cada um: o tempo do evento, o do quadro (mediana e p95), a fração da tela
//! repintada e quantos controles o quadro pintou. A última linha, `AYLIS_DRAG4K`, é o JSON
//! que vai para `medicoes/drag_4k.json`.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use aylis_controls::{Form, ImageView, PaintCanvas};
use aylis_core::*;

const W: u32 = 3840;
const H: u32 = 2160;

/// O que o `on_paint` das anotações desenha.
#[derive(Default)]
struct Notes {
    selection: RectF,
    stroke: Vec<PointF>,
}

const STROKE_WIDTH: f32 = 6.0;
const HANDLE: f32 = 10.0;

/// O que muda quando a seleção anda de `before` para `after`: as faixas das bordas das duas
/// (o contorno, as alças e o escurecido que entra ou sai), da largura das alças mais o passo.
/// O miolo, igual nos dois quadros, não é repintado.
fn selection_edges(before: RectF, after: RectF) -> Vec<RectF> {
    let step = (after.x - before.x).abs().max((after.y - before.y).abs());
    let band = HANDLE + step;
    let mut out = Vec::new();
    for r in [before, after] {
        let outer = r.inflate(band);
        out.push(RectF::new(outer.x, outer.y, outer.width, 2.0 * band));
        out.push(RectF::new(outer.x, r.bottom() - band, outer.width, 2.0 * band));
        out.push(RectF::new(outer.x, outer.y, 2.0 * band, outer.height));
        out.push(RectF::new(r.right() - band, outer.y, 2.0 * band, outer.height));
    }
    out
}

struct Scene {
    ui: Ui,
    form: ComponentId,
    target: ComponentId,
    notes: Rc<RefCell<Notes>>,
}

/// Uma imagem de fundo 4K com detalhe em todo pixel (o print de uma tela cheia).
fn screenshot() -> Image {
    let pixels: Vec<u8> = (0..W * H)
        .flat_map(|i| {
            let (x, y) = (i % W, i / W);
            [(x / 15) as u8, (y / 9) as u8, ((x ^ y) & 0xFF) as u8, 255]
        })
        .collect();
    Image::rgba(W, H, pixels)
}

/// O form do print: a imagem de fundo e as anotações por cima (`paintcanvas`: só o
/// `PaintCanvas` com uma imagem 4K).
fn build(text: Box<dyn TextService>, paint_canvas: bool, background: &Image) -> Scene {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(text);
    let form = ui.add_root(Form::new("print", W as f32, H as f32));
    ui.show(form).unwrap();
    let notes = Rc::new(RefCell::new(Notes { selection: RectF::new(400.0, 300.0, 1200.0, 700.0), stroke: Vec::new() }));
    let target = if paint_canvas {
        let pc = ui.add(PaintCanvas::default(), form);
        ui.invoke(pc, "new", &[PropValue::Int(W as i64), PropValue::Int(H as i64)]).unwrap();
        ui.control_mut(pc).unwrap().set_position(0.0, 0.0);
        pc
    } else {
        let image = ui.add(ImageView::default(), form);
        ui.set_prop(image, "align", "client").unwrap();
        ui.get_mut::<ImageView>(image).unwrap().set_picture(Some(background.clone()));
        let panel = ui.create("Panel", Some(form)).unwrap();
        ui.set_prop(panel, "align", "client").unwrap();
        let state = notes.clone();
        ui.on_paint(panel, move |canvas, ctx, _| {
            let n = state.borrow();
            let s = n.selection;
            let (w, h) = (ctx.rect.width, ctx.rect.height);
            // O escurecido fora da seleção, o contorno e as alças.
            let dim = Brush::Solid(Color::rgba(0, 0, 0, 110));
            canvas.fill_rect(RectF::new(0.0, 0.0, w, s.y), 0.0, &dim);
            canvas.fill_rect(RectF::new(0.0, s.bottom(), w, h - s.bottom()), 0.0, &dim);
            canvas.fill_rect(RectF::new(0.0, s.y, s.x, s.height), 0.0, &dim);
            canvas.fill_rect(RectF::new(s.right(), s.y, w - s.right(), s.height), 0.0, &dim);
            canvas.stroke_rect(s, 0.0, &Stroke::new(Color::rgb(0, 120, 215), 2.0));
            for (x, y) in [(s.x, s.y), (s.right(), s.y), (s.x, s.bottom()), (s.right(), s.bottom()), (s.center().x, s.y), (s.center().x, s.bottom()), (s.x, s.center().y), (s.right(), s.center().y)] {
                canvas.fill_rect(RectF::new(x - HANDLE / 2.0, y - HANDLE / 2.0, HANDLE, HANDLE), 0.0, &Brush::Solid(Color::WHITE));
            }
            if let Some((first, rest)) = n.stroke.split_first() {
                let mut path = Path::new();
                path.move_to(first.x, first.y);
                for p in rest {
                    path.line_to(p.x, p.y);
                }
                canvas.stroke_path(&path, &Stroke::new(Color::rgb(230, 30, 30), STROKE_WIDTH).round());
            }
        });
        panel
    };
    ui.set_time(1000.0);
    ui.layout();
    Scene { ui, form, target, notes }
}

/// Onde o traço passa no quadro `i` (uma espiral larga, ~12 px por quadro).
fn stroke_point(i: usize) -> PointF {
    let t = i as f32 * 0.02;
    PointF::new(1920.0 + (300.0 + 40.0 * t) * (t * 1.3).cos(), 1080.0 + (250.0 + 30.0 * t) * (t * 1.3).sin())
}

/// Um passo do cenário: muda o desenho e avisa o `Ui` (o controle todo, ou só o que mudou).
fn step(scene: &mut Scene, kind: &str, i: usize) {
    let partial = kind.ends_with("parte");
    if kind == "paintcanvas" {
        let p = stroke_point(i);
        let event = PointerEvent { position: p, button: PointerButton::Primary, kind: PointerKind::Mouse, ..Default::default() };
        if i == 0 {
            scene.ui.pointer_down(scene.form, event);
        } else {
            scene.ui.pointer_move(scene.form, event);
        }
        return;
    }
    let changed = {
        let mut n = scene.notes.borrow_mut();
        if kind.starts_with("selecao") {
            let before = n.selection;
            // Vai e volta, 9 × 5 px por quadro (um arrasto rápido a 60 Hz).
            let dir = if (i / 60) % 2 == 0 { 1.0 } else { -1.0 };
            n.selection = before.offset(9.0 * dir, 5.0 * dir);
            selection_edges(before, n.selection)
        } else {
            let p = stroke_point(i);
            let last = n.stroke.last().copied().unwrap_or(p);
            n.stroke.push(p);
            vec![RectF::new(last.x.min(p.x), last.y.min(p.y), (last.x - p.x).abs(), (last.y - p.y).abs()).inflate(STROKE_WIDTH)]
        }
    };
    if partial {
        for rect in changed {
            scene.ui.damage_part(scene.target, rect);
        }
    } else {
        scene.ui.damage(scene.target);
    }
}

/// Um quadro: as áreas que o `Ui` pediu (ou tudo), por cima do quadro anterior; devolve o
/// tempo, a fração da tela e os controles pintados.
trait Painter {
    fn name(&self) -> &'static str;
    fn text_service(&self) -> Box<dyn TextService>;
    fn frame(&mut self, ui: &mut Ui, form: ComponentId) -> (f64, f64, usize);

    /// Quantos pixels do quadro montado por partes diferem da pintura inteira (`None`: este
    /// renderizador não confere; o Direct2D desenha na placa).
    fn verify(&mut self, _ui: &mut Ui, _form: ComponentId) -> Option<usize> {
        None
    }
}

struct Skia {
    renderer: aylis_skia::SkiaRenderer,
    pixels: Vec<u32>,
}

impl Painter for Skia {
    fn name(&self) -> &'static str {
        "Skia (CPU)"
    }

    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(self.renderer.text_service())
    }

    fn frame(&mut self, ui: &mut Ui, form: ComponentId) -> (f64, f64, usize) {
        let started = Instant::now();
        let fraction = match ui.take_frame(form).snapped(1.0) {
            FrameDamage::Parts { moves, areas } => {
                self.renderer.render_parts_to_pixels(&mut self.pixels, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
                areas.iter().map(|a| (a.width * a.height) as f64).sum::<f64>() / (W as f64 * H as f64)
            }
            _ => {
                self.renderer.render_area_to_pixels(&mut self.pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
                1.0
            }
        };
        (started.elapsed().as_secs_f64() * 1000.0, fraction.min(1.0), ui.last_paint_count())
    }

    fn verify(&mut self, ui: &mut Ui, form: ComponentId) -> Option<usize> {
        let mut full = vec![0u32; self.pixels.len()];
        self.renderer.render_area_to_pixels(&mut full, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
        // A beirada anti-aliasing cortada pela área muda alguns tons (o limite do `partial_paint`).
        let tones = |a: u32, b: u32| (0..3).map(|i| ((a >> (8 * i)) & 0xFF).abs_diff((b >> (8 * i)) & 0xFF)).max().unwrap_or(0);
        Some(self.pixels.iter().zip(&full).filter(|(a, b)| tones(**a, **b) > 48).count())
    }
}

#[cfg(windows)]
struct D2d {
    renderer: aylis_d2d::D2dRenderer,
    target: aylis_d2d::HwndTarget,
    hwnd: windows::Win32::Foundation::HWND,
}

#[cfg(windows)]
impl D2d {
    fn new() -> Self {
        use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, WINDOW_EX_STYLE, WS_POPUP};
        use windows::core::w;
        // SAFETY: uma janela escondida da classe pronta "STATIC", destruída no `Drop`.
        let hwnd = unsafe { CreateWindowExW(WINDOW_EX_STYLE(0), w!("STATIC"), w!(""), WS_POPUP, 0, 0, W as i32, H as i32, None, None, None, None) }.expect("janela");
        let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
        let target = renderer.hwnd_target(hwnd.0 as isize, W, H, false).expect("alvo da janela");
        Self { renderer, target, hwnd }
    }
}

#[cfg(windows)]
impl Drop for D2d {
    fn drop(&mut self) {
        // SAFETY: a janela criada em `new`.
        unsafe {
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyWindow(self.hwnd);
        }
    }
}

#[cfg(windows)]
impl Painter for D2d {
    fn name(&self) -> &'static str {
        "Direct2D"
    }

    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(self.renderer.text_service())
    }

    fn frame(&mut self, ui: &mut Ui, form: ComponentId) -> (f64, f64, usize) {
        let started = Instant::now();
        let (moves, areas) = match ui.take_frame(form).snapped(1.0) {
            FrameDamage::Parts { moves, areas } => (moves, areas),
            _ => (Vec::new(), Vec::new()),
        };
        let parts = (!areas.is_empty()).then_some((&moves[..], &areas[..]));
        let mut painted_whole = parts.is_none();
        self.target
            .draw_parts(&self.renderer, W, H, 1.0, Color::WHITE, parts, |c, a| match a {
                Some(a) => ui.paint_regions(form, c, a),
                None => {
                    painted_whole = true;
                    ui.paint_region(form, c, None)
                }
            })
            .expect("quadro");
        let fraction = if painted_whole { 1.0 } else { areas.iter().map(|a| (a.width * a.height) as f64).sum::<f64>() / (W as f64 * H as f64) };
        (started.elapsed().as_secs_f64() * 1000.0, fraction.min(1.0), ui.last_paint_count())
    }
}

fn percentile(values: &[f64], q: f64) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v.get(((v.len() as f64 - 1.0) * q).round() as usize).copied().unwrap_or(0.0)
}

const SCENARIOS: [&str; 5] = ["selecao_inteiro", "selecao_parte", "traco_inteiro", "traco_parte", "paintcanvas"];

fn measure(painter: &mut dyn Painter, frames: usize, background: &Image) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for kind in SCENARIOS {
        let mut scene = build(painter.text_service(), kind == "paintcanvas", background);
        // O primeiro quadro é inteiro (a janela abrindo, as imagens subindo para a placa).
        scene.ui.request_repaint();
        painter.frame(&mut scene.ui, scene.form);
        let first = painter.frame(&mut scene.ui, scene.form);
        let _ = first;
        let (mut events, mut times, mut areas, mut painted) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for i in 0..frames {
            let started = Instant::now();
            step(&mut scene, kind, i);
            events.push(started.elapsed().as_secs_f64() * 1000.0);
            let (ms, fraction, count) = painter.frame(&mut scene.ui, scene.form);
            times.push(ms);
            areas.push(fraction);
            painted.push(count as f64);
        }
        let area = areas.iter().sum::<f64>() / frames.max(1) as f64;
        // A imagem montada por partes tem de ser a da pintura inteira.
        let wrong = painter.verify(&mut scene.ui, scene.form);
        println!(
            "{:12} {:16} evento {:6.2} ms  quadro p50 {:6.2} ms  p95 {:6.2} ms  área {:7.3}%  controles {}{}",
            painter.name(),
            kind,
            percentile(&events, 0.5),
            percentile(&times, 0.5),
            percentile(&times, 0.95),
            area * 100.0,
            percentile(&painted, 0.5),
            wrong.map_or(String::new(), |n| format!("  pixels diferentes da pintura inteira: {n}"))
        );
        assert_eq!(wrong.unwrap_or(0), 0, "{kind}: a pintura por partes deixou resto");
        out.push(serde_json::json!({
            "cenario": kind,
            "evento_ms": round(percentile(&events, 0.5)),
            "quadro_p50_ms": round(percentile(&times, 0.5)),
            "quadro_p95_ms": round(percentile(&times, 0.95)),
            "area_pct": (area * 100_000.0).round() / 1000.0,
            "controles": percentile(&painted, 0.5),
        }));
    }
    out
}

fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn main() {
    let frames = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(120usize);
    if cfg!(debug_assertions) {
        println!("aviso: compilado em debug; as medidas valem em --release");
    }
    let background = screenshot();
    let mut results = serde_json::Map::new();
    let mut skia = Skia { renderer: aylis_skia::SkiaRenderer::new(), pixels: vec![0; (W * H) as usize] };
    results.insert("skia_cpu".into(), measure(&mut skia, frames, &background).into());
    #[cfg(windows)]
    {
        let mut d2d = D2d::new();
        results.insert("direct2d".into(), measure(&mut d2d, frames, &background).into());
    }
    println!("AYLIS_DRAG4K {}", serde_json::Value::Object(results));
}
