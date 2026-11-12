//! O relatório do Tabernáculo, lote 4 (layout e texto): o que dá para conferir sem janela.

use aylis_controls::*;
use aylis_core::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 600.0, 400.0));
    ui.show(form).unwrap();
    ui.layout();
    (ui, form)
}

fn rect(ui: &Ui, id: ComponentId) -> RectF {
    ui.control(id).unwrap().bounds()
}

fn sized(ui: &mut Ui, parent: ComponentId, w: f32, h: f32) -> ComponentId {
    let id = ui.add(Layout::default(), parent);
    ui.control_mut(id).unwrap().set_width(w).set_height(h);
    id
}

/// L05: o `gap` do Layout entre os encaixados, também na altura automática.
#[test]
fn layout_gap_spaces_the_docked_children() {
    let (mut ui, form) = setup();
    let col = ui.add(Layout::default(), form);
    ui.set_prop(col, "gap", 10).unwrap();
    ui.set_prop(col, "align", "top").unwrap();
    ui.set_prop(col, "auto_size", true).unwrap();
    let a = sized(&mut ui, col, 50.0, 20.0);
    let b = sized(&mut ui, col, 50.0, 20.0);
    for id in [a, b] {
        ui.set_prop(id, "align", "top").unwrap();
    }
    ui.layout();
    assert_eq!(rect(&ui, b).y - rect(&ui, a).bottom(), 10.0);
    assert_eq!(rect(&ui, col).height, 50.0, "20 + 10 + 20");
}

/// L04: o FlowLayout centra na altura da linha, e o `grow` empurra o resto para o fim.
#[test]
fn flow_layout_aligns_vertically_and_grows_a_spacer() {
    let (mut ui, form) = setup();
    let flow = ui.add(FlowLayout::default(), form);
    ui.control_mut(flow).unwrap().set_rect(0.0, 0.0, 400.0, 100.0);
    ui.set_prop(flow, "v_align", "center").unwrap();
    ui.set_prop(flow, "gap", 0).unwrap();
    let tall = sized(&mut ui, flow, 50.0, 40.0);
    let short = sized(&mut ui, flow, 50.0, 20.0);
    let spacer = sized(&mut ui, flow, 0.0, 10.0);
    ui.set_prop(spacer, "grow", 1).unwrap();
    let end = sized(&mut ui, flow, 50.0, 20.0);
    ui.layout();
    assert_eq!(rect(&ui, short).y - rect(&ui, tall).y, 10.0, "no meio dos 40");
    assert_eq!(rect(&ui, end).right(), 400.0, "o espaçador empurrou para o fim");
}

/// L12: com `baseline`, os textos de alturas diferentes ficam na mesma linha de base.
#[test]
fn flow_layout_aligns_text_by_baseline() {
    let (mut ui, form) = setup();
    let flow = ui.add(FlowLayout::default(), form);
    ui.control_mut(flow).unwrap().set_rect(0.0, 0.0, 400.0, 100.0);
    ui.set_prop(flow, "baseline", true).unwrap();
    let title = ui.add(Label::new("Título"), flow);
    ui.control_mut(title).unwrap().set_width(80.0).set_height(40.0);
    ui.set_prop(title, "font_size", 24).unwrap();
    let badge = ui.add(Label::new("novo"), flow);
    ui.control_mut(badge).unwrap().set_width(40.0).set_height(18.0);
    ui.layout();
    let base = |ui: &Ui, id| rect(ui, id).y + ui.component(id).unwrap().baseline(ui, id, rect(ui, id).height).unwrap();
    assert!((base(&ui, title) - base(&ui, badge)).abs() < 0.5, "{} × {}", base(&ui, title), base(&ui, badge));
}

