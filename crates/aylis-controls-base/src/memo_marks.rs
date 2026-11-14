//! Marcadores no código: numerados de 0 a 9 (Ctrl+Shift+número marca ou
//! desmarca a linha, Ctrl+número vai até ela; Ctrl+B põe o primeiro número livre a partir do 1, ou
//! tira o marcador da linha) e sem número (Ctrl+Alt+K marca, Ctrl+Alt+L vai
//! ao próximo e Ctrl+Alt+J ao anterior). Aparecem na calha e acompanham as linhas inseridas e
//! apagadas acima deles; cada mudança dispara `on_bookmarks_change` (quem usa o editor guarda).

use aylis_core::{Brush, Canvas, EventArgs, EventCtx, Key, KeyEvent, RectF, TextAlign, TextStyle};

use crate::Memo;

/// Um marcador: a linha (a partir de 0) e o número, se tiver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bookmark {
    pub line: usize,
    pub number: Option<u8>,
}

/// O número de uma tecla de dígito, com ou sem o Shift (no teclado americano e no ABNT2, Shift+1
/// chega como `!`, Shift+6 como `^` ou `¨`...).
fn digit_of(c: char) -> Option<u8> {
    let shifted = [')', '!', '@', '#', '$', '%', '^', '&', '*', '('];
    if let Some(d) = c.to_digit(10) {
        return Some(d as u8);
    }
    if c == '¨' {
        return Some(6);
    }
    shifted.iter().position(|&s| s == c).map(|i| i as u8)
}

impl Memo {
    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Troca os marcadores (os guardados de outra vez); fora do texto são descartados.
    pub fn set_bookmarks(&mut self, marks: Vec<Bookmark>) {
        let lines = self.text.split('\n').count();
        self.bookmarks = marks.into_iter().filter(|m| m.line < lines).collect();
        self.bookmarks.sort_by_key(|m| m.line);
        self.sync_marks();
    }

    /// Marca a linha do cursor com `number` (ou tira, se ela já tem esse número); o número sai
    /// da linha onde estava, e a linha fica com um marcador só, o último (antes,
    /// Ctrl+Shift+1 e depois Ctrl+Shift+2 deixavam os dois na linha, conferência de 08/11).
    pub fn toggle_numbered_bookmark(&mut self, number: u8) {
        let line = self.caret_line() - 1;
        let here = self.bookmarks.iter().any(|m| m.line == line && m.number == Some(number));
        self.bookmarks.retain(|m| m.number != Some(number) && m.line != line);
        if !here {
            self.bookmarks.push(Bookmark { line, number: Some(number) });
        }
        self.bookmarks.sort_by_key(|m| m.line);
        self.sync_marks();
    }

    /// Marca a linha do cursor sem número (ou tira o marcador dela).
    pub fn toggle_bookmark(&mut self) {
        let line = self.caret_line() - 1;
        if self.bookmarks.iter().any(|m| m.line == line) {
            self.bookmarks.retain(|m| m.line != line);
        } else {
            self.bookmarks.push(Bookmark { line, number: None });
            self.bookmarks.sort_by_key(|m| m.line);
        }
        self.sync_marks();
    }

    /// Ctrl+B: a linha do cursor com marcador perde o marcador; sem, ganha o primeiro número
    /// livre, começando pelo 1 (1 a 9, depois o 0). Com os dez em uso, um sem número.
    pub fn toggle_next_bookmark(&mut self) {
        let line = self.caret_line() - 1;
        if self.bookmarks.iter().any(|m| m.line == line) {
            self.bookmarks.retain(|m| m.line != line);
        } else {
            let free = (1..=9).chain([0]).find(|n| !self.bookmarks.iter().any(|m| m.number == Some(*n)));
            self.bookmarks.push(Bookmark { line, number: free });
            self.bookmarks.sort_by_key(|m| m.line);
        }
        self.sync_marks();
    }

    /// Vai até o marcador `number`; `false` se não há.
    pub fn go_to_bookmark(&mut self, number: u8) -> bool {
        let Some(line) = self.bookmarks.iter().find(|m| m.number == Some(number)).map(|m| m.line) else { return false };
        self.go_to_line(line);
        true
    }

