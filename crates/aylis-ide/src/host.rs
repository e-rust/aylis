//! O que a IDE pede da máquina e muda entre o desktop e a IDE lite do navegador: as fontes
//! instaladas, a foto da tela, as placas de vídeo e a conversão de FBX. Na lite, o que não
//! existe lá volta vazio (ou um erro que diz onde fazer).

use aylis_core::{Canvas, Color};

/// As famílias de fonte instaladas no sistema (no navegador, nenhuma além das do aylis).
pub fn system_font_families() -> Vec<String> {
    #[cfg(feature = "desktop")]
    return aylis_skia::system_font_families();
    #[cfg(not(feature = "desktop"))]
    Vec::new()
}

/// Pinta numa imagem e devolve o PNG (a foto da tela do relato e do `--png`).
pub fn render_png(width: u32, height: u32, background: Color, paint: impl FnOnce(&mut dyn Canvas)) -> Option<Vec<u8>> {
    #[cfg(feature = "desktop")]
    return aylis_skia::SkiaRenderer::new().render_png(width, height, 1.0, background, |canvas| paint(canvas));
    #[cfg(not(feature = "desktop"))]
    {
        let _ = (width, height, background, paint);
        None
    }
}

/// As placas de vídeo pelo nome (para escolher uma nas opções do projeto). Perguntar ao
/// sistema leva ~200 ms e as placas não mudam com a IDE aberta: a primeira resposta fica.
pub fn gpu_adapters() -> Vec<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static ADAPTERS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
        ADAPTERS.get_or_init(aylis_3d::adapters).clone()
    }
    #[cfg(target_arch = "wasm32")]
    Vec::new()
}

/// Pergunta as placas numa thread, para a aba Geral das Opções do projeto abrir sem esperar.
pub fn prefetch_gpu_adapters() {
    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(|| drop(gpu_adapters()));
}

/// Converte os bytes de um FBX no modelo do aylis (só a IDE de desktop lê FBX).
pub fn model_from_fbx(bytes: &[u8], dir: Option<&std::path::Path>) -> Result<aylis_3d::Model, String> {
    #[cfg(feature = "desktop")]
    return aylis_3d::fbx::from_fbx(bytes, dir);
    #[cfg(not(feature = "desktop"))]
    {
        let _ = (bytes, dir);
        Err("importar FBX só na IDE de desktop".into())
    }
}
