//! TabStrip: a faixa de abas do navegador (como as do Edge). A aba ativa tem os cantos de
//! cima arredondados e se junta ao conteúdo de baixo, com as curvas para fora na base; as
//! outras ficam na faixa, separadas por um traço fino. Cada aba tem ícone, título e o ×
//! (na ativa e na que está sob o mouse); o "+" no fim abre uma nova. As abas encolhem para
//! caber, arrastar troca a ordem e o clique do meio fecha.
//!
//! Só a faixa: o conteúdo de cada aba fica por conta de quem usa (o `on_change` diz qual
//! mostrar). `tabs` no layout: `"Início|home;Relatório|description"` (título e ícone).
//!
//! Eventos: `on_change` (a aba escolhida: o índice), `on_close` (o × ou o clique do meio: o
//! índice; com `auto_close` a aba sai sozinha, sem ele quem decide é o código: perguntar se
//! salva, por exemplo, e chamar `remove`), `on_add` (o "+"), `on_reorder` (arrastou:
//! `"de,para"`), `on_tear_off` (arrastou a aba para fora e soltou longe de qualquer faixa: o
//! índice), `on_tab_drop` (chegou aqui uma aba de outra faixa: `"de,para"`; a origem em
//! [`TabStrip::last_drop`]) e `on_new_window` (a faixa da janela nova, criada pela aba
//! arrancada).
//!
//! Como as do navegador, a aba puxada para fora da faixa vira um arrasto com a prévia junto
//! do mouse (o título e a miniatura do conteúdo; no Windows ela segue o cursor pela tela,
//! fora da janela também). Solta noutra faixa do mesmo `drop_group`, em qualquer janela do
//! app, a aba muda para lá, no lugar marcado; de volta na própria faixa, só muda de lugar;
//! longe de todas, vira uma janela.
//!
//! Com páginas ([`TabStrip::add_page`]: cada aba com o seu conteúdo) a faixa cuida de tudo:
//! mostra só a página da aba escolhida, leva a página junto para a outra faixa, abre a janela
//! nova (com uma faixa igual, os mesmos handlers e a página) onde a aba foi solta, e fecha a
//! janela que ficou sem abas (`close_empty`). A janela de uma aba só é arrastada inteira:
//! solta longe, ela vai para lá; solta noutra faixa, junta-se a ela (arrastar uma janela de
//! fora para dentro). Sem páginas, a faixa só move os títulos e avisa (`on_tab_drop`,
//! `on_tear_off`): o conteúdo é de quem usa. No `on_tear_off` com páginas, recusar
//! (`ui.refuse`) impede a janela nova.

use std::cell::{Cell, RefCell};

use aylis_core::{
    Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, DragEnd, DragVisual, EventArgs, EventCtx, PaintCtx, Path, PointF,
    PointerButton, PointerEvent, PropValue, RectF, SizeF, Stroke, TextAlign, TextStyle, Trimming, Ui, WheelEvent, WindowCommand, WindowPlacement, properties,
};

use crate::leaf_control;

/// Quanto além da faixa (para cima ou para baixo) a aba puxada vira arrasto.
const TEAR_OFF: f32 = 40.0;
/// O começo dos dados de um arrasto de aba (`aylis-tab:grupo:faixa:índice`).
const TAB_DRAG: &str = "aylis-tab:";
/// A largura da prévia que acompanha o mouse e a altura do título nela.
const PREVIEW_WIDTH: f32 = 300.0;
const PREVIEW_HEAD: f32 = 34.0;

/// Uma aba.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tab {
    pub title: String,
    /// Ícone (`"description"`, `"pacote:nome"`); vazio: sem ícone.
    pub icon: String,
    /// A dica (o caminho do arquivo, por exemplo).
    pub hint: String,
    /// Alteração não salva: um ponto no lugar do × (que volta com o mouse em cima).
    pub modified: bool,
    /// O conteúdo da aba ([`TabStrip::add_page`]): só o da escolhida aparece, e ele vai
    /// junto quando a aba muda de faixa ou de janela.
    pub page: Option<ComponentId>,
}

/// A última aba que chegou de outra faixa (`on_tab_drop`): de onde veio e para onde foi.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabDrop {
    /// A faixa de origem (pode estar em outra janela).
    pub source: ComponentId,
    /// O índice que a aba tinha lá.
    pub from: usize,
    /// O índice dela aqui.
    pub to: usize,
}

/// Os dados de um arrasto de aba: (grupo, faixa de origem, índice).
fn parse_drag(data: &str) -> Option<(&str, ComponentId, usize)> {
    let rest = data.strip_prefix(TAB_DRAG)?;
    let mut parts = rest.rsplitn(3, ':');
    let index = parts.next()?.parse().ok()?;
    let source = ComponentId::from_u64(parts.next()?.parse().ok()?);
    Some((parts.next()?, source, index))
}

/// Onde cada coisa ficou na última pintura (para o mouse).
#[derive(Clone, Debug, Default)]
struct Hits {
    tabs: Vec<RectF>,
    closes: Vec<RectF>,
    add: Option<RectF>,
}

