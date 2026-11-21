//! O OK das Opções não mexe nas origens dos pacotes de quem não mexeu nelas: antes, o campo dos
//! espelhos (um `Memo`) era lido como `Edit`, vinha vazio, e o OK apagava os espelhos e as
//! pastas gravados (achado ao ligar o MCP no Fedora em 20/11).

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{app, options};

/// Um teste só neste binário: ele troca a pasta do usuário do processo.
#[test]
fn ok_keeps_the_package_sources() {
    let home = std::env::temp_dir().join(format!("aylis_ide_options_sources_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    for key in ["USERPROFILE", "HOME", "LOCALAPPDATA", "APPDATA"] {
        // SAFETY: o único teste deste binário, antes de qualquer thread dele ler o ambiente.
        unsafe { std::env::set_var(key, &home) };
    }
    // O OK das Opções grava o `config.toml` do cargo, no `CARGO_HOME`, que o `cargo test` passa
    // apontando para o de verdade: sem trocar, o teste mudava a pasta de compilação da máquina
    // inteira para a pasta temporária dele (e enchia o disco; 21/11).
    // SAFETY: como acima.
    unsafe { std::env::set_var("CARGO_HOME", home.join(".cargo")) };
    let real_config = aylis_tools::cargo_config::global_path();
    assert!(real_config.as_ref().is_some_and(|p| p.starts_with(&home)), "o config.toml do cargo do teste fica na pasta dele: {real_config:?}");
    use aylis_tools::settings::{PACKAGE_DIRS, PACKAGE_MIRRORS, list, set_list};
    // Um espelho gravado: o OK não pode apagá-lo.
    set_list(PACKAGE_MIRRORS, &["https://empresa.example/pacotes.json".to_string()]).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    options::open(&mut ide.ui, &state, &ide.ids);
    let ui = &mut ide.ui;
    let form = ui.find("frmOpcoesIde").expect("a tela abriu");
    let mut all = vec![form];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    let ok = all.iter().copied().find(|&c| ui.get::<aylis_controls::Button>(c).is_some_and(|b| b.text == "OK")).expect("o OK");
    ui.click(ok);
    ui.advance(0.05);
    assert_eq!(list(PACKAGE_MIRRORS), ["https://empresa.example/pacotes.json"], "o OK manteve o espelho");

    // Com uma pasta e outro espelho gravados, o OK também os mantém.
    set_list(PACKAGE_DIRS, &["/srv/pacotes".to_string()]).unwrap();
    set_list(PACKAGE_MIRRORS, &["https://exemplo.com/pacotes.json".to_string()]).unwrap();
    options::open(&mut ide.ui, &state, &ide.ids);
    let ui = &mut ide.ui;
    let form = ui.find("frmOpcoesIde").expect("a tela abriu de novo");
    let mut all = vec![form];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    let ok = all.iter().copied().find(|&c| ui.get::<aylis_controls::Button>(c).is_some_and(|b| b.text == "OK")).expect("o OK");
    ui.click(ok);
    ui.advance(0.05);
    assert_eq!(list(PACKAGE_DIRS), ["/srv/pacotes"]);
    assert_eq!(list(PACKAGE_MIRRORS), ["https://exemplo.com/pacotes.json"]);
    let _ = std::fs::remove_dir_all(&home);
}
