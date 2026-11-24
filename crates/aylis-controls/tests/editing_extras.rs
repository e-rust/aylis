//! Os recursos de edição e de texto: a seta e o texto do `PaintCanvas`, o `ComboBox` com
//! rótulo, `outlined` e "×", o `sql` e as cores e os tokens do realce no próprio controle, o
//! ícone do campo pelo estilo, o `stroke_dash` do `Rectangle`, a etiqueta do `markup` e a barra
//! de rolagem pelo tema.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::platform::OffscreenService;
use aylis_core::theme::{ColorRef, Style, StyleProps};
use aylis_core::*;

/// O que a pintura desenhou: retângulos cheios e contornados, caminhos (os ícones) e textos.
#[derive(Default)]
struct Rec {
    fills: Vec<(RectF, Color)>,
    strokes: Vec<(RectF, Color, Option<Vec<f32>>)>,
    paths: Vec<(RectF, Color)>,
    texts: Vec<(String, RectF, Color)>,
}

fn solid(b: &Brush) -> Color {
    match b {
        Brush::Solid(c) => *c,
        _ => Color::TRANSPARENT,
    }
}

impl Canvas for Rec {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, r: RectF, _: f32, b: &Brush) {
        self.fills.push((r, solid(b)));
    }
    fn stroke_rect(&mut self, r: RectF, _: f32, s: &Stroke) {
        self.strokes.push((r, solid(&s.brush), s.dash.clone()));
    }
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, r: RectF, style: &TextStyle) {
        self.texts.push((text.to_string(), r, style.color));
    }
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * 8.0, 16.0)
    }
    fn fill_path(&mut self, p: &Path, b: &Brush) {
        self.paths.push((p.bounds(), solid(b)));
    }
    fn stroke_path(&mut self, p: &Path, s: &Stroke) {
        self.paths.push((p.bounds(), solid(&s.brush)));
    }
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 800.0, 600.0));
    (ui, f)
}

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
}

fn paint(ui: &mut Ui, f: ComponentId) -> Rec {
    let mut r = Rec::default();
    ui.layout();
    ui.paint(f, &mut r);
    r
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            other => format!("{other:?}"),
        })
    });
    l
}

/// Muda o tema do `Ui` (um estilo a mais).
fn with_style(ui: &mut Ui, key: &str, style: Style) {
    let mut theme = ui.theme().clone();
    theme.set_style(key, style);
    ui.set_theme(theme);
}

// ----- item 51: o ComboBox com a moldura do Edit -----

#[test]
fn the_combo_box_has_label_outlined_and_clear_button() {
    let (mut ui, f) = setup();
    let c = ui.create("ComboBox.outlined", Some(f)).unwrap();
    place(&mut ui, c, 10.0, 10.0, 200.0, 36.0);
    ui.set_prop(c, "items", "Clientes;Pedidos").unwrap();
    ui.set_prop(c, "label", "Tabela").unwrap();
    ui.set_prop(c, "clear_button", true).unwrap();
    // O estilo `outlined` é o mesmo do Edit.
    let theme = ui.theme().clone();
    let state = ControlState::default();
    assert_eq!(theme.resolve(&["ComboBox"], "outlined", state).border_color, theme.resolve(&["Edit"], "outlined", state).border_color);
    assert!(theme.style("ComboBox.outlined").is_some());

    // Sem opção: o rótulo no lugar do texto, na cor do placeholder; sem o "×".
    let r = paint(&mut ui, f);
    let label = r.texts.iter().find(|(t, ..)| t == "Tabela").expect("o rótulo dentro do campo");
    assert!(label.1.height > 20.0, "no lugar do texto, não pequeno em cima: {:?}", label.1);
    // Escolhida uma opção: o rótulo sobe para a borda (pequeno, em cima) e o "×" aparece.
    ui.set_prop(c, "item_index", 1i64).unwrap();
    let r = paint(&mut ui, f);
    assert!(r.texts.iter().any(|(t, ..)| t == "Pedidos"));
    let floating = r.texts.iter().find(|(t, ..)| t == "Tabela").expect("o rótulo sobre a borda");
    assert!(floating.1.y < 0.0, "o rótulo sobe para a borda: {:?}", floating.1);
    let changes = log(&mut ui, c, "on_change");
    // O "×" fica antes da seta, no fim do campo: limpa sem abrir a lista.
    let x = 10.0 + 200.0 - 18.0 - 8.0 - 7.0;
    ui.pointer_down(f, at(x, 28.0));
    ui.pointer_up(f, at(x, 28.0));
    assert_eq!(ui.get::<ComboBox>(c).unwrap().item_index, -1);
    assert!(!ui.get::<ComboBox>(c).unwrap().is_open(), "o \"×\" não abre a lista");
    assert_eq!(*changes.borrow(), vec![String::new()]);
}

