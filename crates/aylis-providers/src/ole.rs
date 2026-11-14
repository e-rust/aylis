//! O componente `OleObject`: automação OLE de um programa instalado (Excel, Word, Outlook...).
//! A plataforma faz o trabalho
//! ([`aylis_core::platform::OleService`]; só no Windows), e o componente fala com ela por
//! caminhos como os do VBA: `Workbooks.Add`, `ActiveSheet.Range("A1").Value`.
//!
//! O caminho é só endereço: nomes separados por `.`, cada um com argumentos literais entre
//! parênteses (números, `"textos"`, `true`/`false`, `null`). Não há variáveis, contas nem
//! laços (nada de linguagem própria).

use aylis_core::platform::{OleReleaser, OleService, OleValue, ole_allowed, ole_date, ole_date_parts};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventCtx, MethodInfo, PropValue, Ui, properties};
use aylis_core::providers::CellValue;

use crate::export::Export;

/// Comanda um programa instalado pela automação OLE (Excel, Word, Outlook),
/// por caminhos do VBA: `ActiveSheet.Range("A1").Value`.
///
/// Os métodos, a partir do objeto do programa:
///
/// - `get_value(caminho)`: o valor, como texto (uma matriz vem em JSON);
/// - `set_value(caminho, valor)`: grava (`true`/`false` como booleano, o resto como texto, que o
///   Excel converte em número ou data como se fosse digitado);
/// - `call(caminho)`: chama o método (`Workbooks.Add`, `ActiveWorkbook.SaveAs("c:\\x.xlsx")`);
/// - `get_values`/`set_values`: uma matriz JSON inteira numa chamada (`Range("A1:C3").Value`);
/// - `put_data(célula, data_source)`: os títulos e os registros de um dataset a partir da
///   célula, numa chamada só (o Excel; usa o `Resize` da célula).
///
/// Só no Windows; nas outras plataformas o componente existe e `open` responde que não há
/// automação. Programas fora do Office entram pelo código do app
/// (`aylis_core::platform::allow_ole`).
pub struct OleObject {
    /// O programa (`Excel.Application`, `Word.Application`); fora do Office, só o que o app
    /// permitir (`allow_ole`).
    pub prog_id: String,
    /// Usa o programa já aberto (só cria se não houver).
    pub use_running: bool,
    /// Milissegundos para esperar cada resposta do programa (um diálogo aberto nele o prende).
    pub call_timeout: u32,
    root: Option<u64>,
    /// Larga o objeto quando o componente é liberado (o `Drop` não tem o `Ui`).
    releaser: Option<OleReleaser>,
}

impl Default for OleObject {
    fn default() -> Self {
        Self { prog_id: "Excel.Application".into(), use_running: false, call_timeout: 30_000, root: None, releaser: None }
    }
}

impl Drop for OleObject {
    fn drop(&mut self) {
        if let (Some(root), Some(release)) = (self.root.take(), &self.releaser) {
            release(root);
        }
    }
}

impl OleObject {
    fn opened(&self) -> bool {
        self.root.is_some()
    }
    fn set_opened(&mut self, _: bool) {}

    fn service(ui: &mut Ui) -> Result<&mut Box<dyn OleService>, String> {
        ui.services_mut().ole.as_mut().ok_or_else(|| aylis_core::messages::t("a automação OLE só existe no Windows"))
    }

    /// Conecta ao programa (cria ou pega o aberto). Já conectado, não faz nada.
    fn open(&mut self, ui: &mut Ui) -> Result<u64, String> {
        if let Some(root) = self.root {
            return Ok(root);
        }
        let prog_id = self.prog_id.trim().to_string();
        if prog_id.is_empty() {
            return Err(aylis_core::messages::t("o OleObject precisa do prog_id"));
        }
        if !ole_allowed(&prog_id) {
            return Err(aylis_core::messages::tf("a automação de '{0}' não foi permitida pelo app", &[&prog_id]));
        }
        let timeout = f64::from(self.call_timeout) / 1000.0;
        let service = Self::service(ui)?;
        service.set_timeout(timeout);
        let root = service.create(&prog_id, self.use_running)?;
        self.releaser = service.releaser();
        self.root = Some(root);
        Ok(root)
    }

