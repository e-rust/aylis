//! [`MediaCard`]: o cartão de um vídeo, foto ou pasta numa biblioteca de mídia.

use std::any::Any;
use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Image, ImageFit, Key,
    KeyEvent, MethodInfo, PaintCtx, PointF, PointerButton, PointerEvent, PropValue, RectF, Stroke, TextAlign, Trimming, properties,
};

use crate::{Edit, leaf_control};

/// O cartão de um item de uma biblioteca de mídia (vídeos, fotos, gravações).
///
/// Normal: a miniatura em cima ([`set_image`](MediaCard::set_image); sem ela, o `icon` no
/// meio), a `duration` no canto de baixo, o `title` e embaixo o `subtitle` com os `badges`
/// à direita (separados por `;`; `*` na frente destaca, `icon:nome` é um ícone). `compact`
/// é a linha de uma pasta: o `icon` à esquerda, o título e o subtítulo ao lado.
///
/// Seleção: a caixa no canto (aparece com o mouse em cima, com `selection_mode` ou
/// selecionado) dispara `on_select`; `selected` marca e `order` (> 0) mostra o número da
/// ordem na caixa. `dimmed` apaga (recortado, esperando colar); `progress` ≥ 0 desenha a
/// barra sobre a miniatura com o `progress_text`. O clique no resto dispara `on_click`; o
/// botão direito, `on_context` (com a posição).
///
/// Renomear: com `editable_title`, o clique no nome abre um campo sobre ele (o método
/// `rename` também abre, para o F2). Enter ou sair do campo grava no `title` e dispara
/// `on_rename` com o nome novo; Esc cancela.
pub struct MediaCard {
    pub base: ControlBase,
    /// O nome do item, embaixo da miniatura.
    pub title: String,
    /// A linha menor embaixo do título (tamanho, data).
    pub subtitle: String,
    /// A duração no canto de baixo da miniatura (`"02:15"`); vazio: nenhuma.
    pub duration: String,
    /// Os selos à direita do subtítulo, separados por `;` (`*` na frente destaca,
    /// `icon:nome` é um ícone).
    pub badges: String,
    /// O ícone no meio quando não há miniatura (e à esquerda no `compact`).
    pub icon: String,
    /// A linha de uma pasta: o ícone à esquerda, o título e o subtítulo ao lado.
    pub compact: bool,
    /// Marcado (a caixa da seleção aparece preenchida).
    pub selected: bool,
    /// O número da ordem mostrado na caixa da seleção; 0: nenhum.
    pub order: u32,
    /// A caixa da seleção sempre à vista (não só com o mouse em cima).
    pub selection_mode: bool,
    /// Aparece apagado (recortado, esperando colar).
    pub dimmed: bool,
    /// A barra de progresso sobre a miniatura, de 0 a 1; negativo: sem barra.
    pub progress: f32,
    /// O texto junto da barra de progresso.
    pub progress_text: String,
    /// Um alvo de soltar marcado (a pasta sob o item arrastado).
    pub highlighted: bool,
    /// O clique no nome abre o campo de renomear (em vez do `on_click`); o nome novo vem
    /// no `on_rename`.
    pub editable_title: bool,
    image: Option<Image>,
    /// O campo de renomear aberto.
    editor: Option<ComponentId>,
    /// O clique que fechou o campo não abre o item.
    swallow_click: bool,
    hover: Cell<bool>,
    /// Onde o último botão foi apertado (o `click` não traz a posição).
    pressed_at: Cell<PointF>,
}

impl Default for MediaCard {
    fn default() -> Self {
        let mut base = ControlBase::new(170.0, 148.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            title: String::new(),
            subtitle: String::new(),
            duration: String::new(),
            badges: String::new(),
            icon: String::new(),
            compact: false,
            selected: false,
            order: 0,
            selection_mode: false,
            dimmed: false,
            progress: -1.0,
            progress_text: String::new(),
            highlighted: false,
            editable_title: false,
            image: None,
            editor: None,
            swallow_click: false,
            hover: Cell::new(false),
            pressed_at: Cell::new(PointF::ZERO),
        }
    }
}