#[test]
fn the_combo_box_without_the_new_properties_looks_as_before() {
    let (mut ui, f) = setup();
    let c = ui.create("ComboBox", Some(f)).unwrap();
    place(&mut ui, c, 10.0, 10.0, 200.0, 36.0);
    ui.set_prop(c, "items", "Um;Dois").unwrap();
    ui.set_prop(c, "item_index", 0i64).unwrap();
    let r = paint(&mut ui, f);
    // O fundo, a borda e o texto; nenhum rótulo, "×" ou linha de baixo.
    assert_eq!(r.texts.iter().filter(|(t, ..)| !t.is_empty()).map(|(t, ..)| t.as_str()).collect::<Vec<_>>(), vec!["Um"]);
    assert_eq!(r.paths.len(), 0, "sem ícone (a seta é traço)");
}

// ----- item 55: o ícone do Edit pelo estilo -----

#[test]
fn the_edit_icon_size_color_and_gap_come_from_the_style() {
    let (mut ui, f) = setup();
    let e = ui.create("Edit", Some(f)).unwrap();
    place(&mut ui, e, 10.0, 10.0, 240.0, 36.0);
    ui.set_prop(e, "icon", "search").unwrap();
    ui.set_prop(e, "text", "abc").unwrap();
    // Os valores de antes: 1,15 vez a fonte, `text_muted`.
    let muted = ui.theme().color("text_muted");
    let before = paint(&mut ui, f);
    let icon = before.paths.iter().find(|(_, c)| *c == muted).expect("o ícone na cor de antes");
    let text_x_before = before.texts.iter().find(|(t, ..)| t == "abc").unwrap().1.x;
    let icon_before = icon.0.width;
    assert!(icon_before <= (14.0f32 * 1.15).round() + 0.5);

    let mut edit = ui.theme().style("Edit").cloned().unwrap();
    edit.base.icon_size = Some(24.0);
    edit.base.icon_color = Some(ColorRef::Value(Color::rgb(0x12, 0x34, 0x56)));
    edit.base.icon_gap = Some(16.0);
    edit.focused.icon_color = Some(ColorRef::Value(Color::rgb(0, 200, 0)));
    with_style(&mut ui, "Edit", edit);
    let after = paint(&mut ui, f);
    let icon = after.paths.iter().find(|(_, c)| *c == Color::rgb(0x12, 0x34, 0x56)).expect("o ícone na cor do estilo");
    assert!((icon.0.width / icon_before - 24.0 / 16.0).abs() < 0.1, "o ícone na proporção 24/16: {icon_before} → {:?}", icon.0);
    let text_x = after.texts.iter().find(|(t, ..)| t == "abc").unwrap().1.x;
    // O ícone de 16 com 6 de espaço passa a 24 com 16: o texto anda 8 + 10.
    assert!((text_x - text_x_before - 18.0).abs() < 0.5, "o texto anda o tamanho e o espaço a mais: {text_x_before} → {text_x}");
    // Com foco, a cor do estado `focused`.
    ui.set_focus(e);
    let focused = paint(&mut ui, f);
    assert!(focused.paths.iter().any(|(_, c)| *c == Color::rgb(0, 200, 0)), "a cor do ícone com foco");
}

