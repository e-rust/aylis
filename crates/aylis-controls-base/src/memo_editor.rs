//! O que o [`CodeEditor`](crate::CodeEditor) liga no [`Memo`]: dobrar blocos (pelo recuo,
//! como o VS Code), vários cursores (Alt+clique, Ctrl+Alt+↑/↓, Ctrl+D, Ctrl+Shift+L) e o
//! minimapa à direita.

use aylis_core::{Brush, Canvas, EventCtx, Key, KeyEvent, PaintCtx, PointF, RectF, Stroke};

use super::{Memo, Row};

/// A largura do minimapa (com a barra de rolagem na beirada direita dele).
pub(super) const MINIMAP_WIDTH: f32 = 84.0;
/// A coluna das setas de dobrar, na calha.
pub(super) const FOLD_WIDTH: f32 = 14.0;

/// O recuo de uma linha (tab vale 4); `None` se ela está em branco.
fn indent_of(line: &str) -> Option<usize> {
    if line.trim().is_empty() {
        return None;
    }
    Some(line.chars().take_while(|c| *c == ' ' || *c == '\t').map(|c| if c == '\t' { 4 } else { 1 }).sum())
}

/// A linha abre (`Some(true)`) ou fecha (`Some(false)`) uma region, na convenção de cada
/// linguagem: `#region` (C#), `// #region` (Rust, Go, Java, C++, TypeScript), `# region`
/// (Python) e `{$REGION}` (Free Pascal), com o `end` correspondente.
pub fn region_mark(line: &str) -> Option<bool> {
    let t = line.trim_start();
    // A palavra (`region`, `endregion`) e o que vem depois dela: fim, espaço, aspa ou `}`.
    let word = |rest: &str| -> Option<bool> {
        let lower = rest.to_ascii_lowercase();
        let (open, len) = if lower.starts_with("endregion") { (false, 9) } else if lower.starts_with("region") { (true, 6) } else { return None };
        lower[len..].chars().next().is_none_or(|c| c.is_whitespace() || matches!(c, '}' | '\x27' | '"')).then_some(open)
    };
    if let Some(rest) = t.strip_prefix("{$") {
        return word(rest);
    }
    if let Some(rest) = t.strip_prefix("//") {
        return word(rest.trim_start().strip_prefix('#')?);
    }
    if let Some(rest) = t.strip_prefix('#') {
        // `#region` (C#) ou `# region` (Python).
        return word(rest.trim_start());
    }
    // As regions das linguagens importadas do VS Code.
    crate::textmate::region_mark(line)
}

/// A última linha do bloco que a linha `line` abre (as de baixo com mais recuo), se ela
/// abre um. A linha que fecha (`}`, `end`) fica de fora: continua à vista, como no VS Code.
pub fn fold_end(lines: &[&str], line: usize) -> Option<usize> {
    // Region: até o fim dela (com a linha do fim), contando as de dentro.
    if region_mark(lines.get(line)?) == Some(true) {
        let mut depth = 0;
        for (j, l) in lines.iter().enumerate().skip(line + 1) {
            match region_mark(l) {
                Some(true) => depth += 1,
                Some(false) if depth == 0 => return Some(j),
                Some(false) => depth -= 1,
                None => {}
            }
        }
        return None;
    }
    let base = indent_of(lines.get(line)?)?;
    let mut end = None;
    for (j, l) in lines.iter().enumerate().skip(line + 1) {
        match indent_of(l) {
            None => continue,
            Some(i) if i > base => end = Some(j),
            Some(_) => break,
        }
    }
    end
}

impl Memo {
    // -----------------------------------------------------------------------
    // Dobrar

