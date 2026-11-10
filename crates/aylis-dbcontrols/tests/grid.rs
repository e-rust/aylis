use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

/// Canvas que só conta as operações de desenho.
#[derive(Default)]
struct Counter {
    texts: usize,
    ops: usize,
}

impl Canvas for Counter {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {
        self.ops += 1;
    }
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {
        self.ops += 1;
    }
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {
        self.ops += 1;
    }
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {
        self.ops += 1;
    }
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {
        self.ops += 1;
    }
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {
        self.texts += 1;
        self.ops += 1;
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

struct Setup {
    ui: Ui,
    form: ComponentId,
    cds: ComponentId,
    grid: ComponentId,
}

/// Grid 600x(32 de cabeçalho + 10 linhas de 30) sobre `rows` registros.
fn setup(rows: usize) -> Setup {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(30).required(), cds);
    ui.add(Field::new("valor", FieldType::Currency).label("Valor").format("#,##0.00"), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 32.0 + 300.0);
    ui.layout();
    let data = (0..rows).map(|i| vec![(i as i64 + 1).into(), format!("Cliente {}", i + 1).into(), Decimal::new(i as i128 * 150, 2).into()]).collect();
    data_set(&mut ui, cds).load_rows(data).unwrap();
    ui.set_focus(grid);
    Setup { ui, form, cds, grid }
}

fn grid(s: &Setup) -> &DataGrid {
    s.ui.get::<DataGrid>(s.grid).unwrap()
}

fn rec_no(s: &mut Setup) -> Option<usize> {
    data_set(&mut s.ui, s.cds).rec_no()
}

fn key(k: Key) -> KeyEvent {
    KeyEvent { key: k, ..Default::default() }
}

fn ctrl(k: Key) -> KeyEvent {
    KeyEvent { key: k, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() }
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn click(s: &mut Setup, x: f32, y: f32) {
    s.ui.pointer_move(s.form, at(x, y));
    s.ui.pointer_down(s.form, at(x, y));
    s.ui.pointer_up(s.form, at(x, y));
}

#[test]
fn columns_come_from_the_fields() {
    let s = setup(5);
    let cols = grid(&s).grid_columns();
    assert_eq!(cols.iter().map(|c| c.title.as_str()).collect::<Vec<_>>(), ["Código", "Nome", "Valor"]);
    assert_eq!(cols[2].align, TextAlign::Trailing);
    assert!(cols[1].width > cols[0].width);
}

#[test]
fn only_visible_rows_are_prepared_and_painted() {
    let mut s = setup(100_000);
    assert_eq!(grid(&s).cached_rows(), 11, "10 linhas inteiras + 1 parcial");
    let mut counter = Counter::default();
    let started = std::time::Instant::now();
    s.ui.paint(s.form, &mut counter);
    let elapsed = started.elapsed();
    assert!(counter.texts <= 3 + 11 * 3 + 2, "textos pintados: {}", counter.texts);
    assert!(elapsed.as_millis() < 200, "pintura com 100 mil registros levou {elapsed:?}");

    // ir ao último registro custa o mesmo
    s.ui.key_down(s.form, ctrl(Key::End));
    assert_eq!(rec_no(&mut s), Some(99_999));
    assert_eq!(grid(&s).top_row(), 99_990);
    assert_eq!(grid(&s).cached_rows(), 10);
}

#[test]
fn keyboard_moves_the_cursor_and_keeps_it_visible() {
    let mut s = setup(100);
    s.ui.key_down(s.form, key(Key::Down));
    s.ui.key_down(s.form, key(Key::Down));
    assert_eq!(rec_no(&mut s), Some(2));
    s.ui.key_down(s.form, key(Key::PageDown));
    assert_eq!(rec_no(&mut s), Some(12));
    assert_eq!(grid(&s).top_row(), 3, "rolou para mostrar o registro atual");
    s.ui.key_down(s.form, ctrl(Key::Home));
    assert_eq!(rec_no(&mut s), Some(0));
    assert_eq!(grid(&s).top_row(), 0);

    s.ui.key_down(s.form, key(Key::Right));
    s.ui.key_down(s.form, key(Key::Right));
    s.ui.key_down(s.form, key(Key::Right));
    assert_eq!(grid(&s).selected_column(), 2, "não passa da última coluna");
}

#[test]
fn click_selects_row_and_column_and_wheel_scrolls() {
    let mut s = setup(100);
    // linha 4 (y = 32 + 4*30 + 10), coluna "Nome" (x = 22 + 80 + 10)
    click(&mut s, 112.0, 162.0);
    assert_eq!(rec_no(&mut s), Some(4));
    assert_eq!(grid(&s).selected_column(), 1);

    s.ui.wheel(s.form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -40.0), ..Default::default() });
    assert_eq!(grid(&s).top_row(), 3, "uma volta da roda = 3 linhas");
    s.ui.wheel(s.form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, 4000.0), ..Default::default() });
    assert_eq!(grid(&s).top_row(), 0);
}

