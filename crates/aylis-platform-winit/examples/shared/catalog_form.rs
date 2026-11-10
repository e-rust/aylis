//! As telas do exemplo `catalogo` (Material 3 e iOS 26 lado a lado), usadas também pelo
//! demo do navegador (`?vidro`).

use aylis_controls::*;
use aylis_core::*;

const W: f32 = 390.0;
const H: f32 = 844.0;

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn top(ui: &mut Ui, parent: ComponentId, id: ComponentId, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_align(Align::Top).set_height(h).set_margins([0.0, 0.0, 0.0, 10.0]);
    let _ = parent;
    id
}

/// Uma linha com controles lado a lado (largura fixa cada).
fn row(ui: &mut Ui, parent: ComponentId, h: f32) -> ComponentId {
    let r = ui.add(Layout::default(), parent);
    top(ui, parent, r, h)
}

fn left(ui: &mut Ui, id: ComponentId, w: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_align(Align::Left).set_width(w).set_margins([0.0, 0.0, 10.0, 0.0]);
    id
}

/// Fotos coloridas (para o vidro ter o que desfocar).
fn photos(ui: &mut Ui, parent: ComponentId, h: f32) {
    let grid = ui.add(GridLayout::default(), parent);
    top(ui, parent, grid, h);
    let _ = ui.set_prop(grid, "columns", 3i64);
    let _ = ui.set_prop(grid, "gap", 4.0);
    let colors = ["#FF6B6B", "#4ECDC4", "#FFD93D", "#6C5CE7", "#00B894", "#FD79A8", "#0984E3", "#E17055", "#A29BFE"];
    for c in colors {
        let r = ui.add(Rectangle::default(), grid);
        ui.control_mut(r).unwrap().set_size(10.0, (h - 8.0) / 3.0);
        let _ = ui.set_prop(r, "fill", Color::from_hex(c).unwrap());
        let _ = ui.set_prop(r, "radius", 6);
    }
}

