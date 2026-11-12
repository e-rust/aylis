//! Componentes de interface moderna: ícone, selo, aviso, avatar, seletor segmentado,
//! indicador de atividade, medidor de nível, trilha de navegação, barra de mídia, campo de
//! atalho e direcional.
//!
//! Todos tiram as cores do tema (tokens como `primary`, `success_soft`, `meter_low`) e
//! aceitam variantes (`style`) que aparecem como subtipos na paleta.

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Image, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerButton, PointerEvent, PropEnum, RectF, Stroke, TextAlign, Trimming,
    properties,
};

use crate::leaf_control;

/// Cor de destaque de uma variante semântica (`success`, `warning`, `danger`, `info`,
/// `primary`); outras caem em `default`.
fn tone<'a>(variant: &'a str, default: &'a str) -> &'a str {
    match variant {
        "primary" | "success" | "warning" | "danger" | "info" => variant,
        _ => default,
    }
}

/// Divide uma lista "a;b;c" (vazia = nenhum item).
fn split_items(items: &str) -> Vec<&str> {
    // `;` ou quebra de linha.
    if items.is_empty() { Vec::new() } else { items.split([';', '\n']).map(|s| s.trim_end_matches('\r')).collect() }
}

// ===========================================================================
// Icon
// ===========================================================================

/// Um ícone sozinho (como um `TPath` com dados prontos). `icon` é o nome (os do aylis ou
/// os registrados com `Ui::register_icon`); a cor vem do tema (`text`) ou de `color`.
pub struct IconView {
    pub base: ControlBase,
    pub icon: String,
    pub color: Option<Color>,
}

impl Default for IconView {
    fn default() -> Self {
        let mut base = ControlBase::new(24.0, 24.0);
        base.set_hit_test(false);
        Self { base, icon: "star".into(), color: None }
    }
}

impl IconView {
    pub fn new(icon: impl Into<String>) -> Self {
        Self { icon: icon.into(), ..Default::default() }
    }
}

impl Component for IconView {
    leaf_control!("Icon");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        icon: String => "Appearance",
        color: Option<Color> => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let token = tone(ctx.variant(), "");
        let color = self.color.unwrap_or_else(|| if token.is_empty() { ctx.style.text } else { ctx.theme.color(token) });
        ctx.draw_icon(canvas, &self.icon, ctx.rect, color);
    }
}

// ===========================================================================
// Badge
// ===========================================================================

/// Selo (etiqueta, "chip"): um texto curto numa pílula, com ícone ou bolinha opcionais.
/// As variantes `primary`, `success`, `warning`, `danger`, `info` e `outline` mudam a cor.
///
/// Com `target` (um controle irmão) ele fica preso ao canto dele (o `v-badge`): o "3"
/// sobre o sino de notificações. `anchor` escolhe o canto; sem texto, vira uma bolinha.
pub struct Badge {
    pub base: ControlBase,
    pub text: String,
    pub icon: String,
    /// Bolinha colorida antes do texto (estado: no ar, gravando...).
    pub dot: bool,
    pub target: Option<ComponentId>,
    pub anchor: BadgeAnchor,
}

/// O canto do alvo onde o `Badge` fica.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeAnchor {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl PropEnum for BadgeAnchor {
    const NAMES: &'static [&'static str] = &["top_right", "top_left", "bottom_right", "bottom_left"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::TopRight, Self::TopLeft, Self::BottomRight, Self::BottomLeft].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl Default for Badge {
    fn default() -> Self {
        let mut base = ControlBase::new(72.0, 22.0);
        base.set_hit_test(false);
        Self { base, text: "Selo".into(), icon: String::new(), dot: false, target: None, anchor: BadgeAnchor::TopRight }
    }
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// Preso a `target`: o tamanho segue o texto (bolinha sem texto).
    fn sync(&mut self, name: &str) {
        if self.target.is_some() && matches!(name, "target" | "text") {
            let n = self.text.chars().count();
            let (w, h) = if n == 0 { (10.0, 10.0) } else { ((n as f32 * 7.0 + 10.0).max(18.0), 18.0) };
            self.base.set_size(w, h);
        }
    }
}

impl Component for Badge {
    leaf_control!("Badge");

    properties! {
        text: String => "Appearance",
        icon: String => "Appearance",
        dot: bool => "Appearance",
        target: Option<ComponentId> => "Layout",
        anchor: BadgeAnchor => "Layout";
        on_set: sync
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.target == Some(freed) {
            self.target = None;
        }
    }

    fn attach(&self) -> Option<(ComponentId, f32, f32)> {
        let (fx, fy) = match self.anchor {
            BadgeAnchor::TopRight => (1.0, 0.0),
            BadgeAnchor::TopLeft => (0.0, 0.0),
            BadgeAnchor::BottomRight => (1.0, 1.0),
            BadgeAnchor::BottomLeft => (0.0, 1.0),
        };
        // Um pouco para dentro do canto, como no Vuetify.
        let inset = 0.12;
        Some((self.target?, fx + if fx > 0.5 { -inset } else { inset }, fy + if fy > 0.5 { -inset } else { inset }))
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        if self.target.is_some() {
            // Preso: pílula cheia na cor de destaque, com a borda do fundo em volta.
            let color = ctx.theme.color(tone(ctx.variant(), "danger"));
            canvas.fill_rect(r.inflate(1.5), r.height / 2.0 + 1.5, &Brush::Solid(ctx.theme.color("background")));
            canvas.fill_rect(r, r.height / 2.0, &Brush::Solid(color));
            if !self.text.is_empty() {
                let mut t = ctx.text_style();
                t.h_align = TextAlign::Center;
                t.color = Color::WHITE;
                t.font.size = (r.height * 0.62).round();
                t.font.weight = FontWeight::SemiBold;
                canvas.draw_text(&self.text, r, &t);
            }
            return;
        }
        let radius = s.radius.min(r.height / 2.0);
        canvas.fill_rect(r, radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 && !s.border_color.is_transparent() {
            canvas.stroke_rect(r, radius, &Stroke::new(s.border_color, s.border_width));
        }
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        let inner = r.deflate(s.padding);
        let side = (text.font.size * 1.1).round();
        let mut lead = 0.0;
        if self.dot {
            lead += 8.0 + 5.0;
        }
        if !self.icon.is_empty() {
            lead += side + 4.0;
        }
        let tw = canvas.measure_text(&self.text, &text).width.min(inner.width - lead).max(0.0);
        let mut x = inner.x + ((inner.width - lead - tw) / 2.0).max(0.0);
        let cy = inner.y + inner.height / 2.0;
        if self.dot {
            canvas.fill_ellipse(RectF::new(x, cy - 4.0, 8.0, 8.0), &Brush::Solid(s.text));
            x += 13.0;
        }
        if !self.icon.is_empty() {
            ctx.draw_icon(canvas, &self.icon, RectF::new(x, cy - side / 2.0, side, side), s.text);
            x += side + 4.0;
        }
        text.h_align = TextAlign::Leading;
        canvas.draw_text(&self.text, RectF::new(x, inner.y, tw + 1.0, inner.height), &text);
    }
}

// ===========================================================================
// Banner
// ===========================================================================

/// Aviso em faixa (alerta): ícone, título e texto, com a cor da variante (`info` padrão,
/// `success`, `warning`, `danger`). Com `closable`, um "×" esconde a faixa e dispara
/// `on_close`.
pub struct Banner {
    pub base: ControlBase,
    pub title: String,
    pub text: String,
    /// Ícone; vazio = o da variante (info, success, warning, error).
    pub icon: String,
    pub closable: bool,
    close_rect: Cell<Option<RectF>>,
}

impl Default for Banner {
    fn default() -> Self {
        Self {
            base: ControlBase::new(360.0, 64.0),
            title: "Aviso".into(),
            text: String::new(),
            icon: String::new(),
            closable: false,
            close_rect: Cell::new(None),
        }
    }
}

impl Banner {
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self { title: title.into(), text: text.into(), ..Default::default() }
    }
}

