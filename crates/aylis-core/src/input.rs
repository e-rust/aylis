//! Despacho de entrada: ponteiro (mouse/toque), roda, teclado e foco.
//!
//! A plataforma traduz os eventos nativos e chama os métodos `pointer_*`, `wheel`,
//! `key_*` e `text_input` do [`Ui`], sempre informando o form (raiz) e posições em
//! coordenadas do form. O `Ui` encontra o controle certo, atualiza os estados (hover,
//! pressed, focused), chama os métodos do componente e dispara os eventos públicos
//! (`on_click`, `on_pointer_down`...).
//!
//! Regras:
//! - o controle sob o ponteiro é o mais de cima com `hit_test = true`; controles com
//!   `hit_test = false` deixam o ponteiro passar para os filhos e para o que está atrás;
//! - controles desabilitados (ou com algum ancestral desabilitado) não recebem eventos;
//! - ao pressionar o botão principal, o controle "captura" o ponteiro: recebe os
//!   movimentos e o soltar mesmo fora dele;
//! - o clique só acontece se o botão for solto sobre o mesmo controle;
//! - a roda vai para o controle sob o ponteiro e sobe para os pais até alguém tratar;
//! - o teclado vai para o controle com foco e sobe para os pais até alguém tratar;
//!   Tab / Shift+Tab, se ninguém tratar, movem o foco;
//! - no toque, arrastar o dedo além de alguns pontos vira rolagem: o controle tocado
//!   perde o toque (não há clique) e o deslocamento vai como roda (`touch = true`) para
//!   o ScrollBox/DataGrid de baixo; ao soltar com o dedo em movimento, a rolagem continua
//!   por inércia. Controles com `pan_on_touch` desligado recebem o arrasto normalmente;
//! - no toque, o foco muda ao soltar (um toque), não ao encostar: começar a rolar em
//!   cima de um campo de texto não abre o teclado.
//! - arrastar e soltar entre controles:
//!   num controle `draggable` (ou dentro dele), passar de [`DRAG_START`] pontos com o
//!   botão principal pressionado dispara `on_drag_start`; se o handler chamar
//!   [`Ui::begin_drag`], o arrasto começa (sem clique ao soltar). O destino é o controle
//!   `accepts_drops` sob o mouse (ou o pai mais próximo que aceite): `on_drag_over` ao
//!   entrar (recusar com [`Ui::reject_drop`]), `on_drag_drop` ao soltar nele; a origem
//!   recebe `on_drag_end` (`true` se foi solto num destino). Esc cancela.

use crate::anim::Trigger;
use crate::component::{Component, ComponentId};
use crate::event::{EventArgs, EventCtx, Key, KeyEvent, PointerButton, PointerEvent, PointerKind, WheelEvent};
use crate::geometry::{PointF, RectF, SizeF};
use crate::platform::CursorShape;
use crate::ui::Ui;

/// Intervalo máximo entre cliques para contar como duplo clique, em segundos.
const DOUBLE_CLICK_TIME: f64 = 0.5;
/// Distância máxima entre cliques para contar como duplo clique.
const DOUBLE_CLICK_DISTANCE: f32 = 4.0;
/// Quanto o mouse anda, com o botão pressionado num controle `draggable`, até o arrasto começar.
pub const DRAG_START: f32 = 5.0;
/// Quanto o dedo anda antes de o toque virar rolagem.
const TOUCH_SLOP: f32 = 8.0;
/// O deslizar (`on_swipe`): o dedo anda pelo menos isto, saindo a pelo menos esta velocidade
/// (pontos por segundo), sem ter rolado mais que isto no mesmo eixo.
const SWIPE_DISTANCE: f32 = 32.0;
const SWIPE_SPEED: f32 = 250.0;
const SWIPE_SCROLLED: f32 = 4.0;
/// A faixa da borda do form em que um deslizar abre a gaveta (fora dela, o gesto de voltar do
/// Android, que pega a borda de fora, não briga com o app).
const EDGE_SWIPE: f32 = 32.0;
/// Inércia: abaixo desta velocidade (unidades por segundo) a rolagem para...
const FLING_MIN_SPEED: f32 = 40.0;
/// ...e a velocidade cai por este fator exponencial por segundo.
const FLING_DECAY: f32 = 3.5;

/// O dedo que comanda o toque (rolagem, clique). Dois dedos viram pinça ([`Pinch`]).
#[derive(Clone, Copy, Debug)]
struct Touch {
    id: u64,
    /// Controle tocado: a rolagem começa nele e sobe para os pais, mesmo que o conteúdo
    /// passe por baixo do dedo.
    target: Option<ComponentId>,
    start: PointF,
    last: PointF,
    time: f64,
    panning: bool,
    velocity: PointF,
    /// Quanto o arrasto rolou alguma coisa em cada eixo: o deslizar só vale no eixo em que
    /// nada rolou (senão era rolagem).
    scrolled: PointF,
}

/// Botão (ou dedo) segurado: vira toque longo se ficar parado.
#[derive(Clone, Copy, Debug)]
struct Press {
    target: ComponentId,
    /// Posição no form.
    position: PointF,
    time: f64,
    touch: bool,
    /// Já disparou: o soltar não gera clique.
    fired: bool,
}

/// Rolagem por inércia depois de soltar o dedo.
#[derive(Clone, Copy, Debug)]
struct Fling {
    target: Option<ComponentId>,
    position: PointF,
    velocity: PointF,
    time: f64,
}

fn speed(v: PointF) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}

/// Estado de entrada de um form.
#[derive(Clone, Debug, Default)]
pub(crate) struct InputState {
    /// O controle com arquivos do sistema passando por cima ([`Ui::files_hover`]).
    pub(crate) file_hover: Option<ComponentId>,
    hovered: Option<ComponentId>,
    captured: Option<ComponentId>,
    focused: Option<ComponentId>,
    /// O foco veio de um clique ou toque (não do teclado): o anel de foco só aparece nos
    /// campos de texto, como o `:focus-visible` dos navegadores.
    pointer_focus: bool,
    /// Último clique: controle, tempo, posição, contagem.
    last_click: Option<(ComponentId, f64, PointF, u8)>,
    touch: Option<Touch>,
    fling: Option<Fling>,
    press: Option<Press>,
    pub(crate) hint: Option<HintState>,
    /// Os dedos na tela agora (id, posição no form).
    fingers: Vec<(u64, PointF)>,
    pinch: Option<Pinch>,
    /// Um gesto de dois dedos acabou e ainda há dedo na tela: ele não vira toque.
    after_pinch: bool,
    /// Arrasto entre controles em andamento.
    pub(crate) drag: Option<Drag>,
    /// Uma tecla espera a resposta do backend (`on_key_down`/`on_key_press` remotos): as que
    /// chegam ficam em `held`, em ordem (veja `keyflow.rs`).
    pub(crate) holding: bool,
    pub(crate) held: std::collections::VecDeque<crate::keyflow::Held>,
    /// Entregando a fila agora (o que ela entrega não volta para ela).
    pub(crate) draining: bool,
    /// O último pressionar foi recusado (o backend ocupado): o soltar dele não vira clique.
    pub(crate) refused_press: bool,
}

/// Um arrasto entre controles: a origem, o que vai (texto livre do app), o rótulo que
/// acompanha o mouse e o destino sob ele.
#[derive(Clone, Debug)]
pub(crate) struct Drag {
    pub(crate) source: ComponentId,
    pub(crate) data: String,
    pub(crate) label: String,
    /// Posição do mouse no form.
    pub(crate) position: PointF,
    pub(crate) target: Option<ComponentId>,
    pub(crate) accepted: bool,
    /// O `on_drag_start` remoto ainda não deu os dados (o número da pergunta).
    pub(crate) starting: Option<u64>,
    /// O `on_drag_over` remoto do destino de agora ainda não respondeu.
    pub(crate) over: Option<(ComponentId, u64)>,
    /// Soltou antes da resposta: solta quando ela chegar.
    pub(crate) dropped: bool,
    /// O que acompanha o cursor (a miniatura do item), além do rótulo.
    pub(crate) visual: Option<DragVisual>,
    /// O mouse no form do destino (que pode ser outra janela): para os ganchos `drop_*`.
    pub(crate) target_position: PointF,
}

/// Como acabou um arrasto (para o [`Component::drag_ended`](crate::Component::drag_ended) da
/// origem).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragEnd {
    /// Soltou num destino que aceitou.
    Dropped,
    /// Soltou onde ninguém aceita (fora das janelas do app, inclusive).
    Nowhere,
    /// Esc ou a janela perdeu o arrasto.
    Cancelled,
}

/// O mouse na tela durante um arrasto: pixels físicos e a escala (pixels por ponto) da
/// janela onde o arrasto começou.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenPointer {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
}

/// O que acompanha o cursor num arrasto: uma imagem (a miniatura de um vídeo, o ícone de um
/// arquivo), do tamanho pedido, meio transparente, presa ao cursor pelo ponto `hotspot`. No
/// Windows ela vai numa janela própria e segue o mouse por cima de tudo, fora da janela
/// também; nos outros sistemas, o form a pinta junto do mouse.
#[derive(Clone, Debug, PartialEq)]
pub struct DragVisual {
    pub image: crate::Image,
    /// Tamanho na tela, em pontos.
    pub size: SizeF,
    /// O ponto da imagem (em pontos, a partir do canto) que fica no cursor.
    pub hotspot: PointF,
    /// 0..1.
    pub opacity: f32,
}

impl DragVisual {
    /// A imagem do tamanho `size`, com o cursor um pouco à esquerda e acima do centro (como o
    /// Explorer) e 80% de opacidade.
    pub fn new(image: crate::Image, size: SizeF) -> Self {
        Self { image, size, hotspot: PointF::new(size.width / 2.0, size.height * 0.6), opacity: 0.8 }
    }
}

/// Pinça em andamento: o controle que recebe e a medida inicial e anterior dos dedos.
#[derive(Clone, Copy, Debug)]
struct Pinch {
    target: ComponentId,
    ids: (u64, u64),
    start_distance: f32,
    start_angle: f32,
    last_distance: f32,
    last_angle: f32,
    last_center: PointF,
}

/// Distância, ângulo e centro entre dois pontos.
fn finger_geometry(a: PointF, b: PointF) -> (f32, f32, PointF) {
    let d = b - a;
    ((d.x * d.x + d.y * d.y).sqrt().max(1.0), d.y.atan2(d.x), PointF::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0))
}

/// A dica (balão) do controle sob o mouse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HintState {
    pub(crate) target: ComponentId,
    pub(crate) since: f64,
    /// Onde o mouse estava (coordenadas do form).
    pub(crate) position: PointF,
    pub(crate) shown: bool,
}

/// Tempo com o mouse parado em cima até a dica aparecer, em segundos: o padrão, quando nem o
/// tema (`durations.hint`, em ms) nem o app (`Ui::set_hint_delay`) dizem outro.
pub const HINT_DELAY: f64 = 1.0;

/// Quanto o mouse pode tremer (em pixels) e ainda contar como parado para a dica.
const HINT_STILL: f32 = 3.0;

