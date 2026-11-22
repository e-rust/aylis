//! Animações dentro das linhas do `VirtualList` (decidido em 22/11): a linha reciclada recomeça
//! as animações, o item escolhido chega marcado sem animar, o `appear` vale uma vez por item, as
//! animações começam e param pelo `invoke` (`start`, `stop`) e o gatilho `hover_within` vale com
//! o mouse em qualquer um dos de dentro.

use aylis_controls::{Form, Label, Panel, VirtualList};
use aylis_core::*;

fn opacity(ui: &Ui, id: ComponentId) -> f64 {
    ui.get_prop(id, "opacity").unwrap().to_f64().unwrap()
}

/// Uma lista de 100 itens de 40 de altura com o modelo `pnlLinha` (um painel com um texto) e a
/// animação `animation` no painel, com o nome `anmLinha`.
fn setup(animation: PropertyAnimation, row_opacity: f32) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmLista", 300.0, 400.0));
    let list = ui.add(VirtualList::default(), form);
    ui.control_mut(list).unwrap().set_width(300.0).set_height(400.0);
    let row = ui.add(Panel::new(300.0, 40.0), list);
    ui.set_name(row, "pnlLinha").unwrap();
    ui.control_mut(row).unwrap().set_opacity(row_opacity);
    ui.add(Label::new(""), row);
    let anim = ui.add(FloatAnimation(animation), row);
    ui.set_name(anim, "anmLinha").unwrap();
    ui.set_prop(list, "item_height", 40.0).unwrap();
    ui.set_prop(list, "selection", "single").unwrap();
    ui.set_prop(list, "item_count", 100).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    VirtualList::refresh(&mut ui, list);
    ui.layout();
    (ui, form, list)
}

fn row_of(ui: &Ui, list: ComponentId, item: usize) -> ComponentId {
    VirtualList::item(ui, list, item).unwrap_or_else(|| panic!("o item {item} não está à vista"))
}

fn anim_of(ui: &Ui, list: ComponentId, item: usize) -> ComponentId {
    ui.find_in(row_of(ui, list, item), "anmLinha").unwrap()
}

fn scroll(ui: &mut Ui, list: ComponentId, item: usize) {
    VirtualList::scroll_to(ui, list, item);
    ui.layout();
}

#[test]
fn a_recycled_row_stops_the_animation_of_the_previous_item() {
    let animation = PropertyAnimation::float("opacity", 0.2).with_duration(1000.0).with_easing(Easing::Linear);
    let (mut ui, _form, list) = setup(animation, 1.0);
    ui.set_time(0.0);
    // Começa pelo `invoke`, como nas outras linguagens.
    let anim = anim_of(&ui, list, 0);
    ui.invoke(anim, "start", &[]).unwrap();
    ui.set_time(0.5);
    ui.tick();
    let row = row_of(&ui, list, 0);
    assert!((opacity(&ui, row) - 0.6).abs() < 0.01, "no meio: {}", opacity(&ui, row));
    // Rola para longe: a linha vai para outro item, a animação para e o valor volta.
    scroll(&mut ui, list, 50);
    assert!(VirtualList::item(&ui, list, 0).is_none());
    ui.set_time(0.75);
    ui.tick();
    assert_eq!(opacity(&ui, row), 1.0, "a linha reciclada não leva a animação do item anterior");
    ui.set_time(2.0);
    ui.tick();
    assert_eq!(opacity(&ui, row), 1.0);
    // O `stop` deixa o valor onde está.
    let anim = anim_of(&ui, list, 50);
    ui.invoke(anim, "start", &[]).unwrap();
    ui.set_time(2.5);
    ui.tick();
    ui.invoke(anim, "stop", &[]).unwrap();
    ui.set_time(3.0);
    ui.tick();
    let row = row_of(&ui, list, 50);
    assert!((opacity(&ui, row) - 0.6).abs() < 0.01, "parado no meio: {}", opacity(&ui, row));
    assert!(ui.invoke(anim, "rewind", &[]).is_err());
}

