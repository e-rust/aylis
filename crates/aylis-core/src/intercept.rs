//! O interceptador de eventos (item 18 dos ajustes): um `EventInterceptor` (componente não
//! visual) recebe, num lugar só, os eventos de qualquer componente do form dele
//! (`scope = form`) ou da aplicação inteira (`scope = application`), antes do handler:
//!
//! - `on_intercept` é um evento-pergunta (veja `question.rs`) com o form, o componente, o
//!   nome do evento, os argumentos e o handler ligado ([`Intercepted`]); responder `cancel`
//!   ([`Ui::cancel_event`]) ou recusar com mensagem (a exceção) faz o handler não rodar. Num
//!   evento que já é pergunta (`before_post`, `on_close_query`, `before_delete`), cancelar é
//!   recusar: o registro não grava, o form não fecha;
//! - `after_intercept` roda depois do handler (auditoria);
//! - a ordem: os da aplicação, os do form, o handler; vários na ordem de criação, e o primeiro
//!   que cancela encerra. Passa também pelos eventos sem handler ligado;
//! - `events` e `classes` filtram (vazio = todos). No "todos" ficam de fora os eventos
//!   contínuos (mover o mouse, rolar, os de [`COALESCED_EVENTS`]): só entram pedidos pelo
//!   nome, para um handler remoto não receber uma ida e volta por movimento do mouse;
//! - com o handler do interceptador num backend em outra linguagem, a resposta vem depois: o
//!   evento original espera (e, se ele próprio é pergunta, a pergunta dele também);
//! - o botão com `modal_result` só fecha o form se o clique não foi cancelado; o navegador de
//!   dados pergunta `before_action` antes de cada botão, que passa por aqui;
//! - em modo dev, o evento cancelado vai para o log com o caminho do interceptador.

use std::cell::RefCell;

use crate::component::ComponentId;
use crate::event::{EventArgs, Intercepted};
use crate::forms::ModalResult;
use crate::prop::{PropEnum, PropValue};
use crate::question::{Answer, Asked, COALESCED_EVENTS};
use crate::ui::{Handler, Ui};

/// A classe do componente (em `aylis-controls-base`; o núcleo a acha pelo nome, como o
/// `ApplicationEvents`).
pub const INTERCEPTOR_CLASS: &str = "EventInterceptor";

/// Os eventos contínuos, fora do "todos" (`events` vazio).
pub const CONTINUOUS_EVENTS: &[&str] =
    &["on_pointer_move", "on_mouse_move", "on_pointer_enter", "on_pointer_leave", "on_mouse_enter", "on_mouse_leave", "on_paint", "on_gesture"];

/// Os próprios eventos do interceptador nunca são interceptados.
const OWN_EVENTS: &[&str] = &["on_intercept", "after_intercept"];

#[derive(Default)]
pub(crate) struct Interceptors {
    /// Os interceptadores do `Ui` (`None` = procurar de novo; cai quando um componente entra).
    cache: RefCell<Option<Vec<ComponentId>>>,
    /// Dentro de um `on_intercept`/`after_intercept`: os eventos disparados ali não são
    /// interceptados (senão o handler do interceptador se interceptaria).
    running: u32,
}

impl Interceptors {
    pub(crate) fn forget(&self) {
        *self.cache.borrow_mut() = None;
    }
}

/// O que o `on_intercept` respondeu: cancelou (e a mensagem, vazia se calado)?
fn cancelled(answer: &Answer) -> Option<String> {
    match answer {
        Answer::Refused(message) => Some(message.clone()),
        accepted if accepted.flag("cancel", false) => Some(String::new()),
        _ => None,
    }
}

impl Ui {
    /// Dentro de um `on_intercept`: o handler do evento interceptado não roda (o
    /// `Cancel := True`).
    pub fn cancel_event(&mut self) {
        self.reply("cancel", true);
    }

