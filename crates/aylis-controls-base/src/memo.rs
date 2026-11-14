//! Memo: texto de várias linhas.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use aylis_core::{
    Brush, Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, TextStyle, Trimming, WheelEvent, properties,
};

use crate::leaf_control;
use crate::syntax::{self, Span};

/// Texto de várias linhas.
///
/// - Enter quebra a linha (Tab continua passando o foco, como o `WantTabs = False`);
/// - setas, Home/End (da linha; com Ctrl, do texto), PageUp/PageDown, Ctrl+A/C/X/V;
/// - mouse: clique posiciona, arrastar seleciona, duplo clique seleciona a palavra;
/// - a roda (e o dedo) rola; `on_change` a cada alteração; `on_caret_move` quando o
///   cursor anda (teclado ou mouse).
///
/// Texto: `wrap` quebra as linhas longas na largura do
/// campo (também durante a edição: setas, Home/End e o mouse seguem as linhas quebradas);
/// sem `wrap`, o texto rola na horizontal e, fora da edição, `trimming` põe "…" nas linhas
/// que não cabem. `h_align` alinha cada linha; o `justify` (com `wrap`, fora da edição)
/// espalha as linhas quebradas na largura toda.
///
/// Para código: `syntax` (`rust`, `python`, `json`) colore o texto em fonte monoespaçada
/// (sem quebra automática) e o Enter mantém o recuo da linha (mais um nível depois de `{`,
/// `(`, `[` ou `:`); `line_numbers` mostra os números das linhas; `want_tabs` faz o Tab
/// recuar 4 espaços (Shift+Tab volta) em vez de passar o foco.
///
/// Para um depurador: [`Memo::set_breakpoints`] desenha os pontos de parada na calha dos
/// números, [`Memo::set_exec_line`] destaca a linha onde o programa parou, e um clique na
/// calha dispara `on_gutter_click` com a linha (a partir de 1).
pub struct Memo {
    pub base: ControlBase,
    /// O texto todo, com as linhas separadas por quebra de linha.
    pub text: String,
    /// O texto apagado que aparece com o campo vazio.
    pub placeholder: String,
    /// A borda da caixa (desligada: só o texto, sem o contorno nem o traço do foco; dentro de
    /// um painel ou numa tabela). Ligada por padrão.
    pub border: bool,
    /// Só leitura: dá para selecionar e copiar, mas não editar.
    pub read_only: bool,
    /// Dado pessoal (LGPD): as gravações do `EventRecorder` mascaram o que é digitado aqui,
    /// a não ser com o `record_sensitive` dele ligado. Liga e desliga em tempo de execução.
    pub sensitive: bool,
    /// Limite de caracteres; 0 = sem limite.
    pub max_length: usize,
    /// Os caracteres liberados (vencem um bloqueio do form ou de um painel), os bloqueados
    /// e se só os liberados valem (`allowed_chars = "0-9"` com `only_allowed`: só números).
    /// Faixas como `a-z`; `-` é o hífen. Vale para o digitado e o colado.
    pub allowed_chars: String,
    /// Os caracteres que não entram (faixas como em `allowed_chars`).
    pub blocked_chars: String,
    /// Só entram os caracteres de `allowed_chars`.
    pub only_allowed: bool,
    /// Trocas do que o teclado produz: pares "digitado, entregue" (`",."` troca a vírgula por
    /// ponto), ao digitar e ao colar; vale também o de um contêiner (veja o `Form`).
    pub char_map: String,
    /// Maiúsculas ou minúsculas no valor: ao digitar, ao colar e quando
    /// o código grava o `text`. Ligado a um campo, manda o `char_case` do campo.
    pub char_case: aylis_core::CharCase,
    /// O alinhamento do texto na linha (esquerda, centro, direita).
    pub h_align: TextAlign,
    /// Quebra automática de linha.
    pub wrap: bool,
    /// Reticências nas linhas que não cabem (sem `wrap`, fora da edição).
    pub trimming: Trimming,
    /// Linguagem do realce (vazio = texto comum).
    pub syntax: String,
    /// Mostra o número de cada linha à esquerda.
    pub line_numbers: bool,
    /// O Tab entra no texto (senão, passa o foco para o próximo controle).
    pub want_tabs: bool,
    /// Quantas linhas cada dente da roda do mouse rola (o touchpad acompanha o dedo). Com o
    /// Ctrl, cada dente rola uma página.
    pub wheel_lines: u32,
    /// Sublinha as palavras erradas; o clique direito numa delas mostra as correções.
    pub spellcheck: bool,
    /// Completa a palavra enquanto digita (Tab ou seta para a direita aceita).
    pub autocomplete: bool,
    /// Idioma da correção e das sugestões; vazio: o do sistema.
    pub language: String,
    /// Celular: a faixa de sugestões e a correção automática do teclado do sistema
    /// (desligue em códigos, logins, identificadores).
    pub keyboard_suggestions: bool,
    /// Cor do cursor; vazio: a cor do texto.
    pub caret_color: Option<aylis_core::Color>,
    /// A largura do cursor, em pontos.
    pub caret_width: f32,
    /// O formato do cursor: barra, bloco ou sublinhado.
    pub caret_style: crate::CaretStyle,
    /// Intervalo de piscar (ms); 0: não pisca.
    pub caret_blink: u32,
    /// Palavras erradas (trechos em caracteres do texto).
    misspelled: Vec<(usize, usize)>,
    /// O resto da palavra sugerida (depois do cursor).
    completion: String,
    spell_menu: Option<aylis_core::ComponentId>,
    /// Linhas com ponto de parada (a partir de 1) e a linha onde o programa parou.
    breakpoints: Vec<usize>,
    exec_line: Option<usize>,
    /// O texto no fim da linha onde parou (o tempo e a memória do passo, no depurador).
    exec_note: String,
    /// Marcadores (veja `memo_marks.rs`) e o texto da última vez (para eles acompanharem as
    /// linhas).
    bookmarks: Vec<marks::Bookmark>,
    marks_text: String,
    /// A chave do texto que os marcadores acompanham (cada edição daqui os anda direto; outra
    /// chave: o texto mudou por fora e eles se acertam comparando com `marks_text`).
    marks_key: Cell<u64>,
    /// Largura da calha na última pintura (para o clique).
    gutter_width: Cell<f32>,
    /// Realce já calculado: (hash do texto, linguagem, trechos de cada linha).
    highlight: RefCell<Option<(u64, String, Rc<Vec<Vec<Span>>>)>>,
    /// O realce retomado da linha editada (veja `memo_highlight.rs`), pela chave do texto que
    /// ele acompanha.
    incremental: RefCell<(u64, highlight::Highlight)>,
    /// O realce por gramática ainda não chegou ao fim do texto (pede outro quadro).
    highlight_pending: std::cell::Cell<bool>,
    caret: usize,
    anchor: usize,
    /// Posição horizontal desejada ao subir/descer (em pixels, a partir do início da linha).
    goal_x: Option<f32>,
    scroll: Cell<PointF>,
    /// O deslize da roda em andamento: de onde, para onde (o `y` da rolagem) e quando começou.
    glide: Cell<Option<(f32, f32, f64)>>,
    /// O cursor e o tamanho do texto na última pintura: a rolagem só vai atrás do cursor
    /// quando um dos dois muda (senão a roda do mouse era desfeita a cada repintura).
    followed: Cell<Option<(usize, usize)>>,
    /// `go_to_line` pediu para mostrar o cursor (rola mesmo sem o foco: a linha onde o
    /// depurador parou aparece com o foco no app depurado).
    reveal: Cell<bool>,
    /// Medidas da última pintura (para o mouse e as setas): origem do texto, altura e
    /// largura da área de texto.
    origin: Cell<PointF>,
    line_height: Cell<f32>,
    text_width: Cell<f32>,
    visible_lines: Cell<usize>,
    /// A altura da área de texto: a roda e o minimapa calculam o fim da rolagem com ela, a
    /// mesma conta da pintura (com as linhas inteiras à vista, a roda passava do fim, a
    /// pintura voltava e a imagem já copiada ficava deslocada).
    text_height: Cell<f32>,
    text_style: RefCell<TextStyle>,
    blink_start: f64,
    dragging: bool,
    /// A barra de rolagem da última pintura: o trilho, o polegar e a rolagem máxima
    /// (`None`: o texto cabe e não há barra).
    bar: Cell<Option<(RectF, RectF, f32)>>,
    /// A barra horizontal (linhas mais largas que o editor, sem a quebra automática).
    hbar: Cell<Option<(RectF, RectF, f32)>>,
    /// Arrastando o polegar de uma barra: se é a horizontal e onde ele foi pego (a
    /// distância do começo dele).
    bar_grab: Option<(bool, f32)>,
    /// A largura da linha mais larga, guardada pela fonte e pela chave do texto (medir tudo a
    /// cada pintura sairia caro num arquivo grande), e as linhas editadas desde então (medidas
    /// na próxima pintura: a mais larga só cresce até o texto ser trocado).
    widest: RefCell<(u64, u64, f32, Vec<usize>)>,
    /// O começo de cada linha do texto (em caracteres e em bytes), pelo hash do texto: a
    /// pintura acha a posição de um caractere sem contar o texto desde o começo.
    line_table: RefCell<(u64, Rc<Vec<(usize, usize)>>)>,
    /// As linhas visuais já montadas, pela chave do texto e pela das dobras e da largura
    /// (montar as milhares de um fonte a cada quadro e a cada evento da roda pesava). Uma
    /// edição troca só as da linha editada (sem quebra automática nem dobras).
    rows_cache: RefCell<(u64, u64, Rc<Vec<Row>>)>,
    /// Muda a cada edição do texto (a chave dos caches, sem passar o texto inteiro por um
    /// hash a cada quadro: num fonte de um milhão de linhas, isso sozinho levava dezenas de ms).
    text_rev: Cell<u64>,
    /// O par do colchete do cursor, pela chave do texto e pela posição (achar o par pode
    /// andar o arquivo inteiro: uma vez por mudança, não a cada quadro).
    bracket_cache: Cell<Option<(u64, usize, Option<(usize, usize)>)>>,
    /// A tabela do quadro sendo pintado (só durante a pintura: o texto não muda nela).
    painting: RefCell<Option<Rc<Vec<(usize, usize)>>>>,
    /// Os traços do minimapa já calculados (relativos ao canto da faixa), pela chave do texto,
    /// da faixa e das cores: só o cursor andando não refaz nada.
    minimap_strokes: RefCell<(u64, Rc<Vec<(RectF, aylis_core::Color)>>)>,
    bar_hover: Cell<bool>,
    /// Editor de código (o que o VS Code tem): pares que fecham, operações de linha,
    /// Ctrl+D, as ocorrências da palavra e os pedidos ao servidor de linguagem.
    pub code_editing: bool,
    /// Dobrar blocos (o `CodeEditor` liga): as linhas que abrem um bloco recolhido, com o
    /// texto delas (a dobra acompanha a linha quando o texto de cima muda).
    pub(crate) folding: bool,
    folds: RefCell<Vec<(usize, String)>>,
    /// O minimapa à direita (o `CodeEditor` liga) e onde ele ficou na última pintura (a
    /// faixa e os pixels por linha); arrastando nele.
    pub(crate) minimap: bool,
    /// O tema de cores do código (`Dark+`, `Monokai`...; vazio: segue o tema do app). O
    /// `CodeEditor` publica como `color_theme`.
    pub(crate) color_theme: String,
    minimap_rect: Cell<Option<(RectF, f32)>>,
    minimap_drag: bool,
    /// Os cursores além do principal (âncora, cursor), como no VS Code.
    extra: Vec<(usize, usize)>,
    // O desfazer: as diferenças de cada passo, o último tipo de mudança (para juntar
    // digitações) e a chave do texto depois da última edição guardada (outra chave: o texto
    // mudou por fora, e as posições guardadas não valem mais).
    undo_stack: Vec<code::Step>,
    redo_stack: Vec<code::Step>,
    last_edit: Option<(code::EditKind, f64, usize)>,
    history_key: u64,
    // A lista de sugestões: todas, as que casam com o digitado, onde começa a palavra, a
    // marcada, a primeira à vista e o retângulo da última pintura (para o clique).
    suggestions: Vec<code::Suggestion>,
    suggest_shown: Vec<usize>,
    suggest_start: usize,
    suggest_index: usize,
    suggest_first: Cell<usize>,
    suggest_rect: Cell<Option<(RectF, f32)>>,
    /// A dica (texto e o índice da palavra), a ajuda dos parâmetros e os erros.
    tip: Option<(String, usize)>,
    signature: Option<String>,
    diagnostics: Vec<code::Diagnostic>,
    /// Os trechos realçados pelo localizar.
    highlights: Vec<(usize, usize)>,
    /// A palavra (o começo dela) sob o mouse, para avisar só quando muda.
    hover_word: Option<usize>,
    /// Com Ctrl sobre um nome (código): o trecho sublinhado, com o cursor de mãozinha, que o
    /// Ctrl+clique leva à declaração (como no VS Code).
    link: Option<(usize, usize)>,
    /// Onde o mouse está (local), para o sublinhado aparecer ao apertar o Ctrl parado.
    pointer_at: Option<PointF>,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

impl Default for Memo {
    fn default() -> Self {
        let mut base = ControlBase::new(240.0, 120.0);
        base.set_can_focus(true).set_cursor(CursorShape::Text);
        Self { overscroll: aylis_core::Overscroll::Auto,
            base,
            text: String::new(),
            placeholder: String::new(),
            border: true,
            read_only: false,
            sensitive: false,
            max_length: 0,
            allowed_chars: String::new(),
            blocked_chars: String::new(),
            only_allowed: false,
            char_map: String::new(),
            char_case: aylis_core::CharCase::Normal,
            h_align: TextAlign::Leading,
            wrap: false,
            trimming: Trimming::Character,
            syntax: String::new(),
            line_numbers: false,
            want_tabs: false,
            wheel_lines: 8,
            spellcheck: false,
            autocomplete: false,
            language: String::new(),
            keyboard_suggestions: true,
            caret_color: None,
            caret_width: 1.5,
            caret_style: crate::CaretStyle::Line,
            caret_blink: 1000,
            misspelled: Vec::new(),
            completion: String::new(),
            spell_menu: None,
            highlight: RefCell::new(None),
            incremental: RefCell::default(),
            highlight_pending: std::cell::Cell::new(false),
            breakpoints: Vec::new(),
            bookmarks: Vec::new(),
            marks_text: String::new(),
            marks_key: Cell::new(0),
            exec_line: None,
            exec_note: String::new(),
            gutter_width: Cell::new(0.0),
            caret: 0,
            anchor: 0,
            goal_x: None,
            scroll: Cell::new(PointF::ZERO),
            glide: Cell::new(None),
            followed: Cell::new(None),
            reveal: Cell::new(false),
            bar: Cell::new(None),
            hbar: Cell::new(None),
            bar_grab: None,
            widest: RefCell::new((0, 0, 0.0, Vec::new())),
            line_table: RefCell::new((0, Rc::new(Vec::new()))),
            rows_cache: RefCell::new((0, 0, Rc::new(Vec::new()))),
            text_rev: Cell::new(0),
            bracket_cache: Cell::new(None),
            painting: RefCell::new(None),
            minimap_strokes: RefCell::new((0, Rc::new(Vec::new()))),
            bar_hover: Cell::new(false),
            origin: Cell::new(PointF::ZERO),
            line_height: Cell::new(20.0),
            text_width: Cell::new(200.0),
            visible_lines: Cell::new(5),
            text_height: Cell::new(0.0),
            text_style: RefCell::default(),
            blink_start: 0.0,
            dragging: false,
            code_editing: false,
            folding: false,
            folds: RefCell::new(Vec::new()),
            minimap: false,
            color_theme: String::new(),
            minimap_rect: Cell::new(None),
            minimap_drag: false,
            extra: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_edit: None,
            history_key: 0,
            suggestions: Vec::new(),
            suggest_shown: Vec::new(),
            suggest_start: 0,
            suggest_index: 0,
            suggest_first: Cell::new(0),
            suggest_rect: Cell::new(None),
            tip: None,
            signature: None,
            diagnostics: Vec::new(),
            highlights: Vec::new(),
            hover_word: None,
            link: None,
            pointer_at: None,
        }
    }
}

/// Quantas linhas depois das à vista a calha olha para achar o fim de um bloco (a seta de
/// dobrar): o bastante para qualquer região de verdade, sem ler o arquivo inteiro.
const FOLD_LOOKAHEAD: usize = 20_000;

/// O byte do caractere `chars` pela tabela das linhas: acha a linha e conta só dentro dela.
fn byte_in(text: &str, table: &[(usize, usize)], chars: usize) -> usize {
    let i = table.partition_point(|&(c, _)| c <= chars).saturating_sub(1);
    let (c0, b0) = table.get(i).copied().unwrap_or((0, 0));
    b0 + text[b0..].char_indices().nth(chars - c0).map_or(text.len() - b0, |(b, _)| b)
}

fn byte_index(text: &str, chars: usize) -> usize {
    text.char_indices().nth(chars).map_or(text.len(), |(b, _)| b)
}

/// Tabulação vira 4 espaços na tela.
fn expand_tabs(s: &str) -> std::borrow::Cow<'_, str> {
    if s.contains('\t') { s.replace('\t', "    ").into() } else { s.into() }
}

