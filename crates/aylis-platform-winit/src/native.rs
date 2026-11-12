//! Vistas nativas sobre os controles ([`Component::native_view`](aylis_core::Component::native_view)):
//! o navegador embutido do `WebView`. No desktop e no iOS é o `wry` (WebView2 no Windows,
//! WKWebView no macOS e no iOS, WebKitGTK no Linux), uma vista filha posicionada sobre o
//! controle; no navegador, um `<iframe>` por cima do canvas; no Android, o `WebView` do
//! sistema numa janela-filha da `AylisActivity`, sobre o controle.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use aylis_core::{ComponentId, EventArgs, NativeCommand, NativeView, RectF, Ui, WebMessage};
use winit::window::Window;

/// Eventos que a página mandou, entregues ao `Ui` no loop (`on_load`, `on_navigate`,
/// `on_blocked`, `on_message`): o componente, o evento, o texto e a URL de quem mandou (na
/// mensagem).
type Events = Rc<RefCell<Vec<(ComponentId, &'static str, String, String)>>>;

#[cfg(native_webview)]
struct Hosted {
    view: wry::WebView,
    /// O ambiente com a pasta de dados (vive tanto quanto a vista).
    _context: Option<Box<wry::WebContext>>,
    generation: u64,
    rect: RectF,
    visible: bool,
    /// Para onde a página pode navegar (o handler do wry consulta na hora).
    navigation: Rc<RefCell<Vec<String>>>,
}

#[cfg(android_webview)]
struct Hosted {
    id: u64,
    /// Pixels por unidade lógica.
    scale: f32,
    generation: u64,
    rect: RectF,
    visible: bool,
}

#[cfg(web_webview)]
struct Hosted {
    frame: web_sys::HtmlIFrameElement,
    /// O canvas do form: o iframe acompanha a posição dele na página.
    canvas: web_sys::HtmlCanvasElement,
    /// Último `left;top;width;height` gravado (evita mexer no DOM à toa).
    placed: RefCell<String>,
    generation: u64,
    rect: RectF,
    /// A parte que aparece (o recorte dos pais), nas coordenadas de `rect`.
    clip: Option<RectF>,
    visible: bool,
    _listeners: Vec<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>>,
}

#[derive(Default)]
pub(crate) struct NativeViews {
    hosted: HashMap<ComponentId, Hosted>,
    /// As que não puderam ser criadas, com a página (`generation`) da tentativa.
    failed: HashMap<ComponentId, u64>,
    events: Events,
    /// A última página de cada vista (no Android, a URL das mensagens).
    #[cfg(android_webview)]
    pages: HashMap<ComponentId, String>,
}

impl NativeViews {
    /// Cria, move, recarrega e tira as vistas conforme os controles; entrega os eventos.
    /// `windows`: cada form com a janela dele, onde o conteúdo começa nela (no navegador, as
    /// janelas flutuantes ficam no mesmo canvas) e se ele aparece (coberto por um maximizado, não).
    pub(crate) fn sync(&mut self, ui: &mut Ui, windows: &[(ComponentId, Rc<Window>, aylis_core::PointF, bool)]) {
        let mut seen = HashSet::new();
        // Com o layout pendente, a visibilidade e o lugar são os do último layout: na abertura,
        // os do tamanho de projeto do form (1200 de largura), antes de a vista do celular
        // esconder o que não cabe. A vista nativa que nascesse aí carregaria a página à toa (o
        // lingote 3D da Landing baixava ~10 MB no celular, onde fica escondido).
        let settled = !ui.needs_layout();
        for (form, window, origin, shown) in windows {
            // O form com zoom: a vista nativa acompanha (posição e tamanho em pontos da janela).
            let zoom = ui.form_zoom(*form);
            // O primeiro da ordem de tabulação do form: só ele recebe o foco ao nascer (antes
            // todo WebView tomava o foco do teclado; o relatório do Tabernáculo, W01).
            let first = ui.tab_order(*form).first().copied();
            for mut slot in ui.native_views(*form) {
                let to_window = |r: RectF| RectF::new(origin.x + r.x * zoom, origin.y + r.y * zoom, r.width * zoom, r.height * zoom);
                slot.rect = to_window(slot.rect);
                slot.clip = slot.clip.map(to_window);
                slot.visible &= *shown;
                seen.insert(slot.id);
                let NativeView::Web { url, html, generation, navigation, sandbox, background, data_dir, browser_args } = &slot.view;
                match self.hosted.get_mut(&slot.id) {
                    Some(h) => {
                        set_navigation(h, navigation);
                        if h.generation != *generation {
                            h.generation = *generation;
                            set_sandbox(h, sandbox);
                            load(h, url, html);
                        }
                        if h.rect != slot.rect || cfg!(web_webview) {
                            h.rect = slot.rect;
                            #[cfg(web_webview)]
                            {
                                h.clip = slot.clip;
                            }
                            place(h);
                        }
                        if h.visible != slot.visible {
                            h.visible = slot.visible;
                            show(h);
                        }
                        for c in &slot.commands {
                            command(h, c);
                        }
                    }
                    // Escondida: só nasce quando aparece (um WebView que o app nunca mostra não
                    // custa nada). E a que falhou não tenta de novo a cada quadro: criar um
                    // WebView2 é lento e bloqueia (a tela caía para 1 quadro por segundo e a
                    // mensagem de erro se repetia); tenta de novo quando a página muda.
                    None if !slot.visible || !settled || self.failed.get(&slot.id) == Some(generation) => {}
                    None => {
                        let page = Page { url, html, navigation, sandbox, background: *background, data_dir, browser_args };
                        if let Some(mut h) = create(window, slot.id, &page, slot.rect, slot.visible, first == Some(slot.id), &self.events) {
                            h.generation = *generation;
                            for c in &slot.commands {
                                command(&h, c);
                            }
                            self.hosted.insert(slot.id, h);
                            self.failed.remove(&slot.id);
                        } else {
                            self.failed.insert(slot.id, *generation);
                        }
                    }
                }
            }
        }
        self.hosted.retain(|id, h| {
            let keep = seen.contains(id);
            if !keep {
                destroy(h);
            }
            keep
        });
        #[cfg(android_webview)]
        for (id, kind, text) in crate::android_jni::take_web_events() {
            let id = ComponentId::from_u64(id);
            let name = match kind {
                0 => "on_load",
                1 => "on_navigate",
                _ => "on_message",
            };
            // O WebView do Android não diz quem mandou: vale a página aberta (como no Linux).
            if name != "on_message" {
                self.pages.insert(id, text.clone());
            }
            let page = self.pages.get(&id).cloned().unwrap_or_default();
            self.events.borrow_mut().push((id, name, text, page));
        }
        #[cfg(all(native_webview, target_os = "linux"))]
        pump_gtk();
        let events = std::mem::take(&mut *self.events.borrow_mut());
        for (id, event, text, url) in events {
            // A mensagem vai ao componente, que confere a origem antes do `on_message`.
            if event == "on_message" {
                ui.send_message(id, &WebMessage { text, url });
            } else {
                ui.fire(id, event, &EventArgs::Text(text));
            }
        }
    }

    /// Há vistas que precisam do laço acordado (o GTK no Linux avança junto com o aylis).
    pub(crate) fn needs_pump(&self) -> bool {
        cfg!(all(native_webview, target_os = "linux")) && !self.hosted.is_empty()
    }

}

// ----- desktop: wry ----------------------------------------------------------

#[cfg(native_webview)]
fn bounds(r: RectF) -> wry::Rect {
    use wry::dpi::{LogicalPosition, LogicalSize};
    wry::Rect { position: LogicalPosition::new(r.x as f64, r.y as f64).into(), size: LogicalSize::new(r.width as f64, r.height as f64).into() }
}

/// O que uma vista nova abre (as partes da `NativeView::Web`).
struct Page<'a> {
    url: &'a str,
    html: &'a str,
    #[cfg_attr(not(native_webview), allow(dead_code))]
    navigation: &'a [String],
    #[cfg_attr(not(web_webview), allow(dead_code))]
    sandbox: &'a str,
    #[cfg_attr(not(native_webview), allow(dead_code))]
    background: Option<aylis_core::Color>,
    #[cfg_attr(not(native_webview), allow(dead_code))]
    data_dir: &'a str,
    #[cfg_attr(not(native_webview), allow(dead_code))]
    browser_args: &'a str,
}

