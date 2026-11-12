//! Acesso tipado aos componentes, como no Delphi: cada form vira uma instância global com
//! o nome dele e um campo por componente, e cada componente tem as propriedades com o tipo
//! certo — `frmCalculadora.pnlLateral.Visible` em vez de `Aylis.Call('get', ...)`.
//!
//! As classes e propriedades vêm de `resources/components.json` (o catálogo do registro;
//! veja `aylis_json::catalog`). Para cada linguagem a geração escreve um arquivo
//! `components` com as classes dos componentes que os forms usam, a classe de cada form e
//! as instâncias; o registro dos forms liga o cliente a elas, e o arquivo de eventos de cada
//! form ganha o `import`/`uses` da instância dele. Ler é um `get` na tela (espera a
//! resposta; no TypeScript, `await`); gravar é um `set`.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde_json::Value;

use crate::{Language, Project};

/// O tipo de uma propriedade no código.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Bool,
    Int,
    Float,
    /// Texto, cor (`#RRGGBB`), enum (o nome) ou referência a outro componente (o nome).
    Text,
    /// Um componente como argumento de método (`scroll_to(componente)`): cada linguagem recebe o
    /// objeto dela e manda o id. Só nos métodos; as propriedades de referência são `Text`.
    Component,
}

#[derive(Clone, Debug)]
struct Prop {
    name: String,
    kind: Kind,
    read_only: bool,
}

/// O catálogo: as propriedades de cada classe, na ordem do Object Inspector.
pub struct Catalog(serde_json::Map<String, Value>);

impl Catalog {
    /// O de `resources/components.json` (vazio se não houver).
    pub fn load() -> Catalog {
        let path = crate::codegen::resources_dir().join("components.json");
        let map = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v.as_object().cloned()).unwrap_or_default();
        Catalog(map)
    }

    /// A classe está no catálogo (sem montar a lista das propriedades: o filtro de cada componente
    /// de cada form montava, 50 mil vezes no ERP de 1009 forms).
    fn has(&self, class: &str) -> bool {
        self.0.get(class).is_some_and(Value::is_object)
    }

    /// A classe do catálogo mais parecida com `class`: a que está contida no nome
    /// (`ImageView` → `Image`) ou a de menos letras trocadas (até 3).
    fn closest(&self, class: &str) -> Option<String> {
        let names: Vec<&String> = self.0.iter().filter(|(_, v)| v.is_object()).map(|(k, _)| k).collect();
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
        serde_json::to_string(&self.0).unwrap_or_default().hash(&mut h);
        format!("{:016x}", h.finish())
    }

    fn props(&self, class: &str) -> Option<Vec<Prop>> {
        let map = self.0.get(class)?.as_object()?;
        Some(
            map.iter()
                .filter(|(name, _)| !name.contains('('))
                .filter_map(|(name, kind)| {
                    let text = kind.as_str()?;
                    // Os eventos-pergunta (`"on_close_query": "question:can_close"`) não são propriedades.
                    if text.starts_with("question") || text == "coalesce" {
                        return None;
                    }
                    let (kind, read_only) = text.split_once(':').map_or((text, false), |(k, ro)| (k, ro == "ro"));
                    Some(Prop { name: name.clone(), kind: kind_of(kind), read_only })
                })
                .collect(),
        )
    }

    /// Os métodos (`"resize(width:int,height:int)": "invoke:bool"`; o argumento sem nome
    /// vem só com o tipo, `"move_to_monitor(int)"`).
    fn methods(&self, class: &str) -> Vec<Method> {
        let Some(map) = self.0.get(class).and_then(Value::as_object) else { return Vec::new() };
        map.iter()
            .filter_map(|(key, value)| {
                let (name, rest) = key.split_once('(')?;
                let (mut args, mut params) = (Vec::new(), Vec::new());
                for (i, arg) in rest.trim_end_matches(')').split(',').filter(|a| !a.is_empty()).enumerate() {
                    let (param, kind) = arg.split_once(':').unwrap_or(("", arg));
                    args.push(if kind == "component" { Kind::Component } else { kind_of(kind) });
                    // Só um identificador simples vira nome (o resto, o `a1` de sempre).
                    let ok = !param.is_empty() && param.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') && param.starts_with(|c: char| c.is_ascii_lowercase());
                    params.push(if ok { param.to_string() } else { format!("a{}", i + 1) });
                }
                let (via, ret) = value.as_str()?.split_once(':')?;
                let returns = (ret != "void").then(|| kind_of(ret));
                Some(Method { name: name.to_string(), args, params, via: via.to_string(), returns })
            })
            .collect()
    }
}

