//! Trabalho em segundo plano ligado a um dono (um componente ou um form): quando o dono é
//! liberado (o form fechou), o resultado é descartado, em vez de rodar sobre um form que não
//! existe mais (o acesso à memória de um form já fechado).
//!
//! - [`Poster::post_to`]: de outra thread, roda na thread da tela, se o dono ainda existe;
//! - [`Ui::background`]: o trabalho numa thread e o fim na tela (o jeito documentado; na web,
//!   onde o wasm não cria threads, roda na própria thread da tela);
//! - [`Ui::spawn_local`]: uma tarefa `async` na thread da tela, cancelada com o dono (para
//!   esperar rede e arquivos sem travar, e para bibliotecas `async`). É para **esperar**, não
//!   para calcular: um cálculo pesado numa tarefa `async` trava a tela.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

use crate::component::ComponentId;
use crate::post::Poster;
use crate::ui::Ui;

impl Poster {
    /// Como [`post`](Self::post), ligado a `owner`: se ele foi liberado até lá, `job` é
    /// descartado.
    pub fn post_to(&self, owner: ComponentId, job: impl FnOnce(&mut Ui) + Send + 'static) {
        self.post(move |ui| {
            if ui.contains(owner) {
                job(ui);
            }
        });
    }
}

/// O fim de uma tarefa: a função que roda na tela com o resultado.
type Finish = Box<dyn FnOnce(&mut Ui)>;

pub(crate) struct LocalTask {
    owner: ComponentId,
    future: Pin<Box<dyn Future<Output = Finish>>>,
}

#[derive(Default)]
pub(crate) struct Tasks {
    next: u64,
    running: HashMap<u64, LocalTask>,
}

/// Acorda uma tarefa: pede à tela (de qualquer thread) para avançá-la.
struct TaskWaker {
    id: u64,
    poster: Poster,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        let id = self.id;
        self.poster.post(move |ui| ui.poll_task(id));
    }
}

