// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import (
	"fmt"

	"aylis.dev/cliente/aylis"
)

var _ = fmt.Sprint

func asBool(v any) bool {
	b, _ := v.(bool)
	return b
}

func asNumber(v any) float64 {
	switch n := v.(type) {
	case float64:
		return n
	case int:
		return float64(n)
	case int64:
		return float64(n)
	}
	return 0
}

func asText(v any) string {
	if t, ok := v.(string); ok {
		return t
	}
	if v == nil {
		return ""
	}
	return fmt.Sprint(v)
}

func askCmd(app *aylis.Aylis, cmd string, fields aylis.M) any {
	v, err := app.Call(cmd, fields)
	if err != nil {
		aylis.Log(cmd+":", err)
	}
	return v
}

// aylisButton é um Button de um form.
type aylisButton struct{ aylis.Component }

func (c aylisButton) Text() string { return asText(c.Get("text")) }
func (c aylisButton) SetText(v string) { c.Set("text", v) }

func (c aylisButton) Icon() string { return asText(c.Get("icon")) }
func (c aylisButton) SetIcon(v string) { c.Set("icon", v) }

func (c aylisButton) CheckedIcon() string { return asText(c.Get("checked_icon")) }
func (c aylisButton) SetCheckedIcon(v string) { c.Set("checked_icon", v) }

func (c aylisButton) IconPosition() string { return asText(c.Get("icon_position")) }
func (c aylisButton) SetIconPosition(v string) { c.Set("icon_position", v) }

func (c aylisButton) IconSize() float64 { return asNumber(c.Get("icon_size")) }
func (c aylisButton) SetIconSize(v float64) { c.Set("icon_size", v) }

func (c aylisButton) Checkable() bool { return asBool(c.Get("checkable")) }
func (c aylisButton) SetCheckable(v bool) { c.Set("checkable", v) }

func (c aylisButton) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisButton) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisButton) ModalResult() string { return asText(c.Get("modal_result")) }
func (c aylisButton) SetModalResult(v string) { c.Set("modal_result", v) }

func (c aylisButton) RepeatEnter() bool { return asBool(c.Get("repeat_enter")) }
func (c aylisButton) SetRepeatEnter(v bool) { c.Set("repeat_enter", v) }

func (c aylisButton) DefaultButton() bool { return asBool(c.Get("default_button")) }
func (c aylisButton) SetDefaultButton(v bool) { c.Set("default_button", v) }

func (c aylisButton) CancelButton() bool { return asBool(c.Get("cancel_button")) }
func (c aylisButton) SetCancelButton(v bool) { c.Set("cancel_button", v) }

func (c aylisButton) HAlign() string { return asText(c.Get("h_align")) }
func (c aylisButton) SetHAlign(v string) { c.Set("h_align", v) }

func (c aylisButton) GroupIndex() int64 { return int64(asNumber(c.Get("group_index"))) }
func (c aylisButton) SetGroupIndex(v int64) { c.Set("group_index", v) }

func (c aylisButton) Radius() string { return asText(c.Get("radius")) }
func (c aylisButton) SetRadius(v string) { c.Set("radius", v) }

func (c aylisButton) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisButton) SetLeft(v float64) { c.Set("left", v) }

func (c aylisButton) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisButton) SetTop(v float64) { c.Set("top", v) }

func (c aylisButton) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisButton) SetWidth(v float64) { c.Set("width", v) }

func (c aylisButton) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisButton) SetHeight(v float64) { c.Set("height", v) }

func (c aylisButton) Align() string { return asText(c.Get("align")) }
func (c aylisButton) SetAlign(v string) { c.Set("align", v) }

func (c aylisButton) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisButton) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisButton) Margins() string { return asText(c.Get("margins")) }
func (c aylisButton) SetMargins(v string) { c.Set("margins", v) }

func (c aylisButton) Padding() string { return asText(c.Get("padding")) }
func (c aylisButton) SetPadding(v string) { c.Set("padding", v) }

func (c aylisButton) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisButton) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisButton) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisButton) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisButton) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisButton) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisButton) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisButton) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisButton) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisButton) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisButton) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisButton) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisButton) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisButton) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisButton) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisButton) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisButton) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisButton) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisButton) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisButton) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisButton) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisButton) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisButton) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisButton) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisButton) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisButton) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisButton) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisButton) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisButton) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisButton) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisButton) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisButton) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisButton) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisButton) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisButton) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisButton) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisButton) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisButton) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisButton) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisButton) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisButton) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisButton) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisButton) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisButton) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisButton) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisButton) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisButton) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisButton) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisButton) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisButton) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisButton) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisButton) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisButton) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisButton) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisButton) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisButton) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisButton) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisButton) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisButton) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisButton) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisButton) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisButton) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisButton) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisButton) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisButton) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisButton) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisButton) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisButton) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisButton) Style() string { return asText(c.Get("style")) }
func (c aylisButton) SetStyle(v string) { c.Set("style", v) }

func (c aylisButton) Theme() string { return asText(c.Get("theme")) }
func (c aylisButton) SetTheme(v string) { c.Set("theme", v) }

func (c aylisButton) Fill() string { return asText(c.Get("fill")) }
func (c aylisButton) SetFill(v string) { c.Set("fill", v) }

func (c aylisButton) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisButton) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisButton) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisButton) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisButton) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisButton) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisButton) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisButton) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisButton) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisButton) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisButton) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisButton) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisButton) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisButton) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisButton) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisButton) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisButton) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisButton) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisButton) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisButton) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisButton) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisButton) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisButton) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisButton) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisButton) Hint() string { return asText(c.Get("hint")) }
func (c aylisButton) SetHint(v string) { c.Set("hint", v) }

func (c aylisButton) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisButton) SetTag(v int64) { c.Set("tag", v) }

func (c aylisButton) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisButton) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisButton) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisButton) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisButton) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisButton) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisButton) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisDivider é um Divider de um form.
type aylisDivider struct{ aylis.Component }

func (c aylisDivider) Vertical() bool { return asBool(c.Get("vertical")) }
func (c aylisDivider) SetVertical(v bool) { c.Set("vertical", v) }

func (c aylisDivider) Text() string { return asText(c.Get("text")) }
func (c aylisDivider) SetText(v string) { c.Set("text", v) }

func (c aylisDivider) Thickness() float64 { return asNumber(c.Get("thickness")) }
func (c aylisDivider) SetThickness(v float64) { c.Set("thickness", v) }

func (c aylisDivider) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDivider) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDivider) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDivider) SetTop(v float64) { c.Set("top", v) }

func (c aylisDivider) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDivider) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDivider) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDivider) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDivider) Align() string { return asText(c.Get("align")) }
func (c aylisDivider) SetAlign(v string) { c.Set("align", v) }

func (c aylisDivider) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisDivider) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisDivider) Margins() string { return asText(c.Get("margins")) }
func (c aylisDivider) SetMargins(v string) { c.Set("margins", v) }

func (c aylisDivider) Padding() string { return asText(c.Get("padding")) }
func (c aylisDivider) SetPadding(v string) { c.Set("padding", v) }

func (c aylisDivider) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDivider) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDivider) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDivider) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDivider) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDivider) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDivider) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDivider) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDivider) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisDivider) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisDivider) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisDivider) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisDivider) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDivider) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDivider) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDivider) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDivider) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDivider) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDivider) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDivider) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDivider) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDivider) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDivider) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDivider) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDivider) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisDivider) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisDivider) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDivider) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDivider) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDivider) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDivider) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisDivider) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisDivider) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDivider) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDivider) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDivider) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDivider) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDivider) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDivider) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDivider) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDivider) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDivider) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDivider) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDivider) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDivider) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDivider) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDivider) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDivider) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDivider) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDivider) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDivider) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDivider) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDivider) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisDivider) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisDivider) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDivider) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDivider) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDivider) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDivider) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisDivider) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisDivider) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisDivider) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisDivider) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisDivider) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisDivider) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisDivider) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisDivider) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisDivider) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisDivider) Style() string { return asText(c.Get("style")) }
func (c aylisDivider) SetStyle(v string) { c.Set("style", v) }

func (c aylisDivider) Theme() string { return asText(c.Get("theme")) }
func (c aylisDivider) SetTheme(v string) { c.Set("theme", v) }

func (c aylisDivider) Fill() string { return asText(c.Get("fill")) }
func (c aylisDivider) SetFill(v string) { c.Set("fill", v) }

func (c aylisDivider) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisDivider) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisDivider) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisDivider) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisDivider) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisDivider) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisDivider) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisDivider) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisDivider) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisDivider) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisDivider) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisDivider) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisDivider) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDivider) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDivider) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDivider) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDivider) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisDivider) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisDivider) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDivider) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDivider) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDivider) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDivider) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDivider) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDivider) Hint() string { return asText(c.Get("hint")) }
func (c aylisDivider) SetHint(v string) { c.Set("hint", v) }

func (c aylisDivider) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDivider) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDivider) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisDivider) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisDivider) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisDivider) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisDivider) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisDivider) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisForm é um Form de um form.
type aylisForm struct{ aylis.Form }

