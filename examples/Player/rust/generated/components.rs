//! Os componentes dos forms com as propriedades tipadas e a struct de cada form, com os
//! componentes resolvidos ao abrir (`self.edtNome.set_text(ui, ...)`).
//! Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#![allow(non_snake_case, dead_code, unused_imports)]

use aylis::prelude::handles::{self, Handle};
use aylis::prelude::{ComponentId, PropValue, Ui};

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ButtonStyle {
    #[default]
    Default,
    Accent,
    Danger,
    Elevated,
    Fab,
    FabExtended,
    Flat,
    Ghost,
    Glass,
    GlassProminent,
    Glossy,
    Icon,
    IconFilled,
    IconOutlined,
    IconTonal,
    Info,
    Link,
    Negative,
    Outline,
    Pill,
    Positive,
    Push,
    Reveal,
    Round,
    Rounded,
    Secondary,
    Success,
    Text,
    Tonal,
    Unelevated,
    Warning,
}

impl aylis::prelude::PropEnum for ButtonStyle {
    const NAMES: &'static [&'static str] = &["", "accent", "danger", "elevated", "fab", "fab_extended", "flat", "ghost", "glass", "glass_prominent", "glossy", "icon", "icon_filled", "icon_outlined", "icon_tonal", "info", "link", "negative", "outline", "pill", "positive", "push", "reveal", "round", "rounded", "secondary", "success", "text", "tonal", "unelevated", "warning"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Accent, Self::Danger, Self::Elevated, Self::Fab, Self::FabExtended, Self::Flat, Self::Ghost, Self::Glass, Self::GlassProminent, Self::Glossy, Self::Icon, Self::IconFilled, Self::IconOutlined, Self::IconTonal, Self::Info, Self::Link, Self::Negative, Self::Outline, Self::Pill, Self::Positive, Self::Push, Self::Reveal, Self::Round, Self::Rounded, Self::Secondary, Self::Success, Self::Text, Self::Tonal, Self::Unelevated, Self::Warning].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for DividerStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FontWeight {
    #[default]
    Default,
    Thin,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    Black,
}

impl aylis::prelude::PropEnum for FontWeight {
    const NAMES: &'static [&'static str] = &["default", "thin", "light", "normal", "medium", "semibold", "bold", "black"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Thin, Self::Light, Self::Normal, Self::Medium, Self::Semibold, Self::Bold, Self::Black].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FormStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FormStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconStyle {
    #[default]
    Default,
    Muted,
}

impl aylis::prelude::PropEnum for IconStyle {
    const NAMES: &'static [&'static str] = &["", "muted"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Muted].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LabelStyle {
    #[default]
    Default,
    Caption,
    Error,
    Heading,
    Kbd,
    Link,
    Mono,
    Muted,
    Title,
}

impl aylis::prelude::PropEnum for LabelStyle {
    const NAMES: &'static [&'static str] = &["", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Caption, Self::Error, Self::Heading, Self::Kbd, Self::Link, Self::Mono, Self::Muted, Self::Title].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LayoutStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for LayoutStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ListViewStyle {
    #[default]
    Default,
    Flat,
}

impl aylis::prelude::PropEnum for ListViewStyle {
    const NAMES: &'static [&'static str] = &["", "flat"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Flat].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MainMenuStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for MainMenuStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PanelStyle {
    #[default]
    Default,
    Appbar,
    Card,
    Filled,
    Flat,
    Glass,
    Outlined,
    Overlay,
    Reveal,
    Sidebar,
}

impl aylis::prelude::PropEnum for PanelStyle {
    const NAMES: &'static [&'static str] = &["", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Appbar, Self::Card, Self::Filled, Self::Flat, Self::Glass, Self::Outlined, Self::Overlay, Self::Reveal, Self::Sidebar].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SeekBarStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for SeekBarStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeName {
    #[default]
    Default,
    AylisLight,
    AylisDark,
    Material3Light,
    Material3Dark,
    Material2Light,
    Material2Dark,
    Android7Light,
    Android7Dark,
    IOS26Light,
    IOS26Dark,
    IOS13Light,
    IOS13Dark,
    IOS7Light,
    WinUI3Light,
    WinUI3Dark,
    Windows11Light,
    Windows11Dark,
    Windows10Light,
    Windows10Dark,
    Win32,
    Windows7,
    WindowsPhoneDark,
    WindowsPhoneLight,
    Vuetify3Light,
    Vuetify3Dark,
    Bootstrap5Light,
    Bootstrap5Dark,
    ShadcnUiLight,
    ShadcnUiDark,
    AntDesign5Light,
    AntDesign5Dark,
    ChakraUILight,
    ChakraUIDark,
    Quasar2Light,
    Quasar2Dark,
}

