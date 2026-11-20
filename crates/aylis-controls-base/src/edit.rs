//! Campo de texto de uma linha.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
    PaintCtx, PointF, PointerButton, PointerEvent, PointerKind, RectF, Stroke, TextAlign, TextStyle, Trimming, Ui, properties,
};

use aylis_core::messages::{t, tf};

use crate::leaf_control;

/// Campo de texto de uma linha.
///
/// - mouse: clique posiciona o cursor, arrastar seleciona, duplo clique seleciona a
///   palavra, triplo clique seleciona tudo;
/// - teclado: setas, Home/End (com Shift seleciona, com Ctrl anda por palavra),
///   Backspace/Delete (com Ctrl apagam a palavra), Ctrl+A/C/X/V;
/// - `on_change` dispara a cada alteração do texto pelo usuário.
///
/// Ao receber o foco, seleciona tudo.
///
/// Texto: `h_align` e `v_align` posicionam o texto;
/// fora da edição, `wrap` quebra em várias linhas (se o campo for alto) e `trimming`
/// põe "…" no que não couber (por caractere, por palavra ou nenhum). Editando, o texto
/// fica numa linha e rola com o cursor.
///
/// `icon` mostra um ícone à esquerda (a lupa da busca) e `clear_button` um "×" à direita
/// que apaga o texto (só aparece com texto).
///
/// Com o `auto_size` (do `ControlBase`), o campo fica da largura do texto enquanto se digita
/// (`min_width`/`max_width` limitam); com `plain`, é só o texto (sem fundo, borda nem
/// padding), como a anotação sobre uma imagem. Sem fundo, a seleção fica translúcida e o
/// cursor ganha um contorno na cor do fundo do tema (aparece sobre qualquer cor).
///
/// Como o `v-text-field` do Vuetify: `label` (rótulo que sobe para a borda com foco ou
/// texto), `helper` (ajuda embaixo), `error` (mensagem de erro, borda vermelha),
/// `counter` (`3/20` embaixo), `prefix`/`suffix` (`R$`, `kg`), `mask` (`#` dígito,
/// `A` letra, `X` letra ou dígito; o resto entra sozinho: `(##) #####-####`) e `rules`
/// (`required;email;min:3;max:20;number;cpf`, conferidas ao sair do campo e por
/// [`validate_form`]). Ajuda, erro e contador ocupam os 18 pontos de baixo do controle.
pub struct Edit {
    pub base: ControlBase,
    /// O texto do campo (com a `mask`, já formatado).
    pub text: String,
    /// Texto apagado mostrado enquanto o campo está vazio.
    pub placeholder: String,
    /// A borda da caixa (desligada: só o texto, sem o contorno nem o traço do foco; dentro de
    /// um painel ou numa tabela). Ligada por padrão.
    pub border: bool,
    /// Só o texto: sem o fundo do tema (um `fill` próprio ainda vale), sem a borda e sem o
    /// padding. Com o `auto_size`, o campo é só as letras e cresce para a direita.
    pub plain: bool,
    /// Limite de caracteres; 0 = sem limite.
    pub max_length: usize,
    /// Os caracteres liberados (vencem um bloqueio do form ou de um painel), os bloqueados
    /// e se só os liberados valem (`allowed_chars = "0-9"` com `only_allowed`: só números).
    /// Faixas como `a-z`; `-` é o hífen. Vale para o digitado e o colado.
    pub allowed_chars: String,
    /// Os caracteres recusados ao digitar e ao colar (faixas como `a-z`); vazio: nenhum.
    pub blocked_chars: String,
    /// Só os caracteres de `allowed_chars` valem; o resto é recusado.
    pub only_allowed: bool,
    /// Trocas do que o teclado produz: pares "digitado, entregue" (`",."` troca a vírgula por
    /// ponto), ao digitar e ao colar; vale também o de um contêiner (veja o `Form`).
    pub char_map: String,
    /// Maiúsculas ou minúsculas no valor: ao digitar, ao colar e quando
    /// o código grava o `text`. Ligado a um campo, manda o `char_case` do campo.
    pub char_case: aylis_core::CharCase,
    /// Mostra e deixa selecionar e copiar, mas não deixa editar.
    pub read_only: bool,
    /// Mostra `•` no lugar dos caracteres e bloqueia copiar/recortar.
    pub password: bool,
    /// Dado pessoal (LGPD): as gravações do `EventRecorder` mascaram o que é digitado aqui,
    /// a não ser com o `record_sensitive` dele ligado. Liga e desliga em tempo de execução.
    pub sensitive: bool,
    /// Ganhar o foco seleciona o texto todo (ligado por padrão).
    pub auto_select: bool,
    /// Posição do texto na largura: começo, centro ou fim.
    pub h_align: TextAlign,
    /// Posição do texto na altura: em cima, no meio ou embaixo.
    pub v_align: TextAlign,
    /// Quebra automática de linha (fora da edição).
    pub wrap: bool,
    /// Reticências quando o texto não cabe (fora da edição).
    pub trimming: Trimming,
    /// Ícone à esquerda do texto (a lupa da busca); vazio: nenhum.
    pub icon: String,
    /// Um "×" à direita que apaga o texto (só aparece com texto).
    pub clear_button: bool,
    /// Rótulo do campo, que sobe para a borda com foco ou com texto; vazio: sem rótulo.
    pub label: String,
    /// Texto de ajuda embaixo do campo (o `error` aparece no lugar dele quando há).
    pub helper: String,
    /// Mensagem de erro embaixo do campo, com a borda vermelha; vazio: sem erro.
    pub error: String,
    /// Mostra o contador de caracteres embaixo (`3/20` com `max_length`).
    pub counter: bool,
    /// Texto fixo antes do valor (`R$`), fora do que é editado.
    pub prefix: String,
    /// Texto fixo depois do valor (`kg`), fora do que é editado.
    pub suffix: String,
    /// Máscara de digitação: `#` dígito, `A` letra, `X` letra ou dígito, o resto entra
    /// sozinho (`(##) #####-####`); vazio: livre.
    pub mask: String,
    /// Regras conferidas ao sair do campo e por `validate_form`, separadas por `;`
    /// (`required;email;min:3;max:20;number;cpf`); vazio: nenhuma.
    pub rules: String,
    /// Sublinha as palavras erradas (o motor de texto da plataforma ou o do app); o clique
    /// direito numa delas mostra as correções.
    pub spellcheck: bool,
    /// Completa a palavra enquanto digita: o resto aparece apagado depois do cursor e o
    /// Tab (ou a seta para a direita) aceita.
    pub autocomplete: bool,
    /// Idioma da correção e das sugestões (`"pt-BR"`); vazio: o do sistema.
    pub language: String,
    /// Celular: a faixa de sugestões e a correção automática do teclado do sistema
    /// (desligue em códigos, logins, identificadores).
    pub keyboard_suggestions: bool,
    /// O menu de texto padrão (Recortar, Copiar, Colar, Selecionar tudo) no botão direito e no
    /// toque longo, que no toque seleciona a palavra (com as alças para ajustar). Só abre
    /// quando nem o campo nem um pai tem `popup_menu`. Ligado por padrão.
    pub text_menu: bool,
    /// Cor do cursor; vazio: a cor do texto.
    pub caret_color: Option<Color>,
    /// Largura do cursor (linha) ou altura (sublinhado), em pontos.
    pub caret_width: f32,
    /// Forma do cursor: linha, bloco ou sublinhado.
    pub caret_style: CaretStyle,
    /// Intervalo de piscar (ms, o ciclo inteiro); 0: não pisca.
    pub caret_blink: u32,
    /// Cursor e âncora da última vez (para o `on_caret_move` só quando mudam).
    last_selection: (usize, usize),
    /// Palavras erradas (trechos em bytes de `text`).
    misspelled: Vec<(usize, usize)>,
    /// O resto da palavra sugerida (aparece depois do cursor).
    completion: String,
    /// O menu de texto e das correções (filho do próprio Edit, criado na primeira vez).
    spell_menu: Option<ComponentId>,
    /// O menu de texto está aberto (ele tem o foco): a seleção continua à vista, e o foco que
    /// volta ao campo não seleciona tudo.
    menu_open: bool,
    /// A seleção veio do toque longo: as alças aparecem nas pontas.
    touch_select: bool,
    /// Arrastando uma alça: se é a do começo e o `pan_on_touch` de antes.
    handle_drag: Option<(bool, bool)>,
    /// As áreas de toque das alças na última pintura (local).
    handles: Cell<Option<(RectF, RectF)>>,
    /// O pé da linha do texto na última pintura (as alças e o menu do toque ficam embaixo).
    line_bottom: Cell<f32>,
    /// Onde ficou o "×" na última pintura (local).
    clear_rect: Cell<Option<RectF>>,
    /// Desfazer e refazer do que o usuário digitou (Ctrl+Z, Ctrl+Y): (texto, cursor, âncora).
    /// `committed` é o estado depois da última alteração; digitar em sequência (menos de
    /// [`UNDO_PAUSE`] entre as teclas) vira um passo só.
    undo: Vec<(String, usize, usize)>,
    redo: Vec<(String, usize, usize)>,
    committed: Option<(String, usize, usize)>,
    last_change: f64,
    /// Posição do cursor e da âncora da seleção, em caracteres.
    caret: usize,
    anchor: usize,
    scroll: Cell<f32>,
    /// Onde o texto começa (x local), guardado na pintura para o tratamento do mouse.
    text_left: Cell<f32>,
    /// Último estilo de texto pintado (para medir no tratamento do mouse).
    text_style: RefCell<TextStyle>,
    blink_start: f64,
    dragging: bool,
    /// Caracteres aceitos ao digitar ou colar (NumberBox, DateEdit...). `None` = todos.
    pub(crate) filter: Option<fn(char) -> bool>,
    /// Não dispara `on_change` (quem compõe o Edit dispara o próprio).
    pub(crate) silent: bool,
    /// Espaço reservado à direita (botões do NumberBox, do DateEdit).
    pub(crate) right_inset: f32,
}