func (c aylisForm) Caption() string { return asText(c.Get("caption")) }
func (c aylisForm) SetCaption(v string) { c.Set("caption", v) }

func (c aylisForm) Position() string { return asText(c.Get("position")) }
func (c aylisForm) SetPosition(v string) { c.Set("position", v) }

func (c aylisForm) BorderStyle() string { return asText(c.Get("border_style")) }
func (c aylisForm) SetBorderStyle(v string) { c.Set("border_style", v) }

func (c aylisForm) WindowState() string { return asText(c.Get("window_state")) }
func (c aylisForm) SetWindowState(v string) { c.Set("window_state", v) }

func (c aylisForm) Transition() string { return asText(c.Get("transition")) }
func (c aylisForm) SetTransition(v string) { c.Set("transition", v) }

func (c aylisForm) Topmost() bool { return asBool(c.Get("topmost")) }
func (c aylisForm) SetTopmost(v bool) { c.Set("topmost", v) }

func (c aylisForm) ShowInTaskbar() bool { return asBool(c.Get("show_in_taskbar")) }
func (c aylisForm) SetShowInTaskbar(v bool) { c.Set("show_in_taskbar", v) }

func (c aylisForm) ActiveControl() string { return asText(c.Get("active_control")) }
func (c aylisForm) SetActiveControl(v string) { c.Set("active_control", v) }

func (c aylisForm) TabOrder() string { return asText(c.Get("tab_order")) }
func (c aylisForm) SetTabOrder(v string) { c.Set("tab_order", v) }

func (c aylisForm) TabKeys() string { return asText(c.Get("tab_keys")) }
func (c aylisForm) SetTabKeys(v string) { c.Set("tab_keys", v) }

func (c aylisForm) BackTabKeys() string { return asText(c.Get("back_tab_keys")) }
func (c aylisForm) SetBackTabKeys(v string) { c.Set("back_tab_keys", v) }

func (c aylisForm) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisForm) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisForm) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisForm) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisForm) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisForm) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisForm) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisForm) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisForm) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisForm) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisForm) KeyPreview() bool { return asBool(c.Get("key_preview")) }
func (c aylisForm) SetKeyPreview(v bool) { c.Set("key_preview", v) }

func (c aylisForm) Zoom() float64 { return asNumber(c.Get("zoom")) }
func (c aylisForm) SetZoom(v float64) { c.Set("zoom", v) }

func (c aylisForm) Monitor() int64 { return int64(asNumber(c.Get("monitor"))) }
func (c aylisForm) SetMonitor(v int64) { c.Set("monitor", v) }

func (c aylisForm) RememberPlacement() bool { return asBool(c.Get("remember_placement")) }
func (c aylisForm) SetRememberPlacement(v bool) { c.Set("remember_placement", v) }

func (c aylisForm) Gpu() string { return asText(c.Get("gpu")) }
func (c aylisForm) SetGpu(v string) { c.Set("gpu", v) }

func (c aylisForm) ActivateOnShow() bool { return asBool(c.Get("activate_on_show")) }
func (c aylisForm) SetActivateOnShow(v bool) { c.Set("activate_on_show", v) }

func (c aylisForm) CloseAction() string { return asText(c.Get("close_action")) }
func (c aylisForm) SetCloseAction(v string) { c.Set("close_action", v) }

func (c aylisForm) FrameRate() float64 { return asNumber(c.Get("frame_rate")) }
func (c aylisForm) SetFrameRate(v float64) { c.Set("frame_rate", v) }

func (c aylisForm) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisForm) SetLeft(v float64) { c.Set("left", v) }

func (c aylisForm) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisForm) SetTop(v float64) { c.Set("top", v) }

func (c aylisForm) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisForm) SetWidth(v float64) { c.Set("width", v) }

func (c aylisForm) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisForm) SetHeight(v float64) { c.Set("height", v) }

func (c aylisForm) Align() string { return asText(c.Get("align")) }
func (c aylisForm) SetAlign(v string) { c.Set("align", v) }

func (c aylisForm) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisForm) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisForm) Margins() string { return asText(c.Get("margins")) }
func (c aylisForm) SetMargins(v string) { c.Set("margins", v) }

func (c aylisForm) Padding() string { return asText(c.Get("padding")) }
func (c aylisForm) SetPadding(v string) { c.Set("padding", v) }

func (c aylisForm) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisForm) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisForm) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisForm) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisForm) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisForm) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisForm) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisForm) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisForm) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisForm) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisForm) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisForm) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisForm) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisForm) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisForm) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisForm) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisForm) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisForm) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisForm) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisForm) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisForm) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisForm) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisForm) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisForm) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisForm) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisForm) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisForm) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisForm) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisForm) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisForm) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisForm) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisForm) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisForm) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisForm) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisForm) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisForm) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisForm) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisForm) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisForm) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisForm) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisForm) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisForm) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisForm) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisForm) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisForm) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisForm) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisForm) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisForm) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisForm) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisForm) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisForm) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisForm) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisForm) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisForm) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisForm) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisForm) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisForm) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisForm) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisForm) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisForm) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisForm) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisForm) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisForm) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisForm) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisForm) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisForm) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisForm) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisForm) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisForm) Style() string { return asText(c.Get("style")) }
func (c aylisForm) SetStyle(v string) { c.Set("style", v) }

func (c aylisForm) Theme() string { return asText(c.Get("theme")) }
func (c aylisForm) SetTheme(v string) { c.Set("theme", v) }

func (c aylisForm) Fill() string { return asText(c.Get("fill")) }
func (c aylisForm) SetFill(v string) { c.Set("fill", v) }

func (c aylisForm) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisForm) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisForm) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisForm) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisForm) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisForm) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisForm) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisForm) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisForm) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisForm) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisForm) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisForm) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisForm) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisForm) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisForm) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisForm) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisForm) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisForm) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisForm) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisForm) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisForm) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisForm) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisForm) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisForm) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisForm) Hint() string { return asText(c.Get("hint")) }
func (c aylisForm) SetHint(v string) { c.Set("hint", v) }

func (c aylisForm) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisForm) SetTag(v int64) { c.Set("tag", v) }

func (c aylisForm) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisForm) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisForm) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisForm) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisForm) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisForm) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisForm) MoveToMonitor(monitor int64) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "move_to_monitor", "args": []any{monitor}}) }
func (c aylisForm) GpuAdapter() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapter", "args": []any{}})) }
func (c aylisForm) GpuAdapters() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapters", "args": []any{}})) }
func (c aylisForm) Focused() string { return asText(askCmd(c.App, "focused", aylis.M{"target": c.ID})) }
func (c aylisForm) FocusNext() { c.App.Send("focus_next", aylis.M{"target": c.ID}) }
func (c aylisForm) FocusPrev() { c.App.Send("focus_prev", aylis.M{"target": c.ID}) }

// aylisIcon é um Icon de um form.
type aylisIcon struct{ aylis.Component }

func (c aylisIcon) Icon() string { return asText(c.Get("icon")) }
func (c aylisIcon) SetIcon(v string) { c.Set("icon", v) }

func (c aylisIcon) Color() string { return asText(c.Get("color")) }
func (c aylisIcon) SetColor(v string) { c.Set("color", v) }

func (c aylisIcon) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisIcon) SetLeft(v float64) { c.Set("left", v) }

func (c aylisIcon) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisIcon) SetTop(v float64) { c.Set("top", v) }

func (c aylisIcon) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisIcon) SetWidth(v float64) { c.Set("width", v) }

func (c aylisIcon) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisIcon) SetHeight(v float64) { c.Set("height", v) }

func (c aylisIcon) Align() string { return asText(c.Get("align")) }
func (c aylisIcon) SetAlign(v string) { c.Set("align", v) }

func (c aylisIcon) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisIcon) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisIcon) Margins() string { return asText(c.Get("margins")) }
func (c aylisIcon) SetMargins(v string) { c.Set("margins", v) }

func (c aylisIcon) Padding() string { return asText(c.Get("padding")) }
func (c aylisIcon) SetPadding(v string) { c.Set("padding", v) }

func (c aylisIcon) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisIcon) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisIcon) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisIcon) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisIcon) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisIcon) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisIcon) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisIcon) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisIcon) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisIcon) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisIcon) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisIcon) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisIcon) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisIcon) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisIcon) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisIcon) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisIcon) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisIcon) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisIcon) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisIcon) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisIcon) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisIcon) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisIcon) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisIcon) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisIcon) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisIcon) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisIcon) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisIcon) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisIcon) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisIcon) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisIcon) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisIcon) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisIcon) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisIcon) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisIcon) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisIcon) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisIcon) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisIcon) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisIcon) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisIcon) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisIcon) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisIcon) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisIcon) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisIcon) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisIcon) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisIcon) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisIcon) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisIcon) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisIcon) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisIcon) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisIcon) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisIcon) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisIcon) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisIcon) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisIcon) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisIcon) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisIcon) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisIcon) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisIcon) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisIcon) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisIcon) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisIcon) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisIcon) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisIcon) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisIcon) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisIcon) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisIcon) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisIcon) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisIcon) Style() string { return asText(c.Get("style")) }
func (c aylisIcon) SetStyle(v string) { c.Set("style", v) }

