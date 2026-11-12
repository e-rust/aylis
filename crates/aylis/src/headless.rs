//! O modo sem plataforma: o `Ui` com as classes, o texto medido e o desenho fora da tela do
//! renderizador, sem janela nem `platform::run`. Para testes de tela e fotos (o relatório do
//! Tabernáculo, B04): monta o form, simula o usuário (clicar, digitar, avançar o relógio) e
//! tira a foto em PNG.
//!
//! ```no_run
//! use aylis::headless::Headless;
//!
//! # fn main() -> Result<(), aylis::Error> {
//! let mut h = Headless::new();
//! h.load_form(r#"{"aylis": 1, "root": {"class": "Form", "name": "frmTeste", "width": 320, "height": 200,
//!     "children": [{"class": "Button", "name": "btnOk", "text": "OK"}]}}"#)?;
//! h.click("btnOk")?;
//! h.advance(0.5); // os `Timer` e as animações andam pelo relógio falso
//! let png = h.render_png().expect("sem renderizador");
//! # Ok(()) }
//! ```
//!
//! De um projeto: [`Headless::open`] (a pasta do `.aylisproj`), [`Headless::bundled`] (o
//! projeto embutido) ou [`Application::headless`].
//!
//! - **Renderizador:** o do projeto, como na janela; sem projeto, o automático (Direct2D no
//!   Windows quando o app compila só com a feature `d2d`, Skia nas outras). O texto é medido
//!   pelo mesmo renderizador que desenha, então a foto bate com a tela.
//! - **Relógio:** é o falso do `Ui` ([`Ui::advance`]): nada anda sozinho, e o cursor de texto
//!   e as animações saem iguais a cada execução.
//! - **Ponteiro:** só o simulado. Depois de [`click`](Headless::click) o ponteiro fica sobre o
//!   controle (o hover aparece na foto, como na tela); [`leave`](Headless::leave) tira.
//! - **Na web** o modo não existe: o desenho fora da tela do navegador precisa do documento.

use aylis_core::{ComponentId, Key, KeyEvent, Modifiers, PointF, PointerButton, PointerEvent, Ui};

use crate::{Application, EmbeddedProject, Error};

/// O `Ui` sem janela, com o form em teste. O `ui` é público: propriedades, eventos e
/// componentes pelo caminho de sempre (`h.ui.get_prop(...)`).
pub struct Headless {
    pub ui: Ui,
    /// O form que recebe a entrada e sai na foto ([`set_form`](Self::set_form)).
    form: Option<ComponentId>,
}

impl Default for Headless {
    fn default() -> Self {
        Self::new()
    }
}

impl Headless {
    /// Um `Ui` com todas as classes ([`register_all`](crate::register_all)), o texto e o
    /// desenho fora da tela do renderizador automático. Os forms entram por
    /// [`load_form`](Self::load_form).
    pub fn new() -> Self {
        let mut ui = Ui::new();
        crate::register_all(&mut ui);
        Self::from_ui(ui, aylis_platform_winit::Renderer::Auto)
    }

    /// Um `Ui` montado pelo app (com as classes dele), mais o texto e o desenho fora da tela de
    /// `renderer`. O que já estiver posto fica.
    pub fn from_ui(mut ui: Ui, renderer: aylis_platform_winit::Renderer) -> Self {
        aylis_platform_winit::install_text_service(&mut ui, renderer);
        aylis_platform_winit::install_offscreen_service(&mut ui, renderer);
        let form = ui.main_form();
        let mut h = Self { ui, form: None };
        if let Some(f) = form {
            h.set_form(f);
        }
        h
    }

