use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{CheckBox, Form, Label};
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

struct Screen {
    ui: Ui,
    form: ComponentId,
    cds: ComponentId,
    name: ComponentId,
    limit: ComponentId,
    title: ComponentId,
    active: ComponentId,
    nav: ComponentId,
}

fn screen() -> Screen {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);

    let form = ui.add_root(Form::new("dados", 600.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::AutoInc), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").required(), cds);
    ui.add(Field::new("limite", FieldType::Currency).label("Limite").format("R$ #,##0.00").min(0.0), cds);
    ui.add(Field::new("ativo", FieldType::Boolean), cds);
    let ds = ui.add(DataSource::new(cds), form);

    let nav = ui.add(DataNavigator::new(ds), form);
    ui.control_mut(nav).unwrap().set_rect(0.0, 0.0, 306.0, 32.0);
    let title = ui.add(Label::new(""), form);
    ui.control_mut(title).unwrap().set_rect(0.0, 40.0, 200.0, 20.0);
    ui.set_binding(title, "text", Some(ds), "nome");
    let name = ui.add(CellEditor::new(ds, "nome"), form);
    ui.control_mut(name).unwrap().set_rect(0.0, 70.0, 200.0, 32.0);
    let limit = ui.add(CellEditor::new(ds, "limite"), form);
    ui.control_mut(limit).unwrap().set_rect(0.0, 110.0, 200.0, 32.0);
    let active = ui.add(CheckBox::new("Ativo"), form);
    ui.control_mut(active).unwrap().set_rect(0.0, 150.0, 120.0, 24.0);
    ui.set_binding(active, "checked", Some(ds), "ativo");

    let rows = vec![
        vec![1.into(), "Ana".into(), Decimal::parse("1500").unwrap().into(), true.into()],
        vec![2.into(), "Bruno".into(), Decimal::parse("20").unwrap().into(), false.into()],
    ];
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    ui.layout();
    Screen { ui, form, cds, name, limit, title, active, nav }
}

fn edit_text(ui: &Ui, id: ComponentId) -> String {
    ui.get::<CellEditor>(id).unwrap().base.text.clone()
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    ui.pointer_move(form, at(x, y));
    ui.pointer_down(form, at(x, y));
    ui.pointer_up(form, at(x, y));
}

/// Centro do botão do navegador.
fn nav_button(button: NavButton) -> (f32, f32) {
    let i = NavButton::ALL.iter().position(|&b| b == button).unwrap() as f32;
    (i * 34.0 + 17.0, 16.0)
}

#[test]
fn controls_show_the_current_record_and_follow_the_cursor() {
    let mut s = screen();
    assert_eq!(s.ui.get_prop(s.title, "text").unwrap(), PropValue::String("Ana".into()));
    assert_eq!(edit_text(&s.ui, s.limit), "R$ 1.500,00", "texto de exibição");
    assert_eq!(s.ui.get_prop(s.active, "checked").unwrap(), PropValue::Bool(true));

    data_set(&mut s.ui, s.cds).next().unwrap();
    s.ui.process_posted();
    assert_eq!(s.ui.get_prop(s.title, "text").unwrap(), PropValue::String("Bruno".into()));
    assert_eq!(edit_text(&s.ui, s.name), "Bruno");
    assert_eq!(s.ui.get_prop(s.active, "checked").unwrap(), PropValue::Bool(false));
}

#[test]
fn typing_starts_editing_and_exit_writes_the_field() {
    let mut s = screen();
    s.ui.set_focus(s.limit);
    assert_eq!(edit_text(&s.ui, s.limit), "1500,00", "com foco, texto de edição");
    s.ui.text_input(s.form, "2.750,5"); // substitui a seleção (tudo)
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Edit, "auto_edit");
    assert_eq!(data_set(&mut s.ui, s.cds).text("limite").unwrap(), "R$ 1.500,00", "ainda não gravou");

    s.ui.set_focus(s.name);
    let ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.text("limite").unwrap(), "R$ 2.750,50");
    assert_eq!(edit_text(&s.ui, s.limit), "R$ 2.750,50", "sem foco, volta ao texto de exibição");
}

#[test]
fn navigator_post_writes_pending_text_and_validates() {
    let mut s = screen();
    let errors = Rc::new(RefCell::new(Vec::new()));
    let e = errors.clone();
    s.ui.on(s.nav, "on_error", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            e.borrow_mut().push(t.clone());
        }
    });

    // valor inválido: post recusado, borda de erro, continua em edição
    s.ui.set_focus(s.limit);
    s.ui.text_input(s.form, "-10");
    let (x, y) = nav_button(NavButton::Post);
    click(&mut s.ui, s.form, x, y);
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Edit);
    assert!(s.ui.get::<CellEditor>(s.limit).unwrap().error().unwrap().contains("mínimo"));
    assert_eq!(errors.borrow().len(), 1, "{:?}", errors.borrow());

    // Esc desfaz; corrige e grava pelo navegador (sem sair do campo)
    s.ui.key_down(s.form, KeyEvent { key: Key::Escape, ..Default::default() });
    assert_eq!(edit_text(&s.ui, s.limit), "1500,00");
    s.ui.text_input(s.form, "99");
    click(&mut s.ui, s.form, x, y);
    let ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.state(), DataSetState::Browse);
    assert_eq!(ds.text("limite").unwrap(), "R$ 99,00");
    assert_eq!(ds.change_count(), 1);
}

