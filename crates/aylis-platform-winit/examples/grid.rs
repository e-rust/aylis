//! DataGrid com 1 milhão de pedidos, buscados do "backend" sob demanda em pacotes (como o
//! `FetchOptions.Mode = fmOnDemand` do FireDAC).
//!
//! ```text
//! cargo run --release -p aylis-platform-winit --example grid
//! cargo run --release -p aylis-platform-winit --example grid -- --all 300000
//! ```
//!
//! Role com a roda, a barra ou o teclado (PageDown, Ctrl+End...). Enter, F2 ou começar a
//! digitar editam a célula; Insert inclui; Ctrl+Delete exclui. Com `--all N`, carrega N
//! registros de uma vez para mostrar a virtualização sem busca sob demanda.

use std::time::Instant;

use aylis_controls::{Button, Form, Label, Layout};
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::{DataGrid, DataNavigator};

const TOTAL: usize = 1_000_000;
const PACKET: usize = 2_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let all: Option<usize> = args.iter().position(|a| a == "--all").and_then(|i| args.get(i + 1)?.parse().ok());

    let mut ui = Ui::new();
    if args.iter().any(|a| a == "--dark") {
        ui.set_theme(Theme::dark());
    }
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let muted = ui.theme().color("text_muted");

    let form = ui.add_root(Form::new("aylis — 1 milhão de pedidos (DataGrid)", 1000.0, 640.0));
    ui.control_mut(form).unwrap().set_padding(10.0);

    let cds = ui.add(ClientDataSet::new().with_key("numero"), form);
    ui.add(Field::new("numero", FieldType::Integer).label("Pedido"), cds);
    ui.add(Field::new("cliente", FieldType::String).label("Cliente").size(32).required(), cds);
    ui.add(Field::new("data", FieldType::Date).label("Data"), cds);
    ui.add(Field::new("valor", FieldType::Currency).label("Valor").format("R$ #,##0.00").min(0.0), cds);
    ui.add(
        Field::new("status", FieldType::Integer).label("Situação").align(TextAlign::Leading).width(110.0).on_get_text(|v, _| {
            match v.as_i64() {
                Some(1) => "Aberto",
                Some(2) => "Faturado",
                Some(3) => "Entregue",
                Some(4) => "Cancelado",
                _ => "",
            }
            .into()
        }),
        cds,
    );
    ui.add(Field::new("pago", FieldType::Boolean).label("Pago"), cds);
    let ds = ui.add(DataSource::new(cds), form);

    let top = ui.add(Layout::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    let nav = ui.add(DataNavigator::new(ds), top);
    ui.control_mut(nav).unwrap().set_align(Align::Left).set_width(9.0 * 36.0).set_margins([0.0, 2.0, 12.0, 2.0]);
    let send = ui.add(Button::new("Enviar alterações"), top);
    ui.control_mut(send).unwrap().set_align(Align::Left).set_width(160.0);
    let info = ui.add(Label::new(""), top);
    ui.control_mut(info).unwrap().set_align(Align::Client);
    ui.set_prop(info, "h_align", "trailing")?;
    ui.set_prop(info, "color", muted)?;

    let status = ui.add(Label::new("Role, edite (Enter/F2 ou digite), Insert inclui, Ctrl+Delete exclui."), form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(24.0).set_margins([0.0, 6.0, 0.0, 0.0]);
    ui.set_name(status, "lblStatus")?;

    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_align(Align::Client);

    // ----- "backend" -----
    data_set(&mut ui, cds).on_apply(|changes| changes.iter().map(|c| ApplyResult::ok(c.reference)).collect());
    let started = Instant::now();
    match all {
        Some(n) => {
            data_set(&mut ui, cds).load_rows((0..n).map(order).collect())?;
        }
        None => {
            let mut d = data_set(&mut ui, cds);
            d.on_fetch(|ui, cds, loaded| {
                let end = (loaded + PACKET).min(TOTAL);
                let _ = data_set(ui, cds).append_rows((loaded..end).map(order).collect(), end < TOTAL);
            });
            d.open()?;
            d.set_has_more(true)?;
            d.fetch_more()?;
        }
    }
    let load_time = started.elapsed();

    ui.on_click(send, move |ui, _| {
        let n = aylis_data::view(ui, cds).change_count();
        let text = match data_set(ui, cds).apply_updates() {
            Ok(()) => format!("{n} alteração(ões) enviada(s)"),
            Err(e) => format!("⚠ {e}"),
        };
        say(ui, &text);
    });
    ui.on(cds, "on_error", |ui, _, args| {
        if let EventArgs::Text(message) = args {
            say(ui, &format!("⚠ {message}"));
        }
    });
    let update_info = move |ui: &mut Ui| {
        let v = aylis_data::view(ui, cds);
        let loaded = v.record_count();
        let position = v.rec_no().map_or(0, |n| n + 1);
        let more = if v.has_more() { format!(" de {}", fmt(TOTAL)) } else { String::new() };
        let pending = v.change_count();
        let text = format!("Registro {} · carregados {}{} · {} pendente(s)", fmt(position), fmt(loaded), more, pending);
        let _ = ui.set_prop(info, "text", text);
    };
    ui.on(ds, "on_data_change", move |ui, _, _| update_info(ui));
    ui.on(ds, "on_state_change", move |ui, _, _| update_info(ui));
    update_info(&mut ui);
    say(&mut ui, &format!("Primeira carga em {:.1} ms", load_time.as_secs_f64() * 1000.0));

    // `--png arquivo.png`: rola, edita e mede a pintura (com o Skia na CPU).
    #[cfg(feature = "cpu")]
    if let Some(i) = args.iter().position(|a| a == "--png") {
        let path = args.get(i + 1).ok_or("informe o arquivo depois de --png")?;
        ui.layout();
        ui.set_focus(grid);
        for _ in 0..30 {
            ui.key_down(form, KeyEvent { key: Key::PageDown, ..Default::default() });
        }
        let mut d = data_set(&mut ui, cds);
        d.edit()?;
        d.set_text("valor", "1.234,56")?;
        d.post()?;
        d.next()?;
        d.next()?;
        update_info(&mut ui);
        let renderer = aylis_skia::SkiaRenderer::new();
        let background = ui.theme().color("background");
        let frames = 20;
        let started = Instant::now();
        for _ in 0..frames {
            ui.request_repaint();
            renderer.render_rgba(1000, 640, 1.0, background, |c| ui.paint(form, c));
        }
        let per_frame = started.elapsed().as_secs_f64() * 1000.0 / frames as f64;
        let loaded = aylis_data::view(&ui, cds).record_count();
        println!("carga inicial: {:.1} ms · registros carregados: {loaded} · pintura completa (CPU): {per_frame:.2} ms/frame", load_time.as_secs_f64() * 1000.0);
        let png = renderer
            .render_png(1000, 640, 1.0, background, |c| ui.paint(form, c))
            .ok_or("falha ao gerar o PNG")?;
        std::fs::write(path, png)?;
        return Ok(());
    }

    aylis_platform_winit::run(ui, form)
}

/// Pedido número `i`, gerado de forma determinística.
fn order(i: usize) -> Vec<Value> {
    const NAMES: [&str; 12] = [
        "Maria Silva", "João Pereira", "Ana Costa", "Carlos Souza", "Paula Nogueira", "Lucas Lima",
        "Fernanda Rocha", "Rafael Alves", "Juliana Melo", "Bruno Dias", "Camila Freitas", "Diego Martins",
    ];
    let h = (i as u64).wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    let day = (h >> 8) % 365;
    let date = Date::new(2025, 1 + (day / 31) as u8 % 12, 1 + (day % 28) as u8).unwrap();
    vec![
        Value::Int(i as i64 + 1),
        Value::String(format!("{} #{}", NAMES[(h >> 16) as usize % NAMES.len()], (h >> 24) % 1000)),
        Value::Date(date),
        Value::Decimal(Decimal::new(((h >> 32) % 2_000_000) as i128, 2)),
        Value::Int(1 + ((h >> 40) % 4) as i64),
        Value::Bool((h >> 48) % 3 != 0),
    ]
}

fn fmt(n: usize) -> String {
    aylis_data::format::format_number(Decimal::new(n as i128, 0), "#,##0", &LocaleInfo::pt_br())
}

fn say(ui: &mut Ui, text: &str) {
    if let Some(label) = ui.find("lblStatus") {
        let _ = ui.set_prop(label, "text", text);
    }
}
