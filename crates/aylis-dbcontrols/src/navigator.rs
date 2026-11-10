//! [`DataNavigator`]: botões de navegação e edição de um dataset.

use std::any::Any;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, PaintCtx, PointF,
    PointerEvent, RectF, Stroke, Ui, properties,
};
use aylis_data::{DataEvent, DataMessage, DataSource, data_set, view};

/// Botões do navegador, na ordem em que aparecem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavButton {
    First,
    Prior,
    Next,
    Last,
    Insert,
    Delete,
    Edit,
    Post,
    Cancel,
}

impl NavButton {
    pub const ALL: [NavButton; 9] = [
        NavButton::First,
        NavButton::Prior,
        NavButton::Next,
        NavButton::Last,
        NavButton::Insert,
        NavButton::Delete,
        NavButton::Edit,
        NavButton::Post,
        NavButton::Cancel,
    ];

    /// O nome do botão no `before_action` (`"first"`, `"post"`...).
    pub fn name(self) -> &'static str {
        match self {
            NavButton::First => "first",
            NavButton::Prior => "prior",
            NavButton::Next => "next",
            NavButton::Last => "last",
            NavButton::Insert => "insert",
            NavButton::Delete => "delete",
            NavButton::Edit => "edit",
            NavButton::Post => "post",
            NavButton::Cancel => "cancel",
        }
    }
}

/// Situação do dataset, guardada para a pintura (que não acessa o `Ui`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct NavState {
    active: bool,
    empty: bool,
    at_first: bool,
    at_last: bool,
    editing: bool,
    /// A gravação espera o backend: nada se mexe até a resposta.
    waiting: bool,
    read_only: bool,
}

/// Navegador (como o `TDBNavigator`): primeiro, anterior, próximo, último, incluir,
/// excluir, editar, gravar e cancelar. Os botões se habilitam conforme o estado do
/// dataset. Erros (por exemplo um campo obrigatório no `post`) disparam `on_error`.
pub struct DataNavigator {
    pub base: ControlBase,
    pub data_source: Option<ComponentId>,
    state: NavState,
    hovered: Option<usize>,
    pressed: Option<usize>,
}

impl Default for DataNavigator {
    fn default() -> Self {
        let mut base = ControlBase::new(NavButton::ALL.len() as f32 * 34.0, 32.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, data_source: None, state: NavState::default(), hovered: None, pressed: None }
    }
}

impl DataNavigator {
    pub fn new(data_source: ComponentId) -> Self {
        Self { data_source: Some(data_source), ..Default::default() }
    }

    fn data_set(&self, ui: &Ui) -> Option<ComponentId> {
        let source = ui.get::<DataSource>(self.data_source?)?;
        if source.enabled { source.data_set } else { None }
    }

    fn refresh(&mut self, ui: &Ui) {
        self.state = match self.data_set(ui) {
            Some(ds) => {
                let v = view(ui, ds);
                NavState {
                    active: v.is_active(),
                    empty: v.is_empty(),
                    at_first: v.at_first(),
                    at_last: v.at_last(),
                    editing: v.state().is_editing(),
                    waiting: v.is_waiting(),
                    read_only: ui.get::<aylis_data::ClientDataSet>(ds).is_some_and(|c| c.read_only),
                }
            }
            None => NavState::default(),
        };
    }

    pub fn is_enabled(&self, button: NavButton) -> bool {
        let s = self.state;
        if !s.active || !self.base.enabled() || s.waiting {
            return false;
        }
        match button {
            NavButton::First | NavButton::Prior => !s.empty && !s.at_first && !s.editing,
            NavButton::Next | NavButton::Last => !s.empty && !s.at_last && !s.editing,
            NavButton::Insert => !s.read_only && !s.editing,
            NavButton::Delete | NavButton::Edit => !s.read_only && !s.empty && !s.editing,
            NavButton::Post | NavButton::Cancel => s.editing,
        }
    }

    fn index_at(&self, x: f32) -> Option<usize> {
        let width = self.base.bounds().width / NavButton::ALL.len() as f32;
        (x >= 0.0 && width > 0.0).then(|| (x / width) as usize).filter(|&i| i < NavButton::ALL.len())
    }

    /// Executa a ação de um botão (também pode ser chamado por código).
    pub fn press(&mut self, ui: &mut Ui, id: ComponentId, button: NavButton) {
        self.refresh(ui);
        if !self.is_enabled(button) {
            return;
        }
        let Some(ds) = self.data_set(ui) else { return };
        let mut ds = data_set(ui, ds);
        let result = match button {
            NavButton::First => ds.first(),
            NavButton::Prior => ds.prior(),
            NavButton::Next => ds.next(),
            NavButton::Last => ds.last(),
            NavButton::Insert => ds.insert(),
            NavButton::Delete => ds.delete(),
            NavButton::Edit => ds.edit(),
            NavButton::Post => ds.post(),
            NavButton::Cancel => ds.cancel(),
        };
        if let Err(e) = result {
            ui.queue_event(id, "on_error", EventArgs::Text(e.message));
        }
        self.refresh(ui);
    }
}

