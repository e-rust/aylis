// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms;

import dev.aylis.Aylis;

/** Os forms ({@code import static forms.Components.frmX;}: a janela do form cujo evento está
 *  rodando, a primeira fora dos handlers) e as classes dos componentes. */
public final class Components {
    private Components() {}

    static boolean asBool(Object v) {
        return v instanceof Boolean b && b;
    }

    static double asNumber(Object v) {
        return v instanceof Number n ? n.doubleValue() : 0;
    }

    static String asText(Object v) {
        return v == null ? "" : v.toString();
    }

    public static FrmIBGEComponents frmIBGE;

    /** Registra as classes dos forms (o registro dos forms chama). */
    public static void registerClasses() {
        Aylis.registerFormClass("frmIBGE", FrmIBGEComponents::new, form -> frmIBGE = (FrmIBGEComponents) form);
    }

    /** Um ClientDataSet de um form. */
    public static class ClientDataSet extends Aylis.AylisComponent {
        public ClientDataSet(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getKeyFields() {
            return asText(getProp("key_fields"));
        }

        public void setKeyFields(String value) {
            setProp("key_fields", value);
        }

        public String getApplyMode() {
            return asText(getProp("apply_mode"));
        }

        public void setApplyMode(String value) {
            setProp("apply_mode", value);
        }

        public boolean isReadOnly() {
            return asBool(getProp("read_only"));
        }

        public void setReadOnly(boolean value) {
            setProp("read_only", value);
        }

        public boolean isActive() {
            return asBool(getProp("active"));
        }

        public void setActive(boolean value) {
            setProp("active", value);
        }

        public String getFilter() {
            return asText(getProp("filter"));
        }

        public void setFilter(String value) {
            setProp("filter", value);
        }

        public boolean isFiltered() {
            return asBool(getProp("filtered"));
        }

        public void setFiltered(boolean value) {
            setProp("filtered", value);
        }

        public String getIndexFieldNames() {
            return asText(getProp("index_field_names"));
        }

        public void setIndexFieldNames(String value) {
            setProp("index_field_names", value);
        }

        public String getIndexes() {
            return asText(getProp("indexes"));
        }

        public void setIndexes(String value) {
            setProp("indexes", value);
        }

        public String getIndexName() {
            return asText(getProp("index_name"));
        }

        public void setIndexName(String value) {
            setProp("index_name", value);
        }

        public String getParams() {
            return asText(getProp("params"));
        }

        public void setParams(String value) {
            setProp("params", value);
        }

        public String getStatusFilter() {
            return asText(getProp("status_filter"));
        }

        public void setStatusFilter(String value) {
            setProp("status_filter", value);
        }

        public String getMasterSource() {
            return asText(getProp("master_source"));
        }

        public void setMasterSource(String value) {
            setProp("master_source", value);
        }

        public String getMasterFields() {
            return asText(getProp("master_fields"));
        }

        public void setMasterFields(String value) {
            setProp("master_fields", value);
        }

        public boolean isApplySeparately() {
            return asBool(getProp("apply_separately"));
        }

        public void setApplySeparately(boolean value) {
            setProp("apply_separately", value);
        }

        public long getRecordTotal() {
            return (long) asNumber(getProp("record_total"));
        }

        public void setRecordTotal(long value) {
            setProp("record_total", value);
        }

        public long getMaxRecords() {
            return (long) asNumber(getProp("max_records"));
        }

        public void setMaxRecords(long value) {
            setProp("max_records", value);
        }

        public String getPartialNotice() {
            return asText(getProp("partial_notice"));
        }
    }

