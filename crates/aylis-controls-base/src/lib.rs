//! # aylis-controls-base
//!
//! Os controles básicos do aylis: forms, botões, edição, listas, menus, abas, diálogos e
//! componentes de sistema, Quasar e o editor de código. Os apps usam pelo `aylis-controls`,
//! que junta este crate com os das outras famílias e registra as classes.
//!
//! Toda a aparência vem do tema (estilo com o nome da classe); propriedades da instância
//! como `fill` ou `color`, quando definidas, ganham do tema.

mod autocomplete;
mod basic;
pub mod docking;
mod mnemonic;
mod chat;
mod chips;
mod clipboard;
mod clock;
mod color;
mod combo;
mod containers;
mod dates;
mod edit;
mod emoji;
mod feedback;
pub mod field;
mod files;
mod grid;
mod drag_image;
mod image;
mod lists;
pub mod looks;
mod memo;
mod code_editor;
mod tab_strip;
pub mod search;
pub mod scroll_bar;
pub mod syntax;
pub mod textmate;
pub mod textmate_theme;
mod menus;
mod navigation;
mod number;
mod planner;
mod records;
mod platform;
pub mod quasar;
mod rich_editor;
pub mod shapes;
pub use shapes::LineDirection;
pub mod recorder;
pub mod recycler;
pub mod list_header;
pub mod image_list;
mod render_settings;
mod window_settings;
mod system;
mod popover;
mod tabs;
mod text_menu;
mod taskbar;
mod titlebar;
mod tray;
mod frame_view;
mod crop_box;
mod media_card;
mod record_button;
mod screen_layout;
mod toggles;
mod widgets;

pub use autocomplete::{AutoComplete, fold};
pub use basic::{
    BorderStyle, Button, CloseAction, CursorConfine, Form, FullScreenMode, KeepAwake, WindowCorners, WindowShape, FormPosition, Frame, IconPosition, Label, Layout, Panel, ProgressBar, Rectangle, TOPMOST_LOST, TopmostLost, WindowState, find_matches, markup_plain, markup_runs, markup_spans, MarkupSpan,
    window_state_commands,
};
pub use chips::{Chip, ChipGroup, OtpInput, RangeSlider, Rating, SplitButton};
pub use clipboard::Clipboard;
pub use clock::TimePicker;
pub use color::{ColorEdit, ColorPicker};
pub use combo::{ComboArrow, ComboBox};
pub use feedback::{ActionSheet, action_sheet, EmptyState, Pagination, ProgressCircle, SkeletonLoader, SnackAction, Snackbar, Stepper, Timeline, snackbar};
pub use files::{FileDropZone, FileInput};
pub use grid::{Divider, FlowLayout, GridLayout};
pub use containers::{ScrollBox, SmoothScroll, Splitter};
pub use edit::{CaretStyle, Edit, apply_mask, check_rules, validate_form};
pub use drag_image::DragImage;
pub use image::ImageView;
pub use menus::{MainMenu, MenuItem, PopupMenu, parse_shortcut};
pub use toggles::{CheckBox, RadioButton, Switch};
pub use chat::{ChatMessage, ChatView, MessageStatus, Waveform};
pub use dates::{Calendar, DateEdit, DateTimeEdit, Day, TimeEdit, days_in_month};
pub use popover::{Popover, PopoverPlacement};
pub use lists::{LIST_VIEW_ITEM, ListBox, ListItem, ListView, RadioGroup, TREE_NODE, TreeLeafLayout, TreeNode, TreeNodeRects, TreeView};
pub use code_editor::CodeEditor;
pub use tab_strip::{Tab, TabDrop, TabMove, TabStrip};
pub use memo::{Bookmark, Diagnostic, Memo, Suggestion, block_end};
pub use navigation::{AvatarGroup, BottomNavigation, BottomSheet, Carousel, NavigationDrawer, SpeedDial};
pub use number::{NumberBox, TrackBar};
pub use platform::{
    ActivityRings, AppBar, AppBarSize, ButtonGroup, Gauge, LoadingIndicator, NavigationRail, PageControl, SideSheet, UpDown, WheelPicker,
    paint_glass,
};
pub use emoji::{EmojiInfo, EmojiPicker, emoji_groups, emoji_list};
pub use titlebar::TitleBar;
pub use taskbar::Taskbar;
pub use tray::TrayIcon;
pub use planner::{CalendarEvent, CalendarEventClick, CalendarView, CommandPalette, EventCalendar, PALETTE_SELECTION, PaletteSelection, parse_events};
pub use render_settings::RenderSettings;
pub use window_settings::{WindowSettings, WindowSettingsScope};
pub use image_list::{IMAGE_LIST_ITEM, ImageList, ImageListItem, Picture};
pub use list_header::{HEADER_COLUMN, HeaderColumn, ListHeader};
pub use recycler::{Kinder, ListOrientation, RowSource, ScrollBarMode, SelectionMode, Sizer, SwipeAction, VirtualList};
pub use recorder::{Analytics, EventRecorder, Recording, RECORDING_EXTENSION};
pub use system::{ApplicationEvents, ExceptionEvent, BusyAction, BusyWatcher, DataModule, DialogKind, FrameTimer, DlgKind, EventInterceptor, InterceptScope, MessageDialog, Notification, OpenDialog, SaveDialog, Timer, confirm, message_dlg, parse_filter, prompt, set_dialog_icon, set_prompt_required, show_message, toast};
pub use tabs::{Expander, GroupBox, StatusBar, TabControl, TabItem, TabPosition, ToolBar};
pub use feedback::{ToastKind, ToastStack, notify};
pub use frame_view::FrameView;
pub use crop_box::{CropBox, CropSelection};
pub use media_card::MediaCard;
pub use quasar::{AjaxBar, Card, ChatBubble, InnerLoading, Knob, MarkupTable, OptionGroup, OptionKind, Parallax, PopupEdit, SlideItem, SpinnerKind, TableSeparator, Uploader, paint_spinner};
pub use rich_editor::{Block as RichBlock, BlockKind as RichBlockKind, DEFAULT_TOOLBAR as RICH_EDITOR_TOOLBAR, Format as RichFormat, RichEditor};
pub use record_button::RecordButton;
pub use screen_layout::{ScreenItem, ScreenLayout};
pub use widgets::{
    FpsMeter,
    ActivityIndicator, Avatar, Badge, BadgeAnchor, Banner, Breadcrumb, DirectionPad, Joystick, HotkeyEdit, IconView, LevelMeter, Presence,
    SeekBar, SegmentedControl, format_time, hotkey_text, name_color,
};

pub(crate) use aylis_core::leaf_control;
