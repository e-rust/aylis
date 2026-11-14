//! O índice de nomes: `find`, `find_in` e `find_path` sem percorrer a árvore a cada
//! chamada.
//!
//! Um índice por escopo (form), montado quando alguém procura nele, e o global (o `find`,
//! em qualquer form), montado à parte. Uma mudança joga fora só o escopo afetado (e o
//! global): criar um frame repetido (`set_name`, `bind`, `set_parent` de cada instância)
//! não remonta os nomes da tela inteira a cada passo, o que deixava montar N frames
//! quadrático. Componente que entra sem nome não muda índice nenhum.
//!
//! A chave é o nome em minúsculas ASCII, como a comparação de antes (`eq_ignore_ascii_case`).

use std::collections::HashMap;

use crate::component::ComponentId;
use crate::ui::Ui;

#[derive(Default)]
pub(crate) struct NameIndex {
    /// Escopo → (nome → o componente): os nomes de cada form, montados
    /// um escopo por vez.
    scoped: HashMap<ComponentId, HashMap<String, ComponentId>>,
    /// Nome → o primeiro na ordem da árvore, em qualquer form (o `find`).
    global: Option<HashMap<String, ComponentId>>,
    /// (escopo, nome base) → o próximo número a tentar no `free_name_in`: o frame repetido
    /// não testa `fraX2`, `fraX3`... desde o começo a cada instância.
    next_suffix: HashMap<(ComponentId, String), u32>,
}

impl Ui {
    /// Joga o índice inteiro fora.
    pub(crate) fn forget_names(&self) {
        *self.names.borrow_mut() = NameIndex::default();
    }

    /// Joga fora o índice do escopo (`None`: o nome de um form raiz, que só está no global)
    /// e o global.
    pub(crate) fn forget_scope(&self, scope: Option<ComponentId>) {
        let mut names = self.names.borrow_mut();
        names.global = None;
        if let Some(scope) = scope {
            names.scoped.remove(&scope);
        }
    }

    /// Joga fora os escopos que deixaram de existir (os forms de uma subárvore liberada).
    pub(crate) fn forget_scopes(&self, gone: &[ComponentId]) {
        let mut names = self.names.borrow_mut();
        if !names.scoped.is_empty() {
            for id in gone {
                names.scoped.remove(id);
            }
        }
        if !names.next_suffix.is_empty() {
            let gone: std::collections::HashSet<&ComponentId> = gone.iter().collect();
            names.next_suffix.retain(|(scope, _), _| !gone.contains(scope));
        }
    }

    /// Os nomes `items` saem do índice de `scope` (que continua valendo), e o global cai.
    pub(crate) fn names_removed(&self, scope: Option<ComponentId>, items: &[(ComponentId, String)]) {
        let mut names = self.names.borrow_mut();
        names.global = None;
        let Some(map) = scope.and_then(|s| names.scoped.get_mut(&s)) else { return };
        for (id, name) in items {
            let key = name.to_ascii_lowercase();
            if map.get(&key) == Some(id) {
                map.remove(&key);
            }
        }
    }

    /// Os nomes `items` entram no índice de `scope`, e o global cai. Um nome que lá já é de
    /// outro (o primeiro na ordem da árvore decidiria) joga o escopo fora: ele é remontado.
    pub(crate) fn names_added(&self, scope: Option<ComponentId>, items: &[(ComponentId, String)]) {
        let mut names = self.names.borrow_mut();
        names.global = None;
        let Some(scope) = scope else { return };
        let Some(map) = names.scoped.get_mut(&scope) else { return };
        let mut clash = false;
        for (id, name) in items.iter().filter(|(_, n)| !n.is_empty()) {
            let key = name.to_ascii_lowercase();
            match map.get(&key) {
                Some(other) if other != id => clash = true,
                Some(_) => {}
                None => {
                    map.insert(key, *id);
                }
            }
        }
        if clash {
            names.scoped.remove(&scope);
        }
    }

