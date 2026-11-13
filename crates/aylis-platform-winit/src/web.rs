//! Navegador: o winit cria o `<canvas>` e o aylis desenha nele com o Canvas 2D.

use std::num::NonZeroU32;
use std::rc::Rc;

use aylis_core::Color;
use aylis_web::Canvas2d;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use winit::event_loop::ActiveEventLoop;
use winit::platform::web::{WindowAttributesExtWebSys, WindowExtWebSys};
use winit::window::{Window, WindowAttributes};

use crate::Error;

pub(crate) struct WebBackend {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

pub(crate) fn create(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Result<(Rc<Window>, WebBackend), Error> {
    let attributes = attributes.with_append(true).with_focusable(true).with_prevent_default(true);
    let window = event_loop.create_window(attributes)?;
    let canvas = window.canvas().ok_or("o winit não criou o <canvas>")?;
    let ctx = canvas
        .get_context("2d")
        .map_err(js_error)?
        .ok_or("o navegador não forneceu o contexto 2d")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(js_error)?;
    // Recebe o teclado sem precisar clicar antes.
    let _ = canvas.focus();
    watch_wheel(&canvas);
    Ok((Rc::new(window), WebBackend { canvas, ctx }))
}

thread_local! {
    /// Onde estava o ponteiro na última roda (pontos do canvas), até o winit entregá-la.
    static WHEEL_AT: std::cell::Cell<Option<(f32, f32)>> = const { std::cell::Cell::new(None) };
}

/// O winit entrega a roda sem posição: o aylis usaria a do último movimento, e uma roda sem
/// movimento antes (a página acabou de abrir, o painel de teste) cairia no canto de cima. Um
/// ouvinte na captura da janela, que roda antes do winit, guarda a posição do próprio evento.
fn watch_wheel(canvas: &HtmlCanvasElement) {
    let Some(window) = web_sys::window() else { return };
    let canvas = canvas.clone();
    let on_wheel = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let get = |key: &str| js_sys::Reflect::get(&e, &key.into()).ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let rect = canvas.get_bounding_client_rect();
        WHEEL_AT.with(|w| w.set(Some(((get("clientX") - rect.left()) as f32, (get("clientY") - rect.top()) as f32))));
    });
    let _ = window.add_event_listener_with_callback_and_bool("wheel", on_wheel.as_ref().unchecked_ref(), true);
    on_wheel.forget();
}

/// A posição da roda que o winit está entregando agora (uma vez só).
pub(crate) fn wheel_position() -> Option<aylis_core::PointF> {
    WHEEL_AT.with(|w| w.take()).map(|(x, y)| aylis_core::PointF::new(x, y))
}

impl WebBackend {
    /// Pinta o quadro. Com `parts` (as cópias da rolagem e as áreas, em unidades lógicas
    /// alinhadas ao pixel), só elas: o canvas do navegador guarda a imagem do quadro anterior,
    /// a não ser que mude de tamanho (aí `paint` recebe `None` e pinta tudo). `paint` recebe
    /// as áreas.
    #[allow(clippy::type_complexity)]
    pub(crate) fn draw(
        &mut self,
        (width, height): (NonZeroU32, NonZeroU32),
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[aylis_core::RectF])>,
        paint: impl FnOnce(&mut Canvas2d<'_>, Option<&[aylis_core::RectF]>),
    ) -> Result<(), Error> {
        // O tamanho em pixels físicos (o CSS cuida do tamanho lógico na página). Mudar o
        // tamanho apaga o canvas.
        let mut parts = parts;
        if self.canvas.width() != width.get() {
            self.canvas.set_width(width.get());
            parts = None;
        }
        if self.canvas.height() != height.get() {
            self.canvas.set_height(height.get());
            parts = None;
        }
        let ctx = &self.ctx;
        let s = scale as f64;
        let pixels = |a: &aylis_core::RectF| {
            let (l, t) = ((a.x as f64 * s).floor(), (a.y as f64 * s).floor());
            let (r, b) = ((a.right() as f64 * s).ceil(), (a.bottom() as f64 * s).ceil());
            (l, t, r - l, b - t)
        };
        // As cópias: o canvas desenhado nele mesmo (o navegador copia a origem antes).
        for m in parts.map_or(&[][..], |(moves, _)| moves) {
            let (x, y, w, h) = pixels(&m.rect);
            let (dx, dy) = ((m.delta.x as f64 * s).round(), (m.delta.y as f64 * s).round());
            ctx.save();
            let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            ctx.set_global_alpha(1.0);
            ctx.begin_path();
            ctx.rect(x + dx, y + dy, w, h);
            ctx.clip();
            let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self.canvas, x, y, w, h, x + dx, y + dy, w, h);
            ctx.restore();
        }
        let whole = [aylis_core::RectF::new(0.0, 0.0, width.get() as f32 / scale, height.get() as f32 / scale)];
        let areas = parts.map_or(&whole[..], |(_, areas)| areas);
        let given = parts.map(|(_, areas)| areas);
        ctx.save();
        let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        ctx.set_global_alpha(1.0);
        ctx.begin_path();
        for a in areas {
            let (x, y, w, h) = pixels(a);
            ctx.rect(x, y, w, h);
        }
        ctx.clip();
        ctx.set_fill_style_str(&format!("rgb({},{},{})", background.r, background.g, background.b));
        for a in areas {
            let (x, y, w, h) = pixels(a);
            ctx.fill_rect(x, y, w, h);
        }
        let _ = ctx.scale(s, s);
        paint(&mut Canvas2d::new(ctx), given);
        ctx.restore();
        Ok(())
    }
}

