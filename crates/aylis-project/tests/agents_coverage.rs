//! O guia para agentes do Novo projeto (2a) e a cobertura de handlers (4b).

use aylis_project::new_project::{NewProjectOptions, create_with};
use aylis_project::{Language, agents, codegen, coverage, create};

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis-agents-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Com a opção marcada, o projeto novo nasce com o `AGENTS.md` da linguagem, o `CLAUDE.md` de
/// uma linha e o `.mcp.json` que chama o `aylis-tool mcp`.
#[test]
fn the_new_project_gets_the_agent_guide() {
    let dir = temp("guia");
    let tool = std::path::PathBuf::from(r"C:\Aylis\aylis-tool.exe");
    let options = NewProjectOptions { agent_guide: true, aylis_tool: Some(tool.clone()), ..Default::default() };
    let project = create_with(&dir, "Vendas", Language::Python, &options).unwrap();
    let agents = std::fs::read_to_string(project.dir().join("AGENTS.md")).unwrap();
    assert!(agents.starts_with("# Vendas"), "{agents}");
    assert!(agents.contains("O código em Python") && agents.contains("python/generated/") && agents.contains("Vendas.aylisproj"), "{agents}");
    assert!(agents.contains("MessageDialog") && agents.contains("SQL") && agents.contains("post_to"), "as regras comuns");
    let claude = std::fs::read_to_string(project.dir().join("CLAUDE.md")).unwrap();
    assert_eq!(claude.lines().count(), 1, "uma linha: {claude}");
    assert!(claude.contains("@AGENTS.md"));
    let mcp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(project.dir().join(".mcp.json")).unwrap()).unwrap();
    assert_eq!(mcp["mcpServers"]["aylis"]["command"], tool.display().to_string());
    assert_eq!(mcp["mcpServers"]["aylis"]["args"], serde_json::json!(["mcp"]));

    // Desmarcada: nada disso.
    let plain = create_with(&dir, "Outro", Language::Rust, &NewProjectOptions::default()).unwrap();
    for file in ["AGENTS.md", "CLAUDE.md", ".mcp.json"] {
        assert!(!plain.dir().join(file).exists(), "{file}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// Cada linguagem tem a sua parte, sem marcador sobrando; o que já existe na pasta fica (um
/// projeto criado dentro de um repositório), e o `.mcp.json` só ganha o `aylis`.
#[test]
fn the_guide_keeps_what_was_there() {
    let dir = temp("existente");
    for language in Language::ALL {
        let project = create(&dir, &format!("App{}", language.folder()), language).unwrap();
        let text = agents::guide_text(&project);
        for mark in ["{name}", "{language}", "{folder}", "{project_file}", "{language_section}"] {
            assert!(!text.contains(mark), "{language:?}: sobrou {mark}");
        }
        assert!(text.contains(&format!("O código em {}", language.name())), "{language:?}");
        assert!(text.contains(&format!("{}/generated/", language.folder())), "{language:?}");
    }
    let project = create(&dir, "Repo", Language::Rust).unwrap();
    std::fs::write(project.dir().join("AGENTS.md"), "regras do repositório\n").unwrap();
    std::fs::write(project.dir().join(".mcp.json"), r#"{"mcpServers":{"outro":{"command":"x"}}}"#).unwrap();
    let written = agents::write_guide(&project, None).unwrap();
    assert_eq!(std::fs::read_to_string(project.dir().join("AGENTS.md")).unwrap(), "regras do repositório\n");
    assert!(written.iter().any(|p| p.ends_with("CLAUDE.md")) && !written.iter().any(|p| p.ends_with("AGENTS.md")));
    let mcp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(project.dir().join(".mcp.json")).unwrap()).unwrap();
    assert_eq!(mcp["mcpServers"]["outro"]["command"], "x");
    assert_eq!(mcp["mcpServers"]["aylis"]["command"], "aylis-tool", "sem o caminho: pelo PATH");
    // De novo: nada a gravar.
    assert!(agents::write_guide(&project, None).unwrap().is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Os handlers ligados nos forms, menos os que o app avisou que rodaram; os que faltam vêm com
/// o arquivo e a linha do código.
#[test]
fn the_handler_coverage_lists_what_never_ran() {
    let dir = temp("cobertura");
    let project = create(&dir, "Loja", Language::Rust).unwrap();
    let form = project.dir().join(&project.form("frmPrincipal").unwrap().file);
    let layout = serde_json::json!({
        "aylis": 1,
        "root": {
            "class": "Form", "name": "frmPrincipal", "on_show": "frmPrincipal_show",
            "children": [
                { "class": "Button", "name": "btnSalvar", "on_click": "btnSalvar_click" },
                { "class": "Button", "name": "btnGravar", "on_click": "btnSalvar_click" },
                { "class": "Button", "name": "btnSair", "on_click": "btnSair_click" }
            ]
        }
    });
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    codegen::generate(&project).unwrap();

    let handlers = coverage::project_handlers(&project);
    let names: Vec<String> = handlers.iter().map(coverage::Handler::key).collect();
    assert_eq!(names, ["frmPrincipal.frmPrincipal_show", "frmPrincipal.btnSalvar_click", "frmPrincipal.btnSair_click"]);
    assert_eq!(handlers[1].events, ["btnSalvar.on_click", "btnGravar.on_click"], "um handler, dois eventos");

    assert_eq!(coverage::project_report(&project).covered.len(), 0, "nada rodou ainda");
    assert_eq!(coverage::record(&project, &["frmPrincipal.btnSalvar_click".into(), "frmOutro.x".into()]).unwrap(), 2);
    assert_eq!(coverage::record(&project, &["frmPrincipal.btnSalvar_click".into()]).unwrap(), 0, "o que já estava não conta de novo");
    // Sem o form: cobre o de mesmo nome.
    coverage::record(&project, &["btnSair_click".into()]).unwrap();
    let report = coverage::project_report(&project);
    assert_eq!(report.total(), 3);
    assert_eq!(report.missing.iter().map(|h| h.handler.as_str()).collect::<Vec<_>>(), ["frmPrincipal_show"]);
    assert!(report.since_ms > 0);
    let json = coverage::report_json(&project, &report);
    assert_eq!(json["covered"], 2);
    let missing = &json["missing"][0];
    assert_eq!(missing["handler"], "frmPrincipal_show");
    assert!(missing["file"].as_str().unwrap().ends_with(".rs"), "{json}");
    assert!(missing["line"].as_u64().unwrap() > 1, "{json}");
    // Fora do git.
    assert!(std::fs::read_to_string(project.dir().join(".aylis/.gitignore")).unwrap().lines().any(|l| l.trim() == "*"));

    coverage::clear(&project).unwrap();
    assert_eq!(coverage::project_report(&project).missing.len(), 3, "limpar recomeça do zero");
    let _ = std::fs::remove_dir_all(&dir);
}