/// Uma linha visual: de `start` a `end` (em caracteres do texto todo, sem o `\n`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Row {
    start: usize,
    end: usize,
    /// A linha do texto (a partir de 0): com blocos dobrados, a linha visual não é a do
    /// texto.
    line: usize,
    /// Abre um bloco dobrado (o `⋯` no fim).
    folded: bool,
}

impl Memo {
    /// A rolagem foi de `before` para a de agora: o texto (com a calha dos números) anda e a
    /// plataforma copia o que já estava pintado; só a faixa que apareceu, a barra e o
    /// minimapa são pintados. Sem a pintura de antes (a barra ainda não medida) ou de lado,
    /// o controle inteiro.
    fn scrolled_from(&self, ctx: &mut EventCtx<'_>, before: PointF) {
        let now = self.scroll.get();
        let Some((track, _, _)) = self.bar.get() else {
            ctx.repaint();
            return;
        };
        if now.x != before.x {
            ctx.repaint();
            return;
        }
        let origin = self.origin.get();
        let left = origin.x - self.gutter_width.get();
        let content = RectF::new(left, track.y, (track.x - left).max(0.0), track.height);
        ctx.scrolled(content, PointF::new(0.0, before.y - now.y));
        ctx.repaint_part(track);
        // O texto é recortado um pixel além da área (veja a pintura): essa beirada também
        // andou e não está na cópia.
        ctx.repaint_part(RectF::new(content.x - 2.0, content.y - 2.0, content.width + 4.0, 3.0));
        ctx.repaint_part(RectF::new(content.x - 2.0, content.bottom() - 1.0, content.width + 4.0, 3.0));
        ctx.repaint_part(RectF::new(content.x - 2.0, content.y, 3.0, content.height));
        for part in [self.hbar.get().map(|(t, _, _)| t), self.minimap_rect.get().map(|(m, _)| m)].into_iter().flatten() {
            ctx.repaint_part(part);
        }
    }

    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// Até onde a rolagem vertical vai. No editor de código, até a última
    /// linha ficar no alto (quando o texto passa da altura); no `Memo` comum, até ela
    /// aparecer embaixo. Em pixels inteiros, como a rolagem copiada (o deslize arredonda):
    /// um fim fracionário arredondado para cima voltava meio pixel na pintura.
    fn scroll_max(&self, total: f32, view: f32, line_height: f32) -> f32 {
        if self.code_editing && total > view {
            (total - line_height).max(0.0).floor()
        } else {
            (total - view).max(0.0).floor()
        }
    }

    /// Leva o polegar da barra que está sendo arrastada até o mouse (no ponto em que foi
    /// pego) e rola junto.
    fn drag_bar(&mut self, ctx: &mut EventCtx<'_>, at: PointF) {
        let Some((horizontal, grab)) = self.bar_grab else { return };
        self.glide.set(None);
        let bar = if horizontal { self.hbar.get() } else { self.bar.get() };
        let Some((track, thumb, max)) = bar else { return };
        let mut s = self.scroll.get();
        if horizontal {
            let room = (track.width - thumb.width).max(1.0);
            s.x = ((at.x - grab - track.x) / room).clamp(0.0, 1.0) * max;
        } else {
            let room = (track.height - thumb.height).max(1.0);
            s.y = ((at.y - grab - track.y) / room).clamp(0.0, 1.0) * max;
        }
        self.scroll.set(s);
        ctx.repaint();
    }

    /// A barra sob o ponto: `Some(true)` a horizontal, `Some(false)` a vertical.
    fn bar_at(&self, p: PointF) -> Option<bool> {
        if self.bar.get().is_some_and(|(track, _, _)| track.inflate(2.0).contains(p)) {
            Some(false)
        } else if self.hbar.get().is_some_and(|(track, _, _)| track.inflate(2.0).contains(p)) {
            Some(true)
        } else {
            None
        }
    }

    /// As linhas do texto (separadas por Enter; sem considerar a quebra automática).
    pub fn lines(&self) -> Vec<&str> {
        self.text.split('\n').collect()
    }

    /// Acrescenta uma linha no fim (como o `Lines.Add`).
    pub fn add_line(&mut self, line: &str) {
        let end = self.len();
        let piece = if self.text.is_empty() { line.to_string() } else { format!("\n{line}") };
        self.splice(end, end, &piece);
        let end = self.len();
        self.caret = end;
        self.anchor = end;
    }

    pub fn selection(&self) -> (usize, usize) {
        (self.caret.min(self.anchor), self.caret.max(self.anchor))
    }

    pub fn selected_text(&self) -> &str {
        let (a, b) = self.selection();
        &self.text[self.byte_at(a)..self.byte_at(b)]
    }

    pub fn select_all(&mut self) {
        self.anchor = 0;
        self.caret = self.len();
    }

    /// Posição do cursor (linha, coluna), começando em 0 (linhas do texto, não as visuais).
    pub fn caret_position(&self) -> (usize, usize) {
        // Pela tabela das linhas: não conta o texto até o cursor (num fonte enorme, a cada
        // tecla).
        let line = self.line_of(self.caret);
        let table = self.lines_of(self.text_key());
        (line, self.caret.min(self.len()) - table[line].0)
    }

    /// A linha do texto (a partir de 0) do caractere `index`, pela tabela das linhas.
    fn line_of(&self, index: usize) -> usize {
        let table = self.lines_of(self.text_key());
        table.partition_point(|&(c, _)| c <= index).saturating_sub(1)
    }

    /// O tamanho em caracteres, pela tabela das linhas (só conta a última linha; contar o
    /// texto inteiro a cada quadro pesava num fonte enorme).
    fn len(&self) -> usize {
        let table = self.lines_of(self.text_key());
        let (chars, bytes) = table.last().copied().unwrap_or((0, 0));
        chars + self.text[bytes..].chars().count()
    }

    fn after_set(&mut self, name: &str) {
        if name == "text" {
            self.text_changed();
            let end = self.len();
            self.caret = end;
            self.anchor = end;
            self.clear_history();
            self.hide_suggestions();
            self.diagnostics.clear();
            self.folds.borrow_mut().clear();
            self.extra.clear();
        }
    }

    /// Texto entre duas posições (em caracteres).
    fn slice(&self, a: usize, b: usize) -> &str {
        if let Some(table) = self.painting.borrow().as_ref() {
            return &self.text[byte_in(&self.text, table, a)..byte_in(&self.text, table, b)];
        }
        &self.text[self.byte_at(a)..self.byte_at(b)]
    }

    /// A chave do texto para os caches da pintura, sem ler o texto (o tempo não cresce com o
    /// tamanho do fonte): a versão (cada edição daqui incrementa, [`Memo::text_changed`]),
    /// mais o endereço e o tamanho, que mudam quando o texto é trocado por fora (a
    /// propriedade, o código do app: um `String` novo).
    fn text_key(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (self.text_rev.get(), self.text.len(), self.text.as_ptr() as usize).hash(&mut h);
        h.finish()
    }

    /// O byte do caractere `index` pela tabela das linhas (busca binária e só a linha dele:
    /// não varre o texto desde o começo, o que num fonte enorme custava a cada quadro).
    fn byte_at(&self, index: usize) -> usize {
        let table = self.lines_of(self.text_key());
        byte_in(&self.text, &table, index)
    }

    /// O texto mudou aqui dentro (os caches que dependem dele são refeitos).
    fn text_changed(&self) {
        self.text_rev.set(self.text_rev.get() + 1);
    }

