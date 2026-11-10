mod common;

use common::{Fancy, Op, RecordingCanvas, boxy};
use aylis_core::*;

fn with_boxy_style(ui: &mut Ui) {
    ui.theme_mut().set_style(
        "Boxy",
        Style::new(StyleProps::new().fill("primary"))
            .hover(StyleProps::new().fill("#FF0000"))
            .disabled(StyleProps::new().opacity(0.5)),
    );
    ui.theme_mut().set_style("Boxy.danger", Style::new(StyleProps::new().fill("danger")));
}

#[test]
fn children_are_painted_at_their_absolute_position() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(200.0, 100.0));
    let panel = ui.add(boxy(0.0, 0.0), form);
    ui.control_mut(panel).unwrap().set_align(Align::Client).set_padding(10.0);
    let inner = ui.add(boxy(0.0, 20.0), panel);
    ui.control_mut(inner).unwrap().set_align(Align::Top);

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    assert_eq!(
        canvas.fills(),
        vec![
            RectF::new(0.0, 0.0, 200.0, 100.0),
            RectF::new(0.0, 0.0, 200.0, 100.0),
            RectF::new(10.0, 10.0, 180.0, 20.0),
        ]
    );
    assert!(canvas.balanced(), "todo save tem restore");
}

#[test]
fn invisible_controls_and_their_children_are_skipped() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let hidden = ui.add(boxy(10.0, 10.0), form);
    ui.add(boxy(5.0, 5.0), hidden);
    ui.control_mut(hidden).unwrap().set_visible(false);

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    assert_eq!(canvas.fills().len(), 1);
}

#[test]
fn theme_resolves_class_variant_and_state() {
    let mut ui = Ui::new();
    with_boxy_style(&mut ui);
    let primary = ui.theme().color("primary");
    let danger = ui.theme().color("danger");

    let form = ui.add_root(boxy(100.0, 100.0));
    ui.add(boxy(10.0, 10.0), form);
    let hovered = ui.add(boxy(10.0, 10.0), form);
    ui.control_mut(hovered).unwrap().state_mut().hover = true;
    let variant = ui.add(boxy(10.0, 10.0), form);
    ui.control_mut(variant).unwrap().set_style("danger");

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    let colors: Vec<Color> = canvas
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Fill { color, .. } => Some(*color),
            _ => None,
        })
        .collect();
    assert_eq!(colors, vec![primary, primary, Color::rgb(255, 0, 0), danger]);
}

#[test]
fn disabled_parent_disables_children_and_applies_theme_opacity() {
    let mut ui = Ui::new();
    with_boxy_style(&mut ui);
    let form = ui.add_root(boxy(100.0, 100.0));
    let panel = ui.add(boxy(50.0, 50.0), form);
    ui.add(boxy(10.0, 10.0), panel);
    ui.control_mut(panel).unwrap().set_enabled(false);

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    let layers: Vec<_> = canvas.ops.iter().filter(|op| matches!(op, Op::Layer { .. })).collect();
    assert_eq!(layers, vec![&Op::Layer { opacity: 0.5 }, &Op::Layer { opacity: 0.5 }]);
    assert!(canvas.balanced());
}

#[test]
fn composed_control_uses_base_style_and_paints_on_top() {
    let mut ui = Ui::new();
    with_boxy_style(&mut ui);
    let primary = ui.theme().color("primary");
    let form = ui.add_root(boxy(100.0, 100.0));
    let fancy = ui.add(Fancy::default(), form);
    ui.set_prop(fancy, "glow", 2).unwrap();
    ui.set_prop(fancy, "left", 5).unwrap();

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    assert_eq!(&canvas.ops[1..], &[
        // estilo "Boxy" herdado, porque não existe estilo "Fancy"
        Op::Fill { rect: RectF::new(5.0, 0.0, 100.0, 100.0), color: primary },
        Op::Stroke { rect: RectF::new(5.0, 0.0, 100.0, 100.0), width: 2.0 },
    ]);
}

#[test]
fn on_paint_can_wrap_the_default_painting() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let b = ui.add(boxy(10.0, 10.0), form);
    ui.on_paint(b, |canvas, ctx, default| {
        default(canvas, ctx);
        if ctx.state.hover {
            canvas.stroke_rect(ctx.rect, 0.0, &Stroke::new(Color::WHITE, 3.0));
        }
    });
    ui.control_mut(b).unwrap().state_mut().hover = true;

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    assert!(matches!(canvas.ops.last(), Some(Op::Stroke { width, .. }) if *width == 3.0));
    assert_eq!(canvas.fills().len(), 2, "a pintura padrão continua acontecendo");
}

