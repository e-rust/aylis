//! O DocViewer abre os documentos de teste (`tests/data`, gerados por
//! `aylis-tool test-docs`).

use aylis_docview::office::{Block, read_document, read_sheets};
use aylis_docview::pdf::{page_sizes, render_page};

fn data(name: &str) -> Vec<u8> {
    std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(name)).unwrap()
}

#[test]
fn pdf_pages_render_with_text_and_shapes() {
    let bytes = data("sample.pdf");
    assert_eq!(page_sizes(&bytes).unwrap(), vec![(612.0, 792.0), (612.0, 792.0)]);
    let pdf = hayro::hayro_syntax::Pdf::new(bytes).unwrap();
    let (w, h, rgba) = render_page(&pdf, 0, 1.0).unwrap();
    assert_eq!((w, h), (612, 792));
    let px = |x: u32, y: u32| {
        let i = ((y * w + x) * 4) as usize;
        [rgba[i], rgba[i + 1], rgba[i + 2]]
    };
    // O retângulo azul (72..372 x, 600..660 em coordenadas do PDF, de baixo para cima).
    let [r, g, b] = px(200, 792 - 630);
    assert!(b > 200 && r < 80, "azul: {:?}", (r, g, b));
    // O título em preto: algum pixel escuro na linha do texto.
    let dark = (72..400).any(|x| px(x, 792 - 725)[0] < 100);
    assert!(dark, "o texto apareceu");
    // Fundo branco.
    assert_eq!(px(5, 5), [255, 255, 255]);
}

#[test]
fn xlsx_sheets_and_values() {
    let sheets = read_sheets(&data("sample.xlsx"), "xlsx").unwrap();
    assert_eq!(sheets.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["Vendas", "Clientes"]);
    assert_eq!(sheets[0].rows[1], vec!["Caneta", "10", "2.5"]);
    assert_eq!(sheets[1].rows[1], vec!["Ana", "Recife"]);
}

#[test]
fn docx_text() {
    let blocks = read_document(&data("sample.docx"), "docx").unwrap();
    assert_eq!(blocks[0], Block::Paragraph { text: "Contrato".into(), level: 1 });
    assert_eq!(blocks[1], Block::Paragraph { text: "Este documento foi aberto pelo aylis, sem o Word.".into(), level: 0 });
    assert_eq!(blocks[2], Block::Table(vec![vec!["Item".into(), "Preço".into()], vec!["Licença".into(), "100".into()]]));
}

/// Um ZIP pequeno com uma entrada de `size` bytes de zeros (comprime ~1000:1).
fn zip_bomb(entry: &str, size: usize) -> Vec<u8> {
    use std::io::Write;
    let mut out = std::io::Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut out);
    zip.start_file(entry, zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated)).unwrap();
    let chunk = vec![0u8; 1024 * 1024];
    for _ in 0..size / chunk.len() {
        zip.write_all(&chunk).unwrap();
    }
    zip.finish().unwrap();
    out.into_inner()
}

#[test]
fn zip_bombs_are_refused() {
    let mb = 1024 * 1024;
    let docx = zip_bomb("word/document.xml", 20 * mb);
    assert!(docx.len() < mb, "a bomba é pequena: {} bytes", docx.len());
    let err = aylis_docview::office::read_document_with(&docx, "docx", 10 * mb).unwrap_err();
    assert!(err.contains("grande demais"), "{err}");
    let xlsx = zip_bomb("xl/worksheets/sheet1.xml", 20 * mb);
    let err = aylis_docview::office::read_sheets_with(&xlsx, "xlsx", 10 * mb).unwrap_err();
    assert!(err.contains("grande demais"), "{err}");
    // A razão absurda também barra, mesmo dentro do tamanho.
    assert!(aylis_docview::office::read_sheets_with(&xlsx, "xlsx", 100 * mb).is_err());
    // Os documentos normais continuam abrindo com os limites padrão.
    assert!(read_document(&data("sample.docx"), "docx").is_ok());
    assert!(read_sheets(&data("sample.xlsx"), "xlsx").is_ok());
}

#[test]
fn huge_pdf_pages_are_rendered_smaller() {
    let pdf = hayro::hayro_syntax::Pdf::new(data("sample.pdf")).unwrap();
    // Com zoom 10, a página daria 6120 × 7920 (48 MP); com 1 MP de limite, cabe nele.
    let (w, h, rgba) = aylis_docview::pdf::render_page_with(&pdf, 0, 10.0, 1_000_000.0).unwrap();
    assert!((w as u64 * h as u64) <= 1_010_000, "{w} × {h}");
    assert_eq!(rgba.len(), (w * h * 4) as usize);
}
