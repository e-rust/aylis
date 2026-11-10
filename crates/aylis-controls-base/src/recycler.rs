//! [`VirtualList`]: lista de itens desenhados com um modelo seu (como o `RecyclerView` do
//! Android ou o `TDBCtrlGrid` do Delphi). Só existem as linhas visíveis (e mais duas): ao
//! rolar, as que saem por um lado são reaproveitadas para os itens que entram pelo outro,
//! então uma lista de um milhão de itens custa o mesmo que uma de vinte.
//!
//! O modelo é o primeiro filho da lista (desenhado no próprio designer: um painel com
//! imagem, textos, botões...) ou um form inteiro do projeto (`item_form`). Cada linha é
//! uma cópia dele. Ao mostrar o item `i` numa linha, a lista preenche os componentes de
//! `bindings` com os valores da fonte (se houver) e dispara `on_bind` com `i`: o handler
//! acha a linha com [`VirtualList::item`] e preenche o resto. Perto do fim, `on_load_more`
//! avisa (uma vez por tamanho) para a lista infinita: o handler busca mais e aumenta
//! `item_count`.
//!
//! Formatos: lista vertical (o normal) ou horizontal (`orientation`), e grade com `columns`
//! itens por linha (ou por coluna, na horizontal). O tamanho de cada item pode variar: `sizes`
//! (`80;120;80`, o resto com `item_height`) ou, pelo código, [`VirtualList::set_sizer`]; as
//! posições ficam somadas, e achar o primeiro item à vista é uma busca binária.

use std::cell::RefCell;

use aylis_core::{ArrangeItem, Component, ComponentId, ControlBase, EventArgs, EventCtx, PropEnum, PropValue, RectF, Ui, WheelEvent, properties};

use crate::leaf_control;

/// De onde vêm os valores dos itens (um dataset, uma lista em memória...).
pub trait RowSource {
    /// Quantos itens.
    fn count(&self, ui: &Ui) -> usize;
    /// O valor do campo `field` no item `row`, como texto.
    fn value(&self, ui: &Ui, row: usize, field: &str) -> Option<String>;
}

/// Linhas a mais além das visíveis (as que estão entrando na tela).
const SPARE: usize = 2;
/// A quantos itens do fim `on_load_more` avisa.
const LOAD_MORE_AHEAD: usize = 5;

/// Para onde a lista corre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl PropEnum for ListOrientation {
    const NAMES: &'static [&'static str] = &["vertical", "horizontal"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Vertical, Self::Horizontal].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O tamanho de cada item, pelo código (variável).
pub type Sizer = Box<dyn Fn(usize) -> f32>;

/// Lista virtual com modelo de item e reciclagem.
pub struct VirtualList {
    pub base: ControlBase,
    /// Quantos itens (com uma fonte, vem dela).
    pub item_count: i64,
    pub item_height: f32,
    /// Na horizontal: a largura de cada item.
    pub item_width: f32,
    pub orientation: ListOrientation,
    /// Grade: quantos itens lado a lado (na horizontal, um embaixo do outro). 1 = lista.
    pub columns: i64,
    /// Tamanhos próprios dos itens, em ordem (`80;120;80`; os que faltam usam `item_height`
    /// ou `item_width`). Só na lista (uma coluna).
    pub sizes: String,
    /// Um form do projeto como modelo (vazio: o primeiro filho da lista).
    pub item_form: String,
    /// `"lblNome=nome;imgFoto=foto"`: componente do modelo = campo da fonte.
    pub bindings: String,
    sizer: Option<Sizer>,
    /// Começo de cada linha somado (com tamanhos variáveis), e para quantos itens.
    starts: RefCell<(usize, Vec<f32>)>,
    /// A quantidade de itens na última arrumação (o `arrange` não tem o `Ui`).
    known_count: std::cell::Cell<usize>,
    offset: f32,
    template: Option<ComponentId>,
    /// As linhas: (instância, item mostrado).
    pool: RefCell<Vec<(ComponentId, Option<usize>)>>,
    source: Option<Box<dyn RowSource>>,
    /// `on_load_more` já avisou com esta quantidade.
    load_more_at: Option<usize>,
}

impl Default for VirtualList {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 400.0);
        base.set_clip_children(true);
        Self {
            base,
            item_count: 0,
            item_height: 64.0,
            item_width: 160.0,
            orientation: ListOrientation::Vertical,
            columns: 1,
            sizes: String::new(),
            item_form: String::new(),
            bindings: String::new(),
            sizer: None,
            starts: RefCell::new((usize::MAX, Vec::new())),
            known_count: std::cell::Cell::new(0),
            offset: 0.0,
            template: None,
            pool: RefCell::default(),
            source: None,
            load_more_at: None,
        }
    }
}