impl Component for Banner {
    leaf_control!("Banner");

    properties! {
        title: String => "Appearance",
        text: String => "Appearance",
        icon: String => "Appearance",
        closable: bool => "Behavior";
        events: on_close
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.close_rect.get().is_some_and(|r| r.inflate(4.0).contains(event.position)) {
            self.base.set_visible(false);
            ctx.ui.invalidate();
            ctx.fire("on_close", EventArgs::None);
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        let token = tone(ctx.variant(), "info");
        // As cores do tipo (informação, sucesso...), trocadas pelas próprias se houver.
        let accent = ctx.colors.accent.unwrap_or_else(|| ctx.theme.color(token));
        let fill = ctx.colors.fill.unwrap_or_else(|| ctx.theme.color(&format!("{token}_soft")));
        canvas.fill_rect(r, s.radius, &Brush::Solid(fill));
        canvas.stroke_rect(r, s.radius, &Stroke::new(ctx.colors.border.unwrap_or(accent.with_opacity(0.35)), 1.0));
        canvas.save();
        canvas.clip_round_rect(r, s.radius);
        canvas.fill_rect(RectF::new(r.x, r.y, 4.0, r.height), 0.0, &Brush::Solid(accent));
        canvas.restore();
        let inner = r.deflate(s.padding);
        let icon = if !self.icon.is_empty() {
            self.icon.as_str()
        } else {
            match token {
                "success" => "success",
                "warning" => "warning",
                "danger" => "error",
                _ => "info",
            }
        };
        let side = 20.0;
        ctx.draw_icon(canvas, icon, RectF::new(inner.x + 4.0, inner.y + 1.0, side, side), accent);
        let left = inner.x + side + 14.0;
        let mut right = inner.right();
        if self.closable {
            let c = RectF::new(inner.right() - 16.0, inner.y + 2.0, 16.0, 16.0);
            ctx.draw_icon(canvas, "close", c, ctx.theme.color("text_muted"));
            self.close_rect.set(Some(c));
            right -= 24.0;
        } else {
            self.close_rect.set(None);
        }
        let width = (right - left).max(0.0);
        let mut title = ctx.text_style();
        title.font.weight = FontWeight::SemiBold;
        title.v_align = TextAlign::Leading;
        let line = title.font.size * 1.45;
        let mut y = inner.y;
        if !self.title.is_empty() {
            canvas.draw_text(&self.title, RectF::new(left, y, width, line), &title);
            y += line;
        }
        if !self.text.is_empty() {
            let mut body = ctx.text_style();
            body.color = ctx.theme.color("text_muted");
            body.v_align = TextAlign::Leading;
            body.wrap = true;
            body.trimming = Trimming::Word;
            canvas.draw_text(&self.text, RectF::new(left, y, width, (inner.bottom() - y).max(line)), &body);
        }
    }
}

// ===========================================================================
// Avatar
// ===========================================================================

/// Presença mostrada no canto do avatar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Presence {
    #[default]
    None,
    Online,
    Away,
    Busy,
    Offline,
}

