//! DataGrid: caixa de marcar em campos lógicos, várias linhas selecionadas, totais no rodapé
//! e busca incremental.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::platform::{ClipboardService, LocaleInfo};
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

struct Setup {
    ui: Ui,
    form: ComponentId,
    cds: ComponentId,
    grid: ComponentId,
}

/// Colunas: Código (22..102), Nome (102..327), Valor (327..447), Ativo (447..517).
/// Linhas de 30 a partir de y = 32.
fn setup(rows: usize) -> Setup {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(30), cds);
    ui.add(Field::new("valor", FieldType::Currency).label("Valor").format("#,##0.00"), cds);
    ui.add(Field::new("ativo", FieldType::Boolean).label("Ativo"), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 32.0 + 300.0);
    ui.layout();
    let data = (0..rows)
        .map(|i| vec![(i as i64 + 1).into(), format!("Cliente {}", i + 1).into(), Decimal::new(i as i128 * 150, 2).into(), (i % 2 == 0).into()])
        .collect();
    data_set(&mut ui, cds).load_rows(data).unwrap();
    ui.set_focus(grid);
    ui.paint(form, &mut NullCanvas);
    Setup { ui, form, cds, grid }
}

fn grid(s: &Setup) -> &DataGrid {
    s.ui.get::<DataGrid>(s.grid).unwrap()
}

fn rec_no(s: &mut Setup) -> Option<usize> {
    data_set(&mut s.ui, s.cds).rec_no()
}

fn row_y(row: usize) -> f32 {
    32.0 + 15.0 + row as f32 * 30.0
}

fn press(s: &mut Setup, e: PointerEvent) {
    s.ui.pointer_move(s.form, e.clone());
    s.ui.pointer_down(s.form, e.clone());
    s.ui.pointer_up(s.form, e);
}

fn click(s: &mut Setup, x: f32, row: usize) {
    press(s, PointerEvent { position: PointF::new(x, row_y(row)), ..Default::default() });
}

fn click_with(s: &mut Setup, x: f32, row: usize, ctrl: bool, shift: bool) {
    press(s, PointerEvent { position: PointF::new(x, row_y(row)), modifiers: Modifiers { ctrl, shift, ..Default::default() }, ..Default::default() });
}

fn key(k: Key) -> KeyEvent {
    KeyEvent { key: k, ..Default::default() }
}

fn active(s: &mut Setup) -> Option<bool> {
    data_set(&mut s.ui, s.cds).value("ativo").unwrap().as_bool()
}

#[test]
fn boolean_fields_are_check_boxes_toggled_by_click_and_space() {
    let mut s = setup(10);
    let cols = grid(&s).grid_columns();
    assert!(cols[3].check && !cols[1].check, "o campo lógico vira caixa");
    // Clique na caixa da linha 3: vai até ela e alterna (era falso).
    click(&mut s, 482.0, 3);
    assert_eq!(rec_no(&mut s), Some(3));
    assert_eq!(active(&mut s), Some(true));
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Edit, "fica em edição, como no editor de célula");
    data_set(&mut s.ui, s.cds).post().unwrap();
    // Espaço com a coluna lógica escolhida alterna de novo.
    assert_eq!(grid(&s).selected_column(), 3);
    s.ui.key_down(s.form, key(Key::Space));
    s.ui.text_input(s.form, " ");
    assert_eq!(active(&mut s), Some(false));
    assert!(grid(&s).editor().is_none(), "o espaço não abre o editor");
    // Clique fora da caixa (na mesma célula) só escolhe.
    data_set(&mut s.ui, s.cds).post().unwrap();
    click(&mut s, 452.0, 3);
    assert_eq!(active(&mut s), Some(false));
    // Somente leitura: não alterna.
    s.ui.set_prop(s.grid, "read_only", true).unwrap();
    click(&mut s, 482.0, 4);
    assert_eq!(active(&mut s), Some(true));
    assert_eq!(data_set(&mut s.ui, s.cds).state(), DataSetState::Browse);
}

