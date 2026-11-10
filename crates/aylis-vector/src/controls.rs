//! Os componentes: `Lottie` (animação) e `Svg` (imagem vetorial).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use aylis_core::{Canvas, Color, Component, ControlBase, ImageFit, PaintCtx, RectF, SizeF, Stroke, properties};

use crate::geom::Affine;
use crate::lottie::Animation;
use crate::svg::SvgImage;

/// `class_name`, `control` e `control_mut` de um controle com `base`.
macro_rules! leaf_control {
    ($name:literal) => {
        fn class_name(&self) -> &'static str {
            $name
        }

        fn control(&self) -> Option<&ControlBase> {
            Some(&self.base)
        }

        fn control_mut(&mut self) -> Option<&mut ControlBase> {
            Some(&mut self.base)
        }
    };
}

/// `"pacote:nome"` vira `assets/lottie/<pacote>/<nome>.json` (as animações que a IDE
/// copia para o projeto); o resto é um caminho dentro do projeto.
pub fn lottie_path(source: &str) -> String {
    match source.split_once(':') {
        Some((pack, name)) if !pack.is_empty() && !name.contains('/') && !source.contains('/') => {
            format!("assets/lottie/{pack}/{name}.json")
        }
        _ => source.to_string(),
    }
}

/// O retângulo (na tela) da área `content` encaixada em `r` com `fit`.
fn fit_rect(fit: ImageFit, content: SizeF, r: RectF) -> RectF {
    let (src, dest) = fit.place(content, r);
    let m = Affine::rect_to_rect(src, dest);
    let a = m.apply(aylis_core::PointF::new(0.0, 0.0));
    let b = m.apply(aylis_core::PointF::new(content.width, content.height));
    RectF::new(a.x, a.y, b.x - a.x, b.y - a.y)
}

fn placeholder(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
    if ctx.design {
        let color = ctx.theme.color("text_muted").with_opacity(0.6);
        canvas.stroke_rect(ctx.rect.inflate(-0.5), 4.0, &Stroke::new(color, 1.0).dashed(vec![4.0, 3.0]));
    }
}

// ===========================================================================
// Lottie
// ===========================================================================

/// Animação Lottie (o JSON do After Effects/Bodymovin), como o `TLottieAnimation`. Desenha
/// vetor: nítida em qualquer tamanho, em qualquer renderizador.
///
/// - `source`: `"noto:fogo"` (pacote de animações da IDE; o app leva só as citadas) ou um
///   caminho no projeto (`assets/lottie/carregando.json`);
/// - `playing`, `repeat`, `speed` (1 = normal) e `position` (segundos; onde fica parada);
/// - pelo código: [`Lottie::set_animation`] com uma [`Animation`] lida de qualquer lugar.
pub struct Lottie {
    pub base: ControlBase,
    pub source: String,
    pub playing: bool,
    pub repeat: bool,
    pub speed: f32,
    pub position: f32,
    pub fit: ImageFit,
    animation: Option<Rc<Animation>>,
    loaded: RefCell<Option<(String, Option<Rc<Animation>>)>>,
    /// O instante (relógio da pintura) que corresponde ao começo da animação.
    started: Cell<Option<f64>>,
}

impl Default for Lottie {
    fn default() -> Self {
        let mut base = ControlBase::new(96.0, 96.0);
        base.set_hit_test(false);
        Self {
            base,
            source: String::new(),
            playing: true,
            repeat: true,
            speed: 1.0,
            position: 0.0,
            fit: ImageFit::Contain,
            animation: None,
            loaded: RefCell::new(None),
            started: Cell::new(None),
        }
    }
}

impl Lottie {
    pub fn new(source: impl Into<String>) -> Self {
        Self { source: source.into(), ..Default::default() }
    }

    /// Usa uma animação lida pelo código (vale mais que `source`).
    pub fn set_animation(&mut self, animation: Option<Animation>) {
        self.animation = animation.map(Rc::new);
        self.started.set(None);
    }

