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

func (c aylisButton) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisButton) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisButton) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisButton) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisButton) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisButton) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisButton) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisButton) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisButton) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisButton) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisButton) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

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

func (c aylisForm) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisForm) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisForm) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisForm) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisForm) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisForm) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisForm) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisForm) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisForm) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisForm) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisForm) MoveToMonitor(a1 int64) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "move_to_monitor", "args": []any{a1}}) }
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

func (c aylisGridLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisGridLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisGridLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisGridLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisGridLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisGridLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisGridLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisGridLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisLabel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLabel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLabel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLabel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLabel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLabel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLabel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLabel) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisLabel) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisLabel) SetPopupMenu(v string) { c.Set("popup_menu", v) }

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

func (c aylisLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisListView) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisListView) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisListView) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisListView) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisListView) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisListView) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisListView) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisListView) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisListView) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisListView) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisListView) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

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

func (c aylisPanel) Radius() float64 { return asNumber(c.Get("radius")) }
func (c aylisPanel) SetRadius(v float64) { c.Set("radius", v) }

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

func (c aylisPanel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisPanel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisPanel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisPanel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisPanel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisPanel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisPanel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisPanel) SetClipChildren(v bool) { c.Set("clip_children", v) }

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

func (c aylisPanel) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisPanel) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisTabControl é um TabControl de um form.
type aylisTabControl struct{ aylis.Component }

func (c aylisTabControl) TabIndex() int64 { return int64(asNumber(c.Get("tab_index"))) }
func (c aylisTabControl) SetTabIndex(v int64) { c.Set("tab_index", v) }

func (c aylisTabControl) ShowTabs() bool { return asBool(c.Get("show_tabs")) }
func (c aylisTabControl) SetShowTabs(v bool) { c.Set("show_tabs", v) }

func (c aylisTabControl) Transition() string { return asText(c.Get("transition")) }
func (c aylisTabControl) SetTransition(v string) { c.Set("transition", v) }

func (c aylisTabControl) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTabControl) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTabControl) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTabControl) SetTop(v float64) { c.Set("top", v) }

func (c aylisTabControl) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTabControl) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTabControl) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTabControl) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTabControl) Align() string { return asText(c.Get("align")) }
func (c aylisTabControl) SetAlign(v string) { c.Set("align", v) }

func (c aylisTabControl) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTabControl) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTabControl) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTabControl) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTabControl) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTabControl) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTabControl) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTabControl) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTabControl) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTabControl) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTabControl) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTabControl) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTabControl) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTabControl) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTabControl) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTabControl) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTabControl) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTabControl) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTabControl) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTabControl) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTabControl) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisTabControl) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisTabControl) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTabControl) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTabControl) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTabControl) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTabControl) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisTabControl) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisTabControl) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTabControl) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTabControl) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTabControl) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTabControl) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTabControl) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTabControl) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTabControl) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTabControl) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTabControl) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTabControl) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTabControl) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTabControl) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTabControl) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTabControl) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTabControl) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTabControl) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTabControl) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTabControl) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTabControl) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTabControl) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisTabControl) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisTabControl) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTabControl) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTabControl) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTabControl) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTabControl) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisTabControl) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisTabControl) Style() string { return asText(c.Get("style")) }
func (c aylisTabControl) SetStyle(v string) { c.Set("style", v) }

func (c aylisTabControl) Theme() string { return asText(c.Get("theme")) }
func (c aylisTabControl) SetTheme(v string) { c.Set("theme", v) }

func (c aylisTabControl) Fill() string { return asText(c.Get("fill")) }
func (c aylisTabControl) SetFill(v string) { c.Set("fill", v) }

func (c aylisTabControl) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisTabControl) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisTabControl) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisTabControl) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisTabControl) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisTabControl) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisTabControl) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisTabControl) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisTabControl) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisTabControl) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisTabControl) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisTabControl) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisTabControl) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTabControl) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTabControl) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTabControl) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTabControl) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisTabControl) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisTabControl) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTabControl) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTabControl) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTabControl) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTabControl) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTabControl) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTabControl) Hint() string { return asText(c.Get("hint")) }
func (c aylisTabControl) SetHint(v string) { c.Set("hint", v) }

