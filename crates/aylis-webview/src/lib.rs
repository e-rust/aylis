//! # aylis-webview
//!
//! O componente [`WebView`] (o `TWebBrowser` do FMX): mostra uma página da web dentro do
//! form. O controle só descreve o que mostrar ([`Component::native_view`]); quem cria o
//! navegador é a plataforma (`aylis-platform-winit` com a feature `webview`):
//!
//! | Onde | Navegador |
//! |---|---|
//! | Windows | WebView2 (Edge, já vem no Windows 10 e 11) |
//! | macOS, iOS | WKWebView |
//! | Linux | WebKitGTK (precisa da `libwebkit2gtk-4.1`; só X11) |
//! | Android | o `WebView` do sistema, numa janela sobre o app (a `AylisActivity`) |
//! | Navegador (wasm) | `<iframe>` sobre o canvas |
//!
//! Eventos: `on_load` (página carregada, endereço em `EventArgs::Text`), `on_navigate`
//! (vai abrir um endereço), `on_blocked` (a navegação para um endereço fora de
//! `allowed_navigation` foi impedida) e `on_message` (a página mandou um texto com
//! `window.ipc.postMessage("...")` no desktop ou `parent.postMessage("...", "*")` no
//! navegador). Para mandar para a página: [`WebView::eval`].
//!
//! Segurança: só chegam ao `on_message` as mensagens das origens de `allowed_origins` (vazia:
//! a origem do `url`); a URL de quem mandou fica em `message_url` durante o evento (e vai no
//! `info` do evento para os backends de outras linguagens). No Linux e no Android a URL é a
//! da página principal, mesmo quando quem mandou é um iframe dentro dela.

use std::cell::RefCell;

use aylis_core::{
    Brush, Canvas, Component, ControlBase, EventArgs, EventCtx, NativeCommand, NativeView, PaintCtx, PointF, RectF, Stroke, TextAlign,
    TextStyle, Ui, WebMessage, properties, url_origin,
};

/// Navegador embutido. `url` abre um endereço; sem ele, `html` mostra o texto dado.
pub struct WebView {
    pub base: ControlBase,
    pub url: String,
    pub html: String,
    /// Origens que podem mandar mensagens (`https://pagamento.exemplo.com`), separadas por
    /// vírgula ou espaço; vazia, a origem do `url` (com `html`, só o próprio conteúdo).
    pub allowed_origins: String,
    /// Para onde a página pode navegar (origens ou prefixos, separados por vírgula ou
    /// espaço); vazia, qualquer lugar. Um endereço fora vira `on_blocked`.
    pub allowed_navigation: String,
    /// O atributo `sandbox` do `<iframe>` no navegador (`allow-scripts allow-forms`...).
    pub sandbox: String,
    /// Com o app rodando, enquanto a página carrega, um skeleton no lugar (blocos no tom do
    /// fundo): a vista nativa fica por cima dele quando a página aparece. Sem ele, a reserva do
    /// designer (a barra de endereço e o aviso).
    pub skeleton: bool,
    /// A URL de quem mandou a última mensagem (vale dentro do `on_message`).
    message_url: String,
    /// Muda a cada `url`/`html` novo ou [`WebView::reload`] (a plataforma recarrega).
    generation: u64,
    commands: RefCell<Vec<NativeCommand>>,
}

impl Default for WebView {
    fn default() -> Self {
        let base = ControlBase::new(480.0, 320.0);
        Self {
            base,
            url: "https://".into(),
            html: String::new(),
            allowed_origins: String::new(),
            allowed_navigation: String::new(),
            sandbox: String::new(),
            skeleton: false,
            message_url: String::new(),
            generation: 0,
            commands: RefCell::new(Vec::new()),
        }
    }
}

