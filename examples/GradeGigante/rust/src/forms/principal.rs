//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! Milhões de registros com 100 colunas (50 de texto, 49 de números e o código) no
//! `ClientDataSet`, mostrados pelo `DataGrid`. Os dados são inventados em paralelo (uma
//! thread por núcleo), cada parte num `RowBatch` (já nas colunas do dataset, sem copiar
//! os textos repetidos); as partes se juntam e entram no dataset **de uma vez**
//! (`load_batch`), com uma só ordenação. A tela continua respondendo e mostra o andamento.
//!
//! Cada carga tem um número: o que chega de uma carga que já acabou (o andamento atrasado,
//! o fim de uma carga parada) não mexe na tela da carga nova. O Parar vale até o último
//! momento: um lote já montado que ainda não entrou no dataset é jogado fora.

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use aylis::prelude::*;
use crate::generated::FrmPrincipal;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// De quantos em quantos registros cada thread confere o Parar e conta o andamento.
const STEP: usize = 4096;
const TEXT_COLUMNS: usize = 50;
const NUMBER_COLUMNS: usize = 49;
/// Os prefixos dos textos (a mesma ordem dos títulos do form).
const PREFIXES: [&str; 10] = ["Cliente", "Cidade", "Produto", "Categoria", "Vendedor", "Região", "Situação", "Marca", "Fornecedor", "Setor"];

/// A carga em andamento: o número dela e o aviso para parar.
struct Carga {
    numero: u64,
    stop: Arc<AtomicBool>,
}

thread_local! {
    static ATUAL: RefCell<Option<Carga>> = const { RefCell::new(None) };
}

/// O número da próxima carga.
static PROXIMA: AtomicU64 = AtomicU64::new(1);

/// A carga `numero` ainda é a que está em andamento (e não foi parada)?
fn em_andamento(numero: u64) -> bool {
    ATUAL.with(|a| a.borrow().as_ref().is_some_and(|c| c.numero == numero && !c.stop.load(Ordering::Relaxed)))
}

/// Um número de 0 a 999 que parece aleatório, igual em toda carga.
fn mix(column: usize, row: usize) -> usize {
    (row.wrapping_mul(2654435761) ^ column.wrapping_mul(40503)) % 1000
}

/// Os registros de `rows` num lote (`None` se pediram para parar no meio).
fn parte(rows: std::ops::Range<usize>, palavras: &[Vec<String>], feitos: &AtomicUsize, stop: &AtomicBool) -> Option<RowBatch> {
    let mut lote = RowBatch::new(1 + TEXT_COLUMNS + NUMBER_COLUMNS);
    for row in rows {
        lote.int(row as i64 + 1);
        for (c, lista) in palavras.iter().enumerate() {
            lote.text(&lista[mix(c, row)]);
        }
        for c in 0..NUMBER_COLUMNS {
            let x = mix(c + 100, row) as f64;
            if c % 2 == 0 {
                lote.int((x as i64) * 37 + c as i64);
            } else {
                lote.float(x * 12.75 + c as f64 / 100.0);
            }
        }
        lote.end_row();
        if lote.len() % STEP == 0 {
            feitos.fetch_add(STEP, Ordering::Relaxed);
            if stop.load(Ordering::Relaxed) {
                return None;
            }
        }
    }
    Some(lote)
}

/// Memória privada do processo, em MB (só no Windows).
#[cfg(windows)]
fn memoria_mb() -> Option<f64> {
    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        cb: u32,
        page_faults: u32,
        sizes: [usize; 8],
        private: usize,
    }
    unsafe extern "system" {
        fn GetCurrentProcess() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut Counters, cb: u32) -> i32;
    }
    let mut c = Counters { cb: std::mem::size_of::<Counters>() as u32, ..Default::default() };
    // SAFETY: a struct tem o formato da PROCESS_MEMORY_COUNTERS_EX, com o tamanho informado.
    (unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut c, c.cb) } != 0).then(|| c.private as f64 / (1024.0 * 1024.0))
}

#[cfg(not(windows))]
fn memoria_mb() -> Option<f64> {
    None
}

fn milhares(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(ch);
    }
    out
}

impl FrmPrincipal {
    fn status(&mut self, ui: &mut Ui, texto: &str, feitos: usize, total: usize, inicio: Instant) {
        let memoria = memoria_mb().map(|m| format!(" · memória {} MB", milhares(m as usize))).unwrap_or_default();
        let texto = format!("{texto} {} de {} registros em {:.1} s{memoria}", milhares(feitos), milhares(total), inicio.elapsed().as_secs_f64());
        self.lblStatus.set_text(ui, texto);
        self.pbCarga.set_value(ui, feitos as f64);
    }

    /// A carga `numero` acabou (pronta, com erro ou parada). Se outra já começou, a tela é
    /// dela: nada muda.
    fn terminou(&mut self, ui: &mut Ui, numero: u64) {
        let era_a_atual = ATUAL.with(|a| {
            let mut a = a.borrow_mut();
            let atual = a.as_ref().is_some_and(|c| c.numero == numero);
            if atual {
                a.take();
            }
            atual
        });
        if era_a_atual {
            self.btnCarregar.set_enabled(ui, true);
            self.btnParar.set_enabled(ui, false);
        }
    }

