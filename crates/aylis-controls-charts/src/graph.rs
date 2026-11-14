//! NodeGraph: editor de nós e cabos (roteamento de áudio, fluxos, pipelines).
//!
//! - fundo de pontos numa grade; os nós encaixam nela;
//! - arrastar o fundo move a vista, a roda dá zoom no ponto do mouse; botões de zoom e
//!   "enquadrar tudo" no canto;
//! - arrastar um nó o move (`on_node_moved`); arrastar da porta de saída (direita) até um
//!   nó com entrada cria um cabo (`on_connect`);
//! - clicar seleciona nó (`on_node_click`) ou cabo (`on_cable_click`); Delete apaga o cabo
//!   selecionado (`on_cable_delete`).
//!
//! Os eventos levam o id (texto) do nó ou do cabo; os dados ficam no próprio controle
//! ([`NodeGraph::node`], [`NodeGraph::cable`]).

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, EventArgs, EventCtx, FontWeight, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerButton, PointerEvent, RectF, Stroke, TextAlign, Trimming, WheelEvent, properties,
};

use crate::leaf_control;

/// Um nó do grafo. Posição e tamanho em unidades do "mundo" (antes do zoom).
#[derive(Clone, Debug, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    /// Cor de destaque (barra lateral, portas, cabos que saem dele). `None` = `primary`.
    pub color: Option<Color>,
    pub position: PointF,
    pub width: f32,
    pub has_input: bool,
    pub has_output: bool,
    /// Medidor de nível embaixo (0 a 1); `None` = sem medidor.
    pub level: Option<f32>,
    /// Selos em cima, à direita ("exclusivo", "5.0 ms").
    pub badges: Vec<String>,
}

impl GraphNode {
    pub fn new(id: impl Into<String>, title: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: String::new(),
            icon: String::new(),
            color: None,
            position: PointF::new(x, y),
            width: 300.0,
            has_input: true,
            has_output: true,
            level: None,
            badges: Vec::new(),
        }
    }

    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = s.into();
        self
    }

    pub fn icon(mut self, s: impl Into<String>) -> Self {
        self.icon = s.into();
        self
    }

    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }

    /// Só saída (uma fonte: microfone).
    pub fn source(mut self) -> Self {
        self.has_input = false;
        self
    }

    /// Só entrada (um destino: alto-falante).
    pub fn sink(mut self) -> Self {
        self.has_output = false;
        self
    }

    pub fn level(mut self, v: f32) -> Self {
        self.level = Some(v);
        self
    }

    pub fn badge(mut self, b: impl Into<String>) -> Self {
        self.badges.push(b.into());
        self
    }

    fn height(&self) -> f32 {
        if self.level.is_some() { 84.0 } else { 72.0 }
    }

    fn rect(&self) -> RectF {
        RectF::new(self.position.x, self.position.y, self.width, self.height())
    }

    fn out_port(&self) -> PointF {
        PointF::new(self.position.x + self.width, self.position.y + self.height() / 2.0)
    }

    fn in_port(&self) -> PointF {
        PointF::new(self.position.x, self.position.y + self.height() / 2.0)
    }
}

/// Um cabo, de um nó (saída) a outro (entrada).
#[derive(Clone, Debug, PartialEq)]
pub struct GraphCable {
    pub id: String,
    pub from: String,
    pub to: String,
    /// Rótulo no meio ("0.0 dB", "mudo").
    pub label: String,
    /// Tracejado (desligado, mudo).
    pub dashed: bool,
    /// `None` = a cor do nó de origem.
    pub color: Option<Color>,
}

impl GraphCable {
    pub fn new(id: impl Into<String>, from: impl Into<String>, to: impl Into<String>) -> Self {
        Self { id: id.into(), from: from.into(), to: to.into(), label: String::new(), dashed: false, color: None }
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = l.into();
        self
    }

    pub fn dashed(mut self, d: bool) -> Self {
        self.dashed = d;
        self
    }
}

