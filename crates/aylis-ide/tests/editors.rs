//! Os editores visuais do Object Inspector: relógio de ângulo, sombra, curva, tempos da
//! animação e cor. Com `AYLIS_SHOTS=pasta`, cada diálogo vira um PNG para conferir.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use aylis_controls::{AngleDial, Button, EasingEditor, Edit, Label, Rectangle, ShadowEditor, TimingEditor};
use aylis_core::{ComponentId, ModalResult, PointF, PointerEvent, PropValue, Ui};
use aylis_ide::document::Document;
use aylis_ide::propeditors::{self, Timing};
use aylis_ide::state::shared;
use aylis_ide::{app, dialogs};

fn all(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

fn text(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

fn shot(ui: &mut Ui, form: ComponentId, name: &str) {
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.set_time(0.45);
    ui.layout();
    let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
    std::fs::write(PathBuf::from(dir).join(name), png).unwrap();
}

fn click(ui: &mut Ui, root: ComponentId, id: ComponentId) {
    let r = ui.absolute_rect(id).unwrap();
    let e = PointerEvent { position: PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0), ..Default::default() };
    ui.pointer_move(root, e);
    ui.pointer_down(root, e);
    ui.pointer_up(root, e);
}

/// Abre um diálogo num `Ui` com o tema da IDE e devolve o form dele.
fn open(ui: &mut Ui) -> ComponentId {
    let dialog = *ui.open_forms().last().unwrap();
    assert!(ui.is_modal(dialog));
    ui.layout();
    dialog
}

fn host() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.add_root(aylis_controls::Form::new("IDE", 1000.0, 700.0));
    ui.show(main).unwrap();
    (ui, main)
}

fn edits(ui: &Ui, dialog: ComponentId) -> Vec<ComponentId> {
    all(ui, dialog).into_iter().filter(|&id| ui.get::<Edit>(id).is_some()).collect()
}

fn set_text(ui: &mut Ui, edit: ComponentId, value: &str) {
    ui.get_mut::<Edit>(edit).unwrap().text = value.into();
    ui.fire(edit, "on_change", &aylis_core::EventArgs::Value(value.into()));
}

#[test]
fn angle_editor_returns_the_degrees() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    propeditors::angle_editor(&mut ui, "Ângulo: rotation", 30.0, move |_, v| *g.borrow_mut() = Some(v));
    let d = open(&mut ui);
    let dial = all(&ui, d).into_iter().find(|&id| ui.get::<AngleDial>(id).is_some()).expect("o relógio");
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 30.0);
    // O atalho de 135° muda o relógio e o campo.
    let b135 = all(&ui, d).into_iter().find(|&id| ui.get::<Button>(id).is_some() && text(&ui, id) == "135°").unwrap();
    click(&mut ui, d, b135);
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 135.0);
    // Digitar move o ponteiro.
    let e = edits(&ui, d)[0];
    set_text(&mut ui, e, "200");
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 200.0);
    shot(&mut ui, d, "editor_angulo.png");
    ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(*got.borrow(), Some(200.0));
}

#[test]
fn shadow_editor_returns_the_spec_or_empty() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    propeditors::shadow_editor(&mut ui, "0 4 12 #00000040", Vec::new(), move |_, v| *g.borrow_mut() = Some(v));
    let d = open(&mut ui);
    let ed = all(&ui, d).into_iter().find(|&id| ui.get::<ShadowEditor>(id).is_some()).unwrap();
    assert_eq!(ui.get::<ShadowEditor>(ed).unwrap().blur, 12.0);
    // Uma pronta preenche o editor e o valor.
    let elevated = all(&ui, d).into_iter().find(|&id| ui.get::<Button>(id).is_some() && text(&ui, id) == "Elevada").unwrap();
    click(&mut ui, d, elevated);
    assert_eq!(ui.get::<ShadowEditor>(ed).unwrap().blur, 28.0);
    let fields = edits(&ui, d);
    let (color, spec) = (fields[0], fields[1]);
    assert_eq!(text(&ui, spec), "0 12 28 #00000047");
    // Mudar a cor reescreve o valor.
    set_text(&mut ui, color, "#2563EB66");
    assert_eq!(text(&ui, spec), "0 12 28 #2563EB66");
    shot(&mut ui, d, "editor_sombra.png");
    ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(got.borrow().as_deref(), Some("0 12 28 #2563EB66"));

    // "Nenhuma" devolve vazio.
    let g = got.clone();
    propeditors::shadow_editor(&mut ui, "", Vec::new(), move |_, v| *g.borrow_mut() = Some(v));
    let d = open(&mut ui);
    let none = all(&ui, d).into_iter().find(|&id| ui.get::<Button>(id).is_some() && text(&ui, id) == "Nenhuma").unwrap();
    click(&mut ui, d, none);
    ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(got.borrow().as_deref(), Some(""));
}

