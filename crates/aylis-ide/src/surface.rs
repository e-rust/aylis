//! A superfície de design: pinta o form do documento dentro da IDE e trata seleção (um
//! ou vários, por clique ou por área), arrasto com guias, alças de redimensionamento, zoom,
//! colocação de componentes da paleta e a bandeja de não visuais.

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerButton, PointerEvent, RectF, Stroke, TextAlign, WheelEvent,
};

use crate::document::{Document, Guide, Handle};
use crate::state::Shared;

const TRAY: f32 = 52.0;
const CHIP: f32 = 160.0;
const HANDLE: f32 = 7.0;
const MARGIN: f32 = 28.0;

#[derive(Clone, Debug, PartialEq)]
enum Drag {
    None,
    Move { items: Vec<(ComponentId, RectF)>, from: PointF, began: bool },
    Resize { id: ComponentId, handle: Handle, start: RectF, from: PointF, began: bool },
    /// Seleção por área: do ponto `from` (coordenadas do form) até o atual.
    Area { from: PointF, to: PointF },
    /// Um ponto de arraste do próprio controle (o separador das colunas de uma grade).
    Grip { id: ComponentId, grip: usize, from: PointF, began: bool },
    /// Ligando dois componentes com um cano ou fio (`class`, escolhido na paleta): apertado
    /// em `from` no ponto `start`, o mouse agora em `to` (coordenadas do form).
    Connect { class: String, from: ComponentId, start: PointF, to: PointF },
}

/// As classes que se ligam arrastando de um componente a outro.
fn is_link_class(class: &str) -> bool {
    matches!(aylis_core::split_variant(class.split('@').next().unwrap_or(class)).0, "Pipe" | "Wire")
}

/// Superfície de design. Dispara `on_change` quando o documento ou a seleção mudam (a
/// IDE atualiza a estrutura e o Object Inspector), `on_dbl_click` no duplo clique e
/// `on_context_menu` (com a posição) no clique direito.
pub struct DesignSurface {
    pub base: ControlBase,
    state: Shared,
    /// Onde o form aparece dentro da superfície.
    origin: PointF,
    drag: Drag,
    /// O chip da bandeja cujo X está sob o ponteiro (destacado em vermelho).
    close_hover: Option<usize>,
}

impl DesignSurface {
    /// A última aba foi fechada: a superfície mostra só o aviso e não mexe no form vazio.
    fn no_form(&self) -> bool {
        self.state.borrow().no_form
    }

    /// Onde o canto do form está na superfície (a rolagem do designer).
    pub fn origin(&self) -> PointF {
        self.origin
    }

    pub fn new(state: Shared) -> Self {
        let mut base = ControlBase::new(400.0, 300.0);
        base.set_can_focus(true).set_clip_children(true).set_pan_on_touch(false);
        Self { base, state, origin: PointF::new(MARGIN, MARGIN), drag: Drag::None, close_hover: None }
    }

    fn zoom(&self) -> f32 {
        self.state.borrow().zoom
    }

    /// A bandeja dos não visuais: embaixo do form; num DataModule, a superfície inteira.
    fn tray_rect(&self) -> Option<RectF> {
        let b = self.base.bounds();
        let st = self.state.borrow();
        if st.doc.is_data_module() {
            return Some(RectF::new(0.0, 0.0, b.width, b.height));
        }
        let has = !st.doc.non_visual().is_empty();
        has.then(|| RectF::new(0.0, b.height - TRAY, b.width, TRAY))
    }

