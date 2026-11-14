//! [`ClientDataSet`] (dados em memória com change log), [`DataSource`] e as mensagens
//! que avisam os controles ligados.

use std::fmt;

use aylis_core::{Component, ComponentId, EventCtx, PropEnum, Ui, properties};

use crate::field::FieldKind;
use crate::store::ColumnStore;
use crate::value::Value;

/// Estado do dataset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataSetState {
    #[default]
    Inactive,
    Browse,
    Edit,
    Insert,
}

impl DataSetState {
    pub fn is_editing(self) -> bool {
        matches!(self, DataSetState::Edit | DataSetState::Insert)
    }
}

/// Quando o delta vai para o backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ApplyMode {
    /// Acumula as alterações até `apply_updates()` (como `CachedUpdates`).
    #[default]
    Cached,
    /// Cada `post`/`delete` envia na hora.
    OnPost,
}

impl PropEnum for ApplyMode {
    const NAMES: &'static [&'static str] = &["cached", "on_post"];

    fn from_index(index: usize) -> Option<Self> {
        [ApplyMode::Cached, ApplyMode::OnPost].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Situação de um registro em relação ao que veio do backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowStatus {
    Unmodified,
    Modified,
    Inserted,
}

/// Um registro.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// Identificador interno, usado como `ref` no delta.
    pub id: u64,
    /// Os valores: nas colunas do dataset (carregado) ou próprios (alterado, incluído).
    /// Leia com [`ClientDataSet::row_values`].
    pub(crate) data: RowData,
    pub status: RowStatus,
    /// Valores antes da primeira alteração (registros `Modified` e excluídos).
    pub original: Option<Box<Vec<Value>>>,
    /// Erro devolvido pelo backend na última tentativa de gravar.
    pub error: Option<Box<String>>,
}

/// Onde estão os valores de um registro.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RowData {
    /// Na posição tal do armazenamento em colunas.
    Stored(u32),
    Owned(Box<Vec<Value>>),
}

/// Leitura dos valores de um registro, venham das colunas ou de uma lista.
#[derive(Clone, Copy)]
pub(crate) enum Src<'a> {
    Slice(&'a [Value]),
    Store(&'a ColumnStore, u32),
}

impl Src<'_> {
    pub(crate) fn get(&self, col: usize) -> Value {
        match self {
            Src::Slice(v) => v.get(col).cloned().unwrap_or_default(),
            Src::Store(s, row) => s.get(*row, col),
        }
    }
}

/// Os valores de `row` (com o armazenamento `store`).
pub(crate) fn src<'a>(store: &'a ColumnStore, row: &'a Row) -> Src<'a> {
    match &row.data {
        RowData::Stored(i) => Src::Store(store, *i),
        RowData::Owned(v) => Src::Slice(v),
    }
}

/// Garante valores próprios em `row` (antes de alterar) e devolve-os.
pub(crate) fn owned<'a>(store: &ColumnStore, row: &'a mut Row) -> &'a mut Vec<Value> {
    if let RowData::Stored(i) = row.data {
        row.data = RowData::Owned(Box::new(store.row(i)));
    }
    match &mut row.data {
        RowData::Owned(v) => v,
        RowData::Stored(_) => unreachable!("convertido acima"),
    }
}

/// Tipo de alteração no delta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    Insert,
    Update,
    Delete,
}

impl ChangeKind {
    pub fn name(self) -> &'static str {
        match self {
            ChangeKind::Insert => "insert",
            ChangeKind::Update => "update",
            ChangeKind::Delete => "delete",
        }
    }
}

/// Uma alteração enviada ao backend.
#[derive(Clone, Debug, PartialEq)]
pub struct Change {
    /// Identifica o registro na resposta. Único no processo (não só no dataset): o resultado
    /// de um detalhe, vindo no delta do mestre, acha sozinho o dataset dele.
    pub reference: u64,
    pub kind: ChangeKind,
    /// O nome do dataset do registro (o próprio, ou um detalhe no delta do mestre).
    pub data_set: String,
    /// Campos-chave com os valores originais (update e delete).
    pub key: Vec<(String, Value)>,
    /// Valores antigos: só os campos alterados (update) ou todos (delete).
    pub old: Vec<(String, Value)>,
    /// Valores novos: só os campos alterados (update) ou todos (insert).
    pub new: Vec<(String, Value)>,
    /// Detalhe ligado a um mestre também incluído agora: a chave do mestre ainda é a
    /// provisória, e a gerada pelo banco vai para os campos de ligação ([`crate::Delta::accept`]).
    pub master: Option<MasterLink>,
}

