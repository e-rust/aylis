//! Pendências fechadas: ListBox com várias seleções e "hoje" no fuso local.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("t", 300.0, 400.0));
    let list = ui.add(ListBox::new(&["um", "dois", "três", "quatro", "cinco"]), f);
    ui.control_mut(list).unwrap().set_rect(0.0, 0.0, 200.0, 300.0);
    let _ = ui.set_prop(list, "multi_select", true);
    ui.layout();
    (ui, f, list)
}

fn click(ui: &mut Ui, f: ComponentId, row: usize, modifiers: Modifiers) {
    // Moldura de 1 px; linhas de 28.
    let e = PointerEvent { position: PointF::new(50.0, 1.0 + row as f32 * 28.0 + 14.0), modifiers, ..Default::default() };
    ui.pointer_move(f, e);
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
}

#[test]
fn list_box_selects_several_items() {
    let (mut ui, f, list) = setup();
    let changes = Rc::new(RefCell::new(Vec::new()));
    let c = changes.clone();
    ui.on(list, "on_change", move |_, _, a| {
        if let EventArgs::Text(t) = a {
            c.borrow_mut().push(t.clone());
        }
    });
    let ctrl = Modifiers { ctrl: true, ..Default::default() };
    let shift = Modifiers { shift: true, ..Default::default() };
    click(&mut ui, f, 1, Modifiers::default());
    click(&mut ui, f, 3, ctrl);
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_indices(), [1, 3]);
    assert_eq!(ui.get_prop(list, "selected").unwrap(), PropValue::String("1;3".into()));
    // Shift: da âncora (3) até o 0, no lugar do resto.
    click(&mut ui, f, 0, shift);
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_texts(), ["um", "dois", "três", "quatro"]);
    // Ctrl+clique tira; clique simples fica só com um.
    click(&mut ui, f, 2, ctrl);
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_indices(), [0, 1, 3]);
    click(&mut ui, f, 4, Modifiers::default());
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_indices(), [4]);
    assert_eq!(changes.borrow().last().unwrap(), "cinco");
    // Teclado: Shift+↑ estende, Ctrl+A seleciona tudo.
    ui.set_focus(list);
    ui.key_down(f, KeyEvent { key: Key::Up, modifiers: shift, ..Default::default() });
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_indices(), [3, 4]);
    ui.key_down(f, KeyEvent { key: Key::Char('a'), modifiers: ctrl, ..Default::default() });
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_indices().len(), 5);
    // Gravado no form e lido de volta.
    let _ = ui.set_prop(list, "selected", "0;2");
    assert_eq!(ui.get::<ListBox>(list).unwrap().selected_texts(), ["um", "três"]);
}

#[test]
fn single_selection_is_unchanged() {
    let (mut ui, f, list) = setup();
    let _ = ui.set_prop(list, "multi_select", false);
    click(&mut ui, f, 2, Modifiers { ctrl: true, ..Default::default() });
    let l = ui.get::<ListBox>(list).unwrap();
    assert_eq!((l.item_index, l.selected_indices()), (2, vec![2]));
}

#[test]
fn today_follows_the_local_time_zone() {
    let utc = web_time_now();
    let before = Day::today();
    // Três dias à frente do UTC (um fuso de mentira, para o teste).
    aylis_core::platform::set_local_offset_fn(|_| 3 * 86_400);
    let after = Day::today();
    assert_eq!(after.days() - before.days(), 3, "{before:?} → {after:?} ({utc})");
}

fn web_time_now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

fn kinds(language: &str, text: &str) -> Vec<Vec<(String, syntax::TokenKind)>> {
    let lines: Vec<Vec<char>> = text.split('\n').map(|l| l.chars().collect()).collect();
    syntax::highlight(language, text)
        .into_iter()
        .zip(&lines)
        .map(|(spans, line)| spans.into_iter().filter(|s| s.kind != syntax::TokenKind::Plain).map(|s| (line[s.start..s.end].iter().collect(), s.kind)).collect())
        .collect()
}

