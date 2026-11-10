//! Repintar só o que mudou: um controle que muda o próprio desenho marca a área dele
//! ([`Ui::damage`], que o `ctx.repaint()` chama), e a plataforma repinta só ela, por cima da
//! imagem do quadro anterior (quando o renderizador guarda essa imagem). Na dúvida, a janela
//! inteira: [`Ui::request_repaint`] e [`Ui::invalidate`] (o layout mudou), um controle girado
//! ou espelhado, que desenha por cima de tudo, ou num form que não está numa janela (o do
//! designer). Um controle deslocado (`translate`) repinta o lugar novo e o antigo, dele e
//! dos filhos.

use crate::component::ComponentId;
use crate::geometry::RectF;
use crate::ui::Ui;

/// A folga além dos efeitos: o anti-aliasing da borda e o anel de foco, que passam um pouco.
const EDGE: f32 = 3.0;

/// Quantas marcas do fim da lista o [`push_recent`] confere.
const RECENT: usize = 16;

/// Anota `id` na lista do quadro, se ele não está entre as últimas marcas. Sem quadro no
/// meio (o relatório que monta milhares de páginas num evento, mudando e voltando o texto
/// dos mesmos rótulos), a lista crescia com cada mudança; os poucos controles que se
/// revezam ficam no fim e não entram de novo. O custo é fixo (não procura na lista
/// inteira); repetidos mais antigos o quadro ainda tira ([`Ui::take_frame`]).
fn push_recent(list: &mut Vec<ComponentId>, id: ComponentId) {
    if !list.iter().rev().take(RECENT).any(|&x| x == id) {
        list.push(id);
    }
}

impl Ui {
    /// Até onde o desenho do controle vai, nas coordenadas do form: o retângulo dele com a
    /// sombra mais larga dos estados dele, mais onde ele desenhou na última pintura
    /// ([`crate::ink`]). Ainda não pintado (fora da vista até agora), só o retângulo: não há
    /// desenho velho para apagar, e se ele desenhar além a pintura percebe e o próximo
    /// quadro é inteiro.
    fn extent(&self, id: ComponentId, rect: RectF) -> Option<RectF> {
        let visual = self.entry(id)?.visual.borrow();
        let r = rect.inflate(visual.reach.unwrap_or(0.0) + EDGE);
        Some(visual.ink.map_or(r, |ink| r.union(ink.inflate(EDGE))))
    }

    /// O desenho do controle `id` mudou (e só o dele, dentro da área dele): a próxima pintura
    /// pode repintar só essa parte da janela. Só anota: a área sai na hora do quadro
    /// ([`Ui::take_damage`]), quando o controle não está ocupado no meio de um evento dele.
    pub fn damage(&mut self, id: ComponentId) {
        self.repaint_requested = true;
        push_recent(&mut self.frame_ids.borrow_mut(), id);
    }

    /// Como [`Ui::damage`], quando só o lugar ou a transparência dele mudou (`translate`,
    /// `opacity`): a área é repintada, mas a camada guardada dele (`cache_layer`) continua
    /// valendo — só a dos ancestrais, que o mostram em outro lugar, é refeita.
    pub fn damage_placement(&mut self, id: ComponentId) {
        self.repaint_requested = true;
        push_recent(&mut self.placed_ids.borrow_mut(), id);
    }

    /// O desenho de dentro de `id` (o próprio, ou o de um descendente) mudou: a camada
    /// guardada dele e a de cada ancestral não valem mais.
    fn bump_versions(&self, id: ComponentId, itself: bool) {
        let mut current = if itself { Some(id) } else { self.parent(id) };
        while let Some(c) = current {
            if let Some(entry) = self.entry(c) {
                entry.visual.borrow_mut().version += 1;
            }
            current = self.parent(c);
        }
    }

    /// A área do controle no form, se dá para repintar só ela. Deslocado (`translate`, uma
    /// animação), a área é a do lugar onde ele está agora; girado, a janela inteira.
    fn damage_rect(&self, id: ComponentId) -> Option<RectF> {
        let root = self.root_of(id);
        if self.overlay_users.borrow().contains(&id) || !self.open_forms().contains(&root) || self.flips_content(id) {
            return None;
        }
        let mut shift = crate::PointF::ZERO;
        let mut current = Some(id);
        while let Some(c) = current {
            let cb = self.control(c)?;
            if cb.rotation() != 0.0 {
                return None;
            }
            let t = cb.translate();
            shift = crate::PointF::new(shift.x + t.x, shift.y + t.y);
            current = self.parent(c);
        }
        self.absolute_rect(id).map(|r| r.offset(shift.x, shift.y))
    }

