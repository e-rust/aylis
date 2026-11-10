//! Componentes da fase 16: Memo, NumberBox, TrackBar, datas, listas, abas, Timer...

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;

struct NullCanvas;

impl Canvas for NullCanvas {
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
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let f = ui.add_root(Form::new("teste", 500.0, 400.0));
    (ui, f)
}

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
}

fn paint(ui: &mut Ui, f: ComponentId) {
    ui.layout();
    ui.paint(f, &mut NullCanvas);
}

fn changes(ui: &mut Ui, id: ComponentId) -> Rc<RefCell<Vec<String>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    ui.on(id, "on_change", move |_, _, args| {
        l.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            EventArgs::Value(v) => format!("{v:?}"),
            other => format!("{other:?}"),
        });
    });
    log
}

#[test]
fn memo_edits_several_lines() {
    let (mut ui, f) = setup();
    let m = ui.add(Memo::new(""), f);
    place(&mut ui, m, 10.0, 10.0, 300.0, 120.0);
    ui.set_focus(m);
    paint(&mut ui, f);
    ui.text_input(f, "primeira");
    ui.key_down(f, key(Key::Enter));
    ui.text_input(f, "segunda");
    ui.key_down(f, key(Key::Up));
    ui.key_down(f, key(Key::End));
    ui.text_input(f, "!");
    let memo = ui.get::<Memo>(m).unwrap();
    assert_eq!(memo.lines(), ["primeira!", "segunda"]);
    assert_eq!(memo.caret_position(), (0, 9));
    ui.key_down(f, KeyEvent { key: Key::End, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    ui.key_down(f, key(Key::Backspace));
    assert_eq!(ui.get::<Memo>(m).unwrap().text, "primeira!\nsegund");
    // Tab continua passando o foco
    let other = ui.add(Button::new("ok"), f);
    ui.key_down(f, key(Key::Tab));
    assert_eq!(ui.focused(f), Some(other));
}

#[test]
fn number_box_steps_clamps_and_reads_the_locale() {
    let (mut ui, f) = setup();
    let n = ui.add(NumberBox::new(5.0, 0.0, 10.0), f);
    ui.set_prop(n, "decimals", 1u32).unwrap();
    place(&mut ui, n, 10.0, 10.0, 120.0, 32.0);
    let log = changes(&mut ui, n);
    paint(&mut ui, f);
    assert_eq!(ui.get::<NumberBox>(n).unwrap().edit.text, "5,0", "vírgula do pt-BR");
    ui.set_focus(n);
    ui.key_down(f, key(Key::Up));
    assert_eq!(ui.get::<NumberBox>(n).unwrap().value, 6.0);
    // digitar e sair do campo
    ui.key_down(f, KeyEvent { key: Key::Char('a'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    ui.text_input(f, "7,25x");
    let other = ui.add(Button::new("ok"), f);
    ui.set_focus(other);
    let nb = ui.get::<NumberBox>(n).unwrap();
    assert_eq!((nb.value, nb.edit.text.as_str()), (7.3, "7,3"), "letras são ignoradas; arredonda para 1 casa");
    ui.set_prop(n, "value", 50.0).unwrap();
    assert_eq!(ui.get::<NumberBox>(n).unwrap().value, 10.0, "limitado ao máximo");
    // O valor mudado pelo código também avisa (como na VCL), com o que ficou.
    assert_eq!(log.borrow().len(), 3);
    assert_eq!(NumberBox::parse("1.234,5"), Some(1234.5));
    assert_eq!(NumberBox::parse("1,234.5"), Some(1234.5));
}

#[test]
fn track_bar_follows_the_pointer_and_keys() {
    let (mut ui, f) = setup();
    let t = ui.add(TrackBar::new(0.0, 0.0, 100.0), f);
    place(&mut ui, t, 0.0, 0.0, 210.0, 28.0);
    let log = changes(&mut ui, t);
    paint(&mut ui, f);
    ui.pointer_down(f, at(105.0, 14.0));
    assert_eq!(ui.get::<TrackBar>(t).unwrap().value, 50.0);
    ui.pointer_move(f, at(200.0, 14.0));
    ui.pointer_up(f, at(200.0, 14.0));
    assert_eq!(ui.get::<TrackBar>(t).unwrap().value, 100.0);
    ui.set_focus(t);
    ui.key_down(f, key(Key::Home));
    assert_eq!(ui.get::<TrackBar>(t).unwrap().value, 0.0);
    assert_eq!(log.borrow().len(), 3);
}

#[test]
fn calendar_math() {
    let d = Day::new(2024, 2, 29).unwrap();
    assert_eq!(d.weekday(), 4, "29/02/2024 foi quinta");
    assert_eq!(Day::from_days(d.days()), d);
    assert_eq!(d.add_months(12), Day::new(2025, 2, 28).unwrap());
    assert_eq!(Day::new(2026, 1, 31).unwrap().add_months(1).iso(), "2026-02-28");
    assert_eq!(Day::from_days(0).iso(), "1970-01-01");
    assert_eq!(Day::parse("24/10/26", "dd/mm/yyyy").unwrap().iso(), "2026-10-24");
    assert_eq!(Day::parse("10/24/2026", "mm/dd/yyyy").unwrap().format("dd/mm/yyyy"), "24/10/2026");
    assert!(Day::parse("31/02/2026", "dd/mm/yyyy").is_none());
    assert!(Day::new(2023, 2, 29).is_none());
}

#[test]
fn date_edit_typing_and_calendar_popup() {
    let (mut ui, f) = setup();
    let d = ui.add(DateEdit::new("2026-10-24"), f);
    place(&mut ui, d, 10.0, 10.0, 150.0, 32.0);
    let log = changes(&mut ui, d);
    paint(&mut ui, f);
    assert_eq!(ui.get::<DateEdit>(d).unwrap().edit.text, "24/10/2026");
    ui.set_focus(d);
    ui.text_input(f, "01/02/2027");
    ui.key_down(f, key(Key::Enter));
    assert_eq!(ui.get::<DateEdit>(d).unwrap().date, "2027-02-01");
    // abre o calendário pelo ícone e escolhe um dia
    ui.pointer_down(f, at(150.0, 25.0));
    ui.pointer_up(f, at(150.0, 25.0));
    let popup = ui.roots().iter().flat_map(|&r| ui.children(r).to_vec()).find(|&c| ui.get::<Calendar>(c).is_some()).expect("calendário aberto");
    paint(&mut ui, f);
    let r = ui.absolute_rect(popup).unwrap();
    // segunda linha, primeira coluna da grade
    let cell = PointF::new(r.x + 6.0 + 10.0, r.y + 36.0 + 24.0 + 50.0);
    ui.pointer_down(f, at(cell.x, cell.y));
    ui.pointer_up(f, at(cell.x, cell.y));
    let date = ui.get::<DateEdit>(d).unwrap().date.clone();
    assert!(date.starts_with("2027-02-0"), "um dia da 2ª semana de fevereiro: {date}");
    assert!(ui.get::<Calendar>(popup).is_none(), "o calendário fechou");
    assert_eq!(log.borrow().len(), 2);
}

#[test]
fn time_edit_parses_and_steps() {
    assert_eq!(TimeEdit::parse("14:30"), Some(14 * 3600 + 30 * 60));
    assert_eq!(TimeEdit::parse("1430"), Some(14 * 3600 + 30 * 60));
    assert_eq!(TimeEdit::parse("25:00"), None);
    let (mut ui, f) = setup();
    let t = ui.add(TimeEdit::new("23:59"), f);
    place(&mut ui, t, 10.0, 10.0, 100.0, 32.0);
    paint(&mut ui, f);
    ui.set_focus(t);
    ui.key_down(f, key(Key::Up));
    assert_eq!(ui.get::<TimeEdit>(t).unwrap().time, "00:00", "dá a volta na meia-noite");
}

#[test]
fn list_box_and_list_view_navigate() {
    let (mut ui, f) = setup();
    let l = ui.add(ListBox::new(&["um", "dois", "três"]), f);
    place(&mut ui, l, 0.0, 0.0, 200.0, 60.0);
    let log = changes(&mut ui, l);
    paint(&mut ui, f);
    ui.pointer_down(f, at(20.0, 40.0));
    assert_eq!(ui.get::<ListBox>(l).unwrap().text(), "dois");
    ui.set_focus(l);
    ui.key_down(f, key(Key::End));
    assert_eq!(ui.get::<ListBox>(l).unwrap().text(), "três");
    assert_eq!(*log.borrow(), ["dois", "três"]);

    let v = ui.add(ListView::default(), f);
    ui.set_prop(v, "items", "Maria|Cliente desde 2020;João;Ana|VIP").unwrap();
    assert_eq!(ui.get::<ListView>(v).unwrap().item(0).unwrap().detail, "Cliente desde 2020");
    assert_eq!(ui.get::<ListView>(v).unwrap().count(), 3);
}

#[test]
fn tree_view_reads_levels_and_expands() {
    let (mut ui, f) = setup();
    let t = ui.add(TreeView::default(), f);
    place(&mut ui, t, 0.0, 0.0, 240.0, 300.0);
    ui.set_prop(t, "items", "Brasil;>São Paulo;>>Campinas;>Rio;Portugal").unwrap();
    let tree = ui.get::<TreeView>(t).unwrap();
    assert_eq!(tree.nodes().len(), 2);
    assert_eq!(tree.nodes()[0].children[0].children[0].text, "Campinas");
    let log = changes(&mut ui, t);
    paint(&mut ui, f);
    ui.set_focus(t);
    ui.key_down(f, key(Key::Down)); // Brasil
    ui.key_down(f, key(Key::Right)); // abre
    ui.key_down(f, key(Key::Down)); // São Paulo
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "Brasil/São Paulo");
    ui.key_down(f, key(Key::Left)); // fechado: vai para o pai
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "Brasil");
    assert_eq!(*log.borrow(), ["Brasil", "Brasil/São Paulo", "Brasil"]);
}

/// O `full_expand` (o `FullExpand` do Delphi) dispara o `on_expand` de cada nó, um por vez:
/// o carregamento sob demanda funciona, e os filhos novos também são abertos.
#[test]
fn full_expand_fires_on_expand_for_every_node_and_loads_on_demand() {
    let (mut ui, f) = setup();
    let t = ui.add(TreeView::default(), f);
    place(&mut ui, t, 0.0, 0.0, 240.0, 300.0);
    // Cada nó começa com um filho provisório ("…"), trocado ao abrir.
    ui.set_prop(t, "items", "Brasil;>…;Portugal;>…").unwrap();
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    ui.on(t, "on_expand", move |ui, id, args| {
        let EventArgs::Text(text) = args else { return };
        l.borrow_mut().push(text.clone());
        let tree = ui.get_mut::<TreeView>(id).unwrap();
        let path = tree.last_toggled().unwrap();
        let kids = match text.as_str() {
            "Brasil" => vec![TreeNode::new("São Paulo").with_children(vec![TreeNode::new("…")]), TreeNode::new("Rio")],
            "São Paulo" => vec![TreeNode::new("Campinas")],
            _ => vec![TreeNode::new("Lisboa")],
        };
        tree.set_children(&path, kids);
    });
    TreeView::full_expand(&mut ui, t);
    assert_eq!(*log.borrow(), ["Brasil", "São Paulo", "Portugal"], "um evento por nó, os carregados também");
    let tree = ui.get::<TreeView>(t).unwrap();
    assert_eq!(tree.nodes()[0].children[0].children[0].text, "Campinas");
    assert!(tree.nodes()[0].expanded && tree.nodes()[0].children[0].expanded && tree.nodes()[1].expanded);
    // E fechar tudo: os de dentro antes dos pais.
    let closed = Rc::new(RefCell::new(Vec::new()));
    let c = closed.clone();
    ui.on(t, "on_collapse", move |_, _, args| {
        if let EventArgs::Text(text) = args {
            c.borrow_mut().push(text.clone());
        }
    });
    TreeView::full_collapse(&mut ui, t);
    assert_eq!(*closed.borrow(), ["São Paulo", "Brasil", "Portugal"]);
    // Pelo `invoke` (como as outras linguagens chamam): abre logo depois, pela fila.
    assert!(ui.get::<TreeView>(t).unwrap().methods().iter().any(|m| m.name == "full_expand"));
    log.borrow_mut().clear();
    ui.invoke(t, "full_expand", &[]).unwrap();
    ui.process_posted();
    assert_eq!(*log.borrow(), ["Brasil", "São Paulo", "Portugal"]);
    ui.invoke(t, "full_collapse", &[]).unwrap();
    ui.process_posted();
    assert!(!ui.get::<TreeView>(t).unwrap().nodes()[0].expanded);
}

/// A barra de rolagem das listas se arrasta com o mouse (o Inspetor do app tinha só a roda).
#[test]
fn the_scroll_bar_of_a_long_tree_can_be_dragged() {
    let (mut ui, f) = setup();
    let t = ui.add(TreeView::default(), f);
    place(&mut ui, t, 0.0, 0.0, 240.0, 300.0);
    let items: Vec<String> = (0..100).map(|i| format!("Item {i}")).collect();
    ui.set_prop(t, "items", items.join(";")).unwrap();
    paint(&mut ui, f);
    // Pega a barra no alto e arrasta até embaixo (passando da lista: o arraste continua).
    ui.pointer_down(f, at(234.0, 12.0));
    ui.pointer_move(f, at(234.0, 200.0));
    ui.pointer_move(f, at(234.0, 500.0));
    ui.pointer_up(f, at(234.0, 500.0));
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "", "clicar na barra não escolhe item");
    paint(&mut ui, f);
    // Rolou até o fim: a última linha visível é o último item.
    ui.pointer_down(f, at(40.0, 290.0));
    ui.pointer_up(f, at(40.0, 290.0));
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "Item 99");
    // Clicar na faixa da barra, fora dela, faz a barra pular para lá (o topo).
    ui.pointer_down(f, at(234.0, 2.0));
    ui.pointer_up(f, at(234.0, 2.0));
    paint(&mut ui, f);
    ui.pointer_down(f, at(40.0, 10.0));
    ui.pointer_up(f, at(40.0, 10.0));
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "Item 0");
}

