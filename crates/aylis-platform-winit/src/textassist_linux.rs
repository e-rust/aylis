//! O motor de texto nativo do Linux: o corretor e as sugestões pelo Enchant 2 (o que o
//! GNOME e o LibreOffice usam, por cima do Hunspell, com o dicionário pessoal do usuário),
//! aberto em tempo de execução (`dlopen`: sem ele instalado, o app abre do mesmo jeito,
//! sem correção). O completar usa as palavras do dicionário Hunspell do idioma
//! (`/usr/share/hunspell/pt_BR.dic`), carregadas na primeira vez.
//!
//! Para ter: `sudo apt install libenchant-2-2 hunspell-pt-br` (e os outros idiomas).

use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char, c_int, c_void};

use aylis_core::textassist::{Misspelling, TextAssist, word_before, words};

type BrokerInit = unsafe extern "C" fn() -> *mut c_void;
type RequestDict = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
type DictCheck = unsafe extern "C" fn(*mut c_void, *const c_char, isize) -> c_int;
type DictSuggest = unsafe extern "C" fn(*mut c_void, *const c_char, isize, *mut usize) -> *mut *mut c_char;
type FreeList = unsafe extern "C" fn(*mut c_void, *mut *mut c_char);
type DictAdd = unsafe extern "C" fn(*mut c_void, *const c_char, isize);

/// As funções do Enchant e o "broker" (quem acha os dicionários).
struct Enchant {
    _lib: libloading::Library,
    broker: *mut c_void,
    request: RequestDict,
    check: DictCheck,
    suggest: DictSuggest,
    free_list: FreeList,
    add: Option<DictAdd>,
}

impl Enchant {
    fn open() -> Option<Enchant> {
        for name in ["libenchant-2.so.2", "libenchant-2.so", "libenchant.so.1"] {
            // SAFETY: a biblioteca do sistema; as funções são as da API pública do Enchant,
            // com as assinaturas dela.
            let Ok(lib) = (unsafe { libloading::Library::new(name) }) else { continue };
            let found = unsafe {
                let init: Option<BrokerInit> = lib.get::<BrokerInit>(b"enchant_broker_init\0").ok().map(|s| *s);
                let request = lib.get::<RequestDict>(b"enchant_broker_request_dict\0").ok().map(|s| *s);
                let check = lib.get::<DictCheck>(b"enchant_dict_check\0").ok().map(|s| *s);
                let suggest = lib.get::<DictSuggest>(b"enchant_dict_suggest\0").ok().map(|s| *s);
                let free_list = lib.get::<FreeList>(b"enchant_dict_free_string_list\0").ok().map(|s| *s);
                // O Enchant 2 tem `enchant_dict_add`; o 1, `enchant_dict_add_to_pwl`.
                let add = lib.get::<DictAdd>(b"enchant_dict_add\0").or_else(|_| lib.get::<DictAdd>(b"enchant_dict_add_to_pwl\0")).ok().map(|s| *s);
                (init, request, check, suggest, free_list, add)
            };
            if let (Some(init), Some(request), Some(check), Some(suggest), Some(free_list), add) = found {
                // SAFETY: cria o broker (vive enquanto o motor existir).
                let broker = unsafe { init() };
                if broker.is_null() {
                    continue;
                }
                return Some(Enchant { _lib: lib, broker, request, check, suggest, free_list, add });
            }
        }
        None
    }
}

/// Um dicionário aberto (ponteiro do Enchant).
#[derive(Clone, Copy)]
struct Dict(*mut c_void);

pub(crate) struct LinuxTextAssist {
    enchant: Option<Enchant>,
    dicts: HashMap<String, Option<Dict>>,
    /// As palavras do dicionário Hunspell, ordenadas em minúsculas (para completar).
    words: HashMap<String, Vec<String>>,
    default_language: String,
}

/// `pt-BR` → `pt_BR` (o nome dos dicionários).
fn tag(language: &str) -> String {
    language.replace('-', "_")
}

