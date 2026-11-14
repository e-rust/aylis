//! Carregar e salvar a árvore de componentes.

use aylis_core::{ComponentId, PropKind, PropValue, Ui, read_prop};
use serde_json::{Map, Value};

use crate::value::{describe, from_prop, object, to_prop};
use crate::{Error, FORMAT_VERSION, Result};

/// `uses`: os DataModules que o form usa (o aplicativo cria antes; ver
/// `aylis_project::ProjectForms`).
pub(crate) const RESERVED: [&str; 6] = ["class", "name", "children", "uses", "bindings", "views"];

/// As propriedades que limitam outras (o `value` cabe entre elas): carregadas primeiro.
const LIMITS: [&str; 6] = ["min", "max", "minimum", "maximum", "min_value", "max_value"];

/// Carrega um documento (`{"aylis": 1, "root": {...}}`) e devolve o componente raiz.
///
/// Com `parent`, a raiz vira filha dele (para carregar um pedaço de tela dentro de outra);
/// sem, vira um form. Se der erro, nada do documento fica no `Ui`.
pub fn load(ui: &mut Ui, json: &str, parent: Option<ComponentId>) -> Result<ComponentId> {
    let value: Value = serde_json::from_str(json.trim_start_matches('\u{feff}'))?;
    load_value(ui, &value, parent)
}

/// Como [`load`], a partir de um valor já lido (por exemplo vindo do protocolo).
///
/// Carregar não dispara o `on_change` dos valores.
pub fn load_value(ui: &mut Ui, document: &Value, parent: Option<ComponentId>) -> Result<ComponentId> {
    ui.without_change_events(|ui| load_document(ui, document, parent))
}

fn load_document(ui: &mut Ui, document: &Value, parent: Option<ComponentId>) -> Result<ComponentId> {
    let doc = object(document, "o documento").map_err(structure(""))?;
    if let Some(version) = doc.get("aylis") {
        let version = version.as_u64().ok_or_else(|| structure("")("\"aylis\" deve ser o número da versão".into()))?;
        // Só o próprio formato: outro (mais novo ou mais antigo) passa pelo aylis-migrate.
        if version != FORMAT_VERSION {
            return Err(Error::Version(version));
        }
    }
    for key in doc.keys() {
        if !matches!(key.as_str(), "aylis" | "root" | "$schema") {
            return Err(structure("")(format!("chave desconhecida no documento: '{key}' (use \"aylis\" e \"root\")")));
        }
    }
    let root = doc.get("root").ok_or_else(|| structure("")("falta \"root\"".into()))?;

    let mut loader = Loader { ui, created: None, deferred: Vec::new(), bindings: Vec::new(), views: Vec::new(), errors: None };
    let result = loader.node(root, parent, &mut Vec::new()).and_then(|id| loader.fixup().map(|_| id));
    if result.is_err() {
        if let Some(created) = loader.created {
            loader.ui.free(created);
        }
    }
    result
}

pub(crate) struct Loader<'a> {
    pub(crate) ui: &'a mut Ui,
    /// Primeiro componente criado (para desfazer em caso de erro).
    pub(crate) created: Option<ComponentId>,
    /// Referências a outros componentes, resolvidas no fim: (componente, propriedade, nome, caminho).
    pub(crate) deferred: Vec<(ComponentId, String, String, String)>,
    /// As ligações a dados, resolvidas no fim: (componente, propriedade, DataSource, campo).
    pub(crate) bindings: Vec<(ComponentId, String, String, String)>,
    /// As vistas por largura de um form (veja `aylis_core::views`), resolvidas no fim.
    pub(crate) views: Vec<(ComponentId, Value)>,
    /// Com `Some`, os erros de propriedade e de classe são juntados aqui e o carregamento
    /// segue ([`validate`]): todos de uma vez, em vez de um por tentativa.
    pub(crate) errors: Option<Vec<Error>>,
}