    /// A tabela das linhas do texto de chave `key` (refeita só quando o texto muda).
    fn lines_of(&self, key: u64) -> Rc<Vec<(usize, usize)>> {
        {
            let (k, table) = &*self.line_table.borrow();
            if *k == key && !table.is_empty() {
                return table.clone();
            }
        }
        let mut table = vec![(0usize, 0usize)];
        let mut chars = 0;
        for (b, c) in self.text.char_indices() {
            chars += 1;
            if c == '\n' {
                table.push((chars, b + 1));
            }
        }
        let table = Rc::new(table);
        *self.line_table.borrow_mut() = (key, table.clone());
        table
    }

    /// As linhas visuais, com a quebra automática se `wrap`.
    fn rows(&self, width: f32, measure: &mut dyn FnMut(&str) -> f32) -> Rc<Vec<Row>> {
        // Guardadas: só o texto, as dobras e (com a quebra automática) a largura e a fonte
        // mudam as linhas visuais.
        let text = self.text_key();
        let params = self.rows_params(width);
        {
            let (t, p, rows) = &*self.rows_cache.borrow();
            if *t == text && *p == params {
                return rows.clone();
            }
        }
        let rows = Rc::new(self.build_rows(width, measure));
        *self.rows_cache.borrow_mut() = (text, params, rows.clone());
        rows
    }

    /// A chave do que, além do texto, muda as linhas visuais: as dobras e (com a quebra
    /// automática) a largura e a fonte.
    fn rows_params(&self, width: f32) -> u64 {
        use std::hash::{Hash, Hasher};
        let wraps = self.wrap && self.syntax.is_empty() && width > 0.0;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (self.folding, &*self.folds.borrow(), wraps).hash(&mut h);
        if wraps {
            let style = self.text_style.borrow();
            (width.to_bits(), style.font.size.to_bits(), &style.font.family, style.font.weight as u16).hash(&mut h);
        }
        h.finish()
    }

    fn build_rows(&self, width: f32, measure: &mut dyn FnMut(&str) -> f32) -> Vec<Row> {
        let mut rows = Vec::new();
        let mut start = 0;
        let lines: Vec<&str> = self.text.split('\n').collect();
        // As linhas dentro de um bloco dobrado não aparecem.
        let hidden = if self.folding { self.hidden_lines(&lines) } else { Vec::new() };
        let folds = self.folds.borrow();
        for (number, line) in lines.iter().enumerate() {
            let n = line.chars().count();
            if hidden.get(number).copied().unwrap_or(false) {
                start += n + 1;
                continue;
            }
            if self.wrap && self.syntax.is_empty() && width > 0.0 {
                let mut m = |s: &str| measure(&expand_tabs(s));
                for (a, b) in aylis_core::text::wrap_ranges(line, width, &mut m) {
                    rows.push(Row { start: start + a, end: start + b, line: number, folded: false });
                }
            } else {
                let folded = !hidden.is_empty() && folds.iter().any(|(l, _)| *l == number);
                rows.push(Row { start, end: start + n, line: number, folded });
            }
            start += n + 1;
        }
        rows
    }

    /// Quanto o realce por gramática anda por quadro num arquivo grande.
    const HIGHLIGHT_BUDGET_MS: u64 = 8;

    /// A partir de quantos bytes o realce sem gramática vai aos poucos (abaixo, de uma vez:
    /// um fonte de 15 mil linhas leva poucos ms).
    const PROGRESSIVE_FROM: usize = 1 << 20;

    /// Os trechos coloridos de cada linha do texto de chave `key` (recalculados só quando o
    /// texto muda; a pintura já tem a chave).
    fn spans_keyed(&self, key: u64) -> Rc<Vec<Vec<Span>>> {
        if let Some((k, lang, spans)) = &*self.highlight.borrow() {
            if *k == key && *lang == self.syntax {
                return spans.clone();
            }
        }
        // Solta o realce anterior: o realce incremental edita os trechos no lugar.
        self.highlight.borrow_mut().take();
        let table = self.lines_of(key);
        let mut slot = self.incremental.borrow_mut();
        // O texto mudou por fora (ou a linguagem): começa do zero. As edições daqui acertam o
        // realce na hora (veja `splice`) e ele continua da linha editada.
        if slot.0 != key || !slot.1.is_for(&self.syntax) || slot.1.lines() != table.len() {
            *slot = (key, highlight::Highlight::start(&self.syntax, table.len()));
        }
        // Pela gramática ou num texto grande, alguns ms por quadro (de uma vez, um fonte de um
        // milhão de linhas travava mais de um segundo ao abrir); o resto vem nos próximos.
        let grammar = matches!(slot.1, highlight::Highlight::Grammar { .. });
        let budget = (grammar || self.text.len() > Self::PROGRESSIVE_FROM).then(|| std::time::Duration::from_millis(Self::HIGHLIGHT_BUDGET_MS));
        let text = self.text.as_str();
        let line = |i: usize| {
            let a = table[i].1;
            let b = table.get(i + 1).map_or(text.len(), |next| next.1 - 1);
            &text[a..b]
        };
        let (spans, done) = slot.1.resume(line, budget);
        self.highlight_pending.set(!done);
        if done {
            *self.highlight.borrow_mut() = Some((key, self.syntax.clone(), spans.clone()));
        }
        spans
    }

    /// Início da linha (do texto) onde está o índice `index`.
    fn line_start(&self, index: usize) -> usize {
        let table = self.lines_of(self.text_key());
        table[self.line_of(index)].0
    }

    /// Enter num código: a quebra e o recuo da linha (mais um nível depois de abrir um
    /// bloco).
    fn newline_with_indent(&self) -> String {
        let start = self.line_start(self.caret);
        let before = self.slice(start, self.caret).to_string();
        let mut indent: String = before.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
        let opens = match crate::textmate::language_config(&self.syntax).and_then(|c| c.increase_indent.clone()) {
            Some(pattern) => pattern.is_match(&before),
            None => before.trim_end().ends_with(['{', '(', '[', ':']),
        };
        if opens {
            indent.push_str("    ");
        }
        format!("\n{indent}")
    }

    /// Shift+Tab: tira até 4 espaços do começo da linha do cursor.
    fn unindent(&mut self) -> bool {
        let start = self.line_start(self.caret);
        let spaces = self.text[self.byte_at(start)..].chars().take(4).take_while(|c| *c == ' ').count();
        if spaces == 0 {
            return false;
        }
        let caret = self.caret.saturating_sub(spaces).max(start);
        self.delete_range(start, start + spaces);
        self.caret = caret;
        self.anchor = caret;
        true
    }

    /// As linhas visuais medidas com o `Ui` (nos eventos).
    fn rows_ui(&self, ctx: &EventCtx<'_>) -> Rc<Vec<Row>> {
        let style = self.text_style.borrow().clone();
        let mut measure = |s: &str| ctx.ui.measure_text(s, &style).width;
        self.rows(self.text_width.get(), &mut measure)
    }

    /// A linha visual do índice: a última que começa nele ou antes (numa quebra
    /// automática, o cursor fica no começo da linha de baixo).
    fn row_of(rows: &[Row], index: usize) -> usize {
        // Em ordem pelo começo: busca binária (andar a lista toda pesava num fonte enorme).
        rows.partition_point(|r| r.start <= index).saturating_sub(1)
    }

    /// Deslocamento horizontal de uma linha pelo alinhamento.
    fn shift(&self, row_width: f32) -> f32 {
        let free = self.text_width.get() - row_width;
        if free <= 0.0 {
            return 0.0;
        }
        match self.h_align {
            TextAlign::Leading | TextAlign::Justify => 0.0,
            TextAlign::Center => free / 2.0,
            TextAlign::Trailing => free,
        }
    }

    fn has_selection(&self) -> bool {
        self.caret != self.anchor
    }

    fn delete_range(&mut self, start: usize, end: usize) -> bool {
        if start >= end || self.read_only {
            return false;
        }
        self.splice(start, end, "");
        self.caret = start;
        self.anchor = start;
        true
    }

    /// Troca os caracteres `start..end` por `new` (o caminho de toda mudança do texto feita
    /// aqui dentro) e guarda a diferença no passo do desfazer em andamento.
    pub(super) fn splice(&mut self, start: usize, end: usize, new: &str) {
        let before = self.text_key();
        let (at, removed) = self.splice_raw(start, end, new);
        self.record(before, at, removed, new);
    }

    /// Troca os caracteres `start..end` por `new` sem passar pelo desfazer; devolve onde
    /// começou e o que saiu. O que depende do texto é acertado só da linha editada em
    /// diante (a tabela das linhas, as linhas visuais, o realce, a linha mais larga e os
    /// marcadores): as linhas de depois só andam. Num fonte de um milhão de linhas, refazer
    /// tudo isso a cada tecla levava centenas de ms.
    pub(super) fn splice_raw(&mut self, start: usize, end: usize, new: &str) -> (usize, String) {
        let len = self.len();
        let (start, end) = (start.min(len), end.min(len));
        let end = end.max(start);
        let old_key = self.text_key();
        let table = self.lines_of(old_key);
        let old_lines = table.len();
        let (a, b) = (byte_in(&self.text, &table, start), byte_in(&self.text, &table, end));
        let line_of = |c: usize| table.partition_point(|&(s, _)| s <= c).saturating_sub(1);
        let (la, lb) = (line_of(start), line_of(end));
        let line_end = |i: usize| table.get(i + 1).map_or(self.text.len(), |next| next.1 - 1);
        let new_nl = new.bytes().filter(|&c| c == b'\n').count();
        // As linhas de antes do trecho (os marcadores comparam com as de depois).
        let marks = !self.bookmarks.is_empty() && self.marks_key.get() == old_key && new_nl != lb - la;
        let old_local = marks.then(|| self.text[table[la].1..line_end(lb)].to_string());
        drop(table);
        let removed = self.text[a..b].to_string();
        let removed_chars = end - start;
        self.text.replace_range(a..b, new);
        self.text_changed();
        let new_key = self.text_key();
        let dc = new.chars().count() as isize - removed_chars as isize;
        let db = new.len() as isize - (b - a) as isize;
        let dl = new_nl as isize - (lb - la) as isize;
        let shift = |v: usize, d: isize| (v as isize + d) as usize;
        // A tabela das linhas: as quebras do trecho saem, as do novo entram, as de depois andam.
        let table = {
            let mut slot = self.line_table.borrow_mut();
            let t = Rc::make_mut(&mut slot.1);
            let (mut c, mut bytes) = (start, a);
            let mut breaks = Vec::with_capacity(new_nl);
            for ch in new.chars() {
                c += 1;
                bytes += ch.len_utf8();
                if ch == '\n' {
                    breaks.push((c, bytes));
                }
            }
            t.splice(la + 1..lb + 1, breaks);
            for e in &mut t[la + 1 + new_nl..] {
                *e = (shift(e.0, dc), shift(e.1, db));
            }
            slot.0 = new_key;
            slot.1.clone()
        };
        let new_len = table.last().map_or(0, |&(c, b)| c + self.text[b..].chars().count());
        // As linhas visuais (uma por linha do texto: sem quebra automática nem dobras).
        {
            let mut slot = self.rows_cache.borrow_mut();
            let plain = !(self.wrap && self.syntax.is_empty()) && self.folds.borrow().is_empty();
            if slot.0 == old_key && plain && slot.2.len() == old_lines {
                let rows = Rc::make_mut(&mut slot.2);
                let fresh = (la..=la + new_nl).map(|line| {
                    let start = table[line].0;
                    let end = table.get(line + 1).map_or(new_len, |next| next.0 - 1);
                    Row { start, end, line, folded: false }
                });
                rows.splice(la..=lb, fresh);
                for r in &mut rows[la + 1 + new_nl..] {
                    r.start = shift(r.start, dc);
                    r.end = shift(r.end, dc);
                    r.line = shift(r.line, dl);
                }
                slot.0 = new_key;
            }
        }
        // O realce continua da linha editada.
        self.highlight.borrow_mut().take();
        {
            let mut slot = self.incremental.borrow_mut();
            if slot.0 == old_key && slot.1.lines() == old_lines {
                slot.1.edit(la, lb + 1, new_nl + 1);
                slot.0 = new_key;
            }
        }
        // A linha mais larga: as editadas são medidas na próxima pintura.
        {
            let mut w = self.widest.borrow_mut();
            if w.1 == old_key && w.3.len() < 64 {
                for p in &mut w.3 {
                    *p = if *p > lb { shift(*p, dl) } else { (*p).min(la) };
                }
                w.3.extend(la..=la + new_nl);
                w.3.sort_unstable();
                w.3.dedup();
                w.1 = new_key;
            }
        }
        // Os marcadores: comparam só as linhas do trecho, pelas pontas.
        if let Some(old_local) = old_local {
            let new_local = &self.text[table[la].1..table.get(la + new_nl + 1).map_or(self.text.len(), |next| next.1 - 1)];
            let (old_l, new_l): (Vec<&str>, Vec<&str>) = (old_local.split('\n').collect(), new_local.split('\n').collect());
            let prefix = old_l.iter().zip(&new_l).take_while(|(x, y)| x == y).count();
            let max_suffix = old_l.len().min(new_l.len()) - prefix;
            let suffix = old_l.iter().rev().zip(new_l.iter().rev()).take(max_suffix).take_while(|(x, y)| x == y).count();
            self.move_marks(la + prefix, lb + 1 - suffix, la + new_l.len() - suffix, table.len());
        }
        if !self.bookmarks.is_empty() && self.marks_key.get() == old_key {
            self.marks_text.replace_range(a..b, new);
            self.marks_key.set(new_key);
        }
        (start, removed)
    }