    /// As linhas escondidas (uma posição por linha do texto; vazio: nada dobrado). Acerta as
    /// dobras antes: a que perdeu a linha (o texto acima mudou) é achada pelo texto dela
    /// por perto; a que não abre mais um bloco sai.
    pub(super) fn hidden_lines(&self, lines: &[&str]) -> Vec<bool> {
        let mut folds = self.folds.borrow_mut();
        if folds.is_empty() {
            return Vec::new();
        }
        folds.retain_mut(|(line, text)| {
            if lines.get(*line).is_none_or(|l| l.trim() != text) {
                let near = (0..400).flat_map(|d| [line.checked_sub(d), Some(*line + d)]).flatten().find(|&j| lines.get(j).is_some_and(|l| l.trim() == text));
                match near {
                    Some(j) => *line = j,
                    None => return false,
                }
            }
            fold_end(lines, *line).is_some()
        });
        folds.sort();
        folds.dedup_by_key(|(l, _)| *l);
        let mut hidden = vec![false; lines.len()];
        for (line, _) in folds.iter() {
            if let Some(end) = fold_end(lines, *line) {
                for h in hidden.iter_mut().take(end + 1).skip(line + 1) {
                    *h = true;
                }
            }
        }
        hidden
    }

    /// Acerta as dobras com o texto de agora (a linha de cada uma pode ter andado).
    fn sync_folds(&self) {
        let lines = self.lines();
        let _ = self.hidden_lines(&lines);
    }

    /// As linhas dobradas (a partir de 0), já acertadas com o texto de agora.
    pub fn folded_lines(&self) -> Vec<usize> {
        self.sync_folds();
        self.folds.borrow().iter().map(|(l, _)| *l).collect()
    }

    /// A linha (a partir de 0) abre um bloco que dá para dobrar?
    pub fn can_fold(&self, line: usize) -> bool {
        let lines = self.lines();
        fold_end(&lines, line).is_some()
    }

    /// Dobra ou desdobra o bloco que a linha abre. `false`: ela não abre um bloco.
    pub fn toggle_fold(&mut self, line: usize) -> bool {
        self.sync_folds();
        if self.folds.borrow().iter().any(|(l, _)| *l == line) {
            self.folds.borrow_mut().retain(|(l, _)| *l != line);
            return true;
        }
        self.fold(line)
    }

    /// Dobra o bloco que a linha abre; o cursor que estava dentro vai para o fim dela.
    pub fn fold(&mut self, line: usize) -> bool {
        let lines = self.lines();
        let Some(end) = fold_end(&lines, line) else { return false };
        let text = lines[line].trim().to_string();
        let start: usize = lines.iter().take(line).map(|l| l.chars().count() + 1).sum();
        let line_end = start + lines[line].chars().count();
        let block_end: usize = lines.iter().take(end + 1).map(|l| l.chars().count() + 1).sum::<usize>() - 1;
        drop(lines);
        let inside = |i: usize| i > line_end && i <= block_end;
        if inside(self.caret) || inside(self.anchor) {
            self.caret = line_end;
            self.anchor = line_end;
        }
        self.extra.retain(|&(a, c)| !inside(a) && !inside(c));
        let mut folds = self.folds.borrow_mut();
        if !folds.iter().any(|(l, _)| *l == line) {
            folds.push((line, text));
        }
        true
    }

    /// Dobra o bloco mais de dentro onde o cursor está (ou o que a linha dele abre).
    pub fn fold_at_caret(&mut self) -> bool {
        self.sync_folds();
        let (caret_line, _) = self.caret_position();
        let lines = self.lines();
        let target = (0..=caret_line).rev().find(|&k| !self.folds.borrow().iter().any(|(l, _)| *l == k) && fold_end(&lines, k).is_some_and(|e| k == caret_line || e >= caret_line));
        drop(lines);
        target.is_some_and(|k| self.fold(k))
    }

    /// Desdobra o bloco da linha do cursor (o dobrado nela ou o que a contém).
    pub fn unfold_at_caret(&mut self) -> bool {
        self.sync_folds();
        let (caret_line, _) = self.caret_position();
        let before = self.folds.borrow().len();
        self.folds.borrow_mut().retain(|(l, _)| *l != caret_line);
        before != self.folds.borrow().len()
    }

