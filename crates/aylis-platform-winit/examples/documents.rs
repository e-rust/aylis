//! DocViewer: um PDF, uma planilha e um DOCX, cada um numa aba (os arquivos de teste do
//! aylis-docview, ou outro arquivo passado na linha de comando).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example documents --features cpu
//! cargo run -p aylis-platform-winit --example documents --features cpu -- relatorio.pdf
//! ```

use aylis_controls::*;
use aylis_core::*;
use aylis_docview::DocViewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_docview::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — documentos", 900.0, 700.0));
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    let data = concat!(env!("CARGO_MANIFEST_DIR"), "/../aylis-docview/tests/data/");
    let files: Vec<String> = match std::env::args().nth(1) {
        Some(f) => vec![f],
        None => ["sample.pdf", "sample.xlsx", "sample.docx"].iter().map(|f| format!("{data}{f}")).collect(),
    };
    for file in files {
        let name = std::path::Path::new(&file).file_name().and_then(|n| n.to_str()).unwrap_or("arquivo").to_string();
        let tab = TabControl::add_tab(&mut ui, tabs, &name);
        let viewer = ui.add(DocViewer::new(file), tab);
        ui.control_mut(viewer).unwrap().set_align(Align::Client);
    }
    // AYLIS_TAB=1 abre direto na segunda aba (para as capturas de tela).
    if let Some(i) = std::env::var("AYLIS_TAB").ok().and_then(|t| t.parse().ok()) {
        TabControl::select(&mut ui, tabs, i);
    }
    aylis_platform_winit::run(ui, form)
}