    fn close(&mut self, ui: &mut Ui) {
        if let Some(root) = self.root.take() {
            if let Ok(s) = Self::service(ui) {
                s.release(root);
            }
        }
    }

    /// Anda pelo caminho até o último nome: o objeto onde ele está, o nome e os argumentos.
    /// Os objetos do meio do caminho são largados por quem chama ([`walk_release`]).
    fn walk(&mut self, ui: &mut Ui, path: &str) -> Result<(u64, Vec<u64>, Segment), String> {
        let mut segments = parse_path(path)?;
        let last = segments.pop().ok_or_else(|| aylis_core::messages::t("caminho vazio"))?;
        let root = self.open(ui)?;
        let service = Self::service(ui)?;
        let mut object = root;
        let mut held = Vec::new();
        for s in segments {
            match service.invoke(object, &s.name, s.args) {
                Ok(OleValue::Object(o)) => {
                    held.push(o);
                    object = o;
                }
                Ok(_) => {
                    release_all(service, &held);
                    return Err(aylis_core::messages::tf("'{0}' não é um objeto", &[&s.name]));
                }
                Err(e) => {
                    release_all(service, &held);
                    return Err(e);
                }
            }
        }
        Ok((object, held, last))
    }

    /// O valor no fim do caminho (um objeto devolvido é largado e vira `Null`).
    pub fn read(&mut self, ui: &mut Ui, path: &str) -> Result<OleValue, String> {
        let (object, held, last) = self.walk(ui, path)?;
        let service = Self::service(ui)?;
        let result = service.invoke(object, &last.name, last.args);
        release_all(service, &held);
        match result? {
            OleValue::Object(o) => {
                service.release(o);
                Ok(OleValue::Null)
            }
            v => Ok(v),
        }
    }

    /// Grava o valor no fim do caminho.
    pub fn write(&mut self, ui: &mut Ui, path: &str, value: OleValue) -> Result<(), String> {
        let (object, held, last) = self.walk(ui, path)?;
        let service = Self::service(ui)?;
        let result = service.put(object, &last.name, last.args, value);
        release_all(service, &held);
        result
    }

    /// Escreve os títulos e os registros de um `DataSource` (ou de um `Export`, com os campos
    /// dele) a partir da célula (`Range("A1")`): uma matriz só, numa chamada. Devolve quantos
    /// registros foram.
    pub fn put_data(&mut self, ui: &mut Ui, cell: &str, source: ComponentId) -> Result<usize, String> {
        let table = if ui.get::<Export>(source).is_some() {
            Export::table(ui, source)?
        } else {
            let e = Export { data_source: Some(source), ..Export::default() };
            crate::export::table_of(ui, &e)?
        };
        let mut rows = Vec::with_capacity(table.rows.len() + 1);
        rows.push(table.columns.iter().map(|c| OleValue::String(c.title.clone())).collect::<Vec<_>>());
        for row in &table.rows {
            rows.push(row.iter().map(cell_value).collect());
        }
        let cols = table.columns.len().max(1);
        let count = table.rows.len();
        let path = format!("{}.Resize({}, {}).Value", cell.trim(), rows.len(), cols);
        self.write(ui, &path, OleValue::Array(rows))?;
        Ok(count)
    }
}

fn release_all(service: &mut Box<dyn OleService>, held: &[u64]) {
    for o in held.iter().rev() {
        service.release(*o);
    }
}

