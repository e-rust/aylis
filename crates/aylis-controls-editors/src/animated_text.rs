//! [`AnimatedText`]: texto com efeitos de entrada, saída e contínuos por letra, palavra ou
//! linha (como os de um programa de apresentação), vários ao mesmo tempo ou encadeados.
//!
//! Os efeitos vêm em `effects`, separados por `;`. Cada um é o nome e os parâmetros
//! `chave=valor` (e as palavras `out`, `after`, `loop`):
//!
//! ```text
//! fade by=letter duration=500 stagger=40; slide dy=30 by=word duration=600 easing=back
//! typewriter speed=30 cursor; wave amplitude=6 period=1200 after
//! ```
//!
//! | Efeito | O que faz | Parâmetros próprios |
//! |---|---|---|
//! | `fade` | aparece (com `out`, some) | — |
//! | `slide` | vem de um deslocamento | `dx`, `dy` (pontos; padrão `dy=24`) |
//! | `scale` | cresce (ou encolhe) até o tamanho | `from` (padrão 0) |
//! | `rotate` | gira até ficar reto | `from` (graus; padrão -90) |
//! | `typewriter` | máquina de escrever, uma letra por vez | `speed` (ms por letra), `cursor` |
//! | `color` | muda de cor até a final | `from` (`#ff0000`) |
//! | `tracking` | as letras chegam juntando | `from` (pontos entre letras; padrão 20) |
//! | `wave` | onda contínua | `amplitude`, `period` (ms) |
//! | `shake` | tremor contínuo | `amplitude` |
//! | `blink` | pisca | `period` (ms) |
//!
//! Comuns a todos: `by=letter|word|line|all` (a unidade animada), `duration` e `delay` (ms),
//! `stagger` (ms entre uma unidade e a seguinte), `order=normal|reverse|center|random`,
//! `easing=linear|in|out|in_out|back|bounce|elastic`, `out` (saída: do normal para o
//! efeito) e `after` (começa quando o efeito anterior termina, mais o `delay`). Sem `after`,
//! todos começam juntos (cada um com o seu `delay`): simultâneos ou em cascata, como quiser.
//!
//! Antes de começar, cada efeito segura o estado inicial dele (o `slide` deslocado, o
//! `scale` pequeno) sem esconder a letra: para "aparecer deslizando", junte `fade` e `slide`.

use std::cell::RefCell;

use aylis_core::prop::PropKind;
use aylis_core::{
    Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, MethodInfo, PaintCtx, PropValue, RectF, TextAlign, TextStyle,
    Ui, properties,
};

use crate::leaf_control;