    /// Algum `EventInterceptor` vale para o evento `event` de `id` (um componente com ação
    /// própria pergunta antes de agir só quando há alguém para responder).
    pub fn is_intercepted(&self, id: ComponentId, event: &str) -> bool {
        !self.interceptors_for(id, event).is_empty()
    }

    /// Os interceptadores vivos, em ordem de criação.
    fn interceptor_list(&self) -> Vec<ComponentId> {
        if self.interceptors.cache.borrow().is_none() {
            let mut found: Vec<ComponentId> = self.components().filter(|&c| self.class_name(c) == Some(INTERCEPTOR_CLASS)).collect();
            found.sort();
            *self.interceptors.cache.borrow_mut() = Some(found);
        }
        let list = self.interceptors.cache.borrow().clone().unwrap_or_default();
        list.into_iter().filter(|&c| self.contains(c)).collect()
    }

    fn text_prop(&self, id: ComponentId, name: &str) -> String {
        match self.get_prop(id, name) {
            Ok(PropValue::String(s) | PropValue::Enum(s)) => s,
            _ => String::new(),
        }
    }

    /// Os interceptadores que valem para o evento `event` de `id`: os da aplicação, depois os
    /// do form; cada um com o filtro dele.
    fn interceptors_for(&self, id: ComponentId, event: &str) -> Vec<ComponentId> {
        if self.interceptors.running > 0 || OWN_EVENTS.contains(&event) {
            return Vec::new();
        }
        let all = self.interceptor_list();
        if all.is_empty() || all.contains(&id) {
            return Vec::new();
        }
        let form = self.form_of(id);
        let class = self.class_name(id).unwrap_or_default();
        let applies = |c: &ComponentId| {
            if !matches!(self.get_prop(*c, "enabled"), Ok(PropValue::Bool(true)) | Err(_)) {
                return false;
            }
            let events = self.text_prop(*c, "events");
            let listed = |list: &str, name: &str| list.split([';', ',']).map(str::trim).any(|n| n.eq_ignore_ascii_case(name));
            let event_ok = if events.trim().is_empty() {
                !CONTINUOUS_EVENTS.contains(&event) && !COALESCED_EVENTS.contains(&event)
            } else {
                listed(&events, event)
            };
            let classes = self.text_prop(*c, "classes");
            event_ok && (classes.trim().is_empty() || listed(&classes, class))
        };
        let scope_of = |c: &ComponentId| self.text_prop(*c, "scope");
        let mut out: Vec<ComponentId> = all.iter().copied().filter(|c| scope_of(c) == "application").filter(applies).collect();
        out.extend(all.iter().copied().filter(|c| scope_of(c) != "application" && self.form_of(*c) == form).filter(applies));
        out
    }

    /// O handler ligado ao evento (o nome; vazio para uma closure ou nenhum).
    fn bound_handler(&self, id: ComponentId, event: &str) -> String {
        self.entry(id)
            .and_then(|e| e.events.iter().find(|(n, _)| n == event).map(|(_, h)| h.clone()))
            .map(|h| match h {
                Handler::Named(n) => n,
                Handler::Closure(_) => String::new(),
            })
            .unwrap_or_default()
    }

    fn intercepted_args(&self, id: ComponentId, event: &str, args: &EventArgs) -> EventArgs {
        EventArgs::Intercept(Box::new(Intercepted {
            form: Some(self.form_of(id)),
            sender: Some(id),
            event: event.to_string(),
            handler: self.bound_handler(id, event),
            args: args.clone(),
        }))
    }

    /// O `fire` com os interceptadores: `None` se nenhum vale para o evento (o `fire` segue
    /// direto); senão, o que o `fire` devolve.
    pub(crate) fn intercept(&mut self, id: ComponentId, event: &str, args: &EventArgs) -> Option<bool> {
        let chain = self.interceptors_for(id, event);
        if chain.is_empty() {
            return None;
        }
        Some(self.intercept_from(id, event, args, &chain, 0))
    }

