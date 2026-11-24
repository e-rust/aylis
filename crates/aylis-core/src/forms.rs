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
//! [`Ui::show_modal_in`] é o modal de uma parte da tela (a página de uma aba): o form fica
//! por cima do contêiner, com o fundo escurecido, e só ele bloqueia; as outras abas e janelas
//! continuam respondendo.
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
/// área interna e se está maximizada ou em tela cheia.
/// A plataforma atualiza quando a janela é movida ou muda de
/// tamanho; maximizada ou em tela cheia, guarda a posição e o tamanho de antes (para voltar a
/// eles; em tela cheia, a posição fica no monitor dela). Para
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
    /// Em tela cheia (no monitor da posição): reabre em tela cheia nele.
    pub full_screen: bool,
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

    /// Abre o form como modal dentro de `container` (a página de uma aba, um painel): por cima
    /// dele, centrado, com o fundo escurecido (o estilo `ModalVeil` do tema; sem ele, a
    /// sombra de sempre), e só o contêiner fica bloqueado. O mouse no resto do contêiner não
    /// passa, o teclado fica no form (o Tab dá a volta nele) e o Esc cancela (o botão com
    /// `cancel_button`, ou `Cancel`). O resto da janela, as outras abas e as outras janelas
    /// continuam respondendo.
    ///
    /// O retorno é o do [`show_modal`](Self::show_modal): `on_close` recebe o
    /// [`ModalResult`] quando o form fecha ([`set_modal_result`](Self::set_modal_result), um
    /// botão com `modal_result`, [`close`](Self::close)). O modal vai junto com o contêiner (a
    /// aba arrancada para outra janela leva ele) e fecha como `Cancel` quando o form de fora
    /// fecha. Fechado, o form volta a ser um form solto (não é liberado).
    pub fn show_modal_in(
        &mut self,
        form: ComponentId,
        container: ComponentId,
        on_close: impl FnOnce(&mut Ui, ModalResult) + 'static,
    ) -> Result<()> {
        if !self.contains(form) || !self.contains(container) {
            return Err(Error::InvalidId);
        }
        if self.parent(form).is_some() || self.is_open(form) || self.inner_modal(form).is_some() || form == container || self.is_ancestor(form, container) {
            return Err(Error::invalid("o form do show_modal_in tem de ser um form solto e fechado, fora do contêiner"));
        }
        let size = self.bounds(container).map_or((0.0, 0.0), |b| (b.width, b.height));
        let mut veil = ModalVeil::default();
        veil.base.set_rect(0.0, 0.0, size.0, size.1).set_anchors(crate::Anchors::LEFT.union(crate::Anchors::TOP).union(crate::Anchors::RIGHT).union(crate::Anchors::BOTTOM));
        let veil = self.add(veil, container);
        let _ = self.set_child_index(veil, usize::MAX);
        if let Err(e) = self.set_parent(form, Some(veil)) {
            self.free(veil);
            return Err(e);
        }
        if let Some(cb) = self.control_mut(form) {
            cb.set_align(crate::Align::Center).set_visible(true);
        }
        let root = self.root_of(container);
        let focus_before = self.focused(root);
        self.extension::<InnerModals>().0.push(InnerModal { form, veil, container, result: ModalResult::None, on_close: Some(Box::new(on_close)), focus_before });
        self.report(&crate::activity::Activity::FormShown { form, modal: true });
        // O primeiro foco: o `active_control` do form ou o primeiro da ordem do Tab nele.
        let first = match self.get_prop(form, "active_control") {
            Ok(PropValue::Component(Some(first))) => Some(first),
            _ => self.tab_order(root).into_iter().find(|&c| self.is_ancestor(form, c)),
        };
        match first {
            Some(first) => {
                self.set_focus(first);
            }
            None => {
                if focus_before.is_some_and(|f| !self.is_ancestor(form, f)) {
                    self.clear_focus(root);
                }
            }
        }
        self.fire(form, "on_show", &EventArgs::None);
        for inner in self.embedded_forms(form) {
            self.fire(inner, "on_show", &EventArgs::None);
        }
        self.invalidate();
        Ok(())
    }

    /// O form aberto pelo [`show_modal_in`](Self::show_modal_in) que tem `id` dentro (o próprio
    /// form também), o mais de dentro.
    pub fn inner_modal_of(&self, id: ComponentId) -> Option<ComponentId> {
        let modals = self.extension_ref::<InnerModals>()?;
        modals.0.iter().rev().find(|m| m.form == id || self.is_ancestor(m.form, id)).map(|m| m.form)
    }

    /// `id` fica atrás de um modal de contêiner ([`show_modal_in`](Self::show_modal_in)): está
    /// no contêiner e fora do form do modal (não recebe o foco).
    pub fn blocked_by_inner_modal(&self, id: ComponentId) -> bool {
        let Some(modals) = self.extension_ref::<InnerModals>() else { return false };
        modals.0.iter().any(|m| (m.container == id || self.is_ancestor(m.container, id)) && m.form != id && !self.is_ancestor(m.form, id))
    }

    fn inner_modal(&self, form: ComponentId) -> Option<usize> {
        self.extension_ref::<InnerModals>()?.0.iter().position(|m| m.form == form)
    }

    /// O modal de contêiner fecha: o form volta a ser solto (escondido), o véu sai, o foco
    /// volta para onde estava, e o `on_close` e a função do `show_modal_in` recebem o resultado.
    fn finish_inner_modal(&mut self, form: ComponentId, result: ModalResult) {
        let Some(pos) = self.inner_modal(form) else { return };
        let entry = self.extension::<InnerModals>().0.remove(pos);
        self.report(&crate::activity::Activity::FormClosed { form });
        if self.contains(form) {
            if let Some(cb) = self.control_mut(form) {
                cb.set_visible(false);
                cb.set_align(crate::Align::None);
            }
            let _ = self.set_parent(form, None);
        }
        if self.contains(entry.veil) {
            self.free(entry.veil);
        }
        if let Some(before) = entry.focus_before.filter(|&f| self.contains(f) && !self.blocked_by_inner_modal(f)) {
            self.set_focus(before);
        }
        if self.contains(form) {
            self.fire(form, "on_close", &EventArgs::Value(PropValue::Enum(result.name().to_string())));
        }
        if let Some(callback) = entry.on_close {
            callback(self, result);
        }
        if self.contains(form) && matches!(self.get_prop(form, "close_action"), Ok(PropValue::Enum(ref a)) if a == "free") {
            self.free_later(form);
        }
        self.invalidate();
    }

    /// Fecha o modal de contêiner (o `close` dele): pergunta ao `on_close_query` e fecha com o
    /// resultado que estiver (`Cancel` sem nenhum).
    fn close_inner_modal(&mut self, form: ComponentId) -> bool {
        let result = move |ui: &Ui| ui.inner_modal(form).and_then(|i| ui.extension_ref::<InnerModals>().map(|m| m.0[i].result)).unwrap_or_default();
        match self.ask(form, "on_close_query", &EventArgs::None) {
            crate::question::Asked::Answered(answer) => {
                if !self.close_answer(form, &answer) {
                    if let Some(i) = self.inner_modal(form) {
                        self.extension::<InnerModals>().0[i].result = ModalResult::None;
                    }
                    return false;
                }
            }
            crate::question::Asked::Deferred(token) => {
                self.on_answer(token, move |ui, answer| {
                    if ui.close_answer(form, &answer) {
                        let r = result(ui);
                        ui.finish_inner_modal(form, if r == ModalResult::None { ModalResult::Cancel } else { r });
                    } else if let Some(i) = ui.inner_modal(form) {
                        ui.extension::<InnerModals>().0[i].result = ModalResult::None;
                    }
                });
                return false;
            }
        }
        let r = result(self);
        self.finish_inner_modal(form, if r == ModalResult::None { ModalResult::Cancel } else { r });
        true
    }

    /// O form de fora fechou: os modais de contêiner dentro dele fecham como `Cancel`.
    fn cancel_inner_modals_of(&mut self, outer: ComponentId) {
        let inside: Vec<ComponentId> = self
            .extension_ref::<InnerModals>()
            .map(|m| m.0.iter().filter(|m| !self.contains(m.container) || self.root_of(m.container) == outer).map(|m| m.form).collect())
            .unwrap_or_default();
        for form in inside.into_iter().rev() {
            self.finish_inner_modal(form, ModalResult::Cancel);
        }
    }

    /// O Esc ou o Enter que ninguém usou, com o foco num modal de contêiner: o form dele (para
    /// achar o botão padrão e o de cancelar só lá dentro).
    pub(crate) fn inner_modal_scope(&self, root: ComponentId) -> Option<ComponentId> {
        self.focused(root).and_then(|f| self.inner_modal_of(f))
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
                self.active_form = Some(form);
            }
            return Ok(());
        }
        if self.main_form.is_none() {
            self.main_form = Some(form);
        }
        // O principal nunca tem dona: possuído por outro form (os abertos antes dele, um por
        // monitor), ele ficava preso por cima dela e herdava o `topmost` dela (testado em 20/11:
        // a paleta sempre por cima atrás da janela principal).
        // A dona é a janela ativa (a de onde o usuário pediu), e não a aberta por último: com
        // duas janelas, o modal aberto pela outra ficava preso a esta.
        let active = self.active_form.filter(|&a| a != form && self.open_forms.iter().any(|f| f.form == a));
        let owner = if self.main_form == Some(form) { None } else { active.or_else(|| self.open_forms.last().map(|f| f.form)) };
        self.open_forms.push(OpenForm { form, modal, owner, result: ModalResult::None, on_close });
        if let Some(cb) = self.control_mut(form) {
            cb.set_visible(true);
        }
        // Mostrar sem ativar (`activate_on_show` desligado; o relatório do Tabernáculo, J05).
        if !matches!(self.get_prop(form, "activate_on_show"), Ok(PropValue::Bool(false))) {
            self.activate = Some(form);
            self.active_form = Some(form);
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

    /// O form aberto numa janela deixa de ter janela, sem fechar: sem o `on_close_query`, o
    /// `on_close` nem o `close_action`, e continua com o estado dele (vai ser embutido em outro,
    /// como a aba de uma faixa). O principal não larga a janela (`false`), nem um modal.
    pub fn release_window(&mut self, form: ComponentId) -> bool {
        let Some(pos) = self.open_forms.iter().position(|f| f.form == form) else { return true };
        if self.main_form == Some(form) || self.open_forms[pos].modal {
            return false;
        }
        let entry = self.open_forms.remove(pos);
        self.frame_requests.borrow_mut().remove(&form);
        if let Some(state) = self.input.get_mut(&form) {
            *state = Default::default();
        }
        self.activate = entry.owner.filter(|o| self.is_open(*o)).or_else(|| self.open_forms.last().map(|f| f.form));
        self.active_form = self.activate;
        self.invalidate();
        true
    }

    /// Fecha o form (como `Close`). Antes pergunta ao `on_close_query`: um handler que
    /// chamar [`cancel_close`](Self::cancel_close) impede o fechamento (devolve `false`).
    /// Depois dispara `on_close` (com o `ModalResult` como valor) e, se o form era
    /// modal, chama a função passada ao [`show_modal`](Self::show_modal). Fechar o form
    /// principal encerra o aplicativo. O form não é liberado (use [`free`](Self::free)).
    pub fn close(&mut self, form: ComponentId) -> bool {
        if self.inner_modal(form).is_some() {
            return self.close_inner_modal(form);
        }
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
        self.active_form = self.activate;
        let result = entry.result;
        self.cancel_inner_modals_of(form);
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
        if let Some(i) = self.inner_modal(form) {
            self.extension::<InnerModals>().0[i].result = result;
            if result == ModalResult::None {
                return;
            }
            if self.call_depth > 0 {
                if !self.pending_closes.contains(&form) {
                    self.pending_closes.push(form);
                }
            } else {
                self.close(form);
            }
            return;
        }
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
        if let Some(m) = self.extension_ref::<InnerModals>().and_then(|m| m.0.iter().find(|m| m.form == form)) {
            return m.result;
        }
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
        // Já aberto, deixa de ser possuído (veja o `open`).
        self.set_form_owner(form, None);
    }

    /// Existe um form modal aberto depois deste: ele não recebe mouse nem teclado.
    pub fn input_blocked(&self, form: ComponentId) -> bool {
        self.blocking_modal(form).is_some()
    }

    /// O modal que bloqueia o form (o último aberto depois dele), se houver.
    pub fn blocking_modal(&self, form: ComponentId) -> Option<ComponentId> {
        let top_modal = self.open_forms.iter().rposition(|f| f.modal)?;
        let pos = self.open_forms.iter().position(|f| f.form == form)?;
        (pos < top_modal).then(|| self.open_forms[top_modal].form)
    }

    /// Form que a plataforma deve trazer para a frente (o pedido é consumido).
    pub fn take_activation(&mut self) -> Option<ComponentId> {
        self.activate.take()
    }

    /// Esquece forms liberados (chamado por `free`).
    pub(crate) fn forget_forms(&mut self, freed: &[ComponentId]) {
        self.open_forms.retain(|f| !freed.contains(&f.form));
        // Um modal de contêiner liberado junto (o form, o véu ou o contêiner): a função dele
        // recebe `Cancel` depois do `free`.
        let gone: Vec<InnerModal> = match self.extension_ref::<InnerModals>().is_some() {
            true => {
                let modals = &mut self.extension::<InnerModals>().0;
                let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(modals).into_iter().partition(|m| freed.contains(&m.form) || freed.contains(&m.veil) || freed.contains(&m.container));
                *modals = kept;
                gone
            }
            false => Vec::new(),
        };
        for entry in gone {
            if let Some(callback) = entry.on_close {
                self.set_timeout(0, move |ui| callback(ui, ModalResult::Cancel));
            }
        }
        // Os pedaços de um form em vários monitores (o print aberto e liberado a cada vez).
        self.form_pieces.retain(|f, _| !freed.contains(f));
        if self.main_form.is_some_and(|m| freed.contains(&m)) {
            self.main_form = None;
        }
    }
}