fn list(text: &str) -> Vec<String> {
    text.split([',', ';', ' ', '\n', '\t']).map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

impl WebView {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), ..Default::default() }
    }

    /// Abre um endereço.
    pub fn navigate(&mut self, url: impl Into<String>) {
        self.url = url.into();
        self.generation += 1;
    }

    /// Mostra um HTML (no lugar do endereço).
    pub fn load_html(&mut self, html: impl Into<String>) {
        self.url.clear();
        self.html = html.into();
        self.generation += 1;
    }

    /// Roda JavaScript na página (no desktop; no navegador só em página do mesmo site).
    pub fn eval(&self, script: impl Into<String>) {
        self.commands.borrow_mut().push(NativeCommand::Eval(script.into()));
    }

    pub fn back(&self) {
        self.commands.borrow_mut().push(NativeCommand::Back);
    }

    pub fn forward(&self) {
        self.commands.borrow_mut().push(NativeCommand::Forward);
    }

    pub fn reload(&mut self) {
        self.generation += 1;
    }

    fn sync(&mut self, name: &str) {
        if matches!(name, "url" | "html" | "sandbox") {
            self.generation += 1;
        }
    }

    /// A URL de quem mandou a última mensagem (leia dentro do `on_message`).
    pub fn message_url(&self) -> String {
        self.message_url.clone()
    }

    fn set_message_url(&mut self, _value: String) {}

    /// A mensagem vem de uma origem permitida? Sem lista, a do `url`; com `html` (sem
    /// endereço), só o próprio conteúdo (`about:`/`data:`/vazio).
    pub fn accepts_message_from(&self, url: &str) -> bool {
        let allowed = list(&self.allowed_origins);
        let origin = url_origin(url);
        if allowed.is_empty() {
            let page = if self.url.is_empty() || self.url == "https://" { String::new() } else { url_origin(&self.url) };
            return if page.is_empty() { url.is_empty() || url.starts_with("about:") || url.starts_with("data:") || origin == "null" } else { origin == page };
        }
        allowed.iter().any(|a| url_origin(a) == origin)
    }

    /// O que aparece por baixo (designer, plataforma sem navegador, página carregando).
    fn caption(&self) -> String {
        if !self.url.is_empty() && self.url != "https://" {
            self.url.clone()
        } else if !self.html.is_empty() {
            "(HTML)".into()
        } else {
            "WebView".into()
        }
    }
}

pub use aylis_core::navigation_allowed;

impl Component for WebView {
    fn class_name(&self) -> &'static str {
        "WebView"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        url: String => "Data",
        html: String => "Data",
        allowed_origins: String => "Behavior",
        allowed_navigation: String => "Behavior",
        skeleton: bool => "Appearance",
        sandbox: String => "Behavior";
        events: on_load, on_navigate, on_blocked, on_message;
        computed: message_url: String => message_url / set_message_url;
        on_set: sync
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    /// A plataforma entrega cada mensagem da página aqui: de uma origem permitida, vira
    /// `on_message` (com `message_url` preenchida); de outra, é descartada.
    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn std::any::Any) {
        let Some(m) = message.downcast_ref::<WebMessage>() else { return };
        if !self.accepts_message_from(&m.url) {
            return;
        }
        self.message_url = m.url.clone();
        ctx.fire("on_message", EventArgs::Text(m.text.clone()));
    }

    /// Para os backends de outras linguagens: a URL de quem mandou, no `info` do evento.
    fn event_info(&self, event: &str) -> Vec<(&'static str, String)> {
        if event == "on_message" { vec![("url", self.message_url.clone())] } else { Vec::new() }
    }


    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        // Rodando, com `skeleton`: blocos no tom do texto, bem apagados, enquanto a página não
        // cobre (sem animação: a vista nativa fica por cima e este desenho não sabe quando).
        if self.skeleton && !ctx.design {
            let block = Brush::Solid(theme.color("text").with_opacity(0.07));
            let pad = (r.width.min(r.height) * 0.08).clamp(8.0, 24.0);
            let inner = r.deflate(aylis_core::Bounds::all(pad));
            canvas.fill_rect(RectF::new(inner.x, inner.y, inner.width * 0.45, 14.0), 7.0, &block);
            canvas.fill_rect(RectF::new(inner.x, inner.y + 28.0, inner.width, (inner.height - 28.0).max(0.0)), 10.0, &block);
            return;
        }
        canvas.fill_rect(r, 4.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), 4.0, &Stroke::new(theme.color("border"), 1.0));
        // Barra de endereço e um globo: dá para ver o que vai abrir.
        let bar = RectF::new(r.x + 8.0, r.y + 8.0, (r.width - 16.0).max(0.0), 28.0);
        canvas.fill_rect(bar, 14.0, &Brush::Solid(theme.color("surface_2")));
        ctx.draw_icon(canvas, "link", RectF::new(bar.x + 6.0, bar.y + 4.0, 20.0, 20.0), theme.color("text_muted"));
        let small = theme.fonts.get("small").map_or(12.0, |f| f.size);
        let style = TextStyle { font: ctx.font(small), color: theme.color("text_muted"), ..Default::default() };
        canvas.draw_text(&self.caption(), RectF::new(bar.x + 32.0, bar.y, bar.width - 40.0, bar.height), &style);
        if !ctx.design {
            let style = TextStyle { h_align: TextAlign::Center, ..style };
            let note = RectF::new(r.x, r.y + r.height / 2.0 - 10.0, r.width, 20.0);
            canvas.draw_text("Navegador embutido (aparece no app em execução)", note, &style);
        }
        let _ = PointF::ZERO;
    }
}

