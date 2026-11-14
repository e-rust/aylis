//! Animações de propriedades (o "Animator").
//!
//! Qualquer propriedade numérica ou de cor pode ser animada pelo nome, porque o
//! sistema de propriedades permite ler e gravar qualquer uma:
//!
//! - por código: [`Ui::animate`];
//! - por componente filho, como no FMX: [`PropertyAnimation`] (`FloatAnimation` /
//!   `ColorAnimation`), disparado por um [`Trigger`] (hover, pressed, focused, checked)
//!   do controle pai, sem código.
//!
//! A plataforma chama [`Ui::tick`] a cada frame enquanto [`Ui::is_animating`] for
//! verdadeiro; parado, nada roda.

use crate::color::Color;
use crate::component::{Component, ComponentId};
use crate::error::{Error, Result};
use crate::prop::{PropEnum, PropInfo, PropKind, PropType, PropValue};
use crate::ui::Ui;

/// Curva de aceleração (suavização). Como propriedade, é gravada pelo nome (`out_quad`,
/// ou os do CSS: `ease`, `ease-in`...) ou como `cubic-bezier(x1, y1, x2, y2)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Easing {
    #[default]
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    OutCubic,
    InOutCubic,
    /// Passa um pouco do destino e volta (efeito "mola" leve).
    OutBack,
    /// Curva de Bézier cúbica de (0,0) a (1,1) com os dois pontos de controle, como a
    /// `cubic-bezier()` do CSS. O `x` dos pontos fica entre 0 e 1; o `y` pode passar.
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    pub const ALL: [Easing; 7] = [
        Easing::Linear,
        Easing::InQuad,
        Easing::OutQuad,
        Easing::InOutQuad,
        Easing::OutCubic,
        Easing::InOutCubic,
        Easing::OutBack,
    ];

    /// Aplica a curva a `t` (0..1).
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::InQuad => t * t,
            Easing::OutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::OutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::OutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::CubicBezier(x1, y1, x2, y2) => cubic_bezier(x1.clamp(0.0, 1.0), y1, x2.clamp(0.0, 1.0), y2, t),
        }
    }

    /// Nomes das curvas prontas, na ordem de [`Easing::ALL`].
    pub const NAMES: &'static [&'static str] =
        &["linear", "in_quad", "out_quad", "in_out_quad", "out_cubic", "in_out_cubic", "out_back"];

    /// As curvas do CSS (`ease`, `ease-in`, `ease-out`, `ease-in-out`) com os pontos delas.
    pub const CSS: [(&'static str, [f32; 4]); 4] = [
        ("ease", [0.25, 0.1, 0.25, 1.0]),
        ("ease-in", [0.42, 0.0, 1.0, 1.0]),
        ("ease-out", [0.0, 0.0, 0.58, 1.0]),
        ("ease-in-out", [0.42, 0.0, 0.58, 1.0]),
    ];

    /// Lê um nome (`out_quad`, `OutQuad`, `out-quad`), um nome do CSS ou
    /// `cubic-bezier(x1, y1, x2, y2)`.
    pub fn parse(text: &str) -> Option<Easing> {
        let text = text.trim();
        if let Some(args) = text.strip_prefix("cubic-bezier(").and_then(|a| a.strip_suffix(')')) {
            let v: Vec<f32> = args.split(',').map(|n| n.trim().parse::<f32>()).collect::<Result<_, _>>().ok()?;
            let [x1, y1, x2, y2] = v[..] else { return None };
            return Some(Easing::CubicBezier(x1, y1, x2, y2));
        }
        if let Some((_, [a, b, c, d])) = Easing::CSS.iter().find(|(n, _)| n.eq_ignore_ascii_case(text)) {
            return Some(Easing::CubicBezier(*a, *b, *c, *d));
        }
        let plain: String = text.chars().filter(|c| *c != '_' && *c != '-').collect();
        Easing::NAMES
            .iter()
            .position(|n| n.replace('_', "").eq_ignore_ascii_case(&plain))
            .map(|i| Easing::ALL[i])
    }

    /// O texto que [`Easing::parse`] lê.
    pub fn to_spec(self) -> String {
        match self {
            Easing::CubicBezier(a, b, c, d) => {
                if let Some((name, _)) = Easing::CSS.iter().find(|(_, p)| *p == [a, b, c, d]) {
                    return name.to_string();
                }
                let n = |v: f32| format!("{}", (v * 1000.0).round() / 1000.0);
                format!("cubic-bezier({}, {}, {}, {})", n(a), n(b), n(c), n(d))
            }
            named => Easing::NAMES[Easing::ALL.iter().position(|e| *e == named).unwrap_or(0)].to_string(),
        }
    }

    /// Os pontos de controle da curva como Bézier cúbica (as prontas são aproximações das
    /// funções, as mesmas do easings.net). É o que o editor da curva abre para ajustar.
    pub fn to_bezier(self) -> [f32; 4] {
        match self {
            Easing::Linear => [0.0, 0.0, 1.0, 1.0],
            Easing::InQuad => [0.55, 0.085, 0.68, 0.53],
            Easing::OutQuad => [0.25, 0.46, 0.45, 0.94],
            Easing::InOutQuad => [0.455, 0.03, 0.515, 0.955],
            Easing::OutCubic => [0.215, 0.61, 0.355, 1.0],
            Easing::InOutCubic => [0.645, 0.045, 0.355, 1.0],
            Easing::OutBack => [0.175, 0.885, 0.32, 1.275],
            Easing::CubicBezier(a, b, c, d) => [a, b, c, d],
        }
    }
}

