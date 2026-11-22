//! JSON Schema gerado a partir do registro de classes: dá autocompletar e validação de
//! layouts no VS Code (e em qualquer editor com suporte a JSON Schema).
//!
//! Para associar, ponha no topo do arquivo de layout:
//! `"$schema": "caminho/para/aylis.schema.json"`.

use aylis_core::{PropKind, Ui, prop_infos_of};
use serde_json::{Map, Value, json};

/// Gera o schema de documentos de layout com todas as classes registradas no `Ui`.
pub fn schema(ui: &Ui) -> Value {
    let mut defs = Map::new();
    let mut classes: Vec<_> = ui.registry().iter().collect();
    classes.sort_by_key(|c| c.name);
    let mut variants = Vec::new();

    for class in &classes {
        let instance = (class.create)();
        let mut props = Map::new();
        props.insert("class".into(), json!({ "const": class.name }));
        props.insert(
            "name".into(),
            json!({ "type": "string", "pattern": "^[A-Za-z_][A-Za-z0-9_]*$", "description": "Nome do componente (único no form)" }),
        );
        for info in prop_infos_of(instance.as_ref()) {
            let mut schema = kind_schema(info.kind);
            if let Value::Object(map) = &mut schema {
                map.insert("description".into(), Value::String(format!("{} · {}", info.category, kind_label(info.kind))));
            }
            props.insert(info.name.to_string(), schema);
        }
        props.insert("children".into(), json!({ "type": "array", "items": { "$ref": "#/$defs/node" } }));
        defs.insert(
            class.name.to_string(),
            json!({
                "type": "object",
                "description": format!("{} (paleta: {})", class.name, class.category),
                "properties": props,
                "patternProperties": { "^(margins|padding)\\.(left|top|right|bottom)$": { "type": "number" } },
                "required": ["class"],
                "additionalProperties": false
            }),
        );
        variants.push(json!({ "$ref": format!("#/$defs/{}", class.name) }));
    }

    let class_names: Vec<&str> = classes.iter().map(|c| c.name).collect();
    defs.insert(
        "node".into(),
        json!({
            "type": "object",
            "properties": { "class": { "enum": class_names } },
            "required": ["class"],
            "anyOf": variants
        }),
    );

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Layout do Aylis",
        "type": "object",
        "properties": {
            "$schema": { "type": "string" },
            "aylis": { "const": crate::FORMAT_VERSION, "description": "Versão do formato" },
            "root": { "$ref": "#/$defs/node" }
        },
        "required": ["root"],
        "additionalProperties": false,
        "$defs": defs
    })
}

