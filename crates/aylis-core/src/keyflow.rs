//! O caminho de uma tecla (item 3.6 dos ajustes):
//!
//! - `on_key_down`: a aplicação (o `ApplicationEvents`) → o
//!   form, se ele tem `key_preview` → o controle com foco → os pais, até
//!   alguém tratar; depois o Tab, os atalhos e as letras com `&`;
//! - um caractere digitado: `char_map` → `char_case` → filtros (`allowed_chars`...) →
//!   `on_key_press` (aplicação, form com `key_preview`, controle; vê o caractere já
//!   trocado, pode trocar de novo ou engolir) → o controle.
//!
//! `on_key_down` e `on_key_press` são eventos-pergunta (veja `question.rs`): em Rust o handler
//! engole a tecla na hora ([`Ui::suppress_key`]) ou troca o caractere
//! ([`Ui::replace_key_char`]); ligados a um handler remoto, a tecla espera a
//! resposta, e as que chegam nesse meio-tempo ficam numa fila do form, em ordem, sem se
//! perder. Quem não liga o evento não paga nada.

use crate::component::ComponentId;
use crate::event::{EventArgs, KeyEvent};
use crate::prop::PropValue;
use crate::question::{Answer, Asked};
use crate::ui::Ui;

/// Uma entrada de teclado guardada enquanto uma tecla espera a resposta do backend.
#[derive(Clone, Debug)]
pub(crate) enum Held {
    Down(KeyEvent),
    Up(KeyEvent),
    Text(String),
}

/// Onde continuar o `on_key_down` depois de uma resposta adiada.
#[derive(Clone, Copy, Debug)]
enum Stage {
    /// Os `ApplicationEvents`, a partir deste índice.
    App(usize),
    /// O form com `key_preview`.
    Preview,
    /// Subindo do controle `id` para os pais (o evento de `id` já respondido se `asked`).
    Level(ComponentId, bool),
}

impl Ui {
    /// Dentro de um `on_key_down` ou `on_key_press`: engole a tecla.
    pub fn suppress_key(&mut self) {
        self.reply("key", "");
    }

    /// Dentro de um `on_key_press`: entrega `c` no lugar do caractere digitado.
    pub fn replace_key_char(&mut self, c: char) {
        self.reply("key", c.to_string());
    }

    /// Os componentes de eventos da aplicação (`ApplicationEvents`), em todos os forms.
    pub(crate) fn application_events(&self) -> Vec<ComponentId> {
        self.components().filter(|&c| self.class_name(c) == Some("ApplicationEvents")).collect()
    }

    /// O app vai acabar (o laço da plataforma terminou): o `on_terminate` de cada
    /// `ApplicationEvents`, para gravar o que falta (o relatório do Tabernáculo, P04).
    pub fn fire_terminate(&mut self) {
        for app in self.application_events() {
            self.fire(app, "on_terminate", &crate::EventArgs::None);
        }
        self.process_events();
    }

    /// Há um ícone na bandeja visível: o app continua rodando sem form aberto (o relatório do
    /// Tabernáculo, P12).
    pub fn has_tray_icon(&self) -> bool {
        self.components().any(|c| self.class_name(c) == Some("TrayIcon") && !matches!(self.get_prop(c, "visible"), Ok(PropValue::Bool(false))))
    }

    /// O form recebe as teclas antes do controle com foco (`key_preview`).
    fn key_preview(&self, root: ComponentId) -> bool {
        matches!(self.get_prop(root, "key_preview"), Ok(PropValue::Bool(true)))
    }

    /// Uma tecla espera a resposta do backend neste form, ou o backend está ocupado: a
    /// entrada vai para a fila, em ordem (as que já esperam saem antes).
    pub(crate) fn hold_input(&mut self, root: ComponentId, held: Held) -> bool {
        let busy = self.busy_blocks_input();
        let state = self.input.entry(root).or_default();
        if state.draining || (!state.holding && !busy && state.held.is_empty()) {
            return false;
        }
        state.held.push_back(held);
        true
    }

    /// A resposta chegou: solta a fila, em ordem (uma tecla da fila pode esperar de novo).
    fn release_input(&mut self, root: ComponentId) {
        if let Some(state) = self.input.get_mut(&root) {
            state.holding = false;
        }
        self.drain_held(root);
    }

    /// Entrega a fila, em ordem, até uma tecla esperar de novo ou o backend ficar ocupado.
    fn drain_held(&mut self, root: ComponentId) {
        loop {
            if self.busy_blocks_input() {
                return;
            }
            let Some(state) = self.input.get_mut(&root) else { return };
            if state.holding || state.draining {
                return;
            }
            let Some(next) = state.held.pop_front() else { return };
            state.draining = true;
            match next {
                Held::Down(e) => self.key_down(root, e),
                Held::Up(e) => self.key_up(root, e),
                Held::Text(t) => self.text_input(root, &t),
            }
            if let Some(state) = self.input.get_mut(&root) {
                state.draining = false;
            }
        }
    }

    fn start_holding(&mut self, root: ComponentId) {
        self.input.entry(root).or_default().holding = true;
    }

    /// O `on_key_down` de `id`: `Some(true)` engoliu, `Some(false)` segue, `None` adiou (e
    /// `resume` roda com a resposta: segue se não engoliu).
    fn ask_key_down(&mut self, root: ComponentId, id: ComponentId, event: KeyEvent, next: Stage) -> Option<bool> {
        if !self.has_handler(id, "on_key_down") {
            return Some(false);
        }
        match self.ask(id, "on_key_down", &EventArgs::Key(event)) {
            Asked::Answered(answer) => Some(swallowed(&answer)),
            Asked::Deferred(token) => {
                self.start_holding(root);
                self.on_answer(token, move |ui, answer| {
                    if !swallowed(&answer) {
                        ui.key_down_from(root, event, next);
                    }
                    ui.release_input(root);
                });
                None
            }
        }
    }

