// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include <memory>
#include <optional>
#include <string>

#include "../aylis.hpp"

namespace aylis_components {

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
    std::string icon_position() const { return get_prop_("icon_position").str(); }
    void set_icon_position(const std::string& v) const { set_prop_("icon_position", v); }
    double icon_size() const { return get_prop_("icon_size").number(); }
    void set_icon_size(double v) const { set_prop_("icon_size", v); }
    bool checkable() const { return get_prop_("checkable").boolean(); }
    void set_checkable(bool v) const { set_prop_("checkable", v); }
    bool checked() const { return get_prop_("checked").boolean(); }
    void set_checked(bool v) const { set_prop_("checked", v); }
    std::string modal_result() const { return get_prop_("modal_result").str(); }
    void set_modal_result(const std::string& v) const { set_prop_("modal_result", v); }
    bool repeat_enter() const { return get_prop_("repeat_enter").boolean(); }
    void set_repeat_enter(bool v) const { set_prop_("repeat_enter", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um CheckBox de um form.
struct CheckBox : aylis::Component {
    CheckBox() = default;
    explicit CheckBox(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    bool checked() const { return get_prop_("checked").boolean(); }
    void set_checked(bool v) const { set_prop_("checked", v); }
    double box_size() const { return get_prop_("box_size").number(); }
    void set_box_size(double v) const { set_prop_("box_size", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um Circle de um form.
struct Circle : aylis::Component {
    Circle() = default;
    explicit Circle(const aylis::Component& c) : aylis::Component(c) {}
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string stroke() const { return get_prop_("stroke").str(); }
    void set_stroke(const std::string& v) const { set_prop_("stroke", v); }
    double stroke_width() const { return get_prop_("stroke_width").number(); }
    void set_stroke_width(double v) const { set_prop_("stroke_width", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um ComboBox de um form.
struct ComboBox : aylis::Component {
    ComboBox() = default;
    explicit ComboBox(const aylis::Component& c) : aylis::Component(c) {}
    std::string items() const { return get_prop_("items").str(); }
    void set_items(const std::string& v) const { set_prop_("items", v); }
    std::string values() const { return get_prop_("values").str(); }
    void set_values(const std::string& v) const { set_prop_("values", v); }
    long long item_index() const { return static_cast<long long>(get_prop_("item_index").number()); }
    void set_item_index(long long v) const { set_prop_("item_index", v); }
    std::string h_align() const { return get_prop_("h_align").str(); }
    void set_h_align(const std::string& v) const { set_prop_("h_align", v); }
    std::string trimming() const { return get_prop_("trimming").str(); }
    void set_trimming(const std::string& v) const { set_prop_("trimming", v); }
    std::string arrow() const { return get_prop_("arrow").str(); }
    void set_arrow(const std::string& v) const { set_prop_("arrow", v); }
    bool sensitive() const { return get_prop_("sensitive").boolean(); }
    void set_sensitive(bool v) const { set_prop_("sensitive", v); }
    std::string value() const { return get_prop_("value").str(); }
    void set_value(const std::string& v) const { set_prop_("value", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um Divider de um form.
struct Divider : aylis::Component {
    Divider() = default;
    explicit Divider(const aylis::Component& c) : aylis::Component(c) {}
    bool vertical() const { return get_prop_("vertical").boolean(); }
    void set_vertical(bool v) const { set_prop_("vertical", v); }
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    double thickness() const { return get_prop_("thickness").number(); }
    void set_thickness(double v) const { set_prop_("thickness", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um Form de um form.
struct Form : aylis::FormBase {
    Form() = default;
    using aylis::FormBase::FormBase;
    std::string caption() const { return get_prop_("caption").str(); }
    void set_caption(const std::string& v) const { set_prop_("caption", v); }
    std::string position() const { return get_prop_("position").str(); }
    void set_position(const std::string& v) const { set_prop_("position", v); }
    std::string border_style() const { return get_prop_("border_style").str(); }
    void set_border_style(const std::string& v) const { set_prop_("border_style", v); }
    std::string window_state() const { return get_prop_("window_state").str(); }
    void set_window_state(const std::string& v) const { set_prop_("window_state", v); }
    std::string transition() const { return get_prop_("transition").str(); }
    void set_transition(const std::string& v) const { set_prop_("transition", v); }
    bool topmost() const { return get_prop_("topmost").boolean(); }
    void set_topmost(bool v) const { set_prop_("topmost", v); }
    bool show_in_taskbar() const { return get_prop_("show_in_taskbar").boolean(); }
    void set_show_in_taskbar(bool v) const { set_prop_("show_in_taskbar", v); }
    std::string active_control() const { return get_prop_("active_control").str(); }
    void set_active_control(const std::string& v) const { set_prop_("active_control", v); }
    std::string tab_order() const { return get_prop_("tab_order").str(); }
    void set_tab_order(const std::string& v) const { set_prop_("tab_order", v); }
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
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void move_to_monitor(long long a1) const { tell_("invoke", {{"target", aylis::Json(id_)}, {"method", "move_to_monitor"}, {"args", aylis::Json::array({a1})}}); }
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
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um Label de um form.
struct Label : aylis::Component {
    Label() = default;
    explicit Label(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    std::string h_align() const { return get_prop_("h_align").str(); }
    void set_h_align(const std::string& v) const { set_prop_("h_align", v); }
    std::string v_align() const { return get_prop_("v_align").str(); }
    void set_v_align(const std::string& v) const { set_prop_("v_align", v); }
    bool wrap() const { return get_prop_("wrap").boolean(); }
    void set_wrap(bool v) const { set_prop_("wrap", v); }
    std::string trimming() const { return get_prop_("trimming").str(); }
    void set_trimming(const std::string& v) const { set_prop_("trimming", v); }
    bool bold() const { return get_prop_("bold").boolean(); }
    void set_bold(bool v) const { set_prop_("bold", v); }
    std::string color() const { return get_prop_("color").str(); }
    void set_color(const std::string& v) const { set_prop_("color", v); }
    std::string highlight() const { return get_prop_("highlight").str(); }
    void set_highlight(const std::string& v) const { set_prop_("highlight", v); }
    double letter_spacing() const { return get_prop_("letter_spacing").number(); }
    void set_letter_spacing(double v) const { set_prop_("letter_spacing", v); }
    std::string text_case() const { return get_prop_("text_case").str(); }
    void set_text_case(const std::string& v) const { set_prop_("text_case", v); }
    bool markup() const { return get_prop_("markup").boolean(); }
    void set_markup(bool v) const { set_prop_("markup", v); }
    std::string outline() const { return get_prop_("outline").str(); }
    void set_outline(const std::string& v) const { set_prop_("outline", v); }
    double outline_width() const { return get_prop_("outline_width").number(); }
    void set_outline_width(double v) const { set_prop_("outline_width", v); }
    std::string focus_control() const { return get_prop_("focus_control").str(); }
    void set_focus_control(const std::string& v) const { set_prop_("focus_control", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
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
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um MainMenu de um form.
struct MainMenu : aylis::Component {
    MainMenu() = default;
    explicit MainMenu(const aylis::Component& c) : aylis::Component(c) {}
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um MenuItem de um form.
struct MenuItem : aylis::Component {
    MenuItem() = default;
    explicit MenuItem(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    std::string shortcut() const { return get_prop_("shortcut").str(); }
    void set_shortcut(const std::string& v) const { set_prop_("shortcut", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool checked() const { return get_prop_("checked").boolean(); }
    void set_checked(bool v) const { set_prop_("checked", v); }
    bool auto_check() const { return get_prop_("auto_check").boolean(); }
    void set_auto_check(bool v) const { set_prop_("auto_check", v); }
    std::string icon() const { return get_prop_("icon").str(); }
    void set_icon(const std::string& v) const { set_prop_("icon", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool danger() const { return get_prop_("danger").boolean(); }
    void set_danger(bool v) const { set_prop_("danger", v); }
};

// Um OpenDialog de um form.
struct OpenDialog : aylis::Component {
    OpenDialog() = default;
    explicit OpenDialog(const aylis::Component& c) : aylis::Component(c) {}
    std::string title() const { return get_prop_("title").str(); }
    void set_title(const std::string& v) const { set_prop_("title", v); }
    std::string filter() const { return get_prop_("filter").str(); }
    void set_filter(const std::string& v) const { set_prop_("filter", v); }
    std::string initial_dir() const { return get_prop_("initial_dir").str(); }
    void set_initial_dir(const std::string& v) const { set_prop_("initial_dir", v); }
    std::string file_name() const { return get_prop_("file_name").str(); }
    void set_file_name(const std::string& v) const { set_prop_("file_name", v); }
    bool multiple() const { return get_prop_("multiple").boolean(); }
    void set_multiple(bool v) const { set_prop_("multiple", v); }
    void execute() const { tell_("execute", {{"target", aylis::Json(id_)}}); }
};

// Um PaintCanvas de um form.
struct PaintCanvas : aylis::Component {
    PaintCanvas() = default;
    explicit PaintCanvas(const aylis::Component& c) : aylis::Component(c) {}
    std::string tool() const { return get_prop_("tool").str(); }
    void set_tool(const std::string& v) const { set_prop_("tool", v); }
    std::string color() const { return get_prop_("color").str(); }
    void set_color(const std::string& v) const { set_prop_("color", v); }
    std::string back_color() const { return get_prop_("back_color").str(); }
    void set_back_color(const std::string& v) const { set_prop_("back_color", v); }
    long long brush_size() const { return static_cast<long long>(get_prop_("brush_size").number()); }
    void set_brush_size(long long v) const { set_prop_("brush_size", v); }
    bool filled() const { return get_prop_("filled").boolean(); }
    void set_filled(bool v) const { set_prop_("filled", v); }
    double zoom() const { return get_prop_("zoom").number(); }
    void set_zoom(double v) const { set_prop_("zoom", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    bool undo() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "undo"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool redo() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "redo"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool can_undo() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "can_undo"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool can_redo() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "can_redo"}, {"args", aylis::Json::array({})}}).boolean(); }
    std::string size() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "size"}, {"args", aylis::Json::array({})}}).str(); }
    std::string checksum() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "checksum"}, {"args", aylis::Json::array({})}}).str(); }
    bool clear() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "clear"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool resize(long long width, long long height) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "resize"}, {"args", aylis::Json::array({width, height})}}).boolean(); }
    bool new_(long long width, long long height) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "new"}, {"args", aylis::Json::array({width, height})}}).boolean(); }
    bool open(const std::string& path) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "open"}, {"args", aylis::Json::array({path})}}).boolean(); }
    bool save(const std::string& path) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "save"}, {"args", aylis::Json::array({path})}}).boolean(); }
    bool select_all() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "select_all"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool has_selection() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "has_selection"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool copy() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "copy"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool cut() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "cut"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool paste() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "paste"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool delete_() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "delete"}, {"args", aylis::Json::array({})}}).boolean(); }
    bool deselect() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "deselect"}, {"args", aylis::Json::array({})}}).boolean(); }
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
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string fill2() const { return get_prop_("fill2").str(); }
    void set_fill2(const std::string& v) const { set_prop_("fill2", v); }
    std::string fill_mid() const { return get_prop_("fill_mid").str(); }
    void set_fill_mid(const std::string& v) const { set_prop_("fill_mid", v); }
    double fill_mid_at() const { return get_prop_("fill_mid_at").number(); }
    void set_fill_mid_at(double v) const { set_prop_("fill_mid_at", v); }
    double gradient_angle() const { return get_prop_("gradient_angle").number(); }
    void set_gradient_angle(double v) const { set_prop_("gradient_angle", v); }
    bool gradient_radial() const { return get_prop_("gradient_radial").boolean(); }
    void set_gradient_radial(bool v) const { set_prop_("gradient_radial", v); }
    std::string stroke() const { return get_prop_("stroke").str(); }
    void set_stroke(const std::string& v) const { set_prop_("stroke", v); }
    double stroke_width() const { return get_prop_("stroke_width").number(); }
    void set_stroke_width(double v) const { set_prop_("stroke_width", v); }
    double radius() const { return get_prop_("radius").number(); }
    void set_radius(double v) const { set_prop_("radius", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um SaveDialog de um form.
struct SaveDialog : aylis::Component {
    SaveDialog() = default;
    explicit SaveDialog(const aylis::Component& c) : aylis::Component(c) {}
    std::string title() const { return get_prop_("title").str(); }
    void set_title(const std::string& v) const { set_prop_("title", v); }
    std::string filter() const { return get_prop_("filter").str(); }
    void set_filter(const std::string& v) const { set_prop_("filter", v); }
    std::string initial_dir() const { return get_prop_("initial_dir").str(); }
    void set_initial_dir(const std::string& v) const { set_prop_("initial_dir", v); }
    std::string file_name() const { return get_prop_("file_name").str(); }
    void set_file_name(const std::string& v) const { set_prop_("file_name", v); }
    bool multiple() const { return get_prop_("multiple").boolean(); }
    void set_multiple(bool v) const { set_prop_("multiple", v); }
    void execute() const { tell_("execute", {{"target", aylis::Json(id_)}}); }
};

// Um ScrollBox de um form.
struct ScrollBox : aylis::Component {
    ScrollBox() = default;
    explicit ScrollBox(const aylis::Component& c) : aylis::Component(c) {}
    bool horizontal() const { return get_prop_("horizontal").boolean(); }
    void set_horizontal(bool v) const { set_prop_("horizontal", v); }
    bool pull_to_refresh() const { return get_prop_("pull_to_refresh").boolean(); }
    void set_pull_to_refresh(bool v) const { set_prop_("pull_to_refresh", v); }
    bool refreshing() const { return get_prop_("refreshing").boolean(); }
    void set_refreshing(bool v) const { set_prop_("refreshing", v); }
    double wheel_step() const { return get_prop_("wheel_step").number(); }
    void set_wheel_step(double v) const { set_prop_("wheel_step", v); }
    std::string smooth() const { return get_prop_("smooth").str(); }
    void set_smooth(const std::string& v) const { set_prop_("smooth", v); }
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
    double scroll_position() const { return get_prop_("scroll_position").number(); }
    void set_scroll_position(double v) const { set_prop_("scroll_position", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void scroll_to(const aylis::Component& a1) const { tell_("invoke", {{"target", aylis::Json(id_)}, {"method", "scroll_to"}, {"args", aylis::Json::array({a1.id()})}}); }
    std::string visible_components(const aylis::Component& a1, bool a2) const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "visible_components"}, {"args", aylis::Json::array({a1.id(), a2})}}).str(); }
};

