//! Um form em vários monitores (`span_monitors`), sem precisar de vários monitores: cada janela
//! pinta o seu pedaço com o deslocamento dele, e o pedaço tem de sair igual ao recorte da
//! pintura do form inteiro (inteiro e por parte, no Skia e, no Windows, no Direct2D, com escala
//! inteira e quebrada); a entrada de uma janela chega ao controle certo do form; e a lista de
//! um ComboBox abre dentro do pedaço em que nasce.

use aylis_core::span::{self, Piece, ScreenRect};
use aylis_core::*;

/// A tela do teste, em pixels: o form ocupa três "monitores" (um à esquerda, alto, e dois à
/// direita, um em cima do outro).
const W: u32 = 1280;
const H: u32 = 960;
const MONITORS: [ScreenRect; 3] = [ScreenRect::new(0, 0, 500, 960), ScreenRect::new(500, 0, 780, 400), ScreenRect::new(500, 400, 780, 560)];
const CELL: (f32, f32) = (150.0, 70.0);
/// Como no `partial_paint`: a beirada anti-aliasing de um desenho cortado pela borda da
/// superfície muda alguns tons; resto de verdade muda o pixel inteiro (veja
/// [`Painter::tolerance`]).
const TOLERANCE: u32 = 48;

/// Um renderizador: pinta numa superfície `w`×`h` (pixels `0x00RRGGBB`), inteira ou só em
/// `area` por cima de `previous`.
trait Painter {
    fn text_service(&self) -> Box<dyn TextService>;
    /// Quanto um pixel do pedaço pode mudar sem contar como resto. No Skia, um desenho cortado
    /// pela borda da superfície passa por outro caminho de rasterização (as curvas cortadas
    /// viram retas), como na pintura por parte, só que aqui sem a margem de 16 px (a borda é a
    /// da janela), e o caminho inteiro muda, não só junto da borda. Medido: um arco quase
    /// deitado cortado pela divisa muda até 65 tons, em pixels até 32 px dela (32 pixels em
    /// 104 controles; o Manometer a 150%, 49 tons). É a divisa entre dois monitores, onde já há
    /// a moldura deles; resto de verdade muda o pixel inteiro (mais de 200 tons). O Direct2D
    /// sai igual até a borda, com a tolerância do `partial_paint`.
    fn tolerance(&self) -> u32 {
        TOLERANCE
    }
    fn render(&self, w: u32, h: u32, scale: f32, over: Option<(&[u32], RectF)>, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Vec<u32>;
}

impl Painter for aylis_skia::SkiaRenderer {
    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_skia::SkiaRenderer::text_service(self))
    }

    fn tolerance(&self) -> u32 {
        96
    }

    fn render(&self, w: u32, h: u32, scale: f32, over: Option<(&[u32], RectF)>, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Vec<u32> {
        let mut pixels = over.map_or_else(|| vec![0u32; (w * h) as usize], |(p, _)| p.to_vec());
        self.render_area_to_pixels(&mut pixels, w, h, scale, Color::WHITE, over.map(|(_, a)| a), |c| paint(c));
        pixels
    }
}

