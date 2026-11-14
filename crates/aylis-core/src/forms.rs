//! Forms abertos como janelas: `show`, `show_modal`, `close` e `ModalResult`, com
//! a pergunta antes de fechar (`on_close_query`) e o aviso ao fechar (`on_close`).
//!
//! O core só mantém a lista de forms abertos (na ordem em que estão empilhados) e as
//! regras; a plataforma abre e fecha as janelas de verdade para acompanhar essa lista
//! ([`Ui::open_forms`]). No celular e no navegador, onde há uma tela só, a plataforma
//! mostra o form do topo.
//!
//! O `ShowModal` do aylis **não bloqueia**: o resultado chega numa função quando o form
//! fecha. Um loop modal dentro de um evento deixaria a interface sem se redesenhar.
//!
//! ```
//! # use aylis_core::*;
//! # #[derive(Default)] struct F { base: ControlBase }
//! # impl Component for F {
//! #   fn class_name(&self) -> &'static str { "Form" }
//! #   fn control(&self) -> Option<&ControlBase> { Some(&self.base) }
//! #   fn control_mut(&mut self) -> Option<&mut ControlBase> { Some(&mut self.base) }
//! # }
//! let mut ui = Ui::new();
//! let principal = ui.add_root(F::default());
//! let cliente = ui.add_root(F::default());
//! ui.show(principal).unwrap();
//! ui.show_modal(cliente, |_ui, result| {
//!     if result == ModalResult::Ok { /* gravar */ }
//! }).unwrap();
//! ui.set_modal_result(cliente, ModalResult::Ok); // como um botão com modal_result = ok
//! assert_eq!(ui.open_forms(), vec![principal]);
//! ```

use crate::component::ComponentId;
use crate::error::{Error, Result};
use crate::event::EventArgs;
use crate::prop::{PropEnum, PropValue};
use crate::ui::Ui;

/// Onde a janela de um form fica na tela, em pixels físicos: posição do canto, tamanho da
/// área interna e se está maximizada.
/// A plataforma atualiza quando a janela é movida ou muda de
/// tamanho; maximizada, guarda a posição e o tamanho de antes (para voltar a eles). Para
/// reabrir onde estava, o app guarda isto e devolve com [`Ui::set_window_placement`]
/// antes do `show`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowPlacement {
    pub x: i32,
    pub y: i32,
    /// 0 = ainda não se sabe (fica o tamanho do form).
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl Ui {
    /// Onde está a janela do form (ou onde vai abrir), se a plataforma ou o app disseram.
    pub fn window_placement(&self, form: ComponentId) -> Option<WindowPlacement> {
        self.placements.get(&form).copied()
    }

    pub fn set_window_placement(&mut self, form: ComponentId, placement: WindowPlacement) {
        self.placements.insert(form, placement);
    }

    /// A janela do form nasce escondida (o app que começa na bandeja); depois aparece com
    /// [`WindowCommand::Show`](crate::WindowCommand::Show). Vale para a próxima vez que a
    /// janela for criada.
    pub fn set_start_hidden(&mut self, form: ComponentId, hidden: bool) {
        if hidden {
            self.start_hidden.insert(form);
        } else {
            self.start_hidden.remove(&form);
        }
    }

    /// A plataforma pergunta ao criar a janela (e o pedido se gasta).
    pub fn take_start_hidden(&mut self, form: ComponentId) -> bool {
        self.start_hidden.remove(&form)
    }
}

/// Resultado de um form modal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ModalResult {
    #[default]
    None,
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
    Close,
}

impl ModalResult {
    pub const ALL: [ModalResult; 9] = [
        ModalResult::None,
        ModalResult::Ok,
        ModalResult::Cancel,
        ModalResult::Abort,
        ModalResult::Retry,
        ModalResult::Ignore,
        ModalResult::Yes,
        ModalResult::No,
        ModalResult::Close,
    ];
}

impl PropEnum for ModalResult {
    const NAMES: &'static [&'static str] = &["none", "ok", "cancel", "abort", "retry", "ignore", "yes", "no", "close"];

    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        Self::ALL.iter().position(|&r| r == self).expect("todos os valores estão em ALL")
    }
}

/// Chamada quando um form aberto com [`Ui::show_modal`] fecha.
pub type ModalCallback = Box<dyn FnOnce(&mut Ui, ModalResult)>;

