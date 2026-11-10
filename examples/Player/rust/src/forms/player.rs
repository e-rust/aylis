//! Eventos do form frmPlayer: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
//! anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
//! toca é o componente `VideoPlayer` (vpVideo), áudio e vídeo.

use std::cell::RefCell;

use aylis::prelude::*;
use crate::generated::FrmPlayer;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// O que o form guarda.
struct Estado {
    fila: Vec<String>,
    /// O índice da mídia atual na fila (-1: nenhuma).
    atual: i64,
    tocando: bool,
    embaralhar: bool,
    /// 0: não repete; 1: repete a fila; 2: repete a mídia.
    repetir: i64,
    mudo: bool,
    /// A semente do embaralhar (a mesma conta em todas as linguagens).
    semente: i64,
}

thread_local! {
    static ESTADO: RefCell<Estado> = const {
        RefCell::new(Estado { fila: Vec::new(), atual: -1, tocando: false, embaralhar: false, repetir: 0, mudo: false, semente: 12345 })
    };
}

/// As extensões de áudio (o resto é vídeo).
const AUDIO: [&str; 7] = ["mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"];

/// O nome do arquivo sem a pasta.
fn nome_do_arquivo(caminho: &str) -> &str {
    caminho.rsplit(['\\', '/']).next().unwrap_or(caminho)
}

/// A pasta do arquivo (vazio se não tiver).
fn pasta(caminho: &str) -> &str {
    caminho.rfind(['\\', '/']).map_or("", |i| &caminho[..i])
}

/// O nome sem a extensão.
fn sem_extensao(nome: &str) -> &str {
    match nome.rfind('.') {
        Some(i) if i > 0 => &nome[..i],
        _ => nome,
    }
}

fn tipo(caminho: &str) -> &'static str {
    let nome = nome_do_arquivo(caminho);
    let ext = nome.rfind('.').map_or(String::new(), |i| nome[i + 1..].to_lowercase());
    if AUDIO.contains(&ext.as_str()) { "Música" } else { "Vídeo" }
}

/// "3:07"
fn tempo(segundos: f64) -> String {
    let s = segundos.max(0.0).floor() as i64;
    format!("{}:{:02}", s / 60, s % 60)
}

impl FrmPlayer {
    /// A fila na lista, com a mídia atual selecionada.
    fn atualizar_fila(&mut self, ui: &mut Ui) {
        let (itens, atual) = ESTADO.with(|e| {
            let e = e.borrow();
            let itens: Vec<String> = e.fila.iter().map(|c| format!("{}|{}", nome_do_arquivo(c), pasta(c))).collect();
            (itens.join(";"), e.atual)
        });
        self.lvFila.set_items(ui, itens);
        self.lvFila.set_item_index(ui, atual);
    }

    /// O título, o detalhe e o título da janela.
    fn mostrar_atual(&mut self, ui: &mut Ui) {
        let (atual, total, caminho) = ESTADO.with(|e| {
            let e = e.borrow();
            let caminho = usize::try_from(e.atual).ok().and_then(|i| e.fila.get(i).cloned());
            (e.atual, e.fila.len(), caminho)
        });
        match caminho {
            None => {
                self.lblTitulo.set_text(ui, "Nenhuma mídia");
                self.lblDetalhe.set_text(ui, "");
                self.set_caption(ui, "Reprodutor Multimídia");
            }
            Some(c) => {
                let titulo = sem_extensao(nome_do_arquivo(&c)).to_string();
                self.lblTitulo.set_text(ui, titulo.clone());
                self.lblDetalhe.set_text(ui, format!("{} · {} de {}", tipo(&c), atual + 1, total));
                self.set_caption(ui, format!("{titulo} - Reprodutor Multimídia"));
            }
        }
    }

