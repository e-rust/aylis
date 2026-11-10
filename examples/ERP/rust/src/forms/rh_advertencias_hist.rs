//! Os eventos do form frmRhAdvertenciasHist: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::FrmRhAdvertenciasHist;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmRhAdvertenciasHist {
    /// As partes desta tela para o código comum do ERP (`crate::erp`), pelo acesso tipado.
    fn partes(&self) -> crate::erp::Partes {
        crate::erp::Partes { form: self.form.id, mestre: None, itens: Some(self.cdsItens.id), status: Some(self.lblStatus.id), telas: None }
    }

    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::abrir(ui, self.partes());
    }

    pub fn edt_busca_change(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        crate::erp::buscar(ui, self.partes(), args.text().unwrap_or_default());
    }
}