/// Confere um layout inteiro e devolve **todos** os erros (vazio: carrega). Monta numa raiz
/// de passagem no `ui` e a libera no fim. Um layout de 24 KB com cinco erros ia e voltava
/// cinco vezes, um erro por vez (relatório do RTSPlayer, 10/11).
pub fn validate(ui: &mut Ui, json: &str) -> Vec<Error> {
    let doc: Value = match serde_json::from_str(json.trim_start_matches('\u{feff}')) {
        Ok(v) => v,
        Err(e) => return vec![Error::Structure { path: String::new(), message: format!("JSON inválido: {e}") }],
    };
    let Some(root) = doc.get("root") else { return vec![structure("")("falta \"root\"".into())] };
    let mut loader = Loader { ui, created: None, deferred: Vec::new(), bindings: Vec::new(), views: Vec::new(), errors: Some(Vec::new()) };
    let result = loader.node(root, None, &mut Vec::new()).and_then(|_| loader.fixup());
    let mut errors = loader.errors.take().unwrap_or_default();
    if let Err(e) = result {
        errors.push(e);
    }
    if let Some(created) = loader.created {
        loader.ui.free(created);
    }
    errors
}

impl Loader<'_> {
    /// Um erro que não impede o resto: no [`validate`], anotado (e segue); senão, devolvido.
    fn soft(&mut self, error: Error) -> Result<()> {
        match self.errors.as_mut() {
            Some(list) => {
                list.push(error);
                Ok(())
            }
            None => Err(error),
        }
    }

    pub(crate) fn node(&mut self, value: &Value, parent: Option<ComponentId>, path: &mut Vec<String>) -> Result<ComponentId> {
        // O caminho (`frmX > pnl > btn`) só é montado quando há um erro: cada componente de um
        // frame repetido (`FraX::create_in`) passava por dois `join` (G21 do Tabernáculo).
        let obj = object(value, "cada componente").map_err(|e| structure(&path.join(" > "))(e))?;
        let class = obj
            .get("class")
            .and_then(Value::as_str)
            .ok_or_else(|| structure(&path.join(" > "))("falta \"class\" (o nome da classe do componente)".into()))?;
        let name = match obj.get("name") {
            None | Some(Value::Null) => None,
            Some(Value::String(s)) => Some(s.as_str()),
            Some(other) => return Err(structure(&path.join(" > "))(format!("\"name\" deve ser texto, recebido {}", describe(other)))),
        };
        path.push(name.unwrap_or(class).to_string());
        let here = |path: &[String]| path.join(" > ");

        let id = match self.ui.create(class, parent) {
            Ok(id) => id,
            // Classe que não existe: no `validate`, anota e pula o componente (e os filhos).
            Err(source) => {
                let error = Error::Component { path: here(path), source };
                match parent {
                    Some(p) if self.errors.is_some() => {
                        self.soft(error)?;
                        path.pop();
                        return Ok(p);
                    }
                    _ => return Err(error),
                }
            }
        };
        if self.created.is_none() && parent.is_some() && class == "Form" {
            // Um form carregado dentro de outro tem os próprios nomes (pode repetir os do
            // form de fora). Um pedaço de tela que não é form entra nos nomes do de fora.
            self.ui.set_name_scope(id, true);
        }
        self.created.get_or_insert(id);
        if let Some(name) = name {
            if let Err(source) = self.ui.set_name(id, name) {
                self.soft(Error::Component { path: here(path), source })?;
            }
        }

        // Os limites antes do resto: o `value` de um NumberBox ou de
        // uma TrackBar, gravado antes do `max` na ordem da classe, não é cortado pelo máximo
        // padrão ao abrir (o 10.000.000 virava 1000).
        // O `translate` também vem antes: desligado, os textos não são traduzidos ao carregar.
        // (Duas passadas pelo objeto, sem a lista ordenada de cada componente.)
        let first = |key: &str| LIMITS.contains(&key) || key == "translate";
        let entries = obj.iter().filter(|(key, _)| first(key)).chain(obj.iter().filter(|(key, _)| !first(key)));
        for (key, value) in entries {
            // `bindings` como texto é a propriedade da lista (`DataRepeater`, `VirtualList`:
            // "lblNome=nome"); como objeto, as ligações de propriedade a campo (abaixo). Pulada
            // sempre, a da lista sumia ao carregar o form (relatório do RTSPlayer, 10/11).
            if RESERVED.contains(&key.as_str()) && !(key == "bindings" && value.is_string()) {
                continue;
            }
            if let Err(e) = self.property(id, class, key, value, path) {
                self.soft(e)?;
            }
        }

        // "bindings": { "text": { "source": "dsClientes", "field": "nome" } }
        if let Some(Value::Object(list)) = obj.get("bindings") {
            for (property, b) in list {
                let source = b.get("source").and_then(Value::as_str).unwrap_or("").to_string();
                let field = b.get("field").and_then(Value::as_str).unwrap_or("").to_string();
                self.bindings.push((id, property.clone(), source, field));
            }
        }
        // "views": [ { "name": "phone", "max_width": 599, "changes": { "btn": { "visible": false } } } ]
        if let Some(views) = obj.get("views").filter(|v| v.is_array()) {
            self.views.push((id, views.clone()));
        }

        // O Frame mostra o form de `form`: filhos dele no JSON tomavam o lugar desse form, e
        // excluir o Frame os jogava no form de fora (relatório do InsightDB, 11/11).
        if class == "Frame" && obj.get("children").and_then(Value::as_array).is_some_and(|c| !c.is_empty()) {
            self.soft(structure(&here(path))("o Frame mostra o form da propriedade `form` e não tem filhos: ponha os componentes no form dele".into()))?;
            path.pop();
            return Ok(id);
        }
        match obj.get("children") {
            None | Some(Value::Null) => {}
            Some(Value::Array(children)) => {
                for child in children {
                    self.node(child, Some(id), path)?;
                }
            }
            Some(other) => {
                return Err(structure(&here(path))(format!("\"children\" deve ser uma lista, recebido {}", describe(other))));
            }
        }
        path.pop();
        Ok(id)
    }

    /// Uma propriedade do componente `id` (da classe `class`) pelo JSON.
    fn property(&mut self, id: ComponentId, class: &str, key: &str, value: &Value, path: &[String]) -> Result<()> {
        let fail = |message: String| Error::Property { path: path.join(" > "), property: key.to_string(), message };
        // "margins.left": 4
        if key.contains('.') {
            let v = value.as_f64().ok_or_else(|| fail(format!("esperado número, recebido {}", describe(value))))?;
            return self.ui.set_prop(id, key, v).map_err(|e| fail(e.to_string()));
        }
        let info = self.ui.prop_info(id, key).ok_or_else(|| fail(format!("a classe {class} não tem esta propriedade")))?;
        let prop = to_prop(value, info.kind, self.ui.theme()).map_err(fail)?;
        if info.kind == PropKind::Component {
            if let PropValue::String(target) = prop {
                self.deferred.push((id, key.to_string(), target, path.join(" > ")));
            }
            return Ok(());
        }
        // Textos da tela: traduzidos para o idioma atual (o original fica lembrado).
        if let (PropValue::String(text), true) = (&prop, aylis_core::i18n::TRANSLATABLE.contains(&key)) {
            let text = text.clone();
            return self.ui.set_translated(id, key, &text).map_err(|e| fail(strip_property(e)));
        }
        self.ui.set_prop(id, key, prop).map_err(|e| fail(strip_property(e)))
    }

    /// Resolve as referências por nome depois que tudo existe.
    pub(crate) fn fixup(&mut self) -> Result<()> {
        for (id, property, target, path) in std::mem::take(&mut self.deferred) {
            if let Err(e) = self.ui.set_prop(id, &property, target.as_str()) {
                self.soft(Error::Property { path, property, message: strip_property(e) })?;
            }
        }
        for (id, property, source, field) in std::mem::take(&mut self.bindings) {
            let source = self.ui.resolve_reference(id, &source);
            self.ui.set_binding(id, &property, source, &field);
        }
        for (form, list) in std::mem::take(&mut self.views) {
            let views = load_views(self.ui, form, &list);
            self.ui.set_views(form, views);
        }
        Ok(())
    }
}

