//! A IDE com mouse e teclado simulados.

use std::path::PathBuf;

use aylis_controls::{Edit, Form};
use aylis_core::{ComponentId, Key, KeyEvent, Modifiers, ModalResult, PointF, PointerEvent, PropKind, PropValue, Ui};
use aylis_ide::document::{AlignOp, Document};
use aylis_ide::state::shared;
use aylis_ide::{actions, app, inspector};
use serde_json::{Value, json};

fn text_of(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

fn click(ui: &mut Ui, root: ComponentId, p: PointF) {
    let e = PointerEvent { position: p, ..Default::default() };
    ui.pointer_move(root, e);
    ui.pointer_down(root, e);
    ui.pointer_up(root, e);
}

fn center_of(ui: &Ui, id: ComponentId) -> PointF {
    let r = ui.absolute_rect(id).unwrap();
    PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0)
}

/// A linha (de um painel) cujo texto é `text`.
fn row_with(ui: &Ui, panel: ComponentId, text: &str) -> ComponentId {
    ui.children(panel)
        .iter()
        .copied()
        .find(|&r| ui.children(r).iter().any(|&l| text_of(ui, l) == text))
        .unwrap_or_else(|| panic!("linha '{text}' não encontrada"))
}

fn ctrl(key: Key) -> KeyEvent {
    KeyEvent { key, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() }
}

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn inspector_parses_typed_text() {
    assert_eq!(inspector::parse(PropKind::Float, "12,5").unwrap(), json!(12.5));
    assert_eq!(inspector::parse(PropKind::Int, "7").unwrap(), json!(7));
    assert!(inspector::parse(PropKind::Int, "sete").is_err());
    assert_eq!(inspector::parse(PropKind::Bounds, "8").unwrap(), json!(8.0));
    assert_eq!(inspector::parse(PropKind::Bounds, "4, 8").unwrap(), json!([4.0, 8.0]));
    assert_eq!(inspector::parse(PropKind::Bounds, "1 2 3 4").unwrap(), json!([1.0, 2.0, 3.0, 4.0]));
    assert!(inspector::parse(PropKind::Bounds, "1, 2, 3").is_err());
    assert_eq!(inspector::parse(PropKind::Color, "").unwrap(), Value::Null);
    assert_eq!(inspector::parse(PropKind::String, "").unwrap(), json!(""));
}

#[test]
fn place_a_button_and_edit_it_in_the_inspector() {
    let mut ide = app::build(shared(Document::new()));
    // A paleta só aparece fora da tela inicial (o "Novo form solto").
    ide.state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &ide.state.clone(), &ide.ids.clone());
    let form = ide.form;
    ide.ui.layout();

    // paleta → Button; depois clique no form do designer
    let button_row = row_with(&ide.ui, ide.ids.palette, "Button");
    let p = center_of(&ide.ui, button_row);
    click(&mut ide.ui, form, p);
    assert_eq!(ide.state.borrow().pending.as_deref(), Some("Button"));
    let s = ide.ui.absolute_rect(ide.ids.surface).unwrap();
    click(&mut ide.ui, form, PointF::new(s.x + 28.0 + 40.0, s.y + 28.0 + 40.0));
    let button = ide.state.borrow().doc.ui.find("Button1").expect("Button1 criado");
    assert_eq!(ide.state.borrow().doc.selection, Some(button));
    assert_eq!(ide.ui.children(ide.ids.structure).len(), 2, "Form1 + Button1 na estrutura");

    // inspector: busca "text"; a linha tem um Edit; digita e confirma com Enter
    ide.state.borrow_mut().inspector_filter = "text".into();
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    ide.ui.layout();
    let line = ide
        .ui
        .children(ide.ids.inspector)
        .iter()
        .copied()
        // O rótulo mostra o nome legível; o técnico fica na dica.
        .find(|&l| ide.ui.children(l).first().is_some_and(|&lbl| matches!(ide.ui.get_prop(lbl, "hint"), Ok(PropValue::String(h)) if h == "text")))
        .expect("linha 'text' no inspector");
    let edit = *ide.ui.children(line).last().unwrap();
    let p = center_of(&ide.ui, edit);
    click(&mut ide.ui, form, p);
    for _ in 0..10 {
        ide.ui.key_down(form, KeyEvent { key: Key::Backspace, ..Default::default() });
    }
    ide.ui.text_input(form, "Salvar");
    ide.ui.key_down(form, KeyEvent { key: Key::Enter, ..Default::default() });
    assert_eq!(text_of(&ide.state.borrow().doc.ui, button), "Salvar");
    assert!(ide.state.borrow().doc.modified);

    // Ctrl+Z pelo menu Editar volta o texto
    ide.ui.set_focus(ide.ids.surface);
    ide.ui.key_down(form, ctrl(Key::Char('z')));
    let st = ide.state.borrow();
    let b = st.doc.ui.find("Button1").unwrap();
    assert_eq!(text_of(&st.doc.ui, b), "Button1");
}

