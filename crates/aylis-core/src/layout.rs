//! Motor de alinhamento (o `AlignObjects` do FMX).
//!
//! Para cada controle, de cima para baixo na árvore:
//! 1. a área livre começa como o retângulo do controle menos o `padding`;
//! 2. os alinhamentos de encaixe consomem a área, nesta ordem: `MostTop`, `MostBottom`,
//!    `MostLeft`, `MostRight`, `Top`, `Bottom`, `Left`, `Right` (com o mesmo
//!    alinhamento, vale a ordem dos filhos). Cada um usa o próprio tamanho + `margins`;
//! 3. `Client`, `Center`, `VertCenter`, `HorzCenter`, `Horizontal`, `Vertical` e `Fit`
//!    usam a área que sobrou;
//! 4. `Contents` usa o controle inteiro, ignorando o padding;
//! 5. `None` usa `left/top/width/height`, ajustados pelas `anchors` quando o pai muda
//!    de tamanho.
//!
//! Controles invisíveis não ocupam espaço. Margens não afetam controles com `None`.

use crate::align::{Align, Anchors};
use crate::component::{AnchorRef, ComponentId, ControlBase};
use crate::event::EventArgs;
use crate::geometry::{PointF, RectF, SizeF};
use crate::ui::Ui;

const DOCK_ORDER: [Align; 8] = [
    Align::MostTop,
    Align::MostBottom,
    Align::MostLeft,
    Align::MostRight,
    Align::Top,
    Align::Bottom,
    Align::Left,
    Align::Right,
];

impl Ui {
    /// Recalcula o layout só se algo mudou desde a última vez.
    pub fn update_layout(&mut self) {
        if self.layout_dirty {
            self.layout();
        } else if !self.dirty_forms.is_empty() {
            // Só os forms que mudaram (como o `AlignControls` da VCL, que refaz o pai do
            // controle e não a aplicação inteira).
            let _profile = crate::profile::scope(crate::profile::Lane::Layout, || "layout de form".into());
            if !self.views.is_empty() {
                self.apply_views();
            }
            if self.layout_dirty {
                self.layout();
                return;
            }
            let forms: Vec<ComponentId> = self.roots().iter().copied().filter(|r| self.dirty_forms.contains(r)).collect();
            self.dirty_forms.clear();
            // Onde cada controle estava: depois do layout, só quem mudou de lugar, de tamanho
            // ou de visibilidade é repintado (o que mudou por dentro já marcou a área dele).
            let before: Vec<(ComponentId, Option<RectF>, bool)> = forms.iter().flat_map(|&f| self.placed(f)).collect();
            self.layout_roots(&forms);
            for (id, rect, visible) in before {
                if self.absolute_rect(id) != rect || self.cb(id).is_some_and(|cb| cb.visible()) != visible {
                    self.damage(id);
                }
            }
        }
    }

    /// Os controles de `root` (sem ele) com a posição no form e a visibilidade.
    fn placed(&self, root: ComponentId) -> Vec<(ComponentId, Option<RectF>, bool)> {
        let mut out = Vec::new();
        let mut stack = self.children(root).to_vec();
        while let Some(id) = stack.pop() {
            out.push((id, self.absolute_rect(id), self.cb(id).is_some_and(|cb| cb.visible())));
            stack.extend(self.children(id));
        }
        out
    }

    /// Recalcula a posição e o tamanho reais de todos os controles. Dispara `on_resize`
    /// nos controles cujo tamanho mudou.
    ///
    /// Um controle raiz (form) fica sempre na origem com o tamanho pedido; `left`/`top`
    /// dele são a posição da janela, que é assunto da plataforma.
    pub fn layout(&mut self) {
        let _profile = crate::profile::scope(crate::profile::Lane::Layout, || "layout".into());
        // A vista de cada form pela largura (as mudanças dela entram antes de alinhar).
        if !self.views.is_empty() {
            self.apply_views();
        }
        self.layout_dirty = false;
        self.dirty_forms.clear();
        let roots = self.roots().to_vec();
        self.layout_roots(&roots);
        self.request_repaint();
    }