impl Ui {
    /// Roda `work` numa thread e, no fim, `done` na tela com o resultado, se `owner` ainda
    /// existe (sem acessar o form
    /// fechado). Na web, onde o wasm não cria threads, `work` roda na thread da tela: funciona,
    /// mas a tela espera durante um cálculo pesado (uma espera de rede vai pelo
    /// [`spawn_local`](Self::spawn_local)).
    pub fn background<T: Send + 'static>(
        &mut self,
        owner: ComponentId,
        work: impl FnOnce() -> T + Send + 'static,
        done: impl FnOnce(&mut Ui, T) + Send + 'static,
    ) {
        let poster = self.poster();
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let value = work();
            poster.post_to(owner, move |ui| done(ui, value));
        });
        #[cfg(target_arch = "wasm32")]
        {
            let value = work();
            poster.post_to(owner, move |ui| done(ui, value));
        }
    }

    /// Uma tarefa `async` na thread da tela, ligada a `owner`: com o dono liberado, ela é
    /// cancelada (o `future` é descartado e `done` não roda). No fim, `done` recebe o
    /// resultado na tela.
    pub fn spawn_local<T: 'static>(&mut self, owner: ComponentId, future: impl Future<Output = T> + 'static, done: impl FnOnce(&mut Ui, T) + 'static) {
        let future: Pin<Box<dyn Future<Output = Finish>>> = Box::pin(async move {
            let value = future.await;
            Box::new(move |ui: &mut Ui| done(ui, value)) as Finish
        });
        let id = self.tasks.next;
        self.tasks.next += 1;
        self.tasks.running.insert(id, LocalTask { owner, future });
        self.poll_task(id);
    }

    /// Avança a tarefa `id` (a primeira vez e a cada vez que ela é acordada).
    pub(crate) fn poll_task(&mut self, id: u64) {
        let Some(mut task) = self.tasks.running.remove(&id) else { return };
        if !self.contains(task.owner) {
            return; // o dono foi liberado: a tarefa morre aqui
        }
        let waker = Waker::from(Arc::new(TaskWaker { id, poster: self.poster() }));
        let mut cx = Context::from_waker(&waker);
        let owner = task.owner;
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task.future.as_mut().poll(&mut cx)));
        match polled {
            Ok(Poll::Ready(finish)) => {
                self.guarded(owner, "spawn_local", finish);
            }
            Ok(Poll::Pending) => {
                self.tasks.running.insert(id, task);
            }
            Err(payload) => {
                let message = crate::exception::panic_message(payload.as_ref());
                self.raise_exception(crate::Exception { message, path: self.path_of(owner), event: "spawn_local".into() });
            }
        }
    }

    /// As tarefas cujo dono saiu (o `free` chama).
    pub(crate) fn drop_tasks_of(&mut self, doomed: &[ComponentId]) {
        self.tasks.running.retain(|_, t| !doomed.contains(&t.owner));
    }

    /// Quantas tarefas `async` estão esperando.
    pub fn running_tasks(&self) -> usize {
        self.tasks.running.len()
    }

    /// Baixa `url` (HTTP GET) sem travar a tela, para usar dentro do
    /// [`spawn_local`](Self::spawn_local): `let corpo = ui.http_get(url).await?`.
    pub fn http_get(&mut self, url: &str) -> CallbackFuture<Result<Vec<u8>, String>> {
        match self.services_mut().http.as_mut() {
            Some(http) => from_callback(|done| http.get(url, done)),
            None => CallbackFuture::ready(Err("sem o serviço de HTTP nesta plataforma".into())),
        }
    }

    /// O diálogo de arquivos, esperando a escolha: `let arquivos = ui.choose_files(opções).await`.
    pub fn choose_files(&mut self, options: crate::platform::FileDialogOptions) -> CallbackFuture<Vec<String>> {
        match self.services_mut().file_dialogs.as_mut() {
            Some(dialogs) => from_callback(|done| dialogs.show(options, done)),
            None => CallbackFuture::ready(Vec::new()),
        }
    }

    /// Pede as permissões, esperando a resposta do usuário.
    pub fn request_permissions(
        &mut self,
        permissions: Vec<crate::platform::Permission>,
    ) -> CallbackFuture<Vec<(crate::platform::Permission, crate::platform::PermissionState)>> {
        match self.services_mut().permissions.as_mut() {
            Some(service) => from_callback(|done| service.request(permissions, done)),
            None => CallbackFuture::ready(Vec::new()),
        }
    }

    /// Imprime, esperando o diálogo de impressão: `Ok(true)` impresso, `Ok(false)` cancelado.
    pub fn print_pages(&mut self, title: &str, pages: crate::pages::PageSet) -> CallbackFuture<Result<bool, String>> {
        match self.services_mut().printer.as_mut() {
            Some(printer) => from_callback(|done| printer.print(title, pages, done)),
            None => CallbackFuture::ready(Err("sem impressão nesta plataforma".into())),
        }
    }
}

/// Um `Future` para um serviço que responde por callback (de qualquer thread).
pub struct CallbackFuture<T> {
    shared: Arc<Mutex<(Option<T>, Option<Waker>)>>,
}

impl<T> CallbackFuture<T> {
    /// Já com a resposta (o serviço não existe nesta plataforma, por exemplo).
    pub fn ready(value: T) -> Self {
        CallbackFuture { shared: Arc::new(Mutex::new((Some(value), None))) }
    }
}

impl<T> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let Ok(mut shared) = self.shared.lock() else { return Poll::Pending };
        match shared.0.take() {
            Some(value) => Poll::Ready(value),
            None => {
                shared.1 = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

/// Liga um serviço de callback a um `Future`: `start` recebe o `done` que entrega o valor.
pub fn from_callback<T: Send + 'static>(start: impl FnOnce(Box<dyn FnOnce(T) + Send>)) -> CallbackFuture<T> {
    let shared: Arc<Mutex<(Option<T>, Option<Waker>)>> = Arc::new(Mutex::new((None, None)));
    let answer = shared.clone();
    start(Box::new(move |value| {
        let waker = answer.lock().ok().and_then(|mut s| {
            s.0 = Some(value);
            s.1.take()
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }));
    CallbackFuture { shared }
}