#[test]
fn closing_the_last_tab_leaves_no_form_open() {
    let base = temp("sem_form");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(strip_titles(&ide).len(), 1);
    // Fechar a única aba: nenhuma aba, nada na estrutura nem no inspetor (antes vinha um Form1).
    actions::close_tab(&mut ide.ui, &state, &ide.ids, 0);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(state.borrow().no_form);
    assert!(strip_titles(&ide).is_empty(), "sem aba de form novo");
    assert!(ide.ui.children(ide.ids.structure).is_empty());
    // Abrir um form do projeto: volta a ter a aba, só ela.
    actions::open_project_form(&state, "frmPrincipal");
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let st = state.borrow();
    assert!(!st.no_form);
    assert_eq!((st.tab_count(), st.doc.form_name().as_str()), (1, "frmPrincipal"));
    drop(st);
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_project_with_two_forms_tabs_and_save_all() {
    let base = temp("projeto");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap(); // como o run faz
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(state.borrow().doc.form_name(), "frmPrincipal");
    assert!(state.borrow().project.is_some());

    // Projeto > Novo form: o diálogo (um form modal da própria IDE) pede o nome
    actions::new_form(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    assert!(ide.ui.is_modal(dialog));
    assert!(ide.ui.input_blocked(form), "a IDE espera o diálogo");
    let edit = ide.ui.focused(dialog).expect("o campo do nome tem o foco");
    ide.ui.get_mut::<Edit>(edit).unwrap().text = "frmClientes".into();
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert!(!ide.ui.contains(dialog), "o diálogo é liberado ao fechar");
    {
        let st = state.borrow();
        assert_eq!(st.doc.form_name(), "frmClientes", "o form novo abre numa aba");
        assert_eq!(st.tab_count(), 2);
        assert!(st.project.as_ref().unwrap().form("frmClientes").is_some());
    }
    assert!(base.join("Loja/forms/clientes.json").is_file());

    // edita o form novo, volta para a primeira aba e salva tudo
    state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    state.borrow_mut().switch_to(0);
    assert_eq!(state.borrow().doc.form_name(), "frmPrincipal");
    assert!(state.borrow().any_modified());
    assert_eq!(actions::save_all(&state), 0);
    assert!(!state.borrow().any_modified());
    let saved = std::fs::read_to_string(base.join("Loja/forms/clientes.json")).unwrap();
    assert!(saved.contains("Button1"));

    // gerar código: um arquivo por form, com handlers qualificados pelo form
    let files = actions::generate_code(&mut ide.ui, &state).unwrap();
    assert!(files.iter().any(|f| f.ends_with("clientes.rs")));
    drop(ide);
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn multi_selection_aligns_and_moves_together() {
    let mut doc = Document::new();
    let a = doc.add("Button", Some(PointF::new(16.0, 16.0))).unwrap();
    let b = doc.add("Button", Some(PointF::new(80.0, 120.0))).unwrap();
    let c = doc.add("Button", Some(PointF::new(300.0, 64.0))).unwrap();
    doc.select_many(vec![b, c, a]); // a é o principal (o último)
    doc.align_selection(AlignOp::Left).unwrap();
    let left = |doc: &Document, id| doc.ui.control(id).unwrap().left();
    assert_eq!((left(&doc, b), left(&doc, c)), (left(&doc, a), left(&doc, a)));
    doc.align_selection(AlignOp::DistributeV).unwrap();

    // Ctrl+clique tira um da seleção; o arrasto move os que sobraram juntos
    doc.toggle(c);
    assert_eq!(doc.selected, vec![b, a]);
    let items: Vec<_> = doc.selected.iter().map(|&id| (id, doc.start_rect(id).unwrap())).collect();
    doc.move_group(&items, PointF::new(40.0, 0.0));
    assert_eq!(left(&doc, a) - items[1].1.x, 40.0);
    assert_eq!(left(&doc, b) - items[0].1.x, 40.0);

    // desfazer volta tudo (um passo por operação)
    assert!(doc.undo());
    let _ = Form::default();
}

#[test]
fn the_window_and_the_panels_come_back_where_they_were() {
    let mut ide = app::build(shared(Document::new()));
    ide.ui.layout();
    // O Object Inspector à esquerda, a paleta à direita.
    let left = ide.ui.parent(ide.ids.inspector).unwrap();
    let right = ide.ui.parent(ide.ids.palette).unwrap();
    ide.ui.control_mut(left).unwrap().set_width(300.0);
    ide.ui.control_mut(right).unwrap().set_width(420.0);
    ide.ui.control_mut(ide.ids.structure).unwrap().set_height(250.0);
    // A plataforma conta onde a janela está (aqui, à mão).
    let placement = aylis_core::WindowPlacement { x: 120, y: 80, width: 1600, height: 1000, maximized: true };
    ide.ui.set_window_placement(ide.form, placement);
    let saved = actions::layout_json(&ide.ui, &ide.ids);
    assert_eq!(saved["panels"]["inspector_side"], 300.0);
    assert_eq!(saved["window"]["maximized"], true);

    // Uma IDE nova, como no próximo início.
    let mut next = app::build(shared(Document::new()));
    actions::apply_layout(&mut next.ui, &next.ids, &saved);
    next.ui.layout();
    let w = |ui: &aylis_core::Ui, id| ui.control(id).unwrap().width();
    assert_eq!(w(&next.ui, next.ui.parent(next.ids.inspector).unwrap()), 300.0);
    assert_eq!(w(&next.ui, next.ui.parent(next.ids.palette).unwrap()), 420.0);
    assert_eq!(next.ui.control(next.ids.structure).unwrap().height(), 250.0);
    assert_eq!(next.ui.window_placement(next.form), Some(placement));
    // Valores absurdos (outra tela, arquivo mexido) ficam de fora.
    let mut other = app::build(shared(Document::new()));
    actions::apply_layout(&mut other.ui, &other.ids, &serde_json::json!({ "panels": { "inspector_side": 5.0, "palette_side": "x" } }));
    assert_eq!(w(&other.ui, other.ui.parent(other.ids.inspector).unwrap()), 350.0);
}

#[test]
fn the_palette_shows_where_platform_dependent_components_work() {
    let mut ide = app::build(shared(Document::new()));
    ide.ui.layout();
    let icons = |ui: &Ui, row: ComponentId| -> Vec<String> {
        ui.children(row)
            .iter()
            .filter_map(|&c| match ui.get_prop(c, "icon") {
                Ok(PropValue::String(s)) => Some(s),
                _ => None,
            })
            .collect()
    };
    let tray = row_with(&ide.ui, ide.ids.palette, "TrayIcon");
    // Não visual: o ícone dele à esquerda e os das plataformas à direita.
    assert!(icons(&ide.ui, tray).contains(&"pin".to_string()));
    let mut got: Vec<String> = icons(&ide.ui, tray).into_iter().filter(|i| i.starts_with("platform:")).collect();
    got.reverse();
    assert_eq!(got, ["platform:desktop_windows", "platform:laptop_mac", "platform:terminal"]);
    assert!(matches!(ide.ui.get_prop(tray, "hint"), Ok(PropValue::String(h)) if h.contains("Windows, macOS, Linux")));
    assert!(ide.ui.icon("platform:android").is_some(), "o pacote de ícones das plataformas");
    // Um componente de todas as plataformas não tem ícone.
    let button = row_with(&ide.ui, ide.ids.palette, "Button");
    assert!(icons(&ide.ui, button).is_empty());
}

#[test]
fn quitting_with_changes_asks_with_the_aylis_message_dialog() {
    let mut ide = app::build(shared(Document::new()));
    ide.ui.show(ide.form).unwrap();
    ide.state.borrow_mut().doc.modified = true;
    let open_box = |ui: &Ui| ui.roots().iter().copied().find(|&r| ui.is_modal(r));
    let dialogs = |ui: &Ui| ui.components().filter(|&c| ui.class_name(c) == Some("MessageDialog")).count();

    // "Não": a IDE fica aberta; a caixa e o componente saem.
    actions::quit(&mut ide.ui, &ide.state, &ide.ids);
    let b = open_box(&ide.ui).expect("a pergunta é um form do aylis, não a caixa do sistema");
    assert_eq!(dialogs(&ide.ui), 1, "a pergunta é um componente MessageDialog");
    ide.ui.set_modal_result(b, ModalResult::No);
    assert!(!ide.state.borrow().quit_confirmed && ide.ui.is_open(ide.form));
    assert_eq!(dialogs(&ide.ui), 0);

    // "Sim": fecha.
    actions::quit(&mut ide.ui, &ide.state, &ide.ids);
    let b = open_box(&ide.ui).unwrap();
    ide.ui.set_modal_result(b, ModalResult::Yes);
    assert!(ide.state.borrow().quit_confirmed);
    assert!(!ide.ui.is_open(ide.form));
}

#[test]
fn a_menu_shortcut_does_not_wipe_the_ide() {
    // Ctrl+Shift+N (Novo projeto) e cancelar: o item foi criado antes do "Recentes" e,
    // com a cópia dos ids daquele momento, o refresh apagava os painéis do form.
    let mut ide = app::build(shared(Document::new()));
    ide.ui.show(ide.form).unwrap();
    let before = ide.ui.children(ide.form).to_vec();
    let key = KeyEvent { key: Key::Char('n'), modifiers: Modifiers { ctrl: true, shift: true, ..Default::default() }, repeat: false };
    ide.ui.key_down(ide.form, key);
    let dialog = ide.ui.roots().iter().copied().find(|&r| ide.ui.is_modal(r)).expect("abriu o Novo projeto");
    assert_eq!(ide.ui.children(ide.form), before.as_slice(), "os painéis continuam com o diálogo aberto");
    ide.ui.set_modal_result(dialog, ModalResult::Cancel);
    ide.ui.key_up(ide.form, key);
    assert_eq!(ide.ui.children(ide.form), before.as_slice(), "e depois de cancelar");
    // O "Recentes" do menu recebe a lista, não o form.
    assert!(!ide.ui.children(ide.ids.mi_recent).is_empty());
}

#[test]
fn closing_the_shadow_editor_by_the_x_keeps_the_ide() {
    // Gravação do usuário: "…" da sombra no Object Inspector, fechar pelo X da janela,
    // uma tecla e um clique no designer; a IDE ficou só com o form.
    let mut ide = app::build(shared(Document::new()));
    ide.ui.show(ide.form).unwrap();
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    let before = ide.ui.children(ide.form).to_vec();
    aylis_ide::propeditors::shadow_editor(&mut ide.ui, "", Vec::new(), |_, _| {});
    let dialog = ide.ui.roots().iter().copied().find(|&r| ide.ui.is_modal(r)).expect("abriu a Sombra");
    assert!(ide.ui.close(dialog), "o X fecha");
    assert_eq!(ide.ui.children(ide.form), before.as_slice(), "depois do X");
    ide.ui.key_down(ide.form, KeyEvent { key: Key::Unknown, modifiers: Modifiers::default(), repeat: false });
    let press = PointerEvent { position: PointF::new(650.0, 407.0), ..Default::default() };
    ide.ui.pointer_down(ide.form, press);
    ide.ui.pointer_up(ide.form, press);
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    assert_eq!(ide.ui.children(ide.form), before.as_slice(), "depois da tecla e do clique");
}

#[test]
fn rotated_components_select_and_show_handles_where_they_appear() {
    let mut doc = Document::new();
    let b = doc.add("Button", Some(PointF::new(200.0, 180.0))).unwrap();
    doc.ui.control_mut(b).unwrap().set_rect(200.0, 180.0, 120.0, 40.0);
    doc.set_prop(b, "rotation", &json!(90)).unwrap();
    doc.ui.layout();
    let r = doc.rect_of(b).unwrap();
    let c = r.center();
    // Girado 90° em torno do centro: o canto de cima à esquerda vai para cima à direita.
    let tl = doc.rotated_point(b, r.origin());
    assert!((tl.x - (c.x + 20.0)).abs() < 0.01 && (tl.y - (c.y - 60.0)).abs() < 0.01, "{tl:?}");
    assert_eq!(doc.total_rotation(b), 90.0);
    // Clicar onde o botão aparece (acima do centro) seleciona; no retângulo sem giro, não.
    assert_eq!(doc.component_at(PointF::new(c.x, c.y - 50.0)), Some(b));
    assert_ne!(doc.component_at(PointF::new(c.x - 55.0, c.y)), Some(b));

    // Dentro de um painel girado: os dois giros somam.
    let p = doc.add("Panel", Some(PointF::new(380.0, 260.0))).unwrap();
    doc.ui.control_mut(p).unwrap().set_rect(380.0, 260.0, 200.0, 140.0);
    doc.set_prop(p, "rotation", &json!(15)).unwrap();
    doc.select(Some(p));
    let e = doc.add("Edit", Some(PointF::new(420.0, 300.0))).unwrap();
    assert_eq!(doc.ui.parent(e), Some(p));
    doc.set_prop(e, "rotation", &json!(20)).unwrap();
    doc.ui.layout();
    assert_eq!(doc.total_rotation(e), 35.0);
    let er = doc.rect_of(e).unwrap();
    let shown = doc.rotated_point(e, er.center());
    assert_eq!(doc.component_at(shown), Some(e), "o centro de onde o Edit aparece é dele");
}

#[test]
fn resizing_a_rotated_component_keeps_the_opposite_side_still() {
    use aylis_ide::document::Handle;
    let mut doc = Document::new();
    let b = doc.add("Button", Some(PointF::new(200.0, 180.0))).unwrap();
    doc.ui.control_mut(b).unwrap().set_rect(200.0, 180.0, 120.0, 40.0);
    doc.set_prop(b, "rotation", &json!(90)).unwrap();
    doc.ui.layout();
    let close = |a: PointF, b: PointF| (a.x - b.x).abs() < 0.6 && (a.y - b.y).abs() < 0.6;

    // Girado 90°, a alça "de cima" do botão aparece à direita na tela. Puxá-la para a
    // direita aumenta a altura (o lado de baixo, à esquerda na tela, fica parado).
    let start = doc.rect_of(b).unwrap();
    let still = doc.rotated_point(b, Handle::Bottom.point(start));
    let seen = doc.rotated_point(b, Handle::Top.point(start));
    assert!(seen.x > still.x + 30.0, "a alça de cima está à direita na tela");
    doc.resize_rotated(b, Handle::Top, start, PointF::new(16.0, 0.0));
    let now = doc.rect_of(b).unwrap();
    assert_eq!((now.width, now.height), (120.0, 56.0));
    assert!(close(doc.rotated_point(b, Handle::Bottom.point(now)), still), "o lado oposto não andou");

    // A alça "da direita" aparece embaixo: puxar para baixo aumenta a largura.
    let start = now;
    let still = doc.rotated_point(b, Handle::Left.point(start));
    doc.resize_rotated(b, Handle::Right, start, PointF::new(0.0, 24.0));
    let now = doc.rect_of(b).unwrap();
    assert_eq!((now.width, now.height), (144.0, 56.0));
    assert!(close(doc.rotated_point(b, Handle::Left.point(now)), still));

    // Dentro de um painel girado (o início do arrasto é relativo ao painel).
    let p = doc.add("Panel", Some(PointF::new(380.0, 260.0))).unwrap();
    doc.ui.control_mut(p).unwrap().set_rect(380.0, 260.0, 200.0, 140.0);
    doc.set_prop(p, "rotation", &json!(30)).unwrap();
    doc.select(Some(p));
    let e = doc.add("Edit", Some(PointF::new(420.0, 300.0))).unwrap();
    doc.ui.control_mut(e).unwrap().set_rect(20.0, 40.0, 100.0, 30.0);
    doc.set_prop(e, "rotation", &json!(45)).unwrap();
    doc.ui.layout();
    let abs = doc.rect_of(e).unwrap();
    let still = doc.rotated_point(e, Handle::Left.point(abs));
    let start = doc.start_rect(e).unwrap();
    doc.resize_rotated(e, Handle::Right, start, PointF::new(10.0, 10.0));
    let now = doc.rect_of(e).unwrap();
    assert!(now.width > 100.0, "a largura cresceu: {now:?}");
    assert!(close(doc.rotated_point(e, Handle::Left.point(now)), still), "o lado esquerdo do Edit não andou");
}

#[test]
fn double_click_creates_the_default_event_or_opens_the_component_editor() {
    let mut doc = Document::new();
    let add = |doc: &mut Document, class: &str| {
        let id = doc.add(class, Some(PointF::new(40.0, 40.0))).unwrap();
        doc.select(Some(doc.root));
        id
    };
    let button = add(&mut doc, "Button");
    let edit = add(&mut doc, "Edit");
    let timer = add(&mut doc, "Timer");
    let msg = add(&mut doc, "MessageDialog");
    let event = |doc: &Document, id| actions::default_event_name(doc, id);
    assert_eq!(event(&doc, button).as_deref(), Some("on_click"));
    assert_eq!(event(&doc, edit).as_deref(), Some("on_change"));
    assert_eq!(event(&doc, timer).as_deref(), Some("on_timer"));
    assert_eq!(event(&doc, msg).as_deref(), Some("on_close"));
    assert_eq!(event(&doc, doc.root).as_deref(), Some("on_show"));
    doc.set_prop(msg, "title", &json!("Pedidos")).unwrap();
    doc.set_prop(msg, "text", &json!("Salvar?")).unwrap();

    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    // Botão: o handler do clique nasce com o nome padrão.
    ide.state.borrow_mut().doc.select(Some(button));
    actions::designer_dbl_click(&mut ide.ui, &ide.state, &ide.ids);
    let name = ide.state.borrow().doc.ui.name(button).unwrap().to_string();
    let expected = aylis_project::codegen::default_handler_name(&name, "on_click");
    assert_eq!(ide.state.borrow().doc.ui.get_prop(button, "on_click").ok(), Some(PropValue::Event(Some(expected))));
    // MessageDialog: o duplo clique mostra a caixa como ela vai aparecer.
    ide.state.borrow_mut().doc.select(Some(msg));
    actions::designer_dbl_click(&mut ide.ui, &ide.state, &ide.ids);
    let shown = ide.ui.roots().iter().copied().find(|&r| ide.ui.is_modal(r)).expect("abriu o teste da caixa");
    assert_eq!(ide.ui.get_prop(shown, "caption").ok(), Some(PropValue::String("Pedidos".into())));
    ide.ui.set_modal_result(shown, ModalResult::Yes);
    assert!(ide.state.borrow().status.contains("yes"), "{}", ide.state.borrow().status);
}

#[test]
fn double_click_on_a_dataset_opens_the_fields_window() {
    let mut doc = Document::new();
    let cds = doc.add("ClientDataSet", None).unwrap();
    for _ in 0..2 {
        doc.select(Some(cds));
        doc.add("Field", None).unwrap();
    }
    doc.select(Some(cds));
    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    actions::designer_dbl_click(&mut ide.ui, &ide.state, &ide.ids);

    // Não é modal: a IDE continua usável com a janela aberta.
    let window = ide.ui.roots().iter().copied().find(|&r| r != ide.form && ide.ui.is_open(r)).expect("abriu a janela de campos");
    assert!(!ide.ui.is_modal(window));
    let caption = |ui: &Ui| match ui.get_prop(window, "caption") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    };
    assert!(caption(&ide.ui).starts_with("Campos de"), "{}", caption(&ide.ui));
    let rows = |ui: &Ui| {
        let list = ui.components().find(|&c| ui.class_name(c) == Some("ScrollBox") && ui.form_of(c) == window).unwrap();
        ui.children(list).iter().copied().filter(|&r| ui.class_name(r) == Some("Rectangle")).collect::<Vec<_>>()
    };
    assert_eq!(rows(&ide.ui).len(), 2);

    // Clicar no segundo campo seleciona ele: o Object Inspector mostra o Field.
    let second = ide.state.borrow().doc.ui.children(cds)[1];
    ide.ui.fire(rows(&ide.ui)[1], "on_click", &aylis_core::EventArgs::None);
    assert_eq!(ide.state.borrow().doc.selection, Some(second));
    let title = match ide.ui.get_prop(ide.ids.inspector_title, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    };
    assert!(title.ends_with("Field"), "{title}");

    // ↑ sobe o campo; + Adicionar cria outro (a lista acompanha).
    let button = |ui: &Ui, text: &str| ui.components().find(|&c| ui.form_of(c) == window && ui.get_prop(c, "text").ok() == Some(PropValue::String(text.into()))).unwrap();
    ide.ui.fire(button(&ide.ui, "↑"), "on_click", &aylis_core::EventArgs::None);
    assert_eq!(ide.state.borrow().doc.ui.children(cds)[0], second);
    ide.ui.fire(button(&ide.ui, "+ Adicionar"), "on_click", &aylis_core::EventArgs::None);
    assert_eq!(rows(&ide.ui).len(), 3);
}

#[test]
fn searching_the_palette_after_scrolling_shows_the_results() {
    // A paleta estava rolada para baixo; a busca filtra para poucos itens: eles têm que
    // aparecer (a rolagem não pode ficar além do fim do conteúdo novo).
    let mut ide = app::build(shared(Document::new()));
    // A paleta só aparece fora da tela inicial (o "Novo form solto").
    ide.state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &ide.state.clone(), &ide.ids.clone());
    ide.ui.show(ide.form).unwrap();
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    ide.ui.layout();
    ide.ui.control_mut(ide.ids.palette).unwrap().set_scroll_offset(PointF::new(0.0, 2500.0));
    ide.ui.layout();
    let rows_before = ide.ui.children(ide.ids.palette).len();
    let search = ide.ui.components().find(|&c| ide.ui.get_prop(c, "placeholder").ok() == Some(PropValue::String("Buscar componente".into()))).unwrap();
    // Digitando de verdade (foco no campo e o texto pelo teclado), como o usuário.
    ide.ui.set_focus(search);
    ide.ui.text_input(ide.form, "Client");
    ide.ui.process_events();
    ide.ui.layout();
    let texts: Vec<String> = ide
        .ui
        .children(ide.ids.palette)
        .iter()
        .flat_map(|&r| ide.ui.children(r).to_vec())
        .filter_map(|l| match ide.ui.get_prop(l, "text") {
            Ok(PropValue::String(s)) => Some(s),
            _ => None,
        })
        .collect();
    assert!(ide.ui.children(ide.ids.palette).len() < rows_before / 4, "a lista filtrou: {texts:?}");
    assert!(texts.iter().any(|t| t == "ClientDataSet"), "{texts:?}");
    let view = ide.ui.bounds(ide.ids.palette).unwrap();
    let visible = ide.ui.children(ide.ids.palette).iter().filter(|&&c| ide.ui.laid_out_bounds(c).is_some_and(|b| b.bottom() > 0.0 && b.y < view.height)).count();
    assert!(visible > 0, "os resultados da busca ficaram fora da área visível");
}

#[test]
fn the_x_on_a_tray_chip_deletes_the_non_visual_component() {
    let mut doc = Document::new();
    let menu = doc.add("PopupMenu", None).unwrap();
    doc.select(Some(doc.root));
    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    ide.ui.layout();
    // O X fica à direita do primeiro chip da bandeja (embaixo da superfície).
    let surface = ide.ui.absolute_rect(ide.ids.surface).unwrap();
    let tray_y = surface.bottom() - 52.0;
    let x = PointF::new(surface.x + 12.0 + 160.0 - 17.0, tray_y + 10.0 + 16.0);
    let press = PointerEvent { position: x, click_count: 1, ..Default::default() };
    ide.ui.pointer_down(ide.form, press);
    ide.ui.pointer_up(ide.form, press);
    assert!(!ide.state.borrow().doc.ui.contains(menu), "o X excluiu o PopupMenu");
    assert!(ide.state.borrow_mut().doc.undo(), "o Ctrl+Z traz de volta");
    assert_eq!(ide.state.borrow().doc.non_visual().len(), 1);
}

#[test]
fn double_click_on_a_popup_menu_opens_the_menu_editor() {
    let mut doc = Document::new();
    let menu = doc.add("PopupMenu", None).unwrap();
    doc.select(Some(menu));
    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    actions::designer_dbl_click(&mut ide.ui, &ide.state, &ide.ids);
    let window = ide.ui.roots().iter().copied().find(|&r| r != ide.form && ide.ui.is_open(r)).expect("abriu o editor de menu");
    assert!(!ide.ui.is_modal(window));
    let button = |ui: &Ui, text: &str| ui.components().find(|&c| ui.form_of(c) == window && ui.get_prop(c, "text").ok() == Some(PropValue::String(text.into()))).unwrap();
    let click = |ide: &mut app::Ide, text: &str| {
        let b = button(&ide.ui, text);
        ide.ui.fire(b, "on_click", &aylis_core::EventArgs::None);
    };
    // Abrir, um divisor, Sair; um subitem dentro de Sair.
    click(&mut ide, "+ Item");
    click(&mut ide, "+ Divisor");
    click(&mut ide, "+ Item");
    click(&mut ide, "+ Subitem");
    let doc_ui = |ide: &app::Ide, f: &dyn Fn(&aylis_core::Ui) -> usize| f(&ide.state.borrow().doc.ui);
    assert_eq!(doc_ui(&ide, &|ui| ui.children(menu).len()), 3, "item, divisor, item");
    let second = ide.state.borrow().doc.ui.children(menu)[1];
    assert_eq!(ide.state.borrow().doc.ui.get_prop(second, "text").ok(), Some(PropValue::String("-".into())));
    let third = ide.state.borrow().doc.ui.children(menu)[2];
    assert_eq!(doc_ui(&ide, &|ui| ui.children(third).len()), 1, "o subitem ficou dentro do terceiro");
    // O item novo fica selecionado: o Object Inspector mostra o MenuItem.
    let title = match ide.ui.get_prop(ide.ids.inspector_title, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    };
    assert!(title.ends_with("MenuItem"), "{title}");
    // A árvore tem as 4 linhas.
    let list = ide.ui.components().find(|&c| ide.ui.class_name(c) == Some("ScrollBox") && ide.ui.form_of(c) == window).unwrap();
    assert_eq!(ide.ui.children(list).iter().filter(|&&r| ide.ui.class_name(r) == Some("Rectangle")).count(), 4);
}

#[test]
fn an_open_inspector_dropdown_closes_when_the_selection_changes() {
    let mut doc = Document::new();
    let a = doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    let b = doc.add("Button", Some(PointF::new(40.0, 120.0))).unwrap();
    doc.select(Some(a));
    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    ide.ui.layout();
    let popups = |ui: &Ui| ui.components().filter(|&c| ui.class_name(c) == Some("ComboPopup")).count();
    // Abre a lista de um combo do Object Inspector (como o popup_menu do print).
    let combo = ide.ui.components().find(|&c| ide.ui.class_name(c) == Some("ComboBox") && ide.ui.form_of(c) == ide.form && {
        let mut p = ide.ui.parent(c);
        let mut inside = false;
        while let Some(x) = p {
            inside |= x == ide.ids.inspector;
            p = ide.ui.parent(x);
        }
        inside
    }).expect("um combo no Object Inspector");
    ide.ui.click(combo);
    assert_eq!(popups(&ide.ui), 1, "a lista abriu");
    // Outro componente: o Object Inspector é refeito e a lista vai junto com o combo.
    ide.state.borrow_mut().doc.select(Some(b));
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    assert_eq!(popups(&ide.ui), 0, "a lista não ficou órfã na tela");
}

#[test]
fn the_palette_keeps_non_visual_components_in_their_own_section() {
    let mut ide = app::build(shared(Document::new()));
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    // Os textos da paleta em ordem (títulos e linhas).
    let texts: Vec<String> = ide
        .ui
        .children(ide.ids.palette)
        .iter()
        .filter_map(|&c| {
            let label = if ide.ui.class_name(c) == Some("Label") { Some(c) } else { ide.ui.children(c).iter().copied().find(|&l| ide.ui.class_name(l) == Some("Label")) };
            match label.map(|l| ide.ui.get_prop(l, "text")) {
                Some(Ok(PropValue::String(s))) => Some(s),
                _ => None,
            }
        })
        .collect();
    let section = texts.iter().position(|t| t == "NÃO VISUAIS").expect("a seção dos não visuais");
    for nv in ["Timer", "PopupMenu", "ClientDataSet", "MessageDialog"] {
        let at = texts.iter().position(|t| t == nv).unwrap_or_else(|| panic!("{nv} na paleta"));
        assert!(at > section, "{nv} só na seção dos não visuais");
    }
    assert!(texts.iter().position(|t| t == "Button").unwrap() < section, "os visuais ficam nas categorias de cima");
    assert_eq!(aylis_ide::nonvisual::icon("Timer"), Some("clock"));
    assert!(!aylis_ide::nonvisual::description("ClientDataSet").is_empty());
}

#[test]
fn the_tab_order_dialog_writes_the_form_list_and_first_focus() {
    let mut doc = Document::new();
    let a = doc.add("Edit", Some(PointF::new(20.0, 20.0))).unwrap();
    let b = doc.add("Edit", Some(PointF::new(20.0, 80.0))).unwrap();
    let c = doc.add("Edit", Some(PointF::new(20.0, 140.0))).unwrap();
    let names: Vec<String> = [a, b, c].iter().map(|&i| doc.ui.name(i).unwrap().to_string()).collect();
    let mut ide = app::build(shared(doc));
    ide.ui.show(ide.form).unwrap();
    aylis_ide::taborder::open(&mut ide.ui, &ide.state, &ide.ids);
    let dialog = ide.ui.roots().iter().copied().find(|&r| ide.ui.is_modal(r)).expect("abriu o diálogo");
    let list = ide.ui.components().find(|&x| ide.ui.class_name(x) == Some("ScrollBox") && ide.ui.form_of(x) == dialog).unwrap();
    let rows = |ui: &Ui| ui.children(list).to_vec();
    assert_eq!(rows(&ide.ui).len(), 3);
    let button = |ui: &Ui, text: &str| ui.components().find(|&x| ui.form_of(x) == dialog && ui.get_prop(x, "text").ok() == Some(PropValue::String(text.into()))).unwrap();
    // O terceiro sobe para o segundo lugar e vira o primeiro foco.
    ide.ui.fire(rows(&ide.ui)[2], "on_click", &aylis_core::EventArgs::None);
    ide.ui.fire(button(&ide.ui, "↑"), "on_click", &aylis_core::EventArgs::None);
    ide.ui.fire(button(&ide.ui, "★ Começar por este"), "on_click", &aylis_core::EventArgs::None);
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    let st = ide.state.borrow();
    let root = st.doc.root;
    assert_eq!(st.doc.ui.get_prop(root, "tab_order").ok(), Some(PropValue::String(format!("{}; {}; {}", names[0], names[2], names[1]))));
    assert_eq!(st.doc.ui.get_prop(root, "active_control").ok(), Some(PropValue::Component(Some(c))));
}

#[test]
fn a_component_outside_the_form_can_still_be_picked_and_popup_menu_lists_only_menus() {
    let mut doc = Document::new();
    let button = doc.add("Button", Some(PointF::new(16.0, 16.0))).unwrap();
    let form = doc.ui.bounds(doc.root).unwrap();
    doc.ui.control_mut(button).unwrap().set_position(form.width + 40.0, 20.0);
    doc.ui.layout();
    let inside_button = PointF::new(form.width + 50.0, 30.0);
    assert_eq!(doc.component_at(inside_button), Some(button), "o que está fora do form ainda dá para pegar");
    assert_eq!(doc.component_at(PointF::new(form.width + 400.0, 30.0)), None);

    let menu = doc.add("PopupMenu", None).unwrap();
    let menu_name = doc.ui.name(menu).unwrap().to_string();
    let candidates = doc.reference_candidates_for(button, "popup_menu");
    assert_eq!(candidates, vec![menu_name]);
    // Uma propriedade desconhecida aceita qualquer componente, menos o form e o próprio.
    assert_eq!(doc.reference_candidates_for(button, "focus_control"), vec![doc.ui.name(menu).unwrap().to_string()]);
}

#[test]
fn the_palette_has_platform_tabs_and_places_components_with_that_style() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    // A paleta só aparece fora da tela inicial (o "Novo form solto").
    ide.state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &ide.state.clone(), &ide.ids.clone());
    ide.ui.layout();
    let tab = row_with(&ide.ui, ide.ids.palette_tabs, "Apple");
    let at = center_of(&ide.ui, tab);
    click(&mut ide.ui, ide.ids.form, at);
    ide.ui.layout();
    assert_eq!(state.borrow().palette_tab, "Apple");
    // As versões da plataforma, cada uma com os componentes visuais.
    for family in ["iOS 26", "iOS 13", "iOS 7"] {
        row_with(&ide.ui, ide.ids.palette, family);
    }
    let switch = ide
        .ui
        .children(ide.ids.palette)
        .iter()
        .copied()
        .find(|&r| ide.ui.children(r).iter().any(|&l| text_of(&ide.ui, l) == "Button"))
        .expect("o Button na primeira versão (iOS 26)");
    let at = center_of(&ide.ui, switch);
    click(&mut ide.ui, ide.ids.form, at);
    assert_eq!(state.borrow().pending.as_deref(), Some("Button@iOS 26"));
    // Escolher na paleta sai da tela inicial (o painel do projeto volta e a paleta desce).
    ide.ui.layout();

    // Dentro da versão, um nível por categoria: recolher a Standard esconde o Button.
    let has_button = |ide: &app::Ide| ide.ui.children(ide.ids.palette).iter().any(|&r| ide.ui.children(r).iter().any(|&l| text_of(&ide.ui, l) == "Button"));
    for open in [false, true] {
        let standard = row_with(&ide.ui, ide.ids.palette, "Standard");
        let at = center_of(&ide.ui, standard);
        click(&mut ide.ui, ide.ids.form, at);
        ide.ui.layout();
        assert_eq!(has_button(&ide), open, "a categoria recolhe e abre");
    }
    // A versão também recolhe e abre pelo clique no cabeçalho (no nome, não só na seta).
    for open in [false, true] {
        let header = row_with(&ide.ui, ide.ids.palette, "iOS 26");
        let at = center_of(&ide.ui, header);
        click(&mut ide.ui, ide.ids.form, at);
        ide.ui.layout();
        assert_eq!(has_button(&ide), open, "o cabeçalho da versão recolhe e abre");
    }

    // Colocado no form, fica com o tema da versão (claro ou escuro como o do app).
    let id = state.borrow_mut().doc.add("Switch@iOS 26", Some(PointF::new(20.0, 20.0))).unwrap();
    let st = state.borrow();
    assert_eq!(st.doc.ui.get_prop(id, "theme").unwrap(), PropValue::String("iOS 26".into()));
    assert_eq!(st.doc.ui.theme_of(id).look, aylis_core::Look::Ios26);
    assert_eq!(st.doc.ui.class_name(id), Some("Switch"));
}