    /// Alinha os forms `roots` (e dispara os `on_resize` de quem mudou de tamanho).
    fn layout_roots(&mut self, roots: &[ComponentId]) {
        let mut resized = Vec::new();
        for &root in roots {
            // As escalas da vista ativa descem do form para os filhos em `realign`.
            let (text, spacing) = self.view_scales(root);
            if let Some(cb) = self.cb_mut(root) {
                cb.text_scale = text;
                cb.spacing_scale = spacing;
            }
            if let Some(cb) = self.cb_mut(root) {
                let old = cb.bounds.size();
                // O mínimo vale também no form: numa janela (ou tela) menor, o que passar fica
                // cortado. O máximo do form é só o da janela (a plataforma limita).
                cb.bounds = RectF::new(0.0, 0.0, cb.width().max(cb.min_width()), cb.height().max(cb.min_height()));
                if cb.bounds.size() != old {
                    resized.push(root);
                }
            }
            self.realign(root, &mut resized);
            self.place_followers(root);
        }
        for &id in &resized {
            self.with_component(id, |c, ctx| c.resized(ctx));
        }
        for id in resized {
            self.queue_event(id, "on_resize", EventArgs::None);
        }
        self.process_events();
    }

    fn realign(&mut self, id: ComponentId, resized: &mut Vec<ComponentId>) {
        let Some(cb) = self.cb(id) else { return };
        let size = cb.bounds.size();
        let full = RectF::from_size(size);
        let mut area = full.deflate(cb.padding()).deflate(cb.safe_area);
        let scales = (cb.text_scale, cb.spacing_scale);
        let children = self.children(id).to_vec();
        // As escalas da vista do form valem para todos os controles dele (um form embutido
        // com vistas próprias usa as dele).
        for &child in &children {
            let own = if self.views.is_empty() { scales } else { self.own_view_scales(child).unwrap_or(scales) };
            if let Some(cb) = self.cb_mut(child) {
                if (cb.text_scale, cb.spacing_scale) != own {
                    cb.text_scale = own.0;
                    cb.spacing_scale = own.1;
                }
            }
        }
        let old_sizes: Vec<Option<SizeF>> =
            children.iter().map(|&c| self.cb(c).map(|b| b.bounds.size())).collect();

        // `auto_size`: a altura sai do conteúdo, na largura que o filho vai ter.
        for &child in &children {
            let Some(cb) = self.cb(child).filter(|cb| cb.visible() && cb.auto_size()) else { continue };
            let width = match cb.align() {
                Align::Top | Align::Bottom => cb.band_width((area.width - cb.margins().horizontal()).max(0.0)),
                Align::None => cb.clamp_width(cb.width()),
                _ => continue,
            };
            if let Some(h) = self.auto_height_of(child, width) {
                if let Some(cb) = self.cb_mut(child) {
                    cb.set_height(h);
                }
            }
        }

        for pass in DOCK_ORDER {
            for &child in &children {
                if let Some(cb) = self.cb_mut(child) {
                    if cb.visible() && cb.align() == pass {
                        dock(cb, &mut area);
                    }
                }
            }
        }
        // Contêineres que organizam os filhos (grade, fluxo) cuidam dos que estão soltos.
        let free: Vec<crate::component::ArrangeItem> = children
            .iter()
            .filter_map(|&c| {
                let cb = self.cb(c).filter(|cb| cb.visible() && cb.align() == Align::None)?;
                Some(crate::component::ArrangeItem::of(c, cb))
            })
            .collect();
        let arranged = if free.is_empty() { None } else { self.component(id).and_then(|c| c.arrange(area, &free)) };
        if let Some(rects) = &arranged {
            for (item, rect) in free.iter().zip(rects) {
                if let Some(cb) = self.cb_mut(item.id) {
                    cb.bounds = *rect;
                }
            }
        }
        for &child in &children {
            if arranged.is_some() && free.iter().any(|f| f.id == child) {
                continue;
            }
            if let Some(cb) = self.cb_mut(child) {
                if cb.visible() && !cb.align().is_dock() {
                    place(cb, area, full);
                }
            }
        }

        // Controles presos a um irmão (o Badge no canto do botão).
        for &child in &children {
            let Some((target, fx, fy)) = self.component(child).and_then(|c| c.attach()) else { continue };
            if self.parent(target) != Some(id) || target == child {
                continue;
            }
            let Some(t) = self.cb(target).map(|cb| cb.bounds) else { continue };
            if let Some(cb) = self.cb_mut(child) {
                let (w, h) = (cb.width(), cb.height());
                cb.bounds = RectF::new(t.x + t.width * fx - w / 2.0, t.y + t.height * fy - h / 2.0, w, h);
            }
        }

        // Conectores (canos, fios) presos a dois irmãos: em volta do caminho entre eles.
        for &child in &children {
            let Some(c) = self.component(child).and_then(|c| c.platform_hooks()).and_then(|h| h.connection()) else { continue };
            if self.parent(c.from) != Some(id) || self.parent(c.to) != Some(id) || c.from == child || c.to == child {
                continue;
            }
            let (Some(a), Some(b)) = (self.cb(c.from).map(|cb| cb.bounds), self.cb(c.to).map(|cb| cb.bounds)) else { continue };
            let points = c.route(a, b);
            let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
            for p in &points {
                (x0, y0, x1, y1) = (x0.min(p.x), y0.min(p.y), x1.max(p.x), y1.max(p.y));
            }
            // Folga para a espessura do cano.
            let pad = 12.0;
            if let Some(cb) = self.cb_mut(child) {
                cb.bounds = RectF::new(x0 - pad, y0 - pad, x1 - x0 + 2.0 * pad, y1 - y0 + 2.0 * pad);
            }
        }

        // Tamanho do conteúdo e rolagem.
        let (padding, offset) = match self.cb(id) {
            Some(cb) => (cb.padding(), cb.scroll_offset()),
            None => return,
        };
        let mut extent = SizeF::new(padding.horizontal(), padding.vertical());
        for &child in &children {
            if let Some(cb) = self.cb(child).filter(|cb| cb.visible()) {
                let m = cb.margins();
                extent.width = extent.width.max(cb.bounds.right() + m.right + padding.right);
                extent.height = extent.height.max(cb.bounds.bottom() + m.bottom + padding.bottom);
            }
        }
        if let Some(cb) = self.cb_mut(id) {
            cb.content_size = extent;
        }
        // Interface espelhada (árabe): os filhos do outro lado (o encaixe à esquerda vai
        // para a direita, o fluxo e a grade correm da direita para a esquerda).
        if self.is_mirrored(id) {
            for &child in &children {
                if let Some(cb) = self.cb_mut(child) {
                    let b = cb.bounds;
                    cb.bounds = RectF::new(size.width - b.right(), b.y, b.width, b.height);
                }
            }
        }
        // O form rola só o que o conteúdo passa da parte visível (o teclado fechou: volta).
        let offset = if offset.y > 0.0 && self.parent(id).is_none() {
            let y = offset.y.min(self.form_scroll_max(id));
            if let Some(cb) = self.cb_mut(id) {
                cb.set_scroll_offset(crate::geometry::PointF::new(offset.x, y));
            }
            crate::geometry::PointF::new(offset.x, y)
        } else if offset != crate::geometry::PointF::ZERO && self.parent(id).is_some() {
            // Um contêiner rolado cujo conteúdo encolheu (uma busca filtrou a lista): a
            // rolagem não passa do fim, senão o que sobrou fica acima da parte visível.
            let size = self.cb(id).map_or(SizeF::new(0.0, 0.0), |cb| cb.bounds.size());
            let clamped = crate::geometry::PointF::new(
                offset.x.min((extent.width - size.width).max(0.0)).max(0.0),
                offset.y.min((extent.height - size.height).max(0.0)).max(0.0),
            );
            if clamped != offset {
                if let Some(cb) = self.cb_mut(id) {
                    cb.set_scroll_offset(clamped);
                }
            }
            clamped
        } else {
            offset
        };
        if let Some(cb) = self.cb_mut(id) {
            cb.laid_out_scroll = offset;
        }
        if offset != crate::geometry::PointF::ZERO {
            for &child in &children {
                if let Some(cb) = self.cb_mut(child) {
                    cb.bounds = cb.bounds.offset(-offset.x, -offset.y);
                }
            }
        }

        for (&child, old) in children.iter().zip(old_sizes) {
            // Escondido: o ramo inteiro fica para quando aparecer (mostrar marca o layout).
            // Telas e abas escondidas (dezenas de textos com altura automática) custavam a
            // cada layout como se estivessem à vista.
            if !self.cb(child).is_some_and(|cb| cb.visible()) {
                continue;
            }
            if let (Some(old), Some(cb)) = (old, self.cb(child)) {
                if cb.bounds.size() != old {
                    resized.push(child);
                }
            }
            self.realign(child, resized);
        }
    }
}

