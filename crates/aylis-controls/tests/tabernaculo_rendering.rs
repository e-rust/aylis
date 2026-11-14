//! O relatório do Tabernáculo, o que se confere pelos pixels: a mesma tela pintada no Skia e,
//! no Windows, no Direct2D (o Canvas 2D do navegador segue o mesmo contrato do `Canvas`).

use aylis_controls::*;
use aylis_core::*;

const W: u32 = 320;
const H: u32 = 200;

/// Pinta o form e devolve os pixels RGBA, num renderizador.
type Render = fn(&mut Ui, ComponentId) -> Vec<u8>;

fn skia(ui: &mut Ui, form: ComponentId) -> Vec<u8> {
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    renderer.render_rgba(W, H, 1.0, Color::WHITE, |c| ui.paint(form, c)).expect("Skia")
}

#[cfg(windows)]
fn d2d(ui: &mut Ui, form: ComponentId) -> Vec<u8> {
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    renderer.render_rgba(W, H, 1.0, Color::WHITE, |c| ui.paint(form, c)).expect("Direct2D")
}

fn renderers() -> Vec<(&'static str, Render)> {
    #[allow(unused_mut)]
    let mut list: Vec<(&'static str, Render)> = vec![("Skia", skia)];
    #[cfg(windows)]
    list.push(("Direct2D", d2d));
    list
}

fn pixel(rgba: &[u8], x: u32, y: u32) -> [u8; 3] {
    let i = ((y * W + x) * 4) as usize;
    [rgba[i], rgba[i + 1], rgba[i + 2]]
}

fn near(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 3)
}

/// O form branco, sem borda, com o tema claro.
fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", W as f32, H as f32));
    ui.set_prop(form, "fill", Color::WHITE).unwrap();
    ui.show(form).unwrap();
    (ui, form)
}

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) {
    for (prop, value) in [("align", PropValue::from("none")), ("left", x.into()), ("top", y.into()), ("width", w.into()), ("height", h.into())] {
        ui.set_prop(id, prop, value).unwrap();
    }
}

/// C08: o `fill` com `with_opacity(0.7)` deixa ver o que está atrás, pela propriedade, pelo
/// campo do Panel (`base.fill`) e pela cor própria de um Label.
#[test]
fn a_fill_with_opacity_lets_the_back_show() {
    let red = Color::rgb(255, 0, 0).with_opacity(0.7);
    // 0,7 de vermelho sobre branco: (255, 77, 77); sobre preto: (179, 0, 0).
    for (name, render) in renderers() {
        let (mut ui, form) = setup();
        let back = ui.add(Rectangle::default(), form);
        place(&mut ui, back, 0.0, 100.0, W as f32, 100.0);
        ui.set_prop(back, "fill", Color::BLACK).unwrap();
        ui.set_prop(back, "stroke_width", 0.0).unwrap();
        let by_prop = ui.add(Panel::default(), form);
        place(&mut ui, by_prop, 10.0, 80.0, 60.0, 60.0);
        ui.set_prop(by_prop, "fill", red).unwrap();
        let by_field = ui.add(Panel::default(), form);
        place(&mut ui, by_field, 110.0, 80.0, 60.0, 60.0);
        ui.get_mut::<Panel>(by_field).unwrap().base.fill = Some(red);
        let label = ui.add(Label::new(""), form);
        place(&mut ui, label, 210.0, 80.0, 60.0, 60.0);
        ui.set_prop(label, "fill", red).unwrap();
        let rgba = render(&mut ui, form);
        for (what, x) in [("propriedade", 40), ("campo", 140), ("Label", 240)] {
            assert!(near(pixel(&rgba, x, 90), [255, 77, 77]), "{name}, {what}, sobre o branco: {:?}", pixel(&rgba, x, 90));
            assert!(near(pixel(&rgba, x, 130), [179, 0, 0]), "{name}, {what}, sobre o preto: {:?}", pixel(&rgba, x, 130));
        }
    }
}

