//! Ligações a dataset ("Ligar a campo…", o LiveBindings): qualquer propriedade de qualquer
//! componente mostra um campo, um texto montado ou uma expressão, e as editáveis gravam de
//! volta.

use aylis_controls::{CheckBox, Edit, Form, Label, TrackBar};
use aylis_core::*;
use aylis_data::*;

fn setup() -> (Ui, ComponentId, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("dados", 600.0, 400.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("nome", FieldType::String), cds);
    ui.add(Field::new("preco", FieldType::Currency), cds);
    ui.add(Field::new("quantidade", FieldType::Integer), cds);
    ui.add(Field::new("ativo", FieldType::Boolean), cds);
    let ds = ui.add(DataSource::new(cds), form);
    ui.set_prop(ds, "auto_edit", true).unwrap();
    let rows = vec![
        vec!["Café".into(), Decimal::parse("12.5").unwrap().into(), 3.into(), true.into()],
        vec!["Pão".into(), Decimal::parse("0.75").unwrap().into(), 10.into(), false.into()],
    ];
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    (ui, form, cds, ds)
}

#[test]
fn any_property_shows_a_field_a_template_or_an_expression() {
    let (mut ui, form, cds, ds) = setup();
    let name = ui.add(Label::new(""), form);
    let total = ui.add(Label::new(""), form);
    let sum = ui.add(Label::new(""), form);
    let active = ui.add(CheckBox::new("Ativo"), form);
    let qty = ui.add(TrackBar::default(), form);
    ui.set_binding(name, "text", Some(ds), "nome");
    ui.set_binding(total, "text", Some(ds), "Total: {preco:moeda} x {quantidade}");
    ui.set_binding(sum, "text", Some(ds), "=preco * quantidade");
    ui.set_binding(active, "checked", Some(ds), "ativo");
    ui.set_binding(qty, "value", Some(ds), "quantidade");
    // Já mostra ao ligar.
    assert_eq!(ui.get_prop(name, "text").unwrap(), PropValue::String("Café".into()));
    let PropValue::String(t) = ui.get_prop(total, "text").unwrap() else { panic!() };
    assert!(t.starts_with("Total: ") && t.contains("12") && t.ends_with(" x 3"), "{t}");
    assert_eq!(ui.get_prop(sum, "text").unwrap(), PropValue::String("37.5".into()));
    assert_eq!(ui.get_prop(active, "checked").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(qty, "value").unwrap(), PropValue::Float(3.0));
    // E acompanha o registro atual.
    data_set(&mut ui, cds).next().unwrap();
    ui.process_posted();
    assert_eq!(ui.get_prop(name, "text").unwrap(), PropValue::String("Pão".into()));
    assert_eq!(ui.get_prop(active, "checked").unwrap(), PropValue::Bool(false));
    assert_eq!(ui.get_prop(sum, "text").unwrap(), PropValue::String("7.5".into()));
}

#[test]
fn editable_properties_write_back_and_the_layout_keeps_the_bindings() {
    let (mut ui, form, cds, ds) = setup();
    ui.set_name(ds, "dsProdutos").unwrap();
    let edit = ui.add(Edit::new(""), form);
    ui.set_name(edit, "edtNome").unwrap();
    let check = ui.add(CheckBox::new("Ativo"), form);
    ui.set_binding(edit, "text", Some(ds), "nome");
    ui.set_binding(check, "checked", Some(ds), "ativo");
    // O usuário muda: o evento de mudança grava no campo (o dataset entra em edição sozinho).
    ui.set_prop(edit, "text", "Café forte").unwrap();
    ui.fire(edit, "on_change", &EventArgs::None);
    ui.set_prop(check, "checked", false).unwrap();
    ui.fire(check, "on_click", &EventArgs::None);
    {
        let v = view(&ui, cds);
        assert!(v.state().is_editing());
        assert_eq!(v.value("nome").unwrap(), Value::String("Café forte".into()));
        assert_eq!(v.value("ativo").unwrap(), Value::Bool(false));
    }
    data_set(&mut ui, cds).post().unwrap();
    // O form grava e lê as ligações.
    let json = aylis_json::save(&ui, form).unwrap();
    assert!(json.contains("\"bindings\""), "{json}");
    let mut ui2 = Ui::new();
    aylis_controls::register(&mut ui2);
    aylis_data::register(&mut ui2);
    aylis_json::load(&mut ui2, &json, None).unwrap();
    let edit2 = ui2.find("edtNome").unwrap();
    let ds2 = ui2.find("dsProdutos").unwrap();
    let b = ui2.binding(edit2, "text").unwrap();
    assert_eq!((b.source, b.path.as_str()), (Some(ds2), "nome"));
}

