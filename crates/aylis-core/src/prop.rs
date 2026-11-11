//! Sistema de propriedades publicadas (o "RTTI" do aylis).
//!
//! Cada componente declara suas propriedades com [`PropInfo`] e as lê/grava por nome
//! com [`PropValue`]. É isso que permite carregar e salvar layouts, o Object Inspector
//! da IDE, animações por nome e o controle remoto da tela por outras linguagens.

use crate::align::Anchors;
use crate::color::Color;
use crate::component::ComponentId;
use crate::error::{Error, Result};
use crate::geometry::Bounds;

/// Enum que pode ser usado como propriedade (lido e gravado pelo nome).
pub trait PropEnum: Sized + Copy + 'static {
    /// Nomes de todos os valores, na ordem de [`PropEnum::index`].
    const NAMES: &'static [&'static str];

    fn from_index(index: usize) -> Option<Self>;
    fn index(self) -> usize;

    fn name(self) -> &'static str {
        Self::NAMES[self.index()]
    }

    /// Procura pelo nome, sem diferenciar maiúsculas.
    fn from_name(name: &str) -> Option<Self> {
        Self::NAMES
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .and_then(Self::from_index)
    }
}

/// Tipo de uma propriedade; define o editor no Object Inspector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropKind {
    Bool,
    Int,
    Float,
    String,
    Color,
    /// Um valor entre os nomes dados.
    Enum(&'static [&'static str]),
    /// Zero ou mais valores entre os nomes dados.
    Set(&'static [&'static str]),
    Bounds,
    /// Referência a outro componente (gravada pelo nome).
    Component,
    /// Evento (gravado pelo nome do handler).
    Event,
    /// Ângulo em graus (valor `Float`). Na IDE: o relógio de ângulo.
    Angle,
    /// Duração em milissegundos (valor `Float`). Na IDE: a linha do tempo da animação.
    Duration,
    /// Sombra `"dx dy desfoque [espalhamento] #cor"` (valor `String`; vazio = sem sombra).
    /// Na IDE: o editor de sombra.
    Shadow,
    /// Curva de suavização: um nome (`out_quad`, `ease-in`...) ou
    /// `cubic-bezier(x1, y1, x2, y2)` (valor `String`). Na IDE: o editor da curva.
    Easing,
}

/// O tipo da propriedade: o do campo, ou o dado com `as` no [`properties!`].
#[doc(hidden)]
pub const fn kind_or(default: PropKind, over: &[PropKind]) -> PropKind {
    if let [k] = over { *k } else { default }
}

/// Metadados de uma propriedade publicada.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropInfo {
    pub name: &'static str,
    pub kind: PropKind,
    /// Agrupamento no Object Inspector ("Layout", "Appearance", "Events"...).
    pub category: &'static str,
    pub read_only: bool,
    /// Só em execução (como o `SelStart` do Delphi): lida e gravada pelo código e pelo
    /// protocolo, mas não vai para o JSON do form, não é copiada ao duplicar e não aparece
    /// no Object Inspector.
    pub runtime: bool,
}

impl PropInfo {
    pub const fn new(name: &'static str, kind: PropKind, category: &'static str) -> Self {
        Self { name, kind, category, read_only: false, runtime: false }
    }

    pub const fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub const fn runtime(mut self) -> Self {
        self.runtime = true;
        self
    }
}

/// Valor de uma propriedade.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum PropValue {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Color(Color),
    Enum(String),
    Set(Vec<String>),
    Bounds(Bounds),
    Component(Option<ComponentId>),
    Event(Option<String>),
}

