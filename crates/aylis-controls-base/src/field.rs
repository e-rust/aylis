//! A moldura dos campos: o fundo, a borda, a linha de baixo do look, o rótulo que flutua, o
//! ícone e o "×" de limpar. Uma peça só para o `Edit` e o `ComboBox`: o mesmo desenho nos
//! dois, sem cópia.
//!
//! O ícone segue o estilo do controle: `icon_size`, `icon_color` e `icon_gap`, com os estados
//! do tema (`hover`, `focused`, `disabled`...) e, com erro, o estilo `<Classe>.error` por cima
//! (só os tokens do ícone). Sem eles, os valores de antes: 1,15 vez a fonte, `text_muted` e 6.

use aylis_core::{Brush, Canvas, Color, PaintCtx, RectF, ResolvedStyle, Stroke, TextAlign};

/// A moldura de um campo.
pub struct FieldFrame<'a> {
    /// O rótulo: dentro do campo (como o placeholder) ou, com foco ou valor, pequeno sobre a
    /// borda (ou dentro, em cima, na variante `filled`).
    pub label: &'a str,
    /// Com erro: a borda e o rótulo na cor `danger`.
    pub error: bool,
    /// O campo tem valor (o rótulo sobe mesmo sem o foco).
    pub has_value: bool,
    /// Desenha a borda.
    pub border: bool,
    /// A linha de baixo do look (Material, Windows 11) com a borda (o `Edit` fora do
    /// `plain`; o `ComboBox` não a tinha e continua sem).
    pub underline: bool,
    /// O estilo do campo vazio (`Edit.empty`): o `border_style` dele troca o traço da borda.
    pub empty_style: Option<&'a str>,
}

impl FieldFrame<'_> {
    /// O fundo, a borda e a linha de baixo do look em `boxr`, no estilo `s`.
    pub fn paint_box(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: &ResolvedStyle, boxr: RectF) {
        canvas.fill_rect(boxr, s.radius, &Brush::Solid(s.fill));
        let border = if self.error { ctx.theme.color("danger") } else { s.border_color };
        if self.border && s.border_width > 0.0 && !border.is_transparent() {
            // Vazio, o estilo `Edit.empty` do tema (se houver) troca o traço da borda: o
            // contorno tracejado do campo ainda não preenchido (C37 do Tabernáculo).
            let empty_dash = (!self.has_value)
                .then(|| self.empty_style.and_then(|k| ctx.theme.style(k)))
                .flatten()
                .and_then(|st| st.base.border_style.as_deref())
                .map(|b| match b {
                    "dashed" => vec![6.0, 4.0],
                    "dotted" => vec![1.5, 3.0],
                    _ => Vec::new(),
                });
            let mut stroke = s.border_stroke(border, s.border_width.max(if self.error { 1.5 } else { 0.0 }));
            if let Some(dash) = empty_dash {
                stroke = if dash.is_empty() { Stroke::new(border, stroke.width) } else { Stroke::new(border, stroke.width).dashed(dash) };
            }
            canvas.stroke_rect(boxr, s.radius, &stroke);
        }
        if self.border && self.underline {
            crate::looks::paint_field_underline_in(canvas, ctx, boxr);
        }
    }

    /// O rótulo flutuante (com foco ou valor); devolve se flutuou. Sem flutuar, quem pinta o
    /// texto põe o rótulo no lugar do placeholder.
    pub fn paint_label(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: &ResolvedStyle, boxr: RectF) -> bool {
        let floating = !self.label.is_empty() && (ctx.state.focused || self.has_value);
        if !floating {
            return false;
        }
        let mut t = ctx.text_style();
        t.font.size = (t.font.size * 0.75).round();
        t.color = if self.error {
            ctx.theme.color("danger")
        } else if ctx.state.focused {
            ctx.theme.color("primary")
        } else {
            ctx.theme.color("text_muted")
        };
        let w = canvas.measure_text(self.label, &t).width + 8.0;
        if ctx.variant() == "filled" {
            // Campo preenchido: o rótulo fica dentro, em cima (o texto desce).
            let r = RectF::new(boxr.x + s.padding.left - 2.0, boxr.y + 4.0, w, t.font.size * 1.4);
            canvas.draw_text(self.label, r, &t);
        } else {
            let r = RectF::new(boxr.x + 8.0, boxr.y - t.font.size * 0.7, w, t.font.size * 1.4);
            let back = if s.fill.a > 200 { s.fill } else { ctx.theme.color("background") };
            canvas.fill_rect(RectF::new(r.x, boxr.y - 1.0, r.width, 3.0), 0.0, &Brush::Solid(back));
            t.h_align = TextAlign::Center;
            canvas.draw_text(self.label, r, &t);
        }
        true
    }

    /// A área de dentro: `boxr` sem o padding e, com o rótulo flutuando na variante `filled`,
    /// sem a faixa dele em cima.
    pub fn inner(&self, ctx: &PaintCtx<'_>, s: &ResolvedStyle, boxr: RectF, floating: bool) -> RectF {
        let mut inner = boxr.deflate(s.padding);
        if floating && ctx.variant() == "filled" {
            inner.y += 8.0;
            inner.height = (inner.height - 8.0).max(0.0);
        }
        inner
    }
}