/// O `Chart` ligado a um dataset (no lugar do `DBChart`): os rótulos de um campo e uma série
/// por campo, um ponto por registro, acompanhando o dataset.
#[test]
fn a_chart_shows_the_dataset_through_bindings() {
    let (mut ui, form, cds, ds) = setup();
    let chart = ui.add(aylis_controls::Chart::default(), form);
    ui.set_binding(chart, "labels", Some(ds), "nome");
    ui.set_binding(chart, "series", Some(ds), "Preço[line;#EF4444]=preco|Qtd=quantidade");
    assert_eq!(ui.get_prop(chart, "labels").unwrap(), PropValue::String("Café;Pão".into()));
    assert_eq!(ui.get_prop(chart, "series").unwrap(), PropValue::String("Preço[line;#EF4444]=12.5,0.75|Qtd=3,10".into()));
    let c = ui.get::<aylis_controls::Chart>(chart).unwrap();
    assert_eq!(c.data().len(), 2);
    assert_eq!(c.data()[0].name, "Preço");
    // Só os campos: o nome da série é o do campo.
    ui.set_binding(chart, "series", Some(ds), "quantidade");
    assert_eq!(ui.get_prop(chart, "series").unwrap(), PropValue::String("quantidade=3,10".into()));
    // Um registro novo entra no gráfico.
    {
        let mut d = data_set(&mut ui, cds);
        d.append().unwrap();
        d.set_text("nome", "Leite").unwrap();
        d.set_value("quantidade", 2).unwrap();
        d.post().unwrap();
    }
    ui.process_posted();
    assert_eq!(ui.get_prop(chart, "labels").unwrap(), PropValue::String("Café;Pão;Leite".into()));
    assert_eq!(ui.get_prop(chart, "series").unwrap(), PropValue::String("quantidade=3,10,2".into()));
}

#[test]
fn a_combo_box_lists_one_dataset_and_writes_the_code_in_another() {
    let (mut ui, form, _, _) = setup();
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("nome", FieldType::String), cds);
    ui.add(Field::new("cidade", FieldType::Integer), cds);
    let ds = ui.add(DataSource::new(cds), form);
    ui.set_prop(ds, "auto_edit", true).unwrap();
    data_set(&mut ui, cds).load_rows(vec![vec!["Ana".into(), 10.into()], vec!["Bia".into(), Value::Null]]).unwrap();
    let cities = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("id", FieldType::Integer), cities);
    ui.add(Field::new("nome", FieldType::String), cities);
    let list = ui.add(DataSource::new(cities), form);
    data_set(&mut ui, cities).load_rows(vec![vec![10.into(), "Santos".into()], vec![20.into(), "Campinas".into()]]).unwrap();
    let combo = ui.add(aylis_controls::ComboBox::default(), form);
    ui.set_binding(combo, "value", Some(ds), "cidade");
    ui.set_binding(combo, "items", Some(list), "nome");
    ui.set_binding(combo, "values", Some(list), "id");
    assert_eq!(ui.get_prop(combo, "items").unwrap(), PropValue::String("Santos;Campinas".into()));
    assert_eq!(ui.get_prop(combo, "values").unwrap(), PropValue::String("10;20".into()));
    // Escolher "Campinas" grava 20 no campo.
    ui.set_prop(combo, "item_index", 1).unwrap();
    ui.fire(combo, "on_change", &EventArgs::None);
    assert_eq!(view(&ui, cds).value("cidade").unwrap(), Value::Int(20));
    data_set(&mut ui, cds).post().unwrap();
    data_set(&mut ui, cds).next().unwrap();
    assert_eq!(ui.get_prop(combo, "item_index").unwrap(), PropValue::Int(-1), "o outro registro não tem cidade");
}

#[test]
fn the_record_arriving_fires_on_change_without_writing_back() {
    // Como o `TDBEdit` da VCL: o registro muda, o valor chega na tela e o `on_change` sai; mas
    // é o campo chegando, não o usuário mudando: o dataset não entra em edição.
    let (mut ui, form, cds, ds) = setup();
    let edit = ui.add(Edit::new(""), form);
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.on(edit, "on_change", move |_, _, args| s.borrow_mut().push(args.text().unwrap_or_default().to_string()));
    ui.set_binding(edit, "text", Some(ds), "nome");
    data_set(&mut ui, cds).next().unwrap();
    ui.process_posted();
    assert_eq!(*seen.borrow(), vec!["Café".to_string(), "Pão".to_string()]);
    let v = view(&ui, cds);
    assert!(!v.state().is_editing(), "o valor que chegou não volta para o campo");
    assert_eq!(v.value("nome").unwrap(), Value::String("Pão".into()));
}
