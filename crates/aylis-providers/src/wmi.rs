//! O componente `WmiQuery`: consultas e eventos do WMI do Windows (o hardware, os discos, os
//! processos, os serviços...), pela API nativa. A plataforma faz o trabalho
//! ([`aylis_core::platform::WmiService`]; só no Windows), numa thread dela; o resultado
//! volta à tela pelo [`Poster`](aylis_core::Poster) e vai para um `ClientDataSet`.
//!
//! Só consulta (WQL), sempre nesta máquina: nada de chamar métodos do WMI (o
//! `Win32_Process.Create` executaria programas) nem de conectar a outro computador.

use std::sync::{Arc, mpsc};
use std::time::Duration;

use aylis_core::platform::{WmiService, WmiTable, WmiType, WmiValue, WmiWatch};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};
use aylis_data::{Field, FieldType};
use serde_json::{Map, Value as Json};

/// Consulta o WMI: `query` em WQL (`SELECT Name, Size, FreeSpace FROM Win32_LogicalDisk`) no
/// `namespace` (`root\cimv2`).
///
/// - `execute()`: roda e espera (até `timeout`), põe as linhas no `data_set` (um
///   `ClientDataSet`; sem campos, ele ganha um por coluna, no tipo do CIM) e em `result` (JSON),
///   e devolve quantas vieram;
/// - `execute_async()`: o mesmo sem esperar; o fim chega no `on_done` (o número de linhas) ou
///   no `on_error`;
/// - `get_value(consulta, propriedade)`: o valor da propriedade na primeira linha, como texto
///   (`get_value("SELECT SerialNumber FROM Win32_BIOS", "SerialNumber")`);
/// - `get_json(consulta)`: todas as linhas como uma lista JSON de objetos;
/// - `watch`: assina a consulta de eventos (`SELECT * FROM __InstanceCreationEvent WITHIN 2
///   WHERE TargetInstance ISA 'Win32_Process'`); cada evento chega no `on_event`, como um
///   objeto JSON (o `TargetInstance` dentro), e fica em `last_event`.
///
/// Só no Windows; nas outras plataformas o componente existe e responde que não há WMI.
pub struct WmiQuery {
    /// Onde estão as classes (`root\cimv2`, o padrão; só desta máquina).
    pub namespace: String,
    /// A consulta em WQL (com `watch`, uma consulta de eventos).
    pub query: String,
    /// O `ClientDataSet` que recebe as linhas.
    pub data_set: Option<ComponentId>,
    /// Segundos que `execute`, `get_value` e `get_json` esperam.
    pub timeout: f64,
    /// Assina a consulta de eventos enquanto ligado (só com o app rodando).
    pub watch: bool,
    busy: bool,
    row_count: i64,
    result: String,
    last_event: String,
    subscription: Option<WmiWatch>,
    /// A assinatura falhou: não tenta de novo até o `watch` ser religado.
    watch_failed: bool,
}

impl Default for WmiQuery {
    fn default() -> Self {
        Self {
            namespace: "root\\cimv2".into(),
            query: "SELECT * FROM Win32_OperatingSystem".into(),
            data_set: None,
            timeout: 30.0,
            watch: false,
            busy: false,
            row_count: 0,
            result: String::new(),
            last_event: String::new(),
            subscription: None,
            watch_failed: false,
        }
    }
}

impl WmiQuery {
    pub fn busy(&self) -> bool {
        self.busy
    }
    pub fn row_count(&self) -> i64 {
        self.row_count
    }
    /// As linhas da última consulta, em JSON (uma lista de objetos).
    pub fn result(&self) -> String {
        self.result.clone()
    }
    /// O último evento da assinatura, em JSON.
    pub fn last_event(&self) -> String {
        self.last_event.clone()
    }
    fn set_b(&mut self, _: bool) {}
    fn set_i(&mut self, _: i64) {}
    fn set_s(&mut self, _: String) {}

    fn service(ui: &mut Ui) -> Result<&mut Box<dyn WmiService>, String> {
        ui.services_mut().wmi.as_mut().ok_or_else(|| aylis_core::messages::t("o WMI só existe no Windows"))
    }

    fn checked_query(query: &str) -> Result<String, String> {
        let q = query.trim();
        if q.is_empty() {
            return Err(aylis_core::messages::t("o WmiQuery precisa da consulta"));
        }
        Ok(q.to_string())
    }

    /// Roda `query` e espera o resultado (até `timeout` segundos).
    pub fn run(ui: &mut Ui, namespace: &str, query: &str, timeout: f64) -> Result<WmiTable, String> {
        let query = Self::checked_query(query)?;
        let (tx, rx) = mpsc::channel();
        Self::service(ui)?.query(namespace, &query, Box::new(move |r| {
            let _ = tx.send(r);
        }));
        let wait = if timeout.is_finite() && timeout > 0.0 { Duration::from_secs_f64(timeout) } else { Duration::from_secs(30) };
        match rx.recv_timeout(wait) {
            Ok(r) => r,
            Err(mpsc::RecvTimeoutError::Timeout) => Err(aylis_core::messages::t("o WMI não respondeu a tempo")),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(aylis_core::messages::t("o WMI parou")),
        }
    }

