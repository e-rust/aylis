//! Ajudante para o backend gravar o delta: separa por tipo (incluídos, alterados, excluídos)
//! e, ao aceitar a inclusão de um mestre com a chave que o banco gerou, troca a chave
//! provisória nas alterações seguintes dos detalhes dele. O SQL fica com o programador.
//!
//! ```
//! use aylis_data::{ApplyResult, Change, ChangeKind, Delta, Value};
//!
//! fn gravar(changes: &[Change]) -> Vec<ApplyResult> {
//!     let mut delta = Delta::new(changes.to_vec());
//!     for i in 0..delta.len() {
//!         let change = delta.change(i).clone(); // já com a chave do mestre trocada
//!         match change.kind {
//!             ChangeKind::Insert => {
//!                 let id = 1043; // insert ... returning id
//!                 delta.accept(change.reference, vec![("id".into(), Value::Int(id))]);
//!             }
//!             _ => delta.accept(change.reference, Vec::new()),
//!         }
//!     }
//!     delta.into_results()
//! }
//! ```

use crate::dataset::{ApplyResult, Change, ChangeKind};
use crate::value::Value;

/// As alterações de um `apply_updates`, na ordem de gravar, com os resultados de cada uma.
#[derive(Clone, Debug, Default)]
pub struct Delta {
    changes: Vec<Change>,
    results: Vec<ApplyResult>,
}

impl Delta {
    pub fn new(changes: Vec<Change>) -> Self {
        Self { changes, results: Vec::new() }
    }

    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// A alteração `index` (com as chaves de mestre já trocadas pelos `accept` anteriores).
    pub fn change(&self, index: usize) -> &Change {
        &self.changes[index]
    }

    /// As alterações de um tipo, na ordem de gravar.
    pub fn of_kind(&self, kind: ChangeKind) -> impl Iterator<Item = &Change> {
        self.changes.iter().filter(move |c| c.kind == kind)
    }

    pub fn inserted(&self) -> impl Iterator<Item = &Change> {
        self.of_kind(ChangeKind::Insert)
    }

    pub fn updated(&self) -> impl Iterator<Item = &Change> {
        self.of_kind(ChangeKind::Update)
    }

    pub fn deleted(&self) -> impl Iterator<Item = &Change> {
        self.of_kind(ChangeKind::Delete)
    }

    /// As alterações de um dataset (o mestre ou um detalhe).
    pub fn of_data_set<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Change> + 'a {
        self.changes.iter().filter(move |c| c.data_set.eq_ignore_ascii_case(name))
    }

    /// Gravou: `values` são os valores que o banco definiu (a chave gerada, um carimbo de
    /// data). Os detalhes que dependem desta inclusão (`master`) recebem a chave nos campos de
    /// ligação, no `new` e no `key`.
    pub fn accept(&mut self, reference: u64, values: Vec<(String, Value)>) {
        for change in self.changes.iter_mut() {
            let Some(link) = &change.master else { continue };
            if link.reference != reference {
                continue;
            }
            for (detail, master) in &link.fields {
                let Some((_, value)) = values.iter().find(|(n, _)| n.eq_ignore_ascii_case(master)) else { continue };
                for list in [&mut change.new, &mut change.key] {
                    if let Some(slot) = list.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(detail)) {
                        slot.1 = value.clone();
                    }
                }
            }
        }
        self.results.push(ApplyResult { reference, error: None, values });
    }

    /// Não gravou: a mensagem vai para a tela, e o registro continua pendente.
    pub fn reject(&mut self, reference: u64, message: impl Into<String>) {
        self.results.push(ApplyResult::error(reference, message));
    }

    pub fn results(&self) -> &[ApplyResult] {
        &self.results
    }

    pub fn into_results(self) -> Vec<ApplyResult> {
        self.results
    }
}