    /// Onde os descendentes de `id` desenharam na última pintura: quem se desloca leva os
    /// filhos junto, e o lugar antigo deles também precisa ser repintado.
    fn subtree_ink(&self, id: ComponentId) -> Option<RectF> {
        let mut ink: Option<RectF> = None;
        let mut stack: Vec<ComponentId> = self.children(id).to_vec();
        while let Some(c) = stack.pop() {
            if let Some(entry) = self.entry(c) {
                if let Some(r) = entry.visual.borrow().ink {
                    ink = Some(ink.map_or(r, |i| i.union(r)));
                }
            }
            stack.extend_from_slice(self.children(c));
        }
        ink
    }

    /// Só a parte `rect` de `id` mudou (nas coordenadas dele): a barra de rolagem que andou,
    /// o cursor de texto. Junto com [`Ui::scroll_damage`], a faixa nova e a barra são pintadas
    /// sem repintar o controle inteiro.
    pub fn damage_part(&mut self, id: ComponentId, rect: RectF) {
        self.repaint_requested = true;
        self.part_ids.borrow_mut().push((id, rect));
    }

    /// O conteúdo de `rect` de `id` (nas coordenadas dele) andou `delta` desde a última
    /// pintura (a rolagem: `delta.y` negativo quando o texto sobe). Se der, a plataforma
    /// copia os pixels já pintados e só a faixa que apareceu é pintada ([`Ui::take_frame`]);
    /// senão, `rect` inteiro é repintado. O que muda fora de `rect` (a barra) o controle marca
    /// com [`Ui::damage_part`].
    pub fn scroll_damage(&mut self, id: ComponentId, rect: RectF, delta: crate::PointF) {
        // Não andou (a rolagem já no fim): nada a copiar nem a pintar.
        if delta.x == 0.0 && delta.y == 0.0 {
            return;
        }
        self.repaint_requested = true;
        self.scroll_moves.borrow_mut().push((id, rect, delta));
    }

    /// Onde os pixels de `rect` (de `id`, nas coordenadas dele) podem ser copiados para andar
    /// com a rolagem, nas coordenadas do form: o pedaço que aparece (recortado pelos
    /// ancestrais e pelo form), se nada é pintado por cima dele e se o fundo do controle é
    /// opaco e parado. `None`: repinta.
    fn movable(&self, root: ComponentId, id: ComponentId, rect: RectF) -> Option<RectF> {
        let abs = self.damage_rect(id)?;
        let entry = self.entry(id)?;
        let cb = self.control(id)?;
        {
            let v = entry.visual.borrow();
            let style = v.shown.as_ref()?;
            if v.shift != cb.translate() || style.fill.a != 255 || style.material != crate::theme::Material::None || style.opacity < 1.0 || cb.opacity() < 1.0 {
                return None;
            }
        }
        // Nada por cima: os filhos dele, e os irmãos pintados depois dele e de cada ancestral.
        let mut region = rect.offset(abs.x, abs.y);
        let covers = |c: ComponentId, region: RectF| {
            self.control(c).is_some_and(|ccb| ccb.visible()) && self.damage_rect(c).and_then(|r| self.extent(c, r)).is_none_or(|r| r.intersects(region))
        };
        if self.children(id).iter().any(|&c| covers(c, region)) {
            return None;
        }
        let mut node = id;
        while let Some(parent) = self.parent(node) {
            let pcb = self.control(parent)?;
            if pcb.opacity() < 1.0 {
                return None;
            }
            if pcb.clip_children() {
                region = intersection(region, self.damage_rect(parent)?)?;
            }
            let siblings = self.children(parent);
            let at = siblings.iter().position(|&c| c == node)?;
            if siblings[at + 1..].iter().any(|&c| covers(c, region)) {
                return None;
            }
            node = parent;
        }
        let form = self.control(root).map(|f| RectF::new(0.0, 0.0, f.width(), f.height()))?;
        intersection(region, form)
    }

