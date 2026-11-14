//! O Object Inspector: propriedades e eventos do selecionado (ou do que os selecionados
//! têm em comum), com um editor para cada tipo.

use std::path::Path;

use aylis_controls::{AngleDial, Button, CheckBox, ComboBox, Edit, Layout, MenuItem, PopupMenu, Rectangle};
use aylis_core::{Align, Color, ComponentId, Easing, EventArgs, Key, PointF, PropKind, PropValue, Ui};
use serde_json::Value;

use crate::actions;
use crate::app::{Ids, refresh};
use crate::dialogs;
use crate::propeditors::{self, Timing};
use crate::state::Shared;
use crate::widgets::{caption, clear, label, muted};

const PROP_CATEGORIES: [&str; 12] = ["Identidade", "Layout", "Appearance", "Font", "Text", "Caret", "Keyboard", "Behavior", "Data", "Lookup", "Animation", "Misc"];

/// Texto mostrado no editor para um valor JSON.
fn display(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Array(items) => items.iter().map(display).collect::<Vec<_>>().join(", "),
        other => other.to_string(),
    }
}

/// Texto digitado → valor JSON, conforme o tipo da propriedade.
pub fn parse(kind: PropKind, text: &str) -> Result<Value, String> {
    let t = text.trim();
    let number = |s: &str| -> Result<Value, String> {
        let normalized = if s.contains('.') { s.to_string() } else { s.replace(',', ".") };
        normalized.parse::<f64>().map(Value::from).map_err(|_| format!("'{s}' não é um número"))
    };
    Ok(match kind {
        _ if t.is_empty() && !matches!(kind, PropKind::String) => Value::Null,
        PropKind::Int => Value::from(t.parse::<i64>().map_err(|_| format!("'{t}' não é um número inteiro"))?),
        PropKind::Float => number(t)?,
        PropKind::Angle | PropKind::Duration => {
            Value::from(propeditors::parse_number(t).ok_or_else(|| format!("'{t}' não é um número"))?)
        }
        PropKind::Bounds => {
            let parts: Vec<&str> = t.split([',', ';', ' ']).filter(|p| !p.is_empty()).collect();
            match parts.len() {
                1 => number(parts[0])?,
                2 | 4 => Value::Array(parts.into_iter().map(number).collect::<Result<_, _>>()?),
                _ => return Err("use 1, 2 (h, v) ou 4 (esq, cima, dir, baixo) números".into()),
            }
        }
        PropKind::Bool => Value::Bool(matches!(t.to_lowercase().as_str(), "true" | "sim" | "1")),
        _ => Value::String(text.to_string()),
    })
}

/// Grava a propriedade em todos os alvos.
fn commit(ui: &mut Ui, state: &Shared, ids: &Ids, targets: &[ComponentId], name: &str, value: Value) {
    // Renomear (o componente, o form ou o handler de um evento): o resto do projeto vem
    // depois, com a prévia (`crate::rename`).
    let rename = {
        let st = state.borrow();
        let target = targets[0];
        let new = value.as_str().unwrap_or_default().to_string();
        if name == "name" {
            let kind = if target == st.doc.root { crate::rename::Renamed::Form } else { crate::rename::Renamed::Component };
            Some((kind, st.doc.ui.name(target).unwrap_or_default().to_string(), new))
        } else {
            None
        }
    };
    // Com o servidor de linguagem ainda carregando, o código ficaria para trás: o nome não muda,
    // e a mensagem diz por quê.
    if let Some((kind, old, new)) = &rename {
        if old != new {
            if let Some(message) = crate::rename::refused_while_loading(state, *kind, old) {
                refuse_rename(ui, state, ids, &message);
                return;
            }
        }
    }
    {
        let mut st = state.borrow_mut();
        let result = if name == "name" {
            match value.as_str() {
                Some(n) => st.doc.rename_component(targets[0], n),
                None => Err("informe um nome".into()),
            }
        } else {
            st.doc.set_prop_many(targets, name, &value)
        };
        st.status = match result {
            Ok(()) => String::new(),
            Err(e) => format!("⚠ {name}: {e}"),
        };
    }
    refresh(ui, state, ids);
    if let Some((kind, old, new)) = rename.filter(|_| state.borrow().status.is_empty()) {
        crate::rename::after_rename(ui, state, ids, kind, &old, &new);
    }
}

/// O renomear recusado: a caixa de erro com o motivo, e o inspector volta ao nome de antes.
fn refuse_rename(ui: &mut Ui, state: &Shared, ids: &Ids, message: &str) {
    let text = ui.tr(message);
    aylis_controls::message_dlg(ui, &text, aylis_controls::DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
    state.borrow_mut().inspector_key = 0;
    refresh(ui, state, ids);
}

/// Grava várias propriedades de uma vez (as que mudaram) e refaz o painel uma vez só.
fn commit_many(ui: &mut Ui, state: &Shared, ids: &Ids, targets: &[ComponentId], values: Vec<(String, Value)>) {
    {
        let mut st = state.borrow_mut();
        let current = st.doc.props_json(targets[0]);
        let same = |a: Option<&Value>, b: &Value| match (a.and_then(Value::as_f64), b.as_f64()) {
            (Some(x), Some(y)) => (x - y).abs() < 1e-9,
            _ => a == Some(b),
        };
        let mut errors = Vec::new();
        for (name, value) in values {
            if !same(current.get(&name), &value) {
                if let Err(e) = st.doc.set_prop_many(targets, &name, &value) {
                    errors.push(format!("⚠ {name}: {e}"));
                }
            }
        }
        st.status = errors.join("  ");
    }
    refresh(ui, state, ids);
}

/// Os tempos para o editor da linha do tempo. Numa animação (`duration` junto de `delay`,
/// `easing`, `auto_reverse`, `looping`), edita todos; senão, só a propriedade.
fn timing_of(state: &Shared, target: ComponentId, name: &str, value: &Value) -> (Timing, bool) {
    let st = state.borrow();
    let doc = &st.doc;
    let kind_of = |n: &str| doc.ui.prop_info(target, n).map(|i| i.kind);
    let group = matches!(name, "duration" | "delay") && kind_of("duration") == Some(PropKind::Duration);
    if !group {
        return (Timing { duration: value.as_f64().unwrap_or(0.0), ..Default::default() }, false);
    }
    let values = doc.props_json(target);
    let num = |n: &str| values.get(n).and_then(Value::as_f64);
    let flag = |n: &str| (kind_of(n) == Some(PropKind::Bool)).then(|| values.get(n).and_then(Value::as_bool).unwrap_or(false));
    let timing = Timing {
        duration: num("duration").unwrap_or(0.0),
        delay: (kind_of("delay") == Some(PropKind::Duration)).then(|| num("delay").unwrap_or(0.0)),
        easing: (kind_of("easing") == Some(PropKind::Easing)).then(|| values.get("easing").map(display).unwrap_or_else(|| "linear".into())),
        auto_reverse: flag("auto_reverse"),
        looping: flag("looping"),
    };
    (timing, true)
}

/// O recuo das propriedades dentro de um grupo: o nome do grupo fica na borda do painel (como
/// a busca e o título do componente) e as propriedades dele, à frente (pedido do autor em 09/11).
const GROUP_INDENT: f32 = 16.0;

/// O recuo de uma parte de propriedade (os lados de `margins`), à frente da propriedade.
const PART_INDENT: f32 = GROUP_INDENT + 14.0;

/// Uma linha: rótulo + área do editor.
fn line(ui: &mut Ui, parent: ComponentId, text: &str, indent: f32) -> ComponentId {
    let line = ui.add(Layout::default(), parent);
    ui.control_mut(line).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 3.0]);
    let l = label(ui, line, text, Align::Left, 150.0 - indent);
    ui.control_mut(l).unwrap().set_margins([indent, 0.0, 0.0, 0.0]);
    muted(ui, l);
    let _ = ui.set_prop(l, "font_size", 12.5);
    line
}