#[test]
fn code_is_highlighted_by_language() {
    use syntax::TokenKind::*;
    let rust = kinds("rust", "pub fn soma(a: i32) -> Vec<u8> { // fim\n    let s = \"um \\\" dois\"; println!(\"{s}\", 42);\n    /* bloco\n continua */ 'x' 'a }");
    assert_eq!(rust[0], [("pub".into(), Keyword), ("fn".into(), Keyword), ("soma".into(), Function), ("i32".into(), Type), ("Vec".into(), Type), ("u8".into(), Type), ("// fim".into(), Comment)]);
    assert_eq!(rust[1], [("let".into(), Keyword), ("\"um \\\" dois\"".into(), String), ("println".into(), Function), ("\"{s}\"".into(), String), ("42".into(), Number)]);
    assert_eq!(rust[2], [("/* bloco".into(), Comment)], "o comentário de bloco continua");
    assert_eq!(rust[3], [(" continua */".into(), Comment), ("'x'".into(), String), ("'a".into(), Type)], "caractere e tempo de vida");

    let py = kinds("python", "@property\ndef nome(self):  # comentário\n    return f\"oi {x}\" + '''várias\nlinhas''' + None");
    assert_eq!(py[0], [("property".into(), Function)]);
    assert_eq!(py[1], [("def".into(), Keyword), ("nome".into(), Function), ("self".into(), Keyword), ("# comentário".into(), Comment)]);
    assert_eq!(py[2], [("return".into(), Keyword), ("f\"oi {x}\"".into(), String), ("'''várias".into(), String)]);
    assert_eq!(py[3], [("linhas'''".into(), String), ("None".into(), Keyword)]);

    let json = kinds("json", "{\"nome\": \"Ana\", \"idade\": 30, \"ativo\": true}");
    assert_eq!(json[0], [("\"nome\"".into(), Type), ("\"Ana\"".into(), String), ("\"idade\"".into(), Type), ("30".into(), Number), ("\"ativo\"".into(), Type), ("true".into(), Keyword)]);
    // Linguagem desconhecida: nada colorido.
    assert!(kinds("cobol", "MOVE A TO B").iter().all(Vec::is_empty));
}

#[test]
fn the_memo_edits_code_with_indentation_and_tabs() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("t", 400.0, 300.0));
    let memo = ui.add(Memo::new(""), f);
    ui.control_mut(memo).unwrap().set_rect(0.0, 0.0, 380.0, 280.0);
    for (k, v) in [("syntax", PropValue::from("rust")), ("line_numbers", true.into()), ("want_tabs", true.into())] {
        ui.set_prop(memo, k, v).unwrap();
    }
    ui.layout();
    ui.set_focus(memo);
    let key = |ui: &mut Ui, k: Key, shift: bool| ui.key_down(f, KeyEvent { key: k, modifiers: Modifiers { shift, ..Default::default() }, ..Default::default() });
    ui.text_input(f, "fn main() {");
    key(&mut ui, Key::Enter, false);
    ui.text_input(f, "let x = 1;");
    key(&mut ui, Key::Enter, false);
    assert_eq!(ui.get::<Memo>(memo).unwrap().text, "fn main() {\n    let x = 1;\n    ", "o recuo continua e aumenta depois de {{");
    // Tab recua (e não passa o foco); Shift+Tab volta.
    key(&mut ui, Key::Tab, false);
    assert_eq!(ui.focused(f), Some(memo));
    assert!(ui.get::<Memo>(memo).unwrap().text.ends_with("\n        "));
    key(&mut ui, Key::Tab, true);
    key(&mut ui, Key::Tab, true);
    ui.text_input(f, "}");
    assert_eq!(ui.get::<Memo>(memo).unwrap().text, "fn main() {\n    let x = 1;\n}");
    // Sem want_tabs, o Tab passa o foco, como num Memo comum.
    let other = ui.add(Edit::new(""), f);
    ui.control_mut(other).unwrap().set_rect(0.0, 285.0, 100.0, 10.0);
    ui.set_prop(memo, "want_tabs", false).unwrap();
    ui.set_focus(memo);
    key(&mut ui, Key::Tab, false);
    assert_eq!(ui.focused(f), Some(other));
}

