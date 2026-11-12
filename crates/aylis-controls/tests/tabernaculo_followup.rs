//! O relatório do Tabernáculo, o que ficou do lote 7 (abas laterais, área cliente, o espaço
//! dos segmentos, o layout que assenta antes de pintar, o rodapé do modal): o que dá para
//! conferir sem janela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 600.0, 400.0));
    ui.show(form).unwrap();
    ui.layout();
    (ui, form)
}

/// L02: o app arruma as teclas no `on_resize` do teclado (a largura de cada uma pela largura
/// nova). Ao mudar o tamanho, o layout refaz até assentar antes de pintar: o mesmo
/// `update_layout` (o da janela antes de cada quadro) já deixa as teclas na largura nova.
#[test]
fn what_on_resize_changes_settles_before_the_frame() {
    let (mut ui, form) = setup();
    let pad = ui.add(Layout::default(), form);
    ui.set_prop(pad, "align", "client").unwrap();
    let keys: Vec<ComponentId> = (0..4).map(|i| ui.add(Button::new(&format!("{i}")), pad)).collect();
    for &k in &keys {
        ui.set_prop(k, "align", "none").unwrap();
    }
    let ids = keys.clone();
    ui.on(pad, "on_resize", move |ui, sender, _| {
        let width = ui.absolute_rect(sender).map_or(0.0, |r| r.width) / ids.len() as f32;
        for (i, &k) in ids.iter().enumerate() {
            let _ = ui.set_prop(k, "left", (i as f32 * width) as f64);
            let _ = ui.set_prop(k, "width", width as f64);
        }
    });
    ui.update_layout();
    assert_eq!(ui.control(keys[3]).unwrap().width(), 150.0);
    // A janela mudou de tamanho (a plataforma põe o tamanho novo no form e refaz o layout).
    ui.control_mut(form).unwrap().set_size(800.0, 400.0);
    ui.invalidate();
    ui.update_layout();
    assert!(!ui.needs_layout(), "o layout não assentou");
    let r = ui.absolute_rect(keys[3]).unwrap();
    assert_eq!((r.x, r.width), (600.0, 200.0), "a tecla ficou com a largura de antes");
}

/// J11: a área cliente se define pelo código (`client_width`/`client_height`, o `ClientWidth`
/// do Delphi, ou o `width`/`height`, que no Aylis já são a área de dentro): o form muda e a
/// janela recebe o pedido de ficar com aquela área (a moldura por fora). No designer, nada vai
/// para a janela.
#[test]
fn the_client_size_is_set_by_code_and_the_window_follows() {
    let (mut ui, form) = setup();
    let _ = ui.take_window_commands();
    ui.set_prop(form, "client_width", 500.0).unwrap();
    ui.set_prop(form, "client_height", 300.0).unwrap();
    let cb = ui.control(form).unwrap();
    assert_eq!((cb.width(), cb.height()), (500.0, 300.0));
    assert_eq!(ui.get_prop(form, "client_width").unwrap(), PropValue::Float(500.0));
    let commands = ui.take_window_commands();
    assert!(commands.iter().any(|&(f, c)| f == form && c == WindowCommand::ApplySize), "{commands:?}");
    ui.set_prop(form, "height", 320.0).unwrap();
    assert_eq!(ui.get_prop(form, "client_height").unwrap(), PropValue::Float(320.0));
    assert!(ui.take_window_commands().contains(&(form, WindowCommand::ApplySize)));
    // No designer, o tamanho do form é só do projeto.
    ui.set_design_mode(true);
    ui.set_prop(form, "client_width", 420.0).unwrap();
    assert!(ui.take_window_commands().is_empty());
    // A propriedade é de execução: não vai para o JSON do form.
    assert!(ui.prop_info(form, "client_width").is_some_and(|p| p.runtime));
}

/// L15: o rodapé da caixa de mensagem pelo estilo `MessageDialog.footer`: sem ele, 36 de
/// altura e 14 acima (como antes); com `gap` 0 e `padding` 4 em cima e embaixo, 44 e nada.
#[test]
fn the_message_box_footer_follows_the_theme() {
    use aylis_core::theme::{Style, StyleProps};
    let footer_of = |ui: &Ui, dialog: ComponentId| {
        let footer = ui.children(dialog).iter().copied().find(|&c| ui.get::<Layout>(c).is_some()).unwrap();
        let cb = ui.control(footer).unwrap();
        (cb.height(), cb.margins().top, ui.control(dialog).unwrap().height())
    };
    let (mut ui, _form) = setup();
    let plain = message_dlg(&mut ui, "Salvar?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], |_, _| {});
    let (height, gap, total) = footer_of(&ui, plain);
    assert_eq!((height, gap), (36.0, 14.0));
    let mut theme = ui.theme().clone();
    theme.set_style("MessageDialog.footer", Style::new(StyleProps { gap: Some(0.0), padding: Some(Bounds::new(0.0, 4.0, 0.0, 4.0)), ..Default::default() }));
    ui.set_theme(theme);
    let themed = message_dlg(&mut ui, "Salvar?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], |_, _| {});
    let (height, gap, themed_total) = footer_of(&ui, themed);
    assert_eq!((height, gap), (44.0, 0.0));
    assert_eq!(themed_total, total - 6.0, "a caixa acompanha o rodapé");
}
