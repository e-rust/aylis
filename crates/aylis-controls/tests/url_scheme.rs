//! `UrlScheme` com uma plataforma falsa: ligado, registra o prefixo, entrega o link que abriu
//! o app e os que outra cópia repassa (só os do seu prefixo), e traz a janela para a frente.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::platform::{LinkService, check_scheme, push_startup_link};
use aylis_core::*;

type Receiver = Box<dyn Fn(String) + Send>;

#[derive(Clone, Default)]
struct FakeLinks {
    calls: Arc<Mutex<Vec<String>>>,
    registered: Arc<Mutex<Vec<String>>>,
    receivers: Arc<Mutex<Vec<Receiver>>>,
}

impl LinkService for FakeLinks {
    fn open_url(&mut self, _url: &str) -> Result<(), String> {
        Ok(())
    }
    fn can_register_schemes(&self) -> bool {
        true
    }
    fn register_scheme(&mut self, scheme: &str, description: &str) -> Result<(), String> {
        self.calls.lock().unwrap().push(format!("register {scheme} {description}"));
        self.registered.lock().unwrap().push(scheme.to_string());
        Ok(())
    }
    fn unregister_scheme(&mut self, scheme: &str) -> Result<(), String> {
        self.registered.lock().unwrap().retain(|s| s != scheme);
        Ok(())
    }
    fn scheme_registered(&mut self, scheme: &str) -> bool {
        self.registered.lock().unwrap().iter().any(|s| s == scheme)
    }
    fn receive_links(&mut self, on_link: Receiver) {
        self.receivers.lock().unwrap().push(on_link);
    }
}

fn record(ui: &mut Ui, id: ComponentId) -> Rc<RefCell<Vec<String>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_link", "on_error"] {
        let log = log.clone();
        ui.on(id, event, move |_, _, args| {
            if let EventArgs::Text(t) = args {
                log.borrow_mut().push(format!("{event}:{t}"));
            }
        });
    }
    log
}

#[test]
fn the_prefix_is_checked() {
    assert_eq!(check_scheme("MeuApp://").as_deref(), Ok("meuapp"));
    assert_eq!(check_scheme("loja.pedidos+v2").as_deref(), Ok("loja.pedidos+v2"));
    for bad in ["", "ab", "1app", "meu app", "meu/app", "http", "HTTPS", "file", "javascript", "ms-settings", "windows-x"] {
        assert!(check_scheme(bad).is_err(), "{bad}");
    }
}

#[test]
fn active_registers_receives_and_delivers_only_its_prefix() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let links = FakeLinks::default();
    ui.services_mut().links = Some(Box::new(links.clone()));
    // O link que abriu o app chega antes do form.
    push_startup_link("pedidos://abrir/7".into());
    push_startup_link("outroapp://nao/e/meu".into());

    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let scheme = ui.create("UrlScheme", Some(form)).unwrap();
    let log = record(&mut ui, scheme);
    ui.set_prop(scheme, "scheme", "Pedidos").unwrap();
    ui.set_prop(scheme, "description", "Pedidos da loja").unwrap();
    assert!(links.calls.lock().unwrap().is_empty(), "desligado não registra");
    ui.set_prop(scheme, "active", true).unwrap();
    ui.set_time(0.01);
    ui.run_timers();
    assert_eq!(*links.calls.lock().unwrap(), ["register pedidos Pedidos da loja"]);
    assert_eq!(log.borrow().as_slice(), ["on_link:pedidos://abrir/7"]);
    assert_eq!(ui.invoke(scheme, "is_registered", &[]).unwrap(), PropValue::Bool(true));

    // Outra cópia repassa (de outra thread, pela fila): só o do prefixo, e a janela vem.
    log.borrow_mut().clear();
    let _ = ui.take_window_commands();
    for receiver in links.receivers.lock().unwrap().iter() {
        receiver("PEDIDOS://abrir/8".into());
        receiver("outroapp://x".into());
    }
    ui.process_posted();
    assert_eq!(log.borrow().as_slice(), ["on_link:PEDIDOS://abrir/8"]);
    assert_eq!(ui.take_window_commands(), [(form, WindowCommand::Show)]);
    assert_eq!(links.receivers.lock().unwrap().len(), 1, "um pedido de links só");

    ui.invoke(scheme, "unregister", &[]).unwrap();
    assert_eq!(ui.invoke(scheme, "is_registered", &[]).unwrap(), PropValue::Bool(false));
}

#[test]
fn a_bad_prefix_goes_to_on_error_and_nothing_is_registered() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let links = FakeLinks::default();
    ui.services_mut().links = Some(Box::new(links.clone()));
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let scheme = ui.create("UrlScheme", Some(form)).unwrap();
    let log = record(&mut ui, scheme);
    ui.set_prop(scheme, "scheme", "https").unwrap();
    ui.set_prop(scheme, "active", true).unwrap();
    ui.set_time(0.01);
    ui.run_timers();
    assert!(links.calls.lock().unwrap().is_empty());
    assert_eq!(log.borrow().len(), 1);
    assert!(log.borrow()[0].starts_with("on_error:"), "{:?}", log.borrow());
    assert!(ui.invoke(scheme, "register", &[]).is_err());
}

#[test]
fn in_the_designer_it_does_nothing() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_design_mode(true);
    let links = FakeLinks::default();
    ui.services_mut().links = Some(Box::new(links.clone()));
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let scheme = ui.create("UrlScheme", Some(form)).unwrap();
    ui.set_prop(scheme, "scheme", "designer").unwrap();
    ui.set_prop(scheme, "active", true).unwrap();
    ui.set_time(0.01);
    ui.run_timers();
    assert!(links.calls.lock().unwrap().is_empty());
    assert!(links.receivers.lock().unwrap().is_empty());
}
