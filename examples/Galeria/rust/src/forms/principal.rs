//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! A galeria: um cartão por componente visual registrado, com o componente de verdade
//! dentro (passe o mouse, clique, digite), como a Galeria da IDE. Publicada para a web, é a
//! galeria em wasm da página do aylis.

use aylis::prelude::*;
use crate::generated::FrmPrincipal;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// Margem em volta do componente e a altura do título do cartão.
const PAD: f32 = 14.0;
const TITULO: f32 = 30.0;
/// O tamanho de quem não tem tamanho próprio (alinhado ao pai).
const EXEMPLO: (f32, f32) = (240.0, 140.0);

/// Classes que não entram: os forms, as que são parte de outra (item de menu, aba,
/// coluna) e as que mexem no sistema.
const FORA: &[&str] = &["Form", "DataModule", "MenuItem", "TabItem", "TitleBar", "SlideItem"];

impl FrmPrincipal {
    /// Um cartão por classe visual, na ordem das categorias da paleta.
    pub fn montar(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (sender, args);
        let grade = self.fluCartoes.id;
        let mut classes: Vec<(String, String)> = ui
            .registry()
            .iter()
            .filter(|c| !FORA.contains(&c.name))
            .map(|c| (c.category.to_string(), c.name.to_string()))
            .collect();
        classes.sort();
        let mut quantos = 0;
        for (_, classe) in classes {
            if cartao(ui, grade, &classe) {
                quantos += 1;
            }
        }
        self.lblConta.set_text(ui, format!("{quantos} componentes"));
    }

    /// Mostra só os cartões cujo nome tem o texto da busca.
    pub fn filtrar(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (sender, args);
        let busca = self.edtBusca.text(ui).to_lowercase();
        let grade = self.fluCartoes.id;
        let mut visiveis = 0;
        for cartao in ui.children(grade).to_vec() {
            let nome = ui.name(cartao).unwrap_or_default().trim_start_matches("crd").to_lowercase();
            let mostra = busca.is_empty() || nome.contains(&busca);
            if let Some(cb) = ui.control_mut(cartao) {
                cb.set_visible(mostra);
            }
            visiveis += usize::from(mostra);
        }
        self.lblConta.set_text(ui, format!("{visiveis} componentes"));
    }
}

/// O cartão de uma classe: o título e o componente criado de verdade. Os não visuais saem.
fn cartao(ui: &mut Ui, grade: ComponentId, classe: &str) -> bool {
    let Ok(cartao) = ui.create("Panel", Some(grade)) else { return false };
    let _ = ui.set_name(cartao, &format!("crd{classe}"));
    let _ = ui.set_prop(cartao, "style", "card");
    let Ok(id) = ui.create(classe, Some(cartao)) else {
        ui.free(cartao);
        return false;
    };
    if ui.control(id).is_none() {
        ui.free(cartao);
        return false;
    }
    // O texto inicial é o nome da classe, como no designer.
    for prop in ["text", "caption"] {
        if let Ok(PropValue::String(atual)) = ui.get_prop(id, prop) {
            if atual.is_empty() {
                let _ = ui.set_prop(id, prop, classe);
            }
        }
    }
    let (w, h, align) = ui.control(id).map(|cb| (cb.width(), cb.height(), cb.align())).unwrap_or((0.0, 0.0, Align::None));
    let proprio = |v: f32, exemplo: f32| if v >= 1.0 { v.min(1100.0) } else { exemplo };
    let (w, h) = match align {
        Align::None => (proprio(w, EXEMPLO.0), proprio(h, EXEMPLO.1)),
        Align::Top | Align::Bottom => (360.0, proprio(h, 40.0)),
        Align::Left | Align::Right => (proprio(w, 200.0), EXEMPLO.1),
        _ => EXEMPLO,
    };
    if let Some(cb) = ui.control_mut(id) {
        cb.set_align(Align::None);
        cb.set_rect(PAD, TITULO, w, h.min(420.0));
    }
    let largura = (w + 2.0 * PAD).max(220.0);
    let titulo = ui.add(Label::new(classe), cartao);
    if let Some(cb) = ui.control_mut(titulo) {
        cb.set_rect(PAD, 6.0, largura - 2.0 * PAD, 22.0);
    }
    let _ = ui.set_prop(titulo, "bold", true);
    if let Some(cb) = ui.control_mut(cartao) {
        cb.set_size(largura, h.min(420.0) + TITULO + PAD);
    }
    true
}