impl Ui {
    /// A altura do conteúdo de `id` na largura `width` (o `auto_size`): a que o controle
    /// mede ou, num contêiner, a soma dos filhos encaixados em cima e embaixo (sem nenhum, só
    /// o padding: uma lista vazia não ocupa lugar).
    fn auto_height_of(&self, id: ComponentId, width: f32) -> Option<f32> {
        if let Some(h) = self.component(id).and_then(|c| c.auto_height(self, id, width)) {
            return Some(h.ceil());
        }
        let cb = self.cb(id)?;
        let padding = cb.padding();
        let inner = width - padding.horizontal();
        let mut total = padding.vertical();
        for &child in self.children(id) {
            let Some(c) = self.cb(child).filter(|c| c.visible() && matches!(c.align(), Align::Top | Align::Bottom)) else { continue };
            let m = c.margins();
            let h = if c.auto_size() { self.auto_height_of(child, c.band_width((inner - m.horizontal()).max(0.0))).unwrap_or(c.height()) } else { c.height() };
            let h = c.clamp_height(h);
            total += h + m.vertical();
        }
        // Um contêiner que organiza os soltos (grade, fluxo): mais a altura do que ele monta
        // nesta largura (a grade com `row_height`, o fluxo pelos tamanhos dos filhos).
        let free: Vec<crate::component::ArrangeItem> = self
            .children(id)
            .iter()
            .filter_map(|&c| {
                let cb = self.cb(c).filter(|cb| cb.visible() && cb.align() == Align::None)?;
                Some(crate::component::ArrangeItem::of(c, cb))
            })
            .collect();
        if !free.is_empty() {
            let area = RectF::new(0.0, 0.0, inner.max(0.0), 0.0);
            if let Some(rects) = self.component(id).and_then(|c| c.arrange(area, &free)) {
                let bottom = rects.iter().zip(&free).map(|(r, f)| r.bottom() + f.margins.bottom).fold(0.0f32, f32::max);
                total += bottom;
            }
        }
        Some(total)
    }
}

