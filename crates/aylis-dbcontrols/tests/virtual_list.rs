//! DataRepeater: um item por registro, com os campos ligados pelo `bindings`, e a lista
//! acompanha o dataset.

use aylis_controls::{Form, Label, Panel, VirtualList};
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

fn text(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

#[test]
fn one_item_per_record_and_it_follows_the_data_set() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("lista", 400.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String).size(30), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let data = (0..1000).map(|i| vec![(i as i64 + 1).into(), format!("Cliente {}", i + 1).into()]).collect();
    data_set(&mut ui, cds).load_rows(data).unwrap();

    let list = ui.add(DataRepeater::default(), form);
    ui.control_mut(list).unwrap().set_rect(0.0, 0.0, 400.0, 300.0);
    let template = ui.add(Panel::new(400.0, 60.0), list);
    let name = ui.add(Label::new(""), template);
    ui.set_name(name, "lblNome").unwrap();
    ui.set_prop(list, "item_height", 60.0).unwrap();
    ui.set_prop(list, "bindings", "lblNome=nome").unwrap();
    ui.set_prop(list, "data_source", ds).unwrap();
    ui.layout();

    let label_of = |ui: &Ui, i: usize| text(ui, ui.find_in(VirtualList::item(ui, list, i).unwrap(), "lblNome").unwrap());
    assert_eq!(VirtualList::visible(&ui, list).len(), 7, "300 / 60 = 5, mais 2");
    assert_eq!(label_of(&ui, 0), "Cliente 1");
    assert_eq!(label_of(&ui, 4), "Cliente 5");

    // Mudou o nome no dataset: a linha acompanha.
    {
        let mut d = data_set(&mut ui, cds);
        d.first().unwrap();
        d.edit().unwrap();
        d.set_value("nome", "Ana").unwrap();
        d.post().unwrap();
    }
    ui.layout();
    assert_eq!(label_of(&ui, 0), "Ana");
}

/// Como no RTSPlayer: o form vem do JSON, o rótulo fica fundo no modelo (linha → cartão →
/// rótulo) e os dados chegam depois, por `load_json`. As linhas mostravam o texto do modelo
/// (relatório do MCP, 10/11).
#[test]
fn the_bindings_fill_rows_loaded_from_json() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    aylis_data::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let json = r#"{"aylis": 1, "root": {"class": "Form", "name": "fraLista", "width": 400, "height": 400, "children": [
        {"class": "ClientDataSet", "name": "cdsLista", "children": [
            {"class": "Field", "name": "fldTitulo", "field_name": "titulo"}
        ]},
        {"class": "DataSource", "name": "dsLista", "data_set": "cdsLista"},
        {"class": "DataRepeater", "name": "drLista", "data_source": "dsLista", "item_height": 80, "bindings": "lblNome=titulo", "align": "client", "children": [
            {"class": "Layout", "name": "linItem", "height": 80, "align": "top", "children": [
                {"class": "Panel", "name": "crdItem", "height": 72, "align": "top", "children": [
                    {"class": "Label", "name": "lblNome", "text": "nome", "align": "top", "height": 24}
                ]}
            ]}
        ]}
    ]}}"#;
    let form = aylis_json::load(&mut ui, json, None).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    let cds = ui.find_in(form, "cdsLista").unwrap();
    data_set(&mut ui, cds).load_json(&serde_json::json!([{"titulo": "ayla"}, {"titulo": "bia"}])).unwrap();
    ui.layout();
    let list = ui.find_in(form, "drLista").unwrap();
    let label_of = |ui: &Ui, i: usize| text(ui, ui.find_in(VirtualList::item(ui, list, i).expect("a linha"), "lblNome").expect("o rótulo na linha"));
    assert_eq!(label_of(&ui, 0), "ayla");
    assert_eq!(label_of(&ui, 1), "bia");
}

/// O botão dentro da linha chama o handler do form (a linha é um escopo de nomes, e o
/// handler do form não era achado); o handler sabe o registro (`index_of`), acha a peça da
/// linha pelo componente do modelo (`part`), e os `bindings` preenchem outra propriedade
/// (`btnLado.enabled=ativo`). Relatório do RTSPlayer, 10/11 (itens 11 e 14).
#[test]
fn a_click_inside_a_row_reaches_the_form_and_knows_the_record() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    aylis_data::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let json = r#"{"aylis": 1, "root": {"class": "Form", "name": "fraInicio", "width": 400, "height": 400, "children": [
        {"class": "ClientDataSet", "name": "cdsLista", "children": [
            {"class": "Field", "name": "fldTitulo", "field_name": "titulo"},
            {"class": "Field", "name": "fldAtivo", "field_name": "ativo"}
        ]},
        {"class": "DataSource", "name": "dsLista", "data_set": "cdsLista"},
        {"class": "DataRepeater", "name": "drLista", "data_source": "dsLista", "item_height": 80, "bindings": "lblNome=titulo;btnLado.enabled=ativo", "align": "client", "children": [
            {"class": "Panel", "name": "crdItem", "height": 72, "align": "top", "children": [
                {"class": "Label", "name": "lblNome", "text": "nome", "align": "top", "height": 24},
                {"class": "Button", "name": "btnLado", "text": "ver", "align": "right", "width": 60, "on_click": "btnLado_click"}
            ]}
        ]}
    ]}}"#;
    let form = aylis_json::load(&mut ui, json, None).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    let cds = ui.find_in(form, "cdsLista").unwrap();
    data_set(&mut ui, cds).load_json(&serde_json::json!([{"titulo": "ayla", "ativo": "true"}, {"titulo": "bia", "ativo": "false"}])).unwrap();
    ui.layout();
    let list = ui.find_in(form, "drLista").unwrap();
    let model_button = ui.find_in(form, "btnLado").unwrap();
    let clicked = std::rc::Rc::new(std::cell::Cell::new(None));
    let seen = clicked.clone();
    ui.register_handler("fraInicio.btnLado_click", move |ui, sender, _| seen.set(VirtualList::index_of(ui, list, sender)));

    let button_of = |ui: &Ui, i: usize| VirtualList::part(ui, list, i, model_button).expect("o botão na linha");
    assert_ne!(button_of(&ui, 1), model_button);
    assert_eq!(ui.get_prop(button_of(&ui, 0), "enabled").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(button_of(&ui, 1), "enabled").unwrap(), PropValue::Bool(false));
    ui.fire(button_of(&ui, 0), "on_click", &EventArgs::None);
    assert_eq!(clicked.get(), Some(0), "o handler do form recebeu o clique da linha 0");
    // Pelo `invoke` (o protocolo e as outras linguagens), o mesmo.
    assert_eq!(ui.invoke(list, "index_of", &[PropValue::Component(Some(button_of(&ui, 1)))]).unwrap(), PropValue::Int(1));
    assert_eq!(ui.invoke(list, "part", &[PropValue::Int(1), PropValue::Component(Some(model_button))]).unwrap(), PropValue::Component(Some(button_of(&ui, 1))));
}
