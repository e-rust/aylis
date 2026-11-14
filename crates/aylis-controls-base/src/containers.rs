//! Contêineres: ScrollBox e Splitter.

use aylis_core::{
    Align, Brush, Canvas, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Ui, WheelEvent, properties,
};

use crate::leaf_control;

const BAR: f32 = 8.0;

/// Se a roda desliza no [`ScrollBox`]: como o app manda ([`Ui::scroll_options`](aylis_core::Ui::scroll_options)), sempre ou nunca.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SmoothScroll {
    #[default]
    Default,
    On,
    Off,
}

impl aylis_core::PropEnum for SmoothScroll {
    const NAMES: &'static [&'static str] = &["default", "on", "off"];
    fn from_index(i: usize) -> Option<Self> {
        [SmoothScroll::Default, SmoothScroll::On, SmoothScroll::Off].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Área com rolagem vertical. Os filhos usam o layout normal
/// (por exemplo `align: top`, empilhados); o que passar da altura rola com a roda do
/// mouse ou arrastando a barra. `on_scroll` avisa quando o usuário rola (roda, dedo, barra),
/// com a posição nova; o que o código posiciona não avisa. `scroll_position` lê e grava a
/// posição, `scroll_to(componente)` rola o mínimo para ele ficar inteiro à vista e
/// `visible_components(lista, inteiros)` diz quais filhos da lista estão à vista (os caminhos,
/// separados por `;`; em Rust, `ScrollBox::visible_children`).
///
/// Com `horizontal` rola para os lados: filhos
/// com `align: left` lado a lado; a roda comum também rola para os lados.
///
/// A roda anda `wheel_step` pontos por dente (0 = o do app) e desliza conforme `smooth`.
pub struct ScrollBox {
    pub base: ControlBase,
    /// Rola para os lados em vez de para baixo (filhos com `align: left`, lado a lado).
    pub horizontal: bool,
    /// Puxar para baixo no topo atualiza (o `v-pull-to-refresh`): dispara `on_refresh` e
    /// mostra o indicador enquanto `refreshing` estiver ligado (quem atualiza desliga).
    pub pull_to_refresh: bool,
    /// Mostra o indicador de atualização no topo; quem atualiza desliga ao terminar.
    pub refreshing: bool,
    /// Pontos por dente da roda (0 = o do app, [`Ui::scroll_options`](aylis_core::Ui::scroll_options)).
    pub wheel_step: f32,
    /// Deslizar na roda: `default` (o do app), `on` ou `off`.
    pub smooth: SmoothScroll,
    /// A calha da barra: `true` (o padrão) reserva o lugar dela sempre (o `scrollbar-gutter:
    /// stable`); `false`, a barra passa por cima do conteúdo, que usa a largura toda (L01 do
    /// Tabernáculo).
    pub gutter: bool,
    /// O deslize em andamento: de onde, para onde e quando começou.
    glide: Option<(f32, f32, f64)>,
    dragging: Option<f32>,
    /// Posição sem arredondar: passos menores que um ponto (dedo devagar, fim da
    /// inércia) se somam em vez de se perder.
    exact: f32,
    /// Quanto foi puxado no topo e quando (some se parar de puxar).
    pull: std::cell::Cell<(f32, f64)>,
    /// `on_end_reached` já avisou para este tamanho de conteúdo.
    end_fired_at: Option<f32>,
    /// Os caracteres dos campos dentro da caixa (veja o `Form`).
    pub allowed_chars: String,
    /// Os caracteres que os campos de dentro não aceitam (digitados ou colados); faixas como
    /// `a-z`. Vazio: nenhum.
    pub blocked_chars: String,
    /// Só os caracteres de `allowed_chars` entram nos campos de dentro.
    pub only_allowed: bool,
    /// O filtro de caracteres da caixa manda nos campos de dentro (sem ele, o do campo decide
    /// primeiro).
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

impl Default for ScrollBox {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 200.0);
        base.set_clip_children(true).set_padding([0.0, 0.0, BAR + 2.0, 0.0]);
        Self { overscroll: aylis_core::Overscroll::Auto, base, horizontal: false, pull_to_refresh: false, refreshing: false, wheel_step: 0.0, smooth: SmoothScroll::Default, gutter: true, glide: None, dragging: None, exact: 0.0, pull: std::cell::Cell::new((0.0, 0.0)), end_fired_at: None, allowed_chars: String::new(), blocked_chars: String::new(), only_allowed: false, chars_override: false, char_map: String::new() }
    }
}