pub struct TabStrip {
    pub base: ControlBase,
    tabs: Vec<Tab>,
    /// A aba ativa, a partir de 0; -1: nenhuma.
    pub tab_index: i64,
    /// O × em cada aba.
    pub closable: bool,
    /// O "+" depois da última aba.
    pub show_add: bool,
    /// O × fecha a aba sozinho (desligado: só avisa, e o código fecha com `remove`).
    pub auto_close: bool,
    /// A largura máxima de uma aba (elas encolhem quando não cabem).
    pub max_tab_width: f32,
    /// A aba puxada para fora da faixa vira arrasto (para outra faixa ou uma janela nova).
    pub tear_off: bool,
    /// As abas só passam entre faixas do mesmo grupo (vazio também é um grupo).
    pub drop_group: String,
    /// Onde ficam as páginas das abas (vazio: o pai da faixa).
    pub page_host: Option<ComponentId>,
    /// Com páginas: a janela que fica sem abas (a última foi para outra faixa) fecha.
    pub close_empty: bool,
    hits: RefCell<Hits>,
    hover: Cell<Option<usize>>,
    hover_close: Cell<bool>,
    hover_add: Cell<bool>,
    /// Arrastando uma aba: qual e onde o mouse desceu.
    drag: Option<(usize, f32)>,
    dragging: bool,
    /// Arrastando dentro da faixa: o mouse agora e a distância dele ao canto da aba pega (a
    /// aba segue o mouse, como no Edge e no Chrome).
    drag_mouse: f32,
    grab_dx: f32,
    /// O desvio de cada aba do lugar dela, que vai a zero quadro a quadro: as vizinhas
    /// deslizam para o lugar novo, e a solta, para o seu.
    offsets: Vec<f32>,
    last_frame: Option<f64>,
    /// O começo das abas e a largura de cada uma (da última pintura).
    slots: Cell<(f32, f32)>,
    /// A rolagem da faixa (as abas não cabem com o nome inteiro: a roda rola na horizontal).
    scroll_x: Cell<f32>,
    /// Até onde a faixa rola (da última pintura).
    scroll_max: Cell<f32>,
    /// A largura que cabe o nome mais comprido (medida quando os nomes mudam: a marca deles).
    fit_w: Cell<(u64, f32)>,
    /// A aba ativa da última pintura (trocou: a faixa rola até ela).
    shown_active: Cell<Option<usize>>,
    /// Até onde (o canto esquerdo) a aba arrastada pode ir: a ponta da faixa, antes do "+".
    drag_max: Cell<f32>,
    /// A largura das abas que está na tela quando ela não é a natural: presa depois de fechar
    /// uma aba com o mouse na faixa (o × da próxima fica sob o mouse, como no Chrome e no
    /// Edge) e, quando o mouse sai, indo até a natural quadro a quadro.
    width_now: Option<f32>,
    width_held: bool,
    /// A largura natural (da última pintura): o alvo da animação.
    natural_w: Cell<f32>,
    /// A aba que saiu da faixa num arrasto (fica apagada até acabar) e o ponto dela que foi
    /// pego (a partir do canto da aba).
    lifted: Option<(usize, PointF)>,
    /// Um arrasto de aba está sobre a faixa: o lugar onde ela entraria (0..=n).
    drop_at: Cell<Option<usize>>,
    last_drop: Option<TabDrop>,
}

impl Default for TabStrip {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 40.0);
        base.set_cursor(CursorShape::Default);
        Self {
            base,
            tabs: Vec::new(),
            tab_index: 0,
            closable: true,
            show_add: true,
            auto_close: true,
            max_tab_width: 240.0,
            tear_off: true,
            drop_group: String::new(),
            page_host: None,
            close_empty: true,
            hits: RefCell::default(),
            hover: Cell::new(None),
            hover_close: Cell::new(false),
            hover_add: Cell::new(false),
            drag: None,
            dragging: false,
            drag_mouse: 0.0,
            grab_dx: 0.0,
            offsets: Vec::new(),
            last_frame: None,
            slots: Cell::new((8.0, 0.0)),
            scroll_x: Cell::new(0.0),
            scroll_max: Cell::new(0.0),
            fit_w: Cell::new((0, 0.0)),
            shown_active: Cell::new(None),
            drag_max: Cell::new(0.0),
            width_now: None,
            width_held: false,
            natural_w: Cell::new(0.0),
            lifted: None,
            drop_at: Cell::new(None),
            last_drop: None,
        }
    }
}

/// Os cantos de cima da aba ativa e as curvas da base.
const RADIUS: f32 = 8.0;
const ADD_SIZE: f32 = 30.0;
const MIN_TAB: f32 = 44.0;

