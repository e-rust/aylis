//! Os componentes dos forms com as propriedades tipadas e a struct de cada form, com os
//! componentes resolvidos ao abrir (`self.edtNome.set_text(ui, ...)`).
//! Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#![allow(non_snake_case, dead_code, unused_imports)]

use aylis::prelude::handles::{self, Handle};
use aylis::prelude::{ComponentId, PropValue, Ui};

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ChartSeriesFill {
    #[default]
    None,
    Origin,
    Start,
    End,
    Series,
}

impl aylis::prelude::PropEnum for ChartSeriesFill {
    const NAMES: &'static [&'static str] = &["none", "origin", "start", "end", "series"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Origin, Self::Start, Self::End, Self::Series].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ChartSeriesKind {
    #[default]
    Line,
    Bar,
    Area,
    Scatter,
}

impl aylis::prelude::PropEnum for ChartSeriesKind {
    const NAMES: &'static [&'static str] = &["line", "bar", "area", "scatter"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Line, Self::Bar, Self::Area, Self::Scatter].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ChartSeriesPoint {
    #[default]
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    None,
}

impl aylis::prelude::PropEnum for ChartSeriesPoint {
    const NAMES: &'static [&'static str] = &["circle", "square", "triangle", "diamond", "cross", "none"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Circle, Self::Square, Self::Triangle, Self::Diamond, Self::Cross, Self::None].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DBChartStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for DBChartStyle {
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
pub enum DataGridStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for DataGridStyle {
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
pub enum GridTotalFunction {
    #[default]
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

impl aylis::prelude::PropEnum for GridTotalFunction {
    const NAMES: &'static [&'static str] = &["sum", "avg", "min", "max", "count"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Sum, Self::Avg, Self::Min, Self::Max, Self::Count].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum GroupBoxStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for GroupBoxStyle {
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
pub enum ImageStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for ImageStyle {
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
pub enum SplitterStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for SplitterStyle {
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
pub enum SvgStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for SvgStyle {
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

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub x: Vec<f64>,
    pub radius: Vec<f64>,
    pub kind: ChartSeriesKind,
    pub right_axis: bool,
    pub color: aylis::prelude::Color,
    pub width: f64,
    pub dash: Vec<f64>,
    pub point: ChartSeriesPoint,
    pub fill: ChartSeriesFill,
    pub fill_series: i64,
    pub hidden: bool,
}

impl ChartSeries {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            name: v.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            values: v.get("values").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default(),
            x: v.get("x").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default(),
            radius: v.get("radius").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default(),
            kind: v.get("kind").and_then(|x| x.as_str()).and_then(<ChartSeriesKind as aylis::prelude::PropEnum>::from_name).or_else(|| <ChartSeriesKind as aylis::prelude::PropEnum>::from_index(0)).expect("enum"),
            right_axis: v.get("right_axis").and_then(|x| x.as_bool()).unwrap_or_default(),
            color: v.get("color").and_then(|x| x.as_str()).and_then(aylis::prelude::Color::from_hex).unwrap_or_default(),
            width: v.get("width").and_then(|x| x.as_f64()).unwrap_or_default(),
            dash: v.get("dash").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default(),
            point: v.get("point").and_then(|x| x.as_str()).and_then(<ChartSeriesPoint as aylis::prelude::PropEnum>::from_name).or_else(|| <ChartSeriesPoint as aylis::prelude::PropEnum>::from_index(0)).expect("enum"),
            fill: v.get("fill").and_then(|x| x.as_str()).and_then(<ChartSeriesFill as aylis::prelude::PropEnum>::from_name).or_else(|| <ChartSeriesFill as aylis::prelude::PropEnum>::from_index(0)).expect("enum"),
            fill_series: v.get("fill_series").and_then(|x| x.as_i64()).unwrap_or_default(),
            hidden: v.get("hidden").and_then(|x| x.as_bool()).unwrap_or_default(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("name".into(), aylis::json::serde_json::Value::from(self.name.clone()));
        m.insert("values".into(), aylis::json::serde_json::Value::from(self.values.clone()));
        m.insert("x".into(), aylis::json::serde_json::Value::from(self.x.clone()));
        m.insert("radius".into(), aylis::json::serde_json::Value::from(self.radius.clone()));
        m.insert("kind".into(), aylis::json::serde_json::Value::from(aylis::prelude::PropEnum::name(self.kind)));
        m.insert("right_axis".into(), aylis::json::serde_json::Value::from(self.right_axis.clone()));
        m.insert("color".into(), aylis::json::serde_json::Value::from(self.color.to_hex()));
        m.insert("width".into(), aylis::json::serde_json::Value::from(self.width.clone()));
        m.insert("dash".into(), aylis::json::serde_json::Value::from(self.dash.clone()));
        m.insert("point".into(), aylis::json::serde_json::Value::from(aylis::prelude::PropEnum::name(self.point)));
        m.insert("fill".into(), aylis::json::serde_json::Value::from(aylis::prelude::PropEnum::name(self.fill)));
        m.insert("fill_series".into(), aylis::json::serde_json::Value::from(self.fill_series.clone()));
        m.insert("hidden".into(), aylis::json::serde_json::Value::from(self.hidden.clone()));
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

impl Default for ChartSeries {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSpec {
    pub field: String,
    pub title: String,
    pub width: f64,
    pub read_only: bool,
    pub color: aylis::prelude::Color,
    pub text_color: aylis::prelude::Color,
}

impl ColumnSpec {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            field: v.get("field").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            title: v.get("title").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            width: v.get("width").and_then(|x| x.as_f64()).unwrap_or_default(),
            read_only: v.get("read_only").and_then(|x| x.as_bool()).unwrap_or_default(),
            color: v.get("color").and_then(|x| x.as_str()).and_then(aylis::prelude::Color::from_hex).unwrap_or_default(),
            text_color: v.get("text_color").and_then(|x| x.as_str()).and_then(aylis::prelude::Color::from_hex).unwrap_or_default(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("field".into(), aylis::json::serde_json::Value::from(self.field.clone()));
        m.insert("title".into(), aylis::json::serde_json::Value::from(self.title.clone()));
        m.insert("width".into(), aylis::json::serde_json::Value::from(self.width.clone()));
        m.insert("read_only".into(), aylis::json::serde_json::Value::from(self.read_only.clone()));
        m.insert("color".into(), aylis::json::serde_json::Value::from(self.color.to_hex()));
        m.insert("text_color".into(), aylis::json::serde_json::Value::from(self.text_color.to_hex()));
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

impl Default for ColumnSpec {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct FilterParam {
    pub name: String,
    pub value: String,
}

impl FilterParam {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            name: v.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            value: v.get("value").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("name".into(), aylis::json::serde_json::Value::from(self.name.clone()));
        m.insert("value".into(), aylis::json::serde_json::Value::from(self.value.clone()));
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

impl Default for FilterParam {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct GridTotal {
    pub field: String,
    pub function: GridTotalFunction,
}

impl GridTotal {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            field: v.get("field").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            function: v.get("function").and_then(|x| x.as_str()).and_then(<GridTotalFunction as aylis::prelude::PropEnum>::from_name).or_else(|| <GridTotalFunction as aylis::prelude::PropEnum>::from_index(0)).expect("enum"),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("field".into(), aylis::json::serde_json::Value::from(self.field.clone()));
        m.insert("function".into(), aylis::json::serde_json::Value::from(aylis::prelude::PropEnum::name(self.function)));
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

impl Default for GridTotal {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct IndexDef {
    pub name: String,
    pub fields: String,
}

impl IndexDef {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            name: v.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            fields: v.get("fields").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("name".into(), aylis::json::serde_json::Value::from(self.name.clone()));
        m.insert("fields".into(), aylis::json::serde_json::Value::from(self.fields.clone()));
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

impl Default for IndexDef {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct MasterField {
    pub field: String,
    pub master_field: String,
}

impl MasterField {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            field: v.get("field").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            master_field: v.get("master_field").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("field".into(), aylis::json::serde_json::Value::from(self.field.clone()));
        m.insert("master_field".into(), aylis::json::serde_json::Value::from(self.master_field.clone()));
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

impl Default for MasterField {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}
include!("components/classes/ClientDataSet.rs");
include!("components/classes/DBChart.rs");
include!("components/classes/DataGrid.rs");
include!("components/classes/DataSource.rs");
include!("components/classes/Field.rs");
include!("components/classes/Form.rs");
include!("components/classes/GroupBox.rs");
include!("components/classes/Image.rs");
include!("components/classes/Label.rs");
include!("components/classes/Layout.rs");
include!("components/classes/Panel.rs");
include!("components/classes/Splitter.rs");
include!("components/classes/Svg.rs");
include!("components/forms/ibge.rs");
