//! O editor de texto rico (`RichEditor`, o `QEditor` do Quasar): digitar, formatar pela
//! barra e pelo teclado e ler o `html`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Canvas que só mede (8 por caractere).
#[derive(Default)]
struct Probe {
    texts: Vec<String>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * 8.0, 16.0)
    }
    fn fill_path(&mut self, _: &Path, _: &Brush) {}
    fn stroke_path(&mut self, _: &Path, _: &Stroke) {}
    fn draw_shadow(&mut self, _: RectF, _: f32, _: &Shadow) {}
}

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 800.0, 600.0));
    ui.show(f).unwrap();
    let e = ui.add(RichEditor::default(), f);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 600.0, 300.0);
    ui.layout();
    ui.set_focus(e);
    (ui, f, e)
}

fn html(ui: &Ui, e: ComponentId) -> String {
    ui.get::<RichEditor>(e).unwrap().html.clone()
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn ctrl(c: char) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() }
}

fn shift(k: Key) -> KeyEvent {
    KeyEvent { key: k, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() }
}

fn click(ui: &mut Ui, f: ComponentId, x: f32, y: f32) {
    let event = PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, click_count: 1, ..Default::default() };
    ui.pointer_down(f, event.clone());
    ui.pointer_up(f, event);
}

#[test]
fn typing_then_ctrl_b_on_the_selection_makes_it_bold() {
    let (mut ui, f, e) = setup();
    let changes = Rc::new(RefCell::new(Vec::new()));
    let c = changes.clone();
    ui.on(e, "on_change", move |_, _, args| {
        if let EventArgs::Value(PropValue::String(s)) = args {
            c.borrow_mut().push(s.clone());
        }
    });
    ui.text_input(f, "Olá mundo");
    assert_eq!(html(&ui, e), "Olá mundo", "um parágrafo comum sai sem o <p>");
    for _ in 0..5 {
        ui.key_down(f, shift(Key::Left));
    }
    ui.key_down(f, ctrl('b'));
    assert_eq!(html(&ui, e), "Olá <b>mundo</b>");
    assert_eq!(changes.borrow().last().map(String::as_str), Some("Olá <b>mundo</b>"), "on_change leva o html");
    // De novo: tira o negrito.
    ui.key_down(f, ctrl('b'));
    assert_eq!(html(&ui, e), "Olá mundo");
    // Ctrl+Z volta o negrito; Ctrl+Y tira de novo.
    ui.key_down(f, ctrl('z'));
    assert_eq!(html(&ui, e), "Olá <b>mundo</b>");
    ui.key_down(f, ctrl('y'));
    assert_eq!(html(&ui, e), "Olá mundo");
}

#[test]
fn toolbar_button_without_selection_applies_to_what_is_typed_next() {
    let (mut ui, f, e) = setup();
    ui.layout();
    ui.paint(f, &mut Probe::default());
    ui.text_input(f, "a ");
    // O primeiro botão da barra é o negrito (no canto de cima, à esquerda).
    click(&mut ui, f, 10.0 + 4.0 + 15.0, 10.0 + 3.0 + 15.0);
    ui.text_input(f, "forte");
    // O itálico é o segundo.
    click(&mut ui, f, 10.0 + 4.0 + 32.0 + 15.0, 10.0 + 3.0 + 15.0);
    ui.text_input(f, "!");
    assert_eq!(html(&ui, e), "a <b>forte</b><b><i>!</i></b>");
    // Digitar seguido é um desfazer só.
    ui.key_down(f, ctrl('z'));
    assert_eq!(html(&ui, e), "a <b>forte</b>");
}

