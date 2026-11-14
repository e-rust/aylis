//! O canal de desenvolvimento de ponta a ponta: o servidor da IDE e um app conectado por
//! WebSocket cifrado (Noise): inspetor, hot reload empurrado pela IDE e rastro de eventos.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use aylis_core::{PropValue, Ui};
use aylis_devlink::Target;
use aylis_ide::devlink::DevServer;
use serde_json::{Value, json};

const FORM: &str = r#"{"root": {"class": "Form", "name": "frmA", "width": 300, "height": 200, "children": [
    {"class": "Label", "name": "lblTitulo", "text": "antes"}]}}"#;

/// Faz o pedido numa thread e, nesta, roda a fila da interface até a resposta chegar
/// (como o laço de eventos do app).
fn ask<T: Send + 'static>(ui: &mut Ui, job: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(job());
    });
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        ui.process_posted();
        if let Ok(v) = rx.try_recv() {
            return v;
        }
        assert!(Instant::now() < deadline, "sem resposta do app");
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn text(ui: &Ui, path: &str) -> String {
    match ui.find_path(path).map(|id| ui.get_prop(id, "text")) {
        Some(Ok(PropValue::String(s))) => s,
        other => panic!("{path}: {other:?}"),
    }
}

/// O app: o form aberto, ainda sem o canal.
fn app() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, FORM, None).unwrap();
    ui.show(form).unwrap();
    ui
}

/// Um teste por vez: o canal do app é um só por processo (como num app de verdade).
static ONE_APP: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// O servidor da IDE numa porta livre e o alvo que o app embutiria.
fn server() -> (std::sync::Arc<DevServer>, Target) {
    let (private, public) = aylis_devlink::keypair().unwrap();
    let secret = aylis_devlink::secret().unwrap();
    let server = DevServer::start("127.0.0.1:0", private, secret).unwrap();
    let target = Target { address: format!("127.0.0.1:{}", server.port()), ide_public: public, secret };
    (server, target)
}