/// `y` da Bézier (0,0)-(x1,y1)-(x2,y2)-(1,1) no ponto em que `x` = `x`: acha o parâmetro
/// por Newton e, se não convergir, por bisseção.
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
    let curve = |a: f32, b: f32, s: f32| 3.0 * a * s * (1.0 - s) * (1.0 - s) + 3.0 * b * s * s * (1.0 - s) + s * s * s;
    let slope = |a: f32, b: f32, s: f32| 3.0 * a * (1.0 - s) * (1.0 - s) + 6.0 * (b - a) * s * (1.0 - s) + 3.0 * (1.0 - b) * s * s;
    let mut s = x;
    for _ in 0..8 {
        let err = curve(x1, x2, s) - x;
        if err.abs() < 1e-5 {
            return curve(y1, y2, s);
        }
        let d = slope(x1, x2, s);
        if d.abs() < 1e-6 {
            break;
        }
        s -= err / d;
    }
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    s = x;
    for _ in 0..40 {
        let v = curve(x1, x2, s);
        if (v - x).abs() < 1e-5 {
            break;
        }
        if v < x { lo = s } else { hi = s }
        s = (lo + hi) / 2.0;
    }
    curve(y1, y2, s)
}

impl std::str::FromStr for Easing {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Easing::parse(s).ok_or_else(|| {
            Error::invalid(format!("curva '{s}' inválida; use um de: {}, ease, ease-in, ease-out, ease-in-out ou cubic-bezier(x1, y1, x2, y2)", Easing::NAMES.join(", ")))
        })
    }
}

impl PropType for Easing {
    const KIND: PropKind = PropKind::Easing;
    fn to_value(&self) -> PropValue {
        PropValue::String(self.to_spec())
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_str()?.parse()
    }
}

/// Estado do controle pai que dispara uma animação.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Trigger {
    /// Só por código ([`Ui::start_animation`]).
    #[default]
    None,
    Hover,
    Pressed,
    Focused,
    Checked,
    /// Ao aparecer na tela (o controle entrou na área visível, rolando ou não): uma vez só,
    /// como o "scroll reveal" da web.
    Appear,
}

