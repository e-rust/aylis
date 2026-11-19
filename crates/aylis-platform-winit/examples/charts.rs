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
        (
            "Linha suavizada, tracejada",
            "line",
            vec![
                ("tension", 0.4.into()),
                ("series", r#"[{"name":"Vendas","values":[12,19,15,24,28,33],"point":"triangle"},{"name":"Meta","values":[15,18,21,24,27,30],"dash":[6,4],"point":"none"}]"#.into()),
            ],
        ),
        (
            "Área até a outra série",
            "line",
            vec![("series", r#"[{"name":"Máx","values":[20,24,23,30,34,38],"fill":"series","fill_series":1},{"name":"Mín","values":[8,11,10,14,15,18]}]"#.into())],
        ),
        ("Barras deitadas", "horizontal_bar", vec![]),
        ("Barras empilhadas", "stacked_bar", vec![]),
        ("Áreas empilhadas", "stacked_area", vec![]),
        (
            "Dispersão",
            "scatter",
            vec![("series", r#"[{"name":"A","x":[1,2,3,4,5,6],"values":[2,4,3,6,5,8]},{"name":"B","x":[1,2,3,4,5,6],"values":[5,3,7,4,9,6],"point":"square"}]"#.into())],
        ),
        (
            "Bolhas",
            "bubble",
            vec![(
                "series",
                r#"[{"name":"Lojas","x":[10,25,40,55],"values":[20,35,15,40],"radius":[8,14,20,10]},{"name":"Quiosques","x":[15,35,50],"values":[10,25,30],"radius":[6,9,5]}]"#.into(),
            )],
        ),
        ("Área polar", "polar_area", vec![("labels", "Norte;Sul;Leste;Oeste;Centro".into()), ("series", r#"[{"name":"Vendas","values":[11,16,7,3,14]}]"#.into())]),
        (
            "Misto, dois eixos",
            "mixed",
            vec![(
                "series",
                r##"[{"name":"Pedidos","values":[120,190,150,240,280,330],"kind":"bar"},{"name":"Conversão","values":[2.1,2.6,2.2,3.1,3.4,3.9],"kind":"line","right_axis":true,"color":"#EF4444"}]"##.into(),
            )],
        ),
        ("Escala logarítmica", "line", vec![("log_scale", true.into()), ("series", r#"[{"name":"Usuários","values":[3,40,520,6100,70000,820000]}]"#.into())]),
        (
            "Eixo de tempo, zoom",
            "line",
            vec![
                ("time_axis", true.into()),
                ("zoom", true.into()),
                ("labels", "2026-01-05;2026-01-20;2026-02-14;2026-03-30;2026-05-02;2026-07-15;2026-10-27".into()),
                ("series", r#"[{"name":"Saldo","values":[10,14,9,18,22,17,26]}]"#.into()),
            ],
        ),
        ("Dados em JSON (Chart.js)", "mixed", vec![("json", r##"{"labels":["T1","T2","T3","T4"],"datasets":[{"label":"Receita","type":"bar","data":[40,55,48,70]},{"label":"Custo","type":"line","borderColor":"#8B5CF6","borderDash":[4,3],"pointStyle":"diamond","data":[30,35,41,45]}]}"##.into())]),
        ("Invertido, 0 a 50", "bar", vec![("reverse", true.into()), ("auto_scale", false.into()), ("y_min", 0.0.into()), ("y_max", 50.0.into())]),
        ("Pizza", "pie", vec![("labels", "A;B;C;D".into()), ("series", r#"[{"name":"Fatias","values":[40,25,20,15]}]"#.into())]),
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
