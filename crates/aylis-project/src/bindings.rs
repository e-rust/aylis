//! Acesso tipado aos componentes: cada form vira uma instância global com
//! o nome dele e um campo por componente, e cada componente tem as propriedades com o tipo
//! certo — `frmCalculadora.pnlLateral.Visible` em vez de `Aylis.Call('get', ...)`.
//!
//! As classes e propriedades vêm de `resources/components.json` (o catálogo do registro;
//! veja `aylis_json::catalog`). Para cada linguagem a geração escreve um arquivo
//! `components` com as classes dos componentes que os forms usam, a classe de cada form e
//! as instâncias; o registro dos forms liga o cliente a elas, e o arquivo de eventos de cada
//! form ganha o `import`/`uses` da instância dele. Ler é um `get` na tela (espera a
//! resposta; no TypeScript, `await`); gravar é um `set`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde_json::Value;

use crate::{Language, Project};

/// O tipo de uma propriedade no código.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Bool,
    Int,
    Float,
    Text,
    /// Uma referência a outro componente: o objeto do componente em cada linguagem (vai pelo id).
    Component,
    /// A cor (o tipo `Color` de cada cliente: RGBA ou um nome do tema).
    Color,
    /// As margens (`margins`, `padding`): o tipo `Bounds` de cada cliente.
    Bounds,
    /// Um enum: o nome do tipo (em [`Model::enums`]).
    Enum(String),
    /// Um conjunto: o nome do tipo do elemento (um enum, em [`Model::enums`]).
    Set(String),
    /// Uma data, uma hora, ou o dia e a hora (no fio, o texto ISO; vazio = nenhum).
    Date,
    Time,
    DateTime,
    /// Uma lista de textos, de números ou de cores (no fio, separados por `;`).
    List,
    NumberList,
    ColorList,
    /// A sombra, a curva de suavização e os pontos (no fio, o texto de sempre).
    Shadow,
    Easing,
    PointList,
    /// Um registro e uma lista de registros (no fio, o JSON; em [`Model::records`]).
    Record(String),
    RecordList(String),
}

/// O que um evento leva, para o handler tipado.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EventKind {
    /// Nada: o handler recebe só quem disparou.
    None,
    Pointer,
    Wheel,
    Gesture,
    Swipe,
    Key,
    DateRange,
    /// Um valor (o texto, um número, uma data, um enum...).
    Value(Kind),
}

/// A assinatura tipada de um handler: a classe de quem dispara (`None`: componentes de classes
/// diferentes ligados ao mesmo handler) e o que o evento leva.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HandlerSig {
    pub sender: Option<String>,
    pub arg: EventKind,
}

/// A assinatura tipada do handler ligado a `events` (a classe e o evento de cada ligação):
/// só quando todos os eventos declaram o que levam e levam o mesmo. Os eventos que perguntam
/// e os não declarados ficam com a assinatura de sempre (`None`).
pub(crate) fn handler_signature(events: &[(String, String)]) -> Option<HandlerSig> {
    if events.is_empty() {
        return None;
    }
    let catalog = catalog();
    let mut arg: Option<EventKind> = None;
    let mut sender: Option<String> = Some(events[0].0.clone());
    for (class, event) in events {
        let kind = catalog.event_kind(class, event)?;
        if arg.as_ref().is_some_and(|a| *a != kind) {
            return None;
        }
        arg = Some(kind);
        if sender.as_deref() != Some(class.as_str()) {
            sender = None;
        }
    }
    Some(HandlerSig { sender, arg: arg? })
}

/// O tipo do que o evento leva, em Rust (`None`: o evento não leva nada).
fn rust_event_ty(arg: &EventKind) -> Option<String> {
    let catalog = catalog();
    Some(match arg {
        EventKind::None => return None,
        EventKind::Pointer => "aylis::prelude::PointerEvent".into(),
        EventKind::Wheel => "aylis::prelude::WheelEvent".into(),
        EventKind::Gesture => "aylis::prelude::GestureEvent".into(),
        EventKind::Swipe => "aylis::prelude::SwipeEvent".into(),
        EventKind::Key => "aylis::prelude::KeyEvent".into(),
        EventKind::DateRange => "aylis::prelude::DateRange".into(),
        // O registro de um evento (o texto com o JSON): a struct do framework, no prelude.
        EventKind::Value(Kind::Record(n)) => format!("aylis::prelude::{n}"),
        EventKind::Value(k) => rust_ty(k, &catalog.enums),
    })
}

/// Os parâmetros do handler tipado em Rust, depois do `&mut self`: `ui`, `sender` (o handle da
/// classe) e, quando o evento leva algo, `value`.
pub(crate) fn rust_handler_params(sig: &HandlerSig) -> String {
    let sender = sig.sender.as_ref().map_or("ComponentId".to_string(), |c| format!("crate::generated::{c}"));
    match rust_event_ty(&sig.arg) {
        Some(t) => format!("ui: &mut Ui, sender: {sender}, value: {t}"),
        None => format!("ui: &mut Ui, sender: {sender}"),
    }
}

/// O catálogo, lido uma vez por processo (a geração pede a assinatura de cada handler).
fn catalog() -> std::sync::Arc<Catalog> {
    static CATALOG: std::sync::Mutex<Option<std::sync::Arc<Catalog>>> = std::sync::Mutex::new(None);
    let mut c = CATALOG.lock().unwrap_or_else(|e| e.into_inner());
    c.get_or_insert_with(|| std::sync::Arc::new(Catalog::load())).clone()
}

#[derive(Clone, Debug)]
struct Prop {
    name: String,
    kind: Kind,
    read_only: bool,
}

/// Um enum do acesso tipado: os valores e, quando existe, o tipo Rust dele no `aylis::prelude`
/// (o Rust usa o mesmo tipo do framework; as outras linguagens ganham um gerado).
#[derive(Clone, Debug, PartialEq, Eq)]
struct EnumDef {
    values: Vec<String>,
    rust: Option<String>,
    /// O tipo Rust do conjunto que usa este enum como elemento (`Anchors` do `Anchor`).
    rust_set: Option<String>,
    /// De onde vêm os valores do projeto (os do catálogo são só os do Aylis): veja
    /// [`project_names`].
    names: Option<ProjectNames>,
}

/// Os enums com nomes do projeto: os valores mudam com os temas e os forms dele.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ProjectNames {
    /// As variantes do tema para a classe (o `style`).
    Style(String),
    /// Os temas (o `theme`).
    Theme,
    /// Os forms (o `form` do Frame, o `item_form` da lista).
    Form,
}

/// Nomes que um enum não pode ter: os tipos que os clientes e o gerado já usam.
const RESERVED_TYPES: [&str; 22] = [
    "Color", "Bounds", "Ref", "Component", "Form", "Handle", "Ui", "PropValue", "ComponentId", "Error", "Result", "Option", "String", "Vec", "Event", "Aylis", "Json",
    "Object", "Enum", "List", "Map", "Set",
];

/// O catálogo: as propriedades de cada classe, na ordem do Object Inspector, e os enums.
pub struct Catalog {
    map: serde_json::Map<String, Value>,
    /// `(classe, propriedade)` (ou `(classe, "método.argumento")`) → o nome do enum.
    enum_of: std::collections::HashMap<(String, String), String>,
    enums: BTreeMap<String, EnumDef>,
    /// Os registros (`$records` do catálogo): o nome e os campos (o tipo de cada um).
    records: BTreeMap<String, Vec<(String, Kind)>>,
}

/// `enum(WindowState:normal|maximized)` → (é conjunto, o tipo Rust, os valores).
fn enum_spec(text: &str) -> Option<(bool, String, Vec<String>)> {
    let (set, rest) = if let Some(r) = text.strip_prefix("enum(") {
        (false, r)
    } else {
        (true, text.strip_prefix("set(")?)
    };
    let inner = rest.strip_suffix(')')?;
    let (ty, values) = inner.split_once(':').unwrap_or(("", inner));
    let values: Vec<String> = values.split('|').filter(|v| !v.is_empty()).map(str::to_string).collect();
    (!values.is_empty()).then(|| (set, ty.to_string(), values))
}

/// Um argumento de método do catálogo (`path:string`, `string`, `orientation:enum(T:a|b)`):
/// o nome (vazio, sem nome) e o tipo.
fn split_arg(arg: &str) -> (&str, &str) {
    if arg.starts_with("enum(") || arg.starts_with("set(") {
        return ("", arg);
    }
    arg.split_once(':').unwrap_or(("", arg))
}

/// O tipo sem o `:ro` do fim (`enum(..):ro` → `enum(..)`, `float:ro` → `float`).
fn split_ro(text: &str) -> (&str, bool) {
    match text.strip_suffix(":ro") {
        Some(t) => (t, true),
        None => (text, false),
    }
}

/// O nome do elemento de um conjunto (`Anchors` → `Anchor`).
fn singular(name: &str) -> String {
    name.strip_suffix('s').filter(|n| !n.is_empty()).unwrap_or(name).to_string()
}

impl Catalog {
    /// O de `resources/components.json` (vazio se não houver).
    pub fn load() -> Catalog {
        let path = crate::codegen::resources_dir().join("components.json");
        let map = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v.as_object().cloned()).unwrap_or_default();
        Catalog::from_map(map)
    }

    fn from_map(map: serde_json::Map<String, Value>) -> Catalog {
        // Os enums de todas as classes: o nome é o do tipo Rust (ou o da propriedade). Dois
        // enums com o mesmo nome e valores diferentes ficam com o nome da classe na frente.
        // `$records` não é classe.
        let classes: BTreeSet<String> = map.iter().filter(|(k, v)| v.is_object() && !k.starts_with('$')).map(|(k, _)| k.clone()).collect();
        let mut enum_of = std::collections::HashMap::new();
        let mut enums: BTreeMap<String, EnumDef> = BTreeMap::new();
        for (class, props) in map.iter().filter(|(k, _)| !k.starts_with('$')).filter_map(|(k, v)| v.as_object().map(|o| (k, o))) {
            for (key, value) in props {
                let Some(text) = value.as_str() else { continue };
                // As propriedades e os argumentos dos métodos (`resize(width:int)`).
                let mut found: Vec<(String, String, &str)> = Vec::new();
                if let Some((name, rest)) = key.split_once('(') {
                    for (i, arg) in rest.strip_suffix(')').unwrap_or(rest).split(',').filter(|a| !a.is_empty()).enumerate() {
                        let (param, kind) = split_arg(arg);
                        let param = if param.is_empty() { format!("a{}", i + 1) } else { param.to_string() };
                        found.push((format!("{name}.{param}"), param, kind));
                    }
                    if let Some((_, ret)) = text.split_once(':') {
                        found.push((format!("{name}()"), name.to_string(), ret));
                    }
                } else {
                    found.push((key.clone(), key.clone(), split_ro(text).0));
                }
                for (slot, base, kind) in found {
                    let Some((set, rust, values)) = enum_spec(kind) else { continue };
                    let rust = (!rust.is_empty()).then_some(rust);
                    let mut name = match (&rust, set) {
                        (Some(r), true) => singular(r),
                        (Some(r), false) => r.clone(),
                        (None, true) => singular(&pascal(&base)),
                        (None, false) => pascal(&base),
                    };
                    if classes.contains(&name) || RESERVED_TYPES.contains(&name.as_str()) {
                        name.push_str("Kind");
                    }
                    let def = EnumDef { values, rust: if set { None } else { rust.clone() }, rust_set: if set { rust.clone() } else { None }, names: None };
                    // O mesmo nome com outros valores: o da classe na frente (sem o tipo Rust, que
                    // ficaria ambíguo).
                    if enums.get(&name).is_some_and(|d| d.values != def.values) {
                        name = format!("{class}{name}");
                        let def = EnumDef { rust: None, rust_set: None, ..def.clone() };
                        while enums.get(&name).is_some_and(|d| d.values != def.values) {
                            name.push('_');
                        }
                        enums.entry(name.clone()).or_insert(def);
                    } else {
                        let entry = enums.entry(name.clone()).or_insert_with(|| def.clone());
                        // O mesmo enum, visto como elemento de um conjunto e sozinho.
                        if entry.rust.is_none() {
                            entry.rust = def.rust.clone();
                        }
                        if entry.rust_set.is_none() {
                            entry.rust_set = def.rust_set.clone();
                        }
                    }
                    enum_of.insert((class.clone(), slot), name);
                }
            }
        }
        // Os nomes do projeto: o `style` de cada classe (um enum por classe), o `theme` e as
        // referências a form, com os valores do Aylis (`$styles`, `$themes`); o gerador junta
        // os do projeto ([`project_names`]). O primeiro valor, vazio, é o padrão.
        let strings = |v: Option<&Value>| -> Vec<String> { v.and_then(Value::as_array).map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let free = |name: String, enums: &BTreeMap<String, EnumDef>| {
            let mut name = name;
            while classes.contains(&name) || RESERVED_TYPES.contains(&name.as_str()) || enums.contains_key(&name) {
                name.push_str("Name");
            }
            name
        };
        let theme_enum = free("ThemeName".into(), &enums);
        let form_enum = free("FormName".into(), &enums);
        let mut uses_theme = false;
        let mut uses_form = false;
        for (class, props) in map.iter().filter(|(k, _)| !k.starts_with('$')).filter_map(|(k, v)| v.as_object().map(|o| (k, o))) {
            for (key, value) in props.iter().filter(|(k, _)| !k.contains('(')) {
                match value.as_str().map(|t| split_ro(t).0) {
                    Some("style") => {
                        let name = free(format!("{class}Style"), &enums);
                        let values = std::iter::once(String::new()).chain(strings(map.get("$styles").and_then(|s| s.get(class)))).collect();
                        enums.insert(name.clone(), EnumDef { values, rust: None, rust_set: None, names: Some(ProjectNames::Style(class.clone())) });
                        enum_of.insert((class.clone(), key.clone()), name);
                    }
                    Some("theme") => {
                        uses_theme = true;
                        enum_of.insert((class.clone(), key.clone()), theme_enum.clone());
                    }
                    Some("form") => {
                        uses_form = true;
                        enum_of.insert((class.clone(), key.clone()), form_enum.clone());
                    }
                    _ => {}
                }
            }
        }
        if uses_theme {
            let values = std::iter::once(String::new()).chain(strings(map.get("$themes"))).collect();
            enums.insert(theme_enum, EnumDef { values, rust: None, rust_set: None, names: Some(ProjectNames::Theme) });
        }
        if uses_form {
            enums.insert(form_enum, EnumDef { values: vec![String::new()], rust: None, rust_set: None, names: Some(ProjectNames::Form) });
        }
        // Os registros (o resultado dos métodos com campos: `Screen.monitors`).
        let mut catalog = Catalog { map, enum_of, enums, records: BTreeMap::new() };
        let records: Vec<(String, Vec<(String, String)>)> = catalog
            .map
            .get("$records")
            .and_then(Value::as_object)
            .map(|o| {
                o.iter()
                    .map(|(name, fields)| {
                        let fields = fields.as_object().map(|f| f.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
                        (name.clone(), fields)
                    })
                    .collect()
            })
            .unwrap_or_default();
        for (name, fields) in records {
            let slot_class = format!("$record:{name}");
            for (f, k) in &fields {
                let Some((set, rust, values)) = enum_spec(k) else { continue };
                let rust = (!rust.is_empty()).then_some(rust);
                let mut enum_name = match (&rust, set) {
                    (Some(r), true) => singular(r),
                    (Some(r), false) => r.clone(),
                    (None, true) => singular(&format!("{name}{}", pascal(f))),
                    (None, false) => format!("{name}{}", pascal(f)),
                };
                let def = EnumDef { values, rust: if set { None } else { rust.clone() }, rust_set: if set { rust.clone() } else { None }, names: None };
                if catalog.enums.get(&enum_name).is_some_and(|d| d.values != def.values) {
                    enum_name = format!("{name}{enum_name}");
                }
                catalog.enums.entry(enum_name.clone()).or_insert(def);
                catalog.enum_of.insert((slot_class.clone(), f.clone()), enum_name);
            }
            let kinds = fields.into_iter().map(|(f, k)| {
                let kind = catalog.kind(&slot_class, &f, &k);
                (f, kind)
            }).collect();
            catalog.records.insert(name, kinds);
        }
        catalog
    }

    /// O tipo de uma propriedade (ou argumento) no código.
    fn kind(&self, class: &str, slot: &str, text: &str) -> Kind {
        match text {
            "bool" => Kind::Bool,
            "int" => Kind::Int,
            "float" => Kind::Float,
            "color" => Kind::Color,
            "bounds" => Kind::Bounds,
            "component" => Kind::Component,
            "date" => Kind::Date,
            "time" => Kind::Time,
            "datetime" => Kind::DateTime,
            "list" => Kind::List,
            "numbers" => Kind::NumberList,
            "colors" => Kind::ColorList,
            "shadow" => Kind::Shadow,
            "easing" => Kind::Easing,
            "points" => Kind::PointList,
            t if t.starts_with("record(") => Kind::Record(t["record(".len()..].trim_end_matches(')').to_string()),
            t if t.starts_with("records(") => Kind::RecordList(t["records(".len()..].trim_end_matches(')').to_string()),
            "style" | "theme" | "form" => self.enum_of.get(&(class.to_string(), slot.to_string())).map_or(Kind::Text, |n| Kind::Enum(n.clone())),
            t if t.starts_with("enum(") || t.starts_with("set(") => match self.enum_of.get(&(class.to_string(), slot.to_string())) {
                Some(name) if t.starts_with("set(") => Kind::Set(name.clone()),
                Some(name) => Kind::Enum(name.clone()),
                None => Kind::Text,
            },
            _ => Kind::Text,
        }
    }

    /// O que o evento `event` da classe leva, quando o componente declara (`"event:float"`).
    fn event_kind(&self, class: &str, event: &str) -> Option<EventKind> {
        let text = self.map.get(class)?.get(event)?.as_str()?.strip_prefix("event:")?;
        Some(match text {
            "none" => EventKind::None,
            "pointer" => EventKind::Pointer,
            "wheel" => EventKind::Wheel,
            "gesture" => EventKind::Gesture,
            "swipe" => EventKind::Swipe,
            "key" => EventKind::Key,
            "daterange" => EventKind::DateRange,
            "string" => EventKind::Value(Kind::Text),
            other => match self.kind(class, event, other) {
                // Os tipos que o `EventValue` do Rust sabe tirar do evento.
                k @ (Kind::Bool | Kind::Int | Kind::Float | Kind::Color | Kind::Date | Kind::Time | Kind::DateTime | Kind::List | Kind::Component | Kind::Enum(_) | Kind::Shadow | Kind::Easing | Kind::Record(_)) => EventKind::Value(k),
                _ => return None,
            },
        })
    }

    /// A classe está no catálogo (sem montar a lista das propriedades: o filtro de cada componente
    /// de cada form montava, 50 mil vezes no ERP de 1009 forms).
    fn has(&self, class: &str) -> bool {
        self.map.get(class).is_some_and(Value::is_object)
    }

    /// A classe do catálogo mais parecida com `class`: a que está contida no nome
    /// (`ImageView` → `Image`) ou a de menos letras trocadas (até 3).
    fn closest(&self, class: &str) -> Option<String> {
        let names: Vec<&String> = self.map.iter().filter(|(k, v)| v.is_object() && !k.starts_with('$')).map(|(k, _)| k).collect();
        let lower = class.to_lowercase();
        if let Some(inside) = names.iter().filter(|n| lower.contains(&n.to_lowercase()) || n.to_lowercase().contains(&lower)).max_by_key(|n| n.len()) {
            return Some((*inside).clone());
        }
        let distance = |a: &str, b: &str| {
            let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
            let mut row: Vec<usize> = (0..=b.len()).collect();
            for i in 1..=a.len() {
                let mut prev = row[0];
                row[0] = i;
                for j in 1..=b.len() {
                    let cur = row[j];
                    row[j] = (row[j] + 1).min(row[j - 1] + 1).min(prev + usize::from(a[i - 1] != b[j - 1]));
                    prev = cur;
                }
            }
            row[b.len()]
        };
        names.into_iter().map(|n| (distance(&lower, &n.to_lowercase()), n)).filter(|(d, _)| *d <= 3).min_by_key(|(d, _)| *d).map(|(_, n)| n.clone())
    }

    /// Muda quando o catálogo muda (os pedaços gerados com outro catálogo não valem).
    fn stamp(&self) -> String {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        serde_json::to_string(&self.map).unwrap_or_default().hash(&mut h);
        format!("{:016x}", h.finish())
    }

    fn props(&self, class: &str) -> Option<Vec<Prop>> {
        let map = self.map.get(class)?.as_object()?;
        Some(
            map.iter()
                .filter(|(name, _)| !name.contains('('))
                .filter_map(|(name, kind)| {
                    let text = kind.as_str()?;
                    // Os eventos-pergunta (`"on_close_query": "question:can_close"`) não são propriedades.
                    if text.starts_with("question") || text == "coalesce" {
                        return None;
                    }
                    let (kind, read_only) = split_ro(text);
                    Some(Prop { name: name.clone(), kind: self.kind(class, name, kind), read_only })
                })
                .collect(),
        )
    }

    /// Os métodos (`"resize(width:int,height:int)": "invoke:bool"`; o argumento sem nome
    /// vem só com o tipo, `"move_to_monitor(int)"`).
    fn methods(&self, class: &str) -> Vec<Method> {
        let Some(map) = self.map.get(class).and_then(Value::as_object) else { return Vec::new() };
        map.iter()
            .filter_map(|(key, value)| {
                let (name, rest) = key.split_once('(')?;
                let (mut args, mut params) = (Vec::new(), Vec::new());
                for (i, arg) in rest.strip_suffix(')').unwrap_or(rest).split(',').filter(|a| !a.is_empty()).enumerate() {
                    let (param, kind) = split_arg(arg);
                    let slot = format!("{name}.{}", if param.is_empty() { format!("a{}", i + 1) } else { param.to_string() });
                    args.push(self.kind(class, &slot, kind));
                    // Só um identificador simples vira nome (o resto, o `a1` de sempre).
                    let ok = !param.is_empty() && param.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') && param.starts_with(|c: char| c.is_ascii_lowercase());
                    params.push(if ok { param.to_string() } else { format!("a{}", i + 1) });
                }
                let (via, ret) = value.as_str()?.split_once(':')?;
                let returns = (ret != "void").then(|| self.kind(class, &format!("{name}()"), ret));
                Some(Method { name: name.to_string(), args, params, via: via.to_string(), returns })
            })
            .collect()
    }
}

/// Um método: pelo `invoke` do protocolo ou por um comando dele (`media`, `execute`,
/// `show`, `close`).
#[derive(Clone, Debug)]
struct Method {
    name: String,
    args: Vec<Kind>,
    /// O nome de cada argumento (do catálogo; sem nome, `a1`, `a2`...).
    params: Vec<String>,
    via: String,
    returns: Option<Kind>,
}

impl Method {
    /// Os campos do pedido além do `target`: `(chave, valor)` com os argumentos pelo nome
    /// já no jeito da linguagem (`args` vem pronto em `list`).
    fn fields(&self, list: &str) -> Vec<(String, String)> {
        match self.via.as_str() {
            // Devolve um componente: com o id e o nome (`"ref": true`), para montar o objeto.
            "invoke" if self.returns == Some(Kind::Component) => {
                vec![("method".into(), format!("\"{}\"", self.name)), ("args".into(), list.to_string()), ("ref".into(), "true".into())]
            }
            "invoke" => vec![("method".into(), format!("\"{}\"", self.name)), ("args".into(), list.to_string())],
            "media" => {
                let mut f = vec![("op".into(), format!("\"{}\"", self.name))];
                if let Some(first) = self.params.first() {
                    f.push(("seconds".into(), first.clone()));
                }
                f
            }
            _ => Vec::new(),
        }
    }

    /// O comando do protocolo (`invoke`, `media`, `execute`...).
    fn command(&self) -> &str {
        &self.via
    }

    /// Espera a resposta da tela (devolve algo, ou é `close`)?
    fn waits(&self) -> bool {
        self.returns.is_some()
    }

    fn arg_names(&self) -> Vec<String> {
        self.params.clone()
    }
}

/// Um form: o nome e os componentes (nome e classe), juntando as variantes por plataforma.
#[derive(Clone)]
struct FormModel {
    name: String,
    components: Vec<(String, String)>,
    /// O módulo do arquivo do form (`forms/clientes.json` → `clientes`).
    module: String,
    /// Os componentes que só existem em algumas variantes (no código, opcionais).
    optional: BTreeSet<String>,
}

/// O modelo de um form, lido do disco.
fn form_model(project: &Project, entry: &crate::FormEntry, files: &mut crate::FormFiles) -> Result<FormModel, String> {
    {
        let mut model = FormModel { name: entry.name.clone(), components: Vec::new(), module: crate::codegen::module_name(&entry.file), optional: BTreeSet::new() };
        let mut each: Vec<BTreeSet<String>> = Vec::new();
        for (_, layout) in project.read_entry_variants(entry, files)? {
            let doc: Value = serde_json::from_str(layout.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", entry.file))?;
            let root = doc.get("root").unwrap_or(&doc);
            if let Some(name) = root.get("name").and_then(Value::as_str) {
                model.name = name.to_string();
            }
            let mut mine = Vec::new();
            for child in root.get("children").and_then(Value::as_array).into_iter().flatten() {
                walk(child, &mut mine);
            }
            each.push(mine.iter().map(|(n, _)| n.clone()).collect());
            for c in mine {
                if !model.components.iter().any(|(n, _)| *n == c.0) {
                    model.components.push(c);
                }
            }
        }
        model.optional = model.components.iter().map(|(n, _)| n.clone()).filter(|n| each.iter().any(|v| !v.contains(n))).collect();
        Ok(model)
    }
}

fn walk(node: &Value, out: &mut Vec<(String, String)>) {
    let name = node.get("name").and_then(Value::as_str).unwrap_or_default();
    let class = node.get("class").and_then(Value::as_str).unwrap_or_default();
    let base = class.split('.').next().unwrap_or(class);
    if !name.is_empty() && !base.is_empty() && crate::is_identifier(name) && !out.iter().any(|(n, _)| n == name) {
        out.push((name.to_string(), base.to_string()));
    }
    for child in node.get("children").and_then(Value::as_array).into_iter().flatten() {
        walk(child, out);
    }
}

/// O arquivo gerado de cada linguagem (relativo à pasta dela).
pub fn file(language: Language) -> &'static str {
    match language {
        Language::Rust => "generated/components.rs",
        Language::Python => "src/forms/components.py",
        Language::TypeScript => "src/forms/components.ts",
        Language::CSharp => "src/Forms/Components.cs",
        Language::Java => "src/forms/Components.java",
        Language::Go => "src/forms/components.go",
        Language::Cpp => "src/forms/components.hpp",
        Language::Pascal => "src/forms/ayliscomponents.pas",
    }
}

/// `font_size` → `FontSize`.
fn pascal(name: &str) -> String {
    name.split('_').filter(|p| !p.is_empty()).map(|p| p[..1].to_uppercase() + &p[1..]).collect()
}

/// `PaintCanvas` → `paint_canvas`.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// Os nomes dos valores de um enum como identificadores no estilo de `style`
/// (`full_screen` → `FullScreen`), sem repetir.
fn variants(values: &[String], style: fn(&str) -> String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for v in values {
        let mut name = style(v);
        while out.contains(&name) {
            name.push('_');
        }
        out.push(name);
    }
    out
}

/// As palavras de um valor (`top-left`, `full_screen`, `2x` → `top`, `left` / `full`, `screen` / `2x`).
/// O camelCase também separa (`frmOutro` → `frm`, `Outro`: o nome de um form vira `FRM_OUTRO`).
fn words(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in value.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()) {
        let mut word = String::new();
        let mut prev_lower = false;
        for c in chunk.chars() {
            if c.is_ascii_uppercase() && prev_lower && !word.is_empty() {
                out.push(std::mem::take(&mut word));
            }
            prev_lower = c.is_ascii_lowercase();
            word.push(c);
        }
        out.push(word);
    }
    out
}

/// `full_screen` → `FullScreen` (`2x` → `N2x`; vazio, o padrão do `style` e do `theme` → `Default`).
fn variant_pascal(value: &str) -> String {
    let name: String = words(value).iter().map(|w| w[..1].to_uppercase() + &w[1..]).collect();
    match name.chars().next() {
        None => "Default".into(),
        Some(c) if c.is_ascii_digit() => format!("N{name}"),
        _ => name,
    }
}

/// `full_screen` → `FULL_SCREEN` (`2x` → `V_2X`; vazio → `DEFAULT`).
fn variant_upper(value: &str) -> String {
    let name = words(value).iter().map(|w| w.to_uppercase()).collect::<Vec<_>>().join("_");
    match name.chars().next() {
        None => "DEFAULT".into(),
        Some(c) if c.is_ascii_digit() => format!("V_{name}"),
        _ => name,
    }
}

/// O tipo de um valor em Rust (os enums do framework pelo `aylis::prelude`).
fn rust_ty(kind: &Kind, enums: &BTreeMap<String, EnumDef>) -> String {
    match kind {
        Kind::Bool => "bool".into(),
        Kind::Int => "i64".into(),
        Kind::Float => "f64".into(),
        Kind::Text => "String".into(),
        Kind::Color => "aylis::prelude::Color".into(),
        Kind::Bounds => "aylis::prelude::Bounds".into(),
        Kind::Component => "Option<ComponentId>".into(),
        Kind::Enum(n) => match enums.get(n).and_then(|d| d.rust.as_ref()) {
            Some(r) => format!("aylis::prelude::{r}"),
            None => n.clone(),
        },
        Kind::Set(n) => match enums.get(n).and_then(|d| d.rust_set.as_ref()) {
            Some(r) => format!("aylis::prelude::{r}"),
            None => format!("Vec<{n}>"),
        },
        Kind::Date => "Option<aylis::prelude::Date>".into(),
        Kind::Time => "Option<aylis::prelude::Time>".into(),
        Kind::DateTime => "Option<aylis::prelude::DateTime>".into(),
        Kind::List => "Vec<String>".into(),
        Kind::NumberList => "Vec<f64>".into(),
        Kind::ColorList => "Vec<aylis::prelude::Color>".into(),
        Kind::Shadow => "Option<aylis::prelude::Shadow>".into(),
        Kind::Easing => "aylis::prelude::Easing".into(),
        Kind::PointList => "Vec<aylis::prelude::PointF>".into(),
        Kind::Record(n) => format!("Option<{n}>"),
        Kind::RecordList(n) => format!("Vec<{n}>"),
    }
}

