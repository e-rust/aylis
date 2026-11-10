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

// aylisClientDataSet é um ClientDataSet de um form.
type aylisClientDataSet struct{ aylis.Component }

func (c aylisClientDataSet) KeyFields() string { return asText(c.Get("key_fields")) }
func (c aylisClientDataSet) SetKeyFields(v string) { c.Set("key_fields", v) }

func (c aylisClientDataSet) ApplyMode() string { return asText(c.Get("apply_mode")) }
func (c aylisClientDataSet) SetApplyMode(v string) { c.Set("apply_mode", v) }

func (c aylisClientDataSet) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisClientDataSet) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisClientDataSet) Active() bool { return asBool(c.Get("active")) }
func (c aylisClientDataSet) SetActive(v bool) { c.Set("active", v) }

func (c aylisClientDataSet) Filter() string { return asText(c.Get("filter")) }
func (c aylisClientDataSet) SetFilter(v string) { c.Set("filter", v) }

func (c aylisClientDataSet) Filtered() bool { return asBool(c.Get("filtered")) }
func (c aylisClientDataSet) SetFiltered(v bool) { c.Set("filtered", v) }

func (c aylisClientDataSet) IndexFieldNames() string { return asText(c.Get("index_field_names")) }
func (c aylisClientDataSet) SetIndexFieldNames(v string) { c.Set("index_field_names", v) }

func (c aylisClientDataSet) Indexes() string { return asText(c.Get("indexes")) }
func (c aylisClientDataSet) SetIndexes(v string) { c.Set("indexes", v) }

func (c aylisClientDataSet) IndexName() string { return asText(c.Get("index_name")) }
func (c aylisClientDataSet) SetIndexName(v string) { c.Set("index_name", v) }

func (c aylisClientDataSet) Params() string { return asText(c.Get("params")) }
func (c aylisClientDataSet) SetParams(v string) { c.Set("params", v) }

func (c aylisClientDataSet) StatusFilter() string { return asText(c.Get("status_filter")) }
func (c aylisClientDataSet) SetStatusFilter(v string) { c.Set("status_filter", v) }

func (c aylisClientDataSet) MasterSource() string { return asText(c.Get("master_source")) }
func (c aylisClientDataSet) SetMasterSource(v string) { c.Set("master_source", v) }

func (c aylisClientDataSet) MasterFields() string { return asText(c.Get("master_fields")) }
func (c aylisClientDataSet) SetMasterFields(v string) { c.Set("master_fields", v) }

func (c aylisClientDataSet) ApplySeparately() bool { return asBool(c.Get("apply_separately")) }
func (c aylisClientDataSet) SetApplySeparately(v bool) { c.Set("apply_separately", v) }

func (c aylisClientDataSet) RecordTotal() int64 { return int64(asNumber(c.Get("record_total"))) }
func (c aylisClientDataSet) SetRecordTotal(v int64) { c.Set("record_total", v) }

func (c aylisClientDataSet) MaxRecords() int64 { return int64(asNumber(c.Get("max_records"))) }
func (c aylisClientDataSet) SetMaxRecords(v int64) { c.Set("max_records", v) }

func (c aylisClientDataSet) PartialNotice() string { return asText(c.Get("partial_notice")) }

// aylisDBChart é um DBChart de um form.
type aylisDBChart struct{ aylis.Component }

func (c aylisDBChart) DataSource() string { return asText(c.Get("data_source")) }
func (c aylisDBChart) SetDataSource(v string) { c.Set("data_source", v) }

func (c aylisDBChart) LabelField() string { return asText(c.Get("label_field")) }
func (c aylisDBChart) SetLabelField(v string) { c.Set("label_field", v) }

func (c aylisDBChart) ValueFields() string { return asText(c.Get("value_fields")) }
func (c aylisDBChart) SetValueFields(v string) { c.Set("value_fields", v) }

func (c aylisDBChart) Kind() string { return asText(c.Get("kind")) }
func (c aylisDBChart) SetKind(v string) { c.Set("kind", v) }

func (c aylisDBChart) Title() string { return asText(c.Get("title")) }
func (c aylisDBChart) SetTitle(v string) { c.Set("title", v) }

func (c aylisDBChart) Labels() string { return asText(c.Get("labels")) }
func (c aylisDBChart) SetLabels(v string) { c.Set("labels", v) }

func (c aylisDBChart) Series() string { return asText(c.Get("series")) }
func (c aylisDBChart) SetSeries(v string) { c.Set("series", v) }

func (c aylisDBChart) Json() string { return asText(c.Get("json")) }
func (c aylisDBChart) SetJson(v string) { c.Set("json", v) }

func (c aylisDBChart) Legend() bool { return asBool(c.Get("legend")) }
func (c aylisDBChart) SetLegend(v bool) { c.Set("legend", v) }

func (c aylisDBChart) Stepped() bool { return asBool(c.Get("stepped")) }
func (c aylisDBChart) SetStepped(v bool) { c.Set("stepped", v) }

func (c aylisDBChart) Tension() float64 { return asNumber(c.Get("tension")) }
func (c aylisDBChart) SetTension(v float64) { c.Set("tension", v) }

func (c aylisDBChart) Animate() bool { return asBool(c.Get("animate")) }
func (c aylisDBChart) SetAnimate(v bool) { c.Set("animate", v) }

func (c aylisDBChart) Zoom() bool { return asBool(c.Get("zoom")) }
func (c aylisDBChart) SetZoom(v bool) { c.Set("zoom", v) }

func (c aylisDBChart) AutoScale() bool { return asBool(c.Get("auto_scale")) }
func (c aylisDBChart) SetAutoScale(v bool) { c.Set("auto_scale", v) }

func (c aylisDBChart) YMin() float64 { return asNumber(c.Get("y_min")) }
func (c aylisDBChart) SetYMin(v float64) { c.Set("y_min", v) }

func (c aylisDBChart) YMax() float64 { return asNumber(c.Get("y_max")) }
func (c aylisDBChart) SetYMax(v float64) { c.Set("y_max", v) }

func (c aylisDBChart) LogScale() bool { return asBool(c.Get("log_scale")) }
func (c aylisDBChart) SetLogScale(v bool) { c.Set("log_scale", v) }

func (c aylisDBChart) Reverse() bool { return asBool(c.Get("reverse")) }
func (c aylisDBChart) SetReverse(v bool) { c.Set("reverse", v) }

func (c aylisDBChart) TimeAxis() bool { return asBool(c.Get("time_axis")) }
func (c aylisDBChart) SetTimeAxis(v bool) { c.Set("time_axis", v) }