impl VirtualList {
    /// A linha que mostra o item `index` agora (`None` se ele não está à vista).
    pub fn item(ui: &Ui, list: ComponentId, index: usize) -> Option<ComponentId> {
        ui.get::<VirtualList>(list)?.pool.borrow().iter().find(|(_, i)| *i == Some(index)).map(|(id, _)| *id)
    }

    /// Os itens à vista agora, em ordem.
    pub fn visible(ui: &Ui, list: ComponentId) -> Vec<usize> {
        let Some(l) = ui.get::<VirtualList>(list) else { return Vec::new() };
        let mut out: Vec<usize> = l.pool.borrow().iter().filter_map(|(_, i)| *i).collect();
        out.sort_unstable();
        out
    }

    /// Liga uma fonte de valores (os `bindings` e a quantidade vêm dela).
    pub fn set_source(ui: &mut Ui, list: ComponentId, source: Box<dyn RowSource>) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            l.source = Some(source);
        }
        Self::refresh(ui, list);
    }

    /// Mostra de novo todos os itens à vista (os dados mudaram).
    pub fn refresh(ui: &mut Ui, list: ComponentId) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            for (_, index) in l.pool.borrow_mut().iter_mut() {
                *index = None;
            }
        }
        ui.send_message(list, &Refresh);
    }

    /// Rola até o item `index` ficar no topo.
    pub fn scroll_to(ui: &mut Ui, list: ComponentId, index: usize) {
        ui.send_message(list, &ScrollTo(index));
    }

    /// Troca a fonte (para componentes que embrulham a lista, como a `DataRepeater`):
    /// as linhas são preenchidas de novo no próximo [`resync`](Self::resync).
    pub fn set_row_source(&mut self, source: Option<Box<dyn RowSource>>) {
        self.source = source;
        self.load_more_at = None;
        for (_, index) in self.pool.borrow_mut().iter_mut() {
            *index = None;
        }
    }

    /// Esquece o que cada linha mostra e preenche de novo (os dados mudaram).
    pub fn resync(&mut self, ctx: &mut EventCtx<'_>) {
        for (_, index) in self.pool.borrow_mut().iter_mut() {
            *index = None;
        }
        self.sync(ctx);
    }

    /// Tamanho variável por código: `size(i)` é a altura (na horizontal, a largura) do item
    /// `i`. Só na lista (uma coluna).
    pub fn set_sizer(ui: &mut Ui, list: ComponentId, size: impl Fn(usize) -> f32 + 'static) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            l.sizer = Some(Box::new(size));
            l.starts.borrow_mut().0 = usize::MAX;
        }
        Self::refresh(ui, list);
    }

    /// Onde começa o item `index` e o tamanho dele, no sentido da lista (para rolar até ele,
    /// para testes).
    pub fn item_span(ui: &Ui, list: ComponentId, index: usize) -> Option<(f32, f32)> {
        let l = ui.get::<VirtualList>(list)?;
        let count = l.count(ui);
        let line = index / l.lanes();
        Some((l.line_start(line, count), l.line_size(line)))
    }

    /// Os itens lado a lado (as colunas da grade).
    fn lanes(&self) -> usize {
        self.columns.max(1) as usize
    }

    fn horizontal(&self) -> bool {
        self.orientation == ListOrientation::Horizontal
    }

    /// O tamanho de um item sem tamanho próprio.
    fn uniform(&self) -> f32 {
        if self.horizontal() { self.item_width.max(8.0) } else { self.item_height.max(8.0) }
    }

    /// Os tamanhos variam (lista com `sizes` ou com o tamanho pelo código).
    fn variable(&self) -> bool {
        self.lanes() == 1 && (self.sizer.is_some() || !self.sizes.trim().is_empty())
    }

    /// O tamanho da linha `line` (na grade, todas iguais).
    fn line_size(&self, line: usize) -> f32 {
        if !self.variable() {
            return self.uniform();
        }
        if let Some(s) = &self.sizer {
            return s(line).max(1.0);
        }
        self.sizes.split(';').nth(line).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or_else(|| self.uniform()).max(1.0)
    }

    fn lines(&self, count: usize) -> usize {
        count.div_ceil(self.lanes())
    }

    /// Os começos das linhas somados (refeitos quando a quantidade muda).
    fn ensure_starts(&self, count: usize) {
        let lines = self.lines(count);
        let mut starts = self.starts.borrow_mut();
        if starts.0 == lines {
            return;
        }
        let mut v = Vec::with_capacity(lines + 1);
        let mut at = 0.0;
        for line in 0..lines {
            v.push(at);
            at += self.line_size(line);
        }
        v.push(at);
        *starts = (lines, v);
    }

    /// Onde começa a linha `line`.
    fn line_start(&self, line: usize, count: usize) -> f32 {
        if !self.variable() {
            return line as f32 * self.uniform();
        }
        self.ensure_starts(count);
        let starts = self.starts.borrow();
        starts.1.get(line).or(starts.1.last()).copied().unwrap_or(0.0)
    }

    /// A linha em `pos` (no sentido da lista).
    fn line_at(&self, pos: f32, count: usize) -> usize {
        if !self.variable() {
            return (pos / self.uniform()).floor().max(0.0) as usize;
        }
        self.ensure_starts(count);
        let starts = self.starts.borrow();
        starts.1.partition_point(|s| *s <= pos).saturating_sub(1)
    }

    /// O tamanho da área à vista, no sentido da lista.
    fn viewport(&self) -> f32 {
        if self.horizontal() { self.base.width() } else { self.base.height() }
    }

    fn count(&self, ui: &Ui) -> usize {
        match &self.source {
            Some(s) => s.count(ui),
            None => self.item_count.max(0) as usize,
        }
    }

    fn max_offset(&self, count: usize) -> f32 {
        (self.line_start(self.lines(count), count) - self.viewport()).max(0.0)
    }

    /// Acha (ou cria) o modelo e esconde: as linhas são cópias dele.
    fn ensure_template(&mut self, ctx: &mut EventCtx<'_>) -> Option<ComponentId> {
        if self.template.is_some_and(|t| ctx.ui.contains(t)) {
            return self.template;
        }
        let pool: Vec<ComponentId> = self.pool.borrow().iter().map(|(id, _)| *id).collect();
        let template = if self.item_form.is_empty() {
            ctx.ui.children(ctx.id).iter().copied().find(|c| !pool.contains(c))?
        } else {
            let form = ctx.ui.create_form(&self.item_form).ok()?;
            ctx.ui.clone_component(form, Some(ctx.id)).ok()?
        };
        if let Some(cb) = ctx.ui.control_mut(template) {
            cb.set_visible(false);
        }
        self.template = Some(template);
        Some(template)
    }

    /// Põe as linhas nos itens à vista, criando as que faltam e reaproveitando as que
    /// saíram. Os itens que mudaram de linha recebem os valores e o `on_bind`.
    fn sync(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return; // no designer, o modelo aparece como foi desenhado
        }
        let Some(template) = self.ensure_template(ctx) else { return };
        let count = self.count(ctx.ui);
        self.offset = self.offset.clamp(0.0, self.max_offset(count));
        self.known_count.set(count);
        // As linhas à vista (e mais algumas), e os itens delas.
        let first = self.line_at(self.offset, count);
        let last = self.line_at((self.offset + self.viewport() - 0.01).max(self.offset), count) + SPARE;
        let lanes = self.lanes();
        let wanted: Vec<usize> = (first * lanes..((last + 1) * lanes).min(count)).collect();

        // Linhas que bastam.
        while self.pool.borrow().len() < wanted.len() {
            let Ok(copy) = ctx.ui.clone_component(template, Some(ctx.id)) else { break };
            if let Some(cb) = ctx.ui.control_mut(copy) {
                cb.set_visible(true);
            }
            self.pool.borrow_mut().push((copy, None));
        }
        // As que continuam no lugar ficam; as outras vão para os itens que faltam.
        let mut fresh = Vec::new();
        {
            let mut pool = self.pool.borrow_mut();
            let kept: Vec<usize> = pool.iter().filter_map(|(_, i)| i.filter(|i| wanted.contains(i))).collect();
            let mut missing = wanted.iter().copied().filter(|i| !kept.contains(i));
            for (id, index) in pool.iter_mut() {
                if index.is_some_and(|i| wanted.contains(&i)) {
                    continue;
                }
                *index = missing.next();
                if let Some(i) = *index {
                    fresh.push((*id, i));
                }
            }
        }
        for (id, _) in self.pool.borrow().iter() {
            let shown = self.pool.borrow().iter().any(|(p, i)| p == id && i.is_some());
            if let Some(cb) = ctx.ui.control_mut(*id) {
                cb.set_visible(shown);
            }
        }
        for (id, index) in fresh {
            self.bind(ctx, id, index);
            ctx.fire("on_bind", EventArgs::Value(PropValue::Int(index as i64)));
        }
        // Lista infinita: perto do fim, pede mais (uma vez por quantidade).
        if count > 0 && wanted.last().is_some_and(|&last| last + LOAD_MORE_AHEAD >= count) && self.load_more_at != Some(count) {
            self.load_more_at = Some(count);
            ctx.fire("on_load_more", EventArgs::Value(PropValue::Int(count as i64)));
        }
        ctx.ui.invalidate();
    }

    /// Preenche os componentes de `bindings` da linha com os valores do item.
    fn bind(&self, ctx: &mut EventCtx<'_>, row: ComponentId, index: usize) {
        let Some(source) = &self.source else { return };
        for pair in self.bindings.split(';') {
            let Some((component, field)) = pair.split_once('=') else { continue };
            let Some(target) = ctx.ui.find_in(row, component.trim()) else { continue };
            let value = source.value(ctx.ui, index, field.trim()).unwrap_or_default();
            // O texto, ou a imagem, ou o valor: a primeira que o componente tiver.
            for prop in ["text", "source", "value", "checked"] {
                if ctx.ui.prop_info(target, prop).is_some() {
                    let _ = ctx.ui.set_prop(target, prop, value.as_str());
                    break;
                }
            }
        }
    }
}