fn kind_of(text: &str) -> Kind {
    match text {
        "bool" => Kind::Bool,
        "int" => Kind::Int,
        "float" => Kind::Float,
        _ => Kind::Text,
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

    /// Os argumentos como vão no pedido: o componente vira o id dele (`field` diz como a
    /// linguagem lê o id de um objeto: `{}.id`); os outros, como vieram.
    fn arg_values(&self, field: &str) -> Vec<String> {
        self.arg_names().into_iter().zip(&self.args).map(|(a, k)| if *k == Kind::Component { field.replace("{}", &a) } else { a }).collect()
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

/// O tipo de um argumento de método em Rust.
fn rust_arg(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "bool",
        Kind::Int => "i64",
        Kind::Float => "f64",
        Kind::Text => "&str",
        Kind::Component => "impl Handle",
    }
}

fn ts_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "boolean",
        Kind::Int | Kind::Float => "number",
        Kind::Text => "string",
        Kind::Component => "AylisComponent",
    }
}

fn cs_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "bool",
        Kind::Int => "long",
        Kind::Float => "double",
        Kind::Text => "string",
        Kind::Component => "AylisComponent",
    }
}

fn java_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "boolean",
        Kind::Int => "long",
        Kind::Float => "double",
        Kind::Text => "String",
        Kind::Component => "Aylis.AylisComponent",
    }
}

fn go_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "bool",
        Kind::Int => "int64",
        Kind::Float => "float64",
        Kind::Text => "string",
        Kind::Component => "aylis.Ref",
    }
}

fn cpp_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "bool",
        Kind::Int => "long long",
        Kind::Float => "double",
        Kind::Text => "const std::string&",
        Kind::Component => "const aylis::Component&",
    }
}

/// `function Resize(A1: Int64; A2: Int64): Boolean` (com `T<Classe>.` na implementação).
fn pascal_method_head(mth: &Method, class: Option<&str>) -> String {
    let params: Vec<String> = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| format!("{}: {}", pascal_arg(a), pascal_type(*k))).collect();
    let params = if params.is_empty() { String::new() } else { format!("({})", params.join("; ")) };
    let name = match class {
        Some(c) => format!("{c}.{}", pascal_ident(&pascal(&mth.name))),
        None => pascal_ident(&pascal(&mth.name)),
    };
    match mth.returns {
        Some(k) => format!("function {name}{params}: {}", pascal_type(k)),
        None => format!("procedure {name}{params}"),
    }
}

/// O argumento em Pascal, como no Delphi (`path` → `APath`; o sem nome, `a1` → `A1`): com o
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
    if PYTHON_KEYWORDS.contains(&name) { format!("{name}_") } else { name.to_string() }
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
    let methods = classes.iter().map(|c| (c.clone(), catalog.methods(c))).collect();
    let classes = classes.into_iter().filter_map(|c| catalog.props(&c).map(|p| (c, p))).collect();
    Model { forms, classes, methods }
}

// ---------------------------------------------------------------------------
// Um pedaço por form e por classe