#[cfg(native_webview)]
fn create(window: &Rc<Window>, id: ComponentId, page: &Page<'_>, rect: RectF, visible: bool, focused: bool, events: &Events) -> Option<Hosted> {
    let (e1, e2, e3) = (events.clone(), events.clone(), events.clone());
    let navigation = Rc::new(RefCell::new(page.navigation.to_vec()));
    let allowed = navigation.clone();
    // A pasta de dados (W07): um ambiente próprio, que vive com a vista.
    let mut context = (!page.data_dir.is_empty()).then(|| Box::new(wry::WebContext::new(Some(std::path::PathBuf::from(page.data_dir)))));
    let mut builder = match context.as_deref_mut() {
        Some(ctx) => wry::WebViewBuilder::new_with_web_context(ctx),
        None => wry::WebViewBuilder::new(),
    };
    if let Some(c) = page.background {
        builder = builder.with_background_color((c.r, c.g, c.b, c.a));
    }
    #[cfg(target_os = "windows")]
    if !page.browser_args.is_empty() {
        use wry::WebViewBuilderExtWindows;
        builder = builder.with_additional_browser_args(page.browser_args);
    }
    let mut builder = builder
        .with_bounds(bounds(rect))
        .with_visible(visible)
        .with_focused(focused)
        // A URL de quem mandou (no Linux, a da página principal mesmo vindo de um iframe).
        .with_ipc_handler(move |req| e1.borrow_mut().push((id, "on_message", req.body().clone(), req.uri().to_string())))
        .with_navigation_handler(move |url| {
            let ok = aylis_core::navigation_allowed(&allowed.borrow(), &url);
            e2.borrow_mut().push((id, if ok { "on_navigate" } else { "on_blocked" }, url, String::new()));
            ok
        })
        .with_on_page_load_handler(move |event, url| {
            if matches!(event, wry::PageLoadEvent::Finished) {
                e3.borrow_mut().push((id, "on_load", url, String::new()));
            }
        });
    builder = if !page.url.is_empty() { builder.with_url(page.url) } else { builder.with_html(page.html) };
    #[cfg(target_os = "linux")]
    if gtk::init().is_err() {
        aylis_core::stderr_line!("aylis: WebView: o GTK não iniciou (o WebView no Linux precisa do X11)");
        return None;
    }
    // No iOS o wry põe a vista dentro da do app (não há janela filha).
    #[cfg(target_os = "ios")]
    let built = builder.build(&**window);
    #[cfg(not(target_os = "ios"))]
    let built = builder.build_as_child(&**window);
    match built {
        Ok(view) => Some(Hosted { view, _context: context, generation: 0, rect, visible, navigation }),
        Err(e) => {
            aylis_core::stderr_line!("aylis: WebView: {e}");
            None
        }
    }
}