/// Um conjunto que tem o tipo Rust do framework (`Anchors`)?
fn rust_set_type(kind: &Kind, enums: &BTreeMap<String, EnumDef>) -> bool {
    matches!(kind, Kind::Set(n) if enums.get(n).is_some_and(|d| d.rust_set.is_some()))
}

/// O tipo de um argumento de método em Rust.
fn rust_arg(kind: &Kind, enums: &BTreeMap<String, EnumDef>) -> String {
    match kind {
        Kind::Text => "&str".into(),
        Kind::Component => "impl Handle".into(),
        Kind::Set(n) if !rust_set_type(kind, enums) => format!("&[{n}]"),
        Kind::List => "&[&str]".into(),
        Kind::NumberList => "&[f64]".into(),
        Kind::ColorList => "&[aylis::prelude::Color]".into(),
        Kind::PointList => "&[aylis::prelude::PointF]".into(),
        k => rust_ty(k, enums),
    }
}

/// O argumento `a` (de tipo `kind`) como `PropValue`, em Rust.
fn rust_value(a: &str, kind: &Kind, enums: &BTreeMap<String, EnumDef>) -> String {
    match kind {
        Kind::Text => format!("PropValue::String({a}.to_string())"),
        Kind::Component => format!("PropValue::Component(Some({a}.id()))"),
        Kind::Color => format!("PropValue::Color({a})"),
        Kind::Bounds => format!("PropValue::Bounds({a})"),
        Kind::Enum(_) => format!("PropValue::enum_value({a})"),
        Kind::Set(_) if rust_set_type(kind, enums) => format!("PropValue::from({a})"),
        Kind::Set(_) => format!("handles::set_value({a})"),
        Kind::Date => format!("handles::date_value({a})"),
        Kind::Time => format!("handles::time_value({a})"),
        Kind::DateTime => format!("handles::datetime_value({a})"),
        Kind::List => format!("handles::list_value({a})"),
        Kind::NumberList => format!("handles::numbers_value({a})"),
        Kind::ColorList => format!("handles::colors_value({a})"),
        Kind::Shadow => format!("handles::shadow_value({a})"),
        Kind::Easing => format!("handles::easing_value({a})"),
        Kind::PointList => format!("handles::points_value({a})"),
        Kind::RecordList(n) => format!("PropValue::String({n}::list_text(&{a}))"),
        Kind::Record(_) => format!("PropValue::String({a}.as_ref().map(|r| r.to_json().to_string()).unwrap_or_default())"),
        _ => format!("{a}.into()"),
    }
}

/// O `PropValue` `v` como o tipo `kind`, em Rust (o padrão quando não serve).
fn rust_from(v: &str, kind: &Kind, enums: &BTreeMap<String, EnumDef>) -> String {
    let ty = rust_ty(kind, enums);
    match kind {
        Kind::Bool => format!("{v}.to_bool().unwrap_or_default()"),
        Kind::Int => format!("{v}.to_i64().unwrap_or_default()"),
        Kind::Float => format!("{v}.to_f64().unwrap_or_default()"),
        Kind::Text => format!("handles::text(ui, Some({v}))"),
        Kind::Color => format!("{v}.to_color().unwrap_or_default()"),
        Kind::Bounds => format!("{v}.to_bounds().unwrap_or_default()"),
        Kind::Component => format!("{v}.to_component().ok().flatten()"),
        Kind::Enum(_) => format!("{v}.to_enum::<{ty}>().ok().or_else(|| <{ty} as aylis::prelude::PropEnum>::from_index(0)).expect(\"enum\")"),
        Kind::Set(_) if rust_set_type(kind, enums) => format!("{v}.to_anchors().unwrap_or_default()"),
        Kind::Set(n) => format!("match {v} {{ PropValue::Set(names) => names.iter().filter_map(|n| <{n} as aylis::prelude::PropEnum>::from_name(n)).collect(), _ => Vec::new() }}"),
        Kind::Date => format!("{v}.to_str().ok().and_then(aylis::prelude::Date::parse_iso)"),
        Kind::Time => format!("{v}.to_str().ok().and_then(aylis::prelude::Time::parse_iso)"),
        Kind::DateTime => format!("{v}.to_str().ok().and_then(aylis::prelude::DateTime::parse_iso)"),
        Kind::List => format!("{v}.to_str().unwrap_or_default().split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()"),
        Kind::NumberList => format!("{v}.to_str().unwrap_or_default().split(';').filter_map(|s| s.trim().parse().ok()).collect()"),
        Kind::ColorList => format!("{v}.to_str().unwrap_or_default().split(';').filter_map(|s| aylis::prelude::Color::from_hex(s.trim())).collect()"),
        Kind::Shadow => format!("{v}.to_str().ok().and_then(aylis::prelude::Shadow::parse)"),
        Kind::Easing => format!("{v}.to_str().ok().and_then(aylis::prelude::Easing::parse).unwrap_or_default()"),
        Kind::PointList => format!("{v}.to_str().unwrap_or_default().split_whitespace().filter_map(|p| {{ let (x, y) = p.split_once(',')?; Some(aylis::prelude::PointF::new(x.parse().ok()?, y.parse().ok()?)) }}).collect()"),
        Kind::Record(n) => format!("{v}.to_str().ok().and_then(|t| aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(t).ok()).filter(|j| !j.is_null()).map(|j| {n}::from_json(&j))"),
        Kind::RecordList(n) => format!("{v}.to_str().ok().and_then(|t| aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(t).ok()).and_then(|j| j.as_array().map(|a| a.iter().map({n}::from_json).collect())).unwrap_or_default()"),
    }
}

/// Os enums do acesso tipado de cada linguagem, refeitos ao juntar o arquivo.
fn types(language: Language, enums: &BTreeMap<String, EnumDef>, records: &BTreeMap<String, Vec<(String, Kind)>>) -> String {
    let enums_text = match language {
        Language::Rust => rust_types(enums),
        Language::Python => python_types(enums),
        Language::TypeScript => typescript_types(enums),
        Language::CSharp => csharp_types(enums),
        Language::Java => java_types(enums),
        Language::Go => go_types(enums),
        Language::Cpp => cpp_types(enums),
        Language::Pascal => pascal_types(enums),
    };
    let order = record_order(records);
    if language == Language::Pascal {
        // Os registros nos três pedaços do Pascal: os tipos, as declarações e os corpos.
        let (head, decls, body) = pascal_records(records, &order);
        let mut parts = enums_text.split(TYPES_SEP);
        let (h, d, b) = (parts.next().unwrap_or_default(), parts.next().unwrap_or_default(), parts.next().unwrap_or_default());
        return format!("{h}{head}{TYPES_SEP}{d}{decls}{TYPES_SEP}{b}{body}");
    }
    let records_text = match language {
        Language::Rust => rust_records(records, &order, enums),
        Language::Python => python_records(records, &order),
        Language::TypeScript => typescript_records(records, &order),
        Language::CSharp => csharp_records(records, &order),
        Language::Java => java_records(records, &order),
        Language::Go => go_records(records, &order),
        Language::Cpp => cpp_records(records, &order),
        Language::Pascal => String::new(),
    };
    format!("{enums_text}{records_text}")
}

/// Os enums sem tipo no framework (os outros vêm do `aylis::prelude`), com o `PropEnum`.
fn rust_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums.iter().filter(|(_, d)| d.rust.is_none() && d.rust_set.is_none()) {
        let vs = variants(&def.values, variant_pascal);
        s.push_str(&format!("\n/// Os valores de um enum do acesso tipado.\n#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]\npub enum {name} {{\n"));
        for (i, v) in vs.iter().enumerate() {
            s.push_str(&format!("    {}{v},\n", if i == 0 { "#[default]\n    " } else { "" }));
        }
        let names: Vec<String> = def.values.iter().map(|v| format!("\"{v}\"")).collect();
        let all: Vec<String> = vs.iter().map(|v| format!("Self::{v}")).collect();
        s.push_str(&format!(
            "}}\n\nimpl aylis::prelude::PropEnum for {name} {{\n    const NAMES: &'static [&'static str] = &[{}];\n    fn from_index(index: usize) -> Option<Self> {{\n        [{}].get(index).copied()\n    }}\n    fn index(self) -> usize {{\n        self as usize\n    }}\n}}\n",
            names.join(", "),
            all.join(", ")
        ));
    }
    s
}

/// `function Resize(A1: Int64; A2: Int64): Boolean` (com `T<Classe>.` na implementação).
fn pascal_method_head(mth: &Method, class: Option<&str>) -> String {
    let params: Vec<String> = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| format!("{}: {}", pascal_arg(a), pascal_type(k))).collect();
    let params = if params.is_empty() { String::new() } else { format!("({})", params.join("; ")) };
    let name = match class {
        Some(c) => format!("{c}.{}", pascal_ident(&pascal(&mth.name))),
        None => pascal_ident(&pascal(&mth.name)),
    };
    match &mth.returns {
        Some(k) => format!("function {name}{params}: {}", pascal_type(k)),
        None => format!("procedure {name}{params}"),
    }
}

/// O argumento em Pascal (`path` → `APath`; o sem nome, `a1` → `A1`): com o
/// `A` na frente, não bate com uma propriedade da classe (o `Width` do `resize`), o que o
/// Free Pascal recusa.
fn pascal_arg(name: &str) -> String {
    if name.strip_prefix('a').is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())) {
        name.to_uppercase()
    } else {
        format!("A{}", pascal(name))
    }
}

/// `font_size` → `fontSize`.
fn camel(name: &str) -> String {
    let p = pascal(name);
    p[..1].to_lowercase() + &p[1..]
}

const RUST_KEYWORDS: [&str; 38] = [
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "static", "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "box", "yield", "try", "macro",
];
const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for",
    "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
];
const CPP_KEYWORDS: [&str; 44] = [
    "and", "auto", "bool", "break", "case", "catch", "char", "class", "const", "continue", "default", "delete", "do", "double", "else", "enum", "explicit",
    "export", "extern", "false", "float", "for", "friend", "goto", "if", "inline", "int", "long", "namespace", "new", "not", "operator", "or", "private",
    "protected", "public", "register", "return", "short", "signed", "sizeof", "static", "struct", "switch",
];
const CPP_KEYWORDS_2: [&str; 14] = ["template", "this", "throw", "true", "try", "typedef", "typename", "union", "unsigned", "using", "virtual", "void", "volatile", "while"];
const PASCAL_KEYWORDS: [&str; 46] = [
    "and", "array", "as", "asm", "begin", "case", "class", "const", "constructor", "destructor", "div", "do", "downto", "else", "end", "except", "file",
    "finally", "for", "function", "goto", "if", "implementation", "in", "inherited", "inline", "interface", "is", "label", "mod", "nil", "not", "object",
    "of", "on", "or", "packed", "procedure", "program", "property", "raise", "record", "repeat", "set", "string", "type",
];
const PASCAL_KEYWORDS_2: [&str; 10] = ["then", "to", "try", "unit", "until", "uses", "var", "while", "with", "xor"];

fn rust_ident(name: &str) -> String {
    match name {
        "self" | "Self" | "super" | "crate" => format!("{name}_"),
        n if RUST_KEYWORDS.contains(&n) => format!("r#{n}"),
        n => n.to_string(),
    }
}

fn python_ident(name: &str) -> String {
    // `property` também: no corpo da classe, um membro com esse nome esconde o `@property` dos
    // que vêm depois (o `FloatAnimation.property`).
    if PYTHON_KEYWORDS.contains(&name) || name == "property" { format!("{name}_") } else { name.to_string() }
}

fn cpp_ident(name: &str) -> String {
    if CPP_KEYWORDS.contains(&name) || CPP_KEYWORDS_2.contains(&name) { format!("{name}_") } else { name.to_string() }
}

fn pascal_ident(name: &str) -> String {
    let lower = name.to_lowercase();
    if PASCAL_KEYWORDS.contains(&lower.as_str()) || PASCAL_KEYWORDS_2.contains(&lower.as_str()) { format!("&{name}") } else { name.to_string() }
}

/// O que cada linguagem precisa saber de um projeto: os forms e as classes usadas.
struct Model {
    forms: Vec<FormModel>,
    /// As classes usadas (e `Form`), com as propriedades, em ordem.
    classes: Vec<(String, Vec<Prop>)>,
    /// Os métodos de cada classe usada.
    methods: std::collections::BTreeMap<String, Vec<Method>>,
    /// Os enums que as classes usadas usam (nas propriedades e nos métodos).
    enums: BTreeMap<String, EnumDef>,
    /// Os registros que os métodos das classes usadas devolvem (e os de dentro deles).
    records: BTreeMap<String, Vec<(String, Kind)>>,
}

impl Model {
    fn methods(&self, class: &str) -> &[Method] {
        self.methods.get(class).map_or(&[], Vec::as_slice)
    }

    /// As classes dos componentes de um form, sem repetir (o `SenderAs<Classe>`).
    fn classes_of(form: &FormModel) -> Vec<&str> {
        let set: BTreeSet<&str> = form.components.iter().map(|(_, c)| c.as_str()).collect();
        set.into_iter().collect()
    }
}

/// O nome do tipo de um form (`frmCalculadora` → `FrmCalculadoraComponents`).
fn form_type(form: &str) -> String {
    format!("{}Components", pascal(form))
}

/// Os componentes do form que não colidem com as propriedades do próprio form.
fn fields<'a>(form: &'a FormModel, form_props: &[Prop], same: impl Fn(&str, &str) -> bool) -> Vec<&'a (String, String)> {
    form.components.iter().filter(|(n, _)| !form_props.iter().any(|p| same(&p.name, n))).collect()
}

/// O modelo com os forms dados e as classes que eles usam (e `Form`).
/// Os componentes cuja classe não é do Aylis nem de um pacote usado, mas é parecida com uma do
/// Aylis: quase sempre um nome errado (`"class": "ImageView"` no lugar de `Image`), que o gerador
/// deixa de fora do acesso tipado. Uma classe do próprio app (registrada por ele) sem nenhuma
/// parecida não entra. As mensagens vão no "Gerar código" (o relatório do Tabernáculo, G02).
pub fn suspicious_classes(project: &Project) -> Vec<String> {
    let catalog = Catalog::load();
    let packages: BTreeSet<String> = crate::packages::used(&crate::Files::Dir(project.dir())).into_iter().map(|m| m.component).filter(|c| !c.is_empty()).collect();
    let mut files = crate::FormFiles::default();
    let mut out = Vec::new();
    for entry in &project.forms {
        let Ok(form) = form_model(project, entry, &mut files) else { continue };
        for (name, class) in form.components.iter().filter(|(_, c)| !catalog.has(c) && !packages.contains(c)) {
            if let Some(near) = catalog.closest(class) {
                out.push(format!("{}.{name}: a classe '{class}' não é do Aylis (você quis dizer '{near}'?)", form.name));
            }
        }
    }
    out
}

fn model_of(catalog: &Catalog, mut forms: Vec<FormModel>) -> Model {
    for f in &mut forms {
        f.components.retain(|(_, class)| catalog.has(class));
    }
    let mut used: BTreeSet<String> = forms.iter().flat_map(|f| f.components.iter().map(|(_, c)| c.clone())).collect();
    used.insert("Form".into());
    model_with(catalog, forms, used)
}

fn model_with(catalog: &Catalog, forms: Vec<FormModel>, classes: BTreeSet<String>) -> Model {
    let methods: BTreeMap<String, Vec<Method>> = classes.iter().map(|c| (c.clone(), catalog.methods(c))).collect();
    let classes: Vec<(String, Vec<Prop>)> = classes.into_iter().filter_map(|c| catalog.props(&c).map(|p| (c, p))).collect();
    let kinds = classes.iter().flat_map(|(_, p)| p.iter().map(|p| &p.kind)).chain(methods.values().flatten().flat_map(|m| m.args.iter().chain(m.returns.as_ref())));
    let kinds: Vec<&Kind> = kinds.collect();
    let enums: BTreeMap<String, EnumDef> = kinds
        .iter()
        .filter_map(|k| match k {
            Kind::Enum(n) | Kind::Set(n) => catalog.enums.get(n).map(|d| (n.clone(), d.clone())),
            _ => None,
        })
        .collect();
    // Os registros, com os de dentro deles (a lista de modos do monitor).
    let mut records = BTreeMap::new();
    let mut pending: Vec<String> = kinds.iter().filter_map(|k| if let Kind::Record(n) | Kind::RecordList(n) = k { Some(n.clone()) } else { None }).collect();
    while let Some(name) = pending.pop() {
        if records.contains_key(&name) {
            continue;
        }
        let Some(fields) = catalog.records.get(&name) else { continue };
        pending.extend(fields.iter().filter_map(|(_, k)| if let Kind::Record(n) | Kind::RecordList(n) = k { Some(n.clone()) } else { None }));
        records.insert(name, fields.clone());
    }
    let mut enums = enums;
    for k in records.values().flatten().map(|(_, k)| k) {
        if let Kind::Enum(n) | Kind::Set(n) = k {
            if let Some(d) = catalog.enums.get(n) {
                enums.insert(n.clone(), d.clone());
            }
        }
    }
    Model { forms, classes, methods, enums, records }
}

// ---------------------------------------------------------------------------
// Um pedaço por form e por classe

/// O que um laço do gerador percorre: as classes ou os forms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Class,
    Form,
    /// Os enums das classes usadas (refeitos inteiros ao juntar o arquivo; veja [`types`]).
    Types,
}

enum Seg {
    Text(String),
    Items(Part, Vec<(String, String)>),
}

/// O texto de um gerador com o pedaço de cada classe e de cada form marcado (os laços chamam
/// `begin`, `item` e `end`): juntar tudo dá o arquivo de sempre, e o pedaço de um form sai sem
/// os outros. É o que deixa regerar só o form que mudou.
struct Out {
    segs: Vec<Seg>,
    open: bool,
}

impl From<String> for Out {
    fn from(text: String) -> Self {
        Out { segs: vec![Seg::Text(text)], open: false }
    }
}

impl Out {
    fn push_str(&mut self, text: &str) {
        if self.open {
            if let Some(Seg::Items(_, items)) = self.segs.last_mut() {
                if let Some((_, item)) = items.last_mut() {
                    item.push_str(text);
                    return;
                }
            }
        }
        match self.segs.last_mut() {
            Some(Seg::Text(s)) => s.push_str(text),
            _ => self.segs.push(Seg::Text(text.to_string())),
        }
    }

    fn begin(&mut self, part: Part) {
        self.segs.push(Seg::Items(part, Vec::new()));
        self.open = true;
    }

    fn item(&mut self, name: &str) {
        if let Some(Seg::Items(_, items)) = self.segs.last_mut() {
            items.push((name.to_string(), String::new()));
        }
    }

    fn end(&mut self) {
        self.open = false;
    }

    /// Os pedaços de `name` em cada laço de `part`, na ordem (vazio onde ele não aparece).
    fn pieces(&self, part: Part, name: &str) -> Vec<String> {
        self.segs
            .iter()
            .filter_map(|s| match s {
                Seg::Items(p, items) if *p == part => Some(items.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap_or_default()),
                _ => None,
            })
            .collect()
    }
}

fn generator(language: Language, m: &Model) -> Out {
    match language {
        Language::Rust => rust(m),
        Language::Python => python(m),
        Language::TypeScript => typescript(m),
        Language::CSharp => csharp(m),
        Language::Java => java(m),
        Language::Go => go(m),
        Language::Cpp => cpp(m),
        Language::Pascal => pascal_unit(m),
    }
}

/// A versão do gerador: mudar o que ele escreve (ou o catálogo) faz regerar todos os pedaços.
/// 8: as classes de cada form (`classes.json`), para tirar as que nenhum usa mais.
/// 9: a feature `text-shaping` no `Cargo.toml` só com fontes em `assets/fonts`.
/// 10: o `generated/mod.rs` sem o `use form_handler` (G07) e o `main.rs` com o `_` (G06).
/// 11: o `create_in` transparente sem `fill` próprio, como o Frame (G05).
/// 12: o `on_back` do Pascal com `var Handled: Boolean` (14/11).
/// 13: a `orientation` do Form e o `orientation`/`set_orientation` do Screen no catálogo (14/11).
/// 14: os enums, os conjuntos, a cor, as margens e as referências tipados em todas as
/// linguagens (18/11).
/// 15: as datas, as horas e as listas separadas por `;` tipadas (18/11).
/// 16: os handlers tipados (quem disparou e o valor do evento no tipo) e o `FromSender` (19/11).
/// 17: a sombra, a curva e os pontos tipados (19/11).
/// 18: os enums dos argumentos e dos resultados dos métodos; o `property` do Python (19/11).
/// 19: os registros (o resultado dos métodos com campos: `Screen.monitors`) (19/11).
/// 20: o registro do C# pelo nome inteiro (o `Screen.Monitor` escondia o tipo).
/// 21: os itens com campos num texto (`ListView.items`: `Título|detalhe;Outro`) (19/11).
/// 22: os nomes do projeto tipados: o `style` de cada classe, o `theme` e os forms (19/11).
/// 23: o ponteiro, a tecla, a roda, o gesto e o deslize tipados nas outras linguagens (os
/// registros `PointerEvent`...) e o handler de classes diferentes com quem disparou (19/11).
/// 24: os registros em JSON nas propriedades (o fim dos itens `a|b;c`), com cor, data, listas,
/// enum e registro dentro de registro; os eventos com registro (19/11).
/// 25: o `field` dos registros em Python e o campo com o nome do registro em C# (19/11).
const GENERATOR_VERSION: u32 = 25;

/// Entre os pedaços de um arquivo (os laços do gerador), nos arquivos das linguagens que juntam.
const SEP: &str = "\u{1e}\n";

/// A pasta dos pedaços (`generated/components/forms`, `.../classes`) e a extensão deles: em Rust
/// cada um é um arquivo de verdade (`include!`); nas outras, o arquivo de sempre é montado
/// juntando os pedaços (o Java não tem classe parcial: separar mudaria o `Components.Button`).
fn parts_dir(project: &Project) -> PathBuf {
    project.code_dir().join("generated/components")
}

fn part_ext(language: Language) -> &'static str {
    if language == Language::Rust { "rs" } else { "part" }
}