func (c aylisIcon) Theme() string { return asText(c.Get("theme")) }
func (c aylisIcon) SetTheme(v string) { c.Set("theme", v) }

func (c aylisIcon) Fill() string { return asText(c.Get("fill")) }
func (c aylisIcon) SetFill(v string) { c.Set("fill", v) }

func (c aylisIcon) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisIcon) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisIcon) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisIcon) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisIcon) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisIcon) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisIcon) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisIcon) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisIcon) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisIcon) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisIcon) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisIcon) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisIcon) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisIcon) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisIcon) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisIcon) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisIcon) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisIcon) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisIcon) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisIcon) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisIcon) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisIcon) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisIcon) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisIcon) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisIcon) Hint() string { return asText(c.Get("hint")) }
func (c aylisIcon) SetHint(v string) { c.Set("hint", v) }

func (c aylisIcon) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisIcon) SetTag(v int64) { c.Set("tag", v) }

func (c aylisIcon) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisIcon) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisIcon) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisIcon) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisIcon) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisIcon) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisLabel é um Label de um form.
type aylisLabel struct{ aylis.Component }

func (c aylisLabel) Text() string { return asText(c.Get("text")) }
func (c aylisLabel) SetText(v string) { c.Set("text", v) }

func (c aylisLabel) HAlign() string { return asText(c.Get("h_align")) }
func (c aylisLabel) SetHAlign(v string) { c.Set("h_align", v) }

func (c aylisLabel) VAlign() string { return asText(c.Get("v_align")) }
func (c aylisLabel) SetVAlign(v string) { c.Set("v_align", v) }

func (c aylisLabel) Wrap() bool { return asBool(c.Get("wrap")) }
func (c aylisLabel) SetWrap(v bool) { c.Set("wrap", v) }

func (c aylisLabel) Trimming() string { return asText(c.Get("trimming")) }
func (c aylisLabel) SetTrimming(v string) { c.Set("trimming", v) }

func (c aylisLabel) Bold() bool { return asBool(c.Get("bold")) }
func (c aylisLabel) SetBold(v bool) { c.Set("bold", v) }

func (c aylisLabel) Color() string { return asText(c.Get("color")) }
func (c aylisLabel) SetColor(v string) { c.Set("color", v) }

func (c aylisLabel) Highlight() string { return asText(c.Get("highlight")) }
func (c aylisLabel) SetHighlight(v string) { c.Set("highlight", v) }

func (c aylisLabel) LetterSpacing() float64 { return asNumber(c.Get("letter_spacing")) }
func (c aylisLabel) SetLetterSpacing(v float64) { c.Set("letter_spacing", v) }

func (c aylisLabel) TextCase() string { return asText(c.Get("text_case")) }
func (c aylisLabel) SetTextCase(v string) { c.Set("text_case", v) }

func (c aylisLabel) Markup() bool { return asBool(c.Get("markup")) }
func (c aylisLabel) SetMarkup(v bool) { c.Set("markup", v) }

func (c aylisLabel) Outline() string { return asText(c.Get("outline")) }
func (c aylisLabel) SetOutline(v string) { c.Set("outline", v) }

func (c aylisLabel) OutlineWidth() float64 { return asNumber(c.Get("outline_width")) }
func (c aylisLabel) SetOutlineWidth(v float64) { c.Set("outline_width", v) }

func (c aylisLabel) LineHeight() float64 { return asNumber(c.Get("line_height")) }
func (c aylisLabel) SetLineHeight(v float64) { c.Set("line_height", v) }

func (c aylisLabel) TextShadow() string { return asText(c.Get("text_shadow")) }
func (c aylisLabel) SetTextShadow(v string) { c.Set("text_shadow", v) }

func (c aylisLabel) FocusControl() string { return asText(c.Get("focus_control")) }
func (c aylisLabel) SetFocusControl(v string) { c.Set("focus_control", v) }

func (c aylisLabel) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisLabel) SetLeft(v float64) { c.Set("left", v) }

func (c aylisLabel) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisLabel) SetTop(v float64) { c.Set("top", v) }

func (c aylisLabel) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisLabel) SetWidth(v float64) { c.Set("width", v) }

func (c aylisLabel) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisLabel) SetHeight(v float64) { c.Set("height", v) }

func (c aylisLabel) Align() string { return asText(c.Get("align")) }
func (c aylisLabel) SetAlign(v string) { c.Set("align", v) }

func (c aylisLabel) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisLabel) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisLabel) Margins() string { return asText(c.Get("margins")) }
func (c aylisLabel) SetMargins(v string) { c.Set("margins", v) }

func (c aylisLabel) Padding() string { return asText(c.Get("padding")) }
func (c aylisLabel) SetPadding(v string) { c.Set("padding", v) }

func (c aylisLabel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLabel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLabel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLabel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLabel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLabel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLabel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLabel) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisLabel) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisLabel) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisLabel) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisLabel) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisLabel) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisLabel) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisLabel) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisLabel) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisLabel) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisLabel) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisLabel) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisLabel) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisLabel) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisLabel) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisLabel) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisLabel) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisLabel) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisLabel) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisLabel) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisLabel) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisLabel) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisLabel) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisLabel) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisLabel) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisLabel) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisLabel) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisLabel) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisLabel) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisLabel) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisLabel) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisLabel) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisLabel) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisLabel) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisLabel) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisLabel) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisLabel) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisLabel) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisLabel) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisLabel) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisLabel) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisLabel) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisLabel) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisLabel) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisLabel) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisLabel) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisLabel) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisLabel) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisLabel) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisLabel) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisLabel) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisLabel) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisLabel) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisLabel) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisLabel) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisLabel) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisLabel) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisLabel) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisLabel) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisLabel) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisLabel) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisLabel) Style() string { return asText(c.Get("style")) }
func (c aylisLabel) SetStyle(v string) { c.Set("style", v) }

func (c aylisLabel) Theme() string { return asText(c.Get("theme")) }
func (c aylisLabel) SetTheme(v string) { c.Set("theme", v) }

func (c aylisLabel) Fill() string { return asText(c.Get("fill")) }
func (c aylisLabel) SetFill(v string) { c.Set("fill", v) }

func (c aylisLabel) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisLabel) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisLabel) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisLabel) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisLabel) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisLabel) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisLabel) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisLabel) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisLabel) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisLabel) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisLabel) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisLabel) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisLabel) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisLabel) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisLabel) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisLabel) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisLabel) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisLabel) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisLabel) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisLabel) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisLabel) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisLabel) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisLabel) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisLabel) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisLabel) Hint() string { return asText(c.Get("hint")) }
func (c aylisLabel) SetHint(v string) { c.Set("hint", v) }

func (c aylisLabel) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisLabel) SetTag(v int64) { c.Set("tag", v) }

func (c aylisLabel) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisLabel) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisLabel) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisLabel) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisLabel) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisLabel) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisLabel) MinContentWidth() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "min_content_width", "args": []any{}})) }
func (c aylisLabel) MaxContentWidth() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "max_content_width", "args": []any{}})) }

// aylisLayout é um Layout de um form.
type aylisLayout struct{ aylis.Component }

func (c aylisLayout) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisLayout) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisLayout) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisLayout) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisLayout) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisLayout) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisLayout) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisLayout) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisLayout) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisLayout) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisLayout) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisLayout) SetGap(v float64) { c.Set("gap", v) }

func (c aylisLayout) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisLayout) SetLeft(v float64) { c.Set("left", v) }

func (c aylisLayout) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisLayout) SetTop(v float64) { c.Set("top", v) }

func (c aylisLayout) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisLayout) SetWidth(v float64) { c.Set("width", v) }

func (c aylisLayout) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisLayout) SetHeight(v float64) { c.Set("height", v) }

func (c aylisLayout) Align() string { return asText(c.Get("align")) }
func (c aylisLayout) SetAlign(v string) { c.Set("align", v) }

func (c aylisLayout) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisLayout) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisLayout) Margins() string { return asText(c.Get("margins")) }
func (c aylisLayout) SetMargins(v string) { c.Set("margins", v) }

func (c aylisLayout) Padding() string { return asText(c.Get("padding")) }
func (c aylisLayout) SetPadding(v string) { c.Set("padding", v) }

func (c aylisLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisLayout) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisLayout) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisLayout) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisLayout) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisLayout) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisLayout) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisLayout) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisLayout) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisLayout) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisLayout) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisLayout) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisLayout) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisLayout) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisLayout) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisLayout) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisLayout) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisLayout) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisLayout) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisLayout) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisLayout) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisLayout) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisLayout) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisLayout) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisLayout) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisLayout) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisLayout) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisLayout) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisLayout) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisLayout) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisLayout) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisLayout) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisLayout) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisLayout) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisLayout) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisLayout) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisLayout) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisLayout) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisLayout) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisLayout) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisLayout) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisLayout) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisLayout) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisLayout) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisLayout) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisLayout) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisLayout) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisLayout) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisLayout) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisLayout) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisLayout) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisLayout) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisLayout) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisLayout) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisLayout) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisLayout) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisLayout) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisLayout) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisLayout) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisLayout) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisLayout) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisLayout) Style() string { return asText(c.Get("style")) }
func (c aylisLayout) SetStyle(v string) { c.Set("style", v) }