#[test]
fn rows_are_fetched_on_demand_in_packets() {
    let mut s = setup(0);
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let total = 1000usize;
    let mut ds = data_set(&mut s.ui, s.cds);
    ds.on_fetch(move |ui, cds, loaded| {
        c.set(c.get() + 1);
        let end = (loaded + 50).min(total);
        let rows = (loaded..end).map(|i| vec![(i as i64).into(), format!("Item {i}").into(), Value::Null]).collect();
        data_set(ui, cds).append_rows(rows, end < total).unwrap();
    });
    ds.set_has_more(true).unwrap();
    ds.fetch_more().unwrap();
    assert_eq!(ds.record_count(), 50);
    assert_eq!(calls.get(), 1);

    // rolar até perto do fim do que foi carregado busca mais
    for _ in 0..40 {
        s.ui.key_down(s.form, key(Key::PageDown));
    }
    let loaded = data_set(&mut s.ui, s.cds).record_count();
    assert!(loaded > 300, "carregados: {loaded}");
    assert!(calls.get() >= 6);
    s.ui.key_down(s.form, ctrl(Key::End));
    for _ in 0..100 {
        s.ui.key_down(s.form, key(Key::PageDown));
    }
    assert_eq!(data_set(&mut s.ui, s.cds).record_count(), total);
    assert!(!aylis_data::view(&s.ui, s.cds).has_more());
}

#[test]
fn enter_edits_the_cell_and_writes_on_enter() {
    let mut s = setup(10);
    s.ui.key_down(s.form, key(Key::Right)); // coluna "Nome"
    s.ui.key_down(s.form, key(Key::Enter));
    let editor = grid(&s).editor().expect("editor aberto");
    assert_eq!(s.ui.focused(s.form), Some(editor));
    assert_eq!(s.ui.get::<CellEditor>(editor).unwrap().base.text, "Cliente 1");

    s.ui.text_input(s.form, "Maria"); // substitui a seleção
    s.ui.key_down(s.form, key(Key::Enter));
    assert!(grid(&s).editor().is_none(), "Enter fecha o editor");
    assert!(!s.ui.contains(editor));
    assert_eq!(s.ui.focused(s.form), Some(s.grid), "o foco volta ao grid");
    let mut ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.state(), DataSetState::Edit);
    assert_eq!(ds.text("nome").unwrap(), "Maria");
    ds.post().unwrap();
    assert_eq!(ds.change_count(), 1);
}

#[test]
fn typing_starts_the_editor_with_that_key() {
    let mut s = setup(10);
    s.ui.key_down(s.form, key(Key::Right));
    s.ui.text_input(s.form, "J");
    s.ui.text_input(s.form, "o");
    let editor = grid(&s).editor().unwrap();
    assert_eq!(s.ui.get::<CellEditor>(editor).unwrap().base.text, "Jo", "a primeira tecla não é perdida nem selecionada");
}

#[test]
fn escape_discards_and_tab_moves_to_the_next_column() {
    let mut s = setup(10);
    s.ui.key_down(s.form, key(Key::Right));
    s.ui.text_input(s.form, "X");
    s.ui.key_down(s.form, key(Key::Escape)); // CellEditor desfaz o texto
    s.ui.key_down(s.form, key(Key::Escape)); // o grid fecha e cancela
    assert!(grid(&s).editor().is_none());
    let ds = data_set(&mut s.ui, s.cds);
    assert_eq!(ds.state(), DataSetState::Browse);
    assert_eq!(ds.text("nome").unwrap(), "Cliente 1");

    s.ui.key_down(s.form, key(Key::Enter));
    s.ui.key_down(s.form, key(Key::Tab));
    assert_eq!(grid(&s).selected_column(), 2);
    let editor = grid(&s).editor().unwrap();
    assert_eq!(s.ui.get::<CellEditor>(editor).unwrap().data_field, "valor");
}