fn cell_value(c: &CellValue) -> OleValue {
    match c {
        CellValue::Empty => OleValue::Null,
        CellValue::Text(t) => OleValue::String(t.clone()),
        CellValue::Number(n) => OleValue::Float(*n),
        CellValue::Bool(b) => OleValue::Bool(*b),
        CellValue::Date(y, m, d) => OleValue::Date(ole_date(*y, *m, *d, 0, 0, 0)),
        CellValue::DateTime(y, m, d, h, mi, s) => OleValue::Date(ole_date(*y, *m, *d, *h, *mi, *s)),
    }
}

impl Component for OleObject {
    fn class_name(&self) -> &'static str {
        "OleObject"
    }

    properties! {
        prog_id: String => "Behavior",
        use_running: bool => "Behavior",
        call_timeout: u32 => "Behavior";
        computed: opened: bool => opened / set_opened
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, Component as Comp, Float, Int, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("open", &[], Some(Bool)),
            MethodInfo::new("release", &[], None),
            MethodInfo::new("get_value", &[String], Some(String)).params(&["path"]),
            MethodInfo::new("get_number", &[String], Some(Float)).params(&["path"]),
            MethodInfo::new("set_value", &[String, String], None).params(&["path", "value"]),
            MethodInfo::new("call", &[String], Some(String)).params(&["path"]),
            MethodInfo::new("get_values", &[String], Some(String)).params(&["path"]),
            MethodInfo::new("set_values", &[String, String], None).params(&["path", "values"]),
            MethodInfo::new("put_data", &[String, Comp], Some(Int)).params(&["cell", "data_source"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| -> Result<std::string::String, std::string::String> {
            args.get(i).and_then(|v| v.to_str().ok()).map(str::to_string).ok_or_else(|| format!("{method} pede o argumento {}", i + 1))
        };
        match method {
            "open" => self.open(ctx.ui).map(|_| PropValue::Bool(true)),
            "release" => {
                self.close(ctx.ui);
                Ok(PropValue::Null)
            }
            "get_value" | "call" | "get_values" => {
                let v = self.read(ctx.ui, &text(0)?)?;
                Ok(PropValue::String(if method == "get_values" { to_json(&v).to_string() } else { to_text(&v) }))
            }
            "get_number" => match self.read(ctx.ui, &text(0)?)? {
                OleValue::Int(i) => Ok(PropValue::Float(i as f64)),
                OleValue::Float(f) | OleValue::Date(f) => Ok(PropValue::Float(f)),
                OleValue::Bool(b) => Ok(PropValue::Float(if b { -1.0 } else { 0.0 })),
                OleValue::Null => Ok(PropValue::Float(0.0)),
                OleValue::String(s) => s.trim().replace(',', ".").parse().map(PropValue::Float).map_err(|_| aylis_core::messages::tf("'{0}' não é um número", &[&s])),
                _ => Err(aylis_core::messages::t("o valor não é um número")),
            },
            "set_value" => {
                let value = match args.get(1) {
                    Some(PropValue::Bool(b)) => OleValue::Bool(*b),
                    Some(PropValue::Int(i)) => OleValue::Int(*i),
                    Some(PropValue::Float(f)) => OleValue::Float(*f),
                    Some(PropValue::Null) | None => OleValue::Null,
                    // `true`/`false` vão como booleano: a conversão do texto pelo COM depende
                    // do idioma (em português ele esperaria "Verdadeiro").
                    Some(v) => match v.to_str().unwrap_or_default() {
                        t if t.eq_ignore_ascii_case("true") => OleValue::Bool(true),
                        t if t.eq_ignore_ascii_case("false") => OleValue::Bool(false),
                        t => OleValue::String(t.to_string()),
                    },
                };
                self.write(ctx.ui, &text(0)?, value).map(|_| PropValue::Null)
            }
            "set_values" => {
                let json: serde_json::Value = serde_json::from_str(&text(1)?).map_err(|e| aylis_core::messages::tf("matriz JSON inválida: {0}", &[&e.to_string()]))?;
                self.write(ctx.ui, &text(0)?, from_json_matrix(&json)?).map(|_| PropValue::Null)
            }
            "put_data" => {
                let source = match args.get(1) {
                    Some(PropValue::Component(Some(c))) => *c,
                    _ => return Err(aylis_core::messages::t("put_data pede o data_source")),
                };
                self.put_data(ctx.ui, &text(0)?, source).map(|n| PropValue::Int(n as i64))
            }
            _ => Err(format!("OleObject não tem o método '{method}'")),
        }
    }

}

/// O valor como texto: a data em ISO (`2026-11-12` ou `2026-11-12 08:30:00`), a matriz em
/// JSON.
pub fn to_text(v: &OleValue) -> String {
    match v {
        OleValue::Null | OleValue::Object(_) => String::new(),
        OleValue::Bool(b) => b.to_string(),
        OleValue::Int(i) => i.to_string(),
        OleValue::Float(f) => f.to_string(),
        OleValue::String(s) => s.clone(),
        OleValue::Date(d) => date_text(*d),
        OleValue::Array(_) => to_json(v).to_string(),
    }
}

fn date_text(d: f64) -> String {
    let (y, m, day, h, mi, s) = ole_date_parts(d);
    if h == 0 && mi == 0 && s == 0 { format!("{y:04}-{m:02}-{day:02}") } else { format!("{y:04}-{m:02}-{day:02} {h:02}:{mi:02}:{s:02}") }
}

/// O valor em JSON (um valor só vira uma matriz 1 × 1 no `get_values`, como o Excel faz com
/// uma célula).
fn to_json(v: &OleValue) -> serde_json::Value {
    use serde_json::Value as J;
    let scalar = |v: &OleValue| match v {
        OleValue::Null | OleValue::Object(_) => J::Null,
        OleValue::Bool(b) => J::Bool(*b),
        OleValue::Int(i) => J::from(*i),
        OleValue::Float(f) => serde_json::Number::from_f64(*f).map_or(J::Null, J::Number),
        OleValue::String(s) => J::String(s.clone()),
        OleValue::Date(d) => J::String(date_text(*d)),
        OleValue::Array(_) => J::Null,
    };
    match v {
        OleValue::Array(rows) => J::Array(rows.iter().map(|r| J::Array(r.iter().map(scalar).collect())).collect()),
        other => J::Array(vec![J::Array(vec![scalar(other)])]),
    }
}

/// Uma matriz JSON (`[[1, "a"], [2, "b"]]`; uma lista simples vira uma linha).
fn from_json_matrix(json: &serde_json::Value) -> Result<OleValue, String> {
    use serde_json::Value as J;
    let scalar = |v: &J| match v {
        J::Null => OleValue::Null,
        J::Bool(b) => OleValue::Bool(*b),
        J::Number(n) => n.as_i64().map_or_else(|| OleValue::Float(n.as_f64().unwrap_or(0.0)), OleValue::Int),
        J::String(s) => OleValue::String(s.clone()),
        other => OleValue::String(other.to_string()),
    };
    let J::Array(rows) = json else { return Err(aylis_core::messages::t("a matriz tem de ser uma lista JSON")) };
    let rows: Vec<Vec<OleValue>> = if rows.iter().all(|r| r.is_array()) {
        rows.iter().map(|r| r.as_array().map(|c| c.iter().map(scalar).collect()).unwrap_or_default()).collect()
    } else {
        vec![rows.iter().map(scalar).collect()]
    };
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    if rows.is_empty() || width == 0 {
        return Err(aylis_core::messages::t("a matriz está vazia"));
    }
    // Linhas curtas completam com vazio (a matriz do OLE é retangular).
    Ok(OleValue::Array(rows.into_iter().map(|mut r| {
        r.resize(width, OleValue::Null);
        r
    }).collect()))
}

/// Um nome do caminho com os argumentos.
#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub name: String,
    pub args: Vec<OleValue>,
}

