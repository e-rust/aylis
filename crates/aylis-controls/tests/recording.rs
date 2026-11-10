//! EventRecorder (gravação da sessão) e Analytics (métricas de uso).

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;

fn setup(dir: &std::path::Path) -> (Ui, ComponentId, ComponentId) {
    let _ = std::fs::remove_dir_all(dir);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let f = ui.add_root(Form::new("Pedidos", 500.0, 400.0));
    ui.set_name(f, "frmPedidos").unwrap();
    ui.show(f).unwrap();
    let rec = ui.add(EventRecorder::default(), f);
    ui.set_prop(rec, "folder", dir.display().to_string()).unwrap();
    ui.set_prop(rec, "app_name", "Pedidos").unwrap();
    (ui, f, rec)
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), click_count: 1, ..Default::default() }
}

fn key(c: char, ctrl: bool, shift: bool) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers { ctrl, shift, ..Default::default() }, repeat: false }
}

#[test]
fn records_the_session_and_reads_it_back() {
    let dir = std::env::temp_dir().join(format!("aylis_recording_{}", std::process::id()));
    let (mut ui, f, rec) = setup(&dir);
    let save = ui.add(Button::new("Salvar"), f);
    ui.set_name(save, "btnSalvar").unwrap();
    ui.control_mut(save).unwrap().set_rect(10.0, 10.0, 120.0, 32.0);
    let pass = ui.add(Edit::new(""), f);
    ui.set_name(pass, "edtSenha").unwrap();
    ui.control_mut(pass).unwrap().set_rect(10.0, 60.0, 200.0, 32.0);
    ui.set_prop(pass, "password", true).unwrap();
    ui.on_click(save, |_, _| {});
    ui.layout();

    // O que a plataforma conta de si vai para o cabeçalho.
    ui.set_platform_info("renderizador", "Skia GPU (OpenGL)");
    // Com `active` (o padrão), começa sozinho no primeiro tique.
    assert!(!EventRecorder::recording(&ui, rec));
    ui.set_time(1.0);
    ui.run_timers();
    ui.process_posted();
    assert!(EventRecorder::recording(&ui, rec), "começou sozinho");
    ui.set_time(2.0);
    assert!(!ui.run_timers(), "depois de começar, o tique de partida sai da agenda");
    let file = std::path::PathBuf::from(ui.get::<EventRecorder>(rec).unwrap().file_name.clone());

    ui.pointer_down(f, at(20.0, 20.0));
    ui.pointer_up(f, at(20.0, 20.0));
    ui.key_down(f, key('s', true, false));
    ui.pointer_down(f, at(20.0, 70.0));
    ui.pointer_up(f, at(20.0, 70.0));
    ui.text_input(f, "segredo");
    Analytics::track(&mut ui, "order_saved", "1250.00");
    ui.report(&activity::Activity::Error { message: "falhou ao salvar" });
    let dlg = message_dlg(&mut ui, "Sair?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], |_, _| {});
    ui.set_modal_result(dlg, ModalResult::No);
    EventRecorder::stop(&mut ui, rec);
    ui.pointer_down(f, at(20.0, 20.0)); // depois de parar: não entra

    let r = Recording::open(&file).unwrap();
    assert_eq!(r.header["app"], "Pedidos");
    assert_eq!(r.header["platform"]["renderizador"], "Skia GPU (OpenGL)");
    let types: Vec<&str> = r.entries.iter().map(|e| e["type"].as_str().unwrap()).collect();
    assert!(types.contains(&"pointer_down") && types.contains(&"key") && types.contains(&"text"), "{types:?}");
    let click = r.entries.iter().find(|e| e["type"] == "pointer_down").unwrap();
    assert_eq!(click["target"], "frmPedidos.btnSalvar");
    assert_eq!(click["target_text"], "Salvar");
    assert!(r.entries.iter().any(|e| e["type"] == "event" && e["event"] == "on_click" && e["component"] == "frmPedidos.btnSalvar"));
    assert!(r.entries.iter().any(|e| e["type"] == "key" && e["key"] == "Ctrl+S"));
    // Campo de senha: só asteriscos.
    let typed = r.entries.iter().find(|e| e["type"] == "text").unwrap();
    assert_eq!(typed["text"], "*******");
    assert_eq!(typed["focus"], "frmPedidos.edtSenha");
    assert!(r.entries.iter().any(|e| e["type"] == "metric" && e["name"] == "order_saved"));
    assert_eq!(r.errors(), 1);
    assert!(r.entries.iter().any(|e| e["type"] == "form_shown" && e["modal"] == true));
    assert!(r.entries.iter().any(|e| e["type"] == "form_closed"));
    assert_eq!(r.entries.iter().filter(|e| e["type"] == "pointer_down").count(), 2, "o clique depois de parar não entra");
    assert!(Recording::describe(click).contains("Button \"Salvar\""));
    // Uma linha cortada no fim (o app caiu) não estraga a leitura.
    let text = std::fs::read_to_string(&file).unwrap() + "{\"t\": 9.0, \"type\": \"ke";
    assert_eq!(Recording::parse(&text).unwrap().entries.len(), r.entries.len());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn analytics_sums_usage_between_sessions() {
    let dir = std::env::temp_dir().join(format!("aylis_analytics_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let file = dir.join("analytics.json");
    for session in 1..=2u64 {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        let f = ui.add_root(Form::new("Pedidos", 500.0, 400.0));
        ui.set_name(f, "frmPedidos").unwrap();
        ui.show(f).unwrap();
        let b = ui.add(Button::new("Salvar"), f);
        ui.set_name(b, "btnSalvar").unwrap();
        ui.control_mut(b).unwrap().set_rect(10.0, 10.0, 120.0, 32.0);
        ui.layout();
        let a = ui.add(Analytics::default(), f);
        ui.set_prop(a, "file", file.display().to_string()).unwrap();
        ui.set_prop(a, "app_name", "Pedidos").unwrap();
        Analytics::start(&mut ui, a);
        ui.pointer_down(f, at(20.0, 20.0));
        ui.pointer_up(f, at(20.0, 20.0));
        ui.key_down(f, key('s', true, false));
        Analytics::track(&mut ui, "value", "10");
        ui.report(&activity::Activity::Error { message: "x" });
        let m = Analytics::summary(&ui, a).unwrap();
        assert_eq!(m["sessions"], session);
        assert_eq!(m["clicks"]["frmPedidos.btnSalvar"], session);
        assert_eq!(m["shortcuts"]["Ctrl+S"], session);
        assert_eq!(m["metrics"]["value"]["sum"], (10 * session) as f64);
        assert_eq!(m["errors"], session);
        Analytics::stop(&mut ui, a);
    }
    let saved: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(saved["sessions"], 2);
    assert_eq!(saved["last_errors"].as_array().unwrap().len(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_recording_from_the_old_format_opens_with_the_new_keys() {
    let old = concat!(
        "{\"tipo\":\"inicio\",\"formato\":1,\"app\":\"Aylis IDE\",\"inicio_ms\":1,\"so\":\"windows\",\"plataforma\":{}}\n",
        "{\"t\":1.0,\"tipo\":\"ponteiro_baixo\",\"form\":\"frmIde\",\"alvo\":\"frmIde.btnRun\",\"alvo_classe\":\"Button\",\"botao\":\"direito\"}\n",
        "{\"t\":2.0,\"tipo\":\"tecla\",\"tecla\":\"Ctrl+S\",\"foco\":\"frmIde.edt\"}\n",
        "{\"t\":3.0,\"tipo\":\"erro\",\"mensagem\":\"falhou\",\"local\":\"x.rs:1\"}\n",
    );
    let r = Recording::parse(old).unwrap();
    assert_eq!(r.header["start_ms"], 1);
    assert_eq!(r.header["os"], "windows");
    let click = &r.entries[0];
    assert_eq!((click["type"].as_str(), click["target"].as_str(), click["target_class"].as_str(), click["button"].as_str()), (Some("pointer_down"), Some("frmIde.btnRun"), Some("Button"), Some("right")));
    assert_eq!((r.entries[1]["type"].as_str(), r.entries[1]["key"].as_str(), r.entries[1]["focus"].as_str()), (Some("key"), Some("Ctrl+S"), Some("frmIde.edt")));
    assert_eq!(r.errors(), 1);
    assert_eq!(r.entries[2]["message"], "falhou");
}

#[test]
fn personal_data_is_masked_unless_record_sensitive_is_on() {
    let dir = std::env::temp_dir().join(format!("aylis_recording_lgpd_{}", std::process::id()));
    let (mut ui, f, rec) = setup(&dir);
    let cpf = ui.add(Edit::new(""), f);
    ui.set_name(cpf, "edtCpf").unwrap();
    ui.control_mut(cpf).unwrap().set_rect(10.0, 10.0, 200.0, 32.0);
    ui.set_prop(cpf, "sensitive", true).unwrap();
    let pass = ui.add(Edit::new(""), f);
    ui.set_name(pass, "edtSenha").unwrap();
    ui.control_mut(pass).unwrap().set_rect(10.0, 60.0, 200.0, 32.0);
    ui.set_prop(pass, "password", true).unwrap();
    ui.layout();
    let file = EventRecorder::start(&mut ui, rec).unwrap();

    ui.pointer_down(f, at(20.0, 20.0));
    ui.pointer_up(f, at(20.0, 20.0));
    ui.text_input(f, "123");
    // O conteúdo do campo não aparece nem na descrição do foco.
    ui.key_down(f, key('4', false, false));
    // O app liga a gravação dos dados pessoais (fica registrado) e depois desliga o campo.
    ui.set_prop(rec, "record_sensitive", true).unwrap();
    ui.text_input(f, "456");
    ui.set_prop(rec, "record_sensitive", false).unwrap();
    ui.set_prop(cpf, "sensitive", false).unwrap();
    ui.text_input(f, "7");
    // Senha: sempre mascarada, mesmo com o record_sensitive.
    ui.set_prop(rec, "record_sensitive", true).unwrap();
    ui.pointer_down(f, at(20.0, 70.0));
    ui.pointer_up(f, at(20.0, 70.0));
    ui.text_input(f, "segredo");
    EventRecorder::stop(&mut ui, rec);

    let r = Recording::open(&file).unwrap();
    assert_eq!(r.header["record_sensitive"], false);
    let texts: Vec<&str> = r.entries.iter().filter(|e| e["type"] == "text").map(|e| e["text"].as_str().unwrap()).collect();
    assert_eq!(texts, ["***", "456", "7", "*******"]);
    // Antes de ligar o record_sensitive, nada do CPF; a senha, nunca.
    let all = std::fs::read_to_string(&file).unwrap();
    // Até a linha que liga o record_sensitive (pelo tipo: a ordem das chaves do JSON muda
    // quando o serde_json vem com o `preserve_order`, ligado pela IDE nas compilações juntas).
    let before: String = all.lines().take_while(|l| !l.contains("\"type\":\"record_sensitive\"")).collect::<Vec<_>>().join("\n");
    assert!(!before.contains("\"123") && !all.contains("segredo"), "{all}");
    let key4 = r.entries.iter().find(|e| e["type"] == "key").unwrap();
    assert_eq!(key4["key"], "*");
    assert_eq!(key4["focus_masked"], true);
    let changes: Vec<String> = r
        .entries
        .iter()
        .filter(|e| e["type"] == "record_sensitive" || e["type"] == "sensitive")
        .map(|e| format!("{} {}", e["type"].as_str().unwrap(), e["on"]))
        .collect();
    assert_eq!(changes, ["sensitive true", "record_sensitive true", "record_sensitive false", "sensitive false", "record_sensitive true"]);
    let _ = std::fs::remove_dir_all(&dir);
}
