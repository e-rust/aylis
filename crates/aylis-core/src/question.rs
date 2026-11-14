//! Eventos que perguntam (item 3 dos ajustes): o `on_close_query` (`CanClose`), o
//! `before_post` e o `before_delete` (recusar a gravação), o `on_validate` do campo, o
//! `on_key_down`/`on_key_press` (engolir ou trocar a tecla), o `on_drag_start` e o
//! `on_drag_over`. Quem dispara pergunta com [`Ui::ask`] e recebe a resposta:
//!
//! - um handler em Rust responde na hora, dentro do `fire`: [`Ui::refuse`] recusa (o `Err`
//!   do handler que devolve `Result`), [`Ui::reply`] grava um campo da resposta
//!   (`can_close`, `key`, `data`...);
//! - um handler que não pode responder já (o backend em outra linguagem, um handler do mesmo
//!   form que espera o que roda terminar) adia com [`Ui::defer_question`] e responde depois
//!   com [`Ui::answer`]; quem perguntou recebe [`Asked::Deferred`] e diz, com
//!   [`Ui::on_answer`], o que fazer quando a resposta chegar (fechar o form, gravar o
//!   registro, entregar a tecla).
//!
//! Sem handler, a resposta é aceitar.

use std::collections::HashMap;
use std::rc::Rc;

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::prop::PropValue;
use crate::ui::Ui;

/// A resposta a um evento-pergunta.
#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    /// Aceito, com os campos que o handler respondeu (`can_close`, `key`...).
    Accepted(Vec<(String, PropValue)>),
    /// Recusado (a exceção do handler; a mensagem vai para a tela, vazia para recusar calado).
    Refused(String),
}

impl Answer {
    /// O campo respondido (`None` se o handler não mexeu nele).
    pub fn value(&self, name: &str) -> Option<&PropValue> {
        match self {
            Answer::Accepted(values) => values.iter().rev().find(|(n, _)| n == name).map(|(_, v)| v),
            Answer::Refused(_) => None,
        }
    }

    /// O campo booleano respondido, ou `default`.
    pub fn flag(&self, name: &str, default: bool) -> bool {
        match self.value(name) {
            Some(PropValue::Bool(b)) => *b,
            _ => default,
        }
    }

    pub fn is_refused(&self) -> bool {
        matches!(self, Answer::Refused(_))
    }
}

/// O que [`Ui::ask`] devolve.
#[derive(Clone, Debug, PartialEq)]
pub enum Asked {
    /// O handler respondeu na hora (ou não havia handler: aceito, sem campos).
    Answered(Answer),
    /// O handler vai responder depois: o número da pergunta, para o [`Ui::on_answer`].
    Deferred(u64),
}

/// O que fazer quando a resposta de uma pergunta adiada chegar.
type Continuation = Box<dyn FnOnce(&mut Ui, Answer)>;

/// Uma pergunta em andamento (a pilha: um handler pode perguntar de novo).
#[derive(Default)]
struct Frame {
    /// O evento que perguntou (o interceptador sabe se a pergunta é a do evento dele).
    target: Option<(ComponentId, String)>,
    refused: Option<String>,
    values: Vec<(String, PropValue)>,
    deferred: Option<u64>,
}

#[derive(Default)]
pub(crate) struct Questions {
    stack: Vec<Frame>,
    next: u64,
    /// As adiadas: quem perguntou (o caminho, para o log) e o que fazer na resposta.
    waiting: HashMap<u64, (ComponentId, String, Option<Continuation>)>,
    /// As respostas que chegaram antes do `on_answer` (o handler respondeu dentro do próprio
    /// `fire`, depois de adiar).
    early: HashMap<u64, Answer>,
    /// Os forms cujo `on_close_query` espera o backend, e os que a resposta já liberou.
    pub(crate) closes: Vec<ComponentId>,
    pub(crate) closing_now: Vec<ComponentId>,
    pub(crate) unresponsive: Option<Rc<dyn Fn(&mut Ui, ComponentId)>>,
    pub(crate) terminate: Option<Rc<dyn Fn(&mut Ui)>>,
}

