//! Transições entre páginas e forms: esmaecer e deslizar. A página nova entra enquanto a
//! antiga sai, as duas desenhadas ao mesmo tempo (deslocadas com `translate_x`/`translate_y`
//! e `opacity`, que não mexem no layout); no fim, a antiga é escondida (ou o form é
//! fechado) e os valores voltam ao normal.
//!
//! Quem usa: o `TabControl` (propriedade `transition`), os forms empilhados do celular e
//! do navegador (propriedade `transition` do form, ao abrir e fechar) e o app, por
//! [`Ui::transition`].

use crate::component::ComponentId;
use crate::geometry::PointF;
use crate::prop::PropEnum;
use crate::ui::Ui;

/// O tipo de transição. `slide_left`: o conteúdo anda para a esquerda (a página nova entra
/// pela direita, como ao avançar); `slide_right`, o contrário (ao voltar).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Transition {
    #[default]
    None,
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
}

impl Transition {
    pub const ALL: [Transition; 6] =
        [Transition::None, Transition::Fade, Transition::SlideLeft, Transition::SlideRight, Transition::SlideUp, Transition::SlideDown];

    /// A transição de volta (para fechar o que abriu com esta).
    pub fn reversed(self) -> Self {
        match self {
            Transition::SlideLeft => Transition::SlideRight,
            Transition::SlideRight => Transition::SlideLeft,
            Transition::SlideUp => Transition::SlideDown,
            Transition::SlideDown => Transition::SlideUp,
            other => other,
        }
    }
}

