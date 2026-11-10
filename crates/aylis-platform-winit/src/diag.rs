//! Diagnóstico: linhas no stderr ou, no Android, no logcat; configuração por variável de
//! ambiente ou, no Android, por propriedade do sistema.

/// Escreve uma linha de diagnóstico: no stderr, e no Android no logcat (com a etiqueta
/// `aylis`), onde o stderr não aparece.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
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

/// No Android, os avisos e erros do `log` (os do wgpu, por exemplo: shader que o driver
/// recusou) vão para o logcat, com a etiqueta do módulo; sem isso, somem.
#[cfg(target_os = "android")]
pub(crate) fn android_logger() {
    struct Logcat;
    impl log::Log for Logcat {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= log::Level::Warn
        }
        fn log(&self, record: &log::Record) {
            if !self.enabled(record.metadata()) {
                return;
            }
            #[link(name = "log")]
            unsafe extern "C" {
                fn __android_log_write(prio: i32, tag: *const std::ffi::c_char, text: *const std::ffi::c_char) -> i32;
            }
            // 5 = ANDROID_LOG_WARN, 6 = ANDROID_LOG_ERROR
            let prio = if record.level() == log::Level::Error { 6 } else { 5 };
            let tag = std::ffi::CString::new(record.target()).unwrap_or_default();
            if let Ok(text) = std::ffi::CString::new(record.args().to_string()) {
                unsafe { __android_log_write(prio, tag.as_ptr(), text.as_ptr()) };
            }
        }
        fn flush(&self) {}
    }
    static LOGCAT: Logcat = Logcat;
    // Um app que já pôs o próprio logger fica com o dele.
    if log::set_logger(&LOGCAT).is_ok() {
        log::set_max_level(log::LevelFilter::Warn);
    }
}

/// Uma configuração de diagnóstico: a variável de ambiente `name` e, no Android (onde o app
/// não recebe variáveis), a propriedade do sistema `debug.aylis.<nome sem AYLIS_, minúsculo>`
/// (`adb shell setprop debug.aylis.bench 300`).
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn diagnostic_setting(name: &str) -> Option<String> {
    // Só com a feature `diagnostics` (a `dev` do app liga; os exemplos e testes do aylis também):
    // o app publicado não muda de comportamento por variável de ambiente.
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

/// Segundos que o horário local está à frente do UTC no instante `utc` (com horário de
/// verão), pelo sistema.
pub(crate) fn local_offset(utc: i64) -> i64 {
    #[cfg(windows)]
    {
        let _ = utc;
        #[repr(C)]
        struct SystemTime([u16; 8]);
        #[repr(C)]
        struct TimeZone {
            bias: i32,
            standard_name: [u16; 32],
            standard_date: SystemTime,
            standard_bias: i32,
            daylight_name: [u16; 32],
            daylight_date: SystemTime,
            daylight_bias: i32,
        }
        unsafe extern "system" {
            fn GetTimeZoneInformation(info: *mut TimeZone) -> u32;
        }
        let mut tz: TimeZone = unsafe { std::mem::zeroed() };
        // 1 = horário padrão, 2 = de verão; o "bias" é UTC − local, em minutos.
        let kind = unsafe { GetTimeZoneInformation(&mut tz) };
        let bias = tz.bias + if kind == 2 { tz.daylight_bias } else if kind == 1 { tz.standard_bias } else { 0 };
        -(bias as i64) * 60
    }
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(utc as f64 * 1000.0));
        -(date.get_timezone_offset() as i64) * 60
    }
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    {
        // struct tm: 9 ints, depois tm_gmtoff (long) e tm_zone (glibc, bionic, Apple).
        #[repr(C)]
        struct Tm {
            fields: [i32; 9],
            gmtoff: std::ffi::c_long,
            zone: *const std::ffi::c_char,
        }
        unsafe extern "C" {
            fn localtime_r(t: *const i64, tm: *mut Tm) -> *mut Tm;
        }
        let mut tm: Tm = unsafe { std::mem::zeroed() };
        if unsafe { localtime_r(&utc, &mut tm) }.is_null() { 0 } else { tm.gmtoff as i64 }
    }
    #[cfg(not(any(windows, unix, target_arch = "wasm32")))]
    {
        let _ = utc;
        0
    }
}