func (c aylisDBChart) ClickedSeries() int64 { return int64(asNumber(c.Get("clicked_series"))) }
func (c aylisDBChart) SetClickedSeries(v int64) { c.Set("clicked_series", v) }

func (c aylisDBChart) ClickedIndex() int64 { return int64(asNumber(c.Get("clicked_index"))) }
func (c aylisDBChart) SetClickedIndex(v int64) { c.Set("clicked_index", v) }

func (c aylisDBChart) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDBChart) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDBChart) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDBChart) SetTop(v float64) { c.Set("top", v) }

func (c aylisDBChart) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDBChart) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDBChart) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDBChart) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDBChart) Align() string { return asText(c.Get("align")) }
func (c aylisDBChart) SetAlign(v string) { c.Set("align", v) }

func (c aylisDBChart) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDBChart) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDBChart) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDBChart) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDBChart) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDBChart) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDBChart) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDBChart) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDBChart) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDBChart) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDBChart) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDBChart) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDBChart) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDBChart) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDBChart) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDBChart) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDBChart) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDBChart) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDBChart) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDBChart) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDBChart) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisDBChart) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisDBChart) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDBChart) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDBChart) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDBChart) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDBChart) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisDBChart) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisDBChart) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDBChart) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDBChart) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDBChart) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDBChart) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDBChart) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDBChart) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDBChart) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDBChart) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDBChart) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDBChart) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDBChart) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDBChart) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDBChart) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDBChart) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDBChart) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDBChart) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDBChart) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDBChart) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDBChart) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDBChart) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisDBChart) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisDBChart) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDBChart) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDBChart) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDBChart) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDBChart) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisDBChart) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisDBChart) Style() string { return asText(c.Get("style")) }
func (c aylisDBChart) SetStyle(v string) { c.Set("style", v) }

func (c aylisDBChart) Theme() string { return asText(c.Get("theme")) }
func (c aylisDBChart) SetTheme(v string) { c.Set("theme", v) }

func (c aylisDBChart) Fill() string { return asText(c.Get("fill")) }
func (c aylisDBChart) SetFill(v string) { c.Set("fill", v) }

func (c aylisDBChart) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisDBChart) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisDBChart) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisDBChart) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisDBChart) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisDBChart) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisDBChart) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisDBChart) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisDBChart) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisDBChart) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisDBChart) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisDBChart) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisDBChart) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDBChart) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDBChart) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDBChart) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDBChart) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisDBChart) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisDBChart) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDBChart) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDBChart) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDBChart) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDBChart) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDBChart) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDBChart) Hint() string { return asText(c.Get("hint")) }
func (c aylisDBChart) SetHint(v string) { c.Set("hint", v) }

func (c aylisDBChart) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDBChart) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDBChart) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisDBChart) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisDataGrid é um DataGrid de um form.
type aylisDataGrid struct{ aylis.Component }

func (c aylisDataGrid) DataSource() string { return asText(c.Get("data_source")) }
func (c aylisDataGrid) SetDataSource(v string) { c.Set("data_source", v) }

func (c aylisDataGrid) Columns() string { return asText(c.Get("columns")) }
func (c aylisDataGrid) SetColumns(v string) { c.Set("columns", v) }

func (c aylisDataGrid) RowHeight() float64 { return asNumber(c.Get("row_height")) }
func (c aylisDataGrid) SetRowHeight(v float64) { c.Set("row_height", v) }

func (c aylisDataGrid) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisDataGrid) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisDataGrid) Sortable() bool { return asBool(c.Get("sortable")) }
func (c aylisDataGrid) SetSortable(v bool) { c.Set("sortable", v) }

func (c aylisDataGrid) ColumnMove() bool { return asBool(c.Get("column_move")) }
func (c aylisDataGrid) SetColumnMove(v bool) { c.Set("column_move", v) }

func (c aylisDataGrid) FixedColumns() int64 { return int64(asNumber(c.Get("fixed_columns"))) }
func (c aylisDataGrid) SetFixedColumns(v int64) { c.Set("fixed_columns", v) }

func (c aylisDataGrid) FilterRow() bool { return asBool(c.Get("filter_row")) }
func (c aylisDataGrid) SetFilterRow(v bool) { c.Set("filter_row", v) }

func (c aylisDataGrid) MultiSelect() bool { return asBool(c.Get("multi_select")) }
func (c aylisDataGrid) SetMultiSelect(v bool) { c.Set("multi_select", v) }

func (c aylisDataGrid) Footer() string { return asText(c.Get("footer")) }
func (c aylisDataGrid) SetFooter(v string) { c.Set("footer", v) }

func (c aylisDataGrid) IncrementalSearch() bool { return asBool(c.Get("incremental_search")) }
func (c aylisDataGrid) SetIncrementalSearch(v bool) { c.Set("incremental_search", v) }

func (c aylisDataGrid) CursorColor() string { return asText(c.Get("cursor_color")) }
func (c aylisDataGrid) SetCursorColor(v string) { c.Set("cursor_color", v) }

func (c aylisDataGrid) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDataGrid) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDataGrid) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDataGrid) SetTop(v float64) { c.Set("top", v) }

func (c aylisDataGrid) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDataGrid) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDataGrid) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDataGrid) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDataGrid) Align() string { return asText(c.Get("align")) }
func (c aylisDataGrid) SetAlign(v string) { c.Set("align", v) }

func (c aylisDataGrid) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDataGrid) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDataGrid) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDataGrid) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDataGrid) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDataGrid) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDataGrid) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDataGrid) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDataGrid) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDataGrid) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDataGrid) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDataGrid) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDataGrid) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDataGrid) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDataGrid) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDataGrid) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDataGrid) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDataGrid) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDataGrid) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDataGrid) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDataGrid) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisDataGrid) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisDataGrid) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDataGrid) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDataGrid) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDataGrid) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDataGrid) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisDataGrid) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisDataGrid) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDataGrid) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDataGrid) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDataGrid) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDataGrid) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDataGrid) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDataGrid) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDataGrid) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDataGrid) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDataGrid) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDataGrid) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDataGrid) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDataGrid) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDataGrid) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDataGrid) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDataGrid) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDataGrid) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDataGrid) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDataGrid) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDataGrid) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDataGrid) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisDataGrid) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisDataGrid) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDataGrid) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDataGrid) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDataGrid) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDataGrid) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisDataGrid) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisDataGrid) Style() string { return asText(c.Get("style")) }
func (c aylisDataGrid) SetStyle(v string) { c.Set("style", v) }

func (c aylisDataGrid) Theme() string { return asText(c.Get("theme")) }
func (c aylisDataGrid) SetTheme(v string) { c.Set("theme", v) }

func (c aylisDataGrid) Fill() string { return asText(c.Get("fill")) }
func (c aylisDataGrid) SetFill(v string) { c.Set("fill", v) }

