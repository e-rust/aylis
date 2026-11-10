//! A forma dos controles em cada [`Look`] (o visual de cada plataforma). As cores vêm do
//! tema; aqui ficam só as diferenças de desenho: tamanho e jeito do Switch, CheckBox
//! redondo do iOS, controle deslizante do Windows 11, gradiente do Windows 7...

use aylis_core::{Brush, Canvas, Color, GradientStop, Look, PaintCtx, PointF, RectF, Shadow, Stroke};

fn shadow(canvas: &mut dyn Canvas, r: RectF, radius: f32, alpha: u8, blur: f32, dy: f32) {
    canvas.draw_shadow(r, radius, &Shadow::new(Color::rgba(0, 0, 0, alpha), blur, 0.0, dy));
}

// ----- Switch -----------------------------------------------------------------

/// Tamanho do trilho do Switch (largura, altura) no look.
pub fn switch_track(look: Look) -> (f32, f32) {
    match look {
        Look::Aylis => (36.0, 20.0),
        Look::Material3 => (44.0, 26.0),
        Look::Material2 => (36.0, 14.0),
        Look::Ios7 | Look::Ios13 => (44.0, 26.0),
        Look::Ios26 => (52.0, 24.0),
        Look::Win7 | Look::Win10 => (44.0, 20.0),
        Look::Win11 => (40.0, 20.0),
        Look::WinPhone => (56.0, 22.0),
    }
}

/// Desenha o Switch: `track` é o trilho, `t` vai de 0 (desligado) a 1 (ligado).
pub fn paint_switch(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, track: RectF, t: f32, checked: bool) {
    let s = ctx.style;
    let theme = ctx.theme;
    let r = track.height / 2.0;
    let lerp = |a: f32, b: f32| a + (b - a) * t;
    match ctx.look() {
        Look::Aylis => {
            canvas.fill_rect(track, r, &Brush::Solid(s.fill));
            if s.border_width > 0.0 {
                canvas.stroke_rect(track.inflate(2.0), r + 2.0, &Stroke::new(s.border_color, s.border_width));
            }
            let knob = track.height - 4.0;
            let x = track.x + 2.0 + (track.width - knob - 4.0) * t;
            canvas.fill_ellipse(RectF::new(x, track.y + 2.0, knob, knob), &Brush::Solid(Color::WHITE));
        }
        Look::Material3 => {
            canvas.fill_rect(track, r, &Brush::Solid(s.fill));
            if !checked {
                canvas.stroke_rect(track, r, &Stroke::new(theme.color("border"), 2.0));
            }
            // A bolinha cresce ao ligar (16 → 24, proporcional à altura).
            let k = track.height / 32.0;
            let d = lerp(16.0, 24.0) * k;
            let cx = lerp(track.x + r, track.right() - r);
            let color = if checked { theme.color("on_primary") } else { theme.color("border") };
            canvas.fill_ellipse(RectF::new(cx - d / 2.0, track.y + (track.height - d) / 2.0, d, d), &Brush::Solid(color));
        }
        Look::Material2 => {
            let fill = if checked { theme.color("primary").with_opacity(0.5) } else { s.fill.with_opacity(0.8) };
            canvas.fill_rect(track, r, &Brush::Solid(fill));
            let d = track.height * 1.45;
            let cx = lerp(track.x + d / 2.0 - 2.0, track.right() - d / 2.0 + 2.0);
            let knob = RectF::new(cx - d / 2.0, track.y + (track.height - d) / 2.0, d, d);
            shadow(canvas, knob, d / 2.0, 70, 3.0, 1.0);
            let color = if checked { theme.color("primary") } else if theme.dark { Color::rgb(0xBD, 0xBD, 0xBD) } else { Color::rgb(0xFA, 0xFA, 0xFA) };
            canvas.fill_ellipse(knob, &Brush::Solid(color));
        }
        Look::Ios7 | Look::Ios13 => {
            let on = theme.color("success");
            if ctx.look() == Look::Ios7 {
                canvas.fill_rect(track, r, &Brush::Solid(theme.color("surface")));
                canvas.stroke_rect(track, r, &Stroke::new(theme.color("track"), 1.5));
                if t > 0.0 {
                    canvas.fill_rect(track, r, &Brush::Solid(on.with_opacity(t)));
                }
            } else {
                canvas.fill_rect(track, r, &Brush::Solid(s.fill.lerp(on, t)));
            }
            let d = track.height - 4.0;
            let x = track.x + 2.0 + (track.width - d - 4.0) * t;
            let knob = RectF::new(x, track.y + 2.0, d, d);
            shadow(canvas, knob, d / 2.0, 60, 4.0, 2.0);
            canvas.fill_ellipse(knob, &Brush::Solid(Color::WHITE));
        }
        Look::Ios26 => {
            canvas.fill_rect(track, r, &Brush::Solid(theme.color("track").lerp(theme.color("success"), t)));
            // A bolinha é uma cápsula de vidro.
            let (w, h) = (track.height * 1.5, track.height - 4.0);
            let x = track.x + 2.0 + (track.width - w - 4.0) * t;
            let knob = RectF::new(x, track.y + 2.0, w, h);
            shadow(canvas, knob, h / 2.0, 55, 5.0, 2.0);
            canvas.fill_rect(knob, h / 2.0, &Brush::Solid(Color::WHITE));
            canvas.stroke_rect(knob, h / 2.0, &Stroke::new(Color::rgba(0, 0, 0, 18), 1.0));
        }
        Look::Win7 | Look::Win10 | Look::Win11 => {
            let win11 = ctx.look() == Look::Win11;
            if checked {
                canvas.fill_rect(track, r, &Brush::Solid(theme.color("primary")));
            } else {
                let width = if win11 { 1.0 } else { 2.0 };
                canvas.stroke_rect(track, r, &Stroke::new(s.border_color, width));
            }
            let d = if win11 { if ctx.state.hover { 14.0 } else { 12.0 } } else { 10.0 } * track.height / 20.0;
            let pad = (track.height - d) / 2.0;
            let x = track.x + pad + (track.width - d - 2.0 * pad) * t;
            let color = if checked { theme.color("on_primary") } else if win11 { theme.color("text_muted") } else { theme.color("text") };
            canvas.fill_ellipse(RectF::new(x, track.y + pad, d, d), &Brush::Solid(color));
        }
        Look::WinPhone => {
            // Moldura, o miolo na cor de destaque (com um vão) e o "dedo" retangular, mais
            // alto que o trilho.
            let fg = theme.color("text");
            canvas.stroke_rect(track.inflate(-1.5), 0.0, &Stroke::new(fg, 3.0));
            let inner = track.inflate(-5.0);
            if t > 0.0 {
                canvas.fill_rect(RectF::new(inner.x, inner.y, inner.width * t, inner.height), 0.0, &Brush::Solid(theme.color("primary")));
            }
            let w = 12.0;
            let x = track.x + (track.width - w) * t;
            canvas.fill_rect(RectF::new(x, track.y - 3.0, w, track.height + 6.0), 0.0, &Brush::Solid(fg));
        }
    }
}

