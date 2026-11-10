//! Os clientes das outras linguagens (`clients/`): cada demo carrega a tela, simula o
//! clique, o handler lê o campo e responde; o teste confere a resposta. Uma linguagem sem
//! compilador na máquina é pulada (com aviso).
//!
//! As ferramentas são achadas como o aylis acha (`aylis_tools::hostlang::find`: o caminho das
//! Opções da IDE, `aylis_tools::settings`, o PATH ou a pasta padrão da instalação); o C++ usa o
//! `vcvars` das Opções (vcvars64.bat do Visual C++) ou `g++` no PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().unwrap()
}

fn tool(key: &str, name: &str) -> Option<PathBuf> {
    if let Some(p) = aylis_tools::settings::tool(key) {
        return Some(p);
    }
    Command::new(name).arg("--version").output().ok().filter(|o| o.status.success()).map(|_| PathBuf::from(name))
}

/// Como o aylis acha a ferramenta da linguagem (variável, PATH ou a pasta padrão da
/// instalação): `go` e `fpc` não têm `--version`.
fn toolchain(language: aylis_project::Language) -> Option<aylis_tools::hostlang::Toolchain> {
    aylis_tools::hostlang::find(language).ok()
}

/// Roda o backend pelo aylis-host (sem janela) e devolve o que ele escreveu no stderr.
fn selftest(backend: &[&str], envs: &[(&str, &Path)]) -> String {
    let png = std::env::temp_dir().join(format!("aylis_lang_{}_{}.png", std::process::id(), backend[0].replace(['/', '\\', ':', '.'], "_")));
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_aylis-host"));
    cmd.current_dir(root()).arg("--png").arg(&png).arg("--").args(backend).arg("--selftest");
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let out = cmd.output().unwrap();
    let _ = std::fs::remove_file(png);
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn check(lang: &str, stderr: &str) {
    assert!(stderr.contains(&format!("SELFTEST {lang} Olá, Ana!")), "{lang}: {stderr}");
    // Fechar pelo backend pergunta ao on_close_query, e a tela espera a resposta (item 3).
    assert!(stderr.contains(&format!("SELFTEST {lang} pergunta")), "{lang}: {stderr}");
    // O ajudante do delta (item 16): o filtro por tipo e a chave do mestre passando ao item.
    assert!(stderr.contains(&format!("SELFTEST {lang} delta 2 1 1 501 1")), "{lang}: {stderr}");
}

fn build(cmd: &mut Command) -> bool {
    if cmd.get_current_dir().is_none() {
        cmd.current_dir(root());
    }
    let out = cmd.output().unwrap();
    if !out.status.success() {
        panic!("compilação: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }
    true
}

#[test]
fn typescript_on_node_and_bun() {
    let mut ran = 0;
    if tool("bun", "bun").is_some() {
        check("typescript-bun", &selftest(&["bun", "clients/typescript/demo.ts"], &[]));
        ran += 1;
    }
    // O Node roda TypeScript direto a partir do 22.6 (--experimental-strip-types).
    let node_ok = Command::new("node").arg("--version").output().ok().and_then(|o| {
        let v = String::from_utf8_lossy(&o.stdout).trim().trim_start_matches('v').to_string();
        let mut p = v.split('.').map(|x| x.parse::<u32>().unwrap_or(0));
        Some((p.next()?, p.next()?))
    });
    if node_ok.is_some_and(|(major, minor)| major > 22 || (major == 22 && minor >= 6)) {
        check("typescript-node", &selftest(&["node", "--experimental-strip-types", "--no-warnings", "clients/typescript/demo.ts"], &[]));
        ran += 1;
    }
    if ran == 0 {
        eprintln!("nem Bun nem Node 22.6+: pulado");
    }
}

#[test]
fn java() {
    if tool("javac", "javac").is_none() {
        return eprintln!("javac não encontrado: pulado");
    }
    let out = std::env::temp_dir().join(format!("aylis_java_{}", std::process::id()));
    build(Command::new("javac").args(["-encoding", "UTF-8", "-d"]).arg(&out).args(["clients/java/dev/aylis/Aylis.java", "clients/java/dev/aylis/Session.java", "clients/java/dev/aylis/User.java", "clients/java/dev/aylis/Server.java", "clients/java/Demo.java"]));
    check("java", &selftest(&["java", "-cp", out.to_str().unwrap(), "Demo"], &[]));
    let _ = std::fs::remove_dir_all(out);
}

#[test]
fn go() {
    let Some(go) = toolchain(aylis_project::Language::Go).map(|t| t.program) else { return eprintln!("go não encontrado: pulado") };
    let exe = std::env::temp_dir().join(format!("aylis_go_demo_{}{}", std::process::id(), std::env::consts::EXE_SUFFIX));
    build(Command::new(&go).current_dir(root().join("clients/go")).args(["build", "-o"]).arg(&exe).arg("./demo"));
    check("go", &selftest(&[exe.to_str().unwrap()], &[]));
    let _ = std::fs::remove_file(exe);
}

#[test]
fn csharp() {
    let Some(dotnet) = toolchain(aylis_project::Language::CSharp).map(|t| t.program) else { return eprintln!("dotnet não encontrado: pulado") };
    let out = std::env::temp_dir().join(format!("aylis_cs_{}", std::process::id()));
    build(Command::new(&dotnet).args(["build", "clients/csharp", "-c", "Release", "-v", "q", "-o"]).arg(&out).env("DOTNET_CLI_TELEMETRY_OPTOUT", "1").env("DOTNET_NOLOGO", "1"));
    let dll = out.join("AylisDemo.dll");
    check("csharp", &selftest(&[dotnet.to_str().unwrap(), dll.to_str().unwrap()], &[]));
    let _ = std::fs::remove_dir_all(out);
}

#[test]
fn cpp() {
    let exe = std::env::temp_dir().join(format!("aylis_cpp_demo_{}{}", std::process::id(), std::env::consts::EXE_SUFFIX));
    if let Some(vcvars) = aylis_tools::settings::tool("vcvars") {
        // Um .bat: o cmd não entende as aspas escapadas de um argumento.
        let obj = exe.with_extension("obj");
        let bat = exe.with_extension("bat");
        let script = format!(
            "@call \"{}\" >nul\r\ncl /nologo /std:c++17 /EHsc /utf-8 /O2 clients\\cpp\\demo.cpp /Fe:\"{}\" /Fo:\"{}\"\r\n",
            vcvars.display(),
            exe.display(),
            obj.display()
        );
        std::fs::write(&bat, script).unwrap();
        build(Command::new("cmd").arg("/C").arg(&bat));
        let _ = std::fs::remove_file(obj);
        let _ = std::fs::remove_file(bat);
    } else if tool("gxx", "g++").is_some() {
        build(Command::new("g++").args(["-std=c++17", "-O2", "clients/cpp/demo.cpp", "-o"]).arg(&exe));
    } else {
        return eprintln!("nem Visual C++ (vcvars nas Opções da IDE) nem g++: pulado");
    }
    check("cpp", &selftest(&[exe.to_str().unwrap()], &[]));
    let _ = std::fs::remove_file(exe);
}

#[test]
fn free_pascal() {
    let Some(pascal) = toolchain(aylis_project::Language::Pascal) else { return eprintln!("fpc não encontrado: pulado") };
    let fpc = pascal.program.clone();
    let out = std::env::temp_dir().join(format!("aylis_fpc_{}", std::process::id()));
    std::fs::create_dir_all(&out).unwrap();
    let mut cmd = Command::new(&fpc);
    cmd.args(["-Mobjfpc"]);
    if cfg!(windows) {
        cmd.arg("-Px86_64");
    }
    if let Some(units) = pascal.extra {
        cmd.arg(format!("-Fu{}", units.join("*").display())).arg(format!("-Fu{}", units.join("rtl").display()));
    }
    cmd.arg(format!("-FU{}", out.display())).arg(format!("-FE{}", out.display())).arg("clients/pascal/demo.lpr");
    build(&mut cmd);
    let exe = out.join(format!("demo{}", std::env::consts::EXE_SUFFIX));
    check("pascal", &selftest(&[exe.to_str().unwrap()], &[]));
    let _ = std::fs::remove_dir_all(out);
}
