//! Onde é usado (item 20): os usos de um componente, de um campo ou de um handler no projeto
//! inteiro, por nome, nos forms e no código.
//!
//! - **componente** (`cdsCliente`): as propriedades dos forms que apontam para ele
//!   (`data_source`, `master_source`, `dmDados.cdsCliente` de outro form, as ligações
//!   `bindings`, as vistas) e o nome no código, palavra inteira;
//! - **campo** (`cdsCliente.Saldo`): os controles ligados ao dataset dele (pelos `DataSource`
//!   que apontam para ele) com o campo no `data_field` ou nas colunas, as expressões e as
//!   chaves do próprio dataset (filtro, índice, campos calculados, mestre-detalhe) e, no código,
//!   o nome do campo entre aspas (`"saldo"`, como o código pede o valor);
//! - **handler** (`btn_salvar_click`): os eventos dos forms ligados a ele e o nome no código.
//!
//! O resultado vem no formato da pesquisa no projeto ([`crate::projectsearch`]), que mostra a
//! lista e leva ao lugar com um clique.

use std::path::{Path, PathBuf};

use aylis_controls::search::SearchOptions;
use aylis_project::Project;
use serde_json::Value;

use crate::projectsearch::{FormHit, Results};

/// O que procurar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// Um componente do form `form`.
    Component { form: String, name: String },
    /// O campo `field` (o `field_name`) do dataset `data_set` do form `form`.
    Field { form: String, data_set: String, field: String },
    /// Um handler de evento (o nome do método).
    Handler { name: String },
}

impl Target {
    /// Como mostrar na janela ("Usos de cdsCliente.saldo").
    pub fn label(&self) -> String {
        match self {
            Target::Component { name, .. } => name.clone(),
            Target::Field { data_set, field, .. } => format!("{data_set}.{field}"),
            Target::Handler { name } => format!("{name}()"),
        }
    }
}

/// O alvo para o componente `component` do form `form` (o layout JSON dele): um `Field` vira
/// o campo do dataset pai; o resto, o componente.
pub fn target_of(layout: &Value, form: &str, component: &str) -> Target {
    fn find<'a>(node: &'a Value, parent: Option<&'a Value>, name: &str) -> Option<(&'a Value, Option<&'a Value>)> {
        if node["name"].as_str() == Some(name) {
            return Some((node, parent));
        }
        node["children"].as_array()?.iter().find_map(|c| find(c, Some(node), name))
    }
    if let Some((node, Some(parent))) = find(&layout["root"], None, component) {
        if node["class"].as_str() == Some("Field") {
            let field = node["field_name"].as_str().unwrap_or(component).to_string();
            let data_set = parent["name"].as_str().unwrap_or_default().to_string();
            return Target::Field { form: form.to_string(), data_set, field };
        }
    }
    Target::Component { form: form.to_string(), name: component.to_string() }
}

/// O texto tem `word` como palavra inteira (sem diferenciar maiúsculas quando `fold`)?
fn has_word(text: &str, word: &str, fold: bool) -> bool {
    let (text, word) = if fold { (text.to_lowercase(), word.to_lowercase()) } else { (text.to_string(), word.to_string()) };
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(&word).any(|(i, _)| {
        let before = text[..i].chars().next_back().is_none_or(|c| !ident(c));
        let after = text[i + word.len()..].chars().next().is_none_or(|c| !ident(c));
        before && after
    })
}

/// O texto de um valor do form (só os simples; listas viram texto separado por vírgula).
fn shown(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Array(items) if items.iter().all(Value::is_string) => Some(items.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")),
        _ => None,
    }
}

/// Todos os componentes de um form: (nome, nó).
fn components(node: &Value, out: &mut Vec<(String, Value)>) {
    if let Some(name) = node["name"].as_str() {
        out.push((name.to_string(), node.clone()));
    }
    for child in node["children"].as_array().into_iter().flatten() {
        components(child, out);
    }
}

/// Uma referência por nome aponta para `name` do form `owner`, vista do form `form`? (O nome
/// sozinho no mesmo form; `owner.name` de qualquer form.)
fn refers(value: &str, name: &str, owner: &str, form: &str) -> bool {
    value == name && form == owner || value.split_once('.').is_some_and(|(f, n)| f == owner && n == name)
}

