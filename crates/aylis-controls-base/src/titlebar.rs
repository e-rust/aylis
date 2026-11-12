//! TitleBar: a barra de título própria (com `border_style = custom` no form). Mostra o
//! título e os botões minimizar, maximizar/restaurar e fechar; os controles postos dentro
//! dela (botões, busca, abas) ficam à esquerda dos botões padrão. Arrastar a barra move a
//! janela e o duplo clique maximiza, como a do sistema. No Windows 11, o mouse parado no botão
//! maximizar abre o menu de layouts do próprio sistema (Snap), como na barra do Windows.

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, EventCtx, PaintCtx, PointF, PointerButton, PointerEvent, PropValue, RectF, Stroke,
    WindowCommand, properties,
};

use crate::leaf_control;

const BUTTON: f32 = 46.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Minimize,
    Maximize,
    Close,
}

/// Barra de título própria.
pub struct TitleBar {
    pub base: ControlBase,
    /// Vazio: o `caption` do form.
    pub caption: String,
    pub show_minimize: bool,
    pub show_maximize: bool,
    pub show_close: bool,
    hover: Cell<Option<Part>>,
    pressed: Cell<Option<Part>>,
}

impl Default for TitleBar {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 36.0);
        base.set_align(aylis_core::Align::Top);
        let mut bar = Self {
            base,
            caption: String::new(),
            show_minimize: true,
            show_maximize: true,
            show_close: true,
            hover: Cell::new(None),
            pressed: Cell::new(None),
        };
        bar.sync_padding();
        bar
    }
}

impl TitleBar {
    /// Os botões padrão visíveis, da esquerda para a direita, com o retângulo de cada um.
    fn buttons(&self, r: RectF) -> Vec<(Part, RectF)> {
        let parts: Vec<Part> = [(Part::Minimize, self.show_minimize), (Part::Maximize, self.show_maximize), (Part::Close, self.show_close)]
            .into_iter()
            .filter(|(_, on)| *on)
            .map(|(p, _)| p)
            .collect();
        let start = r.right() - BUTTON * parts.len() as f32;
        parts.into_iter().enumerate().map(|(i, p)| (p, RectF::new(start + BUTTON * i as f32, r.y, BUTTON, r.height))).collect()
    }

    fn part_at(&self, p: PointF) -> Option<Part> {
        let r = RectF::from_size(self.base.bounds().size());
        self.buttons(r).into_iter().find(|(_, b)| b.contains(p)).map(|(part, _)| part)
    }

    /// Diz ao `Ui` onde fica o botão maximizar (no lugar da barra): a plataforma o entrega ao
    /// sistema como o botão da janela, e no Windows 11 o mouse parado nele abre o menu de
    /// layouts (Snap), como na barra do sistema.
    fn report_maximize(&self, ctx: &mut EventCtx<'_>) {
        let r = RectF::from_size(self.base.bounds().size());
        let rect = self.buttons(r).into_iter().find(|(p, _)| *p == Part::Maximize).map(|(_, b)| b);
        ctx.ui.set_maximize_button(ctx.id, rect);
    }

    /// Os filhos (botões próprios) não invadem os botões padrão.
    fn sync_padding(&mut self) {
        let n = [self.show_minimize, self.show_maximize, self.show_close].iter().filter(|b| **b).count();
        let p = self.base.padding();
        self.base.set_padding([p.left, p.top, BUTTON * n as f32 + 4.0, p.bottom]);
    }
}

impl Component for TitleBar {
    leaf_control!("TitleBar");