/// Uma árvore rolada para baixo que encolhe (os nós trocados por menos, como a documentação
/// ao abrir uma página): a rolagem cai para o novo máximo também para o mouse. Antes, a
/// pintura usava a rolagem limitada e o mouse a velha, e o hover e o clique ficavam algumas
/// linhas abaixo do ponteiro.
#[test]
fn a_tree_that_shrinks_keeps_the_mouse_on_the_row_under_it() {
    let (mut ui, f) = setup();
    let t = ui.add(TreeView::default(), f);
    place(&mut ui, t, 0.0, 0.0, 240.0, 300.0);
    let items: Vec<String> = (0..60).map(|i| format!("Item {i}")).collect();
    ui.set_prop(t, "items", items.join(";")).unwrap();
    paint(&mut ui, f);
    // Rola até embaixo.
    for _ in 0..20 {
        ui.wheel(f, aylis_core::WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -120.0), ..Default::default() });
    }
    paint(&mut ui, f);
    // A árvore encolhe para caber sem rolagem; a primeira linha à vista é a do topo.
    let few: Vec<String> = (0..5).map(|i| format!("Novo {i}")).collect();
    ui.set_prop(t, "items", few.join(";")).unwrap();
    paint(&mut ui, f);
    ui.pointer_down(f, at(40.0, 10.0));
    ui.pointer_up(f, at(40.0, 10.0));
    assert_eq!(ui.get::<TreeView>(t).unwrap().selected_path(), "Novo 0", "o clique pega a linha sob o mouse");
}

