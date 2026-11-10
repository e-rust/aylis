// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include <memory>
#include <optional>
#include <string>

#include "../aylis.hpp"

namespace aylis_components {

// Um ClientDataSet de um form.
struct ClientDataSet : aylis::Component {
    ClientDataSet() = default;
    explicit ClientDataSet(const aylis::Component& c) : aylis::Component(c) {}
    std::string key_fields() const { return get_prop_("key_fields").str(); }
    void set_key_fields(const std::string& v) const { set_prop_("key_fields", v); }
    std::string apply_mode() const { return get_prop_("apply_mode").str(); }
    void set_apply_mode(const std::string& v) const { set_prop_("apply_mode", v); }
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
    std::string indexes() const { return get_prop_("indexes").str(); }
    void set_indexes(const std::string& v) const { set_prop_("indexes", v); }
    std::string index_name() const { return get_prop_("index_name").str(); }
    void set_index_name(const std::string& v) const { set_prop_("index_name", v); }
    std::string params() const { return get_prop_("params").str(); }
    void set_params(const std::string& v) const { set_prop_("params", v); }
    std::string status_filter() const { return get_prop_("status_filter").str(); }
    void set_status_filter(const std::string& v) const { set_prop_("status_filter", v); }
    std::string master_source() const { return get_prop_("master_source").str(); }
    void set_master_source(const std::string& v) const { set_prop_("master_source", v); }
    std::string master_fields() const { return get_prop_("master_fields").str(); }
    void set_master_fields(const std::string& v) const { set_prop_("master_fields", v); }
    bool apply_separately() const { return get_prop_("apply_separately").boolean(); }
    void set_apply_separately(bool v) const { set_prop_("apply_separately", v); }
    long long record_total() const { return static_cast<long long>(get_prop_("record_total").number()); }
    void set_record_total(long long v) const { set_prop_("record_total", v); }
    long long max_records() const { return static_cast<long long>(get_prop_("max_records").number()); }
    void set_max_records(long long v) const { set_prop_("max_records", v); }
    std::string partial_notice() const { return get_prop_("partial_notice").str(); }
};

// Um DBChart de um form.
struct DBChart : aylis::Component {
    DBChart() = default;
    explicit DBChart(const aylis::Component& c) : aylis::Component(c) {}
    std::string data_source() const { return get_prop_("data_source").str(); }
    void set_data_source(const std::string& v) const { set_prop_("data_source", v); }
    std::string label_field() const { return get_prop_("label_field").str(); }
    void set_label_field(const std::string& v) const { set_prop_("label_field", v); }
    std::string value_fields() const { return get_prop_("value_fields").str(); }
    void set_value_fields(const std::string& v) const { set_prop_("value_fields", v); }
    std::string kind() const { return get_prop_("kind").str(); }
    void set_kind(const std::string& v) const { set_prop_("kind", v); }
    std::string title() const { return get_prop_("title").str(); }
    void set_title(const std::string& v) const { set_prop_("title", v); }
    std::string labels() const { return get_prop_("labels").str(); }
    void set_labels(const std::string& v) const { set_prop_("labels", v); }
    std::string series() const { return get_prop_("series").str(); }
    void set_series(const std::string& v) const { set_prop_("series", v); }
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

// Um DataGrid de um form.
struct DataGrid : aylis::Component {
    DataGrid() = default;
    explicit DataGrid(const aylis::Component& c) : aylis::Component(c) {}
    std::string data_source() const { return get_prop_("data_source").str(); }
    void set_data_source(const std::string& v) const { set_prop_("data_source", v); }
    std::string columns() const { return get_prop_("columns").str(); }
    void set_columns(const std::string& v) const { set_prop_("columns", v); }
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
    std::string footer() const { return get_prop_("footer").str(); }
    void set_footer(const std::string& v) const { set_prop_("footer", v); }
    bool incremental_search() const { return get_prop_("incremental_search").boolean(); }
    void set_incremental_search(bool v) const { set_prop_("incremental_search", v); }
    std::string cursor_color() const { return get_prop_("cursor_color").str(); }
    void set_cursor_color(const std::string& v) const { set_prop_("cursor_color", v); }
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

// Um DataSource de um form.
struct DataSource : aylis::Component {
    DataSource() = default;
    explicit DataSource(const aylis::Component& c) : aylis::Component(c) {}
    std::string data_set() const { return get_prop_("data_set").str(); }
    void set_data_set(const std::string& v) const { set_prop_("data_set", v); }
    bool auto_edit() const { return get_prop_("auto_edit").boolean(); }
    void set_auto_edit(bool v) const { set_prop_("auto_edit", v); }
    bool enabled() const { return get_prop_("enabled").boolean(); }
    void set_enabled(bool v) const { set_prop_("enabled", v); }
};

// Um Field de um form.
struct Field : aylis::Component {
    Field() = default;
    explicit Field(const aylis::Component& c) : aylis::Component(c) {}
    std::string field_name() const { return get_prop_("field_name").str(); }
    void set_field_name(const std::string& v) const { set_prop_("field_name", v); }
    std::string type() const { return get_prop_("type").str(); }
    void set_type(const std::string& v) const { set_prop_("type", v); }
    long long size() const { return static_cast<long long>(get_prop_("size").number()); }
    void set_size(long long v) const { set_prop_("size", v); }
    long long scale() const { return static_cast<long long>(get_prop_("scale").number()); }
    void set_scale(long long v) const { set_prop_("scale", v); }
    std::string rounding() const { return get_prop_("rounding").str(); }
    void set_rounding(const std::string& v) const { set_prop_("rounding", v); }
    std::string display_label() const { return get_prop_("display_label").str(); }
    void set_display_label(const std::string& v) const { set_prop_("display_label", v); }
    double display_width() const { return get_prop_("display_width").number(); }
    void set_display_width(double v) const { set_prop_("display_width", v); }
    std::string display_format() const { return get_prop_("display_format").str(); }
    void set_display_format(const std::string& v) const { set_prop_("display_format", v); }
    std::string edit_format() const { return get_prop_("edit_format").str(); }
    void set_edit_format(const std::string& v) const { set_prop_("edit_format", v); }
    std::string alignment() const { return get_prop_("alignment").str(); }
    void set_alignment(const std::string& v) const { set_prop_("alignment", v); }
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
    std::string char_case() const { return get_prop_("char_case").str(); }
    void set_char_case(const std::string& v) const { set_prop_("char_case", v); }
    std::string edit_mask() const { return get_prop_("edit_mask").str(); }
    void set_edit_mask(const std::string& v) const { set_prop_("edit_mask", v); }
    bool sensitive() const { return get_prop_("sensitive").boolean(); }
    void set_sensitive(bool v) const { set_prop_("sensitive", v); }
    std::string kind() const { return get_prop_("kind").str(); }
    void set_kind(const std::string& v) const { set_prop_("kind", v); }
    std::string expression() const { return get_prop_("expression").str(); }
    void set_expression(const std::string& v) const { set_prop_("expression", v); }
    std::string lookup_data_set() const { return get_prop_("lookup_data_set").str(); }
    void set_lookup_data_set(const std::string& v) const { set_prop_("lookup_data_set", v); }
    std::string key_fields() const { return get_prop_("key_fields").str(); }
    void set_key_fields(const std::string& v) const { set_prop_("key_fields", v); }
    std::string lookup_key_fields() const { return get_prop_("lookup_key_fields").str(); }
    void set_lookup_key_fields(const std::string& v) const { set_prop_("lookup_key_fields", v); }
    std::string lookup_result_field() const { return get_prop_("lookup_result_field").str(); }
    void set_lookup_result_field(const std::string& v) const { set_prop_("lookup_result_field", v); }
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

// Um Image de um form.
struct Image : aylis::Component {
    Image() = default;
    explicit Image(const aylis::Component& c) : aylis::Component(c) {}
    std::string image() const { return get_prop_("image").str(); }
    void set_image(const std::string& v) const { set_prop_("image", v); }
    std::string fit() const { return get_prop_("fit").str(); }
    void set_fit(const std::string& v) const { set_prop_("fit", v); }
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

// Um Svg de um form.
struct Svg : aylis::Component {
    Svg() = default;
    explicit Svg(const aylis::Component& c) : aylis::Component(c) {}
    std::string source() const { return get_prop_("source").str(); }
    void set_source(const std::string& v) const { set_prop_("source", v); }
    std::string color() const { return get_prop_("color").str(); }
    void set_color(const std::string& v) const { set_prop_("color", v); }
    std::string fit() const { return get_prop_("fit").str(); }
    void set_fit(const std::string& v) const { set_prop_("fit", v); }
    std::string highlight() const { return get_prop_("highlight").str(); }
    void set_highlight(const std::string& v) const { set_prop_("highlight", v); }
    std::string highlight_color() const { return get_prop_("highlight_color").str(); }
    void set_highlight_color(const std::string& v) const { set_prop_("highlight_color", v); }
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
    // O ClientDataSet deste form com o nome (como o FindComponent; vazio se não há).
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
    // O DBChart deste form com o nome (como o FindComponent; vazio se não há).
    DBChart find_d_b_chart(const std::string& name) const {
        if (name == "dbcPopulacao") return dbcPopulacao;
        if (name == "dbcMaiores") return dbcMaiores;
        if (name == "dbcEstados") return dbcEstados;
        return DBChart();
    }
    // O DBChart que disparou o evento.
    DBChart sender_as_d_b_chart(const aylis::Event& ev) const { return find_d_b_chart(ev.sender_name); }
    // O DataGrid deste form com o nome (como o FindComponent; vazio se não há).
    DataGrid find_data_grid(const std::string& name) const {
        if (name == "grdEstados") return grdEstados;
        if (name == "grdDistritos") return grdDistritos;
        if (name == "grdMunicipios") return grdMunicipios;
        return DataGrid();
    }
    // O DataGrid que disparou o evento.
    DataGrid sender_as_data_grid(const aylis::Event& ev) const { return find_data_grid(ev.sender_name); }
    // O DataSource deste form com o nome (como o FindComponent; vazio se não há).
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
    // O Field deste form com o nome (como o FindComponent; vazio se não há).
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
    // O GroupBox deste form com o nome (como o FindComponent; vazio se não há).
    GroupBox find_group_box(const std::string& name) const {
        if (name == "gbEstados") return gbEstados;
        if (name == "gbDistritos") return gbDistritos;
        if (name == "gbMunicipios") return gbMunicipios;
        return GroupBox();
    }
    // O GroupBox que disparou o evento.
    GroupBox sender_as_group_box(const aylis::Event& ev) const { return find_group_box(ev.sender_name); }
    // O Image deste form com o nome (como o FindComponent; vazio se não há).
    Image find_image(const std::string& name) const {
        if (name == "imgBandeiraEstado") return imgBandeiraEstado;
        if (name == "imgBandeiraMunicipio") return imgBandeiraMunicipio;
        return Image();
    }
    // O Image que disparou o evento.
    Image sender_as_image(const aylis::Event& ev) const { return find_image(ev.sender_name); }
    // O Label deste form com o nome (como o FindComponent; vazio se não há).
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
    // O Layout deste form com o nome (como o FindComponent; vazio se não há).
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
    // O Panel deste form com o nome (como o FindComponent; vazio se não há).
    Panel find_panel(const std::string& name) const {
        if (name == "pnlEstado") return pnlEstado;
        if (name == "pnlMunicipio") return pnlMunicipio;
        return Panel();
    }
    // O Panel que disparou o evento.
    Panel sender_as_panel(const aylis::Event& ev) const { return find_panel(ev.sender_name); }
    // O Splitter deste form com o nome (como o FindComponent; vazio se não há).
    Splitter find_splitter(const std::string& name) const {
        if (name == "splListas") return splListas;
        return Splitter();
    }
    // O Splitter que disparou o evento.
    Splitter sender_as_splitter(const aylis::Event& ev) const { return find_splitter(ev.sender_name); }
    // O Svg deste form com o nome (como o FindComponent; vazio se não há).
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
