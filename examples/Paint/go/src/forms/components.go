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

// aylisCheckBox é um CheckBox de um form.
type aylisCheckBox struct{ aylis.Component }

func (c aylisCheckBox) Text() string { return asText(c.Get("text")) }
func (c aylisCheckBox) SetText(v string) { c.Set("text", v) }

func (c aylisCheckBox) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisCheckBox) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisCheckBox) BoxSize() float64 { return asNumber(c.Get("box_size")) }
func (c aylisCheckBox) SetBoxSize(v float64) { c.Set("box_size", v) }

func (c aylisCheckBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisCheckBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisCheckBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisCheckBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisCheckBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisCheckBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisCheckBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisCheckBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisCheckBox) Align() string { return asText(c.Get("align")) }
func (c aylisCheckBox) SetAlign(v string) { c.Set("align", v) }

func (c aylisCheckBox) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisCheckBox) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisCheckBox) Margins() string { return asText(c.Get("margins")) }
func (c aylisCheckBox) SetMargins(v string) { c.Set("margins", v) }

func (c aylisCheckBox) Padding() string { return asText(c.Get("padding")) }
func (c aylisCheckBox) SetPadding(v string) { c.Set("padding", v) }

func (c aylisCheckBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisCheckBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisCheckBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisCheckBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisCheckBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisCheckBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisCheckBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisCheckBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisCheckBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisCheckBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisCheckBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisCheckBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisCheckBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisCheckBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisCheckBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisCheckBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisCheckBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisCheckBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisCheckBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisCheckBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisCheckBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisCheckBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisCheckBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisCheckBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisCheckBox) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisCheckBox) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisCheckBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisCheckBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisCheckBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisCheckBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisCheckBox) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisCheckBox) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisCheckBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisCheckBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisCheckBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisCheckBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisCheckBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisCheckBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisCheckBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisCheckBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisCheckBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisCheckBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisCheckBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisCheckBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisCheckBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisCheckBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisCheckBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisCheckBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisCheckBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisCheckBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisCheckBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisCheckBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisCheckBox) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisCheckBox) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisCheckBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisCheckBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisCheckBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisCheckBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisCheckBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisCheckBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisCheckBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisCheckBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisCheckBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisCheckBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisCheckBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisCheckBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisCheckBox) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisCheckBox) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisCheckBox) Style() string { return asText(c.Get("style")) }
func (c aylisCheckBox) SetStyle(v string) { c.Set("style", v) }

func (c aylisCheckBox) Theme() string { return asText(c.Get("theme")) }
func (c aylisCheckBox) SetTheme(v string) { c.Set("theme", v) }

func (c aylisCheckBox) Fill() string { return asText(c.Get("fill")) }
func (c aylisCheckBox) SetFill(v string) { c.Set("fill", v) }

func (c aylisCheckBox) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisCheckBox) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisCheckBox) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisCheckBox) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisCheckBox) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisCheckBox) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisCheckBox) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisCheckBox) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisCheckBox) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisCheckBox) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisCheckBox) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisCheckBox) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisCheckBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisCheckBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisCheckBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisCheckBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisCheckBox) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisCheckBox) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisCheckBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisCheckBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisCheckBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisCheckBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisCheckBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisCheckBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisCheckBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisCheckBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisCheckBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisCheckBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisCheckBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisCheckBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisCheckBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisCheckBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisCheckBox) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisCheckBox) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisCheckBox) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisCircle é um Circle de um form.
type aylisCircle struct{ aylis.Component }

func (c aylisCircle) Fill() string { return asText(c.Get("fill")) }
func (c aylisCircle) SetFill(v string) { c.Set("fill", v) }

func (c aylisCircle) Stroke() string { return asText(c.Get("stroke")) }
func (c aylisCircle) SetStroke(v string) { c.Set("stroke", v) }

func (c aylisCircle) StrokeWidth() float64 { return asNumber(c.Get("stroke_width")) }
func (c aylisCircle) SetStrokeWidth(v float64) { c.Set("stroke_width", v) }

func (c aylisCircle) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisCircle) SetLeft(v float64) { c.Set("left", v) }

func (c aylisCircle) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisCircle) SetTop(v float64) { c.Set("top", v) }

func (c aylisCircle) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisCircle) SetWidth(v float64) { c.Set("width", v) }

func (c aylisCircle) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisCircle) SetHeight(v float64) { c.Set("height", v) }

func (c aylisCircle) Align() string { return asText(c.Get("align")) }
func (c aylisCircle) SetAlign(v string) { c.Set("align", v) }

func (c aylisCircle) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisCircle) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisCircle) Margins() string { return asText(c.Get("margins")) }
func (c aylisCircle) SetMargins(v string) { c.Set("margins", v) }

func (c aylisCircle) Padding() string { return asText(c.Get("padding")) }
func (c aylisCircle) SetPadding(v string) { c.Set("padding", v) }

func (c aylisCircle) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisCircle) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisCircle) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisCircle) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisCircle) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisCircle) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisCircle) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisCircle) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisCircle) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisCircle) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisCircle) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisCircle) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisCircle) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisCircle) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisCircle) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisCircle) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisCircle) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisCircle) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisCircle) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisCircle) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisCircle) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisCircle) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisCircle) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisCircle) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisCircle) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisCircle) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisCircle) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisCircle) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisCircle) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisCircle) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisCircle) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisCircle) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisCircle) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisCircle) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisCircle) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisCircle) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisCircle) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisCircle) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisCircle) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisCircle) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisCircle) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisCircle) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisCircle) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisCircle) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisCircle) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisCircle) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisCircle) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisCircle) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisCircle) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisCircle) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisCircle) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisCircle) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisCircle) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisCircle) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisCircle) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisCircle) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisCircle) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisCircle) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisCircle) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisCircle) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisCircle) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisCircle) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisCircle) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisCircle) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisCircle) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisCircle) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisCircle) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisCircle) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisCircle) Style() string { return asText(c.Get("style")) }
func (c aylisCircle) SetStyle(v string) { c.Set("style", v) }

func (c aylisCircle) Theme() string { return asText(c.Get("theme")) }
func (c aylisCircle) SetTheme(v string) { c.Set("theme", v) }

func (c aylisCircle) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisCircle) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisCircle) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisCircle) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisCircle) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisCircle) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisCircle) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisCircle) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisCircle) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisCircle) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisCircle) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisCircle) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisCircle) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisCircle) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisCircle) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisCircle) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisCircle) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisCircle) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisCircle) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisCircle) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisCircle) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisCircle) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisCircle) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisCircle) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisCircle) Hint() string { return asText(c.Get("hint")) }
func (c aylisCircle) SetHint(v string) { c.Set("hint", v) }