impl TabStrip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Acrescenta uma aba no fim (não muda a ativa). Devolve o índice.
    pub fn add(&mut self, title: impl Into<String>, icon: impl Into<String>) -> usize {
        self.tabs.push(Tab { title: title.into(), icon: icon.into(), ..Default::default() });
        self.offsets.clear();
        self.tabs.len() - 1
    }

    /// Troca as abas todas e a escolhida (quem mantém a lista fora da faixa, como a IDE).
    /// Sem mudança nenhuma, não mexe: o deslize e a largura presa depois de fechar continuam.
    pub fn set_tabs(&mut self, tabs: Vec<Tab>, index: usize) {
        if self.tabs != tabs {
            self.tabs = tabs;
            self.offsets.clear();
        }
        let index = index.min(self.tabs.len().saturating_sub(1)) as i64;
        if self.tab_index != index && !self.dragging {
            self.tab_index = index;
        }
    }

    /// Tira a aba; a ativa continua a mesma (ou a vizinha, se era ela).
    pub fn remove(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }
        self.tabs.remove(index);
        self.offsets.clear();
        let active = self.tab_index.max(0) as usize;
        if index < active || (index == active && active >= self.tabs.len()) {
            self.tab_index = active.saturating_sub(1) as i64;
        }
    }

    pub fn tab_mut(&mut self, index: usize) -> Option<&mut Tab> {
        self.tabs.get_mut(index)
    }

    pub fn set_title(&mut self, index: usize, title: impl Into<String>) {
        if let Some(t) = self.tabs.get_mut(index) {
            t.title = title.into();
        }
    }

    pub fn set_modified(&mut self, index: usize, modified: bool) {
        if let Some(t) = self.tabs.get_mut(index) {
            t.modified = modified;
        }
    }

    /// Troca de lugar (como arrastar): a ativa acompanha.
    pub fn move_tab(&mut self, from: usize, to: usize) {
        if from >= self.tabs.len() || to >= self.tabs.len() || from == to {
            return;
        }
        let tab = self.tabs.remove(from);
        self.tabs.insert(to, tab);
        let active = self.tab_index.max(0) as usize;
        self.tab_index = if active == from {
            to as i64
        } else if from < active && to >= active {
            active as i64 - 1
        } else if from > active && to <= active {
            active as i64 + 1
        } else {
            active as i64
        };
    }

    /// Arrastando dentro da faixa: onde a aba pega aparece (segue o mouse, sem sair das
    /// pontas da faixa; a última pode ser empurrada até antes do "+", que vai junto).
    fn drag_x(&self, n: usize) -> f32 {
        let (left, w) = self.slots.get();
        let last = left + n.saturating_sub(1) as f32 * w;
        (self.drag_mouse - self.grab_dx).clamp(left, last.max(self.drag_max.get()))
    }

    /// Move a aba pelo arrasto, com as que mudaram de lugar saindo de onde estavam na tela
    /// (o desvio vai a zero no `frame`).
    fn slide_tab(&mut self, ctx: &mut EventCtx<'_>, from: usize, to: usize) {
        let w = self.slots.get().1;
        self.offsets.resize(self.tabs.len(), 0.0);
        let moved = self.offsets.remove(from);
        self.offsets.insert(to, moved);
        let (step, range) = if from < to { (w, from..to) } else { (-w, to + 1..from + 1) };
        for k in range {
            self.offsets[k] += step;
        }
        self.move_tab(from, to);
        self.start_frames(ctx);
    }

    fn start_frames(&mut self, ctx: &mut EventCtx<'_>) {
        if self.last_frame.is_none() {
            self.last_frame = Some(ctx.ui.time());
        }
        ctx.ui.request_frames(ctx.id);
    }

    /// As abas como texto: `"Título|ícone;Outra"` (o que vai para o layout).
    pub fn tabs_text(&self) -> String {
        self.tabs
            .iter()
            .map(|t| if t.icon.is_empty() { t.title.replace(';', ",") } else { format!("{}|{}", t.title.replace(';', ","), t.icon) })
            .collect::<Vec<_>>()
            .join(";")
    }

    pub fn set_tabs_text(&mut self, text: String) {
        self.offsets.clear();
        self.tabs = text
            .split(';')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let mut parts = s.splitn(2, '|');
                Tab { title: parts.next().unwrap_or_default().trim().to_string(), icon: parts.next().unwrap_or_default().trim().to_string(), ..Default::default() }
            })
            .collect();
        self.tab_index = self.tab_index.clamp(0, self.tabs.len().saturating_sub(1) as i64);
    }

    fn select(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if self.tab_index != index as i64 {
            self.tab_index = index as i64;
            show_pages(ctx.ui, &self.pages());
            ctx.fire("on_change", EventArgs::Value((index as i64).into()));
        }
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        // Fechou com o mouse: as abas ficam da largura de agora enquanto ele estiver na faixa
        // (o × da próxima vem para debaixo dele, para fechar várias seguidas).
        let (_, w) = self.slots.get();
        if w > 0.0 {
            self.width_now = Some(self.width_now.unwrap_or(w));
            self.width_held = true;
        }
        ctx.fire("on_close", EventArgs::Value((index as i64).into()));
        if self.auto_close {
            let was = self.tab_index;
            let page = self.tabs.get(index).and_then(|t| t.page);
            self.remove(index);
            if let Some(page) = page {
                ctx.ui.free(page);
            }
            show_pages(ctx.ui, &self.pages());
            if self.tab_index != was || index as i64 == was {
                ctx.fire("on_change", EventArgs::Value(self.tab_index.into()));
            }
        }
        ctx.repaint();
    }

    /// Chegou uma aba arrastada (de outra faixa ou desta mesma) no lugar `slot`.
    fn receive(&mut self, ctx: &mut EventCtx<'_>, source: ComponentId, from: usize, slot: usize) {
        if source == ctx.id {
            // A aba voltou para a própria faixa: só muda de lugar.
            let to = if slot > from { slot - 1 } else { slot }.min(self.tabs.len().saturating_sub(1));
            if to != from {
                self.move_tab(from, to);
                ctx.fire("on_reorder", EventArgs::Text(format!("{from},{to}")));
            }
            self.select(ctx, to);
            return;
        }
        let Some(tab) = ctx.ui.get_mut::<TabStrip>(source).and_then(|s| {
            if from >= s.tabs.len() {
                return None;
            }
            let tab = s.tabs[from].clone();
            s.remove(from);
            Some(tab)
        }) else {
            return;
        };
        // A origem mostra a página que ficou escolhida lá e avisa.
        TabStrip::sync_pages(ctx.ui, source);
        let left = ctx.ui.get::<TabStrip>(source).map_or(0, |s| s.tabs.len());
        let source_index = ctx.ui.get::<TabStrip>(source).map_or(0, |s| s.tab_index);
        ctx.ui.fire(source, "on_change", &EventArgs::Value(source_index.into()));
        let to = slot.min(self.tabs.len());
        if let Some(page) = tab.page {
            if let Some(host) = self.host(ctx.ui, ctx.id) {
                let _ = ctx.ui.set_parent(page, Some(host));
            }
        }
        let had_page = tab.page.is_some();
        self.tabs.insert(to, tab);
        self.offsets.clear();
        self.tab_index = to as i64;
        show_pages(ctx.ui, &self.pages());
        self.last_drop = Some(TabDrop { source, from, to });
        ctx.fire("on_tab_drop", EventArgs::Text(format!("{from},{to}")));
        ctx.fire("on_change", EventArgs::Value((to as i64).into()));
        // A janela de onde a aba saiu ficou vazia: fecha (a daqui vira a principal, se era ela).
        let empty_closes = ctx.ui.get::<TabStrip>(source).is_some_and(|s| s.close_empty);
        if had_page && left == 0 && empty_closes {
            let old = ctx.ui.root_of(source);
            let here = ctx.ui.root_of(ctx.id);
            if old != here && ctx.ui.is_open(old) {
                if ctx.ui.main_form() == Some(old) {
                    ctx.ui.set_main_form(here);
                }
                ctx.ui.close(old);
            }
        }
        ctx.ui.window_command(ctx.id, WindowCommand::Show);
        ctx.repaint();
    }

    /// A largura de aba que cabe o nome mais comprido (o ícone e o × juntos). Medida só
    /// quando os nomes mudam (a marca deles): a pintura não mede as abas todas a cada quadro.
    fn fit_width(&self, canvas: &mut dyn Canvas, style: &TextStyle) -> f32 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for t in &self.tabs {
            (&t.title, t.icon.is_empty()).hash(&mut h);
        }
        let key = h.finish();
        let (cached, width) = self.fit_w.get();
        if cached == key && width > 0.0 {
            return width;
        }
        let widest = self.tabs.iter().map(|t| canvas.measure_text(&t.title, style).width + if t.icon.is_empty() { 0.0 } else { 22.0 }).fold(0.0, f32::max);
        // 12 de margem à esquerda, o × (30) e uma folga à direita.
        let width = (widest + 12.0 + 34.0).max(MIN_TAB);
        self.fit_w.set((key, width));
        width
    }

    fn tab_at(&self, p: PointF) -> Option<usize> {
        self.hits.borrow().tabs.iter().position(|r| r.contains(p))
    }

    /// A última aba que chegou de outra faixa (para o `on_tab_drop` sem páginas: quem usa
    /// leva o conteúdo de lá para cá).
    pub fn last_drop(&self) -> Option<TabDrop> {
        self.last_drop
    }

    /// Acrescenta uma aba com o seu conteúdo (`page`, que passa para o `page_host`, ocupando
    /// o espaço todo se não tiver alinhamento) e a escolhe. Devolve o índice.
    pub fn add_page(ui: &mut Ui, strip: ComponentId, title: &str, icon: &str, page: ComponentId) -> usize {
        let Some(host) = ui.get::<TabStrip>(strip).map(|s| s.page_host).map(|h| h.or_else(|| ui.parent(strip))) else { return 0 };
        if let Some(host) = host.filter(|&h| ui.parent(page) != Some(h)) {
            let _ = ui.set_parent(page, Some(host));
        }
        if let Some(cb) = ui.control_mut(page).filter(|cb| cb.align() == Align::None) {
            cb.set_align(Align::Client);
        }
        let Some(s) = ui.get_mut::<TabStrip>(strip) else { return 0 };
        s.tabs.push(Tab { title: title.into(), icon: icon.into(), page: Some(page), ..Default::default() });
        let index = s.tabs.len() - 1;
        s.tab_index = index as i64;
        Self::sync_pages(ui, strip);
        index
    }

    /// Fecha a aba pelo código (com página, ela é liberada) e mostra a que ficou escolhida.
    pub fn close_tab(ui: &mut Ui, strip: ComponentId, index: usize) {
        let Some(s) = ui.get_mut::<TabStrip>(strip) else { return };
        let page = s.tabs.get(index).and_then(|t| t.page);
        s.remove(index);
        if let Some(page) = page {
            ui.free(page);
        }
        Self::sync_pages(ui, strip);
    }

    /// Deixa visível só a página da aba escolhida.
    pub fn sync_pages(ui: &mut Ui, strip: ComponentId) {
        let Some(s) = ui.get::<TabStrip>(strip) else { return };
        let pages = s.pages();
        show_pages(ui, &pages);
    }

    /// (página, é a escolhida) de cada aba com página.
    fn pages(&self) -> Vec<(ComponentId, bool)> {
        let active = self.tab_index.max(0) as usize;
        self.tabs.iter().enumerate().filter_map(|(i, t)| t.page.map(|p| (p, i == active))).collect()
    }

    /// Onde ficam as páginas desta faixa.
    fn host(&self, ui: &Ui, id: ComponentId) -> Option<ComponentId> {
        self.page_host.filter(|&h| ui.contains(h)).or_else(|| ui.parent(id))
    }

    /// O lugar (0..=n) onde uma aba solta no ponto `x` entraria.
    fn slot_at(&self, x: f32) -> usize {
        self.hits.borrow().tabs.iter().filter(|r| r.x + r.width / 2.0 < x).count()
    }

    /// O canto da faixa no form (para converter o ponto do mouse).
    fn origin_in_form(&self, ui: &Ui, id: ComponentId) -> PointF {
        let own = self.base.bounds();
        let parent = ui.parent(id).and_then(|p| ui.absolute_rect(p)).unwrap_or_default();
        PointF::new(parent.x + own.x, parent.y + own.y)
    }

    /// A prévia que acompanha o mouse: o título da aba e, embaixo, a miniatura do conteúdo
    /// (a página; sem página, a janela). `grab`: o ponto pego, a partir do canto da aba.
    fn preview(&self, ui: &Ui, id: ComponentId, index: usize, grab: PointF) -> Option<DragVisual> {
        let tab = self.tabs.get(index)?;
        let content = tab.page.filter(|&p| ui.contains(p)).unwrap_or_else(|| ui.root_of(id));
        let shot = ui.snapshot(content, 1.0);
        let bounds = ui.control(content).map(|c| c.bounds()).unwrap_or_default();
        let body = match &shot {
            Some(_) if bounds.width > 1.0 => ((PREVIEW_WIDTH - 2.0) * bounds.height / bounds.width).clamp(60.0, 220.0),
            _ => 0.0,
        };
        let size = SizeF::new(PREVIEW_WIDTH, PREVIEW_HEAD + body + 1.0);
        let theme = ui.theme();
        let (surface, text, border, back) = (theme.color("surface"), theme.color("text"), theme.color("border"), theme.color("background"));
        let title = tab.title.clone();
        let service = ui.services().offscreen.as_ref()?;
        let png = service.render_png(size.width.ceil() as u32, size.height.ceil() as u32, 2.0, Color::TRANSPARENT, &mut |canvas| {
            let all = RectF::new(0.0, 0.0, size.width, size.height);
            canvas.fill_rect(all, 8.0, &Brush::Solid(surface));
            let mut style = TextStyle { color: text, v_align: TextAlign::Center, trimming: Trimming::Character, ..Default::default() };
            style.font.weight = aylis_core::FontWeight::Bold;
            canvas.draw_text(&title, RectF::new(12.0, 0.0, size.width - 24.0, PREVIEW_HEAD), &style);
            if let Some(image) = &shot {
                let dest = RectF::new(1.0, PREVIEW_HEAD, size.width - 2.0, body);
                canvas.fill_rect(dest, 0.0, &Brush::Solid(back));
                let px = canvas.image_size(image).unwrap_or(SizeF::new(bounds.width, bounds.height));
                // Conteúdo mais alto que a prévia: mostra a parte de cima, sem achatar.
                let src_h = (px.width * body / dest.width).min(px.height);
                canvas.save();
                canvas.clip_round_rect(RectF::new(1.0, PREVIEW_HEAD, size.width - 2.0, body), 7.0);
                canvas.draw_image(image, RectF::new(0.0, 0.0, px.width, src_h), RectF::new(dest.x, dest.y, dest.width, dest.width * src_h / px.width.max(1.0)), 1.0);
                canvas.restore();
            }
            canvas.stroke_rect(all.inflate(-0.5), 8.0, &Stroke::new(border, 1.0));
        })?;
        let hotspot = PointF::new(grab.x.clamp(12.0, size.width - 12.0), grab.y.clamp(6.0, PREVIEW_HEAD - 6.0));
        Some(DragVisual { image: aylis_core::Image::encoded(png), size, hotspot, opacity: 0.9 })
    }

    /// Começa o arrasto da aba `index` (puxada para fora da faixa).
    fn lift(&mut self, ctx: &mut EventCtx<'_>, index: usize, p: PointF) {
        let Some(rect) = self.hits.borrow().tabs.get(index).copied() else { return };
        // O mouse já saiu da faixa: na altura, o meio da aba.
        let grab = PointF::new((p.x - rect.x).clamp(0.0, rect.width), rect.height / 2.0);
        let visual = self.preview(ctx.ui, ctx.id, index, grab);
        let data = format!("{TAB_DRAG}{}:{}:{index}", self.drop_group, ctx.id.to_u64());
        let origin = self.origin_in_form(ctx.ui, ctx.id);
        // Com a prévia, o título já vai nela; sem ela (sem desenho fora da tela), vai o rótulo.
        let title = if visual.is_some() { String::new() } else { self.tabs[index].title.clone() };
        if ctx.ui.begin_drag_now(ctx.id, data, title, visual, PointF::new(origin.x + p.x, origin.y + p.y)) {
            self.lifted = Some((index, grab));
            self.hover.set(None);
            ctx.repaint();
        }
    }

    /// Onde fica o canto da janela para que o ponto `grab` (no form) caia sob o mouse.
    fn window_corner(ui: &Ui, form: ComponentId, grab: PointF) -> Option<(i32, i32)> {
        let screen = ui.screen_pointer()?;
        let scale = screen.scale.max(0.1);
        // A moldura do sistema (a borda e a barra de título) fica antes do conteúdo.
        let framed = !matches!(ui.get_prop(form, "border_style"), Ok(PropValue::Enum(ref s)) if s == "none" || s == "custom");
        let frame = if framed { PointF::new(8.0, 31.0) } else { PointF::default() };
        Some(((screen.x as f32 - (grab.x + frame.x) * scale) as i32, (screen.y as f32 - (grab.y + frame.y) * scale) as i32))
    }

    /// A aba solta longe de qualquer faixa (com página): vira uma janela, com uma faixa igual
    /// a esta (os mesmos handlers), onde o mouse está. A janela de uma aba só vai inteira.
    fn tear_into_window(&mut self, ctx: &mut EventCtx<'_>, index: usize, grab: PointF) {
        let form = ctx.ui.root_of(ctx.id);
        let tab_x = self.hits.borrow().tabs.get(index).map_or(8.0, |r| r.x);
        let origin = self.origin_in_form(ctx.ui, ctx.id);
        if self.tabs.len() == 1 {
            let grab_in_form = PointF::new(origin.x + tab_x + grab.x, origin.y + 6.0 + grab.y);
            if let Some((x, y)) = Self::window_corner(ctx.ui, form, grab_in_form) {
                ctx.ui.window_command(form, WindowCommand::MoveTo(x, y));
            }
            return;
        }
        let tab = self.tabs[index].clone();
        let was = self.tab_index;
        self.remove(index);
        show_pages(ctx.ui, &self.pages());
        if self.tab_index != was || index as i64 == was {
            ctx.fire("on_change", EventArgs::Value(self.tab_index.into()));
        }
        // A janela nova, do tamanho desta.
        let placement = ctx.ui.window_placement(form).unwrap_or_default();
        let (window, new_strip, host, strip_origin) = match self.new_instance(ctx.ui, ctx.id, form) {
            Some((window, strip, host)) => (window, strip, host, origin),
            None => {
                let (window, strip, host) = self.plain_window(ctx.ui, ctx.id, form, &tab.title);
                (window, strip, host, PointF::default())
            }
        };
        if let Some(s) = ctx.ui.get_mut::<TabStrip>(new_strip) {
            s.tabs.push(Tab { page: tab.page, ..tab.clone() });
            s.tab_index = 0;
        }
        if let (Some(page), Some(host)) = (tab.page, host) {
            let _ = ctx.ui.set_parent(page, Some(host));
            if let Some(cb) = ctx.ui.control_mut(page) {
                cb.set_visible(true);
            }
        }
        let grab_in_form = PointF::new(strip_origin.x + 8.0 + grab.x, strip_origin.y + 6.0 + grab.y);
        let corner = Self::window_corner(ctx.ui, window, grab_in_form);
        if corner.is_some() || placement.width > 0 {
            let (x, y) = corner.unwrap_or((placement.x + 40, placement.y + 40));
            ctx.ui.set_window_placement(window, WindowPlacement { x, y, width: placement.width, height: placement.height, maximized: false });
        }
        let _ = ctx.ui.show(window);
        // Como as do navegador: janela independente, não presa por cima da outra.
        ctx.ui.set_form_owner(window, None);
        ctx.fire("on_new_window", EventArgs::Value(PropValue::Component(Some(new_strip))));
        ctx.repaint();
    }

    /// A janela nova como outra instância do mesmo form do projeto (como o navegador abre
    /// outra janela dele): os handlers por nome falam com a instância nova. A faixa dela é a
    /// de mesmo nome, que chega vazia (as abas do layout saem); o `on_show` da instância vê
    /// a aba arrancada lá dentro e pode pular as abas iniciais. `None`: o form não veio de
    /// um projeto (sem nome ou sem fábrica de forms).
    fn new_instance(&self, ui: &mut Ui, id: ComponentId, form: ComponentId) -> Option<(ComponentId, ComponentId, Option<ComponentId>)> {
        let form_name = ui.name(form).filter(|n| !n.is_empty())?.to_string();
        let strip_name = ui.name(id).filter(|n| !n.is_empty())?.to_string();
        ui.services().forms.as_ref()?;
        let window = ui.create_form_instance(&form_name).ok()?;
        let Some(strip) = ui.find_in(window, &strip_name).filter(|&s| ui.get::<TabStrip>(s).is_some()) else {
            ui.free(window);
            return None;
        };
        let host = {
            let s = ui.get_mut::<TabStrip>(strip)?;
            s.tabs.clear();
            s.tab_index = 0;
            s.page_host
        };
        Some((window, strip, host.or_else(|| ui.parent(strip))))
    }

    /// A janela nova sem projeto: um form com uma faixa igual a esta (os mesmos handlers) e o
    /// lugar das páginas embaixo.
    fn plain_window(&self, ui: &mut Ui, id: ComponentId, form: ComponentId, title: &str) -> (ComponentId, ComponentId, Option<ComponentId>) {
        let bounds = ui.control(form).map(|c| c.bounds()).unwrap_or(RectF::new(0.0, 0.0, 900.0, 600.0));
        let window = ui.add_root(crate::Form::new(title, bounds.width, bounds.height));
        let mut strip = TabStrip { closable: self.closable, show_add: self.show_add, auto_close: self.auto_close, max_tab_width: self.max_tab_width, tear_off: self.tear_off, drop_group: self.drop_group.clone(), close_empty: self.close_empty, ..Default::default() };
        strip.base.set_align(Align::Top).set_height(self.base.height()).set_style(self.base.style().to_string());
        let new_strip = ui.add(strip, window);
        let host = ui.add(crate::Layout::default(), window);
        ui.control_mut(host).unwrap().set_align(Align::Client);
        ui.copy_events(id, new_strip);
        if let Some(s) = ui.get_mut::<TabStrip>(new_strip) {
            s.page_host = Some(host);
        }
        (window, new_strip, Some(host))
    }
}

