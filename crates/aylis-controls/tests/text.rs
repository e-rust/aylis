//! Correção e sugestão de texto nos campos (`spellcheck`, `autocomplete`), com um motor
//! próprio (`WordList`) no lugar do nativo.

use aylis_controls::*;
use aylis_core::textassist::WordList;
use aylis_core::*;

fn setup(words: &[&str]) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text_assist = Some(Box::new(WordList::new(words.iter().copied())));
    let f = ui.add_root(Form::new("teste", 500.0, 300.0));
    ui.show(f).unwrap();
    let e = ui.add(Edit::new(""), f);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 400.0, 32.0);
    ui.layout();
    ui.set_focus(e);
    (ui, f, e)
}

fn text(ui: &Ui, e: ComponentId) -> String {
    ui.get::<Edit>(e).unwrap().text.clone()
}

#[test]
fn autocomplete_suggests_the_rest_and_tab_accepts() {
    let (mut ui, f, e) = setup(&["computador", "compra", "casa"]);
    ui.set_prop(e, "autocomplete", true).unwrap();
    ui.text_input(f, "Comp");
    assert_eq!(ui.get::<Edit>(e).unwrap().completion(), "ra", "o resto aparece depois do cursor");
    // "Comp" tem duas: a primeira em ordem ("compra"), com a caixa do que foi digitado.
    ui.key_down(f, KeyEvent { key: Key::Tab, ..Default::default() });
    assert_eq!(text(&ui, e), "Compra");
    // Esc dispensa; outra tecla segue normal.
    ui.text_input(f, " cas");
    ui.key_down(f, KeyEvent { key: Key::Escape, ..Default::default() });
    ui.key_down(f, KeyEvent { key: Key::Tab, ..Default::default() });
    assert_eq!(text(&ui, e), "Compra cas", "depois do Esc o Tab não completa");
}

#[test]
fn spellcheck_marks_words_and_the_menu_replaces() {
    let (mut ui, f, e) = setup(&["olá", "mundo", "bonito"]);
    ui.set_prop(e, "spellcheck", true).unwrap();
    ui.text_input(f, "olá mudno");
    // A palavra encostada no cursor ainda está sendo digitada: não marca.
    assert_eq!(misspelled(&mut ui, e), 0);
    ui.text_input(f, " ");
    assert_eq!(misspelled(&mut ui, e), 1, "terminou a palavra: marcada");
    // Clique direito em cima de "mudno": o menu de correções; escolher troca a palavra.
    let x = 10.0 + 8.0 + 6.0 * 7.7;
    ui.pointer_down(f, PointerEvent { position: PointF::new(x, 26.0), button: PointerButton::Secondary, click_count: 1, ..Default::default() });
    let item = ui.components().find(|&c| ui.get::<MenuItem>(c).is_some_and(|m| m.text == "mundo")).expect("a sugestão no menu");
    ui.click(item);
    ui.process_events();
    assert_eq!(text(&ui, e), "olá mundo ");
}

/// Quantas palavras o Edit marcou (o sublinhado).
fn misspelled(ui: &mut Ui, e: ComponentId) -> usize {
    ui.get::<Edit>(e).unwrap().misspelled().len()
}

#[test]
fn memo_marks_words_on_any_line_and_completes_mid_text() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text_assist = Some(Box::new(WordList::new(["primeira", "linha", "segunda", "computador"])));
    let f = ui.add_root(Form::new("teste", 500.0, 300.0));
    ui.show(f).unwrap();
    let m = ui.add(Memo::new(""), f);
    ui.control_mut(m).unwrap().set_rect(10.0, 10.0, 400.0, 200.0);
    ui.set_prop(m, "spellcheck", true).unwrap();
    ui.set_prop(m, "autocomplete", true).unwrap();
    ui.layout();
    ui.set_focus(m);
    ui.text_input(f, "primeira linha\nsegunda lnha ");
    let memo = ui.get::<Memo>(m).unwrap();
    let marked: Vec<String> = memo.misspelled().iter().map(|&(a, b)| memo.text.chars().skip(a).take(b - a).collect()).collect();
    assert_eq!(marked, ["lnha"], "a palavra errada da segunda linha");
    // Completa também antes de uma quebra de linha (no fim da palavra).
    ui.text_input(f, "compu");
    assert_eq!(ui.get::<Memo>(m).unwrap().completion(), "tador");
    ui.key_down(f, KeyEvent { key: Key::Tab, ..Default::default() });
    assert!(ui.get::<Memo>(m).unwrap().text.ends_with("computador"));
}

