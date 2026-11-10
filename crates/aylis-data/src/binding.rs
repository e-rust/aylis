//! As ligações de propriedades a dados ([`aylis_core::Binding`], o LiveBindings): mostra o
//! campo (ou o texto montado, ou a expressão) na propriedade quando o dataset muda e grava de
//! volta quando o usuário muda uma propriedade ligada só a um campo.
//!
//! O ligar fica no núcleo (`Ui::set_binding`, o JSON do form, a IDE em "Ligar a campo…");
//! aqui, [`install`] (chamado pelo [`register`](crate::register)) liga o `Ui` a este motor.
//!
//! O que vai em `path`:
//! - `nome`: o campo, como o dataset mostra (`display_format`, máscara);
//! - `Total: {valor:moeda}`: um texto com campos; depois do `:`, `moeda`, um formato de
//!   número (`#,##0.00`) ou de data (`dd/mm/yyyy`);
//! - `=quantidade * preco`: uma expressão com os campos do mesmo dataset (a do filtro).

use std::cell::Cell;
use std::rc::Rc;

use aylis_core::{Binding, ComponentId, PropKind, PropValue, Ui};

use crate::dataset::{DataEvent, DataSource};
use crate::expr::Expr;
use crate::handle::{data_set, view};
use crate::value::Value;

/// Liga o `Ui` ao motor das ligações.
pub fn install(ui: &mut Ui) {
    ui.set_binding_sink(Rc::new(write_back), Rc::new(|ui: &mut Ui, id| apply(ui, id)));
}

/// O dataset por trás do `DataSource` da ligação (se ligado e habilitado).
fn data_set_of(ui: &Ui, source: Option<ComponentId>) -> Option<ComponentId> {
    let s = ui.get::<DataSource>(source?)?;
    if s.enabled { s.data_set } else { None }
}

/// Mostra os valores de todas as ligações de `id`.
pub fn apply(ui: &mut Ui, id: ComponentId) {
    let mut bindings: Vec<Binding> = ui.bindings(id).to_vec();
    // As listas antes: o `value` de uma lista suspensa escolhe entre os `values`.
    bindings.sort_by_key(|b| !LIST_PROPS.contains(&b.property.as_str()));
    for b in bindings {
        if b.is_field() {
            metadata(ui, id, &b);
        }
        if let Some(value) = compute(ui, id, &b) {
            if ui.get_prop(id, &b.property).ok().as_ref() != Some(&value) {
                let _ = ui.set_prop(id, &b.property, value);
            }
        }
    }
}

/// Os metadados do campo no componente (se ele tiver a propriedade): tamanho, máscara,
/// leitura e dado pessoal.
fn metadata(ui: &mut Ui, id: ComponentId, b: &Binding) {
    // No designer só o valor aparece: os metadados iriam para o JSON do form.
    if ui.design_mode() {
        return;
    }
    let Some(ds) = data_set_of(ui, b.source) else { return };
    let v = view(ui, ds);
    let Ok(field) = v.field(&b.path) else { return };
    let read_only = field.is_read_only() || ui.get::<crate::ClientDataSet>(ds).is_some_and(|c| c.read_only);
    let (size, mask, sensitive) = (field.size, field.edit_mask.clone(), field.sensitive);
    let main = matches!(b.property.as_str(), "text" | "value" | "checked" | "item_index" | "date");
    if !main {
        return;
    }
    let mut set = |name: &str, value: PropValue| {
        if ui.prop_info(id, name).is_some() && ui.get_prop(id, name).ok().as_ref() != Some(&value) {
            let _ = ui.set_prop(id, name, value);
        }
    };
    set("read_only", PropValue::Bool(read_only));
    if b.property == "text" {
        if size > 0 {
            set("max_length", PropValue::Int(size as i64));
        }
        if !mask.is_empty() {
            set("mask", PropValue::String(mask));
        }
        if sensitive {
            set("sensitive", PropValue::Bool(true));
        }
    }
}

/// O dataset avisou: mostra de novo os componentes ligados a ele.
pub(crate) fn notify(ui: &mut Ui, sources: &[ComponentId], event: &DataEvent) {
    if *event == DataEvent::UpdateData {
        return;
    }
    let mut bound: Vec<ComponentId> = Vec::new();
    for &s in sources {
        for c in ui.bound_to(s) {
            if !bound.contains(&c) {
                bound.push(c);
            }
        }
    }
    for c in bound {
        // Este componente está gravando de volta (o `edit` do dataset avisa no meio): mostrar
        // o campo agora apagaria o valor que o usuário acabou de escolher.
        if WRITING.with(Cell::get) == Some(c) {
            continue;
        }
        // O campo que mudou foi este mesmo componente (o usuário digitando): não sobrescreve.
        if let DataEvent::RecordChanged(Some(field)) = event {
            if ui.bindings(c).iter().any(|b| b.is_field() && b.path.eq_ignore_ascii_case(field)) && is_focused(ui, c) {
                continue;
            }
        }
        apply(ui, c);
    }
}