    /// Tecla pressionada: veja o módulo.
    pub(crate) fn key_down_flow(&mut self, root: ComponentId, event: KeyEvent) {
        self.key_down_from(root, event, Stage::App(0));
    }

    fn key_down_from(&mut self, root: ComponentId, event: KeyEvent, stage: Stage) {
        let mut stage = stage;
        loop {
            match stage {
                Stage::App(i) => {
                    let apps = self.application_events();
                    let Some(&app) = apps.get(i) else {
                        stage = Stage::Preview;
                        continue;
                    };
                    match self.ask_key_down(root, app, event, Stage::App(i + 1)) {
                        Some(true) | None => return,
                        Some(false) => stage = Stage::App(i + 1),
                    }
                }
                Stage::Preview => {
                    let first = self.focused(root).unwrap_or(root);
                    if self.key_preview(root) && first != root {
                        match self.ask_key_down(root, root, event, Stage::Level(first, false)) {
                            Some(true) | None => return,
                            Some(false) => {}
                        }
                    }
                    stage = Stage::Level(first, false);
                }
                Stage::Level(id, asked) => {
                    // Com `key_preview`, o form já recebeu a tecla.
                    let skip = id == root && self.key_preview(root) && self.focused(root).is_some_and(|f| f != root);
                    if !asked && !skip {
                        match self.ask_key_down(root, id, event, Stage::Level(id, true)) {
                            Some(true) | None => return,
                            Some(false) => {}
                        }
                    }
                    if self.deliver_key_down(id, event) {
                        return;
                    }
                    match self.parent(id) {
                        Some(p) if self.contains(p) => stage = Stage::Level(p, false),
                        _ => {
                            self.key_down_unhandled(root, event);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// O caractere digitado passa pelo `on_key_press` (aplicação, form com `key_preview`,
    /// controle) e vai ao controle `id`. `done` recebe o caractere final (`None`: engolido),
    /// na hora ou quando a resposta adiada chegar.
    pub(crate) fn press_chars(&mut self, root: ComponentId, id: ComponentId, chars: Vec<char>) {
        let mut targets = self.application_events();
        if self.key_preview(root) && root != id {
            targets.push(root);
        }
        targets.push(id);
        targets.retain(|&t| self.has_handler(t, "on_key_press"));
        if targets.is_empty() {
            let text: String = chars.into_iter().collect();
            self.deliver_text(id, &text);
            return;
        }
        let mut out = String::new();
        let mut rest = chars.into_iter();
        while let Some(c) = rest.next() {
            match self.press_from(root, id, &targets, 0, c) {
                Ok(Some(c)) => out.push(c),
                Ok(None) => {}
                Err(()) => {
                    // Adiou: o que veio antes vai já; o resto espera a resposta, na fila.
                    let tail: String = rest.collect();
                    let state = self.input.entry(root).or_default();
                    if !tail.is_empty() {
                        state.held.push_front(Held::Text(tail));
                    }
                    if !out.is_empty() {
                        self.deliver_text(id, &out);
                    }
                    return;
                }
            }
        }
        if !out.is_empty() {
            self.deliver_text(id, &out);
        }
    }

    /// Pergunta aos alvos a partir de `i`: `Ok(c)` respondido na hora, `Err` adiado (a
    /// resposta continua e entrega o caractere).
    fn press_from(&mut self, root: ComponentId, id: ComponentId, targets: &[ComponentId], i: usize, c: char) -> Result<Option<char>, ()> {
        let mut c = c;
        for (k, &t) in targets.iter().enumerate().skip(i) {
            match self.ask(t, "on_key_press", &EventArgs::Text(c.to_string())) {
                Asked::Answered(answer) => match pressed(&answer, c) {
                    Some(n) => c = n,
                    None => return Ok(None),
                },
                Asked::Deferred(token) => {
                    self.start_holding(root);
                    let targets = targets.to_vec();
                    self.on_answer(token, move |ui, answer| {
                        if let Some(n) = pressed(&answer, c) {
                            match ui.press_from(root, id, &targets, k + 1, n) {
                                Ok(Some(n)) => ui.deliver_text(id, &n.to_string()),
                                Ok(None) => {}
                                // Adiou de novo mais adiante: aquela resposta entrega e solta.
                                Err(()) => return,
                            }
                        }
                        ui.release_input(root);
                    });
                    return Err(());
                }
            }
        }
        Ok(Some(c))
    }

    fn deliver_text(&mut self, id: ComponentId, text: &str) {
        if self.contains(id) {
            self.deliver(id, |c, ctx| c.text_input(ctx, text));
        }
    }
}

/// O `on_key_down` engoliu a tecla? (recusou, ou respondeu `key` vazio ou `handled`).
fn swallowed(answer: &Answer) -> bool {
    match answer {
        Answer::Refused(_) => true,
        a => matches!(a.value("key"), Some(PropValue::String(s)) if s.is_empty()) || a.flag("handled", false),
    }
}

/// O caractere depois do `on_key_press`: o mesmo, o trocado (`key`) ou `None` (engolido).
fn pressed(answer: &Answer, c: char) -> Option<char> {
    match answer {
        Answer::Refused(_) => None,
        a => match a.value("key") {
            Some(PropValue::String(s)) => s.chars().next(),
            _ if a.flag("handled", false) => None,
            _ => Some(c),
        },
    }
}


/// O backend deixou de estar ocupado: as teclas que esperavam entram (veja `busy.rs`).
pub(crate) fn release_after_busy(ui: &mut Ui, root: ComponentId) {
    ui.drain_held(root);
}