    /// O que repintar no form `root` no próximo quadro (nas coordenadas dele). Zera a conta
    /// (a plataforma chama uma vez por quadro). Com [`FrameDamage::Whole`], ou quando a imagem
    /// anterior não está guardada, a janela inteira.
    pub fn take_frame(&mut self, root: ComponentId) -> FrameDamage {
        // O layout pendente antes: ele marca quem mudou de lugar.
        self.update_layout();
        // O que mudou por dentro, o que só mudou de lugar ou de transparência, as partes e as
        // rolagens.
        let mut content = std::mem::take(&mut *self.frame_ids.borrow_mut());
        let mut placed = std::mem::take(&mut *self.placed_ids.borrow_mut());
        // Sem repetidos: um controle marcado várias vezes no quadro conta uma (e as marcas dos
        // forms que não foram pintados, que voltam para a fila, não se acumulam).
        for list in [&mut content, &mut placed] {
            let mut seen = std::collections::HashSet::with_capacity(list.len());
            list.retain(|id| seen.insert(*id));
        }
        let parts = std::mem::take(&mut *self.part_ids.borrow_mut());
        let scrolls = std::mem::take(&mut *self.scroll_moves.borrow_mut());
        let anything = !(content.is_empty() && placed.is_empty() && parts.is_empty() && scrolls.is_empty());
        let mut areas = Vec::new();
        let mut moves = Vec::new();
        let mut whole = false;
        let (mut others, mut others_placed, mut other_parts, mut other_scrolls) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        // Os deste form que mudaram por dentro (uma rolagem deles não vira cópia).
        let mut changed: Vec<ComponentId> = Vec::new();
        for (id, inside) in content.into_iter().map(|id| (id, true)).chain(placed.into_iter().map(|id| (id, false))) {
            if !self.contains(id) {
                continue;
            }
            if self.root_of(id) != root {
                if inside { others.push(id) } else { others_placed.push(id) }
                continue;
            }
            if inside {
                changed.push(id);
            }
            self.bump_versions(id, inside);
            match self.damage_rect(id).and_then(|r| self.extent(id, r)) {
                Some(mut r) => {
                    // O deslocamento mudou desde a última pintura: ele e os filhos saem do
                    // lugar antigo, e o desenho no lugar novo é o antigo andado da diferença
                    // (o que passa da borda também anda).
                    let now = self.control(id).map_or(crate::PointF::ZERO, |cb| cb.translate());
                    let (before, own) = self.entry(id).map_or((now, None), |e| {
                        let v = e.visual.borrow();
                        (v.shift, v.ink)
                    });
                    if now != before {
                        let (dx, dy) = (now.x - before.x, now.y - before.y);
                        for ink in [own, self.subtree_ink(id)].into_iter().flatten() {
                            r = r.union(ink.inflate(EDGE)).union(ink.offset(dx, dy).inflate(EDGE));
                        }
                    }
                    areas.push(r);
                }
                None => whole = true,
            }
        }
        for (id, rect) in parts {
            if !self.contains(id) {
                continue;
            }
            if self.root_of(id) != root {
                other_parts.push((id, rect));
                continue;
            }
            self.bump_versions(id, true);
            match self.damage_rect(id) {
                Some(abs) => areas.push(rect.offset(abs.x, abs.y).inflate(EDGE)),
                None => whole = true,
            }
        }
        // As rolagens: as de um mesmo controle no quadro se somam.
        let mut scrolled: Vec<(ComponentId, RectF, crate::PointF)> = Vec::new();
        for (id, rect, delta) in scrolls {
            if !self.contains(id) {
                continue;
            }
            if self.root_of(id) != root {
                other_scrolls.push((id, rect, delta));
                continue;
            }
            match scrolled.iter_mut().find(|(i, _, _)| *i == id) {
                Some(s) => {
                    s.1 = rect;
                    s.2 = crate::PointF::new(s.2.x + delta.x, s.2.y + delta.y);
                }
                None => scrolled.push((id, rect, delta)),
            }
        }
        // Algo por cima de tudo (a dica aberta, o arrasto): nada se copia.
        let floating = self.visible_hint(root).is_some() || self.drag_view(root).is_some();
        for (id, rect, d) in scrolled {
            self.bump_versions(id, true);
            let region = if floating || changed.contains(&id) { None } else { self.movable(root, id, rect) };
            match region {
                Some(r) if d.x.abs() < r.width && d.y.abs() < r.height && (d.x != 0.0 || d.y != 0.0) => {
                    // O que estava em `dest - d` vai para `dest`; a faixa que sobra aparece.
                    let Some(dest) = intersection(r.offset(d.x, d.y), r) else {
                        areas.push(r);
                        continue;
                    };
                    moves.push(ScrollMove { rect: dest.offset(-d.x, -d.y), delta: d });
                    if d.y > 0.0 {
                        areas.push(RectF::new(r.x, r.y, r.width, d.y));
                    } else if d.y < 0.0 {
                        areas.push(RectF::new(r.x, r.bottom() + d.y, r.width, -d.y));
                    }
                    if d.x > 0.0 {
                        areas.push(RectF::new(r.x, r.y, d.x, r.height));
                    } else if d.x < 0.0 {
                        areas.push(RectF::new(r.right() + d.x, r.y, -d.x, r.height));
                    }
                }
                Some(r) => areas.push(r),
                None => match self.damage_rect(id) {
                    Some(abs) => areas.push(rect.offset(abs.x, abs.y).inflate(EDGE)),
                    None => whole = true,
                },
            }
        }
        // As dos outros forms ficam para a pintura deles.
        self.frame_ids.borrow_mut().extend(others);
        self.placed_ids.borrow_mut().extend(others_placed);
        self.part_ids.borrow_mut().extend(other_parts);
        self.scroll_moves.borrow_mut().extend(other_scrolls);
        let epoch = self.damage_epoch;
        let full = self.painted_epoch.insert(root, epoch) != Some(epoch);
        if full || whole || !anything {
            return FrameDamage::Whole;
        }
        let areas = merge_areas(areas);
        if moves.is_empty() && areas.is_empty() {
            return FrameDamage::Whole;
        }
        FrameDamage::Parts { moves, areas }
    }

