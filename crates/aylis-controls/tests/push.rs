//! `PushNotifications` com um serviço de plataforma falso: registra sozinho quando o form
//! abre, o token, as mensagens e os toques viram eventos e propriedades, e o que a plataforma
//! não mostra sozinha vira notificação do sistema.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::platform::{
    NotificationService, PushConfig, PushEvent, PushMessage, PushService, SystemNotification, json_field, parse_push_line, push_event,
    push_message_from_payload,
};
use aylis_core::*;

#[derive(Default)]
struct Shared {
    calls: Vec<String>,
    events: Vec<PushEvent>,
    shown: Vec<(String, String)>,
}

type Link = Arc<Mutex<Shared>>;

struct FakePush {
    link: Link,
    shows: bool,
}

impl PushService for FakePush {
    fn available(&self) -> bool {
        true
    }
    fn provider(&self) -> &'static str {
        "fcm"
    }
    fn register(&mut self, c: &PushConfig) {
        self.link.lock().unwrap().calls.push(format!("register {} {} {} {} {} {}", c.sender_id, c.project_id, c.app_id, c.api_key, c.vapid_key, c.show_in_foreground));
    }
    fn unregister(&mut self) {
        self.link.lock().unwrap().calls.push("unregister".into());
    }
    fn poll(&mut self) -> Vec<PushEvent> {
        std::mem::take(&mut self.link.lock().unwrap().events)
    }
    fn shows_notifications(&self) -> bool {
        self.shows
    }
}

struct FakeNotifications(Link);

