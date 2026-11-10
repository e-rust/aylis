//! Um erro num handler ou num componente não derruba o app, como o
//! `Application.HandleException` do Delphi: o `Ui` pega o pânico (Rust) ou o erro que o
//! backend relata (as outras linguagens), e o mostra ao usuário; o app continua aberto.
//!
//! O caminho de um erro: o handler da aplicação ([`Ui::on_exception`], o
//! `TApplicationEvents.OnException`) e, se ele não tratar, a exibição padrão
//! ([`Ui::set_exception_display`], a caixa de mensagem que o `aylis-controls` instala). Sempre
//! vai também para o `activity` (o log e a gravação de uso).
//!
//! Um pânico na pintura não vira caixa (seria uma por quadro): o componente é marcado, pinta
//! um retângulo de erro no lugar dele e o erro é registrado uma vez.

use std::any::Any;
use std::rc::Rc;

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::ui::Ui;

/// Um erro que chegou ao `Ui`.
#[derive(Clone, Debug, PartialEq)]
pub struct Exception {
    /// A mensagem (a do `panic!`, a da exceção do backend).
    pub message: String,
    /// Onde: o caminho do componente (`frmPedido.btnSalvar`), vazio se não se sabe.
    pub path: String,
    /// O evento ou o método que estava rodando (`on_click`, `paint`), vazio se não se sabe.
    pub event: String,
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.path.is_empty(), self.event.is_empty()) {
            (false, false) => write!(f, "{} ({}.{})", self.message, self.path, self.event),
            (false, true) => write!(f, "{} ({})", self.message, self.path),
            _ => f.write_str(&self.message),
        }
    }
}

/// O handler da aplicação: devolve `true` se tratou (aí a caixa padrão não aparece).
pub type ExceptionHandlerFn = dyn Fn(&mut Ui, &Exception) -> bool;
/// A exibição padrão (a caixa de mensagem).
pub type ExceptionDisplayFn = dyn Fn(&mut Ui, &Exception);

/// A mensagem de um pânico (`panic!("...")` ou um `unwrap` que falhou).
pub fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "erro desconhecido".to_string()
    }
}

impl Ui {
    /// O tratamento de erro da aplicação (o `TApplicationEvents.OnException`): recebe todo
    /// erro de handler e de componente; devolva `true` se tratou (log próprio, envio ao
    /// suporte) e a caixa padrão não aparece.
    pub fn on_exception(&mut self, f: impl Fn(&mut Ui, &Exception) -> bool + 'static) {
        self.exceptions.handler = Some(Rc::new(f));
    }

    /// Como um erro é mostrado quando a aplicação não trata (o `aylis-controls` põe a caixa de
    /// mensagem do aylis).
    pub fn set_exception_display(&mut self, f: impl Fn(&mut Ui, &Exception) + 'static) {
        self.exceptions.display = Some(Rc::new(f));
    }

    /// Entrega um erro: ao `activity`, ao handler da aplicação e, se ele não tratar, à
    /// exibição padrão. O protocolo usa para o erro relatado pelo backend.
    pub fn raise_exception(&mut self, exception: Exception) {
        let text = exception.to_string();
        self.report(&crate::activity::Activity::Error { message: &text });
        // Um erro dentro do tratamento de outro só vai para o log (sem laço de caixas).
        if self.exceptions.handling {
            crate::stderr_line!("aylis: erro ao tratar um erro: {text}");
            return;
        }
        self.exceptions.handling = true;
        let mut handled = match self.exceptions.handler.clone() {
            Some(handler) => self.guard_call(|ui| handler(ui, &exception)).unwrap_or(false),
            None => false,
        };
        // O `on_exception` de um `ApplicationEvents` (por nome: em Rust ou no backend) trata.
        if !handled {
            let args = EventArgs::Text(format!(
                "{{\"message\":{},\"path\":{},\"event\":{}}}",
                json_string(&exception.message),
                json_string(&exception.path),
                json_string(&exception.event)
            ));
            for app in self.application_events() {
                if self.has_handler(app, "on_exception") {
                    self.fire(app, "on_exception", &args);
                    handled = true;
                }
            }
        }
        if !handled {
            match self.exceptions.display.clone() {
                Some(display) => {
                    let _ = self.guard_call(|ui| display(ui, &exception));
                }
                None => crate::stderr_line!("aylis: {text}"),
            }
        }
        self.exceptions.handling = false;
    }

    /// Roda `f` pegando um pânico (devolve `None` nele); quem chama decide o que fazer.
    fn guard_call<R>(&mut self, f: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(self))).ok()
    }

    /// Roda um handler ou método de `id` pegando o pânico: o erro vai para
    /// [`raise_exception`](Self::raise_exception) e o resultado é `None`.
    pub(crate) fn guarded<R>(&mut self, id: ComponentId, event: &str, f: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(self))) {
            Ok(r) => Some(r),
            Err(payload) => {
                let exception = Exception { message: panic_message(payload.as_ref()), path: self.path_of(id), event: event.to_string() };
                self.raise_exception(exception);
                None
            }
        }
    }

    /// Um componente que entrou em pânico ao pintar: pinta o retângulo de erro dali em
    /// diante (até ser liberado).
    pub fn paint_failed(&self, id: ComponentId) -> bool {
        self.exceptions.paint_failures.borrow().contains(&id)
    }

    /// Anota o pânico na pintura (uma vez por componente) para registrar depois do quadro.
    pub(crate) fn note_paint_failure(&self, id: ComponentId, payload: &(dyn Any + Send)) {
        if self.exceptions.paint_failures.borrow_mut().insert(id) {
            let exception = Exception { message: panic_message(payload), path: self.path_of(id), event: "paint".into() };
            crate::stderr_line!("aylis: {exception}");
            self.exceptions.paint_reports.borrow_mut().push(exception);
        }
    }

    /// Os pânicos de pintura do último quadro vão para o `activity` (a pintura só tem `&self`).
    pub(crate) fn flush_paint_failures(&mut self) {
        let reports = std::mem::take(&mut *self.exceptions.paint_reports.borrow_mut());
        for exception in reports {
            let text = exception.to_string();
            self.report(&crate::activity::Activity::Error { message: &text });
        }
    }
}

/// O estado dos erros no `Ui`.
#[derive(Default)]
pub(crate) struct Exceptions {
    handler: Option<Rc<ExceptionHandlerFn>>,
    display: Option<Rc<ExceptionDisplayFn>>,
    /// Tratando um erro agora (um erro dentro do tratamento só vai para o log).
    handling: bool,
    /// Os componentes que entraram em pânico ao pintar.
    paint_failures: std::cell::RefCell<std::collections::HashSet<ComponentId>>,
    paint_reports: std::cell::RefCell<Vec<Exception>>,
}

/// O texto como string JSON (o núcleo não depende do `serde_json`).
fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
