//! A área de trabalho fora das janelas e o que ela pede no núcleo: o mouse na tela, o form sob
//! um ponto, a transparência do sistema, a captura de um monitor, o `border_color: none`, o
//! form da tecla e o `active_form`.

use std::cell::Cell;
use std::rc::Rc;

use aylis_core::monitors::Monitor;
use aylis_core::*;

/// Um form qualquer.
#[derive(Default)]
struct Block {
    base: ControlBase,
}

impl Component for Block {
    fn class_name(&self) -> &'static str {
        "Block"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }
}

/// A plataforma de mentira: o mouse num ponto, um form sob ele e a captura que devolve um quadrado.
struct FakeDesktop {
    cursor: Option<(i32, i32)>,
    form: Option<ComponentId>,
    transparency: Option<bool>,
}

impl DesktopService for FakeDesktop {
    fn cursor_position(&self) -> Option<(i32, i32)> {
        self.cursor
    }
    fn form_at(&self, _x: i32, _y: i32) -> Option<ComponentId> {
        self.form
    }
    fn transparency(&self) -> Option<bool> {
        self.transparency
    }
    fn capture_monitor(&mut self, area: (i32, i32, u32, u32), max_width: u32, done: Box<dyn FnOnce(Result<Image, String>) + Send>) {
        let w = max_width.min(area.2).max(1);
        let h = (area.3 as u64 * w as u64 / area.2.max(1) as u64).max(1) as u32;
        done(Ok(Image::rgba(w, h, vec![255; w as usize * h as usize * 4])));
    }
}

fn monitors() -> Vec<Monitor> {
    let m = |device: &str, x: i32, scale: f64, primary: bool| Monitor {
        name: device.into(),
        device: device.into(),
        device_path: format!("path-{device}"),
        primary,
        x,
        y: 0,
        width: 3840,
        height: 2160,
        work_x: x,
        work_y: 0,
        work_width: 3840,
        work_height: 2100,
        scale,
        refresh_hz: 60.0,
        modes: Vec::new(),
    };
    vec![m("A", 0, 1.5, true), m("B", 3840, 2.0, false)]
}

fn ui_with(desktop: FakeDesktop) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    let form = ui.add_root(Block::default());
    ui.show(form).unwrap();
    ui.set_monitors(monitors());
    ui.services_mut().desktop = Some(Box::new(desktop));
    (ui, form)
}