/// Tudo o que uma classe aceita no layout e no código: cada propriedade com o tipo, a categoria,
/// os valores (de um enum ou conjunto) e como escrever no JSON; os eventos; e os métodos do
/// `invoke`. `None` se a classe não existe. É o `class_info` do MCP: sem ele, o agente tentava
/// às cegas (`trimming: "ellipsis"`, `radius` num `Label`, `on_double_click`; relatório do
/// RTSPlayer, 10/11).
///
/// Enxuto para o agente (15/11): cada propriedade numa entrada `nome: "tipo"` (o enum com os
/// valores, `a | b | c`), e as propriedades e os eventos comuns a todo controle (posição,
/// `align`, fonte, cores, `on_click`...) só pelo nome, em `common`: iguais em todas as classes,
/// eram dois terços de cada resposta. `inherited` traz os comuns com o tipo, como os da classe.
pub fn class_info(ui: &Ui, class: &str, inherited: bool) -> Option<Value> {
    let entry = ui.registry().iter().find(|c| c.name.eq_ignore_ascii_case(class))?;
    let instance = (entry.create)();
    let (common_props, common_events) = if inherited || instance.control().is_none() { (Vec::new(), Vec::new()) } else { common_members(ui) };
    let mut properties = Map::new();
    let mut events = Vec::new();
    for info in prop_infos_of(instance.as_ref()) {
        if info.kind == PropKind::Event {
            if !common_events.contains(&info.name) {
                events.push(json!(info.name));
            }
            continue;
        }
        if common_props.contains(&info.name) {
            continue;
        }
        let mut text = match TEXT_FORMATS.iter().find(|(name, _)| *name == info.name).filter(|_| !matches!(info.kind, PropKind::Record(_) | PropKind::RecordList(_))) {
            Some((_, hint)) => format!("texto: {hint}"),
            None => match info.kind {
                // A lista de registros: os campos de cada objeto.
                PropKind::RecordList(r) => format!("lista de {} [{{{}}}]", r.name, r.fields.iter().map(|(f, k)| format!("{f}: {}", kind_label(*k))).collect::<Vec<_>>().join(", ")),
                PropKind::Record(r) => format!("{} {{{}}}", r.name, r.fields.iter().map(|(f, k)| format!("{f}: {}", kind_label(*k))).collect::<Vec<_>>().join(", ")),
                PropKind::Enum(names) => names.join(" | "),
                PropKind::Set(names) => format!("lista de: {}", names.join(", ")),
                kind => kind_label(kind).to_string(),
            },
        };
        if info.read_only {
            text.push_str(" (só leitura)");
        }
        if info.runtime {
            text.push_str(" (só em execução)");
        }
        properties.insert(info.name.to_string(), json!(text));
    }
    let methods: Vec<Value> = instance
        .methods()
        .iter()
        .map(|m| {
            let args: Vec<String> = m.args.iter().enumerate().map(|(i, k)| format!("{}: {}", m.params.get(i).copied().unwrap_or("_"), kind_label(*k))).collect();
            let returns = m.returns.map(|k| format!(" -> {}", kind_label(k))).unwrap_or_default();
            json!(format!("{}({}){returns}", m.name, args.join(", ")))
        })
        .collect();
    let mut out = Map::new();
    out.insert("class".into(), json!(entry.name));
    out.insert("category".into(), json!(entry.category));
    out.insert("properties".into(), Value::Object(properties));
    out.insert("events".into(), json!(events));
    if !methods.is_empty() {
        out.insert("methods".into(), json!(methods));
    }
    if !common_props.is_empty() {
        out.insert("common".into(), json!({ "properties": common_props, "events": common_events, "note": "os de todo controle, só os nomes: inherited true traz os tipos" }));
    }
    out.insert(
        "note".into(),
        json!("margins/padding: número, [h, v] ou [l, t, r, b] (\"margins.left\": 4 muda um lado); cor: \"#RRGGBB\", token do tema (primary, text_muted) ou com opacidade (\"primary/15\"); aparência vazia (null) usa o estilo do tema, que pode ter borda: 0 tira"),
    );
    Some(Value::Object(out))
}