    fn delete_selection(&mut self) -> bool {
        let (a, b) = self.selection();
        self.delete_range(a, b)
    }

    fn insert(&mut self, text: &str) -> bool {
        if self.read_only {
            return false;
        }
        let clean: String = text.replace("\r\n", "\n").chars().filter(|c| *c == '\n' || *c == '\t' || !c.is_control()).collect();
        let deleted = self.delete_selection();
        let room = if self.max_length == 0 { usize::MAX } else { self.max_length.saturating_sub(self.len()) };
        let piece: String = clean.chars().take(room).collect();
        if piece.is_empty() {
            return deleted;
        }
        self.splice(self.caret, self.caret, &piece);
        self.caret += piece.chars().count();
        self.anchor = self.caret;
        true
    }

    fn move_to(&mut self, index: usize, extend: bool) {
        self.caret = index.min(self.len());
        if !extend {
            self.anchor = self.caret;
        }
    }

    /// A palavra sob o cursor (vazia fora de uma palavra).
    pub fn word_at_caret(&self) -> String {
        let (a, b) = self.word_bounds(self.caret);
        self.slice(a, b.max(a)).to_string()
    }

    /// A palavra em volta de `index`. Só olha a vizinhança (a pintura chama a cada quadro: copiar
    /// o texto inteiro, num fonte de milhares de linhas, pesava).
    fn word_bounds(&self, index: usize) -> (usize, usize) {
        let word = |c: char| c.is_alphanumeric() || c == '_';
        let at = self.byte_at(index);
        let index = if at == self.text.len() { self.len() } else { index };
        let before = self.text[..at].chars().rev().take_while(|c| word(*c)).count();
        let after = self.text[at..].chars().take_while(|c| word(*c)).count();
        (index - before, index + after)
    }

    /// O começo da palavra antes de `from` (andando só por perto: copiar o texto inteiro para
    /// isso pesava num fonte enorme).
    fn prev_word(&self, from: usize) -> usize {
        let from = from.min(self.len());
        let mut back = self.text[..self.byte_at(from)].chars().rev().peekable();
        let mut n = 0;
        while back.next_if(|c| c.is_whitespace()).is_some() {
            n += 1;
        }
        while back.next_if(|c| !c.is_whitespace()).is_some() {
            n += 1;
        }
        from - n
    }

    fn next_word(&self, from: usize) -> usize {
        let from = from.min(self.len());
        let mut ahead = self.text[self.byte_at(from)..].chars().peekable();
        let mut n = 0;
        while ahead.next_if(|c| !c.is_whitespace()).is_some() {
            n += 1;
        }
        while ahead.next_if(|c| c.is_whitespace()).is_some() {
            n += 1;
        }
        from + n
    }

    /// O caractere `index` (pela tabela das linhas: sem contar o texto desde o começo).
    fn char_at(&self, index: usize) -> Option<char> {
        self.text[self.byte_at(index)..].chars().next()
    }