#[test]
fn navigator_buttons_follow_the_dataset_state() {
    let mut s = screen();
    let nav = |ui: &Ui, b| ui.get::<DataNavigator>(s.nav).unwrap().is_enabled(b);
    assert!(!nav(&s.ui, NavButton::First) && !nav(&s.ui, NavButton::Prior), "no primeiro registro");
    assert!(nav(&s.ui, NavButton::Next) && nav(&s.ui, NavButton::Insert));
    assert!(!nav(&s.ui, NavButton::Post));

    let (x, y) = nav_button(NavButton::Last);
    click(&mut s.ui, s.form, x, y);
    assert_eq!(data_set(&mut s.ui, s.cds).rec_no(), Some(1));
    assert!(!nav(&s.ui, NavButton::Next) && nav(&s.ui, NavButton::Prior));

    let (x, y) = nav_button(NavButton::Insert);
    click(&mut s.ui, s.form, x, y);
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Insert);
    assert!(nav(&s.ui, NavButton::Post) && nav(&s.ui, NavButton::Cancel) && !nav(&s.ui, NavButton::Insert));
    assert_eq!(edit_text(&s.ui, s.name), "", "registro novo em branco");

    // post sem nome: obrigatório
    let (px, py) = nav_button(NavButton::Post);
    click(&mut s.ui, s.form, px, py);
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Insert);

    let (cx, cy) = nav_button(NavButton::Cancel);
    click(&mut s.ui, s.form, cx, cy);
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Browse);
    assert_eq!(data_set(&mut s.ui, s.cds).record_count(), 2);
}

#[test]
fn checkbox_writes_the_boolean_field() {
    let mut s = screen();
    click(&mut s.ui, s.form, 5.0, 160.0);
    let ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.state(), DataSetState::Edit);
    assert_eq!(ds.value("ativo").unwrap(), Value::Bool(false));
    assert_eq!(s.ui.get_prop(s.active, "checked").unwrap(), PropValue::Bool(false));
}

#[test]
fn read_only_datasets_block_typing() {
    let mut s = screen();
    s.ui.get_mut::<ClientDataSet>(s.cds).unwrap().read_only = true;
    data_set(&mut s.ui, s.cds).refresh_links();
    s.ui.set_focus(s.name);
    s.ui.text_input(s.form, "X");
    assert_eq!(edit_text(&s.ui, s.name), "Ana");
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Browse);
}

#[test]
fn a_data_screen_described_in_json() {
    let json = r#"{
      "root": {
        "class": "Form", "name": "FrmProdutos",
        "children": [
          { "class": "ClientDataSet", "name": "cdsProdutos", "key_fields": "codigo", "active": true,
            "children": [
              { "class": "Field", "name": "codigo", "type": "integer" },
              { "class": "Field", "name": "descricao", "type": "string", "size": 40, "display_label": "Descrição", "required": true },
              { "class": "Field", "name": "preco", "type": "currency", "display_format": "R$ #,##0.00" }
            ] },
          { "class": "DataSource", "name": "dsProdutos", "data_set": "cdsProdutos" },
          { "class": "DataNavigator", "data_source": "dsProdutos", "align": "top" },
          { "class": "Edit", "name": "edtDescricao", "align": "top", "bindings": { "text": { "source": "dsProdutos", "field": "descricao" } } },
          { "class": "Label", "name": "lblPreco", "align": "top", "bindings": { "text": { "source": "dsProdutos", "field": "preco" } } }
        ]
      }
    }"#;
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    aylis_json::load(&mut ui, json, None).unwrap();
    aylis_data::open_active(&mut ui);

    let cds = ui.find("cdsProdutos").unwrap();
    let rows = serde_json_rows();
    data_set(&mut ui, cds).load_json(&rows).unwrap();
    ui.process_posted();
    let edit = ui.find("edtDescricao").unwrap();
    assert_eq!(ui.get_prop(edit, "text").unwrap(), PropValue::String("Teclado".into()));
    assert_eq!(ui.get_prop(ui.find("lblPreco").unwrap(), "text").unwrap(), PropValue::String("R$ 149,90".into()));

    // e o layout com dados volta para JSON igual (as ligações, não os valores)
    let saved = aylis_json::save(&ui, ui.find("FrmProdutos").unwrap()).unwrap();
    assert!(saved.contains("\"data_source\": \"dsProdutos\"") && saved.contains("\"bindings\"") && saved.contains("\"type\": \"currency\""), "{saved}");
    assert!(!saved.contains("Teclado"), "{saved}");
}

fn serde_json_rows() -> serde_json::Value {
    serde_json::json!([ { "codigo": 1, "descricao": "Teclado", "preco": "149.90" } ])
}

#[test]
fn navigator_asks_before_action_and_a_refusal_keeps_the_record() {
    // Item 18 (04/11): o botão pergunta antes (o `BeforeAction` do Delphi), e um
    // `EventInterceptor` pode recusar no lugar de cada form.
    let mut s = screen();
    let asked = Rc::new(RefCell::new(Vec::new()));
    let a = asked.clone();
    s.ui.on(s.nav, "before_action", move |ui, _, args| {
        let button = args.text().unwrap_or_default().to_string();
        if button == "delete" {
            ui.refuse("excluir não é permitido");
        }
        a.borrow_mut().push(button);
    });
    let (x, y) = nav_button(NavButton::Delete);
    click(&mut s.ui, s.form, x, y);
    s.ui.process_posted();
    assert_eq!(*asked.borrow(), ["delete"]);
    assert_eq!(data_set(&mut s.ui, s.cds).record_count(), 2, "recusado: o registro fica");

    let (x, y) = nav_button(NavButton::Last);
    click(&mut s.ui, s.form, x, y);
    s.ui.process_posted();
    assert_eq!(data_set(&mut s.ui, s.cds).rec_no(), Some(1), "aceito: o botão age");
}