impl Default for Edit {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Text);
        Self {
            base,
            text: String::new(),
            placeholder: String::new(),
            border: true,
            plain: false,
            max_length: 0,
            allowed_chars: String::new(),
            blocked_chars: String::new(),
            only_allowed: false,
            char_map: String::new(),
            char_case: aylis_core::CharCase::Normal,
            read_only: false,
            password: false,
            sensitive: false,
            auto_select: true,
            h_align: TextAlign::Leading,
            v_align: TextAlign::Center,
            wrap: false,
            trimming: Trimming::Character,
            icon: String::new(),
            clear_button: false,
            label: String::new(),
            helper: String::new(),
            error: String::new(),
            counter: false,
            prefix: String::new(),
            suffix: String::new(),
            mask: String::new(),
            rules: String::new(),
            spellcheck: false,
            autocomplete: false,
            language: String::new(),
            keyboard_suggestions: true,
            text_menu: true,
            caret_color: None,
            caret_width: 1.5,
            caret_style: CaretStyle::Line,
            caret_blink: 1000,
            last_selection: (0, 0),
            misspelled: Vec::new(),
            completion: String::new(),
            spell_menu: None,
            menu_open: false,
            touch_select: false,
            handle_drag: None,
            handles: Cell::new(None),
            line_bottom: Cell::new(0.0),
            clear_rect: Cell::new(None),
            undo: Vec::new(),
            redo: Vec::new(),
            committed: None,
            last_change: f64::NEG_INFINITY,
            caret: 0,
            anchor: 0,
            scroll: Cell::new(0.0),
            text_left: Cell::new(0.0),
            text_style: RefCell::default(),
            blink_start: 0.0,
            dragging: false,
            filter: None,
            silent: false,
            right_inset: 0.0,
        }
    }
}

impl Edit {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let end = text.chars().count();
        Self { text, caret: end, anchor: end, ..Default::default() }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Posição do cursor, em caracteres.
    /// As palavras marcadas como erradas (trechos em bytes de `text`).
    pub fn misspelled(&self) -> &[(usize, usize)] {
        &self.misspelled
    }

    /// O resto da palavra sugerida que aparece depois do cursor (vazio: nenhuma).
    pub fn completion(&self) -> &str {
        &self.completion
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    /// Seleção atual (início, fim), em caracteres. Vazia quando início == fim.
    pub fn selection(&self) -> (usize, usize) {
        (self.caret.min(self.anchor), self.caret.max(self.anchor))
    }

    pub fn selected_text(&self) -> &str {
        let (start, end) = self.selection();
        &self.text[self.byte(start)..self.byte(end)]
    }

    /// Seleciona do caractere `start` até `end`.
    pub fn select(&mut self, start: usize, end: usize) {
        let len = self.len();
        self.anchor = start.min(len);
        self.caret = end.min(len);
    }

    pub fn select_all(&mut self) {
        self.select(0, self.len());
    }

    fn len(&self) -> usize {
        self.text.chars().count()
    }

    fn byte(&self, chars: usize) -> usize {
        byte_index(&self.text, chars)
    }

    fn display(&self) -> Cow<'_, str> {
        if self.password { Cow::Owned("•".repeat(self.len())) } else { Cow::Borrowed(&self.text) }
    }

    /// Chamado pelo macro depois que uma propriedade é gravada.
    fn after_set(&mut self, name: &str) {
        if (name == "text" || name == "mask") && !self.mask.is_empty() {
            self.text = apply_mask(&self.mask, &self.text);
        }
        if name == "text" {
            let end = self.len();
            self.caret = end;
            self.anchor = end;
        }
    }

    fn has_selection(&self) -> bool {
        self.caret != self.anchor
    }

    fn delete_range(&mut self, start: usize, end: usize) -> bool {
        if start >= end {
            return false;
        }
        let (a, b) = (self.byte(start), self.byte(end));
        self.text.replace_range(a..b, "");
        self.caret = start;
        self.anchor = start;
        true
    }

    fn delete_selection(&mut self) -> bool {
        let (start, end) = self.selection();
        self.delete_range(start, end)
    }

    /// Insere no cursor, substituindo a seleção e respeitando `max_length`.
    fn insert(&mut self, text: &str) -> bool {
        if self.read_only {
            return false;
        }
        let clean: String = text.chars().filter(|c| !c.is_control() && self.filter.is_none_or(|f| f(*c))).collect();
        let deleted = self.delete_selection();
        let room = if self.max_length == 0 { usize::MAX } else { self.max_length.saturating_sub(self.len()) };
        let piece: String = clean.chars().take(room).collect();
        if piece.is_empty() {
            return deleted;
        }
        let at = self.byte(self.caret);
        self.text.insert_str(at, &piece);
        self.caret += piece.chars().count();
        self.anchor = self.caret;
        true
    }

    fn move_to(&mut self, position: usize, extend: bool) {
        self.caret = position.min(self.len());
        if !extend {
            self.anchor = self.caret;
        }
    }

    fn prev_word(&self, from: usize) -> usize {
        let chars: Vec<char> = self.text.chars().collect();
        let mut i = from.min(chars.len());
        while i > 0 && chars[i - 1].is_whitespace() {
            i -= 1;
        }
        while i > 0 && !chars[i - 1].is_whitespace() {
            i -= 1;
        }
        i
    }

    fn next_word(&self, from: usize) -> usize {
        let chars: Vec<char> = self.text.chars().collect();
        let mut i = from.min(chars.len());
        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        i
    }

    /// Caractere mais próximo de `x` (coordenada local do controle).
    fn index_at(&self, ctx: &EventCtx<'_>, x: f32) -> usize {
        let style = self.text_style.borrow().clone();
        let display = self.display();
        let target = x - self.text_left.get() + self.scroll.get();
        let width = |n: usize| ctx.ui.measure_text(&display[..byte_index(&display, n)], &style).width;
        // Busca binária: as larguras dos prefixos crescem com o número de caracteres.
        let (mut lo, mut hi) = (0usize, self.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if width(mid + 1) <= target { lo = mid + 1 } else { hi = mid }
        }
        if lo < self.len() {
            let (left, right) = (width(lo), width(lo + 1));
            if target - left > right - target {
                return lo + 1;
            }
        }
        lo
    }