    /// O próximo número a tentar para `base` em `scope` (2 na primeira vez).
    pub(crate) fn next_suffix(&self, scope: ComponentId, base: &str) -> u32 {
        self.names.borrow().next_suffix.get(&(scope, base.to_ascii_lowercase())).copied().unwrap_or(2)
    }

    pub(crate) fn set_next_suffix(&self, scope: ComponentId, base: &str, next: u32) {
        self.names.borrow_mut().next_suffix.insert((scope, base.to_ascii_lowercase()), next);
    }

    /// Os nomes (com nome) que pertencem ao escopo de `id` junto com ele: ele e a subárvore
    /// até os forms embutidos (que levam os próprios nomes).
    pub(crate) fn names_with(&self, id: ComponentId) -> Vec<(ComponentId, String)> {
        let mut ids = Vec::new();
        self.collect_scope(id, &mut ids);
        ids.into_iter().filter_map(|c| self.name(c).filter(|n| !n.is_empty()).map(|n| (c, n.to_string()))).collect()
    }

    /// O primeiro componente com o nome, em qualquer form.
    pub(crate) fn indexed_find(&self, name: &str) -> Option<ComponentId> {
        let key = name.to_ascii_lowercase();
        if self.names.borrow().global.is_none() {
            let global = self.build_global();
            self.names.borrow_mut().global = Some(global);
        }
        let found = self.names.borrow().global.as_ref().and_then(|g| g.get(&key).copied());
        if cfg!(debug_assertions) {
            let fresh = self.roots().iter().find_map(|&r| self.find_deep(r, name));
            if fresh != found {
                return self.stale_names(name, fresh);
            }
        }
        found
    }

    /// O componente com o nome no form `scope` (que é um escopo de nomes).
    pub(crate) fn indexed_find_in_scope(&self, scope: ComponentId, name: &str) -> Option<ComponentId> {
        let key = name.to_ascii_lowercase();
        if !self.names.borrow().scoped.contains_key(&scope) {
            let index = self.build_scope(scope);
            self.names.borrow_mut().scoped.insert(scope, index);
        }
        let found = self.names.borrow().scoped.get(&scope).and_then(|s| s.get(&key).copied());
        if cfg!(debug_assertions) {
            let fresh = self.find_in_children(scope, name);
            if fresh != found {
                return self.stale_names(name, fresh);
            }
        }
        found
    }

    /// O índice discordou da busca completa (um caminho que muda a árvore sem avisar):
    /// avisa em desenvolvimento e fica com a busca completa.
    fn stale_names(&self, name: &str, fresh: Option<ComponentId>) -> Option<ComponentId> {
        crate::stderr_line!("aylis: índice de nomes desatualizado para '{name}'");
        self.forget_names();
        fresh
    }

    /// Os nomes do escopo, na ordem da árvore (o primeiro vence), sem entrar nos forms
    /// embutidos nele (mas com o nome deles), como o `find_in_children`.
    fn build_scope(&self, scope: ComponentId) -> HashMap<String, ComponentId> {
        let mut index = HashMap::new();
        let mut stack: Vec<ComponentId> = self.children(scope).iter().rev().copied().collect();
        while let Some(id) = stack.pop() {
            if let Some(name) = self.name(id).filter(|n| !n.is_empty()) {
                index.entry(name.to_ascii_lowercase()).or_insert(id);
            }
            if !self.is_name_scope(id) {
                stack.extend(self.children(id).iter().rev());
            }
        }
        index
    }

    fn build_global(&self) -> HashMap<String, ComponentId> {
        let mut index = HashMap::new();
        let mut stack: Vec<ComponentId> = self.roots().iter().rev().copied().collect();
        while let Some(id) = stack.pop() {
            if let Some(name) = self.name(id).filter(|n| !n.is_empty()) {
                index.entry(name.to_ascii_lowercase()).or_insert(id);
            }
            stack.extend(self.children(id).iter().rev());
        }
        index
    }
}