#[test]
fn an_invisible_component_shows_in_the_designer_only_while_selected() {
    let mut doc = Document::new();
    let button = doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    doc.set_prop(button, "visible", &json!(false)).unwrap();
    doc.ui.layout();
    // Selecionado (acabou de ser posto): continua na tela e no clique; a propriedade diz
    // falso.
    assert!(doc.ui.control(button).unwrap().visible());
    let r = doc.rect_of(button).unwrap();
    assert_eq!(doc.component_at(PointF::new(r.x + 5.0, r.y + 5.0)), Some(button));
    assert_eq!(doc.ui.get_prop(button, "visible").unwrap(), PropValue::Bool(false));

    // Salvo como invisível; aberto de novo no designer, some (como no app) até ser
    // selecionado.
    let text = doc.to_json().unwrap();
    assert!(text.contains("\"visible\": false"), "{text}");
    let mut back = Document::from_json(&text).unwrap();
    let id = back.ui.find(doc.ui.name(button).unwrap()).unwrap();
    assert!(!back.ui.control(id).unwrap().visible());
    back.select(Some(id));
    assert!(back.ui.control(id).unwrap().visible());
    assert_eq!(back.ui.get_prop(id, "visible").unwrap(), PropValue::Bool(false));

    // Fora do designer (em execução) some de verdade.
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_json::load(&mut ui, &text, None).unwrap();
    let id = ui.find(doc.ui.name(button).unwrap()).unwrap();
    assert!(!ui.control(id).unwrap().visible());
}