#[test]
fn the_ide_inspects_the_app_and_pushes_hot_reload_through_the_channel() {
    let _one = ONE_APP.lock().unwrap_or_else(|e| e.into_inner());
    let (server, target) = server();
    assert!(!server.connected());
    assert_eq!(server.call(json!({ "cmd": "tree" })).unwrap_err(), "o app não está conectado");

    // O app: o form aberto e o canal ligado (o normal é o alvo embutido pela IDE).
    let mut ui = app();
    aylis::dev_connect(&ui, target);
    // O app só conecta com o laço de eventos rodando (aqui, a fila processada pelo `ask`).
    let s = server.clone();
    assert!(ask(&mut ui, move || s.wait(Duration::from_secs(10))), "o app conectou");

    // Inspetor: a árvore e mudar uma propriedade ao vivo.
    let s = server.clone();
    let tree: Value = ask(&mut ui, move || s.call(json!({ "cmd": "tree" })).unwrap());
    assert!(tree.to_string().contains("lblTitulo"), "{tree}");
    let s = server.clone();
    ask(&mut ui, move || s.call(json!({ "cmd": "set", "target": "frmA.lblTitulo", "props": { "text": "pelo inspetor" } })).unwrap());
    assert_eq!(text(&ui, "frmA.lblTitulo"), "pelo inspetor");

    // Hot reload: a IDE salvou o form com outro texto e um botão novo.
    let new_item = FORM.replace("\"antes\"", "\"depois\"").replace(
        r#"{"class": "Label", "name": "lblTitulo", "text": "depois"}"#,
        r#"{"class": "Label", "name": "lblTitulo", "text": "depois"}, {"class": "Button", "name": "btnOk", "text": "OK"}"#,
    );
    let s = server.clone();
    let done = ask(&mut ui, move || s.reload("form", "frmA", &new_item)).unwrap();
    assert!(done.starts_with("frmA recarregado"), "{done}");
    assert_eq!(text(&ui, "frmA.lblTitulo"), "depois");
    assert_eq!(text(&ui, "frmA.btnOk"), "OK");

    // Um form com erro não derruba o app: a IDE recebe a mensagem.
    let s = server.clone();
    let err = ask(&mut ui, move || s.reload("form", "frmA", "{ quebrado")).unwrap_err();
    assert!(err.contains("continua como estava"), "{err}");
    assert_eq!(text(&ui, "frmA.lblTitulo"), "depois");

    // Tema pelo canal.
    let s = server.clone();
    let done = ask(&mut ui, move || s.reload("theme", "", r##"{"name": "Meu", "colors": {"primary": "#FF0000"}}"##)).unwrap();
    assert_eq!(done, "tema recarregado");
    assert_eq!(ui.theme().color("primary"), aylis_core::Color::rgb(255, 0, 0));
}

#[test]
fn an_app_with_another_secret_is_not_accepted() {
    let _one = ONE_APP.lock().unwrap_or_else(|e| e.into_inner());
    let (server, mut target) = server();
    target.secret[0] ^= 1;
    let mut ui = app();
    aylis::dev_connect(&ui, target);
    let s = server.clone();
    assert!(!ask(&mut ui, move || s.wait(Duration::from_secs(2))), "sem o segredo do projeto, não conecta");
}

/// A cobertura de handlers (4b): o primeiro disparo de cada handler com nome chega à IDE
/// sozinho (sem rastro ligado), uma vez por execução, com o form na frente.
#[test]
fn the_handlers_that_ran_arrive_through_the_channel() {
    let _one = ONE_APP.lock().unwrap_or_else(|e| e.into_inner());
    let (server, target) = server();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, r#"{"root": {"class": "Form", "name": "frmB", "children": [{"class": "Button", "name": "btnSalvar", "on_click": "btnSalvar_click"}]}}"#, None).unwrap();
    ui.show(form).unwrap();
    aylis::dev_connect(&ui, target);
    let s = server.clone();
    assert!(ask(&mut ui, move || s.wait(Duration::from_secs(10))));
    let button = ui.find_path("frmB.btnSalvar").unwrap();
    for _ in 0..3 {
        ui.fire(button, "on_click", &aylis_core::EventArgs::default());
    }
    let s = server.clone();
    let handlers = ask(&mut ui, move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut all = Vec::new();
        while all.is_empty() && Instant::now() < deadline {
            all.extend(s.take_handlers());
            std::thread::sleep(Duration::from_millis(20));
        }
        // Mais um pouco: os outros dois disparos não repetem.
        std::thread::sleep(Duration::from_millis(500));
        all.extend(s.take_handlers());
        all
    });
    assert_eq!(handlers, ["frmB.btnSalvar_click"]);
    assert!(server.take_traces().is_empty(), "sem rastro ligado, nada no rastro");
}

#[test]
fn the_event_trace_arrives_through_the_channel() {
    let _one = ONE_APP.lock().unwrap_or_else(|e| e.into_inner());
    let (server, target) = server();
    server.set_trace(true);
    let mut ui = app();
    aylis::dev_connect(&ui, target);
    let s = server.clone();
    assert!(ask(&mut ui, move || s.wait(Duration::from_secs(10))));
    // O rastro liga logo que o app conecta: um clique com handler vira uma linha na IDE.
    let label = ui.find_path("frmA.lblTitulo").unwrap();
    ui.on_click(label, |_, _| {});
    let s = server.clone();
    ask(&mut ui, move || s.call(json!({ "cmd": "tree" })).unwrap());
    ui.fire(label, "on_click", &aylis_core::EventArgs::default());
    let s = server.clone();
    let lines = ask(&mut ui, move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let lines = s.take_traces();
            if !lines.is_empty() || Instant::now() > deadline {
                return lines;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    });
    assert!(lines.iter().any(|l| l.contains("frmA.lblTitulo.on_click")), "{lines:?}");
}

#[test]
fn the_profiler_records_handlers_layout_and_marks_through_the_channel() {
    let _one = ONE_APP.lock().unwrap_or_else(|e| e.into_inner());
    let (server, target) = server();
    let mut ui = app();
    let form = ui.find("frmA").unwrap();
    // Um handler que demora e marca um trecho à mão.
    ui.register_handler("frmA.lbl_click", |_, _, _| {
        let _p = aylis_core::profile::profile("consulta lenta");
        std::thread::sleep(Duration::from_millis(15));
    });
    let label = ui.find_path("frmA.lblTitulo").unwrap();
    ui.set_prop(label, "on_click", PropValue::Event(Some("lbl_click".into()))).unwrap();
    aylis::dev_connect(&ui, target);
    let s = server.clone();
    assert!(ask(&mut ui, move || s.wait(Duration::from_secs(10))), "o app conectou");

    let s = server.clone();
    ask(&mut ui, move || s.call(json!({ "cmd": "profile", "on": true })).unwrap());
    ui.fire(label, "on_click", &aylis_core::EventArgs::None);
    ui.layout();
    let _ = form;
    let s = server.clone();
    let answer = ask(&mut ui, move || s.call(json!({ "cmd": "profile_take" })).unwrap());
    let s = server.clone();
    ask(&mut ui, move || s.call(json!({ "cmd": "profile", "on": false })).unwrap());

    let mut rec = aylis_ide::profiler::Recording::default();
    rec.add(&answer);
    let rows = aylis_ide::profiler::aggregate(&rec.blocks);
    let handler = rows.iter().find(|r| r.name == "frmA.lbl_click").unwrap_or_else(|| panic!("{answer}"));
    assert_eq!(handler.calls, 1);
    assert!(handler.total >= 0.015, "{}", handler.total);
    // O tempo da marca fica fora do próprio do handler.
    let mark = rows.iter().find(|r| r.name == "consulta lenta").expect("a marca");
    assert!(mark.total >= 0.015 && handler.own < 0.005, "próprio {}", handler.own);
    assert!(rows.iter().any(|r| r.lane == aylis_core::profile::Lane::Layout), "layout: {rows:?}");
    assert!(!rec.memory.is_empty(), "a memória do app");
}