impl InputState {
    /// Quando a dica pendente deve aparecer.
    pub(crate) fn hint_due(&self, delay: f64) -> Option<f64> {
        self.hint.filter(|h| !h.shown).map(|h| h.since + delay)
    }

    /// Quando o toque longo em andamento vence (se houver).
    /// Só com o dedo ou a caneta (J18 do Tabernáculo): o mouse não conta, e nem acorda o laço.
    pub(crate) fn long_press_due(&self) -> Option<f64> {
        self.press.filter(|p| !p.fired && p.touch).map(|p| p.time + crate::timers::LONG_PRESS_TIME)
    }

    /// O toque longo venceu: marca como disparado e devolve (alvo, posição, é toque).
    pub(crate) fn take_long_press(&mut self, now: f64) -> Option<(ComponentId, PointF, bool)> {
        let p = self.press.as_mut()?;
        if p.fired || !p.touch || now < p.time + crate::timers::LONG_PRESS_TIME {
            return None;
        }
        p.fired = true;
        // O botão segurado não fica "pressionado" visualmente depois do toque longo.
        Some((p.target, p.position, p.touch))
    }

    /// Esquece componentes liberados.
    pub(crate) fn forget(&mut self, freed: &[ComponentId]) {
        let gone = |id: &mut Option<ComponentId>| {
            if id.is_some_and(|i| freed.contains(&i)) {
                *id = None;
            }
        };
        gone(&mut self.hovered);
        gone(&mut self.captured);
        gone(&mut self.focused);
        if let Some(t) = &mut self.touch {
            gone(&mut t.target);
        }
        if let Some(f) = &mut self.fling {
            gone(&mut f.target);
        }
        if self.press.is_some_and(|p| freed.contains(&p.target)) {
            self.press = None;
        }
        if self.last_click.is_some_and(|(i, ..)| freed.contains(&i)) {
            self.last_click = None;
        }
    }
}

impl Ui {
    fn input_state(&mut self, root: ComponentId) -> &mut InputState {
        self.input.entry(root).or_default()
    }

    /// O foco do form veio de um clique ou toque (o anel de foco fica só nos campos de texto).
    pub(crate) fn focus_from_pointer(&self, root: ComponentId) -> bool {
        self.input.get(&root).is_some_and(|s| s.pointer_focus)
    }

    /// O foco do form veio do teclado (Tab, atalho): o anel de foco aparece, como o
    /// `:focus-visible` do CSS (o relatório do Tabernáculo, J08).
    pub fn focus_visible(&self, form: ComponentId) -> bool {
        !self.focus_from_pointer(self.root_of(form))
    }

    /// Os menus, listas e camadas abertos por cima do form (a lista de um ComboBox, um menu de
    /// contexto, um calendário): os que fecham com o clique fora (o relatório do Tabernáculo,
    /// J09).
    pub fn open_popups(&self, form: ComponentId) -> Vec<ComponentId> {
        let root = self.root_of(form);
        self.children(root).iter().copied().filter(|&c| self.component(c).is_some_and(|p| p.closes_on_outside_press() || p.dies_with().is_some())).collect()
    }

    // =====================================================================
    // Consultas
    // =====================================================================

