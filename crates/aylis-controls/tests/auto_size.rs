//! `auto_size`: a altura sai do conteúdo (o texto quebrado na largura, a soma dos filhos).

use aylis_controls::{Form, Label, Layout};
use aylis_core::*;

#[test]
fn labels_and_containers_take_the_height_of_their_content() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 300.0, 600.0));
    let _ = ui.set_prop(form, "padding", 0i64);
    let column = ui.add(Layout::default(), form);
    ui.control_mut(column).unwrap().set_align(Align::Top).set_auto_size(true).set_padding([0.0, 10.0, 0.0, 10.0]);
    let long = "uma dica comprida que precisa quebrar em várias linhas numa coluna estreita de trezentos pontos";
    let a = ui.add(Label::new("Título"), column);
    let b = ui.add(Label::new(long), column);
    for (l, wrap) in [(a, false), (b, true)] {
        ui.control_mut(l).unwrap().set_align(Align::Top).set_auto_size(true).set_margins([0.0, 6.0, 0.0, 0.0]);
        ui.set_prop(l, "wrap", wrap).unwrap();
    }
    ui.layout();
    let (ha, hb) = (ui.bounds(a).unwrap().height, ui.bounds(b).unwrap().height);
    assert!(ha > 5.0 && hb > ha * 2.0, "uma linha {ha}, várias {hb}");
    // O contêiner: padding + filhos + margens.
    assert_eq!(ui.bounds(column).unwrap().height, 20.0 + ha + hb + 12.0);
    // Mais largo, menos linhas.
    ui.control_mut(form).unwrap().set_width(900.0);
    ui.layout();
    assert!(ui.bounds(b).unwrap().height < hb);
    // Sem texto, sem altura (a dica vazia não ocupa lugar).
    ui.set_prop(b, "text", "").unwrap();
    ui.layout();
    assert_eq!(ui.bounds(b).unwrap().height, 0.0);
    // Um contêiner sem filhos fica só com o padding.
    let empty = ui.add(Layout::default(), form);
    ui.control_mut(empty).unwrap().set_align(Align::Top).set_auto_size(true);
    ui.layout();
    assert_eq!(ui.bounds(empty).unwrap().height, 0.0);
}

#[test]
fn hidden_branches_are_laid_out_only_when_shown() {
    // Uma tela escondida não custa no layout: o tamanho dela sai quando aparece.
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 300.0, 200.0));
    let _ = ui.set_prop(form, "padding", 0i64);
    let screen = ui.add(Layout::default(), form);
    ui.control_mut(screen).unwrap().set_align(Align::Contents).set_visible(false);
    let inner = ui.add(Label::new("texto"), screen);
    ui.control_mut(inner).unwrap().set_align(Align::Client);
    ui.layout();
    assert_ne!(ui.bounds(inner).unwrap().width, 300.0, "escondido: não foi recalculado");
    ui.set_prop(screen, "visible", true).unwrap();
    ui.update_layout();
    assert_eq!(ui.bounds(inner).unwrap().width, 300.0, "ao aparecer, ocupa a tela");
}

#[test]
fn hiding_a_screen_takes_the_focus_out_of_it() {
    // O campo com foco estava numa tela que foi escondida: o foco sai (com o on_exit), e
    // a digitação não vai para um controle fora da tela.
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 300.0, 200.0));
    let screen = ui.add(Layout::default(), form);
    let edit = ui.add(aylis_controls::Edit::new(""), screen);
    ui.show(form).unwrap();
    assert!(ui.set_focus(edit));
    let exited = std::rc::Rc::new(std::cell::Cell::new(false));
    let seen = exited.clone();
    ui.on(edit, "on_exit", move |_, _, _| seen.set(true));
    ui.set_prop(screen, "visible", false).unwrap();
    assert_eq!(ui.focused(form), None);
    assert!(exited.get());
}

#[test]
fn a_grid_and_a_flow_take_the_height_of_what_they_arrange() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 900.0, 600.0));
    let _ = ui.set_prop(form, "padding", 0i64);
    // Uma seção: o título e a grade de cartões, as duas com auto_size.
    let section = ui.add(Layout::default(), form);
    ui.control_mut(section).unwrap().set_align(Align::Top).set_auto_size(true).set_padding([0.0, 20.0, 0.0, 20.0]);
    let grid = ui.create("GridLayout", Some(section)).unwrap();
    ui.control_mut(grid).unwrap().set_align(Align::Top).set_auto_size(true);
    for (k, v) in [("columns", PropValue::Int(4)), ("gap", PropValue::Float(10.0)), ("row_height", PropValue::Float(100.0))] {
        ui.set_prop(grid, k, v).unwrap();
    }
    for _ in 0..8 {
        ui.create("Panel", Some(grid)).unwrap();
    }
    ui.layout();
    // 8 cartões em 4 colunas: 2 linhas de 100 e um vão de 10.
    assert_eq!(ui.bounds(grid).unwrap().height, 210.0);
    assert_eq!(ui.bounds(section).unwrap().height, 250.0, "a seção cresce com a grade");
    // Uma coluna só (a vista de celular): 8 linhas.
    ui.set_prop(grid, "columns", 1i64).unwrap();
    ui.layout();
    assert_eq!(ui.bounds(grid).unwrap().height, 8.0 * 100.0 + 7.0 * 10.0);
    // O fluxo: os filhos pelos tamanhos deles, quebrando na largura.
    let flow = ui.create("FlowLayout", Some(form)).unwrap();
    ui.control_mut(flow).unwrap().set_align(Align::Top).set_auto_size(true);
    for _ in 0..5 {
        let chip = ui.create("Panel", Some(flow)).unwrap();
        ui.control_mut(chip).unwrap().set_size(400.0, 40.0);
    }
    ui.layout();
    let h = ui.bounds(flow).unwrap().height;
    assert!((120.0..=160.0).contains(&h), "2 por linha em 900: 3 linhas de 40 e os vãos, veio {h}");
}
