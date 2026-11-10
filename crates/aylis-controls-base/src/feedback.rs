//! Retorno e progresso: `Snackbar` (aviso com ação), `ToastStack` (avisos empilhados no
//! canto, [`notify`]), `SkeletonLoader` (esqueleto enquanto
//! carrega), `ProgressCircle` (progresso redondo), `EmptyState` (tela vazia), `Pagination`,
//! `Stepper` (etapas) e `Timeline` (linha do tempo).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, GradientStop, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerEvent, RectF, Stroke, TextAlign, Ui, properties,
};

use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

fn accent(ctx: &PaintCtx<'_>) -> Color {
    let v = ctx.variant();
    ctx.theme.color(match v {
        "success" | "warning" | "danger" | "info" => v,
        _ => "primary",
    })
}

// ===========================================================================
// Snackbar
// ===========================================================================

/// O que roda ao clicar na ação de uma mensagem.
pub type SnackAction = Box<dyn FnMut(&mut Ui)>;

/// Aviso no rodapé com uma ação opcional (o `v-snackbar`): "Item apagado · DESFAZER".
/// Aparece com `active` (ou [`Snackbar::show`]) e some sozinho depois de `timeout` ms
/// (0 = fica). `on_action` no clique da ação; `on_close` quando cada mensagem some.
///
/// Fila (o `v-snackbar-queue`): [`Snackbar::enqueue`] com uma mensagem já na tela deixa
/// a nova esperando; ela aparece quando a atual some. Para um aviso rápido sem colocar o
/// componente no form, veja [`snackbar`] (que também usa a fila).
pub struct Snackbar {
    pub base: ControlBase,
    pub text: String,
    pub action: String,
    pub timeout: u32,
    pub active: bool,
    action_rect: Cell<RectF>,
    queue: std::collections::VecDeque<(String, String, Option<SnackAction>)>,
    current: Option<SnackAction>,
    /// Criado por [`snackbar`]: sai do form quando a fila acaba.
    transient: bool,
}

impl Default for Snackbar {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 48.0);
        base.set_cursor(CursorShape::Default);
        Self {
            base,
            text: "Item apagado".into(),
            action: "Desfazer".into(),
            timeout: 4000,
            active: false,
            action_rect: Cell::new(RectF::default()),
            queue: Default::default(),
            current: None,
            transient: false,
        }
    }
}

impl Snackbar {
    /// Mostra com outro texto (e reinicia o tempo).
    pub fn show(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.active = true;
    }

    /// Mostra agora ou, se já há uma mensagem na tela, põe na fila. `on_action` roda no
    /// clique da ação desta mensagem.
    pub fn enqueue(&mut self, text: impl Into<String>, action: impl Into<String>, on_action: Option<SnackAction>) {
        let (text, action) = (text.into(), action.into());
        if self.active {
            self.queue.push_back((text, action, on_action));
        } else {
            self.text = text;
            self.action = action;
            self.current = on_action;
            self.active = true;
        }
    }

    /// Quantas mensagens esperam a vez.
    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    /// A mensagem atual sai; a próxima da fila (se houver) entra.
    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.active {
            return;
        }
        ctx.fire("on_close", EventArgs::None);
        match self.queue.pop_front() {
            Some((text, action, on_action)) => {
                self.text = text;
                self.action = action;
                self.current = on_action;
            }
            None => {
                self.active = false;
                self.current = None;
                if self.transient {
                    let id = ctx.id;
                    ctx.ui.poster().post(move |ui| {
                        ui.free(id);
                    });
                }
            }
        }
        ctx.repaint();
    }
}

impl Component for Snackbar {
    leaf_control!("Snackbar");

    properties! {
        text: String => "Appearance",
        action: String => "Appearance",
        timeout: u32 as Duration => "Behavior",
        active: bool => "Behavior";
        events: on_action, on_close
    }

