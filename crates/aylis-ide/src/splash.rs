//! A abertura da IDE: o lingote do logo em 3D (o `Model3D`, com luz de verdade), girando em
//! volta do próprio eixo enquanto a IDE abre. Segurar o botão do mouse sobre ele prende a
//! abertura e o arraste para os lados gira o lingote; ao soltar, ele volta a girar sozinho e a
//! IDE abre logo depois.
//!
//! O lingote (o modelo, a câmera, o giro e o brilho) é o `aylis_3d::ingot`, o mesmo dos
//! exemplos e do teste de celular; aqui ficam o fundo, o texto e o segurar com o mouse.

use std::cell::Cell;
use std::rc::Rc;

use aylis_3d::Model3D;
use aylis_controls::Form;
use aylis_core::{
    Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, EventCtx, FontSpec, FontWeight, GradientStop, PaintCtx, PointF,
    PointerEvent, RectF, Stroke, TextAlign, TextStyle, Ui,
};

/// Tempo mínimo na tela, em milissegundos.
const MIN_MS: u64 = 3500;
/// Depois de soltar o lingote, quanto falta para a IDE abrir (segundos).
const AFTER_RELEASE: f64 = 0.5;
/// Graus por ponto arrastado.
const DRAG: f32 = 0.6;
/// A cor do fundo do logo, em cima e embaixo.
const BACKGROUND: (Color, Color) = (Color::rgb(22, 24, 33), Color::rgb(16, 18, 27));

/// O que o lingote e o temporizador da abertura dividem.
#[derive(Default)]
struct Hold {
    held: Cell<bool>,
    /// Quando foi solto pela última vez (em [`Ui::time`]); `None` se nunca foi segurado.
    released_at: Cell<Option<f64>>,
    /// O `Model3D` do lingote.
    model: Cell<Option<ComponentId>>,
}

/// A chave da configuração da IDE que liga a tela de abertura (Ferramentas → Opções da IDE).
const ENABLED_KEY: &str = "show_splash";

/// A tela de abertura aparece ao iniciar a IDE (ligada, se ninguém desligou).
pub fn enabled() -> bool {
    crate::actions::config_flag(ENABLED_KEY, true)
}

/// Liga ou desliga a tela de abertura nas próximas vezes.
pub fn set_enabled(on: bool) {
    crate::actions::update_config(ENABLED_KEY, serde_json::Value::from(on));
}

/// Abre a tela de abertura e, passado o tempo mínimo (e solto o lingote), troca para
/// `main_form`, que vira o form principal.
pub fn show(ui: &mut Ui, main_form: ComponentId) -> Result<ComponentId, aylis_core::Error> {
    let form = ui.add_root(Form::new("aylis", 600.0, 340.0));
    let _ = ui.set_name(form, "frmSplash");
    let _ = ui.set_prop(form, "border_style", "none");
    let _ = ui.set_prop(form, "position", "screen_center");
    let _ = ui.set_prop(form, "topmost", true);
    let hold = Rc::new(Hold::default());
    let splash = ui.add(Splash { base: ControlBase::new(600.0, 340.0), hold: hold.clone(), drag: None }, form);
    ui.control_mut(splash).unwrap().set_align(Align::Client);

    // O lingote à esquerda; o pé some no fundo no próprio modelo (`aylis_3d::ingot`).
    let mut model = Model3D::default();
    aylis_3d::ingot::apply(&mut model);
    let model = ui.add(model, splash);
    ui.control_mut(model).unwrap().set_left(-10.0).set_top(-10.0).set_width(360.0).set_height(470.0).set_hit_test(false);
    hold.model.set(Some(model));

    let front = ui.add(Front { base: ControlBase::new(600.0, 340.0) }, splash);
    ui.control_mut(front).unwrap().set_align(Align::Client).set_hit_test(false);

    ui.show(form)?;
    ui.set_timeout(MIN_MS, move |ui| finish_when_released(ui, form, main_form, hold));
    Ok(form)
}

