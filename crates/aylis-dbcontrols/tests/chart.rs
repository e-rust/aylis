//! DBChart: o gráfico acompanha os registros do dataset.

use aylis_controls::Form;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

#[test]
fn the_chart_follows_the_records() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("dados", 600.0, 400.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("mes", FieldType::String), cds);
    ui.add(Field::new("vendas", FieldType::Currency), cds);
    ui.add(Field::new("custos", FieldType::Integer), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let chart = ui.add(DBChart::new(ds, "mes", &["vendas", "custos"]), form);
    ui.set_prop(chart, "kind", "bar").unwrap();
    let rows = vec![
        vec!["Jan".into(), Decimal::parse("10.5").unwrap().into(), 4.into()],
        vec!["Fev".into(), Decimal::parse("20").unwrap().into(), 7.into()],
    ];
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    ui.process_posted();
    let c = &ui.get::<DBChart>(chart).unwrap().base;
    assert_eq!(c.data_labels(), ["Jan", "Fev"]);
    assert_eq!(c.data()[0].values, [10.5, 20.0]);
    assert_eq!((c.data()[1].name.as_str(), c.data()[1].values.clone()), ("custos", vec![4.0, 7.0]));
    // Um registro novo entra no gráfico; a cor escolhida para a série continua.
    ui.get_mut::<DBChart>(chart).unwrap().base.set_series_visible(1, false);
    {
        let mut d = data_set(&mut ui, cds);
        d.append().unwrap();
        d.set_value("mes", "Mar").unwrap();
        d.set_value("vendas", Decimal::parse("5").unwrap()).unwrap();
        d.set_value("custos", 2).unwrap();
        d.post().unwrap();
    }
    ui.process_posted();
    let c = &ui.get::<DBChart>(chart).unwrap().base;
    assert_eq!(c.data_labels(), ["Jan", "Fev", "Mar"]);
    assert_eq!(c.data()[1].values, [4.0, 7.0, 2.0]);
    assert!(c.data()[1].hidden, "o que a legenda escondeu continua escondido");
    assert_eq!(ui.get_prop(chart, "kind").unwrap(), PropValue::Enum("bar".into()));
}
