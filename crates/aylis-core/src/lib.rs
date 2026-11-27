//! # aylis-core
//!
//! Núcleo do aylis: modelo de componentes, árvore, propriedades publicadas, layout
//! (Align, Anchors, Margins, Padding), temas e as interfaces (traits) com a
//! plataforma.
//!
//! Este crate **não depende de nada do sistema operacional**. Janela, input, timers e o
//! desenho de verdade ficam em crates de plataforma e de renderização, que implementam
//! os traits definidos aqui ([`Canvas`], [`TextService`], [`platform`]).
//!
//! ## Conceitos
//!
//! | Conceito | No Aylis |
//! |---|---|
//! | Componente | qualquer tipo que implementa [`Component`] |
//! | Controle visual | um [`Component`] que tem um [`ControlBase`] |
//! | Estender um componente | composição: o componente guarda o "pai" e devolve em [`Component::inner`] |
//! | Criar e pôr num pai | [`Ui::add`] / [`Ui::create`] |
//! | Liberar | [`Ui::free`] (libera os filhos junto) |
//! | Achar pelo nome | [`Ui::find`] / [`Ui::find_in`] / [`Ui::find_path`] (`"frmClientes.btnSalvar"`) |
//! | Nomes únicos por form | [`Ui::form_of`]: todo form é um escopo de nomes |
//! | Abrir, abrir modal e fechar | [`Ui::show`] / [`Ui::show_modal`] (resultado numa função) / [`Ui::close`] |
//! | Propriedade por nome | [`Ui::set_prop`] / [`Ui::get_prop`] |
//! | Ligar um evento | [`Ui::on_click`] / [`Ui::on`] ou por nome via [`Ui::register_handler`] |

pub mod activity;
pub mod automation;
pub mod background;
mod busy;
pub mod align;
pub mod anim;
pub mod canvas;
pub mod clipboard;
pub mod color;
pub mod component;
pub mod cursor_image;
pub mod devices;
pub mod error;
pub mod files;
pub mod event;
pub mod event_value;
pub mod exception;
pub mod fonts;
pub mod forms;
/// A escala, o fim do redimensionamento e o mouse sobre a janela de um form.
pub mod form_window;
/// Os monitores (lista, área útil, taxa) e a posição guardada de cada form.
pub mod monitors;
pub mod ndef;
mod frames;
pub use frames::{INPUT_GRACE, InputLatency};
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
pub mod keycode;
pub mod keys;
pub mod memory_panel;
pub mod messages;
mod input;
mod intercept;
mod keyflow;
mod layout;
pub use layout::is_right_to_left_language;
pub mod overscroll;
pub mod orientation;
pub mod system_bars;
pub mod system_keys;
/// Um form em vários monitores (uma janela por monitor).
pub mod span;
pub mod pages;
pub mod paint;
pub mod palette;
pub mod path;
pub mod date;
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
    BlendMode, Brush, Canvas, FontSpec, FontWeight, GpuDevice, GpuSurface, GradientStop, LineCap, LineJoin, Shadow, Stroke, TextAlign,
    TextAntialias, TextService, TextStyle, decorate_line,
};
pub use color::Color;
pub use component::{
    ArrangeItem, ChildPointer, ClipShape, ColorOverride, Component, ComponentId, Connection, ControlBase, ControlState, DesignHooks, DragDrop, LayoutHooks, Popup, Timed, AccessibleItems, FONT_WEIGHTS, FontOverride, MethodInfo, NativeCommand, NativeView, NativeViewSlot, PlatformHooks, WebMessage, navigation_allowed, url_origin,
    prop_infos_of, read_prop,
};
pub use error::{Error, Result};
pub use ui::DesignChanged;
pub use input::{DragEnd, DragVisual, FORM_DRAG, HINT_DELAY, ScreenPointer};
pub use event::{
    ClickPointer, EventArgs, EventCtx, GestureEvent, GesturePhase, Intercepted, Key, KeyEvent, Modifiers, PointerButton, PointerEvent, PointerKind,
    PopupRequest, ScrollOptions, SwipeDirection, SwipeEvent, WHEEL_LINE, WheelEvent,
};
pub use keycode::KeyCode;
pub use cursor_image::CursorImage;
pub use system_keys::{KeyboardCaptureRequest, LockKeys};
pub use forms::{ModalResult, WindowPlacement};
pub use bindings::Binding;
pub use timing::TimerPrecision;
pub use overscroll::Overscroll;
pub use orientation::{FormOrientation, ScreenOrientation};
pub use system_bars::{BarIcons, SystemBarsStyle};
pub use transition::{TRANSITION_MS, Transition};
pub use taskbar::{JumpTask, TaskbarEvent, TaskbarProgress, TaskbarSpec, ThumbButton};
pub use tray::{TrayEvent, TrayEventKind, TraySpec, parse_guid};
pub use window::{BorderColor, ResizeEdge, WINDOW_SETTINGS_CLASS, WindowBackdrop, WindowCommand, WindowOptions};
pub use geometry::{Bounds, PointF, RectF, SizeF};
pub use icons::Icon;
pub use keys::{CharCase, CharFilter, mnemonic, parse_shortcut, shortcut_matches, shortcut_text, strip_mnemonic};
pub use image::{Image, ImageFit, ImageSource, WeakImage};
pub use image_cache::{ImageCache, ImageCacheLimits, LayerCache, LayerCaching};
pub use paint::PaintCtx;
pub use path::{Path, PathCmd};
pub use date::{Date, DateTime, Time};
pub use prop::RecordInfo;
pub use event_value::{DateRange, EventArg, EventValue, FromSender};
pub use gpu::{BlendCompositor, FormBlendCompositor, FormPresent};
pub use platform::{Biometry, CursorShape, DesktopService, NotificationPriority, Permission, PermissionState, SecondInstance, SingleInstance};
pub use providers::ErrorCorrection;
pub use post::Poster;
pub use prop::{PropEnum, PropInfo, PropKind, PropValue};
pub use registry::{ClassInfo, ClassRegistry, VariantInfo, split_variant};
pub use text::{TextCase, Trimming};
pub use theme::{ColorRef, Look, Material, MenuMetrics, Metric, ResolvedStyle, ScrollBarLook, Style, StyleProps, Theme};
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
