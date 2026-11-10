//! O CodeEditor: um componente próprio (o Memo comum fica só caixa de texto), dobrar blocos e
//! vários cursores.

use aylis_controls::*;
use aylis_core::*;

fn with(key: Key, ctrl: bool, shift: bool, alt: bool) -> KeyEvent {
    KeyEvent { key, modifiers: Modifiers { ctrl, shift, alt, ..Default::default() }, ..Default::default() }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn setup(text: &str) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 800.0, 500.0));
    let e = ui.add(CodeEditor::new(text).with_syntax("rust"), form);
    ui.control_mut(e).unwrap().set_align(Align::Client);
    ui.layout();
    ui.set_focus(e);
    (ui, form, e)
}

fn memo(ui: &Ui, e: ComponentId) -> &Memo {
    ui.get::<Memo>(e).unwrap()
}

fn text(ui: &Ui, e: ComponentId) -> String {
    memo(ui, e).text.clone()
}

#[test]
fn code_editor_is_its_own_class_and_the_memo_stays_simple() {
    let (mut ui, _, e) = setup("x");
    assert_eq!(ui.class_name(e), Some("CodeEditor"));
    assert!(memo(&ui, e).code_editing, "o de dentro é o Memo com o que é de código ligado");
    assert_eq!(ui.get_prop(e, "minimap").unwrap(), PropValue::Bool(true));
    ui.set_prop(e, "folding", false).unwrap();
    assert_eq!(ui.get_prop(e, "folding").unwrap(), PropValue::Bool(false));
    // O Memo comum não mostra mais o que é de código.
    let names: Vec<&str> = Memo::default().props().iter().map(|p| p.name).collect();
    assert!(!names.contains(&"code_editing") && !names.contains(&"on_complete_request"), "{names:?}");
    let editor: Vec<&str> = CodeEditor::default().props().iter().map(|p| p.name).collect();
    assert!(editor.contains(&"minimap") && editor.contains(&"on_complete_request"), "{editor:?}");
}

const CODE: &str = "fn a() {\n    let x = 1;\n    let y = 2;\n}\nfn b() {\n    1\n}";

#[test]
fn folding_hides_the_block_and_follows_edits() {
    let (mut ui, form, e) = setup(CODE);
    assert!(memo(&ui, e).can_fold(0));
    assert!(!memo(&ui, e).can_fold(1));
    // O cursor dentro do bloco: dobrar leva ele para o fim da linha que abre.
    ui.get_mut::<Memo>(e).unwrap().go_to(3, 5);
    ui.key_down(form, with(Key::Char('['), true, true, false));
    assert_eq!(memo(&ui, e).folded_lines(), [0]);
    assert_eq!(memo(&ui, e).caret_position(), (0, 8));
    // Descendo, as linhas escondidas são puladas: vai direto para o "}".
    ui.key_down(form, key(Key::Down));
    assert_eq!(memo(&ui, e).caret_position().0, 3);
    // Uma linha nova em cima: a dobra acompanha a linha dela.
    ui.get_mut::<Memo>(e).unwrap().go_to(1, 1);
    ui.key_down(form, key(Key::Enter));
    assert_eq!(memo(&ui, e).folded_lines(), [1]);
    // Desdobrar na linha dobrada.
    ui.get_mut::<Memo>(e).unwrap().go_to(2, 1);
    ui.key_down(form, with(Key::Char(']'), true, true, false));
    assert!(memo(&ui, e).folded_lines().is_empty());
    // Dobrar tudo pega os blocos de fora; ir para uma linha escondida abre o bloco dela.
    ui.get_mut::<Memo>(e).unwrap().fold_all();
    assert_eq!(memo(&ui, e).folded_lines(), [1, 5]);
    ui.get_mut::<Memo>(e).unwrap().go_to_line(2);
    assert_eq!(memo(&ui, e).folded_lines(), [5]);
}

#[test]
fn several_cursors_type_delete_and_undo_together() {
    let (mut ui, form, e) = setup("ab\nab\nab");
    ui.get_mut::<Memo>(e).unwrap().go_to(1, 1);
    ui.key_down(form, with(Key::Down, true, false, true));
    ui.key_down(form, with(Key::Down, true, false, true));
    assert_eq!(memo(&ui, e).cursors().len(), 3);
    ui.text_input(form, "x");
    assert_eq!(text(&ui, e), "xab\nxab\nxab");
    ui.key_down(form, key(Key::Delete));
    assert_eq!(text(&ui, e), "xb\nxb\nxb");
    ui.key_down(form, key(Key::Backspace));
    assert_eq!(text(&ui, e), "b\nb\nb");
    ui.key_down(form, key(Key::End));
    ui.key_down(form, key(Key::Enter));
    assert_eq!(text(&ui, e), "b\n\nb\n\nb\n");
    // O Ctrl+Z desfaz a mudança de todos de uma vez.
    ui.key_down(form, with(Key::Char('z'), true, false, false));
    assert_eq!(text(&ui, e), "b\nb\nb");
    // Esc volta a um cursor só.
    ui.key_down(form, key(Key::Escape));
    assert_eq!(memo(&ui, e).cursors().len(), 1);
}

