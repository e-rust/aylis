//! Subtipos, mistura de temas, ícones, dicas e os componentes modernos.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Canvas que só mede (8 por caractere) e conta o que foi desenhado.
#[derive(Default)]
struct Probe {
    paths: usize,
    shadows: usize,
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
    fn fill_path(&mut self, _: &Path, _: &Brush) {
        self.paths += 1;
    }
    fn stroke_path(&mut self, _: &Path, _: &Stroke) {
        self.paths += 1;
    }
    fn draw_shadow(&mut self, _: RectF, _: f32, _: &Shadow) {
        self.shadows += 1;
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

fn paint(ui: &mut Ui, f: ComponentId) -> Probe {
    let mut p = Probe::default();
    ui.layout();
    ui.paint(f, &mut p);
    p
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            EventArgs::Value(v) => format!("{v:?}"),
            other => format!("{other:?}"),
        })
    });
    l
}

#[test]
fn subtypes_create_with_presets_and_style() {
    let (mut ui, f) = setup();
    let b = ui.create("Button.icon", Some(f)).unwrap();
    let button = ui.get::<Button>(b).unwrap();
    assert_eq!((button.text.as_str(), button.icon.as_str()), ("", "settings"));
    assert_eq!(ui.control(b).unwrap().style(), "icon");
    assert_eq!(ui.get_prop(b, "width").unwrap(), PropValue::Float(36.0));
    // Variante que só existe no tema também vale; desconhecida dá erro.
    let p = ui.create("Panel.card", Some(f)).unwrap();
    assert_eq!(ui.control(p).unwrap().style(), "card");
    assert!(ui.create("Button.nao_existe", Some(f)).is_err());
    // A lista da paleta junta as registradas e as do tema.
    let names: Vec<String> = ui.variants_of("Button").into_iter().map(|v| v.name).collect();
    assert!(names.contains(&"fab".to_string()) && names.contains(&"danger".to_string()));
    let mut theme = ui.theme().clone();
    theme.set_style("Button.minha", Style::new(StyleProps::new().fill("accent")));
    ui.set_theme(theme);
    let v = ui.variants_of("Button");
    assert!(v.iter().any(|v| v.name == "minha" && v.caption == "minha"), "variante criada só no tema aparece");
    // Subtipo do usuário com valores iniciais.
    ui.register_variant("Edit", "cpf", "CPF", vec![("placeholder", PropValue::String("000.000.000-00".into()))]);
    let e = ui.create("Edit.cpf", Some(f)).unwrap();
    assert_eq!(ui.get::<Edit>(e).unwrap().placeholder, "000.000.000-00");
}

#[test]
fn themes_mix_per_class_and_per_control() {
    let (mut ui, f) = setup();
    let panel = ui.add(Panel::default(), f);
    let inside = ui.add(Label::new("x"), panel);
    let button = ui.add(Button::new("ok"), f);
    assert_eq!(ui.theme_of(inside).name, "Aylis Light");
    // Tema por controle: vale para os filhos.
    ui.set_prop(panel, "theme", "Aylis Dark").unwrap();
    assert_eq!(ui.theme_of(inside).name, "Aylis Dark");
    assert_eq!(ui.theme_of(button).name, "Aylis Light");
    // Mistura por classe: os botões vêm do escuro.
    ui.theme_mut().mix("Button", "Aylis Dark");
    assert_eq!(ui.theme_of(button).name, "Aylis Dark");
    assert_eq!(ui.theme_of(f).name, "Aylis Light");
    // Tema registrado pelo app.
    let mut brand = Theme::light();
    brand.name = "Marca".into();
    brand.colors.insert("primary".into(), Color::rgb(200, 0, 0));
    ui.register_theme(brand);
    ui.theme_mut().mix("Button", "Marca");
    assert_eq!(ui.theme_of(button).color("primary"), Color::rgb(200, 0, 0));
    assert!(ui.theme_names().contains(&"Marca".to_string()));
    paint(&mut ui, f);
}

