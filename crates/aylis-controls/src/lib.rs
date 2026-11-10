//! # aylis-controls
//!
//! Controles padrão do aylis:
//!
//! | Controle | Equivalente no Delphi |
//! |---|---|
//! | [`Form`] | `TForm` |
//! | [`Layout`] | `TLayout` |
//! | [`Rectangle`] | `TRectangle` |
//! | [`Panel`] | `TPanel` |
//! | [`Label`] | `TLabel` |
//! | [`Button`] | `TButton` |
//! | [`CheckBox`] | `TCheckBox` |
//! | [`RadioButton`] | `TRadioButton` |
//! | [`Switch`] | `TSwitch` |
//! | [`ProgressBar`] | `TProgressBar` |
//! | [`Edit`] | `TEdit` |
//! | [`ComboBox`] | `TComboBox` |
//! | [`ScrollBox`] | `TVertScrollBox` |
//! | [`Splitter`] | `TSplitter` |
//! | [`ImageView`] (classe `Image`) | `TImage` |
//! | [`MainMenu`], [`PopupMenu`], [`MenuItem`] | `TMainMenu`, `TPopupMenu`, `TMenuItem` |
//! | [`Memo`] | `TMemo` |
//! | [`CodeEditor`] | `TSynEdit` (o editor de código) |
//! | [`NumberBox`] | `TNumberBox` / `TSpinBox` |
//! | [`TrackBar`] | `TTrackBar` |
//! | [`DateEdit`], [`TimeEdit`], [`Calendar`] | `TDateEdit`, `TTimeEdit`, `TCalendar` |
//! | [`ListBox`], [`RadioGroup`] | `TListBox`, `TRadioGroup` |
//! | [`ListView`], [`TreeView`] | `TListView`, `TTreeView` (virtualizados) |
//! | [`TabControl`], [`TabItem`] | `TTabControl`, `TTabItem` |
//! | [`GroupBox`], [`Expander`] | `TGroupBox`, `TExpander` |
//! | [`StatusBar`], [`ToolBar`] | `TStatusBar`, `TToolBar` |
//! | [`Timer`] | `TTimer` |
//! | [`DragImage`] | `TDragObject.GetDragImages` / `TCustomImageList` do arrasto: a imagem junto do cursor |
//! | [`OpenDialog`], [`SaveDialog`] | `TOpenDialog`, `TSaveDialog` (assíncronos; desktop, navegador, Android e iOS: leia e grave com `aylis_core::files`) |
//! | [`Permissions`], [`Biometrics`] | consultar e pedir permissões; digital, rosto, Windows Hello |
//! | [`AppLink`] | abrir endereços e outros apps, compartilhar, receber links e o que foi compartilhado |
//! | [`AnimatedText`] | texto com efeitos de entrada, saída e contínuos (apresentações) |
//! | [`MessageDialog`], [`message_dlg`], [`show_message`], [`toast`] | `TTaskDialog`, `MessageDlg`, `ShowMessage`, toast do Android |
//! | [`Chart`] | `TChart` (linhas, barras, área, pizza) |
//!
//! Toda a aparência vem do tema (estilo com o nome da classe); propriedades da instância
//! como `fill` ou `color`, quando definidas, ganham do tema.

//! Os controles moram em crates por família, que compilam em paralelo; este crate junta
//! todos (a mesma API de antes) e registra as classes:
//!
//! | Crate | O que tem |
//! |---|---|
//! | `aylis-controls-base` | forms, botões, edição, listas, menus, abas, sistema, Quasar, código |
//! | `aylis-controls-charts` | [`Chart`], [`bi`], [`trader`], [`NodeGraph`], [`MapView`] |
//! | `aylis-controls-industrial` | [`industrial`] (telas de supervisão) |
//! | `aylis-controls-device` | sensores, permissões, biometria, [`AppLink`] |
//! | `aylis-controls-editors` | [`AngleDial`] e os outros editores visuais, [`PaintCanvas`], [`AnimatedText`] |

