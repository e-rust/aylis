//! [`CropBox`]: a área de recorte por cima de uma imagem ou de um vídeo, com as alças
//! nos cantos e nas bordas (no jeito de um editor de vídeo).

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, PaintCtx, PointF, PointerButton,
    PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::leaf_control;

/// O que um arrasto mexe: o miolo (mover) ou uma borda/canto (`n`, `se`...).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Grip {
    n: bool,
    s: bool,
    w: bool,
    e: bool,
}

impl Grip {
    const MOVE: Grip = Grip { n: false, s: false, w: false, e: false };

    fn is_move(self) -> bool {
        self == Grip::MOVE
    }

    fn cursor(self) -> CursorShape {
        match (self.n || self.s, self.w || self.e) {
            (false, false) => CursorShape::Move,
            (true, false) => CursorShape::ResizeVertical,
            (false, true) => CursorShape::ResizeHorizontal,
            _ if (self.n && self.w) || (self.s && self.e) => CursorShape::ResizeNwSe,
            _ => CursorShape::ResizeNeSw,
        }
    }
}

struct Drag {
    grip: Grip,
    from: PointF,
    start: (f32, f32, f32, f32),
    moved: f32,
}

/// O recorte de uma imagem de `source_width` × `source_height` (pixels da origem; o
/// controle cobre a imagem inteira e escala). A seleção (`sel_x`, `sel_y`, `sel_width`,
/// `sel_height`, em pixels da origem) é o que fica; fora dela a imagem escurece.
///
/// - Arrastar o miolo move; as alças dos cantos e das bordas redimensionam (as das
///   bordas somem quando o lado fica pequeno demais na tela).
/// - `min_size`: o menor lado (pixels da origem); `snap`: arredonda posição e tamanho
///   para múltiplos dele (2 = pares, o que os codificadores de vídeo pedem).
/// - `caption`: o texto no meio da seleção enquanto arrasta (vazio = "largura×altura").
/// - `framed`: o visual de editor web — terços sempre à vista, alças de 8 px brancas por
///   fora da moldura (verdes arrastando) e o tamanho em cima, à esquerda (dentro, num
///   selo, quando não cabe acima). Desenha um pouco fora do controle: deixe uns 10 px
///   de margem em volta.
/// - Eventos: `on_change` a cada passo do arrasto, `on_end` ao soltar e `on_tap`
///   num clique sem arrastar no miolo (o play/pausa de uma prévia, por exemplo).
pub struct CropBox {
    pub base: ControlBase,
    /// Largura da imagem de origem, em pixels (o controle cobre a imagem inteira e escala).
    pub source_width: f32,
    /// Altura da imagem de origem, em pixels.
    pub source_height: f32,
    /// Esquerda da seleção, em pixels da origem.
    pub sel_x: f32,
    /// Topo da seleção, em pixels da origem.
    pub sel_y: f32,
    /// Largura da seleção, em pixels da origem.
    pub sel_width: f32,
    /// Altura da seleção, em pixels da origem.
    pub sel_height: f32,
    /// O menor lado que a seleção pode ter, em pixels da origem.
    pub min_size: f32,
    /// Arredonda posição e tamanho para múltiplos dele (2 = pares, o que os codificadores
    /// de vídeo pedem); 0 ou 1: só para o pixel inteiro.
    pub snap: f32,
    /// Texto no meio da seleção enquanto arrasta; vazio: "largura×altura".
    pub caption: String,
    /// A cor de fora da seleção.
    pub shade: Color,
    /// O visual de editor web: terços sempre à vista, alças brancas por fora da moldura e o
    /// tamanho em cima (desenha um pouco fora do controle: deixe uns 10 px de margem).
    pub framed: bool,
    drag: Option<Drag>,
    dragging: Cell<bool>,
}

impl Default for CropBox {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 180.0);
        base.set_cursor(CursorShape::Move);
        Self {
            base,
            source_width: 1920.0,
            source_height: 1080.0,
            sel_x: 0.0,
            sel_y: 0.0,
            sel_width: 1920.0,
            sel_height: 1080.0,
            min_size: 32.0,
            snap: 2.0,
            caption: String::new(),
            shade: Color::rgb(0, 0, 0).with_opacity(0.55),
            framed: false,
            drag: None,
            dragging: Cell::new(false),
        }
    }
}

/// Lado mínimo (pixels de tela) para as alças das bordas caberem.
const EDGE_MIN: f32 = 46.0;
const GRIP: f32 = 10.0;
/// Até onde (para fora da seleção) as alças do `framed` respondem: os 8 px dela e o aro.
const FRAMED_REACH: f32 = 10.0;

