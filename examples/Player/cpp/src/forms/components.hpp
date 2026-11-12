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
    bool default_button() const { return get_prop_("default_button").boolean(); }
    void set_default_button(bool v) const { set_prop_("default_button", v); }
    bool cancel_button() const { return get_prop_("cancel_button").boolean(); }
    void set_cancel_button(bool v) const { set_prop_("cancel_button", v); }
    std::string h_align() const { return get_prop_("h_align").str(); }
    void set_h_align(const std::string& v) const { set_prop_("h_align", v); }
    long long group_index() const { return static_cast<long long>(get_prop_("group_index").number()); }
    void set_group_index(long long v) const { set_prop_("group_index", v); }
    std::string radius() const { return get_prop_("radius").str(); }
    void set_radius(const std::string& v) const { set_prop_("radius", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
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
    std::string gpu() const { return get_prop_("gpu").str(); }
    void set_gpu(const std::string& v) const { set_prop_("gpu", v); }
    bool activate_on_show() const { return get_prop_("activate_on_show").boolean(); }
    void set_activate_on_show(bool v) const { set_prop_("activate_on_show", v); }
    std::string close_action() const { return get_prop_("close_action").str(); }
    void set_close_action(const std::string& v) const { set_prop_("close_action", v); }
    double frame_rate() const { return get_prop_("frame_rate").number(); }
    void set_frame_rate(double v) const { set_prop_("frame_rate", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void move_to_monitor(long long monitor) const { tell_("invoke", {{"target", aylis::Json(id_)}, {"method", "move_to_monitor"}, {"args", aylis::Json::array({monitor})}}); }
    std::string gpu_adapter() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "gpu_adapter"}, {"args", aylis::Json::array({})}}).str(); }
    std::string gpu_adapters() const { return ask_("invoke", {{"target", aylis::Json(id_)}, {"method", "gpu_adapters"}, {"args", aylis::Json::array({})}}).str(); }
    void show() const { tell_("show", {{"target", aylis::Json(id_)}}); }
    bool close() const { return ask_("close", {{"target", aylis::Json(id_)}}).boolean(); }
    std::string focused() const { return ask_("focused", {{"target", aylis::Json(id_)}}).str(); }
    void focus_next() const { tell_("focus_next", {{"target", aylis::Json(id_)}}); }
    void focus_prev() const { tell_("focus_prev", {{"target", aylis::Json(id_)}}); }
};

