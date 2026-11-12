//! Os eventos do form frmCtbRazoesLista: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::FrmCtbRazoesLista;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmCtbRazoesLista {
    /// As partes desta tela para o código comum do ERP (`crate::erp`), pelo acesso tipado.
    fn partes(&self) -> crate::erp::Partes {
        crate::erp::Partes { form: self.form.id, mestre: Some(self.cdsMestre.id), itens: None, status: Some(self.lblStatus.id), telas: None }
    }

    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::abrir(ui, self.partes());
    }

    pub fn btn_abrir_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::abrir_tela(ui, self.partes());
    }

    pub fn edt_busca_change(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        crate::erp::buscar(ui, self.partes(), args.text().unwrap_or_default());
    }

    pub fn grd_lista_dbl_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::abrir_tela(ui, self.partes());
    }
}
