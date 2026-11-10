//! Contêineres que organizam os filhos sozinhos: `GridLayout` (grade responsiva, o
//! `v-row`/`v-col` do Vuetify ou o `TGridPanelLayout` do FMX), `FlowLayout` (em fila, quebrando
//! a linha, o `TFlowLayout`) e o `Divider` (linha divisória).
//!
//! Os filhos com `align = None` são organizados; os encaixados (Top, Client...) continuam
//! como sempre e a grade usa o espaço que sobrar.

use aylis_core::{ArrangeItem, Canvas, Component, ControlBase, PaintCtx, PointF, RectF, Stroke, TextAlign, properties};

use crate::leaf_control;

// ===========================================================================
// GridLayout
// ===========================================================================

/// Grade: os filhos em colunas de mesma largura, linha a linha. Com `columns` = 0 a
/// quantidade de colunas acompanha a largura (cada uma com pelo menos
/// `min_column_width`): a grade responsiva dos cartões. A altura da linha é a do filho
/// mais alto dela (ou `row_height`, se > 0).
pub struct GridLayout {
    pub base: ControlBase,
    pub columns: u32,
    pub min_column_width: f32,
    pub gap: f32,
    pub row_height: f32,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self { base: ControlBase::new(480.0, 320.0), columns: 0, min_column_width: 200.0, gap: 12.0, row_height: 0.0 }
    }
}

impl GridLayout {
    /// Quantas colunas cabem na largura.
    pub fn columns_for(&self, width: f32) -> usize {
        self.columns_with_gap(width, self.gap)
    }

    /// Como [`columns_for`](Self::columns_for), com o espaço já escalado pela vista.
    fn columns_with_gap(&self, width: f32, gap: f32) -> usize {
        if self.columns > 0 {
            return self.columns as usize;
        }
        (((width + gap) / (self.min_column_width.max(1.0) + gap)).floor() as usize).max(1)
    }
}

impl Component for GridLayout {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("GridLayout");

    properties! {
        columns: u32 => "Layout",
        min_column_width: f32 => "Layout",
        gap: f32 => "Layout",
        row_height: f32 => "Layout",
    }

    fn arrange(&self, area: RectF, children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        let gap = self.gap * children.first().map_or(1.0, |c| c.spacing);
        let cols = self.columns_with_gap(area.width, gap);
        let w = ((area.width - gap * (cols as f32 - 1.0)) / cols as f32).max(0.0);
        // Cada filho na linha e coluna dele: o que ocupa várias colunas (`column_span`) e não
        // cabe no que sobrou da linha vai para a próxima; mais que a grade tem, a linha toda.
        let mut cells = Vec::with_capacity(children.len());
        let (mut row, mut col) = (0usize, 0usize);
        for c in children {
            let span = (c.span.max(1) as usize).min(cols);
            if col > 0 && col + span > cols {
                (row, col) = (row + 1, 0);
            }
            cells.push((row, col, span));
            col += span;
            if col >= cols {
                (row, col) = (row + 1, 0);
            }
        }
        let rows = cells.last().map_or(0, |&(r, ..)| r + 1);
        let mut heights = vec![0.0f32; rows];
        for (c, &(r, ..)) in children.iter().zip(&cells) {
            heights[r] = if self.row_height > 0.0 { self.row_height } else { heights[r].max(c.size.height + c.margins.top + c.margins.bottom) };
        }
        let mut tops = Vec::with_capacity(rows);
        let mut y = area.y;
        for h in &heights {
            tops.push(y);
            y += h + gap;
        }
        let out = children
            .iter()
            .zip(&cells)
            .map(|(c, &(r, col, span))| {
                let x = area.x + col as f32 * (w + gap);
                let cw = w * span as f32 + gap * (span as f32 - 1.0);
                let (m, h) = (c.margins, heights[r]);
                RectF::new(x + m.left, tops[r] + m.top, (cw - m.left - m.right).max(0.0), (h - m.top - m.bottom).max(0.0))
            })
            .collect();
        Some(out)
    }
}

// ===========================================================================
// FlowLayout
// ===========================================================================