#[test]
fn selection_and_caret_by_property() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 500.0, 300.0));
    ui.show(f).unwrap();
    let e = ui.add(Edit::new("olá mundo"), f);
    let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    for ev in ["on_change", "on_caret_move"] {
        let l = log.clone();
        ui.on(e, ev, move |_, _, _| l.borrow_mut().push(ev));
    }
    // SelStart/SelLength/SelText.
    ui.set_prop(e, "sel_start", 4_i64).unwrap();
    ui.set_prop(e, "sel_length", 5_i64).unwrap();
    assert_eq!(ui.get_prop(e, "sel_text").unwrap(), PropValue::String("mundo".into()));
    ui.set_prop(e, "read_only", true).unwrap();
    ui.set_prop(e, "sel_text", "aylis").unwrap();
    ui.process_events();
    assert_eq!(text(&ui, e), "olá aylis", "o código troca a seleção mesmo com read_only");
    assert!(log.borrow().contains(&"on_change") && log.borrow().contains(&"on_caret_move"), "{:?}", log.borrow());
    assert_eq!(ui.get_prop(e, "sel_start").unwrap(), PropValue::Int(9), "o cursor depois do texto inserido");
    // Só em execução: não aparecem como propriedades salvas.
    assert!(ui.prop_info(e, "sel_text").unwrap().runtime);
    assert!(!ui.prop_info(e, "caret_style").unwrap().runtime);
    // A aparência do cursor.
    ui.set_prop(e, "caret_style", "block").unwrap();
    ui.set_prop(e, "caret_blink", 0_i64).unwrap();
    ui.set_prop(e, "caret_color", "#ff0000").unwrap();
    let edit = ui.get::<Edit>(e).unwrap();
    assert_eq!((edit.caret_style, edit.caret_blink), (CaretStyle::Block, 0));

    // Memo: linha e coluna do cursor.
    let m = ui.add(Memo::new("a\nbc\ndef"), f);
    ui.set_prop(m, "caret_line", 3_i64).unwrap();
    ui.set_prop(m, "caret_column", 2_i64).unwrap();
    assert_eq!(ui.get_prop(m, "sel_start").unwrap(), PropValue::Int(6));
    assert_eq!((ui.get_prop(m, "caret_line").unwrap(), ui.get_prop(m, "caret_column").unwrap()), (PropValue::Int(3), PropValue::Int(2)));
    ui.set_prop(m, "sel_length", 2_i64).unwrap();
    assert_eq!(ui.get_prop(m, "sel_text").unwrap(), PropValue::String("ef".into()));
}

#[test]
fn text_set_by_code_is_checked_right_away() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text_assist = Some(Box::new(WordList::new(["bom", "dia"])));
    let f = ui.add_root(Form::new("teste", 500.0, 300.0));
    let e = ui.add(Edit::new(""), f);
    ui.set_prop(e, "spellcheck", true).unwrap();
    ui.set_prop(e, "text", "bom dai").unwrap();
    assert_eq!(ui.get::<Edit>(e).unwrap().misspelled(), &[(4, 7)], "sem digitar: já marcado");
    let m = ui.add(Memo::new(""), f);
    ui.set_prop(m, "spellcheck", true).unwrap();
    ui.set_prop(m, "text", "bom\ndai").unwrap();
    assert_eq!(ui.get::<Memo>(m).unwrap().misspelled(), &[(4, 7)]);
}
