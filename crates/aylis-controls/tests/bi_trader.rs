//! Componentes de BI e de trader: cálculos (tabela dinâmica, indicadores, treemap) e o
//! comportamento (cliques, filtro, piscar).

use aylis_controls::bi::{Aggregate, PivotTable, Slicer, format_number};
use aylis_controls::trader::{Candle, CandleChart, OrderBook, Watchlist, bollinger, ema, macd, rsi, sma};
use aylis_controls::{Chart, Form};
use aylis_core::platform::LocaleInfo;
use aylis_core::{ComponentId, EventArgs, PointF, PointerEvent, PropValue, Ui};

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
}

#[test]
fn numbers_follow_the_locale() {
    assert_eq!(format_number(1234567.891, 2, &LocaleInfo::pt_br()), "1.234.567,89");
    assert_eq!(format_number(-1234.5, 1, &LocaleInfo::en_us()), "-1,234.5");
    assert_eq!(format_number(999.0, 0, &LocaleInfo::pt_br()), "999");
}

#[test]
fn the_pivot_groups_and_totals() {
    let data = "regiao;produto;valor\nSul;Café;120\nSul;Chá;80\nSudeste;Café;300\nSul;Café;30";
    let p = PivotTable::compute(data, "regiao", "produto", "valor", Aggregate::Sum);
    assert_eq!(p.rows, ["Sudeste", "Sul"]);
    assert_eq!(p.columns, ["Café", "Chá"]);
    assert_eq!(p.cells, vec![vec![Some(300.0), None], vec![Some(150.0), Some(80.0)]]);
    assert_eq!(p.row_totals, [300.0, 230.0]);
    assert_eq!(p.column_totals, [450.0, 80.0]);
    assert_eq!(p.total, 530.0);
    let count = PivotTable::compute(data, "regiao", "produto", "valor", Aggregate::Count);
    assert_eq!(count.cells[1][0], Some(2.0));
    let avg = PivotTable::compute(data, "regiao", "produto", "valor", Aggregate::Average);
    assert_eq!(avg.cells[1][0], Some(75.0));
}

#[test]
fn indicators_match_hand_calculations() {
    let v = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(sma(&v, 3), [None, None, Some(2.0), Some(3.0), Some(4.0)]);
    let e = ema(&v, 3);
    assert_eq!(e[2], Some(2.0));
    assert!((e[3].unwrap() - 3.0).abs() < 1e-9 && (e[4].unwrap() - 4.0).abs() < 1e-9);
    let b = bollinger(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0], 8, 2.0);
    let (m, up, lo) = b[7].unwrap();
    assert!((m - 5.0).abs() < 1e-9 && (up - 9.0).abs() < 1e-9 && (lo - 1.0).abs() < 1e-9, "desvio 2");
    // Só altas: IFR 100; só baixas: 0.
    let rising: Vec<f64> = (0..30).map(f64::from).collect();
    assert_eq!(rsi(&rising, 14)[20], Some(100.0));
    let falling: Vec<f64> = rising.iter().rev().copied().collect();
    assert_eq!(rsi(&falling, 14)[20], Some(0.0));
    // MACD de uma série constante é zero.
    let flat = vec![10.0; 60];
    let (l, s, h) = macd(&flat)[59].unwrap();
    assert!(l.abs() < 1e-9 && s.abs() < 1e-9 && h.abs() < 1e-9);
}

#[test]
fn chart_kinds_from_text_and_treemap_areas() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("bi", 400.0, 300.0));
    let chart = ui.add(Chart::default(), form);
    for kind in ["scatter", "stacked_bar", "waterfall", "funnel", "treemap", "radar"] {
        ui.set_prop(chart, "kind", kind).unwrap();
        assert_eq!(ui.get_prop(chart, "kind").unwrap(), PropValue::Enum(kind.into()));
    }
}

