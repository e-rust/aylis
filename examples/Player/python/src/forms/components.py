"""Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from aylis_client import CurrentForm, AylisComponent, AylisForm, register_form_class


class Button(AylisComponent):
    """Um Button de um form."""

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

    @property
    def icon(self):
        return self._get("icon")

    @icon.setter
    def icon(self, value):
        self._set("icon", value)

    @property
    def checked_icon(self):
        return self._get("checked_icon")

    @checked_icon.setter
    def checked_icon(self, value):
        self._set("checked_icon", value)

    @property
    def icon_position(self):
        return self._get("icon_position")

    @icon_position.setter
    def icon_position(self, value):
        self._set("icon_position", value)

    @property
    def icon_size(self):
        return self._get("icon_size")

    @icon_size.setter
    def icon_size(self, value):
        self._set("icon_size", value)

    @property
    def checkable(self):
        return self._get("checkable")

    @checkable.setter
    def checkable(self, value):
        self._set("checkable", value)

    @property
    def checked(self):
        return self._get("checked")

    @checked.setter
    def checked(self, value):
        self._set("checked", value)

    @property
    def modal_result(self):
        return self._get("modal_result")

    @modal_result.setter
    def modal_result(self, value):
        self._set("modal_result", value)

    @property
    def repeat_enter(self):
        return self._get("repeat_enter")

    @repeat_enter.setter
    def repeat_enter(self, value):
        self._set("repeat_enter", value)

    @property
    def default_button(self):
        return self._get("default_button")

    @default_button.setter
    def default_button(self, value):
        self._set("default_button", value)

    @property
    def cancel_button(self):
        return self._get("cancel_button")

    @cancel_button.setter
    def cancel_button(self, value):
        self._set("cancel_button", value)

    @property
    def h_align(self):
        return self._get("h_align")

    @h_align.setter
    def h_align(self, value):
        self._set("h_align", value)

    @property
    def group_index(self):
        return self._get("group_index")

    @group_index.setter
    def group_index(self, value):
        self._set("group_index", value)

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

    def set_focus(self):
        self.aylis.send("focus", target=self.id)


class Divider(AylisComponent):
    """Um Divider de um form."""

    @property
    def vertical(self):
        return self._get("vertical")

    @vertical.setter
    def vertical(self, value):
        self._set("vertical", value)

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

    @property
    def thickness(self):
        return self._get("thickness")

    @thickness.setter
    def thickness(self, value):
        self._set("thickness", value)

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


class Icon(AylisComponent):
    """Um Icon de um form."""

    @property
    def icon(self):
        return self._get("icon")

    @icon.setter
    def icon(self, value):
        self._set("icon", value)

    @property
    def color(self):
        return self._get("color")

    @color.setter
    def color(self, value):
        self._set("color", value)

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


class ListView(AylisComponent):
    """Um ListView de um form."""

    @property
    def items(self):
        return self._get("items")

    @items.setter
    def items(self, value):
        self._set("items", value)

    @property
    def item_index(self):
        return self._get("item_index")

    @item_index.setter
    def item_index(self, value):
        self._set("item_index", value)

    @property
    def item_height(self):
        return self._get("item_height")

    @item_height.setter
    def item_height(self, value):
        self._set("item_height", value)

    @property
    def accessory(self):
        return self._get("accessory")

    @accessory.setter
    def accessory(self, value):
        self._set("accessory", value)

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


class MainMenu(AylisComponent):
    """Um MainMenu de um form."""

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


class MenuItem(AylisComponent):
    """Um MenuItem de um form."""

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

    @property
    def shortcut(self):
        return self._get("shortcut")

    @shortcut.setter
    def shortcut(self, value):
        self._set("shortcut", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def checked(self):
        return self._get("checked")

    @checked.setter
    def checked(self, value):
        self._set("checked", value)

    @property
    def auto_check(self):
        return self._get("auto_check")

    @auto_check.setter
    def auto_check(self, value):
        self._set("auto_check", value)

    @property
    def icon(self):
        return self._get("icon")

    @icon.setter
    def icon(self, value):
        self._set("icon", value)

    @property
    def visible(self):
        return self._get("visible")

    @visible.setter
    def visible(self, value):
        self._set("visible", value)

    @property
    def danger(self):
        return self._get("danger")

    @danger.setter
    def danger(self, value):
        self._set("danger", value)

    @property
    def header(self):
        return self._get("header")

    @header.setter
    def header(self, value):
        self._set("header", value)


class OpenDialog(AylisComponent):
    """Um OpenDialog de um form."""

    @property
    def title(self):
        return self._get("title")

    @title.setter
    def title(self, value):
        self._set("title", value)

    @property
    def filter(self):
        return self._get("filter")

    @filter.setter
    def filter(self, value):
        self._set("filter", value)

    @property
    def initial_dir(self):
        return self._get("initial_dir")

    @initial_dir.setter
    def initial_dir(self, value):
        self._set("initial_dir", value)

    @property
    def file_name(self):
        return self._get("file_name")

    @file_name.setter
    def file_name(self, value):
        self._set("file_name", value)

    @property
    def multiple(self):
        return self._get("multiple")

    @multiple.setter
    def multiple(self, value):
        self._set("multiple", value)

    def execute(self):
        self.aylis.send("execute", target=self.id)


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


class SeekBar(AylisComponent):
    """Um SeekBar de um form."""

    @property
    def position(self):
        return self._get("position")

    @position.setter
    def position(self, value):
        self._set("position", value)

    @property
    def duration(self):
        return self._get("duration")

    @duration.setter
    def duration(self, value):
        self._set("duration", value)

    @property
    def buffered(self):
        return self._get("buffered")

    @buffered.setter
    def buffered(self, value):
        self._set("buffered", value)

    @property
    def markers(self):
        return self._get("markers")

    @markers.setter
    def markers(self, value):
        self._set("markers", value)

    @property
    def show_time(self):
        return self._get("show_time")

    @show_time.setter
    def show_time(self, value):
        self._set("show_time", value)

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


class Timer(AylisComponent):
    """Um Timer de um form."""

    @property
    def interval(self):
        return self._get("interval")

    @interval.setter
    def interval(self, value):
        self._set("interval", value)

    @property
    def enabled(self):
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value):
        self._set("enabled", value)

    @property
    def precision(self):
        return self._get("precision")

    @precision.setter
    def precision(self, value):
        self._set("precision", value)

    @property
    def background(self):
        return self._get("background")

    @background.setter
    def background(self, value):
        self._set("background", value)

    @property
    def one_shot(self):
        return self._get("one_shot")

    @one_shot.setter
    def one_shot(self, value):
        self._set("one_shot", value)

    @property
    def drift(self):
        return self._get("drift")

    @drift.setter
    def drift(self, value):
        self._set("drift", value)

    def restart(self):
        self.aylis.send("invoke", target=self.id, method="restart", args=[])


class TrackBar(AylisComponent):
    """Um TrackBar de um form."""

    @property
    def value(self):
        return self._get("value")

    @value.setter
    def value(self, value):
        self._set("value", value)

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
    def frequency(self):
        return self._get("frequency")

    @frequency.setter
    def frequency(self, value):
        self._set("frequency", value)

    @property
    def vertical(self):
        return self._get("vertical")

    @vertical.setter
    def vertical(self, value):
        self._set("vertical", value)

    @property
    def thumb_size(self):
        return self._get("thumb_size")

    @thumb_size.setter
    def thumb_size(self, value):
        self._set("thumb_size", value)

    @property
    def track_size(self):
        return self._get("track_size")

    @track_size.setter
    def track_size(self, value):
        self._set("track_size", value)

    @property
    def solid_thumb(self):
        return self._get("solid_thumb")

    @solid_thumb.setter
    def solid_thumb(self, value):
        self._set("solid_thumb", value)

    @property
    def limit(self):
        return self._get("limit")

    @limit.setter
    def limit(self, value):
        self._set("limit", value)

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


class VideoPlayer(AylisComponent):
    """Um VideoPlayer de um form."""

    @property
    def file(self):
        return self._get("file")

    @file.setter
    def file(self, value):
        self._set("file", value)

    @property
    def autoplay(self):
        return self._get("autoplay")

    @autoplay.setter
    def autoplay(self, value):
        self._set("autoplay", value)

    @property
    def looping(self):
        return self._get("looping")

    @looping.setter
    def looping(self, value):
        self._set("looping", value)

    @property
    def volume(self):
        return self._get("volume")

    @volume.setter
    def volume(self, value):
        self._set("volume", value)

    @property
    def muted(self):
        return self._get("muted")

    @muted.setter
    def muted(self, value):
        self._set("muted", value)

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
    def rate(self):
        return self._get("rate")

    @rate.setter
    def rate(self, value):
        self._set("rate", value)

    @property
    def output(self):
        return self._get("output")

    @output.setter
    def output(self, value):
        self._set("output", value)

    @property
    def show_video(self):
        return self._get("show_video")

    @show_video.setter
    def show_video(self, value):
        self._set("show_video", value)

    @property
    def frames_ahead(self):
        return self._get("frames_ahead")

    @frames_ahead.setter
    def frames_ahead(self, value):
        self._set("frames_ahead", value)

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

    def play(self):
        self.aylis.send("media", target=self.id, op="play")

    def pause(self):
        self.aylis.send("media", target=self.id, op="pause")

    def stop(self):
        self.aylis.send("media", target=self.id, op="stop")

    def seek(self, seconds):
        self.aylis.send("media", target=self.id, op="seek", seconds=seconds)

    def position(self):
        return self.aylis.call("media", target=self.id, op="position")

    def duration(self):
        return self.aylis.call("media", target=self.id, op="duration")


class FrmPlayerComponents(Form):
    """O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância)."""

    def bind(self, components):
        self.mnuPrincipal = self._resolve(components, "mnuPrincipal", "MainMenu", MainMenu)
        self.mnuArquivo = self._resolve(components, "mnuArquivo", "MenuItem", MenuItem)
        self.mnuAbrir = self._resolve(components, "mnuAbrir", "MenuItem", MenuItem)
        self.mnuLimparFila = self._resolve(components, "mnuLimparFila", "MenuItem", MenuItem)
        self.mnuSep1 = self._resolve(components, "mnuSep1", "MenuItem", MenuItem)
        self.mnuSair = self._resolve(components, "mnuSair", "MenuItem", MenuItem)
        self.mnuReproduzir = self._resolve(components, "mnuReproduzir", "MenuItem", MenuItem)
        self.mnuReproduzirPausar = self._resolve(components, "mnuReproduzirPausar", "MenuItem", MenuItem)
        self.mnuParar = self._resolve(components, "mnuParar", "MenuItem", MenuItem)
        self.mnuAnterior = self._resolve(components, "mnuAnterior", "MenuItem", MenuItem)
        self.mnuProxima = self._resolve(components, "mnuProxima", "MenuItem", MenuItem)
        self.mnuSep2 = self._resolve(components, "mnuSep2", "MenuItem", MenuItem)
        self.mnuEmbaralhar = self._resolve(components, "mnuEmbaralhar", "MenuItem", MenuItem)
        self.mnuRepetir = self._resolve(components, "mnuRepetir", "MenuItem", MenuItem)
        self.mnuExibir = self._resolve(components, "mnuExibir", "MenuItem", MenuItem)
        self.mnuAgora = self._resolve(components, "mnuAgora", "MenuItem", MenuItem)
        self.mnuFila = self._resolve(components, "mnuFila", "MenuItem", MenuItem)
        self.pnlControles = self._resolve(components, "pnlControles", "Panel", Panel)
        self.lytBarra = self._resolve(components, "lytBarra", "Layout", Layout)
        self.lblTempo = self._resolve(components, "lblTempo", "Label", Label)
        self.lblDuracao = self._resolve(components, "lblDuracao", "Label", Label)
        self.sbPosicao = self._resolve(components, "sbPosicao", "SeekBar", SeekBar)
        self.lytInfo = self._resolve(components, "lytInfo", "Layout", Layout)
        self.icoCapa = self._resolve(components, "icoCapa", "Icon", Icon)
        self.lytTextos = self._resolve(components, "lytTextos", "Layout", Layout)
        self.lblTitulo = self._resolve(components, "lblTitulo", "Label", Label)
        self.lblDetalhe = self._resolve(components, "lblDetalhe", "Label", Label)
        self.lytVolume = self._resolve(components, "lytVolume", "Layout", Layout)
        self.btnMudo = self._resolve(components, "btnMudo", "Button", Button)
        self.tbVolume = self._resolve(components, "tbVolume", "TrackBar", TrackBar)
        self.lytBotoes = self._resolve(components, "lytBotoes", "Layout", Layout)
        self.lytBotoesCentro = self._resolve(components, "lytBotoesCentro", "Layout", Layout)
        self.btnEmbaralhar = self._resolve(components, "btnEmbaralhar", "Button", Button)
        self.btnAnterior = self._resolve(components, "btnAnterior", "Button", Button)
        self.btnReproduzir = self._resolve(components, "btnReproduzir", "Button", Button)
        self.btnProxima = self._resolve(components, "btnProxima", "Button", Button)
        self.btnRepetir = self._resolve(components, "btnRepetir", "Button", Button)
        self.lytNavegacao = self._resolve(components, "lytNavegacao", "Layout", Layout)
        self.btnAgora = self._resolve(components, "btnAgora", "Button", Button)
        self.btnFila = self._resolve(components, "btnFila", "Button", Button)
        self.divNavegacao = self._resolve(components, "divNavegacao", "Divider", Divider)
        self.btnAbrirArquivos = self._resolve(components, "btnAbrirArquivos", "Button", Button)
        self.pnlAgora = self._resolve(components, "pnlAgora", "Panel", Panel)
        self.vpVideo = self._resolve(components, "vpVideo", "VideoPlayer", VideoPlayer)
        self.pnlFila = self._resolve(components, "pnlFila", "Panel", Panel)
        self.lblFila = self._resolve(components, "lblFila", "Label", Label)
        self.lytFilaBotoes = self._resolve(components, "lytFilaBotoes", "Layout", Layout)
        self.btnAdicionar = self._resolve(components, "btnAdicionar", "Button", Button)
        self.btnLimparFila = self._resolve(components, "btnLimparFila", "Button", Button)
        self.lvFila = self._resolve(components, "lvFila", "ListView", ListView)
        self.dlgAbrir = self._resolve(components, "dlgAbrir", "OpenDialog", OpenDialog)
        self.tmrPosicao = self._resolve(components, "tmrPosicao", "Timer", Timer)

    def find_button(self, name):
        """O Button deste form com o nome (como o FindComponent)."""
        if name == "btnMudo":
            return self.btnMudo
        if name == "btnEmbaralhar":
            return self.btnEmbaralhar
        if name == "btnAnterior":
            return self.btnAnterior
        if name == "btnReproduzir":
            return self.btnReproduzir
        if name == "btnProxima":
            return self.btnProxima
        if name == "btnRepetir":
            return self.btnRepetir
        if name == "btnAgora":
            return self.btnAgora
        if name == "btnFila":
            return self.btnFila
        if name == "btnAbrirArquivos":
            return self.btnAbrirArquivos
        if name == "btnAdicionar":
            return self.btnAdicionar
        if name == "btnLimparFila":
            return self.btnLimparFila
        return None

    def sender_as_button(self, event):
        """O Button que disparou o evento."""
        return self.find_button(event["sender"]["name"])

    def find_divider(self, name):
        """O Divider deste form com o nome (como o FindComponent)."""
        if name == "divNavegacao":
            return self.divNavegacao
        return None

    def sender_as_divider(self, event):
        """O Divider que disparou o evento."""
        return self.find_divider(event["sender"]["name"])

    def find_icon(self, name):
        """O Icon deste form com o nome (como o FindComponent)."""
        if name == "icoCapa":
            return self.icoCapa
        return None

    def sender_as_icon(self, event):
        """O Icon que disparou o evento."""
        return self.find_icon(event["sender"]["name"])

    def find_label(self, name):
        """O Label deste form com o nome (como o FindComponent)."""
        if name == "lblTempo":
            return self.lblTempo
        if name == "lblDuracao":
            return self.lblDuracao
        if name == "lblTitulo":
            return self.lblTitulo
        if name == "lblDetalhe":
            return self.lblDetalhe
        if name == "lblFila":
            return self.lblFila
        return None

    def sender_as_label(self, event):
        """O Label que disparou o evento."""
        return self.find_label(event["sender"]["name"])

    def find_layout(self, name):
        """O Layout deste form com o nome (como o FindComponent)."""
        if name == "lytBarra":
            return self.lytBarra
        if name == "lytInfo":
            return self.lytInfo
        if name == "lytTextos":
            return self.lytTextos
        if name == "lytVolume":
            return self.lytVolume
        if name == "lytBotoes":
            return self.lytBotoes
        if name == "lytBotoesCentro":
            return self.lytBotoesCentro
        if name == "lytNavegacao":
            return self.lytNavegacao
        if name == "lytFilaBotoes":
            return self.lytFilaBotoes
        return None

    def sender_as_layout(self, event):
        """O Layout que disparou o evento."""
        return self.find_layout(event["sender"]["name"])

    def find_list_view(self, name):
        """O ListView deste form com o nome (como o FindComponent)."""
        if name == "lvFila":
            return self.lvFila
        return None

    def sender_as_list_view(self, event):
        """O ListView que disparou o evento."""
        return self.find_list_view(event["sender"]["name"])

    def find_main_menu(self, name):
        """O MainMenu deste form com o nome (como o FindComponent)."""
        if name == "mnuPrincipal":
            return self.mnuPrincipal
        return None

    def sender_as_main_menu(self, event):
        """O MainMenu que disparou o evento."""
        return self.find_main_menu(event["sender"]["name"])

    def find_menu_item(self, name):
        """O MenuItem deste form com o nome (como o FindComponent)."""
        if name == "mnuArquivo":
            return self.mnuArquivo
        if name == "mnuAbrir":
            return self.mnuAbrir
        if name == "mnuLimparFila":
            return self.mnuLimparFila
        if name == "mnuSep1":
            return self.mnuSep1
        if name == "mnuSair":
            return self.mnuSair
        if name == "mnuReproduzir":
            return self.mnuReproduzir
        if name == "mnuReproduzirPausar":
            return self.mnuReproduzirPausar
        if name == "mnuParar":
            return self.mnuParar
        if name == "mnuAnterior":
            return self.mnuAnterior
        if name == "mnuProxima":
            return self.mnuProxima
        if name == "mnuSep2":
            return self.mnuSep2
        if name == "mnuEmbaralhar":
            return self.mnuEmbaralhar
        if name == "mnuRepetir":
            return self.mnuRepetir
        if name == "mnuExibir":
            return self.mnuExibir
        if name == "mnuAgora":
            return self.mnuAgora
        if name == "mnuFila":
            return self.mnuFila
        return None

    def sender_as_menu_item(self, event):
        """O MenuItem que disparou o evento."""
        return self.find_menu_item(event["sender"]["name"])

    def find_open_dialog(self, name):
        """O OpenDialog deste form com o nome (como o FindComponent)."""
        if name == "dlgAbrir":
            return self.dlgAbrir
        return None

    def sender_as_open_dialog(self, event):
        """O OpenDialog que disparou o evento."""
        return self.find_open_dialog(event["sender"]["name"])

    def find_panel(self, name):
        """O Panel deste form com o nome (como o FindComponent)."""
        if name == "pnlControles":
            return self.pnlControles
        if name == "pnlAgora":
            return self.pnlAgora
        if name == "pnlFila":
            return self.pnlFila
        return None

    def sender_as_panel(self, event):
        """O Panel que disparou o evento."""
        return self.find_panel(event["sender"]["name"])

    def find_seek_bar(self, name):
        """O SeekBar deste form com o nome (como o FindComponent)."""
        if name == "sbPosicao":
            return self.sbPosicao
        return None

    def sender_as_seek_bar(self, event):
        """O SeekBar que disparou o evento."""
        return self.find_seek_bar(event["sender"]["name"])

    def find_timer(self, name):
        """O Timer deste form com o nome (como o FindComponent)."""
        if name == "tmrPosicao":
            return self.tmrPosicao
        return None

    def sender_as_timer(self, event):
        """O Timer que disparou o evento."""
        return self.find_timer(event["sender"]["name"])

    def find_track_bar(self, name):
        """O TrackBar deste form com o nome (como o FindComponent)."""
        if name == "tbVolume":
            return self.tbVolume
        return None

    def sender_as_track_bar(self, event):
        """O TrackBar que disparou o evento."""
        return self.find_track_bar(event["sender"]["name"])

    def find_video_player(self, name):
        """O VideoPlayer deste form com o nome (como o FindComponent)."""
        if name == "vpVideo":
            return self.vpVideo
        return None

    def sender_as_video_player(self, event):
        """O VideoPlayer que disparou o evento."""
        return self.find_video_player(event["sender"]["name"])


# A janela de frmPlayer cujo evento está rodando (a primeira, fora dos handlers).
frmPlayer = CurrentForm("frmPlayer")
register_form_class("frmPlayer", FrmPlayerComponents, frmPlayer._activate)