impl ScrollBox {
    /// Tamanho do conteúdo e da área visível no eixo da rolagem.
    fn axis(&self) -> (f32, f32) {
        let (c, b) = (self.base.content_size(), self.base.bounds());
        if self.horizontal { (c.width, b.width) } else { (c.height, b.height) }
    }

    /// Quanto dá para rolar. Menos de meio ponto (o arredondamento das medidas) é nada: a
    /// barra não aparece sem ter o que rolar.
    pub fn max_scroll(&self) -> f32 {
        let (content, view) = self.axis();
        if content - view < 0.5 { 0.0 } else { content - view }
    }

    /// O padding do lado da barra: a calha, ou nada.
    fn apply_gutter(&mut self) {
        let room = if self.gutter { BAR + 2.0 } else { 0.0 };
        self.base.set_padding(if self.horizontal { [0.0, 0.0, 0.0, room] } else { [0.0, 0.0, room, 0.0] });
    }

    /// Posição atual da rolagem.
    pub fn scroll(&self) -> f32 {
        let o = self.base.scroll_offset();
        if self.horizontal { o.x } else { o.y }
    }

    /// Rola até `y` (arredondado). `user`: quem rolou foi o usuário (a roda, o dedo, a barra),
    /// e o `on_scroll` avisa; o que o próprio aylis ajusta (o tamanho mudou) não avisa.
    fn set_scroll(&mut self, ctx: &mut EventCtx<'_>, y: f32, user: bool) -> bool {
        self.set_scroll_exact(ctx, y.round(), user)
    }

    /// Como [`set_scroll`](Self::set_scroll), sem arredondar (o meio do deslize).
    fn set_scroll_exact(&mut self, ctx: &mut EventCtx<'_>, y: f32, user: bool) -> bool {
        let y = y.clamp(0.0, self.max_scroll());
        if y == self.scroll() {
            return false;
        }
        // Só os filhos andam (sem refazer o layout do form a cada passo).
        let offset = if self.horizontal { PointF::new(y, 0.0) } else { PointF::new(0.0, y) };
        ctx.ui.scroll_content(ctx.id, &mut self.base, offset);
        // Chegou perto do fim: avisa uma vez por tamanho de conteúdo (rolagem infinita).
        let max = self.max_scroll();
        if max > 0.0 && y >= max - END_MARGIN && self.end_fired_at != Some(max) {
            self.end_fired_at = Some(max);
            ctx.fire("on_end_reached", EventArgs::None);
        }
        if user {
            ctx.fire("on_scroll", EventArgs::Value(aylis_core::PropValue::Float(f64::from(y))));
        }
        true
    }

    fn thumb(&self) -> Option<RectF> {
        let b = self.base.bounds();
        let (content, view) = self.axis();
        if content - view < 0.5 || view <= 0.0 {
            return None;
        }
        let len = (view * view / content).max(24.0);
        let at = (view - len) * self.scroll() / self.max_scroll().max(1.0);
        Some(if self.horizontal { RectF::new(at, b.height - BAR - 1.0, len, BAR) } else { RectF::new(b.width - BAR - 1.0, at, BAR, len) })
    }

    fn sync(&mut self, name: &str) {
        if name == "horizontal" {
            // A barra fica embaixo (horizontal) ou à direita (vertical).
            self.apply_gutter();
            self.base.set_scroll_offset(PointF::ZERO);
            self.exact = 0.0;
        }
        if name == "gutter" {
            self.apply_gutter();
        }
    }