/// O mestre (incluído no mesmo delta) de que um detalhe depende.
#[derive(Clone, Debug, PartialEq)]
pub struct MasterLink {
    pub data_set: String,
    /// O `reference` da alteração do mestre.
    pub reference: u64,
    /// Pares (campo do detalhe, campo do mestre), como o `master_fields`.
    pub fields: Vec<(String, String)>,
}

/// O id de um registro novo: único no processo, para o `ref` do delta não repetir entre
/// datasets.
pub(crate) fn new_row_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// Resposta do backend para uma alteração.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplyResult {
    pub reference: u64,
    /// `None` = gravado; `Some(mensagem)` = recusado.
    pub error: Option<String>,
    /// Valores definidos pelo backend (ids gerados, carimbos de data...).
    pub values: Vec<(String, Value)>,
}

impl ApplyResult {
    pub fn ok(reference: u64) -> Self {
        Self { reference, ..Default::default() }
    }

    pub fn with_value(mut self, field: &str, value: impl Into<Value>) -> Self {
        self.values.push((field.to_string(), value.into()));
        self
    }

    pub fn error(reference: u64, message: impl Into<String>) -> Self {
        Self { reference, error: Some(message.into()), values: Vec::new() }
    }
}

/// Erro de uma operação de dados (mensagem pronta para o usuário).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataError {
    pub message: String,
    /// Campo relacionado, quando houver.
    pub field: Option<String>,
    /// A operação não rodou porque a gravação da edição espera a resposta do `before_post`
    /// (ou da validação) do backend: quem chamou refaz depois (o protocolo refaz sozinho).
    pub waiting: bool,
}

impl DataError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), field: None, waiting: false }
    }

    pub fn on_field(field: &str, message: impl Into<String>) -> Self {
        Self { message: message.into(), field: Some(field.to_string()), waiting: false }
    }

    /// A gravação pendente espera o backend (veja [`waiting`](Self::waiting)).
    pub fn waiting() -> Self {
        Self { message: "a gravação espera a resposta do backend".into(), field: None, waiting: true }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DataError {}

impl From<String> for DataError {
    fn from(message: String) -> Self {
        DataError::new(message)
    }
}

impl From<&str> for DataError {
    fn from(message: &str) -> Self {
        DataError::new(message)
    }
}

pub type DataResult<T = ()> = Result<T, DataError>;

/// O que mudou no dataset (enviado aos controles ligados em [`DataMessage`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataEvent {
    /// Abriu ou fechou.
    ActiveChanged,
    /// Os dados mudaram de forma geral (carga, post, cancel, delete...).
    DataSetChanged,
    /// O cursor mudou de registro.
    Scrolled,
    /// Um campo (ou, com `None`, vários) do registro atual mudou.
    RecordChanged(Option<String>),
    /// Mudou o estado (Browse, Edit, Insert).
    StateChanged,
    /// Antes do `post`: controles com texto pendente devem gravá-lo no campo.
    UpdateData,
}

/// Mensagem entregue (com `Ui::send_message`) aos controles cujo `data_source` aponta
/// para um `DataSource` deste dataset.
#[derive(Clone, Debug, PartialEq)]
pub struct DataMessage {
    pub data_set: ComponentId,
    pub event: DataEvent,
}

pub(crate) type DataSetHook = dyn FnMut(&mut Ui, ComponentId) -> Result<(), String>;
pub(crate) type FetchHook = dyn FnMut(&mut Ui, ComponentId, usize);
pub(crate) type ApplyHandler = dyn FnMut(&[Change]) -> Vec<ApplyResult>;
/// Filtro em Rust: recebe uma função que lê os campos.
pub(crate) type FilterHook = dyn Fn(&dyn Fn(&str) -> Value) -> bool;

#[derive(Default)]
pub(crate) struct Hooks {
    pub(crate) on_new_record: Option<Box<DataSetHook>>,
    pub(crate) before_post: Option<Box<DataSetHook>>,
    pub(crate) after_post: Option<Box<DataSetHook>>,
    pub(crate) before_delete: Option<Box<DataSetHook>>,
    pub(crate) after_scroll: Option<Box<DataSetHook>>,
    pub(crate) apply: Option<Box<ApplyHandler>>,
    pub(crate) fetch: Option<Box<FetchHook>>,
    pub(crate) filter_record: Option<Box<FilterHook>>,
}