// ----- item 57: o tracejado do Rectangle -----

#[test]
fn the_rectangle_has_its_own_stroke_dash() {
    let (mut ui, f) = setup();
    let r = ui.create("Rectangle", Some(f)).unwrap();
    place(&mut ui, r, 10.0, 10.0, 100.0, 60.0);
    ui.set_prop(r, "stroke", Color::rgb(200, 0, 0)).unwrap();
    ui.set_prop(r, "stroke_width", 2.0f32).unwrap();
    let dash_of = |rec: &Rec| rec.strokes.iter().find(|(_, c, _)| *c == Color::rgb(200, 0, 0)).map(|(_, _, d)| d.clone());
    assert_eq!(dash_of(&paint(&mut ui, f)), Some(None), "sem `stroke_dash`: a borda do tema (contínua)");
    ui.set_prop(r, "stroke_dash", "6;4").unwrap();
    assert_eq!(dash_of(&paint(&mut ui, f)), Some(Some(vec![6.0, 4.0])));
    ui.set_prop(r, "stroke_dash", "3").unwrap();
    assert_eq!(dash_of(&paint(&mut ui, f)), Some(Some(vec![3.0, 3.0])), "um número: traço e vão iguais");
    ui.set_prop(r, "stroke_dash", "0").unwrap();
    assert_eq!(dash_of(&paint(&mut ui, f)), Some(None), "`0`: contínua mesmo com o tema tracejado");
    assert_eq!(ui.get_prop(r, "stroke_dash").unwrap(), PropValue::String("0".into()));
}

// ----- item 58: a etiqueta do markup -----

#[test]
fn the_markup_tag_has_background_and_border_and_does_not_break() {
    let spans = markup_spans("a <tag=primary,#FF0000><b>b</b> c</tag> d <tag>e</tag>");
    let tagged: Vec<_> = spans.iter().filter_map(|s| s.tag.as_ref().map(|t| (s.text.as_str(), t.fill.as_str(), t.border.clone(), t.id))).collect();
    assert_eq!(tagged, vec![("b", "primary", Some("#FF0000".to_string()), 0), (" c", "primary", Some("#FF0000".to_string()), 0), ("e", "surface_2", None, 1)]);
    assert_eq!(markup_plain("x <tag=warning>y</tag>"), "x y");

    let (mut ui, f) = setup();
    let l = ui.create("Label", Some(f)).unwrap();
    // Estreito: "word word" cabe numa linha, a etiqueta não; ela desce inteira.
    place(&mut ui, l, 10.0, 10.0, 110.0, 80.0);
    ui.set_prop(l, "markup", true).unwrap();
    ui.set_prop(l, "wrap", true).unwrap();
    ui.set_prop(l, "text", "word word <tag=primary,#FF0000>tag two</tag>").unwrap();
    let r = paint(&mut ui, f);
    let primary = ui.theme().color("primary");
    let back = r.fills.iter().find(|(_, c)| *c == primary).expect("o fundo da etiqueta");
    assert!(r.strokes.iter().any(|(_, c, _)| *c == Color::rgb(255, 0, 0)), "a borda da etiqueta");
    let piece = r.texts.iter().find(|(t, ..)| t.contains("tag")).unwrap();
    assert_eq!(piece.0, "tag two", "a etiqueta é desenhada inteira, sem quebrar no meio");
    let word = r.texts.iter().find(|(t, ..)| t == "word").unwrap();
    assert!(piece.1.y > word.1.y, "a etiqueta que não cabe desce inteira");
    let pad = ui.theme().spacing.get("xs").copied().unwrap_or(4.0);
    assert!((back.0.width - (7.0 * 8.0 + 2.0 * pad)).abs() < 0.5, "a largura do texto mais o espaço do tema: {:?}", back.0);
}

// ----- item 59: a barra de rolagem pelo tema -----