impl PropEnum for Presence {
    const NAMES: &'static [&'static str] = &["none", "online", "away", "busy", "offline"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Online, Self::Away, Self::Busy, Self::Offline].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Foto ou iniciais de uma pessoa, redondo (ou quadrado arredondado com a variante
/// `square`), com a presença no canto. Sem foto, as iniciais de `display_name` sobre uma cor
/// tirada do próprio nome (a mesma pessoa tem sempre a mesma cor).
pub struct Avatar {
    pub base: ControlBase,
    /// Nome da pessoa (as iniciais e a cor saem dele). Não é o `name` do componente.
    pub display_name: String,
    /// Foto do aplicativo (`assets/...`); vazio = iniciais.
    pub image: String,
    pub presence: Presence,
    picture: Option<Image>,
    loaded: RefCell<Option<(String, Option<Image>)>>,
}

impl Default for Avatar {
    fn default() -> Self {
        let mut base = ControlBase::new(40.0, 40.0);
        base.set_hit_test(false);
        Self {
            base,
            display_name: "Aylis".into(),
            image: String::new(),
            presence: Presence::None,
            picture: None,
            loaded: RefCell::new(None),
        }
    }
}

/// Cores próprias das iniciais (boas no claro e no escuro, com texto branco).
const AVATAR_COLORS: [Color; 8] = [
    Color::rgb(0x25, 0x63, 0xEB),
    Color::rgb(0x93, 0x33, 0xEA),
    Color::rgb(0xDB, 0x27, 0x77),
    Color::rgb(0xEA, 0x58, 0x0C),
    Color::rgb(0x0D, 0x94, 0x88),
    Color::rgb(0x16, 0xA3, 0x4A),
    Color::rgb(0x4F, 0x46, 0xE5),
    Color::rgb(0x0E, 0x74, 0x90),
];

/// A cor de uma pessoa pelo nome (sempre a mesma para o mesmo nome).
pub fn name_color(name: &str) -> Color {
    let hash = name.bytes().fold(5381u32, |h, b| h.wrapping_mul(33) ^ b as u32);
    AVATAR_COLORS[hash as usize % AVATAR_COLORS.len()]
}

impl Avatar {
    pub fn new(name: impl Into<String>) -> Self {
        Self { display_name: name.into(), ..Default::default() }
    }

    /// Foto criada por código (baixada, da câmera).
    pub fn set_picture(&mut self, picture: Option<Image>) {
        self.picture = picture;
    }

    /// As iniciais: primeira letra do primeiro e do último nome.
    pub fn initials(&self) -> String {
        let words: Vec<&str> = self.display_name.split_whitespace().collect();
        let first = |w: &str| w.chars().next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default();
        match words.as_slice() {
            [] => String::new(),
            [one] => first(one),
            [a, .., b] => first(a) + &first(b),
        }
    }

    fn color(&self) -> Color {
        name_color(&self.display_name)
    }

    fn current(&self, ctx: &PaintCtx<'_>) -> Option<Image> {
        if self.picture.is_some() {
            return self.picture.clone();
        }
        if self.image.is_empty() {
            return None;
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(path, _)| *path != self.image) {
            *loaded = Some((self.image.clone(), ctx.resource(&self.image).map(Image::encoded)));
        }
        loaded.as_ref().and_then(|(_, i)| i.clone())
    }
}

impl Component for Avatar {
    leaf_control!("Avatar");

    properties! {
        display_name: String => "Appearance",
        image: String => "Appearance",
        presence: Presence => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let side = ctx.rect.width.min(ctx.rect.height);
        let r = RectF::new((ctx.rect.width - side) / 2.0, (ctx.rect.height - side) / 2.0, side, side);
        let radius = if ctx.variant() == "square" { side * 0.22 } else { side / 2.0 };
        match self.current(ctx) {
            Some(image) => {
                canvas.save();
                canvas.clip_round_rect(r, radius);
                if let Some(size) = canvas.image_size(&image) {
                    // Cobre o quadrado cortando o excesso (como `cover`).
                    let s = size.width.min(size.height);
                    let src = RectF::new((size.width - s) / 2.0, (size.height - s) / 2.0, s, s);
                    canvas.draw_image(&image, src, r, 1.0);
                }
                canvas.restore();
            }
            None => {
                // As cores próprias (`fill`, `text_color`) no lugar da cor do nome e do branco.
                canvas.fill_rect(r, radius, &Brush::Solid(ctx.colors.fill.unwrap_or_else(|| self.color())));
                let mut text = ctx.text_style();
                text.color = ctx.colors.text.unwrap_or(Color::WHITE);
                text.h_align = TextAlign::Center;
                text.font.size = (side * 0.4).max(8.0);
                text.font.weight = FontWeight::SemiBold;
                canvas.draw_text(&self.initials(), r, &text);
            }
        }
        let token = match self.presence {
            Presence::None => return,
            Presence::Online => "presence_online",
            Presence::Away => "presence_away",
            Presence::Busy => "presence_busy",
            Presence::Offline => "presence_offline",
        };
        let d = (side * 0.28).max(8.0);
        let dot = RectF::new(r.right() - d + 1.0, r.bottom() - d + 1.0, d, d);
        // Anel da cor do fundo separa a bolinha da foto.
        canvas.fill_ellipse(dot.inflate(2.0), &Brush::Solid(ctx.theme.color("surface")));
        canvas.fill_ellipse(dot, &Brush::Solid(ctx.theme.color(token)));
    }
}

// ===========================================================================
// SegmentedControl
// ===========================================================================

/// Seletor segmentado: opções lado a lado, uma escolhida (abas de modo, filtro de vista).
/// `items` = "Tela;Janela;Área", `icons` opcionais na mesma ordem. A marcação desliza até a
/// opção escolhida. Setas trocam a opção com o foco.
pub struct SegmentedControl {
    pub base: ControlBase,
    pub items: String,
    pub icons: String,
    pub item_index: i64,
    /// Posição animada da marcação (de onde saiu, quando).
    slide: Cell<(f32, f64)>,
    shown: Cell<f32>,
}

impl Default for SegmentedControl {
    fn default() -> Self {
        let mut base = ControlBase::new(240.0, 34.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, items: "Um;Dois;Três".into(), icons: String::new(), item_index: 0, slide: Cell::new((0.0, -1.0)), shown: Cell::new(0.0) }
    }
}

const SLIDE: f64 = 0.18;

impl SegmentedControl {
    /// Segmentos com contorno (iOS 7 e Material): o escolhido preenchido, com "✓" no
    /// Material.
    fn paint_outlined(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect.inflate(-0.5);
        let theme = ctx.theme;
        let material = ctx.look().is_material();
        let items = split_items(&self.items);
        if items.is_empty() {
            return;
        }
        let n = items.len();
        let w = r.width / n as f32;
        let radius = if material { r.height / 2.0 } else { ctx.style.radius };
        let line = if material { theme.color("border") } else { theme.color("primary") };
        let (sel_fill, sel_text, text) = if material {
            let fill = theme.colors.get("secondary_container").copied().unwrap_or_else(|| theme.color("primary_soft"));
            let on = theme.colors.get("on_secondary_container").copied().unwrap_or_else(|| theme.color("text"));
            (fill, on, theme.color("text"))
        } else {
            (theme.color("primary"), theme.color("on_primary"), theme.color("primary"))
        };
        let index = self.item_index.clamp(0, n as i64 - 1) as usize;
        canvas.save();
        canvas.clip_round_rect(r, radius);
        canvas.fill_rect(RectF::new(r.x + index as f32 * w, r.y, w, r.height), 0.0, &Brush::Solid(sel_fill));
        canvas.restore();
        canvas.stroke_rect(r, radius, &Stroke::new(line, 1.0));
        for i in 1..n {
            let x = r.x + i as f32 * w;
            canvas.draw_line(PointF::new(x, r.y), PointF::new(x, r.bottom()), &Stroke::new(line, 1.0));
        }
        for (i, item) in items.iter().enumerate() {
            let cell = RectF::new(r.x + i as f32 * w, r.y, w, r.height);
            let selected = i == index;
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.color = if selected { sel_text } else { text };
            if material && selected {
                let side = (t.font.size * 1.1).round();
                let tw = canvas.measure_text(item, &t).width;
                let left = cell.x + ((cell.width - side - 4.0 - tw) / 2.0).max(4.0);
                ctx.draw_icon(canvas, "check", RectF::new(left, cell.y + (cell.height - side) / 2.0, side, side), t.color);
                t.h_align = TextAlign::Leading;
                canvas.draw_text(item, RectF::new(left + side + 4.0, cell.y, (cell.right() - left - side - 4.0).max(0.0), cell.height), &t);
            } else {
                canvas.draw_text(item, cell, &t);
            }
        }
        if ctx.state.focused {
            canvas.stroke_rect(ctx.rect, radius, &Stroke::new(theme.color("focus_ring"), 2.0));
        }
    }
}

impl SegmentedControl {
    pub fn new(items: &str) -> Self {
        Self { items: items.into(), ..Default::default() }
    }

    fn count(&self) -> usize {
        split_items(&self.items).len()
    }

    fn select(&mut self, ctx: &mut EventCtx<'_>, index: i64) {
        let index = index.clamp(0, self.count() as i64 - 1);
        if index != self.item_index {
            self.slide.set((self.shown.get(), ctx.ui.time()));
            self.item_index = index;
            ctx.fire("on_change", EventArgs::Value(index.into()));
        }
        ctx.repaint();
    }
}

impl Component for SegmentedControl {
    leaf_control!("SegmentedControl");

    properties! {
        items: String => "Data",
        icons: String => "Appearance",
        item_index: i64 => "Data";
        events: on_change
    }

    // Os segmentos para leitores de tela e automação (como opções de rádio).
    fn access_items(&self, _: &aylis_core::Ui, _: ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        let items = split_items(&self.items);
        let b = self.base.bounds();
        let w = b.width / items.len().max(1) as f32;
        items
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                let on = i as i64 == self.item_index;
                AccessItem::new(ItemRole::Radio, t).checked(on).selected(on).rect(RectF::new(i as f32 * w, 0.0, w, b.height))
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        if index >= self.count() {
            return false;
        }
        self.select(ctx, index as i64);
        true
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let n = self.count();
        if n == 0 || event.button != PointerButton::Primary {
            return;
        }
        let w = self.base.bounds().width / n as f32;
        self.select(ctx, (event.position.x / w).floor() as i64);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let to = match event.key {
            Key::Left => self.item_index - 1,
            Key::Right => self.item_index + 1,
            Key::Home => 0,
            Key::End => self.count() as i64 - 1,
            _ => return,
        };
        self.select(ctx, to);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let look = ctx.look();
        if look == aylis_core::Look::Ios7 || look.is_material() {
            return self.paint_outlined(canvas, ctx);
        }
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        let items = split_items(&self.items);
        if items.is_empty() {
            return;
        }
        let icons = split_items(&self.icons);
        let n = items.len();
        let w = r.width / n as f32;
        // Marcação deslizando.
        let target = self.item_index.clamp(0, n as i64 - 1) as f32 * w;
        let (from, start) = self.slide.get();
        let t = if start < 0.0 { 1.0 } else { ((ctx.time - start) / SLIDE).clamp(0.0, 1.0) as f32 };
        let eased = 1.0 - (1.0 - t) * (1.0 - t);
        let x = from + (target - from) * eased;
        if t < 1.0 {
            ctx.request_frame();
        }
        self.shown.set(x);
        let thumb = RectF::new(x + 3.0, r.y + 3.0, w - 6.0, r.height - 6.0);
        let thumb_radius = (s.radius - 2.0).max(0.0);
        if let Some(shadow) = ctx.theme.shadows.get("sm") {
            canvas.draw_shadow(thumb, thumb_radius, shadow);
        }
        canvas.fill_rect(thumb, thumb_radius, &Brush::Solid(ctx.theme.color("surface")));
        if ctx.state.focused {
            canvas.stroke_rect(r, s.radius, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
        for (i, item) in items.iter().enumerate() {
            let cell = RectF::new(r.x + i as f32 * w, r.y, w, r.height);
            let selected = i as i64 == self.item_index;
            let mut text = ctx.text_style();
            text.h_align = TextAlign::Center;
            text.color = if selected { ctx.theme.color("text") } else { ctx.theme.color("text_muted") };
            if selected {
                text.font.weight = FontWeight::SemiBold;
            }
            match icons.get(i).filter(|i| !i.is_empty()) {
                Some(icon) => {
                    let side = (text.font.size * 1.15).round();
                    let tw = canvas.measure_text(item, &text).width;
                    let gap = if item.is_empty() { 0.0 } else { 6.0 };
                    let left = cell.x + ((cell.width - side - gap - tw) / 2.0).max(4.0);
                    ctx.draw_icon(canvas, icon, RectF::new(left, cell.y + (cell.height - side) / 2.0, side, side), text.color);
                    text.h_align = TextAlign::Leading;
                    canvas.draw_text(item, RectF::new(left + side + gap, cell.y, (cell.right() - left - side - gap).max(0.0), cell.height), &text);
                }
                None => canvas.draw_text(item, cell.deflate(aylis_core::Bounds::new(6.0, 0.0, 6.0, 0.0)), &text),
            }
        }
    }
}

// ===========================================================================
// ActivityIndicator
// ===========================================================================

/// Indicador de atividade girando (o `TAniIndicator`). Só anima com `active`. `kind` escolhe
/// o desenho (os spinners do Quasar: `dots`, `bars`, `audio`, `ios`...).
pub struct ActivityIndicator {
    pub base: ControlBase,
    pub active: bool,
    pub kind: crate::quasar::SpinnerKind,
}

impl Default for ActivityIndicator {
    fn default() -> Self {
        let mut base = ControlBase::new(32.0, 32.0);
        base.set_hit_test(false);
        Self { base, active: true, kind: crate::quasar::SpinnerKind::Ring }
    }
}

impl Component for ActivityIndicator {
    leaf_control!("ActivityIndicator");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        active: bool => "Behavior",
        kind: crate::quasar::SpinnerKind => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !self.active {
            return;
        }
        if self.kind != crate::quasar::SpinnerKind::Ring {
            let color = ctx.colors.accent.unwrap_or_else(|| ctx.theme.color(tone(ctx.variant(), "primary")));
            crate::quasar::paint_spinner(canvas, ctx.rect, self.kind, color, ctx.time as f32);
            ctx.request_frame();
            return;
        }
        let side = ctx.rect.width.min(ctx.rect.height);
        let width = (side / 9.0).max(2.0);
        let r = (side - width) / 2.0;
        let c = PointF::new(ctx.rect.width / 2.0, ctx.rect.height / 2.0);
        let color = ctx.theme.color(tone(ctx.variant(), "primary"));
        canvas.stroke_ellipse(RectF::new(c.x - r - width / 2.0, c.y - r - width / 2.0, 2.0 * r + width, 2.0 * r + width), &Stroke::new(color.with_opacity(0.18), width));
        // Arco que gira e "respira" (cresce e encolhe).
        let t = ctx.time as f32;
        let start = (t * 360.0) % 360.0;
        let sweep = 60.0 + 200.0 * (0.5 + 0.5 * (t * 2.6).sin());
        let mut arc = Path::new();
        arc.arc(c, r, r, start, sweep);
        canvas.stroke_path(&arc, &Stroke::new(color, width).round());
        ctx.request_frame();
    }
}

// ===========================================================================
// LevelMeter
// ===========================================================================

/// Medidor de nível (VU) para áudio: barra de `value` (0 a 1) com as cores do tema
/// (`meter_low` → `meter_mid` → `meter_high`) e marca de pico que cai devagar.
/// `segments` > 0 desenha em gomos (como os medidores de mesa de som).
///
/// `stereo` desenha duas barras (a de baixo com `value_right`); `gradient` pinta um
/// degradê pela posição (verde, âmbar perto de `high`, vermelho no fim) que aparece só até
/// o nível; `muted` enche a barra de `meter_high` (o dispositivo está mudo); `dim` apaga as
/// cores (o dispositivo desligado).
pub struct LevelMeter {
    pub base: ControlBase,
    pub value: f32,
    pub value_right: f32,
    pub stereo: bool,
    pub gradient: bool,
    pub muted: bool,
    pub dim: bool,
    pub vertical: bool,
    pub segments: usize,
    pub show_peak: bool,
    /// Daqui para cima a cor é `meter_mid`.
    pub mid: f32,
    /// Daqui para cima a cor é `meter_high`.
    pub high: f32,
    /// Tempo, em ms, que a barra leva até o valor novo (0 = salta): com níveis que
    /// chegam a cada 100 ms, uns 90 ms deixam o movimento contínuo.
    pub smoothing: f32,
    peak: Cell<(f32, f64)>,
    /// Por barra (esquerda, direita): de onde, para onde e quando começou a ir.
    glide: Cell<[(f32, f32, f64); 2]>,
}

impl Default for LevelMeter {
    fn default() -> Self {
        let mut base = ControlBase::new(160.0, 8.0);
        base.set_hit_test(false);
        Self {
            base,
            value: 0.0,
            value_right: 0.0,
            stereo: false,
            gradient: false,
            muted: false,
            dim: false,
            vertical: false,
            segments: 0,
            show_peak: true,
            mid: 0.7,
            high: 0.9,
            smoothing: 0.0,
            peak: Cell::new((0.0, 0.0)),
            glide: Cell::new([(0.0, 0.0, 0.0); 2]),
        }
    }
}

const PEAK_HOLD: f64 = 1.0;

impl LevelMeter {
    fn color_at(&self, ctx: &PaintCtx<'_>, level: f32) -> Color {
        let token = if level >= self.high { "meter_high" } else if level >= self.mid { "meter_mid" } else { "meter_low" };
        ctx.theme.color(token)
    }

    /// Nível de 0 a 1 a partir de decibéis (-60 dB = 0, 0 dB = 1).
    pub fn from_db(db: f32) -> f32 {
        ((db + 60.0) / 60.0).clamp(0.0, 1.0)
    }
}

impl Component for LevelMeter {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value" | "value_right" | "muted" | "dim")
    }

    leaf_control!("LevelMeter");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        value: f32 => "Data",
        value_right: f32 => "Data",
        stereo: bool => "Appearance",
        gradient: bool => "Appearance",
        muted: bool => "Data",
        dim: bool => "Appearance",
        vertical: bool => "Appearance",
        segments: usize => "Appearance",
        show_peak: bool => "Appearance",
        mid: f32 => "Appearance",
        high: f32 => "Appearance",
        smoothing: f32 => "Behavior",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        if self.stereo {
            // Duas barras com 1 px entre elas (a de cima é a esquerda).
            let (a, b) = if self.vertical {
                let w = ((r.width - 1.0) / 2.0).max(0.0);
                (RectF::new(r.x, r.y, w, r.height), RectF::new(r.x + w + 1.0, r.y, w, r.height))
            } else {
                let h = ((r.height - 1.0) / 2.0).max(0.0);
                (RectF::new(r.x, r.y, r.width, h), RectF::new(r.x, r.y + h + 1.0, r.width, h))
            };
            self.paint_track(canvas, ctx, a, self.value, true);
            self.paint_track(canvas, ctx, b, self.value_right, false);
        } else {
            self.paint_track(canvas, ctx, r, self.value, true);
        }
    }
}