impl PropEnum for Transition {
    const NAMES: &'static [&'static str] = &["none", "fade", "slide_left", "slide_right", "slide_up", "slide_down"];

    fn from_index(index: usize) -> Option<Self> {
        Transition::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Duração padrão, em milissegundos.
pub const TRANSITION_MS: f32 = 250.0;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Running {
    from: Option<ComponentId>,
    to: Option<ComponentId>,
    kind: Transition,
    start: f64,
    duration: f64,
    /// Opacidade de antes (volta no fim).
    from_opacity: f32,
    to_opacity: f32,
    /// No fim, fecha o form `from` (em vez de só esconder).
    close_from: bool,
}

impl Ui {
    /// Troca `from` por `to` com uma transição: `to` fica visível e entra, `from` sai e no
    /// fim fica invisível. Qualquer um dos dois pode faltar (só entrar, só sair).
    pub fn transition(&mut self, from: Option<ComponentId>, to: Option<ComponentId>, kind: Transition, duration_ms: f32) {
        self.start_transition(from, to, kind, duration_ms, false);
    }

    pub(crate) fn start_transition(&mut self, from: Option<ComponentId>, to: Option<ComponentId>, kind: Transition, duration_ms: f32, close_from: bool) {
        // Uma transição em andamento nos mesmos controles termina na hora.
        let pending: Vec<Running> = self
            .transitions
            .iter()
            .filter(|r| [r.from, r.to].iter().flatten().any(|c| Some(*c) == from || Some(*c) == to))
            .copied()
            .collect();
        for r in pending {
            self.finish_transition(r);
        }
        self.transitions.retain(|r| ![r.from, r.to].iter().flatten().any(|c| Some(*c) == from || Some(*c) == to));
        if let Some(t) = to {
            if let Some(cb) = self.control_mut(t) {
                cb.set_visible(true);
            }
        }
        let opacity = |ui: &Ui, id: Option<ComponentId>| id.and_then(|c| ui.control(c)).map_or(1.0, |cb| cb.opacity());
        let running = Running {
            from,
            to,
            kind,
            start: self.time,
            duration: (duration_ms.max(0.0) / 1000.0) as f64,
            from_opacity: opacity(self, from),
            to_opacity: opacity(self, to),
            close_from,
        };
        if kind == Transition::None || running.duration <= 0.0 {
            self.finish_transition(running);
            return;
        }
        self.apply_transition(&running, 0.0);
        self.transitions.push(running);
        self.invalidate();
        self.request_repaint();
    }

    /// O controle está entrando (uma transição de entrada em andamento).
    pub(crate) fn transitions_entering(&self, id: ComponentId) -> bool {
        self.transitions.iter().any(|r| r.to == Some(id))
    }

    /// Há transição em andamento com este controle.
    pub fn in_transition(&self, id: ComponentId) -> bool {
        self.transitions.iter().any(|r| r.from == Some(id) || r.to == Some(id))
    }

    pub(crate) fn tick_transitions(&mut self) {
        if self.transitions.is_empty() {
            return;
        }
        let now = self.time;
        let running = std::mem::take(&mut self.transitions);
        let mut keep = Vec::new();
        for r in running {
            let t = ((now - r.start) / r.duration).clamp(0.0, 1.0) as f32;
            if t >= 1.0 {
                self.finish_transition(r);
            } else {
                self.apply_transition(&r, crate::anim::Easing::OutCubic.apply(t));
                keep.push(r);
            }
        }
        // Transições que começaram durante o fim de outra (um form fechando abre outro).
        keep.append(&mut self.transitions);
        self.transitions = keep;
        // Sem `request_repaint`: o `apply_transition` marca só o lugar ou a transparência de
        // quem anda (a janela inteira refazia as camadas guardadas a cada quadro, e as páginas
        // eram repintadas controle por controle; medido em `medicoes/transicoes.json`).
    }

    /// O deslocamento de quem entra no começo (o tamanho do controle na direção).
    fn slide_vector(&self, kind: Transition, id: ComponentId) -> PointF {
        let size = self.control(id).map(|cb| cb.bounds().size()).unwrap_or_default();
        match kind {
            Transition::SlideLeft => PointF::new(size.width, 0.0),
            Transition::SlideRight => PointF::new(-size.width, 0.0),
            Transition::SlideUp => PointF::new(0.0, size.height),
            Transition::SlideDown => PointF::new(0.0, -size.height),
            _ => PointF::ZERO,
        }
    }

    fn apply_transition(&mut self, r: &Running, p: f32) {
        match r.kind {
            Transition::Fade => {
                if let Some(cb) = r.to.and_then(|c| self.cb_mut(c)) {
                    cb.set_opacity(r.to_opacity * p);
                }
                if let Some(cb) = r.from.and_then(|c| self.cb_mut(c)) {
                    cb.set_opacity(r.from_opacity * (1.0 - p));
                }
            }
            Transition::None => return,
            kind => {
                if let Some(to) = r.to {
                    let v = self.slide_vector(kind, to);
                    if let Some(cb) = self.cb_mut(to) {
                        cb.set_translate(PointF::new(v.x * (1.0 - p), v.y * (1.0 - p)));
                    }
                }
                if let Some(from) = r.from {
                    let v = self.slide_vector(kind, from);
                    if let Some(cb) = self.cb_mut(from) {
                        cb.set_translate(PointF::new(-v.x * p, -v.y * p));
                    }
                }
            }
        }
        // Só o lugar ou a transparência mudou (por isso o `cb_mut`, e não o `control_mut`, que
        // marcaria o desenho de dentro e o layout do form): a área antiga e a nova são
        // repintadas, e a camada guardada de cada um (veja `paint_node`) continua valendo.
        for id in [r.from, r.to].into_iter().flatten() {
            self.damage_placement(id);
        }
    }

    fn finish_transition(&mut self, r: Running) {
        // As imagens das duas páginas não valem mais (a pintura do fim é inteira, e a página
        // que sai some): saem do renderizador na próxima pintura, em vez de ocupar memória
        // até passar do limite (`image_cache.layers_mb`).
        for id in [r.from, r.to].into_iter().flatten() {
            if let Some(key) = self.entry(id).and_then(|e| e.visual.borrow_mut().layer_key.take()) {
                self.dead_layers.get_mut().push(key);
            }
        }
        if let Some(cb) = r.to.and_then(|c| self.control_mut(c)) {
            cb.set_translate(PointF::ZERO);
            cb.set_opacity(r.to_opacity);
            cb.set_visible(true);
        }
        if let Some(from) = r.from {
            if let Some(cb) = self.control_mut(from) {
                cb.set_translate(PointF::ZERO);
                cb.set_opacity(r.from_opacity);
            }
            if r.close_from {
                self.closing_now = Some(from);
                self.close(from);
                self.closing_now = None;
            } else if let Some(cb) = self.control_mut(from) {
                cb.set_visible(false);
            }
        }
        self.invalidate();
        self.request_repaint();
    }
}
