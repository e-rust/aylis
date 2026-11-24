//! O `DataGrid`: o editor só de leitura, o `empty_text` e o cabeçalho vazio, o `row_at` e o
//! `value_at`, a dica do título e o `on_title_click` sem `sortable` e a barra de rolagem pelo
//! estilo `ScrollBar` do tema.

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
    grid: ComponentId,
}

/// Colunas: Código (22..102), Nome (102..327); linhas de 30 a partir de y = 32.
fn setup(rows: usize, columns: &[Column]) -> Setup {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código").width(80.0), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").width(225.0), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    for c in columns {
        ui.add(c.clone(), grid);
    }
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 332.0);
    ui.layout();
    let data = (0..rows).map(|i| vec![(i as i64 + 1).into(), format!("Cliente {}", i + 1).into()]).collect();
    data_set(&mut ui, cds).load_rows(data).unwrap();
    ui.set_focus(grid);
    let mut s = Setup { ui, form, cds, grid };
    paint(&mut s);
    s
}

fn paint(s: &mut Setup) -> Vec<Op> {
    let mut canvas = RecordingCanvas::new(None);
    s.ui.paint(s.form, &mut canvas);
    canvas.ops
}

fn grid(s: &Setup) -> &DataGrid {
    s.ui.get::<DataGrid>(s.grid).unwrap()
}

fn row_y(row: usize) -> f32 {
    32.0 + 15.0 + row as f32 * 30.0
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn press(s: &mut Setup, e: PointerEvent) {
    s.ui.pointer_move(s.form, e.clone());
    s.ui.pointer_down(s.form, e.clone());
    s.ui.pointer_up(s.form, e);
}

fn key(k: Key) -> KeyEvent {
    KeyEvent { key: k, ..Default::default() }
}

fn has_text(ops: &[Op], text: &str) -> bool {
    ops.iter().any(|op| matches!(op, Op::Text { text: t, .. } if t == text))
}

/// Item 40: numa grade só de leitura, Enter abre o editor só de leitura (o texto selecionável),
/// e nada entra em edição.
#[test]
fn a_read_only_cell_opens_a_read_only_editor() {
    let mut s = setup(5, &[]);
    s.ui.set_prop(s.grid, "read_only", true).unwrap();
    // Digitar não abre nada.
    s.ui.text_input(s.form, "x");
    assert!(grid(&s).editor().is_none());
    press(&mut s, at(200.0, row_y(2)));
    s.ui.key_down(s.form, key(Key::Enter));
    let editor = grid(&s).editor().expect("o editor só de leitura");
    let edit = s.ui.get::<CellEditor>(editor).unwrap();
    assert!(edit.base.read_only);
    assert_eq!(edit.base.text, "Cliente 3");
    s.ui.text_input(s.form, "zz");
    assert_eq!(s.ui.get::<CellEditor>(editor).unwrap().base.text, "Cliente 3");
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Browse);
    s.ui.key_down(s.form, key(Key::Escape));
    assert!(grid(&s).editor().is_none());
    // A coluna só de leitura numa grade editável: o mesmo.
    let mut s = setup(5, &[Column::new("id"), Column { read_only: true, ..Column::new("nome") }]);
    press(&mut s, at(200.0, row_y(1)));
    s.ui.key_down(s.form, key(Key::Enter));
    let editor = grid(&s).editor().expect("o editor da coluna só de leitura");
    assert!(s.ui.get::<CellEditor>(editor).unwrap().base.read_only);
    // A coluna editável continua editável.
    s.ui.key_down(s.form, key(Key::Escape));
    press(&mut s, at(50.0, row_y(1)));
    s.ui.key_down(s.form, key(Key::Enter));
    let editor = grid(&s).editor().unwrap();
    assert!(!s.ui.get::<CellEditor>(editor).unwrap().base.read_only);
}

