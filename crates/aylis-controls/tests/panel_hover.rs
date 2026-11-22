//! O `Panel` que reage ao mouse (`hover_fill`, `pressed_fill`, `interactive`), também com o
//! mouse num filho (o relatório do player de música, 21/11: as linhas e os botões feitos de
//! painel não reagiam).

use aylis_controls::{Form, Label, Panel};
use aylis_core::*;

/// Canvas que guarda a cor de cada retângulo preenchido.
#[derive(Default)]
struct Probe {
    fills: Vec<Color>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, brush: &Brush) {
        if let Brush::Solid(c) = brush {
            self.fills.push(*c);
        }
    }
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.5, style.font.size * 1.2)
    }
}

fn fills(ui: &mut Ui, form: ComponentId) -> Vec<Color> {
    let mut p = Probe::default();
    ui.paint(form, &mut p);
    p.fills
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

#[test]
fn the_panel_changes_its_fill_with_the_mouse_over_it_or_a_child() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 300.0, 300.0));
    let row = ui.add(Panel::new(200.0, 40.0), form);
    let red = Color::rgb(200, 0, 0);
    let dark = Color::rgb(90, 0, 0);
    ui.set_prop(row, "hover_fill", red).unwrap();
    ui.set_prop(row, "pressed_fill", dark).unwrap();
    // Um filho que recebe o mouse (com hit_test): o painel ainda conta como "com o mouse".
    let child = ui.add(Label::new("música"), row);
    ui.set_prop(child, "hit_test", true).unwrap();
    ui.control_mut(child).unwrap().set_width(100.0).set_height(20.0);
    ui.show(form).unwrap();
    ui.layout();
    assert!(!fills(&mut ui, form).contains(&red), "sem mouse, o fundo de sempre");
    ui.pointer_move(form, at(10.0, 10.0));
    assert_eq!(ui.hovered(form), Some(child));
    assert!(fills(&mut ui, form).contains(&red), "mouse no filho");
    ui.pointer_down(form, at(10.0, 10.0));
    assert!(fills(&mut ui, form).contains(&dark), "apertado");
    ui.pointer_up(form, at(10.0, 10.0));
    ui.pointer_move(form, at(280.0, 280.0));
    assert!(!fills(&mut ui, form).contains(&red), "saiu");
}

#[test]
fn interactive_uses_the_theme_without_a_color() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 300.0, 300.0));
    let row = ui.add(Panel::new(200.0, 40.0), form);
    ui.show(form).unwrap();
    ui.layout();
    let before = fills(&mut ui, form);
    ui.set_prop(row, "interactive", true).unwrap();
    ui.pointer_move(form, at(10.0, 10.0));
    let after = fills(&mut ui, form);
    assert_ne!(before, after, "com o mouse, outro fundo");
    assert_eq!(ui.control(row).unwrap().cursor(), CursorShape::Pointer);
}
