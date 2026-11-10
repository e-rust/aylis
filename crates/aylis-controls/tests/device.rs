//! `Permissions` e `Biometrics` com serviços de plataforma falsos: a resposta chega pelos
//! eventos, pela fila da interface; sem serviço (desktop), tudo liberado e sem biometria.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{AppLink, Biometrics, Form, Permissions};
use aylis_core::platform::{BiometricResult, BiometricService, Biometry, Incoming, LinkService, Permission, PermissionService, PermissionState, Sensor, SensorService};
use aylis_core::*;

struct FakePermissions;

impl PermissionService for FakePermissions {
    fn state(&self, p: Permission) -> PermissionState {
        if p == Permission::Camera { PermissionState::Prompt } else { PermissionState::Denied }
    }

    fn request(&mut self, permissions: Vec<Permission>, done: Box<dyn FnOnce(Vec<(Permission, PermissionState)>) + Send>) {
        // Libera a câmera, nega o resto.
        done(permissions.into_iter().map(|p| (p, if p == Permission::Camera { PermissionState::Granted } else { PermissionState::Denied })).collect());
    }
}

struct FakeBiometrics(BiometricResult);

impl BiometricService for FakeBiometrics {
    fn availability(&self) -> Biometry {
        Biometry::Face
    }

    fn authenticate(&mut self, reason: &str, allow: bool, done: Box<dyn FnOnce(BiometricResult) + Send>) {
        assert_eq!(reason, "Confirme");
        assert!(allow);
        done(self.0.clone());
    }
}

fn record(ui: &mut Ui, id: ComponentId, events: &[&'static str]) -> Rc<RefCell<Vec<String>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    for &event in events {
        let log = log.clone();
        ui.on(id, event, move |_, _, args| log.borrow_mut().push(format!("{event}:{}", args.text().unwrap_or_default())));
    }
    log
}

#[test]
fn permissions_ask_the_platform_and_answer_in_on_result() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let perm = ui.create("Permissions", Some(form)).unwrap();
    ui.set_prop(perm, "permissions", "camera;location;nada").unwrap();
    ui.set_prop(perm, "reasons", "camera=Para a foto").unwrap();
    let log = record(&mut ui, perm, &["on_result"]);

    // Sem serviço (desktop): liberado.
    assert_eq!(Permissions::state(&ui, Permission::Camera), PermissionState::Granted);
    Permissions::request_all(&mut ui, perm);
    ui.process_posted();
    assert_eq!(log.borrow().as_slice(), ["on_result:camera=granted;location=granted"]);

    ui.services_mut().permissions = Some(Box::new(FakePermissions));
    assert_eq!(Permissions::state(&ui, Permission::Camera), PermissionState::Prompt);
    // Pelo invoke (o caminho das outras linguagens).
    assert_eq!(ui.invoke(perm, "state", &[PropValue::from("location")]).unwrap(), PropValue::from("denied"));
    assert!(ui.invoke(perm, "state", &[PropValue::from("xyz")]).is_err());
    ui.invoke(perm, "request", &[PropValue::from("camera;microphone")]).unwrap();
    ui.process_posted();
    ui.process_posted();
    assert_eq!(log.borrow().last().unwrap(), "on_result:camera=granted;microphone=denied");
    assert_eq!(ui.get::<Permissions>(perm).unwrap().reason(Permission::Camera), "Para a foto");
}