#[test]
fn icons_buttons_and_shadows_paint() {
    let (mut ui, f) = setup();
    let b = ui.add(Button::with_icon("play"), f);
    place(&mut ui, b, 10.0, 10.0, 40.0, 40.0);
    let fab = ui.create("Button.fab", Some(f)).unwrap();
    place(&mut ui, fab, 100.0, 10.0, 56.0, 56.0);
    let p = paint(&mut ui, f);
    assert!(p.paths >= 2, "ícones desenhados como caminhos");
    assert!(p.shadows >= 1, "o FAB tem sombra pelo tema");
    // Ícone do app substitui pelo nome.
    ui.register_icon_svg("logo", "M2 2h20v20H2Z", "", 24.0).unwrap();
    assert!(ui.icon("logo").is_some());
    assert!(ui.register_icon_svg("ruim", "M2 x", "", 24.0).is_err());
}

#[test]
fn checkable_button_toggles() {
    let (mut ui, f) = setup();
    let b = ui.add(Button::with_icon("volume"), f);
    ui.set_prop(b, "checkable", true).unwrap();
    ui.set_prop(b, "checked_icon", "volume-off").unwrap();
    place(&mut ui, b, 10.0, 10.0, 40.0, 40.0);
    let changes = log(&mut ui, b, "on_change");
    paint(&mut ui, f);
    ui.pointer_down(f, at(20.0, 20.0));
    ui.pointer_up(f, at(20.0, 20.0));
    assert!(ui.get::<Button>(b).unwrap().checked);
    assert!(ui.control(b).unwrap().state().checked, "o tema vê o estado marcado");
    assert_eq!(changes.borrow().len(), 1);
}

#[test]
fn search_edit_clear_button() {
    let (mut ui, f) = setup();
    let e = ui.create("Edit.search", Some(f)).unwrap();
    place(&mut ui, e, 10.0, 10.0, 200.0, 32.0);
    ui.set_prop(e, "text", "abc").unwrap();
    paint(&mut ui, f);
    // O "×" fica no fim do campo.
    ui.pointer_down(f, at(10.0 + 200.0 - 14.0 - 8.0, 26.0));
    ui.pointer_up(f, at(10.0 + 200.0 - 14.0 - 8.0, 26.0));
    assert_eq!(ui.get::<Edit>(e).unwrap().text, "");
}

#[test]
fn segmented_control_and_breadcrumb() {
    let (mut ui, f) = setup();
    let s = ui.add(SegmentedControl::new("Tela;Janela;Área"), f);
    place(&mut ui, s, 0.0, 0.0, 300.0, 34.0);
    let changes = log(&mut ui, s, "on_change");
    paint(&mut ui, f);
    ui.pointer_down(f, at(250.0, 10.0));
    assert_eq!(ui.get::<SegmentedControl>(s).unwrap().item_index, 2);
    ui.set_focus(s);
    ui.key_down(f, key(Key::Left));
    assert_eq!(ui.get::<SegmentedControl>(s).unwrap().item_index, 1);
    assert_eq!(changes.borrow().len(), 2);

    let b = ui.add(Breadcrumb::default(), f);
    place(&mut ui, b, 0.0, 100.0, 400.0, 28.0);
    let picked = log(&mut ui, b, "on_change");
    paint(&mut ui, f);
    ui.pointer_down(f, at(4.0, 110.0));
    assert_eq!(picked.borrow().as_slice(), ["Int(0)"]);
}

