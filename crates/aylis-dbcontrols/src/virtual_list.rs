//! [`DataRepeater`]: a [`VirtualList`] ligada a um dataset (como o `TDBCtrlGrid`): um item
//! por registro, desenhado com o modelo, com os campos de `bindings` preenchidos sozinhos.

use std::any::Any;

use aylis_controls::{RowSource, VirtualList};
use aylis_core::{Component, ComponentId, EventCtx, Ui, properties};
use aylis_data::{DataEvent, FieldLink, view};

use crate::data_message;

/// Os registros do dataset por trás de um `DataSource`.
struct DataRows(ComponentId);

impl DataRows {
    fn data_set(&self, ui: &Ui) -> Option<ComponentId> {
        FieldLink { data_source: Some(self.0), data_field: String::new() }.data_set(ui)
    }
}

impl RowSource for DataRows {
    fn count(&self, ui: &Ui) -> usize {
        self.data_set(ui).map_or(0, |ds| view(ui, ds).record_count())
    }

    fn value(&self, ui: &Ui, row: usize, field: &str) -> Option<String> {
        let ds = self.data_set(ui)?;
        view(ui, ds).value_at(row, field).ok().map(|v| v.to_string())
    }
}

/// Lista virtual de registros: `data_source` dá a quantidade e os valores; `bindings`
/// (`"lblNome=nome;lblPreco=preco"`) liga os componentes do modelo aos campos.
#[derive(Default)]
pub struct DataRepeater {
    pub base: VirtualList,
    pub data_source: Option<ComponentId>,
}

impl Component for DataRepeater {
    fn class_name(&self) -> &'static str {
        "DataRepeater"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    fn control(&self) -> Option<&aylis_core::ControlBase> {
        Some(&self.base.base)
    }

    fn control_mut(&mut self) -> Option<&mut aylis_core::ControlBase> {
        Some(&mut self.base.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data"
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "data_source" {
            self.base.set_row_source(self.data_source.map(|ds| Box::new(DataRows(ds)) as Box<dyn RowSource>));
            self.base.resync(ctx);
        } else {
            self.base.property_changed(ctx, name);
        }
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        match data_message(message) {
            // Registros novos, apagados, alterados, filtro: preenche de novo.
            Some(m) if m.event != DataEvent::UpdateData => self.base.resync(ctx),
            Some(_) => {}
            None => self.base.message(ctx, message),
        }
    }
}
