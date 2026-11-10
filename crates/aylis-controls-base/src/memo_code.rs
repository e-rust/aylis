//! O Memo como editor de código (o que o VS Code tem): desfazer e refazer, localizar e
//! substituir, as operações de linha (mover, duplicar, apagar, comentar, recuar), os pares
//! que fecham sozinhos, o par do colchete e as outras ocorrências da palavra, Ctrl+D, e o
//! que vem do servidor de linguagem: a lista de sugestões, as dicas, a ajuda dos parâmetros
//! e os erros sublinhados.
//!
//! Submódulo do `memo`: mexe nos campos dele direto. Os eventos que a IDE escuta:
//! `on_complete_request` (pedir sugestões: o índice do cursor), `on_signature_request`
//! (abriu um parêntese), `on_hover_index` (a palavra sob o mouse; -1 fora).

use aylis_core::{Brush, Canvas, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, PointF, RectF, Stroke, TextAlign, TextStyle, Trimming};

use super::{Memo, expand_tabs};

/// Uma sugestão da lista (o autocompletar).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Suggestion {
    /// O que aparece na lista.
    pub label: String,
    /// O tipo ou a assinatura, à direita (apagado).
    pub detail: String,
    /// O que entra no texto (vazio: o `label`).
    pub insert: String,
    /// `function`, `variable`, `class`, `keyword`... (a letra à esquerda).
    pub kind: String,
}

/// Um erro, aviso ou dica no texto (sublinhado ondulado).
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    /// Trecho em caracteres.
    pub start: usize,
    pub end: usize,
    /// 1 erro, 2 aviso, 3 informação, 4 dica (como no LSP).
    pub severity: u8,
    pub message: String,
}

/// O texto e o cursor guardados para o desfazer.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Snapshot {
    text: String,
    caret: usize,
    anchor: usize,
}

/// Que tipo de mudança: digitações seguidas viram um passo só do desfazer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EditKind {
    Typing,
    Deleting,
    Other,
}

/// Quantas linhas a lista de sugestões mostra.
const LIST_ROWS: usize = 10;
/// Os pares que fecham sozinhos.
const PAIRS: [(char, char); 6] = [('(', ')'), ('[', ']'), ('{', '}'), ('"', '"'), ('\'', '\''), ('`', '`')];

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// O trecho de `new` que difere de `old` (em caracteres): o começo e o fim, sem o que os dois
/// têm igual na frente e atrás.
fn changed_region(old: &str, new: &str) -> (usize, usize) {
    let prefix = old.chars().zip(new.chars()).take_while(|(a, b)| a == b).count();
    let (old_len, new_len) = (old.chars().count(), new.chars().count());
    let max_suffix = old_len.min(new_len) - prefix;
    let suffix = old.chars().rev().zip(new.chars().rev()).take(max_suffix).take_while(|(a, b)| a == b).count();
    (prefix, new_len - suffix)
}

impl Memo {
    // ---------------------------------------------------------------- desfazer

    /// Guarda o estado antes de uma mudança. Digitações (ou apagamentos) seguidas, sem
    /// pausa, ficam num passo só, como no VS Code.
    pub(super) fn begin_edit(&mut self, kind: EditKind, now: f64) {
        let merge = kind != EditKind::Other && self.last_edit.is_some_and(|(k, t, at)| k == kind && now - t < 1.0 && at == self.caret);
        if !merge {
            self.undo_stack.push(Snapshot { text: self.text.clone(), caret: self.caret, anchor: self.anchor });
            if self.undo_stack.len() > 500 {
                self.undo_stack.remove(0);
            }
        }
        self.redo_stack.clear();
        self.last_edit = Some((kind, now, usize::MAX));
    }