/// As propriedades e os eventos que todas as classes visuais têm (os do `ControlBase`), na ordem
/// em que aparecem: a interseção do registro, e não uma lista escrita à mão, para acompanhar o
/// que a base ganhar.
fn common_members(ui: &Ui) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut common: Option<Vec<(&'static str, bool)>> = None;
    for class in ui.registry().iter() {
        let instance = (class.create)();
        if instance.control().is_none() {
            continue;
        }
        let names: Vec<(&'static str, bool)> = prop_infos_of(instance.as_ref()).iter().map(|i| (i.name, i.kind == PropKind::Event)).collect();
        common = Some(match common {
            None => names,
            Some(before) => before.into_iter().filter(|n| names.contains(n)).collect(),
        });
    }
    let all = common.unwrap_or_default();
    (all.iter().filter(|(_, e)| !e).map(|(n, _)| *n).collect(), all.iter().filter(|(_, e)| *e).map(|(n, _)| *n).collect())
}

/// Como escrever as propriedades de texto com estrutura (o `class_info` mostra): o tipo diz só
/// "texto" (relatório do InsightDB, 11/11: `items` com quebra de linha sumia sem erro).
const TEXT_FORMATS: &[(&str, &str)] = &[
    ("items", "os itens separados por ; (ou um por linha): \"Ativo;Inativo;Pendente\""),
    ("values", "os códigos dos itens, na mesma ordem, separados por ;"),
    ("labels", "os rótulos separados por ;: \"Jan;Fev;Mar\""),
    ("sizes", "os tamanhos em ordem, separados por ;: \"80;120;80\""),
    ("selected", "os índices separados por ;"),
    ("colors", "as cores em ordem, separadas por ;: \"#2563EB;#16A34A\""),
];

fn kind_schema(kind: PropKind) -> Value {
    match kind {
        PropKind::Bool => json!({ "type": "boolean" }),
        PropKind::Int => json!({ "type": "integer" }),
        PropKind::Float => json!({ "type": ["number", "null"] }),
        PropKind::Angle => json!({ "type": "number", "examples": [0, 45, 90] }),
        PropKind::Duration => json!({ "type": "number", "minimum": 0, "examples": [200, 500] }),
        PropKind::Shadow => json!({ "type": ["string", "null"], "examples": ["0 4 12 #00000040", "0 2 6 1 #2563EB66"] }),
        PropKind::Easing => json!({
            "type": "string",
            "examples": ["out_quad", "ease-in-out", "cubic-bezier(0.2, 0, 0, 1)"]
        }),
        PropKind::String | PropKind::List | PropKind::NumberList | PropKind::ColorList | PropKind::PointList | PropKind::StyleName | PropKind::ThemeName | PropKind::FormName => json!({ "type": "string" }),
        PropKind::Record(_) => json!({ "type": ["object", "string"] }),
        PropKind::RecordList(_) => json!({ "type": ["array", "string"] }),
        PropKind::Date => json!({ "type": "string", "examples": ["2026-10-24"] }),
        PropKind::Time => json!({ "type": "string", "examples": ["14:30"] }),
        PropKind::DateTime => json!({ "type": "string", "examples": ["2026-11-12T14:30"] }),
        PropKind::Color => json!({ "type": ["string", "null"], "examples": ["#2563EB", "primary", "text_muted"] }),
        PropKind::Enum(names) => json!({ "enum": names }),
        PropKind::Set(names) => json!({
            "anyOf": [
                { "type": "array", "items": { "enum": names }, "uniqueItems": true },
                { "type": "string" }
            ]
        }),
        PropKind::Bounds => json!({
            "anyOf": [
                { "type": "number" },
                { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 },
                { "type": "array", "items": { "type": "number" }, "minItems": 4, "maxItems": 4 },
                {
                    "type": "object",
                    "properties": {
                        "left": { "type": "number" }, "top": { "type": "number" },
                        "right": { "type": "number" }, "bottom": { "type": "number" }
                    },
                    "additionalProperties": false
                }
            ]
        }),
        PropKind::Component => json!({ "type": ["string", "null"] }),
        PropKind::Event => json!({ "type": ["string", "null"] }),
    }
}

fn kind_label(kind: PropKind) -> &'static str {
    match kind {
        PropKind::Bool => "true/false",
        PropKind::Int => "inteiro",
        PropKind::Float => "número",
        PropKind::Angle => "ângulo em graus",
        PropKind::Duration => "milissegundos",
        PropKind::Shadow => "sombra: \"dx dy desfoque [espalhamento] #cor\"",
        PropKind::Easing => "curva: nome ou \"cubic-bezier(x1, y1, x2, y2)\"",
        PropKind::String => "texto",
        PropKind::Date => "data em ISO (\"2026-10-24\")",
        PropKind::Time => "hora (\"14:30\")",
        PropKind::DateTime => "dia e hora em ISO (\"2026-11-12T14:30\")",
        PropKind::List => "textos separados por ;",
        PropKind::NumberList => "números separados por ;",
        PropKind::ColorList => "cores separadas por ;",
        PropKind::PointList => "pontos x,y separados por espaço",
        PropKind::Record(_) => "registro em JSON",
        PropKind::RecordList(_) => "lista de registros em JSON",
        PropKind::StyleName => "variante do tema (\"primary\", \"outline\"...)",
        PropKind::ThemeName => "nome de um tema",
        PropKind::FormName => "nome de um form do projeto",
        PropKind::Color => "cor: \"#RRGGBB\" ou token do tema",
        PropKind::Enum(_) => "um dos valores",
        PropKind::Set(_) => "lista de valores",
        PropKind::Bounds => "número, [h, v], [l, t, r, b] ou objeto",
        PropKind::Component => "nome de outro componente",
        PropKind::Event => "nome do handler",
    }
}