func (c aylisCircle) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisCircle) SetTag(v int64) { c.Set("tag", v) }

func (c aylisCircle) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisCircle) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisCircle) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisCircle) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisCircle) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisCircle) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisComboBox é um ComboBox de um form.
type aylisComboBox struct{ aylis.Component }

func (c aylisComboBox) Items() string { return asText(c.Get("items")) }
func (c aylisComboBox) SetItems(v string) { c.Set("items", v) }

func (c aylisComboBox) Values() string { return asText(c.Get("values")) }
func (c aylisComboBox) SetValues(v string) { c.Set("values", v) }

func (c aylisComboBox) ItemIndex() int64 { return int64(asNumber(c.Get("item_index"))) }
func (c aylisComboBox) SetItemIndex(v int64) { c.Set("item_index", v) }

func (c aylisComboBox) HAlign() string { return asText(c.Get("h_align")) }
func (c aylisComboBox) SetHAlign(v string) { c.Set("h_align", v) }

func (c aylisComboBox) Trimming() string { return asText(c.Get("trimming")) }
func (c aylisComboBox) SetTrimming(v string) { c.Set("trimming", v) }

func (c aylisComboBox) Arrow() string { return asText(c.Get("arrow")) }
func (c aylisComboBox) SetArrow(v string) { c.Set("arrow", v) }

func (c aylisComboBox) Sensitive() bool { return asBool(c.Get("sensitive")) }
func (c aylisComboBox) SetSensitive(v bool) { c.Set("sensitive", v) }

func (c aylisComboBox) Groups() string { return asText(c.Get("groups")) }
func (c aylisComboBox) SetGroups(v string) { c.Set("groups", v) }

func (c aylisComboBox) DisabledItems() string { return asText(c.Get("disabled_items")) }
func (c aylisComboBox) SetDisabledItems(v string) { c.Set("disabled_items", v) }

func (c aylisComboBox) Value() string { return asText(c.Get("value")) }
func (c aylisComboBox) SetValue(v string) { c.Set("value", v) }

func (c aylisComboBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisComboBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisComboBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisComboBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisComboBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisComboBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisComboBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisComboBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisComboBox) Align() string { return asText(c.Get("align")) }
func (c aylisComboBox) SetAlign(v string) { c.Set("align", v) }

func (c aylisComboBox) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisComboBox) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisComboBox) Margins() string { return asText(c.Get("margins")) }
func (c aylisComboBox) SetMargins(v string) { c.Set("margins", v) }

func (c aylisComboBox) Padding() string { return asText(c.Get("padding")) }
func (c aylisComboBox) SetPadding(v string) { c.Set("padding", v) }

func (c aylisComboBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisComboBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisComboBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisComboBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisComboBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisComboBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisComboBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisComboBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisComboBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisComboBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisComboBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisComboBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisComboBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisComboBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisComboBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisComboBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisComboBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisComboBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisComboBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisComboBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisComboBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisComboBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisComboBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisComboBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisComboBox) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisComboBox) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisComboBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisComboBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisComboBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisComboBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisComboBox) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisComboBox) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisComboBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisComboBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisComboBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisComboBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisComboBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisComboBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisComboBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisComboBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisComboBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisComboBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisComboBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisComboBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisComboBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisComboBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisComboBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisComboBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisComboBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisComboBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisComboBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisComboBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisComboBox) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisComboBox) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisComboBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisComboBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisComboBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisComboBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisComboBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisComboBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisComboBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisComboBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisComboBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisComboBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisComboBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisComboBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisComboBox) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisComboBox) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisComboBox) Style() string { return asText(c.Get("style")) }
func (c aylisComboBox) SetStyle(v string) { c.Set("style", v) }

func (c aylisComboBox) Theme() string { return asText(c.Get("theme")) }
func (c aylisComboBox) SetTheme(v string) { c.Set("theme", v) }

func (c aylisComboBox) Fill() string { return asText(c.Get("fill")) }
func (c aylisComboBox) SetFill(v string) { c.Set("fill", v) }

func (c aylisComboBox) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisComboBox) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisComboBox) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisComboBox) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisComboBox) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisComboBox) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisComboBox) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisComboBox) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisComboBox) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisComboBox) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisComboBox) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisComboBox) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisComboBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisComboBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisComboBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisComboBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisComboBox) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisComboBox) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisComboBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisComboBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisComboBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisComboBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisComboBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisComboBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisComboBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisComboBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisComboBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisComboBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisComboBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisComboBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisComboBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisComboBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisComboBox) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisComboBox) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisComboBox) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

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

// aylisGridLayout é um GridLayout de um form.
type aylisGridLayout struct{ aylis.Component }

func (c aylisGridLayout) Columns() int64 { return int64(asNumber(c.Get("columns"))) }
func (c aylisGridLayout) SetColumns(v int64) { c.Set("columns", v) }

func (c aylisGridLayout) MinColumnWidth() float64 { return asNumber(c.Get("min_column_width")) }
func (c aylisGridLayout) SetMinColumnWidth(v float64) { c.Set("min_column_width", v) }

func (c aylisGridLayout) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisGridLayout) SetGap(v float64) { c.Set("gap", v) }

func (c aylisGridLayout) RowHeight() float64 { return asNumber(c.Get("row_height")) }
func (c aylisGridLayout) SetRowHeight(v float64) { c.Set("row_height", v) }

func (c aylisGridLayout) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisGridLayout) SetLeft(v float64) { c.Set("left", v) }

func (c aylisGridLayout) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisGridLayout) SetTop(v float64) { c.Set("top", v) }

func (c aylisGridLayout) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisGridLayout) SetWidth(v float64) { c.Set("width", v) }

func (c aylisGridLayout) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisGridLayout) SetHeight(v float64) { c.Set("height", v) }

func (c aylisGridLayout) Align() string { return asText(c.Get("align")) }
func (c aylisGridLayout) SetAlign(v string) { c.Set("align", v) }

func (c aylisGridLayout) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisGridLayout) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisGridLayout) Margins() string { return asText(c.Get("margins")) }
func (c aylisGridLayout) SetMargins(v string) { c.Set("margins", v) }

func (c aylisGridLayout) Padding() string { return asText(c.Get("padding")) }
func (c aylisGridLayout) SetPadding(v string) { c.Set("padding", v) }

