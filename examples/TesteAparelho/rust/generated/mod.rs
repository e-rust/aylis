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
    ui.register_handler("frmPrincipal.cam_ligar_click", aylis::prelude::instances::form_handler(FrmPrincipal::cam_ligar_click));
    ui.register_handler("frmPrincipal.cam_trocar_click", aylis::prelude::instances::form_handler(FrmPrincipal::cam_trocar_click));
    ui.register_handler("frmPrincipal.cam_foto_click", aylis::prelude::instances::form_handler(FrmPrincipal::cam_foto_click));
    ui.register_handler("frmPrincipal.cam_ciclo_click", aylis::prelude::instances::form_handler(FrmPrincipal::cam_ciclo_click));
    ui.register_handler("frmPrincipal.cam_qualidade_change", aylis::prelude::instances::form_handler(FrmPrincipal::cam_qualidade_change));
    ui.register_handler("frmPrincipal.cam_flash_click", aylis::prelude::instances::form_handler(FrmPrincipal::cam_flash_click));
    ui.register_handler("frmPrincipal.cam_pronta", aylis::prelude::instances::form_handler(FrmPrincipal::cam_pronta));
    ui.register_handler("frmPrincipal.mic_ligar_click", aylis::prelude::instances::form_handler(FrmPrincipal::mic_ligar_click));
    ui.register_handler("frmPrincipal.mic_gravar_click", aylis::prelude::instances::form_handler(FrmPrincipal::mic_gravar_click));
    ui.register_handler("frmPrincipal.mic_apps_click", aylis::prelude::instances::form_handler(FrmPrincipal::mic_apps_click));
    ui.register_handler("frmPrincipal.vid_tocar_click", aylis::prelude::instances::form_handler(FrmPrincipal::vid_tocar_click));
    ui.register_handler("frmPrincipal.vid_parar_click", aylis::prelude::instances::form_handler(FrmPrincipal::vid_parar_click));
    ui.register_handler("frmPrincipal.vid_ciclo_click", aylis::prelude::instances::form_handler(FrmPrincipal::vid_ciclo_click));
    ui.register_handler("frmPrincipal.not_simples_click", aylis::prelude::instances::form_handler(FrmPrincipal::not_simples_click));
    ui.register_handler("frmPrincipal.not_botoes_click", aylis::prelude::instances::form_handler(FrmPrincipal::not_botoes_click));
    ui.register_handler("frmPrincipal.not_progresso_click", aylis::prelude::instances::form_handler(FrmPrincipal::not_progresso_click));
    ui.register_handler("frmPrincipal.not_form_click", aylis::prelude::instances::form_handler(FrmPrincipal::not_form_click));
    ui.register_handler("frmPrincipal.not_acao", aylis::prelude::instances::form_handler(FrmPrincipal::not_acao));
    ui.register_handler("frmPrincipal.tema_click", aylis::prelude::instances::form_handler(FrmPrincipal::tema_click));
    ui.register_handler("frmPrincipal.fps_click", aylis::prelude::instances::form_handler(FrmPrincipal::fps_click));
    ui.register_handler("frmPrincipal.abrir_site_click", aylis::prelude::instances::form_handler(FrmPrincipal::abrir_site_click));
    ui.register_handler("frmPrincipal.compartilhar_click", aylis::prelude::instances::form_handler(FrmPrincipal::compartilhar_click));
    ui.register_handler("frmPrincipal.vibrar_click", aylis::prelude::instances::form_handler(FrmPrincipal::vibrar_click));
    ui.register_handler("frmPrincipal.servico_click", aylis::prelude::instances::form_handler(FrmPrincipal::servico_click));
    ui.register_handler("frmPrincipal.servico_mensagem", aylis::prelude::instances::form_handler(FrmPrincipal::servico_mensagem));
    ui.register_handler("frmPrincipal.link_erro", aylis::prelude::instances::form_handler(FrmPrincipal::link_erro));
    ui.register_handler("frmPrincipal.discar_click", aylis::prelude::instances::form_handler(FrmPrincipal::discar_click));
    ui.register_handler("frmPrincipal.chamada_estado", aylis::prelude::instances::form_handler(FrmPrincipal::chamada_estado));
    ui.register_handler("frmPrincipal.link_recebido", aylis::prelude::instances::form_handler(FrmPrincipal::link_recebido));
    ui.register_handler("frmPrincipal.compartilhado_recebido", aylis::prelude::instances::form_handler(FrmPrincipal::compartilhado_recebido));
    ui.register_handler("frmPrincipal.tick_timer", aylis::prelude::instances::form_handler(FrmPrincipal::tick_timer));
}
