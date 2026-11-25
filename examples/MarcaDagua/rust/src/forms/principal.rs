//! Os eventos do form frmPrincipal: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).
//!
//! Uma grade de vídeos, cada um com a marca d'água própria. A célula do form (`layCelula`:
//! o `vpVideo`, a camada `layMarca` e o texto `lblMarca`) é o modelo, e as outras são cópias
//! dela (`ui.clone_component`). O texto é branco, girado, repetido em linhas, e a camada está
//! em `blend_mode: exclusion`: com o branco, ela dá o negativo (o mesmo pixel da
//! `difference`), então cada letra sai na cor invertida do vídeo que está por baixo dela,
//! quadro a quadro, sem o app tocar nos pixels. No Direct2D, a `exclusion` é a mistura que a
//! placa faz sem copiar o fundo; a `difference` copia o fundo a cada camada.
//!
//! Sem o vídeo ("Vídeo" desmarcado), cada célula mostra uma imagem fixa e nenhum decodificador
//! roda: sobra só o custo da marca na placa. "Mover" faz a marca deslizar (a `FloatAnimation`
//! do `lblMarca`, a cada quadro), o que obriga a refazer a camada `difference` de todas as
//! células em todo quadro.
//!
//! Para medir sem clicar: `marca_dagua --grid 36` abre com 36 vídeos e `--mark TEXTO` troca a
//! marca (vazio: sem marca); `--still` tira o vídeo, `--move` move a marca, `--blend normal`
//! desliga a inversão, `--blend difference` inverte pela `difference` (para comparar o custo) e
//! `--paint-rate N` junta as células que mudaram em até N pinturas por segundo;
//! com `AYLIS_BENCH`, o tempo de cada quadro.

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::{FloatAnimation, FrmPrincipal, Image, Label, Layout, VideoPlayer};

/// As escolhas do `segGrade`: quantos vídeos ao todo (até 1000).
const COUNTS: [usize; 11] = [1, 4, 9, 16, 36, 64, 100, 256, 400, 625, 1000];
/// O espaço entre as repetições do texto.
const GAP: &str = "      ";
/// Quanto a marca desliza com "Mover", no máximo (o `stop_value` da `aniMarca`, que o `fit`
/// ajusta ao tamanho da célula).
const SLIDE: f32 = 60.0;
/// O espaço entre as células da grade (o `gap` do form).
const CELL_GAP: f32 = 2.0;

/// O texto da marca: `text` repetido `per_line` vezes em `lines` linhas, cada linha deslocada
/// meia repetição da de cima (o padrão em xadrez das marcas d'água).
pub fn watermark_text(text: &str, lines: usize, per_line: usize) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    let line = vec![text; per_line].join(GAP);
    let half = " ".repeat(text.chars().count() / 2 + GAP.len() / 2);
    (0..lines)
        .map(|i| if i % 2 == 0 { line.clone() } else { format!("{half}{line}") })
        .collect::<Vec<_>>()
        .join("\n")
}

/// A marca de uma célula: o texto da barra com o número dela ("CÂMERA 07").
pub fn cell_mark(base: &str, index: usize) -> String {
    let base = base.trim();
    if base.is_empty() { String::new() } else { format!("{base} {:02}", index + 1) }
}

/// As colunas e as linhas da grade para `count` vídeos: quase quadrada, e a última linha
/// pode ficar incompleta (1000 = 32 colunas × 32 linhas, a última com 8).
pub fn grid_shape(count: usize) -> (usize, usize) {
    let count = count.max(1);
    let columns = (count as f64).sqrt().ceil() as usize;
    (columns, count.div_ceil(columns))
}

/// A quantidade da escolha `index` do `segGrade`.
fn count_at(index: i64) -> usize {
    COUNTS[(index.max(0) as usize).min(COUNTS.len() - 1)]
}

/// Uma célula da grade: a imagem fixa, o vídeo, a camada da marca, o texto e a animação dele
/// (na ordem do modelo).
struct Cell {
    image: Image,
    video: VideoPlayer,
    layer: Layout,
    label: Label,
    animation: FloatAnimation,
}

/// O vídeo de todas as células até o usuário abrir outro.
const SAMPLE_VIDEO: &str = "assets/video/sample.mp4";

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {
    /// A geometria e o texto da última arrumação: o `on_resize` da grade volta a cada mudança
    /// da altura das linhas, e refazer o texto sem mudança repintaria tudo à toa.
    fitted: Option<(usize, i32, i32, String, usize)>,
    /// O vídeo aberto (vazio: o de exemplo).
    video_file: String,
    /// A inversão pela `difference` (`--blend difference`), no lugar da `exclusion`.
    difference: bool,
}

