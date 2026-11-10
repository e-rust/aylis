//! Relatório de bandas com a visualização: vendas por região com título, cabeçalho e rodapé
//! de página, grupos com totais, linhas e uma faixa colorida; os botões da barra exportam o
//! PDF e imprimem. `--pdf arquivo.pdf` só grava o PDF e sai.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example report --features cpu
//! ```

use aylis_controls::shapes::Line;
use aylis_controls::{Form, Label};
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — relatório", 900.0, 820.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("regiao", FieldType::String), cds);
    ui.add(Field::new("produto", FieldType::String), cds);
    ui.add(Field::new("quantidade", FieldType::Integer), cds);
    ui.add(Field::new("valor", FieldType::Currency).format("R$ #,##0.00"), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let mut rows = Vec::new();
    for (r, region) in ["Centro-Oeste", "Nordeste", "Norte", "Sudeste", "Sul"].iter().enumerate() {
        for i in 0..(6 + r * 2) {
            let value = Decimal::parse(&format!("{}.{:02}", 120 + i * 37 + r * 11, (i * 13) % 100)).unwrap();
            rows.push(vec![(*region).into(), format!("Produto {:02}", i + 1).into(), ((i as i64 % 7) + 1).into(), value.into()]);
        }
    }
    data_set(&mut ui, cds).load_rows(rows)?;

    let rep = ui.add(Report::new(ds), form);
    ui.set_prop(rep, "title", "Vendas por região")?;
    let band = |ui: &mut Ui, kind: BandKind, height: f32| ui.add(ReportBand::new(kind, height), rep);
    let text = |ui: &mut Ui, band: ComponentId, x: f32, w: f32, text: &str, size: f32, bold: bool, align: &str| {
        let l = ui.add(Label::new(text), band);
        let height = ui.control(band).unwrap().bounds().height;
        ui.control_mut(l).unwrap().set_rect(x, 0.0, w, height);
        let _ = ui.set_prop(l, "font_size", size as f64);
        if bold {
            let _ = ui.set_prop(l, "font_weight", "bold");
        }
        let _ = ui.set_prop(l, "h_align", PropValue::Enum(align.into()));
        l
    };
    let title = band(&mut ui, BandKind::Title, 56.0);
    text(&mut ui, title, 0.0, 523.0, "Vendas por região", 22.0, true, "leading");
    let header = band(&mut ui, BandKind::PageHeader, 26.0);
    text(&mut ui, header, 0.0, 300.0, "Emitido em [DATE] às [TIME]", 9.0, false, "leading");
    text(&mut ui, header, 300.0, 223.0, "Página [PAGE] de [PAGES]", 9.0, false, "trailing");
    let group = ui.add(ReportBand::new(BandKind::GroupHeader, 30.0).grouped_by("regiao"), rep);
    ui.set_prop(group, "fill", Color::from_hex("#E8EEFB").unwrap())?;
    text(&mut ui, group, 8.0, 300.0, "[regiao]", 13.0, true, "leading");
    text(&mut ui, group, 330.0, 60.0, "Qtd.", 10.0, true, "trailing");
    text(&mut ui, group, 400.0, 115.0, "Valor", 10.0, true, "trailing");
    let detail = band(&mut ui, BandKind::Detail, 20.0);
    text(&mut ui, detail, 8.0, 40.0, "[LINE]", 9.0, false, "leading");
    text(&mut ui, detail, 50.0, 270.0, "[produto]", 10.0, false, "leading");
    text(&mut ui, detail, 330.0, 60.0, "[quantidade]", 10.0, false, "trailing");
    text(&mut ui, detail, 400.0, 115.0, "[valor]", 10.0, false, "trailing");
    let footer = ui.add(ReportBand::new(BandKind::GroupFooter, 30.0).grouped_by("regiao"), rep);
    let line = ui.add(Line::default(), footer);
    ui.control_mut(line).unwrap().set_rect(330.0, 0.0, 193.0, 4.0);
    ui.set_prop(line, "direction", PropValue::Enum("horizontal".into()))?;
    ui.set_prop(line, "stroke_width", 1.0)?;
    text(&mut ui, footer, 8.0, 300.0, "Total de [regiao] ([COUNT()] produtos)", 10.0, true, "leading");
    text(&mut ui, footer, 330.0, 60.0, "[SUM(quantidade)]", 10.0, true, "trailing");
    text(&mut ui, footer, 400.0, 115.0, "[SUM(valor)]", 10.0, true, "trailing");
    let summary = band(&mut ui, BandKind::Summary, 44.0);
    text(&mut ui, summary, 8.0, 300.0, "Total geral · média [AVG(valor)]", 12.0, true, "leading");
    text(&mut ui, summary, 330.0, 193.0, "[SUM(valor)]", 14.0, true, "trailing");
    let page_footer = band(&mut ui, BandKind::PageFooter, 24.0);
    text(&mut ui, page_footer, 0.0, 523.0, "aylis · relatório de bandas", 8.0, false, "center");

    let args: Vec<String> = std::env::args().collect();
    if let Some(path) = args.iter().position(|a| a == "--pdf").and_then(|i| args.get(i + 1)) {
        ui.layout();
        std::fs::write(path, report::to_pdf(&mut ui, rep))?;
        return Ok(());
    }
    let preview = ui.add(ReportPreview::new(rep), form);
    ui.control_mut(preview).unwrap().set_align(Align::Client);
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
