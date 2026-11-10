//! Leitura de planilhas (XLSX, XLSM, XLS, XLSB, ODS pelo `calamine`; CSV) e de textos
//! (DOCX, ODT: o XML de dentro do zip) para o que o visualizador mostra.

use std::io::{Cursor, Read};

/// Uma planilha: nome e células (linhas de colunas, já em texto).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sheet {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

/// Um bloco de texto do documento.
#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    /// Parágrafo; `level` > 0 é título (1 = o maior).
    Paragraph { text: String, level: u8 },
    /// Tabela: linhas de células.
    Table(Vec<Vec<String>>),
}

/// Lê uma planilha pela extensão.
pub fn read_sheets(bytes: &[u8], extension: &str) -> Result<Vec<Sheet>, String> {
    read_sheets_with(bytes, extension, MAX_SHEETS)
}

/// Como [`read_sheets`], com o maior conteúdo descompactado (`max` bytes). O `calamine` não
/// aceita limite: o índice do ZIP é conferido antes (XLSX, XLSM, XLSB, ODS); o XLS antigo e o
/// CSV, pelo tamanho do arquivo.
pub fn read_sheets_with(bytes: &[u8], extension: &str, max: usize) -> Result<Vec<Sheet>, String> {
    if bytes.starts_with(b"PK") {
        check_zip(bytes, max)?;
    } else if bytes.len() > max {
        return Err(too_big());
    }
    if extension == "csv" || extension == "tsv" {
        return Ok(vec![Sheet { name: "CSV".into(), rows: parse_csv(&String::from_utf8_lossy(bytes)) }]);
    }
    use calamine::{Data, Reader};
    let mut book = calamine::open_workbook_auto_from_rs(Cursor::new(bytes)).map_err(|e| e.to_string())?;
    let names = book.sheet_names().to_vec();
    let mut out = Vec::new();
    for name in names {
        let range = book.worksheet_range(&name).map_err(|e| e.to_string())?;
        let rows = range
            .rows()
            .map(|r| {
                r.iter()
                    .map(|c| match c {
                        Data::Empty => String::new(),
                        Data::String(s) => s.clone(),
                        Data::Float(f) => {
                            if f.fract() == 0.0 && f.abs() < 1e15 { format!("{}", *f as i64) } else { format!("{f}") }
                        }
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => if *b { "VERDADEIRO" } else { "FALSO" }.to_string(),
                        Data::DateTime(d) => excel_date(d.as_f64()),
                        Data::DateTimeIso(s) | Data::DurationIso(s) => s.clone(),
                        Data::Error(e) => format!("#{e:?}"),
                    })
                    .collect()
            })
            .collect();
        out.push(Sheet { name, rows });
    }
    Ok(out)
}

/// Número de série do Excel (dias desde 30/12/1899) em `dd/mm/aaaa` (e `hh:mm`, se tiver).
fn excel_date(serial: f64) -> String {
    let days = serial.floor() as i64;
    let minutes = ((serial - serial.floor()) * 1440.0).round() as i64;
    // Dias desde 1970-01-01, depois o calendário civil (Howard Hinnant).
    let z = days - 25569 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    if minutes == 0 { format!("{d:02}/{m:02}/{y}") } else { format!("{d:02}/{m:02}/{y} {:02}:{:02}", minutes / 60, minutes % 60) }
}

/// CSV com `;`, `,` ou tabulação (a que aparecer mais na primeira linha), com aspas.
pub fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let first = text.lines().next().unwrap_or("");
    let sep = [';', ',', '\t'].into_iter().max_by_key(|s| first.matches(*s).count()).unwrap_or(',');
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                cell.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            c if c == sep && !quoted => row.push(std::mem::take(&mut cell)),
            '\n' if !quoted => {
                row.push(std::mem::take(&mut cell).trim_end_matches('\r').to_string());
                rows.push(std::mem::take(&mut row));
            }
            c => cell.push(c),
        }
    }
    if !cell.is_empty() || !row.is_empty() {
        row.push(cell);
        rows.push(row);
    }
    rows
}

/// O padrão do maior texto descompactado de um DOCX ou ODT.
pub const MAX_DOCUMENT: usize = 64 * 1024 * 1024;
/// O padrão do maior conteúdo descompactado de uma planilha (todas as entradas do ZIP).
pub const MAX_SHEETS: usize = 256 * 1024 * 1024;
/// Acima desta razão entre descompactado e compactado, o ZIP é tratado como bomba.
pub const MAX_RATIO: u64 = 100;

fn too_big() -> String {
    "documento grande demais ou corrompido".into()
}