    fn timer_interval(&self) -> Option<f64> {
        (self.active && self.timeout > 0).then(|| f64::from(self.timeout) / 1000.0)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        self.close(ctx);
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.active && !self.action.is_empty() && self.action_rect.get().contains(event.position) {
            if let Some(mut run) = self.current.take() {
                run(ctx.ui);
            }
            ctx.fire("on_action", EventArgs::None);
            self.close(ctx);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !self.active && !ctx.design {
            return;
        }
        let r = ctx.rect;
        // Fundo escuro no tema claro e claro no escuro (o "inverse surface" do Material).
        let (bg, fg) = if ctx.theme.dark { (Color::rgb(0xE6, 0xE1, 0xE5), Color::rgb(0x31, 0x30, 0x33)) } else { (Color::rgb(0x31, 0x30, 0x33), Color::rgb(0xF4, 0xEF, 0xF4)) };
        canvas.draw_shadow(r, 6.0, &aylis_core::Shadow::new(Color::rgba(0, 0, 0, 60), 12.0, 0.0, 4.0));
        canvas.fill_rect(r, 6.0, &Brush::Solid(bg));
        let mut t = ctx.text_style();
        t.color = fg;
        let mut right = r.right() - 16.0;
        if !self.action.is_empty() {
            let mut a = t.clone();
            // A cor de destaque "invertida": escura sobre o fundo claro do tema escuro.
            a.color = if ctx.theme.dark {
                ctx.theme.color("primary").lerp(Color::BLACK, 0.5)
            } else {
                ctx.theme.colors.get("primary_inverse").copied().unwrap_or(Color::rgb(0xD0, 0xBC, 0xFF))
            };
            a.font.weight = FontWeight::SemiBold;
            let label = ctx.ui.tr(&self.action).to_uppercase();
            let w = canvas.measure_text(&label, &a).width + 16.0;
            let ar = RectF::new(right - w, r.y + 6.0, w, r.height - 12.0);
            a.h_align = TextAlign::Center;
            canvas.draw_text(&label, ar, &a);
            self.action_rect.set(ar);
            right = ar.x - 8.0;
        }
        canvas.draw_text(&self.text, RectF::new(r.x + 16.0, r.y, (right - r.x - 16.0).max(0.0), r.height), &t);
    }
}

/// Mostra um snackbar no rodapé do form da frente, sem colocar o componente no form: some
/// sozinho em 4 s; `on_action` (se houver ação) roda no clique. Com outro já na tela, a
/// mensagem entra na fila dele. Devolve o snackbar.
pub fn snackbar(ui: &mut Ui, text: &str, action: &str, on_action: impl FnMut(&mut Ui) + 'static) -> Option<aylis_core::ComponentId> {
    let form = ui.open_forms().last().copied().or_else(|| ui.roots().first().copied())?;
    let existing = ui.children(form).iter().copied().find(|&c| ui.get::<Snackbar>(c).is_some_and(|s| s.transient && s.active));
    if let Some(id) = existing {
        if let Some(bar) = ui.get_mut::<Snackbar>(id) {
            bar.enqueue(text, action, Some(Box::new(on_action)));
        }
        return Some(id);
    }
    let (w, h) = ui.bounds(form).map_or((360.0, 640.0), |b| (b.width, b.height));
    let width = (w - 32.0).clamp(200.0, 480.0);
    let mut bar = Snackbar { transient: true, ..Default::default() };
    bar.enqueue(text, action, Some(Box::new(on_action)));
    bar.base.set_rect((w - width) / 2.0, h - 48.0 - 24.0, width, 48.0);
    Some(ui.add(bar, form))
}

// ===========================================================================
// ToastStack
// ===========================================================================

/// O tipo de um aviso: a cor do cartão.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

struct ToastEntry {
    id: u64,
    title: String,
    message: String,
    kind: ToastKind,
    born: f64,
    expires: f64,
    /// Fechado (pelo clique ou pelo prazo): sai com a animação e depois some.
    leaving: Option<f64>,
}

/// Avisos que não bloqueiam, empilhados no canto de cima à direita do form (o "toast"):
/// título, mensagem e a cor do tipo. Cada um some no prazo (ou no clique); um aviso com
/// o mesmo título de um que está na tela só renova o prazo dele. Entra deslizando e
/// esmaecendo. Normalmente não vai no form: [`notify`] cria a pilha na primeira vez.
pub struct ToastStack {
    pub base: ControlBase,
    entries: RefCell<Vec<ToastEntry>>,
    /// Onde cada cartão foi desenhado (o clique só vale neles; fora, passa para baixo).
    cards: RefCell<Vec<(u64, RectF)>>,
    next_id: Cell<u64>,
}

impl Default for ToastStack {
    fn default() -> Self {
        let mut base = ControlBase::new(380.0, 600.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, entries: RefCell::new(Vec::new()), cards: RefCell::new(Vec::new()), next_id: Cell::new(1) }
    }
}

/// Entrada e saída, em segundos.
const TOAST_FADE: f64 = 0.2;
/// Largura máxima de um cartão e o espaço entre eles.
const TOAST_WIDTH: f32 = 380.0;
const TOAST_GAP: f32 = 8.0;

impl ToastStack {
    /// Mostra um aviso (ou renova o de mesmo título) por `ttl_ms`. Devolve o número dele.
    pub fn show(&self, now: f64, title: &str, message: &str, kind: ToastKind, ttl_ms: u32) -> u64 {
        let expires = now + ttl_ms as f64 / 1000.0;
        let mut entries = self.entries.borrow_mut();
        if let Some(e) = entries.iter_mut().find(|e| e.leaving.is_none() && e.title == title) {
            e.expires = expires;
            e.message = message.to_string();
            return e.id;
        }
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        entries.push(ToastEntry { id, title: title.into(), message: message.into(), kind, born: now, expires, leaving: None });
        id
    }

    /// Fecha um aviso (com a animação de saída).
    pub fn dismiss(&self, now: f64, id: u64) {
        if let Some(e) = self.entries.borrow_mut().iter_mut().find(|e| e.id == id && e.leaving.is_none()) {
            e.leaving = Some(now);
        }
    }

    /// Quantos avisos estão na tela (sem contar os que estão saindo).
    pub fn count(&self) -> usize {
        self.entries.borrow().iter().filter(|e| e.leaving.is_none()).count()
    }

    fn colors(ctx: &PaintCtx<'_>, kind: ToastKind) -> (Color, Color, Color, Color) {
        let t = ctx.theme;
        let on = |token: &str, fallback: Color| t.colors.get(token).copied().unwrap_or(fallback);
        match kind {
            ToastKind::Info => (t.color("surface"), on("border_2", t.color("border")), t.color("text"), t.color("text_muted")),
            ToastKind::Success => (t.color("success"), t.color("success"), t.color("on_success"), t.color("on_success").with_opacity(0.85)),
            ToastKind::Error => (t.color("danger"), t.color("danger"), t.color("on_danger"), t.color("on_danger").with_opacity(0.85)),
            ToastKind::Warning => {
                let text = on("on_warning", Color::rgb(0x1C, 0x19, 0x17));
                (t.color("warning"), on("warning_border", t.color("warning")), text, text.with_opacity(0.8))
            }
        }
    }
}

impl Component for ToastStack {
    leaf_control!("ToastStack");

