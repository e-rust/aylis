//! A barra de rolagem pelo tema: o estilo `ScrollBar` do tema, compartilhado por todos os
//! controles que rolam, e o `scroll_bar` do estilo de um controle (ou de uma variante), que
//! muda só a barra dele:
//!
//! ```json
//! "ScrollBar": { "width": 6, "fill": "transparent", "thumb": "text_muted/40", "radius": 3,
//!                "hover": { "thumb": "text_muted/70" } },
//! "ScrollBox.fina": { "scroll_bar": { "width": 4 } }
//! ```
//!
//! O que o tema não diz fica como era em cada controle (largura, cores e raio de antes).

use aylis_core::{Brush, Canvas, Color, Component, ComponentId, PaintCtx, RectF, ScrollBarLook, Ui};

/// A barra do controle que pinta: o `ScrollBar` do tema e o `scroll_bar` do estilo dele.
pub fn look(ctx: &PaintCtx<'_>, hover: bool) -> ScrollBarLook {
    ctx.theme.scroll_bar(ctx.style.scroll_bar.as_deref(), hover)
}

/// A barra fora da pintura (num evento, no layout): o estilo do controle `component`
/// (que pode estar fora do `Ui` durante o evento) no tema dele.
pub fn look_of(ui: &Ui, id: ComponentId, component: &dyn Component, hover: bool) -> ScrollBarLook {
    let theme = ui.theme_of(id);
    let Some(cb) = component.control() else { return theme.scroll_bar(None, hover) };
    let mut classes = Vec::new();
    component.class_chain(&mut classes);
    let own = theme.resolve(&classes, cb.style(), cb.state()).scroll_bar;
    theme.scroll_bar(own.as_deref(), hover)
}

/// Os valores de um controle para a barra: os de antes do tema, usados no que o tema não diz.
#[derive(Clone, Copy, Debug)]
pub struct Defaults {
    /// A largura da faixa da barra.
    pub width: f32,
    /// A cor do trilho (`None`: sem trilho).
    pub track: Option<Color>,
    /// A cor do polegar de antes (o controle já escolhe a de arrastando: sem tema, o mouse
    /// em cima não muda nada, como antes).
    pub thumb: Color,
}

/// A barra já decidida: o tema por cima dos valores do controle.
#[derive(Clone, Copy, Debug)]
pub struct Bar {
    pub width: f32,
    pub track: Option<Color>,
    pub thumb: Color,
    /// O raio do polegar (`None`: o do controle, em geral meia largura).
    pub radius: Option<f32>,
}

/// Junta o tema (o estado `hover` do estilo com o mouse no polegar ou arrastando) e os
/// valores do controle.
pub fn resolve(ctx: &PaintCtx<'_>, hover: bool, defaults: Defaults) -> Bar {
    let l = look(ctx, hover);
    Bar {
        width: l.width.unwrap_or(defaults.width),
        track: l.track.or(defaults.track),
        thumb: l.thumb.unwrap_or(defaults.thumb),
        radius: l.radius,
    }
}

impl Bar {
    /// Pinta o trilho (se houver cor) e o polegar, com o raio do tema ou `radius`.
    pub fn paint(&self, canvas: &mut dyn Canvas, track: Option<RectF>, thumb: RectF, radius: f32) {
        if let (Some(t), Some(color)) = (track, self.track) {
            if !color.is_transparent() {
                canvas.fill_rect(t, self.radius.unwrap_or(0.0), &Brush::Solid(color));
            }
        }
        let r = self.radius.unwrap_or(radius).min(thumb.width.min(thumb.height) / 2.0).max(0.0);
        canvas.fill_rect(thumb, r, &Brush::Solid(self.thumb));
    }
}