impl PropValue {
    pub fn kind_name(&self) -> &'static str {
        match self {
            PropValue::Null => "null",
            PropValue::Bool(_) => "bool",
            PropValue::Int(_) => "int",
            PropValue::Float(_) => "float",
            PropValue::String(_) => "string",
            PropValue::Color(_) => "color",
            PropValue::Enum(_) => "enum",
            PropValue::Set(_) => "set",
            PropValue::Bounds(_) => "bounds",
            PropValue::Component(_) => "component",
            PropValue::Event(_) => "event",
        }
    }

    pub fn to_bool(&self) -> Result<bool> {
        match self {
            PropValue::Bool(b) => Ok(*b),
            _ => Err(Error::type_mismatch("bool", self)),
        }
    }

    pub fn to_i64(&self) -> Result<i64> {
        match self {
            PropValue::Int(v) => Ok(*v),
            PropValue::Float(v) if v.fract() == 0.0 => Ok(*v as i64),
            _ => Err(Error::type_mismatch("int", self)),
        }
    }

    pub fn to_f64(&self) -> Result<f64> {
        match self {
            PropValue::Float(v) => Ok(*v),
            PropValue::Int(v) => Ok(*v as f64),
            _ => Err(Error::type_mismatch("float", self)),
        }
    }

    pub fn to_f32(&self) -> Result<f32> {
        self.to_f64().map(|v| v as f32)
    }

    pub fn to_str(&self) -> Result<&str> {
        match self {
            PropValue::String(s) | PropValue::Enum(s) => Ok(s),
            _ => Err(Error::type_mismatch("string", self)),
        }
    }

    /// Aceita uma cor ou um texto `#RRGGBB[AA]`.
    pub fn to_color(&self) -> Result<Color> {
        match self {
            PropValue::Color(c) => Ok(*c),
            PropValue::String(s) => s.parse(),
            _ => Err(Error::type_mismatch("color", self)),
        }
    }

    /// Aceita `Bounds` ou um número (o mesmo valor nos quatro lados).
    /// Aceita também o texto, como no JSON: `"8"` (todos), `"16, 12"` (horizontal, vertical)
    /// ou `"12, 4, 12, 0"` (esquerda, cima, direita, baixo); vírgula ou espaço separam. É como
    /// o acesso tipado das linguagens manda `margins` e `padding`.
    pub fn to_bounds(&self) -> Result<Bounds> {
        match self {
            PropValue::Bounds(b) => Ok(*b),
            PropValue::Int(_) | PropValue::Float(_) => Ok(Bounds::all(self.to_f32()?)),
            PropValue::String(s) => {
                let numbers: Option<Vec<f32>> = s.split([',', ' ', ';']).filter(|p| !p.trim().is_empty()).map(|p| p.trim().parse::<f32>().ok()).collect();
                match numbers.as_deref() {
                    Some([all]) => Ok(Bounds::all(*all)),
                    Some([h, v]) => Ok(Bounds::symmetric(*h, *v)),
                    Some([l, t, r, b]) => Ok(Bounds::new(*l, *t, *r, *b)),
                    _ => Err(Error::invalid(format!("margens inválidas '{s}'; use 1, 2 (h, v) ou 4 (esquerda, cima, direita, baixo) números"))),
                }
            }
            _ => Err(Error::type_mismatch("bounds", self)),
        }
    }

    /// Aceita o nome do valor (como `Enum` ou `String`).
    pub fn to_enum<T: PropEnum>(&self) -> Result<T> {
        let name = match self {
            PropValue::Enum(s) | PropValue::String(s) => s,
            _ => return Err(Error::type_mismatch("enum", self)),
        };
        T::from_name(name).ok_or_else(|| {
            Error::invalid(format!("valor '{name}' inválido; use um de: {}", T::NAMES.join(", ")))
        })
    }

    /// Aceita uma lista de nomes ou um texto separado por vírgulas (`"left,right"`).
    pub fn to_anchors(&self) -> Result<Anchors> {
        let parsed = match self {
            PropValue::Set(names) => Anchors::from_names(names),
            PropValue::String(s) | PropValue::Enum(s) => {
                Anchors::from_names(s.split(',').map(str::trim).filter(|p| !p.is_empty()))
            }
            _ => return Err(Error::type_mismatch("set", self)),
        };
        parsed.ok_or_else(|| {
            Error::invalid(format!("âncoras inválidas; use: {}", Anchors::NAMES.join(", ")))
        })
    }

    pub fn to_component(&self) -> Result<Option<ComponentId>> {
        match self {
            PropValue::Component(c) => Ok(*c),
            PropValue::Null => Ok(None),
            _ => Err(Error::type_mismatch("component", self)),
        }
    }

    /// Nome do handler de um evento (`None` desliga o evento).
    pub fn to_event(&self) -> Result<Option<String>> {
        match self {
            PropValue::Event(e) => Ok(e.clone()),
            PropValue::String(s) => Ok((!s.is_empty()).then(|| s.clone())),
            PropValue::Null => Ok(None),
            _ => Err(Error::type_mismatch("event", self)),
        }
    }
}

impl From<bool> for PropValue {
    fn from(v: bool) -> Self {
        PropValue::Bool(v)
    }
}

impl From<i32> for PropValue {
    fn from(v: i32) -> Self {
        PropValue::Int(v as i64)
    }
}

impl From<i64> for PropValue {
    fn from(v: i64) -> Self {
        PropValue::Int(v)
    }
}

impl From<u32> for PropValue {
    fn from(v: u32) -> Self {
        PropValue::Int(v as i64)
    }
}

impl From<f32> for PropValue {
    fn from(v: f32) -> Self {
        PropValue::Float(v as f64)
    }
}

impl From<f64> for PropValue {
    fn from(v: f64) -> Self {
        PropValue::Float(v)
    }
}

impl From<&str> for PropValue {
    fn from(v: &str) -> Self {
        PropValue::String(v.to_string())
    }
}