    /// A área a repintar do form `root` na próxima pintura (nas coordenadas dele), num
    /// retângulo só (as rolagens contam a origem e o destino): `None` é a janela inteira.
    /// Veja [`Ui::take_frame`], que separa o que dá para copiar.
    pub fn take_damage(&mut self, root: ComponentId) -> Option<RectF> {
        self.take_frame(root).bounds()
    }
}

/// Pixels já pintados que andam (a rolagem): os de `rect` (nas coordenadas do form) vão para
/// `rect` deslocado de `delta`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollMove {
    pub rect: RectF,
    pub delta: crate::PointF,
}

/// O que repintar num quadro ([`Ui::take_frame`]).
#[derive(Clone, Debug, PartialEq)]
pub enum FrameDamage {
    /// A janela inteira.
    Whole,
    /// Por cima do quadro anterior: primeiro as cópias (na ordem), depois cada área pintada
    /// com o recorte nela.
    Parts { moves: Vec<ScrollMove>, areas: Vec<RectF> },
}

impl FrameDamage {
    /// Tudo o que muda, num retângulo (as cópias contam a origem e o destino). `None`: a
    /// janela inteira.
    pub fn bounds(&self) -> Option<RectF> {
        match self {
            FrameDamage::Whole => None,
            FrameDamage::Parts { moves, areas } => {
                let all = moves.iter().flat_map(|m| [m.rect, m.rect.offset(m.delta.x, m.delta.y)]).chain(areas.iter().copied());
                all.reduce(|a, b| a.union(b))
            }
        }
    }

    /// Só as cópias que caem em pixels inteiros na escala `scale` (a região e o
    /// deslocamento): as outras viram área a pintar (a origem e o destino).
    pub fn snapped(self, scale: f32) -> FrameDamage {
        let FrameDamage::Parts { moves, mut areas } = self else { return self };
        let whole = |v: f32| ((v * scale) - (v * scale).round()).abs() < 0.01;
        let mut kept = Vec::new();
        for m in moves {
            let r = m.rect;
            if [r.x, r.y, r.right(), r.bottom(), m.delta.x, m.delta.y].into_iter().all(whole) {
                kept.push(m);
            } else {
                areas.push(r.union(r.offset(m.delta.x, m.delta.y)));
            }
        }
        FrameDamage::Parts { moves: kept, areas }
    }

    /// Sem as cópias (o renderizador não guarda o quadro, ou não sabe copiar): cada uma vira
    /// a área da origem e do destino.
    pub fn without_moves(self) -> FrameDamage {
        let FrameDamage::Parts { moves, mut areas } = self else { return self };
        areas.extend(moves.iter().map(|m| m.rect.union(m.rect.offset(m.delta.x, m.delta.y))));
        FrameDamage::Parts { moves: Vec::new(), areas }
    }
}

/// A interseção de dois retângulos (`None` se não se tocam).
fn intersection(a: RectF, b: RectF) -> Option<RectF> {
    let (x, y) = (a.x.max(b.x), a.y.max(b.y));
    let (r, bottom) = (a.right().min(b.right()), a.bottom().min(b.bottom()));
    (r > x && bottom > y).then(|| RectF::new(x, y, r - x, bottom - y))
}

/// Junta as áreas cuja união quase não passa da soma delas: menos áreas para
/// pintar, sem pintar muito a mais (60 barras espalhadas numa tela gigante não viram o
/// retângulo que contém todas). Muitas áreas: uma só, a que contém todas.
fn merge_areas(mut rects: Vec<RectF>) -> Vec<RectF> {
    if rects.len() > 256 {
        return rects.into_iter().reduce(|a, b| a.union(b)).into_iter().collect();
    }
    let size = |r: RectF| r.width.max(0.0) * r.height.max(0.0);
    loop {
        let mut joined = None;
        'search: for i in 0..rects.len() {
            for j in i + 1..rects.len() {
                let (a, b) = (rects[i], rects[j]);
                let u = a.union(b);
                // Tocar não basta: duas faixas finas que se cruzam no canto (as beiradas de um
                // Memo) virariam o retângulo inteiro; o recorte já é a união exata delas.
                if size(u) <= (size(a) + size(b)) * 1.25 {
                    joined = Some((i, j, u));
                    break 'search;
                }
            }
        }
        match joined {
            Some((i, j, u)) => {
                rects[i] = u;
                rects.swap_remove(j);
            }
            None => return rects,
        }
    }
}
