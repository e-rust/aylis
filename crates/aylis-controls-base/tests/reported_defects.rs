//! Os defeitos dos controles base vistos em apps reais: o duplo clique duas vezes na `TreeView`,
//! as cores do menu da bandeja, o `icon_in_arrow` que empurra o texto de todos, o duplo clique
//! perdido no `refresh` da `VirtualList`, o erro do `Edit` que encolhe a caixa, o `highlight` do
//! `Label` com `line_height` e os "a conferir rodando": o `Timer` posto num modal e o
//! `minimap: false`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls_base::{confirm, CodeEditor, Edit, Form, Label, Layout, Panel, TabStrip, Timer, TrayIcon, TreeLeafLayout, TreeView, VirtualList};
use aylis_core::*;

/// Guarda o que foi pintado: os retângulos preenchidos (com a cor) e os textos.
#[derive(Default)]
struct Recorder {
    fills: Vec<(RectF, f32, Color)>,
    texts: Vec<(String, RectF)>,
}

impl Canvas for Recorder {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _dx: f32, _dy: f32) {}
    fn scale(&mut self, _sx: f32, _sy: f32) {}
    fn rotate(&mut self, _degrees: f32) {}
    fn clip_rect(&mut self, _rect: RectF) {}
    fn save_layer(&mut self, _opacity: f32) {}
    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        if let Brush::Solid(c) = brush {
            self.fills.push((rect, radius, *c));
        }
    }
    fn stroke_rect(&mut self, _rect: RectF, _radius: f32, _stroke: &Stroke) {}
    fn fill_ellipse(&mut self, _rect: RectF, _brush: &Brush) {}
    fn stroke_ellipse(&mut self, _rect: RectF, _stroke: &Stroke) {}
    fn draw_line(&mut self, _from: PointF, _to: PointF, _stroke: &Stroke) {}
    fn draw_text(&mut self, text: &str, rect: RectF, _style: &TextStyle) {
        self.texts.push((text.to_string(), rect));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.5, style.font.size * 1.3)
    }
}

fn form(ui: &mut Ui, w: f32, h: f32) -> ComponentId {
    let f = ui.add_root(Form::new("frmTeste", w, h));
    ui.show(f).unwrap();
    f
}

fn press(ui: &mut Ui, f: ComponentId, p: PointF) {
    let e = PointerEvent { position: p, button: PointerButton::Primary, click_count: 1, ..Default::default() };
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
}

fn center(r: RectF) -> PointF {
    PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0)
}