    /// O ponto de partida do histórico: o texto antes da primeira alteração do usuário.
    fn start_history(&mut self) {
        if self.committed.is_none() {
            self.committed = Some((self.text.clone(), self.caret, self.anchor));
        }
    }

    /// Guarda no histórico o estado de antes da alteração do usuário (o `committed`), a não
    /// ser que ela continue a anterior (digitando em sequência).
    fn remember(&mut self, now: f64) {
        let now_state = (self.text.clone(), self.caret, self.anchor);
        let before = self.committed.replace(now_state);
        // O texto não mudou (desfazer, refazer, só o cursor): o histórico fica como está.
        if before.as_ref().is_some_and(|b| b.0 == self.text) {
            return;
        }
        if let Some(before) = before
            && now - self.last_change > UNDO_PAUSE
        {
            self.undo.push(before);
            if self.undo.len() > 100 {
                self.undo.remove(0);
            }
        }
        self.redo.clear();
        self.last_change = now;
    }

    /// Desfaz (`back`) ou refaz um passo; `false` se não havia.
    fn step(&mut self, back: bool) -> bool {
        let popped = if back { self.undo.pop() } else { self.redo.pop() };
        let Some((text, caret, anchor)) = popped else { return false };
        let current = (std::mem::replace(&mut self.text, text), self.caret, self.anchor);
        if back {
            self.redo.push(current);
        } else {
            self.undo.push(current);
        }
        (self.caret, self.anchor) = (caret.min(self.len()), anchor.min(self.len()));
        self.committed = Some((self.text.clone(), self.caret, self.anchor));
        self.last_change = f64::NEG_INFINITY;
        true
    }

    fn changed(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.mask.is_empty() {
            let masked = apply_mask(&self.mask, &self.text);
            if masked != self.text {
                self.text = masked;
                let end = self.len();
                self.caret = end;
                self.anchor = end;
            }
        }
        self.remember(ctx.ui.time());
        // Já mostrou erro: confere de novo a cada tecla (o erro some ao corrigir).
        if !self.error.is_empty() && !self.rules.is_empty() {
            self.error = self.validate().unwrap_or_default();
        }
        self.assist(ctx.ui, true);
        // Crescendo com o texto: o layout mede de novo antes da próxima pintura.
        if self.base.auto_size() {
            ctx.invalidate();
        }
        if self.silent {
            return;
        }
        ctx.fire("on_change", EventArgs::Value(self.text.clone().into()));
    }

    /// Consulta o motor de texto: as palavras erradas e (se `typing`, com o cursor no fim
    /// de uma palavra) o resto da palavra sugerida.
    fn assist(&mut self, ui: &mut Ui, typing: bool) {
        self.completion.clear();
        let active = !self.password && !self.read_only && self.mask.is_empty();
        let Some(engine) = ui.services_mut().text_assist.as_mut().filter(|_| active) else {
            self.misspelled.clear();
            return;
        };
        let caret = byte_index(&self.text, self.caret);
        if self.spellcheck {
            // A palavra ainda sendo digitada (encostada no cursor) não é marcada.
            self.misspelled = engine
                .check(&self.text, &self.language)
                .into_iter()
                .map(|m| (m.start, m.end))
                .filter(|&(s, e)| !(typing && s <= caret && e == caret))
                .collect();
        } else {
            self.misspelled.clear();
        }
        // Completa só no fim do texto, sem seleção, com uma palavra começada.
        if self.autocomplete && typing && caret == self.text.len() && !self.has_selection() {
            let before = &self.text[..caret];
            let typed = aylis_core::textassist::word_before(before);
            if typed.chars().count() >= 2 {
                let lower = typed.to_lowercase();
                let best = engine
                    .complete(before, &self.language)
                    .into_iter()
                    .find(|w| w.chars().count() > typed.chars().count() && w.to_lowercase().starts_with(&lower));
                if let Some(word) = best {
                    self.completion = word.chars().skip(typed.chars().count()).collect();
                }
            }
        }
    }

    /// Aceita a palavra sugerida (Tab ou seta para a direita).
    fn accept_completion(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        if self.completion.is_empty() {
            return false;
        }
        let rest = std::mem::take(&mut self.completion);
        if self.insert(&rest) {
            self.changed(ctx);
            // Acabou de aceitar: não sugere de novo em cima da palavra completa.
            self.completion.clear();
        }
        true
    }

    /// O trecho (em bytes) da palavra errada sob a posição `index` (em caracteres).
    fn misspelled_at(&self, index: usize) -> Option<(usize, usize)> {
        let at = byte_index(&self.text, index);
        self.misspelled.iter().copied().find(|&(s, e)| s <= at && at <= e)
    }

    /// O menu sobre uma palavra errada: as correções e "Adicionar ao dicionário".
    fn spelling_items(&self, ctx: &mut EventCtx<'_>, menu: ComponentId, (start, end): (usize, usize)) {
        let id = ctx.id;
        let word = self.text[start..end].to_string();
        let replace = move |ui: &mut Ui, choice: &str| {
            let (text, grows) = match ui.get_mut::<Edit>(id) {
                Some(edit) => {
                    edit.replace_range(start, end, choice);
                    (edit.text.clone(), edit.base.auto_size())
                }
                None => return,
            };
            if grows {
                ui.invalidate_form(id);
            }
            ui.fire(id, "on_change", &EventArgs::Value(text.into()));
        };
        let learned = move |ui: &mut Ui| {
            if let Some(edit) = ui.get_mut::<Edit>(id) {
                edit.misspelled.retain(|&r| r != (start, end));
            }
        };
        add_spelling_items(ctx, menu, &word, &self.language, replace, learned);
    }

    /// A palavra (até o espaço) em volta do caractere `index`: o duplo clique e o toque longo.
    fn word_range(&self, index: usize) -> (usize, usize) {
        let start = self.prev_word((index + 1).min(self.len()));
        let chars: Vec<char> = self.text.chars().collect();
        let mut end = start;
        while end < chars.len() && !chars[end].is_whitespace() {
            end += 1;
        }
        (start, end)
    }

    /// O menu de texto (com as correções em cima, sobre uma palavra errada). No toque, a
    /// palavra sob o dedo é selecionada, com as alças, e o menu abre embaixo delas.
    fn open_text_menu(&mut self, ctx: &mut EventCtx<'_>, at: PointF, touch: bool) -> bool {
        if !self.text_menu {
            return false;
        }
        self.completion.clear();
        self.dragging = false;
        let index = self.index_at(ctx, at.x);
        if touch && !self.text.is_empty() {
            let (start, end) = self.word_range(index);
            self.select(start, end);
        }
        self.touch_select = touch && self.has_selection();
        let menu = crate::text_menu::menu_slot(ctx, &mut self.spell_menu);
        if self.spellcheck
            && let Some(range) = self.misspelled_at(index)
        {
            self.spelling_items(ctx, menu, range);
            ctx.ui.add(crate::MenuItem::new("-"), menu);
        }
        let what = crate::text_menu::TextMenu { selected: self.has_selection(), has_text: !self.text.is_empty(), read_only: self.read_only, password: self.password };
        crate::text_menu::add_text_items(ctx.ui, menu, ctx.id, what, Self::menu_closed);
        // No toque, embaixo das alças (o dedo não cobre o menu nem o texto); no mouse, no clique.
        let at = if touch {
            let x = if self.has_selection() { self.x_of(ctx, self.selection().0) } else { at.x };
            PointF::new(x, self.line_bottom.get() + crate::text_menu::BELOW_HANDLES)
        } else {
            at
        };
        self.menu_open = true;
        crate::text_menu::popup(ctx, menu, at, Self::menu_closed);
        ctx.repaint();
        true
    }

    /// O menu de texto fechou: sem o foco, a seleção some (como ao sair do campo).
    fn menu_closed(ui: &mut Ui, id: ComponentId) {
        let focused = ui.focused(ui.root_of(id)) == Some(id);
        if let Some(edit) = ui.get_mut::<Edit>(id) {
            edit.menu_open = false;
            if !focused {
                edit.anchor = edit.caret;
                edit.touch_select = false;
            }
        }
    }

