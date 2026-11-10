//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use aylis::prelude::*;
use crate::generated::FrmPrincipal;

use aylis::scene3d::Model3D;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// Os níveis: quantos lingotes por lado da grade (1, 4, 9, 16, 25 e 36 ao todo).
const LADOS: [usize; 6] = [1, 2, 3, 4, 5, 6];

impl FrmPrincipal {
    /// Deixa `lado` x `lado` lingotes na grade (o primeiro é o do form), girando ao mesmo tempo.
    fn montar(&mut self, ui: &mut Ui, lado: usize) {
        let grid = self.grdLingotes.id;
        let quantos = lado * lado;
        let mut filhos: Vec<ComponentId> = ui.children(grid).to_vec();
        while filhos.len() > quantos {
            if let Some(id) = filhos.pop() {
                ui.free(id);
            }
        }
        while filhos.len() < quantos {
            let mut view = Model3D::default();
            view.background = Some(Color::rgb(0x16, 0x18, 0x21));
            filhos.push(ui.add(view, grid));
        }
        for &id in &filhos {
            ligar(ui, id);
        }
        self.grdLingotes.set_columns(ui, lado as i64);
        self.ajustar_linhas(ui, lado);
        self.lblInfo.set_text(ui, if quantos == 1 { "1 lingote".to_string() } else { format!("{quantos} lingotes") });
    }

    /// As linhas da grade dividem a altura toda (a grade só tem largura automática).
    fn ajustar_linhas(&mut self, ui: &mut Ui, lado: usize) {
        let grid = self.grdLingotes.id;
        let altura = ui.control(grid).map_or(0.0, |c| c.bounds().height);
        let gap = 2.0;
        let linha = ((altura - gap * (lado as f32 - 1.0)) / lado as f32).max(40.0);
        if (self.grdLingotes.row_height(ui) as f32 - linha).abs() > 0.5 {
            self.grdLingotes.set_row_height(ui, linha as f64);
        }
    }

    fn lado_atual(&mut self, ui: &Ui) -> usize {
        LADOS[(self.segNivel.item_index(ui).max(0) as usize).min(LADOS.len() - 1)]
    }

    pub fn nivel_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let lado = self.lado_atual(ui);
        self.montar(ui, lado);
    }

    /// Na primeira volta liga o lingote do form; depois acompanha o tamanho da tela (girar o
    /// celular muda a altura das linhas).
    pub fn ajuste_timer(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let lado = self.lado_atual(ui);
        for id in ui.children(self.grdLingotes.id).to_vec() {
            ligar(ui, id);
        }
        self.ajustar_linhas(ui, lado);
    }
}

/// Liga o lingote da abertura da IDE num `Model3D` que ainda não tem modelo.
fn ligar(ui: &mut Ui, id: ComponentId) {
    if let Some(view) = ui.get_mut::<Model3D>(id)
        && view.model().is_none()
    {
        aylis::scene3d::ingot::apply(view);
    }
}

