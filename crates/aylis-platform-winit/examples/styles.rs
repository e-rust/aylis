//! Os temas de plataforma lado a lado: os mesmos controles com o visual do aylis, do
//! Android (7, Material 2 e 3), do iOS (7, 13 e 26), do Windows (7, 10, 11, WinUI 3 e Win32) e
//! do Windows Phone. Cada painel
//! só troca a propriedade `theme` (que vale para os filhos).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example styles
//! ```

use aylis_controls::*;
use aylis_core::*;

const THEMES: [&str; 23] = [
    "Aylis Light",
    "Material 3 Light",
    "Material 2 Light",
    "Android 7 Light",
    "iOS 26 Light",
    "iOS 13 Light",
    "iOS 7 Light",
    "WinUI 3 Light",
    "Aylis Dark",
    "Material 3 Dark",
    "Material 2 Dark",
    "Android 7 Dark",
    "iOS 26 Dark",
    "iOS 13 Dark",
    "Windows 11 Light",
    "WinUI 3 Dark",
    "Windows 11 Dark",
    "Windows 10 Light",
    "Windows 10 Dark",
    "Win32",
    "Windows 7",
    "Windows Phone Dark",
    "Windows Phone Light",
];

const W: f32 = 236.0;
const H: f32 = 330.0;

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn sample(ui: &mut Ui, form: ComponentId, theme: &str, x: f32, y: f32) {
    let bg = ui.theme_named(theme).map_or(Color::WHITE, |t| t.color("background"));
    let panel = ui.add(Rectangle::default(), form);
    put(ui, panel, x, y, W, H);
    ui.set_prop(panel, "fill", bg).unwrap();
    ui.set_prop(panel, "radius", 10).unwrap();
    ui.set_prop(panel, "theme", theme).unwrap();
    let title = ui.add(Label::new(theme), panel);
    put(ui, title, 12.0, 8.0, W - 24.0, 22.0);
    ui.set_prop(title, "style", "heading").unwrap();
    let b = ui.add(Button::new("Salvar"), panel);
    put(ui, b, 12.0, 38.0, 100.0, 34.0);
    let b2 = ui.add(Button::new("Cancelar"), panel);
    put(ui, b2, 120.0, 38.0, 104.0, 34.0);
    ui.set_prop(b2, "style", "secondary").unwrap();
    let e = ui.add(Edit::new("Texto"), panel);
    put(ui, e, 12.0, 82.0, W - 24.0, 34.0);
    let c = ui.add(CheckBox::new("Lembrar"), panel);
    put(ui, c, 12.0, 126.0, 104.0, 26.0);
    ui.set_prop(c, "checked", true).unwrap();
    let c2 = ui.add(CheckBox::new("Não"), panel);
    put(ui, c2, 124.0, 126.0, 100.0, 26.0);
    let r = ui.add(RadioButton::new("Sim"), panel);
    put(ui, r, 12.0, 158.0, 104.0, 26.0);
    ui.set_prop(r, "checked", true).unwrap();
    let r2 = ui.add(RadioButton::new("Talvez"), panel);
    put(ui, r2, 124.0, 158.0, 100.0, 26.0);
    let s = ui.add(Switch::new("Wi-Fi"), panel);
    put(ui, s, 12.0, 192.0, 110.0, 28.0);
    ui.set_prop(s, "checked", true).unwrap();
    let s2 = ui.add(Switch::new("Off"), panel);
    put(ui, s2, 124.0, 192.0, 104.0, 28.0);
    let t = ui.add(TrackBar::new(60.0, 0.0, 100.0), panel);
    put(ui, t, 8.0, 228.0, W - 16.0, 30.0);
    let p = ui.add(ProgressBar::default(), panel);
    put(ui, p, 12.0, 268.0, W - 24.0, 6.0);
    ui.set_prop(p, "value", 45.0).unwrap();
    let seg = ui.add(SegmentedControl::new("Dia;Semana;Mês"), panel);
    put(ui, seg, 12.0, 286.0, W - 24.0, 32.0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let cols = 8;
    let gap = 10.0;
    let width = cols as f32 * (W + gap) + gap;
    let rows = THEMES.len().div_ceil(cols);
    let height = rows as f32 * (H + gap) + gap;
    let form = ui.add_root(Form::new("aylis — estilos de plataforma", width, height));
    ui.set_name(form, "frmEstilos")?;
    ui.set_prop(form, "fill", Color::rgb(0x88, 0x8E, 0x99)).ok();
    for (i, theme) in THEMES.iter().enumerate() {
        let (col, row) = (i % cols, i / cols);
        sample(&mut ui, form, theme, gap + col as f32 * (W + gap), gap + row as f32 * (H + gap));
    }
    aylis_platform_winit::run(ui, form)
}
