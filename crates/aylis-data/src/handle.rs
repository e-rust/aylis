//! Operações sobre um [`ClientDataSet`] que vive no [`Ui`].
//!
//! ```ignore
//! let mut ds = aylis_data::data_set(&mut ui, cds);
//! ds.first()?;
//! while !ds.eof() {
//!     println!("{}", ds.text("nome")?);
//!     ds.next()?;
//! }
//! ds.edit()?;
//! ds.set_text("limite", "1.500,00")?;
//! ds.post()?;
//! ```
//!
//! Cada operação que muda dados, estado ou posição avisa os controles ligados (por um
//! `DataSource`) com uma [`DataMessage`].

use aylis_core::messages::{t, tf};
use aylis_core::{Answer, Asked, ComponentId, EventArgs, PropValue, Ui};

use crate::dataset::{
    ApplyMode, Asking, ApplyResult, Change, ChangeKind, ClientDataSet, Column, DataError, DataEvent, DataMessage, DataResult,
    DataSetHook, DataSetState, DataSource, Hooks, MasterLink, Row, RowData, RowStatus, Src, new_row_id, owned, src,
};
use crate::store::{ColumnStore, SortKey};

/// Manda num evento só os valores que esperam texto do `on_get_text` do campo `id`:
/// `{"field": "valor", "values": [1, 2, 3]}`. A resposta é a operação `field_texts`.
fn request_texts(ui: &mut Ui, id: ComponentId) {
    let Some(field) = ui.get::<Field>(id) else { return };
    let values = field.take_pending_texts();
    if values.is_empty() {
        return;
    }
    let name = if field.field_name.is_empty() { ui.name(id).unwrap_or_default().to_string() } else { field.field_name.clone() };
    let args = serde_json::json!({ "field": name, "values": values.iter().map(crate::json::value_to_json).collect::<Vec<_>>() });
    ui.fire(id, "on_get_text", &EventArgs::Text(args.to_string()));
}

/// Uma linha como o grid a vê: os registros carregados e, durante uma inclusão, o
/// registro novo na posição onde vai entrar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowInfo<'a> {
    /// `None` para o registro novo (ainda em inclusão).
    pub status: Option<RowStatus>,
    /// Erro devolvido pelo backend.
    pub error: Option<&'a str>,
    /// É o registro atual.
    pub current: bool,
    /// Identificador do registro (o marcador): não muda com
    /// a ordem nem com o filtro. `None` no registro novo.
    pub id: Option<u64>,
}
use crate::expr::Expr;
use crate::field::{Field, FieldKind, FieldType};
use crate::value::Value;

/// Valor de uma coluna num registro: o dado guardado ou, em campos calculados e lookup,
/// o valor obtido agora (`depth` evita ciclos entre campos).
fn cell_value(ui: &Ui, columns: &[Column], values: &Src<'_>, col: usize, depth: u8) -> Value {
    let Some(column) = columns.get(col) else { return Value::Null };
    match column.kind {
        FieldKind::Data => values.get(col),
        _ if depth > 8 => Value::Null,
        kind => {
            let Some(field) = ui.get::<Field>(column.field) else { return Value::Null };
            let get = |name: &str| {
                columns.iter().position(|c| c.name.eq_ignore_ascii_case(name)).map(|i| cell_value(ui, columns, values, i, depth + 1))
            };
            if kind == FieldKind::Calculated {
                return field.compute(&get);
            }
            let Some(ds) = field.lookup_data_set else { return Value::Null };
            let keys: Vec<Value> = field.key_fields.split(';').map(|k| get(k.trim()).unwrap_or_default()).collect();
            if keys.iter().all(Value::is_null) {
                return Value::Null;
            }
            view(ui, ds).lookup_at(&field.lookup_key_fields, &keys, &field.lookup_result_field, depth + 1).unwrap_or_default()
        }
    }
}

/// Lê `nome;cidade desc` como (coluna, decrescente).
fn index_spec(columns: &[Column], spec: &str) -> Result<Vec<(usize, bool)>, String> {
    spec.split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut words = part.split_whitespace();
            let name = words.next().unwrap_or_default();
            let desc = words.next().is_some_and(|w| w.eq_ignore_ascii_case("desc"));
            columns
                .iter()
                .position(|c| c.name.eq_ignore_ascii_case(name))
                .map(|i| (i, desc))
                .ok_or_else(|| format!("índice: campo '{name}' não existe"))
        })
        .collect()
}

/// Pares (campo do detalhe, campo do mestre) de `master_fields`.
fn master_pairs(spec: &str) -> Vec<(String, String)> {
    spec.split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|p| match p.split_once('=') {
            Some((d, m)) => (d.trim().to_string(), m.trim().to_string()),
            None => (p.to_string(), p.to_string()),
        })
        .collect()
}

/// Operações (leitura e escrita) sobre o dataset `id`. Se o dataset tiver `active`
/// ligado e ainda estiver fechado, abre.
pub fn data_set(ui: &mut Ui, id: ComponentId) -> DataSet<'_> {
    let mut ds = DataSet { ui, id };
    let wants_open = ds.ui.get::<ClientDataSet>(id).is_some_and(|c| c.wants_open && !c.is_active());
    if wants_open {
        let _ = ds.open();
    }
    let refilter = ds.ui.get::<ClientDataSet>(id).is_some_and(|c| c.needs_refilter && c.is_active() && !c.state.is_editing());
    if refilter {
        let _ = ds.refilter();
    }
    ds
}

/// Só leitura (útil onde só se tem `&Ui`, como na pintura de um grid).
pub fn view(ui: &Ui, id: ComponentId) -> DataView<'_> {
    DataView { ui, id }
}

/// Abre todos os datasets com `active` ligado (por exemplo depois de carregar um layout).
pub fn open_active(ui: &mut Ui) {
    let pending: Vec<ComponentId> = ui
        .components()
        .filter(|&c| ui.get::<ClientDataSet>(c).is_some_and(|d| d.wants_open && !d.is_active()))
        .collect();
    for id in pending {
        let _ = data_set(ui, id).open();
    }
}

// ===========================================================================

/// Leitura do dataset.
#[derive(Clone, Copy)]
pub struct DataView<'a> {
    ui: &'a Ui,
    id: ComponentId,
}

impl<'a> DataView<'a> {
    /// A gravação espera o backend (o `before_post`, o `before_delete` ou o `on_apply_updates`
    /// remotos): os controles DB* não deixam mexer até a resposta.
    pub fn is_waiting(&self) -> bool {
        self.cds().is_ok_and(|c| c.asking.is_some() || c.applying)
    }

