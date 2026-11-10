//! Conversa: lista de mensagens em bolhas ([`ChatView`]) e onda de áudio ([`Waveform`]).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, EventArgs, EventCtx, FontWeight, PaintCtx, PointF, PointerButton,
    PointerEvent, PropEnum, RectF, Stroke, TextAlign, TextStyle, Trimming, WheelEvent, properties,
};

use crate::leaf_control;
use crate::widgets::name_color;

/// Situação de uma mensagem enviada.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageStatus {
    #[default]
    None,
    /// Enviando (relógio).
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}

impl PropEnum for MessageStatus {
    const NAMES: &'static [&'static str] = &["none", "pending", "sent", "delivered", "read", "failed"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Pending, Self::Sent, Self::Delivered, Self::Read, Self::Failed].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Uma mensagem da conversa.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChatMessage {
    pub id: String,
    pub author: String,
    pub text: String,
    /// Horário já formatado ("14:32").
    pub time: String,
    /// Enviada por quem usa o app (bolha à direita).
    pub mine: bool,
    pub status: MessageStatus,
    /// Trecho citado (resposta a outra mensagem): (autor, texto).
    pub reply: Option<(String, String)>,
    /// Aviso do sistema no meio da conversa ("Hoje", "Ana entrou no grupo").
    pub system: bool,
}

impl ChatMessage {
    pub fn new(id: impl Into<String>, author: impl Into<String>, text: impl Into<String>) -> Self {
        Self { id: id.into(), author: author.into(), text: text.into(), ..Default::default() }
    }

    pub fn mine(mut self) -> Self {
        self.mine = true;
        self
    }

    pub fn at(mut self, time: impl Into<String>) -> Self {
        self.time = time.into();
        self
    }

    pub fn status(mut self, s: MessageStatus) -> Self {
        self.status = s;
        self
    }

    pub fn reply_to(mut self, author: impl Into<String>, text: impl Into<String>) -> Self {
        self.reply = Some((author.into(), text.into()));
        self
    }

    /// Aviso do sistema (centralizado, sem bolha).
    pub fn system(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self { id: id.into(), text: text.into(), system: true, ..Default::default() }
    }
}

/// Medidas de uma mensagem já disposta.
#[derive(Clone, Debug, Default)]
struct Placed {
    top: f32,
    height: f32,
    bubble: RectF,
    lines: Vec<String>,
    /// Primeira de uma sequência do mesmo autor (mostra nome e avatar).
    first: bool,
}

const PAD_X: f32 = 12.0;
const PAD_Y: f32 = 7.0;
const AVATAR: f32 = 30.0;

/// Lista de mensagens de uma conversa (como um app de chat): bolhas à direita para as
/// suas e à esquerda para as dos outros, com nome e avatar no começo de cada sequência,
/// citação de resposta, horário e confirmação de leitura. Só desenha o que está na tela
/// (milhares de mensagens sem custo) e acompanha o fim quando chega mensagem nova, se
/// já estava no fim.
pub struct ChatView {
    pub base: ControlBase,
    /// Nome e avatar dos outros (conversa em grupo).
    pub show_authors: bool,
    messages: Vec<ChatMessage>,
    layout: RefCell<(f32, Vec<Placed>, f32)>,
    dirty: Cell<bool>,
    scroll: Cell<f32>,
    stick_bottom: Cell<bool>,
    view_height: Cell<f32>,
}

impl Default for ChatView {
    fn default() -> Self {
        let mut base = ControlBase::new(420.0, 480.0);
        base.set_clip_children(true);
        Self {
            base,
            show_authors: true,
            messages: Vec::new(),
            layout: RefCell::new((0.0, Vec::new(), 0.0)),
            dirty: Cell::new(true),
            scroll: Cell::new(0.0),
            stick_bottom: Cell::new(true),
            view_height: Cell::new(0.0),
        }
    }
}