    /// O botão de girar o aparelho, no canto de cima à direita (só com um aparelho que gira).
    fn rotate_rect(&self, device: Option<(&'static aylis_core::devices::Device, bool)>) -> Option<RectF> {
        device.filter(|(d, _)| d.rotates())?;
        Some(RectF::new(self.base.bounds().width - 112.0, 26.0, 100.0, 28.0))
    }

    /// O lugar do componente `index` na bandeja (quebra linha quando não cabe).
    /// O X de excluir do chip (à direita, centrado na altura).
    fn close_rect(chip: RectF) -> RectF {
        RectF::new(chip.right() - 26.0, chip.y + (chip.height - 18.0) / 2.0, 18.0, 18.0)
    }

    /// O chip cujo X está no ponto `p` (coordenadas da superfície).
    fn close_at(&self, p: PointF) -> Option<usize> {
        let tray = self.tray_rect().filter(|t| t.contains(p))?;
        let count = self.state.borrow().doc.non_visual().len();
        (0..count).find(|&i| Self::close_rect(Self::chip_rect(tray, i)).inflate(3.0).contains(p))
    }

    fn chip_rect(tray: RectF, index: usize) -> RectF {
        let columns = (((tray.width - 12.0) / (CHIP + 8.0)).floor() as usize).max(1);
        let (row, column) = (index / columns, index % columns);
        RectF::new(12.0 + column as f32 * (CHIP + 8.0), tray.y + 10.0 + row as f32 * (TRAY - 12.0), CHIP, TRAY - 20.0)
    }

    /// Ponto da superfície → coordenadas do form.
    fn to_doc(&self, p: PointF) -> PointF {
        let z = self.zoom();
        PointF::new((p.x - self.origin.x) / z, (p.y - self.origin.y) / z)
    }

    /// A roda para os controles do form desenhado sob o ponteiro. Sem deslize (`lines`
    /// falso): o `Ui` do form não anda de quadro em quadro no designer, então a rolagem salta
    /// direto para a posição nova. `true` se algum controle rolou.
    fn scroll_inside(&mut self, event: &WheelEvent) -> bool {
        let z = self.zoom();
        let position = self.to_doc(event.position);
        let mut st = self.state.borrow_mut();
        let root = st.doc.root;
        let e = WheelEvent { position, delta: PointF::new(event.delta.x / z, event.delta.y / z), lines: false, touch: false, ..*event };
        let scrolled = st.doc.ui.wheel_controls(root, e);
        if scrolled {
            // A rolagem põe os filhos no lugar novo pelo layout: o clique e a seleção usam ele.
            st.doc.ui.layout();
        }
        scrolled
    }

    /// Retângulo do form → coordenadas da superfície, com o zoom `z`.
    /// A rolagem pela roda: só no eixo em que o form (com o que
    /// passou da borda dele) não cabe na área, e sem deixar ele sair dela. Cabendo, fica
    /// onde está.
    fn clamp_origin(&mut self, before: PointF) {
        let z = self.zoom();
        let b = self.base.bounds();
        let area = (b.width, self.tray_rect().map_or(b.height, |t| t.y));
        let content = {
            let st = self.state.borrow();
            let doc = &st.doc;
            let mut r = doc.rect_of(doc.root).unwrap_or_default();
            for (id, _) in doc.tree() {
                if doc.is_visual(id) {
                    if let Some(c) = doc.rect_of(id) {
                        r = r.union(c);
                    }
                }
            }
            r
        };
        let axis = |origin: f32, before: f32, start: f32, size: f32, view: f32| {
            let (start, size) = (start * z, size * z);
            if size + 2.0 * MARGIN <= view {
                // Cabe: não rola (só volta para dentro, se estava fora).
                return before.clamp(MARGIN - start, view - MARGIN - start - size);
            }
            origin.clamp(view - MARGIN - start - size, MARGIN - start)
        };
        self.origin.x = axis(self.origin.x, before.x, content.x, content.width, area.0);
        self.origin.y = axis(self.origin.y, before.y, content.y, content.height, area.1);
    }

    fn to_screen(&self, r: RectF, z: f32) -> RectF {
        RectF::new(self.origin.x + r.x * z, self.origin.y + r.y * z, r.width * z, r.height * z)
    }

    /// Um ponto do form → coordenadas da superfície.
    fn point_to_screen(&self, p: PointF, z: f32) -> PointF {
        PointF::new(self.origin.x + p.x * z, self.origin.y + p.y * z)
    }

    /// Onde fica a alça `h` do controle na superfície: no retângulo dele já girado (com o
    /// giro dele e dos contêineres), como a pintura mostra.
    fn handle_point(&self, doc: &Document, id: ComponentId, r: RectF, h: Handle, z: f32) -> PointF {
        self.point_to_screen(doc.rotated_point(id, h.point(r)), z)
    }

    fn handle_at(&self, p: PointF) -> Option<Handle> {
        let st = self.state.borrow();
        if st.doc.selected.len() != 1 {
            return None;
        }
        let selected = st.doc.selection?;
        if !st.doc.is_visual(selected) {
            return None;
        }
        let r = st.doc.rect_of(selected)?;
        // A alça escondida pela rolagem de um contêiner não se pega.
        let area = st.doc.visible_area(selected).map(|a| self.to_screen(a, st.zoom).inflate(HANDLE));
        Handle::ALL.into_iter().find(|&h| {
            let c = self.handle_point(&st.doc, selected, r, h, st.zoom);
            (c.x - p.x).abs() <= HANDLE && (c.y - p.y).abs() <= HANDLE && area.is_none_or(|a| a.contains(c))
        })
    }

    fn changed(&self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_change", EventArgs::None);
        ctx.repaint();
    }

    /// O ponto de arraste do controle embaixo do ponto (coordenadas do form), se houver.
    fn grip_at(&self, dp: PointF) -> Option<(ComponentId, usize)> {
        let st = self.state.borrow();
        let id = st.doc.component_at(dp).filter(|&id| id != st.doc.root)?;
        let r = st.doc.rect_of(id)?;
        let grip = st.doc.ui.component(id)?.design_hooks()?.design_grip(dp - r.origin())?;
        Some((id, grip))
    }

    /// Mostra o form inteiro de novo (depois de trocar de aba ou de zoom).
    pub fn reset_view(&mut self) {
        self.origin = PointF::new(MARGIN, MARGIN);
    }
}

impl Component for DesignSurface {
    fn class_name(&self) -> &'static str {
        "DesignSurface"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn props(&self) -> &'static [aylis_core::PropInfo] {
        // Os eventos que a superfície dispara para a IDE.
        const PROPS: &[aylis_core::PropInfo] = &[
            aylis_core::PropInfo::new("on_change", aylis_core::PropKind::Event, "Events"),
            aylis_core::PropInfo::new("on_context_menu", aylis_core::PropKind::Event, "Events"),
        ];
        PROPS
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.no_form() {
            return;
        }
        let p = event.position;
        let dp = self.to_doc(p);
        let device = self.state.borrow().doc.device;
        if self.rotate_rect(device).is_some_and(|r| r.contains(p)) {
            if let Some((d, landscape)) = device {
                self.state.borrow_mut().doc.show_device(Some((d, !landscape)));
            }
            ctx.fire("on_change", EventArgs::None);
            ctx.repaint();
            return;
        }
        if event.button == PointerButton::Secondary {
            // Clique direito: seleciona o que está embaixo (se não estiver selecionado) e
            // pede o menu de contexto.
            {
                let mut st = self.state.borrow_mut();
                let hit = st.doc.component_at(dp).unwrap_or(st.doc.root);
                if !st.doc.is_selected(hit) {
                    st.doc.select(Some(hit));
                }
            }
            self.changed(ctx);
            return;
        }
        if event.button != PointerButton::Primary {
            return;
        }
        // bandeja de não visuais
        if let Some(tray) = self.tray_rect().filter(|t| t.contains(p)) {
            let chips = self.state.borrow().doc.non_visual();
            // O X do chip exclui o componente (como o Delete: o Ctrl+Z traz de volta).
            if let Some(i) = self.close_at(p) {
                let mut st = self.state.borrow_mut();
                st.doc.select(Some(chips[i]));
                let name = st.doc.ui.name(chips[i]).unwrap_or_default().to_string();
                if st.doc.delete_selected() {
                    st.status = format!("{name} excluído (Ctrl+Z desfaz)");
                }
                drop(st);
                self.close_hover = None;
                self.changed(ctx);
                return;
            }
            if let Some(i) = (0..chips.len()).find(|&i| Self::chip_rect(tray, i).contains(p)) {
                let mut st = self.state.borrow_mut();
                if event.modifiers.ctrl || event.modifiers.shift {
                    st.doc.toggle(chips[i]);
                } else {
                    st.doc.select(Some(chips[i]));
                }
            }
            self.changed(ctx);
            return;
        }
        // componente da paleta esperando para ser colocado
        let pending = self.state.borrow_mut().pending.take();
        // Cano e fio: apertar num componente e soltar em outro liga os dois.
        if let Some(class) = pending.as_ref().filter(|c| is_link_class(c)) {
            let from = {
                let st = self.state.borrow();
                st.doc.component_at(dp).filter(|&h| h != st.doc.root)
            };
            if let Some(from) = from {
                self.drag = Drag::Connect { class: class.clone(), from, start: dp, to: dp };
                ctx.repaint();
                return;
            }
        }
        if let Some(class) = pending {
            let result = self.state.borrow_mut().doc.add(&class, Some(dp));
            if let Err(message) = result {
                self.state.borrow_mut().status = message;
            }
            self.changed(ctx);
            return;
        }
        if let Some(handle) = self.handle_at(p) {
            let st = self.state.borrow();
            if let (Some(id), Some(start)) = (st.doc.selection, st.doc.selection.and_then(|s| st.doc.start_rect(s))) {
                self.drag = Drag::Resize { id, handle, start, from: dp, began: false };
            }
            return;
        }
        if let Some((id, grip)) = self.grip_at(dp).filter(|_| event.click_count == 1) {
            self.state.borrow_mut().doc.select(Some(id));
            self.drag = Drag::Grip { id, grip, from: dp, began: false };
            self.changed(ctx);
            return;
        }
        let (hit, root) = {
            let st = self.state.borrow();
            (st.doc.component_at(dp), st.doc.root)
        };
        let additive = event.modifiers.ctrl || event.modifiers.shift;
        let mut st = self.state.borrow_mut();
        match hit {
            // Fora de qualquer controle, ou no fundo do form: seleção por área.
            None => {
                if !additive {
                    st.doc.select(Some(root));
                }
                self.drag = Drag::Area { from: dp, to: dp };
            }
            Some(h) if h == root && event.click_count == 1 => {
                if !additive {
                    st.doc.select(Some(root));
                }
                self.drag = Drag::Area { from: dp, to: dp };
            }
            // Um componente que não se move (alinhado ao pai: o ScrollBox e os layouts que
            // cobrem o form): o clique seleciona ele e o arrasto abre o retângulo de seleção
            // dentro dele, como no fundo do form (relatado pelo autor em 07/11, na Landing,
            // onde não sobrava fundo livre para começar o retângulo).
            Some(h) if event.click_count == 1 && !additive && !st.doc.movable(h) => {
                if !st.doc.is_selected(h) {
                    st.doc.select(Some(h));
                } else {
                    st.doc.selection = Some(h);
                }
                self.drag = Drag::Area { from: dp, to: dp };
            }
            Some(h) => {
                if additive {
                    st.doc.toggle(h);
                } else if !st.doc.is_selected(h) {
                    st.doc.select(Some(h));
                } else {
                    // Já selecionado (num grupo): vira o principal e o grupo continua.
                    st.doc.selection = Some(h);
                }
                // No duplo clique o Ui já disparou on_dbl_click (o 1º clique selecionou).
                if event.click_count == 1 && !additive && st.doc.movable(h) {
                    let mut items: Vec<(ComponentId, RectF)> = Vec::new();
                    // O clicado primeiro: é ele que encaixa nas guias.
                    for id in std::iter::once(h).chain(st.doc.selected.iter().copied().filter(|&s| s != h)) {
                        if st.doc.movable(id) {
                            if let Some(start) = st.doc.start_rect(id) {
                                items.push((id, start));
                            }
                        }
                    }
                    self.drag = Drag::Move { items, from: dp, began: false };
                }
            }
        }
        drop(st);
        self.changed(ctx);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.no_form() {
            return;
        }
        let dp = self.to_doc(event.position);
        match &mut self.drag {
            Drag::Move { items, from, began } => {
                let delta = dp - *from;
                if !*began && delta.x.abs() < 3.0 && delta.y.abs() < 3.0 {
                    return;
                }
                let mut st = self.state.borrow_mut();
                if !*began {
                    st.doc.begin_change();
                    *began = true;
                }
                let items = items.clone();
                st.doc.move_group(&items, delta);
                ctx.repaint();
            }
            Drag::Resize { id, handle, start, from, began } => {
                let delta = dp - *from;
                if !*began && delta.x.abs() < 3.0 && delta.y.abs() < 3.0 {
                    return;
                }
                let mut st = self.state.borrow_mut();
                if !*began {
                    st.doc.begin_change();
                    *began = true;
                }
                // Girado: no eixo do controle, com o lado oposto parado na tela.
                st.doc.resize_rotated(*id, *handle, *start, delta);
                ctx.repaint();
            }
            Drag::Area { to, .. } | Drag::Connect { to, .. } => {
                *to = dp;
                ctx.repaint();
            }
            Drag::Grip { id, grip, from, began } => {
                let delta = dp - *from;
                if !*began && delta.x.abs() < 2.0 && delta.y.abs() < 2.0 {
                    return;
                }
                let mut st = self.state.borrow_mut();
                let begin = !*began;
                if begin {
                    st.doc.begin_change();
                    *began = true;
                }
                let change = st.doc.ui.component_mut(*id).and_then(|c| c.design_hooks_mut()).and_then(|h| h.design_drag(*grip, delta, begin));
                if let Some((target, name, value)) = change {
                    let _ = st.doc.ui.set_prop(target.unwrap_or(*id), name, value);
                }
                ctx.repaint();
            }
            Drag::None => {
                let over_close = self.close_at(event.position);
                if over_close != self.close_hover {
                    self.close_hover = over_close;
                    let tip = if over_close.is_some() { "Excluir" } else { "" };
                    self.base.set_hint(tip);
                    ctx.repaint();
                }
                let cursor = match self.handle_at(event.position) {
                    // A seta aponta para onde a alça está na tela (girada com o controle).
                    Some(h) => {
                        let st = self.state.borrow();
                        let turn = st.doc.selection.map_or(0.0, |id| st.doc.total_rotation(id));
                        resize_cursor(h.angle() + turn)
                    }
                    None if over_close.is_some() => CursorShape::Pointer,
                    None if self.grip_at(dp).is_some() => CursorShape::ResizeHorizontal,
                    None if self.state.borrow().pending.is_some() => CursorShape::Crosshair,
                    None => CursorShape::Default,
                };
                if cursor != self.base.cursor() {
                    self.base.set_cursor(cursor);
                }
            }
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.no_form() {
            return;
        }
        if event.button == PointerButton::Secondary {
            ctx.fire("on_context_menu", EventArgs::Pointer(*event));
            return;
        }
        let drag = std::mem::replace(&mut self.drag, Drag::None);
        let changed = match drag {
            Drag::Move { began, .. } | Drag::Resize { began, .. } | Drag::Grip { began, .. } => began,
            Drag::Area { from, to } => {
                let area = RectF::from_points(from, to);
                if area.width > 3.0 || area.height > 3.0 {
                    let mut st = self.state.borrow_mut();
                    let found = st.doc.components_in(area);
                    if !found.is_empty() {
                        let mut ids = if event.modifiers.ctrl || event.modifiers.shift { st.doc.selected.clone() } else { Vec::new() };
                        ids.extend(found);
                        st.doc.select_many(ids);
                    }
                }
                true
            }
            Drag::Connect { class, from, start, to } => {
                let mut st = self.state.borrow_mut();
                let target = st.doc.component_at(to).filter(|&h| h != st.doc.root && h != from);
                let class = class.split('@').next().unwrap_or(&class).to_string();
                match target.map(|t| st.doc.connect(&class, from, start, t, to)) {
                    Some(Ok(_)) => st.status = String::new(),
                    Some(Err(e)) => st.status = format!("⚠ {e}"),
                    None => {
                        // Soltou fora de outro componente: o cano continua esperando.
                        st.pending = Some(class);
                        st.status = "Solte sobre outro componente para ligar".into();
                    }
                }
                true
            }
            Drag::None => false,
        };
        self.state.borrow_mut().doc.guides.clear();
        if changed {
            self.changed(ctx);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if self.no_form() {
            return;
        }
        if event.modifiers.ctrl {
            // Ctrl + roda: zoom em volta do ponteiro.
            let before = self.to_doc(event.position);
            {
                let mut st = self.state.borrow_mut();
                let factor = if event.delta.y > 0.0 { 1.1 } else { 1.0 / 1.1 };
                st.zoom = (st.zoom * factor).clamp(0.25, 4.0);
            }
            let z = self.zoom();
            self.origin = PointF::new(event.position.x - before.x * z, event.position.y - before.y * z);
            self.changed(ctx);
        } else if event.modifiers.shift {
            let before = self.origin;
            self.origin.x += event.delta.y;
            self.clamp_origin(before);
        } else if self.scroll_inside(event) {
            // Um ScrollBox (ou outro controle que rola) do form sob o ponteiro: rola ele, para
            // ver e editar o que está abaixo (relatado pelo autor em 07/11, na Landing).
        } else {
            let before = self.origin;
            self.origin.x += event.delta.x;
            self.origin.y += event.delta.y;
            self.clamp_origin(before);
        }
        ctx.repaint();
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if self.no_form() {
            return;
        }
        // Pedido do autor em 08/11: a seta e o Ctrl+seta movem 1 px,
        // Ctrl+Shift+seta move 8 px, Shift+seta muda o tamanho 1 px; Alt+↑/↓ muda a ordem entre os
        // irmãos. Valem para todos os escolhidos.
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        let (shift, alt) = (event.modifiers.shift, event.modifiers.alt);
        let mut st = self.state.borrow_mut();
        let selected = st.doc.selection;
        let delta = match event.key {
            Key::Left => (-1.0, 0.0),
            Key::Right => (1.0, 0.0),
            Key::Up => (0.0, -1.0),
            Key::Down => (0.0, 1.0),
            _ => (0.0, 0.0),
        };
        match event.key {
            Key::Up | Key::Down if alt => {
                if let Some(id) = selected {
                    st.doc.move_in_order(id, event.key == Key::Up);
                }
            }
            Key::Left | Key::Right | Key::Up | Key::Down if shift && !ctrl => {
                if let Some(id) = selected {
                    st.doc.resize_by(id, delta.0, delta.1);
                }
            }
            Key::Left | Key::Right | Key::Up | Key::Down => {
                if let Some(id) = selected {
                    let step = if ctrl && shift { 8.0 } else { 1.0 };
                    st.doc.nudge(id, delta.0 * step, delta.1 * step);
                }
            }
            Key::Escape => {
                if st.pending.take().is_none() {
                    let parent = selected.and_then(|s| st.doc.ui.parent(s));
                    if parent.is_some() {
                        st.doc.select(parent);
                    }
                }
            }
            _ => return,
        }
        drop(st);
        ctx.set_handled();
        self.changed(ctx);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("background")));
        // pontinhos de fundo
        let dot = Brush::Solid(theme.color("text").with_opacity(0.06));
        let mut y = 12.0;
        while y < r.height {
            let mut x = 12.0;
            while x < r.width {
                canvas.fill_rect(RectF::new(x, y, 1.5, 1.5), 0.0, &dot);
                x += 24.0;
            }
            y += 24.0;
        }
        if self.no_form() {
            let mut text = ctx.text_style();
            text.h_align = TextAlign::Center;
            text.v_align = TextAlign::Center;
            text.color = theme.color("text_muted");
            let hint = "Nenhum form aberto: abra um no painel Projeto ou crie com Ctrl+N";
            canvas.draw_text(hint, RectF::new(0.0, 0.0, r.width, r.height), &text);
            return;
        }