#[test]
fn project_only_menus_are_disabled_without_a_project() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    app::refresh(&mut ide.ui, &state, &ide.ids);
    // Os itens pelo texto, em qualquer nível do menu.
    let find = |ui: &Ui, text: &str| -> ComponentId {
        let mut stack = vec![ide.form];
        while let Some(id) = stack.pop() {
            if ui.class_name(id) == Some("MenuItem") && ui.get_prop(id, "text").ok() == Some(PropValue::String(text.into())) {
                return id;
            }
            stack.extend(ui.children(id).iter().copied());
        }
        panic!("item '{text}' não encontrado")
    };
    let enabled = |ui: &Ui, id| ui.get_prop(id, "enabled").unwrap() == PropValue::Bool(true);
    let only = ["Opções do projeto…", "Novo DataModule…", "Remover form do projeto…", "Publicar…", "Depurar na IDE"];
    for text in only {
        assert!(!enabled(&ide.ui, find(&ide.ui, text)), "'{text}' sem projeto");
    }
    for text in ["Novo form…", "Gerar código", "Executar", "Animações…"] {
        assert!(enabled(&ide.ui, find(&ide.ui, text)), "'{text}' funciona com form solto");
    }

    let base = temp("menus_projeto");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    for text in only {
        assert!(enabled(&ide.ui, find(&ide.ui, text)), "'{text}' com projeto");
    }
}

#[test]
fn the_status_bar_shows_how_long_the_build_has_been_running() {
    use aylis_ide::jobs::{self, Msg, Slot};
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let (go, wait) = std::sync::mpsc::channel::<()>();
    jobs::start(&state, Slot::Main, "Executando Loja — Windows", move |_, sender| {
        sender.send(Msg::Event(aylis_tools::Event::Step("Compilando Loja…".into())));
        let _ = wait.recv();
        sender.send(Msg::Event(aylis_tools::Event::Step("Executando Loja…".into())));
        Ok("Loja fechou".into())
    });
    // Compilando: o tique de cada segundo põe o tempo na barra de status.
    jobs::tick(&mut ide.ui, &state, &ide.ids);
    assert_eq!(text_of(&ide.ui, ide.ids.status), "Executando Loja — Windows… 0 s");

    go.send(()).unwrap();
    for _ in 0..200 {
        jobs::drain(&mut ide.ui, &state, &ide.ids);
        if state.borrow().job.is_none() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let st = state.borrow();
    assert!(st.job.is_none());
    assert!(st.output.iter().any(|l| l.text == "✔ Compilado em 0 s"), "o tempo da compilação nas mensagens");
    assert_eq!(st.status, "■ Loja fechou", "com o programa já rodando, o fim não soma o tempo dele");
    assert_eq!(jobs::elapsed_text(std::time::Duration::from_secs(83)), "1 min 23 s");
}

#[test]
fn the_icon_property_opens_the_icon_picker_even_without_a_project() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    let button = state.borrow_mut().doc.add("Button", Some(PointF::new(20.0, 20.0))).unwrap();
    state.borrow_mut().doc.ui.set_prop(button, "icon", "search").unwrap();
    state.borrow_mut().inspector_filter = "icon".into();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    // As linhas de ícone (icon, checked_icon) têm o "…".
    let more_of = |ui: &Ui, prop: &str| -> ComponentId {
        let line = ui
            .children(ide.ids.inspector)
            .iter()
            .copied()
            // Pelo nome técnico (na dica do rótulo, que mostra o legível).
            .find(|&l| ui.children(l).first().is_some_and(|&lbl| matches!(ui.get_prop(lbl, "hint"), Ok(PropValue::String(h)) if h == prop)))
            .unwrap_or_else(|| panic!("linha '{prop}'"));
        *ui.children(line).iter().find(|&&c| text_of(ui, c) == "…").unwrap_or_else(|| panic!("'…' em {prop}"))
    };
    more_of(&ide.ui, "checked_icon");
    let more = more_of(&ide.ui, "icon");
    ide.ui.fire(more, "on_click", &aylis_core::EventArgs::None);
    let dialog = *ide.ui.open_forms().last().unwrap();
    assert!(ide.ui.is_modal(dialog), "o seletor abriu sem projeto");
    // Começa no pacote do ícone atual (os do aylis); um clique escolhe outro.
    let mut stack = vec![dialog];
    let mut home = None;
    while let Some(id) = stack.pop() {
        if ide.ui.get_prop(id, "hint").ok() == Some(PropValue::String("home".into())) {
            home = Some(id);
        }
        stack.extend(ide.ui.children(id).iter().copied());
    }
    ide.ui.fire(home.expect("o ícone home na grade"), "on_click", &aylis_core::EventArgs::None);
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert_eq!(state.borrow().doc.ui.get_prop(button, "icon").unwrap(), PropValue::String("home".into()));
}

#[test]
fn the_gallery_has_platform_tabs_and_shows_components_in_that_version() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    let gallery = aylis_ide::gallery::open(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let all = |ui: &Ui| -> Vec<ComponentId> {
        let mut out = Vec::new();
        let mut stack = vec![gallery];
        while let Some(id) = stack.pop() {
            out.push(id);
            stack.extend(ui.children(id).iter().copied());
        }
        out
    };
    // A aba (o retângulo dono do texto).
    let tab_with = |ui: &Ui, text: &str| all(ui).into_iter().find(|&id| ui.class_name(id) == Some("Label") && text_of(ui, id) == text).and_then(|l| ui.parent(l));
    assert!(tab_with(&ide.ui, "iOS 26").is_none(), "na aba aylis não há versões");
    let apple = tab_with(&ide.ui, "Apple").expect("aba Apple");
    ide.ui.fire(apple, "on_click", &aylis_core::EventArgs::None);
    ide.ui.layout();
    for v in ["iOS 26", "iOS 13", "iOS 7"] {
        assert!(tab_with(&ide.ui, v).is_some(), "versão {v}");
    }
    // Os cartões mostram os componentes de verdade no tema da versão.
    let switch = all(&ide.ui).into_iter().find(|&id| ide.ui.class_name(id) == Some("Switch")).expect("um Switch vivo");
    assert_eq!(ide.ui.theme_of(switch).look, aylis_core::Look::Ios26);
    // A versão mais antiga troca o desenho.
    let ios7 = tab_with(&ide.ui, "iOS 7").unwrap();
    ide.ui.fire(ios7, "on_click", &aylis_core::EventArgs::None);
    let switch = all(&ide.ui).into_iter().find(|&id| ide.ui.class_name(id) == Some("Switch")).unwrap();
    assert_eq!(ide.ui.theme_of(switch).look, aylis_core::Look::Ios7);
    // O "Usar" do cartão do Button escolhe o Button daquela versão.
    let use_it = all(&ide.ui)
        .into_iter()
        .find(|&b| {
            text_of(&ide.ui, b) == "Usar" && ide.ui.parent(b).is_some_and(|bar| ide.ui.children(bar).iter().any(|&l| text_of(&ide.ui, l) == "Button"))
        })
        .expect("o Usar do Button");
    ide.ui.fire(use_it, "on_click", &aylis_core::EventArgs::None);
    assert_eq!(state.borrow().pending.as_deref(), Some("Button@iOS 7"));
}

#[test]
fn the_wheel_scrolls_the_designer_only_when_the_form_does_not_fit() {
    use aylis_ide::surface::DesignSurface;
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    // Sai da tela inicial para o designer (o "Novo form solto").
    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let s = ide.ui.absolute_rect(ide.ids.surface).unwrap();
    let at = PointF::new(s.x + s.width / 2.0, s.y + s.height / 2.0);
    let origin = |ui: &Ui| ui.get::<DesignSurface>(ide.ids.surface).unwrap().origin();
    let wheel = |ui: &mut Ui, dy: f32| {
        ui.wheel(ide.form, aylis_core::WheelEvent { position: at, delta: PointF::new(0.0, dy), ..Default::default() });
    };
    // O form cabe (o caso do Model3D da gravação): a roda não empurra nada.
    let start = origin(&ide.ui);
    for _ in 0..10 {
        wheel(&mut ide.ui, 120.0);
    }
    assert_eq!(origin(&ide.ui), start);
    // Com zoom de 400% ele não cabe: a roda rola, mas o form não sai da área.
    state.borrow_mut().zoom = 4.0;
    for _ in 0..50 {
        wheel(&mut ide.ui, -120.0);
    }
    let low = origin(&ide.ui).y;
    assert!(low < start.y, "rolou para cima");
    let form = state.borrow().doc.rect_of(state.borrow().doc.root).unwrap();
    assert!(low + form.height * 4.0 >= s.height - 60.0, "o fim do form fica à vista");
    wheel(&mut ide.ui, -120.0);
    assert_eq!(origin(&ide.ui).y, low, "parou na borda");
}

#[test]
fn references_list_only_the_right_type_and_field_stays_off_the_palette() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.layout();
    let (ds, cds, grid) = {
        let mut st = state.borrow_mut();
        let doc = &mut st.doc;
        let grid = doc.add("DataGrid", Some(PointF::new(20.0, 20.0))).unwrap();
        doc.add("Button", Some(PointF::new(300.0, 20.0))).unwrap();
        let cds = doc.add("ClientDataSet", None).unwrap();
        doc.select(Some(cds));
        doc.add("Field", None).unwrap();
        let ds = doc.add("DataSource", None).unwrap();
        (ds, cds, grid)
    };
    let st = state.borrow();
    let name = |id| st.doc.ui.name(id).unwrap().to_string();
    // data_set: só o ClientDataSet (nem o form, nem o botão, nem o Field).
    assert_eq!(st.doc.reference_candidates_for(ds, "data_set"), vec![name(cds)]);
    assert_eq!(st.doc.reference_candidates_for(grid, "data_source"), vec![name(ds)]);
    assert!(st.doc.reference_candidates_for(grid, "active_control").is_empty() || !st.doc.reference_candidates_for(grid, "active_control").contains(&name(grid)));
    drop(st);

    // Field e MenuItem não estão na paleta (entram pela janela de campos e pelo editor de menu).
    state.borrow_mut().palette_filter = "field".into();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let texts: Vec<String> = ide.ui.children(ide.ids.palette).iter().flat_map(|&r| ide.ui.children(r).iter().map(|&l| text_of(&ide.ui, l)).collect::<Vec<_>>()).collect();
    assert!(!texts.iter().any(|t| t == "Field"), "{texts:?}");

    // A busca do Object Inspector vale só para o componente em que foi digitada.
    {
        let mut st = state.borrow_mut();
        st.doc.select(Some(ds));
    }
    app::refresh(&mut ide.ui, &state, &ide.ids);
    state.borrow_mut().inspector_filter = "data".into();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    state.borrow_mut().doc.select(Some(cds));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(state.borrow().inspector_filter.is_empty(), "trocar de componente limpa a busca");
    assert_eq!(text_of(&ide.ui, ide.ids.inspector_search), "");
}