pub(crate) struct OpenForm {
    pub(crate) form: ComponentId,
    pub(crate) modal: bool,
    /// Form que estava ativo quando este abriu (dono da janela modal).
    pub(crate) owner: Option<ComponentId>,
    pub(crate) result: ModalResult,
    pub(crate) on_close: Option<ModalCallback>,
}

impl Ui {
    /// Cria o form `name` pela fábrica de forms do aplicativo.
    /// Se já existe um form raiz com esse nome, devolve
    /// ele. O form é criado fechado: abra com [`show`](Self::show) ou
    /// [`show_modal`](Self::show_modal).
    pub fn create_form(&mut self, name: &str) -> Result<ComponentId> {
        let existing = self.roots().iter().copied().find(|&r| self.name(r).is_some_and(|n| n.eq_ignore_ascii_case(name)));
        if let Some(form) = existing {
            return Ok(form);
        }
        let fail = |message: String| Error::FormNotCreated { name: name.to_string(), message };
        let mut factory = self.services_mut().forms.take().ok_or_else(|| fail("nenhuma fábrica de forms (abra um projeto)".into()))?;
        let result = factory.create_form(self, name);
        if self.services().forms.is_none() {
            self.services_mut().forms = Some(factory);
        }
        result.map_err(fail)
    }

    /// Cria mais uma instância do form `name`, mesmo que ele já exista (duas janelas de
    /// cliente): cada uma com a sua instância e os
    /// handlers falando com ela. O form é criado fechado.
    ///
    /// A segunda ganha um nome livre (`frmCliente2`, como o frame repetido): os caminhos
    /// (`frmCliente2.edtNome`) e o protocolo distinguem as duas, e os handlers continuam os
    /// do `frmCliente` ([`form_class`](Self::form_class)).
    pub fn create_form_instance(&mut self, name: &str) -> Result<ComponentId> {
        let fail = |message: String| Error::FormNotCreated { name: name.to_string(), message };
        let mut factory = self.services_mut().forms.take().ok_or_else(|| fail("nenhuma fábrica de forms (abra um projeto)".into()))?;
        let result = factory.create_form(self, name);
        if self.services().forms.is_none() {
            self.services_mut().forms = Some(factory);
        }
        let form = result.map_err(fail)?;
        self.free_root_name(form);
        Ok(form)
    }

    /// Outra raiz já tem o nome de `form`: ele ganha o primeiro livre (`nome2`, `nome3`...).
    fn free_root_name(&mut self, form: ComponentId) {
        let Some(name) = self.name(form).filter(|n| !n.is_empty()).map(str::to_string) else { return };
        let taken = |ui: &Ui, candidate: &str| ui.roots().iter().any(|&r| r != form && ui.name(r).is_some_and(|n| n.eq_ignore_ascii_case(candidate)));
        if !taken(self, &name) {
            return;
        }
        let free = (2..).map(|n| format!("{name}{n}")).find(|c| !taken(self, c)).expect("sempre há um número livre");
        self.remember_form_class(form, &name);
        let _ = self.set_name(form, &free);
    }

    /// Os forms se empilham numa superfície só (o celular e o navegador; a plataforma
    /// liga): abrir e fechar um form usam a propriedade `transition` dele.
    pub fn set_stacked_forms(&mut self, stacked: bool) {
        self.stacked_forms = stacked;
    }

    /// A `transition` do form (se ele tiver a propriedade), quando os forms se empilham.
    fn form_transition(&self, form: ComponentId) -> crate::transition::Transition {
        if !self.stacked_forms {
            return crate::transition::Transition::None;
        }
        match self.get_prop(form, "transition") {
            Ok(PropValue::Enum(name)) => <crate::transition::Transition as PropEnum>::from_name(&name).unwrap_or_default(),
            _ => crate::transition::Transition::None,
        }
    }

    /// Abre o form numa janela, sem bloquear os outros (como `Show`). O primeiro form
    /// aberto é o principal: fechá-lo encerra o aplicativo. Se o form já está aberto,
    /// vem para a frente. Se ele está embutido em outro (tem pai), só fica visível.
    pub fn show(&mut self, form: ComponentId) -> Result<()> {
        self.open(form, false, None)
    }