fn is_focused(ui: &Ui, id: ComponentId) -> bool {
    ui.control(id).is_some_and(|c| c.state().focused)
}

/// As propriedades que recebem o campo de todos os registros (separados por `;`): as listas
/// e os rótulos do eixo do `Chart`.
pub const LIST_PROPS: [&str; 4] = ["items", "values", "labels", "series"];

/// As séries do `Chart` ligadas a campos: `Vendas=vendas|Custos[line;#EF4444]=custos` (o nome
/// e as opções da série antes do `=`, o campo depois) ou só os campos (`vendas;custos`), um
/// ponto por registro: `Vendas=10,20,30|Custos=5,8,9`, o formato do `series`.
fn series_of(v: &crate::handle::DataView<'_>, path: &str) -> String {
    let parts: Vec<&str> = if path.contains('=') { path.split('|').collect() } else { path.split(';').collect() };
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| {
            let (head, field) = p.rsplit_once('=').map_or((p, p), |(h, f)| (h.trim(), f.trim()));
            let values: Vec<String> = (0..v.record_count()).map(|r| v.value_at(r, field).ok().and_then(|x| x.as_f64()).unwrap_or(0.0).to_string()).collect();
            format!("{head}={}", values.join(","))
        })
        .collect::<Vec<_>>()
        .join("|")
}

/// O valor da ligação, já no tipo da propriedade.
pub fn compute(ui: &Ui, id: ComponentId, b: &Binding) -> Option<PropValue> {
    let ds = data_set_of(ui, b.source)?;
    let v = view(ui, ds);
    if !v.is_active() {
        return None;
    }
    let kind = ui.prop_info(id, &b.property)?.kind;
    if let Some(expr) = b.path.strip_prefix('=') {
        let expr = Expr::parse(expr).ok()?;
        let value = expr.eval(&|name| v.value(name).ok()).ok()?;
        return Some(to_prop(&value, kind, &text_of(&value)));
    }
    if b.path.contains('{') {
        return Some(PropValue::String(template(ui, ds, &b.path)));
    }
    // As séries de um gráfico: um ponto por registro em cada uma.
    if b.property == "series" {
        return Some(PropValue::String(series_of(&v, &b.path)));
    }
    // As listas (`items` e `values` de uma lista suspensa, `labels` de um gráfico): o campo
    // de todos os registros.
    if LIST_PROPS.contains(&b.property.as_str()) {
        let all: Vec<String> = (0..v.record_count()).map(|r| v.text_at(r, &b.path).unwrap_or_default().replace(';', ",")).collect();
        return Some(PropValue::String(all.join(";")));
    }
    if v.is_empty() {
        return Some(empty(kind));
    }
    let value = v.value(&b.path).ok()?;
    let text = if is_focused(ui, id) { v.edit_text(&b.path).ok()? } else { v.text(&b.path).ok()? };
    Some(to_prop(&value, kind, &text))
}

fn text_of(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Decimal(d) => d.to_f64().to_string(),
        Value::String(s) => s.clone(),
        other => format!("{other:?}"),
    }
}

/// O valor vazio de cada tipo (dataset sem registros).
fn empty(kind: PropKind) -> PropValue {
    match kind {
        PropKind::Bool => PropValue::Bool(false),
        PropKind::Int => PropValue::Int(0),
        PropKind::Float | PropKind::Angle | PropKind::Duration => PropValue::Float(0.0),
        _ => PropValue::String(String::new()),
    }
}

fn to_prop(value: &Value, kind: PropKind, text: &str) -> PropValue {
    match kind {
        PropKind::Bool => PropValue::Bool(value.as_bool().unwrap_or_else(|| value.as_f64().is_some_and(|n| n != 0.0))),
        PropKind::Int => PropValue::Int(value.as_i64().or_else(|| value.as_f64().map(|f| f.round() as i64)).unwrap_or(0)),
        PropKind::Float | PropKind::Angle | PropKind::Duration => PropValue::Float(value.as_f64().unwrap_or(0.0)),
        PropKind::Enum(_) => PropValue::Enum(text.to_string()),
        PropKind::Color => aylis_core::Color::from_hex(text.trim()).map_or_else(|| PropValue::String(text.to_string()), PropValue::Color),
        _ => PropValue::String(text.to_string()),
    }
}

