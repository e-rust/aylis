//! Vistas do form (os "breakpoints"): ajustes por cima do form base, escolhidos pelo tamanho
//! real do form, pela orientação e pela plataforma. Um form com `views` = `phone` até 599 e
//! `tablet` até 1023 fica com o desenho base acima de 1023; abaixo, as mudanças da vista
//! (propriedades, contêiner, escala de fonte e de espaços) valem por cima dele, e voltam ao
//! base quando o form sai das condições.
//!
//! Vale uma vista por vez: a primeira da lista (em ordem de `max_width`, a mais estreita
//! antes; sem limite de largura, por último) cujas condições todas valem. Assim uma vista de
//! celular ganha da de tablet num form estreito, como as faixas do CSS.
//!
//! A vista pode mudar um componente de contêiner (as pseudo-propriedades [`PARENT`] e
//! [`INDEX`]): a barra lateral que no celular vai para o rodapé. O form guarda sempre o base:
//! [`Ui::view_base`] diz o valor base de uma propriedade mudada pela vista ativa e
//! [`Ui::base_children`] os filhos no base (o `aylis-json` grava esses). O designer força uma
//! vista ([`Ui::force_view`]) para editar nela.

use crate::component::ComponentId;
use crate::geometry::SizeF;
use crate::prop::{PropEnum, PropValue};
use crate::ui::Ui;

/// Pseudo-propriedade da vista: o nome do contêiner para onde o componente vai (vazio: o
/// próprio form).
pub const PARENT: &str = "parent";
/// Pseudo-propriedade da vista: a posição do componente entre os irmãos (a ordem de encaixe).
pub const INDEX: &str = "index";

/// As plataformas que uma vista pode pedir ([`FormView::platforms`]).
pub const PLATFORMS: [&str; 6] = ["windows", "linux", "macos", "android", "ios", "web"];

/// A plataforma em que o app roda, como em [`PLATFORMS`].
pub fn current_platform() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "web"
    } else if cfg!(target_os = "android") {
        "android"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

/// A orientação que uma vista pede.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ViewOrientation {
    #[default]
    Any,
    /// Mais alto que largo.
    Portrait,
    /// Mais largo que alto (ou quadrado).
    Landscape,
}

