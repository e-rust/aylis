//! O desempenho no `Analytics` (item 38, 3c): com `performance`, o tempo de cada handler
//! (vezes, média, p95, máximo) e os quadros lentos, somados entre as sessões no arquivo; sem
//! ela (o padrão), nada disso é medido. Num arquivo de teste só seu: o profiler do núcleo é um
//! só por processo.

use aylis_controls::*;
use aylis_core::pages::RecordingCanvas;
use aylis_core::*;

fn session(file: &std::path::Path, performance: bool, clicks: usize) -> serde_json::Value {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("Pedidos", 300.0, 200.0));
    ui.set_name(f, "frmPedidos").unwrap();
    ui.show(f).unwrap();
    let b = ui.add(Button::new("Salvar"), f);
    ui.set_name(b, "btnSalvar").unwrap();
    ui.set_prop(b, "on_click", PropValue::Event(Some("btnSalvarClick".into()))).unwrap();
    ui.register_handler("frmPedidos.btnSalvarClick", |_, _, _| std::thread::sleep(std::time::Duration::from_millis(12)));
    let a = ui.add(Analytics::default(), f);
    ui.set_prop(a, "file", file.display().to_string()).unwrap();
    ui.set_prop(a, "app_name", "Pedidos").unwrap();
    ui.set_prop(a, "performance", performance).unwrap();
    Analytics::start(&mut ui, a);
    for _ in 0..clicks {
        ui.click(b);
        ui.process_events();
    }
    ui.paint(f, &mut RecordingCanvas::new(None));
    Analytics::stop(&mut ui, a);
    assert!(!profile::is_on(), "o Analytics desliga o profiler que ligou");
    serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap()
}

#[test]
fn handler_times_and_frames_add_up_across_sessions_only_when_asked() {
    let dir = std::env::temp_dir().join(format!("aylis_analytics_perf_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    // Desligado (o padrão): nada de desempenho.
    let off = dir.join("off.json");
    assert!(session(&off, false, 2)["performance"].is_null());

    let file = dir.join("analytics.json");
    let first = session(&file, true, 3);
    let h = &first["performance"]["handlers"]["frmPedidos.btnSalvarClick"];
    assert_eq!(h["count"], 3, "{first}");
    assert!(h["mean_ms"].as_f64().unwrap() >= 10.0 && h["max_ms"].as_f64().unwrap() >= h["mean_ms"].as_f64().unwrap(), "{h}");
    assert!(h["p95_ms"].as_f64().unwrap() <= h["max_ms"].as_f64().unwrap(), "{h}");
    assert!(first["performance"]["frames"]["count"].as_u64().unwrap() >= 1, "{first}");

    // A segunda sessão soma com a primeira.
    let second = session(&file, true, 2);
    assert_eq!(second["sessions"], 2);
    assert_eq!(second["performance"]["handlers"]["frmPedidos.btnSalvarClick"]["count"], 5, "{second}");
    let _ = std::fs::remove_dir_all(&dir);
}
