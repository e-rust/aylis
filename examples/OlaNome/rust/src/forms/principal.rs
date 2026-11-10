//! Os eventos do form frmPrincipal: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).
//!
//! O tutorial "Seu primeiro projeto" da documentação é feito com este projeto.

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::FrmPrincipal;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmPrincipal {
    /// O clique em "Saudar": lê o nome e escreve a saudação.
    pub fn btn_saudar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let nome = self.edtNome.text(ui);
        let nome = if nome.trim().is_empty() { "mundo".to_string() } else { nome.trim().to_string() };
        self.lblResposta.set_text(ui, format!("Olá, {nome}!"));
    }
}