/// As vistas do JSON, com os valores no tipo de cada propriedade:
/// `[{ "name", "max_width", "min_width", "min_height", "max_height", "orientation",
/// "platforms": [...], "font_scale", "spacing_scale", "changes": { "componente": { "prop": valor,
/// "parent": "pnlRodape", "index": 0 } } }]` (só `name` é obrigatório). O que não existe mais
/// (componente, propriedade) fica de fora.
fn load_views(ui: &Ui, form: ComponentId, list: &Value) -> Vec<aylis_core::FormView> {
    use aylis_core::views::{INDEX, PARENT};
    let mut views = Vec::new();
    for v in list.as_array().into_iter().flatten() {
        let Some(name) = v.get("name").and_then(Value::as_str) else { continue };
        let number = |key: &str, default: f64| v.get(key).and_then(Value::as_f64).unwrap_or(default) as f32;
        let mut view = aylis_core::FormView::new(name, number("max_width", 0.0));
        view.min_width = number("min_width", 0.0);
        view.min_height = number("min_height", 0.0);
        view.max_height = number("max_height", 0.0);
        view.font_scale = number("font_scale", 1.0);
        view.spacing_scale = number("spacing_scale", 1.0);
        if let Some(o) = v.get("orientation").and_then(Value::as_str) {
            view.orientation = <aylis_core::ViewOrientation as aylis_core::PropEnum>::from_name(o).unwrap_or_default();
        }
        view.platforms = v.get("platforms").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        for (component, props) in v.get("changes").and_then(Value::as_object).into_iter().flatten() {
            let Some(id) = (if component.is_empty() { Some(form) } else { ui.find_in(form, component) }) else { continue };
            for (property, value) in props.as_object().into_iter().flatten() {
                let value = match property.as_str() {
                    PARENT => value.as_str().map(|s| PropValue::String(s.to_string())),
                    INDEX => value.as_i64().map(PropValue::Int),
                    _ => crate::prop_from_json(ui, id, property, value).ok(),
                };
                if let Some(value) = value {
                    view.changes.push(aylis_core::ViewChange { component: component.clone(), property: property.clone(), value });
                }
            }
        }
        views.push(view);
    }
    views
}