    /// Arquivos soltos na janela do form `root` no ponto `pos`: vão para o controle sob o
    /// ponto e sobem pelos pais até um aceitar ([`Component::files_dropped`]). Se nenhum
    /// aceitar (o sistema nem sempre informa a posição), o primeiro controle à vista do form
    /// que aceita arquivos recebe (um escondido ou desabilitado, nem ele nem os filhos: o
    /// relatório do Tabernáculo, F01). Devolve `true` se alguém aceitou.
    pub fn drop_files(&mut self, root: ComponentId, pos: PointF, paths: Vec<String>) -> bool {
        let mut target = self.hit_test(root, pos);
        // Os que já receberam (e recusaram) não recebem de novo na procura de reserva.
        let mut tried = Vec::new();
        while let Some(id) = target {
            if self.with_component(id, |c, ctx| c.platform_hooks_mut().is_some_and(|h| h.files_dropped(ctx, &paths))).unwrap_or(false) {
                self.process_events();
                return true;
            }
            tried.push(id);
            target = self.parent(id);
        }
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if self.cb(id).is_some_and(|cb| !cb.visible() || !cb.enabled()) {
                continue;
            }
            if tried.contains(&id) {
                stack.extend(self.children(id).iter().rev().copied());
                continue;
            }
            if self.with_component(id, |c, ctx| c.platform_hooks_mut().is_some_and(|h| h.files_dropped(ctx, &paths))).unwrap_or(false) {
                self.process_events();
                return true;
            }
            stack.extend(self.children(id).iter().rev().copied());
        }
        // O `on_reject` de quem recusou.
        self.process_events();
        false
    }

    /// Arquivos do sistema passando por cima da janela do form (`Some`) ou saindo dela, soltos
    /// ou cancelados (`None`): o controle à vista que os receberia fica sabendo, para se
    /// destacar (o relatório do Tabernáculo, F03).
    pub fn files_hover(&mut self, root: ComponentId, pos: PointF, paths: Option<Vec<String>>) {
        let previous = self.input_state(root).file_hover.take();
        let Some(paths) = paths else {
            if let Some(old) = previous.filter(|&o| self.contains(o)) {
                self.with_component(old, |c, ctx| c.platform_hooks_mut().map(|h| h.files_hovering(ctx, None)));
                self.process_events();
            }
            return;
        };
        let mut found = None;
        let mut target = self.hit_test(root, pos);
        while let Some(id) = target {
            if self.with_component(id, |c, ctx| c.platform_hooks_mut().is_some_and(|h| h.files_hovering(ctx, Some(&paths)))).unwrap_or(false) {
                found = Some(id);
                break;
            }
            target = self.parent(id);
        }
        if let Some(old) = previous.filter(|&o| Some(o) != found && self.contains(o)) {
            self.with_component(old, |c, ctx| c.platform_hooks_mut().map(|h| h.files_hovering(ctx, None)));
        }
        self.input_state(root).file_hover = found;
        self.process_events();
    }

    /// Controle sob o ponto `pos` (coordenadas do form).
    pub fn hit_test(&self, root: ComponentId, pos: PointF) -> Option<ComponentId> {
        self.hit_node(root, pos)
    }

    fn hit_node(&self, id: ComponentId, pos_in_parent: PointF) -> Option<ComponentId> {
        let cb = self.cb(id)?;
        if !cb.visible() || (cb.design_only && !self.design_mode) {
            return None;
        }
        let b = cb.bounds();
        // No espaço de dentro (desfeitos o lugar, o deslocamento, o giro e a escala): o mouse
        // acerta onde o controle está desenhado (o `translate` de uma animação, da linha
        // deslizada da lista, da página em transição), como a pintura.
        let local = unscale(cb, unrotate(cb, pos_in_parent - b.origin() - cb.translate()));
        let b = RectF::from_size(cb.inner_size());
        let inside = local.x >= 0.0 && local.y >= 0.0 && local.x < b.width && local.y < b.height;
        if cb.clip_children() && !inside {
            return None;
        }
        if let Some(hit) = self.children(id).iter().rev().find_map(|&c| self.hit_node(c, local)) {
            return Some(hit);
        }
        let shape = || self.component(id).is_none_or(|c| c.contains_point(local, SizeF::new(b.width, b.height)));
        // Um pouco além da borda, para quem desenha alças do lado de fora (`hit_slop`).
        let near = || {
            let m = self.component(id).map_or(0.0, |c| c.hit_slop());
            m > 0.0 && local.x >= -m && local.y >= -m && local.x < b.width + m && local.y < b.height + m
        };
        ((inside || near()) && cb.hit_test() && shape()).then_some(id)
    }

    /// O controle e todos os ancestrais estão visíveis e habilitados.
    pub fn is_effectively_enabled(&self, id: ComponentId) -> bool {
        let mut current = Some(id);
        while let Some(c) = current {
            if let Some(cb) = self.cb(c) {
                if !cb.enabled() || !cb.visible() {
                    return false;
                }
            }
            current = self.parent(c);
        }
        self.contains(id)
    }

    /// Controle sob o ponteiro no form.
    pub fn hovered(&self, root: ComponentId) -> Option<ComponentId> {
        self.input.get(&root)?.hovered
    }

    /// Controle com o foco do teclado no form.
    pub fn focused(&self, root: ComponentId) -> Option<ComponentId> {
        self.input.get(&root)?.focused
    }

    /// Cursor a mostrar: o do controle sob o ponteiro ou, se for `Default`, o do pai mais
    /// próximo que defina um.
    pub fn cursor(&self, root: ComponentId) -> CursorShape {
        // O backend ocupado passou do `delay`: o cursor de espera (veja `busy.rs`).
        if self.busy_cursor() {
            return CursorShape::Wait;
        }
        let mut current = self.hovered(root);
        while let Some(c) = current {
            if let Some(cb) = self.cb(c) {
                if cb.cursor() != CursorShape::Default {
                    return cb.cursor();
                }
            }
            current = self.parent(c);
        }
        CursorShape::Default
    }

    /// O ponto `pos` (coordenadas do form) nas coordenadas do controle, desfazendo a
    /// posição e o giro de cada ancestral e do próprio controle.
    pub fn local_point(&self, id: ComponentId, pos: PointF) -> PointF {
        let mut chain = vec![id];
        let mut parent = self.parent(id);
        while let Some(p) = parent {
            chain.push(p);
            parent = self.parent(p);
        }
        let local = chain.iter().rev().fold(pos, |pt, &c| match self.cb(c) {
            Some(cb) => unscale(cb, unrotate(cb, pt - cb.bounds().origin() - cb.translate())),
            None => pt,
        });
        // Interface espelhada: o controle se desenha virado, então o mouse também vira.
        match self.cb(id).filter(|_| self.flips_content(id)) {
            Some(cb) => PointF::new(cb.inner_size().width - local.x, local.y),
            None => local,
        }
    }

    fn enabled_hit(&self, root: ComponentId, pos: PointF) -> Option<ComponentId> {
        self.hit_test(root, pos).filter(|&id| self.is_effectively_enabled(id))
    }

    /// Chama um método do componente e devolve se ele marcou o evento como tratado.
    pub(crate) fn deliver(&mut self, id: ComponentId, f: impl FnOnce(&mut dyn Component, &mut EventCtx<'_>)) -> bool {
        self.with_component(id, |c, ctx| {
            f(c, ctx);
            ctx.handled()
        })
        .unwrap_or(false)
    }

    fn set_state_flag(&mut self, id: ComponentId, trigger: Trigger, value: bool) {
        let Some(cb) = self.cb_mut(id) else { return };
        let state = cb.state_mut();
        let flag = match trigger {
            Trigger::Hover => &mut state.hover,
            Trigger::Pressed => &mut state.pressed,
            Trigger::Focused => &mut state.focused,
            Trigger::Checked => &mut state.checked,
            Trigger::None | Trigger::Appear => return,
        };
        if *flag != value {
            *flag = value;
            // O desenho dele não depende desse estado (o fundo do form quando o mouse sai de
            // um botão para ele): nada a repintar.
            // Um filho que herda o estado muda junto (C25): a área toda dele.
            let heirs = self.children(id).iter().any(|&c| self.cb(c).is_some_and(|cb| cb.inherit_state()));
            if heirs || !self.drawing_unchanged(id) {
                self.damage(id);
            }
            self.run_triggers(id, trigger, value);
        }
    }

    // =====================================================================
    // Ponteiro
    // =====================================================================

    fn set_hover(&mut self, root: ComponentId, new: Option<ComponentId>) {
        let old = self.input_state(root).hovered;
        if old == new {
            return;
        }
        self.input_state(root).hovered = new;
        if let Some(old) = old.filter(|&o| self.contains(o)) {
            self.set_state_flag(old, Trigger::Hover, false);
            self.fire(old, "on_pointer_leave", &EventArgs::None);
            self.deliver(old, |c, ctx| c.pointer_leave(ctx));
        }
        if let Some(new) = new {
            self.set_state_flag(new, Trigger::Hover, true);
            self.fire(new, "on_pointer_enter", &EventArgs::None);
            self.deliver(new, |c, ctx| c.pointer_enter(ctx));
        }
    }

    /// O ponteiro se moveu. `event.position` em coordenadas do form.
    pub fn pointer_move(&mut self, root: ComponentId, event: PointerEvent) {
        self.report(&crate::activity::Activity::PointerMove { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        self.update_layout();
        if event.kind == PointerKind::Touch && (self.finger_move(root, event) || self.touch_pan(root, event)) {
            return;
        }
        if self.drag_move(root, event) {
            return;
        }
        let state = self.input_state(root);
        if state.press.is_some_and(|p| !p.fired && ((p.position.x - event.position.x).abs() > TOUCH_SLOP || (p.position.y - event.position.y).abs() > TOUCH_SLOP)) {
            state.press = None;
        }
        let hit = self.enabled_hit(root, event.position);
        self.set_hover(root, hit);
        self.update_hint(root, hit, event.position, event.kind == PointerKind::Touch);
        let target = self.input_state(root).captured.or(hit);
        if let Some(target) = target {
            let e = PointerEvent { position: self.local_point(target, event.position), ..event };
            self.fire(target, "on_pointer_move", &EventArgs::Pointer(e));
            self.deliver(target, |c, ctx| c.pointer_move(ctx, &e));
        }
    }

    /// A dica a mostrar: a do controle sob o mouse ou do pai mais próximo que tenha uma.
    fn update_hint(&mut self, root: ComponentId, hit: Option<ComponentId>, position: PointF, touch: bool) {
        let mut target = None;
        let mut cur = hit;
        while let Some(c) = cur.filter(|_| !touch) {
            if self.cb(c).is_some_and(|cb| !cb.hint().is_empty()) {
                target = Some(c);
                break;
            }
            if c == root {
                break;
            }
            cur = self.parent(c);
        }
        let now = self.time;
        let state = self.input_state(root);
        let old = state.hint;
        match (old, target) {
            (Some(h), Some(t)) if h.target == t => {
                // Ainda escondida: acompanha o mouse, e o tempo conta de novo enquanto ele anda
                // (a dica é para o mouse parado em cima, não para quem só passa); um tremido de
                // poucos pixels não conta. Já aberta, fica parada.
                if !h.shown {
                    let d = position - h.position;
                    let moved = d.x.abs() > HINT_STILL || d.y.abs() > HINT_STILL;
                    state.hint = Some(if moved { HintState { position, since: now, ..h } } else { h });
                }
                return;
            }
            (_, Some(t)) => state.hint = Some(HintState { target: t, since: now, position, shown: false }),
            (_, None) => state.hint = None,
        }
        if old.is_some_and(|h| h.shown) {
            self.request_repaint();
        }
    }

    /// Esconde a dica (clique, roda, saída da janela).
    pub(crate) fn hide_hint(&mut self, root: ComponentId) {
        if let Some(h) = self.input_state(root).hint.take() {
            if h.shown {
                self.request_repaint();
            }
        }
    }

    /// A dica aberta agora no form: (texto, posição do mouse no form).
    pub fn visible_hint(&self, root: ComponentId) -> Option<(String, PointF)> {
        let h = self.input.get(&root)?.hint.filter(|h| h.shown)?;
        let text = self.cb(h.target)?.hint().to_string();
        Some((text, h.position))
    }

    /// Um botão foi pressionado (ou um dedo tocou a tela).
    pub fn pointer_down(&mut self, root: ComponentId, event: PointerEvent) {
        self.report(&crate::activity::Activity::PointerDown { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        // O backend ainda roda um handler: o clique novo é recusado (o form continua pintando).
        if self.busy_blocks_input() {
            self.input_state(root).refused_press = true;
            return;
        }
        // Os botões extras do mouse (voltar e avançar): a navegação do form, como num
        // navegador. Sem `on_navigate` tratando, seguem como um botão qualquer.
        if matches!(event.button, PointerButton::Back | PointerButton::Forward) && self.navigate(root, event.button == PointerButton::Forward) {
            return;
        }
        self.hide_hint(root);
        self.update_layout();
        let hit = self.enabled_hit(root, event.position);
        self.dismiss_popups(root, hit);
        let touch = event.kind == PointerKind::Touch;
        if touch {
            if self.finger_down(root, event) {
                return;
            }
            let now = self.time;
            let state = self.input_state(root);
            if state.touch.is_some_and(|t| t.id != event.pointer_id) {
                return;
            }
            // Tocar na tela para a inércia (como em qualquer celular).
            state.fling = None;
            let p = event.position;
            state.touch = Some(Touch { id: event.pointer_id, target: hit, start: p, last: p, time: now, panning: false, velocity: PointF::ZERO, scrolled: PointF::ZERO });
        }
        self.set_hover(root, hit);
        let Some(target) = hit else { return };

        let now = self.time;
        let state = self.input_state(root);
        let click_count = match state.last_click {
            Some((id, time, pos, count))
                if id == target
                    && now - time <= DOUBLE_CLICK_TIME
                    && (pos.x - event.position.x).abs() <= DOUBLE_CLICK_DISTANCE
                    && (pos.y - event.position.y).abs() <= DOUBLE_CLICK_DISTANCE =>
            {
                count.saturating_add(1)
            }
            _ => 1,
        };
        state.last_click = Some((target, now, event.position, click_count));
        if event.button == PointerButton::Primary {
            state.captured = Some(target);
            state.press = Some(Press { target, position: event.position, time: now, touch, fired: false });
        }

        if event.button == PointerButton::Primary {
            self.set_state_flag(target, Trigger::Pressed, true);
        }
        if !touch {
            if let Some(focusable) = self.focusable_ancestor(target) {
                self.input_state(root).pointer_focus = true;
                self.set_focus(focusable);
            }
        }

        let e = PointerEvent { position: self.local_point(target, event.position), click_count, ..event };
        self.fire(target, "on_pointer_down", &EventArgs::Pointer(e));
        if click_count == 2 && event.button == PointerButton::Primary {
            self.fire(target, "on_dbl_click", &EventArgs::Pointer(e));
        }
        self.deliver(target, |c, ctx| c.pointer_down(ctx, &e));
    }

    /// As camadas do form que fecham com o mouse apertado fora
    /// ([`Component::closes_on_outside_press`]): sai a que não contém o ponto e cujo dono
    /// também não. O dono repinta (a seta do ComboBox volta). O clique segue para quem está
    /// embaixo (outro ComboBox abre a lista dele). A camada com o foco o perde antes de sair:
    /// quem grava ao sair (o campo de renomear do MediaCard) grava.
    fn dismiss_popups(&mut self, root: ComponentId, hit: Option<ComponentId>) {
        // O ponto caiu em `area` ou num descendente dela.
        let inside = |ui: &Ui, area: ComponentId| {
            let mut current = hit;
            while let Some(c) = current {
                if c == area {
                    return true;
                }
                current = ui.parent(c);
            }
            false
        };
        let doomed: Vec<(ComponentId, Option<ComponentId>)> = self
            .children(root)
            .iter()
            .filter(|&&c| self.cb(c).is_none_or(|cb| cb.visible()))
            .filter_map(|&c| self.component(c).filter(|p| p.closes_on_outside_press()).map(|p| (c, p.dies_with())))
            .filter(|&(popup, owner)| !inside(self, popup) && !owner.is_some_and(|o| inside(self, o)))
            .collect();
        for (popup, owner) in doomed {
            let mut focused = self.input_state(root).focused;
            while let Some(f) = focused.filter(|&f| f != popup) {
                focused = self.parent(f);
            }
            if focused.is_some() {
                self.clear_focus(root);
            }
            let kept = self.with_component(popup, |c, ctx| c.dismissed(ctx)).unwrap_or(false);
            if !kept {
                self.free(popup);
            }
            if let Some(owner) = owner.filter(|&o| self.contains(o)) {
                self.damage(owner);
            }
        }
    }

    /// Um botão foi solto. Se for o principal e for solto sobre o mesmo controle em que
    /// foi pressionado, gera o clique.
    pub fn pointer_up(&mut self, root: ComponentId, event: PointerEvent) {
        self.report(&crate::activity::Activity::PointerUp { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        // O pressionar foi recusado (o backend estava ocupado): o soltar também, sem clique. Um
        // gesto que começou antes (um arrasto) termina normalmente.
        if std::mem::take(&mut self.input_state(root).refused_press) {
            return;
        }
        self.update_layout();
        if event.kind == PointerKind::Touch && self.finger_up(root, event) {
            return;
        }
        if event.kind == PointerKind::Touch {
            // O último trecho do arrasto, entre o último movimento e o soltar.
            if self.input_state(root).touch.is_some_and(|t| t.id == event.pointer_id && t.panning && t.last != event.position) {
                self.touch_pan(root, event);
            }
            let now = self.time;
            // O puxão de fim da rolagem volta sozinho.
            if self.overscroll_active() {
                self.overscroll_release();
            }
            let state = self.input_state(root);
            match state.touch {
                Some(t) if t.id != event.pointer_id => return,
                Some(t) => {
                    state.touch = None;
                    if t.panning {
                        // Só continua se o dedo ainda estava andando ao soltar.
                        let moving = now - t.time < 0.1;
                        if moving && speed(t.velocity) >= FLING_MIN_SPEED {
                            state.fling = Some(Fling { target: t.target, position: t.start, velocity: t.velocity, time: now });
                            self.request_repaint();
                        }
                        if moving {
                            self.swipe_check(root, t);
                        }
                        // O fim do arrasto, ao tocado e aos pais (a linha deslizada da lista).
                        let mut current = t.target.filter(|&c| self.contains(c));
                        while let Some(id) = current {
                            self.with_component(id, |c, ctx| c.touch_end(ctx));
                            current = self.parent(id);
                        }
                        return;
                    }
                }
                None => {}
            }
        }
        if event.button == PointerButton::Primary && self.drag_drop(root) {
            return;
        }
        let long_pressed = event.button == PointerButton::Primary && self.input_state(root).press.take().is_some_and(|p| p.fired);
        let captured = if event.button == PointerButton::Primary {
            self.input_state(root).captured.take()
        } else {
            None
        };
        let hit = self.enabled_hit(root, event.position);
        let Some(target) = captured.or(hit).filter(|&t| self.contains(t)) else { return };

        if event.button == PointerButton::Primary {
            self.set_state_flag(target, Trigger::Pressed, false);
        }
        // Toque (sem arrasto): o foco vem agora.
        if event.kind == PointerKind::Touch && captured.is_some() {
            if let Some(focusable) = self.focusable_ancestor(target) {
                self.input_state(root).pointer_focus = true;
                self.set_focus(focusable);
            }
        }
        let e = PointerEvent { position: self.local_point(target, event.position), ..event };
        self.fire(target, "on_pointer_up", &EventArgs::Pointer(e));
        self.deliver(target, |c, ctx| c.pointer_up(ctx, &e));

        if event.button == PointerButton::Primary && hit == Some(target) && self.is_effectively_enabled(target) && !long_pressed {
            self.click(target);
        }
        if event.button == PointerButton::Secondary && hit == Some(target) {
            self.open_popup_menu(target, event.position);
        }
        // O controle pode ter mudado de lugar durante o clique.
        self.set_hover(root, self.enabled_hit(root, event.position));
    }

    /// Começa o arrasto pedido em `on_drag_start`: `data` vai para os destinos
    /// (`on_drag_over`, `on_drag_drop`) e `label` acompanha o mouse. Fora de um
    /// `on_drag_start` não faz nada.
    pub fn begin_drag(&mut self, data: impl Into<String>, label: impl Into<String>) {
        self.drag_request = Some((data.into(), label.into()));
    }

    /// A imagem que acompanha o cursor neste arrasto (no lugar da do `drag_image` do
    /// controle). Dentro de `on_drag_start` vale para o arrasto que começa; durante um
    /// arrasto, troca a dele (`None` tira).
    pub fn set_drag_image(&mut self, visual: Option<DragVisual>) {
        let mut applied = false;
        for state in self.input.values_mut() {
            if let Some(d) = state.drag.as_mut() {
                d.visual = visual.clone();
                applied = true;
            }
        }
        if applied {
            self.request_repaint();
        } else {
            self.drag_visual_request = visual;
        }
    }

    /// O arrasto em andamento com imagem: o que acompanha o cursor e a posição do mouse no form.
    pub fn drag_visual(&self, root: ComponentId) -> Option<(DragVisual, PointF)> {
        let d = self.input.get(&root)?.drag.as_ref()?;
        Some((d.visual.clone()?, d.position))
    }

    /// A plataforma mostra a imagem do arrasto numa janela própria (o Windows): o form não a
    /// pinta.
    pub fn set_native_drag_image(&mut self, on: bool) {
        self.native_drag_image = on;
    }

    pub fn native_drag_image(&self) -> bool {
        self.native_drag_image
    }

    /// Dentro de `on_drag_over`: este destino não aceita o que vem (sem destaque; soltar
    /// ali não dispara `on_drag_drop`).
    pub fn reject_drop(&mut self) {
        self.drop_rejected = true;
    }

    /// O que está sendo arrastado no form agora.
    pub fn drag_data(&self, root: ComponentId) -> Option<&str> {
        self.input.get(&root)?.drag.as_ref().map(|d| d.data.as_str())
    }

    /// O arrasto em andamento: (rótulo, posição do mouse no form, destino aceito).
    pub fn drag_view(&self, root: ComponentId) -> Option<(String, PointF, Option<ComponentId>)> {
        let d = self.input.get(&root)?.drag.as_ref()?;
        Some((d.label.clone(), d.position, d.target.filter(|_| d.accepted)))
    }

    /// Começa um arrasto agora, pelo próprio componente `source` no meio de um gesto dele (a
    /// aba puxada para fora da faixa): `position` é o mouse no form. O botão continua
    /// apertado; dali em diante o mouse é do arrasto (destinos, imagem, soltar). `false` se
    /// já havia um.
    /// `label` vazio: nada de texto junto do mouse (a imagem basta).
    pub fn begin_drag_now(&mut self, source: ComponentId, data: impl Into<String>, label: impl Into<String>, visual: Option<DragVisual>, position: PointF) -> bool {
        let root = self.root_of(source);
        if self.input_state(root).drag.is_some() {
            return false;
        }
        let state = self.input_state(root);
        state.press = None;
        if let Some(c) = state.captured.take() {
            self.set_state_flag(c, Trigger::Pressed, false);
        }
        self.hide_hint(root);
        let data = data.into();
        self.input_state(root).drag = Some(Drag {
            source,
            label: label.into(),
            data,
            position,
            target: None,
            accepted: false,
            starting: None,
            over: None,
            dropped: false,
            visual,
            target_position: position,
        });
        let target = self.drop_target_at(root, position, source);
        self.drag_retarget(root, target, position, position);
        true
    }

    /// A plataforma informa onde o mouse está na tela durante um arrasto (pixels físicos) e
    /// a escala da janela de origem: quem solta fora das janelas (a aba que vira janela) sabe
    /// onde.
    pub fn set_screen_pointer(&mut self, pointer: Option<ScreenPointer>) {
        self.screen_pointer = pointer;
    }

    /// O último ponto da tela informado pela plataforma (`None` sem plataforma de janelas).
    pub fn screen_pointer(&self) -> Option<ScreenPointer> {
        self.screen_pointer
    }

    /// O componente recebe o arrasto pelos ganchos `drop_*` (e não por eventos)?
    fn hook_target(&self, id: ComponentId, data: &str) -> bool {
        self.component(id).is_some_and(|c| c.drop_accepts(self, id, data))
    }

    /// O controle arrastável que contém `id` (ele mesmo ou um pai).
    fn draggable_ancestor(&self, root: ComponentId, id: ComponentId) -> Option<ComponentId> {
        let mut cur = Some(id);
        while let Some(c) = cur {
            if self.cb(c).is_some_and(|cb| cb.draggable()) {
                return Some(c);
            }
            if c == root {
                return None;
            }
            cur = self.parent(c);
        }
        None
    }

    /// O destino sob `pos`: o controle que aceita soltar (`accepts_drops` ou `drop_types`; ou
    /// o pai mais próximo que aceite), nunca a própria origem nem algo dentro dela.
    /// Quem recebe pelos ganchos ([`Component::drop_accepts`](crate::Component::drop_accepts))
    /// vale mesmo sendo a origem.
    fn drop_target_at(&self, root: ComponentId, pos: PointF, source: ComponentId) -> Option<ComponentId> {
        let data = self.input.values().find_map(|s| s.drag.as_ref().filter(|d| d.source == source).map(|d| d.data.clone())).unwrap_or_default();
        let mut cur = self.enabled_hit(root, pos);
        while let Some(c) = cur {
            if !data.is_empty() && self.hook_target(c, &data) {
                return Some(c);
            }
            if c == source {
                return None;
            }
            if self.cb(c).is_some_and(|cb| cb.accepts_drops() || !cb.drop_types().is_empty()) {
                // Dentro da origem não vale.
                let mut up = self.parent(c);
                while let Some(p) = up {
                    if p == source {
                        return None;
                    }
                    up = self.parent(p);
                }
                return Some(c);
            }
            if c == root {
                return None;
            }
            cur = self.parent(c);
        }
        None
    }

    /// O mouse andou: começa o arrasto (passou do limite num controle arrastável) ou
    /// acompanha o que já está em andamento. `true` = o movimento foi do arrasto.
    fn drag_move(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        if self.input_state(root).drag.is_none() && !self.start_drag(root, event) {
            return false;
        }
        let Some(drag) = self.input_state(root).drag.clone() else { return false };
        let target = self.drop_target_at(root, event.position, drag.source);
        self.drag_retarget(root, target, event.position, event.position);
        true
    }

    /// Durante um arrasto, o mouse está sobre outra janela do app (a plataforma diz qual e
    /// onde, no form dela): o destino pode ser um controle daquele form. `form` `None`: fora de
    /// todas as janelas.
    pub fn drag_over_form(&mut self, root: ComponentId, form: Option<ComponentId>, pos: PointF) {
        let Some(drag) = self.input.get(&root).and_then(|s| s.drag.clone()) else { return };
        let target = form.filter(|&f| f != root).and_then(|f| self.drop_target_at(f, pos, drag.source));
        let position = drag.position;
        self.drag_retarget(root, target, position, pos);
    }

    /// Há um arrasto em andamento que começou neste form?
    pub fn dragging(&self, root: ComponentId) -> bool {
        self.input.get(&root).is_some_and(|s| s.drag.is_some())
    }

    /// Começa o arrasto: o `drag_data` declarado, o item da lista ou da grade, ou o
    /// `on_drag_start` (pergunta: o handler dá os dados com `begin_drag`, ou responde `data` e
    /// `label`; recusar cancela). `false` se não começou.
    fn start_drag(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        let state = self.input_state(root);
        let Some(press) = state.press.filter(|p| !p.fired && !p.touch) else { return false };
        if (press.position.x - event.position.x).abs() <= DRAG_START && (press.position.y - event.position.y).abs() <= DRAG_START {
            return false;
        }
        let Some(source) = self.draggable_ancestor(root, press.target) else { return false };
        // O declarado (sem handler) e o item que a lista ou a grade mandam sozinhas.
        let declared = self.cb(source).map(|cb| (cb.drag_data().to_string(), cb.drag_label().to_string())).filter(|(d, _)| !d.is_empty());
        let declared = declared.or_else(|| self.component(source).and_then(|c| c.drag_item(self)));
        self.drag_request = None;
        self.drag_visual_request = None;
        let mut pending = None;
        let (data, label) = match self.ask(source, "on_drag_start", &EventArgs::None) {
            crate::question::Asked::Answered(crate::question::Answer::Refused(_)) => return false,
            crate::question::Asked::Answered(answer) => {
                let answered = answer.value("data").and_then(|v| v.to_str().ok()).filter(|d| !d.is_empty()).map(|d| {
                    let label = answer.value("label").and_then(|v| v.to_str().ok()).unwrap_or(d).to_string();
                    (d.to_string(), label)
                });
                match self.drag_request.take().or(answered).or(declared) {
                    Some(d) => d,
                    None => return false,
                }
            }
            crate::question::Asked::Deferred(token) => {
                // O backend dá os dados: o arrasto começa já, com o declarado (ou vazio), e muda
                // (ou é cancelado) com a resposta.
                pending = Some(token);
                let (root2, declared2) = (root, declared.clone());
                self.on_answer(token, move |ui, answer| ui.drag_start_answered(root2, answer, declared2));
                declared.unwrap_or_default()
            }
        };
        let state = self.input_state(root);
        state.press = None;
        if let Some(c) = state.captured.take() {
            self.set_state_flag(c, Trigger::Pressed, false);
        }
        self.hide_hint(root);
        let label = if label.is_empty() { data.clone() } else { label };
        // A imagem: a que o handler pediu ou a do `drag_image` do controle.
        let visual = self.drag_visual_request.take().or_else(|| {
            let image = self.cb(source).and_then(|cb| cb.drag_image())?;
            self.component(image)?.drag_visual(self, source)
        });
        self.input_state(root).drag = Some(Drag { source, data, label, position: event.position, target: None, accepted: false, starting: pending, over: None, dropped: false, visual, target_position: event.position });
        true
    }

    /// A resposta do `on_drag_start` remoto: os dados (ou o cancelamento).
    fn drag_start_answered(&mut self, root: ComponentId, answer: crate::question::Answer, declared: Option<(String, String)>) {
        let Some(drag) = self.input.get_mut(&root).and_then(|s| s.drag.as_mut()) else { return };
        drag.starting = None;
        let data = answer.value("data").and_then(|v| v.to_str().ok()).map(str::to_string).filter(|d| !d.is_empty());
        let label = answer.value("label").and_then(|v| v.to_str().ok()).map(str::to_string);
        match (answer.is_refused(), data.or_else(|| declared.as_ref().map(|d| d.0.clone()))) {
            (false, Some(data)) => {
                drag.label = label.filter(|l| !l.is_empty()).or_else(|| declared.map(|d| d.1).filter(|l| !l.is_empty())).unwrap_or_else(|| data.clone());
                drag.data = data;
                // O destino de agora pergunta de novo, com os dados de verdade.
                let (target, position, dropped, target_position) = (drag.target.take(), drag.position, drag.dropped, drag.target_position);
                drag.accepted = false;
                self.drag_retarget(root, target, position, target_position);
                if dropped {
                    self.drag_drop(root);
                }
            }
            _ => {
                self.cancel_drag(root);
            }
        }
    }

    /// O destino sob o mouse mudou (ou não): `drop_types` decide sem perguntar; senão, o
    /// `on_drag_over` (pergunta: recusar, `reject_drop` ou `accept` falso não aceitam; um
    /// destino remoto fica neutro até a resposta).
    /// `target_position`: o mouse no form do destino (outra janela, às vezes).
    fn drag_retarget(&mut self, root: ComponentId, target: Option<ComponentId>, position: PointF, target_position: PointF) {
        let Some(drag) = self.input_state(root).drag.clone() else { return };
        let mut accepted = drag.accepted;
        let mut over = drag.over;
        // Quem recebe pelos ganchos: avisa que saiu e acompanha o mouse em cima.
        if let Some(old) = drag.target.filter(|&t| Some(t) != target && self.hook_target(t, &drag.data)) {
            self.deliver(old, |c, ctx| c.drop_leave(ctx));
        }
        if let Some(t) = target.filter(|&t| drag.starting.is_none() && self.hook_target(t, &drag.data)) {
            let local = self.local_point(t, target_position);
            let data = drag.data.clone();
            self.deliver(t, |c, ctx| c.drop_hover(ctx, &data, local));
            if let Some(d) = self.input_state(root).drag.as_mut() {
                d.position = position;
                d.target = Some(t);
                d.accepted = true;
                d.over = None;
                d.target_position = target_position;
            }
            self.request_repaint();
            return;
        }
        // Passando por cima do mesmo destino (que aceitou): `on_drag_move`, com o ponto (o
        // `dragover` do HTML; o relatório do Tabernáculo, J16).
        if target == drag.target && drag.accepted && drag.starting.is_none()
            && let Some(t) = target
        {
            let local = self.local_point(t, target_position);
            self.fire(t, "on_drag_move", &EventArgs::Pointer(crate::PointerEvent { position: local, ..Default::default() }));
        }
        if target != drag.target && drag.starting.is_none() {
            // Saiu do destino de antes.
            if let Some(old) = drag.target.filter(|&o| self.contains(o)) {
                self.fire(old, "on_drag_leave", &EventArgs::Text(drag.data.clone()));
            }
            accepted = false;
            over = None;
            if let Some(t) = target {
                let types = self.cb(t).map(|cb| cb.drop_types().to_string()).unwrap_or_default();
                if !types.is_empty() {
                    let kind = drag.data.split(':').next().unwrap_or_default().trim();
                    accepted = types.split([';', ',']).map(str::trim).any(|k| k.eq_ignore_ascii_case(kind));
                } else {
                    self.drop_rejected = false;
                    match self.ask(t, "on_drag_over", &EventArgs::Text(drag.data.clone())) {
                        crate::question::Asked::Answered(answer) => accepted = drag_accepted(&answer) && !std::mem::take(&mut self.drop_rejected),
                        crate::question::Asked::Deferred(token) => {
                            over = Some((t, token));
                            self.on_answer(token, move |ui, answer| ui.drag_over_answered(root, t, token, answer));
                        }
                    }
                }
            }
        }
        if let Some(d) = self.input_state(root).drag.as_mut() {
            d.position = position;
            d.target = target;
            d.accepted = accepted;
            d.over = over;
            d.target_position = target_position;
        }
        self.request_repaint();
    }

    /// A resposta do `on_drag_over` remoto: vale se o mouse ainda está no mesmo destino.
    fn drag_over_answered(&mut self, root: ComponentId, target: ComponentId, token: u64, answer: crate::question::Answer) {
        let Some(drag) = self.input.get_mut(&root).and_then(|s| s.drag.as_mut()) else { return };
        if drag.over != Some((target, token)) {
            return;
        }
        drag.over = None;
        drag.accepted = drag_accepted(&answer);
        let dropped = drag.dropped;
        self.request_repaint();
        if dropped {
            self.drag_drop(root);
        }
    }

    /// Soltou durante um arrasto: entrega ao destino aceito e avisa a origem. Com a resposta
    /// do `on_drag_start` ou do `on_drag_over` ainda pendente, solta quando ela chegar.
    fn drag_drop(&mut self, root: ComponentId) -> bool {
        let Some(drag) = self.input_state(root).drag.as_mut() else { return false };
        if drag.starting.is_some() || drag.over.is_some() {
            drag.dropped = true;
            return true;
        }
        let Some(drag) = self.input_state(root).drag.take() else { return false };
        let dropped = drag.accepted && drag.target.is_some_and(|t| self.contains(t));
        if let Some(t) = drag.target.filter(|_| dropped) {
            if self.hook_target(t, &drag.data) {
                let local = self.local_point(t, drag.target_position);
                self.deliver(t, |c, ctx| c.drop_receive(ctx, &drag.data, local));
            } else {
                // O ponto onde soltou, no destino: `drop_point` durante o `on_drag_drop` (J15).
                self.drop_point = Some(self.local_point(t, drag.target_position));
                self.fire(t, "on_drag_drop", &EventArgs::Text(drag.data.clone()));
            }
        }
        if self.contains(drag.source) {
            let end = if dropped { crate::DragEnd::Dropped } else { crate::DragEnd::Nowhere };
            self.deliver(drag.source, |c, ctx| c.drag_ended(ctx, &drag.data, end));
        }
        if self.contains(drag.source) {
            self.fire(drag.source, "on_drag_end", &EventArgs::Value(dropped.into()));
        }
        self.request_repaint();
        true
    }

    /// Onde o último arrasto foi solto, nas coordenadas do destino (vale no `on_drag_drop`).
    pub fn drop_point(&self) -> Option<PointF> {
        self.drop_point
    }

    /// Cancela o arrasto em andamento (Esc). `true` se havia um.
    pub fn cancel_drag(&mut self, root: ComponentId) -> bool {
        let Some(drag) = self.input_state(root).drag.take() else { return false };
        let state = self.input_state(root);
        state.press = None;
        state.captured = None;
        if let Some(t) = drag.target.filter(|&t| self.hook_target(t, &drag.data)) {
            self.deliver(t, |c, ctx| c.drop_leave(ctx));
        }
        if self.contains(drag.source) {
            self.deliver(drag.source, |c, ctx| c.drag_ended(ctx, &drag.data, crate::DragEnd::Cancelled));
        }
        if self.contains(drag.source) {
            self.fire(drag.source, "on_drag_end", &EventArgs::Value(false.into()));
        }
        self.request_repaint();
        true
    }

    /// Abre o menu de contexto do controle (ou do pai mais próximo que tenha um).
    pub fn open_popup_menu(&mut self, target: ComponentId, at: PointF) -> bool {
        let mut current = Some(target);
        while let Some(c) = current {
            if let Some(menu) = self.cb(c).and_then(|cb| cb.popup_menu()).filter(|&m| self.contains(m)) {
                return self.send_message(menu, &crate::event::PopupRequest { at, source: target });
            }
            current = self.parent(c);
        }
        false
    }

    /// O ponteiro saiu da janela.
    pub fn pointer_leave(&mut self, root: ComponentId) {
        self.hide_hint(root);
        if self.input_state(root).captured.is_none() {
            self.set_hover(root, None);
        }
    }

    /// Cancela a interação em andamento (a janela perdeu o foco, o toque foi cancelado...).
    pub fn pointer_cancel(&mut self, root: ComponentId) {
        if let Some(p) = self.input_state(root).pinch.take() {
            let e = crate::event::GestureEvent { phase: crate::event::GesturePhase::End, scale: 1.0, delta_scale: 1.0, center: p.last_center, ..Default::default() };
            self.gesture_from(p.target, e);
        }
        let state = self.input_state(root);
        state.fingers.clear();
        state.after_pinch = false;
        state.touch = None;
        if let Some(captured) = self.input_state(root).captured.take() {
            self.set_state_flag(captured, Trigger::Pressed, false);
        }
        self.set_hover(root, None);
    }

    /// Roda do mouse / rolagem. Sobe do controle sob o ponteiro para os pais até algum
    /// componente marcar como tratado.
    pub fn wheel(&mut self, root: ComponentId, event: WheelEvent) {
        self.report(&crate::activity::Activity::Wheel { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        if self.busy_blocks_input() {
            return;
        }
        self.hide_hint(root);
        if !self.wheel_handled(root, event) {
            // Ninguém rolou: o form rola, se o conteúdo não cabe (o teclado aberto, a tela
            // pequena).
            self.scroll_form(root, event.delta.y);
        }
    }

    /// A roda só para os controles sob o ponteiro (subindo pelos pais), sem o form rolar
    /// quando ninguém trata: o designer da IDE rola o `ScrollBox` do form desenhado e, sem
    /// ninguém para rolar, move a própria vista. `true` se algum controle tratou.
    pub fn wheel_controls(&mut self, root: ComponentId, event: WheelEvent) -> bool {
        self.wheel_handled(root, event)
    }

    fn wheel_handled(&mut self, root: ComponentId, event: WheelEvent) -> bool {
        self.update_layout();
        let hit = self.enabled_hit(root, event.position);
        self.wheel_from(hit, event)
    }

    /// Entrega a roda a `start` e sobe para os pais até alguém tratar.
    fn wheel_from(&mut self, start: Option<ComponentId>, event: WheelEvent) -> bool {
        let mut current = start;
        while let Some(id) = current {
            let e = WheelEvent { position: self.local_point(id, event.position), ..event };
            self.fire(id, "on_wheel", &EventArgs::Wheel(e));
            if self.deliver(id, |c, ctx| c.wheel(ctx, &e)) {
                // A roda pode ter ligado um timer (o deslize da rolagem).
                self.sync_timer(id);
                return true;
            }
            // `captures_wheel`: a roda fica com ele (o `on_wheel` dele fez o que queria, um
            // zoom), e os contêineres de fora não rolam junto.
            if self.cb(id).is_some_and(|cb| cb.captures_wheel()) {
                return true;
            }
            current = self.parent(id);
        }
        false
    }

    // =====================================================================
    // Pinça e giro (dois dedos)
    // =====================================================================

    /// Um dedo tocou. Com o segundo dedo começa a pinça: o toque do primeiro (rolagem,
    /// clique, toque longo) é cancelado. Devolve `true` se o dedo não segue como toque.
    fn finger_down(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        let state = self.input_state(root);
        state.fingers.retain(|(id, _)| *id != event.pointer_id);
        state.fingers.push((event.pointer_id, event.position));
        if state.pinch.is_some() || state.after_pinch || state.fingers.len() > 2 {
            return true;
        }
        if state.fingers.len() < 2 {
            return false;
        }
        let ((a_id, a), (b_id, b)) = (state.fingers[0], state.fingers[1]);
        let first_target = state.touch.and_then(|t| t.target);
        state.touch = None;
        state.fling = None;
        state.press = None;
        if let Some(c) = self.input_state(root).captured.take() {
            self.set_state_flag(c, Trigger::Pressed, false);
        }
        let (distance, angle, center) = finger_geometry(a, b);
        let Some(target) = first_target.or_else(|| self.enabled_hit(root, center)) else { return true };
        self.input_state(root).pinch =
            Some(Pinch { target, ids: (a_id, b_id), start_distance: distance, start_angle: angle, last_distance: distance, last_angle: angle, last_center: center });
        let e = crate::event::GestureEvent { phase: crate::event::GesturePhase::Begin, center, scale: 1.0, delta_scale: 1.0, ..Default::default() };
        self.gesture_from(target, e);
        true
    }

    /// Um dedo andou. Durante a pinça, gera o gesto; devolve `true` se o movimento não
    /// segue como toque.
    fn finger_move(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        let state = self.input_state(root);
        if let Some(f) = state.fingers.iter_mut().find(|(id, _)| *id == event.pointer_id) {
            f.1 = event.position;
        }
        if state.after_pinch {
            return true;
        }
        let Some(mut p) = state.pinch else { return false };
        let pos = |id: u64| state.fingers.iter().find(|(i, _)| *i == id).map(|(_, p)| *p);
        let (Some(a), Some(b)) = (pos(p.ids.0), pos(p.ids.1)) else { return true };
        let (distance, angle, center) = finger_geometry(a, b);
        let wrap = |r: f32| (r + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        let e = crate::event::GestureEvent {
            phase: crate::event::GesturePhase::Update,
            center,
            scale: distance / p.start_distance,
            delta_scale: distance / p.last_distance,
            rotation: wrap(angle - p.start_angle),
            delta_rotation: wrap(angle - p.last_angle),
            pan: center - p.last_center,
        };
        p.last_distance = distance;
        p.last_angle = angle;
        p.last_center = center;
        state.pinch = Some(p);
        self.gesture_from(p.target, e);
        true
    }

    /// Um dedo saiu. Acaba a pinça; o dedo que sobrar não vira toque até sair também.
    fn finger_up(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        let state = self.input_state(root);
        state.fingers.retain(|(id, _)| *id != event.pointer_id);
        if let Some(p) = state.pinch.take() {
            state.after_pinch = !state.fingers.is_empty();
            let scale = p.last_distance / p.start_distance;
            let e = crate::event::GestureEvent { phase: crate::event::GesturePhase::End, center: p.last_center, scale, delta_scale: 1.0, ..Default::default() };
            self.gesture_from(p.target, e);
            return true;
        }
        if state.after_pinch {
            state.after_pinch = !state.fingers.is_empty();
            return true;
        }
        false
    }

    /// Gesto já reconhecido pelo sistema (pinça e giro do touchpad no macOS): vai ao
    /// controle sob `event.center` (coordenadas do form) e sobe para os pais.
    pub fn gesture(&mut self, root: ComponentId, event: crate::event::GestureEvent) {
        if self.input_blocked(root) {
            return;
        }
        self.update_layout();
        if let Some(target) = self.enabled_hit(root, event.center) {
            self.gesture_from(target, event);
        }
    }

    /// Entrega o gesto ao controle e sobe para os pais até alguém tratar.
    fn gesture_from(&mut self, start: ComponentId, event: crate::event::GestureEvent) -> bool {
        self.request_repaint();
        let mut current = Some(start);
        while let Some(id) = current {
            let e = crate::event::GestureEvent { center: self.local_point(id, event.center), ..event };
            self.fire(id, "on_gesture", &EventArgs::Gesture(e));
            if self.deliver(id, |c, ctx| c.gesture(ctx, &e)) {
                return true;
            }
            current = self.parent(id);
        }
        false
    }

    // =====================================================================
    // Deslizar (`on_swipe`)
    // =====================================================================

    /// O dedo saiu de um arrasto ainda andando: rápido e longo o bastante, sem ter rolado nada
    /// no eixo em que mais andou, é um deslizar. Começando na borda do form, vai primeiro ao
    /// `edge_swipe` dos componentes do form; senão, ao controle tocado, subindo.
    fn swipe_check(&mut self, root: ComponentId, t: Touch) {
        let distance = t.last - t.start;
        let horizontal = distance.x.abs() >= distance.y.abs();
        let (along, scrolled, velocity) = if horizontal { (distance.x, t.scrolled.x, t.velocity.x) } else { (distance.y, t.scrolled.y, t.velocity.y) };
        if along.abs() < SWIPE_DISTANCE || velocity.abs() < SWIPE_SPEED || scrolled > SWIPE_SCROLLED || along.signum() != velocity.signum() {
            return;
        }
        let direction = match (horizontal, along > 0.0) {
            (true, true) => crate::event::SwipeDirection::Right,
            (true, false) => crate::event::SwipeDirection::Left,
            (false, true) => crate::event::SwipeDirection::Down,
            (false, false) => crate::event::SwipeDirection::Up,
        };
        let event = crate::event::SwipeEvent { direction, start: t.start, distance, velocity: t.velocity };
        // Da borda do form, primeiro a gaveta (como no Android: puxar da borda abre, mesmo com
        // um carrossel embaixo); ninguém tratando, segue como um deslizar comum.
        let form = self.cb(root).map(|cb| cb.bounds().size()).unwrap_or_default();
        let at_edge = t.start.x <= EDGE_SWIPE || t.start.x >= form.width - EDGE_SWIPE || t.start.y <= EDGE_SWIPE || t.start.y >= form.height - EDGE_SWIPE;
        if at_edge && self.edge_swipe(root, event) {
            return;
        }
        let target = t.target.filter(|&c| self.is_effectively_enabled(c));
        self.swipe_from(target, event);
    }

    /// O deslizar da borda vai ao `edge_swipe` dos componentes do form (só um trata).
    fn edge_swipe(&mut self, root: ComponentId, event: crate::event::SwipeEvent) -> bool {
        // De cima para baixo na árvore (raro: só no deslizar da borda). Um escondido também
        // ouve (a gaveta fechada fica escondida), mas não os filhos dele.
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            // Só habilitado (o escondido conta: quem esconde é ele mesmo, e os pais são visíveis,
            // senão o percurso nem chegava aqui).
            if !self.cb(id).is_some_and(|cb| cb.enabled()) {
                continue;
            }
            if id != root && self.with_component(id, |c, ctx| c.edge_swipe(ctx, &event)).unwrap_or(false) {
                self.request_repaint();
                return true;
            }
            if self.cb(id).is_some_and(|cb| cb.visible()) {
                stack.extend(self.children(id).iter().rev().copied());
            }
        }
        false
    }

    /// Entrega o deslizar ao controle e sobe para os pais até alguém tratar.
    fn swipe_from(&mut self, start: Option<ComponentId>, event: crate::event::SwipeEvent) -> bool {
        let mut current = start;
        while let Some(id) = current {
            let e = crate::event::SwipeEvent { start: self.local_point(id, event.start), ..event };
            self.fire(id, "on_swipe", &EventArgs::Swipe(e));
            if self.deliver(id, |c, ctx| c.swipe(ctx, &e)) {
                self.request_repaint();
                return true;
            }
            current = self.parent(id);
        }
        false
    }

    // =====================================================================
    // Rolagem pelo toque
    // =====================================================================

    /// O controle (e os pais) aceitam que o arrasto do dedo vire rolagem.
    fn pans_on_touch(&self, id: ComponentId) -> bool {
        let mut current = Some(id);
        while let Some(c) = current {
            if self.cb(c).is_some_and(|cb| !cb.pan_on_touch()) {
                return false;
            }
            current = self.parent(c);
        }
        true
    }

    /// Movimento de um dedo. Devolve `true` se virou rolagem (o movimento não vai para o
    /// controle).
    fn touch_pan(&mut self, root: ComponentId, event: PointerEvent) -> bool {
        let now = self.time;
        let Some(mut t) = self.input_state(root).touch else { return false };
        if t.id != event.pointer_id {
            return true;
        }
        if !t.panning {
            let moved = event.position - t.start;
            if moved.x.abs() < TOUCH_SLOP && moved.y.abs() < TOUCH_SLOP {
                return false;
            }
            let captured = self.input_state(root).captured;
            if captured.is_some_and(|c| !self.pans_on_touch(c)) {
                // Este controle quer o arrasto (Splitter...): o toque segue normal.
                self.input_state(root).touch = None;
                return false;
            }
            // Vira rolagem: o controle tocado solta o toque e não recebe clique.
            if let Some(c) = self.input_state(root).captured.take() {
                self.set_state_flag(c, Trigger::Pressed, false);
            }
            self.set_hover(root, None);
            self.input_state(root).press = None;
            t.panning = true;
        }
        let delta = event.position - t.last;
        let dt = (now - t.time).max(0.004) as f32;
        let v = PointF::new(delta.x / dt, delta.y / dt);
        t.velocity = PointF::new(t.velocity.x * 0.5 + v.x * 0.5, t.velocity.y * 0.5 + v.y * 0.5);
        t.last = event.position;
        t.time = now;
        self.input_state(root).touch = Some(t);
        self.update_layout();
        let target = t.target.filter(|&c| self.is_effectively_enabled(c));
        // O fim da rolagem (`overscroll.rs`): o puxão aberto fecha antes de rolar de novo; o
        // que ninguém rola (o conteúdo na ponta) abre o puxão do contêiner.
        let host = self.overscroll_host(target);
        let delta = match host {
            Some(h) => self.overscroll_take_back(h, delta),
            None => delta,
        };
        let mut scrolled = false;
        if delta != PointF::ZERO {
            scrolled = self.wheel_from(target, WheelEvent { position: t.start, delta, touch: true, ..Default::default() });
            if !scrolled {
                match host {
                    Some(h) => self.overscroll_drag(h, delta),
                    None => scrolled = self.scroll_form(root, delta.y),
                }
            }
        }
        if scrolled {
            if let Some(t) = self.input_state(root).touch.as_mut() {
                t.scrolled = PointF::new(t.scrolled.x + delta.x.abs(), t.scrolled.y + delta.y.abs());
            }
        }
        true
    }

    /// Há rolagem por inércia em andamento.
    pub(crate) fn is_flinging(&self) -> bool {
        self.input.values().any(|s| s.fling.is_some())
    }

    /// Avança a inércia (chamado por [`Ui::tick`]). Para quando a velocidade cai ou quando
    /// o conteúdo chega ao fim (ninguém trata a roda).
    pub(crate) fn tick_flings(&mut self) {
        let now = self.time;
        let roots: Vec<ComponentId> = self.input.iter().filter(|(_, s)| s.fling.is_some()).map(|(&r, _)| r).collect();
        for root in roots {
            let Some(mut f) = self.input_state(root).fling else { continue };
            // Limita o passo: depois de uma pausa longa a inércia não dá um salto.
            let dt = (now - f.time).clamp(0.0, 0.05) as f32;
            if dt <= 0.0 {
                continue;
            }
            let delta = PointF::new(f.velocity.x * dt, f.velocity.y * dt);
            let keep = (-FLING_DECAY * dt).exp();
            f.velocity = PointF::new(f.velocity.x * keep, f.velocity.y * keep);
            f.time = now;
            self.input_state(root).fling = Some(f);
            self.update_layout();
            let target = f.target.filter(|&c| self.is_effectively_enabled(c));
            let moved = self.wheel_from(target, WheelEvent { position: f.position, delta, touch: true, ..Default::default() })
                || self.scroll_form(root, delta.y);
            // "Ninguém rolou" só conta como fim se o passo era de pelo menos um ponto.
            if (!moved && speed(delta) >= 1.0) || speed(f.velocity) < FLING_MIN_SPEED {
                self.input_state(root).fling = None;
                // Bateu na ponta com velocidade: o efeito de fim da rolagem recebe o impulso.
                if !moved && speed(f.velocity) >= FLING_MIN_SPEED {
                    if let Some(host) = self.overscroll_host(target) {
                        self.overscroll_absorb(host, f.velocity);
                    }
                }
            }
        }
    }

    // =====================================================================
    // Foco e teclado
    // =====================================================================

    fn can_take_focus(&self, id: ComponentId) -> bool {
        self.cb(id).is_some_and(|cb| cb.can_focus()) && self.is_effectively_enabled(id)
    }

    fn focusable_ancestor(&self, id: ComponentId) -> Option<ComponentId> {
        let mut current = Some(id);
        while let Some(c) = current {
            if self.can_take_focus(c) {
                return Some(c);
            }
            current = self.parent(c);
        }
        None
    }

    /// Dá o foco ao controle (como `SetFocus`). Devolve `false` se ele não pode receber
    /// foco (`can_focus` desligado, invisível ou desabilitado).
    pub fn set_focus(&mut self, id: ComponentId) -> bool {
        if !self.can_take_focus(id) {
            return false;
        }
        let root = self.root_of(id);
        let old = self.input_state(root).focused;
        if old == Some(id) {
            return true;
        }
        self.input_state(root).focused = Some(id);
        if let Some(old) = old.filter(|&o| self.contains(o)) {
            self.set_state_flag(old, Trigger::Focused, false);
            self.fire(old, "on_exit", &EventArgs::None);
            self.deliver(old, |c, ctx| c.focus_exit(ctx));
        }
        self.set_state_flag(id, Trigger::Focused, true);
        self.fire(id, "on_enter", &EventArgs::None);
        self.deliver(id, |c, ctx| c.focus_enter(ctx));
        true
    }

    /// Tira o foco de qualquer controle do form.
    pub fn clear_focus(&mut self, root: ComponentId) {
        if let Some(old) = self.input_state(root).focused.take() {
            self.set_state_flag(old, Trigger::Focused, false);
            self.fire(old, "on_exit", &EventArgs::None);
            self.deliver(old, |c, ctx| c.focus_exit(ctx));
        }
    }

    /// Move o foco para o próximo (ou anterior) controle na ordem da árvore, como o Tab.
    pub fn focus_next(&mut self, root: ComponentId, forward: bool) -> bool {
        let order = self.tab_order(root);
        if order.is_empty() {
            return false;
        }
        let current = self.focused(root).and_then(|f| order.iter().position(|&c| c == f));
        let next = match (current, forward) {
            (None, true) => 0,
            (None, false) => order.len() - 1,
            (Some(i), true) => (i + 1) % order.len(),
            (Some(i), false) => (i + order.len() - 1) % order.len(),
        };
        self.set_focus(order[next])
    }

    /// A ordem do Tab no form: a lista `tab_order` do form (nomes separados por `;`, de
    /// qualquer nível) e depois os que não estão nela, na ordem da árvore. Só entra quem
    /// pode receber o foco agora (visível, habilitado, `tab_stop`).
    pub fn tab_order(&self, root: ComponentId) -> Vec<ComponentId> {
        let mut natural = Vec::new();
        self.collect_tab_order(root, &mut natural);
        let custom = match self.get_prop(root, "tab_order") {
            Ok(crate::prop::PropValue::String(s)) if !s.trim().is_empty() => s,
            _ => return natural,
        };
        let mut order: Vec<ComponentId> = custom
            .split(';')
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .filter_map(|n| self.find_in(root, n))
            .filter(|id| natural.contains(id))
            .collect();
        order.dedup();
        let rest: Vec<ComponentId> = natural.into_iter().filter(|id| !order.contains(id)).collect();
        order.extend(rest);
        order
    }

    fn collect_tab_order(&self, id: ComponentId, out: &mut Vec<ComponentId>) {
        let Some(cb) = self.cb(id) else { return };
        if !cb.visible() || !cb.enabled() {
            return;
        }
        if cb.can_focus() && cb.tab_stop() {
            out.push(id);
        }
        for &c in self.children(id) {
            self.collect_tab_order(c, out);
        }
    }

    /// Tecla pressionada: vai para o controle com foco (ou o form) e sobe para os pais
    /// até alguém tratar. Tab / Shift+Tab sem tratamento movem o foco.
    pub fn key_down(&mut self, root: ComponentId, event: KeyEvent) {
        self.report(&crate::activity::Activity::KeyDown { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        // Uma tecla anterior espera a resposta do backend: esta vai para a fila, em ordem.
        if self.hold_input(root, crate::keyflow::Held::Down(event)) {
            return;
        }
        if event.key == Key::Escape && self.cancel_drag(root) {
            return;
        }
        // Mexeu no teclado: o anel de foco volta a aparecer (fora as teclas de modificador).
        if !matches!(event.key, Key::Shift | Key::Control | Key::Alt | Key::Meta) {
            self.input_state(root).pointer_focus = false;
        }
        // As teclas que o form põe no lugar do Tab (`tab_keys`, `back_tab_keys`): o Enter que
        // pula de campo, como nos sistemas de cadastro. O controle que usa a tecla (o Enter de
        // um Memo) fica com ela.
        if let Some(forward) = self.tab_key(root, &event) {
            self.focus_next(root, forward);
            return;
        }
        // Alt+← e Alt+→: o mesmo que os botões extras do mouse, antes do controle com foco (o
        // editor de texto andaria com o cursor), como nos navegadores e no VS Code. Sem
        // `on_navigate` tratando, a tecla segue o caminho normal.
        if event.modifiers.alt
            && !event.modifiers.ctrl
            && !event.modifiers.shift
            && matches!(event.key, Key::Left | Key::Right)
            && self.navigate(root, event.key == Key::Right)
        {
            return;
        }
        self.key_down_flow(root, event);
    }

    /// Voltar (`forward` falso) ou avançar na navegação da janela `root`: o `on_navigate`,
    /// com o texto `"back"` ou `"forward"`, do controle com foco ou do primeiro pai dele que
    /// trata (um form embutido, como o editor de código dentro da IDE, antes da janela). É o
    /// que os botões extras do mouse, Alt+← e Alt+→ fazem; o app guarda o próprio histórico
    /// (as páginas de uma documentação, os lugares por onde o editor passou). Devolve se
    /// alguém tratou.
    pub fn navigate(&mut self, root: ComponentId, forward: bool) -> bool {
        let args = EventArgs::Text(if forward { "forward" } else { "back" }.into());
        let mut at = self.focused(root).or(Some(root));
        while let Some(id) = at {
            if self.fire(id, "on_navigate", &args) {
                return true;
            }
            if id == root {
                break;
            }
            at = self.parent(id);
        }
        false
    }

    /// O componente trata a tecla (o `key_down` dele)?
    pub(crate) fn deliver_key_down(&mut self, id: ComponentId, event: KeyEvent) -> bool {
        self.deliver(id, |c, ctx| c.key_down(ctx, &event))
    }

    /// Ninguém tratou a tecla: o Tab, os atalhos e as letras com `&`.
    pub(crate) fn key_down_unhandled(&mut self, root: ComponentId, event: KeyEvent) {
        // O Enter e o Esc que ninguém usou: o botão padrão e o de cancelar do form (o relatório
        // do Tabernáculo, J23).
        let plain = !event.modifiers.ctrl && !event.modifiers.alt && !event.modifiers.shift && !event.modifiers.meta;
        if plain && !event.repeat && matches!(event.key, Key::Enter | Key::Escape) {
            let prop = if event.key == Key::Enter { "default_button" } else { "cancel_button" };
            if let Some(button) = self.flagged_button(root, prop) {
                self.click(button);
                return;
            }
        }
        if event.key == Key::Tab && !event.modifiers.ctrl && !event.modifiers.alt {
            self.focus_next(root, !event.modifiers.shift);
        } else if !event.repeat && !self.dispatch_shortcut(root, event) {
            // Alt+letra: o controle com essa letra marcada com "&" na legenda.
            if let (true, false, Key::Char(c)) = (event.modifiers.alt, event.modifiers.ctrl, event.key) {
                if let Some(id) = self.mnemonic_target(root, c) {
                    self.activate_mnemonic(id);
                }
            }
        }
    }

    /// O primeiro controle à vista e habilitado do form com a propriedade `prop` ligada (o
    /// botão padrão ou o de cancelar).
    fn flagged_button(&self, root: ComponentId, prop: &str) -> Option<ComponentId> {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if id != root && self.cb(id).is_some_and(|cb| !cb.visible() || !cb.enabled()) {
                continue;
            }
            if id != root && matches!(self.get_prop(id, prop), Ok(crate::prop::PropValue::Bool(true))) {
                return Some(id);
            }
            stack.extend(self.children(id).iter().rev().copied());
        }
        None
    }

    /// A tecla é uma das que o form usa como Tab (`Some(true)`) ou Shift+Tab (`Some(false)`)?
    fn tab_key(&self, root: ComponentId, event: &KeyEvent) -> Option<bool> {
        let list = |prop: &str| match self.get_prop(root, prop) {
            Ok(crate::prop::PropValue::String(s)) => crate::keys::parse_shortcut_list(&s),
            _ => Vec::new(),
        };
        let forward = if list("tab_keys").into_iter().any(|s| crate::keys::shortcut_matches(s, event)) {
            true
        } else if list("back_tab_keys").into_iter().any(|s| crate::keys::shortcut_matches(s, event)) {
            false
        } else {
            return None;
        };
        let captured = self.focused(root).and_then(|f| self.component(f)).is_some_and(|c| c.captures_key(event));
        (!captured).then_some(forward)
    }

    /// O controle do form com a letra `c` marcada com `&` na legenda (visível e habilitado,
    /// ele e os pais).
    pub fn mnemonic_target(&self, root: ComponentId, c: char) -> Option<ComponentId> {
        let c = c.to_lowercase().next()?;
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if self.cb(id).is_some_and(|cb| !cb.visible() || !cb.enabled()) && id != root {
                continue;
            }
            if id != root && self.component(id).and_then(|comp| comp.mnemonic_text()).and_then(crate::keys::mnemonic) == Some(c) {
                return Some(id);
            }
            stack.extend(self.children(id).iter().rev());
        }
        None
    }

    /// As letras de `&` repetidas no form: cada letra com os controles que a usam (para a
    /// IDE avisar).
    pub fn mnemonic_conflicts(&self, root: ComponentId) -> Vec<(char, Vec<ComponentId>)> {
        let mut seen: Vec<(char, Vec<ComponentId>)> = Vec::new();
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if let Some(m) = self.component(id).and_then(|c| c.mnemonic_text()).and_then(crate::keys::mnemonic) {
                match seen.iter_mut().find(|(c, _)| *c == m) {
                    Some((_, ids)) => ids.push(id),
                    None => seen.push((m, vec![id])),
                }
            }
            stack.extend(self.children(id).iter().rev());
        }
        seen.retain(|(_, ids)| ids.len() > 1);
        seen
    }

    /// Alt+letra num controle: o rótulo leva o foco ao `focus_control` dele;
    /// o que recebe foco e não é clicável (um grupo) passa ao
    /// primeiro filho; o resto é clicado.
    fn activate_mnemonic(&mut self, id: ComponentId) {
        if let Ok(crate::prop::PropValue::Component(Some(target))) = self.get_prop(id, "focus_control") {
            self.set_focus(target);
            return;
        }
        if self.component(id).is_some_and(|c| c.mnemonic_focuses_child()) {
            let root = self.root_of(id);
            let order = self.tab_order(root);
            if let Some(&first) = order.iter().find(|&&c| self.is_ancestor(id, c)) {
                self.set_focus(first);
            }
            return;
        }
        if self.cb(id).is_some_and(|cb| cb.can_focus()) {
            self.set_focus(id);
        }
        self.click(id);
    }

    /// `ancestor` está acima de `id` na árvore?
    fn is_ancestor(&self, ancestor: ComponentId, id: ComponentId) -> bool {
        let mut current = self.parent(id);
        while let Some(p) = current {
            if p == ancestor {
                return true;
            }
            current = self.parent(p);
        }
        false
    }

    /// Os caracteres que o componente diz aceitar ou bloquear (`allowed_chars`,
    /// `blocked_chars`, `only_allowed`, `chars_override`). `None` se ele não diz nada.
    pub fn char_filter_of(&self, id: ComponentId) -> Option<crate::keys::CharFilter> {
        use crate::prop::PropValue;
        let text = |p: &str| match self.get_prop(id, p) {
            Ok(PropValue::String(s)) => s,
            _ => String::new(),
        };
        let flag = |p: &str| matches!(self.get_prop(id, p), Ok(PropValue::Bool(true)));
        let filter = crate::keys::CharFilter {
            allowed: text("allowed_chars"),
            blocked: text("blocked_chars"),
            only_allowed: flag("only_allowed"),
            overrides: flag("chars_override"),
            map: text("char_map"),
            case: match self.get_prop(id, "char_case") {
                Ok(PropValue::Enum(e)) => <crate::keys::CharCase as crate::prop::PropEnum>::from_name(&e).unwrap_or_default(),
                _ => Default::default(),
            },
        };
        (!filter.is_empty()).then_some(filter)
    }

    /// O caractere pode ser digitado em `id`? Decide o campo e, acima dele, os contêineres
    /// até o form: o filho primeiro, a não ser que um pai tenha `chars_override`. `own` é
    /// o filtro do próprio campo quando quem pergunta é ele (o componente está ocupado e
    /// não responde às propriedades).
    pub fn accepts_char(&self, id: ComponentId, own: Option<&crate::keys::CharFilter>, ch: char) -> bool {
        let mut chain = Vec::new();
        match own {
            Some(f) => chain.push(f.clone()),
            None => chain.extend(self.char_filter_of(id)),
        }
        let mut current = self.parent(id);
        while let Some(p) = current {
            chain.extend(self.char_filter_of(p));
            current = self.parent(p);
        }
        crate::keys::char_allowed(&chain, ch)
    }

    /// O texto como o campo `id` recebe (o digitado e o colado): o `char_map` (do campo ou dos
    /// contêineres), o `char_case` do campo e só os caracteres que ele aceita.
    pub fn filter_text(&self, id: ComponentId, own: Option<&crate::keys::CharFilter>, text: &str) -> String {
        let mut chain = Vec::new();
        match own {
            Some(f) => chain.push(f.clone()),
            None => chain.extend(self.char_filter_of(id)),
        }
        let case = chain.first().map(|f| f.case).unwrap_or_default();
        let mut current = self.parent(id);
        while let Some(p) = current {
            chain.extend(self.char_filter_of(p));
            current = self.parent(p);
        }
        let mapped: String = text.chars().map(|c| crate::keys::map_char(&chain, c)).collect();
        case.apply(&mapped).chars().filter(|&c| crate::keys::char_allowed(&chain, c)).collect()
    }

    /// Oferece a tecla aos componentes do form (itens de menu com atalho), até um tratar. Um
    /// form embutido (outro form dentro deste, como o editor de código dentro da IDE) onde
    /// está o foco tem a vez primeiro: o Ctrl+S dele vale mais que o do form de fora.
    fn dispatch_shortcut(&mut self, root: ComponentId, event: KeyEvent) -> bool {
        let menus = self.active_popup_menus(root);
        let mut current = self.focused(root).and_then(|f| self.parent(f));
        while let Some(p) = current.filter(|&p| p != root) {
            if self.class_name(p) == Some("Form") && self.dispatch_in(p, event, &menus) {
                return true;
            }
            current = self.parent(p);
        }
        self.dispatch_in(root, event, &menus)
    }

    /// Os menus de contexto cujos atalhos valem agora: os do `popup_menu` do controle com o
    /// foco e dos pais dele, até o form (antes valiam todos, até
    /// com o menu de outro controle; o relatório do Tabernáculo, J01).
    fn active_popup_menus(&self, root: ComponentId) -> Vec<ComponentId> {
        let mut out = Vec::new();
        let mut current = self.focused(root).or(Some(root));
        while let Some(id) = current {
            if let Ok(crate::PropValue::Component(Some(menu))) = self.get_prop(id, "popup_menu") {
                out.push(menu);
            }
            current = self.parent(id);
        }
        out
    }

    fn dispatch_in(&mut self, root: ComponentId, event: KeyEvent, menus: &[ComponentId]) -> bool {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if self.cb(id).is_some_and(|cb| !cb.visible() || !cb.enabled()) && id != root {
                continue;
            }
            if self.class_name(id) == Some("PopupMenu") && !menus.contains(&id) {
                continue;
            }
            if self.deliver(id, |c, ctx| c.shortcut(ctx, &event)) {
                return true;
            }
            stack.extend(self.children(id).iter().rev());
        }
        false
    }

    pub fn key_up(&mut self, root: ComponentId, event: KeyEvent) {
        self.report(&crate::activity::Activity::KeyUp { form: root, event });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        if self.hold_input(root, crate::keyflow::Held::Up(event)) {
            return;
        }
        self.bubble_key(root, |c, ctx| c.key_up(ctx, &event), "on_key_up", event);
    }

    fn bubble_key(
        &mut self,
        root: ComponentId,
        f: impl Fn(&mut dyn Component, &mut EventCtx<'_>),
        event_name: &str,
        event: KeyEvent,
    ) -> bool {
        let mut current = self.focused(root).or(Some(root));
        while let Some(id) = current {
            self.fire(id, event_name, &EventArgs::Key(event));
            let handled = self.deliver(id, &f);
            if handled {
                return true;
            }
            current = self.parent(id);
        }
        false
    }

    /// O controle com foco edita texto: no celular a plataforma mostra o teclado virtual
    /// enquanto isto for verdade.
    pub fn wants_text_input(&self, root: ComponentId) -> bool {
        self.focused(root).and_then(|f| self.component(f)).is_some_and(|c| c.accepts_text())
    }

    /// Texto digitado (já com acentos/IME resolvidos), entregue ao controle com foco.
    pub fn text_input(&mut self, root: ComponentId, text: &str) {
        self.report(&crate::activity::Activity::Text { form: root, text });
        if self.input_blocked(root) {
            return; // há um form modal aberto por cima
        }
        if self.hold_input(root, crate::keyflow::Held::Text(text.to_string())) {
            return;
        }
        if let Some(id) = self.focused(root) {
            // `char_map`, `char_case` e os caracteres que o campo (ou um pai dele) não aceita.
            let text = self.filter_text(id, None, text);
            if text.is_empty() {
                return;
            }
            self.press_chars(root, id, text.chars().collect());
        }
    }
}

/// Desfaz o giro do controle (em volta do centro) num ponto já relativo a ele: a pintura
/// gira com `rotation`, e o clique tem de cair no mesmo lugar.
/// Do retângulo do controle (já sem o lugar e o giro) para o espaço de dentro dele (`scale`).
fn unscale(cb: &crate::component::ControlBase, local: PointF) -> PointF {
    let s = cb.scale();
    if s == 1.0 { local } else { PointF::new(local.x / s, local.y / s) }
}

fn unrotate(cb: &crate::component::ControlBase, local: PointF) -> PointF {
    let degrees = cb.rotation();
    if degrees == 0.0 {
        return local;
    }
    let b = cb.bounds();
    let (cx, cy) = (b.width / 2.0, b.height / 2.0);
    let (s, c) = (-degrees.to_radians()).sin_cos();
    let (dx, dy) = (local.x - cx, local.y - cy);
    PointF::new(cx + dx * c - dy * s, cy + dx * s + dy * c)
}

/// O `on_drag_over` aceitou? (não recusou e não respondeu `accept` falso).
fn drag_accepted(answer: &crate::question::Answer) -> bool {
    !answer.is_refused() && answer.flag("accept", true)
}
