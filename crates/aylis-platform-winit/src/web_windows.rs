//! Navegador: os forms abertos por cima do principal viram janelas dentro do canvas (barra de
//! título com o caption e o fechar, sombra, arrastar pela barra; o modal escurece o que está
//! por baixo), como no desktop. Numa tela estreita (celular), cada form ocupa a tela toda,
//! como antes.

use std::collections::HashMap;

use aylis_core::canvas::{Brush, Shadow, Stroke, TextAlign, TextStyle};
use aylis_core::{Canvas, Color, ComponentId, CursorShape, FontWeight, ModalResult, PointF, PointerEvent, RectF, SizeF, Ui, WheelEvent};

/// Altura da barra de título.
const TITLE: f32 = 34.0;
/// Largura do canvas (em unidades lógicas) a partir da qual os forms viram janelas.
pub(crate) const MIN_WIDTH: f32 = 720.0;
/// Folga entre a janela e a borda do canvas.
const GAP: f32 = 16.0;
const RADIUS: f32 = 10.0;

/// O que o ponteiro fez.
pub(crate) enum Pointer {
    Move(PointerEvent),
    Down(PointerEvent),
    Up(PointerEvent),
    Wheel(WheelEvent),
    Leave,
}

#[derive(Default)]
pub(crate) struct Floating {
    /// Onde está cada janela (o canto da barra de título), no canvas.
    pos: HashMap<ComponentId, PointF>,
    /// A janela sendo arrastada pela barra e onde ela foi pega.
    drag: Option<(ComponentId, PointF)>,
    /// O form que recebeu o aperto: recebe o resto do arrasto, mesmo fora dele.
    capture: Option<ComponentId>,
    /// O form sob o ponteiro (`None`: sobre uma barra de título).
    hover: Option<ComponentId>,
    /// O fechar sob o ponteiro.
    close_hover: Option<ComponentId>,
}

impl Floating {
    /// As janelas: os forms abertos depois do principal, na ordem (o último fica por cima).
    fn forms(ui: &Ui) -> Vec<ComponentId> {
        ui.open_forms().into_iter().skip(1).collect()
    }

    /// Onde fica o conteúdo de cada form no canvas (o principal em 0,0; as janelas abaixo da
    /// barra) e se ele aparece (um form maximizado esconde os de baixo): as vistas nativas (o
    /// `<iframe>` do `WebView`) vão para lá.
    #[cfg_attr(not(web_webview), allow(dead_code))]
    pub(crate) fn origins(&mut self, ui: &mut Ui, base: ComponentId, canvas: SizeF) -> Vec<(ComponentId, PointF, bool)> {
        let forms = Self::forms(ui);
        let covered = forms.iter().rposition(|&f| Self::full(ui, f));
        let mut out = vec![(base, PointF::new(0.0, 0.0), covered.is_none())];
        for (i, form) in forms.into_iter().enumerate() {
            let frame = self.frame(ui, form, canvas);
            out.push((form, PointF::new(frame.x, frame.y + Self::bar(ui, form)), covered.is_none_or(|c| i >= c)));
        }
        out
    }

    /// O form maximizado ou em tela cheia (`window_state`): ocupa o canvas inteiro, sem a barra
    /// de título (uma página do site, como a documentação), como a janela maximizada no
    /// desktop.
    fn full(ui: &Ui, form: ComponentId) -> bool {
        matches!(ui.get_prop(form, "window_state").ok().and_then(|v| v.to_str().ok().map(str::to_string)).as_deref(), Some("maximized" | "full_screen"))
    }

    /// A altura da barra de título do form (nenhuma no maximizado).
    fn bar(ui: &Ui, form: ComponentId) -> f32 {
        if Self::full(ui, form) { 0.0 } else { TITLE }
    }