// ----- CheckBox e RadioButton --------------------------------------------------

/// Lado da marca do CheckBox / RadioButton no look.
pub fn mark_size(look: Look) -> f32 {
    match look {
        Look::Aylis | Look::Material2 | Look::Material3 => 18.0,
        Look::Ios7 | Look::Ios13 | Look::Ios26 => 22.0,
        Look::Win7 => 13.0,
        Look::Win10 | Look::Win11 => 20.0,
        Look::WinPhone => 26.0,
    }
}

fn check_mark(canvas: &mut dyn Canvas, b: RectF, color: Color, width: f32) {
    let k = b.width / 18.0;
    let p = |x: f32, y: f32| PointF::new(b.x + x * k, b.y + y * k);
    let mut path = aylis_core::Path::new();
    let (a, m, c) = (p(4.5, 9.5), p(7.5, 12.5), p(13.5, 5.5));
    path.move_to(a.x, a.y).line_to(m.x, m.y).line_to(c.x, c.y);
    canvas.stroke_path(&path, &Stroke::new(color, width).round());
}

/// A caixa do CheckBox.
pub fn paint_check(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, b: RectF, checked: bool) {
    let s = ctx.style;
    let look = ctx.look();
    if look == Look::Win7 {
        // Caixa com gradiente e o "visto" escuro.
        let brush = Brush::Linear {
            start: PointF::new(b.x, b.y),
            end: PointF::new(b.right(), b.bottom()),
            stops: vec![GradientStop::new(0.0, Color::rgb(0xDB, 0xDB, 0xDB)), GradientStop::new(1.0, Color::rgb(0xFC, 0xFC, 0xFC))],
        };
        canvas.fill_rect(b, 0.0, &brush);
        canvas.stroke_rect(b, 0.0, &Stroke::new(s.border_color, 1.0));
        if checked {
            check_mark(canvas, b.inflate(-1.0), Color::rgb(0x4A, 0x5F, 0x97), 2.0);
        }
        return;
    }
    let radius = s.radius.min(b.width / 2.0);
    canvas.fill_rect(b, radius, &Brush::Solid(s.fill));
    if s.border_width > 0.0 && !(look.is_ios() && checked) {
        canvas.stroke_rect(b, radius, &Stroke::new(s.border_color, s.border_width));
    }
    if checked {
        // No Windows Phone o "visto" é preto sobre a caixa cinza; nos outros, sobre o destaque.
        let (color, width) = match look {
            Look::WinPhone => (ctx.theme.color("field_text"), 3.0),
            _ if look.is_ios() => (ctx.theme.color("on_primary"), 2.2),
            // Tema com a caixa marcada clara (o Win32): o `check_mark` dele.
            _ => (ctx.theme.colors.get("check_mark").copied().unwrap_or_else(|| ctx.theme.color("on_primary")), 2.0),
        };
        check_mark(canvas, b.inflate(-b.width * 0.08), color, width);
    }
}