    /// Dobra todos os blocos de fora (os que não estão dentro de outro).
    pub fn fold_all(&mut self) {
        let lines = self.lines();
        let mut line = 0;
        let mut list = Vec::new();
        while line < lines.len() {
            match fold_end(&lines, line) {
                Some(end) => {
                    list.push(line);
                    line = end + 1;
                }
                None => line += 1,
            }
        }
        drop(lines);
        for l in list {
            self.fold(l);
        }
    }

    pub fn unfold_all(&mut self) {
        self.folds.borrow_mut().clear();
    }

    /// Dobra os blocos do nível `level` (1: os de fora, como as funções de um arquivo; 2: os
    /// de dentro deles...) e desdobra o resto. As regions contam como nível 1.
    pub fn fold_level(&mut self, level: usize) {
        self.unfold_all();
        let lines = self.lines();
        let mut open: Vec<usize> = Vec::new();
        let mut list = Vec::new();
        for line in 0..lines.len() {
            open.retain(|&end| end >= line);
            if let Some(end) = fold_end(&lines, line) {
                if open.len() + 1 == level.max(1) {
                    list.push(line);
                }
                open.push(end);
            }
        }
        drop(lines);
        for l in list {
            self.fold(l);
        }
    }

    /// Desdobra o que esconde a posição (o cursor foi para dentro de um bloco dobrado: busca,
    /// ir para a linha, o depurador).
    pub(super) fn unfold_at(&mut self, index: usize) {
        if self.folds.borrow().is_empty() {
            return;
        }
        self.sync_folds();
        let lines = self.lines();
        let line = self.text.chars().take(index).filter(|c| *c == '\n').count();
        let hiding: Vec<usize> = self.folds.borrow().iter().filter(|(l, _)| fold_end(&lines, *l).is_some_and(|e| line > *l && line <= e)).map(|(l, _)| *l).collect();
        drop(lines);
        if !hiding.is_empty() {
            self.folds.borrow_mut().retain(|(l, _)| !hiding.contains(l));
        }
    }

    /// Os cursores todos ficam à vista (nenhum dentro de um bloco dobrado).
    pub(super) fn reveal_cursors(&mut self) {
        let mut at: Vec<usize> = vec![self.caret];
        at.extend(self.extra.iter().map(|&(_, c)| c));
        for i in at {
            self.unfold_at(i);
        }
    }

    /// A seta da dobra na calha: aberta (para baixo) ou dobrada (para o lado).
    pub(super) fn paint_fold_mark(canvas: &mut dyn Canvas, x: f32, y: f32, lh: f32, folded: bool, color: aylis_core::Color) {
        let c = PointF::new(x + FOLD_WIDTH / 2.0, y + lh / 2.0);
        let s = Stroke::new(color, 1.4);
        if folded {
            canvas.draw_line(PointF::new(c.x - 2.0, c.y - 4.0), PointF::new(c.x + 2.0, c.y), &s);
            canvas.draw_line(PointF::new(c.x + 2.0, c.y), PointF::new(c.x - 2.0, c.y + 4.0), &s);
        } else {
            canvas.draw_line(PointF::new(c.x - 4.0, c.y - 2.0), PointF::new(c.x, c.y + 2.0), &s);
            canvas.draw_line(PointF::new(c.x, c.y + 2.0), PointF::new(c.x + 4.0, c.y - 2.0), &s);
        }
    }

    // -----------------------------------------------------------------------
    // Vários cursores

    /// Todos os cursores (âncora, cursor), o principal primeiro.
    pub fn cursors(&self) -> Vec<(usize, usize)> {
        let mut all = vec![(self.anchor, self.caret)];
        all.extend(self.extra.iter().copied());
        all
    }

    /// Mais um cursor (com seleção, se `anchor != caret`); ele vira o principal, como no
    /// Alt+clique do VS Code.
    pub fn add_cursor(&mut self, anchor: usize, caret: usize) {
        let len = self.text.chars().count();
        let (anchor, caret) = (anchor.min(len), caret.min(len));
        if (anchor, caret) == (self.anchor, self.caret) || self.extra.contains(&(anchor, caret)) {
            return;
        }
        self.extra.push((self.anchor, self.caret));
        self.anchor = anchor;
        self.caret = caret;
    }

