//! Põe o ícone da IDE (`resources/aylis-ide.ico`, todos os tamanhos) no executável do Windows.

fn main() {
    let ico = "resources/aylis-ide.ico";
    println!("cargo:rerun-if-changed={ico}");
    let windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc");
    if !(windows && msvc) {
        return;
    }
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("aylis-ide.res");
    match std::fs::read(ico).map_err(|e| e.to_string()).and_then(|ico| aylis_build::ico_res(&ico)) {
        Ok(res) => {
            std::fs::write(&out, res).expect("gravar aylis-ide.res");
            // O link.exe aceita .res direto (converte sozinho).
            println!("cargo:rustc-link-arg-bins={}", out.display());
        }
        Err(e) => println!("cargo:warning=ícone {ico} ignorado: {e}"),
    }
}