func (c aylisDataGrid) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisDataGrid) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisDataGrid) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisDataGrid) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisDataGrid) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisDataGrid) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisDataGrid) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisDataGrid) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisDataGrid) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisDataGrid) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisDataGrid) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisDataGrid) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisDataGrid) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDataGrid) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDataGrid) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDataGrid) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDataGrid) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisDataGrid) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisDataGrid) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDataGrid) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDataGrid) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDataGrid) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDataGrid) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDataGrid) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDataGrid) Hint() string { return asText(c.Get("hint")) }
func (c aylisDataGrid) SetHint(v string) { c.Set("hint", v) }

func (c aylisDataGrid) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDataGrid) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDataGrid) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisDataGrid) SetPopupMenu(v string) { c.Set("popup_menu", v) }
func (c aylisDataGrid) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisDataSource é um DataSource de um form.
type aylisDataSource struct{ aylis.Component }

func (c aylisDataSource) DataSet() string { return asText(c.Get("data_set")) }
func (c aylisDataSource) SetDataSet(v string) { c.Set("data_set", v) }

func (c aylisDataSource) AutoEdit() bool { return asBool(c.Get("auto_edit")) }
func (c aylisDataSource) SetAutoEdit(v bool) { c.Set("auto_edit", v) }

func (c aylisDataSource) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDataSource) SetEnabled(v bool) { c.Set("enabled", v) }

// aylisField é um Field de um form.
type aylisField struct{ aylis.Component }

func (c aylisField) FieldName() string { return asText(c.Get("field_name")) }
func (c aylisField) SetFieldName(v string) { c.Set("field_name", v) }

func (c aylisField) Type() string { return asText(c.Get("type")) }
func (c aylisField) SetType(v string) { c.Set("type", v) }

func (c aylisField) Size() int64 { return int64(asNumber(c.Get("size"))) }
func (c aylisField) SetSize(v int64) { c.Set("size", v) }

func (c aylisField) Scale() int64 { return int64(asNumber(c.Get("scale"))) }
func (c aylisField) SetScale(v int64) { c.Set("scale", v) }

func (c aylisField) Rounding() string { return asText(c.Get("rounding")) }
func (c aylisField) SetRounding(v string) { c.Set("rounding", v) }

func (c aylisField) DisplayLabel() string { return asText(c.Get("display_label")) }
func (c aylisField) SetDisplayLabel(v string) { c.Set("display_label", v) }

func (c aylisField) DisplayWidth() float64 { return asNumber(c.Get("display_width")) }
func (c aylisField) SetDisplayWidth(v float64) { c.Set("display_width", v) }

func (c aylisField) DisplayFormat() string { return asText(c.Get("display_format")) }
func (c aylisField) SetDisplayFormat(v string) { c.Set("display_format", v) }

func (c aylisField) EditFormat() string { return asText(c.Get("edit_format")) }
func (c aylisField) SetEditFormat(v string) { c.Set("edit_format", v) }

func (c aylisField) Alignment() string { return asText(c.Get("alignment")) }
func (c aylisField) SetAlignment(v string) { c.Set("alignment", v) }

func (c aylisField) Required() bool { return asBool(c.Get("required")) }
func (c aylisField) SetRequired(v bool) { c.Set("required", v) }

func (c aylisField) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisField) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisField) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisField) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisField) Min() float64 { return asNumber(c.Get("min")) }
func (c aylisField) SetMin(v float64) { c.Set("min", v) }

func (c aylisField) Max() float64 { return asNumber(c.Get("max")) }
func (c aylisField) SetMax(v float64) { c.Set("max", v) }

func (c aylisField) DefaultValue() string { return asText(c.Get("default_value")) }
func (c aylisField) SetDefaultValue(v string) { c.Set("default_value", v) }

func (c aylisField) DisplayValues() string { return asText(c.Get("display_values")) }
func (c aylisField) SetDisplayValues(v string) { c.Set("display_values", v) }

func (c aylisField) CharCase() string { return asText(c.Get("char_case")) }
func (c aylisField) SetCharCase(v string) { c.Set("char_case", v) }

func (c aylisField) EditMask() string { return asText(c.Get("edit_mask")) }
func (c aylisField) SetEditMask(v string) { c.Set("edit_mask", v) }

func (c aylisField) Sensitive() bool { return asBool(c.Get("sensitive")) }
func (c aylisField) SetSensitive(v bool) { c.Set("sensitive", v) }

func (c aylisField) Kind() string { return asText(c.Get("kind")) }
func (c aylisField) SetKind(v string) { c.Set("kind", v) }

func (c aylisField) Expression() string { return asText(c.Get("expression")) }
func (c aylisField) SetExpression(v string) { c.Set("expression", v) }

func (c aylisField) LookupDataSet() string { return asText(c.Get("lookup_data_set")) }
func (c aylisField) SetLookupDataSet(v string) { c.Set("lookup_data_set", v) }

func (c aylisField) KeyFields() string { return asText(c.Get("key_fields")) }
func (c aylisField) SetKeyFields(v string) { c.Set("key_fields", v) }

func (c aylisField) LookupKeyFields() string { return asText(c.Get("lookup_key_fields")) }
func (c aylisField) SetLookupKeyFields(v string) { c.Set("lookup_key_fields", v) }

func (c aylisField) LookupResultField() string { return asText(c.Get("lookup_result_field")) }
func (c aylisField) SetLookupResultField(v string) { c.Set("lookup_result_field", v) }

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

// aylisGroupBox é um GroupBox de um form.
type aylisGroupBox struct{ aylis.Component }

func (c aylisGroupBox) Caption() string { return asText(c.Get("caption")) }
func (c aylisGroupBox) SetCaption(v string) { c.Set("caption", v) }

func (c aylisGroupBox) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisGroupBox) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisGroupBox) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisGroupBox) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisGroupBox) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisGroupBox) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisGroupBox) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisGroupBox) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisGroupBox) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisGroupBox) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisGroupBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisGroupBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisGroupBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisGroupBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisGroupBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisGroupBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisGroupBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisGroupBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisGroupBox) Align() string { return asText(c.Get("align")) }
func (c aylisGroupBox) SetAlign(v string) { c.Set("align", v) }