    /// Volta a um cursor só.
    pub fn clear_cursors(&mut self) {
        self.extra.clear();
    }

    pub fn has_extra_cursors(&self) -> bool {
        !self.extra.is_empty()
    }

    /// Junta os cursores que caíram no mesmo lugar (ou com as seleções se cruzando).
    fn merge_cursors(&mut self) {
        let primary = (self.anchor, self.caret);
        let mut all = self.cursors();
        all.sort_by_key(|&(a, c)| a.min(c));
        let mut merged: Vec<(usize, usize)> = Vec::new();
        for cur in all {
            match merged.last_mut() {
                Some(last) if cur.0.min(cur.1) < last.0.max(last.1) || cur.0.min(cur.1) == last.0.min(last.1) => {
                    let (a, b) = (last.0.min(last.1).min(cur.0.min(cur.1)), last.0.max(last.1).max(cur.0.max(cur.1)));
                    *last = if last.1 >= last.0 { (a, b) } else { (b, a) };
                }
                _ => merged.push(cur),
            }
        }
        let main = merged.iter().position(|&c| c == primary || (c.0.min(c.1) <= primary.1 && primary.1 <= c.0.max(c.1))).unwrap_or(0);
        let (a, c) = merged.remove(main);
        self.anchor = a;
        self.caret = c;
        self.extra = merged;
    }

    /// A mesma mudança em todos os cursores: `f` age no principal (`self.anchor`,
    /// `self.caret`) e cada cursor passa por ele, do último para o primeiro (assim a mudança
    /// de um não mexe na posição dos de antes); no fim, cada um anda o que os de antes
    /// acrescentaram ou tiraram.
    pub(super) fn edit_all(&mut self, mut f: impl FnMut(&mut Memo)) {
        let primary = (self.anchor, self.caret);
        let mut all = self.cursors();
        all.sort_by_key(|&(a, c)| a.min(c));
        let mut results = vec![(0usize, 0usize, 0isize); all.len()];
        for (i, &(a, c)) in all.iter().enumerate().rev() {
            self.anchor = a;
            self.caret = c;
            let before = self.text.chars().count() as isize;
            f(self);
            let delta = self.text.chars().count() as isize - before;
            results[i] = (self.anchor, self.caret, delta);
        }
        let mut shift = 0isize;
        let mut moved = Vec::with_capacity(all.len());
        for &(a, c, delta) in &results {
            moved.push(((a as isize + shift).max(0) as usize, (c as isize + shift).max(0) as usize));
            shift += delta;
        }
        let main = all.iter().position(|&x| x == primary).unwrap_or(0);
        let (a, c) = moved.remove(main);
        self.anchor = a;
        self.caret = c;
        self.extra = moved;
        self.merge_cursors();
    }

    /// Anda com todos os cursores (setas, Home, End), cada um pela regra do principal.
    pub(super) fn move_all(&mut self, mut f: impl FnMut(&mut Memo)) {
        let primary = (self.anchor, self.caret);
        let all = self.cursors();
        let mut moved = Vec::with_capacity(all.len());
        for &(a, c) in &all {
            self.anchor = a;
            self.caret = c;
            f(self);
            moved.push((self.anchor, self.caret));
        }
        let main = all.iter().position(|&x| x == primary).unwrap_or(0);
        let (a, c) = moved.remove(main);
        self.anchor = a;
        self.caret = c;
        self.extra = moved;
        self.merge_cursors();
    }

