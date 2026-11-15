//! Põe o ícone da IDE (`resources/aylis-ide.ico`, todos os tamanhos) e os dados de versão no
//! executável do Windows.

fn main() {
    let ico = "resources/aylis-ide.ico";
    println!("cargo:rerun-if-changed={ico}");
    let windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc");
    if !(windows && msvc) {
        return;
    }
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("aylis-ide.res");
    let mut res = match std::fs::read(ico).map_err(|e| e.to_string()).and_then(|ico| aylis_build::ico_res(&ico)) {
        Ok(res) => res,
        Err(e) => {
            println!("cargo:warning=ícone {ico} ignorado: {e}");
            Vec::new()
        }
    };
    // Os dados de versão (empresa, produto, versão), como nos apps publicados.
    let exe = "aylis-ide.exe".to_string();
    let info = aylis_build::VersionInfo {
        company: "Aylis".into(),
        description: "Aylis IDE".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        product: "Aylis".into(),
        copyright: "© Aylis".into(),
        original_filename: exe.clone(),
        internal_name: exe,
        language: 0x0416,
    };
    aylis_build::add_version(&mut res, &info);
    std::fs::write(&out, res).expect("gravar aylis-ide.res");
    // O link.exe aceita .res direto (converte sozinho).
    println!("cargo:rustc-link-arg-bins={}", out.display());
}
