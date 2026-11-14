//! Forms com instância: cada form aberto tem a struct dele, com
//! os componentes já resolvidos e o estado do usuário, e os handlers recebem essa struct. Um
//! form aberto duas vezes (duas janelas de cliente, um frame embutido duas vezes) tem duas
//! instâncias, e cada handler fala com a sua.
//!
//! O código gerado registra a classe do form ([`Ui::register_form_class`]): ao criar o form,
//! a fábrica de forms chama [`Ui::bind_form`], que monta a instância (e recusa o form se os
//! componentes que o código espera não estão nele). Os handlers por nome são
//! [`form_handler`]: acham a instância pelo form de quem disparou.
//!
//! Um evento do mesmo form disparado dentro de um handler dele (um `next` no dataset dispara
//! o `after_scroll` na hora) fica para quando o handler terminar: a instância está emprestada
//! ao handler que roda. É o que as outras linguagens já fazem (o cliente guarda numa fila os
//! eventos que chegam enquanto espera uma resposta).

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::ui::Ui;

/// Monta a instância de um form recém-criado (`root`); `Err` diz o que não bate.
pub type Binder = Rc<dyn Fn(&mut Ui, ComponentId) -> Result<Box<dyn Any>, String>>;

/// Um handler do mesmo form que esperou o que rodava terminar.
type Deferred = Box<dyn FnOnce(&mut Ui)>;

/// A instância de um form aberto: aqui, ou emprestada a um handler que roda (com os eventos
/// que chegaram nesse meio-tempo).
enum State {
    Here(Box<dyn Any>),
    Busy(Vec<Deferred>),
}

struct Slot {
    kind: TypeId,
    /// O nome com que o form foi criado (o da classe): uma instância embutida mais de uma
    /// vez no mesmo form ganha outro nome (`fraLinha2`) e continua com os handlers dele.
    class: String,
    state: State,
}

#[derive(Default)]
pub(crate) struct Instances {
    binders: HashMap<String, Binder>,
    slots: HashMap<ComponentId, Slot>,
    /// A cópia dos componentes de cada form (sem o estado), que vale também enquanto a
    /// instância está com um handler.
    components: HashMap<ComponentId, Box<dyn Any>>,
    /// O nome de origem das segundas janelas sem instância (o backend em outra linguagem):
    /// `frmPrincipal2` continua com os handlers do `frmPrincipal`.
    renamed: HashMap<ComponentId, String>,
}

impl Ui {
    /// Registra a classe do form `name` (o código gerado faz): ao criar um form com esse
    /// nome, `bind` monta a instância dele.
    pub fn register_form_class(&mut self, name: &str, bind: impl Fn(&mut Ui, ComponentId) -> Result<Box<dyn Any>, String> + 'static) {
        self.instances.binders.insert(name.to_ascii_lowercase(), Rc::new(bind));
        // Os forms com esse nome criados antes do registro (o principal, que o aplicativo
        // cria ao abrir o projeto) ganham a instância agora; um que não bate com o código é
        // o erro do aplicativo, na caixa de erro.
        let existing: Vec<ComponentId> =
            self.components().filter(|&c| self.form_of(c) == c && self.name(c).is_some_and(|n| n.eq_ignore_ascii_case(name)) && !self.instances.slots.contains_key(&c)).collect();
        for form in existing {
            if let Err(message) = self.bind_form(form) {
                self.raise_exception(crate::exception::Exception { message, path: self.path_of(form), event: String::new() });
            }
        }
    }

    /// Monta a instância do form `form` recém-criado, se a classe dele foi registrada. `Err`
    /// diz o que não bate entre o form e o código (um componente que sumiu ou mudou de
    /// classe): quem criou não deve abrir o form.
    pub fn bind_form(&mut self, form: ComponentId) -> Result<(), String> {
        let Some(class) = self.name(form).map(str::to_string) else { return Ok(()) };
        let Some(bind) = self.instances.binders.get(&class.to_ascii_lowercase()).cloned() else { return Ok(()) };
        let instance = bind(self, form)?;
        let kind = (*instance).type_id();
        self.instances.slots.insert(form, Slot { kind, class, state: State::Here(instance) });
        Ok(())
    }

