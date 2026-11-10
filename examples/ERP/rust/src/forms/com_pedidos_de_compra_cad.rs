//! Os eventos do form frmComPedidosDeCompraCad: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::FrmComPedidosDeCompraCad;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmComPedidosDeCompraCad {
    /// As partes desta tela para o código comum do ERP (`crate::erp`), pelo acesso tipado.
    fn partes(&self) -> crate::erp::Partes {
        crate::erp::Partes { form: self.form.id, mestre: Some(self.cdsMestre.id), itens: Some(self.cdsItens.id), status: Some(self.lblStatus.id), telas: None }
    }

    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::abrir(ui, self.partes());
    }

    pub fn btn_novo_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::novo(ui, self.partes());
    }

    pub fn btn_salvar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::salvar(ui, self.partes());
    }

    pub fn btn_excluir_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::excluir(ui, self.partes());
    }

    pub fn btn_cancelar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::cancelar(ui, self.partes());
    }

    pub fn btn_novo_item_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::novo_item(ui, self.partes());
    }

    pub fn btn_fechar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::erp::fechar(ui, self.partes());
    }

    pub fn edt_busca_change(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        crate::erp::buscar(ui, self.partes(), args.text().unwrap_or_default());
    }
}
