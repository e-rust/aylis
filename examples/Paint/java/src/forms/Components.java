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

    public static FrmPaintComponents frmPaint;

    /** Registra as classes dos forms (o registro dos forms chama). */
    public static void registerClasses() {
        Aylis.registerFormClass("frmPaint", FrmPaintComponents::new, form -> frmPaint = (FrmPaintComponents) form);
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

    /** Um CheckBox de um form. */
    public static class CheckBox extends Aylis.AylisComponent {
        public CheckBox(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
        }

        public boolean isChecked() {
            return asBool(getProp("checked"));
        }

        public void setChecked(boolean value) {
            setProp("checked", value);
        }

        public double getBoxSize() {
            return asNumber(getProp("box_size"));
        }

        public void setBoxSize(double value) {
            setProp("box_size", value);
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

    /** Um Circle de um form. */
    public static class Circle extends Aylis.AylisComponent {
        public Circle(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getFill() {
            return asText(getProp("fill"));
        }

        public void setFill(String value) {
            setProp("fill", value);
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

    /** Um ComboBox de um form. */
    public static class ComboBox extends Aylis.AylisComponent {
        public ComboBox(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getItems() {
            return asText(getProp("items"));
        }

        public void setItems(String value) {
            setProp("items", value);
        }

        public String getValues() {
            return asText(getProp("values"));
        }

        public void setValues(String value) {
            setProp("values", value);
        }

        public long getItemIndex() {
            return (long) asNumber(getProp("item_index"));
        }

        public void setItemIndex(long value) {
            setProp("item_index", value);
        }

        public String getHAlign() {
            return asText(getProp("h_align"));
        }

        public void setHAlign(String value) {
            setProp("h_align", value);
        }

        public String getTrimming() {
            return asText(getProp("trimming"));
        }

        public void setTrimming(String value) {
            setProp("trimming", value);
        }

        public String getArrow() {
            return asText(getProp("arrow"));
        }

        public void setArrow(String value) {
            setProp("arrow", value);
        }

        public boolean isSensitive() {
            return asBool(getProp("sensitive"));
        }

        public void setSensitive(boolean value) {
            setProp("sensitive", value);
        }

        public String getValue() {
            return asText(getProp("value"));
        }

        public void setValue(String value) {
            setProp("value", value);
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

    /** Um PaintCanvas de um form. */
    public static class PaintCanvas extends Aylis.AylisComponent {
        public PaintCanvas(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getTool() {
            return asText(getProp("tool"));
        }

        public void setTool(String value) {
            setProp("tool", value);
        }

        public String getColor() {
            return asText(getProp("color"));
        }

        public void setColor(String value) {
            setProp("color", value);
        }

        public String getBackColor() {
            return asText(getProp("back_color"));
        }

        public void setBackColor(String value) {
            setProp("back_color", value);
        }

        public long getBrushSize() {
            return (long) asNumber(getProp("brush_size"));
        }

        public void setBrushSize(long value) {
            setProp("brush_size", value);
        }

        public boolean isFilled() {
            return asBool(getProp("filled"));
        }

        public void setFilled(boolean value) {
            setProp("filled", value);
        }

        public double getZoom() {
            return asNumber(getProp("zoom"));
        }

        public void setZoom(double value) {
            setProp("zoom", value);
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

        public boolean undo() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "undo", "args", java.util.List.of())));
        }

        public boolean redo() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "redo", "args", java.util.List.of())));
        }

        public boolean canUndo() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "can_undo", "args", java.util.List.of())));
        }

        public boolean canRedo() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "can_redo", "args", java.util.List.of())));
        }

        public String size() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "size", "args", java.util.List.of())));
        }

        public String checksum() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "checksum", "args", java.util.List.of())));
        }

        public boolean clear() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "clear", "args", java.util.List.of())));
        }

        public boolean resize(long width, long height) {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "resize", "args", java.util.List.of(width, height))));
        }

        public boolean new_(long width, long height) {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "new", "args", java.util.List.of(width, height))));
        }

        public boolean open(String path) {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "open", "args", java.util.List.of(path))));
        }

        public boolean save(String path) {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "save", "args", java.util.List.of(path))));
        }

        public boolean selectAll() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "select_all", "args", java.util.List.of())));
        }

        public boolean hasSelection() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "has_selection", "args", java.util.List.of())));
        }

        public boolean copy() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "copy", "args", java.util.List.of())));
        }

        public boolean cut() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "cut", "args", java.util.List.of())));
        }

        public boolean paste() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "paste", "args", java.util.List.of())));
        }

        public boolean delete() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "delete", "args", java.util.List.of())));
        }

        public boolean deselect() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "deselect", "args", java.util.List.of())));
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

    /** Um SaveDialog de um form. */
    public static class SaveDialog extends Aylis.AylisComponent {
        public SaveDialog(Aylis aylis, long id, String name) {
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

    /** Um ScrollBox de um form. */
    public static class ScrollBox extends Aylis.AylisComponent {
        public ScrollBox(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public boolean isHorizontal() {
            return asBool(getProp("horizontal"));
        }

        public void setHorizontal(boolean value) {
            setProp("horizontal", value);
        }

        public boolean isPullToRefresh() {
            return asBool(getProp("pull_to_refresh"));
        }

        public void setPullToRefresh(boolean value) {
            setProp("pull_to_refresh", value);
        }

        public boolean isRefreshing() {
            return asBool(getProp("refreshing"));
        }

        public void setRefreshing(boolean value) {
            setProp("refreshing", value);
        }

        public double getWheelStep() {
            return asNumber(getProp("wheel_step"));
        }

        public void setWheelStep(double value) {
            setProp("wheel_step", value);
        }

        public String getSmooth() {
            return asText(getProp("smooth"));
        }

        public void setSmooth(String value) {
            setProp("smooth", value);
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

        public double getScrollPosition() {
            return asNumber(getProp("scroll_position"));
        }

        public void setScrollPosition(double value) {
            setProp("scroll_position", value);
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

        public void scrollTo(Aylis.AylisComponent a1) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "scroll_to", "args", java.util.List.of(a1.id)));
        }

        public String visibleComponents(Aylis.AylisComponent a1, boolean a2) {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "visible_components", "args", java.util.List.of(a1.id, a2))));
        }
    }

    /** Um StatusBar de um form. */
    public static class StatusBar extends Aylis.AylisComponent {
        public StatusBar(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public String getText() {
            return asText(getProp("text"));
        }

        public void setText(String value) {
            setProp("text", value);
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

    /** O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
    public static final class FrmPaintComponents extends Form {
        public Layout lytTopo;
        public MainMenu mnuPrincipal;
        public MenuItem mnuArquivo;
        public MenuItem mnuNovo;
        public MenuItem mnuAbrir;
        public MenuItem mnuSalvar;
        public MenuItem mnuSalvarComo;
        public MenuItem mnuSep1;
        public MenuItem mnuSair;
        public MenuItem mnuEditar;
        public MenuItem mnuDesfazer;
        public MenuItem mnuRefazer;
        public MenuItem mnuSep2;
        public MenuItem mnuRecortar;
        public MenuItem mnuCopiar;
        public MenuItem mnuColar;
        public MenuItem mnuExcluir;
        public MenuItem mnuSelecionarTudo;
        public MenuItem mnuSep3;
        public MenuItem mnuLimpar;
        public MenuItem mnuExibir;
        public MenuItem mnuAmpliar;
        public MenuItem mnuReduzir;
        public MenuItem mnuTamanhoReal;
        public Divider divTopo;
        public Button btnSalvar;
        public Button btnDesfazer;
        public Button btnRefazer;
        public Panel pnlFaixa;
        public Layout lytSelecao;
        public Label lblSelecaoGrupo;
        public Button btnSelecionar;
        public Divider divSelecao;
        public Layout lytFerramentas;
        public Label lblFerramentasGrupo;
        public GridLayout grdFerramentas;
        public Button btnLapis;
        public Button btnBalde;
        public Button btnBorracha;
        public Button btnContaGotas;
        public Divider divFerramentas;
        public Layout lytPinceis;
        public Label lblPinceis;
        public Button btnPincel;
        public Divider divPinceis;
        public Layout lytFormas;
        public Label lblFormas;
        public CheckBox chkPreencher;
        public GridLayout grdFormas;
        public Button btnLinha;
        public Button btnRetangulo;
        public Button btnElipse;
        public Divider divFormas;
        public Layout lytTamanho;
        public Label lblTamanhoGrupo;
        public ComboBox cbEspessura;
        public Divider divTamanho;
        public Layout lytCores;
        public Label lblCores;
        public Circle btnCor1;
        public Circle btnCor2;
        public GridLayout grdPaleta;
        public Circle btnPaleta01;
        public Circle btnPaleta02;
        public Circle btnPaleta03;
        public Circle btnPaleta04;
        public Circle btnPaleta05;
        public Circle btnPaleta06;
        public Circle btnPaleta07;
        public Circle btnPaleta08;
        public Circle btnPaleta09;
        public Circle btnPaleta10;
        public Circle btnPaleta11;
        public Circle btnPaleta12;
        public Circle btnPaleta13;
        public Circle btnPaleta14;
        public Circle btnPaleta15;
        public Circle btnPaleta16;
        public Circle btnPaleta17;
        public Circle btnPaleta18;
        public Circle btnPaleta19;
        public Circle btnPaleta20;
        public StatusBar stbStatus;
        public Label lblPosicao;
        public Label lblTamanho;
        public Label lblZoom;
        public ScrollBox sbxArea;
        public PaintCanvas pcDesenho;
        public OpenDialog dlgAbrir;
        public SaveDialog dlgSalvar;

        public FrmPaintComponents(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        @Override
        public void bind(java.util.List<java.util.Map<String, Object>> list) {
            lytTopo = resolve(list, "lytTopo", "Layout", Layout::new);
            mnuPrincipal = resolve(list, "mnuPrincipal", "MainMenu", MainMenu::new);
            mnuArquivo = resolve(list, "mnuArquivo", "MenuItem", MenuItem::new);
            mnuNovo = resolve(list, "mnuNovo", "MenuItem", MenuItem::new);
            mnuAbrir = resolve(list, "mnuAbrir", "MenuItem", MenuItem::new);
            mnuSalvar = resolve(list, "mnuSalvar", "MenuItem", MenuItem::new);
            mnuSalvarComo = resolve(list, "mnuSalvarComo", "MenuItem", MenuItem::new);
            mnuSep1 = resolve(list, "mnuSep1", "MenuItem", MenuItem::new);
            mnuSair = resolve(list, "mnuSair", "MenuItem", MenuItem::new);
            mnuEditar = resolve(list, "mnuEditar", "MenuItem", MenuItem::new);
            mnuDesfazer = resolve(list, "mnuDesfazer", "MenuItem", MenuItem::new);
            mnuRefazer = resolve(list, "mnuRefazer", "MenuItem", MenuItem::new);
            mnuSep2 = resolve(list, "mnuSep2", "MenuItem", MenuItem::new);
            mnuRecortar = resolve(list, "mnuRecortar", "MenuItem", MenuItem::new);
            mnuCopiar = resolve(list, "mnuCopiar", "MenuItem", MenuItem::new);
            mnuColar = resolve(list, "mnuColar", "MenuItem", MenuItem::new);
            mnuExcluir = resolve(list, "mnuExcluir", "MenuItem", MenuItem::new);
            mnuSelecionarTudo = resolve(list, "mnuSelecionarTudo", "MenuItem", MenuItem::new);
            mnuSep3 = resolve(list, "mnuSep3", "MenuItem", MenuItem::new);
            mnuLimpar = resolve(list, "mnuLimpar", "MenuItem", MenuItem::new);
            mnuExibir = resolve(list, "mnuExibir", "MenuItem", MenuItem::new);
            mnuAmpliar = resolve(list, "mnuAmpliar", "MenuItem", MenuItem::new);
            mnuReduzir = resolve(list, "mnuReduzir", "MenuItem", MenuItem::new);
            mnuTamanhoReal = resolve(list, "mnuTamanhoReal", "MenuItem", MenuItem::new);
            divTopo = resolve(list, "divTopo", "Divider", Divider::new);
            btnSalvar = resolve(list, "btnSalvar", "Button", Button::new);
            btnDesfazer = resolve(list, "btnDesfazer", "Button", Button::new);
            btnRefazer = resolve(list, "btnRefazer", "Button", Button::new);
            pnlFaixa = resolve(list, "pnlFaixa", "Panel", Panel::new);
            lytSelecao = resolve(list, "lytSelecao", "Layout", Layout::new);
            lblSelecaoGrupo = resolve(list, "lblSelecaoGrupo", "Label", Label::new);
            btnSelecionar = resolve(list, "btnSelecionar", "Button", Button::new);
            divSelecao = resolve(list, "divSelecao", "Divider", Divider::new);
            lytFerramentas = resolve(list, "lytFerramentas", "Layout", Layout::new);
            lblFerramentasGrupo = resolve(list, "lblFerramentasGrupo", "Label", Label::new);
            grdFerramentas = resolve(list, "grdFerramentas", "GridLayout", GridLayout::new);
            btnLapis = resolve(list, "btnLapis", "Button", Button::new);
            btnBalde = resolve(list, "btnBalde", "Button", Button::new);
            btnBorracha = resolve(list, "btnBorracha", "Button", Button::new);
            btnContaGotas = resolve(list, "btnContaGotas", "Button", Button::new);
            divFerramentas = resolve(list, "divFerramentas", "Divider", Divider::new);
            lytPinceis = resolve(list, "lytPinceis", "Layout", Layout::new);
            lblPinceis = resolve(list, "lblPinceis", "Label", Label::new);
            btnPincel = resolve(list, "btnPincel", "Button", Button::new);
            divPinceis = resolve(list, "divPinceis", "Divider", Divider::new);
            lytFormas = resolve(list, "lytFormas", "Layout", Layout::new);
            lblFormas = resolve(list, "lblFormas", "Label", Label::new);
            chkPreencher = resolve(list, "chkPreencher", "CheckBox", CheckBox::new);
            grdFormas = resolve(list, "grdFormas", "GridLayout", GridLayout::new);
            btnLinha = resolve(list, "btnLinha", "Button", Button::new);
            btnRetangulo = resolve(list, "btnRetangulo", "Button", Button::new);
            btnElipse = resolve(list, "btnElipse", "Button", Button::new);
            divFormas = resolve(list, "divFormas", "Divider", Divider::new);
            lytTamanho = resolve(list, "lytTamanho", "Layout", Layout::new);
            lblTamanhoGrupo = resolve(list, "lblTamanhoGrupo", "Label", Label::new);
            cbEspessura = resolve(list, "cbEspessura", "ComboBox", ComboBox::new);
            divTamanho = resolve(list, "divTamanho", "Divider", Divider::new);
            lytCores = resolve(list, "lytCores", "Layout", Layout::new);
            lblCores = resolve(list, "lblCores", "Label", Label::new);
            btnCor1 = resolve(list, "btnCor1", "Circle", Circle::new);
            btnCor2 = resolve(list, "btnCor2", "Circle", Circle::new);
            grdPaleta = resolve(list, "grdPaleta", "GridLayout", GridLayout::new);
            btnPaleta01 = resolve(list, "btnPaleta01", "Circle", Circle::new);
            btnPaleta02 = resolve(list, "btnPaleta02", "Circle", Circle::new);
            btnPaleta03 = resolve(list, "btnPaleta03", "Circle", Circle::new);
            btnPaleta04 = resolve(list, "btnPaleta04", "Circle", Circle::new);
            btnPaleta05 = resolve(list, "btnPaleta05", "Circle", Circle::new);
            btnPaleta06 = resolve(list, "btnPaleta06", "Circle", Circle::new);
            btnPaleta07 = resolve(list, "btnPaleta07", "Circle", Circle::new);
            btnPaleta08 = resolve(list, "btnPaleta08", "Circle", Circle::new);
            btnPaleta09 = resolve(list, "btnPaleta09", "Circle", Circle::new);
            btnPaleta10 = resolve(list, "btnPaleta10", "Circle", Circle::new);
            btnPaleta11 = resolve(list, "btnPaleta11", "Circle", Circle::new);
            btnPaleta12 = resolve(list, "btnPaleta12", "Circle", Circle::new);
            btnPaleta13 = resolve(list, "btnPaleta13", "Circle", Circle::new);
            btnPaleta14 = resolve(list, "btnPaleta14", "Circle", Circle::new);
            btnPaleta15 = resolve(list, "btnPaleta15", "Circle", Circle::new);
            btnPaleta16 = resolve(list, "btnPaleta16", "Circle", Circle::new);
            btnPaleta17 = resolve(list, "btnPaleta17", "Circle", Circle::new);
            btnPaleta18 = resolve(list, "btnPaleta18", "Circle", Circle::new);
            btnPaleta19 = resolve(list, "btnPaleta19", "Circle", Circle::new);
            btnPaleta20 = resolve(list, "btnPaleta20", "Circle", Circle::new);
            stbStatus = resolve(list, "stbStatus", "StatusBar", StatusBar::new);
            lblPosicao = resolve(list, "lblPosicao", "Label", Label::new);
            lblTamanho = resolve(list, "lblTamanho", "Label", Label::new);
            lblZoom = resolve(list, "lblZoom", "Label", Label::new);
            sbxArea = resolve(list, "sbxArea", "ScrollBox", ScrollBox::new);
            pcDesenho = resolve(list, "pcDesenho", "PaintCanvas", PaintCanvas::new);
            dlgAbrir = resolve(list, "dlgAbrir", "OpenDialog", OpenDialog::new);
            dlgSalvar = resolve(list, "dlgSalvar", "SaveDialog", SaveDialog::new);
        }

        /** O Button deste form com o nome (como o FindComponent). */
        public Button findButton(String name) {
            return switch (name) {
                case "btnSalvar" -> btnSalvar;
                case "btnDesfazer" -> btnDesfazer;
                case "btnRefazer" -> btnRefazer;
                case "btnSelecionar" -> btnSelecionar;
                case "btnLapis" -> btnLapis;
                case "btnBalde" -> btnBalde;
                case "btnBorracha" -> btnBorracha;
                case "btnContaGotas" -> btnContaGotas;
                case "btnPincel" -> btnPincel;
                case "btnLinha" -> btnLinha;
                case "btnRetangulo" -> btnRetangulo;
                case "btnElipse" -> btnElipse;
                default -> null;
            };
        }

        /** O Button que disparou o evento. */
        public Button senderAsButton(Aylis.Event ev) {
            return findButton(String.valueOf(ev.sender().get("name")));
        }

        /** O CheckBox deste form com o nome (como o FindComponent). */
        public CheckBox findCheckBox(String name) {
            return switch (name) {
                case "chkPreencher" -> chkPreencher;
                default -> null;
            };
        }

        /** O CheckBox que disparou o evento. */
        public CheckBox senderAsCheckBox(Aylis.Event ev) {
            return findCheckBox(String.valueOf(ev.sender().get("name")));
        }

        /** O Circle deste form com o nome (como o FindComponent). */
        public Circle findCircle(String name) {
            return switch (name) {
                case "btnCor1" -> btnCor1;
                case "btnCor2" -> btnCor2;
                case "btnPaleta01" -> btnPaleta01;
                case "btnPaleta02" -> btnPaleta02;
                case "btnPaleta03" -> btnPaleta03;
                case "btnPaleta04" -> btnPaleta04;
                case "btnPaleta05" -> btnPaleta05;
                case "btnPaleta06" -> btnPaleta06;
                case "btnPaleta07" -> btnPaleta07;
                case "btnPaleta08" -> btnPaleta08;
                case "btnPaleta09" -> btnPaleta09;
                case "btnPaleta10" -> btnPaleta10;
                case "btnPaleta11" -> btnPaleta11;
                case "btnPaleta12" -> btnPaleta12;
                case "btnPaleta13" -> btnPaleta13;
                case "btnPaleta14" -> btnPaleta14;
                case "btnPaleta15" -> btnPaleta15;
                case "btnPaleta16" -> btnPaleta16;
                case "btnPaleta17" -> btnPaleta17;
                case "btnPaleta18" -> btnPaleta18;
                case "btnPaleta19" -> btnPaleta19;
                case "btnPaleta20" -> btnPaleta20;
                default -> null;
            };
        }

        /** O Circle que disparou o evento. */
        public Circle senderAsCircle(Aylis.Event ev) {
            return findCircle(String.valueOf(ev.sender().get("name")));
        }

        /** O ComboBox deste form com o nome (como o FindComponent). */
        public ComboBox findComboBox(String name) {
            return switch (name) {
                case "cbEspessura" -> cbEspessura;
                default -> null;
            };
        }

        /** O ComboBox que disparou o evento. */
        public ComboBox senderAsComboBox(Aylis.Event ev) {
            return findComboBox(String.valueOf(ev.sender().get("name")));
        }

        /** O Divider deste form com o nome (como o FindComponent). */
        public Divider findDivider(String name) {
            return switch (name) {
                case "divTopo" -> divTopo;
                case "divSelecao" -> divSelecao;
                case "divFerramentas" -> divFerramentas;
                case "divPinceis" -> divPinceis;
                case "divFormas" -> divFormas;
                case "divTamanho" -> divTamanho;
                default -> null;
            };
        }

        /** O Divider que disparou o evento. */
        public Divider senderAsDivider(Aylis.Event ev) {
            return findDivider(String.valueOf(ev.sender().get("name")));
        }

        /** O GridLayout deste form com o nome (como o FindComponent). */
        public GridLayout findGridLayout(String name) {
            return switch (name) {
                case "grdFerramentas" -> grdFerramentas;
                case "grdFormas" -> grdFormas;
                case "grdPaleta" -> grdPaleta;
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
                case "lblSelecaoGrupo" -> lblSelecaoGrupo;
                case "lblFerramentasGrupo" -> lblFerramentasGrupo;
                case "lblPinceis" -> lblPinceis;
                case "lblFormas" -> lblFormas;
                case "lblTamanhoGrupo" -> lblTamanhoGrupo;
                case "lblCores" -> lblCores;
                case "lblPosicao" -> lblPosicao;
                case "lblTamanho" -> lblTamanho;
                case "lblZoom" -> lblZoom;
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
                case "lytTopo" -> lytTopo;
                case "lytSelecao" -> lytSelecao;
                case "lytFerramentas" -> lytFerramentas;
                case "lytPinceis" -> lytPinceis;
                case "lytFormas" -> lytFormas;
                case "lytTamanho" -> lytTamanho;
                case "lytCores" -> lytCores;
                default -> null;
            };
        }

        /** O Layout que disparou o evento. */
        public Layout senderAsLayout(Aylis.Event ev) {
            return findLayout(String.valueOf(ev.sender().get("name")));
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
                case "mnuNovo" -> mnuNovo;
                case "mnuAbrir" -> mnuAbrir;
                case "mnuSalvar" -> mnuSalvar;
                case "mnuSalvarComo" -> mnuSalvarComo;
                case "mnuSep1" -> mnuSep1;
                case "mnuSair" -> mnuSair;
                case "mnuEditar" -> mnuEditar;
                case "mnuDesfazer" -> mnuDesfazer;
                case "mnuRefazer" -> mnuRefazer;
                case "mnuSep2" -> mnuSep2;
                case "mnuRecortar" -> mnuRecortar;
                case "mnuCopiar" -> mnuCopiar;
                case "mnuColar" -> mnuColar;
                case "mnuExcluir" -> mnuExcluir;
                case "mnuSelecionarTudo" -> mnuSelecionarTudo;
                case "mnuSep3" -> mnuSep3;
                case "mnuLimpar" -> mnuLimpar;
                case "mnuExibir" -> mnuExibir;
                case "mnuAmpliar" -> mnuAmpliar;
                case "mnuReduzir" -> mnuReduzir;
                case "mnuTamanhoReal" -> mnuTamanhoReal;
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

        /** O PaintCanvas deste form com o nome (como o FindComponent). */
        public PaintCanvas findPaintCanvas(String name) {
            return switch (name) {
                case "pcDesenho" -> pcDesenho;
                default -> null;
            };
        }

        /** O PaintCanvas que disparou o evento. */
        public PaintCanvas senderAsPaintCanvas(Aylis.Event ev) {
            return findPaintCanvas(String.valueOf(ev.sender().get("name")));
        }

        /** O Panel deste form com o nome (como o FindComponent). */
        public Panel findPanel(String name) {
            return switch (name) {
                case "pnlFaixa" -> pnlFaixa;
                default -> null;
            };
        }

        /** O Panel que disparou o evento. */
        public Panel senderAsPanel(Aylis.Event ev) {
            return findPanel(String.valueOf(ev.sender().get("name")));
        }

        /** O SaveDialog deste form com o nome (como o FindComponent). */
        public SaveDialog findSaveDialog(String name) {
            return switch (name) {
                case "dlgSalvar" -> dlgSalvar;
                default -> null;
            };
        }

        /** O SaveDialog que disparou o evento. */
        public SaveDialog senderAsSaveDialog(Aylis.Event ev) {
            return findSaveDialog(String.valueOf(ev.sender().get("name")));
        }

        /** O ScrollBox deste form com o nome (como o FindComponent). */
        public ScrollBox findScrollBox(String name) {
            return switch (name) {
                case "sbxArea" -> sbxArea;
                default -> null;
            };
        }

        /** O ScrollBox que disparou o evento. */
        public ScrollBox senderAsScrollBox(Aylis.Event ev) {
            return findScrollBox(String.valueOf(ev.sender().get("name")));
        }

        /** O StatusBar deste form com o nome (como o FindComponent). */
        public StatusBar findStatusBar(String name) {
            return switch (name) {
                case "stbStatus" -> stbStatus;
                default -> null;
            };
        }

        /** O StatusBar que disparou o evento. */
        public StatusBar senderAsStatusBar(Aylis.Event ev) {
            return findStatusBar(String.valueOf(ev.sender().get("name")));
        }
    }
}
