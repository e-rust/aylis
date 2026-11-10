//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;
use aylis::prelude::instances::form_handler;

pub mod components;
pub use components::*;

#[path = "../src/forms/ibge.rs"]
pub mod ibge;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmIBGE", FrmIBGE::bind);
    ui.register_handler("frmIBGE.form_show", form_handler(FrmIBGE::form_show));
    ui.register_handler("frmIBGE.estado_mudou", form_handler(FrmIBGE::estado_mudou));
    ui.register_handler("frmIBGE.municipio_mudou", form_handler(FrmIBGE::municipio_mudou));
}
