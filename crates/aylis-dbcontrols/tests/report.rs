//! Relatório de bandas: título, cabeçalho e rodapé de página, grupos com totais, detalhe,
//! sumário, quebra de página e o PDF.

use aylis_controls::{Form, Label};
use aylis_core::pages::Op;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

/// A memória que esta thread alocou (viva e o pico), contada pelo alocador do teste: os
/// outros testes, rodando em paralelo, não entram na conta de quem mede.
mod meter {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    pub struct Meter;

    thread_local! {
        static LIVE: Cell<isize> = const { Cell::new(0) };
        static PEAK: Cell<isize> = const { Cell::new(0) };
    }

    fn add(delta: isize) {
        // `try_with`: no fim da thread, as variáveis dela já foram embora.
        let _ = LIVE.try_with(|live| {
            let now = live.get() + delta;
            live.set(now);
            let _ = PEAK.try_with(|peak| peak.set(peak.get().max(now)));
        });
    }

    // SAFETY: repassa cada pedido ao alocador do sistema, sem mexer nos ponteiros; só conta
    // os tamanhos.
    unsafe impl GlobalAlloc for Meter {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // SAFETY: o mesmo contrato do `GlobalAlloc::alloc`, repassado.
            let p = unsafe { System.alloc(layout) };
            if !p.is_null() {
                add(layout.size() as isize);
            }
            p
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            // SAFETY: idem.
            let p = unsafe { System.alloc_zeroed(layout) };
            if !p.is_null() {
                add(layout.size() as isize);
            }
            p
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            // SAFETY: idem.
            unsafe { System.dealloc(ptr, layout) };
            add(-(layout.size() as isize));
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            // SAFETY: idem.
            let p = unsafe { System.realloc(ptr, layout, new_size) };
            if !p.is_null() {
                add(new_size as isize - layout.size() as isize);
            }
            p
        }
    }

    /// Zera o pico (a partir do que está vivo agora) e devolve o vivo.
    pub fn start() -> isize {
        let live = LIVE.with(Cell::get);
        PEAK.with(|p| p.set(live));
        live
    }

    /// (vivo, pico) em bytes, desde o `start` que devolveu `base`.
    pub fn since(base: isize) -> (isize, isize) {
        (LIVE.with(Cell::get) - base, PEAK.with(Cell::get) - base)
    }
}

#[global_allocator]
static METER: meter::Meter = meter::Meter;

/// Vendas por região, `n` por região.
fn report(n: usize) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("relatório", 800.0, 600.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("regiao", FieldType::String), cds);
    ui.add(Field::new("produto", FieldType::String), cds);
    ui.add(Field::new("valor", FieldType::Integer), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let mut rows = Vec::new();
    for region in ["Norte", "Sul"] {
        for i in 0..n {
            rows.push(vec![region.into(), format!("Item {}", i + 1).into(), ((i as i64 + 1) * 10).into()]);
        }
    }
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    let rep = ui.add(Report::new(ds), form);
    let band = |ui: &mut Ui, kind: BandKind, height: f32, field: &str, text: &str| {
        let b = ui.add(ReportBand::new(kind, height).grouped_by(field), rep);
        let l = ui.add(Label::new(text), b);
        ui.control_mut(l).unwrap().set_rect(0.0, 0.0, 400.0, height);
        b
    };
    band(&mut ui, BandKind::Title, 40.0, "", "Vendas por região");
    band(&mut ui, BandKind::PageHeader, 20.0, "", "Página [PAGE] de [PAGES]");
    band(&mut ui, BandKind::GroupHeader, 24.0, "regiao", "Região [regiao]");
    band(&mut ui, BandKind::Detail, 18.0, "", "[produto]: [valor] ([valor * 2])");
    band(&mut ui, BandKind::GroupFooter, 22.0, "regiao", "Total [regiao]: [SUM(valor)] em [COUNT()]");
    band(&mut ui, BandKind::Summary, 30.0, "", "Geral: [SUM(valor)], maior [MAX(valor)]");
    band(&mut ui, BandKind::PageFooter, 20.0, "", "[PAGE]/[PAGES]");
    ui.layout();
    (ui, rep, form)
}

