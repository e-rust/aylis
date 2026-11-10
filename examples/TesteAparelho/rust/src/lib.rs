//! TesteAparelho: o aplicativo. Gerado pela IDE do aylis (pode editar).

#[path = "../generated/mod.rs"]
pub mod generated;

/// Cria o aplicativo: tema, idioma, assets e os forms marcados como "criar no início",
/// com os handlers de todos os forms registrados.
///
/// No desenvolvimento os arquivos vêm da pasta do projeto; no aplicativo publicado,
/// de dentro do executável.
pub fn app() -> Result<aylis::Application, aylis::Error> {
    let mut app = aylis::Application::bundled(aylis::include_project!())?;
    generated::register(&mut app.ui);
    aylis::background::set_worker(background::run);
    Ok(app)
}

/// Android: a NativeActivity carrega a biblioteca e chama esta função.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android: aylis::platform::android::AndroidApp) {
    aylis::run_android(android, app);
}

/// Navegador: a página chama esta função depois de carregar o WebAssembly.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn aylis_start() {
    aylis::run_web(app);
}

// O serviço em segundo plano (acrescentado pela IDE ao usar o BackgroundService).
pub mod background;

/// Android: o serviço em primeiro plano (também sozinho, no boot ou quando o sistema o
/// reinicia) começa a lógica sem tela.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_aylis_AylisActivity_00024Background_nativeStart(_env: *mut std::ffi::c_void, _class: *mut std::ffi::c_void) {
    aylis::background::start(Some(background::run));
}
