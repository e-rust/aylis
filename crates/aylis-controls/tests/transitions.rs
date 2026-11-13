//! Transições: a aba nova desliza para dentro enquanto a antiga sai, e os forms empilhados
//! (celular, navegador) entram e saem com a `transition` deles.

use aylis_controls::*;
use aylis_core::*;

fn translate_x(ui: &Ui, id: ComponentId) -> f32 {
    ui.control(id).unwrap().translate().x
}

#[test]
fn tabs_slide_in_the_order_of_the_tabs() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    let a = TabControl::add_tab(&mut ui, tabs, "A");
    let b = TabControl::add_tab(&mut ui, tabs, "B");
    let _ = ui.set_prop(tabs, "transition", PropValue::Enum("slide_left".into()));
    ui.update_layout();
    let width = ui.bounds(b).unwrap().width;
    assert!(width > 100.0);

    // Avança para B: B entra pela direita, A sai pela esquerda.
    ui.set_time(10.0);
    TabControl::select(&mut ui, tabs, 1);
    assert!(ui.control(a).unwrap().visible() && ui.control(b).unwrap().visible(), "as duas aparecem");
    assert!((translate_x(&ui, b) - width).abs() < 0.5);
    ui.set_time(10.1);
    ui.tick();
    assert!(translate_x(&ui, b) > 0.0 && translate_x(&ui, b) < width);
    assert!(translate_x(&ui, a) < 0.0);
    assert!(ui.is_animating());
    ui.set_time(11.0);
    ui.tick();
    assert_eq!(translate_x(&ui, b), 0.0);
    assert!(!ui.control(a).unwrap().visible(), "a antiga some no fim");
    assert_eq!(translate_x(&ui, a), 0.0, "e volta ao lugar");

    // Volta para A: A entra pela esquerda.
    ui.set_time(20.0);
    TabControl::select(&mut ui, tabs, 0);
    assert!(translate_x(&ui, a) < 0.0);
    ui.set_time(21.0);
    ui.tick();
    assert!(!ui.control(b).unwrap().visible());
}

#[test]
fn stacked_forms_slide_in_and_out() {
    let mut ui = Ui::new();
    ui.set_stacked_forms(true);
    let main = ui.add_root(Form::new("principal", 400.0, 600.0));
    let second = ui.add_root(Form::new("segunda", 400.0, 600.0));
    let _ = ui.set_prop(second, "transition", PropValue::Enum("slide_left".into()));
    ui.show(main).unwrap();
    ui.update_layout();

    ui.set_time(1.0);
    ui.show(second).unwrap();
    assert!((translate_x(&ui, second) - 400.0).abs() < 0.5, "entra pela direita");
    ui.set_time(2.0);
    ui.tick();
    assert_eq!(translate_x(&ui, second), 0.0);

    // Fechar: sai pela direita (o caminho de volta) e só então fecha.
    ui.set_time(3.0);
    assert!(ui.close(second));
    assert!(ui.is_open(second), "ainda aberto durante a saída");
    ui.set_time(3.1);
    ui.tick();
    assert!(translate_x(&ui, second) > 0.0);
    ui.set_time(4.0);
    ui.tick();
    assert!(!ui.is_open(second), "fechou no fim");
    assert_eq!(translate_x(&ui, second), 0.0);
    assert!(ui.is_open(main));
}

#[test]
fn without_stacking_forms_open_and_close_at_once() {
    let mut ui = Ui::new();
    let main = ui.add_root(Form::new("principal", 400.0, 600.0));
    let second = ui.add_root(Form::new("segunda", 400.0, 600.0));
    let _ = ui.set_prop(second, "transition", PropValue::Enum("fade".into()));
    ui.show(main).unwrap();
    ui.show(second).unwrap();
    assert_eq!(ui.control(second).unwrap().opacity(), 1.0);
    assert!(ui.close(second));
    assert!(!ui.is_open(second));
}

/// Um canvas que só guarda as camadas pela chave (como um renderizador, sem pixels).
#[derive(Default)]
struct Layers {
    live: std::collections::HashSet<u64>,
    open: Vec<Option<u64>>,
    built: usize,
}

