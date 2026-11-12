//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;

pub mod components;
pub use components::*;

#[path = "../src/forms/paint.rs"]
pub mod paint;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmPaint", FrmPaint::bind);
    ui.register_handler("frmPaint.novo_click", aylis::prelude::instances::form_handler(FrmPaint::novo_click));
    ui.register_handler("frmPaint.abrir_click", aylis::prelude::instances::form_handler(FrmPaint::abrir_click));
    ui.register_handler("frmPaint.salvar_click", aylis::prelude::instances::form_handler(FrmPaint::salvar_click));
    ui.register_handler("frmPaint.salvar_como_click", aylis::prelude::instances::form_handler(FrmPaint::salvar_como_click));
    ui.register_handler("frmPaint.sair_click", aylis::prelude::instances::form_handler(FrmPaint::sair_click));
    ui.register_handler("frmPaint.desfazer_click", aylis::prelude::instances::form_handler(FrmPaint::desfazer_click));
    ui.register_handler("frmPaint.refazer_click", aylis::prelude::instances::form_handler(FrmPaint::refazer_click));
    ui.register_handler("frmPaint.recortar_click", aylis::prelude::instances::form_handler(FrmPaint::recortar_click));
    ui.register_handler("frmPaint.copiar_click", aylis::prelude::instances::form_handler(FrmPaint::copiar_click));
    ui.register_handler("frmPaint.colar_click", aylis::prelude::instances::form_handler(FrmPaint::colar_click));
    ui.register_handler("frmPaint.excluir_click", aylis::prelude::instances::form_handler(FrmPaint::excluir_click));
    ui.register_handler("frmPaint.selecionar_tudo_click", aylis::prelude::instances::form_handler(FrmPaint::selecionar_tudo_click));
    ui.register_handler("frmPaint.limpar_click", aylis::prelude::instances::form_handler(FrmPaint::limpar_click));
    ui.register_handler("frmPaint.ampliar_click", aylis::prelude::instances::form_handler(FrmPaint::ampliar_click));
    ui.register_handler("frmPaint.reduzir_click", aylis::prelude::instances::form_handler(FrmPaint::reduzir_click));
    ui.register_handler("frmPaint.tamanho_real_click", aylis::prelude::instances::form_handler(FrmPaint::tamanho_real_click));
    ui.register_handler("frmPaint.ferramenta_click", aylis::prelude::instances::form_handler(FrmPaint::ferramenta_click));
    ui.register_handler("frmPaint.preencher_change", aylis::prelude::instances::form_handler(FrmPaint::preencher_change));
    ui.register_handler("frmPaint.espessura_change", aylis::prelude::instances::form_handler(FrmPaint::espessura_change));
    ui.register_handler("frmPaint.cor1_click", aylis::prelude::instances::form_handler(FrmPaint::cor1_click));
    ui.register_handler("frmPaint.cor2_click", aylis::prelude::instances::form_handler(FrmPaint::cor2_click));
    ui.register_handler("frmPaint.paleta_click", aylis::prelude::instances::form_handler(FrmPaint::paleta_click));
    ui.register_handler("frmPaint.desenho_change", aylis::prelude::instances::form_handler(FrmPaint::desenho_change));
    ui.register_handler("frmPaint.desenho_pick", aylis::prelude::instances::form_handler(FrmPaint::desenho_pick));
    ui.register_handler("frmPaint.desenho_cursor", aylis::prelude::instances::form_handler(FrmPaint::desenho_cursor));
    ui.register_handler("frmPaint.abrir_select", aylis::prelude::instances::form_handler(FrmPaint::abrir_select));
    ui.register_handler("frmPaint.salvar_select", aylis::prelude::instances::form_handler(FrmPaint::salvar_select));
}
