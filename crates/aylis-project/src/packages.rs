//! Pacotes de componente compostos (`.aylispkg`, tipo A): um form com propriedades expostas,
//! usado no form de quem instalou como um `Frame` com `form = "pkg:Nome"` (sem compilar,
//! em todas as linguagens e plataformas).
//!
//! No projeto, cada pacote usado fica em `packages/<Nome>/` (uma versão por projeto): o
//! `package.json` ([`Manifest`]) e o form dele. O valor de cada propriedade exposta vai no
//! `values` do frame (`{"valor": 42}`) e é aplicado no componente de dentro que ela aponta
//! (`"target": "pbNivel.value"`). Instalar, assinar e conferir ficam no `aylis-tools` (a IDE).

use std::collections::BTreeMap;

use aylis_core::{ComponentId, Ui};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Files;

/// O começo do `form` de um frame que mostra um pacote (`pkg:Termometro`).
pub const PREFIX: &str = "pkg:";

/// A pasta dos pacotes dentro do projeto.
pub const DIR: &str = "packages";

/// O `package.json` de um pacote.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Manifest {
    /// O nome do componente na paleta (também a pasta e o `pkg:Nome`).
    pub name: String,
    /// Versão semântica (`1.2.0`): a menor muda sem quebrar; a maior pode renomear ou tirar
    /// propriedades (veja `renamed` e `notes`).
    pub version: String,
    pub author: String,
    pub description: String,
    /// A aba da paleta (`Industrial`, `Elétrica`...).
    pub category: String,
    /// `composite` (o form, tipo A) ou `code` (um crate Rust, tipo B).
    pub kind: String,
    /// O arquivo do form dentro do pacote.
    pub form: String,
    /// Tipo B: o nome do crate Rust (o `[package] name` dele, em `crate/` no pacote).
    #[serde(rename = "crate", skip_serializing_if = "String::is_empty")]
    pub crate_name: String,
    /// Tipo B: a classe que o crate registra (o `register(ui)` dele) e que o frame mostra.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub component: String,
    pub properties: Vec<ExposedProp>,
    pub events: Vec<ExposedEvent>,
    /// Notas do autor por versão (a IDE mostra ao atualizar para uma versão maior).
    pub notes: BTreeMap<String, String>,
    /// Propriedades renomeadas numa versão maior: nome antigo → novo (vazio: removida).
    pub renamed: BTreeMap<String, String>,
}

/// Uma propriedade do pacote, ligada a uma propriedade de um componente de dentro.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExposedProp {
    pub name: String,
    /// `bool`, `number`, `text` ou `color` (o editor no inspector).
    #[serde(rename = "type")]
    pub kind: String,
    /// `componente.propriedade` dentro do form do pacote.
    pub target: String,
    pub default: Value,
    pub description: String,
}

/// Um evento do pacote: o de um componente de dentro, com outro nome.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExposedEvent {
    pub name: String,
    /// `componente.on_evento` dentro do form do pacote.
    pub target: String,
}

impl Manifest {
    pub fn parse(json: &str) -> Result<Self, String> {
        let m: Manifest = serde_json::from_str(json.trim_start_matches('\u{feff}')).map_err(|e| format!("package.json: {e}"))?;
        if m.name.trim().is_empty() || !m.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("package.json: o nome deve ser uma palavra (letras, números e _)".into());
        }
        if Version::parse(&m.version).is_none() {
            return Err(format!("package.json: versão '{}' fora do formato 1.2.3", m.version));
        }
        if m.is_code() {
            let crate_ok = !m.crate_name.is_empty() && m.crate_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
            if !crate_ok || !aylis_core::is_identifier(&m.component) {
                return Err("package.json: um pacote de código (kind = code) diz o crate (`crate`) e a classe (`component`)".into());
            }
        }
        Ok(m)
    }

    /// Tipo B: um crate Rust compilado no app (veja [`code_dependency`]).
    pub fn is_code(&self) -> bool {
        self.kind == "code"
    }

    /// O nome do crate como aparece no código Rust (`medidor-aylis` → `medidor_aylis`).
    pub fn crate_ident(&self) -> String {
        self.crate_name.replace('-', "_")
    }

    /// O arquivo do form (o padrão é `form.json`).
    pub fn form_file(&self) -> &str {
        if self.form.is_empty() { "form.json" } else { &self.form }
    }

    pub fn property(&self, name: &str) -> Option<&ExposedProp> {
        self.properties.iter().find(|p| p.name.eq_ignore_ascii_case(name))
    }

    pub fn event(&self, name: &str) -> Option<&ExposedEvent> {
        self.events.iter().find(|e| e.name.eq_ignore_ascii_case(name))
    }
}

