//! O motor de texto nativo do Windows: o corretor ortográfico do sistema (`ISpellChecker`,
//! o mesmo do Word e do Edge, com o dicionário do usuário) e a previsão de texto
//! (`TextPredictionGenerator`, a das sugestões do teclado virtual). Um de cada por idioma,
//! criados na primeira vez; sem o idioma instalado, aquele recurso fica sem resposta.

use std::collections::HashMap;

use aylis_core::textassist::{Misspelling, TextAssist, word_before};
use windows::Data::Text::TextPredictionGenerator;
use windows::Win32::Globalization::{GetUserDefaultLocaleName, ISpellChecker, ISpellCheckerFactory, SpellCheckerFactory};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree};
use windows::core::{HSTRING, PWSTR};

#[derive(Default)]
pub(crate) struct WindowsTextAssist {
    factory: Option<ISpellCheckerFactory>,
    checkers: HashMap<String, Option<ISpellChecker>>,
    predictors: HashMap<String, Option<TextPredictionGenerator>>,
    default_language: String,
}

impl WindowsTextAssist {
    pub(crate) fn new() -> Self {
        // SAFETY: inicializa o COM nesta thread (a da interface); se já estava, não muda nada.
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        }
        // SAFETY: cria a fábrica do corretor do sistema (in-process).
        let factory = unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_INPROC_SERVER).ok() };
        Self { factory, default_language: user_language(), ..Default::default() }
    }

    fn language<'a>(&'a self, language: &'a str) -> &'a str {
        if language.is_empty() { &self.default_language } else { language }
    }

    fn checker(&mut self, language: &str) -> Option<ISpellChecker> {
        let lang = self.language(language).to_string();
        if let Some(found) = self.checkers.get(&lang) {
            return found.clone();
        }
        let checker = self.factory.as_ref().and_then(|f| {
            let tag = HSTRING::from(&lang);
            // SAFETY: chamadas COM da fábrica, com a marca do idioma.
            let supported = unsafe { f.IsSupported(&tag) }.is_ok_and(|b| b.as_bool());
            supported.then(|| unsafe { f.CreateSpellChecker(&tag) }.ok()).flatten()
        });
        self.checkers.insert(lang, checker.clone());
        checker
    }

    fn predictor(&mut self, language: &str) -> Option<TextPredictionGenerator> {
        let lang = self.language(language).to_string();
        if let Some(found) = self.predictors.get(&lang) {
            return found.clone();
        }
        let generator = TextPredictionGenerator::Create(&HSTRING::from(&lang))
            .ok()
            .filter(|g| !g.LanguageAvailableButNotInstalled().unwrap_or(true));
        self.predictors.insert(lang, generator.clone());
        generator
    }
}

impl TextAssist for WindowsTextAssist {
    fn check(&mut self, text: &str, language: &str) -> Vec<Misspelling> {
        let Some(checker) = self.checker(language) else { return Vec::new() };
        // O Windows conta em unidades UTF-16: a tabela leva para bytes do texto.
        let mut utf16_to_byte = Vec::with_capacity(text.len() + 1);
        for (byte, c) in text.char_indices() {
            for _ in 0..c.len_utf16() {
                utf16_to_byte.push(byte);
            }
        }
        utf16_to_byte.push(text.len());
        let mut out = Vec::new();
        // SAFETY: Check devolve um enumerador COM; cada erro é lido e solto pelo windows-rs.
        let Ok(errors) = (unsafe { checker.Check(&HSTRING::from(text)) }) else { return out };
        loop {
            let mut error = None;
            // SAFETY: Next preenche `error` com o próximo (ou S_FALSE no fim).
            let hr = unsafe { errors.Next(&mut error) };
            let Some(error) = error.filter(|_| hr.is_ok()) else { break };
            // SAFETY: leitura das propriedades do erro.
            let (start, len) = unsafe { (error.StartIndex().unwrap_or(0) as usize, error.Length().unwrap_or(0) as usize) };
            if let (Some(&s), Some(&e)) = (utf16_to_byte.get(start), utf16_to_byte.get(start + len)) {
                if s < e {
                    out.push(Misspelling { start: s, end: e });
                }
            }
        }
        out
    }

    fn suggest(&mut self, word: &str, language: &str) -> Vec<String> {
        let Some(checker) = self.checker(language) else { return Vec::new() };
        let mut out = Vec::new();
        // SAFETY: Suggest devolve um IEnumString; cada PWSTR é liberado com CoTaskMemFree.
        unsafe {
            let Ok(list) = checker.Suggest(&HSTRING::from(word)) else { return out };
            loop {
                let mut item = [PWSTR::null()];
                let mut fetched = 0u32;
                if list.Next(&mut item, Some(&mut fetched)).is_err() || fetched == 0 || item[0].is_null() {
                    break;
                }
                if let Ok(s) = item[0].to_string() {
                    out.push(s);
                }
                CoTaskMemFree(Some(item[0].0 as *const _));
                if out.len() >= 8 {
                    break;
                }
            }
        }
        out
    }

    fn complete(&mut self, before: &str, language: &str) -> Vec<String> {
        let typed = word_before(before);
        if typed.chars().count() < 2 {
            return Vec::new();
        }
        let Some(generator) = self.predictor(language) else { return Vec::new() };
        let Ok(operation) = generator.GetCandidatesAsync(&HSTRING::from(typed)) else { return Vec::new() };
        // A previsão é rápida (local): espera o resultado aqui mesmo.
        let Ok(candidates) = operation.join() else { return Vec::new() };
        let size = candidates.Size().unwrap_or(0);
        (0..size).filter_map(|i| candidates.GetAt(i).ok()).map(|h| h.to_string_lossy()).filter(|w| !w.is_empty()).take(5).collect()
    }

    fn add_word(&mut self, word: &str, language: &str) {
        if let Some(checker) = self.checker(language) {
            // SAFETY: Add grava a palavra no dicionário do usuário (vale para todos os apps).
            let _ = unsafe { checker.Add(&HSTRING::from(word)) };
        }
    }
}

/// A marca do idioma do usuário no Windows (`pt-BR`).
fn user_language() -> String {
    let mut buffer = [0u16; 85];
    // SAFETY: o buffer tem o tamanho máximo de um nome de localidade.
    let n = unsafe { GetUserDefaultLocaleName(&mut buffer) };
    if n > 1 { String::from_utf16_lossy(&buffer[..n as usize - 1]) } else { "en-US".into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_windows_engine_checks_suggests_and_completes() {
        let mut engine = WindowsTextAssist::new();
        let lang = "pt-BR";
        if engine.checker(lang).is_none() {
            aylis_core::stderr_line!("corretor do Windows sem pt-BR: teste pulado");
            return;
        }
        let text = "eu gosto do meu cachoro";
        let errors = engine.check(text, lang);
        let words: Vec<&str> = errors.iter().map(|m| &text[m.start..m.end]).collect();
        assert_eq!(words, ["cachoro"], "só a palavra errada");
        let suggestions = engine.suggest("cachoro", lang);
        assert!(suggestions.iter().any(|s| s == "cachorro"), "{suggestions:?}");
        // Acentos: os trechos são de bytes (o Windows conta em UTF-16).
        let text = "não está cansaço errdo";
        let errors = engine.check(text, lang);
        assert_eq!(errors.iter().map(|m| &text[m.start..m.end]).collect::<Vec<_>>(), ["errdo"]);
        let completions = engine.complete("eu uso o compu", lang);
        aylis_core::stderr_line!("previsão do Windows para \"compu\": {completions:?}");
    }
}
