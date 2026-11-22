//! [`ListHeader`]: o cabeçalho opcional de um `VirtualList` (ou de um `DataRepeater`). Decidido em
//! 22/11 (o relatório de um player de música portado para o Aylis) sem impor colunas à lista: a
//! linha continua um desenho livre, e o cabeçalho só alinha cada título a um componente do modelo
//! pelo nome (`part`), lendo a posição dele na primeira linha à vista. A linha que não tem aquele
//! componente, ou que o esconde, não participa.
//!
//! Arrastar a divisa de um título muda a largura do componente em todas as linhas (e no modelo);
//! clicar num título com `sortable` troca a ordem e avisa o `on_sort` (o app ordena os próprios
//! dados: a lista não tem dados para ordenar). Sem linha à vista, os títulos ficam lado a lado
//! com as larguras dadas.

use std::cell::Cell;

use aylis_core::{Brush, Canvas, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, PaintCtx, PointerEvent, PropKind, PropValue, RectF, Stroke, TextAlign, Ui, properties};

use crate::VirtualList;
use crate::leaf_control;

/// O registro de uma coluna do cabeçalho.
pub const HEADER_COLUMN: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "HeaderColumn",
    fields: &[
        ("title", PropKind::String),
        ("part", PropKind::String),
        ("width", PropKind::Float),
        ("min_width", PropKind::Float),
        ("align", PropKind::String),
        ("sortable", PropKind::Bool),
        ("resizable", PropKind::Bool),
    ],
};

/// Uma coluna: o título e o componente do modelo que ela acompanha.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HeaderColumn {
    pub title: String,
    /// O nome do componente do modelo da linha (`lblTitulo`).
    pub part: String,
    /// A largura inicial do componente (0: a do modelo).
    pub width: f64,
    /// O mínimo ao arrastar a divisa (0: 24).
    pub min_width: f64,
    /// `left` (o padrão), `center` ou `right`.
    pub align: String,
    pub sortable: bool,
    pub resizable: bool,
}

/// A faixa sensível à divisa de um título (de cada lado).
const GRIP: f32 = 4.0;

/// O cabeçalho de uma lista.
pub struct ListHeader {
    pub base: ControlBase,
    /// O `VirtualList` (ou `DataRepeater`) que o cabeçalho acompanha.
    pub list: Option<ComponentId>,
    /// As colunas: a lista de [`HEADER_COLUMN`] em JSON.
    pub columns: String,
    /// A coluna da ordem de agora (o `part`; vazio: nenhuma): a seta aparece nela.
    pub sort_part: String,
    /// A ordem da coluna `sort_part` é decrescente (a seta para baixo).
    pub sort_descending: bool,
    /// Arrastando a divisa: a coluna, onde o mouse começou e a largura de então.
    drag: Option<(usize, f32, f32)>,
    /// O mouse andou desde o apertar (não é clique).
    moved: Cell<bool>,
}

impl Default for ListHeader {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 32.0);
        base.set_cursor(CursorShape::Default);
        Self { base, list: None, columns: String::new(), sort_part: String::new(), sort_descending: false, drag: None, moved: Cell::new(false) }
    }
}

impl ListHeader {
    fn parsed(&self) -> Vec<HeaderColumn> {
        crate::records::parse_list(&self.columns)
    }

    /// Onde fica cada coluna no cabeçalho: a posição do componente na primeira linha à vista; sem
    /// linha (ou sem o componente nela), logo depois da anterior com a largura dada.
    fn spans(&self, ui: &Ui, id: ComponentId) -> Vec<(f32, f32)> {
        let me = ui.absolute_rect(id).unwrap_or_default();
        let first = self.list.and_then(|l| VirtualList::visible(ui, l).first().copied());
        let mut x = 0.0f32;
        self.parsed()
            .iter()
            .map(|c| {
                let real = first
                    .and_then(|i| VirtualList::part_named(ui, self.list?, i, c.part.trim()))
                    .filter(|p| ui.control(*p).is_some_and(|cb| cb.visible()))
                    .and_then(|p| ui.absolute_rect(p));
                let span = match real {
                    Some(r) => (r.x - me.x, r.width),
                    None => (x, if c.width > 0.0 { c.width as f32 } else { 120.0 }),
                };
                x = span.0 + span.1;
                span
            })
            .collect()
    }

    /// Põe a largura `width` no componente `part` de todas as linhas e do modelo.
    fn apply_width(ui: &mut Ui, list: ComponentId, part: &str, width: f32) {
        for row in VirtualList::rows_and_templates(ui, list) {
            if let Some(p) = ui.find_in(row, part) {
                let _ = ui.set_prop(p, "width", f64::from(width));
            }
        }
        ui.invalidate();
    }