#[test]
fn a_new_project_takes_the_ide_language_as_the_app_language() {
    assert_eq!(aylis_ide::lang::app_locale("en"), "en-US");
    assert_eq!(aylis_ide::lang::app_locale("ja"), "ja-JP");
    let state = shared(Document::new());
    state.borrow_mut().language = "es".into();
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::new_project(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    let mut stack = vec![dialog];
    let mut edits = Vec::new();
    while let Some(id) = stack.pop() {
        if ide.ui.get::<Edit>(id).is_some() {
            edits.push(id);
        }
        stack.extend(ide.ui.children(id).iter().rev().copied());
    }
    // Nome e pasta (os dois primeiros campos).
    let base = temp("idioma_novo");
    let _ = std::fs::remove_dir_all(base.join("Vendas"));
    ide.ui.get_mut::<Edit>(edits[0]).unwrap().text = "Vendas".into();
    ide.ui.get_mut::<Edit>(edits[1]).unwrap().text = base.display().to_string();
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    let st = state.borrow();
    let project = st.project.as_ref().expect("o projeto foi criado e aberto");
    assert_eq!(project.locale, "es-ES", "o idioma da IDE (espanhol), não um fixo");
    // A IDE confia sozinha no projeto que cria: a pasta de teste sai da lista do usuário.
    aylis_tools::trust::forget_dir(&project.dir());
}

#[test]
fn the_run_target_combo_lists_only_what_the_language_supports() {
    let base = temp("alvos");
    let _ = std::fs::remove_dir_all(base.join("Loja"));
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    let items = |ui: &Ui| ui.get::<aylis_controls::ComboBox>(ide.ids.target_combo).unwrap().items.clone();
    // Sem projeto: só a prévia no desktop.
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(items(&ide.ui), aylis_tools::Target::Desktop.name());
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let rust = items(&ide.ui);
    assert!(rust.contains("Linux (WSL)") && rust.contains("Android") && rust.contains("Navegador"), "{rust}");
    assert!(!rust.contains("iOS"), "iOS só num Mac: {rust}");
    // Escolhe Android e troca a linguagem para Python: só o desktop sobra, e o alvo volta.
    state.borrow_mut().run_target = aylis_tools::Target::Android;
    state.borrow_mut().project.as_mut().unwrap().language = aylis_project::Language::Python;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(items(&ide.ui), aylis_tools::Target::Desktop.name());
    assert_eq!(state.borrow().run_target, aylis_tools::Target::Desktop);
}

#[test]
fn the_new_texts_and_the_messages_with_values_are_translated() {
    let state = aylis_ide::state::shared(aylis_ide::document::Document::new());
    let mut ide = aylis_ide::app::build(state.clone());
    for code in ["en", "ja", "ar"] {
        aylis_ide::lang::apply(&mut ide.ui, code);
        let tr = |s: &str| ide.ui.tr(s);
        assert_ne!(tr("Placa de vídeo"), "Placa de vídeo", "{code}");
        // Uma mensagem já montada com os valores: pelo modelo.
        assert_ne!(tr("Projeto Loja aberto (Rust)"), "Projeto Loja aberto (Rust)", "{code}");
        assert!(tr("Projeto Loja aberto (Rust)").contains("Loja"), "{code}: o valor continua");
    }
    aylis_ide::lang::apply(&mut ide.ui, "en");
    // Os que saíam em português com a IDE em inglês (roteiro 84, conferência de 08/11).
    for pt in ["Executar sem depurar", "Compilar e executar em release", "Compilar", "Compilar em release", "Mostrar componentes escondidos", "Abrir em outra janela", "Ir para o Design", "Nome do executável", "Navegador (web)", "(troque em Opções do projeto → Aparência)"] {
        assert_ne!(ide.ui.tr(pt), pt, "{pt}");
    }
    assert_eq!(ide.ui.tr("Opções do projeto · Geral"), "Project options · General");
    // A categoria "Data" da paleta aparece como "Dados" (o "Data" em português é o rótulo do
    // `date`, que vira "Date").
    assert_eq!(ide.ui.tr(aylis_ide::panels::palette_label("Data")), "Data");
    assert_eq!(ide.ui.tr("Form Cadastro adicionado"), "Form Cadastro added");
    assert_eq!(ide.ui.tr("3 min 12 s"), "3 min 12 s");
    aylis_ide::lang::apply(&mut ide.ui, "pt");
    assert_eq!(ide.ui.tr("Placa de vídeo"), "Placa de vídeo");
}

/// Sem projeto, a IDE abre na tela inicial (os recentes), não num form vazio; escolher um
/// componente na paleta vai para o designer.
#[test]
fn the_ide_opens_on_the_home_screen() {
    let state = shared(Document::new());
    state.borrow_mut().recent = vec![std::path::PathBuf::from("C:/projetos/Loja/Loja.aylisproj")];
    let mut ide = app::build(state.clone());
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(ide.ui.control(ide.ids.home).unwrap().visible());
    assert!(!ide.ui.control(ide.ids.surface).unwrap().visible(), "o designer fica escondido");
    let list = ide.ui.get::<aylis_controls::ListView>(ide.ids.home_list).unwrap();
    assert_eq!(list.item(0).map(|i| i.text.as_str()), Some("Loja"));
    aylis_ide::panels::choose(&state, "Button");
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(!ide.ui.control(ide.ids.home).unwrap().visible());
    assert!(ide.ui.control(ide.ids.surface).unwrap().visible());
}

/// O código abre embutido na IDE, no lugar do designer (Design | Código embaixo); a aba vai
/// para outra janela e, fechando a janela, volta. O painel do projeto mostra os forms em
/// árvore e os fontes da linguagem, sem os de cada form.
#[test]
fn code_opens_inside_the_ide_and_tabs_move_to_another_window() {
    use aylis_controls::TreeView;
    let base = temp("embutido");
    let project = aylis_project::create(&base, "Agenda", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);

    // O painel do projeto: a árvore por dono (Forms, com a unit e o JSON de cada um; Programa;
    // Units; o que o aylis gera, com cadeado).
    let tree = ide.ui.children(ide.ids.project).iter().copied().find(|&c| ide.ui.get::<TreeView>(c).is_some()).expect("árvore do projeto");
    let t = ide.ui.get::<TreeView>(tree).unwrap();
    let roots: Vec<String> = t.nodes().iter().map(|n| n.text.clone()).collect();
    assert_eq!(roots[0], "Forms");
    assert!(roots.iter().any(|r| r == "Programa"), "{roots:?}");
    assert!(roots.iter().any(|r| r.starts_with("Gerado pelo Aylis")), "{roots:?}");
    fn all(nodes: &[aylis_controls::TreeNode], out: &mut Vec<String>) {
        for n in nodes {
            out.push(n.data.clone());
            all(&n.children, out);
        }
    }
    let mut data = Vec::new();
    all(t.nodes(), &mut data);
    let form_source = aylis_project::codegen::form_source(&project, &project.main_form).unwrap();
    assert!(data.iter().any(|d| d == &format!("form:{}", project.main_form)), "{data:?}");
    // A unit do form fica dentro dele (não solta entre as units).
    fn find<'a>(nodes: &'a [aylis_controls::TreeNode], data: &str) -> Option<&'a aylis_controls::TreeNode> {
        nodes.iter().find_map(|n| if n.data == data { Some(n) } else { find(&n.children, data) })
    }
    let form_node = find(t.nodes(), &format!("form:{}", project.main_form)).unwrap();
    let unit = form_source.file_name().unwrap().to_string_lossy().to_string();
    assert!(form_node.children.iter().any(|c| c.data.starts_with("file:") && c.data.ends_with(&unit)), "a unit dentro do form");
    assert_eq!(data.iter().filter(|d| d.starts_with("file:") && d.ends_with(&unit)).count(), 1, "a unit só uma vez: {data:?}");
    assert!(data.iter().any(|d| d.starts_with("file:") && d.ends_with("main.rs")), "{data:?}");
    // A árvore nasce fechada (o relatório do Tabernáculo, I10): o que o usuário abre fica
    // guardado por projeto.
    assert!(t.nodes().iter().all(|n| !n.expanded), "tudo recolhido");

    // F12: o código embutido no centro.
    actions::open_code(&mut ide.ui, &state, None);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let visible = |ui: &Ui, id| ui.control(id).unwrap().visible();
    assert!(visible(&ide.ui, ide.ids.code_host) && !visible(&ide.ui, ide.ids.surface), "código no lugar do designer");
    assert_eq!(aylis_ide::codewindow::code_form(&ide.ui), Some(ide.form), "embutido na janela da IDE");
    // F12 no código volta ao Design; de novo no designer, volta ao código.
    let (modifiers, key) = aylis_core::parse_shortcut("F12").unwrap();
    ide.ui.key_down(ide.form, KeyEvent { key, modifiers, repeat: false });
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(visible(&ide.ui, ide.ids.surface) && !visible(&ide.ui, ide.ids.code_host), "F12: Design");
    actions::open_code(&mut ide.ui, &state, None);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let windows = ide.ui.open_forms().len();

    // A aba vai para outra janela: o centro volta ao designer.
    aylis_ide::codewindow::detach_active(&mut ide.ui);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(ide.ui.open_forms().len(), windows + 1, "a janela de código");
    let outside = *ide.ui.open_forms().last().unwrap();
    assert!(visible(&ide.ui, ide.ids.surface) && !visible(&ide.ui, ide.ids.code_host));
    assert!(aylis_ide::codewindow::text_of(&ide.ui, &form_source).is_some(), "o arquivo continua aberto");

    // Fechar a janela de fora devolve a aba à IDE.
    assert!(ide.ui.close(outside));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(aylis_ide::codewindow::embedded_has_tabs(&ide.ui));
    assert!(visible(&ide.ui, ide.ids.code_host), "de volta, no código embutido");

    // "Design" volta ao form.
    state.borrow_mut().code_view = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(visible(&ide.ui, ide.ids.surface));
    let _ = std::fs::remove_dir_all(base);
}

/// Toda propriedade que o Object Inspector mostra tem um nome legível
/// (`resources/locales/properties.tsv`), traduzido nos idiomas da IDE.
#[test]
fn forms_and_code_share_one_tab_strip() {
    let base = temp("abas_unificadas");
    let project = aylis_project::create(&base, "Agenda", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    // O texto de cada aba da faixa da IDE (com o ponto quando há alteração não salva).
    let tab_texts = |ide: &aylis_ide::app::Ide| -> Vec<String> { strip_titles(ide) };
    let form_name = state.borrow().doc.form_name();
    assert_eq!(tab_texts(&ide), vec![form_name.clone()]);

    // F12: o código do form fica na mesma aba (não abre outra faixa dentro do editor).
    actions::open_code(&mut ide.ui, &state, None);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let visible = |ui: &Ui, id| ui.control(id).unwrap().visible();
    assert!(visible(&ide.ui, ide.ids.code_host) && visible(&ide.ui, ide.ids.tabs), "as abas ficam em cima do código");
    assert_eq!(tab_texts(&ide), vec![form_name.clone()], "form e código numa aba só");
    let mut stack = ide.ui.children(ide.ids.code_host).to_vec();
    assert!(!stack.is_empty(), "o editor embutido");
    while let Some(id) = stack.pop() {
        assert!(ide.ui.get::<aylis_controls::MainMenu>(id).is_none(), "sem barra de menu no editor embutido");
        if ide.ui.get::<aylis_controls::TabStrip>(id).is_some() {
            assert!(!visible(&ide.ui, id), "sem a faixa de abas do editor");
        }
        if ide.ui.get::<aylis_controls::Button>(id).is_some_and(|b| b.text == "Salvar") {
            assert!(!visible(&ide.ui, id), "sem o botão Salvar no editor");
        }
        stack.extend(ide.ui.children(id).iter().copied());
    }

    // Um fonte que não é de form ganha a aba dele, depois dos forms.
    let main = project.code_dir().join("src").join("main.rs");
    aylis_ide::codewindow::open(&mut ide.ui, &main, 1).unwrap();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(tab_texts(&ide), vec![form_name.clone(), "main.rs".to_string()]);
    // Alterado, a aba mostra o ponto; o Salvar da IDE grava o código.
    let memo = ide.ui.focused(ide.form).expect("foco no código");
    if let Some(m) = ide.ui.get_mut::<aylis_controls::Memo>(memo) {
        m.text.insert_str(0, "// novo\n");
    }
    ide.ui.fire(memo, "on_change", &aylis_core::EventArgs::None);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(tab_texts(&ide)[1], "main.rs •");
    actions::save_current(&mut ide.ui, &state, &ide.ids);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(std::fs::read_to_string(&main).unwrap().starts_with("// novo\n"));
    assert_eq!(tab_texts(&ide)[1], "main.rs");

    if let Some(dir) = std::env::var_os("AYLIS_IDE_SHOTS") {
        let renderer = aylis_skia::SkiaRenderer::new();
        ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
        ide.ui.layout();
        let (w, h) = ide.ui.control(ide.form).map_or((1600.0, 1000.0), |c| (c.width(), c.height()));
        let background = ide.ui.theme().color("background");
        let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ide.ui.paint(ide.form, c)).unwrap();
        std::fs::write(PathBuf::from(dir).join("abas_unificadas.png"), png).unwrap();
    }

    // Clicar na aba do form, no código, mostra o código dele; "Design" volta ao form.
    let strip = tab_strip(&ide);
    ide.ui.fire(strip, "on_change", &aylis_core::EventArgs::Value(0i64.into()));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let form_source = aylis_project::codegen::form_source(&project, &project.main_form).unwrap();
    assert_eq!(aylis_ide::codewindow::embedded_active(&ide.ui), Some(aylis_ide::debugger::norm(&form_source)));
    ide.ui.click(ide.ids.view_design);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(visible(&ide.ui, ide.ids.surface) && !visible(&ide.ui, ide.ids.code_host));
    // O × da aba do código (o on_close da faixa, com o índice) fecha o arquivo.
    ide.ui.fire(tab_strip(&ide), "on_close", &aylis_core::EventArgs::Value(1i64.into()));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(tab_texts(&ide), vec![form_name]);
    let _ = std::fs::remove_dir_all(base);
}

/// A explicação de cada propriedade sai do comentário `///` do campo no código (o que a página
/// do componente e o `component_docs` do MCP mostram; decidido com o autor em 11/11).
#[test]
fn property_docs_come_from_the_code_comments() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs = aylis_ide::reference::source_field_docs(&root);
    let radius = docs.get("Rectangle").and_then(|f| f.get("radius")).expect("o radius do Rectangle");
    assert!(radius.contains("Negativo") && radius.contains("[cima-esquerda"), "{radius}");
    assert!(docs.get("ControlBase").and_then(|f| f.get("anchors")).is_some_and(|d| d.contains("align: none")));
}