    fn contains_point(&self, local: PointF, _size: aylis_core::SizeF) -> bool {
        self.cards.borrow().iter().any(|(_, r)| r.contains(local))
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.cards.borrow().iter().find(|(_, r)| r.contains(event.position)).map(|(id, _)| *id);
        if let Some(id) = hit {
            self.dismiss(ctx.ui.time(), id);
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let now = ctx.time;
        let r = ctx.rect;
        // O prazo vencido vira saída; a saída acabada sai da lista.
        {
            let mut entries = self.entries.borrow_mut();
            for e in entries.iter_mut().filter(|e| e.leaving.is_none() && now >= e.expires) {
                e.leaving = Some(now);
            }
            entries.retain(|e| e.leaving.is_none_or(|t| now - t < TOAST_FADE));
        }
        let entries = self.entries.borrow();
        let mut cards = Vec::new();
        let mut y = r.y;
        let mut next_wake = f64::INFINITY;
        for e in entries.iter() {
            let (fill, border, text, text2) = Self::colors(ctx, e.kind);
            let mut title = ctx.text_style();
            title.font.weight = FontWeight::SemiBold;
            title.color = text;
            title.wrap = true;
            let mut body = ctx.text_style();
            body.font.size = (body.font.size - 1.0).max(9.0);
            body.color = text2;
            body.wrap = true;
            // Largura: o texto mais largo (até o máximo); altura: as linhas que couberem.
            let max_text = TOAST_WIDTH.min(r.width) - 28.0;
            let tw = if e.title.is_empty() { 0.0 } else { canvas.measure_text(&e.title, &title).width };
            let bw = if e.message.is_empty() { 0.0 } else { canvas.measure_text(&e.message, &body).width };
            let text_w = tw.max(bw).min(max_text);
            let lines = |w: f32| if w <= 0.0 { 0.0 } else { (w / max_text).ceil().max(1.0) };
            let th = lines(tw) * title.font.size * 1.35;
            let bh = lines(bw) * body.font.size * 1.4;
            let h = 20.0 + th + bh + if th > 0.0 && bh > 0.0 { 2.0 } else { 0.0 };
            let w = text_w + 28.0;
            // Entrada: desce 12 px e aparece; saída: some.
            let k = match e.leaving {
                Some(t) => 1.0 - ((now - t) / TOAST_FADE).clamp(0.0, 1.0),
                None => ((now - e.born) / TOAST_FADE).clamp(0.0, 1.0),
            } as f32;
            if k < 1.0 {
                ctx.request_frame();
            }
            next_wake = next_wake.min(e.expires);
            let card = RectF::new(r.right() - w, y - 12.0 * (1.0 - k), w, h);
            canvas.save();
            canvas.save_layer(k);
            canvas.draw_shadow(card, 8.0, &aylis_core::Shadow { color: Color::rgb(0, 0, 0).with_opacity(0.35), blur: 24.0, offset: PointF::new(0.0, 8.0), spread: 0.0 });
            canvas.fill_rect(card, 8.0, &Brush::Solid(fill));
            canvas.stroke_rect(card.inflate(-0.5), 8.0, &Stroke::new(border, 1.0));
            let mut ty = card.y + 10.0;
            if th > 0.0 {
                canvas.draw_text(&e.title, RectF::new(card.x + 14.0, ty, text_w, th), &title);
                ty += th + 2.0;
            }
            if bh > 0.0 {
                canvas.draw_text(&e.message, RectF::new(card.x + 14.0, ty, text_w, bh), &body);
            }
            canvas.restore();
            canvas.restore();
            cards.push((e.id, RectF::new(card.x - r.x, card.y - r.y, card.width, card.height)));
            y += (h + TOAST_GAP) * k.max(if e.leaving.is_some() { k } else { 1.0 });
        }
        *self.cards.borrow_mut() = cards;
        if next_wake.is_finite() {
            ctx.request_frame_at(next_wake);
        }
    }
}

/// Mostra um aviso no canto de cima à direita do form da frente (cria a pilha na primeira
/// vez): `title` em negrito, `message` embaixo, a cor de `kind`, por `ttl_ms`. Com outro
/// aviso de mesmo título na tela, só renova o prazo dele. Clicar fecha.
pub fn notify(ui: &mut Ui, title: &str, message: &str, kind: ToastKind, ttl_ms: u32) -> Option<aylis_core::ComponentId> {
    let form = ui.open_forms().last().copied().or_else(|| ui.roots().first().copied())?;
    let id = match ui.children(form).iter().copied().find(|&c| ui.get::<ToastStack>(c).is_some()) {
        Some(id) => id,
        None => {
            let (w, h) = ui.bounds(form).map_or((800.0, 600.0), |b| (b.width, b.height));
            let mut stack = ToastStack::default();
            let width = TOAST_WIDTH.min(w - 32.0).max(120.0);
            stack.base.set_rect(w - width - 16.0, 16.0, width, (h - 32.0).max(48.0));
            stack.base.set_anchors(aylis_core::Anchors::TOP.union(aylis_core::Anchors::RIGHT).union(aylis_core::Anchors::BOTTOM));
            ui.add(stack, form)
        }
    };
    // Por cima do resto do form.
    let _ = ui.set_child_index(id, usize::MAX);
    let now = ui.time();
    if let Some(stack) = ui.get::<ToastStack>(id) {
        stack.show(now, title, message, kind, ttl_ms);
    }
    ui.request_repaint();
    Some(id)
}

// ===========================================================================
// SkeletonLoader
// ===========================================================================

/// Esqueleto do conteúdo enquanto carrega (o `v-skeleton-loader`), com brilho passando.
/// `layout` lista as peças separadas por vírgula: `avatar`, `heading`, `text`, `text@3`
/// (três linhas), `image`, `button`, `chip`, `list-item` (avatar e duas linhas), `card`
/// (imagem, título e texto), `table-row`.
pub struct SkeletonLoader {
    pub base: ControlBase,
    pub layout: String,
    pub animated: bool,
}

impl Default for SkeletonLoader {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 180.0);
        base.set_hit_test(false);
        Self { base, layout: "card".into(), animated: true }
    }
}

