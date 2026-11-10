//! Pequenos construtores de interface usados pelos painéis e diálogos da IDE.

use aylis_controls::{Button, Label, Layout, Rectangle};
use aylis_core::{Align, Color, ComponentId, CursorShape, PropertyAnimation, Trigger, Ui};

/// Label alinhado (`size` é a altura em top/bottom, a largura em left/right).
pub fn label(ui: &mut Ui, parent: ComponentId, text: &str, align: Align, size: f32) -> ComponentId {
    let l = ui.add(Label::new(text), parent);
    let cb = ui.control_mut(l).unwrap();
    cb.set_align(align);
    if matches!(align, Align::Top | Align::Bottom) {
        cb.set_height(size);
    } else if matches!(align, Align::Left | Align::Right) {
        cb.set_width(size);
    }
    l
}

/// Label pequeno e apagado (título de seção).
pub fn caption(ui: &mut Ui, parent: ComponentId, text: &str) -> ComponentId {
    let l = label(ui, parent, text, Align::Top, 24.0);
    ui.control_mut(l).unwrap().set_margins([4.0, 6.0, 0.0, 0.0]);
    muted(ui, l);
    let _ = ui.set_prop(l, "font_size", 11);
    l
}

pub fn muted(ui: &mut Ui, id: ComponentId) {
    let color = ui.theme().color("text_muted");
    let _ = ui.set_prop(id, "color", color);
}

/// Botão alinhado à esquerda (barra de ferramentas).
pub fn tool(ui: &mut Ui, parent: ComponentId, text: &str, width: f32, style: &str) -> ComponentId {
    let b = ui.add(Button::new(text), parent);
    ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 4.0, 0.0]).set_style(style);
    b
}

/// Barra horizontal (para botões).
pub fn bar(ui: &mut Ui, parent: ComponentId, align: Align, height: f32) -> ComponentId {
    let b = ui.add(Layout::default(), parent);
    ui.control_mut(b).unwrap().set_align(align).set_height(height);
    b
}

/// Uma linha clicável (retângulo + texto), com destaque ao passar o mouse.
pub fn row(ui: &mut Ui, parent: ComponentId, text: &str, indent: f32, selected: bool, height: f32) -> ComponentId {
    let r = ui.add(Rectangle::new(0.0, height), parent);
    ui.control_mut(r).unwrap().set_align(Align::Top).set_margins([0.0, 0.0, 0.0, 1.0]).set_cursor(CursorShape::Pointer);
    let _ = ui.set_prop(r, "radius", 5);
    let fill = if selected { ui.theme().color("primary").with_opacity(0.28) } else { Color::TRANSPARENT };
    let _ = ui.set_prop(r, "fill", fill);
    if !selected {
        let hover = ui.theme().color("text").with_opacity(0.07);
        ui.add(PropertyAnimation::color("fill", hover).with_trigger(Trigger::Hover).with_duration(100.0), r);
    }
    let l = ui.add(Label::new(text), r);
    ui.control_mut(l).unwrap().set_align(Align::Client).set_margins([10.0 + indent, 0.0, 6.0, 0.0]);
    r
}

/// O título de um grupo numa página de configuração (as Opções do projeto): 13 pontos, em
/// negrito, como os grupos do Object Inspector. A [`caption`] (11, apagada) fica para os
/// painéis da IDE; nas páginas, ela ficava pequena demais (pedido do autor em 07/11).
/// O texto até o primeiro " (" ou " · " é o título (em negrito); o resto, a explicação, vem ao
/// lado, apagada (o negrito na frase inteira pesava e cortava as longas).
pub fn section(ui: &mut Ui, parent: ComponentId, text: &str) -> ComponentId {
    // Traduzido inteiro antes de dividir (a tradução é da frase; os pedaços sozinhos não têm).
    let text = ui.tr(text);
    let text = text.as_str();
    let line = ui.add(Layout::default(), parent);
    ui.control_mut(line).unwrap().set_align(Align::Top).set_height(28.0).set_margins([2.0, 10.0, 0.0, 2.0]);
    let cut = [" (", " · "].iter().filter_map(|s| text.find(s)).min();
    let (title, rest) = match cut {
        Some(i) => (&text[..i], text[i..].trim_start_matches(" · ").trim()),
        None => (text, ""),
    };
    // O título com a largura do próprio texto (na fonte do tema, em negrito), e a explicação logo
    // depois dele.
    let mut style = aylis_core::TextStyle::default();
    style.font = ui.theme().fonts.get("default").cloned().unwrap_or_default();
    style.font.size = 13.0;
    style.font.weight = aylis_core::FontWeight::Bold;
    let width = ui.measure_text(title, &style).width.ceil() + 4.0;
    let t = ui.add(Label::new(title), line);
    ui.control_mut(t).unwrap().set_align(Align::Left).set_width(width);
    let _ = ui.set_prop(t, "font_size", 13);
    let _ = ui.set_prop(t, "bold", true);
    if !rest.is_empty() {
        let r = ui.add(Label::new(rest), line);
        ui.control_mut(r).unwrap().set_align(Align::Client).set_margins([8.0, 0.0, 0.0, 0.0]);
        let _ = ui.set_prop(r, "font_size", 12);
        muted(ui, r);
    }
    line
}

/// Uma lista que rola numa moldura (a borda do tema, cantos arredondados), ocupando o resto de
/// `parent`; devolve a caixa de rolagem, onde vão as linhas. Sem a moldura, a barra de rolagem
/// parecia solta na tela (pedido do autor em 07/11, nas Opções do projeto). O recuo à direita
/// deixa a barra fora das linhas.
pub fn framed_list(ui: &mut Ui, parent: ComponentId) -> ComponentId {
    let frame = ui.add(Rectangle::new(0.0, 0.0), parent);
    ui.control_mut(frame).unwrap().set_align(Align::Client).set_padding([8.0, 8.0, 4.0, 8.0]).set_margins([0.0, 6.0, 0.0, 0.0]);
    let border = ui.theme().color("border");
    let _ = ui.set_prop(frame, "stroke", border);
    let _ = ui.set_prop(frame, "stroke_width", 1.0);
    let _ = ui.set_prop(frame, "radius", 6.0);
    let _ = ui.set_prop(frame, "fill", Color::TRANSPARENT);
    let list = ui.add(aylis_controls::ScrollBox::default(), frame);
    ui.control_mut(list).unwrap().set_align(Align::Client).set_padding([0.0, 0.0, 16.0, 0.0]);
    list
}

/// Marca ou desmarca uma linha feita por [`row`] (o destaque e o realce ao passar o mouse),
/// sem refazê-la.
pub fn set_row_selected(ui: &mut Ui, r: ComponentId, selected: bool) {
    let fill = if selected { ui.theme().color("primary").with_opacity(0.28) } else { Color::TRANSPARENT };
    let _ = ui.set_prop(r, "fill", fill);
    let hover = ui.children(r).iter().copied().find(|&c| ui.get::<PropertyAnimation>(c).is_some());
    match (selected, hover) {
        (true, Some(h)) => {
            ui.free(h);
        }
        (false, None) => {
            let color = ui.theme().color("text").with_opacity(0.07);
            ui.add(PropertyAnimation::color("fill", color).with_trigger(Trigger::Hover).with_duration(100.0), r);
        }
        _ => {}
    }
}

/// Remove todos os filhos.
pub fn clear(ui: &mut Ui, id: ComponentId) {
    let children = ui.children(id).to_vec();
    ui.free_many(&children);
}