func (c aylisGroupBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisGroupBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisGroupBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisGroupBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisGroupBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisGroupBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisGroupBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisGroupBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisGroupBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisGroupBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisGroupBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisGroupBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisGroupBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisGroupBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisGroupBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisGroupBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisGroupBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisGroupBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisGroupBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisGroupBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisGroupBox) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisGroupBox) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisGroupBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisGroupBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisGroupBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisGroupBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisGroupBox) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisGroupBox) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisGroupBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisGroupBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisGroupBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisGroupBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisGroupBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisGroupBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisGroupBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisGroupBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisGroupBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisGroupBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisGroupBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisGroupBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisGroupBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisGroupBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisGroupBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisGroupBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisGroupBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisGroupBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisGroupBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisGroupBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisGroupBox) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisGroupBox) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisGroupBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisGroupBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisGroupBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisGroupBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisGroupBox) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisGroupBox) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisGroupBox) Style() string { return asText(c.Get("style")) }
func (c aylisGroupBox) SetStyle(v string) { c.Set("style", v) }

func (c aylisGroupBox) Theme() string { return asText(c.Get("theme")) }
func (c aylisGroupBox) SetTheme(v string) { c.Set("theme", v) }

func (c aylisGroupBox) Fill() string { return asText(c.Get("fill")) }
func (c aylisGroupBox) SetFill(v string) { c.Set("fill", v) }

func (c aylisGroupBox) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisGroupBox) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisGroupBox) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisGroupBox) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisGroupBox) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisGroupBox) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisGroupBox) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisGroupBox) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisGroupBox) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisGroupBox) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisGroupBox) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisGroupBox) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisGroupBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisGroupBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisGroupBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisGroupBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisGroupBox) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisGroupBox) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisGroupBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisGroupBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisGroupBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisGroupBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisGroupBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisGroupBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisGroupBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisGroupBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisGroupBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisGroupBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisGroupBox) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisGroupBox) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisImage é um Image de um form.
type aylisImage struct{ aylis.Component }

func (c aylisImage) Image() string { return asText(c.Get("image")) }
func (c aylisImage) SetImage(v string) { c.Set("image", v) }

func (c aylisImage) Fit() string { return asText(c.Get("fit")) }
func (c aylisImage) SetFit(v string) { c.Set("fit", v) }

func (c aylisImage) Radius() float64 { return asNumber(c.Get("radius")) }
func (c aylisImage) SetRadius(v float64) { c.Set("radius", v) }

func (c aylisImage) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisImage) SetLeft(v float64) { c.Set("left", v) }

func (c aylisImage) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisImage) SetTop(v float64) { c.Set("top", v) }

func (c aylisImage) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisImage) SetWidth(v float64) { c.Set("width", v) }

func (c aylisImage) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisImage) SetHeight(v float64) { c.Set("height", v) }

func (c aylisImage) Align() string { return asText(c.Get("align")) }
func (c aylisImage) SetAlign(v string) { c.Set("align", v) }

func (c aylisImage) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisImage) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisImage) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisImage) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisImage) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisImage) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisImage) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisImage) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisImage) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisImage) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisImage) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisImage) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisImage) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisImage) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisImage) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisImage) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisImage) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisImage) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisImage) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisImage) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisImage) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisImage) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisImage) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisImage) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisImage) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisImage) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisImage) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisImage) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisImage) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisImage) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisImage) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisImage) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisImage) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisImage) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisImage) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisImage) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisImage) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisImage) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisImage) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisImage) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisImage) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisImage) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisImage) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisImage) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisImage) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisImage) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisImage) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisImage) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisImage) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisImage) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisImage) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisImage) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisImage) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisImage) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisImage) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisImage) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisImage) Style() string { return asText(c.Get("style")) }
func (c aylisImage) SetStyle(v string) { c.Set("style", v) }

func (c aylisImage) Theme() string { return asText(c.Get("theme")) }
func (c aylisImage) SetTheme(v string) { c.Set("theme", v) }

func (c aylisImage) Fill() string { return asText(c.Get("fill")) }
func (c aylisImage) SetFill(v string) { c.Set("fill", v) }

func (c aylisImage) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisImage) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisImage) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisImage) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisImage) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisImage) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisImage) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisImage) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisImage) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisImage) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisImage) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisImage) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisImage) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisImage) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisImage) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisImage) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisImage) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisImage) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisImage) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisImage) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisImage) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisImage) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisImage) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisImage) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisImage) Hint() string { return asText(c.Get("hint")) }
func (c aylisImage) SetHint(v string) { c.Set("hint", v) }

func (c aylisImage) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisImage) SetTag(v int64) { c.Set("tag", v) }

func (c aylisImage) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisImage) SetPopupMenu(v string) { c.Set("popup_menu", v) }

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

// aylisSplitter é um Splitter de um form.
type aylisSplitter struct{ aylis.Component }

func (c aylisSplitter) MinSize() float64 { return asNumber(c.Get("min_size")) }
func (c aylisSplitter) SetMinSize(v float64) { c.Set("min_size", v) }

func (c aylisSplitter) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSplitter) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSplitter) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSplitter) SetTop(v float64) { c.Set("top", v) }

func (c aylisSplitter) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSplitter) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSplitter) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSplitter) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSplitter) Align() string { return asText(c.Get("align")) }
func (c aylisSplitter) SetAlign(v string) { c.Set("align", v) }

func (c aylisSplitter) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSplitter) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSplitter) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSplitter) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSplitter) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSplitter) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSplitter) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSplitter) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSplitter) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSplitter) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSplitter) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSplitter) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSplitter) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSplitter) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSplitter) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSplitter) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSplitter) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSplitter) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSplitter) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSplitter) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSplitter) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisSplitter) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisSplitter) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSplitter) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSplitter) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSplitter) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSplitter) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisSplitter) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisSplitter) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSplitter) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSplitter) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSplitter) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSplitter) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSplitter) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSplitter) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSplitter) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSplitter) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSplitter) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSplitter) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSplitter) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSplitter) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSplitter) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSplitter) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSplitter) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSplitter) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSplitter) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSplitter) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSplitter) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSplitter) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisSplitter) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisSplitter) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSplitter) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSplitter) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSplitter) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSplitter) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisSplitter) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisSplitter) Style() string { return asText(c.Get("style")) }
func (c aylisSplitter) SetStyle(v string) { c.Set("style", v) }

func (c aylisSplitter) Theme() string { return asText(c.Get("theme")) }
func (c aylisSplitter) SetTheme(v string) { c.Set("theme", v) }

func (c aylisSplitter) Fill() string { return asText(c.Get("fill")) }
func (c aylisSplitter) SetFill(v string) { c.Set("fill", v) }

func (c aylisSplitter) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisSplitter) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisSplitter) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisSplitter) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisSplitter) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisSplitter) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisSplitter) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisSplitter) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisSplitter) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisSplitter) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisSplitter) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisSplitter) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisSplitter) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSplitter) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSplitter) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSplitter) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSplitter) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisSplitter) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisSplitter) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSplitter) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSplitter) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSplitter) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSplitter) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSplitter) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSplitter) Hint() string { return asText(c.Get("hint")) }
func (c aylisSplitter) SetHint(v string) { c.Set("hint", v) }

