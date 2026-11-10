//! Componentes do Material 3 e do iOS: comportamento e o vidro (Liquid Glass).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Canvas que conta os desfoques e guarda os textos.
#[derive(Default)]
struct Probe {
    blurs: Vec<RectF>,
    texts: Vec<String>,
}

impl Canvas for Probe {
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
    fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * 8.0, 16.0)
    }
    fn backdrop_blur(&mut self, rect: RectF, _: f32, _: f32) -> bool {
        self.blurs.push(rect);
        true
    }
}

fn setup(theme: &str) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    if let Some(t) = aylis_core::themes::builtin(theme) {
        ui.set_theme(t.clone());
    }
    let f = ui.add_root(Form::new("teste", 400.0, 800.0));
    (ui, f)
}

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
}

fn paint(ui: &mut Ui, f: ComponentId) -> Probe {
    let mut p = Probe::default();
    ui.layout();
    ui.paint(f, &mut p);
    p
}

fn click(ui: &mut Ui, f: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_move(f, e);
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            EventArgs::Value(v) => format!("{v:?}"),
            other => format!("{other:?}"),
        })
    });
    l
}

#[test]
fn liquid_glass_blurs_behind_the_ios26_controls() {
    let (mut ui, f) = setup("iOS 26 Light");
    let b = ui.add(Button::new("Ok"), f);
    place(&mut ui, b, 10.0, 10.0, 100.0, 40.0);
    let flat = ui.add(Button::new("Link"), f);
    place(&mut ui, flat, 10.0, 60.0, 100.0, 40.0);
    let _ = ui.set_prop(flat, "style", "flat");
    let p = paint(&mut ui, f);
    assert_eq!(p.blurs, vec![RectF::new(0.0, 0.0, 100.0, 40.0)], "só o botão de vidro desfoca o fundo");
    assert_eq!(ui.theme().resolve(&["Button"], "", ControlState::default()).material, Material::Glass);
    // No Material 3, nada de vidro; no tema do aylis, só com o subtipo.
    let (mut ui, f) = setup("Material 3 Light");
    let b = ui.add(Button::new("Ok"), f);
    place(&mut ui, b, 10.0, 10.0, 100.0, 40.0);
    assert!(paint(&mut ui, f).blurs.is_empty());
    let g = ui.create("Button.glass", Some(f)).unwrap();
    place(&mut ui, g, 10.0, 60.0, 100.0, 40.0);
    assert_eq!(paint(&mut ui, f).blurs.len(), 1);
}

#[test]
fn app_bar_navigation_and_actions() {
    for theme in ["Material 3 Light", "iOS 26 Light", "iOS 13 Light"] {
        let (mut ui, f) = setup(theme);
        let bar = ui.add(AppBar::default(), f);
        let navs = log(&mut ui, bar, "on_nav");
        let acts = log(&mut ui, bar, "on_action");
        let p = paint(&mut ui, f);
        assert!(p.texts.contains(&"Título".to_string()), "{theme}");
        click(&mut ui, f, 26.0, 26.0);
        click(&mut ui, f, 400.0 - 30.0, 26.0);
        assert_eq!(navs.borrow().len(), 1, "{theme}");
        assert_eq!(acts.borrow().as_slice(), ["Int(1)"], "{theme}: a última ação (menu)");
    }
}

#[test]
fn rail_side_sheet_page_control_and_up_down() {
    let (mut ui, f) = setup("Material 3 Light");
    let rail = ui.add(NavigationRail::default(), f);
    let changes = log(&mut ui, rail, "on_change");
    paint(&mut ui, f);
    // FAB em cima (16 + 56 + 28), depois os itens de 68 em 68.
    click(&mut ui, f, 44.0, 100.0 + 68.0 + 16.0);
    assert_eq!(changes.borrow().as_slice(), ["Int(1)"]);

    let sheet = ui.add(SideSheet::default(), f);
    let closed = log(&mut ui, sheet, "on_close");
    paint(&mut ui, f);
    click(&mut ui, f, 400.0 - 32.0, 36.0);
    assert_eq!(closed.borrow().len(), 1);
    assert!(!ui.control(sheet).unwrap().visible());

    let pc = ui.add(PageControl::default(), f);
    place(&mut ui, pc, 100.0, 700.0, 140.0, 28.0);
    ui.layout();
    click(&mut ui, f, 230.0, 714.0);
    click(&mut ui, f, 230.0, 714.0);
    assert_eq!(ui.get::<PageControl>(pc).unwrap().index, 2);

    let ud = ui.add(UpDown::default(), f);
    place(&mut ui, ud, 100.0, 600.0, 94.0, 32.0);
    let values = log(&mut ui, ud, "on_change");
    ui.layout();
    click(&mut ui, f, 180.0, 616.0);
    click(&mut ui, f, 110.0, 616.0);
    click(&mut ui, f, 110.0, 616.0);
    click(&mut ui, f, 110.0, 616.0);
    assert_eq!(ui.get::<UpDown>(ud).unwrap().value, 0.0, "não passa do mínimo");
    assert_eq!(values.borrow().len(), 3, "o último clique (no mínimo) não muda");
}