fn write_if_changed(path: &std::path::Path, text: &str) -> Result<bool, String> {
    if std::fs::read_to_string(path).ok().as_deref() == Some(text) {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
    Ok(true)
}

/// A versão dos pedaços: a do gerador, a linguagem e o catálogo.
fn parts_version(project: &Project, catalog: &Catalog) -> String {
    format!("{GENERATOR_VERSION} {} {}\n", project.language.folder(), catalog.stamp())
}

/// Os pedaços estão todos valendo (a mesma versão, e o arquivo de sempre existe)?
fn parts_valid(project: &Project, catalog: &Catalog) -> bool {
    let dir = parts_dir(project);
    std::fs::read_to_string(dir.join("version")).ok().as_deref() == Some(parts_version(project, catalog).as_str())
        && project.code_dir().join(file(project.language)).is_file()
}

/// As marcas dos JSON de que cada pedaço foi feito (o do form e o das variantes), pelo módulo
/// do form: o pedaço vale enquanto elas são as de agora.
type SourceStamps = std::collections::BTreeMap<String, Vec<crate::FileStamp>>;

fn sources_file(project: &Project) -> PathBuf {
    parts_dir(project).join("sources")
}

fn load_sources(project: &Project) -> SourceStamps {
    std::fs::read_to_string(sources_file(project)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

/// As marcas de agora do JSON do form e das variantes dele.
fn form_stamps(project: &Project, entry: &crate::FormEntry, files: &mut crate::FormFiles) -> Vec<crate::FileStamp> {
    std::iter::once(entry.file.clone())
        .chain(project.entry_variants(entry, files).iter().map(|v| crate::variant_file(&entry.file, v)))
        .filter_map(|f| files.stamp(&project.file(&f)))
        .collect()
}

/// O pedaço do form está em dia: existe e foi feito dos JSON como estão agora (a marca de cada
/// um, não a hora comparada com a do pedaço: gravado no mesmo milissegundo, o form parecia
/// mais velho que o pedaço).
fn piece_fresh(project: &Project, entry: &crate::FormEntry, files: &mut crate::FormFiles, sources: &SourceStamps) -> bool {
    let module = crate::codegen::module_name(&entry.file);
    let piece = parts_dir(project).join("forms").join(format!("{module}.{}", part_ext(project.language)));
    files.exists(&piece) && sources.get(&module).is_some_and(|recorded| *recorded == form_stamps(project, entry, files))
}

/// Os forms que mudaram desde a última geração (o pedaço deles ficou para trás); `None`: todos
/// (outra versão do gerador ou do catálogo, ou nunca gerado). Quem gera o resto (as units, o
/// registro, a verificação) olha só estes.
pub fn stale_forms(project: &Project) -> Option<Vec<String>> {
    let catalog = Catalog::load();
    if !parts_valid(project, &catalog) {
        return None;
    }
    let mut files = crate::FormFiles::default();
    let sources = load_sources(project);
    Some(project.forms.iter().filter(|e| !piece_fresh(project, e, &mut files, &sources)).map(|e| e.name.clone()).collect())
}

/// Os arquivos de tema do projeto: os de `themes/` e o do `theme` do projeto, se for arquivo.
fn theme_files(project: &Project) -> Vec<String> {
    let mut out: Vec<String> = crate::Files::Dir(project.dir()).list("themes/").into_iter().filter(|f| f.to_lowercase().ends_with(".json")).collect();
    if project.theme.to_lowercase().ends_with(".json") && !out.contains(&project.theme) {
        out.push(project.theme.clone());
    }
    out
}

/// A marca dos nomes do projeto que entram nos enums (os temas; os forms já estão no índice):
/// mudou, o arquivo de sempre é refeito com os valores novos.
fn names_stamp(project: &Project) -> String {
    let themes = theme_files(project).into_iter().map(|f| {
        let stamp = std::fs::metadata(project.file(&f)).ok().and_then(|m| crate::FileStamp::of(&m));
        format!("theme {f} {}\n", stamp.map_or_else(String::new, |s| format!("{} {}", s.nanos, s.len)))
    });
    themes.chain(package_names(project).into_iter().map(|p| format!("package {p}\n"))).collect()
}

/// Os pacotes do projeto (as pastas de `packages/` com `package.json`), sem descer nelas: o
/// `Frame` mostra o form de um pelo `pkg:Nome`.
fn package_names(project: &Project) -> Vec<String> {
    let dir = project.dir().join(crate::packages::DIR);
    let mut out: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().join("package.json").is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    out.sort();
    out
}

/// Junta aos enums com nomes do projeto (o `style` de cada classe, o `theme`, as referências a
/// form) os do projeto: as variantes e os nomes dos temas dele e os forms. O catálogo só
/// conhece os do Aylis.
fn project_names(project: &Project, enums: &mut BTreeMap<String, EnumDef>) {
    if !enums.values().any(|d| d.names.is_some()) {
        return;
    }
    let files = crate::Files::Dir(project.dir());
    let themes: Vec<aylis_core::Theme> = theme_files(project).iter().filter_map(|f| files.read_text(f).ok()).filter_map(|t| aylis_json::load_theme(&t).ok()).collect();
    // Só o que vira texto em qualquer linguagem (sem aspas, barra invertida nem controle).
    let fits = |v: &str| !v.is_empty() && !v.chars().any(|c| c.is_control() || matches!(c, '"' | '\'' | '\\'));
    for def in enums.values_mut() {
        let more: Vec<String> = match &def.names {
            None => continue,
            Some(ProjectNames::Style(class)) => themes.iter().flat_map(|t| t.variants_of(class)).collect(),
            Some(ProjectNames::Theme) => themes.iter().map(|t| t.name.clone()).collect(),
            Some(ProjectNames::Form) => project.forms.iter().map(|f| f.name.clone()).chain(package_names(project).into_iter().map(|p| format!("{}{p}", crate::packages::PREFIX))).collect(),
        };
        for v in more {
            if fits(&v) && !def.values.iter().any(|x| x.eq_ignore_ascii_case(&v)) {
                def.values.push(v);
            }
        }
    }
}

/// Gera o acesso tipado da linguagem do projeto (`project.language`) e põe o `import` da
/// instância em cada arquivo de eventos. Devolve os arquivos escritos.
pub fn generate(project: &Project) -> Result<Vec<PathBuf>, String> {
    generate_in(project, None)
}

/// O [`generate`] olhando só o form `only` (o F12; `None`: todos). Cada form tem o pedaço dele em
/// `generated/components/forms`, refeito só quando o JSON do form (ou de uma variante) não é
/// mais o de que ele foi feito (as marcas em `generated/components/sources`); cada classe
/// usada, o dela em `.../classes`. O arquivo de sempre só junta os pedaços.
pub fn generate_in(project: &Project, only: Option<&str>) -> Result<Vec<PathBuf>, String> {
    let catalog = Catalog::load();
    let language = project.language;
    let dir = parts_dir(project);
    let (forms_dir, classes_dir) = (dir.join("forms"), dir.join("classes"));
    let ext = part_ext(language);
    let main = project.code_dir().join(file(language));
    let mut written = Vec::new();
    // Outra versão do gerador ou outro catálogo: os pedaços de antes não valem.
    let version = parts_version(project, &catalog);
    let full = !parts_valid(project, &catalog);
    if full {
        // Só os pedaços: a lista dos forms e a marca do registro, na mesma pasta, ficam.
        let _ = std::fs::remove_dir_all(&forms_dir);
        let _ = std::fs::remove_dir_all(&classes_dir);
        std::fs::create_dir_all(&forms_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&classes_dir).map_err(|e| e.to_string())?;
        write_if_changed(&dir.join("version"), &version)?;
    }
    let mut files = crate::FormFiles::default();
    let mut sources = if full { SourceStamps::new() } else { load_sources(project) };
    let sources_before = sources.clone();
    let mut changed = full;
    let mut regenerated: Vec<String> = Vec::new();
    // As classes que cada form usa (o módulo dele → as classes): a classe que nenhum form usa
    // mais sai, sem reler os forms que não mudaram. Ficava o acesso de uma classe cuja feature
    // saiu do Cargo.toml, e o build quebrava no gerado (relatório do RTSPlayer, 10/11).
    let uses_file = dir.join("classes.json");
    let mut uses: std::collections::BTreeMap<String, BTreeSet<String>> =
        if full { Default::default() } else { std::fs::read_to_string(&uses_file).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default() };
    let uses_before = uses.clone();
    for entry in project.forms.iter().filter(|e| full || only.is_none_or(|o| e.name.eq_ignore_ascii_case(o))) {
        let module = crate::codegen::module_name(&entry.file);
        let piece = forms_dir.join(format!("{module}.{ext}"));
        if !full && piece_fresh(project, entry, &mut files, &sources) {
            continue;
        }
        // As marcas de antes de ler: um form salvo enquanto o pedaço é feito fica para trás.
        sources.insert(module.clone(), form_stamps(project, entry, &mut files));
        let model = model_of(&catalog, vec![form_model(project, entry, &mut files)?]);
        uses.insert(module, model.classes.iter().map(|(c, _)| c.clone()).collect());
        // As classes que ainda não têm pedaço (as que este form passou a usar).
        for (class, _) in &model.classes {
            let path = classes_dir.join(format!("{class}.{ext}"));
            if !path.is_file() {
                let one = model_with(&catalog, Vec::new(), BTreeSet::from([class.clone()]));
                std::fs::write(&path, generator(language, &one).pieces(Part::Class, class).join(SEP)).map_err(|e| e.to_string())?;
                changed = true;
            }
        }
        let form = &model.forms[0];
        if write_if_changed(&piece, &generator(language, &model).pieces(Part::Form, &form.name).join(SEP))? {
            changed = true;
        }
        regenerated.push(entry.name.clone());
    }
    // Os forms que saíram do projeto saem das marcas.
    let modules: BTreeSet<String> = project.forms.iter().map(|e| crate::codegen::module_name(&e.file)).collect();
    sources.retain(|m, _| modules.contains(m));
    uses.retain(|m, _| modules.contains(m));
    // Com as classes de todos os forms conhecidas (um projeto gerado antes desta lista ainda não
    // tem as de todos), a classe que nenhum usa sai.
    if modules.iter().all(|m| uses.contains_key(m)) {
        let used: BTreeSet<&String> = uses.values().flatten().collect();
        for e in std::fs::read_dir(&classes_dir).into_iter().flatten().flatten() {
            let stem = e.path().file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            if !used.contains(&stem) && std::fs::remove_file(e.path()).is_ok() {
                changed = true;
            }
        }
    }
    if uses != uses_before {
        let text = serde_json::to_string(&uses).map_err(|e| e.to_string())?;
        std::fs::write(&uses_file, text).map_err(|e| format!("não consegui gravar {}: {e}", uses_file.display()))?;
    }
    if sources != sources_before {
        let text = serde_json::to_string(&sources).map_err(|e| e.to_string())?;
        std::fs::write(sources_file(project), text).map_err(|e| format!("não consegui gravar {}: {e}", sources_file(project).display()))?;
    }
    files.forget(&classes_dir);
    files.forget(&forms_dir);
    // O arquivo de sempre: o que é fixo do gerador e, em cada laço, os pedaços na ordem (as
    // classes pelo nome, os forms na ordem do projeto). Os forms que saíram do projeto saem.
    // Com os temas: os enums com nomes do projeto mudam com eles.
    let list: String = project.forms.iter().map(|e| format!("{}\n", e.name)).chain(std::iter::once(names_stamp(project))).collect();
    if changed || std::fs::read_to_string(dir.join("index")).ok().as_deref() != Some(list.as_str()) {
        let modules: Vec<String> = project.forms.iter().map(|e| crate::codegen::module_name(&e.file)).collect();
        for e in std::fs::read_dir(&forms_dir).into_iter().flatten().flatten() {
            let stem = e.path().file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            if !modules.contains(&stem) {
                let _ = std::fs::remove_file(e.path());
            }
        }
        let mut classes: Vec<String> = std::fs::read_dir(&classes_dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()))
            .collect();
        classes.sort();
        let read_piece = |path: PathBuf| std::fs::read_to_string(&path).map_err(|e| format!("não consegui ler {}: {e}", path.display()));
        let mut form_pieces: Vec<Vec<String>> = Vec::new();
        let mut class_pieces: Vec<Vec<String>> = Vec::new();
        if language != Language::Rust {
            for m in &modules {
                form_pieces.push(read_piece(forms_dir.join(format!("{m}.{ext}")))?.split(SEP).map(str::to_string).collect());
            }
            for c in &classes {
                class_pieces.push(read_piece(classes_dir.join(format!("{c}.{ext}")))?.split(SEP).map(str::to_string).collect());
            }
        }
        let skeleton = generator(language, &Model { forms: Vec::new(), classes: Vec::new(), methods: Default::default(), enums: Default::default(), records: Default::default() });
        // Os enums das classes que entram no arquivo.
        let mut used = model_with(&catalog, Vec::new(), classes.iter().cloned().collect());
        project_names(project, &mut used.enums);
        if language != Language::Rust {
            used.records.extend(event_records(&catalog, &classes));
        }
        let (mut class_loop, mut form_loop) = (0, 0);
        let types_text = types(language, &used.enums, &used.records);
        let mut types_pieces = types_text.split(TYPES_SEP);
        let mut text = String::new();
        for seg in &skeleton.segs {
            match seg {
                Seg::Text(t) => text.push_str(t),
                Seg::Items(Part::Class, _) => {
                    for (i, c) in classes.iter().enumerate() {
                        match language {
                            Language::Rust => text.push_str(&format!("include!(\"components/classes/{c}.rs\");\n")),
                            _ => text.push_str(class_pieces[i].get(class_loop).map_or("", String::as_str)),
                        }
                    }
                    class_loop += 1;
                }
                Seg::Items(Part::Form, _) => {
                    for (i, m) in modules.iter().enumerate() {
                        match language {
                            Language::Rust => text.push_str(&format!("include!(\"components/forms/{m}.rs\");\n")),
                            _ => text.push_str(form_pieces[i].get(form_loop).map_or("", String::as_str)),
                        }
                    }
                    form_loop += 1;
                }
                Seg::Items(Part::Types, _) => text.push_str(types_pieces.next().unwrap_or_default()),
            }
        }
        if write_if_changed(&main, &text)? {
            written.push(main.clone());
        }
        write_if_changed(&dir.join("index"), &list)?;
    }
    // O `import` da instância do form no arquivo de eventos dele.
    for entry in project.forms.iter().filter(|e| regenerated.contains(&e.name) || only.is_some_and(|o| e.name.eq_ignore_ascii_case(o))) {
        let source = crate::codegen::entry_source(project, entry);
        let Ok(text) = std::fs::read_to_string(&source) else { continue };
        if let Some(new) = with_import(language, &text, &entry.name) {
            std::fs::write(&source, new).map_err(|e| format!("não consegui gravar {}: {e}", source.display()))?;
            written.push(source);
        }
    }
    Ok(written)
}

const HEADER: &str = "Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).\nGerado pela IDE do Aylis a cada \"Gerar código\": não edite.";

/// O arquivo de eventos com a linha que traz a instância do form, se ainda não tem.
fn with_import(language: Language, text: &str, form: &str) -> Option<String> {
    let insert_after = |text: &str, line_matches: &dyn Fn(&str) -> bool, new: &str| -> Option<String> {
        let mut lines: Vec<&str> = text.split('\n').collect();
        let at = lines.iter().rposition(|l| line_matches(l)).map_or(0, |i| i + 1);
        lines.insert(at, new);
        Some(lines.join("\n"))
    };
    match language {
        Language::Go => None,
        Language::Rust => {
            // A struct do form (os handlers são métodos dela).
            if text.contains("crate::generated::") {
                return None;
            }
            insert_after(text, &|l| l.starts_with("use "), &format!("use crate::generated::{};", form_struct(form)))
        }
        Language::Python => {
            if text.contains("from .components import") {
                return None;
            }
            // Depois da docstring do módulo (a primeira linha que fecha as aspas triplas).
            let mut lines: Vec<&str> = text.split('\n').collect();
            let end = if lines.first().is_some_and(|l| l.starts_with("\"\"\"")) {
                let first = lines[0];
                if first.len() > 3 && first[3..].contains("\"\"\"") { 0 } else { lines.iter().skip(1).position(|l| l.contains("\"\"\"")).map_or(0, |i| i + 1) }
            } else {
                0
            };
            let new = format!("\nfrom .components import {form}  # noqa: E402");
            lines.insert(end + 1, &new);
            Some(lines.join("\n"))
        }
        Language::TypeScript => {
            if text.contains("./components.ts") {
                return None;
            }
            insert_after(text, &|l| l.starts_with("import "), &format!("import {{ {form} }} from \"./components.ts\";"))
        }
        Language::CSharp => {
            if text.contains("using static App.Forms.Components;") {
                return None;
            }
            insert_after(text, &|l| l.starts_with("using "), "using static App.Forms.Components;")
        }
        Language::Java => {
            // A instância do form e os enums do acesso tipado (`WindowState.FULL_SCREEN`).
            let mut out = text.to_string();
            if !out.contains("import static forms.Components.") {
                out = insert_after(&out, &|l| l.starts_with("package ") || l.starts_with("import "), &format!("import static forms.Components.{form};"))?;
            }
            if !out.contains("import forms.Components.*;") {
                out = insert_after(&out, &|l| l.starts_with("package ") || l.starts_with("import "), "import forms.Components.*;")?;
            }
            (out != text).then_some(out)
        }
        Language::Cpp => {
            if text.contains("\"components.hpp\"") || text.contains("\"forms.hpp\"") {
                return None;
            }
            insert_after(text, &|l| l.starts_with("#include "), "#include \"components.hpp\"")
        }
        Language::Pascal => {
            if text.to_lowercase().contains("ayliscomponents") {
                return None;
            }
            // No primeiro `uses` (o da interface): `uses fpjson, aylis;` → `..., ayliscomponents;`.
            let lower = text.to_lowercase();
            let uses = lower.find("\nuses")?;
            let semicolon = uses + text[uses..].find(';')?;
            Some(format!("{}, ayliscomponents{}", &text[..semicolon], &text[semicolon..]))
        }
    }
}

// ---------------------------------------------------------------------------
// Rust (no mesmo processo: as funções recebem o `ui`)

/// O nome da struct de um form em Rust (`frmCalculadora` → `FrmCalculadora`).
pub fn form_struct(form: &str) -> String {
    pascal(form)
}

/// Nomes de campo que a struct do form já usa.
const RUST_FORM_FIELDS: [&str; 2] = ["form", "data"];

fn rust(m: &Model) -> Out {
    let s = String::from(
        "//! Os componentes dos forms com as propriedades tipadas e a struct de cada form, com os\n\
         //! componentes resolvidos ao abrir (`self.edtNome.set_text(ui, ...)`).\n\
         //! Gerado pela IDE do Aylis a cada \"Gerar código\": não edite.\n\n\
         #![allow(non_snake_case, dead_code, unused_imports)]\n\n\
         use aylis::prelude::handles::{self, Handle};\n\
         use aylis::prelude::{ComponentId, PropValue, Ui};\n",
    );
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        s.push_str(&format!(
            "\n/// Um {class} de um form.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {class} {{\n    pub id: ComponentId,\n}}\n\n\
             impl Handle for {class} {{\n    fn id(&self) -> ComponentId {{\n        self.id\n    }}\n}}\n\n/// Quem disparou um handler tipado (`sender: {class}`): só um {class}.\nimpl aylis::prelude::FromSender for {class} {{\n    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {{\n        (ui.class_name(id) == Some(\"{class}\")).then_some(Self {{ id }})\n    }}\n}}\n\nimpl {class} {{\n"
        ));
        for p in props {
            let method = rust_ident(&p.name);
            let ty = rust_ty(&p.kind, &m.enums);
            let read = match &p.kind {
                Kind::Bool => "handles::read(ui, self.id, PROP).and_then(|v| v.to_bool().ok()).unwrap_or_default()".to_string(),
                Kind::Int => "handles::read(ui, self.id, PROP).and_then(|v| v.to_i64().ok()).unwrap_or_default()".to_string(),
                Kind::Float => "handles::read(ui, self.id, PROP).and_then(|v| v.to_f64().ok()).unwrap_or_default()".to_string(),
                Kind::Text => "handles::text(ui, handles::read(ui, self.id, PROP))".to_string(),
                Kind::Color => "handles::read(ui, self.id, PROP).and_then(|v| v.to_color().ok()).unwrap_or_default()".to_string(),
                Kind::Bounds => "handles::read(ui, self.id, PROP).and_then(|v| v.to_bounds().ok()).unwrap_or_default()".to_string(),
                Kind::Component => "handles::read_ref(ui, self.id, PROP)".to_string(),
                Kind::Enum(_) => format!("handles::read_enum::<{ty}>(ui, self.id, PROP)"),
                k if rust_set_type(k, &m.enums) => "handles::read(ui, self.id, PROP).and_then(|v| v.to_anchors().ok()).unwrap_or_default()".to_string(),
                Kind::Set(n) => format!("handles::read_set::<{n}>(ui, self.id, PROP)"),
                Kind::Date => "handles::read_date(ui, self.id, PROP)".to_string(),
                Kind::Time => "handles::read_time(ui, self.id, PROP)".to_string(),
                Kind::DateTime => "handles::read_datetime(ui, self.id, PROP)".to_string(),
                Kind::List => "handles::read_list(ui, self.id, PROP)".to_string(),
                Kind::NumberList => "handles::read_numbers(ui, self.id, PROP)".to_string(),
                Kind::ColorList => "handles::read_colors(ui, self.id, PROP)".to_string(),
                Kind::Shadow => "handles::read_shadow(ui, self.id, PROP)".to_string(),
                Kind::Easing => "handles::read_easing(ui, self.id, PROP)".to_string(),
                Kind::PointList => "handles::read_points(ui, self.id, PROP)".to_string(),
                Kind::RecordList(n) => format!("{n}::list_from_text(&handles::text(ui, handles::read(ui, self.id, PROP)))"),
                Kind::Record(n) => format!("aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(&handles::text(ui, handles::read(ui, self.id, PROP))).ok().filter(|j| j.is_object()).map(|j| {n}::from_json(&j))"),
            };
            let prop = format!("\"{}\"", p.name);
            s.push_str(&format!("    pub fn {method}(self, ui: &Ui) -> {ty} {{\n        {}\n    }}\n", read.replace("PROP", &prop)));
            if !p.read_only {
                let (arg, val) = match &p.kind {
                    Kind::Text => ("value: impl Into<String>".to_string(), "Into::<String>::into(value)".to_string()),
                    Kind::Color => ("value: impl Into<handles::ColorRef>".to_string(), "Into::<handles::ColorRef>::into(value)".to_string()),
                    Kind::Component => ("value: impl Into<handles::Ref>".to_string(), "Into::<handles::Ref>::into(value)".to_string()),
                    Kind::Enum(_) => (format!("value: {ty}"), "PropValue::enum_value(value)".to_string()),
                    k @ Kind::Set(_) if rust_set_type(k, &m.enums) => (format!("value: {ty}"), "PropValue::from(value)".to_string()),
                    Kind::Set(n) => (format!("value: &[{n}]"), "handles::set_value(value)".to_string()),
                    Kind::Date => (format!("value: {ty}"), "handles::date_value(value)".to_string()),
                    Kind::Time => (format!("value: {ty}"), "handles::time_value(value)".to_string()),
                    Kind::DateTime => (format!("value: {ty}"), "handles::datetime_value(value)".to_string()),
                    Kind::List => ("value: &[impl AsRef<str>]".to_string(), "handles::list_value(value)".to_string()),
                    Kind::NumberList => ("value: &[f64]".to_string(), "handles::numbers_value(value)".to_string()),
                    Kind::ColorList => ("value: &[aylis::prelude::Color]".to_string(), "handles::colors_value(value)".to_string()),
                    Kind::Shadow => ("value: Option<aylis::prelude::Shadow>".to_string(), "handles::shadow_value(value)".to_string()),
                    Kind::Easing => ("value: aylis::prelude::Easing".to_string(), "handles::easing_value(value)".to_string()),
                    Kind::PointList => ("value: &[aylis::prelude::PointF]".to_string(), "handles::points_value(value)".to_string()),
                    Kind::RecordList(n) => (format!("value: &[{n}]"), format!("PropValue::String({n}::list_text(value))")),
                    Kind::Record(_) => (format!("value: {ty}"), "PropValue::String(value.map(|r| r.to_json().to_string()).unwrap_or_default())".to_string()),
                    _ => (format!("value: {ty}"), "value".to_string()),
                };
                s.push_str(&format!("    pub fn set_{}(self, ui: &mut Ui, {arg}) {{\n        handles::write(ui, self.id, {prop}, {val})\n    }}\n", p.name));
            }
        }
        for mth in m.methods(class) {
            let name = rust_ident(&mth.name);
            match mth.via.as_str() {
                "invoke" => {
                    let params: String = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| format!(", {a}: {}", rust_arg(k, &m.enums))).collect();
                    let values: Vec<String> = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| rust_value(a, k, &m.enums)).collect();
                    // A falha volta como `Err` com o motivo (e vai para a atividade do app).
                    let call = format!("handles::try_call(ui, self.id, \"{}\", &[{}])", mth.name, values.join(", "));
                    let (ret, body) = match &mth.returns {
                        None => ("()".to_string(), format!("{call}.map(|_| ())")),
                        Some(k) => (rust_ty(k, &m.enums), format!("let v = {call}?;\n        Ok({})", rust_from("v", k, &m.enums))),
                    };

                    s.push_str(&format!("    pub fn {name}(self, ui: &mut Ui{params}) -> Result<{ret}, String> {{\n        {body}\n    }}\n"));
                }
                "show" => s.push_str("    pub fn show(self, ui: &mut Ui) {\n        let _ = ui.show(self.id);\n    }\n"),
                "close" => s.push_str("    pub fn close(self, ui: &mut Ui) -> bool {\n        ui.close(self.id)\n    }\n"),
                // O foco: `set_focus` do controle; no form, quem tem o foco e o Tab pelo código.
                "focus" => s.push_str("    pub fn set_focus(self, ui: &mut Ui) -> bool {\n        ui.set_focus(self.id)\n    }\n"),
                "focused" => s.push_str(
                    "    /// O nome do controle com foco (vazio sem foco).\n    pub fn focused(self, ui: &Ui) -> String {\n        ui.focused(ui.root_of(self.id)).and_then(|f| ui.name(f)).unwrap_or_default().to_string()\n    }\n",
                ),
                "focus_next" | "focus_prev" => s.push_str(&format!(
                    "    pub fn {name}(self, ui: &mut Ui) -> bool {{\n        let root = ui.root_of(self.id);\n        ui.focus_next(root, {})\n    }}\n",
                    mth.via == "focus_next"
                )),
                // Diálogos e mídia: as funções das classes do aylis (o nome qualificado, porque
                // o handle daqui tem o mesmo nome da classe).
                "execute" => s.push_str(&format!("    pub fn execute(self, ui: &mut Ui) {{\n        aylis::prelude::{class}::execute(ui, self.id);\n    }}\n")),
                "media" => {
                    let (params, ret, body) = match mth.name.as_str() {
                        "seek" => (", seconds: f64", "", format!("aylis::prelude::{class}::seek(ui, self.id, seconds);")),
                        "play" => ("", "", format!("let _ = aylis::prelude::{class}::play(ui, self.id);")),
                        "position" => ("", " -> f64", format!("aylis::prelude::{class}::position(ui, self.id)")),
                        "duration" => ("", " -> f64", format!("aylis::prelude::{class}::duration(ui, self.id).unwrap_or_default()")),
                        op => ("", "", format!("aylis::prelude::{class}::{op}(ui, self.id);")),
                    };
                    s.push_str(&format!("    pub fn {name}(self, ui: &mut Ui{params}){ret} {{\n        {body}\n    }}\n"));
                }
                _ => {}
            }
        }
        s.push_str("}\n");
    }
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_struct(&f.name);
        let fs: Vec<&(String, String)> = f.components.iter().filter(|(n, _)| !RUST_FORM_FIELDS.contains(&n.as_str())).collect();
        let optional = |n: &str| f.optional.contains(n);
        s.push_str(&format!(
            "\n/// O form {name}: os componentes (resolvidos ao abrir; os que só existem em algumas\n\
             /// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela\n\
             /// aberta tem a sua; os handlers são métodos dela.\n\
             pub struct {ty} {{\n    pub form: Form,\n",
            name = f.name
        ));
        for (n, c) in &fs {
            let field = if optional(n) { format!("Option<{c}>") } else { c.clone() };
            s.push_str(&format!("    pub {}: {field},\n", rust_ident(n)));
        }
        s.push_str(&format!("    pub data: super::{}::Data,\n}}\n\n", f.module));
        s.push_str(&format!(
            "/// Os componentes do {name} sem o estado, para o código fora dos handlers\n\
             /// (`{ty}::components(ui)`): uma cópia, que se usa à vontade com o `ui`.\n\
             #[derive(Clone, Copy, Debug)]\n\
             pub struct {ty}Components {{\n    pub form: Form,\n",
            name = f.name
        ));
        for (n, c) in &fs {
            let field = if optional(n) { format!("Option<{c}>") } else { c.clone() };
            s.push_str(&format!("    pub {}: {field},\n", rust_ident(n)));
        }
        s.push_str("}\n\n");
        s.push_str(&format!("impl std::ops::Deref for {ty} {{\n    type Target = Form;\n    fn deref(&self) -> &Form {{\n        &self.form\n    }}\n}}\n\nimpl {ty} {{\n"));
        // A montagem: o que não bate impede o form de abrir.
        s.push_str(&format!(
            "    /// Monta a instância de um {name} recém-criado (o `register` liga). Um componente que\n\
             \x20   /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.\n\
             \x20   pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {{\n\
             \x20       let mut problems: Vec<String> = Vec::new();\n\
             \x20       let instance = Self {{\n            form: Form {{ id: root }},\n",
            name = f.name
        ));
        for (n, c) in &fs {
            let value = if optional(n) {
                format!("ui.find_in(root, \"{n}\").map(|id| {c} {{ id }})")
            } else {
                format!("{c} {{ id: handles::resolve(ui, root, \"{n}\", \"{c}\", &mut problems) }}")
            };
            s.push_str(&format!("            {}: {value},\n", rust_ident(n)));
        }
        s.push_str(&format!(
            "            data: Default::default(),\n        }};\n        \
             if !problems.is_empty() {{\n            \
             return Err(format!(\"o form {name} não bate com o código: {{}}\", problems.join(\"; \")));\n        }}\n        \
             ui.set_form_components(root, Box::new({ty}Components {{\n            form: instance.form,\n",
            name = f.name
        ));
        for (n, _) in &fs {
            let field = rust_ident(n);
            s.push_str(&format!("            {field}: instance.{field},\n"));
        }
        s.push_str("        }));\n        Ok(Box::new(instance))\n    }\n\n");
        // Fora dos handlers, ou num código que um handler do próprio form chama (quando a
        // instância está emprestada e o `get` dá `None`): a cópia dos componentes.
        s.push_str(&format!(
            "    /// Os componentes do {name} aberto (o primeiro, se houver dois), sem o estado: valem\n\
             \x20   /// também enquanto um handler dele roda, quando o `get` dá `None`.\n\
             \x20   pub fn components(ui: &Ui) -> Option<{ty}Components> {{\n        ui.form_components(ui.find_path(\"{name}\")?)\n    }}\n\n",
            name = f.name
        ));
        // Criar uma janela do form, sem o nome em texto
        // no código do usuário: `FrmClientes::create(ui)?` e depois `ui.show(form)`.
        s.push_str(&format!(
            "    /// Cria o {name} (ou devolve o que já está criado); mostrar é com\n\
             \x20   /// `ui.show`.\n\
             \x20   pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {{\n        ui.create_form(\"{name}\")\n    }}\n\n",
            name = f.name
        ));
        // Um frame repetido (uma linha de lista, um cartão): mais uma instância dentro de
        // `parent`, já com os componentes tipados, em vez de montar a peça em código.
        s.push_str(&format!(
            "    /// Cria mais um {name} dentro de `parent` (uma linha, um cartão: cada um com a sua\n\
             \x20   /// instância e os handlers falando com ela) e devolve os componentes dele. O nome\n\
             \x20   /// segue livre no form de `parent` (`{name}`, `{name}2`...).\n\
             \x20   pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<{ty}Components, aylis::prelude::Error> {{\n\
             \x20       let root = ui.create_form_instance(\"{name}\")?;\n\
             \x20       let name = ui.free_name_in(parent, \"{name}\");\n\
             \x20       if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {{\n\
             \x20           ui.free(root);\n\
             \x20           return Err(e);\n\
             \x20       }}\n\
             \x20       // Como o componente Frame: sem `fill` próprio, transparente (G05).\n\
             \x20       if matches!(ui.get_prop(root, \"fill\"), Ok(aylis::prelude::PropValue::Null)) {{\n\
             \x20           let _ = ui.set_prop(root, \"fill\", aylis::prelude::Color::TRANSPARENT);\n\
             \x20       }}\n\
             \x20       ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)\n    }}\n\n\
             \x20   /// Os componentes de uma instância do {name} (a raiz dela: o `form.id` de um\n\
             \x20   /// [`create_in`](Self::create_in)).\n\
             \x20   pub fn components_of(ui: &Ui, root: ComponentId) -> Option<{ty}Components> {{\n        ui.form_components(root)\n    }}\n\n",
            name = f.name
        ));
        // Outro form chegando neste: o primeiro aberto.
        s.push_str(&format!(
            "    /// O {name} aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.\n\
             \x20   pub fn get(ui: &Ui) -> Option<&{ty}> {{\n        ui.form_instance(ui.find_path(\"{name}\")?)\n    }}\n\n\
             \x20   pub fn get_mut(ui: &mut Ui) -> Option<&mut {ty}> {{\n        let form = ui.find_path(\"{name}\")?;\n        ui.form_instance_mut(form)\n    }}\n",
            name = f.name
        ));
        // Pelo nome (`self.find_button("btn7")`) e o do evento
        // (`self.sender_as_button(ui, sender)`), pela classe.
        for class in Model::classes_of(f) {
            let sn = snake(class);
            s.push_str(&format!("\n    /// O {class} deste form com o nome (`None` se não há um {class} com ele).\n    pub fn find_{sn}(&self, name: &str) -> Option<{class}> {{\n        match name {{\n"));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                let value = if optional(n) { format!("self.{}", rust_ident(n)) } else { format!("Some(self.{})", rust_ident(n)) };
                s.push_str(&format!("            \"{n}\" => {value},\n"));
            }
            s.push_str("            _ => None,\n        }\n    }\n");
            s.push_str(&format!(
                "\n    /// O {class} que disparou o evento (`None` se o `sender` não é um {class} deste form).\n    pub fn sender_as_{sn}(&self, ui: &Ui, sender: ComponentId) -> Option<{class}> {{\n        self.find_{sn}(ui.name(sender)?)\n    }}\n"
            ));
        }
        s.push_str("}\n");
    }
    s.end();
    s
}

// ---------------------------------------------------------------------------
// Python

/// O tipo de um valor em Python (as anotações).
fn python_ty(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "bool".into(),
        Kind::Int => "int".into(),
        Kind::Float => "float".into(),
        Kind::Text => "str".into(),
        Kind::Color => "Color".into(),
        Kind::Bounds => "Bounds".into(),
        Kind::Component => "AylisComponent | None".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("list[{n}]"),
        Kind::Date => "datetime.date | None".into(),
        Kind::Time => "datetime.time | None".into(),
        Kind::DateTime => "datetime.datetime | None".into(),
        Kind::List => "list[str]".into(),
        Kind::NumberList => "list[float]".into(),
        Kind::ColorList => "list[Color]".into(),
        Kind::Shadow => "Shadow | None".into(),
        Kind::Easing => "Easing".into(),
        Kind::PointList => "list[Point]".into(),
        Kind::Record(n) => format!("{n} | None"),
        Kind::RecordList(n) => format!("list[{n}]"),
    }
}

/// O valor JSON `v` da tela como o tipo `kind`, em Python.
fn python_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color => format!("Color.from_json({v})"),
        Kind::Bounds => format!("Bounds.from_json({v})"),
        Kind::Enum(n) => format!("_enum({n}, {v})"),
        Kind::Set(n) => format!("[_enum({n}, x) for x in ({v} or [])]"),
        Kind::Component => format!("self._component({v})"),
        Kind::Date => format!("_date({v})"),
        Kind::Time => format!("_time({v})"),
        Kind::DateTime => format!("_datetime({v})"),
        Kind::List => format!("_list({v})"),
        Kind::NumberList => format!("_numbers({v})"),
        Kind::ColorList => format!("_colors({v})"),
        Kind::Shadow => format!("Shadow.from_json({v})"),
        Kind::Easing => format!("Easing.from_json({v})"),
        Kind::PointList => format!("Point.list_from_json({v})"),
        Kind::Record(n) => format!("_record({n}, {v})"),
        Kind::RecordList(n) => format!("_records({n}, {v})"),
        _ => v.to_string(),
    }
}

/// O valor `a` do tipo `kind` como vai para a tela, em Python.
fn python_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.to_json()"),
        Kind::Enum(_) => format!("{a}.value"),
        Kind::Set(_) => format!("[x.value for x in {a}]"),
        Kind::Component => format!("{a}.id"),
        Kind::Date => format!("({a}.isoformat() if {a} else \"\")"),
        Kind::Time => format!("_time_text({a})"),
        Kind::DateTime => format!("_datetime_text({a})"),
        Kind::List => format!("\";\".join({a})"),
        Kind::NumberList => format!("\";\".join(repr(float(x)) for x in {a})"),
        Kind::ColorList => format!("\";\".join(c.to_json() for c in {a})"),
        Kind::Shadow => format!("({a}.to_json() if {a} else \"\")"),
        Kind::Easing => format!("{a}.to_json()"),
        Kind::PointList => format!("Point.list_to_json({a})"),
        Kind::RecordList(_) => format!("_records_text({a})"),
        Kind::Record(_) => format!("_record_text({a})"),
        _ => a.to_string(),
    }
}

/// Os enums em Python (`class WindowState(str, Enum)`: o valor é o nome que a tela entende).
fn python_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums {
        s.push_str(&format!("\n\nclass {name}(str, Enum):\n    \"\"\"Os valores de um enum do acesso tipado.\"\"\"\n\n"));
        for (v, id) in def.values.iter().zip(variants(&def.values, variant_upper)) {
            s.push_str(&format!("    {id} = \"{v}\"\n"));
        }
    }
    s
}