/// Troca para o form principal se o lingote não está seguro; senão, confere de novo daqui a
/// pouco.
fn finish_when_released(ui: &mut Ui, splash: ComponentId, main_form: ComponentId, hold: Rc<Hold>) {
    let waiting = hold.held.get() || hold.released_at.get().is_some_and(|t| ui.time() - t < AFTER_RELEASE);
    if waiting && ui.is_open(splash) {
        ui.set_timeout(60, move |ui| finish_when_released(ui, splash, main_form, hold));
        return;
    }
    // A abertura fecha antes de a IDE abrir: no Windows, a janela aberta com outra na frente
    // fica "possuída" por ela e sumiria junto.
    ui.set_main_form(main_form);
    ui.close(splash);
    let _ = ui.show(main_form);
}

// ---------------------------------------------------------------------------
// A tela

/// O fundo e o texto da abertura; recebe o mouse no lugar do `Model3D` (que só desenha).
struct Splash {
    base: ControlBase,
    hold: Rc<Hold>,
    /// O x do ponteiro no último movimento, enquanto arrasta.
    drag: Option<f32>,
}

impl Splash {
    fn model<'a>(&self, ui: &'a mut Ui) -> Option<&'a mut Model3D> {
        ui.get_mut::<Model3D>(self.hold.model.get()?)
    }
}

impl Component for Splash {
    fn class_name(&self) -> &'static str {
        "IdeSplash"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.drag = Some(event.position.x);
        self.hold.held.set(true);
        if let Some(m) = self.model(ctx.ui) {
            m.auto_rotate = 0.0;
        }
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some(last) = self.drag else { return };
        // Só gira em volta do eixo comprido: a câmera continua onde está. O `model_rotation`
        // só muda a pintura (não refaz o layout da IDE por trás a cada movimento).
        if let Some(model) = self.hold.model.get() {
            let now = ctx.ui.get::<Model3D>(model).map_or(0.0, |m| m.model_rotation);
            let _ = ctx.ui.set_prop(model, "model_rotation", now + (event.position.x - last) * DRAG);
        }
        self.drag = Some(event.position.x);
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.drag = None;
        self.hold.held.set(false);
        self.hold.released_at.set(Some(ctx.ui.time()));
        if let Some(m) = self.model(ctx.ui) {
            m.auto_rotate = aylis_3d::ingot::SPIN;
        }
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(
            r,
            0.0,
            &Brush::Linear {
                start: PointF::new(r.x, r.y),
                end: PointF::new(r.x, r.bottom()),
                stops: vec![GradientStop::new(0.0, BACKGROUND.0), GradientStop::new(1.0, BACKGROUND.1)],
            },
        );

        let x = r.x + r.width * 0.53;
        let title = TextStyle {
            font: FontSpec { size: 64.0, weight: FontWeight::Bold, ..FontSpec::default() },
            color: Color::WHITE,
            h_align: TextAlign::Leading,
            v_align: TextAlign::Trailing,
            ..Default::default()
        };
        canvas.draw_text("AYLIS", RectF::new(x, r.y, r.right() - x, r.height * 0.52), &title);
        let subtitle = TextStyle {
            font: FontSpec { size: 24.0, ..FontSpec::default() },
            color: Color::rgb(176, 181, 191),
            h_align: TextAlign::Leading,
            v_align: TextAlign::Leading,
            ..Default::default()
        };
        canvas.draw_text("GUI Framework", RectF::new(x + 3.0, r.y + r.height * 0.54, r.right() - x, 40.0), &subtitle);
    }
}

/// Por cima do lingote: a borda da janela e a barra de
/// "carregando".
struct Front {
    base: ControlBase,
}

impl Component for Front {
    fn class_name(&self) -> &'static str {
        "IdeSplashFront"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // A barra anda mesmo com o lingote seguro (e parado).
        ctx.request_frame();

        canvas.stroke_rect(r.inflate(-0.5), 0.0, &Stroke::new(Color::rgba(255, 255, 255, 22), 1.0));