/// O idioma do sistema (`LANG=pt_BR.UTF-8` → `pt-BR`).
fn system_language() -> String {
    let lang = std::env::var("LC_ALL").or_else(|_| std::env::var("LC_MESSAGES")).or_else(|_| std::env::var("LANG")).unwrap_or_default();
    let code = lang.split(['.', '@']).next().unwrap_or("").replace('_', "-");
    if code.is_empty() || code == "C" || code == "POSIX" { "en-US".into() } else { code }
}

/// As palavras de um dicionário Hunspell (`.dic`: a quantidade na primeira linha, depois
/// `palavra/marcas`), ordenadas em minúsculas e sem repetir.
pub(crate) fn hunspell_words(text: &str) -> Vec<String> {
    let mut out: Vec<String> = text
        .lines()
        .skip(1)
        .filter_map(|l| {
            let w = l.split(['/', '\t']).next()?.trim();
            (w.chars().count() > 2 && w.chars().all(|c| c.is_alphabetic() || c == '-' || c == '\'')).then(|| w.to_lowercase())
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// As palavras de `sorted` que começam com `prefix` (já em minúsculas), as mais curtas
/// primeiro (as comuns costumam ser curtas).
pub(crate) fn completions(sorted: &[String], prefix: &str, limit: usize) -> Vec<String> {
    let start = sorted.partition_point(|w| w.as_str() < prefix);
    let mut found: Vec<&String> = sorted[start..].iter().take_while(|w| w.starts_with(prefix)).filter(|w| w.len() > prefix.len()).take(400).collect();
    found.sort_by_key(|w| (w.chars().count(), w.as_str()));
    found.into_iter().take(limit).cloned().collect()
}

/// Com a caixa do que foi digitado (`"Compu"` → `"Computador"`).
fn keep_case(typed: &str, word: &str) -> String {
    if typed.chars().count() > 1 && typed.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase) {
        return word.to_uppercase();
    }
    if typed.chars().next().is_some_and(char::is_uppercase) {
        let mut chars = word.chars();
        return chars.next().map(|f| f.to_uppercase().chain(chars).collect()).unwrap_or_default();
    }
    word.to_string()
}

impl LinuxTextAssist {
    pub(crate) fn new() -> Self {
        Self { enchant: Enchant::open(), dicts: HashMap::new(), words: HashMap::new(), default_language: system_language() }
    }

    /// Há o Enchant no sistema.
    #[cfg(test)]
    pub(crate) fn available(&self) -> bool {
        self.enchant.is_some()
    }

    fn language(&self, language: &str) -> String {
        if language.is_empty() { self.default_language.clone() } else { language.to_string() }
    }

    fn dict(&mut self, language: &str) -> Option<Dict> {
        let lang = self.language(language);
        if let Some(d) = self.dicts.get(&lang) {
            return *d;
        }
        let dict = self.enchant.as_ref().and_then(|e| {
            let name = CString::new(tag(&lang)).ok()?;
            // SAFETY: o broker é válido; o Enchant devolve nulo sem o dicionário.
            let d = unsafe { (e.request)(e.broker, name.as_ptr()) };
            (!d.is_null()).then_some(Dict(d))
        });
        self.dicts.insert(lang, dict);
        dict
    }

    fn word_list(&mut self, language: &str) -> &[String] {
        let lang = self.language(language);
        self.words.entry(lang.clone()).or_insert_with(|| {
            let name = tag(&lang);
            let short = name.split('_').next().unwrap_or("").to_string();
            ["/usr/share/hunspell", "/usr/share/myspell", "/usr/share/myspell/dicts"]
                .iter()
                .flat_map(|dir| [format!("{dir}/{name}.dic"), format!("{dir}/{short}.dic")])
                .find_map(|path| std::fs::read(path).ok())
                .map(|bytes| hunspell_words(&String::from_utf8_lossy(&bytes)))
                .unwrap_or_default()
        })
    }
}

impl TextAssist for LinuxTextAssist {
    fn check(&mut self, text: &str, language: &str) -> Vec<Misspelling> {
        let Some(dict) = self.dict(language) else { return Vec::new() };
        let Some(e) = &self.enchant else { return Vec::new() };
        words(text)
            .into_iter()
            .filter(|&(s, end)| {
                let word = &text[s..end];
                if word.chars().all(|c| !c.is_alphabetic()) {
                    return false;
                }
                let Ok(c) = CString::new(word) else { return false };
                // SAFETY: o dicionário é válido; o texto é UTF-8 com o tamanho em bytes.
                unsafe { (e.check)(dict.0, c.as_ptr(), word.len() as isize) != 0 }
            })
            .map(|(start, end)| Misspelling { start, end })
            .collect()
    }

    fn suggest(&mut self, word: &str, language: &str) -> Vec<String> {
        let Some(dict) = self.dict(language) else { return Vec::new() };
        let Some(e) = &self.enchant else { return Vec::new() };
        let Ok(c) = CString::new(word) else { return Vec::new() };
        let mut n = 0usize;
        // SAFETY: a lista volta do Enchant e é liberada por ele.
        unsafe {
            let list = (e.suggest)(dict.0, c.as_ptr(), word.len() as isize, &mut n);
            if list.is_null() {
                return Vec::new();
            }
            let out = (0..n).filter_map(|i| (*list.add(i)).as_ref().map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())).take(8).collect();
            (e.free_list)(dict.0, list);
            out
        }
    }

    fn complete(&mut self, before: &str, language: &str) -> Vec<String> {
        let prefix = word_before(before);
        if prefix.chars().count() < 2 {
            return Vec::new();
        }
        let lower = prefix.to_lowercase();
        completions(self.word_list(language), &lower, 5).into_iter().map(|w| keep_case(prefix, &w)).collect()
    }

    fn add_word(&mut self, word: &str, language: &str) {
        let Some(dict) = self.dict(language) else { return };
        let (Some(e), Ok(c)) = (&self.enchant, CString::new(word)) else { return };
        if let Some(add) = e.add {
            // SAFETY: grava no dicionário pessoal do usuário (o do Enchant).
            unsafe { add(dict.0, c.as_ptr(), word.len() as isize) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hunspell_words_are_read_and_completed() {
        let dic = "6\ncasa/S\ncasamento/S\ncasaco\ncasa/X\nab\n123\n";
        let words = hunspell_words(dic);
        assert_eq!(words, ["casa", "casaco", "casamento"]);
        assert_eq!(completions(&words, "cas", 5), ["casa", "casaco", "casamento"]);
        assert_eq!(keep_case("Cas", "casaco"), "Casaco");
        assert_eq!(tag("pt-BR"), "pt_BR");
    }

    /// No Linux com o Enchant e os dicionários (`apt install libenchant-2-2 hunspell-pt-br
    /// hunspell-en-us`): o corretor de verdade. Sem eles, só avisa.
    #[test]
    fn the_system_spell_checker() {
        let mut t = LinuxTextAssist::new();
        if !t.available() {
            aylis_core::stderr_line!("sem o Enchant: teste pulado");
            return;
        }
        let wrong = t.check("Eu gosto do meu cachorrro", "pt-BR");
        if t.dict("pt-BR").is_none() {
            aylis_core::stderr_line!("sem o dicionário pt-BR: teste pulado");
            return;
        }
        let words: Vec<&str> = wrong.iter().map(|m| &"Eu gosto do meu cachorrro"[m.start..m.end]).collect();
        assert_eq!(words, ["cachorrro"]);
        assert!(!words.contains(&"gosto"));
        let s = t.suggest("cachorrro", "pt-BR");
        assert!(s.iter().any(|w| w == "cachorro"), "{s:?}");
        let c = t.complete("vou para a compu", "pt-BR");
        assert!(c.iter().any(|w| w.starts_with("comput")), "{c:?}");
        aylis_core::stderr_line!("ok: erradas {words:?}, sugestões {s:?}, completar {c:?}");
    }
}
