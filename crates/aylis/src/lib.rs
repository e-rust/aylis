//! # aylis
//!
//! Interfaces visuais em Rust, com designer e desenho próprio igual em todas as
//! plataformas. Este crate junta as partes:
//!
//! | Módulo | O que tem |
//! |---|---|
//! | [`core`] | `Ui`, componentes, propriedades, layout, eventos, temas, forms |
//! | [`controls`] | Button, Edit, Label, Panel, ComboBox, Image... |
//! | [`data`] | ClientDataSet, Field, DataSource |
//! | [`dbcontrols`] | DataGrid, DataNavigator, DataRepeater, DBImage, DBChart, relatórios |
//! | [`providers`] | Barcode (código de barras e QR Code), Export (CSV, XLSX) e os provedores |
//! | [`json`] | layouts e temas em JSON |
//! | [`media`] | MediaPlayer (áudio), VideoPlayer, decodificadores |
//! | [`vector`] | Lottie (animações After Effects, pacote Noto) e Svg (imagens vetoriais) |
//! | [`markdown`] | Markdown (texto rico: títulos, listas, tabelas, código, links) |
//! | [`docview`] | DocViewer (PDF, XLSX/XLS/ODS/CSV, DOCX/ODT, sem programa externo) |
//! | [`webview`] | WebView (navegador embutido: WebView2, WKWebView, iframe) |
//! | [`scene3d`] | Model3D (modelos glTF/GLB animados, com controles por cima) |
//! | [`mcp`] | McpServer, McpTool, McpResource e McpPrompt: o app como servidor MCP (agentes de IA usam o app pelos componentes) |
//! | [`project`] | projetos (`.aylisproj`), geração de código |
//! | [`platform`] | janelas (desktop, celular, navegador) |
//! | [`headless`] | o modo sem plataforma: testes e fotos sem janela (clicar, digitar, avançar o relógio, PNG) |
//!
//! O jeito mais simples de começar é um projeto criado pela IDE. O código gerado é:
//!
//! ```ignore
//! // src/lib.rs
//! #[path = "../generated/mod.rs"]
//! pub mod generated;
//!
//! pub fn app() -> Result<aylis::Application, aylis::Error> {
//!     // forms, temas e assets embutidos pelo build.rs (aylis_build::project())
//!     let mut app = aylis::Application::bundled(aylis::include_project!())?;
//!     generated::register(&mut app.ui);
//!     Ok(app)
//! }
//!
//! // src/forms/principal.rs: os handlers são métodos da struct do form (gerada), com
//! // os componentes resolvidos e o estado de cada janela aberta.
//! impl FrmPrincipal {
//!     pub fn somar_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
//!         self.data.total += 1;
//!         self.lblTotal.set_text(ui, self.data.total.to_string());
//!     }
//! }
//!
//! // src/main.rs
//! fn main() -> Result<(), aylis::Error> {
//!     vendas::app()?.run()
//! }
//! ```
//!
//! Sem `build.rs`, [`Application::open`] lê o projeto de uma pasta.
//!
//! ## App enxuto
//!
//! O app publicado registra só as classes que os forms usam (o `build.rs` gera a lista,
//! veja [`classes`]): o código dos outros componentes nem entra no executável. Crates
//! inteiros (dados, controles de dados, mídia) saem desligando as features `data`,
//! `dbcontrols`, `media` e `remote`.

pub use aylis_controls as controls;
pub use aylis_core as core;
/// O texto no idioma do app de qualquer lugar e thread (o mesmo dicionário do `ui.tr`):
/// `aylis::tr("Gravando")`, `aylis::tr_with("{0} de {1}", &[&a, &b])`.
pub use aylis_core::{tr, tr_with};

/// Utilitários de texto para o app: [`text::fold`] (sem acento e sem maiúscula, para comparar
/// numa busca), a mesma função do `AutoComplete` e da `CommandPalette`.
pub mod text {
    pub use aylis_controls::fold;
}

/// Carrega as traduções do projeto embutido (`locales/<idioma>.json`) para o [`tr`], sem o
/// `Ui`: o app que precisa traduzir antes de a janela existir. Com a janela aberta, o
/// [`Application::set_language`] faz isto e traduz a tela.
pub fn load_translations(embedded: &EmbeddedProject, language: &str) {
    aylis_core::messages::set_language(language);
    let map = aylis_project::Bundle::parse(embedded.bundle)
        .ok()
        .and_then(|b| aylis_project::Files::Bundle(b).read(&aylis_project::i18n::locale_file(language)))
        .map(|text| aylis_project::i18n::parse(&String::from_utf8_lossy(&text)))
        .unwrap_or_default();
    aylis_core::i18n::set_dictionary(map);
}
/// O serviço em segundo plano: a lógica sem tela do projeto (`src/background.rs`).
pub use aylis_core::background;
#[cfg(feature = "data")]
pub use aylis_data as data;
#[cfg(feature = "dbcontrols")]
pub use aylis_dbcontrols as dbcontrols;
#[cfg(feature = "providers")]
pub use aylis_providers as providers;
pub use aylis_json as json;
#[cfg(feature = "media")]
pub use aylis_media as media;
#[cfg(feature = "vector")]
pub use aylis_vector as vector;
#[cfg(feature = "markdown")]
pub use aylis_markdown as markdown;
#[cfg(feature = "docview")]
pub use aylis_docview as docview;
#[cfg(feature = "webview")]
pub use aylis_webview as webview;
#[cfg(feature = "3d")]
pub use aylis_3d as scene3d;
#[cfg(feature = "mcp")]
pub use aylis_remote::mcp;
pub use aylis_platform_winit as platform;
pub use aylis_project as project;

