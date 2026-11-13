//! Ferramentas → Pacotes de ferramentas, as origens nas Opções e o "Corrigir" de um pacote sem
//! origem, numa pasta de usuário vazia (sem pacotes instalados, sem origens configuradas).

use aylis_core::{ComponentId, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{app, mcp, toolpacksview};
use aylis_tools::Target;
use aylis_tools::env::{Check, Fix, Status};
use serde_json::json;

fn all_under(ui: &Ui, root: ComponentId) -> Vec<ComponentId> {
    let mut all = vec![root];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    all
}

fn buttons(ui: &Ui, root: ComponentId) -> Vec<(String, ComponentId)> {
    all_under(ui, root).into_iter().filter_map(|c| ui.get::<aylis_controls::Button>(c).map(|b| (b.text.clone(), c))).collect()
}

fn button(ui: &Ui, root: ComponentId, text: &str) -> ComponentId {
    buttons(ui, root).into_iter().find(|(t, _)| t == text).unwrap_or_else(|| panic!("sem o botão {text}")).1
}

/// Um teste só neste binário: ele troca a pasta do usuário do processo.
#[test]
fn the_tool_packages_screen_and_the_fix_without_a_source() {
    let home = std::env::temp_dir().join(format!("aylis_ide_toolpacks_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    for key in ["USERPROFILE", "HOME", "LOCALAPPDATA", "APPDATA"] {
        // SAFETY: o único teste deste binário, antes de qualquer thread dele ler o ambiente.
        unsafe { std::env::set_var(key, &home) };
    }
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    // A verificação das linguagens que a IDE começa ao abrir ocuparia a vaga do ambiente.
    state.borrow_mut().env_job = None;
    let run = |ui: &mut Ui, name: &str, args: serde_json::Value| mcp::run_tool(ui, &state, &ide.ids, name, &args);

    // O "Corrigir…" de um item que um pacote resolve, sem origem nenhuma: não roda, diz e
    // oferece as Opções.
    let fix = Fix { description: "Instala o pacote JDK".into(), command: vec!["winget".into(), "install".into()], interactive: false, fetch: Some("jdk".into()), env: Vec::new() };
    assert_eq!(fix.command_line(), "aylis-tool packages install jdk");
    state.borrow_mut().checks = vec![Check { target: Target::Android, name: "Java (JDK 17+)".into(), status: Status::Missing, detail: "não encontrado".into(), fix: Some(fix), help: None }];
    state.borrow_mut().env_language = Some(aylis_project::Language::Rust);
    aylis_ide::tooldialogs::environment(&mut ide.ui, &state, &ide.ids, false);
    state.borrow_mut().env_job = None;
    aylis_ide::tooldialogs::fill_environment(&mut ide.ui, &state, &ide.ids);
    run(&mut ide.ui, "dialog_click", json!({ "text": "Corrigir…" })).unwrap();
    let message = run(&mut ide.ui, "dialog_state", json!({})).unwrap().to_string();
    assert!(message.contains("Nenhuma origem de pacotes"), "{message}");
    run(&mut ide.ui, "dialog_click", json!({ "text": "Não" })).unwrap();
    assert!(state.borrow().env_job.is_none(), "nada rodou");
    let env_form = ide.ui.find("frmAmbiente").unwrap();
    ide.ui.click(button(&ide.ui, env_form, "Fechar"));

    // A tela: um pacote por linha (os desta plataforma), nenhum instalado.
    toolpacksview::open(&mut ide.ui, &state, &ide.ids);
    state.borrow_mut().env_job = None;
    let form = ide.ui.find("frmPacotesFerramentas").expect("a tela abriu");
    let available: Vec<String> = aylis_tools::toolpacks::Manifest::embedded().packages().into_iter().filter(|p| p.available()).map(|p| p.name).collect();
    let checks: Vec<(String, ComponentId)> = all_under(&ide.ui, form)
        .into_iter()
        .filter(|&c| ide.ui.get::<aylis_controls::CheckBox>(c).is_some())
        .map(|c| (ide.ui.get_prop(c, "hint").ok().and_then(|v| v.to_str().ok().map(str::to_string)).unwrap_or_default(), c))
        .collect();
    assert_eq!(checks.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>(), available);
    assert!(state.borrow().toolpacks.iter().all(|r| r.installed.is_none()), "a pasta do usuário está vazia");
    let texts: Vec<String> = buttons(&ide.ui, form).into_iter().map(|(t, _)| t).collect();
    for wanted in ["Instalar", "Remover", "Cancelar", "Fechar", "Origens…"] {
        assert!(texts.iter().any(|t| t == wanted), "falta {wanted}: {texts:?}");
    }

    // Instalar sem marcar nada: só a mensagem.
    ide.ui.click(button(&ide.ui, form, "Instalar"));
    assert_eq!(state.borrow().toolpacks_log, "Marque pelo menos um pacote.");
    // Marcado, sem origem: a mensagem e, com o Sim, as Opções na aba Pacotes.
    let _ = ide.ui.set_prop(checks[0].1, "checked", true);
    ide.ui.click(button(&ide.ui, form, "Instalar"));
    assert!(state.borrow().env_job.is_none(), "sem origem, nada começa");
    let message = run(&mut ide.ui, "dialog_state", json!({})).unwrap().to_string();
    assert!(message.contains("Nenhuma origem de pacotes"), "{message}");
    run(&mut ide.ui, "dialog_click", json!({ "text": "Sim" })).unwrap();
    let options = ide.ui.find("frmOpcoesIde").expect("as Opções abriram");
    let tabs = all_under(&ide.ui, options).into_iter().find(|&c| ide.ui.get::<aylis_controls::TabControl>(c).is_some()).unwrap();
    let pages: Vec<String> = all_under(&ide.ui, tabs).into_iter().filter_map(|c| ide.ui.get::<aylis_controls::TabItem>(c).map(|t| t.caption.clone())).collect();
    let index = ide.ui.get::<aylis_controls::TabControl>(tabs).unwrap().tab_index as usize;
    assert_eq!(pages.get(index).map(String::as_str), Some("Pacotes"), "{pages:?}");
    assert!(buttons(&ide.ui, options).iter().any(|(t, _)| t == "Testar as origens"));
    let _ = std::fs::remove_dir_all(&home);
}