// Um StatusBar de um form.
struct StatusBar : aylis::Component {
    StatusBar() = default;
    explicit StatusBar(const aylis::Component& c) : aylis::Component(c) {}
    std::string text() const { return get_prop_("text").str(); }
    void set_text(const std::string& v) const { set_prop_("text", v); }
    double left() const { return get_prop_("left").number(); }
    void set_left(double v) const { set_prop_("left", v); }
    double top() const { return get_prop_("top").number(); }
    void set_top(double v) const { set_prop_("top", v); }
    double width() const { return get_prop_("width").number(); }
    void set_width(double v) const { set_prop_("width", v); }
    double height() const { return get_prop_("height").number(); }
    void set_height(double v) const { set_prop_("height", v); }
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    bool visible() const { return get_prop_("visible").boolean(); }
    void set_visible(bool v) const { set_prop_("visible", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    bool hit_test() const { return get_prop_("hit_test").boolean(); }
    void set_hit_test(bool v) const { set_prop_("hit_test", v); }
    bool clip_children() const { return get_prop_("clip_children").boolean(); }
    void set_clip_children(bool v) const { set_prop_("clip_children", v); }
    bool captures_wheel() const { return get_prop_("captures_wheel").boolean(); }
    void set_captures_wheel(bool v) const { set_prop_("captures_wheel", v); }
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
    std::string drag_image() const { return get_prop_("drag_image").str(); }
    void set_drag_image(const std::string& v) const { set_prop_("drag_image", v); }
    std::string drop_types() const { return get_prop_("drop_types").str(); }
    void set_drop_types(const std::string& v) const { set_prop_("drop_types", v); }
    bool tab_stop() const { return get_prop_("tab_stop").boolean(); }
    void set_tab_stop(bool v) const { set_prop_("tab_stop", v); }
    std::string cursor() const { return get_prop_("cursor").str(); }
    void set_cursor(const std::string& v) const { set_prop_("cursor", v); }
    double opacity() const { return get_prop_("opacity").number(); }
    void set_opacity(double v) const { set_prop_("opacity", v); }
    bool cache_layer() const { return get_prop_("cache_layer").boolean(); }
    void set_cache_layer(bool v) const { set_prop_("cache_layer", v); }
    double rotation() const { return get_prop_("rotation").number(); }
    void set_rotation(double v) const { set_prop_("rotation", v); }
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
    std::string self_align() const { return get_prop_("self_align").str(); }
    void set_self_align(const std::string& v) const { set_prop_("self_align", v); }
    bool auto_size() const { return get_prop_("auto_size").boolean(); }
    void set_auto_size(bool v) const { set_prop_("auto_size", v); }
    long long column_span() const { return static_cast<long long>(get_prop_("column_span").number()); }
    void set_column_span(long long v) const { set_prop_("column_span", v); }
    std::string shadow() const { return get_prop_("shadow").str(); }
    void set_shadow(const std::string& v) const { set_prop_("shadow", v); }
    std::string style() const { return get_prop_("style").str(); }
    void set_style(const std::string& v) const { set_prop_("style", v); }
    std::string theme() const { return get_prop_("theme").str(); }
    void set_theme(const std::string& v) const { set_prop_("theme", v); }
    std::string fill() const { return get_prop_("fill").str(); }
    void set_fill(const std::string& v) const { set_prop_("fill", v); }
    std::string border_color() const { return get_prop_("border_color").str(); }
    void set_border_color(const std::string& v) const { set_prop_("border_color", v); }
    std::string text_color() const { return get_prop_("text_color").str(); }
    void set_text_color(const std::string& v) const { set_prop_("text_color", v); }
    std::string accent_color() const { return get_prop_("accent_color").str(); }
    void set_accent_color(const std::string& v) const { set_prop_("accent_color", v); }
    std::string success_color() const { return get_prop_("success_color").str(); }
    void set_success_color(const std::string& v) const { set_prop_("success_color", v); }
    std::string warning_color() const { return get_prop_("warning_color").str(); }
    void set_warning_color(const std::string& v) const { set_prop_("warning_color", v); }
    std::string danger_color() const { return get_prop_("danger_color").str(); }
    void set_danger_color(const std::string& v) const { set_prop_("danger_color", v); }
    std::string font_family() const { return get_prop_("font_family").str(); }
    void set_font_family(const std::string& v) const { set_prop_("font_family", v); }
    double font_size() const { return get_prop_("font_size").number(); }
    void set_font_size(double v) const { set_prop_("font_size", v); }
    std::string font_weight() const { return get_prop_("font_weight").str(); }
    void set_font_weight(const std::string& v) const { set_prop_("font_weight", v); }
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
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
struct FrmPaintComponents : Form {
    Layout lytTopo;
    MainMenu mnuPrincipal;
    MenuItem mnuArquivo;
    MenuItem mnuNovo;
    MenuItem mnuAbrir;
    MenuItem mnuSalvar;
    MenuItem mnuSalvarComo;
    MenuItem mnuSep1;
    MenuItem mnuSair;
    MenuItem mnuEditar;
    MenuItem mnuDesfazer;
    MenuItem mnuRefazer;
    MenuItem mnuSep2;
    MenuItem mnuRecortar;
    MenuItem mnuCopiar;
    MenuItem mnuColar;
    MenuItem mnuExcluir;
    MenuItem mnuSelecionarTudo;
    MenuItem mnuSep3;
    MenuItem mnuLimpar;
    MenuItem mnuExibir;
    MenuItem mnuAmpliar;
    MenuItem mnuReduzir;
    MenuItem mnuTamanhoReal;
    Divider divTopo;
    Button btnSalvar;
    Button btnDesfazer;
    Button btnRefazer;
    Panel pnlFaixa;
    Layout lytSelecao;
    Label lblSelecaoGrupo;
    Button btnSelecionar;
    Divider divSelecao;
    Layout lytFerramentas;
    Label lblFerramentasGrupo;
    GridLayout grdFerramentas;
    Button btnLapis;
    Button btnBalde;
    Button btnBorracha;
    Button btnContaGotas;
    Divider divFerramentas;
    Layout lytPinceis;
    Label lblPinceis;
    Button btnPincel;
    Divider divPinceis;
    Layout lytFormas;
    Label lblFormas;
    CheckBox chkPreencher;
    GridLayout grdFormas;
    Button btnLinha;
    Button btnRetangulo;
    Button btnElipse;
    Divider divFormas;
    Layout lytTamanho;
    Label lblTamanhoGrupo;
    ComboBox cbEspessura;
    Divider divTamanho;
    Layout lytCores;
    Label lblCores;
    Circle btnCor1;
    Circle btnCor2;
    GridLayout grdPaleta;
    Circle btnPaleta01;
    Circle btnPaleta02;
    Circle btnPaleta03;
    Circle btnPaleta04;
    Circle btnPaleta05;
    Circle btnPaleta06;
    Circle btnPaleta07;
    Circle btnPaleta08;
    Circle btnPaleta09;
    Circle btnPaleta10;
    Circle btnPaleta11;
    Circle btnPaleta12;
    Circle btnPaleta13;
    Circle btnPaleta14;
    Circle btnPaleta15;
    Circle btnPaleta16;
    Circle btnPaleta17;
    Circle btnPaleta18;
    Circle btnPaleta19;
    Circle btnPaleta20;
    StatusBar stbStatus;
    Label lblPosicao;
    Label lblTamanho;
    Label lblZoom;
    ScrollBox sbxArea;
    PaintCanvas pcDesenho;
    OpenDialog dlgAbrir;
    SaveDialog dlgSalvar;

    FrmPaintComponents() = default;
    using Form::Form;

    // Monta os componentes da janela (o form que não bate com o código é erro).
    void bind(const aylis::Json& list) override {
        lytTopo = Layout(resolve_(list, "lytTopo", "Layout"));
        mnuPrincipal = MainMenu(resolve_(list, "mnuPrincipal", "MainMenu"));
        mnuArquivo = MenuItem(resolve_(list, "mnuArquivo", "MenuItem"));
        mnuNovo = MenuItem(resolve_(list, "mnuNovo", "MenuItem"));
        mnuAbrir = MenuItem(resolve_(list, "mnuAbrir", "MenuItem"));
        mnuSalvar = MenuItem(resolve_(list, "mnuSalvar", "MenuItem"));
        mnuSalvarComo = MenuItem(resolve_(list, "mnuSalvarComo", "MenuItem"));
        mnuSep1 = MenuItem(resolve_(list, "mnuSep1", "MenuItem"));
        mnuSair = MenuItem(resolve_(list, "mnuSair", "MenuItem"));
        mnuEditar = MenuItem(resolve_(list, "mnuEditar", "MenuItem"));
        mnuDesfazer = MenuItem(resolve_(list, "mnuDesfazer", "MenuItem"));
        mnuRefazer = MenuItem(resolve_(list, "mnuRefazer", "MenuItem"));
        mnuSep2 = MenuItem(resolve_(list, "mnuSep2", "MenuItem"));
        mnuRecortar = MenuItem(resolve_(list, "mnuRecortar", "MenuItem"));
        mnuCopiar = MenuItem(resolve_(list, "mnuCopiar", "MenuItem"));
        mnuColar = MenuItem(resolve_(list, "mnuColar", "MenuItem"));
        mnuExcluir = MenuItem(resolve_(list, "mnuExcluir", "MenuItem"));
        mnuSelecionarTudo = MenuItem(resolve_(list, "mnuSelecionarTudo", "MenuItem"));
        mnuSep3 = MenuItem(resolve_(list, "mnuSep3", "MenuItem"));
        mnuLimpar = MenuItem(resolve_(list, "mnuLimpar", "MenuItem"));
        mnuExibir = MenuItem(resolve_(list, "mnuExibir", "MenuItem"));
        mnuAmpliar = MenuItem(resolve_(list, "mnuAmpliar", "MenuItem"));
        mnuReduzir = MenuItem(resolve_(list, "mnuReduzir", "MenuItem"));
        mnuTamanhoReal = MenuItem(resolve_(list, "mnuTamanhoReal", "MenuItem"));
        divTopo = Divider(resolve_(list, "divTopo", "Divider"));
        btnSalvar = Button(resolve_(list, "btnSalvar", "Button"));
        btnDesfazer = Button(resolve_(list, "btnDesfazer", "Button"));
        btnRefazer = Button(resolve_(list, "btnRefazer", "Button"));
        pnlFaixa = Panel(resolve_(list, "pnlFaixa", "Panel"));
        lytSelecao = Layout(resolve_(list, "lytSelecao", "Layout"));
        lblSelecaoGrupo = Label(resolve_(list, "lblSelecaoGrupo", "Label"));
        btnSelecionar = Button(resolve_(list, "btnSelecionar", "Button"));
        divSelecao = Divider(resolve_(list, "divSelecao", "Divider"));
        lytFerramentas = Layout(resolve_(list, "lytFerramentas", "Layout"));
        lblFerramentasGrupo = Label(resolve_(list, "lblFerramentasGrupo", "Label"));
        grdFerramentas = GridLayout(resolve_(list, "grdFerramentas", "GridLayout"));
        btnLapis = Button(resolve_(list, "btnLapis", "Button"));
        btnBalde = Button(resolve_(list, "btnBalde", "Button"));
        btnBorracha = Button(resolve_(list, "btnBorracha", "Button"));
        btnContaGotas = Button(resolve_(list, "btnContaGotas", "Button"));
        divFerramentas = Divider(resolve_(list, "divFerramentas", "Divider"));
        lytPinceis = Layout(resolve_(list, "lytPinceis", "Layout"));
        lblPinceis = Label(resolve_(list, "lblPinceis", "Label"));
        btnPincel = Button(resolve_(list, "btnPincel", "Button"));
        divPinceis = Divider(resolve_(list, "divPinceis", "Divider"));
        lytFormas = Layout(resolve_(list, "lytFormas", "Layout"));
        lblFormas = Label(resolve_(list, "lblFormas", "Label"));
        chkPreencher = CheckBox(resolve_(list, "chkPreencher", "CheckBox"));
        grdFormas = GridLayout(resolve_(list, "grdFormas", "GridLayout"));
        btnLinha = Button(resolve_(list, "btnLinha", "Button"));
        btnRetangulo = Button(resolve_(list, "btnRetangulo", "Button"));
        btnElipse = Button(resolve_(list, "btnElipse", "Button"));
        divFormas = Divider(resolve_(list, "divFormas", "Divider"));
        lytTamanho = Layout(resolve_(list, "lytTamanho", "Layout"));
        lblTamanhoGrupo = Label(resolve_(list, "lblTamanhoGrupo", "Label"));
        cbEspessura = ComboBox(resolve_(list, "cbEspessura", "ComboBox"));
        divTamanho = Divider(resolve_(list, "divTamanho", "Divider"));
        lytCores = Layout(resolve_(list, "lytCores", "Layout"));
        lblCores = Label(resolve_(list, "lblCores", "Label"));
        btnCor1 = Circle(resolve_(list, "btnCor1", "Circle"));
        btnCor2 = Circle(resolve_(list, "btnCor2", "Circle"));
        grdPaleta = GridLayout(resolve_(list, "grdPaleta", "GridLayout"));
        btnPaleta01 = Circle(resolve_(list, "btnPaleta01", "Circle"));
        btnPaleta02 = Circle(resolve_(list, "btnPaleta02", "Circle"));
        btnPaleta03 = Circle(resolve_(list, "btnPaleta03", "Circle"));
        btnPaleta04 = Circle(resolve_(list, "btnPaleta04", "Circle"));
        btnPaleta05 = Circle(resolve_(list, "btnPaleta05", "Circle"));
        btnPaleta06 = Circle(resolve_(list, "btnPaleta06", "Circle"));
        btnPaleta07 = Circle(resolve_(list, "btnPaleta07", "Circle"));
        btnPaleta08 = Circle(resolve_(list, "btnPaleta08", "Circle"));
        btnPaleta09 = Circle(resolve_(list, "btnPaleta09", "Circle"));
        btnPaleta10 = Circle(resolve_(list, "btnPaleta10", "Circle"));
        btnPaleta11 = Circle(resolve_(list, "btnPaleta11", "Circle"));
        btnPaleta12 = Circle(resolve_(list, "btnPaleta12", "Circle"));
        btnPaleta13 = Circle(resolve_(list, "btnPaleta13", "Circle"));
        btnPaleta14 = Circle(resolve_(list, "btnPaleta14", "Circle"));
        btnPaleta15 = Circle(resolve_(list, "btnPaleta15", "Circle"));
        btnPaleta16 = Circle(resolve_(list, "btnPaleta16", "Circle"));
        btnPaleta17 = Circle(resolve_(list, "btnPaleta17", "Circle"));
        btnPaleta18 = Circle(resolve_(list, "btnPaleta18", "Circle"));
        btnPaleta19 = Circle(resolve_(list, "btnPaleta19", "Circle"));
        btnPaleta20 = Circle(resolve_(list, "btnPaleta20", "Circle"));
        stbStatus = StatusBar(resolve_(list, "stbStatus", "StatusBar"));
        lblPosicao = Label(resolve_(list, "lblPosicao", "Label"));
        lblTamanho = Label(resolve_(list, "lblTamanho", "Label"));
        lblZoom = Label(resolve_(list, "lblZoom", "Label"));
        sbxArea = ScrollBox(resolve_(list, "sbxArea", "ScrollBox"));
        pcDesenho = PaintCanvas(resolve_(list, "pcDesenho", "PaintCanvas"));
        dlgAbrir = OpenDialog(resolve_(list, "dlgAbrir", "OpenDialog"));
        dlgSalvar = SaveDialog(resolve_(list, "dlgSalvar", "SaveDialog"));
    }
    // O Button deste form com o nome (como o FindComponent; vazio se não há).
    Button find_button(const std::string& name) const {
        if (name == "btnSalvar") return btnSalvar;
        if (name == "btnDesfazer") return btnDesfazer;
        if (name == "btnRefazer") return btnRefazer;
        if (name == "btnSelecionar") return btnSelecionar;
        if (name == "btnLapis") return btnLapis;
        if (name == "btnBalde") return btnBalde;
        if (name == "btnBorracha") return btnBorracha;
        if (name == "btnContaGotas") return btnContaGotas;
        if (name == "btnPincel") return btnPincel;
        if (name == "btnLinha") return btnLinha;
        if (name == "btnRetangulo") return btnRetangulo;
        if (name == "btnElipse") return btnElipse;
        return Button();
    }
    // O Button que disparou o evento.
    Button sender_as_button(const aylis::Event& ev) const { return find_button(ev.sender_name); }
    // O CheckBox deste form com o nome (como o FindComponent; vazio se não há).
    CheckBox find_check_box(const std::string& name) const {
        if (name == "chkPreencher") return chkPreencher;
        return CheckBox();
    }
    // O CheckBox que disparou o evento.
    CheckBox sender_as_check_box(const aylis::Event& ev) const { return find_check_box(ev.sender_name); }
    // O Circle deste form com o nome (como o FindComponent; vazio se não há).
    Circle find_circle(const std::string& name) const {
        if (name == "btnCor1") return btnCor1;
        if (name == "btnCor2") return btnCor2;
        if (name == "btnPaleta01") return btnPaleta01;
        if (name == "btnPaleta02") return btnPaleta02;
        if (name == "btnPaleta03") return btnPaleta03;
        if (name == "btnPaleta04") return btnPaleta04;
        if (name == "btnPaleta05") return btnPaleta05;
        if (name == "btnPaleta06") return btnPaleta06;
        if (name == "btnPaleta07") return btnPaleta07;
        if (name == "btnPaleta08") return btnPaleta08;
        if (name == "btnPaleta09") return btnPaleta09;
        if (name == "btnPaleta10") return btnPaleta10;
        if (name == "btnPaleta11") return btnPaleta11;
        if (name == "btnPaleta12") return btnPaleta12;
        if (name == "btnPaleta13") return btnPaleta13;
        if (name == "btnPaleta14") return btnPaleta14;
        if (name == "btnPaleta15") return btnPaleta15;
        if (name == "btnPaleta16") return btnPaleta16;
        if (name == "btnPaleta17") return btnPaleta17;
        if (name == "btnPaleta18") return btnPaleta18;
        if (name == "btnPaleta19") return btnPaleta19;
        if (name == "btnPaleta20") return btnPaleta20;
        return Circle();
    }
    // O Circle que disparou o evento.
    Circle sender_as_circle(const aylis::Event& ev) const { return find_circle(ev.sender_name); }
    // O ComboBox deste form com o nome (como o FindComponent; vazio se não há).
    ComboBox find_combo_box(const std::string& name) const {
        if (name == "cbEspessura") return cbEspessura;
        return ComboBox();
    }
    // O ComboBox que disparou o evento.
    ComboBox sender_as_combo_box(const aylis::Event& ev) const { return find_combo_box(ev.sender_name); }
    // O Divider deste form com o nome (como o FindComponent; vazio se não há).
    Divider find_divider(const std::string& name) const {
        if (name == "divTopo") return divTopo;
        if (name == "divSelecao") return divSelecao;
        if (name == "divFerramentas") return divFerramentas;
        if (name == "divPinceis") return divPinceis;
        if (name == "divFormas") return divFormas;
        if (name == "divTamanho") return divTamanho;
        return Divider();
    }
    // O Divider que disparou o evento.
    Divider sender_as_divider(const aylis::Event& ev) const { return find_divider(ev.sender_name); }
    // O GridLayout deste form com o nome (como o FindComponent; vazio se não há).
    GridLayout find_grid_layout(const std::string& name) const {
        if (name == "grdFerramentas") return grdFerramentas;
        if (name == "grdFormas") return grdFormas;
        if (name == "grdPaleta") return grdPaleta;
        return GridLayout();
    }
    // O GridLayout que disparou o evento.
    GridLayout sender_as_grid_layout(const aylis::Event& ev) const { return find_grid_layout(ev.sender_name); }
    // O Label deste form com o nome (como o FindComponent; vazio se não há).
    Label find_label(const std::string& name) const {
        if (name == "lblSelecaoGrupo") return lblSelecaoGrupo;
        if (name == "lblFerramentasGrupo") return lblFerramentasGrupo;
        if (name == "lblPinceis") return lblPinceis;
        if (name == "lblFormas") return lblFormas;
        if (name == "lblTamanhoGrupo") return lblTamanhoGrupo;
        if (name == "lblCores") return lblCores;
        if (name == "lblPosicao") return lblPosicao;
        if (name == "lblTamanho") return lblTamanho;
        if (name == "lblZoom") return lblZoom;
        return Label();
    }
    // O Label que disparou o evento.
    Label sender_as_label(const aylis::Event& ev) const { return find_label(ev.sender_name); }
    // O Layout deste form com o nome (como o FindComponent; vazio se não há).
    Layout find_layout(const std::string& name) const {
        if (name == "lytTopo") return lytTopo;
        if (name == "lytSelecao") return lytSelecao;
        if (name == "lytFerramentas") return lytFerramentas;
        if (name == "lytPinceis") return lytPinceis;
        if (name == "lytFormas") return lytFormas;
        if (name == "lytTamanho") return lytTamanho;
        if (name == "lytCores") return lytCores;
        return Layout();
    }
    // O Layout que disparou o evento.
    Layout sender_as_layout(const aylis::Event& ev) const { return find_layout(ev.sender_name); }
    // O MainMenu deste form com o nome (como o FindComponent; vazio se não há).
    MainMenu find_main_menu(const std::string& name) const {
        if (name == "mnuPrincipal") return mnuPrincipal;
        return MainMenu();
    }
    // O MainMenu que disparou o evento.
    MainMenu sender_as_main_menu(const aylis::Event& ev) const { return find_main_menu(ev.sender_name); }
    // O MenuItem deste form com o nome (como o FindComponent; vazio se não há).
    MenuItem find_menu_item(const std::string& name) const {
        if (name == "mnuArquivo") return mnuArquivo;
        if (name == "mnuNovo") return mnuNovo;
        if (name == "mnuAbrir") return mnuAbrir;
        if (name == "mnuSalvar") return mnuSalvar;
        if (name == "mnuSalvarComo") return mnuSalvarComo;
        if (name == "mnuSep1") return mnuSep1;
        if (name == "mnuSair") return mnuSair;
        if (name == "mnuEditar") return mnuEditar;
        if (name == "mnuDesfazer") return mnuDesfazer;
        if (name == "mnuRefazer") return mnuRefazer;
        if (name == "mnuSep2") return mnuSep2;
        if (name == "mnuRecortar") return mnuRecortar;
        if (name == "mnuCopiar") return mnuCopiar;
        if (name == "mnuColar") return mnuColar;
        if (name == "mnuExcluir") return mnuExcluir;
        if (name == "mnuSelecionarTudo") return mnuSelecionarTudo;
        if (name == "mnuSep3") return mnuSep3;
        if (name == "mnuLimpar") return mnuLimpar;
        if (name == "mnuExibir") return mnuExibir;
        if (name == "mnuAmpliar") return mnuAmpliar;
        if (name == "mnuReduzir") return mnuReduzir;
        if (name == "mnuTamanhoReal") return mnuTamanhoReal;
        return MenuItem();
    }
    // O MenuItem que disparou o evento.
    MenuItem sender_as_menu_item(const aylis::Event& ev) const { return find_menu_item(ev.sender_name); }
    // O OpenDialog deste form com o nome (como o FindComponent; vazio se não há).
    OpenDialog find_open_dialog(const std::string& name) const {
        if (name == "dlgAbrir") return dlgAbrir;
        return OpenDialog();
    }
    // O OpenDialog que disparou o evento.
    OpenDialog sender_as_open_dialog(const aylis::Event& ev) const { return find_open_dialog(ev.sender_name); }
    // O PaintCanvas deste form com o nome (como o FindComponent; vazio se não há).
    PaintCanvas find_paint_canvas(const std::string& name) const {
        if (name == "pcDesenho") return pcDesenho;
        return PaintCanvas();
    }
    // O PaintCanvas que disparou o evento.
    PaintCanvas sender_as_paint_canvas(const aylis::Event& ev) const { return find_paint_canvas(ev.sender_name); }
    // O Panel deste form com o nome (como o FindComponent; vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlFaixa") return pnlFaixa;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O SaveDialog deste form com o nome (como o FindComponent; vazio se não há).
    SaveDialog find_save_dialog(const std::string& name) const {
        if (name == "dlgSalvar") return dlgSalvar;
        return SaveDialog();
    }
    // O SaveDialog que disparou o evento.
    SaveDialog sender_as_save_dialog(const aylis::Event& ev) const { return find_save_dialog(ev.sender_name); }
    // O ScrollBox deste form com o nome (como o FindComponent; vazio se não há).
    ScrollBox find_scroll_box(const std::string& name) const {
        if (name == "sbxArea") return sbxArea;
        return ScrollBox();
    }
    // O ScrollBox que disparou o evento.
    ScrollBox sender_as_scroll_box(const aylis::Event& ev) const { return find_scroll_box(ev.sender_name); }
    // O StatusBar deste form com o nome (como o FindComponent; vazio se não há).
    StatusBar find_status_bar(const std::string& name) const {
        if (name == "stbStatus") return stbStatus;
        return StatusBar();
    }
    // O StatusBar que disparou o evento.
    StatusBar sender_as_status_bar(const aylis::Event& ev) const { return find_status_bar(ev.sender_name); }
};

}  // namespace aylis_components

// Cada frmX é a janela do form cujo evento está rodando (a primeira, fora dos handlers): a
// instância é copiada para cá antes do handler, então &frmX.componente vale sempre.
inline aylis_components::FrmPaintComponents frmPaint;

namespace aylis_components {

// Registra as classes dos forms (o registro dos forms chama).
inline void register_classes() {
    aylis::Aylis::register_form_class(
        "frmPaint", [](aylis::Aylis* app, long long id, const std::string& name) -> std::unique_ptr<aylis::FormBase> { return std::make_unique<FrmPaintComponents>(app, id, name); },
        [](aylis::FormBase& form) { ::frmPaint = static_cast<FrmPaintComponents&>(form); });
}

}  // namespace aylis_components