/// Coluna aberta: o componente `Field` e o nome.
#[derive(Clone, Debug)]
pub(crate) struct Column {
    pub(crate) field: ComponentId,
    pub(crate) name: String,
    pub(crate) kind: FieldKind,
}

/// Dataset em memória.
///
/// Não sabe nada de banco: os registros chegam do backend (`load_rows`, `load_json`) e as
/// alterações voltam como **delta** (`apply_updates`). Os campos são os componentes
/// [`Field`](crate::Field) filhos. As operações ficam em [`DataSet`](crate::DataSet):
/// `aylis_data::data_set(&mut ui, cds).next()`.
#[derive(Default)]
pub struct ClientDataSet {
    /// Campos-chave separados por `;` (identificam o registro no delta).
    pub key_fields: String,
    /// Quando o delta vai para o backend: `cached` (acumula até o `apply_updates`) ou
    /// `on_post` (cada gravação ou exclusão envia na hora).
    pub apply_mode: ApplyMode,
    /// Só leitura: os registros não podem ser incluídos, alterados nem excluídos.
    pub read_only: bool,
    /// Expressão de filtro (`cidade = 'São Paulo' and limite > 1000`, ver [`crate::expr`]);
    /// vale com `filtered` ligado.
    pub filter: String,
    /// Aplica o `filter`: só os registros que passam nele aparecem (os outros continuam no
    /// delta).
    pub filtered: bool,
    /// Ordem dos registros: campos separados por `;`, com `desc` opcional
    /// (`cidade;limite desc`).
    pub index_field_names: String,
    /// Índices com nome: `porNome=nome|porValor=valor desc;nome`.
    pub indexes: String,
    /// O índice de `indexes` em uso; vale no lugar de
    /// `index_field_names`. Ordenar por campos (`set_index`) volta a ser sem nome.
    pub index_name: String,
    /// Valores dos parâmetros do filtro (`:minimo`): `minimo=100;cidade=São Paulo`. Por
    /// código, [`DataSet::set_param`](crate::DataSet::set_param) aceita qualquer valor.
    pub params: String,
    /// Só os registros nestas situações: `unmodified;modified;inserted` (vazio = todos),
    /// como o `StatusFilter`.
    pub status_filter: String,
    /// Master-detail: o `DataSource` do dataset mestre.
    pub master_source: Option<ComponentId>,
    /// Master-detail: `campo_daqui=campo_do_mestre` separados por `;` (só o nome se
    /// forem iguais): `cliente_id=id`.
    pub master_fields: String,
    /// Master-detail: as alterações deste detalhe vão à parte, no `apply_updates` dele.
    /// Desligado (o padrão), vão junto no delta do mestre, na ordem de gravar.
    pub apply_separately: bool,
    /// Parâmetros dados por código (valem mais que os de `params`).
    pub(crate) param_values: Vec<(String, Value)>,
    /// Registros escondidos pelo filtro ou pelo mestre (continuam no delta).
    pub(crate) hidden: Vec<Row>,
    pub(crate) needs_refilter: bool,
    /// Aberto: os campos e os registros ficam disponíveis; desligar fecha. Ligado pela
    /// propriedade `active`, a abertura acontece no próximo acesso por `data_set(...)`.
    pub(crate) wants_open: bool,
    pub(crate) columns: Vec<Column>,
    /// A posição de cada coluna pelo nome em minúsculas (um grid procura a coluna de cada
    /// célula pelo nome).
    pub(crate) column_index: std::collections::HashMap<String, usize>,
    pub(crate) rows: Vec<Row>,
    /// Os valores dos registros carregados, em colunas (ver `store.rs`).
    pub(crate) store: ColumnStore,
    pub(crate) deleted: Vec<Row>,
    pub(crate) cursor: usize,
    pub(crate) bof: bool,
    pub(crate) eof: bool,
    pub(crate) state: DataSetState,
    pub(crate) buffer: Option<Vec<Value>>,
    pub(crate) insert_at: usize,
    pub(crate) next_temp_key: i64,
    pub(crate) disabled: u32,
    pub(crate) pending_notify: bool,
    pub(crate) pending_error: Option<String>,
    /// Erros de validação que o backend mandou (`on_validate` remoto): campo e mensagem.
    pub(crate) field_errors: Vec<(String, String)>,
    /// Campos que mandaram o texto digitado ao `on_set_text` por nome (backend remoto) e
    /// esperam a resposta (`set_value` ou `set_field_error`).
    pub(crate) awaiting_text: Vec<String>,
    /// Um `post` pedido enquanto havia resposta pendente: roda quando a última chega.
    pub(crate) post_deferred: bool,
    /// O `before_post` ou o `before_delete` esperam a resposta do backend (o registro só
    /// grava ou sai com ela); `answered` deixa a próxima chamada passar sem perguntar de novo.
    pub(crate) asking: Option<Asking>,
    pub(crate) answered: Option<Asking>,
    /// O `on_apply_updates` remoto ainda não respondeu (o dataset está "aplicando") e quantos
    /// registros o último `apply_updates` não gravou.
    pub(crate) applying: bool,
    pub(crate) apply_errors: usize,
    /// Quantos registros a consulta tem no backend (0: não se sabe). Com os dados em
    /// partes, a barra de rolagem da grade usa este total.
    pub record_total: i64,
    /// Limite de registros na memória (0: sem limite): a busca sob demanda para nele, e a
    /// grade avisa que mostra só os primeiros.
    pub max_records: i64,
    /// O backend tem mais registros além dos carregados (busca sob demanda).
    pub(crate) has_more: bool,
    pub(crate) fetching: bool,
    /// O aviso de que ordenar, filtrar ou localizar valeu só para os registros carregados
    /// (há mais no backend e ninguém tratou o `on_server_query`); vazio: nada a avisar.
    pub(crate) partial_notice: String,
    /// O `on_server_query` está rodando: o que o backend fizer no dataset (recarregar,
    /// ordenar) não pergunta de novo.
    pub(crate) querying: bool,
    pub(crate) hooks: Hooks,
}