        // Carregando: um trecho em brasa correndo no pé da janela.
        let track = RectF::new(r.x + 1.0, r.bottom() - 3.0, r.width - 2.0, 2.0);
        let t = (ctx.time * 0.8).fract() as f32;
        let w = track.width * 0.28;
        let left = track.x - w + (track.width + w) * t;
        canvas.save();
        canvas.clip_rect(track);
        canvas.fill_rect(
            RectF::new(left, track.y, w, track.height),
            0.0,
            &Brush::Linear {
                start: PointF::new(left, track.y),
                end: PointF::new(left + w, track.y),
                stops: vec![
                    GradientStop::new(0.0, Color::rgba(255, 140, 40, 0)),
                    GradientStop::new(0.7, Color::rgb(255, 150, 40)),
                    GradientStop::new(1.0, Color::rgb(255, 238, 190)),
                ],
            },
        );
        canvas.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O lingote como o loading desenha (a câmera, as luzes e o brilho dele), girado `turn`
    /// graus a partir da posição do logo, num PNG sobre o fundo da abertura. Para comparar com
    /// o logo sem abrir a janela: `AYLIS_SPLASH_PNG=saida.png cargo test -p aylis-ide --lib
    /// splash -- --ignored` (`AYLIS_SPLASH_TURN` gira).
    #[test]
    #[ignore]
    fn renders_the_ingot_to_png() {
        use glam::{Mat4, Quat, Vec3};
        use aylis_3d::ingot::{FOV, GLOW};
        use aylis_3d::{Engine, Frame, Output};
        let Some(out) = std::env::var_os("AYLIS_SPLASH_PNG") else { return };
        let turn: f32 = std::env::var("AYLIS_SPLASH_TURN").ok().and_then(|t| t.parse().ok()).unwrap_or(0.0);
        let Ok(mut engine) = Engine::new(None) else { return };
        let model = aylis_3d::ingot::model();
        let (w, h) = (540u32, 705u32);
        let pose = model.rest_pose();
        // Como o Model3D com `turntable`: o modelo gira, a câmera e as luzes não.
        let spin = Mat4::from_quat(Quat::from_rotation_y((45.0 + turn).to_radians()));
        let still = model.world(&pose, Mat4::IDENTITY);
        let mut world = model.world(&pose, spin);
        for node in [1, 2, 3, 4] {
            world[node] = still[node];
        }
        let (_, rotation, eye) = still[4].to_scale_rotation_translation();
        let aspect = w as f32 / h as f32;
        let frame = Frame {
            width: w,
            height: h,
            view: Mat4::from_rotation_translation(rotation, eye).inverse(),
            proj: glam::camera::rh::proj::directx::perspective(FOV.to_radians(), aspect, 0.5, 40.0),
            eye,
            background: [0.0; 4],
            light_dir: Vec3::NEG_Y,
            file_lights: true,
            shadows: false,
            ray_traced: false,
            sample: 0,
            glow: GLOW,
        };
        let mut gpu = engine.upload(&model);
        let mut view = engine.view();
        let Some(Output::Pixels { rgba, .. }) = engine.render(&model, &mut gpu, &mut view, &world, &pose.weights, &frame, false) else { panic!("sem pixels") };
        // Sobre o fundo da abertura (os pixels vêm com alfa não pré-multiplicado; o pé já some
        // no próprio modelo).
        let bg = BACKGROUND.0;
        let mut img = image::RgbaImage::new(w, h);
        for (i, px) in img.pixels_mut().enumerate() {
            let p = &rgba[i * 4..i * 4 + 4];
            let a = f32::from(p[3]) / 255.0;
            let mix = |c: u8, b: u8| (f32::from(c) * a + f32::from(b) * (1.0 - a)).round() as u8;
            *px = image::Rgba([mix(p[0], bg.r), mix(p[1], bg.g), mix(p[2], bg.b), 255]);
        }
        img.save(&out).unwrap();
    }
}