/// O que está selecionado.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum GraphSelection {
    #[default]
    None,
    Node(String),
    Cable(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Drag {
    None,
    Pan { start: PointF, origin: PointF },
    Node { id: String, grab: PointF, moved: bool },
    Wire { from: String, at: PointF },
}

const GRID: f32 = 24.0;
const MIN_ZOOM: f32 = 0.3;
const MAX_ZOOM: f32 = 2.0;
const PORT: f32 = 6.0;
const BUTTON: f32 = 30.0;

/// Editor de nós e cabos. Veja o módulo.
pub struct NodeGraph {
    pub base: ControlBase,
    /// Encaixe na grade ao soltar um nó.
    pub snap: bool,
    /// Pinta a grade de pontos no fundo.
    pub show_grid: bool,
    nodes: Vec<GraphNode>,
    cables: Vec<GraphCable>,
    selection: GraphSelection,
    /// Deslocamento da vista (pixels) e zoom.
    pan: PointF,
    zoom: f32,
    drag: Drag,
    next_cable: u64,
    hover_port: Cell<Option<usize>>,
    /// O usuário já mexeu na vista (aí o controle não enquadra mais sozinho).
    user_view: bool,
}

impl Default for NodeGraph {
    fn default() -> Self {
        let mut base = ControlBase::new(640.0, 400.0);
        base.set_can_focus(true).set_clip_children(true).set_pan_on_touch(false);
        Self {
            base,
            snap: true,
            show_grid: true,
            nodes: Vec::new(),
            cables: Vec::new(),
            selection: GraphSelection::None,
            pan: PointF::new(0.0, 0.0),
            zoom: 1.0,
            drag: Drag::None,
            next_cable: 1,
            hover_port: Cell::new(None),
            user_view: false,
        }
    }
}

/// Curva que sai na horizontal e chega na horizontal (como a dos editores de nós).
fn cable_path(a: PointF, b: PointF) -> (Path, PointF) {
    let dx = b.x - a.x;
    let off = if dx >= 0.0 { 0.5 * dx } else { 6.25 * (-dx).sqrt() };
    let (c1, c2) = (PointF::new(a.x + off, a.y), PointF::new(b.x - off, b.y));
    let mut p = Path::new();
    p.move_to(a.x, a.y).cubic_to(c1.x, c1.y, c2.x, c2.y, b.x, b.y);
    let mid = PointF::new((a.x + 3.0 * c1.x + 3.0 * c2.x + b.x) / 8.0, (a.y + b.y) / 2.0);
    (p, mid)
}

impl NodeGraph {
    // ----- dados -----

    /// Acrescenta (ou substitui, pelo id) um nó.
    pub fn add_node(&mut self, node: GraphNode) {
        match self.nodes.iter_mut().find(|n| n.id == node.id) {
            Some(n) => *n = node,
            None => self.nodes.push(node),
        }
    }

    /// Remove o nó e os cabos dele.
    pub fn remove_node(&mut self, id: &str) {
        self.nodes.retain(|n| n.id != id);
        self.cables.retain(|c| c.from != id && c.to != id);
        if self.selection == GraphSelection::Node(id.to_string()) {
            self.selection = GraphSelection::None;
        }
    }

    pub fn node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut GraphNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes
    }

    /// Nível do medidor de um nó (chamado várias vezes por segundo).
    pub fn set_level(&mut self, id: &str, level: f32) {
        if let Some(n) = self.node_mut(id) {
            n.level = Some(level.clamp(0.0, 1.0));
        }
    }

    /// Acrescenta (ou substitui, pelo id) um cabo.
    pub fn connect(&mut self, cable: GraphCable) {
        match self.cables.iter_mut().find(|c| c.id == cable.id) {
            Some(c) => *c = cable,
            None => self.cables.push(cable),
        }
    }

    pub fn disconnect(&mut self, id: &str) {
        self.cables.retain(|c| c.id != id);
        if self.selection == GraphSelection::Cable(id.to_string()) {
            self.selection = GraphSelection::None;
        }
    }

    pub fn cable(&self, id: &str) -> Option<&GraphCable> {
        self.cables.iter().find(|c| c.id == id)
    }

    pub fn cable_mut(&mut self, id: &str) -> Option<&mut GraphCable> {
        self.cables.iter_mut().find(|c| c.id == id)
    }

    pub fn cables(&self) -> &[GraphCable] {
        &self.cables
    }

    pub fn selection(&self) -> &GraphSelection {
        &self.selection
    }

    pub fn select(&mut self, selection: GraphSelection) {
        self.selection = selection;
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Um cabo novo de `from` para `to` é aceito? (não liga um nó nele mesmo nem repete).
    pub fn can_connect(&self, from: &str, to: &str) -> bool {
        from != to
            && self.node(from).is_some_and(|n| n.has_output)
            && self.node(to).is_some_and(|n| n.has_input)
            && !self.cables.iter().any(|c| c.from == from && c.to == to)
    }

    // ----- vista -----

    fn to_world(&self, p: PointF) -> PointF {
        PointF::new((p.x - self.pan.x) / self.zoom, (p.y - self.pan.y) / self.zoom)
    }

    /// Ponto do mundo (dos nós) na tela (local do controle).
    pub fn to_view(&self, p: PointF) -> PointF {
        PointF::new(p.x * self.zoom + self.pan.x, p.y * self.zoom + self.pan.y)
    }

    fn zoom_at(&mut self, factor: f32, at: PointF) {
        let world = self.to_world(at);
        self.zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
        self.pan = PointF::new(at.x - world.x * self.zoom, at.y - world.y * self.zoom);
    }

    /// Ajusta zoom e posição para todos os nós caberem.
    pub fn fit(&mut self) {
        if self.nodes.is_empty() {
            self.pan = PointF::new(0.0, 0.0);
            self.zoom = 1.0;
            return;
        }
        let mut bounds = self.nodes[0].rect();
        for n in &self.nodes[1..] {
            bounds = bounds.union(n.rect());
        }
        let view = self.base.bounds();
        let margin = 40.0;
        let zx = (view.width - 2.0 * margin) / bounds.width.max(1.0);
        let zy = (view.height - 2.0 * margin) / bounds.height.max(1.0);
        self.zoom = zx.min(zy).clamp(MIN_ZOOM, 1.0);
        self.pan = PointF::new(
            view.width / 2.0 - (bounds.x + bounds.width / 2.0) * self.zoom,
            view.height / 2.0 - (bounds.y + bounds.height / 2.0) * self.zoom,
        );
    }

    /// Botões de zoom no canto de baixo à esquerda: (+, −, enquadrar).
    fn buttons(&self) -> [RectF; 3] {
        let h = self.base.bounds().height;
        let x = 12.0;
        let y = h - 12.0 - 3.0 * BUTTON;
        [0, 1, 2].map(|i| RectF::new(x, y + i as f32 * BUTTON, BUTTON, BUTTON))
    }

    // ----- testes de clique (em coordenadas do mundo) -----

    fn node_at(&self, w: PointF) -> Option<usize> {
        self.nodes.iter().rposition(|n| n.rect().contains(w))
    }

    fn out_port_at(&self, w: PointF) -> Option<usize> {
        let r = (PORT + 6.0) / self.zoom.min(1.0);
        self.nodes.iter().rposition(|n| {
            let p = n.out_port();
            n.has_output && (w.x - p.x).hypot(w.y - p.y) <= r
        })
    }

    fn cable_at(&self, w: PointF) -> Option<usize> {
        let tolerance = 6.0 / self.zoom;
        self.cables.iter().rposition(|c| {
            let (Some(a), Some(b)) = (self.node(&c.from), self.node(&c.to)) else { return false };
            cable_path(a.out_port(), b.in_port()).0.distance_to(w) <= tolerance
        })
    }

    /// Nó que aceita um cabo solto em `w` (dentro dele ou perto da porta de entrada).
    fn drop_target(&self, from: &str, w: PointF) -> Option<String> {
        let radius = 40.0 / self.zoom;
        self.nodes
            .iter()
            .filter(|n| self.can_connect(from, &n.id))
            .find(|n| n.rect().contains(w) || (w.x - n.in_port().x).hypot(w.y - n.in_port().y) <= radius)
            .map(|n| n.id.clone())
    }

    fn accent(&self, ctx: &PaintCtx<'_>, node: &GraphNode) -> Color {
        node.color.unwrap_or_else(|| ctx.theme.color("primary"))
    }
}

impl Component for NodeGraph {
    leaf_control!("NodeGraph");

    properties! {
        snap: bool => "Behavior",
        show_grid: bool => "Appearance";
        events: on_node_click, on_node_moved, on_cable_click, on_cable_delete, on_connect, on_pane_click
    }

    /// Enquadra os nós ao mudar de tamanho, até o usuário mover ou dar zoom na vista.
    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.user_view {
            self.fit();
            ctx.repaint();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        if event.button != PointerButton::Primary {
            return;
        }
        // Botões de zoom.
        let buttons = self.buttons();
        let center = PointF::new(self.base.bounds().width / 2.0, self.base.bounds().height / 2.0);
        if buttons.iter().any(|b| b.contains(p)) {
            self.user_view = true;
        }
        if buttons[0].contains(p) {
            self.zoom_at(1.2, center);
        } else if buttons[1].contains(p) {
            self.zoom_at(1.0 / 1.2, center);
        } else if buttons[2].contains(p) {
            self.fit();
        } else {
            let w = self.to_world(p);
            if let Some(i) = self.out_port_at(w) {
                self.drag = Drag::Wire { from: self.nodes[i].id.clone(), at: w };
            } else if let Some(i) = self.node_at(w) {
                let n = &self.nodes[i];
                let id = n.id.clone();
                self.drag = Drag::Node { id: id.clone(), grab: PointF::new(w.x - n.position.x, w.y - n.position.y), moved: false };
                // O nó escolhido fica por cima.
                let node = self.nodes.remove(i);
                self.nodes.push(node);
                self.selection = GraphSelection::Node(id.clone());
                ctx.fire("on_node_click", EventArgs::Text(id));
            } else if let Some(i) = self.cable_at(w) {
                let id = self.cables[i].id.clone();
                self.selection = GraphSelection::Cable(id.clone());
                ctx.fire("on_cable_click", EventArgs::Text(id));
            } else {
                self.drag = Drag::Pan { start: p, origin: self.pan };
                if self.selection != GraphSelection::None {
                    self.selection = GraphSelection::None;
                }
                ctx.fire("on_pane_click", EventArgs::None);
            }
        }
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        let w = self.to_world(p);
        match &mut self.drag {
            Drag::None => {
                let hover = self.out_port_at(w);
                if hover != self.hover_port.get() {
                    self.hover_port.set(hover);
                    ctx.repaint();
                }
                return;
            }
            Drag::Pan { start, origin } => {
                self.pan = PointF::new(origin.x + p.x - start.x, origin.y + p.y - start.y);
                self.user_view = true;
            }
            Drag::Node { id, grab, moved } => {
                *moved = true;
                let (id, grab) = (id.clone(), *grab);
                if let Some(n) = self.node_mut(&id) {
                    n.position = PointF::new(w.x - grab.x, w.y - grab.y);
                }
            }
            Drag::Wire { at, .. } => *at = w,
        }
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let w = self.to_world(event.position);
        match std::mem::replace(&mut self.drag, Drag::None) {
            Drag::Node { id, moved: true, .. } => {
                let snap = self.snap;
                if let Some(n) = self.node_mut(&id) {
                    if snap {
                        n.position = PointF::new((n.position.x / GRID).round() * GRID, (n.position.y / GRID).round() * GRID);
                    }
                }
                ctx.fire("on_node_moved", EventArgs::Text(id));
            }
            Drag::Wire { from, .. } => {
                if let Some(to) = self.drop_target(&from, w) {
                    let id = format!("cabo{}", self.next_cable);
                    self.next_cable += 1;
                    self.cables.push(GraphCable::new(id.clone(), from, to));
                    self.selection = GraphSelection::Cable(id.clone());
                    ctx.fire("on_connect", EventArgs::Text(id));
                }
            }
            _ => {}
        }
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let at = event.position;
        let factor = if event.delta.y > 0.0 { 1.1 } else { 1.0 / 1.1 };
        self.zoom_at(factor, at);
        self.user_view = true;
        ctx.repaint();
        ctx.set_handled();
    }

    /// Pinça: zoom no ponto entre os dedos; o centro andando arrasta a vista.
    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::GestureEvent) {
        self.pan = self.pan + event.pan;
        self.zoom_at(event.delta_scale, event.center);
        self.user_view = true;
        ctx.repaint();
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if matches!(event.key, Key::Delete | Key::Backspace) {
            if let GraphSelection::Cable(id) = self.selection.clone() {
                self.disconnect(&id);
                ctx.fire("on_cable_delete", EventArgs::Text(id));
                ctx.repaint();
                ctx.set_handled();
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let s = ctx.style;
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        canvas.save();
        canvas.clip_round_rect(r, s.radius);
        // Grade de pontos.
        if self.show_grid {
            let step = GRID * self.zoom;
            if step >= 8.0 {
                let dot = theme.color("text_muted").with_opacity(0.28);
                let x0 = self.pan.x.rem_euclid(step);
                let y0 = self.pan.y.rem_euclid(step);
                let size = (1.6 * self.zoom).clamp(1.0, 2.2);
                let mut y = y0;
                while y < r.height {
                    let mut x = x0;
                    while x < r.width {
                        canvas.fill_rect(RectF::new(x - size / 2.0, y - size / 2.0, size, size), size / 2.0, &Brush::Solid(dot));
                        x += step;
                    }
                    y += step;
                }
            }
        }
        canvas.save();
        canvas.translate(self.pan.x, self.pan.y);
        canvas.scale(self.zoom, self.zoom);
        let mut small = ctx.text_style();
        small.font.size = 12.0;
        small.h_align = TextAlign::Center;
        // Cabos.
        for c in &self.cables {
            let (Some(a), Some(b)) = (self.node(&c.from), self.node(&c.to)) else { continue };
            let (path, mid) = cable_path(a.out_port(), b.in_port());
            let selected = self.selection == GraphSelection::Cable(c.id.clone());
            let color = c.color.unwrap_or_else(|| self.accent(ctx, a));
            let mut stroke = Stroke::new(color, if selected { 4.0 } else { 2.5 }).round();
            if c.dashed {
                stroke = stroke.dashed(vec![6.0, 6.0]);
                stroke.brush = Brush::Solid(theme.color("text_muted"));
            }
            if selected {
                canvas.stroke_path(&path, &Stroke::new(color.with_opacity(0.25), 10.0).round());
            }
            canvas.stroke_path(&path, &stroke);
            if !c.label.is_empty() {
                let w = canvas.measure_text(&c.label, &small).width + 14.0;
                let pill = RectF::new(mid.x - w / 2.0, mid.y - 11.0, w, 22.0);
                canvas.fill_rect(pill, 11.0, &Brush::Solid(theme.color("surface")));
                canvas.stroke_rect(pill, 11.0, &Stroke::new(if selected { color } else { theme.color("border") }, 1.0));
                small.color = theme.color("text");
                canvas.draw_text(&c.label, pill, &small);
            }
        }
        // Cabo sendo puxado.
        if let Drag::Wire { from, at } = &self.drag {
            if let Some(a) = self.node(from) {
                let (path, _) = cable_path(a.out_port(), *at);
                canvas.stroke_path(&path, &Stroke::new(self.accent(ctx, a), 2.5).round().dashed(vec![8.0, 6.0]));
            }
        }
        // Nós.
        for (i, n) in self.nodes.iter().enumerate() {
            let rect = n.rect();
            let accent = self.accent(ctx, n);
            let selected = self.selection == GraphSelection::Node(n.id.clone());
            if let Some(shadow) = theme.shadows.get(if selected { "lg" } else { "md" }) {
                let mut glow = *shadow;
                if selected {
                    glow.color = accent.with_opacity(0.45);
                }
                canvas.draw_shadow(rect, 10.0, &glow);
            }
            canvas.fill_rect(rect, 10.0, &Brush::Solid(theme.color("surface")));
            canvas.stroke_rect(rect, 10.0, &Stroke::new(if selected { accent } else { theme.color("border") }, if selected { 2.0 } else { 1.0 }));
            canvas.save();
            canvas.clip_round_rect(rect, 10.0);
            canvas.fill_rect(RectF::new(rect.x, rect.y, 4.0, rect.height), 0.0, &Brush::Solid(accent));
            canvas.restore();
            let mut x = rect.x + 18.0;
            if !n.icon.is_empty() {
                let icon = RectF::new(x, rect.y + 18.0, 32.0, 32.0);
                canvas.fill_rect(icon, 8.0, &Brush::Solid(accent.with_opacity(0.15)));
                ctx.draw_icon(canvas, &n.icon, icon.deflate(aylis_core::Bounds::all(6.0)), accent);
                x += 44.0;
            }
            let mut title = ctx.text_style();
            title.font.weight = FontWeight::SemiBold;
            title.font.size = 15.0;
            title.trimming = Trimming::Character;
            let text_w = rect.right() - x - 14.0;
            canvas.draw_text(&n.title, RectF::new(x, rect.y + 14.0, text_w, 22.0), &title);
            if !n.subtitle.is_empty() {
                let mut sub = ctx.text_style();
                sub.color = theme.color("text_muted");
                sub.font.size = 12.5;
                canvas.draw_text(&n.subtitle, RectF::new(x, rect.y + 36.0, text_w, 18.0), &sub);
            }
            if let Some(level) = n.level {
                let track = RectF::new(x, rect.bottom() - 16.0, rect.right() - x - 16.0, 4.0);
                canvas.fill_rect(track, 2.0, &Brush::Solid(theme.color("track")));
                let w = track.width * level.clamp(0.0, 1.0);
                if w > 0.0 {
                    canvas.fill_rect(RectF::new(track.x, track.y, w, track.height), 2.0, &Brush::Solid(accent));
                }
            }
            // Selos em cima, encostados na borda direita.
            let mut bx = rect.right() - 12.0;
            for b in n.badges.iter().rev() {
                let w = canvas.measure_text(b, &small).width + 12.0;
                bx -= w;
                let pill = RectF::new(bx, rect.y - 10.0, w, 20.0);
                canvas.fill_rect(pill, 10.0, &Brush::Solid(theme.color("surface_2")));
                canvas.stroke_rect(pill, 10.0, &Stroke::new(theme.color("border"), 1.0));
                small.color = theme.color("text_muted");
                canvas.draw_text(b, pill, &small);
                bx -= 4.0;
            }
            // Portas.
            let port = |canvas: &mut dyn Canvas, p: PointF, big: bool| {
                let d = if big { PORT * 2.0 + 4.0 } else { PORT * 2.0 };
                let pr = RectF::new(p.x - d / 2.0, p.y - d / 2.0, d, d);
                canvas.fill_ellipse(pr, &Brush::Solid(accent));
                canvas.stroke_ellipse(pr, &Stroke::new(theme.color("surface"), 2.0));
            };
            if n.has_input {
                port(canvas, n.in_port(), false);
            }
            if n.has_output {
                port(canvas, n.out_port(), self.hover_port.get() == Some(i));
            }
        }
        canvas.restore();
        // Botões de zoom (fora da escala).
        let icons = ["plus", "minus", "fullscreen"];
        let group = self.buttons();
        let all = group[0].union(group[2]);
        canvas.fill_rect(all, 8.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(all, 8.0, &Stroke::new(theme.color("border"), 1.0));
        for (b, icon) in group.iter().zip(icons) {
            ctx.draw_icon(canvas, icon, b.deflate(aylis_core::Bounds::all(8.0)), theme.color("text"));
        }
        canvas.restore();
        if s.border_width > 0.0 {
            canvas.stroke_rect(r, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
    }
}