impl Component for SkeletonLoader {
    leaf_control!("SkeletonLoader");

    properties! {
        layout: String => "Appearance",
        animated: bool => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let base = ctx.theme.color("text").with_opacity(0.10);
        let mut shapes: Vec<(RectF, f32)> = Vec::new();
        let mut y = r.y;
        let w = r.width;
        let line = |y: &mut f32, frac: f32, h: f32, shapes: &mut Vec<(RectF, f32)>, x: f32, width: f32| {
            shapes.push((RectF::new(x, *y, width * frac, h), h / 2.0));
            *y += h + 10.0;
        };
        for token in self.layout.split(',').map(str::trim).filter(|t| !t.is_empty()) {
            let (name, count) = token.split_once('@').map_or((token, 1), |(n, c)| (n, c.parse().unwrap_or(1)));
            for _ in 0..count {
                match name {
                    "avatar" => {
                        shapes.push((RectF::new(r.x, y, 40.0, 40.0), 20.0));
                        y += 50.0;
                    }
                    "heading" => line(&mut y, 0.55, 20.0, &mut shapes, r.x, w),
                    "text" => line(&mut y, 0.92, 12.0, &mut shapes, r.x, w),
                    "image" => {
                        shapes.push((RectF::new(r.x, y, w, 120.0), 6.0));
                        y += 130.0;
                    }
                    "button" => {
                        shapes.push((RectF::new(r.x, y, 96.0, 36.0), 6.0));
                        y += 46.0;
                    }
                    "chip" => {
                        shapes.push((RectF::new(r.x, y, 72.0, 30.0), 15.0));
                        y += 40.0;
                    }
                    "list-item" => {
                        shapes.push((RectF::new(r.x, y, 40.0, 40.0), 20.0));
                        shapes.push((RectF::new(r.x + 52.0, y + 4.0, (w - 52.0) * 0.6, 12.0), 6.0));
                        shapes.push((RectF::new(r.x + 52.0, y + 24.0, (w - 52.0) * 0.85, 10.0), 5.0));
                        y += 52.0;
                    }
                    "table-row" => {
                        for c in 0..4 {
                            let cw = w / 4.0;
                            shapes.push((RectF::new(r.x + c as f32 * cw + 4.0, y + 8.0, cw - 16.0, 12.0), 6.0));
                        }
                        y += 32.0;
                    }
                    _ => {
                        // card
                        shapes.push((RectF::new(r.x, y, w, 110.0), 6.0));
                        y += 120.0;
                        line(&mut y, 0.55, 18.0, &mut shapes, r.x, w);
                        line(&mut y, 0.92, 12.0, &mut shapes, r.x, w);
                        line(&mut y, 0.75, 12.0, &mut shapes, r.x, w);
                    }
                }
            }
        }
        canvas.save();
        canvas.clip_rect(r);
        for (s, radius) in &shapes {
            canvas.fill_rect(*s, *radius, &Brush::Solid(base));
        }
        if self.animated {
            // Uma faixa clara passa da esquerda para a direita a cada 1,5 s.
            let t = (ctx.time % 1.5 / 1.5) as f32;
            let x = r.x - r.width * 0.5 + t * r.width * 2.0;
            let shine = Brush::Linear {
                start: PointF::new(x - 80.0, 0.0),
                end: PointF::new(x + 80.0, 0.0),
                stops: vec![
                    GradientStop::new(0.0, Color::TRANSPARENT),
                    GradientStop::new(0.5, ctx.theme.color("surface").with_opacity(0.55)),
                    GradientStop::new(1.0, Color::TRANSPARENT),
                ],
            };
            for (s, radius) in &shapes {
                canvas.fill_rect(*s, *radius, &shine);
            }
            ctx.request_frame();
        }
        canvas.restore();
    }
}

// ===========================================================================
// ProgressCircle
// ===========================================================================

/// Progresso redondo (o `v-progress-circular`): `value` de 0 a 100 com o número no meio
/// (`show_value`), ou girando sem fim com `indeterminate`.
pub struct ProgressCircle {
    pub base: ControlBase,
    pub value: f32,
    pub indeterminate: bool,
    pub thickness: f32,
    pub show_value: bool,
}

impl Default for ProgressCircle {
    fn default() -> Self {
        let mut base = ControlBase::new(56.0, 56.0);
        base.set_hit_test(false);
        Self { base, value: 65.0, indeterminate: false, thickness: 5.0, show_value: true }
    }
}

