//! Eventos do form frmPaint: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! O desenho em si é do componente `PaintCanvas` (pcDesenho); aqui ficam o menu, as
//! ferramentas, as cores, o zoom, o arquivo e a barra de status.

use std::cell::RefCell;

use aylis::prelude::*;
use crate::generated::{Button, FrmPaint};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
struct Estado {
    arquivo: String,
    modificado: bool,
    cor_ativa: u8,
    zoom: i64,
}

thread_local! {
    static ESTADO: RefCell<Estado> = const { RefCell::new(Estado { arquivo: String::new(), modificado: false, cor_ativa: 1, zoom: 100 }) };
}

/// O nome do arquivo sem a pasta.
fn nome_do_arquivo(caminho: &str) -> &str {
    caminho.rsplit(['\\', '/']).next().unwrap_or(caminho)
}

impl FrmPaint {
    /// Os botões de ferramenta e a ferramenta de cada um.
    fn ferramentas(&self) -> [(Button, PaintTool); 9] {
        [
            (self.btnSelecionar, PaintTool::Select),
            (self.btnLapis, PaintTool::Pencil),
            (self.btnPincel, PaintTool::Brush),
            (self.btnBorracha, PaintTool::Eraser),
            (self.btnBalde, PaintTool::Fill),
            (self.btnContaGotas, PaintTool::Picker),
            (self.btnLinha, PaintTool::Line),
            (self.btnRetangulo, PaintTool::Rectangle),
            (self.btnElipse, PaintTool::Ellipse),
        ]
    }

    /// "*Sem título - Paint", como o Paint do Windows.
    fn titulo(&mut self, ui: &mut Ui) {
        let (arquivo, modificado) = ESTADO.with(|e| (e.borrow().arquivo.clone(), e.borrow().modificado));
        let nome = if arquivo.is_empty() { "Sem título" } else { nome_do_arquivo(&arquivo) };
        self.set_caption(ui, format!("{}{nome} - Paint", if modificado { "*" } else { "" }));
    }

    /// Desfazer, refazer e o tamanho da imagem acompanham o desenho.
    fn atualizar_edicao(&mut self, ui: &mut Ui) {
        let desfazer = self.pcDesenho.can_undo(ui).unwrap_or(false);
        let refazer = self.pcDesenho.can_redo(ui).unwrap_or(false);
        self.mnuDesfazer.set_enabled(ui, desfazer);
        self.btnDesfazer.set_enabled(ui, desfazer);
        self.mnuRefazer.set_enabled(ui, refazer);
        self.btnRefazer.set_enabled(ui, refazer);
        let tamanho = self.pcDesenho.size(ui).ok().flatten().map(|s| format!("{} × {}px", s.width, s.height)).unwrap_or_default();
        self.lblTamanho.set_text(ui, tamanho);
    }

    fn marcar_modificado(&mut self, ui: &mut Ui) {
        ESTADO.with(|e| e.borrow_mut().modificado = true);
        self.titulo(ui);
        self.atualizar_edicao(ui);
    }

    fn aplicar_zoom(&mut self, ui: &mut Ui, zoom: i64) {
        ESTADO.with(|e| e.borrow_mut().zoom = zoom);
        self.pcDesenho.set_zoom(ui, zoom as f64 / 100.0);
        self.lblZoom.set_text(ui, format!("{zoom}%"));
    }