impl LevelMeter {
    /// Uma barra em `r` com o nível `value` (a de `with_peak` leva a marca de pico).
    /// O nível mostrado na barra `i`: com `smoothing`, vai até `target` em linha reta.
    fn glide(&self, ctx: &PaintCtx<'_>, i: usize, target: f32) -> f32 {
        if self.smoothing <= 0.0 {
            return target;
        }
        let dur = self.smoothing as f64 / 1000.0;
        let mut all = self.glide.get();
        let (from, to, at) = all[i];
        let shown = |t: f64| from + (to - from) * (((t - at) / dur).clamp(0.0, 1.0) as f32);
        if (to - target).abs() > f32::EPSILON {
            all[i] = (shown(ctx.time), target, ctx.time);
            self.glide.set(all);
        }
        let (from, to, at) = all[i];
        let k = ((ctx.time - at) / dur).clamp(0.0, 1.0) as f32;
        if k < 1.0 {
            ctx.request_frame();
        }
        from + (to - from) * k
    }

    fn paint_track(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, r: RectF, value: f32, with_peak: bool) {
        let value = if self.muted { 1.0 } else { self.glide(ctx, if with_peak { 0 } else { 1 }, value.clamp(0.0, 1.0)) };
        let radius = ctx.style.radius.min(r.width.min(r.height) / 2.0);
        let alpha = if self.dim { 0.25 } else { 1.0 };
        canvas.fill_rect(r, radius, &Brush::Solid(ctx.style.fill));
        let length = if self.vertical { r.height } else { r.width };
        // Pico: segura um pouco e cai. Só na faixa que o mostra: sem `show_peak` (ou na segunda
        // faixa do estéreo), a queda pedia um quadro por atualização do monitor para nada.
        let (mut peak, mut at) = self.peak.get();
        if self.show_peak && with_peak {
            if value >= peak {
                peak = value;
                at = ctx.time;
            } else if ctx.time - at > PEAK_HOLD {
                peak = (peak - ((ctx.time - at - PEAK_HOLD) as f32) * 0.6).max(value);
                ctx.request_frame();
            } else {
                ctx.request_frame_at(at + PEAK_HOLD);
            }
            self.peak.set((peak, at));
        }
        let bar = |from: f32, to: f32| -> RectF {
            if self.vertical {
                RectF::new(r.x, r.bottom() - to * length, r.width, (to - from) * length)
            } else {
                RectF::new(r.x + from * length, r.y, (to - from) * length, r.height)
            }
        };
        if self.muted {
            canvas.fill_rect(r, radius, &Brush::Solid(ctx.theme.color("meter_high").with_opacity(alpha)));
            return;
        }
        if self.gradient {
            if value > 0.0 {
                canvas.save();
                canvas.clip_round_rect(bar(0.0, value), radius);
                let (low, mid, high) = (ctx.theme.color("meter_low"), ctx.theme.color("meter_mid"), ctx.theme.color("meter_high"));
                let stop = |offset: f32, color: Color| aylis_core::GradientStop { offset, color: color.with_opacity(alpha) };
                let (start, end) =
                    if self.vertical { (PointF::new(r.x, r.bottom()), PointF::new(r.x, r.y)) } else { (PointF::new(r.x, r.y), PointF::new(r.right(), r.y)) };
                let stops = vec![stop(0.0, low), stop(self.mid.min(self.high), low), stop(self.high, mid), stop(((self.high + 1.0) / 2.0).min(1.0), high), stop(1.0, high)];
                canvas.fill_rect(r, 0.0, &Brush::Linear { start, end, stops });
                canvas.restore();
            }
            return;
        }
        if self.segments > 0 {
            let n = self.segments;
            let gap = 2.0 / length;
            for i in 0..n {
                let (a, b) = (i as f32 / n as f32, (i + 1) as f32 / n as f32);
                let lit = b <= value + 0.5 / n as f32;
                let color = self.color_at(ctx, a);
                let fill = if lit { color.with_opacity(alpha) } else { color.with_opacity(0.12 * alpha) };
                canvas.fill_rect(bar(a, (b - gap).max(a)), 1.0, &Brush::Solid(fill));
            }
        } else if value > 0.0 {
            canvas.save();
            canvas.clip_round_rect(r, radius);
            // Faixas de cor até o valor.
            for (a, b) in [(0.0, self.mid), (self.mid, self.high), (self.high, 1.0)] {
                if value > a {
                    canvas.fill_rect(bar(a, value.min(b)), 0.0, &Brush::Solid(self.color_at(ctx, a).with_opacity(alpha)));
                }
            }
            canvas.restore();
        }
        if self.show_peak && with_peak && peak > 0.01 {
            let p = if self.vertical {
                RectF::new(r.x, r.bottom() - peak * length, r.width, 2.0)
            } else {
                RectF::new(r.x + peak * length - 2.0, r.y, 2.0, r.height)
            };
            canvas.fill_rect(p, 0.0, &Brush::Solid(self.color_at(ctx, peak)));
        }
    }
}

