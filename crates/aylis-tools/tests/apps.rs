//! Os apps de demonstração (`examples/<App>`): a Calculadora, o Paint e o Reprodutor
//! Multimídia do Windows e o painel com os dados abertos do IBGE, cada um com o código em Rust, Python, TypeScript, C#, Java, Go, C++
//! e Free Pascal (uma pasta por linguagem) e um visual só. O mesmo roteiro
//! (`examples/scripts/<app>.json`, tocado sem janela) tem de dar o mesmo estado em todas.
//!
//! Demora (compila cada linguagem) e precisa das ferramentas de cada uma:
//! `cargo test -p aylis-tools --test apps -- --ignored --nocapture`. Uma linguagem sem
//! compilador na máquina é pulada com aviso (as que estão fora do PATH vão na aba da linguagem
//! nas Opções da IDE, `aylis_tools::settings`).

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::{Language, Project};
use aylis_tools::process::Control;

const APPS: [(&str, &str); 4] = [("Calculadora", "calculadora"), ("Paint", "paint"), ("Player", "player"), ("IBGE", "ibge")];

fn root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().map(|p| PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\"))).unwrap()
}

fn project(app: &str) -> Project {
    Project::open(root().join("examples").join(app).join(format!("{app}.aylisproj"))).unwrap()
}

#[test]
fn every_app_has_all_languages_and_nothing_to_port() {
    for (app, _) in APPS {
        let project = project(app);
        assert_eq!(project.code_languages(), Language::ALL.to_vec(), "{app}: uma pasta por linguagem");
        for language in Language::ALL {
            let pending = aylis_project::sync::pending(&project, language).unwrap();
            let lines: Vec<String> = pending.iter().map(|p| p.describe()).collect();
            assert!(pending.is_empty(), "{app} em {}: {}", language.name(), lines.join("\n"));
        }
    }
}

#[test]
#[ignore]
fn the_same_script_gives_the_same_state_in_every_language() {
    // O aylis-host do teste tem de ser o desta versão do código.
    let status = Command::new("cargo").current_dir(root()).args(["build", "-q", "-p", "aylis-host"]).status().unwrap();
    assert!(status.success(), "não compilou o aylis-host");
    let host = root().join("target/debug").join(aylis_tools::env::exe("aylis-host"));
    let output = std::env::temp_dir().join(format!("aylis_apps_{}", std::process::id()));
    std::fs::create_dir_all(&output).unwrap();
    let mut failures = Vec::new();
    for (app, script) in APPS {
        let script = root().join(format!("examples/scripts/{script}.json"));
        let mut reference: Option<String> = None;
        for language in Language::ALL {
            let project = project(app).with_language(language);
            if language != Language::Rust && language != Language::Python && aylis_tools::hostlang::find(language).is_err() {
                eprintln!("{app} em {}: sem as ferramentas da linguagem, pulado", language.name());
                continue;
            }
            let file = output.join(format!("{app}-{}.json", language.folder()));
            let state = aylis_tools::run::script(&project, &script, &file, Some(host.clone()), &Control::new(), &mut |_| {});
            let name = language.name();
            match (state, &reference) {
                (Err(e), _) => failures.push(format!("{app} em {name}: {e}")),
                (Ok(e), None) => {
                    eprintln!("{app} em {name}: referência");
                    reference = Some(e);
                }
                (Ok(e), Some(r)) if &e == r => eprintln!("{app} em {name}: igual"),
                (Ok(e), Some(r)) => failures.push(format!("{app} em {name}: estado diferente\n--- referência\n{r}\n--- {name}\n{e}")),
            }
        }
    }
    let _ = std::fs::remove_dir_all(&output);
    let _ = remove_saved();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// O ProcessamentoAssincrono (só em Free Pascal): o trabalho numa TThread com o modal de progresso. O
/// Send da thread mostra o andamento, o Post fecha o modal no fim e o Cancelar roda enquanto a
/// thread trabalha.
#[test]
fn processamento_assincrono_runs_the_work_in_a_thread() {
    if aylis_tools::hostlang::find(Language::Pascal).is_err() {
        return eprintln!("ProcessamentoAssincrono: sem o Free Pascal, pulado");
    }
    let status = Command::new("cargo").current_dir(root()).args(["build", "-q", "-p", "aylis-host"]).status().unwrap();
    assert!(status.success(), "não compilou o aylis-host");
    let host = root().join("target/debug").join(aylis_tools::env::exe("aylis-host"));
    let output = std::env::temp_dir().join(format!("aylis_processamento_{}", std::process::id()));
    std::fs::create_dir_all(&output).unwrap();
    let project = project("ProcessamentoAssincrono");
    let state = |script: &str| {
        let script = root().join(format!("examples/scripts/{script}.json"));
        aylis_tools::run::script(&project, &script, &output.join("estado.json"), Some(host.clone()), &Control::new(), &mut |_| {}).unwrap()
    };
    let done = state("processamentoassincrono");
    assert!(done.contains("Mês fechado: 100 notas"), "{done}");
    let cancelled = state("processamentoassincrono-cancel");
    assert!(cancelled.contains("Cancelado na nota"), "{cancelled}");
    // O X no meio: o on_close_query recusa e cancela (o Cancelar fica desabilitado por ele).
    let closed = state("processamentoassincrono-close");
    assert!(closed.contains("Cancelado na nota") && closed.contains("false"), "{closed}");
    let _ = std::fs::remove_dir_all(&output);
}

/// O roteiro do Paint grava `script-paint.png` (na pasta de onde a linguagem roda).
fn remove_saved() -> std::io::Result<()> {
    let paint = Path::new(&root()).join("examples/Paint");
    let _ = std::fs::remove_file(paint.join("script-paint.png"));
    for language in Language::ALL {
        let _ = std::fs::remove_file(paint.join(language.folder()).join("script-paint.png"));
    }
    Ok(())
}