/// As propriedades expostas de um frame de pacote (`form = "pkg:Nome"`), cada uma com o
/// editor do tipo dela; vazio volta ao padrão do pacote.
fn package_rows(ui: &mut Ui, state: &Shared, ids: &Ids, frame: ComponentId) {
    let (manifest, values) = {
        let st = state.borrow();
        let form = dialogs::prop_text(&st.doc.ui, frame, "form");
        let Some(package) = aylis_project::packages::package_of(&form) else { return };
        let Some(project) = st.project.as_ref() else { return };
        let Some(m) = aylis_project::packages::manifest(&aylis_project::Files::Dir(project.dir()), package) else { return };
        (m, dialogs::prop_text(&st.doc.ui, frame, "values"))
    };
    caption(ui, ids.inspector, &format!("{} {}", manifest.name, manifest.version));
    for p in &manifest.properties {
        let l = line(ui, ids.inspector, &p.name, 4.0);
        if let Some(&label) = ui.children(l).first() {
            let hint = if p.description.is_empty() { p.target.clone() } else { format!("{} ({})", p.description, p.target) };
            let _ = ui.set_prop(label, "hint", hint);
        }
        let current = aylis_project::packages::value_of(&values, &p.name);
        let save = {
            let (state, ids, values, name) = (state.clone(), *ids, values.clone(), p.name.clone());
            move |ui: &mut Ui, value: Value| {
                let new = aylis_project::packages::with_value(&values, &name, value);
                {
                    let mut st = state.borrow_mut();
                    let result = st.doc.set_prop(frame, "values", &Value::String(new));
                    st.status = result.err().map(|e| format!("⚠ {name}: {e}")).unwrap_or_default();
                    st.doc.refresh_frames();
                }
                refresh(ui, &state, &ids);
            }
        };
        if p.kind == "bool" {
            let c = ui.add(CheckBox::new(""), l);
            ui.control_mut(c).unwrap().set_align(Align::Client);
            let on = current.as_ref().or(Some(&p.default)).and_then(Value::as_bool).unwrap_or(false);
            let _ = ui.set_prop(c, "checked", on);
            ui.on(c, "on_change", move |ui, _, args| save(ui, Value::Bool(matches!(args, EventArgs::Value(PropValue::Bool(true))))));
            continue;
        }
        let shown = current.as_ref().map(|v| v.as_str().map_or_else(|| v.to_string(), str::to_string)).unwrap_or_default();
        let number = p.kind == "number";
        let placeholder = p.default.as_str().map_or_else(|| p.default.to_string(), str::to_string);
        let e = text_editor(ui, l, shown, move |ui, text| {
            let text = text.trim().to_string();
            let value = if text.is_empty() {
                Value::Null
            } else if number {
                match text.replace(',', ".").parse::<f64>() {
                    Ok(n) => serde_json::json!(n),
                    Err(_) => return,
                }
            } else {
                Value::String(text)
            };
            save(ui, value);
        });
        let _ = ui.set_prop(e, "placeholder", placeholder);
    }
    // Os eventos do pacote: o nome do handler do form (como os eventos de qualquer
    // componente; o código dele é escrito à mão no form do usuário).
    for ev in &manifest.events {
        let l = line(ui, ids.inspector, &ev.name, 4.0);
        if let Some(&label) = ui.children(l).first() {
            let _ = ui.set_prop(label, "hint", format!("evento: {}", ev.target));
        }
        let current = aylis_project::packages::value_of(&values, &ev.name).and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
        let (state, ids, values, name) = (state.clone(), *ids, values.clone(), ev.name.clone());
        let e = text_editor(ui, l, current, move |ui, text| {
            let text = text.trim().to_string();
            if !text.is_empty() && !aylis_core::is_identifier(&text) {
                state.borrow_mut().status = format!("⚠ {name}: nome de handler inválido (letras, dígitos e _)");
                refresh(ui, &state, &ids);
                return;
            }
            let value = if text.is_empty() { Value::Null } else { Value::String(text) };
            let new = aylis_project::packages::with_value(&values, &name, value);
            {
                let mut st = state.borrow_mut();
                let result = st.doc.set_prop(frame, "values", &Value::String(new));
                st.status = result.err().map(|e| format!("⚠ {name}: {e}")).unwrap_or_default();
                st.doc.refresh_frames();
            }
            refresh(ui, &state, &ids);
        });
        let _ = ui.set_prop(e, "placeholder", "nome do handler");
    }
}

/// O menu de contexto da linha de uma propriedade: "Ligar a campo…" e, se ligada,
/// "Desligar do campo". A propriedade ligada aparece com 🔗 e a ligação na dica.
fn binding_menu(ui: &mut Ui, state: &Shared, ids: &Ids, target: ComponentId, line: ComponentId, property: &str) {
    let bound = {
        let st = state.borrow();
        st.doc.ui.binding(target, property).map(|b| {
            let source = b.source.and_then(|s| st.doc.ui.reference_name(target, s)).unwrap_or_default();
            (source, b.path.clone())
        })
    };
    if let (Some((source, path)), Some(&label)) = (&bound, ui.children(line).first()) {
        let text = format!("🔗 {}", crate::lang::property_label(property));
        let _ = ui.set_prop(label, "text", text);
        let _ = ui.set_prop(label, "hint", format!("{property}: ligado a {source} → {path}"));
    }
    let menu = ui.add(PopupMenu::default(), line);
    let link = ui.add(MenuItem::new("Ligar a campo…"), menu);
    {
        let (state, ids, property, current) = (state.clone(), *ids, property.to_string(), bound.clone());
        ui.on_click(link, move |ui, _| bind_dialog(ui, &state, &ids, target, &property, current.clone()));
    }
    if bound.is_some() {
        let unlink = ui.add(MenuItem::new("Desligar do campo"), menu);
        let (state, ids, property) = (state.clone(), *ids, property.to_string());
        ui.on_click(unlink, move |ui, _| {
            let result = state.borrow_mut().doc.set_binding(target, &property, "", "");
            state.borrow_mut().status = result.err().map(|e| format!("⚠ {e}")).unwrap_or_default();
            refresh(ui, &state, &ids);
        });
    }
    let _ = ui.set_prop(line, "popup_menu", PropValue::Component(Some(menu)));
}