    properties! {
        caption: String => "Appearance",
        show_minimize: bool => "Appearance",
        show_maximize: bool => "Appearance",
        show_close: bool => "Appearance"
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, _name: &str) {
        self.sync_padding();
        self.report_maximize(ctx);
        ctx.invalidate();
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        self.report_maximize(ctx);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let part = self.part_at(event.position);
        if part != self.hover.get() {
            self.hover.set(part);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(None);
        self.pressed.set(None);
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if let Some(part) = self.part_at(event.position) {
            self.pressed.set(Some(part));
            ctx.repaint();
        } else if event.click_count == 2 && self.show_maximize {
            ctx.ui.window_command(ctx.id, WindowCommand::ToggleMaximize);
        } else {
            ctx.ui.window_command(ctx.id, WindowCommand::StartDrag);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let pressed = self.pressed.take();
        if pressed.is_some() && pressed == self.part_at(event.position) {
            let command = match pressed {
                Some(Part::Minimize) => WindowCommand::Minimize,
                Some(Part::Maximize) => WindowCommand::ToggleMaximize,
                _ => WindowCommand::Close,
            };
            ctx.ui.window_command(ctx.id, command);
        }
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let fill = if ctx.style.fill.a > 0 { ctx.style.fill } else { theme.color("surface") };
        canvas.fill_rect(r, 0.0, &Brush::Solid(fill));
        let text_color = theme.color("text");
        // O título: o próprio ou o do form.
        let root = ctx.ui.root_of(ctx.id);
        let caption = if self.caption.is_empty() {
            match ctx.ui.get_prop(root, "caption") {
                Ok(PropValue::String(c)) => c,
                _ => String::new(),
            }
        } else {
            self.caption.clone()
        };
        let buttons = self.buttons(r);
        let first = buttons.first().map_or(r.right(), |(_, b)| b.x);
        let mut t = ctx.text_style();
        t.color = text_color;
        // Os filhos (botões próprios) ficam à esquerda; o título depois deles.
        let left = r.x + self.base.padding().left + 12.0;
        let after_children = ctx.ui.children(ctx.id).iter().filter_map(|&c| ctx.ui.laid_out_bounds(c)).map(|b| b.right() + 12.0).fold(left, f32::max);
        canvas.draw_text(&ctx.ui.tr(&caption), RectF::new(after_children, r.y, (first - after_children - 8.0).max(0.0), r.height), &t);
        let maximized = ctx.ui.is_maximized(root);
        for (part, b) in buttons {
            let hover = self.hover.get() == Some(part);
            let pressed = self.pressed.get() == Some(part);
            let red = part == Part::Close && (hover || pressed);
            if red {
                canvas.fill_rect(b, 0.0, &Brush::Solid(if pressed { Color::rgb(0xF1, 0x70, 0x7A) } else { Color::rgb(0xC4, 0x2B, 0x1C) }));
            } else if hover || pressed {
                canvas.fill_rect(b, 0.0, &Brush::Solid(text_color.with_opacity(if pressed { 0.14 } else { 0.08 })));
            }
            let stroke = Stroke::new(if red { Color::WHITE } else { text_color }, 1.0);
            let c = PointF::new(b.x + b.width / 2.0, b.y + b.height / 2.0);
            match part {
                Part::Minimize => canvas.draw_line(PointF::new(c.x - 5.0, c.y), PointF::new(c.x + 5.0, c.y), &stroke),
                Part::Maximize if maximized => {
                    // Restaurar: dois quadrados.
                    canvas.stroke_rect(RectF::new(c.x - 5.0, c.y - 3.0, 8.0, 8.0), 1.0, &stroke);
                    canvas.draw_line(PointF::new(c.x - 3.0, c.y - 5.0), PointF::new(c.x + 5.0, c.y - 5.0), &stroke);
                    canvas.draw_line(PointF::new(c.x + 5.0, c.y - 5.0), PointF::new(c.x + 5.0, c.y + 3.0), &stroke);
                }
                Part::Maximize => canvas.stroke_rect(RectF::new(c.x - 5.0, c.y - 5.0, 10.0, 10.0), 1.0, &stroke),
                Part::Close => {
                    canvas.draw_line(PointF::new(c.x - 5.0, c.y - 5.0), PointF::new(c.x + 5.0, c.y + 5.0), &stroke);
                    canvas.draw_line(PointF::new(c.x + 5.0, c.y - 5.0), PointF::new(c.x - 5.0, c.y + 5.0), &stroke);
                }
            }
        }
    }
}