func (c aylisGridLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisGridLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisGridLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisGridLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisGridLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisGridLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisGridLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisGridLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisGridLayout) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisGridLayout) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisGridLayout) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisGridLayout) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisGridLayout) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisGridLayout) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisGridLayout) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisGridLayout) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisGridLayout) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisGridLayout) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisGridLayout) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisGridLayout) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisGridLayout) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisGridLayout) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisGridLayout) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisGridLayout) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisGridLayout) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisGridLayout) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisGridLayout) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisGridLayout) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisGridLayout) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisGridLayout) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisGridLayout) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisGridLayout) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisGridLayout) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisGridLayout) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisGridLayout) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisGridLayout) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisGridLayout) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisGridLayout) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisGridLayout) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisGridLayout) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisGridLayout) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisGridLayout) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisGridLayout) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisGridLayout) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisGridLayout) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisGridLayout) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisGridLayout) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisGridLayout) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisGridLayout) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisGridLayout) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisGridLayout) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisGridLayout) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisGridLayout) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisGridLayout) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisGridLayout) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisGridLayout) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisGridLayout) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisGridLayout) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisGridLayout) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisGridLayout) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisGridLayout) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisGridLayout) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisGridLayout) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisGridLayout) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisGridLayout) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisGridLayout) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisGridLayout) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisGridLayout) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisGridLayout) Style() string { return asText(c.Get("style")) }
func (c aylisGridLayout) SetStyle(v string) { c.Set("style", v) }

func (c aylisGridLayout) Theme() string { return asText(c.Get("theme")) }
func (c aylisGridLayout) SetTheme(v string) { c.Set("theme", v) }

func (c aylisGridLayout) Fill() string { return asText(c.Get("fill")) }
func (c aylisGridLayout) SetFill(v string) { c.Set("fill", v) }

func (c aylisGridLayout) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisGridLayout) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisGridLayout) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisGridLayout) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisGridLayout) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisGridLayout) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisGridLayout) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisGridLayout) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisGridLayout) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisGridLayout) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisGridLayout) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisGridLayout) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisGridLayout) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisGridLayout) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisGridLayout) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisGridLayout) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisGridLayout) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisGridLayout) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisGridLayout) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisGridLayout) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisGridLayout) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisGridLayout) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisGridLayout) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisGridLayout) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisGridLayout) Hint() string { return asText(c.Get("hint")) }
func (c aylisGridLayout) SetHint(v string) { c.Set("hint", v) }

func (c aylisGridLayout) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisGridLayout) SetTag(v int64) { c.Set("tag", v) }

func (c aylisGridLayout) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisGridLayout) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisGridLayout) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisGridLayout) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisGridLayout) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisGridLayout) SetPopupMenu(v string) { c.Set("popup_menu", v) }

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

// aylisPaintCanvas é um PaintCanvas de um form.
type aylisPaintCanvas struct{ aylis.Component }

func (c aylisPaintCanvas) Tool() string { return asText(c.Get("tool")) }
func (c aylisPaintCanvas) SetTool(v string) { c.Set("tool", v) }

func (c aylisPaintCanvas) Color() string { return asText(c.Get("color")) }
func (c aylisPaintCanvas) SetColor(v string) { c.Set("color", v) }

func (c aylisPaintCanvas) BackColor() string { return asText(c.Get("back_color")) }
func (c aylisPaintCanvas) SetBackColor(v string) { c.Set("back_color", v) }

func (c aylisPaintCanvas) BrushSize() int64 { return int64(asNumber(c.Get("brush_size"))) }
func (c aylisPaintCanvas) SetBrushSize(v int64) { c.Set("brush_size", v) }

func (c aylisPaintCanvas) Filled() bool { return asBool(c.Get("filled")) }
func (c aylisPaintCanvas) SetFilled(v bool) { c.Set("filled", v) }

func (c aylisPaintCanvas) Zoom() float64 { return asNumber(c.Get("zoom")) }
func (c aylisPaintCanvas) SetZoom(v float64) { c.Set("zoom", v) }

func (c aylisPaintCanvas) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisPaintCanvas) SetLeft(v float64) { c.Set("left", v) }

func (c aylisPaintCanvas) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisPaintCanvas) SetTop(v float64) { c.Set("top", v) }

func (c aylisPaintCanvas) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisPaintCanvas) SetWidth(v float64) { c.Set("width", v) }

func (c aylisPaintCanvas) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisPaintCanvas) SetHeight(v float64) { c.Set("height", v) }

func (c aylisPaintCanvas) Align() string { return asText(c.Get("align")) }
func (c aylisPaintCanvas) SetAlign(v string) { c.Set("align", v) }

func (c aylisPaintCanvas) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisPaintCanvas) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisPaintCanvas) Margins() string { return asText(c.Get("margins")) }
func (c aylisPaintCanvas) SetMargins(v string) { c.Set("margins", v) }

func (c aylisPaintCanvas) Padding() string { return asText(c.Get("padding")) }
func (c aylisPaintCanvas) SetPadding(v string) { c.Set("padding", v) }

func (c aylisPaintCanvas) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisPaintCanvas) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisPaintCanvas) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisPaintCanvas) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisPaintCanvas) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisPaintCanvas) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisPaintCanvas) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisPaintCanvas) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisPaintCanvas) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisPaintCanvas) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisPaintCanvas) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisPaintCanvas) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisPaintCanvas) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisPaintCanvas) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisPaintCanvas) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisPaintCanvas) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisPaintCanvas) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisPaintCanvas) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisPaintCanvas) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisPaintCanvas) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisPaintCanvas) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisPaintCanvas) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisPaintCanvas) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisPaintCanvas) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisPaintCanvas) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisPaintCanvas) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisPaintCanvas) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisPaintCanvas) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisPaintCanvas) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisPaintCanvas) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisPaintCanvas) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisPaintCanvas) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisPaintCanvas) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisPaintCanvas) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisPaintCanvas) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisPaintCanvas) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisPaintCanvas) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisPaintCanvas) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisPaintCanvas) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisPaintCanvas) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisPaintCanvas) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisPaintCanvas) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisPaintCanvas) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisPaintCanvas) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisPaintCanvas) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisPaintCanvas) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisPaintCanvas) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisPaintCanvas) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisPaintCanvas) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisPaintCanvas) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisPaintCanvas) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisPaintCanvas) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisPaintCanvas) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisPaintCanvas) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisPaintCanvas) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisPaintCanvas) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisPaintCanvas) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisPaintCanvas) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisPaintCanvas) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisPaintCanvas) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisPaintCanvas) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisPaintCanvas) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisPaintCanvas) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisPaintCanvas) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisPaintCanvas) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisPaintCanvas) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisPaintCanvas) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisPaintCanvas) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisPaintCanvas) Style() string { return asText(c.Get("style")) }
func (c aylisPaintCanvas) SetStyle(v string) { c.Set("style", v) }