/// Os modais de contêiner abertos ([`Ui::show_modal_in`]), do mais velho ao mais novo.
#[derive(Default)]
pub(crate) struct InnerModals(Vec<InnerModal>);

pub(crate) struct InnerModal {
    form: ComponentId,
    /// O fundo escurecido no contêiner, com o form dentro.
    veil: ComponentId,
    container: ComponentId,
    result: ModalResult,
    on_close: Option<ModalCallback>,
    /// Onde estava o foco do form de fora (volta para lá quando o modal fecha).
    focus_before: Option<ComponentId>,
}

/// O fundo escurecido de um modal de contêiner: cobre o contêiner e fica com o mouse que cai
/// fora do form. A cor é a do estilo `ModalVeil` do tema; sem ele, a sombra de sempre (preto
/// a 32%, a mesma da `ActionSheet`).
pub struct ModalVeil {
    base: crate::ControlBase,
}

impl Default for ModalVeil {
    fn default() -> Self {
        Self { base: crate::ControlBase::new(0.0, 0.0) }
    }
}

impl crate::Component for ModalVeil {
    fn class_name(&self) -> &'static str {
        "ModalVeil"
    }

    fn control(&self) -> Option<&crate::ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut crate::ControlBase> {
        Some(&mut self.base)
    }

    fn paint(&self, canvas: &mut dyn crate::Canvas, ctx: &crate::PaintCtx<'_>) {
        let fill = if ctx.style.fill.is_transparent() { crate::Color::BLACK.with_opacity(0.32) } else { ctx.style.fill };
        canvas.fill_rect(ctx.rect, ctx.style.radius, &crate::Brush::Solid(fill));
    }

    fn pointer_down(&mut self, ctx: &mut crate::EventCtx<'_>, _event: &crate::PointerEvent) {
        // O clique no fundo não passa para o que está embaixo.
        ctx.set_handled();
    }
}
