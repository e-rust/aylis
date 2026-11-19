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
    public enum ApplyMode
    {
        Cached,
        OnPost,
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
    public enum CharCase
    {
        Normal,
        Upper,
        Lower,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ChartKind
    {
        Line,
        Bar,
        Area,
        Pie,
        Donut,
        Sparkline,
        Heatmap,
        Scatter,
        StackedBar,
        Waterfall,
        Funnel,
        Treemap,
        Radar,
        HorizontalBar,
        Bubble,
        PolarArea,
        StackedLine,
        StackedArea,
        Mixed,
        Sankey,
        Waffle,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ChartSeriesFill
    {
        None,
        Origin,
        Start,
        End,
        Series,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ChartSeriesKind
    {
        Line,
        Bar,
        Area,
        Scatter,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ChartSeriesPoint
    {
        Circle,
        Square,
        Triangle,
        Diamond,
        Cross,
        None,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum CloseAction
    {
        Hide,
        Free,
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
    public enum DBChartStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum DataGridStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FieldKind
    {
        Data,
        Calculated,
        Lookup,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum FieldType
    {
        String,
        Memo,
        Integer,
        Float,
        Currency,
        Boolean,
        Date,
        Time,
        Datetime,
        Blob,
        AutoInc,
        Timestamp,
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
    public enum GridTotalFunction
    {
        Sum,
        Avg,
        Min,
        Max,
        Count,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum GroupBoxStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum HorizontalWheel
    {
        Shift,
        Always,
        None,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ImageFit
    {
        Contain,
        Cover,
        Stretch,
        Center,
        Original,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum ImageStyle
    {
        Default,
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
    public enum LegendPosition
    {
        TopRight,
        TopLeft,
        Top,
        Bottom,
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
    public enum Rounding
    {
        HalfUp,
        HalfEven,
        Truncate,
        Up,
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
    public enum SplitterStyle
    {
        Default,
    }

    /// <summary>Os valores de um enum do acesso tipado.</summary>
    public enum SvgStyle
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
    public enum TitleAlign
    {
        Left,
        Center,
        Right,
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
    public enum ValueFormat
    {
        Short,
        Full,
        Percent,
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
        public static readonly string[] ApplyMode = ["cached", "on_post"];
        public static readonly string[] BlendMode = ["normal", "multiply", "screen", "overlay", "darken", "lighten", "color_dodge", "color_burn", "hard_light", "soft_light", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];
        public static readonly string[] BorderStyle = ["sizeable", "single", "none", "custom"];
        public static readonly string[] CharCase = ["normal", "upper", "lower"];
        public static readonly string[] ChartKind = ["line", "bar", "area", "pie", "donut", "sparkline", "heatmap", "scatter", "stacked_bar", "waterfall", "funnel", "treemap", "radar", "horizontal_bar", "bubble", "polar_area", "stacked_line", "stacked_area", "mixed", "sankey", "waffle"];
        public static readonly string[] ChartSeriesFill = ["none", "origin", "start", "end", "series"];
        public static readonly string[] ChartSeriesKind = ["line", "bar", "area", "scatter"];
        public static readonly string[] ChartSeriesPoint = ["circle", "square", "triangle", "diamond", "cross", "none"];
        public static readonly string[] CloseAction = ["hide", "free"];
        public static readonly string[] CursorShape = ["default", "pointer", "text", "wait", "crosshair", "move", "not_allowed", "resize_horizontal", "resize_vertical", "resize_ne_sw", "resize_nw_se", "grab", "grabbing", "none"];
        public static readonly string[] DBChartStyle = [""];
        public static readonly string[] DataGridStyle = [""];
        public static readonly string[] FieldKind = ["data", "calculated", "lookup"];
        public static readonly string[] FieldType = ["string", "memo", "integer", "float", "currency", "boolean", "date", "time", "datetime", "blob", "auto_inc", "timestamp"];
        public static readonly string[] FontWeight = ["default", "thin", "light", "normal", "medium", "semibold", "bold", "black"];
        public static readonly string[] FormOrientation = ["project", "any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"];
        public static readonly string[] FormPosition = ["center", "screen_center", "designed", "default", "top_left", "top_right", "bottom_left", "bottom_right"];
        public static readonly string[] FormPresent = ["project", "window", "composition", "composed"];
        public static readonly string[] FormStyle = [""];
        public static readonly string[] GridTotalFunction = ["sum", "avg", "min", "max", "count"];
        public static readonly string[] GroupBoxStyle = [""];
        public static readonly string[] HorizontalWheel = ["shift", "always", "none"];
        public static readonly string[] ImageFit = ["contain", "cover", "stretch", "center", "original"];
        public static readonly string[] ImageStyle = [""];
        public static readonly string[] LabelStyle = ["", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title"];
        public static readonly string[] LayerCaching = ["project", "on", "off"];
        public static readonly string[] LayoutStyle = [""];
        public static readonly string[] LegendPosition = ["top_right", "top_left", "top", "bottom"];
        public static readonly string[] Overscroll = ["auto", "glow", "stretch", "bounce", "none"];
        public static readonly string[] PanelStyle = ["", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"];
        public static readonly string[] Rounding = ["half_up", "half_even", "truncate", "up"];
        public static readonly string[] SelfAlign = ["stretch", "start", "center", "end"];
        public static readonly string[] SplitterStyle = [""];
        public static readonly string[] SvgStyle = [""];
        public static readonly string[] TextAlign = ["leading", "center", "trailing", "justify"];
        public static readonly string[] TextCase = ["none", "upper", "lower"];
        public static readonly string[] ThemeName = ["", "Aylis Light", "Aylis Dark", "Material 3 Light", "Material 3 Dark", "Material 2 Light", "Material 2 Dark", "Android 7 Light", "Android 7 Dark", "iOS 26 Light", "iOS 26 Dark", "iOS 13 Light", "iOS 13 Dark", "iOS 7 Light", "WinUI 3 Light", "WinUI 3 Dark", "Windows 11 Light", "Windows 11 Dark", "Windows 10 Light", "Windows 10 Dark", "Win32", "Windows 7", "Windows Phone Dark", "Windows Phone Light", "Vuetify 3 Light", "Vuetify 3 Dark", "Bootstrap 5 Light", "Bootstrap 5 Dark", "shadcn/ui Light", "shadcn/ui Dark", "Ant Design 5 Light", "Ant Design 5 Dark", "Chakra UI Light", "Chakra UI Dark", "Quasar 2 Light", "Quasar 2 Dark"];
        public static readonly string[] TitleAlign = ["left", "center", "right"];
        public static readonly string[] Transition = ["none", "fade", "slide_left", "slide_right", "slide_up", "slide_down"];
        public static readonly string[] Trimming = ["none", "character", "word"];
        public static readonly string[] ValueFormat = ["short", "full", "percent"];
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
    public sealed record ChartSeries(string Name, double[] Values, double[] X, double[] Radius, ChartSeriesKind Kind, bool RightAxis, Color Color, double Width, double[] Dash, ChartSeriesPoint Point, ChartSeriesFill Fill, long FillSeries, bool Hidden)
    {
        /// <summary>Com os padrões: <c>new ChartSeries { Campo = valor }</c> preenche só os dados.</summary>
        public ChartSeries() : this(FromJson(null)) { }

        internal static ChartSeries FromJson(JsonNode? n) => new(Components.Text(n?["name"]), n?["values"] is JsonArray lvalues ? lvalues.Select(x => Components.Number(x)).ToArray() : [], n?["x"] is JsonArray lx ? lx.Select(x => Components.Number(x)).ToArray() : [], n?["radius"] is JsonArray lradius ? lradius.Select(x => Components.Number(x)).ToArray() : [], EnumNames.Of<ChartSeriesKind>(n?["kind"], EnumNames.ChartSeriesKind), Components.Bool(n?["right_axis"]), Color.FromJson(n?["color"]), Components.Number(n?["width"]), n?["dash"] is JsonArray ldash ? ldash.Select(x => Components.Number(x)).ToArray() : [], EnumNames.Of<ChartSeriesPoint>(n?["point"], EnumNames.ChartSeriesPoint), EnumNames.Of<ChartSeriesFill>(n?["fill"], EnumNames.ChartSeriesFill), (long)Components.Number(n?["fill_series"]), Components.Bool(n?["hidden"]));

        internal JsonObject ToJson() => new() { ["name"] = JsonValue.Create(Name), ["values"] = new JsonArray(Values.Select(x => (JsonNode?)JsonValue.Create(x)).ToArray()), ["x"] = new JsonArray(X.Select(x => (JsonNode?)JsonValue.Create(x)).ToArray()), ["radius"] = new JsonArray(Radius.Select(x => (JsonNode?)JsonValue.Create(x)).ToArray()), ["kind"] = JsonValue.Create(EnumNames.ChartSeriesKind[(int)Kind]), ["right_axis"] = JsonValue.Create(RightAxis), ["color"] = Color.ToJson(), ["width"] = JsonValue.Create(Width), ["dash"] = new JsonArray(Dash.Select(x => (JsonNode?)JsonValue.Create(x)).ToArray()), ["point"] = JsonValue.Create(EnumNames.ChartSeriesPoint[(int)Point]), ["fill"] = JsonValue.Create(EnumNames.ChartSeriesFill[(int)Fill]), ["fill_series"] = JsonValue.Create(FillSeries), ["hidden"] = JsonValue.Create(Hidden) };

        internal static ChartSeries? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static ChartSeries[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(ChartSeries? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<ChartSeries> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record ColumnSpec(string Field, string Title, double Width, bool ReadOnly, Color Color, Color TextColor)
    {
        /// <summary>Com os padrões: <c>new ColumnSpec { Campo = valor }</c> preenche só os dados.</summary>
        public ColumnSpec() : this(FromJson(null)) { }

        internal static ColumnSpec FromJson(JsonNode? n) => new(Components.Text(n?["field"]), Components.Text(n?["title"]), Components.Number(n?["width"]), Components.Bool(n?["read_only"]), Color.FromJson(n?["color"]), Color.FromJson(n?["text_color"]));

        internal JsonObject ToJson() => new() { ["field"] = JsonValue.Create(Field), ["title"] = JsonValue.Create(Title), ["width"] = JsonValue.Create(Width), ["read_only"] = JsonValue.Create(ReadOnly), ["color"] = Color.ToJson(), ["text_color"] = TextColor.ToJson() };

        internal static ColumnSpec? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static ColumnSpec[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(ColumnSpec? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<ColumnSpec> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record FilterParam(string Name, string Value)
    {
        /// <summary>Com os padrões: <c>new FilterParam { Campo = valor }</c> preenche só os dados.</summary>
        public FilterParam() : this(FromJson(null)) { }

        internal static FilterParam FromJson(JsonNode? n) => new(Components.Text(n?["name"]), Components.Text(n?["value"]));

        internal JsonObject ToJson() => new() { ["name"] = JsonValue.Create(Name), ["value"] = JsonValue.Create(Value) };

        internal static FilterParam? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static FilterParam[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(FilterParam? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<FilterParam> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
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
    public sealed record GridTotal(string Field, GridTotalFunction Function)
    {
        /// <summary>Com os padrões: <c>new GridTotal { Campo = valor }</c> preenche só os dados.</summary>
        public GridTotal() : this(FromJson(null)) { }

        internal static GridTotal FromJson(JsonNode? n) => new(Components.Text(n?["field"]), EnumNames.Of<GridTotalFunction>(n?["function"], EnumNames.GridTotalFunction));

        internal JsonObject ToJson() => new() { ["field"] = JsonValue.Create(Field), ["function"] = JsonValue.Create(EnumNames.GridTotalFunction[(int)Function]) };

        internal static GridTotal? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static GridTotal[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(GridTotal? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<GridTotal> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
    }

    /// <summary>Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).</summary>
    public sealed record IndexDef(string Name, string Fields)
    {
        /// <summary>Com os padrões: <c>new IndexDef { Campo = valor }</c> preenche só os dados.</summary>
        public IndexDef() : this(FromJson(null)) { }

        internal static IndexDef FromJson(JsonNode? n) => new(Components.Text(n?["name"]), Components.Text(n?["fields"]));

        internal JsonObject ToJson() => new() { ["name"] = JsonValue.Create(Name), ["fields"] = JsonValue.Create(Fields) };

        internal static IndexDef? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static IndexDef[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(IndexDef? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<IndexDef> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
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
    public sealed record MasterField(string Field, string MasterFieldValue)
    {
        /// <summary>Com os padrões: <c>new MasterField { Campo = valor }</c> preenche só os dados.</summary>
        public MasterField() : this(FromJson(null)) { }

        internal static MasterField FromJson(JsonNode? n) => new(Components.Text(n?["field"]), Components.Text(n?["master_field"]));

        internal JsonObject ToJson() => new() { ["field"] = JsonValue.Create(Field), ["master_field"] = JsonValue.Create(MasterFieldValue) };

        internal static MasterField? FromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonObject o ? FromJson(o) : null;

        internal static MasterField[] ListFromText(JsonNode? v) => Components.Text(v) is { Length: > 0 } t && JsonNode.Parse(t) is JsonArray a ? a.Select(FromJson).ToArray() : [];

        internal static string ToText(MasterField? r) => r?.ToJson().ToJsonString() ?? "";

        internal static string ListToText(System.Collections.Generic.IEnumerable<MasterField> items) => new JsonArray(items.Select(x => (JsonNode?)x.ToJson()).ToArray()).ToJsonString();
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

        public static AylisUi.FrmIBGEComponents frmIBGE = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmIBGE", (aylis, id, name) => new AylisUi.FrmIBGEComponents(aylis, id, name), form => frmIBGE = (AylisUi.FrmIBGEComponents)form);
        }
    }
}

namespace App.Forms.AylisUi
{
    using static App.Forms.Components;

    /// <summary>Um ClientDataSet de um form.</summary>
    public class ClientDataSet(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string KeyFields { get => Text(GetProp("key_fields")); set => SetProp("key_fields", value); }
        public ApplyMode ApplyMode { get => EnumNames.Of<ApplyMode>(GetProp("apply_mode"), EnumNames.ApplyMode); set => SetProp("apply_mode", EnumNames.ApplyMode[(int)value]); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Active { get => Bool(GetProp("active")); set => SetProp("active", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public bool Filtered { get => Bool(GetProp("filtered")); set => SetProp("filtered", value); }
        public string IndexFieldNames { get => Text(GetProp("index_field_names")); set => SetProp("index_field_names", value); }
        public global::App.Forms.IndexDef[] Indexes { get => global::App.Forms.IndexDef.ListFromText(GetProp("indexes")); set => SetProp("indexes", global::App.Forms.IndexDef.ListToText(value)); }
        public string IndexName { get => Text(GetProp("index_name")); set => SetProp("index_name", value); }
        public global::App.Forms.FilterParam[] Params { get => global::App.Forms.FilterParam.ListFromText(GetProp("params")); set => SetProp("params", global::App.Forms.FilterParam.ListToText(value)); }
        public string StatusFilter { get => Text(GetProp("status_filter")); set => SetProp("status_filter", value); }
        public AylisComponent? MasterSource { get => GetRef("master_source"); set => SetRef("master_source", value); }
        public global::App.Forms.MasterField[] MasterFields { get => global::App.Forms.MasterField.ListFromText(GetProp("master_fields")); set => SetProp("master_fields", global::App.Forms.MasterField.ListToText(value)); }
        public bool ApplySeparately { get => Bool(GetProp("apply_separately")); set => SetProp("apply_separately", value); }
        public long RecordTotal { get => (long)Number(GetProp("record_total")); set => SetProp("record_total", value); }
        public long MaxRecords { get => (long)Number(GetProp("max_records")); set => SetProp("max_records", value); }
        public string PartialNotice => Text(GetProp("partial_notice"));
        public string OnMasterChange { get => Text(GetProp("on_master_change")); set => SetProp("on_master_change", value); }
        public string AfterScroll { get => Text(GetProp("after_scroll")); set => SetProp("after_scroll", value); }
        public string AfterPost { get => Text(GetProp("after_post")); set => SetProp("after_post", value); }
        public string OnStateChange { get => Text(GetProp("on_state_change")); set => SetProp("on_state_change", value); }
        public string OnReconcileError { get => Text(GetProp("on_reconcile_error")); set => SetProp("on_reconcile_error", value); }
        public string OnError { get => Text(GetProp("on_error")); set => SetProp("on_error", value); }
        public string OnFetch { get => Text(GetProp("on_fetch")); set => SetProp("on_fetch", value); }
        public string OnServerQuery { get => Text(GetProp("on_server_query")); set => SetProp("on_server_query", value); }
    }

    /// <summary>Um DBChart de um form.</summary>
    public class DBChart(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public AylisComponent? DataSource { get => GetRef("data_source"); set => SetRef("data_source", value); }
        public string LabelField { get => Text(GetProp("label_field")); set => SetProp("label_field", value); }
        public string[] ValueFields { get => ListOf(GetProp("value_fields")); set => SetProp("value_fields", string.Join(";", value)); }
        public ChartKind Kind { get => EnumNames.Of<ChartKind>(GetProp("kind"), EnumNames.ChartKind); set => SetProp("kind", EnumNames.ChartKind[(int)value]); }
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string[] Labels { get => ListOf(GetProp("labels")); set => SetProp("labels", string.Join(";", value)); }
        public global::App.Forms.ChartSeries[] Series { get => global::App.Forms.ChartSeries.ListFromText(GetProp("series")); set => SetProp("series", global::App.Forms.ChartSeries.ListToText(value)); }
        public string Json { get => Text(GetProp("json")); set => SetProp("json", value); }
        public bool Legend { get => Bool(GetProp("legend")); set => SetProp("legend", value); }
        public bool Stepped { get => Bool(GetProp("stepped")); set => SetProp("stepped", value); }
        public double Tension { get => Number(GetProp("tension")); set => SetProp("tension", value); }
        public bool Animate { get => Bool(GetProp("animate")); set => SetProp("animate", value); }
        public bool Zoom { get => Bool(GetProp("zoom")); set => SetProp("zoom", value); }
        public bool AutoScale { get => Bool(GetProp("auto_scale")); set => SetProp("auto_scale", value); }
        public double YMin { get => Number(GetProp("y_min")); set => SetProp("y_min", value); }
        public double YMax { get => Number(GetProp("y_max")); set => SetProp("y_max", value); }
        public bool LogScale { get => Bool(GetProp("log_scale")); set => SetProp("log_scale", value); }
        public bool Reverse { get => Bool(GetProp("reverse")); set => SetProp("reverse", value); }
        public bool TimeAxis { get => Bool(GetProp("time_axis")); set => SetProp("time_axis", value); }
        public long ClickedSeries { get => (long)Number(GetProp("clicked_series")); set => SetProp("clicked_series", value); }
        public long ClickedIndex { get => (long)Number(GetProp("clicked_index")); set => SetProp("clicked_index", value); }
        public ValueFormat ValueFormat { get => EnumNames.Of<ValueFormat>(GetProp("value_format"), EnumNames.ValueFormat); set => SetProp("value_format", EnumNames.ValueFormat[(int)value]); }
        public double LabelAngle { get => Number(GetProp("label_angle")); set => SetProp("label_angle", value); }
        public TitleAlign TitleAlign { get => EnumNames.Of<TitleAlign>(GetProp("title_align"), EnumNames.TitleAlign); set => SetProp("title_align", EnumNames.TitleAlign[(int)value]); }
        public bool TitleBold { get => Bool(GetProp("title_bold")); set => SetProp("title_bold", value); }
        public double AreaOpacity { get => Number(GetProp("area_opacity")); set => SetProp("area_opacity", value); }
        public LegendPosition LegendPosition { get => EnumNames.Of<LegendPosition>(GetProp("legend_position"), EnumNames.LegendPosition); set => SetProp("legend_position", EnumNames.LegendPosition[(int)value]); }
        public bool Stacked { get => Bool(GetProp("stacked")); set => SetProp("stacked", value); }
        public string Colors { get => Text(GetProp("colors")); set => SetProp("colors", value); }
        public string CenterText { get => Text(GetProp("center_text")); set => SetProp("center_text", value); }
        public string SliceLabels { get => Text(GetProp("slice_labels")); set => SetProp("slice_labels", value); }
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
        public DBChartStyle Style { get => EnumNames.Of<DBChartStyle>(GetProp("style"), EnumNames.DBChartStyle); set => SetProp("style", EnumNames.DBChartStyle[(int)value]); }
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

    /// <summary>Um DataGrid de um form.</summary>
    public class DataGrid(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public Overscroll Overscroll { get => EnumNames.Of<Overscroll>(GetProp("overscroll"), EnumNames.Overscroll); set => SetProp("overscroll", EnumNames.Overscroll[(int)value]); }
        public AylisComponent? DataSource { get => GetRef("data_source"); set => SetRef("data_source", value); }
        public global::App.Forms.ColumnSpec[] Columns { get => global::App.Forms.ColumnSpec.ListFromText(GetProp("columns")); set => SetProp("columns", global::App.Forms.ColumnSpec.ListToText(value)); }
        public double RowHeight { get => Number(GetProp("row_height")); set => SetProp("row_height", value); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Sortable { get => Bool(GetProp("sortable")); set => SetProp("sortable", value); }
        public bool ColumnMove { get => Bool(GetProp("column_move")); set => SetProp("column_move", value); }
        public long FixedColumns { get => (long)Number(GetProp("fixed_columns")); set => SetProp("fixed_columns", value); }
        public bool FilterRow { get => Bool(GetProp("filter_row")); set => SetProp("filter_row", value); }
        public bool MultiSelect { get => Bool(GetProp("multi_select")); set => SetProp("multi_select", value); }
        public global::App.Forms.GridTotal[] Footer { get => global::App.Forms.GridTotal.ListFromText(GetProp("footer")); set => SetProp("footer", global::App.Forms.GridTotal.ListToText(value)); }
        public bool IncrementalSearch { get => Bool(GetProp("incremental_search")); set => SetProp("incremental_search", value); }
        public Color CursorColor { get => Color.FromJson(GetProp("cursor_color")); set => SetProp("cursor_color", value.ToJson()); }
        public bool Indicator { get => Bool(GetProp("indicator")); set => SetProp("indicator", value); }
        public bool HighlightCurrent { get => Bool(GetProp("highlight_current")); set => SetProp("highlight_current", value); }
        public HorizontalWheel HorizontalWheel { get => EnumNames.Of<HorizontalWheel>(GetProp("horizontal_wheel"), EnumNames.HorizontalWheel); set => SetProp("horizontal_wheel", EnumNames.HorizontalWheel[(int)value]); }
        public string OnError { get => Text(GetProp("on_error")); set => SetProp("on_error", value); }
        public string OnTitleClick { get => Text(GetProp("on_title_click")); set => SetProp("on_title_click", value); }
        public string OnSelectionChange { get => Text(GetProp("on_selection_change")); set => SetProp("on_selection_change", value); }
        public string OnColumnMoved { get => Text(GetProp("on_column_moved")); set => SetProp("on_column_moved", value); }
        public double ScrollLeft { get => Number(GetProp("scroll_left")); set => SetProp("scroll_left", value); }
        public long TopRow { get => (long)Number(GetProp("top_row")); set => SetProp("top_row", value); }
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
        public DataGridStyle Style { get => EnumNames.Of<DataGridStyle>(GetProp("style"), EnumNames.DataGridStyle); set => SetProp("style", EnumNames.DataGridStyle[(int)value]); }
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

    /// <summary>Um DataSource de um form.</summary>
    public class DataSource(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public AylisComponent? DataSet { get => GetRef("data_set"); set => SetRef("data_set", value); }
        public bool AutoEdit { get => Bool(GetProp("auto_edit")); set => SetProp("auto_edit", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public string OnDataChange { get => Text(GetProp("on_data_change")); set => SetProp("on_data_change", value); }
        public string OnStateChange { get => Text(GetProp("on_state_change")); set => SetProp("on_state_change", value); }
    }

    /// <summary>Um Field de um form.</summary>
    public class Field(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string FieldName { get => Text(GetProp("field_name")); set => SetProp("field_name", value); }
        public FieldType Type { get => EnumNames.Of<FieldType>(GetProp("type"), EnumNames.FieldType); set => SetProp("type", EnumNames.FieldType[(int)value]); }
        public long Size { get => (long)Number(GetProp("size")); set => SetProp("size", value); }
        public long Scale { get => (long)Number(GetProp("scale")); set => SetProp("scale", value); }
        public Rounding Rounding { get => EnumNames.Of<Rounding>(GetProp("rounding"), EnumNames.Rounding); set => SetProp("rounding", EnumNames.Rounding[(int)value]); }
        public string DisplayLabel { get => Text(GetProp("display_label")); set => SetProp("display_label", value); }
        public double DisplayWidth { get => Number(GetProp("display_width")); set => SetProp("display_width", value); }
        public string DisplayFormat { get => Text(GetProp("display_format")); set => SetProp("display_format", value); }
        public string EditFormat { get => Text(GetProp("edit_format")); set => SetProp("edit_format", value); }
        public TextAlign Alignment { get => EnumNames.Of<TextAlign>(GetProp("alignment"), EnumNames.TextAlign); set => SetProp("alignment", EnumNames.TextAlign[(int)value]); }
        public bool Required { get => Bool(GetProp("required")); set => SetProp("required", value); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public double Min { get => Number(GetProp("min")); set => SetProp("min", value); }
        public double Max { get => Number(GetProp("max")); set => SetProp("max", value); }
        public string DefaultValue { get => Text(GetProp("default_value")); set => SetProp("default_value", value); }
        public string DisplayValues { get => Text(GetProp("display_values")); set => SetProp("display_values", value); }
        public CharCase CharCase { get => EnumNames.Of<CharCase>(GetProp("char_case"), EnumNames.CharCase); set => SetProp("char_case", EnumNames.CharCase[(int)value]); }
        public string EditMask { get => Text(GetProp("edit_mask")); set => SetProp("edit_mask", value); }
        public bool Sensitive { get => Bool(GetProp("sensitive")); set => SetProp("sensitive", value); }
        public FieldKind Kind { get => EnumNames.Of<FieldKind>(GetProp("kind"), EnumNames.FieldKind); set => SetProp("kind", EnumNames.FieldKind[(int)value]); }
        public string Expression { get => Text(GetProp("expression")); set => SetProp("expression", value); }
        public AylisComponent? LookupDataSet { get => GetRef("lookup_data_set"); set => SetRef("lookup_data_set", value); }
        public string KeyFields { get => Text(GetProp("key_fields")); set => SetProp("key_fields", value); }
        public string LookupKeyFields { get => Text(GetProp("lookup_key_fields")); set => SetProp("lookup_key_fields", value); }
        public string LookupResultField { get => Text(GetProp("lookup_result_field")); set => SetProp("lookup_result_field", value); }
        public string OnChange { get => Text(GetProp("on_change")); set => SetProp("on_change", value); }
        public string OnGetText { get => Text(GetProp("on_get_text")); set => SetProp("on_get_text", value); }
        public string OnSetText { get => Text(GetProp("on_set_text")); set => SetProp("on_set_text", value); }
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

    /// <summary>Um GroupBox de um form.</summary>
    public class GroupBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
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
        public GroupBoxStyle Style { get => EnumNames.Of<GroupBoxStyle>(GetProp("style"), EnumNames.GroupBoxStyle); set => SetProp("style", EnumNames.GroupBoxStyle[(int)value]); }
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

    /// <summary>Um Image de um form.</summary>
    public class Image(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string ImageValue { get => Text(GetProp("image")); set => SetProp("image", value); }
        public ImageFit Fit { get => EnumNames.Of<ImageFit>(GetProp("fit"), EnumNames.ImageFit); set => SetProp("fit", EnumNames.ImageFit[(int)value]); }
        public double Radius { get => Number(GetProp("radius")); set => SetProp("radius", value); }
        public bool Zoomable { get => Bool(GetProp("zoomable")); set => SetProp("zoomable", value); }
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
        public ImageStyle Style { get => EnumNames.Of<ImageStyle>(GetProp("style"), EnumNames.ImageStyle); set => SetProp("style", EnumNames.ImageStyle[(int)value]); }
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

    /// <summary>Um Splitter de um form.</summary>
    public class Splitter(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public double MinSize { get => Number(GetProp("min_size")); set => SetProp("min_size", value); }
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
        public SplitterStyle Style { get => EnumNames.Of<SplitterStyle>(GetProp("style"), EnumNames.SplitterStyle); set => SetProp("style", EnumNames.SplitterStyle[(int)value]); }
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

    /// <summary>Um Svg de um form.</summary>
    public class Svg(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Source { get => Text(GetProp("source")); set => SetProp("source", value); }
        public Color Color { get => Color.FromJson(GetProp("color")); set => SetProp("color", value.ToJson()); }
        public ImageFit Fit { get => EnumNames.Of<ImageFit>(GetProp("fit"), EnumNames.ImageFit); set => SetProp("fit", EnumNames.ImageFit[(int)value]); }
        public string[] Highlight { get => ListOf(GetProp("highlight")); set => SetProp("highlight", string.Join(";", value)); }
        public Color HighlightColor { get => Color.FromJson(GetProp("highlight_color")); set => SetProp("highlight_color", value.ToJson()); }
        public string ZoomTo { get => Text(GetProp("zoom_to")); set => SetProp("zoom_to", value); }
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
        public SvgStyle Style { get => EnumNames.Of<SvgStyle>(GetProp("style"), EnumNames.SvgStyle); set => SetProp("style", EnumNames.SvgStyle[(int)value]); }
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

    /// <summary>O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmIBGEComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public ClientDataSet cdsEstados = null!;
        public Field cdsEstadosId = null!;
        public Field cdsEstadosSigla = null!;
        public Field cdsEstadosNome = null!;
        public Field cdsEstadosRegiao = null!;
        public Field cdsEstadosPopulacao = null!;
        public Field cdsEstadosArea = null!;
        public Field cdsEstadosDensidade = null!;
        public Field cdsEstadosPib = null!;
        public Field cdsEstadosPibPerCapita = null!;
        public Field cdsEstadosEstimativa = null!;
        public Field cdsEstadosEstimativaAno = null!;
        public Field cdsEstadosMunicipios = null!;
        public Field cdsEstadosBandeira = null!;
        public DataSource dsEstados = null!;
        public ClientDataSet cdsMunicipios = null!;
        public Field cdsMunicipiosId = null!;
        public Field cdsMunicipiosUfId = null!;
        public Field cdsMunicipiosUf = null!;
        public Field cdsMunicipiosNome = null!;
        public Field cdsMunicipiosMesorregiao = null!;
        public Field cdsMunicipiosMicrorregiao = null!;
        public Field cdsMunicipiosRegiaoIntermediaria = null!;
        public Field cdsMunicipiosRegiaoImediata = null!;
        public Field cdsMunicipiosPopulacao = null!;
        public Field cdsMunicipiosArea = null!;
        public Field cdsMunicipiosDensidade = null!;
        public Field cdsMunicipiosPib = null!;
        public Field cdsMunicipiosPibParticipacao = null!;
        public Field cdsMunicipiosPibPerCapita = null!;
        public Field cdsMunicipiosEstimativa = null!;
        public Field cdsMunicipiosEstimativaAno = null!;
        public Field cdsMunicipiosDistritos = null!;
        public Field cdsMunicipiosBandeira = null!;
        public DataSource dsMunicipios = null!;
        public ClientDataSet cdsDistritos = null!;
        public Field cdsDistritosId = null!;
        public Field cdsDistritosMunicipioId = null!;
        public Field cdsDistritosNome = null!;
        public DataSource dsDistritos = null!;
        public ClientDataSet cdsPopulacao = null!;
        public Field cdsPopulacaoAno = null!;
        public Field cdsPopulacaoHabitantes = null!;
        public DataSource dsPopulacao = null!;
        public ClientDataSet cdsMaiores = null!;
        public Field cdsMaioresNome = null!;
        public Field cdsMaioresHabitantes = null!;
        public DataSource dsMaiores = null!;
        public Label lblFonte = null!;
        public Layout lytListas = null!;
        public GroupBox gbEstados = null!;
        public DataGrid grdEstados = null!;
        public GroupBox gbDistritos = null!;
        public DataGrid grdDistritos = null!;
        public GroupBox gbMunicipios = null!;
        public DataGrid grdMunicipios = null!;
        public Splitter splListas = null!;
        public Layout lytPainel = null!;
        public Panel pnlEstado = null!;
        public Svg svgBrasil = null!;
        public Layout lytBandeiraEstado = null!;
        public Label lblEstado = null!;
        public Label lblRegiao = null!;
        public Image imgBandeiraEstado = null!;
        public Layout lytNumerosEstado = null!;
        public Label dbtEstadoPopulacao = null!;
        public Label lblEstadoEstimativa = null!;
        public Label dbtEstadoEstimativa = null!;
        public Label dbtEstadoArea = null!;
        public Label dbtEstadoDensidade = null!;
        public Label dbtEstadoPib = null!;
        public Label dbtEstadoPibPerCapita = null!;
        public Label dbtEstadoMunicipios = null!;
        public Panel pnlMunicipio = null!;
        public Svg svgMunicipio = null!;
        public Svg svgEstado = null!;
        public Layout lytBandeiraMunicipio = null!;
        public Label lblMunicipio = null!;
        public Label lblCodigo = null!;
        public Image imgBandeiraMunicipio = null!;
        public Label lblSemBandeira = null!;
        public Layout lytNumerosMunicipio = null!;
        public Label dbtMesorregiao = null!;
        public Label dbtMicrorregiao = null!;
        public Label dbtRegiaoImediata = null!;
        public Label dbtPopulacao = null!;
        public Label lblEstimativa = null!;
        public Label dbtEstimativa = null!;
        public Label dbtArea = null!;
        public Label dbtDensidade = null!;
        public Label dbtPib = null!;
        public Label dbtPibPerCapita = null!;
        public Label dbtPibParticipacao = null!;
        public Layout lytGraficos = null!;
        public DBChart dbcPopulacao = null!;
        public DBChart dbcMaiores = null!;
        public DBChart dbcEstados = null!;

        public override void Bind(JsonArray list)
        {
            cdsEstados = Resolve(list, "cdsEstados", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsEstadosId = Resolve(list, "cdsEstadosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosSigla = Resolve(list, "cdsEstadosSigla", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosNome = Resolve(list, "cdsEstadosNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosRegiao = Resolve(list, "cdsEstadosRegiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPopulacao = Resolve(list, "cdsEstadosPopulacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosArea = Resolve(list, "cdsEstadosArea", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosDensidade = Resolve(list, "cdsEstadosDensidade", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPib = Resolve(list, "cdsEstadosPib", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPibPerCapita = Resolve(list, "cdsEstadosPibPerCapita", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosEstimativa = Resolve(list, "cdsEstadosEstimativa", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosEstimativaAno = Resolve(list, "cdsEstadosEstimativaAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosMunicipios = Resolve(list, "cdsEstadosMunicipios", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosBandeira = Resolve(list, "cdsEstadosBandeira", "Field", (i, d, n) => new Field(i, d, n));
            dsEstados = Resolve(list, "dsEstados", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsMunicipios = Resolve(list, "cdsMunicipios", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsMunicipiosId = Resolve(list, "cdsMunicipiosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosUfId = Resolve(list, "cdsMunicipiosUfId", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosUf = Resolve(list, "cdsMunicipiosUf", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosNome = Resolve(list, "cdsMunicipiosNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosMesorregiao = Resolve(list, "cdsMunicipiosMesorregiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosMicrorregiao = Resolve(list, "cdsMunicipiosMicrorregiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosRegiaoIntermediaria = Resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosRegiaoImediata = Resolve(list, "cdsMunicipiosRegiaoImediata", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPopulacao = Resolve(list, "cdsMunicipiosPopulacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosArea = Resolve(list, "cdsMunicipiosArea", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosDensidade = Resolve(list, "cdsMunicipiosDensidade", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPib = Resolve(list, "cdsMunicipiosPib", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPibParticipacao = Resolve(list, "cdsMunicipiosPibParticipacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPibPerCapita = Resolve(list, "cdsMunicipiosPibPerCapita", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosEstimativa = Resolve(list, "cdsMunicipiosEstimativa", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosEstimativaAno = Resolve(list, "cdsMunicipiosEstimativaAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosDistritos = Resolve(list, "cdsMunicipiosDistritos", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosBandeira = Resolve(list, "cdsMunicipiosBandeira", "Field", (i, d, n) => new Field(i, d, n));
            dsMunicipios = Resolve(list, "dsMunicipios", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsDistritos = Resolve(list, "cdsDistritos", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsDistritosId = Resolve(list, "cdsDistritosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsDistritosMunicipioId = Resolve(list, "cdsDistritosMunicipioId", "Field", (i, d, n) => new Field(i, d, n));
            cdsDistritosNome = Resolve(list, "cdsDistritosNome", "Field", (i, d, n) => new Field(i, d, n));
            dsDistritos = Resolve(list, "dsDistritos", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsPopulacao = Resolve(list, "cdsPopulacao", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsPopulacaoAno = Resolve(list, "cdsPopulacaoAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsPopulacaoHabitantes = Resolve(list, "cdsPopulacaoHabitantes", "Field", (i, d, n) => new Field(i, d, n));
            dsPopulacao = Resolve(list, "dsPopulacao", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsMaiores = Resolve(list, "cdsMaiores", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsMaioresNome = Resolve(list, "cdsMaioresNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsMaioresHabitantes = Resolve(list, "cdsMaioresHabitantes", "Field", (i, d, n) => new Field(i, d, n));
            dsMaiores = Resolve(list, "dsMaiores", "DataSource", (i, d, n) => new DataSource(i, d, n));
            lblFonte = Resolve(list, "lblFonte", "Label", (i, d, n) => new Label(i, d, n));
            lytListas = Resolve(list, "lytListas", "Layout", (i, d, n) => new Layout(i, d, n));
            gbEstados = Resolve(list, "gbEstados", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdEstados = Resolve(list, "grdEstados", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            gbDistritos = Resolve(list, "gbDistritos", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdDistritos = Resolve(list, "grdDistritos", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            gbMunicipios = Resolve(list, "gbMunicipios", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdMunicipios = Resolve(list, "grdMunicipios", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            splListas = Resolve(list, "splListas", "Splitter", (i, d, n) => new Splitter(i, d, n));
            lytPainel = Resolve(list, "lytPainel", "Layout", (i, d, n) => new Layout(i, d, n));
            pnlEstado = Resolve(list, "pnlEstado", "Panel", (i, d, n) => new Panel(i, d, n));
            svgBrasil = Resolve(list, "svgBrasil", "Svg", (i, d, n) => new Svg(i, d, n));
            lytBandeiraEstado = Resolve(list, "lytBandeiraEstado", "Layout", (i, d, n) => new Layout(i, d, n));
            lblEstado = Resolve(list, "lblEstado", "Label", (i, d, n) => new Label(i, d, n));
            lblRegiao = Resolve(list, "lblRegiao", "Label", (i, d, n) => new Label(i, d, n));
            imgBandeiraEstado = Resolve(list, "imgBandeiraEstado", "Image", (i, d, n) => new Image(i, d, n));
            lytNumerosEstado = Resolve(list, "lytNumerosEstado", "Layout", (i, d, n) => new Layout(i, d, n));
            dbtEstadoPopulacao = Resolve(list, "dbtEstadoPopulacao", "Label", (i, d, n) => new Label(i, d, n));
            lblEstadoEstimativa = Resolve(list, "lblEstadoEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoEstimativa = Resolve(list, "dbtEstadoEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoArea = Resolve(list, "dbtEstadoArea", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoDensidade = Resolve(list, "dbtEstadoDensidade", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoPib = Resolve(list, "dbtEstadoPib", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoPibPerCapita = Resolve(list, "dbtEstadoPibPerCapita", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoMunicipios = Resolve(list, "dbtEstadoMunicipios", "Label", (i, d, n) => new Label(i, d, n));
            pnlMunicipio = Resolve(list, "pnlMunicipio", "Panel", (i, d, n) => new Panel(i, d, n));
            svgMunicipio = Resolve(list, "svgMunicipio", "Svg", (i, d, n) => new Svg(i, d, n));
            svgEstado = Resolve(list, "svgEstado", "Svg", (i, d, n) => new Svg(i, d, n));
            lytBandeiraMunicipio = Resolve(list, "lytBandeiraMunicipio", "Layout", (i, d, n) => new Layout(i, d, n));
            lblMunicipio = Resolve(list, "lblMunicipio", "Label", (i, d, n) => new Label(i, d, n));
            lblCodigo = Resolve(list, "lblCodigo", "Label", (i, d, n) => new Label(i, d, n));
            imgBandeiraMunicipio = Resolve(list, "imgBandeiraMunicipio", "Image", (i, d, n) => new Image(i, d, n));
            lblSemBandeira = Resolve(list, "lblSemBandeira", "Label", (i, d, n) => new Label(i, d, n));
            lytNumerosMunicipio = Resolve(list, "lytNumerosMunicipio", "Layout", (i, d, n) => new Layout(i, d, n));
            dbtMesorregiao = Resolve(list, "dbtMesorregiao", "Label", (i, d, n) => new Label(i, d, n));
            dbtMicrorregiao = Resolve(list, "dbtMicrorregiao", "Label", (i, d, n) => new Label(i, d, n));
            dbtRegiaoImediata = Resolve(list, "dbtRegiaoImediata", "Label", (i, d, n) => new Label(i, d, n));
            dbtPopulacao = Resolve(list, "dbtPopulacao", "Label", (i, d, n) => new Label(i, d, n));
            lblEstimativa = Resolve(list, "lblEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstimativa = Resolve(list, "dbtEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtArea = Resolve(list, "dbtArea", "Label", (i, d, n) => new Label(i, d, n));
            dbtDensidade = Resolve(list, "dbtDensidade", "Label", (i, d, n) => new Label(i, d, n));
            dbtPib = Resolve(list, "dbtPib", "Label", (i, d, n) => new Label(i, d, n));
            dbtPibPerCapita = Resolve(list, "dbtPibPerCapita", "Label", (i, d, n) => new Label(i, d, n));
            dbtPibParticipacao = Resolve(list, "dbtPibParticipacao", "Label", (i, d, n) => new Label(i, d, n));
            lytGraficos = Resolve(list, "lytGraficos", "Layout", (i, d, n) => new Layout(i, d, n));
            dbcPopulacao = Resolve(list, "dbcPopulacao", "DBChart", (i, d, n) => new DBChart(i, d, n));
            dbcMaiores = Resolve(list, "dbcMaiores", "DBChart", (i, d, n) => new DBChart(i, d, n));
            dbcEstados = Resolve(list, "dbcEstados", "DBChart", (i, d, n) => new DBChart(i, d, n));
        }

        /// <summary>O ClientDataSet deste form com o nome.</summary>
        public ClientDataSet? FindClientDataSet(string name) => name switch
        {
            "cdsEstados" => cdsEstados,
            "cdsMunicipios" => cdsMunicipios,
            "cdsDistritos" => cdsDistritos,
            "cdsPopulacao" => cdsPopulacao,
            "cdsMaiores" => cdsMaiores,
            _ => null,
        };

        /// <summary>O ClientDataSet que disparou o evento.</summary>
        public ClientDataSet? SenderAsClientDataSet(AylisClient.AylisEvent ev) => FindClientDataSet(ev.SenderName);

        /// <summary>O DBChart deste form com o nome.</summary>
        public DBChart? FindDBChart(string name) => name switch
        {
            "dbcPopulacao" => dbcPopulacao,
            "dbcMaiores" => dbcMaiores,
            "dbcEstados" => dbcEstados,
            _ => null,
        };

        /// <summary>O DBChart que disparou o evento.</summary>
        public DBChart? SenderAsDBChart(AylisClient.AylisEvent ev) => FindDBChart(ev.SenderName);

        /// <summary>O DataGrid deste form com o nome.</summary>
        public DataGrid? FindDataGrid(string name) => name switch
        {
            "grdEstados" => grdEstados,
            "grdDistritos" => grdDistritos,
            "grdMunicipios" => grdMunicipios,
            _ => null,
        };

        /// <summary>O DataGrid que disparou o evento.</summary>
        public DataGrid? SenderAsDataGrid(AylisClient.AylisEvent ev) => FindDataGrid(ev.SenderName);

        /// <summary>O DataSource deste form com o nome.</summary>
        public DataSource? FindDataSource(string name) => name switch
        {
            "dsEstados" => dsEstados,
            "dsMunicipios" => dsMunicipios,
            "dsDistritos" => dsDistritos,
            "dsPopulacao" => dsPopulacao,
            "dsMaiores" => dsMaiores,
            _ => null,
        };

        /// <summary>O DataSource que disparou o evento.</summary>
        public DataSource? SenderAsDataSource(AylisClient.AylisEvent ev) => FindDataSource(ev.SenderName);

        /// <summary>O Field deste form com o nome.</summary>
        public Field? FindField(string name) => name switch
        {
            "cdsEstadosId" => cdsEstadosId,
            "cdsEstadosSigla" => cdsEstadosSigla,
            "cdsEstadosNome" => cdsEstadosNome,
            "cdsEstadosRegiao" => cdsEstadosRegiao,
            "cdsEstadosPopulacao" => cdsEstadosPopulacao,
            "cdsEstadosArea" => cdsEstadosArea,
            "cdsEstadosDensidade" => cdsEstadosDensidade,
            "cdsEstadosPib" => cdsEstadosPib,
            "cdsEstadosPibPerCapita" => cdsEstadosPibPerCapita,
            "cdsEstadosEstimativa" => cdsEstadosEstimativa,
            "cdsEstadosEstimativaAno" => cdsEstadosEstimativaAno,
            "cdsEstadosMunicipios" => cdsEstadosMunicipios,
            "cdsEstadosBandeira" => cdsEstadosBandeira,
            "cdsMunicipiosId" => cdsMunicipiosId,
            "cdsMunicipiosUfId" => cdsMunicipiosUfId,
            "cdsMunicipiosUf" => cdsMunicipiosUf,
            "cdsMunicipiosNome" => cdsMunicipiosNome,
            "cdsMunicipiosMesorregiao" => cdsMunicipiosMesorregiao,
            "cdsMunicipiosMicrorregiao" => cdsMunicipiosMicrorregiao,
            "cdsMunicipiosRegiaoIntermediaria" => cdsMunicipiosRegiaoIntermediaria,
            "cdsMunicipiosRegiaoImediata" => cdsMunicipiosRegiaoImediata,
            "cdsMunicipiosPopulacao" => cdsMunicipiosPopulacao,
            "cdsMunicipiosArea" => cdsMunicipiosArea,
            "cdsMunicipiosDensidade" => cdsMunicipiosDensidade,
            "cdsMunicipiosPib" => cdsMunicipiosPib,
            "cdsMunicipiosPibParticipacao" => cdsMunicipiosPibParticipacao,
            "cdsMunicipiosPibPerCapita" => cdsMunicipiosPibPerCapita,
            "cdsMunicipiosEstimativa" => cdsMunicipiosEstimativa,
            "cdsMunicipiosEstimativaAno" => cdsMunicipiosEstimativaAno,
            "cdsMunicipiosDistritos" => cdsMunicipiosDistritos,
            "cdsMunicipiosBandeira" => cdsMunicipiosBandeira,
            "cdsDistritosId" => cdsDistritosId,
            "cdsDistritosMunicipioId" => cdsDistritosMunicipioId,
            "cdsDistritosNome" => cdsDistritosNome,
            "cdsPopulacaoAno" => cdsPopulacaoAno,
            "cdsPopulacaoHabitantes" => cdsPopulacaoHabitantes,
            "cdsMaioresNome" => cdsMaioresNome,
            "cdsMaioresHabitantes" => cdsMaioresHabitantes,
            _ => null,
        };

        /// <summary>O Field que disparou o evento.</summary>
        public Field? SenderAsField(AylisClient.AylisEvent ev) => FindField(ev.SenderName);

        /// <summary>O GroupBox deste form com o nome.</summary>
        public GroupBox? FindGroupBox(string name) => name switch
        {
            "gbEstados" => gbEstados,
            "gbDistritos" => gbDistritos,
            "gbMunicipios" => gbMunicipios,
            _ => null,
        };

        /// <summary>O GroupBox que disparou o evento.</summary>
        public GroupBox? SenderAsGroupBox(AylisClient.AylisEvent ev) => FindGroupBox(ev.SenderName);

        /// <summary>O Image deste form com o nome.</summary>
        public Image? FindImage(string name) => name switch
        {
            "imgBandeiraEstado" => imgBandeiraEstado,
            "imgBandeiraMunicipio" => imgBandeiraMunicipio,
            _ => null,
        };

        /// <summary>O Image que disparou o evento.</summary>
        public Image? SenderAsImage(AylisClient.AylisEvent ev) => FindImage(ev.SenderName);

        /// <summary>O Label deste form com o nome.</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblFonte" => lblFonte,
            "lblEstado" => lblEstado,
            "lblRegiao" => lblRegiao,
            "dbtEstadoPopulacao" => dbtEstadoPopulacao,
            "lblEstadoEstimativa" => lblEstadoEstimativa,
            "dbtEstadoEstimativa" => dbtEstadoEstimativa,
            "dbtEstadoArea" => dbtEstadoArea,
            "dbtEstadoDensidade" => dbtEstadoDensidade,
            "dbtEstadoPib" => dbtEstadoPib,
            "dbtEstadoPibPerCapita" => dbtEstadoPibPerCapita,
            "dbtEstadoMunicipios" => dbtEstadoMunicipios,
            "lblMunicipio" => lblMunicipio,
            "lblCodigo" => lblCodigo,
            "lblSemBandeira" => lblSemBandeira,
            "dbtMesorregiao" => dbtMesorregiao,
            "dbtMicrorregiao" => dbtMicrorregiao,
            "dbtRegiaoImediata" => dbtRegiaoImediata,
            "dbtPopulacao" => dbtPopulacao,
            "lblEstimativa" => lblEstimativa,
            "dbtEstimativa" => dbtEstimativa,
            "dbtArea" => dbtArea,
            "dbtDensidade" => dbtDensidade,
            "dbtPib" => dbtPib,
            "dbtPibPerCapita" => dbtPibPerCapita,
            "dbtPibParticipacao" => dbtPibParticipacao,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome.</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytListas" => lytListas,
            "lytPainel" => lytPainel,
            "lytBandeiraEstado" => lytBandeiraEstado,
            "lytNumerosEstado" => lytNumerosEstado,
            "lytBandeiraMunicipio" => lytBandeiraMunicipio,
            "lytNumerosMunicipio" => lytNumerosMunicipio,
            "lytGraficos" => lytGraficos,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O Panel deste form com o nome.</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlEstado" => pnlEstado,
            "pnlMunicipio" => pnlMunicipio,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O Splitter deste form com o nome.</summary>
        public Splitter? FindSplitter(string name) => name switch
        {
            "splListas" => splListas,
            _ => null,
        };

        /// <summary>O Splitter que disparou o evento.</summary>
        public Splitter? SenderAsSplitter(AylisClient.AylisEvent ev) => FindSplitter(ev.SenderName);

        /// <summary>O Svg deste form com o nome.</summary>
        public Svg? FindSvg(string name) => name switch
        {
            "svgBrasil" => svgBrasil,
            "svgMunicipio" => svgMunicipio,
            "svgEstado" => svgEstado,
            _ => null,
        };

        /// <summary>O Svg que disparou o evento.</summary>
        public Svg? SenderAsSvg(AylisClient.AylisEvent ev) => FindSvg(ev.SenderName);
    }
}