/// Os filhos em fila, no tamanho deles, quebrando a linha quando não cabem (chips,
/// etiquetas, botões de filtro). `h_align` alinha cada linha; `justify` espalha as linhas
/// cheias na largura toda (a última fica à esquerda).
pub struct FlowLayout {
    pub base: ControlBase,
    pub gap: f32,
    pub line_gap: f32,
    pub h_align: TextAlign,
}

impl Default for FlowLayout {
    fn default() -> Self {
        Self { base: ControlBase::new(320.0, 120.0), gap: 8.0, line_gap: 8.0, h_align: TextAlign::Leading }
    }
}

impl Component for FlowLayout {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("FlowLayout");

    properties! {
        gap: f32 => "Layout",
        line_gap: f32 => "Layout",
        h_align: TextAlign => "Layout",
    }

    fn arrange(&self, area: RectF, children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        // Os espaços com a escala da vista ativa.
        let spacing = children.first().map_or(1.0, |c| c.spacing);
        let (gap, line_gap) = (self.gap * spacing, self.line_gap * spacing);
        let mut out = vec![RectF::default(); children.len()];
        let mut line: Vec<usize> = Vec::new();
        let mut x = 0.0f32;
        let mut y = area.y;
        let mut line_h = 0.0f32;
        // `last`: a última linha, que no justificado fica à esquerda (como num texto).
        let flush = |line: &mut Vec<usize>, used: f32, y: f32, out: &mut Vec<RectF>, last: bool| {
            let free = (area.width - used).max(0.0);
            let shift = match self.h_align {
                TextAlign::Leading | TextAlign::Justify => 0.0,
                TextAlign::Center => free / 2.0,
                TextAlign::Trailing => free,
            };
            let extra = if self.h_align == TextAlign::Justify && !last && line.len() > 1 { free / (line.len() - 1) as f32 } else { 0.0 };
            let mut cx = area.x + shift;
            for &i in line.iter() {
                let c = &children[i];
                let m = c.margins;
                out[i] = RectF::new(cx + m.left, y + m.top, c.size.width, c.size.height);
                cx += c.size.width + m.left + m.right + gap + extra;
            }
            line.clear();
        };
        for (i, c) in children.iter().enumerate() {
            let w = c.size.width + c.margins.left + c.margins.right;
            let h = c.size.height + c.margins.top + c.margins.bottom;
            if !line.is_empty() && x + w > area.width {
                flush(&mut line, x - gap, y, &mut out, false);
                y += line_h + line_gap;
                x = 0.0;
                line_h = 0.0;
            }
            line.push(i);
            x += w + gap;
            line_h = line_h.max(h);
        }
        flush(&mut line, x - gap, y, &mut out, true);
        Some(out)
    }
}

// ===========================================================================
// Divider
// ===========================================================================

/// Linha divisória (o `v-divider`), horizontal ou vertical, com texto no meio opcional.
pub struct Divider {
    pub base: ControlBase,
    pub vertical: bool,
    pub text: String,
    pub thickness: f32,
}

impl Default for Divider {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 16.0);
        base.set_hit_test(false);
        Self { base, vertical: false, text: String::new(), thickness: 1.0 }
    }
}

impl Component for Divider {
    leaf_control!("Divider");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        vertical: bool => "Appearance",
        text: String => "Appearance",
        thickness: f32 => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = ctx.theme.color("border");
        let stroke = Stroke::new(color, self.thickness.max(0.5));
        if self.vertical {
            let x = r.x + r.width / 2.0;
            canvas.draw_line(PointF::new(x, r.y), PointF::new(x, r.bottom()), &stroke);
            return;
        }
        let y = r.y + r.height / 2.0;
        if self.text.is_empty() {
            canvas.draw_line(PointF::new(r.x, y), PointF::new(r.right(), y), &stroke);
            return;
        }
        let mut t = ctx.text_style();
        t.color = ctx.theme.color("text_muted");
        t.h_align = TextAlign::Center;
        let tw = canvas.measure_text(&self.text, &t).width + 16.0;
        let left = r.x + (r.width - tw) / 2.0;
        canvas.draw_line(PointF::new(r.x, y), PointF::new(left, y), &stroke);
        canvas.draw_line(PointF::new(left + tw, y), PointF::new(r.right(), y), &stroke);
        canvas.draw_text(&self.text, RectF::new(left, r.y, tw, r.height), &t);
    }
}