impl Component for DataNavigator {
    fn class_name(&self) -> &'static str {
        "DataNavigator"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data";
        events: before_action, on_error
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        let Some(m) = message.downcast_ref::<DataMessage>() else { return };
        if self.data_set(ctx.ui) == Some(m.data_set) && m.event != DataEvent::UpdateData {
            self.refresh(ctx.ui);
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hovered = self.index_at(event.position.x);
        if hovered != self.hovered {
            self.hovered = hovered;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered = None;
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pressed = self.index_at(event.position.x);
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        ctx.repaint();
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(i) = self.pressed.take() {
            let id = ctx.id;
            let button = NavButton::ALL[i];
            // O `before_action` (o `BeforeAction` do Delphi) pergunta antes, e passa pelos
            // `EventInterceptor`: recusado, o botão não age. Sem ninguém para perguntar, age já.
            if ctx.ui.has_handler(id, "before_action") || ctx.ui.is_intercepted(id, "before_action") {
                ctx.ui.poster().post(move |ui| ask_before_action(ui, id, button));
            } else {
                self.press(ctx.ui, id, button);
            }
            ctx.repaint();
        }
        ctx.fire("on_click", EventArgs::None);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 6.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r, 6.0, &Stroke::new(theme.color("border"), 1.0));
        let width = r.width / NavButton::ALL.len() as f32;
        for (i, &button) in NavButton::ALL.iter().enumerate() {
            let cell = RectF::new(r.x + i as f32 * width, r.y, width, r.height);
            let enabled = self.is_enabled(button);
            if enabled && self.hovered == Some(i) {
                let overlay = if ctx.state.pressed && self.pressed == Some(i) { 0.16 } else { 0.08 };
                canvas.fill_rect(cell.inflate(-3.0), 4.0, &Brush::Solid(theme.color("text").with_opacity(overlay)));
            }
            if i == 4 || i == 7 {
                let x = cell.x.round() + 0.5;
                canvas.draw_line(PointF::new(x, r.y + 6.0), PointF::new(x, r.bottom() - 6.0), &Stroke::new(theme.color("border"), 1.0));
            }
            let color = if enabled {
                match button {
                    NavButton::Delete => theme.color("danger"),
                    NavButton::Post => theme.color("primary"),
                    _ => theme.color("text"),
                }
            } else {
                theme.color("text_muted").with_opacity(0.45)
            };
            draw_icon(canvas, button, cell.center(), color);
        }
    }
}

/// Ícones desenhados com linhas (não dependem de fonte).
fn draw_icon(canvas: &mut dyn Canvas, button: NavButton, c: PointF, color: Color) {
    let stroke = Stroke::new(color, 1.8);
    let mut line = |x1: f32, y1: f32, x2: f32, y2: f32| {
        canvas.draw_line(PointF::new(c.x + x1, c.y + y1), PointF::new(c.x + x2, c.y + y2), &stroke);
    };
    match button {
        NavButton::First => {
            line(-5.0, -5.0, -5.0, 5.0);
            line(4.0, -5.0, -1.0, 0.0);
            line(-1.0, 0.0, 4.0, 5.0);
        }
        NavButton::Prior => {
            line(2.5, -5.0, -2.5, 0.0);
            line(-2.5, 0.0, 2.5, 5.0);
        }
        NavButton::Next => {
            line(-2.5, -5.0, 2.5, 0.0);
            line(2.5, 0.0, -2.5, 5.0);
        }
        NavButton::Last => {
            line(5.0, -5.0, 5.0, 5.0);
            line(-4.0, -5.0, 1.0, 0.0);
            line(1.0, 0.0, -4.0, 5.0);
        }
        NavButton::Insert => {
            line(-5.5, 0.0, 5.5, 0.0);
            line(0.0, -5.5, 0.0, 5.5);
        }
        NavButton::Delete => line(-5.5, 0.0, 5.5, 0.0),
        NavButton::Edit => {
            line(-5.0, 5.0, 4.0, -4.0);
            line(2.0, -6.0, 6.0, -2.0);
            line(-5.0, 5.0, -5.0, 2.0);
            line(-5.0, 5.0, -2.0, 5.0);
        }
        NavButton::Post => {
            line(-5.5, 0.5, -1.5, 4.5);
            line(-1.5, 4.5, 5.5, -4.5);
        }
        NavButton::Cancel => {
            line(-4.5, -4.5, 4.5, 4.5);
            line(4.5, -4.5, -4.5, 4.5);
        }
    }
}

/// Pergunta o `before_action` e, aceito (agora ou quando o backend responder), executa o botão.
fn ask_before_action(ui: &mut Ui, id: ComponentId, button: NavButton) {
    let run = move |ui: &mut Ui, answer: aylis_core::Answer| {
        if let aylis_core::Answer::Refused(message) = answer {
            if !message.is_empty() {
                ui.queue_event(id, "on_error", EventArgs::Text(message));
            }
            return;
        }
        ui.with_component(id, |c, ctx| {
            let any: &mut dyn Any = c;
            if let Some(nav) = any.downcast_mut::<DataNavigator>() {
                nav.press(ctx.ui, ctx.id, button);
                ctx.repaint();
            }
        });
    };
    match ui.ask(id, "before_action", &EventArgs::Text(button.name().into())) {
        aylis_core::Asked::Answered(answer) => run(ui, answer),
        aylis_core::Asked::Deferred(token) => ui.on_answer(token, run),
    }
}