/// G12: o lugar fixo na grade (linha, coluna) e várias linhas.
#[test]
fn grid_layout_places_children_in_their_cells() {
    let (mut ui, form) = setup();
    let grid = ui.add(GridLayout::default(), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 300.0, 300.0);
    ui.set_prop(grid, "columns", 3).unwrap();
    ui.set_prop(grid, "gap", 0).unwrap();
    let fixed = sized(&mut ui, grid, 10.0, 30.0);
    ui.set_prop(fixed, "grid_row", 1).unwrap();
    ui.set_prop(fixed, "grid_column", 2).unwrap();
    let tall = sized(&mut ui, grid, 10.0, 30.0);
    ui.set_prop(tall, "row_span", 2).unwrap();
    let next = sized(&mut ui, grid, 10.0, 30.0);
    ui.layout();
    assert_eq!((rect(&ui, fixed).x, rect(&ui, fixed).y), (200.0, 30.0));
    assert_eq!(rect(&ui, tall).height, 60.0, "duas linhas de 30");
    assert_eq!(rect(&ui, next).x, 100.0, "o próximo segue na coluna livre");
}

/// L01: sem a calha, o conteúdo usa a largura toda; a barra não aparece sem o que rolar.
#[test]
fn scroll_box_gutter_is_optional() {
    let (mut ui, form) = setup();
    let sb = ui.add(ScrollBox::default(), form);
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    let inner = sized(&mut ui, sb, 10.0, 99.8);
    ui.set_prop(inner, "align", "top").unwrap();
    ui.layout();
    assert_eq!(rect(&ui, inner).width, 190.0, "a calha de sempre");
    ui.set_prop(sb, "gutter", false).unwrap();
    ui.layout();
    assert_eq!(rect(&ui, inner).width, 200.0);
    assert_eq!(ui.get::<ScrollBox>(sb).unwrap().max_scroll(), 0.0);
}

/// L07: o form com `auto_size` fica da altura do conteúdo; sem, como sempre.
#[test]
fn an_auto_size_form_takes_the_content_height() {
    let (mut ui, form) = setup();
    assert_eq!(ui.auto_form_height(form), None, "desligado por padrão");
    let a = sized(&mut ui, form, 10.0, 40.0);
    ui.set_prop(a, "align", "top").unwrap();
    ui.set_prop(form, "auto_size", true).unwrap();
    ui.layout();
    assert_eq!(ui.auto_form_height(form), Some(40.0));
    assert_eq!(rect(&ui, form).height, 40.0);
}

/// L11: o tamanho preferido já conta o tema.
#[test]
fn preferred_size_counts_the_theme() {
    let (mut ui, form) = setup();
    let b = ui.add(Button::new("Um texto razoável"), form);
    let pad = ui.style_padding(b);
    let size = ui.preferred_size(b);
    assert!(size.width > pad.horizontal() + 40.0, "{size:?}");
}

/// T04, T05 e T02: a altura de linha e só o peso pelo estilo; as medidas do conteúdo.
#[test]
fn line_height_font_weight_and_content_widths() {
    let (mut ui, form) = setup();
    let mut theme = ui.theme().clone();
    theme.set_style("Label.forte", Style::new(StyleProps { font_weight: Some(FontWeight::SemiBold), line_height: Some(2.0), ..Default::default() }));
    ui.set_theme(theme);
    let label = ui.add(Label::new("uma duas três quatro cinco seis sete oito nove dez"), form);
    ui.set_prop(label, "style", "forte").unwrap();
    ui.set_prop(label, "wrap", true).unwrap();
    ui.set_prop(label, "align", "top").unwrap();
    ui.set_prop(label, "auto_size", true).unwrap();
    ui.control_mut(form).unwrap().set_width(120.0);
    ui.layout();
    assert_eq!(ui.style_line_height(label), 2.0);
    let size = ui.font_of(label).size;
    let h = rect(&ui, label).height;
    assert!(h >= size * 2.0 * 2.0, "duas ou mais linhas de 2× a fonte: {h}");
    let min = ui.invoke(label, "min_content_width", &[]).unwrap();
    let max = ui.invoke(label, "max_content_width", &[]).unwrap();
    assert!(matches!((min, max), (PropValue::Float(a), PropValue::Float(b)) if a < b));
}