/// A unidade que cada efeito anima.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    Letter,
    Word,
    Line,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Order {
    Normal,
    Reverse,
    Center,
    Random,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Curve {
    Linear,
    In,
    Out,
    InOut,
    Back,
    Bounce,
    Elastic,
}

impl Curve {
    fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Curve::Linear => t,
            Curve::In => t * t * t,
            Curve::Out => 1.0 - (1.0 - t).powi(3),
            Curve::InOut => {
                if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
            }
            Curve::Back => {
                let (c1, c3) = (1.70158, 2.70158);
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Curve::Bounce => {
                let (n, d) = (7.5625, 2.75);
                if t < 1.0 / d {
                    n * t * t
                } else if t < 2.0 / d {
                    let t = t - 1.5 / d;
                    n * t * t + 0.75
                } else if t < 2.5 / d {
                    let t = t - 2.25 / d;
                    n * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d;
                    n * t * t + 0.984375
                }
            }
            Curve::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = std::f32::consts::TAU / 3.0;
                    2f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Fade,
    Slide { dx: f32, dy: f32 },
    Scale { from: f32 },
    Rotate { from: f32 },
    Typewriter { cursor: bool },
    Color { from: Color },
    Tracking { from: f32 },
    Wave { amplitude: f32, period: f32 },
    Shake { amplitude: f32 },
    Blink { period: f32 },
}

impl Kind {
    /// Os contínuos não terminam: só começam (onda, tremor, piscar).
    fn continuous(&self) -> bool {
        matches!(self, Kind::Wave { .. } | Kind::Shake { .. } | Kind::Blink { .. })
    }
}

/// Um efeito de `effects`, já lido.
#[derive(Clone, Debug, PartialEq)]
struct Effect {
    kind: Kind,
    unit: Unit,
    order: Order,
    curve: Curve,
    delay: f32,
    duration: f32,
    stagger: f32,
    out: bool,
    after: bool,
}

/// Lê `effects`. Efeitos desconhecidos ficam de fora (o erro vai para o `errors`).
fn parse_effects(text: &str, errors: &mut Vec<String>) -> Vec<Effect> {
    let mut out = Vec::new();
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let mut words = part.split_whitespace();
        let Some(name) = words.next() else { continue };
        let mut params: Vec<(String, String)> = Vec::new();
        let mut flags: Vec<String> = Vec::new();
        for w in words {
            match w.split_once('=') {
                Some((k, v)) => params.push((k.to_ascii_lowercase(), v.to_string())),
                None => flags.push(w.to_ascii_lowercase()),
            }
        }
        let get = |k: &str| params.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());
        let num = |k: &str, default: f32| get(k).and_then(|v| v.trim_end_matches("ms").parse::<f32>().ok()).unwrap_or(default);
        let flag = |f: &str| flags.iter().any(|x| x == f);
        let kind = match name.to_ascii_lowercase().as_str() {
            "fade" => Kind::Fade,
            "slide" => Kind::Slide { dx: num("dx", 0.0), dy: num("dy", if get("dx").is_some() { 0.0 } else { 24.0 }) },
            "scale" | "zoom" => Kind::Scale { from: num("from", 0.0) },
            "rotate" => Kind::Rotate { from: num("from", -90.0) },
            "typewriter" | "type" => Kind::Typewriter { cursor: flag("cursor") },
            "color" => Kind::Color { from: get("from").and_then(Color::from_hex).unwrap_or(Color::rgb(255, 255, 255)) },
            "tracking" | "spacing" => Kind::Tracking { from: num("from", 20.0) },
            "wave" => Kind::Wave { amplitude: num("amplitude", 6.0), period: num("period", 1200.0).max(1.0) },
            "shake" => Kind::Shake { amplitude: num("amplitude", 2.0) },
            "blink" => Kind::Blink { period: num("period", 800.0).max(1.0) },
            other => {
                errors.push(format!("efeito desconhecido: {other}"));
                continue;
            }
        };
        let typewriter = matches!(kind, Kind::Typewriter { .. });
        let unit = match get("by").unwrap_or(if typewriter { "letter" } else { "letter" }) {
            "word" | "words" => Unit::Word,
            "line" | "lines" => Unit::Line,
            "all" | "text" => Unit::All,
            _ => Unit::Letter,
        };
        let order = match get("order").unwrap_or("normal") {
            "reverse" => Order::Reverse,
            "center" => Order::Center,
            "random" => Order::Random,
            _ => Order::Normal,
        };
        let curve = match get("easing").unwrap_or(if typewriter { "linear" } else { "out" }) {
            "linear" => Curve::Linear,
            "in" => Curve::In,
            "in_out" | "inout" => Curve::InOut,
            "back" => Curve::Back,
            "bounce" => Curve::Bounce,
            "elastic" => Curve::Elastic,
            _ => Curve::Out,
        };
        // A máquina de escrever: `speed` é o intervalo entre as letras.
        let (duration, stagger) = if typewriter { (0.0, num("speed", num("stagger", 40.0))) } else { (num("duration", 500.0).max(0.0), num("stagger", 0.0).max(0.0)) };
        out.push(Effect { kind, unit, order, curve, delay: num("delay", 0.0), duration, stagger, out: flag("out"), after: flag("after") });
    }
    out
}

/// Uma letra posicionada (no espaço do controle) e as unidades a que pertence.
#[derive(Clone, Debug)]
struct Glyph {
    text: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    /// Índice da letra (sem os espaços), da palavra e da linha.
    letter: usize,
    word: usize,
    line: usize,
}