#[cfg(windows)]
impl Painter for aylis_d2d::D2dRenderer {
    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_d2d::D2dRenderer::text_service(self))
    }

    fn render(&self, w: u32, h: u32, scale: f32, over: Option<(&[u32], RectF)>, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Vec<u32> {
        let previous: Option<Vec<u8>> = over.map(|(p, _)| p.iter().flat_map(|&x| [(x >> 16) as u8, (x >> 8) as u8, x as u8, 255]).collect());
        let rgba = self.render_area_rgba(w, h, scale, Color::WHITE, previous.as_deref().zip(over.map(|(_, a)| a)), |c| paint(c)).expect("Direct2D");
        rgba.chunks_exact(4).map(|p| (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32).collect()
    }
}

/// O recorte de `piece` (pixels da tela) numa imagem da tela inteira.
fn crop(full: &[u32], piece: &Piece) -> Vec<u32> {
    let s = piece.screen;
    (0..s.height).flat_map(|y| {
        let row = (s.y as u32 + y) * W + s.x as u32;
        full[row as usize..(row + s.width) as usize].iter().copied()
    }).collect()
}

/// Quantos pixels mudam mais que `tolerance`, e o primeiro.
fn differences(a: &[u32], b: &[u32], width: u32, tolerance: u32) -> (usize, Option<(u32, u32)>) {
    let tones = |a: u32, b: u32| (0..3).map(|s| ((a >> (s * 8)) & 0xFF).abs_diff((b >> (s * 8)) & 0xFF)).max().unwrap_or(0);
    let mut first = None;
    let count = a.iter().zip(b).enumerate().filter(|(_, (x, y))| tones(**x, **y) > tolerance).inspect(|(i, _)| {
        first.get_or_insert(((*i as u32) % width, (*i as u32) / width));
    }).count();
    (count, first)
}

/// Um form com um controle de cada classe visual numa grade (como no `partial_paint`).
fn screen(painter: &dyn Painter, scale: f32) -> (Ui, ComponentId, Vec<(&'static str, ComponentId)>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(painter.text_service());
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", (W as f32 / scale) as f64).unwrap();
    ui.set_prop(form, "height", (H as f32 / scale) as f64).unwrap();
    ui.show(form).unwrap();
    let classes: Vec<&'static str> = ui.registry().iter().map(|c| c.name).filter(|n| *n != "Form").collect();
    let mut controls = Vec::new();
    for class in classes {
        let Ok(id) = ui.create(class, Some(form)) else { continue };
        if ui.control(id).is_none() {
            continue;
        }
        let n = controls.len();
        let (col, row) = (n % 8, n / 8);
        if row >= 13 {
            ui.free(id);
            continue;
        }
        let _ = ui.set_prop(id, "align", "none");
        let _ = ui.set_prop(id, "left", 4.0 + col as f64 * CELL.0 as f64);
        let _ = ui.set_prop(id, "top", 4.0 + row as f64 * CELL.1 as f64);
        let _ = ui.set_prop(id, "width", (CELL.0 - 12.0) as f64);
        let _ = ui.set_prop(id, "height", (CELL.1 - 12.0) as f64);
        controls.push((class, id));
    }
    ui.set_time(1000.0);
    ui.layout();
    (ui, form, controls)
}

/// Pinta cada pedaço como a plataforma pinta (`Ui::paint_piece`, `None`: o pedaço inteiro)
/// e confere com o recorte da pintura inteira; depois muda os controles (o mouse em cima, um
/// deslocamento) e pinta só o que mudou em cada pedaço, por cima do pedaço anterior.
fn compare(painter: Box<dyn Painter>, name: &str, scale: f32) {
    let (mut ui, form, controls) = screen(&*painter, scale);
    let pieces = span::split(ScreenRect::new(0, 0, W, H), &MONITORS);
    assert_eq!(pieces.len(), 3);
    ui.set_form_pieces(form, pieces.iter().map(|p| p.rect_in_form(scale as f64)).collect());
    let paint_full = |ui: &mut Ui| painter.render(W, H, scale, None, &mut |c| ui.paint(form, c));
    let paint_piece = |ui: &mut Ui, piece: &Piece, over: Option<(&[u32], RectF)>, first: bool| {
        let rect = piece.rect_in_form(scale as f64);
        let areas: Option<Vec<RectF>> = over.map(|(_, a)| vec![a]);
        painter.render(piece.screen.width, piece.screen.height, scale, over, &mut |c| ui.paint_piece(form, c, rect, areas.as_deref(), first))
    };

    let mut failures = Vec::new();
    let full = paint_full(&mut ui);
    let mut images: Vec<Vec<u32>> = Vec::new();
    for (k, piece) in pieces.iter().enumerate() {
        let image = paint_piece(&mut ui, piece, None, k == 0);
        let (count, first) = differences(&image, &crop(&full, piece), piece.screen.width, painter.tolerance());
        if count > 0 {
            failures.push(format!("pedaço {k} inteiro: {count} pixels diferentes, o primeiro em {first:?}"));
        }
        images.push(image);
    }
    let _ = ui.take_damage(form);

    let mut compared = 0;
    for &(class, id) in &controls {
        if !ui.contains(id) {
            continue;
        }
        let Some(r) = ui.absolute_rect(id) else { continue };
        let center = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        for step in ["hover", "slide", "back"] {
            match step {
                "hover" => ui.pointer_move(form, PointerEvent { position: center, ..Default::default() }),
                _ => {
                    let shift = if step == "slide" { PointF::new(15.0, 10.0) } else { PointF::ZERO };
                    ui.control_mut(id).unwrap().set_translate(shift);
                    ui.damage(id);
                }
            }
            // O que mudou (um retângulo, nas coordenadas do form) e, em cada pedaço, a parte
            // dele, nas coordenadas da janela (como a plataforma passa ao renderizador).
            let area = ui.take_damage(form);
            let full = paint_full(&mut ui);
            let _ = ui.take_damage(form);
            let mut first = true;
            for (k, piece) in pieces.iter().enumerate() {
                let rect = piece.rect_in_form(scale as f64);
                let image = match area {
                    None => paint_piece(&mut ui, piece, None, std::mem::replace(&mut first, false)),
                    Some(a) => match a.intersect(rect) {
                        // Nada mudou neste pedaço: a janela fica como estava.
                        None => images[k].clone(),
                        Some(part) => {
                            compared += 1;
                            let local = part.offset(-rect.x, -rect.y);
                            paint_piece(&mut ui, piece, Some((&images[k], local)), std::mem::replace(&mut first, false))
                        }
                    },
                };
                let expected = crop(&full, piece);
                let (count, at) = differences(&image, &expected, piece.screen.width, painter.tolerance());
                if count > 0 {
                    let worst = image.iter().zip(&expected).map(|(a, b)| (0..3).map(|s| ((a >> (s * 8)) & 0xFF).abs_diff((b >> (s * 8)) & 0xFF)).max().unwrap_or(0)).max().unwrap_or(0);
                    failures.push(format!("{class} ({step}), pedaço {k}: {count} pixels diferentes, o primeiro em {at:?}, até {worst} tons (área {area:?})"));
                    // Para investigar: AYLIS_SPAN_DUMP=pasta grava o pedaço e o recorte esperado
                    // da primeira falha.
                    if let (Ok(dir), 1) = (std::env::var("AYLIS_SPAN_DUMP"), failures.len()) {
                        let png = |p: &[u32]| {
                            let rgba: Vec<u8> = p.iter().flat_map(|&x| [(x >> 16) as u8, (x >> 8) as u8, x as u8, 255]).collect();
                            aylis_skia::encode_png(&rgba, piece.screen.width, piece.screen.height).unwrap()
                        };
                        let _ = std::fs::create_dir_all(&dir);
                        let _ = std::fs::write(format!("{dir}/pedaco.png"), png(&image));
                        let _ = std::fs::write(format!("{dir}/esperado.png"), png(&expected));
                    }
                }
                images[k] = image;
            }
        }
    }
    eprintln!("{name} (escala {scale}): {compared} pedaços pintados por parte, {} controles", controls.len());
    assert!(compared > controls.len() / 2, "poucos pedaços por parte ({compared}): o teste não está exercitando a pintura por área");
    assert!(failures.is_empty(), "{name} (escala {scale}): o pedaço não saiu igual ao recorte do form inteiro:\n{}", failures.join("\n"));
}

#[test]
fn each_piece_paints_the_same_as_the_whole_form_in_skia() {
    compare(Box::new(aylis_skia::SkiaRenderer::new()), "Skia", 1.0);
    compare(Box::new(aylis_skia::SkiaRenderer::new()), "Skia", 1.5);
}

#[cfg(windows)]
#[test]
fn each_piece_paints_the_same_as_the_whole_form_in_direct2d() {
    compare(Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D", 1.0);
    compare(Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D", 1.5);
}

/// O form de 1000 × 500 numa tela 1,5 com os dois monitores do lado a lado: um botão no
/// pedaço da direita recebe o clique dado na janela da direita.
#[test]
fn the_input_of_a_piece_reaches_the_right_control() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", 1000.0).unwrap();
    ui.set_prop(form, "height", 500.0).unwrap();
    ui.show(form).unwrap();
    let button = ui.create("Button", Some(form)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 700.0.into()), ("top", 100.0.into()), ("width", 120.0.into()), ("height", 40.0.into())] {
        ui.set_prop(button, prop, value).unwrap();
    }
    let clicks = std::rc::Rc::new(std::cell::Cell::new(0));
    let counter = clicks.clone();
    ui.on(button, "on_click", move |_, _, _| counter.set(counter.get() + 1));
    ui.layout();
    // Na tela, o form começa em x = 1500 e o segundo monitor em 1920.
    let scale = 1.5;
    let pieces = span::split(ScreenRect::new(1500, 0, 1500, 750), &[ScreenRect::new(0, 0, 1920, 1080), ScreenRect::new(1920, 0, 2560, 1440)]);
    let right = pieces[1];
    // O botão vai de x = 700 a 820 no form: 1050 a 1230 px, ou 630 a 810 na janela da direita
    // (que começa em 420 px do form).
    let at = right.to_form(700.0, 180.0, scale);
    assert!(at.x > 700.0 && at.x < 820.0, "{at:?}");
    let event = |position| PointerEvent { position, button: PointerButton::Primary, ..Default::default() };
    ui.pointer_move(form, event(at));
    ui.pointer_down(form, event(at));
    ui.pointer_up(form, event(at));
    assert_eq!(clicks.get(), 1, "o clique na janela da direita chega ao botão");
    // O mesmo ponto na janela da esquerda (sem a origem) cai no fundo do form.
    let wrong = pieces[0].to_form(700.0, 180.0, scale);
    ui.pointer_down(form, event(wrong));
    ui.pointer_up(form, event(wrong));
    assert_eq!(clicks.get(), 1);
}

/// A lista de um ComboBox no fim do monitor de cima abre para cima, dentro dele, e não
/// partida entre ele e o de baixo.
#[test]
fn a_combo_list_opens_inside_its_piece() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", 800.0).unwrap();
    ui.set_prop(form, "height", 900.0).unwrap();
    ui.show(form).unwrap();
    let combo = ui.create("ComboBox", Some(form)).unwrap();
    ui.set_prop(combo, "items", "Um\nDois\nTrês\nQuatro\nCinco").unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 100.0.into()), ("top", 400.0.into()), ("width", 200.0.into()), ("height", 30.0.into())] {
        ui.set_prop(combo, prop, value).unwrap();
    }
    ui.layout();
    // Dois monitores de 800 × 450, um em cima do outro.
    let pieces = span::split(ScreenRect::new(0, 0, 800, 900), &[ScreenRect::new(0, 0, 800, 450), ScreenRect::new(0, 450, 800, 450)]);
    let top = pieces[0].rect_in_form(1.0);
    ui.set_form_pieces(form, pieces.iter().map(|p| p.rect_in_form(1.0)).collect());
    let center = PointF::new(200.0, 415.0);
    let event = |position| PointerEvent { position, button: PointerButton::Primary, ..Default::default() };
    ui.pointer_move(form, event(center));
    ui.pointer_down(form, event(center));
    ui.pointer_up(form, event(center));
    assert!(ui.get::<aylis_controls::ComboBox>(combo).unwrap().is_open(), "a lista abriu");
    let popup = *ui.children(form).iter().find(|&&c| ui.class_name(c) == Some("ComboPopup")).expect("a lista");
    let r = ui.absolute_rect(popup).unwrap();
    assert!(r.y >= top.y && r.bottom() <= top.bottom(), "a lista ficou no monitor de cima: {r:?} em {top:?}");
}
