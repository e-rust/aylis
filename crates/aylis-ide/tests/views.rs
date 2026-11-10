//! As vistas do form no designer: qualquer propriedade mexida na vista fica só nela, mudar
//! um componente de contêiner também, e as vistas se criam, renomeiam e apagam (com desfazer).

use aylis_core::{FormView, PointF, PropValue, ViewOrientation};
use aylis_ide::document::Document;

fn saved(doc: &Document) -> serde_json::Value {
    serde_json::from_str(&doc.to_json().unwrap()).unwrap()
}

fn child<'a>(node: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    node["children"].as_array().unwrap().iter().find(|c| c["name"] == name).unwrap_or_else(|| panic!("{name} em {node}"))
}

fn view<'a>(json: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    json["root"]["views"].as_array().unwrap().iter().find(|v| v["name"] == name).unwrap()
}

/// Fonte, padding e colunas da grade mexidos no celular: vão para a vista; o base fica.
#[test]
fn any_property_edited_in_a_view_stays_in_the_view() {
    let mut doc = Document::new();
    let label = doc.add("Label", Some(PointF::new(40.0, 40.0))).unwrap();
    let grid = doc.add("GridLayout", Some(PointF::new(40.0, 120.0))).unwrap();
    let (label_name, grid_name) = (doc.ui.name(label).unwrap().to_string(), doc.ui.name(grid).unwrap().to_string());
    doc.enter_view(Some("phone"), 390.0);
    doc.begin_change();
    doc.ui.set_prop(label, "font_size", 30.0).unwrap();
    doc.ui.set_prop(grid, "columns", 1_i64).unwrap();
    doc.ui.set_prop(doc.root, "padding", aylis_core::Bounds::all(6.0)).unwrap();
    doc.sync_view();
    let json = saved(&doc);
    let phone = view(&json, "phone");
    assert_eq!(phone["changes"][&label_name]["font_size"], 30.0, "{phone}");
    assert_eq!(phone["changes"][&grid_name]["columns"], 1, "{phone}");
    assert_eq!(phone["changes"][""]["padding"], 6.0, "o padding do form também: {phone}");
    assert!(child(&json["root"], &label_name).get("font_size").is_none(), "o base sem a fonte");
    assert_ne!(json["root"].get("padding"), Some(&serde_json::json!(6.0)), "o form base sem o padding da vista");
    assert!(phone["changes"].get("").and_then(|c| c.get("width")).is_none(), "a largura da prévia não entra");

    // De volta ao base, a fonte é a do desenho; no celular de novo, a da vista.
    doc.enter_view(None, 0.0);
    let label = doc.ui.find(&label_name).unwrap();
    assert_ne!(doc.ui.get_prop(label, "font_size").unwrap(), PropValue::Float(30.0));
    doc.enter_view(Some("phone"), 390.0);
    assert_eq!(doc.ui.get_prop(doc.ui.find(&label_name).unwrap(), "font_size").unwrap(), PropValue::Float(30.0));
}

/// No celular, o botão vai do painel lateral para o rodapé: a vista guarda o contêiner e a
/// posição; o JSON, a árvore do base; "Voltar ao base" devolve o lugar.
#[test]
fn moving_a_component_in_a_view_keeps_the_base_tree() {
    let mut doc = Document::new();
    let side = doc.add("Panel", Some(PointF::new(20.0, 20.0))).unwrap();
    let footer = doc.add("Panel", Some(PointF::new(300.0, 300.0))).unwrap();
    let other = doc.add("Button", None).unwrap();
    doc.ui.set_parent(other, Some(footer)).unwrap();
    let button = doc.add("Button", None).unwrap();
    doc.ui.set_parent(button, Some(side)).unwrap();
    let names: Vec<String> = [side, footer, button, other].iter().map(|&c| doc.ui.name(c).unwrap().to_string()).collect();
    let (side_name, footer_name, button_name) = (&names[0], &names[1], &names[2]);

    doc.enter_view(Some("phone"), 390.0);
    doc.begin_change();
    let (button, footer) = (doc.ui.find(button_name).unwrap(), doc.ui.find(footer_name).unwrap());
    doc.ui.set_parent(button, Some(footer)).unwrap();
    doc.ui.set_child_index(button, 0).unwrap();
    doc.ui.layout();
    doc.sync_view();
    let json = saved(&doc);
    let phone = view(&json, "phone");
    assert_eq!(phone["changes"][button_name]["parent"], footer_name.as_str(), "{phone}");
    assert_eq!(phone["changes"][button_name]["index"], 0, "{phone}");
    assert!(phone["changes"].get(&names[3]).is_none(), "o irmão que só andou uma casa não entra: {phone}");
    assert_eq!(child(child(&json["root"], side_name), button_name)["class"], "Button", "no base, no painel lateral");
    assert_eq!(child(&json["root"], footer_name)["children"].as_array().unwrap().len(), 1);

    // Fora e de volta: o lugar de cada um.
    doc.enter_view(None, 0.0);
    let (button, side, footer) = (doc.ui.find(button_name).unwrap(), doc.ui.find(side_name).unwrap(), doc.ui.find(footer_name).unwrap());
    assert_eq!(doc.ui.parent(button), Some(side));
    doc.enter_view(Some("phone"), 390.0);
    assert_eq!(doc.ui.parent(button), Some(footer));
    assert_eq!(doc.ui.children(footer)[0], button);

    // Voltar ao base, só na vista: o botão volta ao painel e a vista fica sem ele.
    assert!(doc.view_back_to_base(button));
    assert_eq!(doc.ui.parent(button), Some(side));
    let json = saved(&doc);
    assert!(view(&json, "phone")["changes"].get(button_name).is_none(), "{json}");
}

