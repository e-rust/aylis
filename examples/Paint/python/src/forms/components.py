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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def set_focus(self):
        self.aylis.send("focus", target=self.id)


class CheckBox(AylisComponent):
    """Um CheckBox de um form."""

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

    @property
    def checked(self):
        return self._get("checked")

    @checked.setter
    def checked(self, value):
        self._set("checked", value)

    @property
    def box_size(self):
        return self._get("box_size")

    @box_size.setter
    def box_size(self, value):
        self._set("box_size", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def set_focus(self):
        self.aylis.send("focus", target=self.id)


class Circle(AylisComponent):
    """Um Circle de um form."""

    @property
    def fill(self):
        return self._get("fill")

    @fill.setter
    def fill(self, value):
        self._set("fill", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class ComboBox(AylisComponent):
    """Um ComboBox de um form."""

    @property
    def items(self):
        return self._get("items")

    @items.setter
    def items(self, value):
        self._set("items", value)

    @property
    def values(self):
        return self._get("values")

    @values.setter
    def values(self, value):
        self._set("values", value)

    @property
    def item_index(self):
        return self._get("item_index")

    @item_index.setter
    def item_index(self, value):
        self._set("item_index", value)

    @property
    def h_align(self):
        return self._get("h_align")

    @h_align.setter
    def h_align(self, value):
        self._set("h_align", value)

    @property
    def trimming(self):
        return self._get("trimming")

    @trimming.setter
    def trimming(self, value):
        self._set("trimming", value)

    @property
    def arrow(self):
        return self._get("arrow")

    @arrow.setter
    def arrow(self, value):
        self._set("arrow", value)

    @property
    def sensitive(self):
        return self._get("sensitive")

    @sensitive.setter
    def sensitive(self, value):
        self._set("sensitive", value)

    @property
    def value(self):
        return self._get("value")

    @value.setter
    def value(self, value):
        self._set("value", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def move_to_monitor(self, a1):
        self.aylis.send("invoke", target=self.id, method="move_to_monitor", args=[a1])

    def focused(self):
        return self.aylis.call("focused", target=self.id)

    def focus_next(self):
        self.aylis.send("focus_next", target=self.id)

    def focus_prev(self):
        self.aylis.send("focus_prev", target=self.id)


class GridLayout(AylisComponent):
    """Um GridLayout de um form."""

    @property
    def columns(self):
        return self._get("columns")

    @columns.setter
    def columns(self, value):
        self._set("columns", value)

    @property
    def min_column_width(self):
        return self._get("min_column_width")

    @min_column_width.setter
    def min_column_width(self, value):
        self._set("min_column_width", value)

    @property
    def gap(self):
        return self._get("gap")

    @gap.setter
    def gap(self, value):
        self._set("gap", value)

    @property
    def row_height(self):
        return self._get("row_height")

    @row_height.setter
    def row_height(self, value):
        self._set("row_height", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


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


class PaintCanvas(AylisComponent):
    """Um PaintCanvas de um form."""

    @property
    def tool(self):
        return self._get("tool")

    @tool.setter
    def tool(self, value):
        self._set("tool", value)

    @property
    def color(self):
        return self._get("color")

    @color.setter
    def color(self, value):
        self._set("color", value)

    @property
    def back_color(self):
        return self._get("back_color")

    @back_color.setter
    def back_color(self, value):
        self._set("back_color", value)

    @property
    def brush_size(self):
        return self._get("brush_size")

    @brush_size.setter
    def brush_size(self, value):
        self._set("brush_size", value)

    @property
    def filled(self):
        return self._get("filled")

    @filled.setter
    def filled(self, value):
        self._set("filled", value)

    @property
    def zoom(self):
        return self._get("zoom")

    @zoom.setter
    def zoom(self, value):
        self._set("zoom", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def undo(self):
        return self.aylis.call("invoke", target=self.id, method="undo", args=[])

    def redo(self):
        return self.aylis.call("invoke", target=self.id, method="redo", args=[])

    def can_undo(self):
        return self.aylis.call("invoke", target=self.id, method="can_undo", args=[])

    def can_redo(self):
        return self.aylis.call("invoke", target=self.id, method="can_redo", args=[])

    def size(self):
        return self.aylis.call("invoke", target=self.id, method="size", args=[])

    def checksum(self):
        return self.aylis.call("invoke", target=self.id, method="checksum", args=[])

    def clear(self):
        return self.aylis.call("invoke", target=self.id, method="clear", args=[])

    def resize(self, width, height):
        return self.aylis.call("invoke", target=self.id, method="resize", args=[width, height])

    def new(self, width, height):
        return self.aylis.call("invoke", target=self.id, method="new", args=[width, height])

    def open(self, path):
        return self.aylis.call("invoke", target=self.id, method="open", args=[path])

    def save(self, path):
        return self.aylis.call("invoke", target=self.id, method="save", args=[path])

    def select_all(self):
        return self.aylis.call("invoke", target=self.id, method="select_all", args=[])

    def has_selection(self):
        return self.aylis.call("invoke", target=self.id, method="has_selection", args=[])

    def copy(self):
        return self.aylis.call("invoke", target=self.id, method="copy", args=[])

    def cut(self):
        return self.aylis.call("invoke", target=self.id, method="cut", args=[])

    def paste(self):
        return self.aylis.call("invoke", target=self.id, method="paste", args=[])

    def delete(self):
        return self.aylis.call("invoke", target=self.id, method="delete", args=[])

    def deselect(self):
        return self.aylis.call("invoke", target=self.id, method="deselect", args=[])

    def set_focus(self):
        self.aylis.send("focus", target=self.id)


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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class SaveDialog(AylisComponent):
    """Um SaveDialog de um form."""

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


class ScrollBox(AylisComponent):
    """Um ScrollBox de um form."""

    @property
    def horizontal(self):
        return self._get("horizontal")

    @horizontal.setter
    def horizontal(self, value):
        self._set("horizontal", value)

    @property
    def pull_to_refresh(self):
        return self._get("pull_to_refresh")

    @pull_to_refresh.setter
    def pull_to_refresh(self, value):
        self._set("pull_to_refresh", value)

    @property
    def refreshing(self):
        return self._get("refreshing")

    @refreshing.setter
    def refreshing(self, value):
        self._set("refreshing", value)

    @property
    def wheel_step(self):
        return self._get("wheel_step")

    @wheel_step.setter
    def wheel_step(self, value):
        self._set("wheel_step", value)

    @property
    def smooth(self):
        return self._get("smooth")

    @smooth.setter
    def smooth(self, value):
        self._set("smooth", value)

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
    def scroll_position(self):
        return self._get("scroll_position")

    @scroll_position.setter
    def scroll_position(self, value):
        self._set("scroll_position", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)

    def scroll_to(self, a1):
        self.aylis.send("invoke", target=self.id, method="scroll_to", args=[a1.id])

    def visible_components(self, a1, a2):
        return self.aylis.call("invoke", target=self.id, method="visible_components", args=[a1.id, a2])


class StatusBar(AylisComponent):
    """Um StatusBar de um form."""

    @property
    def text(self):
        return self._get("text")

    @text.setter
    def text(self, value):
        self._set("text", value)

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
    def popup_menu(self):
        return self._get("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value):
        self._set("popup_menu", value)


class FrmPaintComponents(Form):
    """O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância)."""

    def bind(self, components):
        self.lytTopo = self._resolve(components, "lytTopo", "Layout", Layout)
        self.mnuPrincipal = self._resolve(components, "mnuPrincipal", "MainMenu", MainMenu)
        self.mnuArquivo = self._resolve(components, "mnuArquivo", "MenuItem", MenuItem)
        self.mnuNovo = self._resolve(components, "mnuNovo", "MenuItem", MenuItem)
        self.mnuAbrir = self._resolve(components, "mnuAbrir", "MenuItem", MenuItem)
        self.mnuSalvar = self._resolve(components, "mnuSalvar", "MenuItem", MenuItem)
        self.mnuSalvarComo = self._resolve(components, "mnuSalvarComo", "MenuItem", MenuItem)
        self.mnuSep1 = self._resolve(components, "mnuSep1", "MenuItem", MenuItem)
        self.mnuSair = self._resolve(components, "mnuSair", "MenuItem", MenuItem)
        self.mnuEditar = self._resolve(components, "mnuEditar", "MenuItem", MenuItem)
        self.mnuDesfazer = self._resolve(components, "mnuDesfazer", "MenuItem", MenuItem)
        self.mnuRefazer = self._resolve(components, "mnuRefazer", "MenuItem", MenuItem)
        self.mnuSep2 = self._resolve(components, "mnuSep2", "MenuItem", MenuItem)
        self.mnuRecortar = self._resolve(components, "mnuRecortar", "MenuItem", MenuItem)
        self.mnuCopiar = self._resolve(components, "mnuCopiar", "MenuItem", MenuItem)
        self.mnuColar = self._resolve(components, "mnuColar", "MenuItem", MenuItem)
        self.mnuExcluir = self._resolve(components, "mnuExcluir", "MenuItem", MenuItem)
        self.mnuSelecionarTudo = self._resolve(components, "mnuSelecionarTudo", "MenuItem", MenuItem)
        self.mnuSep3 = self._resolve(components, "mnuSep3", "MenuItem", MenuItem)
        self.mnuLimpar = self._resolve(components, "mnuLimpar", "MenuItem", MenuItem)
        self.mnuExibir = self._resolve(components, "mnuExibir", "MenuItem", MenuItem)
        self.mnuAmpliar = self._resolve(components, "mnuAmpliar", "MenuItem", MenuItem)
        self.mnuReduzir = self._resolve(components, "mnuReduzir", "MenuItem", MenuItem)
        self.mnuTamanhoReal = self._resolve(components, "mnuTamanhoReal", "MenuItem", MenuItem)
        self.divTopo = self._resolve(components, "divTopo", "Divider", Divider)
        self.btnSalvar = self._resolve(components, "btnSalvar", "Button", Button)
        self.btnDesfazer = self._resolve(components, "btnDesfazer", "Button", Button)
        self.btnRefazer = self._resolve(components, "btnRefazer", "Button", Button)
        self.pnlFaixa = self._resolve(components, "pnlFaixa", "Panel", Panel)
        self.lytSelecao = self._resolve(components, "lytSelecao", "Layout", Layout)
        self.lblSelecaoGrupo = self._resolve(components, "lblSelecaoGrupo", "Label", Label)
        self.btnSelecionar = self._resolve(components, "btnSelecionar", "Button", Button)
        self.divSelecao = self._resolve(components, "divSelecao", "Divider", Divider)
        self.lytFerramentas = self._resolve(components, "lytFerramentas", "Layout", Layout)
        self.lblFerramentasGrupo = self._resolve(components, "lblFerramentasGrupo", "Label", Label)
        self.grdFerramentas = self._resolve(components, "grdFerramentas", "GridLayout", GridLayout)
        self.btnLapis = self._resolve(components, "btnLapis", "Button", Button)
        self.btnBalde = self._resolve(components, "btnBalde", "Button", Button)
        self.btnBorracha = self._resolve(components, "btnBorracha", "Button", Button)
        self.btnContaGotas = self._resolve(components, "btnContaGotas", "Button", Button)
        self.divFerramentas = self._resolve(components, "divFerramentas", "Divider", Divider)
        self.lytPinceis = self._resolve(components, "lytPinceis", "Layout", Layout)
        self.lblPinceis = self._resolve(components, "lblPinceis", "Label", Label)
        self.btnPincel = self._resolve(components, "btnPincel", "Button", Button)
        self.divPinceis = self._resolve(components, "divPinceis", "Divider", Divider)
        self.lytFormas = self._resolve(components, "lytFormas", "Layout", Layout)
        self.lblFormas = self._resolve(components, "lblFormas", "Label", Label)
        self.chkPreencher = self._resolve(components, "chkPreencher", "CheckBox", CheckBox)
        self.grdFormas = self._resolve(components, "grdFormas", "GridLayout", GridLayout)
        self.btnLinha = self._resolve(components, "btnLinha", "Button", Button)
        self.btnRetangulo = self._resolve(components, "btnRetangulo", "Button", Button)
        self.btnElipse = self._resolve(components, "btnElipse", "Button", Button)
        self.divFormas = self._resolve(components, "divFormas", "Divider", Divider)
        self.lytTamanho = self._resolve(components, "lytTamanho", "Layout", Layout)
        self.lblTamanhoGrupo = self._resolve(components, "lblTamanhoGrupo", "Label", Label)
        self.cbEspessura = self._resolve(components, "cbEspessura", "ComboBox", ComboBox)
        self.divTamanho = self._resolve(components, "divTamanho", "Divider", Divider)
        self.lytCores = self._resolve(components, "lytCores", "Layout", Layout)
        self.lblCores = self._resolve(components, "lblCores", "Label", Label)
        self.btnCor1 = self._resolve(components, "btnCor1", "Circle", Circle)
        self.btnCor2 = self._resolve(components, "btnCor2", "Circle", Circle)
        self.grdPaleta = self._resolve(components, "grdPaleta", "GridLayout", GridLayout)
        self.btnPaleta01 = self._resolve(components, "btnPaleta01", "Circle", Circle)
        self.btnPaleta02 = self._resolve(components, "btnPaleta02", "Circle", Circle)
        self.btnPaleta03 = self._resolve(components, "btnPaleta03", "Circle", Circle)
        self.btnPaleta04 = self._resolve(components, "btnPaleta04", "Circle", Circle)
        self.btnPaleta05 = self._resolve(components, "btnPaleta05", "Circle", Circle)
        self.btnPaleta06 = self._resolve(components, "btnPaleta06", "Circle", Circle)
        self.btnPaleta07 = self._resolve(components, "btnPaleta07", "Circle", Circle)
        self.btnPaleta08 = self._resolve(components, "btnPaleta08", "Circle", Circle)
        self.btnPaleta09 = self._resolve(components, "btnPaleta09", "Circle", Circle)
        self.btnPaleta10 = self._resolve(components, "btnPaleta10", "Circle", Circle)
        self.btnPaleta11 = self._resolve(components, "btnPaleta11", "Circle", Circle)
        self.btnPaleta12 = self._resolve(components, "btnPaleta12", "Circle", Circle)
        self.btnPaleta13 = self._resolve(components, "btnPaleta13", "Circle", Circle)
        self.btnPaleta14 = self._resolve(components, "btnPaleta14", "Circle", Circle)
        self.btnPaleta15 = self._resolve(components, "btnPaleta15", "Circle", Circle)
        self.btnPaleta16 = self._resolve(components, "btnPaleta16", "Circle", Circle)
        self.btnPaleta17 = self._resolve(components, "btnPaleta17", "Circle", Circle)
        self.btnPaleta18 = self._resolve(components, "btnPaleta18", "Circle", Circle)
        self.btnPaleta19 = self._resolve(components, "btnPaleta19", "Circle", Circle)
        self.btnPaleta20 = self._resolve(components, "btnPaleta20", "Circle", Circle)
        self.stbStatus = self._resolve(components, "stbStatus", "StatusBar", StatusBar)
        self.lblPosicao = self._resolve(components, "lblPosicao", "Label", Label)
        self.lblTamanho = self._resolve(components, "lblTamanho", "Label", Label)
        self.lblZoom = self._resolve(components, "lblZoom", "Label", Label)
        self.sbxArea = self._resolve(components, "sbxArea", "ScrollBox", ScrollBox)
        self.pcDesenho = self._resolve(components, "pcDesenho", "PaintCanvas", PaintCanvas)
        self.dlgAbrir = self._resolve(components, "dlgAbrir", "OpenDialog", OpenDialog)
        self.dlgSalvar = self._resolve(components, "dlgSalvar", "SaveDialog", SaveDialog)

    def find_button(self, name):
        """O Button deste form com o nome (como o FindComponent)."""
        if name == "btnSalvar":
            return self.btnSalvar
        if name == "btnDesfazer":
            return self.btnDesfazer
        if name == "btnRefazer":
            return self.btnRefazer
        if name == "btnSelecionar":
            return self.btnSelecionar
        if name == "btnLapis":
            return self.btnLapis
        if name == "btnBalde":
            return self.btnBalde
        if name == "btnBorracha":
            return self.btnBorracha
        if name == "btnContaGotas":
            return self.btnContaGotas
        if name == "btnPincel":
            return self.btnPincel
        if name == "btnLinha":
            return self.btnLinha
        if name == "btnRetangulo":
            return self.btnRetangulo
        if name == "btnElipse":
            return self.btnElipse
        return None

    def sender_as_button(self, event):
        """O Button que disparou o evento."""
        return self.find_button(event["sender"]["name"])

    def find_check_box(self, name):
        """O CheckBox deste form com o nome (como o FindComponent)."""
        if name == "chkPreencher":
            return self.chkPreencher
        return None

    def sender_as_check_box(self, event):
        """O CheckBox que disparou o evento."""
        return self.find_check_box(event["sender"]["name"])

    def find_circle(self, name):
        """O Circle deste form com o nome (como o FindComponent)."""
        if name == "btnCor1":
            return self.btnCor1
        if name == "btnCor2":
            return self.btnCor2
        if name == "btnPaleta01":
            return self.btnPaleta01
        if name == "btnPaleta02":
            return self.btnPaleta02
        if name == "btnPaleta03":
            return self.btnPaleta03
        if name == "btnPaleta04":
            return self.btnPaleta04
        if name == "btnPaleta05":
            return self.btnPaleta05
        if name == "btnPaleta06":
            return self.btnPaleta06
        if name == "btnPaleta07":
            return self.btnPaleta07
        if name == "btnPaleta08":
            return self.btnPaleta08
        if name == "btnPaleta09":
            return self.btnPaleta09
        if name == "btnPaleta10":
            return self.btnPaleta10
        if name == "btnPaleta11":
            return self.btnPaleta11
        if name == "btnPaleta12":
            return self.btnPaleta12
        if name == "btnPaleta13":
            return self.btnPaleta13
        if name == "btnPaleta14":
            return self.btnPaleta14
        if name == "btnPaleta15":
            return self.btnPaleta15
        if name == "btnPaleta16":
            return self.btnPaleta16
        if name == "btnPaleta17":
            return self.btnPaleta17
        if name == "btnPaleta18":
            return self.btnPaleta18
        if name == "btnPaleta19":
            return self.btnPaleta19
        if name == "btnPaleta20":
            return self.btnPaleta20
        return None

    def sender_as_circle(self, event):
        """O Circle que disparou o evento."""
        return self.find_circle(event["sender"]["name"])

    def find_combo_box(self, name):
        """O ComboBox deste form com o nome (como o FindComponent)."""
        if name == "cbEspessura":
            return self.cbEspessura
        return None

    def sender_as_combo_box(self, event):
        """O ComboBox que disparou o evento."""
        return self.find_combo_box(event["sender"]["name"])

    def find_divider(self, name):
        """O Divider deste form com o nome (como o FindComponent)."""
        if name == "divTopo":
            return self.divTopo
        if name == "divSelecao":
            return self.divSelecao
        if name == "divFerramentas":
            return self.divFerramentas
        if name == "divPinceis":
            return self.divPinceis
        if name == "divFormas":
            return self.divFormas
        if name == "divTamanho":
            return self.divTamanho
        return None

    def sender_as_divider(self, event):
        """O Divider que disparou o evento."""
        return self.find_divider(event["sender"]["name"])

    def find_grid_layout(self, name):
        """O GridLayout deste form com o nome (como o FindComponent)."""
        if name == "grdFerramentas":
            return self.grdFerramentas
        if name == "grdFormas":
            return self.grdFormas
        if name == "grdPaleta":
            return self.grdPaleta
        return None

    def sender_as_grid_layout(self, event):
        """O GridLayout que disparou o evento."""
        return self.find_grid_layout(event["sender"]["name"])

    def find_label(self, name):
        """O Label deste form com o nome (como o FindComponent)."""
        if name == "lblSelecaoGrupo":
            return self.lblSelecaoGrupo
        if name == "lblFerramentasGrupo":
            return self.lblFerramentasGrupo
        if name == "lblPinceis":
            return self.lblPinceis
        if name == "lblFormas":
            return self.lblFormas
        if name == "lblTamanhoGrupo":
            return self.lblTamanhoGrupo
        if name == "lblCores":
            return self.lblCores
        if name == "lblPosicao":
            return self.lblPosicao
        if name == "lblTamanho":
            return self.lblTamanho
        if name == "lblZoom":
            return self.lblZoom
        return None

    def sender_as_label(self, event):
        """O Label que disparou o evento."""
        return self.find_label(event["sender"]["name"])

    def find_layout(self, name):
        """O Layout deste form com o nome (como o FindComponent)."""
        if name == "lytTopo":
            return self.lytTopo
        if name == "lytSelecao":
            return self.lytSelecao
        if name == "lytFerramentas":
            return self.lytFerramentas
        if name == "lytPinceis":
            return self.lytPinceis
        if name == "lytFormas":
            return self.lytFormas
        if name == "lytTamanho":
            return self.lytTamanho
        if name == "lytCores":
            return self.lytCores
        return None

    def sender_as_layout(self, event):
        """O Layout que disparou o evento."""
        return self.find_layout(event["sender"]["name"])

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
        if name == "mnuNovo":
            return self.mnuNovo
        if name == "mnuAbrir":
            return self.mnuAbrir
        if name == "mnuSalvar":
            return self.mnuSalvar
        if name == "mnuSalvarComo":
            return self.mnuSalvarComo
        if name == "mnuSep1":
            return self.mnuSep1
        if name == "mnuSair":
            return self.mnuSair
        if name == "mnuEditar":
            return self.mnuEditar
        if name == "mnuDesfazer":
            return self.mnuDesfazer
        if name == "mnuRefazer":
            return self.mnuRefazer
        if name == "mnuSep2":
            return self.mnuSep2
        if name == "mnuRecortar":
            return self.mnuRecortar
        if name == "mnuCopiar":
            return self.mnuCopiar
        if name == "mnuColar":
            return self.mnuColar
        if name == "mnuExcluir":
            return self.mnuExcluir
        if name == "mnuSelecionarTudo":
            return self.mnuSelecionarTudo
        if name == "mnuSep3":
            return self.mnuSep3
        if name == "mnuLimpar":
            return self.mnuLimpar
        if name == "mnuExibir":
            return self.mnuExibir
        if name == "mnuAmpliar":
            return self.mnuAmpliar
        if name == "mnuReduzir":
            return self.mnuReduzir
        if name == "mnuTamanhoReal":
            return self.mnuTamanhoReal
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

    def find_paint_canvas(self, name):
        """O PaintCanvas deste form com o nome (como o FindComponent)."""
        if name == "pcDesenho":
            return self.pcDesenho
        return None

    def sender_as_paint_canvas(self, event):
        """O PaintCanvas que disparou o evento."""
        return self.find_paint_canvas(event["sender"]["name"])

    def find_panel(self, name):
        """O Panel deste form com o nome (como o FindComponent)."""
        if name == "pnlFaixa":
            return self.pnlFaixa
        return None

    def sender_as_panel(self, event):
        """O Panel que disparou o evento."""
        return self.find_panel(event["sender"]["name"])

    def find_save_dialog(self, name):
        """O SaveDialog deste form com o nome (como o FindComponent)."""
        if name == "dlgSalvar":
            return self.dlgSalvar
        return None

    def sender_as_save_dialog(self, event):
        """O SaveDialog que disparou o evento."""
        return self.find_save_dialog(event["sender"]["name"])

    def find_scroll_box(self, name):
        """O ScrollBox deste form com o nome (como o FindComponent)."""
        if name == "sbxArea":
            return self.sbxArea
        return None

    def sender_as_scroll_box(self, event):
        """O ScrollBox que disparou o evento."""
        return self.find_scroll_box(event["sender"]["name"])

    def find_status_bar(self, name):
        """O StatusBar deste form com o nome (como o FindComponent)."""
        if name == "stbStatus":
            return self.stbStatus
        return None

    def sender_as_status_bar(self, event):
        """O StatusBar que disparou o evento."""
        return self.find_status_bar(event["sender"]["name"])


# A janela de frmPaint cujo evento está rodando (a primeira, fora dos handlers).
frmPaint = CurrentForm("frmPaint")
register_form_class("frmPaint", FrmPaintComponents, frmPaint._activate)
