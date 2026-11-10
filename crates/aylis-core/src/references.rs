//! Quem aponta para quem: as propriedades do tipo componente (o `data_source` de um DBEdit, o
//! `data_set` de um DataSource, o `popup_menu`) formam um índice que o `Ui` monta quando
//! alguém pergunta e joga fora quando uma referência pode ter mudado.
//!
//! Serve a duas coisas: o dataset avisa só os controles ligados a ele (como os `TDataLink`
//! do Delphi), sem percorrer o app inteiro a cada registro; e liberar um componente limpa
//! sozinho as referências a ele (o `Notification` do Delphi, sem cada componente fazer à mão).
//!
//! O índice cai quando: uma propriedade de componente é gravada (`set_prop`), um componente
//! entra ou sai, ou alguém pega o componente para alterar direto (`get_mut`,
//! `component_mut`). Um método de componente que troca a referência no próprio campo, por
//! dentro, não derruba o índice: use `ctx.ui.set_prop` depois que o método terminar (em modo
//! de desenvolvimento, a conferência abaixo avisa).

use std::collections::HashMap;

use crate::component::ComponentId;
use crate::prop::{PropKind, PropValue};
use crate::ui::Ui;

/// Destino → quem aponta para ele, por qual propriedade.
pub(crate) type Index = HashMap<ComponentId, Vec<(ComponentId, &'static str)>>;

impl Ui {
    /// Quem aponta para `target` por uma propriedade do tipo componente, e por qual:
    /// `[(dbEdit, "data_source"), ...]`.
    pub fn referrers(&self, target: ComponentId) -> Vec<(ComponentId, &'static str)> {
        if self.references.borrow().is_none() {
            let index = self.build_references();
            *self.references.borrow_mut() = Some(index);
        }
        let found = self.references.borrow().as_ref().and_then(|i| i.get(&target).cloned()).unwrap_or_default();
        // Em desenvolvimento, confere com a busca completa: uma referência trocada por fora
        // (no campo, dentro de um método) apareceria aqui.
        if cfg!(debug_assertions) {
            let fresh = self.build_references().remove(&target).unwrap_or_default();
            if fresh != found {
                crate::stderr_line!("aylis: índice de referências desatualizado para {}: {found:?} ≠ {fresh:?}", self.path_of(target));
                *self.references.borrow_mut() = None;
                return fresh;
            }
        }
        found
    }

    /// Quem aponta para algum de `targets` (liberar vários de uma vez): uma consulta ao índice
    /// e, em desenvolvimento, uma só conferência completa. Perguntar um por um refazia a
    /// conferência para cada um: liberar a página de Traduções do NoOBS (2557 componentes)
    /// levava 2,7 s em debug (relatado pelo autor em 07/11).
    pub(crate) fn referrers_of_all(&self, targets: &[ComponentId]) -> Vec<(ComponentId, &'static str)> {
        if self.references.borrow().is_none() {
            let index = self.build_references();
            *self.references.borrow_mut() = Some(index);
        }
        let of = |index: &Index| -> Vec<(ComponentId, &'static str)> { targets.iter().filter_map(|t| index.get(t)).flatten().copied().collect() };
        let found = self.references.borrow().as_ref().map(of).unwrap_or_default();
        if cfg!(debug_assertions) {
            let fresh = of(&self.build_references());
            if fresh != found {
                crate::stderr_line!("aylis: índice de referências desatualizado ao liberar: {found:?} ≠ {fresh:?}");
                *self.references.borrow_mut() = None;
                return fresh;
            }
        }
        found
    }

    /// Joga o índice fora (uma referência pode ter mudado).
    pub(crate) fn forget_references(&self) {
        *self.references.borrow_mut() = None;
    }

    /// Componentes liberados: saem do índice (como destino e como quem aponta), sem
    /// remontá-lo.
    pub(crate) fn prune_references(&self, gone: &std::collections::HashSet<ComponentId>) {
        if let Some(index) = self.references.borrow_mut().as_mut() {
            index.retain(|target, sources| {
                sources.retain(|(source, _)| !gone.contains(source));
                !gone.contains(target) && !sources.is_empty()
            });
        }
    }

    fn build_references(&self) -> Index {
        let mut index = Index::new();
        let mut names: Vec<&'static str> = Vec::new();
        for source in self.components() {
            // Só as propriedades do tipo componente (poucas: `popup_menu`, `drag_image`,
            // `data_source`...), sem montar a lista inteira de cada um.
            let Some(component) = self.component(source) else { continue };
            names.clear();
            let mut layer = Some(component);
            while let Some(l) = layer {
                names.extend(l.props().iter().filter(|p| p.kind == PropKind::Component).map(|p| p.name));
                layer = l.inner();
            }
            if component.control().is_some() {
                names.extend(crate::ControlBase::PROPS.iter().filter(|p| p.kind == PropKind::Component).map(|p| p.name));
            }
            names.sort_unstable();
            names.dedup();
            for &name in &names {
                if let Ok(PropValue::Component(Some(target))) = self.get_prop(source, name) {
                    index.entry(target).or_default().push((source, name));
                }
            }
        }
        index
    }
}