func (c aylisLayout) Theme() string { return asText(c.Get("theme")) }
func (c aylisLayout) SetTheme(v string) { c.Set("theme", v) }

func (c aylisLayout) Fill() string { return asText(c.Get("fill")) }
func (c aylisLayout) SetFill(v string) { c.Set("fill", v) }

func (c aylisLayout) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisLayout) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisLayout) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisLayout) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisLayout) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisLayout) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisLayout) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisLayout) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisLayout) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisLayout) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisLayout) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisLayout) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisLayout) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisLayout) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisLayout) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisLayout) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisLayout) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisLayout) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisLayout) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisLayout) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisLayout) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisLayout) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisLayout) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisLayout) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisLayout) Hint() string { return asText(c.Get("hint")) }
func (c aylisLayout) SetHint(v string) { c.Set("hint", v) }

func (c aylisLayout) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisLayout) SetTag(v int64) { c.Set("tag", v) }

func (c aylisLayout) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisLayout) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisLayout) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisLayout) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisLayout) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisLayout) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisListView é um ListView de um form.
type aylisListView struct{ aylis.Component }

func (c aylisListView) Items() string { return asText(c.Get("items")) }
func (c aylisListView) SetItems(v string) { c.Set("items", v) }

func (c aylisListView) ItemIndex() int64 { return int64(asNumber(c.Get("item_index"))) }
func (c aylisListView) SetItemIndex(v int64) { c.Set("item_index", v) }

func (c aylisListView) ItemHeight() float64 { return asNumber(c.Get("item_height")) }
func (c aylisListView) SetItemHeight(v float64) { c.Set("item_height", v) }

func (c aylisListView) Accessory() bool { return asBool(c.Get("accessory")) }
func (c aylisListView) SetAccessory(v bool) { c.Set("accessory", v) }

func (c aylisListView) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisListView) SetLeft(v float64) { c.Set("left", v) }

func (c aylisListView) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisListView) SetTop(v float64) { c.Set("top", v) }

func (c aylisListView) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisListView) SetWidth(v float64) { c.Set("width", v) }

func (c aylisListView) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisListView) SetHeight(v float64) { c.Set("height", v) }

func (c aylisListView) Align() string { return asText(c.Get("align")) }
func (c aylisListView) SetAlign(v string) { c.Set("align", v) }

func (c aylisListView) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisListView) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisListView) Margins() string { return asText(c.Get("margins")) }
func (c aylisListView) SetMargins(v string) { c.Set("margins", v) }

func (c aylisListView) Padding() string { return asText(c.Get("padding")) }
func (c aylisListView) SetPadding(v string) { c.Set("padding", v) }

func (c aylisListView) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisListView) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisListView) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisListView) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisListView) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisListView) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisListView) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisListView) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisListView) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisListView) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisListView) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisListView) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisListView) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisListView) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisListView) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisListView) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisListView) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisListView) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisListView) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisListView) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisListView) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisListView) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisListView) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisListView) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisListView) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisListView) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisListView) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisListView) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisListView) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisListView) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisListView) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisListView) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisListView) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisListView) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisListView) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisListView) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisListView) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisListView) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisListView) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisListView) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisListView) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisListView) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisListView) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisListView) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisListView) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisListView) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisListView) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisListView) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisListView) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisListView) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisListView) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisListView) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisListView) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisListView) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisListView) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisListView) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisListView) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisListView) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisListView) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisListView) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisListView) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisListView) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisListView) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisListView) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisListView) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisListView) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisListView) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisListView) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisListView) Style() string { return asText(c.Get("style")) }
func (c aylisListView) SetStyle(v string) { c.Set("style", v) }

func (c aylisListView) Theme() string { return asText(c.Get("theme")) }
func (c aylisListView) SetTheme(v string) { c.Set("theme", v) }

func (c aylisListView) Fill() string { return asText(c.Get("fill")) }
func (c aylisListView) SetFill(v string) { c.Set("fill", v) }

func (c aylisListView) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisListView) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisListView) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisListView) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisListView) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisListView) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisListView) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisListView) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisListView) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisListView) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisListView) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisListView) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisListView) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisListView) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisListView) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisListView) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisListView) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisListView) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisListView) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisListView) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisListView) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisListView) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisListView) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisListView) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisListView) Hint() string { return asText(c.Get("hint")) }
func (c aylisListView) SetHint(v string) { c.Set("hint", v) }

func (c aylisListView) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisListView) SetTag(v int64) { c.Set("tag", v) }

func (c aylisListView) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisListView) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisListView) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisListView) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisListView) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisListView) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisListView) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisMainMenu é um MainMenu de um form.
type aylisMainMenu struct{ aylis.Component }

func (c aylisMainMenu) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisMainMenu) SetLeft(v float64) { c.Set("left", v) }

func (c aylisMainMenu) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisMainMenu) SetTop(v float64) { c.Set("top", v) }

func (c aylisMainMenu) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisMainMenu) SetWidth(v float64) { c.Set("width", v) }

func (c aylisMainMenu) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisMainMenu) SetHeight(v float64) { c.Set("height", v) }

func (c aylisMainMenu) Align() string { return asText(c.Get("align")) }
func (c aylisMainMenu) SetAlign(v string) { c.Set("align", v) }

func (c aylisMainMenu) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisMainMenu) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisMainMenu) Margins() string { return asText(c.Get("margins")) }
func (c aylisMainMenu) SetMargins(v string) { c.Set("margins", v) }

func (c aylisMainMenu) Padding() string { return asText(c.Get("padding")) }
func (c aylisMainMenu) SetPadding(v string) { c.Set("padding", v) }

func (c aylisMainMenu) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisMainMenu) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisMainMenu) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisMainMenu) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisMainMenu) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisMainMenu) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisMainMenu) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisMainMenu) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisMainMenu) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisMainMenu) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisMainMenu) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisMainMenu) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisMainMenu) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisMainMenu) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisMainMenu) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisMainMenu) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisMainMenu) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisMainMenu) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisMainMenu) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisMainMenu) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisMainMenu) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisMainMenu) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisMainMenu) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisMainMenu) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisMainMenu) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisMainMenu) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisMainMenu) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisMainMenu) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisMainMenu) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisMainMenu) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisMainMenu) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisMainMenu) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisMainMenu) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisMainMenu) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisMainMenu) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisMainMenu) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisMainMenu) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisMainMenu) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisMainMenu) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisMainMenu) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisMainMenu) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisMainMenu) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisMainMenu) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisMainMenu) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisMainMenu) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisMainMenu) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisMainMenu) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisMainMenu) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisMainMenu) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisMainMenu) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisMainMenu) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisMainMenu) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisMainMenu) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisMainMenu) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisMainMenu) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisMainMenu) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisMainMenu) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisMainMenu) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisMainMenu) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisMainMenu) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisMainMenu) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisMainMenu) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisMainMenu) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisMainMenu) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisMainMenu) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisMainMenu) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisMainMenu) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisMainMenu) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisMainMenu) Style() string { return asText(c.Get("style")) }
func (c aylisMainMenu) SetStyle(v string) { c.Set("style", v) }

func (c aylisMainMenu) Theme() string { return asText(c.Get("theme")) }
func (c aylisMainMenu) SetTheme(v string) { c.Set("theme", v) }

func (c aylisMainMenu) Fill() string { return asText(c.Get("fill")) }
func (c aylisMainMenu) SetFill(v string) { c.Set("fill", v) }

func (c aylisMainMenu) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisMainMenu) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisMainMenu) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisMainMenu) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisMainMenu) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisMainMenu) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisMainMenu) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisMainMenu) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisMainMenu) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisMainMenu) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisMainMenu) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisMainMenu) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisMainMenu) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisMainMenu) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisMainMenu) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisMainMenu) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisMainMenu) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisMainMenu) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisMainMenu) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisMainMenu) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisMainMenu) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisMainMenu) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisMainMenu) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisMainMenu) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisMainMenu) Hint() string { return asText(c.Get("hint")) }
func (c aylisMainMenu) SetHint(v string) { c.Set("hint", v) }

func (c aylisMainMenu) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisMainMenu) SetTag(v int64) { c.Set("tag", v) }

func (c aylisMainMenu) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisMainMenu) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisMainMenu) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisMainMenu) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisMainMenu) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisMainMenu) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisMenuItem é um MenuItem de um form.
type aylisMenuItem struct{ aylis.Component }

func (c aylisMenuItem) Text() string { return asText(c.Get("text")) }
func (c aylisMenuItem) SetText(v string) { c.Set("text", v) }

func (c aylisMenuItem) Shortcut() string { return asText(c.Get("shortcut")) }
func (c aylisMenuItem) SetShortcut(v string) { c.Set("shortcut", v) }

func (c aylisMenuItem) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisMenuItem) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisMenuItem) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisMenuItem) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisMenuItem) AutoCheck() bool { return asBool(c.Get("auto_check")) }
func (c aylisMenuItem) SetAutoCheck(v bool) { c.Set("auto_check", v) }

