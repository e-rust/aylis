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
    ui.register_handler("frmPrincipal.frmPrincipal_show", aylis::prelude::instances::form_handler(FrmPrincipal::frmPrincipal_show));
    ui.register_handler("frmPrincipal.btnAbrir_click", aylis::prelude::instances::form_handler(FrmPrincipal::btnAbrir_click));
    ui.register_handler("frmPrincipal.btnTocar_click", aylis::prelude::instances::form_handler(FrmPrincipal::btnTocar_click));
    ui.register_handler("frmPrincipal.segGrade_change", aylis::prelude::instances::form_handler(FrmPrincipal::segGrade_change));
    ui.register_handler("frmPrincipal.edtTexto_change", aylis::prelude::instances::form_handler(FrmPrincipal::edtTexto_change));
    ui.register_handler("frmPrincipal.chkInverter_change", aylis::prelude::instances::form_handler(FrmPrincipal::chkInverter_change));
    ui.register_handler("frmPrincipal.chkMover_change", aylis::prelude::instances::form_handler(FrmPrincipal::chkMover_change));
    ui.register_handler("frmPrincipal.chkVideo_change", aylis::prelude::instances::form_handler(FrmPrincipal::chkVideo_change));
    ui.register_handler("frmPrincipal.grdVideos_resize", aylis::prelude::instances::form_handler(FrmPrincipal::grdVideos_resize));
    ui.register_handler("frmPrincipal.vpVideo_error", aylis::prelude::instances::form_handler(FrmPrincipal::vpVideo_error));
    ui.register_handler("frmPrincipal.dlgAbrir_select", aylis::prelude::instances::form_handler(FrmPrincipal::dlgAbrir_select));
}
