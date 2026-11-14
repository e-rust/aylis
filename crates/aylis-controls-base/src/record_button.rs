//! [`RecordButton`]: o botão redondo de gravar dos gravadores de tela e de áudio.

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, KeyEvent, PaintCtx, Path, PointF, RectF, Shadow, Stroke,
    TextAlign, TextStyle, properties,
};

use crate::leaf_control;

/// Botão redondo de gravar: um círculo vermelho com o ponto branco no meio que vira um
/// quadrado (parar) gravando, com o brilho pulsando. `paused` para o pulso; `finalizing`
/// gira um arco verde em volta (o arquivo ainda está sendo fechado) e ignora o clique;
/// `countdown` > 0 mostra os segundos no meio e um anel âmbar que esvazia
/// (`countdown_fraction` de 1 a 0). Desabilitado (`enabled: false`) fica cinza.
/// `on_click` no clique.
pub struct RecordButton {
    pub base: ControlBase,
    /// Gravando: o ponto vira o quadrado de parar e o brilho pulsa.
    pub recording: bool,
    /// Gravação pausada: o brilho para de pulsar.
    pub paused: bool,
    /// Fechando o arquivo: um arco verde gira em volta e o clique é ignorado.
    pub finalizing: bool,
    /// Os segundos da contagem antes de gravar, no meio do botão; 0 = sem contagem.
    pub countdown: u32,
    /// Quanto da contagem falta, de 1 (o anel âmbar cheio) a 0 (vazio).
    pub countdown_fraction: f32,
    /// O brilho colorido em volta (desligue com uma camada translúcida por cima, onde ele
    /// vira pontos soltos em alguns renderizadores).
    pub glow: bool,
    /// Quando o último número da contagem apareceu (o "pulo" do número); `NaN` = acabou
    /// de mudar (a próxima pintura marca a hora).
    countdown_at: std::cell::Cell<f64>,
}

impl Default for RecordButton {
    fn default() -> Self {
        let mut base = ControlBase::new(72.0, 72.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, recording: false, paused: false, finalizing: false, countdown: 0, countdown_fraction: 1.0, glow: true, countdown_at: std::cell::Cell::new(0.0) }
    }
}

/// Período do pulso gravando e a volta do arco de finalizando, em segundos.
const PULSE: f64 = 1.8;
const SPIN: f64 = 0.9;

impl RecordButton {
    fn sync(&mut self, name: &str) {
        if name == "countdown" {
            self.countdown_at.set(f64::NAN);
        }
    }
}