impl From<String> for PropValue {
    fn from(v: String) -> Self {
        PropValue::String(v)
    }
}

impl From<Color> for PropValue {
    fn from(v: Color) -> Self {
        PropValue::Color(v)
    }
}

impl From<Bounds> for PropValue {
    fn from(v: Bounds) -> Self {
        PropValue::Bounds(v)
    }
}

impl From<Anchors> for PropValue {
    fn from(v: Anchors) -> Self {
        PropValue::Set(v.names())
    }
}

impl From<ComponentId> for PropValue {
    fn from(v: ComponentId) -> Self {
        PropValue::Component(Some(v))
    }
}

impl From<Option<ComponentId>> for PropValue {
    fn from(v: Option<ComponentId>) -> Self {
        PropValue::Component(v)
    }
}

impl From<crate::align::Align> for PropValue {
    fn from(v: crate::align::Align) -> Self {
        PropValue::enum_value(v)
    }
}

impl PropValue {
    /// Valor de enum a partir de qualquer [`PropEnum`].
    pub fn enum_value<T: PropEnum>(v: T) -> Self {
        PropValue::Enum(v.name().to_string())
    }
}

/// Tipo que pode ser campo de uma propriedade publicada (usado pelo [`properties!`]).
pub trait PropType: Sized {
    const KIND: PropKind;
    fn to_value(&self) -> PropValue;
    fn from_value(value: PropValue) -> Result<Self>;
}