        let tray = self.tray_rect();
        let zoom = self.zoom();
        let mut st = self.state.borrow_mut();
        let pending = st.pending.clone();
        let doc = &mut st.doc;
        let form_rect = doc.rect_of(doc.root).unwrap_or_default();
        let work = RectF::new(0.0, 0.0, r.width, tray.map_or(r.height, |t| t.y));
        let primary = theme.color("primary");

        // Num DataModule a bandeja ocupa tudo: a área do form fica vazia (sem form).
        canvas.save();
        canvas.clip_rect(work);
        let screen_form = self.to_screen(form_rect, zoom);
        let device = doc.device;
        let rounded = device.filter(|(d, _)| d.kind != aylis_core::devices::DeviceKind::Desktop).map_or(0.0, |(d, _)| d.radius);
        if let Some((d, landscape)) = device {
            if d.kind == aylis_core::devices::DeviceKind::Desktop {
                // A tela do computador, com o form como uma janela no meio dela.
                let size = d.size(landscape);
                let monitor = self.to_screen(
                    RectF::new(form_rect.x - (size.width - form_rect.width) / 2.0, form_rect.y - (size.height - form_rect.height) / 2.0, size.width, size.height),
                    zoom,
                );
                let edge = 12.0 * zoom.max(0.5);
                canvas.fill_rect(monitor.inflate(edge), edge, &Brush::Solid(Color::rgb(0x1F, 0x22, 0x2A)));
                canvas.fill_rect(monitor, 0.0, &Brush::Solid(Color::rgb(0x2B, 0x3A, 0x55)));
            } else {
                let bezel = if d.kind == aylis_core::devices::DeviceKind::Phone { 12.0 } else { 18.0 } * zoom;
                let body = screen_form.inflate(bezel);
                canvas.fill_rect(body, d.radius * zoom + bezel, &Brush::Solid(Color::rgb(0x1F, 0x22, 0x2A)));
                canvas.stroke_rect(body, d.radius * zoom + bezel, &Stroke::new(Color::rgb(0x4B, 0x50, 0x5C), 1.5));
            }
        // Numa vista de celular ou tablet sem aparelho: uma moldura genérica, desenhada.
        } else if let Some(mode) = doc.view_mode.as_ref().filter(|m| m.name == "phone" || m.name == "tablet") {
            let bezel = if mode.name == "phone" { 14.0 } else { 20.0 } * zoom;
            let body = screen_form.inflate(bezel);
            canvas.fill_rect(body, bezel * 2.2, &Brush::Solid(Color::rgb(0x1F, 0x22, 0x2A)));
            canvas.stroke_rect(body, bezel * 2.2, &Stroke::new(Color::rgb(0x4B, 0x50, 0x5C), 1.5));
            if mode.name == "phone" {
                // A câmera no alto, no meio da borda.
                let d = (bezel * 0.45).max(4.0);
                canvas.fill_ellipse(RectF::new(body.x + body.width / 2.0 - d / 2.0, body.y + (bezel - d) / 2.0, d, d), &Brush::Solid(Color::rgb(0x0B, 0x0C, 0x10)));
            }
        }
        if device.is_none() {
            canvas.fill_rect(screen_form.offset(0.0, 6.0).inflate(2.0), 8.0, &Brush::Solid(Color::BLACK.with_opacity(0.25)));
        }
        canvas.save();
        canvas.translate(self.origin.x, self.origin.y);
        canvas.scale(zoom, zoom);
        // O que passa da borda do form (não aparece em execução) fica à vista, apagado. Sem
        // nada para fora, nem a camada (ela custa um quadro inteiro fora da tela).
        if doc.ui.overflows(doc.root) {
            canvas.save();
            let mut outside = Path::new();
            outside.rect(form_rect, 0.0);
            canvas.clip_path(&outside, true);
            canvas.save_layer(0.35);
            doc.ui.paint_overflow(doc.root, canvas);
            canvas.restore();
            canvas.restore();
        }
        if rounded > 0.0 {
            canvas.clip_round_rect(form_rect, rounded);
        } else {
            canvas.clip_rect(form_rect);
        }
        doc.ui.paint(doc.root, canvas);
        // O que o sistema desenha por cima no aparelho (a hora, o recorte, a barra de gestos).
        if let Some((d, landscape)) = device {
            aylis_core::devices::paint_overlay(canvas, d, landscape, form_rect, doc.dark);
        }
        canvas.restore();
        if device.is_none() {
            canvas.stroke_rect(screen_form.inflate(1.0), 0.0, &Stroke::new(theme.color("border"), 1.0));
        }