fn scroll_bar_theme(ui: &mut Ui) {
    let bar = Style::new(StyleProps { width: Some(4.0), thumb: Some(ColorRef::Value(Color::rgb(1, 2, 3))), ..Default::default() })
        .hover(StyleProps { thumb: Some(ColorRef::Value(Color::rgb(4, 5, 6))), ..Default::default() });
    with_style(ui, "ScrollBar", bar);
}

#[test]
fn the_scroll_box_bar_follows_the_scroll_bar_style() {
    let (mut ui, f) = setup();
    let s = ui.create("ScrollBox", Some(f)).unwrap();
    place(&mut ui, s, 0.0, 0.0, 200.0, 100.0);
    let inner = ui.create("Panel", Some(s)).unwrap();
    place(&mut ui, inner, 0.0, 0.0, 150.0, 400.0);
    // Sem o estilo: a barra de antes (8 de largura, `text_muted` a 30%).
    let r = paint(&mut ui, f);
    let muted = ui.theme().color("text_muted").with_opacity(0.3);
    let thumb = r.fills.iter().find(|(_, c)| *c == muted).expect("o polegar de antes");
    assert_eq!(thumb.0.width, 8.0);
    assert_eq!(ui.control(s).unwrap().padding().right, 10.0, "a calha de antes");

    scroll_bar_theme(&mut ui);
    // O tema mudou: o próximo tamanho (ou estilo) relê a barra.
    place(&mut ui, s, 0.0, 0.0, 210.0, 100.0);
    let r = paint(&mut ui, f);
    let thumb = r.fills.iter().find(|(_, c)| *c == Color::rgb(1, 2, 3)).expect("o polegar do tema");
    assert_eq!(thumb.0.width, 4.0);
    assert_eq!(ui.control(s).unwrap().padding().right, 6.0, "a calha acompanha a largura");
    // Com o mouse em cima do polegar: a cor do `hover`.
    let t = thumb.0;
    ui.pointer_move(f, at(t.x + 2.0, t.y + 4.0));
    let r = paint(&mut ui, f);
    assert!(r.fills.iter().any(|(_, c)| *c == Color::rgb(4, 5, 6)), "o polegar com o mouse");
}

#[test]
fn each_control_can_change_only_its_own_bar() {
    let (mut ui, f) = setup();
    scroll_bar_theme(&mut ui);
    // O `scroll_bar` do estilo do ListBox muda só a dele.
    let mut list = ui.theme().style("ListBox").cloned().unwrap_or_default();
    list.base.scroll_bar = Some(Box::new(Style::new(StyleProps { width: Some(9.0), thumb: Some(ColorRef::Value(Color::rgb(7, 8, 9))), ..Default::default() })));
    with_style(&mut ui, "ListBox", list);
    let l = ui.create("ListBox", Some(f)).unwrap();
    place(&mut ui, l, 0.0, 0.0, 200.0, 100.0);
    ui.set_prop(l, "items", (0..50).map(|i| format!("item {i}")).collect::<Vec<_>>().join(";")).unwrap();
    let m = ui.create("Memo", Some(f)).unwrap();
    place(&mut ui, m, 300.0, 0.0, 200.0, 100.0);
    ui.set_prop(m, "text", (0..50).map(|i| format!("linha {i}")).collect::<Vec<_>>().join("\n")).unwrap();
    let r = paint(&mut ui, f);
    let list_thumb = r.fills.iter().find(|(_, c)| *c == Color::rgb(7, 8, 9)).expect("a barra só do ListBox");
    assert_eq!(list_thumb.0.width, 9.0);
    // O Memo fica com a do tema (a largura 4 menos a margem do polegar, que é dele).
    let memo_thumb = r.fills.iter().find(|(_, c)| *c == Color::rgb(1, 2, 3)).expect("a barra do tema no Memo");
    assert!(memo_thumb.0.width <= 4.0);
}

// ----- item 52: o sql, as cores e os tokens do realce -----