impl NotificationService for FakeNotifications {
    fn notify(&mut self, n: &SystemNotification) -> Result<(), String> {
        self.0.lock().unwrap().shown.push((n.title.clone(), n.body.clone()));
        Ok(())
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

/// A fila de push é do processo: os testes que rodam componentes vão um de cada vez.
static SERIAL: Mutex<()> = Mutex::new(());

fn step(ui: &mut Ui, t: &mut f64) {
    *t += 1.5;
    ui.set_time(*t);
    ui.run_timers();
}

#[test]
fn push_registers_receives_and_opens() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let push = ui.create("PushNotifications", Some(form)).unwrap();
    let log = record(&mut ui, push, &["on_token", "on_message", "on_opened", "on_error"]);
    let mut t = 0.0;
    // Sem serviço: o registro automático avisa no on_error (uma vez só).
    step(&mut ui, &mut t);
    step(&mut ui, &mut t);
    assert_eq!(ui.get_prop(push, "available").unwrap(), PropValue::Bool(false));
    assert_eq!(log.borrow().len(), 1);
    assert!(log.borrow()[0].starts_with("on_error:"), "{:?}", log.borrow());
    log.borrow_mut().clear();

    // Com o serviço: o `register` manda a configuração do componente.
    let link: Link = Default::default();
    ui.services_mut().push = Some(Box::new(FakePush { link: link.clone(), shows: true }));
    for (prop, value) in [("sender_id", " 123 "), ("project_id", "loja"), ("app_id", "1:123:android:abc"), ("api_key", "AIza"), ("vapid_key", "BKx")] {
        ui.set_prop(push, prop, value).unwrap();
    }
    ui.set_prop(push, "show_in_foreground", false).unwrap();
    assert_eq!(ui.invoke(push, "register", &[]).unwrap(), PropValue::Bool(true));
    assert_eq!(link.lock().unwrap().calls, ["register 123 loja 1:123:android:abc AIza BKx false"]);
    assert_eq!(ui.get_prop(push, "provider").unwrap(), PropValue::from("fcm"));

    // O token, uma mensagem e o toque, pelo serviço e pela fila global (o toque que abriu o app).
    link.lock().unwrap().events.extend([
        PushEvent::Token("tok-1".into()),
        PushEvent::Message(PushMessage { id: "m1".into(), title: "Pedido".into(), body: "Chegou o 42".into(), data: r#"{"order":"42"}"#.into() }),
    ]);
    push_event(PushEvent::Opened(PushMessage { id: "m0".into(), title: "Antigo".into(), body: String::new(), data: r#"{"order":"7"}"#.into() }));
    step(&mut ui, &mut t);
    assert_eq!(*log.borrow(), [r#"on_token:tok-1"#, r#"on_message:{"order":"42"}"#, r#"on_opened:{"order":"7"}"#]);
    assert_eq!(ui.get_prop(push, "token").unwrap(), PropValue::from("tok-1"));
    assert_eq!(ui.get_prop(push, "registered").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(push, "available").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(push, "title").unwrap(), PropValue::from("Antigo"));
    assert_eq!(ui.get_prop(push, "message_id").unwrap(), PropValue::from("m0"));
    assert_eq!(ui.invoke(push, "data_value", &[PropValue::from("order")]).unwrap(), PropValue::from("7"));

    ui.invoke(push, "unregister", &[]).unwrap();
    assert_eq!(link.lock().unwrap().calls.last().unwrap(), "unregister");
    assert_eq!(ui.get_prop(push, "token").unwrap(), PropValue::from(""));
    assert_eq!(ui.get_prop(push, "registered").unwrap(), PropValue::Bool(false));
}

#[test]
fn push_shows_what_the_platform_does_not() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let push = ui.create("PushNotifications", Some(form)).unwrap();
    let link: Link = Default::default();
    // O UnifiedPush e o WNS raw só entregam: o componente mostra com o show_in_foreground.
    ui.services_mut().push = Some(Box::new(FakePush { link: link.clone(), shows: false }));
    ui.services_mut().notifications = Some(Box::new(FakeNotifications(link.clone())));
    let mut t = 0.0;
    step(&mut ui, &mut t);
    assert_eq!(link.lock().unwrap().calls.len(), 1, "registrou sozinho (active)");
    let message = |title: &str| PushMessage { title: title.into(), body: "corpo".into(), data: "{}".into(), ..Default::default() };
    link.lock().unwrap().events.extend([PushEvent::Message(message("Um")), PushEvent::Message(PushMessage { data: "{}".into(), ..Default::default() })]);
    step(&mut ui, &mut t);
    assert_eq!(link.lock().unwrap().shown, [("Um".to_string(), "corpo".to_string())], "a só de dados não aparece");
    ui.set_prop(push, "show_in_foreground", false).unwrap();
    link.lock().unwrap().events.push(PushEvent::Message(message("Dois")));
    step(&mut ui, &mut t);
    assert_eq!(link.lock().unwrap().shown.len(), 1, "sem show_in_foreground, só o on_message");
}

#[test]
fn push_lines_and_payloads_become_messages() {
    let line = "message\tm1\tPedido\tChegou\t{\"order\":\"42\"}";
    assert_eq!(
        parse_push_line(line),
        Some(PushEvent::Message(PushMessage { id: "m1".into(), title: "Pedido".into(), body: "Chegou".into(), data: r#"{"order":"42"}"#.into() }))
    );
    assert_eq!(parse_push_line("token\tabc"), Some(PushEvent::Token("abc".into())));
    assert_eq!(parse_push_line("error\tsem rede"), Some(PushEvent::Error("sem rede".into())));
    assert!(matches!(parse_push_line("opened\t\t\t\t"), Some(PushEvent::Opened(m)) if m.data == "{}"));
    assert_eq!(parse_push_line("outra coisa"), None);

    let m = push_message_from_payload(r#"{"title":"Oi","body":"Tudo bem?","id":"9","data":{"chat":"5"}}"#);
    assert_eq!((m.id.as_str(), m.title.as_str(), m.body.as_str(), m.data.as_str()), ("9", "Oi", "Tudo bem?", r#"{"chat":"5"}"#));
    let fcm = push_message_from_payload(r#"{"notification":{"title":"T","body":"B"},"data":{"a":"1"}}"#);
    assert_eq!((fcm.title.as_str(), fcm.body.as_str(), fcm.data.as_str()), ("T", "B", r#"{"a":"1"}"#));
    let plain = push_message_from_payload("só texto");
    assert_eq!((plain.body.as_str(), plain.data.as_str()), ("só texto", "{}"));
}

#[test]
fn json_field_reads_the_data() {
    // Os escapes do JSON chegam como estão (o `\\u` aqui é a barra e o u no texto).
    let json = "{ \"order\": \"42\", \"kind\":\"pedido \\\"novo\\\"\", \"n\": 3, \"o\": {\"a\": [1, 2]}, \"e\": \"\\u00e9\\uD83D\\uDE00\\n\" }";
    assert_eq!(json_field(json, "order").as_deref(), Some("42"));
    assert_eq!(json_field(json, "kind").as_deref(), Some("pedido \"novo\""));
    assert_eq!(json_field(json, "n").as_deref(), Some("3"));
    assert_eq!(json_field(json, "o").as_deref(), Some(r#"{"a": [1, 2]}"#));
    assert_eq!(json_field(json, "e").as_deref(), Some("é😀\n"));
    assert_eq!(json_field(json, "falta"), None);
    assert_eq!(json_field("{}", "x"), None);
    assert_eq!(json_field("não é json", "x"), None);
}

// ---------------------------------------------------------------------------
// PushTrigger, WakeAppAction e NotifyAction

use aylis_core::platform::{ActionEvent, ActionService, data_matches, parse_action_line, render_template};

#[derive(Default)]
struct ActionLog {
    executed: Vec<(String, String)>,
    stopped: Vec<String>,
    rules: Vec<String>,
    events: Vec<ActionEvent>,
}

struct FakeActions(Arc<Mutex<ActionLog>>);

impl ActionService for FakeActions {
    fn execute(&mut self, spec: &str, data: &str) -> Result<(), String> {
        self.0.lock().unwrap().executed.push((spec.to_string(), data.to_string()));
        Ok(())
    }
    fn stop(&mut self, component: &str) {
        self.0.lock().unwrap().stopped.push(component.to_string());
    }
    fn set_push_rules(&mut self, rules: &str) {
        self.0.lock().unwrap().rules.push(rules.to_string());
    }
    fn poll(&mut self) -> Vec<ActionEvent> {
        std::mem::take(&mut self.0.lock().unwrap().events)
    }
}

/// Um form com o push, a regra `kind=call` e a ação de abrir o app.
fn call_form(ui: &mut Ui) -> (ComponentId, ComponentId, ComponentId) {
    aylis_controls::register(ui);
    let form = ui.add_root(Form::new("frmPrincipal", 100.0, 100.0));
    ui.set_name(form, "frmPrincipal").unwrap();
    let push = ui.create("PushNotifications", Some(form)).unwrap();
    let trigger = ui.create("PushTrigger", Some(form)).unwrap();
    let wake = ui.create("WakeAppAction", Some(form)).unwrap();
    ui.set_name(push, "push").unwrap();
    ui.set_name(trigger, "chamada").unwrap();
    ui.set_name(wake, "tocar").unwrap();
    ui.set_prop(trigger, "source", PropValue::Component(Some(push))).unwrap();
    ui.set_prop(trigger, "action", PropValue::Component(Some(wake))).unwrap();
    ui.set_prop(trigger, "filter", "kind=call").unwrap();
    ui.set_prop(wake, "title", "{caller}").unwrap();
    ui.set_prop(wake, "text", "Chamada de voz").unwrap();
    ui.set_prop(wake, "buttons", "answer:Atender;reject:Recusar").unwrap();
    (push, trigger, wake)
}

fn call_message(kind: &str) -> PushEvent {
    PushEvent::Message(PushMessage { id: "c1".into(), title: "Push".into(), body: "corpo".into(), data: format!(r#"{{"kind":"{kind}","caller":"Ana"}}"#) })
}

#[test]
fn trigger_runs_the_action_with_the_app_open() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut ui = Ui::new();
    let (_push, trigger, wake) = call_form(&mut ui);
    let link: Link = Default::default();
    // Sem o ActionService (desktop): o componente casa a regra e a ação mostra o form e avisa.
    ui.services_mut().push = Some(Box::new(FakePush { link: link.clone(), shows: false }));
    ui.services_mut().notifications = Some(Box::new(FakeNotifications(link.clone())));
    let log = record(&mut ui, trigger, &["on_trigger"]);
    let wake_log = record(&mut ui, wake, &["on_open", "on_timeout", "on_button"]);
    let mut t = 0.0;
    step(&mut ui, &mut t);
    link.lock().unwrap().events.push(call_message("call"));
    step(&mut ui, &mut t);
    ui.process_posted();
    assert_eq!(log.borrow().len(), 1, "a regra disparou");
    assert!(log.borrow()[0].contains("\"caller\":\"Ana\"") && log.borrow()[0].contains("\"title\":\"Push\""), "{:?}", log.borrow());
    assert_eq!(wake_log.borrow().len(), 1);
    assert!(wake_log.borrow()[0].starts_with("on_open:"));
    // A notificação da ação (com o modelo preenchido), e não a comum do push (replace_default).
    assert_eq!(link.lock().unwrap().shown, [("Ana".to_string(), "Chamada de voz".to_string())]);
    assert_eq!(ui.get_prop(wake, "running").unwrap(), PropValue::Bool(true));

    // Outra mensagem não casa: só a notificação comum.
    link.lock().unwrap().events.push(call_message("chat"));
    step(&mut ui, &mut t);
    ui.process_posted();
    assert_eq!(log.borrow().len(), 1);
    assert_eq!(link.lock().unwrap().shown.last().unwrap(), &("Push".to_string(), "corpo".to_string()));

    // Sem resposta no tempo: para e avisa.
    ui.set_prop(wake, "timeout", 2.0f32).unwrap();
    ui.invoke(wake, "execute", &[PropValue::from(r#"{"caller":"Bia"}"#)]).unwrap();
    step(&mut ui, &mut t);
    step(&mut ui, &mut t);
    assert!(wake_log.borrow().iter().any(|e| e.starts_with("on_timeout:")), "{:?}", wake_log.borrow());
    assert_eq!(ui.get_prop(wake, "running").unwrap(), PropValue::Bool(false));
}

#[test]
fn the_platform_gets_the_rules_and_reports_back() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut ui = Ui::new();
    let (push, trigger, wake) = call_form(&mut ui);
    let link: Link = Default::default();
    let actions: Arc<Mutex<ActionLog>> = Default::default();
    ui.services_mut().push = Some(Box::new(FakePush { link: link.clone(), shows: true }));
    ui.services_mut().actions = Some(Box::new(FakeActions(actions.clone())));
    let trigger_log = record(&mut ui, trigger, &["on_trigger"]);
    let wake_log = record(&mut ui, wake, &["on_open", "on_button", "on_timeout"]);
    let mut t = 0.0;
    step(&mut ui, &mut t);
    // As regras vão para a plataforma (para valer com o app fechado), com a ação inteira.
    let rules = actions.lock().unwrap().rules.last().cloned().expect("mandou as regras");
    assert!(rules.contains(r#""trigger":"frmPrincipal.chamada""#) && rules.contains(r#""filter":"kind=call""#), "{rules}");
    assert!(rules.contains(r#""type":"wake""#) && rules.contains(r#""component":"frmPrincipal.tocar""#) && rules.contains(r#""title":"{caller}""#), "{rules}");
    assert!(rules.contains(r#"["answer","Atender"]"#) && rules.contains(r#""open":["answer"]"#), "{rules}");
    // Uma mudança chega à plataforma (a cada 2 s); sem mudança, não manda de novo.
    let sent = actions.lock().unwrap().rules.len();
    step(&mut ui, &mut t);
    step(&mut ui, &mut t);
    assert_eq!(actions.lock().unwrap().rules.len(), sent);
    ui.set_prop(trigger, "filter", "kind=video").unwrap();
    step(&mut ui, &mut t);
    step(&mut ui, &mut t);
    assert!(actions.lock().unwrap().rules.last().unwrap().contains("kind=video"));

    // Com a plataforma executando as regras, a mensagem não roda a regra de novo aqui.
    link.lock().unwrap().events.push(call_message("video"));
    step(&mut ui, &mut t);
    ui.process_posted();
    assert!(trigger_log.borrow().is_empty());
    assert!(actions.lock().unwrap().executed.is_empty());

    // O que a plataforma conta vira evento de cada componente.
    actions.lock().unwrap().events.extend([
        parse_action_line("trigger\tfrmPrincipal.chamada\t\t{\"caller\":\"Ana\"}").unwrap(),
        parse_action_line("open\tfrmPrincipal.tocar\t\t{\"caller\":\"Ana\"}").unwrap(),
        parse_action_line("button\tfrmPrincipal.tocar\tanswer\t{\"caller\":\"Ana\"}").unwrap(),
    ]);
    step(&mut ui, &mut t);
    ui.process_posted();
    assert_eq!(*trigger_log.borrow(), [r#"on_trigger:{"caller":"Ana"}"#]);
    assert_eq!(*wake_log.borrow(), [r#"on_open:{"caller":"Ana"}"#, "on_button:answer"]);
    assert_eq!(ui.get_prop(wake, "data").unwrap(), PropValue::from(r#"{"caller":"Ana"}"#));

    // Executar e parar pelo código vão para a plataforma.
    ui.invoke(wake, "execute", &[PropValue::from(r#"{"caller":"Bia"}"#)]).unwrap();
    let (spec, data) = actions.lock().unwrap().executed.last().cloned().unwrap();
    assert!(spec.contains(r#""ringtone":"default""#) && spec.contains(r#""timeout":30"#), "{spec}");
    assert_eq!(data, r#"{"caller":"Bia"}"#);
    ui.invoke(wake, "stop", &[]).unwrap();
    assert_eq!(actions.lock().unwrap().stopped, ["frmPrincipal.tocar"]);

    // O NotifyAction leva o canal, a prioridade e os botões (todos abrem o app por padrão).
    let form = ui.form_of(push);
    let notify = ui.create("NotifyAction", Some(form)).unwrap();
    ui.set_name(notify, "aviso").unwrap();
    ui.set_prop(notify, "buttons", "ver:Ver pedido {order}").unwrap();
    ui.set_prop(notify, "priority", PropValue::Enum("high".into())).unwrap();
    ui.invoke(notify, "execute", &[PropValue::from(r#"{"order":"42"}"#)]).unwrap();
    let (spec, _) = actions.lock().unwrap().executed.last().cloned().unwrap();
    assert!(
        spec.contains(r#""type":"notify""#) && spec.contains(r#""priority":4"#) && spec.contains(r#"["ver","Ver pedido {order}"]"#) && spec.contains(r#""open":["*"]"#),
        "{spec}"
    );
}

#[test]
fn templates_and_filters() {
    let data = r#"{"caller":"Ana","kind":"call","n":3,"vazio":""}"#;
    assert_eq!(render_template("Chamada de {caller} ({n}) {falta}", data), "Chamada de Ana (3) ");
    assert_eq!(render_template("{{literal}} e {caller}", data), "{literal} e Ana");
    assert_eq!(render_template("sem fecho {caller", data), "sem fecho {caller");
    assert!(data_matches("", data));
    assert!(data_matches("kind=call; caller", data));
    assert!(data_matches("kind!=chat", data));
    assert!(!data_matches("kind=chat", data));
    assert!(!data_matches("vazio", data));
    assert!(!data_matches("falta", data));
    let e = parse_action_line("button\tfrm.x\tanswer\t{}").unwrap();
    assert_eq!((e.kind.as_str(), e.component.as_str(), e.detail.as_str(), e.data.as_str()), ("button", "frm.x", "answer", "{}"));
    assert_eq!(parse_action_line("timeout\tfrm.x").unwrap().data, "{}");
    assert!(parse_action_line("").is_none());
}