    /// Vai ao próximo marcador depois do cursor (`forward`) ou ao anterior, dando a volta.
    pub fn go_to_next_bookmark(&mut self, forward: bool) -> bool {
        if self.bookmarks.is_empty() {
            return false;
        }
        let line = self.caret_line() - 1;
        let lines: Vec<usize> = self.bookmarks.iter().map(|m| m.line).collect();
        let target = if forward {
            lines.iter().copied().find(|&l| l > line).unwrap_or(lines[0])
        } else {
            lines.iter().rev().copied().find(|&l| l < line).unwrap_or(lines[lines.len() - 1])
        };
        self.go_to_line(target);
        true
    }

    /// Depois de uma edição: os marcadores abaixo do trecho mudado andam junto com as linhas
    /// (as do meio ficam no começo dele). Compara o texto de antes linha a linha, pelas pontas.
    pub(super) fn shift_bookmarks(&mut self) {
        // As edições daqui já andaram com eles (veja `Memo::splice_raw`).
        if self.bookmarks.is_empty() || self.marks_key.get() == self.text_key() {
            return;
        }
        let old: Vec<&str> = self.marks_text.split('\n').collect();
        let new: Vec<&str> = self.text.split('\n').collect();
        if old.len() != new.len() {
            let prefix = old.iter().zip(&new).take_while(|(a, b)| a == b).count();
            let max_suffix = old.len().min(new.len()) - prefix;
            let suffix = old.iter().rev().zip(new.iter().rev()).take(max_suffix).take_while(|(a, b)| a == b).count();
            let (old_end, new_end, lines) = (old.len() - suffix, new.len() - suffix, new.len());
            drop((old, new));
            self.move_marks(prefix, old_end, new_end, lines);
        }
        self.sync_marks();
    }

    /// As linhas `[prefix, old_end)` viraram `[prefix, new_end)` (o texto tem `lines` linhas):
    /// os marcadores de depois andam junto; os do meio ficam no trecho.
    pub(super) fn move_marks(&mut self, prefix: usize, old_end: usize, new_end: usize, lines: usize) {
        if old_end == new_end {
            return;
        }
        let delta = new_end as isize - old_end as isize;
        let last = lines.saturating_sub(1);
        for m in &mut self.bookmarks {
            if m.line >= old_end {
                m.line = (m.line as isize + delta).max(0) as usize;
            } else if m.line > prefix {
                m.line = m.line.min(prefix + new_end.saturating_sub(prefix + 1));
            }
            m.line = m.line.min(last);
        }
        self.bookmarks.dedup_by(|a, b| a.line == b.line && a.number == b.number);
    }

    /// Os marcadores valem para o texto de agora (guarda uma cópia, para acompanhar uma troca
    /// feita por fora).
    fn sync_marks(&mut self) {
        self.marks_text = self.text.clone();
        self.marks_key.set(self.text_key());
    }

    /// As teclas dos marcadores; `true` se tratou.
    pub(super) fn marks_key(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) -> bool {
        let m = event.modifiers;
        let ctrl = m.ctrl || m.meta;
        let Key::Char(c) = event.key else { return false };
        let changed = match (ctrl, m.shift, m.alt, digit_of(c)) {
            (true, true, false, Some(n)) => {
                self.toggle_numbered_bookmark(n);
                true
            }
            (true, false, false, Some(n)) => {
                if self.go_to_bookmark(n) {
                    self.activity(ctx);
                }
                ctx.set_handled();
                return true;
            }
            _ if ctrl && m.alt && !m.shift && c == 'k' => {
                self.toggle_bookmark();
                true
            }
            _ if ctrl && !m.alt && !m.shift && c.eq_ignore_ascii_case(&'b') => {
                self.toggle_next_bookmark();
                true
            }
            _ if ctrl && m.alt && !m.shift && (c == 'l' || c == 'j') => {
                if self.go_to_next_bookmark(c == 'l') {
                    self.activity(ctx);
                }
                ctx.set_handled();
                return true;
            }
            _ => return false,
        };
        if changed {
            ctx.fire("on_bookmarks_change", EventArgs::None);
            ctx.repaint();
        }
        ctx.set_handled();
        true
    }