#[test]
fn invalid_values_keep_the_editor_open() {
    let mut s = setup(10);
    s.ui.key_down(s.form, key(Key::End)); // coluna "Valor"
    s.ui.key_down(s.form, key(Key::Enter));
    s.ui.text_input(s.form, "abc");
    s.ui.key_down(s.form, key(Key::Enter));
    let editor = grid(&s).editor().expect("continua aberto");
    assert!(s.ui.get::<CellEditor>(editor).unwrap().error().is_some());
}

#[test]
fn insert_shows_the_new_row_and_header_drag_resizes() {
    let mut s = setup(3);
    s.ui.key_down(s.form, key(Key::Down));
    s.ui.key_down(s.form, key(Key::Insert));
    let v = aylis_data::view(&s.ui, s.cds);
    assert_eq!(v.state(), DataSetState::Insert);
    assert_eq!(v.visible_count(), 4);
    assert_eq!(v.visible_current(), Some(1));
    assert_eq!(grid(&s).cached_rows(), 4);

    // arrasta a borda direita da coluna "Código" (x = 22 + 80) no cabeçalho
    let before = grid(&s).grid_columns()[0].width;
    s.ui.pointer_move(s.form, at(102.0, 16.0));
    assert_eq!(s.ui.cursor(s.form), CursorShape::ResizeHorizontal);
    s.ui.pointer_down(s.form, at(102.0, 16.0));
    s.ui.pointer_move(s.form, at(142.0, 16.0));
    s.ui.pointer_up(s.form, at(142.0, 16.0));
    assert_eq!(grid(&s).grid_columns()[0].width, before + 40.0);
}

#[test]
fn columns_property_chooses_and_orders() {
    let mut s = setup(3);
    s.ui.set_prop(s.grid, "columns", "valor;nome").unwrap();
    data_set(&mut s.ui, s.cds).refresh_links();
    let titles: Vec<String> = grid(&s).grid_columns().iter().map(|c| c.title.clone()).collect();
    assert_eq!(titles, ["Valor", "Nome"]);
}

/// Canvas que guarda os textos e as cores de fundo pintados.
#[derive(Default)]
struct Recorder {
    texts: Vec<(String, Color, bool)>,
    fills: Vec<Color>,
    custom: usize,
}

impl Canvas for Recorder {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, b: &Brush) {
        if let Brush::Solid(c) = b {
            self.fills.push(*c);
        }
    }
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {
        self.custom += 1;
    }
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, style: &TextStyle) {
        self.texts.push((text.to_string(), style.color, style.font.weight == FontWeight::Bold));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

#[test]
fn custom_draw_gets_the_row_data_and_styles_rows_and_cells() {
    let mut s = setup(5);
    let red = Color::rgb(0xFF, 0xEB, 0xEE);
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    {
        let seen = seen.clone();
        DataGrid::on_draw_cell(&mut s.ui, s.grid, move |cell| {
            if cell.column.is_none() {
                seen.borrow_mut().push((cell.row, cell.json()));
            }
            let id = cell.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            match cell.column.as_deref() {
                // A linha 2 inteira em vermelho claro.
                None if id == 2 => CellStyle { background: Some(red), ..Default::default() },
                // O nome em negrito e o valor trocado na linha 3.
                Some("nome") => CellStyle { bold: true, ..Default::default() },
                Some("valor") if id == 3 => CellStyle { text: Some("três".into()), ..Default::default() },
                // O código da linha 4 pintado à mão (uma bolinha).
                Some("id") if id == 4 => CellStyle {
                    paint: Some(std::rc::Rc::new(|c: &mut dyn Canvas, r: RectF| c.fill_ellipse(r.inflate(-8.0), &Brush::Solid(Color::BLACK)))),
                    ..Default::default()
                },
                _ => CellStyle::default(),
            }
        });
    }
    let rows = seen.borrow().clone();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0].0, 0);
    assert_eq!(rows[1].1, r#"{"id":2,"nome":"Cliente 2","valor":"1.50"}"#);

    let mut rec = Recorder::default();
    s.ui.paint(s.form, &mut rec);
    assert!(rec.fills.contains(&red), "fundo da linha 2");
    assert!(rec.texts.iter().any(|(t, _, bold)| t == "Cliente 1" && *bold), "nome em negrito");
    assert!(rec.texts.iter().any(|(t, _, _)| t == "três"), "texto trocado");
    assert!(!rec.texts.iter().any(|(t, _, _)| t == "4"), "o código 4 foi pintado à mão");
    assert_eq!(rec.custom, 1);
}