fn python(m: &Model) -> Out {
    let s = format!(
        "\"\"\"{HEADER}\"\"\"\n\nfrom __future__ import annotations\n\nimport datetime\nimport json\nimport dataclasses\nfrom dataclasses import dataclass\nfrom enum import Enum\n\nfrom aylis_client import CurrentForm, AylisComponent, AylisForm, Color, Bounds, Shadow, Easing, Point, register_form_class\n\n\n\
         def _enum(cls, value):\n    \"\"\"O valor da tela como o enum (o primeiro, se não for um dos nomes).\"\"\"\n    try:\n        return cls(value)\n    except ValueError:\n        return next(iter(cls))\n{PY_HELPERS}"
    );
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        // O Form é a base das classes dos forms (show e close já vêm do AylisForm).
        let base = if class == "Form" { "AylisForm" } else { "AylisComponent" };
        s.push_str(&format!("\n\nclass {class}({base}):\n    \"\"\"Um {class} de um form.\"\"\"\n"));
        for p in props {
            let name = python_ident(&p.name);
            let ty = python_ty(&p.kind);
            let get = match &p.kind {
                Kind::Component => format!("self._get_ref(\"{}\")", p.name),
                k => python_from(&format!("self._get(\"{}\")", p.name), k),
            };
            s.push_str(&format!("\n    @property\n    def {name}(self) -> {ty}:\n        return {get}\n"));
            if !p.read_only {
                let set = match &p.kind {
                    Kind::Component => format!("self._set_ref(\"{}\", value)", p.name),
                    k => format!("self._set(\"{}\", {})", p.name, python_value("value", k)),
                };
                s.push_str(&format!("\n    @{name}.setter\n    def {name}(self, value: {ty}) -> None:\n        {set}\n"));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| class != "Form" || !matches!(mth.via.as_str(), "show" | "close")) {
            let args = mth.arg_names();
            let params: String = args.iter().zip(&mth.args).map(|(a, k)| format!(", {a}: {}", if *k == Kind::Component { "AylisComponent".to_string() } else { python_ty(k) })).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| python_value(a, k)).collect();
            let fields: String = mth.fields(&format!("[{}]", values.join(", "))).iter().map(|(k, v)| format!(", {k}={}", if v == "true" { "True" } else { v })).collect();
            let call = format!("self.aylis.{}(\"{}\", target=self.id{fields})", if mth.waits() { "call" } else { "send" }, mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (python_ty(k), format!("return {}", python_from(&call, k))),
                None => ("None".to_string(), call),
            };
            s.push_str(&format!("\n    def {}(self{params}) -> {ret}:\n        {body}\n", python_ident(&mth.name)));
        }
    }
    s.end();

    let form_props = m.classes.iter().find(|(c, _)| c == "Form").map(|(_, p)| p.clone()).unwrap_or_default();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_type(&f.name);
        let fs = fields(f, &form_props, |p, n| p == n);
        s.push_str(&format!(
            "\n\nclass {ty}(Form):\n    \"\"\"O form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).\"\"\"\n\n    def bind(self, components):\n",
            f.name
        ));
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("        self.{n} = self._resolve_optional(components, \"{n}\", {c})\n"));
            } else {
                s.push_str(&format!("        self.{n} = self._resolve(components, \"{n}\", \"{c}\", {c})\n"));
            }
        }
        if fs.is_empty() {
            s.push_str("        pass\n");
        }
        for class in Model::classes_of(f) {
            let sn = snake(class);
            s.push_str(&format!("\n    def find_{sn}(self, name):\n        \"\"\"O {class} deste form com o nome.\"\"\"\n"));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                s.push_str(&format!("        if name == \"{n}\":\n            return self.{n}\n"));
            }
            s.push_str(&format!(
                "        return None\n\n    def sender_as_{sn}(self, event):\n        \"\"\"O {class} que disparou o evento.\"\"\"\n        return self.find_{sn}(event[\"sender\"][\"name\"])\n"
            ));
        }
    }
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!(
            "\n\n# A janela de {name} cujo evento está rodando (a primeira, fora dos handlers).\n{name} = CurrentForm(\"{name}\")\nregister_form_class(\"{name}\", {ty}, {name}._activate)\n",
            name = f.name,
            ty = form_type(&f.name)
        ));
    }
    s.end();
    s
}

// ---------------------------------------------------------------------------
// TypeScript (ler espera a tela: `await frm.comp.prop`)

/// O tipo de um valor em TypeScript (o enum é a união dos nomes).
fn ts_ty(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "boolean".into(),
        Kind::Int | Kind::Float => "number".into(),
        Kind::Text => "string".into(),
        Kind::Color => "Color".into(),
        Kind::Bounds => "Bounds".into(),
        Kind::Component => "AylisComponent | null".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("{n}[]"),
        Kind::Date => "LocalDate | null".into(),
        Kind::Time => "LocalTime | null".into(),
        Kind::DateTime => "LocalDateTime | null".into(),
        Kind::List => "string[]".into(),
        Kind::NumberList => "number[]".into(),
        Kind::ColorList => "Color[]".into(),
        Kind::Shadow => "Shadow | null".into(),
        Kind::Easing => "Easing".into(),
        Kind::PointList => "Point[]".into(),
        Kind::Record(n) => format!("{n} | null"),
        Kind::RecordList(n) => format!("{n}[]"),
    }
}

/// A promessa `p` com o valor JSON da tela como o tipo `kind`, em TypeScript.
fn ts_from(p: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color => format!("{p}.then(Color.fromJson)"),
        Kind::Bounds => format!("{p}.then(Bounds.fromJson)"),
        Kind::Component => format!("{p}.then((v) => this.componentOf(v))"),
        Kind::Set(_) => format!("{p}.then((v) => (v ?? []) as any)"),
        Kind::Date => format!("{p}.then(LocalDate.fromJson)"),
        Kind::Time => format!("{p}.then(LocalTime.fromJson)"),
        Kind::DateTime => format!("{p}.then(LocalDateTime.fromJson)"),
        Kind::List => format!("{p}.then(listOf)"),
        Kind::NumberList => format!("{p}.then(numbersOf)"),
        Kind::ColorList => format!("{p}.then(colorsOf)"),
        Kind::Shadow => format!("{p}.then(Shadow.fromJson)"),
        Kind::Easing => format!("{p}.then(Easing.fromJson)"),
        Kind::PointList => format!("{p}.then(Point.listFromJson)"),
        Kind::Record(n) => format!("{p}.then((t) => {{ const j = typeof t === \"string\" ? JSON.parse(t || \"null\") : t; return j ? {}(j) : null; }})", record_fn(n)),
        Kind::RecordList(n) => format!("{p}.then((t) => ((typeof t === \"string\" ? JSON.parse(t || \"[]\") : t) ?? []).map({}))", record_fn(n)),
        _ => format!("{p} as Promise<any>"),
    }
}

/// O valor `a` do tipo `kind` como vai para a tela, em TypeScript.
fn ts_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.toJson()"),
        Kind::Component => format!("{a}.id"),
        Kind::Date | Kind::Time | Kind::DateTime => format!("({a} ? {a}.toString() : \"\")"),
        Kind::List | Kind::NumberList => format!("{a}.join(\";\")"),
        Kind::ColorList => format!("{a}.map((c) => c.toJson()).join(\";\")"),
        Kind::Shadow => format!("({a} ? {a}.toJson() : \"\")"),
        Kind::Easing => format!("{a}.toJson()"),
        Kind::PointList => format!("Point.listToJson({a})"),
        Kind::RecordList(n) => format!("JSON.stringify({a}.map({}))", record_to_fn(n)),
        Kind::Record(n) => format!("({a} ? JSON.stringify({}({a})) : \"\")", record_to_fn(n)),
        _ => a.to_string(),
    }
}

/// Os enums em TypeScript: a união dos nomes que a tela entende (`"normal" | "maximized"`).
fn typescript_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums {
        let values: Vec<String> = def.values.iter().map(|v| format!("\"{v}\"")).collect();
        s.push_str(&format!("\n/** Os valores de um enum do acesso tipado. */\nexport type {name} = {};\n", values.join(" | ")));
    }
    s
}

fn typescript(m: &Model) -> Out {
    let s = format!(
        "// {}\n\nimport type {{ ComponentInfo, AylisEvent }} from \"../aylis.ts\";\nimport {{ AylisComponent, AylisForm, Color, Bounds, Shadow, Easing, Point, registerFormClass }} from \"../aylis.ts\";\n{TS_HELPERS}",
        HEADER.replace('\n', "\n// ")
    );
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        // O Form é a base das classes dos forms (show e close já vêm do AylisForm).
        let base = if class == "Form" { "AylisForm" } else { "AylisComponent" };
        s.push_str(&format!("\n/** Um {class} de um form. */\nexport class {class} extends {base} {{\n"));
        for p in props {
            let ty = ts_ty(&p.kind);
            let name = camel(&p.name);
            let get = match &p.kind {
                Kind::Component => format!("this.getRef(\"{}\")", p.name),
                k => ts_from(&format!("this.getProp(\"{}\")", p.name), k),
            };
            s.push_str(&format!("    get {name}(): Promise<{ty}> {{\n        return {get};\n    }}\n"));
            if !p.read_only {
                let set = match &p.kind {
                    Kind::Component => format!("this.setRef(\"{}\", value)", p.name),
                    k => format!("this.setProp(\"{}\", {})", p.name, ts_value("value", k)),
                };
                s.push_str(&format!("    set {name}(value: {ty}) {{\n        {set};\n    }}\n"));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| class != "Form" || !matches!(mth.via.as_str(), "show" | "close")) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{a}: {}", if *k == Kind::Component { "AylisComponent".to_string() } else { ts_ty(k) })).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| ts_value(a, k)).collect();
            let fields: String = mth.fields(&format!("[{}]", values.join(", "))).iter().map(|(k, v)| format!(", {k}: {v}")).collect();
            let request = format!("(\"{}\", {{ target: this.id{fields} }})", mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (format!(": Promise<{}>", ts_ty(k)), format!("return {};", ts_from(&format!("this.aylis.call{request}"), k))),
                None => (String::new(), format!("this.aylis.send{request};")),
            };
            s.push_str(&format!("    {}({}){ret} {{\n        {body}\n    }}\n", camel(&mth.name), params.join(", ")));
        }
        s.push_str("}\n");
    }
    s.end();
    let form_props = m.classes.iter().find(|(c, _)| c == "Form").map(|(_, p)| p.clone()).unwrap_or_default();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_type(&f.name);
        let fs = fields(f, &form_props, |p, n| camel(p) == n);
        s.push_str(&format!("\n/** O form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */\nexport class {ty} extends Form {{\n", f.name));
        for (n, c) in &fs {
            let opt = if f.optional.contains(n) { "?" } else { "!" };
            s.push_str(&format!("    {n}{opt}: {c};\n"));
        }
        s.push_str("    bind(list: ComponentInfo[]) {\n");
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("        this.{n} = this.resolveOptional(list, \"{n}\", {c});\n"));
            } else {
                s.push_str(&format!("        this.{n} = this.resolve(list, \"{n}\", \"{c}\", {c});\n"));
            }
        }
        s.push_str("    }\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!("    /** O {class} deste form com o nome. */\n    find{class}(name: string): {class} | undefined {{\n        switch (name) {{\n"));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                s.push_str(&format!("            case \"{n}\": return this.{n};\n"));
            }
            s.push_str(&format!(
                "        }}\n        return undefined;\n    }}\n    /** O {class} que disparou o evento. */\n    senderAs{class}(ev: AylisEvent): {class} | undefined {{\n        return this.find{class}(ev.sender.name);\n    }}\n"
            ));
        }
        s.push_str(&format!(
            "}}\n\n/** A janela de {name} cujo evento está rodando (a primeira, fora dos handlers). */\nexport let {name}: {ty};\nregisterFormClass(\"{name}\", {ty}, (form) => {{\n    {name} = form as {ty};\n}});\n",
            name = f.name
        ));
    }
    s.end();
    s
}

// ---------------------------------------------------------------------------
// C#

/// O tipo de um valor em C#.
fn cs_ty(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "bool".into(),
        Kind::Int => "long".into(),
        Kind::Float => "double".into(),
        Kind::Text => "string".into(),
        Kind::Color => "Color".into(),
        Kind::Bounds => "Bounds".into(),
        Kind::Component => "AylisComponent?".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("{n}[]"),
        Kind::Date => "DateOnly?".into(),
        Kind::Time => "TimeOnly?".into(),
        Kind::DateTime => "DateTime?".into(),
        Kind::List => "string[]".into(),
        Kind::NumberList => "double[]".into(),
        Kind::ColorList => "Color[]".into(),
        Kind::Shadow => "Shadow?".into(),
        Kind::Easing => "Easing".into(),
        Kind::PointList => "Point[]".into(),
        Kind::Record(n) => format!("global::App.Forms.{n}?"),
        Kind::RecordList(n) => format!("global::App.Forms.{n}[]"),
    }
}

/// O `JsonNode` `v` da tela como o tipo `kind`, em C# (dentro de um componente).
fn cs_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("Bool({v})"),
        Kind::Int => format!("(long)Number({v})"),
        Kind::Float => format!("Number({v})"),
        Kind::Text => format!("Text({v})"),
        Kind::Color => format!("Color.FromJson({v})"),
        Kind::Bounds => format!("Bounds.FromJson({v})"),
        Kind::Component => format!("ComponentOf({v})"),
        Kind::Enum(n) => format!("EnumNames.Of<{n}>({v}, EnumNames.{n})"),
        Kind::Set(n) => format!("EnumNames.SetOf<{n}>({v}, EnumNames.{n})"),
        Kind::Date => format!("DateOf({v})"),
        Kind::Time => format!("TimeOf({v})"),
        Kind::DateTime => format!("DateTimeOf({v})"),
        Kind::List => format!("ListOf({v})"),
        Kind::NumberList => format!("NumbersOf({v})"),
        Kind::ColorList => format!("ColorsOf({v})"),
        Kind::Shadow => format!("Shadow.FromJson({v})"),
        Kind::Easing => format!("Easing.FromJson({v})"),
        Kind::PointList => format!("Point.ListFromJson({v})"),
        Kind::Record(n) => format!("global::App.Forms.{n}.FromText({v})"),
        Kind::RecordList(n) => format!("global::App.Forms.{n}.ListFromText({v})"),
    }
}

/// O valor `a` do tipo `kind` como `JsonNode` para a tela, em C#.
fn cs_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.ToJson()"),
        Kind::Component => format!("{a}.Id"),
        Kind::Enum(n) => format!("EnumNames.{n}[(int){a}]"),
        Kind::Set(n) => format!("EnumNames.Names({a}, EnumNames.{n})"),
        Kind::Date => format!("DateText({a})"),
        Kind::Time => format!("TimeText({a})"),
        Kind::DateTime => format!("DateTimeText({a})"),
        Kind::List => format!("string.Join(\";\", {a})"),
        Kind::NumberList => format!("NumbersText({a})"),
        Kind::ColorList => format!("string.Join(\";\", {a}.Select(c => c.ToJson()))"),
        Kind::Shadow => format!("({a}?.ToJson() ?? \"\")"),
        Kind::Easing => format!("{a}.ToJson()"),
        Kind::PointList => format!("Point.ListToJson({a})"),
        Kind::RecordList(n) => format!("global::App.Forms.{n}.ListToText({a})"),
        Kind::Record(n) => format!("global::App.Forms.{n}.ToText({a})"),
        _ => a.to_string(),
    }
}

/// Os enums em C#, com os nomes que a tela entende (`EnumNames`).
fn csharp_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums {
        s.push_str(&format!("    /// <summary>Os valores de um enum do acesso tipado.</summary>\n    public enum {name}\n    {{\n"));
        for v in variants(&def.values, variant_pascal) {
            s.push_str(&format!("        {v},\n"));
        }
        s.push_str("    }\n\n");
    }
    s.push_str("    /// <summary>Os nomes que a tela entende de cada enum do acesso tipado.</summary>\n    internal static class EnumNames\n    {\n");
    for (name, def) in enums {
        let values: Vec<String> = def.values.iter().map(|v| format!("\"{v}\"")).collect();
        s.push_str(&format!("        public static readonly string[] {name} = [{}];\n", values.join(", ")));
    }
    s.push_str(
        "\n        public static T Of<T>(JsonNode? v, string[] names) where T : struct, System.Enum\n        {\n            var i = System.Array.IndexOf(names, Components.Text(v));\n            return (T)(object)(i < 0 ? 0 : i);\n        }\n\n\
         \x20       public static T[] SetOf<T>(JsonNode? v, string[] names) where T : struct, System.Enum =>\n            v is JsonArray a ? a.Select(x => System.Array.IndexOf(names, Components.Text(x))).Where(i => i >= 0).Select(i => (T)(object)i).ToArray() : [];\n\n\
         \x20       public static JsonArray Names<T>(System.Collections.Generic.IEnumerable<T> values, string[] names) where T : struct, System.Enum =>\n            new(values.Select(v => (JsonNode?)names[(int)(object)v]).ToArray());\n    }\n\n",
    );
    s
}

fn csharp(m: &Model) -> Out {
    let s = format!("// {}\n\nusing System.Globalization;\nusing System.Linq;\nusing System.Text.Json.Nodes;\nusing AylisClient;\n\nnamespace App.Forms\n{{\n", HEADER.replace('\n', "\n// "));
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    let mut s_text = String::new();
    let s_ref = &mut s_text;
    s_ref.push_str(
        "    /// <summary>Os forms (<c>using static App.Forms.Components;</c>): cada frmX é a janela do\n    /// form cujo evento está rodando (a primeira, fora dos handlers).</summary>\n    public static class Components\n    {\n",
    );
    s_ref.push_str("        internal static bool Bool(JsonNode? v) => v is JsonValue j && j.TryGetValue<bool>(out var b) && b;\n");
    s_ref.push_str("        internal static double Number(JsonNode? v) => v is JsonValue j && j.TryGetValue<double>(out var d) ? d : 0;\n");
    s_ref.push_str("        internal static string Text(JsonNode? v) => v is JsonValue j && j.TryGetValue<string>(out var t) ? t : v?.ToJsonString() ?? \"\";\n");
    s_ref.push_str(CS_HELPERS);
    s.push_str(&s_text);
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!("\n        public static AylisUi.{ty} {} = null!;\n", f.name, ty = form_type(&f.name)));
    }
    s.end();
    s.push_str("\n        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>\n        public static void RegisterClasses()\n        {\n");
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!(
            "            Aylis.RegisterFormClass(\"{name}\", (aylis, id, name) => new AylisUi.{ty}(aylis, id, name), form => {name} = (AylisUi.{ty})form);\n",
            name = f.name,
            ty = form_type(&f.name)
        ));
    }
    s.end();
    s.push_str("        }\n    }\n}\n\nnamespace App.Forms.AylisUi\n{\n    using static App.Forms.Components;\n");
    // O que a base (AylisComponent, AylisForm) já tem: as propriedades com o mesmo nome escondem
    // as dela (`new`), os métodos ficam de fora.
    const BASE_PROPS: [&str; 3] = ["Name", "Id", "Aylis"];
    const BASE_METHODS: [&str; 3] = ["Show", "Close", "SetFocus"];
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let base = if class == "Form" { "AylisForm" } else { "AylisComponent" };
        s.push_str(&format!("\n    /// <summary>Um {class} de um form.</summary>\n    public class {class}(Aylis aylis, long id, string name) : {base}(aylis, id, name)\n    {{\n"));
        for p in props {
            let mut name = pascal(&p.name);
            if name == *class {
                name.push_str("Value");
            }
            let hide = if BASE_PROPS.contains(&name.as_str()) { "new " } else { "" };
            let ty = cs_ty(&p.kind);
            let get = match &p.kind {
                Kind::Component => format!("GetRef(\"{}\")", p.name),
                k => cs_from(&format!("GetProp(\"{}\")", p.name), k),
            };
            let set = match &p.kind {
                Kind::Component => format!("SetRef(\"{}\", value)", p.name),
                k => format!("SetProp(\"{}\", {})", p.name, cs_value("value", k)),
            };
            if p.read_only {
                s.push_str(&format!("        public {hide}{ty} {name} => {get};\n"));
            } else {
                s.push_str(&format!("        public {hide}{ty} {name} {{ get => {get}; set => {set}; }}\n"));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !BASE_METHODS.contains(&pascal(&mth.name).as_str())) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", if *k == Kind::Component { "AylisComponent".to_string() } else { cs_ty(k) })).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| cs_value(a, k)).collect();
            let list = if args.is_empty() { "new JsonArray()".to_string() } else { format!("new JsonArray({})", values.join(", ")) };
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", [\"{k}\"] = {v}")).collect();
            let request = format!("(\"{}\", new JsonObject {{ [\"target\"] = Id{fields} }})", mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (cs_ty(k), cs_from(&format!("Aylis.Call{request}"), k)),
                None => ("void".to_string(), format!("Aylis.Send{request}")),
            };
            s.push_str(&format!("        public {ret} {}({}) => {body};\n", pascal(&mth.name), params.join(", ")));
        }
        s.push_str("    }\n");
    }
    s.end();
    let form_props = m.classes.iter().find(|(c, _)| c == "Form").map(|(_, p)| p.clone()).unwrap_or_default();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_type(&f.name);
        let fs = fields(f, &form_props, |p, n| pascal(p) == n);
        s.push_str(&format!(
            "\n    /// <summary>O form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>\n    public sealed class {ty}(Aylis aylis, long id, string name) : Form(aylis, id, name)\n    {{\n",
            f.name
        ));
        for (n, c) in &fs {
            let opt = if f.optional.contains(n) { "?" } else { "" };
            s.push_str(&format!("        public {c}{opt} {n} = null!;\n"));
        }
        s.push_str("\n        public override void Bind(JsonArray list)\n        {\n");
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("            {n} = ResolveOptional(list, \"{n}\", (i, d, n) => new {c}(i, d, n));\n"));
            } else {
                s.push_str(&format!("            {n} = Resolve(list, \"{n}\", \"{c}\", (i, d, n) => new {c}(i, d, n));\n"));
            }
        }
        s.push_str("        }\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!("\n        /// <summary>O {class} deste form com o nome.</summary>\n        public {class}? Find{class}(string name) => name switch\n        {{\n"));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                s.push_str(&format!("            \"{n}\" => {n},\n"));
            }
            s.push_str(&format!(
                "            _ => null,\n        }};\n\n        /// <summary>O {class} que disparou o evento.</summary>\n        public {class}? SenderAs{class}(AylisClient.AylisEvent ev) => Find{class}(ev.SenderName);\n"
            ));
        }
        s.push_str("    }\n");
    }
    s.end();
    s.push_str("}\n");
    s
}

// ---------------------------------------------------------------------------
// Java (getX/isX/setX)

/// O tipo de um valor em Java.
fn java_ty(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "boolean".into(),
        Kind::Int => "long".into(),
        Kind::Float => "double".into(),
        Kind::Text => "String".into(),
        Kind::Color => "Aylis.Color".into(),
        Kind::Bounds => "Aylis.Bounds".into(),
        Kind::Component => "Aylis.AylisComponent".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("java.util.List<{n}>"),
        Kind::Date => "java.time.LocalDate".into(),
        Kind::Time => "java.time.LocalTime".into(),
        Kind::DateTime => "java.time.LocalDateTime".into(),
        Kind::List => "java.util.List<String>".into(),
        Kind::NumberList => "java.util.List<Double>".into(),
        Kind::ColorList => "java.util.List<Aylis.Color>".into(),
        Kind::Shadow => "Aylis.Shadow".into(),
        Kind::Easing => "Aylis.Easing".into(),
        Kind::PointList => "java.util.List<Aylis.Point>".into(),
        Kind::Record(n) => n.clone(),
        Kind::RecordList(n) => format!("java.util.List<{n}>"),
    }
}

/// O valor `v` da tela como o tipo `kind`, em Java (dentro de um componente).
fn java_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("asBool({v})"),
        Kind::Int => format!("(long) asNumber({v})"),
        Kind::Float => format!("asNumber({v})"),
        Kind::Text => format!("asText({v})"),
        Kind::Color => format!("Aylis.Color.fromJson({v})"),
        Kind::Bounds => format!("Aylis.Bounds.fromJson({v})"),
        Kind::Component => format!("componentOf({v})"),
        Kind::Enum(n) => format!("{n}.of({v})"),
        Kind::Set(n) => format!("{n}.listOf({v})"),
        Kind::Date => format!("asDate({v})"),
        Kind::Time => format!("asTime({v})"),
        Kind::DateTime => format!("asDateTime({v})"),
        Kind::List => format!("asList({v})"),
        Kind::NumberList => format!("asNumbers({v})"),
        Kind::ColorList => format!("asColors({v})"),
        Kind::Shadow => format!("Aylis.Shadow.fromJson({v})"),
        Kind::Easing => format!("Aylis.Easing.fromJson({v})"),
        Kind::PointList => format!("Aylis.Point.listFromJson({v})"),
        Kind::Record(n) => format!("{n}.fromText({v})"),
        Kind::RecordList(n) => format!("{n}.listFromText({v})"),
    }
}

/// O valor `a` do tipo `kind` como vai para a tela, em Java.
fn java_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.toJson()"),
        Kind::Component => format!("{a}.id"),
        Kind::Enum(_) => format!("{a}.value"),
        Kind::Set(n) => format!("{n}.names({a})"),
        Kind::Date => format!("dateText({a})"),
        Kind::Time => format!("timeText({a})"),
        Kind::DateTime => format!("dateTimeText({a})"),
        Kind::Shadow => format!("({a} == null ? \"\" : {a}.toJson())"),
        Kind::Easing => format!("{a}.toJson()"),
        Kind::PointList => format!("Aylis.Point.listToJson({a})"),
        Kind::RecordList(n) => format!("{n}.listToText({a})"),
        Kind::Record(n) => format!("{n}.toText({a})"),
        Kind::List | Kind::NumberList | Kind::ColorList => format!("listText({a})"),
        _ => a.to_string(),
    }
}

/// Os enums em Java (dentro de `Components`), com o nome que a tela entende em `value`.
fn java_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums {
        let items: Vec<String> = def.values.iter().zip(variants(&def.values, variant_upper)).map(|(v, id)| format!("{id}(\"{v}\")")).collect();
        s.push_str(&format!(
            "\n    /** Os valores de um enum do acesso tipado. */\n    public enum {name} {{\n        {};\n\n        public final String value;\n\n        {name}(String value) {{\n            this.value = value;\n        }}\n\n\
             \x20       public static {name} of(Object v) {{\n            for ({name} e : values()) if (e.value.equals(v)) return e;\n            return values()[0];\n        }}\n\n\
             \x20       public static java.util.List<{name}> listOf(Object v) {{\n            java.util.List<{name}> out = new java.util.ArrayList<>();\n            if (v instanceof java.util.List<?> l) for (Object x : l) for ({name} e : values()) if (e.value.equals(x)) out.add(e);\n            return out;\n        }}\n\n\
             \x20       public static java.util.List<String> names(java.util.Collection<{name}> values) {{\n            java.util.List<String> out = new java.util.ArrayList<>();\n            for ({name} e : values) out.add(e.value);\n            return out;\n        }}\n    }}\n",
            items.join(",\n        ")
        ));
    }
    s
}