/// A faixa horizontal de um controle em cima ou embaixo: a largura (a da faixa ou, com
/// `self_align`, a dele; dentro dos limites) e o x (no começo, no meio ou no fim; passando
/// da área, encostado à esquerda e o resto cortado).
fn band(cb: &ControlBase, area: &RectF) -> (f32, f32) {
    let m = cb.margins();
    let available = (area.width - m.horizontal()).max(0.0);
    let w = cb.band_width(available);
    (area.x + m.left + cb.band_offset(available, w), w)
}

/// A faixa vertical de um controle à esquerda ou à direita: o y e a altura.
fn column(cb: &ControlBase, area: &RectF) -> (f32, f32) {
    let m = cb.margins();
    let available = (area.height - m.vertical()).max(0.0);
    let h = cb.band_height(available);
    (area.y + m.top + cb.band_offset(available, h), h)
}

/// Encaixa o controle numa borda da área livre e diminui a área.
fn dock(cb: &mut ControlBase, area: &mut RectF) {
    let m = cb.margins();
    match cb.align() {
        Align::Top | Align::MostTop => {
            let (x, w) = band(cb, area);
            let h = ratio_height(cb, w);
            cb.bounds = RectF::new(x, area.y + m.top, w, h);
            // Continua empilhando além da borda (o que passar fica fora da área, como no
            // FMX; um ScrollBox rola até lá).
            let used = h + m.vertical();
            area.y += used;
            area.height = (area.height - used).max(0.0);
        }
        Align::Bottom | Align::MostBottom => {
            let (x, w) = band(cb, area);
            let h = ratio_height(cb, w);
            cb.bounds = RectF::new(x, area.bottom() - m.bottom - h, w, h);
            area.height = (area.height - h - m.vertical()).max(0.0);
        }
        Align::Left | Align::MostLeft => {
            let (y, h) = column(cb, area);
            let w = ratio_width(cb, h);
            cb.bounds = RectF::new(area.x + m.left, y, w, h);
            let used = w + m.horizontal();
            area.x += used;
            area.width = (area.width - used).max(0.0);
        }
        Align::Right | Align::MostRight => {
            let (y, h) = column(cb, area);
            let w = ratio_width(cb, h);
            cb.bounds = RectF::new(area.right() - m.right - w, y, w, h);
            area.width = (area.width - w - m.horizontal()).max(0.0);
        }
        _ => {}
    }
}