#[test]
fn biometrics_answer_in_success_fail_or_cancel() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let bio = ui.create("Biometrics", Some(form)).unwrap();
    ui.set_prop(bio, "reason", "Confirme").unwrap();
    ui.set_prop(bio, "allow_device_credential", true).unwrap();
    let log = record(&mut ui, bio, &["on_success", "on_fail", "on_cancel"]);

    // Sem serviço: sem biometria, e pedir falha com o motivo.
    assert!(!Biometrics::available(&ui));
    Biometrics::authenticate(&mut ui, bio);
    ui.process_posted();
    assert!(log.borrow()[0].starts_with("on_fail:"), "{:?}", log.borrow());

    for (result, expected) in [
        (BiometricResult::Success, "on_success:"),
        (BiometricResult::Cancelled, "on_cancel:"),
        (BiometricResult::Failed("não reconheceu".into()), "on_fail:não reconheceu"),
    ] {
        ui.services_mut().biometrics = Some(Box::new(FakeBiometrics(result)));
        assert_eq!(Biometrics::availability(&ui), Biometry::Face);
        ui.invoke(bio, "authenticate", &[]).unwrap();
        ui.process_posted();
        ui.process_posted();
        assert_eq!(log.borrow().last().unwrap(), expected);
    }
    assert_eq!(ui.invoke(bio, "availability", &[]).unwrap(), PropValue::from("face"));
}

/// Guarda o que o `AppLink` pediu; abrir `ruim:` falha e o pedido devolve o que recebeu.
struct FakeLinks(std::sync::Arc<std::sync::Mutex<Vec<String>>>);

impl LinkService for FakeLinks {
    fn open_url(&mut self, url: &str) -> Result<(), String> {
        self.0.lock().unwrap().push(format!("url {url}"));
        if url.starts_with("ruim:") { Err("não abriu".into()) } else { Ok(()) }
    }

    fn share(&mut self, text: &str, files: &[String]) -> Result<(), String> {
        self.0.lock().unwrap().push(format!("share {text} {}", files.join("|")));
        Ok(())
    }

    fn request(&mut self, action: &str, data: &str, done: Box<dyn FnOnce(Result<String, String>) + Send>) {
        done(Ok(format!("{action}:{data}")));
    }
}

#[test]
fn app_link_asks_the_platform_and_delivers_what_arrives() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let link = ui.create("AppLink", Some(form)).unwrap();
    let log = record(&mut ui, link, &["on_link", "on_shared", "on_result", "on_error"]);
    // Sem serviço: o motivo.
    assert!(AppLink::open_url(&mut ui, "https://aylis.dev").is_err());

    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    ui.services_mut().links = Some(Box::new(FakeLinks(calls.clone())));
    ui.invoke(link, "open_url", &[PropValue::from("https://aylis.dev")]).unwrap();
    ui.invoke(link, "open_url", &[PropValue::from("ruim:x")]).unwrap();
    ui.invoke(link, "share", &[PropValue::from("olá"), PropValue::from("a.png;b.txt")]).unwrap();
    ui.invoke(link, "request", &[PropValue::from("PICK"), PropValue::from("content://x")]).unwrap();
    ui.invoke(link, "open_app", &[PropValue::from("com.whatsapp")]).unwrap();
    ui.process_posted();
    ui.process_posted();
    assert_eq!(*calls.lock().unwrap(), ["url https://aylis.dev", "url ruim:x", "share olá a.png|b.txt"]);
    assert_eq!(
        log.borrow().as_slice(),
        // A resposta do pedido passa pela fila mais uma vez.
        ["on_error:não abriu", "on_error:abrir outro app não existe aqui (com.whatsapp)", "on_result:PICK:content://x"]
    );

    // O que chega de fora espera um AppLink e vira evento no timer dele.
    log.borrow_mut().clear();
    aylis_core::platform::push_incoming(Incoming::Link("meuapp://pedido/42".into()));
    aylis_core::platform::push_incoming(Incoming::Shared("um texto".into()));
    ui.set_time(1.0);
    ui.run_timers();
    assert_eq!(log.borrow().as_slice(), ["on_link:meuapp://pedido/42", "on_shared:um texto"]);
}

/// Bateria a 42% carregando; a localização só mede ligada; vibra.
struct FakeSensors(std::sync::Arc<std::sync::Mutex<Vec<String>>>);

impl SensorService for FakeSensors {
    fn available(&self, sensor: Sensor) -> bool {
        sensor != Sensor::Light
    }

    fn set_active(&mut self, sensor: Sensor, on: bool) {
        self.0.lock().unwrap().push(format!("{sensor:?}={on}"));
    }

