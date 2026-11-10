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