/// Pixels da tela por pixel CSS agora (`devicePixelRatio`: muda com o zoom do navegador e
/// ao passar para outro monitor).
pub(crate) fn device_scale() -> f64 {
    web_sys::window().map(|w| w.device_pixel_ratio()).filter(|s| *s > 0.0).unwrap_or(1.0)
}

/// O tamanho do canvas em pixels da tela, medido na página agora (o do winit fica velho
/// enquanto ele não vê a troca de escala).
pub(crate) fn physical_size(window: &Window) -> winit::dpi::PhysicalSize<u32> {
    match window.canvas() {
        Some(canvas) => {
            let rect = canvas.get_bounding_client_rect();
            let s = device_scale();
            winit::dpi::PhysicalSize::new((rect.width() * s).round() as u32, (rect.height() * s).round() as u32)
        }
        None => window.inner_size(),
    }
}

fn js_error(value: impl Into<JsValue>) -> Error {
    format!("{:?}", value.into()).into()
}

/// Mostra o erro no console do navegador.
pub(crate) fn report(error: &Error) {
    web_sys::console::error_1(&format!("aylis: {error}").into());
}

thread_local! {
    /// O último texto copiado ou colado: o `get_text` é síncrono, e o navegador só entrega a
    /// área de transferência de forma assíncrona (ou no evento `paste`).
    static CLIPBOARD: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

/// A área de transferência no navegador. Copiar vai para a do sistema pelo
/// `navigator.clipboard.writeText` (o Ctrl+C é um gesto do usuário, então é aceito sem
/// perguntar); colar vem do evento `paste` (veja [`watch_paste`]).
pub(crate) struct WebClipboard;

impl aylis_core::platform::ClipboardService for WebClipboard {
    fn get_text(&mut self) -> Option<String> {
        CLIPBOARD.with(|c| Some(c.borrow().clone()).filter(|t| !t.is_empty()))
    }

    fn set_text(&mut self, text: &str) {
        self.set_content(&aylis_core::clipboard::ClipboardContent::text(text));
    }

    /// O texto e o HTML vão juntos pelo `ClipboardItem` (onde houver; senão, o
    /// `writeText`). A imagem e os formatos do app ficam na cópia do processo: o navegador só
    /// grava PNG, e de forma assíncrona.
    fn set_content(&mut self, content: &aylis_core::clipboard::ClipboardContent) {
        let text = content.text.clone().unwrap_or_default();
        CLIPBOARD.with(|c| *c.borrow_mut() = text.clone());
        let Some(window) = web_sys::window() else { return };
        let Ok(clipboard) = js_sys::Reflect::get(&window.navigator(), &"clipboard".into()) else { return };
        let method = |name: &str| js_sys::Reflect::get(&clipboard, &name.into()).ok().and_then(|f| f.dyn_into::<js_sys::Function>().ok());
        // As promessas ficam sem resposta: sem permissão, o que foi copiado ainda vale dentro
        // do app.
        if let (Some(html), Some(write)) = (&content.html, method("write")) {
            if let Some(item) = clipboard_item(&window, &[("text/plain", &text), ("text/html", html)]) {
                let _ = write.call1(&clipboard, &js_sys::Array::of1(&item));
                return;
            }
        }
        if let Some(write) = method("writeText") {
            let _ = write.call1(&clipboard, &text.into());
        }
    }
}

/// Um `ClipboardItem` com um `Blob` por formato.
fn clipboard_item(window: &web_sys::Window, parts: &[(&str, &str)]) -> Option<wasm_bindgen::JsValue> {
    let construct = |name: &str, args: &js_sys::Array| {
        let ctor = js_sys::Reflect::get(window, &name.into()).ok()?.dyn_into::<js_sys::Function>().ok()?;
        js_sys::Reflect::construct(&ctor, args).ok()
    };
    let items = js_sys::Object::new();
    for (mime, value) in parts {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"type".into(), &(*mime).into()).ok()?;
        let blob = construct("Blob", &js_sys::Array::of2(&js_sys::Array::of1(&(*value).into()), &options))?;
        js_sys::Reflect::set(&items, &(*mime).into(), &blob).ok()?;
    }
    construct("ClipboardItem", &js_sys::Array::of1(&items))
}