/// C08, com o app rodando: o painel pintado opaco passa a 0,7 pela propriedade e pelo campo
/// (com `ui.damage`), e o quadro seguinte, pintado só na área que mudou, mostra o de trás.
#[test]
fn a_fill_turned_translucent_repaints_what_is_behind() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let red = Color::rgb(255, 0, 0);
    for by_field in [false, true] {
        let (mut ui, form) = setup();
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
        let back = ui.add(Rectangle::default(), form);
        place(&mut ui, back, 0.0, 100.0, W as f32, 100.0);
        ui.set_prop(back, "fill", Color::BLACK).unwrap();
        ui.set_prop(back, "stroke_width", 0.0).unwrap();
        let panel = ui.add(Panel::default(), form);
        place(&mut ui, panel, 10.0, 80.0, 60.0, 60.0);
        ui.set_prop(panel, "fill", red).unwrap();
        ui.set_prop(panel, "radius", PropValue::Null).unwrap_or(());
        ui.layout();
        let mut pixels = vec![0u32; (W * H) as usize];
        renderer.render_area_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
        let _ = ui.take_damage(form);
        if by_field {
            ui.get_mut::<Panel>(panel).unwrap().base.fill = Some(red.with_opacity(0.7));
            ui.damage(panel);
        } else {
            ui.set_prop(panel, "fill", red.with_opacity(0.7)).unwrap();
        }
        // Até dois quadros (o primeiro pode só avisar que o painel deixou de cobrir).
        for _ in 0..2 {
            ui.layout();
            // `None`: a janela inteira.
            let area = ui.take_damage(form);
            renderer.render_area_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, area, |c| ui.paint_region(form, c, area));
        }
        let at = |x: u32, y: u32| {
            let p = pixels[(y * W + x) as usize];
            [(p >> 16) as u8, (p >> 8) as u8, p as u8]
        };
        assert!(near(at(40, 130), [179, 0, 0]), "campo {by_field}: sobre o preto {:?}", at(40, 130));
        assert!(near(at(40, 90), [255, 77, 77]), "campo {by_field}: sobre o branco {:?}", at(40, 90));
    }
}

/// C24: a opacidade do contêiner desce aos filhos (uma camada com o grupo
/// inteiro): o filho preto num painel a 0,5 sai cinza, e onde dois filhos se cobrem
/// não escurece mais (a camada é composta uma vez só).
#[test]
fn a_container_opacity_applies_to_its_children_as_a_group() {
    for (name, render) in renderers() {
        let (mut ui, form) = setup();
        let panel = ui.add(Layout::default(), form);
        place(&mut ui, panel, 20.0, 20.0, 200.0, 120.0);
        ui.set_prop(panel, "opacity", 0.5).unwrap();
        let a = ui.add(Rectangle::default(), panel);
        place(&mut ui, a, 0.0, 0.0, 100.0, 100.0);
        let b = ui.add(Rectangle::default(), panel);
        place(&mut ui, b, 50.0, 0.0, 100.0, 100.0);
        for r in [a, b] {
            ui.set_prop(r, "fill", Color::BLACK).unwrap();
            ui.set_prop(r, "stroke_width", 0.0).unwrap();
            ui.set_prop(r, "radius", Bounds::all(0.0)).unwrap();
        }
        let rgba = render(&mut ui, form);
        for x in [40, 90, 150] {
            assert!(near(pixel(&rgba, x, 60), [128, 128, 128]), "{name}: x {x} {:?}", pixel(&rgba, x, 60));
        }
    }
}

