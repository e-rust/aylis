//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! O que a página pediu de código (o resto é só o form, montado na IDE): o lingote 3D do
//! hero (o modelo é montado em Rust), a documentação e o "Experimentar" abrindo a IDE lite.

use aylis::prelude::*;
use crate::generated::{FrmDocumentacao, FrmPrincipal};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmPrincipal {
    /// O lingote da abertura da IDE no hero, girando. No navegador ele vem do wasm da família 3D
    /// (`aylis-tool demos`), num `<iframe>` carregado depois da página: o 3D (o wgpu, ~5 MB) fica
    /// fora do wasm da página, que abre rápido. Fora do navegador, o `Model3D` de verdade.
    pub fn abrir(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (sender, args);
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ui.set_prop(self.wbLingote.id, "url", "demos/3d/index.html?class=Model3D&scene=ingot&fill=1&bg=0F1117".to_string());
            let _ = ui.set_prop(self.wbLingote.id, "visible", true);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut view = aylis::scene3d::Model3D::default();
            aylis::scene3d::ingot::apply(&mut view);
            let id = ui.add(view, self.layLingote.id);
            if let Some(cb) = ui.control_mut(id) {
                cb.set_align(Align::Client);
            }
        }
    }

    /// "Experimentar": a IDE lite (`aylis-tool ide-web --out <página>/ide`), ao lado da página.
    pub fn experimentar(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = aylis::controls::AppLink::open_url(ui, "ide/");
    }

    /// "Baixar a IDE": as versões publicadas no GitHub (o instalador de cada uma).
    pub fn baixar(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = aylis::controls::AppLink::open_url(ui, "https://github.com/e-rust/aylis/releases");
    }

    /// "Documentação": os guias e a referência de cada componente (`aylis-tool docs`).
    pub fn abrir_documentacao(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if let Ok(form) = FrmDocumentacao::create(ui) {
            let _ = ui.show(form);
        }
    }
}