func (c aylisPaintCanvas) Theme() string { return asText(c.Get("theme")) }
func (c aylisPaintCanvas) SetTheme(v string) { c.Set("theme", v) }

func (c aylisPaintCanvas) Fill() string { return asText(c.Get("fill")) }
func (c aylisPaintCanvas) SetFill(v string) { c.Set("fill", v) }

func (c aylisPaintCanvas) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisPaintCanvas) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisPaintCanvas) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisPaintCanvas) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisPaintCanvas) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisPaintCanvas) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisPaintCanvas) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisPaintCanvas) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisPaintCanvas) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisPaintCanvas) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisPaintCanvas) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisPaintCanvas) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisPaintCanvas) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisPaintCanvas) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisPaintCanvas) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisPaintCanvas) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisPaintCanvas) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisPaintCanvas) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisPaintCanvas) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisPaintCanvas) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisPaintCanvas) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisPaintCanvas) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisPaintCanvas) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisPaintCanvas) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisPaintCanvas) Hint() string { return asText(c.Get("hint")) }
func (c aylisPaintCanvas) SetHint(v string) { c.Set("hint", v) }

func (c aylisPaintCanvas) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisPaintCanvas) SetTag(v int64) { c.Set("tag", v) }

func (c aylisPaintCanvas) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisPaintCanvas) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisPaintCanvas) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisPaintCanvas) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisPaintCanvas) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisPaintCanvas) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisPaintCanvas) Undo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "undo", "args": []any{}})) }
func (c aylisPaintCanvas) Redo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "redo", "args": []any{}})) }
func (c aylisPaintCanvas) CanUndo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "can_undo", "args": []any{}})) }
func (c aylisPaintCanvas) CanRedo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "can_redo", "args": []any{}})) }
func (c aylisPaintCanvas) Size() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "size", "args": []any{}})) }
func (c aylisPaintCanvas) Checksum() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "checksum", "args": []any{}})) }
func (c aylisPaintCanvas) Clear() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "clear", "args": []any{}})) }
func (c aylisPaintCanvas) Resize(width int64, height int64) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "resize", "args": []any{width, height}})) }
func (c aylisPaintCanvas) New(width int64, height int64) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "new", "args": []any{width, height}})) }
func (c aylisPaintCanvas) Open(path string) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "open", "args": []any{path}})) }
func (c aylisPaintCanvas) Save(path string) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "save", "args": []any{path}})) }
func (c aylisPaintCanvas) SelectAll() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "select_all", "args": []any{}})) }
func (c aylisPaintCanvas) HasSelection() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "has_selection", "args": []any{}})) }
func (c aylisPaintCanvas) Copy() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "copy", "args": []any{}})) }
func (c aylisPaintCanvas) Cut() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "cut", "args": []any{}})) }
func (c aylisPaintCanvas) Paste() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "paste", "args": []any{}})) }
func (c aylisPaintCanvas) Delete() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "delete", "args": []any{}})) }
func (c aylisPaintCanvas) Deselect() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "deselect", "args": []any{}})) }
func (c aylisPaintCanvas) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

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

// aylisSaveDialog é um SaveDialog de um form.
type aylisSaveDialog struct{ aylis.Component }

func (c aylisSaveDialog) Title() string { return asText(c.Get("title")) }
func (c aylisSaveDialog) SetTitle(v string) { c.Set("title", v) }

func (c aylisSaveDialog) Filter() string { return asText(c.Get("filter")) }
func (c aylisSaveDialog) SetFilter(v string) { c.Set("filter", v) }

func (c aylisSaveDialog) InitialDir() string { return asText(c.Get("initial_dir")) }
func (c aylisSaveDialog) SetInitialDir(v string) { c.Set("initial_dir", v) }

func (c aylisSaveDialog) FileName() string { return asText(c.Get("file_name")) }
func (c aylisSaveDialog) SetFileName(v string) { c.Set("file_name", v) }

func (c aylisSaveDialog) Multiple() bool { return asBool(c.Get("multiple")) }
func (c aylisSaveDialog) SetMultiple(v bool) { c.Set("multiple", v) }
func (c aylisSaveDialog) Execute() { c.App.Send("execute", aylis.M{"target": c.ID}) }

// aylisScrollBox é um ScrollBox de um form.
type aylisScrollBox struct{ aylis.Component }

func (c aylisScrollBox) Horizontal() bool { return asBool(c.Get("horizontal")) }
func (c aylisScrollBox) SetHorizontal(v bool) { c.Set("horizontal", v) }

func (c aylisScrollBox) PullToRefresh() bool { return asBool(c.Get("pull_to_refresh")) }
func (c aylisScrollBox) SetPullToRefresh(v bool) { c.Set("pull_to_refresh", v) }

func (c aylisScrollBox) Refreshing() bool { return asBool(c.Get("refreshing")) }
func (c aylisScrollBox) SetRefreshing(v bool) { c.Set("refreshing", v) }

func (c aylisScrollBox) WheelStep() float64 { return asNumber(c.Get("wheel_step")) }
func (c aylisScrollBox) SetWheelStep(v float64) { c.Set("wheel_step", v) }

func (c aylisScrollBox) Smooth() string { return asText(c.Get("smooth")) }
func (c aylisScrollBox) SetSmooth(v string) { c.Set("smooth", v) }

func (c aylisScrollBox) Gutter() bool { return asBool(c.Get("gutter")) }
func (c aylisScrollBox) SetGutter(v bool) { c.Set("gutter", v) }

func (c aylisScrollBox) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisScrollBox) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisScrollBox) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisScrollBox) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisScrollBox) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisScrollBox) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisScrollBox) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisScrollBox) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisScrollBox) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisScrollBox) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisScrollBox) ScrollPosition() float64 { return asNumber(c.Get("scroll_position")) }
func (c aylisScrollBox) SetScrollPosition(v float64) { c.Set("scroll_position", v) }

