//! Apelidos de configuração usados no código:
//! - `web`: navegador (wasm32);
//! - `native_cpu`, `native_gpu`, `native_clipboard`: fora do navegador, com a feature
//!   correspondente (`native_clipboard` só no desktop; `native_gpu` ainda não no iOS,
//!   que não tem OpenGL no glutin: lá desenha na CPU até existir o backend Metal);
//! - `native_skia`: `native_cpu` ou `native_gpu` (o Skia está presente);
//! - `native_d2d`: Windows com a feature `d2d` (Direct2D, sem Skia);
//! - `mobile`: Android ou iOS;
//! - `native_webview` / `web_webview` / `android_webview`: a feature `webview` no Windows,
//!   no macOS, no Linux e no iOS (wry), no navegador (iframe) ou no Android (o WebView do
//!   sistema na `AylisActivity`).

fn main() {
    for name in ["web", "native_cpu", "native_gpu", "native_clipboard", "native_skia", "native_d2d", "mobile", "native_webview", "web_webview", "android_webview", "native_tray"] {
        println!("cargo::rustc-check-cfg=cfg({name})");
    }
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|a| a == "wasm32");
    if wasm {
        println!("cargo::rustc-cfg=web");
        if std::env::var("CARGO_FEATURE_WEBVIEW").is_ok() {
            println!("cargo::rustc-cfg=web_webview");
        }
        return;
    }
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let mobile = os == "android" || os == "ios";
    if mobile {
        println!("cargo::rustc-cfg=mobile");
    }
    let mut skia = false;
    for feature in ["cpu", "gpu", "clipboard", "d2d"] {
        if (mobile && feature == "clipboard") || (os == "ios" && feature == "gpu") || (feature == "d2d" && os != "windows") {
            continue;
        }
        if std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok() {
            println!("cargo::rustc-cfg=native_{feature}");
            skia |= feature == "cpu" || feature == "gpu";
        }
    }
    if skia {
        println!("cargo::rustc-cfg=native_skia");
    }
    if std::env::var("CARGO_FEATURE_TRAY").is_ok() && (os == "windows" || os == "macos") {
        println!("cargo::rustc-cfg=native_tray");
    }
    if std::env::var("CARGO_FEATURE_WEBVIEW").is_ok() {
        match os.as_str() {
            "windows" | "macos" | "linux" | "ios" => println!("cargo::rustc-cfg=native_webview"),
            "android" => println!("cargo::rustc-cfg=android_webview"),
            _ => {}
        }
    }
}