/// O círculo do RadioButton.
pub fn paint_radio(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, b: RectF, checked: bool) {
    let s = ctx.style;
    let look = ctx.look();
    if look.is_ios() {
        // O iOS não tem rádio: a marca redonda das listas, como o CheckBox.
        return paint_check(canvas, ctx, b, checked);
    }
    if look.is_material() {
        // Anel e ponto na cor de destaque, sem preencher.
        let color = if checked { ctx.theme.color("primary") } else { s.border_color };
        canvas.stroke_ellipse(b, &Stroke::new(color, 2.0));
        if checked {
            canvas.fill_ellipse(b.inflate(-b.width * 0.25), &Brush::Solid(color));
        }
        return;
    }
    if look == Look::Win7 {
        let brush = Brush::Linear {
            start: PointF::new(b.x, b.y),
            end: PointF::new(b.right(), b.bottom()),
            stops: vec![GradientStop::new(0.0, Color::rgb(0xDB, 0xDB, 0xDB)), GradientStop::new(1.0, Color::rgb(0xFC, 0xFC, 0xFC))],
        };
        canvas.fill_ellipse(b, &brush);
        canvas.stroke_ellipse(b, &Stroke::new(s.border_color, 1.0));
        if checked {
            canvas.fill_ellipse(b.inflate(-3.5), &Brush::Solid(Color::rgb(0x2E, 0x7B, 0xD5)));
        }
        return;
    }
    canvas.fill_ellipse(b, &Brush::Solid(s.fill));
    if s.border_width > 0.0 {
        canvas.stroke_ellipse(b, &Stroke::new(s.border_color, s.border_width));
    }
    if checked {
        let inner = if look == Look::Win11 && ctx.state.hover { 0.32 } else { 0.28 };
        let dot = if look == Look::WinPhone { ctx.theme.color("field_text") } else { ctx.theme.colors.get("check_mark").copied().unwrap_or_else(|| ctx.theme.color("on_primary")) };
        canvas.fill_ellipse(b.inflate(-b.width * inner), &Brush::Solid(dot));
    }
}

// ----- TrackBar ---------------------------------------------------------------

/// Espessura do trilho e lado do controle (o que se arrasta) no look.
pub fn slider_metrics(look: Look) -> (f32, f32) {
    match look {
        Look::Aylis => (4.0, 18.0),
        // Material 3 Expressive: trilho grosso e o controle é uma barra vertical.
        Look::Material3 => (16.0, 44.0),
        Look::Material2 => (2.0, 12.0),
        Look::Ios7 | Look::Ios13 | Look::Ios26 => (4.0, 24.0),
        Look::Win7 => (4.0, 18.0),
        Look::Win10 => (2.0, 22.0),
        Look::Win11 => (4.0, 20.0),
        Look::WinPhone => (12.0, 24.0),
    }
}