#[test]
fn tab_control_shows_one_page() {
    let (mut ui, f) = setup();
    let tc = ui.add(TabControl::default(), f);
    place(&mut ui, tc, 0.0, 0.0, 400.0, 300.0);
    let a = TabControl::add_tab(&mut ui, tc, "Dados");
    let b = TabControl::add_tab(&mut ui, tc, "Endereço");
    paint(&mut ui, f);
    assert!(ui.control(a).unwrap().visible() && !ui.control(b).unwrap().visible());
    ui.set_prop(tc, "tab_index", 1i64).unwrap();
    assert!(!ui.control(a).unwrap().visible() && ui.control(b).unwrap().visible());
    // clique na primeira aba
    ui.pointer_down(f, at(20.0, 15.0));
    assert!(ui.control(a).unwrap().visible());
    assert_eq!(ui.get::<TabControl>(tc).unwrap().tab_index, 0);
}

#[test]
fn expander_collapses_to_the_header() {
    let (mut ui, f) = setup();
    let e = ui.add(Expander::new("Mais"), f);
    place(&mut ui, e, 0.0, 0.0, 260.0, 200.0);
    let inside = ui.add(Button::new("x"), e);
    place(&mut ui, inside, 10.0, 60.0, 100.0, 30.0);
    paint(&mut ui, f);
    ui.pointer_down(f, at(20.0, 20.0));
    assert_eq!(ui.control(e).unwrap().height(), 40.0);
    assert!(!ui.control(inside).unwrap().visible());
    ui.set_prop(e, "expanded", true).unwrap();
    assert_eq!(ui.control(e).unwrap().height(), 200.0);
    assert!(ui.control(inside).unwrap().visible());
}

