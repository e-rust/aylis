//! O `code-sync.json` diante da edição à mão no `impl` (G22 do relatório do Tabernáculo): o
//! `cargo fmt`, uma chave dentro de um texto, a versão antiga comentada acima do handler, a
//! função renomeada à mão e a apagada. A sincronização se refaz pelo que está no fonte, e o
//! código do usuário nunca se perde.

use aylis_project::sync::{self, Pending};
use aylis_project::{Language, Project, codegen, create};

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_sync_hand_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// O form com dois botões: `btnSalvar.on_click` → `salvar`, `btnLimpar.on_click` → `limpar`.
fn set_form(project: &Project, salvar: &str, limpar: &str) {
    let layout = format!(
        r#"{{"aylis":1,"root":{{"class":"Form","name":"frmPrincipal","children":[{{"class":"Button","name":"btnSalvar","on_click":"{salvar}"}},{{"class":"Button","name":"btnLimpar","on_click":"{limpar}"}}]}}}}"#
    );
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
}

fn source(project: &Project, language: Language) -> std::path::PathBuf {
    codegen::form_source(&project.with_language(language), "frmPrincipal").unwrap()
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

/// Troca `from` por `to` no arquivo (uma vez), como o usuário no editor. A marca do arquivo
/// precisa mudar: no mesmo instante da geração, o sistema pode dar a mesma hora.
fn edit(path: &std::path::Path, from: &str, to: &str) {
    let text = read(path);
    assert!(text.contains(from), "{}: falta {from:?}\n{text}", path.display());
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(path, text.replacen(from, to, 1)).unwrap();
}

const STUB_RUST: &str = "let _ = (ui, sender, args);";

/// Rust e Python em dia, cada handler com código nas duas.
fn two_languages(name: &str) -> (std::path::PathBuf, Project, std::path::PathBuf, std::path::PathBuf) {
    let base = temp(name);
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    set_form(&project, "salvar_click", "limpar_click");
    codegen::generate(&project).unwrap();
    let rust = source(&project, Language::Rust);
    // O primeiro corpo tem uma chave dentro de um texto (um `format!` com `{{`, um `"}"`).
    let text = read(&rust);
    let salvar = text.find("fn salvar_click(").unwrap();
    let at = salvar + text[salvar..].find(STUB_RUST).unwrap();
    let mut text = text;
    text.replace_range(at..at + STUB_RUST.len(), "let _ = (sender, args);\n        let fecha = \"}\";\n        ui.log(&format!(\"salvando {fecha}\"));\n        let total = soma(1, 2);\n        let _ = total;");
    std::fs::write(&rust, &text).unwrap();
    edit(&rust, STUB_RUST, "let _ = (sender, args);\n        ui.log(\"limpando\");");
    codegen::generate(&project).unwrap();
    project.language = Language::Python;
    codegen::generate(&project).unwrap();
    let python = source(&project, Language::Python);
    edit(&python, "def salvar_click(aylis, event):\n    pass", "def salvar_click(aylis, event):\n    fecha = '}'\n    aylis.log(f'salvando {fecha}')\n    total = soma(1, 2)");
    edit(&python, "def limpar_click(aylis, event):\n    pass", "def limpar_click(aylis, event):\n    aylis.log('limpando')");
    codegen::generate(&project).unwrap();
    project.language = Language::Rust;
    codegen::generate(&project).unwrap();
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![], "em dia");
    assert_eq!(sync::pending(&project, Language::Rust).unwrap(), vec![], "em dia");
    (base, project, rust, python)
}

fn changed(project: &Project, language: Language) -> Vec<String> {
    sync::pending(project, language)
        .unwrap()
        .into_iter()
        .filter_map(|p| match p {
            Pending::Changed { handler, .. } => Some(handler),
            _ => None,
        })
        .collect()
}

/// O `cargo fmt` (o recuo, as linhas, a vírgula do fim numa chamada quebrada, o `impl` vazio
/// juntado) não é lógica nova: o Python não fica pendente.
#[test]
fn rustfmt_is_not_new_logic() {
    let (base, project, rust, _) = two_languages("fmt");
    edit(&rust, "let total = soma(1, 2);", "let total = soma(\n            1,\n            2,\n        );");
    edit(&rust, "ui.log(\"limpando\");", "ui.log(\n            \"limpando\",\n        );");
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), Vec::<String>::new(), "{}", read(&rust));
    std::fs::remove_dir_all(&base).unwrap();
}

