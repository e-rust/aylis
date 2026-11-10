//! Os códigos de barras do próprio Aylis (o provedor padrão): Code 128, EAN-13, EAN-8, UPC-A,
//! Code 39, ITF (o intercalado 2 de 5 do boleto) e QR Code (pelo crate `qrcode`).
//!
//! Um formato novo é um provedor novo (outro componente com `BarcodeProvider`), sem mexer
//! aqui nem no `Barcode`.

use aylis_core::providers::{BarcodeImage, BarcodeOptions, BarcodeProvider, ErrorCorrection};

/// Os formatos que o provedor padrão gera.
pub const FORMATS: [&str; 7] = ["code128", "ean13", "ean8", "upca", "code39", "itf", "qr"];

/// O gerador padrão.
#[derive(Clone, Copy, Debug, Default)]
pub struct BuiltinBarcodes;

impl BarcodeProvider for BuiltinBarcodes {
    fn formats(&self) -> Vec<&'static str> {
        FORMATS.to_vec()
    }

    fn encode(&self, format: &str, data: &str, options: &BarcodeOptions) -> Result<BarcodeImage, String> {
        match format.to_ascii_lowercase().as_str() {
            "code128" => code128(data),
            "ean13" => ean13(data),
            "ean8" => ean8(data),
            "upca" => ean13(&format!("0{data}")).map(|image| match image {
                // O UPC-A é o EAN-13 com o 0 na frente (o texto, sem ele).
                BarcodeImage::Linear { modules, text } => BarcodeImage::Linear { modules, text: text[1..].to_string() },
                other => other,
            }),
            "code39" => code39(data),
            "itf" => itf(data),
            "qr" => qr(data, options.error_correction),
            other => Err(format!("formato desconhecido: {other} (o provedor sabe {})", FORMATS.join(", "))),
        }
    }
}

/// Larguras alternadas (barra, espaço, barra...) em módulos.
fn push_widths(modules: &mut Vec<bool>, widths: &str) {
    for (i, w) in widths.bytes().enumerate() {
        let bar = i % 2 == 0;
        modules.extend(std::iter::repeat_n(bar, usize::from(w - b'0')));
    }
}

/// Um padrão de módulos escrito com 1 (barra) e 0 (espaço).
fn push_bits(modules: &mut Vec<bool>, bits: &str) {
    modules.extend(bits.bytes().map(|b| b == b'1'));
}

// ---------------------------------------------------------------------------
// Code 128

/// As larguras dos 107 símbolos (barra, espaço, barra, espaço, barra, espaço); o 106 é a
/// parada, com a barra final.
const CODE128: [&str; 107] = [
    "212222", "222122", "222221", "121223", "121322", "131222", "122213", "122312", "132212", "221213", "221312", "231212", "112232", "122132", "122231", "113222",
    "123122", "123221", "223211", "221132", "221231", "213212", "223112", "312131", "311222", "321122", "321221", "312212", "322112", "322211", "212123", "212321",
    "232121", "111323", "131123", "131321", "112313", "132113", "132311", "211313", "231113", "231311", "112133", "112331", "132131", "113123", "113321", "133121",
    "313121", "211331", "231131", "213113", "213311", "213131", "311123", "311321", "331121", "312113", "312311", "332111", "314111", "221411", "431111", "111224",
    "111422", "121124", "121421", "141122", "141221", "112214", "112412", "122114", "122411", "142112", "142211", "241211", "221114", "413111", "241112", "134111",
    "111242", "121142", "121241", "114212", "124112", "124211", "411212", "421112", "421211", "212141", "214121", "412121", "111143", "111341", "131141", "114113",
    "114311", "411113", "411311", "113141", "114131", "311141", "411131", "211412", "211214", "211232", "2331112",
];
const START_B: usize = 104;
const START_C: usize = 105;
const CODE_B: usize = 100;
const CODE_C: usize = 99;

