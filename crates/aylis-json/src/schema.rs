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
pub fn class_info(ui: &Ui, class: &str) -> Option<Value> {
    let entry = ui.registry().iter().find(|c| c.name.eq_ignore_ascii_case(class))?;
    let instance = (entry.create)();
    let mut properties = Vec::new();
    let mut events = Vec::new();
    for info in prop_infos_of(instance.as_ref()) {
        if info.kind == PropKind::Event {
            events.push(json!(info.name));
            continue;
        }
        let mut p = Map::new();
        p.insert("name".into(), json!(info.name));
        p.insert("type".into(), json!(kind_label(info.kind)));
        p.insert("category".into(), json!(info.category));
        if let PropKind::Enum(names) | PropKind::Set(names) = info.kind {
            p.insert("values".into(), json!(names));
        }
        if let Some(example) = kind_schema(info.kind).get("examples") {
            p.insert("examples".into(), example.clone());
        }
        if info.read_only {
            p.insert("read_only".into(), json!(true));
        }
        if info.runtime {
            p.insert("runtime_only".into(), json!(true));
        }
        properties.push(Value::Object(p));
    }
    let methods: Vec<Value> = instance
        .methods()
        .iter()
        .map(|m| {
            let args: Vec<Value> = m.args.iter().enumerate().map(|(i, k)| json!({ "name": m.params.get(i).copied().unwrap_or(""), "type": kind_label(*k) })).collect();
            json!({ "name": m.name, "args": args, "returns": m.returns.map(kind_label) })
        })
        .collect();
    Some(json!({
        "class": entry.name,
        "category": entry.category,
        "properties": properties,
        "events": events,
        "methods": methods,
        "note": "no JSON: margins/padding como número, [h, v] ou [l, t, r, b]; \"margins.left\": 4 muda um lado; cores como \"#RRGGBB\" ou token do tema",
    }))
}

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
        PropKind::String => json!({ "type": "string" }),
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
/// `int`, `float`, `string`, `color`, `enum`, `component`, `bounds` e `set` (`anchors`), estes
/// dois como texto no código; `:ro` quando é só leitura). Os eventos e os outros conjuntos
/// ficam de fora. Uma classe por linha.
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
                PropKind::Bool => "bool",
                PropKind::Int => "int",
                PropKind::Float | PropKind::Angle | PropKind::Duration => "float",
                PropKind::String | PropKind::Shadow | PropKind::Easing => "string",
                PropKind::Color => "color",
                PropKind::Enum(_) => "enum",
                PropKind::Component => "component",
                // Os eventos que perguntam: `"question:campos"` (a IDE gera a assinatura certa).
                PropKind::Event => match aylis_core::question::question_fields(info.name) {
                    Some(fields) => {
                        props.insert(info.name.to_string(), Value::String(format!("question:{}", fields.join(","))));
                        continue;
                    }
                    // Os que se juntam enquanto o anterior não terminou no backend.
                    None if aylis_core::question::COALESCED_EVENTS.contains(&info.name) => {
                        props.insert(info.name.to_string(), Value::String("coalesce".into()));
                        continue;
                    }
                    None => continue,
                },
                // Margens e âncoras no acesso tipado como texto ("12, 4, 12, 0", "left,top"):
                // o `set_prop` aceita (relatório do RTSPlayer, 10/11: faltavam `margins`,
                // `padding` e `anchors`). Os outros conjuntos ficam de fora.
                PropKind::Bounds => "bounds",
                PropKind::Set(_) if info.name == "anchors" => "set",
                PropKind::Set(_) => continue,
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
                        Some(name) => format!("{name}:{}", method_kind(*k)),
                        None => method_kind(*k).to_string(),
                    })
                    .collect();
                (format!("{}({})", m.name, args.join(",")), format!("invoke:{}", m.returns.map_or("void", method_kind)))
            })
            .collect();
        // Quem recebe foco: `set_focus()` (o `SetFocus`).
        if instance.control().is_some_and(|cb| cb.can_focus()) {
            methods.push(("set_focus()".into(), "focus:void".into()));
        }
        let commands: &[(&str, &str)] = match class.name {
            // O foco do form: quem tem (o `ActiveControl`) e o próximo/anterior do Tab.
            "Form" => &[("show()", "show:void"), ("close()", "close:bool"), ("focused()", "focused:string"), ("focus_next()", "focus_next:void"), ("focus_prev()", "focus_prev:void")],
            "OpenDialog" | "SaveDialog" => &[("execute()", "execute:void")],
            "VideoPlayer" | "MediaPlayer" => &[
                ("play()", "media:void"),
                ("pause()", "media:void"),
                ("stop()", "media:void"),
                ("seek(seconds:float)", "media:void"),
                ("position()", "media:float"),
                ("duration()", "media:float"),
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
    format!("{{\n{}\n}}\n", lines.join(",\n"))
}

/// O tipo de um argumento ou resultado de método no catálogo.
fn method_kind(kind: PropKind) -> &'static str {
    match kind {
        PropKind::Bool => "bool",
        PropKind::Int => "int",
        PropKind::Float | PropKind::Angle | PropKind::Duration => "float",
        // O componente (cada linguagem passa o objeto dela; chega pelo id ou pelo caminho).
        PropKind::Component => "component",
        _ => "string",
    }
}
