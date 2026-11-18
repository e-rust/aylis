//! A forma e a transparência do form (`window_shape`, `shape_radius`, `transparent`): o fundo e
//! os filhos ficam dentro da forma, com a borda suave, e fora dela a pintura não toca (a janela
//! transparente deixa ver o que está atrás).

use aylis_core::*;

const W: u32 = 200;
const H: u32 = 100;

/// Um form de 200 × 100 com a forma `shape`, fundo `fill` e um painel azul do tamanho dele.
fn shaped_form(shape: &str, fill: Color) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.set_prop(form, "window_shape", shape).unwrap();
    ui.set_prop(form, "shape_radius", 30.0).unwrap();
    ui.set_prop(form, "fill", fill).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    (ui, form)
}

/// Os pixels RGBA pintados sobre o transparente (como a janela transparente começa).
fn render(painter: &str, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
    match painter {
        #[cfg(windows)]
        "Direct2D" => aylis_d2d::D2dRenderer::new().unwrap().render_rgba(W, H, 1.0, Color::TRANSPARENT, |c| ui.paint_region(form, c, None)).unwrap(),
        _ => aylis_skia::SkiaRenderer::new().render_rgba(W, H, 1.0, Color::TRANSPARENT, |c| ui.paint_region(form, c, None)).unwrap(),
    }
}

fn painters() -> Vec<&'static str> {
    let mut list = vec!["Skia"];
    if cfg!(windows) {
        list.push("Direct2D");
    }
    list
}

fn alpha(rgba: &[u8], x: u32, y: u32) -> u8 {
    rgba[((y * W + x) * 4 + 3) as usize]
}

#[test]
fn the_shape_maps_to_the_clip() {
    let (ui, form) = shaped_form("ellipse", Color::rgb(200, 0, 0));
    assert_eq!(ui.component(form).unwrap().clip_shape(), Some(ClipShape::Ellipse));
    let (ui, form) = shaped_form("rounded", Color::rgb(200, 0, 0));
    assert_eq!(ui.component(form).unwrap().clip_shape(), Some(ClipShape::RoundRect(30.0)));
    let (ui, form) = shaped_form("rectangle", Color::rgb(200, 0, 0));
    assert_eq!(ui.component(form).unwrap().clip_shape(), None);
    // O raio não passa da metade do lado menor (o retângulo vira uma pílula, não some).
    assert_eq!(ClipShape::RoundRect(500.0).radius(RectF::new(0.0, 0.0, 200.0, 100.0)), 50.0);
}

/// A elipse: o canto fica transparente, o centro opaco e a borda tem pixels intermediários
/// (a suavização), também com um filho que cobre o form inteiro.
#[test]
fn an_ellipse_form_paints_only_inside_with_a_smooth_edge() {
    for painter in painters() {
        let (mut ui, form) = shaped_form("ellipse", Color::rgb(200, 0, 0));
        let panel = ui.create("Panel", Some(form)).unwrap();
        ui.set_prop(panel, "align", "client").unwrap();
        ui.set_prop(panel, "fill", Color::rgb(0, 0, 200)).unwrap();
        ui.layout();
        let rgba = render(painter, &mut ui, form);
        assert_eq!(alpha(&rgba, 2, 2), 0, "{painter}: o canto ficou pintado (o filho saiu da forma)");
        assert_eq!(alpha(&rgba, W - 3, H - 3), 0, "{painter}: o canto de baixo ficou pintado");
        assert_eq!(alpha(&rgba, W / 2, H / 2), 255, "{painter}: o centro não é opaco");
        let center = ((H / 2 * W + W / 2) * 4) as usize;
        assert!(rgba[center + 2] > 150 && rgba[center] < 50, "{painter}: o filho não aparece no centro: {:?}", &rgba[center..center + 4]);
        // Ao longo da borda há pixels nem opacos nem transparentes.
        let soft = (0..W).flat_map(|x| (0..H).map(move |y| (x, y))).filter(|&(x, y)| (1..255).contains(&alpha(&rgba, x, y))).count();
        assert!(soft > 50, "{painter}: a borda não é suave ({soft} pixels intermediários)");
    }
}

