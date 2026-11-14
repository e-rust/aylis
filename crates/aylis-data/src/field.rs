//! [`Field`]: a definição de uma coluna do dataset.
//! É um componente filho do `ClientDataSet`, então aparece
//! no arquivo de layout e no designer.

use std::cell::RefCell;

use aylis_core::messages::{t, tf};
use aylis_core::platform::LocaleInfo;
use aylis_core::{Component, ComponentId, Error, PropEnum, PropInfo, PropKind, PropValue, Result, TextAlign};

use crate::expr::Expr;
use crate::mask::Mask;

use crate::format::{date_time_pattern, format_date_time, format_number, parse_date_time, parse_number, plain_number, split};
use crate::value::{Date, DateTime, Decimal, Rounding, Time, Timestamp, Value};

/// Tipo do campo; define como o valor é guardado, exibido e editado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FieldType {
    #[default]
    String,
    /// Texto longo.
    Memo,
    Integer,
    Float,
    /// Dinheiro: decimal exato com `scale` casas.
    Currency,
    Boolean,
    Date,
    Time,
    DateTime,
    Blob,
    /// Inteiro gerado pelo backend; somente leitura. Registros novos recebem um número
    /// provisório negativo até o backend devolver o definitivo.
    AutoInc,
    /// Um instante (a hora do pedido, a última alteração): guardado em UTC, mostrado e
    /// digitado no horário local de quem vê.
    Timestamp,
}

impl FieldType {
    pub const ALL: [FieldType; 12] = [
        FieldType::String,
        FieldType::Memo,
        FieldType::Integer,
        FieldType::Float,
        FieldType::Currency,
        FieldType::Boolean,
        FieldType::Date,
        FieldType::Time,
        FieldType::DateTime,
        FieldType::Blob,
        FieldType::AutoInc,
        FieldType::Timestamp,
    ];

    pub fn is_numeric(self) -> bool {
        matches!(self, FieldType::Integer | FieldType::Float | FieldType::Currency | FieldType::AutoInc)
    }
}

impl PropEnum for FieldType {
    const NAMES: &'static [&'static str] =
        &["string", "memo", "integer", "float", "currency", "boolean", "date", "time", "datetime", "blob", "auto_inc", "timestamp"];

    fn from_index(index: usize) -> Option<Self> {
        FieldType::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// De onde vem o valor do campo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldKind {
    /// Dado do registro (vem do backend e volta no delta).
    #[default]
    Data,
    /// Calculado a partir dos outros campos (`expression` ou `on_calculate`); não vai no
    /// delta.
    Calculated,
    /// Buscado em outro dataset (`lookup_data_set`): o `key_fields` deste registro é
    /// procurado no `lookup_key_fields` de lá e o resultado é o `lookup_result_field`.
    Lookup,
}