impl Component for ProgressCircle {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("ProgressCircle");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        value: f32 => "Data",
        indeterminate: bool => "Appearance",
        thickness: f32 => "Appearance",
        show_value: bool => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let side = ctx.rect.width.min(ctx.rect.height);
        let w = self.thickness.max(1.0);
        let r = (side - w) / 2.0;
        let c = PointF::new(ctx.rect.width / 2.0, ctx.rect.height / 2.0);
        let color = accent(ctx);
        let ring = RectF::new(c.x - r - w / 2.0, c.y - r - w / 2.0, 2.0 * r + w, 2.0 * r + w);
        canvas.stroke_ellipse(ring, &Stroke::new(color.with_opacity(0.18), w));
        let (start, sweep) = if self.indeterminate {
            let t = ctx.time as f32;
            ((t * 300.0) % 360.0, 40.0 + 220.0 * (0.5 + 0.5 * (t * 2.2).sin()))
        } else {
            (-90.0, 360.0 * (self.value / 100.0).clamp(0.0, 1.0))
        };
        if sweep > 0.1 {
            let mut arc = Path::new();
            arc.arc(c, r, r, start, sweep);
            canvas.stroke_path(&arc, &Stroke::new(color, w).round());
        }
        if self.indeterminate {
            ctx.request_frame();
        } else if self.show_value && side >= 32.0 {
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = (side * 0.26).max(9.0);
            canvas.draw_text(&format!("{:.0}", self.value.clamp(0.0, 100.0)), RectF::new(0.0, 0.0, ctx.rect.width, ctx.rect.height), &t);
        }
    }
}

// ===========================================================================
// EmptyState
// ===========================================================================

/// Tela vazia (o `v-empty-state`): ícone grande, título, texto e um botão de ação
/// (`on_action`). "Nenhum resultado", "Sem conexão", "Comece criando um item".
pub struct EmptyState {
    pub base: ControlBase,
    pub icon: String,
    pub title: String,
    pub text: String,
    pub action: String,
    action_rect: Cell<RectF>,
}

impl Default for EmptyState {
    fn default() -> Self {
        Self {
            base: ControlBase::new(320.0, 260.0),
            icon: "search".into(),
            title: "Nada por aqui".into(),
            text: "Nenhum item encontrado. Tente outra busca.".into(),
            action: "Limpar filtros".into(),
            action_rect: Cell::new(RectF::default()),
        }
    }
}

impl Component for EmptyState {
    leaf_control!("EmptyState");

    properties! {
        icon: String => "Appearance",
        title: String => "Appearance",
        text: String => "Appearance",
        action: String => "Appearance";
        events: on_action
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if !self.action.is_empty() && self.action_rect.get().contains(event.position) {
            ctx.fire("on_action", EventArgs::None);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let muted = ctx.theme.color("text_muted");
        let mut y = r.y + (r.height - 220.0).max(0.0) / 2.0;
        if !self.icon.is_empty() {
            let side = 64.0;
            canvas.fill_ellipse(RectF::new(r.x + (r.width - side - 24.0) / 2.0, y, side + 24.0, side + 24.0), &Brush::Solid(ctx.theme.color("surface_2")));
            ctx.draw_icon(canvas, &self.icon, RectF::new(r.x + (r.width - side) / 2.0 + 0.0, y + 12.0, side, side), muted);
            y += side + 36.0;
        }
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.font.size += 4.0;
        t.font.weight = FontWeight::SemiBold;
        canvas.draw_text(&self.title, RectF::new(r.x, y, r.width, 26.0), &t);
        y += 30.0;
        let mut b = ctx.text_style();
        b.h_align = TextAlign::Center;
        b.color = muted;
        b.wrap = true;
        canvas.draw_text(&self.text, RectF::new(r.x + 16.0, y, r.width - 32.0, 40.0), &b);
        y += 48.0;
        if !self.action.is_empty() {
            let mut a = ctx.text_style();
            a.h_align = TextAlign::Center;
            a.color = ctx.theme.color("on_primary");
            let w = canvas.measure_text(&self.action, &a).width + 32.0;
            let ar = RectF::new(r.x + (r.width - w) / 2.0, y, w, 36.0);
            canvas.fill_rect(ar, 18.0, &Brush::Solid(accent(ctx)));
            canvas.draw_text(&self.action, ar, &a);
            self.action_rect.set(ar);
        }
    }
}

// ===========================================================================
// Pagination
// ===========================================================================

/// Paginação (o `v-pagination`): `page` de 1 a `length`, com no máximo `total_visible`
/// botões (o resto vira "…"). `on_change` com a página nova.
pub struct Pagination {
    pub base: ControlBase,
    pub page: i64,
    pub length: i64,
    pub total_visible: u32,
    buttons: RefCell<Vec<(RectF, i64)>>,
}

impl Default for Pagination {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 36.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, page: 1, length: 10, total_visible: 7, buttons: RefCell::new(Vec::new()) }
    }
}

impl Pagination {
    /// Os números mostrados (0 = "…").
    pub fn visible_pages(&self) -> Vec<i64> {
        let (n, p) = (self.length.max(1), self.page.clamp(1, self.length.max(1)));
        let max = self.total_visible.max(5) as i64;
        if n <= max {
            return (1..=n).collect();
        }
        if p <= max - 3 {
            // Perto do começo: 1 2 3 4 5 … 20
            let mut out: Vec<i64> = (1..=max - 2).collect();
            out.extend([0, n]);
            out
        } else if p >= n - (max - 4) {
            // Perto do fim: 1 … 16 17 18 19 20
            let mut out = vec![1, 0];
            out.extend(n - (max - 3)..=n);
            out
        } else {
            // No meio: 1 … 5 6 7 … 20
            let side = (max - 5) / 2;
            let mut out = vec![1, 0];
            out.extend(p - side..=p + side);
            out.extend([0, n]);
            out
        }
    }