    /// Guarda o resultado (`result`, `row_count`) e enche o `data_set`.
    fn finish(ui: &mut Ui, id: ComponentId, table: &WmiTable) -> Result<usize, String> {
        let data_set = ui.get::<WmiQuery>(id).and_then(|q| q.data_set);
        if let Some(q) = ui.get_mut::<WmiQuery>(id) {
            q.result = rows_json(table).to_string();
            q.row_count = table.rows.len() as i64;
        }
        if let Some(ds) = data_set {
            fill(ui, ds, table)?;
        }
        Ok(table.rows.len())
    }

    /// Liga ou desliga a assinatura conforme o `watch`.
    fn sync_watch(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.watch {
            self.subscription = None;
            self.watch_failed = false;
            return;
        }
        if self.subscription.is_some() || self.watch_failed {
            return;
        }
        let id = ctx.id;
        let poster = ctx.ui.poster();
        let listener: aylis_core::platform::WmiListener = Arc::new(move |event| {
            poster.post_to(id, move |ui| match event {
                Ok(props) => {
                    let json = object_json(&props).to_string();
                    if let Some(q) = ui.get_mut::<WmiQuery>(id) {
                        q.last_event = json.clone();
                    }
                    ui.queue_event(id, "on_event", EventArgs::Text(json));
                }
                Err(e) => {
                    if let Some(q) = ui.get_mut::<WmiQuery>(id) {
                        q.subscription = None;
                        q.watch_failed = true;
                    }
                    ui.queue_event(id, "on_error", EventArgs::Text(e));
                }
            });
        });
        let started = Self::checked_query(&self.query).and_then(|query| Self::service(ctx.ui)?.watch(&self.namespace, &query, listener));
        match started {
            Ok(w) => self.subscription = Some(w),
            Err(e) => {
                self.watch_failed = true;
                ctx.fire("on_error", EventArgs::Text(e));
            }
        }
    }

    /// O `watch` ainda não foi aplicado (ligar ou desligar a assinatura).
    fn watch_pending(&self) -> bool {
        if self.watch { self.subscription.is_none() && !self.watch_failed } else { self.subscription.is_some() }
    }
}