/// Os usos nos forms.
fn in_forms(forms: &[(String, PathBuf, Value)], target: &Target) -> Vec<FormHit> {
    let mut hits = Vec::new();
    let mut hit = |form: &str, file: &Path, component: &str, property: &str, value: String| {
        hits.push(FormHit { form: form.to_string(), file: file.to_path_buf(), component: component.to_string(), property: property.to_string(), value });
    };
    for (form, file, layout) in forms {
        let mut list = Vec::new();
        components(&layout["root"], &mut list);
        match target {
            Target::Component { form: owner, name } => {
                for (component, node) in &list {
                    for (key, value) in node.as_object().into_iter().flatten() {
                        match key.as_str() {
                            "children" | "class" | "name" => {}
                            // As ligações: `{"text": {"source": "dsX", "field": ".."}}`.
                            "bindings" => {
                                for (prop, b) in value.as_object().into_iter().flatten() {
                                    if b["source"].as_str().is_some_and(|s| refers(s, name, owner, form)) {
                                        hit(form, file, component, &format!("bindings.{prop}"), b["source"].as_str().unwrap_or_default().to_string());
                                    }
                                }
                            }
                            // O que as vistas mudam nele (no form dele).
                            "views" if form == owner => {
                                for view in value.as_array().into_iter().flatten() {
                                    if view["changes"].get(name.as_str()).is_some() {
                                        hit(form, file, component, "views", view["name"].as_str().unwrap_or_default().to_string());
                                    }
                                }
                            }
                            _ => {
                                if let Some(text) = value.as_str().filter(|s| refers(s, name, owner, form)) {
                                    hit(form, file, component, key, text.to_string());
                                }
                            }
                        }
                    }
                }
            }
            Target::Field { form: owner, data_set, field } => {
                // Os DataSource deste form (ou de outro, apontando para o dataset pelo nome
                // completo) ligados ao dataset do campo.
                let sources: Vec<String> = list.iter().filter(|(_, n)| n["class"].as_str() == Some("DataSource") && n["data_set"].as_str().is_some_and(|d| refers(d, data_set, owner, form))).map(|(n, _)| n.clone()).collect();
                // O campo entre colchetes num texto (`"Cliente: [cliente]"`, o jeito do
                // relatório), nos forms ligados ao dataset.
                let placeholder = format!("[{}]", field.to_lowercase());
                if !sources.is_empty() {
                    for (component, node) in &list {
                        for key in ["text", "caption"] {
                            if let Some(text) = node[key].as_str().filter(|t| t.to_lowercase().contains(&placeholder)) {
                                hit(form, file, component, key, text.to_string());
                            }
                        }
                    }
                }
                for (component, node) in &list {
                    let bound = node["data_source"].as_str().is_some_and(|s| sources.iter().any(|d| d == s) || s.split_once('.').is_some_and(|(_, n)| sources.iter().any(|d| d == n)));
                    // O próprio dataset e os campos dele (no form dele).
                    let own = form == owner && (component == data_set || list.iter().any(|(n, p)| n == data_set && p["children"].as_array().is_some_and(|c| c.iter().any(|c| c["name"].as_str() == Some(component.as_str())))));
                    if !bound && !own {
                        continue;
                    }
                    for (key, value) in node.as_object().into_iter().flatten() {
                        if matches!(key.as_str(), "children" | "class" | "name" | "data_source" | "display_label" | "hint" | "text" | "caption") {
                            continue;
                        }
                        // O próprio campo não se usa: só o que fala dele em outro lugar.
                        if key == "field_name" {
                            continue;
                        }
                        if let Some(text) = shown(value).filter(|t| has_word(t, field, true)) {
                            hit(form, file, component, key, text);
                        }
                    }
                }
            }
            Target::Handler { name } => {
                for (component, node) in &list {
                    for (key, value) in node.as_object().into_iter().flatten() {
                        if key.starts_with("on_") || key.starts_with("before_") || key.starts_with("after_") {
                            if value.as_str() == Some(name.as_str()) {
                                hit(form, file, component, key, name.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    hits
}

/// O que procurar no código: o nome (palavra inteira) ou, para um campo, o nome entre aspas.
fn code_options(target: &Target) -> SearchOptions {
    match target {
        Target::Component { name, .. } | Target::Handler { name } => SearchOptions { query: name.clone(), case: true, whole: true, ..Default::default() },
        Target::Field { field, .. } => {
            let escaped: String = field.chars().map(|c| if c.is_alphanumeric() || c == '_' { c.to_string() } else { format!("\\{c}") }).collect();
            SearchOptions { query: format!("[\"']{escaped}[\"']"), regex: true, ..Default::default() }
        }
    }
}

/// Os usos de `target` no projeto: nos forms (pelo JSON de cada um) e nos fontes do código.
pub fn find(project: &Project, target: &Target) -> Result<Results, String> {
    let forms: Vec<(String, PathBuf, Value)> = project
        .forms
        .iter()
        .filter_map(|f| {
            let file = project.file(&f.file);
            let layout = serde_json::from_str(&std::fs::read_to_string(&file).ok()?).ok()?;
            Some((f.name.clone(), file, layout))
        })
        .collect();
    let form_files: Vec<PathBuf> = forms.iter().map(|(_, f, _)| f.clone()).collect();
    let code = crate::projectsearch::files(&project.dir(), "", "*.json", &form_files);
    let mut results = crate::projectsearch::search(&code_options(target), &code, &[])?;
    // O código gerado pela IDE repete os nomes de todos os componentes: fica de fora.
    results.text.retain(|h| !h.file.components().any(|c| c.as_os_str() == "generated"));
    results.forms = in_forms(&forms, target);
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn forms() -> Vec<(String, PathBuf, Value)> {
        let clientes = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmClientes", "on_show": "form_show", "children": [
            { "class": "ClientDataSet", "name": "cdsCliente", "filter": "saldo > 0", "index_field_names": "nome;Saldo desc", "children": [
                { "class": "Field", "name": "cdsClienteNome", "field_name": "nome" },
                { "class": "Field", "name": "cdsClienteSaldo", "field_name": "saldo", "display_label": "Saldo" },
                { "class": "Field", "name": "cdsClienteDobro", "field_name": "dobro", "field_kind": "calculated", "expression": "saldo * 2" }
            ] },
            { "class": "DataSource", "name": "dsCliente", "data_set": "cdsCliente" },
            { "class": "DBEdit", "name": "edtSaldo", "data_source": "dsCliente", "data_field": "saldo" },
            { "class": "DBEdit", "name": "edtNome", "data_source": "dsCliente", "data_field": "nome", "on_change": "form_show" },
            { "class": "DataGrid", "name": "grdClientes", "data_source": "dsCliente", "columns": ["nome", "saldo"] },
            { "class": "Label", "name": "lblSaldo", "text": "saldo", "bindings": { "visible": { "source": "dsCliente", "field": "ativo" } } }
        ] } });
        let pedidos = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmPedidos", "children": [
            { "class": "DataSource", "name": "dsClientes", "data_set": "frmClientes.cdsCliente" },
            { "class": "DBEdit", "name": "edtLimite", "data_source": "dsClientes", "data_field": "saldo" },
            { "class": "DBEdit", "name": "edtOutro", "data_source": "dsOutro", "data_field": "saldo" }
        ] } });
        vec![("frmClientes".into(), PathBuf::from("forms/clientes.json"), clientes), ("frmPedidos".into(), PathBuf::from("forms/pedidos.json"), pedidos)]
    }

    fn places(hits: &[FormHit]) -> Vec<String> {
        hits.iter().map(|h| format!("{}.{}.{}", h.form, h.component, h.property)).collect()
    }

    #[test]
    fn a_component_is_found_by_name_also_from_other_forms() {
        let f = forms();
        let target = target_of(&f[0].2, "frmClientes", "dsCliente");
        assert_eq!(target, Target::Component { form: "frmClientes".into(), name: "dsCliente".into() });
        assert_eq!(places(&in_forms(&f, &target)), ["frmClientes.edtSaldo.data_source", "frmClientes.edtNome.data_source", "frmClientes.grdClientes.data_source", "frmClientes.lblSaldo.bindings.visible"]);
        let cds = Target::Component { form: "frmClientes".into(), name: "cdsCliente".into() };
        assert_eq!(places(&in_forms(&f, &cds)), ["frmClientes.dsCliente.data_set", "frmPedidos.dsClientes.data_set"], "o de outro form pelo nome completo");
    }

    #[test]
    fn a_field_is_found_in_bound_controls_columns_and_expressions_but_not_in_texts() {
        let f = forms();
        let target = target_of(&f[0].2, "frmClientes", "cdsClienteSaldo");
        assert_eq!(target, Target::Field { form: "frmClientes".into(), data_set: "cdsCliente".into(), field: "saldo".into() });
        assert_eq!(target.label(), "cdsCliente.saldo");
        assert_eq!(
            places(&in_forms(&f, &target)),
            ["frmClientes.cdsCliente.filter", "frmClientes.cdsCliente.index_field_names", "frmClientes.cdsClienteDobro.expression", "frmClientes.edtSaldo.data_field", "frmClientes.grdClientes.columns", "frmPedidos.edtLimite.data_field"],
            "o texto do Label e o campo de outro dataset ficam de fora"
        );
        assert!(code_options(&target).find_all("ds.value(\"saldo\")?; let saldo = 1;").unwrap().len() == 1, "no código, entre aspas");
    }

    #[test]
    fn a_handler_is_found_in_every_event_bound_to_it() {
        let target = Target::Handler { name: "form_show".into() };
        assert_eq!(places(&in_forms(&forms(), &target)), ["frmClientes.frmClientes.on_show", "frmClientes.edtNome.on_change"]);
    }
}