fn decimal(value: &Value, scale: u8) -> Option<crate::Decimal> {
    value.as_decimal().or_else(|| value.as_f64().and_then(|f| crate::Decimal::from_f64(f, scale)))
}

/// `Total: {valor:moeda}` com os campos do registro atual.
fn template(ui: &Ui, ds: ComponentId, text: &str) -> String {
    let v = view(ui, ds);
    let locale = ui.locale();
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('}') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let inner = &rest[open + 1..open + close];
        let (name, format) = inner.split_once(':').map_or((inner.trim(), ""), |(n, f)| (n.trim(), f.trim()));
        let shown = match (format, v.value(name)) {
            ("", _) => v.text(name).unwrap_or_default(),
            (_, Ok(value)) if value.is_null() => String::new(),
            ("moeda" | "currency", Ok(value)) => match decimal(&value, 2) {
                Some(n) => format!("{} {}", locale.currency_symbol, crate::format::format_number(n, "#,##0.00", &locale)),
                None => text_of(&value),
            },
            (pattern, Ok(value)) if value.as_f64().is_some() => match decimal(&value, 6) {
                Some(n) => crate::format::format_number(n, pattern, &locale),
                None => text_of(&value),
            },
            (pattern, Ok(Value::Date(d))) => crate::format::format_date_time(Some(d), None, pattern),
            (pattern, Ok(Value::DateTime(dt))) => crate::format::format_date_time(Some(dt.date), Some(dt.time), pattern),
            (pattern, Ok(Value::Timestamp(ts))) => {
                let dt = ts.to_local();
                crate::format::format_date_time(Some(dt.date), Some(dt.time), pattern)
            }
            (_, Ok(value)) => text_of(&value),
            (_, Err(_)) => format!("{{{inner}?}}"),
        };
        out.push_str(&shown);
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

thread_local! {
    /// O componente que está gravando de volta agora.
    static WRITING: Cell<Option<ComponentId>> = const { Cell::new(None) };
}

/// O usuário mudou um componente ligado: grava as propriedades ligadas só a um campo.
fn write_back(ui: &mut Ui, id: ComponentId) {
    let outer = WRITING.with(|w| w.replace(Some(id)));
    write_fields(ui, id);
    WRITING.with(|w| w.set(outer));
}

fn write_fields(ui: &mut Ui, id: ComponentId) {
    let bindings: Vec<Binding> = ui.bindings(id).iter().filter(|b| b.is_field() && !LIST_PROPS.contains(&b.property.as_str())).cloned().collect();
    for b in bindings {
        let Some(ds) = data_set_of(ui, b.source) else { continue };
        let Ok(now) = ui.get_prop(id, &b.property) else { continue };
        // Só se mudou de verdade (o evento pode vir de um clique que não mexeu no valor).
        if compute(ui, id, &b).as_ref() == Some(&now) {
            continue;
        }
        let read_only = view(ui, ds).field(&b.path).map_or(true, |f| f.is_read_only());
        if read_only {
            apply(ui, id);
            continue;
        }
        let auto_edit = b.source.and_then(|s| ui.get::<DataSource>(s)).is_some_and(|s| s.auto_edit);
        let mut d = data_set(ui, ds);
        if !d.state().is_editing() {
            if !auto_edit || d.edit().is_err() {
                drop(d);
                apply(ui, id);
                continue;
            }
        }
        let result = match &now {
            PropValue::Bool(v) => d.set_value(&b.path, *v),
            PropValue::Int(n) => d.set_value(&b.path, *n),
            PropValue::Float(n) => d.set_value(&b.path, *n),
            PropValue::Enum(s) => d.set_text(&b.path, s),
            PropValue::Color(c) => d.set_text(&b.path, &c.to_hex()),
            PropValue::String(s) => d.set_text(&b.path, s),
            other => d.set_text(&b.path, &format!("{other:?}")),
        };
        // O erro de validação aparece no próprio componente (o `error` do Edit), se ele tiver.
        let error = match result {
            Ok(()) => String::new(),
            Err(e) => e.message,
        };
        drop(d);
        if ui.prop_info(id, "error").is_some() {
            let _ = ui.set_prop(id, "error", error);
        }
    }
}