    /// Onde fica (x local) o começo do caractere `n`, como na última pintura.
    fn x_of(&self, ctx: &EventCtx<'_>, n: usize) -> f32 {
        let style = self.text_style.borrow().clone();
        let display = self.display();
        self.text_left.get() - self.scroll.get() + ctx.ui.measure_text(&display[..byte_index(&display, n)], &style).width
    }

    /// Troca o trecho `start..end` (bytes) por `text` (a correção escolhida no menu).
    fn replace_range(&mut self, start: usize, end: usize, text: &str) {
        if end > self.text.len() || start > end || !self.text.is_char_boundary(start) || !self.text.is_char_boundary(end) {
            return;
        }
        self.text.replace_range(start..end, text);
        let caret = self.text[..start + text.len()].chars().count();
        self.caret = caret;
        self.anchor = caret;
        self.misspelled.retain(|&(s, _)| s < start);
    }

    fn activity(&mut self, ctx: &mut EventCtx<'_>) {
        self.blink_start = ctx.ui.time();
        if (self.caret, self.anchor) != self.last_selection {
            self.last_selection = (self.caret, self.anchor);
            ctx.fire("on_caret_move", EventArgs::None);
        }
        ctx.repaint();
    }

    /// Onde a seleção começa (em caracteres). Gravar põe o cursor ali,
    /// sem seleção.
    pub fn sel_start(&self) -> usize {
        self.selection().0
    }

    pub fn set_sel_start(&mut self, start: usize) {
        let at = start.min(self.len());
        self.caret = at;
        self.anchor = at;
    }

    /// Quantos caracteres estão selecionados. Gravar seleciona a
    /// partir do `sel_start`.
    pub fn sel_length(&self) -> usize {
        let (a, b) = self.selection();
        b - a
    }

    pub fn set_sel_length(&mut self, length: usize) {
        let start = self.sel_start();
        self.anchor = start;
        self.caret = (start + length).min(self.len());
    }

    /// O texto selecionado. Gravar troca a seleção (ou insere no
    /// cursor) e dispara `on_change`; vale mesmo com `read_only` (só o usuário é barrado).
    pub fn sel_text(&self) -> String {
        self.selected_text().to_string()
    }

    pub fn set_sel_text(&mut self, text: String) {
        let read_only = std::mem::replace(&mut self.read_only, false);
        self.insert(&text);
        self.read_only = read_only;
    }

    /// Insere `text` no cursor (trocando a seleção), como se fosse digitado.
    pub fn insert_text(&mut self, text: &str) {
        self.set_sel_text(text.to_string());
    }

    fn copy(&self, ctx: &mut EventCtx<'_>) {
        if self.password || !self.has_selection() {
            return;
        }
        let text = self.selected_text().to_string();
        aylis_core::clipboard::set_text(ctx.ui, &text);
    }
}

/// As correções de uma palavra errada no menu do campo (Edit e Memo): as sugestões do motor
/// de texto, um divisor e "Adicionar ao dicionário" (o menu de texto vem depois delas).
/// `replace` troca a palavra pela escolhida; `learned` tira o sublinhado.
pub(crate) fn add_spelling_items(
    ctx: &mut EventCtx<'_>,
    menu: ComponentId,
    word: &str,
    language: &str,
    replace: impl Fn(&mut Ui, &str) + Clone + 'static,
    learned: impl Fn(&mut Ui) + 'static,
) {
    let suggestions = ctx.ui.services_mut().text_assist.as_mut().map(|e| e.suggest(word, language)).unwrap_or_default();
    if suggestions.is_empty() {
        let none = ctx.ui.add(crate::MenuItem::new(t("(sem sugestões)")), menu);
        let _ = ctx.ui.set_prop(none, "enabled", false);
    }
    for s in suggestions.into_iter().take(6) {
        let item = ctx.ui.add(crate::MenuItem::new(s.clone()), menu);
        let replace = replace.clone();
        ctx.ui.on_click(item, move |ui, _| {
            replace(ui, &s);
            ui.request_repaint();
        });
    }
    ctx.ui.add(crate::MenuItem::new("-"), menu);
    let learn = ctx.ui.add(crate::MenuItem::new(t("Adicionar ao dicionário")), menu);
    let (word, language) = (word.to_string(), language.to_string());
    ctx.ui.on_click(learn, move |ui, _| {
        if let Some(engine) = ui.services_mut().text_assist.as_mut() {
            engine.add_word(&word, &language);
        }
        learned(ui);
        ui.request_repaint();
    });
}

/// Formato do cursor de texto.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CaretStyle {
    /// A barra vertical de sempre.
    #[default]
    Line,
    /// Um bloco sobre o caractere (como nos terminais).
    Block,
    /// Um traço embaixo do caractere.
    Underline,
}

impl aylis_core::PropEnum for CaretStyle {
    const NAMES: &'static [&'static str] = &["line", "block", "underline"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Line, Self::Block, Self::Underline].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Desenha o cursor em `x` (o começo do caractere), na faixa `y..y + height`; `char_w` é a
/// largura do caractere sob ele (bloco e sublinhado).
/// `halo`: um contorno por baixo (o campo sem fundo, sobre uma imagem: veja [`caret_halo`]).
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_caret(canvas: &mut dyn Canvas, style: CaretStyle, x: f32, y: f32, height: f32, char_w: f32, width: f32, color: Color, halo: Option<Color>) {
    match style {
        CaretStyle::Line => {
            let x = x.round() + 0.5;
            if let Some(h) = halo {
                canvas.draw_line(PointF::new(x, y - 1.0), PointF::new(x, y + height + 1.0), &Stroke::new(h, width.max(0.5) + 2.0));
            }
            canvas.draw_line(PointF::new(x, y), PointF::new(x, y + height), &Stroke::new(color, width.max(0.5)));
        }
        CaretStyle::Block => {
            let r = RectF::new(x, y, char_w.max(2.0), height);
            if let Some(h) = halo {
                canvas.stroke_rect(r.inflate(0.5), 1.0, &Stroke::new(h, 1.0));
            }
            canvas.fill_rect(r, 1.0, &Brush::Solid(color.with_opacity(0.45)));
        }
        CaretStyle::Underline => {
            let h = width.max(1.0);
            let r = RectF::new(x, y + height - h, char_w.max(2.0), h);
            if let Some(c) = halo {
                canvas.fill_rect(r.inflate(1.0), 0.0, &Brush::Solid(c));
            }
            canvas.fill_rect(r, 0.0, &Brush::Solid(color));
        }
    }
}

/// O lugar do cursor depois da última letra, no `auto_size` do Edit e do Memo (a largura
/// dele e um pixel): a caixa medida cabe o texto e o cursor no fim, sem rolar.
pub(crate) fn caret_room(caret_width: f32) -> f32 {
    caret_width.max(0.5).ceil() + 1.0
}

/// O fundo deixa ver o que está atrás (o `plain` do Edit e do Memo, um `fill`
/// transparente): a seleção e o cursor se ajustam para aparecer sobre qualquer cor.
pub(crate) fn see_through(fill: Color) -> bool {
    fill.a < 128
}

/// A cor da seleção: a do tema, translúcida sobre um fundo que deixa ver o que está atrás
/// (os temas clássicos têm a seleção opaca, que cobriria a imagem).
pub(crate) fn selection_color(selection: Color, see_through: bool) -> Color {
    if see_through && selection.a > 160 { selection.with_opacity(0.4) } else { selection }
}

/// O contorno do cursor sem fundo: a cor do fundo do tema ou a do texto, a que mais se
/// afasta da do cursor (o cursor escuro ganha a borda clara, e o contrário), meio
/// translúcida. Assim ele aparece sobre o claro e sobre o escuro.
pub(crate) fn caret_halo(theme: &aylis_core::Theme, caret: Color) -> Color {
    let luma = |c: Color| 0.2126 * c.r as f32 + 0.7152 * c.g as f32 + 0.0722 * c.b as f32;
    let (back, text) = (theme.color("background"), theme.color("text"));
    let far = if (luma(back) - luma(caret)).abs() >= (luma(text) - luma(caret)).abs() { back } else { text };
    far.with_opacity(0.85)
}