/// Mostra só as páginas marcadas.
fn show_pages(ui: &mut Ui, pages: &[(ComponentId, bool)]) {
    for &(page, shown) in pages {
        if let Some(cb) = ui.control_mut(page) {
            cb.set_visible(shown);
        }
    }
    ui.invalidate();
}

impl Component for TabStrip {
    leaf_control!("TabStrip");

    properties! {
        tab_index: i64 => "Behavior",
        closable: bool => "Behavior",
        show_add: bool => "Behavior",
        auto_close: bool => "Behavior",
        max_tab_width: f32 => "Appearance",
        tear_off: bool => "Behavior",
        drop_group: String => "Behavior",
        page_host: Option<ComponentId> => "Behavior",
        close_empty: bool => "Behavior";
        events: on_change, on_close, on_add, on_reorder, on_tear_off, on_tab_drop, on_new_window;
        computed: tabs: String => tabs_text / set_tabs_text
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.page_host == Some(freed) {
            self.page_host = None;
        }
        for t in &mut self.tabs {
            if t.page == Some(freed) {
                t.page = None;
            }
        }
    }

    // O arrasto de aba (desta faixa ou de outra, em qualquer janela) é recebido aqui mesmo.
    // Uma aba sem página vinda de outra faixa só entra se quem usa leva o conteúdo junto (tem
    // o `on_tab_drop`): faixas sem páginas que não esperam abas de fora não as recebem.
    fn drop_accepts(&self, ui: &Ui, id: ComponentId, data: &str) -> bool {
        let Some((group, source, index)) = parse_drag(data) else { return false };
        if group != self.drop_group || !ui.contains(source) {
            return false;
        }
        let has_page = ui.get::<TabStrip>(source).and_then(|s| s.tabs.get(index)).is_some_and(|t| t.page.is_some());
        source == id || has_page || ui.has_handler(id, "on_tab_drop")
    }

