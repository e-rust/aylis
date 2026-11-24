//! Windows: os cursores de mão (`CursorShape::Grab` e `Grabbing`). O Windows não tem mão aberta
//! nem fechada entre os cursores do sistema, e o winit dava a seta de mover (`IDC_SIZEALL`) no
//! lugar delas. Aqui, as duas mãos desenhadas em 16 × 16 e ampliadas para a escala do monitor
//! (cada pixel vira um quadrado, como os cursores antigos).

use aylis_core::CursorShape;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Cursor, CustomCursor};

/// A mão aberta: `X` contorno, `o` preenchimento, `.` transparente.
const OPEN: [&str; 16] = [
    ".......XX.......",
    "......XooXXX....",
    "....XXXooXooX...",
    "...XooXooXooXXX.",
    "...XooXooXooXooX",
    "...XooXooXooXooX",
    ".XXXooXooXooXooX",
    "XooXoooooooooooX",
    "XooooooooooooooX",
    ".XoooooooooooooX",
    ".XoooooooooooooX",
    "..XooooooooooooX",
    "..XoooooooooooX.",
    "...XooooooooooX.",
    "....XooooooooX..",
    "....XXXXXXXXXX..",
];

/// A mão fechada (segurando).
const CLOSED: [&str; 16] = [
    "................",
    "................",
    "................",
    "................",
    "....XX.XX.XX.XX.",
    "...XooXooXooXooX",
    ".XXXoooooooooooX",
    "XooXoooooooooooX",
    "XooooooooooooooX",
    ".XoooooooooooooX",
    "..XooooooooooooX",
    "..XoooooooooooX.",
    "...XooooooooooX.",
    "....XooooooooX..",
    "....XXXXXXXXXX..",
    "................",
];

/// O ponto do desenho (em pixels de 16 × 16) que fica na ponta do mouse: o meio da palma.
const HOTSPOT: (u16, u16) = (8, 8);

/// Os pixels RGBA do desenho ampliado `factor` vezes (cada pixel vira um quadrado).
pub(crate) fn rgba(art: &[&str; 16], factor: u32) -> Vec<u8> {
    let side = 16 * factor as usize;
    let mut out = vec![0u8; side * side * 4];
    for (y, row) in art.iter().enumerate() {
        for (x, c) in row.bytes().enumerate() {
            let color = match c {
                b'X' => [0, 0, 0, 255],
                b'o' => [255, 255, 255, 255],
                _ => continue,
            };
            for dy in 0..factor as usize {
                for dx in 0..factor as usize {
                    let (px, py) = (x * factor as usize + dx, y * factor as usize + dy);
                    let i = (py * side + px) * 4;
                    out[i..i + 4].copy_from_slice(&color);
                }
            }
        }
    }
    out
}

/// As duas mãos em cada escala inteira, de 1 a 4 (a de cada monitor sai da mais perto).
pub(crate) fn create(event_loop: &ActiveEventLoop) -> Vec<(u32, CustomCursor, CustomCursor)> {
    (1..=4)
        .filter_map(|factor| {
            let side = 16 * factor as u16;
            let make = |art: &[&str; 16]| {
                let source = CustomCursor::from_rgba(rgba(art, factor), side, side, HOTSPOT.0 * factor as u16, HOTSPOT.1 * factor as u16).ok()?;
                Some(event_loop.create_custom_cursor(source))
            };
            Some((factor, make(&OPEN)?, make(&CLOSED)?))
        })
        .collect()
}

/// O cursor da janela para `shape`: as mãos desenhadas, na escala mais perto da janela
/// (`scale`); os outros, os do sistema (pelo winit).
pub(crate) fn cursor(hands: &[(u32, CustomCursor, CustomCursor)], scale: f64, shape: CursorShape) -> Cursor {
    if matches!(shape, CursorShape::Grab | CursorShape::Grabbing) {
        let want = scale.round().max(1.0) as u32;
        if let Some((_, open, closed)) = hands.iter().min_by_key(|(f, ..)| f.abs_diff(want)) {
            return if shape == CursorShape::Grab { open.clone() } else { closed.clone() }.into();
        }
    }
    crate::input::cursor(shape).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hands_are_16_by_16_and_scale_to_the_monitor() {
        for art in [&OPEN, &CLOSED] {
            assert!(art.iter().all(|row| row.len() == 16 && row.bytes().all(|c| matches!(c, b'X' | b'o' | b'.'))));
        }
        let px = rgba(&OPEN, 2);
        assert_eq!(px.len(), 32 * 32 * 4);
        // O contorno do alto do dedo do meio (7, 0) vira o quadrado 14..16 × 0..2.
        let at = |x: usize, y: usize| &px[(y * 32 + x) * 4..(y * 32 + x) * 4 + 4];
        assert_eq!(at(14, 0), &[0, 0, 0, 255]);
        assert_eq!(at(15, 1), &[0, 0, 0, 255]);
        assert_eq!(at(0, 0), &[0, 0, 0, 0], "fora da mão, transparente");
        // A ponta do mouse cai na palma, não fora da mão.
        assert_eq!(OPEN[HOTSPOT.1 as usize].as_bytes()[HOTSPOT.0 as usize], b'o');
        assert_eq!(CLOSED[HOTSPOT.1 as usize].as_bytes()[HOTSPOT.0 as usize], b'o');
    }
}