/// O sublinhado ondulado das palavras erradas, de `x1` a `x2` na altura `y`.
pub(crate) fn draw_wave(canvas: &mut dyn Canvas, x1: f32, x2: f32, y: f32, stroke: &Stroke) {
    let step = 2.0;
    let mut x = x1;
    let mut up = true;
    while x < x2 {
        let nx = (x + step).min(x2);
        let (y0, y1) = if up { (y + 1.0, y - 1.0) } else { (y - 1.0, y + 1.0) };
        canvas.draw_line(PointF::new(x, y0), PointF::new(nx, y1), stroke);
        x = nx;
        up = !up;
    }
}

fn byte_index(text: &str, chars: usize) -> usize {
    text.char_indices().nth(chars).map_or(text.len(), |(b, _)| b)
}

impl Component for Edit {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    leaf_control!("Edit");

    fn accepts_text(&self) -> bool {
        true
    }

    // As teclas de edição ficam no campo: não viram o Tab do form (`tab_keys`).
    fn captures_key(&self, event: &KeyEvent) -> bool {
        !event.modifiers.ctrl && matches!(event.key, Key::Left | Key::Right | Key::Home | Key::End | Key::Backspace | Key::Delete)
    }

    fn context_menu(&mut self, ctx: &mut EventCtx<'_>, at: PointF, touch: bool) -> bool {
        self.open_text_menu(ctx, at, touch)
    }

    // As alças da seleção pelo toque ficam embaixo do texto, fora do campo: o dedo as pega.
    fn hit_slop(&self) -> f32 {
        if self.handles.get().is_some() { crate::text_menu::HANDLE_REACH } else { 0.0 }
    }

    fn contains_point(&self, local: PointF, size: aylis_core::SizeF) -> bool {
        let inside = local.x >= 0.0 && local.y >= 0.0 && local.x < size.width && local.y < size.height;
        inside || crate::text_menu::handle_at(self.handles.get(), local).is_some()
    }