    fn drop_hover(&mut self, ctx: &mut EventCtx<'_>, _data: &str, position: PointF) {
        let slot = Some(self.slot_at(position.x));
        if self.drop_at.replace(slot) != slot {
            ctx.repaint();
        }
    }

    fn drop_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if self.drop_at.take().is_some() {
            ctx.repaint();
        }
    }

    fn drop_receive(&mut self, ctx: &mut EventCtx<'_>, data: &str, position: PointF) {
        let slot = self.drop_at.take().unwrap_or_else(|| self.slot_at(position.x));
        let Some((_, source, from)) = parse_drag(data) else { return };
        if source == ctx.id {
            self.lifted = None;
        }
        self.receive(ctx, source, from, slot);
    }

    fn drag_ended(&mut self, ctx: &mut EventCtx<'_>, _data: &str, end: DragEnd) {
        let Some((index, grab)) = self.lifted.take() else { return };
        ctx.repaint();
        if end != DragEnd::Nowhere || index >= self.tabs.len() {
            return;
        }
        if self.tabs[index].page.is_none() {
            // Sem páginas: quem usa decide (como levar a aba para outra janela).
            ctx.fire("on_tear_off", EventArgs::Value((index as i64).into()));
            return;
        }
        let asked = ctx.ui.ask(ctx.id, "on_tear_off", &EventArgs::Value((index as i64).into()));
        if matches!(asked, aylis_core::question::Asked::Answered(aylis_core::question::Answer::Refused(_))) {
            return;
        }
        self.tear_into_window(ctx, index, grab);
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        let hit_add = self.hits.borrow().add.is_some_and(|r| r.contains(p));
        if event.button == PointerButton::Primary && hit_add {
            ctx.fire("on_add", EventArgs::None);
            ctx.set_handled();
            return;
        }
        let Some(i) = self.tab_at(p) else { return };
        ctx.set_handled();
        match event.button {
            PointerButton::Middle if self.closable => self.close(ctx, i),
            PointerButton::Primary => {
                if self.closable && self.hits.borrow().closes.get(i).is_some_and(|r| r.contains(p)) {
                    self.close(ctx, i);
                    return;
                }
                self.select(ctx, i);
                self.drag = Some((i, p.x));
                self.dragging = false;
                self.drag_mouse = p.x;
                self.grab_dx = p.x - self.hits.borrow().tabs.get(i).map_or(p.x, |r| r.x);
                ctx.repaint();
            }
            _ => {}
        }
    }

    /// A roda rola a faixa na horizontal quando as abas não cabem.
    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let delta = if event.delta.x.abs() > event.delta.y.abs() { event.delta.x } else { event.delta.y };
        let before = self.scroll_x.get();
        self.scroll_x.set((before - delta).clamp(0.0, self.scroll_max.get()));
        if (self.scroll_x.get() - before).abs() > 0.1 {
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        // Arrastando: a aba segue o mouse e troca de lugar quando o meio dela passa do meio da
        // vizinha; as vizinhas deslizam para o lugar novo (como no Edge e no Chrome).
        if let Some((from, start)) = self.drag {
            if !self.dragging && (p.x - start).abs() > 5.0 {
                self.dragging = true;
            }
            if self.dragging {
                // Puxada para longe da faixa: vira o arrasto com a prévia (para outra faixa ou
                // uma janela nova).
                let h = self.base.bounds().height;
                if self.tear_off && (p.y < -TEAR_OFF || p.y > h + TEAR_OFF) {
                    self.drag = None;
                    self.dragging = false;
                    self.lift(ctx, from, p);
                    return;
                }
                self.drag_mouse = p.x;
                let n = self.tabs.len();
                let (left, w) = self.slots.get();
                if w > 0.0 && n > 0 {
                    let center = self.drag_x(n) + w / 2.0;
                    let to = (((center - left) / w).floor().max(0.0) as usize).min(n - 1);
                    if to != from {
                        self.slide_tab(ctx, from, to);
                        self.drag = Some((to, start));
                        ctx.fire("on_reorder", EventArgs::Text(format!("{from},{to}")));
                    }
                }
                ctx.repaint();
                return;
            }
        }
        let over = self.tab_at(p);
        let over_close = over.is_some_and(|i| self.hits.borrow().closes.get(i).is_some_and(|r| r.contains(p)));
        let over_add = self.hits.borrow().add.is_some_and(|r| r.contains(p));
        if (over != self.hover.replace(over)) | (over_close != self.hover_close.replace(over_close)) | (over_add != self.hover_add.replace(over_add)) {
            let hint = over.and_then(|i| self.tabs.get(i)).map(|t| if t.hint.is_empty() { t.title.clone() } else { t.hint.clone() }).unwrap_or_default();
            self.base.set_hint(hint);
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        let dragged = self.drag.take().map(|(i, _)| i);
        if std::mem::take(&mut self.dragging) {
            // Solta: a aba desliza de onde está até o lugar dela.
            if let Some(i) = dragged.filter(|&i| i < self.tabs.len()) {
                let (left, w) = self.slots.get();
                self.offsets.resize(self.tabs.len(), 0.0);
                self.offsets[i] = self.drag_x(self.tabs.len()) - (left + i as f32 * w);
                self.start_frames(ctx);
            }
            ctx.repaint();
        }
    }

    /// Um passo do deslize: os desvios caem pela metade a cada ~40 ms (rápido no começo,
    /// assentando suave), no tempo do quadro.
    fn frame(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        let now = ctx.ui.time();
        let dt = (now - self.last_frame.unwrap_or(now)).clamp(0.0, 0.1) as f32;
        self.last_frame = Some(now);
        let k = (-dt / 0.055).exp();
        let mut moving = false;
        for o in &mut self.offsets {
            *o *= k;
            if o.abs() < 0.5 {
                *o = 0.0;
            } else {
                moving = true;
            }
        }
        // A largura solta (o mouse saiu da faixa) vai até a natural, no mesmo ritmo.
        if let Some(w) = self.width_now.filter(|_| !self.width_held) {
            let target = self.natural_w.get();
            let next = target + (w - target) * k;
            if (next - target).abs() < 0.5 {
                self.width_now = None;
            } else {
                self.width_now = Some(next);
                moving = true;
            }
        }
        ctx.repaint();
        if !moving {
            self.last_frame = None;
        }
        moving
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        // O mouse saiu: a largura presa vai até a natural, deslizando.
        if std::mem::take(&mut self.width_held) && self.width_now.is_some() {
            self.start_frames(ctx);
        }
        if self.hover.replace(None).is_some() | self.hover_close.replace(false) | self.hover_add.replace(false) {
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let s = ctx.style;
        canvas.fill_rect(r, 0.0, &Brush::Solid(s.fill));
        let theme = ctx.theme;
        let active_fill = theme.color("surface");
        let text = s.text;
        let muted = theme.color("text_muted");
        let mut hits = Hits::default();
        let n = self.tabs.len();
        let left = r.x + 8.0;
        let add_w = if self.show_add { ADD_SIZE + 8.0 } else { 0.0 };
        let room = (r.width - 16.0 - add_w).max(0.0);
        // Todas da mesma largura, com o nome inteiro (até o `max_tab_width`): com muitas abas, elas
        // não encolhem até "fr…" (relatado pelo autor em 08/11); passando da faixa, a roda rola.
        let fit = self.fit_width(canvas, &ctx.text_style()).min(self.max_tab_width.max(MIN_TAB));
        let natural = if n == 0 { 0.0 } else { (room / n as f32).clamp(fit, self.max_tab_width.max(fit)) };
        self.natural_w.set(natural);
        // Presa ou animando: a largura de agora, sem passar do que cabe.
        let w = match self.width_now {
            Some(now) if n > 0 => now.min((room / n as f32).max(natural)),
            _ => natural,
        };
        let top = r.y + 6.0;
        let bottom = r.bottom();
        let active = self.tab_index.max(0) as usize;
        // A rolagem: no limite do que sobra; a aba ativa nova vem para a vista.
        let overflow = (n as f32 * w - room).max(0.0);
        let mut scroll = self.scroll_x.get().clamp(0.0, overflow);
        if self.shown_active.replace(Some(active)) != Some(active) && active < n {
            let (a0, a1) = (active as f32 * w, (active + 1) as f32 * w);
            if a0 < scroll {
                scroll = a0;
            } else if a1 > scroll + room {
                scroll = (a1 - room).min(overflow);
            }
        }
        self.scroll_x.set(scroll);
        self.scroll_max.set(overflow);
        let left = left - scroll;
        let hover = self.hover.get();
        let mut style = ctx.text_style();
        style.v_align = TextAlign::Center;
        style.trimming = Trimming::Character;
        canvas.save();
        canvas.clip_rect(r);
        let lifted = self.lifted.map(|(i, _)| i);
        self.slots.set((left, w));
        // A aba arrastada vai até a ponta da faixa (antes do "+"), mesmo passando do último lugar.
        self.drag_max.set((r.right() - 8.0 - add_w - w).max(left));
        let mut rightmost = left + n as f32 * w;
        // Os lugares (para o mouse) ficam fixos; o desenho anda com o desvio de cada aba, e a
        // arrastada fica sob o mouse, pintada por último (por cima das vizinhas).
        for i in 0..n {
            let x = left + i as f32 * w;
            hits.tabs.push(RectF::new(x, top, w, bottom - top));
            // O × só vale onde ele é desenhado: com a aba estreita ele não aparece, e o clique na
            // borda direita fechava a aba (relatado pelo autor em 08/11).
            let close_drawn = self.closable && w > MIN_TAB + 12.0;
            hits.closes.push(if close_drawn { RectF::new(x + w - 30.0, top + (bottom - top - 22.0) / 2.0, 22.0, 22.0) } else { RectF::default() });
        }
        let dragged = self.drag.filter(|_| self.dragging).map(|(i, _)| i);
        let order = (0..n).filter(|&i| i != active).chain((active < n).then_some(active));
        for i in order {
            let tab = &self.tabs[i];
            let x = if dragged == Some(i) { self.drag_x(n) } else { left + i as f32 * w + self.offsets.get(i).copied().unwrap_or(0.0) };
            rightmost = rightmost.max(x + w);
            if lifted == Some(i) {
                // A aba que saiu num arrasto: só o contorno tracejado no lugar dela.
                let ghost = RectF::new(x + 3.0, top + 3.0, w - 6.0, bottom - top - 7.0);
                canvas.stroke_rect(ghost, 6.0, &Stroke::new(muted.with_opacity(0.6), 1.0));
                let mut ts = style.clone();
                ts.color = muted.with_opacity(0.5);
                canvas.draw_text(&tab.title, RectF::new(x + 12.0, top, (w - 24.0).max(0.0), bottom - top), &ts);
                hits.closes[i] = RectF::default();
                continue;
            }
            let is_active = i == active;
            if is_active {
                // A aba ativa: cantos de cima redondos e as curvas para fora na base.
                let (x0, x1) = (x, x + w);
                let mut path = Path::new();
                path.move_to(x0 - RADIUS, bottom)
                    .quad_to(x0, bottom, x0, bottom - RADIUS)
                    .line_to(x0, top + RADIUS)
                    .quad_to(x0, top, x0 + RADIUS, top)
                    .line_to(x1 - RADIUS, top)
                    .quad_to(x1, top, x1, top + RADIUS)
                    .line_to(x1, bottom - RADIUS)
                    .quad_to(x1, bottom, x1 + RADIUS, bottom)
                    .close();
                canvas.fill_path(&path, &Brush::Solid(active_fill));
            } else if hover == Some(i) {
                canvas.fill_rect(RectF::new(x + 2.0, top + 3.0, w - 4.0, bottom - top - 7.0), 6.0, &Brush::Solid(text.with_opacity(0.07)));
            }
            // O traço entre duas abas de fundo (some ao lado da ativa e da do mouse).
            let next_plain = i + 1 < n && i + 1 != active && hover != Some(i + 1);
            if !is_active && hover != Some(i) && next_plain {
                canvas.draw_line(PointF::new(x + w, top + 9.0), PointF::new(x + w, bottom - 9.0), &Stroke::new(muted.with_opacity(0.45), 1.0));
            }
            let mut cx = x + 12.0;
            if !tab.icon.is_empty() && w > MIN_TAB + 8.0 {
                ctx.draw_icon(canvas, &tab.icon, RectF::new(cx, top + (bottom - top - 16.0) / 2.0, 16.0, 16.0), if is_active { text } else { muted });
                cx += 22.0;
            }
            let show_close = self.closable && (is_active || hover == Some(i) || tab.modified) && w > MIN_TAB + 12.0;
            let close = RectF::new(x + w - 30.0, top + (bottom - top - 22.0) / 2.0, 22.0, 22.0);
            let text_right = if show_close { close.x - 4.0 } else { x + w - 10.0 };
            let mut ts = style.clone();
            ts.color = if is_active { text } else { muted };
            canvas.draw_text(&tab.title, RectF::new(cx, top, (text_right - cx).max(0.0), bottom - top), &ts);
            if show_close {
                let over_close = hover == Some(i) && self.hover_close.get();
                if tab.modified && !over_close {
                    // Não salvo: o ponto (o × volta com o mouse em cima).
                    let d = 8.0;
                    canvas.fill_ellipse(RectF::new(close.x + (close.width - d) / 2.0, close.y + (close.height - d) / 2.0, d, d), &Brush::Solid(ts.color));
                } else {
                    if over_close {
                        canvas.fill_rect(close, 4.0, &Brush::Solid(text.with_opacity(0.12)));
                    }
                    let c = PointF::new(close.x + close.width / 2.0, close.y + close.height / 2.0);
                    let k = 4.0;
                    let st = Stroke::new(ts.color, 1.3);
                    canvas.draw_line(PointF::new(c.x - k, c.y - k), PointF::new(c.x + k, c.y + k), &st);
                    canvas.draw_line(PointF::new(c.x + k, c.y - k), PointF::new(c.x - k, c.y + k), &st);
                }
            }
        }
        if self.show_add {
            // O "+" vem logo depois da aba mais à direita na tela: a empurrada leva ele junto (com
            // a faixa rolando, fica na ponta).
            let ax = (rightmost + 6.0).min(r.right() - 8.0 - ADD_SIZE);
            let add = RectF::new(ax, top + (bottom - top - ADD_SIZE) / 2.0, ADD_SIZE, ADD_SIZE);
            if self.hover_add.get() {
                canvas.fill_rect(add, 6.0, &Brush::Solid(text.with_opacity(0.08)));
            }
            let c = PointF::new(add.x + add.width / 2.0, add.y + add.height / 2.0);
            let st = Stroke::new(text, 1.4);
            canvas.draw_line(PointF::new(c.x - 6.0, c.y), PointF::new(c.x + 6.0, c.y), &st);
            canvas.draw_line(PointF::new(c.x, c.y - 6.0), PointF::new(c.x, c.y + 6.0), &st);
            hits.add = Some(add);
        }
        // Uma aba arrastada está sobre a faixa: a marca de onde ela entra.
        if let Some(slot) = self.drop_at.get() {
            let x = (left + slot.min(n) as f32 * w).min(r.right() - 3.0);
            let primary = theme.color("primary");
            canvas.fill_rect(RectF::new(x - 1.5, top + 2.0, 3.0, bottom - top - 4.0), 1.5, &Brush::Solid(primary));
            canvas.fill_ellipse(RectF::new(x - 4.0, top - 1.0, 8.0, 8.0), &Brush::Solid(primary));
        }
        canvas.restore();
        *self.hits.borrow_mut() = hits;
    }
}