// ===========================================================================
// Breadcrumb
// ===========================================================================

/// Trilha de navegação ("Início › Vídeos › 2026"): cada parte é clicável e dispara
/// `on_change` com o índice. A última é a atual (em destaque).
pub struct Breadcrumb {
    pub base: ControlBase,
    pub items: String,
    hits: RefCell<Vec<RectF>>,
    hover: Cell<Option<usize>>,
}

impl Default for Breadcrumb {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 28.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, items: "Início;Pasta;Atual".into(), hits: RefCell::default(), hover: Cell::new(None) }
    }
}

impl Breadcrumb {
    fn item_at(&self, p: PointF) -> Option<usize> {
        self.hits.borrow().iter().position(|r| r.contains(p))
    }
}

impl Component for Breadcrumb {
    leaf_control!("Breadcrumb");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        items: String => "Data";
        events: on_change
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let h = self.item_at(event.position);
        if h != self.hover.get() {
            self.hover.set(h);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(None);
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(i) = self.item_at(event.position) {
            ctx.fire("on_change", EventArgs::Value((i as i64).into()));
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let items = split_items(&self.items);
        let mut hits = self.hits.borrow_mut();
        hits.clear();
        let mut x = ctx.rect.x;
        let h = ctx.rect.height;
        let muted = ctx.theme.color("text_muted");
        let side = (ctx.style.font.size * 1.0).round();
        for (i, item) in items.iter().enumerate() {
            let last = i + 1 == items.len();
            let mut text = ctx.text_style();
            text.color = if last { ctx.theme.color("text") } else if self.hover.get() == Some(i) { ctx.theme.color("primary") } else { muted };
            if last {
                text.font.weight = FontWeight::SemiBold;
            }
            let w = canvas.measure_text(item, &text).width + 2.0;
            let cell = RectF::new(x, 0.0, w.min((ctx.rect.right() - x).max(0.0)), h);
            canvas.draw_text(item, cell, &text);
            hits.push(cell);
            x += w;
            if !last {
                ctx.draw_icon(canvas, "chevron-right", RectF::new(x + 2.0, (h - side) / 2.0, side, side), muted);
                x += side + 4.0;
            }
            if x > ctx.rect.right() {
                break;
            }
        }
    }
}

// ===========================================================================
// SeekBar
// ===========================================================================

/// Barra de reprodução de mídia: posição, trecho já carregado (`buffered`), marcas
/// (`markers`, em segundos: "12.5;30") e, sob o mouse, o horário naquele ponto.
/// Arrastar dispara `on_change` (posição nova, a cada movimento) e soltar `on_seek`.
pub struct SeekBar {
    pub base: ControlBase,
    /// Em segundos.
    pub position: f64,
    pub duration: f64,
    pub buffered: f64,
    pub markers: String,
    pub show_time: bool,
    dragging: bool,
    /// A posição escolhida com o mouse enquanto o botão está apertado: ganha de quem
    /// atualiza `position` de fora nesse meio tempo (o timer de um player).
    held: Option<f64>,
    hover: Cell<Option<f32>>,
}

impl Default for SeekBar {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 28.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer).set_pan_on_touch(false);
        Self { base, position: 0.0, duration: 100.0, buffered: 0.0, markers: String::new(), show_time: true, dragging: false, held: None, hover: Cell::new(None) }
    }
}