func (c aylisMenuItem) Icon() string { return asText(c.Get("icon")) }
func (c aylisMenuItem) SetIcon(v string) { c.Set("icon", v) }

func (c aylisMenuItem) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisMenuItem) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisMenuItem) Danger() bool { return asBool(c.Get("danger")) }
func (c aylisMenuItem) SetDanger(v bool) { c.Set("danger", v) }

func (c aylisMenuItem) Header() bool { return asBool(c.Get("header")) }
func (c aylisMenuItem) SetHeader(v bool) { c.Set("header", v) }

// aylisOpenDialog é um OpenDialog de um form.
type aylisOpenDialog struct{ aylis.Component }

func (c aylisOpenDialog) Title() string { return asText(c.Get("title")) }
func (c aylisOpenDialog) SetTitle(v string) { c.Set("title", v) }

func (c aylisOpenDialog) Filter() string { return asText(c.Get("filter")) }
func (c aylisOpenDialog) SetFilter(v string) { c.Set("filter", v) }

func (c aylisOpenDialog) InitialDir() string { return asText(c.Get("initial_dir")) }
func (c aylisOpenDialog) SetInitialDir(v string) { c.Set("initial_dir", v) }

func (c aylisOpenDialog) FileName() string { return asText(c.Get("file_name")) }
func (c aylisOpenDialog) SetFileName(v string) { c.Set("file_name", v) }

func (c aylisOpenDialog) Multiple() bool { return asBool(c.Get("multiple")) }
func (c aylisOpenDialog) SetMultiple(v bool) { c.Set("multiple", v) }
func (c aylisOpenDialog) Execute() { c.App.Send("execute", aylis.M{"target": c.ID}) }

// aylisPanel é um Panel de um form.
type aylisPanel struct{ aylis.Component }

func (c aylisPanel) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisPanel) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisPanel) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisPanel) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisPanel) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisPanel) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisPanel) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisPanel) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisPanel) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisPanel) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisPanel) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisPanel) SetGap(v float64) { c.Set("gap", v) }

func (c aylisPanel) Fill() string { return asText(c.Get("fill")) }
func (c aylisPanel) SetFill(v string) { c.Set("fill", v) }

func (c aylisPanel) Fill2() string { return asText(c.Get("fill2")) }
func (c aylisPanel) SetFill2(v string) { c.Set("fill2", v) }

func (c aylisPanel) FillMid() string { return asText(c.Get("fill_mid")) }
func (c aylisPanel) SetFillMid(v string) { c.Set("fill_mid", v) }

func (c aylisPanel) FillMidAt() float64 { return asNumber(c.Get("fill_mid_at")) }
func (c aylisPanel) SetFillMidAt(v float64) { c.Set("fill_mid_at", v) }

func (c aylisPanel) GradientAngle() float64 { return asNumber(c.Get("gradient_angle")) }
func (c aylisPanel) SetGradientAngle(v float64) { c.Set("gradient_angle", v) }

func (c aylisPanel) GradientRadial() bool { return asBool(c.Get("gradient_radial")) }
func (c aylisPanel) SetGradientRadial(v bool) { c.Set("gradient_radial", v) }

func (c aylisPanel) Stroke() string { return asText(c.Get("stroke")) }
func (c aylisPanel) SetStroke(v string) { c.Set("stroke", v) }

func (c aylisPanel) StrokeWidth() float64 { return asNumber(c.Get("stroke_width")) }
func (c aylisPanel) SetStrokeWidth(v float64) { c.Set("stroke_width", v) }

func (c aylisPanel) Radius() string { return asText(c.Get("radius")) }
func (c aylisPanel) SetRadius(v string) { c.Set("radius", v) }

func (c aylisPanel) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisPanel) SetLeft(v float64) { c.Set("left", v) }

func (c aylisPanel) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisPanel) SetTop(v float64) { c.Set("top", v) }

func (c aylisPanel) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisPanel) SetWidth(v float64) { c.Set("width", v) }

func (c aylisPanel) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisPanel) SetHeight(v float64) { c.Set("height", v) }

func (c aylisPanel) Align() string { return asText(c.Get("align")) }
func (c aylisPanel) SetAlign(v string) { c.Set("align", v) }

func (c aylisPanel) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisPanel) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisPanel) Margins() string { return asText(c.Get("margins")) }
func (c aylisPanel) SetMargins(v string) { c.Set("margins", v) }

func (c aylisPanel) Padding() string { return asText(c.Get("padding")) }
func (c aylisPanel) SetPadding(v string) { c.Set("padding", v) }

func (c aylisPanel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisPanel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisPanel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisPanel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisPanel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisPanel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisPanel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisPanel) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisPanel) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisPanel) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisPanel) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisPanel) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisPanel) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisPanel) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisPanel) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisPanel) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisPanel) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisPanel) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisPanel) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisPanel) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisPanel) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisPanel) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisPanel) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisPanel) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisPanel) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisPanel) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisPanel) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisPanel) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisPanel) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisPanel) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisPanel) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisPanel) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisPanel) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisPanel) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisPanel) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisPanel) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisPanel) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisPanel) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisPanel) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisPanel) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisPanel) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisPanel) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisPanel) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisPanel) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisPanel) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisPanel) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisPanel) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisPanel) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisPanel) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisPanel) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisPanel) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisPanel) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisPanel) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisPanel) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisPanel) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisPanel) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisPanel) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisPanel) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisPanel) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisPanel) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisPanel) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisPanel) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisPanel) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisPanel) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisPanel) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisPanel) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisPanel) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisPanel) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisPanel) Style() string { return asText(c.Get("style")) }
func (c aylisPanel) SetStyle(v string) { c.Set("style", v) }

func (c aylisPanel) Theme() string { return asText(c.Get("theme")) }
func (c aylisPanel) SetTheme(v string) { c.Set("theme", v) }

func (c aylisPanel) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisPanel) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisPanel) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisPanel) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisPanel) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisPanel) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisPanel) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisPanel) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisPanel) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisPanel) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisPanel) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisPanel) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisPanel) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisPanel) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisPanel) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisPanel) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisPanel) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisPanel) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisPanel) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisPanel) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisPanel) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisPanel) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisPanel) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisPanel) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisPanel) Hint() string { return asText(c.Get("hint")) }
func (c aylisPanel) SetHint(v string) { c.Set("hint", v) }

func (c aylisPanel) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisPanel) SetTag(v int64) { c.Set("tag", v) }

func (c aylisPanel) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisPanel) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisPanel) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisPanel) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisPanel) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisPanel) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisSeekBar é um SeekBar de um form.
type aylisSeekBar struct{ aylis.Component }

func (c aylisSeekBar) Position() float64 { return asNumber(c.Get("position")) }
func (c aylisSeekBar) SetPosition(v float64) { c.Set("position", v) }

func (c aylisSeekBar) Duration() float64 { return asNumber(c.Get("duration")) }
func (c aylisSeekBar) SetDuration(v float64) { c.Set("duration", v) }

func (c aylisSeekBar) Buffered() float64 { return asNumber(c.Get("buffered")) }
func (c aylisSeekBar) SetBuffered(v float64) { c.Set("buffered", v) }

func (c aylisSeekBar) Markers() string { return asText(c.Get("markers")) }
func (c aylisSeekBar) SetMarkers(v string) { c.Set("markers", v) }

func (c aylisSeekBar) ShowTime() bool { return asBool(c.Get("show_time")) }
func (c aylisSeekBar) SetShowTime(v bool) { c.Set("show_time", v) }

func (c aylisSeekBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSeekBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSeekBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSeekBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisSeekBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSeekBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSeekBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSeekBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSeekBar) Align() string { return asText(c.Get("align")) }
func (c aylisSeekBar) SetAlign(v string) { c.Set("align", v) }

func (c aylisSeekBar) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisSeekBar) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisSeekBar) Margins() string { return asText(c.Get("margins")) }
func (c aylisSeekBar) SetMargins(v string) { c.Set("margins", v) }

func (c aylisSeekBar) Padding() string { return asText(c.Get("padding")) }
func (c aylisSeekBar) SetPadding(v string) { c.Set("padding", v) }

func (c aylisSeekBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSeekBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSeekBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSeekBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSeekBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSeekBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSeekBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSeekBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSeekBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisSeekBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisSeekBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisSeekBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisSeekBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSeekBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSeekBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSeekBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSeekBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSeekBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSeekBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSeekBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSeekBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSeekBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSeekBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSeekBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSeekBar) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisSeekBar) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisSeekBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSeekBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSeekBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSeekBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSeekBar) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisSeekBar) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisSeekBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSeekBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSeekBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSeekBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSeekBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSeekBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSeekBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSeekBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSeekBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSeekBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSeekBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSeekBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSeekBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSeekBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSeekBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSeekBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSeekBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSeekBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSeekBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSeekBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSeekBar) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisSeekBar) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisSeekBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSeekBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSeekBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSeekBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSeekBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisSeekBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisSeekBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisSeekBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisSeekBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisSeekBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisSeekBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisSeekBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisSeekBar) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisSeekBar) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisSeekBar) Style() string { return asText(c.Get("style")) }
func (c aylisSeekBar) SetStyle(v string) { c.Set("style", v) }