/// O que o código de um form costuma usar: `use aylis::prelude::*;`.
pub mod prelude {
    // Os controles por uma lista explícita, e não `aylis_controls::*`: as structs que têm o
    // nome de um registro do acesso tipado (`TreeNode`, `MapMarker`, `PushButton`...) ficam de
    // fora, senão `use aylis::prelude::*` junto de `use generated::*` deixa o nome ambíguo. Elas seguem pelo caminho completo
    // (`aylis::controls::TreeNode`). O teste `prelude_names` do `aylis-tools` confere.
    pub use aylis_controls::{
        action_sheet, ActionSheet, ActivityIndicator, ActivityRings, Aggregate, AjaxBar, Analytics, AngleDial,
        AnimatedText, AppBar, AppBarSize, ApplicationEvents, AppLink, apply_mask, AutoComplete, Avatar,
        AvatarGroup, BackgroundService, Badge, BadgeAnchor, Banner, Battery, bi, Biometrics, Bitmap,
        block_end, Bluetooth, BLUETOOTH_DEVICE, BLUETOOTH_VALUE, BluetoothValue, Bookmark, BorderStyle,
        BottomNavigation, BottomSheet, Breadcrumb, BusyAction, BusyWatcher, Button, ButtonGroup, Calendar,
        CalendarEventClick, CalendarView, CALL_RECORD, Card, CaretStyle, Carousel, Chart, CHART_SERIES,
        ChartKind, ChartSync, ChatBubble, ChatMessage, ChatView, check_rules, CheckBox, Chip, ChipGroup,
        Clipboard, CloseAction, CodeEditor, ColorEdit, ColorPicker, ComboArrow, ComboBox, CommandPalette,
        Compass, confirm, Conveyor, CropBox, CropSelection, CursorConfine, DataModule, DateEdit, DateTimeEdit,
        Day, days_in_month, Diagnostic, DialogKind, DirectionPad, Divider, DlgKind, docking, DragImage,
        DrawMode, EasingEditor, Edit, ElectricalSymbol, EmergencyStop, emoji_groups, emoji_list, EmojiInfo,
        EmojiPicker, EmptyState, EventCalendar, EventInterceptor, EventRecorder, ExceptionEvent, Expander,
        Fan, field, FileDropZone, FileInput, find_matches, FlowLayout, FlowMeter, fold, Form, format_time,
        FormPosition, FpsMeter, Frame, FrameTimer, FrameView, FullScreenMode, Gamepad, GATT_CHARACTERISTIC,
        Gauge, GraphCable, GraphNode, GraphSelection, GridLayout, GroupBox, HEADER_COLUMN, hotkey_text,
        HotkeyEdit, IconPosition, IconView, image_list, IMAGE_LIST_ITEM, IMAGE_SIZE, ImageList, ImageView,
        industrial, InnerLoading, InterceptScope, Joystick, KeepAwake, KeyboardCapture, Kinder, Knob, Label, Layout,
        LegendPosition, LevelMeter, LightSensor, LineDirection, list_header, LIST_VIEW_ITEM, ListBox,
        ListHeader, ListItem, ListOrientation, ListView, LoadingIndicator, Location, LockScreen, looks,
        LowerPanel, MainMenu, Manometer, MAP_MARKER, MAP_POINT, MapPoint, MapView, markup_plain, markup_runs,
        markup_spans, MarkupSpan, MarkupTable, MediaCard, Memo, MenuItem, message_dlg, MessageDialog,
        MessageStatus, MotionSensor, Motor, name_color, NavigationDrawer, NavigationRail, Network, Nfc,
        NFC_RECORD, NodeGraph, Notification, NOTIFICATION_BUTTON, notify, NotifyAction, NumberBox, OpenDialog,
        OptionGroup, OptionKind, OtpInput, PageControl, Pagination, paint_glass, paint_spinner, PaintCanvas,
        PaintTool, PAIRED_DEVICE, PALETTE_SELECTION, PaletteSelection, Panel, Parallax, parse_events,
        parse_filter, parse_hex, parse_shortcut, parse_time, PERMISSION_ANSWER, PERMISSION_REASON,
        PERMISSION_RESULT, PermissionAnswer, PermissionResult, Permissions, Phone, Picture, Pipe, PointStyle,
        Popover, PopoverPlacement, PopupEdit, PopupMenu, Port, Presence, ProgressBar, ProgressCircle, project,
        prompt, ProximitySensor, Pump, PUSH_BUTTON, PushButtons, PushNotifications, PushTrigger, quasar,
        RadioButton, RadioGroup, RangeSlider, Rating, RecordButton, recorder, Recording, RECORDING_EXTENSION,
        Rectangle, recycler, register, RenderSettings, RICH_EDITOR_TOOLBAR, RichBlock, RichBlockKind,
        RichEditor, RichFormat, Ringtone, RowSource, SaveDialog, Screen, ScreenItem, ScreenLayout, scroll_bar,
        ScrollBarMode, ScrollBox, search, SeekBar, SegmentedControl, SelectionMode, SelectorSwitch, Series,
        SeriesFill, SeriesKind, set_dialog_icon, set_prompt_required, SevenSegment, ShadowEditor, shapes,
        SHARED_CONTENT, SharedContent, show_message, SideSheet, SignalLight, SignalState, Silo, Sizer,
        SkeletonLoader, SlideItem, SmoothScroll, SnackAction, Snackbar, snackbar, SpeedDial, SpinnerKind,
        SplitButton, Splitter, StatusBar, Stepper, Suggestion, SwipeAction, Switch, Symbol, SyncEvent,
        SyncKind, syntax, SystemBars, Tab, TabControl, TabDrop, TabItem, TableSeparator, TabMove, TabPosition,
        TabStrip, TankLevel, Taskbar, TEXT_EFFECT, textmate, textmate_theme, Thermometer, TimeEdit, Timeline,
        TimePicker, Timer, TimingEditor, TitleAlign, TitleBar, toast, ToastKind, ToastStack, ToolBar, TOPMOST_LOST, TopmostLost,
        TrackBar, trader, TrayIcon, TREE_NODE, TreeLeafLayout, TreeNodeRects, TreeView, unproject, UpDown,
        Uploader, UrlScheme, validate_form, ValueFormat, Valve, ValveState, Vibration, VirtualList,
        WakeAppAction, Waveform, WheelPicker, window_state_commands, WindowCorners, WindowSettings,
        WindowSettingsScope, WindowShape, WindowState, Wire,
    };
    pub use aylis_core::*;
    // Do `aylis_core::*`, as que também são registros do acesso tipado (o mesmo motivo): a
    // importação privada esconde o nome do `*` público (`aylis::core::JumpTask` e
    // `aylis::core::LockKeys` seguem valendo).
    #[allow(unused_imports, hidden_glob_reexports)]
    use aylis_core::{JumpTask, LockKeys, ThumbButton};
    #[cfg(feature = "data")]
    pub use aylis_data::{
        ApplyMode, ApplyResult, Change, ChangeKind, ClientDataSet, DataSet, DataSource, Date, DateTime, Decimal, Delta, Field, FieldKind, FieldType,
        Rounding, RowBatch, Time, Value as DataValue, data_set, view,
        FieldValue, GetTextRequest, MasterChange, ServerQuery, SetTextRequest,
    };
    #[cfg(feature = "dbcontrols")]
    pub use aylis_dbcontrols::{BandKind, Column, ColumnKind, ColumnLayout, DBChart, DBImage, DataGrid, DataNavigator, DataRepeater, GridColumn, HorizontalWheel, NavButton};
    #[cfg(feature = "media")]
    pub use aylis_media::{AudioSpectrum, CameraQuality, MediaInfo, MediaPlayer, MediaTagsInfo, MediaSession, Playlist, RepeatMode, SpectrumStyle, VideoPlayer};
    #[cfg(feature = "vector")]
    pub use aylis_vector::{Lottie, Svg};
    #[cfg(feature = "markdown")]
    pub use aylis_markdown::Markdown;
    #[cfg(feature = "docview")]
    pub use aylis_docview::DocViewer;
    #[cfg(feature = "webview")]
    pub use aylis_webview::WebView;
    #[cfg(feature = "3d")]
    pub use aylis_3d::Model3D;
    #[cfg(feature = "mcp")]
    pub use aylis_remote::mcp::{McpPrompt, McpResource, McpResult, McpServer, McpTool};
    #[cfg(feature = "http")]
    pub use aylis_core::providers::{HttpRequest, HttpResponse};
    #[cfg(feature = "http")]
    pub use aylis_providers::net::{HttpClient, HttpResult};
    #[cfg(feature = "websocket")]
    pub use aylis_providers::net::{WebSocketClient, WebSocketClose};

    pub use crate::Application;
}