#[test]
fn easing_editor_picks_a_preset_or_a_curve() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    propeditors::easing_editor(&mut ui, "out_quad", move |_, v| *g.borrow_mut() = Some(v));
    let d = open(&mut ui);
    let editor = all(&ui, d).into_iter().find(|&id| ui.get::<EasingEditor>(id).is_some_and(|e| !e.compact)).unwrap();
    let row = all(&ui, d)
        .into_iter()
        .find(|&id| ui.get::<Rectangle>(id).is_some() && ui.children(id).iter().any(|&c| ui.get::<Label>(c).is_some() && text(&ui, c) == "Material 3: padrão"))
        .expect("a linha da curva do Material 3");
    ui.click(row);
    let e = ui.get::<EasingEditor>(editor).unwrap();
    assert_eq!((e.x1, e.y1, e.x2, e.y2), (0.2, 0.0, 0.0, 1.0));
    shot(&mut ui, d, "editor_curva.png");
    ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(got.borrow().as_deref(), Some("cubic-bezier(0.2, 0, 0, 1)"));
}

#[test]
fn timing_editor_returns_every_time() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    let initial = Timing { duration: 300.0, delay: Some(100.0), easing: Some("ease-out".into()), auto_reverse: Some(true), looping: Some(false) };
    propeditors::timing_editor(&mut ui, "Tempos da animação", initial, move |_, t| *g.borrow_mut() = Some(t));
    let d = open(&mut ui);
    let timeline = all(&ui, d).into_iter().find(|&id| ui.get::<TimingEditor>(id).is_some()).unwrap();
    let t = ui.get::<TimingEditor>(timeline).unwrap();
    assert_eq!((t.delay, t.duration, t.auto_reverse), (100.0, 300.0, true));
    let fields = edits(&ui, d);
    set_text(&mut ui, fields[0], "450");
    assert_eq!(ui.get::<TimingEditor>(timeline).unwrap().duration, 450.0, "o campo muda a linha do tempo");
    shot(&mut ui, d, "editor_tempos.png");
    ui.set_modal_result(d, ModalResult::Ok);
    let t = got.borrow().clone().unwrap();
    assert_eq!(t, Timing { duration: 450.0, delay: Some(100.0), easing: Some("ease-out".into()), auto_reverse: Some(true), looping: Some(false) });
}

#[test]
fn color_picker_has_the_hsv_board_and_the_theme_tokens() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    let tokens = vec![("primary".to_string(), aylis_core::Color::rgb(0x25, 0x63, 0xEB))];
    dialogs::color_picker(&mut ui, "primary", tokens, move |_, v| *g.borrow_mut() = Some(v));
    let d = open(&mut ui);
    let picker = all(&ui, d).into_iter().find(|&id| ui.get::<aylis_controls::ColorPicker>(id).is_some()).expect("o quadro de cores");
    assert_eq!(ui.get::<aylis_controls::ColorPicker>(picker).unwrap().color, aylis_core::Color::rgb(0x25, 0x63, 0xEB), "o token abre no quadro");
    shot(&mut ui, d, "editor_cor.png");
    ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(got.borrow().as_deref(), Some("primary"));
}

