//! Recarregar um layout num form aberto, sem recriá-lo (o "hot reload"): os componentes
//! que continuam no layout ficam (com dados, foco, posição de rolagem e os handlers
//! ligados por código), só as propriedades mudam; os novos são criados e os que saíram do
//! layout são liberados.

use aylis_core::{ComponentId, PropKind, PropValue, Ui, read_prop};
use serde_json::{Map, Value};

use crate::layout::{Loader, RESERVED};
use crate::value::{object, to_prop};
use crate::{Error, Result};

/// O que mudou num [`reload`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReloadReport {
    /// Propriedades alteradas.
    pub updated: usize,
    pub created: usize,
    pub removed: usize,
}

/// Aplica o layout `json` ao form (ou contêiner) `root` já existente.
///
/// Os filhos são casados pelo nome (e, sem nome, pela classe e pela ordem). Só são
/// liberados os componentes com nome que estavam em `previous` (o layout anterior) e saíram
/// do novo: os criados pelo código do aplicativo ficam. Como ao carregar, não dispara o
/// `on_change` dos valores.
pub fn reload(ui: &mut Ui, root: ComponentId, json: &str, previous: Option<&str>) -> Result<ReloadReport> {
    ui.without_change_events(|ui| reload_document(ui, root, json, previous))
}

fn reload_document(ui: &mut Ui, root: ComponentId, json: &str, previous: Option<&str>) -> Result<ReloadReport> {
    let doc: Value = serde_json::from_str(json.trim_start_matches('\u{feff}'))?;
    let node = doc.get("root").ok_or_else(|| Error::Structure { path: String::new(), message: "falta \"root\"".into() })?;
    let old_doc: Option<Value> = previous.and_then(|p| serde_json::from_str::<Value>(p.trim_start_matches('\u{feff}')).ok());
    let old_root = old_doc.as_ref().map(|v| &v["root"]);
    let mut before = Vec::new();
    if let Some(old) = old_root {
        collect_names(old, &mut before);
    }
    let mut now = Vec::new();
    collect_names(node, &mut now);

    let class = node.get("class").and_then(Value::as_str).unwrap_or_default();
    let current = ui.component(root).map(|c| c.class_name()).unwrap_or_default();
    if class != current {
        return Err(Error::Structure {
            path: String::new(),
            message: format!("o layout é de um {class}, mas o componente é um {current}"),
        });
    }
    let mut report = ReloadReport::default();
    let mut loader = Loader { ui, created: None, deferred: Vec::new(), bindings: Vec::new(), views: Vec::new(), errors: None };
    apply(&mut loader, root, node, old_root, &mut vec![class.to_string()], &mut report)?;
    loader.fixup()?;
    // Saíram do layout: só os que tinham nome e estavam no layout anterior.
    let gone: Vec<ComponentId> = before
        .iter()
        .filter(|n| !now.iter().any(|m| m.eq_ignore_ascii_case(n)))
        .filter_map(|n| ui.find_in(root, n))
        .collect();
    for id in gone {
        if ui.contains(id) {
            ui.free(id);
            report.removed += 1;
        }
    }
    ui.invalidate();
    Ok(report)
}

fn collect_names(node: &Value, out: &mut Vec<String>) {
    if let Some(name) = node.get("name").and_then(Value::as_str) {
        out.push(name.to_string());
    }
    if let Some(children) = node.get("children").and_then(Value::as_array) {
        children.iter().for_each(|c| collect_names(c, out));
    }
}

/// O filho de `old` que corresponde a `children[index]` (pelo nome; sem nome, pela classe
/// e pela ordem entre os sem nome da mesma classe).
fn old_child<'a>(old: Option<&'a Value>, children: &[Value], index: usize) -> Option<&'a Value> {
    let olds = old?.get("children")?.as_array()?;
    let child = &children[index];
    let class = child.get("class");
    match child.get("name").and_then(Value::as_str) {
        Some(name) => olds.iter().find(|o| o.get("name").and_then(Value::as_str).is_some_and(|n| n.eq_ignore_ascii_case(name))),
        None => {
            let nth = children[..index].iter().filter(|c| c.get("name").is_none() && c.get("class") == class).count();
            olds.iter().filter(|o| o.get("name").is_none() && o.get("class") == class).nth(nth)
        }
    }
}