func (c aylisSplitter) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSplitter) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSplitter) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisSplitter) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// aylisSvg é um Svg de um form.
type aylisSvg struct{ aylis.Component }

func (c aylisSvg) Source() string { return asText(c.Get("source")) }
func (c aylisSvg) SetSource(v string) { c.Set("source", v) }

func (c aylisSvg) Color() string { return asText(c.Get("color")) }
func (c aylisSvg) SetColor(v string) { c.Set("color", v) }

func (c aylisSvg) Fit() string { return asText(c.Get("fit")) }
func (c aylisSvg) SetFit(v string) { c.Set("fit", v) }

func (c aylisSvg) Highlight() string { return asText(c.Get("highlight")) }
func (c aylisSvg) SetHighlight(v string) { c.Set("highlight", v) }

func (c aylisSvg) HighlightColor() string { return asText(c.Get("highlight_color")) }
func (c aylisSvg) SetHighlightColor(v string) { c.Set("highlight_color", v) }

func (c aylisSvg) ZoomTo() string { return asText(c.Get("zoom_to")) }
func (c aylisSvg) SetZoomTo(v string) { c.Set("zoom_to", v) }

func (c aylisSvg) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSvg) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSvg) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSvg) SetTop(v float64) { c.Set("top", v) }

func (c aylisSvg) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSvg) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSvg) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSvg) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSvg) Align() string { return asText(c.Get("align")) }
func (c aylisSvg) SetAlign(v string) { c.Set("align", v) }

func (c aylisSvg) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSvg) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSvg) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSvg) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSvg) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSvg) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSvg) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSvg) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSvg) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSvg) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSvg) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSvg) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSvg) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSvg) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSvg) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSvg) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSvg) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSvg) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSvg) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSvg) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSvg) DragImage() string { return asText(c.Get("drag_image")) }
func (c aylisSvg) SetDragImage(v string) { c.Set("drag_image", v) }

func (c aylisSvg) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSvg) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSvg) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSvg) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSvg) Cursor() string { return asText(c.Get("cursor")) }
func (c aylisSvg) SetCursor(v string) { c.Set("cursor", v) }

func (c aylisSvg) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSvg) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSvg) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSvg) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSvg) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSvg) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSvg) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSvg) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSvg) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSvg) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSvg) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSvg) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSvg) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSvg) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSvg) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSvg) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSvg) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSvg) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSvg) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSvg) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSvg) SelfAlign() string { return asText(c.Get("self_align")) }
func (c aylisSvg) SetSelfAlign(v string) { c.Set("self_align", v) }

func (c aylisSvg) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSvg) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSvg) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSvg) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSvg) Shadow() string { return asText(c.Get("shadow")) }
func (c aylisSvg) SetShadow(v string) { c.Set("shadow", v) }

func (c aylisSvg) Style() string { return asText(c.Get("style")) }
func (c aylisSvg) SetStyle(v string) { c.Set("style", v) }

func (c aylisSvg) Theme() string { return asText(c.Get("theme")) }
func (c aylisSvg) SetTheme(v string) { c.Set("theme", v) }

func (c aylisSvg) Fill() string { return asText(c.Get("fill")) }
func (c aylisSvg) SetFill(v string) { c.Set("fill", v) }

func (c aylisSvg) BorderColor() string { return asText(c.Get("border_color")) }
func (c aylisSvg) SetBorderColor(v string) { c.Set("border_color", v) }

func (c aylisSvg) TextColor() string { return asText(c.Get("text_color")) }
func (c aylisSvg) SetTextColor(v string) { c.Set("text_color", v) }

func (c aylisSvg) AccentColor() string { return asText(c.Get("accent_color")) }
func (c aylisSvg) SetAccentColor(v string) { c.Set("accent_color", v) }

func (c aylisSvg) SuccessColor() string { return asText(c.Get("success_color")) }
func (c aylisSvg) SetSuccessColor(v string) { c.Set("success_color", v) }

func (c aylisSvg) WarningColor() string { return asText(c.Get("warning_color")) }
func (c aylisSvg) SetWarningColor(v string) { c.Set("warning_color", v) }

func (c aylisSvg) DangerColor() string { return asText(c.Get("danger_color")) }
func (c aylisSvg) SetDangerColor(v string) { c.Set("danger_color", v) }

func (c aylisSvg) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSvg) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSvg) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSvg) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSvg) FontWeight() string { return asText(c.Get("font_weight")) }
func (c aylisSvg) SetFontWeight(v string) { c.Set("font_weight", v) }

func (c aylisSvg) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSvg) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSvg) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSvg) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSvg) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSvg) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSvg) Hint() string { return asText(c.Get("hint")) }
func (c aylisSvg) SetHint(v string) { c.Set("hint", v) }

func (c aylisSvg) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSvg) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSvg) PopupMenu() string { return asText(c.Get("popup_menu")) }
func (c aylisSvg) SetPopupMenu(v string) { c.Set("popup_menu", v) }