#[test]
fn sql_is_built_in_and_case_insensitive() {
    assert!(aylis_controls::syntax::LANGUAGES.contains(&"sql"));
    let lines = aylis_controls::syntax::highlight("sql", "SELECT nome, COUNT(*) FROM TCliente -- conta\nWHERE id = 'it''s' AND n > 10");
    let kinds = |line: usize, text: &str| {
        let src: Vec<&str> = "SELECT nome, COUNT(*) FROM TCliente -- conta\nWHERE id = 'it''s' AND n > 10".split('\n').collect();
        let chars: Vec<char> = src[line].chars().collect();
        lines[line].iter().find(|s| chars[s.start..s.end].iter().collect::<String>().contains(text)).map(|s| s.kind)
    };
    use aylis_controls::syntax::TokenKind::{Comment, Function, Keyword, Number, Plain};
    assert_eq!(kinds(0, "SELECT"), Some(Keyword));
    assert_eq!(kinds(0, "COUNT"), Some(Function));
    assert_eq!(kinds(0, "TCliente"), Some(Plain), "a convenção de tipos do Pascal não vale no SQL");
    assert_eq!(kinds(0, "conta"), Some(Comment));
    assert_eq!(kinds(1, "where"), None);
    assert_eq!(kinds(1, "WHERE"), Some(Keyword));
    assert_eq!(kinds(1, "it''s"), Some(aylis_controls::syntax::TokenKind::String), "a aspa dobrada continua o texto");
    assert_eq!(kinds(1, "10"), Some(Number));
}

#[test]
fn syntax_tokens_add_words_and_prefixes_to_the_language() {
    use aylis_controls::syntax::{SyntaxToken, TokenKind, apply_tokens, highlight};
    let text = "select @total, nolock, 'a @b' from t";
    let spans = &highlight("sql", text)[0];
    let tokens = vec![
        SyntaxToken { name: "variavel".into(), prefix: "@".into(), color: "#FF0000".into(), ..Default::default() },
        SyntaxToken { name: "dica".into(), words: vec!["NOLOCK".into()], color: "#00FF00".into(), ..Default::default() },
    ];
    let colors = [Color::rgb(255, 0, 0), Color::rgb(0, 255, 0)];
    let chars: Vec<char> = text.chars().collect();
    let out = apply_tokens(spans, &chars, &tokens, &colors, true);
    let colored: Vec<(String, Color)> = out.iter().filter_map(|(s, c)| c.map(|c| (chars[s.start..s.end].iter().collect(), c))).collect();
    assert_eq!(colored, vec![("@total".to_string(), colors[0]), ("nolock".to_string(), colors[1])], "dentro do texto ('a @b') não vale");
    // Os pedaços cobrem a linha inteira, sem buraco, e o tipo de cada um continua.
    let covered: usize = out.iter().map(|(s, _)| s.end - s.start).sum();
    assert_eq!(covered, spans.iter().map(|s| s.end - s.start).sum::<usize>());
    assert!(out.iter().any(|(s, c)| c.is_none() && s.kind == TokenKind::Keyword));
}