impl CropBox {
    /// A seleção inteira (`sel_x`, `sel_y`, `sel_width`, `sel_height`).
    pub fn selection(&self) -> (f32, f32, f32, f32) {
        (self.sel_x, self.sel_y, self.sel_width, self.sel_height)
    }

    /// Troca a seleção (ajustada aos limites, ao `min_size` e ao `snap`).
    pub fn set_selection(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let (x, y, w, h) = self.fit(x, y, w, h);
        (self.sel_x, self.sel_y, self.sel_width, self.sel_height) = (x, y, w, h);
    }

    /// A imagem inteira selecionada (sem recorte).
    pub fn is_full(&self) -> bool {
        self.sel_x <= 0.0 && self.sel_y <= 0.0 && self.sel_width >= self.source_width - 1.0 && self.sel_height >= self.source_height - 1.0
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging.get()
    }

    fn fit(&self, x: f32, y: f32, w: f32, h: f32) -> (f32, f32, f32, f32) {
        let snap = |v: f32| if self.snap > 1.0 { (v / self.snap).floor() * self.snap } else { v.round() };
        let (sw, sh) = (snap(self.source_width.max(1.0)), snap(self.source_height.max(1.0)));
        let x = snap(x.clamp(0.0, sw));
        let y = snap(y.clamp(0.0, sh));
        let min = self.min_size.max(1.0);
        let w = snap(w).max(min.min(sw)).min(sw - x).max(self.snap.max(1.0));
        let h = snap(h).max(min.min(sh)).min(sh - y).max(self.snap.max(1.0));
        (x, y, w, h)
    }

    /// O visual `framed`: moldura fina com fio escuro por fora, terços, alças de fora e o
    /// tamanho em cima.
    fn paint_framed(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: RectF, accent: Color) {
        let busy = self.dragging.get();
        canvas.stroke_rect(s.inflate(0.5), 0.0, &Stroke::new(Color::rgb(0, 0, 0).with_opacity(0.55), 1.0));
        canvas.stroke_rect(s.inflate(-0.5), 0.0, &Stroke::new(if busy { accent } else { Color::WHITE.with_opacity(0.9) }, 1.0));
        let line = Stroke::new(Color::WHITE.with_opacity(0.9 * 0.35), 1.0);
        for i in 1..3 {
            let fx = (s.x + s.width * i as f32 / 3.0).round() + 0.5;
            let fy = (s.y + s.height * i as f32 / 3.0).round() + 0.5;
            canvas.draw_line(PointF::new(fx, s.y), PointF::new(fx, s.bottom()), &line);
            canvas.draw_line(PointF::new(s.x, fy), PointF::new(s.right(), fy), &line);
        }
        // O tamanho: em cima, à esquerda; sem lugar acima, num selo dentro.
        let text = if self.caption.is_empty() { format!("{}\u{00D7}{}", self.sel_width as i32, self.sel_height as i32) } else { self.caption.clone() };
        let mut style = ctx.text_style();
        style.v_align = TextAlign::Center;
        // (o palco da web tem 15 px de folga em volta: 22 px acima dele = 7 acima da imagem)
        if s.y - ctx.rect.y >= 7.0 {
            style.font.size = 13.0;
            style.color = ctx.theme.color("text");
            let h = canvas.measure_text(&text, &style).height;
            canvas.draw_text(&text, RectF::new(s.x, s.y - 3.0 - h, s.width.max(120.0), h), &style);
        } else {
            style.font.size = 11.0;
            style.color = Color::WHITE;
            let size = canvas.measure_text(&text, &style);
            let badge = RectF::new(s.x + 3.0, s.y + 3.0, size.width + 12.0, size.height + 4.0);
            canvas.fill_rect(badge, 4.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.75)));
            style.h_align = TextAlign::Center;
            canvas.draw_text(&text, badge, &style);
        }
        // Alças de 8 px do lado de fora (aro escuro de 10 px por trás).
        let handle = |canvas: &mut dyn Canvas, x: f32, y: f32| {
            let h = RectF::new(x, y, 8.0, 8.0);
            canvas.fill_rect(h.inflate(1.0), 3.0, &Brush::Solid(if busy { accent } else { Color::rgb(0, 0, 0).with_opacity(0.55) }));
            canvas.fill_rect(h, 2.0, &Brush::Solid(if busy { accent } else { Color::WHITE }));
        };
        let (mx, my) = (s.x + s.width / 2.0 - 4.0, s.y + s.height / 2.0 - 4.0);
        if s.width >= EDGE_MIN {
            handle(canvas, mx, s.y - 8.0);
            handle(canvas, mx, s.bottom());
        }
        if s.height >= EDGE_MIN {
            handle(canvas, s.x - 8.0, my);
            handle(canvas, s.right(), my);
        }
        for (x, y) in [(s.x - 8.0, s.y - 8.0), (s.right(), s.y - 8.0), (s.x - 8.0, s.bottom()), (s.right(), s.bottom())] {
            handle(canvas, x, y);
        }
    }

    /// Escala origem → tela.
    fn scale(&self, r: RectF) -> (f32, f32) {
        (r.width / self.source_width.max(1.0), r.height / self.source_height.max(1.0))
    }

    /// O retângulo da seleção no controle (`r` = a área do controle).
    fn sel_rect(&self, r: RectF) -> RectF {
        let (kx, ky) = self.scale(r);
        RectF::new(r.x + self.sel_x * kx, r.y + self.sel_y * ky, self.sel_width * kx, self.sel_height * ky)
    }

    fn area(&self) -> RectF {
        RectF::new(0.0, 0.0, self.base.width(), self.base.height())
    }

    /// O que está sob `p` (coordenadas do controle): uma alça, o miolo ou nada.
    fn grip_at(&self, p: PointF) -> Option<Grip> {
        let s = self.sel_rect(self.area());
        // As alças do `framed` ficam do lado de fora (8 px e o aro): valem até a borda delas.
        let (inner, outer) = if self.framed { (GRIP / 2.0 + 2.0, FRAMED_REACH) } else { (GRIP / 2.0 + 2.0, GRIP / 2.0 + 2.0) };
        if !s.inflate(outer).contains(p) {
            return None;
        }
        // Perto da borda: até `inner` para dentro da seleção e `outer` para fora.
        let near = |a: f32, b: f32, out: f32| { let d = (a - b) * out; d >= -inner && d <= outer };
        let (edges_h, edges_v) = (s.width >= EDGE_MIN, s.height >= EDGE_MIN);
        let mut g = Grip { n: near(p.y, s.y, -1.0), s: near(p.y, s.bottom(), 1.0), w: near(p.x, s.x, -1.0), e: near(p.x, s.right(), 1.0) };
        let corner = (g.n || g.s) && (g.w || g.e);
        // Sem espaço para a alça da borda, só os cantos redimensionam.
        if !corner {
            if (g.n || g.s) && !edges_h {
                g.n = false;
                g.s = false;
            }
            if (g.w || g.e) && !edges_v {
                g.w = false;
                g.e = false;
            }
        }
        Some(g)
    }

    fn selection_value(&self) -> EventArgs {
        EventArgs::Text(format!("{} {} {} {}", self.sel_x, self.sel_y, self.sel_width, self.sel_height))
    }
}