    properties! {
        text: String => "Data",
        placeholder: String => "Appearance",
        border: bool => "Appearance",
        plain: bool => "Appearance",
        max_length: usize => "Behavior",
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        char_map: String => "Keyboard",
        char_case: aylis_core::CharCase => "Keyboard",
        read_only: bool => "Behavior",
        password: bool => "Behavior",
        sensitive: bool => "Behavior",
        auto_select: bool => "Behavior",
        h_align: TextAlign => "Text",
        v_align: TextAlign => "Text",
        wrap: bool => "Text",
        trimming: Trimming => "Text",
        icon: String => "Appearance",
        clear_button: bool => "Behavior",
        label: String => "Appearance",
        helper: String => "Appearance",
        error: String => "Data",
        counter: bool => "Appearance",
        prefix: String => "Appearance",
        suffix: String => "Appearance",
        mask: String => "Behavior",
        rules: String as List => "Behavior",
        spellcheck: bool => "Behavior",
        autocomplete: bool => "Behavior",
        language: String => "Behavior",
        keyboard_suggestions: bool => "Behavior",
        text_menu: bool => "Behavior",
        caret_color: Option<Color> => "Caret",
        caret_width: f32 => "Caret",
        caret_style: CaretStyle => "Caret",
        caret_blink: u32 as Duration => "Caret";
        events: on_change: String, on_caret_move: (), on_paste: String;
        computed: sel_start: usize => sel_start / set_sel_start,
            sel_length: usize => sel_length / set_sel_length,
            sel_text: String => sel_text / set_sel_text;
        on_set: after_set
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        self.start_history();
        // Do teclado (o físico ou o do menu): as alças do toque saem, menos no Ctrl+A e no
        // Ctrl+C (a seleção continua a mesma).
        if !(matches!(event.key, Key::Char('a' | 'c')) && (event.modifiers.ctrl || event.modifiers.meta)) {
            self.touch_select = false;
        }
        let shift = event.modifiers.shift;
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        // A palavra sugerida: Tab ou seta para a direita aceita, Esc dispensa.
        if !self.completion.is_empty() && !ctrl && !shift {
            match event.key {
                Key::Tab | Key::Right => {
                    self.accept_completion(ctx);
                    ctx.set_handled();
                    self.activity(ctx);
                    return;
                }
                Key::Escape => {
                    self.completion.clear();
                    ctx.set_handled();
                    self.activity(ctx);
                    return;
                }
                _ => self.completion.clear(),
            }
        }
        let len = self.len();
        let mut changed = false;
        match event.key {
            Key::Left => {
                let to = if self.has_selection() && !shift {
                    self.selection().0
                } else if ctrl {
                    self.prev_word(self.caret)
                } else {
                    self.caret.saturating_sub(1)
                };
                self.move_to(to, shift);
            }
            Key::Right => {
                let to = if self.has_selection() && !shift {
                    self.selection().1
                } else if ctrl {
                    self.next_word(self.caret)
                } else {
                    (self.caret + 1).min(len)
                };
                self.move_to(to, shift);
            }
            Key::Home => self.move_to(0, shift),
            Key::End => self.move_to(len, shift),
            Key::Backspace if !self.read_only => {
                changed = self.delete_selection() || {
                    let from = if ctrl { self.prev_word(self.caret) } else { self.caret.saturating_sub(1) };
                    self.delete_range(from, self.caret)
                };
            }
            Key::Delete if !self.read_only => {
                changed = self.delete_selection() || {
                    let to = if ctrl { self.next_word(self.caret) } else { (self.caret + 1).min(len) };
                    self.delete_range(self.caret, to)
                };
            }
            Key::Char('z') | Key::Char('Z') if ctrl && !self.read_only => changed = self.step(!shift),
            Key::Char('y') | Key::Char('Y') if ctrl && !self.read_only => changed = self.step(false),
            Key::Char('a') if ctrl => self.select_all(),
            Key::Char('c') if ctrl => self.copy(ctx),
            Key::Char('x') if ctrl => {
                if !self.password && !self.read_only {
                    self.copy(ctx);
                    changed = self.delete_selection();
                }
            }
            Key::Char('v') if ctrl => {
                let pasted = aylis_core::clipboard::text(ctx.ui);
                if let Some(text) = pasted {
                    // O colado passa pelos mesmos caracteres do digitado.
                    let text = ctx.ui.filter_text(ctx.id, Some(&self.char_filter()), &text);
                    changed = self.insert(&text);
                    // O que entrou pelo colar (o app reconhece a colagem sem contar linhas;
                    // C22 do Tabernáculo).
                    if changed {
                        ctx.fire("on_paste", EventArgs::Text(text));
                    }
                }
            }
            Key::Backspace | Key::Delete => {}
            _ => return,
        }
        ctx.set_handled();
        if changed {
            self.changed(ctx);
        }
        self.activity(ctx);
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.start_history();
        self.touch_select = false;
        if self.insert(text) {
            self.changed(ctx);
        }
        ctx.set_handled();
        self.activity(ctx);
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        // O dedo numa alça: arrasta aquela ponta da seleção (sem rolar o que está em volta).
        if event.kind == PointerKind::Touch
            && let Some(start) = crate::text_menu::handle_at(self.handles.get(), event.position)
        {
            let (a, b) = self.selection();
            (self.anchor, self.caret) = if start { (b, a) } else { (a, b) };
            let pan = self.base.pan_on_touch();
            self.base.set_pan_on_touch(false);
            self.handle_drag = Some((start, pan));
            ctx.set_handled();
            return;
        }
        self.touch_select = false;
        self.completion.clear();
        if self.clear_rect.get().is_some_and(|r| r.inflate(4.0).contains(event.position)) {
            self.text.clear();
            self.caret = 0;
            self.anchor = 0;
            self.changed(ctx);
            self.activity(ctx);
            return;
        }
        let index = self.index_at(ctx, event.position.x);
        match event.click_count {
            2 => {
                let (start, end) = self.word_range(index);
                self.select(start, end);
            }
            n if n >= 3 => self.select_all(),
            _ => {
                self.move_to(index, event.modifiers.shift);
                self.dragging = true;
            }
        }
        self.activity(ctx);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((start, _)) = self.handle_drag {
            let index = self.index_at(ctx, event.position.x);
            self.caret = crate::text_menu::drag_handle(self.anchor, index, start, self.len());
            self.activity(ctx);
            return;
        }
        if self.dragging {
            let index = self.index_at(ctx, event.position.x);
            self.move_to(index, true);
            self.activity(ctx);
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = false;
        if let Some((_, pan)) = self.handle_drag.take() {
            self.base.set_pan_on_touch(pan);
        }
    }

    fn focus_enter(&mut self, ctx: &mut EventCtx<'_>) {
        // Sem `auto_select`, o cursor fica onde estava (o relatório do Tabernáculo, C36). Voltando
        // do menu de texto, a seleção é a de antes dele.
        if self.auto_select && !self.menu_open {
            self.select_all();
        }
        self.activity(ctx);
    }

    // Gravar o `sel_text` por código muda o texto: avisa como uma edição do usuário.
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // O `char_case` vale também para o texto gravado pelo código (e ao mudar a regra).
        if matches!(name, "text" | "char_case") && self.char_case != aylis_core::CharCase::Normal {
            let cased = self.char_case.apply(&self.text);
            if cased != self.text {
                self.text = cased;
            }
        }
        if name == "sel_text" {
            self.changed(ctx);
        }
        // Texto vindo do form, do banco ou do código: o histórico do usuário recomeça.
        if name == "text" && self.committed.as_ref().is_some_and(|c| c.0 != self.text) {
            self.undo.clear();
            self.redo.clear();
            self.committed = None;
        }
        // Texto vindo do form, do banco ou do código: sublinha já (sem sugerir).
        if matches!(name, "text" | "spellcheck" | "language") {
            self.assist(ctx.ui, false);
            ctx.repaint();
        }
        if matches!(name, "sel_text" | "sel_start" | "sel_length") {
            self.activity(ctx);
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.completion.clear();
        self.dragging = false;
        ctx.repaint();
        // O foco foi para o menu de texto: a seleção fica (o item age sobre ela).
        if self.menu_open {
            return;
        }
        if !self.rules.is_empty() {
            self.error = self.validate().unwrap_or_default();
        }
        self.anchor = self.caret;
        self.touch_select = false;
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // Só o texto: sem o fundo do tema (o `fill` próprio vale), a borda e o padding.
        let plain_style;
        let s = if self.plain {
            let mut st = ctx.style.clone();
            st.fill = ctx.colors.fill.unwrap_or(Color::TRANSPARENT);
            st.border_width = 0.0;
            st.padding = aylis_core::Bounds::default();
            plain_style = st;
            &plain_style
        } else {
            ctx.style
        };
        let see_through = see_through(s.fill);
        let danger = ctx.theme.color("danger");
        // Ajuda, erro e contador ficam embaixo da caixa.
        let below = !self.helper.is_empty() || !self.error.is_empty() || self.counter;
        let mut boxr = ctx.rect;
        if below {
            boxr.height = (boxr.height - BELOW).max(16.0);
        }
        canvas.fill_rect(boxr, s.radius, &Brush::Solid(s.fill));
        let border = if self.error.is_empty() { s.border_color } else { danger };
        if self.border && s.border_width > 0.0 && !border.is_transparent() {
            // Vazio, o estilo `Edit.empty` do tema (se houver) troca o traço da borda: o
            // contorno tracejado do campo ainda não preenchido (C37 do Tabernáculo).
            let empty_dash = self.text.is_empty().then(|| ctx.theme.style("Edit.empty")).flatten().and_then(|st| st.base.border_style.as_deref()).map(|b| match b {
                "dashed" => vec![6.0, 4.0],
                "dotted" => vec![1.5, 3.0],
                _ => Vec::new(),
            });
            let mut stroke = s.border_stroke(border, s.border_width.max(if self.error.is_empty() { 0.0 } else { 1.5 }));
            if let Some(dash) = empty_dash {
                stroke = if dash.is_empty() { Stroke::new(border, stroke.width) } else { Stroke::new(border, stroke.width).dashed(dash) };
            }
            canvas.stroke_rect(boxr, s.radius, &stroke);
        }
        if self.border && !self.plain {
            crate::looks::paint_field_underline_in(canvas, ctx, boxr);
        }
        if below {
            let mut t = ctx.text_style();
            t.font.size = (t.font.size * 0.8).round();
            t.color = if self.error.is_empty() { ctx.theme.color("text_muted") } else { danger };
            let line = RectF::new(boxr.x + 4.0, boxr.bottom() + 2.0, boxr.width - 8.0, BELOW - 2.0);
            let message = if self.error.is_empty() { &self.helper } else { &self.error };
            canvas.draw_text(message, line, &t);
            if self.counter {
                let text = if self.max_length > 0 { format!("{}/{}", self.len(), self.max_length) } else { self.len().to_string() };
                t.h_align = TextAlign::Trailing;
                t.color = ctx.theme.color("text_muted");
                canvas.draw_text(&text, line, &t);
            }
        }
        // O rótulo: dentro do campo (como o placeholder) ou pequeno, sobre a borda.
        let floating = !self.label.is_empty() && (ctx.state.focused || !self.text.is_empty());
        if floating {
            let mut t = ctx.text_style();
            t.font.size = (t.font.size * 0.75).round();
            t.color = if !self.error.is_empty() {
                danger
            } else if ctx.state.focused {
                ctx.theme.color("primary")
            } else {
                ctx.theme.color("text_muted")
            };
            let w = canvas.measure_text(&self.label, &t).width + 8.0;
            if ctx.variant() == "filled" {
                // Campo preenchido: o rótulo fica dentro, em cima (o texto desce).
                let r = RectF::new(boxr.x + s.padding.left - 2.0, boxr.y + 4.0, w, t.font.size * 1.4);
                canvas.draw_text(&self.label, r, &t);
            } else {
                let r = RectF::new(boxr.x + 8.0, boxr.y - t.font.size * 0.7, w, t.font.size * 1.4);
                let back = if s.fill.a > 200 { s.fill } else { ctx.theme.color("background") };
                canvas.fill_rect(RectF::new(r.x, boxr.y - 1.0, r.width, 3.0), 0.0, &Brush::Solid(back));
                t.h_align = TextAlign::Center;
                canvas.draw_text(&self.label, r, &t);
            }
        }

        let mut inner = boxr.deflate(s.padding);
        if floating && ctx.variant() == "filled" {
            inner.y += 8.0;
            inner.height = (inner.height - 8.0).max(0.0);
        }
        inner.width = (inner.width - self.right_inset).max(0.0);
        let muted = ctx.theme.color("text_muted");
        let icon_side = (ctx.style.font.size * 1.15).round();
        if !self.icon.is_empty() {
            let r = RectF::new(inner.x, inner.y + (inner.height - icon_side) / 2.0, icon_side, icon_side);
            ctx.draw_icon(canvas, &self.icon, r, muted);
            inner.x += icon_side + 6.0;
            inner.width = (inner.width - icon_side - 6.0).max(0.0);
        }
        if self.clear_button && !self.text.is_empty() && !self.read_only {
            let side = (icon_side * 0.85).round();
            let r = RectF::new(inner.right() - side, inner.y + (inner.height - side) / 2.0, side, side);
            ctx.draw_icon(canvas, "close", r, muted);
            self.clear_rect.set(Some(r));
            inner.width = (inner.width - side - 4.0).max(0.0);
        } else {
            self.clear_rect.set(None);
        }
        let mut style = ctx.text_style();
        style.v_align = self.v_align;
        // Prefixo e sufixo, discretos (`R$ ... kg`): só com texto ou foco.
        if !(self.prefix.is_empty() && self.suffix.is_empty()) && (ctx.state.focused || !self.text.is_empty()) {
            let mut t = style.clone();
            t.color = muted;
            if !self.prefix.is_empty() {
                let w = canvas.measure_text(&self.prefix, &t).width + 4.0;
                canvas.draw_text(&self.prefix, RectF::new(inner.x, inner.y, w, inner.height), &t);
                inner.x += w;
                inner.width = (inner.width - w).max(0.0);
            }
            if !self.suffix.is_empty() {
                let w = canvas.measure_text(&self.suffix, &t).width + 4.0;
                t.h_align = TextAlign::Trailing;
                canvas.draw_text(&self.suffix, RectF::new(inner.right() - w, inner.y, w, inner.height), &t);
                inner.width = (inner.width - w).max(0.0);
            }
        }
        *self.text_style.borrow_mut() = style.clone();
        // Com o menu de texto aberto (ele tem o foco), o campo continua como na edição: a
        // seleção à vista, sem o cursor.
        let focused = ctx.state.focused || self.menu_open;
        let display = self.display();
        let width_of = |canvas: &mut dyn Canvas, n: usize| {
            canvas.measure_text(&display[..byte_index(&display, n)], &style).width
        };

        // Rola na horizontal para manter o cursor visível.
        let total = width_of(canvas, self.len());
        let caret_x = width_of(canvas, self.caret);
        let mut scroll = self.scroll.get();
        // Fora da edição o texto aparece do começo (com "…"): o clique que dá o foco
        // tem de ser medido nessa mesma posição.
        if total <= inner.width || !focused {
            scroll = 0.0;
        } else {
            if caret_x - scroll > inner.width - 2.0 {
                scroll = caret_x - inner.width + 2.0;
            }
            if caret_x - scroll < 0.0 {
                scroll = caret_x;
            }
            scroll = scroll.clamp(0.0, (total - inner.width + 2.0).max(0.0));
        }
        self.scroll.set(scroll);
        // Alinhamento horizontal: só quando o texto cabe (senão ele rola).
        let shift = if total <= inner.width {
            match self.h_align {
                TextAlign::Leading | TextAlign::Justify => 0.0,
                TextAlign::Center => (inner.width - total) / 2.0,
                TextAlign::Trailing => inner.width - total,
            }
        } else {
            0.0
        };
        self.text_left.set(inner.x + shift);
        let origin = inner.x + shift - scroll;
        // A faixa da linha (cursor, seleção), conforme o alinhamento vertical.
        let line_height = (style.font.size * 1.45).min(inner.height);
        let line_y = match self.v_align {
            TextAlign::Leading | TextAlign::Justify => inner.y,
            TextAlign::Center => inner.y + (inner.height - line_height) / 2.0,
            TextAlign::Trailing => inner.bottom() - line_height,
        };

        canvas.save();
        // Sem padding (`plain`), o recorte não passa da caixa (a pintura por parte não
        // repintaria o que saiu dela).
        let clip = inner.inflate(1.0);
        canvas.clip_rect(if self.plain { clip.intersect(boxr).unwrap_or(inner) } else { clip });
        let mut selected_rect = None;
        if focused && self.has_selection() {
            let (start, end) = self.selection();
            let (x1, x2) = (width_of(canvas, start), width_of(canvas, end));
            let rect = RectF::new(origin + x1, line_y + 1.0, x2 - x1, (line_height - 2.0).max(0.0));
            canvas.fill_rect(rect, s.radius.min(2.0), &Brush::Solid(selection_color(ctx.theme.color("selection"), see_through)));
            selected_rect = Some(rect);
        }
        let mut shown = style.clone();
        shown.h_align = self.h_align;
        if self.text.is_empty() {
            shown.color = placeholder_color(ctx.theme);
            shown.trimming = self.trimming;
            // Com rótulo: ele fica no lugar do placeholder até o campo ganhar o foco.
            let hint = if !self.label.is_empty() && !focused { &self.label } else { &self.placeholder };
            canvas.draw_text(&ctx.ui.tr(hint), inner, &shown);
        } else if !focused {
            // Fora da edição: quebra de linha e reticências como configurado.
            shown.wrap = self.wrap;
            shown.trimming = self.trimming;
            canvas.draw_text(&display, inner, &shown);
        } else {
            shown.h_align = TextAlign::Leading;
            shown.trimming = Trimming::None;
            canvas.draw_text(&display, RectF::new(origin, inner.y, total + 8.0, inner.height), &shown);
            // Tema com a cor do texto selecionado (o branco sobre o azul do Win32): o trecho
            // selecionado de novo, nessa cor.
            if let (Some(rect), Some(&color)) = (selected_rect.filter(|_| !see_through), ctx.theme.colors.get("selection_text")) {
                canvas.save();
                canvas.clip_rect(rect);
                let mut sel = shown.clone();
                sel.color = color;
                canvas.draw_text(&display, RectF::new(origin, inner.y, total + 8.0, inner.height), &sel);
                canvas.restore();
            }
            // A palavra sugerida, apagada, depois do cursor.
            if !self.completion.is_empty() {
                let mut ghost = shown.clone();
                ghost.color = ctx.theme.color("text_muted");
                let w = canvas.measure_text(&self.completion, &ghost).width + 8.0;
                canvas.draw_text(&self.completion, RectF::new(origin + caret_x, inner.y, w, inner.height), &ghost);
            }
        }
        // Palavras erradas: o sublinhado ondulado (medido como o texto).
        if !self.misspelled.is_empty() && !self.password && (focused || total <= inner.width) {
            let wave = Stroke::new(ctx.theme.color("danger"), 1.2);
            let y = line_y + line_height - 2.5;
            for &(start, end) in &self.misspelled {
                if end > self.text.len() || !self.text.is_char_boundary(start) || !self.text.is_char_boundary(end) {
                    continue;
                }
                let (a, b) = (self.text[..start].chars().count(), self.text[..end].chars().count());
                let (x1, x2) = (origin + width_of(canvas, a), origin + width_of(canvas, b));
                draw_wave(canvas, x1, x2, y, &wave);
            }
        }
        if ctx.state.focused && !ctx.state.disabled {
            // O cursor pisca (`caret_blink`); entre uma troca e outra a plataforma pode dormir.
            let blink = f64::from(self.caret_blink) / 1000.0;
            let phase = if blink > 0.0 { (ctx.time - self.blink_start).rem_euclid(blink) } else { 0.0 };
            let visible = blink <= 0.0 || phase < blink / 2.0;
            if visible {
                // A largura do caractere sob o cursor (bloco e sublinhado).
                let next_char = display.chars().nth(self.caret).filter(|c| !c.is_whitespace()).map(|c| c.to_string());
                let char_w = next_char.map_or(style.font.size * 0.55, |c| canvas.measure_text(&c, &style).width);
                let color = self.caret_color.unwrap_or(style.color);
                let halo = see_through.then(|| caret_halo(ctx.theme, color));
                draw_caret(canvas, self.caret_style, origin + caret_x, line_y + 2.0, line_height - 4.0, char_w, self.caret_width, color, halo);
            }
            if blink > 0.0 {
                let next = if visible { blink / 2.0 - phase } else { blink - phase };
                ctx.request_frame_at(ctx.time + next);
            }
        }
        canvas.restore();
        // As alças da seleção feita pelo toque, embaixo das pontas. Ficam fora do campo: por
        // cima de tudo do form (o vizinho de baixo não as cobre).
        let bottom = line_y + line_height;
        self.line_bottom.set(bottom);
        let handles = (focused && self.touch_select && self.has_selection() && !ctx.state.disabled).then(|| {
            let (start, end) = self.selection();
            let clamp = |x: f32| x.clamp(inner.x, inner.right());
            (clamp(origin + width_of(canvas, start)), clamp(origin + width_of(canvas, end)))
        });
        self.handles.set(handles.map(|(x1, x2)| crate::text_menu::handle_rects(x1, bottom, x2, bottom)));
        if let Some((x1, x2)) = handles {
            let color = ctx.theme.color("primary");
            ctx.overlay(move |c| {
                crate::text_menu::draw_handle(c, x1, bottom, color);
                crate::text_menu::draw_handle(c, x2, bottom, color);
            });
        }
    }
}

impl aylis_core::LayoutHooks for Edit {
    /// `auto_size`: o texto (ou, vazio, o placeholder), o ícone, o "×", o prefixo e o
    /// sufixo, o lugar do cursor e o padding; o campo acompanha o que se digita. Sem o
    /// `auto_size`, a largura desenhada.
    fn auto_width(&self, ui: &Ui, id: ComponentId) -> Option<f32> {
        if !self.base.auto_size() {
            return None;
        }
        let style = TextStyle { font: ui.font_of(id), ..Default::default() };
        let measure = |s: &str| ui.measure_text(s, &style).width;
        let display = self.display();
        let shown: &str = if !self.text.is_empty() {
            &display
        } else if self.placeholder.is_empty() {
            &self.label
        } else {
            &self.placeholder
        };
        let mut width = measure(shown);
        let icon_side = (style.font.size * 1.15).round();
        if !self.icon.is_empty() {
            width += icon_side + 6.0;
        }
        if self.clear_button && !self.read_only {
            width += (icon_side * 0.85).round() + 4.0;
        }
        for piece in [&self.prefix, &self.suffix] {
            if !piece.is_empty() {
                width += measure(piece) + 4.0;
            }
        }
        let pad = if self.plain { aylis_core::Bounds::default() } else { ui.style_padding(id) };
        Some((width + self.right_inset + caret_room(self.caret_width) + pad.horizontal()).ceil())
    }