#[test]
fn the_memo_changes_token_colors_and_adds_tokens_without_a_theme() {
    let (mut ui, f) = setup();
    let m = ui.create("CodeEditor", Some(f)).unwrap();
    place(&mut ui, m, 0.0, 0.0, 500.0, 200.0);
    ui.set_prop(m, "syntax", "sql").unwrap();
    ui.set_prop(m, "minimap", false).unwrap();
    ui.set_prop(m, "text", "select @x from t").unwrap();
    ui.set_prop(m, "keyword_color", Color::rgb(10, 20, 30)).unwrap();
    ui.set_prop(m, "syntax_tokens", r##"[{"name": "variavel", "prefix": "@", "color": "#AA00AA"}]"##).unwrap();
    let r = paint(&mut ui, f);
    let color_of = |text: &str| r.texts.iter().find(|(t, ..)| t == text).map(|(_, _, c)| *c);
    assert_eq!(color_of("select"), Some(Color::rgb(10, 20, 30)), "a cor da palavra-chave trocada no controle");
    assert_eq!(color_of("from"), Some(Color::rgb(10, 20, 30)));
    assert_eq!(color_of("@x"), Some(Color::rgb(0xAA, 0, 0xAA)), "o token novo");
    assert_eq!(ui.get_prop(m, "keyword_color").unwrap(), PropValue::Color(Color::rgb(10, 20, 30)));
}

// ----- item 23: a seta e o texto do PaintCanvas -----

struct SkiaOffscreen;

impl OffscreenService for SkiaOffscreen {
    fn render_png(&self, _: u32, _: u32, _: f32, _: Color, _: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        None
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        aylis_skia::SkiaRenderer::new().render_rgba(width, height, scale, background, |c| paint(c))
    }
}

fn paint_canvas() -> (Ui, ComponentId, ComponentId) {
    let (mut ui, f) = setup();
    let p = ui.create("PaintCanvas", Some(f)).unwrap();
    ui.invoke(p, "new", &[PropValue::Int(200), PropValue::Int(120)]).unwrap();
    (ui, f, p)
}

fn pixel(ui: &Ui, p: ComponentId, x: i32, y: i32) -> Color {
    ui.get::<PaintCanvas>(p).unwrap().bitmap().get(x, y).unwrap()
}

#[test]
fn the_arrow_tool_draws_a_line_with_a_filled_head() {
    let (mut ui, f, p) = paint_canvas();
    ui.set_prop(p, "tool", "arrow").unwrap();
    ui.set_prop(p, "brush_size", 2i64).unwrap();
    ui.set_prop(p, "color", Color::rgb(255, 0, 0)).unwrap();
    paint(&mut ui, f);
    ui.pointer_down(f, at(20.0, 60.0));
    ui.pointer_move(f, at(100.0, 60.0));
    ui.pointer_move(f, at(180.0, 60.0));
    ui.pointer_up(f, at(180.0, 60.0));
    let red = Color::rgb(255, 0, 0);
    assert_eq!(pixel(&ui, p, 60, 60), red, "a linha");
    assert_eq!(pixel(&ui, p, 179, 60), red, "a ponta");
    // A ponta é mais larga que a linha (a cabeça da seta), perto do fim.
    assert_eq!(pixel(&ui, p, 172, 64), red, "a cabeça abre para os lados");
    assert_ne!(pixel(&ui, p, 100, 64), red, "a linha é fina no meio");
    assert!(ui.invoke(p, "can_undo", &[]).unwrap() == PropValue::Bool(true));
}

#[test]
fn the_text_tool_turns_into_pixels_when_confirmed() {
    let (mut ui, f, p) = paint_canvas();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.services_mut().offscreen = Some(Box::new(SkiaOffscreen));
    ui.set_prop(p, "tool", "text").unwrap();
    ui.set_prop(p, "color", Color::rgb(0, 0, 255)).unwrap();
    paint(&mut ui, f);
    let before = ui.invoke(p, "checksum", &[]).unwrap();
    ui.pointer_down(f, at(10.0, 10.0));
    ui.pointer_up(f, at(10.0, 10.0));
    ui.text_input(f, "Câmera 3");
    // Escrevendo: a prévia na tela, a imagem ainda igual.
    let r = paint(&mut ui, f);
    assert!(r.texts.iter().any(|(t, ..)| t == "Câmera 3"), "a prévia do texto");
    assert_eq!(ui.invoke(p, "checksum", &[]).unwrap(), before);
    // Enter confirma: o texto vira pixel azul na imagem.
    ui.key_down(f, KeyEvent { key: Key::Enter, ..Default::default() });
    assert_ne!(ui.invoke(p, "checksum", &[]).unwrap(), before);
    let bitmap = ui.get::<PaintCanvas>(p).unwrap().bitmap().clone();
    let blue = (8..40).flat_map(|y| (8..120).map(move |x| (x, y))).filter(|&(x, y)| bitmap.get(x, y).is_some_and(|c| c.b > 200 && c.r < 80)).count();
    assert!(blue > 20, "pixels do texto na imagem: {blue}");
    // Dá para desfazer, e o Esc desiste sem mudar a imagem.
    assert_eq!(ui.invoke(p, "undo", &[]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.invoke(p, "checksum", &[]).unwrap(), before);
    ui.pointer_down(f, at(10.0, 60.0));
    ui.pointer_up(f, at(10.0, 60.0));
    ui.text_input(f, "nada");
    ui.key_down(f, KeyEvent { key: Key::Escape, ..Default::default() });
    assert_eq!(ui.invoke(p, "checksum", &[]).unwrap(), before);
    // Pelo código: `draw_text`.
    assert_eq!(ui.invoke(p, "draw_text", &[PropValue::Int(5), PropValue::Int(5), PropValue::String("ok".into())]).unwrap(), PropValue::Bool(true));
    assert_ne!(ui.invoke(p, "checksum", &[]).unwrap(), before);
}

#[test]
fn without_offscreen_drawing_the_text_does_not_change_the_image() {
    // Um `Ui` sem o desenho fora da tela (nenhuma plataforma o pôs): o texto não vira pixel
    // (degrada). No navegador há o `aylis_web::WebOffscreen` (aylis-web, `web_canvas`).
    let (mut ui, f, p) = paint_canvas();
    paint(&mut ui, f);
    let before = ui.invoke(p, "checksum", &[]).unwrap();
    assert_eq!(ui.invoke(p, "draw_text", &[PropValue::Int(5), PropValue::Int(5), PropValue::String("ok".into())]).unwrap(), PropValue::Bool(false));
    assert_eq!(ui.invoke(p, "checksum", &[]).unwrap(), before);
}

#[test]
fn the_virtual_list_bar_follows_the_scroll_bar_style() {
    let (mut ui, f) = setup();
    let list = ui.add(VirtualList::default(), f);
    place(&mut ui, list, 0.0, 0.0, 300.0, 200.0);
    let template = ui.add(Panel::new(300.0, 40.0), list);
    let _ = template;
    ui.set_prop(list, "item_height", 40.0).unwrap();
    ui.set_prop(list, "item_count", 100i64).unwrap();
    ui.set_prop(list, "scroll_bar", "always").unwrap();
    // Sem o estilo: a barra de antes (10 de largura, o cursor de 6 a 45% do `text_muted`).
    let r = paint(&mut ui, f);
    let muted = ui.theme().color("text_muted");
    let thumb = r.fills.iter().find(|(_, c)| *c == muted.with_opacity(0.45)).expect("o cursor de antes");
    assert_eq!(thumb.0.width, 6.0);
    assert!(r.fills.iter().any(|(_, c)| *c == muted.with_opacity(0.08)), "o trilho de antes");

    scroll_bar_theme(&mut ui);
    place(&mut ui, list, 0.0, 0.0, 310.0, 200.0);
    let r = paint(&mut ui, f);
    let thumb = r.fills.iter().find(|(_, c)| *c == Color::rgb(1, 2, 3)).expect("o cursor do tema");
    assert_eq!(thumb.0.width, 1.0, "a barra de 4 menos a margem de 2 dos lados (no mínimo 1)");
    // As linhas usam a largura que a barra deixou.
    let row = VirtualList::item(&ui, list, 0).unwrap();
    assert_eq!(ui.control(row).unwrap().bounds().width, 306.0);
    // Com o mouse em cima do cursor: a cor do `hover`.
    let t = thumb.0;
    ui.pointer_move(f, at(t.x + 0.5, t.y + 4.0));
    let r = paint(&mut ui, f);
    assert!(r.fills.iter().any(|(_, c)| *c == Color::rgb(4, 5, 6)), "o cursor com o mouse");
}