func (c aylisTabControl) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTabControl) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTabControl) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisTabControl) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisTabControl) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisTabItem é um TabItem de um form.
type aylisTabItem struct{ aylis.Component }

func (c aylisTabItem) Caption() string { return asText(c.Get("caption")) }
func (c aylisTabItem) SetCaption(v string) { c.Set("caption", v) }

func (c aylisTabItem) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTabItem) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTabItem) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTabItem) SetTop(v float64) { c.Set("top", v) }

func (c aylisTabItem) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTabItem) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTabItem) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTabItem) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTabItem) Align() string { return asText(c.Get("align")) }
func (c aylisTabItem) SetAlign(v string) { c.Set("align", v) }

func (c aylisTabItem) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTabItem) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTabItem) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTabItem) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTabItem) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTabItem) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTabItem) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTabItem) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTabItem) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTabItem) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTabItem) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTabItem) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTabItem) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTabItem) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTabItem) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTabItem) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTabItem) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTabItem) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTabItem) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTabItem) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTabItem) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisTabItem) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisTabItem) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTabItem) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTabItem) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTabItem) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTabItem) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisTabItem) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisTabItem) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTabItem) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTabItem) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTabItem) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTabItem) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTabItem) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTabItem) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTabItem) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTabItem) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTabItem) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTabItem) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTabItem) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTabItem) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTabItem) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTabItem) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTabItem) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTabItem) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTabItem) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTabItem) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTabItem) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTabItem) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisTabItem) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisTabItem) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTabItem) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTabItem) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTabItem) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTabItem) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisTabItem) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisTabItem) Style() string { return asText(c.Get("style")) }
func (c aylisTabItem) SetStyle(v string) { c.Set("style", v) }

func (c aylisTabItem) Theme() string { return asText(c.Get("theme")) }
func (c aylisTabItem) SetTheme(v string) { c.Set("theme", v) }

func (c aylisTabItem) Fill() string { return asText(c.Get("fill")) }
func (c aylisTabItem) SetFill(v string) { c.Set("fill", v) }

func (c aylisTabItem) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisTabItem) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisTabItem) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisTabItem) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisTabItem) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisTabItem) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisTabItem) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisTabItem) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisTabItem) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisTabItem) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisTabItem) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisTabItem) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisTabItem) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTabItem) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTabItem) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTabItem) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTabItem) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisTabItem) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisTabItem) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTabItem) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTabItem) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTabItem) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTabItem) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTabItem) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTabItem) Hint() string { return asText(c.Get("hint")) }
func (c aylisTabItem) SetHint(v string) { c.Set("hint", v) }