#[test]
fn bands_groups_and_totals() {
    let (mut ui, rep, _) = report(3);
    let pages = report::prepare(&mut ui, rep);
    assert_eq!(pages.pages.len(), 1);
    assert_eq!(
        pages.pages[0].texts(),
        [
            "Página 1 de 1",
            "Vendas por região",
            "Região Norte",
            "Item 1: 10 (20)",
            "Item 2: 20 (40)",
            "Item 3: 30 (60)",
            "Total Norte: 60 em 3",
            "Região Sul",
            "Item 1: 10 (20)",
            "Item 2: 20 (40)",
            "Item 3: 30 (60)",
            "Total Sul: 60 em 3",
            "Geral: 120, maior 30",
            "1/1",
        ]
    );
    // Os textos do designer voltam como estavam.
    let labels: Vec<String> = ui.components().filter_map(|c| ui.get::<Label>(c).map(|l| l.text.clone())).collect();
    assert!(labels.contains(&"[produto]: [valor] ([valor * 2])".to_string()));
}

#[test]
fn long_reports_break_pages_with_header_and_footer_on_each() {
    let (mut ui, rep, _) = report(30);
    let pages = report::prepare(&mut ui, rep);
    let n = pages.pages.len();
    assert!(n >= 2, "{n} páginas");
    for (i, page) in pages.pages.iter().enumerate() {
        let texts = page.texts();
        assert_eq!(texts[0], format!("Página {} de {n}", i + 1));
        assert_eq!(*texts.last().unwrap(), format!("{}/{n}", i + 1));
        // Nada passa da área útil (o rodapé fica no pé).
        for op in &page.ops {
            if let Op::Text { rect, .. } = op {
                assert!(rect.bottom() <= pages.height - 36.0 + 0.5, "{rect:?}");
            }
        }
    }
    // Um grupo em página nova.
    let header = ui.components().find(|c| ui.get::<ReportBand>(*c).is_some_and(|b| b.kind == BandKind::GroupHeader)).unwrap();
    ui.set_prop(header, "new_page", true).unwrap();
    let pages = report::prepare(&mut ui, rep);
    let starts: Vec<usize> = pages.pages.iter().enumerate().filter(|(_, p)| p.texts().contains(&"Região Sul")).map(|(i, _)| i).collect();
    let south = starts[0];
    assert_eq!(pages.pages[south].texts()[1], "Região Sul", "a região começa no alto da página");
}

#[test]
fn the_pdf_opens_with_every_page() {
    let (mut ui, rep, _) = report(30);
    let pages = report::prepare(&mut ui, rep).pages.len();
    let pdf = report::to_pdf(&mut ui, rep);
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let sizes = aylis_docview::pdf::page_sizes(&pdf).unwrap();
    assert_eq!(sizes.len(), pages);
    assert_eq!(sizes[0], (595.0, 842.0));
    // Deitada.
    ui.set_prop(rep, "landscape", true).unwrap();
    let sizes = aylis_docview::pdf::page_sizes(&report::to_pdf(&mut ui, rep)).unwrap();
    assert_eq!(sizes[0], (842.0, 595.0));
}

#[test]
fn the_report_only_shows_in_the_designer_and_the_preview_follows_the_data() {
    let (mut ui, rep, form) = report(2);
    assert!(ui.control(rep).unwrap().design_only());
    let preview = ui.add(ReportPreview::new(rep), form);
    ui.control_mut(preview).unwrap().set_rect(0.0, 0.0, 400.0, 500.0);
    ReportPreview::refresh(&mut ui, preview);
    assert_eq!(ui.get::<ReportPreview>(preview).unwrap().pages().unwrap().pages.len(), 1);
    // Um registro novo: a visualização monta de novo sozinha.
    let cds = ui.components().find(|c| ui.class_name(*c) == Some("ClientDataSet")).unwrap();
    {
        let mut d = data_set(&mut ui, cds);
        d.append().unwrap();
        d.set_value("regiao", "Oeste").unwrap();
        d.set_value("produto", "Novo").unwrap();
        d.set_value("valor", 5).unwrap();
        d.post().unwrap();
    }
    ui.process_posted();
    let texts: Vec<String> = ui.get::<ReportPreview>(preview).unwrap().pages().unwrap().pages[0].texts().iter().map(|s| s.to_string()).collect();
    assert!(texts.contains(&"Região Oeste".to_string()), "{texts:?}");
}