    fn go(&mut self, ctx: &mut EventCtx<'_>, page: i64) {
        let page = page.clamp(1, self.length.max(1));
        if page != self.page {
            self.page = page;
            ctx.fire("on_change", EventArgs::Value(page.into()));
            ctx.repaint();
        }
    }
}

impl Component for Pagination {
    leaf_control!("Pagination");

    properties! {
        page: i64 => "Data",
        length: i64 => "Data",
        total_visible: u32 => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.buttons.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, p)| *p);
        if let Some(p) = hit {
            self.go(ctx, p);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let to = match event.key {
            Key::Left => self.page - 1,
            Key::Right => self.page + 1,
            Key::Home => 1,
            Key::End => self.length,
            _ => return,
        };
        self.go(ctx, to);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let pages = self.visible_pages();
        let gap = 4.0;
        let fit = (ctx.rect.width + gap) / (pages.len() as f32 + 2.0) - gap;
        let h = ctx.rect.height.min(40.0).min(fit).max(16.0);
        let mut buttons = Vec::new();
        let total = (pages.len() as f32 + 2.0) * (h + gap) - gap;
        let mut x = ((ctx.rect.width - total) / 2.0).max(0.0);
        let y = (ctx.rect.height - h) / 2.0;
        let color = accent(ctx);
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        let muted = ctx.theme.color("text_muted");
        let mut button = |canvas: &mut dyn Canvas, x: f32, label: &str, icon: &str, target: Option<i64>, current: bool| {
            let r = RectF::new(x, y, h, h);
            if current {
                canvas.fill_rect(r, h / 2.0, &Brush::Solid(color));
            }
            let fg = if current { ctx.theme.color("on_primary") } else if target.is_none() { muted } else { ctx.theme.color("text") };
            if icon.is_empty() {
                let mut tt = t.clone();
                tt.color = fg;
                canvas.draw_text(label, r, &tt);
            } else {
                ctx.draw_icon(canvas, icon, r.inflate(-h * 0.25), fg);
            }
            if let Some(p) = target {
                buttons.push((r, p));
            }
        };
        let prev = (self.page > 1).then_some(self.page - 1);
        button(canvas, x, "", "chevron-left", prev, false);
        x += h + gap;
        for p in &pages {
            if *p == 0 {
                button(canvas, x, "…", "", None, false);
            } else {
                button(canvas, x, &p.to_string(), "", Some(*p), *p == self.page);
            }
            x += h + gap;
        }
        let next = (self.page < self.length).then_some(self.page + 1);
        button(canvas, x, "", "chevron-right", next, false);
        *self.buttons.borrow_mut() = buttons;
    }
}

// ===========================================================================
// Stepper
// ===========================================================================

/// Etapas de um processo (o `v-stepper`): `items` com os nomes, `step` a atual (0 é a
/// primeira); as anteriores ficam com ✓. Horizontal ou `vertical`. Clicar numa etapa já
/// feita volta para ela (`on_change`).
pub struct Stepper {
    pub base: ControlBase,
    pub items: String,
    pub step: i64,
    pub vertical: bool,
    hits: RefCell<Vec<RectF>>,
}

impl Default for Stepper {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 64.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, items: "Dados;Endereço;Pagamento;Confirmação".into(), step: 1, vertical: false, hits: RefCell::new(Vec::new()) }
    }
}

impl Stepper {
    pub fn next(&mut self) {
        self.step = (self.step + 1).min(split(&self.items).len() as i64);
    }

    pub fn back(&mut self) {
        self.step = (self.step - 1).max(0);
    }
}

impl Component for Stepper {
    leaf_control!("Stepper");