/// Colar: o winit cancela o Ctrl+V (o `prevent_default` das teclas), e aí o navegador não
/// dispara o `paste`. Um ouvinte na captura da janela, que roda antes do winit, deixa o
/// Ctrl+V passar direto; o `paste` traz o texto, que vira a área de transferência do app, e o
/// Ctrl+V é entregue ao form de cima (cada controle cola do seu jeito, como no desktop).
pub(crate) fn watch_paste(poster: aylis_core::Poster) {
    let Some(window) = web_sys::window() else { return };
    let on_key = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let flag = |key: &str| js_sys::Reflect::get(&e, &key.into()).ok().and_then(|v| v.as_bool()).unwrap_or(false);
        let key = js_sys::Reflect::get(&e, &"key".into()).ok().and_then(|v| v.as_string()).unwrap_or_default();
        if key.eq_ignore_ascii_case("v") && (flag("ctrlKey") || flag("metaKey")) && !flag("altKey") {
            e.stop_immediate_propagation();
        }
    });
    let _ = window.add_event_listener_with_callback_and_bool("keydown", on_key.as_ref().unchecked_ref(), true);
    on_key.forget();
    let on_paste = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let text = js_sys::Reflect::get(&e, &"clipboardData".into())
            .ok()
            .filter(|d| !d.is_undefined() && !d.is_null())
            .and_then(|data| {
                let get = js_sys::Reflect::get(&data, &"getData".into()).ok()?.dyn_into::<js_sys::Function>().ok()?;
                get.call1(&data, &"text/plain".into()).ok()?.as_string()
            })
            .unwrap_or_default();
        e.prevent_default();
        if text.is_empty() {
            return;
        }
        CLIPBOARD.with(|c| *c.borrow_mut() = text);
        poster.post(|ui| {
            let Some(top) = ui.open_forms().last().copied() else { return };
            let modifiers = aylis_core::Modifiers { ctrl: true, ..Default::default() };
            ui.key_down(top, aylis_core::KeyEvent { key: aylis_core::Key::Char('v'), modifiers, repeat: false });
        });
    });
    let _ = window.add_event_listener_with_callback_and_bool("paste", on_paste.as_ref().unchecked_ref(), true);
    on_paste.forget();
}

/// Os botões extras do mouse (voltar e avançar) sobre o app são do app (o `on_navigate` do
/// form, como no desktop): o navegador não sai da página por eles. O navegador volta no
/// soltar do botão, e é ali que se cancela.
pub(crate) fn keep_mouse_navigation() {
    let Some(window) = web_sys::window() else { return };
    let on_up = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let button = js_sys::Reflect::get(&e, &"button".into()).ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
        if button == 3.0 || button == 4.0 {
            e.prevent_default();
        }
    });
    for event in ["mouseup", "pointerup", "auxclick"] {
        let _ = window.add_event_listener_with_callback_and_bool(event, on_up.as_ref().unchecked_ref(), false);
    }
    on_up.forget();
}