fn phone(ui: &mut Ui, form: ComponentId, theme: &str, x: f32, second: bool) {
    let bg = ui.theme_named(theme).map_or(Color::WHITE, |t| t.color("background"));
    let screen = ui.add(Rectangle::default(), form);
    put(ui, screen, x, 20.0, W, H);
    let _ = ui.set_prop(screen, "fill", bg);
    let _ = ui.set_prop(screen, "radius", 44);
    let _ = ui.set_prop(screen, "theme", theme);
    ui.control_mut(screen).unwrap().set_clip_children(true);
    let ios = theme.starts_with("iOS");

    // Barra do topo.
    let bar = ui.add(AppBar::default(), screen);
    let _ = ui.set_prop(bar, "size", "large");
    let _ = ui.set_prop(bar, "title", if second { "Ajustes" } else { "Fotos" });
    let _ = ui.set_prop(bar, "nav_icon", if second { "arrow-left" } else { "" });
    let _ = ui.set_prop(bar, "back_text", if ios { "Voltar" } else { "" });
    let _ = ui.set_prop(bar, "actions", "search;plus");
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(AppBar::height_for(AppBarSize::Large, if ios { Look::Ios26 } else { Look::Material3 }));
    ui.control_mut(screen).unwrap().set_padding([0.0, 30.0, 0.0, 0.0]);

    let body = ui.add(Layout::default(), screen);
    ui.control_mut(body).unwrap().set_align(Align::Client).set_padding([16.0, 6.0, 16.0, 0.0]);

    if !second {
        let search = ui.create("Edit.search", Some(body)).unwrap();
        top(ui, body, search, 40.0);
        let _ = ui.set_prop(search, "placeholder", "Buscar");
        let _ = ui.set_prop(search, "icon", "search");
        let seg = ui.add(SegmentedControl::new("Anos;Meses;Dias;Todas"), body);
        top(ui, body, seg, 34.0);
        let _ = ui.set_prop(seg, "item_index", 3i64);
        photos(ui, body, 250.0);
        // Botões: os do Material 3 ou os de vidro do iOS.
        let r1 = row(ui, body, 40.0);
        let styles: &[(&str, &str)] = if ios { &[("", "Editar"), ("glass_prominent", "Compartilhar")] } else { &[("", "Salvar"), ("tonal", "Tonal"), ("outline", "Contorno")] };
        for (style, text) in styles {
            let b = ui.add(Button::new(*text), r1);
            left(ui, b, if ios { 150.0 } else { 104.0 });
            let _ = ui.set_prop(b, "style", *style);
        }
        let r2 = row(ui, body, 34.0);
        let s = ui.add(Switch::new(""), r2);
        left(ui, s, 64.0);
        let _ = ui.set_prop(s, "checked", true);
        let t = ui.add(TrackBar::new(60.0, 0.0, 100.0), r2);
        ui.control_mut(t).unwrap().set_align(Align::Client);
        let r3 = row(ui, body, 36.0);
        let ud = ui.add(UpDown::default(), r3);
        left(ui, ud, 94.0);
        let bg = ui.add(ButtonGroup::default(), r3);
        ui.control_mut(bg).unwrap().set_align(Align::Client);
        let r4 = row(ui, body, 56.0);
        let li = ui.add(LoadingIndicator::default(), r4);
        left(ui, li, 52.0);
        let li2 = ui.create("LoadingIndicator.contained", Some(r4)).unwrap();
        left(ui, li2, 52.0);
        let g = ui.add(Gauge::default(), r4);
        left(ui, g, 56.0);
        let rings = ui.add(ActivityRings::default(), r4);
        left(ui, rings, 56.0);
        let pc = ui.add(PageControl::default(), r4);
        ui.control_mut(pc).unwrap().set_align(Align::Client);
    } else {
        // Segunda tela: roda de opções, chips, cartões e a folha.
        let wheel = ui.add(WheelPicker::default(), body);
        top(ui, body, wheel, 170.0);
        let flow = ui.add(FlowLayout::default(), body);
        top(ui, body, flow, 80.0);
        for (t, checked) in [("Wi-Fi", true), ("Bluetooth", false), ("Avião", false), ("Foco", true)] {
            let c = ui.add(Chip::new(t), flow);
            ui.control_mut(c).unwrap().set_size(100.0, 32.0);
            let _ = ui.set_prop(c, "checked", checked);
        }
        for (style, title) in [("card", "Elevado / vidro"), ("filled", "Preenchido"), ("outlined", "Com contorno")] {
            let p = ui.add(Panel::default(), body);
            top(ui, body, p, 64.0);
            let _ = ui.set_prop(p, "style", style);
            ui.control_mut(p).unwrap().set_padding(14.0);
            let l = ui.add(Label::new(title), p);
            ui.control_mut(l).unwrap().set_align(Align::Client);
        }
        let ef = ui.create("Edit.filled", Some(body)).unwrap();
        top(ui, body, ef, 56.0);
        let _ = ui.set_prop(ef, "label", "Nome");
        let _ = ui.set_prop(ef, "text", "Eduardo");
    }

    // Fotos rolando por baixo da barra (o vidro desfoca o que passa atrás).
    let strip = ui.add(GridLayout::default(), screen);
    put(ui, strip, 0.0, H - 150.0, W, 120.0);
    let _ = ui.set_prop(strip, "columns", 4i64);
    let _ = ui.set_prop(strip, "gap", 3.0);
    for c in ["#E17055", "#00B894", "#6C5CE7", "#FDCB6E", "#0984E3", "#FD79A8", "#2D3436", "#55EFC4"] {
        let r = ui.add(Rectangle::default(), strip);
        ui.control_mut(r).unwrap().set_size(10.0, 58.0);
        let _ = ui.set_prop(r, "fill", Color::from_hex(c).unwrap());
    }
    // Tab bar / navigation bar: por cima do conteúdo, embaixo.
    let nav = ui.add(BottomNavigation::default(), screen);
    ui.control_mut(nav).unwrap().set_align(Align::None).set_rect(0.0, H - (if ios { 92.0 } else { 80.0 }) - 30.0, W, if ios { 74.0 } else { 80.0 });
    let _ = ui.set_prop(nav, "items", "Biblioteca;Para você;Álbuns;Buscar");
    let _ = ui.set_prop(nav, "icons", "image;star;grid;search");
    if !ios {
        let fab = ui.create("Button.fab_extended", Some(screen)).unwrap();
        put(ui, fab, W - 160.0, H - 236.0, 140.0, 56.0);
    }
}

/// Monta o form com os dois "celulares". `second`: a 2ª tela de cada.
pub fn build(dark: bool, second: bool) -> Result<(Ui, ComponentId), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — Material 3 e iOS 26", 2.0 * W + 60.0, H + 40.0));
    ui.set_name(form, "frmCatalogo")?;
    ui.set_prop(form, "fill", Color::rgb(0x5A, 0x60, 0x6B)).ok();
    let (m3, ios) = if dark { ("Material 3 Dark", "iOS 26 Dark") } else { ("Material 3 Light", "iOS 26 Light") };
    phone(&mut ui, form, m3, 20.0, second);
    phone(&mut ui, form, ios, W + 40.0, second);
    Ok((ui, form))
}
