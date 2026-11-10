//! Eventos do form frmPrincipal: abas como as do navegador. Cada aba tem a sua página
//! (`TabStrip::add_page`); arrastar a aba para fora abre outra instância deste form com ela,
//! e arrastar para a faixa de outra janela leva a aba (e a página) para lá.

use std::sync::atomic::{AtomicUsize, Ordering};

use aylis::prelude::*;
use aylis::controls::{Label, Memo, Panel, TabStrip};
use crate::generated::FrmPrincipal;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// O número da próxima aba, contado entre todas as janelas.
static PROXIMA: AtomicUsize = AtomicUsize::new(1);

/// As cores das páginas, uma por aba (para reconhecer a página na prévia do arrasto).
const CORES: [&str; 6] = ["#E8F0FE", "#FDECEA", "#E6F4EA", "#FEF7E0", "#F3E8FD", "#E4F7FB"];

impl FrmPrincipal {
    /// Abre com três abas. A janela nascida de uma aba arrancada já chega com ela: não cria.
    pub fn principal_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let strip = self.tbsAbas.id;
        if ui.get::<TabStrip>(strip).is_some_and(|s| !s.tabs().is_empty()) {
            self.abas_change(ui, strip, &EventArgs::None);
            return;
        }
        for _ in 0..3 {
            nova_aba(ui, strip);
        }
        if let Some(s) = ui.get_mut::<TabStrip>(strip) {
            s.tab_index = 0;
        }
        TabStrip::sync_pages(ui, strip);
        self.abas_change(ui, strip, &EventArgs::None);
    }

    /// O "+": uma aba nova nesta janela.
    pub fn abas_add(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        nova_aba(ui, sender);
        self.abas_change(ui, sender, &EventArgs::None);
    }

    /// O título da janela segue a aba escolhida.
    pub fn abas_change(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        let titulo = ui.get::<TabStrip>(sender).and_then(|s| s.tabs().get(s.tab_index.max(0) as usize).map(|t| t.title.clone()));
        if let Some(titulo) = titulo {
            self.form.set_caption(ui, format!("{titulo} — Abas"));
        }
    }
}

/// Cria a página (um título e um texto que dá para editar, para ver que ele vai junto) e a
/// aba dela na faixa `strip`.
fn nova_aba(ui: &mut Ui, strip: ComponentId) {
    let n = PROXIMA.fetch_add(1, Ordering::Relaxed);
    let Some(host) = ui.get::<TabStrip>(strip).and_then(|s| s.page_host) else { return };
    let page = ui.add(Panel::default(), host);
    ui.control_mut(page).unwrap().set_padding(16.0);
    let _ = ui.set_prop(page, "fill", CORES[(n - 1) % CORES.len()]);
    let titulo = ui.add(Label::new(format!("Página {n}")), page);
    ui.control_mut(titulo).unwrap().set_align(Align::Top).set_height(40.0);
    let _ = ui.set_prop(titulo, "font_size", 22.0);
    let texto = ui.add(Memo::default(), page);
    ui.control_mut(texto).unwrap().set_align(Align::Client).set_margins([0.0, 8.0, 0.0, 0.0]);
    let _ = ui.set_prop(texto, "text", format!("Texto da página {n}.\nEscreva algo aqui e arraste a aba para outra janela: o texto vai junto."));
    TabStrip::add_page(ui, strip, &format!("Aba {n}"), "", page);
}
