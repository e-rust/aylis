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