impl ChatView {
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
        self.dirty.set(true);
    }

    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    pub fn message_mut(&mut self, id: &str) -> Option<&mut ChatMessage> {
        self.dirty.set(true);
        self.messages.iter_mut().find(|m| m.id == id)
    }

    pub fn remove_message(&mut self, id: &str) {
        self.messages.retain(|m| m.id != id);
        self.dirty.set(true);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.dirty.set(true);
        self.scroll.set(0.0);
    }

    /// Rola até a última mensagem (e continua acompanhando as novas).
    pub fn scroll_to_end(&mut self) {
        self.stick_bottom.set(true);
    }

    fn text_style(ctx: &PaintCtx<'_>) -> TextStyle {
        let mut t = ctx.text_style();
        t.v_align = TextAlign::Leading;
        t.trimming = Trimming::None;
        t
    }

    fn line_height(style: &TextStyle) -> f32 {
        (style.font.size * 1.4).round()
    }

    /// Dispõe as mensagens para a largura `width` (refaz só quando muda algo).
    fn relayout(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, width: f32) {
        let mut layout = self.layout.borrow_mut();
        if !self.dirty.get() && (layout.0 - width).abs() < 0.5 {
            return;
        }
        let style = Self::text_style(ctx);
        let lh = Self::line_height(&style);
        let small = 11.5;
        let mut placed = Vec::with_capacity(self.messages.len());
        let mut y = 10.0;
        let mut prev: Option<&ChatMessage> = None;
        let left = if self.show_authors { 12.0 + AVATAR + 8.0 } else { 12.0 };
        let max_w = ((width - left - 12.0) * 0.78).max(80.0);
        for m in &self.messages {
            if m.system {
                let h = 26.0;
                y += 6.0;
                placed.push(Placed { top: y, height: h, bubble: RectF::new(0.0, y, width, h), lines: vec![m.text.clone()], first: true });
                y += h + 8.0;
                prev = None;
                continue;
            }
            let first = prev.is_none_or(|p| p.author != m.author || p.mine != m.mine);
            if first && prev.is_some() {
                y += 8.0;
            }
            let mut measure = |s: &str| canvas.measure_text(s, &style).width;
            let lines = aylis_core::text::layout_lines(&m.text, max_w - 2.0 * PAD_X, usize::MAX, true, Trimming::None, &mut measure);
            let text_w = lines.iter().map(|l| measure(l)).fold(0.0f32, f32::max);
            let mut small_style = style.clone();
            small_style.font.size = small;
            let footer_w = canvas.measure_text(&m.time, &small_style).width + if m.mine { 22.0 } else { 0.0 };
            let show_author = first && !m.mine && self.show_authors;
            let author_w = if show_author {
                let mut a = small_style.clone();
                a.font.weight = FontWeight::SemiBold;
                canvas.measure_text(&m.author, &a).width
            } else {
                0.0
            };
            let reply_h = if m.reply.is_some() { 44.0 } else { 0.0 };
            // O horário cabe na última linha se sobrar espaço; senão vai para baixo.
            let last_w = lines.last().map_or(0.0, |l| canvas.measure_text(l, &style).width);
            let inline_footer = last_w + footer_w + 10.0 <= max_w - 2.0 * PAD_X;
            let mut content_w = text_w.max(author_w).max(if inline_footer { last_w + footer_w + 10.0 } else { footer_w });
            if m.reply.is_some() {
                content_w = content_w.max(160.0f32.min(max_w - 2.0 * PAD_X));
            }
            let bubble_w = (content_w + 2.0 * PAD_X).min(max_w);
            let mut h = 2.0 * PAD_Y + lines.len() as f32 * lh + reply_h;
            if show_author {
                h += 18.0;
            }
            if !inline_footer {
                h += 16.0;
            }
            let x = if m.mine { width - 12.0 - bubble_w } else { left };
            placed.push(Placed { top: y, height: h, bubble: RectF::new(x, y, bubble_w, h), lines, first });
            y += h + 3.0;
            prev = Some(m);
        }
        let total = y + 10.0;
        *layout = (width, placed, total);
        self.dirty.set(false);
    }

    fn max_scroll(&self) -> f32 {
        (self.layout.borrow().2 - self.view_height.get()).max(0.0)
    }

    fn message_at(&self, p: PointF) -> Option<usize> {
        let y = p.y + self.scroll.get();
        self.layout.borrow().1.iter().position(|pl| pl.bubble.contains(PointF::new(p.x, y)))
    }
}

