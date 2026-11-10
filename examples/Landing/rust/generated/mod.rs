//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;
use aylis::prelude::instances::form_handler;

pub mod components;
pub use components::*;

#[path = "../src/forms/principal.rs"]
pub mod principal;

#[path = "../src/forms/documentacao.rs"]
pub mod documentacao;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmPrincipal", FrmPrincipal::bind);
    ui.register_handler("frmPrincipal.abrir", form_handler(FrmPrincipal::abrir));
    ui.register_handler("frmPrincipal.abrir_documentacao", form_handler(FrmPrincipal::abrir_documentacao));
    ui.register_handler("frmPrincipal.experimentar", form_handler(FrmPrincipal::experimentar));
    ui.register_handler("frmPrincipal.baixar", form_handler(FrmPrincipal::baixar));
    ui.register_form_class("frmDocumentacao", FrmDocumentacao::bind);
    ui.register_handler("frmDocumentacao.form_show", form_handler(FrmDocumentacao::form_show));
    ui.register_handler("frmDocumentacao.navegar", form_handler(FrmDocumentacao::navegar));
    ui.register_handler("frmDocumentacao.voltar", form_handler(FrmDocumentacao::voltar));
    ui.register_handler("frmDocumentacao.buscar", form_handler(FrmDocumentacao::buscar));
    ui.register_handler("frmDocumentacao.abrir_no", form_handler(FrmDocumentacao::abrir_no));
    ui.register_handler("frmDocumentacao.abrir_achado", form_handler(FrmDocumentacao::abrir_achado));
    ui.register_handler("frmDocumentacao.ir_para_titulo", form_handler(FrmDocumentacao::ir_para_titulo));
    ui.register_handler("frmDocumentacao.demo_mensagem", form_handler(FrmDocumentacao::demo_mensagem));
    ui.register_handler("frmDocumentacao.anterior", form_handler(FrmDocumentacao::anterior));
    ui.register_handler("frmDocumentacao.proxima", form_handler(FrmDocumentacao::proxima));
    ui.register_handler("frmDocumentacao.seguir_link", form_handler(FrmDocumentacao::seguir_link));
}
