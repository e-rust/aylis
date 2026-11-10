//! As mensagens do próprio aylis no idioma do app. O idioma vale para o processo, por isso
//! um teste só (num binário à parte) troca entre português, inglês e espanhol.

use aylis_controls::*;
use aylis_core::*;

fn buttons(ui: &Ui, root: ComponentId) -> Vec<String> {
    let mut all = vec![root];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    // Os botões ficam alinhados à direita: o último pedido é o primeiro filho.
    let mut texts: Vec<String> = all.iter().filter_map(|&c| ui.get::<Button>(c)).map(|b| b.text.clone()).collect();
    texts.reverse();
    texts
}

#[test]
fn aylis_messages_follow_the_language() {
    let mut ui = Ui::new();
    let ask = |ui: &mut Ui| {
        let dlg = message_dlg(ui, "?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No, ModalResult::Cancel], |_, _| {});
        buttons(ui, dlg)
    };

    // Português: o texto original.
    assert_eq!(ask(&mut ui), ["Sim", "Não", "Cancelar"]);
    assert_eq!(check_rules("required", "").as_deref(), Some("Obrigatório"));
    assert_eq!(check_rules("min:3", "ab").as_deref(), Some("Mínimo de 3 caracteres"));

    messages::set_language("en-US");
    assert_eq!(messages::language(), "en");
    assert_eq!(ask(&mut ui), ["Yes", "No", "Cancel"]);
    assert_eq!(check_rules("required", "").as_deref(), Some("Required"));
    assert_eq!(check_rules("min:3", "ab").as_deref(), Some("At least 3 characters"));
    assert_eq!(check_rules("email", "x@y").as_deref(), Some("Invalid e-mail"));

    messages::set_language("es");
    assert_eq!(ask(&mut ui), ["Sí", "No", "Cancelar"]);
    assert_eq!(check_rules("max:2", "abc").as_deref(), Some("Máximo 2 caracteres"));

    // O dicionário do app vence o do aylis.
    let mut words = std::collections::HashMap::new();
    words.insert("Sim".to_string(), "Vale".to_string());
    ui.set_translations(words);
    assert_eq!(ui.tr("Sim"), "Vale");
    assert_eq!(ui.tr("Não"), "No");

    // Idioma sem catálogo: fica em português.
    messages::set_language("de");
    assert_eq!(messages::t("Cancelar"), "Cancelar");
    messages::set_language("pt-BR");
}