/// O lado da caixa de seleção e a folga dela até a borda.
const CHECK: f32 = 20.0;
const CHECK_PAD: f32 = 8.0;

impl MediaCard {
    pub fn set_image(&mut self, image: Option<Image>) {
        self.image = image;
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    fn check_rect(&self) -> RectF {
        RectF::new(CHECK_PAD, CHECK_PAD, CHECK, CHECK)
    }

    fn shows_check(&self) -> bool {
        !self.compact && (self.selected || self.selection_mode || self.hover.get())
    }

    /// Onde o nome é desenhado, nas coordenadas do card (a pintura usa o mesmo).
    fn title_rect(&self, width: f32, height: f32) -> RectF {
        if self.compact {
            let x = 49.0;
            RectF::new(x, (height - 16.8 - 15.4) / 2.0, (width - 13.0 - x).max(0.0), 16.8)
        } else {
            let thumb_h = ((width - 2.0) * 9.0 / 16.0).min(height - 50.0);
            RectF::new(10.0, 1.0 + thumb_h + 1.0 + 8.0, width - 20.0, 16.8)
        }
    }

    pub fn is_renaming(&self) -> bool {
        self.editor.is_some()
    }

    /// Abre o campo de renomear sobre o nome, com o nome atual selecionado.
    pub fn start_rename(&mut self, ctx: &mut EventCtx<'_>) {
        if self.editor.is_some() {
            return;
        }
        let b = self.base.bounds();
        let t = self.title_rect(b.width, b.height);
        // O campo é mais alto que o texto: centrado na linha e um pouco mais largo.
        let local = RectF::new(b.x + t.x - 4.0, b.y + t.y + t.height / 2.0 - 13.0, t.width + 8.0, 26.0);
        let rect = ctx.ui.to_form(ctx.id, local);
        let mut edit = Edit::new(self.title.clone());
        edit.select_all();
        let editor = TitleEditor { edit, owner: ctx.id, done: false };
        let root = ctx.ui.root_of(ctx.id);
        let id = ctx.ui.add(editor, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(rect.x, rect.y, rect.width, rect.height);
        }
        ctx.ui.follow(id, ctx.id);
        ctx.request_focus(id);
        self.editor = Some(id);
        ctx.repaint();
    }

    fn finish_rename(&mut self, ctx: &mut EventCtx<'_>, name: Option<String>) {
        if let Some(id) = self.editor.take() {
            ctx.ui.free(id);
        }
        if let Some(name) = name.map(|n| n.trim().to_string()).filter(|n| !n.is_empty() && *n != self.title) {
            self.title = name.clone();
            ctx.fire("on_rename", EventArgs::Text(name));
        }
        ctx.repaint();
    }
}

/// O que o campo de renomear manda ao card: o nome digitado, ou `None` (Esc).
struct RenameDone(Option<String>);

/// O campo de renomear de um [`MediaCard`]: um [`Edit`] numa camada sobre o form, que
/// avisa o card ao confirmar ou cancelar e some junto com ele.
struct TitleEditor {
    edit: Edit,
    owner: ComponentId,
    done: bool,
}

impl TitleEditor {
    fn finish(&mut self, ctx: &mut EventCtx<'_>, name: Option<String>) {
        if !self.done {
            self.done = true;
            ctx.ui.send_message(self.owner, &RenameDone(name));
        }
    }
}

impl Component for TitleEditor {
    fn class_name(&self) -> &'static str {
        "MediaCardEditor"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.edit)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.edit)
    }

    fn dies_with(&self) -> Option<ComponentId> {
        Some(self.owner)
    }

    // O clique fora (mesmo onde não há foco) fecha e grava, pelo `focus_exit`.
    fn closes_on_outside_press(&self) -> bool {
        true
    }

    fn captures_key(&self, event: &KeyEvent) -> bool {
        matches!(event.key, Key::Enter | Key::Escape) || self.edit.captures_key(event)
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::Enter => {
                let name = self.edit.text.clone();
                self.finish(ctx, Some(name));
                ctx.set_handled();
            }
            Key::Escape => {
                self.finish(ctx, None);
                ctx.set_handled();
            }
            _ => self.edit.key_down(ctx, event),
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.edit.focus_exit(ctx);
        let name = self.edit.text.clone();
        self.finish(ctx, Some(name));
    }
}

