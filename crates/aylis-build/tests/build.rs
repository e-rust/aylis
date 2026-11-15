use aylis_build::*;

/// Um PNG mínimo (só o cabeçalho importa aqui).
fn png(w: u32, h: u32) -> Vec<u8> {
    let mut p = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
    p.extend_from_slice(&w.to_be_bytes());
    p.extend_from_slice(&h.to_be_bytes());
    p.extend_from_slice(&[8, 6, 0, 0, 0, 0, 0, 0, 0]);
    p
}

#[test]
fn packs_the_project_files_and_reads_them_back() {
    let dir = std::env::temp_dir().join(format!("aylis_build_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("forms")).unwrap();
    std::fs::create_dir_all(dir.join("assets/images")).unwrap();
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("Loja.aylisproj"), "{}").unwrap();
    std::fs::write(dir.join("forms/principal.json"), "{\"aylis\":1}").unwrap();
    std::fs::write(dir.join("assets/images/logo.png"), [1u8, 2, 3]).unwrap();
    std::fs::write(dir.join("assets/images/.thumbs"), "x").unwrap();
    std::fs::write(dir.join("src/main.rs"), "fn main() {}").unwrap();

    let bundle = pack(&dir).unwrap();
    let files = unpack(&bundle).unwrap();
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["Loja.aylisproj", "assets/images/logo.png", "forms/principal.json"], "sem src/ nem ocultos");
    assert_eq!(files[1].1, [1, 2, 3]);
    assert!(unpack(&bundle[..bundle.len() - 1]).is_err(), "pacote cortado");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn icon_resources() {
    assert_eq!(png_size(&png(48, 32)), Some((48, 32)));
    assert!(icon_res(&png(512, 512)).is_err(), "grande demais");
    let res = icon_res(&png(256, 256)).unwrap();
    assert_eq!(res.len() % 4, 0);
    // entrada vazia, depois RT_ICON (3) id 1
    assert_eq!(&res[..8], &[0, 0, 0, 0, 32, 0, 0, 0]);
    assert_eq!(&res[40..48], &[0xFF, 0xFF, 3, 0, 0xFF, 0xFF, 1, 0]);
    let ico = ico(&png(256, 256)).unwrap();
    assert_eq!(&ico[..8], &[0, 0, 1, 0, 1, 0, 0, 0], "256 vira 0");
}

/// Os dados de versão do `.exe` (14/11: o Cortex XDR e o SmartScreen marcavam os apps sem
/// empresa nem produto): o `.res` leva o `RT_VERSION` com os textos do projeto.
#[test]
fn the_res_has_the_version_info_of_the_project() {
    let project = serde_json::json!({
        "aylis_project": 1, "name": "Loja",
        "publish": { "app_name": "Vendas: 2024", "version": "1.2.3-beta", "publisher": "Empresa Ltda" }
    });
    let info = VersionInfo::of_value(&project);
    assert_eq!(info.original_filename, "Vendas 2024.exe", "o nome do .exe publicado");
    assert_eq!(info.language, 0x0416, "sem locale: português do Brasil");
    // Junto do ícone, no mesmo .res.
    let mut res = icon_res(&png(256, 256)).unwrap();
    add_version(&mut res, &info);
    assert_eq!(res.len() % 4, 0);
    let entries = res_entries(&res).unwrap();
    assert_eq!(entries.iter().map(|e| (e.0, e.1)).collect::<Vec<_>>(), [(0, 0), (3, 1), (14, 1), (16, 1)]);
    let (_, _, language, data) = entries[3];
    assert_eq!(language, 0x0416);
    let (numbers, strings) = read_version_data(data).unwrap();
    assert_eq!(numbers, [1, 2, 3, 0]);
    let get = |k: &str| strings.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());
    assert_eq!(get("CompanyName"), Some("Empresa Ltda"));
    assert_eq!(get("FileDescription"), Some("Vendas: 2024"));
    assert_eq!(get("ProductName"), Some("Vendas: 2024"));
    assert_eq!(get("FileVersion"), Some("1.2.3-beta"));
    assert_eq!(get("ProductVersion"), Some("1.2.3-beta"));
    assert_eq!(get("LegalCopyright"), Some("© Empresa Ltda"));
    assert_eq!(get("OriginalFilename"), Some("Vendas 2024.exe"));
    assert_eq!(get("InternalName"), Some("Vendas 2024.exe"));
    // A tabela de textos e a tradução: 0416 com Unicode (04B0).
    let utf16 = |s: &str| s.encode_utf16().flat_map(u16::to_le_bytes).collect::<Vec<u8>>();
    assert!(data.windows(16).any(|w| w == utf16("041604B0").as_slice()));
    assert!(data.windows(4).any(|w| w == [0x16, 0x04, 0xB0, 0x04]), "VarFileInfo\\Translation");

    // Sem fabricante: o nome do app; em inglês, 0409; só a versão, sem ícone.
    let info = VersionInfo::of_value(&serde_json::json!({ "name": "Paint", "locale": "en-US" }));
    assert_eq!((info.company.as_str(), info.version.as_str(), info.language), ("Paint", "1.0.0", 0x0409));
    let res = version_res(&info);
    let entries = res_entries(&res).unwrap();
    assert_eq!(entries.iter().map(|e| (e.0, e.1, e.2)).collect::<Vec<_>>(), [(0, 0, 0), (16, 1, 0x0409)]);
    assert!(entries[1].3.windows(16).any(|w| w == utf16("040904B0").as_slice()));
    assert_eq!(version4("0.52.7-beta"), "0.52.7.0");
}

