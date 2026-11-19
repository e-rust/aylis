//! O argumento tipado dos eventos: cada componente declara o que o evento leva
//! (`events: on_change: f64` no [`crate::properties!`]), o catálogo do acesso tipado escreve o
//! handler com esse tipo nas 8 linguagens, e em Rust o [`EventValue`] tira o valor do
//! [`EventArgs`] (o handler recebe `value: f64`, não o `args` para decifrar).

use crate::component::ComponentId;
use crate::event::{EventArgs, GestureEvent, KeyEvent, PointerEvent, SwipeEvent, WheelEvent};
use crate::prop::{PropEnum, PropKind, PropValue};
use crate::ui::Ui;
use crate::{Color, Date, DateTime, Time};

/// O que um evento leva no `args`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventArg {
    /// Ainda não declarado: o handler recebe o evento inteiro, como antes.
    Unknown,
    /// Nada (o clique, o foco): o handler recebe só quem disparou.
    None,
    Pointer,
    Wheel,
    Gesture,
    Swipe,
    Key,
    /// O intervalo de datas (`início;fim`) do `Calendar` com `range`.
    DateRange,
    /// Um valor do tipo dado (o `PropKind::String` é o texto).
    Value(PropKind),
}

/// Um tipo que um handler recebe como o argumento do evento.
pub trait EventValue: Sized + 'static {
    /// O que o evento declara no catálogo.
    const ARG: EventArg;
    /// O valor tirado do `args` (`None`: o evento veio com outra coisa).
    fn from_args(args: &EventArgs) -> Option<Self>;
}

/// O texto do `args`, venha como `Text` ou como valor texto (ou enum).
fn text_of(args: &EventArgs) -> Option<&str> {
    match args {
        EventArgs::Text(t) => Some(t),
        EventArgs::Value(PropValue::String(t) | PropValue::Enum(t)) => Some(t),
        _ => None,
    }
}

impl EventValue for () {
    const ARG: EventArg = EventArg::None;
    fn from_args(_args: &EventArgs) -> Option<Self> {
        Some(())
    }
}

impl EventValue for bool {
    const ARG: EventArg = EventArg::Value(PropKind::Bool);
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Value(v) => v.to_bool().ok(),
            _ => None,
        }
    }
}

impl EventValue for i64 {
    const ARG: EventArg = EventArg::Value(PropKind::Int);
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Value(v) => v.to_i64().ok(),
            EventArgs::Text(t) => t.trim().parse().ok(),
            _ => None,
        }
    }
}

impl EventValue for f64 {
    const ARG: EventArg = EventArg::Value(PropKind::Float);
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Value(v) => v.to_f64().ok(),
            EventArgs::Text(t) => t.trim().parse().ok(),
            _ => None,
        }
    }
}

impl EventValue for String {
    const ARG: EventArg = EventArg::Value(PropKind::String);
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).map(str::to_string)
    }
}

impl EventValue for Color {
    const ARG: EventArg = EventArg::Value(PropKind::Color);
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Value(PropValue::Color(c)) => Some(*c),
            _ => text_of(args).and_then(Color::from_hex),
        }
    }
}

impl EventValue for Option<Date> {
    const ARG: EventArg = EventArg::Value(PropKind::Date);
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).map(Date::parse_iso)
    }
}

impl EventValue for Option<Time> {
    const ARG: EventArg = EventArg::Value(PropKind::Time);
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).map(Time::parse_iso)
    }
}

impl EventValue for Option<DateTime> {
    const ARG: EventArg = EventArg::Value(PropKind::DateTime);
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).map(DateTime::parse_iso)
    }
}

impl EventValue for Option<crate::Shadow> {
    const ARG: EventArg = EventArg::Value(PropKind::Shadow);
    fn from_args(args: &EventArgs) -> Option<Self> {
        Some(text_of(args).and_then(crate::Shadow::parse))
    }
}

impl EventValue for crate::Easing {
    const ARG: EventArg = EventArg::Value(PropKind::Easing);
    fn from_args(args: &EventArgs) -> Option<Self> {
        Some(text_of(args).and_then(crate::Easing::parse).unwrap_or_default())
    }
}

impl EventValue for Vec<String> {
    const ARG: EventArg = EventArg::Value(PropKind::List);
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).map(|t| t.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect())
    }
}

impl EventValue for Option<ComponentId> {
    const ARG: EventArg = EventArg::Value(PropKind::Component);
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Value(v) => v.to_component().ok(),
            EventArgs::None => Some(None),
            _ => None,
        }
    }
}

impl EventValue for PointerEvent {
    const ARG: EventArg = EventArg::Pointer;
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Pointer(p) => Some(*p),
            _ => None,
        }
    }
}

impl EventValue for WheelEvent {
    const ARG: EventArg = EventArg::Wheel;
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Wheel(w) => Some(*w),
            _ => None,
        }
    }
}

impl EventValue for GestureEvent {
    const ARG: EventArg = EventArg::Gesture;
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Gesture(g) => Some(*g),
            _ => None,
        }
    }
}

impl EventValue for SwipeEvent {
    const ARG: EventArg = EventArg::Swipe;
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Swipe(s) => Some(*s),
            _ => None,
        }
    }
}

impl EventValue for KeyEvent {
    const ARG: EventArg = EventArg::Key;
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Key(k) => Some(*k),
            _ => None,
        }
    }
}

impl EventValue for DateRange {
    const ARG: EventArg = EventArg::DateRange;
    fn from_args(args: &EventArgs) -> Option<Self> {
        let text = text_of(args)?;
        let (start, end) = text.split_once(';').unwrap_or((text, ""));
        Some(DateRange { start: Date::parse_iso(start), end: Date::parse_iso(end) })
    }
}

impl<E: PropEnum> EventValue for E {
    const ARG: EventArg = EventArg::Value(PropKind::Enum(E::NAMES));
    fn from_args(args: &EventArgs) -> Option<Self> {
        text_of(args).and_then(E::from_name)
    }
}

/// Um intervalo de datas: o `on_change` do `Calendar` com `range` (o fim é `None` enquanto só
/// o começo foi escolhido).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub start: Option<Date>,
    pub end: Option<Date>,
}

/// Quem disparou o evento, como o handler tipado recebe: o handle do componente com a classe
/// dele (o que o acesso tipado gera), ou o `ComponentId`.
pub trait FromSender: Sized {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self>;
}

impl FromSender for ComponentId {
    fn from_sender(_ui: &Ui, id: ComponentId) -> Option<Self> {
        Some(id)
    }
}