/// O controle do TrackBar em `c` (centro).
pub fn paint_thumb(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, c: PointF, side: f32, vertical: bool) {
    let theme = ctx.theme;
    let primary = theme.color("primary");
    let knob = RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side);
    match ctx.look() {
        Look::Aylis => {
            canvas.fill_ellipse(knob.offset(0.0, 1.0), &Brush::Solid(Color::BLACK.with_opacity(0.15)));
            canvas.fill_ellipse(knob, &Brush::Solid(theme.color("surface")));
            let ring = if ctx.state.focused { theme.color("focus_ring") } else { primary };
            canvas.stroke_ellipse(knob, &Stroke::new(ring, if ctx.state.focused { 2.5 } else { 2.0 }));
        }
        Look::Material3 => {
            // O vão em volta da barra (cor do fundo) e a barra de 4 × 44.
            let gap = if vertical { RectF::new(c.x - 12.0, c.y - 6.0, 24.0, 12.0) } else { RectF::new(c.x - 6.0, c.y - 12.0, 12.0, 24.0) };
            canvas.fill_rect(gap, 0.0, &Brush::Solid(theme.color("background")));
            let w = if ctx.state.pressed { 2.0 } else { 4.0 };
            let bar = if vertical { RectF::new(c.x - side / 2.0, c.y - w / 2.0, side, w) } else { RectF::new(c.x - w / 2.0, c.y - side / 2.0, w, side) };
            canvas.fill_rect(bar, w / 2.0, &Brush::Solid(primary));
        }
        Look::Material2 => {
            canvas.fill_ellipse(knob, &Brush::Solid(primary));
            if ctx.state.hover || ctx.state.focused {
                canvas.fill_ellipse(knob.inflate(side * 0.6), &Brush::Solid(primary.with_opacity(0.12)));
            }
        }
        Look::Ios7 | Look::Ios13 | Look::Ios26 => {
            shadow(canvas, knob, side / 2.0, 70, 5.0, 2.0);
            canvas.fill_ellipse(knob, &Brush::Solid(Color::WHITE));
        }
        Look::Win7 => {
            let r = if vertical { RectF::new(c.x - 9.0, c.y - 5.5, 18.0, 11.0) } else { RectF::new(c.x - 5.5, c.y - 9.0, 11.0, 18.0) };
            let brush = Brush::Linear {
                start: PointF::new(r.x, r.y),
                end: PointF::new(r.right(), r.y),
                stops: vec![GradientStop::new(0.0, Color::rgb(0xF2, 0xF2, 0xF2)), GradientStop::new(1.0, Color::rgb(0xCF, 0xCF, 0xCF))],
            };
            canvas.fill_rect(r, 2.0, &brush);
            canvas.stroke_rect(r, 2.0, &Stroke::new(Color::rgb(0x70, 0x70, 0x70), 1.0));
        }
        Look::Win10 => {
            let r = if vertical { RectF::new(c.x - side / 2.0, c.y - 4.0, side, 8.0) } else { RectF::new(c.x - 4.0, c.y - side / 2.0, 8.0, side) };
            canvas.fill_rect(r, 4.0, &Brush::Solid(primary));
        }
        Look::Win11 => {
            canvas.fill_ellipse(knob, &Brush::Solid(theme.color("surface")));
            canvas.stroke_ellipse(knob, &Stroke::new(theme.color("control_stroke"), 1.0));
            let inner = if ctx.state.hover { 0.6 } else { 0.5 };
            let d = side * inner;
            canvas.fill_ellipse(RectF::new(c.x - d / 2.0, c.y - d / 2.0, d, d), &Brush::Solid(primary));
        }
        Look::WinPhone => {
            // Um traço fino na cor do texto, atravessando o trilho grosso.
            let r = if vertical { RectF::new(c.x - side / 2.0, c.y - 3.0, side, 6.0) } else { RectF::new(c.x - 3.0, c.y - side / 2.0, 6.0, side) };
            canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("text")));
        }
    }
}

// ----- Button e campos ----------------------------------------------------------