impl PropEnum for Trigger {
    const NAMES: &'static [&'static str] = &["none", "hover", "pressed", "focused", "checked", "appear"];

    fn from_index(index: usize) -> Option<Self> {
        [Trigger::None, Trigger::Hover, Trigger::Pressed, Trigger::Focused, Trigger::Checked, Trigger::Appear]
            .get(index)
            .copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AnimValue {
    Float(f64),
    Color(Color),
}

impl AnimValue {
    fn from_prop(v: &PropValue) -> Option<AnimValue> {
        match v {
            PropValue::Float(_) | PropValue::Int(_) => v.to_f64().ok().map(AnimValue::Float),
            PropValue::Color(_) | PropValue::String(_) => v.to_color().ok().map(AnimValue::Color),
            _ => None,
        }
    }

    fn to_prop(self) -> PropValue {
        match self {
            AnimValue::Float(v) => PropValue::Float(v),
            AnimValue::Color(c) => PropValue::Color(c),
        }
    }

    fn lerp(self, to: AnimValue, t: f32) -> AnimValue {
        match (self, to) {
            (AnimValue::Float(a), AnimValue::Float(b)) => AnimValue::Float(a + (b - a) * t as f64),
            (AnimValue::Color(a), AnimValue::Color(b)) => AnimValue::Color(a.lerp(b, t)),
            (_, b) => b,
        }
    }
}

/// `0:0; 0.6:1.15; 1:1` → os passos em ordem; `None` se vazio ou mal escrito.
pub fn parse_keyframes(text: &str) -> Option<Vec<(f32, f64)>> {
    let mut steps = Vec::new();
    for item in text.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        let (t, v) = item.split_once(':')?;
        let t: f32 = t.trim().replace(',', ".").parse().ok()?;
        let v: f64 = v.trim().replace(',', ".").parse().ok()?;
        steps.push((t.clamp(0.0, 1.0), v));
    }
    if steps.len() < 2 {
        return None;
    }
    steps.sort_by(|a, b| a.0.total_cmp(&b.0));
    Some(steps)
}

/// O valor da sequência no tempo `t` (0 a 1), em linha reta entre os passos.
fn sample_keyframes(steps: &[(f32, f64)], t: f32) -> f64 {
    let Some(first) = steps.first() else { return 0.0 };
    if t <= first.0 {
        return first.1;
    }
    for w in steps.windows(2) {
        let ((t0, v0), (t1, v1)) = (w[0], w[1]);
        if t <= t1 {
            let f = if t1 > t0 { ((t - t0) / (t1 - t0)) as f64 } else { 1.0 };
            return v0 + (v1 - v0) * f;
        }
    }
    steps.last().map_or(0.0, |s| s.1)
}

/// Uma animação em andamento.
#[derive(Clone, Debug)]
pub(crate) struct Running {
    pub(crate) target: ComponentId,
    property: String,
    from: AnimValue,
    to: AnimValue,
    /// Quando começa (o atraso já somado).
    start: f64,
    duration: f64,
    easing: Easing,
    auto_reverse: bool,
    looping: bool,
    /// Os passos (fração do tempo, valor) de uma sequência; vazio: de `from` a `to`.
    keyframes: std::rc::Rc<[(f32, f64)]>,
    /// Componente de animação que iniciou esta (se houver).
    pub(crate) owner: Option<ComponentId>,
}

/// Opções de [`Ui::animate`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationOptions {
    pub duration_ms: f32,
    /// Espera antes de começar (o valor fica no inicial enquanto isso).
    pub delay_ms: f32,
    pub easing: Easing,
    /// Ao chegar no destino, volta ao início.
    pub auto_reverse: bool,
    /// Repete para sempre (até [`Ui::stop_animations`]).
    pub looping: bool,
}

impl Default for AnimationOptions {
    fn default() -> Self {
        Self { duration_ms: 200.0, delay_ms: 0.0, easing: Easing::OutQuad, auto_reverse: false, looping: false }
    }
}

impl AnimationOptions {
    pub fn new(duration_ms: f32, easing: Easing) -> Self {
        Self { duration_ms, easing, ..Default::default() }
    }
}

impl Ui {
    /// Anima uma propriedade (numérica ou cor) do valor atual até `to`.
    ///
    /// Uma animação anterior da mesma propriedade é substituída e a nova parte do
    /// valor em que a anterior parou, então não há saltos.
    pub fn animate(
        &mut self,
        id: ComponentId,
        property: &str,
        to: impl Into<PropValue>,
        options: AnimationOptions,
    ) -> Result<()> {
        let current = self.get_prop(id, property)?;
        self.animate_from_to(id, property, current, to.into(), options, None)
    }