    fn read(&mut self, sensor: Sensor) -> Option<Vec<f64>> {
        match sensor {
            Sensor::Battery => Some(vec![0.42, 1.0]),
            Sensor::Location => self.0.lock().unwrap().contains(&"Location=true".to_string()).then(|| vec![-23.55, -46.63, 5.0, 760.0, 0.0]),
            Sensor::Network => Some(vec![1.0, 1.0]),
            _ => None,
        }
    }

    fn vibrate(&mut self, ms: u32) -> bool {
        self.0.lock().unwrap().push(format!("vibrate {ms}"));
        true
    }
}

#[test]
fn sensors_read_the_platform_in_their_timer() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    ui.services_mut().sensors = Some(Box::new(FakeSensors(calls.clone())));
    let battery = ui.create("Battery", Some(form)).unwrap();
    let network = ui.create("Network", Some(form)).unwrap();
    let location = ui.create("Location", Some(form)).unwrap();
    let light = ui.create("LightSensor", Some(form)).unwrap();
    let log = record(&mut ui, battery, &["on_change"]);
    let moved = record(&mut ui, location, &["on_change"]);
    for t in [5.5, 11.0] {
        ui.set_time(t);
        ui.run_timers();
    }
    assert_eq!(log.borrow().len(), 1, "muda uma vez só");
    assert_eq!(ui.get_prop(battery, "level").unwrap(), PropValue::Float(42.0));
    assert_eq!(ui.get_prop(battery, "charging").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(network, "kind").unwrap(), PropValue::from("wifi"));
    // Desligada, a localização nem liga.
    assert!(moved.borrow().is_empty() && !calls.lock().unwrap().iter().any(|c| c.starts_with("Location")));
    ui.set_prop(location, "active", true).unwrap();
    ui.set_time(12.5);
    ui.run_timers();
    assert_eq!(moved.borrow().len(), 1);
    assert_eq!(ui.get_prop(location, "latitude").unwrap(), PropValue::Float(-23.55));
    ui.set_prop(location, "active", false).unwrap();
    assert!(calls.lock().unwrap().contains(&"Location=false".to_string()));
    // Sem o sensor: available falso.
    ui.set_prop(light, "active", true).unwrap();
    ui.set_time(14.0);
    ui.run_timers();
    assert_eq!(ui.get_prop(light, "available").unwrap(), PropValue::Bool(false));
    let vib = ui.create("Vibration", Some(form)).unwrap();
    assert_eq!(ui.invoke(vib, "vibrate", &[PropValue::from(150i64)]).unwrap(), PropValue::Bool(true));
    assert!(calls.lock().unwrap().contains(&"vibrate 150".to_string()));
}

/// Um controle que o teste mexe por fora.
struct FakePads(std::sync::Arc<std::sync::Mutex<Vec<aylis_core::platform::GamepadState>>>);

impl aylis_core::platform::GamepadService for FakePads {
    fn pads(&mut self) -> Vec<aylis_core::platform::GamepadState> {
        self.0.lock().unwrap().clone()
    }

    fn rumble(&mut self, index: usize, _strength: f32, _ms: u32) -> bool {
        index < self.0.lock().unwrap().len()
    }
}