#[cfg(native_webview)]
fn load(h: &Hosted, url: &str, html: &str) {
    let _ = if !url.is_empty() { h.view.load_url(url) } else { h.view.load_html(html) };
}

#[cfg(native_webview)]
fn set_navigation(h: &Hosted, navigation: &[String]) {
    if *h.navigation.borrow() != navigation {
        *h.navigation.borrow_mut() = navigation.to_vec();
    }
}

/// O `sandbox` só existe no `<iframe>` do navegador.
#[cfg(native_webview)]
fn set_sandbox(_h: &Hosted, _sandbox: &str) {}

#[cfg(native_webview)]
fn place(h: &Hosted) {
    let _ = h.view.set_bounds(bounds(h.rect));
}

#[cfg(native_webview)]
fn show(h: &Hosted) {
    let _ = h.view.set_visible(h.visible);
}

#[cfg(native_webview)]
fn command(h: &Hosted, c: &NativeCommand) {
    let _ = match c {
        NativeCommand::Eval(js) => h.view.evaluate_script(js),
        NativeCommand::Back => h.view.go_back(),
        NativeCommand::Forward => h.view.evaluate_script("history.forward()"),
        NativeCommand::Reload => h.view.reload(),
    };
}

#[cfg(native_webview)]
fn destroy(_h: &Hosted) {}