impl Component for ChatView {
    leaf_control!("ChatView");

    properties! {
        show_authors: bool => "Appearance";
        events: on_message_click
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let dy = if event.touch { event.delta.y } else { event.delta.y.signum() * 60.0 };
        let max = self.max_scroll();
        let next = (self.scroll.get() - dy).clamp(0.0, max);
        if (next - self.scroll.get()).abs() < 0.5 {
            return;
        }
        self.scroll.set(next);
        self.stick_bottom.set(next >= max - 1.0);
        ctx.repaint();
        ctx.set_handled();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if let Some(i) = self.message_at(event.position) {
            if !self.messages[i].system {
                ctx.fire("on_message_click", EventArgs::Text(self.messages[i].id.clone()));
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("background")));
        self.view_height.set(r.height);
        self.relayout(canvas, ctx, r.width);
        if self.stick_bottom.get() {
            self.scroll.set(self.max_scroll());
        } else {
            self.scroll.set(self.scroll.get().clamp(0.0, self.max_scroll()));
        }
        let scroll = self.scroll.get();
        let layout = self.layout.borrow();
        let style = Self::text_style(ctx);
        let lh = Self::line_height(&style);
        let mut small = style.clone();
        small.font.size = 11.5;
        canvas.save();
        canvas.clip_rect(r);
        canvas.translate(0.0, -scroll);
        for (m, pl) in self.messages.iter().zip(&layout.1) {
            if pl.top + pl.height < scroll - 40.0 || pl.top > scroll + r.height + 40.0 {
                continue;
            }
            if m.system {
                let mut t = small.clone();
                t.h_align = TextAlign::Center;
                t.v_align = TextAlign::Center;
                t.color = theme.color("text_muted");
                let w = canvas.measure_text(&m.text, &t).width + 20.0;
                let pill = RectF::new((r.width - w) / 2.0, pl.top, w, pl.height);
                canvas.fill_rect(pill, pl.height / 2.0, &Brush::Solid(theme.color("surface_2")));
                canvas.draw_text(&m.text, pill, &t);
                continue;
            }
            let b = pl.bubble;
            let (fill, fg, muted) = if m.mine {
                let on = theme.color("on_primary");
                (theme.color("primary"), on, on.with_opacity(0.75))
            } else {
                (theme.color("surface"), theme.color("text"), theme.color("text_muted"))
            };
            if let (true, false, true) = (pl.first, m.mine, self.show_authors) {
                let a = RectF::new(12.0, b.y, AVATAR, AVATAR);
                canvas.fill_ellipse(a, &Brush::Solid(name_color(&m.author)));
                let mut ini = small.clone();
                ini.color = Color::WHITE;
                ini.h_align = TextAlign::Center;
                ini.v_align = TextAlign::Center;
                ini.font.weight = FontWeight::SemiBold;
                let initial: String = m.author.chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default();
                canvas.draw_text(&initial, a, &ini);
            }
            if let Some(shadow) = theme.shadows.get("sm") {
                canvas.draw_shadow(b, 12.0, shadow);
            }
            canvas.fill_rect(b, 12.0, &Brush::Solid(fill));
            if !m.mine {
                canvas.stroke_rect(b, 12.0, &Stroke::new(theme.color("border"), 1.0));
            }
            let inner_w = b.width - 2.0 * PAD_X;
            let mut y = b.y + PAD_Y;
            if pl.first && !m.mine && self.show_authors {
                let mut a = small.clone();
                a.font.weight = FontWeight::SemiBold;
                a.color = name_color(&m.author);
                canvas.draw_text(&m.author, RectF::new(b.x + PAD_X, y, inner_w, 16.0), &a);
                y += 18.0;
            }
            if let Some((author, text)) = &m.reply {
                let q = RectF::new(b.x + PAD_X, y, inner_w, 38.0);
                canvas.fill_rect(q, 6.0, &Brush::Solid(if m.mine { Color::BLACK.with_opacity(0.15) } else { theme.color("surface_2") }));
                canvas.fill_rect(RectF::new(q.x, q.y, 3.0, q.height), 1.5, &Brush::Solid(if m.mine { fg } else { name_color(author) }));
                let mut qa = small.clone();
                qa.font.weight = FontWeight::SemiBold;
                qa.color = if m.mine { fg } else { name_color(author) };
                canvas.draw_text(author, RectF::new(q.x + 9.0, q.y + 3.0, q.width - 12.0, 15.0), &qa);
                let mut qt = small.clone();
                qt.color = muted;
                qt.trimming = Trimming::Character;
                canvas.draw_text(text, RectF::new(q.x + 9.0, q.y + 19.0, q.width - 12.0, 15.0), &qt);
                y += 44.0;
            }
            let mut t = style.clone();
            t.color = fg;
            for line in &pl.lines {
                canvas.draw_text(line, RectF::new(b.x + PAD_X, y, inner_w + 1.0, lh), &t);
                y += lh;
            }
            // Horário e confirmação, no canto de baixo.
            let mut f = small.clone();
            f.color = muted;
            f.h_align = TextAlign::Trailing;
            f.v_align = TextAlign::Center;
            let status_w = if m.mine { 20.0 } else { 0.0 };
            let fy = b.bottom() - PAD_Y - 14.0;
            canvas.draw_text(&m.time, RectF::new(b.x + PAD_X, fy, inner_w - status_w, 14.0), &f);
            if m.mine {
                let icon_r = RectF::new(b.right() - PAD_X - 16.0, fy - 1.0, 16.0, 16.0);
                match m.status {
                    MessageStatus::None => {}
                    MessageStatus::Pending => ctx.draw_icon(canvas, "clock", icon_r, muted),
                    MessageStatus::Sent => ctx.draw_icon(canvas, "check", icon_r, muted),
                    MessageStatus::Delivered | MessageStatus::Read => {
                        let color = if m.status == MessageStatus::Read { theme.color("info") } else { muted };
                        let color = if m.status == MessageStatus::Read && m.mine { Color::rgb(0x7D, 0xD3, 0xFC) } else { color };
                        ctx.draw_icon(canvas, "check", RectF::new(icon_r.x - 4.0, icon_r.y, 16.0, 16.0), color);
                        ctx.draw_icon(canvas, "check", icon_r, color);
                    }
                    MessageStatus::Failed => ctx.draw_icon(canvas, "warning", icon_r, theme.color("danger")),
                }
            }
        }
        canvas.restore();
    }
}