/// Pedidos (o mestre) com itens (o detalhe, pelo `master_source`), `items` itens cada.
fn master_detail(items: &[i64]) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("mestre-detalhe", 800.0, 600.0));
    let orders = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("numero", FieldType::Integer), orders);
    ui.add(Field::new("cliente", FieldType::String), orders);
    let orders_ds = ui.add(DataSource::new(orders), form);
    let lines = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("pedido", FieldType::Integer), lines);
    ui.add(Field::new("produto", FieldType::String), lines);
    ui.add(Field::new("valor", FieldType::Integer), lines);
    ui.set_prop(lines, "master_source", PropValue::Component(Some(orders_ds))).unwrap();
    ui.set_prop(lines, "master_fields", "pedido=numero").unwrap();
    let lines_ds = ui.add(DataSource::new(lines), form);
    let mut rows = Vec::new();
    for (o, &n) in items.iter().enumerate() {
        for i in 0..n {
            rows.push(vec![(o as i64 + 1).into(), format!("P{}.{}", o + 1, i + 1).into(), ((i + 1) * 10).into()]);
        }
    }
    data_set(&mut ui, lines).load_rows(rows).unwrap();
    let masters = (0..items.len()).map(|o| vec![(o as i64 + 1).into(), format!("Cliente {}", o + 1).into()]).collect();
    data_set(&mut ui, orders).load_rows(masters).unwrap();
    let rep = ui.add(Report::new(orders_ds), form);
    let band = |ui: &mut Ui, band: ReportBand, text: &str| {
        let height = band.base.bounds().height;
        let b = ui.add(band, rep);
        let l = ui.add(Label::new(text), b);
        ui.control_mut(l).unwrap().set_rect(0.0, 0.0, 400.0, height);
        b
    };
    band(&mut ui, ReportBand::new(BandKind::Detail, 20.0), "Pedido [numero] de [cliente]");
    band(&mut ui, ReportBand::sub(BandKind::SubHeader, 16.0, lines_ds), "Itens de [cliente]: [COUNT()]");
    band(&mut ui, ReportBand::sub(BandKind::SubDetail, 16.0, lines_ds), "[produto] = [valor] ([valor + numero])");
    band(&mut ui, ReportBand::sub(BandKind::SubFooter, 16.0, lines_ds), "Soma do pedido [numero]: [SUM(valor)]");
    band(&mut ui, ReportBand::new(BandKind::Summary, 20.0), "Pedidos: [COUNT()]");
    ui.layout();
    (ui, rep, orders)
}

#[test]
fn sub_detail_bands_print_the_detail_records_of_each_master_record() {
    let (mut ui, rep, orders) = master_detail(&[2, 0, 3]);
    data_set(&mut ui, orders).move_to(1).unwrap();
    let pages = report::prepare(&mut ui, rep);
    assert_eq!(
        pages.pages[0].texts(),
        [
            "Pedido 1 de Cliente 1",
            "Itens de Cliente 1: 2",
            "P1.1 = 10 (11)",
            "P1.2 = 20 (21)",
            "Soma do pedido 1: 30",
            // Sem itens: nem cabeçalho nem rodapé do detalhe.
            "Pedido 2 de Cliente 2",
            "Pedido 3 de Cliente 3",
            "Itens de Cliente 3: 3",
            "P3.1 = 10 (13)",
            "P3.2 = 20 (23)",
            "P3.3 = 30 (33)",
            "Soma do pedido 3: 60",
            "Pedidos: 3",
        ]
    );
    // O cursor do mestre volta para onde estava.
    assert_eq!(view(&ui, orders).rec_no(), Some(1));
}

#[test]
fn walking_the_master_does_not_scroll_the_linked_controls() {
    // As grades do mestre e do detalhe não podem ver o relatório passando pelos registros:
    // antes, ao montar a visualização, a grade do mestre acompanhava cada registro e, na
    // volta, deixava o registro clicado no topo.
    let (mut ui, rep, orders) = master_detail(&[2, 4, 3]);
    let lines = ui.components().filter(|c| ui.class_name(*c) == Some("ClientDataSet")).nth(1).unwrap();
    data_set(&mut ui, orders).move_to(1).unwrap();
    data_set(&mut ui, lines).move_to(2).unwrap();
    let scrolls = std::rc::Rc::new(std::cell::Cell::new(0));
    for ds in [orders, lines] {
        let s = scrolls.clone();
        data_set(&mut ui, ds).on_after_scroll(move |_, _| {
            s.set(s.get() + 1);
            Ok(())
        });
    }
    let pages = report::prepare(&mut ui, rep);
    assert!(pages.pages[0].texts().contains(&"P3.3 = 30 (33)"));
    assert_eq!(scrolls.get(), 0, "ninguém soube do percurso");
    assert_eq!(view(&ui, orders).rec_no(), Some(1));
    assert_eq!(view(&ui, lines).rec_no(), Some(2), "o detalhe volta ao registro dele");
    assert_eq!(view(&ui, lines).text("produto").unwrap(), "P2.3");
}

