//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;

pub mod components;
pub use components::*;

#[path = "../src/forms/principal.rs"]
pub mod principal;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmPrincipal", FrmPrincipal::bind);
    ui.register_handler("frmPrincipal.principal_show", aylis::prelude::instances::form_handler(FrmPrincipal::principal_show));
    ui.register_handler("frmPrincipal.abas_change", aylis::prelude::instances::form_handler(FrmPrincipal::abas_change));
    ui.register_handler("frmPrincipal.abas_add", aylis::prelude::instances::form_handler(FrmPrincipal::abas_add));
}
