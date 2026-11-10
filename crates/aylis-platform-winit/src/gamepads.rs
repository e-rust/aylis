//! Controles de jogo no desktop e no navegador, pelo `gilrs` (Windows: Windows.Gaming.Input;
//! Linux: evdev; macOS: IOKit; navegador: a Gamepad API). O componente `Gamepad` lê a cada
//! quadro; os eventos do `gilrs` só servem para atualizar o estado.

use gilrs::ff::{BaseEffect, BaseEffectType, Effect, EffectBuilder, Replay, Ticks};
use gilrs::{Axis, Button, Gilrs};
use aylis_core::platform::{GamepadService, GamepadState};

pub(crate) struct GilrsPads {
    gilrs: Gilrs,
    /// As vibrações tocando (soltar o `Effect` para a vibração).
    effects: Vec<Effect>,
}

impl GilrsPads {
    /// `None` se o sistema não deixa ler os controles.
    pub(crate) fn new() -> Option<Self> {
        let gilrs = match Gilrs::new() {
            Ok(g) => g,
            // Sem suporte nesta plataforma: um `Gilrs` vazio, que não acha nada.
            Err(gilrs::Error::NotImplemented(g)) => g,
            Err(_) => return None,
        };
        Some(Self { gilrs, effects: Vec::new() })
    }
}

const BUTTONS: [(Button, &str); 17] = [
    (Button::South, "a"),
    (Button::East, "b"),
    (Button::West, "x"),
    (Button::North, "y"),
    (Button::LeftTrigger, "lb"),
    (Button::RightTrigger, "rb"),
    (Button::LeftTrigger2, "lt"),
    (Button::RightTrigger2, "rt"),
    (Button::Select, "back"),
    (Button::Start, "start"),
    (Button::LeftThumb, "l3"),
    (Button::RightThumb, "r3"),
    (Button::DPadUp, "up"),
    (Button::DPadDown, "down"),
    (Button::DPadLeft, "left"),
    (Button::DPadRight, "right"),
    (Button::Mode, "guide"),
];

impl GamepadService for GilrsPads {
    fn pads(&mut self) -> Vec<GamepadState> {
        while self.gilrs.next_event().is_some() {}
        self.gilrs
            .gamepads()
            .map(|(_, pad)| {
                let mut state = GamepadState { name: pad.name().to_string(), ..Default::default() };
                for (button, name) in BUTTONS {
                    state.set_button(name, pad.is_pressed(button));
                }
                let trigger = |b| pad.button_data(b).map_or(0.0, |d| d.value());
                // O `gilrs` dá o y para cima; o aylis, para baixo (como a tela).
                state.axes = [
                    pad.value(Axis::LeftStickX),
                    -pad.value(Axis::LeftStickY),
                    pad.value(Axis::RightStickX),
                    -pad.value(Axis::RightStickY),
                    trigger(Button::LeftTrigger2),
                    trigger(Button::RightTrigger2),
                ];
                state
            })
            .collect()
    }

    fn rumble(&mut self, index: usize, strength: f32, ms: u32) -> bool {
        let Some((id, pad)) = self.gilrs.gamepads().nth(index) else { return false };
        if !pad.is_ff_supported() {
            return false;
        }
        let magnitude = (strength.clamp(0.0, 1.0) * f32::from(u16::MAX)) as u16;
        let effect = EffectBuilder::new()
            .add_effect(BaseEffect {
                kind: BaseEffectType::Strong { magnitude },
                scheduling: Replay { play_for: Ticks::from_ms(ms), ..Default::default() },
                ..Default::default()
            })
            .gamepads(&[id])
            .finish(&mut self.gilrs);
        let Ok(effect) = effect else { return false };
        let ok = effect.play().is_ok();
        // Guarda as últimas (as antigas já terminaram ou foram trocadas).
        if self.effects.len() >= 8 {
            self.effects.remove(0);
        }
        self.effects.push(effect);
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O `gilrs` abre nesta máquina; mostra os controles conectados (nenhum também vale).
    #[test]
    fn opens_and_lists_the_connected_pads() {
        let mut pads = GilrsPads::new().expect("o gilrs abre");
        for pad in pads.pads() {
            aylis_core::stderr_line!("controle: {} botões={:#x} eixos={:?}", pad.name, pad.buttons, pad.axes);
        }
    }
}