impl Ui {
    /// Dispara o evento-pergunta `event` de `id` e devolve a resposta (veja o módulo).
    pub fn ask(&mut self, id: ComponentId, event: &str, args: &EventArgs) -> Asked {
        self.questions.stack.push(Frame { target: Some((id, event.to_string())), ..Frame::default() });
        self.fire(id, event, args);
        let frame = self.questions.stack.pop().unwrap_or_default();
        if let Some(token) = frame.deferred {
            self.questions.waiting.entry(token).or_insert((id, event.to_string(), None));
            return Asked::Deferred(token);
        }
        Asked::Answered(match frame.refused {
            Some(message) => Answer::Refused(message),
            None => Answer::Accepted(frame.values),
        })
    }

    /// A pergunta em andamento é a do evento `event` de `id` (e ainda não foi adiada).
    pub(crate) fn question_is_for(&self, id: ComponentId, event: &str) -> bool {
        self.questions.stack.last().is_some_and(|f| f.deferred.is_none() && f.target.as_ref().is_some_and(|(i, e)| *i == id && e == event))
    }

    /// `true` dentro do handler de um evento-pergunta (o receptor remoto manda o evento com o
    /// número da pergunta e adia).
    pub fn asking(&self) -> bool {
        self.questions.stack.last().is_some_and(|f| f.deferred.is_none())
    }

    /// Dentro de um evento-pergunta: recusa. A
    /// mensagem vai para a tela; vazia, recusa calado.
    pub fn refuse(&mut self, message: impl Into<String>) {
        if let Some(frame) = self.questions.stack.last_mut() {
            frame.refused = Some(message.into());
        }
    }

    /// Dentro de um evento-pergunta: grava um campo da resposta (`can_close`, `key`,
    /// `key_char`, `data`, `label`, `accept`).
    pub fn reply(&mut self, name: &str, value: impl Into<PropValue>) {
        if let Some(frame) = self.questions.stack.last_mut() {
            frame.values.push((name.to_string(), value.into()));
        }
    }

    /// Dentro de um evento-pergunta: a resposta vem depois, por [`answer`](Self::answer) com o
    /// número devolvido. Fora de uma pergunta, devolve 0 (e o `answer(0, ..)` não faz nada).
    pub fn defer_question(&mut self) -> u64 {
        let Some(frame) = self.questions.stack.last_mut() else { return 0 };
        if let Some(token) = frame.deferred {
            return token;
        }
        self.questions.next += 1;
        let token = self.questions.next;
        frame.deferred = Some(token);
        token
    }

    /// O que fazer quando a resposta da pergunta adiada `token` chegar (se já chegou, roda
    /// agora).
    pub fn on_answer(&mut self, token: u64, f: impl FnOnce(&mut Ui, Answer) + 'static) {
        if let Some(answer) = self.questions.early.remove(&token) {
            self.questions.waiting.remove(&token);
            f(self, answer);
            return;
        }
        if let Some(entry) = self.questions.waiting.get_mut(&token) {
            entry.2 = Some(Box::new(f));
        }
    }

    /// A resposta da pergunta adiada `token` (o backend respondeu). Devolve `false` se não
    /// havia essa pergunta (já respondida, ou o número é de outra tela).
    pub fn answer(&mut self, token: u64, answer: Answer) -> bool {
        let Some((_, _, continuation)) = self.questions.waiting.remove(&token) else {
            // Adiada e respondida dentro do mesmo `fire` (antes de o `ask` voltar).
            if token != 0 && token <= self.questions.next && self.questions.stack.iter().any(|f| f.deferred == Some(token)) {
                self.questions.early.insert(token, answer);
                return true;
            }
            return false;
        };
        match continuation {
            Some(f) => f(self, answer),
            None => {
                self.questions.early.insert(token, answer);
            }
        }
        true
    }