    properties! {
        items: String => "Data",
        step: i64 => "Data",
        vertical: bool => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.hits.borrow().iter().position(|r| r.contains(event.position));
        if let Some(i) = hit.map(|i| i as i64).filter(|i| *i < self.step) {
            self.step = i;
            ctx.fire("on_change", EventArgs::Value(i.into()));
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let items = split(&self.items);
        if items.is_empty() {
            return;
        }
        let color = accent(ctx);
        let muted = ctx.theme.color("text_muted");
        let border = ctx.theme.color("border");
        let d = 28.0;
        let n = items.len();
        let mut hits = Vec::new();
        let mut t = ctx.text_style();
        let circle = |canvas: &mut dyn Canvas, c: PointF, i: usize| {
            let r = RectF::new(c.x - d / 2.0, c.y - d / 2.0, d, d);
            let done = (i as i64) < self.step;
            let current = i as i64 == self.step;
            let fill = if done || current { color } else { ctx.theme.color("surface_2") };
            canvas.fill_ellipse(r, &Brush::Solid(fill));
            let fg = if done || current { ctx.theme.color("on_primary") } else { muted };
            if done {
                ctx.draw_icon(canvas, "check", r.inflate(-6.0), fg);
            } else {
                let mut tt = ctx.text_style();
                tt.h_align = TextAlign::Center;
                tt.color = fg;
                tt.font.weight = FontWeight::SemiBold;
                canvas.draw_text(&(i + 1).to_string(), r, &tt);
            }
        };
        if self.vertical {
            let gap = (ctx.rect.height / n as f32).max(d + 8.0);
            for (i, item) in items.iter().enumerate() {
                let c = PointF::new(d / 2.0 + 2.0, i as f32 * gap + d / 2.0);
                if i + 1 < n {
                    canvas.draw_line(PointF::new(c.x, c.y + d / 2.0 + 4.0), PointF::new(c.x, c.y + gap - d / 2.0 - 4.0), &Stroke::new(border, 1.0));
                }
                circle(canvas, c, i);
                t.color = if i as i64 > self.step { muted } else { ctx.theme.color("text") };
                t.font.weight = if i as i64 == self.step { FontWeight::SemiBold } else { FontWeight::Normal };
                let tr = RectF::new(d + 12.0, c.y - d / 2.0, ctx.rect.width - d - 12.0, d);
                canvas.draw_text(item, tr, &t);
                hits.push(RectF::new(0.0, c.y - d / 2.0, ctx.rect.width, d));
            }
        } else {
            let w = ctx.rect.width / n as f32;
            let cy = d / 2.0 + 4.0;
            for (i, item) in items.iter().enumerate() {
                let c = PointF::new(w * (i as f32 + 0.5), cy);
                if i + 1 < n {
                    let line = if (i as i64) < self.step { color } else { border };
                    canvas.draw_line(PointF::new(c.x + d / 2.0 + 6.0, cy), PointF::new(c.x + w - d / 2.0 - 6.0, cy), &Stroke::new(line, if (i as i64) < self.step { 2.0 } else { 1.0 }));
                }
                circle(canvas, c, i);
                t.h_align = TextAlign::Center;
                t.color = if i as i64 > self.step { muted } else { ctx.theme.color("text") };
                t.font.weight = if i as i64 == self.step { FontWeight::SemiBold } else { FontWeight::Normal };
                canvas.draw_text(item, RectF::new(c.x - w / 2.0, cy + d / 2.0 + 4.0, w, 20.0), &t);
                hits.push(RectF::new(c.x - w / 2.0, 0.0, w, ctx.rect.height));
            }
        }
        *self.hits.borrow_mut() = hits;
    }
}

// ===========================================================================
// Timeline
// ===========================================================================

/// Linha do tempo (o `v-timeline`): `items` separados por `;`, cada um
/// `título|quando|texto` (as partes depois do título são opcionais). Pontos na cor da
/// variante; o primeiro em destaque.
pub struct Timeline {
    pub base: ControlBase,
    pub items: String,
    pub item_height: f32,
}

impl Default for Timeline {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 240.0);
        base.set_hit_test(false);
        Self {
            base,
            items: "Pedido entregue|hoje, 14:20|Recebido por Ana;Saiu para entrega|hoje, 08:05;Pedido enviado|ontem, 17:40|Transportadora XPTO;Pagamento aprovado|ontem, 10:12".into(),
            item_height: 60.0,
        }
    }
}

impl Component for Timeline {
    leaf_control!("Timeline");

    properties! {
        items: String => "Data",
        item_height: f32 => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let items = split(&self.items);
        let color = accent(ctx);
        let border = ctx.theme.color("border");
        let x = 10.0;
        let h = self.item_height.max(24.0);
        for (i, item) in items.iter().enumerate() {
            let mut parts = item.split('|');
            let (title, when, text) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
            let y = i as f32 * h;
            if i + 1 < items.len() {
                canvas.draw_line(PointF::new(x, y + 16.0), PointF::new(x, y + h + 2.0), &Stroke::new(border, 2.0));
            }
            let dot = RectF::new(x - 6.0, y + 2.0, 12.0, 12.0);
            if i == 0 {
                canvas.fill_ellipse(dot.inflate(3.0), &Brush::Solid(color.with_opacity(0.2)));
                canvas.fill_ellipse(dot, &Brush::Solid(color));
            } else {
                canvas.fill_ellipse(dot, &Brush::Solid(ctx.theme.color("surface")));
                canvas.stroke_ellipse(dot, &Stroke::new(color, 2.0));
            }
            let mut t = ctx.text_style();
            t.font.weight = FontWeight::SemiBold;
            let tx = x + 18.0;
            let tw = ctx.rect.width - tx;
            canvas.draw_text(title, RectF::new(tx, y - 2.0, tw, 20.0), &t);
            if !when.is_empty() {
                let mut m = ctx.text_style();
                m.color = ctx.theme.color("text_muted");
                m.font.size = (m.font.size * 0.85).round();
                m.h_align = TextAlign::Trailing;
                canvas.draw_text(when, RectF::new(tx, y - 2.0, tw, 20.0), &m);
            }
            if !text.is_empty() {
                let mut b = ctx.text_style();
                b.color = ctx.theme.color("text_muted");
                canvas.draw_text(text, RectF::new(tx, y + 18.0, tw, 20.0), &b);
            }
        }
    }
}


// ===========================================================================
// ActionSheet
// ===========================================================================

type PickAction = Box<dyn FnOnce(&mut Ui, Option<usize>)>;

/// A lista de ações por cima do form (o "action sheet" do iOS, a "bottom sheet" de
/// ações do Material). Feita por [`action_sheet`].
pub struct ActionSheet {
    pub base: ControlBase,
    pub title: String,
    pub actions: Vec<String>,
    /// Ação destrutiva (em vermelho), se houver.
    pub destructive: Option<usize>,
    pub cancel: String,
    rects: RefCell<Vec<(RectF, Option<usize>)>>,
    on_pick: Option<PickAction>,
}

impl ActionSheet {
    fn finish(&mut self, ctx: &mut EventCtx<'_>, choice: Option<usize>) {
        if let Some(pick) = self.on_pick.take() {
            pick(ctx.ui, choice);
        }
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| {
            ui.free(id);
        });
    }
}