// frmIBGEComponents: o form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmIBGEComponents struct {
	aylisForm
	cdsEstados aylisClientDataSet
	cdsEstadosId aylisField
	cdsEstadosSigla aylisField
	cdsEstadosNome aylisField
	cdsEstadosRegiao aylisField
	cdsEstadosPopulacao aylisField
	cdsEstadosArea aylisField
	cdsEstadosDensidade aylisField
	cdsEstadosPib aylisField
	cdsEstadosPibPerCapita aylisField
	cdsEstadosEstimativa aylisField
	cdsEstadosEstimativaAno aylisField
	cdsEstadosMunicipios aylisField
	cdsEstadosBandeira aylisField
	dsEstados aylisDataSource
	cdsMunicipios aylisClientDataSet
	cdsMunicipiosId aylisField
	cdsMunicipiosUfId aylisField
	cdsMunicipiosUf aylisField
	cdsMunicipiosNome aylisField
	cdsMunicipiosMesorregiao aylisField
	cdsMunicipiosMicrorregiao aylisField
	cdsMunicipiosRegiaoIntermediaria aylisField
	cdsMunicipiosRegiaoImediata aylisField
	cdsMunicipiosPopulacao aylisField
	cdsMunicipiosArea aylisField
	cdsMunicipiosDensidade aylisField
	cdsMunicipiosPib aylisField
	cdsMunicipiosPibParticipacao aylisField
	cdsMunicipiosPibPerCapita aylisField
	cdsMunicipiosEstimativa aylisField
	cdsMunicipiosEstimativaAno aylisField
	cdsMunicipiosDistritos aylisField
	cdsMunicipiosBandeira aylisField
	dsMunicipios aylisDataSource
	cdsDistritos aylisClientDataSet
	cdsDistritosId aylisField
	cdsDistritosMunicipioId aylisField
	cdsDistritosNome aylisField
	dsDistritos aylisDataSource
	cdsPopulacao aylisClientDataSet
	cdsPopulacaoAno aylisField
	cdsPopulacaoHabitantes aylisField
	dsPopulacao aylisDataSource
	cdsMaiores aylisClientDataSet
	cdsMaioresNome aylisField
	cdsMaioresHabitantes aylisField
	dsMaiores aylisDataSource
	lblFonte aylisLabel
	lytListas aylisLayout
	gbEstados aylisGroupBox
	grdEstados aylisDataGrid
	gbDistritos aylisGroupBox
	grdDistritos aylisDataGrid
	gbMunicipios aylisGroupBox
	grdMunicipios aylisDataGrid
	splListas aylisSplitter
	lytPainel aylisLayout
	pnlEstado aylisPanel
	svgBrasil aylisSvg
	lytBandeiraEstado aylisLayout
	lblEstado aylisLabel
	lblRegiao aylisLabel
	imgBandeiraEstado aylisImage
	lytNumerosEstado aylisLayout
	dbtEstadoPopulacao aylisLabel
	lblEstadoEstimativa aylisLabel
	dbtEstadoEstimativa aylisLabel
	dbtEstadoArea aylisLabel
	dbtEstadoDensidade aylisLabel
	dbtEstadoPib aylisLabel
	dbtEstadoPibPerCapita aylisLabel
	dbtEstadoMunicipios aylisLabel
	pnlMunicipio aylisPanel
	svgMunicipio aylisSvg
	svgEstado aylisSvg
	lytBandeiraMunicipio aylisLayout
	lblMunicipio aylisLabel
	lblCodigo aylisLabel
	imgBandeiraMunicipio aylisImage
	lblSemBandeira aylisLabel
	lytNumerosMunicipio aylisLayout
	dbtMesorregiao aylisLabel
	dbtMicrorregiao aylisLabel
	dbtRegiaoImediata aylisLabel
	dbtPopulacao aylisLabel
	lblEstimativa aylisLabel
	dbtEstimativa aylisLabel
	dbtArea aylisLabel
	dbtDensidade aylisLabel
	dbtPib aylisLabel
	dbtPibPerCapita aylisLabel
	dbtPibParticipacao aylisLabel
	lytGraficos aylisLayout
	dbcPopulacao aylisDBChart
	dbcMaiores aylisDBChart
	dbcEstados aylisDBChart
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmIBGEComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "cdsEstados", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsEstados = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosId", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosSigla", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosSigla = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosRegiao", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosRegiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPopulacao", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPopulacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosArea", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosArea = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosDensidade", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosDensidade = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPib", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPib = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPibPerCapita", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPibPerCapita = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosEstimativa", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosEstimativa = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosEstimativaAno", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosEstimativaAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosMunicipios", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosMunicipios = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosBandeira", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosBandeira = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsEstados", "DataSource"); err != nil {
		return err
	} else {
		f.dsEstados = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipios", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsMunicipios = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosId", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosUfId", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosUfId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosUf", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosUf = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosMesorregiao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosMesorregiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosMicrorregiao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosMicrorregiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosRegiaoIntermediaria = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosRegiaoImediata", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosRegiaoImediata = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPopulacao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPopulacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosArea", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosArea = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosDensidade", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosDensidade = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPib", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPib = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPibParticipacao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPibParticipacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPibPerCapita", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPibPerCapita = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosEstimativa", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosEstimativa = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosEstimativaAno", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosEstimativaAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosDistritos", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosDistritos = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosBandeira", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosBandeira = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsMunicipios", "DataSource"); err != nil {
		return err
	} else {
		f.dsMunicipios = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsDistritos", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsDistritos = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosId", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosMunicipioId", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosMunicipioId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsDistritos", "DataSource"); err != nil {
		return err
	} else {
		f.dsDistritos = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacao", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsPopulacao = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacaoAno", "Field"); err != nil {
		return err
	} else {
		f.cdsPopulacaoAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacaoHabitantes", "Field"); err != nil {
		return err
	} else {
		f.cdsPopulacaoHabitantes = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsPopulacao", "DataSource"); err != nil {
		return err
	} else {
		f.dsPopulacao = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsMaiores", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsMaiores = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsMaioresNome", "Field"); err != nil {
		return err
	} else {
		f.cdsMaioresNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMaioresHabitantes", "Field"); err != nil {
		return err
	} else {
		f.cdsMaioresHabitantes = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsMaiores", "DataSource"); err != nil {
		return err
	} else {
		f.dsMaiores = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "lblFonte", "Label"); err != nil {
		return err
	} else {
		f.lblFonte = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytListas", "Layout"); err != nil {
		return err
	} else {
		f.lytListas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "gbEstados", "GroupBox"); err != nil {
		return err
	} else {
		f.gbEstados = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdEstados", "DataGrid"); err != nil {
		return err
	} else {
		f.grdEstados = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "gbDistritos", "GroupBox"); err != nil {
		return err
	} else {
		f.gbDistritos = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdDistritos", "DataGrid"); err != nil {
		return err
	} else {
		f.grdDistritos = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "gbMunicipios", "GroupBox"); err != nil {
		return err
	} else {
		f.gbMunicipios = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdMunicipios", "DataGrid"); err != nil {
		return err
	} else {
		f.grdMunicipios = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "splListas", "Splitter"); err != nil {
		return err
	} else {
		f.splListas = aylisSplitter{c}
	}
	if c, err := f.Resolve(list, "lytPainel", "Layout"); err != nil {
		return err
	} else {
		f.lytPainel = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "pnlEstado", "Panel"); err != nil {
		return err
	} else {
		f.pnlEstado = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "svgBrasil", "Svg"); err != nil {
		return err
	} else {
		f.svgBrasil = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "lytBandeiraEstado", "Layout"); err != nil {
		return err
	} else {
		f.lytBandeiraEstado = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblEstado", "Label"); err != nil {
		return err
	} else {
		f.lblEstado = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblRegiao", "Label"); err != nil {
		return err
	} else {
		f.lblRegiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "imgBandeiraEstado", "Image"); err != nil {
		return err
	} else {
		f.imgBandeiraEstado = aylisImage{c}
	}
	if c, err := f.Resolve(list, "lytNumerosEstado", "Layout"); err != nil {
		return err
	} else {
		f.lytNumerosEstado = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPopulacao", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPopulacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblEstadoEstimativa", "Label"); err != nil {
		return err
	} else {
		f.lblEstadoEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoEstimativa", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoArea", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoArea = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoDensidade", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoDensidade = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPib", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPib = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPibPerCapita", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPibPerCapita = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoMunicipios", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoMunicipios = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "pnlMunicipio", "Panel"); err != nil {
		return err
	} else {
		f.pnlMunicipio = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "svgMunicipio", "Svg"); err != nil {
		return err
	} else {
		f.svgMunicipio = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "svgEstado", "Svg"); err != nil {
		return err
	} else {
		f.svgEstado = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "lytBandeiraMunicipio", "Layout"); err != nil {
		return err
	} else {
		f.lytBandeiraMunicipio = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblMunicipio", "Label"); err != nil {
		return err
	} else {
		f.lblMunicipio = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblCodigo", "Label"); err != nil {
		return err
	} else {
		f.lblCodigo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "imgBandeiraMunicipio", "Image"); err != nil {
		return err
	} else {
		f.imgBandeiraMunicipio = aylisImage{c}
	}
	if c, err := f.Resolve(list, "lblSemBandeira", "Label"); err != nil {
		return err
	} else {
		f.lblSemBandeira = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytNumerosMunicipio", "Layout"); err != nil {
		return err
	} else {
		f.lytNumerosMunicipio = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbtMesorregiao", "Label"); err != nil {
		return err
	} else {
		f.dbtMesorregiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtMicrorregiao", "Label"); err != nil {
		return err
	} else {
		f.dbtMicrorregiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtRegiaoImediata", "Label"); err != nil {
		return err
	} else {
		f.dbtRegiaoImediata = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPopulacao", "Label"); err != nil {
		return err
	} else {
		f.dbtPopulacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblEstimativa", "Label"); err != nil {
		return err
	} else {
		f.lblEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstimativa", "Label"); err != nil {
		return err
	} else {
		f.dbtEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtArea", "Label"); err != nil {
		return err
	} else {
		f.dbtArea = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtDensidade", "Label"); err != nil {
		return err
	} else {
		f.dbtDensidade = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPib", "Label"); err != nil {
		return err
	} else {
		f.dbtPib = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPibPerCapita", "Label"); err != nil {
		return err
	} else {
		f.dbtPibPerCapita = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPibParticipacao", "Label"); err != nil {
		return err
	} else {
		f.dbtPibParticipacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytGraficos", "Layout"); err != nil {
		return err
	} else {
		f.lytGraficos = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbcPopulacao", "DBChart"); err != nil {
		return err
	} else {
		f.dbcPopulacao = aylisDBChart{c}
	}
	if c, err := f.Resolve(list, "dbcMaiores", "DBChart"); err != nil {
		return err
	} else {
		f.dbcMaiores = aylisDBChart{c}
	}
	if c, err := f.Resolve(list, "dbcEstados", "DBChart"); err != nil {
		return err
	} else {
		f.dbcEstados = aylisDBChart{c}
	}
	return nil
}

