"""Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from aylis_client import CurrentForm, AylisComponent, AylisForm, register_form_class


class ClientDataSet(AylisComponent):
    """Um ClientDataSet de um form."""

    @property
    def key_fields(self):
        return self._get("key_fields")

    @key_fields.setter
    def key_fields(self, value):
        self._set("key_fields", value)

    @property
    def apply_mode(self):
        return self._get("apply_mode")

    @apply_mode.setter
    def apply_mode(self, value):
        self._set("apply_mode", value)

    @property
    def read_only(self):
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value):
        self._set("read_only", value)

    @property
    def active(self):
        return self._get("active")

    @active.setter
    def active(self, value):
        self._set("active", value)

    @property
    def filter(self):
        return self._get("filter")

    @filter.setter
    def filter(self, value):
        self._set("filter", value)

    @property
    def filtered(self):
        return self._get("filtered")

    @filtered.setter
    def filtered(self, value):
        self._set("filtered", value)

    @property
    def index_field_names(self):
        return self._get("index_field_names")

    @index_field_names.setter
    def index_field_names(self, value):
        self._set("index_field_names", value)

    @property
    def indexes(self):
        return self._get("indexes")

    @indexes.setter
    def indexes(self, value):
        self._set("indexes", value)

    @property
    def index_name(self):
        return self._get("index_name")

    @index_name.setter
    def index_name(self, value):
        self._set("index_name", value)

    @property
    def params(self):
        return self._get("params")

    @params.setter
    def params(self, value):
        self._set("params", value)

    @property
    def status_filter(self):
        return self._get("status_filter")

    @status_filter.setter
    def status_filter(self, value):
        self._set("status_filter", value)

    @property
    def master_source(self):
        return self._get("master_source")

    @master_source.setter
    def master_source(self, value):
        self._set("master_source", value)

    @property
    def master_fields(self):
        return self._get("master_fields")

    @master_fields.setter
    def master_fields(self, value):
        self._set("master_fields", value)

    @property
    def apply_separately(self):
        return self._get("apply_separately")

    @apply_separately.setter
    def apply_separately(self, value):
        self._set("apply_separately", value)

    @property
    def record_total(self):
        return self._get("record_total")

    @record_total.setter
    def record_total(self, value):
        self._set("record_total", value)

    @property
    def max_records(self):
        return self._get("max_records")

    @max_records.setter
    def max_records(self, value):
        self._set("max_records", value)

    @property
    def partial_notice(self):
        return self._get("partial_notice")


class DBChart(AylisComponent):
    """Um DBChart de um form."""

    @property
    def data_source(self):
        return self._get("data_source")

    @data_source.setter
    def data_source(self, value):
        self._set("data_source", value)

    @property
    def label_field(self):
        return self._get("label_field")

    @label_field.setter
    def label_field(self, value):
        self._set("label_field", value)

    @property
    def value_fields(self):
        return self._get("value_fields")

    @value_fields.setter
    def value_fields(self, value):
        self._set("value_fields", value)

    @property
    def kind(self):
        return self._get("kind")

    @kind.setter
    def kind(self, value):
        self._set("kind", value)

    @property
    def title(self):
        return self._get("title")

    @title.setter
    def title(self, value):
        self._set("title", value)

    @property
    def labels(self):
        return self._get("labels")

    @labels.setter
    def labels(self, value):
        self._set("labels", value)

    @property
    def series(self):
        return self._get("series")

    @series.setter
    def series(self, value):
        self._set("series", value)

    @property
    def json(self):
        return self._get("json")

    @json.setter
    def json(self, value):
        self._set("json", value)

    @property
    def legend(self):
        return self._get("legend")

    @legend.setter
    def legend(self, value):
        self._set("legend", value)

    @property
    def stepped(self):
        return self._get("stepped")

    @stepped.setter
    def stepped(self, value):
        self._set("stepped", value)

    @property
    def tension(self):
        return self._get("tension")

    @tension.setter
    def tension(self, value):
        self._set("tension", value)

    @property
    def animate(self):
        return self._get("animate")

    @animate.setter
    def animate(self, value):
        self._set("animate", value)

    @property
    def zoom(self):
        return self._get("zoom")

    @zoom.setter
    def zoom(self, value):
        self._set("zoom", value)

    @property
    def auto_scale(self):
        return self._get("auto_scale")

    @auto_scale.setter
    def auto_scale(self, value):
        self._set("auto_scale", value)

    @property
    def y_min(self):
        return self._get("y_min")

    @y_min.setter
    def y_min(self, value):
        self._set("y_min", value)

    @property
    def y_max(self):
        return self._get("y_max")

    @y_max.setter
    def y_max(self, value):
        self._set("y_max", value)

    @property
    def log_scale(self):
        return self._get("log_scale")

    @log_scale.setter
    def log_scale(self, value):
        self._set("log_scale", value)

    @property
    def reverse(self):
        return self._get("reverse")

    @reverse.setter
    def reverse(self, value):
        self._set("reverse", value)

    @property
    def time_axis(self):
        return self._get("time_axis")

    @time_axis.setter
    def time_axis(self, value):
        self._set("time_axis", value)

    @property
    def clicked_series(self):
        return self._get("clicked_series")

    @clicked_series.setter
    def clicked_series(self, value):
        self._set("clicked_series", value)

    @property
    def clicked_index(self):
        return self._get("clicked_index")

    @clicked_index.setter
    def clicked_index(self, value):
        self._set("clicked_index", value)

    @property
    def value_format(self):
        return self._get("value_format")

    @value_format.setter
    def value_format(self, value):
        self._set("value_format", value)

    @property
    def label_angle(self):
        return self._get("label_angle")

    @label_angle.setter
    def label_angle(self, value):
        self._set("label_angle", value)

    @property
    def title_align(self):
        return self._get("title_align")

    @title_align.setter
    def title_align(self, value):
        self._set("title_align", value)

    @property
    def title_bold(self):
        return self._get("title_bold")

    @title_bold.setter
    def title_bold(self, value):
        self._set("title_bold", value)

    @property
    def area_opacity(self):
        return self._get("area_opacity")

    @area_opacity.setter
    def area_opacity(self, value):
        self._set("area_opacity", value)

    @property
    def legend_position(self):
        return self._get("legend_position")

    @legend_position.setter
    def legend_position(self, value):
        self._set("legend_position", value)

    @property
    def stacked(self):
        return self._get("stacked")

    @stacked.setter
    def stacked(self, value):
        self._set("stacked", value)

    @property
    def colors(self):
        return self._get("colors")

    @colors.setter
    def colors(self, value):
        self._set("colors", value)

    @property
    def center_text(self):
        return self._get("center_text")

    @center_text.setter
    def center_text(self, value):
        self._set("center_text", value)

    @property
    def slice_labels(self):
        return self._get("slice_labels")

    @slice_labels.setter
    def slice_labels(self, value):
        self._set("slice_labels", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class DataGrid(AylisComponent):
    """Um DataGrid de um form."""

    @property
    def data_source(self):
        return self._get("data_source")

    @data_source.setter
    def data_source(self, value):
        self._set("data_source", value)

    @property
    def columns(self):
        return self._get("columns")

    @columns.setter
    def columns(self, value):
        self._set("columns", value)

    @property
    def row_height(self):
        return self._get("row_height")

    @row_height.setter
    def row_height(self, value):
        self._set("row_height", value)

    @property
    def read_only(self):
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value):
        self._set("read_only", value)

    @property
    def sortable(self):
        return self._get("sortable")

    @sortable.setter
    def sortable(self, value):
        self._set("sortable", value)

    @property
    def column_move(self):
        return self._get("column_move")

    @column_move.setter
    def column_move(self, value):
        self._set("column_move", value)

    @property
    def fixed_columns(self):
        return self._get("fixed_columns")

    @fixed_columns.setter
    def fixed_columns(self, value):
        self._set("fixed_columns", value)

    @property
    def filter_row(self):
        return self._get("filter_row")

    @filter_row.setter
    def filter_row(self, value):
        self._set("filter_row", value)

    @property
    def multi_select(self):
        return self._get("multi_select")

    @multi_select.setter
    def multi_select(self, value):
        self._set("multi_select", value)

    @property
    def footer(self):
        return self._get("footer")

    @footer.setter
    def footer(self, value):
        self._set("footer", value)

    @property
    def incremental_search(self):
        return self._get("incremental_search")

    @incremental_search.setter
    def incremental_search(self, value):
        self._set("incremental_search", value)

    @property
    def cursor_color(self):
        return self._get("cursor_color")

    @cursor_color.setter
    def cursor_color(self, value):
        self._set("cursor_color", value)

    @property
    def indicator(self):
        return self._get("indicator")

    @indicator.setter
    def indicator(self, value):
        self._set("indicator", value)

    @property
    def highlight_current(self):
        return self._get("highlight_current")

    @highlight_current.setter
    def highlight_current(self, value):
        self._set("highlight_current", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def set_focus(self):
        self.aylis.send("focus", target=self.id)


class DataSource(AylisComponent):
    """Um DataSource de um form."""

    @property
    def data_set(self):
        return self._get("data_set")

    @data_set.setter
    def data_set(self, value):
        self._set("data_set", value)

    @property
    def auto_edit(self):
        return self._get("auto_edit")

    @auto_edit.setter
    def auto_edit(self, value):
        self._set("auto_edit", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)


class Field(AylisComponent):
    """Um Field de um form."""

    @property
    def field_name(self):
        return self._get("field_name")

    @field_name.setter
    def field_name(self, value):
        self._set("field_name", value)

    @property
    def type(self):
        return self._get("type")

    @type.setter
    def type(self, value):
        self._set("type", value)

    @property
    def size(self):
        return self._get("size")

    @size.setter
    def size(self, value):
        self._set("size", value)

    @property
    def scale(self):
        return self._get("scale")

    @scale.setter
    def scale(self, value):
        self._set("scale", value)

    @property
    def rounding(self):
        return self._get("rounding")

    @rounding.setter
    def rounding(self, value):
        self._set("rounding", value)

    @property
    def display_label(self):
        return self._get("display_label")

    @display_label.setter
    def display_label(self, value):
        self._set("display_label", value)

    @property
    def display_width(self):
        return self._get("display_width")

    @display_width.setter
    def display_width(self, value):
        self._set("display_width", value)

    @property
    def display_format(self):
        return self._get("display_format")

    @display_format.setter
    def display_format(self, value):
        self._set("display_format", value)

    @property
    def edit_format(self):
        return self._get("edit_format")

    @edit_format.setter
    def edit_format(self, value):
        self._set("edit_format", value)

    @property
    def alignment(self):
        return self._get("alignment")

    @alignment.setter
    def alignment(self, value):
        self._set("alignment", value)

    @property
    def required(self):
        return self._get("required")

    @required.setter
    def required(self, value):
        self._set("required", value)

    @property
    def read_only(self):
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value):
        self._set("read_only", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def min(self):
        return self._get("min")

    @min.setter
    def min(self, value):
        self._set("min", value)

    @property
    def max(self):
        return self._get("max")

    @max.setter
    def max(self, value):
        self._set("max", value)

    @property
    def default_value(self):
        return self._get("default_value")

    @default_value.setter
    def default_value(self, value):
        self._set("default_value", value)

    @property
    def display_values(self):
        return self._get("display_values")

    @display_values.setter
    def display_values(self, value):
        self._set("display_values", value)

    @property
    def char_case(self):
        return self._get("char_case")

    @char_case.setter
    def char_case(self, value):
        self._set("char_case", value)

    @property
    def edit_mask(self):
        return self._get("edit_mask")

    @edit_mask.setter
    def edit_mask(self, value):
        self._set("edit_mask", value)

    @property
    def sensitive(self):
        return self._get("sensitive")

    @sensitive.setter
    def sensitive(self, value):
        self._set("sensitive", value)

    @property
    def kind(self):
        return self._get("kind")

    @kind.setter
    def kind(self, value):
        self._set("kind", value)

    @property
    def expression(self):
        return self._get("expression")

    @expression.setter
    def expression(self, value):
        self._set("expression", value)

    @property
    def lookup_data_set(self):
        return self._get("lookup_data_set")

    @lookup_data_set.setter
    def lookup_data_set(self, value):
        self._set("lookup_data_set", value)

    @property
    def key_fields(self):
        return self._get("key_fields")

    @key_fields.setter
    def key_fields(self, value):
        self._set("key_fields", value)

    @property
    def lookup_key_fields(self):
        return self._get("lookup_key_fields")

    @lookup_key_fields.setter
    def lookup_key_fields(self, value):
        self._set("lookup_key_fields", value)

    @property
    def lookup_result_field(self):
        return self._get("lookup_result_field")

    @lookup_result_field.setter
    def lookup_result_field(self, value):
        self._set("lookup_result_field", value)


class Form(AylisForm):
    """Um Form de um form."""

    @property
    def caption(self):
        return self._get("caption")

    @caption.setter
    def caption(self, value):
        self._set("caption", value)

    @property
    def position(self):
        return self._get("position")

    @position.setter
    def position(self, value):
        self._set("position", value)

    @property
    def border_style(self):
        return self._get("border_style")

    @border_style.setter
    def border_style(self, value):
        self._set("border_style", value)

    @property
    def window_state(self):
        return self._get("window_state")

    @window_state.setter
    def window_state(self, value):
        self._set("window_state", value)

    @property
    def transition(self):
        return self._get("transition")

    @transition.setter
    def transition(self, value):
        self._set("transition", value)

    @property
    def topmost(self):
        return self._get("topmost")

    @topmost.setter
    def topmost(self, value):
        self._set("topmost", value)

    @property
    def show_in_taskbar(self):
        return self._get("show_in_taskbar")

    @show_in_taskbar.setter
    def show_in_taskbar(self, value):
        self._set("show_in_taskbar", value)

    @property
    def active_control(self):
        return self._get("active_control")

    @active_control.setter
    def active_control(self, value):
        self._set("active_control", value)

    @property
    def tab_order(self):
        return self._get("tab_order")

    @tab_order.setter
    def tab_order(self, value):
        self._set("tab_order", value)

    @property
    def tab_keys(self):
        return self._get("tab_keys")

    @tab_keys.setter
    def tab_keys(self, value):
        self._set("tab_keys", value)

    @property
    def back_tab_keys(self):
        return self._get("back_tab_keys")

    @back_tab_keys.setter
    def back_tab_keys(self, value):
        self._set("back_tab_keys", value)

    @property
    def allowed_chars(self):
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value):
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self):
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value):
        self._set("blocked_chars", value)

    @property
    def only_allowed(self):
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value):
        self._set("only_allowed", value)

    @property
    def chars_override(self):
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value):
        self._set("chars_override", value)

    @property
    def char_map(self):
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value):
        self._set("char_map", value)

    @property
    def key_preview(self):
        return self._get("key_preview")

    @key_preview.setter
    def key_preview(self, value):
        self._set("key_preview", value)

    @property
    def zoom(self):
        return self._get("zoom")

    @zoom.setter
    def zoom(self, value):
        self._set("zoom", value)

    @property
    def monitor(self):
        return self._get("monitor")

    @monitor.setter
    def monitor(self, value):
        self._set("monitor", value)

    @property
    def remember_placement(self):
        return self._get("remember_placement")

    @remember_placement.setter
    def remember_placement(self, value):
        self._set("remember_placement", value)

    @property
    def gpu(self):
        return self._get("gpu")

    @gpu.setter
    def gpu(self, value):
        self._set("gpu", value)

    @property
    def activate_on_show(self):
        return self._get("activate_on_show")

    @activate_on_show.setter
    def activate_on_show(self, value):
        self._set("activate_on_show", value)

    @property
    def close_action(self):
        return self._get("close_action")

    @close_action.setter
    def close_action(self, value):
        self._set("close_action", value)

    @property
    def frame_rate(self):
        return self._get("frame_rate")

    @frame_rate.setter
    def frame_rate(self, value):
        self._set("frame_rate", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def move_to_monitor(self, monitor):
        self.aylis.send("invoke", target=self.id, method="move_to_monitor", args=[monitor])

    def gpu_adapter(self):
        return self.aylis.call("invoke", target=self.id, method="gpu_adapter", args=[])

    def gpu_adapters(self):
        return self.aylis.call("invoke", target=self.id, method="gpu_adapters", args=[])

    def focused(self):
        return self.aylis.call("focused", target=self.id)

    def focus_next(self):
        self.aylis.send("focus_next", target=self.id)

    def focus_prev(self):
        self.aylis.send("focus_prev", target=self.id)


class GroupBox(AylisComponent):
    """Um GroupBox de um form."""

    @property
    def caption(self):
        return self._get("caption")

    @caption.setter
    def caption(self, value):
        self._set("caption", value)

    @property
    def allowed_chars(self):
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value):
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self):
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value):
        self._set("blocked_chars", value)

    @property
    def only_allowed(self):
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value):
        self._set("only_allowed", value)

    @property
    def chars_override(self):
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value):
        self._set("chars_override", value)

    @property
    def char_map(self):
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value):
        self._set("char_map", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class Image(AylisComponent):
    """Um Image de um form."""

    @property
    def image(self):
        return self._get("image")

    @image.setter
    def image(self, value):
        self._set("image", value)

    @property
    def fit(self):
        return self._get("fit")

    @fit.setter
    def fit(self, value):
        self._set("fit", value)

    @property
    def radius(self):
        return self._get("radius")

    @radius.setter
    def radius(self, value):
        self._set("radius", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class Label(AylisComponent):
    """Um Label de um form."""

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

    @property
    def h_align(self):
        return self._get("h_align")

    @h_align.setter
    def h_align(self, value):
        self._set("h_align", value)

    @property
    def v_align(self):
        return self._get("v_align")

    @v_align.setter
    def v_align(self, value):
        self._set("v_align", value)

    @property
    def wrap(self):
        return self._get("wrap")

    @wrap.setter
    def wrap(self, value):
        self._set("wrap", value)

    @property
    def trimming(self):
        return self._get("trimming")

    @trimming.setter
    def trimming(self, value):
        self._set("trimming", value)

    @property
    def bold(self):
        return self._get("bold")

    @bold.setter
    def bold(self, value):
        self._set("bold", value)

    @property
    def color(self):
        return self._get("color")

    @color.setter
    def color(self, value):
        self._set("color", value)

    @property
    def highlight(self):
        return self._get("highlight")

    @highlight.setter
    def highlight(self, value):
        self._set("highlight", value)

    @property
    def letter_spacing(self):
        return self._get("letter_spacing")

    @letter_spacing.setter
    def letter_spacing(self, value):
        self._set("letter_spacing", value)

    @property
    def text_case(self):
        return self._get("text_case")

    @text_case.setter
    def text_case(self, value):
        self._set("text_case", value)

    @property
    def markup(self):
        return self._get("markup")

    @markup.setter
    def markup(self, value):
        self._set("markup", value)

    @property
    def outline(self):
        return self._get("outline")

    @outline.setter
    def outline(self, value):
        self._set("outline", value)

    @property
    def outline_width(self):
        return self._get("outline_width")

    @outline_width.setter
    def outline_width(self, value):
        self._set("outline_width", value)

    @property
    def line_height(self):
        return self._get("line_height")

    @line_height.setter
    def line_height(self, value):
        self._set("line_height", value)

    @property
    def text_shadow(self):
        return self._get("text_shadow")

    @text_shadow.setter
    def text_shadow(self, value):
        self._set("text_shadow", value)

    @property
    def focus_control(self):
        return self._get("focus_control")

    @focus_control.setter
    def focus_control(self, value):
        self._set("focus_control", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def min_content_width(self):
        return self.aylis.call("invoke", target=self.id, method="min_content_width", args=[])

    def max_content_width(self):
        return self.aylis.call("invoke", target=self.id, method="max_content_width", args=[])


class Layout(AylisComponent):
    """Um Layout de um form."""

    @property
    def allowed_chars(self):
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value):
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self):
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value):
        self._set("blocked_chars", value)

    @property
    def only_allowed(self):
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value):
        self._set("only_allowed", value)

    @property
    def chars_override(self):
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value):
        self._set("chars_override", value)

    @property
    def char_map(self):
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value):
        self._set("char_map", value)

    @property
    def gap(self):
        return self._get("gap")

    @gap.setter
    def gap(self, value):
        self._set("gap", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class Panel(AylisComponent):
    """Um Panel de um form."""

    @property
    def allowed_chars(self):
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value):
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self):
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value):
        self._set("blocked_chars", value)

    @property
    def only_allowed(self):
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value):
        self._set("only_allowed", value)

    @property
    def chars_override(self):
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value):
        self._set("chars_override", value)

    @property
    def char_map(self):
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value):
        self._set("char_map", value)

    @property
    def gap(self):
        return self._get("gap")

    @gap.setter
    def gap(self, value):
        self._set("gap", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def fill2(self):
        return self._get("fill2")

    @fill2.setter
    def fill2(self, value):
        self._set("fill2", value)

    @property
    def fill_mid(self):
        return self._get("fill_mid")

    @fill_mid.setter
    def fill_mid(self, value):
        self._set("fill_mid", value)

    @property
    def fill_mid_at(self):
        return self._get("fill_mid_at")

    @fill_mid_at.setter
    def fill_mid_at(self, value):
        self._set("fill_mid_at", value)

    @property
    def gradient_angle(self):
        return self._get("gradient_angle")

    @gradient_angle.setter
    def gradient_angle(self, value):
        self._set("gradient_angle", value)

    @property
    def gradient_radial(self):
        return self._get("gradient_radial")

    @gradient_radial.setter
    def gradient_radial(self, value):
        self._set("gradient_radial", value)

    @property
    def stroke(self):
        return self._get("stroke")

    @stroke.setter
    def stroke(self, value):
        self._set("stroke", value)

    @property
    def stroke_width(self):
        return self._get("stroke_width")

    @stroke_width.setter
    def stroke_width(self, value):
        self._set("stroke_width", value)

    @property
    def radius(self):
        return self._get("radius")

    @radius.setter
    def radius(self, value):
        self._set("radius", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class Splitter(AylisComponent):
    """Um Splitter de um form."""

    @property
    def min_size(self):
        return self._get("min_size")

    @min_size.setter
    def min_size(self, value):
        self._set("min_size", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class Svg(AylisComponent):
    """Um Svg de um form."""

    @property
    def source(self):
        return self._get("source")

    @source.setter
    def source(self, value):
        self._set("source", value)

    @property
    def color(self):
        return self._get("color")

    @color.setter
    def color(self, value):
        self._set("color", value)

    @property
    def fit(self):
        return self._get("fit")

    @fit.setter
    def fit(self, value):
        self._set("fit", value)

    @property
    def highlight(self):
        return self._get("highlight")

    @highlight.setter
    def highlight(self, value):
        self._set("highlight", value)

    @property
    def highlight_color(self):
        return self._get("highlight_color")

    @highlight_color.setter
    def highlight_color(self, value):
        self._set("highlight_color", value)

    @property
    def zoom_to(self):
        return self._get("zoom_to")

    @zoom_to.setter
    def zoom_to(self, value):
        self._set("zoom_to", value)

    @property
    def left(self):
        return self._get("left")

    @left.setter
    def left(self, value):
        self._set("left", value)

    @property
    def top(self):
        return self._get("top")

    @top.setter
    def top(self, value):
        self._set("top", value)

    @property
    def width(self):
        return self._get("width")

    @width.setter
    def width(self, value):
        self._set("width", value)

    @property
    def height(self):
        return self._get("height")

    @height.setter
    def height(self, value):
        self._set("height", value)

    @property
    def align(self):
        return self._get("align")

    @align.setter
    def align(self, value):
        self._set("align", value)

    @property
    def anchors(self):
        return self._get("anchors")

    @anchors.setter
    def anchors(self, value):
        self._set("anchors", value)

    @property
    def margins(self):
        return self._get("margins")

    @margins.setter
    def margins(self, value):
        self._set("margins", value)

    @property
    def padding(self):
        return self._get("padding")

    @padding.setter
    def padding(self, value):
        self._set("padding", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def hit_test(self):
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value):
        self._set("hit_test", value)

    @property
    def clip_children(self):
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value):
        self._set("clip_children", value)

    @property
    def inherit_state(self):
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value):
        self._set("inherit_state", value)

    @property
    def translate(self):
        return self._get("translate")

    @translate.setter
    def translate(self, value):
        self._set("translate", value)

    @property
    def captures_wheel(self):
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value):
        self._set("captures_wheel", value)

    @property
    def can_focus(self):
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value):
        self._set("can_focus", value)

    @property
    def draggable(self):
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value):
        self._set("draggable", value)

    @property
    def accepts_drops(self):
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value):
        self._set("accepts_drops", value)

    @property
    def drag_data(self):
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value):
        self._set("drag_data", value)

    @property
    def drag_label(self):
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value):
        self._set("drag_label", value)

    @property
    def drag_image(self):
        return self._get("drag_image")

    @drag_image.setter
    def drag_image(self, value):
        self._set("drag_image", value)

    @property
    def drop_types(self):
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value):
        self._set("drop_types", value)

    @property
    def tab_stop(self):
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value):
        self._set("tab_stop", value)

    @property
    def cursor(self):
        return self._get("cursor")

    @cursor.setter
    def cursor(self, value):
        self._set("cursor", value)

    @property
    def opacity(self):
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value):
        self._set("opacity", value)

    @property
    def cache_layer(self):
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value):
        self._set("cache_layer", value)

    @property
    def rotation(self):
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value):
        self._set("rotation", value)

    @property
    def translate_x(self):
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value):
        self._set("translate_x", value)

    @property
    def translate_y(self):
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value):
        self._set("translate_y", value)

    @property
    def aspect_ratio(self):
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value):
        self._set("aspect_ratio", value)

    @property
    def max_width(self):
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value):
        self._set("max_width", value)

    @property
    def min_width(self):
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value):
        self._set("min_width", value)

    @property
    def min_height(self):
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value):
        self._set("min_height", value)

    @property
    def max_height(self):
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value):
        self._set("max_height", value)

    @property
    def self_align(self):
        return self._get("self_align")

    @self_align.setter
    def self_align(self, value):
        self._set("self_align", value)

    @property
    def auto_size(self):
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value):
        self._set("auto_size", value)

    @property
    def column_span(self):
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value):
        self._set("column_span", value)

    @property
    def grid_row(self):
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value):
        self._set("grid_row", value)

    @property
    def grid_column(self):
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value):
        self._set("grid_column", value)

    @property
    def row_span(self):
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value):
        self._set("row_span", value)

    @property
    def grow(self):
        return self._get("grow")

    @grow.setter
    def grow(self, value):
        self._set("grow", value)

    @property
    def shadow(self):
        return self._get("shadow")

    @shadow.setter
    def shadow(self, value):
        self._set("shadow", value)

    @property
    def style(self):
        return self._get("style")

    @style.setter
    def style(self, value):
        self._set("style", value)

    @property
    def theme(self):
        return self._get("theme")

    @theme.setter
    def theme(self, value):
        self._set("theme", value)

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

    @property
    def border_color(self):
        return self._get("border_color")

    @border_color.setter
    def border_color(self, value):
        self._set("border_color", value)

    @property
    def text_color(self):
        return self._get("text_color")

    @text_color.setter
    def text_color(self, value):
        self._set("text_color", value)

    @property
    def accent_color(self):
        return self._get("accent_color")

    @accent_color.setter
    def accent_color(self, value):
        self._set("accent_color", value)

    @property
    def success_color(self):
        return self._get("success_color")

    @success_color.setter
    def success_color(self, value):
        self._set("success_color", value)

    @property
    def warning_color(self):
        return self._get("warning_color")

    @warning_color.setter
    def warning_color(self, value):
        self._set("warning_color", value)

    @property
    def danger_color(self):
        return self._get("danger_color")

    @danger_color.setter
    def danger_color(self, value):
        self._set("danger_color", value)

    @property
    def font_family(self):
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value):
        self._set("font_family", value)

    @property
    def font_size(self):
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value):
        self._set("font_size", value)

    @property
    def font_weight(self):
        return self._get("font_weight")

    @font_weight.setter
    def font_weight(self, value):
        self._set("font_weight", value)

    @property
    def font_italic(self):
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value):
        self._set("font_italic", value)

    @property
    def font_underline(self):
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value):
        self._set("font_underline", value)

    @property
    def font_strikeout(self):
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value):
        self._set("font_strikeout", value)

    @property
    def hint(self):
        return self._get("hint")

    @hint.setter
    def hint(self, value):
        self._set("hint", value)

    @property
    def tag(self):
        return self._get("tag")

    @tag.setter
    def tag(self, value):
        self._set("tag", value)

    @property
    def tag_string(self):
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value):
        self._set("tag_string", value)

    @property
    def accessible_name(self):
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value):
        self._set("accessible_name", value)

    @property
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class FrmIBGEComponents(Form):
    """O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância)."""

    def bind(self, components):
        self.cdsEstados = self._resolve(components, "cdsEstados", "ClientDataSet", ClientDataSet)
        self.cdsEstadosId = self._resolve(components, "cdsEstadosId", "Field", Field)
        self.cdsEstadosSigla = self._resolve(components, "cdsEstadosSigla", "Field", Field)
        self.cdsEstadosNome = self._resolve(components, "cdsEstadosNome", "Field", Field)
        self.cdsEstadosRegiao = self._resolve(components, "cdsEstadosRegiao", "Field", Field)
        self.cdsEstadosPopulacao = self._resolve(components, "cdsEstadosPopulacao", "Field", Field)
        self.cdsEstadosArea = self._resolve(components, "cdsEstadosArea", "Field", Field)
        self.cdsEstadosDensidade = self._resolve(components, "cdsEstadosDensidade", "Field", Field)
        self.cdsEstadosPib = self._resolve(components, "cdsEstadosPib", "Field", Field)
        self.cdsEstadosPibPerCapita = self._resolve(components, "cdsEstadosPibPerCapita", "Field", Field)
        self.cdsEstadosEstimativa = self._resolve(components, "cdsEstadosEstimativa", "Field", Field)
        self.cdsEstadosEstimativaAno = self._resolve(components, "cdsEstadosEstimativaAno", "Field", Field)
        self.cdsEstadosMunicipios = self._resolve(components, "cdsEstadosMunicipios", "Field", Field)
        self.cdsEstadosBandeira = self._resolve(components, "cdsEstadosBandeira", "Field", Field)
        self.dsEstados = self._resolve(components, "dsEstados", "DataSource", DataSource)
        self.cdsMunicipios = self._resolve(components, "cdsMunicipios", "ClientDataSet", ClientDataSet)
        self.cdsMunicipiosId = self._resolve(components, "cdsMunicipiosId", "Field", Field)
        self.cdsMunicipiosUfId = self._resolve(components, "cdsMunicipiosUfId", "Field", Field)
        self.cdsMunicipiosUf = self._resolve(components, "cdsMunicipiosUf", "Field", Field)
        self.cdsMunicipiosNome = self._resolve(components, "cdsMunicipiosNome", "Field", Field)
        self.cdsMunicipiosMesorregiao = self._resolve(components, "cdsMunicipiosMesorregiao", "Field", Field)
        self.cdsMunicipiosMicrorregiao = self._resolve(components, "cdsMunicipiosMicrorregiao", "Field", Field)
        self.cdsMunicipiosRegiaoIntermediaria = self._resolve(components, "cdsMunicipiosRegiaoIntermediaria", "Field", Field)
        self.cdsMunicipiosRegiaoImediata = self._resolve(components, "cdsMunicipiosRegiaoImediata", "Field", Field)
        self.cdsMunicipiosPopulacao = self._resolve(components, "cdsMunicipiosPopulacao", "Field", Field)
        self.cdsMunicipiosArea = self._resolve(components, "cdsMunicipiosArea", "Field", Field)
        self.cdsMunicipiosDensidade = self._resolve(components, "cdsMunicipiosDensidade", "Field", Field)
        self.cdsMunicipiosPib = self._resolve(components, "cdsMunicipiosPib", "Field", Field)
        self.cdsMunicipiosPibParticipacao = self._resolve(components, "cdsMunicipiosPibParticipacao", "Field", Field)
        self.cdsMunicipiosPibPerCapita = self._resolve(components, "cdsMunicipiosPibPerCapita", "Field", Field)
        self.cdsMunicipiosEstimativa = self._resolve(components, "cdsMunicipiosEstimativa", "Field", Field)
        self.cdsMunicipiosEstimativaAno = self._resolve(components, "cdsMunicipiosEstimativaAno", "Field", Field)
        self.cdsMunicipiosDistritos = self._resolve(components, "cdsMunicipiosDistritos", "Field", Field)
        self.cdsMunicipiosBandeira = self._resolve(components, "cdsMunicipiosBandeira", "Field", Field)
        self.dsMunicipios = self._resolve(components, "dsMunicipios", "DataSource", DataSource)
        self.cdsDistritos = self._resolve(components, "cdsDistritos", "ClientDataSet", ClientDataSet)
        self.cdsDistritosId = self._resolve(components, "cdsDistritosId", "Field", Field)
        self.cdsDistritosMunicipioId = self._resolve(components, "cdsDistritosMunicipioId", "Field", Field)
        self.cdsDistritosNome = self._resolve(components, "cdsDistritosNome", "Field", Field)
        self.dsDistritos = self._resolve(components, "dsDistritos", "DataSource", DataSource)
        self.cdsPopulacao = self._resolve(components, "cdsPopulacao", "ClientDataSet", ClientDataSet)
        self.cdsPopulacaoAno = self._resolve(components, "cdsPopulacaoAno", "Field", Field)
        self.cdsPopulacaoHabitantes = self._resolve(components, "cdsPopulacaoHabitantes", "Field", Field)
        self.dsPopulacao = self._resolve(components, "dsPopulacao", "DataSource", DataSource)
        self.cdsMaiores = self._resolve(components, "cdsMaiores", "ClientDataSet", ClientDataSet)
        self.cdsMaioresNome = self._resolve(components, "cdsMaioresNome", "Field", Field)
        self.cdsMaioresHabitantes = self._resolve(components, "cdsMaioresHabitantes", "Field", Field)
        self.dsMaiores = self._resolve(components, "dsMaiores", "DataSource", DataSource)
        self.lblFonte = self._resolve(components, "lblFonte", "Label", Label)
        self.lytListas = self._resolve(components, "lytListas", "Layout", Layout)
        self.gbEstados = self._resolve(components, "gbEstados", "GroupBox", GroupBox)
        self.grdEstados = self._resolve(components, "grdEstados", "DataGrid", DataGrid)
        self.gbDistritos = self._resolve(components, "gbDistritos", "GroupBox", GroupBox)
        self.grdDistritos = self._resolve(components, "grdDistritos", "DataGrid", DataGrid)
        self.gbMunicipios = self._resolve(components, "gbMunicipios", "GroupBox", GroupBox)
        self.grdMunicipios = self._resolve(components, "grdMunicipios", "DataGrid", DataGrid)
        self.splListas = self._resolve(components, "splListas", "Splitter", Splitter)
        self.lytPainel = self._resolve(components, "lytPainel", "Layout", Layout)
        self.pnlEstado = self._resolve(components, "pnlEstado", "Panel", Panel)
        self.svgBrasil = self._resolve(components, "svgBrasil", "Svg", Svg)
        self.lytBandeiraEstado = self._resolve(components, "lytBandeiraEstado", "Layout", Layout)
        self.lblEstado = self._resolve(components, "lblEstado", "Label", Label)
        self.lblRegiao = self._resolve(components, "lblRegiao", "Label", Label)
        self.imgBandeiraEstado = self._resolve(components, "imgBandeiraEstado", "Image", Image)
        self.lytNumerosEstado = self._resolve(components, "lytNumerosEstado", "Layout", Layout)
        self.dbtEstadoPopulacao = self._resolve(components, "dbtEstadoPopulacao", "Label", Label)
        self.lblEstadoEstimativa = self._resolve(components, "lblEstadoEstimativa", "Label", Label)
        self.dbtEstadoEstimativa = self._resolve(components, "dbtEstadoEstimativa", "Label", Label)
        self.dbtEstadoArea = self._resolve(components, "dbtEstadoArea", "Label", Label)
        self.dbtEstadoDensidade = self._resolve(components, "dbtEstadoDensidade", "Label", Label)
        self.dbtEstadoPib = self._resolve(components, "dbtEstadoPib", "Label", Label)
        self.dbtEstadoPibPerCapita = self._resolve(components, "dbtEstadoPibPerCapita", "Label", Label)
        self.dbtEstadoMunicipios = self._resolve(components, "dbtEstadoMunicipios", "Label", Label)
        self.pnlMunicipio = self._resolve(components, "pnlMunicipio", "Panel", Panel)
        self.svgMunicipio = self._resolve(components, "svgMunicipio", "Svg", Svg)
        self.svgEstado = self._resolve(components, "svgEstado", "Svg", Svg)
        self.lytBandeiraMunicipio = self._resolve(components, "lytBandeiraMunicipio", "Layout", Layout)
        self.lblMunicipio = self._resolve(components, "lblMunicipio", "Label", Label)
        self.lblCodigo = self._resolve(components, "lblCodigo", "Label", Label)
        self.imgBandeiraMunicipio = self._resolve(components, "imgBandeiraMunicipio", "Image", Image)
        self.lblSemBandeira = self._resolve(components, "lblSemBandeira", "Label", Label)
        self.lytNumerosMunicipio = self._resolve(components, "lytNumerosMunicipio", "Layout", Layout)
        self.dbtMesorregiao = self._resolve(components, "dbtMesorregiao", "Label", Label)
        self.dbtMicrorregiao = self._resolve(components, "dbtMicrorregiao", "Label", Label)
        self.dbtRegiaoImediata = self._resolve(components, "dbtRegiaoImediata", "Label", Label)
        self.dbtPopulacao = self._resolve(components, "dbtPopulacao", "Label", Label)
        self.lblEstimativa = self._resolve(components, "lblEstimativa", "Label", Label)
        self.dbtEstimativa = self._resolve(components, "dbtEstimativa", "Label", Label)
        self.dbtArea = self._resolve(components, "dbtArea", "Label", Label)
        self.dbtDensidade = self._resolve(components, "dbtDensidade", "Label", Label)
        self.dbtPib = self._resolve(components, "dbtPib", "Label", Label)
        self.dbtPibPerCapita = self._resolve(components, "dbtPibPerCapita", "Label", Label)
        self.dbtPibParticipacao = self._resolve(components, "dbtPibParticipacao", "Label", Label)
        self.lytGraficos = self._resolve(components, "lytGraficos", "Layout", Layout)
        self.dbcPopulacao = self._resolve(components, "dbcPopulacao", "DBChart", DBChart)
        self.dbcMaiores = self._resolve(components, "dbcMaiores", "DBChart", DBChart)
        self.dbcEstados = self._resolve(components, "dbcEstados", "DBChart", DBChart)

    def find_client_data_set(self, name):
        """O ClientDataSet deste form com o nome (como o FindComponent)."""
        if name == "cdsEstados":
            return self.cdsEstados
        if name == "cdsMunicipios":
            return self.cdsMunicipios
        if name == "cdsDistritos":
            return self.cdsDistritos
        if name == "cdsPopulacao":
            return self.cdsPopulacao
        if name == "cdsMaiores":
            return self.cdsMaiores
        return None

    def sender_as_client_data_set(self, event):
        """O ClientDataSet que disparou o evento."""
        return self.find_client_data_set(event["sender"]["name"])

    def find_d_b_chart(self, name):
        """O DBChart deste form com o nome (como o FindComponent)."""
        if name == "dbcPopulacao":
            return self.dbcPopulacao
        if name == "dbcMaiores":
            return self.dbcMaiores
        if name == "dbcEstados":
            return self.dbcEstados
        return None

    def sender_as_d_b_chart(self, event):
        """O DBChart que disparou o evento."""
        return self.find_d_b_chart(event["sender"]["name"])

    def find_data_grid(self, name):
        """O DataGrid deste form com o nome (como o FindComponent)."""
        if name == "grdEstados":
            return self.grdEstados
        if name == "grdDistritos":
            return self.grdDistritos
        if name == "grdMunicipios":
            return self.grdMunicipios
        return None

    def sender_as_data_grid(self, event):
        """O DataGrid que disparou o evento."""
        return self.find_data_grid(event["sender"]["name"])

    def find_data_source(self, name):
        """O DataSource deste form com o nome (como o FindComponent)."""
        if name == "dsEstados":
            return self.dsEstados
        if name == "dsMunicipios":
            return self.dsMunicipios
        if name == "dsDistritos":
            return self.dsDistritos
        if name == "dsPopulacao":
            return self.dsPopulacao
        if name == "dsMaiores":
            return self.dsMaiores
        return None

    def sender_as_data_source(self, event):
        """O DataSource que disparou o evento."""
        return self.find_data_source(event["sender"]["name"])

    def find_field(self, name):
        """O Field deste form com o nome (como o FindComponent)."""
        if name == "cdsEstadosId":
            return self.cdsEstadosId
        if name == "cdsEstadosSigla":
            return self.cdsEstadosSigla
        if name == "cdsEstadosNome":
            return self.cdsEstadosNome
        if name == "cdsEstadosRegiao":
            return self.cdsEstadosRegiao
        if name == "cdsEstadosPopulacao":
            return self.cdsEstadosPopulacao
        if name == "cdsEstadosArea":
            return self.cdsEstadosArea
        if name == "cdsEstadosDensidade":
            return self.cdsEstadosDensidade
        if name == "cdsEstadosPib":
            return self.cdsEstadosPib
        if name == "cdsEstadosPibPerCapita":
            return self.cdsEstadosPibPerCapita
        if name == "cdsEstadosEstimativa":
            return self.cdsEstadosEstimativa
        if name == "cdsEstadosEstimativaAno":
            return self.cdsEstadosEstimativaAno
        if name == "cdsEstadosMunicipios":
            return self.cdsEstadosMunicipios
        if name == "cdsEstadosBandeira":
            return self.cdsEstadosBandeira
        if name == "cdsMunicipiosId":
            return self.cdsMunicipiosId
        if name == "cdsMunicipiosUfId":
            return self.cdsMunicipiosUfId
        if name == "cdsMunicipiosUf":
            return self.cdsMunicipiosUf
        if name == "cdsMunicipiosNome":
            return self.cdsMunicipiosNome
        if name == "cdsMunicipiosMesorregiao":
            return self.cdsMunicipiosMesorregiao
        if name == "cdsMunicipiosMicrorregiao":
            return self.cdsMunicipiosMicrorregiao
        if name == "cdsMunicipiosRegiaoIntermediaria":
            return self.cdsMunicipiosRegiaoIntermediaria
        if name == "cdsMunicipiosRegiaoImediata":
            return self.cdsMunicipiosRegiaoImediata
        if name == "cdsMunicipiosPopulacao":
            return self.cdsMunicipiosPopulacao
        if name == "cdsMunicipiosArea":
            return self.cdsMunicipiosArea
        if name == "cdsMunicipiosDensidade":
            return self.cdsMunicipiosDensidade
        if name == "cdsMunicipiosPib":
            return self.cdsMunicipiosPib
        if name == "cdsMunicipiosPibParticipacao":
            return self.cdsMunicipiosPibParticipacao
        if name == "cdsMunicipiosPibPerCapita":
            return self.cdsMunicipiosPibPerCapita
        if name == "cdsMunicipiosEstimativa":
            return self.cdsMunicipiosEstimativa
        if name == "cdsMunicipiosEstimativaAno":
            return self.cdsMunicipiosEstimativaAno
        if name == "cdsMunicipiosDistritos":
            return self.cdsMunicipiosDistritos
        if name == "cdsMunicipiosBandeira":
            return self.cdsMunicipiosBandeira
        if name == "cdsDistritosId":
            return self.cdsDistritosId
        if name == "cdsDistritosMunicipioId":
            return self.cdsDistritosMunicipioId
        if name == "cdsDistritosNome":
            return self.cdsDistritosNome
        if name == "cdsPopulacaoAno":
            return self.cdsPopulacaoAno
        if name == "cdsPopulacaoHabitantes":
            return self.cdsPopulacaoHabitantes
        if name == "cdsMaioresNome":
            return self.cdsMaioresNome
        if name == "cdsMaioresHabitantes":
            return self.cdsMaioresHabitantes
        return None

    def sender_as_field(self, event):
        """O Field que disparou o evento."""
        return self.find_field(event["sender"]["name"])

    def find_group_box(self, name):
        """O GroupBox deste form com o nome (como o FindComponent)."""
        if name == "gbEstados":
            return self.gbEstados
        if name == "gbDistritos":
            return self.gbDistritos
        if name == "gbMunicipios":
            return self.gbMunicipios
        return None

    def sender_as_group_box(self, event):
        """O GroupBox que disparou o evento."""
        return self.find_group_box(event["sender"]["name"])

    def find_image(self, name):
        """O Image deste form com o nome (como o FindComponent)."""
        if name == "imgBandeiraEstado":
            return self.imgBandeiraEstado
        if name == "imgBandeiraMunicipio":
            return self.imgBandeiraMunicipio
        return None

    def sender_as_image(self, event):
        """O Image que disparou o evento."""
        return self.find_image(event["sender"]["name"])

    def find_label(self, name):
        """O Label deste form com o nome (como o FindComponent)."""
        if name == "lblFonte":
            return self.lblFonte
        if name == "lblEstado":
            return self.lblEstado
        if name == "lblRegiao":
            return self.lblRegiao
        if name == "dbtEstadoPopulacao":
            return self.dbtEstadoPopulacao
        if name == "lblEstadoEstimativa":
            return self.lblEstadoEstimativa
        if name == "dbtEstadoEstimativa":
            return self.dbtEstadoEstimativa
        if name == "dbtEstadoArea":
            return self.dbtEstadoArea
        if name == "dbtEstadoDensidade":
            return self.dbtEstadoDensidade
        if name == "dbtEstadoPib":
            return self.dbtEstadoPib
        if name == "dbtEstadoPibPerCapita":
            return self.dbtEstadoPibPerCapita
        if name == "dbtEstadoMunicipios":
            return self.dbtEstadoMunicipios
        if name == "lblMunicipio":
            return self.lblMunicipio
        if name == "lblCodigo":
            return self.lblCodigo
        if name == "lblSemBandeira":
            return self.lblSemBandeira
        if name == "dbtMesorregiao":
            return self.dbtMesorregiao
        if name == "dbtMicrorregiao":
            return self.dbtMicrorregiao
        if name == "dbtRegiaoImediata":
            return self.dbtRegiaoImediata
        if name == "dbtPopulacao":
            return self.dbtPopulacao
        if name == "lblEstimativa":
            return self.lblEstimativa
        if name == "dbtEstimativa":
            return self.dbtEstimativa
        if name == "dbtArea":
            return self.dbtArea
        if name == "dbtDensidade":
            return self.dbtDensidade
        if name == "dbtPib":
            return self.dbtPib
        if name == "dbtPibPerCapita":
            return self.dbtPibPerCapita
        if name == "dbtPibParticipacao":
            return self.dbtPibParticipacao
        return None

    def sender_as_label(self, event):
        """O Label que disparou o evento."""
        return self.find_label(event["sender"]["name"])

    def find_layout(self, name):
        """O Layout deste form com o nome (como o FindComponent)."""
        if name == "lytListas":
            return self.lytListas
        if name == "lytPainel":
            return self.lytPainel
        if name == "lytBandeiraEstado":
            return self.lytBandeiraEstado
        if name == "lytNumerosEstado":
            return self.lytNumerosEstado
        if name == "lytBandeiraMunicipio":
            return self.lytBandeiraMunicipio
        if name == "lytNumerosMunicipio":
            return self.lytNumerosMunicipio
        if name == "lytGraficos":
            return self.lytGraficos
        return None

    def sender_as_layout(self, event):
        """O Layout que disparou o evento."""
        return self.find_layout(event["sender"]["name"])

    def find_panel(self, name):
        """O Panel deste form com o nome (como o FindComponent)."""
        if name == "pnlEstado":
            return self.pnlEstado
        if name == "pnlMunicipio":
            return self.pnlMunicipio
        return None

    def sender_as_panel(self, event):
        """O Panel que disparou o evento."""
        return self.find_panel(event["sender"]["name"])

    def find_splitter(self, name):
        """O Splitter deste form com o nome (como o FindComponent)."""
        if name == "splListas":
            return self.splListas
        return None

    def sender_as_splitter(self, event):
        """O Splitter que disparou o evento."""
        return self.find_splitter(event["sender"]["name"])

    def find_svg(self, name):
        """O Svg deste form com o nome (como o FindComponent)."""
        if name == "svgBrasil":
            return self.svgBrasil
        if name == "svgMunicipio":
            return self.svgMunicipio
        if name == "svgEstado":
            return self.svgEstado
        return None

    def sender_as_svg(self, event):
        """O Svg que disparou o evento."""
        return self.find_svg(event["sender"]["name"])


# A janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers).
frmIBGE = CurrentForm("frmIBGE")
register_form_class("frmIBGE", FrmIBGEComponents, frmIBGE._activate)