    /// O caractere de `row` mais perto de `x` (pixels a partir do começo do texto da linha).
    fn index_in_row(&self, ctx: &EventCtx<'_>, row: Row, x: f32) -> usize {
        let style = self.text_style.borrow().clone();
        let line = self.slice(row.start, row.end).to_string();
        let width = |n: usize| ctx.ui.measure_text(&expand_tabs(&line[..byte_index(&line, n)]), &style).width;
        // Numa quebra automática, o fim da linha é o começo da de baixo: fica antes.
        let count = line.chars().count();
        let count = if self.wrap && count > 0 && row.end < self.len() && !self.char_at(row.end).is_some_and(|c| c == '\n') {
            count.saturating_sub(1).max(line.trim_end().chars().count())
        } else {
            count
        };
        let (mut lo, mut hi) = (0usize, count);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if width(mid + 1) <= x { lo = mid + 1 } else { hi = mid }
        }
        if lo < count && x - width(lo) > width(lo + 1) - x {
            lo += 1;
        }
        row.start + lo
    }

    /// Caractere sob um ponto local.
    fn index_at(&self, ctx: &EventCtx<'_>, p: PointF) -> usize {
        let origin = self.origin.get();
        let scroll = self.scroll.get();
        let lh = self.line_height.get().max(1.0);
        let rows = self.rows_ui(ctx);
        let r = (((p.y - origin.y + scroll.y) / lh).max(0.0) as usize).min(rows.len().saturating_sub(1));
        let Some(&row) = rows.get(r) else { return 0 };
        let style = self.text_style.borrow().clone();
        let row_width = ctx.ui.measure_text(&expand_tabs(self.slice(row.start, row.end)), &style).width;
        let x = p.x - origin.x + scroll.x - self.shift(row_width);
        self.index_in_row(ctx, row, x)
    }

    /// Sobe ou desce `lines` linhas visuais, mantendo a posição horizontal.
    fn vertical(&mut self, ctx: &EventCtx<'_>, lines: isize, extend: bool) {
        let rows = self.rows_ui(ctx);
        let current = Self::row_of(&rows, self.caret);
        let style = self.text_style.borrow().clone();
        let row = rows[current];
        let x = ctx.ui.measure_text(&expand_tabs(self.slice(row.start, self.caret)), &style).width;
        let goal = *self.goal_x.get_or_insert(x);
        let target = (current as isize + lines).clamp(0, rows.len() as isize - 1) as usize;
        let index = self.index_in_row(ctx, rows[target], goal);
        self.caret = index;
        if !extend {
            self.anchor = index;
        }
    }

    fn copy(&self, ctx: &mut EventCtx<'_>) {
        if self.has_selection() {
            let text = self.selected_text().to_string();
            aylis_core::clipboard::set_text(ctx.ui, &text);
        }
    }

    fn changed(&mut self, ctx: &mut EventCtx<'_>) {
        // Digitando, a dica do mouse some.
        self.tip = None;
        self.shift_bookmarks();
        self.assist(ctx.ui, true);
        ctx.fire("on_change", EventArgs::Value(self.text.clone().into()));
    }

    /// As palavras erradas e, no fim de uma palavra, o resto sugerido (como no Edit).
    fn assist(&mut self, ui: &mut aylis_core::Ui, typing: bool) {
        self.completion.clear();
        let Some(engine) = ui.services_mut().text_assist.as_mut().filter(|_| !self.read_only) else {
            self.misspelled.clear();
            return;
        };
        let caret = self.caret;
        let to_chars = |text: &str, byte: usize| text[..byte].chars().count();
        if self.spellcheck {
            let text = &self.text;
            self.misspelled = engine
                .check(text, &self.language)
                .into_iter()
                .map(|m| (to_chars(text, m.start), to_chars(text, m.end)))
                .filter(|&(s, e)| !(typing && s <= caret && e == caret))
                .collect();
        } else {
            self.misspelled.clear();
        }
        // No fim de uma palavra (o que vem depois não é letra), sem seleção.
        let next = self.char_at(caret);
        if self.autocomplete && typing && !self.has_selection() && next.is_none_or(|c| !aylis_core::textassist::is_word_char(c)) {
            let before = &self.text[..self.byte_at(caret)];
            let typed = aylis_core::textassist::word_before(before).to_string();
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

    /// Seleciona de `start` a `end` (em caracteres; iguais: só o cursor).
    pub fn select(&mut self, start: usize, end: usize) {
        let len = self.len();
        self.anchor = start.min(len);
        self.caret = end.min(len);
    }

    /// Onde a seleção começa (em caracteres).
    pub fn sel_start(&self) -> usize {
        self.selection().0
    }

    pub fn set_sel_start(&mut self, start: usize) {
        self.select(start, start);
    }

    pub fn sel_length(&self) -> usize {
        let (a, b) = self.selection();
        b - a
    }

    pub fn set_sel_length(&mut self, length: usize) {
        let start = self.sel_start();
        self.select(start, start + length);
    }

    pub fn sel_text(&self) -> String {
        self.selected_text().to_string()
    }

    /// Troca a seleção (ou insere no cursor); vale com `read_only`.
    pub fn set_sel_text(&mut self, text: String) {
        let read_only = std::mem::replace(&mut self.read_only, false);
        self.insert(&text);
        self.read_only = read_only;
    }

    /// Insere `text` no cursor (trocando a seleção), como se fosse digitado.
    pub fn insert_text(&mut self, text: &str) {
        self.set_sel_text(text.to_string());
    }

    /// A linha do cursor (a partir de 1).
    pub fn caret_line(&self) -> usize {
        self.caret_position().0 + 1
    }

    /// A coluna do cursor (a partir de 1).
    pub fn caret_column(&self) -> usize {
        self.caret_position().1 + 1
    }

    /// Põe o cursor na linha (a partir de 1), na mesma coluna (ou no fim da linha).
    pub fn set_caret_line(&mut self, line: usize) {
        let column = self.caret_column();
        self.go_to(line, column);
    }

    pub fn set_caret_column(&mut self, column: usize) {
        let line = self.caret_line();
        self.go_to(line, column);
    }

    /// Põe o cursor na linha e coluna (a partir de 1; além do fim, no fim).
    pub fn go_to(&mut self, line: usize, column: usize) {
        // Pela tabela das linhas (sem andar o texto até a linha).
        let table = self.lines_of(self.text_key());
        let len = self.len();
        let Some(&(start, _)) = table.get(line.max(1) - 1) else {
            self.select(len, len);
            return;
        };
        let end = table.get(line.max(1)).map_or(len, |next| next.0 - 1);
        let index = start + (column.max(1) - 1).min(end - start);
        self.select(index, index);
    }

    /// As palavras marcadas como erradas (trechos em caracteres).
    pub fn misspelled(&self) -> &[(usize, usize)] {
        &self.misspelled
    }

    /// O resto da palavra sugerida (vazio: nenhuma).
    pub fn completion(&self) -> &str {
        &self.completion
    }

    /// Troca os caracteres `start..end` por `text` (a correção escolhida no menu).
    fn replace_chars(&mut self, start: usize, end: usize, text: &str) {
        if start > end {
            return;
        }
        self.splice(start, end, text);
        let caret = start + text.chars().count();
        self.caret = caret;
        self.anchor = caret;
        self.misspelled.retain(|&(s, _)| s < start);
    }

    fn activity(&mut self, ctx: &mut EventCtx<'_>) {
        // O cursor foi parar dentro de um bloco dobrado: ele abre.
        self.reveal_cursors();
        self.blink_start = ctx.ui.time();
        ctx.fire("on_caret_move", EventArgs::None);
        ctx.repaint();
    }

    /// As linhas com ponto de parada (a partir de 1), desenhadas na calha.
    pub fn set_breakpoints(&mut self, lines: &[usize]) {
        self.breakpoints = lines.to_vec();
    }

    pub fn breakpoints(&self) -> &[usize] {
        &self.breakpoints
    }

    /// A linha onde o programa parou (a partir de 1), destacada; `None` tira o destaque.
    /// Vai até ela (a pintura rola, mesmo sem o foco).
    pub fn set_exec_line(&mut self, line: Option<usize>) {
        if line != self.exec_line {
            self.exec_note.clear();
        }
        self.exec_line = line;
        if let Some(l) = line {
            self.go_to_line(l.saturating_sub(1));
        }
    }

    pub fn exec_line(&self) -> Option<usize> {
        self.exec_line
    }

    /// Um texto apagado no fim da linha onde o programa parou (o depurador põe o tempo e a
    /// memória do passo). Vazio: nada. Sai junto com a linha (`set_exec_line(None)`).
    pub fn set_exec_note(&mut self, note: impl Into<String>) {
        self.exec_note = note.into();
    }

    pub fn exec_note(&self) -> &str {
        &self.exec_note
    }

    /// Põe o cursor no começo da linha `line` (a partir de 0) do texto; a próxima pintura
    /// rola até ela (mesmo sem o foco).
    pub fn go_to_line(&mut self, line: usize) {
        let index = self.lines_of(self.text_key()).get(line).map_or(self.len(), |&(c, _)| c);
        self.caret = index;
        self.anchor = index;
        self.goal_x = None;
        self.extra.clear();
        self.unfold_at(index);
        self.reveal.set(true);
    }
}

impl Component for Memo {
    leaf_control!("Memo");

    fn accepts_text(&self) -> bool {
        true
    }

    // O Enter (nova linha) e as setas ficam no Memo: não viram o Tab do form (`tab_keys`).
    fn captures_key(&self, event: &KeyEvent) -> bool {
        !event.modifiers.ctrl
            && (matches!(event.key, Key::Enter | Key::Up | Key::Down | Key::Left | Key::Right | Key::Home | Key::End | Key::PageUp | Key::PageDown | Key::Backspace | Key::Delete)
                || (event.key == Key::Tab && self.want_tabs))
    }

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        text: String => "Data",
        placeholder: String => "Appearance",
        border: bool => "Appearance",
        read_only: bool => "Behavior",
        sensitive: bool => "Behavior",
        max_length: usize => "Behavior",
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        char_map: String => "Keyboard",
        char_case: aylis_core::CharCase => "Keyboard",
        h_align: TextAlign => "Text",
        wrap: bool => "Text",
        trimming: Trimming => "Text",
        syntax: String => "Text",
        line_numbers: bool => "Text",
        want_tabs: bool => "Behavior",
        wheel_lines: u32 => "Behavior",
        spellcheck: bool => "Behavior",
        autocomplete: bool => "Behavior",
        language: String => "Behavior",
        keyboard_suggestions: bool => "Behavior",
        caret_color: Option<aylis_core::Color> => "Caret",
        caret_width: f32 => "Caret",
        caret_style: crate::CaretStyle => "Caret",
        caret_blink: u32 as Duration => "Caret";
        events: on_change, on_caret_move, on_gutter_click, on_bookmarks_change, on_paste;
        computed: sel_start: usize => sel_start / set_sel_start,
            sel_length: usize => sel_length / set_sel_length,
            sel_text: String => sel_text / set_sel_text,
            caret_line: usize => caret_line / set_caret_line,
            caret_column: usize => caret_column / set_caret_column;
        on_set: after_set
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        // Ctrl apertado com o mouse parado sobre um nome: a mãozinha aparece já.
        if matches!(event.key, Key::Control | Key::Meta) {
            self.update_link(ctx, true);
        }
        // Andando com o cursor (fora da lista de sugestões), a dica e a ajuda dos parâmetros
        // somem, como no VS Code.
        if !self.suggestions_visible() && matches!(event.key, Key::Up | Key::Down | Key::Home | Key::End | Key::PageUp | Key::PageDown) {
            self.tip = None;
            self.signature = None;
        }
        if self.code_editing && self.marks_key(ctx, event) {
            return;
        }
        if self.code_editing && self.multi_key(ctx, event) {
            return;
        }
        if self.code_key(ctx, event) {
            return;
        }
        let shift = event.modifiers.shift;
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        // A palavra sugerida: Tab ou seta para a direita aceita, Esc dispensa.
        if !self.completion.is_empty() && !ctrl && !shift {
            match event.key {
                Key::Tab | Key::Right => {
                    let rest = std::mem::take(&mut self.completion);
                    if self.insert(&rest) {
                        self.changed(ctx);
                        self.completion.clear();
                    }
                    ctx.set_handled();
                    self.activity(ctx);
                    return;
                }
                Key::Escape => {
                    self.completion.clear();
                    ctx.set_handled();
                    ctx.repaint();
                    return;
                }
                _ => self.completion.clear(),
            }
        }
        let mut changed = false;
        let keep_goal = matches!(event.key, Key::Up | Key::Down | Key::PageUp | Key::PageDown);
        // As teclas que mudam o texto guardam o estado de antes (o Ctrl+Z).
        let edit_kind = match event.key {
            Key::Backspace | Key::Delete => Some(code::EditKind::Deleting),
            Key::Enter | Key::Tab => Some(code::EditKind::Typing),
            Key::Char('x') | Key::Char('v') if ctrl => Some(code::EditKind::Other),
            _ => None,
        };
        let history = self.undo_stack.len();
        if let Some(kind) = edit_kind.filter(|_| !self.read_only) {
            self.begin_edit(kind, ctx.ui.time());
        }
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
                    self.caret + 1
                };
                self.move_to(to, shift);
            }
            Key::Up => self.vertical(ctx, -1, shift),
            Key::Down => self.vertical(ctx, 1, shift),
            Key::PageUp => self.vertical(ctx, -(self.visible_lines.get().max(1) as isize), shift),
            Key::PageDown => self.vertical(ctx, self.visible_lines.get().max(1) as isize, shift),
            Key::Home | Key::End if ctrl => {
                let to = if event.key == Key::Home { 0 } else { self.len() };
                self.move_to(to, shift);
            }
            Key::Home | Key::End => {
                // Começo ou fim da linha visual.
                let rows = self.rows_ui(ctx);
                let row = rows[Self::row_of(&rows, self.caret)];
                let to = if event.key == Key::Home {
                    row.start
                } else {
                    // Numa quebra automática, o fim fica antes do espaço da quebra.
                    let line = self.slice(row.start, row.end);
                    let wrapped = row.end < self.len() && self.char_at(row.end) != Some('\n');
                    if wrapped { row.start + line.trim_end().chars().count() } else { row.end }
                };
                self.move_to(to, shift);
            }
            Key::Enter if !self.read_only => {
                let text = if self.syntax.is_empty() { "\n".to_string() } else { self.newline_with_indent() };
                changed = self.insert(&text);
            }
            Key::Tab if self.want_tabs && !self.read_only && !ctrl => {
                changed = if shift { self.unindent() } else { self.insert("    ") };
            }
            Key::Backspace if !self.read_only => {
                changed = self.delete_selection() || {
                    let from = if ctrl { self.prev_word(self.caret) } else { self.caret.saturating_sub(1) };
                    self.delete_range(from, self.caret)
                };
            }
            Key::Delete if !self.read_only => {
                changed = self.delete_selection() || {
                    let to = if ctrl { self.next_word(self.caret) } else { (self.caret + 1).min(self.len()) };
                    self.delete_range(self.caret, to)
                };
            }
            Key::Char('a') if ctrl => self.select_all(),
            Key::Char('c') if ctrl => self.copy(ctx),
            Key::Char('x') if ctrl && !self.read_only => {
                self.copy(ctx);
                changed = self.delete_selection();
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
            Key::Enter | Key::Backspace | Key::Delete => {}
            _ => return,
        }
        if !keep_goal {
            self.goal_x = None;
        }
        if edit_kind.is_some() {
            if changed {
                self.end_edit();
            } else if self.undo_stack.len() > history {
                self.undo_stack.pop(); // não mudou nada
            }
        }
        ctx.set_handled();
        if changed {
            self.changed(ctx);
            // Apagando, a lista de sugestões acompanha (ou some antes da palavra).
            if self.suggestions_visible() {
                if self.caret < self.suggest_start { self.hide_suggestions() } else { self.filter_suggestions_now() }
            }
        } else if matches!(event.key, Key::Left | Key::Right | Key::Up | Key::Down | Key::Home | Key::End | Key::PageUp | Key::PageDown) {
            // Andando com o cursor: a lista, a dica e a ajuda dos parâmetros somem (como no VS Code).
            self.hide_suggestions();
            self.tip = None;
            if !matches!(event.key, Key::Left | Key::Right) {
                self.signature = None;
            }
        }
        self.activity(ctx);
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.goal_x = None;
        if self.code_editing && self.multi_text(ctx, text) {
            return;
        }
        if self.code_editing && self.code_text(ctx, text) {
            ctx.set_handled();
            return;
        }
        let history = self.undo_stack.len();
        if !self.read_only {
            self.begin_edit(code::EditKind::Typing, ctx.ui.time());
        }
        if self.insert(text) {
            self.end_edit();
            self.changed(ctx);
            if let Some(c) = text.chars().last() {
                self.after_typing(ctx, c);
            }
        } else if self.undo_stack.len() > history {
            self.undo_stack.pop();
        }
        ctx.set_handled();
        self.activity(ctx);
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.click_suggestion(ctx, event.position) {
            return;
        }
        self.tip = None;
        if event.button == PointerButton::Secondary && self.spellcheck {
            let index = self.index_at(ctx, event.position);
            if let Some((start, end)) = self.misspelled.iter().copied().find(|&(s, e)| s <= index && index <= e) {
                let id = ctx.id;
                let word = self.slice(start, end.max(start)).to_string();
                let replace = move |ui: &mut aylis_core::Ui, choice: &str| {
                    let text = match ui.get_mut::<Memo>(id) {
                        Some(memo) => {
                            memo.replace_chars(start, end, choice);
                            memo.text.clone()
                        }
                        None => return,
                    };
                    ui.fire(id, "on_change", &EventArgs::Value(text.into()));
                };
                let learned = move |ui: &mut aylis_core::Ui| {
                    if let Some(memo) = ui.get_mut::<Memo>(id) {
                        memo.misspelled.retain(|&r| r != (start, end));
                    }
                };
                let language = self.language.clone();
                crate::edit::spelling_menu(ctx, &mut self.spell_menu, event.position, &word, &language, replace, learned);
                ctx.set_handled();
            }
            return;
        }
        if event.button != PointerButton::Primary {
            return;
        }
        // A barra de rolagem: pegar o polegar (ou clicar no trilho, que leva o polegar até
        // ali) rola sem mexer no cursor.
        if let Some(horizontal) = self.bar_at(event.position) {
            let (_, thumb, _) = if horizontal { self.hbar.get() } else { self.bar.get() }.unwrap_or_default();
            let grab = match (horizontal, thumb.contains(event.position)) {
                (true, true) => event.position.x - thumb.x,
                (true, false) => thumb.width / 2.0,
                (false, true) => event.position.y - thumb.y,
                (false, false) => thumb.height / 2.0,
            };
            self.bar_grab = Some((horizontal, grab));
            self.drag_bar(ctx, event.position);
            ctx.set_handled();
            return;
        }
        // O minimapa: clicar (ou arrastar) rola até ali.
        if self.minimap && self.in_minimap(event.position) {
            self.minimap_drag = true;
            self.minimap_scroll(event.position);
            ctx.set_handled();
            ctx.repaint();
            return;
        }
        self.completion.clear();
        // Clique na calha dos números: a linha (para ligar um ponto de parada); na coluna
        // das setas, dobra ou desdobra o bloco.
        let gutter = self.gutter_width.get();
        if gutter > 0.0 && event.position.x < self.origin.get().x {
            let lh = self.line_height.get().max(1.0);
            let r = ((event.position.y - self.origin.get().y + self.scroll.get().y) / lh).floor().max(0.0) as usize;
            if let Some(row) = self.rows_ui(ctx).get(r).copied() {
                let fold_column = self.folding && self.code_editing && event.position.x >= self.origin.get().x - editor::FOLD_WIDTH;
                if fold_column {
                    if self.toggle_fold(row.line) {
                        ctx.repaint();
                    }
                } else {
                    ctx.fire("on_gutter_click", EventArgs::Value(aylis_core::PropValue::Int(row.line as i64 + 1)));
                }
            }
            ctx.set_handled();
            return;
        }
        let index = self.index_at(ctx, event.position);
        self.goal_x = None;
        // Alt+clique: mais um cursor (como no VS Code); o clique comum volta a um só.
        if self.code_editing && event.modifiers.alt && event.click_count == 1 {
            self.add_cursor(index, index);
            ctx.set_handled();
            self.activity(ctx);
            return;
        }
        self.extra.clear();
        // Ctrl+clique numa palavra: `on_word_click` com ela (a IDE vai para a declaração).
        if event.click_count == 1 && (event.modifiers.ctrl || event.modifiers.meta) {
            self.move_to(index, false);
            let (a, b) = self.word_bounds(index);
            let word = self.slice(a, b).to_string();
            if !word.trim().is_empty() {
                ctx.fire("on_word_click", EventArgs::Text(word));
            }
            ctx.set_handled();
            ctx.repaint();
            return;
        }
        match event.click_count {
            2 => {
                let (a, b) = self.word_bounds(index);
                self.anchor = a;
                self.caret = b;
            }
            n if n >= 3 => {
                // a linha do texto inteira (de Enter a Enter)
                let table = self.lines_of(self.text_key());
                let line = self.line_of(index);
                self.anchor = table[line].0;
                self.caret = table.get(line + 1).map_or(self.len(), |next| next.0 - 1);
            }
            _ => {
                self.move_to(index, event.modifiers.shift);
                self.dragging = true;
            }
        }
        self.activity(ctx);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.minimap_drag {
            self.minimap_scroll(event.position);
            ctx.repaint();
            return;
        }
        if self.bar_grab.is_some() {
            self.drag_bar(ctx, event.position);
            return;
        }
        // Sobre uma barra: elas ficam mais fortes e o cursor vira a seta.
        let over = self.bar_at(event.position).is_some();
        if over != self.bar_hover.replace(over) {
            self.base.set_cursor(if over { CursorShape::Default } else { CursorShape::Text });
            ctx.repaint();
        }
        self.pointer_at = Some(event.position);
        if self.dragging {
            let index = self.index_at(ctx, event.position);
            self.move_to(index, true);
            self.activity(ctx);
        } else if self.code_editing && !over && event.position.x >= self.origin.get().x {
            let index = self.index_at(ctx, event.position);
            self.hover_at(ctx, Some(index));
        }
        self.update_link(ctx, event.modifiers.ctrl || event.modifiers.meta);
    }

    fn key_up(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if matches!(event.key, Key::Control | Key::Meta) {
            self.update_link(ctx, false);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = false;
        self.minimap_drag = false;
        if self.bar_grab.take().is_some() {
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if self.code_editing {
            self.hover_at(ctx, None);
        }
        self.pointer_at = None;
        self.update_link(ctx, false);
        if self.bar_hover.replace(false) {
            self.base.set_cursor(CursorShape::Text);
            ctx.repaint();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        // De lado: Shift+roda ou o touchpad indo de lado, até onde a barra horizontal deixa.
        let sideways = event.delta.x.abs() > event.delta.y.abs() || event.modifiers.shift;
        if sideways {
            let Some((_, _, hmax)) = self.hbar.get() else { return };
            let d = if event.delta.x.abs() > event.delta.y.abs() { event.delta.x } else { event.delta.y };
            let d = if event.touch { d } else { d.signum() * self.line_height.get() * self.wheel_lines.max(1) as f32 };
            let mut s = self.scroll.get();
            let next = (s.x - d).clamp(0.0, hmax);
            if (next - s.x).abs() < 0.5 {
                return;
            }
            s.x = next;
            self.scroll.set(s);
            ctx.repaint();
            ctx.set_handled();
            return;
        }
        let lh = self.line_height.get();
        let rows = self.rows_ui(ctx).len();
        let total = rows as f32 * lh;
        let max = self.scroll_max(total, self.text_height.get(), lh);
        let mut s = self.scroll.get();
        // Ctrl + roda: uma página por dente (a altura à vista menos uma linha).
        let lines = if event.modifiers.ctrl { (self.visible_lines.get() as f32 - 1.0).max(1.0) } else { self.wheel_lines.max(1) as f32 };
        let dy = if event.touch { event.delta.y } else { event.delta.y.signum() * lh * lines };
        // Roda com dentes e o deslize ligado: desliza até lá, quadro a quadro (os dentes
        // seguidos somam ao destino); o dedo e o touchpad acompanham direto.
        if !event.touch && event.lines && ctx.ui.smooth_scrolling() {
            let base = self.glide.get().map_or(s.y, |(_, to, _)| to);
            let target = (base - dy).clamp(0.0, max);
            if (target - base).abs() < 0.5 {
                return; // no limite: a roda passa para quem está fora
            }
            self.glide.set(Some((s.y, target, ctx.ui.time())));
            ctx.ui.request_frames(ctx.id);
            ctx.set_handled();
            return;
        }
        self.glide.set(None);
        let next = (s.y - dy).clamp(0.0, max);
        if (next - s.y).abs() < 0.5 {
            return; // no limite: a roda passa para quem está fora
        }
        let before = s;
        s.y = next;
        self.scroll.set(s);
        self.scrolled_from(ctx, before);
        ctx.set_handled();
    }

    /// Um passo do deslize da roda, no tempo do quadro (acompanha o monitor, sem degraus).
    fn frame(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        let Some((from, to, at)) = self.glide.get() else { return false };
        let ms = ctx.ui.scroll_options().smooth_ms.max(1.0) as f64;
        let k = ((ctx.ui.time() - at) * 1000.0 / ms).clamp(0.0, 1.0) as f32;
        let eased = 1.0 - (1.0 - k).powi(3);
        let mut s = self.scroll.get();
        let before = s;
        // Em pixels inteiros: o texto já pintado anda junto (é copiado, não repintado).
        s.y = if k >= 1.0 { to.round() } else { (from + (to - from) * eased).round() };
        self.scroll.set(s);
        self.scrolled_from(ctx, before);
        if k >= 1.0 {
            self.glide.set(None);
        }
        k < 1.0
    }

    fn focus_enter(&mut self, ctx: &mut EventCtx<'_>) {
        self.activity(ctx);
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // O `char_case` vale também para o texto gravado pelo código (e ao mudar a regra).
        if matches!(name, "text" | "char_case") && self.char_case != aylis_core::CharCase::Normal {
            let cased = self.char_case.apply(&self.text);
            if cased != self.text {
                self.text = cased;
                self.text_changed();
            }
        }
        if name == "sel_text" {
            self.changed(ctx);
        }
        // Texto vindo do form, do banco ou do código: sublinha já (sem sugerir).
        if matches!(name, "text" | "spellcheck" | "language") {
            self.assist(ctx.ui, false);
            ctx.repaint();
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.dragging = false;
        self.completion.clear();
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // O texto não muda durante a pintura: a chave e a tabela das linhas valem até o fim.
        let key = self.text_key();
        *self.painting.borrow_mut() = Some(self.lines_of(key));
        self.paint_text(canvas, ctx, key);
        *self.painting.borrow_mut() = None;
    }
}

impl Memo {
    fn paint_text(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, key: u64) {
        let s = ctx.style;
        // O tema de cores do código, quando escolhido, manda no fundo e nas cores do texto.
        let ct = if self.code_editing { syntax::code_theme(&self.color_theme) } else { None };
        canvas.fill_rect(ctx.rect, s.radius, &Brush::Solid(ct.map_or(s.fill, |t| t.background())));
        if self.border && s.border_width > 0.0 && !s.border_color.is_transparent() {
            canvas.stroke_rect(ctx.rect, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let mut inner = ctx.rect.deflate(s.padding);
        let mut style = ctx.text_style();
        if let Some(t) = ct {
            style.color = t.foreground();
        }
        let selection = ct.map_or_else(|| ctx.theme.color("selection"), |t| t.selection());
        let code = !self.syntax.is_empty();
        if code {
            if let Some(mono) = ctx.theme.fonts.get("mono") {
                style.font.family = mono.family.clone();
            }
        }
        // Os números das linhas ficam numa calha à esquerda; o texto começa depois dela.
        // Com dobras, a calha ganha a coluna das setas (à direita dos números).
        let fold_w = if self.folding && self.code_editing { editor::FOLD_WIDTH } else { 0.0 };
        // No editor de código, a coluna dos marcadores (Ctrl+B) à esquerda dos números: sem ela
        // o marcador cobria o número (relatado pelo autor em 07/11).
        let mark_w = if self.code_editing { 16.0 } else { 0.0 };
        let gutter = if self.line_numbers {
            let digits = self.painting.borrow().as_ref().map_or(1, |t| t.len()).to_string().len().max(2);
            canvas.measure_text(&"0".repeat(digits), &style).width + 18.0 + fold_w + mark_w
        } else {
            0.0
        };
        let gutter_rect = RectF::new(inner.x, inner.y, gutter, inner.height);
        self.gutter_width.set(gutter);
        inner.x += gutter;
        inner.width = (inner.width - gutter).max(1.0);
        // O minimapa fica à direita do texto (a barra de rolagem na beirada dele).
        let mini = if self.minimap && self.code_editing && inner.width > editor::MINIMAP_WIDTH * 3.0 { editor::MINIMAP_WIDTH } else { 0.0 };
        inner.width -= mini;
        if mini == 0.0 {
            self.minimap_rect.set(None);
        }
        *self.text_style.borrow_mut() = style.clone();
        let lh = (style.font.size * 1.45).round().max(10.0);
        self.line_height.set(lh);
        // Com quebra, a barrinha de rolagem tem lugar: o texto quebra antes dela.
        self.text_width.set(inner.width.max(1.0));
        let visible = ((inner.height / lh).floor() as usize).max(1);
        self.visible_lines.set(visible);
        self.text_height.set(inner.height);
        self.origin.set(PointF::new(inner.x, inner.y));
        let focused = ctx.state.focused;

        let rows = {
            let mut measure = |t: &str| canvas.measure_text(t, &style).width;
            self.rows(inner.width, &mut measure)
        };
        let width_of = |canvas: &mut dyn Canvas, a: usize, b: usize| canvas.measure_text(&expand_tabs(self.slice(a, b)), &style).width;

        // Rola para manter o cursor visível.
        let crow = Self::row_of(&rows, self.caret);
        let row = rows[crow];
        let caret_row_width = width_of(canvas, row.start, row.end);
        let caret_x = width_of(canvas, row.start, self.caret) + self.shift(caret_row_width);
        let mut scroll = self.scroll.get();
        let caret_y = crow as f32 * lh;
        // Só quando o cursor andou ou o texto mudou; a roda do mouse e a barra de rolagem
        // mexem na rolagem sem mexer no cursor (nem o clique na barra, que dá o foco).
        let now = (self.caret, self.text.len());
        let moved = self.followed.replace(Some(now)) != Some(now);
        let reveal = self.reveal.take();
        if (focused && moved) || reveal {
            // O cursor andou: a rolagem vai atrás dele (um deslize da roda em andamento para).
            self.glide.set(None);
            // Ir para uma linha fora da vista (o depurador parou nela): fica no meio,
            // e não colada na borda.
            if reveal && (caret_y < scroll.y || caret_y + lh > scroll.y + inner.height) {
                scroll.y = (caret_y - (inner.height - lh) / 2.0).max(0.0);
            } else if caret_y < scroll.y {
                scroll.y = caret_y;
            } else if caret_y + lh > scroll.y + inner.height {
                scroll.y = caret_y + lh - inner.height;
            }
            if self.wrap {
                scroll.x = 0.0;
            } else if caret_x - scroll.x > inner.width - 2.0 {
                scroll.x = caret_x - inner.width + 2.0;
            } else if caret_x < scroll.x {
                scroll.x = caret_x;
            }
        } else if !focused && !self.wrap && self.trimming != Trimming::None {
            scroll.x = 0.0; // fora da edição, as linhas longas terminam em "…"
        }
        // A linha mais larga (sem quebra automática): até onde a rolagem de lado vai.
        let widest = if self.wrap {
            0.0
        } else {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            style.font.size.to_bits().hash(&mut h);
            style.font.family.hash(&mut h);
            let font = h.finish();
            let mut cached = self.widest.borrow_mut();
            if cached.0 != font || cached.1 != key {
                // Mede só as linhas mais compridas em caracteres (medir as milhares de um
                // fonte a cada tecla era o que mais pesava): a mais larga está entre elas.
                let mut longest: Vec<&Row> = rows.iter().collect();
                let top = longest.len().min(8);
                if top > 0 {
                    longest.select_nth_unstable_by_key(top - 1, |r| std::cmp::Reverse(r.end - r.start));
                }
                let w = longest[..top].iter().map(|r| width_of(canvas, r.start, r.end)).fold(0.0f32, f32::max);
                *cached = (font, key, w, Vec::new());
            } else if !cached.3.is_empty() {
                // Editou: só as linhas editadas (pela tabela das linhas).
                let table = self.lines_of(key);
                let len = self.len();
                let lines = std::mem::take(&mut cached.3);
                for line in lines.into_iter().filter(|&l| l < table.len()) {
                    let end = table.get(line + 1).map_or(len, |next| next.0 - 1);
                    cached.2 = cached.2.max(width_of(canvas, table[line].0, end));
                }
            }
            cached.2
        };
        // Fora da edição com "…" no fim das linhas longas, não há o que rolar de lado.
        let trimmed = !focused && self.trimming != Trimming::None;
        let hmax = if trimmed { 0.0 } else { (widest + 8.0 - inner.width).max(0.0) };
        scroll.y = scroll.y.clamp(0.0, self.scroll_max(rows.len() as f32 * lh, inner.height, lh));
        scroll.x = scroll.x.clamp(0.0, hmax);
        self.scroll.set(scroll);

        canvas.save();
        canvas.clip_rect(inner.inflate(1.0));
        let first = (scroll.y / lh) as usize;
        let (sel_a, sel_b) = self.selection();
        let mut text_style = style.clone();
        text_style.v_align = TextAlign::Center;
        text_style.trimming = Trimming::None;
        if self.text.is_empty() {
            let mut hint = style.clone();
            hint.color = crate::edit::placeholder_color(ctx.theme);
            hint.v_align = TextAlign::Leading;
            hint.h_align = self.h_align;
            hint.wrap = true;
            hint.trimming = self.trimming;
            canvas.draw_text(&self.placeholder, inner, &hint);
        }
        let trim = !focused && !self.wrap && self.trimming != Trimming::None && !code;
        // Justificado só fora da edição (editando, o cursor anda pelas posições medidas do
        // texto à esquerda).
        let justify = !focused && self.wrap && self.h_align == TextAlign::Justify && !code;
        let spans = code.then(|| self.spans_keyed(key));
        if self.highlight_pending.get() {
            ctx.request_frame();
        }
        let dark = ctx.theme.dark;
        // As ocorrências da palavra do cursor e o par do colchete (só o que está à vista).
        let view = (rows.get(first).map_or(0, |r| r.start), rows.get((first + visible + 1).min(rows.len().saturating_sub(1))).map_or(0, |r| r.end));
        let occurrences = if focused { self.occurrences(view.0, view.1) } else { Vec::new() };
        let brackets = if focused && self.code_editing { self.matching_bracket_cached() } else { None };
        // A linha visual de uma linha do texto (com blocos dobrados elas não batem).
        let row_of_line = |line: usize| rows.iter().position(|r| r.line == line);
        if let Some(r) = self.exec_line.and_then(|line| row_of_line(line.saturating_sub(1))) {
            // Onde o programa parou (amarelo, como nos depuradores).
            let y = inner.y + r as f32 * lh - scroll.y;
            canvas.fill_rect(RectF::new(inner.x, y, inner.width, lh), 0.0, &Brush::Solid(aylis_core::Color::rgb(0xFF, 0xD5, 0x4F).with_opacity(if ctx.theme.dark { 0.28 } else { 0.45 })));
            // A nota do depurador, depois do texto da última linha visual dela.
            let last = self.exec_line.and_then(|line| rows.iter().rposition(|row| row.line == line.saturating_sub(1)));
            if let Some(last) = last.filter(|_| !self.exec_note.is_empty()) {
                let row = rows[last];
                let x = inner.x - scroll.x + width_of(canvas, row.start, row.end) + style.font.size * 2.0;
                let y = inner.y + last as f32 * lh - scroll.y;
                let mut note = style.clone();
                note.font.italic = true;
                note.color = ctx.theme.color("text_muted");
                note.v_align = TextAlign::Center;
                note.h_align = TextAlign::Leading;
                canvas.draw_text(&self.exec_note, RectF::new(x, y, (inner.right() - x).max(400.0), lh), &note);
            }
        }
        if code && focused {
            // A linha do cursor, de leve.
            let y = inner.y + caret_y - scroll.y;
            canvas.fill_rect(RectF::new(inner.x, y, inner.width, lh), 0.0, &Brush::Solid(ct.map_or(style.color.with_opacity(0.05), |t| t.line_highlight())));
        }
        for (i, row) in rows.iter().enumerate().skip(first).take(visible + 1) {
            let y = inner.y + i as f32 * lh - scroll.y;
            let row_width = width_of(canvas, row.start, row.end);
            let x = inner.x - scroll.x + self.shift(row_width);
            let mut selected_rect = None;
            if focused && self.has_selection() && sel_b > row.start && sel_a <= row.end {
                let from = width_of(canvas, row.start, sel_a.max(row.start));
                let to_index = sel_b.min(row.end);
                let mut to = width_of(canvas, row.start, to_index);
                if sel_b > row.end {
                    to += 6.0; // a seleção continua na linha de baixo
                }
                let rect = RectF::new(x + from, y + 1.0, (to - from).max(0.0), lh - 2.0);
                canvas.fill_rect(rect, s.radius.min(2.0), &Brush::Solid(selection));
                selected_rect = Some(rect);
            }
            // As seleções dos outros cursores.
            if focused {
                for &(ea, ec) in &self.extra {
                    let (a, b) = (ea.min(ec), ea.max(ec));
                    if a != b && b > row.start && a <= row.end {
                        let from = width_of(canvas, row.start, a.max(row.start));
                        let mut to = width_of(canvas, row.start, b.min(row.end));
                        if b > row.end {
                            to += 6.0;
                        }
                        canvas.fill_rect(RectF::new(x + from, y + 1.0, (to - from).max(0.0), lh - 2.0), 2.0, &Brush::Solid(selection));
                    }
                }
            }
            // O bloco dobrado: o "⋯" depois do texto (clicar na seta da calha abre).
            if row.folded {
                let mut ts = text_style.clone();
                ts.color = ctx.theme.color("text_muted");
                ts.h_align = TextAlign::Center;
                let bx = x + row_width + 8.0;
                let b = RectF::new(bx, y + 3.0, 26.0, lh - 6.0);
                canvas.fill_rect(b, 3.0, &Brush::Solid(ctx.theme.color("text").with_opacity(0.08)));
                canvas.draw_text("⋯", b, &ts);
            }
            self.paint_row_marks(canvas, ctx, &text_style, (row.start, row.end), x, y, lh, &occurrences, brackets);
            let line = expand_tabs(self.slice(row.start, row.end)).into_owned();
            if line.is_empty() {
                continue;
            }
            // Linha sem trechos: o realce ainda não chegou nela (vai na cor do texto).
            if let Some(line_spans) = spans.as_ref().and_then(|s| s.get(row.line)).filter(|s| !s.is_empty()) {
                // Cada trecho na sua cor. A posição soma as larguras já medidas (o pedaço entre
                // um trecho e o próximo, e o próprio trecho), em vez de medir o começo da linha
                // de novo a cada trecho.
                let raw: Vec<char> = self.slice(row.start, row.end).chars().collect();
                let (mut done, mut px) = (0usize, 0.0f32);
                for span in line_spans {
                    let (a, b) = (span.start.min(raw.len()).max(done), span.end.min(raw.len()));
                    if a >= b {
                        continue;
                    }
                    if a > done {
                        let gap: String = raw[done..a].iter().collect();
                        px += canvas.measure_text(&expand_tabs(&gap), &text_style).width;
                    }
                    let piece: String = raw[a..b].iter().collect();
                    let piece = expand_tabs(&piece).into_owned();
                    let pw = canvas.measure_text(&piece, &text_style).width;
                    done = b;
                    let mut ts = text_style.clone();
                    ts.color = syntax::span_color(ct, span, dark, style.color);
                    canvas.draw_text(&piece, RectF::new(x + px, y, pw + 8.0, lh), &ts);
                    px += pw;
                }
            } else if let Some(words) = (justify && row.end < self.len() && self.char_at(row.end) != Some('\n'))
                .then(|| aylis_core::text::justify_line(&line, inner.width, &mut |s: &str| canvas.measure_text(s, &text_style).width))
                .flatten()
            {
                // Justificado (fora da edição): a linha quebrada ocupa a largura toda.
                for (wx, word) in words {
                    let w = canvas.measure_text(&word, &text_style).width;
                    canvas.draw_text(&word, RectF::new(inner.x - scroll.x + wx, y, w + 8.0, lh), &text_style);
                }
            } else if trim && row_width > inner.width {
                let mut ts = text_style.clone();
                ts.trimming = self.trimming;
                ts.h_align = self.h_align;
                canvas.draw_text(&line, RectF::new(inner.x, y, inner.width, lh), &ts);
            } else {
                canvas.draw_text(&line, RectF::new(x, y, row_width + 8.0, lh), &text_style);
            }
            // Tema com a cor do texto selecionado (o branco sobre o azul do Win32): o trecho
            // selecionado da linha de novo, nessa cor.
            if let (Some(rect), Some(&color)) = (selected_rect.filter(|_| ct.is_none()), ctx.theme.colors.get("selection_text")) {
                canvas.save();
                canvas.clip_rect(rect);
                let mut ts = text_style.clone();
                ts.color = color;
                canvas.draw_text(&line, RectF::new(x, y, row_width + 8.0, lh), &ts);
                canvas.restore();
            }
            // Palavras erradas desta linha: o sublinhado ondulado.
            let wave = Stroke::new(ctx.theme.color("danger"), 1.2);
            for &(a, b) in &self.misspelled {
                if b <= row.start || a >= row.end {
                    continue;
                }
                let x1 = x + width_of(canvas, row.start, a.max(row.start));
                let x2 = x + width_of(canvas, row.start, b.min(row.end));
                crate::edit::draw_wave(canvas, x1, x2, y + lh - 3.0, &wave);
            }
        }
        // A palavra sugerida, apagada, depois do cursor.
        if focused && !self.completion.is_empty() {
            let mut ghost = text_style.clone();
            ghost.color = ctx.theme.color("text_muted");
            let w = canvas.measure_text(&self.completion, &ghost).width + 8.0;
            let y = inner.y + caret_y - scroll.y;
            canvas.draw_text(&self.completion, RectF::new(inner.x - scroll.x + caret_x, y, w, lh), &ghost);
        }
        if focused && !ctx.state.disabled {
            let blink = f64::from(self.caret_blink) / 1000.0;
            let phase = if blink > 0.0 { (ctx.time - self.blink_start).rem_euclid(blink) } else { 0.0 };
            let on = blink <= 0.0 || phase < blink / 2.0;
            if on {
                let x = inner.x - scroll.x + caret_x;
                let y = inner.y + caret_y - scroll.y;
                let next_char = self.slice(self.caret, self.caret + 1).chars().next().filter(|c| !c.is_whitespace()).map(|c| c.to_string());
                let char_w = next_char.map_or(style.font.size * 0.55, |c| canvas.measure_text(&c, &text_style).width);
                let color = self.caret_color.or(ct.map(|t| t.cursor())).unwrap_or(style.color);
                crate::edit::draw_caret(canvas, self.caret_style, x, y + 3.0, lh - 6.0, char_w, self.caret_width, color);
                // Os outros cursores.
                for &(_, ec) in &self.extra {
                    let r = Self::row_of(&rows, ec);
                    let row = rows[r];
                    let ex = inner.x - scroll.x + width_of(canvas, row.start, ec.max(row.start).min(row.end)) + self.shift(width_of(canvas, row.start, row.end));
                    let ey = inner.y + r as f32 * lh - scroll.y;
                    crate::edit::draw_caret(canvas, self.caret_style, ex, ey + 3.0, lh - 6.0, char_w, self.caret_width, color);
                }
            }
            if blink > 0.0 {
                ctx.request_frame_at(ctx.time + if on { blink / 2.0 - phase } else { blink - phase });
            }
        }
        canvas.restore();
        // A lista de sugestões, a ajuda dos parâmetros e a dica, por cima do texto.
        let caret_screen = PointF::new(inner.x - scroll.x + caret_x, inner.y + caret_y - scroll.y);
        let tip_at = self.tip.as_ref().map(|&(_, at)| {
            let r = Self::row_of(&rows, at);
            let row = rows[r];
            PointF::new(inner.x - scroll.x + width_of(canvas, row.start, at.max(row.start).min(row.end)), inner.y + r as f32 * lh - scroll.y)
        });
        if mini > 0.0 {
            let area = RectF::new(inner.right(), inner.y, mini - BAR_WIDTH - 2.0, inner.height);
            self.paint_minimap(canvas, ctx, area, &rows, spans.as_ref().map(|s| s.as_slice()), lh, scroll.y, inner.height, key);
        }
        self.paint_popups(canvas, ctx, &style, caret_screen, lh, inner, tip_at);
        if gutter > 0.0 {
            canvas.save();
            canvas.clip_rect(gutter_rect);
            let mut ns = style.clone();
            ns.h_align = TextAlign::Trailing;
            ns.v_align = TextAlign::Center;
            let muted = ct.map_or_else(|| ctx.theme.color("text_muted"), |t| t.gutter());
            let danger = ctx.theme.color("danger");
            // As linhas do texto à vista, mais uma folga para achar o fim de um bloco (pela
            // tabela das linhas: dividir o texto inteiro a cada quadro pesava num fonte enorme).
            let (window_start, window) = match rows.get(first) {
                Some(top) if fold_w > 0.0 => {
                    let table = self.lines_of(key);
                    let bottom = rows.get((first + visible).min(rows.len().saturating_sub(1))).map_or(top.line, |r| r.line);
                    let end = (bottom + 1 + FOLD_LOOKAHEAD).min(table.len());
                    let line = |i: usize| {
                        let a = table[i].1;
                        let b = table.get(i + 1).map_or(self.text.len(), |next| next.1 - 1);
                        &self.text[a..b]
                    };
                    (top.line, (top.line..end).map(line).collect::<Vec<&str>>())
                }
                _ => (0, Vec::new()),
            };
            for i in first..(first + visible + 1).min(rows.len()) {
                let y = inner.y + i as f32 * lh - scroll.y;
                let row = rows[i];
                // A seta de dobrar: sempre na dobrada; nas que abrem bloco, de leve.
                if fold_w > 0.0 && (row.folded || row.line.checked_sub(window_start).is_some_and(|l| editor::fold_end(&window, l).is_some())) {
                    let color = if row.folded { style.color } else { muted.with_opacity(0.55) };
                    Self::paint_fold_mark(canvas, gutter_rect.x + gutter - fold_w, y, lh, row.folded, color);
                }
                self.paint_bookmark(canvas, row.line, gutter_rect.x, y, lh, &ns, ctx.theme.color("primary"));
                // Ponto de parada: o círculo vermelho no lugar do número.
                if self.breakpoints.contains(&(row.line + 1)) {
                    let d = (lh * 0.55).min(gutter - 6.0);
                    canvas.fill_ellipse(RectF::new(gutter_rect.x + gutter - fold_w - 10.0 - d, y + (lh - d) / 2.0, d, d), &Brush::Solid(danger));
                    continue;
                }
                ns.color = if i == crow && focused { style.color } else { muted.with_opacity(0.8) };
                canvas.draw_text(&(row.line + 1).to_string(), RectF::new(gutter_rect.x, y, gutter - fold_w - 10.0, lh), &ns);
            }
            if let Some(r) = self.exec_line.and_then(|line| row_of_line(line.saturating_sub(1))) {
                // A seta da linha em execução, na beirada da calha.
                let y = inner.y + r as f32 * lh - scroll.y + lh / 2.0;
                let x = gutter_rect.x + 1.0;
                let s = Stroke::new(aylis_core::Color::rgb(0xE0, 0x9A, 0x00), 2.0);
                canvas.draw_line(PointF::new(x, y - 4.0), PointF::new(x + 5.0, y), &s);
                canvas.draw_line(PointF::new(x + 5.0, y), PointF::new(x, y + 4.0), &s);
            }
            canvas.restore();
        }
        // A barra de rolagem vertical (como no VS Code): na borda direita, quando o texto não
        // cabe. O polegar mostra a parte à vista; arrastar ou clicar no trilho rola.
        // E a horizontal embaixo, quando há linha mais larga que o editor.
        let total = rows.len() as f32 * lh;
        let max = self.scroll_max(total, inner.height, lh);
        let (has_v, has_h) = (max > 0.5, hmax > 0.5);
        let active = self.bar_grab.is_some() || self.bar_hover.get();
        let track_fill = Brush::Solid(ctx.theme.color("text").with_opacity(0.05));
        let thumb_fill = Brush::Solid(ctx.theme.color("text_muted").with_opacity(if active { 0.7 } else { 0.4 }));
        if has_v {
            // Com as duas, cada uma deixa o canto para a outra.
            let height = inner.height - if has_h { BAR_WIDTH } else { 0.0 };
            let track = RectF::new(inner.right() + mini - BAR_WIDTH, inner.y, BAR_WIDTH, height);
            // A parte à vista no total que dá para rolar (no editor, com o espaço depois da
            // última linha).
            let thumb_h = (inner.height / (max + inner.height) * track.height).clamp(24.0_f32.min(track.height), track.height);
            let thumb_y = track.y + (scroll.y / max).clamp(0.0, 1.0) * (track.height - thumb_h);
            let thumb = RectF::new(track.x + 2.0, thumb_y, BAR_WIDTH - 4.0, thumb_h);
            if active {
                canvas.fill_rect(track, 0.0, &track_fill);
            }
            canvas.fill_rect(thumb, (BAR_WIDTH - 4.0) / 2.0, &thumb_fill);
            self.bar.set(Some((track, thumb, max)));
        } else {
            self.bar.set(None);
        }
        if has_h {
            let width = inner.width - if has_v { BAR_WIDTH } else { 0.0 };
            let track = RectF::new(inner.x, inner.bottom() - BAR_WIDTH, width, BAR_WIDTH);
            let content = widest + 8.0;
            let thumb_w = (inner.width / content * track.width).clamp(24.0_f32.min(track.width), track.width);
            let thumb_x = track.x + (scroll.x / hmax).clamp(0.0, 1.0) * (track.width - thumb_w);
            let thumb = RectF::new(thumb_x, track.y + 2.0, thumb_w, BAR_WIDTH - 4.0);
            if active {
                canvas.fill_rect(track, 0.0, &track_fill);
            }
            canvas.fill_rect(thumb, (BAR_WIDTH - 4.0) / 2.0, &thumb_fill);
            self.hbar.set(Some((track, thumb, hmax)));
        } else {
            self.hbar.set(None);
        }
    }
}

/// A largura da barra de rolagem do [`Memo`].
const BAR_WIDTH: f32 = 12.0;

#[path = "memo_code.rs"]
mod code;
#[path = "memo_editor.rs"]
mod editor;
#[path = "memo_highlight.rs"]
mod highlight;
#[path = "memo_marks.rs"]
pub(crate) mod marks;
pub use marks::Bookmark;
/// A última linha do bloco que a linha abre (pelo recuo), como no dobrar do `CodeEditor`.
pub use editor::fold_end as block_end;
pub use code::{Diagnostic, Suggestion};

impl Memo {
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

#[cfg(test)]
mod splice_tests {
    use super::*;

    fn memo(text: &str) -> Memo {
        let mut m = Memo::default();
        m.text = text.to_string();
        m
    }

    /// A tabela, as linhas visuais e o realce feitos do zero para o texto de agora.
    fn rebuilt(m: &Memo) -> (Vec<(usize, usize)>, Vec<Row>, Vec<Vec<Span>>) {
        let fresh = Memo { syntax: m.syntax.clone(), ..memo(&m.text) };
        let table = fresh.lines_of(fresh.text_key()).as_ref().clone();
        let rows = fresh.rows(0.0, &mut |_| 0.0).as_ref().clone();
        (table, rows, syntax::highlight(&m.syntax, &m.text))
    }

    /// O que a pintura usa, acertado pelas edições (sem refazer do zero).
    fn current(m: &Memo) -> (Vec<(usize, usize)>, Vec<Row>, Vec<Vec<Span>>) {
        let key = m.text_key();
        let table = m.lines_of(key).as_ref().clone();
        let rows = m.rows(0.0, &mut |_| 0.0).as_ref().clone();
        (table, rows, m.spans_keyed(key).as_ref().clone())
    }

    #[test]
    fn the_line_table_rows_and_highlight_follow_each_edit() {
        let mut m = memo(&(0..300).map(|i| if i % 9 == 0 { format!("/* bloco {i}") } else if i % 9 == 4 { "fim */ let s = \"á\";".to_string() } else { format!("let x{i} = {i};") }).collect::<Vec<_>>().join("\n"));
        m.syntax = "rust".into();
        let _ = current(&m);
        // Um gerador simples (sempre a mesma sequência).
        let mut seed = 7u64;
        let mut next = |n: usize| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as usize % n.max(1)
        };
        let pieces = ["x", "\n", "a\nb\n", "/*", "*/", "\"", "", "çã\né", "\n\n\n"];
        for round in 0..400 {
            let len = m.len();
            let start = next(len + 1);
            let end = (start + next(40)).min(len);
            let piece = pieces[next(pieces.len())];
            let key = m.text_key();
            m.splice(start, end, piece);
            assert_ne!(m.text_key(), key);
            // Acertados pela edição, não refeitos.
            assert_eq!(m.line_table.borrow().0, m.text_key());
            assert_eq!(m.rows_cache.borrow().0, m.text_key());
            assert_eq!(m.incremental.borrow().0, m.text_key());
            // Só de vez em quando realça (várias edições seguidas, sem pintar entre elas).
            if round % 3 == 0 {
                assert_eq!(current(&m), rebuilt(&m), "depois da edição {round}");
            }
        }
        assert_eq!(current(&m), rebuilt(&m));
        // Tirar e pôr todas as linhas.
        let len = m.len();
        m.splice(0, len, "");
        assert_eq!(current(&m), rebuilt(&m));
        m.splice(0, 0, "a\n\nb\n");
        assert_eq!(current(&m), rebuilt(&m));
    }

    #[test]
    fn undo_and_redo_keep_only_the_difference() {
        // Um texto grande: o desfazer guarda só os trechos trocados, não cópias dele.
        let big = "let x = 1;\n".repeat(50_000);
        let mut m = memo(&big);
        for i in 0..100 {
            m.begin_edit(code::EditKind::Other, i as f64 * 10.0);
            m.select(1000 * i, 1000 * i + 3);
            if i % 2 == 0 {
                m.insert("novo\ntexto");
            } else {
                m.delete_selection();
            }
            m.end_edit();
        }
        assert!(m.history_bytes() < 2_000, "o desfazer guarda {} bytes", m.history_bytes());
        while m.undo() {}
        assert_eq!(m.text, big);
        while m.redo() {}
        let redone = m.text.clone();
        while m.undo() {}
        assert_eq!(m.text, big);
        assert_ne!(redone, big);
    }

    #[test]
    fn undo_with_several_cursors_and_merged_typing() {
        let mut m = memo("um\ndois\ntres");
        m.code_editing = true;
        m.select(2, 2);
        m.add_cursor(7, 7);
        m.add_cursor(12, 12);
        // Duas digitações seguidas viram um passo; o Enter, outro.
        for (i, text) in ["x", "y", "\n"].iter().enumerate() {
            let kind = if *text == "\n" { code::EditKind::Other } else { code::EditKind::Typing };
            m.begin_edit(kind, i as f64 * 0.1);
            m.edit_all(|m| {
                m.delete_selection();
                m.insert(text);
            });
            m.end_edit();
        }
        assert_eq!(m.text, "umxy\n\ndoisxy\n\ntresxy\n");
        assert_eq!(m.undo_stack.len(), 2);
        assert!(m.undo());
        assert_eq!(m.text, "umxy\ndoisxy\ntresxy");
        assert!(m.undo());
        assert_eq!(m.text, "um\ndois\ntres");
        assert!(!m.undo());
        assert!(m.redo());
        assert_eq!(m.text, "umxy\ndoisxy\ntresxy");
        assert!(m.redo());
        assert_eq!(m.text, "umxy\n\ndoisxy\n\ntresxy\n");
        // A tabela das linhas e as linhas visuais seguem o desfazer.
        assert!(m.undo());
        assert_eq!(current(&m).0, rebuilt(&m).0);
        assert_eq!(current(&m).1, rebuilt(&m).1);
    }

    #[test]
    fn a_text_changed_from_outside_drops_the_history() {
        let mut m = memo("abc");
        m.begin_edit(code::EditKind::Other, 0.0);
        m.select(3, 3);
        m.insert("d");
        m.end_edit();
        // O app troca o campo direto: as posições guardadas não valem mais.
        m.text = "outro texto bem diferente".into();
        assert!(!m.undo());
        assert_eq!(m.text, "outro texto bem diferente");
    }

    #[test]
    fn bookmarks_follow_lines_inserted_above_them() {
        let mut m = memo("a\nb\nc\nd");
        m.set_bookmarks(vec![marks::Bookmark { line: 1, number: Some(1) }, marks::Bookmark { line: 3, number: None }]);
        // Enter no começo da linha "b": o marcador vai com ela.
        m.select(2, 2);
        m.insert("\n");
        m.shift_bookmarks();
        assert_eq!(m.bookmarks().iter().map(|b| b.line).collect::<Vec<_>>(), vec![2, 4]);
        // Apagar as linhas "b" e "c": o de baixo sobe; o da linha apagada fica no começo do
        // trecho (como na comparação do texto inteiro).
        m.splice(3, 7, "");
        m.shift_bookmarks();
        assert_eq!(m.text, "a\n\nd");
        assert_eq!(m.bookmarks().iter().map(|b| b.line).collect::<Vec<_>>(), vec![2, 2]);
    }
}