#[test]
fn the_selected_item_comes_back_checked_without_animating() {
    let animation = PropertyAnimation::float("opacity", 0.5).with_trigger(Trigger::Checked).with_duration(1000.0).with_easing(Easing::Linear);
    let (mut ui, _form, list) = setup(animation, 1.0);
    ui.set_time(0.0);
    VirtualList::select(&mut ui, list, 0);
    ui.layout();
    ui.set_time(0.5);
    ui.tick();
    assert!((opacity(&ui, row_of(&ui, list, 0)) - 0.75).abs() < 0.01, "escolher anima");
    ui.set_time(2.0);
    ui.tick();
    assert_eq!(opacity(&ui, row_of(&ui, list, 0)), 0.5);
    // Sai da tela e volta: chega marcado, já no fim, sem animar.
    scroll(&mut ui, list, 50);
    for i in 50..55 {
        assert_eq!(opacity(&ui, row_of(&ui, list, i)), 1.0, "o item {i} não está escolhido");
    }
    scroll(&mut ui, list, 0);
    let row = row_of(&ui, list, 0);
    assert!(ui.control(row).unwrap().state().checked);
    assert_eq!(opacity(&ui, row), 0.5, "o escolhido volta no fim da animação");
    ui.set_time(2.1);
    ui.tick();
    assert_eq!(opacity(&ui, row), 0.5);
    assert_eq!(opacity(&ui, row_of(&ui, list, 1)), 1.0);
}

#[test]
fn appear_runs_once_per_item_in_a_recycled_list() {
    let animation = PropertyAnimation::float("opacity", 1.0).with_trigger(Trigger::Appear).with_duration(1000.0).with_easing(Easing::Linear);
    let (mut ui, _form, list) = setup(animation, 0.0);
    ui.set_time(0.0);
    ui.tick();
    ui.set_time(0.5);
    ui.tick();
    assert!((opacity(&ui, row_of(&ui, list, 0)) - 0.5).abs() < 0.01, "os primeiros aparecem animando");
    ui.set_time(2.0);
    ui.tick();
    assert_eq!(opacity(&ui, row_of(&ui, list, 0)), 1.0);
    // Os itens que entram rolando, em linhas recicladas, também animam.
    scroll(&mut ui, list, 50);
    ui.set_time(3.0);
    ui.tick();
    ui.set_time(3.5);
    ui.tick();
    let fresh = opacity(&ui, row_of(&ui, list, 50));
    assert!((fresh - 0.5).abs() < 0.01, "o item 50 aparece animando: {fresh}");
    ui.set_time(5.0);
    ui.tick();
    // Voltando, os que já apareceram chegam prontos.
    scroll(&mut ui, list, 0);
    ui.set_time(5.1);
    ui.tick();
    for i in 0..5 {
        assert_eq!(opacity(&ui, row_of(&ui, list, i)), 1.0, "o item {i} já apareceu: não anima de novo");
    }
    // O `refresh` (os dados mudaram) vale de novo para todos.
    VirtualList::refresh(&mut ui, list);
    ui.layout();
    ui.set_time(6.0);
    ui.tick();
    ui.set_time(6.5);
    ui.tick();
    let again = opacity(&ui, row_of(&ui, list, 0));
    assert!((again - 0.5).abs() < 0.01, "depois do refresh, aparece de novo: {again}");
}