    fn salvar_em(&mut self, ui: &mut Ui, caminho: &str) {
        // O método devolve o motivo da falha para a mensagem.
        match self.pcDesenho.save(ui, caminho) {
            Ok(_) => {
                ESTADO.with(|e| {
                    let mut e = e.borrow_mut();
                    e.arquivo = caminho.into();
                    e.modificado = false;
                });
                self.titulo(ui);
            }
            Err(erro) => {
                let _ = message_dlg(ui, &format!("Não foi possível salvar a imagem.\n{erro}"), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
            }
        }
    }

    pub fn novo_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.new(ui, 640, 400);
        ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.arquivo.clear();
            e.modificado = false;
        });
        self.titulo(ui);
        self.atualizar_edicao(ui);
    }

    pub fn abrir_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        self.dlgAbrir.execute(ui);
    }

    pub fn salvar_click(&mut self, ui: &mut Ui, _sender: ComponentId) {
        let arquivo = ESTADO.with(|e| e.borrow().arquivo.clone());
        if arquivo.is_empty() {
            // Sem arquivo ainda: o mesmo do "Salvar como".
            self.dlgSalvar.execute(ui);
        } else {
            self.salvar_em(ui, &arquivo);
        }
    }

    pub fn salvar_como_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        self.dlgSalvar.execute(ui);
    }

    pub fn sair_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        self.close(ui);
    }

    pub fn desfazer_click(&mut self, ui: &mut Ui, _sender: ComponentId) {
        let _ = self.pcDesenho.undo(ui);
        self.marcar_modificado(ui);
    }

    pub fn refazer_click(&mut self, ui: &mut Ui, _sender: ComponentId) {
        let _ = self.pcDesenho.redo(ui);
        self.marcar_modificado(ui);
    }

    pub fn limpar_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.clear(ui);
        self.marcar_modificado(ui);
    }

    pub fn ampliar_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let zoom = ESTADO.with(|e| e.borrow().zoom);
        self.aplicar_zoom(ui, (zoom * 2).min(800));
    }

    pub fn reduzir_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let zoom = ESTADO.with(|e| e.borrow().zoom);
        self.aplicar_zoom(ui, (zoom / 2).max(25));
    }

    pub fn tamanho_real_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        self.aplicar_zoom(ui, 100);
    }

    pub fn ferramenta_click(&mut self, ui: &mut Ui, sender: Button) {
        let Some((botao, ferramenta)) = self.ferramentas().into_iter().find(|(b, _)| *b == sender) else { return };
        self.pcDesenho.set_tool(ui, ferramenta);
        self.marcar_ferramenta(ui, botao);
    }

    /// Só o botão da ferramenta escolhida fica marcado.
    fn marcar_ferramenta(&mut self, ui: &mut Ui, escolhido: Button) {
        for (botao, _) in self.ferramentas() {
            botao.set_checked(ui, botao == escolhido);
        }
    }

    /// A cor que a paleta troca fica com o anel azul, como no Paint.
    fn marcar_cor(&mut self, ui: &mut Ui, cor: u8) {
        ESTADO.with(|e| e.borrow_mut().cor_ativa = cor);
        for (circulo, n) in [(self.btnCor1, 1), (self.btnCor2, 2)] {
            circulo.set_stroke(ui, if n == cor { Color::rgb(0x00, 0x67, 0xC0) } else { Color::rgb(0x9A, 0x9A, 0x9A) });
            circulo.set_stroke_width(ui, if n == cor { 3.0 } else { 1.0 });
        }
    }

    pub fn espessura_change(&mut self, ui: &mut Ui, _sender: crate::generated::ComboBox, value: String) {
        // "5 px" → 5
        if let Ok(n) = value.split(' ').next().unwrap_or_default().parse::<i64>() {
            self.pcDesenho.set_brush_size(ui, n);
        }
    }

    pub fn preencher_change(&mut self, ui: &mut Ui, _sender: crate::generated::CheckBox, value: bool) {
        self.pcDesenho.set_filled(ui, value);
    }

    pub fn cor1_click(&mut self, ui: &mut Ui, _sender: crate::generated::Circle) {
        self.marcar_cor(ui, 1);
    }

    pub fn cor2_click(&mut self, ui: &mut Ui, _sender: crate::generated::Circle) {
        self.marcar_cor(ui, 2);
    }

    pub fn paleta_click(&mut self, ui: &mut Ui, sender: crate::generated::Circle) {
        let cor = sender.fill(ui);
        if ESTADO.with(|e| e.borrow().cor_ativa) == 1 {
            self.pcDesenho.set_color(ui, cor);
            self.btnCor1.set_fill(ui, cor);
        } else {
            self.pcDesenho.set_back_color(ui, cor);
            self.btnCor2.set_fill(ui, cor);
        }
    }

    pub fn desenho_change(&mut self, ui: &mut Ui, _sender: crate::generated::PaintCanvas) {
        self.marcar_modificado(ui);
    }

    pub fn desenho_pick(&mut self, ui: &mut Ui, _sender: crate::generated::PaintCanvas, value: String) {
        // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
        let (botao, cor) = match value.strip_suffix('2') {
            Some(cor) => (self.btnCor2, cor),
            None => (self.btnCor1, value.as_str()),
        };
        if let Some(cor) = Color::from_hex(cor) {
            botao.set_fill(ui, cor);
        }
    }

    pub fn desenho_cursor(&mut self, ui: &mut Ui, _sender: crate::generated::PaintCanvas, value: String) {
        self.lblPosicao.set_text(ui, if value.is_empty() { String::new() } else { format!("{value}px") });
    }

    pub fn abrir_select(&mut self, ui: &mut Ui, _sender: crate::generated::OpenDialog, value: Vec<String>) {
        // Um arquivo só: o primeiro da lista.
        let caminho = value.into_iter().next().unwrap_or_default();
        match self.pcDesenho.open(ui, &caminho) {
            Ok(_) => {
                ESTADO.with(|e| {
                    let mut e = e.borrow_mut();
                    e.arquivo = caminho;
                    e.modificado = false;
                });
                self.titulo(ui);
                self.atualizar_edicao(ui);
            }
            Err(erro) => {
                let _ = message_dlg(ui, &format!("Não foi possível abrir a imagem.\n{erro}"), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
            }
        }
    }

    pub fn salvar_select(&mut self, ui: &mut Ui, _sender: crate::generated::SaveDialog, value: Vec<String>) {
        self.salvar_em(ui, &value.into_iter().next().unwrap_or_default());
    }

    pub fn recortar_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.cut(ui);
    }

    pub fn copiar_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.copy(ui);
    }

    pub fn colar_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
        if self.pcDesenho.paste(ui).unwrap_or(false) {
            self.marcar_ferramenta(ui, self.btnSelecionar);
        }
    }

    pub fn excluir_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.delete(ui);
    }

    pub fn selecionar_tudo_click(&mut self, ui: &mut Ui, _sender: crate::generated::MenuItem) {
        let _ = self.pcDesenho.select_all(ui);
        self.marcar_ferramenta(ui, self.btnSelecionar);
    }
}