    /// Abre o form como modal (como `ShowModal`): os forms abertos antes dele não recebem
    /// mouse nem teclado até ele fechar. Quando fechar, `on_close` recebe o
    /// [`ModalResult`] (definido por [`set_modal_result`](Self::set_modal_result) ou por
    /// um botão com `modal_result`).
    pub fn show_modal(
        &mut self,
        form: ComponentId,
        on_close: impl FnOnce(&mut Ui, ModalResult) + 'static,
    ) -> Result<()> {
        self.open(form, true, Some(Box::new(on_close)))
    }

    fn open(&mut self, form: ComponentId, modal: bool, on_close: Option<ModalCallback>) -> Result<()> {
        if !self.contains(form) {
            return Err(Error::InvalidId);
        }
        if self.parent(form).is_some() {
            if let Some(cb) = self.control_mut(form) {
                cb.set_visible(true);
            }
            return Ok(());
        }
        if let Some(pos) = self.open_forms.iter().position(|f| f.form == form) {
            // Já aberto: vem para a frente (um modal não pode ser reaberto por cima de si).
            if !modal && !self.input_blocked(form) {
                let entry = self.open_forms.remove(pos);
                self.open_forms.push(entry);
                self.activate = Some(form);
            }
            return Ok(());
        }
        if self.main_form.is_none() {
            self.main_form = Some(form);
        }
        let owner = self.open_forms.last().map(|f| f.form);
        self.open_forms.push(OpenForm { form, modal, owner, result: ModalResult::None, on_close });
        if let Some(cb) = self.control_mut(form) {
            cb.set_visible(true);
        }
        // Mostrar sem ativar (`activate_on_show` desligado; o relatório do Tabernáculo, J05).
        if !matches!(self.get_prop(form, "activate_on_show"), Ok(PropValue::Bool(false))) {
            self.activate = Some(form);
        }
        // Empilhado sobre outro: entra com a transição dele.
        let kind = self.form_transition(form);
        if kind != crate::transition::Transition::None && self.main_form != Some(form) {
            self.start_transition(None, Some(form), kind, crate::transition::TRANSITION_MS, false);
        }
        self.report(&crate::activity::Activity::FormShown { form, modal });
        // O primeiro foco (o `active_control` do form), antes do on_show (que pode trocar).
        if let Ok(crate::prop::PropValue::Component(Some(first))) = self.get_prop(form, "active_control") {
            self.set_focus(first);
        }
        self.fire(form, "on_show", &EventArgs::None);
        // Os forms embutidos (o form de um Frame) aparecem junto: recebem o on_show deles depois
        // do de fora (relatório do InsightDB, 11/11: o form dentro de um Frame não recebia).
        for inner in self.embedded_forms(form) {
            self.fire(inner, "on_show", &EventArgs::None);
        }
        self.invalidate();
        Ok(())
    }

    /// Os forms embutidos em `form` (escopos de nomes da classe `Form`, como o de um Frame), de
    /// cima para baixo.
    fn embedded_forms(&self, form: ComponentId) -> Vec<ComponentId> {
        let mut out = Vec::new();
        let mut stack: Vec<ComponentId> = self.children(form).iter().rev().copied().collect();
        while let Some(id) = stack.pop() {
            if self.is_name_scope(id) && self.class_name(id) == Some("Form") {
                out.push(id);
            }
            stack.extend(self.children(id).iter().rev().copied());
        }
        out
    }

