//! A segunda janela do mesmo form (`create_form_instance`) ganha um nome livre
//! (`frmPedido2`), como o frame repetido, e os handlers continuam os do `frmPedido`
//! (achado na conferência de 08/11).

mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::boxy;
use aylis_core::instances::form_handler;
use aylis_core::*;

struct FrmPedido {
    clicks: u32,
}

impl FrmPedido {
    fn salvar_click(&mut self, _ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.clicks += 1;
    }
}

/// A fábrica de forms do teste: monta o `frmPedido` (um form com um botão) e a instância.
struct Factory;

impl aylis_core::platform::FormFactory for Factory {
    fn create_form(&mut self, ui: &mut Ui, name: &str) -> std::result::Result<ComponentId, String> {
        let form = ui.add_root(boxy(300.0, 200.0));
        ui.set_name(form, name).map_err(|e| e.to_string())?;
        let btn = ui.add(boxy(80.0, 30.0), form);
        ui.set_name(btn, "btnSalvar").map_err(|e| e.to_string())?;
        ui.set_prop(btn, "on_click", "salvar_click").map_err(|e| e.to_string())?;
        ui.bind_form(form)?;
        Ok(form)
    }

    fn form_names(&self) -> Vec<String> {
        vec!["frmPedido".into()]
    }
}

fn setup() -> Ui {
    let mut ui = Ui::new();
    ui.register_form_class("frmPedido", |_, _| Ok(Box::new(FrmPedido { clicks: 0 })));
    ui.register_handler("frmPedido.salvar_click", form_handler(FrmPedido::salvar_click));
    ui.services_mut().forms = Some(Box::new(Factory));
    ui
}

#[test]
fn the_second_window_gets_a_free_name_and_keeps_the_handlers() {
    let mut ui = setup();
    let a = ui.create_form_instance("frmPedido").unwrap();
    let b = ui.create_form_instance("frmPedido").unwrap();
    let c = ui.create_form_instance("frmPedido").unwrap();
    assert_eq!((ui.name(a), ui.name(b), ui.name(c)), (Some("frmPedido"), Some("frmPedido2"), Some("frmPedido3")));
    // Os caminhos distinguem as janelas.
    let btn_b = ui.find_path("frmPedido2.btnSalvar").unwrap();
    assert_eq!(ui.form_of(btn_b), b);
    assert_eq!(ui.path_of(btn_b), "frmPedido2.btnSalvar");
    // Os handlers continuam os do frmPedido, cada um com a sua instância.
    assert_eq!(ui.form_class(b), Some("frmPedido"));
    ui.fire(btn_b, "on_click", &EventArgs::None);
    assert_eq!(ui.form_instance::<FrmPedido>(b).unwrap().clicks, 1);
    assert_eq!(ui.form_instance::<FrmPedido>(a).unwrap().clicks, 0);
    // O `create_form` continua achando o primeiro.
    assert_eq!(ui.create_form("frmPedido").unwrap(), a);
}

#[test]
fn a_second_window_without_instance_keeps_the_handlers_of_its_form() {
    // O backend em outra linguagem: sem a classe registrada, os handlers são por nome.
    let mut ui = Ui::new();
    ui.services_mut().forms = Some(Box::new(Factory));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.register_handler("frmPedido.salvar_click", move |ui, sender, _| s.borrow_mut().push(ui.path_of(sender)));
    let _ = ui.create_form_instance("frmPedido");
    // Sem instância registrada o `bind_form` não recusa: a fábrica devolve o form.
    let b = ui.create_form_instance("frmPedido").unwrap();
    assert_eq!(ui.name(b), Some("frmPedido2"));
    assert_eq!(ui.form_class(b), Some("frmPedido"));
    let btn = ui.find_path("frmPedido2.btnSalvar").unwrap();
    ui.fire(btn, "on_click", &EventArgs::None);
    assert_eq!(*seen.borrow(), vec!["frmPedido2.btnSalvar".to_string()]);
    // Liberada, esquece o nome de origem.
    ui.free(b);
    assert_eq!(ui.form_class(b), None);
}