impl Component for CropBox {
    leaf_control!("CropBox");

    properties! {
        source_width: f32 => "Data",
        source_height: f32 => "Data",
        sel_x: f32 => "Data",
        sel_y: f32 => "Data",
        sel_width: f32 => "Data",
        sel_height: f32 => "Data",
        min_size: f32 => "Behavior",
        snap: f32 => "Behavior",
        caption: String => "Appearance",
        framed: bool => "Appearance",
        shade: Color => "Appearance";
        events: on_change, on_end, on_tap
    }

    /// As alças do `framed` passam da borda do controle (a seleção inteira encosta nela).
    fn hit_slop(&self) -> f32 {
        if self.framed { FRAMED_REACH } else { GRIP / 2.0 + 2.0 }
    }

    /// Dentro do retângulo, tudo; fora (o `hit_slop`), só em cima de uma alça.
    fn contains_point(&self, local: PointF, size: aylis_core::SizeF) -> bool {
        let inside = local.x >= 0.0 && local.y >= 0.0 && local.x < size.width && local.y < size.height;
        inside || self.grip_at(local).is_some_and(|g| !g.is_move())
    }

    fn property_changed(&mut self, _ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "source_width" | "source_height" | "min_size" | "snap") {
            let (x, y, w, h) = self.selection();
            self.set_selection(x, y, w, h);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let Some(grip) = self.grip_at(event.position) else { return };
        self.drag = Some(Drag { grip, from: event.position, start: self.selection(), moved: 0.0 });
        self.dragging.set(true);
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some(d) = &mut self.drag else {
            let cursor = self.grip_at(event.position).map_or(CursorShape::Default, Grip::cursor);
            self.base.set_cursor(cursor);
            return;
        };
        let (dxp, dyp) = (event.position.x - d.from.x, event.position.y - d.from.y);
        d.moved = d.moved.max(dxp.abs() + dyp.abs());
        let (grip, (sx, sy, sw, sh)) = (d.grip, d.start);
        let (kx, ky) = self.scale(self.area());
        let (dx, dy) = ((dxp / kx).round(), (dyp / ky).round());
        let min = self.min_size.max(1.0);
        let (mut x, mut y, mut w, mut h) = (sx, sy, sw, sh);
        if grip.is_move() {
            x = (sx + dx).clamp(0.0, (self.source_width - sw).max(0.0));
            y = (sy + dy).clamp(0.0, (self.source_height - sh).max(0.0));
        } else {
            if grip.w {
                let x2 = (sx + dx).clamp(0.0, sx + sw - min);
                w = sx + sw - x2;
                x = x2;
            }
            if grip.e {
                w = (sw + dx).clamp(min, self.source_width - sx);
            }
            if grip.n {
                let y2 = (sy + dy).clamp(0.0, sy + sh - min);
                h = sy + sh - y2;
                y = y2;
            }
            if grip.s {
                h = (sh + dy).clamp(min, self.source_height - sy);
            }
        }
        self.set_selection(x, y, w, h);
        ctx.fire("on_change", self.selection_value());
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        let Some(d) = self.drag.take() else { return };
        self.dragging.set(false);
        if d.grip.is_move() && d.moved < 4.0 {
            (self.sel_x, self.sel_y, self.sel_width, self.sel_height) = d.start;
            ctx.fire("on_tap", EventArgs::None);
        } else {
            ctx.fire("on_end", self.selection_value());
        }
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let s = self.sel_rect(r);
        // Fora da seleção: quatro faixas escuras.
        let shade = Brush::Solid(self.shade);
        for band in [
            RectF::new(r.x, r.y, r.width, s.y - r.y),
            RectF::new(r.x, s.bottom(), r.width, r.bottom() - s.bottom()),
            RectF::new(r.x, s.y, s.x - r.x, s.height),
            RectF::new(s.right(), s.y, r.right() - s.right(), s.height),
        ] {
            if band.width > 0.0 && band.height > 0.0 {
                canvas.fill_rect(band, 0.0, &shade);
            }
        }
        let accent = ctx.theme.color("primary");
        if self.framed {
            self.paint_framed(canvas, ctx, s, accent);
            return;
        }
        canvas.stroke_rect(s.inflate(-0.75), 0.0, &Stroke::new(Color::WHITE.with_opacity(0.9), 1.5));
        // Terços, enquanto arrasta.
        if self.dragging.get() {
            let line = Stroke::new(Color::WHITE.with_opacity(0.35), 1.0);
            for i in 1..3 {
                let fx = s.x + s.width * i as f32 / 3.0;
                let fy = s.y + s.height * i as f32 / 3.0;
                canvas.draw_line(PointF::new(fx, s.y), PointF::new(fx, s.bottom()), &line);
                canvas.draw_line(PointF::new(s.x, fy), PointF::new(s.right(), fy), &line);
            }
        }
        // Alças: cantos sempre; bordas quando o lado cabe.
        let handle = |canvas: &mut dyn Canvas, cx: f32, cy: f32| {
            let h = RectF::new(cx - GRIP / 2.0, cy - GRIP / 2.0, GRIP, GRIP);
            canvas.fill_rect(h, 2.0, &Brush::Solid(Color::WHITE));
            canvas.stroke_rect(h.inflate(-0.5), 2.0, &Stroke::new(accent, 1.0));
        };
        for (cx, cy) in [(s.x, s.y), (s.right(), s.y), (s.x, s.bottom()), (s.right(), s.bottom())] {
            handle(canvas, cx, cy);
        }
        if s.width >= EDGE_MIN {
            handle(canvas, s.x + s.width / 2.0, s.y);
            handle(canvas, s.x + s.width / 2.0, s.bottom());
        }
        if s.height >= EDGE_MIN {
            handle(canvas, s.x, s.y + s.height / 2.0);
            handle(canvas, s.right(), s.y + s.height / 2.0);
        }
        // O tamanho no meio, enquanto arrasta.
        if self.dragging.get() {
            let text = if self.caption.is_empty() { format!("{}\u{00D7}{}", self.sel_width as i32, self.sel_height as i32) } else { self.caption.clone() };
            let mut style = ctx.text_style();
            style.font.size = 12.0;
            style.font.weight = FontWeight::SemiBold;
            style.color = Color::WHITE;
            style.h_align = TextAlign::Center;
            style.v_align = TextAlign::Center;
            let w = canvas.measure_text(&text, &style).width + 16.0;
            let label = RectF::new(s.x + (s.width - w) / 2.0, s.y + (s.height - 22.0) / 2.0, w, 22.0);
            canvas.fill_rect(label, 4.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.7)));
            canvas.draw_text(&text, label, &style);
        }
    }
}