fn java(m: &Model) -> Out {
    let mut s = format!("// {}\n\npackage forms;\n\nimport dev.aylis.Aylis;\n\n", HEADER.replace('\n', "\n// "));
    s.push_str(
        "/** Os forms ({@code import static forms.Components.frmX;}: a janela do form cujo evento está\n *  rodando, a primeira fora dos handlers) e as classes dos componentes. */\npublic final class Components {\n    private Components() {}\n\n",
    );
    s.push_str("    static boolean asBool(Object v) {\n        return v instanceof Boolean b && b;\n    }\n\n    static double asNumber(Object v) {\n        return v instanceof Number n ? n.doubleValue() : 0;\n    }\n\n    static String asText(Object v) {\n        return v == null ? \"\" : v.toString();\n    }\n");
    s.push_str(JAVA_HELPERS);
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!("\n    public static {} {};\n", form_type(&f.name), f.name));
    }
    s.end();
    s.push_str("\n    /** Registra as classes dos forms (o registro dos forms chama). */\n    public static void registerClasses() {\n");
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!(
            "        Aylis.registerFormClass(\"{name}\", {ty}::new, form -> {name} = ({ty}) form);\n",
            name = f.name,
            ty = form_type(&f.name)
        ));
    }
    s.end();
    s.push_str("    }\n");
    const BASE_METHODS: [&str; 3] = ["show", "close", "setFocus"];
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let base = if class == "Form" { "Aylis.AylisForm" } else { "Aylis.AylisComponent" };
        s.push_str(&format!(
            "\n    /** Um {class} de um form. */\n    public static class {class} extends {base} {{\n        public {class}(Aylis aylis, long id, String name) {{\n            super(aylis, id, name);\n        }}\n"
        ));
        for p in props {
            let name = pascal(&p.name);
            let ty = java_ty(&p.kind);
            let prefix = if p.kind == Kind::Bool { "is" } else { "get" };
            let get = match &p.kind {
                Kind::Component => format!("getRef(\"{}\")", p.name),
                k => java_from(&format!("getProp(\"{}\")", p.name), k),
            };
            s.push_str(&format!("\n        public {ty} {prefix}{name}() {{\n            return {get};\n        }}\n"));
            if !p.read_only {
                let set = match &p.kind {
                    Kind::Component => format!("setRef(\"{}\", value)", p.name),
                    k => format!("setProp(\"{}\", {})", p.name, java_value("value", k)),
                };
                s.push_str(&format!("\n        public void set{name}({ty} value) {{\n            {set};\n        }}\n"));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !BASE_METHODS.contains(&camel(&mth.name).as_str())) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", java_ty(k))).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| java_value(a, k)).collect();
            let list = format!("java.util.List.of({})", values.join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", \"{k}\", {v}")).collect();
            let request = format!("(\"{}\", Aylis.obj(\"target\", id{fields}))", mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (java_ty(k), format!("return {};", java_from(&format!("aylis.call{request}"), k))),
                None => ("void".to_string(), format!("aylis.send{request};")),
            };
            let name = match camel(&mth.name).as_str() {
                n @ ("new" | "default" | "switch" | "case" | "class" | "package" | "import" | "throw" | "final") => format!("{n}_"),
                n => n.to_string(),
            };
            s.push_str(&format!("\n        public {ret} {name}({}) {{\n            {body}\n        }}\n", params.join(", ")));
        }
        s.push_str("    }\n");
    }
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_type(&f.name);
        s.push_str(&format!(
            "\n    /** O form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */\n    public static final class {ty} extends Form {{\n",
            f.name
        ));
        for (n, c) in &f.components {
            s.push_str(&format!("        public {c} {n};\n"));
        }
        s.push_str(&format!("\n        public {ty}(Aylis aylis, long id, String name) {{\n            super(aylis, id, name);\n        }}\n\n        @Override\n        public void bind(java.util.List<java.util.Map<String, Object>> list) {{\n"));
        for (n, c) in &f.components {
            if f.optional.contains(n) {
                s.push_str(&format!("            {n} = resolveOptional(list, \"{n}\", {c}::new);\n"));
            } else {
                s.push_str(&format!("            {n} = resolve(list, \"{n}\", \"{c}\", {c}::new);\n"));
            }
        }
        s.push_str("        }\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!("\n        /** O {class} deste form com o nome. */\n        public {class} find{class}(String name) {{\n            return switch (name) {{\n"));
            for (n, _) in f.components.iter().filter(|(_, c)| c == class) {
                s.push_str(&format!("                case \"{n}\" -> {n};\n"));
            }
            s.push_str(&format!(
                "                default -> null;\n            }};\n        }}\n\n        /** O {class} que disparou o evento. */\n        public {class} senderAs{class}(Aylis.Event ev) {{\n            return find{class}(String.valueOf(ev.sender().get(\"name\")));\n        }}\n"
            ));
        }
        s.push_str("    }\n");
    }
    s.end();
    s.push_str("}\n");
    s
}

// ---------------------------------------------------------------------------
// Go (Prop() e SetProp(v))

/// O tipo de um valor em Go (`setter` é o do argumento de gravar: a referência aceita
/// qualquer componente).
fn go_ty(kind: &Kind, setter: bool) -> String {
    match kind {
        Kind::Bool => "bool".into(),
        Kind::Int => "int64".into(),
        Kind::Float => "float64".into(),
        Kind::Text => "string".into(),
        Kind::Color => "aylis.Color".into(),
        Kind::Bounds => "aylis.Bounds".into(),
        Kind::Component if setter => "aylis.Ref".into(),
        Kind::Component => "*aylis.Component".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("[]{n}"),
        Kind::Date | Kind::Time | Kind::DateTime => "time.Time".into(),
        Kind::List => "[]string".into(),
        Kind::NumberList => "[]float64".into(),
        Kind::ColorList => "[]aylis.Color".into(),
        Kind::Shadow => "*aylis.Shadow".into(),
        Kind::Easing => "aylis.Easing".into(),
        Kind::PointList => "[]aylis.Point".into(),
        Kind::Record(n) => format!("*{n}"),
        Kind::RecordList(n) => format!("[]{n}"),
    }
}

/// O valor `v` da tela como o tipo `kind`, em Go (dentro de um componente `c`).
fn go_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("asBool({v})"),
        Kind::Int => format!("int64(asNumber({v}))"),
        Kind::Float => format!("asNumber({v})"),
        Kind::Text => format!("asText({v})"),
        Kind::Color => format!("aylis.ColorFromJSON({v})"),
        Kind::Bounds => format!("aylis.BoundsFromJSON({v})"),
        Kind::Component => format!("c.ComponentOf({v})"),
        Kind::Enum(n) => format!("{n}(asText({v}))"),
        Kind::Set(n) => format!("enumList{n}({v})"),
        Kind::Date => format!("asDate({v})"),
        Kind::Time => format!("asTime({v})"),
        Kind::DateTime => format!("asDateTime({v})"),
        Kind::List => format!("asList({v})"),
        Kind::NumberList => format!("asNumbers({v})"),
        Kind::ColorList => format!("asColors({v})"),
        Kind::Shadow => format!("aylis.ShadowFromJSON({v})"),
        Kind::Easing => format!("aylis.EasingFromJSON({v})"),
        Kind::PointList => format!("aylis.PointsFromJSON({v})"),
        Kind::Record(n) => format!("record{n}({v})"),
        Kind::RecordList(n) => format!("records{n}({v})"),
    }
}

/// O valor `a` do tipo `kind` como vai para a tela, em Go.
fn go_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.JSON()"),
        Kind::Component => format!("{a}.ComponentID()"),
        Kind::Enum(_) => format!("string({a})"),
        Kind::Set(n) => format!("enumNames{n}({a})"),
        Kind::Date => format!("dateText({a})"),
        Kind::Time => format!("timeText({a})"),
        Kind::DateTime => format!("dateTimeText({a})"),
        Kind::List => format!("strings.Join({a}, \";\")"),
        Kind::NumberList => format!("numbersText({a})"),
        Kind::ColorList => format!("colorsText({a})"),
        Kind::Shadow | Kind::Easing => format!("{a}.JSON()"),
        Kind::PointList => format!("aylis.PointsJSON({a})"),
        Kind::RecordList(n) => format!("recordsText{n}({a})"),
        Kind::Record(n) => format!("recordText{n}({a})"),
        _ => a.to_string(),
    }
}

/// Os enums em Go: um tipo `string` com as constantes (`WindowStateFullScreen`).
fn go_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for (name, def) in enums {
        s.push_str(&format!("\n// {name} são os valores de um enum do acesso tipado.\ntype {name} string\n\nconst (\n"));
        for (v, id) in def.values.iter().zip(variants(&def.values, variant_pascal)) {
            s.push_str(&format!("\t{name}{id} {name} = \"{v}\"\n"));
        }
        s.push_str(&format!(
            ")\n\nfunc enumList{name}(v any) []{name} {{\n\tvar out []{name}\n\tif l, ok := v.([]any); ok {{\n\t\tfor _, x := range l {{\n\t\t\tout = append(out, {name}(asText(x)))\n\t\t}}\n\t}}\n\treturn out\n}}\n\n\
             func enumNames{name}(vs []{name}) []string {{\n\tout := make([]string, len(vs))\n\tfor i, v := range vs {{\n\t\tout[i] = string(v)\n\t}}\n\treturn out\n}}\n"
        ));
    }
    s
}

fn go(m: &Model) -> Out {
    let mut s = format!("// {}\n\npackage forms\n\nimport (\n\t\"encoding/json\"\n\t\"fmt\"\n\t\"strconv\"\n\t\"strings\"\n\t\"time\"\n\n\t\"aylis.dev/cliente/aylis\"\n)\n\n", HEADER.replace('\n', "\n// "));
    s.push_str("var _ = fmt.Sprint\n\n");
    s.push_str("func asBool(v any) bool {\n\tb, _ := v.(bool)\n\treturn b\n}\n\nfunc asNumber(v any) float64 {\n\tswitch n := v.(type) {\n\tcase float64:\n\t\treturn n\n\tcase int:\n\t\treturn float64(n)\n\tcase int64:\n\t\treturn float64(n)\n\t}\n\treturn 0\n}\n\n");
    s.push_str("func asText(v any) string {\n\tif t, ok := v.(string); ok {\n\t\treturn t\n\t}\n\tif v == nil {\n\t\treturn \"\"\n\t}\n\treturn fmt.Sprint(v)\n}\n");
    s.push_str("\nfunc askCmd(app *aylis.Aylis, cmd string, fields aylis.M) any {\n\tv, err := app.Call(cmd, fields)\n\tif err != nil {\n\t\taylis.Log(cmd+\":\", err)\n\t}\n\treturn v\n}\n");
    s.push_str(GO_HELPERS);
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let ty = format!("aylis{class}");
        let embed = if class == "Form" { "aylis.Form" } else { "aylis.Component" };
        s.push_str(&format!("\n// {ty} é um {class} de um form.\ntype {ty} struct{{ {embed} }}\n"));
        for p in props {
            let name = pascal(&p.name);
            let t = go_ty(&p.kind, false);
            let get = match &p.kind {
                Kind::Component => format!("c.GetRef(\"{}\")", p.name),
                k => go_from(&format!("c.Get(\"{}\")", p.name), k),
            };
            // O nome do componente na tela já é o campo Name do Component.
            if name == "Name" {
                continue;
            }
            s.push_str(&format!("\nfunc (c {ty}) {name}() {t} {{ return {get} }}\n"));
            if !p.read_only {
                let set = match &p.kind {
                    Kind::Component => format!("c.SetRef(\"{}\", v)", p.name),
                    k => format!("c.Set(\"{}\", {})", p.name, go_value("v", k)),
                };
                s.push_str(&format!("func (c {ty}) Set{name}(v {}) {{ {set} }}\n", go_ty(&p.kind, true)));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !matches!(pascal(&mth.name).as_str(), "Show" | "Close" | "Focus" | "Get" | "Set" | "GetRef" | "SetRef" | "ComponentOf")) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{a} {}", go_ty(k, true))).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| go_value(a, k)).collect();
            let list = format!("[]any{{{}}}", values.join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", \"{k}\": {v}")).collect();
            let request = format!("(c.App, \"{}\", aylis.M{{\"target\": c.ID{fields}}})", mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (format!(" {}", go_ty(k, false)), format!("return {}", go_from(&format!("askCmd{request}"), k))),
                None => (String::new(), format!("c.App.Send(\"{}\", aylis.M{{\"target\": c.ID{fields}}})", mth.command())),
            };
            s.push_str(&format!("func (c {ty}) {}({}){ret} {{ {body} }}\n", pascal(&mth.name), params.join(", ")));
        }
    }
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = format!("{}Components", f.name);
        s.push_str(&format!("\n// {ty}: o form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).\ntype {ty} struct {{\n\taylisForm\n", f.name));
        for (n, c) in &f.components {
            if f.optional.contains(n) {
                s.push_str(&format!("\t{n} *aylis{c}\n"));
            } else {
                s.push_str(&format!("\t{n} aylis{c}\n"));
            }
        }
        s.push_str(&format!("}}\n\n// Bind monta os componentes da janela (o form que não bate com o código é erro).\nfunc (f *{ty}) Bind(list []any) error {{\n"));
        for (n, c) in &f.components {
            if f.optional.contains(n) {
                s.push_str(&format!("\tif c, ok := f.ResolveOptional(list, \"{n}\"); ok {{\n\t\tf.{n} = &aylis{c}{{c}}\n\t}}\n"));
            } else {
                s.push_str(&format!("\tif c, err := f.Resolve(list, \"{n}\", \"{c}\"); err != nil {{\n\t\treturn err\n\t}} else {{\n\t\tf.{n} = aylis{c}{{c}}\n\t}}\n"));
            }
        }
        s.push_str("\treturn nil\n}\n");
        s.push_str(&format!(
            "\n// {name} é a janela de {name} cujo evento está rodando (a primeira, fora dos handlers).\nvar {name} *{ty}\n\nfunc init() {{\n\taylis.RegisterFormClass(\"{name}\", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {{\n\t\treturn &{ty}{{aylisForm: aylisForm{{aylis.Form{{Component: aylis.Component{{App: app, ID: id, Name: name}}}}}}}}\n\t}}, func(form aylis.FormInstance) {{ {name} = form.(*{ty}) }})\n}}\n",
            name = f.name
        ));
        for class in Model::classes_of(f) {
            s.push_str(&format!("\n// Find{class} é o {class} deste form com o nome.\nfunc (f *{ty}) Find{class}(name string) aylis{class} {{\n\tswitch name {{\n"));
            for (n, _) in f.components.iter().filter(|(_, c)| c == class) {
                if f.optional.contains(n) {
                    s.push_str(&format!("\tcase \"{n}\":\n\t\tif f.{n} != nil {{\n\t\t\treturn *f.{n}\n\t\t}}\n"));
                } else {
                    s.push_str(&format!("\tcase \"{n}\":\n\t\treturn f.{n}\n"));
                }
            }
            s.push_str(&format!(
                "\t}}\n\treturn aylis{class}{{}}\n}}\n\n// SenderAs{class} é o {class} que disparou o evento.\nfunc (f *{ty}) SenderAs{class}(ev aylis.Event) aylis{class} {{ return f.Find{class}(ev.Sender.Name) }}\n"
            ));
        }
    }
    s.end();
    s
}

// ---------------------------------------------------------------------------
// C++ (prop() e set_prop(v))

/// O tipo de um valor em C++.
fn cpp_ty(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "bool".into(),
        Kind::Int => "long long".into(),
        Kind::Float => "double".into(),
        Kind::Text => "std::string".into(),
        Kind::Color => "aylis::Color".into(),
        Kind::Bounds => "aylis::Bounds".into(),
        Kind::Component => "std::optional<aylis::Component>".into(),
        Kind::Enum(n) => n.clone(),
        Kind::Set(n) => format!("std::vector<{n}>"),
        Kind::Date => "std::optional<LocalDate>".into(),
        Kind::Time => "std::optional<LocalTime>".into(),
        Kind::DateTime => "std::optional<LocalDateTime>".into(),
        Kind::List => "std::vector<std::string>".into(),
        Kind::NumberList => "std::vector<double>".into(),
        Kind::ColorList => "std::vector<aylis::Color>".into(),
        Kind::Shadow => "std::optional<aylis::Shadow>".into(),
        Kind::Easing => "aylis::Easing".into(),
        Kind::PointList => "std::vector<aylis::Point>".into(),
        Kind::Record(n) => format!("std::optional<{n}>"),
        Kind::RecordList(n) => format!("std::vector<{n}>"),
    }
}

/// O tipo de um argumento em C++ (o que não é número, por referência).
fn cpp_arg(kind: &Kind, method: bool) -> String {
    match kind {
        Kind::Bool | Kind::Int | Kind::Float | Kind::Enum(_) => cpp_ty(kind),
        Kind::Component if method => "const aylis::Component&".into(),
        k => format!("const {}&", cpp_ty(k)),
    }
}

/// O `Json` `v` da tela como o tipo `kind`, em C++ (dentro de um componente).
fn cpp_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("{v}.boolean()"),
        Kind::Int => format!("static_cast<long long>({v}.number())"),
        Kind::Float => format!("{v}.number()"),
        Kind::Text => format!("{v}.str()"),
        Kind::Color => format!("aylis::Color::from_json({v})"),
        Kind::Bounds => format!("aylis::Bounds::from_json({v})"),
        Kind::Component => format!("component_of_({v})"),
        Kind::Enum(n) => format!("enum_of_<{n}>({v}, {n}_names)"),
        Kind::Set(n) => format!("enum_list_<{n}>({v}, {n}_names)"),
        Kind::Date => format!("LocalDate::parse({v}.str())"),
        Kind::Time => format!("LocalTime::parse({v}.str())"),
        Kind::DateTime => format!("LocalDateTime::parse({v}.str())"),
        Kind::List => format!("list_of_({v})"),
        Kind::NumberList => format!("numbers_of_({v})"),
        Kind::ColorList => format!("colors_of_({v})"),
        Kind::Shadow => format!("aylis::Shadow::from_json({v})"),
        Kind::Easing => format!("aylis::Easing::from_json({v})"),
        Kind::PointList => format!("aylis::Point::list_from_json({v})"),
        Kind::Record(n) => format!("{n}::from_text({v})"),
        Kind::RecordList(n) => format!("{n}::list_from_text({v})"),
    }
}

/// O valor `a` do tipo `kind` como `Json` para a tela, em C++.
fn cpp_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Color | Kind::Bounds => format!("{a}.to_json()"),
        Kind::Component => format!("{a}.id()"),
        Kind::Enum(n) => format!("aylis::Json({n}_names[static_cast<int>({a})])"),
        Kind::Set(n) => format!("enum_names_({a}, {n}_names)"),
        Kind::Date | Kind::Time | Kind::DateTime => format!("text_of_({a})"),
        Kind::List => format!("join_({a})"),
        Kind::NumberList => format!("numbers_text_({a})"),
        Kind::ColorList => format!("colors_text_({a})"),
        Kind::Shadow => format!("({a} ? {a}->to_json() : std::string())"),
        Kind::Easing => format!("{a}.to_json()"),
        Kind::PointList => format!("aylis::Point::list_to_json({a})"),
        Kind::RecordList(n) => format!("{n}::list_to_text({a})"),
        Kind::Record(n) => format!("{n}::to_text({a})"),
        _ => a.to_string(),
    }
}

/// Os enums em C++ (`enum class`, com os nomes que a tela entende em `X_names`).
fn cpp_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::from(
        "\n// O valor da tela como o enum (o primeiro, se não for um dos nomes).\ntemplate <class E, size_t N>\nE enum_of_(const aylis::Json& v, const char* const (&names)[N]) {\n    std::string s = v.str();\n    for (size_t i = 0; i < N; ++i)\n        if (s == names[i]) return static_cast<E>(i);\n    return static_cast<E>(0);\n}\n\n\
         template <class E, size_t N>\nstd::vector<E> enum_list_(const aylis::Json& v, const char* const (&names)[N]) {\n    std::vector<E> out;\n    for (const auto& x : v.items()) {\n        std::string s = x.str();\n        for (size_t i = 0; i < N; ++i)\n            if (s == names[i]) out.push_back(static_cast<E>(i));\n    }\n    return out;\n}\n\n\
         template <class E, size_t N>\naylis::Json enum_names_(const std::vector<E>& values, const char* const (&names)[N]) {\n    std::vector<aylis::Json> out;\n    for (E v : values) out.emplace_back(names[static_cast<int>(v)]);\n    return aylis::Json::array(out);\n}\n",
    );
    for (name, def) in enums {
        let values: Vec<String> = def.values.iter().map(|v| format!("\"{v}\"")).collect();
        s.push_str(&format!(
            "\n// Os valores de um enum do acesso tipado.\nenum class {name} {{ {} }};\ninline constexpr const char* {name}_names[] = {{{}}};\ninline const char* to_string({name} v) {{ return {name}_names[static_cast<int>(v)]; }}\n",
            variants(&def.values, variant_pascal).join(", "),
            values.join(", ")
        ));
    }
    s
}

fn cpp(m: &Model) -> Out {
    let s = format!(
        "// {}\n\n#pragma once\n\n#include <cstdio>\n#include <cstdlib>\n#include <memory>\n#include <optional>\n#include <sstream>\n#include <string>\n#include <vector>\n\n#include \"../aylis.hpp\"\n\nnamespace aylis_components {{\n{CPP_HELPERS}",
        HEADER.replace('\n', "\n// ")
    );
    let mut s = Out::from(s);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let base = if class == "Form" { "aylis::FormBase" } else { "aylis::Component" };
        s.push_str(&format!("\n// Um {class} de um form.\nstruct {class} : {base} {{\n    {class}() = default;\n"));
        if class == "Form" {
            s.push_str("    using aylis::FormBase::FormBase;\n");
        } else {
            s.push_str(&format!("    explicit {class}(const aylis::Component& c) : aylis::Component(c) {{}}\n"));
        }
        for p in props {
            let name = cpp_ident(&p.name);
            let ty = cpp_ty(&p.kind);
            let get = match &p.kind {
                Kind::Component => format!("get_ref_(\"{}\")", p.name),
                k => cpp_from(&format!("get_prop_(\"{}\")", p.name), k),
            };
            s.push_str(&format!("    {ty} {name}() const {{ return {get}; }}\n"));
            if !p.read_only {
                let set = match &p.kind {
                    Kind::Component => format!("set_ref_(\"{}\", v)", p.name),
                    k => format!("set_prop_(\"{}\", {})", p.name, cpp_value("v", k)),
                };
                s.push_str(&format!("    void set_{}({} v) const {{ {set}; }}\n", p.name, cpp_arg(&p.kind, false)));
            }
        }
        for mth in m.methods(class) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", cpp_arg(k, true))).collect();
            let values: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| cpp_value(a, k)).collect();
            let list = format!("aylis::Json::array({{{}}})", values.join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", {{\"{k}\", {v}}}")).collect();
            let request = format!("(\"{}\", {{{{\"target\", aylis::Json(id_)}}{fields}}})", mth.command());
            let (ret, body) = match &mth.returns {
                Some(k) => (cpp_ty(k), format!("return {};", cpp_from(&format!("ask_{request}"), k))),
                None => ("void".to_string(), format!("tell_{request};")),
            };
            s.push_str(&format!("    {ret} {}({}) const {{ {body} }}\n", cpp_ident(&mth.name), params.join(", ")));
        }
        s.push_str("};\n");
    }
    s.end();
    let form_props = m.classes.iter().find(|(c, _)| c == "Form").map(|(_, p)| p.clone()).unwrap_or_default();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let ty = form_type(&f.name);
        let fs = fields(f, &form_props, |p, n| cpp_ident(p) == n);
        s.push_str(&format!("\n// O form {} e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).\nstruct {ty} : Form {{\n", f.name));
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("    std::optional<{c}> {n};\n"));
            } else {
                s.push_str(&format!("    {c} {n};\n"));
            }
        }
        s.push_str(&format!("\n    {ty}() = default;\n    using Form::Form;\n\n    // Monta os componentes da janela (o form que não bate com o código é erro).\n    void bind(const aylis::Json& list) override {{\n"));
        if fs.is_empty() {
            s.push_str("        (void)list;\n");
        }
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("        if (auto c = resolve_optional_(list, \"{n}\")) {n} = {c}(c);\n"));
            } else {
                s.push_str(&format!("        {n} = {c}(resolve_(list, \"{n}\", \"{c}\"));\n"));
            }
        }
        s.push_str("    }\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!("    // O {class} deste form com o nome (vazio se não há).\n    {class} find_{sn}(const std::string& name) const {{\n", sn = snake(class)));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                if f.optional.contains(n) {
                    s.push_str(&format!("        if (name == \"{n}\") return {n} ? *{n} : {class}();\n"));
                } else {
                    s.push_str(&format!("        if (name == \"{n}\") return {n};\n"));
                }
            }
            s.push_str(&format!(
                "        return {class}();\n    }}\n    // O {class} que disparou o evento.\n    {class} sender_as_{sn}(const aylis::Event& ev) const {{ return find_{sn}(ev.sender_name); }}\n",
                sn = snake(class)
            ));
        }
        s.push_str("};\n");
    }
    s.end();
    s.push_str("\n}  // namespace aylis_components\n\n// Cada frmX é a janela do form cujo evento está rodando (a primeira, fora dos handlers): a\n// instância é copiada para cá antes do handler, então &frmX.componente vale sempre.\n");
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!("inline aylis_components::{} {};\n", form_type(&f.name), f.name));
    }
    s.end();
    s.push_str("\nnamespace aylis_components {\n\n// Registra as classes dos forms (o registro dos forms chama).\ninline void register_classes() {\n");
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        s.push_str(&format!(
            "    aylis::Aylis::register_form_class(\n        \"{name}\", [](aylis::Aylis* app, long long id, const std::string& name) -> std::unique_ptr<aylis::FormBase> {{ return std::make_unique<{ty}>(app, id, name); }},\n        [](aylis::FormBase& form) {{ ::{name} = static_cast<{ty}&>(form); }});\n",
            name = f.name,
            ty = form_type(&f.name)
        ));
    }
    s.end();
    s.push_str("}\n\n}  // namespace aylis_components\n");
    s
}


// ---------------------------------------------------------------------------
// Free Pascal (propriedades)

fn pascal_unit(m: &Model) -> Out {
    let s = String::from(
        "{ Os componentes dos forms com as propriedades tipadas e a classe base de cada form (com os\n  \
         componentes como campos published, montados quando a tela abre o form). A classe de cada\n  \
         form, com os handlers e o estado, fica na unit dele.\n  \
         Gerado pela IDE do Aylis a cada \"Gerar código\": não edite. }\n\
         unit ayliscomponents;\n\n{$mode objfpc}{$H+}\n\ninterface\n\nuses\n  SysUtils, DateUtils, fpjson, jsonparser, aylis;\n\ntype\n  TAylisDoubleArray = array of Double;\n  TAylisColorArray = array of TAylisColor;\n  { O intervalo de datas (o on_change do DateRangePicker); 0: sem a data. }\n  TAylisDateRange = record\n    StartDate, EndDate: TDateTime;\n  end;\n\n",
    );
    let mut s = Out::from(s);
    // Os enums (os tipos aqui; os nomes e as conversões no implementation).
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let t = format!("T{class}");
        // O Form é a base das classes dos forms (o Show e o Close já vêm do TAylisForm).
        let base = if class == "Form" { "TAylisForm" } else { "TAylisComponent" };
        s.push_str(&format!("  {{ Um {class} de um form. }}\n  {t} = class({base})\n  private\n"));
        for p in props {
            let name = pascal(&p.name);
            let ty = pascal_type(&p.kind);
            s.push_str(&format!("    function Get{name}: {ty};\n"));
            if !p.read_only {
                s.push_str(&format!("    procedure Set{name}(Value: {ty});\n"));
            }
        }
        s.push_str("  public\n");
        for mth in pascal_methods(m, class) {
            s.push_str(&format!("    {};\n", pascal_method_head(mth, None)));
        }
        for p in props {
            let name = pascal(&p.name);
            let ty = pascal_type(&p.kind);
            let write = if p.read_only { String::new() } else { format!(" write Set{name}") };
            s.push_str(&format!("    property {}: {ty} read Get{name}{write};\n", pascal_ident(&name)));
        }
        s.push_str("  end;\n\n");
    }
    s.end();
    let form_props = m.classes.iter().find(|(c, _)| c == "Form").map(|(_, p)| p.clone()).unwrap_or_default();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let base = pascal_form_base(&f.name);
        s.push_str(&format!(
            "  {{ O form {name}: a classe base com os componentes. A do form ({user}), com os\n    handlers (métodos published) e o estado, fica na unit dele. }}\n  {base} = class(TForm)\n  published\n",
            name = f.name,
            user = pascal_form_class(&f.name)
        ));
        for (n, c) in fields(f, &form_props, |p, n| pascal(p).eq_ignore_ascii_case(n)) {
            s.push_str(&format!("    {n}: T{c};\n"));
        }
        s.push_str("  protected\n    procedure BindComponents(List: TJSONArray); override;\n  public\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!(
                "    {{ O {class} deste form com o nome (nil se não há). }}\n    function Find{class}(const Nome: string): T{class};\n    \
                 {{ O {class} que disparou o evento (nil se não é um {class} deste form). }}\n    function SenderAs{class}(Sender: TAylisComponent): T{class}; overload;\n    \
                 function SenderAs{class}(Event: TJSONObject): T{class}; overload;\n"
            ));
        }
        s.push_str("  end;\n\n");
    }
    s.end();
    // As conversões que o despacho dos handlers tipados (aylisforms.pas) também usa.
    s.push_str(
        "function AsText(Data: TJSONData): string;\nfunction AsDate(Data: TJSONData): TDateTime;\nfunction AsTime(Data: TJSONData): TDateTime;\n\
         function AsDateTime(Data: TJSONData): TDateTime;\nfunction AsList(Data: TJSONData): TStringArray;\n\
         function AsNumbers(Data: TJSONData): TAylisDoubleArray;\nfunction AsColors(Data: TJSONData): TAylisColorArray;\n\
         function AsDateRange(Data: TJSONData): TAylisDateRange;\n",
    );
    s.begin(Part::Types);
    s.end();
    s.push_str("\nimplementation\n\nfunction AsText(Data: TJSONData): string;\nbegin\n  if (Data = nil) or (Data.JSONType = jtNull) then\n    Result := ''\n  else\n    Result := Data.AsString;\nend;\n\n");
    s.push_str(PASCAL_HELPERS);
    s.begin(Part::Types);
    s.end();
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let t = format!("T{class}");
        for p in props {
            let name = pascal(&p.name);
            let ty = pascal_type(&p.kind);
            let read = match &p.kind {
                Kind::Component => format!("GetRef('{}')", p.name),
                k => pascal_from(&format!("GetProp('{}')", p.name), k),
            };
            s.push_str(&format!("function {t}.Get{name}: {ty};\nbegin\n  Result := {read};\nend;\n\n"));
            if !p.read_only {
                let write = match &p.kind {
                    Kind::Component => format!("SetRef('{}', Value)", p.name),
                    k => format!("SetProp('{}', {})", p.name, pascal_json("Value", k)),
                };
                s.push_str(&format!("procedure {t}.Set{name}(Value: {ty});\nbegin\n  {write};\nend;\n\n"));
            }
        }
        for mth in pascal_methods(m, class) {
            let args = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| pascal_value(&pascal_arg(a), k)).collect::<Vec<_>>();
            let list = if args.is_empty() { "TJSONArray.Create".to_string() } else { format!("TJSONArray.Create([{}])", args.join(", ")) };
            let names = mth.arg_names();
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", '{k}', {}", if names.contains(v) { pascal_arg(v) } else { v.replace('"', "'") })).collect();
            let request = format!("('{}', TJSONObject.Create(['target', Id{fields}]))", mth.command());
            let body = match &mth.returns {
                Some(k) => format!("Result := {};", pascal_from(&format!("Aylis.Call{request}"), k)),
                None => format!("Aylis.Send{request};"),
            };
            s.push_str(&format!("{};\nbegin\n  {body}\nend;\n\n", pascal_method_head(mth, Some(&t))));
        }
    }
    s.end();
    s.begin(Part::Form);
    for f in &m.forms {
        s.item(&f.name);
        let base = pascal_form_base(&f.name);
        let fs = fields(f, &form_props, |p, n| pascal(p).eq_ignore_ascii_case(n));
        s.push_str(&format!("procedure {base}.BindComponents(List: TJSONArray);\nbegin\n"));
        for (n, c) in &fs {
            if f.optional.contains(n) {
                s.push_str(&format!("  {n} := T{c}(ResolveOptional(List, '{n}', T{c}));\n"));
            } else {
                s.push_str(&format!("  {n} := T{c}(Resolve(List, '{n}', '{c}', T{c}));\n"));
            }
        }
        s.push_str("end;\n\n");
        for class in Model::classes_of(f) {
            s.push_str(&format!("function {base}.Find{class}(const Nome: string): T{class};\nbegin\n  Result := nil;\n"));
            for (n, _) in fs.iter().filter(|(_, c)| c == class) {
                s.push_str(&format!("  if Nome = '{n}' then\n    Result := {n};\n"));
            }
            s.push_str("end;\n\n");
            s.push_str(&format!(
                "function {base}.SenderAs{class}(Sender: TAylisComponent): T{class};\nbegin\n  Result := nil;\n  if Sender <> nil then\n    Result := Find{class}(Sender.Name);\nend;\n\n\
                 function {base}.SenderAs{class}(Event: TJSONObject): T{class};\nbegin\n  Result := Find{class}(TJSONObject(Event.Find('sender')).Get('name', ''));\nend;\n\n"
            ));
        }
    }
    s.end();
    s.push_str("initialization\n  AylisFormat := DefaultFormatSettings;\n  AylisFormat.DecimalSeparator := '.';\nend.\n");
    s
}

