//! NDEF, o formato das tags NFC: ler e montar as mensagens (os registros de texto, endereço e
//! MIME) em Rust puro, para todas as plataformas falarem do mesmo jeito. O Android e os
//! leitores USB (PC/SC) entregam e recebem os bytes; o navegador (Web NFC) já entrega os
//! registros prontos.
//!
//! No componente `Nfc`, os registros vão em texto, um por linha: `text:Olá`,
//! `uri:https://aylis.dev`, `application/json:{"a":1}` (o tipo MIME antes dos dois pontos).

/// Um registro de uma mensagem NDEF.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NdefRecord {
    /// Texto (`T`), com o idioma (`pt`, `en`...).
    Text { text: String, lang: String },
    /// Endereço (`U`): `https://...`, `tel:...`, `mailto:...`.
    Uri(String),
    /// Dados com tipo MIME (`application/json`, `text/vcard`...).
    Mime { mime: String, data: Vec<u8> },
    /// Outro tipo (externo, vazio, desconhecido): o TNF, o tipo e os bytes como vieram.
    Other { tnf: u8, kind: Vec<u8>, data: Vec<u8> },
}

/// Os prefixos que o registro `U` abrevia num byte (a tabela do NFC Forum).
const URI_PREFIXES: [&str; 36] = [
    "",
    "http://www.",
    "https://www.",
    "http://",
    "https://",
    "tel:",
    "mailto:",
    "ftp://anonymous:anonymous@",
    "ftp://ftp.",
    "ftps://",
    "sftp://",
    "smb://",
    "nfs://",
    "ftp://",
    "dav://",
    "news:",
    "telnet://",
    "imap:",
    "rtsp://",
    "urn:",
    "pop:",
    "sip:",
    "sips:",
    "tftp:",
    "btspp://",
    "btl2cap://",
    "btgoep://",
    "tcpobex://",
    "irdaobex://",
    "file://",
    "urn:epc:id:",
    "urn:epc:tag:",
    "urn:epc:pat:",
    "urn:epc:raw:",
    "urn:epc:",
    "urn:nfc:",
];

impl NdefRecord {
    pub fn text(text: impl Into<String>) -> Self {
        NdefRecord::Text { text: text.into(), lang: "pt".into() }
    }

    /// A linha do registro no texto do componente (`text:Olá`).
    pub fn to_line(&self) -> String {
        match self {
            NdefRecord::Text { text, .. } => format!("text:{}", one_line(text)),
            NdefRecord::Uri(uri) => format!("uri:{uri}"),
            NdefRecord::Mime { mime, data } => format!("{mime}:{}", one_line(&String::from_utf8_lossy(data))),
            NdefRecord::Other { tnf, kind, data } => {
                format!("other/{tnf}/{}:{}", String::from_utf8_lossy(kind), data.iter().map(|b| format!("{b:02X}")).collect::<String>())
            }
        }
    }

    /// Lê uma linha (`text:...`, `uri:...`, `tipo/mime:...`). Sem prefixo, é texto.
    pub fn from_line(line: &str) -> Option<Self> {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            return None;
        }
        let (kind, value) = line.split_once(':').unwrap_or(("text", line));
        let value = value.replace("\\n", "\n");
        Some(match kind.trim() {
            "text" => NdefRecord::text(value),
            "uri" | "url" => NdefRecord::Uri(value),
            mime if mime.contains('/') && !mime.starts_with("other/") => NdefRecord::Mime { mime: mime.to_string(), data: value.into_bytes() },
            // `https://...` e afins sem o `uri:` na frente.
            scheme if value.starts_with("//") || URI_PREFIXES.iter().any(|p| p.trim_end_matches(['/', ':']) == scheme) => NdefRecord::Uri(line.to_string()),
            _ => NdefRecord::text(line),
        })
    }

    /// TNF, tipo e carga do registro.
    fn parts(&self) -> (u8, Vec<u8>, Vec<u8>) {
        match self {
            NdefRecord::Text { text, lang } => {
                let lang = lang.as_bytes();
                let mut data = vec![lang.len() as u8 & 0x3F];
                data.extend_from_slice(lang);
                data.extend_from_slice(text.as_bytes());
                (1, b"T".to_vec(), data)
            }
            NdefRecord::Uri(uri) => {
                // O prefixo mais longo que serve vira o byte de abreviação.
                let (code, prefix) = URI_PREFIXES.iter().enumerate().skip(1).filter(|(_, p)| uri.starts_with(**p)).max_by_key(|(_, p)| p.len()).unwrap_or((0, &""));
                let mut data = vec![code as u8];
                data.extend_from_slice(uri[prefix.len()..].as_bytes());
                (1, b"U".to_vec(), data)
            }
            NdefRecord::Mime { mime, data } => (2, mime.as_bytes().to_vec(), data.clone()),
            NdefRecord::Other { tnf, kind, data } => (*tnf, kind.clone(), data.clone()),
        }
    }
}

