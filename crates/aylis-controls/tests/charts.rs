//! O Chart com os tipos e as opções do Chart.js: séries com estilo pelo texto e pelo JSON,
//! a legenda que esconde, o clique no ponto, o zoom, as datas e a animação.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

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

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("gráficos", 600.0, 400.0));
    let c = ui.add(Chart::default(), f);
    ui.control_mut(c).unwrap().set_rect(0.0, 0.0, 600.0, 400.0);
    (ui, f, c)
}

fn paint(ui: &mut Ui, f: ComponentId) {
    ui.layout();
    ui.paint(f, &mut NullCanvas);
}

fn at(p: PointF) -> PointerEvent {
    PointerEvent { position: p, ..Default::default() }
}

#[test]
fn series_carry_their_style_in_the_text() {
    let (mut ui, _, c) = setup();
    ui.set_prop(c, "series", "A[bar;right;#FF0000;width 3;dash 4 2;triangle;fill origin]=1,2|B=1:2,3:4|C[fill -1]=1:2:5").unwrap();
    let chart = ui.get::<Chart>(c).unwrap();
    let a = &chart.data()[0];
    assert_eq!((a.kind, a.right_axis, a.color, a.width, a.dash.clone(), a.point, a.fill), (Some(SeriesKind::Bar), true, Color::from_hex("#FF0000"), Some(3.0), vec![4.0, 2.0], PointStyle::Triangle, SeriesFill::Origin));
    assert_eq!((chart.data()[1].x.clone(), chart.data()[1].values.clone()), (vec![1.0, 3.0], vec![2.0, 4.0]));
    assert_eq!((chart.data()[2].radius.clone(), chart.data()[2].fill), (vec![5.0], SeriesFill::Series(1)));
    // Pelo código, o texto guarda as opções (e volta igual).
    let mut chart = Chart::default();
    chart.set_data(vec!["a".into(), "b".into()], vec![Series::new("X", vec![1.0, 2.0]).with_kind(SeriesKind::Line).on_right_axis().with_point(PointStyle::Diamond)]);
    assert_eq!(chart.series, "X[line;right;diamond]=1,2");
}

#[test]
fn chart_js_json_is_read() {
    let (mut ui, _, c) = setup();
    let json = r##"{"type":"bar","data":{"labels":["Q1","Q2"],"datasets":[
        {"label":"Receita","data":[10,20],"backgroundColor":"#10B981"},
        {"label":"Margem","type":"line","yAxisID":"y1","borderDash":[3,3],"pointStyle":"rect","fill":"-1","data":[{"x":1,"y":0.4},{"x":2,"y":0.5}],"hidden":true}]}}"##;
    ui.set_prop(c, "json", json).unwrap();
    let chart = ui.get::<Chart>(c).unwrap();
    assert_eq!(chart.data_labels(), ["Q1", "Q2"]);
    let (r, m) = (&chart.data()[0], &chart.data()[1]);
    assert_eq!((r.values.clone(), r.color), (vec![10.0, 20.0], Color::from_hex("#10B981")));
    assert_eq!((m.kind, m.right_axis, m.point, m.fill, m.hidden, m.x.clone()), (Some(SeriesKind::Line), true, PointStyle::Square, SeriesFill::Series(0), true, vec![1.0, 2.0]));
}

#[test]
fn every_kind_paints_and_the_legend_hides_series() {
    let (mut ui, f, c) = setup();
    for kind in ChartKind::NAMES {
        ui.set_prop(c, "kind", PropValue::Enum(kind.to_string())).unwrap();
        for (k, v) in [("stepped", true), ("log_scale", true), ("reverse", true), ("time_axis", true)] {
            ui.set_prop(c, k, v).unwrap();
            paint(&mut ui, f);
            ui.set_prop(c, k, false).unwrap();
        }
        paint(&mut ui, f);
    }
    ui.set_prop(c, "kind", "line").unwrap();
    paint(&mut ui, f);
    let items = ui.get::<Chart>(c).unwrap().legend_items();
    assert_eq!(items.len(), 2);
    let (rect, index) = items[0];
    ui.pointer_down(f, at(rect.center()));
    ui.pointer_up(f, at(rect.center()));
    assert!(ui.get::<Chart>(c).unwrap().data()[index].hidden, "o clique na legenda esconde");
    assert!(matches!(ui.get_prop(c, "series").unwrap(), PropValue::String(s) if s.contains("[hidden]")));
    paint(&mut ui, f);
    let rect = ui.get::<Chart>(c).unwrap().legend_items()[0].0;
    ui.pointer_down(f, at(rect.center()));
    assert!(!ui.get::<Chart>(c).unwrap().data()[index].hidden, "outro clique mostra");
    // Na pizza, a legenda é por rótulo.
    ui.set_prop(c, "kind", "pie").unwrap();
    paint(&mut ui, f);
    let (rect, i) = ui.get::<Chart>(c).unwrap().legend_items()[2];
    ui.pointer_down(f, at(rect.center()));
    assert!(ui.get::<Chart>(c).unwrap().label_hidden(i));
}