/// O catálogo das classes para o acesso tipado nas linguagens (`resources/components.json`,
/// lido pela geração de código): cada classe com as propriedades e o tipo delas (`bool`,
/// `int`, `float`, `string`, `color`, `component`, `bounds`, `enum(a|b|c)` e `set(a|b|c)`,
/// estes dois com os valores, que viram um tipo em cada linguagem; `:ro` quando é só
/// leitura). Os eventos comuns ficam de fora. Uma classe por linha.
///
/// Os métodos entram com os parênteses e os argumentos na chave (`nome:tipo`, ou só o tipo
/// quando o método não dá o nome) e, no valor, o
/// comando do protocolo que os chama e o tipo do resultado: `"resize(int,int)":
/// "invoke:bool"` ([`Component::methods`](aylis_core::Component::methods)),
/// `"execute()": "execute:void"`, `"seek(float)": "media:void"`.
pub fn catalog(ui: &Ui) -> String {
    let mut classes: Vec<_> = ui.registry().iter().collect();
    classes.sort_by_key(|c| c.name);
    let mut lines = Vec::new();
    for class in classes {
        let instance = (class.create)();
        let mut props = Map::new();
        for info in prop_infos_of(instance.as_ref()) {
            let kind = match info.kind {
                // Os eventos que perguntam: `"question:campos"` (a IDE gera a assinatura certa).
                PropKind::Event => match aylis_core::question::question_fields(info.name) {
                    Some(fields) => {
                        props.insert(info.name.to_string(), Value::String(format!("question:{}", fields.join(","))));
                        continue;
                    }
                    // O que o evento leva, quando declarado (`"event:float"`, `"event:none"`):
                    // o handler gerado recebe o valor nesse tipo.
                    None if info.event_arg != aylis_core::EventArg::Unknown => {
                        let arg = match info.event_arg {
                            aylis_core::EventArg::Unknown | aylis_core::EventArg::None => "none".to_string(),
                            aylis_core::EventArg::Pointer => "pointer".into(),
                            aylis_core::EventArg::Wheel => "wheel".into(),
                            aylis_core::EventArg::Gesture => "gesture".into(),
                            aylis_core::EventArg::Swipe => "swipe".into(),
                            aylis_core::EventArg::Key => "key".into(),
                            aylis_core::EventArg::DateRange => "daterange".into(),
                            aylis_core::EventArg::Value(kind) => catalog_kind(kind, info.rust_type),
                        };
                        props.insert(info.name.to_string(), Value::String(format!("event:{arg}")));
                        continue;
                    }
                    // Os que se juntam enquanto o anterior não terminou no backend.
                    None if aylis_core::question::COALESCED_EVENTS.contains(&info.name) => {
                        props.insert(info.name.to_string(), Value::String("coalesce".into()));
                        continue;
                    }
                    None => continue,
                },
                other => catalog_kind(other, info.rust_type),
            };
            let kind = if info.read_only { format!("{kind}:ro") } else { kind.to_string() };
            props.insert(info.name.to_string(), Value::String(kind));
        }
        // Os métodos: os do `invoke` e os comandos do protocolo de cada classe.
        let mut methods: Vec<(String, String)> = instance
            .methods()
            .iter()
            .map(|m| {
                // Com o nome do argumento, quando o método dá: `save_to(path:string)`.
                let args: Vec<String> = m
                    .args
                    .iter()
                    .enumerate()
                    .map(|(i, k)| match m.params.get(i) {
                        Some(name) => format!("{name}:{}", catalog_kind(*k, m.arg_types.get(i).copied().unwrap_or_default())),
                        None => catalog_kind(*k, m.arg_types.get(i).copied().unwrap_or_default()),
                    })
                    .collect();
                (format!("{}({})", m.name, args.join(",")), format!("invoke:{}", m.returns.map_or("void".to_string(), |k| catalog_kind(k, m.returns_type))))
            })
            .collect();
        // Quem recebe foco: `set_focus()`.
        if instance.control().is_some_and(|cb| cb.can_focus()) {
            methods.push(("set_focus()".into(), "focus:void".into()));
        }
        let commands: &[(&str, &str)] = match class.name {
            // O foco do form: quem tem e o próximo/anterior do Tab.
            "Form" => &[("show()", "show:void"), ("close()", "close:bool"), ("focused()", "focused:string"), ("focus_next()", "focus_next:void"), ("focus_prev()", "focus_prev:void")],
            "OpenDialog" | "SaveDialog" => &[("execute()", "execute:void")],
            "VideoPlayer" | "MediaPlayer" => &[
                ("play()", "media:void"),
                ("pause()", "media:void"),
                ("stop()", "media:void"),
                ("seek(seconds:float)", "media:void"),
                ("position()", "media:float"),
                ("duration()", "media:float"),
                ("playing()", "media:bool"),
                ("ready()", "media:bool"),
            ],
            _ => &[],
        };
        methods.extend(commands.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        for (key, value) in methods {
            // Um método com o nome de uma propriedade ficaria ambíguo nas linguagens.
            let name = key.split('(').next().unwrap_or_default();
            if !props.contains_key(name) {
                props.insert(key, Value::String(value));
            }
        }
        lines.push(format!("  {}: {}", Value::String(class.name.to_string()), Value::Object(props)));
    }
    // Os registros (`record(Monitor)`): os campos de cada um, numa entrada `$records` (que não
    // é classe).
    let mut records = Map::new();
    for class in ui.registry().iter() {
        let instance = (class.create)();
        for info in prop_infos_of(instance.as_ref()) {
            collect_record(info.kind, &mut records);
        }
        for m in instance.methods() {
            for k in m.args.iter().copied().chain(m.returns) {
                collect_record(k, &mut records);
            }
        }
    }
    if !records.is_empty() {
        lines.insert(0, format!("  \"$records\": {}", Value::Object(records)));
    }
    // Os nomes que o `style` e o `theme` aceitam sem o projeto: as variantes de cada classe (as
    // registradas e as dos temas do Aylis) e os temas do Aylis. O gerador junta os do projeto.
    let builtin: Vec<&aylis_core::Theme> = [aylis_core::themes::builtin("Aylis Light"), aylis_core::themes::builtin("Aylis Dark")].into_iter().flatten().chain(aylis_core::themes::all()).collect();
    let mut styles = Map::new();
    let mut names: Vec<&str> = ui.registry().iter().map(|c| c.name).collect();
    names.sort();
    for class in names {
        let mut found: std::collections::BTreeSet<String> = ui.variants_of(class).into_iter().map(|v| v.name).collect();
        for theme in &builtin {
            found.extend(theme.variants_of(class));
        }
        found.retain(|v| !v.is_empty());
        if !found.is_empty() {
            styles.insert(class.to_string(), Value::Array(found.into_iter().map(Value::String).collect()));
        }
    }
    lines.insert(0, format!("  \"$styles\": {}", Value::Object(styles)));
    let themes: Vec<Value> = builtin.iter().map(|t| Value::String(t.name.clone())).collect();
    lines.insert(0, format!("  \"$themes\": {}", Value::Array(themes)));
    format!("{{\n{}\n}}\n", lines.join(",\n"))
}

/// O registro de `kind` (e os que ele tem dentro) com os campos, em `out`.
fn collect_record(kind: PropKind, out: &mut Map<String, Value>) {
    let (PropKind::Record(r) | PropKind::RecordList(r)) = kind else { return };
    if out.contains_key(r.name) {
        return;
    }
    let fields: Map<String, Value> = r.fields.iter().map(|(name, k)| (name.to_string(), Value::String(catalog_kind(*k, "")))).collect();
    out.insert(r.name.to_string(), Value::Object(fields));
    for (_, k) in r.fields {
        collect_record(*k, out);
    }
}

/// O tipo de uma propriedade, de um argumento ou do resultado de um método no catálogo. O
/// enum e o conjunto levam o nome do tipo Rust e os valores (`enum(WindowState:normal|maximized)`;
/// sem o tipo, `enum(:a|b)`), para cada linguagem ter um tipo com eles em vez de texto.
fn catalog_kind(kind: PropKind, rust_type: &str) -> String {
    // `crate::Align`, `Option<x::Y>`: só o último nome.
    let ty: String = rust_type.chars().filter(|c| !c.is_whitespace()).collect();
    let ty = ty.rsplit("::").next().unwrap_or_default();
    let ty = if ty.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') { ty } else { "" };
    match kind {
        PropKind::Bool => "bool".into(),
        PropKind::Int => "int".into(),
        PropKind::Float | PropKind::Angle | PropKind::Duration => "float".into(),
        PropKind::String | PropKind::Event => "string".into(),
        PropKind::Shadow => "shadow".into(),
        PropKind::Easing => "easing".into(),
        PropKind::Date => "date".into(),
        PropKind::Time => "time".into(),
        PropKind::DateTime => "datetime".into(),
        PropKind::List => "list".into(),
        PropKind::NumberList => "numbers".into(),
        PropKind::ColorList => "colors".into(),
        PropKind::PointList => "points".into(),
        PropKind::Record(r) => format!("record({})", r.name),
        PropKind::RecordList(r) => format!("records({})", r.name),
        // Os nomes do projeto: cada linguagem ganha um enum com os do Aylis e os do projeto.
        PropKind::StyleName => "style".into(),
        PropKind::ThemeName => "theme".into(),
        PropKind::FormName => "form".into(),
        PropKind::Color => "color".into(),
        PropKind::Bounds => "bounds".into(),
        // O componente (cada linguagem passa o objeto dela; chega pelo id ou pelo caminho).
        PropKind::Component => "component".into(),
        PropKind::Enum(names) => format!("enum({ty}:{})", names.join("|")),
        PropKind::Set(names) => format!("set({ty}:{})", names.join("|")),
    }
}

/// O teste que confere que os enums do [`catalog`] existem no `aylis::prelude` com o nome do
/// catálogo (`crates/aylis/tests/catalog_enums.rs`): o acesso tipado em Rust usa o tipo do
/// framework, e um enum que não está lá quebraria o código gerado. Regravado junto com o catálogo.
pub fn catalog_enums_test(ui: &Ui) -> String {
    let mut names = std::collections::BTreeSet::new();
    for class in ui.registry().iter() {
        let instance = (class.create)();
        let mut add = |kind: PropKind, rust_type: &str| {
            if matches!(kind, PropKind::Enum(_) | PropKind::Set(_)) {
                let ty: String = rust_type.chars().filter(|c| !c.is_whitespace()).collect();
                let ty = ty.rsplit("::").next().unwrap_or_default().to_string();
                if !ty.is_empty() && ty.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    names.insert(ty);
                }
            }
        };
        for info in prop_infos_of(instance.as_ref()) {
            add(info.kind, info.rust_type);
        }
        // Os enums dos argumentos e dos resultados dos métodos.
        for m in instance.methods() {
            for (i, k) in m.args.iter().enumerate() {
                add(*k, m.arg_types.get(i).copied().unwrap_or_default());
            }
            if let Some(k) = m.returns {
                add(k, m.returns_type);
            }
        }
    }
    let body: Vec<String> = names.iter().map(|n| format!("    let _: Option<aylis::prelude::{n}> = None;")).collect();
    format!(
        "//! Os enums do catálogo (`resources/components.json`) existem no `aylis::prelude` com o nome\n//! que o catálogo dá: o acesso tipado gerado em Rust usa o tipo do framework. Gerado pelo\n//! `aylis-tool update-catalog`.\n\n#[test]\nfn the_catalog_enums_are_in_the_prelude() {{\n{}\n}}\n",
        body.join("\n")
    )
}
