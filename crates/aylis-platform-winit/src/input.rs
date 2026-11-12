//! Tradução dos eventos do winit para os tipos do aylis.

use aylis_core::{CursorShape, Key, KeyEvent, Modifiers, PointerButton};
use winit::event::{ElementState, MouseButton};
use winit::keyboard::{Key as WinitKey, ModifiersState, NamedKey};
use winit::window::CursorIcon;

pub(crate) fn modifiers(state: ModifiersState) -> Modifiers {
    Modifiers {
        shift: state.shift_key(),
        ctrl: state.control_key(),
        alt: state.alt_key(),
        meta: state.super_key(),
    }
}

pub(crate) fn button(button: MouseButton) -> Option<PointerButton> {
    Some(match button {
        MouseButton::Left => PointerButton::Primary,
        MouseButton::Right => PointerButton::Secondary,
        MouseButton::Middle => PointerButton::Middle,
        MouseButton::Back => PointerButton::Back,
        MouseButton::Forward => PointerButton::Forward,
        MouseButton::Other(_) => return None,
    })
}

pub(crate) fn key_event(event: &winit::event::KeyEvent, modifiers: Modifiers) -> KeyEvent {
    KeyEvent { key: key(&event.logical_key), modifiers, repeat: event.repeat }
}

pub(crate) fn is_pressed(event: &winit::event::KeyEvent) -> bool {
    event.state == ElementState::Pressed
}

fn key(key: &WinitKey) -> Key {
    match key {
        WinitKey::Named(named) => match named {
            NamedKey::Enter => Key::Enter,
            NamedKey::Tab => Key::Tab,
            NamedKey::Escape => Key::Escape,
            NamedKey::Backspace => Key::Backspace,
            NamedKey::Delete => Key::Delete,
            NamedKey::Insert => Key::Insert,
            NamedKey::Home => Key::Home,
            NamedKey::End => Key::End,
            NamedKey::PageUp => Key::PageUp,
            NamedKey::PageDown => Key::PageDown,
            NamedKey::ArrowLeft => Key::Left,
            NamedKey::ArrowRight => Key::Right,
            NamedKey::ArrowUp => Key::Up,
            NamedKey::ArrowDown => Key::Down,
            NamedKey::Space => Key::Space,
            NamedKey::Shift => Key::Shift,
            NamedKey::Control => Key::Control,
            NamedKey::Alt => Key::Alt,
            NamedKey::Super | NamedKey::Meta => Key::Meta,
            NamedKey::F1 => Key::F(1),
            NamedKey::F2 => Key::F(2),
            NamedKey::F3 => Key::F(3),
            NamedKey::F4 => Key::F(4),
            NamedKey::F5 => Key::F(5),
            NamedKey::F6 => Key::F(6),
            NamedKey::F7 => Key::F(7),
            NamedKey::F8 => Key::F(8),
            NamedKey::F9 => Key::F(9),
            NamedKey::F10 => Key::F(10),
            NamedKey::F11 => Key::F(11),
            NamedKey::F12 => Key::F(12),
            _ => Key::Unknown,
        },
        WinitKey::Character(text) => match text.chars().next() {
            Some(' ') => Key::Space,
            Some(c) => Key::Char(c.to_lowercase().next().unwrap_or(c)),
            None => Key::Unknown,
        },
        _ => Key::Unknown,
    }
}

pub(crate) fn cursor(shape: CursorShape) -> CursorIcon {
    match shape {
        CursorShape::Default => CursorIcon::Default,
        CursorShape::Pointer => CursorIcon::Pointer,
        CursorShape::Text => CursorIcon::Text,
        CursorShape::Wait => CursorIcon::Wait,
        CursorShape::Crosshair => CursorIcon::Crosshair,
        CursorShape::Move => CursorIcon::Move,
        CursorShape::NotAllowed => CursorIcon::NotAllowed,
        CursorShape::ResizeHorizontal => CursorIcon::EwResize,
        CursorShape::ResizeVertical => CursorIcon::NsResize,
        CursorShape::ResizeNeSw => CursorIcon::NeswResize,
        CursorShape::ResizeNwSe => CursorIcon::NwseResize,
        CursorShape::Grab => CursorIcon::Grab,
        CursorShape::Grabbing => CursorIcon::Grabbing,
        // Escondido à parte (`set_cursor_visible`); o ícone fica o normal.
        CursorShape::None => CursorIcon::Default,
    }
}

/// Um deslocamento em pixels que é dente de roda: o navegador manda a roda do mouse em pixels,
/// cerca de 100 por dente no Chrome e no Edge, mas com a escala do Windows sai quebrado
/// (66,7 em 150%). Um passo grande num eixo só é roda: os dentes em `x` e `y` (pelo menos
/// um). `None` para o touchpad, que manda passos pequenos.
pub(crate) fn wheel_notches(x: f32, y: f32) -> Option<(f32, f32)> {
    const NOTCH: f32 = 100.0;
    const MIN: f32 = 40.0;
    let notches = |v: f32| v.signum() * (v.abs() / NOTCH).round().max(1.0);
    match (x.abs() >= MIN, y.abs() >= MIN) {
        (false, true) if x == 0.0 => Some((0.0, notches(y))),
        (true, false) if y == 0.0 => Some((notches(x), 0.0)),
        _ => None,
    }
}

#[cfg(test)]
mod wheel_tests {
    use super::wheel_notches;

    #[test]
    fn browser_wheel_pixels_become_notches_and_touchpad_stays_pixels() {
        assert_eq!(wheel_notches(0.0, -100.0), Some((0.0, -1.0)));
        assert_eq!(wheel_notches(0.0, 200.0), Some((0.0, 2.0)));
        assert_eq!(wheel_notches(0.0, 3.5), None);
        // Com a escala do Windows em 150%: um dente.
        assert_eq!(wheel_notches(0.0, -66.67), Some((0.0, -1.0)));
        assert_eq!(wheel_notches(-133.3, 0.0), Some((-1.0, 0.0)));
        assert_eq!(wheel_notches(12.0, -100.0), None);
    }
}
