//! O Reveal Highlight (o `reveal` do estilo): a luz que segue o mouse aparece no controle e o
//! mouse que anda repinta só a parte perto dela.

use aylis_core::*;

const W: u32 = 400;
const H: u32 = 200;

/// Um form com um botão de 120 × 40 em (40, 40), no estilo `reveal` (borda e fundo).
fn screen() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    let button = ui.create("Button", Some(form)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 40.0.into()), ("top", 40.0.into()), ("width", 120.0.into()), ("height", 40.0.into()), ("text", "".into()), ("style", "reveal".into())] {
        ui.set_prop(button, prop, value).unwrap();
    }
    ui.show(form).unwrap();
    ui.set_time(1000.0);
    ui.layout();
    (ui, form, button)
}

fn render(ui: &mut Ui, form: ComponentId) -> Vec<u8> {
    render_with("Skia", ui, form)
}

fn render_with(painter: &str, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
    match painter {
        #[cfg(windows)]
        "Direct2D" => aylis_d2d::D2dRenderer::new().unwrap().render_rgba(W, H, 1.0, Color::WHITE, |c| ui.paint_region(form, c, None)).unwrap(),
        _ => aylis_skia::SkiaRenderer::new().render_rgba(W, H, 1.0, Color::WHITE, |c| ui.paint_region(form, c, None)).unwrap(),
    }
}

fn pixel(rgba: &[u8], x: u32, y: u32) -> [u8; 4] {
    let i = ((y * W + x) * 4) as usize;
    [rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]
}

fn mouse(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    ui.pointer_move(form, PointerEvent { position: PointF::new(x, y), ..Default::default() });
}

#[test]
fn the_light_follows_the_mouse() {
    let painters: &[&str] = if cfg!(windows) { &["Skia", "Direct2D"] } else { &["Skia"] };
    for &painter in painters {
        let (mut ui, form, _) = screen();
        // O mouse longe (o canto de baixo): sem luz no botão.
        mouse(&mut ui, form, 330.0, 160.0);
        let far = render_with(painter, &mut ui, form);
        // Em cima do botão, perto da borda esquerda: o fundo e a borda ali mudam.
        mouse(&mut ui, form, 60.0, 60.0);
        let near = render_with(painter, &mut ui, form);
        assert_ne!(pixel(&far, 60, 60), pixel(&near, 60, 60), "{painter}: o fundo sob o mouse não acendeu");
        assert_ne!(pixel(&far, 40, 60), pixel(&near, 40, 60), "{painter}: a borda perto do mouse não acendeu");
        // Longe do mouse, no mesmo botão (a luz não alcança): igual.
        assert_eq!(pixel(&far, 158, 78), pixel(&near, 158, 78), "{painter}: a luz passou do alcance");
        // O mouse sai do form: a luz some.
        ui.pointer_leave(form);
        let gone = render_with(painter, &mut ui, form);
        assert_eq!(pixel(&far, 60, 60), pixel(&gone, 60, 60), "{painter}: a luz ficou depois de o mouse sair");
    }
}

#[test]
fn moving_the_mouse_repaints_only_near_the_light() {
    let (mut ui, form, _) = screen();
    mouse(&mut ui, form, 330.0, 160.0);
    let _ = render(&mut ui, form);
    let _ = ui.take_damage(form);
    ui.set_time(1010.0);
    let _ = render(&mut ui, form);
    let _ = ui.take_damage(form);
    // Fora do alcance de qualquer controle com reveal: nada marcado (o `take_frame` sem marca
    // nenhuma devolve `Whole`, e a plataforma nem pede o quadro).
    mouse(&mut ui, form, 320.0, 150.0);
    let d = ui.take_frame(form);
    assert!(!matches!(d, FrameDamage::Parts { .. }), "repintou sem luz por perto: {d:?}");
    // Perto do botão: só a parte dele que a luz alcança (o botão, mais a folga da borda e a margem
    // que o núcleo põe em toda área).
    mouse(&mut ui, form, 200.0, 60.0);
    match ui.take_frame(form) {
        FrameDamage::Parts { areas, .. } => {
            assert!(!areas.is_empty(), "a luz não pediu repintura");
            for a in &areas {
                assert!(a.x >= 34.0 && a.right() <= 166.0 && a.y >= 34.0 && a.bottom() <= 86.0, "repintou fora do botão: {a:?}");
                assert!(a.x >= 200.0 - 96.0 - 3.0, "repintou além do alcance: {a:?}");
            }
        }
        FrameDamage::Whole => panic!("repintou a janela inteira"),
    }
}


