//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;
use aylis::prelude::instances::form_handler;

pub mod components;
pub use components::*;

#[path = "../src/forms/calculadora.rs"]
pub mod calculadora;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmCalculadora", FrmCalculadora::bind);
    ui.register_handler("frmCalculadora.form_key_down", form_handler(FrmCalculadora::form_key_down));
    ui.register_handler("frmCalculadora.limpar_historico_click", form_handler(FrmCalculadora::limpar_historico_click));
    ui.register_handler("frmCalculadora.historico_item_click", form_handler(FrmCalculadora::historico_item_click));
    ui.register_handler("frmCalculadora.memoria_item_click", form_handler(FrmCalculadora::memoria_item_click));
    ui.register_handler("frmCalculadora.historico_click", form_handler(FrmCalculadora::historico_click));
    ui.register_handler("frmCalculadora.tecla_click", form_handler(FrmCalculadora::tecla_click));
}