#[test]
fn several_rows_are_selected_with_ctrl_and_shift_and_survive_sorting() {
    let mut s = setup(10);
    s.ui.set_prop(s.grid, "multi_select", true).unwrap();
    let changes = Rc::new(RefCell::new(Vec::new()));
    let log = changes.clone();
    s.ui.on(s.grid, "on_selection_change", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            log.borrow_mut().push(t.clone());
        }
    });
    click(&mut s, 200.0, 1);
    assert_eq!(grid(&s).selected_ids().len(), 1);
    click_with(&mut s, 200.0, 3, true, false);
    let ids = |s: &Setup| -> Vec<u64> { grid(s).selected_ids() };
    let id_of = |s: &Setup, row: usize| view(&s.ui, s.cds).visible_row(row).unwrap().id.unwrap();
    assert_eq!(ids(&s), vec![id_of(&s, 1), id_of(&s, 3)]);
    // Shift: da âncora (linha 3) até a 5, trocando a seleção.
    click_with(&mut s, 200.0, 5, false, true);
    assert_eq!(ids(&s), vec![id_of(&s, 3), id_of(&s, 4), id_of(&s, 5)]);
    // Ctrl+clique tira.
    click_with(&mut s, 200.0, 4, true, false);
    assert_eq!(ids(&s), vec![id_of(&s, 3), id_of(&s, 5)]);
    // Shift+seta para baixo estende a partir da âncora (4, agora).
    s.ui.key_down(s.form, KeyEvent { key: Key::Down, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() });
    assert_eq!(rec_no(&mut s), Some(5));
    assert_eq!(ids(&s), vec![id_of(&s, 4), id_of(&s, 5)]);
    // A seleção segue os registros na outra ordem.
    let chosen = ids(&s);
    s.ui.pointer_down(s.form, PointerEvent { position: PointF::new(200.0, 16.0), ..Default::default() });
    s.ui.pointer_down(s.form, PointerEvent { position: PointF::new(200.0, 16.0), ..Default::default() }); // nome desc
    assert_eq!(ids(&s), chosen);
    assert_eq!(data_set(&mut s.ui, s.cds).text("nome").unwrap(), "Cliente 6", "o cursor fica no registro");
    // Ctrl+A: todas.
    s.ui.key_down(s.form, KeyEvent { key: Key::Char('a'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    assert_eq!(ids(&s).len(), 10);
    assert_eq!(changes.borrow().last().map(String::as_str), Some("10"));
    // Ir até cada selecionado pelo id.
    let mut ds = data_set(&mut s.ui, s.cds);
    assert!(ds.goto_id(chosen[0]).unwrap());
    assert_eq!(ds.text("nome").unwrap(), "Cliente 5");
    // Desligar a seleção múltipla limpa.
    s.ui.set_prop(s.grid, "multi_select", false).unwrap();
    assert!(ids(&s).is_empty());
}

#[test]
fn the_footer_shows_totals_in_the_field_format_and_follows_the_data() {
    let mut s = setup(10);
    s.ui.set_prop(s.grid, "footer", "valor=sum;id=count;nome=max").unwrap();
    s.ui.paint(s.form, &mut NullCanvas);
    // 0 + 1,50 + ... + 13,50 = 67,50
    assert_eq!(grid(&s).footer_text("valor"), Some("67,50"));
    assert_eq!(grid(&s).footer_text("id"), Some("10"));
    assert_eq!(grid(&s).footer_text("nome"), None, "max de texto não tem número");
    // Com o rodapé, cabe uma linha a menos.
    assert_eq!(grid(&s).cached_rows(), 10, "9 visíveis + a parcial");
    // Alterar um valor e gravar atualiza.
    {
        let mut ds = data_set(&mut s.ui, s.cds);
        ds.edit().unwrap();
        ds.set_value("valor", Decimal::new(10000, 2)).unwrap();
        ds.post().unwrap();
    }
    assert_eq!(grid(&s).footer_text("valor"), Some("167,50"));
    // Com filtro, só os que passam.
    data_set(&mut s.ui, s.cds).set_filter("id <= 3").unwrap();
    assert_eq!(grid(&s).footer_text("id"), Some("3"));
    assert_eq!(grid(&s).footer_text("valor"), Some("104,50"), "100,00 + 1,50 + 3,00");
    s.ui.set_prop(s.grid, "footer", "valor=avg").unwrap();
    s.ui.paint(s.form, &mut NullCanvas);
    assert_eq!(grid(&s).footer_text("valor"), Some("34,83"));
}

#[test]
fn incremental_search_finds_by_prefix_in_the_current_column() {
    let mut s = setup(12);
    s.ui.set_prop(s.grid, "incremental_search", true).unwrap();
    s.ui.key_down(s.form, key(Key::Right)); // coluna Nome
    s.ui.set_time(10.0);
    for c in ["c", "l", "i", "e", "n", "t", "e", " ", "1", "1"] {
        s.ui.text_input(s.form, c);
    }
    assert!(grid(&s).editor().is_none(), "digitar busca, não edita");
    assert_eq!(grid(&s).search_text(), "cliente 11");
    assert_eq!(rec_no(&mut s), Some(10));
    // Backspace volta para "cliente 1": o primeiro desde o começo.
    s.ui.key_down(s.form, key(Key::Backspace));
    assert_eq!(rec_no(&mut s), Some(0));
    // Depois da pausa, a busca recomeça (a partir do atual).
    s.ui.set_time(12.0);
    s.ui.text_input(s.form, "C");
    assert_eq!(grid(&s).search_text(), "C");
    assert_eq!(rec_no(&mut s), Some(0));
    // Nada encontrado: fica onde está.
    s.ui.text_input(s.form, "x");
    assert_eq!(rec_no(&mut s), Some(0));
    // Enter ainda edita.
    s.ui.key_down(s.form, key(Key::Escape));
    s.ui.key_down(s.form, key(Key::Enter));
    assert!(grid(&s).editor().is_some());
}

struct FakeClipboard(Rc<RefCell<String>>);

impl ClipboardService for FakeClipboard {
    fn get_text(&mut self) -> Option<String> {
        Some(self.0.borrow().clone())
    }
    fn set_text(&mut self, text: &str) {
        *self.0.borrow_mut() = text.to_string();
    }
}

#[test]
fn ctrl_c_copies_the_current_cell_text() {
    let mut s = setup(5);
    let copied = Rc::new(RefCell::new(String::new()));
    s.ui.services_mut().clipboard = Some(Box::new(FakeClipboard(copied.clone())));
    click(&mut s, 150.0, 2);
    s.ui.key_down(s.form, KeyEvent { key: Key::Char('c'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    assert_eq!(*copied.borrow(), "Cliente 3");
    click(&mut s, 380.0, 3);
    s.ui.key_down(s.form, KeyEvent { key: Key::Insert, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    assert_eq!(*copied.borrow(), "4,50", "o texto formatado, como aparece na grade");
}

#[test]
fn double_click_on_the_header_separator_fits_the_column() {
    let mut s = setup(12);
    double_press(&mut s, 327.0, 16.0);
    let width = grid(&s).grid_columns()[1].width;
    assert!(width < 225.0 && width > 36.0, "encolheu para o maior texto: {width}");
    double_press(&mut s, 102.0 + width, 16.0);
    assert_eq!(grid(&s).grid_columns()[1].width, width, "de novo: a mesma largura");
    // com menos registros o maior texto ("Cliente 5") é mais curto
    let mut few = setup(5);
    double_press(&mut few, 327.0, 16.0);
    assert!(grid(&few).grid_columns()[1].width < width);
}

#[test]
fn narrow_grid_gets_a_horizontal_scroll_bar() {
    let mut s = setup(3);
    s.ui.control_mut(s.grid).unwrap().set_rect(0.0, 0.0, 300.0, 332.0);
    s.ui.layout();
    s.ui.paint(s.form, &mut NullCanvas);
    assert_eq!(grid(&s).scroll_x(), 0.0);
    // clique no trilho, à direita do polegar: anda uma página
    press(&mut s, PointerEvent { position: PointF::new(280.0, 332.0 - 7.0), ..Default::default() });
    assert!(grid(&s).scroll_x() > 0.0);
    assert_eq!(rec_no(&mut s), Some(0), "o clique na barra não muda o registro");
    // Home volta à primeira coluna (e ao começo)
    s.ui.key_down(s.form, key(Key::Home));
    assert_eq!(grid(&s).scroll_x(), 0.0);
}

#[test]
fn wide_grid_has_no_horizontal_bar() {
    let mut s = setup(3);
    // sem a barra, a faixa de baixo é linha da grade: o clique seleciona
    press(&mut s, PointerEvent { position: PointF::new(280.0, 332.0 - 7.0), ..Default::default() });
    assert_eq!(grid(&s).scroll_x(), 0.0);
}

/// Dois cliques seguidos no mesmo lugar (o duplo clique).
fn double_press(s: &mut Setup, x: f32, y: f32) {
    for _ in 0..2 {
        press(s, PointerEvent { position: PointF::new(x, y), ..Default::default() });
    }
}

/// Guarda as cores das bordas de 2 px (a da célula atual).
#[derive(Default)]
struct Strokes(Vec<Color>);

impl Canvas for Strokes {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, stroke: &Stroke) {
        if let (2.0, Brush::Solid(color)) = (stroke.width, &stroke.brush) {
            self.0.push(*color);
        }
    }
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

#[test]
fn current_cell_border_is_the_theme_accent_or_cursor_color() {
    let mut s = setup(3);
    click(&mut s, 150.0, 1);
    let mut canvas = Strokes::default();
    s.ui.paint(s.form, &mut canvas);
    assert_eq!(canvas.0, [s.ui.theme().color("primary")], "a cor de destaque do tema");
    s.ui.set_prop(s.grid, "cursor_color", "#E91E63").unwrap();
    let mut canvas = Strokes::default();
    s.ui.paint(s.form, &mut canvas);
    assert_eq!(canvas.0, [Color::from_hex("#E91E63").unwrap()]);
}

fn titles(s: &Setup) -> Vec<(String, f32)> {
    grid(s).grid_columns().iter().map(|c| (c.title.clone(), c.width)).collect()
}

#[test]
fn closed_data_set_still_shows_the_columns() {
    // Como no designer: o dataset fechado, sem linhas, mas as colunas dos Field.
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(30), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 300.0);
    ui.layout();
    ui.design_changed(form);
    let names: Vec<String> = ui.get::<DataGrid>(grid).unwrap().grid_columns().iter().map(|c| c.title.clone()).collect();
    assert_eq!(names, ["Código", "Nome"]);
}

#[test]
fn columns_list_takes_widths() {
    let mut s = setup(3);
    s.ui.set_prop(s.grid, "columns", "nome:150;valor").unwrap();
    assert_eq!(titles(&s), [("Nome".to_string(), 150.0), ("Valor".to_string(), 120.0)]);
}

#[test]
fn column_components_define_the_grid_columns() {
    let mut s = setup(3);
    let nome = s.ui.add(Column::new("nome").title("Cliente").width(200.0), s.grid);
    s.ui.add(Column::new("valor"), s.grid);
    s.ui.send_message(s.grid, &DesignChanged);
    assert_eq!(titles(&s), [("Cliente".to_string(), 200.0), ("Valor".to_string(), 120.0)]);
    // mudar a coluna pelo nome (Object Inspector, protocolo) refaz a grade
    s.ui.set_prop(nome, "width", 90.0).unwrap();
    s.ui.set_prop(nome, "visible", true).unwrap();
    assert_eq!(titles(&s)[0], ("Cliente".to_string(), 90.0));
    s.ui.set_prop(nome, "visible", false).unwrap();
    assert_eq!(titles(&s), [("Valor".to_string(), 120.0)]);
}

#[test]
fn designer_drags_the_column_separator() {
    let mut s = setup(3);
    // separador entre Nome e Valor (x = 327), no cabeçalho
    let grip = s.ui.component(s.grid).unwrap().design_hooks().unwrap().design_grip(PointF::new(327.0, 16.0));
    assert_eq!(grip, Some(1));
    assert_eq!(s.ui.component(s.grid).unwrap().design_hooks().unwrap().design_grip(PointF::new(327.0, 80.0)), None, "só no cabeçalho");
    let change = s.ui.component_mut(s.grid).unwrap().design_hooks_mut().unwrap().design_drag(1, PointF::new(-25.0, 0.0), true).unwrap();
    assert_eq!(change, (None, "columns", PropValue::String("id:80;nome:200;valor:120;ativo:70".into())));
    // com componentes Column, a largura vai para a coluna
    let nome = s.ui.add(Column::new("nome"), s.grid);
    s.ui.send_message(s.grid, &DesignChanged);
    let change = s.ui.component_mut(s.grid).unwrap().design_hooks_mut().unwrap().design_drag(0, PointF::new(30.0, 0.0), true).unwrap();
    assert_eq!(change, (Some(nome), "width", PropValue::Float(255.0)));
}

fn index(s: &Setup) -> String {
    s.ui.get::<ClientDataSet>(s.cds).unwrap().effective_index().unwrap()
}

fn title_click(s: &mut Setup, x: f32, ctrl: bool) {
    s.ui.pointer_down(s.form, PointerEvent { position: PointF::new(x, 16.0), modifiers: Modifiers { ctrl, ..Default::default() }, ..Default::default() });
    s.ui.pointer_up(s.form, PointerEvent { position: PointF::new(x, 16.0), ..Default::default() });
}

#[test]
fn third_title_click_restores_the_data_set_order() {
    let mut s = setup(4);
    s.ui.set_prop(s.cds, "index_field_names", "valor desc").unwrap();
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "nome");
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "nome desc");
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "valor desc", "a ordem do dataset de volta");
    // a coluna que já é a ordem do dataset: o primeiro clique inverte, o segundo volta
    s.ui.set_prop(s.cds, "index_field_names", "nome").unwrap();
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "nome desc");
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "nome");
}

#[test]
fn ctrl_title_click_adds_columns_to_the_order() {
    let mut s = setup(4);
    title_click(&mut s, 380.0, false); // valor
    title_click(&mut s, 480.0, true); // + ativo
    title_click(&mut s, 200.0, true); // + nome
    assert_eq!(index(&s), "valor;ativo;nome");
    title_click(&mut s, 480.0, true); // ativo decrescente
    assert_eq!(index(&s), "valor;ativo desc;nome");
    title_click(&mut s, 480.0, true); // ativo sai
    assert_eq!(index(&s), "valor;nome");
    // sem Ctrl: só a coluna clicada
    title_click(&mut s, 200.0, false);
    assert_eq!(index(&s), "nome");
    // tirar todas com Ctrl: a ordem do dataset (nenhuma)
    title_click(&mut s, 200.0, true);
    title_click(&mut s, 200.0, true);
    assert_eq!(index(&s), "");
}