impl ClientDataSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// O aviso de que ordenar, filtrar ou localizar valeu só para os registros carregados
    /// (vazio: nada a avisar); veja `DataSet::server_query`.
    pub fn partial_notice(&self) -> &str {
        &self.partial_notice
    }

    pub fn with_key(mut self, key_fields: &str) -> Self {
        self.key_fields = key_fields.to_string();
        self
    }

    pub fn with_apply_mode(mut self, mode: ApplyMode) -> Self {
        self.apply_mode = mode;
        self
    }

    pub fn state(&self) -> DataSetState {
        self.state
    }

    pub fn is_active(&self) -> bool {
        self.state != DataSetState::Inactive
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn deleted_rows(&self) -> &[Row] {
        &self.deleted
    }

    pub(crate) fn column(&self, name: &str) -> Option<usize> {
        if let Some(&i) = self.column_index.get(name) {
            return Some(i);
        }
        if name.bytes().any(|b| b.is_ascii_uppercase()) {
            if let Some(&i) = self.column_index.get(&name.to_ascii_lowercase()) {
                return Some(i);
            }
        }
        // Colunas ainda sem o índice (antes de abrir).
        self.columns.iter().position(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Troca as colunas (ao abrir) e refaz o índice pelo nome.
    pub(crate) fn set_columns(&mut self, columns: Vec<Column>) {
        self.column_index = columns.iter().enumerate().rev().map(|(i, c)| (c.name.to_ascii_lowercase(), i)).collect();
        self.columns = columns;
    }

    /// Valores do registro atual (o buffer, durante edição/inserção).
    pub(crate) fn current(&self) -> Option<Src<'_>> {
        match &self.buffer {
            Some(b) => Some(Src::Slice(b)),
            None => self.rows.get(self.cursor).map(|r| src(&self.store, r)),
        }
    }

    /// Os valores de um registro (de [`rows`](Self::rows) ou
    /// [`deleted_rows`](Self::deleted_rows)), um por campo.
    pub fn row_values(&self, row: &Row) -> Vec<Value> {
        match &row.data {
            RowData::Stored(i) => self.store.row(*i),
            RowData::Owned(v) => v.as_ref().clone(),
        }
    }

    /// Memória aproximada dos registros carregados (colunas), em bytes.
    pub fn storage_bytes(&self) -> usize {
        self.store.approx_bytes()
    }

    fn active_prop(&self) -> bool {
        self.is_active() || self.wants_open
    }

    /// A ordem em uso: a do índice nomeado (`index_name`) ou `index_field_names`.
    pub fn effective_index(&self) -> Result<String, String> {
        if self.index_name.trim().is_empty() {
            return Ok(self.index_field_names.clone());
        }
        self.index_names()
            .into_iter()
            .zip(self.indexes.split('|'))
            .find(|(n, _)| n.eq_ignore_ascii_case(self.index_name.trim()))
            .map(|(_, def)| def.split_once('=').map_or("", |(_, fields)| fields).trim().to_string())
            .ok_or_else(|| format!("índice '{}' não existe em indexes", self.index_name.trim()))
    }

    /// Os nomes dos índices de `indexes`.
    pub fn index_names(&self) -> Vec<String> {
        self.indexes.split('|').filter(|d| !d.trim().is_empty()).map(|d| d.split_once('=').map_or(d, |(n, _)| n).trim().to_string()).collect()
    }

    /// O valor de um parâmetro do filtro: o dado por código ou o de `params` (número,
    /// lógico ou texto).
    pub fn param(&self, name: &str) -> Option<Value> {
        if let Some((_, v)) = self.param_values.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)) {
            return Some(v.clone());
        }
        let (_, text) = self.params.split(';').filter_map(|p| p.split_once('=')).find(|(n, _)| n.trim().eq_ignore_ascii_case(name))?;
        let text = text.trim();
        Some(if let Ok(i) = text.parse::<i64>() {
            Value::Int(i)
        } else if let Ok(x) = text.parse::<f64>() {
            Value::Float(x)
        } else if text.eq_ignore_ascii_case("true") || text.eq_ignore_ascii_case("false") {
            Value::Bool(text.eq_ignore_ascii_case("true"))
        } else {
            Value::String(text.to_string())
        })
    }

    /// As situações que `status_filter` deixa passar (`None` = todas).
    pub(crate) fn status_allowed(&self) -> Option<Vec<RowStatus>> {
        let names: Vec<String> = self.status_filter.split(';').map(|s| s.trim().to_ascii_lowercase()).filter(|s| !s.is_empty()).collect();
        if names.is_empty() {
            return None;
        }
        Some(
            names
                .iter()
                .filter_map(|n| match n.as_str() {
                    "unmodified" => Some(RowStatus::Unmodified),
                    "modified" => Some(RowStatus::Modified),
                    "inserted" => Some(RowStatus::Inserted),
                    _ => None,
                })
                .collect(),
        )
    }
}

