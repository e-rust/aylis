//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;

pub mod components;
pub use components::*;

#[path = "../src/forms/player.rs"]
pub mod player;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmPlayer", FrmPlayer::bind);
    ui.register_handler("frmPlayer.abrir_click", aylis::prelude::instances::form_handler(FrmPlayer::abrir_click));
    ui.register_handler("frmPlayer.limpar_fila_click", aylis::prelude::instances::form_handler(FrmPlayer::limpar_fila_click));
    ui.register_handler("frmPlayer.sair_click", aylis::prelude::instances::form_handler(FrmPlayer::sair_click));
    ui.register_handler("frmPlayer.reproduzir_click", aylis::prelude::instances::form_handler(FrmPlayer::reproduzir_click));
    ui.register_handler("frmPlayer.parar_click", aylis::prelude::instances::form_handler(FrmPlayer::parar_click));
    ui.register_handler("frmPlayer.anterior_click", aylis::prelude::instances::form_handler(FrmPlayer::anterior_click));
    ui.register_handler("frmPlayer.proxima_click", aylis::prelude::instances::form_handler(FrmPlayer::proxima_click));
    ui.register_handler("frmPlayer.embaralhar_click", aylis::prelude::instances::form_handler(FrmPlayer::embaralhar_click));
    ui.register_handler("frmPlayer.repetir_click", aylis::prelude::instances::form_handler(FrmPlayer::repetir_click));
    ui.register_handler("frmPlayer.agora_click", aylis::prelude::instances::form_handler(FrmPlayer::agora_click));
    ui.register_handler("frmPlayer.fila_click", aylis::prelude::instances::form_handler(FrmPlayer::fila_click));
    ui.register_handler("frmPlayer.posicao_seek", aylis::prelude::instances::form_handler(FrmPlayer::posicao_seek));
    ui.register_handler("frmPlayer.mudo_click", aylis::prelude::instances::form_handler(FrmPlayer::mudo_click));
    ui.register_handler("frmPlayer.volume_change", aylis::prelude::instances::form_handler(FrmPlayer::volume_change));
    ui.register_handler("frmPlayer.video_fim", aylis::prelude::instances::form_handler(FrmPlayer::video_fim));
    ui.register_handler("frmPlayer.video_erro", aylis::prelude::instances::form_handler(FrmPlayer::video_erro));
    ui.register_handler("frmPlayer.fila_item_click", aylis::prelude::instances::form_handler(FrmPlayer::fila_item_click));
    ui.register_handler("frmPlayer.abrir_select", aylis::prelude::instances::form_handler(FrmPlayer::abrir_select));
    ui.register_handler("frmPlayer.posicao_timer", aylis::prelude::instances::form_handler(FrmPlayer::posicao_timer));
}