    /// O intervalo que `id` (qualquer componente dentro da caixa) ocupa no conteúdo, no eixo
    /// da rolagem: a posição dele relativa à caixa no último layout mais a rolagem que esse
    /// layout usou. Não lê a própria caixa no `Ui` (no `invoke` ela está fora dele).
    fn span_in(&self, ui: &Ui, scroll_box: ComponentId, id: ComponentId) -> Option<(f32, f32)> {
        let mut r = ui.laid_out_bounds(id)?;
        let mut parent = ui.parent(id);
        loop {
            let p = parent?;
            if p == scroll_box {
                break;
            }
            let b = ui.laid_out_bounds(p)?;
            r = r.offset(b.x, b.y);
            parent = ui.parent(p);
        }
        let o = self.base.laid_out_scroll();
        Some(if self.horizontal { (r.x + o.x, r.width) } else { (r.y + o.y, r.height) })
    }

    /// A rolagem que deixa o intervalo `(start, len)` inteiro à vista andando o mínimo (o maior
    /// que a área mostra o começo); `None` se ele já está.
    fn scroll_for(&self, (start, len): (f32, f32)) -> Option<f32> {
        let (_, view) = self.axis();
        let now = self.scroll();
        let target = if start < now || len > view {
            start
        } else if start + len > now + view {
            start + len - view
        } else {
            return None;
        };
        Some(target.clamp(0.0, self.max_scroll()).round())
    }

    /// `id` aparece na área visível: inteiro (`full`) ou ao menos uma parte.
    fn shows(&self, (start, len): (f32, f32), full: bool) -> bool {
        let (_, view) = self.axis();
        let now = self.scroll();
        if full { start >= now - 0.5 && start + len <= now + view + 0.5 } else { start + len > now && start < now + view }
    }

    /// Rola o mínimo para `child` (qualquer componente dentro da caixa, não só os filhos
    /// diretos) ficar inteiro à vista; o método `scroll_to`. Não dispara o `on_scroll`: quem
    /// rolou foi o código.
    pub fn scroll_into_view(ui: &mut Ui, scroll_box: ComponentId, child: ComponentId) {
        ui.update_layout();
        let Some(sb) = ui.get::<ScrollBox>(scroll_box) else { return };
        let Some(target) = sb.span_in(ui, scroll_box, child).and_then(|span| sb.scroll_for(span)) else { return };
        let horizontal = sb.horizontal;
        if let Some(cb) = ui.control_mut(scroll_box) {
            cb.set_scroll_offset(if horizontal { PointF::new(target, 0.0) } else { PointF::new(0.0, target) });
        }
        ui.invalidate();
    }

    /// Os filhos de `list` (um componente dentro da caixa, ou a própria caixa) que estão à
    /// vista, na ordem: inteiros (`full`) ou ao menos uma parte. É o método
    /// `visible_components`; a lista que acompanha uma posição (a fala atual) usa para saber
    /// se o item está à vista.
    pub fn visible_children(ui: &mut Ui, scroll_box: ComponentId, list: ComponentId, full: bool) -> Vec<ComponentId> {
        ui.update_layout();
        let Some(sb) = ui.get::<ScrollBox>(scroll_box) else { return Vec::new() };
        ui.children(list)
            .iter()
            .copied()
            .filter(|&c| ui.control(c).is_some_and(|cb| cb.visible()))
            .filter(|&c| sb.span_in(ui, scroll_box, c).is_some_and(|span| sb.shows(span, full)))
            .collect()
    }

    /// A propriedade `scroll_position`: gravar rola pelo código (sem `on_scroll`).
    fn set_scroll_position(&mut self, y: f32) {
        let y = y.clamp(0.0, self.max_scroll()).round();
        self.exact = y;
        self.glide = None;
        self.base.set_scroll_offset(if self.horizontal { PointF::new(y, 0.0) } else { PointF::new(0.0, y) });
    }
}

/// `id` está dentro de `ancestor`.
fn is_inside(ui: &Ui, id: ComponentId, ancestor: ComponentId) -> bool {
    let mut current = ui.parent(id);
    while let Some(c) = current {
        if c == ancestor {
            return true;
        }
        current = ui.parent(c);
    }
    false
}

/// A que distância do fim a rolagem avisa `on_end_reached`.
const END_MARGIN: f32 = 48.0;
/// Quanto puxar no topo para atualizar.
const PULL: f32 = 64.0;