    fn animate_from_to(
        &mut self,
        id: ComponentId,
        property: &str,
        from: PropValue,
        to: PropValue,
        options: AnimationOptions,
        owner: Option<ComponentId>,
    ) -> Result<()> {
        let invalid = || Error::invalid("só propriedades numéricas ou de cor podem ser animadas").with_property(property);
        // Cor com nome (`success`, `text/85`): a do tema em uso.
        let named = |v: PropValue| match v {
            PropValue::String(s) if crate::palette::is_named(&s) => self.theme.resolve_color(&s).map_or(PropValue::String(s), PropValue::Color),
            v => v,
        };
        let (from, to) = (named(from), named(to));
        let from = AnimValue::from_prop(&from).ok_or_else(invalid)?;
        let to = AnimValue::from_prop(&to).ok_or_else(invalid)?;
        self.animations.retain(|a| !(a.target == id && a.property == property));
        self.animations.push(Running {
            target: id,
            property: property.to_string(),
            from,
            to,
            start: self.time + (options.delay_ms.max(0.0) / 1000.0) as f64,
            duration: (options.duration_ms.max(0.0) / 1000.0) as f64,
            easing: options.easing,
            auto_reverse: options.auto_reverse,
            looping: options.looping,
            keyframes: std::rc::Rc::from(Vec::new()),
            owner,
        });
        self.request_repaint();
        Ok(())
    }

    /// Para as animações das propriedades do componente (o valor fica onde está).
    pub fn stop_animations(&mut self, id: ComponentId) {
        self.animations.retain(|a| a.target != id && a.owner != Some(id));
    }

    /// Chama [`Component::frame`] em cada quadro até ele devolver `false` (a rolagem com
    /// deslize, que anda no tempo do quadro e não num timer fixo, e acompanha o monitor).
    pub fn request_frames(&mut self, id: ComponentId) {
        if !self.frame_hooks.contains(&id) {
            self.frame_hooks.push(id);
        }
        // Um quadro a pintar, sem dizer onde: o componente marca a área dele no `frame`.
        self.repaint_requested = true;
    }

    fn tick_frame_hooks(&mut self) {
        for id in std::mem::take(&mut self.frame_hooks) {
            let more = self.with_component(id, |c, ctx| c.frame(ctx)).unwrap_or(false);
            if more && self.contains(id) && !self.frame_hooks.contains(&id) {
                self.frame_hooks.push(id);
            }
        }
    }

    /// `id` está com o lugar ou a transparência animando (`translate_x`, `translate_y`,
    /// `opacity`): o desenho dele, igual de um quadro para o outro, vai numa camada guardada.
    pub(crate) fn animates_placement(&self, id: ComponentId) -> bool {
        self.placement_movers().any(|m| m == id)
    }

    /// Os controles com o lugar ou a transparência animando (veja [`Ui::animates_placement`]).
    pub(crate) fn placement_movers(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.animations.iter().filter(|a| matches!(a.property.as_str(), "translate_x" | "translate_y" | "opacity")).map(|a| a.target)
    }

    /// Há animação ou transição de estilo em andamento.
    pub fn is_animating(&self) -> bool {
        !self.animations.is_empty() || !self.frame_hooks.is_empty() || self.frame_requests.borrow().iter().any(|(id, f)| f.0 && self.contains(*id)) || self.is_flinging() || !self.transitions.is_empty()
    }

    /// Avança as animações até o tempo atual ([`Ui::time`]). A plataforma chama isto antes
    /// de cada pintura.
    pub fn tick(&mut self) {
        self.tick_flings();
        self.tick_busy();
        self.tick_transitions();
        self.check_appear();
        self.tick_frame_hooks();
        if self.animations.is_empty() {
            return;
        }
        let _profile = crate::profile::scope(crate::profile::Lane::Animation, || format!("{} animações", self.animations.len()));
        let now = self.time;
        let mut updates = Vec::with_capacity(self.animations.len());
        self.animations.retain_mut(|a| {
            let span = if a.auto_reverse { a.duration * 2.0 } else { a.duration };
            let mut elapsed = (now - a.start).max(0.0);
            let finished = !a.looping && elapsed >= span;
            if a.looping && span > 0.0 {
                elapsed %= span;
            }
            let mut t = if a.duration > 0.0 { (elapsed / a.duration) as f32 } else { 1.0 };
            if finished {
                t = if a.auto_reverse { 0.0 } else { 1.0 };
            } else if a.auto_reverse && t > 1.0 {
                t = 2.0 - t;
            }
            let eased = a.easing.apply(t.min(1.0));
            let value = if a.keyframes.is_empty() { a.from.lerp(a.to, eased) } else { AnimValue::Float(sample_keyframes(&a.keyframes, eased)) };
            updates.push((a.target, a.property.clone(), value.to_prop()));
            !finished
        });
        for (id, property, value) in updates {
            // A propriedade pode ter sumido (componente liberado); só ignora.
            let _ = self.set_prop(id, &property, value);
        }
    }