// Um Icon de um form.
struct Icon : aylis::Component {
    Icon() = default;
    explicit Icon(const aylis::Component& c) : aylis::Component(c) {}
    std::string icon() const { return get_prop_("icon").str(); }
    void set_icon(const std::string& v) const { set_prop_("icon", v); }
    std::string color() const { return get_prop_("color").str(); }
    void set_color(const std::string& v) const { set_prop_("color", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
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
    double line_height() const { return get_prop_("line_height").number(); }
    void set_line_height(double v) const { set_prop_("line_height", v); }
    std::string text_shadow() const { return get_prop_("text_shadow").str(); }
    void set_text_shadow(const std::string& v) const { set_prop_("text_shadow", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
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
    std::string align() const { return get_prop_("align").str(); }
    void set_align(const std::string& v) const { set_prop_("align", v); }
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um ListView de um form.
struct ListView : aylis::Component {
    ListView() = default;
    explicit ListView(const aylis::Component& c) : aylis::Component(c) {}
    std::string items() const { return get_prop_("items").str(); }
    void set_items(const std::string& v) const { set_prop_("items", v); }
    long long item_index() const { return static_cast<long long>(get_prop_("item_index").number()); }
    void set_item_index(long long v) const { set_prop_("item_index", v); }
    double item_height() const { return get_prop_("item_height").number(); }
    void set_item_height(double v) const { set_prop_("item_height", v); }
    bool accessory() const { return get_prop_("accessory").boolean(); }
    void set_accessory(bool v) const { set_prop_("accessory", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
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
    bool header() const { return get_prop_("header").boolean(); }
    void set_header(bool v) const { set_prop_("header", v); }
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
    std::string radius() const { return get_prop_("radius").str(); }
    void set_radius(const std::string& v) const { set_prop_("radius", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
};

// Um SeekBar de um form.
struct SeekBar : aylis::Component {
    SeekBar() = default;
    explicit SeekBar(const aylis::Component& c) : aylis::Component(c) {}
    double position() const { return get_prop_("position").number(); }
    void set_position(double v) const { set_prop_("position", v); }
    double duration() const { return get_prop_("duration").number(); }
    void set_duration(double v) const { set_prop_("duration", v); }
    double buffered() const { return get_prop_("buffered").number(); }
    void set_buffered(double v) const { set_prop_("buffered", v); }
    std::string markers() const { return get_prop_("markers").str(); }
    void set_markers(const std::string& v) const { set_prop_("markers", v); }
    bool show_time() const { return get_prop_("show_time").boolean(); }
    void set_show_time(bool v) const { set_prop_("show_time", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um Timer de um form.
struct Timer : aylis::Component {
    Timer() = default;
    explicit Timer(const aylis::Component& c) : aylis::Component(c) {}
    double interval() const { return get_prop_("interval").number(); }
    void set_interval(double v) const { set_prop_("interval", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
    std::string precision() const { return get_prop_("precision").str(); }
    void set_precision(const std::string& v) const { set_prop_("precision", v); }
    bool background() const { return get_prop_("background").boolean(); }
    void set_background(bool v) const { set_prop_("background", v); }
    bool one_shot() const { return get_prop_("one_shot").boolean(); }
    void set_one_shot(bool v) const { set_prop_("one_shot", v); }
    double drift() const { return get_prop_("drift").number(); }
    void set_drift(double v) const { set_prop_("drift", v); }
    void restart() const { tell_("invoke", {{"target", aylis::Json(id_)}, {"method", "restart"}, {"args", aylis::Json::array({})}}); }
};

// Um TrackBar de um form.
struct TrackBar : aylis::Component {
    TrackBar() = default;
    explicit TrackBar(const aylis::Component& c) : aylis::Component(c) {}
    double value() const { return get_prop_("value").number(); }
    void set_value(double v) const { set_prop_("value", v); }
    double min() const { return get_prop_("min").number(); }
    void set_min(double v) const { set_prop_("min", v); }
    double max() const { return get_prop_("max").number(); }
    void set_max(double v) const { set_prop_("max", v); }
    double frequency() const { return get_prop_("frequency").number(); }
    void set_frequency(double v) const { set_prop_("frequency", v); }
    bool vertical() const { return get_prop_("vertical").boolean(); }
    void set_vertical(bool v) const { set_prop_("vertical", v); }
    double thumb_size() const { return get_prop_("thumb_size").number(); }
    void set_thumb_size(double v) const { set_prop_("thumb_size", v); }
    double track_size() const { return get_prop_("track_size").number(); }
    void set_track_size(double v) const { set_prop_("track_size", v); }
    bool solid_thumb() const { return get_prop_("solid_thumb").boolean(); }
    void set_solid_thumb(bool v) const { set_prop_("solid_thumb", v); }
    double limit() const { return get_prop_("limit").number(); }
    void set_limit(double v) const { set_prop_("limit", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void set_focus() const { tell_("focus", {{"target", aylis::Json(id_)}}); }
};

// Um VideoPlayer de um form.
struct VideoPlayer : aylis::Component {
    VideoPlayer() = default;
    explicit VideoPlayer(const aylis::Component& c) : aylis::Component(c) {}
    std::string file() const { return get_prop_("file").str(); }
    void set_file(const std::string& v) const { set_prop_("file", v); }
    bool autoplay() const { return get_prop_("autoplay").boolean(); }
    void set_autoplay(bool v) const { set_prop_("autoplay", v); }
    bool looping() const { return get_prop_("looping").boolean(); }
    void set_looping(bool v) const { set_prop_("looping", v); }
    double volume() const { return get_prop_("volume").number(); }
    void set_volume(double v) const { set_prop_("volume", v); }
    bool muted() const { return get_prop_("muted").boolean(); }
    void set_muted(bool v) const { set_prop_("muted", v); }
    std::string fit() const { return get_prop_("fit").str(); }
    void set_fit(const std::string& v) const { set_prop_("fit", v); }
    double radius() const { return get_prop_("radius").number(); }
    void set_radius(double v) const { set_prop_("radius", v); }
    double rate() const { return get_prop_("rate").number(); }
    void set_rate(double v) const { set_prop_("rate", v); }
    std::string output() const { return get_prop_("output").str(); }
    void set_output(const std::string& v) const { set_prop_("output", v); }
    bool show_video() const { return get_prop_("show_video").boolean(); }
    void set_show_video(bool v) const { set_prop_("show_video", v); }
    long long frames_ahead() const { return static_cast<long long>(get_prop_("frames_ahead").number()); }
    void set_frames_ahead(long long v) const { set_prop_("frames_ahead", v); }
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
    std::string anchors() const { return get_prop_("anchors").str(); }
    void set_anchors(const std::string& v) const { set_prop_("anchors", v); }
    std::string margins() const { return get_prop_("margins").str(); }
    void set_margins(const std::string& v) const { set_prop_("margins", v); }
    std::string padding() const { return get_prop_("padding").str(); }
    void set_padding(const std::string& v) const { set_prop_("padding", v); }
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
    long long grid_row() const { return static_cast<long long>(get_prop_("grid_row").number()); }
    void set_grid_row(long long v) const { set_prop_("grid_row", v); }
    long long grid_column() const { return static_cast<long long>(get_prop_("grid_column").number()); }
    void set_grid_column(long long v) const { set_prop_("grid_column", v); }
    long long row_span() const { return static_cast<long long>(get_prop_("row_span").number()); }
    void set_row_span(long long v) const { set_prop_("row_span", v); }
    double grow() const { return get_prop_("grow").number(); }
    void set_grow(double v) const { set_prop_("grow", v); }
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
    std::string tag_string() const { return get_prop_("tag_string").str(); }
    void set_tag_string(const std::string& v) const { set_prop_("tag_string", v); }
    std::string accessible_name() const { return get_prop_("accessible_name").str(); }
    void set_accessible_name(const std::string& v) const { set_prop_("accessible_name", v); }
    std::string popup_menu() const { return get_prop_("popup_menu").str(); }
    void set_popup_menu(const std::string& v) const { set_prop_("popup_menu", v); }
    void play() const { tell_("media", {{"target", aylis::Json(id_)}, {"op", "play"}}); }
    void pause() const { tell_("media", {{"target", aylis::Json(id_)}, {"op", "pause"}}); }
    void stop() const { tell_("media", {{"target", aylis::Json(id_)}, {"op", "stop"}}); }
    void seek(double seconds) const { tell_("media", {{"target", aylis::Json(id_)}, {"op", "seek"}, {"seconds", seconds}}); }
    double position() const { return ask_("media", {{"target", aylis::Json(id_)}, {"op", "position"}}).number(); }
    double duration() const { return ask_("media", {{"target", aylis::Json(id_)}, {"op", "duration"}}).number(); }
};

// O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
struct FrmPlayerComponents : Form {
    MainMenu mnuPrincipal;
    MenuItem mnuArquivo;
    MenuItem mnuAbrir;
    MenuItem mnuLimparFila;
    MenuItem mnuSep1;
    MenuItem mnuSair;
    MenuItem mnuReproduzir;
    MenuItem mnuReproduzirPausar;
    MenuItem mnuParar;
    MenuItem mnuAnterior;
    MenuItem mnuProxima;
    MenuItem mnuSep2;
    MenuItem mnuEmbaralhar;
    MenuItem mnuRepetir;
    MenuItem mnuExibir;
    MenuItem mnuAgora;
    MenuItem mnuFila;
    Panel pnlControles;
    Layout lytBarra;
    Label lblTempo;
    Label lblDuracao;
    SeekBar sbPosicao;
    Layout lytInfo;
    Icon icoCapa;
    Layout lytTextos;
    Label lblTitulo;
    Label lblDetalhe;
    Layout lytVolume;
    Button btnMudo;
    TrackBar tbVolume;
    Layout lytBotoes;
    Layout lytBotoesCentro;
    Button btnEmbaralhar;
    Button btnAnterior;
    Button btnReproduzir;
    Button btnProxima;
    Button btnRepetir;
    Layout lytNavegacao;
    Button btnAgora;
    Button btnFila;
    Divider divNavegacao;
    Button btnAbrirArquivos;
    Panel pnlAgora;
    VideoPlayer vpVideo;
    Panel pnlFila;
    Label lblFila;
    Layout lytFilaBotoes;
    Button btnAdicionar;
    Button btnLimparFila;
    ListView lvFila;
    OpenDialog dlgAbrir;
    Timer tmrPosicao;

    FrmPlayerComponents() = default;
    using Form::Form;

    // Monta os componentes da janela (o form que não bate com o código é erro).
    void bind(const aylis::Json& list) override {
        mnuPrincipal = MainMenu(resolve_(list, "mnuPrincipal", "MainMenu"));
        mnuArquivo = MenuItem(resolve_(list, "mnuArquivo", "MenuItem"));
        mnuAbrir = MenuItem(resolve_(list, "mnuAbrir", "MenuItem"));
        mnuLimparFila = MenuItem(resolve_(list, "mnuLimparFila", "MenuItem"));
        mnuSep1 = MenuItem(resolve_(list, "mnuSep1", "MenuItem"));
        mnuSair = MenuItem(resolve_(list, "mnuSair", "MenuItem"));
        mnuReproduzir = MenuItem(resolve_(list, "mnuReproduzir", "MenuItem"));
        mnuReproduzirPausar = MenuItem(resolve_(list, "mnuReproduzirPausar", "MenuItem"));
        mnuParar = MenuItem(resolve_(list, "mnuParar", "MenuItem"));
        mnuAnterior = MenuItem(resolve_(list, "mnuAnterior", "MenuItem"));
        mnuProxima = MenuItem(resolve_(list, "mnuProxima", "MenuItem"));
        mnuSep2 = MenuItem(resolve_(list, "mnuSep2", "MenuItem"));
        mnuEmbaralhar = MenuItem(resolve_(list, "mnuEmbaralhar", "MenuItem"));
        mnuRepetir = MenuItem(resolve_(list, "mnuRepetir", "MenuItem"));
        mnuExibir = MenuItem(resolve_(list, "mnuExibir", "MenuItem"));
        mnuAgora = MenuItem(resolve_(list, "mnuAgora", "MenuItem"));
        mnuFila = MenuItem(resolve_(list, "mnuFila", "MenuItem"));
        pnlControles = Panel(resolve_(list, "pnlControles", "Panel"));
        lytBarra = Layout(resolve_(list, "lytBarra", "Layout"));
        lblTempo = Label(resolve_(list, "lblTempo", "Label"));
        lblDuracao = Label(resolve_(list, "lblDuracao", "Label"));
        sbPosicao = SeekBar(resolve_(list, "sbPosicao", "SeekBar"));
        lytInfo = Layout(resolve_(list, "lytInfo", "Layout"));
        icoCapa = Icon(resolve_(list, "icoCapa", "Icon"));
        lytTextos = Layout(resolve_(list, "lytTextos", "Layout"));
        lblTitulo = Label(resolve_(list, "lblTitulo", "Label"));
        lblDetalhe = Label(resolve_(list, "lblDetalhe", "Label"));
        lytVolume = Layout(resolve_(list, "lytVolume", "Layout"));
        btnMudo = Button(resolve_(list, "btnMudo", "Button"));
        tbVolume = TrackBar(resolve_(list, "tbVolume", "TrackBar"));
        lytBotoes = Layout(resolve_(list, "lytBotoes", "Layout"));
        lytBotoesCentro = Layout(resolve_(list, "lytBotoesCentro", "Layout"));
        btnEmbaralhar = Button(resolve_(list, "btnEmbaralhar", "Button"));
        btnAnterior = Button(resolve_(list, "btnAnterior", "Button"));
        btnReproduzir = Button(resolve_(list, "btnReproduzir", "Button"));
        btnProxima = Button(resolve_(list, "btnProxima", "Button"));
        btnRepetir = Button(resolve_(list, "btnRepetir", "Button"));
        lytNavegacao = Layout(resolve_(list, "lytNavegacao", "Layout"));
        btnAgora = Button(resolve_(list, "btnAgora", "Button"));
        btnFila = Button(resolve_(list, "btnFila", "Button"));
        divNavegacao = Divider(resolve_(list, "divNavegacao", "Divider"));
        btnAbrirArquivos = Button(resolve_(list, "btnAbrirArquivos", "Button"));
        pnlAgora = Panel(resolve_(list, "pnlAgora", "Panel"));
        vpVideo = VideoPlayer(resolve_(list, "vpVideo", "VideoPlayer"));
        pnlFila = Panel(resolve_(list, "pnlFila", "Panel"));
        lblFila = Label(resolve_(list, "lblFila", "Label"));
        lytFilaBotoes = Layout(resolve_(list, "lytFilaBotoes", "Layout"));
        btnAdicionar = Button(resolve_(list, "btnAdicionar", "Button"));
        btnLimparFila = Button(resolve_(list, "btnLimparFila", "Button"));
        lvFila = ListView(resolve_(list, "lvFila", "ListView"));
        dlgAbrir = OpenDialog(resolve_(list, "dlgAbrir", "OpenDialog"));
        tmrPosicao = Timer(resolve_(list, "tmrPosicao", "Timer"));
    }
    // O Button deste form com o nome (como o FindComponent; vazio se não há).
    Button find_button(const std::string& name) const {
        if (name == "btnMudo") return btnMudo;
        if (name == "btnEmbaralhar") return btnEmbaralhar;
        if (name == "btnAnterior") return btnAnterior;
        if (name == "btnReproduzir") return btnReproduzir;
        if (name == "btnProxima") return btnProxima;
        if (name == "btnRepetir") return btnRepetir;
        if (name == "btnAgora") return btnAgora;
        if (name == "btnFila") return btnFila;
        if (name == "btnAbrirArquivos") return btnAbrirArquivos;
        if (name == "btnAdicionar") return btnAdicionar;
        if (name == "btnLimparFila") return btnLimparFila;
        return Button();
    }
    // O Button que disparou o evento.
    Button sender_as_button(const aylis::Event& ev) const { return find_button(ev.sender_name); }
    // O Divider deste form com o nome (como o FindComponent; vazio se não há).
    Divider find_divider(const std::string& name) const {
        if (name == "divNavegacao") return divNavegacao;
        return Divider();
    }
    // O Divider que disparou o evento.
    Divider sender_as_divider(const aylis::Event& ev) const { return find_divider(ev.sender_name); }
    // O Icon deste form com o nome (como o FindComponent; vazio se não há).
    Icon find_icon(const std::string& name) const {
        if (name == "icoCapa") return icoCapa;
        return Icon();
    }
    // O Icon que disparou o evento.
    Icon sender_as_icon(const aylis::Event& ev) const { return find_icon(ev.sender_name); }
    // O Label deste form com o nome (como o FindComponent; vazio se não há).
    Label find_label(const std::string& name) const {
        if (name == "lblTempo") return lblTempo;
        if (name == "lblDuracao") return lblDuracao;
        if (name == "lblTitulo") return lblTitulo;
        if (name == "lblDetalhe") return lblDetalhe;
        if (name == "lblFila") return lblFila;
        return Label();
    }
    // O Label que disparou o evento.
    Label sender_as_label(const aylis::Event& ev) const { return find_label(ev.sender_name); }
    // O Layout deste form com o nome (como o FindComponent; vazio se não há).
    Layout find_layout(const std::string& name) const {
        if (name == "lytBarra") return lytBarra;
        if (name == "lytInfo") return lytInfo;
        if (name == "lytTextos") return lytTextos;
        if (name == "lytVolume") return lytVolume;
        if (name == "lytBotoes") return lytBotoes;
        if (name == "lytBotoesCentro") return lytBotoesCentro;
        if (name == "lytNavegacao") return lytNavegacao;
        if (name == "lytFilaBotoes") return lytFilaBotoes;
        return Layout();
    }
    // O Layout que disparou o evento.
    Layout sender_as_layout(const aylis::Event& ev) const { return find_layout(ev.sender_name); }
    // O ListView deste form com o nome (como o FindComponent; vazio se não há).
    ListView find_list_view(const std::string& name) const {
        if (name == "lvFila") return lvFila;
        return ListView();
    }
    // O ListView que disparou o evento.
    ListView sender_as_list_view(const aylis::Event& ev) const { return find_list_view(ev.sender_name); }
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
        if (name == "mnuAbrir") return mnuAbrir;
        if (name == "mnuLimparFila") return mnuLimparFila;
        if (name == "mnuSep1") return mnuSep1;
        if (name == "mnuSair") return mnuSair;
        if (name == "mnuReproduzir") return mnuReproduzir;
        if (name == "mnuReproduzirPausar") return mnuReproduzirPausar;
        if (name == "mnuParar") return mnuParar;
        if (name == "mnuAnterior") return mnuAnterior;
        if (name == "mnuProxima") return mnuProxima;
        if (name == "mnuSep2") return mnuSep2;
        if (name == "mnuEmbaralhar") return mnuEmbaralhar;
        if (name == "mnuRepetir") return mnuRepetir;
        if (name == "mnuExibir") return mnuExibir;
        if (name == "mnuAgora") return mnuAgora;
        if (name == "mnuFila") return mnuFila;
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
    // O Panel deste form com o nome (como o FindComponent; vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlControles") return pnlControles;
        if (name == "pnlAgora") return pnlAgora;
        if (name == "pnlFila") return pnlFila;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O SeekBar deste form com o nome (como o FindComponent; vazio se não há).
    SeekBar find_seek_bar(const std::string& name) const {
        if (name == "sbPosicao") return sbPosicao;
        return SeekBar();
    }
    // O SeekBar que disparou o evento.
    SeekBar sender_as_seek_bar(const aylis::Event& ev) const { return find_seek_bar(ev.sender_name); }
    // O Timer deste form com o nome (como o FindComponent; vazio se não há).
    Timer find_timer(const std::string& name) const {
        if (name == "tmrPosicao") return tmrPosicao;
        return Timer();
    }
    // O Timer que disparou o evento.
    Timer sender_as_timer(const aylis::Event& ev) const { return find_timer(ev.sender_name); }
    // O TrackBar deste form com o nome (como o FindComponent; vazio se não há).
    TrackBar find_track_bar(const std::string& name) const {
        if (name == "tbVolume") return tbVolume;
        return TrackBar();
    }
    // O TrackBar que disparou o evento.
    TrackBar sender_as_track_bar(const aylis::Event& ev) const { return find_track_bar(ev.sender_name); }
    // O VideoPlayer deste form com o nome (como o FindComponent; vazio se não há).
    VideoPlayer find_video_player(const std::string& name) const {
        if (name == "vpVideo") return vpVideo;
        return VideoPlayer();
    }
    // O VideoPlayer que disparou o evento.
    VideoPlayer sender_as_video_player(const aylis::Event& ev) const { return find_video_player(ev.sender_name); }
};

}  // namespace aylis_components

// Cada frmX é a janela do form cujo evento está rodando (a primeira, fora dos handlers): a
// instância é copiada para cá antes do handler, então &frmX.componente vale sempre.
inline aylis_components::FrmPlayerComponents frmPlayer;

namespace aylis_components {

// Registra as classes dos forms (o registro dos forms chama).
inline void register_classes() {
    aylis::Aylis::register_form_class(
        "frmPlayer", [](aylis::Aylis* app, long long id, const std::string& name) -> std::unique_ptr<aylis::FormBase> { return std::make_unique<FrmPlayerComponents>(app, id, name); },
        [](aylis::FormBase& form) { ::frmPlayer = static_cast<FrmPlayerComponents&>(form); });
}

}  // namespace aylis_components