/// Os cantos arredondados: o canto de fora transparente, o meio da borda de cima opaco.
#[test]
fn a_rounded_form_cuts_the_corners() {
    for painter in painters() {
        let (mut ui, form) = shaped_form("rounded", Color::rgb(0, 120, 0));
        let rgba = render(painter, &mut ui, form);
        assert_eq!(alpha(&rgba, 1, 1), 0, "{painter}: o canto ficou pintado");
        assert_eq!(alpha(&rgba, W / 2, 1), 255, "{painter}: a borda de cima, longe do canto");
        assert_eq!(alpha(&rgba, 1, H / 2), 255, "{painter}: a borda da esquerda, longe do canto");
    }
}

/// A semitransparência: o fundo com alfa sai com o alfa (o vidro fosco, com o desfoque da janela
/// por trás).
#[test]
fn a_translucent_fill_keeps_its_alpha() {
    for painter in painters() {
        let (mut ui, form) = shaped_form("rectangle", Color::rgba(255, 255, 255, 70));
        let rgba = render(painter, &mut ui, form);
        let a = alpha(&rgba, W / 2, H / 2);
        assert!((65..=75).contains(&a), "{painter}: alfa {a}, esperado ~70");
    }
}

/// Mudar a forma ou a transparência com o app rodando pede à janela que se refaça.
#[test]
fn changing_the_shape_asks_the_window() {
    let (mut ui, form) = shaped_form("rectangle", Color::WHITE);
    let _ = ui.take_window_commands();
    let props = [
        ("window_shape", PropValue::from("ellipse")),
        ("shape_radius", 8.0.into()),
        ("shape_radii", "8,8,0,0".into()),
        ("shape_path", "M 0 0 L 10 0 L 5 10 Z".into()),
        ("transparent", true.into()),
    ];
    for (prop, value) in props {
        ui.set_prop(form, prop, value).unwrap();
        assert!(ui.take_window_commands().contains(&(form, WindowCommand::UpdateCorners)), "{prop}");
    }
}


/// Um raio por canto (`shape_radii`): o canto de 30 fica transparente, o de 0 fica pintado.
#[test]
fn each_corner_has_its_own_radius() {
    for painter in painters() {
        let (mut ui, form) = shaped_form("rounded", Color::rgb(0, 120, 0));
        ui.set_prop(form, "shape_radii", "30,30,30,0").unwrap();
        assert!(matches!(ui.component(form).unwrap().clip_shape(), Some(ClipShape::Corners(_))));
        let rgba = render(painter, &mut ui, form);
        assert_eq!(alpha(&rgba, 1, 1), 0, "{painter}: cima-esquerda (30)");
        assert_eq!(alpha(&rgba, W - 2, H - 2), 0, "{painter}: baixo-direita (30)");
        assert_eq!(alpha(&rgba, 1, H - 2), 255, "{painter}: baixo-esquerda (0) ficou recortado");
    }
}

/// Um contorno qualquer (`window_shape = path`): um losango em outra escala, esticado para o
/// form. Os cantos transparentes, o centro opaco e a borda suave; contorno inválido, o retângulo.
#[test]
fn a_path_shape_is_stretched_to_the_form() {
    for painter in painters() {
        let (mut ui, form) = shaped_form("path", Color::rgb(200, 0, 0));
        ui.set_prop(form, "shape_path", "M 5 0 L 10 5 L 5 10 L 0 5 Z").unwrap();
        assert!(matches!(ui.component(form).unwrap().clip_shape(), Some(ClipShape::Path(_))));
        let rgba = render(painter, &mut ui, form);
        for (x, y) in [(3, 3), (W - 4, 3), (3, H - 4), (W - 4, H - 4)] {
            assert_eq!(alpha(&rgba, x, y), 0, "{painter}: o canto ({x}, {y}) ficou pintado");
        }
        assert_eq!(alpha(&rgba, W / 2, H / 2), 255, "{painter}: o centro");
        assert_eq!(alpha(&rgba, W / 2, 3), 255, "{painter}: a ponta de cima");
        let soft = (0..W).flat_map(|x| (0..H).map(move |y| (x, y))).filter(|&(x, y)| (1..255).contains(&alpha(&rgba, x, y))).count();
        assert!(soft > 100, "{painter}: a borda não é suave ({soft})");
    }
    let (mut ui, form) = shaped_form("path", Color::WHITE);
    ui.set_prop(form, "shape_path", "isto não é um caminho").unwrap();
    assert_eq!(ui.component(form).unwrap().clip_shape(), None, "inválido: o retângulo");
}