    /// Inicia uma [`PropertyAnimation`] (normalmente as de `trigger: none`).
    pub fn start_animation(&mut self, animation: ComponentId) -> Result<()> {
        self.play_animation(animation, true)
    }

    /// Dispara as animações filhas de `control` que esperam este `trigger`: ao ativar,
    /// vão para `stop_value`; ao desativar, voltam ao valor original.
    pub(crate) fn run_triggers(&mut self, control: ComponentId, trigger: Trigger, active: bool) {
        let animations: Vec<ComponentId> = self
            .children(control)
            .iter()
            .copied()
            .filter(|&c| self.get::<PropertyAnimation>(c).is_some_and(|a| a.enabled && a.trigger == trigger))
            .collect();
        for animation in animations {
            let _ = self.play_animation(animation, active);
        }
    }

    fn play_animation(&mut self, animation: ComponentId, forward: bool) -> Result<()> {
        let target = self.parent(animation).ok_or(Error::InvalidId)?;
        let a = self.get::<PropertyAnimation>(animation).ok_or(Error::InvalidId)?.clone();
        let current = self.get_prop(target, &a.property)?;
        let original = a.original.clone().unwrap_or_else(|| current.clone());
        if a.original.is_none() {
            if let Some(anim) = self.get_mut::<PropertyAnimation>(animation) {
                anim.original = Some(original.clone());
            }
        }
        let (from, to) = if forward {
            let from = if a.start_value == PropValue::Null { current } else { a.start_value.clone() };
            (from, a.stop_value.clone())
        } else {
            (current, original)
        };
        let options = AnimationOptions {
            duration_ms: a.duration,
            delay_ms: if forward { a.delay } else { 0.0 },
            easing: a.easing,
            auto_reverse: forward && a.auto_reverse,
            looping: forward && a.looping,
        };
        self.animate_from_to(target, &a.property, from, to, options, Some(animation))?;
        // Sequência de passos (só na ida): os valores vêm dela.
        if forward {
            if let Some(steps) = parse_keyframes(&a.keyframes) {
                if let Some(run) = self.animations.iter_mut().find(|r| r.target == target && r.property == a.property) {
                    run.keyframes = std::rc::Rc::from(steps);
                }
            }
        }
        Ok(())
    }

    /// As animações `appear` cujo controle entrou na área visível rodam (uma vez). A lista
    /// das que esperam é refeita quando a árvore muda; com nenhuma, não custa nada.
    pub(crate) fn check_appear(&mut self) {
        if self.appear_waiting.is_none() {
            let waiting: Vec<ComponentId> = self
                .components()
                .filter(|&c| self.get::<PropertyAnimation>(c).is_some_and(|a| a.enabled && a.trigger == Trigger::Appear && !a.appeared))
                .collect();
            self.appear_waiting = Some(waiting);
        }
        let Some(waiting) = self.appear_waiting.clone() else { return };
        if waiting.is_empty() || self.design_mode {
            return;
        }
        let mut left = Vec::new();
        for animation in waiting {
            let Some(target) = self.parent(animation) else { continue };
            if self.visible_fraction(target) >= 0.15 {
                if let Some(a) = self.get_mut::<PropertyAnimation>(animation) {
                    a.appeared = true;
                }
                let _ = self.play_animation(animation, true);
            } else {
                left.push(animation);
            }
        }
        self.appear_waiting = Some(left);
    }