/// "Ligar a campo…": o `DataSource` e depois o campo, um texto com campos
/// (`Total: {valor:moeda}`) ou uma expressão (`=quantidade * preco`).
fn bind_dialog(ui: &mut Ui, state: &Shared, ids: &Ids, target: ComponentId, property: &str, current: Option<(String, String)>) {
    let sources = state.borrow().doc.data_sources();
    if sources.is_empty() {
        state.borrow_mut().status = "⚠ Ponha um DataSource no form para ligar a um campo".into();
        refresh(ui, state, ids);
        return;
    }
    let (state, ids, property) = (state.clone(), *ids, property.to_string());
    let pick_source = move |ui: &mut Ui, source: String| {
        let fields = state.borrow().doc.fields_of(&source);
        let mut items: Vec<String> = fields.clone();
        items.push("Texto com campos…  (Total: {valor:moeda})".into());
        items.push("Expressão…  (=quantidade * preco)".into());
        let (state, property, current) = (state.clone(), property.clone(), current.clone());
        crate::codewindow::quick_pick(ui, &format!("Ligar {property} a: campo de {source}"), items, move |ui, i| {
            let save = {
                let (state, property, source) = (state.clone(), property.clone(), source.clone());
                move |ui: &mut Ui, path: String| {
                    let result = state.borrow_mut().doc.set_binding(target, &property, &source, &path);
                    state.borrow_mut().status = result.err().map(|e| format!("⚠ {e}")).unwrap_or_default();
                    refresh(ui, &state, &ids);
                }
            };
            if let Some(field) = fields.get(i) {
                save(ui, field.clone());
                return;
            }
            let expression = i > fields.len();
            let initial = current.as_ref().map(|c| c.1.clone()).filter(|p| p.starts_with('=') == expression).unwrap_or_else(|| if expression { "=".into() } else { String::new() });
            let names = fields.clone();
            let validate = move |text: &str| -> Result<(), String> {
                let text = text.trim();
                if expression {
                    let body = text.strip_prefix('=').ok_or("a expressão começa com =")?;
                    let parsed = aylis_data::expr::Expr::parse(body)?;
                    if let Some(missing) = parsed.fields().into_iter().find(|f| !names.iter().any(|n| n.eq_ignore_ascii_case(f))) {
                        return Err(format!("o campo '{missing}' não existe"));
                    }
                } else if !text.contains('{') {
                    return Err("use {campo} no texto".into());
                }
                Ok(())
            };
            let title = if expression { "Ligar a uma expressão" } else { "Ligar a um texto com campos" };
            dialogs::input_text(ui, title, if expression { "Expressão" } else { "Texto" }, &initial, validate, save);
        });
    };
    if sources.len() == 1 {
        pick_source(ui, sources[0].clone());
    } else {
        let list = sources.clone();
        crate::codewindow::quick_pick(ui, "Ligar a campo: o DataSource", sources, move |ui, i| pick_source(ui, list[i].clone()));
    }
}

fn small_button(ui: &mut Ui, parent: ComponentId, text: &str) -> ComponentId {
    let b = ui.add(Button::new(text), parent);
    ui.control_mut(b).unwrap().set_align(Align::Right).set_width(28.0).set_margins([3.0, 0.0, 0.0, 0.0]).set_padding(0.0).set_style("flat");
    b
}

/// Caixa de texto que grava ao sair ou com Enter (só se o texto mudou).
fn text_editor(
    ui: &mut Ui,
    parent: ComponentId,
    shown: String,
    on_commit: impl Fn(&mut Ui, String) + Clone + 'static,
) -> ComponentId {
    let e = ui.add(Edit::new(shown.clone()), parent);
    ui.control_mut(e).unwrap().set_align(Align::Client);
    let apply = move |ui: &mut Ui, sender: ComponentId| {
        let text = ui.get::<Edit>(sender).map(|e| e.text.clone()).unwrap_or_default();
        // Sem mudança não refaz o painel (o clique que tirou o foco segue normal).
        if text != shown {
            on_commit(ui, text);
        }
    };
    let on_enter = apply.clone();
    ui.on(e, "on_exit", move |ui, sender, _| apply(ui, sender));
    ui.on(e, "on_key_down", move |ui, sender, args| {
        if matches!(args, EventArgs::Key(k) if k.key == Key::Enter) {
            on_enter(ui, sender);
        }
    });
    e
}

/// O cabeçalho de um grupo de propriedades: o nome legível à esquerda (13 pontos, em negrito)
/// e a seta à direita, como nas categorias da paleta;
/// clicar recolhe e abre. `key` é a categoria (a marca guardada em `inspector_closed`).
fn group_header(ui: &mut Ui, state: &Shared, ids: &Ids, text: &str, key: &str, open: bool) {
    let r = crate::widgets::row(ui, ids.inspector, text, 0.0, false, 28.0);
    ui.control_mut(r).unwrap().set_margins([0.0, 6.0, 0.0, 2.0]);
    if let Some(&l) = ui.children(r).iter().find(|&&c| ui.class_name(c) == Some("Label")) {
        let _ = ui.set_prop(l, "font_size", 13);
        let _ = ui.set_prop(l, "bold", true);
        // Na borda do painel, alinhado com a busca e com o título do componente (o `row` põe 10).
        ui.control_mut(l).unwrap().set_margins([0.0, 0.0, 6.0, 0.0]);
    }
    let arrow = ui.add(aylis_controls::Label::new(if open { "▾" } else { "▸" }), r);
    ui.control_mut(arrow).unwrap().set_align(aylis_core::Align::Right).set_width(26.0).set_hit_test(false);
    let _ = ui.set_prop(arrow, "h_align", aylis_core::PropValue::Enum("center".into()));
    muted(ui, arrow);
    let (state, ids, key) = (state.clone(), *ids, key.to_string());
    ui.on_click(r, move |ui, _| {
        {
            let mut st = state.borrow_mut();
            if !st.inspector_closed.remove(&key) {
                st.inspector_closed.insert(key.clone());
            }
        }
        rebuild(ui, &state, &ids);
    });
}

thread_local! {
    /// O painel do Object Inspector (para [`commit_pending`] achar o campo em edição).
    static PANEL: std::cell::Cell<Option<ComponentId>> = const { std::cell::Cell::new(None) };
}

/// Um campo do Object Inspector ainda em edição (o foco nele) grava agora, como ao sair dele:
/// antes de salvar, executar, compilar e publicar (o Ctrl+Shift+F9 com o cursor no evento
/// apagado executava sem a mudança; conferência de 08/11).
pub fn commit_pending(ui: &mut Ui) {
    let Some(panel) = PANEL.with(std::cell::Cell::get).filter(|&p| ui.contains(p)) else { return };
    let root = ui.root_of(panel);
    let Some(focused) = ui.focused(root) else { return };
    let mut at = Some(focused);
    while let Some(c) = at {
        if c == panel {
            ui.clear_focus(root);
            return;
        }
        at = ui.parent(c);
    }
}

