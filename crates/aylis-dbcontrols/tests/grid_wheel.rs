//! A roda do mouse no `DataGrid` dentro de uma `ScrollBox` (pedido do autor em 16/11): a grade
//! só fica com a roda quando rola; na ponta (a última coluna, a última linha, ou sem nada para
//! rolar), a roda segue para a caixa de fora. As colunas rolam com o Shift (o padrão), e o app
//! escolhe outro jeito (`horizontal_wheel`, `scroll_left`, `top_row`, `captures_wheel`).

use aylis_controls::Form;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

struct Setup {
    ui: Ui,
    form: ComponentId,
    scroll: ComponentId,
    grid: ComponentId,
}

/// Uma caixa com rolagem: um painel, a grade larga (5 registros, todos à vista) e outro painel
/// alto, para a caixa ter o que rolar.
fn setup() -> Setup {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("frmLista", 600.0, 400.0));
    let scroll = ui.create("ScrollBox", Some(form)).unwrap();
    ui.set_prop(scroll, "align", "client").unwrap();
    let top = ui.create("Panel", Some(scroll)).unwrap();
    ui.set_prop(top, "align", "top").unwrap();
    ui.set_prop(top, "height", 100.0).unwrap();
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String).size(30), cds);
    ui.add(Field::new("valor", FieldType::Integer), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), scroll);
    ui.set_prop(grid, "align", "top").unwrap();
    ui.set_prop(grid, "height", 250.0).unwrap();
    ui.set_prop(grid, "columns", r#"[{"field": "id", "width": 400}, {"field": "nome", "width": 400}, {"field": "valor", "width": 400}]"#).unwrap();
    let bottom = ui.create("Panel", Some(scroll)).unwrap();
    ui.set_prop(bottom, "align", "top").unwrap();
    ui.set_prop(bottom, "height", 900.0).unwrap();
    ui.layout();
    let rows = (0..5).map(|i| vec![(i as i64).into(), format!("Cliente {i}").into(), (i as i64 * 10).into()]).collect();
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    ui.layout();
    Setup { ui, form, scroll, grid }
}

fn wheel(s: &mut Setup, shift: bool) {
    let modifiers = Modifiers { shift, ..Default::default() };
    s.ui.wheel(s.form, WheelEvent { position: PointF::new(200.0, 200.0), delta: PointF::new(0.0, -120.0), modifiers, ..Default::default() });
    s.ui.layout();
}

fn box_position(s: &Setup) -> f64 {
    match s.ui.get_prop(s.scroll, "scroll_position") {
        Ok(PropValue::Float(v)) => v,
        Ok(PropValue::Int(v)) => v as f64,
        other => panic!("{other:?}"),
    }
}

fn left(s: &Setup) -> f64 {
    match s.ui.get_prop(s.grid, "scroll_left") {
        Ok(PropValue::Float(v)) => v,
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_wheel_goes_on_to_the_box_when_the_grid_cannot_scroll() {
    let mut s = setup();
    // Os 5 registros cabem: a roda comum sobre a grade rola a caixa de fora.
    wheel(&mut s, false);
    assert!(box_position(&s) > 0.0, "a caixa rolou com o mouse sobre a grade");
    assert_eq!(left(&s), 0.0, "sem o Shift, as colunas não rolam");
}

#[test]
fn shift_scrolls_the_columns_up_to_the_last_and_then_lets_go() {
    let mut s = setup();
    wheel(&mut s, true);
    assert!(left(&s) > 0.0, "com o Shift, as colunas rolam");
    assert_eq!(box_position(&s), 0.0, "a caixa fica parada enquanto a grade rola");
    for _ in 0..50 {
        wheel(&mut s, true);
    }
    let end = left(&s);
    let before = box_position(&s);
    wheel(&mut s, true);
    assert_eq!(left(&s), end, "na última coluna, para");
    // Na ponta, a roda saiu da grade: com `captures_wheel`, ela fica.
    s.ui.set_prop(s.grid, "captures_wheel", true).unwrap();
    s.ui.set_prop(s.scroll, "scroll_position", PropValue::Float(before)).unwrap();
    wheel(&mut s, false);
    assert_eq!(box_position(&s), before, "com captures_wheel, a grade segura a roda");
}

#[test]
fn the_app_chooses_how_the_columns_scroll() {
    let mut s = setup();
    s.ui.set_prop(s.grid, "horizontal_wheel", "always").unwrap();
    wheel(&mut s, false);
    assert!(left(&s) > 0.0, "always: a roda comum rola as colunas");

    let mut s = setup();
    s.ui.set_prop(s.grid, "horizontal_wheel", "none").unwrap();
    wheel(&mut s, true);
    assert_eq!(left(&s), 0.0, "none: só o código rola as colunas");
    s.ui.set_prop(s.grid, "scroll_left", 300.0).unwrap();
    assert_eq!(left(&s), 300.0, "pelo código");
    s.ui.set_prop(s.grid, "scroll_left", 1.0e6).unwrap();
    assert!(left(&s) < 1.0e6, "ajustado ao que existe");
    assert_eq!(s.ui.get_prop(s.grid, "top_row").unwrap(), PropValue::Int(0));
}
