//! As propriedades com registros (`PropKind::RecordList`): o texto é o JSON da lista, lido
//! para as structs do controle uma vez por texto, não a cada pintura.

use std::cell::{Cell, Ref, RefCell};

use serde::de::DeserializeOwned;

/// A lista de registros em JSON; vazio, `[]` ou JSON inválido: nenhum (sem pânico).
pub fn parse_list<T: DeserializeOwned>(text: &str) -> Vec<T> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    serde_json::from_str(text).unwrap_or_default()
}

/// A lista lida de um texto, guardada: refeita só quando o texto muda.
pub struct RecordCache<T> {
    text: RefCell<String>,
    items: RefCell<Vec<T>>,
    read: Cell<bool>,
}

impl<T> Default for RecordCache<T> {
    fn default() -> Self {
        Self { text: RefCell::new(String::new()), items: RefCell::new(Vec::new()), read: Cell::new(false) }
    }
}

impl<T: DeserializeOwned> RecordCache<T> {
    /// Os registros de `text` (do cache, se o texto é o mesmo da última vez).
    pub fn get(&self, text: &str) -> Ref<'_, Vec<T>> {
        if !self.read.get() || *self.text.borrow() != text {
            *self.items.borrow_mut() = parse_list(text);
            *self.text.borrow_mut() = text.to_string();
            self.read.set(true);
        }
        self.items.borrow()
    }
}

/// O registro que um evento leva (o JSON no `EventArgs::Text`, ou num valor texto).
pub fn event_record<T: DeserializeOwned>(args: &aylis_core::EventArgs) -> Option<T> {
    let text = match args {
        aylis_core::EventArgs::Text(t) => t.as_str(),
        aylis_core::EventArgs::Value(aylis_core::PropValue::String(t)) => t.as_str(),
        _ => return None,
    };
    serde_json::from_str(text).ok()
}

/// O `EventValue` de uma struct de evento: o JSON do registro `$record`.
macro_rules! record_event {
    ($ty:ty, $record:expr) => {
        impl aylis_core::EventValue for $ty {
            const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::Record(&$record));
            fn from_args(args: &aylis_core::EventArgs) -> Option<Self> {
                crate::records::event_record(args)
            }
        }
    };
}
pub(crate) use record_event;
