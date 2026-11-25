//! A tecla física: a posição dela no teclado, independente do layout (ABNT2, US, AZERTY).
//!
//! O [`Key`](crate::Key) diz o que a tecla escreve (o layout do sistema já aplicado); o
//! [`KeyCode`] diz qual tecla foi apertada. Quem repassa o teclado a outra máquina (acesso
//! remoto, KVM, emulador) ou usa a posição (WASD num jogo) precisa do físico: o caractere não
//! diz a tecla (o `/` do ABNT2 fica ao lado do Shift direito, que o teclado americano não tem).
//!
//! Os nomes são os do `code` do DOM (W3C UI Events), os mesmos em todas as plataformas.

/// A tecla física, pela posição num teclado americano (o `KeyA` do AZERTY escreve `q`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyCode {
    /// Sem tecla física: o evento sintético (protocolo, roteiro, MCP) ou a tecla que o sistema
    /// não identifica.
    #[default]
    Unidentified,
    /// `KeyA`..`KeyZ` (a letra em maiúscula).
    Letter(char),
    /// `Digit0`..`Digit9`, na fila de cima.
    Digit(u8),
    /// `Numpad0`..`Numpad9`.
    Numpad(u8),
    /// `F1`..`F35`.
    F(u8),
    /// `Lang1`..`Lang5` (teclados coreanos e japoneses).
    Lang(u8),
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Equal,
    /// A tecla ao lado do Shift esquerdo nos teclados ISO (o `\` do ABNT2).
    IntlBackslash,
    /// A tecla ao lado do Shift direito no ABNT2 (`/` e `?`) e no japonês.
    IntlRo,
    IntlYen,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    /// A tecla Windows / Command.
    MetaLeft,
    MetaRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    NumpadAdd,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadMultiply,
    NumpadSubtract,
    Escape,
    Fn,
    PrintScreen,
    ScrollLock,
    Pause,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    Power,
    Sleep,
    WakeUp,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    FnLock,
    Hiragana,
    Katakana,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Resume,
    Suspend,
}

/// As teclas com nome fixo (as de número e letra são montadas em [`KeyCode::name`]).
const NAMED: &[(KeyCode, &str)] = {
    use KeyCode::*;
    &[
        (Backquote, "Backquote"),
        (Backslash, "Backslash"),
        (BracketLeft, "BracketLeft"),
        (BracketRight, "BracketRight"),
        (Comma, "Comma"),
        (Equal, "Equal"),
        (IntlBackslash, "IntlBackslash"),
        (IntlRo, "IntlRo"),
        (IntlYen, "IntlYen"),
        (Minus, "Minus"),
        (Period, "Period"),
        (Quote, "Quote"),
        (Semicolon, "Semicolon"),
        (Slash, "Slash"),
        (AltLeft, "AltLeft"),
        (AltRight, "AltRight"),
        (Backspace, "Backspace"),
        (CapsLock, "CapsLock"),
        (ContextMenu, "ContextMenu"),
        (ControlLeft, "ControlLeft"),
        (ControlRight, "ControlRight"),
        (Enter, "Enter"),
        (MetaLeft, "MetaLeft"),
        (MetaRight, "MetaRight"),
        (ShiftLeft, "ShiftLeft"),
        (ShiftRight, "ShiftRight"),
        (Space, "Space"),
        (Tab, "Tab"),
        (Convert, "Convert"),
        (KanaMode, "KanaMode"),
        (NonConvert, "NonConvert"),
        (Delete, "Delete"),
        (End, "End"),
        (Help, "Help"),
        (Home, "Home"),
        (Insert, "Insert"),
        (PageDown, "PageDown"),
        (PageUp, "PageUp"),
        (ArrowDown, "ArrowDown"),
        (ArrowLeft, "ArrowLeft"),
        (ArrowRight, "ArrowRight"),
        (ArrowUp, "ArrowUp"),
        (NumLock, "NumLock"),
        (NumpadAdd, "NumpadAdd"),
        (NumpadComma, "NumpadComma"),
        (NumpadDecimal, "NumpadDecimal"),
        (NumpadDivide, "NumpadDivide"),
        (NumpadEnter, "NumpadEnter"),
        (NumpadEqual, "NumpadEqual"),
        (NumpadMultiply, "NumpadMultiply"),
        (NumpadSubtract, "NumpadSubtract"),
        (Escape, "Escape"),
        (Fn, "Fn"),
        (PrintScreen, "PrintScreen"),
        (ScrollLock, "ScrollLock"),
        (Pause, "Pause"),
        (AudioVolumeDown, "AudioVolumeDown"),
        (AudioVolumeMute, "AudioVolumeMute"),
        (AudioVolumeUp, "AudioVolumeUp"),
        (MediaPlayPause, "MediaPlayPause"),
        (MediaSelect, "MediaSelect"),
        (MediaStop, "MediaStop"),
        (MediaTrackNext, "MediaTrackNext"),
        (MediaTrackPrevious, "MediaTrackPrevious"),
        (BrowserBack, "BrowserBack"),
        (BrowserFavorites, "BrowserFavorites"),
        (BrowserForward, "BrowserForward"),
        (BrowserHome, "BrowserHome"),
        (BrowserRefresh, "BrowserRefresh"),
        (BrowserSearch, "BrowserSearch"),
        (BrowserStop, "BrowserStop"),
        (LaunchApp1, "LaunchApp1"),
        (LaunchApp2, "LaunchApp2"),
        (LaunchMail, "LaunchMail"),
        (Power, "Power"),
        (Sleep, "Sleep"),
        (WakeUp, "WakeUp"),
        (NumpadBackspace, "NumpadBackspace"),
        (NumpadClear, "NumpadClear"),
        (NumpadClearEntry, "NumpadClearEntry"),
        (NumpadHash, "NumpadHash"),
        (NumpadMemoryAdd, "NumpadMemoryAdd"),
        (NumpadMemoryClear, "NumpadMemoryClear"),
        (NumpadMemoryRecall, "NumpadMemoryRecall"),
        (NumpadMemoryStore, "NumpadMemoryStore"),
        (NumpadMemorySubtract, "NumpadMemorySubtract"),
        (NumpadParenLeft, "NumpadParenLeft"),
        (NumpadParenRight, "NumpadParenRight"),
        (NumpadStar, "NumpadStar"),
        (FnLock, "FnLock"),
        (Hiragana, "Hiragana"),
        (Katakana, "Katakana"),
        (Again, "Again"),
        (Copy, "Copy"),
        (Cut, "Cut"),
        (Find, "Find"),
        (Open, "Open"),
        (Paste, "Paste"),
        (Props, "Props"),
        (Select, "Select"),
        (Undo, "Undo"),
        (Resume, "Resume"),
        (Suspend, "Suspend"),
    ]
};