struct LayerCanvas<'a>(&'a mut Layers);

impl Canvas for LayerCanvas<'_> {
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
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
    fn begin_cached(&mut self, key: u64, _: RectF) -> bool {
        if self.0.live.contains(&key) {
            return true;
        }
        self.0.open.push(Some(key));
        false
    }
    fn end_cached(&mut self) {
        if let Some(key) = self.0.open.pop().flatten() {
            self.0.live.insert(key);
            self.0.built += 1;
        }
    }
    fn forget_cached(&mut self, key: u64) {
        self.0.live.remove(&key);
    }
}

/// Uma troca de aba com transição, quadro a quadro como a plataforma (por parte).
fn run_tab_transition(ui: &mut Ui, form: ComponentId, tabs: ComponentId, to: usize, layers: &mut Layers) {
    let start = ui.time();
    TabControl::select(ui, tabs, to);
    for step in 0..=20 {
        ui.set_time(start + step as f64 / 60.0);
        ui.tick();
        let area = ui.take_damage(form);
        ui.paint_region(form, &mut LayerCanvas(layers), area);
    }
    ui.set_time(start + 1.0);
    ui.tick();
    let area = ui.take_damage(form);
    ui.paint_region(form, &mut LayerCanvas(layers), area);
    assert!(!ui.is_animating(), "a transição acabou");
}

fn tab_form() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    for caption in ["A", "B"] {
        let page = TabControl::add_tab(&mut ui, tabs, caption);
        let b = ui.add(Button::new(caption), page);
        ui.control_mut(b).unwrap().set_align(Align::Top);
    }
    let _ = ui.set_prop(tabs, "transition", PropValue::Enum("slide_left".into()));
    ui.show(form).unwrap();
    ui.set_time(10.0);
    ui.layout();
    (ui, form, tabs)
}

/// As páginas andam como camadas guardadas, e no fim da transição as imagens saem do
/// renderizador (antes, ficavam no cache até ele se limpar sozinho).
#[test]
fn the_transition_layers_leave_the_renderer_when_it_ends() {
    let (mut ui, form, tabs) = tab_form();
    let mut layers = Layers::default();
    ui.paint_region(form, &mut LayerCanvas(&mut layers), None);
    run_tab_transition(&mut ui, form, tabs, 1, &mut layers);
    assert!(layers.built >= 2, "as duas páginas viraram camada: {}", layers.built);
    assert!(layers.live.is_empty(), "no fim, nenhuma imagem fica guardada: {}", layers.live.len());
    run_tab_transition(&mut ui, form, tabs, 0, &mut layers);
    assert!(layers.live.is_empty(), "nem depois da segunda troca: {}", layers.live.len());
}

/// `layer_cache = off` no form, ou `auto_layers` desligado no projeto: a transição não monta
/// camada (repinta as páginas por parte, como sem o cache). `on` no form vale mesmo com o
/// projeto desligado.
#[test]
fn the_form_and_the_project_turn_the_automatic_layers_off() {
    let (mut ui, form, tabs) = tab_form();
    let mut layers = Layers::default();
    let _ = ui.set_prop(form, "layer_cache", PropValue::Enum("off".into()));
    ui.paint_region(form, &mut LayerCanvas(&mut layers), None);
    run_tab_transition(&mut ui, form, tabs, 1, &mut layers);
    assert_eq!(layers.built, 0, "o form desligou as camadas");

    let _ = ui.set_prop(form, "layer_cache", PropValue::Enum("project".into()));
    image_cache::set_auto_layers(false);
    run_tab_transition(&mut ui, form, tabs, 0, &mut layers);
    let off = layers.built;
    let _ = ui.set_prop(form, "layer_cache", PropValue::Enum("on".into()));
    run_tab_transition(&mut ui, form, tabs, 1, &mut layers);
    image_cache::set_auto_layers(true);
    assert_eq!(off, 0, "o projeto desligou as camadas");
    assert!(layers.built >= 2, "o form ligou de novo: {}", layers.built);
}