/// Posiciona os alinhamentos que não consomem espaço.
fn place(cb: &mut ControlBase, area: RectF, full: RectF) {
    let m = cb.margins();
    let (w, h) = (cb.width(), cb.height());
    let inner = area.deflate(m);
    let ratio = cb.aspect_ratio();
    cb.bounds = match cb.align() {
        Align::Client | Align::Center if ratio > 0.0 => {
            // Cabe na área mantendo a proporção, no meio.
            let (fw, fh) = if inner.width / inner.height.max(0.01) > ratio { (inner.height * ratio, inner.height) } else { (inner.width, inner.width / ratio) };
            RectF::new(inner.x + (inner.width - fw) / 2.0, inner.y + (inner.height - fh) / 2.0, fw, fh)
        }
        Align::Client => {
            let w = cb.clamp_width(inner.width);
            RectF::new(inner.x + ((inner.width - w) / 2.0).max(0.0), inner.y, w, cb.clamp_height(inner.height))
        }
        Align::Contents => full.deflate(m),
        Align::Center => {
            let (w, h) = (cb.clamp_width(w), cb.clamp_height(h));
            RectF::new(inner.x + (inner.width - w) / 2.0, inner.y + (inner.height - h) / 2.0, w, h)
        }
        Align::VertCenter => {
            let w = cb.clamp_width(inner.width);
            let h = ratio_height(cb, w);
            RectF::new(inner.x, inner.y + (inner.height - h) / 2.0, w, h)
        }
        Align::HorzCenter => {
            let h = cb.clamp_height(inner.height);
            let w = ratio_width(cb, h);
            RectF::new(inner.x + (inner.width - w) / 2.0, inner.y, w, h)
        }
        Align::Horizontal => {
            let w = cb.clamp_width(inner.width);
            RectF::new(inner.x, cb.top(), w, ratio_height(cb, w))
        }
        Align::Vertical => {
            let h = cb.clamp_height(inner.height);
            RectF::new(cb.left(), inner.y, ratio_width(cb, h), h)
        }
        Align::Fit => {
            let scale = if w > 0.0 && h > 0.0 { (inner.width / w).min(inner.height / h) } else { 0.0 };
            let (fw, fh) = (w * scale, h * scale);
            RectF::new(inner.x + (inner.width - fw) / 2.0, inner.y + (inner.height - fh) / 2.0, fw, fh)
        }
        Align::None => {
            let r = anchored(cb, full.size());
            let (w, h) = (cb.clamp_width(r.width), cb.clamp_height(r.height));
            if ratio > 0.0 { RectF::new(r.x, r.y, w, w / ratio) } else { RectF::new(r.x, r.y, w, h) }
        }
        _ => cb.bounds,
    };
}

/// A altura do controle: a pedida, ou a que sai da largura com `aspect_ratio` (dentro da
/// `min_height`/`max_height`).
fn ratio_height(cb: &ControlBase, width: f32) -> f32 {
    let ratio = cb.aspect_ratio();
    cb.clamp_height(if ratio > 0.0 { (width.max(0.0) / ratio).max(0.0) } else { cb.height() })
}

/// A largura do controle: a pedida, ou a que sai da altura com `aspect_ratio` (dentro da
/// `min_width`/`max_width`).
fn ratio_width(cb: &ControlBase, height: f32) -> f32 {
    let ratio = cb.aspect_ratio();
    cb.clamp_width(if ratio > 0.0 { height.max(0.0) * ratio } else { cb.width() })
}

/// Posição de um controle com `Align::None`, considerando as âncoras.
fn anchored(cb: &mut ControlBase, parent: SizeF) -> RectF {
    let requested = RectF::new(cb.left(), cb.top(), cb.width(), cb.height());
    let anchors = cb.anchors();
    if anchors == Anchors::default() {
        return requested;
    }
    // Guarda o tamanho do pai na primeira vez; daí em diante, as âncoras aplicam a
    // diferença em relação a ele. Mudar posição/tamanho/âncoras zera a referência.
    let reference = *cb.anchor_ref.get_or_insert(AnchorRef { parent, rect: requested });
    let dw = parent.width - reference.parent.width;
    let dh = parent.height - reference.parent.height;
    let (x, width) = axis(
        reference.rect.x,
        reference.rect.width,
        dw,
        anchors.contains(Anchors::LEFT),
        anchors.contains(Anchors::RIGHT),
    );
    let (y, height) = axis(
        reference.rect.y,
        reference.rect.height,
        dh,
        anchors.contains(Anchors::TOP),
        anchors.contains(Anchors::BOTTOM),
    );
    RectF::new(x, y, width.max(0.0), height.max(0.0))
}

