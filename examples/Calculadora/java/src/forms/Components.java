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

    public static FrmCalculadoraComponents frmCalculadora;

    /** Registra as classes dos forms (o registro dos forms chama). */
    public static void registerClasses() {
        Aylis.registerFormClass("frmCalculadora", FrmCalculadoraComponents::new, form -> frmCalculadora = (FrmCalculadoraComponents) form);
    }

    /** Um Button de um form. */
    public static class Button extends Aylis.AylisComponent {
        public Button(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
        }

        public String getIcon() {
            return asText(getProp("icon"));
        }

        public void setIcon(String value) {
            setProp("icon", value);
        }

        public String getCheckedIcon() {
            return asText(getProp("checked_icon"));
        }

        public void setCheckedIcon(String value) {
            setProp("checked_icon", value);
        }

        public String getIconPosition() {
            return asText(getProp("icon_position"));
        }

        public void setIconPosition(String value) {
            setProp("icon_position", value);
        }

        public double getIconSize() {
            return asNumber(getProp("icon_size"));
        }

        public void setIconSize(double value) {
            setProp("icon_size", value);
        }

        public boolean isCheckable() {
            return asBool(getProp("checkable"));
        }

        public void setCheckable(boolean value) {
            setProp("checkable", value);
        }

        public boolean isChecked() {
            return asBool(getProp("checked"));
        }

        public void setChecked(boolean value) {
            setProp("checked", value);
        }

        public String getModalResult() {
            return asText(getProp("modal_result"));
        }

        public void setModalResult(String value) {
            setProp("modal_result", value);
        }

        public boolean isRepeatEnter() {
            return asBool(getProp("repeat_enter"));
        }

        public void setRepeatEnter(boolean value) {
            setProp("repeat_enter", value);
        }

        public boolean isDefaultButton() {
            return asBool(getProp("default_button"));
        }

        public void setDefaultButton(boolean value) {
            setProp("default_button", value);
        }

        public boolean isCancelButton() {
            return asBool(getProp("cancel_button"));
        }

        public void setCancelButton(boolean value) {
            setProp("cancel_button", value);
        }

        public String getHAlign() {
            return asText(getProp("h_align"));
        }

        public void setHAlign(String value) {
            setProp("h_align", value);
        }

        public long getGroupIndex() {
            return (long) asNumber(getProp("group_index"));
        }

        public void setGroupIndex(long value) {
            setProp("group_index", value);
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

    /** Um GridLayout de um form. */
    public static class GridLayout extends Aylis.AylisComponent {
        public GridLayout(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public long getColumns() {
            return (long) asNumber(getProp("columns"));
        }

        public void setColumns(long value) {
            setProp("columns", value);
        }

        public double getMinColumnWidth() {
            return asNumber(getProp("min_column_width"));
        }

        public void setMinColumnWidth(double value) {
            setProp("min_column_width", value);
        }

        public double getGap() {
            return asNumber(getProp("gap"));
        }

        public void setGap(double value) {
            setProp("gap", value);
        }

        public double getRowHeight() {
            return asNumber(getProp("row_height"));
        }

        public void setRowHeight(double value) {
            setProp("row_height", value);
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

    /** Um ListView de um form. */
    public static class ListView extends Aylis.AylisComponent {
        public ListView(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getItems() {
            return asText(getProp("items"));
        }

        public void setItems(String value) {
            setProp("items", value);
        }

        public long getItemIndex() {
            return (long) asNumber(getProp("item_index"));
        }

        public void setItemIndex(long value) {
            setProp("item_index", value);
        }

        public double getItemHeight() {
            return asNumber(getProp("item_height"));
        }

        public void setItemHeight(double value) {
            setProp("item_height", value);
        }

        public boolean isAccessory() {
            return asBool(getProp("accessory"));
        }

        public void setAccessory(boolean value) {
            setProp("accessory", value);
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

    /** Um TabControl de um form. */
    public static class TabControl extends Aylis.AylisComponent {
        public TabControl(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public long getTabIndex() {
            return (long) asNumber(getProp("tab_index"));
        }

        public void setTabIndex(long value) {
            setProp("tab_index", value);
        }

        public boolean isShowTabs() {
            return asBool(getProp("show_tabs"));
        }

        public void setShowTabs(boolean value) {
            setProp("show_tabs", value);
        }

        public String getTransition() {
            return asText(getProp("transition"));
        }

        public void setTransition(String value) {
            setProp("transition", value);
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

    /** Um TabItem de um form. */
    public static class TabItem extends Aylis.AylisComponent {
        public TabItem(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getCaption() {
            return asText(getProp("caption"));
        }

        public void setCaption(String value) {
            setProp("caption", value);
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

    /** O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
    public static final class FrmCalculadoraComponents extends Form {
        public Panel pnlLateral;
        public TabControl tabLateral;
        public TabItem tabHistorico;
        public Button btnLimparHistorico;
        public ListView lvHistorico;
        public TabItem tabMemoria;
        public ListView lvMemoria;
        public Layout lytCorpo;
        public Layout lytTopo;
        public Button btnHistorico;
        public Label lblTitulo;
        public Label lblExpressao;
        public Label lblVisor;
        public GridLayout grdMemoria;
        public Button btnMC;
        public Button btnMR;
        public Button btnMMais;
        public Button btnMMenos;
        public Button btnMS;
        public GridLayout grdTeclado;
        public Button btnPorcento;
        public Button btnCE;
        public Button btnC;
        public Button btnApagar;
        public Button btnInverso;
        public Button btnQuadrado;
        public Button btnRaiz;
        public Button btnDividir;
        public Button btn7;
        public Button btn8;
        public Button btn9;
        public Button btnMultiplicar;
        public Button btn4;
        public Button btn5;
        public Button btn6;
        public Button btnSubtrair;
        public Button btn1;
        public Button btn2;
        public Button btn3;
        public Button btnSomar;
        public Button btnSinal;
        public Button btn0;
        public Button btnVirgula;
        public Button btnIgual;

        public FrmCalculadoraComponents(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        @Override
        public void bind(java.util.List<java.util.Map<String, Object>> list) {
            pnlLateral = resolve(list, "pnlLateral", "Panel", Panel::new);
            tabLateral = resolve(list, "tabLateral", "TabControl", TabControl::new);
            tabHistorico = resolve(list, "tabHistorico", "TabItem", TabItem::new);
            btnLimparHistorico = resolve(list, "btnLimparHistorico", "Button", Button::new);
            lvHistorico = resolve(list, "lvHistorico", "ListView", ListView::new);
            tabMemoria = resolve(list, "tabMemoria", "TabItem", TabItem::new);
            lvMemoria = resolve(list, "lvMemoria", "ListView", ListView::new);
            lytCorpo = resolve(list, "lytCorpo", "Layout", Layout::new);
            lytTopo = resolve(list, "lytTopo", "Layout", Layout::new);
            btnHistorico = resolve(list, "btnHistorico", "Button", Button::new);
            lblTitulo = resolve(list, "lblTitulo", "Label", Label::new);
            lblExpressao = resolve(list, "lblExpressao", "Label", Label::new);
            lblVisor = resolve(list, "lblVisor", "Label", Label::new);
            grdMemoria = resolve(list, "grdMemoria", "GridLayout", GridLayout::new);
            btnMC = resolve(list, "btnMC", "Button", Button::new);
            btnMR = resolve(list, "btnMR", "Button", Button::new);
            btnMMais = resolve(list, "btnMMais", "Button", Button::new);
            btnMMenos = resolve(list, "btnMMenos", "Button", Button::new);
            btnMS = resolve(list, "btnMS", "Button", Button::new);
            grdTeclado = resolve(list, "grdTeclado", "GridLayout", GridLayout::new);
            btnPorcento = resolve(list, "btnPorcento", "Button", Button::new);
            btnCE = resolve(list, "btnCE", "Button", Button::new);
            btnC = resolve(list, "btnC", "Button", Button::new);
            btnApagar = resolve(list, "btnApagar", "Button", Button::new);
            btnInverso = resolve(list, "btnInverso", "Button", Button::new);
            btnQuadrado = resolve(list, "btnQuadrado", "Button", Button::new);
            btnRaiz = resolve(list, "btnRaiz", "Button", Button::new);
            btnDividir = resolve(list, "btnDividir", "Button", Button::new);
            btn7 = resolve(list, "btn7", "Button", Button::new);
            btn8 = resolve(list, "btn8", "Button", Button::new);
            btn9 = resolve(list, "btn9", "Button", Button::new);
            btnMultiplicar = resolve(list, "btnMultiplicar", "Button", Button::new);
            btn4 = resolve(list, "btn4", "Button", Button::new);
            btn5 = resolve(list, "btn5", "Button", Button::new);
            btn6 = resolve(list, "btn6", "Button", Button::new);
            btnSubtrair = resolve(list, "btnSubtrair", "Button", Button::new);
            btn1 = resolve(list, "btn1", "Button", Button::new);
            btn2 = resolve(list, "btn2", "Button", Button::new);
            btn3 = resolve(list, "btn3", "Button", Button::new);
            btnSomar = resolve(list, "btnSomar", "Button", Button::new);
            btnSinal = resolve(list, "btnSinal", "Button", Button::new);
            btn0 = resolve(list, "btn0", "Button", Button::new);
            btnVirgula = resolve(list, "btnVirgula", "Button", Button::new);
            btnIgual = resolve(list, "btnIgual", "Button", Button::new);
        }

        /** O Button deste form com o nome (como o FindComponent). */
        public Button findButton(String name) {
            return switch (name) {
                case "btnLimparHistorico" -> btnLimparHistorico;
                case "btnHistorico" -> btnHistorico;
                case "btnMC" -> btnMC;
                case "btnMR" -> btnMR;
                case "btnMMais" -> btnMMais;
                case "btnMMenos" -> btnMMenos;
                case "btnMS" -> btnMS;
                case "btnPorcento" -> btnPorcento;
                case "btnCE" -> btnCE;
                case "btnC" -> btnC;
                case "btnApagar" -> btnApagar;
                case "btnInverso" -> btnInverso;
                case "btnQuadrado" -> btnQuadrado;
                case "btnRaiz" -> btnRaiz;
                case "btnDividir" -> btnDividir;
                case "btn7" -> btn7;
                case "btn8" -> btn8;
                case "btn9" -> btn9;
                case "btnMultiplicar" -> btnMultiplicar;
                case "btn4" -> btn4;
                case "btn5" -> btn5;
                case "btn6" -> btn6;
                case "btnSubtrair" -> btnSubtrair;
                case "btn1" -> btn1;
                case "btn2" -> btn2;
                case "btn3" -> btn3;
                case "btnSomar" -> btnSomar;
                case "btnSinal" -> btnSinal;
                case "btn0" -> btn0;
                case "btnVirgula" -> btnVirgula;
                case "btnIgual" -> btnIgual;
                default -> null;
            };
        }

        /** O Button que disparou o evento. */
        public Button senderAsButton(Aylis.Event ev) {
            return findButton(String.valueOf(ev.sender().get("name")));
        }

        /** O GridLayout deste form com o nome (como o FindComponent). */
        public GridLayout findGridLayout(String name) {
            return switch (name) {
                case "grdMemoria" -> grdMemoria;
                case "grdTeclado" -> grdTeclado;
                default -> null;
            };
        }

        /** O GridLayout que disparou o evento. */
        public GridLayout senderAsGridLayout(Aylis.Event ev) {
            return findGridLayout(String.valueOf(ev.sender().get("name")));
        }

        /** O Label deste form com o nome (como o FindComponent). */
        public Label findLabel(String name) {
            return switch (name) {
                case "lblTitulo" -> lblTitulo;
                case "lblExpressao" -> lblExpressao;
                case "lblVisor" -> lblVisor;
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
                case "lytCorpo" -> lytCorpo;
                case "lytTopo" -> lytTopo;
                default -> null;
            };
        }

        /** O Layout que disparou o evento. */
        public Layout senderAsLayout(Aylis.Event ev) {
            return findLayout(String.valueOf(ev.sender().get("name")));
        }

        /** O ListView deste form com o nome (como o FindComponent). */
        public ListView findListView(String name) {
            return switch (name) {
                case "lvHistorico" -> lvHistorico;
                case "lvMemoria" -> lvMemoria;
                default -> null;
            };
        }

        /** O ListView que disparou o evento. */
        public ListView senderAsListView(Aylis.Event ev) {
            return findListView(String.valueOf(ev.sender().get("name")));
        }

        /** O Panel deste form com o nome (como o FindComponent). */
        public Panel findPanel(String name) {
            return switch (name) {
                case "pnlLateral" -> pnlLateral;
                default -> null;
            };
        }

        /** O Panel que disparou o evento. */
        public Panel senderAsPanel(Aylis.Event ev) {
            return findPanel(String.valueOf(ev.sender().get("name")));
        }

        /** O TabControl deste form com o nome (como o FindComponent). */
        public TabControl findTabControl(String name) {
            return switch (name) {
                case "tabLateral" -> tabLateral;
                default -> null;
            };
        }

        /** O TabControl que disparou o evento. */
        public TabControl senderAsTabControl(Aylis.Event ev) {
            return findTabControl(String.valueOf(ev.sender().get("name")));
        }

        /** O TabItem deste form com o nome (como o FindComponent). */
        public TabItem findTabItem(String name) {
            return switch (name) {
                case "tabHistorico" -> tabHistorico;
                case "tabMemoria" -> tabMemoria;
                default -> null;
            };
        }

        /** O TabItem que disparou o evento. */
        public TabItem senderAsTabItem(Aylis.Event ev) {
            return findTabItem(String.valueOf(ev.sender().get("name")));
        }
    }
}