    /// O nome com que o form foi criado, se ele tem instância: o dos handlers dele, mesmo
    /// que o form tenha ganhado outro nome ao entrar mais de uma vez num pai.
    pub fn form_class(&self, form: ComponentId) -> Option<&str> {
        match self.instances.slots.get(&form) {
            Some(slot) => Some(slot.class.as_str()),
            None => self.instances.renamed.get(&form).map(String::as_str),
        }
    }

    /// Lembra o nome com que o form foi criado antes de ele ganhar outro (a segunda janela do
    /// mesmo form, veja [`Ui::create_form_instance`]): os handlers continuam os dele.
    pub(crate) fn remember_form_class(&mut self, form: ComponentId, class: &str) {
        if !self.instances.slots.contains_key(&form) {
            self.instances.renamed.insert(form, class.to_string());
        }
    }

    /// A instância do form (`None` se o form não tem uma desse tipo ou se ela está com um
    /// handler que roda agora).
    pub fn form_instance<T: 'static>(&self, form: ComponentId) -> Option<&T> {
        match &self.instances.slots.get(&form)?.state {
            State::Here(b) => b.downcast_ref(),
            State::Busy(_) => None,
        }
    }

    pub fn form_instance_mut<T: 'static>(&mut self, form: ComponentId) -> Option<&mut T> {
        match &mut self.instances.slots.get_mut(&form)?.state {
            State::Here(b) => b.downcast_mut(),
            State::Busy(_) => None,
        }
    }

    /// Guarda a cópia dos componentes do form (o `bind` gerado faz): o código fora dos
    /// handlers (ou chamado por um deles) chega aos componentes por ela mesmo enquanto a
    /// instância está emprestada a um handler, quando o [`Ui::form_instance`] dá `None`.
    pub fn set_form_components(&mut self, form: ComponentId, components: Box<dyn Any>) {
        self.instances.components.insert(form, components);
    }

    /// A cópia dos componentes do form (`None` se o form não tem uma desse tipo).
    pub fn form_components<T: Copy + 'static>(&self, form: ComponentId) -> Option<T> {
        self.instances.components.get(&form)?.downcast_ref().copied()
    }

    /// O form com instância do tipo `T` a que `id` pertence: o form dele ou, num form
    /// embutido, um dos de fora.
    fn instance_of<T: 'static>(&self, id: ComponentId) -> Option<ComponentId> {
        let kind = TypeId::of::<T>();
        let mut current = Some(self.form_of(id));
        while let Some(form) = current {
            if self.instances.slots.get(&form).is_some_and(|s| s.kind == kind) {
                return Some(form);
            }
            current = self.parent(form).map(|p| self.form_of(p));
        }
        None
    }

    /// Roda `f` com a instância `T` do form de `sender`. Se um handler do mesmo form está
    /// rodando, `f` espera ele terminar (e roda logo depois). Devolve se achou a instância.
    pub fn with_form<T: 'static>(&mut self, sender: ComponentId, f: impl FnOnce(&mut T, &mut Ui) + 'static) -> bool {
        let Some(form) = self.instance_of::<T>(sender) else { return false };
        let busy = matches!(self.instances.slots.get(&form).map(|s| &s.state), Some(State::Busy(_)));
        let path = if busy && cfg!(debug_assertions) { self.path_of(sender) } else { String::new() };
        let slot = self.instances.slots.get_mut(&form).expect("achado acima");
        let instance = match std::mem::replace(&mut slot.state, State::Busy(Vec::new())) {
            State::Here(b) => b,
            State::Busy(mut waiting) => {
                // Os automáticos (o `on_resize` que o próprio handler causa ao mexer no layout, o
                // timer, a rolagem) esperarem é o normal, e o aviso só assustava (no NoOBS, a cada
                // execução; relatado pelo autor em 07/11): avisa só os do usuário (clique, tecla...).
                let automatic = self.running_events.last().is_some_and(|e| crate::question::COALESCED_EVENTS.contains(&e.as_str()));
                if cfg!(debug_assertions) && !automatic {
                    crate::stderr_line!("aylis: evento de {path} adiado: um handler do mesmo form está rodando");
                }
                slot.state = State::Busy(Vec::new());
                // Um evento-pergunta (o `before_post` de um `post` dentro do handler) também
                // espera: quem perguntou recebe a resposta quando ele rodar.
                let token = if self.asking() { self.defer_question() } else { 0 };
                waiting.push(Box::new(move |ui: &mut Ui| {
                    if token == 0 {
                        ui.with_form::<T>(sender, f);
                    } else {
                        ui.answering(token, |ui| {
                            ui.with_form::<T>(sender, f);
                        });
                    }
                }));
                if let Some(slot) = self.instances.slots.get_mut(&form) {
                    slot.state = State::Busy(waiting);
                }
                return true;
            }
        };
        let mut instance = instance;
        // Um pânico no handler sobe até o `guarded`, que mostra a caixa de erro; antes, a
        // instância tem de voltar para o form, senão ele fica "ocupado" para sempre e todo evento
        // seguinte dele fica na fila sem rodar (achado na conferência de 08/11). O pânico segue
        // depois de o form estar de volta e a fila rodada.
        let panicked = {
            let (typed, ui) = (instance.downcast_mut::<T>(), &mut *self);
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                if let Some(typed) = typed {
                    f(typed, ui);
                }
            }))
            .err()
        };
        // O form pode ter sido liberado pelo próprio handler (fechou a janela).
        let waiting = match self.instances.slots.get_mut(&form) {
            Some(slot) => match std::mem::replace(&mut slot.state, State::Here(instance)) {
                State::Busy(w) => w,
                State::Here(_) => Vec::new(),
            },
            None => Vec::new(),
        };
        for deferred in waiting {
            deferred(self);
        }
        if let Some(payload) = panicked {
            std::panic::resume_unwind(payload);
        }
        true
    }

    /// Esquece as instâncias dos forms liberados.
    pub(crate) fn forget_instances(&mut self, doomed: &[ComponentId]) {
        if !self.instances.components.is_empty() {
            self.instances.components.retain(|form, _| !doomed.contains(form));
        }
        if !self.instances.slots.is_empty() {
            self.instances.slots.retain(|form, _| !doomed.contains(form));
        }
        if !self.instances.renamed.is_empty() {
            self.instances.renamed.retain(|form, _| !doomed.contains(form));
        }
    }
}