    /// Depois da mudança: onde o cursor ficou (a próxima digitação junta se continuar daqui).
    pub(super) fn end_edit(&mut self) {
        if let Some((k, t, _)) = self.last_edit {
            self.last_edit = Some((k, t, self.caret));
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Ctrl+Z.
    pub fn undo(&mut self) -> bool {
        let Some(s) = self.undo_stack.pop() else { return false };
        let region = changed_region(&self.text, &s.text);
        self.redo_stack.push(Snapshot { text: std::mem::take(&mut self.text), caret: self.caret, anchor: self.anchor });
        self.restore(s, region);
        true
    }

    /// Ctrl+Y (ou Ctrl+Shift+Z).
    pub fn redo(&mut self) -> bool {
        let Some(s) = self.redo_stack.pop() else { return false };
        let region = changed_region(&self.text, &s.text);
        self.undo_stack.push(Snapshot { text: std::mem::take(&mut self.text), caret: self.caret, anchor: self.anchor });
        self.restore(s, region);
        true
    }

    /// Volta ao texto `s`; `region` é o trecho dele que muda (em caracteres). O cursor guardado
    /// vale se está nesse trecho; senão (uma troca feita de fora, como o `edit_code` do agente,
    /// com o cursor do usuário noutro lugar), ele vai ao fim do trecho e a rolagem vai junto:
    /// antes pulava para o lugar de uma edição antiga (conferência de 08/11).
    fn restore(&mut self, s: Snapshot, region: (usize, usize)) {
        self.text = s.text;
        self.text_changed();
        let (start, end) = region;
        let inside = |i: usize| i >= start && i <= end;
        if inside(s.caret) && inside(s.anchor) {
            self.caret = s.caret.min(self.len());
            self.anchor = s.anchor.min(self.len());
        } else {
            self.caret = end.min(self.len());
            self.anchor = self.caret;
        }
        self.last_edit = None;
        self.reveal.set(true);
        self.diagnostics.clear();
    }

    /// Esquece o desfazer (o texto foi trocado de fora).
    pub(super) fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_edit = None;
    }

    // ---------------------------------------------------------------- posições

    /// O índice (em caracteres) da linha `line` (a partir de 0) e da coluna `column` contada
    /// em unidades UTF-16, como o LSP conta. Além do fim, o fim.
    pub fn index_of_position(&self, line: usize, column_utf16: usize) -> usize {
        let mut index = 0;
        for (i, l) in self.text.split('\n').enumerate() {
            if i == line {
                let mut units = 0;
                for (n, c) in l.chars().enumerate() {
                    if units >= column_utf16 {
                        return index + n;
                    }
                    units += c.len_utf16();
                }
                return index + l.chars().count();
            }
            index += l.chars().count() + 1;
        }
        self.len()
    }

    /// A linha (a partir de 0) e a coluna em UTF-16 de um índice (em caracteres).
    pub fn position_of_index(&self, index: usize) -> (usize, usize) {
        let (mut line, mut col) = (0, 0);
        for (i, c) in self.text.chars().enumerate() {
            if i == index {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += c.len_utf16();
            }
        }
        (line, col)
    }

    /// O índice do cursor (em caracteres).
    pub fn caret_index(&self) -> usize {
        self.caret
    }

    /// Troca vários trechos de uma vez (o renomear, a formatação, uma correção), num passo
    /// só do desfazer. Os trechos em caracteres, sem se sobrepor.
    pub fn apply_edits(&mut self, mut edits: Vec<(usize, usize, String)>, now: f64) -> bool {
        if edits.is_empty() || self.read_only {
            return false;
        }
        self.begin_edit(EditKind::Other, now);
        edits.sort_by(|a, b| b.0.cmp(&a.0));
        let caret = self.caret;
        let mut shift: isize = 0;
        for (start, end, text) in edits {
            let (a, b) = (self.byte_at(start), self.byte_at(end.max(start)));
            self.text.replace_range(a..b, &text);
            self.text_changed();
            if start < caret {
                shift += text.chars().count() as isize - (end.max(start) - start) as isize;
            }
        }
        let caret = (caret as isize + shift).max(0) as usize;
        self.caret = caret.min(self.len());
        self.anchor = self.caret;
        self.end_edit();
        true
    }

    /// Troca o texto todo mantendo o cursor (o arquivo mudou fora), num passo do desfazer.
    pub fn replace_text(&mut self, text: &str, now: f64) {
        if text == self.text {
            return;
        }
        self.begin_edit(EditKind::Other, now);
        self.text = text.to_string();
        self.text_changed();
        self.caret = self.caret.min(self.len());
        self.anchor = self.anchor.min(self.len());
        self.end_edit();
    }

    // ---------------------------------------------------------------- localizar

    /// Onde `query` aparece (em caracteres), com ou sem diferenciar maiúsculas e só a palavra
    /// inteira.
    pub fn find_all(&self, query: &str, case: bool, whole: bool) -> Vec<(usize, usize)> {
        if query.is_empty() {
            return Vec::new();
        }
        let fold = |s: &str| if case { s.to_string() } else { s.to_lowercase() };
        let chars: Vec<char> = self.text.chars().collect();
        let hay: Vec<char> = fold(&self.text).chars().collect();
        let needle: Vec<char> = fold(query).chars().collect();
        // Com o minúsculo mudando o tamanho (raro), cai no exato.
        let (hay, needle) = if hay.len() == chars.len() { (hay, needle) } else { (chars.clone(), query.chars().collect()) };
        let mut out = Vec::new();
        let mut i = 0;
        while i + needle.len() <= hay.len() {
            if hay[i..i + needle.len()] == needle[..] {
                let end = i + needle.len();
                let ok = !whole || ((i == 0 || !is_word(chars[i - 1])) && (end == chars.len() || !is_word(chars[end])));
                if ok {
                    out.push((i, end));
                    i = end;
                    continue;
                }
            }
            i += 1;
        }
        out
    }

    /// Seleciona a próxima (ou a anterior) ocorrência a partir do cursor, dando a volta.
    pub fn find(&mut self, query: &str, case: bool, whole: bool, forward: bool) -> bool {
        let all = self.find_all(query, case, whole);
        if all.is_empty() {
            return false;
        }
        let (a, b) = self.selection();
        let pick = if forward {
            all.iter().find(|m| m.0 >= b && (m.0, m.1) != (a, b)).or(all.first())
        } else {
            all.iter().rev().find(|m| m.1 <= a).or(all.last())
        };
        let &(s, e) = pick.expect("há ocorrências");
        self.anchor = s;
        self.caret = e;
        self.reveal.set(true);
        true
    }

    /// Substituir: troca a seleção se ela é uma ocorrência e vai para a próxima.
    pub fn replace(&mut self, query: &str, replacement: &str, case: bool, whole: bool, now: f64) -> bool {
        let sel = self.selection();
        let hit = self.find_all(query, case, whole).contains(&sel) && sel.0 != sel.1;
        if hit {
            self.begin_edit(EditKind::Other, now);
            self.insert(replacement);
            self.end_edit();
        }
        self.find(query, case, whole, true) || hit
    }

    /// Substituir todos (um passo do desfazer). Devolve quantos.
    pub fn replace_all(&mut self, query: &str, replacement: &str, case: bool, whole: bool, now: f64) -> usize {
        let all = self.find_all(query, case, whole);
        let n = all.len();
        self.apply_edits(all.into_iter().map(|(a, b)| (a, b, replacement.to_string())).collect(), now);
        n
    }

    /// Como [`find_all`](Self::find_all), com todas as opções do localizar (expressão regular
    /// e o resto, veja [`SearchOptions`](crate::search::SearchOptions)); `within` limita a um
    /// trecho (o "Na seleção"). Erro: a expressão não é válida.
    pub fn find_with(&self, options: &crate::search::SearchOptions, within: Option<(usize, usize)>) -> Result<Vec<(usize, usize)>, String> {
        let found = options.find_all(&self.text)?;
        Ok(found.into_iter().map(|f| (f.start, f.end)).filter(|&(a, b)| within.is_none_or(|(s, e)| a >= s && b <= e)).collect())
    }

    /// Seleciona a próxima (ou a anterior) ocorrência com as opções, dando a volta.
    pub fn find_next_with(&mut self, options: &crate::search::SearchOptions, within: Option<(usize, usize)>, forward: bool) -> bool {
        let all = self.find_with(options, within).unwrap_or_default();
        if all.is_empty() {
            return false;
        }
        let (a, b) = self.selection();
        let pick = if forward { all.iter().find(|m| m.0 >= b && (m.0, m.1) != (a, b)).or(all.first()) } else { all.iter().rev().find(|m| m.1 <= a).or(all.last()) };
        let &(s, e) = pick.expect("há ocorrências");
        self.anchor = s;
        self.caret = e;
        self.reveal.set(true);
        true
    }

    /// Substituir com as opções: troca a seleção se ela é uma ocorrência (com os grupos e a
    /// caixa) e vai para a próxima.
    pub fn replace_with(&mut self, options: &crate::search::SearchOptions, within: Option<(usize, usize)>, template: &str, now: f64) -> bool {
        let sel = self.selection();
        let found = options.find_all(&self.text).unwrap_or_default();
        let hit = found.iter().find(|f| (f.start, f.end) == sel && f.start != f.end).copied();
        if let Some(f) = hit {
            let with = options.replacement(&self.text, &f, template);
            self.begin_edit(EditKind::Other, now);
            self.insert(&with);
            self.end_edit();
        }
        self.find_next_with(options, within, true) || hit.is_some()
    }

    /// Substituir todos com as opções (um passo do desfazer). Devolve quantos.
    pub fn replace_all_with(&mut self, options: &crate::search::SearchOptions, within: Option<(usize, usize)>, template: &str, now: f64) -> usize {
        let found: Vec<crate::search::Found> = options.find_all(&self.text).unwrap_or_default().into_iter().filter(|f| within.is_none_or(|(s, e)| f.start >= s && f.end <= e)).collect();
        let edits = found.iter().map(|f| (f.start, f.end, options.replacement(&self.text, f, template))).collect::<Vec<_>>();
        let n = edits.len();
        self.apply_edits(edits, now);
        n
    }

    /// Os trechos realçados (as ocorrências do localizar).
    pub fn set_highlights(&mut self, ranges: Vec<(usize, usize)>) {
        self.highlights = ranges;
    }

    // ---------------------------------------------------------------- servidor de linguagem

    /// Mostra a lista de sugestões; `start` é onde começa a palavra que elas completam.
    pub fn show_suggestions(&mut self, items: Vec<Suggestion>, start: usize) {
        self.suggestions = items;
        self.suggest_start = start.min(self.caret);
        self.suggest_index = 0;
        self.suggest_first.set(0);
        self.filter_suggestions();
    }

    pub fn hide_suggestions(&mut self) {
        self.suggestions.clear();
        self.suggest_shown.clear();
    }

    pub fn suggestions_visible(&self) -> bool {
        !self.suggest_shown.is_empty()
    }

    /// As sugestões que a lista mostra agora (as que casam com o digitado), na ordem.
    pub fn visible_suggestions(&self) -> Vec<&Suggestion> {
        self.suggest_shown.iter().filter_map(|&i| self.suggestions.get(i)).collect()
    }

    pub(super) fn filter_suggestions_now(&mut self) {
        self.filter_suggestions();
    }

    /// As sugestões que casam com o que já foi digitado da palavra (em ordem: as que
    /// começam igual primeiro).
    fn filter_suggestions(&mut self) {
        let typed: String = self.text.chars().skip(self.suggest_start).take(self.caret.saturating_sub(self.suggest_start)).collect();
        let typed = typed.to_lowercase();
        let mut scored: Vec<(u8, usize)> = self
            .suggestions
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                let label = s.label.to_lowercase();
                if label.starts_with(&typed) {
                    Some((0, i))
                } else if subsequence(&typed, &label) {
                    Some((1, i))
                } else {
                    None
                }
            })
            .collect();
        scored.sort_by_key(|&(score, i)| (score, i));
        self.suggest_shown = scored.into_iter().map(|(_, i)| i).collect();
        self.suggest_index = self.suggest_index.min(self.suggest_shown.len().saturating_sub(1));
    }

    /// Aceita a sugestão marcada: troca a palavra digitada por ela.
    fn accept_suggestion(&mut self, ctx: &mut EventCtx<'_>) {
        let Some(&i) = self.suggest_shown.get(self.suggest_index) else { return };
        let s = self.suggestions[i].clone();
        let text = if s.insert.is_empty() { s.label } else { s.insert };
        self.begin_edit(EditKind::Other, ctx.ui.time());
        // A palavra inteira sai (também o que estiver depois do cursor nela).
        let (_, word_end) = self.word_bounds(self.caret);
        self.anchor = self.suggest_start;
        self.caret = word_end.max(self.caret);
        self.insert(&text);
        self.end_edit();
        self.hide_suggestions();
        self.changed(ctx);
        // Uma função: abre a ajuda dos parâmetros se entrou o "(".
        if text.ends_with('(') {
            ctx.fire("on_signature_request", EventArgs::Value((self.caret as i64).into()));
        }
    }

    /// Uma dica perto de um índice do texto (o tipo e a documentação, ao parar o mouse).
    pub fn show_tip(&mut self, text: &str, at: usize) {
        self.tip = (!text.trim().is_empty()).then(|| (text.trim().to_string(), at));
    }

    pub fn hide_tip(&mut self) {
        self.tip = None;
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_ref().map(|(t, _)| t.as_str())
    }

    /// Onde o caractere `index` aparece (coordenadas do controle, a altura de uma linha), com a
    /// rolagem e as linhas da última pintura: quem mostra a dica por fora (a IDE, numa caixa
    /// própria) a põe perto da palavra. `None` antes da primeira pintura.
    pub fn index_rect(&self, ui: &aylis_core::Ui, index: usize) -> Option<RectF> {
        let lh = self.line_height.get();
        if lh <= 0.0 {
            return None;
        }
        let style = self.text_style.borrow().clone();
        let mut measure = |s: &str| ui.measure_text(s, &style).width;
        let rows = self.rows(self.text_width.get(), &mut measure);
        let row = *rows.get(Self::row_of(&rows, index))?;
        let r = Self::row_of(&rows, index);
        let at = index.clamp(row.start, row.end);
        let row_width = ui.measure_text(&expand_tabs(self.slice(row.start, row.end)), &style).width;
        let x = ui.measure_text(&expand_tabs(self.slice(row.start, at)), &style).width + self.shift(row_width);
        let (origin, scroll) = (self.origin.get(), self.scroll.get());
        Some(RectF::new(origin.x - scroll.x + x, origin.y + r as f32 * lh - scroll.y, 0.0, lh))
    }

    /// A ajuda dos parâmetros (a assinatura da função, acima do cursor). Vazio esconde.
    pub fn show_signature(&mut self, text: &str) {
        self.signature = (!text.trim().is_empty()).then(|| text.trim().to_string());
    }

    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }

    /// Os erros e avisos (sublinhados; a mensagem vem na dica).
    pub fn set_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics = diagnostics;
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// As mensagens dos erros no índice (para a dica).
    pub fn diagnostics_at(&self, index: usize) -> Vec<&Diagnostic> {
        self.diagnostics.iter().filter(|d| d.start <= index && index <= d.end.max(d.start + 1)).collect()
    }

    // ---------------------------------------------------------------- linhas

    /// As linhas (a partir de 0) da seleção (ou a do cursor).
    fn selected_lines(&self) -> (usize, usize) {
        let (a, b) = self.selection();
        let first = self.position_of_index(a).0;
        let mut last = self.position_of_index(b).0;
        // A seleção que termina no começo de uma linha não a pega.
        if b > a && last > first && self.line_start(b) == b {
            last -= 1;
        }
        (first, last)
    }

    /// O começo da linha `line` (a partir de 0), em caracteres.
    fn line_index(&self, line: usize) -> usize {
        self.text.split('\n').take(line).map(|l| l.chars().count() + 1).sum::<usize>().min(self.len())
    }

    fn line_count(&self) -> usize {
        self.text.split('\n').count()
    }

    /// O comentário de linha da linguagem (o do `language-configuration.json` registrado, se
    /// houver).
    fn comment_prefix(&self) -> String {
        if let Some(config) = crate::textmate::language_config(&self.syntax) {
            return config.line_comment.clone().unwrap_or_default();
        }
        match self.syntax.as_str() {
            "python" => "#",
            "json" => "",
            _ => "//",
        }
        .to_string()
    }

    /// Ctrl+/: comenta ou descomenta as linhas.
    pub fn toggle_comment(&mut self, now: f64) -> bool {
        let prefix = self.comment_prefix();
        let prefix = prefix.as_str();
        if prefix.is_empty() || self.read_only {
            return false;
        }
        let (first, last) = self.selected_lines();
        let lines: Vec<String> = self.text.split('\n').map(str::to_string).collect();
        let span = &lines[first..=last.min(lines.len() - 1)];
        let filled: Vec<&String> = span.iter().filter(|l| !l.trim().is_empty()).collect();
        let all_commented = !filled.is_empty() && filled.iter().all(|l| l.trim_start().starts_with(prefix));
        let indent = filled.iter().map(|l| l.chars().take_while(|c| *c == ' ' || *c == '\t').count()).min().unwrap_or(0);
        let mut edits = Vec::new();
        for (k, l) in span.iter().enumerate() {
            let start = self.line_index(first + k);
            if all_commented {
                let lead = l.chars().take_while(|c| *c == ' ' || *c == '\t').count();
                let rest: String = l.chars().skip(lead + prefix.chars().count()).collect();
                let remove = prefix.chars().count() + usize::from(rest.starts_with(' '));
                edits.push((start + lead, start + lead + remove, String::new()));
            } else if !l.trim().is_empty() {
                edits.push((start + indent, start + indent, format!("{prefix} ")));
            }
        }
        let (a, b) = (self.anchor, self.caret);
        let changed = self.apply_edits(edits, now);
        // A seleção continua nas mesmas linhas.
        let (fa, lb) = (self.line_index(first), self.line_index(last) + self.text.split('\n').nth(last).map_or(0, |l| l.chars().count()));
        if a != b {
            self.anchor = fa;
            self.caret = lb;
        }
        changed
    }

    /// Alt+Seta: sobe ou desce as linhas da seleção.
    pub fn move_lines(&mut self, up: bool, now: f64) -> bool {
        let (first, last) = self.selected_lines();
        if (up && first == 0) || (!up && last + 1 >= self.line_count()) || self.read_only {
            return false;
        }
        let mut lines: Vec<String> = self.text.split('\n').map(str::to_string).collect();
        let (caret_pos, anchor_pos) = (self.position_of_index(self.caret), self.position_of_index(self.anchor));
        self.begin_edit(EditKind::Other, now);
        if up {
            let moved = lines.remove(first - 1);
            lines.insert(last, moved);
        } else {
            let moved = lines.remove(last + 1);
            lines.insert(first, moved);
        }
        self.text = lines.join("\n");
        self.text_changed();
        let delta: isize = if up { -1 } else { 1 };
        let at = |p: (usize, usize)| (p.0 as isize + delta).max(0) as usize;
        self.caret = self.line_index(at(caret_pos)) + self.col_chars(at(caret_pos), caret_pos.1);
        self.anchor = self.line_index(at(anchor_pos)) + self.col_chars(at(anchor_pos), anchor_pos.1);
        self.end_edit();
        true
    }

    /// A coluna em caracteres a partir de uma em UTF-16 (dentro da linha).
    fn col_chars(&self, line: usize, col_utf16: usize) -> usize {
        self.index_of_position(line, col_utf16) - self.line_index(line)
    }

    /// Shift+Alt+Seta: copia as linhas para cima ou para baixo.
    pub fn duplicate_lines(&mut self, down: bool, now: f64) -> bool {
        if self.read_only {
            return false;
        }
        let (first, last) = self.selected_lines();
        let lines: Vec<&str> = self.text.split('\n').collect();
        let block = lines[first..=last].join("\n");
        let n = block.chars().count() + 1;
        let end = self.line_index(last) + lines[last].chars().count();
        let (caret, anchor) = (self.caret, self.anchor);
        self.apply_edits(vec![(end, end, format!("\n{block}"))], now);
        if down {
            self.caret = caret + n;
            self.anchor = anchor + n;
        } else {
            self.caret = caret;
            self.anchor = anchor;
        }
        true
    }

    /// Ctrl+Shift+K: apaga as linhas.
    pub fn delete_lines(&mut self, now: f64) -> bool {
        let (first, last) = self.selected_lines();
        let start = self.line_index(first);
        let total = self.line_count();
        let end = if last + 1 < total { self.line_index(last + 1) } else { self.len() };
        // A última linha leva o "\n" de antes.
        let start = if last + 1 >= total && start > 0 { start - 1 } else { start };
        let done = self.apply_edits(vec![(start, end, String::new())], now);
        let line = first.min(self.line_count().saturating_sub(1));
        let at = self.line_index(line);
        self.caret = at;
        self.anchor = at;
        done
    }

    /// Ctrl+Enter (abaixo) e Ctrl+Shift+Enter (acima): uma linha nova com o mesmo recuo.
    pub fn insert_line(&mut self, below: bool, now: f64) -> bool {
        let line = self.position_of_index(self.caret).0;
        let text = self.text.split('\n').nth(line).unwrap_or_default().to_string();
        let indent: String = text.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
        let at = if below { self.line_index(line) + text.chars().count() } else { self.line_index(line) };
        let piece = if below { format!("\n{indent}") } else { format!("{indent}\n") };
        self.apply_edits(vec![(at, at, piece)], now);
        let caret = if below { at + 1 + indent.chars().count() } else { at + indent.chars().count() };
        self.caret = caret;
        self.anchor = caret;
        true
    }

    /// Tab / Shift+Tab numa seleção de várias linhas, Ctrl+] e Ctrl+[: recua as linhas.
    pub fn indent_lines(&mut self, outdent: bool, now: f64) -> bool {
        let (first, last) = self.selected_lines();
        let lines: Vec<String> = self.text.split('\n').map(str::to_string).collect();
        let mut edits = Vec::new();
        for line in first..=last {
            let start = self.line_index(line);
            if outdent {
                let n = lines[line].chars().take(4).take_while(|c| *c == ' ').count();
                let n = if n == 0 && lines[line].starts_with('\t') { 1 } else { n };
                if n > 0 {
                    edits.push((start, start + n, String::new()));
                }
            } else if !lines[line].is_empty() {
                edits.push((start, start, "    ".into()));
            }
        }
        let had_selection = self.has_selection();
        let changed = self.apply_edits(edits, now);
        if had_selection {
            self.anchor = self.line_index(first);
            self.caret = self.line_index(last) + self.text.split('\n').nth(last).map_or(0, |l| l.chars().count());
        }
        changed
    }

    /// Ctrl+L: seleciona a linha (de novo: a de baixo também).
    pub fn select_line(&mut self) {
        let (first, last) = self.selected_lines();
        let last = if self.has_selection() && self.line_start(self.caret) == self.caret { last + 1 } else { last };
        self.anchor = self.line_index(first);
        self.caret = if last + 1 < self.line_count() { self.line_index(last + 1) } else { self.len() };
    }

    /// Ctrl+D: seleciona a palavra; com ela selecionada, a próxima ocorrência.
    pub fn select_next_occurrence(&mut self) -> bool {
        if !self.has_selection() {
            let (a, b) = self.word_bounds(self.caret);
            if a == b {
                return false;
            }
            self.anchor = a;
            self.caret = b;
            return true;
        }
        // A seleção é uma palavra inteira: procura só a palavra inteira (como no VS Code).
        let query = self.selected_text().to_string();
        let (a, b) = self.selection();
        let whole = query.chars().all(is_word) && self.word_bounds(a) == (a, b);
        self.find(&query, true, whole, true)
    }

    /// Home esperto: o primeiro caractere da linha que não é espaço; de novo, a coluna 0.
    pub(super) fn smart_home(&mut self, extend: bool) {
        let start = self.line_start(self.caret);
        let lead = self.text.chars().skip(start).take_while(|c| *c == ' ' || *c == '\t').count();
        let to = if self.caret == start + lead { start } else { start + lead };
        self.move_to(to, extend);
    }

    /// O par do colchete do lado do cursor: (o do lado, o par).
    pub fn matching_bracket(&self) -> Option<(usize, usize)> {
        // Sem copiar o texto (a pintura chama a cada quadro): anda pelos caracteres a partir
        // do colchete até o par.
        let check = |i: usize| -> Option<(usize, usize)> {
            let at = self.byte_at(i);
            let c = self.text[at..].chars().next()?;
            let (open, close, forward) = match c {
                '(' => ('(', ')', true),
                '[' => ('[', ']', true),
                '{' => ('{', '}', true),
                ')' => ('(', ')', false),
                ']' => ('[', ']', false),
                '}' => ('{', '}', false),
                _ => return None,
            };
            let mut depth = 0i32;
            if forward {
                for (k, d) in self.text[at..].chars().enumerate() {
                    depth += i32::from(d == open) - i32::from(d == close);
                    if depth == 0 {
                        return Some((i, i + k));
                    }
                }
            } else {
                for (k, d) in self.text[..at + c.len_utf8()].chars().rev().enumerate() {
                    depth += i32::from(d == close) - i32::from(d == open);
                    if depth == 0 {
                        return Some((i, i - k));
                    }
                }
            }
            None
        };
        check(self.caret).or_else(|| self.caret.checked_sub(1).and_then(check))
    }

    /// Como [`matching_bracket`](Self::matching_bracket), guardado até o texto ou o cursor
    /// mudarem (a pintura pede a cada quadro).
    pub(super) fn matching_bracket_cached(&self) -> Option<(usize, usize)> {
        let key = self.text_key();
        if let Some((k, caret, pair)) = self.bracket_cache.get() {
            if k == key && caret == self.caret {
                return pair;
            }
        }
        let pair = self.matching_bracket();
        self.bracket_cache.set(Some((key, self.caret, pair)));
        pair
    }

    /// Ctrl+Shift+\: vai para o par do colchete.
    pub fn jump_to_bracket(&mut self) -> bool {
        let Some((_, other)) = self.matching_bracket() else { return false };
        self.move_to(other, false);
        true
    }

    // ---------------------------------------------------------------- teclas

    /// As teclas do editor de código. `true`: tratada (o resto do `key_down` não roda).
    pub(super) fn code_key(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) -> bool {
        let shift = event.modifiers.shift;
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        let alt = event.modifiers.alt;
        let now = ctx.ui.time();
        // A lista de sugestões aberta fica com as setas, o Enter, o Tab e o Esc.
        if self.suggestions_visible() && !ctrl && !alt {
            let n = self.suggest_shown.len();
            let handled = match event.key {
                Key::Down => {
                    self.suggest_index = (self.suggest_index + 1) % n;
                    true
                }
                Key::Up => {
                    self.suggest_index = (self.suggest_index + n - 1) % n;
                    true
                }
                Key::PageDown => {
                    self.suggest_index = (self.suggest_index + LIST_ROWS).min(n - 1);
                    true
                }
                Key::PageUp => {
                    self.suggest_index = self.suggest_index.saturating_sub(LIST_ROWS);
                    true
                }
                Key::Enter | Key::Tab if !shift => {
                    self.accept_suggestion(ctx);
                    true
                }
                Key::Escape => {
                    self.hide_suggestions();
                    true
                }
                _ => false,
            };
            if handled {
                ctx.set_handled();
                self.activity(ctx);
                return true;
            }
        }
        if event.key == Key::Escape && (self.signature.is_some() || self.tip.is_some()) {
            self.signature = None;
            self.tip = None;
            ctx.set_handled();
            ctx.repaint();
            return true;
        }
        // Desfazer e refazer valem em todo Memo.
        let mut changed = false;
        match event.key {
            Key::Char('z') if ctrl && !shift && !alt => changed = self.undo(),
            Key::Char('y') if ctrl && !alt => changed = self.redo(),
            Key::Char('z') if ctrl && shift && !alt => changed = self.redo(),
            _ if !self.code_editing => return false,
            Key::Char(' ') if ctrl && !shift => {
                ctx.fire("on_complete_request", EventArgs::Value((self.caret as i64).into()));
            }
            Key::Char(' ') if ctrl && shift => {
                ctx.fire("on_signature_request", EventArgs::Value((self.caret as i64).into()));
            }
            Key::Char('/') if ctrl => changed = self.toggle_comment(now),
            // Ctrl+D: a palavra; de novo, mais um cursor na próxima ocorrência.
            Key::Char('d') if ctrl && !shift => {
                self.add_next_occurrence();
            }
            Key::Char('l') if ctrl && shift => {
                self.select_all_occurrences();
            }
            Key::Char('l') if ctrl && !shift => self.select_line(),
            Key::Char('k') if ctrl && shift => changed = self.delete_lines(now),
            // Ctrl+Alt+↑/↓: mais um cursor na linha de cima ou de baixo.
            Key::Up | Key::Down if ctrl && alt => {
                self.add_cursor_vertical(event.key == Key::Down);
            }
            // Ctrl+Shift+[ dobra e Ctrl+Shift+] desdobra (com o Shift, o teclado pode mandar a
            // chave no lugar do colchete).
            Key::Char('[' | '{') if ctrl && shift && self.folding => {
                self.fold_at_caret();
            }
            Key::Char(']' | '}') if ctrl && shift && self.folding => {
                self.unfold_at_caret();
            }
            Key::Char(']') if ctrl => changed = self.indent_lines(false, now),
            Key::Char('[') if ctrl => changed = self.indent_lines(true, now),
            Key::Char('\\') if ctrl && shift => {
                self.jump_to_bracket();
            }
            Key::Up | Key::Down if alt && !ctrl => {
                changed = if shift { self.duplicate_lines(event.key == Key::Down, now) } else { self.move_lines(event.key == Key::Up, now) };
            }
            Key::Enter if ctrl && !alt => changed = self.insert_line(!shift, now),
            Key::Home if !ctrl && !alt => self.smart_home(shift),
            Key::Tab if !ctrl && !alt && self.want_tabs && self.selected_lines().0 != self.selected_lines().1 => {
                changed = self.indent_lines(shift, now);
            }
            Key::Backspace if !ctrl && !alt && !self.has_selection() && !self.read_only => {
                // Entre um par vazio ("(|)"), apaga os dois.
                let chars: Vec<char> = self.text.chars().skip(self.caret.saturating_sub(1)).take(2).collect();
                if self.caret > 0 && chars.len() == 2 && self.pairs().iter().any(|&(o, c)| chars[0] == o && chars[1] == c) {
                    self.begin_edit(EditKind::Deleting, now);
                    let at = self.caret - 1;
                    self.delete_range(at, at + 2);
                    self.end_edit();
                    changed = true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
        ctx.set_handled();
        if changed {
            self.hide_suggestions();
            self.changed(ctx);
        }
        self.goal_x = None;
        self.activity(ctx);
        true
    }

    /// Os pares que fecham sozinhos: os do `language-configuration.json` registrado (os de um
    /// caractere), ou os de sempre.
    fn pairs(&self) -> Vec<(char, char)> {
        let from_config = crate::textmate::language_config(&self.syntax).map(|c| {
            let one = |s: &str| {
                let mut it = s.chars();
                it.next().filter(|_| it.next().is_none())
            };
            c.auto_closing.iter().filter_map(|(o, cl)| Some((one(o)?, one(cl)?))).collect::<Vec<_>>()
        });
        from_config.filter(|p| !p.is_empty()).unwrap_or_else(|| PAIRS.to_vec())
    }

    /// O digitado num editor de código: os pares que fecham sozinhos, pular o que fecha e
    /// pedir sugestões e a ajuda dos parâmetros. `true`: tratado.
    pub(super) fn code_text(&mut self, ctx: &mut EventCtx<'_>, text: &str) -> bool {
        let mut chars = text.chars();
        let (Some(c), None) = (chars.next(), chars.next()) else { return false };
        let now = ctx.ui.time();
        let next = self.text.chars().nth(self.caret);
        let prev = self.caret.checked_sub(1).and_then(|i| self.text.chars().nth(i));
        // Fecha o que já está fechado: só passa por cima.
        if !self.has_selection() && next == Some(c) && self.pairs().iter().any(|&(o, cl)| cl == c && (o != c || prev.is_some_and(|p| p != '\\'))) {
            self.move_to(self.caret + 1, false);
            if c == ')' {
                self.signature = None;
            }
            self.after_typing(ctx, c);
            return true;
        }
        if let Some((open, close)) = self.pairs().into_iter().find(|p| p.0 == c) {
            let quote = open == close;
            // Aspas dentro de uma palavra (o apóstrofo de "d'água") não abrem par.
            let word_before = prev.is_some_and(is_word);
            let free_after = next.is_none_or(|n| n.is_whitespace() || ")]};,".contains(n));
            if self.has_selection() {
                // Envolve a seleção.
                self.begin_edit(EditKind::Other, now);
                let (a, b) = self.selection();
                let inner = self.selected_text().to_string();
                self.insert(&format!("{open}{inner}{close}"));
                self.anchor = a + 1;
                self.caret = b + 1;
                self.end_edit();
                self.changed(ctx);
                self.activity(ctx);
                return true;
            }
            if free_after && !(quote && word_before) {
                self.begin_edit(EditKind::Typing, now);
                self.insert(&format!("{open}{close}"));
                self.caret -= 1;
                self.anchor = self.caret;
                self.end_edit();
                self.changed(ctx);
                self.after_typing(ctx, c);
                return true;
            }
        }
        false
    }

    /// Depois de digitar `c`: a lista de sugestões acompanha, e o servidor é chamado.
    pub(super) fn after_typing(&mut self, ctx: &mut EventCtx<'_>, c: char) {
        if !self.code_editing {
            return;
        }
        if is_word(c) || c == '.' || (c == ':' && self.caret >= 2 && self.text.chars().nth(self.caret - 2) == Some(':')) || c == '>' {
            if self.suggestions_visible() && is_word(c) {
                self.filter_suggestions();
            }
            ctx.fire("on_complete_request", EventArgs::Value((self.caret as i64).into()));
        } else {
            self.hide_suggestions();
        }
        if c == '(' || c == ',' {
            ctx.fire("on_signature_request", EventArgs::Value((self.caret as i64).into()));
        } else if c == ')' {
            self.signature = None;
        }
        self.activity(ctx);
    }

    // ---------------------------------------------------------------- mouse

    /// O mouse parou numa palavra: avisa a IDE (para a dica) quando muda de palavra.
    pub(super) fn hover_at(&mut self, ctx: &mut EventCtx<'_>, index: Option<usize>) {
        let word = index.map(|i| self.word_bounds(i)).filter(|(a, b)| a < b);
        let diag = index.filter(|&i| !self.diagnostics_at(i).is_empty());
        let key = word.map(|w| w.0).or(diag);
        if key == self.hover_word {
            return;
        }
        self.hover_word = key;
        let value = match (word, index) {
            (Some(_), Some(i)) => i as i64,
            (None, Some(i)) if diag.is_some() => i as i64,
            _ => -1,
        };
        if value < 0 && self.tip.is_some() {
            self.tip = None;
            ctx.repaint();
        }
        ctx.fire("on_hover_index", EventArgs::Value(value.into()));
    }

    /// Com Ctrl (`ctrl`) e o mouse sobre um nome (variável, função, tipo) num editor de
    /// código: o nome fica sublinhado e o cursor vira a mãozinha, dizendo que o Ctrl+clique
    /// leva à declaração. Sem Ctrl, fora de um nome ou fora do texto, volta ao normal.
    pub(super) fn update_link(&mut self, ctx: &mut EventCtx<'_>, ctrl: bool) {
        let link = self
            .pointer_at
            .filter(|p| ctrl && self.code_editing && !self.dragging && self.bar_at(*p).is_none() && p.x >= self.origin.get().x)
            .map(|p| self.word_bounds(self.index_at(ctx, p)))
            .filter(|&(a, b)| a < b && self.text.chars().nth(a).is_some_and(|c| c.is_alphabetic() || c == '_'));
        if link == self.link {
            return;
        }
        self.link = link;
        self.base.set_cursor(if link.is_some() { aylis_core::CursorShape::Pointer } else { aylis_core::CursorShape::Text });
        ctx.repaint();
    }

    /// Clique na lista de sugestões: aceita o item. `true`: era na lista.
    pub(super) fn click_suggestion(&mut self, ctx: &mut EventCtx<'_>, p: PointF) -> bool {
        let Some((rect, row_h)) = self.suggest_rect.get().filter(|_| self.suggestions_visible()) else { return false };
        if !rect.contains(p) {
            self.hide_suggestions();
            return false;
        }
        let row = ((p.y - rect.y) / row_h).floor().max(0.0) as usize + self.suggest_first.get();
        if row < self.suggest_shown.len() {
            self.suggest_index = row;
            self.accept_suggestion(ctx);
        }
        ctx.set_handled();
        self.activity(ctx);
        true
    }

    // ---------------------------------------------------------------- pintura

    /// Por linha (dentro do recorte do texto): as ocorrências, o par do colchete e os erros.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn paint_row_marks(
        &self,
        canvas: &mut dyn Canvas,
        ctx: &PaintCtx<'_>,
        style: &TextStyle,
        row: (usize, usize),
        x: f32,
        y: f32,
        lh: f32,
        occurrences: &[(usize, usize)],
        brackets: Option<(usize, usize)>,
    ) {
        let width_of = |canvas: &mut dyn Canvas, a: usize, b: usize| canvas.measure_text(&expand_tabs(self.slice(a, b)), style).width;
        let (rs, re) = row;
        let mark = |canvas: &mut dyn Canvas, a: usize, b: usize, fill: Option<aylis_core::Color>, border: Option<aylis_core::Color>| {
            if b <= rs || a >= re.max(rs + 1) || a > re {
                return;
            }
            let x1 = x + width_of(canvas, rs, a.max(rs));
            let x2 = x + width_of(canvas, rs, b.min(re));
            let r = RectF::new(x1, y + 1.0, (x2 - x1).max(2.0), lh - 2.0);
            if let Some(f) = fill {
                canvas.fill_rect(r, 2.0, &Brush::Solid(f));
            }
            if let Some(b) = border {
                canvas.stroke_rect(r, 2.0, &Stroke::new(b, 1.0));
            }
        };
        let accent = ctx.theme.color("primary");
        for &(a, b) in &self.highlights {
            mark(canvas, a, b, Some(ctx.theme.color("warning").with_opacity(0.35)), None);
        }
        for &(a, b) in occurrences {
            mark(canvas, a, b, Some(style.color.with_opacity(0.10)), None);
        }
        if let Some((a, b)) = brackets {
            for i in [a, b] {
                mark(canvas, i, i + 1, Some(accent.with_opacity(0.12)), Some(accent.with_opacity(0.7)));
            }
        }
        // O nome sob o mouse com Ctrl: sublinhado, como um link.
        if let Some((a, b)) = self.link.filter(|&(a, b)| b > rs && a < re.max(rs + 1)) {
            let x1 = x + width_of(canvas, rs, a.max(rs));
            let x2 = x + width_of(canvas, rs, b.min(re));
            let line_y = y + lh - 2.5;
            canvas.draw_line(PointF::new(x1, line_y), PointF::new(x2, line_y), &Stroke::new(accent, 1.0));
        }
        for d in &self.diagnostics {
            let (a, b) = (d.start, d.end.max(d.start + 1));
            if b <= rs || a > re {
                continue;
            }
            let color = match d.severity {
                1 => ctx.theme.color("danger"),
                2 => ctx.theme.color("warning"),
                _ => ctx.theme.color("primary"),
            };
            let x1 = x + width_of(canvas, rs, a.max(rs));
            let x2 = (x + width_of(canvas, rs, b.min(re))).max(x1 + 6.0);
            crate::edit::draw_wave(canvas, x1, x2, y + lh - 3.0, &Stroke::new(color, 1.2));
        }
    }

    /// As ocorrências da palavra do cursor nas linhas à vista (sem seleção).
    pub(super) fn occurrences(&self, from: usize, to: usize) -> Vec<(usize, usize)> {
        if !self.code_editing || self.has_selection() {
            return Vec::new();
        }
        let (a, b) = self.word_bounds(self.caret);
        if b <= a || b - a < 2 {
            return Vec::new();
        }
        // Só no trecho à vista, sem copiar o texto (a pintura chama a cada quadro).
        let word = &self.text[self.byte_at(a)..self.byte_at(b)];
        let n = b - a;
        let (start, end) = (self.byte_at(from), self.byte_at(to.max(from)));
        let view = &self.text[start..end];
        let mut out = Vec::new();
        // O índice (em caracteres) de cada achado, contado a partir do anterior.
        let (mut chars, mut last) = (from, 0);
        for (pos, _) in view.match_indices(word) {
            if pos < last {
                continue;
            }
            chars += view[last..pos].chars().count();
            last = pos;
            let before = self.text[..start + pos].chars().next_back();
            let after = self.text[start + pos + word.len()..].chars().next();
            if !before.is_some_and(is_word) && !after.is_some_and(is_word) {
                out.push((chars, chars + n));
            }
        }
        out
    }

    /// Por cima de tudo: a lista de sugestões, a ajuda dos parâmetros e a dica. `caret` é
    /// a posição do cursor na tela; `area` a do texto.
    pub(super) fn paint_popups(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, style: &TextStyle, caret: PointF, lh: f32, area: RectF, tip_at: Option<PointF>) {
        let fill = ctx.theme.color("surface");
        let border = ctx.theme.color("border");
        let muted = ctx.theme.color("text_muted");
        let mut ui_style = ctx.text_style();
        ui_style.v_align = TextAlign::Center;
        ui_style.trimming = Trimming::Character;
        let box_at = |canvas: &mut dyn Canvas, r: RectF| {
            canvas.fill_rect(RectF::new(r.x + 1.0, r.y + 2.0, r.width, r.height), 6.0, &Brush::Solid(aylis_core::Color::rgba(0, 0, 0, 40)));
            canvas.fill_rect(r, 6.0, &Brush::Solid(fill));
            canvas.stroke_rect(r, 6.0, &Stroke::new(border, 1.0));
        };
        // A lista de sugestões, embaixo do cursor (ou em cima, sem lugar).
        if self.suggestions_visible() {
            let row_h = (ui_style.font.size * 1.7).round();
            let rows = self.suggest_shown.len().min(LIST_ROWS);
            let width = 420.0f32.min(area.width.max(200.0));
            let height = rows as f32 * row_h + 4.0;
            let below = caret.y + lh + height <= area.bottom() || caret.y - height < area.y;
            let y = if below { caret.y + lh + 2.0 } else { caret.y - height - 2.0 };
            let x = caret.x.min(area.right() - width).max(area.x);
            let rect = RectF::new(x, y, width, height);
            box_at(canvas, rect);
            // A rolagem da lista segue o marcado.
            let first = if self.suggest_index >= self.suggest_first.get() + LIST_ROWS {
                self.suggest_index + 1 - LIST_ROWS
            } else if self.suggest_index < self.suggest_first.get() {
                self.suggest_index
            } else {
                self.suggest_first.get()
            };
            // (o `suggest_first` é guardado na pintura: a lista rola junto)
            let first = first.min(self.suggest_shown.len().saturating_sub(rows));
            self.suggest_first.set(first);
            self.suggest_rect.set(Some((RectF::new(rect.x, rect.y + 2.0, rect.width, rows as f32 * row_h), row_h)));
            for (k, &i) in self.suggest_shown.iter().skip(first).take(rows).enumerate() {
                let s = &self.suggestions[i];
                let r = RectF::new(rect.x + 2.0, rect.y + 2.0 + k as f32 * row_h, rect.width - 4.0, row_h);
                if first + k == self.suggest_index {
                    canvas.fill_rect(r, 4.0, &Brush::Solid(ctx.theme.color("selection")));
                }
                let (letter, color) = kind_badge(&s.kind, ctx);
                let mut badge = ui_style.clone();
                badge.color = color;
                badge.h_align = TextAlign::Center;
                badge.font.weight = aylis_core::FontWeight::Bold;
                canvas.draw_text(letter, RectF::new(r.x + 4.0, r.y, 18.0, row_h), &badge);
                let mut label = ui_style.clone();
                label.color = style.color;
                let detail_w = if s.detail.is_empty() { 0.0 } else { (r.width * 0.45).min(canvas.measure_text(&s.detail, &ui_style).width + 8.0) };
                canvas.draw_text(&s.label, RectF::new(r.x + 26.0, r.y, r.width - 34.0 - detail_w, row_h), &label);
                if detail_w > 0.0 {
                    let mut d = ui_style.clone();
                    d.color = muted;
                    d.h_align = TextAlign::Trailing;
                    canvas.draw_text(&s.detail, RectF::new(r.right() - detail_w - 6.0, r.y, detail_w, row_h), &d);
                }
            }
        } else {
            self.suggest_rect.set(None);
        }
        // A ajuda dos parâmetros, em cima da linha do cursor.
        if let Some(sig) = &self.signature {
            self.paint_note(canvas, ctx, sig, PointF::new(caret.x, caret.y), lh, area, true);
        }
        // A dica (o tipo, a documentação, o erro), perto da palavra.
        if let (Some((text, _)), Some(at)) = (&self.tip, tip_at) {
            self.paint_note(canvas, ctx, text, at, lh, area, self.signature.is_none());
        }
    }

    /// Uma caixa de texto (várias linhas) acima (ou abaixo) de um ponto.
    #[allow(clippy::too_many_arguments)]
    fn paint_note(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, text: &str, at: PointF, lh: f32, area: RectF, above: bool) {
        let mut ts = ctx.text_style();
        ts.v_align = TextAlign::Leading;
        ts.wrap = true;
        ts.trimming = Trimming::None;
        let max_w = 560.0f32.min(area.width - 8.0).max(160.0);
        let lines: Vec<&str> = text.lines().take(14).collect();
        let longest = lines.iter().map(|l| canvas.measure_text(l, &ts).width).fold(0.0f32, f32::max);
        let width = (longest + 20.0).min(max_w);
        let line_h = (ts.font.size * 1.4).round();
        let wrapped: usize = lines.iter().map(|l| ((canvas.measure_text(l, &ts).width / (width - 20.0)).ceil() as usize).max(1)).sum();
        let height = wrapped as f32 * line_h + 14.0;
        let fits_above = at.y - height - 4.0 >= area.y;
        let y = if above && fits_above { at.y - height - 4.0 } else { at.y + lh + 4.0 };
        let x = at.x.min(area.right() - width).max(area.x);
        let r = RectF::new(x, y, width, height);
        canvas.fill_rect(RectF::new(r.x + 1.0, r.y + 2.0, r.width, r.height), 6.0, &Brush::Solid(aylis_core::Color::rgba(0, 0, 0, 40)));
        canvas.fill_rect(r, 6.0, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(r, 6.0, &Stroke::new(ctx.theme.color("border"), 1.0));
        canvas.draw_text(&lines.join("\n"), r.deflate(aylis_core::Bounds::all(7.0)), &ts);
    }
}

/// A letra e a cor do tipo de uma sugestão.
fn kind_badge(kind: &str, ctx: &PaintCtx<'_>) -> (&'static str, aylis_core::Color) {
    let k = kind.to_lowercase();
    if k.contains("method") || k.contains("function") || k.contains("constructor") {
        ("ƒ", ctx.theme.color("primary"))
    } else if k.contains("class") || k.contains("struct") || k.contains("interface") || k.contains("enum") || k.contains("type") {
        ("C", ctx.theme.color("warning"))
    } else if k.contains("field") || k.contains("property") {
        ("P", ctx.theme.color("success"))
    } else if k.contains("keyword") || k.contains("snippet") {
        ("K", ctx.theme.color("text_muted"))
    } else if k.contains("module") || k.contains("unit") || k.contains("file") {
        ("M", ctx.theme.color("text_muted"))
    } else {
        ("v", ctx.theme.color("primary"))
    }
}

/// As letras de `needle` aparecem em ordem em `hay` (a busca "aproximada" da lista).
fn subsequence(needle: &str, hay: &str) -> bool {
    let mut it = hay.chars();
    needle.chars().all(|c| it.any(|h| h == c))
}

#[cfg(test)]
mod undo_tests {
    use super::*;

    /// Uma troca feita de fora (o `edit_code` do agente), com o cursor do usuário noutro lugar:
    /// o Ctrl+Z desfaz e o cursor vai ao trecho desfeito, não ao lugar de antes.
    #[test]
    fn undo_puts_the_caret_on_what_was_undone() {
        let mut m = Memo::default();
        m.text = "linha 1\nlinha 2\nlinha 3\n".to_string();
        m.select(2, 2);
        m.apply_edits(vec![(16, 23, "trocada".to_string())], 0.0);
        assert!(m.text.contains("trocada"));
        m.select(2, 2);
        assert!(m.undo());
        assert_eq!(m.text, "linha 1\nlinha 2\nlinha 3\n");
        assert_eq!(m.selection(), (23, 23), "no fim do trecho desfeito");
        assert!(m.redo());
        assert_eq!(m.selection().0, 23);
        // Digitação: o cursor guardado está no trecho e continua valendo.
        assert_eq!(changed_region("abc", "abXc"), (2, 3));
        assert_eq!(changed_region("abXc", "abc"), (2, 2));
    }
}