#[test]
fn clicking_a_bar_tells_the_series_and_the_point() {
    let (mut ui, f, c) = setup();
    ui.set_prop(c, "kind", "bar").unwrap();
    let got = Rc::new(RefCell::new(String::new()));
    let g = got.clone();
    ui.on(c, "on_click", move |_, _, args| *g.borrow_mut() = args.text().unwrap_or_default().to_string());
    paint(&mut ui, f);
    let plot = ui.get::<Chart>(c).unwrap().plot_rect();
    let p = PointF::new(plot.x + plot.width * (2.5 / 6.0), plot.bottom() - 5.0);
    assert_eq!(ui.get::<Chart>(c).unwrap().item_at(p), Some((0, 2)));
    ui.pointer_down(f, at(p));
    ui.pointer_up(f, at(p));
    ui.process_posted();
    assert_eq!(*got.borrow(), "Vendas;2");
    assert_eq!(ui.get_prop(c, "clicked_index").unwrap(), PropValue::Int(2));
    // A dica: passar o mouse marca a coluna.
    ui.pointer_move(f, at(p));
    paint(&mut ui, f);
}

#[test]
fn the_wheel_zooms_and_a_double_click_goes_back() {
    let (mut ui, f, c) = setup();
    ui.set_prop(c, "zoom", true).unwrap();
    paint(&mut ui, f);
    let plot = ui.get::<Chart>(c).unwrap().plot_rect();
    let middle = plot.center();
    ui.wheel(f, WheelEvent { position: middle, delta: PointF::new(0.0, 1.0), ..Default::default() });
    let (a, b) = ui.get::<Chart>(c).unwrap().zoom_window();
    assert!(b - a < 0.9 && a > 0.0, "aproximou no meio: {a}..{b}");
    paint(&mut ui, f);
    for _ in 0..2 {
        ui.pointer_down(f, at(middle));
        ui.pointer_up(f, at(middle));
    }
    assert_eq!(ui.get::<Chart>(c).unwrap().zoom_window(), (0.0, 1.0));
}

#[test]
fn dates_and_the_data_animation() {
    assert_eq!(parse_time("1970-01-02"), Some(1.0));
    assert_eq!(parse_time("2000-03-01"), Some(11017.0));
    assert_eq!(parse_time("27/10/2026"), parse_time("2026-10-27"));
    assert_eq!(parse_time("2026-10-27 12:00"), parse_time("2026-10-27").map(|d| d + 0.5));
    assert_eq!(parse_time("ontem"), None);

    let (mut ui, f, c) = setup();
    ui.set_time(10.0);
    paint(&mut ui, f);
    ui.set_prop(c, "series", "Vendas=50,50,50,50,50,50").unwrap();
    assert!(ui.get::<Chart>(c).unwrap().animating(), "trocar os dados anima");
    paint(&mut ui, f);
    ui.set_time(11.0);
    paint(&mut ui, f);
    assert!(!ui.get::<Chart>(c).unwrap().animating(), "a animação termina");
}

/// Guarda os retângulos e caminhos preenchidos (para conferir o waffle e o Sankey).
#[derive(Default)]
struct Fills {
    rects: Vec<(RectF, Color)>,
    paths: usize,
}