#[test]
fn the_inspector_picks_the_editor_by_property_type() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    let button = state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    state.borrow_mut().doc.select(Some(button));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let line_of = |ui: &Ui, name: &str| {
        ui.children(ide.ids.inspector)
            .iter()
            .copied()
            // Pelo nome técnico (na dica do rótulo, que mostra o legível).
            .find(|&l| ui.children(l).first().is_some_and(|&lbl| matches!(ui.get_prop(lbl, "hint"), Ok(PropValue::String(h)) if h == name)))
            .unwrap_or_else(|| panic!("linha '{name}'"))
    };
    // rotation: o relógio na linha; shadow: a amostra com a sombra.
    let rotation = line_of(&ide.ui, "rotation");
    assert!(ide.ui.children(rotation).iter().any(|&c| ide.ui.get::<AngleDial>(c).is_some()));
    let shadow = line_of(&ide.ui, "shadow");
    assert!(ide.ui.children(shadow).iter().any(|&c| ide.ui.get::<Rectangle>(c).is_some()));
    shot(&mut ide.ui, form, "inspector_editores.png");

    // "…" do rotation abre o relógio; OK grava no componente.
    let more = ide.ui.children(rotation).iter().copied().find(|&c| text(&ide.ui, c) == "…").unwrap();
    ide.ui.click(more);
    let d = open(&mut ide.ui);
    let e = edits(&ide.ui, d)[0];
    set_text(&mut ide.ui, e, "15");
    ide.ui.set_modal_result(d, ModalResult::Ok);
    assert_eq!(state.borrow().doc.ui.control(button).unwrap().rotation(), 15.0);

    // A sombra pelo editor.
    ide.ui.layout();
    let shadow = line_of(&ide.ui, "shadow");
    let more = ide.ui.children(shadow).iter().copied().find(|&c| text(&ide.ui, c) == "…").unwrap();
    ide.ui.click(more);
    let d = open(&mut ide.ui);
    let media = all(&ide.ui, d).into_iter().find(|&id| ide.ui.get::<Button>(id).is_some() && text(&ide.ui, id) == "Média").unwrap();
    click(&mut ide.ui, d, media);
    ide.ui.set_modal_result(d, ModalResult::Ok);
    let s = state.borrow().doc.ui.control(button).unwrap().shadow().expect("com sombra");
    assert_eq!(s.to_spec(), "0 4 12 #00000033");

    // Uma FloatAnimation: a duração abre a linha do tempo com atraso, curva e repetir.
    let anim = state.borrow_mut().doc.ui.add(aylis_core::FloatAnimation::default(), button);
    state.borrow_mut().doc.select(Some(anim));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    let duration = line_of(&ide.ui, "duration");
    let more = ide.ui.children(duration).iter().copied().find(|&c| text(&ide.ui, c) == "…").unwrap();
    ide.ui.click(more);
    let d = open(&mut ide.ui);
    let t = all(&ide.ui, d).into_iter().find_map(|id| ide.ui.get::<TimingEditor>(id).map(|t| (t.duration, t.delay))).unwrap();
    assert_eq!(t, (200.0, 0.0));
    let fields = edits(&ide.ui, d);
    assert_eq!(fields.len(), 3, "duração, atraso e curva");
    set_text(&mut ide.ui, fields[1], "80");
    set_text(&mut ide.ui, fields[2], "ease-in");
    ide.ui.set_modal_result(d, ModalResult::Ok);
    let st = state.borrow();
    let a = st.doc.ui.get::<aylis_core::FloatAnimation>(anim).unwrap();
    assert_eq!((a.0.delay, a.0.easing), (80.0, aylis_core::Easing::parse("ease-in").unwrap()));
}

#[test]
fn font_editor_returns_family_size_weight_and_styles_with_a_live_preview() {
    let (mut ui, _) = host();
    let got = Rc::new(RefCell::new(None));
    let g = got.clone();
    let initial = propeditors::FontChoice { family: "Inter".into(), size: Some(14.0), ..Default::default() };
    propeditors::font_editor(&mut ui, initial, vec!["Inter".into(), "Roboto".into()], Vec::new(), move |_, c| *g.borrow_mut() = Some(c));
    let d = open(&mut ui);
    let list = all(&ui, d).into_iter().find(|&id| ui.get::<aylis_controls::ListBox>(id).is_some()).expect("a lista de famílias");
    let items = ui.get::<aylis_controls::ListBox>(list).unwrap().items.clone();
    assert!(items.starts_with("(a herdada);Inter;Roboto;Segoe UI"), "as do projeto primeiro: {items}");
    assert_eq!(ui.get::<aylis_controls::ListBox>(list).unwrap().item_index, 1, "a atual vem escolhida");
    let preview = all(&ui, d).into_iter().find(|&id| ui.get::<Label>(id).is_some_and(|l| l.text.starts_with("Ação"))).unwrap();
    // Escolher outra família na lista preenche o campo e a prévia.
    ui.set_focus(list);
    ui.key_down(d, aylis_core::KeyEvent { key: aylis_core::Key::Down, ..Default::default() });
    let fields = edits(&ui, d);
    let (family, size) = (fields[0], fields[1]);
    assert_eq!(text(&ui, family), "Roboto");
    assert_eq!(ui.get_prop(preview, "font_family").unwrap(), PropValue::String("Roboto".into()));
    set_text(&mut ui, size, "18,5");
    assert_eq!(ui.get_prop(preview, "font_size").unwrap(), PropValue::Float(18.5));
    // Peso e estilos: o combo (negrito) e as marcas (itálico, sublinhado, tachado).
    let weight = all(&ui, d).into_iter().find(|&id| ui.get::<aylis_controls::ComboBox>(id).is_some()).unwrap();
    ui.set_prop(weight, "item_index", 6).unwrap();
    ui.fire(weight, "on_change", &aylis_core::EventArgs::None);
    let checks: Vec<_> = all(&ui, d).into_iter().filter(|&id| ui.get::<aylis_controls::CheckBox>(id).is_some()).collect();
    assert_eq!(checks.len(), 3);
    click(&mut ui, d, checks[0]);
    click(&mut ui, d, checks[1]);
    assert_eq!(ui.get_prop(preview, "font_weight").unwrap(), PropValue::Enum("bold".into()));
    assert_eq!(ui.get_prop(preview, "font_italic").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(preview, "font_underline").unwrap(), PropValue::Bool(true));
    shot(&mut ui, d, "editor_fonte.png");
    // Tamanho inválido não fecha.
    set_text(&mut ui, size, "grande");
    ui.set_modal_result(d, ModalResult::Ok);
    assert!(got.borrow().is_none());
    set_text(&mut ui, size, "18,5");
    ui.set_modal_result(d, ModalResult::Ok);
    let expected = propeditors::FontChoice { family: "Roboto".into(), size: Some(18.5), weight: "bold".into(), italic: true, underline: true, strikeout: false };
    assert_eq!(*got.borrow(), Some(expected));
}

