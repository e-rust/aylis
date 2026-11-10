//! O componente "ocupado": enquanto um método dele roda, ele está fora do `Ui`, e quem o
//! procura pelo `Ui` recebe um erro que diz isso (e não "propriedade desconhecida").

mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::boxy;
use aylis_core::*;

/// Ao clicar, pede as próprias propriedades pelo `Ui` (o uso errado) e guarda as respostas.
struct SelfAsker {
    base: ControlBase,
    seen: Rc<RefCell<Vec<String>>>,
}

impl Component for SelfAsker {
    fn class_name(&self) -> &'static str {
        "SelfAsker"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        let mut seen = self.seen.borrow_mut();
        seen.push(ctx.ui.get_prop(ctx.id, "width").unwrap_err().to_string());
        seen.push(ctx.ui.set_prop(ctx.id, "width", 50.0).unwrap_err().to_string());
        seen.push(ctx.ui.invoke(ctx.id, "anything", &[]).unwrap_err());
        seen.push(ctx.ui.send_message(ctx.id, &()).to_string());
    }
}

#[test]
fn a_busy_component_says_it_is_busy_with_its_path() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    ui.set_name(form, "frmPedido").unwrap();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let asker = ui.add(SelfAsker { base: ControlBase::new(100.0, 30.0), seen: seen.clone() }, form);
    ui.set_name(asker, "btnTotal").unwrap();

    ui.click(asker);

    let seen = seen.borrow();
    assert_eq!(seen.len(), 4);
    for message in &seen[..3] {
        assert!(message.contains("frmPedido.btnTotal está ocupado"), "{message}");
        assert!(message.contains("use `self`"), "com a dica: {message}");
    }
    assert_eq!(seen[3], "false", "a mensagem não é entregue");
    // Fora do método, ele volta a responder.
    assert_eq!(ui.get_prop(asker, "width").unwrap(), PropValue::Float(100.0));
}

#[test]
fn an_unknown_property_is_still_unknown() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    assert_eq!(ui.get_prop(form, "nada"), Err(Error::UnknownProperty("nada".into())));
}
