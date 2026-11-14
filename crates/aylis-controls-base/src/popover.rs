//! Popover: um cartão flutuante ancorado num componente (a ficha de informação, a caixa de
//! opções de um botão), que fecha com o clique fora ou com o Escape. O relatório do
//! Tabernáculo, C15.

use aylis_core::{Component, ComponentId, EventArgs, EventCtx, Key, KeyEvent, PropEnum, PropValue, Ui, properties};

use crate::Rectangle;

/// De que lado do componente o [`Popover`] abre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverPlacement {
    /// Embaixo (em cima, se não couber).
    #[default]
    Bottom,
    Top,
    Right,
    Left,
}

impl PropEnum for PopoverPlacement {
    const NAMES: &'static [&'static str] = &["bottom", "top", "right", "left"];
    fn from_index(i: usize) -> Option<Self> {
        [Self::Bottom, Self::Top, Self::Right, Self::Left].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Cartão flutuante (o popover das bibliotecas web): desenhado no form
/// com os filhos, fica escondido até `popup(anchor)`, que o abre ao lado de `anchor`, por cima
/// de tudo e sem sair do form. Fecha com o clique fora, o Escape ou `close()`, e avisa em
/// `on_close`. A aparência é o estilo `Popover` do tema.
pub struct Popover {
    pub base: Rectangle,
    /// De que lado do componente abre: `bottom` (em cima, se não couber), `top`, `right` ou
    /// `left`.
    pub placement: PopoverPlacement,
    /// A distância até o componente.
    pub gap: f32,
    anchor: Option<ComponentId>,
}

impl Default for Popover {
    fn default() -> Self {
        let mut base = Rectangle::new(260.0, 160.0);
        base.base.set_visible(false).set_can_focus(true).set_tab_stop(false);
        Self { base, placement: PopoverPlacement::Bottom, gap: 6.0, anchor: None }
    }
}

impl Popover {
    /// Abre ao lado de `anchor` (do mesmo form).
    pub fn popup(ui: &mut Ui, id: ComponentId, anchor: ComponentId) -> Result<(), String> {
        let (placement, gap) = ui.get::<Popover>(id).map(|p| (p.placement, p.gap)).ok_or("não é um Popover")?;
        let root = ui.root_of(anchor);
        // Na raiz do form, a última: por cima de tudo e fora do recorte dos contêineres.
        ui.set_parent(id, Some(root)).map_err(|e| e.to_string())?;
        let target = ui.absolute_rect(anchor).unwrap_or_default();
        // O form ou, com ele em vários monitores, o pedaço em que está a âncora.
        let form = ui.popup_area(root, aylis_core::PointF::new(target.x + target.width / 2.0, target.y + target.height / 2.0));
        let (w, h) = ui.control(id).map(|cb| (cb.width(), cb.height())).unwrap_or((260.0, 160.0));
        let below = target.bottom() + gap;
        let above = target.y - gap - h;
        let (x, y) = match placement {
            PopoverPlacement::Bottom if below + h > form.bottom() && above >= form.y => (target.x, above),
            PopoverPlacement::Bottom => (target.x, below),
            PopoverPlacement::Top if above < form.y => (target.x, below),
            PopoverPlacement::Top => (target.x, above),
            PopoverPlacement::Right => (target.right() + gap, target.y),
            PopoverPlacement::Left => (target.x - gap - w, target.y),
        };
        let x = x.min(form.right() - w - 4.0).max(form.x + 4.0);
        let y = y.min(form.bottom() - h - 4.0).max(form.y + 4.0);
        if let Some(cb) = ui.control_mut(id) {
            cb.set_align(aylis_core::Align::None);
            cb.set_rect(x, y, w, h);
            cb.set_visible(true);
        }
        if let Some(p) = ui.get_mut::<Popover>(id) {
            p.anchor = Some(anchor);
        }
        ui.follow(id, anchor);
        ui.set_focus(id);
        ui.invalidate();
        Ok(())
    }

    /// Fecha (esconde) e avisa em `on_close`.
    pub fn close(ui: &mut Ui, id: ComponentId) {
        let open = ui.control(id).is_some_and(|cb| cb.visible());
        if let Some(cb) = ui.control_mut(id) {
            cb.set_visible(false);
        }
        if open {
            ui.queue_event(id, "on_close", EventArgs::None);
        }
        ui.invalidate();
    }

    /// Esconde por dentro (o componente em uso: o clique fora, o Escape, o `close()`).
    fn hide(&mut self, ctx: &mut EventCtx<'_>) {
        if self.base.base.visible() {
            self.base.base.set_visible(false);
            ctx.ui.queue_event(ctx.id, "on_close", EventArgs::None);
            ctx.invalidate();
        }
    }

    /// Está aberto.
    pub fn is_open(ui: &Ui, id: ComponentId) -> bool {
        ui.control(id).is_some_and(|cb| cb.visible())
    }
}

impl Component for Popover {
    fn paint_is_pure(&self) -> bool {
        true
    }

    fn class_name(&self) -> &'static str {
        "Popover"
    }

    properties! {
        placement: PopoverPlacement => "Behavior",
        gap: f32 => "Behavior";
        events: on_close
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[MethodInfo::new("popup", &[PropKind::Component], None).params(&["anchor"]), MethodInfo::new("close", &[], None)];
        METHODS
    }

    /// `popup(anchor)` abre ao lado do componente; `close()` fecha.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let id = ctx.id;
        match method {
            "popup" => {
                let Some(PropValue::Component(Some(anchor))) = args.first().cloned() else { return Err("popup(anchor): falta o componente".into()) };
                ctx.ui.poster().post(move |ui| {
                    let _ = Popover::popup(ui, id, anchor);
                });
                Ok(PropValue::Null)
            }
            "close" => {
                self.hide(ctx);
                Ok(PropValue::Null)
            }
            _ => Err(format!("método desconhecido: {method}")),
        }
    }

    fn closes_on_outside_press(&self) -> bool {
        true
    }

    fn dismissed(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        self.hide(ctx);
        true
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Escape {
            self.hide(ctx);
            ctx.set_handled();
        }
    }
}