    /** Um DBChart de um form. */
    public static class DBChart extends Aylis.AylisComponent {
        public DBChart(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getDataSource() {
            return asText(getProp("data_source"));
        }

        public void setDataSource(String value) {
            setProp("data_source", value);
        }

        public String getLabelField() {
            return asText(getProp("label_field"));
        }

        public void setLabelField(String value) {
            setProp("label_field", value);
        }

        public String getValueFields() {
            return asText(getProp("value_fields"));
        }

        public void setValueFields(String value) {
            setProp("value_fields", value);
        }

        public String getKind() {
            return asText(getProp("kind"));
        }

        public void setKind(String value) {
            setProp("kind", value);
        }

        public String getTitle() {
            return asText(getProp("title"));
        }

        public void setTitle(String value) {
            setProp("title", value);
        }

        public String getLabels() {
            return asText(getProp("labels"));
        }

        public void setLabels(String value) {
            setProp("labels", value);
        }

        public String getSeries() {
            return asText(getProp("series"));
        }

        public void setSeries(String value) {
            setProp("series", value);
        }

        public String getJson() {
            return asText(getProp("json"));
        }

        public void setJson(String value) {
            setProp("json", value);
        }

        public boolean isLegend() {
            return asBool(getProp("legend"));
        }

        public void setLegend(boolean value) {
            setProp("legend", value);
        }

        public boolean isStepped() {
            return asBool(getProp("stepped"));
        }

        public void setStepped(boolean value) {
            setProp("stepped", value);
        }

        public double getTension() {
            return asNumber(getProp("tension"));
        }

        public void setTension(double value) {
            setProp("tension", value);
        }

        public boolean isAnimate() {
            return asBool(getProp("animate"));
        }

        public void setAnimate(boolean value) {
            setProp("animate", value);
        }

        public boolean isZoom() {
            return asBool(getProp("zoom"));
        }

        public void setZoom(boolean value) {
            setProp("zoom", value);
        }

        public boolean isAutoScale() {
            return asBool(getProp("auto_scale"));
        }

        public void setAutoScale(boolean value) {
            setProp("auto_scale", value);
        }

        public double getYMin() {
            return asNumber(getProp("y_min"));
        }

        public void setYMin(double value) {
            setProp("y_min", value);
        }

        public double getYMax() {
            return asNumber(getProp("y_max"));
        }

        public void setYMax(double value) {
            setProp("y_max", value);
        }

        public boolean isLogScale() {
            return asBool(getProp("log_scale"));
        }

        public void setLogScale(boolean value) {
            setProp("log_scale", value);
        }

        public boolean isReverse() {
            return asBool(getProp("reverse"));
        }

        public void setReverse(boolean value) {
            setProp("reverse", value);
        }

        public boolean isTimeAxis() {
            return asBool(getProp("time_axis"));
        }

        public void setTimeAxis(boolean value) {
            setProp("time_axis", value);
        }

        public long getClickedSeries() {
            return (long) asNumber(getProp("clicked_series"));
        }

        public void setClickedSeries(long value) {
            setProp("clicked_series", value);
        }

        public long getClickedIndex() {
            return (long) asNumber(getProp("clicked_index"));
        }

        public void setClickedIndex(long value) {
            setProp("clicked_index", value);
        }

        public String getValueFormat() {
            return asText(getProp("value_format"));
        }

        public void setValueFormat(String value) {
            setProp("value_format", value);
        }

        public double getLabelAngle() {
            return asNumber(getProp("label_angle"));
        }

        public void setLabelAngle(double value) {
            setProp("label_angle", value);
        }

        public String getTitleAlign() {
            return asText(getProp("title_align"));
        }

        public void setTitleAlign(String value) {
            setProp("title_align", value);
        }

        public boolean isTitleBold() {
            return asBool(getProp("title_bold"));
        }

        public void setTitleBold(boolean value) {
            setProp("title_bold", value);
        }

        public double getAreaOpacity() {
            return asNumber(getProp("area_opacity"));
        }

        public void setAreaOpacity(double value) {
            setProp("area_opacity", value);
        }

        public String getLegendPosition() {
            return asText(getProp("legend_position"));
        }

        public void setLegendPosition(String value) {
            setProp("legend_position", value);
        }

        public boolean isStacked() {
            return asBool(getProp("stacked"));
        }

        public void setStacked(boolean value) {
            setProp("stacked", value);
        }

        public String getColors() {
            return asText(getProp("colors"));
        }

        public void setColors(String value) {
            setProp("colors", value);
        }

        public String getCenterText() {
            return asText(getProp("center_text"));
        }

        public void setCenterText(String value) {
            setProp("center_text", value);
        }

        public String getSliceLabels() {
            return asText(getProp("slice_labels"));
        }

        public void setSliceLabels(String value) {
            setProp("slice_labels", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um DataGrid de um form. */
    public static class DataGrid extends Aylis.AylisComponent {
        public DataGrid(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getDataSource() {
            return asText(getProp("data_source"));
        }

        public void setDataSource(String value) {
            setProp("data_source", value);
        }

        public String getColumns() {
            return asText(getProp("columns"));
        }

        public void setColumns(String value) {
            setProp("columns", value);
        }

        public double getRowHeight() {
            return asNumber(getProp("row_height"));
        }

        public void setRowHeight(double value) {
            setProp("row_height", value);
        }

        public boolean isReadOnly() {
            return asBool(getProp("read_only"));
        }

        public void setReadOnly(boolean value) {
            setProp("read_only", value);
        }

        public boolean isSortable() {
            return asBool(getProp("sortable"));
        }

        public void setSortable(boolean value) {
            setProp("sortable", value);
        }

        public boolean isColumnMove() {
            return asBool(getProp("column_move"));
        }

        public void setColumnMove(boolean value) {
            setProp("column_move", value);
        }

        public long getFixedColumns() {
            return (long) asNumber(getProp("fixed_columns"));
        }

        public void setFixedColumns(long value) {
            setProp("fixed_columns", value);
        }

        public boolean isFilterRow() {
            return asBool(getProp("filter_row"));
        }

        public void setFilterRow(boolean value) {
            setProp("filter_row", value);
        }

        public boolean isMultiSelect() {
            return asBool(getProp("multi_select"));
        }

        public void setMultiSelect(boolean value) {
            setProp("multi_select", value);
        }

        public String getFooter() {
            return asText(getProp("footer"));
        }

        public void setFooter(String value) {
            setProp("footer", value);
        }

        public boolean isIncrementalSearch() {
            return asBool(getProp("incremental_search"));
        }

        public void setIncrementalSearch(boolean value) {
            setProp("incremental_search", value);
        }

        public String getCursorColor() {
            return asText(getProp("cursor_color"));
        }

        public void setCursorColor(String value) {
            setProp("cursor_color", value);
        }

        public boolean isIndicator() {
            return asBool(getProp("indicator"));
        }

        public void setIndicator(boolean value) {
            setProp("indicator", value);
        }

        public boolean isHighlightCurrent() {
            return asBool(getProp("highlight_current"));
        }

        public void setHighlightCurrent(boolean value) {
            setProp("highlight_current", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um DataSource de um form. */
    public static class DataSource extends Aylis.AylisComponent {
        public DataSource(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getDataSet() {
            return asText(getProp("data_set"));
        }

        public void setDataSet(String value) {
            setProp("data_set", value);
        }

        public boolean isAutoEdit() {
            return asBool(getProp("auto_edit"));
        }

        public void setAutoEdit(boolean value) {
            setProp("auto_edit", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }
    }

    /** Um Field de um form. */
    public static class Field extends Aylis.AylisComponent {
        public Field(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getFieldName() {
            return asText(getProp("field_name"));
        }

        public void setFieldName(String value) {
            setProp("field_name", value);
        }

        public String getType() {
            return asText(getProp("type"));
        }

        public void setType(String value) {
            setProp("type", value);
        }

        public long getSize() {
            return (long) asNumber(getProp("size"));
        }

        public void setSize(long value) {
            setProp("size", value);
        }

        public long getScale() {
            return (long) asNumber(getProp("scale"));
        }

        public void setScale(long value) {
            setProp("scale", value);
        }

        public String getRounding() {
            return asText(getProp("rounding"));
        }

        public void setRounding(String value) {
            setProp("rounding", value);
        }

        public String getDisplayLabel() {
            return asText(getProp("display_label"));
        }

        public void setDisplayLabel(String value) {
            setProp("display_label", value);
        }

        public double getDisplayWidth() {
            return asNumber(getProp("display_width"));
        }

        public void setDisplayWidth(double value) {
            setProp("display_width", value);
        }

        public String getDisplayFormat() {
            return asText(getProp("display_format"));
        }

        public void setDisplayFormat(String value) {
            setProp("display_format", value);
        }

        public String getEditFormat() {
            return asText(getProp("edit_format"));
        }

        public void setEditFormat(String value) {
            setProp("edit_format", value);
        }

        public String getAlignment() {
            return asText(getProp("alignment"));
        }

        public void setAlignment(String value) {
            setProp("alignment", value);
        }

        public boolean isRequired() {
            return asBool(getProp("required"));
        }

        public void setRequired(boolean value) {
            setProp("required", value);
        }

        public boolean isReadOnly() {
            return asBool(getProp("read_only"));
        }

        public void setReadOnly(boolean value) {
            setProp("read_only", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public double getMin() {
            return asNumber(getProp("min"));
        }

        public void setMin(double value) {
            setProp("min", value);
        }

        public double getMax() {
            return asNumber(getProp("max"));
        }

        public void setMax(double value) {
            setProp("max", value);
        }

        public String getDefaultValue() {
            return asText(getProp("default_value"));
        }

        public void setDefaultValue(String value) {
            setProp("default_value", value);
        }

        public String getDisplayValues() {
            return asText(getProp("display_values"));
        }

        public void setDisplayValues(String value) {
            setProp("display_values", value);
        }

        public String getCharCase() {
            return asText(getProp("char_case"));
        }

        public void setCharCase(String value) {
            setProp("char_case", value);
        }

        public String getEditMask() {
            return asText(getProp("edit_mask"));
        }

        public void setEditMask(String value) {
            setProp("edit_mask", value);
        }

        public boolean isSensitive() {
            return asBool(getProp("sensitive"));
        }

        public void setSensitive(boolean value) {
            setProp("sensitive", value);
        }

        public String getKind() {
            return asText(getProp("kind"));
        }

        public void setKind(String value) {
            setProp("kind", value);
        }

        public String getExpression() {
            return asText(getProp("expression"));
        }

        public void setExpression(String value) {
            setProp("expression", value);
        }

        public String getLookupDataSet() {
            return asText(getProp("lookup_data_set"));
        }

        public void setLookupDataSet(String value) {
            setProp("lookup_data_set", value);
        }

        public String getKeyFields() {
            return asText(getProp("key_fields"));
        }

        public void setKeyFields(String value) {
            setProp("key_fields", value);
        }

        public String getLookupKeyFields() {
            return asText(getProp("lookup_key_fields"));
        }

        public void setLookupKeyFields(String value) {
            setProp("lookup_key_fields", value);
        }

        public String getLookupResultField() {
            return asText(getProp("lookup_result_field"));
        }

        public void setLookupResultField(String value) {
            setProp("lookup_result_field", value);
        }
    }

    /** Um Form de um form. */
    public static class Form extends Aylis.AylisForm {
        public Form(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getCaption() {
            return asText(getProp("caption"));
        }

        public void setCaption(String value) {
            setProp("caption", value);
        }

        public String getPosition() {
            return asText(getProp("position"));
        }

        public void setPosition(String value) {
            setProp("position", value);
        }

        public String getBorderStyle() {
            return asText(getProp("border_style"));
        }

        public void setBorderStyle(String value) {
            setProp("border_style", value);
        }

        public String getWindowState() {
            return asText(getProp("window_state"));
        }

        public void setWindowState(String value) {
            setProp("window_state", value);
        }

        public String getTransition() {
            return asText(getProp("transition"));
        }

        public void setTransition(String value) {
            setProp("transition", value);
        }

        public boolean isTopmost() {
            return asBool(getProp("topmost"));
        }

        public void setTopmost(boolean value) {
            setProp("topmost", value);
        }

        public boolean isShowInTaskbar() {
            return asBool(getProp("show_in_taskbar"));
        }

        public void setShowInTaskbar(boolean value) {
            setProp("show_in_taskbar", value);
        }

        public String getActiveControl() {
            return asText(getProp("active_control"));
        }

        public void setActiveControl(String value) {
            setProp("active_control", value);
        }

        public String getTabOrder() {
            return asText(getProp("tab_order"));
        }

        public void setTabOrder(String value) {
            setProp("tab_order", value);
        }

        public String getTabKeys() {
            return asText(getProp("tab_keys"));
        }

        public void setTabKeys(String value) {
            setProp("tab_keys", value);
        }

        public String getBackTabKeys() {
            return asText(getProp("back_tab_keys"));
        }

        public void setBackTabKeys(String value) {
            setProp("back_tab_keys", value);
        }

        public String getAllowedChars() {
            return asText(getProp("allowed_chars"));
        }

        public void setAllowedChars(String value) {
            setProp("allowed_chars", value);
        }

        public String getBlockedChars() {
            return asText(getProp("blocked_chars"));
        }

        public void setBlockedChars(String value) {
            setProp("blocked_chars", value);
        }

        public boolean isOnlyAllowed() {
            return asBool(getProp("only_allowed"));
        }

        public void setOnlyAllowed(boolean value) {
            setProp("only_allowed", value);
        }

        public boolean isCharsOverride() {
            return asBool(getProp("chars_override"));
        }

        public void setCharsOverride(boolean value) {
            setProp("chars_override", value);
        }

        public String getCharMap() {
            return asText(getProp("char_map"));
        }

        public void setCharMap(String value) {
            setProp("char_map", value);
        }

        public boolean isKeyPreview() {
            return asBool(getProp("key_preview"));
        }

        public void setKeyPreview(boolean value) {
            setProp("key_preview", value);
        }

        public double getZoom() {
            return asNumber(getProp("zoom"));
        }

        public void setZoom(double value) {
            setProp("zoom", value);
        }

        public long getMonitor() {
            return (long) asNumber(getProp("monitor"));
        }

        public void setMonitor(long value) {
            setProp("monitor", value);
        }

        public boolean isRememberPlacement() {
            return asBool(getProp("remember_placement"));
        }

        public void setRememberPlacement(boolean value) {
            setProp("remember_placement", value);
        }

        public String getGpu() {
            return asText(getProp("gpu"));
        }

        public void setGpu(String value) {
            setProp("gpu", value);
        }

        public boolean isActivateOnShow() {
            return asBool(getProp("activate_on_show"));
        }

        public void setActivateOnShow(boolean value) {
            setProp("activate_on_show", value);
        }

        public String getCloseAction() {
            return asText(getProp("close_action"));
        }

        public void setCloseAction(String value) {
            setProp("close_action", value);
        }

        public double getFrameRate() {
            return asNumber(getProp("frame_rate"));
        }

        public void setFrameRate(double value) {
            setProp("frame_rate", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }

        public void moveToMonitor(long monitor) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "move_to_monitor", "args", java.util.List.of(monitor)));
        }

        public String gpuAdapter() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapter", "args", java.util.List.of())));
        }

        public String gpuAdapters() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapters", "args", java.util.List.of())));
        }

        public String focused() {
            return asText(aylis.call("focused", Aylis.obj("target", id)));
        }

        public void focusNext() {
            aylis.send("focus_next", Aylis.obj("target", id));
        }

        public void focusPrev() {
            aylis.send("focus_prev", Aylis.obj("target", id));
        }
    }

    /** Um GroupBox de um form. */
    public static class GroupBox extends Aylis.AylisComponent {
        public GroupBox(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getCaption() {
            return asText(getProp("caption"));
        }

        public void setCaption(String value) {
            setProp("caption", value);
        }

        public String getAllowedChars() {
            return asText(getProp("allowed_chars"));
        }

        public void setAllowedChars(String value) {
            setProp("allowed_chars", value);
        }

        public String getBlockedChars() {
            return asText(getProp("blocked_chars"));
        }

        public void setBlockedChars(String value) {
            setProp("blocked_chars", value);
        }

        public boolean isOnlyAllowed() {
            return asBool(getProp("only_allowed"));
        }

        public void setOnlyAllowed(boolean value) {
            setProp("only_allowed", value);
        }

        public boolean isCharsOverride() {
            return asBool(getProp("chars_override"));
        }

        public void setCharsOverride(boolean value) {
            setProp("chars_override", value);
        }

        public String getCharMap() {
            return asText(getProp("char_map"));
        }

        public void setCharMap(String value) {
            setProp("char_map", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Image de um form. */
    public static class Image extends Aylis.AylisComponent {
        public Image(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getImage() {
            return asText(getProp("image"));
        }

        public void setImage(String value) {
            setProp("image", value);
        }

        public String getFit() {
            return asText(getProp("fit"));
        }

        public void setFit(String value) {
            setProp("fit", value);
        }

        public double getRadius() {
            return asNumber(getProp("radius"));
        }

        public void setRadius(double value) {
            setProp("radius", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Label de um form. */
    public static class Label extends Aylis.AylisComponent {
        public Label(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
        }

        public String getHAlign() {
            return asText(getProp("h_align"));
        }

        public void setHAlign(String value) {
            setProp("h_align", value);
        }

        public String getVAlign() {
            return asText(getProp("v_align"));
        }

        public void setVAlign(String value) {
            setProp("v_align", value);
        }

        public boolean isWrap() {
            return asBool(getProp("wrap"));
        }

        public void setWrap(boolean value) {
            setProp("wrap", value);
        }

        public String getTrimming() {
            return asText(getProp("trimming"));
        }

        public void setTrimming(String value) {
            setProp("trimming", value);
        }

        public boolean isBold() {
            return asBool(getProp("bold"));
        }

        public void setBold(boolean value) {
            setProp("bold", value);
        }

        public String getColor() {
            return asText(getProp("color"));
        }

        public void setColor(String value) {
            setProp("color", value);
        }

        public String getHighlight() {
            return asText(getProp("highlight"));
        }

        public void setHighlight(String value) {
            setProp("highlight", value);
        }

        public double getLetterSpacing() {
            return asNumber(getProp("letter_spacing"));
        }

        public void setLetterSpacing(double value) {
            setProp("letter_spacing", value);
        }

        public String getTextCase() {
            return asText(getProp("text_case"));
        }

        public void setTextCase(String value) {
            setProp("text_case", value);
        }

        public boolean isMarkup() {
            return asBool(getProp("markup"));
        }

        public void setMarkup(boolean value) {
            setProp("markup", value);
        }

        public String getOutline() {
            return asText(getProp("outline"));
        }

        public void setOutline(String value) {
            setProp("outline", value);
        }

        public double getOutlineWidth() {
            return asNumber(getProp("outline_width"));
        }

        public void setOutlineWidth(double value) {
            setProp("outline_width", value);
        }

        public double getLineHeight() {
            return asNumber(getProp("line_height"));
        }

        public void setLineHeight(double value) {
            setProp("line_height", value);
        }

        public String getTextShadow() {
            return asText(getProp("text_shadow"));
        }

        public void setTextShadow(String value) {
            setProp("text_shadow", value);
        }

        public String getFocusControl() {
            return asText(getProp("focus_control"));
        }

        public void setFocusControl(String value) {
            setProp("focus_control", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }

        public double minContentWidth() {
            return asNumber(aylis.call("invoke", Aylis.obj("target", id, "method", "min_content_width", "args", java.util.List.of())));
        }

        public double maxContentWidth() {
            return asNumber(aylis.call("invoke", Aylis.obj("target", id, "method", "max_content_width", "args", java.util.List.of())));
        }
    }

    /** Um Layout de um form. */
    public static class Layout extends Aylis.AylisComponent {
        public Layout(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getAllowedChars() {
            return asText(getProp("allowed_chars"));
        }

        public void setAllowedChars(String value) {
            setProp("allowed_chars", value);
        }

        public String getBlockedChars() {
            return asText(getProp("blocked_chars"));
        }

        public void setBlockedChars(String value) {
            setProp("blocked_chars", value);
        }

        public boolean isOnlyAllowed() {
            return asBool(getProp("only_allowed"));
        }

        public void setOnlyAllowed(boolean value) {
            setProp("only_allowed", value);
        }

        public boolean isCharsOverride() {
            return asBool(getProp("chars_override"));
        }

        public void setCharsOverride(boolean value) {
            setProp("chars_override", value);
        }

        public String getCharMap() {
            return asText(getProp("char_map"));
        }

        public void setCharMap(String value) {
            setProp("char_map", value);
        }

        public double getGap() {
            return asNumber(getProp("gap"));
        }

        public void setGap(double value) {
            setProp("gap", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Panel de um form. */
    public static class Panel extends Aylis.AylisComponent {
        public Panel(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getAllowedChars() {
            return asText(getProp("allowed_chars"));
        }

        public void setAllowedChars(String value) {
            setProp("allowed_chars", value);
        }

        public String getBlockedChars() {
            return asText(getProp("blocked_chars"));
        }

        public void setBlockedChars(String value) {
            setProp("blocked_chars", value);
        }

        public boolean isOnlyAllowed() {
            return asBool(getProp("only_allowed"));
        }

        public void setOnlyAllowed(boolean value) {
            setProp("only_allowed", value);
        }

        public boolean isCharsOverride() {
            return asBool(getProp("chars_override"));
        }

        public void setCharsOverride(boolean value) {
            setProp("chars_override", value);
        }

        public String getCharMap() {
            return asText(getProp("char_map"));
        }

        public void setCharMap(String value) {
            setProp("char_map", value);
        }

        public double getGap() {
            return asNumber(getProp("gap"));
        }

        public void setGap(double value) {
            setProp("gap", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getFill2() {
            return asText(getProp("fill2"));
        }

        public void setFill2(String value) {
            setProp("fill2", value);
        }

        public String getFillMid() {
            return asText(getProp("fill_mid"));
        }

        public void setFillMid(String value) {
            setProp("fill_mid", value);
        }

        public double getFillMidAt() {
            return asNumber(getProp("fill_mid_at"));
        }

        public void setFillMidAt(double value) {
            setProp("fill_mid_at", value);
        }

        public double getGradientAngle() {
            return asNumber(getProp("gradient_angle"));
        }

        public void setGradientAngle(double value) {
            setProp("gradient_angle", value);
        }

        public boolean isGradientRadial() {
            return asBool(getProp("gradient_radial"));
        }

        public void setGradientRadial(boolean value) {
            setProp("gradient_radial", value);
        }

        public String getStroke() {
            return asText(getProp("stroke"));
        }

        public void setStroke(String value) {
            setProp("stroke", value);
        }

        public double getStrokeWidth() {
            return asNumber(getProp("stroke_width"));
        }

        public void setStrokeWidth(double value) {
            setProp("stroke_width", value);
        }

        public String getRadius() {
            return asText(getProp("radius"));
        }

        public void setRadius(String value) {
            setProp("radius", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Splitter de um form. */
    public static class Splitter extends Aylis.AylisComponent {
        public Splitter(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public double getMinSize() {
            return asNumber(getProp("min_size"));
        }

        public void setMinSize(double value) {
            setProp("min_size", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Svg de um form. */
    public static class Svg extends Aylis.AylisComponent {
        public Svg(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getSource() {
            return asText(getProp("source"));
        }

        public void setSource(String value) {
            setProp("source", value);
        }

        public String getColor() {
            return asText(getProp("color"));
        }

        public void setColor(String value) {
            setProp("color", value);
        }

        public String getFit() {
            return asText(getProp("fit"));
        }

        public void setFit(String value) {
            setProp("fit", value);
        }

        public String getHighlight() {
            return asText(getProp("highlight"));
        }

        public void setHighlight(String value) {
            setProp("highlight", value);
        }

        public String getHighlightColor() {
            return asText(getProp("highlight_color"));
        }

        public void setHighlightColor(String value) {
            setProp("highlight_color", value);
        }

        public String getZoomTo() {
            return asText(getProp("zoom_to"));
        }

        public void setZoomTo(String value) {
            setProp("zoom_to", value);
        }

        public double getLeft() {
            return asNumber(getProp("left"));
        }

        public void setLeft(double value) {
            setProp("left", value);
        }

        public double getTop() {
            return asNumber(getProp("top"));
        }

        public void setTop(double value) {
            setProp("top", value);
        }

        public double getWidth() {
            return asNumber(getProp("width"));
        }

        public void setWidth(double value) {
            setProp("width", value);
        }

        public double getHeight() {
            return asNumber(getProp("height"));
        }

        public void setHeight(double value) {
            setProp("height", value);
        }

        public String getAlign() {
            return asText(getProp("align"));
        }

        public void setAlign(String value) {
            setProp("align", value);
        }

        public String getAnchors() {
            return asText(getProp("anchors"));
        }

        public void setAnchors(String value) {
            setProp("anchors", value);
        }

        public String getMargins() {
            return asText(getProp("margins"));
        }

        public void setMargins(String value) {
            setProp("margins", value);
        }

        public String getPadding() {
            return asText(getProp("padding"));
        }

        public void setPadding(String value) {
            setProp("padding", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isHitTest() {
            return asBool(getProp("hit_test"));
        }

        public void setHitTest(boolean value) {
            setProp("hit_test", value);
        }

        public boolean isClipChildren() {
            return asBool(getProp("clip_children"));
        }

        public void setClipChildren(boolean value) {
            setProp("clip_children", value);
        }

        public boolean isInheritState() {
            return asBool(getProp("inherit_state"));
        }

        public void setInheritState(boolean value) {
            setProp("inherit_state", value);
        }

        public boolean isTranslate() {
            return asBool(getProp("translate"));
        }

        public void setTranslate(boolean value) {
            setProp("translate", value);
        }

        public boolean isCapturesWheel() {
            return asBool(getProp("captures_wheel"));
        }

        public void setCapturesWheel(boolean value) {
            setProp("captures_wheel", value);
        }

        public boolean isCanFocus() {
            return asBool(getProp("can_focus"));
        }

        public void setCanFocus(boolean value) {
            setProp("can_focus", value);
        }

        public boolean isDraggable() {
            return asBool(getProp("draggable"));
        }

        public void setDraggable(boolean value) {
            setProp("draggable", value);
        }

        public boolean isAcceptsDrops() {
            return asBool(getProp("accepts_drops"));
        }

        public void setAcceptsDrops(boolean value) {
            setProp("accepts_drops", value);
        }

        public String getDragData() {
            return asText(getProp("drag_data"));
        }

        public void setDragData(String value) {
            setProp("drag_data", value);
        }

        public String getDragLabel() {
            return asText(getProp("drag_label"));
        }

        public void setDragLabel(String value) {
            setProp("drag_label", value);
        }

        public String getDragImage() {
            return asText(getProp("drag_image"));
        }

        public void setDragImage(String value) {
            setProp("drag_image", value);
        }

        public String getDropTypes() {
            return asText(getProp("drop_types"));
        }

        public void setDropTypes(String value) {
            setProp("drop_types", value);
        }

        public boolean isTabStop() {
            return asBool(getProp("tab_stop"));
        }

        public void setTabStop(boolean value) {
            setProp("tab_stop", value);
        }

        public String getCursor() {
            return asText(getProp("cursor"));
        }

        public void setCursor(String value) {
            setProp("cursor", value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public boolean isCacheLayer() {
            return asBool(getProp("cache_layer"));
        }

        public void setCacheLayer(boolean value) {
            setProp("cache_layer", value);
        }

        public double getRotation() {
            return asNumber(getProp("rotation"));
        }

        public void setRotation(double value) {
            setProp("rotation", value);
        }

        public double getTranslateX() {
            return asNumber(getProp("translate_x"));
        }

        public void setTranslateX(double value) {
            setProp("translate_x", value);
        }

        public double getTranslateY() {
            return asNumber(getProp("translate_y"));
        }

        public void setTranslateY(double value) {
            setProp("translate_y", value);
        }

        public double getAspectRatio() {
            return asNumber(getProp("aspect_ratio"));
        }

        public void setAspectRatio(double value) {
            setProp("aspect_ratio", value);
        }

        public double getMaxWidth() {
            return asNumber(getProp("max_width"));
        }

        public void setMaxWidth(double value) {
            setProp("max_width", value);
        }

        public double getMinWidth() {
            return asNumber(getProp("min_width"));
        }

        public void setMinWidth(double value) {
            setProp("min_width", value);
        }

        public double getMinHeight() {
            return asNumber(getProp("min_height"));
        }

        public void setMinHeight(double value) {
            setProp("min_height", value);
        }

        public double getMaxHeight() {
            return asNumber(getProp("max_height"));
        }

        public void setMaxHeight(double value) {
            setProp("max_height", value);
        }

        public String getSelfAlign() {
            return asText(getProp("self_align"));
        }

        public void setSelfAlign(String value) {
            setProp("self_align", value);
        }

        public boolean isAutoSize() {
            return asBool(getProp("auto_size"));
        }

        public void setAutoSize(boolean value) {
            setProp("auto_size", value);
        }

        public long getColumnSpan() {
            return (long) asNumber(getProp("column_span"));
        }

        public void setColumnSpan(long value) {
            setProp("column_span", value);
        }

        public long getGridRow() {
            return (long) asNumber(getProp("grid_row"));
        }

        public void setGridRow(long value) {
            setProp("grid_row", value);
        }

        public long getGridColumn() {
            return (long) asNumber(getProp("grid_column"));
        }

        public void setGridColumn(long value) {
            setProp("grid_column", value);
        }

        public long getRowSpan() {
            return (long) asNumber(getProp("row_span"));
        }

        public void setRowSpan(long value) {
            setProp("row_span", value);
        }

        public double getGrow() {
            return asNumber(getProp("grow"));
        }

        public void setGrow(double value) {
            setProp("grow", value);
        }

        public String getShadow() {
            return asText(getProp("shadow"));
        }

        public void setShadow(String value) {
            setProp("shadow", value);
        }

        public String getStyle() {
            return asText(getProp("style"));
        }

        public void setStyle(String value) {
            setProp("style", value);
        }

        public String getTheme() {
            return asText(getProp("theme"));
        }

        public void setTheme(String value) {
            setProp("theme", value);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
        }

        public String getBorderColor() {
            return asText(getProp("border_color"));
        }

        public void setBorderColor(String value) {
            setProp("border_color", value);
        }

        public String getTextColor() {
            return asText(getProp("text_color"));
        }

        public void setTextColor(String value) {
            setProp("text_color", value);
        }

        public String getAccentColor() {
            return asText(getProp("accent_color"));
        }

        public void setAccentColor(String value) {
            setProp("accent_color", value);
        }

        public String getSuccessColor() {
            return asText(getProp("success_color"));
        }

        public void setSuccessColor(String value) {
            setProp("success_color", value);
        }

        public String getWarningColor() {
            return asText(getProp("warning_color"));
        }

        public void setWarningColor(String value) {
            setProp("warning_color", value);
        }

        public String getDangerColor() {
            return asText(getProp("danger_color"));
        }

        public void setDangerColor(String value) {
            setProp("danger_color", value);
        }

        public String getFontFamily() {
            return asText(getProp("font_family"));
        }

        public void setFontFamily(String value) {
            setProp("font_family", value);
        }

        public double getFontSize() {
            return asNumber(getProp("font_size"));
        }

        public void setFontSize(double value) {
            setProp("font_size", value);
        }

        public String getFontWeight() {
            return asText(getProp("font_weight"));
        }

        public void setFontWeight(String value) {
            setProp("font_weight", value);
        }

        public boolean isFontItalic() {
            return asBool(getProp("font_italic"));
        }

        public void setFontItalic(boolean value) {
            setProp("font_italic", value);
        }

        public boolean isFontUnderline() {
            return asBool(getProp("font_underline"));
        }

        public void setFontUnderline(boolean value) {
            setProp("font_underline", value);
        }

        public boolean isFontStrikeout() {
            return asBool(getProp("font_strikeout"));
        }

        public void setFontStrikeout(boolean value) {
            setProp("font_strikeout", value);
        }

        public String getHint() {
            return asText(getProp("hint"));
        }

        public void setHint(String value) {
            setProp("hint", value);
        }

        public long getTag() {
            return (long) asNumber(getProp("tag"));
        }

        public void setTag(long value) {
            setProp("tag", value);
        }

        public String getTagString() {
            return asText(getProp("tag_string"));
        }

        public void setTagString(String value) {
            setProp("tag_string", value);
        }

        public String getAccessibleName() {
            return asText(getProp("accessible_name"));
        }

        public void setAccessibleName(String value) {
            setProp("accessible_name", value);
        }

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
    public static final class FrmIBGEComponents extends Form {
        public ClientDataSet cdsEstados;
        public Field cdsEstadosId;
        public Field cdsEstadosSigla;
        public Field cdsEstadosNome;
        public Field cdsEstadosRegiao;
        public Field cdsEstadosPopulacao;
        public Field cdsEstadosArea;
        public Field cdsEstadosDensidade;
        public Field cdsEstadosPib;
        public Field cdsEstadosPibPerCapita;
        public Field cdsEstadosEstimativa;
        public Field cdsEstadosEstimativaAno;
        public Field cdsEstadosMunicipios;
        public Field cdsEstadosBandeira;
        public DataSource dsEstados;
        public ClientDataSet cdsMunicipios;
        public Field cdsMunicipiosId;
        public Field cdsMunicipiosUfId;
        public Field cdsMunicipiosUf;
        public Field cdsMunicipiosNome;
        public Field cdsMunicipiosMesorregiao;
        public Field cdsMunicipiosMicrorregiao;
        public Field cdsMunicipiosRegiaoIntermediaria;
        public Field cdsMunicipiosRegiaoImediata;
        public Field cdsMunicipiosPopulacao;
        public Field cdsMunicipiosArea;
        public Field cdsMunicipiosDensidade;
        public Field cdsMunicipiosPib;
        public Field cdsMunicipiosPibParticipacao;
        public Field cdsMunicipiosPibPerCapita;
        public Field cdsMunicipiosEstimativa;
        public Field cdsMunicipiosEstimativaAno;
        public Field cdsMunicipiosDistritos;
        public Field cdsMunicipiosBandeira;
        public DataSource dsMunicipios;
        public ClientDataSet cdsDistritos;
        public Field cdsDistritosId;
        public Field cdsDistritosMunicipioId;
        public Field cdsDistritosNome;
        public DataSource dsDistritos;
        public ClientDataSet cdsPopulacao;
        public Field cdsPopulacaoAno;
        public Field cdsPopulacaoHabitantes;
        public DataSource dsPopulacao;
        public ClientDataSet cdsMaiores;
        public Field cdsMaioresNome;
        public Field cdsMaioresHabitantes;
        public DataSource dsMaiores;
        public Label lblFonte;
        public Layout lytListas;
        public GroupBox gbEstados;
        public DataGrid grdEstados;
        public GroupBox gbDistritos;
        public DataGrid grdDistritos;
        public GroupBox gbMunicipios;
        public DataGrid grdMunicipios;
        public Splitter splListas;
        public Layout lytPainel;
        public Panel pnlEstado;
        public Svg svgBrasil;
        public Layout lytBandeiraEstado;
        public Label lblEstado;
        public Label lblRegiao;
        public Image imgBandeiraEstado;
        public Layout lytNumerosEstado;
        public Label dbtEstadoPopulacao;
        public Label lblEstadoEstimativa;
        public Label dbtEstadoEstimativa;
        public Label dbtEstadoArea;
        public Label dbtEstadoDensidade;
        public Label dbtEstadoPib;
        public Label dbtEstadoPibPerCapita;
        public Label dbtEstadoMunicipios;
        public Panel pnlMunicipio;
        public Svg svgMunicipio;
        public Svg svgEstado;
        public Layout lytBandeiraMunicipio;
        public Label lblMunicipio;
        public Label lblCodigo;
        public Image imgBandeiraMunicipio;
        public Label lblSemBandeira;
        public Layout lytNumerosMunicipio;
        public Label dbtMesorregiao;
        public Label dbtMicrorregiao;
        public Label dbtRegiaoImediata;
        public Label dbtPopulacao;
        public Label lblEstimativa;
        public Label dbtEstimativa;
        public Label dbtArea;
        public Label dbtDensidade;
        public Label dbtPib;
        public Label dbtPibPerCapita;
        public Label dbtPibParticipacao;
        public Layout lytGraficos;
        public DBChart dbcPopulacao;
        public DBChart dbcMaiores;
        public DBChart dbcEstados;

        public FrmIBGEComponents(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        @Override
        public void bind(java.util.List<java.util.Map<String, Object>> list) {
            cdsEstados = resolve(list, "cdsEstados", "ClientDataSet", ClientDataSet::new);
            cdsEstadosId = resolve(list, "cdsEstadosId", "Field", Field::new);
            cdsEstadosSigla = resolve(list, "cdsEstadosSigla", "Field", Field::new);
            cdsEstadosNome = resolve(list, "cdsEstadosNome", "Field", Field::new);
            cdsEstadosRegiao = resolve(list, "cdsEstadosRegiao", "Field", Field::new);
            cdsEstadosPopulacao = resolve(list, "cdsEstadosPopulacao", "Field", Field::new);
            cdsEstadosArea = resolve(list, "cdsEstadosArea", "Field", Field::new);
            cdsEstadosDensidade = resolve(list, "cdsEstadosDensidade", "Field", Field::new);
            cdsEstadosPib = resolve(list, "cdsEstadosPib", "Field", Field::new);
            cdsEstadosPibPerCapita = resolve(list, "cdsEstadosPibPerCapita", "Field", Field::new);
            cdsEstadosEstimativa = resolve(list, "cdsEstadosEstimativa", "Field", Field::new);
            cdsEstadosEstimativaAno = resolve(list, "cdsEstadosEstimativaAno", "Field", Field::new);
            cdsEstadosMunicipios = resolve(list, "cdsEstadosMunicipios", "Field", Field::new);
            cdsEstadosBandeira = resolve(list, "cdsEstadosBandeira", "Field", Field::new);
            dsEstados = resolve(list, "dsEstados", "DataSource", DataSource::new);
            cdsMunicipios = resolve(list, "cdsMunicipios", "ClientDataSet", ClientDataSet::new);
            cdsMunicipiosId = resolve(list, "cdsMunicipiosId", "Field", Field::new);
            cdsMunicipiosUfId = resolve(list, "cdsMunicipiosUfId", "Field", Field::new);
            cdsMunicipiosUf = resolve(list, "cdsMunicipiosUf", "Field", Field::new);
            cdsMunicipiosNome = resolve(list, "cdsMunicipiosNome", "Field", Field::new);
            cdsMunicipiosMesorregiao = resolve(list, "cdsMunicipiosMesorregiao", "Field", Field::new);
            cdsMunicipiosMicrorregiao = resolve(list, "cdsMunicipiosMicrorregiao", "Field", Field::new);
            cdsMunicipiosRegiaoIntermediaria = resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field", Field::new);
            cdsMunicipiosRegiaoImediata = resolve(list, "cdsMunicipiosRegiaoImediata", "Field", Field::new);
            cdsMunicipiosPopulacao = resolve(list, "cdsMunicipiosPopulacao", "Field", Field::new);
            cdsMunicipiosArea = resolve(list, "cdsMunicipiosArea", "Field", Field::new);
            cdsMunicipiosDensidade = resolve(list, "cdsMunicipiosDensidade", "Field", Field::new);
            cdsMunicipiosPib = resolve(list, "cdsMunicipiosPib", "Field", Field::new);
            cdsMunicipiosPibParticipacao = resolve(list, "cdsMunicipiosPibParticipacao", "Field", Field::new);
            cdsMunicipiosPibPerCapita = resolve(list, "cdsMunicipiosPibPerCapita", "Field", Field::new);
            cdsMunicipiosEstimativa = resolve(list, "cdsMunicipiosEstimativa", "Field", Field::new);
            cdsMunicipiosEstimativaAno = resolve(list, "cdsMunicipiosEstimativaAno", "Field", Field::new);
            cdsMunicipiosDistritos = resolve(list, "cdsMunicipiosDistritos", "Field", Field::new);
            cdsMunicipiosBandeira = resolve(list, "cdsMunicipiosBandeira", "Field", Field::new);
            dsMunicipios = resolve(list, "dsMunicipios", "DataSource", DataSource::new);
            cdsDistritos = resolve(list, "cdsDistritos", "ClientDataSet", ClientDataSet::new);
            cdsDistritosId = resolve(list, "cdsDistritosId", "Field", Field::new);
            cdsDistritosMunicipioId = resolve(list, "cdsDistritosMunicipioId", "Field", Field::new);
            cdsDistritosNome = resolve(list, "cdsDistritosNome", "Field", Field::new);
            dsDistritos = resolve(list, "dsDistritos", "DataSource", DataSource::new);
            cdsPopulacao = resolve(list, "cdsPopulacao", "ClientDataSet", ClientDataSet::new);
            cdsPopulacaoAno = resolve(list, "cdsPopulacaoAno", "Field", Field::new);
            cdsPopulacaoHabitantes = resolve(list, "cdsPopulacaoHabitantes", "Field", Field::new);
            dsPopulacao = resolve(list, "dsPopulacao", "DataSource", DataSource::new);
            cdsMaiores = resolve(list, "cdsMaiores", "ClientDataSet", ClientDataSet::new);
            cdsMaioresNome = resolve(list, "cdsMaioresNome", "Field", Field::new);
            cdsMaioresHabitantes = resolve(list, "cdsMaioresHabitantes", "Field", Field::new);
            dsMaiores = resolve(list, "dsMaiores", "DataSource", DataSource::new);
            lblFonte = resolve(list, "lblFonte", "Label", Label::new);
            lytListas = resolve(list, "lytListas", "Layout", Layout::new);
            gbEstados = resolve(list, "gbEstados", "GroupBox", GroupBox::new);
            grdEstados = resolve(list, "grdEstados", "DataGrid", DataGrid::new);
            gbDistritos = resolve(list, "gbDistritos", "GroupBox", GroupBox::new);
            grdDistritos = resolve(list, "grdDistritos", "DataGrid", DataGrid::new);
            gbMunicipios = resolve(list, "gbMunicipios", "GroupBox", GroupBox::new);
            grdMunicipios = resolve(list, "grdMunicipios", "DataGrid", DataGrid::new);
            splListas = resolve(list, "splListas", "Splitter", Splitter::new);
            lytPainel = resolve(list, "lytPainel", "Layout", Layout::new);
            pnlEstado = resolve(list, "pnlEstado", "Panel", Panel::new);
            svgBrasil = resolve(list, "svgBrasil", "Svg", Svg::new);
            lytBandeiraEstado = resolve(list, "lytBandeiraEstado", "Layout", Layout::new);
            lblEstado = resolve(list, "lblEstado", "Label", Label::new);
            lblRegiao = resolve(list, "lblRegiao", "Label", Label::new);
            imgBandeiraEstado = resolve(list, "imgBandeiraEstado", "Image", Image::new);
            lytNumerosEstado = resolve(list, "lytNumerosEstado", "Layout", Layout::new);
            dbtEstadoPopulacao = resolve(list, "dbtEstadoPopulacao", "Label", Label::new);
            lblEstadoEstimativa = resolve(list, "lblEstadoEstimativa", "Label", Label::new);
            dbtEstadoEstimativa = resolve(list, "dbtEstadoEstimativa", "Label", Label::new);
            dbtEstadoArea = resolve(list, "dbtEstadoArea", "Label", Label::new);
            dbtEstadoDensidade = resolve(list, "dbtEstadoDensidade", "Label", Label::new);
            dbtEstadoPib = resolve(list, "dbtEstadoPib", "Label", Label::new);
            dbtEstadoPibPerCapita = resolve(list, "dbtEstadoPibPerCapita", "Label", Label::new);
            dbtEstadoMunicipios = resolve(list, "dbtEstadoMunicipios", "Label", Label::new);
            pnlMunicipio = resolve(list, "pnlMunicipio", "Panel", Panel::new);
            svgMunicipio = resolve(list, "svgMunicipio", "Svg", Svg::new);
            svgEstado = resolve(list, "svgEstado", "Svg", Svg::new);
            lytBandeiraMunicipio = resolve(list, "lytBandeiraMunicipio", "Layout", Layout::new);
            lblMunicipio = resolve(list, "lblMunicipio", "Label", Label::new);
            lblCodigo = resolve(list, "lblCodigo", "Label", Label::new);
            imgBandeiraMunicipio = resolve(list, "imgBandeiraMunicipio", "Image", Image::new);
            lblSemBandeira = resolve(list, "lblSemBandeira", "Label", Label::new);
            lytNumerosMunicipio = resolve(list, "lytNumerosMunicipio", "Layout", Layout::new);
            dbtMesorregiao = resolve(list, "dbtMesorregiao", "Label", Label::new);
            dbtMicrorregiao = resolve(list, "dbtMicrorregiao", "Label", Label::new);
            dbtRegiaoImediata = resolve(list, "dbtRegiaoImediata", "Label", Label::new);
            dbtPopulacao = resolve(list, "dbtPopulacao", "Label", Label::new);
            lblEstimativa = resolve(list, "lblEstimativa", "Label", Label::new);
            dbtEstimativa = resolve(list, "dbtEstimativa", "Label", Label::new);
            dbtArea = resolve(list, "dbtArea", "Label", Label::new);
            dbtDensidade = resolve(list, "dbtDensidade", "Label", Label::new);
            dbtPib = resolve(list, "dbtPib", "Label", Label::new);
            dbtPibPerCapita = resolve(list, "dbtPibPerCapita", "Label", Label::new);
            dbtPibParticipacao = resolve(list, "dbtPibParticipacao", "Label", Label::new);
            lytGraficos = resolve(list, "lytGraficos", "Layout", Layout::new);
            dbcPopulacao = resolve(list, "dbcPopulacao", "DBChart", DBChart::new);
            dbcMaiores = resolve(list, "dbcMaiores", "DBChart", DBChart::new);
            dbcEstados = resolve(list, "dbcEstados", "DBChart", DBChart::new);
        }

        /** O ClientDataSet deste form com o nome (como o FindComponent). */
        public ClientDataSet findClientDataSet(String name) {
            return switch (name) {
                case "cdsEstados" -> cdsEstados;
                case "cdsMunicipios" -> cdsMunicipios;
                case "cdsDistritos" -> cdsDistritos;
                case "cdsPopulacao" -> cdsPopulacao;
                case "cdsMaiores" -> cdsMaiores;
                default -> null;
            };
        }

        /** O ClientDataSet que disparou o evento. */
        public ClientDataSet senderAsClientDataSet(Aylis.Event ev) {
            return findClientDataSet(String.valueOf(ev.sender().get("name")));
        }

        /** O DBChart deste form com o nome (como o FindComponent). */
        public DBChart findDBChart(String name) {
            return switch (name) {
                case "dbcPopulacao" -> dbcPopulacao;
                case "dbcMaiores" -> dbcMaiores;
                case "dbcEstados" -> dbcEstados;
                default -> null;
            };
        }

        /** O DBChart que disparou o evento. */
        public DBChart senderAsDBChart(Aylis.Event ev) {
            return findDBChart(String.valueOf(ev.sender().get("name")));
        }

        /** O DataGrid deste form com o nome (como o FindComponent). */
        public DataGrid findDataGrid(String name) {
            return switch (name) {
                case "grdEstados" -> grdEstados;
                case "grdDistritos" -> grdDistritos;
                case "grdMunicipios" -> grdMunicipios;
                default -> null;
            };
        }

        /** O DataGrid que disparou o evento. */
        public DataGrid senderAsDataGrid(Aylis.Event ev) {
            return findDataGrid(String.valueOf(ev.sender().get("name")));
        }

        /** O DataSource deste form com o nome (como o FindComponent). */
        public DataSource findDataSource(String name) {
            return switch (name) {
                case "dsEstados" -> dsEstados;
                case "dsMunicipios" -> dsMunicipios;
                case "dsDistritos" -> dsDistritos;
                case "dsPopulacao" -> dsPopulacao;
                case "dsMaiores" -> dsMaiores;
                default -> null;
            };
        }

        /** O DataSource que disparou o evento. */
        public DataSource senderAsDataSource(Aylis.Event ev) {
            return findDataSource(String.valueOf(ev.sender().get("name")));
        }

        /** O Field deste form com o nome (como o FindComponent). */
        public Field findField(String name) {
            return switch (name) {
                case "cdsEstadosId" -> cdsEstadosId;
                case "cdsEstadosSigla" -> cdsEstadosSigla;
                case "cdsEstadosNome" -> cdsEstadosNome;
                case "cdsEstadosRegiao" -> cdsEstadosRegiao;
                case "cdsEstadosPopulacao" -> cdsEstadosPopulacao;
                case "cdsEstadosArea" -> cdsEstadosArea;
                case "cdsEstadosDensidade" -> cdsEstadosDensidade;
                case "cdsEstadosPib" -> cdsEstadosPib;
                case "cdsEstadosPibPerCapita" -> cdsEstadosPibPerCapita;
                case "cdsEstadosEstimativa" -> cdsEstadosEstimativa;
                case "cdsEstadosEstimativaAno" -> cdsEstadosEstimativaAno;
                case "cdsEstadosMunicipios" -> cdsEstadosMunicipios;
                case "cdsEstadosBandeira" -> cdsEstadosBandeira;
                case "cdsMunicipiosId" -> cdsMunicipiosId;
                case "cdsMunicipiosUfId" -> cdsMunicipiosUfId;
                case "cdsMunicipiosUf" -> cdsMunicipiosUf;
                case "cdsMunicipiosNome" -> cdsMunicipiosNome;
                case "cdsMunicipiosMesorregiao" -> cdsMunicipiosMesorregiao;
                case "cdsMunicipiosMicrorregiao" -> cdsMunicipiosMicrorregiao;
                case "cdsMunicipiosRegiaoIntermediaria" -> cdsMunicipiosRegiaoIntermediaria;
                case "cdsMunicipiosRegiaoImediata" -> cdsMunicipiosRegiaoImediata;
                case "cdsMunicipiosPopulacao" -> cdsMunicipiosPopulacao;
                case "cdsMunicipiosArea" -> cdsMunicipiosArea;
                case "cdsMunicipiosDensidade" -> cdsMunicipiosDensidade;
                case "cdsMunicipiosPib" -> cdsMunicipiosPib;
                case "cdsMunicipiosPibParticipacao" -> cdsMunicipiosPibParticipacao;
                case "cdsMunicipiosPibPerCapita" -> cdsMunicipiosPibPerCapita;
                case "cdsMunicipiosEstimativa" -> cdsMunicipiosEstimativa;
                case "cdsMunicipiosEstimativaAno" -> cdsMunicipiosEstimativaAno;
                case "cdsMunicipiosDistritos" -> cdsMunicipiosDistritos;
                case "cdsMunicipiosBandeira" -> cdsMunicipiosBandeira;
                case "cdsDistritosId" -> cdsDistritosId;
                case "cdsDistritosMunicipioId" -> cdsDistritosMunicipioId;
                case "cdsDistritosNome" -> cdsDistritosNome;
                case "cdsPopulacaoAno" -> cdsPopulacaoAno;
                case "cdsPopulacaoHabitantes" -> cdsPopulacaoHabitantes;
                case "cdsMaioresNome" -> cdsMaioresNome;
                case "cdsMaioresHabitantes" -> cdsMaioresHabitantes;
                default -> null;
            };
        }

        /** O Field que disparou o evento. */
        public Field senderAsField(Aylis.Event ev) {
            return findField(String.valueOf(ev.sender().get("name")));
        }

        /** O GroupBox deste form com o nome (como o FindComponent). */
        public GroupBox findGroupBox(String name) {
            return switch (name) {
                case "gbEstados" -> gbEstados;
                case "gbDistritos" -> gbDistritos;
                case "gbMunicipios" -> gbMunicipios;
                default -> null;
            };
        }

        /** O GroupBox que disparou o evento. */
        public GroupBox senderAsGroupBox(Aylis.Event ev) {
            return findGroupBox(String.valueOf(ev.sender().get("name")));
        }

        /** O Image deste form com o nome (como o FindComponent). */
        public Image findImage(String name) {
            return switch (name) {
                case "imgBandeiraEstado" -> imgBandeiraEstado;
                case "imgBandeiraMunicipio" -> imgBandeiraMunicipio;
                default -> null;
            };
        }

        /** O Image que disparou o evento. */
        public Image senderAsImage(Aylis.Event ev) {
            return findImage(String.valueOf(ev.sender().get("name")));
        }

        /** O Label deste form com o nome (como o FindComponent). */
        public Label findLabel(String name) {
            return switch (name) {
                case "lblFonte" -> lblFonte;
                case "lblEstado" -> lblEstado;
                case "lblRegiao" -> lblRegiao;
                case "dbtEstadoPopulacao" -> dbtEstadoPopulacao;
                case "lblEstadoEstimativa" -> lblEstadoEstimativa;
                case "dbtEstadoEstimativa" -> dbtEstadoEstimativa;
                case "dbtEstadoArea" -> dbtEstadoArea;
                case "dbtEstadoDensidade" -> dbtEstadoDensidade;
                case "dbtEstadoPib" -> dbtEstadoPib;
                case "dbtEstadoPibPerCapita" -> dbtEstadoPibPerCapita;
                case "dbtEstadoMunicipios" -> dbtEstadoMunicipios;
                case "lblMunicipio" -> lblMunicipio;
                case "lblCodigo" -> lblCodigo;
                case "lblSemBandeira" -> lblSemBandeira;
                case "dbtMesorregiao" -> dbtMesorregiao;
                case "dbtMicrorregiao" -> dbtMicrorregiao;
                case "dbtRegiaoImediata" -> dbtRegiaoImediata;
                case "dbtPopulacao" -> dbtPopulacao;
                case "lblEstimativa" -> lblEstimativa;
                case "dbtEstimativa" -> dbtEstimativa;
                case "dbtArea" -> dbtArea;
                case "dbtDensidade" -> dbtDensidade;
                case "dbtPib" -> dbtPib;
                case "dbtPibPerCapita" -> dbtPibPerCapita;
                case "dbtPibParticipacao" -> dbtPibParticipacao;
                default -> null;
            };
        }

        /** O Label que disparou o evento. */
        public Label senderAsLabel(Aylis.Event ev) {
            return findLabel(String.valueOf(ev.sender().get("name")));
        }

        /** O Layout deste form com o nome (como o FindComponent). */
        public Layout findLayout(String name) {
            return switch (name) {
                case "lytListas" -> lytListas;
                case "lytPainel" -> lytPainel;
                case "lytBandeiraEstado" -> lytBandeiraEstado;
                case "lytNumerosEstado" -> lytNumerosEstado;
                case "lytBandeiraMunicipio" -> lytBandeiraMunicipio;
                case "lytNumerosMunicipio" -> lytNumerosMunicipio;
                case "lytGraficos" -> lytGraficos;
                default -> null;
            };
        }

        /** O Layout que disparou o evento. */
        public Layout senderAsLayout(Aylis.Event ev) {
            return findLayout(String.valueOf(ev.sender().get("name")));
        }

        /** O Panel deste form com o nome (como o FindComponent). */
        public Panel findPanel(String name) {
            return switch (name) {
                case "pnlEstado" -> pnlEstado;
                case "pnlMunicipio" -> pnlMunicipio;
                default -> null;
            };
        }

        /** O Panel que disparou o evento. */
        public Panel senderAsPanel(Aylis.Event ev) {
            return findPanel(String.valueOf(ev.sender().get("name")));
        }

        /** O Splitter deste form com o nome (como o FindComponent). */
        public Splitter findSplitter(String name) {
            return switch (name) {
                case "splListas" -> splListas;
                default -> null;
            };
        }

        /** O Splitter que disparou o evento. */
        public Splitter senderAsSplitter(Aylis.Event ev) {
            return findSplitter(String.valueOf(ev.sender().get("name")));
        }

        /** O Svg deste form com o nome (como o FindComponent). */
        public Svg findSvg(String name) {
            return switch (name) {
                case "svgBrasil" -> svgBrasil;
                case "svgMunicipio" -> svgMunicipio;
                case "svgEstado" -> svgEstado;
                default -> null;
            };
        }

        /** O Svg que disparou o evento. */
        public Svg senderAsSvg(Aylis.Event ev) {
            return findSvg(String.valueOf(ev.sender().get("name")));
        }
    }
}