    /// O botão do meio mostra o que ele vai fazer: tocar ou pausar.
    fn botao_reproduzir(&mut self, ui: &mut Ui) {
        let tocando = ESTADO.with(|e| e.borrow().tocando);
        self.btnReproduzir.set_icon(ui, if tocando { "pause" } else { "play" });
        self.btnReproduzir.set_hint(ui, if tocando { "Pausar (Ctrl+P)" } else { "Reproduzir (Ctrl+P)" });
    }

    /// Toca a mídia `i` da fila. Tocar é a última coisa: um erro chega no `video_erro`.
    fn tocar(&mut self, ui: &mut Ui, i: i64) {
        let caminho = ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.atual = i;
            e.tocando = true;
            e.fila[i as usize].clone()
        });
        self.vpVideo.set_file(ui, caminho);
        self.mostrar_atual(ui);
        self.atualizar_fila(ui);
        self.botao_reproduzir(ui);
        self.sbPosicao.set_position(ui, 0.0);
        self.vpVideo.play(ui);
    }

    pub fn abrir_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.dlgAbrir.execute(ui);
    }

    pub fn abrir_select(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        // Os arquivos escolhidos vêm separados por ";".
        let novos: Vec<String> = args.text().unwrap_or_default().split(';').filter(|s| !s.is_empty()).map(str::to_string).collect();
        if novos.is_empty() {
            return;
        }
        let (primeiro, atual) = ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            let primeiro = e.fila.len() as i64;
            e.fila.extend(novos);
            (primeiro, e.atual)
        });
        self.atualizar_fila(ui);
        if atual < 0 {
            self.tocar(ui, primeiro);
        } else {
            self.mostrar_atual(ui);
        }
    }

    pub fn limpar_fila_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.vpVideo.stop(ui);
        ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.fila.clear();
            e.atual = -1;
            e.tocando = false;
        });
        self.vpVideo.set_file(ui, "");
        self.atualizar_fila(ui);
        self.mostrar_atual(ui);
        self.botao_reproduzir(ui);
    }

    pub fn sair_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.close(ui);
    }

    pub fn reproduzir_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let (atual, vazia, tocando) = ESTADO.with(|e| {
            let e = e.borrow();
            (e.atual, e.fila.is_empty(), e.tocando)
        });
        if atual < 0 {
            if vazia {
                self.abrir_click(ui, sender, args);
            } else {
                self.tocar(ui, 0);
            }
        } else if tocando {
            self.vpVideo.pause(ui);
            ESTADO.with(|e| e.borrow_mut().tocando = false);
            self.botao_reproduzir(ui);
        } else {
            ESTADO.with(|e| e.borrow_mut().tocando = true);
            self.botao_reproduzir(ui);
            self.vpVideo.play(ui);
        }
    }

    pub fn parar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if ESTADO.with(|e| e.borrow().atual) < 0 {
            return;
        }
        self.vpVideo.stop(ui);
        ESTADO.with(|e| e.borrow_mut().tocando = false);
        self.botao_reproduzir(ui);
        self.sbPosicao.set_position(ui, 0.0);
    }

    pub fn anterior_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let (atual, n, repetir) = ESTADO.with(|e| {
            let e = e.borrow();
            (e.atual, e.fila.len() as i64, e.repetir)
        });
        if atual > 0 {
            self.tocar(ui, atual - 1);
        } else if atual == 0 && repetir == 1 {
            self.tocar(ui, n - 1);
        } else if atual == 0 {
            self.vpVideo.seek(ui, 0.0);
        }
    }

    pub fn proxima_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let i = proximo_indice();
        if i >= 0 {
            self.tocar(ui, i);
        }
    }

    pub fn embaralhar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ligado = ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.embaralhar = !e.embaralhar;
            e.embaralhar
        });
        self.btnEmbaralhar.set_checked(ui, ligado);
        self.btnEmbaralhar.set_hint(ui, if ligado { "Embaralhar: ativado" } else { "Embaralhar: desativado" });
        self.mnuEmbaralhar.set_checked(ui, ligado);
    }

    pub fn repetir_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let modo = ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.repetir = (e.repetir + 1) % 3;
            e.repetir
        });
        self.btnRepetir.set_checked(ui, modo > 0);
        self.btnRepetir.set_icon(ui, if modo == 2 { "repeat_one" } else { "repeat" });
        self.btnRepetir.set_hint(ui, ["Repetir: desativado", "Repetir: todos", "Repetir: um"][modo as usize]);
        self.mnuRepetir.set_checked(ui, modo > 0);
    }

    /// Mostra a página "Reproduzindo agora" ou a "Fila de reprodução".
    fn pagina(&mut self, ui: &mut Ui, agora: bool) {
        self.pnlAgora.set_visible(ui, agora);
        self.pnlFila.set_visible(ui, !agora);
        self.btnAgora.set_checked(ui, agora);
        self.btnFila.set_checked(ui, !agora);
    }

    pub fn agora_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.pagina(ui, true);
    }

    pub fn fila_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.pagina(ui, false);
    }

    pub fn posicao_seek(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Value(v) = args {
            let segundos = v.to_f64().unwrap_or(0.0);
            self.vpVideo.seek(ui, segundos);
        }
    }

    pub fn mudo_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let mudo = ESTADO.with(|e| {
            let mut e = e.borrow_mut();
            e.mudo = !e.mudo;
            e.mudo
        });
        self.vpVideo.set_muted(ui, mudo);
        self.btnMudo.set_icon(ui, if mudo { "mute" } else { "volume" });
        self.btnMudo.set_hint(ui, if mudo { "Com som" } else { "Sem som" });
    }

    pub fn volume_change(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Value(v) = args {
            self.vpVideo.set_volume(ui, v.to_f64().unwrap_or(0.0) / 100.0);
        }
    }

    pub fn video_fim(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let (atual, repetir) = ESTADO.with(|e| (e.borrow().atual, e.borrow().repetir));
        if repetir == 2 && atual >= 0 {
            self.tocar(ui, atual);
            return;
        }
        let i = proximo_indice();
        if i >= 0 {
            self.tocar(ui, i);
        } else {
            ESTADO.with(|e| e.borrow_mut().tocando = false);
            self.botao_reproduzir(ui);
        }
    }

    pub fn video_erro(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ESTADO.with(|e| e.borrow_mut().tocando = false);
        self.botao_reproduzir(ui);
        self.lblDetalhe.set_text(ui, "Não foi possível reproduzir este arquivo");
    }

    pub fn fila_item_click(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let i = match args {
            EventArgs::Value(v) => v.to_i64().unwrap_or(-1),
            _ => -1,
        };
        if i >= 0 && i < ESTADO.with(|e| e.borrow().fila.len() as i64) {
            self.tocar(ui, i);
        }
    }

    pub fn posicao_timer(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if ESTADO.with(|e| e.borrow().atual) < 0 {
            return;
        }
        let posicao = self.vpVideo.position(ui);
        let duracao = self.vpVideo.duration(ui);
        self.sbPosicao.set_duration(ui, duracao);
        self.sbPosicao.set_position(ui, posicao);
        self.lblTempo.set_text(ui, tempo(posicao));
        self.lblDuracao.set_text(ui, tempo(duracao));
    }
}

/// A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a fila.
/// -1: acabou.
fn proximo_indice() -> i64 {
    ESTADO.with(|e| {
        let mut e = e.borrow_mut();
        let n = e.fila.len() as i64;
        if n == 0 {
            return -1;
        }
        if e.embaralhar {
            if n == 1 {
                return 0;
            }
            loop {
                e.semente = (e.semente * 1103515245 + 12345) % 2147483648;
                let i = e.semente % n;
                if i != e.atual {
                    return i;
                }
            }
        }
        if e.atual + 1 < n {
            e.atual + 1
        } else if e.repetir == 1 {
            0
        } else {
            -1
        }
    })
}