#[test]
fn the_cursor_comes_in_pixels_in_dp_and_with_its_monitor() {
    let (ui, _) = ui_with(FakeDesktop { cursor: Some((3840 + 400, 200)), form: None, transparency: None });
    let c = ui.cursor_position().unwrap();
    assert_eq!((c.x, c.y, c.monitor), (4240, 200, Some(1)));
    assert_eq!((c.dp_x, c.dp_y), (200.0, 100.0));
    assert_eq!(c.to_json(), r#"{"x":4240,"y":200,"monitor":1,"dp_x":200,"dp_y":100}"#);
    // Sem o serviço (ou no Wayland, no celular, na web): `None`.
    let (ui, _) = ui_with(FakeDesktop { cursor: None, form: None, transparency: None });
    assert_eq!(ui.cursor_position(), None);
    assert_eq!(Ui::new().cursor_position(), None);
}

#[test]
fn the_form_under_a_point_must_be_open() {
    let mut ui = Ui::new();
    let form = ui.add_root(Block::default());
    ui.services_mut().desktop = Some(Box::new(FakeDesktop { cursor: None, form: Some(form), transparency: None }));
    assert_eq!(ui.form_at_screen(10, 10), None, "fechado não conta");
    ui.show(form).unwrap();
    assert_eq!(ui.form_at_screen(10, 10), Some(form));
}

#[test]
fn the_system_transparency_defaults_to_on_and_counts_the_changes() {
    assert!(Ui::new().system_transparency(), "o sistema que não informa: ligada");
    let (mut ui, _) = ui_with(FakeDesktop { cursor: None, form: None, transparency: Some(false) });
    assert!(!ui.system_transparency(), "o serviço informa antes da plataforma");
    ui.set_system_transparency(false);
    assert_eq!(ui.system_transparency_version(), 0, "o primeiro valor não é mudança");
    ui.set_system_transparency(false);
    assert_eq!(ui.system_transparency_version(), 0);
    ui.set_system_transparency(true);
    assert!(ui.system_transparency());
    assert_eq!(ui.system_transparency_version(), 1);
}

#[test]
fn the_monitor_capture_needs_the_project_option() {
    let (mut ui, _) = ui_with(FakeDesktop { cursor: None, form: None, transparency: None });
    let got: Rc<Cell<Option<Result<(u32, u32), String>>>> = Rc::default();
    let sink = got.clone();
    ui.capture_monitor(0, 320, move |_, r| sink.set(Some(r.map(|i| size(&i)))));
    ui.run_timers();
    let refused = got.take().unwrap().unwrap_err();
    assert!(refused.contains("allow_screen_capture"), "{refused}");
    ui.set_allow_screen_capture(true);
    let sink = got.clone();
    ui.capture_monitor(9, 320, move |_, r| sink.set(Some(r.map(|i| size(&i)))));
    ui.run_timers();
    assert!(got.take().unwrap().is_err(), "monitor que não existe");
    let sink = got.clone();
    ui.capture_monitor(1, 320, move |_, r| sink.set(Some(r.map(|i| size(&i)))));
    ui.process_posted();
    assert_eq!(got.take().unwrap(), Ok((320, 180)));
}

fn size(image: &Image) -> (u32, u32) {
    match image.source() {
        image::ImageSource::Rgba { width, height, .. } => (*width, *height),
        _ => (0, 0),
    }
}

#[test]
fn the_border_color_has_three_states() {
    use aylis_core::prop::PropType;
    assert_eq!(BorderColor::from_value(PropValue::Null).unwrap(), BorderColor::System);
    assert_eq!(BorderColor::from_value(PropValue::String(String::new())).unwrap(), BorderColor::System);
    assert_eq!(BorderColor::from_value(PropValue::String("none".into())).unwrap(), BorderColor::None);
    assert_eq!(BorderColor::from_value(PropValue::String("#FF0000".into())).unwrap(), BorderColor::Color(Color::rgb(255, 0, 0)));
    assert_eq!(BorderColor::None.to_value(), PropValue::String("none".into()));
    assert_eq!(BorderColor::System.to_value(), PropValue::Null);
    // No acesso tipado: o `ColorRef` leva o texto que volta ao mesmo estado.
    let typed: PropValue = handles::ColorRef::from(BorderColor::None).into();
    assert_eq!(BorderColor::from_value(typed).unwrap(), BorderColor::None);
}

#[test]
fn the_key_says_its_form_and_the_active_form_follows() {
    let mut ui = Ui::new();
    let a = ui.add_root(Block::default());
    let b = ui.add_root(Block::default());
    ui.show(a).unwrap();
    ui.show(b).unwrap();
    let seen: Rc<Cell<Option<ComponentId>>> = Rc::default();
    let sink = seen.clone();
    ui.on(a, "on_key_down", move |_, _, args| {
        if let EventArgs::Key(k) = args {
            sink.set(k.form);
        }
    });
    ui.key_down(a, KeyEvent { key: Key::Char('k'), ..Default::default() });
    assert_eq!(seen.get(), Some(a));
    assert_eq!(ui.active_form(), Some(a));
    ui.key_down(b, KeyEvent { key: Key::Char('k'), ..Default::default() });
    assert_eq!(ui.active_form(), Some(b));
    ui.close(b);
    assert_eq!(ui.active_form(), Some(a), "fechado o ativo, o de baixo volta a ser");
    ui.close(a);
    assert_eq!(ui.active_form(), None, "sem form aberto, nenhum");
    // O Caps Lock e o Num Lock nascem desligados.
    assert!(!Modifiers::default().caps_lock && !Modifiers::default().num_lock);
}

#[test]
fn a_remembered_placement_finds_its_monitor_by_the_device_path() {
    use aylis_core::monitors::{SavedPlacement, restore_placement};
    let saved = SavedPlacement { monitor: "path-B".into(), placement: WindowPlacement { x: 4000, y: 100, width: 800, height: 600, maximized: false, full_screen: false } };
    let mut list = monitors();
    // O monitor trocou de porta: o nome do sistema mudou, a identidade não.
    list[1].device = r"\\.\DISPLAY7".into();
    assert_eq!(restore_placement(&saved, &list).map(|p| p.x), Some(4000));
    list[1].device_path = "outro".into();
    assert_eq!(restore_placement(&saved, &list), None);
}