/// C24, com o app rodando: o filho que muda é repintado dentro da camada do pai (a pintura
/// por parte aplica a opacidade dos contêineres de cima), e o pai que fica translúcido
/// depois repinta os filhos.
#[test]
fn a_child_repainted_alone_keeps_the_container_opacity() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let (mut ui, form) = setup();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let panel = ui.add(Layout::default(), form);
    place(&mut ui, panel, 20.0, 20.0, 200.0, 120.0);
    let child = ui.add(Rectangle::default(), panel);
    place(&mut ui, child, 10.0, 10.0, 100.0, 100.0);
    ui.set_prop(child, "fill", Color::BLACK).unwrap();
    ui.set_prop(child, "stroke_width", 0.0).unwrap();
    ui.layout();
    let mut pixels = vec![0u32; (W * H) as usize];
    let mut frame = |ui: &mut Ui, pixels: &mut Vec<u32>| {
        ui.layout();
        let area = ui.take_damage(form);
        renderer.render_area_to_pixels(pixels, W, H, 1.0, Color::WHITE, area, |c| ui.paint_region(form, c, area));
    };
    frame(&mut ui, &mut pixels);
    let at = |pixels: &[u32]| {
        let p = pixels[(80 * W + 80) as usize];
        [(p >> 16) as u8, (p >> 8) as u8, p as u8]
    };
    assert!(near(at(&pixels), [0, 0, 0]));
    ui.set_prop(panel, "opacity", 0.5).unwrap();
    frame(&mut ui, &mut pixels);
    assert!(near(at(&pixels), [128, 128, 128]), "o pai ficou translúcido: {:?}", at(&pixels));
    ui.set_prop(child, "fill", Color::rgb(0, 0, 255)).unwrap();
    frame(&mut ui, &mut pixels);
    assert!(near(at(&pixels), [128, 128, 255]), "o filho mudou sozinho: {:?}", at(&pixels));
}

/// C41: o quadro escolhido do `.ico` da IDE (sete tamanhos, de 16 a 256) é lido pelos
/// decodificadores no tamanho dele: o da bandeja a 150% é o de 24 ou o maior mais perto.
#[test]
fn the_ico_frame_for_the_tray_decodes_at_its_size() {
    let file = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../aylis-ide/resources/aylis-ide.ico")).unwrap();
    for (size, want) in [(16, 16), (24, 24), (32, 32), (40, 48), (512, 256)] {
        let one = aylis_core::image::ico_frame_for_size(&file, size).unwrap();
        let (_, w, h) = aylis_skia::decode_rgba(&one).expect("Skia lê o .ico de um quadro");
        assert_eq!((w, h), (want, want), "Skia, {size} pixels");
        #[cfg(windows)]
        {
            let (_, w, h) = aylis_d2d::decode_rgba(&one).expect("WIC lê o .ico de um quadro");
            assert_eq!((w, h), (want, want), "WIC, {size} pixels");
        }
    }
}