func (c aylisSeekBar) Theme() string { return asText(c.Get("theme")) }
func (c aylisSeekBar) SetTheme(v string) { c.Set("theme", v) }

func (c aylisSeekBar) Fill() string { return asText(c.Get("fill")) }
func (c aylisSeekBar) SetFill(v string) { c.Set("fill", v) }

func (c aylisSeekBar) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisSeekBar) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisSeekBar) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisSeekBar) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisSeekBar) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisSeekBar) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisSeekBar) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisSeekBar) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisSeekBar) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisSeekBar) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisSeekBar) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisSeekBar) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisSeekBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSeekBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSeekBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSeekBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSeekBar) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisSeekBar) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisSeekBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSeekBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSeekBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSeekBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSeekBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSeekBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSeekBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisSeekBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisSeekBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSeekBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSeekBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisSeekBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisSeekBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisSeekBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisSeekBar) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisSeekBar) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisSeekBar) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisTimer é um Timer de um form.
type aylisTimer struct{ aylis.Component }

func (c aylisTimer) Interval() float64 { return asNumber(c.Get("interval")) }
func (c aylisTimer) SetInterval(v float64) { c.Set("interval", v) }

func (c aylisTimer) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTimer) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTimer) Precision() string { return asText(c.Get("precision")) }
func (c aylisTimer) SetPrecision(v string) { c.Set("precision", v) }

func (c aylisTimer) Background() bool { return asBool(c.Get("background")) }
func (c aylisTimer) SetBackground(v bool) { c.Set("background", v) }

func (c aylisTimer) OneShot() bool { return asBool(c.Get("one_shot")) }
func (c aylisTimer) SetOneShot(v bool) { c.Set("one_shot", v) }

func (c aylisTimer) Drift() float64 { return asNumber(c.Get("drift")) }
func (c aylisTimer) SetDrift(v float64) { c.Set("drift", v) }
func (c aylisTimer) Restart() { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "restart", "args": []any{}}) }

// aylisTrackBar é um TrackBar de um form.
type aylisTrackBar struct{ aylis.Component }

func (c aylisTrackBar) Value() float64 { return asNumber(c.Get("value")) }
func (c aylisTrackBar) SetValue(v float64) { c.Set("value", v) }

func (c aylisTrackBar) Min() float64 { return asNumber(c.Get("min")) }
func (c aylisTrackBar) SetMin(v float64) { c.Set("min", v) }

func (c aylisTrackBar) Max() float64 { return asNumber(c.Get("max")) }
func (c aylisTrackBar) SetMax(v float64) { c.Set("max", v) }

func (c aylisTrackBar) Frequency() float64 { return asNumber(c.Get("frequency")) }
func (c aylisTrackBar) SetFrequency(v float64) { c.Set("frequency", v) }

func (c aylisTrackBar) Vertical() bool { return asBool(c.Get("vertical")) }
func (c aylisTrackBar) SetVertical(v bool) { c.Set("vertical", v) }

func (c aylisTrackBar) ThumbSize() float64 { return asNumber(c.Get("thumb_size")) }
func (c aylisTrackBar) SetThumbSize(v float64) { c.Set("thumb_size", v) }

func (c aylisTrackBar) TrackSize() float64 { return asNumber(c.Get("track_size")) }
func (c aylisTrackBar) SetTrackSize(v float64) { c.Set("track_size", v) }

func (c aylisTrackBar) SolidThumb() bool { return asBool(c.Get("solid_thumb")) }
func (c aylisTrackBar) SetSolidThumb(v bool) { c.Set("solid_thumb", v) }

func (c aylisTrackBar) Limit() float64 { return asNumber(c.Get("limit")) }
func (c aylisTrackBar) SetLimit(v float64) { c.Set("limit", v) }

func (c aylisTrackBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTrackBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTrackBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTrackBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisTrackBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTrackBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTrackBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTrackBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTrackBar) Align() string { return asText(c.Get("align")) }
func (c aylisTrackBar) SetAlign(v string) { c.Set("align", v) }

func (c aylisTrackBar) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisTrackBar) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisTrackBar) Margins() string { return asText(c.Get("margins")) }
func (c aylisTrackBar) SetMargins(v string) { c.Set("margins", v) }

func (c aylisTrackBar) Padding() string { return asText(c.Get("padding")) }
func (c aylisTrackBar) SetPadding(v string) { c.Set("padding", v) }

func (c aylisTrackBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTrackBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTrackBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTrackBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTrackBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTrackBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTrackBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTrackBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTrackBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisTrackBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisTrackBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisTrackBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisTrackBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTrackBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTrackBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTrackBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTrackBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTrackBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTrackBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTrackBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTrackBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTrackBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTrackBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTrackBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTrackBar) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisTrackBar) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisTrackBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTrackBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTrackBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTrackBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTrackBar) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisTrackBar) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisTrackBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTrackBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTrackBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTrackBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTrackBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTrackBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTrackBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTrackBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTrackBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTrackBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTrackBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTrackBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTrackBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTrackBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTrackBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTrackBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTrackBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTrackBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTrackBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTrackBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTrackBar) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisTrackBar) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisTrackBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTrackBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTrackBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTrackBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTrackBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisTrackBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisTrackBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisTrackBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisTrackBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisTrackBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisTrackBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisTrackBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisTrackBar) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisTrackBar) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisTrackBar) Style() string { return asText(c.Get("style")) }
func (c aylisTrackBar) SetStyle(v string) { c.Set("style", v) }

func (c aylisTrackBar) Theme() string { return asText(c.Get("theme")) }
func (c aylisTrackBar) SetTheme(v string) { c.Set("theme", v) }

func (c aylisTrackBar) Fill() string { return asText(c.Get("fill")) }
func (c aylisTrackBar) SetFill(v string) { c.Set("fill", v) }

func (c aylisTrackBar) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisTrackBar) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisTrackBar) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisTrackBar) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisTrackBar) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisTrackBar) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisTrackBar) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisTrackBar) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisTrackBar) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisTrackBar) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisTrackBar) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisTrackBar) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisTrackBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTrackBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTrackBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTrackBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTrackBar) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisTrackBar) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisTrackBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTrackBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTrackBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTrackBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTrackBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTrackBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTrackBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisTrackBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisTrackBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTrackBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTrackBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisTrackBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisTrackBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisTrackBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisTrackBar) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisTrackBar) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisTrackBar) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisVideoPlayer é um VideoPlayer de um form.
type aylisVideoPlayer struct{ aylis.Component }

func (c aylisVideoPlayer) File() string { return asText(c.Get("file")) }
func (c aylisVideoPlayer) SetFile(v string) { c.Set("file", v) }

func (c aylisVideoPlayer) Autoplay() bool { return asBool(c.Get("autoplay")) }
func (c aylisVideoPlayer) SetAutoplay(v bool) { c.Set("autoplay", v) }

func (c aylisVideoPlayer) Looping() bool { return asBool(c.Get("looping")) }
func (c aylisVideoPlayer) SetLooping(v bool) { c.Set("looping", v) }

func (c aylisVideoPlayer) Volume() float64 { return asNumber(c.Get("volume")) }
func (c aylisVideoPlayer) SetVolume(v float64) { c.Set("volume", v) }

func (c aylisVideoPlayer) Muted() bool { return asBool(c.Get("muted")) }
func (c aylisVideoPlayer) SetMuted(v bool) { c.Set("muted", v) }

func (c aylisVideoPlayer) Fit() string { return asText(c.Get("fit")) }
func (c aylisVideoPlayer) SetFit(v string) { c.Set("fit", v) }

func (c aylisVideoPlayer) Radius() float64 { return asNumber(c.Get("radius")) }
func (c aylisVideoPlayer) SetRadius(v float64) { c.Set("radius", v) }

func (c aylisVideoPlayer) Rate() float64 { return asNumber(c.Get("rate")) }
func (c aylisVideoPlayer) SetRate(v float64) { c.Set("rate", v) }

func (c aylisVideoPlayer) Output() string { return asText(c.Get("output")) }
func (c aylisVideoPlayer) SetOutput(v string) { c.Set("output", v) }

func (c aylisVideoPlayer) ShowVideo() bool { return asBool(c.Get("show_video")) }
func (c aylisVideoPlayer) SetShowVideo(v bool) { c.Set("show_video", v) }

func (c aylisVideoPlayer) FramesAhead() int64 { return int64(asNumber(c.Get("frames_ahead"))) }
func (c aylisVideoPlayer) SetFramesAhead(v int64) { c.Set("frames_ahead", v) }

func (c aylisVideoPlayer) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisVideoPlayer) SetLeft(v float64) { c.Set("left", v) }

func (c aylisVideoPlayer) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisVideoPlayer) SetTop(v float64) { c.Set("top", v) }

func (c aylisVideoPlayer) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisVideoPlayer) SetWidth(v float64) { c.Set("width", v) }

func (c aylisVideoPlayer) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisVideoPlayer) SetHeight(v float64) { c.Set("height", v) }