func (c aylisScrollBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisScrollBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisScrollBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisScrollBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisScrollBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisScrollBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisScrollBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisScrollBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisScrollBox) Align() string { return asText(c.Get("align")) }
func (c aylisScrollBox) SetAlign(v string) { c.Set("align", v) }

func (c aylisScrollBox) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisScrollBox) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisScrollBox) Margins() string { return asText(c.Get("margins")) }
func (c aylisScrollBox) SetMargins(v string) { c.Set("margins", v) }

func (c aylisScrollBox) Padding() string { return asText(c.Get("padding")) }
func (c aylisScrollBox) SetPadding(v string) { c.Set("padding", v) }

func (c aylisScrollBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisScrollBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisScrollBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisScrollBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisScrollBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisScrollBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisScrollBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisScrollBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisScrollBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisScrollBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisScrollBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisScrollBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisScrollBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisScrollBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisScrollBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisScrollBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisScrollBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisScrollBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisScrollBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisScrollBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisScrollBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisScrollBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisScrollBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisScrollBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisScrollBox) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisScrollBox) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisScrollBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisScrollBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisScrollBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisScrollBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisScrollBox) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisScrollBox) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisScrollBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisScrollBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisScrollBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisScrollBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisScrollBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisScrollBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisScrollBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisScrollBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisScrollBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisScrollBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisScrollBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisScrollBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisScrollBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisScrollBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisScrollBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisScrollBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisScrollBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisScrollBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisScrollBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisScrollBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisScrollBox) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisScrollBox) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisScrollBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisScrollBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisScrollBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisScrollBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisScrollBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisScrollBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisScrollBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisScrollBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisScrollBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisScrollBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisScrollBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisScrollBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisScrollBox) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisScrollBox) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisScrollBox) Style() string { return asText(c.Get("style")) }
func (c aylisScrollBox) SetStyle(v string) { c.Set("style", v) }

func (c aylisScrollBox) Theme() string { return asText(c.Get("theme")) }
func (c aylisScrollBox) SetTheme(v string) { c.Set("theme", v) }

func (c aylisScrollBox) Fill() string { return asText(c.Get("fill")) }
func (c aylisScrollBox) SetFill(v string) { c.Set("fill", v) }

func (c aylisScrollBox) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisScrollBox) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisScrollBox) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisScrollBox) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisScrollBox) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisScrollBox) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisScrollBox) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisScrollBox) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisScrollBox) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisScrollBox) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisScrollBox) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisScrollBox) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisScrollBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisScrollBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisScrollBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisScrollBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisScrollBox) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisScrollBox) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisScrollBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisScrollBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisScrollBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisScrollBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisScrollBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisScrollBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisScrollBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisScrollBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisScrollBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisScrollBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisScrollBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisScrollBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisScrollBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisScrollBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisScrollBox) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisScrollBox) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisScrollBox) ScrollTo(component aylis.Ref) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "scroll_to", "args": []any{component.ComponentID()}}) }
func (c aylisScrollBox) VisibleComponents(list aylis.Ref, fully bool) string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "visible_components", "args": []any{list.ComponentID(), fully}})) }

// aylisStatusBar é um StatusBar de um form.
type aylisStatusBar struct{ aylis.Component }

func (c aylisStatusBar) Text() string { return asText(c.Get("text")) }
func (c aylisStatusBar) SetText(v string) { c.Set("text", v) }

func (c aylisStatusBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisStatusBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisStatusBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisStatusBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisStatusBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisStatusBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisStatusBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisStatusBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisStatusBar) Align() string { return asText(c.Get("align")) }
func (c aylisStatusBar) SetAlign(v string) { c.Set("align", v) }

func (c aylisStatusBar) Anchors() string { return asText(c.Get("anchors")) }
func (c aylisStatusBar) SetAnchors(v string) { c.Set("anchors", v) }

func (c aylisStatusBar) Margins() string { return asText(c.Get("margins")) }
func (c aylisStatusBar) SetMargins(v string) { c.Set("margins", v) }

func (c aylisStatusBar) Padding() string { return asText(c.Get("padding")) }
func (c aylisStatusBar) SetPadding(v string) { c.Set("padding", v) }

func (c aylisStatusBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisStatusBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisStatusBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisStatusBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisStatusBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisStatusBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisStatusBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisStatusBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisStatusBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisStatusBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisStatusBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisStatusBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisStatusBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisStatusBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisStatusBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisStatusBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisStatusBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisStatusBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisStatusBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisStatusBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisStatusBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisStatusBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisStatusBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisStatusBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisStatusBar) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisStatusBar) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisStatusBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisStatusBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisStatusBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisStatusBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisStatusBar) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisStatusBar) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisStatusBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisStatusBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisStatusBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisStatusBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisStatusBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisStatusBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisStatusBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisStatusBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisStatusBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisStatusBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisStatusBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisStatusBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisStatusBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisStatusBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisStatusBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisStatusBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisStatusBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisStatusBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisStatusBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisStatusBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisStatusBar) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisStatusBar) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisStatusBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisStatusBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisStatusBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisStatusBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisStatusBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisStatusBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisStatusBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisStatusBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisStatusBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisStatusBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisStatusBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisStatusBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisStatusBar) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisStatusBar) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisStatusBar) Style() string { return asText(c.Get("style")) }
func (c aylisStatusBar) SetStyle(v string) { c.Set("style", v) }

func (c aylisStatusBar) Theme() string { return asText(c.Get("theme")) }
func (c aylisStatusBar) SetTheme(v string) { c.Set("theme", v) }

func (c aylisStatusBar) Fill() string { return asText(c.Get("fill")) }
func (c aylisStatusBar) SetFill(v string) { c.Set("fill", v) }