// ===========================================================================
// Waveform
// ===========================================================================

/// Onda de um áudio (mensagem de voz, gravação): barras com a intensidade de cada trecho,
/// a parte já tocada em destaque. Clicar ou arrastar busca (`on_seek`, em segundos).
pub struct Waveform {
    pub base: ControlBase,
    pub position: f64,
    pub duration: f64,
    /// Barra de busca de player: trilho fino com o polegar e a onda embaixo, discreta
    /// (sem ela, a onda inteira é a barra, pintada até a posição).
    pub seek_bar: bool,
    /// Com `seek_bar`: pausado, o trilho e o polegar saem na cor do texto em vez do destaque.
    pub paused: bool,
    peaks: Vec<f32>,
    dragging: bool,
}

impl Default for Waveform {
    fn default() -> Self {
        let mut base = ControlBase::new(220.0, 36.0);
        base.set_cursor(aylis_core::CursorShape::Pointer).set_pan_on_touch(false);
        // Uma onda de exemplo até o app passar a verdadeira.
        let peaks = (0..48).map(|i| 0.25 + 0.6 * ((i as f32 * 0.9).sin() * (i as f32 * 0.31).cos()).abs()).collect();
        Self { base, position: 0.0, duration: 10.0, seek_bar: false, paused: false, peaks, dragging: false }
    }
}

impl Waveform {
    /// Intensidades de 0 a 1, uma por barra.
    pub fn set_peaks(&mut self, peaks: Vec<f32>) {
        self.peaks = peaks;
    }