/// Uma linha só (a quebra vira `\n`, que o [`NdefRecord::from_line`] desfaz).
fn one_line(text: &str) -> String {
    text.replace('\n', "\\n")
}

/// Os registros em texto, um por linha.
pub fn records_to_text(records: &[NdefRecord]) -> String {
    records.iter().map(NdefRecord::to_line).collect::<Vec<_>>().join("\n")
}

/// O texto (uma linha por registro) em registros.
pub fn records_from_text(text: &str) -> Vec<NdefRecord> {
    text.lines().filter_map(NdefRecord::from_line).collect()
}

/// Monta a mensagem NDEF (os bytes que vão para a tag).
pub fn encode(records: &[NdefRecord]) -> Vec<u8> {
    let mut out = Vec::new();
    let n = records.len();
    for (i, r) in records.iter().enumerate() {
        let (tnf, kind, data) = r.parts();
        let short = data.len() < 256;
        let mut header = tnf & 0x07;
        if i == 0 {
            header |= 0x80; // MB: começo da mensagem
        }
        if i + 1 == n {
            header |= 0x40; // ME: fim da mensagem
        }
        if short {
            header |= 0x10; // SR: tamanho num byte
        }
        out.push(header);
        out.push(kind.len() as u8);
        if short {
            out.push(data.len() as u8);
        } else {
            out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        }
        out.extend_from_slice(&kind);
        out.extend_from_slice(&data);
    }
    if out.is_empty() {
        // Mensagem vazia: um registro vazio com MB e ME.
        out.extend_from_slice(&[0xD0, 0, 0]);
    }
    out
}

/// Lê a mensagem NDEF. Registros em pedaços (o bit CF) são juntados; bytes a mais ou
/// faltando terminam a leitura com o que já deu para ler.
pub fn decode(bytes: &[u8]) -> Vec<NdefRecord> {
    let mut out = Vec::new();
    let mut i = 0;
    let mut chunked: Option<(u8, Vec<u8>, Vec<u8>)> = None;
    while i + 3 <= bytes.len() {
        let header = bytes[i];
        let tnf = header & 0x07;
        let short = header & 0x10 != 0;
        let has_id = header & 0x08 != 0;
        let chunk = header & 0x20 != 0;
        let kind_len = bytes[i + 1] as usize;
        i += 2;
        let data_len = if short {
            let n = bytes[i] as usize;
            i += 1;
            n
        } else {
            let Some(b) = bytes.get(i..i + 4) else { break };
            i += 4;
            u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize
        };
        let id_len = if has_id {
            let Some(n) = bytes.get(i) else { break };
            i += 1;
            *n as usize
        } else {
            0
        };
        let Some(kind) = bytes.get(i..i + kind_len) else { break };
        i += kind_len + id_len;
        let Some(data) = bytes.get(i..i + data_len) else { break };
        i += data_len;
        match (&mut chunked, chunk) {
            // Os pedaços do meio têm TNF 6 (inalterado).
            (Some((_, _, all)), _) if tnf == 6 => {
                all.extend_from_slice(data);
                if !chunk {
                    let (t, k, d) = chunked.take().unwrap_or_default();
                    out.push(record(t, &k, d));
                }
            }
            (None, true) => chunked = Some((tnf, kind.to_vec(), data.to_vec())),
            _ => out.push(record(tnf, kind, data.to_vec())),
        }
        if header & 0x40 != 0 {
            break;
        }
    }
    out
}

fn record(tnf: u8, kind: &[u8], data: Vec<u8>) -> NdefRecord {
    match (tnf, kind) {
        (1, b"T") if !data.is_empty() => {
            let status = data[0];
            let lang_len = (status & 0x3F) as usize;
            let lang = String::from_utf8_lossy(data.get(1..1 + lang_len).unwrap_or_default()).into_owned();
            let rest = data.get(1 + lang_len..).unwrap_or_default();
            // O bit 7 diz UTF-16 (raro).
            let text = if status & 0x80 != 0 {
                let units: Vec<u16> = rest.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
                String::from_utf16_lossy(&units)
            } else {
                String::from_utf8_lossy(rest).into_owned()
            };
            NdefRecord::Text { text, lang }
        }
        (1, b"U") if !data.is_empty() => {
            let prefix = URI_PREFIXES.get(data[0] as usize).copied().unwrap_or("");
            NdefRecord::Uri(format!("{prefix}{}", String::from_utf8_lossy(&data[1..])))
        }
        (2, mime) => NdefRecord::Mime { mime: String::from_utf8_lossy(mime).into_owned(), data },
        (3, uri) => NdefRecord::Uri(String::from_utf8_lossy(uri).into_owned()),
        _ => NdefRecord::Other { tnf, kind: kind.to_vec(), data },
    }
}