func (c aylisStatusBar) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisStatusBar) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisStatusBar) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisStatusBar) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisStatusBar) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisStatusBar) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisStatusBar) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisStatusBar) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisStatusBar) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisStatusBar) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisStatusBar) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisStatusBar) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisStatusBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisStatusBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisStatusBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisStatusBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisStatusBar) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisStatusBar) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisStatusBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisStatusBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisStatusBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisStatusBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisStatusBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisStatusBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisStatusBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisStatusBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisStatusBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisStatusBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisStatusBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisStatusBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisStatusBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisStatusBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisStatusBar) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisStatusBar) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// frmPaintComponents: o form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmPaintComponents struct {
	aylisForm
	lytTopo aylisLayout
	mnuPrincipal aylisMainMenu
	mnuArquivo aylisMenuItem
	mnuNovo aylisMenuItem
	mnuAbrir aylisMenuItem
	mnuSalvar aylisMenuItem
	mnuSalvarComo aylisMenuItem
	mnuSep1 aylisMenuItem
	mnuSair aylisMenuItem
	mnuEditar aylisMenuItem
	mnuDesfazer aylisMenuItem
	mnuRefazer aylisMenuItem
	mnuSep2 aylisMenuItem
	mnuRecortar aylisMenuItem
	mnuCopiar aylisMenuItem
	mnuColar aylisMenuItem
	mnuExcluir aylisMenuItem
	mnuSelecionarTudo aylisMenuItem
	mnuSep3 aylisMenuItem
	mnuLimpar aylisMenuItem
	mnuExibir aylisMenuItem
	mnuAmpliar aylisMenuItem
	mnuReduzir aylisMenuItem
	mnuTamanhoReal aylisMenuItem
	divTopo aylisDivider
	btnSalvar aylisButton
	btnDesfazer aylisButton
	btnRefazer aylisButton
	pnlFaixa aylisPanel
	lytSelecao aylisLayout
	lblSelecaoGrupo aylisLabel
	btnSelecionar aylisButton
	divSelecao aylisDivider
	lytFerramentas aylisLayout
	lblFerramentasGrupo aylisLabel
	grdFerramentas aylisGridLayout
	btnLapis aylisButton
	btnBalde aylisButton
	btnBorracha aylisButton
	btnContaGotas aylisButton
	divFerramentas aylisDivider
	lytPinceis aylisLayout
	lblPinceis aylisLabel
	btnPincel aylisButton
	divPinceis aylisDivider
	lytFormas aylisLayout
	lblFormas aylisLabel
	chkPreencher aylisCheckBox
	grdFormas aylisGridLayout
	btnLinha aylisButton
	btnRetangulo aylisButton
	btnElipse aylisButton
	divFormas aylisDivider
	lytTamanho aylisLayout
	lblTamanhoGrupo aylisLabel
	cbEspessura aylisComboBox
	divTamanho aylisDivider
	lytCores aylisLayout
	lblCores aylisLabel
	btnCor1 aylisCircle
	btnCor2 aylisCircle
	grdPaleta aylisGridLayout
	btnPaleta01 aylisCircle
	btnPaleta02 aylisCircle
	btnPaleta03 aylisCircle
	btnPaleta04 aylisCircle
	btnPaleta05 aylisCircle
	btnPaleta06 aylisCircle
	btnPaleta07 aylisCircle
	btnPaleta08 aylisCircle
	btnPaleta09 aylisCircle
	btnPaleta10 aylisCircle
	btnPaleta11 aylisCircle
	btnPaleta12 aylisCircle
	btnPaleta13 aylisCircle
	btnPaleta14 aylisCircle
	btnPaleta15 aylisCircle
	btnPaleta16 aylisCircle
	btnPaleta17 aylisCircle
	btnPaleta18 aylisCircle
	btnPaleta19 aylisCircle
	btnPaleta20 aylisCircle
	stbStatus aylisStatusBar
	lblPosicao aylisLabel
	lblTamanho aylisLabel
	lblZoom aylisLabel
	sbxArea aylisScrollBox
	pcDesenho aylisPaintCanvas
	dlgAbrir aylisOpenDialog
	dlgSalvar aylisSaveDialog
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmPaintComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "lytTopo", "Layout"); err != nil {
		return err
	} else {
		f.lytTopo = aylisLayout{c}
	}
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
	if c, err := f.Resolve(list, "mnuNovo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuNovo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAbrir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAbrir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSalvar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSalvar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSalvarComo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSalvarComo = aylisMenuItem{c}
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
	if c, err := f.Resolve(list, "mnuEditar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuEditar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuDesfazer", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuDesfazer = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRefazer", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRefazer = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep2", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep2 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRecortar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRecortar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuCopiar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuCopiar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuColar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuColar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExcluir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExcluir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSelecionarTudo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSelecionarTudo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep3", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep3 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuLimpar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuLimpar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExibir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExibir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAmpliar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAmpliar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuReduzir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReduzir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuTamanhoReal", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuTamanhoReal = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "divTopo", "Divider"); err != nil {
		return err
	} else {
		f.divTopo = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "btnSalvar", "Button"); err != nil {
		return err
	} else {
		f.btnSalvar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnDesfazer", "Button"); err != nil {
		return err
	} else {
		f.btnDesfazer = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRefazer", "Button"); err != nil {
		return err
	} else {
		f.btnRefazer = aylisButton{c}
	}
	if c, err := f.Resolve(list, "pnlFaixa", "Panel"); err != nil {
		return err
	} else {
		f.pnlFaixa = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lytSelecao", "Layout"); err != nil {
		return err
	} else {
		f.lytSelecao = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblSelecaoGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblSelecaoGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnSelecionar", "Button"); err != nil {
		return err
	} else {
		f.btnSelecionar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divSelecao", "Divider"); err != nil {
		return err
	} else {
		f.divSelecao = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytFerramentas", "Layout"); err != nil {
		return err
	} else {
		f.lytFerramentas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblFerramentasGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblFerramentasGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "grdFerramentas", "GridLayout"); err != nil {
		return err
	} else {
		f.grdFerramentas = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnLapis", "Button"); err != nil {
		return err
	} else {
		f.btnLapis = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnBalde", "Button"); err != nil {
		return err
	} else {
		f.btnBalde = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnBorracha", "Button"); err != nil {
		return err
	} else {
		f.btnBorracha = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnContaGotas", "Button"); err != nil {
		return err
	} else {
		f.btnContaGotas = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divFerramentas", "Divider"); err != nil {
		return err
	} else {
		f.divFerramentas = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytPinceis", "Layout"); err != nil {
		return err
	} else {
		f.lytPinceis = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblPinceis", "Label"); err != nil {
		return err
	} else {
		f.lblPinceis = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnPincel", "Button"); err != nil {
		return err
	} else {
		f.btnPincel = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divPinceis", "Divider"); err != nil {
		return err
	} else {
		f.divPinceis = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytFormas", "Layout"); err != nil {
		return err
	} else {
		f.lytFormas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblFormas", "Label"); err != nil {
		return err
	} else {
		f.lblFormas = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "chkPreencher", "CheckBox"); err != nil {
		return err
	} else {
		f.chkPreencher = aylisCheckBox{c}
	}
	if c, err := f.Resolve(list, "grdFormas", "GridLayout"); err != nil {
		return err
	} else {
		f.grdFormas = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnLinha", "Button"); err != nil {
		return err
	} else {
		f.btnLinha = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRetangulo", "Button"); err != nil {
		return err
	} else {
		f.btnRetangulo = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnElipse", "Button"); err != nil {
		return err
	} else {
		f.btnElipse = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divFormas", "Divider"); err != nil {
		return err
	} else {
		f.divFormas = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytTamanho", "Layout"); err != nil {
		return err
	} else {
		f.lytTamanho = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTamanhoGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblTamanhoGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "cbEspessura", "ComboBox"); err != nil {
		return err
	} else {
		f.cbEspessura = aylisComboBox{c}
	}
	if c, err := f.Resolve(list, "divTamanho", "Divider"); err != nil {
		return err
	} else {
		f.divTamanho = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytCores", "Layout"); err != nil {
		return err
	} else {
		f.lytCores = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblCores", "Label"); err != nil {
		return err
	} else {
		f.lblCores = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnCor1", "Circle"); err != nil {
		return err
	} else {
		f.btnCor1 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnCor2", "Circle"); err != nil {
		return err
	} else {
		f.btnCor2 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "grdPaleta", "GridLayout"); err != nil {
		return err
	} else {
		f.grdPaleta = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnPaleta01", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta01 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta02", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta02 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta03", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta03 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta04", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta04 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta05", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta05 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta06", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta06 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta07", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta07 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta08", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta08 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta09", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta09 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta10", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta10 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta11", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta11 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta12", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta12 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta13", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta13 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta14", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta14 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta15", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta15 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta16", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta16 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta17", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta17 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta18", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta18 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta19", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta19 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta20", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta20 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "stbStatus", "StatusBar"); err != nil {
		return err
	} else {
		f.stbStatus = aylisStatusBar{c}
	}
	if c, err := f.Resolve(list, "lblPosicao", "Label"); err != nil {
		return err
	} else {
		f.lblPosicao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblTamanho", "Label"); err != nil {
		return err
	} else {
		f.lblTamanho = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblZoom", "Label"); err != nil {
		return err
	} else {
		f.lblZoom = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "sbxArea", "ScrollBox"); err != nil {
		return err
	} else {
		f.sbxArea = aylisScrollBox{c}
	}
	if c, err := f.Resolve(list, "pcDesenho", "PaintCanvas"); err != nil {
		return err
	} else {
		f.pcDesenho = aylisPaintCanvas{c}
	}
	if c, err := f.Resolve(list, "dlgAbrir", "OpenDialog"); err != nil {
		return err
	} else {
		f.dlgAbrir = aylisOpenDialog{c}
	}
	if c, err := f.Resolve(list, "dlgSalvar", "SaveDialog"); err != nil {
		return err
	} else {
		f.dlgSalvar = aylisSaveDialog{c}
	}
	return nil
}