/// O ícone do campo pelo estilo: o lado, a cor e o espaço até o texto.
#[derive(Clone, Copy, Debug)]
pub struct FieldIcon {
    pub side: f32,
    pub color: Color,
    pub gap: f32,
}

impl FieldIcon {
    /// O ícone de um campo da classe `class` (o nome do estilo), com erro ou não: o estilo
    /// resolvido (com os estados) e, com erro, o `<class>.error` do tema por cima.
    pub fn of(ctx: &PaintCtx<'_>, class: &str, error: bool) -> FieldIcon {
        let s = ctx.style;
        let mut icon = FieldIcon::with_font(s.font.size, ctx.theme.color("text_muted"));
        icon.side = s.icon_size.unwrap_or(icon.side);
        icon.color = s.icon_color.unwrap_or(icon.color);
        icon.gap = s.icon_gap.unwrap_or(icon.gap);
        if error {
            if let Some(e) = ctx.theme.style(&format!("{class}.error")).map(|st| &st.base) {
                icon.side = e.icon_size.unwrap_or(icon.side);
                icon.color = e.icon_color.as_ref().map_or(icon.color, |c| ctx.theme.color_of(c));
                icon.gap = e.icon_gap.unwrap_or(icon.gap);
            }
        }
        icon.side = icon.side.max(0.0);
        icon.gap = icon.gap.max(0.0);
        icon
    }

    /// O mesmo fora da pintura (o `auto_size`): o estilo de `id` resolvido no tema dele (a
    /// cor não importa para medir).
    pub fn of_ui(ui: &aylis_core::Ui, id: aylis_core::ComponentId, class: &str, error: bool) -> FieldIcon {
        let theme = ui.theme_of(id);
        let Some((component, cb)) = ui.component(id).and_then(|c| c.control().map(|cb| (c, cb))) else {
            return FieldIcon::with_font(ui.font_of(id).size, Color::TRANSPARENT);
        };
        let mut classes = Vec::new();
        component.class_chain(&mut classes);
        let s = theme.resolve(&classes, cb.style(), cb.state());
        let mut icon = FieldIcon::with_font(ui.font_of(id).size, Color::TRANSPARENT);
        icon.side = s.icon_size.unwrap_or(icon.side);
        icon.gap = s.icon_gap.unwrap_or(icon.gap);
        if let Some(e) = error.then(|| theme.style(&format!("{class}.error"))).flatten().map(|st| &st.base) {
            icon.side = e.icon_size.unwrap_or(icon.side);
            icon.gap = e.icon_gap.unwrap_or(icon.gap);
        }
        icon.side = icon.side.max(0.0);
        icon.gap = icon.gap.max(0.0);
        icon
    }

    /// Os valores de antes do estilo, pela fonte do campo.
    pub fn with_font(font_size: f32, color: Color) -> FieldIcon {
        FieldIcon { side: (font_size * 1.15).round(), color, gap: 6.0 }
    }

    /// O lado do "×" de limpar.
    pub fn clear_side(&self) -> f32 {
        (self.side * 0.85).round()
    }

    /// O ícone à esquerda de `inner`, que perde o lugar dele.
    pub fn paint_leading(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, name: &str, inner: &mut RectF) {
        let r = RectF::new(inner.x, inner.y + (inner.height - self.side) / 2.0, self.side, self.side);
        ctx.draw_icon(canvas, name, r, self.color);
        inner.x += self.side + self.gap;
        inner.width = (inner.width - self.side - self.gap).max(0.0);
    }

    /// O "×" de limpar à direita de `inner`, que perde o lugar dele; devolve onde ficou (o
    /// clique).
    pub fn paint_clear(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, inner: &mut RectF) -> RectF {
        let side = self.clear_side();
        let r = RectF::new(inner.right() - side, inner.y + (inner.height - side) / 2.0, side, side);
        ctx.draw_icon(canvas, "close", r, self.color);
        inner.width = (inner.width - side - 4.0).max(0.0);
        r
    }
}