/// Aplica `node` ao componente `id`. Com o nó do layout anterior (`old`), só o que mudou
/// nele: o que o usuário fez no app (um texto digitado, uma aba escolhida) fica. Sem ele
/// (primeira vez, ou componente novo), aplica tudo.
fn apply(loader: &mut Loader<'_>, id: ComponentId, node: &Value, old: Option<&Value>, path: &mut Vec<String>, report: &mut ReloadReport) -> Result<()> {
    let here = path.join(" > ");
    let obj: &Map<String, Value> = object(node, "cada componente").map_err(|m| Error::Structure { path: here.clone(), message: m })?;
    let ui = &mut *loader.ui;
    let class = ui.component(id).map(|c| c.class_name()).unwrap_or_default();
    // Valores padrão da classe: propriedade que saiu do JSON volta ao padrão.
    let defaults = ui.registry().get(class).map(|c| (c.create)());
    for info in ui.prop_infos(id) {
        // A propriedade `bindings` da lista (texto) vale; o objeto `bindings` são as ligações a
        // campo, aplicadas à parte.
        let list_bindings = info.name == "bindings" && obj.get("bindings").is_none_or(Value::is_string);
        if info.read_only || (RESERVED.contains(&info.name) && !list_bindings) {
            continue;
        }
        let fail = |message: String| Error::Property { path: here.clone(), property: info.name.to_string(), message };
        if old.is_some_and(|old| old.get(info.name) == obj.get(info.name)) {
            continue; // não mudou no layout
        }
        let wanted = match obj.get(info.name) {
            Some(json) => {
                let prop = to_prop(json, info.kind, ui.theme()).map_err(fail)?;
                if info.kind == PropKind::Component {
                    if let PropValue::String(target) = prop {
                        loader.deferred.push((id, info.name.to_string(), target, here.clone()));
                    }
                    continue;
                }
                prop
            }
            // Eventos ligados por código (closures) não aparecem no JSON: ficam.
            None if info.kind == PropKind::Event => continue,
            None => match defaults.as_ref().and_then(|d| read_prop(d.as_ref(), info.name)) {
                Some(default) => default,
                None => continue,
            },
        };
        if let (PropValue::String(text), true) = (&wanted, aylis_core::i18n::TRANSLATABLE.contains(&info.name)) {
            let text = text.clone();
            if ui.get_prop(id, info.name).ok() != Some(PropValue::String(ui.translate_prop(info.name, &text))) {
                ui.set_translated(id, info.name, &text).map_err(|e| fail(e.to_string()))?;
                report.updated += 1;
            }
            continue;
        }
        if ui.get_prop(id, info.name).ok().as_ref() != Some(&wanted) {
            ui.set_prop(id, info.name, wanted).map_err(|e| fail(e.to_string()))?;
            report.updated += 1;
        }
    }
    // "margins.left": 4
    for (key, value) in obj.iter().filter(|(k, _)| k.contains('.')) {
        if let Some(v) = value.as_f64() {
            let _ = ui.set_prop(id, key, v);
        }
    }

    let children: Vec<Value> = match obj.get("children") {
        Some(Value::Array(c)) => c.clone(),
        _ => Vec::new(),
    };
    let mut used: Vec<ComponentId> = Vec::new();
    let mut order: Vec<ComponentId> = Vec::new();
    for (index, child) in children.iter().enumerate() {
        let class = child.get("class").and_then(Value::as_str).unwrap_or_default();
        let name = child.get("name").and_then(Value::as_str);
        let existing = loader.ui.children(id).iter().copied().find(|&c| {
            let ui = &*loader.ui;
            !used.contains(&c)
                && ui.component(c).is_some_and(|x| x.class_name() == class)
                && match name {
                    Some(n) => ui.name(c).is_some_and(|m| m.eq_ignore_ascii_case(n)),
                    None => ui.name(c).is_none_or(str::is_empty),
                }
        });
        let child_id = match existing {
            Some(c) => {
                path.push(name.unwrap_or(class).to_string());
                apply(loader, c, child, old_child(old, &children, index), path, report)?;
                path.pop();
                c
            }
            None => {
                let c = loader.node(child, Some(id), path)?;
                report.created += 1;
                c
            }
        };
        used.push(child_id);
        order.push(child_id);
    }
    // A ordem dos filhos conta para o alinhamento: segue a do layout.
    for (i, &c) in order.iter().enumerate() {
        let _ = loader.ui.set_child_index(c, i);
    }
    Ok(())
}