const SEEK_PAD: f32 = 8.0;

/// "1:05" ou "1:02:03".
pub fn format_time(seconds: f64) -> String {
    let s = seconds.max(0.0).round() as u64;
    let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
    if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
}

impl SeekBar {
    /// Com o botão apertado, a posição de fora (o timer do player) não tira a barra do
    /// lugar que o usuário está escolhendo.
    fn after_set(&mut self, name: &str) {
        if name == "position" {
            if let Some(p) = self.held.filter(|_| self.dragging) {
                self.position = p;
            }
        }
    }

    fn fraction_at(&self, x: f32) -> f64 {
        let w = (self.base.bounds().width - 2.0 * SEEK_PAD).max(1.0);
        ((x - SEEK_PAD) / w).clamp(0.0, 1.0) as f64
    }

    fn seek_to(&mut self, ctx: &mut EventCtx<'_>, position: f64) {
        let p = position.clamp(0.0, self.duration.max(0.0));
        if self.dragging {
            self.held = Some(p);
        }
        if (p - self.position).abs() > 1e-6 {
            self.position = p;
            ctx.fire("on_change", EventArgs::Value(p.into()));
        }
        ctx.repaint();
    }
}

impl Component for SeekBar {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "position" | "duration")
    }

    leaf_control!("SeekBar");

    properties! {
        position: f64 => "Data",
        duration: f64 => "Data",
        buffered: f64 => "Data",
        markers: String => "Data",
        show_time: bool => "Appearance";
        events: on_change, on_seek;
        on_set: after_set
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.dragging = true;
            let p = self.fraction_at(event.position.x) * self.duration;
            self.seek_to(ctx, p);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.hover.set(Some(event.position.x));
        if self.dragging {
            let p = self.fraction_at(event.position.x) * self.duration;
            self.seek_to(ctx, p);
        } else {
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging {
            self.dragging = false;
            // O ponto escolhido, mesmo que um timer tenha mexido na posição no meio tempo.
            if let Some(p) = self.held.take() {
                self.position = p;
            }
            ctx.fire("on_seek", EventArgs::Value(self.position.into()));
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let step = if event.modifiers.shift { 10.0 } else { 5.0 };
        let to = match event.key {
            Key::Left => self.position - step,
            Key::Right => self.position + step,
            Key::Home => 0.0,
            Key::End => self.duration,
            _ => return,
        };
        self.seek_to(ctx, to);
        ctx.fire("on_seek", EventArgs::Value(self.position.into()));
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let active = self.dragging || self.hover.get().is_some() || ctx.state.focused;
        let thick = if active { 6.0 } else { 4.0 };
        let track = RectF::new(r.x + SEEK_PAD, r.y + (r.height - thick) / 2.0, (r.width - 2.0 * SEEK_PAD).max(0.0), thick);
        let frac = |v: f64| if self.duration > 0.0 { (v / self.duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let primary = ctx.theme.color("primary");
        canvas.fill_rect(track, thick / 2.0, &Brush::Solid(ctx.theme.color("track")));
        let buffered = track.width * frac(self.buffered);
        if buffered > 0.0 {
            canvas.fill_rect(RectF::new(track.x, track.y, buffered, thick), thick / 2.0, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(0.45)));
        }
        let played = track.width * frac(self.position);
        if played > 0.0 {
            canvas.fill_rect(RectF::new(track.x, track.y, played, thick), thick / 2.0, &Brush::Solid(primary));
        }
        for m in split_items(&self.markers) {
            if let Ok(t) = m.trim().replace(',', ".").parse::<f64>() {
                let x = track.x + track.width * frac(t);
                canvas.fill_rect(RectF::new(x - 1.5, track.y - 3.0, 3.0, thick + 6.0), 1.5, &Brush::Solid(ctx.theme.color("warning")));
            }
        }
        if active {
            let cx = track.x + played;
            let d = 14.0;
            canvas.fill_ellipse(RectF::new(cx - d / 2.0, r.y + (r.height - d) / 2.0, d, d), &Brush::Solid(primary));
            canvas.stroke_ellipse(RectF::new(cx - d / 2.0, r.y + (r.height - d) / 2.0, d, d), &Stroke::new(ctx.theme.color("surface"), 2.0));
        }
        if let (true, Some(hx)) = (self.show_time, self.hover.get()) {
            let t = self.fraction_at(hx) * self.duration;
            let label = format_time(t);
            let mut text = ctx.text_style();
            text.font.size = 11.0;
            text.h_align = TextAlign::Center;
            text.color = ctx.theme.color("surface");
            let w = canvas.measure_text(&label, &text).width + 12.0;
            let x = (hx - w / 2.0).clamp(r.x, r.right() - w);
            let bubble = RectF::new(x, r.y - 20.0, w, 18.0);
            let fill = Brush::Solid(ctx.theme.color("text"));
            // O balão sai por cima da barra: por cima de tudo, senão o painel de cima o cobre.
            ctx.overlay(move |c| {
                c.fill_rect(bubble, 4.0, &fill);
                c.draw_text(&label, bubble, &text);
            });
        }
    }
}

// ===========================================================================
// HotkeyEdit
// ===========================================================================

/// Campo de tecla de atalho (o `THotKey`): com o foco, a próxima combinação vira o texto
/// ("Ctrl+Shift+R"). Backspace ou Delete sozinhos limpam. `on_change` a cada troca.
pub struct HotkeyEdit {
    pub base: ControlBase,
    pub hotkey: String,
    pub placeholder: String,
}

impl Default for HotkeyEdit {
    fn default() -> Self {
        let mut base = ControlBase::new(180.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, hotkey: String::new(), placeholder: "Pressione as teclas".into() }
    }
}

/// Nome de uma combinação de teclas ("Ctrl+Alt+F5"); `None` para teclas que só modificam.
pub fn hotkey_text(event: &KeyEvent) -> Option<String> {
    let key = match event.key {
        Key::Char(c) => c.to_uppercase().collect::<String>(),
        Key::F(n) => format!("F{n}"),
        Key::Enter => "Enter".into(),
        Key::Space => "Space".into(),
        Key::Escape => "Esc".into(),
        Key::Insert => "Insert".into(),
        Key::Home => "Home".into(),
        Key::End => "End".into(),
        Key::PageUp => "PageUp".into(),
        Key::PageDown => "PageDown".into(),
        Key::Left => "Left".into(),
        Key::Right => "Right".into(),
        Key::Up => "Up".into(),
        Key::Down => "Down".into(),
        Key::Tab => "Tab".into(),
        Key::Backspace => "Backspace".into(),
        Key::Delete => "Delete".into(),
        Key::Shift | Key::Control | Key::Alt | Key::Meta | Key::Unknown => return None,
    };
    let m = event.modifiers;
    let mut parts = Vec::new();
    if m.ctrl {
        parts.push("Ctrl".to_string());
    }
    if m.alt {
        parts.push("Alt".to_string());
    }
    if m.shift {
        parts.push("Shift".to_string());
    }
    if m.meta {
        parts.push("Meta".to_string());
    }
    parts.push(key);
    Some(parts.join("+"))
}

impl Component for HotkeyEdit {
    leaf_control!("HotkeyEdit");

    properties! {
        hotkey: String => "Data",
        placeholder: String => "Appearance";
        events: on_change
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let m = event.modifiers;
        let plain = !(m.ctrl || m.alt || m.shift || m.meta);
        // Tab sem modificador continua passando o foco.
        if plain && event.key == Key::Tab {
            return;
        }
        let next = if plain && matches!(event.key, Key::Backspace | Key::Delete) {
            Some(String::new())
        } else {
            hotkey_text(event)
        };
        if let Some(text) = next {
            if text != self.hotkey {
                self.hotkey = text;
                ctx.fire("on_change", EventArgs::Value(self.hotkey.clone().into()));
            }
            ctx.repaint();
        }
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        canvas.fill_rect(ctx.rect, s.radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 && !s.border_color.is_transparent() {
            canvas.stroke_rect(ctx.rect, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let inner = ctx.rect.deflate(s.padding);
        let mut text = ctx.text_style();
        if self.hotkey.is_empty() {
            text.color = ctx.theme.color("text_muted");
            canvas.draw_text(&self.placeholder, inner, &text);
            return;
        }
        // Cada tecla numa "tampinha".
        let mut x = inner.x;
        text.font.size = (text.font.size - 1.0).max(9.0);
        text.h_align = TextAlign::Center;
        let cap_h = (inner.height - 2.0).min(text.font.size * 1.7);
        let y = inner.y + (inner.height - cap_h) / 2.0;
        for (i, part) in self.hotkey.split('+').enumerate() {
            if i > 0 {
                let mut plus = text.clone();
                plus.color = ctx.theme.color("text_muted");
                canvas.draw_text("+", RectF::new(x, inner.y, 12.0, inner.height), &plus);
                x += 12.0;
            }
            let w = canvas.measure_text(part, &text).width + 12.0;
            let cap = RectF::new(x, y, w, cap_h);
            canvas.fill_rect(cap, 4.0, &Brush::Solid(ctx.theme.color("surface_2")));
            canvas.stroke_rect(cap, 4.0, &Stroke::new(ctx.theme.color("border"), 1.0));
            canvas.draw_text(part, cap, &text);
            x += w;
        }
    }
}

// ===========================================================================
// DirectionPad
// ===========================================================================

/// Direcional redondo (controle PTZ de câmera, joystick): oito direções e o centro.
/// Apertar dispara `on_press` com a direção ("up", "down_left", "center"...) e soltar,
/// `on_release`. `diagonals = false` deixa só as quatro principais.
pub struct DirectionPad {
    pub base: ControlBase,
    pub diagonals: bool,
    /// Ícone do botão do meio (vazio = sem botão).
    pub center_icon: String,
    pressed: Option<&'static str>,
}

impl Default for DirectionPad {
    fn default() -> Self {
        let mut base = ControlBase::new(140.0, 140.0);
        base.set_cursor(CursorShape::Pointer).set_pan_on_touch(false);
        Self { base, diagonals: true, center_icon: "home".into(), pressed: None }
    }
}

impl DirectionPad {
    /// A direção de um ponto (local).
    pub fn direction_at(&self, p: PointF) -> Option<&'static str> {
        let b = self.base.bounds();
        let r = b.width.min(b.height) / 2.0;
        let (dx, dy) = (p.x - b.width / 2.0, p.y - b.height / 2.0);
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > r {
            return None;
        }
        if dist < r * 0.32 {
            return (!self.center_icon.is_empty()).then_some("center");
        }
        let angle = dy.atan2(dx).to_degrees().rem_euclid(360.0);
        let (sectors, names): (f32, &[&'static str]) = if self.diagonals {
            (8.0, &["right", "down_right", "down", "down_left", "left", "up_left", "up", "up_right"])
        } else {
            (4.0, &["right", "down", "left", "up"])
        };
        let size = 360.0 / sectors;
        let i = (((angle + size / 2.0) / size).floor() as usize) % names.len();
        Some(names[i])
    }
}

impl Component for DirectionPad {
    leaf_control!("DirectionPad");

    properties! {
        diagonals: bool => "Behavior",
        center_icon: String => "Appearance";
        events: on_press, on_release
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if let Some(d) = self.direction_at(event.position) {
            self.pressed = Some(d);
            ctx.fire("on_press", EventArgs::Value(d.into()));
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if let Some(d) = self.pressed.take() {
            ctx.fire("on_release", EventArgs::Value(d.into()));
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let b = ctx.rect;
        let side = b.width.min(b.height);
        let r = RectF::new((b.width - side) / 2.0, (b.height - side) / 2.0, side, side);
        let c = r.center();
        canvas.fill_ellipse(r, &Brush::Solid(ctx.theme.color("surface_2")));
        canvas.stroke_ellipse(r, &Stroke::new(ctx.theme.color("border"), 1.0));
        let primary = ctx.theme.color("primary");
        let muted = ctx.theme.color("text");
        // Setor apertado em destaque.
        if let Some(d) = self.pressed.filter(|d| *d != "center") {
            let names = ["right", "down_right", "down", "down_left", "left", "up_left", "up", "up_right"];
            let i = names.iter().position(|n| *n == d).unwrap_or(0) as f32;
            let size = if self.diagonals { 45.0 } else { 90.0 };
            let start = i * 45.0 - size / 2.0;
            let mut wedge = Path::new();
            wedge.move_to(c.x, c.y);
            wedge.arc(c, side / 2.0 - 1.0, side / 2.0 - 1.0, start, size);
            wedge.close();
            canvas.fill_path(&wedge, &Brush::Solid(ctx.theme.color("primary_soft")));
        }
        let icon = (side * 0.16).max(12.0);
        let reach = side * 0.36;
        for (name, angle, glyph) in [
            ("right", 0.0f32, "chevron-right"),
            ("down", 90.0, "chevron-down"),
            ("left", 180.0, "chevron-left"),
            ("up", 270.0, "chevron-up"),
        ] {
            let a = angle.to_radians();
            let p = PointF::new(c.x + reach * a.cos(), c.y + reach * a.sin());
            let color = if self.pressed == Some(name) { primary } else { muted };
            ctx.draw_icon(canvas, glyph, RectF::new(p.x - icon / 2.0, p.y - icon / 2.0, icon, icon), color);
        }
        if self.diagonals {
            for (name, angle) in [("down_right", 45.0f32), ("down_left", 135.0), ("up_left", 225.0), ("up_right", 315.0)] {
                let a = angle.to_radians();
                let p = PointF::new(c.x + reach * 0.92 * a.cos(), c.y + reach * 0.92 * a.sin());
                let color = if self.pressed == Some(name) { primary } else { ctx.theme.color("text_muted") };
                canvas.fill_ellipse(RectF::new(p.x - 2.5, p.y - 2.5, 5.0, 5.0), &Brush::Solid(color));
            }
        }
        if !self.center_icon.is_empty() {
            let cr = side * 0.3;
            let center = RectF::new(c.x - cr, c.y - cr, 2.0 * cr, 2.0 * cr);
            let fill = if self.pressed == Some("center") { ctx.theme.color("primary_soft") } else { ctx.theme.color("surface") };
            canvas.fill_ellipse(center, &Brush::Solid(fill));
            canvas.stroke_ellipse(center, &Stroke::new(ctx.theme.color("border"), 1.0));
            ctx.draw_icon(canvas, &self.center_icon, center.deflate(aylis_core::Bounds::all(cr * 0.55)), muted);
        }
    }
}

// ===========================================================================
// Joystick
// ===========================================================================

/// Alavanca analógica na tela (jogos no celular, controle de câmera): arrastar o botão dá `x`
/// e `y` de -1 a 1 (y positivo para baixo) com a `dead_zone` aplicada, em `on_change`; soltar
/// dispara `on_release` e, com `spring`, o botão volta ao centro. Para o controle de jogo
/// de verdade, o componente `Gamepad`.
pub struct Joystick {
    pub base: ControlBase,
    pub dead_zone: f32,
    pub spring: bool,
    /// A posição do botão (sem a zona morta), de -1 a 1.
    knob: (f32, f32),
    dragging: bool,
}

impl Default for Joystick {
    fn default() -> Self {
        let mut base = ControlBase::new(140.0, 140.0);
        base.set_cursor(CursorShape::Pointer).set_pan_on_touch(false);
        Self { base, dead_zone: 0.1, spring: true, knob: (0.0, 0.0), dragging: false }
    }
}

impl Joystick {
    /// O valor com a zona morta: abaixo dela é 0 e o resto vai de 0 a 1.
    fn value(&self) -> (f32, f32) {
        let (x, y) = self.knob;
        let len = (x * x + y * y).sqrt();
        let zone = self.dead_zone.clamp(0.0, 0.95);
        if len <= zone || len == 0.0 {
            return (0.0, 0.0);
        }
        let scale = ((len - zone) / (1.0 - zone)).min(1.0) / len;
        (x * scale, y * scale)
    }
    pub fn x(&self) -> f64 {
        f64::from(self.value().0)
    }
    pub fn y(&self) -> f64 {
        f64::from(self.value().1)
    }
    fn set_f(&mut self, _: f64) {}

    /// O raio da base e o do botão (o botão anda até encostar na borda).
    fn radii(&self) -> (f32, f32) {
        let b = self.base.bounds();
        let r = b.width.min(b.height) / 2.0;
        (r, r * 0.38)
    }

    fn move_knob(&mut self, ctx: &mut EventCtx<'_>, p: PointF) {
        let b = self.base.bounds();
        let (r, knob) = self.radii();
        let reach = (r - knob).max(1.0);
        let (mut x, mut y) = ((p.x - b.width / 2.0) / reach, (p.y - b.height / 2.0) / reach);
        let len = (x * x + y * y).sqrt();
        if len > 1.0 {
            (x, y) = (x / len, y / len);
        }
        self.set_knob(ctx, (x, y));
    }

    fn set_knob(&mut self, ctx: &mut EventCtx<'_>, knob: (f32, f32)) {
        let before = self.value();
        self.knob = knob;
        if self.value() != before {
            ctx.fire("on_change", EventArgs::None);
        }
        ctx.repaint();
    }
}

impl Component for Joystick {
    leaf_control!("Joystick");

    properties! {
        dead_zone: f32 => "Behavior",
        spring: bool => "Behavior";
        events: on_change, on_release;
        computed: x: f64 => x / set_f, y: f64 => y / set_f
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.dragging = true;
            self.move_knob(ctx, event.position);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging {
            self.move_knob(ctx, event.position);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging {
            self.dragging = false;
            if self.spring {
                self.set_knob(ctx, (0.0, 0.0));
            }
            ctx.fire("on_release", EventArgs::None);
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let b = ctx.rect;
        let (r, knob) = self.radii();
        let c = PointF::new(b.x + b.width / 2.0, b.y + b.height / 2.0);
        let base = RectF::new(c.x - r, c.y - r, 2.0 * r, 2.0 * r);
        canvas.fill_ellipse(base, &Brush::Solid(ctx.theme.color("surface_2")));
        canvas.stroke_ellipse(base, &Stroke::new(ctx.theme.color("border"), 1.0));
        // A zona morta, de leve.
        let reach = r - knob;
        let zone = reach * self.dead_zone.clamp(0.0, 0.95) + knob;
        canvas.stroke_ellipse(RectF::new(c.x - zone, c.y - zone, 2.0 * zone, 2.0 * zone), &Stroke::new(ctx.theme.color("border"), 1.0));
        let k = PointF::new(c.x + self.knob.0 * reach, c.y + self.knob.1 * reach);
        let rect = RectF::new(k.x - knob, k.y - knob, 2.0 * knob, 2.0 * knob);
        let fill = if self.dragging { ctx.theme.color("primary") } else { ctx.theme.color("surface") };
        canvas.fill_ellipse(rect, &Brush::Solid(fill));
        canvas.stroke_ellipse(rect, &Stroke::new(ctx.theme.color(if self.dragging { "primary" } else { "border" }), 1.5));
    }
}

// ===========================================================================
// FpsMeter
// ===========================================================================

/// Mostra os quadros por segundo (o [`Ui::fps`](aylis_core::Ui::fps)), onde o app quiser.
/// O aylis só desenha quando algo muda: parado, o número cai para 0. Com `live`, o medidor
/// se atualiza duas vezes por segundo (o que já são 2 quadros por segundo).
pub struct FpsMeter {
    pub base: ControlBase,
    /// `{0}` vira o número.
    pub format: String,
    pub live: bool,
}

impl Default for FpsMeter {
    fn default() -> Self {
        let mut base = ControlBase::new(80.0, 24.0);
        base.set_hit_test(false);
        Self { base, format: "{0} fps".into(), live: false }
    }
}

impl Component for FpsMeter {
    leaf_control!("FpsMeter");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        format: String => "Appearance",
        live: bool => "Behavior"
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let text = self.format.replace("{0}", &format!("{:.0}", ctx.ui.fps()));
        let mut t = ctx.text_style();
        t.font = ctx.monospace_font(t.font.size);
        canvas.draw_text(&text, ctx.rect, &t);
        if self.live {
            ctx.request_frame_at(ctx.time + 0.5);
        }
    }
}