/// Vistas novas com condições e escalas, renomear a que está em edição e apagar outra; o
/// Ctrl+Z desfaz.
#[test]
fn views_are_created_renamed_and_removed() {
    let mut doc = Document::new();
    let label = doc.add("Label", Some(PointF::new(40.0, 40.0))).unwrap();
    let label_name = doc.ui.name(label).unwrap().to_string();
    doc.enter_view(Some("phone"), 390.0);
    doc.begin_change();
    doc.ui.set_prop(label, "visible", false).unwrap();
    doc.sync_view();

    let mut portrait = FormView::new("retrato", 0.0);
    portrait.orientation = ViewOrientation::Portrait;
    portrait.font_scale = 1.25;
    portrait.platforms = vec!["android".into(), "ios".into()];
    let phone = doc.views().into_iter().find(|v| v.name == "phone").unwrap();
    let mut renamed = phone.clone();
    renamed.name = "celular".into();
    renamed.max_width = 480.0;
    doc.set_views(vec![(Some("phone".into()), renamed), (None, portrait)]).unwrap();

    assert_eq!(doc.view_mode.as_ref().map(|m| m.name.as_str()), Some("celular"), "continua na vista, com o nome novo");
    let json = saved(&doc);
    let celular = view(&json, "celular");
    assert_eq!(celular["max_width"], 480.0);
    assert_eq!(celular["changes"][&label_name]["visible"], false, "as mudanças vieram junto");
    let retrato = view(&json, "retrato");
    assert_eq!(retrato["orientation"], "portrait");
    assert_eq!(retrato["font_scale"], 1.25);
    assert_eq!(retrato["platforms"], serde_json::json!(["android", "ios"]));
    assert!(json["root"]["views"].as_array().unwrap().iter().all(|v| v["name"] != "phone"));

    // Nomes repetidos ou vazios: recusa, sem mudar nada.
    assert!(doc.set_views(vec![(None, FormView::new("x", 0.0)), (None, FormView::new("x", 10.0))]).is_err());
    assert!(doc.set_views(vec![(None, FormView::new(" ", 0.0))]).is_err());
    assert_eq!(doc.views().len(), 2);

    // Apagar a vista em edição: volta ao base.
    doc.set_views(vec![(Some("retrato".into()), doc.views().into_iter().find(|v| v.name == "retrato").unwrap())]).unwrap();
    assert!(doc.view_mode.is_none());
    assert_eq!(doc.views().len(), 1);
    assert!(doc.undo(), "Ctrl+Z traz a vista apagada");
    assert_eq!(doc.views().len(), 2);
}

/// A prévia de uma vista no designer fica dentro das condições dela.
#[test]
fn the_preview_size_fits_the_view() {
    let doc = Document::new();
    assert_eq!(doc.preview_size(&FormView::new("phone", 599.0)).0, 390.0);
    assert_eq!(doc.preview_size(&FormView::new("tablet", 1023.0)).0, 820.0);
    assert_eq!(doc.preview_size(&FormView::new("estreita", 700.0)).0, 700.0);
    let mut portrait = FormView::new("retrato", 900.0);
    portrait.orientation = ViewOrientation::Portrait;
    let (w, h) = doc.preview_size(&portrait);
    assert!(h.is_some_and(|h| h > w), "{w} × {h:?}");
}

/// Com `AYLIS_SHOTS=pasta`, o diálogo vira um PNG para conferir.
fn shot(ui: &mut aylis_core::Ui, form: aylis_core::ComponentId, name: &str) {
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
    std::fs::write(std::path::PathBuf::from(dir).join(name), png).unwrap();
}

