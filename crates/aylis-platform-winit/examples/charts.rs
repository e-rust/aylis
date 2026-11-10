//! Os tipos de gráfico do `Chart` (os do Chart.js e os de BI), lado a lado: linha em
//! degraus e suavizada, barras de pé e deitadas, empilhados, dispersão, bolhas, área polar,
//! misto com dois eixos, escala logarítmica, eixo de tempo e os dados em JSON. Clique na
//! legenda para esconder uma série; a roda aproxima no gráfico com zoom.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example charts --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — gráficos", 1500.0, 980.0));
    let scroll = ui.add(ScrollBox::default(), form);
    ui.control_mut(scroll).unwrap().set_align(Align::Client);
    let months = "Jan;Fev;Mar;Abr;Mai;Jun";
    let charts: Vec<(&str, &str, Vec<(&str, PropValue)>)> = vec![
        ("Linha em degraus", "line", vec![("stepped", true.into())]),
        ("Linha suavizada, tracejada", "line", vec![("tension", 0.4.into()), ("series", "Vendas[triangle]=12,19,15,24,28,33|Meta[dash 6 4;none]=15,18,21,24,27,30".into())]),
        ("Área até a outra série", "line", vec![("series", "Máx[fill 1]=20,24,23,30,34,38|Mín=8,11,10,14,15,18".into())]),
        ("Barras deitadas", "horizontal_bar", vec![]),
        ("Barras empilhadas", "stacked_bar", vec![]),
        ("Áreas empilhadas", "stacked_area", vec![]),
        ("Dispersão", "scatter", vec![("series", "A=1:2,2:4,3:3,4:6,5:5,6:8|B[square]=1:5,2:3,3:7,4:4,5:9,6:6".into())]),
        ("Bolhas", "bubble", vec![("series", "Lojas=10:20:8,25:35:14,40:15:20,55:40:10|Quiosques=15:10:6,35:25:9,50:30:5".into())]),
        ("Área polar", "polar_area", vec![("labels", "Norte;Sul;Leste;Oeste;Centro".into()), ("series", "Vendas=11,16,7,3,14".into())]),
        ("Misto, dois eixos", "mixed", vec![("series", "Pedidos[bar]=120,190,150,240,280,330|Conversão[line;right;#EF4444]=2.1,2.6,2.2,3.1,3.4,3.9".into())]),
        ("Escala logarítmica", "line", vec![("log_scale", true.into()), ("series", "Usuários=3,40,520,6100,70000,820000".into())]),
        ("Eixo de tempo, zoom", "line", vec![("time_axis", true.into()), ("zoom", true.into()), ("labels", "2026-01-05;2026-01-20;2026-02-14;2026-03-30;2026-05-02;2026-07-15;2026-10-27".into()), ("series", "Saldo=10,14,9,18,22,17,26".into())]),
        ("Dados em JSON (Chart.js)", "mixed", vec![("json", r##"{"labels":["T1","T2","T3","T4"],"datasets":[{"label":"Receita","type":"bar","data":[40,55,48,70]},{"label":"Custo","type":"line","borderColor":"#8B5CF6","borderDash":[4,3],"pointStyle":"diamond","data":[30,35,41,45]}]}"##.into())]),
        ("Invertido, 0 a 50", "bar", vec![("reverse", true.into()), ("auto_scale", false.into()), ("y_min", 0.0.into()), ("y_max", 50.0.into())]),
        ("Pizza", "pie", vec![("labels", "A;B;C;D".into()), ("series", "Fatias=40,25,20,15".into())]),
        ("Radar", "radar", vec![]),
    ];
    let (w, h, cols) = (350.0, 230.0, 4);
    for (i, (title, kind, props)) in charts.into_iter().enumerate() {
        let c = ui.add(Chart::default(), scroll);
        let (col, row) = (i % cols, i / cols);
        ui.control_mut(c).unwrap().set_rect(12.0 + col as f32 * (w + 12.0), 12.0 + row as f32 * (h + 12.0), w, h);
        ui.set_prop(c, "title", title)?;
        ui.set_prop(c, "kind", PropValue::Enum(kind.into()))?;
        if !props.iter().any(|(k, _)| *k == "labels") {
            ui.set_prop(c, "labels", months)?;
        }
        for (k, v) in props {
            ui.set_prop(c, k, v)?;
        }
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