impl Component for ScrollBox {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("ScrollBox");

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        horizontal: bool => "Behavior",
        pull_to_refresh: bool => "Behavior",
        refreshing: bool => "Behavior",
        wheel_step: f32 => "Behavior",
        smooth: SmoothScroll => "Behavior",
        gutter: bool => "Appearance",
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard";
        events: on_end_reached, on_refresh, on_scroll;
        computed: scroll_position: f32 => scroll / set_scroll_position;
        on_set: sync
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("scroll_to", &[PropKind::Component], None).params(&["component"]),
            MethodInfo::new("visible_components", &[PropKind::Component, PropKind::Bool], Some(PropKind::String)).params(&["list", "fully"]),
        ];
        METHODS
    }

    /// `scroll_to(componente)`: rola o mínimo para ele ficar inteiro à vista (sem `on_scroll`).
    /// `visible_components(lista, inteiros)`: os caminhos dos filhos de `lista` (vazio: a
    /// própria caixa) à vista, separados por `;`, só os inteiros ou também os cortados.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        use aylis_core::PropValue;
        let component = |i: usize| match args.get(i) {
            Some(PropValue::Component(c)) => Ok(*c),
            Some(PropValue::String(path)) if path.is_empty() => Ok(None),
            Some(PropValue::String(path)) => ctx.ui.find_path(path).map(Some).ok_or_else(|| format!("não há o componente '{path}'")),
            None | Some(PropValue::Null) => Ok(None),
            Some(other) => Err(format!("esperava um componente, veio {}", other.kind_name())),
        };
        match method {
            "scroll_to" => {
                let child = component(0)?.ok_or("scroll_to pede o componente")?;
                if let Some(target) = self.span_in(ctx.ui, ctx.id, child).and_then(|span| self.scroll_for(span)) {
                    self.set_scroll_position(target);
                    ctx.invalidate();
                }
                Ok(PropValue::Null)
            }
            "visible_components" => {
                let list = component(0)?.unwrap_or(ctx.id);
                let full = args.get(1).and_then(|v| v.to_bool().ok()).unwrap_or(false);
                let ui = &*ctx.ui;
                let shown: Vec<String> = ui
                    .children(list)
                    .iter()
                    .copied()
                    .filter(|&c| ui.control(c).is_some_and(|cb| cb.visible()))
                    .filter(|&c| self.span_in(ui, ctx.id, c).is_some_and(|span| self.shows(span, full)))
                    .map(|c| ui.path_of(c))
                    .collect();
                Ok(PropValue::String(shown.join(";")))
            }
            _ => Err(format!("ScrollBox não tem o método '{method}'")),
        }
    }

    fn scroll_axes(&self) -> (bool, bool) {
        (self.horizontal, !self.horizontal)
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if self.exact.round() != self.scroll() {
            self.exact = self.scroll(); // rolado por outro caminho (barra, scroll_into_view)
        }
        // Na horizontal vale o deslocamento lateral; sem ele (roda comum), o vertical.
        let delta = if self.horizontal && event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
        // Roda com dentes: o passo configurado (no controle ou no app) e, se pedido, o deslize.
        let options = ctx.ui.scroll_options();
        let step = if self.wheel_step > 0.0 { self.wheel_step } else { options.wheel_step };
        let delta = if event.lines && self.wheel_step > 0.0 { delta * step / options.wheel_step.max(1.0) } else { delta };
        let smooth = event.lines
            && match self.smooth {
                SmoothScroll::Default => ctx.ui.smooth_scrolling(),
                SmoothScroll::On => true,
                SmoothScroll::Off => false,
            };
        if smooth && !self.pull_to_refresh {
            // Soma ao destino do deslize em andamento (dentes seguidos aceleram).
            let from = self.exact;
            let base = self.glide.map_or(from, |(_, to, _)| to);
            let target = (base - delta).clamp(0.0, self.max_scroll());
            if (target - base).abs() < 0.5 {
                return; // no limite: a roda sobe para os pais
            }
            self.glide = Some((from, target, ctx.ui.time()));
            ctx.ui.request_frames(ctx.id);
            ctx.set_handled();
            return;
        }
        self.glide = None;
        let target = (self.exact - delta).clamp(0.0, self.max_scroll());
        if !self.horizontal && self.pull_to_refresh && !self.refreshing && self.exact <= 0.0 && event.delta.y > 0.0 {
            // No topo e puxando para baixo: estica até o limite e atualiza.
            let now = ctx.ui.time();
            let (pulled, at) = self.pull.get();
            let pulled = if now - at > 0.4 { 0.0 } else { pulled } + event.delta.y * 0.5;
            if pulled >= PULL {
                self.pull.set((0.0, now));
                self.refreshing = true;
                ctx.fire("on_refresh", EventArgs::None);
            } else {
                self.pull.set((pulled, now));
            }
            ctx.repaint();
            ctx.set_handled();
            return;
        }
        if target == self.exact {
            return; // no limite: não trata (a roda sobe para os pais, a inércia para)
        }
        self.exact = target;
        self.set_scroll(ctx, target, true);
        ctx.set_handled();
    }

    /// Um passo do deslize da roda (desacelerando no fim), em cada quadro e no tempo dele: anda
    /// junto com o monitor (60, 144, 160 Hz), sem degraus. No meio do caminho a posição não é
    /// arredondada (com a escala do sistema, 1 ponto são 1,5 pixel); no fim, sim.
    fn frame(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        let Some((from, to, at)) = self.glide else { return false };
        let ms = ctx.ui.scroll_options().smooth_ms.max(1.0) as f64;
        let k = ((ctx.ui.time() - at) * 1000.0 / ms).clamp(0.0, 1.0) as f32;
        let eased = 1.0 - (1.0 - k).powi(3);
        let y = from + (to - from) * eased;
        self.exact = y;
        if k >= 1.0 {
            self.glide = None;
            self.set_scroll(ctx, y, true);
            return false;
        }
        self.set_scroll_exact(ctx, y, true);
        true
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let mut y = self.scroll();
        if self.horizontal {
            self.set_scroll(ctx, y, false);
            return;
        }
        // Encolheu (no celular, o teclado virtual abriu): mantém à vista o campo com foco.
        let root = ctx.ui.root_of(ctx.id);
        if let Some(focused) = ctx.ui.focused(root).filter(|&f| is_inside(ctx.ui, f, ctx.id)) {
            let view = ctx.ui.to_form(ctx.id, self.base.bounds());
            if let Some(r) = ctx.ui.absolute_rect(focused) {
                const GAP: f32 = 8.0;
                if r.bottom() + GAP > view.bottom() {
                    y += r.bottom() + GAP - view.bottom();
                } else if r.y - GAP < view.y {
                    y -= view.y - (r.y - GAP);
                }
            }
        }
        self.set_scroll(ctx, y, false);
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(thumb) = self.thumb() {
            let on_bar = if self.horizontal { event.position.y >= thumb.y - 2.0 } else { event.position.x >= thumb.x - 2.0 };
            if on_bar {
                self.dragging = Some(if self.horizontal { event.position.x - thumb.x } else { event.position.y - thumb.y });
                ctx.set_handled();
            }
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let (Some(offset), Some(thumb)) = (self.dragging, self.thumb()) else { return };
        let b = self.base.bounds();
        let (room, pos) = if self.horizontal { (b.width - thumb.width, event.position.x) } else { (b.height - thumb.height, event.position.y) };
        let fraction = ((pos - offset) / room.max(1.0)).clamp(0.0, 1.0);
        self.set_scroll(ctx, fraction * self.max_scroll(), true);
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = None;
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !ctx.style.fill.is_transparent() {
            canvas.fill_rect(ctx.rect, ctx.style.radius, &Brush::Solid(ctx.style.fill));
        }
    }

    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (pulled, at) = self.pull.get();
        let pulled = if ctx.time - at > 0.4 { 0.0 } else { pulled };
        if self.refreshing || pulled > 0.0 {
            // O indicador do "puxar para atualizar": desce com o dedo e gira atualizando.
            let d = 32.0;
            let y = if self.refreshing { 12.0 } else { (pulled / PULL) * 12.0 - 8.0 };
            let c = PointF::new(ctx.rect.width / 2.0, y + d / 2.0);
            canvas.fill_ellipse(RectF::new(c.x - d / 2.0, c.y - d / 2.0, d, d), &Brush::Solid(ctx.theme.color("surface")));
            canvas.stroke_ellipse(RectF::new(c.x - d / 2.0, c.y - d / 2.0, d, d), &aylis_core::Stroke::new(ctx.theme.color("border"), 1.0));
            let (start, sweep) = if self.refreshing { ((ctx.time as f32 * 360.0) % 360.0, 270.0) } else { (-90.0, 300.0 * pulled / PULL) };
            let mut arc = aylis_core::Path::new();
            arc.arc(c, 9.0, 9.0, start, sweep);
            canvas.stroke_path(&arc, &aylis_core::Stroke::new(ctx.theme.color("primary"), 2.5).round());
            ctx.request_frame();
        }
        if let Some(thumb) = self.thumb() {
            let alpha = if self.dragging.is_some() { 0.5 } else { 0.3 };
            canvas.fill_rect(thumb, BAR / 2.0, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(alpha)));
        }
    }
}

