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

    public static FrmPlayerComponents frmPlayer;

    /** Registra as classes dos forms (o registro dos forms chama). */
    public static void registerClasses() {
        Aylis.registerFormClass("frmPlayer", FrmPlayerComponents::new, form -> frmPlayer = (FrmPlayerComponents) form);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Divider de um form. */
    public static class Divider extends Aylis.AylisComponent {
        public Divider(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public boolean isVertical() {
            return asBool(getProp("vertical"));
        }

        public void setVertical(boolean value) {
            setProp("vertical", value);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
        }

        public double getThickness() {
            return asNumber(getProp("thickness"));
        }

        public void setThickness(double value) {
            setProp("thickness", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }

        public void moveToMonitor(long a1) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "move_to_monitor", "args", java.util.List.of(a1)));
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

    /** Um Icon de um form. */
    public static class Icon extends Aylis.AylisComponent {
        public Icon(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getIcon() {
            return asText(getProp("icon"));
        }

        public void setIcon(String value) {
            setProp("icon", value);
        }

        public String getColor() {
            return asText(getProp("color"));
        }

        public void setColor(String value) {
            setProp("color", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um MainMenu de um form. */
    public static class MainMenu extends Aylis.AylisComponent {
        public MainMenu(Aylis aylis, long id, String name) {
            super(aylis, id, name);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um MenuItem de um form. */
    public static class MenuItem extends Aylis.AylisComponent {
        public MenuItem(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
        }

        public String getShortcut() {
            return asText(getProp("shortcut"));
        }

        public void setShortcut(String value) {
            setProp("shortcut", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public boolean isChecked() {
            return asBool(getProp("checked"));
        }

        public void setChecked(boolean value) {
            setProp("checked", value);
        }

        public boolean isAutoCheck() {
            return asBool(getProp("auto_check"));
        }

        public void setAutoCheck(boolean value) {
            setProp("auto_check", value);
        }

        public String getIcon() {
            return asText(getProp("icon"));
        }

        public void setIcon(String value) {
            setProp("icon", value);
        }

        public boolean isVisible() {
            return asBool(getProp("visible"));
        }

        public void setVisible(boolean value) {
            setProp("visible", value);
        }

        public boolean isDanger() {
            return asBool(getProp("danger"));
        }

        public void setDanger(boolean value) {
            setProp("danger", value);
        }
    }

    /** Um OpenDialog de um form. */
    public static class OpenDialog extends Aylis.AylisComponent {
        public OpenDialog(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getTitle() {
            return asText(getProp("title"));
        }

        public void setTitle(String value) {
            setProp("title", value);
        }

        public String getFilter() {
            return asText(getProp("filter"));
        }

        public void setFilter(String value) {
            setProp("filter", value);
        }

        public String getInitialDir() {
            return asText(getProp("initial_dir"));
        }

        public void setInitialDir(String value) {
            setProp("initial_dir", value);
        }

        public String getFileName() {
            return asText(getProp("file_name"));
        }

        public void setFileName(String value) {
            setProp("file_name", value);
        }

        public boolean isMultiple() {
            return asBool(getProp("multiple"));
        }

        public void setMultiple(boolean value) {
            setProp("multiple", value);
        }

        public void execute() {
            aylis.send("execute", Aylis.obj("target", id));
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um SeekBar de um form. */
    public static class SeekBar extends Aylis.AylisComponent {
        public SeekBar(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public double getPosition() {
            return asNumber(getProp("position"));
        }

        public void setPosition(double value) {
            setProp("position", value);
        }

        public double getDuration() {
            return asNumber(getProp("duration"));
        }

        public void setDuration(double value) {
            setProp("duration", value);
        }

        public double getBuffered() {
            return asNumber(getProp("buffered"));
        }

        public void setBuffered(double value) {
            setProp("buffered", value);
        }

        public String getMarkers() {
            return asText(getProp("markers"));
        }

        public void setMarkers(String value) {
            setProp("markers", value);
        }

        public boolean isShowTime() {
            return asBool(getProp("show_time"));
        }

        public void setShowTime(boolean value) {
            setProp("show_time", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um Timer de um form. */
    public static class Timer extends Aylis.AylisComponent {
        public Timer(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public double getInterval() {
            return asNumber(getProp("interval"));
        }

        public void setInterval(double value) {
            setProp("interval", value);
        }

        public boolean isEnabled() {
            return asBool(getProp("enabled"));
        }

        public void setEnabled(boolean value) {
            setProp("enabled", value);
        }

        public String getPrecision() {
            return asText(getProp("precision"));
        }

        public void setPrecision(String value) {
            setProp("precision", value);
        }

        public boolean isBackground() {
            return asBool(getProp("background"));
        }

        public void setBackground(boolean value) {
            setProp("background", value);
        }

        public double getDrift() {
            return asNumber(getProp("drift"));
        }

        public void setDrift(double value) {
            setProp("drift", value);
        }
    }

    /** Um TrackBar de um form. */
    public static class TrackBar extends Aylis.AylisComponent {
        public TrackBar(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public double getValue() {
            return asNumber(getProp("value"));
        }

        public void setValue(double value) {
            setProp("value", value);
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

        public double getFrequency() {
            return asNumber(getProp("frequency"));
        }

        public void setFrequency(double value) {
            setProp("frequency", value);
        }

        public boolean isVertical() {
            return asBool(getProp("vertical"));
        }

        public void setVertical(boolean value) {
            setProp("vertical", value);
        }

        public double getThumbSize() {
            return asNumber(getProp("thumb_size"));
        }

        public void setThumbSize(double value) {
            setProp("thumb_size", value);
        }

        public double getTrackSize() {
            return asNumber(getProp("track_size"));
        }

        public void setTrackSize(double value) {
            setProp("track_size", value);
        }

        public boolean isSolidThumb() {
            return asBool(getProp("solid_thumb"));
        }

        public void setSolidThumb(boolean value) {
            setProp("solid_thumb", value);
        }

        public double getLimit() {
            return asNumber(getProp("limit"));
        }

        public void setLimit(double value) {
            setProp("limit", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }
    }

    /** Um VideoPlayer de um form. */
    public static class VideoPlayer extends Aylis.AylisComponent {
        public VideoPlayer(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getFile() {
            return asText(getProp("file"));
        }

        public void setFile(String value) {
            setProp("file", value);
        }

        public boolean isAutoplay() {
            return asBool(getProp("autoplay"));
        }

        public void setAutoplay(boolean value) {
            setProp("autoplay", value);
        }

        public boolean isLooping() {
            return asBool(getProp("looping"));
        }

        public void setLooping(boolean value) {
            setProp("looping", value);
        }

        public double getVolume() {
            return asNumber(getProp("volume"));
        }

        public void setVolume(double value) {
            setProp("volume", value);
        }

        public boolean isMuted() {
            return asBool(getProp("muted"));
        }

        public void setMuted(boolean value) {
            setProp("muted", value);
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

        public double getRate() {
            return asNumber(getProp("rate"));
        }

        public void setRate(double value) {
            setProp("rate", value);
        }

        public String getOutput() {
            return asText(getProp("output"));
        }

        public void setOutput(String value) {
            setProp("output", value);
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

        public String getPopupMenu() {
            return asText(getProp("popup_menu"));
        }

        public void setPopupMenu(String value) {
            setProp("popup_menu", value);
        }

        public void play() {
            aylis.send("media", Aylis.obj("target", id, "op", "play"));
        }

        public void pause() {
            aylis.send("media", Aylis.obj("target", id, "op", "pause"));
        }

        public void stop() {
            aylis.send("media", Aylis.obj("target", id, "op", "stop"));
        }

        public void seek(double seconds) {
            aylis.send("media", Aylis.obj("target", id, "op", "seek", "seconds", seconds));
        }

        public double position() {
            return asNumber(aylis.call("media", Aylis.obj("target", id, "op", "position")));
        }

        public double duration() {
            return asNumber(aylis.call("media", Aylis.obj("target", id, "op", "duration")));
        }
    }

    /** O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
    public static final class FrmPlayerComponents extends Form {
        public MainMenu mnuPrincipal;
        public MenuItem mnuArquivo;
        public MenuItem mnuAbrir;
        public MenuItem mnuLimparFila;
        public MenuItem mnuSep1;
        public MenuItem mnuSair;
        public MenuItem mnuReproduzir;
        public MenuItem mnuReproduzirPausar;
        public MenuItem mnuParar;
        public MenuItem mnuAnterior;
        public MenuItem mnuProxima;
        public MenuItem mnuSep2;
        public MenuItem mnuEmbaralhar;
        public MenuItem mnuRepetir;
        public MenuItem mnuExibir;
        public MenuItem mnuAgora;
        public MenuItem mnuFila;
        public Panel pnlControles;
        public Layout lytBarra;
        public Label lblTempo;
        public Label lblDuracao;
        public SeekBar sbPosicao;
        public Layout lytInfo;
        public Icon icoCapa;
        public Layout lytTextos;
        public Label lblTitulo;
        public Label lblDetalhe;
        public Layout lytVolume;
        public Button btnMudo;
        public TrackBar tbVolume;
        public Layout lytBotoes;
        public Layout lytBotoesCentro;
        public Button btnEmbaralhar;
        public Button btnAnterior;
        public Button btnReproduzir;
        public Button btnProxima;
        public Button btnRepetir;
        public Layout lytNavegacao;
        public Button btnAgora;
        public Button btnFila;
        public Divider divNavegacao;
        public Button btnAbrirArquivos;
        public Panel pnlAgora;
        public VideoPlayer vpVideo;
        public Panel pnlFila;
        public Label lblFila;
        public Layout lytFilaBotoes;
        public Button btnAdicionar;
        public Button btnLimparFila;
        public ListView lvFila;
        public OpenDialog dlgAbrir;
        public Timer tmrPosicao;

        public FrmPlayerComponents(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        @Override
        public void bind(java.util.List<java.util.Map<String, Object>> list) {
            mnuPrincipal = resolve(list, "mnuPrincipal", "MainMenu", MainMenu::new);
            mnuArquivo = resolve(list, "mnuArquivo", "MenuItem", MenuItem::new);
            mnuAbrir = resolve(list, "mnuAbrir", "MenuItem", MenuItem::new);
            mnuLimparFila = resolve(list, "mnuLimparFila", "MenuItem", MenuItem::new);
            mnuSep1 = resolve(list, "mnuSep1", "MenuItem", MenuItem::new);
            mnuSair = resolve(list, "mnuSair", "MenuItem", MenuItem::new);
            mnuReproduzir = resolve(list, "mnuReproduzir", "MenuItem", MenuItem::new);
            mnuReproduzirPausar = resolve(list, "mnuReproduzirPausar", "MenuItem", MenuItem::new);
            mnuParar = resolve(list, "mnuParar", "MenuItem", MenuItem::new);
            mnuAnterior = resolve(list, "mnuAnterior", "MenuItem", MenuItem::new);
            mnuProxima = resolve(list, "mnuProxima", "MenuItem", MenuItem::new);
            mnuSep2 = resolve(list, "mnuSep2", "MenuItem", MenuItem::new);
            mnuEmbaralhar = resolve(list, "mnuEmbaralhar", "MenuItem", MenuItem::new);
            mnuRepetir = resolve(list, "mnuRepetir", "MenuItem", MenuItem::new);
            mnuExibir = resolve(list, "mnuExibir", "MenuItem", MenuItem::new);
            mnuAgora = resolve(list, "mnuAgora", "MenuItem", MenuItem::new);
            mnuFila = resolve(list, "mnuFila", "MenuItem", MenuItem::new);
            pnlControles = resolve(list, "pnlControles", "Panel", Panel::new);
            lytBarra = resolve(list, "lytBarra", "Layout", Layout::new);
            lblTempo = resolve(list, "lblTempo", "Label", Label::new);
            lblDuracao = resolve(list, "lblDuracao", "Label", Label::new);
            sbPosicao = resolve(list, "sbPosicao", "SeekBar", SeekBar::new);
            lytInfo = resolve(list, "lytInfo", "Layout", Layout::new);
            icoCapa = resolve(list, "icoCapa", "Icon", Icon::new);
            lytTextos = resolve(list, "lytTextos", "Layout", Layout::new);
            lblTitulo = resolve(list, "lblTitulo", "Label", Label::new);
            lblDetalhe = resolve(list, "lblDetalhe", "Label", Label::new);
            lytVolume = resolve(list, "lytVolume", "Layout", Layout::new);
            btnMudo = resolve(list, "btnMudo", "Button", Button::new);
            tbVolume = resolve(list, "tbVolume", "TrackBar", TrackBar::new);
            lytBotoes = resolve(list, "lytBotoes", "Layout", Layout::new);
            lytBotoesCentro = resolve(list, "lytBotoesCentro", "Layout", Layout::new);
            btnEmbaralhar = resolve(list, "btnEmbaralhar", "Button", Button::new);
            btnAnterior = resolve(list, "btnAnterior", "Button", Button::new);
            btnReproduzir = resolve(list, "btnReproduzir", "Button", Button::new);
            btnProxima = resolve(list, "btnProxima", "Button", Button::new);
            btnRepetir = resolve(list, "btnRepetir", "Button", Button::new);
            lytNavegacao = resolve(list, "lytNavegacao", "Layout", Layout::new);
            btnAgora = resolve(list, "btnAgora", "Button", Button::new);
            btnFila = resolve(list, "btnFila", "Button", Button::new);
            divNavegacao = resolve(list, "divNavegacao", "Divider", Divider::new);
            btnAbrirArquivos = resolve(list, "btnAbrirArquivos", "Button", Button::new);
            pnlAgora = resolve(list, "pnlAgora", "Panel", Panel::new);
            vpVideo = resolve(list, "vpVideo", "VideoPlayer", VideoPlayer::new);
            pnlFila = resolve(list, "pnlFila", "Panel", Panel::new);
            lblFila = resolve(list, "lblFila", "Label", Label::new);
            lytFilaBotoes = resolve(list, "lytFilaBotoes", "Layout", Layout::new);
            btnAdicionar = resolve(list, "btnAdicionar", "Button", Button::new);
            btnLimparFila = resolve(list, "btnLimparFila", "Button", Button::new);
            lvFila = resolve(list, "lvFila", "ListView", ListView::new);
            dlgAbrir = resolve(list, "dlgAbrir", "OpenDialog", OpenDialog::new);
            tmrPosicao = resolve(list, "tmrPosicao", "Timer", Timer::new);
        }

        /** O Button deste form com o nome (como o FindComponent). */
        public Button findButton(String name) {
            return switch (name) {
                case "btnMudo" -> btnMudo;
                case "btnEmbaralhar" -> btnEmbaralhar;
                case "btnAnterior" -> btnAnterior;
                case "btnReproduzir" -> btnReproduzir;
                case "btnProxima" -> btnProxima;
                case "btnRepetir" -> btnRepetir;
                case "btnAgora" -> btnAgora;
                case "btnFila" -> btnFila;
                case "btnAbrirArquivos" -> btnAbrirArquivos;
                case "btnAdicionar" -> btnAdicionar;
                case "btnLimparFila" -> btnLimparFila;
                default -> null;
            };
        }

        /** O Button que disparou o evento. */
        public Button senderAsButton(Aylis.Event ev) {
            return findButton(String.valueOf(ev.sender().get("name")));
        }

        /** O Divider deste form com o nome (como o FindComponent). */
        public Divider findDivider(String name) {
            return switch (name) {
                case "divNavegacao" -> divNavegacao;
                default -> null;
            };
        }

        /** O Divider que disparou o evento. */
        public Divider senderAsDivider(Aylis.Event ev) {
            return findDivider(String.valueOf(ev.sender().get("name")));
        }

        /** O Icon deste form com o nome (como o FindComponent). */
        public Icon findIcon(String name) {
            return switch (name) {
                case "icoCapa" -> icoCapa;
                default -> null;
            };
        }

        /** O Icon que disparou o evento. */
        public Icon senderAsIcon(Aylis.Event ev) {
            return findIcon(String.valueOf(ev.sender().get("name")));
        }

        /** O Label deste form com o nome (como o FindComponent). */
        public Label findLabel(String name) {
            return switch (name) {
                case "lblTempo" -> lblTempo;
                case "lblDuracao" -> lblDuracao;
                case "lblTitulo" -> lblTitulo;
                case "lblDetalhe" -> lblDetalhe;
                case "lblFila" -> lblFila;
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
                case "lytBarra" -> lytBarra;
                case "lytInfo" -> lytInfo;
                case "lytTextos" -> lytTextos;
                case "lytVolume" -> lytVolume;
                case "lytBotoes" -> lytBotoes;
                case "lytBotoesCentro" -> lytBotoesCentro;
                case "lytNavegacao" -> lytNavegacao;
                case "lytFilaBotoes" -> lytFilaBotoes;
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
                case "lvFila" -> lvFila;
                default -> null;
            };
        }

        /** O ListView que disparou o evento. */
        public ListView senderAsListView(Aylis.Event ev) {
            return findListView(String.valueOf(ev.sender().get("name")));
        }

        /** O MainMenu deste form com o nome (como o FindComponent). */
        public MainMenu findMainMenu(String name) {
            return switch (name) {
                case "mnuPrincipal" -> mnuPrincipal;
                default -> null;
            };
        }

        /** O MainMenu que disparou o evento. */
        public MainMenu senderAsMainMenu(Aylis.Event ev) {
            return findMainMenu(String.valueOf(ev.sender().get("name")));
        }

        /** O MenuItem deste form com o nome (como o FindComponent). */
        public MenuItem findMenuItem(String name) {
            return switch (name) {
                case "mnuArquivo" -> mnuArquivo;
                case "mnuAbrir" -> mnuAbrir;
                case "mnuLimparFila" -> mnuLimparFila;
                case "mnuSep1" -> mnuSep1;
                case "mnuSair" -> mnuSair;
                case "mnuReproduzir" -> mnuReproduzir;
                case "mnuReproduzirPausar" -> mnuReproduzirPausar;
                case "mnuParar" -> mnuParar;
                case "mnuAnterior" -> mnuAnterior;
                case "mnuProxima" -> mnuProxima;
                case "mnuSep2" -> mnuSep2;
                case "mnuEmbaralhar" -> mnuEmbaralhar;
                case "mnuRepetir" -> mnuRepetir;
                case "mnuExibir" -> mnuExibir;
                case "mnuAgora" -> mnuAgora;
                case "mnuFila" -> mnuFila;
                default -> null;
            };
        }

        /** O MenuItem que disparou o evento. */
        public MenuItem senderAsMenuItem(Aylis.Event ev) {
            return findMenuItem(String.valueOf(ev.sender().get("name")));
        }

        /** O OpenDialog deste form com o nome (como o FindComponent). */
        public OpenDialog findOpenDialog(String name) {
            return switch (name) {
                case "dlgAbrir" -> dlgAbrir;
                default -> null;
            };
        }

        /** O OpenDialog que disparou o evento. */
        public OpenDialog senderAsOpenDialog(Aylis.Event ev) {
            return findOpenDialog(String.valueOf(ev.sender().get("name")));
        }

        /** O Panel deste form com o nome (como o FindComponent). */
        public Panel findPanel(String name) {
            return switch (name) {
                case "pnlControles" -> pnlControles;
                case "pnlAgora" -> pnlAgora;
                case "pnlFila" -> pnlFila;
                default -> null;
            };
        }

        /** O Panel que disparou o evento. */
        public Panel senderAsPanel(Aylis.Event ev) {
            return findPanel(String.valueOf(ev.sender().get("name")));
        }

        /** O SeekBar deste form com o nome (como o FindComponent). */
        public SeekBar findSeekBar(String name) {
            return switch (name) {
                case "sbPosicao" -> sbPosicao;
                default -> null;
            };
        }

        /** O SeekBar que disparou o evento. */
        public SeekBar senderAsSeekBar(Aylis.Event ev) {
            return findSeekBar(String.valueOf(ev.sender().get("name")));
        }

        /** O Timer deste form com o nome (como o FindComponent). */
        public Timer findTimer(String name) {
            return switch (name) {
                case "tmrPosicao" -> tmrPosicao;
                default -> null;
            };
        }

        /** O Timer que disparou o evento. */
        public Timer senderAsTimer(Aylis.Event ev) {
            return findTimer(String.valueOf(ev.sender().get("name")));
        }

        /** O TrackBar deste form com o nome (como o FindComponent). */
        public TrackBar findTrackBar(String name) {
            return switch (name) {
                case "tbVolume" -> tbVolume;
                default -> null;
            };
        }

        /** O TrackBar que disparou o evento. */
        public TrackBar senderAsTrackBar(Aylis.Event ev) {
            return findTrackBar(String.valueOf(ev.sender().get("name")));
        }

        /** O VideoPlayer deste form com o nome (como o FindComponent). */
        public VideoPlayer findVideoPlayer(String name) {
            return switch (name) {
                case "vpVideo" -> vpVideo;
                default -> null;
            };
        }

        /** O VideoPlayer que disparou o evento. */
        public VideoPlayer senderAsVideoPlayer(Aylis.Event ev) {
            return findVideoPlayer(String.valueOf(ev.sender().get("name")));
        }
    }
}