    fn cds(&self) -> DataResult<&'a ClientDataSet> {
        self.ui
            .get::<ClientDataSet>(self.id)
            .ok_or_else(|| DataError::new("o componente não é um ClientDataSet (ou já foi liberado)"))
    }

    pub fn id(&self) -> ComponentId {
        self.id
    }

    /// Outro dataset do mesmo `Ui` (um detalhe).
    pub(crate) fn other(&self, id: ComponentId) -> DataView<'a> {
        DataView { ui: self.ui, id }
    }

    pub fn state(&self) -> DataSetState {
        self.cds().map_or(DataSetState::Inactive, |c| c.state)
    }

    pub fn is_active(&self) -> bool {
        self.state() != DataSetState::Inactive
    }

    pub fn record_count(&self) -> usize {
        self.cds().map_or(0, |c| c.rows.len())
    }

    pub fn is_empty(&self) -> bool {
        self.record_count() == 0 && self.state() != DataSetState::Insert
    }

    /// Posição do registro atual (na inserção, onde o novo vai entrar).
    pub fn rec_no(&self) -> Option<usize> {
        let cds = self.cds().ok()?;
        match cds.state {
            DataSetState::Inactive => None,
            DataSetState::Insert => Some(cds.insert_at),
            _ if cds.rows.is_empty() => None,
            _ => Some(cds.cursor),
        }
    }

    /// Verdadeiro depois de `first` ou de tentar voltar antes do
    /// primeiro registro.
    pub fn bof(&self) -> bool {
        self.cds().map_or(true, |c| c.bof || c.rows.is_empty())
    }

    /// Verdadeiro depois de `last` ou de tentar avançar além do
    /// último registro (então `while !eof { ...; next }` passa por todos).
    pub fn eof(&self) -> bool {
        self.cds().map_or(true, |c| c.eof || c.rows.is_empty())
    }

    /// Está no primeiro registro (para habilitar botões de navegação).
    pub fn at_first(&self) -> bool {
        self.cds().map_or(true, |c| c.cursor == 0)
    }

    /// Está no último registro.
    pub fn at_last(&self) -> bool {
        self.cds().map_or(true, |c| c.rows.is_empty() || c.cursor + 1 >= c.rows.len())
    }

    pub fn field_names(&self) -> Vec<String> {
        self.cds().map(|c| c.columns.iter().map(|col| col.name.clone()).collect()).unwrap_or_default()
    }

    fn column(&self, name: &str) -> DataResult<(usize, &'a Column)> {
        let cds = self.cds()?;
        let i = cds.column(name).ok_or_else(|| DataError::on_field(name, format!("campo '{name}' não existe")))?;
        Ok((i, &cds.columns[i]))
    }

    /// O componente `Field` de uma coluna.
    pub fn field(&self, name: &str) -> DataResult<&'a Field> {
        let (_, col) = self.column(name)?;
        self.ui.get::<Field>(col.field).ok_or_else(|| DataError::on_field(name, "o campo foi liberado"))
    }

    /// Id do componente `Field` de uma coluna.
    pub fn field_id(&self, name: &str) -> DataResult<ComponentId> {
        Ok(self.column(name)?.1.field)
    }

    /// Valor do campo no registro atual (`Null` se não houver registro).
    pub fn value(&self, name: &str) -> DataResult<Value> {
        let (i, _) = self.column(name)?;
        let cds = self.cds()?;
        Ok(cds.current().map(|v| cell_value(self.ui, &cds.columns, &v, i, 0)).unwrap_or_default())
    }

    /// Procura o registro com `key_fields` = `keys` e devolve o `result_field` dele.
    /// `None` se não achar.
    pub fn lookup(&self, key_fields: &str, keys: &[Value], result_field: &str) -> Option<Value> {
        self.lookup_at(key_fields, keys, result_field, 0)
    }

    fn lookup_at(&self, key_fields: &str, keys: &[Value], result_field: &str, depth: u8) -> Option<Value> {
        let cds = self.cds().ok()?;
        let cols: Vec<usize> = key_fields.split(';').map(|k| cds.column(k.trim())).collect::<Option<_>>()?;
        let result = cds.column(result_field.trim())?;
        let row = cds.rows.iter().chain(cds.hidden.iter()).find(|r| {
            cols.iter().zip(keys).all(|(&c, k)| {
                crate::expr::compare(&cell_value(self.ui, &cds.columns, &src(&cds.store, r), c, depth), k) == Some(std::cmp::Ordering::Equal)
            })
        })?;
        Some(cell_value(self.ui, &cds.columns, &src(&cds.store, row), result, depth))
    }

    /// Texto para exibir (com `display_format`, `on_get_text`...).
    pub fn text(&self, name: &str) -> DataResult<String> {
        self.display_text(name, &self.value(name)?)
    }

    /// Texto de exibição de `value` no campo `name`: com `on_get_text` ligado (backend
    /// remoto), o que o evento deu; enquanto não chega, o texto padrão (e o valor entra no
    /// próximo pedido, que leva todos os que faltam de uma vez).
    fn display_text(&self, name: &str, value: &Value) -> DataResult<String> {
        let field = self.field(name)?;
        let id = self.field_id(name)?;
        if !value.is_null() && self.ui.has_handler(id, "on_get_text") {
            let (text, schedule) = field.remote_text(value);
            if let Some(text) = text {
                return Ok(text);
            }
            if schedule {
                self.ui.poster().post(move |ui| request_texts(ui, id));
            }
        }
        Ok(field.text_of(value, true, &self.ui.locale()))
    }

    /// Os erros de validação vindos do backend (campo, mensagem).
    pub fn field_errors(&self) -> Vec<(String, String)> {
        self.cds().map(|c| c.field_errors.clone()).unwrap_or_default()
    }

    /// Texto para editar (sem a formatação de exibição).
    pub fn edit_text(&self, name: &str) -> DataResult<String> {
        Ok(self.field(name)?.text_of(&self.value(name)?, false, &self.ui.locale()))
    }

    /// Texto de exibição de um campo num registro qualquer (com o `display_format`): o dos
    /// relatórios.
    pub fn text_at(&self, row: usize, name: &str) -> DataResult<String> {
        self.display_text(name, &self.value_at(row, name)?)
    }

    /// `value` como o campo `name` exibe (a soma de um campo em moeda sai em moeda).
    pub fn format(&self, name: &str, value: &Value) -> DataResult<String> {
        self.display_text(name, value)
    }

    /// Valor de um campo num registro qualquer (para grids).
    pub fn value_at(&self, row: usize, name: &str) -> DataResult<Value> {
        let (i, _) = self.column(name)?;
        let cds = self.cds()?;
        let editing_here = cds.buffer.is_some() && cds.state == DataSetState::Edit && row == cds.cursor;
        let values = if editing_here { cds.buffer.as_deref().map(Src::Slice) } else { cds.rows.get(row).map(|r| src(&cds.store, r)) };
        Ok(values.map(|v| cell_value(self.ui, &cds.columns, &v, i, 0)).unwrap_or_default())
    }

    /// Há mais registros no backend além dos carregados (busca sob demanda).
    pub fn has_more(&self) -> bool {
        self.cds().is_ok_and(|c| c.has_more)
    }

    /// A situação do registro atual: inalterado, alterado ou
    /// incluído desde a carga (ou desde o último `apply_updates`). Durante uma inclusão,
    /// `Inserted`.
    pub fn update_status(&self) -> Option<RowStatus> {
        let cds = self.cds().ok()?;
        if cds.state == DataSetState::Insert {
            return Some(RowStatus::Inserted);
        }
        cds.rows.get(cds.cursor).map(|r| r.status)
    }

    /// Quantas linhas um grid mostra: os registros mais o novo, durante a inclusão.
    pub fn visible_count(&self) -> usize {
        let Ok(cds) = self.cds() else { return 0 };
        cds.rows.len() + usize::from(cds.state == DataSetState::Insert)
    }

    /// Linha atual na contagem de [`visible_count`](Self::visible_count).
    pub fn visible_current(&self) -> Option<usize> {
        self.rec_no()
    }

    fn physical(&self, index: usize) -> Option<Option<usize>> {
        let cds = self.cds().ok()?;
        if cds.state == DataSetState::Insert {
            match index.cmp(&cds.insert_at) {
                std::cmp::Ordering::Less => Some(Some(index)),
                std::cmp::Ordering::Equal => Some(None),
                std::cmp::Ordering::Greater => (index - 1 < cds.rows.len()).then_some(Some(index - 1)),
            }
        } else {
            (index < cds.rows.len()).then_some(Some(index))
        }
    }

    /// Informações da linha `index` (na contagem de `visible_count`).
    pub fn visible_row(&self, index: usize) -> Option<RowInfo<'a>> {
        let cds = self.cds().ok()?;
        let current = self.rec_no() == Some(index);
        Some(match self.physical(index)? {
            None => RowInfo { status: None, error: None, current: true, id: None },
            Some(i) => RowInfo { status: Some(cds.rows[i].status), error: cds.rows[i].error.as_deref().map(String::as_str), current, id: Some(cds.rows[i].id) },
        })
    }

    /// Valor de um campo na linha `index` (na contagem de `visible_count`); a linha em
    /// edição ou inclusão mostra o que está sendo digitado.
    pub fn visible_value(&self, index: usize, name: &str) -> DataResult<Value> {
        match self.physical(index) {
            Some(None) => self.value(name),
            Some(Some(i)) => self.value_at(i, name),
            None => Ok(Value::Null),
        }
    }

    /// Texto de exibição de um campo na linha `index`.
    pub fn visible_text(&self, index: usize, name: &str) -> DataResult<String> {
        self.display_text(name, &self.visible_value(index, name)?)
    }

    /// Quantas alterações ainda não foram aceitas pelo backend.
    pub fn change_count(&self) -> usize {
        self.cds().map_or(0, |c| c.rows.iter().chain(c.hidden.iter()).filter(|r| r.status != RowStatus::Unmodified).count() + c.deleted.len())
    }

    /// As alterações pendentes, prontas para o backend.
    pub fn delta(&self) -> DataResult<Vec<Change>> {
        let cds = self.cds()?;
        let names: Vec<&str> = cds.columns.iter().map(|c| c.name.as_str()).collect();
        // Só os campos de dados vão para o backend (calculados e lookup não).
        let data: Vec<usize> = (0..names.len()).filter(|&i| cds.columns[i].kind == FieldKind::Data).collect();
        let keys: Vec<usize> = if cds.key_fields.trim().is_empty() {
            data.clone()
        } else {
            cds.key_fields
                .split(';')
                .map(|k| cds.column(k.trim()).ok_or_else(|| DataError::new(format!("campo-chave '{}' não existe", k.trim()))))
                .collect::<DataResult<_>>()?
        };
        let pick = |values: &[Value], idx: &mut dyn Iterator<Item = usize>| -> Vec<(String, Value)> {
            idx.map(|i| (names[i].to_string(), values[i].clone())).collect()
        };
        let data_set = self.ui.name(self.id).unwrap_or_default().to_string();
        let change = |reference: u64, kind: ChangeKind, key, old, new| Change { reference, kind, data_set: data_set.clone(), key, old, new, master: None };
        let mut changes = Vec::new();
        let mut rows: Vec<&Row> = cds.rows.iter().chain(cds.hidden.iter()).collect();
        rows.sort_by_key(|r| r.id);
        for row in rows {
            if row.status == RowStatus::Unmodified {
                continue;
            }
            let values = cds.row_values(row);
            match row.status {
                RowStatus::Unmodified => {}
                RowStatus::Inserted => changes.push(change(
                    row.id,
                    ChangeKind::Insert,
                    pick(&values, &mut keys.iter().copied()),
                    Vec::new(),
                    pick(&values, &mut data.iter().copied()),
                )),
                RowStatus::Modified => {
                    let original = row.original.as_deref().unwrap_or(&values);
                    let changed: Vec<usize> = data.iter().copied().filter(|&i| original[i] != values[i]).collect();
                    if changed.is_empty() {
                        continue;
                    }
                    changes.push(change(
                        row.id,
                        ChangeKind::Update,
                        pick(original, &mut keys.iter().copied()),
                        pick(original, &mut changed.iter().copied()),
                        pick(&values, &mut changed.iter().copied()),
                    ));
                }
            }
        }
        for row in &cds.deleted {
            let values = cds.row_values(row);
            changes.push(change(
                row.id,
                ChangeKind::Delete,
                pick(&values, &mut keys.iter().copied()),
                pick(&values, &mut data.iter().copied()),
                Vec::new(),
            ));
        }
        Ok(changes)
    }

    /// Os detalhes ligados a este dataset (`master_source` num `DataSource` dele), em ordem de
    /// criação; `all` = também os de `apply_separately`.
    fn details(&self, all: bool) -> Vec<ComponentId> {
        let ui: &Ui = self.ui;
        let mut out: Vec<ComponentId> = ui
            .referrers(self.id)
            .into_iter()
            .filter(|&(c, prop)| prop == "data_set" && ui.get::<DataSource>(c).is_some())
            .flat_map(|(source, _)| ui.referrers(source))
            .filter(|&(_, prop)| prop == "master_source")
            .map(|(c, _)| c)
            .filter(|&c| ui.get::<ClientDataSet>(c).is_some_and(|d| all || !d.apply_separately))
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// O delta que vai ao backend no `apply_updates`: o deste dataset com os dos detalhes
    /// (os que não têm `apply_separately`, e os detalhes deles), na ordem segura para gravar:
    /// as inclusões e alterações do mestre, depois tudo dos detalhes, e por último as exclusões
    /// do mestre (os itens saem antes do pedido; o pedido entra antes dos itens). O detalhe
    /// incluído sob um mestre também incluído leva o [`MasterLink`].
    pub fn delta_with_details(&self) -> DataResult<Vec<Change>> {
        let own = self.delta()?;
        let details = self.details(false);
        if details.is_empty() {
            return Ok(own);
        }
        let (deletes, mut out): (Vec<Change>, Vec<Change>) = own.into_iter().partition(|c| c.kind == ChangeKind::Delete);
        let name = self.ui.name(self.id).unwrap_or_default().to_string();
        for detail in details {
            let pairs = view(self.ui, detail).cds().map(|c| master_pairs(&c.master_fields)).unwrap_or_default();
            let detail_name = self.ui.name(detail).unwrap_or_default().to_string();
            for mut change in view(self.ui, detail).delta_with_details()? {
                if change.data_set == detail_name && change.kind != ChangeKind::Delete && !pairs.is_empty() {
                    change.master = master_of(&out, &name, &change, &pairs);
                }
                out.push(change);
            }
        }
        out.extend(deletes);
        Ok(out)
    }

    /// O dataset (este ou um detalhe, em qualquer nível) com o registro `reference`.
    pub(crate) fn owner_of(&self, reference: u64) -> Option<ComponentId> {
        let cds = self.cds().ok()?;
        if cds.rows.iter().chain(cds.hidden.iter()).chain(cds.deleted.iter()).any(|r| r.id == reference) {
            return Some(self.id);
        }
        self.details(true).into_iter().find_map(|d| view(self.ui, d).owner_of(reference))
    }
}

/// A inclusão do mestre (em `changes`) cujos campos de ligação têm os valores que o detalhe
/// `change` recebeu: o mestre de que ele depende.
fn master_of(changes: &[Change], master: &str, change: &Change, pairs: &[(String, String)]) -> Option<MasterLink> {
    let value_of = |list: &[(String, Value)], name: &str| list.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone());
    let wanted: Vec<Value> = pairs.iter().map(|(detail, _)| value_of(&change.new, detail)).collect::<Option<_>>()?;
    if wanted.iter().any(|v| *v == Value::Null) {
        return None;
    }
    let found = changes.iter().find(|m| {
        m.kind == ChangeKind::Insert && pairs.iter().zip(&wanted).all(|((_, field), v)| value_of(&m.new, field).as_ref() == Some(v))
    })?;
    Some(MasterLink { data_set: master.to_string(), reference: found.reference, fields: pairs.to_vec() })
}

// ===========================================================================

/// Leitura e escrita do dataset. Criado por [`data_set`].
pub struct DataSet<'a> {
    ui: &'a mut Ui,
    id: ComponentId,
}

