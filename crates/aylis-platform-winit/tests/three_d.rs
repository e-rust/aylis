//! O 3D (wgpu) desenhado no mesmo contexto OpenGL do Skia, sem cópia: o modelo aparece no
//! quadro do Skia, com o controle de cima por cima dele.

use aylis_3d::{Model, Model3D, sample};
use aylis_controls::{Form, Rectangle};
use aylis_core::{Align, Color, Ui};

fn main() {
    let result = aylis_platform_winit::with_offscreen_gl(|gl| {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        aylis_3d::register(&mut ui);
        let form = ui.add_root(Form::new("3d", 320.0, 240.0));
        let view = ui.add(Model3D::default().with_model(Model::from_glb(&sample::tube_glb()).unwrap()), form);
        ui.control_mut(view).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(view, "background", Color::rgb(255, 255, 255));
        // Um controle por cima do 3D, no canto.
        let top = ui.add(Rectangle::new(40.0, 40.0), form);
        ui.control_mut(top).unwrap().set_rect(0.0, 0.0, 40.0, 40.0);
        let _ = ui.set_prop(top, "fill", Color::rgb(0, 0, 255));
        ui.layout();
        let renderer = aylis_skia::SkiaRenderer::new();
        // Dois quadros: o primeiro abre o dispositivo do 3D no contexto do Skia.
        let mut frame = None;
        for _ in 0..2 {
            frame = gl.render_rgba(&renderer, 320, 240, 1.0, Color::WHITE, |c| ui.paint(form, c));
        }
        frame
    });
    let rgba = result.expect("contexto OpenGL").expect("quadro");
    let at = |x: usize, y: usize| {
        let i = (y * 320 + x) * 4;
        [rgba[i], rgba[i + 1], rgba[i + 2]]
    };
    let orange = (0..320).flat_map(|x| (0..240).map(move |y| (x, y))).filter(|&(x, y)| {
        let p = at(x, y);
        let [r, g, b] = p.map(i32::from);
        r > g + 30 && g > b + 20
    });
    let n = orange.count();
    assert!(n > 300, "o tubo aparece no quadro do Skia ({n} pixels)");
    assert_eq!(at(10, 10), [0, 0, 255], "o controle fica por cima do 3D");
    println!("tres_d: ok ({n} pixels do tubo)");
}