/// Conta as chamadas de `event` em `id`, com o argumento de cada uma.
fn record(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<EventArgs>>> {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let c = calls.clone();
    ui.on(id, event, move |_, _, args| c.borrow_mut().push(args.clone()));
    calls
}

#[test]
fn a_double_click_reaches_the_tree_view_once() {
    let mut ui = Ui::new();
    let f = form(&mut ui, 400.0, 300.0);
    let t = ui.add(TreeView::default(), f);
    ui.control_mut(t).unwrap().set_rect(0.0, 0.0, 240.0, 200.0);
    ui.set_prop(t, "items", r#"[{"text":"Câmeras","children":[{"text":"Portão"}]},{"text":"Garagem"}]"#).unwrap();
    ui.update_layout();
    let calls = record(&mut ui, t, "on_dbl_click");
    let node = ui.get::<TreeView>(t).unwrap().node_rects(&[1]).unwrap();
    let at = center(node.text);
    press(&mut ui, f, at);
    press(&mut ui, f, at);
    let calls = calls.borrow();
    assert_eq!(calls.len(), 1, "uma chamada só: {calls:?}");
    assert!(matches!(&calls[0], EventArgs::Text(t) if t == "Garagem"), "com o caminho do nó: {calls:?}");
}

#[test]
fn an_icon_on_a_leaf_in_icon_in_arrow_does_not_push_the_other_texts() {
    let mut ui = Ui::new();
    let f = form(&mut ui, 400.0, 300.0);
    let t = ui.add(TreeView::default(), f);
    ui.control_mut(t).unwrap().set_rect(0.0, 0.0, 240.0, 200.0);
    ui.set_prop(t, "leaf_layout", PropValue::Enum("icon_in_arrow".into())).unwrap();
    let items = |eye: &str| format!(r#"[{{"text":"Visualizações","children":[{{"text":"Sala"{eye}}},{{"text":"Pátio"}}]}},{{"text":"Monitor 2"}}]"#);
    ui.set_prop(t, "items", items("").as_str()).unwrap();
    ui.get_mut::<TreeView>(t).unwrap().expand_all(true);
    let texts = |ui: &Ui| -> Vec<f32> {
        let tv = ui.get::<TreeView>(t).unwrap();
        [&[0][..], &[0, 0], &[0, 1], &[1]].iter().map(|p| tv.node_rects(p).unwrap().text.x).collect()
    };
    let before = texts(&ui);
    // O olho numa folha (a visualização aberta): vai no lugar da seta, e nenhum texto anda.
    ui.set_prop(t, "items", items(r#","icon":"eye""#).as_str()).unwrap();
    ui.get_mut::<TreeView>(t).unwrap().expand_all(true);
    assert_eq!(texts(&ui), before);
    let tv = ui.get::<TreeView>(t).unwrap();
    let sala = tv.node_rects(&[0, 0]).unwrap();
    let icon = sala.icon.expect("a folha com ícone o mostra");
    assert!(icon.x >= sala.arrow.x && icon.right() <= sala.arrow.right(), "no lugar da seta");
    assert_eq!(tv.leaf_layout, TreeLeafLayout::IconInArrow);
}

#[test]
fn a_refresh_between_the_two_clicks_keeps_the_double_click_of_the_virtual_list() {
    let mut ui = Ui::new();
    // As linhas são cópias do modelo (pela classe).
    ui.register_class::<Panel>("Standard");
    ui.register_class::<Label>("Standard");
    let f = form(&mut ui, 300.0, 400.0);
    let list = ui.add(VirtualList::default(), f);
    ui.control_mut(list).unwrap().set_width(300.0).set_height(400.0);
    let row = ui.add(Panel::new(300.0, 40.0), list);
    ui.add(Label::new(""), row);
    ui.set_prop(list, "item_height", 40.0).unwrap();
    ui.set_prop(list, "item_count", 100).unwrap();
    ui.layout();
    VirtualList::refresh(&mut ui, list);
    ui.layout();
    // Rolar recicla as linhas de cima para os itens de baixo: a ordem das linhas deixa de ser a
    // dos itens (era aí que o `refresh` trocava o item de linha).
    VirtualList::scroll_to(&mut ui, list, 5);
    ui.layout();
    let refreshed = Rc::new(RefCell::new(0));
    let r = refreshed.clone();
    ui.on(list, "on_item_click", move |ui, list, _| {
        *r.borrow_mut() += 1;
        VirtualList::refresh(ui, list);
    });
    let doubles = record(&mut ui, list, "on_item_dbl_click");
    let item = 10;
    let before = VirtualList::item(&ui, list, item).unwrap();
    let at = center(ui.absolute_rect(before).unwrap());
    press(&mut ui, f, at);
    ui.layout();
    assert_eq!(*refreshed.borrow(), 1);
    assert_eq!(VirtualList::item(&ui, list, item), Some(before), "o item continua na mesma linha");
    press(&mut ui, f, at);
    let doubles = doubles.borrow();
    assert_eq!(doubles.len(), 1, "o segundo clique é o duplo");
    assert_eq!(doubles[0], EventArgs::Value(PropValue::Int(item as i64)));
}

#[test]
fn the_error_of_an_edit_with_a_fixed_height_does_not_shrink_the_box() {
    let mut ui = Ui::new();
    let f = form(&mut ui, 300.0, 200.0);
    let e = ui.add(Edit::default(), f);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 200.0, 40.0);
    ui.set_prop(e, "error", "Obrigatório").unwrap();
    ui.layout();
    let mut canvas = Recorder::default();
    ui.paint(f, &mut canvas);
    // A pintura do controle é nas coordenadas dele.
    let boxr = canvas.fills.iter().find(|(r, ..)| r.x == 0.0 && r.y == 0.0 && r.width == 200.0).map(|(r, ..)| *r).expect("a caixa do campo");
    assert_eq!(boxr.height, 40.0, "a caixa inteira");
    let (_, line) = canvas.texts.iter().find(|(t, _)| t == "Obrigatório").expect("o erro");
    assert!(line.y >= 40.0, "o erro embaixo da caixa: {line:?}");

    // Com `auto_size`, a altura soma a linha do erro, e a caixa fica com a de uma linha.
    ui.set_prop(e, "auto_size", true).unwrap();
    ui.layout();
    let grown = ui.absolute_rect(e).unwrap();
    assert!(grown.height > 18.0);
    let mut canvas = Recorder::default();
    ui.paint(f, &mut canvas);
    let boxr = canvas.fills.iter().find(|(r, ..)| r.x == 0.0 && r.y == 0.0 && r.width == grown.width).map(|(r, ..)| *r).unwrap();
    assert_eq!(boxr.height, grown.height - 18.0);
}

#[test]
fn the_highlight_of_a_label_shows_with_line_height_and_wrap() {
    let mut ui = Ui::new();
    let f = form(&mut ui, 400.0, 200.0);
    let l = ui.add(Label::new("o gato e o rato no telhado"), f);
    ui.control_mut(l).unwrap().set_rect(0.0, 0.0, 300.0, 100.0);
    ui.set_prop(l, "wrap", true).unwrap();
    ui.set_prop(l, "line_height", 1.35).unwrap();
    ui.set_prop(l, "highlight", "rato").unwrap();
    ui.layout();
    let mark = ui.theme().colors.get("highlight").copied().unwrap_or_else(|| ui.theme().color("warning").with_opacity(0.35));
    let mut canvas = Recorder::default();
    ui.paint(f, &mut canvas);
    let marks: Vec<_> = canvas.fills.iter().filter(|(_, _, c)| *c == mark).collect();
    assert_eq!(marks.len(), 1, "o fundo marcado na palavra: {:?}", canvas.fills);
    assert!(marks[0].0.width > 0.0 && marks[0].0.x > 0.0);
}

#[test]
fn the_tray_menu_follows_the_popup_menu_style_of_the_theme() {
    let mut ui = Ui::new();
    let (fill, text) = (Color::rgb(0x10, 0x20, 0x30), Color::rgb(0xF0, 0xE0, 0xD0));
    ui.theme_mut().styles.insert("PopupMenu".into(), Style::new(StyleProps::default().fill(fill).text(text)));
    let f = form(&mut ui, 300.0, 200.0);
    let tray = ui.add(TrayIcon::default(), f);
    ui.set_prop(tray, "menu_items", r#"[{"text":"Abrir"},{"separator":true},{"text":"Sair"}]"#).unwrap();
    let before: Vec<ComponentId> = ui.roots().to_vec();
    ui.send_message(tray, &TrayEvent { kind: TrayEventKind::RightClick, screen: PointF::new(800.0, 600.0), scale: 1.0 });
    ui.process_posted();
    let menu = ui.roots().iter().copied().find(|r| !before.contains(r)).expect("o menu abriu numa janela");
    ui.layout();
    let mut canvas = Recorder::default();
    ui.paint(menu, &mut canvas);
    assert!(canvas.fills.iter().any(|(_, _, c)| *c == fill), "o fundo do estilo: {:?}", canvas.fills);

    // Sem o estilo, `dark` usa o tema escuro do Aylis (e não uma cor fixa no código).
    let mut ui = Ui::new();
    let f = form(&mut ui, 300.0, 200.0);
    let tray = ui.add(TrayIcon::default(), f);
    ui.set_prop(tray, "menu_items", r#"[{"text":"Abrir"}]"#).unwrap();
    ui.set_prop(tray, "menu_theme", "dark").unwrap();
    let before: Vec<ComponentId> = ui.roots().to_vec();
    ui.send_message(tray, &TrayEvent { kind: TrayEventKind::RightClick, screen: PointF::new(800.0, 600.0), scale: 1.0 });
    ui.process_posted();
    let menu = ui.roots().iter().copied().find(|r| !before.contains(r)).unwrap();
    ui.layout();
    let mut canvas = Recorder::default();
    ui.paint(menu, &mut canvas);
    let surface = Theme::dark().color("surface");
    assert!(canvas.fills.iter().any(|(_, _, c)| *c == surface), "{:?}", canvas.fills);
}

#[test]
fn a_timer_put_inside_an_open_modal_fires() {
    let mut ui = Ui::new();
    let main = form(&mut ui, 400.0, 300.0);
    let modal = ui.add_root(Form::new("frmModal", 300.0, 200.0));
    ui.show_modal(modal, |_, _| {}).unwrap();
    // Um frame (outra raiz) com um timer, posto dentro do modal já aberto (o `create_in`).
    let frame = ui.add_root(Panel::new(200.0, 100.0));
    let timer = ui.add(Timer::default(), frame);
    ui.set_prop(timer, "interval", 100).unwrap();
    ui.set_prop(timer, "enabled", true).unwrap();
    ui.set_parent(frame, Some(modal)).unwrap();
    let ticks = record(&mut ui, timer, "on_timer");
    for _ in 0..5 {
        ui.advance(0.1);
        ui.run_timers();
    }
    assert!(ticks.borrow().len() >= 4, "o timer do frame dentro do modal dispara: {}", ticks.borrow().len());
    let _ = main;
}

#[test]
fn minimap_false_turns_the_minimap_of_the_code_editor_off() {
    let mut ui = Ui::new();
    let f = form(&mut ui, 600.0, 400.0);
    let e = ui.add(CodeEditor::new((0..200).map(|i| format!("linha {i}\n")).collect::<String>()), f);
    ui.control_mut(e).unwrap().set_rect(0.0, 0.0, 600.0, 400.0);
    assert_eq!(ui.get_prop(e, "minimap").unwrap(), PropValue::Bool(true));
    let with = fills(&mut ui, f);
    // Como o layout em JSON põe: `"minimap": false`.
    ui.set_prop(e, "minimap", false).unwrap();
    assert_eq!(ui.get_prop(e, "minimap").unwrap(), PropValue::Bool(false));
    assert!(!ui.get::<CodeEditor>(e).unwrap().minimap());
    let without = fills(&mut ui, f);
    assert!(without < with, "sem os traços do minimapa: {with} → {without}");
}

/// Quantos retângulos a pintura do form preencheu (o minimapa é um por trecho de linha).
fn fills(ui: &mut Ui, f: ComponentId) -> usize {
    ui.layout();
    let mut canvas = Recorder::default();
    ui.paint(f, &mut canvas);
    canvas.fills.len()
}

/// A fábrica de forms do teste: o `frmPrincipal`, com a faixa de abas `tabAbas` em cima e o lugar
/// das páginas embaixo.
struct Factory;

impl aylis_core::platform::FormFactory for Factory {
    fn create_form(&mut self, ui: &mut Ui, name: &str) -> std::result::Result<ComponentId, String> {
        let form = ui.add_root(Form::new("Principal", 800.0, 500.0));
        ui.set_name(form, name).map_err(|e| e.to_string())?;
        let strip = ui.add(TabStrip::new(), form);
        ui.set_name(strip, "tabAbas").map_err(|e| e.to_string())?;
        ui.control_mut(strip).unwrap().set_align(Align::Top).set_height(38.0);
        let host = ui.add(Layout::default(), form);
        ui.control_mut(host).unwrap().set_align(Align::Client);
        ui.get_mut::<TabStrip>(strip).unwrap().page_host = Some(host);
        Ok(form)
    }

    fn form_names(&self) -> Vec<String> {
        vec!["frmPrincipal".into()]
    }
}

#[test]
fn a_tab_torn_from_a_second_instance_opens_the_same_form_with_the_keyboard_focus() {
    let mut ui = Ui::new();
    ui.services_mut().forms = Some(Box::new(Factory));
    let first = ui.create_form_instance("frmPrincipal").unwrap();
    ui.show(first).unwrap();
    // A segunda janela principal (`frmPrincipal2`), com duas abas; a primeira tem um campo.
    let second = ui.create_form_instance("frmPrincipal").unwrap();
    assert_eq!(ui.name(second), Some("frmPrincipal2"));
    let strip = ui.find_in(second, "tabAbas").unwrap();
    let host = ui.get::<TabStrip>(strip).unwrap().page_host.unwrap();
    let mut pages = Vec::new();
    for title in ["Tabelas", "Campos"] {
        let page = ui.add(Panel::default(), host);
        TabStrip::add_page(&mut ui, strip, title, "", page);
        pages.push(page);
    }
    let edit = ui.add(Edit::default(), pages[0]);
    ui.control_mut(edit).unwrap().set_rect(10.0, 10.0, 200.0, 32.0);
    ui.show(second).unwrap();
    ui.layout();
    ui.paint(second, &mut Recorder::default());
    let windows = Rc::new(RefCell::new(None));
    let w = windows.clone();
    ui.on(strip, "on_new_window", move |ui, _, args| {
        if let EventArgs::Value(PropValue::Component(Some(s))) = args {
            *w.borrow_mut() = Some(ui.root_of(*s));
        }
    });

    // Puxa a primeira aba para baixo e a solta longe de qualquer janela.
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(second, at(40.0, 19.0));
    ui.pointer_move(second, at(60.0, 19.0));
    ui.pointer_move(second, at(60.0, 300.0));
    assert!(ui.dragging(second));
    ui.set_screen_pointer(Some(ScreenPointer { x: 1500, y: 600, scale: 1.0 }));
    ui.drag_over_form(second, None, PointF::default());
    ui.pointer_up(second, at(900.0, 700.0));

    let window = windows.borrow().expect("a aba virou janela");
    assert!(ui.find_in(window, "tabAbas").is_some(), "outra instância do frmPrincipal, e não a janela genérica");
    assert_eq!(ui.form_class(window), Some("frmPrincipal"));
    assert_eq!(ui.root_of(pages[0]), window);
    assert_eq!(ui.focused(window), Some(edit), "o teclado vai junto com a aba");
}

#[test]
fn esc_closes_the_confirm_as_cancel() {
    let mut ui = Ui::new();
    let main = form(&mut ui, 400.0, 300.0);
    let answer = Rc::new(RefCell::new(None));
    let a = answer.clone();
    let dialog = confirm(&mut ui, "Excluir", "Excluir o registro?", "Excluir", true, move |_, ok| *a.borrow_mut() = Some(ok));
    ui.layout();
    let esc = KeyEvent { key: Key::Escape, ..Default::default() };
    ui.key_down(dialog, esc);
    ui.key_up(dialog, esc);
    assert_eq!(*answer.borrow(), Some(false), "o Esc fecha como Cancelar");
    assert!(!ui.is_open(main) || ui.open_forms().len() == 1, "o diálogo fechou");
}
