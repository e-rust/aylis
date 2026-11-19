//! Eventos do form frmCalculadora: os handlers são métodos da struct do form, e cada janela
//! tem a sua calculadora (`self.data`).

use aylis::prelude::*;

use crate::motor::{Calculadora, botao_do_teclado, formatar};
use crate::generated::{Button, FrmCalculadora, ListView, ListViewItem};

/// O estado do form: a calculadora desta janela (duas janelas, duas contas).
pub struct Data {
    calc: Calculadora,
}

impl Default for Data {
    fn default() -> Self {
        Data { calc: Calculadora::new() }
    }
}

impl FrmCalculadora {
    /// Os botões que só funcionam sem erro no visor.
    fn sem_erro(&self) -> [Button; 13] {
        [
            self.btnMMais,
            self.btnMMenos,
            self.btnMS,
            self.btnPorcento,
            self.btnInverso,
            self.btnQuadrado,
            self.btnRaiz,
            self.btnDividir,
            self.btnMultiplicar,
            self.btnSubtrair,
            self.btnSomar,
            self.btnSinal,
            self.btnVirgula,
        ]
    }

    /// Põe o estado da calculadora na tela.
    fn atualizar(&mut self, ui: &mut Ui) {
        let calc = self.data.calc.clone();
        let visor = calc.visor();
        // O número encolhe para caber, como no Windows.
        let n = visor.chars().count() as i64;
        let tamanho = if calc.tem_erro() {
            26
        } else if n <= 11 {
            46
        } else {
            (46 * 11 / n).max(22)
        };
        self.lblVisor.set_text(ui, visor);
        self.lblVisor.set_font_size(ui, tamanho as f64);
        self.lblExpressao.set_text(ui, calc.expressao().to_string());
        let erro = calc.tem_erro();
        for botao in self.sem_erro() {
            botao.set_enabled(ui, !erro);
        }
        for botao in [&self.btnMC, &self.btnMR] {
            botao.set_enabled(ui, !erro && !calc.memoria.is_empty());
        }
        let item = |title: String, detail: String| ListViewItem { title, detail };
        let historico: Vec<ListViewItem> = if calc.historico.is_empty() {
            vec![item("Ainda não há histórico".into(), String::new())]
        } else {
            calc.historico.iter().rev().map(|c| item(formatar(c.resultado), c.expressao.clone())).collect()
        };
        self.lvHistorico.set_items(ui, &historico);
        let memoria: Vec<ListViewItem> = if calc.memoria.is_empty() {
            vec![item("Não há nada salvo na memória".into(), String::new())]
        } else {
            calc.memoria.iter().rev().map(|m| item(formatar(*m), String::new())).collect()
        };
        self.lvMemoria.set_items(ui, &memoria);
    }

    /// Aperta um botão pelo texto dele.
    fn apertar(&mut self, ui: &mut Ui, texto: &str) {
        if self.data.calc.tecla(texto) {
            self.atualizar(ui);
        }
    }

    pub fn form_key_down(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = sender;
        let EventArgs::Key(k) = args else { return };
        match botao_do_teclado(&nome_da_tecla(k.key), k.modifiers.ctrl, k.modifiers.shift) {
            Some("historico") => self.historico_click(ui, self.btnHistorico),
            Some(texto) => self.apertar(ui, texto),
            None => {}
        }
    }

    pub fn limpar_historico_click(&mut self, ui: &mut Ui, sender: Button) {
        let _ = sender;
        self.data.calc.historico.clear();
        self.atualizar(ui);
    }

    pub fn historico_item_click(&mut self, ui: &mut Ui, sender: ListView, value: i64) {
        let _ = sender;
        // A lista mostra a conta mais nova em cima.
        let valor = indice(value).and_then(|i| self.data.calc.historico.iter().rev().nth(i).map(|c| c.resultado));
        if let Some(v) = valor {
            self.data.calc.usar(v);
            self.atualizar(ui);
        }
    }

    pub fn memoria_item_click(&mut self, ui: &mut Ui, sender: ListView, value: i64) {
        let _ = sender;
        let valor = indice(value).and_then(|i| self.data.calc.memoria.iter().rev().nth(i).copied());
        if let Some(v) = valor {
            self.data.calc.usar(v);
            self.atualizar(ui);
        }
    }

    pub fn historico_click(&mut self, ui: &mut Ui, sender: Button) {
        let _ = sender;
        self.pnlLateral.set_visible(ui, !self.pnlLateral.visible(ui));
    }

    pub fn tecla_click(&mut self, ui: &mut Ui, sender: Button) {
        let texto = sender.text(ui);
        self.apertar(ui, &texto);
    }
}

/// O índice do item clicado (negativo: nenhum).
fn indice(value: i64) -> Option<usize> {
    usize::try_from(value).ok()
}

/// O nome que o aylis dá à tecla (o mesmo que as outras linguagens recebem no evento).
fn nome_da_tecla(key: Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::F(n) => format!("f{n}"),
        other => format!("{other:?}").to_lowercase(),
    }
}