/// Code 128, trocando para o conjunto C (dois dígitos por símbolo) nos trechos de 4 ou mais
/// dígitos, e B (o ASCII imprimível) no resto.
fn code128(data: &str) -> Result<BarcodeImage, String> {
    if data.is_empty() {
        return Err("sem dado".into());
    }
    if let Some(c) = data.chars().find(|c| !(' '..='~').contains(c)) {
        return Err(format!("o Code 128 não tem o caractere '{c}'"));
    }
    let bytes = data.as_bytes();
    let digits_from = |i: usize| bytes[i..].iter().take_while(|b| b.is_ascii_digit()).count();
    let mut symbols = Vec::new();
    let mut set_c = digits_from(0) >= 4 && digits_from(0) % 2 == 0 || digits_from(0) == bytes.len() && bytes.len() >= 2 && bytes.len() % 2 == 0;
    symbols.push(if set_c { START_C } else { START_B });
    let mut i = 0;
    while i < bytes.len() {
        let run = digits_from(i);
        if set_c {
            if run >= 2 {
                symbols.push(usize::from((bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0')));
                i += 2;
                continue;
            }
            symbols.push(CODE_B);
            set_c = false;
        } else if run >= 4 && (run % 2 == 0 || i + run == bytes.len() && run >= 6) {
            // Um trecho par de dígitos (ou longo no fim): vale o conjunto C; o dígito ímpar
            // que sobra fica no B, antes.
            if run % 2 == 1 {
                symbols.push(usize::from(bytes[i] - b' '));
                i += 1;
            }
            symbols.push(CODE_C);
            set_c = true;
            continue;
        }
        symbols.push(usize::from(bytes[i] - b' '));
        i += 1;
    }
    let check = symbols.iter().enumerate().map(|(k, &s)| if k == 0 { s } else { k * s }).sum::<usize>() % 103;
    symbols.push(check);
    symbols.push(106);
    let mut modules = Vec::new();
    for s in symbols {
        push_widths(&mut modules, CODE128[s]);
    }
    Ok(BarcodeImage::Linear { modules, text: data.to_string() })
}

// ---------------------------------------------------------------------------
// EAN / UPC

const EAN_L: [&str; 10] = ["0001101", "0011001", "0010011", "0111101", "0100011", "0110001", "0101111", "0111011", "0110111", "0001011"];
const EAN_G: [&str; 10] = ["0100111", "0110011", "0011011", "0100001", "0011101", "0111001", "0000101", "0010001", "0001001", "0010111"];
const EAN_R: [&str; 10] = ["1110010", "1100110", "1101100", "1000010", "1011100", "1001110", "1010000", "1000100", "1001000", "1110100"];
/// O primeiro dígito do EAN-13 diz quais dos seis da esquerda usam o G.
const EAN_PARITY: [&str; 10] = ["LLLLLL", "LLGLGG", "LLGGLG", "LLGGGL", "LGLLGG", "LGGLLG", "LGGGLL", "LGLGLG", "LGLGGL", "LGGLGL"];

/// Os dígitos, com o verificador calculado (com `len - 1` dígitos) ou conferido (com `len`).
/// O peso 3 vai nos dígitos de posição ímpar contando da direita (antes do verificador).
fn with_check(data: &str, len: usize, name: &str) -> Result<Vec<u8>, String> {
    if !data.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("o {name} só tem dígitos"));
    }
    let mut digits: Vec<u8> = data.bytes().map(|b| b - b'0').collect();
    if digits.len() != len && digits.len() != len - 1 {
        return Err(format!("o {name} tem {} dígitos (ou {} sem o verificador)", len, len - 1));
    }
    let body = &digits[..len - 1];
    let sum: u32 = body.iter().rev().enumerate().map(|(i, &d)| u32::from(d) * if i % 2 == 0 { 3 } else { 1 }).sum();
    let check = ((10 - sum % 10) % 10) as u8;
    if digits.len() == len {
        if digits[len - 1] != check {
            return Err(format!("o dígito verificador do {name} deveria ser {check}"));
        }
    } else {
        digits.push(check);
    }
    Ok(digits)
}

fn ean13(data: &str) -> Result<BarcodeImage, String> {
    let d = with_check(data, 13, "EAN-13")?;
    let parity = EAN_PARITY[usize::from(d[0])].as_bytes();
    let mut modules = Vec::new();
    push_bits(&mut modules, "101");
    for (i, &digit) in d[1..7].iter().enumerate() {
        push_bits(&mut modules, if parity[i] == b'G' { EAN_G[usize::from(digit)] } else { EAN_L[usize::from(digit)] });
    }
    push_bits(&mut modules, "01010");
    for &digit in &d[7..] {
        push_bits(&mut modules, EAN_R[usize::from(digit)]);
    }
    push_bits(&mut modules, "101");
    Ok(BarcodeImage::Linear { modules, text: d.iter().map(|x| char::from(b'0' + x)).collect() })
}

fn ean8(data: &str) -> Result<BarcodeImage, String> {
    let d = with_check(data, 8, "EAN-8")?;
    let mut modules = Vec::new();
    push_bits(&mut modules, "101");
    for &digit in &d[..4] {
        push_bits(&mut modules, EAN_L[usize::from(digit)]);
    }
    push_bits(&mut modules, "01010");
    for &digit in &d[4..] {
        push_bits(&mut modules, EAN_R[usize::from(digit)]);
    }
    push_bits(&mut modules, "101");
    Ok(BarcodeImage::Linear { modules, text: d.iter().map(|x| char::from(b'0' + x)).collect() })
}

// ---------------------------------------------------------------------------
// Code 39 e ITF (barras estreitas e largas: a larga vale 3 módulos)

const CODE39: [(char, &str); 44] = [
    ('0', "nnnwwnwnn"), ('1', "wnnwnnnnw"), ('2', "nnwwnnnnw"), ('3', "wnwwnnnnn"), ('4', "nnnwwnnnw"), ('5', "wnnwwnnnn"), ('6', "nnwwwnnnn"), ('7', "nnnwnnwnw"),
    ('8', "wnnwnnwnn"), ('9', "nnwwnnwnn"), ('A', "wnnnnwnnw"), ('B', "nnwnnwnnw"), ('C', "wnwnnwnnn"), ('D', "nnnnwwnnw"), ('E', "wnnnwwnnn"), ('F', "nnwnwwnnn"),
    ('G', "nnnnnwwnw"), ('H', "wnnnnwwnn"), ('I', "nnwnnwwnn"), ('J', "nnnnwwwnn"), ('K', "wnnnnnnww"), ('L', "nnwnnnnww"), ('M', "wnwnnnnwn"), ('N', "nnnnwnnww"),
    ('O', "wnnnwnnwn"), ('P', "nnwnwnnwn"), ('Q', "nnnnnnwww"), ('R', "wnnnnnwwn"), ('S', "nnwnnnwwn"), ('T', "nnnnwnwwn"), ('U', "wwnnnnnnw"), ('V', "nwwnnnnnw"),
    ('W', "wwwnnnnnn"), ('X', "nwnnwnnnw"), ('Y', "wwnnwnnnn"), ('Z', "nwwnwnnnn"), ('-', "nwnnnnwnw"), ('.', "wwnnnnwnn"), (' ', "nwwnnnwnn"), ('$', "nwnwnwnnn"),
    ('/', "nwnwnnnwn"), ('+', "nwnnnwnwn"), ('%', "nnnwnwnwn"), ('*', "nwnnwnwnn"),
];

/// Barras e espaços alternados, estreitos (`n`, 1 módulo) e largos (`w`, 3).
fn push_wide(modules: &mut Vec<bool>, pattern: &str, first_bar: bool) {
    for (i, c) in pattern.bytes().enumerate() {
        let bar = (i % 2 == 0) == first_bar;
        modules.extend(std::iter::repeat_n(bar, if c == b'w' { 3 } else { 1 }));
    }
}

fn code39(data: &str) -> Result<BarcodeImage, String> {
    let text = data.to_uppercase();
    if text.is_empty() {
        return Err("sem dado".into());
    }
    if text.contains('*') {
        return Err("o * é o início e o fim do Code 39".into());
    }
    let pattern = |c: char| CODE39.iter().find(|(k, _)| *k == c).map(|(_, p)| *p);
    let mut modules = Vec::new();
    for c in std::iter::once('*').chain(text.chars()).chain(std::iter::once('*')) {
        let p = pattern(c).ok_or_else(|| format!("o Code 39 não tem o caractere '{c}'"))?;
        if !modules.is_empty() {
            modules.push(false); // o espaço entre os caracteres
        }
        push_wide(&mut modules, p, true);
    }
    Ok(BarcodeImage::Linear { modules, text })
}

const ITF: [&str; 10] = ["nnwwn", "wnnnw", "nwnnw", "wwnnn", "nnwnw", "wnwnn", "nwwnn", "nnnww", "wnnwn", "nwnwn"];

/// Intercalado 2 de 5: os dígitos aos pares (o primeiro nas barras, o segundo nos espaços),
/// com um 0 na frente se forem ímpares. É o do boleto bancário (44 dígitos).
fn itf(data: &str) -> Result<BarcodeImage, String> {
    if data.is_empty() || !data.bytes().all(|b| b.is_ascii_digit()) {
        return Err("o ITF só tem dígitos".into());
    }
    let text = if data.len() % 2 == 1 { format!("0{data}") } else { data.to_string() };
    let d: Vec<usize> = text.bytes().map(|b| usize::from(b - b'0')).collect();
    let mut modules = Vec::new();
    push_wide(&mut modules, "nnnn", true);
    for pair in d.chunks(2) {
        let (bars, spaces) = (ITF[pair[0]].as_bytes(), ITF[pair[1]].as_bytes());
        for k in 0..5 {
            modules.extend(std::iter::repeat_n(true, if bars[k] == b'w' { 3 } else { 1 }));
            modules.extend(std::iter::repeat_n(false, if spaces[k] == b'w' { 3 } else { 1 }));
        }
    }
    push_wide(&mut modules, "wnn", true);
    Ok(BarcodeImage::Linear { modules, text })
}

// ---------------------------------------------------------------------------
// QR Code

fn qr(data: &str, correction: ErrorCorrection) -> Result<BarcodeImage, String> {
    if data.is_empty() {
        return Err("sem dado".into());
    }
    let level = match correction {
        ErrorCorrection::Low => qrcode::EcLevel::L,
        ErrorCorrection::Medium => qrcode::EcLevel::M,
        ErrorCorrection::Quartile => qrcode::EcLevel::Q,
        ErrorCorrection::High => qrcode::EcLevel::H,
    };
    let code = qrcode::QrCode::with_error_correction_level(data.as_bytes(), level).map_err(|e| format!("QR Code: {e}"))?;
    let width = code.width();
    let modules = code.to_colors().into_iter().map(|c| c == qrcode::Color::Dark).collect();
    Ok(BarcodeImage::Matrix { width, modules })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear(format: &str, data: &str) -> (Vec<bool>, String) {
        match BuiltinBarcodes.encode(format, data, &BarcodeOptions::default()).unwrap() {
            BarcodeImage::Linear { modules, text } => (modules, text),
            other => panic!("{other:?}"),
        }
    }

    fn bits(modules: &[bool]) -> String {
        modules.iter().map(|&b| if b { '1' } else { '0' }).collect()
    }

    #[test]
    fn the_tables_have_the_right_widths() {
        for (i, p) in CODE128.iter().enumerate() {
            let sum: u32 = p.bytes().map(|b| u32::from(b - b'0')).sum();
            assert_eq!(sum, if i == 106 { 13 } else { 11 }, "Code 128 símbolo {i}");
        }
        for (c, p) in CODE39 {
            assert_eq!(p.matches('w').count(), 3, "Code 39 '{c}': três largos");
        }
        for p in ITF {
            assert_eq!(p.matches('w').count(), 2, "ITF: dois largos em cinco");
        }
        for t in [EAN_L, EAN_G, EAN_R] {
            assert!(t.iter().all(|p| p.len() == 7));
        }
    }

    #[test]
    fn ean13_matches_a_known_code() {
        // 5901234123457: o exemplo clássico do EAN-13 (o 7 é o verificador).
        let (m, text) = linear("ean13", "590123412345");
        assert_eq!(text, "5901234123457");
        assert_eq!(m.len(), 95);
        // O 5 escolhe L G G L L G para os seis da esquerda (9 0 1 2 3 4); à direita, o R.
        let expected = ["101", "0001011", "0100111", "0110011", "0010011", "0111101", "0011101", "01010", "1100110", "1101100", "1000010", "1011100", "1001110", "1000100", "101"].concat();
        assert_eq!(bits(&m), expected);
        assert!(BuiltinBarcodes.encode("ean13", "5901234123458", &BarcodeOptions::default()).is_err(), "verificador errado");
        let (_, upc) = linear("upca", "03600029145");
        assert_eq!(upc, "036000291452", "UPC-A: o verificador do 036000291452");
        assert_eq!(linear("ean8", "9638507").1, "96385074");
    }

    #[test]
    fn code128_uses_set_c_for_digit_runs_and_ends_with_the_stop() {
        let (m, _) = linear("code128", "123456");
        // Início C + 3 pares + verificador + parada: 5 × 11 + 13 módulos.
        assert_eq!(m.len(), 5 * 11 + 13);
        assert_eq!(&bits(&m)[..11], "11010011100", "o início C");
        assert!(bits(&m).ends_with("1100011101011"), "a parada");
        let (m, _) = linear("code128", "Aylis-128");
        assert_eq!(&bits(&m)[..11], "11010010000", "o início B");
        assert!(BuiltinBarcodes.encode("code128", "ação", &BarcodeOptions::default()).is_err());
    }

    #[test]
    fn itf_and_code39_and_qr() {
        let (m, text) = linear("itf", "12345");
        assert_eq!(text, "012345", "ímpar: um 0 na frente");
        assert_eq!(&bits(&m)[..4], "1010", "o início");
        let (m, text) = linear("code39", "aylis");
        assert_eq!(text, "AYLIS");
        assert!(m.first() == Some(&true) && m.last() == Some(&true));
        match BuiltinBarcodes.encode("qr", "https://aylis.dev", &BarcodeOptions::default()).unwrap() {
            BarcodeImage::Matrix { width, modules } => {
                assert_eq!(modules.len(), width * width);
                assert!(width >= 21);
                // Os três quadrados de posição: o canto de cima à esquerda é escuro.
                assert!(modules[0] && modules[6] && modules[6 * width]);
            }
            other => panic!("{other:?}"),
        }
        assert!(BuiltinBarcodes.encode("pdf417", "x", &BarcodeOptions::default()).is_err(), "formato que o padrão não sabe");
    }

    #[test]
    fn the_qr_error_correction_level_reaches_the_code() {
        let qr = |level: ErrorCorrection, data: &str| match BuiltinBarcodes.encode("qr", data, &BarcodeOptions { error_correction: level }).unwrap() {
            BarcodeImage::Matrix { width, modules } => (width, modules),
            other => panic!("{other:?}"),
        };
        // Mais correção, mais módulos para o mesmo dado.
        let data = "https://aylis.dev/docs/componentes/Barcode?registro=000123456789";
        let widths: Vec<usize> = [ErrorCorrection::Low, ErrorCorrection::Medium, ErrorCorrection::Quartile, ErrorCorrection::High].into_iter().map(|l| qr(l, data).0).collect();
        assert!(widths.windows(2).all(|w| w[0] <= w[1]), "{widths:?}");
        assert!(widths[0] < widths[3], "L menor que H: {widths:?}");
        // No mesmo tamanho, o nível vai na informação de formato: outro desenho.
        let (wl, l) = qr(ErrorCorrection::Low, "aylis");
        let (wh, h) = qr(ErrorCorrection::High, "aylis");
        assert_eq!(wl, wh, "dado curto: a versão 1 nos dois");
        assert_ne!(l, h);
        // O padrão continua o M.
        assert_eq!(qr(ErrorCorrection::Medium, data), qr(BarcodeOptions::default().error_correction, data));
    }
}