// frmIBGE é a janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers).
var frmIBGE *frmIBGEComponents

func init() {
	aylis.RegisterFormClass("frmIBGE", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmIBGEComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmIBGE = form.(*frmIBGEComponents) })
}

// FindClientDataSet é o ClientDataSet deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindClientDataSet(name string) aylisClientDataSet {
	switch name {
	case "cdsEstados":
		return f.cdsEstados
	case "cdsMunicipios":
		return f.cdsMunicipios
	case "cdsDistritos":
		return f.cdsDistritos
	case "cdsPopulacao":
		return f.cdsPopulacao
	case "cdsMaiores":
		return f.cdsMaiores
	}
	return aylisClientDataSet{}
}

// SenderAsClientDataSet é o ClientDataSet que disparou o evento.
func (f *frmIBGEComponents) SenderAsClientDataSet(ev aylis.Event) aylisClientDataSet { return f.FindClientDataSet(ev.Sender.Name) }

// FindDBChart é o DBChart deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindDBChart(name string) aylisDBChart {
	switch name {
	case "dbcPopulacao":
		return f.dbcPopulacao
	case "dbcMaiores":
		return f.dbcMaiores
	case "dbcEstados":
		return f.dbcEstados
	}
	return aylisDBChart{}
}

// SenderAsDBChart é o DBChart que disparou o evento.
func (f *frmIBGEComponents) SenderAsDBChart(ev aylis.Event) aylisDBChart { return f.FindDBChart(ev.Sender.Name) }

// FindDataGrid é o DataGrid deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindDataGrid(name string) aylisDataGrid {
	switch name {
	case "grdEstados":
		return f.grdEstados
	case "grdDistritos":
		return f.grdDistritos
	case "grdMunicipios":
		return f.grdMunicipios
	}
	return aylisDataGrid{}
}

// SenderAsDataGrid é o DataGrid que disparou o evento.
func (f *frmIBGEComponents) SenderAsDataGrid(ev aylis.Event) aylisDataGrid { return f.FindDataGrid(ev.Sender.Name) }

// FindDataSource é o DataSource deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindDataSource(name string) aylisDataSource {
	switch name {
	case "dsEstados":
		return f.dsEstados
	case "dsMunicipios":
		return f.dsMunicipios
	case "dsDistritos":
		return f.dsDistritos
	case "dsPopulacao":
		return f.dsPopulacao
	case "dsMaiores":
		return f.dsMaiores
	}
	return aylisDataSource{}
}

// SenderAsDataSource é o DataSource que disparou o evento.
func (f *frmIBGEComponents) SenderAsDataSource(ev aylis.Event) aylisDataSource { return f.FindDataSource(ev.Sender.Name) }