/// Registra `WebView` (paleta "Additional").
pub fn register(ui: &mut Ui) {
    ui.register_class::<WebView>("Additional");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aylis_controls::Form;

    #[test]
    fn describes_the_native_view_and_queues_commands() {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        register(&mut ui);
        let form = ui.add_root(Form::new("f", 400.0, 300.0));
        let w = ui.add(WebView::new("https://aylis.dev"), form);
        ui.control_mut(w).unwrap().set_rect(10.0, 20.0, 200.0, 100.0);
        ui.layout();
        let slots = ui.native_views(form);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].rect, RectF::new(10.0, 20.0, 200.0, 100.0));
        let NativeView::Web { url, generation, .. } = &slots[0].view;
        assert_eq!(url, "https://aylis.dev");
        let first = *generation;

        ui.get::<WebView>(w).unwrap().eval("document.title");
        ui.set_prop(w, "url", "https://example.com").unwrap();
        let slots = ui.native_views(form);
        assert_eq!(slots[0].commands, vec![NativeCommand::Eval("document.title".into())]);
        let NativeView::Web { generation, .. } = &slots[0].view;
        assert!(*generation > first, "endereço novo recarrega");
        assert!(ui.native_views(form)[0].commands.is_empty(), "comandos são entregues uma vez");

        ui.control_mut(w).unwrap().set_visible(false);
        assert!(!ui.native_views(form)[0].visible);
        ui.set_design_mode(true);
        assert!(ui.native_views(form).is_empty(), "o designer não abre navegador");
    }

    #[test]
    fn only_allowed_origins_reach_on_message() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        register(&mut ui);
        let form = ui.add_root(Form::new("f", 400.0, 300.0));
        let w = ui.add(WebView::new("https://pagamento.exemplo.com/checkout"), form);
        let got = Rc::new(RefCell::new(Vec::new()));
        let g = got.clone();
        ui.on(w, "on_message", move |ui, sender, args| {
            let url = ui.get_prop(sender, "message_url").unwrap().to_str().unwrap().to_string();
            g.borrow_mut().push((args.text().unwrap_or_default().to_string(), url));
        });
        let send = |ui: &mut Ui, text: &str, url: &str| ui.send_message(w, &WebMessage { text: text.into(), url: url.into() });
        // Sem lista: só a origem do `url`.
        send(&mut ui, "pagamento_ok", "https://pagamento.exemplo.com/retorno");
        send(&mut ui, "pagamento_ok", "https://anuncio.exemplo.net/x");
        assert_eq!(*got.borrow(), vec![("pagamento_ok".to_string(), "https://pagamento.exemplo.com/retorno".to_string())]);
        // Com lista: as origens dela.
        ui.set_prop(w, "allowed_origins", "https://login.exemplo.com, https://pagamento.exemplo.com").unwrap();
        send(&mut ui, "logado", "https://login.exemplo.com/fim");
        assert_eq!(got.borrow().len(), 2);
        assert_eq!(got.borrow()[1].1, "https://login.exemplo.com/fim");
    }

    #[test]
    fn navigation_follows_the_list() {
        let list = |s: &str| super::list(s);
        assert!(navigation_allowed(&[], "https://qualquer.com"));
        let allowed = list("https://docs.exemplo.com, https://exemplo.com/ajuda");
        assert!(navigation_allowed(&allowed, "https://docs.exemplo.com/guia?x=1"));
        assert!(navigation_allowed(&allowed, "https://exemplo.com/ajuda/faq"));
        assert!(!navigation_allowed(&allowed, "https://exemplo.com/loja"));
        assert!(!navigation_allowed(&allowed, "https://docs.exemplo.com.evil.net/"));
        assert!(navigation_allowed(&allowed, "about:blank"));
    }
}

impl aylis_core::PlatformHooks for WebView {
    fn native_view(&self) -> Option<NativeView> {
        Some(NativeView::Web {
            url: self.url.clone(),
            html: self.html.clone(),
            generation: self.generation,
            navigation: list(&self.allowed_navigation),
            sandbox: self.sandbox.clone(),
        })
    }
    fn take_native_commands(&self) -> Vec<NativeCommand> {
        std::mem::take(&mut *self.commands.borrow_mut())
    }
}
