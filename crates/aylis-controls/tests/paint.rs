//! PaintCanvas: as ferramentas desenham nos pixels, o desfazer volta e os métodos pelo nome
//! (os mesmos do protocolo) funcionam.

use aylis_controls::{Form, PaintCanvas};
use aylis_core::*;

fn scene() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("p", 400.0, 300.0));
    let pc = ui.add(PaintCanvas::default(), form);
    ui.control_mut(pc).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    ui.invoke(pc, "new", &[PropValue::Int(200), PropValue::Int(100)]).unwrap();
    ui.layout();
    (ui, form, pc)
}

fn drag(ui: &mut Ui, form: ComponentId, from: (f32, f32), to: (f32, f32), button: PointerButton) {
    let ev = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), button, ..Default::default() };
    ui.pointer_down(form, ev(from.0, from.1));
    ui.pointer_move(form, ev((from.0 + to.0) / 2.0, (from.1 + to.1) / 2.0));
    ui.pointer_move(form, ev(to.0, to.1));
    ui.pointer_up(form, ev(to.0, to.1));
}

fn pixel(ui: &Ui, pc: ComponentId, x: i32, y: i32) -> Color {
    ui.get::<PaintCanvas>(pc).unwrap().bitmap().get(x, y).unwrap()
}

#[test]
fn tools_draw_and_undo_restores() {
    let (mut ui, form, pc) = scene();
    let white = Color::rgb(255, 255, 255);
    ui.set_prop(pc, "color", Color::rgb(255, 0, 0)).unwrap();
    ui.set_prop(pc, "brush_size", 1i64).unwrap();
    drag(&mut ui, form, (10.0, 10.0), (50.0, 10.0), PointerButton::Primary);
    assert_eq!(pixel(&ui, pc, 30, 10), Color::rgb(255, 0, 0), "o lápis traçou a linha");
    assert_eq!(pixel(&ui, pc, 30, 11), white);

    ui.set_prop(pc, "tool", "rectangle").unwrap();
    drag(&mut ui, form, (60.0, 20.0), (100.0, 60.0), PointerButton::Primary);
    assert_eq!(pixel(&ui, pc, 80, 20), Color::rgb(255, 0, 0), "a borda de cima");
    assert_eq!(pixel(&ui, pc, 80, 40), white, "o retângulo é só o contorno");

    // O balde dentro do retângulo pinta só ele.
    ui.set_prop(pc, "tool", "fill").unwrap();
    ui.set_prop(pc, "color", Color::rgb(0, 0, 255)).unwrap();
    drag(&mut ui, form, (80.0, 40.0), (80.0, 40.0), PointerButton::Primary);
    assert_eq!(pixel(&ui, pc, 70, 30), Color::rgb(0, 0, 255));
    assert_eq!(pixel(&ui, pc, 150, 80), white, "fora do retângulo não");

    // Conta-gotas com o botão direito vira a cor 2.
    ui.set_prop(pc, "tool", "picker").unwrap();
    let picked = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let p = picked.clone();
    ui.on(pc, "on_pick", move |_, _, args| *p.borrow_mut() = args.text().unwrap_or_default().to_string());
    drag(&mut ui, form, (30.0, 10.0), (30.0, 10.0), PointerButton::Secondary);
    assert_eq!(picked.borrow().as_str(), "#FF00002");
    assert_eq!(ui.get_prop(pc, "back_color").unwrap(), PropValue::Color(Color::rgb(255, 0, 0)));

    // Desfazer três vezes volta à imagem em branco; refazer devolve.
    for _ in 0..3 {
        assert_eq!(ui.invoke(pc, "undo", &[]).unwrap(), PropValue::Bool(true));
    }
    assert_eq!(pixel(&ui, pc, 30, 10), white);
    assert_eq!(ui.invoke(pc, "can_redo", &[]).unwrap(), PropValue::Bool(true));
    ui.invoke(pc, "redo", &[]).unwrap();
    assert_eq!(pixel(&ui, pc, 30, 10), Color::rgb(255, 0, 0));
    assert!(ui.invoke(pc, "voar", &[]).is_err());
}