// ---------------------------------------------------------------------------

/// Divisória arrastável: redimensiona o controle encaixado antes
/// dela com o mesmo alinhamento (`left`, `right`, `top` ou `bottom`).
pub struct Splitter {
    pub base: ControlBase,
    /// Tamanho mínimo do controle redimensionado.
    pub min_size: f32,
    drag: Option<(ComponentId, f32, f32)>,
}

impl Default for Splitter {
    fn default() -> Self {
        let mut base = ControlBase::new(6.0, 6.0);
        // No toque, arrastar o Splitter redimensiona (não rola a tela).
        base.set_align(Align::Left).set_cursor(CursorShape::ResizeHorizontal).set_pan_on_touch(false);
        Self { base, min_size: 60.0, drag: None }
    }
}

impl Splitter {
    pub fn new(align: Align) -> Self {
        let mut s = Self::default();
        s.base.set_align(align).set_cursor(match align {
            Align::Top | Align::Bottom | Align::MostTop | Align::MostBottom => CursorShape::ResizeVertical,
            _ => CursorShape::ResizeHorizontal,
        });
        s
    }

    fn vertical_bar(&self) -> bool {
        matches!(self.base.align(), Align::Left | Align::Right | Align::MostLeft | Align::MostRight)
    }

    /// O controle encaixado imediatamente antes, com o mesmo alinhamento.
    fn target(&self, ui: &Ui, id: ComponentId) -> Option<ComponentId> {
        let parent = ui.parent(id)?;
        let siblings = ui.children(parent);
        let me = siblings.iter().position(|&c| c == id)?;
        siblings[..me].iter().rev().copied().find(|&c| ui.control(c).is_some_and(|cb| cb.align() == self.base.align() && cb.visible()))
    }