fn descendants(ui: &aylis_core::Ui, id: aylis_core::ComponentId) -> Vec<aylis_core::ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

fn text_of(ui: &aylis_core::Ui, id: aylis_core::ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

/// O editor "Vistas…": cria uma vista com condições e escalas, recusa um valor inválido
/// (o diálogo fica aberto com o erro), e apaga a vista; o seletor da barra acompanha.
#[test]
fn the_views_dialog_creates_validates_and_removes() {
    use aylis_controls::{Button, CheckBox, ComboBox, Edit};
    use aylis_core::ModalResult;
    let state = aylis_ide::state::shared(Document::new());
    let mut ide = aylis_ide::app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    let ids = *ide.ids;

    aylis_ide::viewsdialog::open(&mut ide.ui, &state, &ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    assert!(ide.ui.is_modal(dialog));
    ide.ui.layout();
    let find = |ui: &aylis_core::Ui, text: &str| descendants(ui, dialog).into_iter().find(|&c| ui.get::<Button>(c).is_some() && text_of(ui, c) == text);
    assert!(find(&ide.ui, "✕").is_none(), "sem vistas, nenhuma linha");
    let add = find(&ide.ui, "+ Vista").expect("+ Vista");
    ide.ui.click(add);
    ide.ui.layout();

    let all = descendants(&ide.ui, dialog);
    let edits: Vec<_> = all.iter().copied().filter(|&c| ide.ui.get::<Edit>(c).is_some()).collect();
    assert_eq!(edits.len(), 7, "nome, largura de/até, altura de/até, fonte, espaços");
    assert_eq!(ide.ui.get::<Edit>(edits[0]).unwrap().text, "vista1", "um nome livre");
    let set = |ui: &mut aylis_core::Ui, edit, value: &str| ui.get_mut::<Edit>(edit).unwrap().text = value.into();
    set(&mut ide.ui, edits[0], "estreita");
    set(&mut ide.ui, edits[2], "700");
    set(&mut ide.ui, edits[5], "1,5");
    set(&mut ide.ui, edits[6], "9");
    let combo = all.iter().copied().find(|&c| ide.ui.get::<ComboBox>(c).is_some()).unwrap();
    let _ = ide.ui.set_prop(combo, "item_index", 1_i64);
    let checks: Vec<_> = all.iter().copied().filter(|&c| ide.ui.get::<CheckBox>(c).is_some()).collect();
    assert_eq!(checks.len(), 6);
    ide.ui.get_mut::<CheckBox>(checks[3]).unwrap().checked = true;
    shot(&mut ide.ui, dialog, "vistas.png");

    // A escala 9 não vale: o diálogo continua aberto, com o erro no rodapé.
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert!(ide.ui.contains(dialog), "recusou");
    let error = descendants(&ide.ui, dialog).into_iter().map(|c| text_of(&ide.ui, c)).find(|t| t.starts_with('⚠')).unwrap_or_default();
    assert!(error.contains("estreita") && error.contains("0,25"), "{error}");
    assert!(state.borrow().doc.views().is_empty(), "nada mudou");

    set(&mut ide.ui, edits[6], "");
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert!(!ide.ui.contains(dialog));
    let views = state.borrow().doc.views();
    assert_eq!(views.len(), 1);
    let v = &views[0];
    assert_eq!((v.name.as_str(), v.max_width, v.font_scale, v.spacing_scale), ("estreita", 700.0, 1.5, 1.0));
    assert_eq!(v.orientation, ViewOrientation::Portrait);
    assert_eq!(v.platforms, ["android"]);
    let items = ide.ui.get::<ComboBox>(ids.view_combo).unwrap().items.clone();
    assert!(items.contains("estreita") && items.contains("Editar vistas…") && items.contains("Telas…"), "{items}");

    // De novo: a vista aparece; ✕ e OK apagam.
    aylis_ide::viewsdialog::open(&mut ide.ui, &state, &ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    ide.ui.layout();
    let name = descendants(&ide.ui, dialog).into_iter().find(|&c| ide.ui.get::<Edit>(c).is_some()).unwrap();
    assert_eq!(ide.ui.get::<Edit>(name).unwrap().text, "estreita");
    let remove = descendants(&ide.ui, dialog).into_iter().find(|&c| ide.ui.get::<Button>(c).is_some() && text_of(&ide.ui, c) == "✕").unwrap();
    ide.ui.click(remove);
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert!(state.borrow().doc.views().is_empty());
}
