//! ComboBox: lista suspensa de opções.

use std::any::Any;

use aylis_core::{
    Brush, Canvas, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, Path,
    PointF, PointerEvent, PropEnum, RectF, Stroke, TextAlign, Trimming, properties,
};

/// O desenho da seta do [`ComboBox`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComboArrow {
    /// "v" de traço, que vira para cima com a lista aberta.
    #[default]
    Chevron,
    /// Triângulo cheio na cor do texto (o `<select>` das páginas web).
    Triangle,
}

impl PropEnum for ComboArrow {
    const NAMES: &'static [&'static str] = &["chevron", "triangle"];
    fn from_index(i: usize) -> Option<Self> {
        [ComboArrow::Chevron, ComboArrow::Triangle].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

use crate::leaf_control;

const ROW: f32 = 28.0;
/// As opções à vista na lista aberta, sem `drop_down_count` (o `DropDownCount` do Delphi).
const DEFAULT_ROWS: i64 = 10;
/// A largura da barra de rolagem da lista aberta.
const BAR: f32 = 6.0;
/// A folga da lista até a borda do form.
const EDGE: f32 = 4.0;
/// O lugar da seta, à direita do texto.
pub const ARROW_ROOM: f32 = 18.0;

/// Lista suspensa (como o `TComboBox` com `Style = csDropDownList`).
///
/// - `items`: opções separadas por `;` (ou pela API, [`ComboBox::set_items`]);
/// - `item_index`: opção escolhida (-1 = nenhuma); `on_change` dispara ao mudar;
/// - clique ou Espaço/Enter abrem a lista; setas trocam a opção com a lista fechada;
/// - `drop_down_count`: quantas opções a lista aberta mostra de uma vez (10); com mais, ela
///   rola (roda, dedo, a barra à direita) e, sem lugar no form, encolhe para caber.
/// - `values`: o código de cada opção (separados por `;`, como `items`); `value` lê e escolhe
///   pelo código (sem `values`, pelo texto). Ligado a um dataset, `items` e `values` vêm da
///   lista e `value` grava o código (o `TDBLookupComboBox`).
pub struct ComboBox {
    pub base: ControlBase,
    pub items: String,
    /// O código de cada opção (`;`); vazio: o código é o próprio texto.
    pub values: String,
    pub item_index: i64,
    /// Alinhamento do texto escolhido.
    pub h_align: TextAlign,
    /// Reticências quando o texto escolhido não couber.
    pub trimming: Trimming,
    /// O desenho da seta.
    pub arrow: ComboArrow,
    /// Dado pessoal (LGPD): as gravações do `EventRecorder` mascaram o que é digitado aqui,
    /// a não ser com o `record_sensitive` dele ligado. Liga e desliga em tempo de execução.
    pub sensitive: bool,
    /// O grupo de cada opção (`;`, na ordem de `items`): a lista mostra o nome do grupo, sem
    /// clique, antes da primeira opção dele (o `<optgroup>`). Vazio: sem grupos.
    pub groups: String,
    /// As opções desligadas, pelos índices (`"2;5"`): aparecem fracas e não se escolhem.
    pub disabled_items: String,
    /// Quantas linhas a lista aberta mostra de uma vez (o `DropDownCount` do Delphi; mínimo 1).
    /// Com mais opções, ela rola; sem lugar no form, mostra menos.
    pub drop_down_count: i64,
    popup: Option<ComponentId>,
}

impl Default for ComboBox {
    fn default() -> Self {
        let mut base = ControlBase::new(160.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, items: String::new(), values: String::new(), item_index: -1, h_align: TextAlign::Leading, trimming: Trimming::Character, arrow: ComboArrow::Chevron, sensitive: false, groups: String::new(), disabled_items: String::new(), drop_down_count: DEFAULT_ROWS, popup: None }
    }
}

/// Mensagem da lista aberta para o ComboBox dono.
#[derive(Debug)]
struct Picked(Option<usize>);

impl ComboBox {
    pub fn new(items: &[&str]) -> Self {
        Self { items: items.join(";"), ..Default::default() }
    }

    pub fn set_items(&mut self, items: &[String]) {
        self.items = items.join(";");
        if self.item_index >= items.len() as i64 {
            self.item_index = -1;
        }
    }

    pub fn item_list(&self) -> Vec<&str> {
        // `;` ou quebra de linha (com a quebra, os itens sumiam sem erro; relatório do InsightDB,
        // 11/11).
        if self.items.is_empty() { Vec::new() } else { self.items.split([';', '\n']).map(|s| s.trim_end_matches('\r')).collect() }
    }

    /// Texto da opção escolhida.
    pub fn text(&self) -> &str {
        usize::try_from(self.item_index).ok().and_then(|i| self.item_list().get(i).copied()).unwrap_or("")
    }

    /// O código da opção escolhida (o texto, sem `values`).
    pub fn value(&self) -> String {
        let Ok(i) = usize::try_from(self.item_index) else { return String::new() };
        if self.values.is_empty() {
            return self.text().to_string();
        }
        self.values.split(';').nth(i).unwrap_or("").to_string()
    }

    /// Escolhe a opção pelo código (sem `values`, pelo texto); código desconhecido: nenhuma.
    pub fn set_value(&mut self, value: String) {
        let list: Vec<&str> = if self.values.is_empty() { self.item_list() } else { self.values.split(';').collect() };
        self.item_index = list.iter().position(|v| *v == value).map_or(-1, |i| i as i64);
    }

    /// Escolhe a opção pelo texto. Devolve `false` se não existir.
    pub fn select_text(&mut self, text: &str) -> bool {
        match self.item_list().iter().position(|i| *i == text) {
            Some(i) => {
                self.item_index = i as i64;
                true
            }
            None => false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.popup.is_some()
    }

    /// A opção `index` pode ser escolhida (não está em `disabled_items`).
    pub fn item_enabled(&self, index: usize) -> bool {
        !self.disabled_items.split(';').any(|s| s.trim().parse::<usize>() == Ok(index))
    }

    /// Liga ou desliga a opção `index`.
    pub fn set_item_enabled(&mut self, index: usize, enabled: bool) {
        let mut off: Vec<usize> = self.disabled_items.split(';').filter_map(|s| s.trim().parse().ok()).filter(|&i| i != index).collect();
        if !enabled {
            off.push(index);
            off.sort_unstable();
        }
        self.disabled_items = off.iter().map(usize::to_string).collect::<Vec<_>>().join(";");
    }

    /// As linhas da lista aberta: a cabeça de cada grupo (sem índice) e as opções.
    fn rows(&self) -> Vec<PopupRow> {
        let groups: Vec<&str> = if self.groups.is_empty() { Vec::new() } else { self.groups.split(';').collect() };
        let mut rows = Vec::new();
        let mut current: Option<&str> = None;
        for (i, text) in self.item_list().into_iter().enumerate() {
            let group = groups.get(i).copied().unwrap_or("");
            if !group.is_empty() && current != Some(group) {
                rows.push(PopupRow { item: None, text: group.to_string(), enabled: false });
            }
            current = Some(group);
            rows.push(PopupRow { item: Some(i), text: text.to_string(), enabled: self.item_enabled(i) });
        }
        rows
    }

    /// A próxima opção ligada a partir de `from`, andando `step` (±1); nenhuma: `None`.
    fn next_enabled(&self, from: i64, step: i64) -> Option<usize> {
        let count = self.item_list().len() as i64;
        let mut i = from + step;
        while (0..count).contains(&i) {
            if self.item_enabled(i as usize) {
                return Some(i as usize);
            }
            i += step;
        }
        None
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if index as i64 != self.item_index && index < self.item_list().len() && self.item_enabled(index) {
            self.item_index = index as i64;
            ctx.fire("on_change", EventArgs::Text(self.text().to_string()));
        }
        ctx.repaint();
    }

    fn open(&mut self, ctx: &mut EventCtx<'_>) {
        let rows = self.rows();
        let count = rows.len();
        if count == 0 || self.popup.is_some() {
            return;
        }
        let selected = rows.iter().position(|r| r.item.is_some_and(|i| i as i64 == self.item_index));
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.base.bounds());
        let root_height = ctx.ui.bounds(root).map_or(f32::MAX, |b| b.height);
        let wanted = count.min(self.drop_down_count.max(1) as usize);
        let (top, visible) = drop_place(rect, root_height, wanted);
        let height = popup_height(visible);
        let popup = ComboPopup {
            base: ControlBase::new(rect.width, height),
            owner: ctx.id,
            items: rows,
            selected,
            hovered: None,
            first: selected.unwrap_or(0).saturating_sub(visible - 1),
            visible,
            touch_rest: 0.0,
            dragging: None,
            bar_press: false,
        };
        let id = ctx.ui.add(popup, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(rect.x, top, rect.width, height);
        }
        ctx.ui.follow(id, ctx.id);
        self.popup = Some(id);
        ctx.repaint();
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(popup) = self.popup.take() {
            ctx.ui.free(popup);
            ctx.repaint();
        }
    }
}

impl Component for ComboBox {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.text().to_string()))
    }

    leaf_control!("ComboBox");

    properties! {
        items: String => "Data",
        values: String => "Data",
        item_index: i64 => "Data",
        h_align: TextAlign => "Text",
        trimming: Trimming => "Text",
        arrow: ComboArrow => "Appearance",
        sensitive: bool => "Behavior",
        groups: String => "Data",
        disabled_items: String => "Data",
        drop_down_count: i64 => "Behavior";
        events: on_change;
        computed: value: String => value / set_value
    }

    /// `auto_size`: a opção mais larga, o padding do estilo e o lugar da seta (C21 do
    /// Tabernáculo; antes, a conta à mão no app).
    fn auto_width(&self, ui: &aylis_core::Ui, id: ComponentId) -> Option<f32> {
        let style = aylis_core::TextStyle { font: ui.font_of(id), ..Default::default() };
        let widest = self.item_list().iter().map(|t| ui.measure_text(t, &style).width).fold(0.0f32, f32::max);
        Some((widest + ui.style_padding(id).horizontal() + ARROW_ROOM + 2.0).ceil())
    }

    fn auto_height(&self, ui: &aylis_core::Ui, id: ComponentId, _width: f32) -> Option<f32> {
        let style = aylis_core::TextStyle { font: ui.font_of(id), ..Default::default() };
        Some((ui.measure_text("Ág", &style).height + ui.style_padding(id).vertical()).ceil().max(24.0))
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() { self.close(ctx) } else { self.open(ctx) }
        ctx.fire("on_click", EventArgs::None);
    }

    // As opções para leitores de tela e automação: escolher uma não precisa abrir a lista.
    fn access_items(&self, _: &aylis_core::Ui, _: ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        self.item_list().iter().enumerate().map(|(i, t)| AccessItem::new(ItemRole::ListItem, *t).selected(i as i64 == self.item_index).disabled(!self.item_enabled(i))).collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        if index >= self.item_list().len() || !self.item_enabled(index) {
            return false;
        }
        self.choose(ctx, index);
        self.close(ctx);
        true
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        // As setas pulam as opções desligadas (C20 do Tabernáculo).
        match event.key {
            Key::Up => {
                if let Some(i) = self.next_enabled(self.item_index.max(0), -1).or_else(|| self.next_enabled(-1, 1).filter(|_| self.item_index < 0)) {
                    self.choose(ctx, i);
                }
            }
            Key::Down => {
                if let Some(i) = self.next_enabled(self.item_index, 1) {
                    self.choose(ctx, i);
                }
            }
            Key::Enter | Key::Space => {
                if self.popup.is_some() { self.close(ctx) } else { self.open(ctx) }
            }
            Key::Escape if self.popup.is_some() => self.close(ctx),
            _ => return,
        }
        ctx.set_handled();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.close(ctx);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(Picked(choice)) = message.downcast_ref::<Picked>() {
            if let Some(i) = choice {
                self.choose(ctx, *i);
            }
            self.close(ctx);
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.popup == Some(freed) {
            self.popup = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        canvas.fill_rect(ctx.rect, s.radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 {
            canvas.stroke_rect(ctx.rect, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let inner = ctx.rect.deflate(s.padding);
        let mut text = ctx.text_style();
        text.h_align = self.h_align;
        text.trimming = self.trimming;
        canvas.draw_text(self.text(), RectF::new(inner.x, inner.y, inner.width - ARROW_ROOM, inner.height), &text);
        // seta
        if self.arrow == ComboArrow::Triangle {
            // 10 × 5, a 8 px da direita.
            let (x, y) = (ctx.rect.right() - 18.0, ctx.rect.center().y - 3.5);
            let mut tri = Path::new();
            tri.move_to(x, y).line_to(x + 10.0, y).line_to(x + 5.0, y + 5.0).close();
            canvas.fill_path(&tri, &Brush::Solid(s.text));
            return;
        }
        let c = PointF::new(ctx.rect.right() - 16.0, ctx.rect.center().y);
        let stroke = Stroke::new(ctx.theme.color("text_muted"), 1.6);
        let (dy, up) = if self.popup.is_some() { (-2.0, true) } else { (2.0, false) };
        let tip = if up { c.y - 2.0 } else { c.y + 2.0 };
        canvas.draw_line(PointF::new(c.x - 4.5, tip - dy * 1.5), PointF::new(c.x, tip), &stroke);
        canvas.draw_line(PointF::new(c.x, tip), PointF::new(c.x + 4.5, tip - dy * 1.5), &stroke);
    }
}

/// A altura da lista aberta com `rows` linhas.
fn popup_height(rows: usize) -> f32 {
    ROW * rows as f32 + 4.0
}

/// Onde a lista abre (o topo, no form) e quantas linhas mostra, para um ComboBox em `rect`
/// (no form) num form de altura `form_height`, querendo `wanted` linhas: embaixo, se couber;
/// senão em cima, se couber; senão do lado com mais lugar, com as linhas que couberem (ao menos
/// uma).
fn drop_place(rect: RectF, form_height: f32, wanted: usize) -> (f32, usize) {
    let below = form_height - rect.bottom() - 2.0 - EDGE;
    let above = rect.y - 2.0 - EDGE;
    let height = popup_height(wanted);
    let fit = |room: f32| (((room - 4.0) / ROW).floor().max(1.0) as usize).min(wanted);
    if height <= below {
        (rect.bottom() + 2.0, wanted)
    } else if height <= above {
        (rect.y - height - 2.0, wanted)
    } else if below >= above {
        (rect.bottom() + 2.0, fit(below))
    } else {
        let rows = fit(above);
        (rect.y - popup_height(rows) - 2.0, rows)
    }
}

/// Uma linha da lista aberta: uma opção (com o índice dela) ou a cabeça de um grupo.
struct PopupRow {
    item: Option<usize>,
    text: String,
    enabled: bool,
}

/// A lista aberta (criada como filha do form, por cima de tudo).
struct ComboPopup {
    base: ControlBase,
    owner: ComponentId,
    items: Vec<PopupRow>,
    selected: Option<usize>,
    hovered: Option<usize>,
    first: usize,
    /// As linhas à vista (o `drop_down_count`, ou menos se o form não tem lugar).
    visible: usize,
    /// Arrasto do dedo ainda não convertido em linhas.
    touch_rest: f32,
    /// A barra sendo arrastada: onde o polegar foi pego (y dentro dele).
    dragging: Option<f32>,
    /// O último aperto foi na barra: o clique que vem dele não escolhe a linha.
    bar_press: bool,
}

impl ComboPopup {
    fn index_at(&self, y: f32) -> Option<usize> {
        let row = ((y - 2.0) / ROW).max(0.0) as usize;
        let i = self.first + row;
        (row < self.visible && i < self.items.len()).then_some(i)
    }

    /// A primeira linha mais alta possível (a lista no fim).
    fn max_first(&self) -> usize {
        self.items.len().saturating_sub(self.visible)
    }

    /// O polegar da barra de rolagem (no controle), se há mais linhas que as à vista.
    fn thumb(&self) -> Option<RectF> {
        let total = self.items.len();
        if total <= self.visible {
            return None;
        }
        let b = self.base.bounds();
        let track = b.height - 4.0;
        let len = (track * self.visible as f32 / total as f32).max(20.0).min(track);
        let at = 2.0 + (track - len) * self.first as f32 / self.max_first().max(1) as f32;
        Some(RectF::new(b.width - BAR - 3.0, at, BAR, len))
    }

    /// Põe o centro do polegar em `y` (no controle): a primeira linha proporcional. Devolve se
    /// mudou.
    fn drag_to(&mut self, y: f32) -> bool {
        let Some(thumb) = self.thumb() else { return false };
        let track = self.base.bounds().height - 4.0;
        let fraction = ((y - 2.0 - thumb.height / 2.0) / (track - thumb.height).max(1.0)).clamp(0.0, 1.0);
        let first = (fraction * self.max_first() as f32).round() as usize;
        let changed = first != self.first;
        self.first = first;
        changed
    }
}

impl Component for ComboPopup {
    leaf_control!("ComboPopup");

    // Some junto com quem a abriu (a tela refeita não deixa a lista órfã).
    fn dies_with(&self) -> Option<ComponentId> {
        Some(self.owner)
    }

    // O clique fora fecha a lista, mesmo onde não recebe foco (o fundo da tela, um rótulo):
    // antes só a perda de foco do ComboBox fechava.
    fn closes_on_outside_press(&self) -> bool {
        true
    }

    // A barra: pegar o polegar e arrastar (na calha, fora do polegar, pula para lá).
    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.bar_press = false;
        let Some(thumb) = self.thumb() else { return };
        if event.position.x < thumb.x - 3.0 {
            return;
        }
        self.bar_press = true;
        if (thumb.y..thumb.bottom()).contains(&event.position.y) {
            self.dragging = Some(event.position.y - thumb.y);
        } else {
            self.dragging = Some(thumb.height / 2.0);
            self.drag_to(event.position.y);
        }
        ctx.repaint();
        ctx.set_handled();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging.take().is_some() {
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(grab) = self.dragging {
            if self.drag_to(event.position.y - grab + self.thumb().map_or(0.0, |t| t.height / 2.0)) {
                ctx.repaint();
            }
            return;
        }
        if self.thumb().is_some_and(|t| event.position.x >= t.x - 3.0) {
            if self.hovered.take().is_some() {
                ctx.repaint();
            }
            return;
        }
        let hovered = self.index_at(event.position.y);
        if hovered != self.hovered {
            self.hovered = hovered;
            ctx.repaint();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::WheelEvent) {
        let max = self.max_first();
        // Dedo: a lista anda junto (uma linha a cada ROW); roda: uma linha por passo.
        let step = if event.touch {
            self.touch_rest -= event.delta.y;
            let rows = (self.touch_rest / ROW).trunc();
            self.touch_rest -= rows * ROW;
            rows as isize
        } else if event.delta.y > 0.0 {
            -1
        } else {
            1
        };
        self.first = (self.first as isize + step).clamp(0, max as isize) as usize;
        ctx.repaint();
        ctx.set_handled();
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        // Soltar a barra não escolhe nada.
        if self.bar_press {
            return;
        }
        // A cabeça de um grupo e a opção desligada não escolhem nem fecham.
        let row = self.hovered.and_then(|h| self.items.get(h));
        if row.is_some_and(|r| !r.enabled) {
            return;
        }
        let choice = row.and_then(|r| r.item);
        let owner = self.owner;
        ctx.ui.send_message(owner, &Picked(choice));
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let theme = ctx.theme;
        let r = ctx.rect;
        canvas.fill_rect(r.offset(0.0, 3.0).inflate(1.0), 7.0, &Brush::Solid(aylis_core::Color::BLACK.with_opacity(0.12)));
        canvas.fill_rect(r, 6.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r, 6.0, &Stroke::new(theme.color("border"), 1.0));
        let mut text = ctx.text_style();
        text.color = theme.color("text");
        // Com a barra, as linhas param antes dela.
        let thumb = self.thumb();
        let room = if thumb.is_some() { BAR + 4.0 } else { 0.0 };
        for (row, i) in (self.first..self.items.len()).take(self.visible).enumerate() {
            let line = RectF::new(r.x + 2.0, r.y + 2.0 + row as f32 * ROW, r.width - 4.0 - room, ROW);
            let item = &self.items[i];
            if item.item.is_none() {
                let mut head = text.clone();
                head.color = theme.color("text_muted");
                head.font.weight = aylis_core::FontWeight::SemiBold;
                canvas.draw_text(&item.text, line.deflate(aylis_core::Bounds::symmetric(8.0, 0.0)), &head);
                continue;
            }
            if Some(i) == self.hovered && item.enabled {
                canvas.fill_rect(line, 4.0, &Brush::Solid(theme.color("primary").with_opacity(0.15)));
            } else if Some(i) == self.selected {
                canvas.fill_rect(line, 4.0, &Brush::Solid(theme.color("text").with_opacity(0.06)));
            }
            let mut t = text.clone();
            if !item.enabled {
                t.color = theme.color("text_muted");
            }
            // Num grupo, as opções ficam recuadas sob a cabeça.
            let indent = if self.items.iter().any(|r| r.item.is_none()) { 18.0 } else { 10.0 };
            canvas.draw_text(&item.text, RectF::new(line.x + indent, line.y, line.width - indent - 10.0, line.height), &t);
        }
        if let Some(thumb) = thumb {
            let alpha = if self.dragging.is_some() { 0.6 } else { 0.35 };
            canvas.fill_rect(thumb.offset(r.x, r.y), BAR / 2.0, &Brush::Solid(theme.color("text_muted").with_opacity(alpha)));
        }
    }
}
