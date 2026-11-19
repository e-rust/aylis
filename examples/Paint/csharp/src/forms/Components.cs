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
    public enum CheckBoxStyle
    {
        Default,
        Round,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CircleStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CloseAction
    {
        Hide,
        Free,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ComboArrow
    {
        Chevron,
        Triangle,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ComboBoxStyle
    {
        Default,
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
    public enum DividerStyle
    {
        Default,
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
    public enum MainMenuStyle
    {
        Default,
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
    public enum Overscroll
    {
        Auto,
        Glow,
        Stretch,
        Bounce,
        None,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum PaintCanvasStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum PaintTool
    {
        Pencil,
        Brush,
        Eraser,
        Line,
        Rectangle,
        Ellipse,
        Fill,
        Picker,
        Select,
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
    public enum ScrollBoxStyle
    {
        Default,
        Horizontal,
        Refresh,
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
    public enum SmoothScroll
    {
        Default,
        On,
        Off,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum StatusBarStyle
    {
        Default,
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
        public static readonly string[] CheckBoxStyle = ["", "round"];
        public static readonly string[] CircleStyle = [""];
        public static readonly string[] CloseAction = ["hide", "free"];
        public static readonly string[] ComboArrow = ["chevron", "triangle"];
        public static readonly string[] ComboBoxStyle = [""];
        public static readonly string[] CursorShape = ["default", "pointer", "text", "wait", "crosshair", "move", "not_allowed", "resize_horizontal", "resize_vertical", "resize_ne_sw", "resize_nw_se", "grab", "grabbing", "none"];
        public static readonly string[] DividerStyle = [""];
        public static readonly string[] FontWeight = ["default", "thin", "light", "normal", "medium", "semibold", "bold", "black"];
        public static readonly string[] FormOrientation = ["project", "any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"];
        public static readonly string[] FormPosition = ["center", "screen_center", "designed", "default", "top_left", "top_right", "bottom_left", "bottom_right"];
        public static readonly string[] FormPresent = ["project", "window", "composition", "composed"];
        public static readonly string[] FormStyle = [""];
        public static readonly string[] GridLayoutStyle = [""];
        public static readonly string[] IconPosition = ["left", "right", "top"];
        public static readonly string[] LabelStyle = ["", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title"];
        public static readonly string[] LayerCaching = ["project", "on", "off"];
        public static readonly string[] LayoutStyle = [""];
        public static readonly string[] MainMenuStyle = [""];
        public static readonly string[] ModalResult = ["none", "ok", "cancel", "abort", "retry", "ignore", "yes", "no", "close"];
        public static readonly string[] Overscroll = ["auto", "glow", "stretch", "bounce", "none"];
        public static readonly string[] PaintCanvasStyle = [""];
        public static readonly string[] PaintTool = ["pencil", "brush", "eraser", "line", "rectangle", "ellipse", "fill", "picker", "select"];
        public static readonly string[] PanelStyle = ["", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"];
        public static readonly string[] ScrollBoxStyle = ["", "horizontal", "refresh"];
        public static readonly string[] SelfAlign = ["stretch", "start", "center", "end"];
        public static readonly string[] SmoothScroll = ["default", "on", "off"];
        public static readonly string[] StatusBarStyle = [""];
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
    public sealed record ImageSize(long Width, long Height)
    {
        /// <summary>Com os padrões: <c>new ImageSize { Campo = valor }</c> preenche só os dados.</summary>
        public ImageSize() : this(FromJson(null)) { }

        internal static ImageSize FromJson(JsonNode? n) => new((long)Components.Number(n?["width"]), (long)Components.Number(n?["height"]));

        internal JsonObject ToJson() => new() { ["width"] = JsonValue.Create(Width), ["height"] = JsonValue.Create(Height) };

        internal static ImageSize? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static ImageSize[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(ImageSize? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<ImageSize> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record KeyEvent(string Key, bool Ctrl, bool Shift, bool Alt)
    {
        /// <summary>Com os padrões: <c>new KeyEvent { Campo = valor }</c> preenche só os dados.</summary>
        public KeyEvent() : this(FromJson(null)) { }

        internal static KeyEvent FromJson(JsonNode? n) => new(Components.Text(n?["key"]), Components.Bool(n?["ctrl"]), Components.Bool(n?["shift"]), Components.Bool(n?["alt"]));

        internal JsonObject ToJson() => new() { ["key"] = JsonValue.Create(Key), ["ctrl"] = JsonValue.Create(Ctrl), ["shift"] = JsonValue.Create(Shift), ["alt"] = JsonValue.Create(Alt) };

        internal static KeyEvent? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static KeyEvent[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(KeyEvent? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<KeyEvent> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
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

        public static AylisUi.FrmPaintComponents frmPaint = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmPaint", (aylis, id, name) => new AylisUi.FrmPaintComponents(aylis, id, name), form => frmPaint = (AylisUi.FrmPaintComponents)form);
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

    /// <summary>Um CheckBox de um form.</summary>
    public class CheckBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public double BoxSize { get => Number(GetProp("box_size")); set => SetProp("box_size", value); }
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
        public CheckBoxStyle Style { get => EnumNames.Of<CheckBoxStyle>(GetProp("style"), EnumNames.CheckBoxStyle); set => SetProp("style", EnumNames.CheckBoxStyle[(int)value]); }
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

    /// <summary>Um Circle de um form.</summary>
    public class Circle(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color Stroke { get => Color.FromJson(GetProp("stroke")); set => SetProp("stroke", value.ToJson()); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
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
        public CircleStyle Style { get => EnumNames.Of<CircleStyle>(GetProp("style"), EnumNames.CircleStyle); set => SetProp("style", EnumNames.CircleStyle[(int)value]); }
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

    /// <summary>Um ComboBox de um form.</summary>
    public class ComboBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string[] Items { get => ListOf(GetProp("items")); set => SetProp("items", string.Join(";", value)); }
        public string Values { get => Text(GetProp("values")); set => SetProp("values", value); }
        public long ItemIndex { get => (long)Number(GetProp("item_index")); set => SetProp("item_index", value); }
        public TextAlign HAlign { get => EnumNames.Of<TextAlign>(GetProp("h_align"), EnumNames.TextAlign); set => SetProp("h_align", EnumNames.TextAlign[(int)value]); }
        public Trimming Trimming { get => EnumNames.Of<Trimming>(GetProp("trimming"), EnumNames.Trimming); set => SetProp("trimming", EnumNames.Trimming[(int)value]); }
        public ComboArrow Arrow { get => EnumNames.Of<ComboArrow>(GetProp("arrow"), EnumNames.ComboArrow); set => SetProp("arrow", EnumNames.ComboArrow[(int)value]); }
        public bool Sensitive { get => Bool(GetProp("sensitive")); set => SetProp("sensitive", value); }
        public string[] Groups { get => ListOf(GetProp("groups")); set => SetProp("groups", string.Join(";", value)); }
        public string[] DisabledItems { get => ListOf(GetProp("disabled_items")); set => SetProp("disabled_items", string.Join(";", value)); }
        public long DropDownCount { get => (long)Number(GetProp("drop_down_count")); set => SetProp("drop_down_count", value); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public string Value { get => Text(GetProp("value")); set => SetProp("value", value); }
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
        public ComboBoxStyle Style { get => EnumNames.Of<ComboBoxStyle>(GetProp("style"), EnumNames.ComboBoxStyle); set => SetProp("style", EnumNames.ComboBoxStyle[(int)value]); }
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

    /// <summary>Um Divider de um form.</summary>
    public class Divider(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public bool Vertical { get => Bool(GetProp("vertical")); set => SetProp("vertical", value); }
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public double Thickness { get => Number(GetProp("thickness")); set => SetProp("thickness", value); }
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
        public DividerStyle Style { get => EnumNames.Of<DividerStyle>(GetProp("style"), EnumNames.DividerStyle); set => SetProp("style", EnumNames.DividerStyle[(int)value]); }
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
        public bool SpanMonitors { get => Bool(GetProp("span_monitors")); set => SetProp("span_monitors", value); }
        public bool PhysicalPixels { get => Bool(GetProp("physical_pixels")); set => SetProp("physical_pixels", value); }
        public bool ActivateOnShow { get => Bool(GetProp("activate_on_show")); set => SetProp("activate_on_show", value); }
        public bool ForceForeground { get => Bool(GetProp("force_foreground")); set => SetProp("force_foreground", value); }
        public CloseAction CloseAction { get => EnumNames.Of<CloseAction>(GetProp("close_action"), EnumNames.CloseAction); set => SetProp("close_action", EnumNames.CloseAction[(int)value]); }
        public double FrameRate { get => Number(GetProp("frame_rate")); set => SetProp("frame_rate", value); }
        public FormOrientation Orientation { get => EnumNames.Of<FormOrientation>(GetProp("orientation"), EnumNames.FormOrientation); set => SetProp("orientation", EnumNames.FormOrientation[(int)value]); }
        public string OnShow { get => Text(GetProp("on_show")); set => SetProp("on_show", value); }
        public string OnWindowCreated { get => Text(GetProp("on_window_created")); set => SetProp("on_window_created", value); }
        public string OnClose { get => Text(GetProp("on_close")); set => SetProp("on_close", value); }
        public string OnActivate { get => Text(GetProp("on_activate")); set => SetProp("on_activate", value); }
        public string OnDeactivate { get => Text(GetProp("on_deactivate")); set => SetProp("on_deactivate", value); }
        public string OnMinimize { get => Text(GetProp("on_minimize")); set => SetProp("on_minimize", value); }
        public string OnRestore { get => Text(GetProp("on_restore")); set => SetProp("on_restore", value); }
        public string OnNavigate { get => Text(GetProp("on_navigate")); set => SetProp("on_navigate", value); }
        public string OnGpuChange { get => Text(GetProp("on_gpu_change")); set => SetProp("on_gpu_change", value); }
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
        public string GpuAdapter() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapter", ["args"] = new JsonArray() }));
        public string[] GpuAdapters() => ListOf(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapters", ["args"] = new JsonArray() }));
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

    /// <summary>Um MainMenu de um form.</summary>
    public class MainMenu(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
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
        public MainMenuStyle Style { get => EnumNames.Of<MainMenuStyle>(GetProp("style"), EnumNames.MainMenuStyle); set => SetProp("style", EnumNames.MainMenuStyle[(int)value]); }
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

    /// <summary>Um MenuItem de um form.</summary>
    public class MenuItem(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public string Shortcut { get => Text(GetProp("shortcut")); set => SetProp("shortcut", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public bool AutoCheck { get => Bool(GetProp("auto_check")); set => SetProp("auto_check", value); }
        public string Icon { get => Text(GetProp("icon")); set => SetProp("icon", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Danger { get => Bool(GetProp("danger")); set => SetProp("danger", value); }
        public bool Header { get => Bool(GetProp("header")); set => SetProp("header", value); }
        public AylisComponent? Content { get => GetRef("content"); set => SetRef("content", value); }
        public string OnClick { get => Text(GetProp("on_click")); set => SetProp("on_click", value); }
    }

    /// <summary>Um OpenDialog de um form.</summary>
    public class OpenDialog(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public string InitialDir { get => Text(GetProp("initial_dir")); set => SetProp("initial_dir", value); }
        public string FileName { get => Text(GetProp("file_name")); set => SetProp("file_name", value); }
        public bool Multiple { get => Bool(GetProp("multiple")); set => SetProp("multiple", value); }
        public string OnSelect { get => Text(GetProp("on_select")); set => SetProp("on_select", value); }
        public string OnCancel { get => Text(GetProp("on_cancel")); set => SetProp("on_cancel", value); }
        public void Execute() => Aylis.Send("execute", new JsonObject { ["target"] = Id });
    }

    /// <summary>Um PaintCanvas de um form.</summary>
    public class PaintCanvas(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public PaintTool Tool { get => EnumNames.Of<PaintTool>(GetProp("tool"), EnumNames.PaintTool); set => SetProp("tool", EnumNames.PaintTool[(int)value]); }
        public Color Color { get => Color.FromJson(GetProp("color")); set => SetProp("color", value.ToJson()); }
        public Color BackColor { get => Color.FromJson(GetProp("back_color")); set => SetProp("back_color", value.ToJson()); }
        public long BrushSize { get => (long)Number(GetProp("brush_size")); set => SetProp("brush_size", value); }
        public bool Filled { get => Bool(GetProp("filled")); set => SetProp("filled", value); }
        public double Zoom { get => Number(GetProp("zoom")); set => SetProp("zoom", value); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public string OnPick { get => Text(GetProp("on_pick")); set => SetProp("on_pick", value); }
        public string OnCursor { get => Text(GetProp("on_cursor")); set => SetProp("on_cursor", value); }
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
        public PaintCanvasStyle Style { get => EnumNames.Of<PaintCanvasStyle>(GetProp("style"), EnumNames.PaintCanvasStyle); set => SetProp("style", EnumNames.PaintCanvasStyle[(int)value]); }
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
        public bool Undo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "undo", ["args"] = new JsonArray() }));
        public bool Redo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "redo", ["args"] = new JsonArray() }));
        public bool CanUndo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "can_undo", ["args"] = new JsonArray() }));
        public bool CanRedo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "can_redo", ["args"] = new JsonArray() }));
        public global::App.Forms.ImageSize? Size() => global::App.Forms.ImageSize.FromText(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "size", ["args"] = new JsonArray() }));
        public string Checksum() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "checksum", ["args"] = new JsonArray() }));
        public bool Clear() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "clear", ["args"] = new JsonArray() }));
        public bool Resize(long width, long height) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "resize", ["args"] = new JsonArray(width, height) }));
        public bool New(long width, long height) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "new", ["args"] = new JsonArray(width, height) }));
        public bool Open(string path) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "open", ["args"] = new JsonArray(path) }));
        public bool Save(string path) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "save", ["args"] = new JsonArray(path) }));
        public bool SelectAll() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "select_all", ["args"] = new JsonArray() }));
        public bool HasSelection() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "has_selection", ["args"] = new JsonArray() }));
        public bool Copy() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "copy", ["args"] = new JsonArray() }));
        public bool Cut() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "cut", ["args"] = new JsonArray() }));
        public bool Paste() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "paste", ["args"] = new JsonArray() }));
        public bool Delete() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "delete", ["args"] = new JsonArray() }));
        public bool Deselect() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "deselect", ["args"] = new JsonArray() }));
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
        public Color Fill { get => Color.FromJson(GetProp("fill")); set => SetProp("fill", value.ToJson()); }
        public Color Fill2 { get => Color.FromJson(GetProp("fill2")); set => SetProp("fill2", value.ToJson()); }
        public Color FillMid { get => Color.FromJson(GetProp("fill_mid")); set => SetProp("fill_mid", value.ToJson()); }
        public double FillMidAt { get => Number(GetProp("fill_mid_at")); set => SetProp("fill_mid_at", value); }
        public double GradientAngle { get => Number(GetProp("gradient_angle")); set => SetProp("gradient_angle", value); }
        public bool GradientRadial { get => Bool(GetProp("gradient_radial")); set => SetProp("gradient_radial", value); }
        public Color Stroke { get => Color.FromJson(GetProp("stroke")); set => SetProp("stroke", value.ToJson()); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
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

    /// <summary>Um SaveDialog de um form.</summary>
    public class SaveDialog(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public string InitialDir { get => Text(GetProp("initial_dir")); set => SetProp("initial_dir", value); }
        public string FileName { get => Text(GetProp("file_name")); set => SetProp("file_name", value); }
        public bool Multiple { get => Bool(GetProp("multiple")); set => SetProp("multiple", value); }
        public string OnSelect { get => Text(GetProp("on_select")); set => SetProp("on_select", value); }
        public string OnCancel { get => Text(GetProp("on_cancel")); set => SetProp("on_cancel", value); }
        public void Execute() => Aylis.Send("execute", new JsonObject { ["target"] = Id });
    }

    /// <summary>Um ScrollBox de um form.</summary>
    public class ScrollBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public Overscroll Overscroll { get => EnumNames.Of<Overscroll>(GetProp("overscroll"), EnumNames.Overscroll); set => SetProp("overscroll", EnumNames.Overscroll[(int)value]); }
        public bool Horizontal { get => Bool(GetProp("horizontal")); set => SetProp("horizontal", value); }
        public bool PullToRefresh { get => Bool(GetProp("pull_to_refresh")); set => SetProp("pull_to_refresh", value); }
        public bool Refreshing { get => Bool(GetProp("refreshing")); set => SetProp("refreshing", value); }
        public double WheelStep { get => Number(GetProp("wheel_step")); set => SetProp("wheel_step", value); }
        public SmoothScroll Smooth { get => EnumNames.Of<SmoothScroll>(GetProp("smooth"), EnumNames.SmoothScroll); set => SetProp("smooth", EnumNames.SmoothScroll[(int)value]); }
        public bool Gutter { get => Bool(GetProp("gutter")); set => SetProp("gutter", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public string OnEndReached { get => Text(GetProp("on_end_reached")); set => SetProp("on_end_reached", value); }
        public string OnRefresh { get => Text(GetProp("on_refresh")); set => SetProp("on_refresh", value); }
        public string OnScroll { get => Text(GetProp("on_scroll")); set => SetProp("on_scroll", value); }
        public double ScrollPosition { get => Number(GetProp("scroll_position")); set => SetProp("scroll_position", value); }
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
        public ScrollBoxStyle Style { get => EnumNames.Of<ScrollBoxStyle>(GetProp("style"), EnumNames.ScrollBoxStyle); set => SetProp("style", EnumNames.ScrollBoxStyle[(int)value]); }
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
        public void ScrollTo(AylisComponent component) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "scroll_to", ["args"] = new JsonArray(component.Id) });
        public string VisibleComponents(AylisComponent list, bool fully) => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "visible_components", ["args"] = new JsonArray(list.Id, fully) }));
    }

    /// <summary>Um StatusBar de um form.</summary>
    public class StatusBar(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
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
        public StatusBarStyle Style { get => EnumNames.Of<StatusBarStyle>(GetProp("style"), EnumNames.StatusBarStyle); set => SetProp("style", EnumNames.StatusBarStyle[(int)value]); }
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

    /// <summary>O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmPaintComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public Layout lytTopo = null!;
        public MainMenu mnuPrincipal = null!;
        public MenuItem mnuArquivo = null!;
        public MenuItem mnuNovo = null!;
        public MenuItem mnuAbrir = null!;
        public MenuItem mnuSalvar = null!;
        public MenuItem mnuSalvarComo = null!;
        public MenuItem mnuSep1 = null!;
        public MenuItem mnuSair = null!;
        public MenuItem mnuEditar = null!;
        public MenuItem mnuDesfazer = null!;
        public MenuItem mnuRefazer = null!;
        public MenuItem mnuSep2 = null!;
        public MenuItem mnuRecortar = null!;
        public MenuItem mnuCopiar = null!;
        public MenuItem mnuColar = null!;
        public MenuItem mnuExcluir = null!;
        public MenuItem mnuSelecionarTudo = null!;
        public MenuItem mnuSep3 = null!;
        public MenuItem mnuLimpar = null!;
        public MenuItem mnuExibir = null!;
        public MenuItem mnuAmpliar = null!;
        public MenuItem mnuReduzir = null!;
        public MenuItem mnuTamanhoReal = null!;
        public Divider divTopo = null!;
        public Button btnSalvar = null!;
        public Button btnDesfazer = null!;
        public Button btnRefazer = null!;
        public Panel pnlFaixa = null!;
        public Layout lytSelecao = null!;
        public Label lblSelecaoGrupo = null!;
        public Button btnSelecionar = null!;
        public Divider divSelecao = null!;
        public Layout lytFerramentas = null!;
        public Label lblFerramentasGrupo = null!;
        public GridLayout grdFerramentas = null!;
        public Button btnLapis = null!;
        public Button btnBalde = null!;
        public Button btnBorracha = null!;
        public Button btnContaGotas = null!;
        public Divider divFerramentas = null!;
        public Layout lytPinceis = null!;
        public Label lblPinceis = null!;
        public Button btnPincel = null!;
        public Divider divPinceis = null!;
        public Layout lytFormas = null!;
        public Label lblFormas = null!;
        public CheckBox chkPreencher = null!;
        public GridLayout grdFormas = null!;
        public Button btnLinha = null!;
        public Button btnRetangulo = null!;
        public Button btnElipse = null!;
        public Divider divFormas = null!;
        public Layout lytTamanho = null!;
        public Label lblTamanhoGrupo = null!;
        public ComboBox cbEspessura = null!;
        public Divider divTamanho = null!;
        public Layout lytCores = null!;
        public Label lblCores = null!;
        public Circle btnCor1 = null!;
        public Circle btnCor2 = null!;
        public GridLayout grdPaleta = null!;
        public Circle btnPaleta01 = null!;
        public Circle btnPaleta02 = null!;
        public Circle btnPaleta03 = null!;
        public Circle btnPaleta04 = null!;
        public Circle btnPaleta05 = null!;
        public Circle btnPaleta06 = null!;
        public Circle btnPaleta07 = null!;
        public Circle btnPaleta08 = null!;
        public Circle btnPaleta09 = null!;
        public Circle btnPaleta10 = null!;
        public Circle btnPaleta11 = null!;
        public Circle btnPaleta12 = null!;
        public Circle btnPaleta13 = null!;
        public Circle btnPaleta14 = null!;
        public Circle btnPaleta15 = null!;
        public Circle btnPaleta16 = null!;
        public Circle btnPaleta17 = null!;
        public Circle btnPaleta18 = null!;
        public Circle btnPaleta19 = null!;
        public Circle btnPaleta20 = null!;
        public StatusBar stbStatus = null!;
        public Label lblPosicao = null!;
        public Label lblTamanho = null!;
        public Label lblZoom = null!;
        public ScrollBox sbxArea = null!;
        public PaintCanvas pcDesenho = null!;
        public OpenDialog dlgAbrir = null!;
        public SaveDialog dlgSalvar = null!;

        public override void Bind(JsonArray list)
        {
            lytTopo = Resolve(list, "lytTopo", "Layout", (i, d, n) => new Layout(i, d, n));
            mnuPrincipal = Resolve(list, "mnuPrincipal", "MainMenu", (i, d, n) => new MainMenu(i, d, n));
            mnuArquivo = Resolve(list, "mnuArquivo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuNovo = Resolve(list, "mnuNovo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAbrir = Resolve(list, "mnuAbrir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSalvar = Resolve(list, "mnuSalvar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSalvarComo = Resolve(list, "mnuSalvarComo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep1 = Resolve(list, "mnuSep1", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSair = Resolve(list, "mnuSair", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuEditar = Resolve(list, "mnuEditar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuDesfazer = Resolve(list, "mnuDesfazer", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuRefazer = Resolve(list, "mnuRefazer", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep2 = Resolve(list, "mnuSep2", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuRecortar = Resolve(list, "mnuRecortar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuCopiar = Resolve(list, "mnuCopiar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuColar = Resolve(list, "mnuColar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuExcluir = Resolve(list, "mnuExcluir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSelecionarTudo = Resolve(list, "mnuSelecionarTudo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep3 = Resolve(list, "mnuSep3", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuLimpar = Resolve(list, "mnuLimpar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuExibir = Resolve(list, "mnuExibir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAmpliar = Resolve(list, "mnuAmpliar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuReduzir = Resolve(list, "mnuReduzir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuTamanhoReal = Resolve(list, "mnuTamanhoReal", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            divTopo = Resolve(list, "divTopo", "Divider", (i, d, n) => new Divider(i, d, n));
            btnSalvar = Resolve(list, "btnSalvar", "Button", (i, d, n) => new Button(i, d, n));
            btnDesfazer = Resolve(list, "btnDesfazer", "Button", (i, d, n) => new Button(i, d, n));
            btnRefazer = Resolve(list, "btnRefazer", "Button", (i, d, n) => new Button(i, d, n));
            pnlFaixa = Resolve(list, "pnlFaixa", "Panel", (i, d, n) => new Panel(i, d, n));
            lytSelecao = Resolve(list, "lytSelecao", "Layout", (i, d, n) => new Layout(i, d, n));
            lblSelecaoGrupo = Resolve(list, "lblSelecaoGrupo", "Label", (i, d, n) => new Label(i, d, n));
            btnSelecionar = Resolve(list, "btnSelecionar", "Button", (i, d, n) => new Button(i, d, n));
            divSelecao = Resolve(list, "divSelecao", "Divider", (i, d, n) => new Divider(i, d, n));
            lytFerramentas = Resolve(list, "lytFerramentas", "Layout", (i, d, n) => new Layout(i, d, n));
            lblFerramentasGrupo = Resolve(list, "lblFerramentasGrupo", "Label", (i, d, n) => new Label(i, d, n));
            grdFerramentas = Resolve(list, "grdFerramentas", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnLapis = Resolve(list, "btnLapis", "Button", (i, d, n) => new Button(i, d, n));
            btnBalde = Resolve(list, "btnBalde", "Button", (i, d, n) => new Button(i, d, n));
            btnBorracha = Resolve(list, "btnBorracha", "Button", (i, d, n) => new Button(i, d, n));
            btnContaGotas = Resolve(list, "btnContaGotas", "Button", (i, d, n) => new Button(i, d, n));
            divFerramentas = Resolve(list, "divFerramentas", "Divider", (i, d, n) => new Divider(i, d, n));
            lytPinceis = Resolve(list, "lytPinceis", "Layout", (i, d, n) => new Layout(i, d, n));
            lblPinceis = Resolve(list, "lblPinceis", "Label", (i, d, n) => new Label(i, d, n));
            btnPincel = Resolve(list, "btnPincel", "Button", (i, d, n) => new Button(i, d, n));
            divPinceis = Resolve(list, "divPinceis", "Divider", (i, d, n) => new Divider(i, d, n));
            lytFormas = Resolve(list, "lytFormas", "Layout", (i, d, n) => new Layout(i, d, n));
            lblFormas = Resolve(list, "lblFormas", "Label", (i, d, n) => new Label(i, d, n));
            chkPreencher = Resolve(list, "chkPreencher", "CheckBox", (i, d, n) => new CheckBox(i, d, n));
            grdFormas = Resolve(list, "grdFormas", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnLinha = Resolve(list, "btnLinha", "Button", (i, d, n) => new Button(i, d, n));
            btnRetangulo = Resolve(list, "btnRetangulo", "Button", (i, d, n) => new Button(i, d, n));
            btnElipse = Resolve(list, "btnElipse", "Button", (i, d, n) => new Button(i, d, n));
            divFormas = Resolve(list, "divFormas", "Divider", (i, d, n) => new Divider(i, d, n));
            lytTamanho = Resolve(list, "lytTamanho", "Layout", (i, d, n) => new Layout(i, d, n));
            lblTamanhoGrupo = Resolve(list, "lblTamanhoGrupo", "Label", (i, d, n) => new Label(i, d, n));
            cbEspessura = Resolve(list, "cbEspessura", "ComboBox", (i, d, n) => new ComboBox(i, d, n));
            divTamanho = Resolve(list, "divTamanho", "Divider", (i, d, n) => new Divider(i, d, n));
            lytCores = Resolve(list, "lytCores", "Layout", (i, d, n) => new Layout(i, d, n));
            lblCores = Resolve(list, "lblCores", "Label", (i, d, n) => new Label(i, d, n));
            btnCor1 = Resolve(list, "btnCor1", "Circle", (i, d, n) => new Circle(i, d, n));
            btnCor2 = Resolve(list, "btnCor2", "Circle", (i, d, n) => new Circle(i, d, n));
            grdPaleta = Resolve(list, "grdPaleta", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnPaleta01 = Resolve(list, "btnPaleta01", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta02 = Resolve(list, "btnPaleta02", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta03 = Resolve(list, "btnPaleta03", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta04 = Resolve(list, "btnPaleta04", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta05 = Resolve(list, "btnPaleta05", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta06 = Resolve(list, "btnPaleta06", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta07 = Resolve(list, "btnPaleta07", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta08 = Resolve(list, "btnPaleta08", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta09 = Resolve(list, "btnPaleta09", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta10 = Resolve(list, "btnPaleta10", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta11 = Resolve(list, "btnPaleta11", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta12 = Resolve(list, "btnPaleta12", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta13 = Resolve(list, "btnPaleta13", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta14 = Resolve(list, "btnPaleta14", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta15 = Resolve(list, "btnPaleta15", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta16 = Resolve(list, "btnPaleta16", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta17 = Resolve(list, "btnPaleta17", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta18 = Resolve(list, "btnPaleta18", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta19 = Resolve(list, "btnPaleta19", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta20 = Resolve(list, "btnPaleta20", "Circle", (i, d, n) => new Circle(i, d, n));
            stbStatus = Resolve(list, "stbStatus", "StatusBar", (i, d, n) => new StatusBar(i, d, n));
            lblPosicao = Resolve(list, "lblPosicao", "Label", (i, d, n) => new Label(i, d, n));
            lblTamanho = Resolve(list, "lblTamanho", "Label", (i, d, n) => new Label(i, d, n));
            lblZoom = Resolve(list, "lblZoom", "Label", (i, d, n) => new Label(i, d, n));
            sbxArea = Resolve(list, "sbxArea", "ScrollBox", (i, d, n) => new ScrollBox(i, d, n));
            pcDesenho = Resolve(list, "pcDesenho", "PaintCanvas", (i, d, n) => new PaintCanvas(i, d, n));
            dlgAbrir = Resolve(list, "dlgAbrir", "OpenDialog", (i, d, n) => new OpenDialog(i, d, n));
            dlgSalvar = Resolve(list, "dlgSalvar", "SaveDialog", (i, d, n) => new SaveDialog(i, d, n));
        }

        /// <summary>O Button deste form com o nome.</summary>
        public Button? FindButton(string name) => name switch
        {
            "btnSalvar" => btnSalvar,
            "btnDesfazer" => btnDesfazer,
            "btnRefazer" => btnRefazer,
            "btnSelecionar" => btnSelecionar,
            "btnLapis" => btnLapis,
            "btnBalde" => btnBalde,
            "btnBorracha" => btnBorracha,
            "btnContaGotas" => btnContaGotas,
            "btnPincel" => btnPincel,
            "btnLinha" => btnLinha,
            "btnRetangulo" => btnRetangulo,
            "btnElipse" => btnElipse,
            _ => null,
        };

        /// <summary>O Button que disparou o evento.</summary>
        public Button? SenderAsButton(AylisClient.AylisEvent ev) => FindButton(ev.SenderName);

        /// <summary>O CheckBox deste form com o nome.</summary>
        public CheckBox? FindCheckBox(string name) => name switch
        {
            "chkPreencher" => chkPreencher,
            _ => null,
        };

        /// <summary>O CheckBox que disparou o evento.</summary>
        public CheckBox? SenderAsCheckBox(AylisClient.AylisEvent ev) => FindCheckBox(ev.SenderName);

        /// <summary>O Circle deste form com o nome.</summary>
        public Circle? FindCircle(string name) => name switch
        {
            "btnCor1" => btnCor1,
            "btnCor2" => btnCor2,
            "btnPaleta01" => btnPaleta01,
            "btnPaleta02" => btnPaleta02,
            "btnPaleta03" => btnPaleta03,
            "btnPaleta04" => btnPaleta04,
            "btnPaleta05" => btnPaleta05,
            "btnPaleta06" => btnPaleta06,
            "btnPaleta07" => btnPaleta07,
            "btnPaleta08" => btnPaleta08,
            "btnPaleta09" => btnPaleta09,
            "btnPaleta10" => btnPaleta10,
            "btnPaleta11" => btnPaleta11,
            "btnPaleta12" => btnPaleta12,
            "btnPaleta13" => btnPaleta13,
            "btnPaleta14" => btnPaleta14,
            "btnPaleta15" => btnPaleta15,
            "btnPaleta16" => btnPaleta16,
            "btnPaleta17" => btnPaleta17,
            "btnPaleta18" => btnPaleta18,
            "btnPaleta19" => btnPaleta19,
            "btnPaleta20" => btnPaleta20,
            _ => null,
        };

        /// <summary>O Circle que disparou o evento.</summary>
        public Circle? SenderAsCircle(AylisClient.AylisEvent ev) => FindCircle(ev.SenderName);

        /// <summary>O ComboBox deste form com o nome.</summary>
        public ComboBox? FindComboBox(string name) => name switch
        {
            "cbEspessura" => cbEspessura,
            _ => null,
        };

        /// <summary>O ComboBox que disparou o evento.</summary>
        public ComboBox? SenderAsComboBox(AylisClient.AylisEvent ev) => FindComboBox(ev.SenderName);

        /// <summary>O Divider deste form com o nome.</summary>
        public Divider? FindDivider(string name) => name switch
        {
            "divTopo" => divTopo,
            "divSelecao" => divSelecao,
            "divFerramentas" => divFerramentas,
            "divPinceis" => divPinceis,
            "divFormas" => divFormas,
            "divTamanho" => divTamanho,
            _ => null,
        };

        /// <summary>O Divider que disparou o evento.</summary>
        public Divider? SenderAsDivider(AylisClient.AylisEvent ev) => FindDivider(ev.SenderName);

        /// <summary>O GridLayout deste form com o nome.</summary>
        public GridLayout? FindGridLayout(string name) => name switch
        {
            "grdFerramentas" => grdFerramentas,
            "grdFormas" => grdFormas,
            "grdPaleta" => grdPaleta,
            _ => null,
        };

        /// <summary>O GridLayout que disparou o evento.</summary>
        public GridLayout? SenderAsGridLayout(AylisClient.AylisEvent ev) => FindGridLayout(ev.SenderName);

        /// <summary>O Label deste form com o nome.</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblSelecaoGrupo" => lblSelecaoGrupo,
            "lblFerramentasGrupo" => lblFerramentasGrupo,
            "lblPinceis" => lblPinceis,
            "lblFormas" => lblFormas,
            "lblTamanhoGrupo" => lblTamanhoGrupo,
            "lblCores" => lblCores,
            "lblPosicao" => lblPosicao,
            "lblTamanho" => lblTamanho,
            "lblZoom" => lblZoom,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome.</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytTopo" => lytTopo,
            "lytSelecao" => lytSelecao,
            "lytFerramentas" => lytFerramentas,
            "lytPinceis" => lytPinceis,
            "lytFormas" => lytFormas,
            "lytTamanho" => lytTamanho,
            "lytCores" => lytCores,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O MainMenu deste form com o nome.</summary>
        public MainMenu? FindMainMenu(string name) => name switch
        {
            "mnuPrincipal" => mnuPrincipal,
            _ => null,
        };

        /// <summary>O MainMenu que disparou o evento.</summary>
        public MainMenu? SenderAsMainMenu(AylisClient.AylisEvent ev) => FindMainMenu(ev.SenderName);

        /// <summary>O MenuItem deste form com o nome.</summary>
        public MenuItem? FindMenuItem(string name) => name switch
        {
            "mnuArquivo" => mnuArquivo,
            "mnuNovo" => mnuNovo,
            "mnuAbrir" => mnuAbrir,
            "mnuSalvar" => mnuSalvar,
            "mnuSalvarComo" => mnuSalvarComo,
            "mnuSep1" => mnuSep1,
            "mnuSair" => mnuSair,
            "mnuEditar" => mnuEditar,
            "mnuDesfazer" => mnuDesfazer,
            "mnuRefazer" => mnuRefazer,
            "mnuSep2" => mnuSep2,
            "mnuRecortar" => mnuRecortar,
            "mnuCopiar" => mnuCopiar,
            "mnuColar" => mnuColar,
            "mnuExcluir" => mnuExcluir,
            "mnuSelecionarTudo" => mnuSelecionarTudo,
            "mnuSep3" => mnuSep3,
            "mnuLimpar" => mnuLimpar,
            "mnuExibir" => mnuExibir,
            "mnuAmpliar" => mnuAmpliar,
            "mnuReduzir" => mnuReduzir,
            "mnuTamanhoReal" => mnuTamanhoReal,
            _ => null,
        };

        /// <summary>O MenuItem que disparou o evento.</summary>
        public MenuItem? SenderAsMenuItem(AylisClient.AylisEvent ev) => FindMenuItem(ev.SenderName);

        /// <summary>O OpenDialog deste form com o nome.</summary>
        public OpenDialog? FindOpenDialog(string name) => name switch
        {
            "dlgAbrir" => dlgAbrir,
            _ => null,
        };

        /// <summary>O OpenDialog que disparou o evento.</summary>
        public OpenDialog? SenderAsOpenDialog(AylisClient.AylisEvent ev) => FindOpenDialog(ev.SenderName);

        /// <summary>O PaintCanvas deste form com o nome.</summary>
        public PaintCanvas? FindPaintCanvas(string name) => name switch
        {
            "pcDesenho" => pcDesenho,
            _ => null,
        };

        /// <summary>O PaintCanvas que disparou o evento.</summary>
        public PaintCanvas? SenderAsPaintCanvas(AylisClient.AylisEvent ev) => FindPaintCanvas(ev.SenderName);

        /// <summary>O Panel deste form com o nome.</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlFaixa" => pnlFaixa,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O SaveDialog deste form com o nome.</summary>
        public SaveDialog? FindSaveDialog(string name) => name switch
        {
            "dlgSalvar" => dlgSalvar,
            _ => null,
        };

        /// <summary>O SaveDialog que disparou o evento.</summary>
        public SaveDialog? SenderAsSaveDialog(AylisClient.AylisEvent ev) => FindSaveDialog(ev.SenderName);

        /// <summary>O ScrollBox deste form com o nome.</summary>
        public ScrollBox? FindScrollBox(string name) => name switch
        {
            "sbxArea" => sbxArea,
            _ => null,
        };

        /// <summary>O ScrollBox que disparou o evento.</summary>
        public ScrollBox? SenderAsScrollBox(AylisClient.AylisEvent ev) => FindScrollBox(ev.SenderName);

        /// <summary>O StatusBar deste form com o nome.</summary>
        public StatusBar? FindStatusBar(string name) => name switch
        {
            "stbStatus" => stbStatus,
            _ => null,
        };

        /// <summary>O StatusBar que disparou o evento.</summary>
        public StatusBar? SenderAsStatusBar(AylisClient.AylisEvent ev) => FindStatusBar(ev.SenderName);
    }
}