// FindField é o Field deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindField(name string) aylisField {
	switch name {
	case "cdsEstadosId":
		return f.cdsEstadosId
	case "cdsEstadosSigla":
		return f.cdsEstadosSigla
	case "cdsEstadosNome":
		return f.cdsEstadosNome
	case "cdsEstadosRegiao":
		return f.cdsEstadosRegiao
	case "cdsEstadosPopulacao":
		return f.cdsEstadosPopulacao
	case "cdsEstadosArea":
		return f.cdsEstadosArea
	case "cdsEstadosDensidade":
		return f.cdsEstadosDensidade
	case "cdsEstadosPib":
		return f.cdsEstadosPib
	case "cdsEstadosPibPerCapita":
		return f.cdsEstadosPibPerCapita
	case "cdsEstadosEstimativa":
		return f.cdsEstadosEstimativa
	case "cdsEstadosEstimativaAno":
		return f.cdsEstadosEstimativaAno
	case "cdsEstadosMunicipios":
		return f.cdsEstadosMunicipios
	case "cdsEstadosBandeira":
		return f.cdsEstadosBandeira
	case "cdsMunicipiosId":
		return f.cdsMunicipiosId
	case "cdsMunicipiosUfId":
		return f.cdsMunicipiosUfId
	case "cdsMunicipiosUf":
		return f.cdsMunicipiosUf
	case "cdsMunicipiosNome":
		return f.cdsMunicipiosNome
	case "cdsMunicipiosMesorregiao":
		return f.cdsMunicipiosMesorregiao
	case "cdsMunicipiosMicrorregiao":
		return f.cdsMunicipiosMicrorregiao
	case "cdsMunicipiosRegiaoIntermediaria":
		return f.cdsMunicipiosRegiaoIntermediaria
	case "cdsMunicipiosRegiaoImediata":
		return f.cdsMunicipiosRegiaoImediata
	case "cdsMunicipiosPopulacao":
		return f.cdsMunicipiosPopulacao
	case "cdsMunicipiosArea":
		return f.cdsMunicipiosArea
	case "cdsMunicipiosDensidade":
		return f.cdsMunicipiosDensidade
	case "cdsMunicipiosPib":
		return f.cdsMunicipiosPib
	case "cdsMunicipiosPibParticipacao":
		return f.cdsMunicipiosPibParticipacao
	case "cdsMunicipiosPibPerCapita":
		return f.cdsMunicipiosPibPerCapita
	case "cdsMunicipiosEstimativa":
		return f.cdsMunicipiosEstimativa
	case "cdsMunicipiosEstimativaAno":
		return f.cdsMunicipiosEstimativaAno
	case "cdsMunicipiosDistritos":
		return f.cdsMunicipiosDistritos
	case "cdsMunicipiosBandeira":
		return f.cdsMunicipiosBandeira
	case "cdsDistritosId":
		return f.cdsDistritosId
	case "cdsDistritosMunicipioId":
		return f.cdsDistritosMunicipioId
	case "cdsDistritosNome":
		return f.cdsDistritosNome
	case "cdsPopulacaoAno":
		return f.cdsPopulacaoAno
	case "cdsPopulacaoHabitantes":
		return f.cdsPopulacaoHabitantes
	case "cdsMaioresNome":
		return f.cdsMaioresNome
	case "cdsMaioresHabitantes":
		return f.cdsMaioresHabitantes
	}
	return aylisField{}
}

// SenderAsField é o Field que disparou o evento.
func (f *frmIBGEComponents) SenderAsField(ev aylis.Event) aylisField { return f.FindField(ev.Sender.Name) }

// FindGroupBox é o GroupBox deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindGroupBox(name string) aylisGroupBox {
	switch name {
	case "gbEstados":
		return f.gbEstados
	case "gbDistritos":
		return f.gbDistritos
	case "gbMunicipios":
		return f.gbMunicipios
	}
	return aylisGroupBox{}
}

// SenderAsGroupBox é o GroupBox que disparou o evento.
func (f *frmIBGEComponents) SenderAsGroupBox(ev aylis.Event) aylisGroupBox { return f.FindGroupBox(ev.Sender.Name) }

// FindImage é o Image deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindImage(name string) aylisImage {
	switch name {
	case "imgBandeiraEstado":
		return f.imgBandeiraEstado
	case "imgBandeiraMunicipio":
		return f.imgBandeiraMunicipio
	}
	return aylisImage{}
}

// SenderAsImage é o Image que disparou o evento.
func (f *frmIBGEComponents) SenderAsImage(ev aylis.Event) aylisImage { return f.FindImage(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblFonte":
		return f.lblFonte
	case "lblEstado":
		return f.lblEstado
	case "lblRegiao":
		return f.lblRegiao
	case "dbtEstadoPopulacao":
		return f.dbtEstadoPopulacao
	case "lblEstadoEstimativa":
		return f.lblEstadoEstimativa
	case "dbtEstadoEstimativa":
		return f.dbtEstadoEstimativa
	case "dbtEstadoArea":
		return f.dbtEstadoArea
	case "dbtEstadoDensidade":
		return f.dbtEstadoDensidade
	case "dbtEstadoPib":
		return f.dbtEstadoPib
	case "dbtEstadoPibPerCapita":
		return f.dbtEstadoPibPerCapita
	case "dbtEstadoMunicipios":
		return f.dbtEstadoMunicipios
	case "lblMunicipio":
		return f.lblMunicipio
	case "lblCodigo":
		return f.lblCodigo
	case "lblSemBandeira":
		return f.lblSemBandeira
	case "dbtMesorregiao":
		return f.dbtMesorregiao
	case "dbtMicrorregiao":
		return f.dbtMicrorregiao
	case "dbtRegiaoImediata":
		return f.dbtRegiaoImediata
	case "dbtPopulacao":
		return f.dbtPopulacao
	case "lblEstimativa":
		return f.lblEstimativa
	case "dbtEstimativa":
		return f.dbtEstimativa
	case "dbtArea":
		return f.dbtArea
	case "dbtDensidade":
		return f.dbtDensidade
	case "dbtPib":
		return f.dbtPib
	case "dbtPibPerCapita":
		return f.dbtPibPerCapita
	case "dbtPibParticipacao":
		return f.dbtPibParticipacao
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmIBGEComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytListas":
		return f.lytListas
	case "lytPainel":
		return f.lytPainel
	case "lytBandeiraEstado":
		return f.lytBandeiraEstado
	case "lytNumerosEstado":
		return f.lytNumerosEstado
	case "lytBandeiraMunicipio":
		return f.lytBandeiraMunicipio
	case "lytNumerosMunicipio":
		return f.lytNumerosMunicipio
	case "lytGraficos":
		return f.lytGraficos
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmIBGEComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlEstado":
		return f.pnlEstado
	case "pnlMunicipio":
		return f.pnlMunicipio
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmIBGEComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSplitter é o Splitter deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindSplitter(name string) aylisSplitter {
	switch name {
	case "splListas":
		return f.splListas
	}
	return aylisSplitter{}
}

// SenderAsSplitter é o Splitter que disparou o evento.
func (f *frmIBGEComponents) SenderAsSplitter(ev aylis.Event) aylisSplitter { return f.FindSplitter(ev.Sender.Name) }

// FindSvg é o Svg deste form com o nome (como o FindComponent).
func (f *frmIBGEComponents) FindSvg(name string) aylisSvg {
	switch name {
	case "svgBrasil":
		return f.svgBrasil
	case "svgMunicipio":
		return f.svgMunicipio
	case "svgEstado":
		return f.svgEstado
	}
	return aylisSvg{}
}

// SenderAsSvg é o Svg que disparou o evento.
func (f *frmIBGEComponents) SenderAsSvg(ev aylis.Event) aylisSvg { return f.FindSvg(ev.Sender.Name) }