    /// Um cursor a mais na linha de cima ou de baixo, na mesma coluna (Ctrl+Alt+↑/↓).
    pub fn add_cursor_vertical(&mut self, down: bool) -> bool {
        let lines = self.lines();
        // Parte do cursor mais acima (subindo) ou mais abaixo (descendo).
        let pick = self.cursors().into_iter().map(|(_, c)| c).fold(None, |best: Option<usize>, c| match best {
            Some(b) if (down && b >= c) || (!down && b <= c) => Some(b),
            _ => Some(c),
        });
        let Some(from) = pick else { return false };
        let line = self.text.chars().take(from).filter(|ch| *ch == '\n').count();
        let col = from - lines.iter().take(line).map(|l| l.chars().count() + 1).sum::<usize>();
        // A próxima linha à vista (as de dentro de um bloco dobrado são puladas).
        let hidden = self.hidden_lines(&lines);
        let mut target = line;
        loop {
            target = if down { target + 1 } else { target.wrapping_sub(1) };
            if !hidden.get(target).copied().unwrap_or(false) {
                break;
            }
        }
        let Some(text) = lines.get(target) else { return false };
        let index = lines.iter().take(target).map(|l| l.chars().count() + 1).sum::<usize>() + col.min(text.chars().count());
        drop(lines);
        self.add_cursor(index, index);
        true
    }

    /// Ctrl+D com seleção: mais um cursor na próxima ocorrência (a seleção de agora fica).
    pub fn add_next_occurrence(&mut self) -> bool {
        if !self.has_selection() {
            return self.select_next_occurrence();
        }
        let current = (self.anchor, self.caret);
        let taken = self.cursors();
        if !self.select_next_occurrence() || taken.contains(&(self.anchor, self.caret)) {
            (self.anchor, self.caret) = current;
            return false;
        }
        let next = (self.anchor, self.caret);
        (self.anchor, self.caret) = current;
        self.add_cursor(next.0, next.1);
        true
    }

    /// Ctrl+Shift+L: um cursor em cada ocorrência da seleção (ou da palavra do cursor).
    pub fn select_all_occurrences(&mut self) -> bool {
        if !self.has_selection() && !self.select_next_occurrence() {
            return false;
        }
        let query: Vec<char> = self.selected_text().chars().collect();
        if query.is_empty() {
            return false;
        }
        let chars: Vec<char> = self.text.chars().collect();
        let mut found = Vec::new();
        let mut i = 0;
        while i + query.len() <= chars.len() {
            if chars[i..i + query.len()] == query[..] {
                found.push(i);
                i += query.len();
            } else {
                i += 1;
            }
        }
        let current = self.selection().0;
        for start in found {
            if start != current {
                let end = start + query.len();
                self.extra.push((start, end));
            }
        }
        self.merge_cursors();
        true
    }

