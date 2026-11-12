//! As telas da IDE comparadas com imagens de referência (item 11.3: "é quase impossível
//! testar tudo visualmente"). Cada tela é desenhada sem janela (Skia na CPU) e comparada com
//! `tests/visual/<tela>-<sistema>.png`, com tolerância: conta como diferente o pixel que muda
//! mais de [`TONES`] tons, e a tela falha se mais de [`SHARE`] dos pixels mudarem.
//!
//! Sem a referência (tela nova, outro sistema), o teste grava a imagem e passa: confira a
//! imagem e faça o commit dela. Uma mudança visual de propósito: rode com
//! `AYLIS_UPDATE_VISUAL=1` para regravar, e confira as imagens no diff. Uma falha grava a
//! imagem nova em `visual/` dentro do `CARGO_TARGET_TMPDIR` (a pasta de compilação) para comparar.

#![cfg(feature = "desktop")]

use std::path::PathBuf;

use aylis_core::{ComponentId, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app, gallery};

const TONES: u8 = 48;
const SHARE: f64 = 0.005;

fn reference(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/visual").join(format!("{name}-{}.png", std::env::consts::OS))
}

/// Desenha o form `form` em RGBA, no tamanho dele.
fn render(ui: &mut Ui, form: ComponentId) -> (Vec<u8>, u32, u32) {
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let (w, h) = ui.control(form).map_or((1440.0, 900.0), |cb| (cb.width(), cb.height()));
    let (w, h) = (w as u32, h as u32);
    let background = ui.theme().color("background");
    let png = renderer.render_png(w, h, 1.0, background, |c| ui.paint(form, c)).expect("PNG");
    let (rgba, w, h) = aylis_skia::decode_rgba(&png).expect("PNG lido");
    (rgba, w, h)
}

/// Compara com a referência (ou grava, se não houver ou se pedirem).
fn check(name: &str, (rgba, w, h): (Vec<u8>, u32, u32)) {
    let path = reference(name);
    let png = aylis_skia::encode_png(&rgba, w, h).expect("PNG");
    if std::env::var_os("AYLIS_UPDATE_VISUAL").is_some() || !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &png).unwrap();
        eprintln!("{name}: referência gravada em {} (confira e faça o commit)", path.display());
        return;
    }
    let (expected, ew, eh) = aylis_skia::decode_rgba(&std::fs::read(&path).unwrap()).expect("referência");
    // Na pasta de compilação (a única da máquina, `%LOCALAPPDATA%ylis	arget`), nunca no
    // `target/` do repositório.
    let failed = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("visual");
    let save = || {
        std::fs::create_dir_all(&failed).unwrap();
        std::fs::write(failed.join(format!("{name}.png")), &png).unwrap();
    };
    if (ew, eh) != (w, h) {
        save();
        panic!("{name}: tamanho {w}x{h}, a referência tem {ew}x{eh} (a nova está em {})", failed.display());
    }
    let changed = rgba.chunks_exact(4).zip(expected.chunks_exact(4)).filter(|(a, b)| a.iter().zip(b.iter()).take(3).any(|(x, y)| x.abs_diff(*y) > TONES)).count();
    let share = changed as f64 / (w * h) as f64;
    if share > SHARE {
        save();
        panic!("{name}: {:.2}% dos pixels mudaram (limite {:.1}%); a nova está em {}", share * 100.0, SHARE * 100.0, failed.join(format!("{name}.png")).display());
    }
}

#[test]
fn the_ide_screens_look_like_the_references() {
    // A IDE sem a configuração do usuário (o layout salvo dos painéis mudaria a imagem).
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let main = ide.form;
    check("principal", render(&mut ide.ui, main));
    let form = gallery::open(&mut ide.ui, &ide.state, &ide.ids);
    check("galeria", render(&mut ide.ui, form));
}

#[test]
fn the_designer_with_a_project_looks_like_the_reference() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/Calculadora/Calculadora.aylisproj");
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let main = ide.form;
    check("designer", render(&mut ide.ui, main));
}

/// Os forms principais dos exemplos (só o layout, com as classes que a IDE registra): o que um
/// controle mudar de aparência aparece aqui.
#[test]
fn the_example_forms_look_like_the_references() {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    for app in ["Calculadora", "Paint", "IBGE", "Player"] {
        // Todas as classes (os dados, a mídia...), como no aplicativo.
        let mut ui = Ui::new();
        aylis_remote::register_all(&mut ui);
        let project = aylis_project::Project::open(examples.join(app).join(format!("{app}.aylisproj"))).unwrap();
        let entry = project.forms.iter().find(|f| f.name == project.main_form).unwrap();
        let text = std::fs::read_to_string(project.dir().join(&entry.file)).unwrap();
        let form = aylis_json::load(&mut ui, &text, None).unwrap_or_else(|e| panic!("{app}: {e}"));
        check(&format!("exemplo-{}", app.to_lowercase()), render(&mut ui, form));
    }
}

/// O visualizador de uma gravação de uso (`aylis-ide --recording`), com uma gravação de exemplo
/// (sem dados de ninguém).
#[test]
fn the_recording_viewer_looks_like_the_reference() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/visual/exemplo.aylisrec");
    let viewer = aylis_ide::recordings::show(&mut ide.ui, &file).expect("gravação de exemplo");
    check("gravacao", render(&mut ide.ui, viewer));
}
