//! A lista das mensagens (compilação, saída do programa, publicação), como a janela de
//! mensagens do Delphi: uma linha por mensagem, com a cor do tipo. Clique seleciona, Shift+clique
//! estende, Ctrl+clique alterna; Ctrl+C copia as selecionadas e Ctrl+A seleciona tudo; o duplo
//! clique numa mensagem com local abre o código na linha; o clique direito abre "Copiar" e
//! "Copiar tudo". Só as linhas à vista são desenhadas (a saída de um programa pode ter milhares).

use std::collections::BTreeSet;

use aylis_controls::PopupMenu;
use aylis_core::{
    Brush, Canvas, Component, ComponentId, ControlBase, EventCtx, FontSpec, Key, KeyEvent, PaintCtx, PointF, PointerButton,
    PointerEvent, RectF, TextAlign, WheelEvent,
};

use crate::jobs::{OutKind, OutputLine};
use crate::state::Shared;

/// Altura de uma linha.
const ROW: f32 = 18.0;
/// Recuo do texto.
const PAD: f32 = 6.0;

pub struct MessageList {
    base: ControlBase,
    state: Shared,
    lines: Vec<OutputLine>,
    selected: BTreeSet<usize>,
    /// Onde começou a seleção (o Shift+clique estende a partir dela).
    anchor: Option<usize>,
    scroll: f32,
    /// O menu de contexto ("Copiar", "Copiar tudo").
    menu: Option<ComponentId>,
}

impl MessageList {
    pub fn new(state: Shared) -> Self {
        let mut base = ControlBase::new(200.0, 100.0);
        base.set_can_focus(true).set_clip_children(true);
        Self { base, state, lines: Vec::new(), selected: BTreeSet::new(), anchor: None, scroll: 0.0, menu: None }
    }

    pub fn set_menu(&mut self, menu: ComponentId) {
        self.menu = Some(menu);
    }

    /// Troca as linhas. Se estava no fim (acompanhando), continua no fim; a seleção fica se as
    /// linhas antigas continuam lá (a saída só cresce), senão some.
    pub fn set_lines(&mut self, lines: Vec<OutputLine>) {
        let at_end = self.scroll >= self.max_scroll() - 1.0;
        let kept = lines.len() >= self.lines.len() && lines[..self.lines.len()] == self.lines[..];
        if !kept {
            self.selected.clear();
            self.anchor = None;
        }
        self.lines = lines;
        if at_end || !kept {
            self.scroll = self.max_scroll();
        }
    }

    /// O texto das linhas selecionadas (uma por linha), na ordem.
    pub fn selected_text(&self) -> String {
        self.selected.iter().filter_map(|&i| self.lines.get(i)).map(|l| l.text.as_str()).collect::<Vec<_>>().join("\n")
    }

    /// O texto de todas as linhas.
    pub fn all_text(&self) -> String {
        self.lines.iter().map(|l| l.text.as_str()).collect::<Vec<_>>().join("\n")
    }

    pub fn select_all(&mut self) {
        self.selected = (0..self.lines.len()).collect();
        self.anchor = Some(0);
    }

    pub fn selected(&self) -> Vec<usize> {
        self.selected.iter().copied().collect()
    }

    fn max_scroll(&self) -> f32 {
        (self.lines.len() as f32 * ROW - self.base.bounds().height).max(0.0)
    }

    fn row_at(&self, y: f32) -> Option<usize> {
        let i = ((y + self.scroll) / ROW).floor();
        (i >= 0.0 && (i as usize) < self.lines.len()).then_some(i as usize)
    }

    /// Clique na linha `i`: só ela, com Shift do âncora até ela, com Ctrl alterna.
    fn click_row(&mut self, i: usize, shift: bool, ctrl: bool) {
        match (shift, self.anchor) {
            (true, Some(a)) => {
                if !ctrl {
                    self.selected.clear();
                }
                self.selected.extend(a.min(i)..=a.max(i));
            }
            _ if ctrl => {
                if !self.selected.remove(&i) {
                    self.selected.insert(i);
                }
                self.anchor = Some(i);
            }
            _ => {
                self.selected = BTreeSet::from([i]);
                self.anchor = Some(i);
            }
        }
    }

    /// A linha `i` inteira à vista.
    fn reveal(&mut self, i: usize) {
        let h = self.base.bounds().height;
        let top = i as f32 * ROW;
        if top < self.scroll {
            self.scroll = top;
        } else if top + ROW > self.scroll + h {
            self.scroll = (top + ROW - h).max(0.0);
        }
    }
}

/// Copia `text` para a área de transferência.
pub fn copy_text(ui: &mut aylis_core::Ui, text: &str) {
    if let Some(c) = ui.services_mut().clipboard.as_mut() {
        c.set_text(text);
    }
}