/// Mensagem interna: refazer as linhas (dados novos).
struct Refresh;
/// Mensagem interna: rolar até um item.
struct ScrollTo(usize);

impl Component for VirtualList {
    leaf_control!("VirtualList");

    properties! {
        item_count: i64 => "Behavior",
        item_height: f32 => "Appearance",
        item_width: f32 => "Appearance",
        orientation: ListOrientation => "Layout",
        columns: i64 => "Layout",
        sizes: String => "Appearance",
        item_form: String => "Behavior",
        bindings: String => "Behavior";
        events: on_bind, on_load_more
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "item_form" => {
                // Outro modelo: as linhas são refeitas.
                for (id, _) in self.pool.take() {
                    ctx.ui.free(id);
                }
                if let Some(t) = self.template.take().filter(|_| !self.item_form.is_empty()) {
                    ctx.ui.free(t);
                }
                self.sync(ctx);
            }
            "item_count" | "item_height" | "item_width" | "orientation" | "columns" | "sizes" => {
                // As posições somadas são refeitas.
                self.starts.borrow_mut().0 = usize::MAX;
                if name == "item_count" && self.count(ctx.ui) < self.load_more_at.unwrap_or(0) {
                    self.load_more_at = None; // a lista diminuiu (filtro, recarga)
                }
                // Antes da primeira medida (carregando o layout) as linhas esperam: os
                // handlers ainda estão sendo ligados.
                if !self.pool.borrow().is_empty() {
                    self.sync(ctx);
                }
            }
            _ => {}
        }
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        self.sync(ctx);
    }

    fn scroll_axes(&self) -> (bool, bool) {
        (self.horizontal(), !self.horizontal())
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let max = self.max_offset(self.count(ctx.ui));
        // Na horizontal, a roda comum (vertical) também anda.
        let delta = if self.horizontal() && event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
        let target = (self.offset - delta).clamp(0.0, max);
        if (target - self.offset).abs() < 0.01 {
            return; // no limite: a rolagem passa para os pais
        }
        self.offset = target;
        self.sync(ctx);
        ctx.set_handled();
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn std::any::Any) {
        if message.is::<Refresh>() {
            self.sync(ctx);
        } else if let Some(ScrollTo(index)) = message.downcast_ref::<ScrollTo>() {
            let count = self.count(ctx.ui);
            self.offset = self.line_start(*index / self.lanes(), count);
            self.sync(ctx);
        }
    }

    /// Cada item no lugar dele (a linha e, na grade, a coluna); o modelo e as linhas sem
    /// item ficam fora.
    fn arrange(&self, area: RectF, children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        let pool = self.pool.borrow();
        if pool.is_empty() {
            return None; // ainda sem linhas (designer, antes do primeiro sync): o normal
        }
        let lanes = self.lanes();
        let count = self.known_count.get();
        let cross_total = if self.horizontal() { area.height } else { area.width };
        let lane_size = cross_total / lanes as f32;
        Some(
            children
                .iter()
                .map(|c| match pool.iter().find(|(id, _)| *id == c.id).and_then(|(_, i)| *i) {
                    Some(i) => {
                        let (line, lane) = (i / lanes, i % lanes);
                        let main = self.line_start(line, count) - self.offset;
                        let size = self.line_size(line);
                        let cross = lane as f32 * lane_size;
                        let m = c.margins;
                        // O lugar inteiro do item, menos as margens do modelo.
                        if self.horizontal() {
                            RectF::new(area.x + main + m.left, area.y + cross + m.top, (size - m.left - m.right).max(0.0), (lane_size - m.top - m.bottom).max(0.0))
                        } else {
                            RectF::new(area.x + cross + m.left, area.y + main + m.top, (lane_size - m.left - m.right).max(0.0), (size - m.top - m.bottom).max(0.0))
                        }
                    }
                    None => RectF::new(area.x, area.y, 0.0, 0.0),
                })
                .collect(),
        )
    }
}