// Os handlers têm o nome do componente (`btnSalvar_click`).
#[allow(non_snake_case)]
impl FrmPrincipal {
    /// As células na ordem da grade (a primeira é a do form).
    fn cells(&self, ui: &Ui) -> Vec<Cell> {
        ui.children(self.grdVideos.id)
            .iter()
            .filter_map(|&cell| {
                let parts = ui.children(cell);
                let (&image, &video, &layer) = (parts.first()?, parts.get(1)?, parts.get(2)?);
                let &label = ui.children(layer).first()?;
                let &animation = ui.children(label).first()?;
                Some(Cell {
                    image: Image { id: image },
                    video: VideoPlayer { id: video },
                    layer: Layout { id: layer },
                    label: Label { id: label },
                    animation: FloatAnimation { id: animation },
                })
            })
            .collect()
    }

    fn video_file(&self) -> String {
        if self.data.video_file.is_empty() { SAMPLE_VIDEO.to_string() } else { self.data.video_file.clone() }
    }

    /// Põe a célula no modo da barra: o vídeo tocando ou a imagem fixa (sem decodificador), e
    /// a marca parada ou deslizando.
    fn apply_mode(&self, ui: &mut Ui, cell: &Cell) {
        let video = self.chkVideo.checked(ui);
        cell.image.set_visible(ui, !video);
        cell.video.set_visible(ui, video);
        if video {
            if cell.video.file(ui).is_empty() {
                cell.video.set_file(ui, self.video_file());
            }
            if self.btnTocar.text(ui) == "Pausar" {
                cell.video.play(ui);
            }
        } else if !cell.video.file(ui).is_empty() {
            cell.video.stop(ui);
            cell.video.set_file(ui, "");
        }
        if self.chkMover.checked(ui) {
            let _ = cell.animation.start(ui);
        } else {
            let _ = cell.animation.stop(ui);
            cell.label.set_translate_x(ui, 0.0);
        }
    }

    fn count(&self, ui: &Ui) -> usize {
        count_at(self.segGrade.item_index(ui))
    }

    /// Deixa `count` células na grade: copia o modelo ou libera as que sobram.
    fn build(&mut self, ui: &mut Ui, count: usize) {
        let grid = self.grdVideos.id;
        let mut cells: Vec<ComponentId> = ui.children(grid).to_vec();
        while cells.len() > count {
            if let Some(id) = cells.pop() {
                ui.free(id);
            }
        }
        let first_new = cells.len();
        while cells.len() < count {
            match ui.clone_component(self.layCelula.id, Some(grid)) {
                Ok(id) => cells.push(id),
                Err(_) => break,
            }
        }
        // A cópia leva as propriedades, mas o vídeo de cada uma abre e toca por conta própria, e a
        // animação começa do zero.
        for cell in self.cells(ui).into_iter().skip(first_new) {
            cell.video.set_file(ui, "");
            self.apply_mode(ui, &cell);
        }
        self.grdVideos.set_columns(ui, grid_shape(count).0 as i64);
        self.fit(ui);
    }

    /// As linhas da grade dividem a altura (a grade só divide a largura sozinha), e o texto de
    /// cada célula acompanha o tamanho dela: cobre a diagonal, girado, sem desenhar à toa
    /// fora da célula.
    fn fit(&mut self, ui: &mut Ui) {
        let (columns, rows) = grid_shape(self.count(ui));
        let (columns, rows) = (columns as f32, rows as f32);
        let bounds = ui.control(self.grdVideos.id).map(|c| c.bounds()).unwrap_or_default();
        let row = ((bounds.height - CELL_GAP * (rows - 1.0)) / rows).max(8.0);
        if (self.grdVideos.row_height(ui) as f32 - row).abs() > 0.5 {
            self.grdVideos.set_row_height(ui, row as f64);
        }
        let col = ((bounds.width - CELL_GAP * (columns - 1.0)) / columns).max(8.0);
        // O deslize acompanha a célula (uma folga fixa multiplicava o texto das pequenas).
        let slide = (col.min(row) * 0.3).min(SLIDE).round();
        // O rótulo girado cobre a célula com folga (os cantos não ficam sem texto), mais o
        // deslize da `aniMarca` dos dois lados.
        let diagonal = ((col * col + row * row).sqrt() * 1.4 + 2.0 * slide).ceil();
        let font = (col.min(row) / 12.0).clamp(6.0, 24.0);
        let base = self.edtTexto.text(ui);
        let key = (columns as usize * rows as usize, col.round() as i32, row.round() as i32, base.clone(), ui.children(self.grdVideos.id).len());
        if self.data.fitted.as_ref() == Some(&key) {
            return;
        }
        self.data.fitted = Some(key);
        let longest = cell_mark(&base, 999).chars().count().max(1) + GAP.len();
        // Linha de 3 fontes de altura; letra em negrito ~0,7 da fonte.
        let lines = (diagonal / (font * 3.0)).ceil() as usize + 1;
        let per_line = (diagonal / (font * 0.7 * longest as f32)).ceil() as usize + 1;
        for (i, cell) in self.cells(ui).into_iter().enumerate() {
            cell.label.set_width(ui, diagonal as f64);
            cell.label.set_height(ui, diagonal as f64);
            cell.label.set_font_size(ui, font as f64);
            cell.label.set_text(ui, watermark_text(&cell_mark(&base, i), lines, per_line));
            cell.animation.set_stop_value(ui, slide as f64);
        }
    }