func (c aylisVideoPlayer) Align() string { return asText(c.Get("align")) }
func (c aylisVideoPlayer) SetAlign(v string) { c.Set("align", v) }

func (c aylisVideoPlayer) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisVideoPlayer) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisVideoPlayer) Margins() string { return asText(c.Get("margins")) }
func (c aylisVideoPlayer) SetMargins(v string) { c.Set("margins", v) }

func (c aylisVideoPlayer) Padding() string { return asText(c.Get("padding")) }
func (c aylisVideoPlayer) SetPadding(v string) { c.Set("padding", v) }

func (c aylisVideoPlayer) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisVideoPlayer) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisVideoPlayer) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisVideoPlayer) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisVideoPlayer) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisVideoPlayer) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisVideoPlayer) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisVideoPlayer) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisVideoPlayer) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisVideoPlayer) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisVideoPlayer) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisVideoPlayer) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisVideoPlayer) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisVideoPlayer) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisVideoPlayer) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisVideoPlayer) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisVideoPlayer) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisVideoPlayer) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisVideoPlayer) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisVideoPlayer) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisVideoPlayer) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisVideoPlayer) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisVideoPlayer) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisVideoPlayer) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisVideoPlayer) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisVideoPlayer) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisVideoPlayer) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisVideoPlayer) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisVideoPlayer) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisVideoPlayer) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisVideoPlayer) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisVideoPlayer) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisVideoPlayer) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisVideoPlayer) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisVideoPlayer) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisVideoPlayer) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisVideoPlayer) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisVideoPlayer) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisVideoPlayer) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisVideoPlayer) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisVideoPlayer) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisVideoPlayer) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisVideoPlayer) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisVideoPlayer) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisVideoPlayer) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisVideoPlayer) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisVideoPlayer) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisVideoPlayer) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisVideoPlayer) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisVideoPlayer) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisVideoPlayer) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisVideoPlayer) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisVideoPlayer) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisVideoPlayer) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisVideoPlayer) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisVideoPlayer) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisVideoPlayer) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisVideoPlayer) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisVideoPlayer) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisVideoPlayer) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisVideoPlayer) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisVideoPlayer) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisVideoPlayer) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisVideoPlayer) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisVideoPlayer) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisVideoPlayer) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisVideoPlayer) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisVideoPlayer) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisVideoPlayer) Style() string { return asText(c.Get("style")) }
func (c aylisVideoPlayer) SetStyle(v string) { c.Set("style", v) }

func (c aylisVideoPlayer) Theme() string { return asText(c.Get("theme")) }
func (c aylisVideoPlayer) SetTheme(v string) { c.Set("theme", v) }

func (c aylisVideoPlayer) Fill() string { return asText(c.Get("fill")) }
func (c aylisVideoPlayer) SetFill(v string) { c.Set("fill", v) }

func (c aylisVideoPlayer) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisVideoPlayer) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisVideoPlayer) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisVideoPlayer) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisVideoPlayer) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisVideoPlayer) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisVideoPlayer) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisVideoPlayer) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisVideoPlayer) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisVideoPlayer) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisVideoPlayer) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisVideoPlayer) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisVideoPlayer) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisVideoPlayer) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisVideoPlayer) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisVideoPlayer) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisVideoPlayer) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisVideoPlayer) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisVideoPlayer) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisVideoPlayer) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisVideoPlayer) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisVideoPlayer) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisVideoPlayer) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisVideoPlayer) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisVideoPlayer) Hint() string { return asText(c.Get("hint")) }
func (c aylisVideoPlayer) SetHint(v string) { c.Set("hint", v) }

func (c aylisVideoPlayer) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisVideoPlayer) SetTag(v int64) { c.Set("tag", v) }

func (c aylisVideoPlayer) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisVideoPlayer) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisVideoPlayer) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisVideoPlayer) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisVideoPlayer) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisVideoPlayer) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisVideoPlayer) Play() { c.App.Send("media", aylis.M{"target": c.ID, "op": "play"}) }
func (c aylisVideoPlayer) Pause() { c.App.Send("media", aylis.M{"target": c.ID, "op": "pause"}) }
func (c aylisVideoPlayer) Stop() { c.App.Send("media", aylis.M{"target": c.ID, "op": "stop"}) }
func (c aylisVideoPlayer) Seek(seconds float64) { c.App.Send("media", aylis.M{"target": c.ID, "op": "seek", "seconds": seconds}) }
func (c aylisVideoPlayer) Position() float64 { return asNumber(askCmd(c.App, "media", aylis.M{"target": c.ID, "op": "position"})) }
func (c aylisVideoPlayer) Duration() float64 { return asNumber(askCmd(c.App, "media", aylis.M{"target": c.ID, "op": "duration"})) }

// frmPlayerComponents: o form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmPlayerComponents struct {
	aylisForm
	mnuPrincipal aylisMainMenu
	mnuArquivo aylisMenuItem
	mnuAbrir aylisMenuItem
	mnuLimparFila aylisMenuItem
	mnuSep1 aylisMenuItem
	mnuSair aylisMenuItem
	mnuReproduzir aylisMenuItem
	mnuReproduzirPausar aylisMenuItem
	mnuParar aylisMenuItem
	mnuAnterior aylisMenuItem
	mnuProxima aylisMenuItem
	mnuSep2 aylisMenuItem
	mnuEmbaralhar aylisMenuItem
	mnuRepetir aylisMenuItem
	mnuExibir aylisMenuItem
	mnuAgora aylisMenuItem
	mnuFila aylisMenuItem
	pnlControles aylisPanel
	lytBarra aylisLayout
	lblTempo aylisLabel
	lblDuracao aylisLabel
	sbPosicao aylisSeekBar
	lytInfo aylisLayout
	icoCapa aylisIcon
	lytTextos aylisLayout
	lblTitulo aylisLabel
	lblDetalhe aylisLabel
	lytVolume aylisLayout
	btnMudo aylisButton
	tbVolume aylisTrackBar
	lytBotoes aylisLayout
	lytBotoesCentro aylisLayout
	btnEmbaralhar aylisButton
	btnAnterior aylisButton
	btnReproduzir aylisButton
	btnProxima aylisButton
	btnRepetir aylisButton
	lytNavegacao aylisLayout
	btnAgora aylisButton
	btnFila aylisButton
	divNavegacao aylisDivider
	btnAbrirArquivos aylisButton
	pnlAgora aylisPanel
	vpVideo aylisVideoPlayer
	pnlFila aylisPanel
	lblFila aylisLabel
	lytFilaBotoes aylisLayout
	btnAdicionar aylisButton
	btnLimparFila aylisButton
	lvFila aylisListView
	dlgAbrir aylisOpenDialog
	tmrPosicao aylisTimer
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmPlayerComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "mnuPrincipal", "MainMenu"); err != nil {
		return err
	} else {
		f.mnuPrincipal = aylisMainMenu{c}
	}
	if c, err := f.Resolve(list, "mnuArquivo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuArquivo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAbrir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAbrir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuLimparFila", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuLimparFila = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep1", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep1 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSair", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSair = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuReproduzir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReproduzir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuReproduzirPausar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReproduzirPausar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuParar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuParar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAnterior", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAnterior = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuProxima", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuProxima = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep2", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep2 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuEmbaralhar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuEmbaralhar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRepetir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRepetir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExibir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExibir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAgora", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAgora = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuFila", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuFila = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "pnlControles", "Panel"); err != nil {
		return err
	} else {
		f.pnlControles = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lytBarra", "Layout"); err != nil {
		return err
	} else {
		f.lytBarra = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTempo", "Label"); err != nil {
		return err
	} else {
		f.lblTempo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblDuracao", "Label"); err != nil {
		return err
	} else {
		f.lblDuracao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "sbPosicao", "SeekBar"); err != nil {
		return err
	} else {
		f.sbPosicao = aylisSeekBar{c}
	}
	if c, err := f.Resolve(list, "lytInfo", "Layout"); err != nil {
		return err
	} else {
		f.lytInfo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "icoCapa", "Icon"); err != nil {
		return err
	} else {
		f.icoCapa = aylisIcon{c}
	}
	if c, err := f.Resolve(list, "lytTextos", "Layout"); err != nil {
		return err
	} else {
		f.lytTextos = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTitulo", "Label"); err != nil {
		return err
	} else {
		f.lblTitulo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblDetalhe", "Label"); err != nil {
		return err
	} else {
		f.lblDetalhe = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytVolume", "Layout"); err != nil {
		return err
	} else {
		f.lytVolume = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnMudo", "Button"); err != nil {
		return err
	} else {
		f.btnMudo = aylisButton{c}
	}
	if c, err := f.Resolve(list, "tbVolume", "TrackBar"); err != nil {
		return err
	} else {
		f.tbVolume = aylisTrackBar{c}
	}
	if c, err := f.Resolve(list, "lytBotoes", "Layout"); err != nil {
		return err
	} else {
		f.lytBotoes = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lytBotoesCentro", "Layout"); err != nil {
		return err
	} else {
		f.lytBotoesCentro = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnEmbaralhar", "Button"); err != nil {
		return err
	} else {
		f.btnEmbaralhar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnAnterior", "Button"); err != nil {
		return err
	} else {
		f.btnAnterior = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnReproduzir", "Button"); err != nil {
		return err
	} else {
		f.btnReproduzir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnProxima", "Button"); err != nil {
		return err
	} else {
		f.btnProxima = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRepetir", "Button"); err != nil {
		return err
	} else {
		f.btnRepetir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lytNavegacao", "Layout"); err != nil {
		return err
	} else {
		f.lytNavegacao = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnAgora", "Button"); err != nil {
		return err
	} else {
		f.btnAgora = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnFila", "Button"); err != nil {
		return err
	} else {
		f.btnFila = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divNavegacao", "Divider"); err != nil {
		return err
	} else {
		f.divNavegacao = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "btnAbrirArquivos", "Button"); err != nil {
		return err
	} else {
		f.btnAbrirArquivos = aylisButton{c}
	}
	if c, err := f.Resolve(list, "pnlAgora", "Panel"); err != nil {
		return err
	} else {
		f.pnlAgora = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "vpVideo", "VideoPlayer"); err != nil {
		return err
	} else {
		f.vpVideo = aylisVideoPlayer{c}
	}
	if c, err := f.Resolve(list, "pnlFila", "Panel"); err != nil {
		return err
	} else {
		f.pnlFila = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lblFila", "Label"); err != nil {
		return err
	} else {
		f.lblFila = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytFilaBotoes", "Layout"); err != nil {
		return err
	} else {
		f.lytFilaBotoes = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnAdicionar", "Button"); err != nil {
		return err
	} else {
		f.btnAdicionar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnLimparFila", "Button"); err != nil {
		return err
	} else {
		f.btnLimparFila = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lvFila", "ListView"); err != nil {
		return err
	} else {
		f.lvFila = aylisListView{c}
	}
	if c, err := f.Resolve(list, "dlgAbrir", "OpenDialog"); err != nil {
		return err
	} else {
		f.dlgAbrir = aylisOpenDialog{c}
	}
	if c, err := f.Resolve(list, "tmrPosicao", "Timer"); err != nil {
		return err
	} else {
		f.tmrPosicao = aylisTimer{c}
	}
	return nil
}