    /// O projeto de uma pasta (o caminho do `.aylisproj`), como [`Application::open`], com o
    /// form principal aberto.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        Ok(Application::open(path)?.headless())
    }

    /// O projeto embutido ([`include_project!`](crate::include_project)), como
    /// [`Application::bundled`], com o form principal aberto.
    pub fn bundled(embedded: EmbeddedProject) -> Result<Self, Error> {
        Ok(Application::bundled(embedded)?.headless())
    }

    /// Carrega um form em JSON (`{"aylis": 1, "root": {...}}`), abre e passa a usá-lo.
    pub fn load_form(&mut self, json: &str) -> Result<ComponentId, Error> {
        let form = aylis_json::load(&mut self.ui, json, None)?;
        self.set_form(form);
        Ok(form)
    }

    /// Cria (se preciso) um form do projeto pelo nome, abre e passa a usá-lo.
    pub fn create_form(&mut self, name: &str) -> Result<ComponentId, Error> {
        let form = self.ui.create_form(name)?;
        self.set_form(form);
        Ok(form)
    }

    /// O form em teste: abre (o `on_show`, o primeiro foco) e assenta o layout.
    pub fn set_form(&mut self, form: ComponentId) {
        let _ = self.ui.show(form);
        self.form = Some(form);
        self.settle();
    }

    /// O form em teste.
    pub fn form(&self) -> Option<ComponentId> {
        self.form
    }

    /// Muda o tamanho do form (a janela redimensionada): o layout e os `on_resize` rodam.
    pub fn resize(&mut self, width: f32, height: f32) {
        if let Some(cb) = self.form.and_then(|f| self.ui.control_mut(f)) {
            cb.set_size(width, height);
        }
        self.settle();
    }

    /// O componente pelo nome, no form em teste (e, se não estiver nele, em qualquer form).
    pub fn find(&self, name: &str) -> Option<ComponentId> {
        self.form.and_then(|f| self.ui.find_in(f, name)).or_else(|| self.ui.find(name))
    }

    fn require(&self, name: &str) -> Result<ComponentId, Error> {
        self.find(name).ok_or_else(|| format!("componente '{name}' não encontrado").into())
    }

    /// Clica no meio do componente com o mouse simulado: o clique passa pelo que estiver por
    /// cima, pelo foco e pelo `enabled`, como na tela. Para disparar o `on_click` direto,
    /// sem o mouse, use `ui.click(id)`.
    pub fn click(&mut self, name: &str) -> Result<(), Error> {
        let id = self.require(name)?;
        self.settle();
        let rect = self.ui.absolute_rect(id).ok_or_else(|| format!("'{name}' não tem posição"))?;
        let root = self.ui.root_of(id);
        self.click_in(root, PointF::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0));
        Ok(())
    }

    /// Clica no ponto `(x, y)` do form em teste.
    pub fn click_at(&mut self, x: f32, y: f32) {
        if let Some(root) = self.form.map(|f| self.ui.root_of(f)) {
            self.settle();
            self.click_in(root, PointF::new(x, y));
        }
    }

    fn click_in(&mut self, root: ComponentId, at: PointF) {
        let ev = PointerEvent { position: at, button: PointerButton::Primary, click_count: 1, ..Default::default() };
        self.ui.pointer_move(root, ev);
        self.ui.pointer_down(root, ev);
        self.ui.pointer_up(root, ev);
        self.settle();
    }

    /// Leva o ponteiro ao ponto `(x, y)` do form (o hover, a dica).
    pub fn move_to(&mut self, x: f32, y: f32) {
        if let Some(root) = self.form.map(|f| self.ui.root_of(f)) {
            self.ui.pointer_move(root, PointerEvent { position: PointF::new(x, y), ..Default::default() });
            self.settle();
        }
    }

    /// O ponteiro sai da janela: some o hover (para a foto sem o realce do mouse).
    pub fn leave(&mut self) {
        if let Some(root) = self.form.map(|f| self.ui.root_of(f)) {
            self.ui.pointer_leave(root);
            self.settle();
        }
    }

    /// Põe o foco no componente (como clicar num campo sem mexer no mouse).
    pub fn focus(&mut self, name: &str) -> Result<(), Error> {
        let id = self.require(name)?;
        if !self.ui.set_focus(id) {
            return Err(format!("'{name}' não recebe o foco").into());
        }
        self.settle();
        Ok(())
    }

    /// Digita o texto no controle com o foco, tecla por tecla (`\n` é o Enter).
    pub fn type_text(&mut self, text: &str) {
        let Some(root) = self.form.map(|f| self.ui.root_of(f)) else { return };
        for c in text.chars() {
            let (key, shift) = match c {
                '\n' => (Key::Enter, false),
                ' ' => (Key::Space, false),
                c => (Key::Char(c.to_lowercase().next().unwrap_or(c)), c.is_uppercase()),
            };
            let event = KeyEvent { key, modifiers: Modifiers { shift, ..Default::default() }, repeat: false };
            self.ui.key_down(root, event);
            if c != '\n' {
                self.ui.text_input(root, &c.to_string());
            }
            self.ui.key_up(root, event);
            self.settle();
        }
    }

    /// Aperta e solta uma tecla (Enter, Tab, as setas, `Key::F(9)`...).
    pub fn press(&mut self, key: Key) {
        self.press_with(KeyEvent { key, ..Default::default() });
    }

    /// Aperta e solta uma tecla com modificadores (Ctrl+S: `Key::Char('s')` com `ctrl`).
    pub fn press_with(&mut self, event: KeyEvent) {
        let Some(root) = self.form.map(|f| self.ui.root_of(f)) else { return };
        self.ui.key_down(root, event);
        self.ui.key_up(root, event);
        self.settle();
    }

    /// Avança o relógio falso: os `Timer`, o `set_timeout`, as animações e as transições andam
    /// `seconds`, sem esperar de verdade.
    pub fn advance(&mut self, seconds: f64) {
        self.ui.advance(seconds);
        self.settle();
    }

    /// Roda o que ficou na fila (eventos, `post` de outras threads) e refaz o layout até
    /// assentar, como a janela faz antes de cada quadro.
    pub fn settle(&mut self) {
        for _ in 0..8 {
            self.ui.process_events();
            if !self.ui.needs_layout() {
                break;
            }
            self.ui.layout();
        }
    }

    /// A foto do form em teste no tamanho dele, em PNG. `None` sem form ou sem o renderizador.
    pub fn render_png(&mut self) -> Option<Vec<u8>> {
        self.render_png_scaled(1.0)
    }

    /// Como [`render_png`](Self::render_png), com `scale` pixels por unidade (2.0: a tela de
    /// alta densidade).
    pub fn render_png_scaled(&mut self, scale: f32) -> Option<Vec<u8>> {
        self.settle();
        let form = self.form?;
        let (w, h) = self.ui.control(form).map(|cb| (cb.width(), cb.height()))?;
        self.ui.render_png(form, w, h, scale)
    }

    /// Grava a foto do form em teste num arquivo PNG.
    pub fn save_png(&mut self, path: impl AsRef<std::path::Path>) -> Result<(), Error> {
        let png = self.render_png().ok_or("sem o renderizador fora da tela")?;
        std::fs::write(path, png)?;
        Ok(())
    }
}

impl Application {
    /// O aplicativo sem janela, para testes e fotos ([`Headless`]): o texto e o desenho fora da
    /// tela do renderizador do projeto, com o form principal aberto.
    pub fn headless(self) -> Headless {
        let renderer = self.options().renderer;
        let main = self.main_form;
        let mut h = Headless::from_ui(self.ui, renderer);
        if h.form != Some(main) {
            h.set_form(main);
        }
        h
    }
}