/// Fundo do botão: o gradiente do Windows 7 ou o traço de baixo do Windows 11; nos outros,
/// a cor do tema. `variant` vazio é o botão padrão.
///
/// Com o anel de foco, o botão encolhe e deixa uma folga entre ele e o anel, onde aparece o
/// fundo de trás: o anel contrasta mesmo quando tem a cor do próprio botão (o primário).
pub fn paint_button_fill(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, variant: &str) {
    let s = ctx.style;
    let border = s.border_width > 0.0 && !s.border_color.is_transparent();
    // O anel fica onde a borda ficaria (no contorno do controle); o botão, para dentro dele
    // a metade do traço mais a folga, da largura do traço.
    let inset = if border && ctx.state.focused { s.border_width * 1.5 } else { 0.0 };
    let r = ctx.rect.inflate(-inset);
    let radius = (s.radius - inset).max(0.0);
    if ctx.look() == Look::Win7 && matches!(variant, "" | "secondary") {
        let (top, bottom) = if ctx.state.pressed {
            ((0xE5F4FC, 0xC4E5F6), (0x98D1EF, 0x68B3DB))
        } else if ctx.state.hover {
            ((0xEAF6FD, 0xD9F0FC), (0xBEE6FD, 0xA7D9F5))
        } else {
            ((0xF2F2F2, 0xEBEBEB), (0xDDDDDD, 0xCFCFCF))
        };
        let c = |v: u32| Color::rgb((v >> 16) as u8, (v >> 8) as u8, v as u8);
        let brush = Brush::Linear {
            start: PointF::new(r.x, r.y),
            end: PointF::new(r.x, r.bottom()),
            stops: vec![
                GradientStop::new(0.0, c(top.0)),
                GradientStop::new(0.49, c(top.1)),
                GradientStop::new(0.5, c(bottom.0)),
                GradientStop::new(1.0, c(bottom.1)),
            ],
        };
        canvas.fill_rect(r, radius, &brush);
        canvas.stroke_rect(r.inflate(-1.0), (radius - 1.0).max(0.0), &Stroke::new(Color::rgba(255, 255, 255, 170), 1.0));
    } else {
        canvas.fill_rect(r, radius, &Brush::Solid(s.fill));
    }
    if border {
        canvas.stroke_rect(ctx.rect, s.radius, &Stroke::new(s.border_color, s.border_width));
    }
    if ctx.look() == Look::Win11 && variant.is_empty() && !ctx.state.pressed {
        // O traço de baixo, um pouco mais escuro (dá a "elevação" dos botões do Win11).
        canvas.save();
        canvas.clip_round_rect(r, radius);
        canvas.fill_rect(RectF::new(r.x, r.bottom() - 1.0, r.width, 1.0), 0.0, &Brush::Solid(ctx.theme.color("control_bottom")));
        canvas.restore();
    }
}

/// Texto do botão no look (o Material 2 escreve em maiúsculas).
pub fn button_text(look: Look, text: &str) -> std::borrow::Cow<'_, str> {
    if look == Look::Material2 { text.to_uppercase().into() } else { text.into() }
}

/// A linha de baixo dos campos (Material 2 e Windows 11), mais grossa e colorida com o foco.
pub fn paint_field_underline(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
    paint_field_underline_in(canvas, ctx, ctx.rect);
}

/// Como [`paint_field_underline`], na caixa `r` (o campo sem o texto de ajuda embaixo).
pub fn paint_field_underline_in(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, r: RectF) {
    let theme = ctx.theme;
    let (color, width) = match (ctx.look(), ctx.state.focused) {
        (Look::Material2, true) => (theme.color("primary"), 2.0),
        (Look::Material2, false) => (theme.color("text_muted"), 1.0),
        (Look::Win11, true) => (theme.color("primary"), 2.0),
        (Look::Win11, false) => (theme.color("field_bottom"), 1.0),
        // O campo "filled" do Material 3 também tem a linha embaixo.
        (Look::Material3, focused) if ctx.variant() == "filled" => {
            if focused { (theme.color("primary"), 2.0) } else { (theme.color("text_muted"), 1.0) }
        }
        _ => return,
    };
    canvas.save();
    canvas.clip_round_rect(r, ctx.style.radius);
    canvas.fill_rect(RectF::new(r.x, r.bottom() - width, r.width, width), 0.0, &Brush::Solid(color));
    canvas.restore();
}

/// O brilho da barra de progresso do Windows 7.
pub fn paint_progress_gloss(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, bar: RectF) {
    if ctx.look() != Look::Win7 || bar.width <= 0.0 {
        return;
    }
    let brush = Brush::Linear {
        start: PointF::new(bar.x, bar.y),
        end: PointF::new(bar.x, bar.bottom()),
        stops: vec![
            GradientStop::new(0.0, Color::rgba(255, 255, 255, 150)),
            GradientStop::new(0.45, Color::rgba(255, 255, 255, 40)),
            GradientStop::new(0.55, Color::rgba(0, 0, 0, 0)),
            GradientStop::new(1.0, Color::rgba(0, 0, 0, 25)),
        ],
    };
    canvas.fill_rect(bar, ctx.style.radius.min(bar.height / 2.0), &brush);
}