#[test]
fn timer_fires_on_its_interval_and_stops() {
    let (mut ui, f) = setup();
    let t = ui.add(Timer::new(100), f);
    let ticks = Rc::new(RefCell::new(0));
    let k = ticks.clone();
    ui.on(t, "on_timer", move |_, _, _| *k.borrow_mut() += 1);
    ui.set_time(0.05);
    assert!(!ui.run_timers());
    assert_eq!(ui.next_wake(), Some(0.1));
    ui.set_time(0.1);
    ui.run_timers();
    ui.set_time(0.2);
    ui.run_timers();
    assert_eq!(*ticks.borrow(), 2);
    Timer::set_enabled(&mut ui, t, false);
    ui.set_time(0.5);
    ui.run_timers();
    assert_eq!(*ticks.borrow(), 2);
    assert_eq!(ui.next_wake(), None, "parado: a plataforma pode dormir");
}

#[test]
fn long_press_fires_and_suppresses_the_click() {
    let (mut ui, f) = setup();
    let b = ui.add(Button::new("segure"), f);
    place(&mut ui, b, 0.0, 0.0, 100.0, 40.0);
    paint(&mut ui, f);
    let log = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_click", "on_long_press"] {
        let l = log.clone();
        ui.on(b, event, move |_, _, _| l.borrow_mut().push(event));
    }
    ui.set_time(1.0);
    ui.pointer_down(f, at(10.0, 10.0));
    ui.set_time(1.6);
    ui.run_timers();
    ui.pointer_up(f, at(10.0, 10.0));
    // um clique normal continua clicando
    ui.set_time(3.0);
    ui.pointer_down(f, at(10.0, 10.0));
    ui.set_time(3.1);
    ui.run_timers();
    ui.pointer_up(f, at(10.0, 10.0));
    assert_eq!(*log.borrow(), ["on_long_press", "on_click"]);
}