#[test]
fn ctrl_d_and_ctrl_shift_l_put_a_cursor_on_each_occurrence() {
    let (mut ui, form, e) = setup("foo bar foo baz foo");
    ui.get_mut::<Memo>(e).unwrap().go_to(1, 2);
    ui.key_down(form, with(Key::Char('d'), true, false, false));
    assert_eq!(memo(&ui, e).sel_text(), "foo");
    ui.key_down(form, with(Key::Char('d'), true, false, false));
    assert_eq!(memo(&ui, e).cursors().len(), 2);
    ui.text_input(form, "q");
    assert_eq!(text(&ui, e), "q bar q baz foo");

    let (mut ui, form, e) = setup("foo bar foo baz foo");
    ui.get_mut::<Memo>(e).unwrap().go_to(1, 2);
    ui.key_down(form, with(Key::Char('l'), true, true, false));
    assert_eq!(memo(&ui, e).cursors().len(), 3);
    ui.text_input(form, "z");
    assert_eq!(text(&ui, e), "z bar z baz z");
}

/// Ctrl+B no código: a linha ganha o primeiro número livre (começando pelo 1) e, em cima de
/// um marcador, perde o marcador; avisa em `on_bookmarks_change`.
#[test]
fn ctrl_b_toggles_the_next_numbered_bookmark() {
    let (mut ui, form, e) = setup("fn a() {}\nfn b() {}\nfn c() {}");
    let changes = std::rc::Rc::new(std::cell::Cell::new(0));
    let seen = changes.clone();
    ui.on(e, "on_bookmarks_change", move |_, _, _| seen.set(seen.get() + 1));
    ui.key_down(form, with(Key::Char('b'), true, false, false));
    ui.key_down(form, key(Key::Down));
    ui.key_down(form, with(Key::Char('b'), true, false, false));
    let marks = |ui: &Ui| memo(ui, e).bookmarks().iter().map(|b| (b.line, b.number)).collect::<Vec<_>>();
    assert_eq!(marks(&ui), vec![(0, Some(1)), (1, Some(2))]);
    ui.key_down(form, with(Key::Char('b'), true, false, false));
    assert_eq!(marks(&ui), vec![(0, Some(1))], "em cima de um marcador, tira");
    assert_eq!(changes.get(), 3);
    assert_eq!(text(&ui, e), "fn a() {}\nfn b() {}\nfn c() {}", "o texto não muda");
}

#[test]
fn the_minimap_gets_the_colors_down_to_the_end() {
    // Arquivo grande: o realce vem aos poucos, alguns ms por quadro. O minimapa guardava os
    // traços do primeiro quadro e ficava só com o começo colorido.
    // O realce aos poucos é o da gramática TextMate (o das IDEs); uma pequena basta.
    const GRAMMAR: &str = r##"{ "scopeName": "source.mini_map", "patterns": [
        { "match": "\\b(fn|let|pub)\\b", "name": "keyword.control.mini_map" },
        { "match": "\\b\\d+\\b", "name": "constant.numeric.mini_map" } ] }"##;
    aylis_controls::textmate::register_grammar("mini_map", GRAMMAR).unwrap();
    let code: Vec<String> = (0..2000).map(|i| format!("pub fn item_{i}() -> u32 {{ let valor = {i}; valor }}")).collect();
    let (mut ui, form, e) = setup(&code.join("\n"));
    ui.get_mut::<Memo>(e).unwrap().syntax = "mini_map".into();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let (w, h) = (800usize, 500usize);
    // Um traço colorido (não cinza) no quarto de baixo da faixa do minimapa.
    let colored_at_the_bottom = |pixels: &[u8]| {
        (h * 3 / 4..h - 4).any(|y| {
            (w - 90..w - 14).any(|x| {
                let p = &pixels[(y * w + x) * 4..(y * w + x) * 4 + 3];
                p.iter().max().unwrap() - p.iter().min().unwrap() > 40
            })
        })
    };
    let mut frames = 0;
    loop {
        let background = ui.theme().color("background");
        let pixels = renderer.render_rgba(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
        frames += 1;
        if colored_at_the_bottom(&pixels) {
            break;
        }
        assert!(frames < 400, "o fim do minimapa não ganhou as cores do realce");
    }
}

/// Ctrl com o mouse sobre um nome (07/11): o cursor vira a mãozinha (o Ctrl+clique leva à
/// declaração), também ao apertar o Ctrl com o mouse parado; soltar o Ctrl ou ir para um
/// espaço volta ao cursor de texto.
#[test]
fn ctrl_over_a_name_shows_the_hand_cursor() {
    let (mut ui, form, _) = setup("fn calcular() {}\n\n\n");
    let mouse = |x: f32, y: f32, ctrl: bool| PointerEvent { position: PointF::new(x, y), modifiers: Modifiers { ctrl, ..Default::default() }, ..Default::default() };
    // Acha um ponto sobre o nome, andando pela primeira linha com o Ctrl apertado.
    let over_name = (0..200).map(|i| i as f32 * 4.0).flat_map(|x| [6.0, 10.0, 14.0].map(|y| (x, y))).find(|&(x, y)| {
        ui.pointer_move(form, mouse(x, y, true));
        ui.cursor(form) == CursorShape::Pointer
    });
    let (x, y) = over_name.expect("a mãozinha aparece em algum ponto do nome");
    ui.pointer_move(form, mouse(x, y, false));
    assert_eq!(ui.cursor(form), CursorShape::Text, "sem Ctrl, o cursor de texto");
    ui.key_down(form, key(Key::Control));
    assert_eq!(ui.cursor(form), CursorShape::Pointer, "apertar o Ctrl parado já mostra a mãozinha");
    ui.key_up(form, key(Key::Control));
    assert_eq!(ui.cursor(form), CursorShape::Text, "soltar o Ctrl volta");
    ui.pointer_move(form, mouse(x, 120.0, true));
    assert_eq!(ui.cursor(form), CursorShape::Text, "numa linha vazia, nada a clicar");
}