impl Component for MessageList {
    fn class_name(&self) -> &'static str {
        "IdeMessageList"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        ctx.ui.set_focus(ctx.id);
        let row = self.row_at(event.position.y);
        match event.button {
            PointerButton::Primary => {
                let Some(i) = row else { return };
                if event.click_count >= 2 {
                    // Duplo clique: o erro abre o código na linha (a pasta publicada abre).
                    if let Some(action) = self.lines[i].action.clone() {
                        crate::jobs::activate(ctx.ui, &self.state, &action);
                    }
                } else {
                    self.click_row(i, event.modifiers.shift, event.modifiers.ctrl);
                }
                ctx.repaint();
            }
            PointerButton::Secondary => {
                if let Some(i) = row.filter(|i| !self.selected.contains(i)) {
                    self.click_row(i, false, false);
                }
                ctx.repaint();
                if let Some(menu) = self.menu {
                    let origin = ctx.ui.absolute_rect(ctx.id).map_or(PointF::ZERO, |r| r.origin());
                    let at = PointF::new(origin.x + event.position.x, origin.y + event.position.y);
                    ctx.ui.poster().post(move |ui| PopupMenu::popup(ui, menu, at));
                }
            }
            _ => {}
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let m = event.modifiers;
        match event.key {
            Key::Char('c') | Key::Char('C') if m.ctrl => {
                let text = if self.selected.is_empty() { self.all_text() } else { self.selected_text() };
                copy_text(ctx.ui, &text);
            }
            Key::Char('a') | Key::Char('A') if m.ctrl => self.select_all(),
            Key::Up | Key::Down if !self.lines.is_empty() => {
                let last = self.lines.len() - 1;
                let from = self.anchor.unwrap_or(0);
                let to = if event.key == Key::Up { from.saturating_sub(1) } else { (from + 1).min(last) };
                if m.shift {
                    let a = self.anchor.unwrap_or(to);
                    self.selected = (a.min(to)..=a.max(to)).collect();
                } else {
                    self.click_row(to, false, false);
                }
                self.anchor = if m.shift { self.anchor.or(Some(to)) } else { Some(to) };
                self.reveal(to);
            }
            Key::Enter => {
                if let Some(action) = self.anchor.and_then(|i| self.lines.get(i)).and_then(|l| l.action.clone()) {
                    crate::jobs::activate(ctx.ui, &self.state, &action);
                }
            }
            _ => return,
        }
        ctx.set_handled();
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let before = self.scroll;
        self.scroll = (self.scroll - event.delta.y).clamp(0.0, self.max_scroll());
        if self.scroll != before {
            ctx.set_handled();
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let mono = theme.fonts.get("mono").map(|f| f.family.clone()).unwrap_or_default();
        let mut style = ctx.text_style();
        style.font = FontSpec::new(mono, 12.0);
        style.h_align = TextAlign::Leading;
        style.v_align = TextAlign::Center;
        let selection = theme.color("primary").with_opacity(if ctx.state.focused { 0.28 } else { 0.16 });
        let color = |kind: OutKind| match kind {
            OutKind::Error => theme.color("danger"),
            OutKind::Warning => theme.color("warning"),
            OutKind::Success => theme.color("success"),
            OutKind::Step => theme.color("primary"),
            OutKind::Muted => theme.color("text_muted"),
            OutKind::Info | OutKind::Program => theme.color("text"),
        };
        let first = (self.scroll / ROW).floor().max(0.0) as usize;
        let count = (r.height / ROW).ceil() as usize + 1;
        for i in first..(first + count).min(self.lines.len()) {
            let y = r.y + i as f32 * ROW - self.scroll;
            let row = RectF::new(r.x, y, r.width, ROW);
            if self.selected.contains(&i) {
                canvas.fill_rect(row, 0.0, &Brush::Solid(selection));
            }
            let line = &self.lines[i];
            let mut s = style.clone();
            s.color = color(line.kind);
            if line.kind == OutKind::Step {
                s.font.weight = aylis_core::FontWeight::Bold;
            }
            canvas.draw_text(&line.text, RectF::new(r.x + PAD, y, (r.width - PAD * 2.0).max(0.0), ROW), &s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;

    fn list(n: usize) -> MessageList {
        let mut l = MessageList::new(crate::state::shared(Document::new()));
        l.set_lines((0..n).map(|i| OutputLine::new(format!("linha {i}"), OutKind::Info)).collect());
        l
    }

    /// Clique escolhe uma, Shift+clique estende do âncora, Ctrl+clique alterna; o texto copiado
    /// é o das selecionadas, na ordem.
    #[test]
    fn selection_and_copy_follow_the_clicks() {
        let mut l = list(10);
        l.click_row(2, false, false);
        l.click_row(5, true, false);
        assert_eq!(l.selected(), vec![2, 3, 4, 5]);
        l.click_row(3, false, true);
        assert_eq!(l.selected(), vec![2, 4, 5]);
        assert_eq!(l.selected_text(), "linha 2\nlinha 4\nlinha 5");
        l.select_all();
        assert_eq!(l.selected().len(), 10);
        assert!(l.all_text().starts_with("linha 0\nlinha 1"));
    }

    /// A saída cresce sem perder a seleção; outra saída (nova compilação) limpa.
    #[test]
    fn growing_output_keeps_the_selection() {
        let mut l = list(3);
        l.click_row(1, false, false);
        let mut more = l.lines.clone();
        more.push(OutputLine::new("nova", OutKind::Error));
        l.set_lines(more);
        assert_eq!(l.selected(), vec![1]);
        l.set_lines(vec![OutputLine::new("outra", OutKind::Info)]);
        assert!(l.selected().is_empty());
    }
}
