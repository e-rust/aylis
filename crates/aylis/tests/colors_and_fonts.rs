//! Cores e fonte próprias em todo componente: `fill`, `border_color`, `text_color`,
//! `accent_color` e a fonte (`font_family`, `font_italic`...) valem para qualquer controle.
//! Cada componente é desenhado num canvas que grava, com e sem as cores, e o teste confere
//! que o fundo, o texto e a fonte que ele desenha seguem as do controle.

use aylis::controls::Form;
use aylis::core::pages::{Op, RecordingCanvas};
use aylis::core::*;

fn close(a: Color, b: Color) -> bool {
    (a.r as i32 - b.r as i32).abs() + (a.g as i32 - b.g as i32).abs() + (a.b as i32 - b.b as i32).abs() < 60
}

/// O que um componente desenhou: os fundos que cobrem boa parte dele, as cores dos textos
/// e as fontes.
struct Drawn {
    backgrounds: Vec<Color>,
    texts: Vec<Color>,
    fonts: Vec<(String, bool)>,
}

fn draw(ui: &mut Ui, id: ComponentId) -> Drawn {
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(id, &mut canvas);
    let area = ui.control(id).map_or(1.0, |c| c.bounds().width * c.bounds().height);
    let mut d = Drawn { backgrounds: Vec::new(), texts: Vec::new(), fonts: Vec::new() };
    for op in &canvas.ops {
        match op {
            Op::FillRect { rect, color, .. } | Op::FillEllipse { rect, color } if rect.width * rect.height >= area * 0.5 => d.backgrounds.push(*color),
            Op::Text { style, text, .. } if !text.trim().is_empty() => {
                d.texts.push(style.color);
                d.fonts.push((style.font.family.clone(), style.font.italic));
            }
            _ => {}
        }
    }
    d
}

/// Componentes cujo fundo e textos são o próprio desenho (o mapa e os nomes dele, o 3D, a
/// roda de cores): as cores próprias não os trocam.
const DRAWN_CONTENT: &[&str] = &["MapView", "ColorPicker", "ChoroplethMap", "Waveform", "ActivityRings", "SkeletonLoader", "Rating", "PageControl", "LoadingIndicator", "Model3D"];

#[test]
fn every_component_follows_its_own_colors_and_font() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let form = ui.add_root(Form::new("cores", 900.0, 700.0));
    let names: Vec<&'static str> = ui.registry().iter().map(|c| c.name).collect();
    let (fill, text) = (Color::rgb(220, 20, 60), Color::rgb(30, 60, 200));
    let mut problems = Vec::new();
    for name in names {
        let Ok(id) = ui.create(name, Some(form)) else { continue };
        let Some(cb) = ui.control_mut(id) else {
            let _ = ui.free(id);
            continue;
        };
        cb.set_rect(10.0, 10.0, 300.0, 160.0);
        for (k, v) in [("text", PropValue::String("Texto".into())), ("caption", "Texto".into()), ("items", "A;B;C".into()), ("value", PropValue::Float(50.0))] {
            let _ = ui.set_prop(id, k, v);
        }
        ui.layout();
        let before = draw(&mut ui, id);
        for (k, v) in [("fill", PropValue::Color(fill)), ("text_color", PropValue::Color(text)), ("font_family", "Courier New".into()), ("font_italic", true.into())] {
            let _ = ui.set_prop(id, k, v);
        }
        let after = draw(&mut ui, id);
        if !before.backgrounds.is_empty() && !DRAWN_CONTENT.contains(&name) && !after.backgrounds.iter().any(|c| close(*c, fill)) {
            problems.push(format!("{name}: o fundo não segue o fill"));
        }
        if !before.texts.is_empty() && !DRAWN_CONTENT.contains(&name) && !after.texts.iter().any(|c| close(*c, text)) {
            problems.push(format!("{name}: o texto não segue o text_color"));
        }
        if after.fonts.iter().any(|(family, italic)| family != "Courier New" || !italic) {
            problems.push(format!("{name}: o texto não segue a fonte do controle"));
        }
        let _ = ui.free(id);
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn the_accent_and_the_meaning_colors_recolor_the_theme_the_control_sees() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let form = ui.add_root(Form::new("cores", 600.0, 400.0));
    let bar = ui.create("ProgressBar", Some(form)).unwrap();
    ui.control_mut(bar).unwrap().set_rect(0.0, 0.0, 200.0, 20.0);
    ui.set_prop(bar, "value", 60.0).unwrap();
    let accent = Color::rgb(255, 0, 255);
    ui.set_prop(bar, "accent_color", accent).unwrap();
    ui.layout();
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(bar, &mut canvas);
    assert!(canvas.ops.iter().any(|op| matches!(op, Op::FillRect { color, .. } if close(*color, accent))), "a barra na cor de destaque");
    // As cores de sentido (o gradiente do Gauge vai do success ao danger).
    let gauge = ui.create("Gauge", Some(form)).unwrap();
    ui.control_mut(gauge).unwrap().set_rect(0.0, 40.0, 120.0, 120.0);
    ui.set_prop(gauge, "gradient", true).unwrap();
    ui.set_prop(gauge, "success_color", Color::rgb(0, 0, 0)).unwrap();
    assert_eq!(ui.get_prop(gauge, "success_color").unwrap(), PropValue::Color(Color::rgb(0, 0, 0)));
    // As cores voltam ao tema com o valor vazio.
    ui.set_prop(bar, "accent_color", PropValue::Null).unwrap();
    assert_eq!(ui.get_prop(bar, "accent_color").unwrap(), PropValue::Null);
}