#[test]
fn headings_lists_and_alignment() {
    let (mut ui, f, e) = setup();
    ui.text_input(f, "Título");
    ui.get_mut::<RichEditor>(e).unwrap().command("h1");
    ui.key_down(f, key(Key::Enter));
    ui.text_input(f, "um");
    ui.get_mut::<RichEditor>(e).unwrap().command("unordered");
    ui.key_down(f, key(Key::Enter));
    ui.text_input(f, "dois");
    assert_eq!(html(&ui, e), "<h1>Título</h1><ul><li>um</li><li>dois</li></ul>", "depois do título vem um parágrafo comum");
    // Enter num item vazio sai da lista.
    ui.key_down(f, key(Key::Enter));
    ui.key_down(f, key(Key::Enter));
    ui.text_input(f, "fim");
    ui.get_mut::<RichEditor>(e).unwrap().command("center");
    assert_eq!(html(&ui, e), "<h1>Título</h1><ul><li>um</li><li>dois</li></ul><p style=\"text-align:center\">fim</p>");
    // Lista numerada, desligando com o mesmo botão.
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    editor.command("ordered");
    assert!(editor.html.ends_with("<ol><li style=\"text-align:center\">fim</li></ol>"));
    editor.command("ordered");
    assert!(editor.html.ends_with("<p style=\"text-align:center\">fim</p>"));
}

#[test]
fn html_is_read_back_and_links_and_clear_format_work() {
    let (mut ui, f, e) = setup();
    let source = "<h2>Notas</h2><p>Veja <a href=\"https://quasar.dev\">o site</a> &amp; <s>isso</s></p><ol><li>a</li><li><u>b</u></li></ol>";
    ui.set_prop(e, "html", source.to_string()).unwrap();
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    assert_eq!(editor.plain_text(), "Notas\nVeja o site & isso\na\nb");
    // Escrito de volta (negrito em tudo e tirado de novo refaz o html), sai igual.
    editor.select_all();
    editor.command("bold");
    editor.command("bold");
    assert_eq!(editor.html, source);
    // O que o navegador gera também entra (strong, em, div, br, entidades).
    ui.set_prop(e, "html", "<div><strong>x</strong>&nbsp;<em>y</em></div><div>z<br>w</div>".to_string()).unwrap();
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    assert_eq!(editor.plain_text(), "x y\nz\nw");
    assert_eq!(editor.html, "<div><strong>x</strong>&nbsp;<em>y</em></div><div>z<br>w</div>", "o html posto fica como veio até a primeira edição");
    // Link na seleção; o botão do link num link tira.
    editor.select(2, 3);
    assert!(editor.set_link("https://aylis.dev"));
    assert_eq!(editor.html, "<p><b>x</b> <a href=\"https://aylis.dev\"><i>y</i></a></p><p>z</p><p>w</p>");
    editor.select(3, 3);
    assert!(editor.command("link"));
    assert_eq!(editor.html, "<p><b>x</b> <i>y</i></p><p>z</p><p>w</p>");
    // Limpar formatação no texto todo.
    editor.select_all();
    assert!(editor.command("removeFormat"));
    assert_eq!(editor.html, "<p>x y</p><p>z</p><p>w</p>");
    let _ = f;
}

#[test]
fn justify_spreads_wrapped_lines_and_keeps_the_last_one() {
    let (mut ui, f, e) = setup();
    // 600 de largura, menos 2 × 10 de margem: 580 (72 letras de 8). A frase quebra uma vez.
    let words = "palavra ".repeat(12);
    let text = words.trim_end();
    ui.set_prop(e, "html", format!("<p style=\"text-align:justify\">{text}</p>")).unwrap();
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    editor.select(0, 0);
    assert_eq!(editor.current_block().align, TextAlign::Justify);
    editor.command("left");
    editor.command("justify");
    assert_eq!(editor.html, format!("<p style=\"text-align:justify\">{text}</p>"));
    let mut probe = Probe::default();
    ui.layout();
    ui.paint(f, &mut probe);
    // A primeira linha sai palavra por palavra (os espaços esticados); a última também é
    // desenhada em pedaços, mas sem esticar.
    assert_eq!(probe.texts.iter().filter(|t| t.starts_with("palavra")).count(), 12);
    // Clicar no fim da primeira linha: o cursor fica antes do espaço da quebra, lá na
    // direita (a linha foi esticada até 580).
    click(&mut ui, f, 10.0 + 10.0 + 579.0, 10.0 + 36.0 + 10.0 + 5.0);
    let (a, _) = ui.get::<RichEditor>(e).unwrap().selection();
    assert_eq!(a, 9 * 8 - 1, "nove palavras na primeira linha");
}

