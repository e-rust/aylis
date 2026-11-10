//! Lista infinita com modelo de item (VirtualList, como o RecyclerView): cada linha é uma
//! cópia do cartão desenhado como primeiro filho; ao rolar, as linhas são reaproveitadas e
//! perto do fim chegam mais 50 itens.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example infinite_list --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — lista infinita", 420.0, 640.0));
    let info = ui.add(Label::new("50 itens"), form);
    ui.control_mut(info).unwrap().set_align(Align::Top).set_height(32.0).set_margins([12.0, 8.0, 12.0, 0.0]);
    let list = ui.add(VirtualList::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    // O modelo: um cartão com avatar, título e detalhe.
    let card = ui.add(Panel::new(400.0, 72.0), list);
    ui.control_mut(card).unwrap().set_margins([8.0, 4.0, 8.0, 4.0]).set_padding(10.0);
    let _ = ui.set_prop(card, "style", "card");
    let avatar = ui.add(Avatar::default(), card);
    ui.set_name(avatar, "imgAvatar")?;
    ui.control_mut(avatar).unwrap().set_align(Align::Left).set_width(44.0);
    let texts = ui.add(Layout::default(), card);
    ui.control_mut(texts).unwrap().set_align(Align::Client).set_margins([12.0, 2.0, 0.0, 0.0]);
    let title = ui.add(Label::new(""), texts);
    ui.set_name(title, "lblTitulo")?;
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(26.0);
    let detail = ui.add(Label::new(""), texts);
    ui.set_name(detail, "lblDetalhe")?;
    ui.control_mut(detail).unwrap().set_align(Align::Top).set_height(22.0);

    ui.set_prop(list, "item_height", 80.0)?;
    ui.set_prop(list, "item_count", 50i64)?;
    ui.on(list, "on_bind", |ui, list, args| {
        let EventArgs::Value(PropValue::Int(i)) = args else { return };
        let Some(row) = VirtualList::item(ui, list, *i as usize) else { return };
        let set = |ui: &mut Ui, name: &str, prop: &str, value: String| {
            if let Some(c) = ui.find_in(row, name) {
                let _ = ui.set_prop(c, prop, value);
            }
        };
        set(ui, "lblTitulo", "text", format!("Pedido #{}", 1000 + i));
        set(ui, "lblDetalhe", "text", format!("{} itens · R$ {},{:02}", i % 7 + 1, (i * 37) % 500, i % 100));
        set(ui, "imgAvatar", "display_name", format!("Cliente {}", (b'A' + (*i % 26) as u8) as char));
    });
    ui.on(list, "on_load_more", move |ui, list, args| {
        let EventArgs::Value(PropValue::Int(n)) = args else { return };
        let _ = ui.set_prop(list, "item_count", n + 50);
        let _ = ui.set_prop(info, "text", format!("{} itens", n + 50));
    });
    aylis_platform_winit::run(ui, form)
}