        // seleção: contorno em todos; alças só com um selecionado
        let single = doc.selected.len() == 1;
        for &id in &doc.selected {
            if !doc.is_visual(id) {
                continue;
            }
            let Some(r) = doc.rect_of(id) else { continue };
            // Dentro de um contêiner que recorta (um ScrollBox rolado): a seleção só aparece na
            // parte visível, e some se ele saiu da vista (relatado pelo autor em 07/11).
            let area = doc.visible_area(id);
            if area.is_some_and(|a| a.is_empty() || !a.intersects(r)) {
                continue;
            }
            let clipped = area.filter(|a| !a.contains_rect(r));
            if let Some(a) = clipped {
                canvas.save();
                canvas.clip_rect(self.to_screen(a, zoom));
            }
            let main = doc.selection == Some(id);
            let color = if main { primary } else { primary.with_opacity(0.7) };
            // O contorno acompanha o giro do controle (e dos contêineres).
            if doc.total_rotation(id) == 0.0 && doc.rotated_point(id, r.origin()) == r.origin() {
                canvas.stroke_rect(self.to_screen(r, zoom).inflate(1.0), 0.0, &Stroke::new(color, 1.5));
            } else {
                let corners = [Handle::TopLeft, Handle::TopRight, Handle::BottomRight, Handle::BottomLeft].map(|h| self.handle_point(doc, id, r, h, zoom));
                for i in 0..4 {
                    canvas.draw_line(corners[i], corners[(i + 1) % 4], &Stroke::new(color, 1.5));
                }
            }
            if single {
                for h in Handle::ALL {
                    let c = self.handle_point(doc, id, r, h, zoom);
                    let square = RectF::new(c.x - 3.5, c.y - 3.5, HANDLE, HANDLE);
                    canvas.fill_rect(square, 1.0, &Brush::Solid(Color::WHITE));
                    canvas.stroke_rect(square, 1.0, &Stroke::new(primary, 1.2));
                }
            } else {
                // Grupo: cantos pequenos.
                for h in [Handle::TopLeft, Handle::TopRight, Handle::BottomRight, Handle::BottomLeft] {
                    let c = self.handle_point(doc, id, r, h, zoom);
                    canvas.fill_rect(RectF::new(c.x - 2.5, c.y - 2.5, 5.0, 5.0), 0.0, &Brush::Solid(theme.color("text_muted")));
                }
            }
            if clipped.is_some() {
                canvas.restore();
            }
        }
        // guias do arrasto
        let guide = Stroke::new(Color::from_hex("#ec4899").unwrap_or(primary), 1.0).dashed(vec![4.0, 3.0]);
        for g in &doc.guides {
            match *g {
                Guide::Vertical { x, from, to } => {
                    let x = self.origin.x + x * zoom;
                    canvas.draw_line(PointF::new(x, self.origin.y + from * zoom), PointF::new(x, self.origin.y + to * zoom), &guide);
                }
                Guide::Horizontal { y, from, to } => {
                    let y = self.origin.y + y * zoom;
                    canvas.draw_line(PointF::new(self.origin.x + from * zoom, y), PointF::new(self.origin.x + to * zoom, y), &guide);
                }
            }
        }
        // seleção por área
        if let Drag::Area { from, to } = &self.drag {
            let area = self.to_screen(RectF::from_points(*from, *to), zoom);
            canvas.fill_rect(area, 0.0, &Brush::Solid(primary.with_opacity(0.12)));
            canvas.stroke_rect(area, 0.0, &Stroke::new(primary, 1.0).dashed(vec![3.0, 3.0]));
        }
        // ligando dois componentes: a linha do ponto de partida até o mouse
        if let Drag::Connect { start, to, .. } = &self.drag {
            let a = self.to_screen(RectF::from_points(*start, *start), zoom).origin();
            let b = self.to_screen(RectF::from_points(*to, *to), zoom).origin();
            canvas.draw_line(a, b, &Stroke::new(primary, 2.0).dashed(vec![6.0, 4.0]));
            canvas.fill_ellipse(RectF::new(a.x - 4.0, a.y - 4.0, 8.0, 8.0), &Brush::Solid(primary));
        }
        canvas.restore();

