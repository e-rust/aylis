//! O conteúdo de exemplo (`aylis::samples`) que a galeria e a documentação mostram.

use aylis::core::pages::Op;
use aylis::core::{PropValue, Ui};

/// O `ReportPreview` de exemplo mostra um relatório de verdade: os clientes, cada um com o
/// `Barcode` da banda de detalhe com o código dele (o `[codigo]` do texto vira o valor).
#[test]
fn the_report_preview_sample_has_a_barcode_in_the_detail_band() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let form = ui.add_root(aylis::controls::Form::new("frmGaleria", 600.0, 700.0));
    let preview = aylis::samples::create(&mut ui, "ReportPreview", form).unwrap();
    let report = ui.find_in(form, "rptDemo").expect("o relatório de exemplo");
    assert_eq!(ui.get_prop(preview, "report").unwrap(), PropValue::Component(Some(report)));
    let pages = aylis::dbcontrols::report::prepare(&mut ui, report);
    let page = &pages.pages[0];
    let texts: Vec<&str> = page.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { Some(text.as_str()) } else { None }).collect();
    assert!(texts.contains(&"Ana Souza") && texts.contains(&"Gabi Torres"), "um registro por linha: {texts:?}");
    assert!(texts.contains(&"CLI-0001") && texts.contains(&"CLI-0007"), "o texto do código de cada um: {texts:?}");
    // As barras: muitos retângulos preenchidos (sete códigos).
    let bars = page.ops.iter().filter(|op| matches!(op, Op::FillRect { .. })).count();
    assert!(bars > 7 * 20, "as barras dos sete códigos: {bars}");
}

/// O Sankey e o waffle na galeria, com os dados que cada um pede.
#[test]
fn the_chart_gallery_has_sankey_and_waffle() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let variants: Vec<(String, String)> = ui.variants_of("Chart").into_iter().map(|v| (v.name, v.caption)).collect();
    for (name, caption) in [("sankey", "Sankey"), ("waffle", "Waffle")] {
        assert!(variants.iter().any(|(n, c)| n == name && c == caption), "Chart · {caption}: {variants:?}");
    }
    let form = ui.add_root(aylis::controls::Form::new("frmGaleria", 600.0, 400.0));
    let chart = ui.create("Chart.sankey", Some(form)).unwrap();
    assert_eq!(ui.get_prop(chart, "kind").unwrap(), PropValue::Enum("sankey".into()));
    assert!(matches!(ui.get_prop(chart, "labels").unwrap(), PropValue::String(s) if s.contains("Receita>Moradia")));
}