    /// Responde à pergunta adiada `token` rodando `f` como se fosse o handler: o que ele
    /// recusar ou responder vira a resposta. É como um handler do mesmo form, que esperou o
    /// outro terminar, responde.
    pub fn answering(&mut self, token: u64, f: impl FnOnce(&mut Ui)) {
        self.questions.stack.push(Frame::default());
        f(self);
        let frame = self.questions.stack.pop().unwrap_or_default();
        let answer = match frame.refused {
            Some(message) => Answer::Refused(message),
            None => Answer::Accepted(frame.values),
        };
        // Adiou de novo (um backend dentro do handler adiado): a resposta é a dele.
        if let Some(inner) = frame.deferred {
            self.on_answer(inner, move |ui, a| {
                ui.answer(token, a);
            });
            return;
        }
        self.answer(token, answer);
    }

    /// A tecla Voltar do celular (decidido em 14/11; o RTSPlayer troca de tela por frames e a
    /// tecla tirava o app da frente): pergunta ao `on_back` do form do topo. Respondido
    /// `handled` (ou recusado), o app tratou e nada mais acontece; senão, `default` faz o de
    /// sempre (a plataforma: cancelar o modal, fechar o form aberto, o principal para segundo
    /// plano). Com o backend em outra linguagem, a resposta chega depois e o `default` espera.
    pub fn back_pressed(&mut self, default: impl FnOnce(&mut Ui) + 'static) {
        let Some(form) = self.open_forms().last().copied() else {
            default(self);
            return;
        };
        let handled = |answer: &Answer| answer.is_refused() || answer.flag("handled", false);
        match self.ask(form, "on_back", &EventArgs::None) {
            Asked::Answered(answer) => {
                if !handled(&answer) {
                    default(self);
                }
            }
            Asked::Deferred(token) => self.on_answer(token, move |ui, answer| {
                if !handled(&answer) {
                    default(ui);
                }
            }),
        }
    }

    /// As perguntas que esperam resposta (para a tela saber que o backend está devendo).
    pub fn questions_waiting(&self) -> usize {
        self.questions.waiting.len()
    }

    /// Esquece as perguntas de componentes liberados.
    pub(crate) fn forget_questions(&mut self, doomed: &[ComponentId]) {
        if !self.questions.waiting.is_empty() {
            self.questions.waiting.retain(|_, (id, _, _)| !doomed.contains(id));
        }
    }
}

/// Os eventos que perguntam e os campos da resposta de cada um (o catálogo marca, e a IDE
/// gera o esqueleto com a assinatura certa). Sem campos: só aceitar ou recusar.
pub const QUESTION_EVENTS: &[(&str, &[&str])] = &[
    ("on_close_query", &["can_close"]),
    // A tecla Voltar do celular: `handled` diz que o app tratou (voltou a tela de dentro).
    ("on_back", &["handled"]),
    ("before_post", &[]),
    ("before_delete", &[]),
    ("on_validate", &[]),
    ("on_key_down", &["key"]),
    ("on_key_press", &["key"]),
    ("on_drag_start", &["data", "label"]),
    ("on_drag_over", &["accept"]),
    ("on_apply_updates", &["results"]),
    ("on_intercept", &["cancel"]),
    ("before_action", &[]),
    // O MCP (`aylis_remote::mcp`): a ferramenta responde `result`; o servidor aceita ou recusa
    // a chamada do agente.
    ("on_execute", &["result"]),
    ("on_call", &[]),
];

/// Os campos da resposta do evento-pergunta `event` (`None` se não é pergunta).
pub fn question_fields(event: &str) -> Option<&'static [&'static str]> {
    QUESTION_EVENTS.iter().find(|(e, _)| *e == event).map(|(_, f)| *f)
}

/// Os eventos que se juntam enquanto o anterior (do mesmo componente) não terminou no backend
/// (o catálogo marca; veja `aylis_remote::coalesces`). Clique e tecla nunca se juntam.
pub const COALESCED_EVENTS: &[&str] = &["on_timer", "on_resize", "on_scroll", "on_pointer_move", "on_wheel", "on_progress", "on_frame"];