/// `new_form` pelo MCP e logo `set_layout` e `save`: vão para o form novo, não para o que
/// estava aberto (relatório do InsightDB, 11/11: gravaram no form errado).
#[test]
fn set_layout_after_new_form_goes_to_the_new_form() {
    use serde_json::json;
    let base = temp("novo_form_mcp");
    let project = aylis_project::create(&base, "Agenda", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let run = |ide: &mut aylis_ide::app::Ide, name: &str, args: serde_json::Value| {
        let r = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, name, &args);
        app::refresh(&mut ide.ui, &state, &ide.ids);
        r
    };
    let principal = std::fs::read_to_string(project.file("forms/principal.json")).unwrap();
    run(&mut ide, "new_form", json!({ "name": "frmDashboard" })).unwrap();
    assert_eq!(state.borrow().doc.form_name(), "frmDashboard");
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmDashboard", "caption": "Painel", "children": [{ "class": "Label", "name": "lblTitulo", "text": "Olá" }] } });
    run(&mut ide, "set_layout", json!({ "layout": layout })).unwrap();
    run(&mut ide, "save", json!({})).unwrap();
    let saved = aylis_project::Project::open(&project.path).unwrap();
    let file = saved.file(&saved.form("frmDashboard").unwrap().file);
    assert!(std::fs::read_to_string(&file).unwrap().contains("lblTitulo"), "o form novo recebeu o layout");
    assert_eq!(std::fs::read_to_string(project.file("forms/principal.json")).unwrap(), principal, "o principal ficou como estava");

    // Uma raiz com o nome de outro form do projeto é recusada (renomearia o aberto).
    let other = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal" } });
    assert!(run(&mut ide, "set_layout", json!({ "layout": other })).is_err());

    // O arquivo do form escolhido na criação (L-03) e movido depois (L-18): o JSON, o fonte e a
    // aba aberta vão junto.
    run(&mut ide, "new_form", json!({ "name": "frmRelatorio", "file": "forms/painel/relatorio.json" })).unwrap();
    assert!(project.file("forms/painel/relatorio.json").is_file());
    run(&mut ide, "move_form", json!({ "name": "frmRelatorio", "file": "forms/painel/resumo.json" })).unwrap();
    let moved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(moved.form("frmRelatorio").unwrap().file, "forms/painel/resumo.json");
    assert!(!project.file("forms/painel/relatorio.json").exists());
    assert!(aylis_project::codegen::form_source(&moved, "frmRelatorio").unwrap().is_file(), "o fonte foi junto");
    assert_eq!(state.borrow().doc.path.as_deref(), Some(moved.file("forms/painel/resumo.json").as_path()));
    assert!(run(&mut ide, "move_form", json!({ "name": "frmRelatorio", "file": "../fora.json" })).is_err());
    let _ = std::fs::remove_dir_all(base);
}

/// O layout conferido devolve todos os erros de uma vez, e o `class_info` diz o que cada
/// propriedade aceita (relatório do RTSPlayer, 10/11: os erros vinham um por tentativa, e as
/// propriedades e os valores eram adivinhados).
#[test]
fn the_layout_check_lists_every_error_and_class_info_lists_the_values() {
    let mut ui = aylis_ide::document::new_document_ui(false);
    let json = r#"{"aylis": 1, "root": {"class": "Form", "name": "frmX", "children": [
        {"class": "Label", "name": "lblA", "trimming": "ellipsis"},
        {"class": "Layout", "name": "layB", "radius": 8},
        {"class": "Button", "name": "btnC", "on_double_click": "btnC_dbl"},
        {"class": "NaoExiste", "name": "x"},
        {"class": "Panel", "name": "pnlD", "shadow": "md"},
        {"class": "Frame", "name": "fraE", "form": "fraOutro", "children": [{"class": "Label", "name": "lblF"}]}
    ]}}"#;
    let errors: Vec<String> = aylis_json::validate(&mut ui, json).iter().map(ToString::to_string).collect();
    assert_eq!(errors.len(), 5, "{errors:#?}");
    assert!(errors.iter().any(|e| e.contains("Frame")), "o Frame com filhos (L-07): {errors:#?}");
    assert!(errors.iter().any(|e| e.contains("trimming")) && errors.iter().any(|e| e.contains("radius")) && errors.iter().any(|e| e.contains("on_double_click")) && errors.iter().any(|e| e.contains("NaoExiste")), "{errors:#?}");
    assert!(ui.roots().is_empty(), "a raiz de passagem foi liberada");
    // O nome de sombra do tema vale no form.
    assert!(aylis_json::validate(&mut ui, r#"{"aylis": 1, "root": {"class": "Panel", "shadow": "md"}}"#).is_empty());

    let label = aylis_json::class_info(&ui, "Label").unwrap();
    let trimming = label["properties"].as_array().unwrap().iter().find(|p| p["name"] == "trimming").expect("trimming");
    assert!(trimming["values"].as_array().unwrap().iter().any(|v| v == "word"), "{trimming}");
    let button = aylis_json::class_info(&ui, "Button").unwrap();
    assert!(button["events"].as_array().unwrap().iter().any(|e| e == "on_dbl_click"));
    assert!(aylis_json::class_info(&ui, "NaoExiste").is_none());
}

/// Um fonte novo na pasta do código (o arquivo por assunto; relatório do RTSPlayer, 10/11): só
/// dentro dela, fora do `generated/`, e sem sobrescrever.
#[test]
fn a_new_source_is_created_inside_the_code_folder() {
    let base = temp("fonte_novo");
    let project = aylis_project::create(&base, "Agenda", aylis_project::Language::Rust).unwrap();
    let file = actions::new_source(&project, "src/nucleo.rs", "pub fn soma() {}\n").unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "pub fn soma() {}\n");
    assert!(file.starts_with(project.code_dir()));
    for bad in ["src/nucleo.rs", "../fora.rs", "C:/fora.rs", "generated/x.rs", ""] {
        assert!(actions::new_source(&project, bad, "").is_err(), "{bad}");
    }
    let _ = std::fs::remove_dir_all(base);
}

/// O `new_project` cria direto numa pasta que já tem outras coisas (o app dentro de um
/// repositório), mas não por cima de outro projeto.
#[test]
fn a_project_can_be_created_in_an_existing_folder() {
    let base = temp("projeto_na_pasta");
    std::fs::create_dir_all(base.join("client/core")).unwrap();
    let project = aylis_project::create_at(&base.join("client"), "RTSPlayer", aylis_project::Language::Rust).unwrap();
    assert_eq!(project.dir(), base.join("client"));
    assert!(base.join("client/core").is_dir(), "o que já estava fica");
    assert!(aylis_project::create_at(&base.join("client"), "Outro", aylis_project::Language::Rust).is_err(), "já tem um projeto");
    let _ = std::fs::remove_dir_all(base);
}

/// A faixa de versão nova aparece com uma release mais nova e some no ✕ (a consulta em si,
/// pela rede, fica fora do teste: o resultado é montado à mão).
#[test]
fn a_newer_release_shows_the_update_banner() {
    use aylis_tools::updates::{Fetched, Release};
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let release = |tag: &str| Release { tag: tag.into(), prerelease: false, url: format!("https://github.com/e-rust/aylis/releases/tag/{tag}") };
    let current = format!("v{}", aylis_ide::updates::current());
    aylis_ide::updates::finish(&mut ide.ui, Fetched { status: 200, releases: vec![release(&current)], error: String::new() }, false);
    assert!(!aylis_ide::updates::banner_visible(&ide.ui), "a mesma versão não avisa");
    aylis_ide::updates::finish(&mut ide.ui, Fetched { status: 0, releases: Vec::new(), error: "sem rede".into() }, false);
    assert!(!aylis_ide::updates::banner_visible(&ide.ui), "sem resposta não avisa");
    aylis_ide::updates::finish(&mut ide.ui, Fetched { status: 200, releases: vec![release("v99.0.0"), release(&current)], error: String::new() }, false);
    assert!(aylis_ide::updates::banner_visible(&ide.ui), "versão nova avisa");
    aylis_ide::updates::hide(&mut ide.ui);
    // Dispensada, a mesma versão não volta sozinha na sessão.
    aylis_ide::updates::finish(&mut ide.ui, Fetched { status: 200, releases: vec![release("v99.0.0")], error: String::new() }, false);
    assert!(!aylis_ide::updates::banner_visible(&ide.ui));
}

/// O Design no código de um form sem aba aberta mostra esse form (antes voltava ao último
/// design mostrado; relatado pelo autor em 10/11).
#[test]
fn design_from_the_code_of_a_form_without_a_tab_opens_that_form() {
    let base = temp("design_sem_aba");
    let mut project = aylis_project::create(&base, "Agenda", aylis_project::Language::Rust).unwrap();
    project.add_form("frmClientes", "Clientes").unwrap();
    project.save().unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_ne!(state.borrow().doc.form_name(), "frmClientes", "só o form principal aberto");

    // O fonte do frmClientes aberto direto (como pelo Pesquisar no projeto), sem a aba do form.
    actions::open_code(&mut ide.ui, &state, None);
    let source = aylis_project::codegen::form_source(&project, "frmClientes").unwrap();
    aylis_ide::codewindow::open(&mut ide.ui, &source, 1).unwrap();
    state.borrow_mut().code_view = true;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert_eq!(aylis_ide::codewindow::embedded_active(&ide.ui), Some(aylis_ide::debugger::norm(&source)));

    ide.ui.click(ide.ids.view_design);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let st = state.borrow();
    assert!(!st.code_view);
    assert_eq!(st.doc.form_name(), "frmClientes", "o Design abriu o form deste fonte");
    drop(st);
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn every_inspector_property_has_a_readable_translated_name() {
    let doc = Document::new();
    let mut ui = doc.ui;
    let classes: Vec<String> = ui.registry().iter().map(|c| c.name.to_string()).collect();
    let root = ui.create("Form", None).unwrap();
    let mut missing = std::collections::BTreeSet::new();
    for class in classes {
        let parent = if class == "Form" || class == "DataModule" { None } else { Some(root) };
        let Ok(id) = ui.create(&class, parent) else { continue };
        for i in ui.prop_infos(id) {
            if i.kind != PropKind::Event && !i.read_only && !i.runtime && !aylis_ide::lang::has_property_label(i.name) {
                missing.insert(format!("{class}.{}", i.name));
            }
        }
    }
    assert!(missing.is_empty(), "sem nome legível em properties.tsv: {missing:?}");
    assert_eq!(aylis_ide::lang::property_label("accent_color"), "Cor de destaque");
    assert_eq!(aylis_ide::lang::category_label("Appearance"), "Aparência");
    let en = aylis_ide::lang::dictionary("en");
    assert_eq!(en.get("Cor de destaque").map(String::as_str), Some("Accent color"));
    let ja = aylis_ide::lang::dictionary("ja");
    assert_eq!(ja.get("Aparência").map(String::as_str), Some("外観"));
}

/// Clicar num componente com subtipos na paleta escolhe e abre os subtipos; clicar de novo
/// desmarca e fecha.
#[test]
fn clicking_a_palette_class_with_subtypes_opens_them() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let button = row_with(&ide.ui, ide.ids.palette, "Button");
    let at = center_of(&ide.ui, button);
    click(&mut ide.ui, ide.form, at);
    ide.ui.layout();
    assert_eq!(state.borrow().pending.as_deref(), Some("Button"));
    assert!(state.borrow().palette_open.contains("Button"), "os subtipos abriram");
    row_with(&ide.ui, ide.ids.palette, "Contorno");
    let button = row_with(&ide.ui, ide.ids.palette, "Button");
    let at = center_of(&ide.ui, button);
    click(&mut ide.ui, ide.form, at);
    assert_eq!(state.borrow().pending, None);
    assert!(!state.borrow().palette_open.contains("Button"), "fechou");
}

/// A tela inicial fica limpa: sem os painéis dos lados (paleta, inspector, estrutura, projeto)
/// nem os menus de edição e projeto; tudo volta ao sair dela.
#[test]
fn the_home_screen_hides_the_panels_and_project_menus() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    // À vista: ele e todos os pais visíveis.
    let shown = |ui: &Ui, id: ComponentId| {
        let mut at = Some(id);
        while let Some(c) = at {
            if ui.control(c).is_some_and(|cb| !cb.visible()) {
                return false;
            }
            at = ui.parent(c);
        }
        true
    };
    let menu_shown = |ui: &Ui, text: &str| {
        ui.components().any(|id| ui.get::<aylis_controls::MenuItem>(id).is_some_and(|m| m.text == text && m.visible))
    };
    assert!(shown(&ide.ui, ide.ids.home));
    for id in [ide.ids.palette, ide.ids.inspector, ide.ids.structure, ide.ids.project] {
        assert!(!shown(&ide.ui, id), "painel à vista na tela inicial");
    }
    // ("Executar" também é um item dentro do menu: os de cima são Editar e Projeto.)
    assert!(!menu_shown(&ide.ui, "Editar") && !menu_shown(&ide.ui, "Projeto") && !menu_shown(&ide.ui, "Salvar"));
    assert!(menu_shown(&ide.ui, "Abrir projeto…") && menu_shown(&ide.ui, "Sair") && menu_shown(&ide.ui, "Opções da IDE…"));

    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    for id in [ide.ids.palette, ide.ids.inspector, ide.ids.structure, ide.ids.project] {
        assert!(shown(&ide.ui, id), "painel escondido fora da tela inicial");
    }
    assert!(menu_shown(&ide.ui, "Editar") && menu_shown(&ide.ui, "Salvar"));
}

