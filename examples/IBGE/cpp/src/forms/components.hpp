// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include <cstdio>
#include <cstdlib>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

#include "../aylis.hpp"

namespace aylis_components {

// Uma data do calendário (sem fuso).
struct LocalDate {
    int year = 0, month = 0, day = 0;
    static std::optional<LocalDate> parse(const std::string& s) {
        LocalDate d;
        if (std::sscanf(s.c_str(), "%d-%d-%d", &d.year, &d.month, &d.day) != 3) return std::nullopt;
        return d;
    }
    std::string to_string() const {
        char out[16];
        std::snprintf(out, sizeof out, "%04d-%02d-%02d", year, month, day);
        return out;
    }
};

// Uma hora do dia (sem fuso).
struct LocalTime {
    int hour = 0, minute = 0, second = 0;
    static std::optional<LocalTime> parse(const std::string& s) {
        LocalTime t;
        int n = std::sscanf(s.c_str(), "%d:%d:%d", &t.hour, &t.minute, &t.second);
        if (n < 2) return std::nullopt;
        return t;
    }
    std::string to_string() const {
        char out[16];
        if (second == 0)
            std::snprintf(out, sizeof out, "%02d:%02d", hour, minute);
        else
            std::snprintf(out, sizeof out, "%02d:%02d:%02d", hour, minute, second);
        return out;
    }
};

// O dia e a hora (sem fuso).
struct LocalDateTime {
    LocalDate date;
    LocalTime time;
    static std::optional<LocalDateTime> parse(const std::string& s) {
        auto at = s.find_first_of("T ");
        auto d = LocalDate::parse(s.substr(0, at));
        if (!d) return std::nullopt;
        auto t = at == std::string::npos ? std::optional<LocalTime>(LocalTime{}) : LocalTime::parse(s.substr(at + 1));
        return LocalDateTime{*d, t.value_or(LocalTime{})};
    }
    std::string to_string() const { return date.to_string() + "T" + time.to_string(); }
};

template <class T>
std::string text_of_(const std::optional<T>& v) { return v ? v->to_string() : std::string(); }

inline std::string trim_(const std::string& s) {
    auto a = s.find_first_not_of(' '), b = s.find_last_not_of(' ');
    return a == std::string::npos ? std::string() : s.substr(a, b - a + 1);
}

inline std::vector<std::string> list_of_(const aylis::Json& v) {
    std::vector<std::string> out;
    std::string s = v.str(), item;
    std::istringstream in(s);
    while (std::getline(in, item, ';')) {
        auto a = item.find_first_not_of(' '), b = item.find_last_not_of(' ');
        if (a != std::string::npos) out.push_back(item.substr(a, b - a + 1));
    }
    return out;
}

inline std::vector<double> numbers_of_(const aylis::Json& v) {
    std::vector<double> out;
    for (const auto& s : list_of_(v)) {
        char* end = nullptr;
        double d = std::strtod(s.c_str(), &end);
        if (end && *end == '\0') out.push_back(d);
    }
    return out;
}

inline std::vector<aylis::Color> colors_of_(const aylis::Json& v) {
    std::vector<aylis::Color> out;
    for (const auto& s : list_of_(v))
        if (!s.empty() && s[0] == '#') out.push_back(aylis::Color::hex(s));
    return out;
}

inline std::string join_(const std::vector<std::string>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) out += (i ? ";" : "") + v[i];
    return out;
}

inline std::string numbers_text_(const std::vector<double>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) {
        char n[32];
        std::snprintf(n, sizeof n, "%.15g", v[i]);
        out += (i ? ";" : "") + std::string(n);
    }
    return out;
}

inline std::string colors_text_(const std::vector<aylis::Color>& v) {
    std::string out;
    for (size_t i = 0; i < v.size(); ++i) out += (i ? ";" : "") + v[i].to_json();
    return out;
}

// O intervalo de datas (o `on_change` do DateRangePicker); vazio: sem a data.
struct DateRange {
    std::optional<LocalDate> start;
    std::optional<LocalDate> end;

    static DateRange from_text(const std::string& s) {
        DateRange r;
        auto i = s.find(';');
        r.start = LocalDate::parse(s.substr(0, i));
        if (i != std::string::npos) r.end = LocalDate::parse(s.substr(i + 1));
        return r;
    }
};

// O valor da tela como o enum (o primeiro, se não for um dos nomes).
template <class E, size_t N>
E enum_of_(const aylis::Json& v, const char* const (&names)[N]) {
    std::string s = v.str();
    for (size_t i = 0; i < N; ++i)
        if (s == names[i]) return static_cast<E>(i);
    return static_cast<E>(0);
}

template <class E, size_t N>
std::vector<E> enum_list_(const aylis::Json& v, const char* const (&names)[N]) {
    std::vector<E> out;
    for (const auto& x : v.items()) {
        std::string s = x.str();
        for (size_t i = 0; i < N; ++i)
            if (s == names[i]) out.push_back(static_cast<E>(i));
    }
    return out;
}

template <class E, size_t N>
aylis::Json enum_names_(const std::vector<E>& values, const char* const (&names)[N]) {
    std::vector<aylis::Json> out;
    for (E v : values) out.emplace_back(names[static_cast<int>(v)]);
    return aylis::Json::array(out);
}