impl PropEnum for ViewOrientation {
    const NAMES: &'static [&'static str] = &["any", "portrait", "landscape"];

    fn from_index(index: usize) -> Option<Self> {
        [ViewOrientation::Any, ViewOrientation::Portrait, ViewOrientation::Landscape].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Uma vista: quando todas as condições valem, as `changes` e as escalas valem por cima do
/// base. Um limite 0 é "sem limite".
#[derive(Clone, Debug, PartialEq)]
pub struct FormView {
    /// `phone`, `tablet` ou outro nome (a IDE mostra Celular, Tablet...).
    pub name: String,
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub orientation: ViewOrientation,
    /// As plataformas onde vale (vazio: todas), como em [`PLATFORMS`].
    pub platforms: Vec<String>,
    /// Multiplica o tamanho de todas as fontes do form (1: como no desenho).
    pub font_scale: f32,
    /// Multiplica os espaços do layout do form: `padding`, `margins` e o `gap` das grades e
    /// fluxos (1: como no desenho).
    pub spacing_scale: f32,
    pub changes: Vec<ViewChange>,
}

impl FormView {
    /// Uma vista sem mudanças, até `max_width` de largura.
    pub fn new(name: impl Into<String>, max_width: f32) -> Self {
        FormView {
            name: name.into(),
            min_width: 0.0,
            max_width,
            min_height: 0.0,
            max_height: 0.0,
            orientation: ViewOrientation::Any,
            platforms: Vec::new(),
            font_scale: 1.0,
            spacing_scale: 1.0,
            changes: Vec::new(),
        }
    }

    /// Se a vista vale num form deste tamanho, nesta plataforma.
    pub fn matches(&self, size: SizeF, platform: &str) -> bool {
        let within = |v: f32, min: f32, max: f32| (min <= 0.0 || v >= min) && (max <= 0.0 || v <= max);
        within(size.width, self.min_width, self.max_width)
            && within(size.height, self.min_height, self.max_height)
            && match self.orientation {
                ViewOrientation::Any => true,
                ViewOrientation::Portrait => size.height > size.width,
                ViewOrientation::Landscape => size.width >= size.height,
            }
            && (self.platforms.is_empty() || self.platforms.iter().any(|p| p == platform))
    }

    /// A ordem de escolha: a mais estreita primeiro; sem limite de largura, por último.
    fn width_key(&self) -> f32 {
        if self.max_width > 0.0 { self.max_width } else { f32::INFINITY }
    }

    /// As escalas, limpas (fora de 0,25 a 4, ou inválidas, viram 1).
    fn scales(&self) -> (f32, f32) {
        let clean = |s: f32| if s.is_finite() && (0.25..=4.0).contains(&s) { s } else { 1.0 };
        (clean(self.font_scale), clean(self.spacing_scale))
    }
}

/// Uma propriedade de um componente do form, na vista ([`PARENT`] e [`INDEX`] mudam o
/// contêiner e a ordem).
#[derive(Clone, Debug, PartialEq)]
pub struct ViewChange {
    /// O nome do componente dentro do form (como no `find_in`); vazio: o próprio form.
    pub component: String,
    pub property: String,
    pub value: PropValue,
}

/// O estado das vistas de um form.
#[derive(Default)]
pub(crate) struct ViewState {
    views: Vec<FormView>,
    /// A vista aplicada agora (`None`: o base).
    active: Option<String>,
    /// O designer escolheu a vista (`Some(None)`: o base, qualquer que seja o tamanho).
    forced: Option<Option<String>>,
    /// Os valores base das propriedades que a vista ativa mudou (para voltar).
    base: Vec<(ComponentId, String, PropValue)>,
    /// Os componentes que a vista ativa mudou de lugar: o pai e a posição de antes, na ordem
    /// em que mudaram (volta-se do fim para o começo).
    moves: Vec<(ComponentId, ComponentId, usize)>,
    /// As escalas de fonte e de espaços da vista ativa.
    scales: (f32, f32),
}

impl Ui {
    /// As vistas do form (na ordem de escolha: da mais estreita para a mais larga).
    /// Reaplica pelo tamanho.
    pub fn set_views(&mut self, form: ComponentId, mut views: Vec<FormView>) {
        views.sort_by(|a, b| a.width_key().total_cmp(&b.width_key()));
        self.restore_view_base(form);
        let state = self.views.entry(form).or_default();
        state.views = views;
        state.active = None;
        self.layout_dirty = true;
        self.request_repaint();
    }

    pub fn views(&self, form: ComponentId) -> &[FormView] {
        self.views.get(&form).map_or(&[], |s| &s.views)
    }

    /// A vista aplicada agora no form (`None`: o base).
    pub fn active_view(&self, form: ComponentId) -> Option<&str> {
        self.views.get(&form)?.active.as_deref()
    }

    /// As escalas de fonte e de espaços da vista ativa do form (`(1, 1)` sem vista).
    pub fn view_scales(&self, form: ComponentId) -> (f32, f32) {
        self.own_view_scales(form).unwrap_or((1.0, 1.0))
    }

    /// As escalas da vista ativa do form, se ele tem uma (`None`: herda as do pai).
    pub(crate) fn own_view_scales(&self, form: ComponentId) -> Option<(f32, f32)> {
        self.views.get(&form).filter(|s| s.active.is_some()).map(|s| s.scales)
    }

    /// O designer escolhe a vista a mostrar e editar: `Some("phone")`, `Some` do base (vazio)
    /// ou `None` para voltar a escolher pelo tamanho.
    pub fn force_view(&mut self, form: ComponentId, view: Option<&str>) {
        let state = self.views.entry(form).or_default();
        state.forced = view.map(|v| (!v.is_empty()).then(|| v.to_string()));
        // A vista forçada pode ser a mesma com outras mudanças (o designer mexeu nela):
        // volta ao base e o layout reaplica.
        self.restore_view_base(form);
        self.layout_dirty = true;
        self.request_repaint();
    }

    /// O valor base de uma propriedade que a vista ativa mudou (`None`: não mudou; vale o
    /// valor de agora). O `aylis-json` grava este no form.
    pub fn view_base(&self, id: ComponentId, property: &str) -> Option<&PropValue> {
        let form = self.form_of(id);
        self.views.get(&form)?.base.iter().find(|(c, p, _)| *c == id && p == property).map(|(_, _, v)| v)
    }

    /// Os filhos de `id` no form base (sem o que a vista ativa trouxe ou levou). O `aylis-json`
    /// grava estes.
    pub fn base_children(&self, id: ComponentId) -> Vec<ComponentId> {
        let mut list = self.children(id).to_vec();
        let form = self.form_of(id);
        let Some(state) = self.views.get(&form).filter(|s| !s.moves.is_empty()) else { return list };
        // Desfaz as mudanças de lugar do fim para o começo, só olhando para este contêiner.
        for &(moved, old_parent, old_index) in state.moves.iter().rev() {
            list.retain(|&c| c != moved);
            if old_parent == id {
                list.insert(old_index.min(list.len()), moved);
            }
        }
        list
    }

    /// O pai e a posição de um componente no form base (`None`: a vista não o mudou de lugar).
    pub fn view_base_place(&self, id: ComponentId) -> Option<(ComponentId, usize)> {
        let form = self.form_of(id);
        self.views.get(&form)?.moves.iter().find(|m| m.0 == id).map(|&(_, p, i)| (p, i))
    }

    /// O designer editou na vista ativa: as mudanças dela passam a ser `changes` (os valores
    /// de agora já são esses) e `base` diz o valor base de cada propriedade mexida (inclusive
    /// as que só a prévia mudou, como a largura do form, que não entram nas mudanças);
    /// `moves`, o lugar no base de quem mudou de contêiner ou de posição.
    pub fn update_active_view(
        &mut self,
        form: ComponentId,
        changes: Vec<ViewChange>,
        base: Vec<(ComponentId, String, PropValue)>,
        moves: Vec<(ComponentId, ComponentId, usize)>,
    ) {
        let Some(state) = self.views.get_mut(&form) else { return };
        let Some(active) = state.active.clone() else { return };
        if let Some(view) = state.views.iter_mut().find(|v| v.name == active) {
            view.changes = changes;
        }
        state.base = base;
        state.moves = moves;
    }

    /// Muda as escalas da vista `name` (o designer, com a vista à mostra).
    pub fn set_view_scales(&mut self, form: ComponentId, name: &str, font_scale: f32, spacing_scale: f32) {
        let Some(state) = self.views.get_mut(&form) else { return };
        let Some(view) = state.views.iter_mut().find(|v| v.name == name) else { return };
        view.font_scale = font_scale;
        view.spacing_scale = spacing_scale;
        if state.active.as_deref() == Some(name) {
            state.scales = view.scales();
        }
        self.layout_dirty = true;
        self.request_repaint();
    }

    /// Volta o que a vista ativa mudou ao base.
    fn restore_view_base(&mut self, form: ComponentId) {
        let Some(state) = self.views.get_mut(&form) else { return };
        let base = std::mem::take(&mut state.base);
        let moves = std::mem::take(&mut state.moves);
        let had_scales = state.scales != (1.0, 1.0);
        state.active = None;
        state.scales = (1.0, 1.0);
        for (id, prop, value) in base.into_iter().rev() {
            let _ = self.set_prop_quiet(id, &prop, value);
        }
        for (id, parent, index) in moves.into_iter().rev() {
            if self.parent(id) != Some(parent) {
                let _ = self.set_parent(id, Some(parent));
            }
            let _ = self.set_child_index(id, index);
        }
        if had_scales {
            self.invalidate_form(form);
        }
    }

    /// O componente de uma mudança da vista: pelo nome, dentro do form; vazio, o próprio form.
    fn view_target(&self, form: ComponentId, component: &str) -> Option<ComponentId> {
        if component.is_empty() { Some(form) } else { self.find_in(form, component) }
    }

    /// Escolhe e aplica a vista de cada form pelo tamanho (chamado pelo layout).
    pub(crate) fn apply_views(&mut self) {
        let forms: Vec<ComponentId> = self.views.keys().copied().collect();
        let platform = current_platform();
        for form in forms {
            let Some(size) = self.cb(form).map(|cb| SizeF::new(cb.width(), cb.height())) else {
                self.views.remove(&form);
                continue;
            };
            let state = &self.views[&form];
            let wanted = match &state.forced {
                Some(view) => view.clone(),
                None => state.views.iter().find(|v| v.matches(size, platform)).map(|v| v.name.clone()),
            };
            if wanted == state.active {
                continue;
            }
            self.restore_view_base(form);
            let Some(view) = wanted.as_ref().and_then(|w| self.views[&form].views.iter().find(|v| &v.name == w).cloned()) else { continue };
            // Primeiro o lugar (o contêiner e depois a posição), depois as propriedades.
            let mut moves = Vec::new();
            for change in view.changes.iter().filter(|c| c.property == PARENT).chain(view.changes.iter().filter(|c| c.property == INDEX)) {
                let Some(id) = self.view_target(form, &change.component) else { continue };
                let Some(old_parent) = self.parent(id) else { continue };
                let old_index = self.children(old_parent).iter().position(|&c| c == id).unwrap_or(0);
                let done = if change.property == PARENT {
                    let target = match change.value.to_str().unwrap_or("") {
                        "" => Some(form),
                        name => self.find_in(form, name).or_else(|| (self.name(form) == Some(name)).then_some(form)),
                    };
                    target.is_some_and(|t| t == old_parent || self.set_parent(id, Some(t)).is_ok())
                } else {
                    let index = change.value.to_f64().map_or(0, |i| i.max(0.0) as usize);
                    self.set_child_index(id, index).is_ok()
                };
                if done {
                    moves.push((id, old_parent, old_index));
                }
            }
            let mut base = Vec::new();
            for change in view.changes.iter().filter(|c| c.property != PARENT && c.property != INDEX) {
                let Some(id) = self.view_target(form, &change.component) else { continue };
                let Ok(old) = self.get_prop(id, &change.property) else { continue };
                if self.set_prop_quiet(id, &change.property, change.value.clone()).is_ok() {
                    base.push((id, change.property.clone(), old));
                }
            }
            let scales = view.scales();
            if scales != (1.0, 1.0) {
                self.invalidate_form(form);
            }
            if let Some(state) = self.views.get_mut(&form) {
                state.base = base;
                state.moves = moves;
                state.active = wanted;
                state.scales = scales;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn size(width: f32, height: f32) -> SizeF {
        SizeF::new(width, height)
    }

    #[test]
    fn the_limits_include_the_edges_and_zero_is_no_limit() {
        let mut v = FormView::new("faixa", 599.0);
        v.min_width = 400.0;
        assert!(v.matches(size(599.0, 800.0), "windows"), "o máximo vale");
        assert!(v.matches(size(400.0, 800.0), "windows"), "o mínimo vale");
        assert!(!v.matches(size(599.5, 800.0), "windows"));
        assert!(!v.matches(size(399.0, 800.0), "windows"));
        let open = FormView::new("sem limite", 0.0);
        assert!(open.matches(size(10.0, 10.0), "web") && open.matches(size(99_999.0, 99_999.0), "web"));
        let mut tall = FormView::new("alta", 0.0);
        (tall.min_height, tall.max_height) = (500.0, 900.0);
        assert!(tall.matches(size(300.0, 500.0), "linux") && tall.matches(size(300.0, 900.0), "linux"));
        assert!(!tall.matches(size(300.0, 901.0), "linux") && !tall.matches(size(300.0, 499.0), "linux"));
    }

    #[test]
    fn a_square_form_is_landscape() {
        let mut portrait = FormView::new("retrato", 0.0);
        portrait.orientation = ViewOrientation::Portrait;
        let mut landscape = FormView::new("paisagem", 0.0);
        landscape.orientation = ViewOrientation::Landscape;
        assert!(landscape.matches(size(500.0, 500.0), "ios") && !portrait.matches(size(500.0, 500.0), "ios"));
        assert!(portrait.matches(size(500.0, 501.0), "ios") && !landscape.matches(size(500.0, 501.0), "ios"));
    }

    #[test]
    fn the_platforms_restrict_and_empty_means_all() {
        let mut mobile = FormView::new("móvel", 0.0);
        mobile.platforms = vec!["android".into(), "ios".into()];
        assert!(mobile.matches(size(400.0, 800.0), "android"));
        assert!(!mobile.matches(size(400.0, 800.0), "windows"));
        assert!(FormView::new("todas", 0.0).matches(size(400.0, 800.0), "windows"));
        assert!(PLATFORMS.contains(&current_platform()));
    }

    #[test]
    fn invalid_scales_become_one() {
        let mut v = FormView::new("x", 0.0);
        for (font, spacing, expected) in [(0.5, 2.0, (0.5, 2.0)), (0.0, 5.0, (1.0, 1.0)), (f32::NAN, 0.25, (1.0, 0.25)), (4.0, f32::INFINITY, (4.0, 1.0))] {
            (v.font_scale, v.spacing_scale) = (font, spacing);
            assert_eq!(v.scales(), expected, "{font} {spacing}");
        }
    }

    #[test]
    fn the_narrowest_view_comes_first_and_no_limit_last() {
        let mut ui = Ui::new();
        let form = ComponentId { index: 0, generation: 0 };
        ui.set_views(form, vec![FormView::new("larga", 0.0), FormView::new("tablet", 1023.0), FormView::new("phone", 599.0)]);
        let names: Vec<&str> = ui.views(form).iter().map(|v| v.name.as_str()).collect();
        assert_eq!(names, ["phone", "tablet", "larga"]);
    }
}