    pub fn frmPrincipal_show(&mut self, ui: &mut Ui, sender: crate::generated::Form) {
        // `--grid N`: começa com N vídeos (o mais perto entre as escolhas), para medir.
        let args: Vec<String> = std::env::args().collect();
        let arg = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned());
        // `--mark TEXTO`: o texto da marca (vazio, sem marca: para medir só os vídeos).
        if let Some(mark) = arg("--mark") {
            self.edtTexto.set_text(ui, mark);
        }
        let flag = |name: &str| args.iter().any(|a| a == name);
        // `--paint-rate N`: as células que mudaram se juntam numa pintura, no máximo N vezes
        // por segundo (o `max_paint_rate` do form; a entrada continua na hora).
        if let Some(rate) = arg("--paint-rate").and_then(|n| n.parse::<f64>().ok()) {
            sender.set_max_paint_rate(ui, rate);
        }
        if flag("--still") {
            self.chkVideo.set_checked(ui, false);
        }
        if flag("--move") {
            self.chkMover.set_checked(ui, true);
        }
        match arg("--blend").as_deref() {
            Some("normal") => self.chkInverter.set_checked(ui, false),
            Some("difference") => {
                self.data.difference = true;
                self.layMarca.set_blend_mode(ui, BlendMode::Difference);
            }
            _ => {}
        }
        let wanted = arg("--grid").and_then(|n| n.parse::<usize>().ok());
        if let Some(wanted) = wanted {
            let index = COUNTS.iter().position(|&c| c >= wanted).unwrap_or(COUNTS.len() - 1);
            // O `on_change` monta a grade.
            self.segGrade.set_item_index(ui, index as i64);
        }
        self.fit(ui);
    }

    pub fn segGrade_change(&mut self, ui: &mut Ui, _sender: crate::generated::SegmentedControl, value: i64) {
        self.build(ui, count_at(value));
    }

    pub fn grdVideos_resize(&mut self, ui: &mut Ui, _sender: crate::generated::GridLayout) {
        self.fit(ui);
    }

    pub fn btnAbrir_click(&mut self, ui: &mut Ui, _sender: crate::generated::Button) {
        self.dlgAbrir.execute(ui);
    }

    /// Tocar/pausar todos: o texto do botão diz o que o clique faz.
    pub fn btnTocar_click(&mut self, ui: &mut Ui, _sender: crate::generated::Button) {
        // O texto do botão é o estado (sem vídeo, nenhum player está tocando).
        let playing = self.btnTocar.text(ui) == "Pausar";
        let cells: Vec<Cell> = self.cells(ui).into_iter().filter(|c| !c.video.file(ui).is_empty()).collect();
        for cell in cells {
            if playing { cell.video.pause(ui) } else { cell.video.play(ui) }
        }
        self.btnTocar.set_text(ui, if playing { "Tocar" } else { "Pausar" });
    }

    pub fn edtTexto_change(&mut self, ui: &mut Ui, _sender: crate::generated::Edit, _value: String) {
        self.fit(ui);
    }

    /// Desligado, a marca fica branca por cima (`normal`), para comparar.
    pub fn chkInverter_change(&mut self, ui: &mut Ui, _sender: crate::generated::CheckBox, value: bool) {
        let mode = match (value, self.data.difference) {
            (false, _) => BlendMode::Normal,
            (true, true) => BlendMode::Difference,
            (true, false) => BlendMode::Exclusion,
        };
        for cell in self.cells(ui) {
            cell.layer.set_blend_mode(ui, mode);
        }
    }

    pub fn vpVideo_error(&mut self, ui: &mut Ui, sender: crate::generated::VideoPlayer, value: String) {
        // Sem arquivo (o modo da imagem fixa) não é erro.
        if sender.file(ui).is_empty() {
            return;
        }
        let _ = message_dlg(ui, &format!("Não foi possível tocar o vídeo.\n{value}"), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
    }

    /// O vídeo escolhido vai para todas as células.
    pub fn dlgAbrir_select(&mut self, ui: &mut Ui, _sender: crate::generated::OpenDialog, value: Vec<String>) {
        let Some(file) = value.into_iter().find(|f| !f.is_empty()) else { return };
        self.data.video_file = file;
        self.btnTocar.set_text(ui, "Pausar");
        self.chkVideo.set_checked(ui, true);
        for cell in self.cells(ui) {
            cell.video.set_file(ui, "");
            self.apply_mode(ui, &cell);
        }
    }

    pub fn chkMover_change(&mut self, ui: &mut Ui, _sender: crate::generated::CheckBox, _value: bool) {
        for cell in self.cells(ui) {
            self.apply_mode(ui, &cell);
        }
    }

    pub fn chkVideo_change(&mut self, ui: &mut Ui, _sender: crate::generated::CheckBox, _value: bool) {
        for cell in self.cells(ui) {
            self.apply_mode(ui, &cell);
        }
    }
}