#[test]
fn save_open_and_size() {
    let (mut ui, form, pc) = scene();
    ui.set_prop(pc, "tool", "ellipse").unwrap();
    ui.set_prop(pc, "filled", true).unwrap();
    ui.set_prop(pc, "back_color", Color::rgb(0, 128, 0)).unwrap();
    drag(&mut ui, form, (20.0, 20.0), (120.0, 80.0), PointerButton::Primary);
    assert_eq!(pixel(&ui, pc, 70, 50), Color::rgb(0, 128, 0), "o meio da elipse preenchido");
    let path = std::env::temp_dir().join(format!("aylis_paint_{}.png", std::process::id()));
    let path_s = path.to_string_lossy().to_string();
    ui.invoke(pc, "save", &[PropValue::String(path_s.clone())]).unwrap();
    ui.invoke(pc, "new", &[PropValue::Int(50), PropValue::Int(40)]).unwrap();
    assert_eq!(ui.invoke(pc, "size", &[]).unwrap(), PropValue::String("50 × 40".into()));
    ui.invoke(pc, "open", &[PropValue::String(path_s)]).unwrap();
    let _ = std::fs::remove_file(&path);
    assert_eq!(ui.invoke(pc, "size", &[]).unwrap(), PropValue::String("200 × 100".into()));
    assert_eq!(pixel(&ui, pc, 70, 50), Color::rgb(0, 128, 0));
    // O controle acompanha o tamanho da imagem e o zoom.
    ui.set_prop(pc, "zoom", 2.0).unwrap();
    ui.layout();
    assert_eq!(ui.control(pc).unwrap().bounds().width, 400.0 + 12.0, "a imagem com zoom e o espaço das alças da borda");
}

#[test]
fn selection_moves_with_the_mouse_and_the_arrows() {
    let (mut ui, form, pc) = scene();
    let red = Color::rgb(255, 0, 0);
    let white = Color::rgb(255, 255, 255);
    ui.set_prop(pc, "color", red).unwrap();
    ui.set_prop(pc, "tool", "rectangle").unwrap();
    ui.set_prop(pc, "filled", true).unwrap();
    ui.set_prop(pc, "back_color", red).unwrap();
    drag(&mut ui, form, (10.0, 10.0), (19.0, 19.0), PointerButton::Primary);
    ui.set_prop(pc, "back_color", white).unwrap();
    assert_eq!(pixel(&ui, pc, 15, 15), red);

    // Seleciona o quadrado e arrasta 20 para a direita: o lugar antigo fica branco.
    ui.set_prop(pc, "tool", "select").unwrap();
    drag(&mut ui, form, (5.0, 5.0), (25.0, 25.0), PointerButton::Primary);
    assert_eq!(ui.invoke(pc, "has_selection", &[]).unwrap(), PropValue::Bool(true));
    drag(&mut ui, form, (15.0, 15.0), (35.0, 15.0), PointerButton::Primary);
    // As setas andam 1 pixel (Shift: 10).
    let key = |k: Key, shift: bool| KeyEvent { key: k, modifiers: Modifiers { shift, ..Default::default() }, repeat: false };
    ui.key_down(form, key(Key::Down, false));
    ui.key_down(form, key(Key::Right, true));
    ui.key_down(form, key(Key::Enter, false));
    assert_eq!(ui.invoke(pc, "has_selection", &[]).unwrap(), PropValue::Bool(false));
    assert_eq!(pixel(&ui, pc, 15, 15), white, "o lugar de onde saiu");
    assert_eq!(pixel(&ui, pc, 10 + 30, 10 + 1), red, "andou 20 + 10 para a direita e 1 para baixo");
    assert_eq!(pixel(&ui, pc, 19 + 30, 19 + 1), red);

    // Desfazer volta o quadrado para o lugar.
    ui.invoke(pc, "undo", &[]).unwrap();
    assert_eq!(pixel(&ui, pc, 15, 15), red);

    // Ctrl+A, copiar, colar: a cópia entra solta no canto e Delete a apaga.
    ui.invoke(pc, "select_all", &[]).unwrap();
    ui.invoke(pc, "copy", &[]).unwrap();
    ui.invoke(pc, "deselect", &[]).unwrap();
    ui.invoke(pc, "paste", &[]).unwrap();
    assert_eq!(ui.invoke(pc, "has_selection", &[]).unwrap(), PropValue::Bool(true));
    ui.key_down(form, key(Key::Delete, false));
    assert_eq!(ui.invoke(pc, "has_selection", &[]).unwrap(), PropValue::Bool(false));
    assert_eq!(pixel(&ui, pc, 15, 15), red, "apagar a cópia solta não mexe na imagem");
}