func (c aylisTabItem) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTabItem) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTabItem) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisTabItem) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// frmCalculadoraComponents: o form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmCalculadoraComponents struct {
	aylisForm
	pnlLateral aylisPanel
	tabLateral aylisTabControl
	tabHistorico aylisTabItem
	btnLimparHistorico aylisButton
	lvHistorico aylisListView
	tabMemoria aylisTabItem
	lvMemoria aylisListView
	lytCorpo aylisLayout
	lytTopo aylisLayout
	btnHistorico aylisButton
	lblTitulo aylisLabel
	lblExpressao aylisLabel
	lblVisor aylisLabel
	grdMemoria aylisGridLayout
	btnMC aylisButton
	btnMR aylisButton
	btnMMais aylisButton
	btnMMenos aylisButton
	btnMS aylisButton
	grdTeclado aylisGridLayout
	btnPorcento aylisButton
	btnCE aylisButton
	btnC aylisButton
	btnApagar aylisButton
	btnInverso aylisButton
	btnQuadrado aylisButton
	btnRaiz aylisButton
	btnDividir aylisButton
	btn7 aylisButton
	btn8 aylisButton
	btn9 aylisButton
	btnMultiplicar aylisButton
	btn4 aylisButton
	btn5 aylisButton
	btn6 aylisButton
	btnSubtrair aylisButton
	btn1 aylisButton
	btn2 aylisButton
	btn3 aylisButton
	btnSomar aylisButton
	btnSinal aylisButton
	btn0 aylisButton
	btnVirgula aylisButton
	btnIgual aylisButton
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmCalculadoraComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "pnlLateral", "Panel"); err != nil {
		return err
	} else {
		f.pnlLateral = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "tabLateral", "TabControl"); err != nil {
		return err
	} else {
		f.tabLateral = aylisTabControl{c}
	}
	if c, err := f.Resolve(list, "tabHistorico", "TabItem"); err != nil {
		return err
	} else {
		f.tabHistorico = aylisTabItem{c}
	}
	if c, err := f.Resolve(list, "btnLimparHistorico", "Button"); err != nil {
		return err
	} else {
		f.btnLimparHistorico = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lvHistorico", "ListView"); err != nil {
		return err
	} else {
		f.lvHistorico = aylisListView{c}
	}
	if c, err := f.Resolve(list, "tabMemoria", "TabItem"); err != nil {
		return err
	} else {
		f.tabMemoria = aylisTabItem{c}
	}
	if c, err := f.Resolve(list, "lvMemoria", "ListView"); err != nil {
		return err
	} else {
		f.lvMemoria = aylisListView{c}
	}
	if c, err := f.Resolve(list, "lytCorpo", "Layout"); err != nil {
		return err
	} else {
		f.lytCorpo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lytTopo", "Layout"); err != nil {
		return err
	} else {
		f.lytTopo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnHistorico", "Button"); err != nil {
		return err
	} else {
		f.btnHistorico = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lblTitulo", "Label"); err != nil {
		return err
	} else {
		f.lblTitulo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblExpressao", "Label"); err != nil {
		return err
	} else {
		f.lblExpressao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblVisor", "Label"); err != nil {
		return err
	} else {
		f.lblVisor = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "grdMemoria", "GridLayout"); err != nil {
		return err
	} else {
		f.grdMemoria = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnMC", "Button"); err != nil {
		return err
	} else {
		f.btnMC = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMR", "Button"); err != nil {
		return err
	} else {
		f.btnMR = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMMais", "Button"); err != nil {
		return err
	} else {
		f.btnMMais = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMMenos", "Button"); err != nil {
		return err
	} else {
		f.btnMMenos = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMS", "Button"); err != nil {
		return err
	} else {
		f.btnMS = aylisButton{c}
	}
	if c, err := f.Resolve(list, "grdTeclado", "GridLayout"); err != nil {
		return err
	} else {
		f.grdTeclado = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnPorcento", "Button"); err != nil {
		return err
	} else {
		f.btnPorcento = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnCE", "Button"); err != nil {
		return err
	} else {
		f.btnCE = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnC", "Button"); err != nil {
		return err
	} else {
		f.btnC = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnApagar", "Button"); err != nil {
		return err
	} else {
		f.btnApagar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnInverso", "Button"); err != nil {
		return err
	} else {
		f.btnInverso = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnQuadrado", "Button"); err != nil {
		return err
	} else {
		f.btnQuadrado = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRaiz", "Button"); err != nil {
		return err
	} else {
		f.btnRaiz = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnDividir", "Button"); err != nil {
		return err
	} else {
		f.btnDividir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn7", "Button"); err != nil {
		return err
	} else {
		f.btn7 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn8", "Button"); err != nil {
		return err
	} else {
		f.btn8 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn9", "Button"); err != nil {
		return err
	} else {
		f.btn9 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMultiplicar", "Button"); err != nil {
		return err
	} else {
		f.btnMultiplicar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn4", "Button"); err != nil {
		return err
	} else {
		f.btn4 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn5", "Button"); err != nil {
		return err
	} else {
		f.btn5 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn6", "Button"); err != nil {
		return err
	} else {
		f.btn6 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSubtrair", "Button"); err != nil {
		return err
	} else {
		f.btnSubtrair = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn1", "Button"); err != nil {
		return err
	} else {
		f.btn1 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn2", "Button"); err != nil {
		return err
	} else {
		f.btn2 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn3", "Button"); err != nil {
		return err
	} else {
		f.btn3 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSomar", "Button"); err != nil {
		return err
	} else {
		f.btnSomar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSinal", "Button"); err != nil {
		return err
	} else {
		f.btnSinal = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn0", "Button"); err != nil {
		return err
	} else {
		f.btn0 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnVirgula", "Button"); err != nil {
		return err
	} else {
		f.btnVirgula = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnIgual", "Button"); err != nil {
		return err
	} else {
		f.btnIgual = aylisButton{c}
	}
	return nil
}