    /// Quanto do controle está à vista (0 a 1): o retângulo dele cortado pelo de cada
    /// contêiner acima (as rolagens) e pelo form aberto.
    fn visible_fraction(&self, id: ComponentId) -> f32 {
        let root = self.root_of(id);
        if !self.is_open(root) {
            return 0.0;
        }
        let Some(r) = self.absolute_rect(id) else { return 0.0 };
        if !self.cb(id).is_some_and(|cb| cb.visible()) {
            return 0.0;
        }
        let area = (r.width * r.height).max(1.0);
        let (mut x0, mut y0, mut x1, mut y1) = (r.x, r.y, r.right(), r.bottom());
        let mut current = self.parent(id);
        while let Some(p) = current {
            if let Some(c) = self.absolute_rect(p) {
                (x0, y0, x1, y1) = (x0.max(c.x), y0.max(c.y), x1.min(c.right()), y1.min(c.bottom()));
            }
            if !self.cb(p).is_none_or(|cb| cb.visible()) {
                return 0.0;
            }
            current = self.parent(p);
        }
        ((x1 - x0).max(0.0) * (y1 - y0).max(0.0)) / area
    }
}

/// Animação declarada como componente filho do controle animado (como `TFloatAnimation`
/// e `TColorAnimation`). Funciona sem código, inclusive vinda do arquivo de layout.
///
/// ```ignore
/// let anim = ui.add(PropertyAnimation::color("fill", Color::rgb(255, 0, 0)), botao);
/// ui.set_prop(anim, "trigger", "hover")?;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyAnimation {
    color: bool,
    pub property: String,
    /// Valor inicial; `Null` = o valor atual da propriedade.
    pub start_value: PropValue,
    pub stop_value: PropValue,
    /// Milissegundos.
    pub duration: f32,
    /// Espera antes de começar, em milissegundos (só na ida).
    pub delay: f32,
    pub easing: Easing,
    pub trigger: Trigger,
    pub auto_reverse: bool,
    pub looping: bool,
    pub enabled: bool,
    /// Sequência de passos (`0:0; 0.6:1.15; 1:1`: fração do tempo e valor), no lugar de ir de
    /// `start_value` a `stop_value` (só números).
    pub keyframes: String,
    /// Valor da propriedade antes da primeira execução (para onde volta).
    original: Option<PropValue>,
    /// O gatilho `appear` já rodou.
    pub(crate) appeared: bool,
}

impl PropertyAnimation {
    fn new(color: bool, property: &str, stop_value: PropValue) -> Self {
        Self {
            color,
            property: property.to_string(),
            start_value: PropValue::Null,
            stop_value,
            duration: 200.0,
            delay: 0.0,
            easing: Easing::OutQuad,
            trigger: Trigger::None,
            auto_reverse: false,
            looping: false,
            enabled: true,
            original: None,
            keyframes: String::new(),
            appeared: false,
        }
    }

    /// `FloatAnimation`: anima uma propriedade numérica.
    pub fn float(property: &str, stop_value: f64) -> Self {
        Self::new(false, property, PropValue::Float(stop_value))
    }

    /// `ColorAnimation`: anima uma propriedade de cor.
    pub fn color(property: &str, stop_value: Color) -> Self {
        Self::new(true, property, PropValue::Color(stop_value))
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;
        self
    }

    pub fn with_duration(mut self, ms: f32) -> Self {
        self.duration = ms;
        self
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_delay(mut self, ms: f32) -> Self {
        self.delay = ms;
        self
    }
}

/// Para o registro de classes: `FloatAnimation`.
#[derive(Clone, Debug, PartialEq)]
pub struct FloatAnimation(pub PropertyAnimation);

impl Default for FloatAnimation {
    fn default() -> Self {
        Self(PropertyAnimation::float("opacity", 1.0))
    }
}

/// Para o registro de classes: `ColorAnimation`.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorAnimation(pub PropertyAnimation);

impl Default for ColorAnimation {
    fn default() -> Self {
        Self(PropertyAnimation::color("fill", Color::TRANSPARENT))
    }
}

macro_rules! delegate_animation {
    ($t:ty, $name:literal) => {
        impl Component for $t {
            fn class_name(&self) -> &'static str {
                $name
            }

            fn inner(&self) -> Option<&dyn Component> {
                Some(&self.0)
            }

            fn inner_mut(&mut self) -> Option<&mut dyn Component> {
                Some(&mut self.0)
            }
        }
    };
}