/// Um eixo das âncoras: (posição, tamanho) dado quanto o pai cresceu.
fn axis(pos: f32, len: f32, delta: f32, near: bool, far: bool) -> (f32, f32) {
    match (near, far) {
        (true, true) => (pos, len + delta),
        (false, true) => (pos + delta, len),
        (true, false) => (pos, len),
        (false, false) => (pos + delta / 2.0, len),
    }
}

impl Ui {
    /// Prende um controle flutuante do form (a lista de um combo, o calendário de um campo de
    /// data) ao controle que o abriu: a rolagem de um contêiner o leva junto, e ele some
    /// enquanto o alvo está fora da parte visível. Chame depois de pôr a posição dele.
    pub fn follow(&mut self, id: ComponentId, target: ComponentId) {
        if let Some(cb) = self.cb_mut(id) {
            cb.follow = Some((target, None));
        }
        self.invalidate();
    }

    /// A parte do controle que aparece (recortada pelos contêineres que recortam os filhos),
    /// relativa ao form; `None` se está toda escondida pela rolagem.
    pub fn shown_rect(&self, id: ComponentId) -> Option<RectF> {
        let mut rect = self.absolute_rect(id)?;
        let mut parent = self.parent(id);
        while let Some(p) = parent {
            let cb = self.cb(p)?;
            if !cb.visible() {
                return None;
            }
            if cb.clip_children() || self.parent(p).is_none() {
                rect = rect.intersect(self.absolute_rect(p)?)?;
            }
            parent = self.parent(p);
        }
        Some(rect)
    }

    fn place_followers(&mut self, root: ComponentId) {
        for child in self.children(root).to_vec() {
            let Some((target, at)) = self.cb(child).and_then(|cb| cb.follow) else { continue };
            let Some(t) = self.absolute_rect(target) else { continue };
            // A distância é a da posição pedida ao abrir (o alvo ainda onde estava).
            let at = at.unwrap_or_else(|| self.cb(child).map_or(PointF::ZERO, |cb| PointF::new(cb.left() - t.x, cb.top() - t.y)));
            let shown = self.shown_rect(target).is_some();
            if let Some(cb) = self.cb_mut(child) {
                let size = cb.bounds.size();
                cb.bounds = RectF::new(t.x + at.x, t.y + at.y, size.width, size.height);
                cb.set_visible(shown);
                cb.follow = Some((target, Some(at)));
            }
        }
    }
}

/// Os idiomas escritos da direita para a esquerda (árabe, hebraico, persa, urdu...).
pub fn is_right_to_left_language(code: &str) -> bool {
    let base = code.split(['-', '_']).next().unwrap_or("").to_lowercase();
    matches!(base.as_str(), "ar" | "he" | "iw" | "fa" | "ur" | "ps" | "sd" | "ug" | "yi" | "dv" | "ckb")
}

impl Ui {
    /// Espelha a interface (da direita para a esquerda, como em árabe): as posições, os
    /// encaixes (Left vira Right), o desenho de cada controle (a caixa do CheckBox à
    /// direita, a seta do ComboBox à esquerda; textos e imagens continuam legíveis) e o
    /// mouse. Os campos de texto ficam só no lugar espelhado (o cursor segue o texto).
    /// [`ControlBase::set_keep_left_to_right`] deixa um controle e os filhos como estão.
    pub fn set_right_to_left(&mut self, on: bool) {
        if self.rtl != on {
            self.rtl = on;
            self.invalidate();
        }
    }

    pub fn right_to_left(&self) -> bool {
        self.rtl
    }

    /// O controle está espelhado: a interface está e nem ele nem um pai pediu para ficar
    /// da esquerda para a direita.
    pub fn is_mirrored(&self, id: ComponentId) -> bool {
        if !self.rtl {
            return false;
        }
        let mut current = Some(id);
        while let Some(c) = current {
            if self.cb(c).is_some_and(|cb| cb.keep_left_to_right) {
                return false;
            }
            current = self.parent(c);
        }
        true
    }

    /// O desenho e o mouse do próprio controle espelhados (os campos de texto, não).
    pub(crate) fn flips_content(&self, id: ComponentId) -> bool {
        self.is_mirrored(id) && !self.component(id).is_some_and(|c| c.accepts_text())
    }
}