/// Uma chave dentro de um texto (`"}"`) não termina o corpo: mudar o que vem depois dela é
/// mudar o handler.
#[test]
fn a_brace_inside_a_string_does_not_end_the_body() {
    let (base, project, rust, _) = two_languages("brace");
    edit(&rust, "soma(1, 2)", "soma(1, 3)");
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), vec!["salvar_click".to_string()], "{}", read(&rust));
    std::fs::remove_dir_all(&base).unwrap();
}

/// A versão antiga comentada acima do handler não é o handler: o corpo é o da função de
/// verdade.
#[test]
fn a_commented_out_copy_is_not_the_handler() {
    let (base, project, rust, _) = two_languages("comment");
    edit(&rust, "    pub fn limpar_click(", "    // pub fn limpar_click(&mut self) { ui.log(\"antigo\"); }\n    pub fn limpar_click(");
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), Vec::<String>::new(), "o comentário não é lógica nova");
    edit(&rust, "ui.log(\"limpando\");", "ui.log(\"limpando tudo\");");
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), vec!["limpar_click".to_string()], "{}", read(&rust));
    std::fs::remove_dir_all(&base).unwrap();
}

/// A função renomeada à mão e depois no form: o histórico vai junto (nada de pendência nem de
/// sobra com o nome antigo), o Python é renomeado com o corpo, e o corpo do Rust fica.
#[test]
fn a_handler_renamed_by_hand_keeps_its_history() {
    let (base, project, rust, python) = two_languages("rename");
    edit(&rust, "pub fn salvar_click(", "pub fn gravar_click(");
    set_form(&project, "gravar_click", "limpar_click");
    codegen::generate(&project).unwrap();
    let (r, p) = (read(&rust), read(&python));
    assert!(r.contains("fn gravar_click(") && r.contains("soma(1, 2)") && !r.contains("fn salvar_click("), "{r}");
    assert!(p.contains("def gravar_click(") && p.contains("soma(1, 2)") && !p.contains("def salvar_click("), "{p}");
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![]);
    assert_eq!(sync::pending(&project, Language::Rust).unwrap(), vec![]);
    let state = std::fs::read_to_string(project.dir().join(sync::FILE)).unwrap();
    assert!(!state.contains("salvar_click"), "sem sobra do nome antigo: {state}");
    std::fs::remove_dir_all(&base).unwrap();
}

/// A função apagada à mão (e o evento ainda no form): a geração não a recria (decisão de
/// 08/11) e o resto do arquivo fica como estava; o outro handler segue acompanhado.
#[test]
fn a_handler_deleted_by_hand_is_not_recreated_and_nothing_else_moves() {
    let (base, project, rust, _) = two_languages("delete");
    let text = read(&rust);
    let start = text.find("    pub fn limpar_click(").unwrap();
    let end = start + text[start..].find("\n    }\n").unwrap() + "\n    }\n".len();
    let mut cut = text.clone();
    cut.replace_range(start..end, "");
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(&rust, &cut).unwrap();
    codegen::generate(&project).unwrap();
    assert_eq!(read(&rust), cut, "nada recriado nem mexido");
    edit(&rust, "soma(1, 2)", "soma(2, 2)");
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), vec!["salvar_click".to_string()]);
    std::fs::remove_dir_all(&base).unwrap();
}