/// O que um laço do gerador percorre: as classes ou os forms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Class,
    Form,
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
const GENERATOR_VERSION: u32 = 11;

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
    let list: String = project.forms.iter().map(|e| format!("{}\n", e.name)).collect();
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
        let skeleton = generator(language, &Model { forms: Vec::new(), classes: Vec::new(), methods: Default::default() });
        let (mut class_loop, mut form_loop) = (0, 0);
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
            if text.contains("import static forms.Components.") {
                return None;
            }
            insert_after(text, &|l| l.starts_with("package ") || l.starts_with("import "), &format!("import static forms.Components.{form};"))
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
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        s.push_str(&format!(
            "\n/// Um {class} de um form.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {class} {{\n    pub id: ComponentId,\n}}\n\n\
             impl Handle for {class} {{\n    fn id(&self) -> ComponentId {{\n        self.id\n    }}\n}}\n\nimpl {class} {{\n"
        ));
        for p in props {
            let method = rust_ident(&p.name);
            let (ty, read) = match p.kind {
                Kind::Bool => ("bool", "handles::read(ui, self.id, PROP).and_then(|v| v.to_bool().ok()).unwrap_or_default()"),
                Kind::Int => ("i64", "handles::read(ui, self.id, PROP).and_then(|v| v.to_i64().ok()).unwrap_or_default()"),
                Kind::Float => ("f64", "handles::read(ui, self.id, PROP).and_then(|v| v.to_f64().ok()).unwrap_or_default()"),
                Kind::Text | Kind::Component => ("String", "handles::text(ui, handles::read(ui, self.id, PROP))"),
            };
            let prop = format!("\"{}\"", p.name);
            s.push_str(&format!("    pub fn {method}(self, ui: &Ui) -> {ty} {{\n        {}\n    }}\n", read.replace("PROP", &prop)));
            if !p.read_only {
                let arg = if p.kind == Kind::Text { "value: impl Into<String>" } else { &format!("value: {ty}") };
                let val = if p.kind == Kind::Text { "Into::<String>::into(value)" } else { "value" };
                s.push_str(&format!("    pub fn set_{}(self, ui: &mut Ui, {arg}) {{\n        handles::write(ui, self.id, {prop}, {val})\n    }}\n", p.name));
            }
        }
        for mth in m.methods(class) {
            let name = rust_ident(&mth.name);
            match mth.via.as_str() {
                "invoke" => {
                    let params: String = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| format!(", {a}: {}", rust_arg(*k))).collect();
                    let values: Vec<String> = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| match k { Kind::Text => format!("PropValue::String({a}.to_string())"), Kind::Component => format!("PropValue::Component(Some({a}.id()))"), _ => format!("{a}.into()") }).collect();
                    // A falha volta como `Err` com o motivo (e vai para a atividade do app).
                    let call = format!("handles::try_call(ui, self.id, \"{}\", &[{}])", mth.name, values.join(", "));
                    let (ret, body) = match mth.returns {
                        None => ("()", format!("{call}.map(|_| ())")),
                        Some(Kind::Bool) => ("bool", format!("{call}.map(|v| v.to_bool().unwrap_or_default())")),
                        Some(Kind::Int) => ("i64", format!("{call}.map(|v| v.to_i64().unwrap_or_default())")),
                        Some(Kind::Float) => ("f64", format!("{call}.map(|v| v.to_f64().unwrap_or_default())")),
                        Some(Kind::Text | Kind::Component) => ("String", format!("let v = {call}?;\n        Ok(handles::text(ui, Some(v)))")),
                    };
                    s.push_str(&format!("    pub fn {name}(self, ui: &mut Ui{params}) -> Result<{ret}, String> {{\n        {body}\n    }}\n"));
                }
                "show" => s.push_str("    pub fn show(self, ui: &mut Ui) {\n        let _ = ui.show(self.id);\n    }\n"),
                "close" => s.push_str("    pub fn close(self, ui: &mut Ui) -> bool {\n        ui.close(self.id)\n    }\n"),
                // O foco: `set_focus` do controle; no form, quem tem o foco e o Tab pelo código.
                "focus" => s.push_str("    pub fn set_focus(self, ui: &mut Ui) -> bool {\n        ui.set_focus(self.id)\n    }\n"),
                "focused" => s.push_str(
                    "    /// O nome do controle com foco (vazio sem foco), como o `ActiveControl`.\n    pub fn focused(self, ui: &Ui) -> String {\n        ui.focused(ui.root_of(self.id)).and_then(|f| ui.name(f)).unwrap_or_default().to_string()\n    }\n",
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
        // Criar uma janela do form (o `Application.CreateForm` do Delphi), sem o nome em texto
        // no código do usuário: `FrmClientes::create(ui)?` e depois `ui.show(form)`.
        s.push_str(&format!(
            "    /// Cria o {name} (ou devolve o que já está criado, como o `CreateForm`); mostrar é com\n\
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
        // Outro form chegando neste (o `Form2.Edit1` do Delphi): o primeiro aberto.
        s.push_str(&format!(
            "    /// O {name} aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.\n\
             \x20   pub fn get(ui: &Ui) -> Option<&{ty}> {{\n        ui.form_instance(ui.find_path(\"{name}\")?)\n    }}\n\n\
             \x20   pub fn get_mut(ui: &mut Ui) -> Option<&mut {ty}> {{\n        let form = ui.find_path(\"{name}\")?;\n        ui.form_instance_mut(form)\n    }}\n",
            name = f.name
        ));
        // Pelo nome (`self.find_button("btn7")`, como o FindComponent) e o do evento
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

fn python(m: &Model) -> Out {
    let s = format!("\"\"\"{HEADER}\"\"\"\n\nfrom aylis_client import CurrentForm, AylisComponent, AylisForm, register_form_class\n");
    let mut s = Out::from(s);
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        // O Form é a base das classes dos forms (show e close já vêm do AylisForm).
        let base = if class == "Form" { "AylisForm" } else { "AylisComponent" };
        s.push_str(&format!("\n\nclass {class}({base}):\n    \"\"\"Um {class} de um form.\"\"\"\n"));
        for p in props {
            let name = python_ident(&p.name);
            s.push_str(&format!("\n    @property\n    def {name}(self):\n        return self._get(\"{}\")\n", p.name));
            if !p.read_only {
                s.push_str(&format!("\n    @{name}.setter\n    def {name}(self, value):\n        self._set(\"{}\", value)\n", p.name));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| class != "Form" || !matches!(mth.via.as_str(), "show" | "close")) {
            let args = mth.arg_names();
            let params: String = args.iter().map(|a| format!(", {a}")).collect();
            let fields: String = mth.fields(&format!("[{}]", mth.arg_values("{}.id").join(", "))).iter().map(|(k, v)| format!(", {k}={v}")).collect();
            let call = format!("self.aylis.{}(\"{}\", target=self.id{fields})", if mth.waits() { "call" } else { "send" }, mth.command());
            let body = if mth.waits() { format!("return {call}") } else { call };
            s.push_str(&format!("\n    def {}(self{params}):\n        {body}\n", python_ident(&mth.name)));
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
            s.push_str(&format!("\n    def find_{sn}(self, name):\n        \"\"\"O {class} deste form com o nome (como o FindComponent).\"\"\"\n"));
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

fn typescript(m: &Model) -> Out {
    let s = format!(
        "// {}\n\nimport type {{ ComponentInfo, AylisEvent }} from \"../aylis.ts\";\nimport {{ AylisComponent, AylisForm, registerFormClass }} from \"../aylis.ts\";\n",
        HEADER.replace('\n', "\n// ")
    );
    let mut s = Out::from(s);
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        // O Form é a base das classes dos forms (show e close já vêm do AylisForm).
        let base = if class == "Form" { "AylisForm" } else { "AylisComponent" };
        s.push_str(&format!("\n/** Um {class} de um form. */\nexport class {class} extends {base} {{\n"));
        for p in props {
            let ty = match p.kind {
                Kind::Bool => "boolean",
                Kind::Int | Kind::Float => "number",
                Kind::Text | Kind::Component => "string",
            };
            let name = camel(&p.name);
            s.push_str(&format!("    get {name}(): Promise<{ty}> {{\n        return this.getProp(\"{}\");\n    }}\n", p.name));
            if !p.read_only {
                s.push_str(&format!("    set {name}(value: {ty}) {{\n        this.setProp(\"{}\", value);\n    }}\n", p.name));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| class != "Form" || !matches!(mth.via.as_str(), "show" | "close")) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{a}: {}", ts_type(*k))).collect();
            let fields: String = mth.fields(&format!("[{}]", mth.arg_values("{}.id").join(", "))).iter().map(|(k, v)| format!(", {k}: {v}")).collect();
            let request = format!("(\"{}\", {{ target: this.id{fields} }})", mth.command());
            let (ret, body) = match mth.returns {
                Some(k) => (format!(": Promise<{}>", ts_type(k)), format!("return this.aylis.call{request} as Promise<any>;")),
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
            s.push_str(&format!("    /** O {class} deste form com o nome (como o FindComponent). */\n    find{class}(name: string): {class} | undefined {{\n        switch (name) {{\n"));
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

fn csharp(m: &Model) -> Out {
    let mut s = format!("// {}\n\nusing System.Text.Json.Nodes;\nusing AylisClient;\n\nnamespace App.Forms\n{{\n", HEADER.replace('\n', "\n// "));
    s.push_str(
        "    /// <summary>Os forms (<c>using static App.Forms.Components;</c>): cada frmX é a janela do\n    /// form cujo evento está rodando (a primeira, fora dos handlers).</summary>\n    public static class Components\n    {\n",
    );
    s.push_str("        internal static bool Bool(JsonNode? v) => v is JsonValue j && j.TryGetValue<bool>(out var b) && b;\n");
    s.push_str("        internal static double Number(JsonNode? v) => v is JsonValue j && j.TryGetValue<double>(out var d) ? d : 0;\n");
    s.push_str("        internal static string Text(JsonNode? v) => v is JsonValue j && j.TryGetValue<string>(out var t) ? t : v?.ToJsonString() ?? \"\";\n");
    let mut s = Out::from(s);
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
            let (ty, read) = match p.kind {
                Kind::Bool => ("bool", "Bool"),
                Kind::Int => ("long", "(long)Number"),
                Kind::Float => ("double", "Number"),
                Kind::Text | Kind::Component => ("string", "Text"),
            };
            let get = format!("{read}(GetProp(\"{}\"))", p.name);
            if p.read_only {
                s.push_str(&format!("        public {hide}{ty} {name} => {get};\n"));
            } else {
                s.push_str(&format!("        public {hide}{ty} {name} {{ get => {get}; set => SetProp(\"{}\", value); }}\n", p.name));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !BASE_METHODS.contains(&pascal(&mth.name).as_str())) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", cs_type(*k))).collect();
            let list = if args.is_empty() { "new JsonArray()".to_string() } else { format!("new JsonArray({})", mth.arg_values("{}.Id").join(", ")) };
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", [\"{k}\"] = {v}")).collect();
            let request = format!("(\"{}\", new JsonObject {{ [\"target\"] = Id{fields} }})", mth.command());
            let (ret, body) = match mth.returns {
                Some(Kind::Bool) => ("bool", format!("Bool(Aylis.Call{request})")),
                Some(Kind::Int) => ("long", format!("(long)Number(Aylis.Call{request})")),
                Some(Kind::Float) => ("double", format!("Number(Aylis.Call{request})")),
                Some(Kind::Text | Kind::Component) => ("string", format!("Text(Aylis.Call{request})")),
                None => ("void", format!("Aylis.Send{request}")),
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
            s.push_str(&format!("\n        /// <summary>O {class} deste form com o nome (como o FindComponent).</summary>\n        public {class}? Find{class}(string name) => name switch\n        {{\n"));
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

fn java(m: &Model) -> Out {
    let mut s = format!("// {}\n\npackage forms;\n\nimport dev.aylis.Aylis;\n\n", HEADER.replace('\n', "\n// "));
    s.push_str(
        "/** Os forms ({@code import static forms.Components.frmX;}: a janela do form cujo evento está\n *  rodando, a primeira fora dos handlers) e as classes dos componentes. */\npublic final class Components {\n    private Components() {}\n\n",
    );
    s.push_str("    static boolean asBool(Object v) {\n        return v instanceof Boolean b && b;\n    }\n\n    static double asNumber(Object v) {\n        return v instanceof Number n ? n.doubleValue() : 0;\n    }\n\n    static String asText(Object v) {\n        return v == null ? \"\" : v.toString();\n    }\n");
    let mut s = Out::from(s);
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
            let (ty, read, prefix) = match p.kind {
                Kind::Bool => ("boolean", "asBool", "is"),
                Kind::Int => ("long", "(long) asNumber", "get"),
                Kind::Float => ("double", "asNumber", "get"),
                Kind::Text | Kind::Component => ("String", "asText", "get"),
            };
            s.push_str(&format!("\n        public {ty} {prefix}{name}() {{\n            return {read}(getProp(\"{}\"));\n        }}\n", p.name));
            if !p.read_only {
                s.push_str(&format!("\n        public void set{name}({ty} value) {{\n            setProp(\"{}\", value);\n        }}\n", p.name));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !BASE_METHODS.contains(&camel(&mth.name).as_str())) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", java_type(*k))).collect();
            let list = format!("java.util.List.of({})", mth.arg_values("{}.id").join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", \"{k}\", {v}")).collect();
            let request = format!("(\"{}\", Aylis.obj(\"target\", id{fields}))", mth.command());
            let (ret, body) = match mth.returns {
                Some(Kind::Bool) => ("boolean", format!("return asBool(aylis.call{request});")),
                Some(Kind::Int) => ("long", format!("return (long) asNumber(aylis.call{request});")),
                Some(Kind::Float) => ("double", format!("return asNumber(aylis.call{request});")),
                Some(Kind::Text | Kind::Component) => ("String", format!("return asText(aylis.call{request});")),
                None => ("void", format!("aylis.send{request};")),
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
            s.push_str(&format!("\n        /** O {class} deste form com o nome (como o FindComponent). */\n        public {class} find{class}(String name) {{\n            return switch (name) {{\n"));
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

fn go(m: &Model) -> Out {
    let mut s = format!("// {}\n\npackage forms\n\nimport (\n\t\"fmt\"\n\n\t\"aylis.dev/cliente/aylis\"\n)\n\n", HEADER.replace('\n', "\n// "));
    s.push_str("var _ = fmt.Sprint\n\n");
    s.push_str("func asBool(v any) bool {\n\tb, _ := v.(bool)\n\treturn b\n}\n\nfunc asNumber(v any) float64 {\n\tswitch n := v.(type) {\n\tcase float64:\n\t\treturn n\n\tcase int:\n\t\treturn float64(n)\n\tcase int64:\n\t\treturn float64(n)\n\t}\n\treturn 0\n}\n\n");
    s.push_str("func asText(v any) string {\n\tif t, ok := v.(string); ok {\n\t\treturn t\n\t}\n\tif v == nil {\n\t\treturn \"\"\n\t}\n\treturn fmt.Sprint(v)\n}\n");
    s.push_str("\nfunc askCmd(app *aylis.Aylis, cmd string, fields aylis.M) any {\n\tv, err := app.Call(cmd, fields)\n\tif err != nil {\n\t\taylis.Log(cmd+\":\", err)\n\t}\n\treturn v\n}\n");
    let mut s = Out::from(s);
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let ty = format!("aylis{class}");
        let embed = if class == "Form" { "aylis.Form" } else { "aylis.Component" };
        s.push_str(&format!("\n// {ty} é um {class} de um form.\ntype {ty} struct{{ {embed} }}\n"));
        for p in props {
            let name = pascal(&p.name);
            let (t, read) = match p.kind {
                Kind::Bool => ("bool", "asBool(V)"),
                Kind::Int => ("int64", "int64(asNumber(V))"),
                Kind::Float => ("float64", "asNumber(V)"),
                Kind::Text | Kind::Component => ("string", "asText(V)"),
            };
            let get = read.replace('V', &format!("c.Get(\"{}\")", p.name));
            // O nome do componente na tela já é o campo Name do Component.
            if name == "Name" {
                continue;
            }
            s.push_str(&format!("\nfunc (c {ty}) {name}() {t} {{ return {get} }}\n"));
            if !p.read_only {
                s.push_str(&format!("func (c {ty}) Set{name}(v {t}) {{ c.Set(\"{}\", v) }}\n", p.name));
            }
        }
        for mth in m.methods(class).iter().filter(|mth| !matches!(pascal(&mth.name).as_str(), "Show" | "Close" | "Focus" | "Get" | "Set")) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{a} {}", go_type(*k))).collect();
            let list = format!("[]any{{{}}}", mth.arg_values("{}.ComponentID()").join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", \"{k}\": {v}")).collect();
            let request = format!("(c.App, \"{}\", aylis.M{{\"target\": c.ID{fields}}})", mth.command());
            let (ret, body) = match mth.returns {
                Some(Kind::Bool) => (" bool", format!("return asBool(askCmd{request})")),
                Some(Kind::Int) => (" int64", format!("return int64(asNumber(askCmd{request}))")),
                Some(Kind::Float) => (" float64", format!("return asNumber(askCmd{request})")),
                Some(Kind::Text | Kind::Component) => (" string", format!("return asText(askCmd{request})")),
                None => ("", format!("c.App.Send(\"{}\", aylis.M{{\"target\": c.ID{fields}}})", mth.command())),
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
            s.push_str(&format!("\n// Find{class} é o {class} deste form com o nome (como o FindComponent).\nfunc (f *{ty}) Find{class}(name string) aylis{class} {{\n\tswitch name {{\n"));
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

fn cpp(m: &Model) -> Out {
    let s = format!("// {}\n\n#pragma once\n\n#include <memory>\n#include <optional>\n#include <string>\n\n#include \"../aylis.hpp\"\n\nnamespace aylis_components {{\n", HEADER.replace('\n', "\n// "));
    let mut s = Out::from(s);
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
            let (ty, read) = match p.kind {
                Kind::Bool => ("bool", "V.boolean()"),
                Kind::Int => ("long long", "static_cast<long long>(V.number())"),
                Kind::Float => ("double", "V.number()"),
                Kind::Text | Kind::Component => ("std::string", "V.str()"),
            };
            let get = read.replace('V', &format!("get_prop_(\"{}\")", p.name));
            s.push_str(&format!("    {ty} {name}() const {{ return {get}; }}\n"));
            if !p.read_only {
                let arg = if p.kind == Kind::Text { "const std::string& v" } else { &format!("{ty} v") };
                s.push_str(&format!("    void set_{}({arg}) const {{ set_prop_(\"{}\", v); }}\n", p.name, p.name));
            }
        }
        for mth in m.methods(class) {
            let args = mth.arg_names();
            let params: Vec<String> = args.iter().zip(&mth.args).map(|(a, k)| format!("{} {a}", cpp_type(*k))).collect();
            let list = format!("aylis::Json::array({{{}}})", mth.arg_values("{}.id()").join(", "));
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", {{\"{k}\", {v}}}")).collect();
            let request = format!("(\"{}\", {{{{\"target\", aylis::Json(id_)}}{fields}}})", mth.command());
            let (ret, body) = match mth.returns {
                Some(Kind::Bool) => ("bool", format!("return ask_{request}.boolean();")),
                Some(Kind::Int) => ("long long", format!("return static_cast<long long>(ask_{request}.number());")),
                Some(Kind::Float) => ("double", format!("return ask_{request}.number();")),
                Some(Kind::Text | Kind::Component) => ("std::string", format!("return ask_{request}.str();")),
                None => ("void", format!("tell_{request};")),
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
            s.push_str(&format!("    // O {class} deste form com o nome (como o FindComponent; vazio se não há).\n    {class} find_{sn}(const std::string& name) const {{\n", sn = snake(class)));
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
// Free Pascal (propriedades, como no Delphi)

fn pascal_unit(m: &Model) -> Out {
    let s = String::from(
        "{ Os componentes dos forms com as propriedades tipadas e a classe base de cada form (com os\n  \
         componentes como campos published, montados quando a tela abre o form). A classe de cada\n  \
         form, com os handlers e o estado, fica na unit dele.\n  \
         Gerado pela IDE do Aylis a cada \"Gerar código\": não edite. }\n\
         unit ayliscomponents;\n\n{$mode objfpc}{$H+}\n\ninterface\n\nuses\n  fpjson, aylis;\n\ntype\n",
    );
    let mut s = Out::from(s);
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let t = format!("T{class}");
        // O Form é a base das classes dos forms (o Show e o Close já vêm do TAylisForm).
        let base = if class == "Form" { "TAylisForm" } else { "TAylisComponent" };
        s.push_str(&format!("  {{ Um {class} de um form. }}\n  {t} = class({base})\n  private\n"));
        for p in props {
            let name = pascal(&p.name);
            let ty = pascal_type(p.kind);
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
            let ty = pascal_type(p.kind);
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
                "    {{ O {class} deste form com o nome, como o FindComponent (nil se não há). }}\n    function Find{class}(const Nome: string): T{class};\n    \
                 {{ O {class} que disparou o evento (nil se não é um {class} deste form). }}\n    function SenderAs{class}(Sender: TAylisComponent): T{class}; overload;\n    \
                 function SenderAs{class}(Event: TJSONObject): T{class}; overload;\n"
            ));
        }
        s.push_str("  end;\n\n");
    }
    s.end();
    s.push_str("implementation\n\nfunction AsText(Data: TJSONData): string;\nbegin\n  if (Data = nil) or (Data.JSONType = jtNull) then\n    Result := ''\n  else\n    Result := Data.AsString;\nend;\n\n");
    s.begin(Part::Class);
    for (class, props) in &m.classes {
        s.item(class);
        let t = format!("T{class}");
        for p in props {
            let name = pascal(&p.name);
            let ty = pascal_type(p.kind);
            let (read, make) = match p.kind {
                Kind::Bool => ("GetProp('@@').AsBoolean", "TJSONBoolean.Create(Value)"),
                Kind::Int => ("GetProp('@@').AsInt64", "TJSONInt64Number.Create(Value)"),
                Kind::Float => ("GetProp('@@').AsFloat", "TJSONFloatNumber.Create(Value)"),
                Kind::Text | Kind::Component => ("AsText(GetProp('@@'))", "TJSONString.Create(Value)"),
            };
            s.push_str(&format!("function {t}.Get{name}: {ty};\nbegin\n  Result := {};\nend;\n\n", read.replace("@@", &p.name)));
            if !p.read_only {
                s.push_str(&format!("procedure {t}.Set{name}(Value: {ty});\nbegin\n  SetProp('{}', {make});\nend;\n\n", p.name));
            }
        }
        for mth in pascal_methods(m, class) {
            let args = mth.arg_names().iter().zip(&mth.args).map(|(a, k)| if *k == Kind::Component { format!("{}.Id", pascal_arg(a)) } else { pascal_arg(a) }).collect::<Vec<_>>();
            let list = if args.is_empty() { "TJSONArray.Create".to_string() } else { format!("TJSONArray.Create([{}])", args.join(", ")) };
            let names = mth.arg_names();
            let fields: String = mth.fields(&list).iter().map(|(k, v)| format!(", '{k}', {}", if names.contains(v) { pascal_arg(v) } else { v.replace('"', "'") })).collect();
            let request = format!("('{}', TJSONObject.Create(['target', Id{fields}]))", mth.command());
            let body = match mth.returns {
                Some(Kind::Bool) => format!("Result := Aylis.Call{request}.AsBoolean;"),
                Some(Kind::Int) => format!("Result := Aylis.Call{request}.AsInt64;"),
                Some(Kind::Float) => format!("Result := Aylis.Call{request}.AsFloat;"),
                Some(Kind::Text | Kind::Component) => format!("Result := AsText(Aylis.Call{request});"),
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
    s.push_str("end.\n");
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

fn pascal_type(kind: Kind) -> &'static str {
    match kind {
        Kind::Bool => "Boolean",
        Kind::Int => "Int64",
        Kind::Float => "Double",
        Kind::Text => "string",
        Kind::Component => "TAylisComponent",
    }
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
