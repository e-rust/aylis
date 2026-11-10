//! # aylis-data
//!
//! Camada de dados do aylis no modelo da VCL — sem lógica de backend:
//!
//! | VCL | aylis |
//! |---|---|
//! | `TClientDataSet` | [`ClientDataSet`]: registros em memória, cursor, estados e change log |
//! | `TField` | [`Field`]: tipo, formatos, validação, `on_get_text`/`on_set_text`/`on_validate` |
//! | `TDataSource` | [`DataSource`] |
//! | `TFieldDataLink` | [`FieldLink`], usado pelos controles DB |
//! | `ApplyUpdates` / `OnReconcileError` | [`DataSet::apply_updates`] / [`DataSet::reconcile`] |
//!
//! O frontend não sabe nada de banco: os dados chegam do backend (`load_rows`,
//! `load_json`) e as alterações voltam como **delta**; o backend decide o que fazer e
//! responde registro a registro.
//!
//! ```
//! use aylis_core::Ui;
//! use aylis_data::*;
//!
//! let mut ui = Ui::new();
//! let cds = ui.add_root(ClientDataSet::new().with_key("id"));
//! ui.add(Field::new("id", FieldType::AutoInc), cds);
//! ui.add(Field::new("nome", FieldType::String).label("Nome").required(), cds);
//!
//! let mut ds = data_set(&mut ui, cds);
//! ds.load_rows(vec![vec![1.into(), "Maria".into()]]).unwrap();
//! ds.edit().unwrap();
//! ds.set_text("nome", "Maria da Silva").unwrap();
//! ds.post().unwrap();
//! assert_eq!(ds.delta().unwrap()[0].kind, ChangeKind::Update);
//! ```

pub mod binding;
mod dataset;
mod delta;
mod events;
pub mod expr;
mod field;
pub mod format;
mod handle;
pub mod json;
pub mod mask;
mod parallel;
mod store;
mod link;
mod value;

pub use dataset::{
    ApplyMode, ApplyResult, Change, ChangeKind, ClientDataSet, DataError, DataEvent, DataMessage, DataResult, MasterLink,
    DataSetState, DataSource, Row, RowStatus,
};
pub use delta::Delta;
pub use events::{get_text, set_text};
pub use field::{CharCase, Field, FieldType};
pub use handle::{DataSet, DataView, RowInfo, data_set, open_active, view};
pub use link::FieldLink;
pub use store::RowBatch;
pub use value::{Date, DateTime, Decimal, MAX_SCALE, Rounding, Time, Timestamp, Value, days_in_month, is_leap_year};

/// Registra as classes de dados (para layouts JSON e para a IDE).
pub fn register(ui: &mut aylis_core::Ui) {
    ui.register_class::<ClientDataSet>("Data");
    ui.register_class::<Field>("Data");
    ui.register_class::<DataSource>("Data");
    // As propriedades ligadas a campos (o LiveBindings).
    binding::install(ui);
}