/// A impressão de antes (FNV-1a dos bytes sem espaço do corpo), que um `code-sync.json`
/// gravado por uma versão anterior tem.
fn legacy_fingerprint(body: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in body.bytes().filter(|b| !b.is_ascii_whitespace()) {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// O `code-sync.json` de uma versão anterior (a impressão antiga): o código que não mudou
/// continua em dia, sem revisão nova, e a impressão é refeita pelo fonte. Uma impressão sem
/// versão que não bate com nenhuma regra conhecida (a de 09/11, por exemplo) também só é
/// refeita: tomá-la por lógica nova deixava a Calculadora, o Paint, o Player e o IBGE com
/// handlers "a portar" sem nada mudado.
#[test]
fn hashes_from_before_still_match() {
    let (base, project, _, _) = two_languages("legacy");
    let path = project.dir().join(sync::FILE);
    let mut state: sync::SyncState = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let old = [("rust", "let _ = (sender, args); ui.log(\"limpando\");"), ("python", "aylis.log('limpando')")];
    for (lang, body) in old {
        state.languages.get_mut(lang).unwrap().handlers.get_mut("frmPrincipal.limpar_click").unwrap().hash = legacy_fingerprint(body);
        state.languages.get_mut(lang).unwrap().handlers.get_mut("frmPrincipal.salvar_click").unwrap().hash = "0123456789abcdef".into();
    }
    std::fs::write(&path, serde_json::to_string_pretty(&state).unwrap()).unwrap();
    sync::record(&project).unwrap();
    let after: sync::SyncState = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(after.revisions["frmPrincipal.limpar_click"], state.revisions["frmPrincipal.limpar_click"], "o mesmo corpo");
    assert_ne!(after.languages["rust"].handlers["frmPrincipal.limpar_click"].hash, legacy_fingerprint(old[0].1), "refeita pelo fonte");
    assert_eq!(after.revisions["frmPrincipal.salvar_click"], state.revisions["frmPrincipal.salvar_click"], "impressão de uma regra desconhecida: sem revisão nova");
    assert!(after.languages["python"].handlers["frmPrincipal.salvar_click"].hash.starts_with("v2:"), "refeita pelo fonte");
    assert_eq!(sync::pending(&project, Language::Rust).unwrap(), vec![], "nada a portar");
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![], "nada a portar");
    // Já na impressão nova, a edição de verdade volta a contar como lógica nova.
    let python = source(&project, Language::Python);
    edit(&python, "total = soma(1, 2)", "total = soma(1, 3)");
    sync::record(&project).unwrap();
    assert_eq!(changed(&project, Language::Rust), vec!["salvar_click".to_string()], "editado em Python");
    std::fs::remove_dir_all(&base).unwrap();
}

/// Só Rust (sem outra linguagem que leve o histórico junto): a função renomeada à mão e depois
/// no form não deixa sobra com o nome antigo, e a cópia comentada de um handler novo não impede
/// a geração de criá-lo.
#[test]
fn rust_alone_renamed_by_hand_and_a_commented_new_handler() {
    let base = temp("rust_alone");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    set_form(&project, "salvar_click", "limpar_click");
    codegen::generate(&project).unwrap();
    let rust = source(&project, Language::Rust);
    edit(&rust, STUB_RUST, "let _ = (sender, args);\n        ui.log(\"salvando\");");
    codegen::generate(&project).unwrap();
    edit(&rust, "pub fn salvar_click(", "pub fn gravar_click(");
    edit(&rust, "    pub fn limpar_click(", "    // pub fn apagar_click(&mut self) {}\n    pub fn limpar_click(");
    set_form(&project, "gravar_click", "apagar_click");
    codegen::generate(&project).unwrap();
    let text = read(&rust);
    assert!(text.contains("ui.log(\"salvando\")") && text.matches("fn gravar_click(").count() == 1, "{text}");
    assert!(text.contains("    pub fn apagar_click(&mut self, ui: &mut Ui"), "o handler novo, apesar da cópia comentada: {text}");
    let state = std::fs::read_to_string(project.dir().join(sync::FILE)).unwrap();
    assert!(!state.contains("salvar_click"), "sem sobra do nome antigo: {state}");
    assert_eq!(sync::handler_location(&project, Language::Rust, "frmPrincipal", "apagar_click").unwrap().1, text[..text.find("    pub fn apagar_click(").unwrap()].matches('\n').count() + 1, "o F12 vai à função, não ao comentário");
    std::fs::remove_dir_all(&base).unwrap();
}

/// O `rustfmt` de verdade (quando está instalado) no arquivo do form: nada fica pendente.
#[test]
fn real_rustfmt_is_not_new_logic() {
    let Ok(out) = std::process::Command::new("rustfmt").arg("--version").output() else {
        eprintln!("sem rustfmt: pulado");
        return;
    };
    if !out.status.success() {
        return;
    }
    let (base, project, rust, _) = two_languages("realfmt");
    // Uma chamada longa, que o rustfmt quebra em linhas (com a vírgula do fim).
    edit(&rust, "let total = soma(1, 2);", "let total = soma(1, 2); ui.log(&format!(\"um texto comprido para passar da largura {} {} {}\", total, total, total));");
    codegen::generate(&project).unwrap();
    project_python_ported(&project);
    std::thread::sleep(std::time::Duration::from_millis(20));
    let status = std::process::Command::new("rustfmt").args(["--edition", "2021"]).arg(&rust).status().unwrap();
    assert!(status.success());
    codegen::generate(&project).unwrap();
    assert_eq!(changed(&project, Language::Python), Vec::<String>::new(), "{}", read(&rust));
    std::fs::remove_dir_all(&base).unwrap();
}

/// O Python acompanha o que mudou no Rust (marcado como portado).
fn project_python_ported(project: &Project) {
    for handler in changed(project, Language::Python) {
        sync::mark_ported(project, Language::Python, "frmPrincipal", &handler).unwrap();
    }
    assert_eq!(changed(project, Language::Python), Vec::<String>::new());
}
