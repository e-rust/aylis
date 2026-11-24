//! A pergunta com campo (`prompt`), o ícone e a cor escolhidos nas caixas, a pilha de avisos
//! (`ToastStack`, `notify`) e o `EmptyState` sem botão por padrão.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 800.0, 600.0));
    ui.show(form).unwrap();
    (ui, form)
}

/// Os componentes abaixo de `id` (ele incluso).
fn subtree(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

fn find_class(ui: &Ui, root: ComponentId, class: &str) -> Vec<ComponentId> {
    subtree(ui, root).into_iter().filter(|&c| ui.class_name(c) == Some(class)).collect()
}

fn key(ui: &mut Ui, root: ComponentId, key: Key) {
    let event = KeyEvent { key, ..Default::default() };
    ui.key_down(root, event);
    ui.key_up(root, event);
}

#[test]
fn the_prompt_returns_the_typed_text_on_ok_and_none_on_cancel() {
    let (mut ui, _) = setup();
    let answer = Rc::new(RefCell::new(None));
    let a = answer.clone();
    let dialog = prompt(&mut ui, "Novo perfil", "Nome do perfil:", "Padrão", move |_, value| *a.borrow_mut() = Some(value));
    assert!(ui.is_modal(dialog));
    let edit = find_class(&ui, dialog, "Edit")[0];
    assert_eq!(ui.focused(dialog), Some(edit), "o foco vai para o campo");
    assert_eq!(ui.get_prop(edit, "text").unwrap(), PropValue::String("Padrão".into()), "o texto inicial");
    ui.set_prop(edit, "text", "Noite").unwrap();
    // O Enter no campo confirma (o OK é o botão padrão).
    key(&mut ui, dialog, Key::Enter);
    assert_eq!(*answer.borrow(), Some(Some("Noite".to_string())));
    assert!(!ui.contains(dialog), "a caixa sai ao fechar");

    let a = answer.clone();
    let dialog = prompt(&mut ui, "Novo perfil", "Nome do perfil:", "", move |_, value| *a.borrow_mut() = Some(value));
    key(&mut ui, dialog, Key::Escape);
    assert_eq!(*answer.borrow(), Some(None), "o Esc cancela: None");
}

#[test]
fn a_required_prompt_does_not_accept_an_empty_field() {
    let (mut ui, _) = setup();
    let answer = Rc::new(RefCell::new(None));
    let a = answer.clone();
    let dialog = prompt(&mut ui, "Novo perfil", "Nome:", "", move |_, value| *a.borrow_mut() = Some(value));
    set_prompt_required(&mut ui, dialog, true);
    let edit = find_class(&ui, dialog, "Edit")[0];
    let ok = find_class(&ui, dialog, "Button").into_iter().find(|&b| ui.get_prop(b, "default_button") == Ok(PropValue::Bool(true))).expect("o OK");
    assert_eq!(ui.get_prop(ok, "enabled").unwrap(), PropValue::Bool(false), "vazio: o OK desligado");
    key(&mut ui, dialog, Key::Enter);
    assert!(answer.borrow().is_none() && ui.contains(dialog), "o Enter não confirma vazio");
    ui.set_prop(edit, "text", "   ").unwrap();
    ui.fire(edit, "on_change", &EventArgs::Text("   ".into()));
    assert_eq!(ui.get_prop(ok, "enabled").unwrap(), PropValue::Bool(false), "só espaços contam como vazio");
    ui.set_prop(edit, "text", "Dia").unwrap();
    ui.fire(edit, "on_change", &EventArgs::Text("Dia".into()));
    assert_eq!(ui.get_prop(ok, "enabled").unwrap(), PropValue::Bool(true));
    ui.click(ok);
    assert_eq!(*answer.borrow(), Some(Some("Dia".to_string())));
}

#[test]
fn the_message_dialog_component_asks_with_a_field_by_property() {
    let (mut ui, form) = setup();
    let d = ui.add(MessageDialog::default(), form);
    ui.set_prop(d, "text", "Nome do perfil:").unwrap();
    ui.set_prop(d, "input", true).unwrap();
    ui.set_prop(d, "value", "Padrão").unwrap();
    ui.set_prop(d, "icon", "edit").unwrap();
    ui.set_prop(d, "icon_color", "success").unwrap();
    let dialog = MessageDialog::execute(&mut ui, d).unwrap();
    let buttons = find_class(&ui, dialog, "Button");
    assert_eq!(buttons.len(), 2, "com o campo e sem botões escolhidos: OK e Cancelar");
    let edit = find_class(&ui, dialog, "Edit")[0];
    ui.set_prop(edit, "text", "Noite").unwrap();
    ui.set_modal_result(dialog, ModalResult::Ok);
    assert_eq!(ui.get::<MessageDialog>(d).unwrap().value, "Noite", "o value fica com o digitado");
    // O Cancelar não muda o value.
    let dialog = MessageDialog::execute(&mut ui, d).unwrap();
    let edit = find_class(&ui, dialog, "Edit")[0];
    ui.set_prop(edit, "text", "Outro").unwrap();
    ui.set_modal_result(dialog, ModalResult::Cancel);
    assert_eq!(ui.get::<MessageDialog>(d).unwrap().value, "Noite");
}

/// A cor do círculo pintado (o primeiro `fill_ellipse` da caixa).
fn circle_color(ui: &mut Ui, dialog: ComponentId) -> Color {
    use aylis_core::pages::Op;
    ui.layout();
    let mut canvas = aylis_core::pages::RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(dialog, &mut canvas);
    canvas
        .ops
        .iter()
        .find_map(|op| match op {
            Op::FillEllipse { color, .. } => Some(*color),
            _ => None,
        })
        .expect("o círculo do ícone")
}

#[test]
fn the_app_chooses_the_icon_and_its_color_in_the_existing_dialogs() {
    let (mut ui, _) = setup();
    let warning = ui.theme().color("warning");
    let danger = ui.theme().color("danger");
    // Sem escolha: a cor do tipo, como sempre.
    let plain = confirm(&mut ui, "Excluir", "Excluir o perfil?", "Excluir", true, |_, _| {});
    assert_eq!(circle_color(&mut ui, plain), warning);
    ui.set_modal_result(plain, ModalResult::Cancel);
    // Escolhidos: o token do tema e uma cor solta.
    let chosen = confirm(&mut ui, "Excluir", "Excluir o perfil?", "Excluir", true, |_, _| {});
    set_dialog_icon(&mut ui, chosen, "delete", Some(ColorRef::Token("danger".into())));
    assert_eq!(circle_color(&mut ui, chosen), danger);
    ui.set_modal_result(chosen, ModalResult::Cancel);
    let purple = Color::rgb(0x6A, 0x1B, 0x9A);
    let message = show_message(&mut ui, "Pronto.");
    set_dialog_icon(&mut ui, message, "", Some(ColorRef::Value(purple)));
    assert_eq!(circle_color(&mut ui, message), purple, "só a cor: o ícone continua o do tipo");
    ui.set_modal_result(message, ModalResult::Ok);
    let dlg = message_dlg(&mut ui, "Salvo.", DlgKind::Information, &[ModalResult::Ok], |_, _| {});
    set_dialog_icon(&mut ui, dlg, "check", None);
    assert_eq!(circle_color(&mut ui, dlg), ui.theme().color("primary"), "só o ícone: a cor do tipo");
}

#[test]
fn the_toast_stack_is_a_class_with_methods_by_name() {
    let (mut ui, form) = setup();
    let stack = ui.create("ToastStack", Some(form)).unwrap();
    ui.set_prop(stack, "card_width", 300.0).unwrap();
    let first = ui.invoke(stack, "show", &[PropValue::from("Salvo"), PropValue::from("O cadastro foi gravado."), PropValue::Enum("success".into()), PropValue::Int(5000)]).unwrap();
    let again = ui.invoke(stack, "show", &[PropValue::from("Salvo"), PropValue::from("De novo."), PropValue::Enum("success".into()), PropValue::Int(5000)]).unwrap();
    assert_eq!(first, again, "o mesmo título só renova");
    assert!(ui.invoke(stack, "show", &[PropValue::from("x"), PropValue::from(""), PropValue::Enum("roxo".into()), PropValue::Int(10)]).is_err(), "tipo inválido é recusado");
    assert_eq!(ui.invoke(stack, "count", &[]).unwrap(), PropValue::Int(1));
    // O notify usa a pilha do form (posta pelo designer), onde ela estiver.
    let panel = ui.add(Panel::default(), form);
    ui.set_parent(stack, Some(panel)).unwrap();
    assert_eq!(notify(&mut ui, "Erro", "Falhou", ToastKind::Error, 5000), Some(stack));
    assert_eq!(ui.invoke(stack, "count", &[]).unwrap(), PropValue::Int(2));
    ui.invoke(stack, "clear", &[]).unwrap();
    assert_eq!(ui.invoke(stack, "count", &[]).unwrap(), PropValue::Int(0));
}

#[test]
fn the_notify_stack_starts_below_the_title_bar() {
    let (mut ui, form) = setup();
    let bar = ui.create("TitleBar", Some(form)).unwrap();
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(40.0);
    ui.layout();
    let stack = notify(&mut ui, "Salvo", "", ToastKind::Info, 1000).unwrap();
    ui.layout();
    let top = ui.bounds(stack).unwrap().y;
    assert!(top >= 40.0 + 8.0, "a pilha fica abaixo da barra de título: {top}");
    // Sem barra: a 16 px do topo, como antes.
    let (mut ui, _) = setup();
    let stack = notify(&mut ui, "Salvo", "", ToastKind::Info, 1000).unwrap();
    assert_eq!(ui.bounds(stack).unwrap().y, 16.0);
}

#[test]
fn a_new_empty_state_has_no_action_button() {
    let (mut ui, form) = setup();
    let empty = ui.create("EmptyState", Some(form)).unwrap();
    assert_eq!(ui.get_prop(empty, "action").unwrap(), PropValue::String(String::new()));
}

#[test]
fn fold_ignores_accents_and_case() {
    assert_eq!(fold("São Paulo"), "sao paulo");
    assert_eq!(fold("AÇÃO Ñandú Ýves"), "acao nandu yves");
    assert!(fold("Configurações").contains(&fold("CONFIGURACOES")));
    assert_eq!(fold("Straße"), "straße", "não troca letras");
}