    /// Calcula as barras a partir das amostras (-1 a 1): o pico de cada trecho.
    pub fn set_samples(&mut self, samples: &[f32], bars: usize) {
        let bars = bars.max(1);
        let chunk = samples.len().div_ceil(bars).max(1);
        let mut peaks: Vec<f32> = samples.chunks(chunk).map(|c| c.iter().fold(0.0f32, |m, s| m.max(s.abs()))).collect();
        let max = peaks.iter().copied().fold(0.0f32, f32::max);
        if max > 0.0 {
            for p in &mut peaks {
                *p /= max;
            }
        }
        self.peaks = peaks;
    }

    pub fn peaks(&self) -> &[f32] {
        &self.peaks
    }

    /// Trilho de 4 px no meio, polegar de 14 (com um halo de 26 sob o mouse) e a onda,
    /// com 14 px, logo abaixo do trilho, crescendo para baixo.
    fn paint_seek_bar(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let text = ctx.theme.color("text");
        let p = if self.duration > 0.0 { (self.position / self.duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let cy = r.y + r.height / 2.0;
        let n = self.peaks.len();
        if n > 0 {
            let wave = text.with_opacity(0.75 * 0.55);
            let slot = r.width / n as f32;
            for (i, h) in self.peaks.iter().enumerate() {
                let bh = (14.0 * h.clamp(0.04, 1.0)).max(1.0);
                let x = r.x + i as f32 * slot;
                canvas.fill_rect(RectF::new(x, cy + 2.0, (slot * 0.88).max(0.75), bh), 0.0, &Brush::Solid(wave));
            }
        }
        let fill = if self.paused { text.with_opacity(0.55) } else { ctx.theme.color("primary") };
        canvas.fill_rect(RectF::new(r.x, cy - 2.0, r.width, 4.0), 2.0, &Brush::Solid(text.with_opacity(0.15)));
        canvas.fill_rect(RectF::new(r.x, cy - 2.0, r.width * p, 4.0), 2.0, &Brush::Solid(fill));
        let x = r.x + r.width * p;
        if ctx.state.hover || self.dragging {
            let halo = if self.paused { text.with_opacity(0.18) } else { ctx.theme.color("primary").with_opacity(0.2) };
            canvas.fill_ellipse(RectF::new(x - 13.0, cy - 13.0, 26.0, 26.0), &Brush::Solid(halo));
        }
        let thumb = if self.paused { text.with_opacity(0.85) } else { ctx.theme.color("primary") };
        canvas.fill_ellipse(RectF::new(x - 7.0, cy - 7.0, 14.0, 14.0), &Brush::Solid(thumb));
    }

    fn seek(&mut self, ctx: &mut EventCtx<'_>, x: f32) {
        let w = self.base.bounds().width.max(1.0);
        self.position = ((x / w).clamp(0.0, 1.0) as f64) * self.duration;
        ctx.fire("on_seek", EventArgs::Value(self.position.into()));
        ctx.repaint();
    }
}

impl Component for Waveform {
    leaf_control!("Waveform");

    properties! {
        position: f64 => "Data",
        duration: f64 => "Data",
        seek_bar: bool => "Appearance",
        paused: bool => "Appearance";
        events: on_seek
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.dragging = true;
            self.seek(ctx, event.position.x);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging {
            self.seek(ctx, event.position.x);
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = false;
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if self.seek_bar {
            self.paint_seek_bar(canvas, ctx);
            return;
        }
        let r = ctx.rect;
        let n = self.peaks.len();
        if n == 0 {
            return;
        }
        let played = if self.duration > 0.0 { (self.position / self.duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let slot = r.width / n as f32;
        let bar = (slot * 0.6).clamp(1.0, 4.0);
        let primary = ctx.theme.color(if ctx.variant() == "light" { "on_primary" } else { "primary" });
        let rest = ctx.theme.color("text_muted").with_opacity(0.45);
        for (i, p) in self.peaks.iter().enumerate() {
            let h = (r.height * p.clamp(0.06, 1.0)).max(2.0);
            let x = r.x + i as f32 * slot + (slot - bar) / 2.0;
            let color = if (i as f32 + 0.5) / n as f32 <= played { primary } else { rest };
            canvas.fill_rect(RectF::new(x, r.y + (r.height - h) / 2.0, bar, h), bar / 2.0, &Brush::Solid(color));
        }
    }
}
