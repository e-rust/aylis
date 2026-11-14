//! [`ScreenLayout`]: as telas (monitores e câmeras) na posição real, para escolher o que
//! entra numa gravação ou transmissão.

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Image, ImageFit, PaintCtx, PointF,
    PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::leaf_control;

/// Uma tela do [`ScreenLayout`], em pixels da área de trabalho.
#[derive(Clone, Debug, Default)]
pub struct ScreenItem {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub enabled: bool,
    /// Câmera (fundo azul e o ícone, o nome no canto) em vez de monitor.
    pub camera: bool,
    /// A prévia ao vivo (cobre o retângulo).
    pub image: Option<Image>,
}

/// As telas na posição real (como o "Configurações de vídeo" do sistema), escaladas para
/// caber na caixa, cada uma com a prévia e a marca no canto: o clique marca ou desmarca
/// e dispara `on_change` com o `id` da tela ([`ScreenLayout::is_enabled`] diz o novo
/// estado). `locked` não deixa mudar (durante a gravação). As telas vêm do código
/// ([`ScreenLayout::set_screens`]); as prévias, com [`ScreenLayout::set_image`].
pub struct ScreenLayout {
    pub base: ControlBase,
    /// Travado: o clique não marca nem desmarca as telas (durante a gravação).
    pub locked: bool,
    screens: Vec<ScreenItem>,
    hovered: Cell<Option<usize>>,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 250.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, locked: false, screens: Vec::new(), hovered: Cell::new(None) }
    }
}

/// Folga em volta das telas, dentro da caixa.
const PAD: f32 = 8.0;

impl ScreenLayout {
    pub fn screens(&self) -> &[ScreenItem] {
        &self.screens
    }

    /// Troca as telas (as prévias das que continuam ficam).
    pub fn set_screens(&mut self, mut screens: Vec<ScreenItem>) {
        for s in screens.iter_mut().filter(|s| s.image.is_none()) {
            s.image = self.screens.iter().find(|o| o.id == s.id).and_then(|o| o.image.clone());
        }
        self.screens = screens;
    }

    pub fn set_image(&mut self, id: &str, image: Option<Image>) {
        if let Some(s) = self.screens.iter_mut().find(|s| s.id == id) {
            s.image = image;
        }
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        self.screens.iter().any(|s| s.id == id && s.enabled)
    }

    /// O retângulo de cada tela na caixa `r` (escala única, centralizado).
    fn rects(&self, r: RectF) -> Vec<RectF> {
        if self.screens.is_empty() {
            return Vec::new();
        }
        let min_x = self.screens.iter().map(|s| s.x).min().unwrap_or(0) as f32;
        let min_y = self.screens.iter().map(|s| s.y).min().unwrap_or(0) as f32;
        let max_x = self.screens.iter().map(|s| s.x + s.width).max().unwrap_or(1) as f32;
        let max_y = self.screens.iter().map(|s| s.y + s.height).max().unwrap_or(1) as f32;
        let (tw, th) = ((max_x - min_x).max(1.0), (max_y - min_y).max(1.0));
        let (aw, ah) = ((r.width - 2.0 * PAD).max(1.0), (r.height - 2.0 * PAD).max(1.0));
        let scale = (aw / tw).min(ah / th);
        let (ox, oy) = (r.x + PAD + (aw - tw * scale) / 2.0, r.y + PAD + (ah - th * scale) / 2.0);
        self.screens
            .iter()
            .map(|s| RectF::new(ox + (s.x as f32 - min_x) * scale, oy + (s.y as f32 - min_y) * scale, s.width as f32 * scale, s.height as f32 * scale))
            .collect()
    }

    fn screen_at(&self, p: PointF) -> Option<usize> {
        let size = self.base.bounds();
        self.rects(RectF::new(0.0, 0.0, size.width, size.height)).iter().position(|r| r.contains(p))
    }
}

impl Component for ScreenLayout {
    leaf_control!("ScreenLayout");