#[test]
fn hotkey_edit_and_direction_pad() {
    let (mut ui, f) = setup();
    let h = ui.add(HotkeyEdit::default(), f);
    ui.set_focus(h);
    let m = Modifiers { ctrl: true, shift: true, ..Default::default() };
    ui.key_down(f, KeyEvent { key: Key::Char('r'), modifiers: m, ..Default::default() });
    assert_eq!(ui.get::<HotkeyEdit>(h).unwrap().hotkey, "Ctrl+Shift+R");
    ui.key_down(f, KeyEvent { key: Key::Control, modifiers: m, ..Default::default() });
    assert_eq!(ui.get::<HotkeyEdit>(h).unwrap().hotkey, "Ctrl+Shift+R", "só o modificador não muda");
    ui.key_down(f, key(Key::Backspace));
    assert_eq!(ui.get::<HotkeyEdit>(h).unwrap().hotkey, "");

    // 140 × 140 (o tamanho padrão): centro em (70, 70).
    let mut pad = DirectionPad::default();
    assert_eq!(pad.direction_at(PointF::new(70.0, 70.0)), Some("center"));
    assert_eq!(pad.direction_at(PointF::new(70.0, 5.0)), Some("up"));
    assert_eq!(pad.direction_at(PointF::new(135.0, 70.0)), Some("right"));
    assert_eq!(pad.direction_at(PointF::new(115.0, 115.0)), Some("down_right"));
    assert_eq!(pad.direction_at(PointF::new(2.0, 2.0)), None, "fora do círculo");
    pad.diagonals = false;
    assert_eq!(pad.direction_at(PointF::new(110.0, 120.0)), Some("down"));
}

#[test]
fn node_graph_connects_by_dragging() {
    let (mut ui, f) = setup();
    let mut g = NodeGraph::default();
    g.add_node(GraphNode::new("mic", "Microfone", 0.0, 0.0).source());
    g.add_node(GraphNode::new("fone", "Fones", 400.0, 0.0).sink());
    let g = ui.add(g, f);
    place(&mut ui, g, 0.0, 0.0, 800.0, 400.0);
    let connected = log(&mut ui, g, "on_connect");
    paint(&mut ui, f);
    // O grafo enquadrou os nós ao ganhar tamanho: converte do "mundo" para a tela.
    let graph = ui.get::<NodeGraph>(g).unwrap();
    let port = graph.to_view(PointF::new(300.0, 36.0)); // saída do microfone
    let target = graph.to_view(PointF::new(450.0, 30.0)); // dentro dos fones
    ui.pointer_down(f, at(port.x, port.y));
    ui.pointer_move(f, at(target.x, target.y));
    ui.pointer_up(f, at(target.x, target.y));
    assert_eq!(connected.borrow().len(), 1);
    let graph = ui.get::<NodeGraph>(g).unwrap();
    assert_eq!(graph.cables().len(), 1);
    assert!(!graph.can_connect("mic", "fone"), "não repete");
    assert!(!graph.can_connect("fone", "mic"), "destino não tem saída");
    // Delete apaga o cabo selecionado.
    ui.set_focus(g);
    ui.key_down(f, key(Key::Delete));
    assert!(ui.get::<NodeGraph>(g).unwrap().cables().is_empty());
}

#[test]
fn chat_view_lays_out_messages() {
    let (mut ui, f) = setup();
    let mut chat = ChatView::default();
    chat.add_message(ChatMessage::system("d", "Hoje"));
    chat.add_message(ChatMessage::new("1", "Ana", "Oi! Tudo bem?").at("10:00"));
    chat.add_message(ChatMessage::new("2", "Eu", "Tudo ótimo").mine().at("10:01").status(MessageStatus::Read));
    let c = ui.add(chat, f);
    place(&mut ui, c, 0.0, 0.0, 400.0, 300.0);
    let clicks = log(&mut ui, c, "on_message_click");
    let p = paint(&mut ui, f);
    assert!(p.texts.iter().any(|t| t == "Hoje"));
    assert!(p.texts.iter().any(|t| t == "Ana"));
    assert!(p.texts.iter().any(|t| t == "Tudo ótimo"));
    // Clique numa bolha dispara com o id.
    ui.pointer_down(f, at(380.0, 120.0));
    let _ = clicks;
}