    /// O marcador da linha `line` na calha: a etiqueta com o número (ou vazia), à esquerda.
    pub(super) fn paint_bookmark(&self, canvas: &mut dyn Canvas, line: usize, x: f32, y: f32, lh: f32, style: &TextStyle, color: aylis_core::Color) {
        let Some(mark) = self.bookmarks.iter().find(|m| m.line == line) else { return };
        let h = (lh * 0.72).min(16.0);
        let tag = RectF::new(x + 2.0, y + (lh - h) / 2.0, h * 0.9, h);
        canvas.fill_rect(tag, 2.0, &Brush::Solid(color));
        if let Some(n) = mark.number {
            let mut s = style.clone();
            s.font.size = (h * 0.75).max(8.0);
            s.color = aylis_core::Color::WHITE;
            s.h_align = TextAlign::Center;
            s.v_align = TextAlign::Center;
            canvas.draw_text(&n.to_string(), tag, &s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memo(text: &str) -> Memo {
        let mut m = Memo::default();
        m.text = text.to_string();
        m
    }

    #[test]
    fn numbered_marks_move_and_toggle() {
        let mut m = memo("a\nb\nc\nd");
        m.go_to_line(1);
        m.toggle_numbered_bookmark(3);
        m.go_to_line(2);
        m.toggle_numbered_bookmark(3);
        assert_eq!(m.bookmarks(), &[Bookmark { line: 2, number: Some(3) }]);
        m.toggle_numbered_bookmark(3);
        assert!(m.bookmarks().is_empty());
        // Uma linha, um marcador: o último número dado a ela.
        m.toggle_numbered_bookmark(1);
        m.toggle_numbered_bookmark(2);
        assert_eq!(m.bookmarks(), &[Bookmark { line: 2, number: Some(2) }]);
        assert_eq!(digit_of('!'), Some(1));
        assert_eq!(digit_of(')'), Some(0));
    }

    #[test]
    fn ctrl_b_takes_the_next_free_number_or_removes() {
        let mut m = memo(&(0..12).map(|i| i.to_string()).collect::<Vec<_>>().join("\n"));
        let numbers = |m: &Memo| m.bookmarks().iter().map(|b| (b.line, b.number)).collect::<Vec<_>>();
        m.go_to_line(2);
        m.toggle_next_bookmark();
        m.go_to_line(5);
        m.toggle_next_bookmark();
        assert_eq!(numbers(&m), vec![(2, Some(1)), (5, Some(2))], "começa pelo 1, depois o 2");
        // Em cima de um marcador: tira.
        m.go_to_line(2);
        m.toggle_next_bookmark();
        assert_eq!(numbers(&m), vec![(5, Some(2))]);
        // O 1 ficou livre: é o próximo.
        m.go_to_line(7);
        m.toggle_next_bookmark();
        assert_eq!(numbers(&m), vec![(5, Some(2)), (7, Some(1))]);
        // Depois do 9 vem o 0; com os dez em uso, um sem número.
        for line in [0, 1, 3, 4, 6, 8, 9, 10] {
            m.go_to_line(line);
            m.toggle_next_bookmark();
        }
        assert_eq!(m.bookmarks().iter().find(|b| b.line == 10).and_then(|b| b.number), Some(0));
        m.go_to_line(11);
        m.toggle_next_bookmark();
        assert_eq!(m.bookmarks().iter().find(|b| b.line == 11).map(|b| b.number), Some(None));
    }

    #[test]
    fn ctrl_b_mark_follows_the_next_edit() {
        // O texto mudou antes de existir marcador: o Ctrl+B tem de partir do texto de agora.
        let mut m = memo("a\nb");
        m.text = "a\nb\nc\nd".into();
        m.go_to_line(3);
        m.toggle_next_bookmark();
        m.text = "nova\na\nb\nc\nd".into();
        m.shift_bookmarks();
        assert_eq!(m.bookmarks(), &[Bookmark { line: 4, number: Some(1) }]);
    }

    #[test]
    fn marks_follow_inserted_and_deleted_lines() {
        let mut m = memo("a\nb\nc\nd");
        m.set_bookmarks(vec![Bookmark { line: 0, number: None }, Bookmark { line: 3, number: Some(1) }]);
        m.text = "a\nnova\noutra\nb\nc\nd".into();
        m.shift_bookmarks();
        assert_eq!(m.bookmarks().iter().map(|b| b.line).collect::<Vec<_>>(), vec![0, 5]);
        m.text = "a\nd".into();
        m.shift_bookmarks();
        assert_eq!(m.bookmarks().iter().map(|b| b.line).collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn next_and_previous_wrap_around() {
        let mut m = memo("0\n1\n2\n3\n4");
        m.set_bookmarks(vec![Bookmark { line: 1, number: None }, Bookmark { line: 3, number: None }]);
        m.go_to_line(2);
        m.go_to_next_bookmark(true);
        assert_eq!(m.caret_line(), 4);
        m.go_to_next_bookmark(true);
        assert_eq!(m.caret_line(), 2);
        m.go_to_next_bookmark(false);
        assert_eq!(m.caret_line(), 4);
    }
}
