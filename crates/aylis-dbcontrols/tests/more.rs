//! DataGrid (ordenar, colunas fixas, filtro), DBImage e a máscara no editor de célula.

use aylis_controls::Form;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

struct NullCanvas;

impl Canvas for NullCanvas {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

struct App {
    ui: Ui,
    form: ComponentId,
    customers: ComponentId,
    ds_customers: ComponentId,
    orders: ComponentId,
    ds_orders: ComponentId,
}

fn app() -> App {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("dados", 800.0, 600.0));
    let customers = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).width(60.0), customers);
    ui.add(Field::new("nome", FieldType::String).width(150.0), customers);
    ui.add(Field::new("cidade", FieldType::String).width(150.0), customers);
    ui.add(Field::new("cpf", FieldType::String).mask(mask::CPF).width(150.0), customers);
    let ds_customers = ui.add(DataSource::new(customers), form);
    data_set(&mut ui, customers)
        .load_rows(vec![
            vec![Value::Int(1), "Maria".into(), "São Paulo".into(), Value::Null],
            vec![Value::Int(2), "João".into(), "Rio".into(), Value::Null],
            vec![Value::Int(3), "Ana".into(), "Recife".into(), Value::Null],
        ])
        .unwrap();
    let orders = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer), orders);
    ui.add(Field::new("cliente_id", FieldType::Integer), orders);
    ui.add(Field::new("status", FieldType::String), orders);
    ui.add(Field::new("obs", FieldType::Memo), orders);
    ui.add(Field::new("foto", FieldType::String), orders);
    let ds_orders = ui.add(DataSource::new(orders), form);
    data_set(&mut ui, orders)
        .load_rows(vec![vec![Value::Int(10), Value::Int(2), "aberto".into(), "primeira linha".into(), "assets/images/x.png".into()]])
        .unwrap();
    App { ui, form, customers, ds_customers, orders, ds_orders }
}

fn paint(ui: &mut Ui, form: ComponentId) {
    ui.layout();
    ui.paint(form, &mut NullCanvas);
}

#[test]
fn grid_sorts_by_title_and_filters_by_column() {
    let App { mut ui, form, customers, ds_customers, .. } = app();
    let grid = ui.add(DataGrid::new(ds_customers), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 300.0);
    ui.set_prop(grid, "filter_row", true).unwrap();
    ui.set_prop(grid, "fixed_columns", 1u32).unwrap();
    paint(&mut ui, form);
    // título "nome" (coluna 2: indicador 22 + id 60)
    ui.pointer_down(form, at(22.0 + 60.0 + 40.0, 16.0));
    ui.pointer_up(form, at(22.0 + 60.0 + 40.0, 16.0));
    let names = |ui: &Ui| (0..3).map(|i| view(ui, customers).value_at(i, "nome").unwrap().to_string()).collect::<Vec<_>>();
    assert_eq!(names(&ui), ["Ana", "João", "Maria"]);
    ui.pointer_down(form, at(22.0 + 60.0 + 40.0, 16.0));
    ui.pointer_up(form, at(22.0 + 60.0 + 40.0, 16.0));
    assert_eq!(names(&ui), ["Maria", "João", "Ana"], "de novo: decrescente");
    // linha de filtro da coluna cidade
    ui.pointer_down(form, at(22.0 + 60.0 + 150.0 + 40.0, 32.0 + 15.0));
    let editor = ui.focused(form).expect("campo de filtro aberto");
    ui.text_input(form, "re");
    ui.key_down(form, key(Key::Enter));
    assert!(!ui.contains(editor));
    assert_eq!(view(&ui, customers).record_count(), 1);
    assert_eq!(view(&ui, customers).value("nome").unwrap(), Value::from("Ana"));
    assert_eq!(ui.get::<ClientDataSet>(customers).unwrap().filter, "[cidade] like '%re%'");
}

#[test]
fn the_image_shows_the_field() {
    let App { mut ui, form, orders, ds_orders, .. } = app();
    let image = ui.add(DBImage::new(ds_orders, "foto"), form);
    data_set(&mut ui, orders).refresh_links();
    paint(&mut ui, form);
    assert_eq!(ui.get::<DBImage>(image).unwrap().base.image, "assets/images/x.png");
}

#[test]
fn the_cell_editor_applies_the_mask_while_typing() {
    let App { mut ui, form, customers, ds_customers, .. } = app();
    let cpf = ui.add(CellEditor::new(ds_customers, "cpf"), form);
    ui.control_mut(cpf).unwrap().set_rect(0.0, 0.0, 200.0, 32.0);
    data_set(&mut ui, customers).refresh_links();
    paint(&mut ui, form);
    ui.set_focus(cpf);
    for c in "12345678901".chars() {
        ui.text_input(form, &c.to_string());
    }
    assert_eq!(ui.get::<CellEditor>(cpf).unwrap().base.text, "123.456.789-01");
    data_set(&mut ui, customers).post().unwrap();
    assert_eq!(view(&ui, customers).value("cpf").unwrap(), Value::from("12345678901"));
}

#[test]
fn a_sensitive_field_makes_its_cell_editor_sensitive() {
    let App { mut ui, form, customers, ds_customers, .. } = app();
    let cpf = ui.add(CellEditor::new(ds_customers, "cpf"), form);
    let field = view(&ui, customers).field_id("cpf").unwrap();
    ui.set_prop(field, "sensitive", true).unwrap();
    data_set(&mut ui, customers).refresh_links();
    paint(&mut ui, form);
    // O gravador (EventRecorder) olha o `field_sensitive` além do `sensitive` do controle.
    assert_eq!(ui.get_prop(cpf, "field_sensitive").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(cpf, "sensitive").unwrap(), PropValue::Bool(false));
}