/// O `.res` com a versão, ligado num executável de verdade pelo `link.exe` (pelo próprio
/// `cargo`), é lido pelo Windows (`GetFileVersionInfoW`, pelo PowerShell).
#[cfg(all(windows, target_env = "msvc"))]
#[test]
fn windows_reads_the_version_of_a_linked_exe() {
    let dir = std::env::temp_dir().join(format!("aylis_build_version_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let info = VersionInfo::of_value(&serde_json::json!({ "name": "Teste", "publish": { "version": "2.5.1", "publisher": "Fábrica Ação" } }));
    std::fs::write(dir.join("app.res"), version_res(&info)).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"versao_teste\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n").unwrap();
    std::fs::write(dir.join("src/main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.join("build.rs"), "fn main() { println!(\"cargo:rustc-link-arg-bins={}\", std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"app.res\").display()); }").unwrap();
    let target = dir.join("target");
    let status = std::process::Command::new(env!("CARGO"))
        .args(["build", "--quiet", "--offline"])
        .current_dir(&dir)
        .env("CARGO_TARGET_DIR", &target)
        .status()
        .unwrap();
    assert!(status.success());
    let exe = target.join("debug").join("versao_teste.exe");
    let script = format!("[Console]::OutputEncoding = [Text.Encoding]::UTF8; $v =(Get-Item -LiteralPath '{}').VersionInfo; \"$($v.CompanyName)|$($v.ProductVersion)|$($v.FileMajorPart).$($v.FileMinorPart).$($v.FileBuildPart)\"", exe.display());
    let out = std::process::Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", &script]).output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(text.trim(), "Fábrica Ação|2.5.1|2.5.1", "{}", String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn reads_an_ico_with_several_sizes_and_makes_its_res() {
    // Um .ico com 16 e 256 (o 256 vai como 0 no cabeçalho).
    let images = [png(16, 16), png(256, 256)];
    let mut ico = vec![0, 0, 1, 0, 2, 0];
    let mut offset = 6 + 16 * images.len() as u32;
    for img in &images {
        let (w, h) = png_size(img).unwrap();
        ico.extend_from_slice(&[w as u8, h as u8, 0, 0, 1, 0, 32, 0]);
        ico.extend_from_slice(&(img.len() as u32).to_le_bytes());
        ico.extend_from_slice(&offset.to_le_bytes());
        offset += img.len() as u32;
    }
    for img in &images {
        ico.extend_from_slice(img);
    }
    let read = ico_images(&ico).unwrap();
    assert_eq!(read.iter().map(|(w, h, _)| (*w, *h)).collect::<Vec<_>>(), vec![(16, 16), (256, 256)]);
    assert_eq!(png_size(ico_largest_png(&ico).unwrap()), Some((256, 256)));
    // O .res: a entrada vazia, um RT_ICON por imagem e o grupo.
    let res = ico_res(&ico).unwrap();
    assert!(res.windows(images[1].len()).any(|w| w == images[1].as_slice()));
    assert!(ico_images(b"nada").is_err());
    // O ícone da IDE é um .ico válido com o tamanho grande.
    let ide = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../aylis-ide/resources/aylis-ide.ico")).unwrap();
    assert_eq!(ico_largest_png(&ide).and_then(png_size), Some((256, 256)));
}

#[test]
fn finds_the_classes_the_code_creates_by_name() {
    let dir = std::env::temp_dir().join(format!("aylis_build_code_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src/screens")).unwrap();
    std::fs::write(dir.join("src/main.rs"), r#"fn main() { let t = "say \"hi\""; ui.create("MediaCard", None); }"#).unwrap();
    std::fs::write(dir.join("src/screens/a.rs"), r#"ui.create("CheckBox", Some(p)); let s = "not a class"; let m = "MediaCard";"#).unwrap();
    assert_eq!(aylis_build::code_classes(&dir.join("src")), ["CheckBox", "MediaCard"]);
    // A classe que liga uma feature: o nome num comentário ou num texto não conta (um fone
    // "Bluetooth" não traz o btleplug); criada, conta.
    std::fs::write(dir.join("src/screens/b.rs"), "/// \"Hands-Free\"/\"Bluetooth\" no nome.\nlet nfc = \"Nfc\";").unwrap();
    assert_eq!(aylis_build::code_classes(&dir.join("src")), ["CheckBox", "MediaCard"]);
    std::fs::write(dir.join("src/screens/b.rs"), "let bt = ui.create( \"Bluetooth\", None);").unwrap();
    assert_eq!(aylis_build::code_classes(&dir.join("src")), ["Bluetooth", "CheckBox", "MediaCard"]);
    let _ = std::fs::remove_dir_all(&dir);
}

/// As classes da documentação ao vivo da página (11/11): só as das famílias pedidas, que rodam
/// no navegador e não pedem feature do `aylis` (essas ficam no wasm da família).
#[test]
fn the_page_registers_only_the_light_families() {
    let facade = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../aylis/src/lib.rs")).unwrap();
    let source = demo_classes_source(&facade, &["Standard", "Industrial"]);
    assert!(source.contains("::aylis::classes::Button(ui);"), "{source}");
    assert!(source.contains("::aylis::classes::Valve(ui);"), "{source}");
    // Outra família (o Gauge, de Additional) e as que pedem feature (o DBGrid, de Data
    // Controls) ficam fora.
    assert!(!source.contains("Gauge(") && !source.contains("DBGrid") && !source.contains("Model3D"), "{source}");
    let classes = source.matches("::aylis::classes::").count();
    assert!(classes > 20, "{classes} classes");
}