impl Canvas for Fills {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn fill_rect(&mut self, rect: RectF, _: f32, brush: &Brush) {
        if let Brush::Solid(color) = brush {
            self.rects.push((rect, *color));
        }
    }
    fn fill_path(&mut self, _: &Path, _: &Brush) {
        self.paths += 1;
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

#[test]
fn waffle_has_one_square_per_percent() {
    let (mut ui, f, c) = setup();
    ui.set_prop(c, "kind", "waffle").unwrap();
    ui.set_prop(c, "labels", "Norte;Sul;Leste").unwrap();
    ui.set_prop(c, "series", "Vendas=1,1,1").unwrap();
    ui.set_prop(c, "legend", false).unwrap();
    ui.layout();
    let mut fills = Fills::default();
    ui.paint(f, &mut fills);
    // Um terço cada: 34 + 33 + 33 (a sobra vai para um deles), os 100 quadrados iguais.
    let squares: Vec<&(RectF, Color)> = fills.rects.iter().filter(|(r, _)| (r.width - r.height).abs() < 0.01 && r.width < 50.0).collect();
    assert_eq!(squares.len(), 100);
    let mut counts: Vec<usize> = Vec::new();
    let mut colors: Vec<Color> = Vec::new();
    for (_, color) in squares {
        match colors.iter().position(|c| c == color) {
            Some(i) => counts[i] += 1,
            None => {
                colors.push(*color);
                counts.push(1);
            }
        }
    }
    counts.sort();
    assert_eq!(counts, [33, 33, 34]);
    // A dica e o clique dizem o item do quadrado.
    let first = fills.rects.iter().find(|(r, _)| r.width < 50.0 && (r.width - r.height).abs() < 0.01).unwrap().0;
    assert_eq!(ui.get::<Chart>(c).unwrap().item_at(first.center()), Some((0, 0)));
}

#[test]
fn sankey_draws_one_band_per_flow_and_nodes_by_the_largest_side() {
    let (mut ui, f, c) = setup();
    ui.set_prop(c, "kind", "sankey").unwrap();
    ui.set_prop(c, "labels", "Salário>Receita;Vendas>Receita;Receita>Gastos;Receita>Poupança").unwrap();
    ui.set_prop(c, "series", "Fluxo=5000,2000,4500,2500").unwrap();
    ui.set_prop(c, "legend", false).unwrap();
    ui.layout();
    let mut fills = Fills::default();
    ui.paint(f, &mut fills);
    assert_eq!(fills.paths, 4, "uma faixa por fluxo");
    // Os nós: barras de 12 px em 3 colunas; a Receita (7000) é a maior e Salário/Receita = 5/7.
    let nodes: Vec<RectF> = fills.rects.iter().filter(|(r, _)| (r.width - 12.0).abs() < 0.01).map(|(r, _)| *r).collect();
    assert_eq!(nodes.len(), 5);
    let mut xs: Vec<i32> = nodes.iter().map(|r| r.x.round() as i32).collect();
    xs.sort();
    xs.dedup();
    assert_eq!(xs.len(), 3, "origens, Receita e destinos");
    let tallest = nodes.iter().map(|r| r.height).fold(0.0, f32::max);
    assert!((nodes[0].height / tallest - 5.0 / 7.0).abs() < 0.01, "{nodes:?}");
    // Um ciclo não trava o desenho.
    ui.set_prop(c, "labels", "A>B;B>A").unwrap();
    ui.set_prop(c, "series", "Fluxo=1,1").unwrap();
    ui.layout();
    ui.paint(f, &mut Fills::default());
}

/// Grava o Sankey e o waffle para conferir a olho (`AYLIS_CHARTS=pasta`).
#[test]
fn save_sankey_and_waffle_for_a_look() {
    let Some(dir) = std::env::var_os("AYLIS_CHARTS") else { return };
    for (kind, labels, series) in [
        ("sankey", "Salário>Receita;Vendas>Receita;Receita>Moradia;Receita>Mercado;Receita>Poupança;Poupança>Reserva;Poupança>Ações", "Fluxo=5000,2000,2200,1800,3000,1800,1200"),
        ("waffle", "Norte;Sul;Leste;Oeste", "Vendas=42,27,19,12"),
    ] {
        let (mut ui, f, c) = setup();
        ui.set_prop(c, "kind", PropValue::Enum(kind.into())).unwrap();
        ui.set_prop(c, "title", kind).unwrap();
        ui.set_prop(c, "labels", labels).unwrap();
        ui.set_prop(c, "series", series).unwrap();
        ui.layout();
        let skia = aylis_skia::SkiaRenderer::new();
        ui.services_mut().text = Some(Box::new(skia.text_service()));
        let png = skia.render_png(600, 400, 1.0, Color::WHITE, |canvas| ui.paint(f, canvas)).unwrap();
        std::fs::write(std::path::Path::new(&dir).join(format!("{kind}.png")), png).unwrap();
    }
}