fn zip_entry(bytes: &[u8], name: &str, max: usize) -> Result<String, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
    let file = archive.by_name(name).map_err(|_| format!("{name} não encontrado no arquivo"))?;
    // O tamanho declarado pode mentir: a leitura também para no limite.
    if file.size() > max as u64 {
        return Err(too_big());
    }
    let mut xml = String::new();
    file.take(max as u64 + 1).read_to_string(&mut xml).map_err(|e| e.to_string())?;
    if xml.len() > max {
        return Err(too_big());
    }
    Ok(xml)
}

/// Uma bomba de ZIP é recusada pelo índice (sem descompactar nada): o total descompactado
/// acima de `max` ou uma razão absurda de compressão.
fn check_zip(bytes: &[u8], max: usize) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
    let (mut total, mut packed) = (0u64, 0u64);
    for i in 0..archive.len() {
        let entry = archive.by_index_raw(i).map_err(|e| e.to_string())?;
        total = total.saturating_add(entry.size());
        packed = packed.saturating_add(entry.compressed_size());
    }
    if total > max as u64 || total > packed.max(1).saturating_mul(MAX_RATIO).max(1024 * 1024) {
        return Err(too_big());
    }
    Ok(())
}

/// Lê um texto (DOCX ou ODT) em blocos.
pub fn read_document(bytes: &[u8], extension: &str) -> Result<Vec<Block>, String> {
    read_document_with(bytes, extension, MAX_DOCUMENT)
}

/// Como [`read_document`], com o maior texto descompactado (`max` bytes).
pub fn read_document_with(bytes: &[u8], extension: &str, max: usize) -> Result<Vec<Block>, String> {
    if extension == "odt" { odt(&zip_entry(bytes, "content.xml", max)?) } else { docx(&zip_entry(bytes, "word/document.xml", max)?) }
}

fn local(name: &str) -> &str {
    name.split_once(':').map_or(name, |(_, local)| local)
}

fn attr(e: &quick_xml::events::BytesStart, key: &str) -> Option<String> {
    e.attributes().flatten().find(|a| local(a.key.as_ref()) == key).and_then(|a| a.normalized_value(quick_xml::XmlVersion::Implicit1_0).ok().map(|v| v.into_owned()))
}

/// Uma referência no texto (`&amp;`, `&#233;`): o quick-xml entrega à parte.
fn push_ref(out: &mut String, r: &quick_xml::events::BytesRef) {
    if let Ok(s) = quick_xml::escape::unescape(&format!("&{};", r.xml10_content())) {
        out.push_str(&s);
    }
}

/// Nível de título pelo estilo (`Heading1`, `Ttulo1`, `Title`...).
fn heading_level(style: &str) -> u8 {
    let lower = style.to_lowercase();
    if lower == "title" || lower == "titulo" || lower == "ttulo" {
        return 1;
    }
    if lower.starts_with("heading") || lower.starts_with("ttulo") || lower.starts_with("titulo") {
        return lower.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(1).clamp(1, 6);
    }
    0
}

