//! Os pacotes sem rede externa: pastas locais, um servidor HTTP local fazendo de espelho e de
//! Google Drive, hashes errados, a troca atômica, as dependências e o remove.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

use serde_json::json;
use sha2::{Digest, Sha256};

use super::*;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_toolpacks_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut buf);
    for (name, data) in entries {
        zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(data).unwrap();
    }
    zip.finish().unwrap();
    buf.into_inner()
}

fn tar_gz_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    let mut tar = tar::Builder::new(gz);
    for (name, data) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(&mut header, name, *data).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

/// Um arquivo do manifesto de teste.
fn file_entry(name: &str, bytes: &[u8], extra: serde_json::Value) -> serde_json::Value {
    let mut v = json!({ "name": name, "size": bytes.len(), "sha256": sha(bytes) });
    if let Some(m) = extra.as_object() {
        for (k, x) in m {
            v[k] = x.clone();
        }
    }
    v
}

fn manifest(packages: serde_json::Value) -> Manifest {
    Manifest::parse(&json!({ "aylis": 1, "packages": packages }).to_string()).unwrap()
}

fn installer(dir: &Path, manifest: Manifest, sources: Sources) -> Installer {
    Installer { manifest, paths: Paths { tools: dir.join("tools"), cache: dir.join("cache") }, sources }
}

fn quiet() -> impl FnMut(Progress) {
    |_| {}
}

type Routes = HashMap<String, (&'static str, Vec<u8>)>;

/// Um servidor HTTP local: `routes` monta, a partir do endereço, o tipo e o corpo de cada
/// caminho (com a consulta). Devolve o endereço e os caminhos pedidos.
fn serve(routes: impl FnOnce(&str) -> Routes) -> (String, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let routes = routes(&base);
    let asked = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let log = asked.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() {
                continue;
            }
            loop {
                let mut h = String::new();
                if reader.read_line(&mut h).is_err() || h.trim().is_empty() {
                    break;
                }
            }
            let path = line.split_whitespace().nth(1).unwrap_or("/").to_string();
            log.lock().unwrap().push(path.clone());
            let (status, kind, body) = match routes.get(&path) {
                Some((kind, body)) => ("200 OK", *kind, body.clone()),
                None => ("404 Not Found", "text/plain", b"nao".to_vec()),
            };
            let head = format!("HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&body);
        }
    });
    (base, asked)
}

#[test]
fn the_embedded_manifest_is_valid_and_complete() {
    let m = Manifest::embedded();
    let mut names: HashMap<String, String> = HashMap::new();
    for name in m.names() {
        let raw = m.packages.iter().find(|p| p.name == name).unwrap();
        for e in &raw.platforms {
            for f in &e.files {
                assert!(is_sha256(&f.sha256), "{name} ({}): {} sem SHA-256 (aylis-tool packages update)", e.key, f.name);
                assert!(f.size > 0, "{name}: {} sem tamanho", f.name);
                assert!(f.url.as_deref().is_some_and(|u| u.starts_with("https://")), "{name}: {} sem URL oficial", f.name);
                // A pasta de pacotes guarda pelo nome: um nome, um arquivo.
                if let Some(other) = names.insert(f.name.clone(), f.sha256.clone()) {
                    assert_eq!(other, f.sha256, "{} com dois conteúdos", f.name);
                }
            }
        }
    }
    for wanted in ["rust", "rust-android", "rust-wasm", "jdk", "android-sdk", "android-ndk", "cargo-ndk", "wasm-bindgen", "python", "node", "dotnet", "go", "nsis", "codelldb", "netcoredbg", "js-debug", "clangd", "bundletool", "jdtls"] {
        let p = m.package_for(wanted, "windows-x86_64").unwrap_or_else(|| panic!("{wanted}"));
        assert!(p.available(), "{wanted} no Windows");
    }
    assert_eq!(m.with_dependencies(&["cargo-ndk"]).unwrap(), ["rust", "rust-android", "android-sdk", "android-ndk", "cargo-ndk"]);
}