/// Lê o caminho (`Workbooks(1).Worksheets("Dados").Range("A1").Value`).
pub fn parse_path(path: &str) -> Result<Vec<Segment>, String> {
    let bad = |why: &str| aylis_core::messages::tf("caminho inválido '{0}': {1}", &[&path, &why]);
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    let skip_ws = |i: &mut usize| {
        while *i < chars.len() && chars[*i].is_whitespace() {
            *i += 1;
        }
    };
    loop {
        skip_ws(&mut i);
        let start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        if i == start || chars[start].is_ascii_digit() {
            return Err(bad(&aylis_core::messages::t("falta um nome")));
        }
        let name: String = chars[start..i].iter().collect();
        let mut args = Vec::new();
        skip_ws(&mut i);
        if i < chars.len() && chars[i] == '(' {
            i += 1;
            loop {
                skip_ws(&mut i);
                if i < chars.len() && chars[i] == ')' && args.is_empty() {
                    i += 1;
                    break;
                }
                args.push(parse_literal(&chars, &mut i).map_err(|e| bad(&e))?);
                skip_ws(&mut i);
                match chars.get(i) {
                    Some(',') => i += 1,
                    Some(')') => {
                        i += 1;
                        break;
                    }
                    _ => return Err(bad(&aylis_core::messages::t("falta fechar o parêntese"))),
                }
            }
        }
        out.push(Segment { name, args });
        skip_ws(&mut i);
        match chars.get(i) {
            None => return Ok(out),
            Some('.') => i += 1,
            Some(c) => return Err(bad(&aylis_core::messages::tf("caractere inesperado '{0}'", &[&c.to_string()]))),
        }
    }
}

