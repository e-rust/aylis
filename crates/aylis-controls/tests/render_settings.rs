//! O `RenderSettings` e o `frame_reuse_kb` do form (decidido em 14/11): o projeto configura o
//! app, a propriedade do form configura o form, e em execução o app troca o valor do app
//! inteiro. Os valores são do processo: os testes que mexem neles passam um de cada vez e
//! devolvem o padrão no fim.

use std::sync::Mutex;

use aylis_core::image_cache::{self, DEFAULT_FRAME_REUSE_KB};
use aylis_core::*;

static GLOBALS: Mutex<()> = Mutex::new(());

fn setup(design: bool) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_design_mode(design);
    let form = ui.create("Form", None).unwrap();
    let settings = ui.create("RenderSettings", Some(form)).unwrap();
    (ui, form, settings)
}

fn reset() {
    image_cache::set_limits(ImageCacheLimits::DEFAULT);
    image_cache::set_auto_layers(true);
    image_cache::set_frame_reuse_kb(DEFAULT_FRAME_REUSE_KB);
    gpu::set_blend_compositor(gpu::BlendCompositor::Direct2d);
}

fn int(ui: &Ui, id: ComponentId, name: &str) -> i64 {
    ui.get_prop(id, name).unwrap().to_i64().unwrap()
}

#[test]
fn render_settings_read_and_change_the_app_values_now() {
    let _turn = GLOBALS.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let (mut ui, _, s) = setup(false);
    // Lê o que vale no processo (o que o projeto aplicou ao abrir).
    image_cache::set_limits(ImageCacheLimits { pixels_mb: 200, files: 1000, layers_mb: 64 });
    assert_eq!((int(&ui, s, "pixels_mb"), int(&ui, s, "files"), int(&ui, s, "layers_mb")), (200, 1000, 64));
    assert_eq!(int(&ui, s, "frame_reuse_kb"), DEFAULT_FRAME_REUSE_KB as i64);
    // Troca: vale já, no app inteiro.
    ui.set_prop(s, "pixels_mb", 48).unwrap();
    ui.set_prop(s, "files", 500).unwrap();
    ui.set_prop(s, "layers_mb", 0).unwrap();
    ui.set_prop(s, "frame_reuse_kb", 256).unwrap();
    ui.set_prop(s, "auto_layers", false).unwrap();
    assert_eq!(image_cache::limits(), ImageCacheLimits { pixels_mb: 48, files: 500, layers_mb: 0 });
    assert_eq!(image_cache::frame_reuse_kb(), 256);
    assert!(!image_cache::auto_layers());
    assert!(!LayerCache::<()>::accepts(1), "sem camadas guardadas a partir de agora");
    // Fora da faixa: ajustado, e ler de volta diz o que ficou.
    ui.set_prop(s, "pixels_mb", 1).unwrap();
    ui.set_prop(s, "files", 10_000_000).unwrap();
    ui.set_prop(s, "frame_reuse_kb", -5).unwrap();
    assert_eq!(int(&ui, s, "pixels_mb"), ImageCacheLimits::PIXELS_MB_RANGE.0 as i64);
    assert_eq!(int(&ui, s, "files"), ImageCacheLimits::FILES_RANGE.1 as i64);
    assert_eq!(int(&ui, s, "frame_reuse_kb"), 0);
    // Não grava no form: as propriedades são de execução (o JSON pula as `runtime`).
    for name in ["pixels_mb", "files", "layers_mb", "auto_layers", "frame_reuse_kb"] {
        assert!(ui.prop_info(s, name).is_some_and(|p| p.runtime), "{name}");
    }
    reset();
}

#[test]
fn in_the_designer_render_settings_do_not_touch_the_ide() {
    let _turn = GLOBALS.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let (mut ui, _, s) = setup(true);
    assert_eq!(int(&ui, s, "pixels_mb"), ImageCacheLimits::DEFAULT.pixels_mb as i64);
    ui.set_prop(s, "pixels_mb", 32).unwrap();
    ui.set_prop(s, "auto_layers", false).unwrap();
    ui.set_prop(s, "frame_reuse_kb", 64).unwrap();
    // O valor fica no componente; os da IDE (do processo), como estavam.
    assert_eq!(int(&ui, s, "pixels_mb"), 32);
    assert_eq!(ui.get_prop(s, "auto_layers").unwrap(), PropValue::Bool(false));
    assert_eq!(image_cache::limits(), ImageCacheLimits::DEFAULT);
    assert!(image_cache::auto_layers());
    assert_eq!(image_cache::frame_reuse_kb(), DEFAULT_FRAME_REUSE_KB);
    reset();
}