pub fn rebuild(ui: &mut Ui, state: &Shared, ids: &Ids) {
    PANEL.with(|p| p.set(Some(ids.inspector)));
    // Outro componente selecionado: a busca de antes não vale para ele (com "data" digitado,
    // um Field parecia sem propriedades).
    let changed = {
        let mut st = state.borrow_mut();
        let now = st.doc.selected.clone();
        let changed = st.inspector_for != now && !st.inspector_for.is_empty();
        st.inspector_for = now;
        if changed {
            st.inspector_filter.clear();
        }
        changed
    };
    if changed {
        let _ = ui.set_prop(ids.inspector_search, "text", "");
    }
    struct Row {
        name: String,
        kind: PropKind,
        category: String,
        value: Value,
    }
    let (targets, title, rows, events_tab, colors, class) = {
        let st = state.borrow();
        let doc = &st.doc;
        let targets: Vec<ComponentId> = if st.no_form { Vec::new() } else { doc.selected.clone() };
        if targets.is_empty() {
            drop(st);
            clear(ui, ids.inspector);
            state.borrow_mut().inspector_key = 0;
            let _ = ui.set_prop(ids.inspector_title, "text", "");
            return;
        }
        let first = targets[targets.len() - 1];
        let class = doc.ui.class_name(first).unwrap_or_default().to_string();
        let title = if targets.len() == 1 {
            format!("{}: {class}", doc.ui.name(first).unwrap_or_default())
        } else {
            format!("{} componentes selecionados", targets.len())
        };
        let filter = st.inspector_filter.to_lowercase();
        let values = doc.props_json(first);
        // Os valores de cada selecionado, uma vez (e não a cada propriedade).
        let all_values: Vec<_> = targets.iter().map(|&t| doc.props_json(t)).collect();
        // Propriedades que todos os selecionados têm (mesmo nome e tipo).
        let mut rows: Vec<Row> = doc
            .ui
            .prop_infos(first)
            .into_iter()
            .filter(|i| (i.kind == PropKind::Event) == st.events_tab && !i.read_only && !i.runtime)
            // A busca acha pelo nome técnico e pelo legível (em português e no idioma da IDE).
            .filter(|i| {
                let label = crate::lang::property_label(i.name);
                filter.is_empty() || i.name.contains(&filter) || label.to_lowercase().contains(&filter) || ui.tr(&label).to_lowercase().contains(&filter)
            })
            .filter(|i| targets.iter().all(|&t| doc.ui.prop_info(t, i.name).is_some_and(|o| o.kind == i.kind)))
            .map(|i| {
                // Valor em branco quando os selecionados têm valores diferentes.
                let value = values.get(i.name).cloned().unwrap_or(Value::Null);
                let same = all_values.iter().all(|v| v.get(i.name) == Some(&value));
                Row { name: i.name.to_string(), kind: i.kind, category: i.category.to_string(), value: if same { value } else { Value::Null } }
            })
            .collect();
        let order = |c: &str| PROP_CATEGORIES.iter().position(|p| *p == c).unwrap_or(PROP_CATEGORIES.len());
        // Na ordem dos nomes legíveis, no idioma da IDE (os eventos, pelo nome).
        let shown = |r: &Row| if st.events_tab { r.name.clone() } else { ui.tr(&crate::lang::property_label(&r.name)).to_lowercase() };
        rows.sort_by_cached_key(|r| (order(&r.category), shown(r)));
        if !st.events_tab && targets.len() == 1 && (filter.is_empty() || "name".contains(&filter)) {
            let name = doc.ui.name(first).unwrap_or_default().to_string();
            rows.insert(0, Row { name: "name".into(), kind: PropKind::String, category: "Identidade".into(), value: Value::String(name) });
        }
        let colors: Vec<(String, Color)> = {
            let mut c: Vec<(String, Color)> = doc.ui.theme().colors.iter().map(|(k, v)| (k.clone(), *v)).collect();
            c.sort_by(|a, b| a.0.cmp(&b.0));
            c
        };
        (targets, title, rows, st.events_tab, colors, class)
    };
    // Nada mudou desde a última vez (o mesmo componente, os mesmos valores, a mesma aba e
    // busca): o painel fica como está. Refazer custava a maior parte de cada refresh da IDE.
    let key = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (&targets, &title, events_tab, &class).hash(&mut h);
        for r in &rows {
            (&r.name, &r.category, r.value.to_string()).hash(&mut h);
        }
        for (name, color) in &colors {
            (name, color.to_hex()).hash(&mut h);
        }
        let st = state.borrow();
        (&st.inspector_filter, st.doc.handler_names()).hash(&mut h);
        // Os grupos recolhidos (recolher ou abrir refaz a lista).
        let mut closed: Vec<&String> = st.inspector_closed.iter().collect();
        closed.sort();
        closed.hash(&mut h);
        // As ligações a dados (o 🔗 e o menu).
        for &t in &targets {
            for b in st.doc.ui.bindings(t) {
                (&b.property, &b.path, b.source).hash(&mut h);
            }
        }
        // Os componentes do form (as listas das propriedades que apontam para outro).
        for (id, _) in st.doc.tree() {
            (st.doc.ui.name(id), st.doc.ui.class_name(id)).hash(&mut h);
        }
        h.finish()
    };
    if state.borrow().inspector_key == key && !ui.children(ids.inspector).is_empty() {
        return;
    }
    state.borrow_mut().inspector_key = key;
    clear(ui, ids.inspector);
    let _ = ui.set_prop(ids.inspector_title, "text", title);

    // Editando numa vista: o aviso e, para o componente, a volta ao base.
    let view = state.borrow().doc.view_mode.as_ref().map(|m| m.name.clone());
    if let Some(view) = view {
        let max = state.borrow().doc.ui.views(state.borrow().doc.root).iter().find(|v| v.name == view).map_or(0.0, |v| v.max_width);
        let label = crate::lang::view_label(&view);
        let limit = if max > 0.0 { format!(" ({} {max:.0})", ui.tr("até")) } else { String::new() };
        caption(ui, ids.inspector, &format!("{} {label}{limit}: {}", ui.tr("Vista"), ui.tr("o que mudar vale só nela")));
        if targets.len() == 1 && targets[0] != state.borrow().doc.root && !events_tab {
            let b = ui.add(Button::new("Voltar ao base"), ids.inspector);
            ui.control_mut(b).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 8.0]);
            let (state, ids, id) = (state.clone(), *ids, targets[0]);
            ui.on_click(b, move |ui, _| {
                if state.borrow_mut().doc.view_back_to_base(id) {
                    crate::app::refresh(ui, &state, &ids);
                }
            });
        }
    }
    // Editor de campos (ClientDataSet).
    if targets.len() == 1 && class == "ClientDataSet" && !events_tab {
        let b = ui.add(Button::new("Editor de campos…"), ids.inspector);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 8.0]);
        let (state, ids, cds) = (state.clone(), *ids, targets[0]);
        ui.on_click(b, move |ui, _| crate::fieldsview::open(ui, &state, &ids, cds));
    }
    // Editor de colunas (DataGrid).
    if targets.len() == 1 && class == "DataGrid" && !events_tab {
        let b = ui.add(Button::new("Editor de colunas…"), ids.inspector);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 8.0]);
        let (state, ids, grid) = (state.clone(), *ids, targets[0]);
        ui.on_click(b, move |ui, _| crate::columnsview::open(ui, &state, &ids, grid));
    }

    // Frame de pacote: as propriedades que o pacote expõe (gravadas no `values` do frame).
    if targets.len() == 1 && class == "Frame" && !events_tab {
        package_rows(ui, state, ids, targets[0]);
    }

    // Os grupos (Identidade, Posição e tamanho...) recolhem e abrem como as categorias da
    // paleta (pedido do autor em 07/11); com busca, ficam abertos.
    let searching = !state.borrow().inspector_filter.is_empty();
    let mut category = String::new();
    let mut open = true;
    for row in rows {
        if row.category != category && !events_tab {
            category = row.category.clone();
            open = searching || !state.borrow().inspector_closed.contains(&category);
            group_header(ui, state, ids, &crate::lang::category_label(&category), &category, open);
        }
        if !open {
            continue;
        }
        let Row { name, kind, value, .. } = row;
        // O nome legível (traduzido com a tela); na dica, o nome usado no código.
        let l = if events_tab { line(ui, ids.inspector, &name, 4.0) } else { line(ui, ids.inspector, &crate::lang::property_label(&name), GROUP_INDENT) };
        if !events_tab {
            if let Some(&label) = ui.children(l).first() {
                let _ = ui.set_prop(label, "hint", name.clone());
            }
        }
        // Botão direito: ligar a propriedade a um campo.
        if !events_tab && targets.len() == 1 && !matches!(kind, PropKind::Component | PropKind::Event) && name != "name" {
            binding_menu(ui, state, ids, targets[0], l, &name);
        }
        let (state2, ids2, targets2) = (state.clone(), *ids, targets.clone());
        let commit_to = move |ui: &mut Ui, name: &str, value: Value| commit(ui, &state2, &ids2, &targets2, name, value);
        let err_to = {
            let (state, ids) = (state.clone(), *ids);
            move |ui: &mut Ui, name: &str, error: &str| {
                state.borrow_mut().status = format!("⚠ {name}: {error}");
                refresh(ui, &state, &ids);
            }
        };
        match kind {
            PropKind::Bool => {
                let c = ui.add(CheckBox::new(""), l);
                ui.control_mut(c).unwrap().set_align(Align::Client);
                let _ = ui.set_prop(c, "checked", value.as_bool().unwrap_or(false));
                ui.on(c, "on_change", move |ui, _, args| {
                    let on = matches!(args, EventArgs::Value(PropValue::Bool(true)));
                    commit_to(ui, &name, Value::Bool(on));
                });
            }
            PropKind::Enum(names) => {
                let c = ui.add(ComboBox::new(names), l);
                ui.control_mut(c).unwrap().set_align(Align::Client);
                if let Some(i) = value.as_str().and_then(|v| names.iter().position(|n| *n == v)) {
                    let _ = ui.set_prop(c, "item_index", i as i64);
                }
                ui.on(c, "on_change", move |ui, _, args| {
                    if let EventArgs::Text(text) = args {
                        commit_to(ui, &name, Value::String(text.clone()));
                    }
                });
            }
            PropKind::Component => {
                // Lista com os componentes do form (e "(nenhum)").
                let candidates = state.borrow().doc.reference_candidates_for(targets[0], &name);
                let mut items = vec!["(nenhum)".to_string()];
                items.extend(candidates);
                let refs: Vec<&str> = items.iter().map(String::as_str).collect();
                let c = ui.add(ComboBox::new(&refs), l);
                ui.control_mut(c).unwrap().set_align(Align::Client);
                let current = display(&value);
                let index = items.iter().position(|i| *i == current).unwrap_or(0);
                let _ = ui.set_prop(c, "item_index", index as i64);
                ui.on(c, "on_change", move |ui, _, args| {
                    if let EventArgs::Text(text) = args {
                        let v = if text == "(nenhum)" { Value::Null } else { Value::String(text.clone()) };
                        commit_to(ui, &name, v);
                    }
                });
            }
            PropKind::Event => {
                // → abre o código (vazio: cria o handler padrão antes);
                // ▾ escolhe um handler existente ou cria o padrão.
                let go = small_button(ui, l, "→");
                let pick = small_button(ui, l, "▾");
                let shown = display(&value);
                let (existing, suggested) = {
                    let st = state.borrow();
                    let component = st.doc.ui.name(targets[0]).unwrap_or("componente").to_string();
                    (st.doc.handler_names(), aylis_project::codegen::default_handler_name(&component, &name))
                };
                {
                    let (state, commit_to, name, suggested) = (state.clone(), commit_to.clone(), name.clone(), suggested.clone());
                    let handler = shown.clone();
                    ui.on_click(go, move |ui, _| {
                        let handler = if handler.is_empty() {
                            commit_to(ui, &name, Value::String(suggested.clone()));
                            suggested.clone()
                        } else {
                            handler.clone()
                        };
                        actions::open_code(ui, &state, Some(&handler));
                    });
                }
                let menu = ui.add(PopupMenu::default(), l);
                let mut choices = vec![format!("Novo: {suggested}")];
                choices.extend(existing.into_iter().filter(|h| *h != suggested));
                choices.push("(nenhum)".into());
                for choice in choices {
                    let item = ui.add(MenuItem::new(choice.clone()), menu);
                    let (commit_to, name, suggested) = (commit_to.clone(), name.clone(), suggested.clone());
                    ui.on_click(item, move |ui, _| {
                        let v = if choice.starts_with("Novo: ") {
                            Value::String(suggested.clone())
                        } else if choice == "(nenhum)" {
                            Value::Null
                        } else {
                            Value::String(choice.clone())
                        };
                        commit_to(ui, &name, v);
                    });
                }
                ui.on_click(pick, move |ui, sender| {
                    let at = ui.absolute_rect(sender).map_or(PointF::ZERO, |r| PointF::new(r.x, r.bottom()));
                    PopupMenu::popup(ui, menu, at);
                });
                let (commit_to, name, state, ids) = (commit_to.clone(), name.clone(), state.clone(), *ids);
                text_editor(ui, l, shown.clone(), move |ui, text| {
                    let new = text.trim().to_string();
                    let v = if new.is_empty() { Value::Null } else { Value::String(new.clone()) };
                    // Digitar outro nome no lugar de um handler que existe no código, com o
                    // novo ainda sem método: é renomear (o método e os outros eventos dele).
                    let renamed = state.borrow().project.as_ref().is_some_and(|p| crate::rename::handler_renamed(p, &shown, &new));
                    if renamed {
                        if let Some(message) = crate::rename::refused_while_loading(&state, crate::rename::Renamed::Handler, &shown) {
                            refuse_rename(ui, &state, &ids, &message);
                            return;
                        }
                    }
                    commit_to(ui, &name, v);
                    if renamed {
                        crate::rename::after_rename(ui, &state, &ids, crate::rename::Renamed::Handler, &shown, &new);
                    }
                });
            }
            PropKind::Color => {
                let more = small_button(ui, l, "…");
                let swatch = ui.add(Rectangle::new(22.0, 0.0), l);
                ui.control_mut(swatch).unwrap().set_align(Align::Right).set_margins([4.0, 4.0, 0.0, 4.0]);
                let text = display(&value);
                let named: std::collections::HashMap<String, Color> = colors.iter().cloned().collect();
                let color = aylis_core::palette::resolve_in(&named, &text).unwrap_or(Color::TRANSPARENT);
                let _ = ui.set_prop(swatch, "fill", color);
                let _ = ui.set_prop(swatch, "radius", 4);
                let border = ui.theme().color("border");
                let _ = ui.set_prop(swatch, "stroke", border);
                let _ = ui.set_prop(swatch, "stroke_width", 1);
                {
                    let (commit_to, name, text, colors) = (commit_to.clone(), name.clone(), text.clone(), colors.clone());
                    ui.on_click(more, move |ui, _| {
                        let (commit_to, name) = (commit_to.clone(), name.clone());
                        dialogs::color_picker(ui, &text, colors.clone(), move |ui, value| {
                            commit_to(ui, &name, if value.is_empty() { Value::Null } else { Value::String(value) });
                        });
                    });
                }
                text_editor(ui, l, text, move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
            PropKind::Angle => {
                // O relógio na própria linha (arrastar grava ao soltar) e o editor grande.
                let more = small_button(ui, l, "…");
                let current = value.as_f64().unwrap_or(0.0);
                let dial = ui.add(AngleDial::new(current as f32), l);
                ui.control_mut(dial).unwrap().set_align(Align::Right).set_width(26.0).set_margins([4.0, 2.0, 0.0, 2.0]);
                let _ = ui.set_prop(dial, "show_value", false);
                {
                    let (commit_to, name) = (commit_to.clone(), name.clone());
                    ui.on(dial, "on_commit", move |ui, _, args| {
                        if let EventArgs::Value(v) = args {
                            commit_to(ui, &name, Value::from(v.to_f64().unwrap_or(0.0)));
                        }
                    });
                }
                {
                    let (commit_to, name) = (commit_to.clone(), name.clone());
                    ui.on_click(more, move |ui, _| {
                        let (commit_to, key) = (commit_to.clone(), name.clone());
                        propeditors::angle_editor(ui, &format!("Ângulo: {name}"), current, move |ui, v| commit_to(ui, &key, Value::from(v)));
                    });
                }
                text_editor(ui, l, display(&value), move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
            PropKind::Shadow => {
                // Uma amostra com a própria sombra e o editor de sombra.
                let more = small_button(ui, l, "…");
                let text = display(&value);
                let sample = ui.add(Rectangle::new(20.0, 0.0), l);
                ui.control_mut(sample).unwrap().set_align(Align::Right).set_margins([6.0, 7.0, 4.0, 7.0]);
                let surface = ui.theme().color("surface");
                let border = ui.theme().color("border");
                let _ = ui.set_prop(sample, "fill", surface);
                let _ = ui.set_prop(sample, "stroke", border);
                let _ = ui.set_prop(sample, "radius", 3);
                let _ = ui.set_prop(sample, "shadow", text.clone());
                {
                    let (commit_to, name, text, colors) = (commit_to.clone(), name.clone(), text.clone(), colors.clone());
                    ui.on_click(more, move |ui, _| {
                        let (commit_to, name) = (commit_to.clone(), name.clone());
                        propeditors::shadow_editor(ui, &text, colors.clone(), move |ui, v| {
                            commit_to(ui, &name, if v.is_empty() { Value::Null } else { Value::String(v) });
                        });
                    });
                }
                text_editor(ui, l, text, move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
            PropKind::Easing => {
                // A miniatura da curva e o editor da curva (os dois abrem o editor).
                let more = small_button(ui, l, "…");
                let text = display(&value);
                let thumb = propeditors::easing_thumb(ui, l, Easing::parse(&text).unwrap_or_default(), Align::Right, 40.0);
                for opener in [more, thumb] {
                    let (commit_to, name, text) = (commit_to.clone(), name.clone(), text.clone());
                    ui.on_click(opener, move |ui, _| {
                        let (commit_to, name) = (commit_to.clone(), name.clone());
                        propeditors::easing_editor(ui, &text, move |ui, v| commit_to(ui, &name, Value::String(v)));
                    });
                }
                text_editor(ui, l, text, move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
            PropKind::Duration => {
                // A linha do tempo (numa animação, com atraso, curva, ida e volta e repetir).
                let more = small_button(ui, l, "…");
                {
                    let (state, ids, targets, name, value) = (state.clone(), *ids, targets.clone(), name.clone(), value.clone());
                    ui.on_click(more, move |ui, _| {
                        let (timing, group) = timing_of(&state, targets[0], &name, &value);
                        let (state, targets, key) = (state.clone(), targets.clone(), name.clone());
                        let title = if group { "Tempos da animação".to_string() } else { format!("Tempo: {name}") };
                        propeditors::timing_editor(ui, &title, timing, move |ui, t| {
                            let mut values = vec![(if group { "duration".to_string() } else { key.clone() }, Value::from(t.duration))];
                            if let Some(v) = t.delay {
                                values.push(("delay".into(), Value::from(v)));
                            }
                            if let Some(v) = t.easing {
                                values.push(("easing".into(), Value::String(v)));
                            }
                            if let Some(v) = t.auto_reverse {
                                values.push(("auto_reverse".into(), Value::Bool(v)));
                            }
                            if let Some(v) = t.looping {
                                values.push(("looping".into(), Value::Bool(v)));
                            }
                            commit_many(ui, &state, &ids, &targets, values);
                        });
                    });
                }
                text_editor(ui, l, display(&value), move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
            PropKind::Bounds => {
                let expanded = state.borrow().expanded.contains(&name);
                let toggle = small_button(ui, l, if expanded { "−" } else { "+" });
                {
                    let (state, ids, name) = (state.clone(), *ids, name.clone());
                    ui.on_click(toggle, move |ui, _| {
                        {
                            let mut st = state.borrow_mut();
                            if !st.expanded.remove(&name) {
                                st.expanded.insert(name.clone());
                            }
                        }
                        refresh(ui, &state, &ids);
                    });
                }
                let shown = display(&value);
                {
                    let (commit_to, name, err_to) = (commit_to.clone(), name.clone(), err_to.clone());
                    text_editor(ui, l, shown, move |ui, text| match parse(kind, &text) {
                        Ok(v) => commit_to(ui, &name, v),
                        Err(e) => err_to(ui, &name, &e),
                    });
                }
                if expanded {
                    // Os quatro lados, um por linha ("margins.left": 4).
                    let sides = bounds_sides(&value);
                    for (i, side) in ["left", "top", "right", "bottom"].into_iter().enumerate() {
                        let sl = line(ui, ids.inspector, side, PART_INDENT);
                        let (commit_to, key, err_to) = (commit_to.clone(), format!("{name}.{side}"), err_to.clone());
                        text_editor(ui, sl, format_number(sides[i]), move |ui, text| match parse(PropKind::Float, &text) {
                            Ok(v) => commit_to(ui, &key, v),
                            Err(e) => err_to(ui, &key, &e),
                        });
                    }
                }
            }
            PropKind::String if class == "Frame" && name == "form" => {
                // Os forms do projeto (menos este e os DataModules).
                let forms = {
                    let st = state.borrow();
                    let own = st.doc.form_name();
                    let forms: Vec<String> = st
                        .project
                        .as_ref()
                        .map(|p| p.forms.iter().map(|f| f.name.clone()).filter(|f| *f != own && !p.is_data_module(f)).collect())
                        .unwrap_or_default();
                    forms
                };
                let mut items = vec!["(nenhum)".to_string()];
                items.extend(forms);
                let refs: Vec<&str> = items.iter().map(String::as_str).collect();
                let c = ui.add(ComboBox::new(&refs), l);
                ui.control_mut(c).unwrap().set_align(Align::Client);
                let current = display(&value);
                let index = items.iter().position(|i| *i == current).unwrap_or(0);
                let _ = ui.set_prop(c, "item_index", index as i64);
                ui.on(c, "on_change", move |ui, _, args| {
                    if let EventArgs::Text(text) = args {
                        let v = if text == "(nenhum)" { String::new() } else { text.clone() };
                        commit_to(ui, &name, Value::String(v));
                    }
                });
            }
            _ => {
                let shown = display(&value);
                // Editores especiais por nome de propriedade.
                if name == "image" || (name == "file" && matches!(class.as_str(), "MediaPlayer" | "VideoPlayer" | "Sound")) || (name == "source" && class == "Model3D") {
                    let more = small_button(ui, l, "…");
                    let kind = match (name.as_str(), class.as_str()) {
                        ("image", _) => AssetKind::Image,
                        (_, "Model3D") => AssetKind::Model,
                        (_, "VideoPlayer") => AssetKind::Video,
                        _ => AssetKind::Audio,
                    };
                    let (state, ids, targets, name) = (state.clone(), *ids, targets.clone(), name.clone());
                    ui.on_click(more, move |_, _| pick_asset(&state, &ids, &targets, &name, kind));
                } else if name == "icon" || name.ends_with("_icon") {
                    // O seletor de ícones (pacotes do aylis, Material, Fluent e os do app).
                    let more = small_button(ui, l, "…");
                    let (state, ids, target, prop) = (state.clone(), *ids, targets[0], name.clone());
                    ui.on_click(more, move |ui, _| actions::pick_icon(ui, &state, &ids, Some(target), prop.clone()));
                } else if name == "font_family" {
                    // O editor de fonte: família, tamanho, peso e estilos.
                    let more = small_button(ui, l, "…");
                    let (state, ids, targets) = (state.clone(), *ids, targets.clone());
                    ui.on_click(more, move |ui, _| {
                        let initial = {
                            let st = state.borrow();
                            let props = st.doc.props_json(targets[0]);
                            let flag = |name: &str| props.get(name).and_then(Value::as_bool).unwrap_or(false);
                            propeditors::FontChoice {
                                family: props.get("font_family").and_then(Value::as_str).unwrap_or_default().to_string(),
                                size: props.get("font_size").and_then(Value::as_f64),
                                // O `bold` do Label conta como negrito.
                                weight: match props.get("font_weight").and_then(Value::as_str) {
                                    Some(w) => w.to_string(),
                                    None if flag("bold") => "bold".into(),
                                    None => "default".into(),
                                },
                                italic: flag("font_italic"),
                                underline: flag("font_underline"),
                                strikeout: flag("font_strikeout"),
                            }
                        };
                        // As fontes nomeadas do projeto primeiro (o componente segue o nome; a prévia
                        // do editor mostra a família dele).
                        let named: Vec<(String, String)> = state.borrow().project.as_ref().map(|p| p.fonts.iter().map(|(n, f)| (n.clone(), f.clone())).collect()).unwrap_or_default();
                        ui.set_font_names(named.clone());
                        let mut families: Vec<String> = named.into_iter().map(|(n, _)| n).collect();
                        families.extend(project_font_families());
                        let system = crate::host::system_font_families();
                        let (state, targets) = (state.clone(), targets.clone());
                        propeditors::font_editor(ui, initial, families, system, move |ui, c| {
                            let mut values = vec![
                                ("font_family".to_string(), Value::String(c.family)),
                                ("font_size".to_string(), c.size.map_or(Value::Null, Value::from)),
                                ("font_weight".to_string(), Value::String(c.weight)),
                                ("font_italic".to_string(), Value::Bool(c.italic)),
                                ("font_underline".to_string(), Value::Bool(c.underline)),
                                ("font_strikeout".to_string(), Value::Bool(c.strikeout)),
                            ];
                            // O peso agora diz o negrito: o `bold` antigo do Label sai.
                            let st = state.borrow();
                            if targets.iter().all(|&t| st.doc.ui.prop_info(t, "bold").is_some()) {
                                values.push(("bold".to_string(), Value::Bool(false)));
                            }
                            drop(st);
                            commit_many(ui, &state, &ids, &targets, values);
                        });
                    });
                } else if matches!(name.as_str(), "allowed_chars" | "blocked_chars" | "char_map" | "char_case") {
                    // A entrada de texto: as trocas, maiúsculas e os caracteres liberados e bloqueados
                    // (com os modelos prontos), num editor só.
                    let more = small_button(ui, l, "…");
                    let (state, ids, targets) = (state.clone(), *ids, targets.clone());
                    ui.on_click(more, move |ui, _| {
                        let initial = {
                            let st = state.borrow();
                            let props = st.doc.props_json(targets[0]);
                            let text = |n: &str| props.get(n).and_then(Value::as_str).unwrap_or_default().to_string();
                            let flag = |n: &str| props.get(n).and_then(Value::as_bool).unwrap_or(false);
                            let container = st.doc.ui.prop_info(targets[0], "chars_override").is_some();
                            let field = st.doc.ui.prop_info(targets[0], "char_case").is_some();
                            propeditors::CharsChoice {
                                allowed: text("allowed_chars"),
                                blocked: text("blocked_chars"),
                                only_allowed: flag("only_allowed"),
                                overrides: container.then(|| flag("chars_override")),
                                map: text("char_map"),
                                case: field.then(|| props.get("char_case").and_then(Value::as_str).unwrap_or("normal").to_string()),
                            }
                        };
                        let (state, targets) = (state.clone(), targets.clone());
                        propeditors::chars_editor(ui, initial, move |ui, c| {
                            let mut values = vec![
                                ("allowed_chars".to_string(), Value::String(c.allowed)),
                                ("blocked_chars".to_string(), Value::String(c.blocked)),
                                ("only_allowed".to_string(), Value::Bool(c.only_allowed)),
                                ("char_map".to_string(), Value::String(c.map)),
                            ];
                            if let Some(case) = c.case {
                                values.push(("char_case".to_string(), Value::String(case)));
                            }
                            if let Some(on) = c.overrides {
                                values.push(("chars_override".to_string(), Value::Bool(on)));
                            }
                            commit_many(ui, &state, &ids, &targets, values);
                        });
                    });
                } else if name == "tab_keys" || name == "back_tab_keys" {
                    // As teclas que fazem o papel do Tab (uma por linha; as comuns sugeridas).
                    let more = small_button(ui, l, "…");
                    let (commit_to, name2, shown) = (commit_to.clone(), name.clone(), shown.clone());
                    ui.on_click(more, move |ui, _| {
                        let items: Vec<String> = shown.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect();
                        let suggestions: Vec<String> = if name2 == "tab_keys" { ["Enter", "Down", "Right", "F2"] } else { ["Shift+Enter", "Up", "Left", "Shift+F2"] }
                            .iter()
                            .map(|s| s.to_string())
                            .collect();
                        let (commit_to, name3) = (commit_to.clone(), name2.clone());
                        let title = if name2 == "tab_keys" { "Teclas que fazem o Tab" } else { "Teclas que fazem o Shift+Tab" };
                        dialogs::list_editor(ui, title, items, suggestions, move |ui, list| {
                            commit_to(ui, &name3, Value::String(list.join(";")));
                        });
                    });
                } else if name == "items" || name == "columns" {
                    let more = small_button(ui, l, "…");
                    let (state, commit_to, name2, shown) = (state.clone(), commit_to.clone(), name.clone(), shown.clone());
                    let target = targets[0];
                    ui.on_click(more, move |ui, _| {
                        let items: Vec<String> = shown.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect();
                        let suggestions = if name2 == "columns" { grid_fields(&state, target) } else { Vec::new() };
                        let (commit_to, name3) = (commit_to.clone(), name2.clone());
                        let title = if name2 == "columns" { "Colunas do grid" } else { "Itens" };
                        dialogs::list_editor(ui, title, items, suggestions, move |ui, list| {
                            commit_to(ui, &name3, Value::String(list.join(";")));
                        });
                    });
                }
                text_editor(ui, l, shown, move |ui, text| match parse(kind, &text) {
                    Ok(v) => commit_to(ui, &name, v),
                    Err(e) => err_to(ui, &name, &e),
                });
            }
        }
    }
}


/// As famílias das fontes do projeto (`assets/fonts`), já registradas pela IDE.
fn project_font_families() -> Vec<String> {
    let mut out = aylis_core::fonts::families();
    out.sort_by_key(|f| f.to_lowercase());
    out.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    out
}

fn bounds_sides(value: &Value) -> [f64; 4] {
    match value {
        Value::Number(n) => [n.as_f64().unwrap_or(0.0); 4],
        Value::Array(a) if a.len() == 2 => {
            let (h, v) = (a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0));
            [h, v, h, v]
        }
        Value::Array(a) if a.len() == 4 => [0, 1, 2, 3].map(|i| a[i].as_f64().unwrap_or(0.0)),
        _ => [0.0; 4],
    }
}

fn format_number(v: f64) -> String {
    if v.fract() == 0.0 { format!("{v:.0}") } else { v.to_string() }
}

/// Campos do dataset ligado ao grid (para sugerir colunas).
fn grid_fields(state: &Shared, grid: ComponentId) -> Vec<String> {
    let st = state.borrow();
    let ui = &st.doc.ui;
    let Ok(PropValue::Component(Some(ds))) = ui.get_prop(grid, "data_source") else { return Vec::new() };
    let Ok(PropValue::Component(Some(cds))) = ui.get_prop(ds, "data_set") else { return Vec::new() };
    ui.children(cds).iter().filter(|&&f| ui.class_name(f) == Some("Field")).filter_map(|&f| ui.name(f).map(str::to_string)).collect()
}

/// Tipo de arquivo do projeto (e a subpasta de `assets/`).
#[derive(Clone, Copy)]
enum AssetKind {
    Image,
    Audio,
    Video,
    Model,
}

impl AssetKind {
    fn folder(self) -> &'static str {
        match self {
            AssetKind::Image => "assets/images",
            AssetKind::Audio => "assets/audio",
            AssetKind::Video => "assets/video",
            AssetKind::Model => "assets/models",
        }
    }

    fn filter(self) -> (&'static str, &'static [&'static str]) {
        match self {
            AssetKind::Image => ("Imagens", &["png", "jpg", "jpeg", "webp", "gif", "bmp", "ico", "svg"]),
            AssetKind::Audio => ("Áudio", &["mp3", "m4a", "aac", "wav", "flac", "ogg"]),
            // MP4 com H.264 + AAC funciona em todas as plataformas.
            AssetKind::Video => ("Vídeo (MP4 H.264 + AAC é o mais compatível)", &["mp4", "m4v", "mov", "webm", "mkv", "wmv", "avi"]),
            // FBX é convertido para .glb ao escolher (o app só carrega glTF).
            AssetKind::Model => ("Modelos 3D (glTF binário ou FBX)", &["glb", "gltf", "fbx"]),
        }
    }
}

/// Escolhe um arquivo (imagem, áudio, vídeo): se estiver fora do projeto, copia para a
/// subpasta de `assets/`. Vídeo fora de MP4 recebe um aviso de compatibilidade.
fn pick_asset(state: &Shared, ids: &Ids, targets: &[ComponentId], prop: &str, kind: AssetKind) {
    let (state2, _ids, targets, prop) = (state.clone(), *ids, targets.to_vec(), prop.to_string());
    let start = state.borrow().project.as_ref().map(|p| p.dir().join(kind.folder()));
    actions::spawn(state, async move {
        let (label, extensions) = kind.filter();
        let mut dialog = crate::files::AsyncFileDialog::new().add_filter(label, extensions);
        if let Some(dir) = start {
            dialog = dialog.set_directory(dir);
        }
        let Some(file) = dialog.pick_file().await else { return };
        let mut st = state2.borrow_mut();
        // FBX: vira .glb em assets/models/ antes de tudo.
        let mut path = file.path().to_path_buf();
        let mut imported = String::new();
        if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("fbx")) {
            let dir = st.project.as_ref().map(|p| p.dir());
            match import_model(&path, dir.as_deref()) {
                Ok((glb, summary)) => {
                    path = glb;
                    imported = format!("  (FBX convertido: {summary})");
                }
                Err(e) => {
                    st.status = format!("⚠ {e}");
                    return;
                }
            }
        }
        let value = match st.project.clone() {
            Some(project) => match relative_asset(&project.dir(), &path, kind.folder()) {
                Ok(rel) => rel,
                Err(e) => {
                    st.status = format!("⚠ {e}");
                    return;
                }
            },
            None => path.display().to_string(),
        };
        let warning = match kind {
            AssetKind::Video if !value.to_lowercase().ends_with(".mp4") => {
                "  (aviso: só MP4 com H.264 + AAC funciona em todas as plataformas)"
            }
            _ => "",
        };
        st.status = match st.doc.set_prop_many(&targets, &prop, &Value::String(value.clone())) {
            Ok(()) => format!("{prop}: {value}{warning}{imported}"),
            Err(e) => format!("⚠ {e}"),
        };
    });
}

/// Converte um FBX em `.glb`: em `assets/models/` do projeto (ou ao lado do FBX, sem
/// projeto). Devolve o arquivo gravado e um resumo ("2 malhas, 34 ossos, 3 clipes").
pub fn import_model(fbx: &Path, project_dir: Option<&Path>) -> Result<(std::path::PathBuf, String), String> {
    let bytes = std::fs::read(fbx).map_err(|e| format!("{}: {e}", fbx.display()))?;
    let model = crate::host::model_from_fbx(&bytes, fbx.parent())?;
    let glb = aylis_3d::glb::to_glb(&model)?;
    let dir = match project_dir {
        Some(p) => p.join("assets").join("models"),
        None => fbx.parent().map(Path::to_path_buf).unwrap_or_default(),
    };
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let stem = fbx.file_stem().and_then(|s| s.to_str()).unwrap_or("modelo");
    let out = dir.join(format!("{stem}.glb"));
    std::fs::write(&out, glb).map_err(|e| format!("{}: {e}", out.display()))?;
    let bones: usize = model.skins.iter().map(|s| s.joints.len()).sum();
    let plural = |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    let summary = [
        plural(model.meshes.len(), "malha", "malhas"),
        plural(bones, "osso", "ossos"),
        plural(model.clips.len(), "clipe", "clipes"),
        plural(model.textures.len(), "textura", "texturas"),
    ]
    .join(", ");
    Ok((out, summary))
}

/// Caminho relativo à pasta do projeto; copia para `folder` (em `assets/`) se estiver fora.
pub fn relative_asset(project_dir: &Path, file: &Path, folder: &str) -> Result<String, String> {
    let dir = project_dir.canonicalize().map_err(|e| e.to_string())?;
    let file_c = file.canonicalize().map_err(|e| e.to_string())?;
    let target = match file_c.strip_prefix(&dir) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => {
            let name = file.file_name().ok_or("arquivo sem nome")?;
            let dest = dir.join(folder).join(name);
            std::fs::create_dir_all(dest.parent().expect("tem pai")).map_err(|e| e.to_string())?;
            std::fs::copy(&file_c, &dest).map_err(|e| format!("não consegui copiar para {folder}: {e}"))?;
            PathBuf::from(folder).join(name)
        }
    };
    Ok(target.to_string_lossy().replace('\\', "/"))
}

use std::path::PathBuf;
