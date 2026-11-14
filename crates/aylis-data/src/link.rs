//! [`FieldLink`]: o que um controle ligado a um campo precisa.

use aylis_core::{ComponentId, Ui};

use crate::dataset::{DataEvent, DataMessage, DataResult, DataSource};
use crate::field::Field;
use crate::handle::{data_set, view};
use crate::value::Value;

/// Ligação de um controle a um campo: `data_source` + `data_field`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldLink {
    pub data_source: Option<ComponentId>,
    pub data_field: String,
}

impl FieldLink {
    /// O dataset por trás do `DataSource` (se ligado e habilitado).
    pub fn data_set(&self, ui: &Ui) -> Option<ComponentId> {
        let source = ui.get::<DataSource>(self.data_source?)?;
        if source.enabled { source.data_set } else { None }
    }

    /// A mensagem é do dataset deste controle e interessa a este campo?
    pub fn concerns(&self, ui: &Ui, message: &DataMessage) -> bool {
        if self.data_set(ui) != Some(message.data_set) {
            return false;
        }
        match &message.event {
            DataEvent::RecordChanged(Some(field)) => field.eq_ignore_ascii_case(&self.data_field),
            _ => true,
        }
    }

    pub fn field<'a>(&self, ui: &'a Ui) -> Option<&'a Field> {
        view(ui, self.data_set(ui)?).field(&self.data_field).ok()
    }

    /// Há um registro para mostrar (dataset aberto e não vazio, ou em inclusão)?
    pub fn has_record(&self, ui: &Ui) -> bool {
        self.data_set(ui).is_some_and(|ds| {
            let v = view(ui, ds);
            v.is_active() && !v.is_empty()
        })
    }

    pub fn value(&self, ui: &Ui) -> Value {
        self.data_set(ui).and_then(|ds| view(ui, ds).value(&self.data_field).ok()).unwrap_or_default()
    }

    pub fn display_text(&self, ui: &Ui) -> String {
        self.data_set(ui).and_then(|ds| view(ui, ds).text(&self.data_field).ok()).unwrap_or_default()
    }

    pub fn edit_text(&self, ui: &Ui) -> String {
        self.data_set(ui).and_then(|ds| view(ui, ds).edit_text(&self.data_field).ok()).unwrap_or_default()
    }

    /// O usuário pode alterar este campo agora (direto ou com auto-edição)?
    pub fn can_modify(&self, ui: &Ui) -> bool {
        let Some(ds) = self.data_set(ui) else { return false };
        let v = view(ui, ds);
        let read_only = ui.get::<crate::ClientDataSet>(ds).is_none_or(|c| c.read_only);
        let field_read_only = v.field(&self.data_field).map_or(true, |f| f.is_read_only());
        let auto_edit = self.data_source.and_then(|s| ui.get::<DataSource>(s)).is_some_and(|s| s.auto_edit);
        v.is_active() && !read_only && !field_read_only && (v.state().is_editing() || auto_edit)
    }

    /// Coloca o dataset em edição se ainda não estiver.
    pub fn begin_edit(&self, ui: &mut Ui) -> DataResult {
        let ds = self.data_set(ui).ok_or("controle sem dataset")?;
        let mut ds = data_set(ui, ds);
        if ds.state().is_editing() { Ok(()) } else { ds.edit() }
    }

    pub fn write_text(&self, ui: &mut Ui, text: &str) -> DataResult {
        let ds = self.data_set(ui).ok_or("controle sem dataset")?;
        data_set(ui, ds).set_text(&self.data_field, text)
    }

    pub fn write_value(&self, ui: &mut Ui, value: impl Into<Value>) -> DataResult {
        let ds = self.data_set(ui).ok_or("controle sem dataset")?;
        data_set(ui, ds).set_value(&self.data_field, value)
    }
}
