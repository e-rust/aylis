//! O efeito de fim da rolagem (decidido em 14/11): quando o dedo arrasta além do começo ou do
//! fim de um contêiner com rolagem, ou a inércia bate na ponta, o contêiner mostra que acabou.
//!
//! - `glow`: o brilho em meia-lua na borda, na cor do tema (o Android até o 11);
//! - `stretch`: o conteúdo estica a partir da borda e volta (o Android 12 em diante);
//! - `bounce`: o conteúdo passa da ponta e volta com mola (o iOS);
//! - `none`: nada (o desktop).
//!
//! Quem rola declara a propriedade `overscroll` (o `ScrollBox`, as listas, a grade, o `Memo`):
//! é assim que o núcleo acha o contêiner do dedo. `auto` (o padrão) segue o sistema, que a
//! plataforma diz em [`Ui::set_overscroll_style`]. O efeito é só desenho e deslocamento feitos
//! pelo núcleo, igual nos três renderizadores, e só existe durante o puxão: parado, nada roda.

use std::collections::HashMap;

use crate::component::ComponentId;
use crate::geometry::PointF;
use crate::ui::Ui;

/// O efeito de fim da rolagem de um contêiner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Overscroll {
    /// O do sistema ([`Ui::overscroll_style`]).
    #[default]
    Auto,
    Glow,
    Stretch,
    Bounce,
    None,
}

