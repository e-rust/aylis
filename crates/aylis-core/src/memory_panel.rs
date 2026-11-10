//! O painel de memória do app em depuração: um quadro no canto do form principal com os
//! quadros por segundo, a memória do processo, o heap do Rust e as imagens guardadas pelos
//! renderizadores ([`crate::image_cache::stats`]). Só aparece quando o app o liga ([`Ui::set_memory_panel`]):
//! o app compilado com `dev` liga, a publicação nunca. Ctrl+Shift+M mostra e esconde.
//!
//! O núcleo não lê a memória do sistema: a plataforma mede o processo e passa em
//! [`ProcessMemory`]; o heap vem de quem conta as alocações ([`set_heap_counter`]).

use std::sync::OnceLock;

use crate::{Brush, Canvas, Color, FontSpec, RectF, Stroke, TextAlign, TextStyle, Ui};

/// A memória do processo, medida pela plataforma (em bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProcessMemory {
    /// A memória privada comprometida (o "commit"; no Linux, a anônima residente).
    pub private: usize,
    /// O working set: o que está na RAM agora, com as páginas das DLLs compartilhadas.
    pub working_set: usize,
}

static HEAP: OnceLock<fn() -> usize> = OnceLock::new();

/// Quem conta as alocações do Rust (o alocador do app com `dev`) se registra aqui.
pub fn set_heap_counter(counter: fn() -> usize) {
    let _ = HEAP.set(counter);
}

/// Os bytes do heap do Rust agora, se alguém os conta.
pub fn heap() -> Option<usize> {
    HEAP.get().map(|f| f())
}

/// `52,3 MB` (ou `812 KB`), com a vírgula do português.
pub fn format_bytes(bytes: usize) -> String {
    let mb = bytes as f64 / 1_048_576.0;
    if mb >= 1.0 { format!("{mb:.1} MB").replace('.', ",") } else { format!("{} KB", bytes / 1024) }
}

/// A taxa de quadros para o painel: os desenhados no último segundo ([`Ui::fps`]) e o teto
/// do app ([`Ui::max_fps`], 0 = a taxa do monitor).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Frames {
    pub fps: f32,
    pub max_fps: f32,
}

/// As linhas do painel: os quadros, o processo, o heap, o total de imagens e cada cache.
pub fn lines(frames: Option<Frames>, process: Option<ProcessMemory>) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(f) = frames {
        // Com a tela parada, só a repintura do próprio painel (1 por segundo) conta.
        let limit = if f.max_fps > 0.0 { format!(" (limite {:.0})", f.max_fps) } else { String::new() };
        out.push(format!("Quadros: {:.0} por segundo{limit}", f.fps));
    }
    match process {
        Some(p) => out.push(format!("Processo: {} comprometida · {} working set", format_bytes(p.private), format_bytes(p.working_set))),
        None => out.push("Processo: — (esta plataforma não informa)".into()),
    }
    if let Some(h) = heap() {
        out.push(format!("Heap do Rust: {}", format_bytes(h)));
    }
    let stats = crate::image_cache::stats();
    let (images, bytes) = stats.iter().fold((0, 0), |(i, b), s| (i + s.images, b + s.bytes));
    out.push(format!("Imagens no cache: {images} · {}", format_bytes(bytes)));
    for s in &stats {
        out.push(format!("  {}: {} · {}", s.name, s.images, format_bytes(s.bytes)));
    }
    out
}

/// Pinta o painel no canto de cima à direita de `form` (o retângulo do form, em pontos).
pub fn paint(canvas: &mut dyn Canvas, form: RectF, frames: Option<Frames>, process: Option<ProcessMemory>) {
    let lines = lines(frames, process);
    let style = TextStyle { font: FontSpec { size: 11.0, ..FontSpec::default() }, color: Color::WHITE, v_align: TextAlign::Center, ..TextStyle::default() };
    let line_h = 16.0;
    let width = lines.iter().map(|l| canvas.measure_text(l, &style).width).fold(0.0f32, f32::max) + 20.0;
    let height = line_h * lines.len() as f32 + 12.0;
    let r = RectF::new(form.right() - width - 8.0, form.y + 8.0, width, height);
    canvas.fill_rect(r, 6.0, &Brush::Solid(Color::rgba(0x10, 0x12, 0x16, 0xE0)));
    canvas.stroke_rect(r, 6.0, &Stroke::new(Color::WHITE.with_opacity(0.15), 1.0));
    for (i, line) in lines.iter().enumerate() {
        let muted = line.starts_with("  ");
        let s = TextStyle { color: if muted { Color::WHITE.with_opacity(0.65) } else { Color::WHITE }, ..style.clone() };
        canvas.draw_text(line, RectF::new(r.x + 10.0, r.y + 6.0 + line_h * i as f32, width - 20.0, line_h), &s);
    }
}

impl Ui {
    /// Liga o painel de memória (`Some(visível)`) ou o deixa indisponível (`None`, o padrão:
    /// o atalho também não faz nada). O app com `dev` liga.
    pub fn set_memory_panel(&mut self, panel: Option<bool>) {
        self.memory_panel = panel;
        self.request_repaint();
    }

    /// Se o painel está à vista agora.
    pub fn memory_panel_visible(&self) -> bool {
        self.memory_panel == Some(true)
    }

    /// Ctrl+Shift+M: mostra ou esconde o painel, se ele estiver disponível. `true` se a tecla
    /// foi dele.
    pub fn toggle_memory_panel(&mut self) -> bool {
        let Some(visible) = self.memory_panel else { return false };
        self.memory_panel = Some(!visible);
        self.invalidate();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_read_in_portuguese() {
        assert_eq!(format_bytes(52 * 1_048_576 + 300 * 1024), "52,3 MB");
        assert_eq!(format_bytes(812 * 1024), "812 KB");
    }
}