    /// Fecha o form (como `Close`). Antes pergunta ao `on_close_query`: um handler que
    /// chamar [`cancel_close`](Self::cancel_close) impede o fechamento (devolve `false`).
    /// Depois dispara `on_close` (com o `ModalResult` como valor) e, se o form era
    /// modal, chama a função passada ao [`show_modal`](Self::show_modal). Fechar o form
    /// principal encerra o aplicativo. O form não é liberado (use [`free`](Self::free)).
    pub fn close(&mut self, form: ComponentId) -> bool {
        if !self.is_open(form) {
            // Embutido ou nunca aberto: só esconde.
            if let Some(cb) = self.control_mut(form) {
                cb.set_visible(false);
            }
            return true;
        }
        // Já perguntou e o backend ainda não respondeu: o segundo pedido é a saída de
        // emergência (o programa não responde).
        if self.questions.closes.contains(&form) {
            self.unresponsive_close(form);
            return false;
        }
        let can_close = if self.questions.closing_now.contains(&form) {
            true // a resposta adiada já aceitou
        } else {
            match self.ask(form, "on_close_query", &EventArgs::None) {
            crate::question::Asked::Answered(answer) => self.close_answer(form, &answer),
            crate::question::Asked::Deferred(token) => {
                // O backend responde depois: o form fica aberto até lá, e fecha com a resposta.
                self.questions.closes.push(form);
                self.on_answer(token, move |ui, answer| {
                    ui.questions.closes.retain(|&f| f != form);
                    if ui.close_answer(form, &answer) && ui.contains(form) {
                        ui.questions.closing_now.push(form);
                        ui.close(form);
                        ui.questions.closing_now.retain(|&f| f != form);
                    }
                });
                false
            }
            }
        };
        if !can_close {
            if let Some(f) = self.open_forms.iter_mut().find(|f| f.form == form) {
                f.result = ModalResult::None;
            }
            return false;
        }
        // O handler pode ter fechado o form (ou outros) enquanto respondia.
        let Some(pos) = self.open_forms.iter().position(|f| f.form == form) else {
            return true;
        };
        // Empilhado: sai com a transição (de volta) e fecha no fim dela.
        let kind = self.form_transition(form);
        if kind != crate::transition::Transition::None && self.closing_now != Some(form) && self.main_form != Some(form) {
            if !self.in_transition(form) || self.transitions_entering(form) {
                self.start_transition(Some(form), None, kind.reversed(), crate::transition::TRANSITION_MS, true);
            }
            return true;
        }
        let entry = self.open_forms.remove(pos);
        self.report(&crate::activity::Activity::FormClosed { form });
        self.frame_requests.borrow_mut().remove(&form);
        if let Some(cb) = self.control_mut(form) {
            cb.set_visible(false);
        }
        if let Some(state) = self.input.get_mut(&form) {
            *state = Default::default();
        }
        self.activate = entry.owner.filter(|o| self.is_open(*o)).or_else(|| self.open_forms.last().map(|f| f.form));
        let result = entry.result;
        self.fire(form, "on_close", &EventArgs::Value(PropValue::Enum(result.name().to_string())));
        if let Some(callback) = entry.on_close {
            callback(self, result);
        }
        if self.main_form == Some(form) {
            self.request_close();
        } else if matches!(self.get_prop(form, "close_action"), Ok(PropValue::Enum(ref a)) if a == "free") {
            // Liberar depois de fechar, fora do fechamento (o relatório do Tabernáculo, J21/J22).
            self.free_later(form);
        }
        self.invalidate();
        true
    }

    /// Libera o componente depois do evento em andamento (na próxima volta dos relógios): de
    /// dentro de um handler do próprio componente, o `free` direto soltaria quem está rodando.
    pub fn free_later(&mut self, id: ComponentId) {
        self.set_timeout(0, move |ui| {
            ui.free(id);
        });
    }

    /// Mostra o form com a dona escolhida (`None`: sem dona, a janela independente), em vez da
    /// do último form aberto (o relatório do Tabernáculo, J19).
    pub fn show_owned(&mut self, form: ComponentId, owner: Option<ComponentId>) -> Result<()> {
        self.show(form)?;
        self.set_form_owner(form, owner);
        Ok(())
    }

    /// Chamado de dentro de um `on_close_query`: o form continua
    /// aberto.
    pub fn cancel_close(&mut self) {
        self.refuse("");
    }

    /// A resposta do `on_close_query`: recusado (com a mensagem na caixa de
    /// erro) ou `can_close` falso mantém o form aberto.
    fn close_answer(&mut self, form: ComponentId, answer: &crate::question::Answer) -> bool {
        match answer {
            crate::question::Answer::Refused(message) => {
                if !message.is_empty() {
                    self.raise_exception(crate::Exception { message: message.clone(), path: self.path_of(form), event: "on_close_query".into() });
                }
                false
            }
            accepted => accepted.flag("can_close", true),
        }
    }

    /// O que fazer quando o usuário pede de novo para fechar um form cujo `on_close_query`
    /// ainda espera o backend (o `aylis-controls` pergunta "O programa não está
    /// respondendo. Encerrar o programa?"). Sem isso, o pedido é ignorado.
    pub fn set_unresponsive_handler(&mut self, f: impl Fn(&mut Ui, ComponentId) + 'static) {
        self.questions.unresponsive = Some(std::rc::Rc::new(f));
    }

    fn unresponsive_close(&mut self, form: ComponentId) {
        if let Some(f) = self.questions.unresponsive.clone() {
            f(self, form);
        }
    }