/// O handler por nome que chama um método da instância do form
/// (`ui.register_handler("frmClientes.salvar_click", form_handler(FrmClientes::salvar_click))`).
pub fn form_handler<T: 'static, R: HandlerResult + 'static>(method: fn(&mut T, &mut Ui, ComponentId, &EventArgs) -> R) -> impl Fn(&mut Ui, ComponentId, &EventArgs) + 'static {
    move |ui, sender, args| {
        let args = args.clone();
        if !ui.with_form::<T>(sender, move |instance, ui| method(instance, ui, sender, &args).finish(ui)) && cfg!(debug_assertions) {
            crate::stderr_line!("aylis: {} não tem a instância do form ({}): o form foi criado sem a classe registrada", ui.path_of(sender), std::any::type_name::<T>());
        }
    }
}

/// O que um handler devolve: nada, ou `Result<(), String>` nos eventos-pergunta (o
/// `before_post`, o `on_close_query`): o `Err` recusa, com a mensagem na tela
/// (e o `?` repassa).
pub trait HandlerResult {
    fn finish(self, ui: &mut Ui);
}

impl HandlerResult for () {
    fn finish(self, _ui: &mut Ui) {}
}

impl HandlerResult for Result<(), String> {
    fn finish(self, ui: &mut Ui) {
        if let Err(message) = self {
            if ui.asking() {
                ui.refuse(message);
            } else {
                // Fora de uma pergunta, o erro é um erro do handler (a caixa de erro).
                ui.raise_exception(crate::Exception { message, path: String::new(), event: String::new() });
            }
        }
    }
}