/// Um `Rectangle` de 120 × 80 em (40, 40), com fundo, o `radius` próprio e o estilo `reveal`
/// de um tema que só acende a borda e não tem raio (o raio fica no controle).
fn card(radius: PropValue) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let mut theme = ui.theme().clone();
    theme.styles.insert("Rectangle.reveal".into(), Style::new(StyleProps { reveal: Some("border".into()), border_width: Some(2.0), ..Default::default() }));
    ui.set_theme(theme);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    let rect = ui.create("Rectangle", Some(form)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 40.0.into()), ("top", 40.0.into()), ("width", 120.0.into()), ("height", 80.0.into()), ("fill", Color::rgb(30, 90, 200).into()), ("style", "reveal".into()), ("radius", radius)] {
        ui.set_prop(rect, prop, value).unwrap();
    }
    ui.show(form).unwrap();
    ui.set_time(1000.0);
    ui.layout();
    (ui, form, rect)
}

#[test]
fn the_light_follows_the_rectangle_own_radius() {
    let painters: &[&str] = if cfg!(windows) { &["Skia", "Direct2D"] } else { &["Skia"] };
    for &painter in painters {
        // O mesmo raio nos quatro cantos: a luz não passa da quina arredondada.
        let (mut ui, form, rect) = card(PropValue::from(16.0));
        assert_eq!(ui.component(rect).unwrap().painted_shape(), Some(ClipShape::RoundRect(16.0)));
        assert_eq!(ui.component(rect).unwrap().clip_shape(), None, "o raio do Reveal não recorta os filhos");
        mouse(&mut ui, form, 330.0, 190.0);
        let far = render_with(painter, &mut ui, form);
        mouse(&mut ui, form, 100.0, 43.0);
        let near = render_with(painter, &mut ui, form);
        assert_ne!(pixel(&far, 100, 41), pixel(&near, 100, 41), "{painter}: a borda perto do mouse não acendeu");
        assert_eq!(pixel(&far, 41, 41), pixel(&near, 41, 41), "{painter}: a luz saiu da quina arredondada (esquerda)");
        assert_eq!(pixel(&far, 158, 41), pixel(&near, 158, 41), "{painter}: a luz saiu da quina arredondada (direita)");

        // Um raio por canto (`[l, t, r, b]`: só o de cima à esquerda): a quina reta acende.
        let (mut ui, form, rect) = card(PropValue::from(Bounds::new(16.0, 0.0, 0.0, 0.0)));
        assert!(matches!(ui.component(rect).unwrap().painted_shape(), Some(ClipShape::Corners(_))));
        mouse(&mut ui, form, 330.0, 190.0);
        let far = render_with(painter, &mut ui, form);
        mouse(&mut ui, form, 100.0, 43.0);
        let near = render_with(painter, &mut ui, form);
        assert_eq!(pixel(&far, 41, 41), pixel(&near, 41, 41), "{painter}: a luz saiu do canto arredondado");
        assert_ne!(pixel(&far, 158, 41), pixel(&near, 158, 41), "{painter}: o canto reto não acendeu");
    }
}

/// O `Panel` (um `Rectangle` por composição) e o `Button` com `radius` dizem o raio também.
#[test]
fn the_panel_and_the_button_tell_their_own_radius() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    let panel = ui.create("Panel", Some(form)).unwrap();
    assert_eq!(ui.component(panel).unwrap().painted_shape(), None, "sem radius: o do estilo");
    ui.set_prop(panel, "radius", 12.0).unwrap();
    assert_eq!(ui.component(panel).unwrap().painted_shape(), Some(ClipShape::RoundRect(12.0)));
    let button = ui.create("Button", Some(form)).unwrap();
    ui.set_prop(button, "radius", Bounds::new(8.0, 8.0, 0.0, 0.0)).unwrap();
    assert_eq!(ui.component(button).unwrap().painted_shape(), Some(ClipShape::Corners(Bounds::new(8.0, 8.0, 0.0, 0.0))));
}
