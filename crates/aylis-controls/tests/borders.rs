//! A borda dos componentes de texto pode sair (`border: false`), neles e nos que os embrulham.

use aylis_controls::*;
use aylis_core::*;

/// Guarda a cor de cada texto desenhado.
#[derive(Default)]
struct Texts(Vec<(String, Color)>);

impl Canvas for Texts {
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
    fn draw_text(&mut self, text: &str, _: RectF, style: &TextStyle) {
        self.0.push((text.to_string(), style.color));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

#[test]
fn win32_selection_is_light_blue_with_black_text() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    ui.set_prop(form, "theme", "Win32").unwrap();
    let e = ui.add(Edit::new("teste"), form);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 200.0, 30.0);
    ui.layout();
    ui.set_focus(e);
    ui.get_mut::<Edit>(e).unwrap().select_all();
    let mut canvas = Texts::default();
    ui.paint(form, &mut canvas);
    let colors: Vec<Color> = canvas.0.iter().filter(|(t, _)| t == "teste").map(|(_, c)| *c).collect();
    assert_eq!(colors, [Color::from_hex("#000000").unwrap()], "o texto selecionado continua preto");
    assert_eq!(ui.theme_named("Win32").unwrap().color("selection"), Color::from_hex("#CCE8FF").unwrap(), "o azul claro da seleção");
}

#[test]
fn text_components_can_drop_the_border() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    for class in ["Edit", "Memo", "CodeEditor", "NumberBox", "AutoComplete"] {
        let id = ui.create(class, Some(form)).unwrap();
        assert_eq!(ui.get_prop(id, "border").unwrap(), PropValue::Bool(true), "{class}: com borda por padrão");
        ui.set_prop(id, "border", false).unwrap();
        assert_eq!(ui.get_prop(id, "border").unwrap(), PropValue::Bool(false), "{class}");
    }
    assert!(!ui.get::<Edit>(ui.children(form)[0]).unwrap().border);
    assert!(!ui.get::<Memo>(ui.children(form)[2]).unwrap().border, "o Memo de dentro do CodeEditor");
}