    /// `auto_size`: uma linha e o padding (mais a ajuda, o erro e o contador embaixo).
    fn auto_height(&self, ui: &Ui, id: ComponentId, _width: f32) -> Option<f32> {
        if !self.base.auto_size() {
            return None;
        }
        let font = ui.font_of(id);
        let pad = if self.plain { aylis_core::Bounds::default() } else { ui.style_padding(id) };
        let below = !self.helper.is_empty() || !self.error.is_empty() || self.counter;
        Some((font.size * 1.45).ceil() + pad.vertical() + if below { BELOW } else { 0.0 })
    }
}

/// Altura do texto de ajuda/erro embaixo do campo.
/// Teclas com menos que isso (segundos) entre elas viram um passo só do desfazer.
const UNDO_PAUSE: f64 = 1.0;
const BELOW: f32 = 18.0;

impl Edit {
    /// Confere `rules`: `None` se está tudo certo, ou a mensagem do primeiro erro.
    pub fn validate(&self) -> Option<String> {
        check_rules(&self.rules, &self.text)
    }
}

/// Aplica a máscara: `#` dígito, `A` letra, `X` letra ou dígito, `*` qualquer; o resto é
/// literal e entra sozinho (só antes de um caractere digitado). O que não serve é
/// descartado.
pub fn apply_mask(mask: &str, text: &str) -> String {
    let slot = |m: char| matches!(m, '#' | 'A' | 'X' | '*');
    let fits = |m: char, c: char| match m {
        '#' => c.is_ascii_digit(),
        'A' => c.is_alphabetic(),
        'X' => c.is_alphanumeric(),
        _ => !c.is_control(),
    };
    let literals: Vec<char> = mask.chars().filter(|m| !slot(*m)).collect();
    let mut raw = text.chars().filter(|c| !literals.contains(c) || mask.contains('*')).peekable();
    let mut out = String::new();
    let mut pending = String::new();
    for m in mask.chars() {
        if !slot(m) {
            pending.push(m);
            continue;
        }
        // Acha o próximo caractere que serve nesta posição.
        let next = loop {
            match raw.next() {
                Some(c) if fits(m, c) => break Some(c),
                Some(_) => continue,
                None => break None,
            }
        };
        let Some(c) = next else { break };
        out.push_str(&pending);
        pending.clear();
        out.push(c);
    }
    out
}

/// Confere as regras (`required;email;min:3;max:20;number;cpf`). `None` = válido.
pub fn check_rules(rules: &str, text: &str) -> Option<String> {
    let value = text.trim();
    for rule in rules.split(';').map(str::trim).filter(|r| !r.is_empty()) {
        let (name, arg) = rule.split_once(':').map_or((rule, ""), |(n, a)| (n.trim(), a.trim()));
        let n: usize = arg.parse().unwrap_or(0);
        let failed = match name {
            "required" => value.is_empty(),
            _ if value.is_empty() => false,
            "email" => {
                let mut parts = value.split('@');
                let (user, domain) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
                user.is_empty() || !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') || parts.next().is_some() || value.contains(' ')
            }
            "min" => value.chars().count() < n,
            "max" => value.chars().count() > n,
            "number" => value.replace(',', ".").parse::<f64>().is_err(),
            "cpf" => !valid_cpf(value),
            _ => false,
        };
        if failed {
            return Some(match name {
                "required" => t("Obrigatório"),
                "email" => t("E-mail inválido"),
                "min" => tf("Mínimo de {0} caracteres", &[&n]),
                "max" => tf("Máximo de {0} caracteres", &[&n]),
                "number" => t("Digite um número"),
                "cpf" => t("CPF inválido"),
                _ => t("Inválido"),
            });
        }
    }
    None
}

fn valid_cpf(text: &str) -> bool {
    let d: Vec<u32> = text.chars().filter_map(|c| c.to_digit(10)).collect();
    if d.len() != 11 || d.iter().all(|x| *x == d[0]) {
        return false;
    }
    let digit = |len: usize| {
        let sum: u32 = (0..len).map(|i| d[i] * (len as u32 + 1 - i as u32)).sum();
        let r = sum * 10 % 11;
        if r == 10 { 0 } else { r }
    };
    digit(9) == d[9] && digit(10) == d[10]
}

/// Confere as `rules` de todos os campos dentro de `root` (um form, um painel) e mostra
/// os erros, como o `validate()` do `v-form`. Devolve `true` se está tudo certo.
pub fn validate_form(ui: &mut aylis_core::Ui, root: aylis_core::ComponentId) -> bool {
    let mut ok = true;
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        stack.extend(ui.children(id).iter().copied());
        if let Some(edit) = ui.get_mut::<Edit>(id) {
            if !edit.rules.is_empty() {
                edit.error = edit.validate().unwrap_or_default();
                ok &= edit.error.is_empty();
            }
        }
    }
    ui.request_repaint();
    ok
}