#[test]
fn message_dlg_returns_the_button() {
    let (mut ui, f) = setup();
    ui.show(f).unwrap();
    let result = Rc::new(RefCell::new(None));
    let r = result.clone();
    let dlg = message_dlg(&mut ui, "Excluir?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], move |_, res| {
        *r.borrow_mut() = Some(res);
    });
    assert!(ui.is_modal(dlg));
    ui.set_modal_result(dlg, ModalResult::Yes);
    assert_eq!(*result.borrow(), Some(ModalResult::Yes));
    assert!(!ui.contains(dlg), "liberado ao fechar");
    assert_eq!(parse_filter("Imagens|*.png;*.jpg|Todos|*.*"), vec![("Imagens".to_string(), vec!["png".to_string(), "jpg".to_string()])]);
}

#[test]
fn message_dialog_is_a_component_with_on_close() {
    let (mut ui, f) = setup();
    ui.show(f).unwrap();
    // Como no Object Inspector: texto, tipo, título e botões por propriedade.
    let d = ui.add(MessageDialog::default(), f);
    ui.set_prop(d, "text", "Salvar as alterações?").unwrap();
    ui.set_prop(d, "kind", "confirmation").unwrap();
    ui.set_prop(d, "title", "Pedidos").unwrap();
    ui.set_prop(d, "buttons", "yes, no, cancel, xyz").unwrap();
    assert_eq!(ui.get::<MessageDialog>(d).unwrap().button_list(), [ModalResult::Yes, ModalResult::No, ModalResult::Cancel]);
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    ui.on(d, "on_close", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            l.borrow_mut().push(t.clone());
        }
    });
    let box_form = MessageDialog::execute(&mut ui, d).unwrap();
    assert!(ui.is_modal(box_form));
    assert_eq!(ui.get_prop(box_form, "caption").ok(), Some(PropValue::String("Pedidos".into())));
    ui.set_modal_result(box_form, ModalResult::No);
    assert_eq!(*log.borrow(), ["no"]);
    assert!(!ui.contains(box_form) && ui.contains(d), "a caixa sai; o componente fica");
    // execute_then: a resposta também no lugar em que abriu.
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    let box_form = MessageDialog::execute_then(&mut ui, d, move |_, r| *g.borrow_mut() = Some(r)).unwrap();
    ui.set_modal_result(box_form, ModalResult::Yes);
    assert_eq!(*got.borrow(), Some(ModalResult::Yes));
    assert_eq!(*log.borrow(), ["no", "yes"]);
}