    /// Lê o JSON e usa a animação.
    pub fn load_json(&mut self, json: &[u8]) -> Result<(), String> {
        self.set_animation(Some(Animation::parse(json)?));
        Ok(())
    }

    pub fn play(&mut self) {
        self.playing = true;
        self.started.set(None);
    }

    /// Para onde está (continua dali com `play`).
    pub fn pause(&mut self, now: f64) {
        if let Some(start) = self.started.get() {
            self.position = ((now - start) as f32) * self.speed;
        }
        self.playing = false;
        self.started.set(None);
    }

    /// Para e volta ao começo.
    pub fn stop(&mut self) {
        self.playing = false;
        self.position = 0.0;
        self.started.set(None);
    }

    fn sync(&mut self, name: &str) {
        if matches!(name, "playing" | "position" | "speed" | "source") {
            self.started.set(None);
        }
    }

    fn current(&self, ctx: &PaintCtx<'_>) -> Option<Rc<Animation>> {
        if let Some(a) = &self.animation {
            return Some(a.clone());
        }
        if self.source.is_empty() {
            return None;
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(s, _)| *s != self.source) {
            let anim = ctx.resource(&lottie_path(&self.source)).and_then(|d| Animation::parse(&d).ok()).map(Rc::new);
            *loaded = Some((self.source.clone(), anim));
        }
        loaded.as_ref().and_then(|(_, a)| a.clone())
    }
}

impl Component for Lottie {
    leaf_control!("Lottie");

    properties! {
        source: String => "Appearance",
        playing: bool => "Behavior",
        repeat: bool => "Behavior",
        speed: f32 => "Behavior",
        position: f32 => "Behavior",
        fit: ImageFit => "Appearance";
        on_set: sync
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let Some(anim) = self.current(ctx) else {
            placeholder(canvas, ctx);
            return;
        };
        let seconds = if self.playing && self.speed != 0.0 {
            let start = self.started.get().unwrap_or_else(|| {
                let s = ctx.time - (self.position / self.speed) as f64;
                self.started.set(Some(s));
                s
            });
            ((ctx.time - start) as f32) * self.speed
        } else {
            self.position
        };
        let (w, h) = anim.size();
        let dest = fit_rect(self.fit, SizeF::new(w, h), ctx.rect);
        canvas.save();
        canvas.clip_rect(ctx.rect);
        anim.render(canvas, anim.frame_at(seconds, self.repeat), dest);
        canvas.restore();
        if self.playing && (self.repeat || seconds < anim.duration()) {
            ctx.request_frame();
        }
    }
}

// ===========================================================================
// Svg
// ===========================================================================

/// Imagem SVG desenhada como vetor (o `TPath` com um arquivo inteiro). `source` é o
/// arquivo no projeto (`assets/images/logo.svg`); `color` é o `currentColor` (sem ela,
/// a cor de texto do tema: ícones em SVG acompanham o tema claro e o escuro).
///
/// Para mapas: `highlight` pinta os elementos com esses `id` (separados por `;`) com
/// `highlight_color` (sem ela, a cor de destaque do tema) e `zoom_to` mostra só a área do
/// elemento com esse `id` (o município dentro do mapa do estado).
pub struct Svg {
    pub base: ControlBase,
    pub source: String,
    pub color: Option<Color>,
    pub fit: ImageFit,
    pub highlight: String,
    pub highlight_color: Option<Color>,
    pub zoom_to: String,
    image: Option<Rc<SvgImage>>,
    loaded: RefCell<Option<(String, Option<Rc<SvgImage>>)>>,
}

impl Default for Svg {
    fn default() -> Self {
        let mut base = ControlBase::new(64.0, 64.0);
        base.set_hit_test(false);
        Self {
            base,
            source: String::new(),
            color: None,
            fit: ImageFit::Contain,
            highlight: String::new(),
            highlight_color: None,
            zoom_to: String::new(),
            image: None,
            loaded: RefCell::new(None),
        }
    }
}