// frmPaint é a janela de frmPaint cujo evento está rodando (a primeira, fora dos handlers).
var frmPaint *frmPaintComponents

func init() {
	aylis.RegisterFormClass("frmPaint", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmPaintComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmPaint = form.(*frmPaintComponents) })
}

// FindButton é o Button deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnSalvar":
		return f.btnSalvar
	case "btnDesfazer":
		return f.btnDesfazer
	case "btnRefazer":
		return f.btnRefazer
	case "btnSelecionar":
		return f.btnSelecionar
	case "btnLapis":
		return f.btnLapis
	case "btnBalde":
		return f.btnBalde
	case "btnBorracha":
		return f.btnBorracha
	case "btnContaGotas":
		return f.btnContaGotas
	case "btnPincel":
		return f.btnPincel
	case "btnLinha":
		return f.btnLinha
	case "btnRetangulo":
		return f.btnRetangulo
	case "btnElipse":
		return f.btnElipse
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmPaintComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindCheckBox é o CheckBox deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindCheckBox(name string) aylisCheckBox {
	switch name {
	case "chkPreencher":
		return f.chkPreencher
	}
	return aylisCheckBox{}
}

// SenderAsCheckBox é o CheckBox que disparou o evento.
func (f *frmPaintComponents) SenderAsCheckBox(ev aylis.Event) aylisCheckBox { return f.FindCheckBox(ev.Sender.Name) }

// FindCircle é o Circle deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindCircle(name string) aylisCircle {
	switch name {
	case "btnCor1":
		return f.btnCor1
	case "btnCor2":
		return f.btnCor2
	case "btnPaleta01":
		return f.btnPaleta01
	case "btnPaleta02":
		return f.btnPaleta02
	case "btnPaleta03":
		return f.btnPaleta03
	case "btnPaleta04":
		return f.btnPaleta04
	case "btnPaleta05":
		return f.btnPaleta05
	case "btnPaleta06":
		return f.btnPaleta06
	case "btnPaleta07":
		return f.btnPaleta07
	case "btnPaleta08":
		return f.btnPaleta08
	case "btnPaleta09":
		return f.btnPaleta09
	case "btnPaleta10":
		return f.btnPaleta10
	case "btnPaleta11":
		return f.btnPaleta11
	case "btnPaleta12":
		return f.btnPaleta12
	case "btnPaleta13":
		return f.btnPaleta13
	case "btnPaleta14":
		return f.btnPaleta14
	case "btnPaleta15":
		return f.btnPaleta15
	case "btnPaleta16":
		return f.btnPaleta16
	case "btnPaleta17":
		return f.btnPaleta17
	case "btnPaleta18":
		return f.btnPaleta18
	case "btnPaleta19":
		return f.btnPaleta19
	case "btnPaleta20":
		return f.btnPaleta20
	}
	return aylisCircle{}
}

// SenderAsCircle é o Circle que disparou o evento.
func (f *frmPaintComponents) SenderAsCircle(ev aylis.Event) aylisCircle { return f.FindCircle(ev.Sender.Name) }

// FindComboBox é o ComboBox deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindComboBox(name string) aylisComboBox {
	switch name {
	case "cbEspessura":
		return f.cbEspessura
	}
	return aylisComboBox{}
}

// SenderAsComboBox é o ComboBox que disparou o evento.
func (f *frmPaintComponents) SenderAsComboBox(ev aylis.Event) aylisComboBox { return f.FindComboBox(ev.Sender.Name) }