#[test]
fn wheel_picker_snaps_to_the_middle_row() {
    let (mut ui, f) = setup("iOS 26 Light");
    let w = ui.add(WheelPicker::default(), f);
    place(&mut ui, w, 0.0, 0.0, 200.0, 180.0);
    let changes = log(&mut ui, w, "on_change");
    ui.layout();
    // Arrasta duas linhas para cima: dezembro (o 9 + 2 = 11).
    let e = |y: f32| PointerEvent { position: PointF::new(100.0, y), ..Default::default() };
    ui.pointer_move(f, e(90.0));
    ui.pointer_down(f, e(90.0));
    ui.pointer_move(f, e(90.0 - 34.0 * 2.2));
    ui.pointer_up(f, e(90.0 - 34.0 * 2.2));
    assert_eq!(ui.get::<WheelPicker>(w).unwrap().item_index, 11);
    assert_eq!(changes.borrow().as_slice(), ["Dezembro"]);
    // A roda do mouse anda uma linha.
    ui.wheel(f, WheelEvent { position: PointF::new(100.0, 90.0), delta: PointF::new(0.0, 40.0), ..Default::default() });
    assert_eq!(ui.get::<WheelPicker>(w).unwrap().item_index, 10);
}

#[test]
fn button_group_bottom_sheet_detents_and_action_sheet() {
    let (mut ui, f) = setup("Material 3 Light");
    let g = ui.add(ButtonGroup::default(), f);
    place(&mut ui, g, 0.0, 0.0, 302.0, 40.0);
    let _ = ui.set_prop(g, "multiple", true);
    paint(&mut ui, f);
    click(&mut ui, f, 20.0, 20.0);
    assert_eq!(ui.get::<ButtonGroup>(g).unwrap().selected, "Dia;Semana");

    // A folha com alturas: arrastar a alça encaixa na mais perto.
    let sheet = ui.add(BottomSheet::default(), f);
    let _ = ui.set_prop(sheet, "detents", "0.25;0.75");
    let _ = ui.set_prop(sheet, "sheet_height", 200.0);
    ui.layout();
    let top = ui.bounds(sheet).unwrap().y;
    let e = |y: f32| PointerEvent { position: PointF::new(200.0, y), ..Default::default() };
    ui.pointer_move(f, e(top + 10.0));
    ui.pointer_down(f, e(top + 10.0));
    ui.pointer_move(f, e(top - 300.0));
    ui.pointer_up(f, e(top - 300.0));
    ui.layout();
    assert_eq!(ui.bounds(sheet).unwrap().height, 600.0, "0.75 de 800");

    let picked = Rc::new(RefCell::new(None));
    let p2 = picked.clone();
    let id = action_sheet(&mut ui, "Foto", &["Compartilhar", "Apagar"], Some(1), "Cancelar", move |_, i| *p2.borrow_mut() = Some(i)).unwrap();
    paint(&mut ui, f);
    // Material: a folha fica no rodapé; a 2ª linha é "Apagar".
    let row = 52.0;
    let h = 56.0 + row * 2.0 + 16.0;
    click(&mut ui, f, 200.0, 800.0 - h + 56.0 + row * 1.5);
    assert_eq!(*picked.borrow(), Some(Some(1)));
    ui.process_posted();
    assert!(!ui.contains(id));
}

#[test]
fn gauges_rings_and_loading_paint_in_every_look() {
    for theme in ["Material 3 Dark", "iOS 26 Dark", "Windows 11 Light"] {
        let (mut ui, f) = setup(theme);
        for c in ["Gauge", "ActivityRings", "LoadingIndicator", "LoadingIndicator.contained", "AppBar.large", "NavigationRail", "PageControl", "UpDown", "WheelPicker", "ButtonGroup.standard", "Button.fab_extended", "Button.icon_tonal", "Edit.filled", "ToolBar.floating"] {
            let id = ui.create(c, Some(f)).unwrap_or_else(|e| panic!("{c}: {e:?}"));
            let _ = id;
        }
        let p = paint(&mut ui, f);
        assert!(!p.texts.is_empty(), "{theme}");
    }
}