    /// O retângulo da janela inteira (com a barra) no canvas. O form fica do tamanho que tem
    /// (o do designer), limitado ao canvas; a primeira vez, no meio. Maximizado, o canvas.
    fn frame(&mut self, ui: &mut Ui, form: ComponentId, canvas: SizeF) -> RectF {
        if Self::full(ui, form) {
            if ui.control(form).is_some_and(|cb| (cb.width(), cb.height()) != (canvas.width, canvas.height)) {
                if let Some(cb) = ui.control_mut(form) {
                    cb.set_size(canvas.width, canvas.height);
                }
                ui.invalidate_form(form);
            }
            return RectF::new(0.0, 0.0, canvas.width, canvas.height);
        }
        let (w, h) = ui.control(form).map_or((400.0, 300.0), |cb| (cb.width(), cb.height()));
        let (fw, fh) = (w.min(canvas.width - 2.0 * GAP).max(160.0), h.min(canvas.height - 2.0 * GAP - TITLE).max(80.0));
        if (fw, fh) != (w, h) {
            if let Some(cb) = ui.control_mut(form) {
                cb.set_size(fw, fh);
            }
        }
        let p = *self.pos.entry(form).or_insert_with(|| PointF::new(((canvas.width - fw) / 2.0).max(GAP), ((canvas.height - fh - TITLE) / 2.0).max(GAP)));
        // A barra sempre alcançável (a janela não some fora do canvas).
        let x = p.x.clamp(GAP - fw + 120.0, (canvas.width - 120.0).max(GAP));
        let y = p.y.clamp(0.0, (canvas.height - TITLE).max(0.0));
        RectF::new(x, y, fw, fh + TITLE)
    }

    fn close_rect(frame: RectF) -> RectF {
        RectF::new(frame.right() - TITLE - 4.0, frame.y, TITLE + 4.0, TITLE)
    }

    /// Pinta as janelas por cima do form principal (já pintado).
    pub(crate) fn paint(&mut self, ui: &mut Ui, canvas: SizeF, c: &mut dyn Canvas) {
        self.pos.retain(|f, _| ui.open_forms().contains(f));
        let theme = ui.theme().clone();
        for form in Self::forms(ui) {
            let frame = self.frame(ui, form, canvas);
            let bar = Self::bar(ui, form);
            if ui.is_modal(form) {
                c.fill_rect(RectF::new(0.0, 0.0, canvas.width, canvas.height), 0.0, &Brush::Solid(Color::BLACK.with_opacity(0.35)));
            }
            if bar > 0.0 {
                c.draw_shadow(frame, RADIUS, &Shadow::new(Color::BLACK.with_opacity(0.45), 28.0, 0.0, 10.0));
                c.fill_rect(frame, RADIUS, &Brush::Solid(theme.color("surface_2")));
            }
            // O form, abaixo da barra.
            c.save();
            c.translate(frame.x, frame.y + bar);
            c.clip_rect(RectF::new(0.0, 0.0, frame.width, frame.height - bar));
            ui.paint(form, c);
            c.restore();
            if bar == 0.0 {
                continue;
            }
            c.stroke_rect(frame, RADIUS, &Stroke::new(theme.color("border"), 1.0));
            let caption = ui.get_prop(form, "caption").ok().and_then(|v| v.to_str().ok().map(str::to_string)).unwrap_or_default();
            let mut style = TextStyle { color: theme.color("text"), v_align: TextAlign::Center, ..TextStyle::default() };
            style.font.weight = FontWeight::Bold;
            c.draw_text(&caption, RectF::new(frame.x + 14.0, frame.y, frame.width - TITLE - 24.0, TITLE), &style);
            let close = Self::close_rect(frame);
            if self.close_hover == Some(form) {
                c.fill_rect(close.deflate(aylis_core::Bounds::all(5.0)), 6.0, &Brush::Solid(theme.color("danger").with_opacity(0.85)));
            }
            let (cx, cy, s) = (close.x + close.width / 2.0, close.y + close.height / 2.0, 5.0);
            let x = Stroke::new(if self.close_hover == Some(form) { Color::WHITE } else { theme.color("text_muted") }, 1.6);
            c.draw_line(PointF::new(cx - s, cy - s), PointF::new(cx + s, cy + s), &x);
            c.draw_line(PointF::new(cx - s, cy + s), PointF::new(cx + s, cy - s), &x);
        }
    }