#[cfg(test)]
mod rule_tests {
    use super::*;

    #[test]
    fn masks_and_rules() {
        assert_eq!(apply_mask("(##) #####-####", "11987654321"), "(11) 98765-4321");
        assert_eq!(apply_mask("(##) #####-####", "119"), "(11) 9");
        assert_eq!(apply_mask("###.###.###-##", "123.456.789-0"), "123.456.789-0");
        assert_eq!(apply_mask("AAA-#A##", "abc1d23"), "abc-1d23");
        assert_eq!(check_rules("required", " "), Some("Obrigatório".into()));
        assert_eq!(check_rules("email", "eu@aylis.dev"), None);
        assert!(check_rules("email", "eu@aylis").is_some());
        assert!(check_rules("min:3", "ab").is_some());
        assert_eq!(check_rules("min:3;number", ""), None, "vazio só falha com required");
        assert_eq!(check_rules("cpf", "529.982.247-25"), None);
        assert!(check_rules("cpf", "111.111.111-11").is_some());
    }
}

impl Edit {
    /// O filtro de caracteres do próprio campo (para o colar, quando ele está ocupado).
    fn char_filter(&self) -> aylis_core::CharFilter {
        aylis_core::CharFilter {
            allowed: self.allowed_chars.clone(),
            blocked: self.blocked_chars.clone(),
            only_allowed: self.only_allowed,
            overrides: false,
            map: self.char_map.clone(),
            case: self.char_case,
        }
    }
}

/// A cor do texto de exemplo (`placeholder`) dos campos: a `placeholder` do tema, se ele
/// tiver (um tema que imita outro visual, com o exemplo mais apagado que o texto secundário),
/// ou a `text_muted`.
pub(crate) fn placeholder_color(theme: &aylis_core::Theme) -> aylis_core::Color {
    theme.colors.get("placeholder").copied().unwrap_or_else(|| theme.color("text_muted"))
}