/// Linux: o WebKitGTK precisa do laço do GTK andando (o aylis chama a cada volta).
#[cfg(all(native_webview, target_os = "linux"))]
fn pump_gtk() {
    while gtk::events_pending() {
        gtk::main_iteration_do(false);
    }
}

// ----- Android: o WebView do sistema na AylisActivity -----------------------------

#[cfg(android_webview)]
fn create(window: &Rc<Window>, id: ComponentId, page: &Page<'_>, rect: RectF, visible: bool, focused: bool, _events: &Events) -> Option<Hosted> {
    let _ = focused;
    let (url, html) = (page.url, page.html);
    if !crate::android_jni::has_webview() {
        static WARNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            aylis_core::stderr_line!("aylis: WebView: precisa da AylisActivity (publique pela IDE ou pelo aylis-tool)");
        }
        return None;
    }
    let h = Hosted { id: id.to_u64(), scale: window.scale_factor() as f32, generation: 0, rect, visible };
    crate::android_jni::web_create(h.id);
    load(&h, url, html);
    place(&h);
    Some(h)
}

#[cfg(android_webview)]
fn load(h: &Hosted, url: &str, html: &str) {
    crate::android_jni::web_load(h.id, url, html);
}

/// No Android a navegação não passa pelo aylis (o WebView do sistema decide): a lista vale
/// só para as mensagens. Documentado no `WebView`.
#[cfg(android_webview)]
fn set_navigation(_h: &Hosted, _navigation: &[String]) {}

#[cfg(android_webview)]
fn set_sandbox(_h: &Hosted, _sandbox: &str) {}

#[cfg(android_webview)]
fn place(h: &Hosted) {
    let s = h.scale;
    let px = |v: f32| (v * s).round() as i32;
    crate::android_jni::web_bounds(h.id, px(h.rect.x), px(h.rect.y), px(h.rect.width), px(h.rect.height), h.visible);
}

#[cfg(android_webview)]
fn show(h: &Hosted) {
    place(h);
}

#[cfg(android_webview)]
fn command(h: &Hosted, c: &NativeCommand) {
    match c {
        NativeCommand::Eval(js) => crate::android_jni::web_command(h.id, 0, js),
        NativeCommand::Back => crate::android_jni::web_command(h.id, 1, ""),
        NativeCommand::Forward => crate::android_jni::web_command(h.id, 2, ""),
        NativeCommand::Reload => crate::android_jni::web_command(h.id, 3, ""),
    }
}

#[cfg(android_webview)]
fn destroy(h: &Hosted) {
    crate::android_jni::web_command(h.id, 4, "");
}

// ----- navegador: <iframe> -------------------------------------------------------

#[cfg(web_webview)]
fn create(window: &Rc<Window>, id: ComponentId, page: &Page<'_>, rect: RectF, visible: bool, focused: bool, events: &Events) -> Option<Hosted> {
    let _ = focused;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    use winit::platform::web::WindowExtWebSys;

    let (url, html) = (page.url, page.html);
    let document = web_sys::window()?.document()?;
    let frame: web_sys::HtmlIFrameElement = document.create_element("iframe").ok()?.dyn_into().ok()?;
    if !page.sandbox.is_empty() {
        let _ = frame.set_attribute("sandbox", page.sandbox);
    }
    let style = frame.style();
    let _ = style.set_property("position", "absolute");
    let _ = style.set_property("border", "0");
    let canvas = window.canvas()?;
    // O iframe vai no corpo da página, posicionado sobre o canvas (veja `place`).
    let _ = document.body()?.append_child(&frame);
    let mut listeners = Vec::new();
    let e1 = events.clone();
    let f1 = frame.clone();
    let on_load = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        let src = f1.src();
        e1.borrow_mut().push((id, "on_load", src, String::new()));
    });
    let _ = frame.add_event_listener_with_callback("load", on_load.as_ref().unchecked_ref());
    listeners.push(on_load);
    // Mensagens da página (parent.postMessage) só deste iframe.
    let e2 = events.clone();
    let f2 = frame.clone();
    let on_message = Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let Ok(m) = e.dyn_into::<web_sys::MessageEvent>() else { return };
        let from_frame = match (m.source(), f2.content_window()) {
            (Some(s), Some(w)) => js_sys::Object::is(&s, &w),
            _ => false,
        };
        if from_frame {
            let text = m.data().as_string().unwrap_or_else(|| js_sys::JSON::stringify(&m.data()).map(String::from).unwrap_or_default());
            // A origem que o navegador garante (`null` num `srcdoc` ou sandbox sem
            // `allow-same-origin`).
            e2.borrow_mut().push((id, "on_message", text, m.origin()));
        }
    });
    let _ = web_sys::window()?.add_event_listener_with_callback("message", on_message.as_ref().unchecked_ref());
    listeners.push(on_message);
    let h = Hosted { frame, canvas, placed: RefCell::new(String::new()), generation: 0, rect, clip: None, visible, _listeners: listeners };
    load(&h, url, html);
    place(&h);
    show(&h);
    Some(h)
}