/// Item 42: o texto da grade vazia e o cabeçalho vazio por `title: null` (o `""` continua o
/// nome do campo).
#[test]
fn empty_text_and_a_null_title() {
    let mut s = setup(0, &[Column::new("id").no_title(), Column::new("nome").title("")]);
    let titles: Vec<&str> = grid(&s).grid_columns().iter().map(|c| c.title.as_str()).collect();
    assert_eq!(titles, ["", "Nome"]);
    assert!(has_text(&paint(&mut s), "Sem registros"));
    s.ui.set_prop(s.grid, "empty_text", "Nenhuma tabela encontrada").unwrap();
    let ops = paint(&mut s);
    assert!(has_text(&ops, "Nenhuma tabela encontrada") && !has_text(&ops, "Sem registros"));
    // Pelo nome (o form, o protocolo): `null` e o texto.
    let id = s.ui.children(s.grid)[0];
    assert_eq!(s.ui.get_prop(id, "title").unwrap(), PropValue::Null);
    s.ui.set_prop(id, "title", "Cód.").unwrap();
    assert_eq!(grid(&s).grid_columns()[0].title, "Cód.");
    s.ui.set_prop(id, "title", PropValue::Null).unwrap();
    assert_eq!(grid(&s).grid_columns()[0].title, "");
    // Na lista `columns`.
    let list = ColumnSpec::parse_list(r#"[{"field": "id", "title": null}, {"field": "nome", "title": "", "hint": "O nome"}]"#);
    assert_eq!(list[0].title, None);
    assert_eq!(list[1].title.as_deref(), Some(""));
    assert_eq!(ColumnSpec::list_to_json(&list), r#"[{"field":"id","title":null},{"field":"nome","hint":"O nome"}]"#);
    let mut s = setup(0, &[]);
    s.ui.set_prop(s.grid, "columns", r#"[{"field": "id", "title": null}, {"field": "nome"}]"#).unwrap();
    paint(&mut s);
    let titles: Vec<&str> = grid(&s).grid_columns().iter().map(|c| c.title.as_str()).collect();
    assert_eq!(titles, ["", "Nome"]);
}

/// Item 43: a linha sob uma altura e o valor de um campo nela, sem mudar o registro atual.
#[test]
fn row_at_and_value_at() {
    let mut s = setup(20, &[]);
    let g = grid(&s);
    assert_eq!(g.row_at(10.0), None, "o cabeçalho");
    assert_eq!(g.row_at(row_y(0)), Some(0));
    assert_eq!(g.row_at(row_y(3)), Some(3));
    assert_eq!(g.value_at(&s.ui, 3, "nome"), Some(Value::String("Cliente 4".into())));
    assert_eq!(g.value_at(&s.ui, 99, "nome"), None);
    assert_eq!(g.value_at(&s.ui, 3, "nada"), None);
    // Rolada, a linha conta do começo dos registros.
    s.ui.set_prop(s.grid, "top_row", 5).unwrap();
    assert_eq!(grid(&s).row_at(row_y(1)), Some(6));
    // Pelas outras linguagens (`invoke`).
    assert_eq!(s.ui.invoke(s.grid, "row_at", &[PropValue::Float(row_y(2) as f64)]).unwrap(), PropValue::Int(7));
    assert_eq!(s.ui.invoke(s.grid, "row_at", &[PropValue::Float(5.0)]).unwrap(), PropValue::Null);
    assert_eq!(s.ui.invoke(s.grid, "value_at", &[PropValue::Int(7), "nome".into()]).unwrap(), PropValue::String("Cliente 8".into()));
    assert_eq!(s.ui.invoke(s.grid, "value_at", &[PropValue::Int(7), "id".into()]).unwrap(), PropValue::String("8".into()));
    assert_eq!(data_set(&mut s.ui, s.cds).rec_no(), Some(0), "o registro atual não muda");
    // Poucas linhas: abaixo da última, nenhuma.
    let s = setup(2, &[]);
    assert_eq!(grid(&s).row_at(row_y(4)), None);
}

/// Item 45: a dica do título e o `on_title_click` também sem `sortable`.
#[test]
fn title_hint_and_title_click_without_sortable() {
    let mut s = setup(5, &[Column::new("id").hint("O código do cliente"), Column::new("nome")]);
    s.ui.set_prop(s.grid, "hint", "A grade").unwrap();
    s.ui.pointer_move(s.form, at(50.0, 16.0));
    assert_eq!(s.ui.control(s.grid).unwrap().hint(), "O código do cliente");
    s.ui.pointer_move(s.form, at(200.0, 16.0));
    assert_eq!(s.ui.control(s.grid).unwrap().hint(), "A grade", "o título sem dica: a da grade");
    s.ui.pointer_move(s.form, at(50.0, 16.0));
    s.ui.pointer_move(s.form, at(50.0, row_y(1)));
    assert_eq!(s.ui.control(s.grid).unwrap().hint(), "A grade");
    // O clique no título sem `sortable`: avisa e não ordena.
    s.ui.set_prop(s.grid, "sortable", false).unwrap();
    let clicks = Rc::new(RefCell::new(Vec::new()));
    let log = clicks.clone();
    s.ui.on(s.grid, "on_title_click", move |_, _, args| log.borrow_mut().push(<String as EventValue>::from_args(args).unwrap()));
    press(&mut s, at(200.0, 16.0));
    assert_eq!(*clicks.borrow(), ["nome"]);
    let index = s.ui.get::<ClientDataSet>(s.cds).unwrap().index_field_names.clone();
    assert!(index.is_empty(), "não ordenou");
}

/// Item 59: a barra da grade pelo estilo `ScrollBar` do tema (a largura e as cores); sem ele,
/// como era.
#[test]
fn the_scroll_bar_follows_the_theme() {
    let mut s = setup(50, &[]);
    let thumb = |ops: &[Op]| {
        ops.iter()
            .filter_map(|op| match op {
                Op::FillRect { rect, color, .. } if rect.x > 580.0 && rect.height > 20.0 => Some((*rect, *color)),
                _ => None,
            })
            .last()
    };
    let (before, _) = thumb(&paint(&mut s)).expect("a barra");
    assert_eq!(before.width, 8.0, "10 de largura, menos a margem de 1 de cada lado");
    let red = Color::rgb(200, 0, 0);
    s.ui.theme_mut().set_style("ScrollBar", Style::new(StyleProps { width: Some(6.0), thumb: Some(ColorRef::Value(red)), ..Default::default() }));
    DataGrid::refresh_rows(&mut s.ui, s.grid);
    let (after, color) = thumb(&paint(&mut s)).unwrap();
    assert_eq!(after.width, 4.0);
    assert_eq!(color, red);
    assert_eq!(after.x, 600.0 - 6.0 - 2.0 + 1.0);
}

/// Item 39: o cabeçalho, as linhas (atual, selecionada, zebra) e a moldura da célula atual pelos
/// sub-estilos do tema, e `frame: false` sem a caixa própria. Sem eles, o desenho de sempre.
#[test]
fn the_grid_parts_follow_the_theme_and_frame_can_be_off() {
    let mut s = setup(6, &[]);
    let surface = s.ui.theme().color("surface");
    let primary = s.ui.theme().color("primary");
    let before = paint(&mut s);
    let fills = |ops: &[Op], color: Color| ops.iter().filter(|op| matches!(op, Op::FillRect { color: c, .. } if *c == color)).count();
    // A caixa própria (fundo e moldura do tamanho da grade).
    let frame = |ops: &[Op]| ops.iter().any(|op| matches!(op, Op::StrokeRect { rect, .. } if rect.width >= 599.0));
    assert!(frame(&before));
    assert!(before.iter().any(|op| matches!(op, Op::StrokeRect { color, width, .. } if *color == primary && *width == 2.0)), "a moldura de sempre");
    let (header, stripe, current, cursor) = (Color::rgb(10, 20, 30), Color::rgb(1, 2, 3), Color::rgb(40, 50, 60), Color::rgb(200, 100, 0));
    let theme = s.ui.theme_mut();
    theme.set_style("DataGrid.header", Style::new(StyleProps { fill: Some(ColorRef::Value(header)), ..Default::default() }));
    theme.set_style("DataGrid.stripe", Style::new(StyleProps { fill: Some(ColorRef::Value(stripe)), ..Default::default() }));
    theme.set_style("DataGrid.row", Style::new(StyleProps::default()).focused(StyleProps { fill: Some(ColorRef::Value(current)), ..Default::default() }));
    theme.set_style("DataGrid.cursor", Style::new(StyleProps { border_color: Some(ColorRef::Value(cursor)), border_width: Some(1.0), ..Default::default() }));
    let ops = paint(&mut s);
    assert_eq!(fills(&ops, header), 1, "o fundo do cabeçalho");
    assert_eq!(fills(&ops, stripe), 3, "as linhas ímpares");
    assert_eq!(fills(&ops, current), 1, "a linha atual");
    assert!(ops.iter().any(|op| matches!(op, Op::StrokeRect { color, width, .. } if *color == cursor && *width == 1.0)), "a moldura da célula atual");
    // O `cursor_color` da grade vale mais que o tema.
    s.ui.set_prop(s.grid, "cursor_color", Color::rgb(0, 0, 255)).unwrap();
    assert!(paint(&mut s).iter().any(|op| matches!(op, Op::StrokeRect { color, .. } if *color == Color::rgb(0, 0, 255))));
    // Sem a caixa própria: nem o fundo nem a moldura de fora (o card em volta faz o papel).
    s.ui.set_prop(s.grid, "frame", false).unwrap();
    let ops = paint(&mut s);
    assert!(!frame(&ops));
    assert!(!ops.iter().any(|op| matches!(op, Op::FillRect { rect, color, .. } if *color == surface && rect.width >= 599.0 && rect.height >= 330.0)));
}

/// A grade custa o mesmo com 2 mil ou 1 milhão de linhas, também com as partes do tema e a
/// barra do `ScrollBar`: as mesmas linhas preparadas e as mesmas operações de pintura, rolada
/// para o meio.
#[test]
fn the_cost_does_not_depend_on_the_row_count() {
    let mut measured = Vec::new();
    for rows in [2_000, 1_000_000] {
        let mut s = setup(rows, &[]);
        let theme = s.ui.theme_mut();
        theme.set_style("DataGrid.stripe", Style::new(StyleProps { fill: Some(ColorRef::Value(Color::rgb(1, 2, 3))), ..Default::default() }));
        theme.set_style("ScrollBar", Style::new(StyleProps { width: Some(6.0), ..Default::default() }));
        s.ui.set_prop(s.grid, "top_row", (rows / 2) as i64).unwrap();
        paint(&mut s);
        let start = std::time::Instant::now();
        let ops = paint(&mut s).len();
        measured.push((grid(&s).cached_rows(), ops, start.elapsed()));
    }
    assert_eq!((measured[0].0, measured[0].1), (measured[1].0, measured[1].1), "{measured:?}");
    println!("pintura: 2 mil {:?}, 1 milhão {:?}", measured[0].2, measured[1].2);
}