impl PropType for bool {
    const KIND: PropKind = PropKind::Bool;
    fn to_value(&self) -> PropValue {
        PropValue::Bool(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_bool()
    }
}

impl PropType for i64 {
    const KIND: PropKind = PropKind::Int;
    fn to_value(&self) -> PropValue {
        PropValue::Int(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_i64()
    }
}

macro_rules! int_prop {
    ($($t:ty),*) => {$(
        impl PropType for $t {
            const KIND: PropKind = PropKind::Int;
            fn to_value(&self) -> PropValue {
                PropValue::Int(*self as i64)
            }
            fn from_value(value: PropValue) -> Result<Self> {
                let v = value.to_i64()?;
                <$t>::try_from(v).map_err(|_| Error::invalid(format!("valor {v} fora do intervalo")))
            }
        }
    )*};
}

int_prop!(i32, u32, usize);

impl PropType for f32 {
    const KIND: PropKind = PropKind::Float;
    fn to_value(&self) -> PropValue {
        PropValue::Float(*self as f64)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_f32()
    }
}

impl PropType for f64 {
    const KIND: PropKind = PropKind::Float;
    fn to_value(&self) -> PropValue {
        PropValue::Float(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_f64()
    }
}

impl PropType for String {
    const KIND: PropKind = PropKind::String;
    fn to_value(&self) -> PropValue {
        PropValue::String(self.clone())
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_str().map(str::to_string)
    }
}

impl PropType for Color {
    const KIND: PropKind = PropKind::Color;
    fn to_value(&self) -> PropValue {
        PropValue::Color(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_color()
    }
}

impl PropType for Bounds {
    const KIND: PropKind = PropKind::Bounds;
    fn to_value(&self) -> PropValue {
        PropValue::Bounds(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_bounds()
    }
}

impl PropType for Anchors {
    const KIND: PropKind = PropKind::Set(&Anchors::NAMES);
    fn to_value(&self) -> PropValue {
        (*self).into()
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_anchors()
    }
}

/// `None` = usar o valor do tema. `Null` (ou texto vazio) volta para `None`.
impl PropType for Option<f32> {
    const KIND: PropKind = PropKind::Float;
    fn to_value(&self) -> PropValue {
        self.map_or(PropValue::Null, |v| PropValue::Float(v as f64))
    }
    fn from_value(value: PropValue) -> Result<Self> {
        match value {
            PropValue::Null => Ok(None),
            v => v.to_f32().map(Some),
        }
    }
}

/// `None` = usar a cor do tema. `Null` (ou texto vazio) volta para `None`.
impl PropType for Option<Color> {
    const KIND: PropKind = PropKind::Color;
    fn to_value(&self) -> PropValue {
        self.map_or(PropValue::Null, PropValue::Color)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        match value {
            PropValue::Null => Ok(None),
            PropValue::String(s) if s.is_empty() => Ok(None),
            v => v.to_color().map(Some),
        }
    }
}

/// `None` = sem sombra (a do tema, se houver). Texto: `"0 4 12 #00000040"`.
impl PropType for Option<crate::canvas::Shadow> {
    const KIND: PropKind = PropKind::Shadow;
    fn to_value(&self) -> PropValue {
        self.map_or(PropValue::Null, |s| PropValue::String(s.to_spec()))
    }
    fn from_value(value: PropValue) -> Result<Self> {
        match value {
            PropValue::Null => Ok(None),
            v => {
                let text = v.to_str()?;
                if text.trim().is_empty() || text.trim().eq_ignore_ascii_case("none") {
                    return Ok(None);
                }
                crate::canvas::Shadow::parse(text)
                    .map(Some)
                    .ok_or_else(|| Error::invalid(format!("sombra inválida: '{text}'; use \"dx dy desfoque [espalhamento] #cor\"")))
            }
        }
    }
}

/// Referência a outro componente.
impl PropType for Option<ComponentId> {
    const KIND: PropKind = PropKind::Component;
    fn to_value(&self) -> PropValue {
        PropValue::Component(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_component()
    }
}

impl<T: PropEnum> PropType for T {
    const KIND: PropKind = PropKind::Enum(T::NAMES);
    fn to_value(&self) -> PropValue {
        PropValue::enum_value(*self)
    }
    fn from_value(value: PropValue) -> Result<Self> {
        value.to_enum()
    }
}

/// Publica campos como propriedades, gerando `props`, `get_prop` e `set_prop` de um
/// [`Component`](crate::Component). Use dentro do `impl Component for ...`.
///
/// O nome da propriedade é o nome do campo. O tipo dela vem do tipo do campo; para trocar
/// o editor na IDE, use `as`: `giro: f32 as Angle`, `tempo: f32 as Duration`.
///
/// Seções opcionais, separadas por `;`:
/// - `events:` eventos próprios da classe (os comuns, como `on_click`, já existem em
///   todo controle);
/// - `on_set:` método `fn(&mut self, name: &str)` chamado depois que uma propriedade é
///   gravada (para manter outros dados em sincronia).
///
/// ```
/// use aylis_core::*;
///
/// #[derive(Default)]
/// struct Gauge { base: ControlBase, value: f32, caption: String, color: Option<Color> }
///
/// impl Component for Gauge {
///     fn class_name(&self) -> &'static str { "Gauge" }
///     fn control(&self) -> Option<&ControlBase> { Some(&self.base) }
///     fn control_mut(&mut self) -> Option<&mut ControlBase> { Some(&mut self.base) }
///
///     properties! {
///         value: f32 => "Data",
///         caption: String => "Appearance",
///         color: Option<Color> => "Appearance";
///         events: on_change
///     }
/// }
///
/// let mut ui = Ui::new();
/// let g = ui.add_root(Gauge::default());
/// ui.set_prop(g, "value", 42).unwrap();
/// assert_eq!(ui.get::<Gauge>(g).unwrap().value, 42.0);
/// assert!(ui.prop_info(g, "on_change").is_some());
/// ```
#[macro_export]
macro_rules! properties {
    (
        $( $field:ident : $ty:ty $( as $kind:ident )? => $category:literal ),* $(,)?
        $( ; events: $( $event:ident ),* $(,)? )?
        $( ; computed: $( $cname:ident : $cty:ty => $cget:ident / $cset:ident ),* $(,)? )?
        $( ; on_set: $hook:ident )?
    ) => {
        fn props(&self) -> &'static [$crate::PropInfo] {
            const PROPS: &[$crate::PropInfo] = &[
                $( $crate::PropInfo::new(
                    stringify!($field),
                    $crate::prop::kind_or(<$ty as $crate::prop::PropType>::KIND, &[$( $crate::PropKind::$kind )?]),
                    $category,
                ), )*
                $( $( $crate::PropInfo::new(stringify!($event), $crate::PropKind::Event, "Events"), )* )?
                $( $( $crate::PropInfo::new(stringify!($cname), <$cty as $crate::prop::PropType>::KIND, "Runtime").runtime(), )* )?
            ];
            PROPS
        }

        fn get_prop(&self, name: &str) -> ::core::option::Option<$crate::PropValue> {
            match name {
                $( stringify!($field) => ::core::option::Option::Some($crate::prop::PropType::to_value(&self.$field)), )*
                $( $( stringify!($cname) => ::core::option::Option::Some($crate::prop::PropType::to_value(&self.$cget())), )* )?
                _ => ::core::option::Option::None,
            }
        }

        fn set_prop(&mut self, name: &str, value: $crate::PropValue) -> $crate::Result<()> {
            match name {
                $( stringify!($field) => {
                    self.$field = <$ty as $crate::prop::PropType>::from_value(value)?;
                } )*
                $( $( stringify!($cname) => {
                    let v: $cty = <$cty as $crate::prop::PropType>::from_value(value)?;
                    self.$cset(v);
                } )* )?
                _ => return ::core::result::Result::Err($crate::Error::UnknownProperty(name.to_string())),
            }
            $( self.$hook(name); )?
            ::core::result::Result::Ok(())
        }
    };
}