impl PropEnum for FieldKind {
    const NAMES: &'static [&'static str] = &["data", "calculated", "lookup"];

    fn from_index(index: usize) -> Option<Self> {
        [FieldKind::Data, FieldKind::Calculated, FieldKind::Lookup].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Maiúsculas/minúsculas forçadas na digitação (como `CharCase`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CharCase {
    #[default]
    Normal,
    Upper,
    Lower,
}

impl PropEnum for CharCase {
    const NAMES: &'static [&'static str] = &["normal", "upper", "lower"];

    fn from_index(index: usize) -> Option<Self> {
        [CharCase::Normal, CharCase::Upper, CharCase::Lower].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

pub type GetTextFn = dyn Fn(&Value, bool) -> String;
pub type SetTextFn = dyn Fn(&str) -> std::result::Result<Value, String>;
pub type ValidateFn = dyn Fn(&Value) -> std::result::Result<(), String>;
/// Cálculo em Rust: recebe uma função que lê os outros campos do registro.
pub type CalculateFn = dyn Fn(&dyn Fn(&str) -> Value) -> Value;

/// Definição de um campo.
///
/// ```ignore
/// ui.add(Field::new("limite", FieldType::Currency).label("Limite").format("R$ #,##0.00").min(0.0), cds);
/// ```
pub struct Field {
    /// Nome da coluna. Vazio = usa o nome do componente.
    pub field_name: String,
    /// O tipo do valor: `string`, `memo`, `integer`, `float`, `currency`, `boolean`, `date`,
    /// `time`, `datetime`, `blob`, `auto_inc` ou `timestamp`.
    pub field_type: FieldType,
    /// Tamanho máximo (texto). 0 = sem limite.
    pub size: usize,
    /// Casas decimais de `currency`.
    pub scale: u8,
    /// Como um valor com mais casas é arredondado para `scale` (ao gravar, vindo da tela, do
    /// backend ou de uma expressão, e ao mostrar): metade para cima (o padrão), bancário,
    /// truncar ou para cima.
    pub rounding: Rounding,
    /// O título do campo nas grades e nos rótulos; vazio: o nome do campo.
    pub display_label: String,
    /// Largura sugerida para grids, em unidades lógicas. 0 = automática.
    pub display_width: f32,
    /// Como o valor aparece (`R$ #,##0.00`, `dd/mm/yyyy`); vazio: o padrão do tipo e do idioma.
    pub display_format: String,
    /// Como um número aparece durante a edição; vazio: o `display_format`.
    pub edit_format: String,
    /// `None` = conforme o tipo (números à direita).
    pub alignment: Option<TextAlign>,
    /// Não aceita gravar o registro com o campo vazio.
    pub required: bool,
    /// Os controles ligados mostram, mas não deixam editar.
    pub read_only: bool,
    /// Aparece nas grades; desligado, a coluna some (o valor continua no registro).
    pub visible: bool,
    /// O menor valor aceito (números); vazio: sem limite.
    pub min: Option<f64>,
    /// O maior valor aceito (números); vazio: sem limite.
    pub max: Option<f64>,
    /// Valor inicial de registros novos, em texto (lido com as regras do tipo).
    pub default_value: String,
    /// Textos para verdadeiro;falso (campos lógicos), como `DisplayValues`.
    pub display_values: String,
    /// Maiúsculas (`upper`) ou minúsculas (`lower`) forçadas no valor; `normal`: como digitado.
    pub char_case: CharCase,
    /// De onde vem o valor: `data` (do registro), `calculated` (a `expression`) ou `lookup`
    /// (procurado em outro dataset).
    pub kind: FieldKind,
    /// Campo calculado: a expressão (`quantidade * preco`, ver [`crate::expr`]).
    pub expression: String,
    /// Lookup: o dataset onde procurar.
    pub lookup_data_set: Option<ComponentId>,
    /// Lookup: campo(s) deste dataset com a chave (`cliente_id`).
    pub key_fields: String,
    /// Lookup: campo(s) do outro dataset que correspondem (`id`).
    pub lookup_key_fields: String,
    /// Lookup: o campo trazido (`nome`).
    pub lookup_result_field: String,
    /// Máscara de edição (`000.000.000-00;0`, ver [`crate::mask`]).
    pub edit_mask: String,
    /// Dado pessoal (LGPD): os controles ligados ao campo contam como `sensitive` (as
    /// gravações do `EventRecorder` mascaram o que é digitado neles).
    pub sensitive: bool,
    parsed: RefCell<Option<std::result::Result<Expr, String>>>,
    calculate: Option<Box<CalculateFn>>,
    get_text: Option<Box<GetTextFn>>,
    set_text: Option<Box<SetTextFn>>,
    validate: Option<Box<ValidateFn>>,
    /// `on_get_text` por evento (backend remoto): textos já recebidos, por valor (JSON).
    remote_texts: RefCell<std::collections::HashMap<String, String>>,
    /// Valores pedidos e ainda sem texto (vão num evento só).
    pending_texts: RefCell<Vec<Value>>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_name: String::new(),
            field_type: FieldType::String,
            size: 0,
            scale: 2,
            rounding: Rounding::HalfUp,
            display_label: String::new(),
            display_width: 0.0,
            display_format: String::new(),
            edit_format: String::new(),
            alignment: None,
            required: false,
            read_only: false,
            visible: true,
            min: None,
            max: None,
            default_value: String::new(),
            display_values: String::new(),
            char_case: CharCase::Normal,
            kind: FieldKind::Data,
            expression: String::new(),
            lookup_data_set: None,
            key_fields: String::new(),
            lookup_key_fields: String::new(),
            lookup_result_field: String::new(),
            edit_mask: String::new(),
            sensitive: false,
            parsed: RefCell::new(None),
            calculate: None,
            get_text: None,
            set_text: None,
            validate: None,
            remote_texts: RefCell::default(),
            pending_texts: RefCell::default(),
        }
    }
}

impl Field {
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        Self { field_name: name.into(), field_type, ..Default::default() }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.display_label = label.into();
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn format(mut self, display_format: impl Into<String>) -> Self {
        self.display_format = display_format.into();
        self
    }

    /// Alinhamento do texto (o padrão é pelo tipo: números à direita).
    pub fn align(mut self, alignment: TextAlign) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Largura sugerida para grids.
    pub fn width(mut self, display_width: f32) -> Self {
        self.display_width = display_width;
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn default_value(mut self, text: impl Into<String>) -> Self {
        self.default_value = text.into();
        self
    }

    pub fn display_values(mut self, values: impl Into<String>) -> Self {
        self.display_values = values.into();
        self
    }

    pub fn char_case(mut self, char_case: CharCase) -> Self {
        self.char_case = char_case;
        self
    }

    /// Como o `OnGetText`: valor → texto. `display` é `true` para exibir e `false` para
    /// editar.
    pub fn on_get_text(mut self, f: impl Fn(&Value, bool) -> String + 'static) -> Self {
        self.get_text = Some(Box::new(f));
        self
    }

    /// Como o `OnSetText`: texto digitado → valor (ou mensagem de erro).
    pub fn on_set_text(mut self, f: impl Fn(&str) -> std::result::Result<Value, String> + 'static) -> Self {
        self.set_text = Some(Box::new(f));
        self
    }

    /// Tem o `on_set_text` em Rust (a closure): ela vale no lugar do evento por nome.
    pub(crate) fn has_set_text(&self) -> bool {
        self.set_text.is_some()
    }

    /// Como o `OnValidate`: recusa um valor com uma mensagem.
    pub fn on_validate(mut self, f: impl Fn(&Value) -> std::result::Result<(), String> + 'static) -> Self {
        self.validate = Some(Box::new(f));
        self
    }

    /// Nome para mensagens e cabeçalhos: o `display_label` ou o nome da coluna.
    /// Campo calculado por expressão.
    pub fn calculated(mut self, expression: impl Into<String>) -> Self {
        self.kind = FieldKind::Calculated;
        self.expression = expression.into();
        self
    }

    /// Campo calculado em Rust: `get("quantidade")` lê os outros campos do registro.
    pub fn on_calculate(mut self, f: impl Fn(&dyn Fn(&str) -> Value) -> Value + 'static) -> Self {
        self.kind = FieldKind::Calculated;
        self.calculate = Some(Box::new(f));
        self
    }

    /// Campo lookup: procura `key_field` deste registro no `lookup_key` de `data_set` e
    /// mostra o `result`.
    pub fn lookup(mut self, data_set: ComponentId, key_field: &str, lookup_key: &str, result: &str) -> Self {
        self.kind = FieldKind::Lookup;
        self.lookup_data_set = Some(data_set);
        self.key_fields = key_field.into();
        self.lookup_key_fields = lookup_key.into();
        self.lookup_result_field = result.into();
        self
    }

    /// Máscara de edição (`000.000.000-00;0`).
    pub fn mask(mut self, mask: impl Into<String>) -> Self {
        self.edit_mask = mask.into();
        self
    }

    /// Calcula o valor de um campo calculado (`get` lê os outros campos).
    pub fn compute(&self, get: &dyn Fn(&str) -> Option<Value>) -> Value {
        if let Some(f) = &self.calculate {
            return f(&|name| get(name).unwrap_or_default());
        }
        if self.expression.trim().is_empty() {
            return Value::Null;
        }
        let mut cache = self.parsed.borrow_mut();
        let parsed = cache.get_or_insert_with(|| Expr::parse(&self.expression));
        match parsed {
            Ok(expr) => expr.eval(get).ok().and_then(|v| self.coerce(v).ok()).unwrap_or_default(),
            Err(_) => Value::Null,
        }
    }

    /// Erro da expressão do campo calculado, se houver (para mostrar no designer).
    pub fn expression_error(&self) -> Option<String> {
        if self.kind != FieldKind::Calculated || self.calculate.is_some() || self.expression.trim().is_empty() {
            return None;
        }
        Expr::parse(&self.expression).err()
    }

    pub fn caption(&self) -> &str {
        if self.display_label.is_empty() { &self.field_name } else { &self.display_label }
    }

    pub fn alignment(&self) -> TextAlign {
        self.alignment.unwrap_or(if self.field_type.is_numeric() { TextAlign::Trailing } else { TextAlign::Leading })
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only || self.field_type == FieldType::AutoInc || self.kind != FieldKind::Data
    }

    fn bool_texts(&self) -> (String, String) {
        let default = t("Sim;Não");
        let values = if self.display_values.is_empty() { &default } else { &self.display_values };
        let (t, f) = values.split_once(';').unwrap_or((values, ""));
        (t.to_string(), f.to_string())
    }

    /// O texto que o evento `on_get_text` deu para `value`, se já chegou. Se não, guarda
    /// o valor para o próximo pedido em lote e devolve `None`; `true` no segundo item quando
    /// a fila estava vazia (quem chamou agenda o envio).
    pub(crate) fn remote_text(&self, value: &Value) -> (Option<String>, bool) {
        let key = crate::json::value_to_json(value).to_string();
        if let Some(text) = self.remote_texts.borrow().get(&key) {
            return (Some(text.clone()), false);
        }
        let mut pending = self.pending_texts.borrow_mut();
        let first = pending.is_empty();
        if !pending.contains(value) {
            pending.push(value.clone());
        }
        (None, first)
    }

    /// Os valores que esperam texto (e esvazia a fila).
    pub(crate) fn take_pending_texts(&self) -> Vec<Value> {
        std::mem::take(&mut *self.pending_texts.borrow_mut())
    }

    /// Guarda textos vindos do `on_get_text` remoto.
    pub(crate) fn store_texts(&self, texts: impl IntoIterator<Item = (Value, String)>) {
        let mut cache = self.remote_texts.borrow_mut();
        for (value, text) in texts {
            cache.insert(crate::json::value_to_json(&value).to_string(), text);
        }
    }

    /// Esquece os textos recebidos (o backend mudou a regra): serão pedidos de novo.
    pub fn clear_texts(&self) {
        self.remote_texts.borrow_mut().clear();
    }

    /// Texto para exibir (`display = true`) ou para editar (`false`).
    pub fn text_of(&self, value: &Value, display: bool, locale: &LocaleInfo) -> String {
        if let Some(f) = &self.get_text {
            return f(value, display);
        }
        let format = if display || self.edit_format.is_empty() { &self.display_format } else { &self.edit_format };
        if let (Some(mask), Value::String(s)) = (Mask::parse(&self.edit_mask), value) {
            return mask.display(s);
        }
        match value {
            Value::Null => String::new(),
            Value::Bool(b) => {
                let (t, f) = self.bool_texts();
                if *b { t } else { f }
            }
            Value::String(s) => s.clone(),
            Value::Int(_) | Value::Float(_) | Value::Decimal(_) => {
                let number = match (value, self.field_type) {
                    (_, FieldType::Currency) => value.as_decimal().and_then(|d| d.round(self.scale, self.rounding)),
                    (Value::Float(f), _) => Decimal::from_f64(*f, 10).map(trim),
                    _ => value.as_decimal(),
                };
                let Some(number) = number else { return value.to_string() };
                if display && !format.is_empty() {
                    format_number(number, format, locale)
                } else if display && self.field_type == FieldType::Currency {
                    format_number(number, &format!("#,##0.{}", "0".repeat(self.scale as usize)), locale)
                } else if !display && !self.edit_format.is_empty() {
                    format_number(number, &self.edit_format, locale)
                } else {
                    plain_number(number, locale)
                }
            }
            Value::Date(d) => format_date_time(Some(*d), None, self.pattern(format, &locale.short_date_format)),
            Value::Time(t) => format_date_time(None, Some(*t), self.pattern(format, &locale.long_time_format)),
            Value::DateTime(dt) => {
                let (d, t) = split(*dt);
                format_date_time(d, t, self.pattern(format, &date_time_pattern(locale)))
            }
            // No horário local de quem vê.
            Value::Timestamp(ts) => {
                let (d, t) = split(ts.to_local());
                format_date_time(d, t, self.pattern(format, &date_time_pattern(locale)))
            }
            Value::Blob(b) => format!("({} bytes)", b.len()),
        }
    }

    fn pattern<'a>(&self, format: &'a str, fallback: &'a str) -> &'a str {
        if format.is_empty() { fallback } else { format }
    }

    /// O valor com as casas do campo, arredondado do jeito dele; estouro é erro (nunca um
    /// valor saturado).
    fn to_scale(&self, value: Decimal) -> std::result::Result<Decimal, String> {
        value.round(self.scale, self.rounding).ok_or_else(|| tf("{0}: o valor {1} não cabe", &[&self.caption(), &value.to_string()]))
    }

    /// Lê o texto digitado. Texto vazio vira `Null`.
    pub fn parse_text(&self, text: &str, locale: &LocaleInfo) -> std::result::Result<Value, String> {
        if let Some(f) = &self.set_text {
            return f(text);
        }
        let trimmed = text.trim();
        if trimmed.is_empty() && self.field_type != FieldType::String && self.field_type != FieldType::Memo {
            return Ok(Value::Null);
        }
        let caption = self.caption();
        if let Some(mask) = Mask::parse(&self.edit_mask) {
            let stored = mask.stored(text);
            if stored.is_empty() {
                return Ok(Value::Null);
            }
            if !mask.is_complete(text) {
                return Err(tf("{0} incompleto: '{1}'", &[&caption, &mask.apply(text)]));
            }
            return Ok(Value::String(stored));
        }
        match self.field_type {
            FieldType::String | FieldType::Memo => {
                if text.is_empty() {
                    return Ok(Value::Null);
                }
                Ok(Value::String(match self.char_case {
                    CharCase::Normal => text.to_string(),
                    CharCase::Upper => text.to_uppercase(),
                    CharCase::Lower => text.to_lowercase(),
                }))
            }
            FieldType::Integer | FieldType::AutoInc => parse_number(trimmed, locale)
                .filter(|d| d.rescale(0) == *d)
                .and_then(|d| d.to_i64())
                .map(Value::Int)
                .ok_or_else(|| tf("'{0}' não é um número inteiro válido para {1}", &[&text, &caption])),
            FieldType::Float => parse_number(trimmed, locale)
                .map(|d| Value::Float(d.to_f64()))
                .ok_or_else(|| tf("'{0}' não é um número válido para {1}", &[&text, &caption])),
            FieldType::Currency => parse_number(trimmed, locale)
                .ok_or_else(|| tf("'{0}' não é um valor válido para {1}", &[&text, &caption]))
                .and_then(|d| self.to_scale(d).map(Value::Decimal)),
            FieldType::Boolean => {
                let (t, f) = self.bool_texts();
                let lower = trimmed.to_lowercase();
                if lower == t.to_lowercase() || ["true", "1", "sim", "s", "yes"].contains(&lower.as_str()) {
                    Ok(Value::Bool(true))
                } else if lower == f.to_lowercase() || ["false", "0", "não", "nao", "n", "no"].contains(&lower.as_str()) {
                    Ok(Value::Bool(false))
                } else {
                    Err(tf("'{0}' não é {1} nem {2}", &[&text, &t, &f]))
                }
            }
            FieldType::Date => {
                let pattern = self.pattern(&self.display_format, &locale.short_date_format);
                parse_date_time(trimmed, pattern)
                    .and_then(|(d, _)| d)
                    .map(Value::Date)
                    .ok_or_else(|| tf("'{0}' não é uma data válida ({1})", &[&text, &pattern]))
            }
            FieldType::Time => {
                let pattern = self.pattern(&self.display_format, &locale.long_time_format);
                parse_date_time(trimmed, pattern)
                    .and_then(|(_, t)| t)
                    .or_else(|| Time::parse_iso(trimmed))
                    .map(Value::Time)
                    .ok_or_else(|| tf("'{0}' não é uma hora válida ({1})", &[&text, &pattern]))
            }
            FieldType::DateTime | FieldType::Timestamp => {
                let full = date_time_pattern(locale);
                let pattern = self.pattern(&self.display_format, &full);
                let typed = parse_date_time(trimmed, pattern)
                    .and_then(|(d, t)| Some(DateTime::new(d?, t.unwrap_or_default())))
                    .or_else(|| {
                        let date_only = parse_date_time(trimmed, &locale.short_date_format)?.0?;
                        Some(DateTime::new(date_only, Time::default()))
                    })
                    .ok_or_else(|| tf("'{0}' não é uma data/hora válida ({1})", &[&text, &pattern]))?;
                // O instante é o do horário local que o usuário digitou.
                Ok(if self.field_type == FieldType::Timestamp { Value::Timestamp(Timestamp::from_local(typed)) } else { Value::DateTime(typed) })
            }
            FieldType::Blob => Err(tf("{0} não pode ser digitado", &[&caption])),
        }
    }

    /// Ajusta o tipo do valor ao do campo (por exemplo inteiro num campo `float`), ou
    /// recusa.
    pub fn coerce(&self, value: Value) -> std::result::Result<Value, String> {
        // O nome do campo só no erro: a carga converte milhões de valores.
        let wrong = |v: &Value| format!("{} não aceita valor do tipo {}", self.caption(), v.kind_name());
        Ok(match (self.field_type, value) {
            (FieldType::Integer | FieldType::AutoInc, Value::Int(i)) => Value::Int(i),
            (FieldType::Float, Value::Float(f)) => Value::Float(f),
            (_, Value::Null) => Value::Null,
            (FieldType::String | FieldType::Memo, Value::String(s)) => Value::String(s),
            (FieldType::Integer | FieldType::AutoInc, v) => Value::Int(v.as_i64().ok_or_else(|| wrong(&v))?),
            (FieldType::Float, v) => Value::Float(v.as_f64().ok_or_else(|| wrong(&v))?),
            (FieldType::Currency, v) => Value::Decimal(self.to_scale(v.as_decimal().ok_or_else(|| wrong(&v))?)?),
            (FieldType::Boolean, Value::Bool(b)) => Value::Bool(b),
            (FieldType::Date, v) => Value::Date(v.as_date().ok_or_else(|| wrong(&v))?),
            (FieldType::Time, Value::Time(t)) => Value::Time(t),
            (FieldType::DateTime, Value::DateTime(dt)) => Value::DateTime(dt),
            (FieldType::DateTime, Value::Date(d)) => Value::DateTime(DateTime::new(d, Time::default())),
            (FieldType::DateTime, Value::Timestamp(ts)) => Value::DateTime(ts.to_local()),
            (FieldType::Timestamp, Value::Timestamp(ts)) => Value::Timestamp(ts),
            // Uma data e hora sem fuso é o horário local de quem grava.
            (FieldType::Timestamp, Value::DateTime(dt)) => Value::Timestamp(Timestamp::from_local(dt)),
            (FieldType::Timestamp, Value::Date(d)) => Value::Timestamp(Timestamp::from_local(DateTime::new(d, Time::default()))),
            (FieldType::Blob, Value::Blob(b)) => Value::Blob(b),
            (_, v) => return Err(wrong(&v)),
        })
    }

    /// Restrições do valor (tamanho, mínimo, máximo e `on_validate`). `required` é
    /// conferido no `post`.
    pub fn check(&self, value: &Value) -> std::result::Result<(), String> {
        let caption = self.caption();
        if let Value::String(s) = value {
            if self.size > 0 && s.chars().count() > self.size {
                return Err(tf("{0}: no máximo {1} caracteres", &[&caption, &self.size]));
            }
        }
        if let Some(number) = value.as_f64().filter(|_| self.field_type.is_numeric()) {
            if self.min.is_some_and(|min| number < min) {
                return Err(tf("{0}: o mínimo é {1}", &[&caption, &self.min.unwrap_or_default()]));
            }
            if self.max.is_some_and(|max| number > max) {
                return Err(tf("{0}: o máximo é {1}", &[&caption, &self.max.unwrap_or_default()]));
            }
        }
        match &self.validate {
            Some(f) => f(value),
            None => Ok(()),
        }
    }

    /// Valor inicial de um registro novo.
    pub fn initial_value(&self, locale: &LocaleInfo) -> Value {
        if self.default_value.is_empty() {
            return Value::Null;
        }
        let v = match self.field_type {
            FieldType::Date => Date::parse_iso(&self.default_value).map(Value::Date),
            FieldType::Time => Time::parse_iso(&self.default_value).map(Value::Time),
            FieldType::DateTime => DateTime::parse_iso(&self.default_value).map(Value::DateTime),
            FieldType::Timestamp => Timestamp::parse_iso(&self.default_value).map(Value::Timestamp),
            FieldType::Float | FieldType::Currency | FieldType::Integer => {
                Decimal::parse(&self.default_value).map(Value::Decimal).and_then(|v| self.coerce(v).ok())
            }
            FieldType::Boolean => Some(Value::Bool(matches!(self.default_value.to_lowercase().as_str(), "true" | "1" | "sim"))),
            _ => None,
        };
        v.or_else(|| self.parse_text(&self.default_value, locale).ok()).unwrap_or(Value::Null)
    }
}

/// Tira zeros desnecessários de um decimal vindo de `f64`.
fn trim(d: Decimal) -> Decimal {
    let mut d = d;
    while d.scale() > 0 && d.mantissa() % 10 == 0 {
        d = Decimal::new(d.mantissa() / 10, d.scale() - 1);
    }
    d
}

const FIELD_PROPS: &[PropInfo] = &[
    PropInfo::new("field_name", PropKind::String, "Data"),
    PropInfo::new("type", PropKind::Enum(FieldType::NAMES), "Data"),
    PropInfo::new("size", PropKind::Int, "Data"),
    PropInfo::new("scale", PropKind::Int, "Data"),
    PropInfo::new("rounding", PropKind::Enum(Rounding::NAMES), "Data"),
    PropInfo::new("display_label", PropKind::String, "Appearance"),
    PropInfo::new("display_width", PropKind::Float, "Appearance"),
    PropInfo::new("display_format", PropKind::String, "Appearance"),
    PropInfo::new("edit_format", PropKind::String, "Appearance"),
    PropInfo::new("alignment", PropKind::Enum(<TextAlign as PropEnum>::NAMES), "Appearance"),
    PropInfo::new("required", PropKind::Bool, "Behavior"),
    PropInfo::new("read_only", PropKind::Bool, "Behavior"),
    PropInfo::new("visible", PropKind::Bool, "Behavior"),
    PropInfo::new("min", PropKind::Float, "Behavior"),
    PropInfo::new("max", PropKind::Float, "Behavior"),
    PropInfo::new("default_value", PropKind::String, "Data"),
    PropInfo::new("display_values", PropKind::String, "Appearance"),
    PropInfo::new("char_case", PropKind::Enum(CharCase::NAMES), "Behavior"),
    PropInfo::new("edit_mask", PropKind::String, "Behavior"),
    PropInfo::new("sensitive", PropKind::Bool, "Behavior"),
    PropInfo::new("kind", PropKind::Enum(FieldKind::NAMES), "Data"),
    PropInfo::new("expression", PropKind::String, "Data"),
    PropInfo::new("lookup_data_set", PropKind::Component, "Lookup"),
    PropInfo::new("key_fields", PropKind::String, "Lookup"),
    PropInfo::new("lookup_key_fields", PropKind::String, "Lookup"),
    PropInfo::new("lookup_result_field", PropKind::String, "Lookup"),
    PropInfo::new("on_change", PropKind::Event, "Events"),
    // Por nome (backend remoto): o texto de exibição vem em lote e assíncrono; a
    // validação chega depois e trava o `post` enquanto houver erro.
    PropInfo::new("on_get_text", PropKind::Event, "Events"),
    // O texto digitado vai ao handler, que grava o valor (ou recusa): `aylis_data::set_text`.
    PropInfo::new("on_set_text", PropKind::Event, "Events"),
    PropInfo::new("on_validate", PropKind::Event, "Events"),
];

impl Component for Field {
    fn class_name(&self) -> &'static str {
        "Field"
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.lookup_data_set == Some(freed) {
            self.lookup_data_set = None;
        }
    }

    fn props(&self) -> &'static [PropInfo] {
        FIELD_PROPS
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        let opt = |v: Option<f64>| v.map_or(PropValue::Null, PropValue::Float);
        Some(match name {
            "field_name" => self.field_name.clone().into(),
            "type" => PropValue::enum_value(self.field_type),
            "size" => PropValue::Int(self.size as i64),
            "scale" => PropValue::Int(self.scale as i64),
            "rounding" => PropValue::enum_value(self.rounding),
            "display_label" => self.display_label.clone().into(),
            "display_width" => self.display_width.into(),
            "display_format" => self.display_format.clone().into(),
            "edit_format" => self.edit_format.clone().into(),
            "alignment" => self.alignment.map_or(PropValue::Null, PropValue::enum_value),
            "required" => self.required.into(),
            "read_only" => self.read_only.into(),
            "sensitive" => self.sensitive.into(),
            "visible" => self.visible.into(),
            "min" => opt(self.min),
            "max" => opt(self.max),
            "default_value" => self.default_value.clone().into(),
            "display_values" => self.display_values.clone().into(),
            "char_case" => PropValue::enum_value(self.char_case),
            "edit_mask" => self.edit_mask.clone().into(),
            "kind" => PropValue::enum_value(self.kind),
            "expression" => self.expression.clone().into(),
            "lookup_data_set" => PropValue::Component(self.lookup_data_set),
            "key_fields" => self.key_fields.clone().into(),
            "lookup_key_fields" => self.lookup_key_fields.clone().into(),
            "lookup_result_field" => self.lookup_result_field.clone().into(),
            _ => return None,
        })
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        let opt = |v: &PropValue| -> Result<Option<f64>> {
            match v {
                PropValue::Null => Ok(None),
                v => v.to_f64().map(Some),
            }
        };
        match name {
            "field_name" => self.field_name = value.to_str()?.to_string(),
            "type" => self.field_type = value.to_enum()?,
            "size" => self.size = value.to_i64()?.max(0) as usize,
            "scale" => self.scale = value.to_i64()?.clamp(0, 18) as u8,
            "rounding" => self.rounding = value.to_enum()?,
            "display_label" => self.display_label = value.to_str()?.to_string(),
            "display_width" => self.display_width = value.to_f32()?,
            "display_format" => self.display_format = value.to_str()?.to_string(),
            "edit_format" => self.edit_format = value.to_str()?.to_string(),
            "alignment" => {
                self.alignment = match value {
                    PropValue::Null => None,
                    v => Some(v.to_enum()?),
                }
            }
            "required" => self.required = value.to_bool()?,
            "read_only" => self.read_only = value.to_bool()?,
            "sensitive" => self.sensitive = value.to_bool()?,
            "visible" => self.visible = value.to_bool()?,
            "min" => self.min = opt(&value)?,
            "max" => self.max = opt(&value)?,
            "default_value" => self.default_value = value.to_str()?.to_string(),
            "display_values" => self.display_values = value.to_str()?.to_string(),
            "char_case" => self.char_case = value.to_enum()?,
            "edit_mask" => self.edit_mask = value.to_str()?.to_string(),
            "kind" => self.kind = value.to_enum()?,
            "expression" => {
                self.expression = value.to_str()?.to_string();
                *self.parsed.borrow_mut() = None;
            }
            "lookup_data_set" => self.lookup_data_set = value.to_component()?,
            "key_fields" => self.key_fields = value.to_str()?.to_string(),
            "lookup_key_fields" => self.lookup_key_fields = value.to_str()?.to_string(),
            "lookup_result_field" => self.lookup_result_field = value.to_str()?.to_string(),
            _ => return Err(Error::UnknownProperty(name.to_string())),
        }
        Ok(())
    }
}
