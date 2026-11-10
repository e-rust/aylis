//! Configurações de texto (como o `TextSettings` do FMX): alinhamento, quebra
//! automática e reticências por caractere, por palavra ou nenhuma.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example text [-- --dark]
//! ```

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;

const LONG: &str = "Entregar o pedido pela manhã e conferir o CEP antes de sair";

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn caption(ui: &mut Ui, parent: ComponentId, text: &str, x: f32, y: f32) {
    let l = ui.add(Label::new(text), parent);
    put(ui, l, x, y, 260.0, 20.0);
    let muted = ui.theme().color("text_muted");
    let _ = ui.set_prop(l, "color", muted);
    let _ = ui.set_prop(l, "font_size", 12);
}

fn enum_prop(ui: &mut Ui, id: ComponentId, name: &str, value: &str) {
    let _ = ui.set_prop(id, name, PropValue::Enum(value.into()));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dark = std::env::args().any(|a| a == "--dark");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    ui.set_theme(if dark { Theme::dark() } else { Theme::light() });
    let form = ui.add_root(Form::new("aylis — texto", 860.0, 520.0));
    ui.set_name(form, "frmTexto")?;

    // ----- Edit: reticências e alinhamento -----
    let col = 20.0;
    for (i, (title, trimming)) in [("Edit · trimming = character", "character"), ("Edit · trimming = word", "word"), ("Edit · trimming = none", "none")]
        .into_iter()
        .enumerate()
    {
        let y = 16.0 + i as f32 * 56.0;
        caption(&mut ui, form, title, col, y);
        let e = ui.add(Edit::new(LONG), form);
        put(&mut ui, e, col, y + 20.0, 240.0, 30.0);
        enum_prop(&mut ui, e, "trimming", trimming);
    }
    for (i, align) in ["leading", "center", "trailing"].into_iter().enumerate() {
        let y = 184.0 + i as f32 * 56.0;
        caption(&mut ui, form, &format!("Edit · h_align = {align}"), col, y);
        let e = ui.add(Edit::new("R$ 1.234,56"), form);
        put(&mut ui, e, col, y + 20.0, 240.0, 30.0);
        enum_prop(&mut ui, e, "h_align", align);
    }
    caption(&mut ui, form, "Edit · wrap (alto, fora da edição)", col, 352.0);
    let e = ui.add(Edit::new(LONG), form);
    put(&mut ui, e, col, 372.0, 240.0, 56.0);
    let _ = ui.set_prop(e, "wrap", true);
    enum_prop(&mut ui, e, "v_align", "leading");

    // ----- Memo: quebra e alinhamento -----
    let col = 300.0;
    let text = format!("{LONG}.\nSegunda linha curta.\n{LONG}, sem falta.");
    caption(&mut ui, form, "Memo · wrap = true", col, 16.0);
    let m = ui.add(Memo::new(text.clone()), form);
    put(&mut ui, m, col, 36.0, 250.0, 150.0);
    let _ = ui.set_prop(m, "wrap", true);
    caption(&mut ui, form, "Memo · wrap = true, h_align = center", col, 200.0);
    let m = ui.add(Memo::new(text.clone()), form);
    put(&mut ui, m, col, 220.0, 250.0, 150.0);
    let _ = ui.set_prop(m, "wrap", true);
    enum_prop(&mut ui, m, "h_align", "center");
    caption(&mut ui, form, "Memo · sem wrap, trimming = word", col, 384.0);
    let m = ui.add(Memo::new(text), form);
    put(&mut ui, m, col, 404.0, 250.0, 90.0);
    enum_prop(&mut ui, m, "trimming", "word");

    // ----- Label e ComboBox -----
    let col = 590.0;
    caption(&mut ui, form, "Label · wrap + trimming = word (3 linhas)", col, 16.0);
    let l = ui.add(Label::new(format!("{LONG}. {LONG}.")), form);
    put(&mut ui, l, col, 36.0, 250.0, 60.0);
    let _ = ui.set_prop(l, "wrap", true);
    enum_prop(&mut ui, l, "trimming", "word");
    enum_prop(&mut ui, l, "v_align", "leading");
    caption(&mut ui, form, "Label · trimming = character", col, 110.0);
    let l = ui.add(Label::new(LONG), form);
    put(&mut ui, l, col, 130.0, 250.0, 22.0);
    caption(&mut ui, form, "ComboBox · h_align = center", col, 170.0);
    let c = ui.add(ComboBox::default(), form);
    put(&mut ui, c, col, 190.0, 250.0, 30.0);
    let _ = ui.set_prop(c, "items", "Pix;Cartão de crédito em várias parcelas sem juros;Dinheiro");
    let _ = ui.set_prop(c, "item_index", 1i64);
    enum_prop(&mut ui, c, "trimming", "word");
    caption(&mut ui, form, "NumberBox · h_align = trailing", col, 236.0);
    let n = ui.add(NumberBox::new(1234.5, 0.0, 99999.0), form);
    put(&mut ui, n, col, 256.0, 250.0, 30.0);
    enum_prop(&mut ui, n, "h_align", "trailing");

    aylis_platform_winit::run(ui, form)
}
