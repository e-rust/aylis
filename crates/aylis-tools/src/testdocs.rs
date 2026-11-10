//! Os documentos de teste do `aylis-docview` (`crates/aylis-docview/tests/data`): um PDF de
//! duas páginas, uma planilha XLSX com duas abas e um DOCX com título, parágrafo e tabela,
//! montados à mão (sem bibliotecas de Office).
//!
//! ```text
//! aylis-tool test-docs
//! ```

use std::io::Write;
use std::path::Path;

/// Grava os três arquivos em `dir`.
pub fn generate(dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    std::fs::write(dir.join("teste.pdf"), pdf()).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("teste.xlsx"), xlsx()?).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("teste.docx"), docx()?).map_err(|e| e.to_string())?;
    Ok(())
}

fn pdf() -> Vec<u8> {
    let pages = [
        "BT /F1 28 Tf 72 720 Td (Relatorio do aylis) Tj ET 0.15 0.39 0.92 rg 72 600 300 60 re f",
        "BT /F1 18 Tf 72 720 Td (Pagina 2) Tj ET 0.9 0.2 0.2 rg 200 400 120 120 re f",
    ];
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into(), String::new(), "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into()];
    let mut kids = Vec::new();
    for content in pages {
        objs.push(format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len()));
        let content_id = objs.len();
        objs.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents {content_id} 0 R /Resources << /Font << /F1 3 0 R >> >> >>"));
        kids.push(format!("{} 0 R", objs.len()));
    }
    objs[1] = format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids.join(" "), kids.len());
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (i, body) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{body}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// Uma célula: número ou texto.
enum Cell {
    Num(f64),
    Text(&'static str),
}

fn sheet(rows: &[&[Cell]]) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>"#);
    for (r, row) in rows.iter().enumerate() {
        let r = r + 1;
        xml.push_str(&format!(r#"<row r="{r}">"#));
        for (c, v) in row.iter().enumerate() {
            let reference = format!("{}{r}", (b'A' + c as u8) as char);
            match v {
                Cell::Num(n) => xml.push_str(&format!(r#"<c r="{reference}"><v>{n}</v></c>"#)),
                Cell::Text(t) => xml.push_str(&format!(r#"<c r="{reference}" t="inlineStr"><is><t>{t}</t></is></c>"#)),
            }
        }
        xml.push_str("</row>");
    }
    xml.push_str("</sheetData></worksheet>");
    xml
}

fn zip_of(files: &[(&str, String)]) -> Result<Vec<u8>, String> {
    let mut z = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let opts = zip::write::SimpleFileOptions::default();
    for (name, text) in files {
        z.start_file(*name, opts).map_err(|e| e.to_string())?;
        z.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(z.finish().map_err(|e| e.to_string())?.into_inner())
}

fn xlsx() -> Result<Vec<u8>, String> {
    use Cell::{Num, Text};
    zip_of(&[
        ("[Content_Types].xml", r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#.into()),
        ("_rels/.rels", r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#.into()),
        ("xl/workbook.xml", r#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Vendas" sheetId="1" r:id="rId1"/><sheet name="Clientes" sheetId="2" r:id="rId2"/></sheets></workbook>"#.into()),
        ("xl/_rels/workbook.xml.rels", r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/></Relationships>"#.into()),
        ("xl/worksheets/sheet1.xml", sheet(&[&[Text("Produto"), Text("Qtd"), Text("Valor")], &[Text("Caneta"), Num(10.0), Num(2.5)], &[Text("Caderno"), Num(3.0), Num(19.9)]])),
        ("xl/worksheets/sheet2.xml", sheet(&[&[Text("Nome"), Text("Cidade")], &[Text("Ana"), Text("Recife")]])),
    ])
}

fn docx() -> Result<Vec<u8>, String> {
    let body = concat!(
        r#"<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Contrato</w:t></w:r></w:p>"#,
        r#"<w:p><w:r><w:t xml:space="preserve">Este documento foi aberto pelo aylis, </w:t></w:r><w:r><w:t>sem o Word.</w:t></w:r></w:p>"#,
        r#"<w:tbl><w:tr><w:tc><w:p><w:r><w:t>Item</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>Preço</w:t></w:r></w:p></w:tc></w:tr>"#,
        r#"<w:tr><w:tc><w:p><w:r><w:t>Licença</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>100</w:t></w:r></w:p></w:tc></w:tr></w:tbl>"#,
    );
    zip_of(&[
        ("[Content_Types].xml", r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="application/xml"/></Types>"#.into()),
        ("word/document.xml", format!(r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{body}</w:body></w:document>"#)),
    ])
}
