//! Controles de jogo no Android. A `NativeActivity` entrega a entrada ao winit, não ao Java:
//! os botões chegam como teclas (o código do Android, que o winit não traduz) e o analógico
//! esquerdo como um "toque" que só se move, vindo do aparelho do controle. Aqui esses eventos
//! viram o estado do controle e não chegam à tela. O Java só lista os controles (id e nome)
//! e vibra. O winit descarta o analógico direito e os gatilhos analógicos (`lt`/`rt` ficam
//! como botões, que a maioria dos controles também manda).

use std::sync::Mutex;
use std::time::{Duration, Instant};

use aylis_core::platform::{GamepadService, GamepadState};
use jni::objects::{JString, JValue};
use winit::event::{DeviceId, KeyEvent, TouchPhase};
use winit::keyboard::{NativeKeyCode, PhysicalKey};

use crate::android_jni::with_env;

struct Pad {
    device: i32,
    state: GamepadState,
}

struct Pads {
    list: Vec<Pad>,
    /// Quando a lista veio do Java pela última vez.
    listed: Option<Instant>,
}

static PADS: Mutex<Pads> = Mutex::new(Pads { list: Vec::new(), listed: None });

/// Os controles (id e nome) pelo `InputDevice` do Android, mantendo o estado dos que seguem.
fn refresh(pads: &mut Pads) {
    pads.listed = Some(Instant::now());
    let text = with_env(|env, a| {
        let s = env.call_method(a, "gamepads", "()Ljava/lang/String;", &[])?.l()?;
        let s: String = env.get_string(&JString::from(s))?.into();
        Ok(s)
    })
    .unwrap_or_default();
    let mut list = Vec::new();
    for line in text.lines() {
        let Some((id, name)) = line.split_once('\t') else { continue };
        let Ok(device) = id.parse::<i32>() else { continue };
        let state = pads.list.iter().find(|p| p.device == device).map(|p| p.state.clone());
        list.push(Pad { device, state: state.unwrap_or_else(|| GamepadState { name: name.to_string(), ..Default::default() }) });
    }
    pads.list = list;
}

/// O número do aparelho no Android (o winit não expõe; o `Debug` mostra `DeviceId(DeviceId(7))`).
fn device_number(id: &DeviceId) -> Option<i32> {
    let text = format!("{id:?}");
    let digits: String = text.chars().filter(|c| c.is_ascii_digit() || *c == '-').collect();
    digits.parse().ok()
}

/// Acha o controle do aparelho; relista (no máximo a cada segundo) se ele ainda não está.
fn with_pad<R>(device: i32, f: impl FnOnce(&mut GamepadState) -> R) -> Option<R> {
    let mut pads = PADS.lock().ok()?;
    if !pads.list.iter().any(|p| p.device == device) && pads.listed.is_none_or(|t| t.elapsed() > Duration::from_secs(1)) {
        refresh(&mut pads);
    }
    pads.list.iter_mut().find(|p| p.device == device).map(|p| f(&mut p.state))
}

/// O botão do controle de um código de tecla do Android (`KeyEvent.KEYCODE_*`).
fn button_name(code: u32) -> Option<&'static str> {
    Some(match code {
        96 => "a",
        97 => "b",
        99 => "x",
        100 => "y",
        102 => "lb",
        103 => "rb",
        104 => "lt",
        105 => "rt",
        106 => "l3",
        107 => "r3",
        108 => "start",
        109 => "back",
        110 => "guide",
        19 => "up",
        20 => "down",
        21 => "left",
        22 => "right",
        _ => return None,
    })
}

/// Uma tecla: se veio de um controle, vira botão. `true`: era do controle e não vai à tela
/// (as setas do direcional seguem também como teclas, para andar pelo foco).
pub(crate) fn key(device: &DeviceId, event: &KeyEvent) -> bool {
    let PhysicalKey::Unidentified(NativeKeyCode::Android(code)) = event.physical_key else {
        return arrow(device, event);
    };
    let Some(name) = button_name(code) else { return false };
    let Some(device) = device_number(device) else { return false };
    let down = event.state.is_pressed();
    with_pad(device, |s| s.set_button(name, down)).is_some()
}

/// As setas do direcional chegam traduzidas (`ArrowUp`...): marca o botão e deixa passar.
fn arrow(device: &DeviceId, event: &KeyEvent) -> bool {
    use winit::keyboard::KeyCode;
    let name = match event.physical_key {
        PhysicalKey::Code(KeyCode::ArrowUp) => "up",
        PhysicalKey::Code(KeyCode::ArrowDown) => "down",
        PhysicalKey::Code(KeyCode::ArrowLeft) => "left",
        PhysicalKey::Code(KeyCode::ArrowRight) => "right",
        _ => return false,
    };
    if let Some(device) = device_number(device) {
        let down = event.state.is_pressed();
        let _ = with_pad(device, |s| s.set_button(name, down));
    }
    false
}

/// Um "toque" do aparelho de um controle é o analógico esquerdo (x e y de -1 a 1).
/// `true`: era do controle e não vai à tela.
pub(crate) fn touch(device: &DeviceId, phase: TouchPhase, x: f64, y: f64) -> bool {
    // O analógico só manda movimento, sempre dentro de -1..1.
    if phase != TouchPhase::Moved || x.abs() > 1.01 || y.abs() > 1.01 {
        return false;
    }
    let Some(device) = device_number(device) else { return false };
    with_pad(device, |s| {
        s.axes[0] = x as f32;
        s.axes[1] = y as f32;
    })
    .is_some()
}

pub(crate) struct AndroidGamepads;

impl GamepadService for AndroidGamepads {
    fn pads(&mut self) -> Vec<GamepadState> {
        let Ok(mut pads) = PADS.lock() else { return Vec::new() };
        if pads.listed.is_none_or(|t| t.elapsed() > Duration::from_secs(1)) {
            refresh(&mut pads);
        }
        pads.list.iter().map(|p| p.state.clone()).collect()
    }

    fn rumble(&mut self, index: usize, strength: f32, ms: u32) -> bool {
        let device = PADS.lock().ok().and_then(|p| p.list.get(index).map(|p| p.device));
        let Some(device) = device else { return false };
        let amplitude = (strength.clamp(0.0, 1.0) * 255.0).round().max(1.0) as i32;
        with_env(|env, a| {
            env.call_method(a, "gamepadRumble", "(III)Z", &[JValue::Int(device), JValue::Int(amplitude), JValue::Int(ms.min(i32::MAX as u32) as i32)])?.z()
        })
        .unwrap_or(false)
    }
}
