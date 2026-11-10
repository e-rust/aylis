//! Consulta grande no servidor (item 17): com os dados em partes (o backend tem mais do que os
//! carregados), ordenar pelo título, filtrar e localizar viram o evento `on_server_query`, que o
//! backend trata recarregando a consulta; sem quem trate, a grade avisa que valeu só para os
//! carregados. O total pode vir do backend (`record_total`), e o `max_records` limita a memória.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

struct Setup {
    ui: Ui,
    form: ComponentId,
    cds: ComponentId,
}

/// Grade de 600 × 332 com os 50 primeiros de uma consulta de 1000 nomes (do 1000 ao 1).
fn setup() -> Setup {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(30), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 332.0);
    ui.layout();
    data_set(&mut ui, cds).load_rows(page(0, 50, false)).unwrap();
    data_set(&mut ui, cds).set_has_more(true).unwrap();
    ui.set_focus(grid);
    Setup { ui, form, cds }
}

/// Os registros `from..to` da consulta (o id e o nome), na ordem do banco (`asc`: do 1 ao
/// 1000; senão, do 1000 ao 1).
fn page(from: usize, to: usize, asc: bool) -> Vec<Vec<Value>> {
    (from..to).map(|i| if asc { i + 1 } else { 1000 - i }).map(|n| vec![(n as i64).into(), format!("Nome {n:04}").into()]).collect()
}

fn texts(s: &mut Setup) -> Vec<String> {
    let mut canvas = RecordingCanvas::new(None);
    s.ui.layout();
    s.ui.paint(s.form, &mut canvas);
    canvas.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { Some(text.clone()) } else { None }).collect()
}

fn click_title(s: &mut Setup, x: f32) {
    let at = PointerEvent { position: PointF::new(x, 16.0), ..Default::default() };
    s.ui.pointer_move(s.form, at);
    s.ui.pointer_down(s.form, at);
    s.ui.pointer_up(s.form, at);
}

#[test]
fn without_a_handler_the_grid_warns_that_only_the_loaded_rows_were_sorted() {
    let mut s = setup();
    assert!(!texts(&mut s).iter().any(|t| t.contains('⚠')), "sem aviso antes");
    click_title(&mut s, 112.0); // o título "Nome"
    let notice = data_set(&mut s.ui, s.cds).partial_notice();
    assert!(notice.contains("Ordenado só entre os 50 registros carregados"), "{notice}");
    assert!(texts(&mut s).iter().any(|t| t.contains('⚠') && t.contains("50 registros")), "a grade mostra o aviso");
    // Filtrar e localizar também avisam.
    data_set(&mut s.ui, s.cds).set_filter("nome like 'Nome 09%'").unwrap();
    assert!(data_set(&mut s.ui, s.cds).partial_notice().starts_with("Filtrado só"));
    data_set(&mut s.ui, s.cds).set_filter("").unwrap();
    assert!(!data_set(&mut s.ui, s.cds).locate("nome", "Nome 0001").unwrap(), "o 1 não está entre os carregados");
    assert!(data_set(&mut s.ui, s.cds).partial_notice().starts_with("Procurado só"));
    // Tudo carregado: o aviso some (a ordem local agora vale para tudo).
    data_set(&mut s.ui, s.cds).append_rows(page(50, 1000, false), false).unwrap();
    assert_eq!(data_set(&mut s.ui, s.cds).partial_notice(), "");
}

#[test]
fn with_a_handler_the_backend_reloads_in_the_asked_order() {
    let mut s = setup();
    let asked: Rc<RefCell<Vec<serde_json::Value>>> = Rc::default();
    let log = asked.clone();
    s.ui.on(s.cds, "on_server_query", move |ui, cds, args| {
        let query: serde_json::Value = serde_json::from_str(args.text().unwrap()).unwrap();
        log.borrow_mut().push(query.clone());
        // O backend refaz a consulta na ordem pedida e manda a primeira página.
        let asc = query["index"].as_str().is_some_and(|i| !i.contains("desc"));
        let mut ds = data_set(ui, cds);
        ds.load_rows(page(0, 50, asc)).unwrap();
        ds.set_has_more(true).unwrap();
        // Mexer no dataset durante o evento não pergunta de novo.
        ds.set_index(query["index"].as_str().unwrap()).unwrap();
    });
    click_title(&mut s, 112.0); // "Nome" crescente
    assert_eq!(asked.borrow().len(), 1, "uma pergunta, sem repetir: {:?}", asked.borrow());
    let q = asked.borrow()[0].clone();
    assert_eq!((q["kind"].as_str(), q["index"].as_str(), q["loaded"].as_i64()), (Some("sort"), Some("nome"), Some(50)));
    let mut ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.partial_notice(), "", "o backend tratou: sem aviso");
    ds.first().unwrap();
    assert_eq!(ds.value("nome").unwrap(), Value::from("Nome 0001"), "o primeiro da consulta inteira, não dos 50");
    assert!(!texts(&mut s).iter().any(|t| t.contains('⚠')));

    // Localizar fora dos carregados: o backend recebe o campo e o valor.
    data_set(&mut s.ui, s.cds).locate("nome", "Nome 0999").unwrap();
    let q = asked.borrow().last().cloned().unwrap();
    assert_eq!((q["kind"].as_str(), q["field"].as_str(), q["value"].as_str()), (Some("locate"), Some("nome"), Some("Nome 0999")));
}

#[test]
fn the_backend_total_sizes_the_scrollbar_and_the_memory_limit_stops_fetching() {
    let mut s = setup();
    s.ui.set_prop(s.cds, "record_total", 1000i64).unwrap();
    s.ui.process_posted();
    let _ = texts(&mut s);
    // O polegar da barra mostra 10 de 1000 linhas (antes: 10 de 50 + uma página).
    let thumb = s.ui.get::<DataGrid>(s.ui.children(s.form)[2]).map(|g| g.scroll_fraction());
    assert!(thumb.is_some_and(|f| f < 0.02), "polegar: {thumb:?}");

    let fetched = Rc::new(RefCell::new(0));
    let count = fetched.clone();
    s.ui.set_prop(s.cds, "max_records", 100i64).unwrap();
    let mut ds = data_set(&mut s.ui, s.cds);
    ds.on_fetch(move |ui, cds, loaded| {
        *count.borrow_mut() += 1;
        data_set(ui, cds).append_rows(page(loaded, loaded + 50, false), true).unwrap();
    });
    assert!(ds.fetch_more().unwrap());
    assert_eq!(ds.record_count(), 100);
    assert!(!ds.fetch_more().unwrap(), "no limite, não busca mais");
    assert_eq!(*fetched.borrow(), 1);
    assert!(ds.partial_notice().contains("Mostrando os primeiros 100 registros"), "{}", ds.partial_notice());
}