// frmPlayer é a janela de frmPlayer cujo evento está rodando (a primeira, fora dos handlers).
var frmPlayer *frmPlayerComponents

func init() {
	aylis.RegisterFormClass("frmPlayer", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmPlayerComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmPlayer = form.(*frmPlayerComponents) })
}

// FindButton é o Button deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnMudo":
		return f.btnMudo
	case "btnEmbaralhar":
		return f.btnEmbaralhar
	case "btnAnterior":
		return f.btnAnterior
	case "btnReproduzir":
		return f.btnReproduzir
	case "btnProxima":
		return f.btnProxima
	case "btnRepetir":
		return f.btnRepetir
	case "btnAgora":
		return f.btnAgora
	case "btnFila":
		return f.btnFila
	case "btnAbrirArquivos":
		return f.btnAbrirArquivos
	case "btnAdicionar":
		return f.btnAdicionar
	case "btnLimparFila":
		return f.btnLimparFila
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmPlayerComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindDivider é o Divider deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindDivider(name string) aylisDivider {
	switch name {
	case "divNavegacao":
		return f.divNavegacao
	}
	return aylisDivider{}
}

// SenderAsDivider é o Divider que disparou o evento.
func (f *frmPlayerComponents) SenderAsDivider(ev aylis.Event) aylisDivider { return f.FindDivider(ev.Sender.Name) }

// FindIcon é o Icon deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindIcon(name string) aylisIcon {
	switch name {
	case "icoCapa":
		return f.icoCapa
	}
	return aylisIcon{}
}

// SenderAsIcon é o Icon que disparou o evento.
func (f *frmPlayerComponents) SenderAsIcon(ev aylis.Event) aylisIcon { return f.FindIcon(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblTempo":
		return f.lblTempo
	case "lblDuracao":
		return f.lblDuracao
	case "lblTitulo":
		return f.lblTitulo
	case "lblDetalhe":
		return f.lblDetalhe
	case "lblFila":
		return f.lblFila
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmPlayerComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytBarra":
		return f.lytBarra
	case "lytInfo":
		return f.lytInfo
	case "lytTextos":
		return f.lytTextos
	case "lytVolume":
		return f.lytVolume
	case "lytBotoes":
		return f.lytBotoes
	case "lytBotoesCentro":
		return f.lytBotoesCentro
	case "lytNavegacao":
		return f.lytNavegacao
	case "lytFilaBotoes":
		return f.lytFilaBotoes
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmPlayerComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindListView é o ListView deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindListView(name string) aylisListView {
	switch name {
	case "lvFila":
		return f.lvFila
	}
	return aylisListView{}
}

// SenderAsListView é o ListView que disparou o evento.
func (f *frmPlayerComponents) SenderAsListView(ev aylis.Event) aylisListView { return f.FindListView(ev.Sender.Name) }

// FindMainMenu é o MainMenu deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindMainMenu(name string) aylisMainMenu {
	switch name {
	case "mnuPrincipal":
		return f.mnuPrincipal
	}
	return aylisMainMenu{}
}

// SenderAsMainMenu é o MainMenu que disparou o evento.
func (f *frmPlayerComponents) SenderAsMainMenu(ev aylis.Event) aylisMainMenu { return f.FindMainMenu(ev.Sender.Name) }

// FindMenuItem é o MenuItem deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindMenuItem(name string) aylisMenuItem {
	switch name {
	case "mnuArquivo":
		return f.mnuArquivo
	case "mnuAbrir":
		return f.mnuAbrir
	case "mnuLimparFila":
		return f.mnuLimparFila
	case "mnuSep1":
		return f.mnuSep1
	case "mnuSair":
		return f.mnuSair
	case "mnuReproduzir":
		return f.mnuReproduzir
	case "mnuReproduzirPausar":
		return f.mnuReproduzirPausar
	case "mnuParar":
		return f.mnuParar
	case "mnuAnterior":
		return f.mnuAnterior
	case "mnuProxima":
		return f.mnuProxima
	case "mnuSep2":
		return f.mnuSep2
	case "mnuEmbaralhar":
		return f.mnuEmbaralhar
	case "mnuRepetir":
		return f.mnuRepetir
	case "mnuExibir":
		return f.mnuExibir
	case "mnuAgora":
		return f.mnuAgora
	case "mnuFila":
		return f.mnuFila
	}
	return aylisMenuItem{}
}

// SenderAsMenuItem é o MenuItem que disparou o evento.
func (f *frmPlayerComponents) SenderAsMenuItem(ev aylis.Event) aylisMenuItem { return f.FindMenuItem(ev.Sender.Name) }

// FindOpenDialog é o OpenDialog deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindOpenDialog(name string) aylisOpenDialog {
	switch name {
	case "dlgAbrir":
		return f.dlgAbrir
	}
	return aylisOpenDialog{}
}

// SenderAsOpenDialog é o OpenDialog que disparou o evento.
func (f *frmPlayerComponents) SenderAsOpenDialog(ev aylis.Event) aylisOpenDialog { return f.FindOpenDialog(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlControles":
		return f.pnlControles
	case "pnlAgora":
		return f.pnlAgora
	case "pnlFila":
		return f.pnlFila
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmPlayerComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSeekBar é o SeekBar deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindSeekBar(name string) aylisSeekBar {
	switch name {
	case "sbPosicao":
		return f.sbPosicao
	}
	return aylisSeekBar{}
}

// SenderAsSeekBar é o SeekBar que disparou o evento.
func (f *frmPlayerComponents) SenderAsSeekBar(ev aylis.Event) aylisSeekBar { return f.FindSeekBar(ev.Sender.Name) }

// FindTimer é o Timer deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindTimer(name string) aylisTimer {
	switch name {
	case "tmrPosicao":
		return f.tmrPosicao
	}
	return aylisTimer{}
}

// SenderAsTimer é o Timer que disparou o evento.
func (f *frmPlayerComponents) SenderAsTimer(ev aylis.Event) aylisTimer { return f.FindTimer(ev.Sender.Name) }

// FindTrackBar é o TrackBar deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindTrackBar(name string) aylisTrackBar {
	switch name {
	case "tbVolume":
		return f.tbVolume
	}
	return aylisTrackBar{}
}

// SenderAsTrackBar é o TrackBar que disparou o evento.
func (f *frmPlayerComponents) SenderAsTrackBar(ev aylis.Event) aylisTrackBar { return f.FindTrackBar(ev.Sender.Name) }

// FindVideoPlayer é o VideoPlayer deste form com o nome (como o FindComponent).
func (f *frmPlayerComponents) FindVideoPlayer(name string) aylisVideoPlayer {
	switch name {
	case "vpVideo":
		return f.vpVideo
	}
	return aylisVideoPlayer{}
}

// SenderAsVideoPlayer é o VideoPlayer que disparou o evento.
func (f *frmPlayerComponents) SenderAsVideoPlayer(ev aylis.Event) aylisVideoPlayer { return f.FindVideoPlayer(ev.Sender.Name) }