fn docx(xml: &str) -> Result<Vec<Block>, String> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut blocks = Vec::new();
    let (mut text, mut level) = (String::new(), 0u8);
    let mut in_text = false;
    // Tabelas: pilha de (linhas, linha atual, célula atual).
    let mut tables: Vec<(Vec<Vec<String>>, Vec<String>, String)> = Vec::new();
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) | Event::Empty(e) if local(e.name().as_ref()) == "pStyle" => {
                level = attr(&e, "val").map_or(0, |s| heading_level(&s));
            }
            Event::Start(e) => match local(e.name().as_ref()) {
                "t" => in_text = true,
                "tbl" => tables.push((Vec::new(), Vec::new(), String::new())),
                "p" => {
                    text.clear();
                    level = 0;
                }
                _ => {}
            },
            Event::Empty(e) => match local(e.name().as_ref()) {
                "tab" => text.push('\t'),
                "br" => text.push('\n'),
                _ => {}
            },
            Event::Text(t) if in_text => text.push_str(&t.xml10_content()),
            Event::GeneralRef(r) if in_text => push_ref(&mut text, &r),
            Event::End(e) => match local(e.name().as_ref()) {
                "t" => in_text = false,
                "p" => {
                    if let Some((_, _, cell)) = tables.last_mut() {
                        if !cell.is_empty() {
                            cell.push('\n');
                        }
                        cell.push_str(&text);
                    } else {
                        blocks.push(Block::Paragraph { text: std::mem::take(&mut text), level });
                    }
                }
                "tc" => {
                    if let Some((_, row, cell)) = tables.last_mut() {
                        row.push(std::mem::take(cell));
                    }
                }
                "tr" => {
                    if let Some((rows, row, _)) = tables.last_mut() {
                        rows.push(std::mem::take(row));
                    }
                }
                "tbl" => {
                    if let Some((rows, _, _)) = tables.pop() {
                        blocks.push(Block::Table(rows));
                    }
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(blocks)
}

fn odt(xml: &str) -> Result<Vec<Block>, String> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut blocks = Vec::new();
    let mut stack: Vec<(String, u8)> = Vec::new();
    let mut tables: Vec<(Vec<Vec<String>>, Vec<String>)> = Vec::new();
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) => match local(e.name().as_ref()) {
                "p" => stack.push((String::new(), 0)),
                "h" => stack.push((String::new(), attr(&e, "outline-level").and_then(|l| l.parse().ok()).unwrap_or(1))),
                "table" => tables.push((Vec::new(), Vec::new())),
                _ => {}
            },
            Event::Empty(e) => {
                if let Some((t, _)) = stack.last_mut() {
                    match local(e.name().as_ref()) {
                        "tab" => t.push('\t'),
                        "line-break" => t.push('\n'),
                        "s" => t.push(' '),
                        _ => {}
                    }
                }
            }
            Event::Text(t) => {
                if let Some((text, _)) = stack.last_mut() {
                    text.push_str(&t.xml10_content());
                }
            }
            Event::GeneralRef(r) => {
                if let Some((text, _)) = stack.last_mut() {
                    push_ref(text, &r);
                }
            }
            Event::End(e) => match local(e.name().as_ref()) {
                "p" | "h" => {
                    if let Some((text, level)) = stack.pop() {
                        if let Some((_, row)) = tables.last_mut() {
                            row.push(text);
                        } else {
                            blocks.push(Block::Paragraph { text, level });
                        }
                    }
                }
                "table-row" => {
                    if let Some((rows, row)) = tables.last_mut() {
                        rows.push(std::mem::take(row));
                    }
                }
                "table" => {
                    if let Some((rows, _)) = tables.pop() {
                        blocks.push(Block::Table(rows));
                    }
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entities_in_the_text_and_in_attributes_are_resolved() {
        let docx_xml = r#"<w:document xmlns:w="w"><w:body><w:p><w:pPr><w:pStyle w:val="Heading&#50;"/></w:pPr><w:r><w:t>Caf&#233; &amp; p&#xE3;o &lt;3</w:t></w:r></w:p></w:body></w:document>"#;
        let blocks = docx(docx_xml).unwrap();
        assert!(matches!(&blocks[..], [Block::Paragraph { text, level: 2 }] if text == "Café & pão <3"), "{blocks:?}");
        let odt_xml = r#"<office:text xmlns:office="o" xmlns:text="t"><text:p>A &amp; B&#33;</text:p></office:text>"#;
        assert!(matches!(&odt(odt_xml).unwrap()[..], [Block::Paragraph { text, .. }] if text == "A & B!"));
    }

    #[test]
    fn excel_serial_dates() {
        assert_eq!(excel_date(45000.0), "15/03/2023");
        assert_eq!(excel_date(45000.5), "15/03/2023 12:00");
        assert_eq!(excel_date(1.0), "31/12/1899");
    }

    #[test]
    fn csv_with_quotes_and_semicolons() {
        let rows = parse_csv("nome;valor\n\"Silva; João\";10\n\"diz \"\"oi\"\"\";2\n");
        assert_eq!(rows, vec![vec!["nome", "valor"], vec!["Silva; João", "10"], vec!["diz \"oi\"", "2"]]);
    }

    #[test]
    fn docx_paragraphs_headings_and_tables() {
        let xml = r#"<w:document xmlns:w="x"><w:body>
            <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Relatório</w:t></w:r></w:p>
            <w:p><w:r><w:t xml:space="preserve">Olá </w:t></w:r><w:r><w:t>mundo &amp; cia</w:t></w:r></w:p>
            <w:tbl><w:tr><w:tc><w:p><w:r><w:t>A</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>B</w:t></w:r></w:p></w:tc></w:tr></w:tbl>
        </w:body></w:document>"#;
        let blocks = docx(xml).unwrap();
        assert_eq!(
            blocks,
            vec![
                Block::Paragraph { text: "Relatório".into(), level: 1 },
                Block::Paragraph { text: "Olá mundo & cia".into(), level: 0 },
                Block::Table(vec![vec!["A".into(), "B".into()]]),
            ]
        );
    }

    #[test]
    fn odt_paragraphs_and_headings() {
        let xml = r#"<office:document-content xmlns:office="o" xmlns:text="t"><office:body><office:text>
            <text:h text:outline-level="2">Seção</text:h><text:p>Linha<text:s/>um</text:p>
        </office:text></office:body></office:document-content>"#;
        assert_eq!(odt(xml).unwrap(), vec![Block::Paragraph { text: "Seção".into(), level: 2 }, Block::Paragraph { text: "Linha um".into(), level: 0 }]);
    }
}