    /// As larguras dadas nas colunas, no modelo e nas linhas (ao ligar a lista ou trocar as colunas).
    fn apply_widths(ui: &mut Ui, id: ComponentId) {
        let Some(h) = ui.get::<ListHeader>(id) else { return };
        let Some(list) = h.list else { return };
        for c in h.parsed().into_iter().filter(|c| c.width > 0.0) {
            Self::apply_width(ui, list, c.part.trim(), c.width as f32);
        }
    }

    /// A coluna cuja divisa (a borda direita) está em `x`.
    fn grip_at(&self, spans: &[(f32, f32)], cols: &[HeaderColumn], x: f32) -> Option<usize> {
        spans.iter().zip(cols).position(|((left, width), c)| c.resizable && (x - (left + width)).abs() <= GRIP)
    }
}

impl Component for ListHeader {
    leaf_control!("ListHeader");

    properties! {
        list: Option<ComponentId> => "Behavior",
        columns: String as RecordList(&HEADER_COLUMN) => "Data",
        sort_part: String => "Data",
        sort_descending: bool => "Data";
        events: on_sort: String, on_column_resize: String
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "list" | "columns") {
            let id = ctx.id;
            ctx.ui.poster().post_to(id, move |ui| ListHeader::apply_widths(ui, id));
        }
        ctx.repaint();
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.list == Some(freed) {
            self.list = None;
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let cols = self.parsed();
        let spans = self.spans(ctx.ui, ctx.id);
        self.moved.set(false);
        if let Some(i) = self.grip_at(&spans, &cols, event.position.x) {
            self.drag = Some((i, event.position.x, spans[i].1));
            ctx.set_handled();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let cols = self.parsed();
        if let (Some((i, start, width)), Some(list)) = (self.drag, self.list) {
            self.moved.set(true);
            let min = if cols[i].min_width > 0.0 { cols[i].min_width as f32 } else { 24.0 };
            let w = (width + event.position.x - start).max(min);
            ListHeader::apply_width(ctx.ui, list, cols[i].part.trim(), w);
            ctx.repaint();
            return;
        }
        // A divisa mostra o cursor de redimensionar.
        let spans = self.spans(ctx.ui, ctx.id);
        let over = self.grip_at(&spans, &cols, event.position.x).is_some();
        self.base.set_cursor(if over { CursorShape::ResizeHorizontal } else { CursorShape::Default });
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let cols = self.parsed();
        if let Some((i, _, _)) = self.drag.take() {
            if self.moved.get() {
                ctx.fire("on_column_resize", EventArgs::Text(cols[i].part.trim().to_string()));
            }
            return;
        }
        // O clique num título que ordena: a mesma coluna inverte; outra começa crescente.
        let spans = self.spans(ctx.ui, ctx.id);
        let Some(i) = spans.iter().position(|(left, width)| event.position.x >= *left && event.position.x < left + width) else { return };
        if !cols[i].sortable {
            return;
        }
        let part = cols[i].part.trim().to_string();
        self.sort_descending = self.sort_part == part && !self.sort_descending;
        self.sort_part = part.clone();
        ctx.repaint();
        ctx.fire("on_sort", EventArgs::Text(part));
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let fill = ctx.colors.fill.unwrap_or(ctx.style.fill);
        if !fill.is_transparent() {
            canvas.fill_rect(r, 0.0, &Brush::Solid(fill));
        }
        let line = ctx.theme.color("border");
        canvas.draw_line(aylis_core::PointF::new(r.x, r.bottom() - 0.5), aylis_core::PointF::new(r.right(), r.bottom() - 0.5), &Stroke::new(line, 1.0));
        let mut style = ctx.text_style();
        style.color = ctx.colors.text.unwrap_or_else(|| ctx.theme.color("text_muted"));
        style.v_align = TextAlign::Center;
        let cols = self.parsed();
        for ((left, width), c) in self.spans(ctx.ui, ctx.id).into_iter().zip(&cols) {
            let cell = RectF::new(r.x + left, r.y, width.max(0.0), r.height);
            let mut s = style.clone();
            s.h_align = match c.align.trim() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Trailing,
                _ => TextAlign::Leading,
            };
            // A seta da ordem na coluna escolhida.
            let title = if !self.sort_part.is_empty() && self.sort_part == c.part.trim() {
                format!("{} {}", c.title, if self.sort_descending { "▼" } else { "▲" })
            } else {
                c.title.clone()
            };
            canvas.draw_text(&title, cell, &s);
            if c.resizable {
                let x = cell.right() - 0.5;
                canvas.draw_line(aylis_core::PointF::new(x, r.y + 8.0), aylis_core::PointF::new(x, r.bottom() - 8.0), &Stroke::new(line, 1.0));
            }
        }
        let _ = PropValue::Null;
    }
}