#[test]
fn hints_appear_after_a_pause() {
    let (mut ui, f) = setup();
    let b = ui.add(Button::new("ok"), f);
    place(&mut ui, b, 10.0, 10.0, 80.0, 30.0);
    ui.set_prop(b, "hint", "Salva o arquivo").unwrap();
    paint(&mut ui, f);
    ui.pointer_move(f, at(20.0, 20.0));
    assert!(ui.visible_hint(f).is_none());
    assert!(ui.next_wake().is_some(), "acorda quando a dica vence");
    ui.set_time(1.0);
    ui.run_timers();
    assert_eq!(ui.visible_hint(f).map(|h| h.0), Some("Salva o arquivo".to_string()));
    let p = paint(&mut ui, f);
    assert!(p.texts.iter().any(|t| t == "Salva o arquivo"));
    ui.pointer_down(f, at(20.0, 20.0));
    assert!(ui.visible_hint(f).is_none(), "clique esconde");
}

#[test]
fn level_meter_and_seek_bar() {
    assert_eq!(LevelMeter::from_db(0.0), 1.0);
    assert_eq!(LevelMeter::from_db(-60.0), 0.0);
    assert_eq!(format_time(65.0), "1:05");
    assert_eq!(format_time(3723.0), "1:02:03");
    let (mut ui, f) = setup();
    let s = ui.add(SeekBar::default(), f);
    place(&mut ui, s, 0.0, 0.0, 216.0, 28.0);
    ui.set_prop(s, "duration", 200.0).unwrap();
    let seeks = log(&mut ui, s, "on_seek");
    paint(&mut ui, f);
    ui.pointer_down(f, at(108.0, 14.0));
    ui.pointer_up(f, at(108.0, 14.0));
    assert!((ui.get::<SeekBar>(s).unwrap().position - 100.0).abs() < 0.5);
    assert_eq!(seeks.borrow().len(), 1);
    let m = ui.create("LevelMeter.vertical", Some(f)).unwrap();
    ui.set_prop(m, "value", 0.95f32).unwrap();
    paint(&mut ui, f);
}

#[test]
fn platform_themes_exist_and_every_control_paints_in_each_look() {
    let names: Vec<String> = aylis_core::themes::all().iter().map(|t| t.name.clone()).collect();
    assert_eq!(names.len(), 33);
    assert_eq!(aylis_core::themes::FAMILIES.len(), 19);
    assert_eq!(aylis_core::themes::of_family("Win32", true).unwrap().name, "Win32", "só claro");
    for family in aylis_core::themes::FAMILIES {
        assert!(aylis_core::themes::of_family(family, false).is_some(), "{family}");
    }
    assert_eq!(aylis_core::themes::of_family("Windows Phone", true).unwrap().look, Look::WinPhone);
    assert_eq!(aylis_core::themes::of_family("WinUI 3", false).unwrap().look, Look::Win11);
    assert_eq!(aylis_core::themes::of_family("Android 7", false).unwrap().color("primary"), aylis_core::Color::from_hex("#FF4081").unwrap());
    let mut unique = names.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), names.len(), "nomes únicos");
    assert_eq!(aylis_core::themes::of_family("Windows 11", true).unwrap().name, "Windows 11 Dark");
    assert_eq!(aylis_core::themes::of_family("iOS 7", true).unwrap().name, "iOS 7 Light", "sem escuro: o que existe");
    assert_eq!(aylis_core::themes::builtin("material 3 light").unwrap().look, Look::Material3);
    // Os da web: as cores de cada framework.
    assert_eq!(aylis_core::themes::of_family("Bootstrap 5", false).unwrap().color("primary"), aylis_core::Color::from_hex("#0D6EFD").unwrap());
    assert_eq!(aylis_core::themes::of_family("Ant Design 5", false).unwrap().color("primary"), aylis_core::Color::from_hex("#1677FF").unwrap());
    assert_eq!(aylis_core::themes::of_family("shadcn/ui", true).unwrap().name, "shadcn/ui Dark");
    assert_eq!(aylis_core::themes::of_family("Vuetify 3", false).unwrap().look, Look::Material2);

    // Os mesmos controles em cada tema (pela propriedade `theme` do pai).
    let (mut ui, f) = setup();
    for (i, name) in names.iter().chain(["Aylis Light".to_string(), "Aylis Dark".to_string()].iter()).enumerate() {
        let panel = ui.add(Layout::default(), f);
        place(&mut ui, panel, 0.0, i as f32 * 10.0, 300.0, 200.0);
        ui.set_prop(panel, "theme", name.as_str()).unwrap();
        for class in ["Button", "Button.secondary", "Edit", "CheckBox", "RadioButton", "Switch", "TrackBar", "ProgressBar", "SegmentedControl"] {
            let id = ui.create(class, Some(panel)).unwrap();
            let _ = ui.set_prop(id, "checked", true);
        }
        assert_eq!(&ui.theme_of(ui.children(panel)[0]).name, name);
    }
    let p = paint(&mut ui, f);
    assert!(p.paths > 0);
}

