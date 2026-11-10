//! A IDE lite do aylis no navegador ("Experimentar" na página do aylis). A página carrega o
//! `.wasm` e chama [`aylis_ide_start`]; a IDE é a mesma do desktop (`aylis-ide`), sem a
//! feature `desktop`: paleta, designer, inspector, estrutura, temas, vistas e prévia, com o
//! projeto guardado no navegador e baixado em ZIP. Monte com `aylis-tool ide-web --out pasta`.

/// A página chama depois de carregar o WebAssembly.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn aylis_ide_start() {
    std::panic::set_hook(Box::new(|info| web_sys::console::error_1(&info.to_string().into())));
    if let Err(e) = aylis_ide::lite::run() {
        web_sys::console::error_1(&format!("aylis-ide: {e}").into());
    }
}