impl Component for MediaCard {
    leaf_control!("MediaCard");

    properties! {
        title: String => "Appearance",
        subtitle: String => "Appearance",
        duration: String => "Appearance",
        badges: String => "Appearance",
        icon: String => "Appearance",
        compact: bool => "Appearance",
        selected: bool => "Data",
        order: u32 => "Data",
        selection_mode: bool => "Behavior",
        dimmed: bool => "Appearance",
        progress: f32 => "Data",
        progress_text: String => "Data",
        highlighted: bool => "Appearance",
        editable_title: bool => "Behavior";
        events: on_click, on_select, on_context, on_rename
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("rename", &[], None)];
        METHODS
    }

    /// `rename()`: abre o campo de renomear (o F2 do app), mesmo sem `editable_title`.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "rename" => {
                self.start_rename(ctx);
                Ok(PropValue::Null)
            }
            _ => Err(format!("MediaCard não tem o método '{method}'")),
        }
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(RenameDone(name)) = message.downcast_ref::<RenameDone>() {
            self.finish_rename(ctx, name.clone());
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.editor == Some(freed) {
            self.editor = None;
        }
    }

    fn pointer_enter(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(true);
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(false);
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pressed_at.set(event.position);
        // O clique em outra parte do card grava o nome que estava sendo editado.
        if let Some(id) = self.editor {
            let name = match ctx.ui.get_prop(id, "text") {
                Ok(PropValue::String(name)) => Some(name),
                _ => None,
            };
            self.finish_rename(ctx, name);
            self.swallow_click = true;
            ctx.set_handled();
            return;
        }
        self.swallow_click = false;
        if event.button == PointerButton::Secondary {
            ctx.fire("on_context", EventArgs::Pointer(*event));
        }
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if std::mem::take(&mut self.swallow_click) {
            return;
        }
        let at = self.pressed_at.get();
        let b = self.base.bounds();
        if self.shows_check() && self.check_rect().inflate(4.0).contains(at) {
            ctx.fire("on_select", EventArgs::None);
        } else if self.editable_title && self.title_rect(b.width, b.height).inflate(2.0).contains(at) {
            self.start_rename(ctx);
        } else {
            ctx.fire("on_click", EventArgs::None);
        }
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let t = ctx.theme;
        let border2 = t.colors.get("border_2").copied().unwrap_or(t.color("border"));
        let hover = ctx.state.hover;
        self.hover.set(hover);
        canvas.save();
        if self.dimmed {
            canvas.save_layer(0.45);
        }
        let fill = if hover { t.colors.get("surface_3").copied().unwrap_or(t.color("surface_2")) } else { t.color("surface_2") };
        canvas.fill_rect(r, 8.0, &Brush::Solid(fill));
        let mut title = ctx.text_style();
        title.font.size = 12.0;
        title.font.weight = FontWeight::SemiBold;
        title.trimming = Trimming::Character;
        title.v_align = TextAlign::Center;
        let mut sub = ctx.text_style();
        sub.font.size = 11.0;
        sub.color = t.colors.get("text_3").copied().unwrap_or(t.color("text_muted"));
        sub.trimming = Trimming::Character;
        sub.v_align = TextAlign::Center;
        if self.compact {
            // Ícone de 26 a 13 da borda; nome e detalhe a partir de 49 (como a pasta da web).
            let icon = RectF::new(r.x + 13.0, r.y + (r.height - 26.0) / 2.0, 26.0, 26.0);
            if !self.icon.is_empty() {
                ctx.draw_icon(canvas, &self.icon, icon, if hover { t.color("text_muted") } else { sub.color });
            }
            let x = r.x + 49.0;
            let w = (r.right() - 13.0 - x).max(0.0);
            let top = r.y + (r.height - 16.8 - 15.4) / 2.0;
            canvas.draw_text(&self.title, RectF::new(x, top, w, 16.8), &title);
            canvas.draw_text(&self.subtitle, RectF::new(x, top + 16.8, w, 15.4), &sub);
        } else {
            let thumb = RectF::new(r.x + 1.0, r.y + 1.0, r.width - 2.0, ((r.width - 2.0) * 9.0 / 16.0).min(r.height - 50.0));
            canvas.save();
            canvas.clip_round_rect(thumb, 8.0);
            canvas.fill_rect(thumb, 0.0, &Brush::Solid(Color::rgb(0x0B, 0x0B, 0x0D)));
            match &self.image {
                Some(image) => {
                    if let Some(size) = image.known_size().or_else(|| canvas.image_size(image)) {
                        let (src, dest) = ImageFit::Cover.place(size, thumb);
                        canvas.draw_image(image, src, dest, 1.0);
                    }
                }
                None if !self.icon.is_empty() => {
                    let side = 28.0f32.min(thumb.height * 0.4);
                    let icon = RectF::new(thumb.x + (thumb.width - side) / 2.0, thumb.y + (thumb.height - side) / 2.0, side, side);
                    ctx.draw_icon(canvas, &self.icon, icon, Color::WHITE.with_opacity(0.3));
                }
                None => {}
            }
            canvas.restore();
            if !self.duration.is_empty() {
                let mut d = ctx.text_style();
                d.font.size = 10.0;
                d.font.weight = FontWeight::SemiBold;
                d.color = Color::WHITE.with_opacity(0.95);
                d.h_align = TextAlign::Center;
                d.v_align = TextAlign::Center;
                let w = canvas.measure_text(&self.duration, &d).width + 12.0;
                let chip = RectF::new(thumb.right() - w - 6.0, thumb.bottom() - 24.0, w, 18.0);
                canvas.fill_rect(chip, 4.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.85)));
                canvas.stroke_rect(chip.inflate(-0.5), 4.0, &Stroke::new(Color::WHITE.with_opacity(0.06), 1.0));
                canvas.draw_text(&self.duration, chip, &d);
            }
            if self.progress >= 0.0 {
                let bar = RectF::new(thumb.x, thumb.bottom() - 4.0, thumb.width, 4.0);
                canvas.fill_rect(bar, 0.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.6)));
                canvas.fill_rect(RectF::new(bar.x, bar.y, bar.width * self.progress.clamp(0.0, 1.0), bar.height), 0.0, &Brush::Solid(t.color("primary")));
                if !self.progress_text.is_empty() {
                    let mut p = ctx.text_style();
                    p.font.size = 11.5;
                    p.font.weight = FontWeight::SemiBold;
                    p.color = Color::WHITE;
                    p.h_align = TextAlign::Center;
                    p.v_align = TextAlign::Center;
                    let label = RectF::new(thumb.x, thumb.y + thumb.height / 2.0 - 12.0, thumb.width, 24.0);
                    canvas.fill_rect(label.inflate(-thumb.width * 0.2), 6.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.6)));
                    canvas.draw_text(&self.progress_text, label, &p);
                }
            }
            // A linha sob a miniatura; o corpo: nome (12 px) e o rodapé de 15,4 px com o
            // tamanho à esquerda e os selos (15 px, fonte 9) à direita.
            canvas.fill_rect(RectF::new(thumb.x, thumb.bottom(), thumb.width, 1.0), 0.0, &Brush::Solid(t.color("border")));
            let x = r.x + 10.0;
            let w = r.width - 20.0;
            let y = thumb.bottom() + 1.0 + 8.0;
            canvas.draw_text(&self.title, RectF::new(x, y, w, 16.8), &title);
            let row = RectF::new(x, y + 16.8 + 1.0, w, 15.4);
            let mut right = row.right();
            let mut badge_style = ctx.text_style();
            badge_style.font.size = 9.0;
            badge_style.h_align = TextAlign::Center;
            badge_style.v_align = TextAlign::Center;
            let by = row.y + (row.height - 15.0) / 2.0;
            let badge_fill = t.color("surface_2");
            let badges: Vec<&str> = self.badges.split(';').filter(|b| !b.is_empty()).collect();
            for b in badges.iter().rev() {
                let (accent, text) = match b.strip_prefix('*') {
                    Some(rest) => (true, rest),
                    None => (false, *b),
                };
                let (w, icon) = match text.strip_prefix("icon:") {
                    Some(name) => (20.0, Some(name)),
                    None => (canvas.measure_text(text, &badge_style).width + 10.0, None),
                };
                let rect = RectF::new(right - w, by, w, 15.0);
                let edge = if accent { t.color("warning") } else { border2 };
                let color = if accent { t.color("warning") } else { sub.color };
                canvas.fill_rect(rect, 3.0, &Brush::Solid(badge_fill));
                canvas.stroke_rect(rect.inflate(-0.5), 3.0, &Stroke::new(edge, 1.0));
                match icon {
                    Some(name) => {
                        let side = 10.0;
                        let at = RectF::new(rect.x + (rect.width - side) / 2.0, rect.y + (rect.height - side) / 2.0, side, side);
                        ctx.draw_icon(canvas, name, at, color);
                    }
                    None => {
                        badge_style.color = color;
                        canvas.draw_text(text, rect, &badge_style);
                    }
                }
                right -= w + 3.0;
            }
            // O tamanho não encolhe por causa dos selos (com eles cortava "25 MB").
            let natural = canvas.measure_text(&self.subtitle, &sub).width + 1.0;
            let room = (right + 3.0 - 6.0 - x).max(natural.min(w));
            canvas.draw_text(&self.subtitle, RectF::new(x, row.y, room, row.height), &sub);
        }
        // Borda: selecionado (2 px), alvo de soltar, mouse em cima.
        let (edge, width) = if self.selected || self.highlighted {
            (t.color("primary"), 2.0)
        } else if hover {
            (border2, 1.0)
        } else {
            (t.color("border"), 1.0)
        };
        canvas.stroke_rect(r.inflate(-width / 2.0), 9.0, &Stroke::new(edge, width));
        if self.shows_check() {
            let check = RectF::new(r.x + CHECK_PAD, r.y + CHECK_PAD, CHECK, CHECK);
            if self.selected {
                canvas.fill_rect(check, 5.0, &Brush::Solid(t.color("primary")));
                if self.order > 0 {
                    let mut n = ctx.text_style();
                    n.font.size = 11.0;
                    n.font.weight = FontWeight::Bold;
                    n.color = t.color("on_primary");
                    n.h_align = TextAlign::Center;
                    n.v_align = TextAlign::Center;
                    canvas.draw_text(&self.order.to_string(), check, &n);
                } else {
                    ctx.draw_icon(canvas, "check", check.inflate(-3.0), t.color("on_primary"));
                }
            } else {
                canvas.fill_rect(check, 5.0, &Brush::Solid(Color::rgb(0, 0, 0).with_opacity(0.55)));
                canvas.stroke_rect(check.inflate(-0.75), 5.0, &Stroke::new(Color::WHITE.with_opacity(0.8), 1.5));
            }
        }
        if self.dimmed {
            canvas.restore();
        }
        canvas.restore();
    }
}
