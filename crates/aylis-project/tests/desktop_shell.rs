//! A tela publicada no desktop por um projeto em outra linguagem (pedido do autor em 17/11):
//! uma casca em Rust com o projeto embutido, que inicia o backend da pasta `backend/`. Antes
//! era o `aylis-host` genérico com os forms soltos, o `.aylisproj` e um atalho `.cmd`.

#[test]
fn the_desktop_shell_embeds_the_project_and_starts_the_backend_beside_it() {
    let base = std::env::temp_dir().join(format!("aylis_casca_desktop_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Casca", aylis_project::Language::Pascal).unwrap();
    // Um argumento com aspas e barra invertida sai como literal válido do Rust.
    let command = vec!["backend/app.exe".to_string(), "--nome=\"a\\b\"".to_string()];
    let shell = aylis_project::codegen::local_shell(&project, &command).unwrap();
    assert!(shell.ends_with(".aylis/shell-desktop"), "{}", shell.display());

    let main = std::fs::read_to_string(shell.join("src/main.rs")).unwrap();
    assert!(main.contains(r#"const BACKEND: &[&str] = &["backend/app.exe", "--nome=\"a\\b\""];"#), "{main}");
    assert!(main.contains("aylis::include_project!()"), "o projeto vai embutido: {main}");
    assert!(main.contains("aylis::spawn_backend("), "{main}");
    assert!(main.contains("windows_subsystem = \"windows\""), "sem janela de console: {main}");

    let cargo = std::fs::read_to_string(shell.join("Cargo.toml")).unwrap();
    assert!(cargo.contains("[[bin]]") && cargo.contains("\"remote\""), "{cargo}");
    let build = std::fs::read_to_string(shell.join("build.rs")).unwrap();
    assert!(build.contains("aylis_build::project_at("), "{build}");
    let _ = std::fs::remove_dir_all(&base);
}