#[test]
fn gamepad_reports_buttons_sticks_and_connection() {
    use aylis_core::platform::GamepadState;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let pads = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    ui.services_mut().gamepads = Some(Box::new(FakePads(pads.clone())));
    let pad = ui.create("Gamepad", Some(form)).unwrap();
    let log = record(&mut ui, pad, &["on_connect", "on_disconnect", "on_button_down", "on_button_up", "on_change"]);
    let mut t = 0.0;
    let mut frame = |ui: &mut Ui| {
        t += 0.02;
        ui.set_time(t);
        ui.run_timers();
    };
    frame(&mut ui);
    assert!(log.borrow().is_empty());
    assert_eq!(ui.get_prop(pad, "connected").unwrap(), PropValue::Bool(false));

    let mut state = GamepadState { name: "Xbox Controller".into(), ..Default::default() };
    state.set_button("a", true);
    // Dentro da zona morta (0,15): não conta.
    state.axes = [0.1, 0.0, 0.0, 0.0, 0.0, 0.0];
    pads.lock().unwrap().push(state.clone());
    frame(&mut ui);
    assert_eq!(*log.borrow(), ["on_connect:", "on_button_down:a"]);
    assert_eq!(ui.get_prop(pad, "device_name").unwrap(), PropValue::from("Xbox Controller"));
    assert_eq!(ui.invoke(pad, "pressed", &[PropValue::from("a")]).unwrap(), PropValue::Bool(true));

    log.borrow_mut().clear();
    state.set_button("a", false);
    state.axes = [1.0, 0.0, 0.0, -1.0, 0.8, 0.0];
    pads.lock().unwrap()[0] = state;
    frame(&mut ui);
    assert_eq!(*log.borrow(), ["on_button_up:a", "on_change:"]);
    assert_eq!(ui.get_prop(pad, "left_x").unwrap(), PropValue::Float(1.0));
    assert_eq!(ui.get_prop(pad, "right_y").unwrap(), PropValue::Float(-1.0));
    assert!((ui.get_prop(pad, "left_trigger").unwrap().to_f64().unwrap() - 0.8).abs() < 1e-6);
    assert_eq!(ui.invoke(pad, "rumble", &[PropValue::Float(0.5), PropValue::from(100i64)]).unwrap(), PropValue::Bool(true));

    // O segundo controle (index 1) ainda não existe.
    let second = ui.create("Gamepad", Some(form)).unwrap();
    ui.set_prop(second, "index", 1u32).unwrap();
    frame(&mut ui);
    assert_eq!(ui.get_prop(second, "connected").unwrap(), PropValue::Bool(false));
    assert_eq!(ui.get_prop(second, "count").unwrap(), PropValue::from(1u32));

    log.borrow_mut().clear();
    pads.lock().unwrap().clear();
    frame(&mut ui);
    assert_eq!(*log.borrow(), ["on_disconnect:", "on_change:"]);
}

#[test]
fn screen_answers_the_monitors_through_invoke() {
    use aylis_core::monitors::Monitor;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let _ = ui.set_name(form, "frmTeste");
    let screen = ui.create("Screen", Some(form)).unwrap();
    let changes = record(&mut ui, screen, &["on_change"]);
    let main = Monitor { name: "XB273K V5".into(), device: "DISPLAY1".into(), primary: true, width: 3840, height: 2160, scale: 1.5, refresh_hz: 160.0, ..Default::default() };
    let side = Monitor { name: "PHL 288P6L".into(), device: "DISPLAY2".into(), x: -3840, width: 3840, height: 2160, scale: 1.5, refresh_hz: 60.0, ..Default::default() };
    ui.set_monitors(vec![main.clone()]);
    ui.set_time(1.5);
    ui.run_timers();
    assert!(changes.borrow().is_empty(), "a primeira leitura não é mudança");
    ui.set_monitors(vec![main, side]);
    ui.set_form_monitor(form, Some(1));
    ui.set_time(3.0);
    ui.run_timers();
    assert_eq!(changes.borrow().len(), 1, "um monitor ligado");
    assert_eq!(ui.invoke(screen, "count", &[]).unwrap(), PropValue::Int(2));
    assert_eq!(ui.invoke(screen, "primary", &[]).unwrap(), PropValue::Int(0));
    assert_eq!(ui.invoke(screen, "refresh_rate", &[PropValue::Int(1)]).unwrap(), PropValue::Float(60.0));
    assert_eq!(ui.invoke(screen, "form_monitor", &[PropValue::from("frmTeste")]).unwrap(), PropValue::Int(1));
    let json = ui.invoke(screen, "monitors", &[]).unwrap();
    let text = json.to_str().unwrap();
    assert!(text.starts_with('[') && text.contains("PHL 288P6L") && text.contains("\"refresh_hz\":160"), "{text}");
}