/// As vistas do form para o JSON (as condições e escalas só quando não são as de sempre).
fn save_views(views: &[aylis_core::FormView]) -> Value {
    let list: Vec<Value> = views
        .iter()
        .map(|v| {
            let mut changes = Map::new();
            for c in &v.changes {
                let entry = changes.entry(c.component.clone()).or_insert_with(|| Value::Object(Map::new()));
                if let Value::Object(props) = entry {
                    props.insert(c.property.clone(), from_prop(&c.value));
                }
            }
            let mut map = Map::new();
            map.insert("name".into(), Value::from(v.name.clone()));
            map.insert("max_width".into(), crate::value::number(v.max_width as f64));
            for (key, value, default) in [
                ("min_width", v.min_width, 0.0),
                ("min_height", v.min_height, 0.0),
                ("max_height", v.max_height, 0.0),
                ("font_scale", v.font_scale, 1.0),
                ("spacing_scale", v.spacing_scale, 1.0),
            ] {
                if value != default {
                    map.insert(key.into(), crate::value::number(value as f64));
                }
            }
            if v.orientation != aylis_core::ViewOrientation::Any {
                map.insert("orientation".into(), Value::from(aylis_core::PropEnum::name(v.orientation)));
            }
            if !v.platforms.is_empty() {
                map.insert("platforms".into(), Value::from(v.platforms.clone()));
            }
            map.insert("changes".into(), Value::Object(changes));
            Value::Object(map)
        })
        .collect();
    Value::Array(list)
}

fn structure(path: &str) -> impl Fn(String) -> Error + '_ {
    move |message| Error::Structure { path: path.to_string(), message }
}