#[test]
fn hover_within_runs_with_the_mouse_over_a_child() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmCartao", 300.0, 200.0));
    let card = ui.add(Panel::new(200.0, 100.0), form);
    ui.control_mut(card).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    // Um filho que recebe o mouse (o texto deixa passar para o pai).
    let child = ui.add(Panel::new(150.0, 30.0), card);
    ui.control_mut(child).unwrap().set_rect(10.0, 10.0, 150.0, 30.0);
    ui.add(FloatAnimation(PropertyAnimation::float("opacity", 0.5).with_trigger(Trigger::HoverWithin).with_duration(0.0)), card);
    let only_hover = ui.add(FloatAnimation(PropertyAnimation::float("rotation", 10.0).with_trigger(Trigger::Hover).with_duration(0.0)), card);
    ui.show(form).unwrap();
    ui.layout();
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.set_time(0.0);
    ui.pointer_move(form, at(20.0, 20.0));
    ui.set_time(0.1);
    ui.tick();
    assert_eq!(opacity(&ui, card), 0.5, "o mouse no filho vale para o cartão");
    assert_eq!(ui.get_prop(card, "rotation").unwrap().to_f64().unwrap(), 0.0, "o `hover` continua só o de baixo do mouse");
    ui.pointer_move(form, at(250.0, 150.0));
    ui.set_time(0.2);
    ui.tick();
    assert_eq!(opacity(&ui, card), 1.0, "saiu: volta");
    let _ = only_hover;
}

#[test]
fn the_list_leaves_the_row_translate_to_the_animation_without_swipe_actions() {
    // A escolhida desliza (`translate_x`): rolar não a põe de volta no lugar (sem `swipe_actions`,
    // o deslocamento da linha é do app).
    let animation = PropertyAnimation::float("translate_x", 8.0).with_trigger(Trigger::Checked).with_duration(0.0);
    let (mut ui, _form, list) = setup(animation, 1.0);
    ui.set_time(0.0);
    VirtualList::select(&mut ui, list, 2);
    ui.layout();
    ui.set_time(0.1);
    ui.tick();
    let shift = |ui: &Ui, i: usize| ui.get_prop(row_of(ui, list, i), "translate_x").unwrap().to_f64().unwrap();
    assert_eq!(shift(&ui, 2), 8.0);
    scroll(&mut ui, list, 50);
    scroll(&mut ui, list, 0);
    ui.set_time(0.2);
    ui.tick();
    assert_eq!(shift(&ui, 2), 8.0, "voltou deslizada");
    assert_eq!(shift(&ui, 1), 0.0);
}

#[test]
fn on_bind_runs_with_the_list_in_place_when_a_row_changes_inside_the_scroll() {
    // Cada chamada a um componente (`with_component`) entrega no fim os eventos agendados; no
    // meio do `scroll_to` da lista (com as linhas mudando de item e as animações recomeçando),
    // isso entregava os `on_bind` com a lista fora do `Ui`, e o handler tipado não achava quem
    // disparou: a linha ficava com o item velho (achado no ListaPlayer, 22/11). Os eventos de
    // quem está no meio de um método agora esperam ele voltar.
    let animation = PropertyAnimation::float("translate_x", 8.0).with_trigger(Trigger::Checked).with_duration(0.0);
    let (mut ui, _form, list) = setup(animation, 1.0);
    ui.set_prop(list, "swipe_actions", "Excluir").unwrap();
    ui.set_time(0.0);
    VirtualList::select(&mut ui, list, 2);
    ui.layout();
    ui.set_time(0.1);
    ui.tick();
    let (orphans, binds) = (std::rc::Rc::new(std::cell::Cell::new(0)), std::rc::Rc::new(std::cell::Cell::new(0)));
    let (o, b) = (orphans.clone(), binds.clone());
    ui.on(list, "on_bind", move |ui, sender, args| {
        b.set(b.get() + 1);
        if ui.class_name(sender).is_none() {
            o.set(o.get() + 1);
            return;
        }
        // Como o player: o handler mexe na animação da linha (um método, outro `with_component`).
        if let EventArgs::Value(PropValue::Int(i)) = args {
            if let Some(anim) = VirtualList::item(ui, sender, *i as usize).and_then(|r| ui.find_in(r, "anmLinha")) {
                let _ = ui.invoke(anim, "stop", &[]);
            }
        }
    });
    ui.invoke(list, "scroll_to", &[PropValue::Int(50)]).unwrap();
    ui.layout();
    ui.invoke(list, "scroll_to", &[PropValue::Int(0)]).unwrap();
    ui.layout();
    assert!(binds.get() > 0);
    assert_eq!(orphans.get(), 0, "{} de {} on_bind com a lista fora do Ui", orphans.get(), binds.get());
}