    /// O texto de todas as seleções, uma por linha (o Ctrl+C com vários cursores).
    pub(super) fn all_selected_text(&self) -> String {
        let mut all = self.cursors();
        all.sort_by_key(|&(a, c)| a.min(c));
        all.iter()
            .filter(|(a, c)| a != c)
            .map(|&(a, c)| {
                let (a, b) = (a.min(c), a.max(c));
                self.text[self.byte_at(a)..self.byte_at(b)].to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// As teclas com vários cursores: digitar, apagar, Enter, as setas, Home, End, copiar,
    /// recortar, colar e o Esc (volta a um cursor). `true`: tratou.
    pub(super) fn multi_key(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) -> bool {
        if self.extra.is_empty() {
            return false;
        }
        let shift = event.modifiers.shift;
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        let now = ctx.ui.time();
        let edit = |m: &mut Memo, kind| {
            if !m.read_only {
                m.begin_edit(kind, now);
            }
        };
        match event.key {
            Key::Escape => self.extra.clear(),
            Key::Backspace if !self.read_only => {
                edit(self, super::code::EditKind::Deleting);
                self.edit_all(|m| {
                    if m.has_selection() {
                        m.delete_selection();
                    } else if m.caret > 0 {
                        m.delete_range(m.caret - 1, m.caret);
                    }
                });
                self.end_edit();
                self.changed(ctx);
            }
            Key::Delete if !self.read_only => {
                edit(self, super::code::EditKind::Deleting);
                let len = self.text.chars().count();
                self.edit_all(|m| {
                    if m.has_selection() {
                        m.delete_selection();
                    } else if m.caret < len.min(m.text.chars().count()) {
                        m.delete_range(m.caret, m.caret + 1);
                    }
                });
                self.end_edit();
                self.changed(ctx);
            }
            Key::Enter if !self.read_only && !ctrl => {
                edit(self, super::code::EditKind::Typing);
                self.edit_all(|m| {
                    let line = m.newline_with_indent();
                    m.insert(&line);
                });
                self.end_edit();
                self.changed(ctx);
            }
            Key::Left | Key::Right if !ctrl => {
                let right = event.key == Key::Right;
                self.move_all(|m| {
                    let (a, b) = m.selection();
                    let to = if m.has_selection() && !shift {
                        if right { b } else { a }
                    } else if right {
                        (m.caret + 1).min(m.text.chars().count())
                    } else {
                        m.caret.saturating_sub(1)
                    };
                    m.move_to(to, shift);
                });
            }
            Key::Home | Key::End if !ctrl => {
                let end = event.key == Key::End;
                self.move_all(|m| {
                    if end {
                        let rest = m.text.chars().skip(m.caret).take_while(|c| *c != '\n').count();
                        m.move_to(m.caret + rest, shift);
                    } else {
                        m.smart_home(shift);
                    }
                });
            }
            Key::Up | Key::Down if !ctrl && !event.modifiers.alt => {
                let lines = if event.key == Key::Up { -1 } else { 1 };
                self.move_all(|m| m.vertical_lines(lines, shift));
            }
            Key::Char('c') | Key::Char('x') if ctrl => {
                let text = self.all_selected_text();
                if !text.is_empty() {
                    if let Some(c) = ctx.ui.services_mut().clipboard.as_mut() {
                        c.set_text(&text);
                    }
                    if event.key == Key::Char('x') && !self.read_only {
                        edit(self, super::code::EditKind::Other);
                        self.edit_all(|m| {
                            m.delete_selection();
                        });
                        self.end_edit();
                        self.changed(ctx);
                    }
                }
            }
            Key::Char('v') if ctrl && !self.read_only => {
                let Some(pasted) = ctx.ui.services_mut().clipboard.as_mut().and_then(|c| c.get_text()) else { return true };
                // Uma linha para cada cursor, se o número bate (como no VS Code).
                let parts: Vec<String> = pasted.replace("\r\n", "\n").split('\n').map(str::to_string).collect();
                let spread = parts.len() == self.extra.len() + 1;
                edit(self, super::code::EditKind::Other);
                let mut order = 0usize;
                let total = self.extra.len() + 1;
                self.edit_all(|m| {
                    // `edit_all` vai do último cursor para o primeiro.
                    let piece = if spread { parts[total - 1 - order].clone() } else { pasted.clone() };
                    order += 1;
                    m.delete_selection();
                    m.insert(&piece);
                });
                self.end_edit();
                self.changed(ctx);
            }
            Key::Char('a') if ctrl => {
                self.extra.clear();
                return false;
            }
            Key::Tab | Key::PageUp | Key::PageDown => {
                self.extra.clear();
                return false;
            }
            _ => return false,
        }
        ctx.set_handled();
        self.activity(ctx);
        true
    }

    /// Digitar com vários cursores: o texto entra em cada um.
    pub(super) fn multi_text(&mut self, ctx: &mut EventCtx<'_>, text: &str) -> bool {
        if self.extra.is_empty() || self.read_only {
            return false;
        }
        self.begin_edit(super::code::EditKind::Typing, ctx.ui.time());
        self.edit_all(|m| {
            m.delete_selection();
            m.insert(text);
        });
        self.end_edit();
        self.changed(ctx);
        ctx.set_handled();
        self.activity(ctx);
        true
    }

    /// Sobe ou desce linhas do texto mantendo a coluna (com vários cursores, sem medir:
    /// cada cursor anda pela sua coluna).
    fn vertical_lines(&mut self, lines: isize, extend: bool) {
        // Pela tabela das linhas: não percorre o texto (num fonte enorme, a cada seta).
        let table = self.lines_of(self.text_key());
        let line = self.line_of(self.caret);
        let col = self.caret - table[line].0;
        let target = (line as isize + lines).clamp(0, table.len() as isize - 1) as usize;
        let length = table.get(target + 1).map_or_else(|| self.len(), |next| next.0 - 1) - table[target].0;
        let index = table[target].0 + col.min(length);
        self.move_to(index, extend);
    }

    // -----------------------------------------------------------------------
    // Minimapa

    /// O minimapa: cada linha vira traços da cor do realce (um pixel por caractere), com a
    /// parte à vista marcada. `area` é a faixa dele; `rows` as linhas visuais.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn paint_minimap(
        &self,
        canvas: &mut dyn Canvas,
        ctx: &PaintCtx<'_>,
        area: RectF,
        rows: &[Row],
        spans: Option<&[Vec<super::Span>]>,
        lh: f32,
        scroll_y: f32,
        view_h: f32,
        text_key: u64,
    ) {
        let dark = ctx.theme.dark;
        let text = crate::syntax::code_theme(&self.color_theme).filter(|_| self.code_editing).map_or_else(|| ctx.theme.color("text"), |t| t.foreground());
        canvas.fill_rect(area, 0.0, &Brush::Solid(text.with_opacity(0.025)));
        if rows.is_empty() {
            return;
        }
        // Dois pixels por linha; num arquivo comprido, a escala diminui para o arquivo inteiro
        // caber (com um piso, as linhas do fim ficavam fora da faixa e o arrasto até o fim
        // do minimapa não chegava ao fim do fonte).
        let scale = (area.height / rows.len() as f32).min(2.0);
        self.minimap_rect.set(Some((area, scale)));
        let first_visible = (scroll_y / lh).floor().max(0.0) as usize;
        let shown = (view_h / lh).ceil() as usize;
        let view = RectF::new(area.x, area.y + first_visible as f32 * scale, area.width, (shown as f32 * scale).max(4.0));
        canvas.fill_rect(view, 0.0, &Brush::Solid(text.with_opacity(0.10)));
        // Os traços dependem só do texto, da faixa e das cores: calculados uma vez (e, no
        // Direct2D, guardados como imagem).
        let key = {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            (text_key, area.width.to_bits(), area.height.to_bits(), scale.to_bits(), rows.len(), dark, &self.color_theme, self.code_editing).hash(&mut h);
            text.to_hex().hash(&mut h);
            h.finish()
        };
        // Num arquivo grande o realce vem aos poucos (alguns ms por quadro) com o mesmo texto:
        // enquanto não termina, os traços são refeitos a cada quadro e não ficam guardados
        // (antes ficavam os do primeiro quadro, só com o começo colorido).
        let pending = self.highlight_pending.get();
        let strokes = {
            let cached = self.minimap_strokes.borrow();
            (cached.0 == key && !pending).then(|| cached.1.clone())
        };
        let strokes = match strokes {
            Some(s) => s,
            None => {
                let s = std::rc::Rc::new(self.minimap_strokes_of(ctx, area, rows, spans, scale, text));
                if !pending {
                    *self.minimap_strokes.borrow_mut() = (key, s.clone());
                }
                s
            }
        };
        if pending || !canvas.begin_cached(key, area) {
            canvas.save();
            canvas.clip_rect(area);
            for (r, color) in strokes.iter() {
                canvas.fill_rect(RectF::new(area.x + r.x, area.y + r.y, r.width, r.height), 0.0, &Brush::Solid(*color));
            }
            canvas.restore();
            if !pending {
                canvas.end_cached();
            }
        }
    }

    /// Os traços do minimapa (relativos ao canto da faixa) e as cores.
    fn minimap_strokes_of(&self, ctx: &PaintCtx<'_>, area: RectF, rows: &[Row], spans: Option<&[Vec<super::Span>]>, scale: f32, text: aylis_core::Color) -> Vec<(RectF, aylis_core::Color)> {
        let dark = ctx.theme.dark;
        let mut out = Vec::new();
        let cw = 1.0f32;
        let max_cols = ((area.width - 8.0) / cw) as usize;
        // Uma linha a cada `step` quando a escala fica abaixo de um pixel.
        let step = if scale < 1.0 { (1.0 / scale).ceil() as usize } else { 1 };
        for (i, row) in rows.iter().enumerate().step_by(step) {
            let y = i as f32 * scale;
            if y > area.height {
                break;
            }
            // Só a linha (pela tabela das linhas) e só o que cabe na faixa: copiar o texto
            // inteiro a cada vez pesava num fonte enorme.
            let line: Vec<char> = self.text[self.byte_at(row.start)..self.byte_at(row.end)].chars().take(max_cols + 1).collect();
            let line = &line[..];
            let h = scale.max(1.0) * 0.8;
            let mut paint_run = |a: usize, b: usize, color: aylis_core::Color| {
                // Só o que não é espaço vira traço.
                let mut j = a;
                while j < b && j < max_cols {
                    if line.get(j).is_some_and(|c| !c.is_whitespace()) {
                        let k = (j..b.min(max_cols)).find(|&k| line.get(k).is_none_or(|c| c.is_whitespace())).unwrap_or(b.min(max_cols));
                        out.push((RectF::new(4.0 + j as f32 * cw, y, (k - j) as f32 * cw, h), color.with_opacity(0.65)));
                        j = k;
                    } else {
                        j += 1;
                    }
                }
            };
            match spans.and_then(|s| s.get(row.line)).filter(|s| !s.is_empty()) {
                Some(list) => {
                    let mut last = 0;
                    for span in list {
                        if span.start > last {
                            paint_run(last, span.start, text);
                        }
                        let color = crate::syntax::span_color(crate::syntax::code_theme(&self.color_theme).filter(|_| self.code_editing), span, dark, text);
                        paint_run(span.start, span.end, color);
                        last = span.end;
                    }
                    if last < line.len() {
                        paint_run(last, line.len(), text);
                    }
                }
                None => paint_run(0, line.len(), text),
            }
        }
        out
    }

    /// O ponto do minimapa vira a rolagem: a linha dele fica no meio da vista.
    pub(super) fn minimap_scroll(&mut self, p: PointF) -> bool {
        let Some((area, scale)) = self.minimap_rect.get() else { return false };
        let lh = self.line_height.get().max(1.0);
        let row = ((p.y - area.y) / scale).max(0.0);
        let view_h = self.text_height.get();
        let mut s = self.scroll.get();
        // Até o fim do fonte (a última linha embaixo da vista), não além.
        let rows = (area.height / scale).ceil();
        let max = self.scroll_max(rows * lh, view_h, lh);
        s.y = (row * lh - view_h / 2.0).clamp(0.0, max.max(0.0));
        self.scroll.set(s);
        true
    }

    pub(super) fn in_minimap(&self, p: PointF) -> bool {
        self.minimap_rect.get().is_some_and(|(area, _)| area.contains(p))
    }
}

#[cfg(test)]
mod region_tests {
    use super::*;

    #[test]
    fn region_marks_follow_each_language() {
        for open in ["#region Cálculos", "// #region Cálculos", "  //#region", "# region Cálculos", "{$REGION 'Cálculos'}"] {
            assert_eq!(region_mark(open), Some(true), "{open}");
        }
        for close in ["#endregion", "// #endregion", "# endregion", "{$ENDREGION}"] {
            assert_eq!(region_mark(close), Some(false), "{close}");
        }
        for other in ["// region of interest", "# regional", "let region = 1;", "#regions"] {
            assert_eq!(region_mark(other), None, "{other}");
        }
    }

    #[test]
    fn a_region_folds_through_its_end_and_counts_nested_ones() {
        let text = ["// #region Fora", "fn a() {}", "// #region Dentro", "fn b() {}", "// #endregion", "// #endregion", "fn c() {}"];
        assert_eq!(fold_end(&text, 0), Some(5));
        assert_eq!(fold_end(&text, 2), Some(4));
    }

    #[test]
    fn fold_level_folds_only_that_depth() {
        let mut m = crate::Memo::default();
        m.text = "fn a() {\n    if x {\n        y();\n    }\n}\nfn b() {\n    z();\n}".into();
        m.fold_level(1);
        assert_eq!(m.folded_lines(), vec![0, 5]);
        m.fold_level(2);
        assert_eq!(m.folded_lines(), vec![1]);
    }
}
