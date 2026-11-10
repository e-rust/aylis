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
const MAX_ROWS: usize = 10;

/// Lista suspensa (como o `TComboBox` com `Style = csDropDownList`).
///
/// - `items`: opções separadas por `;` (ou pela API, [`ComboBox::set_items`]);
/// - `item_index`: opção escolhida (-1 = nenhuma); `on_change` dispara ao mudar;
/// - clique ou Espaço/Enter abrem a lista; setas trocam a opção com a lista fechada.
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
    popup: Option<ComponentId>,
}

impl Default for ComboBox {
    fn default() -> Self {
        let mut base = ControlBase::new(160.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, items: String::new(), values: String::new(), item_index: -1, h_align: TextAlign::Leading, trimming: Trimming::Character, arrow: ComboArrow::Chevron, sensitive: false, popup: None }
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
        if self.items.is_empty() { Vec::new() } else { self.items.split(';').collect() }
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

    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if index as i64 != self.item_index && index < self.item_list().len() {
            self.item_index = index as i64;
            ctx.fire("on_change", EventArgs::Text(self.text().to_string()));
        }
        ctx.repaint();
    }

    fn open(&mut self, ctx: &mut EventCtx<'_>) {
        let count = self.item_list().len();
        if count == 0 || self.popup.is_some() {
            return;
        }
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.base.bounds());
        let height = ROW * count.min(MAX_ROWS) as f32 + 4.0;
        let root_height = ctx.ui.bounds(root).map_or(f32::MAX, |b| b.height);
        // Abre para cima se não couber embaixo.
        let top = if rect.bottom() + height > root_height && rect.y - height > 0.0 { rect.y - height - 2.0 } else { rect.bottom() + 2.0 };
        let popup = ComboPopup {
            base: ControlBase::new(rect.width, height),
            owner: ctx.id,
            items: self.item_list().iter().map(|s| s.to_string()).collect(),
            selected: usize::try_from(self.item_index).ok(),
            hovered: None,
            first: usize::try_from(self.item_index).unwrap_or(0).saturating_sub(MAX_ROWS - 1),
            touch_rest: 0.0,
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
        sensitive: bool => "Behavior";
        events: on_change;
        computed: value: String => value / set_value
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() { self.close(ctx) } else { self.open(ctx) }
        ctx.fire("on_click", EventArgs::None);
    }

    // As opções para leitores de tela e automação: escolher uma não precisa abrir a lista.
    fn access_items(&self, _: &aylis_core::Ui, _: ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        self.item_list().iter().enumerate().map(|(i, t)| AccessItem::new(ItemRole::ListItem, *t).selected(i as i64 == self.item_index)).collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        if index >= self.item_list().len() {
            return false;
        }
        self.choose(ctx, index);
        self.close(ctx);
        true
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let count = self.item_list().len();
        match event.key {
            Key::Up => {
                let i = (self.item_index - 1).max(0) as usize;
                self.choose(ctx, i);
            }
            Key::Down if count > 0 => {
                let i = (self.item_index + 1).min(count as i64 - 1) as usize;
                self.choose(ctx, i);
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
        canvas.draw_text(self.text(), RectF::new(inner.x, inner.y, inner.width - 18.0, inner.height), &text);
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

/// A lista aberta (criada como filha do form, por cima de tudo).
struct ComboPopup {
    base: ControlBase,
    owner: ComponentId,
    items: Vec<String>,
    selected: Option<usize>,
    hovered: Option<usize>,
    first: usize,
    /// Arrasto do dedo ainda não convertido em linhas.
    touch_rest: f32,
}

impl ComboPopup {
    fn index_at(&self, y: f32) -> Option<usize> {
        let i = self.first + ((y - 2.0) / ROW).max(0.0) as usize;
        (i < self.items.len()).then_some(i)
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

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hovered = self.index_at(event.position.y);
        if hovered != self.hovered {
            self.hovered = hovered;
            ctx.repaint();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::WheelEvent) {
        let max = self.items.len().saturating_sub(MAX_ROWS);
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
        let choice = self.hovered;
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
        for (row, i) in (self.first..self.items.len()).take(MAX_ROWS).enumerate() {
            let line = RectF::new(r.x + 2.0, r.y + 2.0 + row as f32 * ROW, r.width - 4.0, ROW);
            if Some(i) == self.hovered {
                canvas.fill_rect(line, 4.0, &Brush::Solid(theme.color("primary").with_opacity(0.15)));
            } else if Some(i) == self.selected {
                canvas.fill_rect(line, 4.0, &Brush::Solid(theme.color("text").with_opacity(0.06)));
            }
            canvas.draw_text(&self.items[i], line.deflate(aylis_core::Bounds::symmetric(10.0, 0.0)), &text);
        }
    }
}