/// Os métodos de uma classe em Pascal (no Form, sem o `show` e o `close`, que o `TAylisForm`
/// já tem).
fn pascal_methods<'a>(m: &'a Model, class: &str) -> Vec<&'a Method> {
    m.methods(class).iter().filter(|mth| class != "Form" || !matches!(mth.via.as_str(), "show" | "close")).collect()
}

/// A classe base gerada de um form em Pascal (`frmClientes` → `TFrmClientesBase`).
pub fn pascal_form_base(form: &str) -> String {
    format!("T{}Base", pascal(form))
}

/// A classe do form em Pascal, na unit do usuário (`frmClientes` → `TFrmClientes`).
pub fn pascal_form_class(form: &str) -> String {
    format!("T{}", pascal(form))
}

fn pascal_type(kind: &Kind) -> String {
    match kind {
        Kind::Bool => "Boolean".into(),
        Kind::Int => "Int64".into(),
        Kind::Float => "Double".into(),
        Kind::Text => "string".into(),
        Kind::Color => "TAylisColor".into(),
        Kind::Bounds => "TAylisBounds".into(),
        Kind::Component => "TAylisComponent".into(),
        Kind::Enum(n) => format!("T{n}"),
        Kind::Set(n) => format!("T{n}Set"),
        Kind::Date | Kind::Time | Kind::DateTime => "TDateTime".into(),
        Kind::List => "TStringArray".into(),
        Kind::NumberList => "TAylisDoubleArray".into(),
        Kind::ColorList => "TAylisColorArray".into(),
        Kind::Shadow => "TAylisShadow".into(),
        Kind::Easing => "TAylisEasing".into(),
        Kind::PointList => "TAylisPointArray".into(),
        Kind::Record(n) => format!("T{n}"),
        Kind::RecordList(n) => format!("T{n}Array"),
    }
}

/// O `TJSONData` `v` da tela como o tipo `kind`, em Pascal (dentro de um componente).
fn pascal_from(v: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("{v}.AsBoolean"),
        Kind::Int => format!("{v}.AsInt64"),
        Kind::Float => format!("{v}.AsFloat"),
        Kind::Text => format!("AsText({v})"),
        Kind::Color => format!("AylisColorFromJSON({v})"),
        Kind::Bounds => format!("AylisBoundsFromJSON({v})"),
        Kind::Component => format!("ComponentOf({v})"),
        Kind::Enum(n) => format!("{n}Of({v})"),
        Kind::Set(n) => format!("{n}SetOf({v})"),
        Kind::Date => format!("AsDate({v})"),
        Kind::Time => format!("AsTime({v})"),
        Kind::DateTime => format!("AsDateTime({v})"),
        Kind::List => format!("AsList({v})"),
        Kind::NumberList => format!("AsNumbers({v})"),
        Kind::ColorList => format!("AsColors({v})"),
        Kind::Shadow => format!("AylisShadowFromJSON({v})"),
        Kind::Easing => format!("AylisEasingFromJSON({v})"),
        Kind::PointList => format!("AylisPointsFromJSON({v})"),
        Kind::Record(n) => format!("{n}FromText({v})"),
        Kind::RecordList(n) => format!("{n}ListFromText({v})"),
    }
}

/// O valor `a` do tipo `kind` como `TJSONData` (o `SetProp`), em Pascal.
fn pascal_json(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool => format!("TJSONBoolean.Create({a})"),
        Kind::Int => format!("TJSONInt64Number.Create({a})"),
        Kind::Float => format!("TJSONFloatNumber.Create({a})"),
        Kind::Text => format!("TJSONString.Create({a})"),
        Kind::Color => format!("AylisColorToJSON({a})"),
        Kind::Bounds => format!("AylisBoundsToJSON({a})"),
        Kind::Component => format!("TJSONInt64Number.Create({a}.Id)"),
        Kind::Enum(n) => format!("TJSONString.Create({n}Names[{a}])"),
        Kind::Set(n) => format!("{n}SetToJSON({a})"),
        Kind::Date => format!("DateText({a})"),
        Kind::Time => format!("TimeText({a})"),
        Kind::DateTime => format!("DateTimeText({a})"),
        Kind::List => format!("ListText({a})"),
        Kind::NumberList => format!("NumbersText({a})"),
        Kind::ColorList => format!("ColorsText({a})"),
        Kind::Shadow => format!("AylisShadowToJSON({a})"),
        Kind::Easing => format!("AylisEasingToJSON({a})"),
        Kind::PointList => format!("AylisPointsToJSON({a})"),
        Kind::RecordList(n) => format!("{n}ListText({a})"),
        Kind::Record(n) => format!("{n}Text({a})"),
    }
}

/// O argumento `a` do tipo `kind` num `TJSONArray.Create([...])`, em Pascal.
fn pascal_value(a: &str, kind: &Kind) -> String {
    match kind {
        Kind::Bool | Kind::Int | Kind::Float | Kind::Text => a.to_string(),
        Kind::Component => format!("{a}.Id"),
        Kind::Enum(n) => format!("{n}Names[{a}]"),
        k => pascal_json(a, k),
    }
}

/// Os enums em Pascal: os tipos (com `scopedenums`: `TWindowState.FullScreen`) e, depois do
/// separador, os nomes que a tela entende e as conversões (no implementation).
fn pascal_types(enums: &BTreeMap<String, EnumDef>) -> String {
    let (mut head, mut decls, mut body) = (String::new(), String::new(), String::new());
    for (name, def) in enums {
        let ids: Vec<String> = variants(&def.values, variant_pascal).iter().map(|v| pascal_ident(v)).collect();
        head.push_str(&format!("  {{ Os valores de um enum do acesso tipado. }}\n  {{$push}}{{$scopedenums on}}\n  T{name} = ({});\n  {{$pop}}\n  T{name}Set = set of T{name};\n\n", ids.join(", ")));
        decls.push_str(&format!(
            "function {name}Of(Data: TJSONData): T{name};\nfunction {name}SetOf(Data: TJSONData): T{name}Set;\nfunction {name}SetToJSON(const S: T{name}Set): TJSONData;\n"
        ));
        let names: Vec<String> = def.values.iter().map(|v| format!("'{v}'")).collect();
        body.push_str(&format!(
            "const\n  {name}Names: array[T{name}] of string = ({});\n\n\
             function {name}Of(Data: TJSONData): T{name};\nvar\n  V: T{name};\nbegin\n  Result := Low(T{name});\n  if (Data <> nil) and (Data.JSONType = jtString) then\n    for V := Low(T{name}) to High(T{name}) do\n      if {name}Names[V] = Data.AsString then\n        Exit(V);\nend;\n\n\
             function {name}SetOf(Data: TJSONData): T{name}Set;\nvar\n  I: Integer;\n  V: T{name};\nbegin\n  Result := [];\n  if (Data = nil) or (Data.JSONType <> jtArray) then\n    Exit;\n  for I := 0 to TJSONArray(Data).Count - 1 do\n    for V := Low(T{name}) to High(T{name}) do\n      if {name}Names[V] = TJSONArray(Data).Strings[I] then\n        Include(Result, V);\nend;\n\n\
             function {name}SetToJSON(const S: T{name}Set): TJSONData;\nvar\n  V: T{name};\nbegin\n  Result := TJSONArray.Create;\n  for V := Low(T{name}) to High(T{name}) do\n    if V in S then\n      TJSONArray(Result).Add({name}Names[V]);\nend;\n\n",
            names.join(", ")
        ));
    }
    format!("{head}{TYPES_SEP}{decls}{TYPES_SEP}{body}")
}

/// Entre os pedaços dos enums de uma linguagem que os quer em mais de um lugar (o Pascal: os
/// tipos no interface, as conversões no implementation).
const TYPES_SEP: char = '\u{1f}';

// ---------------------------------------------------------------------------
// As datas, as horas e as listas separadas por `;` (no fio, texto): a conversão de cada
// linguagem, no começo do arquivo gerado.

const PY_HELPERS: &str = r##"

def _date(value):
    try:
        return datetime.date.fromisoformat(value) if value else None
    except ValueError:
        return None


def _time(value):
    try:
        return datetime.time.fromisoformat(value) if value else None
    except ValueError:
        return None


def _datetime(value):
    try:
        return datetime.datetime.fromisoformat(value) if value else None
    except ValueError:
        return None


def _time_text(t):
    if t is None:
        return ""
    return t.strftime("%H:%M" if t.second == 0 else "%H:%M:%S")


def _datetime_text(d):
    if d is None:
        return ""
    return d.strftime("%Y-%m-%dT%H:%M" if d.second == 0 else "%Y-%m-%dT%H:%M:%S")


def _list(value):
    return [s.strip() for s in (value or "").split(";") if s.strip()]


def _numbers(value):
    out = []
    for s in _list(value):
        try:
            out.append(float(s))
        except ValueError:
            pass
    return out


def _colors(value):
    return [Color.hex(s) for s in _list(value) if s.startswith("#")]


@dataclass
class DateRange:
    """O intervalo de datas (o `on_change` do DateRangePicker); `None`: sem a data."""

    start: "datetime.date | None" = None
    end: "datetime.date | None" = None


def _date_range(value):
    parts = (str(value or "") + ";").split(";")
    return DateRange(_date(parts[0].strip()), _date(parts[1].strip()))
"##;

const TS_HELPERS: &str = r##"
/** Uma data do calendário (sem fuso). */
export class LocalDate {
    readonly year: number;
    readonly month: number;
    readonly day: number;
    constructor(year: number, month: number, day: number) {
        this.year = year;
        this.month = month;
        this.day = day;
    }
    /** `2026-10-24` (o que não é data: `null`). */
    static fromJson(value: unknown): LocalDate | null {
        const m = /^(\d{4})-(\d{2})-(\d{2})/.exec(typeof value === "string" ? value : "");
        return m ? new LocalDate(+m[1], +m[2], +m[3]) : null;
    }
    toString(): string {
        return `${String(this.year).padStart(4, "0")}-${String(this.month).padStart(2, "0")}-${String(this.day).padStart(2, "0")}`;
    }
}

/** Uma hora do dia (sem fuso). */
export class LocalTime {
    readonly hour: number;
    readonly minute: number;
    readonly second: number;
    constructor(hour: number, minute: number, second = 0) {
        this.hour = hour;
        this.minute = minute;
        this.second = second;
    }
    /** `14:30` ou `14:30:15` (o que não é hora: `null`). */
    static fromJson(value: unknown): LocalTime | null {
        const m = /^(\d{1,2}):(\d{2})(?::(\d{2}))?/.exec(typeof value === "string" ? value : "");
        return m ? new LocalTime(+m[1], +m[2], m[3] ? +m[3] : 0) : null;
    }
    toString(): string {
        const p = (n: number) => String(n).padStart(2, "0");
        return `${p(this.hour)}:${p(this.minute)}${this.second ? ":" + p(this.second) : ""}`;
    }
}

/** O dia e a hora (sem fuso). */
export class LocalDateTime {
    readonly date: LocalDate;
    readonly time: LocalTime;
    constructor(date: LocalDate, time: LocalTime) {
        this.date = date;
        this.time = time;
    }
    /** `2026-11-12T14:30` (o que não é: `null`). */
    static fromJson(value: unknown): LocalDateTime | null {
        const [d, t] = (typeof value === "string" ? value : "").split(/[T ]/);
        const date = LocalDate.fromJson(d);
        return date ? new LocalDateTime(date, LocalTime.fromJson(t) ?? new LocalTime(0, 0)) : null;
    }
    toString(): string {
        return `${this.date}T${this.time}`;
    }
}

export function listOf(value: unknown): string[] {
    return (typeof value === "string" ? value : "").split(";").map((s) => s.trim()).filter((s) => s !== "");
}

export function numbersOf(value: unknown): number[] {
    return listOf(value).map(Number).filter((n) => !Number.isNaN(n));
}

export function colorsOf(value: unknown): Color[] {
    return listOf(value).filter((s) => s.startsWith("#")).map((s) => Color.hex(s));
}

/** O intervalo de datas (o `on_change` do DateRangePicker); `null`: sem a data. */
export interface DateRange {
    start: LocalDate | null;
    end: LocalDate | null;
}

export function dateRangeOf(value: unknown): DateRange {
    const [s, e] = (String(value ?? "") + ";").split(";");
    return { start: LocalDate.fromJson(s.trim()), end: LocalDate.fromJson(e.trim()) };
}
"##;

const CS_HELPERS: &str = r##"        internal static DateOnly? DateOf(JsonNode? v) => DateOnly.TryParseExact(Text(v), "yyyy-MM-dd", CultureInfo.InvariantCulture, DateTimeStyles.None, out var d) ? d : null;
        internal static TimeOnly? TimeOf(JsonNode? v) => TimeOnly.TryParse(Text(v), CultureInfo.InvariantCulture, DateTimeStyles.None, out var t) ? t : null;
        internal static DateTime? DateTimeOf(JsonNode? v) => DateTime.TryParse(Text(v), CultureInfo.InvariantCulture, DateTimeStyles.None, out var d) ? d : null;
        internal static string DateText(DateOnly? d) => d?.ToString("yyyy-MM-dd", CultureInfo.InvariantCulture) ?? "";
        internal static string TimeText(TimeOnly? t) => t is { } x ? x.ToString(x.Second == 0 ? "HH:mm" : "HH:mm:ss", CultureInfo.InvariantCulture) : "";
        internal static string DateTimeText(DateTime? d) => d is { } x ? x.ToString(x.Second == 0 ? "yyyy-MM-dd'T'HH:mm" : "yyyy-MM-dd'T'HH:mm:ss", CultureInfo.InvariantCulture) : "";
        internal static string[] ListOf(JsonNode? v) => Text(v).Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        internal static double[] NumbersOf(JsonNode? v) => ListOf(v).Select(s => double.TryParse(s, NumberStyles.Float, CultureInfo.InvariantCulture, out var d) ? (double?)d : null).OfType<double>().ToArray();
        internal static Color[] ColorsOf(JsonNode? v) => ListOf(v).Where(s => s.StartsWith('#')).Select(Color.Hex).ToArray();
        internal static string NumbersText(System.Collections.Generic.IEnumerable<double> v) => string.Join(";", v.Select(d => d.ToString(CultureInfo.InvariantCulture)));

        /// <summary>O intervalo de datas (o <c>on_change</c> do DateRangePicker); null: sem a data.</summary>
        public sealed record DateRange(DateOnly? Start, DateOnly? End);

        internal static DateRange DateRangeOf(JsonNode? v)
        {
            var p = (Text(v) + ";").Split(';');
            return new DateRange(DateOf(JsonValue.Create(p[0].Trim())), DateOf(JsonValue.Create(p[1].Trim())));
        }
"##;

const JAVA_HELPERS: &str = r##"
    static java.time.LocalDate asDate(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalDate.parse(s);
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static java.time.LocalTime asTime(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalTime.parse(s);
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static java.time.LocalDateTime asDateTime(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalDateTime.parse(s.replace(' ', 'T'));
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static String dateText(java.time.LocalDate d) {
        return d == null ? "" : d.toString();
    }

    static String timeText(java.time.LocalTime t) {
        return t == null ? "" : t.withNano(0).toString();
    }

    static String dateTimeText(java.time.LocalDateTime d) {
        return d == null ? "" : d.withNano(0).toString();
    }

    static java.util.List<String> asList(Object v) {
        java.util.List<String> out = new java.util.ArrayList<>();
        for (String s : asText(v).split(";")) if (!s.isBlank()) out.add(s.trim());
        return out;
    }

    static java.util.List<Double> asNumbers(Object v) {
        java.util.List<Double> out = new java.util.ArrayList<>();
        for (String s : asList(v)) {
            try {
                out.add(Double.parseDouble(s));
            } catch (NumberFormatException e) {
                // o que não é número fica de fora
            }
        }
        return out;
    }

    static java.util.List<Aylis.Color> asColors(Object v) {
        java.util.List<Aylis.Color> out = new java.util.ArrayList<>();
        for (String s : asList(v)) if (s.startsWith("#")) out.add(Aylis.Color.hex(s));
        return out;
    }

    static String listText(java.util.Collection<?> v) {
        java.util.List<String> out = new java.util.ArrayList<>();
        for (Object x : v) out.add(x instanceof Aylis.Color c ? c.toJson() : String.valueOf(x));
        return String.join(";", out);
    }

    /** O intervalo de datas (o on_change do DateRangePicker); null: sem a data. */
    public record DateRange(java.time.LocalDate start, java.time.LocalDate end) {}

    static DateRange asDateRange(Object v) {
        String[] p = (asText(v) + ";").split(";", -1);
        return new DateRange(asDate(p[0].trim()), asDate(p[1].trim()));
    }
"##;

const GO_HELPERS: &str = r##"
var _ = json.Marshal

// Date, TimeOfDay e DateTime são o time.Time dos handlers tipados (o código dos forms usa sem
// importar o time; zero é nenhum).
type Date = time.Time
type TimeOfDay = time.Time
type DateTime = time.Time
func asDate(v any) time.Time {
	t, _ := time.ParseInLocation("2006-01-02", asText(v), time.Local)
	return t
}

func asTime(v any) time.Time {
	for _, layout := range []string{"15:04:05", "15:04"} {
		if t, err := time.ParseInLocation(layout, asText(v), time.Local); err == nil {
			return t
		}
	}
	return time.Time{}
}

func asDateTime(v any) time.Time {
	s := strings.Replace(asText(v), " ", "T", 1)
	for _, layout := range []string{"2006-01-02T15:04:05", "2006-01-02T15:04"} {
		if t, err := time.ParseInLocation(layout, s, time.Local); err == nil {
			return t
		}
	}
	return time.Time{}
}

func dateText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	return t.Format("2006-01-02")
}

func timeText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	if t.Second() == 0 {
		return t.Format("15:04")
	}
	return t.Format("15:04:05")
}

func dateTimeText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	if t.Second() == 0 {
		return t.Format("2006-01-02T15:04")
	}
	return t.Format("2006-01-02T15:04:05")
}

func asList(v any) []string {
	var out []string
	for _, s := range strings.Split(asText(v), ";") {
		if s = strings.TrimSpace(s); s != "" {
			out = append(out, s)
		}
	}
	return out
}

func asNumbers(v any) []float64 {
	var out []float64
	for _, s := range asList(v) {
		if f, err := strconv.ParseFloat(s, 64); err == nil {
			out = append(out, f)
		}
	}
	return out
}

func asColors(v any) []aylis.Color {
	var out []aylis.Color
	for _, s := range asList(v) {
		if c, err := aylis.ColorHex(s); err == nil {
			out = append(out, c)
		}
	}
	return out
}

func numbersText(v []float64) string {
	out := make([]string, len(v))
	for i, f := range v {
		out[i] = strconv.FormatFloat(f, 'g', -1, 64)
	}
	return strings.Join(out, ";")
}

func colorsText(v []aylis.Color) string {
	out := make([]string, len(v))
	for i, c := range v {
		out[i] = c.JSON()
	}
	return strings.Join(out, ";")
}

// DateRange é o intervalo de datas (o on_change do DateRangePicker); a data zero: sem a data.
type DateRange struct {
	Start Date
	End   Date
}

func asDateRange(v any) DateRange {
	p := strings.SplitN(asText(v)+";", ";", 3)
	return DateRange{Start: asDate(strings.TrimSpace(p[0])), End: asDate(strings.TrimSpace(p[1]))}
}
"##;

const CPP_HELPERS: &str = r##"
// Uma data do calendário (sem fuso).
struct LocalDate {
    int year = 0, month = 0, day = 0;
    static std::optional<LocalDate> parse(const std::string& s) {
        LocalDate d;
        if (std::sscanf(s.c_str(), "%d-%d-%d", &d.year, &d.month, &d.day) != 3) return std::nullopt;
        return d;
    }
    std::string to_string() const {
        char out[16];
        std::snprintf(out, sizeof out, "%04d-%02d-%02d", year, month, day);
        return out;
    }
};

// Uma hora do dia (sem fuso).
struct LocalTime {
    int hour = 0, minute = 0, second = 0;
    static std::optional<LocalTime> parse(const std::string& s) {
        LocalTime t;
        int n = std::sscanf(s.c_str(), "%d:%d:%d", &t.hour, &t.minute, &t.second);
        if (n < 2) return std::nullopt;
        return t;
    }
    std::string to_string() const {
        char out[16];
        if (second == 0)
            std::snprintf(out, sizeof out, "%02d:%02d", hour, minute);
        else
            std::snprintf(out, sizeof out, "%02d:%02d:%02d", hour, minute, second);
        return out;
    }
};

// O dia e a hora (sem fuso).
struct LocalDateTime {
    LocalDate date;
    LocalTime time;
    static std::optional<LocalDateTime> parse(const std::string& s) {
        auto at = s.find_first_of("T ");
        auto d = LocalDate::parse(s.substr(0, at));
        if (!d) return std::nullopt;
        auto t = at == std::string::npos ? std::optional<LocalTime>(LocalTime{}) : LocalTime::parse(s.substr(at + 1));
        return LocalDateTime{*d, t.value_or(LocalTime{})};
    }
    std::string to_string() const { return date.to_string() + "T" + time.to_string(); }
};

template <class T>
std::string text_of_(const std::optional<T>& v) { return v ? v->to_string() : std::string(); }

inline std::string trim_(const std::string& s) {
    auto a = s.find_first_not_of(' '), b = s.find_last_not_of(' ');
    return a == std::string::npos ? std::string() : s.substr(a, b - a + 1);
}

inline std::vector<std::string> list_of_(const aylis::Json& v) {
    std::vector<std::string> out;
    std::string s = v.str(), item;
    std::istringstream in(s);
    while (std::getline(in, item, ';')) {
        auto a = item.find_first_not_of(' '), b = item.find_last_not_of(' ');
        if (a != std::string::npos) out.push_back(item.substr(a, b - a + 1));
    }
    return out;
}

inline std::vector<double> numbers_of_(const aylis::Json& v) {
    std::vector<double> out;
    for (const auto& s : list_of_(v)) {
        char* end = nullptr;
        double d = std::strtod(s.c_str(), &end);
        if (end && *end == '\0') out.push_back(d);
    }
    return out;
}

inline std::vector<aylis::Color> colors_of_(const aylis::Json& v) {
    std::vector<aylis::Color> out;
    for (const auto& s : list_of_(v))
        if (!s.empty() && s[0] == '#') out.push_back(aylis::Color::hex(s));
    return out;
}

inline std::string join_(const std::vector<std::string>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) out += (i ? ";" : "") + v[i];
    return out;
}

inline std::string numbers_text_(const std::vector<double>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) {
        char n[32];
        std::snprintf(n, sizeof n, "%.15g", v[i]);
        out += (i ? ";" : "") + std::string(n);
    }
    return out;
}

inline std::string colors_text_(const std::vector<aylis::Color>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) out += (i ? ";" : "") + v[i].to_json();
    return out;
}

// O intervalo de datas (o `on_change` do DateRangePicker); vazio: sem a data.
struct DateRange {
    std::optional<LocalDate> start;
    std::optional<LocalDate> end;

    static DateRange from_text(const std::string& s) {
        DateRange r;
        auto i = s.find(';');
        r.start = LocalDate::parse(s.substr(0, i));
        if (i != std::string::npos) r.end = LocalDate::parse(s.substr(i + 1));
        return r;
    }
};
"##;

/// Em Pascal: a data, a hora e o dia e a hora são `TDateTime` (0 é nenhuma data; a hora sem
/// valor é -1, porque 0 é a meia-noite).
const PASCAL_HELPERS: &str = r##"var
  AylisFormat: TFormatSettings;

function AsDate(Data: TJSONData): TDateTime;
var
  S: string;
  Y, M, D: Integer;
begin
  Result := 0;
  S := AsText(Data);
  if (Length(S) >= 10) and TryStrToInt(Copy(S, 1, 4), Y) and TryStrToInt(Copy(S, 6, 2), M) and TryStrToInt(Copy(S, 9, 2), D) then
    TryEncodeDate(Y, M, D, Result);
end;

function AsTime(Data: TJSONData): TDateTime;
var
  S: string;
  H, M, Sec: Integer;
begin
  Result := -1;
  S := AsText(Data);
  Sec := 0;
  if (Length(S) >= 5) and TryStrToInt(Copy(S, 1, 2), H) and TryStrToInt(Copy(S, 4, 2), M) and ((Length(S) < 8) or TryStrToInt(Copy(S, 7, 2), Sec)) then
    if not TryEncodeTime(H, M, Sec, 0, Result) then
      Result := -1;
end;

function AsDateTime(Data: TJSONData): TDateTime;
var
  S: string;
  T: TJSONString;
  Hora: TDateTime;
begin
  S := AsText(Data);
  T := TJSONString.Create(Copy(S, 1, 10));
  try
    Result := AsDate(T);
  finally
    T.Free;
  end;
  if (Result <> 0) and (Length(S) > 11) then
  begin
    T := TJSONString.Create(Copy(S, 12, 8));
    try
      Hora := AsTime(T);
    finally
      T.Free;
    end;
    if Hora >= 0 then
      Result := Result + Hora;
  end;
end;

function DateText(D: TDateTime): TJSONData;
begin
  if D = 0 then
    Result := TJSONString.Create('')
  else
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd', D));
end;

function TimeText(T: TDateTime): TJSONData;
begin
  if T < 0 then
    Result := TJSONString.Create('')
  else if SecondOf(T) = 0 then
    Result := TJSONString.Create(FormatDateTime('hh":"nn', T))
  else
    Result := TJSONString.Create(FormatDateTime('hh":"nn":"ss', T));
end;

function DateTimeText(D: TDateTime): TJSONData;
begin
  if D = 0 then
    Result := TJSONString.Create('')
  else if SecondOf(D) = 0 then
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd"T"hh":"nn', D))
  else
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd"T"hh":"nn":"ss', D));
end;

function AsList(Data: TJSONData): TStringArray;
var
  Partes: TStringArray;
  P: string;
begin
  Result := nil;
  Partes := AsText(Data).Split(';');
  for P in Partes do
    if Trim(P) <> '' then
      Insert(Trim(P), Result, Length(Result));
end;

function AsNumbers(Data: TJSONData): TAylisDoubleArray;
var
  P: string;
  V: Double;
begin
  Result := nil;
  for P in AsList(Data) do
    if TryStrToFloat(P, V, AylisFormat) then
      Insert(V, Result, Length(Result));
end;

function AsColors(Data: TJSONData): TAylisColorArray;
var
  P: string;
begin
  Result := nil;
  for P in AsList(Data) do
    if (P <> '') and (P[1] = '#') then
      Insert(AylisColorHex(P), Result, Length(Result));
end;

function ListText(const A: TStringArray): TJSONData;
begin
  Result := TJSONString.Create(string.Join(';', A));
end;

function NumbersText(const A: TAylisDoubleArray): TJSONData;
var
  I: Integer;
  S: string;
begin
  S := '';
  for I := 0 to High(A) do
  begin
    if I > 0 then
      S := S + ';';
    S := S + FloatToStr(A[I], AylisFormat);
  end;
  Result := TJSONString.Create(S);
end;

function ColorsText(const A: TAylisColorArray): TJSONData;
var
  I: Integer;
  S: string;
  C: TJSONData;
begin
  S := '';
  for I := 0 to High(A) do
  begin
    if I > 0 then
      S := S + ';';
    C := AylisColorToJSON(A[I]);
    try
      S := S + C.AsString;
    finally
      C.Free;
    end;
  end;
  Result := TJSONString.Create(S);
end;


function AsDateRange(Data: TJSONData): TAylisDateRange;
var
  S: string;
  I: Integer;
  T: TJSONString;
begin
  S := AsText(Data);
  I := Pos(';', S);
  if I = 0 then
    I := Length(S) + 1;
  T := TJSONString.Create(Trim(Copy(S, 1, I - 1)));
  try
    Result.StartDate := AsDate(T);
  finally
    T.Free;
  end;
  T := TJSONString.Create(Trim(Copy(S, I + 1, MaxInt)));
  try
    Result.EndDate := AsDate(T);
  finally
    T.Free;
  end;
end;
"##;

// ---------------------------------------------------------------------------
// Os handlers tipados nas outras linguagens: a assinatura do esqueleto, como saber se o
// handler do usuário já é tipado (o de antes recebe o evento inteiro e continua sendo chamado
// assim) e a chamada que o registro gerado faz, com o valor já convertido.

/// A assinatura tipada de um handler na linguagem: no Rust, todos os eventos declarados; nas
/// outras, ainda sem o ponteiro, a roda, o gesto, o deslize, a tecla, o intervalo de datas e a
/// referência (esses ficam com a assinatura de sempre, que recebe o evento inteiro).
pub(crate) fn signature_for(language: Language, events: &[(String, String)]) -> Option<HandlerSig> {
    let mut sig = handler_signature(events)?;
    // O ponteiro, a tecla, a roda, o gesto e o deslize chegam como objeto: nas outras
    // linguagens, o registro com os mesmos campos.
    if language != Language::Rust {
        if let Some((name, _)) = event_record(&sig.arg) {
            sig.arg = EventKind::Value(Kind::Record(name.to_string()));
        }
        // O intervalo de datas chega como `início;fim`: o `DateRange` dos auxiliares de cada uma.
        if sig.arg == EventKind::DateRange {
            sig.arg = EventKind::Value(Kind::Record(DATE_RANGE.to_string()));
        }
    }
    // O componente no valor (o `on_new_window` do TabStrip): no Pascal, o objeto não teria dono
    // (vazaria a cada evento), então ele fica com a assinatura de sempre.
    let other = language == Language::Pascal && matches!(sig.arg, EventKind::Value(Kind::Component));
    (language == Language::Rust || !other).then_some(sig)
}

/// O intervalo de datas de um evento (`DateRangePicker.on_change`): no fio, `início;fim`; nas
/// linguagens, o `DateRange` dos auxiliares de cada uma.
const DATE_RANGE: &str = "DateRange";