/// O `frame_reuse_kb` do form: `-1` segue o do projeto (e o que o `RenderSettings` trocar); um
/// valor próprio vale só naquele form, e é o que a plataforma passa ao Direct2D da janela.
#[test]
fn the_form_frame_reuse_is_its_own_or_the_project_one() {
    let _turn = GLOBALS.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let (mut ui, form, s) = setup(false);
    let other = ui.create("Form", None).unwrap();
    assert_eq!(ui.get_prop(form, "frame_reuse_kb").unwrap(), PropValue::Int(-1));
    assert_eq!(ui.form_frame_reuse_kb(form), None);
    ui.set_prop(form, "frame_reuse_kb", 4096).unwrap();
    assert_eq!(ui.form_frame_reuse_kb(form), Some(4096));
    ui.set_prop(form, "frame_reuse_kb", 10_000_000).unwrap();
    assert_eq!(ui.form_frame_reuse_kb(form), Some(image_cache::FRAME_REUSE_KB_RANGE.1), "acima da faixa: ajustado");
    ui.set_prop(form, "frame_reuse_kb", 0).unwrap();
    // O que decide no Direct2D: o diagnóstico (só em depuração) > o form > o projeto.
    let limit = |ui: &Ui, f| image_cache::frame_reuse_limit(None, ui.form_frame_reuse_kb(f));
    assert_eq!(limit(&ui, form), 0, "o form desliga só nele");
    assert_eq!(limit(&ui, other), DEFAULT_FRAME_REUSE_KB, "o outro segue o projeto");
    ui.set_prop(s, "frame_reuse_kb", 2048).unwrap();
    assert_eq!((limit(&ui, form), limit(&ui, other)), (0, 2048), "o RenderSettings muda o do projeto");
    assert_eq!(image_cache::frame_reuse_limit(Some(7), Some(0)), 7, "o diagnóstico ganha de todos");
    // 320 × 180 (225 KB): reaproveita no outro, não no form desligado.
    assert!(image_cache::reuses_frame(320, 180, limit(&ui, other)) && !image_cache::reuses_frame(320, 180, limit(&ui, form)));
    // Gravado no form (é configuração dele, não de execução).
    assert!(ui.prop_info(form, "frame_reuse_kb").is_some_and(|p| !p.runtime));
    reset();
}

/// O `blend_compositor` (o efeito do Direct2D ou o shader do Aylis em Direct3D), nos três
/// níveis: o projeto (o valor do processo), o form (`project` segue o do projeto) e o
/// `RenderSettings` em execução, que troca o do projeto e não fica gravado.
#[test]
fn the_blend_compositor_is_the_form_one_or_the_project_one() {
    let _turn = GLOBALS.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let (mut ui, form, s) = setup(false);
    let other = ui.create("Form", None).unwrap();
    assert_eq!(ui.get_prop(form, "blend_compositor").unwrap(), PropValue::Enum("project".into()));
    assert_eq!(ui.form_blend_compositor(form), None, "project: o do projeto");
    assert_eq!(ui.get_prop(s, "blend_compositor").unwrap(), PropValue::Enum("direct2d".into()), "o padrão");
    ui.set_prop(form, "blend_compositor", "direct3d").unwrap();
    assert_eq!(ui.form_blend_compositor(form), Some(gpu::BlendCompositor::Direct3d));
    assert_eq!(ui.form_blend_compositor(other), None);
    // O RenderSettings troca o do app inteiro, já.
    ui.set_prop(s, "blend_compositor", "direct3d").unwrap();
    assert_eq!(gpu::blend_compositor(), gpu::BlendCompositor::Direct3d);
    assert!(ui.prop_info(s, "blend_compositor").is_some_and(|p| p.runtime), "não grava no form");
    assert!(ui.prop_info(form, "blend_compositor").is_some_and(|p| !p.runtime), "o do form é gravado");
    // Um nome que não existe é recusado.
    assert!(ui.set_prop(form, "blend_compositor", "opengl").is_err());
    reset();
}