#[test]
fn names_and_paths_with_folders_are_refused() {
    assert!(plain_name("clangd-windows-23.1.0.zip") && plain_name("OpenJDK17U-jdk_x64_windows_hotspot_17.0.21_9.zip"));
    for bad in ["../x.zip", "a/b.zip", r"a\b.zip", ".hidden", ""] {
        assert!(!plain_name(bad), "{bad}");
    }
    for bad in ["../x", "/abs", "C:/x", r"a\..\..\b"] {
        assert!(safe_relative(bad).is_err(), "{bad}");
    }
    let bad = json!({ "x": { "version": "1", "install": "extract", "platforms": { "any": { "files": [{ "name": "a.zip", "sha256": "", "into": "../fora" }] } } } });
    assert!(Manifest::parse(&json!({ "packages": bad }).to_string()).is_err());
    let script = json!({ "x": { "version": "1", "install": "run-script", "platforms": {} } });
    assert!(Manifest::parse(&json!({ "packages": script }).to_string()).unwrap_err().contains("desconhecido"));
}

#[test]
fn archive_entries_outside_the_folder_are_refused() {
    let dir = temp("slip");
    std::fs::write(dir.join("bad.zip"), zip_bytes(&[("../fora.txt", b"x")])).unwrap();
    let err = extract(&dir.join("bad.zip"), &dir.join("out"), 0, None, &Control::default()).unwrap_err();
    assert!(err.contains("fora da pasta"), "{err}");
    assert!(!dir.join("fora.txt").exists());
    // No tar, escrito à mão: o `tar::Builder` não aceita `..`.
    let mut header = tar::Header::new_gnu();
    header.as_gnu_mut().unwrap().name[..12].copy_from_slice(b"../fora2.txt");
    header.set_size(1);
    header.set_cksum();
    let mut raw = header.as_bytes().to_vec();
    raw.extend_from_slice(&[b'x']);
    raw.resize(512 * 2, 0);
    raw.extend_from_slice(&[0; 1024]);
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gz.write_all(&raw).unwrap();
    std::fs::write(dir.join("bad.tar.gz"), gz.finish().unwrap()).unwrap();
    let err = extract(&dir.join("bad.tar.gz"), &dir.join("out2"), 0, None, &Control::default()).unwrap_err();
    assert!(err.contains("fora da pasta"), "{err}");
    assert!(!dir.join("fora2.txt").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn strip_and_prefix_choose_what_is_extracted() {
    let dir = temp("strip");
    std::fs::write(dir.join("a.zip"), zip_bytes(&[("jdk-1/bin/java.exe", b"j"), ("jdk-1/release", b"r"), ("leia.txt", b"l")])).unwrap();
    extract(&dir.join("a.zip"), &dir.join("s"), 1, None, &Control::default()).unwrap();
    assert!(dir.join("s/bin/java.exe").is_file() && dir.join("s/release").is_file());
    assert!(!dir.join("s/leia.txt").exists(), "a entrada do nível de cima fica de fora");
    std::fs::write(dir.join("p.nupkg"), zip_bytes(&[("tools/python.exe", b"p"), ("tools/Lib/os.py", b"o"), ("python.nuspec", b"n")])).unwrap();
    extract(&dir.join("p.nupkg"), &dir.join("p"), 0, Some("tools"), &Control::default()).unwrap();
    assert!(dir.join("p/python.exe").is_file() && dir.join("p/Lib/os.py").is_file());
    assert!(!dir.join("p/python.nuspec").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Um pacote `tool` com um zip na pasta `pkgs`.
fn tool_package(version: &str, zip: &[u8]) -> serde_json::Value {
    json!({
        "tool": {
            "title": "Ferramenta", "version": version, "install": "extract",
            "exports": { "path": ["bin"], "env": { "AYLIS_TEST_TOOL_HOME": "." } },
            "platforms": { "any": { "files": [file_entry(&format!("tool-{version}.zip"), zip, json!({ "strip": 1 }))] } }
        }
    })
}

#[test]
fn installs_from_a_local_folder_and_registers() {
    let dir = temp("folder");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    std::fs::write(dir.join("pkgs/tool-1.0.zip"), &zip).unwrap();
    let i = installer(&dir, manifest(tool_package("1.0", &zip)), Sources::local(vec![dir.join("nao-existe"), dir.join("pkgs")]));
    let mut steps = Vec::new();
    let done = i.install(&["tool"], &Control::default(), &mut |p| steps.push(p)).unwrap();
    assert_eq!(done.len(), 1);
    let home = dir.join("tools/tool/1.0");
    assert_eq!(std::fs::read(home.join("bin/tool.exe")).unwrap(), b"ok");
    assert!(dir.join("cache/tool-1.0.zip").is_file(), "fica no cache");
    assert!(steps.iter().any(|s| matches!(s, Progress::Bytes { from, .. } if from.contains("pkgs"))), "o progresso diz de onde: {steps:?}");
    assert_eq!(i.installed()[0].version, "1.0");
    let env = i.environment();
    assert_eq!(env.path, [home.join("bin")]);
    assert_eq!(env.vars, [("AYLIS_TEST_TOOL_HOME".to_string(), home.clone())]);
    // De novo: já está, nada a fazer (e sem a pasta de pacotes, o cache basta).
    let again = installer(&dir, manifest(tool_package("1.0", &zip)), Sources::default());
    assert!(again.install(&["tool"], &Control::default(), &mut quiet()).unwrap().is_empty());
    std::fs::remove_dir_all(dir.join("tools")).unwrap();
    again.install(&["tool"], &Control::default(), &mut quiet()).unwrap();
    assert!(home.join("bin/tool.exe").is_file(), "reinstalado do cache, sem origem");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_file_with_the_wrong_hash_is_refused() {
    let dir = temp("hash");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    std::fs::write(dir.join("pkgs/tool-1.0.zip"), zip_bytes(&[("tool-1/bin/tool.exe", b"ko")])).unwrap();
    let i = installer(&dir, manifest(tool_package("1.0", &zip)), Sources::local(vec![dir.join("pkgs")]));
    let err = i.install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.contains("tool-1.0.zip") && err.contains("pkgs") && err.contains("não confere"), "{err}");
    assert!(!dir.join("cache/tool-1.0.zip").exists() && !dir.join("tools/tool/1.0").exists());
    // Sem origem nenhuma, o erro diz o que fazer.
    let none = installer(&dir, manifest(tool_package("1.0", &zip)), Sources::default());
    let err = none.install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.contains("Nenhuma origem configurada"), "{err}");
    // Um manifesto sem hash não instala.
    let mut no_hash = tool_package("1.0", &zip);
    no_hash["tool"]["platforms"]["any"]["files"][0]["sha256"] = json!("");
    let err = installer(&dir, manifest(no_hash), Sources::local(vec![dir.join("pkgs")])).install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.contains("packages update"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// O erro começa pela causa (sem internet, os arquivos não estão ao lado do instalador, o
/// SHA-256 não confere), e `causes` a acha para o código de saída do `aylis-tool setup`.
#[test]
fn the_error_says_exactly_what_went_wrong() {
    let dir = temp("causes");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    // Sem a pasta pacotes ao lado do instalador e sem internet (um espelho que não conecta).
    let beside = dir.join("instalador/pacotes");
    let offline = Sources {
        folders: vec![beside.clone()],
        installer_folder: Some(beside.clone()),
        mirrors: vec!["http://127.0.0.1:1/pacotes.json".into()],
        ..Sources::default()
    };
    let err = installer(&dir, manifest(tool_package("1.0", &zip)), offline.clone()).install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    let first = err.lines().next().unwrap();
    assert!(first.contains(fetch::SAYS_OFFLINE) && first.contains(fetch::SAYS_NOT_BESIDE) && first.contains("não há a pasta pacotes"), "{err}");
    assert!(!first.contains(fetch::SAYS_MISMATCH), "{err}");
    assert_eq!(causes(&err), CAUSE_OFFLINE | CAUSE_NOT_BESIDE);
    // A pasta existe, mas sem o arquivo.
    std::fs::create_dir_all(&beside).unwrap();
    let err = installer(&dir, manifest(tool_package("1.0", &zip)), offline).install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.lines().next().unwrap().contains("falta tool-1.0.zip"), "{err}");
    // O arquivo ao lado do instalador, mas com outro conteúdo: só o hash.
    std::fs::write(beside.join("tool-1.0.zip"), zip_bytes(&[("tool-1/bin/tool.exe", b"ko")])).unwrap();
    let local = Sources { folders: vec![beside.clone()], installer_folder: Some(beside.clone()), ..Sources::default() };
    let err = installer(&dir, manifest(tool_package("1.0", &zip)), local).install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert_eq!(causes(&err), CAUSE_MISMATCH, "{err}");
    assert!(err.lines().next().unwrap().contains("corrompido ou é de outra versão"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_failed_install_keeps_the_previous_version() {
    let dir = temp("atomic");
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    let good = zip_bytes(&[("tool-1/bin/tool.exe", b"v1")]);
    std::fs::write(dir.join("pkgs/tool-1.0.zip"), &good).unwrap();
    let mut v1 = tool_package("1.0", &good);
    v1["tool"]["platforms"]["any"]["files"][0]["name"] = json!("tool-1.0.zip");
    installer(&dir, manifest(v1), Sources::local(vec![dir.join("pkgs")])).install(&["tool"], &Control::default(), &mut quiet()).unwrap();
    // A 2.0 tem o hash certo, mas não é um zip: a extração falha no meio.
    let broken = b"PK\x03\x04 isto nao e um zip".to_vec();
    std::fs::write(dir.join("pkgs/tool-2.0.zip"), &broken).unwrap();
    let mut v2 = tool_package("2.0", &broken);
    v2["tool"]["platforms"]["any"]["files"][0]["name"] = json!("tool-2.0.zip");
    let i = installer(&dir, manifest(v2), Sources::local(vec![dir.join("pkgs")]));
    assert!(i.install(&["tool"], &Control::default(), &mut quiet()).is_err());
    assert_eq!(std::fs::read(dir.join("tools/tool/1.0/bin/tool.exe")).unwrap(), b"v1", "a anterior continua");
    assert_eq!(i.installed()[0].version, "1.0");
    let leftovers: Vec<_> = std::fs::read_dir(dir.join("tools/tool")).unwrap().flatten().map(|e| e.file_name()).collect();
    assert_eq!(leftovers, ["1.0"], "sem pasta temporária esquecida");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn dependencies_install_first_and_block_the_remove() {
    let dir = temp("deps");
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    let a = zip_bytes(&[("a.txt", b"a")]);
    let b = zip_bytes(&[("b.txt", b"b")]);
    std::fs::write(dir.join("pkgs/a.zip"), &a).unwrap();
    std::fs::write(dir.join("pkgs/b.zip"), &b).unwrap();
    let m = manifest(json!({
        "base": { "version": "1", "install": "extract", "platforms": { "any": { "files": [file_entry("a.zip", &a, json!({}))] } } },
        "top": { "version": "2", "depends": ["base"], "install": "extract", "platforms": { "any": { "files": [file_entry("b.zip", &b, json!({}))] } } }
    }));
    let i = installer(&dir, m, Sources::local(vec![dir.join("pkgs")]));
    let done = i.install(&["top"], &Control::default(), &mut quiet()).unwrap();
    assert_eq!(done.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["base", "top"]);
    let err = i.remove("base", &mut quiet()).unwrap_err();
    assert!(err.contains("top"), "{err}");
    i.remove("top", &mut quiet()).unwrap();
    assert!(!dir.join("tools/top").exists());
    i.remove("base", &mut quiet()).unwrap();
    assert!(i.installed().is_empty() && !dir.join("tools/base").exists());
    assert!(i.remove("base", &mut quiet()).is_err(), "não instalado");
    let cycle = Manifest::parse(&json!({ "packages": {
        "x": { "version": "1", "depends": ["y"], "install": "extract", "platforms": {} },
        "y": { "version": "1", "depends": ["x"], "install": "extract", "platforms": {} }
    }}).to_string()).unwrap();
    assert!(cycle.with_dependencies(&["x"]).unwrap_err().contains("circular"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// Um tarball de componente do Rust como os oficiais.
fn rust_component(top: &str, component: &str, files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut entries: Vec<(String, Vec<u8>)> = vec![(format!("{top}/components"), format!("{component}\n").into_bytes()), (format!("{top}/install.sh"), b"#!/bin/sh\nexit 1\n".to_vec())];
    entries.push((format!("{top}/{component}/manifest.in"), b"file:x\n".to_vec()));
    for (path, data) in files {
        entries.push((format!("{top}/{component}/{path}"), data.to_vec()));
    }
    let refs: Vec<(&str, &[u8])> = entries.iter().map(|(n, d)| (n.as_str(), d.as_slice())).collect();
    tar_gz_bytes(&refs)
}

#[test]
fn the_rust_toolchain_and_its_targets() {
    let dir = temp("rust");
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    let rustc = rust_component("rustc-1-host", "rustc", &[("bin/rustc.exe", b"rustc"), ("lib/rustlib/etc/x", b"e")]);
    let std_host = rust_component("rust-std-1-host", "rust-std-host", &[("lib/rustlib/host/lib/libstd.rlib", b"s")]);
    let std_wasm = rust_component("rust-std-1-wasm32", "rust-std-wasm32", &[("lib/rustlib/wasm32/lib/libstd.rlib", b"w")]);
    for (n, b) in [("rustc-1.tar.gz", &rustc), ("std-host-1.tar.gz", &std_host), ("std-wasm-1.tar.gz", &std_wasm)] {
        std::fs::write(dir.join("pkgs").join(n), b).unwrap();
    }
    let m = manifest(json!({
        "rust": { "version": "1", "install": "rust-toolchain", "exports": { "path": ["bin"] }, "platforms": { "any": { "files": [
            file_entry("rustc-1.tar.gz", &rustc, json!({})), file_entry("std-host-1.tar.gz", &std_host, json!({}))
        ] } } },
        "rust-wasm": { "version": "1", "depends": ["rust"], "install": { "rust-target": ["wasm32"] }, "platforms": { "any": { "files": [
            file_entry("std-wasm-1.tar.gz", &std_wasm, json!({}))
        ] } } }
    }));
    let i = installer(&dir, m, Sources::local(vec![dir.join("pkgs")]));
    i.install(&["rust-wasm"], &Control::default(), &mut quiet()).unwrap();
    let home = dir.join("tools/rust/1");
    assert!(home.join("bin/rustc.exe").is_file() && home.join("lib/rustlib/host/lib/libstd.rlib").is_file());
    assert!(home.join("lib/rustlib/wasm32/lib/libstd.rlib").is_file(), "o alvo entra no toolchain");
    assert!(!home.join("manifest.in").exists() && !home.join("install.sh").exists(), "sem os arquivos do instalador");
    assert_eq!(i.installed_dir("rust-wasm"), Some(home.clone()));
    // Reinstalar o toolchain leva o alvo junto.
    std::fs::remove_dir_all(&home).unwrap();
    let mut steps = Vec::new();
    i.install(&["rust"], &Control::default(), &mut |p| steps.push(p)).unwrap();
    assert!(home.join("lib/rustlib/wasm32/lib/libstd.rlib").is_file(), "{steps:?}");
    i.remove("rust-wasm", &mut quiet()).unwrap();
    assert!(!home.join("lib/rustlib/wasm32").exists() && home.join("lib/rustlib/host").is_dir());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_mirror_index_with_relative_urls_and_the_drive_confirmation() {
    let dir = temp("mirror");
    let small = zip_bytes(&[("tool-1/bin/tool.exe", b"pelo espelho")]);
    let big = zip_bytes(&[("x.txt", b"pelo drive")]);
    let mut routes = HashMap::new();
    routes.insert("/pacotes/tool-1.0.zip".to_string(), ("application/zip", small.clone()));
    // O Drive: a primeira ida responde a página de "não deu para verificar vírus"; a segunda,
    // com o confirm e o uuid do formulário, o arquivo.
    let form = r#"<!DOCTYPE html><html><body>Google Drive can't scan this file for viruses.
        <form id="download-form" action="ACTION" method="get">
        <input type="submit" value="Download anyway"/>
        <input type="hidden" name="id" value="ID-123_abc"><input type="hidden" name="export" value="download">
        <input type="hidden" name="confirm" value="t"><input type="hidden" name="uuid" value="u-1"></form></body></html>"#;
    let (base, asked) = serve(|base| {
        let mut r = routes.clone();
        r.insert("/download?id=ID-123_abc&export=download".into(), ("text/html; charset=utf-8", form.replace("ACTION", &format!("{base}/download")).into_bytes()));
        r.insert("/download?id=ID-123_abc&export=download&confirm=t&uuid=u-1".into(), ("application/octet-stream", big.clone()));
        r.insert(
            "/pacotes/pacotes.json".into(),
            ("application/json", serde_json::to_vec(&json!({ "aylis": 1, "files": { "tool-1.0.zip": { "url": "tool-1.0.zip" }, "big.zip": { "drive": "ID-123_abc" } } })).unwrap()),
        );
        r
    });
    let m = manifest(json!({
        "tool": tool_package("1.0", &small)["tool"],
        "big": { "version": "1", "install": "extract", "platforms": { "any": { "files": [file_entry("big.zip", &big, json!({}))] } } }
    }));
    let mut sources = Sources { mirrors: vec![format!("{base}/pacotes/pacotes.json")], ..Sources::default() };
    sources.drive = format!("{base}/download");
    let i = installer(&dir, m, sources);
    i.install(&["tool", "big"], &Control::default(), &mut quiet()).unwrap();
    assert_eq!(std::fs::read(dir.join("tools/tool/1.0/bin/tool.exe")).unwrap(), b"pelo espelho");
    assert_eq!(std::fs::read(dir.join("tools/big/1/x.txt")).unwrap(), b"pelo drive");
    let asked = asked.lock().unwrap().clone();
    assert_eq!(asked.iter().filter(|p| p.ends_with("pacotes.json")).count(), 1, "o índice é lido uma vez: {asked:?}");
    assert!(asked.iter().any(|p| p.contains("confirm=t")), "{asked:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_mirror_that_sends_too_much_or_the_wrong_file_is_refused() {
    let dir = temp("mirror_bad");
    let right = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    let mut wrong = right.clone();
    wrong.extend(std::iter::repeat_n(0u8, 1 << 20));
    let (base, _) = serve(|_| {
        let mut r = HashMap::new();
        r.insert("/p/tool-1.0.zip".to_string(), ("application/zip", wrong.clone()));
        r.insert("/p/pacotes.json".to_string(), ("application/json", br#"{"files": {"tool-1.0.zip": {"url": "tool-1.0.zip"}}}"#.to_vec()));
        r
    });
    let i = installer(&dir, manifest(tool_package("1.0", &right)), Sources { mirrors: vec![format!("{base}/p/pacotes.json")], ..Sources::default() });
    let err = i.install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.contains("espelho") && err.contains("maior que"), "{err}");
    assert!(!dir.join("cache").read_dir().unwrap().any(|_| true), "nada fica no cache");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn reads_the_drive_confirmation_form() {
    let page = r#"<form id="download-form" action="https://drive.usercontent.google.com/download" method="get"><input type="hidden" name="id" value="abc"><input type="hidden" name="confirm" value="t"><input type="hidden" name="uuid" value="9f&amp;x"></form>"#;
    let (action, fields) = fetch::drive_form(page).unwrap();
    assert_eq!(action, "https://drive.usercontent.google.com/download");
    assert_eq!(fields, [("id".into(), "abc".into()), ("confirm".into(), "t".into()), ("uuid".into(), "9f&x".into())]);
    assert!(fetch::drive_form("<html>sem formulário</html>").is_none());
    assert!(fetch::allowed_url("https://x") && fetch::allowed_url("http://127.0.0.1:8080/a") && !fetch::allowed_url("http://exemplo.com/a"));
}

#[test]
fn a_cancelled_install_stops_and_leaves_nothing() {
    let dir = temp("cancel");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    std::fs::write(dir.join("pkgs/tool-1.0.zip"), &zip).unwrap();
    let i = installer(&dir, manifest(tool_package("1.0", &zip)), Sources::local(vec![dir.join("pkgs")]));
    let control = Control::default();
    control.cancel();
    assert_eq!(i.install(&["tool"], &control, &mut quiet()).unwrap_err(), crate::process::CANCELLED);
    assert!(i.installed().is_empty() && !dir.join("tools/tool/1.0").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn pack_writes_the_folder_and_the_index() {
    let dir = temp("pack");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"ok")]);
    std::fs::create_dir_all(dir.join("cache")).unwrap();
    std::fs::write(dir.join("cache/tool-1.0.zip"), &zip).unwrap();
    let m = manifest(tool_package("1.0", &zip));
    let report = pack(&m, &dir.join("cache"), &dir.join("out"), &[], &[], &Control::default(), &mut quiet()).unwrap();
    assert_eq!(report.files.len(), 1);
    assert_eq!(report.files[0].4, "cache");
    assert_eq!(std::fs::read(dir.join("out/tool-1.0.zip")).unwrap(), zip);
    let index: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.join("out/pacotes.json")).unwrap()).unwrap();
    assert_eq!(index["files"]["tool-1.0.zip"]["url"], "tool-1.0.zip");
    // A pasta gravada serve de pasta de pacotes.
    let i = installer(&dir.join("other"), m, Sources::local(vec![dir.join("out")]));
    i.install(&["tool"], &Control::default(), &mut quiet()).unwrap();
    assert!(dir.join("other/tools/tool/1.0/bin/tool.exe").is_file());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Sem origem, nada cai na URL oficial do manifesto (decisão do autor): a instalação falha
/// dizendo que não há origem, e o servidor da "URL oficial" não recebe pedido nenhum.
#[test]
fn nothing_falls_back_to_the_official_url() {
    let dir = temp("official");
    let zip = zip_bytes(&[("tool-1/bin/tool.exe", b"oficial")]);
    let (base, asked) = serve(|_| {
        let mut r = HashMap::new();
        r.insert("/oficial/tool-1.0.zip".to_string(), ("application/zip", zip.clone()));
        r
    });
    // O manifesto recusa URL oficial que não seja https: a do servidor local entra depois.
    let mut m = manifest(tool_package("1.0", &zip));
    for f in m.packages.iter_mut().flat_map(|p| p.platforms.iter_mut()).flat_map(|e| e.files.iter_mut()) {
        f.url = Some(format!("{base}/oficial/tool-1.0.zip"));
    }
    let i = installer(&dir, m, Sources::default());
    assert!(!i.sources.has_any());
    let err = i.install(&["tool"], &Control::default(), &mut quiet()).unwrap_err();
    assert!(err.contains("Nenhuma origem configurada"), "{err}");
    assert!(asked.lock().unwrap().is_empty(), "a URL oficial não é usada: {:?}", asked.lock().unwrap());
    assert!(!dir.join("tools/tool").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// O "Testar as origens": quantos arquivos cada pasta e cada espelho tem.
#[test]
fn probing_counts_the_files_of_each_source() {
    let dir = temp("probe");
    let a = zip_bytes(&[("a.txt", b"a")]);
    let b = zip_bytes(&[("b.txt", b"b")]);
    let m = manifest(json!({
        "a": { "version": "1", "install": "extract", "platforms": { "any": { "files": [file_entry("a.zip", &a, json!({}))] } } },
        "b": { "version": "1", "install": "extract", "platforms": { "any": { "files": [file_entry("b.zip", &b, json!({}))] } } }
    }));
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    std::fs::write(dir.join("pkgs/a.zip"), &a).unwrap();
    // O tamanho errado não conta.
    std::fs::write(dir.join("pkgs/b.zip"), b"curto").unwrap();
    let (base, _) = serve(|_| {
        let mut r = HashMap::new();
        r.insert("/p/pacotes.json".to_string(), ("application/json", br#"{"files": {"a.zip": {"url": "a.zip"}, "b.zip": {"drive": "x1"}, "c.zip": {"url": "c.zip"}}}"#.to_vec()));
        r
    });
    let files: Vec<FileSpec> = m.packages().into_iter().flat_map(|p| p.files).collect();
    let sources = Sources { folders: vec![dir.join("pkgs"), dir.join("nao-existe")], mirrors: vec![format!("{base}/p/pacotes.json"), format!("{base}/sem/pacotes.json")], ..Sources::default() };
    let probes = sources.probe(&files, &Control::default());
    let summary: Vec<(bool, usize, usize, bool)> = probes.iter().map(|p| (p.mirror, p.found, p.total, p.error.is_some())).collect();
    assert_eq!(summary, [(false, 1, 2, false), (false, 0, 2, true), (true, 2, 2, false), (true, 0, 2, true)], "{probes:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// As dependências que a máquina já tem ficam de fora (o JDK do bundletool).
#[test]
fn installing_except_skips_the_dependencies_the_system_has() {
    let dir = temp("except");
    let lib = zip_bytes(&[("lib-1/x.txt", b"lib")]);
    let app = zip_bytes(&[("app-1/y.txt", b"app")]);
    let m = manifest(json!({
        "lib": { "version": "1", "install": "extract", "platforms": { "any": { "files": [file_entry("lib.zip", &lib, json!({ "strip": 1 }))] } } },
        "app": { "version": "1", "install": "extract", "depends": ["lib"], "platforms": { "any": { "files": [file_entry("app.zip", &app, json!({ "strip": 1 }))] } } }
    }));
    std::fs::create_dir_all(dir.join("pkgs")).unwrap();
    std::fs::write(dir.join("pkgs/app.zip"), &app).unwrap();
    let i = installer(&dir, m, Sources::local(vec![dir.join("pkgs")]));
    let done = i.install_except(&["app"], &["lib".to_string()], &Control::default(), &mut quiet()).unwrap();
    assert_eq!(done.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["app"]);
    assert!(!dir.join("tools/lib").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// O link de uma pasta compartilhada do Drive serve de espelho: a vista embutida dela dá o nome
/// e o id de cada arquivo (o trecho abaixo tem o formato real da página).
#[test]
fn a_drive_folder_link_becomes_the_mirror_index() {
    use super::fetch::{drive_folder_id, drive_folder_index};
    assert_eq!(drive_folder_id("https://drive.google.com/drive/folders/128gDbpq_v-Z?usp=sharing").as_deref(), Some("128gDbpq_v-Z"));
    assert_eq!(drive_folder_id("https://drive.google.com/drive/u/0/folders/abc123").as_deref(), Some("abc123"));
    assert_eq!(drive_folder_id("https://exemplo.com/pacotes.json"), None);
    assert_eq!(drive_folder_id("https://drive.google.com/file/d/abc/view"), None);
    let entry = |id: &str, name: &str| {
        format!(
            r#"<div class="flip-entry" id="entry-{id}" tabindex="0" role="link"><div class="flip-entry-info"><a href="https://drive.google.com/file/d/{id}/view?usp=drive_web" target="_blank"><div class="flip-entry-visual"><div class="flip-entry-visual-card"><div class="flip-entry-icon"><img src="x" alt="Compressed Archive"/></div></div></div><div class="flip-entry-list-icon"><img src="x" alt=""/></div><div class="flip-entry-title">{name}</div></a></div></div>"#
        )
    };
    let page = format!(
        "<html><body>{}{}<a href=\"https://drive.google.com/drive/folders/sub\"><div class=\"flip-entry-title\">sub</div></a>{}</body></html>",
        entry("1RQQd9DYrBXWglRlMmq89wqq9_dFx9G3R", "android-ndk-r27c-windows.zip"),
        entry("1uqXVOb6tpJdiwiRiYLKifCzbe9BbTxxL", "node-v26.13.1-win-x64.zip"),
        entry("1abc-_", "a &amp; b.zip"),
    );
    let index = drive_folder_index(&page);
    let files = index["files"].as_object().unwrap();
    assert_eq!(files.len(), 3, "{index}");
    assert_eq!(files["android-ndk-r27c-windows.zip"]["drive"], "1RQQd9DYrBXWglRlMmq89wqq9_dFx9G3R");
    assert_eq!(files["node-v26.13.1-win-x64.zip"]["drive"], "1uqXVOb6tpJdiwiRiYLKifCzbe9BbTxxL");
    assert_eq!(files["a & b.zip"]["drive"], "1abc-_");
}