/// Os componentes abaixo de `id` (ele incluso).
fn subtree(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

/// A caixa de mensagem fica do tamanho do texto; o texto enorme (a caixa de pânico) para na
/// altura máxima e rola.
#[test]
fn the_message_box_fits_the_text_and_scrolls_a_long_one() {
    let (mut ui, f) = setup();
    ui.show(f).unwrap();
    let size = |ui: &Ui, id: ComponentId| {
        let cb = ui.control(id).unwrap();
        (cb.width(), cb.height())
    };
    let has_scroll = |ui: &Ui, id: ComponentId| subtree(ui, id).into_iter().any(|c| ui.class_name(c) == Some("ScrollBox"));

    let short = message_dlg(&mut ui, "Salvo.", DlgKind::Information, &[ModalResult::Ok], |_, _| {});
    let (sw, sh) = size(&ui, short);
    assert!(sw < 440.0 && sh < 160.0, "a caixa de um texto curto é pequena: {sw} x {sh}");
    assert!(!has_scroll(&ui, short));
    ui.set_modal_result(short, ModalResult::Ok);

    let lines: Vec<String> = (0..300).map(|i| format!("   {i}: aylis_core::ui::Ui::fire at crates/aylis-core/src/ui.rs")).collect();
    let long = message_dlg(&mut ui, &lines.join("\n"), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
    let (lw, lh) = size(&ui, long);
    assert!(lw > sw, "o texto mais largo alarga a caixa: {lw}");
    assert!(lh <= 520.0, "a altura tem um máximo: {lh}");
    assert!(has_scroll(&ui, long), "o texto que passa da altura rola");
    ui.set_modal_result(long, ModalResult::Ok);
}

/// O X do ícone de erro é desenhado no centro exato do círculo, e o círculo ocupa o ícone.
#[test]
fn the_error_icon_cross_is_centered_in_the_circle() {
    let (mut ui, f) = setup();
    ui.show(f).unwrap();
    let dlg = message_dlg(&mut ui, "Falhou.", DlgKind::Error, &[ModalResult::Ok], |_, _| {});
    ui.layout();
    let mut canvas = aylis_core::pages::RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(dlg, &mut canvas);
    use aylis_core::pages::Op;
    let circle = canvas.ops.iter().find_map(|op| if let Op::FillEllipse { rect, .. } = op { Some(*rect) } else { None }).expect("o círculo");
    let lines: Vec<(PointF, PointF)> = canvas.ops.iter().filter_map(|op| if let Op::Line { from, to, .. } = op { Some((*from, *to)) } else { None }).collect();
    assert_eq!(lines.len(), 2, "o X em dois traços: {lines:?}");
    let c = circle.center();
    for (a, b) in lines {
        let mid = PointF::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
        assert!((mid.x - c.x).abs() < 0.01 && (mid.y - c.y).abs() < 0.01, "o traço fora do centro: {mid:?} x {c:?}");
    }
    assert!((circle.width - 36.0).abs() < 0.01, "o círculo ocupa o ícone: {circle:?}");
    ui.set_modal_result(dlg, ModalResult::Ok);
}

/// O anel de foco do botão fica afastado do botão por uma folga (o fundo de trás aparece),
/// em todos os temas: contrasta mesmo quando tem a cor do botão (o primário no Aylis Light).
#[test]
fn the_button_focus_ring_has_a_gap_in_every_theme() {
    use aylis_core::pages::{Op, RecordingCanvas};
    let mut with_ring = Vec::new();
    let aylis = [aylis_core::Theme::light(), aylis_core::Theme::dark()];
    for theme in aylis.iter().chain(aylis_core::themes::all()) {
        let (mut ui, f) = setup();
        ui.set_theme(theme.clone());
        let b = ui.add(Button::new("Salvar"), f);
        place(&mut ui, b, 20.0, 20.0, 120.0, 36.0);
        ui.layout();
        ui.set_focus(b);
        let mut canvas = RecordingCanvas::new(None);
        ui.request_repaint();
        ui.paint(f, &mut canvas);
        let button = RectF::new(20.0, 20.0, 120.0, 36.0);
        let ring = canvas.ops.iter().find_map(|op| match op {
            Op::StrokeRect { rect, width, .. } if *rect == button => Some(*width),
            _ => None,
        });
        let Some(width) = ring else { continue };
        with_ring.push(theme.name.clone());
        let fill = canvas
            .ops
            .iter()
            .find_map(|op| match op {
                Op::FillRect { rect, .. } if rect.x > button.x && rect.right() < button.right() && rect.height > button.height / 2.0 => Some(*rect),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{}: o fundo do botão", theme.name));
        let gap = fill.x - button.x - width / 2.0;
        assert!(gap >= width - 0.01, "{}: sem folga entre o anel ({width}) e o botão ({gap})", theme.name);
    }
    assert!(with_ring.iter().any(|n| n == "Aylis Light"), "o Aylis Light não pintou o anel: {with_ring:?}");
}

#[test]
fn chart_reads_series_from_text() {
    let (mut ui, f) = setup();
    let c = ui.add(Chart::default(), f);
    ui.set_prop(c, "series", "A=1,2,3|B=4,5").unwrap();
    ui.set_prop(c, "kind", "bar").unwrap();
    let chart = ui.get::<Chart>(c).unwrap();
    assert_eq!(chart.data().len(), 2);
    assert_eq!(chart.data()[1].values, [4.0, 5.0]);
    place(&mut ui, c, 0.0, 0.0, 400.0, 300.0);
    paint(&mut ui, f);
    ui.pointer_move(f, at(200.0, 150.0));
    paint(&mut ui, f);
}

#[test]
fn memo_wraps_while_editing_and_navigates_the_visual_rows() {
    let (mut ui, f) = setup();
    let m = ui.add(Memo::new("um dois tres quatro cinco seis sete oito nove dez"), f);
    ui.set_prop(m, "wrap", true).unwrap();
    place(&mut ui, m, 10.0, 10.0, 120.0, 200.0);
    ui.set_focus(m);
    paint(&mut ui, f);
    let ctrl = Modifiers { ctrl: true, ..Default::default() };
    ui.key_down(f, KeyEvent { key: Key::Home, modifiers: ctrl, ..Default::default() });
    // Fim da linha visual: antes do fim do texto (a linha foi quebrada).
    ui.key_down(f, key(Key::End));
    let (line, col) = ui.get::<Memo>(m).unwrap().caret_position();
    assert_eq!(line, 0);
    assert!(col > 0 && col < 20, "fim da primeira linha visual: {col}");
    // Descer fica na mesma linha do texto, mais adiante.
    ui.key_down(f, key(Key::Home));
    ui.key_down(f, key(Key::Down));
    let (line, col2) = ui.get::<Memo>(m).unwrap().caret_position();
    assert_eq!(line, 0);
    assert!(col2 > col, "desceu para a segunda linha visual: {col2}");
    ui.key_down(f, key(Key::Up));
    assert_eq!(ui.get::<Memo>(m).unwrap().caret_position(), (0, 0));
    // Sem quebra, descer numa linha só vai para o fim.
    ui.set_prop(m, "wrap", false).unwrap();
    ui.key_down(f, KeyEvent { key: Key::Home, modifiers: ctrl, ..Default::default() });
    ui.key_down(f, key(Key::End));
    assert_eq!(ui.get::<Memo>(m).unwrap().caret_position(), (0, 49));
}

#[test]
fn text_settings_are_properties() {
    let (mut ui, f) = setup();
    let e = ui.add(Edit::new("abc"), f);
    ui.set_prop(e, "h_align", PropValue::Enum("center".into())).unwrap();
    ui.set_prop(e, "trimming", PropValue::Enum("word".into())).unwrap();
    ui.set_prop(e, "wrap", true).unwrap();
    let edit = ui.get::<Edit>(e).unwrap();
    assert_eq!((edit.h_align, edit.trimming, edit.wrap), (TextAlign::Center, Trimming::Word, true));
    let l = ui.add(Label::new("x"), f);
    ui.set_prop(l, "trimming", PropValue::Enum("none".into())).unwrap();
    assert_eq!(ui.get::<Label>(l).unwrap().trimming, Trimming::None);
    let c = ui.add(ComboBox::default(), f);
    ui.set_prop(c, "h_align", PropValue::Enum("trailing".into())).unwrap();
    assert_eq!(ui.get::<ComboBox>(c).unwrap().h_align, TextAlign::Trailing);
    let n = ui.add(NumberBox::new(1.0, 0.0, 9.0), f);
    ui.set_prop(n, "h_align", PropValue::Enum("trailing".into())).unwrap();
    assert_eq!(ui.get::<NumberBox>(n).unwrap().edit.h_align, TextAlign::Trailing, "repassada ao Edit interno");
    paint(&mut ui, f);
}

#[test]
fn clicking_a_long_edit_puts_the_caret_where_it_was_clicked() {
    let (mut ui, f) = setup();
    let e = ui.add(Edit::new("Entregar o pedido pela manhã e conferir o CEP antes de sair"), f);
    let other = ui.add(Button::new("ok"), f);
    place(&mut ui, e, 10.0, 10.0, 200.0, 30.0);
    place(&mut ui, other, 10.0, 100.0, 80.0, 30.0);
    ui.set_focus(e);
    paint(&mut ui, f);
    ui.set_focus(other); // o cursor ficou no fim do texto, rolado
    paint(&mut ui, f);
    // Clica perto da borda esquerda: o texto aparece do começo, o cursor vai para lá.
    ui.pointer_down(f, at(16.0, 25.0));
    ui.pointer_up(f, at(16.0, 25.0));
    let (a, b) = ui.get::<Edit>(e).unwrap().selection();
    assert!(a == b && a <= 1, "cursor no começo, não no trecho rolado: {a}");
}

#[test]
fn menu_reports_the_clicked_item_and_auto_checks() {
    let (mut ui, f) = setup();
    ui.show(f).unwrap();
    let menu = ui.add(PopupMenu::default(), f);
    let bold = ui.add(MenuItem::new("Negrito").with_shortcut("Ctrl+B"), menu);
    ui.set_name(bold, "miNegrito").unwrap();
    ui.set_prop(bold, "auto_check", true).unwrap();
    ui.set_prop(bold, "icon", "format_bold").unwrap();
    let clicked = Rc::new(RefCell::new(Vec::new()));
    let c = clicked.clone();
    ui.on(menu, "on_item_click", move |ui, _, args| {
        if let EventArgs::Value(PropValue::Component(Some(item))) = args {
            c.borrow_mut().push(ui.name(*item).unwrap_or_default().to_string());
        }
    });
    let item_clicks = Rc::new(RefCell::new(0));
    let ic = item_clicks.clone();
    ui.on_click(bold, move |_, _| *ic.borrow_mut() += 1);
    let key = KeyEvent { key: Key::Char('b'), modifiers: Modifiers { ctrl: true, ..Default::default() }, repeat: false };
    ui.key_down(f, key);
    ui.process_events();
    assert_eq!(*item_clicks.borrow(), 1, "o on_click do item");
    assert_eq!(*clicked.borrow(), ["miNegrito"], "o on_item_click do menu, com o item");
    assert!(ui.get::<MenuItem>(bold).unwrap().checked, "auto_check marcou");
    ui.key_down(f, key);
    ui.process_events();
    assert!(!ui.get::<MenuItem>(bold).unwrap().checked, "e desmarcou");
}

#[test]
fn joystick_follows_the_drag_and_springs_back() {
    let (mut ui, f) = setup();
    let j = ui.add(aylis_controls::Joystick::default(), f);
    place(&mut ui, j, 0.0, 0.0, 140.0, 140.0);
    let log = changes(&mut ui, j);
    paint(&mut ui, f);
    let value = |ui: &Ui| (ui.get_prop(j, "x").unwrap().to_f64().unwrap(), ui.get_prop(j, "y").unwrap().to_f64().unwrap());
    // No centro (dentro da zona morta) não muda.
    ui.pointer_down(f, at(72.0, 70.0));
    assert_eq!(value(&ui), (0.0, 0.0));
    // Para a direita, além da borda: x = 1.
    ui.pointer_move(f, at(300.0, 70.0));
    assert_eq!(value(&ui), (1.0, 0.0));
    // Para baixo e à esquerda, na diagonal: o comprimento fica em 1.
    ui.pointer_move(f, at(-200.0, 340.0));
    let (x, y) = value(&ui);
    assert!(x < 0.0 && y > 0.0 && ((x * x + y * y).sqrt() - 1.0).abs() < 1e-3, "{x} {y}");
    ui.pointer_up(f, at(-200.0, 340.0));
    assert_eq!(value(&ui), (0.0, 0.0));
    assert_eq!(log.borrow().len(), 3);
}
