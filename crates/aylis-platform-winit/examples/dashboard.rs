//! Painel de BI e mesa de trader: KPIs, tabela dinâmica, segmentador, mapa, os gráficos
//! de BI, e candles com indicadores, livro de ofertas, profundidade, lista de ativos
//! (piscando), times & trades e a faixa de cotações. Os preços mudam a cada 0,5 s.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example dashboard --features cpu
//! ```

use aylis_controls::bi::KpiCard;
use aylis_controls::trader::{TimeAndSales, Trade, Watchlist};
use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — BI e trader", 1400.0, 900.0));
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    let bi = ui.add(TabItem::new("BI"), tabs);
    let trade = ui.add(TabItem::new("Trader"), tabs);
    if std::env::var("AYLIS_DASHBOARD").is_ok_and(|v| v == "trader") {
        let _ = ui.set_prop(tabs, "tab_index", 1);
    }

    let place = |ui: &mut Ui, class: &str, parent: ComponentId, rect: (f32, f32, f32, f32)| -> ComponentId {
        let id = ui.create(class, Some(parent)).unwrap();
        ui.control_mut(id).unwrap().set_rect(rect.0, rect.1, rect.2, rect.3);
        id
    };

    // ---- BI ----
    let kpis = [("Receita", 128_450.0, 112_300.0, "R$ ", "", true), ("Pedidos", 3_412.0, 3_598.0, "", "", true), ("Ticket médio", 37.6, 31.2, "R$ ", "", true), ("Devoluções", 2.1, 2.9, "", "%", false)];
    for (i, (title, value, previous, prefix, suffix, better)) in kpis.iter().enumerate() {
        let k = ui.add(KpiCard::default(), bi);
        ui.control_mut(k).unwrap().set_rect(16.0 + i as f32 * 252.0, 16.0, 240.0, 130.0);
        ui.set_prop(k, "title", *title)?;
        ui.set_prop(k, "value", *value)?;
        ui.set_prop(k, "previous", *previous)?;
        ui.set_prop(k, "prefix", *prefix)?;
        ui.set_prop(k, "suffix", *suffix)?;
        ui.set_prop(k, "decimals", if *value < 100.0 { 1 } else { 0 })?;
        ui.set_prop(k, "higher_is_better", *better)?;
    }
    let slicer = place(&mut ui, "Slicer", bi, (1032.0, 16.0, 330.0, 130.0));
    ui.set_prop(slicer, "selected", "Sudeste;Sul")?;
    place(&mut ui, "PivotTable", bi, (16.0, 160.0, 500.0, 220.0));
    let kinds = [("waterfall", "Fluxo de caixa", "Saldo;Vendas;Serviços;Folha;Impostos;Outros", "Variação=120,80,35,-90,-40,-15"), ("funnel", "Funil de vendas", "Visitas;Leads;Propostas;Fechados", "Etapas=1000,420,160,64"), ("treemap", "Vendas por categoria", "Bebidas;Mercearia;Limpeza;Padaria;Hortifrúti;Frios", "Vendas=420,310,180,150,120,90"), ("radar", "Avaliação", "Preço;Qualidade;Entrega;Suporte;Prazo", "Nós=8,9,7,6,8|Concorrente=7,6,8,8,6"), ("stacked_bar", "Canais", "Jan;Fev;Mar;Abr;Mai;Jun", "Loja=12,14,13,15,16,18|Site=8,9,12,14,17,20|App=3,4,6,8,11,14"), ("scatter", "Preço x volume", "", "Preço=10,12,14,15,18,20,22,25,28,30|A=90,85,80,70,62,58,50,41,35,30|B=60,66,58,55,50,47,40,38,30,25")];
    for (i, (kind, title, labels, series)) in kinds.iter().enumerate() {
        let c = ui.add(Chart::default(), bi);
        let (col, row) = (i % 3, i / 3);
        ui.control_mut(c).unwrap().set_rect(528.0 + col as f32 * 280.0, 160.0 + row as f32 * 230.0, 270.0, 220.0);
        ui.set_prop(c, "kind", *kind)?;
        ui.set_prop(c, "title", *title)?;
        ui.set_prop(c, "labels", *labels)?;
        ui.set_prop(c, "series", *series)?;
        ui.set_prop(c, "legend", false)?;
    }
    let map = place(&mut ui, "ChoroplethMap", bi, (16.0, 392.0, 500.0, 240.0));
    let _ = map;

    // ---- Trader ----
    let tape = place(&mut ui, "TickerTape", trade, (0.0, 0.0, 1400.0, 34.0));
    let _ = tape;
    let candles = place(&mut ui, "CandleChart", trade, (16.0, 48.0, 860.0, 520.0));
    ui.set_prop(candles, "indicators", "sma:9;sma:21;bollinger:20")?;
    ui.set_prop(candles, "volume_profile", true)?;
    place(&mut ui, "OrderBook", trade, (892.0, 48.0, 240.0, 420.0));
    let watch = place(&mut ui, "Watchlist", trade, (1144.0, 48.0, 240.0, 260.0));
    let times = place(&mut ui, "TimeAndSales", trade, (1144.0, 320.0, 240.0, 420.0));
    place(&mut ui, "DepthChart", trade, (16.0, 580.0, 860.0, 200.0));
    let candles2 = place(&mut ui, "CandleChart", trade, (892.0, 480.0, 240.0, 300.0));
    ui.set_prop(candles2, "lower", "rsi")?;
    ui.set_prop(candles2, "indicators", "")?;

    // Preços andando.
    let timer = ui.add(Timer::new(500), form);
    let tick = std::cell::Cell::new(0u32);
    let symbols = ["PETR4", "VALE3", "ITUB4", "BBDC4", "ABEV3", "WEGE3"];
    let prices = std::cell::RefCell::new([38.5, 61.1, 33.4, 14.0, 12.9, 52.3]);
    ui.on(timer, "on_timer", move |ui, _, _| {
        tick.set(tick.get() + 1);
        let tick = tick.get();
        let mut prices = prices.borrow_mut();
        for (i, s) in symbols.iter().enumerate() {
            let step = (((tick as usize * 7 + i * 13) % 11) as f64 - 5.0) / 100.0;
            prices[i] = (prices[i] + step).max(1.0);
            Watchlist::update(ui, watch, s, prices[i], step * 10.0);
        }
        let size = (((tick * 37) % 90) + 1) as f64 * 100.0;
        TimeAndSales::add(ui, times, Trade { time: format!("10:{:02}:{:02}", tick / 60 % 60, tick % 60), price: prices[0], size, buy: tick % 3 != 0 });
    });
    aylis_platform_winit::run(ui, form)
}
