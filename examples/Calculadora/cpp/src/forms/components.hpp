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

// Um TabControl de um form.
struct TabControl : aylis::Component {
    TabControl() = default;
    explicit TabControl(const aylis::Component& c) : aylis::Component(c) {}
    long long tab_index() const { return static_cast<long long>(get_prop_("tab_index").number()); }
    void set_tab_index(long long v) const { set_prop_("tab_index", v); }
    bool show_tabs() const { return get_prop_("show_tabs").boolean(); }
    void set_show_tabs(bool v) const { set_prop_("show_tabs", v); }
    std::string transition() const { return get_prop_("transition").str(); }
    void set_transition(const std::string& v) const { set_prop_("transition", v); }
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

// Um TabItem de um form.
struct TabItem : aylis::Component {
    TabItem() = default;
    explicit TabItem(const aylis::Component& c) : aylis::Component(c) {}
    std::string caption() const { return get_prop_("caption").str(); }
    void set_caption(const std::string& v) const { set_prop_("caption", v); }
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
    // O Button deste form com o nome (como o FindComponent; vazio se não há).
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
    // O GridLayout deste form com o nome (como o FindComponent; vazio se não há).
    GridLayout find_grid_layout(const std::string& name) const {
        if (name == "grdMemoria") return grdMemoria;
        if (name == "grdTeclado") return grdTeclado;
        return GridLayout();
    }
    // O GridLayout que disparou o evento.
    GridLayout sender_as_grid_layout(const aylis::Event& ev) const { return find_grid_layout(ev.sender_name); }
    // O Label deste form com o nome (como o FindComponent; vazio se não há).
    Label find_label(const std::string& name) const {
        if (name == "lblTitulo") return lblTitulo;
        if (name == "lblExpressao") return lblExpressao;
        if (name == "lblVisor") return lblVisor;
        return Label();
    }
    // O Label que disparou o evento.
    Label sender_as_label(const aylis::Event& ev) const { return find_label(ev.sender_name); }
    // O Layout deste form com o nome (como o FindComponent; vazio se não há).
    Layout find_layout(const std::string& name) const {
        if (name == "lytCorpo") return lytCorpo;
        if (name == "lytTopo") return lytTopo;
        return Layout();
    }
    // O Layout que disparou o evento.
    Layout sender_as_layout(const aylis::Event& ev) const { return find_layout(ev.sender_name); }
    // O ListView deste form com o nome (como o FindComponent; vazio se não há).
    ListView find_list_view(const std::string& name) const {
        if (name == "lvHistorico") return lvHistorico;
        if (name == "lvMemoria") return lvMemoria;
        return ListView();
    }
    // O ListView que disparou o evento.
    ListView sender_as_list_view(const aylis::Event& ev) const { return find_list_view(ev.sender_name); }
    // O Panel deste form com o nome (como o FindComponent; vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlLateral") return pnlLateral;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O TabControl deste form com o nome (como o FindComponent; vazio se não há).
    TabControl find_tab_control(const std::string& name) const {
        if (name == "tabLateral") return tabLateral;
        return TabControl();
    }
    // O TabControl que disparou o evento.
    TabControl sender_as_tab_control(const aylis::Event& ev) const { return find_tab_control(ev.sender_name); }
    // O TabItem deste form com o nome (como o FindComponent; vazio se não há).
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