/// C17: as abas do lado escolhido (`tab_position`). As páginas ficam no resto; o clique na
/// coluna da esquerda escolhe a aba, as setas de cima e de baixo trocam; o estilo
/// `TabControl.tab.left` do tema pinta a aba (a escolhida pelo `checked`).
#[test]
fn tabs_on_each_side() {
    use aylis_core::theme::{ColorRef, Style, StyleProps};
    let green = Color::rgb(0, 200, 0);
    for (name, render) in renderers() {
        let (mut ui, form) = setup();
        let mut theme = ui.theme().clone();
        theme.set_style("TabControl.tab", Style::new(StyleProps::default()));
        let mut left = Style::new(StyleProps::default());
        left.checked = StyleProps { fill: Some(ColorRef::Value(green)), ..Default::default() };
        theme.set_style("TabControl.tab.left", left);
        ui.set_theme(theme);
        let tabs = ui.add(TabControl::default(), form);
        place(&mut ui, tabs, 0.0, 0.0, W as f32, H as f32);
        let pages: Vec<ComponentId> = ["Geral", "Saídas", "Áudio"].iter().map(|c| ui.add(TabItem::new(c), tabs)).collect();
        TabControl::show(&mut ui, tabs);
        for (position, page) in [("top", RectF::new(1.0, 38.0, W as f32 - 2.0, H as f32 - 39.0)), ("bottom", RectF::new(1.0, 1.0, W as f32 - 2.0, H as f32 - 39.0)), ("right", RectF::new(1.0, 1.0, W as f32 - 101.0, H as f32 - 2.0)), ("left", RectF::new(100.0, 1.0, W as f32 - 101.0, H as f32 - 2.0))] {
            ui.set_prop(tabs, "tab_position", position).unwrap();
            ui.set_prop(tabs, "tab_width", 100.0).unwrap();
            ui.layout();
            assert_eq!(ui.absolute_rect(pages[0]), Some(page), "{name}: a página com as abas em {position}");
        }
        let rgba = render(&mut ui, form);
        // A primeira aba (a escolhida) com o fundo do estilo; a segunda, sem.
        assert!(near(pixel(&rgba, 50, 6), [0, 200, 0]), "{name}: a aba escolhida {:?}", pixel(&rgba, 50, 6));
        assert!(!near(pixel(&rgba, 50, 6 + 36), [0, 200, 0]), "{name}: a outra aba");
        // O clique na segunda aba (a coluna à esquerda, uma aba abaixo).
        ui.pointer_down(form, PointerEvent { position: PointF::new(50.0, 4.0 + 36.0 + 10.0), ..Default::default() });
        ui.pointer_up(form, PointerEvent { position: PointF::new(50.0, 4.0 + 36.0 + 10.0), ..Default::default() });
        assert_eq!(ui.get_prop(tabs, "tab_index").unwrap(), PropValue::Int(1), "{name}: o clique na coluna");
        // Fora da coluna, nas páginas: nada muda.
        ui.pointer_down(form, PointerEvent { position: PointF::new(200.0, 4.0 + 72.0 + 10.0), ..Default::default() });
        ui.pointer_up(form, PointerEvent { position: PointF::new(200.0, 4.0 + 72.0 + 10.0), ..Default::default() });
        assert_eq!(ui.get_prop(tabs, "tab_index").unwrap(), PropValue::Int(1));
        ui.set_focus(tabs);
        ui.key_down(form, KeyEvent { key: Key::Down, ..Default::default() });
        assert_eq!(ui.get_prop(tabs, "tab_index").unwrap(), PropValue::Int(2), "{name}: a seta de baixo");
        assert!(ui.control(pages[2]).unwrap().visible() && !ui.control(pages[1]).unwrap().visible());
    }
}

/// C38: com `gap` 0 no estilo, os botões do `ButtonGroup` se encostam e a marcação do
/// `SegmentedControl` vai até a borda (como os botões colados do HTML); sem o tema, como antes
/// (2 e 3 pixels).
#[test]
fn a_zero_gap_glues_the_parts() {
    use aylis_core::automation::AccessItem;
    use aylis_core::theme::{Style, StyleProps};
    for glued in [false, true] {
        let (mut ui, form) = setup();
        if glued {
            let mut theme = ui.theme().clone();
            for class in ["ButtonGroup", "SegmentedControl"] {
                let mut style = theme.styles.get(class).cloned().unwrap_or_else(|| Style::new(StyleProps::default()));
                style.base.gap = Some(0.0);
                theme.set_style(class, style);
            }
            ui.set_theme(theme);
        }
        let group = ui.add(ButtonGroup::default(), form);
        place(&mut ui, group, 10.0, 10.0, 300.0, 40.0);
        let seg = ui.add(SegmentedControl::default(), form);
        place(&mut ui, seg, 10.0, 100.0, 300.0, 40.0);
        let rgba = skia(&mut ui, form);
        let rects: Vec<RectF> = ui.access_items(group).iter().filter_map(|i: &AccessItem| i.rect).collect();
        let space = rects[1].x - rects[0].right();
        assert_eq!(space, if glued { 0.0 } else { 2.0 }, "o espaço entre os botões");
        // A marcação (o "surface" do tema) na beirada de dentro do segmento escolhido.
        let surface = ui.theme().color("surface");
        let edge = pixel(&rgba, 11, 120);
        assert_eq!(near(edge, [surface.r, surface.g, surface.b]), glued, "a marcação na borda: {edge:?}");
    }
}
