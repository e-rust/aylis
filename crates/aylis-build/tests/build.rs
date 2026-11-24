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

/// Um ícone citado só no código (`set_icon(ui, "material:info")`) entra no pacote embutido
/// mesmo sem o "Gerar código" da IDE: o `cargo build` sozinho acrescenta ao subconjunto do
/// projeto os que faltam, sem mexer no arquivo do projeto.
#[test]
fn icons_cited_only_in_the_code_go_into_the_bundle() {
    let dir = std::env::temp_dir().join(format!("aylis_build_icons_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("assets/icons")).unwrap();
    std::fs::create_dir_all(dir.join("rust/src/forms")).unwrap();
    std::fs::write(dir.join("Loja.aylisproj"), "{}").unwrap();
    let full = std::fs::read_to_string(icon_packs_dir().expect("os pacotes do Aylis").join("material.txt")).unwrap();
    let (subset, absent) = add_icons(None, &full, &["home"]);
    assert!(absent.is_empty());
    std::fs::write(dir.join("assets/icons/material.txt"), &subset).unwrap();
    std::fs::write(dir.join("rust/src/forms/principal.rs"), "fn f() { btn.set_icon(ui, \"material:info\"); let _ = \"xmaterial:close\"; }").unwrap();

    let names = |text: &str| text.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.split_once('\t')).map(|(n, _)| n.to_string()).collect::<Vec<_>>();
    let bundle = pack_for_build(&dir, &dir.join("rust/src")).unwrap();
    let files = unpack(&bundle).unwrap();
    let packed = files.iter().find(|(n, _)| n == "assets/icons/material.txt").expect("o pacote de ícones");
    let packed = String::from_utf8(packed.1.to_vec()).unwrap();
    assert_eq!(names(&packed), ["home", "info"], "o do código entra; \"xmaterial:\" não é citação");
    assert!(packed.starts_with("# aylis-icons 1"), "com o cabeçalho e a licença");
    assert_eq!(std::fs::read_to_string(dir.join("assets/icons/material.txt")).unwrap(), subset, "o projeto fica como está");
    // Já no subconjunto: o pacote vai como está.
    std::fs::write(dir.join("rust/src/forms/principal.rs"), "\"material:home\"").unwrap();
    assert_eq!(pack_for_build(&dir, &dir.join("rust/src")).unwrap(), pack(&dir).unwrap());
    let _ = std::fs::remove_dir_all(&dir);
}

/// A descrição e os direitos do projeto (`publish.description`, `publish.copyright`) nos dados
/// de versão; sem eles, o nome do app e "© empresa".
#[test]
fn the_version_info_takes_the_description_and_the_copyright_of_the_project() {
    let info = VersionInfo::of_value(&serde_json::json!({
        "name": "Loja",
        "publish": { "version": "3.1.0", "publisher": "Empresa Ltda", "description": "Ponto de venda", "copyright": "© 2026 Empresa Ltda. Todos os direitos reservados." }
    }));
    assert_eq!(info.description, "Ponto de venda");
    assert_eq!(info.copyright, "© 2026 Empresa Ltda. Todos os direitos reservados.");
    let (_, strings) = read_version_data(&version_data(&info)).unwrap();
    let get = |k: &str| strings.iter().find(|(key, _)| key == k).map(|(_, v)| v.clone());
    assert_eq!(get("FileDescription").as_deref(), Some("Ponto de venda"));
    assert_eq!(get("LegalCopyright").as_deref(), Some("© 2026 Empresa Ltda. Todos os direitos reservados."));
    assert_eq!(get("ProductName").as_deref(), Some("Loja"), "o produto continua o nome do app");
    // Em branco vale como vazio.
    let info = VersionInfo::of_value(&serde_json::json!({ "name": "Loja", "publish": { "publisher": "ACME", "description": "  ", "copyright": "" } }));
    assert_eq!((info.description.as_str(), info.copyright.as_str()), ("Loja", "© ACME"));
}

/// O `git` com os argumentos separados, sem configuração da máquina (nome, assinatura).
fn git(dir: &std::path::Path, args: &[&str]) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.name=Aylis", "-c", "user.email=teste@aylis.invalid", "-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success())
}

/// A versão da última tag do git (`git describe --tags --abbrev=0`, sem o `v`), e o motivo
/// quando não há: sem repositório, sem tag, tag que não é versão.
#[test]
fn the_version_comes_from_the_latest_git_tag() {
    if std::process::Command::new("git").arg("--version").output().is_err() {
        eprintln!("sem o git: pulado");
        return;
    }
    let dir = std::env::temp_dir().join(format!("aylis_build_git_tag_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    // A pasta temporária fora de um repositório (o `GIT_CEILING_DIRECTORIES` não vale para o
    // `-C`, então só confere quando o temp não está dentro de um).
    if !git(&dir, &["rev-parse", "--is-inside-work-tree"]) {
        assert_eq!(git_tag(&dir), Err(GitTagError::NotRepository));
    }
    assert!(git(&dir, &["init", "-q"]));
    std::fs::write(dir.join("a.txt"), "1").unwrap();
    assert!(git(&dir, &["add", "a.txt"]));
    assert!(git(&dir, &["commit", "-q", "-m", "um"]));
    assert_eq!(git_tag(&dir), Err(GitTagError::NoTag));
    assert!(git(&dir, &["tag", "v1.4.0"]));
    assert_eq!(git_tag(&dir), Ok("1.4.0".to_string()));
    // A última alcançável do commit atual, e com pré-release.
    std::fs::write(dir.join("a.txt"), "2").unwrap();
    assert!(git(&dir, &["commit", "-q", "-am", "dois"]));
    assert!(git(&dir, &["tag", "-a", "2.0.0-beta.1", "-m", "beta"]));
    assert_eq!(git_tag(&dir), Ok("2.0.0-beta.1".to_string()));
    // O build: o `.aylisproj` embutido e os dados de versão saem com a versão da tag; sem a
    // opção, o projeto fica como está.
    let project = dir.join("App.aylisproj");
    std::fs::write(&project, r#"{"aylis_project": 1, "name": "App", "publish": {"version": "1.0.0", "version_from_git_tag": true}}"#).unwrap();
    let tagged = tagged_project(&project).expect("a versão da tag");
    assert_eq!(tagged["publish"]["version"], "2.0.0-beta.1");
    assert_eq!(VersionInfo::of_value(&tagged).version, "2.0.0-beta.1");
    std::fs::write(&project, r#"{"aylis_project": 1, "name": "App", "publish": {"version": "1.0.0"}}"#).unwrap();
    assert!(tagged_project(&project).is_none());
    std::fs::remove_file(&project).unwrap();
    std::fs::write(dir.join("a.txt"), "3").unwrap();
    assert!(git(&dir, &["commit", "-q", "-am", "três"]));
    assert!(git(&dir, &["tag", "release-final"]));
    assert_eq!(git_tag(&dir), Err(GitTagError::Invalid("release-final".into())));
    let _ = std::fs::remove_dir_all(&dir);
}