/// O registro de um evento que o protocolo manda como objeto (`args_to_json` do
/// `aylis-remote`): o nome e os campos. O Rust usa o tipo do framework (`PointerEvent`...).
fn event_record(arg: &EventKind) -> Option<(&'static str, Vec<(String, Kind)>)> {
    let fields = |list: &[(&str, Kind)]| list.iter().map(|(n, k)| (n.to_string(), k.clone())).collect();
    use Kind::{Bool, Float, Int, Text};
    Some(match arg {
        EventKind::Pointer => ("PointerEvent", fields(&[("x", Float), ("y", Float), ("button", Text), ("clicks", Int)])),
        EventKind::Wheel => ("WheelEvent", fields(&[("x", Float), ("y", Float), ("dx", Float), ("dy", Float)])),
        EventKind::Gesture => (
            "GestureEvent",
            fields(&[("phase", Text), ("x", Float), ("y", Float), ("scale", Float), ("delta_scale", Float), ("rotation", Float), ("delta_rotation", Float), ("pan_x", Float), ("pan_y", Float)]),
        ),
        EventKind::Swipe => ("SwipeEvent", fields(&[("direction", Text), ("x", Float), ("y", Float), ("dx", Float), ("dy", Float), ("vx", Float), ("vy", Float)])),
        EventKind::Key => ("KeyEvent", fields(&[("key", Text), ("ctrl", Bool), ("shift", Bool), ("alt", Bool)])),
        _ => return None,
    })
}

/// Os registros dos eventos das classes dadas (o `PointerEvent` de quem tem `on_mouse_down`...),
/// para as linguagens que não são Rust.
fn event_records(catalog: &Catalog, classes: &[String]) -> BTreeMap<String, Vec<(String, Kind)>> {
    let mut out = BTreeMap::new();
    for class in classes {
        let Some(map) = catalog.map.get(class).and_then(Value::as_object) else { continue };
        for event in map.iter().filter(|(_, v)| v.as_str().is_some_and(|t| t.starts_with("event:"))).map(|(k, _)| k) {
            match catalog.event_kind(class, event) {
                Some(EventKind::Value(Kind::Record(name))) => {
                    // O registro do evento, com os de dentro dele.
                    let mut pending = vec![name];
                    while let Some(name) = pending.pop() {
                        if out.contains_key(&name) {
                            continue;
                        }
                        let Some(fields) = catalog.records.get(&name) else { continue };
                        pending.extend(fields.iter().filter_map(|(_, k)| if let Kind::Record(n) | Kind::RecordList(n) = k { Some(n.clone()) } else { None }));
                        out.insert(name, fields.clone());
                    }
                }
                Some(arg) => {
                    if let Some((name, fields)) = event_record(&arg) {
                        out.insert(name.to_string(), fields);
                    }
                }
                None => {}
            }
        }
    }
    out
}

/// O valor do evento, quando ele leva algo.
fn event_value(sig: &HandlerSig) -> Option<&Kind> {
    match &sig.arg {
        EventKind::Value(k) => Some(k),
        _ => None,
    }
}

/// O tipo de quem disparou em cada linguagem (a classe gerada; sem classe única, o componente).
fn sender_ty(language: Language, sig: &HandlerSig) -> String {
    match (language, sig.sender.as_deref()) {
        (Language::Python, Some(c)) => format!("\"{c}\""),
        (Language::Python, None) => "\"AylisComponent\"".into(),
        (Language::TypeScript, Some(c)) => format!("import(\"./components.ts\").{c}"),
        (Language::TypeScript, None) => "import(\"../aylis.ts\").AylisComponent".into(),
        (Language::CSharp, Some(c)) => format!("AylisUi.{c}"),
        (Language::CSharp, None) => "AylisComponent".into(),
        (Language::Java, Some(c)) => format!("Components.{c}"),
        (Language::Java, None) => "Aylis.AylisComponent".into(),
        (Language::Go, Some(c)) => format!("aylis{c}"),
        (Language::Go, None) => "aylis.Component".into(),
        (Language::Cpp, Some(c)) => format!("aylis_components::{c}"),
        (Language::Cpp, None) => "aylis::Component".into(),
        (Language::Pascal, Some(c)) => format!("T{c}"),
        (Language::Pascal, None) => "TAylisComponent".into(),
        (Language::Rust, _) => String::new(),
    }
}

/// O tipo do valor em cada linguagem, como o handler do usuário escreve.
fn value_ty(language: Language, kind: &Kind) -> String {
    // O componente de um evento (o id dele; `None` quando não veio).
    if *kind == Kind::Component {
        return match language {
            Language::Python => "\"AylisComponent | None\"".into(),
            Language::TypeScript => "import(\"../aylis.ts\").AylisComponent | null".into(),
            Language::CSharp => "AylisComponent?".into(),
            Language::Java => "Aylis.AylisComponent".into(),
            Language::Go => "aylis.Component".into(),
            Language::Cpp => "std::optional<aylis::Component>".into(),
            Language::Pascal | Language::Rust => String::new(),
        };
    }
    if matches!(kind, Kind::Record(n) if n == DATE_RANGE) {
        return match language {
            Language::Python => "\"DateRange\"".into(),
            Language::TypeScript => "import(\"./components.ts\").DateRange".into(),
            Language::CSharp | Language::Java => "Components.DateRange".into(),
            Language::Go => "DateRange".into(),
            Language::Cpp => "aylis_components::DateRange".into(),
            Language::Pascal => "TAylisDateRange".into(),
            Language::Rust => String::new(),
        };
    }
    // O registro de um evento (o ponteiro, a tecla...) sempre vem: sem o opcional.
    if let Kind::Record(n) = kind {
        return match language {
            Language::Python => format!("\"{n}\""),
            Language::TypeScript => format!("import(\"./components.ts\").{n}"),
            Language::CSharp => format!("global::App.Forms.{n}"),
            Language::Java => format!("Components.{n}"),
            Language::Go => n.clone(),
            Language::Cpp => format!("aylis_components::{n}"),
            Language::Pascal => format!("T{n}"),
            Language::Rust => String::new(),
        };
    }
    match language {
        Language::Python => format!("\"{}\"", python_ty(kind)),
        Language::TypeScript => match kind {
            Kind::Enum(n) => format!("import(\"./components.ts\").{n}"),
            Kind::Date => "import(\"./components.ts\").LocalDate | null".into(),
            Kind::Time => "import(\"./components.ts\").LocalTime | null".into(),
            Kind::DateTime => "import(\"./components.ts\").LocalDateTime | null".into(),
            Kind::Color => "import(\"../aylis.ts\").Color".into(),
            Kind::Shadow => "import(\"../aylis.ts\").Shadow | null".into(),
            Kind::Easing => "import(\"../aylis.ts\").Easing".into(),
            k => ts_ty(k),
        },
        Language::CSharp => cs_ty(kind),
        Language::Java => match kind {
            Kind::Enum(n) => format!("Components.{n}"),
            k => java_ty(k),
        },
        Language::Go => match kind {
            Kind::Date => "Date".into(),
            Kind::Time => "TimeOfDay".into(),
            Kind::DateTime => "DateTime".into(),
            k => go_ty(k, false),
        },
        Language::Cpp => match kind {
            Kind::Enum(n) => format!("aylis_components::{n}"),
            Kind::Date => "std::optional<aylis_components::LocalDate>".into(),
            Kind::Time => "std::optional<aylis_components::LocalTime>".into(),
            Kind::DateTime => "std::optional<aylis_components::LocalDateTime>".into(),
            k => cpp_ty(k),
        },
        Language::Pascal => pascal_type(kind),
        Language::Rust => String::new(),
    }
}

/// Os parâmetros do handler tipado, como o esqueleto escreve (sem o nome da função).
pub(crate) fn typed_params(language: Language, sig: &HandlerSig) -> String {
    let sender = sender_ty(language, sig);
    let value = event_value(sig).map(|k| value_ty(language, k));
    match (language, value) {
        (Language::Python, Some(v)) => format!("aylis, sender: {sender}, value: {v}"),
        (Language::Python, None) => format!("aylis, sender: {sender}"),
        (Language::TypeScript, Some(v)) => format!("aylis: Aylis, sender: {sender}, value: {v}"),
        (Language::TypeScript, None) => format!("aylis: Aylis, sender: {sender}"),
        (Language::CSharp, Some(v)) => format!("Aylis aylis, {sender} sender, {v} value"),
        (Language::CSharp, None) => format!("Aylis aylis, {sender} sender"),
        (Language::Java, Some(v)) => format!("Aylis aylis, {sender} sender, {v} value"),
        (Language::Java, None) => format!("Aylis aylis, {sender} sender"),
        (Language::Go, Some(v)) => format!("app *aylis.Aylis, sender {sender}, value {v}"),
        (Language::Go, None) => format!("app *aylis.Aylis, sender {sender}"),
        (Language::Cpp, Some(v)) => format!("aylis::Aylis& app, {sender} sender, {v} value"),
        (Language::Cpp, None) => format!("aylis::Aylis& app, {sender} sender"),
        (Language::Pascal, Some(v)) => format!("Sender: {sender}; Value: {v}"),
        (Language::Pascal, None) => format!("Sender: {sender}"),
        (Language::Rust, _) => rust_handler_params(sig),
    }
}

/// O handler `handler` do usuário, em `text`, já tem a assinatura tipada? (O de antes recebe o
/// evento inteiro: `event`, `AylisEvent`, `aylis.Event`, `TJSONObject`...)
pub(crate) fn is_typed_handler(language: Language, text: &str, handler: &str) -> bool {
    let head = match language {
        Language::Python => format!("def {handler}("),
        Language::TypeScript => format!("function {handler}("),
        Language::CSharp | Language::Java | Language::Cpp => format!("void {handler}("),
        Language::Go => format!(") {handler}("),
        Language::Pascal => format!(".{handler}("),
        Language::Rust => return true,
    };
    let Some(start) = text.find(&head).map(|i| i + head.len()) else { return false };
    let Some(len) = text[start..].find(')') else { return false };
    let params = &text[start..start + len];
    match language {
        Language::Python => params.split(',').nth(1).is_some_and(|p| p.contains(':')),
        Language::TypeScript | Language::CSharp => !params.contains("AylisEvent"),
        // O tipo de algum parâmetro é o evento inteiro (`PointerEvent value` não conta).
        Language::Java => !params.split(',').any(|p| matches!(p.split_whitespace().next(), Some("Aylis.Event" | "Event"))),
        Language::Go => !params.contains("aylis.Event"),
        Language::Cpp => !params.contains("aylis::Event"),
        Language::Pascal => !params.contains("TJSONObject") && !params.contains("var "),
        Language::Rust => true,
    }
}

/// O valor do evento (`args`, a expressão `e` com o JSON dele) no tipo, onde o registro gerado
/// chama o handler.
fn event_arg_value(language: Language, e: &str, kind: &Kind) -> String {
    // O componente chega pelo id (o nome não vem: o componente pode ser de outro form).
    if *kind == Kind::Component {
        return match language {
            Language::Python => format!("(AylisComponent(aylis, int({e}), \"\") if {e} is not None else None)"),
            Language::TypeScript => format!("({e} == null ? null : new AylisComponent(aylis, Number({e}), \"\"))"),
            Language::CSharp => format!("({e} is System.Text.Json.Nodes.JsonValue componentId ? new AylisComponent(aylis, componentId.GetValue<long>(), \"\") : null)"),
            Language::Java => format!("({e} == null ? null : new Aylis.AylisComponent(aylis, ((Number) {e}).longValue(), \"\"))"),
            Language::Go => format!("func() aylis.Component {{ n, _ := {e}.(float64); return aylis.Component{{App: app, ID: int64(n)}} }}()"),
            Language::Cpp => format!("({e}.kind() == aylis::Json::Kind::Number ? std::optional<aylis::Component>(aylis::Component(&app, static_cast<long long>({e}.number()), \"\")) : std::nullopt)"),
            Language::Pascal | Language::Rust => String::new(),
        };
    }
    if matches!(kind, Kind::Record(n) if n == DATE_RANGE) {
        return match language {
            Language::Python => format!("_date_range({e})"),
            Language::TypeScript => format!("components.dateRangeOf({e})"),
            Language::CSharp => format!("Components.DateRangeOf({e})"),
            Language::Java => format!("Components.asDateRange({e})"),
            Language::Go => format!("asDateRange({e})"),
            Language::Cpp => format!("aylis_components::DateRange::from_text({e}.str())"),
            Language::Pascal => format!("AsDateRange({e})"),
            Language::Rust => String::new(),
        };
    }
    // O registro de um evento chega como objeto (não como o texto do JSON).
    if let Kind::Record(n) = kind {
        return match language {
            Language::Python => format!("{n}.from_json({e} or {{}})"),
            Language::TypeScript => format!("components.{}({e})", record_fn(n)),
            Language::CSharp => format!("global::App.Forms.{n}.FromJson({e})"),
            Language::Java => format!("Components.{n}.fromJson({e})"),
            Language::Go => format!("recordOf{n}({e})"),
            Language::Cpp => format!("aylis_components::{n}::from_json({e})"),
            Language::Pascal => format!("{n}FromJSON({e})"),
            Language::Rust => String::new(),
        };
    }
    match language {
        Language::Python => python_from(e, kind),
        Language::TypeScript => match kind {
            Kind::Bool => format!("{e} as boolean"),
            Kind::Int | Kind::Float => format!("Number({e} ?? 0)"),
            Kind::Text => format!("String({e} ?? \"\")"),
            Kind::Color => format!("Color.fromJson({e})"),
            Kind::Date => format!("components.LocalDate.fromJson({e})"),
            Kind::Time => format!("components.LocalTime.fromJson({e})"),
            Kind::DateTime => format!("components.LocalDateTime.fromJson({e})"),
            Kind::List => format!("components.listOf({e})"),
            Kind::NumberList => format!("components.numbersOf({e})"),
            Kind::ColorList => format!("components.colorsOf({e})"),
            Kind::Enum(n) => format!("{e} as components.{n}"),
            Kind::Shadow => format!("Shadow.fromJson({e})"),
            Kind::Easing => format!("Easing.fromJson({e})"),
            _ => e.to_string(),
        },
        Language::CSharp => match kind {
            Kind::Bool => format!("Components.Bool({e})"),
            Kind::Int => format!("(long)Components.Number({e})"),
            Kind::Float => format!("Components.Number({e})"),
            Kind::Text => format!("Components.Text({e})"),
            Kind::Date => format!("Components.DateOf({e})"),
            Kind::Time => format!("Components.TimeOf({e})"),
            Kind::DateTime => format!("Components.DateTimeOf({e})"),
            Kind::List => format!("Components.ListOf({e})"),
            Kind::NumberList => format!("Components.NumbersOf({e})"),
            Kind::ColorList => format!("Components.ColorsOf({e})"),
            k => cs_from(e, k),
        },
        Language::Java => match kind {
            Kind::Enum(n) => format!("Components.{n}.of({e})"),
            Kind::Color => format!("Aylis.Color.fromJson({e})"),
            Kind::Shadow => format!("Aylis.Shadow.fromJson({e})"),
            Kind::Easing => format!("Aylis.Easing.fromJson({e})"),
            Kind::Int => format!("(long) Components.asNumber({e})"),
            k => format!("Components.{}", java_from(e, k)),
        },
        Language::Go => go_from(e, kind),
        Language::Cpp => cpp_from(e, kind),
        Language::Pascal => pascal_from(e, kind),
        Language::Rust => String::new(),
    }
}

/// A chamada do handler tipado no registro gerado: `func` é o handler já qualificado
/// (`paint.ok_click`), `form` a variável da janela do evento (`frmPaint`) e `ev` o evento.
pub(crate) fn typed_call(language: Language, func: &str, form: &str, ev: &str, sig: &HandlerSig) -> String {
    let sender = match (language, sig.sender.as_deref()) {
        // O evento do próprio form (`on_show`): quem disparou é a janela do evento.
        (Language::Python | Language::Cpp, Some("Form")) => form.to_string(),
        (Language::TypeScript, Some("Form")) => format!("components.{form}"),
        (Language::CSharp | Language::Java, Some("Form")) => format!("Components.{form}"),
        (Language::Go, Some("Form")) => format!("{form}.aylisForm"),
        (Language::Pascal, Some("Form")) => "TForm(Sender)".into(),
        (Language::Python, Some(c)) => format!("{form}.sender_as_{}({ev})", snake(c)),
        (Language::Python, None) => format!("AylisComponent(aylis, {ev}[\"sender\"][\"id\"], {ev}[\"sender\"][\"name\"])"),
        (Language::TypeScript, Some(c)) => format!("components.{form}.senderAs{c}({ev})!"),
        (Language::TypeScript, None) => format!("new AylisComponent(aylis, {ev}.sender.id, {ev}.sender.name)"),
        (Language::CSharp, Some(c)) => format!("Components.{form}.SenderAs{c}({ev})!"),
        (Language::CSharp, None) => format!("new AylisComponent(aylis, {ev}.SenderId, {ev}.SenderName)"),
        (Language::Java, Some(c)) => format!("Components.{form}.senderAs{c}({ev})"),
        (Language::Java, None) => format!("new Aylis.AylisComponent(aylis, ((Number) {ev}.sender().get(\"id\")).longValue(), String.valueOf({ev}.sender().get(\"name\")))"),
        (Language::Go, Some(c)) => format!("{form}.SenderAs{c}({ev})"),
        (Language::Go, None) => format!("aylis.Component{{App: app, ID: int64({ev}.Sender.ID), Name: {ev}.Sender.Name}}"),
        (Language::Cpp, Some(c)) => format!("{form}.sender_as_{}({ev})", snake(c)),
        (Language::Cpp, None) => format!("aylis::Component(&app, {ev}.sender_id, {ev}.sender_name)"),
        (Language::Pascal, Some(c)) => format!("T{c}(Sender)"),
        (Language::Pascal, None) => "Sender".into(),
        (Language::Rust, _) => String::new(),
    };
    let args = match language {
        Language::Python => format!("{ev}.get(\"args\")"),
        Language::TypeScript | Language::Cpp => format!("{ev}.args"),
        Language::CSharp | Language::Go => format!("{ev}.Args"),
        Language::Java => format!("{ev}.args()"),
        Language::Pascal => format!("{ev}.Find('args')"),
        Language::Rust => String::new(),
    };
    let app = match language {
        Language::Python | Language::TypeScript | Language::CSharp | Language::Java => "aylis",
        Language::Go | Language::Cpp => "app",
        Language::Pascal | Language::Rust => "",
    };
    let mut values: Vec<String> = Vec::new();
    if !app.is_empty() {
        values.push(app.to_string());
    }
    values.push(sender);
    if let Some(k) = event_value(sig) {
        values.push(event_arg_value(language, &args, k));
    }
    format!("{func}({})", values.join(", "))
}

// ---------------------------------------------------------------------------
// Os registros (o resultado dos métodos com campos, em JSON no fio): um tipo com os campos em
// cada linguagem e a leitura do JSON.

type Records = BTreeMap<String, Vec<(String, Kind)>>;

/// Os registros na ordem em que cada um vem depois dos que ele tem dentro (o `VideoMode` antes
/// do `Monitor`): o C++ e o Pascal precisam do tipo declarado antes.
fn record_order(records: &Records) -> Vec<String> {
    // `seen`: os já visitados (um registro que se contém, os filhos de um nó, não volta a ele).
    fn visit(name: &str, records: &Records, seen: &mut BTreeSet<String>, out: &mut Vec<String>) {
        if !seen.insert(name.to_string()) {
            return;
        }
        if let Some(fields) = records.get(name) {
            for (_, k) in fields {
                if let Kind::Record(n) | Kind::RecordList(n) = k {
                    visit(n, records, seen, out);
                }
            }
            out.push(name.to_string());
        }
    }
    let (mut out, mut seen) = (Vec::new(), BTreeSet::new());
    for name in records.keys() {
        visit(name, records, &mut seen, &mut out);
    }
    out
}

/// O nome da função que lê o registro em TypeScript (`Monitor` → `monitorFromJson`).
fn record_fn(name: &str) -> String {
    format!("{}{}FromJson", name[..1].to_lowercase(), &name[1..])
}

/// O nome da função que grava um registro em JSON (`listViewItemToJson`), no TypeScript.
fn record_to_fn(name: &str) -> String {
    format!("{}{}ToJson", name[..1].to_lowercase(), &name[1..])
}

/// O campo de um registro como o gerador lê e grava: os tipos que um campo pode ter (o resto
/// vai como texto).
fn field_kind(k: &Kind) -> Kind {
    match k {
        Kind::Bool | Kind::Int | Kind::Float | Kind::Text | Kind::Color | Kind::Date | Kind::DateTime | Kind::List | Kind::NumberList | Kind::Record(_) | Kind::RecordList(_) | Kind::Enum(_) => k.clone(),
        _ => Kind::Text,
    }
}

fn rust_records(records: &Records, order: &[String], enums: &BTreeMap<String, EnumDef>) -> String {
    let mut s = String::new();
    for name in order {
        let fields = &records[name];
        s.push_str(&format!("\n/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).\n#[derive(Clone, Debug, PartialEq)]\npub struct {name} {{\n"));
        for (f, k) in fields {
            let ty = match field_kind(k) {
                Kind::Bool => "bool".to_string(),
                Kind::Int => "i64".into(),
                Kind::Float => "f64".into(),
                Kind::Color => "aylis::prelude::Color".into(),
                Kind::Date => "Option<aylis::prelude::Date>".into(),
                Kind::DateTime => "Option<aylis::prelude::DateTime>".into(),
                Kind::List => "Vec<String>".into(),
                Kind::NumberList => "Vec<f64>".into(),
                Kind::Record(n) => format!("Option<Box<{n}>>"),
                Kind::RecordList(n) => format!("Vec<{n}>"),
                k @ Kind::Enum(_) => rust_ty(&k, enums),
                _ => "String".into(),
            };
            s.push_str(&format!("    pub {}: {ty},\n", rust_ident(f)));
        }
        s.push_str(&format!("}}\n\nimpl {name} {{\n    /// O registro lido do JSON.\n    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {{\n        Self {{\n"));
        for (f, k) in fields {
            let get = format!("v.get(\"{f}\")");
            let value = match field_kind(k) {
                Kind::Bool => format!("{get}.and_then(|x| x.as_bool()).unwrap_or_default()"),
                Kind::Int => format!("{get}.and_then(|x| x.as_i64()).unwrap_or_default()"),
                Kind::Float => format!("{get}.and_then(|x| x.as_f64()).unwrap_or_default()"),
                Kind::Color => format!("{get}.and_then(|x| x.as_str()).and_then(aylis::prelude::Color::from_hex).unwrap_or_default()"),
                Kind::Date => format!("{get}.and_then(|x| x.as_str()).and_then(aylis::prelude::Date::parse_iso)"),
                Kind::DateTime => format!("{get}.and_then(|x| x.as_str()).and_then(aylis::prelude::DateTime::parse_iso)"),
                Kind::List => format!("{get}.and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()"),
                Kind::NumberList => format!("{get}.and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default()"),
                Kind::Record(n) => format!("{get}.filter(|x| x.is_object()).map(|x| Box::new({n}::from_json(x)))"),
                Kind::RecordList(n) => format!("{get}.and_then(|x| x.as_array()).map(|a| a.iter().map({n}::from_json).collect()).unwrap_or_default()"),
                k @ Kind::Enum(_) => {
                    let ty = rust_ty(&k, enums);
                    format!("{get}.and_then(|x| x.as_str()).and_then(<{ty} as aylis::prelude::PropEnum>::from_name).or_else(|| <{ty} as aylis::prelude::PropEnum>::from_index(0)).expect(\"enum\")")
                }
                _ => format!("{get}.and_then(|x| x.as_str()).unwrap_or_default().to_string()"),
            };
            s.push_str(&format!("            {}: {value},\n", rust_ident(f)));
        }
        s.push_str("        }\n    }\n\n    /// O registro em JSON.\n    pub fn to_json(&self) -> aylis::json::serde_json::Value {\n        let mut m = aylis::json::serde_json::Map::new();\n");
        for (f, k) in fields {
            let id = rust_ident(f);
            let value = match field_kind(k) {
                Kind::Color => format!("aylis::json::serde_json::Value::from(self.{id}.to_hex())"),
                Kind::Date | Kind::DateTime => format!("aylis::json::serde_json::Value::from(self.{id}.map(|d| d.to_string()).unwrap_or_default())"),
                Kind::Record(_) => format!("self.{id}.as_ref().map_or(aylis::json::serde_json::Value::Null, |r| r.to_json())"),
                Kind::RecordList(_) => format!("aylis::json::serde_json::Value::Array(self.{id}.iter().map(|r| r.to_json()).collect())"),
                Kind::Enum(_) => format!("aylis::json::serde_json::Value::from(aylis::prelude::PropEnum::name(self.{id}))"),
                _ => format!("aylis::json::serde_json::Value::from(self.{id}.clone())"),
            };
            s.push_str(&format!("        m.insert(\"{f}\".into(), {value});\n"));
        }
        s.push_str(
            "        aylis::json::serde_json::Value::Object(m)\n    }\n\n    /// A lista em JSON, como a tela grava.\n    pub fn list_text(items: &[Self]) -> String {\n        aylis::json::serde_json::Value::Array(items.iter().map(Self::to_json).collect()).to_string()\n    }\n\n\
             \x20   /// A lista do texto com o JSON (vazia se não é uma lista).\n    pub fn list_from_text(text: &str) -> Vec<Self> {\n        aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(text).ok().and_then(|j| j.as_array().map(|a| a.iter().map(Self::from_json).collect())).unwrap_or_default()\n    }\n}\n",
        );
        s.push_str(&format!("\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self::from_json(&aylis::json::serde_json::Value::Null)\n    }}\n}}\n"));
    }
    s
}

fn python_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    if !order.is_empty() {
        s.push_str(
            "\n\ndef _record(cls, value):\n    \"\"\"O registro do JSON da tela (None se não há).\"\"\"\n    data = json.loads(value) if isinstance(value, str) and value else value\n    return cls.from_json(data) if isinstance(data, dict) else None\n\n\n\
             def _records(cls, value):\n    data = json.loads(value) if isinstance(value, str) and value else value\n    return [cls.from_json(x) for x in data] if isinstance(data, list) else []\n\n\n\
             def _record_text(r):\n    return json.dumps(r.to_json(), ensure_ascii=False) if r is not None else \"\"\n\n\n\
             def _records_text(items):\n    return json.dumps([r.to_json() for r in items], ensure_ascii=False)\n",
        );
    }
    for name in order {
        let fields = &records[name];
        s.push_str(&format!("\n\n@dataclass\nclass {name}:\n    \"\"\"Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).\"\"\"\n\n"));
        for (f, k) in fields {
            let (ty, default) = match field_kind(k) {
                Kind::Bool => ("bool".to_string(), "False".to_string()),
                Kind::Int => ("int".into(), "0".into()),
                Kind::Float => ("float".into(), "0.0".into()),
                Kind::Color => ("Color".into(), "dataclasses.field(default_factory=Color)".into()),
                Kind::Date => ("datetime.date | None".into(), "None".into()),
                Kind::DateTime => ("datetime.datetime | None".into(), "None".into()),
                Kind::List => ("list[str]".into(), "dataclasses.field(default_factory=list)".into()),
                Kind::NumberList => ("list[float]".into(), "dataclasses.field(default_factory=list)".into()),
                Kind::Record(n) => (format!("{n} | None"), "None".into()),
                Kind::RecordList(n) => (format!("list[{n}]"), "dataclasses.field(default_factory=list)".into()),
                Kind::Enum(n) => (n.clone(), format!("dataclasses.field(default_factory=lambda: next(iter({n})))")),
                _ => ("str".into(), "\"\"".into()),
            };
            s.push_str(&format!("    {}: \"{ty}\" = {default}\n", python_ident(f)));
        }
        s.push_str(&format!("\n    @staticmethod\n    def from_json(data):\n        return {name}(\n"));
        for (f, k) in fields {
            let get = format!("data.get(\"{f}\")");
            let value = match field_kind(k) {
                Kind::Bool => format!("bool({get} or False)"),
                Kind::Int => format!("int({get} or 0)"),
                Kind::Float => format!("float({get} or 0)"),
                Kind::Color => format!("Color.from_json({get}) if {get} else Color()"),
                Kind::Date => format!("_date({get})"),
                Kind::DateTime => format!("_datetime({get})"),
                Kind::List => format!("[str(x) for x in {get} or []]"),
                Kind::NumberList => format!("[float(x) for x in {get} or []]"),
                Kind::Record(n) => format!("{n}.from_json({get}) if isinstance({get}, dict) else None"),
                Kind::RecordList(n) => format!("[{n}.from_json(x) for x in {get} or []]"),
                Kind::Enum(n) => format!("_enum({n}, {get})"),
                _ => format!("str({get} or \"\")"),
            };
            s.push_str(&format!("            {}={value},\n", python_ident(f)));
        }
        s.push_str("        )\n\n    def to_json(self):\n        return {\n");
        for (f, k) in fields {
            let a = format!("self.{}", python_ident(f));
            let value = match field_kind(k) {
                Kind::Color => format!("{a}.to_json()"),
                Kind::Date => format!("({a}.isoformat() if {a} else \"\")"),
                Kind::DateTime => format!("_datetime_text({a})"),
                Kind::List | Kind::NumberList => format!("list({a})"),
                Kind::Record(_) => format!("({a}.to_json() if {a} else None)"),
                Kind::RecordList(_) => format!("[x.to_json() for x in {a}]"),
                Kind::Enum(_) => format!("{a}.value"),
                _ => a,
            };
            s.push_str(&format!("            \"{f}\": {value},\n"));
        }
        s.push_str("        }\n");
    }
    s
}