/// Um literal: `"texto"` (aspas dobradas ou `\"` dentro), número, `true`, `false`, `null`.
fn parse_literal(chars: &[char], i: &mut usize) -> Result<OleValue, String> {
    match chars.get(*i) {
        Some('"') => {
            *i += 1;
            let mut s = String::new();
            loop {
                match chars.get(*i) {
                    None => return Err(aylis_core::messages::t("texto sem as aspas do fim")),
                    Some('\\') if matches!(chars.get(*i + 1), Some('"') | Some('\\')) => {
                        s.push(chars[*i + 1]);
                        *i += 2;
                    }
                    Some('"') if chars.get(*i + 1) == Some(&'"') => {
                        s.push('"');
                        *i += 2;
                    }
                    Some('"') => {
                        *i += 1;
                        return Ok(OleValue::String(s));
                    }
                    Some(c) => {
                        s.push(*c);
                        *i += 1;
                    }
                }
            }
        }
        Some(c) if c.is_ascii_digit() || *c == '-' || *c == '+' || *c == '.' => {
            let start = *i;
            while *i < chars.len() && (chars[*i].is_ascii_alphanumeric() || matches!(chars[*i], '.' | '-' | '+')) {
                *i += 1;
            }
            let t: String = chars[start..*i].iter().collect();
            if let Ok(n) = t.parse::<i64>() {
                Ok(OleValue::Int(n))
            } else {
                t.parse::<f64>().map(OleValue::Float).map_err(|_| aylis_core::messages::tf("número inválido '{0}'", &[&t]))
            }
        }
        Some(_) => {
            let start = *i;
            while *i < chars.len() && chars[*i].is_alphanumeric() {
                *i += 1;
            }
            let t: String = chars[start..*i].iter().collect();
            match t.to_ascii_lowercase().as_str() {
                "true" => Ok(OleValue::Bool(true)),
                "false" => Ok(OleValue::Bool(false)),
                "null" | "nothing" => Ok(OleValue::Null),
                _ => Err(aylis_core::messages::t("argumento: só números, \"textos\", true, false e null")),
            }
        }
        None => Err(aylis_core::messages::t("falta o argumento")),
    }
}