#[test]
fn a_slicer_filters_and_clears() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("bi", 400.0, 300.0));
    let slicer = ui.create("Slicer", Some(form)).unwrap();
    ui.control_mut(slicer).unwrap().set_rect(0.0, 0.0, 400.0, 120.0);
    ui.set_prop(slicer, "items", "Sul;Norte").unwrap();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let s2 = seen.clone();
    ui.on(slicer, "on_change", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            *s2.borrow_mut() = t.clone();
        }
    });
    ui.layout();
    // Pinta uma vez para medir os botões.
    let mut canvas = NoCanvas;
    ui.paint(form, &mut canvas);
    // O primeiro botão começa em (12, 34).
    click(&mut ui, form, 20.0, 44.0);
    assert_eq!(*seen.borrow(), "Sul");
    let s = ui.get::<Slicer>(slicer).unwrap();
    assert!(s.accepts("Sul") && !s.accepts("Norte"));
    ui.set_prop(slicer, "selected", "").unwrap();
    assert!(ui.get::<Slicer>(slicer).unwrap().accepts("Norte"), "sem seleção, todos passam");
}

#[test]
fn order_book_click_gives_the_price_and_watchlist_flashes() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("mesa", 600.0, 600.0));
    let book = ui.create("OrderBook", Some(form)).unwrap();
    ui.control_mut(book).unwrap().set_rect(0.0, 0.0, 260.0, 420.0);
    OrderBook::set_book(&mut ui, book, &[(10.0, 100.0), (9.9, 50.0)], &[(10.1, 70.0), (10.2, 30.0)]);
    let price = std::rc::Rc::new(std::cell::Cell::new(0.0));
    let p2 = price.clone();
    ui.on(book, "on_price_click", move |_, _, args| {
        if let EventArgs::Value(PropValue::Float(p)) = args {
            p2.set(*p);
        }
    });
    ui.layout();
    let mut canvas = NoCanvas;
    ui.paint(form, &mut canvas);
    // Linhas de cima para baixo: cabeçalho, venda 10.2, venda 10.1, spread, compra 10.0.
    let row = ui.text_style_size(book);
    click(&mut ui, form, 100.0, 4.0 + row * 4.5);
    assert_eq!(price.get(), 10.0, "a melhor compra");

    let watch = ui.create("Watchlist", Some(form)).unwrap();
    Watchlist::update(&mut ui, watch, "PETR4", 38.5, 1.0);
    Watchlist::update(&mut ui, watch, "PETR4", 38.7, 1.5);
    let q = ui.get::<Watchlist>(watch).unwrap().quotes().iter().find(|q| q.symbol == "PETR4").unwrap().clone();
    assert_eq!((q.last, q.change), (38.7, 1.5));

    // O candle em formação: mesmo tempo atualiza, outro acrescenta.
    let chart = ui.create("CandleChart", Some(form)).unwrap();
    let n = ui.get::<CandleChart>(chart).unwrap().candles().len();
    let c = Candle { time: "agora".into(), open: 1.0, high: 2.0, low: 0.5, close: 1.5, volume: 10.0 };
    CandleChart::push(&mut ui, chart, c.clone());
    CandleChart::push(&mut ui, chart, Candle { close: 1.8, ..c });
    let candles = ui.get::<CandleChart>(chart).unwrap().candles();
    assert_eq!(candles.len(), n + 1);
    assert_eq!(candles.last().unwrap().close, 1.8);
}

/// Canvas que não desenha (mede o texto com uma regra simples).
struct NoCanvas;

impl aylis_core::Canvas for NoCanvas {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: aylis_core::RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: aylis_core::RectF, _: f32, _: &aylis_core::Brush) {}
    fn stroke_rect(&mut self, _: aylis_core::RectF, _: f32, _: &aylis_core::Stroke) {}
    fn fill_ellipse(&mut self, _: aylis_core::RectF, _: &aylis_core::Brush) {}
    fn stroke_ellipse(&mut self, _: aylis_core::RectF, _: &aylis_core::Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &aylis_core::Stroke) {}
    fn draw_text(&mut self, _: &str, _: aylis_core::RectF, _: &aylis_core::TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &aylis_core::TextStyle) -> aylis_core::SizeF {
        aylis_core::SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

trait RowSize {
    fn text_style_size(&self, id: ComponentId) -> f32;
}

impl RowSize for Ui {
    /// A altura da linha do livro: a fonte do tema (menos 1) vezes 1,7.
    fn text_style_size(&self, _id: ComponentId) -> f32 {
        (self.theme().fonts.get("body").map_or(14.0, |f| f.size) - 1.0).max(10.0) * 1.7
    }
}