    /// A carga `numero` foi parada: o aviso e os botões (se ela ainda é a da tela).
    fn interrompida(&mut self, ui: &mut Ui, numero: u64) {
        if ATUAL.with(|a| a.borrow().as_ref().is_some_and(|c| c.numero == numero)) {
            self.lblStatus.set_text(ui, "Carga interrompida.");
            self.terminou(ui, numero);
        }
    }

    pub fn carregar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let cds = self.cdsDados.id;
        let total = self.nbRegistros.value(ui).max(1.0) as usize;
        // Começa vazio (a ordem e o filtro escolhidos continuam valendo).
        if let Err(e) = data_set(ui, cds).load_rows(Vec::new()) {
            self.lblStatus.set_text(ui, e.message);
            return;
        }
        let stop = Arc::new(AtomicBool::new(false));
        let numero = PROXIMA.fetch_add(1, Ordering::Relaxed);
        ATUAL.with(|a| *a.borrow_mut() = Some(Carga { numero, stop: stop.clone() }));
        self.btnCarregar.set_enabled(ui, false);
        self.btnParar.set_enabled(ui, true);
        self.pbCarga.set_max(ui, total as f64);
        let inicio = Instant::now();
        self.status(ui, "Montando", 0, total, inicio);
        let poster = ui.poster();
        // O que as threads mandam de volta roda depois deste handler: chega à janela pelo
        // form dela (`with_form`), não por `self`.
        let form = self.form.id;
        let feitos = Arc::new(AtomicUsize::new(0));
        // O andamento na tela enquanto as threads montam.
        let montando = Arc::new(AtomicBool::new(true));
        {
            let (poster, feitos, montando) = (poster.clone(), feitos.clone(), montando.clone());
            std::thread::spawn(move || {
                while montando.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(100));
                    let n = feitos.load(Ordering::Relaxed).min(total);
                    let montando = montando.clone();
                    poster.post(move |ui| {
                        // Atrasado (a montagem acabou, a carga foi parada ou já é outra): o
                        // "Montando" velho não cobre o que a tela mostra agora.
                        if montando.load(Ordering::Relaxed) && em_andamento(numero) {
                            ui.with_form(form, move |f: &mut FrmPrincipal, ui| f.status(ui, "Montando", n, total, inicio));
                        }
                    });
                }
            });
        }
        std::thread::spawn(move || {
            // Mil textos diferentes por coluna (o dicionário do ClientDataSet guarda cada um uma vez).
            let palavras: Vec<Vec<String>> =
                (0..TEXT_COLUMNS).map(|c| (0..1000).map(|w| format!("{} {:03}", PREFIXES[c % PREFIXES.len()], w + (c / PREFIXES.len()) * 1000)).collect()).collect();
            // Uma parte por núcleo, montadas ao mesmo tempo.
            let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).min(total.div_ceil(STEP)).max(1);
            // Partes com múltiplos de RowBatch::PART registros se juntam sem copiar.
            let tamanho = total.div_ceil(threads).div_ceil(RowBatch::PART) * RowBatch::PART;
            let partes: Vec<Option<RowBatch>> = std::thread::scope(|s| {
                let feitas: Vec<_> = (0..threads)
                    .map(|t| {
                        let rows = (t * tamanho).min(total)..((t + 1) * tamanho).min(total);
                        let (palavras, feitos, stop) = (&palavras, &feitos, &stop);
                        s.spawn(move || parte(rows, palavras, feitos, stop))
                    })
                    .collect();
                feitas.into_iter().map(|h| h.join().ok().flatten()).collect()
            });
            montando.store(false, Ordering::Relaxed);
            if stop.load(Ordering::Relaxed) || partes.iter().any(Option::is_none) {
                // As partes já montadas saem aqui (antes de avisar a tela).
                drop(partes);
                poster.post(move |ui| {
                    ui.with_form(form, move |f: &mut FrmPrincipal, ui| f.interrompida(ui, numero));
                });
                return;
            }
            let mut todos = RowBatch::default();
            for (i, parte) in partes.into_iter().flatten().enumerate() {
                if i == 0 {
                    todos = parte;
                } else {
                    todos.append(parte);
                }
            }
            let montado = inicio.elapsed().as_secs_f64();
            poster.post(move |ui| {
                ui.with_form(form, move |f: &mut FrmPrincipal, ui| {
                    // Parada enquanto as partes se juntavam: o lote não entra (e a memória dele
                    // volta agora, não fica no dataset).
                    if !em_andamento(numero) {
                        drop(todos);
                        f.interrompida(ui, numero);
                        return;
                    }
                    f.status(ui, "Carregando", total, total, inicio);
                    // Tudo de uma vez: uma troca e uma só ordenação.
                    if let Err(e) = data_set(ui, f.cdsDados.id).load_batch(todos) {
                        f.lblStatus.set_text(ui, e.message);
                        f.terminou(ui, numero);
                        return;
                    }
                    f.status(ui, &format!("Pronto (montado em {montado:.1} s):"), total, total, inicio);
                    f.terminou(ui, numero);
                });
            });
        });
    }

    pub fn parar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ATUAL.with(|a| {
            if let Some(carga) = a.borrow().as_ref() {
                carga.stop.store(true, Ordering::Relaxed);
            }
        });
        self.btnParar.set_enabled(ui, false);
        self.lblStatus.set_text(ui, "Parando…");
    }
}