fn rgb(hex: u32) -> Color {
    Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

impl Component for RecordButton {
    leaf_control!("RecordButton");

    properties! {
        recording: bool => "Data",
        paused: bool => "Data",
        finalizing: bool => "Data",
        countdown: u32 => "Data",
        countdown_fraction: f32 => "Data",
        glow: bool => "Appearance";
        events: on_click;
        on_set: sync
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.finalizing {
            return;
        }
        ctx.fire("on_click", EventArgs::None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        crate::toggles::space_clicks(self, ctx, event);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let size = r.width.min(r.height);
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let dark = ctx.theme.dark;
        let disabled = ctx.state.disabled && !self.recording;
        let hover = ctx.state.hover && !disabled && !self.finalizing;
        let base = if disabled {
            ctx.theme.color("border")
        } else if self.recording {
            rgb(if hover { 0xF87171 } else { 0xEF4444 })
        } else {
            rgb(if hover { 0xC43A37 } else { 0xB1322F })
        };
        let ring = if disabled { ctx.theme.color("border") } else if self.recording { rgb(0xEF4444) } else { base };
        // Pulso gravando (o brilho cresce e volta).
        let pulse = if self.recording && !self.paused {
            ctx.request_frame();
            (0.5 - 0.5 * (std::f64::consts::TAU * (ctx.time % PULSE) / PULSE).cos()) as f32
        } else {
            0.5
        };
        let scale = if ctx.state.pressed && !disabled { 0.94 } else { 1.0 };
        let d = size * scale;
        let circle = |grow: f32| RectF::new(c.x - d / 2.0 - grow, c.y - d / 2.0 - grow, d + 2.0 * grow, d + 2.0 * grow);
        canvas.save();
        if self.finalizing {
            canvas.save_layer(0.75);
        }
        // Sombra escura de baixo e o brilho colorido, atrás do anel de 2 px.
        let outer = circle(2.0);
        let radius = outer.width / 2.0;
        if !disabled {
            let shadow = Color::rgb(0, 0, 0).with_opacity(if dark { 0.38 } else { 0.13 });
            canvas.draw_shadow(outer, radius, &Shadow { color: shadow, blur: 16.0, offset: PointF::new(0.0, 6.0), spread: 0.0 });
            let (blur, alpha) = if self.recording {
                if dark { (24.0 + 24.0 * pulse, 0.4 + 0.25 * pulse) } else { (18.0 + 14.0 * pulse, 0.25 + 0.2 * pulse) }
            } else if hover {
                if dark { (32.0, 0.45) } else { (22.0, 0.28) }
            } else if dark {
                (26.0, 0.35)
            } else {
                (18.0, 0.2)
            };
            if self.glow {
                canvas.draw_shadow(outer, radius, &Shadow { color: base.with_opacity(alpha), blur, offset: PointF::ZERO, spread: 0.0 });
            }
        }
        canvas.fill_ellipse(outer, &Brush::Solid(ring));
        // Miolo com a borda de 3 px na cor do fundo.
        canvas.fill_ellipse(circle(0.0), &Brush::Solid(ctx.theme.color("surface")));
        canvas.fill_ellipse(circle(-3.0), &Brush::Solid(base));
        // Ponto branco → quadrado arredondado gravando (some na contagem).
        if self.countdown == 0 {
            let k = d / 72.0;
            let (side, corner) = if self.recording { (20.0 * k, 4.0 * k) } else { (24.0 * k, 12.0 * k) };
            let mark = if disabled { ctx.theme.color("text_muted") } else { Color::WHITE.with_opacity(0.95) };
            canvas.fill_rect(RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side), corner, &Brush::Solid(mark));
        }
        if self.finalizing {
            canvas.restore();
            // Arco verde girando em volta (um quarto de cima e um da direita).
            ctx.request_frame();
            let start = 360.0 * ((ctx.time % SPIN) / SPIN) as f32 - 90.0;
            let mut arc = Path::new();
            let rr = d / 2.0 + 4.5;
            arc.arc(c, rr, rr, start, 180.0);
            canvas.stroke_path(&arc, &Stroke::new(ctx.theme.color("success"), 3.0));
        }
        if self.countdown > 0 {
            let amber = rgb(0xF59E0B);
            let rr = d / 2.0 + 10.5;
            let track = RectF::new(c.x - rr, c.y - rr, 2.0 * rr, 2.0 * rr);
            canvas.stroke_ellipse(track, &Stroke::new(amber.with_opacity(0.18), 3.0));
            let f = self.countdown_fraction.clamp(0.0, 1.0);
            if f >= 0.999 {
                canvas.stroke_ellipse(track, &Stroke::new(amber, 3.0));
            } else if f > 0.0 {
                let mut arc = Path::new();
                arc.arc(c, rr, rr, -90.0, 360.0 * f);
                canvas.stroke_path(&arc, &Stroke::new(amber, 3.0).round());
            }
            // O número "pula" a cada segundo (encolhe de 1,35 para 1 em 0,35 s).
            if self.countdown_at.get().is_nan() {
                self.countdown_at.set(ctx.time);
            }
            let age = ctx.time - self.countdown_at.get();
            let k = (age / 0.35).clamp(0.0, 1.0) as f32;
            let e = 1.0 - (1.0 - k) * (1.0 - k);
            if k < 1.0 {
                ctx.request_frame();
            }
            let mut style = TextStyle { color: Color::WHITE.with_opacity(0.4 + 0.6 * e), h_align: TextAlign::Center, v_align: TextAlign::Center, ..ctx.text_style() };
            style.font.size = 28.0 * (1.35 - 0.35 * e) * d / 72.0;
            style.font.weight = aylis_core::FontWeight::Bold;
            canvas.draw_text(&self.countdown.to_string(), RectF::new(r.x, c.y - style.font.size, r.width, style.font.size * 2.0), &style);
        }
        canvas.restore();
    }
}