/// No navegador a navegação dentro do `<iframe>` não passa pelo aylis (o `sandbox` limita).
#[cfg(web_webview)]
fn set_navigation(_h: &Hosted, _navigation: &[String]) {}

#[cfg(web_webview)]
fn set_sandbox(h: &Hosted, sandbox: &str) {
    let _ = if sandbox.is_empty() { h.frame.remove_attribute("sandbox") } else { h.frame.set_attribute("sandbox", sandbox) };
}

#[cfg(web_webview)]
fn load(h: &Hosted, url: &str, html: &str) {
    if !url.is_empty() {
        let _ = h.frame.remove_attribute("srcdoc");
        h.frame.set_src(url);
    } else {
        let _ = h.frame.set_attribute("srcdoc", html);
    }
}

#[cfg(web_webview)]
fn place(h: &Hosted) {
    // Posição do canvas na página (ele pode estar centralizado, ou a página rolada).
    let (Some(window), b) = (web_sys::window(), h.canvas.get_bounding_client_rect()) else { return };
    let x = b.left() + window.scroll_x().unwrap_or(0.0) + h.rect.x as f64;
    let y = b.top() + window.scroll_y().unwrap_or(0.0) + h.rect.y as f64;
    // O recorte dos pais (a caixa rolada): o que passa dela não aparece por cima do resto.
    let inset = h.clip.map_or(String::from("none"), |c| {
        let r = h.rect;
        format!("inset({}px {}px {}px {}px)", (c.y - r.y).max(0.0), (r.right() - c.right()).max(0.0), (r.bottom() - c.bottom()).max(0.0), (c.x - r.x).max(0.0))
    });
    let key = format!("{x};{y};{};{};{inset}", h.rect.width, h.rect.height);
    if *h.placed.borrow() == key {
        return;
    }
    let style = h.frame.style();
    let _ = style.set_property("clip-path", &inset);
    let _ = style.set_property("left", &format!("{x}px"));
    let _ = style.set_property("top", &format!("{y}px"));
    let _ = style.set_property("width", &format!("{}px", h.rect.width));
    let _ = style.set_property("height", &format!("{}px", h.rect.height));
    *h.placed.borrow_mut() = key;
}

#[cfg(web_webview)]
fn show(h: &Hosted) {
    let _ = h.frame.style().set_property("display", if h.visible { "block" } else { "none" });
}

#[cfg(web_webview)]
fn command(h: &Hosted, c: &NativeCommand) {
    // Só funciona em página do mesmo site (a política do navegador).
    let Some(w) = h.frame.content_window() else { return };
    match c {
        NativeCommand::Eval(js) => {
            if let Ok(eval) = js_sys::Reflect::get(&w, &"eval".into()) {
                if let Ok(f) = wasm_bindgen::JsCast::dyn_into::<js_sys::Function>(eval) {
                    let _ = f.call1(&w, &js.into());
                }
            }
        }
        NativeCommand::Back => {
            let _ = w.history().map(|h| h.back());
        }
        NativeCommand::Forward => {
            let _ = w.history().map(|h| h.forward());
        }
        NativeCommand::Reload => {
            let _ = w.location().reload();
        }
    }
}

#[cfg(web_webview)]
fn destroy(h: &Hosted) {
    h.frame.remove();
}