#[test]
fn sub_details_break_pages_and_the_preview_follows_the_detail() {
    let (mut ui, rep, _) = master_detail(&[40, 40]);
    let pages = report::prepare(&mut ui, rep);
    assert!(pages.pages.len() >= 2, "{} páginas", pages.pages.len());
    let all: Vec<&str> = pages.pages.iter().flat_map(|p| p.texts()).collect();
    assert_eq!(all.iter().filter(|t| t.starts_with("P2.")).count(), 40);
    assert!(all.contains(&"Soma do pedido 2: 8200"));
    let form = ui.roots()[0];
    let preview = ui.add(ReportPreview::new(rep), form);
    ReportPreview::refresh(&mut ui, preview);
    // Mudou um item: a visualização monta de novo sozinha.
    let lines = ui.components().filter(|c| ui.class_name(*c) == Some("ClientDataSet")).nth(1).unwrap();
    {
        let mut d = data_set(&mut ui, lines);
        d.edit().unwrap();
        d.set_value("produto", "Trocado").unwrap();
        d.post().unwrap();
    }
    ui.process_posted();
    let texts: Vec<String> = ui.get::<ReportPreview>(preview).unwrap().pages().unwrap().pages.iter().flat_map(|p| p.texts()).map(str::to_string).collect();
    assert!(texts.iter().any(|t| t.starts_with("Trocado = ")), "{texts:?}");
}

/// Relatório grande (item 23): o PDF gravado página por página (`write_pdf`) tem as mesmas
/// páginas do `to_pdf`, com o `[PAGES]` certo, sem guardar as páginas na memória.
#[test]
fn a_big_report_is_written_page_by_page_with_the_same_pages() {
    let (mut ui, rep, _) = report(1500);
    let whole = report::to_pdf(&mut ui, rep);
    let pages = report::prepare(&mut ui, rep).pages.len();
    assert!(pages > 50, "páginas: {pages}");
    let mut streamed = Vec::new();
    let written = report::write_pdf(&mut ui, rep, &mut streamed).unwrap();
    assert_eq!(written, pages);
    assert_eq!(streamed, whole, "o mesmo PDF, byte a byte");
    let text = String::from_utf8_lossy(&streamed);
    assert!(text.contains(&format!("{pages} de {pages}")), "o [PAGES] da última página");
    assert!(text.contains(&format!("/Count {pages}")));
    // A memória não cresce com os registros (conferência de 08/11: 30.000 registros deixavam
    // +1 MB e 300.000, +16 MB, nas marcas de repintura de cada rótulo e nos valores dos
    // totais juntados numa lista). Fica só o índice do PDF, uns 24 bytes por página.
    let (small_pages, small_live, small_peak) = write_pdf_memory(3_000);
    let (big_pages, big_live, big_peak) = write_pdf_memory(30_000);
    println!("write_pdf: 3.000 registros ({small_pages} páginas) vivo {small_live:+.3} MB, pico {small_peak:+.3} MB; 30.000 ({big_pages} páginas) vivo {big_live:+.3} MB, pico {big_peak:+.3} MB");
    assert!(small_live.abs() < 0.01 && big_live.abs() < 0.01, "nada fica depois: {small_live:+.3} e {big_live:+.3} MB");
    assert!(big_peak < 0.5, "o pico com 30.000 registros: {big_peak:+.3} MB");
    assert!(big_peak - small_peak < 0.1, "10 vezes os registros, o mesmo pico: {small_peak:+.3} → {big_peak:+.3} MB");
}

/// O mesmo com 300.000 registros (7.501 páginas): demora, por isso fica de fora do `cargo
/// test` comum. `cargo test --release -p aylis-dbcontrols --test report -- --ignored --nocapture`
#[test]
#[ignore]
fn a_huge_report_keeps_the_memory_flat() {
    let (pages, live, peak) = write_pdf_memory(300_000);
    println!("write_pdf: 300.000 registros ({pages} páginas) vivo {live:+.3} MB, pico {peak:+.3} MB");
    assert!(live.abs() < 0.01, "nada fica depois: {live:+.3} MB");
    assert!(peak < 1.0, "o pico: {peak:+.3} MB (só o índice do PDF cresce, ~24 bytes por página)");
}

/// A memória do `write_pdf` com `rows` registros (metade em cada região), gravando num
/// destino que não guarda nada: (páginas, o que ficou vivo depois, o pico durante), em MB.
fn write_pdf_memory(rows: usize) -> (usize, f64, f64) {
    let (mut ui, rep, _) = report(rows / 2);
    let mb = |b: isize| b as f64 / (1024.0 * 1024.0);
    let base = meter::start();
    let pages = report::write_pdf(&mut ui, rep, std::io::sink()).unwrap();
    let (live, peak) = meter::since(base);
    (pages, mb(live), mb(peak))
}