/// A mensagem do core já cita a propriedade; aqui ela vai no campo próprio.
fn strip_property(e: aylis_core::Error) -> String {
    match e {
        aylis_core::Error::TypeMismatch { expected, got, .. } => format!("esperado {expected}, recebido {got}"),
        aylis_core::Error::InvalidValue { message, .. } => message,
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------

/// Salva o componente e seus descendentes como documento JSON (formatado).
///
/// Só são gravadas as propriedades que diferem do padrão da classe. Eventos ligados
/// por closure não têm nome e ficam de fora; os ligados por nome são gravados.
pub fn save(ui: &Ui, root: ComponentId) -> Result<String> {
    let value = save_value(ui, root)?;
    Ok(crate::format::to_form_text(&value))
}

/// Como [`save`], devolvendo o valor JSON.
pub fn save_value(ui: &Ui, root: ComponentId) -> Result<Value> {
    let mut doc = Map::new();
    doc.insert("aylis".into(), Value::from(FORMAT_VERSION));
    doc.insert("root".into(), save_node(ui, root, &mut Vec::new())?);
    Ok(Value::Object(doc))
}

fn save_node(ui: &Ui, id: ComponentId, path: &mut Vec<String>) -> Result<Value> {
    let component = ui.component(id).ok_or_else(|| structure(&path.join(" > "))("componente inexistente".into()))?;
    let class = component.class_name();
    let name = ui.name(id).unwrap_or_default();
    path.push(if name.is_empty() { class.to_string() } else { name.to_string() });
    let here = path.join(" > ");

    // Uma instância nova da classe dá os valores padrão.
    let create = ui
        .registry()
        .get(class)
        .ok_or_else(|| structure(&here)(format!("a classe {class} não está registrada; registre-a para poder salvar")))?
        .create;
    let defaults = create();

    let mut map = Map::new();
    map.insert("class".into(), Value::String(class.to_string()));
    if !name.is_empty() {
        map.insert("name".into(), Value::String(name.to_string()));
    }
    for info in ui.prop_infos(id) {
        if info.read_only || info.runtime {
            continue;
        }
        // O valor de uma propriedade ligada vem dos dados (o designer mostra o registro atual).
        if ui.binding(id, info.name).is_some() {
            continue;
        }
        // Com uma vista por largura ativa, grava o valor do form base (o da vista fica nela).
        let value = match ui.view_base(id, info.name) {
            Some(base) => base.clone(),
            None => ui.get_declared_prop(id, info.name).map_err(|source| Error::Component { path: here.clone(), source })?,
        };
        let json = match (&value, info.kind) {
            (PropValue::Event(None), _) => continue,
            (PropValue::Component(target), PropKind::Component) => {
                match target.and_then(|t| ui.reference_name(id, t)) {
                    Some(target_name) => Value::String(target_name),
                    None => continue,
                }
            }
            (PropValue::Event(Some(_)), _) => from_prop(&value),
            _ => {
                if read_prop(defaults.as_ref(), info.name).as_ref() == Some(&value) {
                    continue;
                }
                from_prop(&value)
            }
        };
        map.insert(info.name.to_string(), json);
    }

    if !ui.bindings(id).is_empty() {
        let mut list = Map::new();
        for b in ui.bindings(id) {
            let mut item = Map::new();
            if let Some(name) = b.source.and_then(|s| ui.reference_name(id, s)) {
                item.insert("source".into(), Value::String(name));
            }
            item.insert("field".into(), Value::String(b.path.clone()));
            list.insert(b.property.clone(), Value::Object(item));
        }
        map.insert("bindings".into(), Value::Object(list));
    }
    if !ui.views(id).is_empty() {
        map.insert("views".into(), save_views(ui.views(id)));
    }

    // O conteúdo de um Frame é o form dele (fica no arquivo do form, não aqui).
    // Com uma vista ativa que mudou componentes de contêiner, os filhos do base.
    let own: Vec<ComponentId> = if class == "Frame" { Vec::new() } else { ui.base_children(id) };
    let children: Vec<Value> = own.iter().map(|&c| save_node(ui, c, path)).collect::<Result<_>>()?;
    if !children.is_empty() {
        map.insert("children".into(), Value::Array(children));
    }
    path.pop();
    Ok(Value::Object(map))
}