        let mut text = ctx.text_style();
        if let Some(class) = pending {
            text.color = primary;
            let hint = if is_link_class(&class) {
                format!("Arraste de um componente até outro para ligar com um {} (Esc cancela)", class.replace('@', " · "))
            } else {
                format!("Clique no form para colocar um {} (Esc cancela)", class.replace('@', " · "))
            };
            canvas.draw_text(&hint, RectF::new(12.0, 4.0, r.width - 24.0, 20.0), &text);
        }
        if (zoom - 1.0).abs() > 0.01 {
            text.color = theme.color("text_muted");
            text.h_align = TextAlign::Trailing;
            canvas.draw_text(&format!("{:.0}%", zoom * 100.0), RectF::new(0.0, 4.0, r.width - 12.0, 20.0), &text);
            text.h_align = TextAlign::Leading;
        }

        // Girar o aparelho: no canto, só com um que gira.
        if let Some(button) = self.rotate_rect(device) {
            canvas.fill_rect(button, 6.0, &Brush::Solid(theme.color("surface")));
            canvas.stroke_rect(button, 6.0, &Stroke::new(theme.color("border"), 1.0));
            let mut text = ctx.text_style();
            text.h_align = TextAlign::Center;
            text.color = theme.color("text");
            let label = if device.is_some_and(|(_, l)| l) { "⟳ Em pé" } else { "⟳ Deitar" };
            canvas.draw_text(&aylis_core::messages::t(label), button, &text);
        }