impl aylis::prelude::PropEnum for ThemeName {
    const NAMES: &'static [&'static str] = &["", "Aylis Light", "Aylis Dark", "Material 3 Light", "Material 3 Dark", "Material 2 Light", "Material 2 Dark", "Android 7 Light", "Android 7 Dark", "iOS 26 Light", "iOS 26 Dark", "iOS 13 Light", "iOS 13 Dark", "iOS 7 Light", "WinUI 3 Light", "WinUI 3 Dark", "Windows 11 Light", "Windows 11 Dark", "Windows 10 Light", "Windows 10 Dark", "Win32", "Windows 7", "Windows Phone Dark", "Windows Phone Light", "Vuetify 3 Light", "Vuetify 3 Dark", "Bootstrap 5 Light", "Bootstrap 5 Dark", "shadcn/ui Light", "shadcn/ui Dark", "Ant Design 5 Light", "Ant Design 5 Dark", "Chakra UI Light", "Chakra UI Dark", "Quasar 2 Light", "Quasar 2 Dark"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::AylisLight, Self::AylisDark, Self::Material3Light, Self::Material3Dark, Self::Material2Light, Self::Material2Dark, Self::Android7Light, Self::Android7Dark, Self::IOS26Light, Self::IOS26Dark, Self::IOS13Light, Self::IOS13Dark, Self::IOS7Light, Self::WinUI3Light, Self::WinUI3Dark, Self::Windows11Light, Self::Windows11Dark, Self::Windows10Light, Self::Windows10Dark, Self::Win32, Self::Windows7, Self::WindowsPhoneDark, Self::WindowsPhoneLight, Self::Vuetify3Light, Self::Vuetify3Dark, Self::Bootstrap5Light, Self::Bootstrap5Dark, Self::ShadcnUiLight, Self::ShadcnUiDark, Self::AntDesign5Light, Self::AntDesign5Dark, Self::ChakraUILight, Self::ChakraUIDark, Self::Quasar2Light, Self::Quasar2Dark].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TrackBarStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for TrackBarStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum VideoPlayerStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for VideoPlayerStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct ListViewItem {
    pub title: String,
    pub detail: String,
}

impl ListViewItem {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            title: v.get("title").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            detail: v.get("detail").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("title".into(), aylis::json::serde_json::Value::from(self.title.clone()));
        m.insert("detail".into(), aylis::json::serde_json::Value::from(self.detail.clone()));
        aylis::json::serde_json::Value::Object(m)
    }

    /// A lista em JSON, como a tela grava.
    pub fn list_text(items: &[Self]) -> String {
        aylis::json::serde_json::Value::Array(items.iter().map(Self::to_json).collect()).to_string()
    }

    /// A lista do texto com o JSON (vazia se não é uma lista).
    pub fn list_from_text(text: &str) -> Vec<Self> {
        aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(text).ok().and_then(|j| j.as_array().map(|a| a.iter().map(Self::from_json).collect())).unwrap_or_default()
    }
}

impl Default for ListViewItem {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct OutputSpec {
    pub name: String,
    pub volume: f64,
    pub delay_ms: f64,
}

impl OutputSpec {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            name: v.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            volume: v.get("volume").and_then(|x| x.as_f64()).unwrap_or_default(),
            delay_ms: v.get("delay_ms").and_then(|x| x.as_f64()).unwrap_or_default(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("name".into(), aylis::json::serde_json::Value::from(self.name.clone()));
        m.insert("volume".into(), aylis::json::serde_json::Value::from(self.volume.clone()));
        m.insert("delay_ms".into(), aylis::json::serde_json::Value::from(self.delay_ms.clone()));
        aylis::json::serde_json::Value::Object(m)
    }

    /// A lista em JSON, como a tela grava.
    pub fn list_text(items: &[Self]) -> String {
        aylis::json::serde_json::Value::Array(items.iter().map(Self::to_json).collect()).to_string()
    }

    /// A lista do texto com o JSON (vazia se não é uma lista).
    pub fn list_from_text(text: &str) -> Vec<Self> {
        aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(text).ok().and_then(|j| j.as_array().map(|a| a.iter().map(Self::from_json).collect())).unwrap_or_default()
    }
}

impl Default for OutputSpec {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}
include!("components/classes/Button.rs");
include!("components/classes/Divider.rs");
include!("components/classes/Form.rs");
include!("components/classes/Icon.rs");
include!("components/classes/Label.rs");
include!("components/classes/Layout.rs");
include!("components/classes/ListView.rs");
include!("components/classes/MainMenu.rs");
include!("components/classes/MenuItem.rs");
include!("components/classes/OpenDialog.rs");
include!("components/classes/Panel.rs");
include!("components/classes/SeekBar.rs");
include!("components/classes/Timer.rs");
include!("components/classes/TrackBar.rs");
include!("components/classes/VideoPlayer.rs");
include!("components/forms/player.rs");