impl crate::prop::PropEnum for Overscroll {
    const NAMES: &'static [&'static str] = &["auto", "glow", "stretch", "bounce", "none"];
    fn from_index(i: usize) -> Option<Self> {
        [Overscroll::Auto, Overscroll::Glow, Overscroll::Stretch, Overscroll::Bounce, Overscroll::None].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Um puxão em andamento num contêiner: quanto o conteúdo passou da ponta (positivo: passou
/// do começo, puxado para baixo ou para a direita), se o dedo ainda segura e quando andou.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Pull {
    pub(crate) amount: PointF,
    pub(crate) held: bool,
    pub(crate) time: f64,
}

/// Quanto o puxão vai, no máximo, em fração do tamanho do contêiner.
const MAX_FRACTION: f32 = 0.2;
/// Quanto do movimento do dedo vira puxão (a resistência cresce perto do máximo).
const RESISTANCE: f32 = 0.55;
/// A volta ao soltar: a fração que sobra cai pela metade a cada ~70 ms.
const RELEASE_RATE: f32 = 10.0;
/// O impulso de quando a inércia bate na ponta, por ponto/segundo de velocidade.
const ABSORB: f32 = 0.04;

#[derive(Default)]
pub(crate) struct Pulls {
    pub(crate) by_host: HashMap<ComponentId, Pull>,
}

impl Ui {
    /// O efeito do sistema para os contêineres em `auto` (a plataforma diz: o Android pela
    /// versão, o iOS a mola, o desktop nenhum).
    pub fn set_overscroll_style(&mut self, style: Overscroll) {
        self.overscroll_style = if style == Overscroll::Auto { Overscroll::None } else { style };
    }

    /// O efeito do sistema (o `auto`).
    pub fn overscroll_style(&self) -> Overscroll {
        self.overscroll_style
    }

    /// O efeito de `id`: `None` se ele não rola (não declara `overscroll`) ou não quer efeito.
    pub(crate) fn overscroll_of(&self, id: ComponentId) -> Option<Overscroll> {
        let style = match self.get_prop(id, "overscroll") {
            Ok(crate::PropValue::Enum(name)) => <Overscroll as crate::prop::PropEnum>::from_name(&name).unwrap_or_default(),
            _ => return None,
        };
        let style = if style == Overscroll::Auto { self.overscroll_style } else { style };
        (style != Overscroll::None).then_some(style)
    }

    /// O contêiner com efeito mais perto de `from` (ele ou um ancestral).
    pub(crate) fn overscroll_host(&self, from: Option<ComponentId>) -> Option<ComponentId> {
        let mut current = from;
        while let Some(id) = current {
            if self.overscroll_of(id).is_some() {
                return Some(id);
            }
            current = self.parent(id);
        }
        None
    }

    /// O puxão de `id` agora (o desenho lê).
    pub(crate) fn overscroll_pull(&self, id: ComponentId) -> Option<PointF> {
        self.pulls.borrow().by_host.get(&id).map(|p| p.amount).filter(|a| a.x != 0.0 || a.y != 0.0)
    }

    /// O máximo do puxão de `host` em cada eixo.
    fn pull_limit(&self, host: ComponentId) -> PointF {
        let size = self.cb(host).map(|cb| cb.bounds().size()).unwrap_or_default();
        PointF::new((size.width * MAX_FRACTION).max(24.0), (size.height * MAX_FRACTION).max(24.0))
    }

    /// O dedo arrastou `delta` em `host` com o conteúdo na ponta: o puxão cresce, com mais
    /// resistência perto do máximo (só no eixo em que o dedo mais andou).
    pub(crate) fn overscroll_drag(&mut self, host: ComponentId, delta: PointF) {
        let limit = self.pull_limit(host);
        let now = self.time;
        let mut pulls = self.pulls.borrow_mut();
        let p = pulls.by_host.entry(host).or_insert(Pull { amount: PointF::ZERO, held: true, time: now });
        let grow = |amount: f32, d: f32, max: f32| {
            let room = (1.0 - amount.abs() / max).max(0.0);
            (amount + d * RESISTANCE * room).clamp(-max, max)
        };
        if delta.y.abs() >= delta.x.abs() {
            p.amount.y = grow(p.amount.y, delta.y, limit.y);
        } else {
            p.amount.x = grow(p.amount.x, delta.x, limit.x);
        }
        p.held = true;
        p.time = now;
        drop(pulls);
        self.damage(host);
    }

    /// O dedo voltou na direção contrária com o puxão aberto: o puxão fecha primeiro, e o que
    /// sobra do movimento vai para a rolagem (devolvido).
    pub(crate) fn overscroll_take_back(&mut self, host: ComponentId, delta: PointF) -> PointF {
        let mut pulls = self.pulls.borrow_mut();
        let Some(p) = pulls.by_host.get_mut(&host) else { return delta };
        let back = |amount: &mut f32, d: f32| -> f32 {
            if *amount == 0.0 || amount.signum() == d.signum() {
                return d;
            }
            let next = *amount + d;
            if next.signum() == amount.signum() {
                *amount = next;
                0.0
            } else {
                *amount = 0.0;
                next
            }
        };
        let rest = PointF::new(back(&mut p.amount.x, delta.x), back(&mut p.amount.y, delta.y));
        let gone = p.amount.x == 0.0 && p.amount.y == 0.0;
        if gone {
            pulls.by_host.remove(&host);
        }
        drop(pulls);
        if rest != delta {
            self.damage(host);
        }
        rest
    }

    /// O dedo saiu: os puxões voltam sozinhos.
    pub(crate) fn overscroll_release(&mut self) {
        let now = self.time;
        for p in self.pulls.borrow_mut().by_host.values_mut() {
            p.held = false;
            p.time = now;
        }
    }

    /// A inércia bateu na ponta de `host` com `velocity` (pontos/s): um impulso que volta.
    pub(crate) fn overscroll_absorb(&mut self, host: ComponentId, velocity: PointF) {
        let limit = self.pull_limit(host);
        let now = self.time;
        let amount = PointF::new((velocity.x * ABSORB).clamp(-limit.x * 0.5, limit.x * 0.5), (velocity.y * ABSORB).clamp(-limit.y * 0.5, limit.y * 0.5));
        self.pulls.borrow_mut().by_host.insert(host, Pull { amount, held: false, time: now });
        self.damage(host);
    }

    /// Há puxão em andamento (pede quadros).
    pub(crate) fn overscroll_active(&self) -> bool {
        !self.pulls.borrow().by_host.is_empty()
    }

    /// Os puxões soltos voltam (chamado por [`Ui::tick`]).
    pub(crate) fn tick_overscroll(&mut self) {
        if self.pulls.borrow().by_host.is_empty() {
            return;
        }
        let now = self.time;
        let mut moved = Vec::new();
        {
            let mut pulls = self.pulls.borrow_mut();
            pulls.by_host.retain(|&host, p| {
                if p.held {
                    return true;
                }
                let dt = (now - p.time).clamp(0.0, 0.05) as f32;
                p.time = now;
                let keep = (-RELEASE_RATE * dt).exp();
                p.amount = PointF::new(p.amount.x * keep, p.amount.y * keep);
                moved.push(host);
                p.amount.x.abs() >= 0.5 || p.amount.y.abs() >= 0.5
            });
        }
        for host in moved {
            if self.contains(host) {
                self.damage(host);
            }
        }
    }
}