#[test]
fn font_size_applies_to_the_selected_run_only() {
    let (mut ui, f, e) = setup();
    ui.text_input(f, "Olá mundo");
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    editor.select(4, 9);
    assert!(editor.command("size-5"));
    assert_eq!(editor.html, "Olá <font size=\"5\">mundo</font>", "só o trecho, não a linha (como o QEditor)");
    assert_eq!(editor.current_format().size, 5);
    // Seleção com dois tamanhos: misto.
    editor.select(0, 9);
    assert_eq!(editor.current_format().size, u8::MAX);
    // O 3 é o normal: tira o tamanho.
    editor.select(4, 9);
    assert!(editor.command("size-3"));
    assert_eq!(editor.html, "Olá mundo");
    // Lido do html: o relativo conta a partir do 3 (o de fora, se houver).
    ui.set_prop(e, "html", "<font size=\"+1\">x</font><font size=\"6\"><font size=\"-2\">y</font></font><font size=\"3\">z</font>".to_string()).unwrap();
    let editor = ui.get_mut::<RichEditor>(e).unwrap();
    editor.select_all();
    editor.command("bold");
    editor.command("bold");
    assert_eq!(editor.html, "<font size=\"4\">xy</font>z", "6 − 2 = 4: o mesmo tamanho do x, um trecho só");
}

#[test]
fn size_button_opens_the_menu_and_the_choice_applies() {
    let (mut ui, f, e) = setup();
    ui.text_input(f, "Olá mundo");
    for _ in 0..5 {
        ui.key_down(f, shift(Key::Left));
    }
    ui.layout();
    ui.paint(f, &mut Probe::default());
    // Negrito, itálico, sublinhado e tachado (4 × 28), o separador (11) e o `size`.
    click(&mut ui, f, 10.0 + 4.0 + 4.0 * 28.0 + 11.0 + 13.0, 10.0 + 3.0 + 15.0);
    let items: Vec<ComponentId> = ui.components().filter(|&c| ui.get::<MenuItem>(c).is_some_and(|m| m.text.contains(" · "))).collect();
    assert_eq!(items.len(), 7, "os 7 tamanhos");
    let checked: Vec<&str> = items.iter().filter_map(|&c| ui.get::<MenuItem>(c).filter(|m| m.checked).map(|m| m.text.as_str())).collect();
    assert_eq!(checked, ["3 · Normal"]);
    let big = items.into_iter().find(|&c| ui.get::<MenuItem>(c).is_some_and(|m| m.text.starts_with('5'))).unwrap();
    ui.click(big);
    ui.process_events();
    assert_eq!(html(&ui, e), "Olá <font size=\"5\">mundo</font>");
}

#[test]
fn paints_text_and_toolbar_and_click_places_the_caret() {
    let (mut ui, f, e) = setup();
    ui.set_prop(e, "html", "<p>abc</p><p>def</p>".to_string()).unwrap();
    let mut probe = Probe::default();
    ui.layout();
    ui.paint(f, &mut probe);
    assert!(probe.texts.iter().any(|t| t == "abc") && probe.texts.iter().any(|t| t == "def"));
    assert!(probe.texts.iter().any(|t| t == "B"), "o botão do negrito");
    // A barra tem uma linha (36) e o texto começa 10 abaixo; cada linha tem 1,5 vez a
    // letra (de 18 a 24). Clique depois do "d" da segunda linha.
    click(&mut ui, f, 10.0 + 10.0 + 8.0, 10.0 + 36.0 + 10.0 + 26.0);
    let editor = ui.get::<RichEditor>(e).unwrap();
    assert_eq!(editor.selection(), (5, 5));
    // Sem barra.
    ui.set_prop(e, "toolbar", String::new()).unwrap();
    let mut probe = Probe::default();
    ui.paint(f, &mut probe);
    assert!(!probe.texts.iter().any(|t| t == "B"));
}