    /// Pergunta aos interceptadores de `chain` a partir de `start`; todos aceitando, roda o
    /// handler e o `after_intercept` de cada um.
    fn intercept_from(&mut self, id: ComponentId, event: &str, args: &EventArgs, chain: &[ComponentId], start: usize) -> bool {
        let wrapped = self.intercepted_args(id, event, args);
        for (i, &interceptor) in chain.iter().enumerate().skip(start) {
            self.interceptors.running += 1;
            let asked = self.ask(interceptor, "on_intercept", &wrapped);
            self.interceptors.running -= 1;
            match asked {
                Asked::Answered(answer) => {
                    if let Some(message) = cancelled(&answer) {
                        self.event_cancelled(interceptor, id, event, message, None);
                        return false;
                    }
                }
                Asked::Deferred(token) => {
                    // A resposta vem do backend: o evento espera. Se ele próprio é pergunta, a
                    // pergunta dele espera também, e a resposta é a do handler (ou a recusa).
                    let outer = if self.question_is_for(id, event) { Some(self.defer_question()) } else { None };
                    let modal = self.hold_modal_result(id, event);
                    let (event, args, chain) = (event.to_string(), args.clone(), chain.to_vec());
                    self.on_answer(token, move |ui, answer| {
                        if let Some(message) = cancelled(&answer) {
                            ui.event_cancelled(interceptor, id, &event, message, outer);
                            return;
                        }
                        if let Some(result) = modal {
                            ui.set_modal_result(ui.form_of(id), result);
                        }
                        match outer {
                            Some(token) => ui.answering(token, |ui| {
                                ui.intercept_from(id, &event, &args, &chain, i + 1);
                            }),
                            None => {
                                ui.intercept_from(id, &event, &args, &chain, i + 1);
                            }
                        }
                    });
                    return true;
                }
            }
        }
        let handled = self.fire_handler(id, event, args);
        self.interceptors.running += 1;
        for &interceptor in chain {
            if self.contains(interceptor) && self.has_handler(interceptor, "after_intercept") {
                self.fire_handler(interceptor, "after_intercept", &wrapped);
            }
        }
        self.interceptors.running -= 1;
        handled
    }

    /// Cancelado: o evento que é pergunta fica recusado, o botão com `modal_result` não fecha o
    /// form e, em modo dev, o log diz quem cancelou.
    fn event_cancelled(&mut self, interceptor: ComponentId, id: ComponentId, event: &str, message: String, outer: Option<u64>) {
        if cfg!(debug_assertions) {
            crate::stderr_line!("aylis: {}.{event} cancelado por {}", self.path_of(id), self.path_of(interceptor));
        }
        match outer {
            Some(token) => {
                self.answer(token, Answer::Refused(message));
            }
            None if self.question_is_for(id, event) => self.refuse(message),
            None => {}
        }
        if event == "on_click" && self.button_modal_result(id).is_some() {
            let form = self.form_of(id);
            self.set_modal_result(form, ModalResult::None);
        }
    }

    /// O `modal_result` de um botão (`None` se não tem ou é `none`).
    fn button_modal_result(&self, id: ComponentId) -> Option<ModalResult> {
        let name = match self.get_prop(id, "modal_result") {
            Ok(PropValue::Enum(n) | PropValue::String(n)) => n,
            _ => return None,
        };
        let index = ModalResult::NAMES.iter().position(|n| n.eq_ignore_ascii_case(&name))?;
        ModalResult::from_index(index).filter(|r| *r != ModalResult::None)
    }

    /// O clique de um botão com `modal_result` espera a resposta: o form não fecha antes dela.
    fn hold_modal_result(&mut self, id: ComponentId, event: &str) -> Option<ModalResult> {
        if event != "on_click" {
            return None;
        }
        let result = self.button_modal_result(id)?;
        let form = self.form_of(id);
        self.set_modal_result(form, ModalResult::None);
        Some(result)
    }
}