impl Component for ActionSheet {
    leaf_control!("ActionSheet");

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.rects.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, c)| *c);
        match hit {
            Some(choice) => self.finish(ctx, choice),
            // Fora das ações: cancela.
            None => self.finish(ctx, None),
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Escape {
            self.finish(ctx, None);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(Color::BLACK.with_opacity(0.32)));
        let mut rects = Vec::new();
        let ios = ctx.look().is_ios();
        let glass = ctx.look() == aylis_core::Look::Ios26;
        let row = if ios { 56.0 } else { 52.0 };
        let danger = theme.color("danger");
        if ios {
            let w = (r.width - 16.0).min(420.0);
            let x = r.x + (r.width - w) / 2.0;
            let head = if self.title.is_empty() { 0.0 } else { 44.0 };
            let group_h = head + row * self.actions.len() as f32;
            let cancel_h = if self.cancel.is_empty() { 0.0 } else { row };
            let bottom = r.bottom() - 10.0;
            let group = RectF::new(x, bottom - cancel_h - (if cancel_h > 0.0 { 8.0 } else { 0.0 }) - group_h, w, group_h);
            let radius = if glass { 26.0 } else { 14.0 };
            let card = |canvas: &mut dyn Canvas, rect: RectF| {
                if glass {
                    crate::platform::paint_glass(canvas, ctx, rect, radius, None);
                } else {
                    canvas.fill_rect(rect, radius, &Brush::Solid(theme.color("surface").with_opacity(0.96)));
                }
            };
            card(canvas, group);
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            if head > 0.0 {
                let mut h = t.clone();
                h.font.size = 13.0;
                h.color = theme.color("text_muted");
                canvas.draw_text(&self.title, RectF::new(group.x, group.y, w, head), &h);
            }
            for (i, a) in self.actions.iter().enumerate() {
                let rr = RectF::new(group.x, group.y + head + i as f32 * row, w, row);
                if i > 0 || head > 0.0 {
                    canvas.draw_line(PointF::new(rr.x, rr.y), PointF::new(rr.right(), rr.y), &Stroke::new(theme.color("border"), 0.5));
                }
                t.font.size = 20.0;
                t.color = if self.destructive == Some(i) { danger } else { theme.color("primary") };
                canvas.draw_text(a, rr, &t);
                rects.push((rr, Some(i)));
            }
            if cancel_h > 0.0 {
                let c = RectF::new(x, bottom - cancel_h, w, cancel_h);
                card(canvas, c);
                t.font.weight = FontWeight::SemiBold;
                t.color = theme.color("primary");
                canvas.draw_text(&self.cancel, c, &t);
                rects.push((c, None));
            }
        } else {
            // Material: folha no rodapé com a lista.
            let head = if self.title.is_empty() { 20.0 } else { 56.0 };
            let h = head + row * self.actions.len() as f32 + 16.0;
            let sheet = RectF::new(r.x, r.bottom() - h, r.width, h);
            canvas.save();
            canvas.clip_rect(sheet);
            canvas.fill_rect(RectF::new(sheet.x, sheet.y, sheet.width, sheet.height + 28.0), 28.0, &Brush::Solid(theme.color("surface")));
            canvas.restore();
            canvas.fill_rect(RectF::new(sheet.x + (sheet.width - 32.0) / 2.0, sheet.y + 10.0, 32.0, 4.0), 2.0, &Brush::Solid(theme.color("text_muted").with_opacity(0.4)));
            let mut t = ctx.text_style();
            if !self.title.is_empty() {
                t.color = theme.color("text_muted");
                canvas.draw_text(&self.title, RectF::new(sheet.x + 24.0, sheet.y + 22.0, sheet.width - 48.0, 28.0), &t);
            }
            for (i, a) in self.actions.iter().enumerate() {
                let rr = RectF::new(sheet.x, sheet.y + head + i as f32 * row, sheet.width, row);
                t.color = if self.destructive == Some(i) { danger } else { theme.color("text") };
                canvas.draw_text(a, RectF::new(rr.x + 24.0, rr.y, rr.width - 48.0, rr.height), &t);
                rects.push((rr, Some(i)));
            }
        }
        *self.rects.borrow_mut() = rects;
    }
}

/// Mostra as ações por cima do form da frente (iOS: cartões no rodapé com o Cancelar à
/// parte; Material: folha com a lista). `on_pick` recebe o índice da ação ou `None`
/// (cancelou, tocou fora ou Esc). `destructive` pinta uma ação de vermelho ("Apagar").
pub fn action_sheet(
    ui: &mut Ui,
    title: &str,
    actions: &[&str],
    destructive: Option<usize>,
    cancel: &str,
    on_pick: impl FnOnce(&mut Ui, Option<usize>) + 'static,
) -> Option<aylis_core::ComponentId> {
    let form = ui.open_forms().last().copied().or_else(|| ui.roots().first().copied())?;
    let mut base = ControlBase::new(10.0, 10.0);
    base.set_align(aylis_core::Align::Contents).set_can_focus(true);
    let sheet = ActionSheet {
        base,
        title: title.into(),
        actions: actions.iter().map(|a| a.to_string()).collect(),
        destructive,
        cancel: cancel.into(),
        rects: RefCell::new(Vec::new()),
        on_pick: Some(Box::new(on_pick)),
    };
    let id = ui.add(sheet, form);
    ui.set_focus(id);
    Some(id)
}