#[test]
fn clip_children_clips_to_the_control() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let panel = ui.add(boxy(40.0, 40.0), form);
    ui.control_mut(panel).unwrap().set_position(10.0, 10.0).set_clip_children(true);
    ui.add(boxy(80.0, 80.0), panel);

    let mut canvas = RecordingCanvas::default();
    ui.paint(form, &mut canvas);
    assert!(canvas.ops.contains(&Op::Clip { rect: RectF::new(10.0, 10.0, 40.0, 40.0) }));
}

#[test]
fn dark_theme_swaps_tokens() {
    let light = Theme::light();
    let dark = Theme::dark();
    let state = ControlState::default();
    let a = light.resolve(&["Button"], "", state);
    let b = dark.resolve(&["Button"], "", state);
    assert_ne!(a.fill, b.fill);
    assert_eq!(a.radius, 6.0);
    assert_eq!(a.transition_ms, 120.0);
    let pressed = light.resolve(&["Button"], "", ControlState { pressed: true, ..state });
    assert_eq!(pressed.fill, light.color("primary_pressed"));
}

#[test]
fn flat_lists_have_no_border_even_focused_in_every_theme() {
    let focused = ControlState { focused: true, ..ControlState::default() };
    for theme in aylis_core::themes::all() {
        for class in ["ListBox", "ListView", "TreeView", "Memo"] {
            let s = theme.resolve(&[class], "flat", focused);
            assert!(s.border_width == 0.0 || s.border_color.a == 0, "{} {class}: borda {s:?}", theme.name);        }
    }
}

/// Grava o que o `snapshot` pinta (no lugar do desenho fora da tela da plataforma).
struct RecordingOffscreen(std::rc::Rc<std::cell::RefCell<(u32, u32, Vec<RectF>)>>);

impl platform::OffscreenService for RecordingOffscreen {
    fn render_png(&self, width: u32, height: u32, _scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        assert_eq!(background, Color::TRANSPARENT, "o fundo do desenho fica transparente");
        let mut canvas = RecordingCanvas::default();
        paint(&mut canvas);
        *self.0.borrow_mut() = (width, height, canvas.fills());
        Some(vec![1, 2, 3])
    }
}

/// O desenho de um controle no meio do form (a imagem que acompanha um arrasto): sai na
/// origem, do tamanho dele, com os filhos, e o controle não sai do lugar.
#[test]
fn a_snapshot_draws_the_control_at_the_origin_without_moving_it() {
    let mut ui = Ui::new();
    with_boxy_style(&mut ui);
    let form = ui.add_root(boxy(400.0, 300.0));
    let card = ui.add(boxy(120.0, 80.0), form);
    ui.control_mut(card).unwrap().set_position(200.0, 150.0);
    let inner = ui.add(boxy(20.0, 10.0), card);
    ui.control_mut(inner).unwrap().set_position(5.0, 6.0);
    ui.layout();
    let recorded = std::rc::Rc::new(std::cell::RefCell::new((0, 0, Vec::new())));
    ui.services_mut().offscreen = Some(Box::new(RecordingOffscreen(recorded.clone())));

    let image = ui.snapshot(card, 2.0).expect("a imagem");
    assert_eq!(image.source(), &ImageSource::Encoded(vec![1, 2, 3]));
    let (w, h, fills) = recorded.borrow().clone();
    assert_eq!((w, h), (120, 80), "do tamanho do controle");
    assert_eq!(fills.first(), Some(&RectF::new(0.0, 0.0, 120.0, 80.0)), "o cartão na origem");
    assert!(fills.contains(&RectF::new(5.0, 6.0, 20.0, 10.0)), "o filho no lugar dele: {fills:?}");
    assert_eq!(ui.control(card).unwrap().bounds(), RectF::new(200.0, 150.0, 120.0, 80.0), "não mexe no controle");
    // Sem o serviço (o navegador), não há desenho.
    ui.services_mut().offscreen = None;
    assert!(ui.snapshot(card, 2.0).is_none());
}
