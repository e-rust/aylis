//! Os componentes dos forms com as propriedades tipadas e a struct de cada form, com os
//! componentes resolvidos ao abrir (`self.edtNome.set_text(ui, ...)`).
//! Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#![allow(non_snake_case, dead_code, unused_imports)]

use aylis::prelude::handles::{self, Handle};
use aylis::prelude::{ComponentId, PropValue, Ui};

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
pub enum FpsMeterStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FpsMeterStyle {
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
pub enum GridLayoutStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for GridLayoutStyle {
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
pub enum Model3DStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for Model3DStyle {
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
pub enum SegmentedControlStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for SegmentedControlStyle {
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
include!("components/classes/Form.rs");
include!("components/classes/FpsMeter.rs");
include!("components/classes/GridLayout.rs");
include!("components/classes/Label.rs");
include!("components/classes/Model3D.rs");
include!("components/classes/Panel.rs");
include!("components/classes/SegmentedControl.rs");
include!("components/classes/Timer.rs");
include!("components/forms/principal.rs");