/// A mensagem NDEF dentro da memória de uma tag do tipo 2 (NTAG, Ultralight: a partir da
/// página 4): o TLV `03 tamanho mensagem FE`. Os leitores USB leem assim.
pub fn from_type2_memory(memory: &[u8]) -> Option<Vec<u8>> {
    let mut i = 0;
    while i < memory.len() {
        match memory[i] {
            0x00 => i += 1,
            0xFE => return None,
            tag => {
                let (len, start) = match *memory.get(i + 1)? {
                    0xFF => (u16::from_be_bytes([*memory.get(i + 2)?, *memory.get(i + 3)?]) as usize, i + 4),
                    n => (n as usize, i + 2),
                };
                if tag == 0x03 {
                    return memory.get(start..start + len).map(<[u8]>::to_vec);
                }
                i = start + len;
            }
        }
    }
    None
}

/// O que gravar na memória de uma tag do tipo 2 (a partir da página 4) para guardar a
/// mensagem: o TLV com o terminador, completado até o fim da página (4 bytes).
pub fn to_type2_memory(message: &[u8]) -> Vec<u8> {
    let mut out = vec![0x03];
    if message.len() < 0xFF {
        out.push(message.len() as u8);
    } else {
        out.push(0xFF);
        out.extend_from_slice(&(message.len() as u16).to_be_bytes());
    }
    out.extend_from_slice(message);
    out.push(0xFE);
    while out.len() % 4 != 0 {
        out.push(0);
    }
    out
}

/// O id da tag em hexadecimal, com dois pontos (`04:A2:1B:...`), como o Android mostra.
pub fn id_text(id: &[u8]) -> String {
    id.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_go_and_come_back() {
        let records = vec![
            NdefRecord::text("Olá, aylis"),
            NdefRecord::Uri("https://www.aylis.dev/nfc".into()),
            NdefRecord::Uri("tel:+5511999999999".into()),
            NdefRecord::Mime { mime: "application/json".into(), data: br#"{"a":1}"#.to_vec() },
            NdefRecord::text("x".repeat(300)),
        ];
        let bytes = encode(&records);
        // O https://www. vira o byte 2.
        assert!(bytes.windows(2).any(|w| w == [2, b'a']));
        assert_eq!(decode(&bytes), records);
        let memory = to_type2_memory(&bytes);
        assert_eq!(memory.len() % 4, 0);
        assert_eq!(from_type2_memory(&memory).as_deref(), Some(&bytes[..]));
    }

    #[test]
    fn text_lines() {
        let text = "text:Olá\nuri:https://aylis.dev\napplication/json:{\"a\":1}\nsem prefixo\nhttps://exemplo.com";
        let records = records_from_text(text);
        assert_eq!(records[0], NdefRecord::text("Olá"));
        assert_eq!(records[1], NdefRecord::Uri("https://aylis.dev".into()));
        assert_eq!(records[2], NdefRecord::Mime { mime: "application/json".into(), data: b"{\"a\":1}".to_vec() });
        assert_eq!(records[3], NdefRecord::text("sem prefixo"));
        assert_eq!(records[4], NdefRecord::Uri("https://exemplo.com".into()));
        assert_eq!(records_to_text(&records[..3]), "text:Olá\nuri:https://aylis.dev\napplication/json:{\"a\":1}");
        assert_eq!(records_from_text(&records_to_text(&[NdefRecord::text("a\nb")])), [NdefRecord::text("a\nb")]);
    }

    #[test]
    fn a_real_tag_and_broken_bytes() {
        // Um NTAG213 com "https://aylis.dev", como o celular grava (cabeçalho D1 01 0A 55 04).
        let memory = [0x03, 0x0E, 0xD1, 0x01, 0x0A, 0x55, 0x04, b'a', b'y', b'l', b'i', b's', b'.', b'd', b'e', b'v', 0xFE, 0, 0];
        let message = from_type2_memory(&memory).unwrap();
        assert_eq!(decode(&message), [NdefRecord::Uri("https://aylis.dev".into())]);
        // Cortado no meio: o que deu para ler.
        assert!(decode(&message[..6]).is_empty());
        assert_eq!(id_text(&[0x04, 0xA2, 0x1B]), "04:A2:1B");
    }
}