impl Svg {
    pub fn new(source: impl Into<String>) -> Self {
        Self { source: source.into(), ..Default::default() }
    }

    /// Usa um SVG dado pelo código (texto), no lugar de `source`.
    pub fn set_svg(&mut self, text: &str) -> Result<(), String> {
        self.image = Some(Rc::new(SvgImage::parse(text)?));
        Ok(())
    }

    fn current(&self, ctx: &PaintCtx<'_>) -> Option<Rc<SvgImage>> {
        if let Some(i) = &self.image {
            return Some(i.clone());
        }
        if self.source.is_empty() {
            return None;
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(s, _)| *s != self.source) {
            let image = ctx.resource(&self.source).and_then(|d| shared_svg(&d));
            *loaded = Some((self.source.clone(), image));
        }
        loaded.as_ref().and_then(|(_, i)| i.clone())
    }
}

/// O SVG lido de um arquivo, dividido entre os `Svg` que mostram o mesmo conteúdo (o mapa
/// do estado aparece inteiro e com zoom no município): lido uma vez só.
fn shared_svg(data: &[u8]) -> Option<Rc<SvgImage>> {
    use std::hash::{Hash, Hasher};
    thread_local! {
        static LOADED: RefCell<Vec<(u64, std::rc::Weak<SvgImage>)>> = const { RefCell::new(Vec::new()) };
    }
    let mut h = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut h);
    let key = h.finish();
    LOADED.with(|l| {
        let mut l = l.borrow_mut();
        l.retain(|(_, w)| w.strong_count() > 0);
        if let Some(image) = l.iter().find(|(k, _)| *k == key).and_then(|(_, w)| w.upgrade()) {
            return Some(image);
        }
        let image = Rc::new(SvgImage::parse(std::str::from_utf8(data).ok()?).ok()?);
        l.push((key, Rc::downgrade(&image)));
        Some(image)
    })
}

impl Component for Svg {
    leaf_control!("Svg");

    properties! {
        source: String => "Appearance",
        color: Option<Color> => "Appearance",
        fit: ImageFit => "Appearance",
        highlight: String => "Appearance",
        highlight_color: Option<Color> => "Appearance",
        zoom_to: String => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let Some(image) = self.current(ctx) else {
            placeholder(canvas, ctx);
            return;
        };
        // zoom_to: a área do elemento com uma folga (a de um ponto ganha um tamanho mínimo).
        let view = match image.bounds_of(self.zoom_to.trim()).filter(|_| !self.zoom_to.trim().is_empty()) {
            Some(b) => {
                let full = image.view_box();
                let pad = (b.width.max(b.height) * 0.08).max(full.width.max(full.height) * 0.005);
                RectF::new(b.x - pad, b.y - pad, b.width + 2.0 * pad, b.height + 2.0 * pad)
            }
            None => image.view_box(),
        };
        let dest = fit_rect(self.fit, SizeF::new(view.width, view.height), ctx.rect);
        let color = self.color.unwrap_or(ctx.style.text);
        let highlight: Vec<&str> = self.highlight.split(';').map(str::trim).filter(|s| !s.is_empty()).collect();
        let highlight_color = self.highlight_color.unwrap_or_else(|| ctx.theme.color("primary"));
        canvas.save();
        canvas.clip_rect(ctx.rect);
        // Guardado como imagem: um mapa com centenas de áreas só é redesenhado quando o
        // tamanho, a área, o destaque ou as cores mudam (repintar a tela não o refaz).
        let key = {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            (Rc::as_ptr(&image) as usize, color, highlight_color, &highlight).hash(&mut h);
            for v in [view.x, view.y, view.width, view.height, dest.x - ctx.rect.x, dest.y - ctx.rect.y, dest.width, dest.height] {
                v.to_bits().hash(&mut h);
            }
            h.finish()
        };
        if !canvas.begin_cached(key, ctx.rect) {
            image.render_visible(canvas, view, dest, Some(ctx.rect), color, &highlight, highlight_color);
            canvas.end_cached();
        }
        canvas.restore();
    }
}
