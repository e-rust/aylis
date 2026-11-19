//! Os handlers tipados (`form_handler`): o método do form recebe quem disparou no tipo da
//! classe (`FromSender`) e o valor do evento no tipo (`EventValue`), em vez do `ComponentId` e
//! do `EventArgs`. O de sempre continua valendo ao lado.

mod common;

use common::boxy;
use aylis_core::instances::form_handler;
use aylis_core::*;

/// O handle de um componente, como o acesso tipado gera (aqui, qualquer `Box`).
#[derive(Clone, Copy)]
struct Caixa {
    id: ComponentId,
}

impl FromSender for Caixa {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.name(id) == Some("caixa")).then_some(Self { id })
    }
}

#[derive(Default)]
struct FrmTeste {
    log: Vec<String>,
}

impl FrmTeste {
    fn clique(&mut self, ui: &mut Ui, sender: Caixa) {
        self.log.push(format!("clique {}", ui.path_of(sender.id)));
    }

    fn valor(&mut self, _ui: &mut Ui, _sender: Caixa, value: f64) {
        self.log.push(format!("valor {value}"));
    }

    fn data(&mut self, _ui: &mut Ui, _sender: ComponentId, value: Option<Date>) {
        self.log.push(format!("data {value:?}"));
    }

    fn antigo(&mut self, _ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.log.push(format!("antigo {args:?}"));
    }
}

fn setup() -> (Ui, ComponentId, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    ui.register_form_class("frmTeste", |_, _| Ok(Box::new(FrmTeste::default())));
    ui.register_handler("frmTeste.clique", form_handler(FrmTeste::clique));
    ui.register_handler("frmTeste.valor", form_handler(FrmTeste::valor));
    ui.register_handler("frmTeste.data", form_handler(FrmTeste::data));
    ui.register_handler("frmTeste.antigo", form_handler(FrmTeste::antigo));
    let form = ui.add_root(boxy(300.0, 200.0));
    ui.set_name(form, "frmTeste").unwrap();
    let caixa = ui.add(boxy(80.0, 30.0), form);
    ui.set_name(caixa, "caixa").unwrap();
    let outra = ui.add(boxy(80.0, 30.0), form);
    ui.set_name(outra, "outra").unwrap();
    ui.bind_form(form).unwrap();
    (ui, form, caixa, outra)
}

fn log(ui: &Ui, form: ComponentId) -> Vec<String> {
    ui.form_instance::<FrmTeste>(form).unwrap().log.clone()
}

#[test]
fn the_typed_handler_gets_the_sender_and_the_value() {
    let (mut ui, form, caixa, _) = setup();
    ui.set_prop(caixa, "on_click", "clique").unwrap();
    ui.set_prop(caixa, "on_dbl_click", "antigo").unwrap();
    ui.fire(caixa, "on_click", &EventArgs::None);
    ui.fire(caixa, "on_dbl_click", &EventArgs::None);
    // O valor: o número e a data (do texto ISO, como o DateEdit manda).
    ui.set_prop(caixa, "on_resize", "valor").unwrap();
    ui.fire(caixa, "on_resize", &EventArgs::Value(PropValue::Float(2.5)));
    ui.set_prop(caixa, "on_enter", "data").unwrap();
    ui.fire(caixa, "on_enter", &EventArgs::Text("2026-12-25".into()));
    assert_eq!(log(&ui, form), ["clique frmTeste.caixa", "antigo None", "valor 2.5", &format!("data {:?}", Date::new(2026, 12, 25))]);
}

#[test]
fn a_sender_or_a_value_of_another_type_does_not_run_the_handler() {
    let (mut ui, form, _, outra) = setup();
    // Outro componente num handler que espera a `caixa`: não roda (vai para a atividade).
    ui.set_prop(outra, "on_click", "clique").unwrap();
    ui.fire(outra, "on_click", &EventArgs::None);
    // Um texto num handler que espera número: também não.
    ui.set_prop(outra, "on_resize", "valor").unwrap();
    ui.fire(outra, "on_resize", &EventArgs::Text("abc".into()));
    assert!(log(&ui, form).is_empty());
}