    properties! {
        locked: bool => "Behavior";
        events: on_change
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let at = self.screen_at(event.position);
        if at != self.hovered.get() {
            self.hovered.set(at);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered.set(None);
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.locked {
            return;
        }
        let Some(i) = self.screen_at(event.position) else { return };
        let screen = &mut self.screens[i];
        screen.enabled = !screen.enabled;
        let id = screen.id.clone();
        ctx.fire("on_change", EventArgs::Text(id));
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let t = ctx.theme;
        let border2 = t.colors.get("border_2").copied().unwrap_or(t.color("border"));
        canvas.fill_rect(r, 8.0, &Brush::Solid(t.color("surface_2")));
        canvas.stroke_rect(r.inflate(-0.5), 8.0, &Stroke::new(t.color("border"), 1.0));
        canvas.save();
        canvas.clip_round_rect(r, 8.0);
        if self.locked {
            canvas.save_layer(0.6);
        }
        let success = t.color("success");
        for (i, (s, rect)) in self.screens.iter().zip(self.rects(r)).enumerate() {
            let hover = self.hovered.get() == Some(i) && !self.locked;
            canvas.save();
            if !s.enabled && !hover {
                canvas.save_layer(0.5);
            }
            if s.enabled {
                canvas.stroke_rect(rect.inflate(1.0), 5.0, &Stroke::new(success, 1.0));
            }
            canvas.save();
            canvas.clip_round_rect(rect, 4.0);
            let fill = if s.camera {
                aylis_controls_gradient(rect, Color::rgb(0x1E, 0x3A, 0x5F), Color::rgb(0x0F, 0x22, 0x40))
            } else {
                aylis_controls_gradient(rect, t.color("surface_3").with_opacity(1.0), t.color("background"))
            };
            canvas.fill_rect(rect, 0.0, &fill);
            let has_image = !s.camera && s.image.is_some();
            if let (false, Some(image)) = (s.camera, &s.image) {
                if let Some(size) = image.known_size().or_else(|| canvas.image_size(image)) {
                    let (src, dest) = ImageFit::Cover.place(size, rect);
                    canvas.draw_image(image, src, dest, 1.0);
                }
            }
            if s.camera {
                let side = 28.0f32.min(rect.width * 0.4).min(rect.height * 0.4);
                let icon = RectF::new(rect.x + (rect.width - side) / 2.0, rect.y + (rect.height - side) / 2.0, side, side);
                ctx.draw_icon(canvas, "camera", icon, Color::WHITE.with_opacity(0.7));
            }
            // O nome (sem prévia; na câmera, no canto de baixo).
            if s.camera || !has_image {
                let mut style = ctx.text_style();
                style.font.size = 10.0;
                style.font.weight = FontWeight::SemiBold;
                style.color = Color::WHITE.with_opacity(0.92);
                style.h_align = TextAlign::Center;
                style.v_align = TextAlign::Center;
                let w = canvas.measure_text(&s.name, &style).width.min(rect.width - 8.0) + 12.0;
                let h = 18.0;
                let label = if s.camera {
                    RectF::new(rect.x + 4.0, rect.bottom() - h - 4.0, w, h)
                } else {
                    RectF::new(rect.x + (rect.width - w) / 2.0, rect.y + (rect.height - h) / 2.0, w, h)
                };
                canvas.fill_rect(label, 3.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.6)));
                canvas.draw_text(&s.name, label, &style);
            }
            // A marca do canto.
            let check = RectF::new(rect.right() - 20.0, rect.y + 4.0, 16.0, 16.0);
            if rect.width > 28.0 && rect.height > 24.0 {
                let (fill, edge) = if s.enabled { (success, success) } else { (Color::rgb(0, 0, 0).with_opacity(0.55), Color::WHITE.with_opacity(0.75)) };
                canvas.fill_rect(check, 3.0, &Brush::Solid(fill));
                canvas.stroke_rect(check.inflate(-0.75), 3.0, &Stroke::new(edge, 1.5));
                if s.enabled {
                    ctx.draw_icon(canvas, "check", check.inflate(-3.0), Color::WHITE);
                }
            }
            canvas.restore();
            canvas.stroke_rect(rect.inflate(-0.75), 4.0, &Stroke::new(if s.enabled { success } else { border2 }, 1.5));
            if !s.enabled && !hover {
                canvas.restore();
            }
            canvas.restore();
        }
        if self.locked {
            canvas.restore();
        }
        canvas.restore();
    }
}

/// Degradê de 135° (do canto de cima à esquerda ao de baixo à direita).
fn aylis_controls_gradient(r: RectF, from: Color, to: Color) -> Brush {
    crate::basic::linear_gradient(r, 135.0, from, to)
}
