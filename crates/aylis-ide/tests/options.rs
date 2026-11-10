//! Ferramentas → Opções da IDE: as abas e os campos de cada ferramenta.

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{app, options};

#[test]
fn the_options_screen_has_a_tab_per_language_and_mcp() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    options::open(&mut ide.ui, &state, &ide.ids);
    let ui = &mut ide.ui;
    let form = ui.find("frmOpcoesIde").expect("a tela abriu");
    let mut all = vec![form];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    let tabs: Vec<String> = all
        .iter()
        .filter_map(|&c| ui.get::<aylis_controls::TabItem>(c).map(|t| t.caption.clone()))
        .collect();
    for wanted in ["Geral", "TypeScript", "C#", "Java", "Go", "C++", "Pascal", "Publicação", "Editor externo", "Rust e Python", "MCP"] {
        assert!(tabs.iter().any(|t| t == wanted), "falta a aba {wanted}: {tabs:?}");
    }
    let buttons: Vec<String> = all.iter().filter_map(|&c| ui.get::<aylis_controls::Button>(c).map(|b| b.text.clone())).collect();
    assert!(buttons.iter().any(|b| b == "Configurar o Claude Code"), "{buttons:?}");
    assert!(buttons.iter().any(|b| b == "Gerar novo token"), "{buttons:?}");
    assert!(buttons.iter().any(|b| b == "Limpar agora"), "a pasta de compilação, na aba do Rust: {buttons:?}");

    // A captura (para conferir o visual): AYLIS_SHOTS=pasta.
    if let Ok(dir) = std::env::var("AYLIS_SHOTS") {
        let renderer = aylis_skia::SkiaRenderer::new();
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
        for (n, index) in [(0usize, "geral"), (7, "editor"), (9, "rust"), (10, "mcp")].map(|(i, n)| (i, n)) {
            let tab_control = all.iter().copied().find(|&c| ui.get::<aylis_controls::TabControl>(c).is_some()).unwrap();
            aylis_controls::TabControl::select(ui, tab_control, n);
            ui.layout();
            let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
            let background = ui.theme().color("background");
            let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
            std::fs::write(std::path::Path::new(&dir).join(format!("opcoes_{index}.png")), png).unwrap();
        }
    }
}