impl Component for WmiQuery {
    fn class_name(&self) -> &'static str {
        "WmiQuery"
    }

    properties! {
        namespace: String => "Behavior",
        query: String => "Behavior",
        data_set: Option<ComponentId> => "Data",
        timeout: f64 => "Behavior",
        watch: bool => "Behavior";
        events: on_done, on_event, on_error;
        computed: busy: bool => busy / set_b, row_count: i64 => row_count / set_i, result: String => result / set_s,
            last_event: String => last_event / set_s
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_set == Some(freed) {
            self.data_set = None;
        }
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Int, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("execute", &[], Some(Int)),
            MethodInfo::new("execute_async", &[], None),
            MethodInfo::new("get_value", &[String, String], Some(String)).params(&["query", "property"]),
            MethodInfo::new("get_json", &[String], Some(String)).params(&["query"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| -> Result<std::string::String, std::string::String> {
            args.get(i).and_then(|v| v.to_str().ok()).map(str::to_string).ok_or_else(|| format!("{method} pede o argumento {}", i + 1))
        };
        match method {
            // O componente está emprestado durante o `invoke`: a consulta roda com os valores
            // dele, e o resultado é guardado aqui mesmo.
            "execute" => {
                let table = Self::run(ctx.ui, &self.namespace, &self.query, self.timeout)?;
                self.result = rows_json(&table).to_string();
                self.row_count = table.rows.len() as i64;
                if let Some(ds) = self.data_set {
                    fill(ctx.ui, ds, &table)?;
                }
                Ok(PropValue::Int(table.rows.len() as i64))
            }
            "execute_async" => {
                let id = ctx.id;
                let query = Self::checked_query(&self.query)?;
                let poster = ctx.ui.poster();
                Self::service(ctx.ui)?.query(&self.namespace, &query, Box::new(move |r| {
                    poster.post_to(id, move |ui| {
                        if let Some(q) = ui.get_mut::<WmiQuery>(id) {
                            q.busy = false;
                        }
                        match r.and_then(|t| Self::finish(ui, id, &t)) {
                            Ok(n) => ui.queue_event(id, "on_done", EventArgs::Value(PropValue::Int(n as i64))),
                            Err(e) => ui.queue_event(id, "on_error", EventArgs::Text(e)),
                        }
                    });
                }));
                self.busy = true;
                Ok(PropValue::Null)
            }
            "get_value" => {
                let table = Self::run(ctx.ui, &self.namespace, &text(0)?, self.timeout)?;
                Ok(PropValue::String(first_value(&table, &text(1)?).map(value_text).unwrap_or_default()))
            }
            "get_json" => {
                let table = Self::run(ctx.ui, &self.namespace, &text(0)?, self.timeout)?;
                Ok(PropValue::String(rows_json(&table).to_string()))
            }
            _ => Err(format!("WmiQuery não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        // Só enquanto há uma mudança do `watch` para aplicar; parado, sem timer.
        self.watch_pending().then_some(0.01)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if !ctx.ui.design_mode() {
            self.sync_watch(ctx);
        }
    }

    fn property_changed(&mut self, _ctx: &mut EventCtx<'_>, name: &str) {
        // Religar o `watch` (ou trocar a consulta) tenta de novo depois de uma falha.
        if matches!(name, "watch" | "query" | "namespace") {
            self.watch_failed = false;
            if name != "watch" {
                self.subscription = None;
            }
        }
    }
}

/// O valor da propriedade (sem diferenciar maiúsculas) na primeira linha.
fn first_value<'a>(table: &'a WmiTable, property: &str) -> Option<&'a WmiValue> {
    let at = table.columns.iter().position(|c| c.name.eq_ignore_ascii_case(property.trim()))?;
    table.rows.first()?.get(at)
}

/// O valor como texto: a data em ISO, a lista e o objeto em JSON.
pub fn value_text(v: &WmiValue) -> String {
    match v {
        WmiValue::Null => String::new(),
        WmiValue::Bool(b) => b.to_string(),
        WmiValue::Int(i) => i.to_string(),
        WmiValue::Float(f) => f.to_string(),
        WmiValue::String(s) | WmiValue::DateTime(s) => s.clone(),
        WmiValue::Array(_) | WmiValue::Object(_) => value_json(v).to_string(),
    }
}

/// O valor em JSON.
pub fn value_json(v: &WmiValue) -> Json {
    match v {
        WmiValue::Null => Json::Null,
        WmiValue::Bool(b) => Json::Bool(*b),
        WmiValue::Int(i) => Json::from(*i),
        WmiValue::Float(f) => serde_json::Number::from_f64(*f).map_or(Json::Null, Json::Number),
        WmiValue::String(s) | WmiValue::DateTime(s) => Json::String(s.clone()),
        WmiValue::Array(items) => Json::Array(items.iter().map(value_json).collect()),
        WmiValue::Object(props) => object_json(props),
    }
}

/// Um objeto (as propriedades de um evento ou de um objeto embutido) em JSON.
pub fn object_json(props: &[(String, WmiValue)]) -> Json {
    Json::Object(props.iter().map(|(n, v)| (n.clone(), value_json(v))).collect::<Map<_, _>>())
}

/// As linhas como uma lista JSON de objetos.
pub fn rows_json(table: &WmiTable) -> Json {
    Json::Array(
        table
            .rows
            .iter()
            .map(|row| Json::Object(table.columns.iter().zip(row).map(|(c, v)| (c.name.clone(), value_json(v))).collect::<Map<_, _>>()))
            .collect(),
    )
}

/// O tipo do campo para uma coluna do WMI (lista e objeto viram texto em JSON).
fn field_type(kind: WmiType) -> FieldType {
    match kind {
        WmiType::Integer => FieldType::Integer,
        WmiType::Float => FieldType::Float,
        WmiType::Boolean => FieldType::Boolean,
        WmiType::DateTime => FieldType::DateTime,
        WmiType::Array | WmiType::Object => FieldType::Memo,
        WmiType::String => FieldType::String,
    }
}

/// Põe as linhas no `ClientDataSet`: sem campos, ele ganha um por coluna; com campos, cada um
/// recebe a coluna do mesmo nome (sem diferenciar maiúsculas), e o que não vier fica vazio.
fn fill(ui: &mut Ui, ds: ComponentId, table: &WmiTable) -> Result<(), String> {
    let has_fields = ui.children(ds).iter().any(|&c| ui.class_name(c) == Some("Field"));
    if !has_fields {
        if ui.class_name(ds) != Some("ClientDataSet") {
            return Err(aylis_core::messages::t("o data_set do WmiQuery tem de ser um ClientDataSet"));
        }
        let _ = aylis_data::data_set(ui, ds).close();
        let prefix = ui.name(ds).unwrap_or_default().to_string();
        for c in &table.columns {
            let f = ui.add(Field::new(c.name.clone(), field_type(c.kind)), ds);
            if !prefix.is_empty() {
                let _ = ui.set_name(f, &format!("{prefix}{}", c.name));
            }
        }
    }
    // Uma lista de valores por coluna: a lista e o objeto vão como texto JSON, e um número
    // grande demais para inteiro (o `uint64` acima do limite), como texto do número.
    let names = Json::Array(table.columns.iter().map(|c| Json::String(c.name.clone())).collect());
    let columns = Json::Array(
        (0..table.columns.len())
            .map(|i| {
                Json::Array(
                    table
                        .rows
                        .iter()
                        .map(|r| match r.get(i) {
                            Some(v @ (WmiValue::Array(_) | WmiValue::Object(_))) => Json::String(value_json(v).to_string()),
                            Some(WmiValue::Float(f)) if table.columns[i].kind == WmiType::Integer => Json::from(*f as i64),
                            Some(v) => value_json(v),
                            None => Json::Null,
                        })
                        .collect(),
                )
            })
            .collect(),
    );
    aylis_data::data_set(ui, ds).load_columns(&names, &columns).map_err(|e| e.to_string())
}