#[test]
fn the_border_handles_resize_the_image() {
    let (mut ui, form, pc) = scene();
    let red = Color::rgb(255, 0, 0);
    ui.set_prop(pc, "color", red).unwrap();
    ui.set_prop(pc, "brush_size", 1i64).unwrap();
    drag(&mut ui, form, (150.0, 50.0), (190.0, 50.0), PointerButton::Primary);
    ui.set_prop(pc, "back_color", Color::rgb(0, 0, 255)).unwrap();
    // A alça do canto fica logo depois da imagem (200 × 100): arrastar para 120 × 160.
    drag(&mut ui, form, (204.0, 104.0), (119.0, 159.0), PointerButton::Primary);
    assert_eq!(ui.invoke(pc, "size", &[]).unwrap(), PropValue::String("120 × 160".into()));
    assert_eq!(pixel(&ui, pc, 10, 150), Color::rgb(0, 0, 255), "o que cresceu tem a cor 2");
    assert_eq!(pixel(&ui, pc, 10, 10), Color::rgb(255, 255, 255), "o que ficou é o mesmo");
    ui.layout();
    assert_eq!(ui.control(pc).unwrap().bounds().width, 120.0 + 12.0, "o controle acompanha");
    // A alça da direita só mexe na largura; desfazer volta ao tamanho de antes.
    drag(&mut ui, form, (124.0, 80.0), (299.0, 80.0), PointerButton::Primary);
    assert_eq!(ui.invoke(pc, "size", &[]).unwrap(), PropValue::String("300 × 160".into()));
    ui.invoke(pc, "undo", &[]).unwrap();
    ui.invoke(pc, "undo", &[]).unwrap();
    assert_eq!(ui.invoke(pc, "size", &[]).unwrap(), PropValue::String("200 × 100".into()));
    assert_eq!(pixel(&ui, pc, 170, 50), red, "o traço cortado volta com o desfazer");
}

#[test]
fn fast_freehand_strokes_are_curves() {
    // Um círculo feito rápido chega com poucos pontos (8 aqui): o traço passa por curvas
    // entre eles, sem as quinas de ligar os pontos com retas.
    let (mut ui, form, pc) = scene();
    ui.set_prop(pc, "brush_size", 1i64).unwrap();
    let (cx, cy, r) = (100.0f32, 50.0f32, 40.0f32);
    let at = |k: usize| {
        let a = k as f32 * std::f32::consts::TAU / 8.0;
        (cx + r * a.cos(), cy + r * a.sin())
    };
    let ev = |(x, y): (f32, f32)| PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(form, ev(at(0)));
    for k in 1..=8 {
        ui.pointer_move(form, ev(at(k)));
    }
    ui.pointer_up(form, ev(at(8)));
    let bitmap = ui.get::<PaintCanvas>(pc).unwrap().bitmap();
    let white = Color::rgb(255, 255, 255);
    let mut painted = 0;
    for y in 0..100 {
        for x in 0..200 {
            if bitmap.get(x, y).unwrap() != white {
                painted += 1;
                let d = ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt();
                // A ponta onde o traço começa e termina fica no vértice 0.
                if (x as f32 - at(0).0).abs() < 8.0 && (y as f32 - at(0).1).abs() < 12.0 {
                    continue;
                }
                assert!(d <= r * 0.93 + 1.5 && d >= r * (std::f32::consts::PI / 8.0).cos() - 1.5, "({x},{y}) a {d} do centro");
            }
        }
    }
    assert!(painted > 200, "o traço fechou a volta ({painted} pixels)");
    for k in 1..8 {
        let (x, y) = at(k);
        // O vértice: as retas passariam por ele; a curva passa por dentro (a 0,93 do raio).
        let (ox, oy) = (cx + (x - cx) * 0.99, cy + (y - cy) * 0.99);
        assert_eq!(bitmap.get(ox as i32, oy as i32).unwrap(), white, "quina {k}");
    }
}

/// Guarda as imagens desenhadas (o id e onde).
#[derive(Default)]
struct Images(Vec<(u64, RectF)>);

impl Canvas for Images {
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
    fn draw_image(&mut self, image: &Image, _: RectF, dest: RectF, _: f32) {
        self.0.push((image.id(), dest));
    }
}

#[test]
fn a_stroke_redraws_only_the_tiles_it_touches() {
    // Numa pintura grande, refazer a imagem inteira a cada movimento do mouse deixava
    // poucos pontos por segundo: só os blocos tocados viram imagem nova.
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("p", 1200.0, 900.0));
    let pc = ui.add(PaintCanvas::default(), form);
    ui.control_mut(pc).unwrap().set_rect(0.0, 0.0, 1100.0, 800.0);
    ui.invoke(pc, "new", &[PropValue::Int(1024), PropValue::Int(768)]).unwrap();
    ui.layout();
    let mut before = Images::default();
    ui.paint(form, &mut before);
    assert_eq!(before.0.len(), 12, "4 × 3 blocos de 256");
    let ev = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(form, ev(100.0, 100.0));
    ui.pointer_move(form, ev(120.0, 110.0));
    let mut after = Images::default();
    ui.paint(form, &mut after);
    let new: Vec<_> = after.0.iter().filter(|(id, _)| !before.0.iter().any(|(b, _)| b == id)).collect();
    assert_eq!(new.len(), 1, "só o bloco do traço: {new:?}");
    assert_eq!(new[0].1.x, 0.0);
    // Os blocos cobrem a imagem toda, com um pixel a mais para não ter emenda.
    let right = after.0.iter().map(|(_, r)| r.x + r.width).fold(0.0f32, f32::max);
    assert_eq!(right, 1024.0);
}
