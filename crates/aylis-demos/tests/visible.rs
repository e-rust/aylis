//! Cada componente que a documentação mostra ao vivo aparece de verdade: desenhado sem janela
//! no tamanho do demo, tem desenho (não é um retângulo liso) e cabe no quadro.

use aylis::prelude::*;

/// A fração dos pixels do componente que não são o fundo do form.
fn ink(rgba: &[u8], width: u32, rect: RectF, background: Color) -> f32 {
    let (bg, mut inked, mut total) = ([background.r as i32, background.g as i32, background.b as i32], 0u32, 0u32);
    for y in rect.y.max(0.0) as u32..rect.bottom() as u32 {
        for x in rect.x.max(0.0) as u32..(rect.right() as u32).min(width) {
            let i = ((y * width + x) * 4) as usize;
            let Some(p) = rgba.get(i..i + 3) else { continue };
            total += 1;
            if p.iter().zip(bg).any(|(&c, b)| (c as i32 - b).abs() > 12) {
                inked += 1;
            }
        }
    }
    if total == 0 { 0.0 } else { inked as f32 / total as f32 }
}

/// Começam do zero e se desenham com o tempo (no primeiro quadro, ainda nada).
const ANIMATED_FROM_ZERO: &[&str] = &["AjaxBar"];

#[test]
fn every_live_demo_draws_something_and_fits() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let (mut blank, mut outside) = (Vec::new(), Vec::new());
    for family in aylis_tools::publish::demo_families() {
        for class in &family.classes {
            let mut ui = Ui::new();
            aylis::register_all(&mut ui);
            ui.services_mut().text = Some(Box::new(renderer.text_service()));
            let Some(form) = aylis_demos::demo_form(&mut ui, class, true) else { continue };
            ui.layout();
            let (w, h) = ui.control(form).map(|cb| (cb.width(), cb.height())).unwrap();
            let component = ui.find_in(form, "demo").or_else(|| ui.children(form).iter().copied().find(|&c| ui.control(c).is_some())).unwrap();
            let rect = ui.absolute_rect(component).unwrap();
            if !(rect.x >= 0.0 && rect.y >= 0.0 && rect.right() <= w + 0.5 && rect.bottom() <= h + 0.5) {
                outside.push(format!("{class} {rect:?} em {w}x{h}"));
            }
            let background = ui.get_prop(form, "fill").ok().and_then(|v| v.to_color().ok()).unwrap_or_else(|| ui.theme().color("background"));
            let rgba = renderer.render_rgba(w.ceil() as u32, h.ceil() as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
            let amount = ink(&rgba, w.ceil() as u32, rect, background);
            // Para olhar: `AYLIS_DEMO_SHOTS=pasta` grava cada demonstração em PNG.
            if let Ok(dir) = std::env::var("AYLIS_DEMO_SHOTS") {
                let png = renderer.render_png(w.ceil() as u32, h.ceil() as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
                std::fs::write(std::path::Path::new(&dir).join(format!("{class}.png")), png).unwrap();
            }
            if amount < 0.005 && !ANIMATED_FROM_ZERO.contains(&class.as_str()) {
                blank.push(format!("{class} ({:.1}%)", amount * 100.0));
            }
        }
    }
    assert!(blank.is_empty() && outside.is_empty(), "sem desenho: {}
fora do quadro: {}", blank.join(", "), outside.join(", "));
}