/// Os componentes do diálogo, em ordem (para achar botões, campos e caixas).
fn descendants(ui: &Ui, root: ComponentId) -> Vec<ComponentId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        out.push(id);
        stack.extend(ui.children(id).iter().rev().copied());
    }
    out
}

#[test]
fn the_new_project_wizard_goes_step_by_step() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::new_project(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    let all = descendants(&ide.ui, dialog);
    let button = |ui: &Ui, text: &str| {
        *all.iter().find(|&&id| ui.get::<aylis_controls::Button>(id).is_some_and(|b| b.text == text)).unwrap_or_else(|| panic!("sem o botão {text}"))
    };
    let (next, back, create) = (button(&ide.ui, "Avançar"), button(&ide.ui, "Voltar"), button(&ide.ui, "Criar"));
    let labels = |ui: &Ui| all.iter().filter_map(|&id| ui.get::<aylis_controls::Label>(id).map(|l| l.text.clone())).collect::<Vec<_>>();
    let visible = |ui: &Ui, id: ComponentId| ui.control(id).is_some_and(|c| c.visible());
    let edits: Vec<ComponentId> = all.iter().copied().filter(|&id| ide.ui.get::<Edit>(id).is_some()).collect();
    let base = temp("assistente");
    ide.ui.set_prop(edits[0], "text", "Loja").unwrap();
    ide.ui.set_prop(edits[1], "text", base.display().to_string()).unwrap();
    ide.ui.fire(edits[0], "on_change", &aylis_core::EventArgs::None);
    let save = std::env::var_os("AYLIS_WIZARD").map(PathBuf::from);
    let mut shot = |ui: &mut Ui, step: usize| {
        let Some(dir) = &save else { return };
        let renderer = aylis_skia::SkiaRenderer::new();
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
        ui.layout();
        let (w, h) = ui.control(dialog).map_or((700.0, 660.0), |c| (c.width(), c.height()));
        let background = ui.theme().color("background");
        let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(dialog, c)).unwrap();
        std::fs::write(dir.join(format!("passo{}.png", step + 1)), png).unwrap();
    };
    assert!(labels(&ide.ui).iter().any(|l| l.starts_with("1 de 5")));
    assert!(!visible(&ide.ui, back) && visible(&ide.ui, next) && !visible(&ide.ui, create));
    shot(&mut ide.ui, 0);
    let click = |ui: &mut Ui, id: ComponentId| {
        let r = ui.absolute_rect(id).unwrap();
        let e = PointerEvent { position: r.center(), ..Default::default() };
        ui.pointer_down(dialog, e.clone());
        ui.pointer_up(dialog, e);
    };
    ide.ui.layout();
    click(&mut ide.ui, next);
    assert!(labels(&ide.ui).iter().any(|l| l.starts_with("2 de 5")), "{:?}", labels(&ide.ui));
    shot(&mut ide.ui, 1);
    // O modelo (o passo 2) vem antes das plataformas: o Desktop marca as do computador.
    click(&mut ide.ui, next);
    assert!(labels(&ide.ui).iter().any(|l| l.starts_with("3 de 5")), "{:?}", labels(&ide.ui));
    shot(&mut ide.ui, 2);
    let checks: Vec<ComponentId> = all.iter().copied().filter(|&id| ide.ui.get::<aylis_controls::CheckBox>(id).is_some()).collect();
    let marked: Vec<bool> = checks[..6].iter().map(|&c| ide.ui.get::<aylis_controls::CheckBox>(c).is_some_and(|c| c.checked)).collect();
    assert_eq!(marked, [true, true, true, false, false, false], "Windows, Linux e macOS");
    // Sem nenhuma plataforma, não avança.
    for &c in &checks[..6] {
        let _ = ide.ui.set_prop(c, "checked", false);
    }
    click(&mut ide.ui, next);
    assert!(labels(&ide.ui).iter().any(|l| l.starts_with("3 de 5")) && labels(&ide.ui).iter().any(|l| l.contains("plataforma")));
    let _ = ide.ui.set_prop(checks[0], "checked", true);
    let _ = ide.ui.set_prop(checks[3], "checked", true);
    for step in 3..5 {
        click(&mut ide.ui, next);
        assert!(labels(&ide.ui).iter().any(|l| l.starts_with(&format!("{} de 5", step + 1))), "{:?}", labels(&ide.ui));
        shot(&mut ide.ui, step);
    }
    assert!(visible(&ide.ui, create) && !visible(&ide.ui, next));
    let summary = labels(&ide.ui).into_iter().find(|l| l.starts_with("Pasta:")).unwrap();
    assert!(summary.contains("Windows, Android") && summary.contains("Loja (dev.aylis.loja)"), "{summary}");
    click(&mut ide.ui, back);
    assert!(labels(&ide.ui).iter().any(|l| l.starts_with("4 de 5")));
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    let st = state.borrow();
    let project = st.project.as_ref().expect("o projeto foi criado e aberto");
    assert_eq!(project.platforms, ["windows", "android"]);
    aylis_tools::trust::forget_dir(&project.dir());
}

#[test]
fn project_options_gather_what_rarely_changes() {
    let base = temp("opcoes_projeto");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    aylis_ide::projectsettings::open(&mut ide.ui, &state, &ide.ids);
    let hub = *ide.ui.open_forms().last().unwrap();
    let snapshot = |ide: &mut aylis_ide::app::Ide, name: &str| {
        let Some(dir) = std::env::var_os("AYLIS_WIZARD") else { return };
        let renderer = aylis_skia::SkiaRenderer::new();
        ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
        ide.ui.layout();
        let (w, h) = ide.ui.control(hub).map_or((1320.0, 880.0), |c| (c.width(), c.height()));
        let background = ide.ui.theme().color("background");
        let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ide.ui.paint(hub, c)).unwrap();
        std::fs::write(PathBuf::from(dir).join(name), png).unwrap();
    };
    snapshot(&mut ide, "opcoes.png");
    // As seções numa lista à esquerda (abas laterais), não em botões.
    let nav = descendants(&ide.ui, hub).into_iter().find(|&id| ide.ui.get::<aylis_controls::ListBox>(id).is_some()).expect("a lista de seções");
    let items = ide.ui.get::<aylis_controls::ListBox>(nav).unwrap().items.clone();
    let expected = ["Geral", "Aparência", "Fontes", "Ícone do app", "Android", "Windows", "Memória", "DataModules usados", "Traduções", "Confiar neste projeto"];
    assert_eq!(items.split(';').collect::<Vec<_>>(), expected);
    let buttons: Vec<String> = descendants(&ide.ui, hub).iter().filter_map(|&id| ide.ui.get::<aylis_controls::Button>(id).map(|b| b.text.clone())).collect();
    assert!(!buttons.iter().any(|b| b == "Geral" || b == "Memória"), "sem um botão por seção: {buttons:?}");
    // A seção Geral já aberta na página, com o nome do projeto.
    let edit_with = |ui: &Ui, text: &str| descendants(ui, hub).into_iter().find(|&id| ui.get::<Edit>(id).is_some_and(|e| e.text == text));
    assert!(edit_with(&ide.ui, "Loja").is_some(), "a página Geral mostra o nome");
    let pick = |ide: &mut aylis_ide::app::Ide, index: usize| {
        ide.ui.layout();
        let r = ide.ui.absolute_rect(nav).unwrap();
        let e = PointerEvent { position: PointF::new(r.x + 30.0, r.y + 38.0 * index as f32 + 19.0), ..Default::default() };
        ide.ui.pointer_down(hub, e.clone());
        ide.ui.pointer_up(hub, e);
    };
    pick(&mut ide, 6);
    snapshot(&mut ide, "opcoes_memoria.png");
    let pixels = edit_with(&ide.ui, "96").expect("a página Memória");
    // Um valor errado não deixa trocar de seção: fica nela, com o erro.
    ide.ui.get_mut::<Edit>(pixels).unwrap().text = "4".into();
    pick(&mut ide, 0);
    assert_eq!(ide.ui.get::<aylis_controls::ListBox>(nav).unwrap().item_index, 6);
    let errors: Vec<String> = descendants(&ide.ui, hub).iter().filter_map(|&id| ide.ui.get::<aylis_controls::Label>(id).map(|l| l.text.clone())).collect();
    assert!(errors.iter().any(|t| t.contains("imagens em pixels")), "o erro aparece: {errors:?}");
    // Certo, trocar de seção grava a que estava aberta.
    ide.ui.get_mut::<Edit>(pixels).unwrap().text = "200".into();
    pick(&mut ide, 0);
    assert_eq!(ide.ui.get::<aylis_controls::ListBox>(nav).unwrap().item_index, 0);
    let saved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(saved.image_cache.map(|c| c.pixels_mb), Some(200));
    assert!(edit_with(&ide.ui, "Loja").is_some(), "de volta à Geral");
    // Com AYLIS_WIZARD, a página de cada seção em PNG (para conferir que o editor coube).
    if std::env::var_os("AYLIS_WIZARD").is_some() {
        for index in 1..expected.len() {
            pick(&mut ide, index);
            snapshot(&mut ide, &format!("opcoes_{index}.png"));
        }
    }
    ide.ui.set_modal_result(hub, ModalResult::Cancel);
    assert!(!ide.ui.is_open(hub));
    aylis_tools::trust::forget_dir(&project.dir());
}

/// Opções do projeto → Memória: os limites do cache de imagens vão para o `image_cache` do
/// `.aylisproj`; fora da faixa, o diálogo não fecha; de volta ao padrão, a chave sai.
#[test]
fn project_memory_saves_the_image_cache_limits() {
    let base = temp("memoria_projeto");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let set = |ide: &mut aylis_ide::app::Ide, values: [&str; 3]| {
        actions::project_memory(&mut ide.ui, &state, &ide.ids);
        let dialog = *ide.ui.open_forms().last().unwrap();
        assert!(ide.ui.get::<Form>(dialog).is_some_and(|f| f.caption.contains("Memória")));
        if let Some(dir) = std::env::var_os("AYLIS_WIZARD") {
            let renderer = aylis_skia::SkiaRenderer::new();
            ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
            ide.ui.layout();
            let (w, h) = ide.ui.control(dialog).map_or((640.0, 440.0), |c| (c.width(), c.height()));
            let background = ide.ui.theme().color("background");
            let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ide.ui.paint(dialog, c)).unwrap();
            std::fs::write(PathBuf::from(dir).join("memoria.png"), png).unwrap();
        }
        let edits: Vec<ComponentId> = descendants(&ide.ui, dialog).into_iter().filter(|&id| ide.ui.get::<Edit>(id).is_some()).collect();
        assert_eq!(edits.len(), 3, "pixels, arquivos e camadas");
        for (edit, value) in edits.iter().zip(values) {
            ide.ui.get_mut::<Edit>(*edit).unwrap().text = value.into();
        }
        ide.ui.set_modal_result(dialog, ModalResult::Ok);
        dialog
    };
    let dialog = set(&mut ide, ["4", "256", "128"]);
    assert!(ide.ui.is_open(dialog), "4 MB está abaixo da faixa: o diálogo continua aberto");
    ide.ui.set_modal_result(dialog, ModalResult::Cancel);
    let dialog = set(&mut ide, ["96", "256", "5000"]);
    assert!(ide.ui.is_open(dialog), "5000 MB de camadas passa da faixa: o diálogo continua aberto");
    ide.ui.set_modal_result(dialog, ModalResult::Cancel);
    set(&mut ide, ["200", "1000", "64"]);
    let saved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(saved.image_cache, Some(aylis_project::ImageCacheSettings { pixels_mb: 200, files: 1000, layers_mb: 64, auto_layers: true }));
    set(&mut ide, ["96", "256", "128"]);
    assert_eq!(aylis_project::Project::open(&project.path).unwrap().image_cache, None, "no padrão, a chave sai do .aylisproj");
    aylis_tools::trust::forget_dir(&project.dir());
}