/// Uma versão `maior.menor.correção`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u32, pub u32, pub u32);

impl Version {
    pub fn parse(text: &str) -> Option<Self> {
        let mut parts = text.trim().trim_start_matches('v').split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next().unwrap_or("0").parse().ok()?;
        let patch = parts.next().unwrap_or("0").split(['-', '+']).next()?.parse().ok()?;
        Some(Self(major, minor, patch))
    }

    /// Mesma versão maior: troca sem quebrar o form.
    pub fn compatible(self, other: Version) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

/// O caminho de um arquivo do pacote dentro do projeto.
pub fn path_in_project(name: &str, file: &str) -> String {
    format!("{DIR}/{name}/{file}")
}

/// O manifesto do pacote usado pelo projeto.
pub fn manifest(files: &Files, name: &str) -> Option<Manifest> {
    let text = files.read_text(&path_in_project(name, "package.json")).ok()?;
    Manifest::parse(&text).ok()
}

/// Os pacotes que o projeto usa (as pastas em `packages/` com `package.json`).
pub fn used(files: &Files) -> Vec<Manifest> {
    let mut names: Vec<String> = files
        .list(&format!("{DIR}/"))
        .into_iter()
        .filter_map(|p| p.strip_prefix(&format!("{DIR}/")).and_then(|r| r.split('/').next()).map(str::to_string))
        .collect();
    names.sort();
    names.dedup();
    names.iter().filter_map(|n| manifest(files, n)).collect()
}

/// O layout do form do pacote, com a raiz renomeada para `root_name` (cada uso do pacote
/// é um form à parte: dois termômetros no mesmo form não brigam pelo nome).
pub fn form_layout(files: &Files, name: &str, root_name: &str) -> Result<Value, String> {
    let m = manifest(files, name).ok_or_else(|| format!("o pacote '{name}' não está no projeto (packages/{name})"))?;
    let text = files.read_text(&path_in_project(name, m.form_file())).map_err(|e| e.to_string())?;
    let mut value: Value = serde_json::from_str(&text).map_err(|e| format!("pacote {name}: {e}"))?;
    value["root"]["name"] = Value::String(root_name.to_string());
    Ok(value)
}

/// O nome do form de dentro de um frame de pacote (único no `Ui`).
pub fn inner_name(ui: &Ui, frame: ComponentId, package: &str) -> String {
    let base = format!("{}_{package}", ui.name(frame).filter(|n| !n.is_empty()).unwrap_or("pkg"));
    let mut name = base.clone();
    let mut n = 2;
    while ui.find_path(&name).is_some() {
        name = format!("{base}{n}");
        n += 1;
    }
    name
}

/// Aplica as propriedades expostas (`values` do frame, com o padrão de cada uma) nos
/// componentes de dentro. Devolve os avisos (propriedade que não existe mais...).
pub fn apply_values(ui: &mut Ui, inner_root: ComponentId, manifest: &Manifest, values: &str) -> Vec<String> {
    let values: serde_json::Map<String, Value> = serde_json::from_str(values).unwrap_or_default();
    let mut warnings = Vec::new();
    for p in &manifest.properties {
        let value = values.iter().find(|(k, _)| k.eq_ignore_ascii_case(&p.name)).map(|(_, v)| v.clone()).unwrap_or_else(|| p.default.clone());
        if value.is_null() {
            continue;
        }
        // Sem componente (`"target": "value"`): a do próprio de dentro (o tipo B).
        let (component, prop) = p.target.rsplit_once('.').unwrap_or(("", p.target.as_str()));
        let found = if component.is_empty() { Some(inner_root) } else { ui.find_in(inner_root, component) };
        let Some(target) = found else {
            warnings.push(format!("{}: o componente '{component}' não existe no pacote", p.name));
            continue;
        };
        match aylis_json::prop_from_json(ui, target, prop, &value) {
            Ok(v) => {
                let _ = ui.set_prop(target, prop, v);
            }
            Err(e) => warnings.push(format!("{}: {e}", p.name)),
        }
    }
    for name in values.keys() {
        if manifest.property(name).is_none() && manifest.event(name).is_none() {
            warnings.push(format!("{name}: o pacote {} {} não tem esta propriedade", manifest.name, manifest.version));
        }
    }
    warnings
}

/// Liga os eventos expostos aos componentes de dentro: nos `values` do frame, o nome do
/// evento e o handler do form do usuário (`"on_alarme": "alarme_disparou"`). O evento de
/// dentro chama o handler como se tivesse disparado no `frame` (o `sender` é o frame, e o
/// nome é procurado no form do usuário, em qualquer linguagem). Devolve os avisos.
pub fn apply_events(ui: &mut Ui, inner_root: ComponentId, frame: ComponentId, manifest: &Manifest, values: &str) -> Vec<String> {
    let values: serde_json::Map<String, Value> = serde_json::from_str(values).unwrap_or_default();
    let mut warnings = Vec::new();
    for e in &manifest.events {
        let Some(handler) = values.iter().find(|(k, _)| k.eq_ignore_ascii_case(&e.name)).and_then(|(_, v)| v.as_str()).map(str::trim).filter(|h| !h.is_empty()) else {
            continue;
        };
        let (component, event) = e.target.rsplit_once('.').unwrap_or(("", e.target.as_str()));
        let found = if component.is_empty() { Some(inner_root) } else { ui.find_in(inner_root, component) };
        let Some(target) = found else {
            warnings.push(format!("{}: o componente '{component}' não existe no pacote", e.name));
            continue;
        };
        if !aylis_core::is_identifier(handler) {
            warnings.push(format!("{}: nome de handler inválido '{handler}' (letras, dígitos e _, sem começar por dígito)", e.name));
            continue;
        }
        let (exposed, handler) = (e.name.clone(), handler.to_string());
        ui.on(target, event, move |ui, _, args| {
            ui.call_handler(frame, &exposed, &handler, args);
        });
    }
    warnings
}

/// Os `values` de um frame ao trocar o pacote de versão maior: renomeia pelo `renamed` do
/// novo e tira as que sumiram. Devolve os valores novos e o que mudou.
pub fn migrate_values(values: &str, new: &Manifest) -> (String, Vec<String>) {
    let old: serde_json::Map<String, Value> = serde_json::from_str(values).unwrap_or_default();
    let mut out = serde_json::Map::new();
    let mut notes = Vec::new();
    for (k, v) in old {
        match new.renamed.iter().find(|(from, _)| from.eq_ignore_ascii_case(&k)) {
            Some((_, to)) if to.is_empty() => notes.push(format!("{k}: removida em {} {}", new.name, new.version)),
            Some((_, to)) => {
                notes.push(format!("{k} → {to}"));
                out.insert(to.clone(), v);
            }
            None if new.property(&k).is_some() => {
                out.insert(k, v);
            }
            None => notes.push(format!("{k}: não existe em {} {}", new.name, new.version)),
        }
    }
    (Value::Object(out).to_string(), notes)
}

/// Se o `form` de um frame é de um pacote, o nome dele.
pub fn package_of(form: &str) -> Option<&str> {
    form.strip_prefix(PREFIX).filter(|n| !n.is_empty())
}

/// O valor `PropValue` de um `values` (para o inspector).
pub fn value_of(values: &str, name: &str) -> Option<Value> {
    let map: serde_json::Map<String, Value> = serde_json::from_str(values).ok()?;
    map.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone())
}

/// `values` com `name` trocado (`Null` tira: volta ao padrão).
pub fn with_value(values: &str, name: &str, value: Value) -> String {
    let mut map: serde_json::Map<String, Value> = serde_json::from_str(values).unwrap_or_default();
    map.retain(|k, _| !k.eq_ignore_ascii_case(name));
    if !value.is_null() {
        map.insert(name.to_string(), value);
    }
    if map.is_empty() { String::new() } else { Value::Object(map).to_string() }
}

/// A linha do `Cargo.toml` do projeto Rust para um pacote de código: o crate em
/// `packages/<Nome>/crate`, visto da pasta do código (`code_dir`, dentro do projeto).
pub fn code_dependency(manifest: &Manifest, project_dir: &std::path::Path, code_dir: &std::path::Path) -> String {
    let depth = code_dir.strip_prefix(project_dir).map_or(1, |rel| rel.components().count());
    let up = "../".repeat(depth);
    format!("{} = {{ path = \"{up}{DIR}/{}/crate\" }}", manifest.crate_name, manifest.name)
}
