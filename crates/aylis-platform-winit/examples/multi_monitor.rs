//! Um app com uma janela por monitor: a do monitor principal comum, a de cada outro monitor em
//! tela cheia (ou comum, com `--no-full`), e uma paleta pequena sempre por cima (`topmost`) que
//! pode ser levada para cima das outras, mesmo das que estão em tela cheia. Cada janela tem itens
//! para arrastar e células que recebem o soltar, e o arrasto vale entre todas as janelas.
//!
//! Cada soltar escreve uma linha `solto: <janela> <célula> <item>` (para os testes que simulam
//! o mouse).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example multi_monitor -- [--no-full] [--monitors 2]
//! ```

use std::io::Write;

use aylis_core::*;

/// Onde ficam os itens e as células, em pontos do form (os testes que simulam o mouse usam).
const ITEM: (f32, f32, f32, f32) = (20.0, 60.0, 160.0, 56.0); // esquerda, topo, largura, altura
const ITEM_STEP: f32 = 70.0;
const CELL: (f32, f32, f32, f32) = (220.0, 60.0, 180.0, 100.0);
const CELL_STEP: (f32, f32) = (200.0, 120.0);

/// Os itens e as células de uma janela; `tag` distingue as janelas no texto.
fn board(ui: &mut Ui, form: ComponentId, tag: &str, items: usize, cells: (usize, usize)) -> Result<(), Error> {
    let title = ui.create("Label", Some(form))?;
    ui.set_prop(title, "left", 20.0)?;
    ui.set_prop(title, "top", 16.0)?;
    ui.set_prop(title, "width", 600.0)?;
    ui.set_prop(title, "height", 32.0)?;
    ui.set_prop(title, "font_size", 18.0)?;
    ui.set_prop(title, "text", format!("Janela {tag}: arraste os itens para as células de qualquer janela"))?;
    for i in 0..items {
        let item = ui.create("Label", Some(form))?;
        ui.set_prop(item, "name", format!("item{tag}{}", i + 1))?;
        ui.set_prop(item, "left", ITEM.0)?;
        ui.set_prop(item, "top", ITEM.1 + i as f32 * ITEM_STEP)?;
        ui.set_prop(item, "width", ITEM.2)?;
        ui.set_prop(item, "height", ITEM.3)?;
        ui.set_prop(item, "text", format!("Item {tag}{}", i + 1))?;
        ui.set_prop(item, "h_align", "center")?;
        ui.set_prop(item, "v_align", "center")?;
        ui.set_prop(item, "style", "primary")?;
        ui.set_prop(item, "draggable", true)?;
        ui.set_prop(item, "drag_data", format!("{tag}{}", i + 1))?;
        ui.set_prop(item, "cursor", "grab")?;
    }
    for row in 0..cells.0 {
        for col in 0..cells.1 {
            let name = format!("{tag}{}{}", (b'A' + row as u8) as char, col + 1);
            let cell = ui.create("Panel", Some(form))?;
            ui.set_prop(cell, "name", format!("cel{name}"))?;
            ui.set_prop(cell, "left", CELL.0 + col as f32 * CELL_STEP.0)?;
            ui.set_prop(cell, "top", CELL.1 + row as f32 * CELL_STEP.1)?;
            ui.set_prop(cell, "width", CELL.2)?;
            ui.set_prop(cell, "height", CELL.3)?;
            ui.set_prop(cell, "accepts_drops", true)?;
            let text = ui.create("Label", Some(cell))?;
            ui.set_prop(text, "align", "client")?;
            ui.set_prop(text, "h_align", "center")?;
            ui.set_prop(text, "v_align", "center")?;
            ui.set_prop(text, "hit_test", false)?;
            ui.set_prop(text, "text", format!("Célula {name}"))?;
            let window = tag.to_string();
            ui.on(cell, "on_drag_drop", move |ui, _, args| {
                let data = match args {
                    EventArgs::Text(t) => t.clone(),
                    _ => String::new(),
                };
                let _ = ui.set_prop(text, "text", format!("Célula {name}\n← {data}"));
                println!("solto: {window} {name} {data}");
                let _ = std::io::stdout().flush();
            });
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let full = !args.iter().any(|a| a == "--no-full");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    // Quantos monitores (o `Ui` só os recebe da plataforma ao rodar): `--monitors N`, 2 sem ele.
    let monitors = args.iter().position(|a| a == "--monitors").and_then(|i| args.get(i + 1)).and_then(|n| n.parse::<usize>().ok()).unwrap_or(2).max(1);

    // A do monitor principal (o 0 na lista do sistema é o principal no Windows).
    let main = ui.create("Form", None)?;
    ui.set_prop(main, "name", "frmMonitor0")?;
    ui.set_prop(main, "caption", "Aylis — monitor 0")?;
    ui.set_prop(main, "width", 1060.0)?;
    ui.set_prop(main, "height", 560.0)?;
    ui.set_prop(main, "monitor", 0i64)?;
    board(&mut ui, main, "0", 4, (3, 4))?;

    // Uma por monitor a mais, em tela cheia.
    for m in 1..monitors {
        let form = ui.create("Form", None)?;
        ui.set_prop(form, "name", format!("frmMonitor{m}"))?;
        ui.set_prop(form, "caption", format!("Aylis — monitor {m}"))?;
        ui.set_prop(form, "width", 1060.0)?;
        ui.set_prop(form, "height", 560.0)?;
        ui.set_prop(form, "monitor", m as i64)?;
        if full {
            ui.set_prop(form, "window_state", "full_screen")?;
        }
        board(&mut ui, form, &m.to_string(), 4, (3, 4))?;
        ui.show(form)?;
    }

    // A paleta: pequena, sempre por cima de todas, no monitor principal.
    let palette = ui.create("Form", None)?;
    ui.set_prop(palette, "name", "frmPaleta")?;
    ui.set_prop(palette, "caption", "Aylis — paleta (sempre por cima)")?;
    ui.set_prop(palette, "width", 420.0)?;
    ui.set_prop(palette, "height", 200.0)?;
    ui.set_prop(palette, "monitor", 0i64)?;
    ui.set_prop(palette, "topmost", true)?;
    board(&mut ui, palette, "P", 2, (1, 1))?;
    ui.show(palette)?;

    aylis_platform_winit::run(ui, main)?;
    Ok(())
}