// FindDivider é o Divider deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindDivider(name string) aylisDivider {
	switch name {
	case "divTopo":
		return f.divTopo
	case "divSelecao":
		return f.divSelecao
	case "divFerramentas":
		return f.divFerramentas
	case "divPinceis":
		return f.divPinceis
	case "divFormas":
		return f.divFormas
	case "divTamanho":
		return f.divTamanho
	}
	return aylisDivider{}
}

// SenderAsDivider é o Divider que disparou o evento.
func (f *frmPaintComponents) SenderAsDivider(ev aylis.Event) aylisDivider { return f.FindDivider(ev.Sender.Name) }

// FindGridLayout é o GridLayout deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindGridLayout(name string) aylisGridLayout {
	switch name {
	case "grdFerramentas":
		return f.grdFerramentas
	case "grdFormas":
		return f.grdFormas
	case "grdPaleta":
		return f.grdPaleta
	}
	return aylisGridLayout{}
}

// SenderAsGridLayout é o GridLayout que disparou o evento.
func (f *frmPaintComponents) SenderAsGridLayout(ev aylis.Event) aylisGridLayout { return f.FindGridLayout(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblSelecaoGrupo":
		return f.lblSelecaoGrupo
	case "lblFerramentasGrupo":
		return f.lblFerramentasGrupo
	case "lblPinceis":
		return f.lblPinceis
	case "lblFormas":
		return f.lblFormas
	case "lblTamanhoGrupo":
		return f.lblTamanhoGrupo
	case "lblCores":
		return f.lblCores
	case "lblPosicao":
		return f.lblPosicao
	case "lblTamanho":
		return f.lblTamanho
	case "lblZoom":
		return f.lblZoom
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmPaintComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytTopo":
		return f.lytTopo
	case "lytSelecao":
		return f.lytSelecao
	case "lytFerramentas":
		return f.lytFerramentas
	case "lytPinceis":
		return f.lytPinceis
	case "lytFormas":
		return f.lytFormas
	case "lytTamanho":
		return f.lytTamanho
	case "lytCores":
		return f.lytCores
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmPaintComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindMainMenu é o MainMenu deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindMainMenu(name string) aylisMainMenu {
	switch name {
	case "mnuPrincipal":
		return f.mnuPrincipal
	}
	return aylisMainMenu{}
}

// SenderAsMainMenu é o MainMenu que disparou o evento.
func (f *frmPaintComponents) SenderAsMainMenu(ev aylis.Event) aylisMainMenu { return f.FindMainMenu(ev.Sender.Name) }

// FindMenuItem é o MenuItem deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindMenuItem(name string) aylisMenuItem {
	switch name {
	case "mnuArquivo":
		return f.mnuArquivo
	case "mnuNovo":
		return f.mnuNovo
	case "mnuAbrir":
		return f.mnuAbrir
	case "mnuSalvar":
		return f.mnuSalvar
	case "mnuSalvarComo":
		return f.mnuSalvarComo
	case "mnuSep1":
		return f.mnuSep1
	case "mnuSair":
		return f.mnuSair
	case "mnuEditar":
		return f.mnuEditar
	case "mnuDesfazer":
		return f.mnuDesfazer
	case "mnuRefazer":
		return f.mnuRefazer
	case "mnuSep2":
		return f.mnuSep2
	case "mnuRecortar":
		return f.mnuRecortar
	case "mnuCopiar":
		return f.mnuCopiar
	case "mnuColar":
		return f.mnuColar
	case "mnuExcluir":
		return f.mnuExcluir
	case "mnuSelecionarTudo":
		return f.mnuSelecionarTudo
	case "mnuSep3":
		return f.mnuSep3
	case "mnuLimpar":
		return f.mnuLimpar
	case "mnuExibir":
		return f.mnuExibir
	case "mnuAmpliar":
		return f.mnuAmpliar
	case "mnuReduzir":
		return f.mnuReduzir
	case "mnuTamanhoReal":
		return f.mnuTamanhoReal
	}
	return aylisMenuItem{}
}

// SenderAsMenuItem é o MenuItem que disparou o evento.
func (f *frmPaintComponents) SenderAsMenuItem(ev aylis.Event) aylisMenuItem { return f.FindMenuItem(ev.Sender.Name) }

// FindOpenDialog é o OpenDialog deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindOpenDialog(name string) aylisOpenDialog {
	switch name {
	case "dlgAbrir":
		return f.dlgAbrir
	}
	return aylisOpenDialog{}
}

// SenderAsOpenDialog é o OpenDialog que disparou o evento.
func (f *frmPaintComponents) SenderAsOpenDialog(ev aylis.Event) aylisOpenDialog { return f.FindOpenDialog(ev.Sender.Name) }

// FindPaintCanvas é o PaintCanvas deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindPaintCanvas(name string) aylisPaintCanvas {
	switch name {
	case "pcDesenho":
		return f.pcDesenho
	}
	return aylisPaintCanvas{}
}

// SenderAsPaintCanvas é o PaintCanvas que disparou o evento.
func (f *frmPaintComponents) SenderAsPaintCanvas(ev aylis.Event) aylisPaintCanvas { return f.FindPaintCanvas(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlFaixa":
		return f.pnlFaixa
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmPaintComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSaveDialog é o SaveDialog deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindSaveDialog(name string) aylisSaveDialog {
	switch name {
	case "dlgSalvar":
		return f.dlgSalvar
	}
	return aylisSaveDialog{}
}

// SenderAsSaveDialog é o SaveDialog que disparou o evento.
func (f *frmPaintComponents) SenderAsSaveDialog(ev aylis.Event) aylisSaveDialog { return f.FindSaveDialog(ev.Sender.Name) }

// FindScrollBox é o ScrollBox deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindScrollBox(name string) aylisScrollBox {
	switch name {
	case "sbxArea":
		return f.sbxArea
	}
	return aylisScrollBox{}
}

// SenderAsScrollBox é o ScrollBox que disparou o evento.
func (f *frmPaintComponents) SenderAsScrollBox(ev aylis.Event) aylisScrollBox { return f.FindScrollBox(ev.Sender.Name) }

// FindStatusBar é o StatusBar deste form com o nome (como o FindComponent).
func (f *frmPaintComponents) FindStatusBar(name string) aylisStatusBar {
	switch name {
	case "stbStatus":
		return f.stbStatus
	}
	return aylisStatusBar{}
}

// SenderAsStatusBar é o StatusBar que disparou o evento.
func (f *frmPaintComponents) SenderAsStatusBar(ev aylis.Event) aylisStatusBar { return f.FindStatusBar(ev.Sender.Name) }