impl DataSet<'_> {
    pub fn view(&self) -> DataView<'_> {
        DataView { ui: self.ui, id: self.id }
    }

    pub fn ui(&mut self) -> &mut Ui {
        self.ui
    }

    fn cds_mut(&mut self) -> DataResult<&mut ClientDataSet> {
        self.ui
            .get_mut::<ClientDataSet>(self.id)
            .ok_or_else(|| DataError::new("o componente não é um ClientDataSet (ou já foi liberado)"))
    }

    // ----- leitura (atalhos para a view) -----

    pub fn state(&self) -> DataSetState {
        self.view().state()
    }

    pub fn is_active(&self) -> bool {
        self.view().is_active()
    }

    pub fn record_count(&self) -> usize {
        self.view().record_count()
    }

    pub fn rec_no(&self) -> Option<usize> {
        self.view().rec_no()
    }

    pub fn bof(&self) -> bool {
        self.view().bof()
    }

    pub fn eof(&self) -> bool {
        self.view().eof()
    }

    pub fn value(&self, field: &str) -> DataResult<Value> {
        self.view().value(field)
    }

    pub fn text(&self, field: &str) -> DataResult<String> {
        self.view().text(field)
    }

    pub fn change_count(&self) -> usize {
        self.view().change_count()
    }

    pub fn delta(&self) -> DataResult<Vec<Change>> {
        self.view().delta()
    }

    /// O delta com o dos detalhes, como vai no `apply_updates` (veja
    /// [`DataView::delta_with_details`]).
    pub fn delta_with_details(&self) -> DataResult<Vec<Change>> {
        self.view().delta_with_details()
    }

    // ----- avisos -----

    /// Avisa os controles ligados e dispara os eventos dos `DataSource`.
    pub(crate) fn notify(&mut self, event: DataEvent) {
        if let Ok(cds) = self.cds_mut() {
            if cds.disabled > 0 {
                cds.pending_notify = true;
                return;
            }
        }
        // Só os ligados, pelo índice de referências do `Ui`: antes,
        // cada registro que mudava percorria o app inteiro duas vezes.
        let ui: &Ui = self.ui;
        let sources: Vec<ComponentId> = ui
            .referrers(self.id)
            .into_iter()
            .filter(|&(c, prop)| prop == "data_set" && ui.get::<DataSource>(c).is_some_and(|s| s.enabled))
            .map(|(c, _)| c)
            .collect();
        if sources.is_empty() {
            return;
        }
        let links: Vec<ComponentId> =
            sources.iter().flat_map(|&s| ui.referrers(s)).filter(|&(_, prop)| prop == "data_source").map(|(c, _)| c).collect();
        let message = DataMessage { data_set: self.id, event: event.clone() };
        for link in links {
            self.ui.send_message(link, &message);
        }
        // As propriedades ligadas a campos ("Ligar a campo…").
        crate::binding::notify(self.ui, &sources, &event);
        if event != DataEvent::UpdateData {
            let name = if event == DataEvent::StateChanged { "on_state_change" } else { "on_data_change" };
            for source in sources {
                self.ui.fire(source, name, &EventArgs::None);
            }
        }
        self.update_details(&event);
    }

    /// Avisa os controles de novo (por exemplo depois de ligar um controle a um dataset
    /// que já estava aberto).
    pub fn refresh_links(&mut self) {
        self.notify(DataEvent::DataSetChanged);
    }

    // ----- filtro, índice e master-detail -----

    fn current_row_id(&self) -> Option<u64> {
        let cds = self.view().cds().ok()?;
        cds.rows.get(cds.cursor).map(|r| r.id)
    }

    /// Liga o filtro (`cidade = 'São Paulo' and limite > 1000`); texto vazio desliga. Com
    /// parâmetros (`limite > :minimo`), dê os valores com [`set_param`](Self::set_param):
    /// um parâmetro sem valor dá erro aqui, mas o filtro fica (sem valor, nada passa).
    pub fn set_filter(&mut self, filter: &str) -> DataResult {
        let cds = self.cds_mut()?;
        cds.filter = filter.to_string();
        cds.filtered = !filter.trim().is_empty();
        self.refilter()?;
        let missing: Vec<String> = Expr::parse(filter).map(|e| e.params()).unwrap_or_default().into_iter().filter(|p| self.view().cds().is_ok_and(|c| c.param(p).is_none())).collect();
        if let Some(name) = missing.first() {
            return self.fail(DataError::new(format!("filtro: o parâmetro :{name} não tem valor (set_param ou params)")));
        }
        self.server_query("filter", Vec::new())
    }

    /// Ordena (`cidade;limite desc`); texto vazio volta à ordem de chegada. Deixa de usar
    /// o índice nomeado. Com mais registros no backend, pede a ordem a ele
    /// ([`server_query`](Self::server_query)).
    pub fn set_index(&mut self, index_field_names: &str) -> DataResult {
        let cds = self.cds_mut()?;
        cds.index_field_names = index_field_names.to_string();
        cds.index_name.clear();
        self.refilter()?;
        self.server_query("sort", Vec::new())
    }

    /// Ordenar, filtrar ou localizar com dados parciais (o backend tem mais do que os
    /// carregados): a operação local valeria só para o que já chegou, então o dataset dispara
    /// `on_server_query` com o texto JSON `{"kind": "sort" | "filter" | "locate", "index":
    /// ordem, "filter": filtro, "loaded": carregados}` (mais `field` e `value` no "locate"),
    /// e o backend recarrega a consulta na ordem e no filtro pedidos (`load_rows` e
    /// `set_has_more`). Sem quem trate, nada muda nos dados e o aviso fica em
    /// `partial_notice` (a grade mostra). O que o backend fizer no dataset durante o evento não
    /// pergunta de novo.
    pub fn server_query(&mut self, kind: &str, extra: Vec<(&str, serde_json::Value)>) -> DataResult {
        let cds = self.view().cds()?;
        if cds.querying {
            return Ok(());
        }
        if !cds.has_more {
            self.cds_mut()?.partial_notice.clear();
            return Ok(());
        }
        let loaded = cds.rows.len();
        let mut query = serde_json::json!({ "kind": kind, "index": cds.effective_index().unwrap_or_default(), "filter": if cds.filtered { cds.filter.clone() } else { String::new() }, "loaded": loaded });
        for (k, v) in extra {
            query[k] = v;
        }
        self.cds_mut()?.querying = true;
        let handled = self.ui.fire(self.id, "on_server_query", &EventArgs::Text(query.to_string()));
        let cds = self.cds_mut()?;
        cds.querying = false;
        cds.partial_notice = if handled {
            String::new()
        } else {
            let template = match kind {
                "sort" => "Ordenado só entre os {0} registros carregados: há mais no servidor",
                "filter" => "Filtrado só entre os {0} registros carregados: há mais no servidor",
                _ => "Procurado só entre os {0} registros carregados: há mais no servidor",
            };
            tf(template, &[&loaded])
        };
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// O aviso de que a última ordem, filtro ou procura valeu só para os registros carregados
    /// (vazio: nada a avisar). Veja [`server_query`](Self::server_query).
    pub fn partial_notice(&self) -> String {
        self.view().cds().map(|c| c.partial_notice.clone()).unwrap_or_default()
    }

    /// Usa um índice de `indexes` pelo nome; vazio volta ao
    /// `index_field_names`.
    pub fn set_index_name(&mut self, name: &str) -> DataResult {
        let cds = self.cds_mut()?;
        let before = std::mem::replace(&mut cds.index_name, name.trim().to_string());
        if let Err(e) = cds.effective_index() {
            cds.index_name = before;
            return self.fail(DataError::new(e));
        }
        self.refilter()?;
        self.server_query("sort", Vec::new())
    }

    /// Valor de um parâmetro do filtro (`:minimo` em `cidade = :cidade and valor > :minimo`).
    pub fn set_param(&mut self, name: &str, value: impl Into<Value>) -> DataResult {
        let name = name.trim().trim_start_matches(':').to_string();
        let value = value.into();
        let cds = self.cds_mut()?;
        match cds.param_values.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(&name)) {
            Some(slot) => slot.1 = value,
            None => cds.param_values.push((name, value)),
        }
        self.refilter()
    }

    /// Só os registros nestas situações (`StatusFilter`); vazio = todos.
    pub fn set_status_filter(&mut self, statuses: &[RowStatus]) -> DataResult {
        let names: Vec<&str> = statuses
            .iter()
            .map(|s| match s {
                RowStatus::Unmodified => "unmodified",
                RowStatus::Modified => "modified",
                RowStatus::Inserted => "inserted",
            })
            .collect();
        self.cds_mut()?.status_filter = names.join(";");
        self.refilter()
    }

    /// Desfaz as alterações do registro atual ainda não aplicadas:
    /// o alterado volta aos valores originais; o incluído sai. Em edição, cancela.
    pub fn revert_record(&mut self) -> DataResult {
        self.ensure_active()?;
        if self.state().is_editing() {
            return self.cancel();
        }
        let cds = self.cds_mut()?;
        let Some(row) = cds.rows.get_mut(cds.cursor) else { return Ok(()) };
        match row.status {
            RowStatus::Unmodified => return Ok(()),
            RowStatus::Modified => {
                if let Some(original) = row.original.take() {
                    row.data = RowData::Owned(original);
                }
                row.status = RowStatus::Unmodified;
                row.error = None;
            }
            RowStatus::Inserted => {
                cds.rows.remove(cds.cursor);
                cds.cursor = cds.cursor.min(cds.rows.len().saturating_sub(1));
                cds.eof = cds.rows.is_empty();
            }
        }
        let keep = self.current_row_id();
        self.rebuild_view(keep)?;
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Traz de volta um registro excluído e ainda não aplicado (o `id` vem de
    /// `deleted_rows()`), com os valores originais. `false` se não está entre os excluídos.
    pub fn restore_deleted(&mut self, id: u64) -> DataResult<bool> {
        let cds = self.cds_mut()?;
        let Some(at) = cds.deleted.iter().position(|r| r.id == id) else { return Ok(false) };
        let mut row = cds.deleted.remove(at);
        row.status = RowStatus::Unmodified;
        row.error = None;
        cds.rows.push(row);
        self.rebuild_view(Some(id))?;
        let cds = self.cds_mut()?;
        if let Some(i) = cds.rows.iter().position(|r| r.id == id) {
            cds.cursor = i;
        }
        self.notify(DataEvent::DataSetChanged);
        Ok(true)
    }

    /// Filtro em Rust, somado ao `filter`: `get("cidade")` lê
    /// um campo do registro.
    pub fn on_filter_record(&mut self, f: impl Fn(&dyn Fn(&str) -> Value) -> bool + 'static) -> DataResult {
        self.cds_mut()?.hooks.filter_record = Some(Box::new(f));
        self.refilter()
    }

    /// Refaz a visão (filtro, mestre e índice) mantendo o registro atual, e avisa os
    /// controles. Chamado sozinho quando essas propriedades mudam.
    pub fn refilter(&mut self) -> DataResult {
        if !self.is_active() {
            return Ok(());
        }
        if self.state().is_editing() {
            // Refaz depois do post/cancel.
            self.cds_mut()?.needs_refilter = true;
            return Ok(());
        }
        let keep = self.current_row_id();
        self.rebuild_view(keep)?;
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Valores que os campos do detalhe recebem do registro atual do mestre:
    /// (coluna daqui, valor). `None` sem mestre; `Some(vazio)`... nunca: sem registro no
    /// mestre devolve `Some` com valores nulos.
    fn master_values(&self) -> DataResult<Option<Vec<(usize, Value)>>> {
        let cds = self.view().cds()?;
        let Some(source) = cds.master_source else { return Ok(None) };
        let pairs = master_pairs(&cds.master_fields);
        if pairs.is_empty() {
            return Ok(None);
        }
        let master = self.ui.get::<DataSource>(source).and_then(|s| s.data_set);
        let mut out = Vec::new();
        for (detail, master_field) in pairs {
            let col = cds.column(&detail).ok_or_else(|| DataError::new(format!("master_fields: campo '{detail}' não existe")))?;
            let value = match master {
                Some(m) if view(self.ui, m).is_active() && !view(self.ui, m).is_empty() => view(self.ui, m).value(&master_field)?,
                _ => Value::Null,
            };
            let field = self.ui.get::<Field>(cds.columns[col].field).ok_or("campo liberado")?;
            out.push((col, field.coerce(value).unwrap_or_default()));
        }
        Ok(Some(out))
    }

    /// Separa os registros visíveis (filtro + mestre), ordena pelo índice e põe o cursor
    /// no registro `keep` (ou no primeiro).
    fn rebuild_view(&mut self, keep: Option<u64>) -> DataResult {
        let _profile = aylis_core::profile::scope(aylis_core::profile::Lane::Data, || "filtro e ordem".into());
        let cds = self.view().cds()?;
        let index_spec_text = match cds.effective_index() {
            Ok(s) => s,
            Err(e) => {
                self.cds_mut()?.needs_refilter = false;
                return self.fail(DataError::new(e));
            }
        };
        let statuses = cds.status_allowed();
        let has_rules = (cds.filtered && !cds.filter.trim().is_empty())
            || cds.hooks.filter_record.is_some()
            || cds.master_source.is_some()
            || !index_spec_text.trim().is_empty()
            || statuses.is_some()
            || !cds.hidden.is_empty();
        if !has_rules {
            self.cds_mut()?.needs_refilter = false;
            return Ok(());
        }
        let filter = if cds.filtered && !cds.filter.trim().is_empty() {
            match Expr::parse(&cds.filter) {
                Ok(e) => Some(e),
                Err(e) => {
                    self.cds_mut()?.needs_refilter = false;
                    return self.fail(DataError::new(format!("filtro inválido: {e}")));
                }
            }
        } else {
            None
        };
        // Os parâmetros do filtro, já com os valores; um ainda sem valor vale nulo (nada
        // passa) até ser dado (o `set_filter` avisa).
        let params: Vec<(String, Value)> =
            filter.as_ref().map(Expr::params).unwrap_or_default().into_iter().map(|name| (format!(":{name}"), cds.param(&name).unwrap_or_default())).collect();
        let index = match index_spec(&cds.columns, &index_spec_text) {
            Ok(i) => i,
            Err(e) => {
                self.cds_mut()?.needs_refilter = false;
                return self.fail(DataError::new(e));
            }
        };
        let master = self.master_values()?;
        let hook = self.cds_mut()?.hooks.filter_record.take();
        let cds = self.cds_mut()?;
        let mut all: Vec<Row> = std::mem::take(&mut cds.rows);
        all.append(&mut cds.hidden);
        // Sem índice, a ordem é a de chegada (o id); com índice, ela desempata.
        if index.is_empty() {
            all.sort_unstable_by_key(|r| r.id);
        }
        let columns = cds.columns.clone();
        let lookup = cds.column_index.clone();
        let store = std::mem::take(&mut cds.store);
        let ui: &Ui = self.ui;
        // Um campo pelo nome (ou um parâmetro `:nome`), lido por `cell`.
        let named = |name: &str, values: &Src<'_>, cell: &dyn Fn(&Src<'_>, usize) -> Value| -> Option<Value> {
            if name.starts_with(':') {
                return params.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone());
            }
            let at = lookup.get(name).copied().or_else(|| lookup.get(&name.to_ascii_lowercase()).copied());
            at.map(|i| cell(values, i))
        };
        // Situação, mestre e filtro (o `on_filter_record` fica à parte: precisa do `Ui`).
        let check = |row: &Row, cell: &dyn Fn(&Src<'_>, usize) -> Value| -> bool {
            let values = src(&store, row);
            if let Some(allowed) = &statuses {
                if !allowed.contains(&row.status) {
                    return false;
                }
            }
            if let Some(m) = &master {
                if m.iter().any(|(col, v)| v.is_null() || crate::expr::compare(&values.get(*col), v) != Some(std::cmp::Ordering::Equal)) {
                    return false;
                }
            }
            match &filter {
                Some(f) => f.matches(&|name| named(name, &values, cell)),
                None => true,
            }
        };
        // Só campos de dados e sem `on_filter_record`: cada registro se decide sem o `Ui`,
        // dividido entre os núcleos.
        let plain = columns.iter().all(|c| c.kind == FieldKind::Data);
        let mask: Vec<bool> = if plain && hook.is_none() {
            crate::parallel::map(&all, |r| check(r, &|v: &Src<'_>, i| v.get(i)))
        } else {
            let cell = |v: &Src<'_>, i: usize| cell_value(ui, &columns, v, i, 0);
            all.iter()
                .map(|r| {
                    check(r, &cell)
                        && match &hook {
                            Some(h) => {
                                let values = src(&store, r);
                                h(&|name| named(name, &values, &cell).unwrap_or_default())
                            }
                            None => true,
                        }
                })
                .collect()
        };
        let mut visible = Vec::with_capacity(mask.iter().filter(|&&m| m).count());
        let mut hidden = Vec::new();
        for (row, keep) in all.into_iter().zip(mask) {
            if keep { visible.push(row) } else { hidden.push(row) }
        }
        if !index.is_empty() {
            // As chaves uma vez por registro (números e posições no dicionário: comparar
            // não copia textos), depois a ordem das posições, desempatada pela de chegada.
            let n = index.len();
            let ranks: Vec<Option<Vec<u32>>> = index.iter().map(|&(c, _)| store.text_ranks(c)).collect();
            let key_of = |r: &Row, k: usize, c: usize| match &r.data {
                RowData::Stored(at) => store.sort_key(*at, c, ranks[k].as_deref()),
                RowData::Owned(v) => SortKey::of(v.get(c).cloned().unwrap_or_default()),
            };
            let keys: Vec<SortKey> = if index.iter().all(|&(c, _)| columns[c].kind == FieldKind::Data) {
                if n == 1 {
                    crate::parallel::map(&visible, |r| key_of(r, 0, index[0].0))
                } else {
                    crate::parallel::map(&visible, |r| index.iter().enumerate().map(|(k, &(c, _))| key_of(r, k, c)).collect::<Vec<_>>()).into_iter().flatten().collect()
                }
            } else {
                visible.iter().flat_map(|r| index.iter().map(|&(c, _)| SortKey::of(cell_value(ui, &columns, &src(&store, r), c, 0))).collect::<Vec<_>>()).collect()
            };
            let ids: Vec<u64> = visible.iter().map(|r| r.id).collect();
            // Uma coluna só, com chaves da mesma família (o clique no título do grid): cada
            // registro vira (nulo, número, id, posição), ordenados direto, sem ir buscar as
            // chaves a cada comparação.
            let packed: Option<Vec<(u8, u64)>> = (n == 1).then(|| keys.iter().map(SortKey::packed).collect()).flatten();
            let family = packed.as_ref().and_then(|p| {
                let mut families = p.iter().map(|k| k.0).filter(|&f| f != 0);
                let first = families.next().unwrap_or(1);
                families.all(|f| f == first).then_some(first)
            });
            let order: Vec<u32> = match (&packed, family) {
                (Some(packed), Some(_)) => {
                    let desc = index[0].1;
                    let mut tuples: Vec<(u8, u64, u64, u32)> = packed
                        .iter()
                        .zip(&ids)
                        .enumerate()
                        .map(|(i, (&(f, v), &id))| match (f == 0, desc) {
                            (null, false) => (u8::from(!null), v, id, i as u32),
                            (null, true) => (u8::from(null), !v, id, i as u32),
                        })
                        .collect();
                    crate::parallel::sort(&mut tuples, |a, b| a.cmp(b));
                    tuples.into_iter().map(|t| t.3).collect()
                }
                _ => {
                    let mut order: Vec<u32> = (0..visible.len() as u32).collect();
                    crate::parallel::sort(&mut order, |&a, &b| {
                        for (k, &(c, desc)) in index.iter().enumerate() {
                            let o = keys[a as usize * n + k].order(&keys[b as usize * n + k], &store, c);
                            let o = if desc { o.reverse() } else { o };
                            if o != std::cmp::Ordering::Equal {
                                return o;
                            }
                        }
                        ids[a as usize].cmp(&ids[b as usize])
                    });
                    order
                }
            };
            drop(keys);
            let mut slots: Vec<Option<Row>> = visible.into_iter().map(Some).collect();
            visible = order.iter().filter_map(|&i| slots[i as usize].take()).collect();
        }
        let cds = self.cds_mut()?;
        cds.store = store;
        cds.cursor = keep.and_then(|id| visible.iter().position(|r| r.id == id)).unwrap_or(0);
        cds.rows = visible;
        cds.hidden = hidden;
        cds.hooks.filter_record = hook;
        cds.needs_refilter = false;
        cds.bof = cds.cursor == 0;
        cds.eof = cds.rows.is_empty();
        Ok(())
    }

    /// Detalhes ligados a este dataset (como mestre).
    fn details(&self) -> Vec<ComponentId> {
        // Os DataSource deste dataset e, deles, os datasets que os têm como `master_source`.
        let ui: &Ui = self.ui;
        ui.referrers(self.id)
            .into_iter()
            .filter(|&(s, prop)| prop == "data_set" && ui.get::<DataSource>(s).is_some())
            .flat_map(|(s, _)| ui.referrers(s))
            .filter(|&(c, prop)| prop == "master_source" && ui.get::<ClientDataSet>(c).is_some())
            .map(|(c, _)| c)
            .collect()
    }

    /// O mestre mudou de registro (ou de dados): cada detalhe grava a edição pendente,
    /// refaz a visão e dispara `on_master_change` (com os valores do mestre em JSON, para
    /// um backend carregar os detalhes sob demanda).
    fn update_details(&mut self, event: &DataEvent) {
        let relevant = match event {
            DataEvent::Scrolled | DataEvent::DataSetChanged | DataEvent::ActiveChanged => true,
            DataEvent::RecordChanged(Some(field)) => {
                let ui: &Ui = self.ui;
                self.details().iter().any(|&d| {
                    ui.get::<ClientDataSet>(d).is_some_and(|c| master_pairs(&c.master_fields).iter().any(|(_, m)| m.eq_ignore_ascii_case(field)))
                })
            }
            _ => false,
        };
        if !relevant {
            return;
        }
        for detail in self.details() {
            let mut ds = DataSet { ui: self.ui, id: detail };
            if !ds.is_active() {
                continue;
            }
            if ds.state().is_editing() {
                let _ = ds.post();
            }
            let _ = ds.refilter();
            if let Ok(Some(values)) = ds.master_values() {
                let names: Vec<String> = ds.view().field_names();
                let map: serde_json::Map<String, serde_json::Value> =
                    values.iter().map(|(c, v)| (names[*c].clone(), crate::json::value_to_json(v))).collect();
                let text = serde_json::Value::Object(map).to_string();
                ds.ui.fire(detail, "on_master_change", &EventArgs::Text(text));
            }
        }
    }

    fn state_changed(&mut self) {
        self.notify(DataEvent::StateChanged);
        self.ui.fire(self.id, "on_state_change", &EventArgs::None);
    }

    /// Dispara `on_error` e devolve o erro.
    fn fail<T>(&mut self, error: DataError) -> DataResult<T> {
        self.ui.fire(self.id, "on_error", &EventArgs::Text(error.message.clone()));
        Err(error)
    }

    /// O `before_post` ou o `before_delete` por nome (eventos-pergunta): `Ok(true)` segue
    /// agora; `Ok(false)` o backend responde depois (o registro fica como está e a operação
    /// roda de novo com a resposta, sem perguntar outra vez); recusado, é o erro, com a
    /// mensagem na tela.
    fn ask_before(&mut self, what: Asking) -> DataResult<bool> {
        let cds = self.cds_mut()?;
        if cds.answered == Some(what) {
            cds.answered = None;
            return Ok(true);
        }
        // Já esperando a resposta: um segundo `post` não pergunta de novo.
        if cds.asking.is_some() {
            return Ok(false);
        }
        let event = match what {
            Asking::Post => "before_post",
            Asking::Delete => "before_delete",
        };
        match self.ui.ask(self.id, event, &EventArgs::None) {
            Asked::Answered(Answer::Refused(message)) => self.refused(event, message).map(|_| false),
            Asked::Answered(_) => Ok(true),
            Asked::Deferred(token) => {
                self.cds_mut()?.asking = Some(what);
                self.state_changed();
                let id = self.id;
                self.ui.on_answer(token, move |ui, answer| {
                    let mut ds = data_set(ui, id);
                    let Ok(cds) = ds.cds_mut() else { return };
                    cds.asking = None;
                    match answer {
                        Answer::Refused(message) => {
                            let _ = ds.refused(event, message);
                            ds.state_changed();
                        }
                        Answer::Accepted(_) => {
                            if let Ok(cds) = ds.cds_mut() {
                                cds.answered = Some(what);
                            }
                            let _ = match what {
                                Asking::Post => ds.post(),
                                Asking::Delete => ds.delete(),
                            };
                            if let Ok(cds) = ds.cds_mut() {
                                cds.answered = None;
                            }
                        }
                    }
                });
                Ok(false)
            }
        }
    }

    /// O `before_post`/`before_delete` recusou: a mensagem vai para a tela (a caixa de erro)
    /// e para o `on_error`; o registro continua em edição.
    fn refused(&mut self, event: &str, message: String) -> DataResult {
        if !message.is_empty() {
            let path = self.ui.path_of(self.id);
            self.ui.raise_exception(aylis_core::Exception { message: message.clone(), path, event: event.to_string() });
        }
        self.fail(DataError::new(message))
    }

    /// O `before_post` ou o `before_delete` esperam a resposta do backend (os controles DB*
    /// mostram que está gravando e não deixam mexer).
    pub fn is_asking(&self) -> bool {
        self.view().cds().is_ok_and(|c| c.asking.is_some() || c.applying)
    }

    /// O `on_apply_updates` remoto ainda não respondeu (os controles DB* mostram que está gravando).
    pub fn is_applying(&self) -> bool {
        self.view().cds().is_ok_and(|c| c.applying)
    }

    fn run_hook(&mut self, pick: fn(&mut Hooks) -> &mut Option<Box<DataSetHook>>) -> Result<(), String> {
        let Ok(cds) = self.cds_mut() else { return Ok(()) };
        let Some(mut hook) = pick(&mut cds.hooks).take() else { return Ok(()) };
        let result = hook(self.ui, self.id);
        if let Ok(cds) = self.cds_mut() {
            pick(&mut cds.hooks).get_or_insert(hook);
        }
        result
    }

    // ----- abrir e fechar -----

    /// Abre o dataset: as colunas são os componentes `Field` filhos.
    pub fn open(&mut self) -> DataResult {
        if self.is_active() {
            return Ok(());
        }
        let _profile = aylis_core::profile::scope(aylis_core::profile::Lane::Data, || "abrir dataset".into());
        let mut columns = Vec::new();
        for child in self.ui.children(self.id).to_vec() {
            if let Some(field) = self.ui.get::<Field>(child) {
                let name = if field.field_name.is_empty() { self.ui.name(child).unwrap_or("").to_string() } else { field.field_name.clone() };
                if name.is_empty() {
                    return self.fail(DataError::new("todo Field precisa de field_name (ou de um nome)"));
                }
                columns.push(Column { field: child, name, kind: field.kind });
            }
        }
        if columns.is_empty() {
            return self.fail(DataError::new("o ClientDataSet não tem campos: adicione componentes Field"));
        }
        let cds = self.cds_mut()?;
        if cds.store.width() != columns.len() {
            cds.rows.clear();
            cds.hidden.clear();
            cds.store = ColumnStore::new(columns.len());
        }
        cds.set_columns(columns);
        cds.state = DataSetState::Browse;
        cds.wants_open = false;
        cds.cursor = 0;
        cds.bof = true;
        cds.eof = cds.rows.is_empty();
        cds.buffer = None;
        self.notify(DataEvent::ActiveChanged);
        Ok(())
    }

    pub fn close(&mut self) -> DataResult {
        let cds = self.cds_mut()?;
        cds.state = DataSetState::Inactive;
        cds.buffer = None;
        cds.wants_open = false;
        self.notify(DataEvent::ActiveChanged);
        Ok(())
    }

    fn ensure_active(&mut self) -> DataResult {
        if self.is_active() { Ok(()) } else { self.fail(DataError::new("o dataset está fechado")) }
    }

    /// Antes de navegar ou inserir: grava a edição em andamento (como `CheckBrowseMode`).
    fn check_browse(&mut self) -> DataResult {
        if !self.state().is_editing() {
            return Ok(());
        }
        self.post()?;
        // A gravação espera o backend (o `before_post` remoto): a operação não segue por cima
        // da edição; quem chamou refaz quando a resposta chegar.
        if self.state().is_editing() && (self.is_asking() || self.awaiting_text()) {
            return Err(DataError::waiting());
        }
        Ok(())
    }

    /// Substitui os dados (como chegam do backend). Abre o dataset se preciso. Cada
    /// linha tem um valor por campo, na ordem dos `Field`.
    pub fn load_rows(&mut self, rows: Vec<Vec<Value>>) -> DataResult {
        self.open()?;
        let loaded = self.prepare_rows(rows)?;
        let cds = self.cds_mut()?;
        cds.hidden.clear();
        cds.store = ColumnStore::new(cds.columns.len());
        cds.rows = Vec::with_capacity(loaded.len());
        for values in loaded {
            let row_id = new_row_id();
            let at = cds.store.push(values);
            cds.rows.push(Row { id: row_id, data: RowData::Stored(at), status: RowStatus::Unmodified, original: None, error: None });
        }
        cds.deleted.clear();
        cds.buffer = None;
        cds.has_more = false;
        cds.fetching = false;
        cds.partial_notice.clear();
        cds.state = DataSetState::Browse;
        cds.cursor = 0;
        cds.bof = true;
        cds.eof = cds.rows.is_empty();
        self.rebuild_view(None)?;
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Substitui os dados por um lote montado fora da interface ([`RowBatch`](crate::RowBatch))
    /// de uma vez: sem conferir valor por valor (o lote já vem no tipo de cada campo) e
    /// com uma só reconstrução da visão (a ordem e o filtro escolhidos valem). O lote tem
    /// uma coluna por campo de dados (ou por campo), na ordem dos `Field`.
    pub fn load_batch(&mut self, batch: crate::RowBatch) -> DataResult {
        self.open()?;
        let data: Vec<bool> = self.cds_mut()?.columns.iter().map(|c| c.kind == FieldKind::Data).collect();
        let data_count = data.iter().filter(|&&d| d).count();
        let store = batch.into_store();
        let store = if store.width() == data.len() {
            store
        } else if store.width() == data_count {
            store.spread(&data)
        } else {
            return self.fail(DataError::new(format!("o lote tem {} colunas para {data_count} campos", store.width())));
        };
        let cds = self.cds_mut()?;
        cds.hidden.clear();
        let n = store.len();
        let mut rows = Vec::with_capacity(n);
        for at in 0..n as u32 {
            let row_id = new_row_id();
            rows.push(Row { id: row_id, data: RowData::Stored(at), status: RowStatus::Unmodified, original: None, error: None });
        }
        cds.rows = rows;
        cds.store = store;
        cds.deleted.clear();
        cds.buffer = None;
        cds.has_more = false;
        cds.fetching = false;
        cds.partial_notice.clear();
        cds.state = DataSetState::Browse;
        cds.cursor = 0;
        cds.bof = true;
        cds.eof = cds.rows.is_empty();
        self.rebuild_view(None)?;
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Confere e converte as linhas que chegam do backend. Cada linha traz um valor por
    /// campo de dados (ou por campo, incluindo calculados e lookup, que são ignorados).
    fn prepare_rows(&mut self, rows: Vec<Vec<Value>>) -> DataResult<Vec<Vec<Value>>> {
        let columns: Vec<(ComponentId, FieldKind)> = self.cds_mut()?.columns.iter().map(|c| (c.field, c.kind)).collect();
        let data_count = columns.iter().filter(|(_, k)| *k == FieldKind::Data).count();
        // Os campos uma vez só (não a cada valor).
        let mut fields = Vec::with_capacity(columns.len());
        for &(field, kind) in &columns {
            fields.push((self.ui.get::<Field>(field).ok_or_else(|| DataError::new("campo liberado"))?, kind));
        }
        let mut problem = None;
        let mut loaded = Vec::with_capacity(rows.len());
        for (n, values) in rows.into_iter().enumerate() {
            let full = values.len() == columns.len();
            if !full && values.len() != data_count {
                return self.fail(DataError::new(format!("linha {n}: {} valores para {} campos", values.len(), data_count)));
            }
            let mut given = values.into_iter();
            let mut row = Vec::with_capacity(columns.len());
            for &(field, kind) in &fields {
                if kind != FieldKind::Data {
                    if full {
                        given.next();
                    }
                    row.push(Value::Null);
                    continue;
                }
                match field.coerce(given.next().unwrap_or_default()) {
                    Ok(v) => row.push(v),
                    Err(e) => {
                        problem = Some(format!("linha {n}: {e}"));
                        break;
                    }
                }
            }
            if problem.is_some() {
                break;
            }
            loaded.push(row);
        }
        match problem {
            Some(message) => self.fail(DataError::new(message)),
            None => Ok(loaded),
        }
    }

    /// Acrescenta registros no fim (o próximo pacote da busca sob demanda) sem mexer no
    /// cursor. `has_more` diz se o backend ainda tem mais.
    pub fn append_rows(&mut self, rows: Vec<Vec<Value>>, has_more: bool) -> DataResult {
        self.open()?;
        let loaded = self.prepare_rows(rows)?;
        let cds = self.cds_mut()?;
        let was_empty = cds.rows.is_empty();
        for values in loaded {
            let row_id = new_row_id();
            let at = cds.store.push(values);
            cds.rows.push(Row { id: row_id, data: RowData::Stored(at), status: RowStatus::Unmodified, original: None, error: None });
        }
        cds.has_more = has_more;
        cds.fetching = false;
        if !has_more {
            cds.partial_notice.clear();
        }
        if was_empty {
            cds.cursor = 0;
            cds.bof = true;
        }
        cds.eof = cds.rows.is_empty();
        let keep = if was_empty { None } else { self.current_row_id() };
        self.rebuild_view(keep)?;
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Diz se o backend ainda tem registros além dos carregados.
    pub fn set_has_more(&mut self, has_more: bool) -> DataResult {
        let cds = self.cds_mut()?;
        cds.has_more = has_more;
        cds.fetching = false;
        if !has_more {
            cds.partial_notice.clear();
        }
        Ok(())
    }

    /// Pede o próximo pacote ao backend (se houver mais): chama o tratador de
    /// [`on_fetch`](Self::on_fetch) ou dispara o evento `on_fetch` com quantos registros
    /// já estão carregados. O backend responde com [`append_rows`](Self::append_rows)
    /// (agora ou depois). Não pede de novo enquanto a resposta não chega.
    pub fn fetch_more(&mut self) -> DataResult<bool> {
        let cds = self.cds_mut()?;
        if !cds.has_more || cds.fetching {
            return Ok(false);
        }
        // O limite de memória: não busca mais, e a grade avisa.
        if cds.max_records > 0 && cds.rows.len() as i64 >= cds.max_records {
            cds.partial_notice = tf("Mostrando os primeiros {0} registros (limite de memória do dataset)", &[&cds.rows.len()]);
            return Ok(false);
        }
        cds.fetching = true;
        let loaded = cds.rows.len();
        let handler = cds.hooks.fetch.take();
        match handler {
            Some(mut handler) => {
                handler(self.ui, self.id, loaded);
                if let Ok(cds) = self.cds_mut() {
                    cds.hooks.fetch.get_or_insert(handler);
                }
            }
            None => {
                if !self.ui.fire(self.id, "on_fetch", &EventArgs::Value(PropValue::Int(loaded as i64))) {
                    self.cds_mut()?.fetching = false;
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Backend em Rust para a busca sob demanda: recebe quantos registros já estão
    /// carregados e responde com `append_rows`.
    pub fn on_fetch(&mut self, f: impl FnMut(&mut Ui, ComponentId, usize) + 'static) -> &mut Self {
        if let Ok(cds) = self.cds_mut() {
            cds.hooks.fetch = Some(Box::new(f));
        }
        self
    }

    // ----- navegação -----

    pub fn first(&mut self) -> DataResult {
        self.move_to(0)?;
        self.set_flags(true, self.record_count() == 0);
        Ok(())
    }

    pub fn last(&mut self) -> DataResult {
        self.move_to(self.record_count().saturating_sub(1))?;
        self.set_flags(self.record_count() == 0, true);
        Ok(())
    }

    // Move o cursor: não é um iterador.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> DataResult {
        let at_last = self.view().at_last();
        if at_last {
            self.check_browse()?;
            self.set_flags(false, true);
            return Ok(());
        }
        let to = self.view().cds()?.cursor + 1;
        self.move_to(to)
    }

    pub fn prior(&mut self) -> DataResult {
        let at_first = self.view().at_first();
        if at_first {
            self.check_browse()?;
            self.set_flags(true, false);
            return Ok(());
        }
        let to = self.view().cds()?.cursor - 1;
        self.move_to(to)
    }

    fn set_flags(&mut self, bof: bool, eof: bool) {
        if let Ok(cds) = self.cds_mut() {
            cds.bof = bof;
            cds.eof = eof;
        }
    }

    /// Vai para o registro `index` (0 = primeiro).
    pub fn move_to(&mut self, index: usize) -> DataResult {
        self.ensure_active()?;
        self.check_browse()?;
        let cds = self.cds_mut()?;
        if cds.rows.is_empty() {
            return Ok(());
        }
        let target = index.min(cds.rows.len() - 1);
        cds.bof = false;
        cds.eof = false;
        if target == cds.cursor {
            return Ok(());
        }
        cds.cursor = target;
        self.notify(DataEvent::Scrolled);
        if let Err(e) = self.run_hook(|h| &mut h.after_scroll) {
            return self.fail(DataError::new(e));
        }
        self.ui.fire(self.id, "after_scroll", &EventArgs::None);
        Ok(())
    }

    /// Vai para o registro `index` sem avisar ninguém (controles, eventos, `after_scroll`),
    /// refiltrando os detalhes (e os detalhes deles) do mesmo jeito. Para percorrer o mestre
    /// por dentro, como o relatório faz: as grades não rolam junto. Quem chama volta ao
    /// registro de antes com [`restore_browse`](Self::restore_browse), passando o que
    /// [`browse_mark`](Self::browse_mark) guardou antes de começar.
    pub fn browse_to(&mut self, index: usize) -> DataResult {
        self.ensure_active()?;
        self.check_browse()?;
        let cds = self.cds_mut()?;
        if cds.rows.is_empty() {
            return Ok(());
        }
        cds.cursor = index.min(cds.rows.len() - 1);
        self.rebuild_details_silently()
    }

    /// Refaz a vista dos detalhes (e dos detalhes deles) sem avisar ninguém.
    fn rebuild_details_silently(&mut self) -> DataResult {
        for detail in self.details() {
            let mut ds = DataSet { ui: self.ui, id: detail };
            if !ds.is_active() || ds.state().is_editing() {
                continue;
            }
            let keep = ds.current_row_id();
            ds.rebuild_view(keep)?;
            ds.rebuild_details_silently()?;
        }
        Ok(())
    }

    /// O registro atual deste dataset e dos detalhes dele, para [`restore_browse`](Self::restore_browse).
    pub fn browse_mark(&mut self) -> Vec<(ComponentId, Option<u64>)> {
        let mut out = vec![(self.id, self.current_row_id())];
        let mut i = 0;
        while i < out.len() {
            for detail in (DataSet { ui: &mut *self.ui, id: out[i].0 }).details() {
                let id = DataSet { ui: &mut *self.ui, id: detail }.current_row_id();
                out.push((detail, id));
            }
            i += 1;
        }
        out
    }

    /// Volta, sem avisar ninguém, aos registros que [`browse_mark`](Self::browse_mark)
    /// guardou: o mestre primeiro, depois cada detalhe já filtrado pelo registro dele.
    pub fn restore_browse(&mut self, mark: &[(ComponentId, Option<u64>)]) -> DataResult {
        for &(id, row) in mark {
            let mut ds = DataSet { ui: &mut *self.ui, id };
            if !ds.is_active() || ds.state().is_editing() {
                continue;
            }
            if id == self.id {
                let at = row.and_then(|r| ds.view().cds().ok()?.rows.iter().position(|x| x.id == r));
                if let Some(at) = at {
                    ds.cds_mut()?.cursor = at;
                }
            } else {
                ds.rebuild_view(row)?;
            }
        }
        Ok(())
    }

    /// Vai para o registro de identificador `id` (de [`RowInfo::id`]), como o
    /// `GotoBookmark`. `false` se ele não está entre os carregados (excluído, filtrado).
    pub fn goto_id(&mut self, id: u64) -> DataResult<bool> {
        let found = self.view().cds()?.rows.iter().position(|r| r.id == id);
        match found {
            Some(i) => self.move_to(i).map(|_| true),
            None => Ok(false),
        }
    }

    /// Procura o primeiro registro com `field = value` e vai até ele (como `Locate`). Não
    /// achou entre os carregados e o backend tem mais: pede a ele
    /// ([`server_query`](Self::server_query) com `"locate"`), que pode carregar o registro e
    /// posicionar; sem quem trate, avisa.
    pub fn locate(&mut self, field: &str, value: impl Into<Value>) -> DataResult<bool> {
        let (col, info) = self.view().column(field)?;
        let wanted = self.ui.get::<Field>(info.field).map(|f| f.coerce(value.into())).ok_or("campo liberado")??;
        let view = self.view();
        let cds = view.cds()?;
        let found = cds.rows.iter().position(|r| cell_value(view.ui, &cds.columns, &src(&cds.store, r), col, 0) == wanted);
        match found {
            Some(i) => self.move_to(i).map(|_| true),
            None => {
                self.server_query("locate", vec![("field", field.into()), ("value", crate::json::value_to_json(&wanted))])?;
                Ok(false)
            }
        }
    }

    // ----- edição -----

    /// Coloca o registro atual em edição (num dataset vazio, começa uma inclusão).
    pub fn edit(&mut self) -> DataResult {
        self.ensure_active()?;
        if self.state().is_editing() {
            return Ok(());
        }
        if self.view().cds()?.read_only {
            return self.fail(DataError::new(t("o dataset é somente leitura")));
        }
        if self.record_count() == 0 {
            return self.append();
        }
        let cds = self.cds_mut()?;
        cds.buffer = Some(cds.row_values(&cds.rows[cds.cursor]));
        cds.state = DataSetState::Edit;
        self.state_changed();
        Ok(())
    }

    /// Começa um registro novo na posição atual (como `Insert`).
    pub fn insert(&mut self) -> DataResult {
        self.start_insert(false)
    }

    /// Começa um registro novo no fim (como `Append`).
    pub fn append(&mut self) -> DataResult {
        self.start_insert(true)
    }

    fn start_insert(&mut self, at_end: bool) -> DataResult {
        self.ensure_active()?;
        if self.view().cds()?.read_only {
            return self.fail(DataError::new(t("o dataset é somente leitura")));
        }
        self.check_browse()?;
        let locale = self.ui.locale();
        let fields: Vec<ComponentId> = self.view().cds()?.columns.iter().map(|c| c.field).collect();
        let mut temp_key = self.view().cds()?.next_temp_key;
        let mut values = Vec::with_capacity(fields.len());
        for field in fields {
            let field = self.ui.get::<Field>(field).ok_or("campo liberado")?;
            values.push(if field.field_type == FieldType::AutoInc {
                temp_key -= 1;
                Value::Int(temp_key)
            } else {
                field.initial_value(&locale)
            });
        }
        // Detalhe: os campos ligados ao mestre já vêm preenchidos.
        for (col, value) in self.master_values()?.unwrap_or_default() {
            values[col] = value;
        }
        let cds = self.cds_mut()?;
        cds.next_temp_key = temp_key;
        cds.insert_at = if at_end || cds.rows.is_empty() { cds.rows.len() } else { cds.cursor };
        cds.buffer = Some(values);
        cds.state = DataSetState::Insert;
        cds.bof = false;
        cds.eof = false;
        self.state_changed();
        if let Err(e) = self.run_hook(|h| &mut h.on_new_record) {
            self.cancel()?;
            return self.fail(DataError::new(e));
        }
        self.notify(DataEvent::RecordChanged(None));
        Ok(())
    }

    /// Grava a edição ou inclusão (como `Post`). Antes, os controles gravam o texto
    /// pendente; depois, valida `required` e chama `before_post`. No modo `on_post`,
    /// envia o delta ao backend em seguida.
    pub fn post(&mut self) -> DataResult {
        if !self.state().is_editing() {
            return Ok(());
        }
        self.notify(DataEvent::UpdateData);
        // Texto digitado ainda no `on_set_text` do backend: grava quando a resposta chegar.
        let cds = self.cds_mut()?;
        if !cds.awaiting_text.is_empty() {
            cds.post_deferred = true;
            return Ok(());
        }
        cds.post_deferred = false;
        if let Some(message) = self.cds_mut()?.pending_error.take() {
            return self.fail(DataError::new(message));
        }
        if let Some((field, message)) = self.view().cds()?.field_errors.first().cloned() {
            return self.fail(DataError::on_field(&field, message));
        }
        let view = self.view();
        let cds = view.cds()?;
        let buffer = cds.buffer.clone().unwrap_or_default();
        for (column, value) in cds.columns.iter().zip(&buffer) {
            let field = view.ui.get::<Field>(column.field).ok_or("campo liberado")?;
            let empty = value.is_null() || value.as_str().is_some_and(|s| s.trim().is_empty());
            if field.required && empty {
                let error = DataError::on_field(&column.name, tf("{0} é obrigatório", &[&field.caption()]));
                return self.fail(error);
            }
        }
        if let Err(e) = self.run_hook(|h| &mut h.before_post) {
            return self.fail(DataError::new(e));
        }
        if !self.ask_before(Asking::Post)? {
            return Ok(());
        }

        let cds = self.cds_mut()?;
        let values = cds.buffer.take().unwrap_or_default();
        match cds.state {
            DataSetState::Edit => {
                let current = cds.row_values(&cds.rows[cds.cursor]);
                let row = &mut cds.rows[cds.cursor];
                if current != values {
                    if row.status == RowStatus::Unmodified {
                        row.original = Some(Box::new(current));
                        row.status = RowStatus::Modified;
                    }
                    row.data = RowData::Owned(Box::new(values));
                    row.error = None;
                }
            }
            DataSetState::Insert => {
                let row_id = new_row_id();
                let row = Row { id: row_id, data: RowData::Owned(Box::new(values)), status: RowStatus::Inserted, original: None, error: None };
                let at = cds.insert_at.min(cds.rows.len());
                cds.rows.insert(at, row);
                cds.cursor = at;
            }
            _ => {}
        }
        cds.state = DataSetState::Browse;
        let on_post = cds.apply_mode == ApplyMode::OnPost;
        // Com filtro ou índice, o registro gravado pode sair da visão ou mudar de lugar.
        let keep = self.current_row_id();
        self.rebuild_view(keep)?;
        self.state_changed();
        self.notify(DataEvent::DataSetChanged);
        if let Err(e) = self.run_hook(|h| &mut h.after_post) {
            return self.fail(DataError::new(e));
        }
        self.ui.fire(self.id, "after_post", &EventArgs::None);
        if on_post { self.apply_updates() } else { Ok(()) }
    }

    /// Descarta a edição ou inclusão (como `Cancel`).
    pub fn cancel(&mut self) -> DataResult {
        if !self.state().is_editing() {
            return Ok(());
        }
        let cds = self.cds_mut()?;
        cds.buffer = None;
        cds.state = DataSetState::Browse;
        cds.cursor = cds.cursor.min(cds.rows.len().saturating_sub(1));
        cds.pending_error = None;
        cds.field_errors.clear();
        cds.awaiting_text.clear();
        cds.post_deferred = false;
        self.state_changed();
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// A resposta do `on_set_text` do campo `name` chegou: sai da espera e, se era a última e
    /// havia um `post` pedido, grava agora.
    fn text_answered(&mut self, name: &str) -> DataResult {
        let cds = self.cds_mut()?;
        let before = cds.awaiting_text.len();
        cds.awaiting_text.retain(|f| !f.eq_ignore_ascii_case(name));
        if before > 0 && cds.awaiting_text.is_empty() && cds.post_deferred {
            cds.post_deferred = false;
            return self.post();
        }
        Ok(())
    }

    /// `true` enquanto algum texto digitado espera a resposta do `on_set_text` do backend.
    pub fn awaiting_text(&self) -> bool {
        self.view().cds().is_ok_and(|c| !c.awaiting_text.is_empty())
    }

    /// Exclui o registro atual (na inclusão, apenas cancela).
    pub fn delete(&mut self) -> DataResult {
        self.ensure_active()?;
        match self.state() {
            DataSetState::Insert => return self.cancel(),
            DataSetState::Edit => self.cancel()?,
            _ => {}
        }
        if self.view().cds()?.read_only {
            return self.fail(DataError::new(t("o dataset é somente leitura")));
        }
        if self.record_count() == 0 {
            return self.fail(DataError::new(t("não há registro para excluir")));
        }
        if let Err(e) = self.run_hook(|h| &mut h.before_delete) {
            return self.fail(DataError::new(e));
        }
        if !self.ask_before(Asking::Delete)? {
            return Ok(());
        }
        let cds = self.cds_mut()?;
        let mut row = cds.rows.remove(cds.cursor);
        if row.status != RowStatus::Inserted {
            if let Some(original) = row.original.take() {
                row.data = RowData::Owned(original);
            }
            cds.deleted.push(row);
        }
        cds.cursor = cds.cursor.min(cds.rows.len().saturating_sub(1));
        cds.eof = cds.rows.is_empty();
        let on_post = cds.apply_mode == ApplyMode::OnPost;
        self.notify(DataEvent::DataSetChanged);
        if on_post { self.apply_updates() } else { Ok(()) }
    }

    /// Textos de exibição do `on_get_text` remoto: pares (valor, texto). Os controles
    /// repintam com eles.
    pub fn set_field_texts(&mut self, field: &str, texts: Vec<(Value, String)>) -> DataResult {
        let id = self.view().field_id(field)?;
        let f = self.ui.get::<Field>(id).ok_or("campo liberado")?;
        let texts: Vec<(Value, String)> = texts.into_iter().map(|(v, t)| (f.coerce(v.clone()).unwrap_or(v), t)).collect();
        f.store_texts(texts);
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Resultado do `on_validate` remoto: `Some(mensagem)` marca o campo com erro (o
    /// `post` recusa enquanto houver); `None` limpa.
    pub fn set_field_error(&mut self, field: &str, message: Option<String>) -> DataResult {
        let (_, column) = self.view().column(field)?;
        let name = column.name.clone();
        let cds = self.cds_mut()?;
        cds.field_errors.retain(|(f, _)| !f.eq_ignore_ascii_case(&name));
        if let Some(m) = message {
            cds.field_errors.push((name.clone(), m));
        }
        self.notify(DataEvent::RecordChanged(Some(name.clone())));
        self.text_answered(&name)
    }

    /// Muda o valor de um campo do registro em edição.
    pub fn set_value(&mut self, field: &str, value: impl Into<Value>) -> DataResult {
        let (col, column) = self.view().column(field)?;
        let (field_id, name) = (column.field, column.name.clone());
        if !self.state().is_editing() {
            return self.fail(DataError::on_field(&name, "o dataset não está em edição (chame edit, insert ou append antes)"));
        }
        let field = self.ui.get::<Field>(field_id).ok_or("campo liberado")?;
        if field.is_read_only() {
            let error = DataError::on_field(&name, tf("{0} é somente leitura", &[&field.caption()]));
            return self.fail(error);
        }
        let checked = field.coerce(value.into()).and_then(|v| field.check(&v).map(|_| v));
        let value = match checked {
            Ok(v) => v,
            Err(message) => {
                // A resposta de um `on_set_text` com valor recusado: o campo fica com o erro (e o
                // `post` adiado falha com ele).
                if self.view().cds().is_ok_and(|c| c.awaiting_text.iter().any(|f| f.eq_ignore_ascii_case(&name))) {
                    self.cds_mut()?.field_errors.push((name.clone(), message.clone()));
                    let _ = self.text_answered(&name);
                }
                return self.fail(DataError::on_field(&name, message));
            }
        };
        let text = field.text_of(&value, false, &self.ui.locale());
        let json = crate::json::value_to_json(&value);
        // O `on_validate` por nome é pergunta: recusado na hora (o `Err` do handler em Rust),
        // o valor não entra e o foco fica no campo; o backend
        // remoto responde depois (o valor entra, e a recusa marca o campo, como o
        // `field_error`; um `post` espera a resposta).
        let mut validating = None;
        if self.ui.has_handler(field_id, "on_validate") {
            let args = serde_json::json!({ "field": name, "value": json, "text": text });
            match self.ui.ask(field_id, "on_validate", &EventArgs::Text(args.to_string())) {
                Asked::Answered(Answer::Refused(message)) => return self.fail(DataError::on_field(&name, message)),
                Asked::Answered(_) => {}
                Asked::Deferred(token) => validating = Some(token),
            }
        }
        let cds = self.cds_mut()?;
        if let Some(buffer) = cds.buffer.as_mut() {
            buffer[col] = value;
        }
        // Valor novo: o erro do anterior não vale mais (a validação remota responde de novo).
        cds.field_errors.retain(|(f, _)| !f.eq_ignore_ascii_case(&name));
        self.notify(DataEvent::RecordChanged(Some(name.clone())));
        self.ui.fire(field_id, "on_change", &EventArgs::Text(text.clone()));
        if let Some(token) = validating {
            let cds = self.cds_mut()?;
            if !cds.awaiting_text.iter().any(|f| f.eq_ignore_ascii_case(&name)) {
                cds.awaiting_text.push(name.clone());
            }
            let (id, field) = (self.id, name.clone());
            self.ui.on_answer(token, move |ui, answer| {
                let mut ds = data_set(ui, id);
                let _ = match answer {
                    Answer::Refused(message) => ds.set_field_error(&field, Some(message)),
                    // Aceito: só sai da espera (um `field_error` mandado pelo handler fica).
                    Answer::Accepted(_) => ds.text_answered(&field),
                };
            });
            return Ok(());
        }
        // Era a resposta de um `on_set_text`: um `post` que esperava grava agora.
        self.text_answered(&name)
    }

    /// Muda um campo a partir do texto digitado (com `on_set_text`, idioma, formato...).
    pub fn set_text(&mut self, field: &str, text: &str) -> DataResult {
        // `on_set_text` por nome (o código gerado pela IDE, o backend remoto): o handler
        // recebe `{"field", "text"}` e grava o valor com `set_value` (ou recusa com
        // `set_field_error`); `aylis_data::set_text` faz isso com uma closure.
        let id = self.view().field_id(field)?;
        if !self.view().field(field)?.has_set_text() && self.ui.has_handler(id, "on_set_text") {
            if !self.state().is_editing() {
                return self.fail(DataError::on_field(field, "o dataset não está em edição (chame edit, insert ou append antes)"));
            }
            let name = self.view().column(field)?.1.name.clone();
            // Até a resposta chegar, um `post` espera (um handler local responde já dentro
            // do `fire`, e a marca sai na hora).
            let cds = self.cds_mut()?;
            if !cds.awaiting_text.iter().any(|f| f.eq_ignore_ascii_case(&name)) {
                cds.awaiting_text.push(name.clone());
            }
            let args = serde_json::json!({ "field": name, "text": text });
            self.ui.fire(id, "on_set_text", &EventArgs::Text(args.to_string()));
            return Ok(());
        }
        let parsed = self.view().field(field)?.parse_text(text, &self.ui.locale());
        match parsed {
            Ok(value) => self.set_value(field, value),
            Err(message) => self.fail(DataError::on_field(field, message)),
        }
    }

    /// Usado pelos controles durante o `UpdateData`: um erro aqui impede o `post`.
    pub fn report_pending_error(&mut self, message: impl Into<String>) {
        if let Ok(cds) = self.cds_mut() {
            cds.pending_error.get_or_insert(message.into());
        }
    }

    // ----- backend -----

    /// Envia as alterações pendentes ao backend: para o tratador de [`on_apply`](Self::on_apply)
    /// (Rust, síncrono) ou, sem ele, pelo evento `on_apply_updates` com o delta em JSON (o
    /// backend responde depois com [`reconcile`](Self::reconcile)).
    pub fn apply_updates(&mut self) -> DataResult {
        self.check_browse()?;
        let details = self.view().details(false);
        for detail in details {
            data_set(self.ui, detail).check_browse()?;
        }
        let changes = self.view().delta_with_details()?;
        if changes.is_empty() {
            return Ok(());
        }
        let handler = self.cds_mut()?.hooks.apply.take();
        match handler {
            Some(mut handler) => {
                let results = handler(&changes);
                if let Ok(cds) = self.cds_mut() {
                    cds.hooks.apply.get_or_insert(handler);
                }
                self.reconcile(&results)
            }
            None => {
                if !self.ui.has_handler(self.id, "on_apply_updates") {
                    return self.fail(DataError::new("nenhum backend recebeu as alterações (defina on_apply ou on_apply_updates)"));
                }
                let name = self.ui.name(self.id).unwrap_or_default().to_string();
                let json = crate::json::delta_to_json(&name, &changes);
                // Pergunta (item 8.3): a resposta são os resultados por registro (`results`, ou o
                // `reconcile` mandado antes); a exceção recusa a gravação inteira. O backend
                // remoto responde depois: o dataset fica "aplicando" até lá.
                match self.ui.ask(self.id, "on_apply_updates", &EventArgs::Text(json)) {
                    Asked::Answered(answer) => self.applied(answer),
                    Asked::Deferred(token) => {
                        self.cds_mut()?.applying = true;
                        self.state_changed();
                        let id = self.id;
                        self.ui.on_answer(token, move |ui, answer| {
                            let mut ds = data_set(ui, id);
                            if let Ok(cds) = ds.cds_mut() {
                                cds.applying = false;
                            }
                            let _ = ds.applied(answer);
                            ds.state_changed();
                        });
                        Ok(())
                    }
                }
            }
        }
    }

    /// A resposta do `on_apply_updates`: os resultados (se vieram nela) e o erro da recusa.
    fn applied(&mut self, answer: Answer) -> DataResult {
        match answer {
            Answer::Refused(message) => {
                if let Ok(cds) = self.cds_mut() {
                    cds.apply_errors = usize::MAX;
                }
                self.refused("on_apply_updates", message)
            }
            accepted => {
                let results = match accepted.value("results") {
                    Some(PropValue::String(text)) if !text.trim().is_empty() => {
                        let text = if text.trim_start().starts_with('[') { format!("{{\"results\": {text}}}") } else { text.clone() };
                        Some(self.parse_results(&text)?)
                    }
                    _ => None,
                };
                match results {
                    Some(results) => self.reconcile(&results),
                    None => Ok(()),
                }
            }
        }
    }

    /// Os registros que o último `apply_updates` não gravou ;
    /// `usize::MAX` se a gravação inteira foi recusada.
    pub fn apply_errors(&self) -> usize {
        self.view().cds().map_or(0, |c| c.apply_errors)
    }

    /// Aplica a resposta do backend: alterações aceitas saem do delta (com os valores que
    /// o backend devolveu, como ids gerados); recusadas ficam pendentes com a mensagem.
    ///
    /// Os resultados dos detalhes (vindos no delta do mestre) vão para o dataset de cada um
    /// pelo `ref`; a chave que o backend gerou para um mestre incluído passa aos campos de
    /// ligação dos detalhes dele (a provisória, `-1`, vira a do banco).
    pub fn reconcile(&mut self, results: &[ApplyResult]) -> DataResult {
        let mut own = Vec::new();
        let mut others: Vec<(ComponentId, Vec<ApplyResult>)> = Vec::new();
        for result in results {
            match self.view().owner_of(result.reference) {
                Some(owner) if owner != self.id => match others.iter_mut().find(|(o, _)| *o == owner) {
                    Some((_, list)) => list.push(result.clone()),
                    None => others.push((owner, vec![result.clone()])),
                },
                _ => own.push(result.clone()),
            }
        }
        let mut errors = Vec::new();
        if let Err(e) = self.reconcile_own(&own) {
            errors.push(e.message);
        }
        let mut count = self.view().cds().map_or(0, |c| c.apply_errors);
        for (owner, list) in others {
            let mut detail = data_set(self.ui, owner);
            if let Err(e) = detail.reconcile_own(&list) {
                errors.push(e.message);
            }
            count += detail.view().cds().map_or(0, |c| c.apply_errors);
        }
        if let Ok(cds) = self.cds_mut() {
            cds.apply_errors = count;
        }
        if errors.is_empty() { Ok(()) } else { Err(DataError::new(errors.join("; "))) }
    }

    /// A chave que o backend devolveu para um mestre (`replaced`: coluna daqui, valor
    /// provisório, valor gravado) vai para os campos de ligação dos detalhes que tinham o
    /// provisório.
    fn pass_keys_to_details(&mut self, replaced: &[(usize, Value, Value)]) -> DataResult {
        let names: Vec<(usize, String)> = {
            let cds = self.view().cds()?;
            replaced.iter().map(|(col, _, _)| (*col, cds.columns[*col].name.clone())).collect()
        };
        for detail in self.view().details(true) {
            let pairs = view(self.ui, detail).cds().map(|c| master_pairs(&c.master_fields)).unwrap_or_default();
            let mut moves: Vec<(usize, Value, Value)> = Vec::new();
            for (col, old, new) in replaced {
                let Some((_, master_field)) = names.iter().find(|(c, _)| c == col) else { continue };
                for (detail_field, _) in pairs.iter().filter(|(_, m)| m.eq_ignore_ascii_case(master_field)) {
                    let (detail_col, column) = view(self.ui, detail).column(detail_field)?;
                    let field = self.ui.get::<Field>(column.field).ok_or("campo liberado")?;
                    let old = field.coerce(old.clone()).unwrap_or_else(|_| old.clone());
                    let new = field.coerce(new.clone()).map_err(DataError::new)?;
                    moves.push((detail_col, old, new));
                }
            }
            if moves.is_empty() {
                continue;
            }
            let mut ds = data_set(self.ui, detail);
            let cds = ds.cds_mut()?;
            let mut changed = false;
            for row in cds.rows.iter_mut().chain(cds.hidden.iter_mut()) {
                for (col, old, new) in &moves {
                    if src(&cds.store, row).get(*col) == *old {
                        owned(&cds.store, row)[*col] = new.clone();
                        changed = true;
                    }
                }
            }
            if changed {
                ds.refilter()?;
                ds.notify(DataEvent::DataSetChanged);
            }
        }
        Ok(())
    }

    /// Os resultados dos registros deste dataset.
    fn reconcile_own(&mut self, results: &[ApplyResult]) -> DataResult {
        // Converte os valores devolvidos para o tipo de cada campo.
        let mut prepared = Vec::with_capacity(results.len());
        for result in results {
            let mut values = Vec::new();
            for (name, value) in &result.values {
                let (col, column) = self.view().column(name)?;
                let field = self.ui.get::<Field>(column.field).ok_or("campo liberado")?;
                values.push((col, field.coerce(value.clone()).map_err(DataError::new)?));
            }
            prepared.push((result.reference, result.error.clone(), values));
        }
        let mut errors = Vec::new();
        // Valores trocados pelo backend (coluna, antes, depois): a chave gerada de um mestre.
        let mut replaced: Vec<(usize, Value, Value)> = Vec::new();
        let cds = self.cds_mut()?;
        for (reference, error, values) in prepared {
            if let Some(row) = cds.rows.iter_mut().chain(cds.hidden.iter_mut()).find(|r| r.id == reference) {
                match error {
                    None => {
                        row.status = RowStatus::Unmodified;
                        row.original = None;
                        row.error = None;
                        if !values.is_empty() {
                            let own = owned(&cds.store, row);
                            for (col, value) in values {
                                if own[col] != value && own[col] != Value::Null {
                                    replaced.push((col, own[col].clone(), value.clone()));
                                }
                                own[col] = value;
                            }
                        }
                    }
                    Some(message) => {
                        row.error = Some(Box::new(message.clone()));
                        errors.push(message);
                    }
                }
            } else if let Some(pos) = cds.deleted.iter().position(|r| r.id == reference) {
                match error {
                    None => {
                        cds.deleted.remove(pos);
                    }
                    Some(message) => {
                        cds.deleted[pos].error = Some(Box::new(message.clone()));
                        errors.push(message);
                    }
                }
            }
        }
        if let Ok(cds) = self.cds_mut() {
            cds.apply_errors = errors.len();
        }
        if !replaced.is_empty() {
            self.pass_keys_to_details(&replaced)?;
        }
        self.notify(DataEvent::DataSetChanged);
        for message in &errors {
            self.ui.fire(self.id, "on_reconcile_error", &EventArgs::Text(message.clone()));
        }
        if errors.is_empty() { Ok(()) } else { Err(DataError::new(errors.join("; "))) }
    }

    /// Desfaz todas as alterações pendentes (como `CancelUpdates`).
    pub fn cancel_updates(&mut self) -> DataResult {
        let cds = self.cds_mut()?;
        let hidden = std::mem::take(&mut cds.hidden);
        cds.rows.extend(hidden);
        cds.buffer = None;
        if cds.state.is_editing() {
            cds.state = DataSetState::Browse;
        }
        cds.rows.retain(|r| r.status != RowStatus::Inserted);
        for row in &mut cds.rows {
            if let Some(original) = row.original.take() {
                row.data = RowData::Owned(original);
            }
            row.status = RowStatus::Unmodified;
            row.error = None;
        }
        for mut row in std::mem::take(&mut cds.deleted) {
            row.status = RowStatus::Unmodified;
            row.error = None;
            cds.rows.push(row);
        }
        cds.cursor = cds.cursor.min(cds.rows.len().saturating_sub(1));
        self.rebuild_view(None)?;
        self.state_changed();
        self.notify(DataEvent::DataSetChanged);
        Ok(())
    }

    /// Suspende os avisos aos controles (para processar muitos registros de uma vez).
    pub fn disable_controls(&mut self) {
        if let Ok(cds) = self.cds_mut() {
            cds.disabled += 1;
        }
    }

    /// Retoma os avisos; se algo mudou enquanto estavam suspensos, avisa uma vez.
    pub fn enable_controls(&mut self) {
        let pending = match self.cds_mut() {
            Ok(cds) => {
                cds.disabled = cds.disabled.saturating_sub(1);
                cds.disabled == 0 && std::mem::take(&mut cds.pending_notify)
            }
            Err(_) => false,
        };
        if pending {
            self.notify(DataEvent::DataSetChanged);
        }
    }

    // ----- eventos em Rust -----

    fn set_hook(&mut self, pick: fn(&mut Hooks) -> &mut Option<Box<DataSetHook>>, hook: Box<DataSetHook>) -> &mut Self {
        if let Ok(cds) = self.cds_mut() {
            *pick(&mut cds.hooks) = Some(hook);
        }
        self
    }

    /// Como o `OnNewRecord`: preencher valores iniciais com `set_value`.
    pub fn on_new_record(&mut self, f: impl FnMut(&mut Ui, ComponentId) -> Result<(), String> + 'static) -> &mut Self {
        self.set_hook(|h| &mut h.on_new_record, Box::new(f))
    }

    /// Como o `BeforePost`: devolver `Err` cancela a gravação (como o `Abort`).
    pub fn on_before_post(&mut self, f: impl FnMut(&mut Ui, ComponentId) -> Result<(), String> + 'static) -> &mut Self {
        self.set_hook(|h| &mut h.before_post, Box::new(f))
    }

    pub fn on_after_post(&mut self, f: impl FnMut(&mut Ui, ComponentId) -> Result<(), String> + 'static) -> &mut Self {
        self.set_hook(|h| &mut h.after_post, Box::new(f))
    }

    /// Como o `BeforeDelete`: devolver `Err` impede a exclusão.
    pub fn on_before_delete(&mut self, f: impl FnMut(&mut Ui, ComponentId) -> Result<(), String> + 'static) -> &mut Self {
        self.set_hook(|h| &mut h.before_delete, Box::new(f))
    }

    pub fn on_after_scroll(&mut self, f: impl FnMut(&mut Ui, ComponentId) -> Result<(), String> + 'static) -> &mut Self {
        self.set_hook(|h| &mut h.after_scroll, Box::new(f))
    }

    /// Backend em Rust, no mesmo processo: recebe o delta e devolve o resultado de cada
    /// alteração.
    pub fn on_apply(&mut self, f: impl FnMut(&[Change]) -> Vec<ApplyResult> + 'static) -> &mut Self {
        if let Ok(cds) = self.cds_mut() {
            cds.hooks.apply = Some(Box::new(f));
        }
        self
    }
}