/// Uma função por classe que pode aparecer num form, que registra só ela
/// (`aylis::classes::Button(&mut ui)`). O registro gerado pelo `build.rs` do projeto
/// ([`aylis_build::CLASSES`]) chama só as das classes usadas: como as outras nunca são
/// citadas, o linker deixa o código delas de fora.
#[allow(non_snake_case)]
pub mod classes {
    use aylis_core::Ui;

    macro_rules! classes {
        ($( $(#[$m:meta])* $name:ident: $ty:ty => $category:literal ),* $(,)?) => {
            $(
                $(#[$m])*
                #[doc = concat!("Registra a classe `", stringify!($name), "`.")]
                pub fn $name(ui: &mut Ui) {
                    ui.register_class::<$ty>($category);
                }
            )*
        };
    }

    classes! {
        Form: aylis_controls::Form => "Standard",
        Label: aylis_controls::Label => "Standard",
        Button: aylis_controls::Button => "Standard",
        Edit: aylis_controls::Edit => "Standard",
        CheckBox: aylis_controls::CheckBox => "Standard",
        RadioButton: aylis_controls::RadioButton => "Standard",
        Switch: aylis_controls::Switch => "Standard",
        ProgressBar: aylis_controls::ProgressBar => "Standard",
        Panel: aylis_controls::Panel => "Standard",
        Layout: aylis_controls::Layout => "Layouts",
        Frame: aylis_controls::Frame => "Layouts",
        Rectangle: aylis_controls::Rectangle => "Shapes",
        Ellipse: aylis_controls::shapes::Ellipse => "Shapes",
        Circle: aylis_controls::shapes::Circle => "Shapes",
        Line: aylis_controls::shapes::Line => "Shapes",
        Polygon: aylis_controls::shapes::Polygon => "Shapes",
        Star: aylis_controls::shapes::Star => "Shapes",
        Pie: aylis_controls::shapes::Pie => "Shapes",
        Arc: aylis_controls::shapes::Arc => "Shapes",
        Path: aylis_controls::shapes::PathShape => "Shapes",
        DataModule: aylis_controls::DataModule => "Standard",
        VirtualList: aylis_controls::VirtualList => "Additional",
        ListHeader: aylis_controls::ListHeader => "Additional",
        ImageList: aylis_controls::ImageList => "Additional",
        KpiCard: aylis_controls::bi::KpiCard => "BI",
        PivotTable: aylis_controls::bi::PivotTable => "BI",
        Slicer: aylis_controls::bi::Slicer => "BI",
        ChoroplethMap: aylis_controls::bi::ChoroplethMap => "BI",
        CandleChart: aylis_controls::trader::CandleChart => "Trader",
        OrderBook: aylis_controls::trader::OrderBook => "Trader",
        DepthChart: aylis_controls::trader::DepthChart => "Trader",
        TickerTape: aylis_controls::trader::TickerTape => "Trader",
        Watchlist: aylis_controls::trader::Watchlist => "Trader",
        TimeAndSales: aylis_controls::trader::TimeAndSales => "Trader",
        ComboBox: aylis_controls::ComboBox => "Standard",
        ScrollBox: aylis_controls::ScrollBox => "Layouts",
        Splitter: aylis_controls::Splitter => "Layouts",
        Image: aylis_controls::ImageView => "Standard",
        MainMenu: aylis_controls::MainMenu => "Menus",
        PopupMenu: aylis_controls::PopupMenu => "Menus",
        MenuItem: aylis_controls::MenuItem => "Menus",
        Memo: aylis_controls::Memo => "Standard",
        CodeEditor: aylis_controls::CodeEditor => "Standard",
        TabStrip: aylis_controls::TabStrip => "Layouts",
        NumberBox: aylis_controls::NumberBox => "Standard",
        TrackBar: aylis_controls::TrackBar => "Standard",
        ListBox: aylis_controls::ListBox => "Standard",
        RadioGroup: aylis_controls::RadioGroup => "Standard",
        GroupBox: aylis_controls::GroupBox => "Standard",
        DateEdit: aylis_controls::DateEdit => "Additional",
        TimeEdit: aylis_controls::TimeEdit => "Additional",
        DateTimeEdit: aylis_controls::DateTimeEdit => "Additional",
        Popover: aylis_controls::Popover => "Menus",
        Calendar: aylis_controls::Calendar => "Additional",
        ListView: aylis_controls::ListView => "Additional",
        TreeView: aylis_controls::TreeView => "Additional",
        Chart: aylis_controls::Chart => "Additional",
        ChartSync: aylis_controls::ChartSync => "Additional",
        TabControl: aylis_controls::TabControl => "Layouts",
        TabItem: aylis_controls::TabItem => "Layouts",
        Expander: aylis_controls::Expander => "Layouts",
        StatusBar: aylis_controls::StatusBar => "Layouts",
        ToolBar: aylis_controls::ToolBar => "Layouts",
        Timer: aylis_controls::Timer => "System",
        FrameTimer: aylis_controls::FrameTimer => "System",
        DragImage: aylis_controls::DragImage => "System",
        ApplicationEvents: aylis_controls::ApplicationEvents => "System",
        EventInterceptor: aylis_controls::EventInterceptor => "System",
        RenderSettings: aylis_controls::RenderSettings => "System",
        WindowSettings: aylis_controls::WindowSettings => "System",
        BusyWatcher: aylis_controls::BusyWatcher => "System",
        OpenDialog: aylis_controls::OpenDialog => "System",
        SaveDialog: aylis_controls::SaveDialog => "System",
        Clipboard: aylis_controls::Clipboard => "System",
        Permissions: aylis_controls::Permissions => "System",
        Biometrics: aylis_controls::Biometrics => "System",
        AppLink: aylis_controls::AppLink => "System",
        UrlScheme: aylis_controls::UrlScheme => "System",
        KeyboardCapture: aylis_controls::KeyboardCapture => "System",
        SystemBars: aylis_controls::SystemBars => "Device",
        Location: aylis_controls::Location => "Device",
        Battery: aylis_controls::Battery => "Device",
        Network: aylis_controls::Network => "Device",
        MotionSensor: aylis_controls::MotionSensor => "Device",
        Compass: aylis_controls::Compass => "Device",
        ProximitySensor: aylis_controls::ProximitySensor => "Device",
        LightSensor: aylis_controls::LightSensor => "Device",
        Vibration: aylis_controls::Vibration => "Device",
        Gamepad: aylis_controls::Gamepad => "Device",
        Screen: aylis_controls::Screen => "Device",
        BackgroundService: aylis_controls::BackgroundService => "Device",
        Phone: aylis_controls::Phone => "Device",
        LockScreen: aylis_controls::LockScreen => "Device",
        Nfc: aylis_controls::Nfc => "Device",
        Bluetooth: aylis_controls::Bluetooth => "Device",
        PushNotifications: aylis_controls::PushNotifications => "Device",
        PushTrigger: aylis_controls::PushTrigger => "Device",
        WakeAppAction: aylis_controls::WakeAppAction => "Device",
        NotifyAction: aylis_controls::NotifyAction => "Device",
        AnimatedText: aylis_controls::AnimatedText => "Standard",
        FloatAnimation: aylis_core::FloatAnimation => "Animations",
        ColorAnimation: aylis_core::ColorAnimation => "Animations",
        Icon: aylis_controls::IconView => "Standard",
        Badge: aylis_controls::Badge => "Additional",
        Banner: aylis_controls::Banner => "Additional",
        Avatar: aylis_controls::Avatar => "Additional",
        SegmentedControl: aylis_controls::SegmentedControl => "Standard",
        ActivityIndicator: aylis_controls::ActivityIndicator => "Additional",
        Breadcrumb: aylis_controls::Breadcrumb => "Additional",
        HotkeyEdit: aylis_controls::HotkeyEdit => "Additional",
        DirectionPad: aylis_controls::DirectionPad => "Additional",
        Joystick: aylis_controls::Joystick => "Additional",
        LevelMeter: aylis_controls::LevelMeter => "Media",
        RecordButton: aylis_controls::RecordButton => "Media",
        ScreenLayout: aylis_controls::ScreenLayout => "Media",
        MediaCard: aylis_controls::MediaCard => "Media",
        FrameView: aylis_controls::FrameView => "Media",
        CropBox: aylis_controls::CropBox => "Media",
        SeekBar: aylis_controls::SeekBar => "Media",
        PaintCanvas: aylis_controls::PaintCanvas => "Additional",
        NodeGraph: aylis_controls::NodeGraph => "Additional",
        ChatView: aylis_controls::ChatView => "Additional",
        Waveform: aylis_controls::Waveform => "Media",
        Card: aylis_controls::Card => "Quasar",
        Knob: aylis_controls::Knob => "Quasar",
        InnerLoading: aylis_controls::InnerLoading => "Quasar",
        MarkupTable: aylis_controls::MarkupTable => "Quasar",
        SlideItem: aylis_controls::SlideItem => "Quasar",
        Parallax: aylis_controls::Parallax => "Quasar",
        Uploader: aylis_controls::Uploader => "Quasar",
        PopupEdit: aylis_controls::PopupEdit => "Quasar",
        OptionGroup: aylis_controls::OptionGroup => "Quasar",
        AjaxBar: aylis_controls::AjaxBar => "Quasar",
        ChatBubble: aylis_controls::ChatBubble => "Quasar",
        RichEditor: aylis_controls::RichEditor => "Quasar",
        GridLayout: aylis_controls::GridLayout => "Layouts",
        FlowLayout: aylis_controls::FlowLayout => "Layouts",
        Divider: aylis_controls::Divider => "Layouts",
        NavigationDrawer: aylis_controls::NavigationDrawer => "Layouts",
        BottomNavigation: aylis_controls::BottomNavigation => "Layouts",
        BottomSheet: aylis_controls::BottomSheet => "Layouts",
        Carousel: aylis_controls::Carousel => "Layouts",
        Chip: aylis_controls::Chip => "Standard",
        ChipGroup: aylis_controls::ChipGroup => "Standard",
        Rating: aylis_controls::Rating => "Standard",
        RangeSlider: aylis_controls::RangeSlider => "Standard",
        OtpInput: aylis_controls::OtpInput => "Additional",
        SplitButton: aylis_controls::SplitButton => "Standard",
        AutoComplete: aylis_controls::AutoComplete => "Standard",
        ColorPicker: aylis_controls::ColorPicker => "Additional",
        ColorEdit: aylis_controls::ColorEdit => "Additional",
        FileInput: aylis_controls::FileInput => "Additional",
        FileDropZone: aylis_controls::FileDropZone => "Additional",
        Snackbar: aylis_controls::Snackbar => "Additional",
        SkeletonLoader: aylis_controls::SkeletonLoader => "Additional",
        ProgressCircle: aylis_controls::ProgressCircle => "Standard",
        EmptyState: aylis_controls::EmptyState => "Additional",
        ToastStack: aylis_controls::ToastStack => "Additional",
        Pagination: aylis_controls::Pagination => "Additional",
        Stepper: aylis_controls::Stepper => "Additional",
        Timeline: aylis_controls::Timeline => "Additional",
        SpeedDial: aylis_controls::SpeedDial => "Additional",
        AvatarGroup: aylis_controls::AvatarGroup => "Additional",
        EventCalendar: aylis_controls::EventCalendar => "Additional",
        CommandPalette: aylis_controls::CommandPalette => "Additional",
        TimePicker: aylis_controls::TimePicker => "Additional",
        AppBar: aylis_controls::AppBar => "Layouts",
        NavigationRail: aylis_controls::NavigationRail => "Layouts",
        SideSheet: aylis_controls::SideSheet => "Layouts",
        PageControl: aylis_controls::PageControl => "Additional",
        EmojiPicker: aylis_controls::EmojiPicker => "Additional",
        MapView: aylis_controls::MapView => "Additional",
        FpsMeter: aylis_controls::FpsMeter => "Additional",
        TitleBar: aylis_controls::TitleBar => "Additional",
        TrayIcon: aylis_controls::TrayIcon => "System",
        Notification: aylis_controls::Notification => "System",
        MessageDialog: aylis_controls::MessageDialog => "System",
        EventRecorder: aylis_controls::EventRecorder => "System",
        Analytics: aylis_controls::Analytics => "System",
        Taskbar: aylis_controls::Taskbar => "System",
        UpDown: aylis_controls::UpDown => "Standard",
        WheelPicker: aylis_controls::WheelPicker => "Additional",
        ButtonGroup: aylis_controls::ButtonGroup => "Standard",
        Gauge: aylis_controls::Gauge => "Additional",
        ActivityRings: aylis_controls::ActivityRings => "Additional",
        LoadingIndicator: aylis_controls::LoadingIndicator => "Additional",
        AngleDial: aylis_controls::AngleDial => "Additional",
        ShadowEditor: aylis_controls::ShadowEditor => "Additional",
        EasingEditor: aylis_controls::EasingEditor => "Additional",
        TimingEditor: aylis_controls::TimingEditor => "Additional",
        #[cfg(feature = "data")] ClientDataSet: aylis_data::ClientDataSet => "Data",
        #[cfg(feature = "data")] Field: aylis_data::Field => "Data",
        #[cfg(feature = "data")] DataSource: aylis_data::DataSource => "Data",
        #[cfg(feature = "dbcontrols")] DataNavigator: aylis_dbcontrols::DataNavigator => "Data Controls",
        #[cfg(feature = "dbcontrols")] DataGrid: aylis_dbcontrols::DataGrid => "Data Controls",
        #[cfg(feature = "dbcontrols")] Column: aylis_dbcontrols::Column => "Data Controls",
        #[cfg(feature = "dbcontrols")] DBImage: aylis_dbcontrols::DBImage => "Data Controls",
        #[cfg(feature = "dbcontrols")] DataRepeater: aylis_dbcontrols::DataRepeater => "Data Controls",
        #[cfg(feature = "dbcontrols")] DBChart: aylis_dbcontrols::DBChart => "Data Controls",
        #[cfg(feature = "dbcontrols")] Report: aylis_dbcontrols::Report => "Reports",
        #[cfg(feature = "dbcontrols")] ReportBand: aylis_dbcontrols::ReportBand => "Reports",
        #[cfg(feature = "dbcontrols")] ReportPreview: aylis_dbcontrols::ReportPreview => "Reports",
        #[cfg(feature = "providers")] Barcode: aylis_providers::Barcode => "Reports",
        #[cfg(feature = "providers")] Export: aylis_providers::Export => "Data",
        #[cfg(feature = "providers")] AylisBarcodeProvider: aylis_providers::AylisBarcodeProvider => "Providers",
        #[cfg(feature = "providers")] CsvProvider: aylis_providers::CsvProvider => "Providers",
        #[cfg(feature = "providers")] XlsxProvider: aylis_providers::XlsxProvider => "Providers",
        #[cfg(feature = "pdf")] KrillaPdfProvider: aylis_providers::KrillaPdfProvider => "Providers",
        #[cfg(feature = "providers")] TileServerProvider: aylis_providers::TileServerProvider => "Providers",
        #[cfg(feature = "providers")] OleObject: aylis_providers::OleObject => "System",
        #[cfg(feature = "providers")] WmiQuery: aylis_providers::WmiQuery => "System",
        #[cfg(feature = "media")] MediaPlayer: aylis_media::MediaPlayer => "Media",
        #[cfg(feature = "media")] VideoPlayer: aylis_media::VideoPlayer => "Media",
        #[cfg(feature = "media")] Microphone: aylis_media::Microphone => "Media",
        #[cfg(feature = "media")] AudioOutput: aylis_media::AudioOutput => "Media",
        #[cfg(feature = "media")] Sound: aylis_media::Sound => "Media",
        #[cfg(feature = "media")] AudioMixer: aylis_media::AudioMixer => "Media",
        #[cfg(feature = "media")] CameraView: aylis_media::CameraView => "Media",
        #[cfg(feature = "media")] MediaInfo: aylis_media::MediaInfo => "Media",
        #[cfg(feature = "media")] SymphoniaTagsProvider: aylis_media::SymphoniaTagsProvider => "Providers",
        #[cfg(feature = "media")] Playlist: aylis_media::Playlist => "Media",
        #[cfg(feature = "media")] AudioSpectrum: aylis_media::AudioSpectrum => "Media",
        #[cfg(feature = "media")] MediaSession: aylis_media::MediaSession => "Media",
        #[cfg(feature = "media-tags")] LoftyTagsProvider: aylis_providers::LoftyTagsProvider => "Providers",
        #[cfg(feature = "vector")] Lottie: aylis_vector::Lottie => "Media",
        #[cfg(feature = "vector")] Svg: aylis_vector::Svg => "Additional",
        #[cfg(feature = "markdown")] Markdown: aylis_markdown::Markdown => "Additional",
        #[cfg(feature = "docview")] DocViewer: aylis_docview::DocViewer => "Additional",
        #[cfg(feature = "webview")] WebView: aylis_webview::WebView => "Additional",
        Manometer: aylis_controls::Manometer => "Industrial",
        Thermometer: aylis_controls::Thermometer => "Industrial",
        TankLevel: aylis_controls::TankLevel => "Industrial",
        FlowMeter: aylis_controls::FlowMeter => "Industrial",
        SevenSegment: aylis_controls::SevenSegment => "Industrial",
        SignalLight: aylis_controls::SignalLight => "Industrial",
        Valve: aylis_controls::Valve => "Industrial",
        Motor: aylis_controls::Motor => "Industrial",
        Pump: aylis_controls::Pump => "Industrial",
        Conveyor: aylis_controls::Conveyor => "Industrial",
        Fan: aylis_controls::Fan => "Industrial",
        Silo: aylis_controls::Silo => "Industrial",
        EmergencyStop: aylis_controls::EmergencyStop => "Industrial",
        SelectorSwitch: aylis_controls::SelectorSwitch => "Industrial",
        PushButtons: aylis_controls::PushButtons => "Industrial",
        Pipe: aylis_controls::Pipe => "Industrial",
        Wire: aylis_controls::Wire => "Electrical",
        #[cfg(feature = "3d")] Model3D: aylis_3d::Model3D => "3D",
        #[cfg(feature = "mcp")] McpServer: aylis_remote::mcp::McpServer => "System",
        #[cfg(feature = "mcp")] McpTool: aylis_remote::mcp::McpTool => "System",
        #[cfg(feature = "mcp")] McpResource: aylis_remote::mcp::McpResource => "System",
        #[cfg(feature = "mcp")] McpPrompt: aylis_remote::mcp::McpPrompt => "System",
        #[cfg(feature = "http")] HttpClient: aylis_providers::net::HttpClient => "System",
        #[cfg(feature = "http")] AylisHttpProvider: aylis_providers::net::AylisHttpProvider => "Providers",
        #[cfg(feature = "websocket")] WebSocketClient: aylis_providers::net::WebSocketClient => "System",
        #[cfg(feature = "websocket")] AylisWebSocketProvider: aylis_providers::net::AylisWebSocketProvider => "Providers",
    }

    /// O símbolo elétrico, com os subtipos da paleta.
    pub fn ElectricalSymbol(ui: &mut Ui) {
        ui.register_class::<aylis_controls::ElectricalSymbol>("Electrical");
        for (name, caption) in aylis_controls::industrial::SYMBOLS {
            ui.register_variant("ElectricalSymbol", name, caption, vec![("symbol", aylis_core::PropValue::Enum(name.into()))]);
        }
    }
}

use aylis_core::{ComponentId, Ui};

/// O componente com conteúdo de exemplo (a galeria, as imagens e as demonstrações da documentação).
pub mod samples;
/// O modo sem plataforma (testes e fotos sem janela).
#[cfg(not(target_arch = "wasm32"))]
pub mod headless;

mod time_zone;

#[cfg(feature = "dev")]
mod dev;
#[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
#[doc(hidden)]
pub mod dev_memory;

/// Repinta o painel de memória a cada segundo enquanto ele estiver à vista (os números mudam
/// sem nada na tela mudar).
#[cfg(feature = "dev")]
fn refresh_memory_panel(ui: &mut Ui) {
    if ui.memory_panel_visible() {
        ui.invalidate();
    }
    ui.set_timeout(1000, refresh_memory_panel);
}

/// Liga o canal de desenvolvimento num `Ui` montado à mão (testes e o `aylis-host --dev`; o
/// app normal faz isso sozinho em `development`, com o alvo embutido).
#[cfg(feature = "dev")]
#[doc(hidden)]
pub fn dev_connect(ui: &Ui, target: aylis_devlink::Target) {
    dev::connect(ui, target, Default::default());
}
#[cfg(feature = "remote")]
mod backend;
#[cfg(feature = "remote")]
pub use backend::connect_backend;
#[cfg(all(feature = "remote", not(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))))]
pub use backend::{BackendProcess, spawn_backend};
#[cfg(all(target_arch = "wasm32", feature = "remote"))]
mod web_remote;
#[cfg(all(target_arch = "wasm32", feature = "remote"))]
pub use web_remote::run_remote_web;

pub type Error = Box<dyn std::error::Error>;

static APP_VERSION: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());

/// Solta a trava do `single_instance` do projeto antes de o app relançar a si mesmo (a
/// atualização que troca o exe e abre o novo): a nova abertura passa a ser a primeira. O
/// mesmo que `ui.release_single_instance()`, sem o `Ui` à mão.
pub fn release_single_instance() {
    aylis_platform_winit::release_single_instance();
}

/// Liga ou desliga a abertura do app quando o usuário entra no sistema (a chave `Run` do
/// usuário no Windows, o autostart no Linux, o agente de login no macOS; no celular e no
/// navegador, o erro). A decisão é do app: o Aylis só grava. O mesmo que
/// `ui.set_start_with_login(on)`.
pub fn start_with_login(on: bool) -> Result<(), String> {
    aylis_platform_winit::set_start_with_login(on)
}

/// O app abre sozinho com o login (o registro aponta para este exe).
pub fn starts_with_login() -> bool {
    aylis_platform_winit::start_with_login()
}

/// A versão do app em execução: a que a publicação gravou (a `publish.version` do projeto ou,
/// com `publish.version_from_git_tag`, a da última tag do git no build), a mesma dos dados do
/// `.exe`, do Android e dos pacotes do Linux. Vale em qualquer handler, sem o [`Application`]
/// (que também tem, em `project.version()`). Vazia antes de o app abrir o projeto.
pub fn app_version() -> String {
    APP_VERSION.read().map(|v| v.clone()).unwrap_or_default()
}

fn set_app_version(version: &str) {
    if let Ok(mut v) = APP_VERSION.write() {
        *v = version.to_string();
    }
    // A versão que o `McpServer` mostra ao agente quando o `server_version` está vazio.
    #[cfg(feature = "mcp")]
    aylis_remote::mcp::stdio::set_app_version(version);
}

/// O projeto embutido pelo `build.rs` (`aylis_build::project()`), mais a pasta dele e o
/// registro só das classes que os forms usam, para [`Application::bundled`]. Use dentro
/// do crate do projeto.
#[macro_export]
macro_rules! include_project {
    () => {
        $crate::EmbeddedProject {
            dir: env!("CARGO_MANIFEST_DIR"),
            bundle: include_bytes!(concat!(env!("OUT_DIR"), "/project.aylispkg")),
            register: include!(concat!(env!("OUT_DIR"), "/aylis_classes.rs")),
            dev: include_str!(concat!(env!("OUT_DIR"), "/aylis_dev.json")),
        }
    };
}

/// Ver [`include_project!`].
#[derive(Clone, Copy)]
pub struct EmbeddedProject {
    /// A pasta do projeto na máquina que compilou.
    pub dir: &'static str,
    /// O pacote com o `.aylisproj`, `forms/`, `themes/` e `assets/`.
    pub bundle: &'static [u8],
    /// Registra as classes que os forms usam (gerado pelo `build.rs`). Para registrar
    /// todas, passe [`register_all`].
    ///
    /// É uma função, e não um `Option` com `register_all` como reserva: só de citar o
    /// `register_all` todos os componentes entrariam no executável.
    pub register: fn(&mut Ui),
    /// O canal de desenvolvimento (o endereço da IDE e as chaves, em JSON), embutido só
    /// quando a IDE compila com a feature `dev`; vazio no app publicado.
    pub dev: &'static str,
}

/// O aplicativo de um projeto: abre o `.aylisproj`,
/// registra todas as classes, aplica tema e idioma, liga os arquivos do projeto (assets) e
/// cria os forms marcados como "criar no início". Os outros forms são criados com
/// `ui.create_form("frmClientes")`. O `on_create` dos DataModules criados no início chega no
/// [`run`](Self::run), depois de o app registrar os handlers (`generated::register`).
pub struct Application {
    pub ui: Ui,
    pub project: aylis_project::Project,
    /// O form principal (fechá-lo encerra).
    pub main_form: ComponentId,
    /// Os arquivos vêm da pasta do projeto (desenvolvimento), não do pacote embutido.
    from_disk: bool,
    /// A pasta do `placements.json` ([`Application::set_placements_dir`]).
    placements_dir: Option<std::path::PathBuf>,
    /// Os DataModules criados no início, com o `on_create` esperando o [`run`](Self::run).
    created: Vec<ComponentId>,
}

impl Application {
    /// Abre o projeto de uma pasta (o caminho do `.aylisproj`).
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let project = aylis_project::Project::open(path)?;
        let files = aylis_project::Files::Dir(project.dir());
        let mut app = Self::start(project, files)?;
        app.from_disk = true;
        Ok(app)
    }

    /// Abre o projeto embutido no executável ([`include_project!`]).
    ///
    /// Compilado pela IDE para desenvolver (a feature `dev`), e com a pasta do projeto ainda
    /// no lugar, lê da pasta: um form ou imagem alterados aparecem sem recompilar. Sem `dev`
    /// (o app publicado), sempre o pacote embutido.
    pub fn bundled(embedded: EmbeddedProject) -> Result<Self, Error> {
        #[cfg(feature = "dev")]
        dev::set_embedded(embedded.dev);
        let bundle = aylis_project::Bundle::parse(embedded.bundle)?;
        let mut project = bundle.project()?;
        // O crate fica na pasta da linguagem (`<projeto>/rust`); o projeto, na de cima.
        let crate_dir = std::path::Path::new(embedded.dir);
        let dir = aylis_build::project_root(crate_dir).unwrap_or_else(|| crate_dir.to_path_buf());
        let from_disk = cfg!(feature = "dev") && aylis_build::find_project(&dir).is_some();
        if from_disk {
            if let Some(path) = aylis_build::find_project(&dir) {
                // A versão gravada no build (a da tag do git, se o projeto pede), não a da pasta.
                let mut app = Self::open(path)?;
                app.project.publish.version = project.publish.version.clone();
                set_app_version(project.version());
                return Ok(app);
            }
        }
        project.path = dir.join(&project.path);
        Self::start_with(project, aylis_project::Files::Bundle(bundle), embedded.register)
    }

    /// Lendo da pasta (desenvolvimento) registra todas as classes: com o hot reload, um
    /// form pode ganhar um componente novo sem recompilar.
    fn start(project: aylis_project::Project, files: aylis_project::Files) -> Result<Self, Error> {
        Self::start_with(project, files, register_all)
    }

    fn start_with(project: aylis_project::Project, files: aylis_project::Files, register: fn(&mut Ui)) -> Result<Self, Error> {
        let mut ui = Ui::new();
        register(&mut ui);
        set_app_version(project.version());
        // A ponte stdio do MCP (`--mcp-stdio`) só com a opção do projeto.
        #[cfg(feature = "mcp")]
        aylis_remote::mcp::stdio::set_enabled(project.publish.mcp_stdio);
        // O PDF do relatório sem provedor ligado e da impressão fora do Windows (o núcleo não
        // escreve PDF): o registro enxuto pode não ter chamado o `aylis_providers::register`.
        #[cfg(feature = "pdf")]
        if ui.services().pdf.is_none() {
            ui.services_mut().pdf = Some(Box::new(aylis_providers::KrillaPdf));
        }
        time_zone::install(&project.time_zone);
        // Quem mede e desenha o texto nesta plataforma (o `text` do projeto, item 10).
        ui.set_text_mode(match project.text().current() {
            aylis_project::TextChoice::Native => aylis_core::TextMode::Native,
            aylis_project::TextChoice::Layout => aylis_core::TextMode::Layout,
            aylis_project::TextChoice::Core => aylis_core::TextMode::Core,
        });
        // O texto medido pelo renderizador do projeto já na montagem dos forms (T01).
        aylis_platform_winit::install_text_service(&mut ui, project.render().current().map_or(aylis_platform_winit::Renderer::Auto, renderer_for));
        let main_form = aylis_project::install_with(&mut ui, &project, files)?;
        let created = aylis_project::data_modules(&ui);
        // O principal é o do projeto, mesmo que outro form (uma abertura) apareça antes dele (o
        // relatório do Tabernáculo, J20).
        ui.set_main_form(main_form);
        #[cfg(feature = "data")]
        aylis_data::open_active(&mut ui);
        #[cfg(feature = "media")]
        aylis_media::autoplay(&mut ui);
        Ok(Self { ui, project, main_form, from_disk: false, placements_dir: None, created })
    }

    /// Recursos de desenvolvimento, só no app compilado pela IDE com a feature `dev` (a
    /// publicação nunca liga; sem variáveis de ambiente nem parâmetros):
    ///
    /// - **hot reload**: lendo da pasta do projeto, um form ou tema salvo aparece no app
    ///   sem reiniciar;
    /// - **canal de desenvolvimento**: o app conecta na IDE no endereço embutido (cifrado,
    ///   veja `dev.rs` e `aylis-devlink`): o Inspetor do app, o hot reload empurrado pela IDE
    ///   (no celular e no navegador) e o rastro de eventos, que a IDE liga pelo canal.
    ///
    /// - **painel de memória**: no canto do form principal, a memória do processo, o heap e
    ///   as imagens no cache (`aylis_core::memory_panel`); começa escondido, e Ctrl+Shift+M mostra e esconde. Fica
    ///   de fora das capturas (`AYLIS_SNAPSHOT`).
    ///
    /// Chamado por [`run`](Self::run).
    pub fn development(&mut self) {
        #[cfg(feature = "dev")]
        {
            if aylis_platform_winit::setting("AYLIS_SNAPSHOT").is_none() {
                // Disponível e escondido: aparece com o Ctrl+Shift+M (aberto, cobria o canto do app
                // em toda execução pela IDE; pedido do autor em 07/11).
                self.ui.set_memory_panel(Some(false));
                #[cfg(not(target_arch = "wasm32"))]
                aylis_core::memory_panel::set_heap_counter(|| crate::dev_memory::live_bytes().max(0) as usize);
                refresh_memory_panel(&mut self.ui);
            }
            if self.from_disk {
                aylis_project::hot_reload(&mut self.ui, &self.project);
            }
            if let Some(target) = dev::target() {
                dev::connect(&self.ui, target, self.project.theme_mix.clone());
            }
        }
    }

    /// Troca o idioma com o app aberto: formatos, traduções (`locales/<idioma>.json`) e
    /// os textos dos forms.
    pub fn set_language(ui: &mut Ui, language: &str) {
        aylis_project::i18n::set_language(ui, language);
    }

    /// Opções da janela tiradas do projeto: o ícone e o renderizador desta plataforma
    /// (no app com `dev`, o diagnóstico `AYLIS_RENDERER` ainda escolhe outro, para testes).
    pub fn options(&self) -> aylis_platform_winit::Options {
        let icon = Some(self.project.publish.icon.as_str())
            .filter(|i| !i.is_empty())
            .and_then(|i| self.ui.services().resources.as_ref().and_then(|r| r.read(i)));
        let renderer = if cfg!(feature = "dev") && aylis_platform_winit::setting("AYLIS_RENDERER").is_some() {
            aylis_platform_winit::Renderer::Auto
        } else {
            self.project.render().current().map_or(aylis_platform_winit::Renderer::Auto, renderer_for)
        };
        aylis_platform_winit::Options {
            icon,
            renderer,
            gpu: aylis_core::gpu::GpuChoice::parse(&self.project.gpu),
            present: aylis_core::gpu::PresentMode::parse(&self.project.present),
            placements_dir: self.placements_dir.clone(),
            app_id: self.project.linux_package_id(),
        }
    }

    /// A pasta onde o app guarda o lugar das janelas (`remember_placement`); sem ela, a pasta de
    /// dados do usuário com o nome do executável (o relatório do Tabernáculo, P09).
    pub fn set_placements_dir(&mut self, dir: impl Into<std::path::PathBuf>) {
        self.placements_dir = Some(dir.into());
    }

    /// Abre o form principal e roda até ele fechar (como `Application.Run`).
    pub fn run(mut self) -> Result<(), Error> {
        // A ponte stdio do MCP (`app --mcp-stdio [servidor]`, com a opção `mcp_stdio` do projeto):
        // o Claude Desktop inicia o executável assim, e ele só repassa as mensagens ao servidor
        // do app que já está aberto, sem janela, sem o `on_create` e sem a instância única. Sem a
        // opção, o argumento é ignorado.
        #[cfg(all(feature = "mcp", not(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))))]
        if self.project.publish.mcp_stdio {
            let args: Vec<String> = std::env::args().collect();
            if let Some(i) = args.iter().position(|a| a == "--mcp-stdio") {
                let server = args.get(i + 1).filter(|a| !a.starts_with("--")).cloned();
                return aylis_remote::mcp::stdio::run_bridge(server.as_deref()).map_err(Into::into);
            }
        }
        // Depurar na IDE: o depurador se anexa a este processo antes de tudo (veja
        // `aylis_platform_winit::wait_for_debugger`).
        #[cfg(all(feature = "dev", windows))]
        aylis_platform_winit::wait_for_debugger();
        // O `on_create` dos DataModules do início, que esperou o app registrar os handlers.
        aylis_project::fire_created(&mut self.ui, &std::mem::take(&mut self.created));
        // Roteiro de teste (só no app com `dev`, na linha que o teste escreve no stdin): sem
        // janela, como o aylis-host faz nas outras linguagens (veja `aylis_remote::script`).
        #[cfg(all(feature = "dev", not(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))))]
        if let Some((script, output)) = aylis_remote::script::start_line().as_ref().and_then(aylis_remote::script::from_start) {
            return aylis_remote::script::run_in_process(&mut self.ui, self.main_form, &script, output.as_deref()).map_err(Into::into);
        }
        // Linux: `--install-shortcut` e `--remove-shortcut` gravam (ou tiram) o atalho no menu
        // deste usuário apontando para este executável, sem abrir a janela: quem recebe a pasta
        // publicada sem instalador ganha o app no menu (decidido em 21/11).
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        if let Some(flag) = std::env::args().nth(1).filter(|a| a == "--install-shortcut" || a == "--remove-shortcut") {
            return linux_shortcut(&self.project, &self.ui, flag == "--install-shortcut").map_err(Into::into);
        }
        // Quem é o app: o nome do início com o sistema (`start_with_login`).
        aylis_platform_winit::set_app_identity(&self.project.linux_package_id(), self.project.app_name());
        // Uma instância só (`single_instance` do projeto): a segunda abertura entrega os
        // argumentos à primeira, que vem para a frente, e sai sem abrir janela (ou sai calada,
        // com `single_instance_second = quiet`). A trava com o nome do projeto, se ele deu um.
        #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
        if !aylis_platform_winit::claim_single_instance_with(
            aylis_core::SingleInstance::parse(&self.project.single_instance),
            &self.project.linux_package_id(),
            &self.project.single_instance_name,
            aylis_core::SecondInstance::parse(&self.project.single_instance_second),
        ) {
            return Ok(());
        }
        self.development();
        let options = self.options();
        aylis_platform_winit::run_with(self.ui, self.main_form, options)
    }
}

/// O atalho do app no menu deste usuário (`~/.local/share/applications/<pacote>.desktop`) e o
/// ícone do projeto (`~/.local/share/icons/hicolor`), apontando para este executável; `false`
/// tira os dois. O mesmo `.desktop` do pacote (o nome, as categorias, o `StartupWMClass`).
#[cfg(all(target_os = "linux", not(target_os = "android")))]
fn linux_shortcut(project: &aylis_project::Project, ui: &Ui, install: bool) -> Result<(), String> {
    use std::path::PathBuf;
    let home = std::env::var_os("HOME").map(PathBuf::from).ok_or("sem HOME: não sei onde fica o menu deste usuário")?;
    let share = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| home.join(".local/share"));
    let id = project.linux_package_id();
    let desktop = share.join("applications").join(format!("{id}.desktop"));
    let icon_file = project.publish.icon.trim();
    let svg = icon_file.to_ascii_lowercase().ends_with(".svg");
    let icon = share.join("icons/hicolor").join(if svg { "scalable/apps".to_string() } else { "256x256/apps".to_string() }).join(format!("{id}.{}", if svg { "svg" } else { "png" }));
    if !install {
        let _ = std::fs::remove_file(&icon);
        std::fs::remove_file(&desktop).map_err(|e| format!("{}: {e}", desktop.display()))?;
        aylis_core::stderr_line!("atalho tirado: {}", desktop.display());
        return Ok(());
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let has_icon = !icon_file.is_empty()
        && ui.services().resources.as_ref().and_then(|r| r.read(icon_file)).is_some_and(|bytes| {
            icon.parent().is_some_and(|d| std::fs::create_dir_all(d).is_ok()) && std::fs::write(&icon, bytes).is_ok()
        });
    let linux = &project.publish.linux;
    let categories: String = if linux.categories.is_empty() { "Utility;".into() } else { linux.categories.iter().map(|c| format!("{};", c.trim().trim_end_matches(';'))).collect() };
    let mime: String = linux.mime_types.iter().map(|m| format!("{};", m.trim())).collect();
    let text = format!(
        "[Desktop Entry]\nType=Application\nName={}\nExec=\"{}\"{}\n{}Terminal=false\nCategories={categories}\nStartupWMClass={id}\n{}",
        project.app_name(),
        exe.display().to_string().replace('"', "\\\""),
        if mime.is_empty() { "" } else { " %F" },
        if has_icon { format!("Icon={id}\n") } else { String::new() },
        if mime.is_empty() { String::new() } else { format!("MimeType={mime}\n") },
    );
    if let Some(dir) = desktop.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(&desktop, text).map_err(|e| format!("{}: {e}", desktop.display()))?;
    let _ = std::process::Command::new("update-desktop-database").arg("-q").arg(share.join("applications")).status();
    aylis_core::stderr_line!("atalho no menu: {}", desktop.display());
    Ok(())
}

/// Android: chame em `android_main` (o código gerado faz isso). Erros vão para o logcat
/// (`adb logcat -s RustStdoutStderr`).
#[cfg(target_os = "android")]
pub fn run_android(android: aylis_platform_winit::android::AndroidApp, make: impl FnOnce() -> Result<Application, Error>) {
    // O logcat e a pasta temporária antes do código do app (que pode usar `temp_dir`).
    aylis_platform_winit::android::prepare(&android);
    let result = make().and_then(|mut app| {
        // O `on_create` dos DataModules do início (veja o `run`).
        aylis_project::fire_created(&mut app.ui, &std::mem::take(&mut app.created));
        app.development();
        let options = app.options();
        aylis_platform_winit::android::run_with(android, app.ui, app.main_form, options)
    });
    if let Err(e) = result {
        aylis_core::stderr_line!("aylis: {e}");
    }
}

/// Navegador: chame na função exportada que a página chama (o código gerado faz isso).
/// Erros vão para o console do navegador.
#[cfg(target_arch = "wasm32")]
pub fn run_web(make: impl FnOnce() -> Result<Application, Error>) {
    // No navegador, o wasm não continua depois de um pânico (não há como pegá-lo, como no
    // desktop): a página mostra o erro e o botão de recarregar, em vez de parar calada.
    std::panic::set_hook(Box::new(|info| {
        web_log(&info.to_string());
        web_crash_screen();
    }));
    if let Err(e) = make().and_then(Application::run) {
        web_log(&format!("aylis: {e}"));
    }
}

#[cfg(target_arch = "wasm32")]
fn web_log(text: &str) {
    web_sys::console::error_1(&text.into());
}

/// A tela "o aplicativo encontrou um erro", por cima de tudo, com o botão de recarregar.
#[cfg(target_arch = "wasm32")]
fn web_crash_screen() {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
    let (Some(body), Ok(panel)) = (document.body(), document.create_element("div")) else { return };
    let message = aylis_core::messages::t("O aplicativo encontrou um erro e precisa ser recarregado.");
    let reload = aylis_core::messages::t("Recarregar");
    let _ = panel.set_attribute(
        "style",
        "position:fixed;inset:0;z-index:2147483647;display:flex;flex-direction:column;align-items:center;\
         justify-content:center;gap:16px;background:rgba(0,0,0,.72);color:#fff;font:16px system-ui,sans-serif",
    );
    panel.set_inner_html(&format!(
        "<div>{message}</div><button onclick=\"location.reload()\" style=\"font:inherit;padding:8px 20px;\
         border:0;border-radius:6px;background:#2563eb;color:#fff;cursor:pointer\">{reload}</button>"
    ));
    let _ = body.append_child(&panel);
}

/// O renderizador da plataforma para a escolha do projeto. Uma escolha que não foi
/// compilada (feature desligada no `Cargo.toml`) vira o automático.
fn renderer_for(choice: aylis_project::RenderChoice) -> aylis_platform_winit::Renderer {
    use aylis_platform_winit::Renderer;
    use aylis_project::RenderChoice;
    match choice {
        RenderChoice::D2d if cfg!(feature = "d2d") && cfg!(windows) => Renderer::D2d,
        RenderChoice::Gpu if cfg!(feature = "skia-gpu") => Renderer::Gpu,
        RenderChoice::Cpu if cfg!(feature = "skia-cpu") => Renderer::Cpu,
        _ => Renderer::Auto,
    }
}

/// Registra todas as classes padrão (controles, dados, controles de dados, mídia) e os
/// subtipos, para os layouts e o protocolo criarem componentes pelo nome. O app publicado
/// usa o registro enxuto gerado pelo `build.rs` (veja [`classes`]).
pub fn register_all(ui: &mut Ui) {
    aylis_controls::register(ui);
    #[cfg(feature = "data")]
    aylis_data::register(ui);
    #[cfg(feature = "dbcontrols")]
    aylis_dbcontrols::register(ui);
    #[cfg(feature = "providers")]
    aylis_providers::register(ui);
    #[cfg(feature = "media")]
    aylis_media::register(ui);
    #[cfg(feature = "vector")]
    aylis_vector::register(ui);
    #[cfg(feature = "markdown")]
    aylis_markdown::register(ui);
    #[cfg(feature = "docview")]
    aylis_docview::register(ui);
    #[cfg(feature = "webview")]
    aylis_webview::register(ui);
    #[cfg(feature = "3d")]
    aylis_3d::register(ui);
    #[cfg(feature = "mcp")]
    aylis_remote::mcp::register(ui);
}
