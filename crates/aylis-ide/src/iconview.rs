//! Projeto → Opções do projeto → Ícone do app: escolhe a imagem (PNG de 256 px ou mais, ou SVG) e mostra como
//! ela fica em cada lugar (barra de tarefas e Explorer do Windows, o adaptativo do Android
//! com as máscaras redonda e arredondada, o iOS, a aba do navegador e o PWA) antes de
//! publicar. A geração de verdade é no publicar (`aylis_tools::icons`).

use aylis_core::{Brush, Canvas, Color, Component, ControlBase, Image, PaintCtx, RectF, TextAlign, Ui};
use aylis_tools::icons::{self, Icon};

use crate::app::{Ids, refresh};
use crate::dialogs;
use crate::state::Shared;

/// Uma prévia: o texto, a imagem, o lado na tela e a máscara (raio em fração do lado; 0.5 é
/// o círculo).
struct Preview {
    label: &'static str,
    image: Image,
    side: f32,
    mask: f32,
    background: Option<Color>,
}

/// O painel das prévias.
struct IconPreview {
    base: ControlBase,
    items: Vec<Preview>,
}

impl Component for IconPreview {
    fn class_name(&self) -> &'static str {
        "IdeIconPreview"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let mut x = r.x + 8.0;
        let mut y = r.y + 8.0;
        let row_h = 150.0;
        for p in &self.items {
            let cell = p.side.max(64.0) + 24.0;
            if x + cell > r.right() {
                x = r.x + 8.0;
                y += row_h;
            }
            let img_rect = RectF::new(x + (cell - p.side) / 2.0, y + (110.0 - p.side) / 2.0, p.side, p.side);
            canvas.save();
            canvas.clip_round_rect(img_rect, p.side * p.mask);
            if let Some(bg) = p.background {
                canvas.fill_rect(img_rect, 0.0, &Brush::Solid(bg));
            }
            let px = p.image.known_size().map_or(p.side, |s| s.width);
            canvas.draw_image(&p.image, RectF::new(0.0, 0.0, px, px), img_rect, 1.0);
            canvas.restore();
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = 11.0;
            canvas.draw_text(p.label, RectF::new(x, y + 114.0, cell, 30.0), &t);
            x += cell + 8.0;
        }
    }
}

fn image_of(img: &image::RgbaImage) -> Image {
    Image::rgba(img.width(), img.height(), img.as_raw().clone())
}

/// As prévias de um ícone.
fn previews(icon: &Icon, background: [u8; 3]) -> Vec<Preview> {
    let bg = Color::rgb(background[0], background[1], background[2]);
    let adaptive = image_of(&icon.adaptive_foreground(216));
    vec![
        Preview { label: "Windows 16", image: image_of(&icon.render(16)), side: 16.0, mask: 0.0, background: None },
        Preview { label: "Windows 32", image: image_of(&icon.render(32)), side: 32.0, mask: 0.0, background: None },
        Preview { label: "Windows 256", image: image_of(&icon.render(256)), side: 96.0, mask: 0.0, background: None },
        Preview { label: "Android (redondo)", image: adaptive.clone(), side: 96.0, mask: 0.5, background: Some(bg) },
        Preview { label: "Android (arredondado)", image: adaptive, side: 96.0, mask: 0.22, background: Some(bg) },
        Preview { label: "iOS", image: image_of(&icon.render(180)), side: 96.0, mask: 0.225, background: Some(Color::WHITE) },
        Preview { label: "Navegador (aba)", image: image_of(&icon.render(32)), side: 32.0, mask: 0.0, background: None },
        Preview { label: "PWA 192", image: image_of(&icon.render(192)), side: 96.0, mask: 0.1, background: None },
    ]
}

/// Projeto → Opções do projeto → Ícone do app
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else { return };
    let Some(file) = crate::files::FileDialog::new().add_filter("Imagem do ícone", &["png", "svg", "jpg", "jpeg"]).set_directory(project.dir()).pick_file() else { return };
    let icon = match Icon::open(&file) {
        Ok(i) => i,
        Err(e) => {
            aylis_controls::message_dlg(ui, &e, aylis_controls::DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
            return;
        }
    };
    let suggested = icon.border_color();
    let current = Color::from_hex(&project.publish.icon_background).map(|c| [c.r, c.g, c.b]).unwrap_or(suggested);
    let d = dialogs::new_dialog(ui, "Ícone do app", 760.0, 520.0);
    let warnings = icon.warnings();
    let note = if warnings.is_empty() { format!("{} — fica nítido em todos os lugares.", file.display()) } else { warnings.join("\n") };
    let line = dialogs::field(ui, d.body, "Imagem");
    let _ = dialogs::edit(ui, line, &note);
    let line = dialogs::field(ui, d.body, "Fundo do adaptativo");
    let hex = format!("#{:02X}{:02X}{:02X}", current[0], current[1], current[2]);
    let background = dialogs::edit(ui, line, &hex);
    let preview = ui.add(IconPreview { base: ControlBase::new(720.0, 320.0), items: previews(&icon, current) }, d.body);
    ui.control_mut(preview).unwrap().set_align(aylis_core::Align::Top).set_height(320.0);
    let (state, ids) = (state.clone(), *ids);
    let check = move |ui: &mut Ui| -> Result<(), String> {
        let text = dialogs::text_of(ui, background);
        if !text.trim().is_empty() && Color::from_hex(text.trim()).is_none() {
            return Err("a cor de fundo é #RRGGBB".into());
        }
        Ok(())
    };
    dialogs::run(ui, d, check, move |ui| {
        let bg = dialogs::text_of(ui, background).trim().to_string();
        let mut p = project.clone();
        // O ícone fica no projeto (copiado para assets/images se veio de fora).
        let rel = match file.strip_prefix(p.dir()) {
            Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
            Err(_) => {
                let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "icone.png".into());
                let dest = p.dir().join("assets/images").join(&name);
                let _ = std::fs::create_dir_all(dest.parent().expect("tem pai"));
                if let Err(e) = std::fs::copy(&file, &dest) {
                    state.borrow_mut().status = format!("⚠ ícone: {e}");
                    refresh(ui, &state, &ids);
                    return;
                }
                format!("assets/images/{name}")
            }
        };
        p.publish.icon = rel;
        p.publish.icon_background = if bg.eq_ignore_ascii_case(&format!("#{:02X}{:02X}{:02X}", suggested[0], suggested[1], suggested[2])) { String::new() } else { bg };
        let saved = p.save().map_err(|e| e.to_string()).and_then(|_| {
            // O .ico do Windows já sai agora (o próximo build põe no .exe).
            let icon = Icon::open(&p.file(&p.publish.icon))?;
            icons::windows(&icon, &p.dir().join(aylis_build::APP_ICON))
        });
        let mut st = state.borrow_mut();
        st.status = match saved {
            Ok(()) => format!("Ícone do app: {} (gerado para cada plataforma ao publicar)", p.publish.icon),
            Err(e) => format!("⚠ ícone: {e}"),
        };
        st.project = Some(p);
        drop(st);
        refresh(ui, &state, &ids);
    });
}