    fn absolute(&self, ctx: &EventCtx<'_>, local: PointF) -> f32 {
        let origin = ctx.ui.to_form(ctx.id, self.base.bounds()).origin();
        if self.vertical_bar() { origin.x + local.x } else { origin.y + local.y }
    }
}

impl Component for Splitter {
    leaf_control!("Splitter");

    properties! {
        min_size: f32 => "Behavior",
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let Some(target) = self.target(ctx.ui, ctx.id) else { return };
        let Some(cb) = ctx.ui.control(target) else { return };
        let size = if self.vertical_bar() { cb.width() } else { cb.height() };
        self.drag = Some((target, self.absolute(ctx, event.position), size));
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some((target, start, size)) = self.drag else { return };
        let delta = self.absolute(ctx, event.position) - start;
        let grows = matches!(self.base.align(), Align::Left | Align::Top | Align::MostLeft | Align::MostTop);
        let new_size = (if grows { size + delta } else { size - delta }).max(self.min_size);
        let vertical = self.vertical_bar();
        if let Some(cb) = ctx.ui.control_mut(target) {
            if vertical { cb.set_width(new_size) } else { cb.set_height(new_size) };
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.drag = None;
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = if self.drag.is_some() || ctx.state.hover {
            ctx.theme.color("primary").with_opacity(0.5)
        } else {
            ctx.theme.color("border")
        };
        let line = if self.vertical_bar() {
            RectF::new(r.center().x - 0.5, r.y, 1.0, r.height)
        } else {
            RectF::new(r.x, r.center().y - 0.5, r.width, 1.0)
        };
        canvas.fill_rect(line, 0.0, &Brush::Solid(color));
    }
}