#[test]
fn the_code_window_highlights_saves_and_goes_to_the_handler_line() {
    let (mut ui, _) = host();
    let dir = std::env::temp_dir().join(format!("aylis_codigo_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("principal.rs");
    std::fs::write(&file, "use aylis::prelude::*;\n\npub fn btn_ok_click(ui: &mut Ui) {\n    // aqui\n}\n").unwrap();
    let w = aylis_ide::codewindow::open(&mut ui, &file, 3).unwrap();
    let memo = ui.get::<aylis_controls::Memo>(w.memo).unwrap();
    assert_eq!(memo.syntax, "rust");
    assert!(memo.line_numbers && memo.want_tabs);
    assert_eq!(memo.caret_position(), (2, 0), "no handler");
    assert_eq!(ui.focused(w.form), Some(w.memo));
    // Digitar marca como não salvo; Salvar grava.
    ui.text_input(w.form, "// novo\n");
    assert!(text(&ui, w.status).starts_with("● não salvo"));
    ui.layout();
    click(&mut ui, w.form, w.save);
    let saved = std::fs::read_to_string(&file).unwrap();
    assert!(saved.contains("// novo\npub fn btn_ok_click"), "{saved:?} / {}", text(&ui, w.status));
    assert!(text(&ui, w.status).starts_with("Ln 4"), "{}", text(&ui, w.status));
    shot(&mut ui, w.form, "editor_codigo.png");
    // Abrir de novo traz a mesma janela, na linha pedida.
    let again = aylis_ide::codewindow::open(&mut ui, &file, 1).unwrap();
    assert_eq!(again.form, w.form);
    assert_eq!(ui.get::<aylis_controls::Memo>(w.memo).unwrap().caret_position(), (0, 0));
    // Fechar com alteração: avisa uma vez; de novo, fecha.
    ui.text_input(w.form, "x");
    assert!(!ui.close(w.form));
    assert!(text(&ui, w.status).contains("não salvas"));
    assert!(ui.close(w.form));
    assert_eq!(aylis_ide::codewindow::syntax_of(std::path::Path::new("main.py")), "python");
    let _ = std::fs::remove_dir_all(dir);
}

/// Ctrl+B na janela de código (o atalho do menu Exibir e o do editor são o mesmo): marca a
/// linha com o próximo número só uma vez, e em cima do marcador tira.
#[test]
fn ctrl_b_in_the_code_window_marks_once() {
    let (mut ui, _) = host();
    let dir = std::env::temp_dir().join(format!("aylis_ctrl_b_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("principal.rs");
    std::fs::write(&file, "fn a() {}\nfn b() {}\n").unwrap();
    let w = aylis_ide::codewindow::open(&mut ui, &file, 1).unwrap();
    let ctrl_b = aylis_core::KeyEvent { key: aylis_core::Key::Char('b'), modifiers: aylis_core::Modifiers { ctrl: true, ..Default::default() }, ..Default::default() };
    let marks = |ui: &aylis_core::Ui| ui.get::<aylis_controls::Memo>(w.memo).unwrap().bookmarks().iter().map(|b| (b.line, b.number)).collect::<Vec<_>>();
    ui.key_down(w.form, ctrl_b);
    assert_eq!(marks(&ui), vec![(0, Some(1))]);
    ui.key_down(w.form, ctrl_b);
    assert!(marks(&ui).is_empty());
    assert!(ui.close(w.form));
    let _ = std::fs::remove_dir_all(dir);
}