    /// O cursor: o do form sob o ponteiro; sobre uma barra de título (ou arrastando a
    /// janela), o comum.
    pub(crate) fn cursor(&self, ui: &Ui) -> CursorShape {
        match self.capture.or(self.hover) {
            Some(form) if self.drag.is_none() => ui.cursor(form),
            _ => CursorShape::Default,
        }
    }

    /// Entrega o ponteiro: arrastar ou fechar pela barra, ou ao form sob ele (com a posição
    /// dentro dele). Devolve se algo mudou na tela.
    pub(crate) fn pointer(&mut self, ui: &mut Ui, base: ComponentId, canvas: SizeF, event: Pointer) -> bool {
        let position = match &event {
            Pointer::Move(e) | Pointer::Down(e) | Pointer::Up(e) => Some(e.position),
            Pointer::Wheel(e) => Some(e.position),
            Pointer::Leave => None,
        };
        let Some(p) = position else {
            if let Some(old) = self.hover.take() {
                ui.pointer_leave(old);
            }
            let repaint = self.close_hover.take().is_some();
            return repaint;
        };
        // Arrastando uma janela pela barra.
        if let Some((form, grab)) = self.drag {
            match event {
                Pointer::Move(_) => {
                    self.pos.insert(form, PointF::new(p.x - grab.x, p.y - grab.y));
                    return true;
                }
                Pointer::Up(_) => {
                    self.drag = None;
                    return false;
                }
                _ => return false,
            }
        }
        // Quem está sob o ponteiro: a janela de cima que o contém, ou o principal.
        let mut target = (base, p);
        let mut title_of = None;
        if let Some(form) = self.capture.filter(|f| ui.open_forms().contains(f)) {
            let origin = if form == base { PointF::new(0.0, 0.0) } else { let f = self.frame(ui, form, canvas); PointF::new(f.x, f.y + Self::bar(ui, form)) };
            target = (form, PointF::new(p.x - origin.x, p.y - origin.y));
        } else {
            for form in Self::forms(ui).into_iter().rev() {
                let frame = self.frame(ui, form, canvas);
                if !frame.contains(p) {
                    continue;
                }
                let bar = Self::bar(ui, form);
                if p.y < frame.y + bar {
                    title_of = Some((form, frame));
                } else {
                    target = (form, PointF::new(p.x - frame.x, p.y - frame.y - bar));
                }
                break;
            }
        }
        let close_hover = title_of.filter(|(_, frame)| Self::close_rect(*frame).contains(p)).map(|(f, _)| f);
        let mut repaint = close_hover != self.close_hover;
        self.close_hover = close_hover;
        if let Some((form, frame)) = title_of {
            if let Some(old) = self.hover.take() {
                ui.pointer_leave(old);
            }
            // Sob um modal, as outras barras não respondem.
            let blocked = ui.input_blocked(form);
            if let Pointer::Down(e) = &event {
                if e.button == aylis_core::PointerButton::Primary && !blocked {
                    if Self::close_rect(frame).contains(p) {
                        if ui.is_modal(form) {
                            ui.set_modal_result(form, ModalResult::Cancel);
                        } else {
                            ui.close(form);
                        }
                    } else {
                        self.drag = Some((form, PointF::new(p.x - frame.x, p.y - frame.y)));
                    }
                    repaint = true;
                }
            }
            return repaint;
        }
        let (form, local) = target;
        if self.hover != Some(form) {
            if let Some(old) = self.hover.take() {
                ui.pointer_leave(old);
            }
            self.hover = Some(form);
        }
        match event {
            Pointer::Move(e) => ui.pointer_move(form, PointerEvent { position: local, ..e }),
            Pointer::Down(e) => {
                self.capture = Some(form);
                ui.pointer_down(form, PointerEvent { position: local, ..e });
            }
            Pointer::Up(e) => {
                self.capture = None;
                ui.pointer_up(form, PointerEvent { position: local, ..e });
            }
            Pointer::Wheel(e) => ui.wheel(form, WheelEvent { position: local, ..e }),
            Pointer::Leave => {}
        }
        repaint
    }
}