impl Component for ClientDataSet {
    fn class_name(&self) -> &'static str {
        "ClientDataSet"
    }

    fn props(&self) -> &'static [aylis_core::PropInfo] {
        use aylis_core::{PropInfo, PropKind};
        const PROPS: &[PropInfo] = &[
            PropInfo::new("key_fields", PropKind::String, "Data"),
            PropInfo::new("apply_mode", PropKind::Enum(ApplyMode::NAMES), "Data"),
            PropInfo::new("read_only", PropKind::Bool, "Data"),
            PropInfo::new("active", PropKind::Bool, "Data"),
            PropInfo::new("filter", PropKind::String, "Data"),
            PropInfo::new("filtered", PropKind::Bool, "Data"),
            PropInfo::new("index_field_names", PropKind::String, "Data"),
            PropInfo::new("indexes", PropKind::String, "Data"),
            PropInfo::new("index_name", PropKind::String, "Data"),
            PropInfo::new("params", PropKind::String, "Data"),
            PropInfo::new("status_filter", PropKind::String, "Data"),
            PropInfo::new("master_source", PropKind::Component, "Data"),
            PropInfo::new("master_fields", PropKind::String, "Data"),
            PropInfo::new("apply_separately", PropKind::Bool, "Data"),
            PropInfo::new("record_total", PropKind::Int, "Data"),
            PropInfo::new("max_records", PropKind::Int, "Data"),
            PropInfo::new("partial_notice", PropKind::String, "Data").read_only().runtime(),
            PropInfo::new("on_master_change", PropKind::Event, "Events"),
            PropInfo::new("after_scroll", PropKind::Event, "Events"),
            PropInfo::new("before_post", PropKind::Event, "Events"),
            PropInfo::new("after_post", PropKind::Event, "Events"),
            PropInfo::new("before_delete", PropKind::Event, "Events"),
            PropInfo::new("on_state_change", PropKind::Event, "Events"),
            PropInfo::new("on_apply_updates", PropKind::Event, "Events"),
            PropInfo::new("on_reconcile_error", PropKind::Event, "Events"),
            PropInfo::new("on_error", PropKind::Event, "Events"),
            PropInfo::new("on_fetch", PropKind::Event, "Events"),
            PropInfo::new("on_server_query", PropKind::Event, "Events"),
        ];
        PROPS
    }

    fn get_prop(&self, name: &str) -> Option<aylis_core::PropValue> {
        use aylis_core::PropValue;
        Some(match name {
            "key_fields" => self.key_fields.clone().into(),
            "apply_mode" => PropValue::enum_value(self.apply_mode),
            "read_only" => self.read_only.into(),
            "active" => self.active_prop().into(),
            "filter" => self.filter.clone().into(),
            "filtered" => self.filtered.into(),
            "index_field_names" => self.index_field_names.clone().into(),
            "indexes" => self.indexes.clone().into(),
            "index_name" => self.index_name.clone().into(),
            "params" => self.params.clone().into(),
            "status_filter" => self.status_filter.clone().into(),
            "master_source" => PropValue::Component(self.master_source),
            "master_fields" => self.master_fields.clone().into(),
            "apply_separately" => self.apply_separately.into(),
            "record_total" => self.record_total.into(),
            "max_records" => self.max_records.into(),
            "partial_notice" => self.partial_notice.clone().into(),
            _ => return None,
        })
    }

    fn set_prop(&mut self, name: &str, value: aylis_core::PropValue) -> aylis_core::Result<()> {
        match name {
            "key_fields" => self.key_fields = value.to_str()?.to_string(),
            "apply_mode" => self.apply_mode = value.to_enum()?,
            "read_only" => self.read_only = value.to_bool()?,
            // Abrir precisa do Ui (para achar os campos): fica pedido.
            "active" => self.wants_open = value.to_bool()?,
            "filter" => self.filter = value.to_str()?.to_string(),
            "filtered" => self.filtered = value.to_bool()?,
            "index_field_names" => {
                self.index_field_names = value.to_str()?.to_string();
                self.index_name.clear();
            }
            "indexes" => self.indexes = value.to_str()?.to_string(),
            "index_name" => self.index_name = value.to_str()?.to_string(),
            "params" => self.params = value.to_str()?.to_string(),
            "status_filter" => self.status_filter = value.to_str()?.to_string(),
            "master_source" => self.master_source = value.to_component()?,
            "master_fields" => self.master_fields = value.to_str()?.to_string(),
            "apply_separately" => self.apply_separately = value.to_bool()?,
            "record_total" => self.record_total = value.to_i64()?.max(0),
            "max_records" => self.max_records = value.to_i64()?.max(0),
            _ => return Err(aylis_core::Error::UnknownProperty(name.to_string())),
        }
        if REFILTER_PROPS.contains(&name) {
            self.needs_refilter = true;
        }
        Ok(())
    }

    /// Filtro, índice ou mestre mudou: refaz a visão logo em seguida (o dataset está
    /// emprestado durante esta chamada).
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // O total do backend muda a barra de rolagem da grade: os controles ligados refazem.
        if name == "record_total" && self.is_active() {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| crate::handle::data_set(ui, id).notify(DataEvent::DataSetChanged));
        }
        if self.needs_refilter && self.is_active() && REFILTER_PROPS.contains(&name) {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| {
                let _ = crate::handle::data_set(ui, id).refilter();
            });
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.master_source == Some(freed) {
            self.master_source = None;
        }
    }
}