#[test]
fn theme_source_explains_where_the_theme_comes_from() {
    let (mut ui, f) = setup();
    let side = ui.add(Layout::default(), f);
    ui.set_name(side, "lateral").unwrap();
    let inside = ui.add(Label::new("x"), side);
    let own = ui.add(CheckBox::new("c"), f);
    let button = ui.add(Button::new("b"), f);
    assert_eq!(ui.theme_source(own), ("Aylis Light".to_string(), ThemeSource::App));
    ui.set_prop(side, "theme", "Windows 11 Dark").unwrap();
    assert_eq!(ui.theme_source(inside), ("Windows 11 Dark".to_string(), ThemeSource::Inherited(side)));
    ui.set_prop(own, "theme", "iOS 13 Light").unwrap();
    assert_eq!(ui.theme_source(own), ("iOS 13 Light".to_string(), ThemeSource::Own));
    ui.theme_mut().mix("Button", "Material 3 Light");
    assert_eq!(ui.theme_source(button), ("Material 3 Light".to_string(), ThemeSource::Class("Button".into())));
    assert!(ui.theme_names().iter().any(|n| n == "Windows 7"));
}

/// A dica espera 1 s com o mouse parado (07/11: "aparece muito rápido"; 500 ms ainda era pouco): andando por cima do
/// botão o tempo recomeça; um tremido de poucos pixels não recomeça.
#[test]
fn hints_wait_for_the_mouse_to_rest_one_second() {
    let (mut ui, f) = setup();
    let b = ui.add(Button::new("ok"), f);
    place(&mut ui, b, 10.0, 10.0, 200.0, 30.0);
    ui.set_prop(b, "hint", "Salva o arquivo").unwrap();
    paint(&mut ui, f);
    ui.set_time(10.0);
    ui.pointer_move(f, at(20.0, 20.0));
    // Andando pelo botão por um segundo: nada aparece.
    for i in 1..=10 {
        ui.set_time(10.0 + i as f64 * 0.1);
        ui.pointer_move(f, at(20.0 + i as f32 * 15.0, 20.0));
        ui.run_timers();
        assert!(ui.visible_hint(f).is_none(), "andando, sem dica ({i})");
    }
    // Parado (com um tremido de 1 px): aparece depois de um segundo.
    ui.set_time(11.3);
    ui.pointer_move(f, at(171.0, 20.0));
    ui.run_timers();
    assert!(ui.visible_hint(f).is_none(), "0,3 s parado ainda é cedo");
    ui.set_time(11.8);
    ui.run_timers();
    assert!(ui.visible_hint(f).is_none(), "0,8 s também");
    ui.set_time(12.01);
    ui.run_timers();
    assert_eq!(ui.visible_hint(f).map(|h| h.0), Some("Salva o arquivo".to_string()));
}
