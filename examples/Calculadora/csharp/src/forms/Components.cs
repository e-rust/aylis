// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using System.Globalization;
using System.Linq;
using System.Text.Json.Nodes;
using AylisClient;

namespace App.Forms
{
    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum Align
    {
        None,
        Top,
        Bottom,
        Left,
        Right,
        MostTop,
        MostBottom,
        MostLeft,
        MostRight,
        Client,
        Contents,
        Center,
        VertCenter,
        HorzCenter,
        Horizontal,
        Vertical,
        Fit,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum Anchor
    {
        Left,
        Top,
        Right,
        Bottom,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum BlendMode
    {
        Normal,
        Multiply,
        Screen,
        Overlay,
        Darken,
        Lighten,
        ColorDodge,
        ColorBurn,
        HardLight,
        SoftLight,
        Difference,
        Exclusion,
        Hue,
        Saturation,
        Color,
        Luminosity,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum BorderStyle
    {
        Sizeable,
        Single,
        None,
        Custom,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ButtonStyle
    {
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

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CloseAction
    {
        Hide,
        Free,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CursorConfine
    {
        None,
        Window,
        Monitor,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CursorShape
    {
        Default,
        Pointer,
        Text,
        Wait,
        Crosshair,
        Move,
        NotAllowed,
        ResizeHorizontal,
        ResizeVertical,
        ResizeNeSw,
        ResizeNwSe,
        Grab,
        Grabbing,
        None,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FontWeight
    {
        Default,
        Thin,
        Light,
        Normal,
        Medium,
        Semibold,
        Bold,
        Black,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FormBlendCompositor
    {
        Project,
        Direct2d,
        Direct3d,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FormOrientation
    {
        Project,
        Any,
        Portrait,
        Landscape,
        SensorPortrait,
        SensorLandscape,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FormPosition
    {
        Center,
        ScreenCenter,
        Designed,
        Default,
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FormPresent
    {
        Project,
        Window,
        Composition,
        Composed,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FormStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FullScreenMode
    {
        Borderless,
        Exclusive,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum GridLayoutStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum IconPosition
    {
        Left,
        Right,
        Top,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum KeepAwake
    {
        Off,
        System,
        Display,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum KeyCode
    {
        Unidentified,
        KeyA,
        KeyB,
        KeyC,
        KeyD,
        KeyE,
        KeyF,
        KeyG,
        KeyH,
        KeyI,
        KeyJ,
        KeyK,
        KeyL,
        KeyM,
        KeyN,
        KeyO,
        KeyP,
        KeyQ,
        KeyR,
        KeyS,
        KeyT,
        KeyU,
        KeyV,
        KeyW,
        KeyX,
        KeyY,
        KeyZ,
        Digit0,
        Digit1,
        Digit2,
        Digit3,
        Digit4,
        Digit5,
        Digit6,
        Digit7,
        Digit8,
        Digit9,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        F25,
        F26,
        F27,
        F28,
        F29,
        F30,
        F31,
        F32,
        F33,
        F34,
        F35,
        Lang1,
        Lang2,
        Lang3,
        Lang4,
        Lang5,
        Backquote,
        Backslash,
        BracketLeft,
        BracketRight,
        Comma,
        Equal,
        IntlBackslash,
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

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum LabelStyle
    {
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

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum LayerCaching
    {
        Project,
        On,
        Off,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum LayoutStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ListViewStyle
    {
        Default,
        Flat,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ModalResult
    {
        None,
        Ok,
        Cancel,
        Abort,
        Retry,
        Ignore,
        Yes,
        No,
        Close,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum PanelStyle
    {
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

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum SelfAlign
    {
        Stretch,
        Start,
        Center,
        End,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum TabControlStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum TabItemStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum TabPosition
    {
        Top,
        Bottom,
        Left,
        Right,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum TextAlign
    {
        Leading,
        Center,
        Trailing,
        Justify,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum TextCase
    {
        None,
        Upper,
        Lower,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ThemeName
    {
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

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum Transition
    {
        None,
        Fade,
        SlideLeft,
        SlideRight,
        SlideUp,
        SlideDown,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum Trimming
    {
        None,
        Character,
        Word,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum WindowCorners
    {
        Default,
        Round,
        Small,
        Square,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum WindowShape
    {
        Rectangle,
        Rounded,
        Ellipse,
        Path,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum WindowState
    {
        Normal,
        Maximized,
        Minimized,
        FullScreen,
    }

    /// <summary>Os nomes que a tela entende de cada enum do acesso tipado.</summary>
    internal static class EnumNames
    {
        public static readonly string[] Align = ["none", "top", "bottom", "left", "right", "most_top", "most_bottom", "most_left", "most_right", "client", "contents", "center", "vert_center", "horz_center", "horizontal", "vertical", "fit"];
        public static readonly string[] Anchor = ["left", "top", "right", "bottom"];
        public static readonly string[] BlendMode = ["normal", "multiply", "screen", "overlay", "darken", "lighten", "color_dodge", "color_burn", "hard_light", "soft_light", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];
        public static readonly string[] BorderStyle = ["sizeable", "single", "none", "custom"];
        public static readonly string[] ButtonStyle = ["", "accent", "danger", "elevated", "fab", "fab_extended", "flat", "ghost", "glass", "glass_prominent", "glossy", "icon", "icon_filled", "icon_outlined", "icon_tonal", "info", "link", "negative", "outline", "pill", "positive", "push", "reveal", "round", "rounded", "secondary", "success", "text", "tonal", "unelevated", "warning"];
        public static readonly string[] CloseAction = ["hide", "free"];
        public static readonly string[] CursorConfine = ["none", "window", "monitor"];
        public static readonly string[] CursorShape = ["default", "pointer", "text", "wait", "crosshair", "move", "not_allowed", "resize_horizontal", "resize_vertical", "resize_ne_sw", "resize_nw_se", "grab", "grabbing", "none"];
        public static readonly string[] FontWeight = ["default", "thin", "light", "normal", "medium", "semibold", "bold", "black"];
        public static readonly string[] FormBlendCompositor = ["project", "direct2d", "direct3d"];
        public static readonly string[] FormOrientation = ["project", "any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"];
        public static readonly string[] FormPosition = ["center", "screen_center", "designed", "default", "top_left", "top_right", "bottom_left", "bottom_right"];
        public static readonly string[] FormPresent = ["project", "window", "composition", "composed"];
        public static readonly string[] FormStyle = [""];
        public static readonly string[] FullScreenMode = ["borderless", "exclusive"];
        public static readonly string[] GridLayoutStyle = [""];
        public static readonly string[] IconPosition = ["left", "right", "top"];
        public static readonly string[] KeepAwake = ["off", "system", "display"];
        public static readonly string[] KeyCode = ["Unidentified", "KeyA", "KeyB", "KeyC", "KeyD", "KeyE", "KeyF", "KeyG", "KeyH", "KeyI", "KeyJ", "KeyK", "KeyL", "KeyM", "KeyN", "KeyO", "KeyP", "KeyQ", "KeyR", "KeyS", "KeyT", "KeyU", "KeyV", "KeyW", "KeyX", "KeyY", "KeyZ", "Digit0", "Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", "Numpad6", "Numpad7", "Numpad8", "Numpad9", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24", "F25", "F26", "F27", "F28", "F29", "F30", "F31", "F32", "F33", "F34", "F35", "Lang1", "Lang2", "Lang3", "Lang4", "Lang5", "Backquote", "Backslash", "BracketLeft", "BracketRight", "Comma", "Equal", "IntlBackslash", "IntlRo", "IntlYen", "Minus", "Period", "Quote", "Semicolon", "Slash", "AltLeft", "AltRight", "Backspace", "CapsLock", "ContextMenu", "ControlLeft", "ControlRight", "Enter", "MetaLeft", "MetaRight", "ShiftLeft", "ShiftRight", "Space", "Tab", "Convert", "KanaMode", "NonConvert", "Delete", "End", "Help", "Home", "Insert", "PageDown", "PageUp", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp", "NumLock", "NumpadAdd", "NumpadComma", "NumpadDecimal", "NumpadDivide", "NumpadEnter", "NumpadEqual", "NumpadMultiply", "NumpadSubtract", "Escape", "Fn", "PrintScreen", "ScrollLock", "Pause", "AudioVolumeDown", "AudioVolumeMute", "AudioVolumeUp", "MediaPlayPause", "MediaSelect", "MediaStop", "MediaTrackNext", "MediaTrackPrevious", "BrowserBack", "BrowserFavorites", "BrowserForward", "BrowserHome", "BrowserRefresh", "BrowserSearch", "BrowserStop", "LaunchApp1", "LaunchApp2", "LaunchMail", "Power", "Sleep", "WakeUp", "NumpadBackspace", "NumpadClear", "NumpadClearEntry", "NumpadHash", "NumpadMemoryAdd", "NumpadMemoryClear", "NumpadMemoryRecall", "NumpadMemoryStore", "NumpadMemorySubtract", "NumpadParenLeft", "NumpadParenRight", "NumpadStar", "FnLock", "Hiragana", "Katakana", "Again", "Copy", "Cut", "Find", "Open", "Paste", "Props", "Select", "Undo", "Resume", "Suspend"];
        public static readonly string[] LabelStyle = ["", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title"];
        public static readonly string[] LayerCaching = ["project", "on", "off"];
        public static readonly string[] LayoutStyle = [""];
        public static readonly string[] ListViewStyle = ["", "flat"];
        public static readonly string[] ModalResult = ["none", "ok", "cancel", "abort", "retry", "ignore", "yes", "no", "close"];
        public static readonly string[] PanelStyle = ["", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"];
        public static readonly string[] SelfAlign = ["stretch", "start", "center", "end"];
        public static readonly string[] TabControlStyle = [""];
        public static readonly string[] TabItemStyle = [""];
        public static readonly string[] TabPosition = ["top", "bottom", "left", "right"];
        public static readonly string[] TextAlign = ["leading", "center", "trailing", "justify"];
        public static readonly string[] TextCase = ["none", "upper", "lower"];
        public static readonly string[] ThemeName = ["", "Aylis Light", "Aylis Dark", "Material 3 Light", "Material 3 Dark", "Material 2 Light", "Material 2 Dark", "Android 7 Light", "Android 7 Dark", "iOS 26 Light", "iOS 26 Dark", "iOS 13 Light", "iOS 13 Dark", "iOS 7 Light", "WinUI 3 Light", "WinUI 3 Dark", "Windows 11 Light", "Windows 11 Dark", "Windows 10 Light", "Windows 10 Dark", "Win32", "Windows 7", "Windows Phone Dark", "Windows Phone Light", "Vuetify 3 Light", "Vuetify 3 Dark", "Bootstrap 5 Light", "Bootstrap 5 Dark", "shadcn/ui Light", "shadcn/ui Dark", "Ant Design 5 Light", "Ant Design 5 Dark", "Chakra UI Light", "Chakra UI Dark", "Quasar 2 Light", "Quasar 2 Dark"];
        public static readonly string[] Transition = ["none", "fade", "slide_left", "slide_right", "slide_up", "slide_down"];
        public static readonly string[] Trimming = ["none", "character", "word"];
        public static readonly string[] WindowCorners = ["default", "round", "small", "square"];
        public static readonly string[] WindowShape = ["rectangle", "rounded", "ellipse", "path"];
        public static readonly string[] WindowState = ["normal", "maximized", "minimized", "full_screen"];

        public static T Of<T>(JsonNode? v, string[] names) where T : struct, System.Enum
        {
            var i = System.Array.IndexOf(names, Components.Text(v));
            return (T)(object)(i < 0 ? 0 : i);
        }

        public static T[] SetOf<T>(JsonNode? v, string[] names) where T : struct, System.Enum =>
            v is JsonArray a ? a.Select(x => System.Array.IndexOf(names, Components.Text(x))).Where(i => i >= 0).Select(i => (T)(object)i).ToArray() : [];

        public static JsonArray Names<T>(System.Collections.Generic.IEnumerable<T> values, string[] names) where T : struct, System.Enum =>
            new(values.Select(v => (JsonNode?)names[(int)(object)v]).ToArray());
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record GestureEvent(string Phase, double X, double Y, double Scale, double DeltaScale, double Rotation, double DeltaRotation, double PanX, double PanY)
    {
        /// <summary>Com os padrões: <c>new GestureEvent { Campo = valor }</c> preenche só os dados.</summary>
        public GestureEvent() : this(FromJson(null)) { }

        internal static GestureEvent FromJson(JsonNode? n) => new(Components.Text(n?["phase"]), Components.Number(n?["x"]), Components.Number(n?["y"]), Components.Number(n?["scale"]), Components.Number(n?["delta_scale"]), Components.Number(n?["rotation"]), Components.Number(n?["delta_rotation"]), Components.Number(n?["pan_x"]), Components.Number(n?["pan_y"]));

        internal JsonObject ToJson() => new() { ["phase"] = JsonValue.Create(Phase), ["x"] = JsonValue.Create(X), ["y"] = JsonValue.Create(Y), ["scale"] = JsonValue.Create(Scale), ["delta_scale"] = JsonValue.Create(DeltaScale), ["rotation"] = JsonValue.Create(Rotation), ["delta_rotation"] = JsonValue.Create(DeltaRotation), ["pan_x"] = JsonValue.Create(PanX), ["pan_y"] = JsonValue.Create(PanY) };

        internal static GestureEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static GestureEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(GestureEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<GestureEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record KeyEvent(string Key, bool Ctrl, bool Shift, bool Alt, bool Meta, bool CapsLock, bool NumLock, bool ScrollLock, KeyCode Code, long Scancode, string Form, long FormId)
    {
        /// <summary>Com os padrões: <c>new KeyEvent { Campo = valor }</c> preenche só os dados.</summary>
        public KeyEvent() : this(FromJson(null)) { }

        internal static KeyEvent FromJson(JsonNode? n) => new(Components.Text(n?["key"]), Components.Bool(n?["ctrl"]), Components.Bool(n?["shift"]), Components.Bool(n?["alt"]), Components.Bool(n?["meta"]), Components.Bool(n?["caps_lock"]), Components.Bool(n?["num_lock"]), Components.Bool(n?["scroll_lock"]), EnumNames.Of<KeyCode>(n?["code"], EnumNames.KeyCode), (long)Components.Number(n?["scancode"]), Components.Text(n?["form"]), (long)Components.Number(n?["form_id"]));

        internal JsonObject ToJson() => new() { ["key"] = JsonValue.Create(Key), ["ctrl"] = JsonValue.Create(Ctrl), ["shift"] = JsonValue.Create(Shift), ["alt"] = JsonValue.Create(Alt), ["meta"] = JsonValue.Create(Meta), ["caps_lock"] = JsonValue.Create(CapsLock), ["num_lock"] = JsonValue.Create(NumLock), ["scroll_lock"] = JsonValue.Create(ScrollLock), ["code"] = JsonValue.Create(EnumNames.KeyCode[(int)Code]), ["scancode"] = JsonValue.Create(Scancode), ["form"] = JsonValue.Create(Form), ["form_id"] = JsonValue.Create(FormId) };

        internal static KeyEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static KeyEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(KeyEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<KeyEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record ListViewItem(string Title, string Detail)
    {
        /// <summary>Com os padrões: <c>new ListViewItem { Campo = valor }</c> preenche só os dados.</summary>
        public ListViewItem() : this(FromJson(null)) { }

        internal static ListViewItem FromJson(JsonNode? n) => new(Components.Text(n?["title"]), Components.Text(n?["detail"]));

        internal JsonObject ToJson() => new() { ["title"] = JsonValue.Create(Title), ["detail"] = JsonValue.Create(Detail) };

        internal static ListViewItem? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static ListViewItem[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(ListViewItem? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<ListViewItem> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record PointerEvent(double X, double Y, string Button, long Clicks)
    {
        /// <summary>Com os padrões: <c>new PointerEvent { Campo = valor }</c> preenche só os dados.</summary>
        public PointerEvent() : this(FromJson(null)) { }

        internal static PointerEvent FromJson(JsonNode? n) => new(Components.Number(n?["x"]), Components.Number(n?["y"]), Components.Text(n?["button"]), (long)Components.Number(n?["clicks"]));

        internal JsonObject ToJson() => new() { ["x"] = JsonValue.Create(X), ["y"] = JsonValue.Create(Y), ["button"] = JsonValue.Create(Button), ["clicks"] = JsonValue.Create(Clicks) };

        internal static PointerEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static PointerEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(PointerEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<PointerEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record SwipeEvent(string Direction, double X, double Y, double Dx, double Dy, double Vx, double Vy)
    {
        /// <summary>Com os padrões: <c>new SwipeEvent { Campo = valor }</c> preenche só os dados.</summary>
        public SwipeEvent() : this(FromJson(null)) { }

        internal static SwipeEvent FromJson(JsonNode? n) => new(Components.Text(n?["direction"]), Components.Number(n?["x"]), Components.Number(n?["y"]), Components.Number(n?["dx"]), Components.Number(n?["dy"]), Components.Number(n?["vx"]), Components.Number(n?["vy"]));

        internal JsonObject ToJson() => new() { ["direction"] = JsonValue.Create(Direction), ["x"] = JsonValue.Create(X), ["y"] = JsonValue.Create(Y), ["dx"] = JsonValue.Create(Dx), ["dy"] = JsonValue.Create(Dy), ["vx"] = JsonValue.Create(Vx), ["vy"] = JsonValue.Create(Vy) };

        internal static SwipeEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static SwipeEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(SwipeEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<SwipeEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record WheelEvent(double X, double Y, double Dx, double Dy)
    {
        /// <summary>Com os padrões: <c>new WheelEvent { Campo = valor }</c> preenche só os dados.</summary>
        public WheelEvent() : this(FromJson(null)) { }

        internal static WheelEvent FromJson(JsonNode? n) => new(Components.Number(n?["x"]), Components.Number(n?["y"]), Components.Number(n?["dx"]), Components.Number(n?["dy"]));

        internal JsonObject ToJson() => new() { ["x"] = JsonValue.Create(X), ["y"] = JsonValue.Create(Y), ["dx"] = JsonValue.Create(Dx), ["dy"] = JsonValue.Create(Dy) };

        internal static WheelEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static WheelEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(WheelEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<WheelEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Os forms (<c>using static App.Forms.Components;</c>): cada frmX é a janela do
    /// form cujo evento está rodando (a primeira, fora dos handlers).</summary>
    public static class Components
    {
        internal static bool Bool(JsonNode? v) => v is JsonValue j && j.TryGetValue<bool>(out var b) && b;
        internal static double Number(JsonNode? v) => v is JsonValue j && j.TryGetValue<double>(out var d) ? d : 0;
        internal static string Text(JsonNode? v) => v is JsonValue j && j.TryGetValue<string>(out var t) ? t : v?.ToJsonString() ?? "";
        internal static DateOnly? DateOf(JsonNode? v) => DateOnly.TryParseExact(Text(v), "yyyy-MM-dd", CultureInfo.InvariantCulture, DateTimeStyles.None, out var d) ? d : null;
        internal static TimeOnly? TimeOf(JsonNode? v) => TimeOnly.TryParse(Text(v), CultureInfo.InvariantCulture, DateTimeStyles.None, out var t) ? t : null;
        internal static DateTime? DateTimeOf(JsonNode? v) => DateTime.TryParse(Text(v), CultureInfo.InvariantCulture, DateTimeStyles.None, out var d) ? d : null;
        internal static string DateText(DateOnly? d) => d?.ToString("yyyy-MM-dd", CultureInfo.InvariantCulture) ?? "";
        internal static string TimeText(TimeOnly? t) => t is { } x ? x.ToString(x.Second == 0 ? "HH:mm" : "HH:mm:ss", CultureInfo.InvariantCulture) : "";
        internal static string DateTimeText(DateTime? d) => d is { } x ? x.ToString(x.Second == 0 ? "yyyy-MM-dd'T'HH:mm" : "yyyy-MM-dd'T'HH:mm:ss", CultureInfo.InvariantCulture) : "";
        internal static string[] ListOf(JsonNode? v) => Text(v).Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        internal static double[] NumbersOf(JsonNode? v) => ListOf(v).Select(s => double.TryParse(s, NumberStyles.Float, CultureInfo.InvariantCulture, out var d) ? (double?)d : null).OfType<double>().ToArray();
        internal static Color[] ColorsOf(JsonNode? v) => ListOf(v).Where(s => s.StartsWith('#')).Select(Color.Hex).ToArray();
        internal static string NumbersText(System.Collections.Generic.IEnumerable<double> v) => string.Join(";", v.Select(d => d.ToString(CultureInfo.InvariantCulture)));

        /// <summary>O intervalo de datas (o <c>on_change</c> do DateRangePicker); null: sem a data.</summary>
        public sealed record DateRange(DateOnly? Start, DateOnly? End);

        internal static DateRange DateRangeOf(JsonNode? v)
        {
            var p = (Text(v) + ";").Split(';');
            return new DateRange(DateOf(JsonValue.Create(p[0].Trim())), DateOf(JsonValue.Create(p[1].Trim())));
        }

        public static AylisUi.FrmCalculadoraComponents frmCalculadora = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmCalculadora", (aylis, id, name) => new AylisUi.FrmCalculadoraComponents(aylis, id, name), form => frmCalculadora = (AylisUi.FrmCalculadoraComponents)form);
        }
    }
}

namespace App.Forms.AylisUi
{
    using static App.Forms.Components;

    /// <summary>Um Button de um form.</summary>
    public class Button(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public string Icon { get => Text(GetProp("icon")); set => SetProp("icon", value); }
        public string CheckedIcon { get => Text(GetProp("checked_icon")); set => SetProp("checked_icon", value); }
        public IconPosition IconPosition { get => EnumNames.Of<IconPosition>(GetProp("icon_position"), EnumNames.IconPosition); set => SetProp("icon_position", EnumNames.IconPosition[(int)value]); }
        public double IconSize { get => Number(GetProp("icon_size")); set => SetProp("icon_size", value); }
        public bool Checkable { get => Bool(GetProp("checkable")); set => SetProp("checkable", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public ModalResult ModalResult { get => EnumNames.Of<ModalResult>(GetProp("modal_result"), EnumNames.ModalResult); set => SetProp("modal_result", EnumNames.ModalResult[(int)value]); }
        public bool RepeatEnter { get => Bool(GetProp("repeat_enter")); set => SetProp("repeat_enter", value); }
        public bool DefaultButton { get => Bool(GetProp("default_button")); set => SetProp("default_button", value); }
        public bool CancelButton { get => Bool(GetProp("cancel_button")); set => SetProp("cancel_button", value); }
        public TextAlign HAlign { get => EnumNames.Of<TextAlign>(GetProp("h_align"), EnumNames.TextAlign); set => SetProp("h_align", EnumNames.TextAlign[(int)value]); }
        public long GroupIndex { get => (long)Number(GetProp("group_index")); set => SetProp("group_index", value); }
        public Bounds Radius { get => Bounds.FromJson(GetProp("radius")); set => SetProp("radius", value.ToJson()); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public ButtonStyle Style { get => EnumNames.Of<ButtonStyle>(GetProp("style"), EnumNames.ButtonStyle); set => SetProp("style", EnumNames.ButtonStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um Form de um form.</summary>
    public class Form(Aylis aylis, long id, string name) : AylisForm(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
        public FormPosition Position { get => EnumNames.Of<FormPosition>(GetProp("position"), EnumNames.FormPosition); set => SetProp("position", EnumNames.FormPosition[(int)value]); }
        public BorderStyle BorderStyle { get => EnumNames.Of<BorderStyle>(GetProp("border_style"), EnumNames.BorderStyle); set => SetProp("border_style", EnumNames.BorderStyle[(int)value]); }
        public WindowCorners WindowCorners { get => EnumNames.Of<WindowCorners>(GetProp("window_corners"), EnumNames.WindowCorners); set => SetProp("window_corners", EnumNames.WindowCorners[(int)value]); }
        public bool Transparent { get => Bool(GetProp("transparent")); set => SetProp("transparent", value); }
        public WindowShape WindowShape { get => EnumNames.Of<WindowShape>(GetProp("window_shape"), EnumNames.WindowShape); set => SetProp("window_shape", EnumNames.WindowShape[(int)value]); }
        public double ShapeRadius { get => Number(GetProp("shape_radius")); set => SetProp("shape_radius", value); }
        public Bounds ShapeRadii { get => Bounds.FromJson(GetProp("shape_radii")); set => SetProp("shape_radii", value.ToJson()); }
        public string ShapePath { get => Text(GetProp("shape_path")); set => SetProp("shape_path", value); }
        public WindowState WindowState { get => EnumNames.Of<WindowState>(GetProp("window_state"), EnumNames.WindowState); set => SetProp("window_state", EnumNames.WindowState[(int)value]); }
        public Transition Transition { get => EnumNames.Of<Transition>(GetProp("transition"), EnumNames.Transition); set => SetProp("transition", EnumNames.Transition[(int)value]); }
        public LayerCaching LayerCache { get => EnumNames.Of<LayerCaching>(GetProp("layer_cache"), EnumNames.LayerCaching); set => SetProp("layer_cache", EnumNames.LayerCaching[(int)value]); }
        public long FrameReuseKb { get => (long)Number(GetProp("frame_reuse_kb")); set => SetProp("frame_reuse_kb", value); }
        public bool Topmost { get => Bool(GetProp("topmost")); set => SetProp("topmost", value); }
        public bool Bottommost { get => Bool(GetProp("bottommost")); set => SetProp("bottommost", value); }
        public KeepAwake KeepAwake { get => EnumNames.Of<KeepAwake>(GetProp("keep_awake"), EnumNames.KeepAwake); set => SetProp("keep_awake", EnumNames.KeepAwake[(int)value]); }
        public CursorConfine ConfineCursor { get => EnumNames.Of<CursorConfine>(GetProp("confine_cursor"), EnumNames.CursorConfine); set => SetProp("confine_cursor", EnumNames.CursorConfine[(int)value]); }
        public double WindowOpacity { get => Number(GetProp("window_opacity")); set => SetProp("window_opacity", value); }
        public FullScreenMode FullScreenMode { get => EnumNames.Of<FullScreenMode>(GetProp("full_screen_mode"), EnumNames.FullScreenMode); set => SetProp("full_screen_mode", EnumNames.FullScreenMode[(int)value]); }
        public long VideoWidth { get => (long)Number(GetProp("video_width")); set => SetProp("video_width", value); }
        public long VideoHeight { get => (long)Number(GetProp("video_height")); set => SetProp("video_height", value); }
        public double VideoRefresh { get => Number(GetProp("video_refresh")); set => SetProp("video_refresh", value); }
        public bool ShowInTaskbar { get => Bool(GetProp("show_in_taskbar")); set => SetProp("show_in_taskbar", value); }
        public AylisComponent? ActiveControl { get => GetRef("active_control"); set => SetRef("active_control", value); }
        public string[] TabOrder { get => ListOf(GetProp("tab_order")); set => SetProp("tab_order", string.Join(";", value)); }
        public string TabKeys { get => Text(GetProp("tab_keys")); set => SetProp("tab_keys", value); }
        public string BackTabKeys { get => Text(GetProp("back_tab_keys")); set => SetProp("back_tab_keys", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public bool KeyPreview { get => Bool(GetProp("key_preview")); set => SetProp("key_preview", value); }
        public double Zoom { get => Number(GetProp("zoom")); set => SetProp("zoom", value); }
        public long Monitor { get => (long)Number(GetProp("monitor")); set => SetProp("monitor", value); }
        public bool RememberPlacement { get => Bool(GetProp("remember_placement")); set => SetProp("remember_placement", value); }
        public string Gpu { get => Text(GetProp("gpu")); set => SetProp("gpu", value); }
        public FormPresent Present { get => EnumNames.Of<FormPresent>(GetProp("present"), EnumNames.FormPresent); set => SetProp("present", EnumNames.FormPresent[(int)value]); }
        public FormBlendCompositor BlendCompositor { get => EnumNames.Of<FormBlendCompositor>(GetProp("blend_compositor"), EnumNames.FormBlendCompositor); set => SetProp("blend_compositor", EnumNames.FormBlendCompositor[(int)value]); }
        public bool SpanMonitors { get => Bool(GetProp("span_monitors")); set => SetProp("span_monitors", value); }
        public bool PhysicalPixels { get => Bool(GetProp("physical_pixels")); set => SetProp("physical_pixels", value); }
        public bool ActivateOnShow { get => Bool(GetProp("activate_on_show")); set => SetProp("activate_on_show", value); }
        public bool ForceForeground { get => Bool(GetProp("force_foreground")); set => SetProp("force_foreground", value); }
        public CloseAction CloseAction { get => EnumNames.Of<CloseAction>(GetProp("close_action"), EnumNames.CloseAction); set => SetProp("close_action", EnumNames.CloseAction[(int)value]); }
        public double FrameRate { get => Number(GetProp("frame_rate")); set => SetProp("frame_rate", value); }
        public FormOrientation Orientation { get => EnumNames.Of<FormOrientation>(GetProp("orientation"), EnumNames.FormOrientation); set => SetProp("orientation", EnumNames.FormOrientation[(int)value]); }
        public AylisComponent? Host { get => GetRef("host"); set => SetRef("host", value); }
        public bool Dockable { get => Bool(GetProp("dockable")); set => SetProp("dockable", value); }
        public string OnShow { get => Text(GetProp("on_show")); set => SetProp("on_show", value); }
        public string OnWindowCreated { get => Text(GetProp("on_window_created")); set => SetProp("on_window_created", value); }
        public string OnClose { get => Text(GetProp("on_close")); set => SetProp("on_close", value); }
        public string OnActivate { get => Text(GetProp("on_activate")); set => SetProp("on_activate", value); }
        public string OnDeactivate { get => Text(GetProp("on_deactivate")); set => SetProp("on_deactivate", value); }
        public string OnMinimize { get => Text(GetProp("on_minimize")); set => SetProp("on_minimize", value); }
        public string OnRestore { get => Text(GetProp("on_restore")); set => SetProp("on_restore", value); }
        public string OnNavigate { get => Text(GetProp("on_navigate")); set => SetProp("on_navigate", value); }
        public string OnGpuChange { get => Text(GetProp("on_gpu_change")); set => SetProp("on_gpu_change", value); }
        public string OnDock { get => Text(GetProp("on_dock")); set => SetProp("on_dock", value); }
        public string OnUndock { get => Text(GetProp("on_undock")); set => SetProp("on_undock", value); }
        public string OnFilesDrop { get => Text(GetProp("on_files_drop")); set => SetProp("on_files_drop", value); }
        public string OnViewChange { get => Text(GetProp("on_view_change")); set => SetProp("on_view_change", value); }
        public string OnTopmostLost { get => Text(GetProp("on_topmost_lost")); set => SetProp("on_topmost_lost", value); }
        public string OnScaleChange { get => Text(GetProp("on_scale_change")); set => SetProp("on_scale_change", value); }
        public string OnResizeEnd { get => Text(GetProp("on_resize_end")); set => SetProp("on_resize_end", value); }
        public double ClientWidth { get => Number(GetProp("client_width")); set => SetProp("client_width", value); }
        public double ClientHeight { get => Number(GetProp("client_height")); set => SetProp("client_height", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public FormStyle Style { get => EnumNames.Of<FormStyle>(GetProp("style"), EnumNames.FormStyle); set => SetProp("style", EnumNames.FormStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
        public void MoveToMonitor(long monitor) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "move_to_monitor", ["args"] = new JsonArray(monitor) });
        public void RequestAttention(bool critical) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "request_attention", ["args"] = new JsonArray(critical) });
        public void BringToFront() => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "bring_to_front", ["args"] = new JsonArray() });
        public void SetActivationToken(string token) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "set_activation_token", ["args"] = new JsonArray(token) });
        public double WindowScale() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "window_scale", ["args"] = new JsonArray() }));
        public bool PointerOver() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "pointer_over", ["args"] = new JsonArray() }));
        public void Place(long monitor, long x, long y, long width, long height, bool activate) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "place", ["args"] = new JsonArray(monitor, x, y, width, height, activate) });
        public string GpuAdapter() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapter", ["args"] = new JsonArray() }));
        public string[] GpuAdapters() => ListOf(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapters", ["args"] = new JsonArray() }));
        public double DropX() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "drop_x", ["args"] = new JsonArray() }));
        public double DropY() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "drop_y", ["args"] = new JsonArray() }));
        public string RenderRgba(double x, double y, double width, double height, double scale) => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "render_rgba", ["args"] = new JsonArray(x, y, width, height, scale) }));
        public string Focused() => Text(Aylis.Call("focused", new JsonObject { ["target"] = Id }));
        public void FocusNext() => Aylis.Send("focus_next", new JsonObject { ["target"] = Id });
        public void FocusPrev() => Aylis.Send("focus_prev", new JsonObject { ["target"] = Id });
    }

    /// <summary>Um GridLayout de um form.</summary>
    public class GridLayout(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public long Columns { get => (long)Number(GetProp("columns")); set => SetProp("columns", value); }
        public double MinColumnWidth { get => Number(GetProp("min_column_width")); set => SetProp("min_column_width", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public double RowHeight { get => Number(GetProp("row_height")); set => SetProp("row_height", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public GridLayoutStyle Style { get => EnumNames.Of<GridLayoutStyle>(GetProp("style"), EnumNames.GridLayoutStyle); set => SetProp("style", EnumNames.GridLayoutStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um Label de um form.</summary>
    public class Label(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public TextAlign HAlign { get => EnumNames.Of<TextAlign>(GetProp("h_align"), EnumNames.TextAlign); set => SetProp("h_align", EnumNames.TextAlign[(int)value]); }
        public TextAlign VAlign { get => EnumNames.Of<TextAlign>(GetProp("v_align"), EnumNames.TextAlign); set => SetProp("v_align", EnumNames.TextAlign[(int)value]); }
        public bool Wrap { get => Bool(GetProp("wrap")); set => SetProp("wrap", value); }
        public Trimming Trimming { get => EnumNames.Of<Trimming>(GetProp("trimming"), EnumNames.Trimming); set => SetProp("trimming", EnumNames.Trimming[(int)value]); }
        public bool Bold { get => Bool(GetProp("bold")); set => SetProp("bold", value); }
        public Color Color { get => Color.FromJson(GetProp("color")); set => SetProp("color", value.ToJson()); }
        public string Highlight { get => Text(GetProp("highlight")); set => SetProp("highlight", value); }
        public double LetterSpacing { get => Number(GetProp("letter_spacing")); set => SetProp("letter_spacing", value); }
        public TextCase TextCase { get => EnumNames.Of<TextCase>(GetProp("text_case"), EnumNames.TextCase); set => SetProp("text_case", EnumNames.TextCase[(int)value]); }
        public bool Markup { get => Bool(GetProp("markup")); set => SetProp("markup", value); }
        public Color Outline { get => Color.FromJson(GetProp("outline")); set => SetProp("outline", value.ToJson()); }
        public double OutlineWidth { get => Number(GetProp("outline_width")); set => SetProp("outline_width", value); }
        public double LineHeight { get => Number(GetProp("line_height")); set => SetProp("line_height", value); }
        public Shadow? TextShadow { get => Shadow.FromJson(GetProp("text_shadow")); set => SetProp("text_shadow", (value?.ToJson() ?? "")); }
        public long MaxLines { get => (long)Number(GetProp("max_lines")); set => SetProp("max_lines", value); }
        public AylisComponent? FocusControl { get => GetRef("focus_control"); set => SetRef("focus_control", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public LabelStyle Style { get => EnumNames.Of<LabelStyle>(GetProp("style"), EnumNames.LabelStyle); set => SetProp("style", EnumNames.LabelStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
        public double MinContentWidth() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "min_content_width", ["args"] = new JsonArray() }));
        public double MaxContentWidth() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "max_content_width", ["args"] = new JsonArray() }));
    }

    /// <summary>Um Layout de um form.</summary>
    public class Layout(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public LayoutStyle Style { get => EnumNames.Of<LayoutStyle>(GetProp("style"), EnumNames.LayoutStyle); set => SetProp("style", EnumNames.LayoutStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um ListView de um form.</summary>
    public class ListView(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public global::App.Forms.ListViewItem[] Items { get => global::App.Forms.ListViewItem.ListFromText(GetProp("items")); set => SetProp("items", global::App.Forms.ListViewItem.ListToText(value)); }
        public long ItemIndex { get => (long)Number(GetProp("item_index")); set => SetProp("item_index", value); }
        public double ItemHeight { get => Number(GetProp("item_height")); set => SetProp("item_height", value); }
        public bool Accessory { get => Bool(GetProp("accessory")); set => SetProp("accessory", value); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public string OnItemClick { get => Text(GetProp("on_item_click")); set => SetProp("on_item_click", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public ListViewStyle Style { get => EnumNames.Of<ListViewStyle>(GetProp("style"), EnumNames.ListViewStyle); set => SetProp("style", EnumNames.ListViewStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um Panel de um form.</summary>
    public class Panel(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public Color HoverFill { get => Color.FromJson(GetProp("hover_fill")); set => SetProp("hover_fill", value.ToJson()); }
        public Color PressedFill { get => Color.FromJson(GetProp("pressed_fill")); set => SetProp("pressed_fill", value.ToJson()); }
        public bool Interactive { get => Bool(GetProp("interactive")); set => SetProp("interactive", value); }
        public Color SelectedFill { get => Color.FromJson(GetProp("selected_fill")); set => SetProp("selected_fill", value.ToJson()); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color Fill2 { get => Color.FromJson(GetProp("fill2")); set => SetProp("fill2", value.ToJson()); }
        public Color FillMid { get => Color.FromJson(GetProp("fill_mid")); set => SetProp("fill_mid", value.ToJson()); }
        public double FillMidAt { get => Number(GetProp("fill_mid_at")); set => SetProp("fill_mid_at", value); }
        public double GradientAngle { get => Number(GetProp("gradient_angle")); set => SetProp("gradient_angle", value); }
        public bool GradientRadial { get => Bool(GetProp("gradient_radial")); set => SetProp("gradient_radial", value); }
        public Color Stroke { get => Color.FromJson(GetProp("stroke")); set => SetProp("stroke", value.ToJson()); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
        public double[] StrokeDash { get => NumbersOf(GetProp("stroke_dash")); set => SetProp("stroke_dash", NumbersText(value)); }
        public Bounds Radius { get => Bounds.FromJson(GetProp("radius")); set => SetProp("radius", value.ToJson()); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public PanelStyle Style { get => EnumNames.Of<PanelStyle>(GetProp("style"), EnumNames.PanelStyle); set => SetProp("style", EnumNames.PanelStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um TabControl de um form.</summary>
    public class TabControl(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public long TabIndex { get => (long)Number(GetProp("tab_index")); set => SetProp("tab_index", value); }
        public bool ShowTabs { get => Bool(GetProp("show_tabs")); set => SetProp("show_tabs", value); }
        public Transition Transition { get => EnumNames.Of<Transition>(GetProp("transition"), EnumNames.Transition); set => SetProp("transition", EnumNames.Transition[(int)value]); }
        public bool Swipe { get => Bool(GetProp("swipe")); set => SetProp("swipe", value); }
        public TabPosition TabPosition { get => EnumNames.Of<TabPosition>(GetProp("tab_position"), EnumNames.TabPosition); set => SetProp("tab_position", EnumNames.TabPosition[(int)value]); }
        public double TabWidth { get => Number(GetProp("tab_width")); set => SetProp("tab_width", value); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public TabControlStyle Style { get => EnumNames.Of<TabControlStyle>(GetProp("style"), EnumNames.TabControlStyle); set => SetProp("style", EnumNames.TabControlStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>Um TabItem de um form.</summary>
    public class TabItem(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
        public string OnShow { get => Text(GetProp("on_show")); set => SetProp("on_show", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public Align Align { get => EnumNames.Of<Align>(GetProp("align"), EnumNames.Align); set => SetProp("align", EnumNames.Align[(int)value]); }
        public Anchor[] Anchors { get => EnumNames.SetOf<Anchor>(GetProp("anchors"), EnumNames.Anchor); set => SetProp("anchors", EnumNames.Names(value, EnumNames.Anchor)); }
        public Bounds Margins { get => Bounds.FromJson(GetProp("margins")); set => SetProp("margins", value.ToJson()); }
        public Bounds Padding { get => Bounds.FromJson(GetProp("padding")); set => SetProp("padding", value.ToJson()); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CapturesGesture { get => Bool(GetProp("captures_gesture")); set => SetProp("captures_gesture", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public AylisComponent? DragImage { get => GetRef("drag_image"); set => SetRef("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public CursorShape Cursor { get => EnumNames.Of<CursorShape>(GetProp("cursor"), EnumNames.CursorShape); set => SetProp("cursor", EnumNames.CursorShape[(int)value]); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public BlendMode BlendMode { get => EnumNames.Of<BlendMode>(GetProp("blend_mode"), EnumNames.BlendMode); set => SetProp("blend_mode", EnumNames.BlendMode[(int)value]); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double Scale { get => Number(GetProp("scale")); set => SetProp("scale", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public SelfAlign SelfAlign { get => EnumNames.Of<SelfAlign>(GetProp("self_align"), EnumNames.SelfAlign); set => SetProp("self_align", EnumNames.SelfAlign[(int)value]); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public Shadow? Shadow { get => Shadow.FromJson(GetProp("shadow")); set => SetProp("shadow", (value?.ToJson() ?? "")); }
        public TabItemStyle Style { get => EnumNames.Of<TabItemStyle>(GetProp("style"), EnumNames.TabItemStyle); set => SetProp("style", EnumNames.TabItemStyle[(int)value]); }
        public ThemeName Theme { get => EnumNames.Of<ThemeName>(GetProp("theme"), EnumNames.ThemeName); set => SetProp("theme", EnumNames.ThemeName[(int)value]); }
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color BorderColor { get => Color.FromJson(GetProp("border_color")); set => SetProp("border_color", value.ToJson()); }
        public Color TextColor { get => Color.FromJson(GetProp("text_color")); set => SetProp("text_color", value.ToJson()); }
        public Color AccentColor { get => Color.FromJson(GetProp("accent_color")); set => SetProp("accent_color", value.ToJson()); }
        public Color SuccessColor { get => Color.FromJson(GetProp("success_color")); set => SetProp("success_color", value.ToJson()); }
        public Color WarningColor { get => Color.FromJson(GetProp("warning_color")); set => SetProp("warning_color", value.ToJson()); }
        public Color DangerColor { get => Color.FromJson(GetProp("danger_color")); set => SetProp("danger_color", value.ToJson()); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public FontWeight FontWeight { get => EnumNames.Of<FontWeight>(GetProp("font_weight"), EnumNames.FontWeight); set => SetProp("font_weight", EnumNames.FontWeight[(int)value]); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public AylisComponent? PopupMenu { get => GetRef("popup_menu"); set => SetRef("popup_menu", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
        public string OnDblClick { get => Text(GetProp("on_dbl_click")); set => SetProp("on_dbl_click", value); }
        public string OnLongPress { get => Text(GetProp("on_long_press")); set => SetProp("on_long_press", value); }
        public string OnPointerDown { get => Text(GetProp("on_pointer_down")); set => SetProp("on_pointer_down", value); }
        public string OnPointerUp { get => Text(GetProp("on_pointer_up")); set => SetProp("on_pointer_up", value); }
        public string OnPointerMove { get => Text(GetProp("on_pointer_move")); set => SetProp("on_pointer_move", value); }
        public string OnPointerEnter { get => Text(GetProp("on_pointer_enter")); set => SetProp("on_pointer_enter", value); }
        public string OnPointerLeave { get => Text(GetProp("on_pointer_leave")); set => SetProp("on_pointer_leave", value); }
        public string OnWheel { get => Text(GetProp("on_wheel")); set => SetProp("on_wheel", value); }
        public string OnGesture { get => Text(GetProp("on_gesture")); set => SetProp("on_gesture", value); }
        public string OnSwipe { get => Text(GetProp("on_swipe")); set => SetProp("on_swipe", value); }
        public string OnKeyUp { get => Text(GetProp("on_key_up")); set => SetProp("on_key_up", value); }
        public string OnEnter { get => Text(GetProp("on_enter")); set => SetProp("on_enter", value); }
        public string OnExit { get => Text(GetProp("on_exit")); set => SetProp("on_exit", value); }
        public string OnResize { get => Text(GetProp("on_resize")); set => SetProp("on_resize", value); }
        public string OnDragMove { get => Text(GetProp("on_drag_move")); set => SetProp("on_drag_move", value); }
        public string OnDragLeave { get => Text(GetProp("on_drag_leave")); set => SetProp("on_drag_leave", value); }
        public string OnDragDrop { get => Text(GetProp("on_drag_drop")); set => SetProp("on_drag_drop", value); }
        public string OnDragEnd { get => Text(GetProp("on_drag_end")); set => SetProp("on_drag_end", value); }
    }

    /// <summary>O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmCalculadoraComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public Panel pnlLateral = null!;
        public TabControl tabLateral = null!;
        public TabItem tabHistorico = null!;
        public Button btnLimparHistorico = null!;
        public ListView lvHistorico = null!;
        public TabItem tabMemoria = null!;
        public ListView lvMemoria = null!;
        public Layout lytCorpo = null!;
        public Layout lytTopo = null!;
        public Button btnHistorico = null!;
        public Label lblTitulo = null!;
        public Label lblExpressao = null!;
        public Label lblVisor = null!;
        public GridLayout grdMemoria = null!;
        public Button btnMC = null!;
        public Button btnMR = null!;
        public Button btnMMais = null!;
        public Button btnMMenos = null!;
        public Button btnMS = null!;
        public GridLayout grdTeclado = null!;
        public Button btnPorcento = null!;
        public Button btnCE = null!;
        public Button btnC = null!;
        public Button btnApagar = null!;
        public Button btnInverso = null!;
        public Button btnQuadrado = null!;
        public Button btnRaiz = null!;
        public Button btnDividir = null!;
        public Button btn7 = null!;
        public Button btn8 = null!;
        public Button btn9 = null!;
        public Button btnMultiplicar = null!;
        public Button btn4 = null!;
        public Button btn5 = null!;
        public Button btn6 = null!;
        public Button btnSubtrair = null!;
        public Button btn1 = null!;
        public Button btn2 = null!;
        public Button btn3 = null!;
        public Button btnSomar = null!;
        public Button btnSinal = null!;
        public Button btn0 = null!;
        public Button btnVirgula = null!;
        public Button btnIgual = null!;

        public override void Bind(JsonArray list)
        {
            pnlLateral = Resolve(list, "pnlLateral", "Panel", (i, d, n) => new Panel(i, d, n));
            tabLateral = Resolve(list, "tabLateral", "TabControl", (i, d, n) => new TabControl(i, d, n));
            tabHistorico = Resolve(list, "tabHistorico", "TabItem", (i, d, n) => new TabItem(i, d, n));
            btnLimparHistorico = Resolve(list, "btnLimparHistorico", "Button", (i, d, n) => new Button(i, d, n));
            lvHistorico = Resolve(list, "lvHistorico", "ListView", (i, d, n) => new ListView(i, d, n));
            tabMemoria = Resolve(list, "tabMemoria", "TabItem", (i, d, n) => new TabItem(i, d, n));
            lvMemoria = Resolve(list, "lvMemoria", "ListView", (i, d, n) => new ListView(i, d, n));
            lytCorpo = Resolve(list, "lytCorpo", "Layout", (i, d, n) => new Layout(i, d, n));
            lytTopo = Resolve(list, "lytTopo", "Layout", (i, d, n) => new Layout(i, d, n));
            btnHistorico = Resolve(list, "btnHistorico", "Button", (i, d, n) => new Button(i, d, n));
            lblTitulo = Resolve(list, "lblTitulo", "Label", (i, d, n) => new Label(i, d, n));
            lblExpressao = Resolve(list, "lblExpressao", "Label", (i, d, n) => new Label(i, d, n));
            lblVisor = Resolve(list, "lblVisor", "Label", (i, d, n) => new Label(i, d, n));
            grdMemoria = Resolve(list, "grdMemoria", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnMC = Resolve(list, "btnMC", "Button", (i, d, n) => new Button(i, d, n));
            btnMR = Resolve(list, "btnMR", "Button", (i, d, n) => new Button(i, d, n));
            btnMMais = Resolve(list, "btnMMais", "Button", (i, d, n) => new Button(i, d, n));
            btnMMenos = Resolve(list, "btnMMenos", "Button", (i, d, n) => new Button(i, d, n));
            btnMS = Resolve(list, "btnMS", "Button", (i, d, n) => new Button(i, d, n));
            grdTeclado = Resolve(list, "grdTeclado", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnPorcento = Resolve(list, "btnPorcento", "Button", (i, d, n) => new Button(i, d, n));
            btnCE = Resolve(list, "btnCE", "Button", (i, d, n) => new Button(i, d, n));
            btnC = Resolve(list, "btnC", "Button", (i, d, n) => new Button(i, d, n));
            btnApagar = Resolve(list, "btnApagar", "Button", (i, d, n) => new Button(i, d, n));
            btnInverso = Resolve(list, "btnInverso", "Button", (i, d, n) => new Button(i, d, n));
            btnQuadrado = Resolve(list, "btnQuadrado", "Button", (i, d, n) => new Button(i, d, n));
            btnRaiz = Resolve(list, "btnRaiz", "Button", (i, d, n) => new Button(i, d, n));
            btnDividir = Resolve(list, "btnDividir", "Button", (i, d, n) => new Button(i, d, n));
            btn7 = Resolve(list, "btn7", "Button", (i, d, n) => new Button(i, d, n));
            btn8 = Resolve(list, "btn8", "Button", (i, d, n) => new Button(i, d, n));
            btn9 = Resolve(list, "btn9", "Button", (i, d, n) => new Button(i, d, n));
            btnMultiplicar = Resolve(list, "btnMultiplicar", "Button", (i, d, n) => new Button(i, d, n));
            btn4 = Resolve(list, "btn4", "Button", (i, d, n) => new Button(i, d, n));
            btn5 = Resolve(list, "btn5", "Button", (i, d, n) => new Button(i, d, n));
            btn6 = Resolve(list, "btn6", "Button", (i, d, n) => new Button(i, d, n));
            btnSubtrair = Resolve(list, "btnSubtrair", "Button", (i, d, n) => new Button(i, d, n));
            btn1 = Resolve(list, "btn1", "Button", (i, d, n) => new Button(i, d, n));
            btn2 = Resolve(list, "btn2", "Button", (i, d, n) => new Button(i, d, n));
            btn3 = Resolve(list, "btn3", "Button", (i, d, n) => new Button(i, d, n));
            btnSomar = Resolve(list, "btnSomar", "Button", (i, d, n) => new Button(i, d, n));
            btnSinal = Resolve(list, "btnSinal", "Button", (i, d, n) => new Button(i, d, n));
            btn0 = Resolve(list, "btn0", "Button", (i, d, n) => new Button(i, d, n));
            btnVirgula = Resolve(list, "btnVirgula", "Button", (i, d, n) => new Button(i, d, n));
            btnIgual = Resolve(list, "btnIgual", "Button", (i, d, n) => new Button(i, d, n));
        }

        /// <summary>O Button deste form com o nome.</summary>
        public Button? FindButton(string name) => name switch
        {
            "btnLimparHistorico" => btnLimparHistorico,
            "btnHistorico" => btnHistorico,
            "btnMC" => btnMC,
            "btnMR" => btnMR,
            "btnMMais" => btnMMais,
            "btnMMenos" => btnMMenos,
            "btnMS" => btnMS,
            "btnPorcento" => btnPorcento,
            "btnCE" => btnCE,
            "btnC" => btnC,
            "btnApagar" => btnApagar,
            "btnInverso" => btnInverso,
            "btnQuadrado" => btnQuadrado,
            "btnRaiz" => btnRaiz,
            "btnDividir" => btnDividir,
            "btn7" => btn7,
            "btn8" => btn8,
            "btn9" => btn9,
            "btnMultiplicar" => btnMultiplicar,
            "btn4" => btn4,
            "btn5" => btn5,
            "btn6" => btn6,
            "btnSubtrair" => btnSubtrair,
            "btn1" => btn1,
            "btn2" => btn2,
            "btn3" => btn3,
            "btnSomar" => btnSomar,
            "btnSinal" => btnSinal,
            "btn0" => btn0,
            "btnVirgula" => btnVirgula,
            "btnIgual" => btnIgual,
            _ => null,
        };

        /// <summary>O Button que disparou o evento.</summary>
        public Button? SenderAsButton(AylisClient.AylisEvent ev) => FindButton(ev.SenderName);

        /// <summary>O GridLayout deste form com o nome.</summary>
        public GridLayout? FindGridLayout(string name) => name switch
        {
            "grdMemoria" => grdMemoria,
            "grdTeclado" => grdTeclado,
            _ => null,
        };

        /// <summary>O GridLayout que disparou o evento.</summary>
        public GridLayout? SenderAsGridLayout(AylisClient.AylisEvent ev) => FindGridLayout(ev.SenderName);

        /// <summary>O Label deste form com o nome.</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblTitulo" => lblTitulo,
            "lblExpressao" => lblExpressao,
            "lblVisor" => lblVisor,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome.</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytCorpo" => lytCorpo,
            "lytTopo" => lytTopo,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O ListView deste form com o nome.</summary>
        public ListView? FindListView(string name) => name switch
        {
            "lvHistorico" => lvHistorico,
            "lvMemoria" => lvMemoria,
            _ => null,
        };

        /// <summary>O ListView que disparou o evento.</summary>
        public ListView? SenderAsListView(AylisClient.AylisEvent ev) => FindListView(ev.SenderName);

        /// <summary>O Panel deste form com o nome.</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlLateral" => pnlLateral,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O TabControl deste form com o nome.</summary>
        public TabControl? FindTabControl(string name) => name switch
        {
            "tabLateral" => tabLateral,
            _ => null,
        };

        /// <summary>O TabControl que disparou o evento.</summary>
        public TabControl? SenderAsTabControl(AylisClient.AylisEvent ev) => FindTabControl(ev.SenderName);

        /// <summary>O TabItem deste form com o nome.</summary>
        public TabItem? FindTabItem(string name) => name switch
        {
            "tabHistorico" => tabHistorico,
            "tabMemoria" => tabMemoria,
            _ => null,
        };

        /// <summary>O TabItem que disparou o evento.</summary>
        public TabItem? SenderAsTabItem(AylisClient.AylisEvent ev) => FindTabItem(ev.SenderName);
    }
}