// ---------------------------------------------------------------------------

/// Propriedades que mudam quais registros aparecem ou a ordem deles.
const REFILTER_PROPS: &[&str] = &["filter", "filtered", "index_field_names", "indexes", "index_name", "params", "status_filter", "master_source", "master_fields"];

/// Liga controles a um dataset.
pub struct DataSource {
    /// O `ClientDataSet` cujos dados os controles ligados mostram e editam.
    pub data_set: Option<ComponentId>,
    /// Começar a digitar num controle coloca o dataset em edição sozinho.
    pub auto_edit: bool,
    /// Desligado, os controles ficam sem dados.
    pub enabled: bool,
}

impl Default for DataSource {
    fn default() -> Self {
        Self { data_set: None, auto_edit: true, enabled: true }
    }
}

impl DataSource {
    pub fn new(data_set: ComponentId) -> Self {
        Self { data_set: Some(data_set), ..Default::default() }
    }
}

impl Component for DataSource {
    fn class_name(&self) -> &'static str {
        "DataSource"
    }

    properties! {
        data_set: Option<ComponentId> => "Data",
        auto_edit: bool => "Data",
        enabled: bool => "Data";
        events: on_data_change, on_state_change
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_set == Some(freed) {
            self.data_set = None;
        }
    }
}

/// O evento-pergunta que o dataset espera (`before_post`, `before_delete`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Asking {
    Post,
    Delete,
}