    /// Encerra sem esperar o backend (a saída de emergência): avisa quem cuida do backend
    /// ([`on_terminate`](Self::on_terminate): o `aylis-host` mata o processo, a ponte
    /// abandona a sessão) e fecha o aplicativo sem perguntar de novo.
    pub fn terminate(&mut self) {
        if let Some(f) = self.questions.terminate.clone() {
            f(self);
        }
        let forms: Vec<ComponentId> = self.questions.closes.drain(..).collect();
        self.questions.closing_now.extend(forms);
        self.request_close();
    }

    /// Quem cuida do backend na saída de emergência ([`terminate`](Self::terminate)).
    pub fn on_terminate(&mut self, f: impl Fn(&mut Ui) + 'static) {
        self.questions.terminate = Some(std::rc::Rc::new(f));
    }

    /// Define o resultado do form. Num form modal, um valor
    /// diferente de `None` fecha o form. De dentro de um evento (um botão com
    /// `modal_result`, um handler), o form fecha quando o evento termina:
    /// até lá um handler ainda pode voltar o resultado para `None` e cancelar.
    pub fn set_modal_result(&mut self, form: ComponentId, result: ModalResult) {
        let Some(open) = self.open_forms.iter_mut().find(|f| f.form == form) else { return };
        open.result = result;
        if !open.modal || result == ModalResult::None {
            return;
        }
        if self.call_depth > 0 {
            if !self.pending_closes.contains(&form) {
                self.pending_closes.push(form);
            }
        } else {
            self.close(form);
        }
    }

    /// Fecha os modais cujo resultado foi definido durante o evento que terminou.
    pub(crate) fn flush_closes(&mut self) {
        for form in std::mem::take(&mut self.pending_closes) {
            if self.modal_result(form) != ModalResult::None {
                self.close(form);
            }
        }
    }

    pub fn modal_result(&self, form: ComponentId) -> ModalResult {
        self.open_forms.iter().find(|f| f.form == form).map_or(ModalResult::None, |f| f.result)
    }

    /// Forms abertos, do fundo para o topo (o último é o ativo).
    pub fn open_forms(&self) -> Vec<ComponentId> {
        self.open_forms.iter().map(|f| f.form).collect()
    }

    pub fn is_open(&self, form: ComponentId) -> bool {
        self.open_forms.iter().any(|f| f.form == form)
    }

    pub fn is_modal(&self, form: ComponentId) -> bool {
        self.open_forms.iter().any(|f| f.form == form && f.modal)
    }

    /// Form que estava ativo quando `form` abriu (a janela dona, para a plataforma).
    pub fn form_owner(&self, form: ComponentId) -> Option<ComponentId> {
        self.open_forms.iter().find(|f| f.form == form).and_then(|f| f.owner)
    }

    /// Troca o dono do form aberto (`None`: janela independente, que não fica presa por cima
    /// de outra nem minimiza junto, como as janelas do navegador). Vale para a janela criada
    /// depois disto: chame logo após o `show`.
    pub fn set_form_owner(&mut self, form: ComponentId, owner: Option<ComponentId>) {
        if let Some(f) = self.open_forms.iter_mut().find(|f| f.form == form) {
            f.owner = owner;
        }
    }

    /// O form principal: o primeiro aberto (fechá-lo encerra o aplicativo).
    pub fn main_form(&self) -> Option<ComponentId> {
        self.main_form
    }

    /// Escolhe o form principal (como `Application.MainForm`).
    pub fn set_main_form(&mut self, form: ComponentId) {
        self.main_form = Some(form);
    }

    /// Existe um form modal aberto depois deste: ele não recebe mouse nem teclado.
    pub fn input_blocked(&self, form: ComponentId) -> bool {
        let Some(top_modal) = self.open_forms.iter().rposition(|f| f.modal) else { return false };
        match self.open_forms.iter().position(|f| f.form == form) {
            Some(pos) => pos < top_modal,
            None => false,
        }
    }

    /// Form que a plataforma deve trazer para a frente (o pedido é consumido).
    pub fn take_activation(&mut self) -> Option<ComponentId> {
        self.activate.take()
    }

    /// Esquece forms liberados (chamado por `free`).
    pub(crate) fn forget_forms(&mut self, freed: &[ComponentId]) {
        self.open_forms.retain(|f| !freed.contains(&f.form));
        if self.main_form.is_some_and(|m| freed.contains(&m)) {
            self.main_form = None;
        }
    }
}