impl KeyCode {
    /// Todas as teclas, `Unidentified` primeiro (o enum das outras linguagens, na mesma ordem).
    pub fn all() -> Vec<KeyCode> {
        let mut all = vec![KeyCode::Unidentified];
        all.extend(('A'..='Z').map(KeyCode::Letter));
        all.extend((0..=9).map(KeyCode::Digit));
        all.extend((0..=9).map(KeyCode::Numpad));
        all.extend((1..=35).map(KeyCode::F));
        all.extend((1..=5).map(KeyCode::Lang));
        all.extend(NAMED.iter().map(|(k, _)| *k));
        all
    }

    /// O nome do `code` do DOM (`KeyA`, `Digit1`, `IntlRo`); `Unidentified` sem tecla física.
    pub fn name(self) -> String {
        match self {
            KeyCode::Unidentified => "Unidentified".into(),
            KeyCode::Letter(c) => format!("Key{}", c.to_ascii_uppercase()),
            KeyCode::Digit(n) => format!("Digit{n}"),
            KeyCode::Numpad(n) => format!("Numpad{n}"),
            KeyCode::F(n) => format!("F{n}"),
            KeyCode::Lang(n) => format!("Lang{n}"),
            named => NAMED.iter().find(|(k, _)| *k == named).map_or("Unidentified", |(_, n)| n).into(),
        }
    }

    /// O contrário de [`name`](Self::name). Aceita também os nomes do winit para a tecla
    /// Windows (`SuperLeft`, `SuperRight`).
    pub fn from_name(name: &str) -> Option<KeyCode> {
        let number = |prefix: &str, max: u8| name.strip_prefix(prefix).and_then(|n| n.parse::<u8>().ok()).filter(|n| *n <= max);
        if let Some(c) = name.strip_prefix("Key").filter(|r| r.len() == 1).and_then(|r| r.chars().next()).filter(char::is_ascii_alphabetic) {
            return Some(KeyCode::Letter(c.to_ascii_uppercase()));
        }
        if let Some(n) = number("Digit", 9) {
            return Some(KeyCode::Digit(n));
        }
        if let Some(n) = number("Numpad", 9).filter(|_| name.len() == 7) {
            return Some(KeyCode::Numpad(n));
        }
        if let Some(n) = number("F", 35).filter(|n| *n >= 1) {
            return Some(KeyCode::F(n));
        }
        if let Some(n) = number("Lang", 5).filter(|n| *n >= 1) {
            return Some(KeyCode::Lang(n));
        }
        match name {
            "Unidentified" => Some(KeyCode::Unidentified),
            "SuperLeft" => Some(KeyCode::MetaLeft),
            "SuperRight" => Some(KeyCode::MetaRight),
            _ => NAMED.iter().find(|(_, n)| *n == name).map(|(k, _)| *k),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_go_and_come_back() {
        for code in KeyCode::all() {
            assert_eq!(KeyCode::from_name(&code.name()), Some(code), "{}", code.name());
        }
        assert_eq!(KeyCode::from_name("SuperLeft"), Some(KeyCode::MetaLeft));
        assert_eq!(KeyCode::from_name("NumpadAdd"), Some(KeyCode::NumpadAdd));
        assert_eq!(KeyCode::from_name("F0"), None);
        assert_eq!(KeyCode::from_name("KeyAB"), None);
    }
}