/// O tema próprio do projeto (`themes/*.json`, sobre um do aylis) aparece no designer ao
/// abrir, no modo dele, e o "Form claro / Form escuro" troca para o outro da mesma família
/// (antes caía no Aylis Light, e o form não mostrava o visual do app).
#[test]
fn the_project_theme_file_shows_in_the_designer() {
    let base = temp("tema_projeto");
    let mut project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    std::fs::create_dir_all(project.dir().join("themes")).unwrap();
    std::fs::write(project.dir().join("themes/loja-dark.json"), r##"{ "name": "Loja Dark", "base": "Aylis Dark", "family": "Loja", "dark": true, "colors": { "primary": "#10B981" } }"##).unwrap();
    std::fs::write(project.dir().join("themes/loja-light.json"), r##"{ "name": "Loja Light", "base": "Aylis Light", "family": "Loja", "dark": false, "colors": { "primary": "#059669" } }"##).unwrap();
    project.theme = "themes/loja-dark.json".into();
    project.save().unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::set_dark(&state, false);
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    assert_eq!(state.borrow().doc.theme.name, "Loja Dark", "abre no tema do app, não na escolha anterior da IDE");
    assert!(state.borrow().doc.dark);
    actions::set_dark(&state, false);
    assert_eq!(state.borrow().doc.theme.name, "Loja Light", "o claro da mesma família, do projeto");
    actions::set_dark(&state, true);
    assert_eq!(state.borrow().doc.theme.name, "Loja Dark");
    aylis_tools::trust::forget_dir(&project.dir());
}

/// A faixa de abas da IDE (a `TabStrip` em cima do designer e do código).
fn tab_strip(ide: &aylis_ide::app::Ide) -> ComponentId {
    ide.ui.children(ide.ids.tabs).iter().copied().find(|&c| ide.ui.get::<aylis_controls::TabStrip>(c).is_some()).expect("a faixa de abas")
}

/// Os títulos das abas da IDE, com " •" nas que têm alteração não salva.
fn strip_titles(ide: &aylis_ide::app::Ide) -> Vec<String> {
    ide.ui.get::<aylis_controls::TabStrip>(tab_strip(ide)).unwrap().tabs().iter().map(|t| if t.modified { format!("{} •", t.title) } else { t.title.clone() }).collect()
}

/// A roda sobre um ScrollBox do form no designer rola ele, como no app, para ver e editar o
/// que está abaixo (relatado pelo autor em 07/11: na Landing, "a scroll interna quando o
/// componente está no design não funciona"). A vista do designer fica onde estava, e o clique
/// depois acha o componente que ficou à vista.
#[test]
fn the_wheel_scrolls_a_scroll_box_of_the_designed_form() {
    use aylis_ide::surface::DesignSurface;
    let buttons: Vec<String> = (0..20).map(|i| format!(r#"{{ "class": "Button", "name": "btn{i}", "text": "{i}", "align": "top", "height": 60 }}"#)).collect();
    let json = format!(
        r#"{{ "aylis": 1, "root": {{ "class": "Form", "name": "frmRolar", "width": 400, "height": 300,
            "children": [{{ "class": "ScrollBox", "name": "scrConteudo", "align": "client", "children": [{}] }}] }} }}"#,
        buttons.join(",")
    );
    let state = shared(Document::from_json(&json).unwrap());
    let mut ide = app::build(state.clone());
    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let s = ide.ui.absolute_rect(ide.ids.surface).unwrap();
    let origin = ide.ui.get::<DesignSurface>(ide.ids.surface).unwrap().origin();
    let zoom = state.borrow().zoom;
    let doc_point = PointF::new(200.0, 150.0);
    let at = PointF::new(s.x + origin.x + doc_point.x * zoom, s.y + origin.y + doc_point.y * zoom);
    let hit = |state: &aylis_ide::state::Shared| {
        let st = state.borrow();
        st.doc.component_at(doc_point).and_then(|id| st.doc.ui.name(id).map(str::to_string))
    };
    assert_eq!(hit(&state).as_deref(), Some("btn2"), "antes de rolar");

    for _ in 0..3 {
        ide.ui.wheel(ide.form, aylis_core::WheelEvent { position: at, delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
    }
    let scroll = {
        let st = state.borrow();
        let id = st.doc.ui.find_in(st.doc.root, "scrConteudo").unwrap();
        st.doc.ui.control(id).unwrap().scroll_offset().y
    };
    assert_eq!(scroll, 300.0, "rolou na hora (sem deslize)");
    assert_eq!(ide.ui.get::<DesignSurface>(ide.ids.surface).unwrap().origin(), origin, "a vista do designer ficou");
    assert_eq!(hit(&state).as_deref(), Some("btn7"), "o clique acha o que ficou à vista");
}

/// Os grupos do Object Inspector recolhem e abrem como as categorias da paleta (pedido do
/// autor em 07/11), e a busca abre os recolhidos.
#[test]
fn the_inspector_groups_collapse_and_open() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    state.borrow_mut().home = false;
    let button = state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    state.borrow_mut().doc.select(Some(button));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let texts = |ui: &Ui| -> Vec<(aylis_core::ComponentId, String)> {
        let mut all = vec![ide.ids.inspector];
        let mut i = 0;
        while i < all.len() {
            all.extend(ui.children(all[i]).iter().copied());
            i += 1;
        }
        all.into_iter().filter_map(|id| ui.get::<aylis_controls::Label>(id).map(|l| (id, l.text.clone()))).collect()
    };
    let group = aylis_ide::lang::category_label("Layout");
    let height = aylis_ide::lang::property_label("height");
    let has = |ui: &Ui, t: &str| texts(ui).iter().any(|(_, x)| x == t);
    assert!(has(&ide.ui, &group) && has(&ide.ui, &height), "o grupo aberto: {:?}", texts(&ide.ui).iter().map(|t| &t.1).collect::<Vec<_>>());
    let header = |ui: &Ui| {
        let label = texts(ui).into_iter().find(|(_, t)| *t == group).unwrap().0;
        ui.parent(label).unwrap()
    };
    let title = texts(&ide.ui).into_iter().find(|(_, t)| *t == group).unwrap().0;
    assert_eq!(ide.ui.get_prop(title, "font_size").ok().and_then(|v| v.to_f64().ok()), Some(13.0), "o título maior que os 11 de antes");

    let h = header(&ide.ui);
    ide.ui.fire(h, "on_click", &aylis_core::EventArgs::None);
    assert!(has(&ide.ui, &group) && !has(&ide.ui, &height), "recolhido: o título fica, a Altura sai: {:?}", texts(&ide.ui).iter().map(|t| &t.1).collect::<Vec<_>>());
    // Outro componente: o grupo continua recolhido.
    let other = state.borrow_mut().doc.add("Edit", Some(PointF::new(40.0, 120.0))).unwrap();
    state.borrow_mut().doc.select(Some(other));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(!has(&ide.ui, &height), "vale para todos os componentes");
    // Com busca, abre.
    state.borrow_mut().inspector_filter = "alt".into();
    aylis_ide::inspector::rebuild(&mut ide.ui, &state, &ide.ids);
    assert!(has(&ide.ui, &height), "a busca mostra o que está recolhido");
    state.borrow_mut().inspector_filter.clear();
    aylis_ide::inspector::rebuild(&mut ide.ui, &state, &ide.ids);
    let h = header(&ide.ui);
    ide.ui.fire(h, "on_click", &aylis_core::EventArgs::None);
    assert!(has(&ide.ui, &height), "aberto de novo");
}

/// O retângulo de seleção começa também sobre um contêiner alinhado (na Landing, o ScrollBox e
/// os layouts cobrem todo o form e não sobrava fundo livre), e a seleção de um componente que
/// a rolagem escondeu não aparece nem se pega pelas alças (relatados pelo autor em 07/11).
#[test]
fn the_selection_area_starts_over_an_aligned_container_and_hides_when_scrolled_away() {
    use aylis_ide::surface::DesignSurface;
    let json = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmArea", "width": 400, "height": 300,
        "children": [{ "class": "ScrollBox", "name": "scrConteudo", "align": "client", "children": [
            { "class": "Button", "name": "btnA", "left": 20, "top": 20, "width": 80, "height": 30 },
            { "class": "Button", "name": "btnB", "left": 120, "top": 20, "width": 80, "height": 30 },
            { "class": "Button", "name": "btnC", "left": 20, "top": 200, "width": 80, "height": 30 },
            { "class": "Button", "name": "btnFim", "left": 20, "top": 900, "width": 80, "height": 30 } ] }] } }"#;
    let state = shared(Document::from_json(json).unwrap());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let s = ide.ui.absolute_rect(ide.ids.surface).unwrap();
    let origin = ide.ui.get::<DesignSurface>(ide.ids.surface).unwrap().origin();
    let zoom = state.borrow().zoom;
    let screen = |p: PointF| PointF::new(s.x + origin.x + p.x * zoom, s.y + origin.y + p.y * zoom);
    let names = |state: &aylis_ide::state::Shared| {
        let st = state.borrow();
        let mut n: Vec<String> = st.doc.selected.iter().filter_map(|&id| st.doc.ui.name(id).map(str::to_string)).collect();
        n.sort();
        n
    };

    // Arrasto a partir do ScrollBox (alinhado: não se move) cobrindo btnA e btnB.
    let (from, to) = (screen(PointF::new(10.0, 10.0)), screen(PointF::new(210.0, 60.0)));
    ide.ui.pointer_move(ide.form, PointerEvent { position: from, ..Default::default() });
    ide.ui.pointer_down(ide.form, PointerEvent { position: from, ..Default::default() });
    ide.ui.pointer_move(ide.form, PointerEvent { position: to, ..Default::default() });
    ide.ui.pointer_up(ide.form, PointerEvent { position: to, ..Default::default() });
    assert_eq!(names(&state), ["btnA", "btnB"], "o retângulo dentro do ScrollBox");

    // Rola até o fim: btnA saiu da vista, e a seleção dele também.
    let btn_a = {
        let st = state.borrow();
        st.doc.ui.find_in(st.doc.root, "btnA").unwrap()
    };
    state.borrow_mut().doc.select(Some(btn_a));
    for _ in 0..10 {
        ide.ui.wheel(ide.form, aylis_core::WheelEvent { position: screen(PointF::new(300.0, 150.0)), delta: PointF::new(0.0, -100.0), lines: true, ..Default::default() });
    }
    let st = state.borrow();
    let r = st.doc.rect_of(btn_a).unwrap();
    let area = st.doc.visible_area(btn_a).expect("dentro de um ScrollBox");
    assert!(!area.intersects(r), "btnA ficou fora da área visível: {r:?} {area:?}");
}

/// As Opções do projeto: cada página ocupa a área visível e só rola quando o conteúdo não
/// cabe, até o fim dele (relatado pelo autor em 07/11: a Fontes tinha barra sem nada para
/// rolar, e o fim da Geral ficava cortado).
#[test]
fn the_project_options_pages_scroll_only_when_needed_and_to_the_end() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    // Uma cópia da Landing (o modelo "Landing page"), não o exemplo: fechar as Opções aplica a
    // seção, e o teste regravava o `Landing.aylisproj` do repositório.
    let base = temp("opcoes_paginas");
    let mut project = aylis_project::create(&base, "Pagina", aylis_project::Language::Rust).unwrap();
    aylis_project::apply_example(&mut project, "Landing").unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let titles = aylis_ide::projectsettings::section_titles();
    for name in ["Geral", "Fontes", "Memória", "Traduções"] {
        let index = titles.iter().position(|t| *t == name).unwrap();
        aylis_ide::projectsettings::open_at(&mut ide.ui, &state, &ide.ids, index);
        let dialog = *ide.ui.open_forms().last().unwrap();
        ide.ui.layout();
        let mut all = vec![dialog];
        let mut k = 0;
        while k < all.len() {
            all.extend(ide.ui.children(all[k]).iter().copied());
            k += 1;
        }
        let host = all.iter().copied().find(|&c| ide.ui.get::<aylis_controls::ScrollBox>(c).is_some()).unwrap();
        let page = *ide.ui.children(host).last().unwrap();
        let body = ide.ui.children(page).iter().copied().find(|&c| ide.ui.control(c).unwrap().align() == aylis_core::Align::Client).unwrap();
        let (b, h) = (ide.ui.control(body).unwrap(), ide.ui.control(host).unwrap());
        assert!(b.content_size().height <= b.bounds().height + 0.5, "{name}: o conteúdo cabe na página ({} > {})", b.content_size().height, b.bounds().height);
        if name != "Geral" {
            assert!(h.content_size().height <= h.bounds().height + 0.5, "{name}: cabe sem rolar ({} > {})", h.content_size().height, h.bounds().height);
        }
        ide.ui.close(dialog);
    }
    let _ = std::fs::remove_dir_all(base);
}

/// O retângulo de seleção entra na barra que ele só corta e pega o que cabe dentro dela (no
/// NoOBS, o campo de busca e os botões de uma barra da largura toda; relatado pelo autor em
/// 07/11); o que cabe inteiro entra como um só.
#[test]
fn the_selection_area_picks_what_fits_inside_a_wider_bar() {
    let json = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmLib", "width": 800, "height": 400,
        "children": [{ "class": "Layout", "name": "lyLibrary", "align": "top", "height": 300, "children": [
            { "class": "Layout", "name": "lyTools", "align": "top", "height": 40, "children": [
                { "class": "Edit", "name": "edBusca", "align": "left", "width": 200 },
                { "class": "Button", "name": "btnA", "align": "left", "width": 30, "margins": [8, 0, 0, 0] },
                { "class": "Button", "name": "btnB", "align": "left", "width": 30, "margins": [8, 0, 0, 0] } ] },
            { "class": "Panel", "name": "pnlCaixa", "left": 20, "top": 120, "width": 100, "height": 60, "children": [
                { "class": "Button", "name": "btnDentro", "left": 10, "top": 10, "width": 60, "height": 30 } ] } ] }] } }"#;
    let doc = Document::from_json(json).unwrap();
    let names = |ids: Vec<ComponentId>| -> Vec<String> { ids.iter().filter_map(|&id| doc.ui.name(id).map(str::to_string)).collect() };
    // Da esquerda até o segundo botão, na altura da barra: os três de dentro dela.
    let found = names(doc.components_in(aylis_core::RectF::new(-2.0, -2.0, 290.0, 44.0)));
    assert_eq!(found, ["edBusca", "btnA", "btnB"]);
    // O painel inteiro no retângulo: ele, não o botão de dentro.
    let found = names(doc.components_in(aylis_core::RectF::new(10.0, 110.0, 200.0, 100.0)));
    assert_eq!(found, ["pnlCaixa"]);
    // Fino, começando dentro do campo e passando pelos botões (a imagem do autor): os que toca.
    let found = names(doc.components_in(aylis_core::RectF::new(120.0, 15.0, 160.0, 8.0)));
    assert_eq!(found, ["edBusca", "btnA", "btnB"]);
    // Só a borda do painel, longe do botão de dentro: o painel.
    let found = names(doc.components_in(aylis_core::RectF::new(10.0, 170.0, 30.0, 20.0)));
    assert_eq!(found, ["pnlCaixa"]);
}