/// A disposição do texto (refeita quando o texto, a fonte ou a largura mudam).
#[derive(Default)]
struct Layout {
    key: String,
    glyphs: Vec<Glyph>,
    letters: usize,
    words: usize,
    lines: usize,
}

/// Texto com efeitos animados (entrada, saída, contínuos), por letra, palavra ou linha. Veja
/// o módulo para a lista de efeitos.
pub struct AnimatedText {
    pub base: ControlBase,
    /// O texto animado (as quebras de linha valem).
    pub text: String,
    /// Os efeitos, separados por `;`: o nome e os parâmetros (`fade by=letter duration=500;
    /// wave amplitude=6 after`). Vazio: o texto parado.
    pub effects: String,
    /// Começa a animar quando aparece.
    pub autoplay: bool,
    /// Repete a animação ao terminar, depois de `loop_delay`.
    pub looping: bool,
    /// Espera entre uma volta e outra (ms).
    pub loop_delay: f32,
    /// O alinhamento do texto na horizontal.
    pub h_align: TextAlign,
    /// O alinhamento do texto na vertical.
    pub v_align: TextAlign,
    /// Quebra as linhas na largura do controle (as quebras do texto sempre valem).
    pub wrap: bool,
    parsed: Vec<Effect>,
    errors: Vec<String>,
    /// Quando começou (tempo da interface); `None`: parado no fim (ou no começo, sem tocar).
    started: RefCell<Option<f64>>,
    /// Parado num ponto (ms desde o começo), por `seek` ou `stop`.
    frozen: RefCell<Option<f32>>,
    playing_once: RefCell<bool>,
    finished: RefCell<bool>,
    layout: RefCell<Layout>,
    id_hint: std::cell::Cell<Option<ComponentId>>,
}

impl Default for AnimatedText {
    fn default() -> Self {
        Self {
            base: ControlBase::new(320.0, 80.0),
            text: String::new(),
            effects: String::new(),
            autoplay: true,
            looping: false,
            loop_delay: 1000.0,
            h_align: TextAlign::Leading,
            v_align: TextAlign::Center,
            wrap: true,
            parsed: Vec::new(),
            errors: Vec::new(),
            started: RefCell::new(None),
            frozen: RefCell::new(None),
            playing_once: RefCell::new(false),
            finished: RefCell::new(false),
            layout: RefCell::new(Layout::default()),
            id_hint: std::cell::Cell::new(None),
        }
    }
}

/// Os tempos de cada efeito: quando começa e quando a última unidade termina (ms).
fn schedule(effects: &[Effect], counts: (usize, usize, usize)) -> Vec<(f32, f32)> {
    let mut times = Vec::with_capacity(effects.len());
    let mut previous_end = 0.0f32;
    for e in effects {
        let start = if e.after { previous_end + e.delay } else { e.delay };
        let units = units_of(e.unit, counts);
        let end = if e.kind.continuous() { start } else { start + e.stagger * units.saturating_sub(1) as f32 + e.duration };
        times.push((start, end));
        previous_end = end;
    }
    times
}

fn units_of(unit: Unit, (letters, words, lines): (usize, usize, usize)) -> usize {
    match unit {
        Unit::Letter => letters,
        Unit::Word => words,
        Unit::Line => lines,
        Unit::All => 1,
    }
}

