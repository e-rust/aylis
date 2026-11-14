//! Ligações de propriedades a dados: qualquer propriedade de
//! qualquer componente pode mostrar um campo de um dataset (ou um texto montado com campos,
//! ou uma expressão), e as editáveis gravam de volta.
//!
//! O núcleo só guarda as ligações ([`Ui::set_binding`]) e avisa quem cuida dos dados quando
//! o usuário muda um componente ligado ([`Ui::set_binding_sink`]); quem lê e grava os campos
//! é o `aylis-data`. No JSON do form elas ficam em `"bindings"` de cada componente.

use std::rc::Rc;

use crate::component::ComponentId;
use crate::ui::Ui;

/// Uma propriedade ligada a dados.
#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    /// A propriedade do componente (`text`, `checked`, `color`...).
    pub property: String,
    /// O `DataSource` de onde vêm os dados.
    pub source: Option<ComponentId>,
    /// O campo (`nome`), um texto com campos (`Total: {valor:moeda}`) ou uma expressão com
    /// os campos do mesmo dataset (`=quantidade * preco`).
    pub path: String,
}

impl Binding {
    /// Só um campo (pode gravar de volta, se a propriedade for editável).
    pub fn is_field(&self) -> bool {
        !self.path.starts_with('=') && !self.path.contains('{')
    }
}

/// Quem cuida dos dados: avisado quando um componente ligado muda pelo usuário.
pub type BindingSink = Rc<dyn Fn(&mut Ui, ComponentId)>;

/// Os eventos que dizem "o usuário mudou o valor".
pub const CHANGE_EVENTS: [&str; 2] = ["on_change", "on_click"];

impl Ui {
    /// Liga (ou, com `path` vazio, desliga) a propriedade `property` de `id` a dados.
    pub fn set_binding(&mut self, id: ComponentId, property: &str, source: Option<ComponentId>, path: &str) {
        let Some(entry) = self.entry_mut(id) else { return };
        entry.bindings.retain(|b| b.property != property);
        if !path.trim().is_empty() {
            entry.bindings.push(Binding { property: property.to_string(), source, path: path.trim().to_string() });
        }
        if let Some(sink) = self.binding_refresh.clone() {
            sink(self, id);
        }
        self.request_repaint();
    }

    /// As ligações de `id`.
    pub fn bindings(&self, id: ComponentId) -> &[Binding] {
        self.entry(id).map_or(&[], |e| e.bindings.as_slice())
    }

    /// A ligação da propriedade, se houver.
    pub fn binding(&self, id: ComponentId, property: &str) -> Option<&Binding> {
        self.bindings(id).iter().find(|b| b.property == property)
    }

    /// Os componentes com alguma ligação a `source`.
    pub fn bound_to(&self, source: ComponentId) -> Vec<ComponentId> {
        self.components().filter(|&c| self.bindings(c).iter().any(|b| b.source == Some(source))).collect()
    }

    /// `sink` é avisado quando um componente com ligações dispara um dos
    /// [`CHANGE_EVENTS`] (grava de volta); `refresh` quando uma ligação é criada ou mudada
    /// (mostra o valor na hora).
    pub fn set_binding_sink(&mut self, sink: BindingSink, refresh: BindingSink) {
        self.binding_sink = Some(sink);
        self.binding_refresh = Some(refresh);
    }

    /// Chamado por [`Ui::fire`]: um componente ligado mudou pelo usuário.
    pub(crate) fn binding_changed(&mut self, id: ComponentId, event: &str) {
        if !CHANGE_EVENTS.contains(&event) || self.bindings(id).is_empty() || self.design_mode {
            return;
        }
        if let Some(sink) = self.binding_sink.clone() {
            sink(self, id);
        }
    }
}