pub use aylis_controls_base::*;
pub use aylis_controls_charts::{
    Chart, ChartKind, GraphCable, GraphNode, GraphSelection, MapView, NodeGraph, PointStyle, Series, SeriesFill, SeriesKind, bi, parse_time, project, trader,
    unproject,
};
pub use aylis_controls_device::{
    AppLink, BackgroundService, Battery, Biometrics, Bluetooth, Nfc, parse_hex, Compass, Gamepad, LightSensor, Location, LockScreen, MotionSensor, Network, Permissions, Phone, ProximitySensor,
    PushNotifications, PushTrigger, WakeAppAction, NotifyAction, Ringtone,
    Screen, Vibration,
};
pub use aylis_controls_editors::{AngleDial, AnimatedText, Bitmap, EasingEditor, PaintCanvas, PaintTool, ShadowEditor, TimingEditor};
pub use aylis_controls_industrial::{
    self as industrial, Conveyor, ElectricalSymbol, EmergencyStop, Fan, FlowMeter, Manometer, Motor, Pipe, Pump, PushButtons, SelectorSwitch, SevenSegment,
    SignalLight, Silo, TankLevel, Thermometer, Valve, Wire,
};

use aylis_core::{ColorAnimation, FloatAnimation, Ui};

/// Registra todos os controles deste crate (e as animações), para poderem ser criados
/// por nome.
pub fn register(ui: &mut Ui) {
    // Um erro num handler ou num componente aparece na caixa de mensagem do aylis, e o app
    // segue (como o `Application.HandleException` do Delphi). Em desenvolvimento, com onde.
    ui.set_exception_display(|ui, e| {
        let text = if cfg!(debug_assertions) { e.to_string() } else { e.message.clone() };
        message_dlg(ui, &text, DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
    });
    // O segundo pedido de fechar enquanto o backend não responde ao `on_close_query`: a saída
    // de emergência, como o "não está respondendo" do sistema (encerra o backend junto).
    ui.set_unresponsive_handler(|ui, _form| {
        let text = aylis_core::messages::t("O programa não está respondendo. Encerrar o programa? As alterações não salvas serão perdidas.");
        message_dlg(ui, &text, DlgKind::Warning, &[aylis_core::ModalResult::Yes, aylis_core::ModalResult::No], |ui, result| {
            if result == aylis_core::ModalResult::Yes {
                ui.terminate();
            }
        });
    });
    ui.register_class::<Form>("Standard");
    // Industriais e elétricos (telas de supervisão).
    industrial::register(ui);
    ui.register_class::<DataModule>("Standard");
    ui.register_class::<Label>("Standard");
    ui.register_class::<Button>("Standard");
    ui.register_class::<Edit>("Standard");
    ui.register_class::<CheckBox>("Standard");
    ui.register_class::<RadioButton>("Standard");
    ui.register_class::<Switch>("Standard");
    ui.register_class::<ProgressBar>("Standard");
    ui.register_class::<Panel>("Standard");
    ui.register_class::<Layout>("Layouts");
    ui.register_class::<Frame>("Layouts");
    ui.register_class::<Rectangle>("Shapes");
    ui.register_class::<shapes::Ellipse>("Shapes");
    ui.register_class::<shapes::Circle>("Shapes");
    ui.register_class::<shapes::Line>("Shapes");
    ui.register_class::<shapes::Polygon>("Shapes");
    ui.register_class::<shapes::Star>("Shapes");
    ui.register_class::<shapes::Pie>("Shapes");
    ui.register_class::<shapes::Arc>("Shapes");
    ui.register_class::<shapes::PathShape>("Shapes");
    ui.register_class::<recycler::VirtualList>("Additional");
    ui.register_class::<bi::KpiCard>("BI");
    ui.register_class::<bi::PivotTable>("BI");
    ui.register_class::<bi::Slicer>("BI");
    ui.register_class::<bi::ChoroplethMap>("BI");
    ui.register_class::<trader::CandleChart>("Trader");
    ui.register_class::<trader::OrderBook>("Trader");
    ui.register_class::<trader::DepthChart>("Trader");
    ui.register_class::<trader::TickerTape>("Trader");
    ui.register_class::<trader::Watchlist>("Trader");
    ui.register_class::<trader::TimeAndSales>("Trader");
    ui.register_class::<ComboBox>("Standard");
    ui.register_class::<ScrollBox>("Layouts");
    ui.register_class::<Splitter>("Layouts");
    ui.register_class::<ImageView>("Standard");
    ui.register_class::<MainMenu>("Menus");
    ui.register_class::<PopupMenu>("Menus");
    ui.register_class::<MenuItem>("Menus");
    ui.register_class::<Memo>("Standard");
    ui.register_class::<CodeEditor>("Standard");
    ui.register_class::<NumberBox>("Standard");
    ui.register_class::<TrackBar>("Standard");
    ui.register_class::<ListBox>("Standard");
    ui.register_class::<RadioGroup>("Standard");
    ui.register_class::<GroupBox>("Standard");
    ui.register_class::<DateEdit>("Additional");
    ui.register_class::<TimeEdit>("Additional");
    ui.register_class::<Calendar>("Additional");
    ui.register_class::<ListView>("Additional");
    ui.register_class::<TreeView>("Additional");
    ui.register_class::<Chart>("Additional");
    ui.register_class::<TabControl>("Layouts");
    ui.register_class::<TabStrip>("Layouts");
    ui.register_class::<TabItem>("Layouts");
    ui.register_class::<Expander>("Layouts");
    ui.register_class::<StatusBar>("Layouts");
    ui.register_class::<ToolBar>("Layouts");
    ui.register_class::<Timer>("System");
    ui.register_class::<DragImage>("System");
    ui.register_class::<ApplicationEvents>("System");
    ui.register_class::<EventInterceptor>("System");
    ui.register_class::<BusyWatcher>("System");
    ui.register_class::<OpenDialog>("System");
    ui.register_class::<Permissions>("System");
    ui.register_class::<Biometrics>("System");
    ui.register_class::<AppLink>("System");
    ui.register_class::<Location>("Device");
    ui.register_class::<Battery>("Device");
    ui.register_class::<Network>("Device");
    ui.register_class::<MotionSensor>("Device");
    ui.register_class::<Compass>("Device");
    ui.register_class::<ProximitySensor>("Device");
    ui.register_class::<LightSensor>("Device");
    ui.register_class::<Vibration>("Device");
    ui.register_class::<Gamepad>("Device");
    ui.register_class::<Screen>("Device");
    ui.register_class::<BackgroundService>("Device");
    ui.register_class::<Phone>("Device");
    ui.register_class::<LockScreen>("Device");
    ui.register_class::<Nfc>("Device");
    ui.register_class::<Bluetooth>("Device");
    ui.register_class::<PushNotifications>("Device");
    ui.register_class::<PushTrigger>("Device");
    ui.register_class::<WakeAppAction>("Device");
    ui.register_class::<NotifyAction>("Device");
    ui.register_class::<AnimatedText>("Standard");
    ui.register_class::<SaveDialog>("System");
    ui.register_class::<FloatAnimation>("Animations");
    ui.register_class::<ColorAnimation>("Animations");
    ui.register_class::<IconView>("Standard");
    ui.register_class::<Badge>("Additional");
    ui.register_class::<Banner>("Additional");
    ui.register_class::<Avatar>("Additional");
    ui.register_class::<SegmentedControl>("Standard");
    ui.register_class::<ActivityIndicator>("Additional");
    ui.register_class::<Breadcrumb>("Additional");
    ui.register_class::<HotkeyEdit>("Additional");
    ui.register_class::<DirectionPad>("Additional");
    ui.register_class::<Joystick>("Additional");
    ui.register_class::<LevelMeter>("Media");
    ui.register_class::<RecordButton>("Media");
    ui.register_class::<ScreenLayout>("Media");
    ui.register_class::<MediaCard>("Media");
    ui.register_class::<FrameView>("Media");
    ui.register_class::<CropBox>("Media");
    ui.register_class::<GridLayout>("Layouts");
    ui.register_class::<FlowLayout>("Layouts");
    ui.register_class::<Divider>("Layouts");
    ui.register_class::<NavigationDrawer>("Layouts");
    ui.register_class::<BottomNavigation>("Layouts");
    ui.register_class::<BottomSheet>("Layouts");
    ui.register_class::<Carousel>("Layouts");
    ui.register_class::<Chip>("Standard");
    ui.register_class::<ChipGroup>("Standard");
    ui.register_class::<Rating>("Standard");
    ui.register_class::<RangeSlider>("Standard");
    ui.register_class::<OtpInput>("Additional");
    ui.register_class::<SplitButton>("Standard");
    ui.register_class::<AutoComplete>("Standard");
    ui.register_class::<ColorPicker>("Additional");
    ui.register_class::<ColorEdit>("Additional");
    ui.register_class::<FileInput>("Additional");
    ui.register_class::<FileDropZone>("Additional");
    ui.register_class::<Snackbar>("Additional");
    ui.register_class::<SkeletonLoader>("Additional");
    ui.register_class::<ProgressCircle>("Standard");
    ui.register_class::<EmptyState>("Additional");
    ui.register_class::<Pagination>("Additional");
    ui.register_class::<Stepper>("Additional");
    ui.register_class::<Timeline>("Additional");
    ui.register_class::<SpeedDial>("Additional");
    ui.register_class::<AvatarGroup>("Additional");
    ui.register_class::<EventCalendar>("Additional");
    ui.register_class::<CommandPalette>("Additional");
    ui.register_class::<TimePicker>("Additional");
    ui.register_class::<AppBar>("Layouts");
    ui.register_class::<NavigationRail>("Layouts");
    ui.register_class::<SideSheet>("Layouts");
    ui.register_class::<PageControl>("Additional");
    ui.register_class::<EmojiPicker>("Additional");
    ui.register_class::<MapView>("Additional");
    ui.register_class::<FpsMeter>("Additional");
    ui.register_class::<TitleBar>("Additional");
    ui.register_class::<TrayIcon>("System");
    ui.register_class::<Notification>("System");
    ui.register_class::<MessageDialog>("System");
    ui.register_class::<EventRecorder>("System");
    ui.register_class::<Analytics>("System");
    ui.register_class::<Taskbar>("System");
    ui.register_class::<UpDown>("Standard");
    ui.register_class::<WheelPicker>("Additional");
    ui.register_class::<ButtonGroup>("Standard");
    ui.register_class::<Gauge>("Additional");
    ui.register_class::<ActivityRings>("Additional");
    ui.register_class::<LoadingIndicator>("Additional");
    ui.register_class::<AngleDial>("Additional");
    ui.register_class::<ShadowEditor>("Additional");
    ui.register_class::<EasingEditor>("Additional");
    ui.register_class::<TimingEditor>("Additional");
    ui.register_class::<SeekBar>("Media");
    ui.register_class::<PaintCanvas>("Additional");
    ui.register_class::<NodeGraph>("Additional");
    ui.register_class::<ChatView>("Additional");
    ui.register_class::<Waveform>("Media");
    // Os do Quasar.
    ui.register_class::<Card>("Quasar");
    ui.register_class::<Knob>("Quasar");
    ui.register_class::<InnerLoading>("Quasar");
    ui.register_class::<MarkupTable>("Quasar");
    ui.register_class::<SlideItem>("Quasar");
    ui.register_class::<Parallax>("Quasar");
    ui.register_class::<Uploader>("Quasar");
    ui.register_class::<PopupEdit>("Quasar");
    ui.register_class::<OptionGroup>("Quasar");
    ui.register_class::<AjaxBar>("Quasar");
    ui.register_class::<ChatBubble>("Quasar");
    ui.register_class::<RichEditor>("Quasar");
    register_variants(ui);
}

/// Os subtipos que o aylis traz: aparecem na paleta debaixo da classe e criam o
/// componente com a variante de estilo e valores iniciais próprios. O app registra os
/// seus com [`Ui::register_variant`] (ou só criando o estilo `Classe.nome` no tema).
fn register_variants(ui: &mut Ui) {
    use aylis_core::PropValue as V;
    let s = |t: &str| V::String(t.into());
    let f = |v: f64| V::Float(v);
    let e = |t: &str| V::Enum(t.into());
    // Button
    ui.register_variant("Button", "secondary", "Secundário", vec![]);
    ui.register_variant("Button", "outline", "Contorno", vec![]);
    ui.register_variant("Button", "flat", "Plano (só texto)", vec![]);
    ui.register_variant("Button", "danger", "Perigo", vec![]);
    ui.register_variant("Button", "success", "Sucesso", vec![]);
    ui.register_variant("Button", "link", "Link", vec![]);
    ui.register_variant("Button", "pill", "Pílula (marcável)", vec![("checkable", V::Bool(true))]);
    ui.register_variant("Button", "icon", "Ícone", vec![("text", s("")), ("icon", s("settings")), ("width", f(36.0)), ("height", f(36.0))]);
    ui.register_variant(
        "Button",
        "fab",
        "Flutuante (FAB)",
        vec![("text", s("")), ("icon", s("plus")), ("width", f(56.0)), ("height", f(56.0))],
    );
    // Edit
    ui.register_variant(
        "Edit",
        "search",
        "Busca",
        vec![("icon", s("search")), ("clear_button", V::Bool(true)), ("placeholder", s("Buscar"))],
    );
    ui.register_variant("Edit", "flat", "Sem borda", vec![]);
    // Panel
    ui.register_variant("Panel", "card", "Cartão", vec![]);
    ui.register_variant("Panel", "flat", "Plano", vec![]);
    ui.register_variant("Panel", "sidebar", "Barra lateral", vec![]);
    ui.register_variant("Panel", "overlay", "Flutuante (sombra)", vec![]);
    // Label
    ui.register_variant("Label", "title", "Título", vec![]);
    ui.register_variant("Label", "heading", "Cabeçalho", vec![]);
    ui.register_variant("Label", "muted", "Discreto", vec![]);
    ui.register_variant("Label", "caption", "Legenda", vec![]);
    ui.register_variant("Label", "mono", "Monoespaçado", vec![]);
    ui.register_variant("Label", "link", "Link", vec![]);
    ui.register_variant("Label", "error", "Erro", vec![]);
    // ProgressBar
    ui.register_variant("ProgressBar", "thin", "Fina", vec![("height", f(4.0))]);
    ui.register_variant("ProgressBar", "success", "Sucesso", vec![]);
    ui.register_variant("ProgressBar", "danger", "Perigo", vec![]);
    // Switch / CheckBox
    ui.register_variant("Switch", "success", "Verde", vec![]);
    ui.register_variant("CheckBox", "round", "Redondo", vec![]);
    // Icon
    ui.register_variant("Icon", "muted", "Discreto", vec![]);
    // Badge e Banner
    for (tone, caption) in [("primary", "Destaque"), ("success", "Sucesso"), ("warning", "Atenção"), ("danger", "Perigo"), ("info", "Informação")] {
        ui.register_variant("Badge", tone, caption, vec![]);
    }
    ui.register_variant("Badge", "outline", "Contorno", vec![]);
    ui.register_variant("Badge", "live", "Ao vivo", vec![("text", s("AO VIVO")), ("dot", V::Bool(true))]);
    for (tone, caption, title) in [("success", "Sucesso", "Pronto"), ("warning", "Atenção", "Atenção"), ("danger", "Erro", "Erro")] {
        ui.register_variant("Banner", tone, caption, vec![("title", s(title))]);
    }
    // Avatar
    ui.register_variant("Avatar", "square", "Quadrado", vec![]);
    ui.register_variant("Avatar", "online", "Com presença", vec![("presence", e("online"))]);
    // LevelMeter
    ui.register_variant(
        "LevelMeter",
        "vertical",
        "Vertical em gomos",
        vec![("vertical", V::Bool(true)), ("segments", V::Int(16)), ("width", f(12.0)), ("height", f(120.0))],
    );
    ui.register_variant(
        "LevelMeter",
        "stereo",
        "Estéreo com degradê",
        vec![("stereo", V::Bool(true)), ("gradient", V::Bool(true)), ("show_peak", V::Bool(false)), ("height", f(7.0))],
    );
    // Componentes do Vuetify
    ui.register_variant("Label", "kbd", "Tecla (Kbd)", vec![("text", s("Ctrl"))]);
    ui.register_variant("Chip", "outline", "Contorno", vec![]);
    for (tone, caption) in [("success", "Sucesso"), ("warning", "Atenção"), ("danger", "Perigo")] {
        ui.register_variant("Chip", tone, caption, vec![]);
    }
    ui.register_variant("Chip", "filter", "Filtro (marcável)", vec![("checkable", V::Bool(true))]);
    ui.register_variant("Chip", "input", "Entrada (com ×)", vec![("closable", V::Bool(true))]);
    ui.register_variant("ProgressCircle", "spinner", "Girando", vec![("indeterminate", V::Bool(true))]);
    ui.register_variant("Stepper", "vertical", "Vertical", vec![("vertical", V::Bool(true)), ("width", f(220.0)), ("height", f(220.0))]);
    ui.register_variant("NavigationDrawer", "rail", "Só ícones", vec![("rail", V::Bool(true)), ("open", V::Bool(false))]);
    ui.register_variant("Chart", "sparkline", "Sparkline", vec![("kind", e("sparkline")), ("legend", V::Bool(false)), ("width", f(160.0)), ("height", f(48.0))]);
    ui.register_variant("Chart", "heatmap", "Mapa de calor", vec![("kind", e("heatmap"))]);
    // Com os dados que cada um pede (os do Chart, meses e duas séries, não dizem nada neles).
    ui.register_variant(
        "Chart",
        "sankey",
        "Sankey",
        vec![
            ("kind", e("sankey")),
            ("labels", s("Salário>Receita;Vendas>Receita;Receita>Moradia;Receita>Mercado;Receita>Poupança;Poupança>Reserva;Poupança>Ações")),
            ("series", s("Fluxo=5000,2000,2200,1800,3000,1800,1200")),
            ("width", f(420.0)),
            ("height", f(240.0)),
        ],
    );
    ui.register_variant("Chart", "waffle", "Waffle", vec![("kind", e("waffle")), ("labels", s("Norte;Sul;Leste;Oeste")), ("series", s("Vendas=42,27,19,12")), ("height", f(240.0))]);
    ui.register_variant("Calendar", "range", "Intervalo", vec![("range", V::Bool(true))]);
    ui.register_variant("Calendar", "month", "Mês", vec![("month_mode", V::Bool(true)), ("height", f(220.0))]);
    ui.register_variant("ScrollBox", "refresh", "Puxar para atualizar", vec![("pull_to_refresh", V::Bool(true))]);
    ui.register_variant("ScrollBox", "horizontal", "Horizontal", vec![("horizontal", V::Bool(true)), ("height", f(80.0))]);
    ui.register_variant("ChipGroup", "slide", "Uma linha (rola)", vec![("wrap", V::Bool(false))]);
    ui.register_variant("EventCalendar", "week", "Semana", vec![("view", e("week"))]);
    ui.register_variant("EventCalendar", "day", "Dia", vec![("view", e("day")), ("width", f(320.0))]);
    // Material 3 e iOS 26
    ui.register_variant("Button", "elevated", "Elevado (M3)", vec![]);
    ui.register_variant("Button", "tonal", "Tonal (M3)", vec![]);
    ui.register_variant("Button", "text", "Texto (M3)", vec![]);
    ui.register_variant("Button", "fab_extended", "FAB estendido", vec![("text", s("Escrever")), ("icon", s("edit")), ("width", f(140.0)), ("height", f(56.0))]);
    for (name, caption) in [("icon_filled", "Ícone cheio"), ("icon_tonal", "Ícone tonal"), ("icon_outlined", "Ícone com contorno")] {
        ui.register_variant("Button", name, caption, vec![("text", s("")), ("icon", s("heart")), ("width", f(40.0)), ("height", f(40.0))]);
    }
    ui.register_variant("Button", "glass", "Vidro (iOS 26)", vec![]);
    ui.register_variant("Button", "glass_prominent", "Vidro em destaque (iOS 26)", vec![]);
    ui.register_variant("Panel", "filled", "Cartão preenchido", vec![]);
    ui.register_variant("Panel", "outlined", "Cartão com contorno", vec![]);
    ui.register_variant("Panel", "glass", "Vidro (Liquid Glass)", vec![]);
    ui.register_variant("Edit", "filled", "Preenchido (M3)", vec![]);
    ui.register_variant("Edit", "outlined", "Com contorno (M3)", vec![]);
    ui.register_variant("ToolBar", "floating", "Flutuante", vec![("height", f(56.0))]);
    ui.register_variant("AppBar", "center", "Título no centro", vec![("size", e("center"))]);
    ui.register_variant("AppBar", "medium", "Média", vec![("size", e("medium")), ("height", f(112.0))]);
    ui.register_variant("AppBar", "large", "Grande", vec![("size", e("large")), ("height", f(152.0))]);
    ui.register_variant("ButtonGroup", "standard", "Separados", vec![("connected", V::Bool(false))]);
    ui.register_variant("LoadingIndicator", "contained", "Com fundo", vec![("contained", V::Bool(true))]);
    ui.register_variant("Gauge", "plain", "Uma cor", vec![("gradient", V::Bool(false))]);
    ui.register_variant("BottomSheet", "detents", "Com alturas (iOS)", vec![("detents", s("0.4;0.9"))]);
    // Os do Quasar (os estilos do QBtn ficam com a cara do Quasar no tema "Quasar 2").
    for (name, caption) in [
        ("push", "Push (Quasar)"),
        ("unelevated", "Sem elevação (Quasar)"),
        ("rounded", "Arredondado (Quasar)"),
        ("glossy", "Brilhante (Quasar)"),
        ("accent", "Accent (Quasar)"),
        ("positive", "Positive (Quasar)"),
        ("negative", "Negative (Quasar)"),
    ] {
        ui.register_variant("Button", name, caption, vec![]);
    }
    ui.register_variant("Button", "round", "Redondo (Quasar)", vec![("text", s("")), ("icon", s("plus")), ("width", f(42.0)), ("height", f(42.0))]);
    ui.register_variant("Card", "flat", "Plano", vec![]);
    ui.register_variant("Card", "bordered", "Com borda", vec![]);
    ui.register_variant("OptionGroup", "checkbox", "Caixas", vec![("kind", e("checkbox")), ("selected", s("Médio"))]);
    ui.register_variant("OptionGroup", "toggle", "Interruptores", vec![("kind", e("toggle")), ("selected", s("Médio"))]);
    ui.register_variant("OptionGroup", "inline", "Na mesma linha", vec![("inline", V::Bool(true)), ("width", f(360.0)), ("height", f(40.0))]);
    ui.register_variant("MarkupTable", "dense", "Densa", vec![("dense", V::Bool(true))]);
    ui.register_variant("MarkupTable", "striped", "Listrada", vec![("striped", V::Bool(true)), ("separator", e("none"))]);
    for (kind, caption) in [("dots", "Pontos"), ("bars", "Barras"), ("audio", "Áudio"), ("ios", "iOS"), ("puff", "Ondas"), ("grid", "Grade"), ("tail", "Rastro"), ("hourglass", "Ampulheta"), ("orbit", "Órbita")] {
        ui.register_variant("ActivityIndicator", kind, caption, vec![("kind", e(kind))]);
    }
    ui.register_variant("ChatBubble", "sent", "Enviada", vec![("sent", V::Bool(true)), ("author", s("Eu")), ("text", s("Tudo certo, até amanhã!"))]);
    ui.register_variant("Knob", "percent", "Porcentagem", vec![("suffix", s("%")), ("thickness", f(0.15))]);
    ui.register_variant("Badge", "counter", "Contador (preso)", vec![("text", s("3")), ("width", f(18.0)), ("height", f(18.0))]);
}