fn typescript_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    for name in order {
        let fields = &records[name];
        s.push_str(&format!("\n/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */\nexport interface {name} {{\n"));
        for (f, k) in fields {
            let ty = match field_kind(k) {
                Kind::Bool => "boolean".to_string(),
                Kind::Int | Kind::Float => "number".into(),
                Kind::Color => "Color".into(),
                Kind::Date => "LocalDate | null".into(),
                Kind::DateTime => "LocalDateTime | null".into(),
                Kind::List => "string[]".into(),
                Kind::NumberList => "number[]".into(),
                Kind::Record(n) => format!("{n} | null"),
                Kind::RecordList(n) => format!("{n}[]"),
                Kind::Enum(n) => n,
                _ => "string".into(),
            };
            s.push_str(&format!("    {}: {ty};\n", camel(f)));
        }
        s.push_str(&format!("}}\n\nexport function {}(v: any): {name} {{\n    return {{\n", record_fn(name)));
        for (f, k) in fields {
            let value = match field_kind(k) {
                Kind::Bool => format!("Boolean(v?.{f})"),
                Kind::Int | Kind::Float => format!("Number(v?.{f} ?? 0)"),
                Kind::Color => format!("Color.fromJson(v?.{f})"),
                Kind::Date => format!("LocalDate.fromJson(v?.{f})"),
                Kind::DateTime => format!("LocalDateTime.fromJson(v?.{f})"),
                Kind::List => format!("(v?.{f} ?? []).map(String)"),
                Kind::NumberList => format!("(v?.{f} ?? []).map(Number)"),
                Kind::Record(n) => format!("v?.{f} ? {}(v.{f}) : null", record_fn(&n)),
                Kind::RecordList(n) => format!("(v?.{f} ?? []).map({})", record_fn(&n)),
                Kind::Enum(n) => format!("(v?.{f} ?? \"\") as {n}"),
                _ => format!("String(v?.{f} ?? \"\")"),
            };
            s.push_str(&format!("        {}: {value},\n", camel(f)));
        }
        s.push_str(&format!("    }};\n}}\n\nexport function {}(r: {name}): any {{\n    return {{\n", record_to_fn(name)));
        for (f, k) in fields {
            let a = format!("r.{}", camel(f));
            let value = match field_kind(k) {
                Kind::Color => format!("{a}.toJson()"),
                Kind::Date | Kind::DateTime => format!("({a} ? {a}.toString() : \"\")"),
                Kind::Record(n) => format!("({a} ? {}({a}) : null)", record_to_fn(&n)),
                Kind::RecordList(n) => format!("{a}.map({})", record_to_fn(&n)),
                _ => a,
            };
            s.push_str(&format!("        {f}: {value},\n"));
        }
        s.push_str(&format!(
            "    }};\n}}\n\n/** Um {name} com os campos dados (o resto, o padrão). */\nexport function new{name}(init: Partial<{name}> = {{}}): {name} {{\n    return {{ ...{}({{}}), ...init }};\n}}\n",
            record_fn(name)
        ));
    }
    s
}

/// O nome da propriedade de um campo em C#: o que repete o nome do registro ganha `Value` (o C#
/// não deixa um membro com o nome do tipo: `MasterField.master_field`).
fn cs_field(record: &str, field: &str) -> String {
    let p = pascal(field);
    if p == record { format!("{p}Value") } else { p }
}

fn csharp_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    for name in order {
        let fields = &records[name];
        let params: Vec<String> = fields
            .iter()
            .map(|(f, k)| {
                let ty = match field_kind(k) {
                    Kind::Bool => "bool".to_string(),
                    Kind::Int => "long".into(),
                    Kind::Float => "double".into(),
                    Kind::Color => "Color".into(),
                    Kind::Date => "DateOnly?".into(),
                    Kind::DateTime => "DateTime?".into(),
                    Kind::List => "string[]".into(),
                    Kind::NumberList => "double[]".into(),
                    Kind::Record(n) => format!("{n}?"),
                    Kind::RecordList(n) => format!("{n}[]"),
                    Kind::Enum(n) => n,
                    _ => "string".into(),
                };
                format!("{ty} {}", cs_field(name, f))
            })
            .collect();
        let values: Vec<String> = fields
            .iter()
            .map(|(f, k)| match field_kind(k) {
                Kind::Bool => format!("Components.Bool(n?[\"{f}\"])"),
                Kind::Int => format!("(long)Components.Number(n?[\"{f}\"])"),
                Kind::Float => format!("Components.Number(n?[\"{f}\"])"),
                Kind::Color => format!("Color.FromJson(n?[\"{f}\"])"),
                Kind::Date => format!("Components.DateOf(n?[\"{f}\"])"),
                Kind::DateTime => format!("Components.DateTimeOf(n?[\"{f}\"])"),
                Kind::List => format!("n?[\"{f}\"] is JsonArray l{f} ? l{f}.Select(x => Components.Text(x)).ToArray() : []"),
                Kind::NumberList => format!("n?[\"{f}\"] is JsonArray l{f} ? l{f}.Select(x => Components.Number(x)).ToArray() : []"),
                Kind::Record(r) => format!("n?[\"{f}\"] is JsonObject o{f} ? {r}.FromJson(o{f}) : null"),
                Kind::RecordList(r) => format!("n?[\"{f}\"] is JsonArray a{f} ? a{f}.Select({r}.FromJson).ToArray() : []"),
                Kind::Enum(e) => format!("EnumNames.Of<{e}>(n?[\"{f}\"], EnumNames.{e})"),
                _ => format!("Components.Text(n?[\"{f}\"])"),
            })
            .collect();
        let to: Vec<String> = fields
            .iter()
            .map(|(f, k)| {
                let a = cs_field(name, f);
                let value = match field_kind(k) {
                    Kind::Color => format!("{a}.ToJson()"),
                    Kind::Date => format!("Components.DateText({a})"),
                    Kind::DateTime => format!("Components.DateTimeText({a})"),
                    Kind::List | Kind::NumberList => format!("new JsonArray({a}.Select(x => (JsonNode?)JsonValue.Create(x)).ToArray())"),
                    Kind::Record(_) => format!("{a}?.ToJson()"),
                    Kind::RecordList(_) => format!("new JsonArray({a}.Select(x => (JsonNode?)x.ToJson()).ToArray())"),
                    Kind::Enum(e) => format!("JsonValue.Create(EnumNames.{e}[(int){a}])"),
                    _ => format!("JsonValue.Create({a})"),
                };
                format!("[\"{f}\"] = {value}")
            })
            .collect();
        s.push_str(&format!(
            "    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>\n    public sealed record {name}({})\n    {{\n\
             \x20       /// <summary>Com os padrões: <c>new {name} {{ Campo = valor }}</c> preenche só os dados.</summary>\n        public {name}() : this(FromJson(null)) {{ }}\n\n\
             \x20       internal static {name} FromJson(JsonNode? n) => new({});\n\n\
             \x20       internal JsonObject ToJson() => new() {{ {} }};\n\n\
             \x20       internal static {name}? FromText(JsonNode? v) => Components.Text(v) is {{ Length: > 0 }} t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;\n\n\
             \x20       internal static {name}[] ListFromText(JsonNode? v) => Components.Text(v) is {{ Length: > 0 }} t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];\n\n\
             \x20       internal static string ToText({name}? r) => r?.ToJson().ToJsonString() ?? \"\";\n\n\
             \x20       internal static string ListToText(System.Collections.Generic.IEnumerable<{name}> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();\n    }}\n\n",
            params.join(", "),
            values.join(", "),
            to.join(", ")
        ));
    }
    s
}

fn java_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    for name in order {
        let fields = &records[name];
        let params: Vec<String> = fields
            .iter()
            .map(|(f, k)| {
                let ty = match field_kind(k) {
                    Kind::Bool => "boolean".to_string(),
                    Kind::Int => "long".into(),
                    Kind::Float => "double".into(),
                    Kind::Color => "Aylis.Color".into(),
                    Kind::Date => "java.time.LocalDate".into(),
                    Kind::DateTime => "java.time.LocalDateTime".into(),
                    Kind::List => "java.util.List<String>".into(),
                    Kind::NumberList => "java.util.List<Double>".into(),
                    Kind::Record(n) => n,
                    Kind::RecordList(n) => format!("java.util.List<{n}>"),
                    Kind::Enum(n) => n,
                    _ => "String".into(),
                };
                format!("{ty} {}", camel(f))
            })
            .collect();
        let values: Vec<String> = fields
            .iter()
            .map(|(f, k)| match field_kind(k) {
                Kind::Bool => format!("asBool(m.get(\"{f}\"))"),
                Kind::Int => format!("(long) asNumber(m.get(\"{f}\"))"),
                Kind::Float => format!("asNumber(m.get(\"{f}\"))"),
                Kind::Color => format!("Aylis.Color.fromJson(m.get(\"{f}\"))"),
                Kind::Date => format!("asDate(m.get(\"{f}\"))"),
                Kind::DateTime => format!("asDateTime(m.get(\"{f}\"))"),
                Kind::List => format!("m.get(\"{f}\") instanceof java.util.List<?> l{} ? l{}.stream().map(String::valueOf).toList() : java.util.List.<String>of()", camel(f), camel(f)),
                Kind::NumberList => format!("m.get(\"{f}\") instanceof java.util.List<?> l{} ? l{}.stream().map(x -> asNumber(x)).toList() : java.util.List.<Double>of()", camel(f), camel(f)),
                Kind::Record(r) => format!("m.get(\"{f}\") instanceof java.util.Map<?, ?> ? {r}.fromJson(m.get(\"{f}\")) : null"),
                Kind::RecordList(r) => format!("{r}.listFromJson(m.get(\"{f}\"))"),
                Kind::Enum(e) => format!("{e}.of(m.get(\"{f}\"))"),
                _ => format!("asText(m.get(\"{f}\"))"),
            })
            .collect();
        let to: Vec<String> = fields
            .iter()
            .map(|(f, k)| {
                let a = camel(f);
                let value = match field_kind(k) {
                    Kind::Color => format!("{a} == null ? \"\" : {a}.toJson()"),
                    Kind::Date => format!("dateText({a})"),
                    Kind::DateTime => format!("dateTimeText({a})"),
                    Kind::Record(_) => format!("{a} == null ? null : {a}.toJson()"),
                    Kind::RecordList(_) => format!("{a}.stream().map(x -> (Object) x.toJson()).toList()"),
                    Kind::Enum(_) => format!("{a} == null ? \"\" : {a}.value"),
                    _ => a,
                };
                format!("            m.put(\"{f}\", {value});\n")
            })
            .collect();
        // Os `withCampo(valor)`: o registro com um campo trocado.
        let names: Vec<String> = fields.iter().map(|(f, _)| camel(f)).collect();
        let withers: Vec<String> = fields
            .iter()
            .zip(&params)
            .map(|((f, _), p)| {
                let ty = p.rsplit_once(' ').map_or("Object", |(t, _)| t);
                let args: Vec<String> = names.iter().map(|n| if *n == camel(f) { "v".to_string() } else { n.clone() }).collect();
                format!("\n        public {name} with{}({ty} v) {{\n            return new {name}({});\n        }}\n", pascal(f), args.join(", "))
            })
            .collect();
        s.push_str(&format!(
            "\n    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */\n    public record {name}({}) {{\n\
             \x20       static {name} fromJson(Object v) {{\n            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();\n            return new {name}({});\n        }}\n\n\
             \x20       java.util.Map<String, Object> toJson() {{\n            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();\n{}            return m;\n        }}\n\n\
             \x20       static java.util.List<{name}> listFromJson(Object v) {{\n            java.util.List<{name}> out = new java.util.ArrayList<>();\n            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));\n            return out;\n        }}\n\n\
             \x20       static {name} fromText(Object v) {{\n            String t = asText(v);\n            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);\n            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;\n        }}\n\n\
             \x20       static java.util.List<{name}> listFromText(Object v) {{\n            String t = asText(v);\n            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));\n        }}\n\n\
             \x20       static String toText({name} r) {{\n            return r == null ? \"\" : Aylis.Json.write(r.toJson());\n        }}\n\n\
             \x20       static String listToText(java.util.List<{name}> items) {{\n            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());\n        }}\n\n\
             \x20       /** Com os padrões; os {{@code with...}} trocam um campo: {{@code {name}.of().with...(valor)}}. */\n        public static {name} of() {{\n            return fromJson(null);\n        }}\n{}    }}\n",
            params.join(", "),
            values.join(", "),
            to.concat(),
            withers.concat()
        ));
    }
    s
}

fn go_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    if !order.is_empty() {
        s.push_str(
            "\nfunc jsonTexts(v any) []string {\n\tl, _ := v.([]any)\n\tout := make([]string, 0, len(l))\n\tfor _, x := range l {\n\t\tout = append(out, asText(x))\n\t}\n\treturn out\n}\n\n\
             func jsonNumbers(v any) []float64 {\n\tl, _ := v.([]any)\n\tout := make([]float64, 0, len(l))\n\tfor _, x := range l {\n\t\tout = append(out, asNumber(x))\n\t}\n\treturn out\n}\n",
        );
    }
    for name in order {
        let fields = &records[name];
        s.push_str(&format!("\n// {name} é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).\ntype {name} struct {{\n"));
        for (f, k) in fields {
            let ty = match field_kind(k) {
                Kind::Bool => "bool".to_string(),
                Kind::Int => "int64".into(),
                Kind::Float => "float64".into(),
                Kind::Color => "aylis.Color".into(),
                Kind::Date => "Date".into(),
                Kind::DateTime => "DateTime".into(),
                Kind::List => "[]string".into(),
                Kind::NumberList => "[]float64".into(),
                Kind::Record(n) => format!("*{n}"),
                Kind::RecordList(n) => format!("[]{n}"),
                Kind::Enum(n) => n,
                _ => "string".into(),
            };
            s.push_str(&format!("\t{} {ty}\n", pascal(f)));
        }
        s.push_str(&format!("}}\n\nfunc recordOf{name}(v any) {name} {{\n\tm, _ := v.(map[string]any)\n\treturn {name}{{\n"));
        for (f, k) in fields {
            let value = match field_kind(k) {
                Kind::Bool => format!("asBool(m[\"{f}\"])"),
                Kind::Int => format!("int64(asNumber(m[\"{f}\"]))"),
                Kind::Float => format!("asNumber(m[\"{f}\"])"),
                Kind::Color => format!("aylis.ColorFromJSON(m[\"{f}\"])"),
                Kind::Date => format!("asDate(m[\"{f}\"])"),
                Kind::DateTime => format!("asDateTime(m[\"{f}\"])"),
                Kind::List => format!("jsonTexts(m[\"{f}\"])"),
                Kind::NumberList => format!("jsonNumbers(m[\"{f}\"])"),
                Kind::Record(r) => format!("func() *{r} {{\n\t\t\tif x, ok := m[\"{f}\"].(map[string]any); ok {{\n\t\t\t\tr := recordOf{r}(x)\n\t\t\t\treturn &r\n\t\t\t}}\n\t\t\treturn nil\n\t\t}}()"),
                Kind::RecordList(r) => format!("listOf{r}(m[\"{f}\"])"),
                Kind::Enum(e) => format!("{e}(asText(m[\"{f}\"]))"),
                _ => format!("asText(m[\"{f}\"])"),
            };
            s.push_str(&format!("\t\t{}: {value},\n", pascal(f)));
        }
        s.push_str(&format!("\t}}\n}}\n\nfunc (r {name}) jsonValue() map[string]any {{\n\treturn map[string]any{{\n"));
        for (f, k) in fields {
            let a = format!("r.{}", pascal(f));
            let value = match field_kind(k) {
                Kind::Color => format!("{a}.JSON()"),
                Kind::Date => format!("dateText({a})"),
                Kind::DateTime => format!("dateTimeText({a})"),
                Kind::Record(_) => format!("func() any {{\n\t\t\tif {a} == nil {{\n\t\t\t\treturn nil\n\t\t\t}}\n\t\t\treturn {a}.jsonValue()\n\t\t}}()"),
                Kind::RecordList(n) => format!("jsonList{n}({a})"),
                Kind::Enum(_) => format!("string({a})"),
                _ => a,
            };
            s.push_str(&format!("\t\t\"{f}\": {value},\n"));
        }
        s.push_str(&format!(
            "\t}}\n}}\n\nfunc jsonList{name}(items []{name}) []any {{\n\tout := make([]any, len(items))\n\tfor i, r := range items {{\n\t\tout[i] = r.jsonValue()\n\t}}\n\treturn out\n}}\n\n\
             func listOf{name}(v any) []{name} {{\n\tl, _ := v.([]any)\n\tout := make([]{name}, 0, len(l))\n\tfor _, x := range l {{\n\t\tout = append(out, recordOf{name}(x))\n\t}}\n\treturn out\n}}\n\n\
             func record{name}(v any) *{name} {{\n\tvar j any\n\tif json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {{\n\t\treturn nil\n\t}}\n\tr := recordOf{name}(j)\n\treturn &r\n}}\n\n\
             func records{name}(v any) []{name} {{\n\tvar j any\n\t_ = json.Unmarshal([]byte(asText(v)), &j)\n\treturn listOf{name}(j)\n}}\n\n\
             func recordText{name}(r *{name}) string {{\n\tif r == nil {{\n\t\treturn \"\"\n\t}}\n\tb, _ := json.Marshal(r.jsonValue())\n\treturn string(b)\n}}\n\n\
             func recordsText{name}(items []{name}) string {{\n\tb, _ := json.Marshal(jsonList{name}(items))\n\treturn string(b)\n}}\n"
        ));
    }
    s
}

fn cpp_records(records: &Records, order: &[String]) -> String {
    let mut s = String::new();
    // A declaração antes: um registro pode ter a lista de outro (ou dele mesmo).
    for name in order {
        s.push_str(&format!("struct {name};\n"));
    }
    for name in order {
        let fields = &records[name];
        s.push_str(&format!("\n// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).\nstruct {name} {{\n"));
        for (f, k) in fields {
            let id = cpp_ident(f);
            let decl = match field_kind(k) {
                Kind::Bool => format!("bool {id} = false;"),
                Kind::Int => format!("long long {id} = 0;"),
                Kind::Float => format!("double {id} = 0;"),
                Kind::Color => format!("aylis::Color {id};"),
                Kind::Date => format!("std::optional<LocalDate> {id};"),
                Kind::DateTime => format!("std::optional<LocalDateTime> {id};"),
                Kind::List => format!("std::vector<std::string> {id};"),
                Kind::NumberList => format!("std::vector<double> {id};"),
                Kind::Record(n) => format!("std::shared_ptr<{n}> {id};"),
                Kind::RecordList(n) => format!("std::vector<{n}> {id};"),
                Kind::Enum(n) => format!("{n} {id} = static_cast<{n}>(0);"),
                _ => format!("std::string {id};"),
            };
            s.push_str(&format!("    {decl}\n"));
        }
        s.push_str(&format!("\n    static {name} from_json(const aylis::Json& v) {{\n        {name} r;\n"));
        for (f, k) in fields {
            let id = cpp_ident(f);
            let value = match field_kind(k) {
                Kind::Bool => format!("r.{id} = v[\"{f}\"].boolean();"),
                Kind::Int => format!("r.{id} = static_cast<long long>(v[\"{f}\"].number());"),
                Kind::Float => format!("r.{id} = v[\"{f}\"].number();"),
                Kind::Color => format!("r.{id} = aylis::Color::from_json(v[\"{f}\"]);"),
                Kind::Date => format!("r.{id} = LocalDate::parse(v[\"{f}\"].str());"),
                Kind::DateTime => format!("r.{id} = LocalDateTime::parse(v[\"{f}\"].str());"),
                Kind::List => format!("for (const auto& x : v[\"{f}\"].items()) r.{id}.push_back(x.str());"),
                Kind::NumberList => format!("for (const auto& x : v[\"{f}\"].items()) r.{id}.push_back(x.number());"),
                Kind::Record(n) => format!("if (v[\"{f}\"].kind() == aylis::Json::Kind::Object) r.{id} = std::make_shared<{n}>({n}::from_json(v[\"{f}\"]));"),
                Kind::RecordList(n) => format!("for (const auto& x : v[\"{f}\"].items()) r.{id}.push_back({n}::from_json(x));"),
                Kind::Enum(n) => format!("r.{id} = enum_of_<{n}>(v[\"{f}\"], {n}_names);"),
                _ => format!("r.{id} = v[\"{f}\"].str();"),
            };
            s.push_str(&format!("        {value}\n"));
        }
        s.push_str("        return r;\n    }\n\n    aylis::Json to_json() const {\n        auto o = aylis::Json::object();\n");
        for (f, k) in fields {
            let id = cpp_ident(f);
            let value = match field_kind(k) {
                Kind::Color => format!("aylis::Json({id}.to_json())"),
                Kind::Date | Kind::DateTime => format!("aylis::Json(text_of_({id}))"),
                Kind::List => format!("[&] {{ std::vector<aylis::Json> a; for (const auto& x : {id}) a.emplace_back(x); return aylis::Json::array(a); }}()"),
                Kind::NumberList => format!("[&] {{ std::vector<aylis::Json> a; for (double x : {id}) a.emplace_back(x); return aylis::Json::array(a); }}()"),
                Kind::Record(_) => format!("{id} ? {id}->to_json() : aylis::Json()"),
                Kind::RecordList(_) => format!("[&] {{ std::vector<aylis::Json> a; for (const auto& x : {id}) a.push_back(x.to_json()); return aylis::Json::array(a); }}()"),
                Kind::Enum(n) => format!("aylis::Json({n}_names[static_cast<int>({id})])"),
                _ => format!("aylis::Json({id})"),
            };
            s.push_str(&format!("        o.set(\"{f}\", {value});\n"));
        }
        s.push_str(&format!(
            "        return o;\n    }}\n\n    static std::optional<{name}> from_text(const aylis::Json& v) {{\n        auto j = aylis::Json::parse(v.str());\n        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;\n        return from_json(j);\n    }}\n\
             \x20   static std::vector<{name}> list_from_text(const aylis::Json& v) {{\n        // O JSON numa variável: o temporário do `parse` morreria antes do laço.\n        const auto j = aylis::Json::parse(v.str());\n        std::vector<{name}> out;\n        for (const auto& x : j.items()) out.push_back(from_json(x));\n        return out;\n    }}\n\
             \x20   static std::string to_text(const std::optional<{name}>& r) {{ return r ? r->to_json().dump() : std::string(); }}\n\
             \x20   static std::string list_to_text(const std::vector<{name}>& items) {{\n        std::vector<aylis::Json> a;\n        for (const auto& x : items) a.push_back(x.to_json());\n        return aylis::Json::array(a).dump();\n    }}\n}};\n"
        ));
    }
    s
}

/// Os registros em Pascal: os tipos (no `type`), as declarações e os corpos das leituras.
fn pascal_records(records: &Records, order: &[String]) -> (String, String, String) {
    let (mut head, mut decls, mut body) = (String::new(), String::new(), String::new());
    if !order.is_empty() {
        decls.push_str("function AylisJSONTexts(Data: TJSONData): TStringArray;\nfunction AylisJSONNumbers(Data: TJSONData): TAylisDoubleArray;\nfunction AylisTextsToJSON(const A: TStringArray): TJSONArray;\nfunction AylisNumbersToJSON(const A: TAylisDoubleArray): TJSONArray;\n");
        body.push_str(
            "function AylisJSONTexts(Data: TJSONData): TStringArray;\nvar\n  I: Integer;\nbegin\n  Result := nil;\n  if (Data = nil) or (Data.JSONType <> jtArray) then\n    Exit;\n  SetLength(Result, Data.Count);\n  for I := 0 to Data.Count - 1 do\n    Result[I] := AsText(Data.Items[I]);\nend;\n\n\
             function AylisJSONNumbers(Data: TJSONData): TAylisDoubleArray;\nvar\n  I: Integer;\nbegin\n  Result := nil;\n  if (Data = nil) or (Data.JSONType <> jtArray) then\n    Exit;\n  SetLength(Result, Data.Count);\n  for I := 0 to Data.Count - 1 do\n    Result[I] := Data.Items[I].AsFloat;\nend;\n\n\
             function AylisTextsToJSON(const A: TStringArray): TJSONArray;\nvar\n  I: Integer;\nbegin\n  Result := TJSONArray.Create;\n  for I := 0 to High(A) do\n    Result.Add(A[I]);\nend;\n\n\
             function AylisNumbersToJSON(const A: TAylisDoubleArray): TJSONArray;\nvar\n  I: Integer;\nbegin\n  Result := TJSONArray.Create;\n  for I := 0 to High(A) do\n    Result.Add(A[I]);\nend;\n\n",
        );
    }
    // Os tipos das listas antes: um registro pode ter a lista de outro (ou dele mesmo).
    for name in order {
        head.push_str(&format!("  P{name} = ^T{name};\n"));
    }
    for name in order {
        let fields = &records[name];
        head.push_str(&format!("  {{ Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }}\n  T{name} = record\n"));
        for (f, k) in fields {
            let ty = match field_kind(k) {
                Kind::Bool => "Boolean".to_string(),
                Kind::Int => "Int64".into(),
                Kind::Float => "Double".into(),
                Kind::Color => "TAylisColor".into(),
                Kind::Date | Kind::DateTime => "TDateTime".into(),
                Kind::List => "TStringArray".into(),
                Kind::NumberList => "TAylisDoubleArray".into(),
                Kind::Record(n) => format!("P{n}"),
                Kind::RecordList(n) => format!("array of T{n}"),
                Kind::Enum(n) => format!("T{n}"),
                _ => "string".into(),
            };
            head.push_str(&format!("    {}: {ty};\n", pascal_ident(&pascal(f))));
        }
        head.push_str(&format!("  end;\n  T{name}Array = array of T{name};\n\n"));
        decls.push_str(&format!(
            "function {name}FromJSON(Data: TJSONData): T{name};\nfunction {name}ListFromJSON(Data: TJSONData): T{name}Array;\nfunction {name}FromText(Data: TJSONData): T{name};\nfunction {name}ListFromText(Data: TJSONData): T{name}Array;\n\
             function {name}ToJSON(const R: T{name}): TJSONObject;\nfunction {name}ListToJSON(const A: array of T{name}): TJSONArray;\nfunction {name}Text(const R: T{name}): TJSONData;\nfunction {name}ListText(const A: T{name}Array): TJSONData;\n"
        ));
        body.push_str(&format!("function {name}FromJSON(Data: TJSONData): T{name};\nvar\n  O: TJSONObject;\nbegin\n  Result := Default(T{name});\n  if (Data = nil) or (Data.JSONType <> jtObject) then\n    Exit;\n  O := TJSONObject(Data);\n"));
        for (f, k) in fields {
            let id = pascal_ident(&pascal(f));
            let value = match field_kind(k) {
                Kind::Bool => format!("O.Get('{f}', False)"),
                Kind::Int => format!("O.Get('{f}', Int64(0))"),
                Kind::Float => format!("O.Get('{f}', Double(0))"),
                Kind::Color => format!("AylisColorFromJSON(O.Find('{f}'))"),
                Kind::Date => format!("AsDate(O.Find('{f}'))"),
                Kind::DateTime => format!("AsDateTime(O.Find('{f}'))"),
                Kind::List => format!("AylisJSONTexts(O.Find('{f}'))"),
                Kind::NumberList => format!("AylisJSONNumbers(O.Find('{f}'))"),
                Kind::Record(n) => {
                    body.push_str(&format!("  if (O.Find('{f}') <> nil) and (O.Find('{f}').JSONType = jtObject) then\n  begin\n    New(Result.{id});\n    Result.{id}^ := {n}FromJSON(O.Find('{f}'));\n  end;\n"));
                    continue;
                }
                Kind::RecordList(n) => format!("{n}ListFromJSON(O.Find('{f}'))"),
                Kind::Enum(n) => format!("{n}Of(O.Find('{f}'))"),
                _ => format!("O.Get('{f}', '')"),
            };
            body.push_str(&format!("  Result.{id} := {value};\n"));
        }
        body.push_str(&format!("end;\n\nfunction {name}ToJSON(const R: T{name}): TJSONObject;\nbegin\n  Result := TJSONObject.Create;\n"));
        for (f, k) in fields {
            let id = pascal_ident(&pascal(f));
            let value = match field_kind(k) {
                Kind::Color => format!("AylisColorToJSON(R.{id})"),
                Kind::Date => format!("DateText(R.{id})"),
                Kind::DateTime => format!("DateTimeText(R.{id})"),
                Kind::List => format!("AylisTextsToJSON(R.{id})"),
                Kind::NumberList => format!("AylisNumbersToJSON(R.{id})"),
                Kind::Record(n) => {
                    body.push_str(&format!("  if R.{id} <> nil then\n    Result.Add('{f}', {n}ToJSON(R.{id}^))\n  else\n    Result.Add('{f}', TJSONNull.Create);\n"));
                    continue;
                }
                Kind::RecordList(n) => format!("{n}ListToJSON(R.{id})"),
                Kind::Enum(n) => format!("{n}Names[R.{id}]"),
                _ => format!("R.{id}"),
            };
            body.push_str(&format!("  Result.Add('{f}', {value});\n"));
        }
        body.push_str(&format!(
            "end;\n\nfunction {name}ListFromJSON(Data: TJSONData): T{name}Array;\nvar\n  I: Integer;\nbegin\n  Result := nil;\n  if (Data = nil) or (Data.JSONType <> jtArray) then\n    Exit;\n  SetLength(Result, Data.Count);\n  for I := 0 to Data.Count - 1 do\n    Result[I] := {name}FromJSON(Data.Items[I]);\nend;\n\n\
             function {name}ListToJSON(const A: array of T{name}): TJSONArray;\nvar\n  I: Integer;\nbegin\n  Result := TJSONArray.Create;\n  for I := 0 to High(A) do\n    Result.Add({name}ToJSON(A[I]));\nend;\n\n\
             function {name}Text(const R: T{name}): TJSONData;\nvar\n  J: TJSONObject;\nbegin\n  J := {name}ToJSON(R);\n  try\n    Result := TJSONString.Create(J.AsJSON);\n  finally\n    J.Free;\n  end;\nend;\n\n\
             function {name}ListText(const A: T{name}Array): TJSONData;\nvar\n  J: TJSONArray;\nbegin\n  J := {name}ListToJSON(A);\n  try\n    Result := TJSONString.Create(J.AsJSON);\n  finally\n    J.Free;\n  end;\nend;\n\n\
             function {name}FromText(Data: TJSONData): T{name};\nvar\n  J: TJSONData;\nbegin\n  try\n    J := GetJSON(AsText(Data));\n  except\n    J := nil;\n  end;\n  try\n    Result := {name}FromJSON(J);\n  finally\n    J.Free;\n  end;\nend;\n\n\
             function {name}ListFromText(Data: TJSONData): T{name}Array;\nvar\n  J: TJSONData;\nbegin\n  try\n    J := GetJSON(AsText(Data));\n  except\n    J := nil;\n  end;\n  try\n    Result := {name}ListFromJSON(J);\n  finally\n    J.Free;\n  end;\nend;\n\n"
        ));
    }
    (head, decls, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_in_each_language() {
        assert_eq!(pascal("font_size"), "FontSize");
        assert_eq!(camel("item_index"), "itemIndex");
        assert_eq!(rust_ident("type"), "r#type");
        assert_eq!(python_ident("from"), "from_");
        assert_eq!(cpp_ident("default"), "default_");
        assert_eq!(pascal_ident("Label"), "&Label");
        assert_eq!(pascal_ident("Visible"), "Visible");
    }
}