/// A posição de animação da unidade `i` de `n` na ordem pedida.
fn rank(order: Order, i: usize, n: usize) -> usize {
    match order {
        Order::Normal => i,
        Order::Reverse => n.saturating_sub(1 + i),
        Order::Center => {
            let mid = (n.saturating_sub(1)) as f32 / 2.0;
            ((i as f32 - mid).abs() * 2.0).round() as usize
        }
        Order::Random => {
            // Embaralha sempre igual (o mesmo texto anima do mesmo jeito).
            let mut x = (i as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            x ^= x >> 29;
            (x % n.max(1) as u64) as usize
        }
    }
}

/// O estado visual de uma letra num instante.
struct Look {
    opacity: f32,
    dx: f32,
    dy: f32,
    scale: f32,
    rotate: f32,
    color: Option<(Color, f32)>,
    spread: f32,
    hidden: bool,
}

impl AnimatedText {
    pub fn new(text: impl Into<String>, effects: impl Into<String>) -> Self {
        let mut me = Self { text: text.into(), effects: effects.into(), ..Default::default() };
        me.reparse();
        me
    }

    fn reparse(&mut self) {
        let mut errors = Vec::new();
        self.parsed = parse_effects(&self.effects, &mut errors);
        self.errors = errors;
    }

    /// Os erros de `effects` (efeitos desconhecidos).
    pub fn effect_errors(&self) -> &[String] {
        &self.errors
    }

    /// Quanto dura a animação inteira (ms; os efeitos contínuos contam só até começar).
    pub fn duration_ms(&self) -> f32 {
        let l = self.layout.borrow();
        let counts = (l.letters.max(1), l.words.max(1), l.lines.max(1));
        schedule(&self.parsed, counts).iter().map(|t| t.1).fold(0.0, f32::max)
    }

    /// Toca do começo.
    pub fn play(ui: &mut Ui, id: ComponentId) {
        let now = ui.time();
        if let Some(t) = ui.get::<AnimatedText>(id) {
            *t.started.borrow_mut() = Some(now);
            *t.frozen.borrow_mut() = None;
            *t.finished.borrow_mut() = false;
            *t.playing_once.borrow_mut() = true;
        }
        ui.request_repaint();
    }

    /// Para onde está.
    pub fn stop(ui: &mut Ui, id: ComponentId) {
        let now = ui.time();
        if let Some(t) = ui.get::<AnimatedText>(id) {
            let at = t.started.borrow().map(|s| ((now - s) * 1000.0) as f32);
            if let Some(at) = at {
                *t.frozen.borrow_mut() = Some(at);
            }
            *t.started.borrow_mut() = None;
        }
        ui.request_repaint();
    }

    /// Mostra o instante `ms` da animação (parado).
    pub fn seek(ui: &mut Ui, id: ComponentId, ms: f32) {
        if let Some(t) = ui.get::<AnimatedText>(id) {
            *t.started.borrow_mut() = None;
            *t.frozen.borrow_mut() = Some(ms.max(0.0));
        }
        ui.request_repaint();
    }

    pub fn is_playing(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<AnimatedText>(id).is_some_and(|t| t.started.borrow().is_some())
    }

    /// Dispõe as letras na área (linhas pelas quebras do texto e, com `wrap`, pela largura).
    fn ensure_layout(&self, canvas: &mut dyn Canvas, style: &TextStyle, area: RectF) {
        let key = format!("{}|{:?}|{}|{:?}|{:?}|{}", self.text, style.font, area.width.round(), self.h_align, self.v_align, self.wrap);
        let mut l = self.layout.borrow_mut();
        if l.key == key {
            return;
        }
        let measure = |canvas: &mut dyn Canvas, s: &str| canvas.measure_text(s, style);
        let line_height = measure(canvas, "Ág").height.max(style.font.size * 1.2);
        // As linhas: as do texto, quebradas por palavra na largura.
        let mut lines: Vec<Vec<String>> = Vec::new();
        for raw in self.text.split('\n') {
            let mut current: Vec<String> = Vec::new();
            let mut width = 0.0;
            let space = measure(canvas, " ").width;
            for word in raw.split(' ') {
                let w = measure(canvas, word).width;
                if self.wrap && !current.is_empty() && width + space + w > area.width {
                    lines.push(std::mem::take(&mut current));
                    width = 0.0;
                }
                width += if current.is_empty() { w } else { space + w };
                current.push(word.to_string());
            }
            lines.push(current);
        }
        let mut glyphs = Vec::new();
        let (mut letter, mut word) = (0usize, 0usize);
        let total_height = line_height * lines.len() as f32;
        let top = match self.v_align {
            TextAlign::Leading => area.y,
            TextAlign::Trailing => area.bottom() - total_height,
            _ => area.y + (area.height - total_height) / 2.0,
        };
        for (li, words) in lines.iter().enumerate() {
            let text = words.join(" ");
            let line_width = measure(canvas, &text).width;
            let left = match self.h_align {
                TextAlign::Center => area.x + (area.width - line_width) / 2.0,
                TextAlign::Trailing => area.right() - line_width,
                _ => area.x,
            };
            let y = top + li as f32 * line_height;
            // A posição de cada letra: a largura do texto até ela (o espaçamento real da fonte).
            let mut prefix = String::new();
            for (wi, w) in words.iter().enumerate() {
                if wi > 0 {
                    prefix.push(' ');
                }
                for ch in w.chars() {
                    let x = left + measure(canvas, &prefix).width;
                    prefix.push(ch);
                    let width = left + measure(canvas, &prefix).width - x;
                    glyphs.push(Glyph { text: ch.to_string(), x, y, width, height: line_height, letter, word, line: li });
                    letter += 1;
                }
                if !w.is_empty() {
                    word += 1;
                }
            }
        }
        *l = Layout { key, glyphs, letters: letter, words: word, lines: lines.len() };
    }

    /// Como a letra `g` aparece no instante `ms`.
    fn look(&self, g: &Glyph, ms: f32, times: &[(f32, f32)], counts: (usize, usize, usize)) -> Look {
        let mut look = Look { opacity: 1.0, dx: 0.0, dy: 0.0, scale: 1.0, rotate: 0.0, color: None, spread: 0.0, hidden: false };
        for (e, &(start, _)) in self.parsed.iter().zip(times) {
            let n = units_of(e.unit, counts);
            let index = match e.unit {
                Unit::Letter => g.letter,
                Unit::Word => g.word,
                Unit::Line => g.line,
                Unit::All => 0,
            };
            let begin = start + e.stagger * rank(e.order, index, n) as f32;
            let local = ms - begin;
            // Contínuos: valem a partir do começo da unidade.
            match e.kind {
                Kind::Wave { amplitude, period } => {
                    if local >= 0.0 {
                        let phase = (local / period + g.letter as f32 * 0.08) * std::f32::consts::TAU;
                        look.dy += phase.sin() * amplitude;
                    }
                    continue;
                }
                Kind::Shake { amplitude } => {
                    if local >= 0.0 {
                        let s = (local * 0.07 + g.letter as f32 * 1.7).sin();
                        let c = (local * 0.09 + g.letter as f32 * 2.3).cos();
                        look.dx += s * amplitude;
                        look.dy += c * amplitude;
                    }
                    continue;
                }
                Kind::Blink { period } => {
                    if local >= 0.0 && (local / (period / 2.0)) as i64 % 2 == 1 {
                        look.opacity *= 0.0;
                    }
                    continue;
                }
                Kind::Typewriter { .. } => {
                    // Aparece de uma vez quando chega a vez (ou some, com `out`).
                    let shown = local >= 0.0;
                    if shown == e.out {
                        look.hidden = true;
                    }
                    continue;
                }
                _ => {}
            }
            let p = if e.duration <= 0.0 { if local >= 0.0 { 1.0 } else { 0.0 } } else { (local / e.duration).clamp(0.0, 1.0) };
            // `k`: 0 = no efeito (o começo da entrada), 1 = normal. Na saída, ao contrário.
            let eased = e.curve.apply(p);
            let k = if e.out { 1.0 - e.curve.apply(p) } else { eased };
            let away = 1.0 - k;
            match e.kind {
                Kind::Fade => look.opacity *= k.clamp(0.0, 1.0),
                Kind::Slide { dx, dy } => {
                    look.dx += dx * away;
                    look.dy += dy * away;
                }
                Kind::Scale { from } => look.scale *= from + (1.0 - from) * k,
                Kind::Rotate { from } => look.rotate += from * away,
                Kind::Color { from } => look.color = Some((from, k.clamp(0.0, 1.0))),
                Kind::Tracking { from } => look.spread += from * away,
                _ => {}
            }
        }
        look
    }

    /// O instante da animação agora (ms) e se ainda pede quadros.
    fn clock(&self, now: f64, total: f32) -> (f32, bool) {
        if let Some(at) = *self.frozen.borrow() {
            return (at, false);
        }
        let started = *self.started.borrow();
        match started {
            Some(s) => {
                let mut ms = ((now - s) * 1000.0) as f32;
                let continuous = self.parsed.iter().any(|e| e.kind.continuous());
                if ms >= total && self.looping {
                    let cycle = total + self.loop_delay.max(0.0);
                    if cycle > 0.0 {
                        ms %= cycle;
                    }
                    return (ms.min(total), true);
                }
                if ms >= total && !continuous {
                    return (total, false);
                }
                (ms, true)
            }
            // Parado sem ter tocado: o fim (o texto inteiro, como no designer).
            None => (total, false),
        }
    }
}

impl Component for AnimatedText {
    leaf_control!("AnimatedText");

    properties! {
        text: String => "Appearance",
        effects: String => "Behavior",
        autoplay: bool => "Behavior",
        looping: bool => "Behavior",
        loop_delay: f32 => "Behavior",
        h_align: TextAlign => "Appearance",
        v_align: TextAlign => "Appearance",
        wrap: bool => "Appearance";
        events: on_finish;
        on_set: on_set
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("play", &[], Some(PropKind::Bool)),
            MethodInfo::new("stop", &[], Some(PropKind::Bool)),
            MethodInfo::new("seek", &[PropKind::Float], Some(PropKind::Bool)).params(&["seconds"]),
            MethodInfo::new("is_playing", &[], Some(PropKind::Bool)),
            MethodInfo::new("duration", &[], Some(PropKind::Float)),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let now = ctx.ui.time();
        match method {
            "play" => {
                *self.started.borrow_mut() = Some(now);
                *self.frozen.borrow_mut() = None;
                *self.finished.borrow_mut() = false;
                *self.playing_once.borrow_mut() = true;
            }
            "stop" => {
                let at = self.started.borrow().map(|s| ((now - s) * 1000.0) as f32);
                if let Some(at) = at {
                    *self.frozen.borrow_mut() = Some(at);
                }
                *self.started.borrow_mut() = None;
            }
            "seek" => {
                let ms = args.first().and_then(|v| v.to_f64().ok()).unwrap_or(0.0) as f32;
                *self.started.borrow_mut() = None;
                *self.frozen.borrow_mut() = Some(ms.max(0.0));
            }
            "is_playing" => return Ok(PropValue::Bool(self.started.borrow().is_some())),
            "duration" => return Ok(PropValue::Float(self.duration_ms() as f64)),
            _ => return Err(format!("AnimatedText não tem o método '{method}'")),
        }
        ctx.repaint();
        Ok(PropValue::Bool(true))
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.id_hint.set(Some(ctx.id));
        let style = ctx.text_style();
        self.ensure_layout(canvas, &style, ctx.rect);
        // Começa sozinho na primeira pintura (fora do designer).
        if self.autoplay && !ctx.design && !*self.playing_once.borrow() && self.frozen.borrow().is_none() {
            *self.started.borrow_mut() = Some(ctx.time);
            *self.playing_once.borrow_mut() = true;
        }
        let layout = self.layout.borrow();
        let counts = (layout.letters.max(1), layout.words.max(1), layout.lines.max(1));
        let times = schedule(&self.parsed, counts);
        let total = times.iter().map(|t| t.1).fold(0.0, f32::max);
        let (ms, running) = if ctx.design { (total, false) } else { self.clock(ctx.time, total) };
        if running {
            ctx.request_frame();
        } else if self.started.borrow().is_some() && !*self.finished.borrow() {
            // Chegou ao fim: avisa uma vez.
            *self.finished.borrow_mut() = true;
            *self.started.borrow_mut() = None;
            *self.frozen.borrow_mut() = Some(total);
            if let Some(id) = self.id_hint.get() {
                ctx.ui.poster().post(move |ui| {
                    ui.fire(id, "on_finish", &EventArgs::None);
                });
            }
        }
        let base_color = style.color;
        let mut last: Option<(f32, f32, f32)> = None;
        for g in &layout.glyphs {
            let look = self.look(g, ms, &times, counts);
            if !look.hidden {
                last = Some((g.x + g.width, g.y, g.height));
            }
            if look.hidden || look.opacity <= 0.001 || g.text == " " {
                continue;
            }
            let mut s = style.clone();
            s.h_align = TextAlign::Leading;
            s.v_align = TextAlign::Leading;
            s.wrap = false;
            let color = match look.color {
                Some((from, k)) => from.lerp(base_color, k),
                None => base_color,
            };
            s.color = color.with_opacity(look.opacity.clamp(0.0, 1.0) * color.a as f32 / 255.0);
            // O espaçamento a mais (tracking) afasta do centro do texto.
            let spread = look.spread * (g.letter as f32 - layout.letters as f32 / 2.0);
            let (cx, cy) = (g.x + g.width / 2.0 + look.dx + spread, g.y + g.height / 2.0 + look.dy);
            canvas.save();
            canvas.translate(cx, cy);
            if look.rotate != 0.0 {
                canvas.rotate(look.rotate);
            }
            if (look.scale - 1.0).abs() > 0.001 {
                canvas.scale(look.scale.max(0.001), look.scale.max(0.001));
            }
            canvas.draw_text(&g.text, RectF::new(-g.width / 2.0, -g.height / 2.0, g.width + 4.0, g.height), &s);
            canvas.restore();
        }
        // O cursor da máquina de escrever, piscando depois da última letra visível.
        let cursor = self.parsed.iter().any(|e| matches!(e.kind, Kind::Typewriter { cursor: true }));
        if cursor && !ctx.design {
            let (x, y, h) = last.unwrap_or_else(|| layout.glyphs.first().map_or((ctx.rect.x, ctx.rect.y, style.font.size), |g| (g.x, g.y, g.height)));
            if (ctx.time * 2.0) as i64 % 2 == 0 {
                canvas.fill_rect(RectF::new(x + 1.0, y + h * 0.15, 2.0, h * 0.7), 0.0, &aylis_core::Brush::Solid(base_color));
            }
            ctx.request_frame_at(ctx.time + 0.5);
        }
    }
}

impl AnimatedText {
    fn on_set(&mut self, name: &str) {
        match name {
            "effects" => {
                self.reparse();
                *self.finished.borrow_mut() = false;
            }
            "text" => {
                self.layout.borrow_mut().key.clear();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_parse_with_defaults_and_errors() {
        let mut errors = Vec::new();
        let e = parse_effects("fade by=word duration=300 stagger=50; slide dy=10 after delay=100 easing=bounce; zzz; wave", &mut errors);
        assert_eq!(e.len(), 3);
        assert_eq!(errors, ["efeito desconhecido: zzz"]);
        assert_eq!((e[0].unit, e[0].duration, e[0].stagger), (Unit::Word, 300.0, 50.0));
        assert!(e[1].after && e[1].delay == 100.0 && e[1].curve == Curve::Bounce);
        assert_eq!(e[1].kind, Kind::Slide { dx: 0.0, dy: 10.0 });
        // 3 palavras: o fade vai até 2*50 + 300 = 400; o slide começa 100 depois e dura 500.
        let t = schedule(&e, (10, 3, 1));
        assert_eq!(t[0], (0.0, 400.0));
        assert_eq!(t[1], (500.0, 500.0 + 9.0 * 0.0 + 500.0));
        assert_eq!(t[2], (0.0, 0.0), "contínuo: só começa");
    }

    #[test]
    fn curves_start_at_zero_and_end_at_one() {
        for c in [Curve::Linear, Curve::In, Curve::Out, Curve::InOut, Curve::Back, Curve::Bounce, Curve::Elastic] {
            assert!(c.apply(0.0).abs() < 1e-4, "{c:?}");
            assert!((c.apply(1.0) - 1.0).abs() < 1e-4, "{c:?}");
        }
        assert_eq!(rank(Order::Reverse, 0, 5), 4);
        assert_eq!(rank(Order::Center, 2, 5), 0);
    }
}
