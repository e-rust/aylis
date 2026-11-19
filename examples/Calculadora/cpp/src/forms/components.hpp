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
enum class BlendMode { Normal, Multiply, Screen, Overlay, Darken, Lighten, ColorDodge, ColorBurn, HardLight, SoftLight, Difference, Exclusion, Hue, Saturation, Color, Luminosity };
inline constexpr const char* BlendMode_names[] = {"normal", "multiply", "screen", "overlay", "darken", "lighten", "color_dodge", "color_burn", "hard_light", "soft_light", "difference", "exclusion", "hue", "saturation", "color", "luminosity"};
inline const char* to_string(BlendMode v) { return BlendMode_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class BorderStyle { Sizeable, Single, None, Custom };
inline constexpr const char* BorderStyle_names[] = {"sizeable", "single", "none", "custom"};
inline const char* to_string(BorderStyle v) { return BorderStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ButtonStyle { Default, Accent, Danger, Elevated, Fab, FabExtended, Flat, Ghost, Glass, GlassProminent, Glossy, Icon, IconFilled, IconOutlined, IconTonal, Info, Link, Negative, Outline, Pill, Positive, Push, Reveal, Round, Rounded, Secondary, Success, Text, Tonal, Unelevated, Warning };
inline constexpr const char* ButtonStyle_names[] = {"", "accent", "danger", "elevated", "fab", "fab_extended", "flat", "ghost", "glass", "glass_prominent", "glossy", "icon", "icon_filled", "icon_outlined", "icon_tonal", "info", "link", "negative", "outline", "pill", "positive", "push", "reveal", "round", "rounded", "secondary", "success", "text", "tonal", "unelevated", "warning"};
inline const char* to_string(ButtonStyle v) { return ButtonStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class CloseAction { Hide, Free };
inline constexpr const char* CloseAction_names[] = {"hide", "free"};
inline const char* to_string(CloseAction v) { return CloseAction_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class CursorShape { Default, Pointer, Text, Wait, Crosshair, Move, NotAllowed, ResizeHorizontal, ResizeVertical, ResizeNeSw, ResizeNwSe, Grab, Grabbing, None };
inline constexpr const char* CursorShape_names[] = {"default", "pointer", "text", "wait", "crosshair", "move", "not_allowed", "resize_horizontal", "resize_vertical", "resize_ne_sw", "resize_nw_se", "grab", "grabbing", "none"};
inline const char* to_string(CursorShape v) { return CursorShape_names[static_cast<int>(v)]; }

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
enum class GridLayoutStyle { Default };
inline constexpr const char* GridLayoutStyle_names[] = {""};
inline const char* to_string(GridLayoutStyle v) { return GridLayoutStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class IconPosition { Left, Right, Top };
inline constexpr const char* IconPosition_names[] = {"left", "right", "top"};
inline const char* to_string(IconPosition v) { return IconPosition_names[static_cast<int>(v)]; }

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
enum class ListViewStyle { Default, Flat };
inline constexpr const char* ListViewStyle_names[] = {"", "flat"};
inline const char* to_string(ListViewStyle v) { return ListViewStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class ModalResult { None, Ok, Cancel, Abort, Retry, Ignore, Yes, No, Close };
inline constexpr const char* ModalResult_names[] = {"none", "ok", "cancel", "abort", "retry", "ignore", "yes", "no", "close"};
inline const char* to_string(ModalResult v) { return ModalResult_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class PanelStyle { Default, Appbar, Card, Filled, Flat, Glass, Outlined, Overlay, Reveal, Sidebar };
inline constexpr const char* PanelStyle_names[] = {"", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar"};
inline const char* to_string(PanelStyle v) { return PanelStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class SelfAlign { Stretch, Start, Center, End };
inline constexpr const char* SelfAlign_names[] = {"stretch", "start", "center", "end"};
inline const char* to_string(SelfAlign v) { return SelfAlign_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TabControlStyle { Default };
inline constexpr const char* TabControlStyle_names[] = {""};
inline const char* to_string(TabControlStyle v) { return TabControlStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TabItemStyle { Default };
inline constexpr const char* TabItemStyle_names[] = {""};
inline const char* to_string(TabItemStyle v) { return TabItemStyle_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class TabPosition { Top, Bottom, Left, Right };
inline constexpr const char* TabPosition_names[] = {"top", "bottom", "left", "right"};
inline const char* to_string(TabPosition v) { return TabPosition_names[static_cast<int>(v)]; }

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
enum class Transition { None, Fade, SlideLeft, SlideRight, SlideUp, SlideDown };
inline constexpr const char* Transition_names[] = {"none", "fade", "slide_left", "slide_right", "slide_up", "slide_down"};
inline const char* to_string(Transition v) { return Transition_names[static_cast<int>(v)]; }

// Os valores de um enum do acesso tipado.
enum class Trimming { None, Character, Word };
inline constexpr const char* Trimming_names[] = {"none", "character", "word"};
inline const char* to_string(Trimming v) { return Trimming_names[static_cast<int>(v)]; }

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
struct GestureEvent;
struct KeyEvent;
struct ListViewItem;
struct PointerEvent;
struct SwipeEvent;
struct WheelEvent;

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
struct ListViewItem {
    std::string title;
    std::string detail;

    static ListViewItem from_json(const aylis::Json& v) {
        ListViewItem r;
        r.title = v["title"].str();
        r.detail = v["detail"].str();
        return r;
    }

    aylis::Json to_json() const {
        auto o = aylis::Json::object();
        o.set("title", aylis::Json(title));
        o.set("detail", aylis::Json(detail));
        return o;
    }

    static std::optional<ListViewItem> from_text(const aylis::Json& v) {
        auto j = aylis::Json::parse(v.str());
        if (j.kind() != aylis::Json::Kind::Object) return std::nullopt;
        return from_json(j);
    }
    static std::vector<ListViewItem> list_from_text(const aylis::Json& v) {
        // O JSON numa variável: o temporário do `parse` morreria antes do laço.
        const auto j = aylis::Json::parse(v.str());
        std::vector<ListViewItem> out;
        for (const auto& x : j.items()) out.push_back(from_json(x));
        return out;
    }
    static std::string to_text(const std::optional<ListViewItem>& r) { return r ? r->to_json().dump() : std::string(); }
    static std::string list_to_text(const std::vector<ListViewItem>& items) {
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

// Um Button de um form.
struct Button : aylis::Component {
    Button() = default;
    explicit Button(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    std::string icon() const { return get_prop_("icon").str(); }
    void set_icon(const std::string& v) const { set_prop_("icon", v); }
    std::string checked_icon() const { return get_prop_("checked_icon").str(); }
    void set_checked_icon(const std::string& v) const { set_prop_("checked_icon", v); }
    IconPosition icon_position() const { return enum_of_<IconPosition>(get_prop_("icon_position"), IconPosition_names); }
    void set_icon_position(IconPosition v) const { set_prop_("icon_position", aylis::Json(IconPosition_names[static_cast<int>(v)])); }
    double icon_size() const { return get_prop_("icon_size").number(); }
    void set_icon_size(double v) const { set_prop_("icon_size", v); }
    bool checkable() const { return get_prop_("checkable").boolean(); }
    void set_checkable(bool v) const { set_prop_("checkable", v); }
    bool checked() const { return get_prop_("checked").boolean(); }
    void set_checked(bool v) const { set_prop_("checked", v); }
    ModalResult modal_result() const { return enum_of_<ModalResult>(get_prop_("modal_result"), ModalResult_names); }
    void set_modal_result(ModalResult v) const { set_prop_("modal_result", aylis::Json(ModalResult_names[static_cast<int>(v)])); }
    bool repeat_enter() const { return get_prop_("repeat_enter").boolean(); }
    void set_repeat_enter(bool v) const { set_prop_("repeat_enter", v); }
    bool default_button() const { return get_prop_("default_button").boolean(); }
    void set_default_button(bool v) const { set_prop_("default_button", v); }
    bool cancel_button() const { return get_prop_("cancel_button").boolean(); }
    void set_cancel_button(bool v) const { set_prop_("cancel_button", v); }
    TextAlign h_align() const { return enum_of_<TextAlign>(get_prop_("h_align"), TextAlign_names); }
    void set_h_align(TextAlign v) const { set_prop_("h_align", aylis::Json(TextAlign_names[static_cast<int>(v)])); }
    long long group_index() const { return static_cast<long long>(get_prop_("group_index").number()); }
    void set_group_index(long long v) const { set_prop_("group_index", v); }
    aylis::Bounds radius() const { return aylis::Bounds::from_json(get_prop_("radius")); }
    void set_radius(const aylis::Bounds& v) const { set_prop_("radius", v.to_json()); }
    std::string on_change() const { return get_prop_("on_change").str(); }
    void set_on_change(const std::string& v) const { set_prop_("on_change", v); }
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
    ButtonStyle style() const { return enum_of_<ButtonStyle>(get_prop_("style"), ButtonStyle_names); }
    void set_style(ButtonStyle v) const { set_prop_("style", aylis::Json(ButtonStyle_names[static_cast<int>(v)])); }
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

// Um GridLayout de um form.
struct GridLayout : aylis::Component {
    GridLayout() = default;
    explicit GridLayout(const aylis::Component& c) : aylis::Component(c) {}
    long long columns() const { return static_cast<long long>(get_prop_("columns").number()); }
    void set_columns(long long v) const { set_prop_("columns", v); }
    double min_column_width() const { return get_prop_("min_column_width").number(); }
    void set_min_column_width(double v) const { set_prop_("min_column_width", v); }
    double gap() const { return get_prop_("gap").number(); }
    void set_gap(double v) const { set_prop_("gap", v); }
    double row_height() const { return get_prop_("row_height").number(); }
    void set_row_height(double v) const { set_prop_("row_height", v); }
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
    GridLayoutStyle style() const { return enum_of_<GridLayoutStyle>(get_prop_("style"), GridLayoutStyle_names); }
    void set_style(GridLayoutStyle v) const { set_prop_("style", aylis::Json(GridLayoutStyle_names[static_cast<int>(v)])); }
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

// Um ListView de um form.
struct ListView : aylis::Component {
    ListView() = default;
    explicit ListView(const aylis::Component& c) : aylis::Component(c) {}
    std::vector<ListViewItem> items() const { return ListViewItem::list_from_text(get_prop_("items")); }
    void set_items(const std::vector<ListViewItem>& v) const { set_prop_("items", ListViewItem::list_to_text(v)); }
    long long item_index() const { return static_cast<long long>(get_prop_("item_index").number()); }
    void set_item_index(long long v) const { set_prop_("item_index", v); }
    double item_height() const { return get_prop_("item_height").number(); }
    void set_item_height(double v) const { set_prop_("item_height", v); }
    bool accessory() const { return get_prop_("accessory").boolean(); }
    void set_accessory(bool v) const { set_prop_("accessory", v); }
    std::string on_change() const { return get_prop_("on_change").str(); }
    void set_on_change(const std::string& v) const { set_prop_("on_change", v); }
    std::string on_item_click() const { return get_prop_("on_item_click").str(); }
    void set_on_item_click(const std::string& v) const { set_prop_("on_item_click", v); }
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
    ListViewStyle style() const { return enum_of_<ListViewStyle>(get_prop_("style"), ListViewStyle_names); }
    void set_style(ListViewStyle v) const { set_prop_("style", aylis::Json(ListViewStyle_names[static_cast<int>(v)])); }
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

// Um TabControl de um form.
struct TabControl : aylis::Component {
    TabControl() = default;
    explicit TabControl(const aylis::Component& c) : aylis::Component(c) {}
    long long tab_index() const { return static_cast<long long>(get_prop_("tab_index").number()); }
    void set_tab_index(long long v) const { set_prop_("tab_index", v); }
    bool show_tabs() const { return get_prop_("show_tabs").boolean(); }
    void set_show_tabs(bool v) const { set_prop_("show_tabs", v); }
    Transition transition() const { return enum_of_<Transition>(get_prop_("transition"), Transition_names); }
    void set_transition(Transition v) const { set_prop_("transition", aylis::Json(Transition_names[static_cast<int>(v)])); }
    bool swipe() const { return get_prop_("swipe").boolean(); }
    void set_swipe(bool v) const { set_prop_("swipe", v); }
    TabPosition tab_position() const { return enum_of_<TabPosition>(get_prop_("tab_position"), TabPosition_names); }
    void set_tab_position(TabPosition v) const { set_prop_("tab_position", aylis::Json(TabPosition_names[static_cast<int>(v)])); }
    double tab_width() const { return get_prop_("tab_width").number(); }
    void set_tab_width(double v) const { set_prop_("tab_width", v); }
    std::string on_change() const { return get_prop_("on_change").str(); }
    void set_on_change(const std::string& v) const { set_prop_("on_change", v); }
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
    TabControlStyle style() const { return enum_of_<TabControlStyle>(get_prop_("style"), TabControlStyle_names); }
    void set_style(TabControlStyle v) const { set_prop_("style", aylis::Json(TabControlStyle_names[static_cast<int>(v)])); }
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

// Um TabItem de um form.
struct TabItem : aylis::Component {
    TabItem() = default;
    explicit TabItem(const aylis::Component& c) : aylis::Component(c) {}
    std::string caption() const { return get_prop_("caption").str(); }
    void set_caption(const std::string& v) const { set_prop_("caption", v); }
    std::string on_show() const { return get_prop_("on_show").str(); }
    void set_on_show(const std::string& v) const { set_prop_("on_show", v); }
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
    TabItemStyle style() const { return enum_of_<TabItemStyle>(get_prop_("style"), TabItemStyle_names); }
    void set_style(TabItemStyle v) const { set_prop_("style", aylis::Json(TabItemStyle_names[static_cast<int>(v)])); }
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

// O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
struct FrmCalculadoraComponents : Form {
    Panel pnlLateral;
    TabControl tabLateral;
    TabItem tabHistorico;
    Button btnLimparHistorico;
    ListView lvHistorico;
    TabItem tabMemoria;
    ListView lvMemoria;
    Layout lytCorpo;
    Layout lytTopo;
    Button btnHistorico;
    Label lblTitulo;
    Label lblExpressao;
    Label lblVisor;
    GridLayout grdMemoria;
    Button btnMC;
    Button btnMR;
    Button btnMMais;
    Button btnMMenos;
    Button btnMS;
    GridLayout grdTeclado;
    Button btnPorcento;
    Button btnCE;
    Button btnC;
    Button btnApagar;
    Button btnInverso;
    Button btnQuadrado;
    Button btnRaiz;
    Button btnDividir;
    Button btn7;
    Button btn8;
    Button btn9;
    Button btnMultiplicar;
    Button btn4;
    Button btn5;
    Button btn6;
    Button btnSubtrair;
    Button btn1;
    Button btn2;
    Button btn3;
    Button btnSomar;
    Button btnSinal;
    Button btn0;
    Button btnVirgula;
    Button btnIgual;

    FrmCalculadoraComponents() = default;
    using Form::Form;

    // Monta os componentes da janela (o form que não bate com o código é erro).
    void bind(const aylis::Json& list) override {
        pnlLateral = Panel(resolve_(list, "pnlLateral", "Panel"));
        tabLateral = TabControl(resolve_(list, "tabLateral", "TabControl"));
        tabHistorico = TabItem(resolve_(list, "tabHistorico", "TabItem"));
        btnLimparHistorico = Button(resolve_(list, "btnLimparHistorico", "Button"));
        lvHistorico = ListView(resolve_(list, "lvHistorico", "ListView"));
        tabMemoria = TabItem(resolve_(list, "tabMemoria", "TabItem"));
        lvMemoria = ListView(resolve_(list, "lvMemoria", "ListView"));
        lytCorpo = Layout(resolve_(list, "lytCorpo", "Layout"));
        lytTopo = Layout(resolve_(list, "lytTopo", "Layout"));
        btnHistorico = Button(resolve_(list, "btnHistorico", "Button"));
        lblTitulo = Label(resolve_(list, "lblTitulo", "Label"));
        lblExpressao = Label(resolve_(list, "lblExpressao", "Label"));
        lblVisor = Label(resolve_(list, "lblVisor", "Label"));
        grdMemoria = GridLayout(resolve_(list, "grdMemoria", "GridLayout"));
        btnMC = Button(resolve_(list, "btnMC", "Button"));
        btnMR = Button(resolve_(list, "btnMR", "Button"));
        btnMMais = Button(resolve_(list, "btnMMais", "Button"));
        btnMMenos = Button(resolve_(list, "btnMMenos", "Button"));
        btnMS = Button(resolve_(list, "btnMS", "Button"));
        grdTeclado = GridLayout(resolve_(list, "grdTeclado", "GridLayout"));
        btnPorcento = Button(resolve_(list, "btnPorcento", "Button"));
        btnCE = Button(resolve_(list, "btnCE", "Button"));
        btnC = Button(resolve_(list, "btnC", "Button"));
        btnApagar = Button(resolve_(list, "btnApagar", "Button"));
        btnInverso = Button(resolve_(list, "btnInverso", "Button"));
        btnQuadrado = Button(resolve_(list, "btnQuadrado", "Button"));
        btnRaiz = Button(resolve_(list, "btnRaiz", "Button"));
        btnDividir = Button(resolve_(list, "btnDividir", "Button"));
        btn7 = Button(resolve_(list, "btn7", "Button"));
        btn8 = Button(resolve_(list, "btn8", "Button"));
        btn9 = Button(resolve_(list, "btn9", "Button"));
        btnMultiplicar = Button(resolve_(list, "btnMultiplicar", "Button"));
        btn4 = Button(resolve_(list, "btn4", "Button"));
        btn5 = Button(resolve_(list, "btn5", "Button"));
        btn6 = Button(resolve_(list, "btn6", "Button"));
        btnSubtrair = Button(resolve_(list, "btnSubtrair", "Button"));
        btn1 = Button(resolve_(list, "btn1", "Button"));
        btn2 = Button(resolve_(list, "btn2", "Button"));
        btn3 = Button(resolve_(list, "btn3", "Button"));
        btnSomar = Button(resolve_(list, "btnSomar", "Button"));
        btnSinal = Button(resolve_(list, "btnSinal", "Button"));
        btn0 = Button(resolve_(list, "btn0", "Button"));
        btnVirgula = Button(resolve_(list, "btnVirgula", "Button"));
        btnIgual = Button(resolve_(list, "btnIgual", "Button"));
    }
    // O Button deste form com o nome (vazio se não há).
    Button find_button(const std::string& name) const {
        if (name == "btnLimparHistorico") return btnLimparHistorico;
        if (name == "btnHistorico") return btnHistorico;
        if (name == "btnMC") return btnMC;
        if (name == "btnMR") return btnMR;
        if (name == "btnMMais") return btnMMais;
        if (name == "btnMMenos") return btnMMenos;
        if (name == "btnMS") return btnMS;
        if (name == "btnPorcento") return btnPorcento;
        if (name == "btnCE") return btnCE;
        if (name == "btnC") return btnC;
        if (name == "btnApagar") return btnApagar;
        if (name == "btnInverso") return btnInverso;
        if (name == "btnQuadrado") return btnQuadrado;
        if (name == "btnRaiz") return btnRaiz;
        if (name == "btnDividir") return btnDividir;
        if (name == "btn7") return btn7;
        if (name == "btn8") return btn8;
        if (name == "btn9") return btn9;
        if (name == "btnMultiplicar") return btnMultiplicar;
        if (name == "btn4") return btn4;
        if (name == "btn5") return btn5;
        if (name == "btn6") return btn6;
        if (name == "btnSubtrair") return btnSubtrair;
        if (name == "btn1") return btn1;
        if (name == "btn2") return btn2;
        if (name == "btn3") return btn3;
        if (name == "btnSomar") return btnSomar;
        if (name == "btnSinal") return btnSinal;
        if (name == "btn0") return btn0;
        if (name == "btnVirgula") return btnVirgula;
        if (name == "btnIgual") return btnIgual;
        return Button();
    }
    // O Button que disparou o evento.
    Button sender_as_button(const aylis::Event& ev) const { return find_button(ev.sender_name); }
    // O GridLayout deste form com o nome (vazio se não há).
    GridLayout find_grid_layout(const std::string& name) const {
        if (name == "grdMemoria") return grdMemoria;
        if (name == "grdTeclado") return grdTeclado;
        return GridLayout();
    }
    // O GridLayout que disparou o evento.
    GridLayout sender_as_grid_layout(const aylis::Event& ev) const { return find_grid_layout(ev.sender_name); }
    // O Label deste form com o nome (vazio se não há).
    Label find_label(const std::string& name) const {
        if (name == "lblTitulo") return lblTitulo;
        if (name == "lblExpressao") return lblExpressao;
        if (name == "lblVisor") return lblVisor;
        return Label();
    }
    // O Label que disparou o evento.
    Label sender_as_label(const aylis::Event& ev) const { return find_label(ev.sender_name); }
    // O Layout deste form com o nome (vazio se não há).
    Layout find_layout(const std::string& name) const {
        if (name == "lytCorpo") return lytCorpo;
        if (name == "lytTopo") return lytTopo;
        return Layout();
    }
    // O Layout que disparou o evento.
    Layout sender_as_layout(const aylis::Event& ev) const { return find_layout(ev.sender_name); }
    // O ListView deste form com o nome (vazio se não há).
    ListView find_list_view(const std::string& name) const {
        if (name == "lvHistorico") return lvHistorico;
        if (name == "lvMemoria") return lvMemoria;
        return ListView();
    }
    // O ListView que disparou o evento.
    ListView sender_as_list_view(const aylis::Event& ev) const { return find_list_view(ev.sender_name); }
    // O Panel deste form com o nome (vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlLateral") return pnlLateral;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O TabControl deste form com o nome (vazio se não há).
    TabControl find_tab_control(const std::string& name) const {
        if (name == "tabLateral") return tabLateral;
        return TabControl();
    }
    // O TabControl que disparou o evento.
    TabControl sender_as_tab_control(const aylis::Event& ev) const { return find_tab_control(ev.sender_name); }
    // O TabItem deste form com o nome (vazio se não há).
    TabItem find_tab_item(const std::string& name) const {
        if (name == "tabHistorico") return tabHistorico;
        if (name == "tabMemoria") return tabMemoria;
        return TabItem();
    }
    // O TabItem que disparou o evento.
    TabItem sender_as_tab_item(const aylis::Event& ev) const { return find_tab_item(ev.sender_name); }
};

}  // namespace aylis_components

// Cada frmX é a janela do form cujo evento está rodando (a primeira, fora dos handlers): a
// instância é copiada para cá antes do handler, então &frmX.componente vale sempre.
inline aylis_components::FrmCalculadoraComponents frmCalculadora;

namespace aylis_components {

// Registra as classes dos forms (o registro dos forms chama).
inline void register_classes() {
    aylis::Aylis::register_form_class(
        "frmCalculadora", [](aylis::Aylis* app, long long id, const std::string& name) -> std::unique_ptr<aylis::FormBase> { return std::make_unique<FrmCalculadoraComponents>(app, id, name); },
        [](aylis::FormBase& form) { ::frmCalculadora = static_cast<FrmCalculadoraComponents&>(form); });
}

}  // namespace aylis_components
