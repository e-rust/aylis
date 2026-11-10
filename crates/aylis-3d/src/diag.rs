//! Diagnóstico: linhas no stderr ou, no Android, no logcat; configuração por variável de
//! ambiente ou, no Android, por propriedade do sistema.

/// Escreve uma linha de diagnóstico: no stderr, e no Android no logcat (com a etiqueta
/// `aylis`), onde o stderr não aparece.
pub(crate) fn diagnostic(line: &str) {
    #[cfg(target_os = "android")]
    {
        #[link(name = "log")]
        unsafe extern "C" {
            fn __android_log_write(prio: i32, tag: *const std::ffi::c_char, text: *const std::ffi::c_char) -> i32;
        }
        if let Ok(text) = std::ffi::CString::new(line) {
            // 4 = ANDROID_LOG_INFO
            unsafe { __android_log_write(4, c"aylis".as_ptr(), text.as_ptr()) };
        }
    }
    #[cfg(not(target_os = "android"))]
    aylis_core::stderr_line!("{line}");
}

/// Uma configuração de diagnóstico: a variável de ambiente `name` e, no Android (onde o app
/// não recebe variáveis), a propriedade do sistema `debug.aylis.<nome sem AYLIS_, minúsculo>`
/// (`adb shell setprop debug.aylis.bench 300`).
pub(crate) fn diagnostic_setting(name: &str) -> Option<String> {
    // Só com a feature `diagnostics` (a `dev` do app liga), como na plataforma: o app
    // publicado não muda de comportamento por variável de ambiente nem propriedade.
    if !cfg!(feature = "diagnostics") {
        return None;
    }
    if let Ok(v) = std::env::var(name) {
        return Some(v);
    }
    #[cfg(target_os = "android")]
    {
        unsafe extern "C" {
            fn __system_property_get(name: *const std::ffi::c_char, value: *mut std::ffi::c_char) -> i32;
        }
        let prop = format!("debug.aylis.{}", name.trim_start_matches("AYLIS_").to_lowercase());
        let prop = std::ffi::CString::new(prop).ok()?;
        let mut value = [0 as std::ffi::c_char; 92];
        let n = unsafe { __system_property_get(prop.as_ptr(), value.as_mut_ptr()) };
        if n > 0 {
            let v = unsafe { std::ffi::CStr::from_ptr(value.as_ptr()) }.to_string_lossy().into_owned();
            return Some(v);
        }
    }
    None
}