#[test]
fn the_grid_exposes_headers_rows_and_cells_for_automation() {
    use aylis_core::automation::{ItemAction, ItemRole};
    let mut s = setup(5);
    s.ui.paint(s.form, &mut Counter::default());
    let items = s.ui.access_items(s.grid);
    let headers: Vec<&str> = items.iter().filter(|i| i.role == ItemRole::ColumnHeader).map(|i| i.name.as_str()).collect();
    assert_eq!(headers, ["Código", "Nome", "Valor"]);
    let rows: Vec<usize> = items.iter().enumerate().filter(|(_, i)| i.role == ItemRole::Row).map(|(n, _)| n).collect();
    assert_eq!(rows.len(), 5);
    // As células ficam dentro da linha, com linha, coluna e o texto formatado.
    let third_row = rows[2];
    let cells: Vec<_> = items.iter().filter(|i| i.parent == Some(third_row)).collect();
    assert_eq!(cells.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["3", "Cliente 3", "3,00"]);
    assert_eq!((cells[1].row, cells[1].column), (Some(2), Some(1)));
    assert!(items[rows[0]].selected, "a linha atual");

    // Clicar na célula "Cliente 4" leva o registro e a coluna até ela.
    assert!(s.ui.click_item(s.grid, "Cliente 4"));
    assert_eq!(rec_no(&mut s), Some(3));
    s.ui.paint(s.form, &mut Counter::default());
    let items = s.ui.access_items(s.grid);
    assert!(items.iter().any(|i| i.role == ItemRole::Cell && i.name == "Cliente 4" && i.selected));
    // Selecionar uma linha também move o registro.
    let row = items.iter().position(|i| i.role == ItemRole::Row && i.name.starts_with("2 ·")).unwrap();
    assert!(s.ui.access_action(s.grid, row, ItemAction::Select));
    assert_eq!(rec_no(&mut s), Some(1));
}

/// O meio do título da coluna `i` (o cabeçalho começa depois do indicador, de 22 pontos).
fn title_x(s: &Setup, i: usize) -> f32 {
    let cols = grid(s).grid_columns();
    22.0 + cols[..i].iter().map(|c| c.width).sum::<f32>() + cols[i].width / 2.0
}

#[test]
fn dragging_a_title_moves_the_column() {
    let mut s = setup(5);
    let moved = Rc::new(Cell::new(false));
    let flag = moved.clone();
    s.ui.on(s.grid, "on_column_moved", move |_, _, _| flag.set(true));
    // Código vai para depois de Valor.
    let (from, to) = (title_x(&s, 0), title_x(&s, 2) + 30.0);
    s.ui.pointer_move(s.form, at(from, 16.0));
    s.ui.pointer_down(s.form, at(from, 16.0));
    for step in 1..=10 {
        s.ui.pointer_move(s.form, at(from + (to - from) * step as f32 / 10.0, 16.0));
    }
    s.ui.pointer_up(s.form, at(to, 16.0));
    let titles: Vec<String> = grid(&s).grid_columns().iter().map(|c| c.title.clone()).collect();
    assert_eq!(titles, ["Nome", "Valor", "Código"]);
    assert!(moved.get(), "on_column_moved");
    // Arrastar não ordena.
    assert_eq!(s.ui.get::<ClientDataSet>(s.cds).unwrap().effective_index().unwrap(), "");
    // Por código também.
    DataGrid::move_column_to(&mut s.ui, s.grid, 2, 0);
    let titles: Vec<String> = grid(&s).grid_columns().iter().map(|c| c.title.clone()).collect();
    assert_eq!(titles, ["Código", "Nome", "Valor"]);
}

#[test]
fn clicking_a_title_still_sorts() {
    let mut s = setup(5);
    let x = title_x(&s, 1);
    click(&mut s, x, 16.0);
    let titles: Vec<String> = grid(&s).grid_columns().iter().map(|c| c.title.clone()).collect();
    assert_eq!(titles, ["Código", "Nome", "Valor"], "a ordem das colunas não muda");
    assert!(s.ui.get::<ClientDataSet>(s.cds).unwrap().index_field_names.to_lowercase().contains("nome"));
}