#[test]
fn the_other_project_languages_are_highlighted_too() {
    use syntax::TokenKind::*;
    // Pascal: palavras sem diferença de maiúsculas, { } e (* *), diretiva {$...}, '' dentro do texto.
    let pas = kinds("pascal", "{ Eventos } unit principal;\n{$mode objfpc}\nprocedure TFrm.Show(Aylis: TAylis; N: Integer);\nBEGIN s := 'it''s' + #13 + $FF; (* fim *) end;");
    assert_eq!(pas[0], [("{ Eventos }".into(), Comment), ("unit".into(), Keyword)]);
    assert_eq!(pas[1], [("{$mode objfpc}".into(), Keyword)]);
    assert_eq!(pas[2], [("procedure".into(), Keyword), ("TFrm".into(), Type), ("Show".into(), Function), ("TAylis".into(), Type), ("Integer".into(), Type)]);
    assert_eq!(pas[3], [("BEGIN".into(), Keyword), ("'it''s'".into(), String), ("#13".into(), Number), ("$FF".into(), Number), ("(* fim *)".into(), Comment), ("end".into(), Keyword)]);

    // TypeScript: crase de várias linhas e decorador.
    let ts = kinds("typescript", "@Component\nexport const n: number = `a\nb` + 'c'; // x");
    assert_eq!(ts[0], [("Component".into(), Function)]);
    assert_eq!(ts[1], [("export".into(), Keyword), ("const".into(), Keyword), ("number".into(), Type), ("`a".into(), String)]);
    assert_eq!(ts[2], [("b`".into(), String), ("'c'".into(), String), ("// x".into(), Comment)]);

    // C#: diretiva, texto literal @"" com "" dentro, $"".
    let cs = kinds("csharp", "#region Eventos\npublic void Salvar() { var s = @\"c:\\x \"\"y\"\"\"; var t = $\"oi {n}\"; }");
    assert_eq!(cs[0], [("#region".into(), Keyword), ("Eventos".into(), Type)]);
    assert_eq!(cs[1], [("public".into(), Keyword), ("void".into(), Keyword), ("Salvar".into(), Function), ("var".into(), Keyword), ("@\"c:\\x \"\"y\"\"\"".into(), String), ("var".into(), Keyword), ("$\"oi {n}\"".into(), String)]);

    // Java: anotação e bloco de texto.
    let java = kinds("java", "@Override\nString s = \"\"\"\n  oi\"\"\";");
    assert_eq!(java[0], [("Override".into(), Function)]);
    assert_eq!(java[1], [("String".into(), Type), ("\"\"\"".into(), String)]);
    assert_eq!(java[2], [("  oi\"\"\"".into(), String)]);

    // Go: crase sem escape e tipos primitivos.
    let go = kinds("go", r"func soma(a int) error { return fmt.Errorf(`c:\x`) }");
    assert_eq!(go[0], [("func".into(), Keyword), ("soma".into(), Function), ("int".into(), Type), ("error".into(), Type), ("return".into(), Keyword), ("Errorf".into(), Function), (r"`c:\x`".into(), String)]);

    // C++: #include, texto cru R"(...)" e números hexadecimais.
    let cpp = kinds("cpp", "#include <vector>\nauto s = R\"(a\\b)\"; int x = 0xFF;");
    assert_eq!(cpp[0], [("#include".into(), Keyword)]);
    assert_eq!(cpp[1], [("auto".into(), Keyword), (r#"R"(a\b)""#.into(), String), ("int".into(), Type), ("0xFF".into(), Number)]);

    // Rust: atributo e texto cru.
    let rust = kinds("rust", "#[derive(Debug)]\nlet p = r#\"C:\\x\"#;");
    assert_eq!(rust[0], [("#[derive(Debug)]".into(), Function)]);
    assert_eq!(rust[1], [("let".into(), Keyword), (r##"r#"C:\x"#"##.into(), String)]);
}
