//! Player: o aplicativo. Gerado pela IDE do aylis (pode editar).

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
