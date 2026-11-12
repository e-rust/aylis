//! # aylis-core
//!
//! Núcleo do aylis: modelo de componentes, árvore, propriedades publicadas, layout no
//! estilo FMX (Align, Anchors, Margins, Padding), temas e as interfaces (traits) com a
//! plataforma.
//!
//! Este crate **não depende de nada do sistema operacional**. Janela, input, timers e o
//! desenho de verdade ficam em crates de plataforma e de renderização, que implementam
//! os traits definidos aqui ([`Canvas`], [`TextService`], [`platform`]).
//!
//! ## Conceitos (comparando com o Delphi)
//!
//! | Delphi / FMX | aylis |
//! |---|---|
//! | `TComponent` | qualquer tipo que implementa [`Component`] |
//! | `TControl` | um [`Component`] que tem um [`ControlBase`] |
//! | `inherited` | composição: o componente guarda o "pai" e devolve em [`Component::inner`] |
//! | `TButton.Create(Self)` + `Parent := X` | [`Ui::add`] / [`Ui::create`] |
//! | `Free` | [`Ui::free`] (libera os filhos junto) |
//! | `FindComponent` | [`Ui::find`] / [`Ui::find_in`] / [`Ui::find_path`] (`"frmClientes.btnSalvar"`) |
//! | `Owner` (nomes únicos por form) | [`Ui::form_of`]: todo form é um escopo de nomes |
//! | `Show` / `ShowModal` / `Close` | [`Ui::show`] / [`Ui::show_modal`] (resultado numa função) / [`Ui::close`] |
//! | `SetPropValue` / RTTI | [`Ui::set_prop`] / [`Ui::get_prop`] |
//! | `OnClick := ...` | [`Ui::on_click`] / [`Ui::on`] ou por nome via [`Ui::register_handler`] |

pub mod activity;
pub mod automation;
pub mod background;
mod busy;
pub mod align;
pub mod anim;
pub mod canvas;
pub mod color;
pub mod component;
pub mod devices;
pub mod error;
pub mod files;
pub mod event;
pub mod exception;
pub mod fonts;
pub mod forms;
/// Os monitores (lista, área útil, taxa) e a posição guardada de cada form.
pub mod monitors;
pub mod ndef;
mod frames;
// Repintar só o que mudou.
mod damage;
pub use damage::{FrameDamage, ScrollMove};
mod ink;
mod trace;
pub mod handles;
pub mod instances;
pub mod geometry;
pub mod gpu;
pub mod i18n;
pub mod icons;
pub mod image;
pub mod image_cache;
pub mod keys;
pub mod memory_panel;
pub mod messages;
mod input;
mod intercept;
mod keyflow;
mod layout;
pub use layout::is_right_to_left_language;
pub mod pages;
pub mod paint;
pub mod palette;
pub mod path;
pub mod platform;
pub mod post;
pub mod tasks;
pub mod profile;
pub mod providers;
pub mod prop;
pub mod question;
pub mod registry;
mod references;
mod names;
mod reveal;
pub mod text;
pub mod textcore;
pub mod textassist;
pub mod taskbar;
pub mod theme;
pub mod bindings;
pub mod timing;
pub mod transition;
pub mod tray;
pub mod themes;
mod timers;
pub mod ui;
pub mod views;
pub mod window;

pub use align::{Align, Anchors, SelfAlign};
pub use i18n::{tr, tr_with};
pub use anim::{AnimationOptions, ColorAnimation, Easing, FloatAnimation, PropertyAnimation, Trigger};
pub use canvas::{
    Brush, Canvas, FontSpec, FontWeight, GpuDevice, GpuSurface, GradientStop, LineCap, LineJoin, Shadow, Stroke, TextAlign,
    TextService, TextStyle, decorate_line,
};
pub use color::Color;
pub use component::{
    ArrangeItem, ColorOverride, Component, ComponentId, Connection, ControlBase, ControlState, DesignHooks, FONT_WEIGHTS, FontOverride, MethodInfo, NativeCommand, NativeView, NativeViewSlot, PlatformHooks, WebMessage, navigation_allowed, url_origin,
    prop_infos_of, read_prop,
};
pub use error::{Error, Result};
pub use ui::DesignChanged;
pub use input::{DragEnd, DragVisual, HINT_DELAY, ScreenPointer};
pub use event::{
    EventArgs, EventCtx, GestureEvent, GesturePhase, Intercepted, Key, KeyEvent, Modifiers, PointerButton, PointerEvent, PointerKind,
    PopupRequest, ScrollOptions, WHEEL_LINE, WheelEvent,
};
pub use forms::{ModalResult, WindowPlacement};
pub use bindings::Binding;
pub use timing::TimerPrecision;
pub use transition::{TRANSITION_MS, Transition};
pub use taskbar::{JumpTask, TaskbarEvent, TaskbarProgress, TaskbarSpec, ThumbButton};
pub use tray::{TrayEvent, TrayEventKind, TraySpec};
pub use window::{ResizeEdge, WindowCommand};
pub use geometry::{Bounds, PointF, RectF, SizeF};
pub use icons::Icon;
pub use keys::{CharCase, CharFilter, mnemonic, parse_shortcut, shortcut_matches, shortcut_text, strip_mnemonic};
pub use image::{Image, ImageFit, ImageSource, WeakImage};
pub use image_cache::{ImageCache, ImageCacheLimits};
pub use paint::PaintCtx;
pub use path::{Path, PathCmd};
pub use platform::CursorShape;
pub use post::Poster;
pub use prop::{PropEnum, PropInfo, PropKind, PropValue};
pub use registry::{ClassInfo, ClassRegistry, VariantInfo, split_variant};
pub use text::{TextCase, Trimming};
pub use theme::{ColorRef, Look, Material, Metric, ResolvedStyle, Style, StyleProps, Theme};
pub use ui::ThemeSource;
pub use ui::Ui;
pub use exception::Exception;
pub use textcore::TextMode;
pub use question::{Answer, Asked};
pub use views::{FormView, ViewChange, ViewOrientation};
pub use ui::{EVENTS_PER_ROUND, LOOP_WARNING, MAX_EVENT_NESTING, MAX_TREE_DEPTH, SAFE_EVENT_NESTING, SAFE_TREE_DEPTH, is_identifier};

/// Uma linha no stderr, como o `eprintln!`, mas sem entrar em pânico se ele estiver fechado:
/// um app aberto pela IDE escreve num pipe, e com a IDE fechada (ou a execução parada) o
/// `eprintln!` derrubava o app ("failed printing to stderr").
#[macro_export]
macro_rules! stderr_line {
    ($($arg:tt)*) => {{
        use ::std::io::Write as _;
        let _ = ::std::writeln!(::std::io::stderr(), $($arg)*);
    }};
}

/// Implementa `class_name`, `control` e `control_mut` para um controle "folha" (que guarda o
/// próprio `ControlBase` no campo `base`). Fica no núcleo porque os controles moram em vários
/// crates (`aylis-controls-base`, `aylis-controls-charts`...).
#[macro_export]
macro_rules! leaf_control {
    ($name:literal) => {
        fn class_name(&self) -> &'static str {
            $name
        }

        fn control(&self) -> Option<&$crate::ControlBase> {
            Some(&self.base)
        }

        fn control_mut(&mut self) -> Option<&mut $crate::ControlBase> {
            Some(&mut self.base)
        }
    };
}