// frmCalculadora é a janela de frmCalculadora cujo evento está rodando (a primeira, fora dos handlers).
var frmCalculadora *frmCalculadoraComponents

func init() {
	aylis.RegisterFormClass("frmCalculadora", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmCalculadoraComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmCalculadora = form.(*frmCalculadoraComponents) })
}

// FindButton é o Button deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnLimparHistorico":
		return f.btnLimparHistorico
	case "btnHistorico":
		return f.btnHistorico
	case "btnMC":
		return f.btnMC
	case "btnMR":
		return f.btnMR
	case "btnMMais":
		return f.btnMMais
	case "btnMMenos":
		return f.btnMMenos
	case "btnMS":
		return f.btnMS
	case "btnPorcento":
		return f.btnPorcento
	case "btnCE":
		return f.btnCE
	case "btnC":
		return f.btnC
	case "btnApagar":
		return f.btnApagar
	case "btnInverso":
		return f.btnInverso
	case "btnQuadrado":
		return f.btnQuadrado
	case "btnRaiz":
		return f.btnRaiz
	case "btnDividir":
		return f.btnDividir
	case "btn7":
		return f.btn7
	case "btn8":
		return f.btn8
	case "btn9":
		return f.btn9
	case "btnMultiplicar":
		return f.btnMultiplicar
	case "btn4":
		return f.btn4
	case "btn5":
		return f.btn5
	case "btn6":
		return f.btn6
	case "btnSubtrair":
		return f.btnSubtrair
	case "btn1":
		return f.btn1
	case "btn2":
		return f.btn2
	case "btn3":
		return f.btn3
	case "btnSomar":
		return f.btnSomar
	case "btnSinal":
		return f.btnSinal
	case "btn0":
		return f.btn0
	case "btnVirgula":
		return f.btnVirgula
	case "btnIgual":
		return f.btnIgual
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindGridLayout é o GridLayout deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindGridLayout(name string) aylisGridLayout {
	switch name {
	case "grdMemoria":
		return f.grdMemoria
	case "grdTeclado":
		return f.grdTeclado
	}
	return aylisGridLayout{}
}

// SenderAsGridLayout é o GridLayout que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsGridLayout(ev aylis.Event) aylisGridLayout { return f.FindGridLayout(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblTitulo":
		return f.lblTitulo
	case "lblExpressao":
		return f.lblExpressao
	case "lblVisor":
		return f.lblVisor
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytCorpo":
		return f.lytCorpo
	case "lytTopo":
		return f.lytTopo
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindListView é o ListView deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindListView(name string) aylisListView {
	switch name {
	case "lvHistorico":
		return f.lvHistorico
	case "lvMemoria":
		return f.lvMemoria
	}
	return aylisListView{}
}

// SenderAsListView é o ListView que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsListView(ev aylis.Event) aylisListView { return f.FindListView(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlLateral":
		return f.pnlLateral
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindTabControl é o TabControl deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindTabControl(name string) aylisTabControl {
	switch name {
	case "tabLateral":
		return f.tabLateral
	}
	return aylisTabControl{}
}

// SenderAsTabControl é o TabControl que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsTabControl(ev aylis.Event) aylisTabControl { return f.FindTabControl(ev.Sender.Name) }

// FindTabItem é o TabItem deste form com o nome (como o FindComponent).
func (f *frmCalculadoraComponents) FindTabItem(name string) aylisTabItem {
	switch name {
	case "tabHistorico":
		return f.tabHistorico
	case "tabMemoria":
		return f.tabMemoria
	}
	return aylisTabItem{}
}

// SenderAsTabItem é o TabItem que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsTabItem(ev aylis.Event) aylisTabItem { return f.FindTabItem(ev.Sender.Name) }