delegate_animation!(FloatAnimation, "FloatAnimation");
delegate_animation!(ColorAnimation, "ColorAnimation");

const FLOAT_PROPS: &[PropInfo] = &[
    PropInfo::new("property", PropKind::String, "Animation"),
    PropInfo::new("start_value", PropKind::Float, "Animation"),
    PropInfo::new("stop_value", PropKind::Float, "Animation"),
    PropInfo::new("duration", PropKind::Duration, "Animation"),
    PropInfo::new("delay", PropKind::Duration, "Animation"),
    PropInfo::new("easing", PropKind::Easing, "Animation"),
    PropInfo::new("trigger", PropKind::Enum(Trigger::NAMES), "Animation"),
    PropInfo::new("auto_reverse", PropKind::Bool, "Animation"),
    PropInfo::new("looping", PropKind::Bool, "Animation"),
    PropInfo::new("enabled", PropKind::Bool, "Animation"),
    PropInfo::new("keyframes", PropKind::String, "Animation"),
];

const COLOR_PROPS: &[PropInfo] = &[
    PropInfo::new("property", PropKind::String, "Animation"),
    PropInfo::new("start_value", PropKind::Color, "Animation"),
    PropInfo::new("stop_value", PropKind::Color, "Animation"),
    PropInfo::new("duration", PropKind::Duration, "Animation"),
    PropInfo::new("delay", PropKind::Duration, "Animation"),
    PropInfo::new("easing", PropKind::Easing, "Animation"),
    PropInfo::new("trigger", PropKind::Enum(Trigger::NAMES), "Animation"),
    PropInfo::new("auto_reverse", PropKind::Bool, "Animation"),
    PropInfo::new("looping", PropKind::Bool, "Animation"),
    PropInfo::new("enabled", PropKind::Bool, "Animation"),
];

impl Component for PropertyAnimation {
    fn class_name(&self) -> &'static str {
        if self.color { "ColorAnimation" } else { "FloatAnimation" }
    }

    fn props(&self) -> &'static [PropInfo] {
        if self.color { COLOR_PROPS } else { FLOAT_PROPS }
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        Some(match name {
            "property" => self.property.clone().into(),
            "start_value" => self.start_value.clone(),
            "stop_value" => self.stop_value.clone(),
            "duration" => self.duration.into(),
            "delay" => self.delay.into(),
            "easing" => self.easing.to_value(),
            "trigger" => PropValue::enum_value(self.trigger),
            "auto_reverse" => self.auto_reverse.into(),
            "looping" => self.looping.into(),
            "enabled" => self.enabled.into(),
            "keyframes" => self.keyframes.clone().into(),
            _ => return None,
        })
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        let value_of = |v: PropValue, color: bool| -> Result<PropValue> {
            Ok(match v {
                PropValue::Null => PropValue::Null,
                v if color => PropValue::Color(v.to_color()?),
                v => PropValue::Float(v.to_f64()?),
            })
        };
        match name {
            "property" => {
                self.property = value.to_str()?.to_string();
                self.original = None;
            }
            "start_value" => self.start_value = value_of(value, self.color)?,
            "stop_value" => self.stop_value = value_of(value, self.color)?,
            "duration" => self.duration = value.to_f32()?.max(0.0),
            "delay" => self.delay = value.to_f32()?.max(0.0),
            "easing" => self.easing = Easing::from_value(value)?,
            "trigger" => self.trigger = value.to_enum()?,
            "auto_reverse" => self.auto_reverse = value.to_bool()?,
            "looping" => self.looping = value.to_bool()?,
            "enabled" => self.enabled = value.to_bool()?,
            "keyframes" => {
                let text = value.to_str()?.to_string();
                if !text.trim().is_empty() && parse_keyframes(&text).is_none() {
                    return Err(Error::invalid("use fração:valor separados por ; (0:0; 0.6:1.15; 1:1)"));
                }
                self.keyframes = text;
            }
            _ => return Err(Error::UnknownProperty(name.to_string())),
        }
        Ok(())
    }
}