// Os valores de um enum do acesso tipado.
enum class Align { None, Top, Bottom, Left, Right, MostTop, MostBottom, MostLeft, MostRight, Client, Contents, Center, VertCenter, HorzCenter, Horizontal, Vertical, Fit };
inline constexpr const char* Align_names[] = {"none", "top", "bottom", "left", "right", "most_top", "most_bottom", "most_left", "most_right", "client", "contents", "center", "vert_center", "horz_center", "horizontal", "vertical", "fit"};
inline const char* to_string(Align v) { return Align_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Anchor { Left, Top, Right, Bottom };
inline constexpr const char* Anchor_names[] = {"left", "top", "right", "bottom"};
inline const char* to_string(Anchor v) { return Anchor_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ApplyMode { Cached, OnPost };
inline constexpr const char* ApplyMode_names[] = {"cached", "on_post"};
inline const char* to_string(ApplyMode v) { return ApplyMode_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class BlendMode { Normal, Multiply, Screen, Overlay, Darken, Lighten, ColorDodge, ColorBurn, HardLight, SoftLight, Difference, Exclusion, Hue, Saturation, Color, Luminosity };
inline constexpr const char* BlendMode_names[] = {"normal", "multiply", "screen", "overlay", "darken", "lighten", "color_dodge", "color_burn", "hard_light", "soft_light", "difference", "exclusion", "hue", "saturation", "color", "luminosity"};
inline const char* to_string(BlendMode v) { return BlendMode_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class BorderStyle { Sizeable, Single, None, Custom };
inline constexpr const char* BorderStyle_names[] = {"sizeable", "single", "none", "custom"};
inline const char* to_string(BorderStyle v) { return BorderStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class CharCase { Normal, Upper, Lower };
inline constexpr const char* CharCase_names[] = {"normal", "upper", "lower"};
inline const char* to_string(CharCase v) { return CharCase_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ChartKind { Line, Bar, Area, Pie, Donut, Sparkline, Heatmap, Scatter, StackedBar, Waterfall, Funnel, Treemap, Radar, HorizontalBar, Bubble, PolarArea, StackedLine, StackedArea, Mixed, Sankey, Waffle };
inline constexpr const char* ChartKind_names[] = {"line", "bar", "area", "pie", "donut", "sparkline", "heatmap", "scatter", "stacked_bar", "waterfall", "funnel", "treemap", "radar", "horizontal_bar", "bubble", "polar_area", "stacked_line", "stacked_area", "mixed", "sankey", "waffle"};
inline const char* to_string(ChartKind v) { return ChartKind_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ChartSeriesFill { None, Origin, Start, End, Series };
inline constexpr const char* ChartSeriesFill_names[] = {"none", "origin", "start", "end", "series"};
inline const char* to_string(ChartSeriesFill v) { return ChartSeriesFill_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ChartSeriesKind { Line, Bar, Area, Scatter };
inline constexpr const char* ChartSeriesKind_names[] = {"line", "bar", "area", "scatter"};
inline const char* to_string(ChartSeriesKind v) { return ChartSeriesKind_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ChartSeriesPoint { Circle, Square, Triangle, Diamond, Cross, None };
inline constexpr const char* ChartSeriesPoint_names[] = {"circle", "square", "triangle", "diamond", "cross", "none"};
inline const char* to_string(ChartSeriesPoint v) { return ChartSeriesPoint_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class CloseAction { Hide, Free };
inline constexpr const char* CloseAction_names[] = {"hide", "free"};
inline const char* to_string(CloseAction v) { return CloseAction_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class CursorShape { Default, Pointer, Text, Wait, Crosshair, Move, NotAllowed, ResizeHorizontal, ResizeVertical, ResizeNeSw, ResizeNwSe, Grab, Grabbing, None };
inline constexpr const char* CursorShape_names[] = {"default", "pointer", "text", "wait", "crosshair", "move", "not_allowed", "resize_horizontal", "resize_vertical", "resize_ne_sw", "resize_nw_se", "grab", "grabbing", "none"};
inline const char* to_string(CursorShape v) { return CursorShape_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class DBChartStyle { Default };
inline constexpr const char* DBChartStyle_names[] = {""};
inline const char* to_string(DBChartStyle v) { return DBChartStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class DataGridStyle { Default };
inline constexpr const char* DataGridStyle_names[] = {""};
inline const char* to_string(DataGridStyle v) { return DataGridStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FieldKind { Data, Calculated, Lookup };
inline constexpr const char* FieldKind_names[] = {"data", "calculated", "lookup"};
inline const char* to_string(FieldKind v) { return FieldKind_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FieldType { String, Memo, Integer, Float, Currency, Boolean, Date, Time, Datetime, Blob, AutoInc, Timestamp };
inline constexpr const char* FieldType_names[] = {"string", "memo", "integer", "float", "currency", "boolean", "date", "time", "datetime", "blob", "auto_inc", "timestamp"};
inline const char* to_string(FieldType v) { return FieldType_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FontWeight { Default, Thin, Light, Normal, Medium, Semibold, Bold, Black };
inline constexpr const char* FontWeight_names[] = {"default", "thin", "light", "normal", "medium", "semibold", "bold", "black"};
inline const char* to_string(FontWeight v) { return FontWeight_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FormOrientation { Project, Any, Portrait, Landscape, SensorPortrait, SensorLandscape };
inline constexpr const char* FormOrientation_names[] = {"project", "any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"};
inline const char* to_string(FormOrientation v) { return FormOrientation_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FormPosition { Center, ScreenCenter, Designed, Default, TopLeft, TopRight, BottomLeft, BottomRight };
inline constexpr const char* FormPosition_names[] = {"center", "screen_center", "designed", "default", "top_left", "top_right", "bottom_left", "bottom_right"};
inline const char* to_string(FormPosition v) { return FormPosition_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FormPresent { Project, Window, Composition, Composed };
inline constexpr const char* FormPresent_names[] = {"project", "window", "composition", "composed"};
inline const char* to_string(FormPresent v) { return FormPresent_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class FormStyle { Default };
inline constexpr const char* FormStyle_names[] = {""};
inline const char* to_string(FormStyle v) { return FormStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class GridTotalFunction { Sum, Avg, Min, Max, Count };
inline constexpr const char* GridTotalFunction_names[] = {"sum", "avg", "min", "max", "count"};
inline const char* to_string(GridTotalFunction v) { return GridTotalFunction_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class GroupBoxStyle { Default };
inline constexpr const char* GroupBoxStyle_names[] = {""};
inline const char* to_string(GroupBoxStyle v) { return GroupBoxStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class HorizontalWheel { Shift, Always, None };
inline constexpr const char* HorizontalWheel_names[] = {"shift", "always", "none"};
inline const char* to_string(HorizontalWheel v) { return HorizontalWheel_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ImageFit { Contain, Cover, Stretch, Center, Original };
inline constexpr const char* ImageFit_names[] = {"contain", "cover", "stretch", "center", "original"};
inline const char* to_string(ImageFit v) { return ImageFit_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ImageStyle { Default };
inline constexpr const char* ImageStyle_names[] = {""};
inline const char* to_string(ImageStyle v) { return ImageStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class LabelStyle { Default, Caption, Error, Heading, Kbd, Link, Mono, Muted, Title };
inline constexpr const char* LabelStyle_names[] = {"", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title"};
inline const char* to_string(LabelStyle v) { return LabelStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class LayerCaching { Project, On, Off };
inline constexpr const char* LayerCaching_names[] = {"project", "on", "off"};
inline const char* to_string(LayerCaching v) { return LayerCaching_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class LayoutStyle { Default };
inline constexpr const char* LayoutStyle_names[] = {""};
inline const char* to_string(LayoutStyle v) { return LayoutStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class LegendPosition { TopRight, TopLeft, Top, Bottom };
inline constexpr const char* LegendPosition_names[] = {"top_right", "top_left", "top", "bottom"};
inline const char* to_string(LegendPosition v) { return LegendPosition_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Overscroll { Auto, Glow, Stretch, Bounce, None };
inline constexpr const char* Overscroll_names[] = {"auto", "glow", "stretch", "bounce", "none"};
inline const char* to_string(Overscroll v) { return Overscroll_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class PanelStyle { Default, Appbar, Card, Filled, Flat, Glass, Outlined, Overlay, Reveal, Sidebar };
inline constexpr const char* PanelStyle_names[] = {"", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"};
inline const char* to_string(PanelStyle v) { return PanelStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Rounding { HalfUp, HalfEven, Truncate, Up };
inline constexpr const char* Rounding_names[] = {"half_up", "half_even", "truncate", "up"};
inline const char* to_string(Rounding v) { return Rounding_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class SelfAlign { Stretch, Start, Center, End };
inline constexpr const char* SelfAlign_names[] = {"stretch", "start", "center", "end"};
inline const char* to_string(SelfAlign v) { return SelfAlign_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class SplitterStyle { Default };
inline constexpr const char* SplitterStyle_names[] = {""};
inline const char* to_string(SplitterStyle v) { return SplitterStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class SvgStyle { Default };
inline constexpr const char* SvgStyle_names[] = {""};
inline const char* to_string(SvgStyle v) { return SvgStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TextAlign { Leading, Center, Trailing, Justify };
inline constexpr const char* TextAlign_names[] = {"leading", "center", "trailing", "justify"};
inline const char* to_string(TextAlign v) { return TextAlign_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TextCase { None, Upper, Lower };
inline constexpr const char* TextCase_names[] = {"none", "upper", "lower"};
inline const char* to_string(TextCase v) { return TextCase_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ThemeName { Default, AylisLight, AylisDark, Material3Light, Material3Dark, Material2Light, Material2Dark, Android7Light, Android7Dark, IOS26Light, IOS26Dark, IOS13Light, IOS13Dark, IOS7Light, WinUI3Light, WinUI3Dark, Windows11Light, Windows11Dark, Windows10Light, Windows10Dark, Win32, Windows7, WindowsPhoneDark, WindowsPhoneLight, Vuetify3Light, Vuetify3Dark, Bootstrap5Light, Bootstrap5Dark, ShadcnUiLight, ShadcnUiDark, AntDesign5Light, AntDesign5Dark, ChakraUILight, ChakraUIDark, Quasar2Light, Quasar2Dark };
inline constexpr const char* ThemeName_names[] = {"", "Aylis Light", "Aylis Dark", "Material 3 Light", "Material 3 Dark", "Material 2 Light", "Material 2 Dark", "Android 7 Light", "Android 7 Dark", "iOS 26 Light", "iOS 26 Dark", "iOS 13 Light", "iOS 13 Dark", "iOS 7 Light", "WinUI 3 Light", "WinUI 3 Dark", "Windows 11 Light", "Windows 11 Dark", "Windows 10 Light", "Windows 10 Dark", "Win32", "Windows 7", "Windows Phone Dark", "Windows Phone Light", "Vuetify 3 Light", "Vuetify 3 Dark", "Bootstrap 5 Light", "Bootstrap 5 Dark", "shadcn/ui Light", "shadcn/ui Dark", "Ant Design 5 Light", "Ant Design 5 Dark", "Chakra UI Light", "Chakra UI Dark", "Quasar 2 Light", "Quasar 2 Dark"};
inline const char* to_string(ThemeName v) { return ThemeName_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TitleAlign { Left, Center, Right };
inline constexpr const char* TitleAlign_names[] = {"left", "center", "right"};
inline const char* to_string(TitleAlign v) { return TitleAlign_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Transition { None, Fade, SlideLeft, SlideRight, SlideUp, SlideDown };
inline constexpr const char* Transition_names[] = {"none", "fade", "slide_left", "slide_right", "slide_up", "slide_down"};
inline const char* to_string(Transition v) { return Transition_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Trimming { None, Character, Word };
inline constexpr const char* Trimming_names[] = {"none", "character", "word"};
inline const char* to_string(Trimming v) { return Trimming_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ValueFormat { Short, Full, Percent };
inline constexpr const char* ValueFormat_names[] = {"short", "full", "percent"};
inline const char* to_string(ValueFormat v) { return ValueFormat_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class WindowCorners { Default, Round, Small, Square };
inline constexpr const char* WindowCorners_names[] = {"default", "round", "small", "square"};
inline const char* to_string(WindowCorners v) { return WindowCorners_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class WindowShape { Rectangle, Rounded, Ellipse, Path };
inline constexpr const char* WindowShape_names[] = {"rectangle", "rounded", "ellipse", "path"};
inline const char* to_string(WindowShape v) { return WindowShape_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class WindowState { Normal, Maximized, Minimized, FullScreen };
inline constexpr const char* WindowState_names[] = {"normal", "maximized", "minimized", "full_screen"};
inline const char* to_string(WindowState v) { return WindowState_names[static_cast<int>(v)]; }
struct ChartSeries;
struct ColumnSpec;
struct FilterParam;
struct GestureEvent;
struct GridTotal;
struct IndexDef;
struct KeyEvent;
struct MasterField;
struct PointerEvent;
struct SwipeEvent;
struct WheelEvent;

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct ChartSeries {
    std::string name;
    std::vector<double> values;
    std::vector<double> x;
    std::vector<double> radius;
    ChartSeriesKind kind = static_cast<ChartSeriesKind>(0);
    bool right_axis = false;
    aylis::Color color;
    double width = 0;
    std::vector<double> dash;
    ChartSeriesPoint point = static_cast<ChartSeriesPoint>(0);
    ChartSeriesFill fill = static_cast<ChartSeriesFill>(0);
    long long fill_series = 0;
    bool hidden = false;

    static ChartSeries from_json(const aylis::Json& v) {
        ChartSeries r;
        r.name = v["name"].str();
        for (const auto& x : v["values"].items()) r.values.push_back(x.number());
        for (const auto& x : v["x"].items()) r.x.push_back(x.number());
        for (const auto& x : v["radius"].items()) r.radius.push_back(x.number());
        r.kind = enum_of_<ChartSeriesKind>(v["kind"], ChartSeriesKind_names);
        r.right_axis = v["right_axis"].boolean();
        r.color = aylis::Color::from_json(v["color"]);
        r.width = v["width"].number();
        for (const auto& x : v["dash"].items()) r.dash.push_back(x.number());
        r.point = enum_of_<ChartSeriesPoint>(v["point"], ChartSeriesPoint_names);
        r.fill = enum_of_<ChartSeriesFill>(v["fill"], ChartSeriesFill_names);
        r.fill_series = static_cast<long long>(v["fill_series"].number());
        r.hidden = v["hidden"].boolean();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("name", aylis::Json(name));
        o.set("values", [&] { std::vector<aylis::Json> a; for (double x : values) a.emplace_back(x); return aylis::Json::array(a); }());
        o.set("x", [&] { std::vector<aylis::Json> a; for (double x : x) a.emplace_back(x); return aylis::Json::array(a); }());
        o.set("radius", [&] { std::vector<aylis::Json> a; for (double x : radius) a.emplace_back(x); return aylis::Json::array(a); }());
        o.set("kind", aylis::Json(ChartSeriesKind_names[static_cast<int>(kind)]));
        o.set("right_axis", aylis::Json(right_axis));
        o.set("color", aylis::Json(color.to_json()));
        o.set("width", aylis::Json(width));
        o.set("dash", [&] { std::vector<aylis::Json> a; for (double x : dash) a.emplace_back(x); return aylis::Json::array(a); }());
        o.set("point", aylis::Json(ChartSeriesPoint_names[static_cast<int>(point)]));
        o.set("fill", aylis::Json(ChartSeriesFill_names[static_cast<int>(fill)]));
        o.set("fill_series", aylis::Json(fill_series));
        o.set("hidden", aylis::Json(hidden));
        return o;
    }

    static std::optional<ChartSeries> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<ChartSeries> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<ChartSeries> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<ChartSeries>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<ChartSeries>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct ColumnSpec {
    std::string field;
    std::string title;
    double width = 0;
    bool read_only = false;
    aylis::Color color;
    aylis::Color text_color;

    static ColumnSpec from_json(const aylis::Json& v) {
        ColumnSpec r;
        r.field = v["field"].str();
        r.title = v["title"].str();
        r.width = v["width"].number();
        r.read_only = v["read_only"].boolean();
        r.color = aylis::Color::from_json(v["color"]);
        r.text_color = aylis::Color::from_json(v["text_color"]);
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("field", aylis::Json(field));
        o.set("title", aylis::Json(title));
        o.set("width", aylis::Json(width));
        o.set("read_only", aylis::Json(read_only));
        o.set("color", aylis::Json(color.to_json()));
        o.set("text_color", aylis::Json(text_color.to_json()));
        return o;
    }

    static std::optional<ColumnSpec> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<ColumnSpec> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<ColumnSpec> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<ColumnSpec>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<ColumnSpec>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct FilterParam {
    std::string name;
    std::string value;

    static FilterParam from_json(const aylis::Json& v) {
        FilterParam r;
        r.name = v["name"].str();
        r.value = v["value"].str();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("name", aylis::Json(name));
        o.set("value", aylis::Json(value));
        return o;
    }

    static std::optional<FilterParam> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<FilterParam> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<FilterParam> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<FilterParam>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<FilterParam>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct GestureEvent {
    std::string phase;
    double x = 0;
    double y = 0;
    double scale = 0;
    double delta_scale = 0;
    double rotation = 0;
    double delta_rotation = 0;
    double pan_x = 0;
    double pan_y = 0;

    static GestureEvent from_json(const aylis::Json& v) {
        GestureEvent r;
        r.phase = v["phase"].str();
        r.x = v["x"].number();
        r.y = v["y"].number();
        r.scale = v["scale"].number();
        r.delta_scale = v["delta_scale"].number();
        r.rotation = v["rotation"].number();
        r.delta_rotation = v["delta_rotation"].number();
        r.pan_x = v["pan_x"].number();
        r.pan_y = v["pan_y"].number();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("phase", aylis::Json(phase));
        o.set("x", aylis::Json(x));
        o.set("y", aylis::Json(y));
        o.set("scale", aylis::Json(scale));
        o.set("delta_scale", aylis::Json(delta_scale));
        o.set("rotation", aylis::Json(rotation));
        o.set("delta_rotation", aylis::Json(delta_rotation));
        o.set("pan_x", aylis::Json(pan_x));
        o.set("pan_y", aylis::Json(pan_y));
        return o;
    }

    static std::optional<GestureEvent> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<GestureEvent> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<GestureEvent> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<GestureEvent>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<GestureEvent>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct GridTotal {
    std::string field;
    GridTotalFunction function = static_cast<GridTotalFunction>(0);

    static GridTotal from_json(const aylis::Json& v) {
        GridTotal r;
        r.field = v["field"].str();
        r.function = enum_of_<GridTotalFunction>(v["function"], GridTotalFunction_names);
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("field", aylis::Json(field));
        o.set("function", aylis::Json(GridTotalFunction_names[static_cast<int>(function)]));
        return o;
    }

    static std::optional<GridTotal> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<GridTotal> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<GridTotal> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<GridTotal>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<GridTotal>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct IndexDef {
    std::string name;
    std::string fields;

    static IndexDef from_json(const aylis::Json& v) {
        IndexDef r;
        r.name = v["name"].str();
        r.fields = v["fields"].str();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("name", aylis::Json(name));
        o.set("fields", aylis::Json(fields));
        return o;
    }

    static std::optional<IndexDef> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<IndexDef> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<IndexDef> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<IndexDef>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<IndexDef>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct KeyEvent {
    std::string key;
    bool ctrl = false;
    bool shift = false;
    bool alt = false;

    static KeyEvent from_json(const aylis::Json& v) {
        KeyEvent r;
        r.key = v["key"].str();
        r.ctrl = v["ctrl"].boolean();
        r.shift = v["shift"].boolean();
        r.alt = v["alt"].boolean();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("key", aylis::Json(key));
        o.set("ctrl", aylis::Json(ctrl));
        o.set("shift", aylis::Json(shift));
        o.set("alt", aylis::Json(alt));
        return o;
    }

    static std::optional<KeyEvent> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<KeyEvent> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<KeyEvent> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<KeyEvent>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<KeyEvent>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct MasterField {
    std::string field;
    std::string master_field;

    static MasterField from_json(const aylis::Json& v) {
        MasterField r;
        r.field = v["field"].str();
        r.master_field = v["master_field"].str();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("field", aylis::Json(field));
        o.set("master_field", aylis::Json(master_field));
        return o;
    }

    static std::optional<MasterField> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<MasterField> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<MasterField> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<MasterField>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<MasterField>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct PointerEvent {
    double x = 0;
    double y = 0;
    std::string button;
    long long clicks = 0;

    static PointerEvent from_json(const aylis::Json& v) {
        PointerEvent r;
        r.x = v["x"].number();
        r.y = v["y"].number();
        r.button = v["button"].str();
        r.clicks = static_cast<long long>(v["clicks"].number());
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("x", aylis::Json(x));
        o.set("y", aylis::Json(y));
        o.set("button", aylis::Json(button));
        o.set("clicks", aylis::Json(clicks));
        return o;
    }

    static std::optional<PointerEvent> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<PointerEvent> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<PointerEvent> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<PointerEvent>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<PointerEvent>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct SwipeEvent {
    std::string direction;
    double x = 0;
    double y = 0;
    double dx = 0;
    double dy = 0;
    double vx = 0;
    double vy = 0;

    static SwipeEvent from_json(const aylis::Json& v) {
        SwipeEvent r;
        r.direction = v["direction"].str();
        r.x = v["x"].number();
        r.y = v["y"].number();
        r.dx = v["dx"].number();
        r.dy = v["dy"].number();
        r.vx = v["vx"].number();
        r.vy = v["vy"].number();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("direction", aylis::Json(direction));
        o.set("x", aylis::Json(x));
        o.set("y", aylis::Json(y));
        o.set("dx", aylis::Json(dx));
        o.set("dy", aylis::Json(dy));
        o.set("vx", aylis::Json(vx));
        o.set("vy", aylis::Json(vy));
        return o;
    }

    static std::optional<SwipeEvent> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<SwipeEvent> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<SwipeEvent> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<SwipeEvent>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<SwipeEvent>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
struct WheelEvent {
    double x = 0;
    double y = 0;
    double dx = 0;
    double dy = 0;

    static WheelEvent from_json(const aylis::Json& v) {
        WheelEvent r;
        r.x = v["x"].number();
        r.y = v["y"].number();
        r.dx = v["dx"].number();
        r.dy = v["dy"].number();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("x", aylis::Json(x));
        o.set("y", aylis::Json(y));
        o.set("dx", aylis::Json(dx));
        o.set("dy", aylis::Json(dy));
        return o;
    }

    static std::optional<WheelEvent> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<WheelEvent> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<WheelEvent> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<WheelEvent>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<WheelEvent>& items) {
        std::vector<aylis::Json> a;
        for (const auto& x : items) a.push_back(x.to_json());
        return aylis::Json::array(a).dump();
    }
};

// Um ClientDataSet de um form.
struct ClientDataSet : aylis::Component {
    ClientDataSet() = default;
    explicit ClientDataSet(const aylis::Component& c) : aylis::Component(c) {}
    std::string key_fields() const { return get_prop_("key_fields").str(); }
    void set_key_fields(const std::string& v) const { set_prop_("key_fields", v); }
    ApplyMode apply_mode() const { return enum_of_<ApplyMode>(get_prop_("apply_mode"), ApplyMode_names); }
    void set_apply_mode(ApplyMode v) const { set_prop_("apply_mode", aylis::Json(ApplyMode_names[static_cast<int>(v)])); }
    bool read_only() const { return get_prop_("read_only").boolean(); }
    void set_read_only(bool v) const { set_prop_("read_only", v); }
    bool active() const { return get_prop_("active").boolean(); }
    void set_active(bool v) const { set_prop_("active", v); }
    std::string filter() const { return get_prop_("filter").str(); }
    void set_filter(const std::string& v) const { set_prop_("filter", v); }
    bool filtered() const { return get_prop_("filtered").boolean(); }
    void set_filtered(bool v) const { set_prop_("filtered", v); }
    std::string index_field_names() const { return get_prop_("index_field_names").str(); }
    void set_index_field_names(const std::string& v) const { set_prop_("index_field_names", v); }
    std::vector<IndexDef> indexes() const { return IndexDef::list_from_text(get_prop_("indexes")); }
    void set_indexes(const std::vector<IndexDef>& v) const { set_prop_("indexes", IndexDef::list_to_text(v)); }
    std::string index_name() const { return get_prop_("index_name").str(); }
    void set_index_name(const std::string& v) const { set_prop_("index_name", v); }
    std::vector<FilterParam> params() const { return FilterParam::list_from_text(get_prop_("params")); }
    void set_params(const std::vector<FilterParam>& v) const { set_prop_("params", FilterParam::list_to_text(v)); }
    std::string status_filter() const { return get_prop_("status_filter").str(); }
    void set_status_filter(const std::string& v) const { set_prop_("status_filter", v); }
    std::optional<aylis::Component> master_source() const { return get_ref_("master_source"); }
    void set_master_source(const std::optional<aylis::Component>& v) const { set_ref_("master_source", v); }
    std::vector<MasterField> master_fields() const { return MasterField::list_from_text(get_prop_("master_fields")); }
    void set_master_fields(const std::vector<MasterField>& v) const { set_prop_("master_fields", MasterField::list_to_text(v)); }
    bool apply_separately() const { return get_prop_("apply_separately").boolean(); }
    void set_apply_separately(bool v) const { set_prop_("apply_separately", v); }
    long long record_total() const { return static_cast<long long>(get_prop_("record_total").number()); }
    void set_record_total(long long v) const { set_prop_("record_total", v); }
    long long max_records() const { return static_cast<long long>(get_prop_("max_records").number()); }
    void set_max_records(long long v) const { set_prop_("max_records", v); }
    std::string partial_notice() const { return get_prop_("partial_notice").str(); }
    std::string on_master_change() const { return get_prop_("on_master_change").str(); }
    void set_on_master_change(const std::string& v) const { set_prop_("on_master_change", v); }
    std::string after_scroll() const { return get_prop_("after_scroll").str(); }
    void set_after_scroll(const std::string& v) const { set_prop_("after_scroll", v); }
    std::string after_post() const { return get_prop_("after_post").str(); }
    void set_after_post(const std::string& v) const { set_prop_("after_post", v); }
    std::string on_state_change() const { return get_prop_("on_state_change").str(); }
    void set_on_state_change(const std::string& v) const { set_prop_("on_state_change", v); }
    std::string on_reconcile_error() const { return get_prop_("on_reconcile_error").str(); }
    void set_on_reconcile_error(const std::string& v) const { set_prop_("on_reconcile_error", v); }
    std::string on_error() const { return get_prop_("on_error").str(); }
    void set_on_error(const std::string& v) const { set_prop_("on_error", v); }
    std::string on_fetch() const { return get_prop_("on_fetch").str(); }
    void set_on_fetch(const std::string& v) const { set_prop_("on_fetch", v); }
    std::string on_server_query() const { return get_prop_("on_server_query").str(); }
    void set_on_server_query(const std::string& v) const { set_prop_("on_server_query", v); }
};

// Um DBChart de um form.
struct DBChart : aylis::Component {
    DBChart() = default;
    explicit DBChart(const aylis::Component& c) : aylis::Component(c) {}
    std::optional<aylis::Component> data_source() const { return get_ref_("data_source"); }
    void set_data_source(const std::optional<aylis::Component>& v) const { set_ref_("data_source", v); }
    std::string label_field() const { return get_prop_("label_field").str(); }
    void set_label_field(const std::string& v) const { set_prop_("label_field", v); }
    std::vector<std::string> value_fields() const { return list_of_(get_prop_("value_fields")); }
    void set_value_fields(const std::vector<std::string>& v) const { set_prop_("value_fields", join_(v)); }
    ChartKind kind() const { return enum_of_<ChartKind>(get_prop_("kind"), ChartKind_names); }
    void set_kind(ChartKind v) const { set_prop_("kind", aylis::Json(ChartKind_names[static_cast<int>(v)])); }
    std::string title() const { return get_prop_("title").str(); }
    void set_title(const std::string& v) const { set_prop_("title", v); }
    std::vector<std::string> labels() const { return list_of_(get_prop_("labels")); }
    void set_labels(const std::vector<std::string>& v) const { set_prop_("labels", join_(v)); }
    std::vector<ChartSeries> series() const { return ChartSeries::list_from_text(get_prop_("series")); }
    void set_series(const std::vector<ChartSeries>& v) const { set_prop_("series", ChartSeries::list_to_text(v)); }
    std::string json() const { return get_prop_("json").str(); }
    void set_json(const std::string& v) const { set_prop_("json", v); }
    bool legend() const { return get_prop_("legend").boolean(); }
    void set_legend(bool v) const { set_prop_("legend", v); }
    bool stepped() const { return get_prop_("stepped").boolean(); }
    void set_stepped(bool v) const { set_prop_("stepped", v); }
    double tension() const { return get_prop_("tension").number(); }
    void set_tension(double v) const { set_prop_("tension", v); }
    bool animate() const { return get_prop_("animate").boolean(); }
    void set_animate(bool v) const { set_prop_("animate", v); }
    bool zoom() const { return get_prop_("zoom").boolean(); }
    void set_zoom(bool v) const { set_prop_("zoom", v); }
    bool auto_scale() const { return get_prop_("auto_scale").boolean(); }
    void set_auto_scale(bool v) const { set_prop_("auto_scale", v); }
    double y_min() const { return get_prop_("y_min").number(); }
    void set_y_min(double v) const { set_prop_("y_min", v); }
    double y_max() const { return get_prop_("y_max").number(); }
    void set_y_max(double v) const { set_prop_("y_max", v); }
    bool log_scale() const { return get_prop_("log_scale").boolean(); }
    void set_log_scale(bool v) const { set_prop_("log_scale", v); }
    bool reverse() const { return get_prop_("reverse").boolean(); }
    void set_reverse(bool v) const { set_prop_("reverse", v); }
    bool time_axis() const { return get_prop_("time_axis").boolean(); }
    void set_time_axis(bool v) const { set_prop_("time_axis", v); }
    long long clicked_series() const { return static_cast<long long>(get_prop_("clicked_series").number()); }
    void set_clicked_series(long long v) const { set_prop_("clicked_series", v); }
    long long clicked_index() const { return static_cast<long long>(get_prop_("clicked_index").number()); }
    void set_clicked_index(long long v) const { set_prop_("clicked_index", v); }
    ValueFormat value_format() const { return enum_of_<ValueFormat>(get_prop_("value_format"), ValueFormat_names); }
    void set_value_format(ValueFormat v) const { set_prop_("value_format", aylis::Json(ValueFormat_names[static_cast<int>(v)])); }
    double label_angle() const { return get_prop_("label_angle").number(); }
    void set_label_angle(double v) const { set_prop_("label_angle", v); }
    TitleAlign title_align() const { return enum_of_<TitleAlign>(get_prop_("title_align"), TitleAlign_names); }
    void set_title_align(TitleAlign v) const { set_prop_("title_align", aylis::Json(TitleAlign_names[static_cast<int>(v)])); }
    bool title_bold() const { return get_prop_("title_bold").boolean(); }
    void set_title_bold(bool v) const { set_prop_("title_bold", v); }
    double area_opacity() const { return get_prop_("area_opacity").number(); }
    void set_area_opacity(double v) const { set_prop_("area_opacity", v); }
    LegendPosition legend_position() const { return enum_of_<LegendPosition>(get_prop_("legend_position"), LegendPosition_names); }
    void set_legend_position(LegendPosition v) const { set_prop_("legend_position", aylis::Json(LegendPosition_names[static_cast<int>(v)])); }
    bool stacked() const { return get_prop_("stacked").boolean(); }
    void set_stacked(bool v) const { set_prop_("stacked", v); }
    std::string colors() const { return get_prop_("colors").str(); }
    void set_colors(const std::string& v) const { set_prop_("colors", v); }
    std::string center_text() const { return get_prop_("center_text").str(); }
    void set_center_text(const std::string& v) const { set_prop_("center_text", v); }
    std::string slice_labels() const { return get_prop_("slice_labels").str(); }
    void set_slice_labels(const std::string& v) const { set_prop_("slice_labels", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    DBChartStyle style() const { return enum_of_<DBChartStyle>(get_prop_("style"), DBChartStyle_names); }
    void set_style(DBChartStyle v) const { set_prop_("style", aylis::Json(DBChartStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um DataGrid de um form.
struct DataGrid : aylis::Component {
    DataGrid() = default;
    explicit DataGrid(const aylis::Component& c) : aylis::Component(c) {}
    Overscroll overscroll() const { return enum_of_<Overscroll>(get_prop_("overscroll"), Overscroll_names); }
    void set_overscroll(Overscroll v) const { set_prop_("overscroll", aylis::Json(Overscroll_names[static_cast<int>(v)])); }
    std::optional<aylis::Component> data_source() const { return get_ref_("data_source"); }
    void set_data_source(const std::optional<aylis::Component>& v) const { set_ref_("data_source", v); }
    std::vector<ColumnSpec> columns() const { return ColumnSpec::list_from_text(get_prop_("columns")); }
    void set_columns(const std::vector<ColumnSpec>& v) const { set_prop_("columns", ColumnSpec::list_to_text(v)); }
    double row_height() const { return get_prop_("row_height").number(); }
    void set_row_height(double v) const { set_prop_("row_height", v); }
    bool read_only() const { return get_prop_("read_only").boolean(); }
    void set_read_only(bool v) const { set_prop_("read_only", v); }
    bool sortable() const { return get_prop_("sortable").boolean(); }
    void set_sortable(bool v) const { set_prop_("sortable", v); }
    bool column_move() const { return get_prop_("column_move").boolean(); }
    void set_column_move(bool v) const { set_prop_("column_move", v); }
    long long fixed_columns() const { return static_cast<long long>(get_prop_("fixed_columns").number()); }
    void set_fixed_columns(long long v) const { set_prop_("fixed_columns", v); }
    bool filter_row() const { return get_prop_("filter_row").boolean(); }
    void set_filter_row(bool v) const { set_prop_("filter_row", v); }
    bool multi_select() const { return get_prop_("multi_select").boolean(); }
    void set_multi_select(bool v) const { set_prop_("multi_select", v); }
    std::vector<GridTotal> footer() const { return GridTotal::list_from_text(get_prop_("footer")); }
    void set_footer(const std::vector<GridTotal>& v) const { set_prop_("footer", GridTotal::list_to_text(v)); }
    bool incremental_search() const { return get_prop_("incremental_search").boolean(); }
    void set_incremental_search(bool v) const { set_prop_("incremental_search", v); }
    aylis::Color cursor_color() const { return aylis::Color::from_json(get_prop_("cursor_color")); }
    void set_cursor_color(const aylis::Color& v) const { set_prop_("cursor_color", v.to_json()); }
    bool indicator() const { return get_prop_("indicator").boolean(); }
    void set_indicator(bool v) const { set_prop_("indicator", v); }
    bool highlight_current() const { return get_prop_("highlight_current").boolean(); }
    void set_highlight_current(bool v) const { set_prop_("highlight_current", v); }
    HorizontalWheel horizontal_wheel() const { return enum_of_<HorizontalWheel>(get_prop_("horizontal_wheel"), HorizontalWheel_names); }
    void set_horizontal_wheel(HorizontalWheel v) const { set_prop_("horizontal_wheel", aylis::Json(HorizontalWheel_names[static_cast<int>(v)])); }
    std::string on_error() const { return get_prop_("on_error").str(); }
    void set_on_error(const std::string& v) const { set_prop_("on_error", v); }
    std::string on_title_click() const { return get_prop_("on_title_click").str(); }
    void set_on_title_click(const std::string& v) const { set_prop_("on_title_click", v); }
    std::string on_selection_change() const { return get_prop_("on_selection_change").str(); }
    void set_on_selection_change(const std::string& v) const { set_prop_("on_selection_change", v); }
    std::string on_column_moved() const { return get_prop_("on_column_moved").str(); }
    void set_on_column_moved(const std::string& v) const { set_prop_("on_column_moved", v); }
    double scroll_left() const { return get_prop_("scroll_left").number(); }
    void set_scroll_left(double v) const { set_prop_("scroll_left", v); }
    long long top_row() const { return static_cast<long long>(get_prop_("top_row").number()); }
    void set_top_row(long long v) const { set_prop_("top_row", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    DataGridStyle style() const { return enum_of_<DataGridStyle>(get_prop_("style"), DataGridStyle_names); }
    void set_style(DataGridStyle v) const { set_prop_("style", aylis::Json(DataGridStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um DataSource de um form.
struct DataSource : aylis::Component {
    DataSource() = default;
    explicit DataSource(const aylis::Component& c) : aylis::Component(c) {}
    std::optional<aylis::Component> data_set() const { return get_ref_("data_set"); }
    void set_data_set(const std::optional<aylis::Component>& v) const { set_ref_("data_set", v); }
    bool auto_edit() const { return get_prop_("auto_edit").boolean(); }
    void set_auto_edit(bool v) const { set_prop_("auto_edit", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    std::string on_data_change() const { return get_prop_("on_data_change").str(); }
    void set_on_data_change(const std::string& v) const { set_prop_("on_data_change", v); }
    std::string on_state_change() const { return get_prop_("on_state_change").str(); }
    void set_on_state_change(const std::string& v) const { set_prop_("on_state_change", v); }
};

// Um Field de um form.
struct Field : aylis::Component {
    Field() = default;
    explicit Field(const aylis::Component& c) : aylis::Component(c) {}
    std::string field_name() const { return get_prop_("field_name").str(); }
    void set_field_name(const std::string& v) const { set_prop_("field_name", v); }
    FieldType type() const { return enum_of_<FieldType>(get_prop_("type"), FieldType_names); }
    void set_type(FieldType v) const { set_prop_("type", aylis::Json(FieldType_names[static_cast<int>(v)])); }
    long long size() const { return static_cast<long long>(get_prop_("size").number()); }
    void set_size(long long v) const { set_prop_("size", v); }
    long long scale() const { return static_cast<long long>(get_prop_("scale").number()); }
    void set_scale(long long v) const { set_prop_("scale", v); }
    Rounding rounding() const { return enum_of_<Rounding>(get_prop_("rounding"), Rounding_names); }
    void set_rounding(Rounding v) const { set_prop_("rounding", aylis::Json(Rounding_names[static_cast<int>(v)])); }
    std::string display_label() const { return get_prop_("display_label").str(); }
    void set_display_label(const std::string& v) const { set_prop_("display_label", v); }
    double display_width() const { return get_prop_("display_width").number(); }
    void set_display_width(double v) const { set_prop_("display_width", v); }
    std::string display_format() const { return get_prop_("display_format").str(); }
    void set_display_format(const std::string& v) const { set_prop_("display_format", v); }
    std::string edit_format() const { return get_prop_("edit_format").str(); }
    void set_edit_format(const std::string& v) const { set_prop_("edit_format", v); }
    TextAlign alignment() const { return enum_of_<TextAlign>(get_prop_("alignment"), TextAlign_names); }
    void set_alignment(TextAlign v) const { set_prop_("alignment", aylis::Json(TextAlign_names[static_cast<int>(v)])); }
    bool required() const { return get_prop_("required").boolean(); }
    void set_required(bool v) const { set_prop_("required", v); }
    bool read_only() const { return get_prop_("read_only").boolean(); }
    void set_read_only(bool v) const { set_prop_("read_only", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    double min() const { return get_prop_("min").number(); }
    void set_min(double v) const { set_prop_("min", v); }
    double max() const { return get_prop_("max").number(); }
    void set_max(double v) const { set_prop_("max", v); }
    std::string default_value() const { return get_prop_("default_value").str(); }
    void set_default_value(const std::string& v) const { set_prop_("default_value", v); }
    std::string display_values() const { return get_prop_("display_values").str(); }
    void set_display_values(const std::string& v) const { set_prop_("display_values", v); }
    CharCase char_case() const { return enum_of_<CharCase>(get_prop_("char_case"), CharCase_names); }
    void set_char_case(CharCase v) const { set_prop_("char_case", aylis::Json(CharCase_names[static_cast<int>(v)])); }
    std::string edit_mask() const { return get_prop_("edit_mask").str(); }
    void set_edit_mask(const std::string& v) const { set_prop_("edit_mask", v); }
    bool sensitive() const { return get_prop_("sensitive").boolean(); }
    void set_sensitive(bool v) const { set_prop_("sensitive", v); }
    FieldKind kind() const { return enum_of_<FieldKind>(get_prop_("kind"), FieldKind_names); }
    void set_kind(FieldKind v) const { set_prop_("kind", aylis::Json(FieldKind_names[static_cast<int>(v)])); }
    std::string expression() const { return get_prop_("expression").str(); }
    void set_expression(const std::string& v) const { set_prop_("expression", v); }
    std::optional<aylis::Component> lookup_data_set() const { return get_ref_("lookup_data_set"); }
    void set_lookup_data_set(const std::optional<aylis::Component>& v) const { set_ref_("lookup_data_set", v); }
    std::string key_fields() const { return get_prop_("key_fields").str(); }
    void set_key_fields(const std::string& v) const { set_prop_("key_fields", v); }
    std::string lookup_key_fields() const { return get_prop_("lookup_key_fields").str(); }
    void set_lookup_key_fields(const std::string& v) const { set_prop_("lookup_key_fields", v); }
    std::string lookup_result_field() const { return get_prop_("lookup_result_field").str(); }
    void set_lookup_result_field(const std::string& v) const { set_prop_("lookup_result_field", v); }
    std::string on_change() const { return get_prop_("on_change").str(); }
    void set_on_change(const std::string& v) const { set_prop_("on_change", v); }
    std::string on_get_text() const { return get_prop_("on_get_text").str(); }
    void set_on_get_text(const std::string& v) const { set_prop_("on_get_text", v); }
    std::string on_set_text() const { return get_prop_("on_set_text").str(); }
    void set_on_set_text(const std::string& v) const { set_prop_("on_set_text", v); }
};

// Um Form de um form.
struct Form : aylis::FormBase {
    Form() = default;
    using aylis::FormBase::FormBase;
    std::string caption() const { return get_prop_("caption").str(); }
    void set_caption(const std::string& v) const { set_prop_("caption", v); }
    FormPosition position() const { return enum_of_<FormPosition>(get_prop_("position"), FormPosition_names); }
    void set_position(FormPosition v) const { set_prop_("position", aylis::Json(FormPosition_names[static_cast<int>(v)])); }
    BorderStyle border_style() const { return enum_of_<BorderStyle>(get_prop_("border_style"), BorderStyle_names); }
    void set_border_style(BorderStyle v) const { set_prop_("border_style", aylis::Json(BorderStyle_names[static_cast<int>(v)])); }
    WindowCorners window_corners() const { return enum_of_<WindowCorners>(get_prop_("window_corners"), WindowCorners_names); }
    void set_window_corners(WindowCorners v) const { set_prop_("window_corners", aylis::Json(WindowCorners_names[static_cast<int>(v)])); }
    bool transparent() const { return get_prop_("transparent").boolean(); }
    void set_transparent(bool v) const { set_prop_("transparent", v); }
    WindowShape window_shape() const { return enum_of_<WindowShape>(get_prop_("window_shape"), WindowShape_names); }
    void set_window_shape(WindowShape v) const { set_prop_("window_shape", aylis::Json(WindowShape_names[static_cast<int>(v)])); }
    double shape_radius() const { return get_prop_("shape_radius").number(); }
    void set_shape_radius(double v) const { set_prop_("shape_radius", v); }
    aylis::Bounds shape_radii() const { return aylis::Bounds::from_json(get_prop_("shape_radii")); }
    void set_shape_radii(const aylis::Bounds& v) const { set_prop_("shape_radii", v.to_json()); }
    std::string shape_path() const { return get_prop_("shape_path").str(); }
    void set_shape_path(const std::string& v) const { set_prop_("shape_path", v); }
    WindowState window_state() const { return enum_of_<WindowState>(get_prop_("window_state"), WindowState_names); }
    void set_window_state(WindowState v) const { set_prop_("window_state", aylis::Json(WindowState_names[static_cast<int>(v)])); }
    Transition transition() const { return enum_of_<Transition>(get_prop_("transition"), Transition_names); }
    void set_transition(Transition v) const { set_prop_("transition", aylis::Json(Transition_names[static_cast<int>(v)])); }
    LayerCaching layer_cache() const { return enum_of_<LayerCaching>(get_prop_("layer_cache"), LayerCaching_names); }
    void set_layer_cache(LayerCaching v) const { set_prop_("layer_cache", aylis::Json(LayerCaching_names[static_cast<int>(v)])); }
    long long frame_reuse_kb() const { return static_cast<long long>(get_prop_("frame_reuse_kb").number()); }
    void set_frame_reuse_kb(long long v) const { set_prop_("frame_reuse_kb", v); }
    bool topmost() const { return get_prop_("topmost").boolean(); }
    void set_topmost(bool v) const { set_prop_("topmost", v); }
    bool show_in_taskbar() const { return get_prop_("show_in_taskbar").boolean(); }
    void set_show_in_taskbar(bool v) const { set_prop_("show_in_taskbar", v); }
    std::optional<aylis::Component> active_control() const { return get_ref_("active_control"); }
    void set_active_control(const std::optional<aylis::Component>& v) const { set_ref_("active_control", v); }
    std::vector<std::string> tab_order() const { return list_of_(get_prop_("tab_order")); }
    void set_tab_order(const std::vector<std::string>& v) const { set_prop_("tab_order", join_(v)); }
    std::string tab_keys() const { return get_prop_("tab_keys").str(); }
    void set_tab_keys(const std::string& v) const { set_prop_("tab_keys", v); }
    std::string back_tab_keys() const { return get_prop_("back_tab_keys").str(); }
    void set_back_tab_keys(const std::string& v) const { set_prop_("back_tab_keys", v); }
    std::string allowed_chars() const { return get_prop_("allowed_chars").str(); }
    void set_allowed_chars(const std::string& v) const { set_prop_("allowed_chars", v); }
    std::string blocked_chars() const { return get_prop_("blocked_chars").str(); }
    void set_blocked_chars(const std::string& v) const { set_prop_("blocked_chars", v); }
    bool only_allowed() const { return get_prop_("only_allowed").boolean(); }
    void set_only_allowed(bool v) const { set_prop_("only_allowed", v); }
    bool chars_override() const { return get_prop_("chars_override").boolean(); }
    void set_chars_override(bool v) const { set_prop_("chars_override", v); }
    std::string char_map() const { return get_prop_("char_map").str(); }
    void set_char_map(const std::string& v) const { set_prop_("char_map", v); }
    bool key_preview() const { return get_prop_("key_preview").boolean(); }
    void set_key_preview(bool v) const { set_prop_("key_preview", v); }
    double zoom() const { return get_prop_("zoom").number(); }
    void set_zoom(double v) const { set_prop_("zoom", v); }
    long long monitor() const { return static_cast<long long>(get_prop_("monitor").number()); }
    void set_monitor(long long v) const { set_prop_("monitor", v); }
    bool remember_placement() const { return get_prop_("remember_placement").boolean(); }
    void set_remember_placement(bool v) const { set_prop_("remember_placement", v); }
    std::string gpu() const { return get_prop_("gpu").str(); }
    void set_gpu(const std::string& v) const { set_prop_("gpu", v); }
    FormPresent present() const { return enum_of_<FormPresent>(get_prop_("present"), FormPresent_names); }
    void set_present(FormPresent v) const { set_prop_("present", aylis::Json(FormPresent_names[static_cast<int>(v)])); }
    bool span_monitors() const { return get_prop_("span_monitors").boolean(); }
    void set_span_monitors(bool v) const { set_prop_("span_monitors", v); }
    bool physical_pixels() const { return get_prop_("physical_pixels").boolean(); }
    void set_physical_pixels(bool v) const { set_prop_("physical_pixels", v); }
    bool activate_on_show() const { return get_prop_("activate_on_show").boolean(); }
    void set_activate_on_show(bool v) const { set_prop_("activate_on_show", v); }
    bool force_foreground() const { return get_prop_("force_foreground").boolean(); }
    void set_force_foreground(bool v) const { set_prop_("force_foreground", v); }
    CloseAction close_action() const { return enum_of_<CloseAction>(get_prop_("close_action"), CloseAction_names); }
    void set_close_action(CloseAction v) const { set_prop_("close_action", aylis::Json(CloseAction_names[static_cast<int>(v)])); }
    double frame_rate() const { return get_prop_("frame_rate").number(); }
    void set_frame_rate(double v) const { set_prop_("frame_rate", v); }
    FormOrientation orientation() const { return enum_of_<FormOrientation>(get_prop_("orientation"), FormOrientation_names); }
    void set_orientation(FormOrientation v) const { set_prop_("orientation", aylis::Json(FormOrientation_names[static_cast<int>(v)])); }
    std::string on_show() const { return get_prop_("on_show").str(); }
    void set_on_show(const std::string& v) const { set_prop_("on_show", v); }
    std::string on_window_created() const { return get_prop_("on_window_created").str(); }
    void set_on_window_created(const std::string& v) const { set_prop_("on_window_created", v); }
    std::string on_close() const { return get_prop_("on_close").str(); }
    void set_on_close(const std::string& v) const { set_prop_("on_close", v); }
    std::string on_activate() const { return get_prop_("on_activate").str(); }
    void set_on_activate(const std::string& v) const { set_prop_("on_activate", v); }
    std::string on_deactivate() const { return get_prop_("on_deactivate").str(); }
    void set_on_deactivate(const std::string& v) const { set_prop_("on_deactivate", v); }
    std::string on_minimize() const { return get_prop_("on_minimize").str(); }
    void set_on_minimize(const std::string& v) const { set_prop_("on_minimize", v); }
    std::string on_restore() const { return get_prop_("on_restore").str(); }
    void set_on_restore(const std::string& v) const { set_prop_("on_restore", v); }
    std::string on_navigate() const { return get_prop_("on_navigate").str(); }
    void set_on_navigate(const std::string& v) const { set_prop_("on_navigate", v); }
    std::string on_gpu_change() const { return get_prop_("on_gpu_change").str(); }
    void set_on_gpu_change(const std::string& v) const { set_prop_("on_gpu_change", v); }
    double client_width() const { return get_prop_("client_width").number(); }
    void set_client_width(double v) const { set_prop_("client_width", v); }
    double client_height() const { return get_prop_("client_height").number(); }
    void set_client_height(double v) const { set_prop_("client_height", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    FormStyle style() const { return enum_of_<FormStyle>(get_prop_("style"), FormStyle_names); }
    void set_style(FormStyle v) const { set_prop_("style", aylis::Json(FormStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
    void move_to_monitor(long long monitor) const { tell_("invoke", {{"target", aylis::Json(id_)}, {"method", "move_to_monitor"}, {"args", aylis::Json::array({monitor})}}); }
    std::string gpu_adapter() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "gpu_adapter"}, {"args", aylis::Json::array({})}}).str(); }
    std::vector<std::string> gpu_adapters() const { return list_of_(ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "gpu_adapters"}, {"args", aylis::Json::array({})}})); }
    std::string render_rgba(double x, double y, double width, double height, double scale) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "render_rgba"}, {"args", aylis::Json::array({x, y, width, height, scale})}}).str(); }
    void show() const { tell_("show", {{"target", aylis::Json(id_)}}); }
    bool close() const { return ask_("close", {{"target", aylis::Json(id_)}}).boolean(); }
    std::string focused() const { return ask_("focused", {{"target", aylis::Json(id_)}}).str(); }
    void focus_next() const { tell_("focus_next", {{"target", aylis::Json(id_)}}); }
    void focus_prev() const { tell_("focus_prev", {{"target", aylis::Json(id_)}}); }
};

// Um GroupBox de um form.
struct GroupBox : aylis::Component {
    GroupBox() = default;
    explicit GroupBox(const aylis::Component& c) : aylis::Component(c) {}
    std::string caption() const { return get_prop_("caption").str(); }
    void set_caption(const std::string& v) const { set_prop_("caption", v); }
    std::string allowed_chars() const { return get_prop_("allowed_chars").str(); }
    void set_allowed_chars(const std::string& v) const { set_prop_("allowed_chars", v); }
    std::string blocked_chars() const { return get_prop_("blocked_chars").str(); }
    void set_blocked_chars(const std::string& v) const { set_prop_("blocked_chars", v); }
    bool only_allowed() const { return get_prop_("only_allowed").boolean(); }
    void set_only_allowed(bool v) const { set_prop_("only_allowed", v); }
    bool chars_override() const { return get_prop_("chars_override").boolean(); }
    void set_chars_override(bool v) const { set_prop_("chars_override", v); }
    std::string char_map() const { return get_prop_("char_map").str(); }
    void set_char_map(const std::string& v) const { set_prop_("char_map", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    GroupBoxStyle style() const { return enum_of_<GroupBoxStyle>(get_prop_("style"), GroupBoxStyle_names); }
    void set_style(GroupBoxStyle v) const { set_prop_("style", aylis::Json(GroupBoxStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um Image de um form.
struct Image : aylis::Component {
    Image() = default;
    explicit Image(const aylis::Component& c) : aylis::Component(c) {}
    std::string image() const { return get_prop_("image").str(); }
    void set_image(const std::string& v) const { set_prop_("image", v); }
    ImageFit fit() const { return enum_of_<ImageFit>(get_prop_("fit"), ImageFit_names); }
    void set_fit(ImageFit v) const { set_prop_("fit", aylis::Json(ImageFit_names[static_cast<int>(v)])); }
    double radius() const { return get_prop_("radius").number(); }
    void set_radius(double v) const { set_prop_("radius", v); }
    bool zoomable() const { return get_prop_("zoomable").boolean(); }
    void set_zoomable(bool v) const { set_prop_("zoomable", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    ImageStyle style() const { return enum_of_<ImageStyle>(get_prop_("style"), ImageStyle_names); }
    void set_style(ImageStyle v) const { set_prop_("style", aylis::Json(ImageStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um Label de um form.
struct Label : aylis::Component {
    Label() = default;
    explicit Label(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    TextAlign h_align() const { return enum_of_<TextAlign>(get_prop_("h_align"), TextAlign_names); }
    void set_h_align(TextAlign v) const { set_prop_("h_align", aylis::Json(TextAlign_names[static_cast<int>(v)])); }
    TextAlign v_align() const { return enum_of_<TextAlign>(get_prop_("v_align"), TextAlign_names); }
    void set_v_align(TextAlign v) const { set_prop_("v_align", aylis::Json(TextAlign_names[static_cast<int>(v)])); }
    bool wrap() const { return get_prop_("wrap").boolean(); }
    void set_wrap(bool v) const { set_prop_("wrap", v); }
    Trimming trimming() const { return enum_of_<Trimming>(get_prop_("trimming"), Trimming_names); }
    void set_trimming(Trimming v) const { set_prop_("trimming", aylis::Json(Trimming_names[static_cast<int>(v)])); }
    bool bold() const { return get_prop_("bold").boolean(); }
    void set_bold(bool v) const { set_prop_("bold", v); }
    aylis::Color color() const { return aylis::Color::from_json(get_prop_("color")); }
    void set_color(const aylis::Color& v) const { set_prop_("color", v.to_json()); }
    std::string highlight() const { return get_prop_("highlight").str(); }
    void set_highlight(const std::string& v) const { set_prop_("highlight", v); }
    double letter_spacing() const { return get_prop_("letter_spacing").number(); }
    void set_letter_spacing(double v) const { set_prop_("letter_spacing", v); }
    TextCase text_case() const { return enum_of_<TextCase>(get_prop_("text_case"), TextCase_names); }
    void set_text_case(TextCase v) const { set_prop_("text_case", aylis::Json(TextCase_names[static_cast<int>(v)])); }
    bool markup() const { return get_prop_("markup").boolean(); }
    void set_markup(bool v) const { set_prop_("markup", v); }
    aylis::Color outline() const { return aylis::Color::from_json(get_prop_("outline")); }
    void set_outline(const aylis::Color& v) const { set_prop_("outline", v.to_json()); }
    double outline_width() const { return get_prop_("outline_width").number(); }
    void set_outline_width(double v) const { set_prop_("outline_width", v); }
    double line_height() const { return get_prop_("line_height").number(); }
    void set_line_height(double v) const { set_prop_("line_height", v); }
    std::optional<aylis::Shadow> text_shadow() const { return aylis::Shadow::from_json(get_prop_("text_shadow")); }
    void set_text_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("text_shadow", (v ? v->to_json() : std::string())); }
    std::optional<aylis::Component> focus_control() const { return get_ref_("focus_control"); }
    void set_focus_control(const std::optional<aylis::Component>& v) const { set_ref_("focus_control", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    LabelStyle style() const { return enum_of_<LabelStyle>(get_prop_("style"), LabelStyle_names); }
    void set_style(LabelStyle v) const { set_prop_("style", aylis::Json(LabelStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
    double min_content_width() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "min_content_width"}, {"args", aylis::Json::array({})}}).number(); }
    double max_content_width() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "max_content_width"}, {"args", aylis::Json::array({})}}).number(); }
};

// Um Layout de um form.
struct Layout : aylis::Component {
    Layout() = default;
    explicit Layout(const aylis::Component& c) : aylis::Component(c) {}
    std::string allowed_chars() const { return get_prop_("allowed_chars").str(); }
    void set_allowed_chars(const std::string& v) const { set_prop_("allowed_chars", v); }
    std::string blocked_chars() const { return get_prop_("blocked_chars").str(); }
    void set_blocked_chars(const std::string& v) const { set_prop_("blocked_chars", v); }
    bool only_allowed() const { return get_prop_("only_allowed").boolean(); }
    void set_only_allowed(bool v) const { set_prop_("only_allowed", v); }
    bool chars_override() const { return get_prop_("chars_override").boolean(); }
    void set_chars_override(bool v) const { set_prop_("chars_override", v); }
    std::string char_map() const { return get_prop_("char_map").str(); }
    void set_char_map(const std::string& v) const { set_prop_("char_map", v); }
    double gap() const { return get_prop_("gap").number(); }
    void set_gap(double v) const { set_prop_("gap", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    LayoutStyle style() const { return enum_of_<LayoutStyle>(get_prop_("style"), LayoutStyle_names); }
    void set_style(LayoutStyle v) const { set_prop_("style", aylis::Json(LayoutStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um Panel de um form.
struct Panel : aylis::Component {
    Panel() = default;
    explicit Panel(const aylis::Component& c) : aylis::Component(c) {}
    std::string allowed_chars() const { return get_prop_("allowed_chars").str(); }
    void set_allowed_chars(const std::string& v) const { set_prop_("allowed_chars", v); }
    std::string blocked_chars() const { return get_prop_("blocked_chars").str(); }
    void set_blocked_chars(const std::string& v) const { set_prop_("blocked_chars", v); }
    bool only_allowed() const { return get_prop_("only_allowed").boolean(); }
    void set_only_allowed(bool v) const { set_prop_("only_allowed", v); }
    bool chars_override() const { return get_prop_("chars_override").boolean(); }
    void set_chars_override(bool v) const { set_prop_("chars_override", v); }
    std::string char_map() const { return get_prop_("char_map").str(); }
    void set_char_map(const std::string& v) const { set_prop_("char_map", v); }
    double gap() const { return get_prop_("gap").number(); }
    void set_gap(double v) const { set_prop_("gap", v); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color fill2() const { return aylis::Color::from_json(get_prop_("fill2")); }
    void set_fill2(const aylis::Color& v) const { set_prop_("fill2", v.to_json()); }
    aylis::Color fill_mid() const { return aylis::Color::from_json(get_prop_("fill_mid")); }
    void set_fill_mid(const aylis::Color& v) const { set_prop_("fill_mid", v.to_json()); }
    double fill_mid_at() const { return get_prop_("fill_mid_at").number(); }
    void set_fill_mid_at(double v) const { set_prop_("fill_mid_at", v); }
    double gradient_angle() const { return get_prop_("gradient_angle").number(); }
    void set_gradient_angle(double v) const { set_prop_("gradient_angle", v); }
    bool gradient_radial() const { return get_prop_("gradient_radial").boolean(); }
    void set_gradient_radial(bool v) const { set_prop_("gradient_radial", v); }
    aylis::Color stroke() const { return aylis::Color::from_json(get_prop_("stroke")); }
    void set_stroke(const aylis::Color& v) const { set_prop_("stroke", v.to_json()); }
    double stroke_width() const { return get_prop_("stroke_width").number(); }
    void set_stroke_width(double v) const { set_prop_("stroke_width", v); }
    aylis::Bounds radius() const { return aylis::Bounds::from_json(get_prop_("radius")); }
    void set_radius(const aylis::Bounds& v) const { set_prop_("radius", v.to_json()); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    PanelStyle style() const { return enum_of_<PanelStyle>(get_prop_("style"), PanelStyle_names); }
    void set_style(PanelStyle v) const { set_prop_("style", aylis::Json(PanelStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um Splitter de um form.
struct Splitter : aylis::Component {
    Splitter() = default;
    explicit Splitter(const aylis::Component& c) : aylis::Component(c) {}
    double min_size() const { return get_prop_("min_size").number(); }
    void set_min_size(double v) const { set_prop_("min_size", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    SplitterStyle style() const { return enum_of_<SplitterStyle>(get_prop_("style"), SplitterStyle_names); }
    void set_style(SplitterStyle v) const { set_prop_("style", aylis::Json(SplitterStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// Um Svg de um form.
struct Svg : aylis::Component {
    Svg() = default;
    explicit Svg(const aylis::Component& c) : aylis::Component(c) {}
    std::string source() const { return get_prop_("source").str(); }
    void set_source(const std::string& v) const { set_prop_("source", v); }
    aylis::Color color() const { return aylis::Color::from_json(get_prop_("color")); }
    void set_color(const aylis::Color& v) const { set_prop_("color", v.to_json()); }
    ImageFit fit() const { return enum_of_<ImageFit>(get_prop_("fit"), ImageFit_names); }
    void set_fit(ImageFit v) const { set_prop_("fit", aylis::Json(ImageFit_names[static_cast<int>(v)])); }
    std::vector<std::string> highlight() const { return list_of_(get_prop_("highlight")); }
    void set_highlight(const std::vector<std::string>& v) const { set_prop_("highlight", join_(v)); }
    aylis::Color highlight_color() const { return aylis::Color::from_json(get_prop_("highlight_color")); }
    void set_highlight_color(const aylis::Color& v) const { set_prop_("highlight_color", v.to_json()); }
    std::string zoom_to() const { return get_prop_("zoom_to").str(); }
    void set_zoom_to(const std::string& v) const { set_prop_("zoom_to", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    Align align() const { return enum_of_<Align>(get_prop_("align"), Align_names); }
    void set_align(Align v) const { set_prop_("align", aylis::Json(Align_names[static_cast<int>(v)])); }
    std::vector<Anchor> anchors() const { return enum_list_<Anchor>(get_prop_("anchors"), Anchor_names); }
    void set_anchors(const std::vector<Anchor>& v) const { set_prop_("anchors", enum_names_(v, Anchor_names)); }
    aylis::Bounds margins() const { return aylis::Bounds::from_json(get_prop_("margins")); }
    void set_margins(const aylis::Bounds& v) const { set_prop_("margins", v.to_json()); }
    aylis::Bounds padding() const { return aylis::Bounds::from_json(get_prop_("padding")); }
    void set_padding(const aylis::Bounds& v) const { set_prop_("padding", v.to_json()); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool inherit_state() const { return get_prop_("inherit_state").boolean(); }
    void set_inherit_state(bool v) const { set_prop_("inherit_state", v); }
    bool translate() const { return get_prop_("translate").boolean(); }
    void set_translate(bool v) const { set_prop_("translate", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
    bool captures_gesture() const { return get_prop_("captures_gesture").boolean(); }
    void set_captures_gesture(bool v) const { set_prop_("captures_gesture", v); }
    bool can_focus() const { return get_prop_("can_focus").boolean(); }
    void set_can_focus(bool v) const { set_prop_("can_focus", v); }
    bool draggable() const { return get_prop_("draggable").boolean(); }
    void set_draggable(bool v) const { set_prop_("draggable", v); }
    bool accepts_drops() const { return get_prop_("accepts_drops").boolean(); }
    void set_accepts_drops(bool v) const { set_prop_("accepts_drops", v); }
    std::string drag_data() const { return get_prop_("drag_data").str(); }
    void set_drag_data(const std::string& v) const { set_prop_("drag_data", v); }
    std::string drag_label() const { return get_prop_("drag_label").str(); }
    void set_drag_label(const std::string& v) const { set_prop_("drag_label", v); }
    std::optional<aylis::Component> drag_image() const { return get_ref_("drag_image"); }
    void set_drag_image(const std::optional<aylis::Component>& v) const { set_ref_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    CursorShape cursor() const { return enum_of_<CursorShape>(get_prop_("cursor"), CursorShape_names); }
    void set_cursor(CursorShape v) const { set_prop_("cursor", aylis::Json(CursorShape_names[static_cast<int>(v)])); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    BlendMode blend_mode() const { return enum_of_<BlendMode>(get_prop_("blend_mode"), BlendMode_names); }
    void set_blend_mode(BlendMode v) const { set_prop_("blend_mode", aylis::Json(BlendMode_names[static_cast<int>(v)])); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
    double scale() const { return get_prop_("scale").number(); }
    void set_scale(double v) const { set_prop_("scale", v); }
    double translate_x() const { return get_prop_("translate_x").number(); }
    void set_translate_x(double v) const { set_prop_("translate_x", v); }
    double translate_y() const { return get_prop_("translate_y").number(); }
    void set_translate_y(double v) const { set_prop_("translate_y", v); }
    double aspect_ratio() const { return get_prop_("aspect_ratio").number(); }
    void set_aspect_ratio(double v) const { set_prop_("aspect_ratio", v); }
    double max_width() const { return get_prop_("max_width").number(); }
    void set_max_width(double v) const { set_prop_("max_width", v); }
    double min_width() const { return get_prop_("min_width").number(); }
    void set_min_width(double v) const { set_prop_("min_width", v); }
    double min_height() const { return get_prop_("min_height").number(); }
    void set_min_height(double v) const { set_prop_("min_height", v); }
    double max_height() const { return get_prop_("max_height").number(); }
    void set_max_height(double v) const { set_prop_("max_height", v); }
    SelfAlign self_align() const { return enum_of_<SelfAlign>(get_prop_("self_align"), SelfAlign_names); }
    void set_self_align(SelfAlign v) const { set_prop_("self_align", aylis::Json(SelfAlign_names[static_cast<int>(v)])); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
    std::optional<aylis::Shadow> shadow() const { return aylis::Shadow::from_json(get_prop_("shadow")); }
    void set_shadow(const std::optional<aylis::Shadow>& v) const { set_prop_("shadow", (v ? v->to_json() : std::string())); }
    SvgStyle style() const { return enum_of_<SvgStyle>(get_prop_("style"), SvgStyle_names); }
    void set_style(SvgStyle v) const { set_prop_("style", aylis::Json(SvgStyle_names[static_cast<int>(v)])); }
    ThemeName theme() const { return enum_of_<ThemeName>(get_prop_("theme"), ThemeName_names); }
    void set_theme(ThemeName v) const { set_prop_("theme", aylis::Json(ThemeName_names[static_cast<int>(v)])); }
    aylis::Color fill() const { return aylis::Color::from_json(get_prop_("fill")); }
    void set_fill(const aylis::Color& v) const { set_prop_("fill", v.to_json()); }
    aylis::Color border_color() const { return aylis::Color::from_json(get_prop_("border_color")); }
    void set_border_color(const aylis::Color& v) const { set_prop_("border_color", v.to_json()); }
    aylis::Color text_color() const { return aylis::Color::from_json(get_prop_("text_color")); }
    void set_text_color(const aylis::Color& v) const { set_prop_("text_color", v.to_json()); }
    aylis::Color accent_color() const { return aylis::Color::from_json(get_prop_("accent_color")); }
    void set_accent_color(const aylis::Color& v) const { set_prop_("accent_color", v.to_json()); }
    aylis::Color success_color() const { return aylis::Color::from_json(get_prop_("success_color")); }
    void set_success_color(const aylis::Color& v) const { set_prop_("success_color", v.to_json()); }
    aylis::Color warning_color() const { return aylis::Color::from_json(get_prop_("warning_color")); }
    void set_warning_color(const aylis::Color& v) const { set_prop_("warning_color", v.to_json()); }
    aylis::Color danger_color() const { return aylis::Color::from_json(get_prop_("danger_color")); }
    void set_danger_color(const aylis::Color& v) const { set_prop_("danger_color", v.to_json()); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    FontWeight font_weight() const { return enum_of_<FontWeight>(get_prop_("font_weight"), FontWeight_names); }
    void set_font_weight(FontWeight v) const { set_prop_("font_weight", aylis::Json(FontWeight_names[static_cast<int>(v)])); }
    bool font_italic() const { return get_prop_("font_italic").boolean(); }
    void set_font_italic(bool v) const { set_prop_("font_italic", v); }
    bool font_underline() const { return get_prop_("font_underline").boolean(); }
    void set_font_underline(bool v) const { set_prop_("font_underline", v); }
    bool font_strikeout() const { return get_prop_("font_strikeout").boolean(); }
    void set_font_strikeout(bool v) const { set_prop_("font_strikeout", v); }
    std::string hint() const { return get_prop_("hint").str(); }
    void set_hint(const std::string& v) const { set_prop_("hint", v); }
    long long tag() const { return static_cast<long long>(get_prop_("tag").number()); }
    void set_tag(long long v) const { set_prop_("tag", v); }
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::optional<aylis::Component> popup_menu() const { return get_ref_("popup_menu"); }
    void set_popup_menu(const std::optional<aylis::Component>& v) const { set_ref_("popup_menu", v); }
    std::string on_click() const { return get_prop_("on_click").str(); }
    void set_on_click(const std::string& v) const { set_prop_("on_click", v); }
    std::string on_dbl_click() const { return get_prop_("on_dbl_click").str(); }
    void set_on_dbl_click(const std::string& v) const { set_prop_("on_dbl_click", v); }
    std::string on_long_press() const { return get_prop_("on_long_press").str(); }
    void set_on_long_press(const std::string& v) const { set_prop_("on_long_press", v); }
    std::string on_pointer_down() const { return get_prop_("on_pointer_down").str(); }
    void set_on_pointer_down(const std::string& v) const { set_prop_("on_pointer_down", v); }
    std::string on_pointer_up() const { return get_prop_("on_pointer_up").str(); }
    void set_on_pointer_up(const std::string& v) const { set_prop_("on_pointer_up", v); }
    std::string on_pointer_move() const { return get_prop_("on_pointer_move").str(); }
    void set_on_pointer_move(const std::string& v) const { set_prop_("on_pointer_move", v); }
    std::string on_pointer_enter() const { return get_prop_("on_pointer_enter").str(); }
    void set_on_pointer_enter(const std::string& v) const { set_prop_("on_pointer_enter", v); }
    std::string on_pointer_leave() const { return get_prop_("on_pointer_leave").str(); }
    void set_on_pointer_leave(const std::string& v) const { set_prop_("on_pointer_leave", v); }
    std::string on_wheel() const { return get_prop_("on_wheel").str(); }
    void set_on_wheel(const std::string& v) const { set_prop_("on_wheel", v); }
    std::string on_gesture() const { return get_prop_("on_gesture").str(); }
    void set_on_gesture(const std::string& v) const { set_prop_("on_gesture", v); }
    std::string on_swipe() const { return get_prop_("on_swipe").str(); }
    void set_on_swipe(const std::string& v) const { set_prop_("on_swipe", v); }
    std::string on_key_up() const { return get_prop_("on_key_up").str(); }
    void set_on_key_up(const std::string& v) const { set_prop_("on_key_up", v); }
    std::string on_enter() const { return get_prop_("on_enter").str(); }
    void set_on_enter(const std::string& v) const { set_prop_("on_enter", v); }
    std::string on_exit() const { return get_prop_("on_exit").str(); }
    void set_on_exit(const std::string& v) const { set_prop_("on_exit", v); }
    std::string on_resize() const { return get_prop_("on_resize").str(); }
    void set_on_resize(const std::string& v) const { set_prop_("on_resize", v); }
    std::string on_drag_move() const { return get_prop_("on_drag_move").str(); }
    void set_on_drag_move(const std::string& v) const { set_prop_("on_drag_move", v); }
    std::string on_drag_leave() const { return get_prop_("on_drag_leave").str(); }
    void set_on_drag_leave(const std::string& v) const { set_prop_("on_drag_leave", v); }
    std::string on_drag_drop() const { return get_prop_("on_drag_drop").str(); }
    void set_on_drag_drop(const std::string& v) const { set_prop_("on_drag_drop", v); }
    std::string on_drag_end() const { return get_prop_("on_drag_end").str(); }
    void set_on_drag_end(const std::string& v) const { set_prop_("on_drag_end", v); }
};

// O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
struct FrmIBGEComponents : Form {
    ClientDataSet cdsEstados;
    Field cdsEstadosId;
    Field cdsEstadosSigla;
    Field cdsEstadosNome;
    Field cdsEstadosRegiao;
    Field cdsEstadosPopulacao;
    Field cdsEstadosArea;
    Field cdsEstadosDensidade;
    Field cdsEstadosPib;
    Field cdsEstadosPibPerCapita;
    Field cdsEstadosEstimativa;
    Field cdsEstadosEstimativaAno;
    Field cdsEstadosMunicipios;
    Field cdsEstadosBandeira;
    DataSource dsEstados;
    ClientDataSet cdsMunicipios;
    Field cdsMunicipiosId;
    Field cdsMunicipiosUfId;
    Field cdsMunicipiosUf;
    Field cdsMunicipiosNome;
    Field cdsMunicipiosMesorregiao;
    Field cdsMunicipiosMicrorregiao;
    Field cdsMunicipiosRegiaoIntermediaria;
    Field cdsMunicipiosRegiaoImediata;
    Field cdsMunicipiosPopulacao;
    Field cdsMunicipiosArea;
    Field cdsMunicipiosDensidade;
    Field cdsMunicipiosPib;
    Field cdsMunicipiosPibParticipacao;
    Field cdsMunicipiosPibPerCapita;
    Field cdsMunicipiosEstimativa;
    Field cdsMunicipiosEstimativaAno;
    Field cdsMunicipiosDistritos;
    Field cdsMunicipiosBandeira;
    DataSource dsMunicipios;
    ClientDataSet cdsDistritos;
    Field cdsDistritosId;
    Field cdsDistritosMunicipioId;
    Field cdsDistritosNome;
    DataSource dsDistritos;
    ClientDataSet cdsPopulacao;
    Field cdsPopulacaoAno;
    Field cdsPopulacaoHabitantes;
    DataSource dsPopulacao;
    ClientDataSet cdsMaiores;
    Field cdsMaioresNome;
    Field cdsMaioresHabitantes;
    DataSource dsMaiores;
    Label lblFonte;
    Layout lytListas;
    GroupBox gbEstados;
    DataGrid grdEstados;
    GroupBox gbDistritos;
    DataGrid grdDistritos;
    GroupBox gbMunicipios;
    DataGrid grdMunicipios;
    Splitter splListas;
    Layout lytPainel;
    Panel pnlEstado;
    Svg svgBrasil;
    Layout lytBandeiraEstado;
    Label lblEstado;
    Label lblRegiao;
    Image imgBandeiraEstado;
    Layout lytNumerosEstado;
    Label dbtEstadoPopulacao;
    Label lblEstadoEstimativa;
    Label dbtEstadoEstimativa;
    Label dbtEstadoArea;
    Label dbtEstadoDensidade;
    Label dbtEstadoPib;
    Label dbtEstadoPibPerCapita;
    Label dbtEstadoMunicipios;
    Panel pnlMunicipio;
    Svg svgMunicipio;
    Svg svgEstado;
    Layout lytBandeiraMunicipio;
    Label lblMunicipio;
    Label lblCodigo;
    Image imgBandeiraMunicipio;
    Label lblSemBandeira;
    Layout lytNumerosMunicipio;
    Label dbtMesorregiao;
    Label dbtMicrorregiao;
    Label dbtRegiaoImediata;
    Label dbtPopulacao;
    Label lblEstimativa;
    Label dbtEstimativa;
    Label dbtArea;
    Label dbtDensidade;
    Label dbtPib;
    Label dbtPibPerCapita;
    Label dbtPibParticipacao;
    Layout lytGraficos;
    DBChart dbcPopulacao;
    DBChart dbcMaiores;
    DBChart dbcEstados;

    FrmIBGEComponents() = default;
    using Form::Form;

    // Monta os componentes da janela (o form que não bate com o código é erro).
    void bind(const aylis::Json& list) override {
        cdsEstados = ClientDataSet(resolve_(list, "cdsEstados", "ClientDataSet"));
        cdsEstadosId = Field(resolve_(list, "cdsEstadosId", "Field"));
        cdsEstadosSigla = Field(resolve_(list, "cdsEstadosSigla", "Field"));
        cdsEstadosNome = Field(resolve_(list, "cdsEstadosNome", "Field"));
        cdsEstadosRegiao = Field(resolve_(list, "cdsEstadosRegiao", "Field"));
        cdsEstadosPopulacao = Field(resolve_(list, "cdsEstadosPopulacao", "Field"));
        cdsEstadosArea = Field(resolve_(list, "cdsEstadosArea", "Field"));
        cdsEstadosDensidade = Field(resolve_(list, "cdsEstadosDensidade", "Field"));
        cdsEstadosPib = Field(resolve_(list, "cdsEstadosPib", "Field"));
        cdsEstadosPibPerCapita = Field(resolve_(list, "cdsEstadosPibPerCapita", "Field"));
        cdsEstadosEstimativa = Field(resolve_(list, "cdsEstadosEstimativa", "Field"));
        cdsEstadosEstimativaAno = Field(resolve_(list, "cdsEstadosEstimativaAno", "Field"));
        cdsEstadosMunicipios = Field(resolve_(list, "cdsEstadosMunicipios", "Field"));
        cdsEstadosBandeira = Field(resolve_(list, "cdsEstadosBandeira", "Field"));
        dsEstados = DataSource(resolve_(list, "dsEstados", "DataSource"));
        cdsMunicipios = ClientDataSet(resolve_(list, "cdsMunicipios", "ClientDataSet"));
        cdsMunicipiosId = Field(resolve_(list, "cdsMunicipiosId", "Field"));
        cdsMunicipiosUfId = Field(resolve_(list, "cdsMunicipiosUfId", "Field"));
        cdsMunicipiosUf = Field(resolve_(list, "cdsMunicipiosUf", "Field"));
        cdsMunicipiosNome = Field(resolve_(list, "cdsMunicipiosNome", "Field"));
        cdsMunicipiosMesorregiao = Field(resolve_(list, "cdsMunicipiosMesorregiao", "Field"));
        cdsMunicipiosMicrorregiao = Field(resolve_(list, "cdsMunicipiosMicrorregiao", "Field"));
        cdsMunicipiosRegiaoIntermediaria = Field(resolve_(list, "cdsMunicipiosRegiaoIntermediaria", "Field"));
        cdsMunicipiosRegiaoImediata = Field(resolve_(list, "cdsMunicipiosRegiaoImediata", "Field"));
        cdsMunicipiosPopulacao = Field(resolve_(list, "cdsMunicipiosPopulacao", "Field"));
        cdsMunicipiosArea = Field(resolve_(list, "cdsMunicipiosArea", "Field"));
        cdsMunicipiosDensidade = Field(resolve_(list, "cdsMunicipiosDensidade", "Field"));
        cdsMunicipiosPib = Field(resolve_(list, "cdsMunicipiosPib", "Field"));
        cdsMunicipiosPibParticipacao = Field(resolve_(list, "cdsMunicipiosPibParticipacao", "Field"));
        cdsMunicipiosPibPerCapita = Field(resolve_(list, "cdsMunicipiosPibPerCapita", "Field"));
        cdsMunicipiosEstimativa = Field(resolve_(list, "cdsMunicipiosEstimativa", "Field"));
        cdsMunicipiosEstimativaAno = Field(resolve_(list, "cdsMunicipiosEstimativaAno", "Field"));
        cdsMunicipiosDistritos = Field(resolve_(list, "cdsMunicipiosDistritos", "Field"));
        cdsMunicipiosBandeira = Field(resolve_(list, "cdsMunicipiosBandeira", "Field"));
        dsMunicipios = DataSource(resolve_(list, "dsMunicipios", "DataSource"));
        cdsDistritos = ClientDataSet(resolve_(list, "cdsDistritos", "ClientDataSet"));
        cdsDistritosId = Field(resolve_(list, "cdsDistritosId", "Field"));
        cdsDistritosMunicipioId = Field(resolve_(list, "cdsDistritosMunicipioId", "Field"));
        cdsDistritosNome = Field(resolve_(list, "cdsDistritosNome", "Field"));
        dsDistritos = DataSource(resolve_(list, "dsDistritos", "DataSource"));
        cdsPopulacao = ClientDataSet(resolve_(list, "cdsPopulacao", "ClientDataSet"));
        cdsPopulacaoAno = Field(resolve_(list, "cdsPopulacaoAno", "Field"));
        cdsPopulacaoHabitantes = Field(resolve_(list, "cdsPopulacaoHabitantes", "Field"));
        dsPopulacao = DataSource(resolve_(list, "dsPopulacao", "DataSource"));
        cdsMaiores = ClientDataSet(resolve_(list, "cdsMaiores", "ClientDataSet"));
        cdsMaioresNome = Field(resolve_(list, "cdsMaioresNome", "Field"));
        cdsMaioresHabitantes = Field(resolve_(list, "cdsMaioresHabitantes", "Field"));
        dsMaiores = DataSource(resolve_(list, "dsMaiores", "DataSource"));
        lblFonte = Label(resolve_(list, "lblFonte", "Label"));
        lytListas = Layout(resolve_(list, "lytListas", "Layout"));
        gbEstados = GroupBox(resolve_(list, "gbEstados", "GroupBox"));
        grdEstados = DataGrid(resolve_(list, "grdEstados", "DataGrid"));
        gbDistritos = GroupBox(resolve_(list, "gbDistritos", "GroupBox"));
        grdDistritos = DataGrid(resolve_(list, "grdDistritos", "DataGrid"));
        gbMunicipios = GroupBox(resolve_(list, "gbMunicipios", "GroupBox"));
        grdMunicipios = DataGrid(resolve_(list, "grdMunicipios", "DataGrid"));
        splListas = Splitter(resolve_(list, "splListas", "Splitter"));
        lytPainel = Layout(resolve_(list, "lytPainel", "Layout"));
        pnlEstado = Panel(resolve_(list, "pnlEstado", "Panel"));
        svgBrasil = Svg(resolve_(list, "svgBrasil", "Svg"));
        lytBandeiraEstado = Layout(resolve_(list, "lytBandeiraEstado", "Layout"));
        lblEstado = Label(resolve_(list, "lblEstado", "Label"));
        lblRegiao = Label(resolve_(list, "lblRegiao", "Label"));
        imgBandeiraEstado = Image(resolve_(list, "imgBandeiraEstado", "Image"));
        lytNumerosEstado = Layout(resolve_(list, "lytNumerosEstado", "Layout"));
        dbtEstadoPopulacao = Label(resolve_(list, "dbtEstadoPopulacao", "Label"));
        lblEstadoEstimativa = Label(resolve_(list, "lblEstadoEstimativa", "Label"));
        dbtEstadoEstimativa = Label(resolve_(list, "dbtEstadoEstimativa", "Label"));
        dbtEstadoArea = Label(resolve_(list, "dbtEstadoArea", "Label"));
        dbtEstadoDensidade = Label(resolve_(list, "dbtEstadoDensidade", "Label"));
        dbtEstadoPib = Label(resolve_(list, "dbtEstadoPib", "Label"));
        dbtEstadoPibPerCapita = Label(resolve_(list, "dbtEstadoPibPerCapita", "Label"));
        dbtEstadoMunicipios = Label(resolve_(list, "dbtEstadoMunicipios", "Label"));
        pnlMunicipio = Panel(resolve_(list, "pnlMunicipio", "Panel"));
        svgMunicipio = Svg(resolve_(list, "svgMunicipio", "Svg"));
        svgEstado = Svg(resolve_(list, "svgEstado", "Svg"));
        lytBandeiraMunicipio = Layout(resolve_(list, "lytBandeiraMunicipio", "Layout"));
        lblMunicipio = Label(resolve_(list, "lblMunicipio", "Label"));
        lblCodigo = Label(resolve_(list, "lblCodigo", "Label"));
        imgBandeiraMunicipio = Image(resolve_(list, "imgBandeiraMunicipio", "Image"));
        lblSemBandeira = Label(resolve_(list, "lblSemBandeira", "Label"));
        lytNumerosMunicipio = Layout(resolve_(list, "lytNumerosMunicipio", "Layout"));
        dbtMesorregiao = Label(resolve_(list, "dbtMesorregiao", "Label"));
        dbtMicrorregiao = Label(resolve_(list, "dbtMicrorregiao", "Label"));
        dbtRegiaoImediata = Label(resolve_(list, "dbtRegiaoImediata", "Label"));
        dbtPopulacao = Label(resolve_(list, "dbtPopulacao", "Label"));
        lblEstimativa = Label(resolve_(list, "lblEstimativa", "Label"));
        dbtEstimativa = Label(resolve_(list, "dbtEstimativa", "Label"));
        dbtArea = Label(resolve_(list, "dbtArea", "Label"));
        dbtDensidade = Label(resolve_(list, "dbtDensidade", "Label"));
        dbtPib = Label(resolve_(list, "dbtPib", "Label"));
        dbtPibPerCapita = Label(resolve_(list, "dbtPibPerCapita", "Label"));
        dbtPibParticipacao = Label(resolve_(list, "dbtPibParticipacao", "Label"));
        lytGraficos = Layout(resolve_(list, "lytGraficos", "Layout"));
        dbcPopulacao = DBChart(resolve_(list, "dbcPopulacao", "DBChart"));
        dbcMaiores = DBChart(resolve_(list, "dbcMaiores", "DBChart"));
        dbcEstados = DBChart(resolve_(list, "dbcEstados", "DBChart"));
    }
    // O ClientDataSet deste form com o nome (vazio se não há).
    ClientDataSet find_client_data_set(const std::string& name) const {
        if (name == "cdsEstados") return cdsEstados;
        if (name == "cdsMunicipios") return cdsMunicipios;
        if (name == "cdsDistritos") return cdsDistritos;
        if (name == "cdsPopulacao") return cdsPopulacao;
        if (name == "cdsMaiores") return cdsMaiores;
        return ClientDataSet();
    }
    // O ClientDataSet que disparou o evento.
    ClientDataSet sender_as_client_data_set(const aylis::Event& ev) const { return find_client_data_set(ev.sender_name); }
    // O DBChart deste form com o nome (vazio se não há).
    DBChart find_d_b_chart(const std::string& name) const {
        if (name == "dbcPopulacao") return dbcPopulacao;
        if (name == "dbcMaiores") return dbcMaiores;
        if (name == "dbcEstados") return dbcEstados;
        return DBChart();
    }
    // O DBChart que disparou o evento.
    DBChart sender_as_d_b_chart(const aylis::Event& ev) const { return find_d_b_chart(ev.sender_name); }
    // O DataGrid deste form com o nome (vazio se não há).
    DataGrid find_data_grid(const std::string& name) const {
        if (name == "grdEstados") return grdEstados;
        if (name == "grdDistritos") return grdDistritos;
        if (name == "grdMunicipios") return grdMunicipios;
        return DataGrid();
    }
    // O DataGrid que disparou o evento.
    DataGrid sender_as_data_grid(const aylis::Event& ev) const { return find_data_grid(ev.sender_name); }
    // O DataSource deste form com o nome (vazio se não há).
    DataSource find_data_source(const std::string& name) const {
        if (name == "dsEstados") return dsEstados;
        if (name == "dsMunicipios") return dsMunicipios;
        if (name == "dsDistritos") return dsDistritos;
        if (name == "dsPopulacao") return dsPopulacao;
        if (name == "dsMaiores") return dsMaiores;
        return DataSource();
    }
    // O DataSource que disparou o evento.
    DataSource sender_as_data_source(const aylis::Event& ev) const { return find_data_source(ev.sender_name); }
    // O Field deste form com o nome (vazio se não há).
    Field find_field(const std::string& name) const {
        if (name == "cdsEstadosId") return cdsEstadosId;
        if (name == "cdsEstadosSigla") return cdsEstadosSigla;
        if (name == "cdsEstadosNome") return cdsEstadosNome;
        if (name == "cdsEstadosRegiao") return cdsEstadosRegiao;
        if (name == "cdsEstadosPopulacao") return cdsEstadosPopulacao;
        if (name == "cdsEstadosArea") return cdsEstadosArea;
        if (name == "cdsEstadosDensidade") return cdsEstadosDensidade;
        if (name == "cdsEstadosPib") return cdsEstadosPib;
        if (name == "cdsEstadosPibPerCapita") return cdsEstadosPibPerCapita;
        if (name == "cdsEstadosEstimativa") return cdsEstadosEstimativa;
        if (name == "cdsEstadosEstimativaAno") return cdsEstadosEstimativaAno;
        if (name == "cdsEstadosMunicipios") return cdsEstadosMunicipios;
        if (name == "cdsEstadosBandeira") return cdsEstadosBandeira;
        if (name == "cdsMunicipiosId") return cdsMunicipiosId;
        if (name == "cdsMunicipiosUfId") return cdsMunicipiosUfId;
        if (name == "cdsMunicipiosUf") return cdsMunicipiosUf;
        if (name == "cdsMunicipiosNome") return cdsMunicipiosNome;
        if (name == "cdsMunicipiosMesorregiao") return cdsMunicipiosMesorregiao;
        if (name == "cdsMunicipiosMicrorregiao") return cdsMunicipiosMicrorregiao;
        if (name == "cdsMunicipiosRegiaoIntermediaria") return cdsMunicipiosRegiaoIntermediaria;
        if (name == "cdsMunicipiosRegiaoImediata") return cdsMunicipiosRegiaoImediata;
        if (name == "cdsMunicipiosPopulacao") return cdsMunicipiosPopulacao;
        if (name == "cdsMunicipiosArea") return cdsMunicipiosArea;
        if (name == "cdsMunicipiosDensidade") return cdsMunicipiosDensidade;
        if (name == "cdsMunicipiosPib") return cdsMunicipiosPib;
        if (name == "cdsMunicipiosPibParticipacao") return cdsMunicipiosPibParticipacao;
        if (name == "cdsMunicipiosPibPerCapita") return cdsMunicipiosPibPerCapita;
        if (name == "cdsMunicipiosEstimativa") return cdsMunicipiosEstimativa;
        if (name == "cdsMunicipiosEstimativaAno") return cdsMunicipiosEstimativaAno;
        if (name == "cdsMunicipiosDistritos") return cdsMunicipiosDistritos;
        if (name == "cdsMunicipiosBandeira") return cdsMunicipiosBandeira;
        if (name == "cdsDistritosId") return cdsDistritosId;
        if (name == "cdsDistritosMunicipioId") return cdsDistritosMunicipioId;
        if (name == "cdsDistritosNome") return cdsDistritosNome;
        if (name == "cdsPopulacaoAno") return cdsPopulacaoAno;
        if (name == "cdsPopulacaoHabitantes") return cdsPopulacaoHabitantes;
        if (name == "cdsMaioresNome") return cdsMaioresNome;
        if (name == "cdsMaioresHabitantes") return cdsMaioresHabitantes;
        return Field();
    }
    // O Field que disparou o evento.
    Field sender_as_field(const aylis::Event& ev) const { return find_field(ev.sender_name); }
    // O GroupBox deste form com o nome (vazio se não há).
    GroupBox find_group_box(const std::string& name) const {
        if (name == "gbEstados") return gbEstados;
        if (name == "gbDistritos") return gbDistritos;
        if (name == "gbMunicipios") return gbMunicipios;
        return GroupBox();
    }
    // O GroupBox que disparou o evento.
    GroupBox sender_as_group_box(const aylis::Event& ev) const { return find_group_box(ev.sender_name); }
    // O Image deste form com o nome (vazio se não há).
    Image find_image(const std::string& name) const {
        if (name == "imgBandeiraEstado") return imgBandeiraEstado;
        if (name == "imgBandeiraMunicipio") return imgBandeiraMunicipio;
        return Image();
    }
    // O Image que disparou o evento.
    Image sender_as_image(const aylis::Event& ev) const { return find_image(ev.sender_name); }
    // O Label deste form com o nome (vazio se não há).
    Label find_label(const std::string& name) const {
        if (name == "lblFonte") return lblFonte;
        if (name == "lblEstado") return lblEstado;
        if (name == "lblRegiao") return lblRegiao;
        if (name == "dbtEstadoPopulacao") return dbtEstadoPopulacao;
        if (name == "lblEstadoEstimativa") return lblEstadoEstimativa;
        if (name == "dbtEstadoEstimativa") return dbtEstadoEstimativa;
        if (name == "dbtEstadoArea") return dbtEstadoArea;
        if (name == "dbtEstadoDensidade") return dbtEstadoDensidade;
        if (name == "dbtEstadoPib") return dbtEstadoPib;
        if (name == "dbtEstadoPibPerCapita") return dbtEstadoPibPerCapita;
        if (name == "dbtEstadoMunicipios") return dbtEstadoMunicipios;
        if (name == "lblMunicipio") return lblMunicipio;
        if (name == "lblCodigo") return lblCodigo;
        if (name == "lblSemBandeira") return lblSemBandeira;
        if (name == "dbtMesorregiao") return dbtMesorregiao;
        if (name == "dbtMicrorregiao") return dbtMicrorregiao;
        if (name == "dbtRegiaoImediata") return dbtRegiaoImediata;
        if (name == "dbtPopulacao") return dbtPopulacao;
        if (name == "lblEstimativa") return lblEstimativa;
        if (name == "dbtEstimativa") return dbtEstimativa;
        if (name == "dbtArea") return dbtArea;
        if (name == "dbtDensidade") return dbtDensidade;
        if (name == "dbtPib") return dbtPib;
        if (name == "dbtPibPerCapita") return dbtPibPerCapita;
        if (name == "dbtPibParticipacao") return dbtPibParticipacao;
        return Label();
    }
    // O Label que disparou o evento.
    Label sender_as_label(const aylis::Event& ev) const { return find_label(ev.sender_name); }
    // O Layout deste form com o nome (vazio se não há).
    Layout find_layout(const std::string& name) const {
        if (name == "lytListas") return lytListas;
        if (name == "lytPainel") return lytPainel;
        if (name == "lytBandeiraEstado") return lytBandeiraEstado;
        if (name == "lytNumerosEstado") return lytNumerosEstado;
        if (name == "lytBandeiraMunicipio") return lytBandeiraMunicipio;
        if (name == "lytNumerosMunicipio") return lytNumerosMunicipio;
        if (name == "lytGraficos") return lytGraficos;
        return Layout();
    }
    // O Layout que disparou o evento.
    Layout sender_as_layout(const aylis::Event& ev) const { return find_layout(ev.sender_name); }
    // O Panel deste form com o nome (vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlEstado") return pnlEstado;
        if (name == "pnlMunicipio") return pnlMunicipio;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O Splitter deste form com o nome (vazio se não há).
    Splitter find_splitter(const std::string& name) const {
        if (name == "splListas") return splListas;
        return Splitter();
    }
    // O Splitter que disparou o evento.
    Splitter sender_as_splitter(const aylis::Event& ev) const { return find_splitter(ev.sender_name); }
    // O Svg deste form com o nome (vazio se não há).
    Svg find_svg(const std::string& name) const {
        if (name == "svgBrasil") return svgBrasil;
        if (name == "svgMunicipio") return svgMunicipio;
        if (name == "svgEstado") return svgEstado;
        return Svg();
    }
    // O Svg que disparou o evento.
    Svg sender_as_svg(const aylis::Event& ev) const { return find_svg(ev.sender_name); }
};

}  // namespace aylis_components

// Cada frmX é a janela do form cujo evento está rodando (a primeira, fora dos handlers): a
// instância é copiada para cá antes do handler, então &frmX.componente vale sempre.
inline aylis_components::FrmIBGEComponents frmIBGE;

namespace aylis_components {

// Registra as classes dos forms (o registro dos forms chama).
inline void register_classes() {
    aylis::Aylis::register_form_class(
        "frmIBGE", [](aylis::Aylis* app, long long id, const std::string& name) -> std::unique_ptr<aylis::FormBase> { return std::make_unique<FrmIBGEComponents>(app, id, name); },
        [](aylis::FormBase& form) { ::frmIBGE = static_cast<FrmIBGEComponents&>(form); });
}

}  // namespace aylis_components