        // bandeja de não visuais
        if let Some(tray) = tray {
            canvas.fill_rect(tray, 0.0, &Brush::Solid(theme.color("surface")));
            canvas.draw_line(PointF::new(0.0, tray.y + 0.5), PointF::new(r.width, tray.y + 0.5), &Stroke::new(theme.color("border"), 1.0));
            for (i, id) in doc.non_visual().into_iter().enumerate() {
                let chip = Self::chip_rect(tray, i);
                let selected = doc.is_selected(id);
                let fill = if selected { primary.with_opacity(0.2) } else { theme.color("background") };
                canvas.fill_rect(chip, 6.0, &Brush::Solid(fill));
                canvas.stroke_rect(chip, 6.0, &Stroke::new(if selected { primary } else { theme.color("border") }, 1.0));
                let name = doc.ui.name(id).unwrap_or_default();
                let class = doc.ui.class_name(id).unwrap_or_default();
                text.color = theme.color("text");
                text.h_align = TextAlign::Leading;
                text.font.size = 12.0;
                // O ícone do componente (cada não visual tem o seu).
                let glyph = crate::nonvisual::icon(class).unwrap_or("plug");
                ctx.draw_icon(canvas, glyph, RectF::new(chip.x + 9.0, chip.y + (chip.height - 18.0) / 2.0, 18.0, 18.0), primary);
                canvas.draw_text(name, RectF::new(chip.x + 34.0, chip.y + 2.0, chip.width - 62.0, 16.0), &text);
                text.color = theme.color("text_muted");
                text.font.size = 11.0;
                canvas.draw_text(class, RectF::new(chip.x + 34.0, chip.y + 16.0, chip.width - 62.0, 14.0), &text);
                // O X de excluir: discreto; vermelho com o ponteiro em cima.
                let close = Self::close_rect(chip);
                let hot = self.close_hover == Some(i);
                if hot {
                    canvas.fill_rect(close, 9.0, &Brush::Solid(theme.color("danger").with_opacity(0.85)));
                }
                let color = if hot { Color::WHITE } else { theme.color("text_muted") };
                let (c, d) = (close.center(), 4.0);
                let stroke = Stroke::new(color, 1.6);
                canvas.draw_line(PointF::new(c.x - d, c.y - d), PointF::new(c.x + d, c.y + d), &stroke);
                canvas.draw_line(PointF::new(c.x + d, c.y - d), PointF::new(c.x - d, c.y + d), &stroke);
            }
        }
    }
}

/// O cursor de redimensionar mais perto da direção `degrees` (0 = direita, 90 = baixo).
fn resize_cursor(degrees: f32) -> CursorShape {
    match ((degrees.rem_euclid(180.0) + 22.5) / 45.0) as u32 % 4 {
        0 => CursorShape::ResizeHorizontal,
        1 => CursorShape::ResizeNwSe,
        2 => CursorShape::ResizeVertical,
        _ => CursorShape::ResizeNeSw,
    }
}
