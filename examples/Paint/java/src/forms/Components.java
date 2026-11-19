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

    static java.time.LocalDate asDate(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalDate.parse(s);
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static java.time.LocalTime asTime(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalTime.parse(s);
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static java.time.LocalDateTime asDateTime(Object v) {
        try {
            String s = asText(v);
            return s.isEmpty() ? null : java.time.LocalDateTime.parse(s.replace(' ', 'T'));
        } catch (java.time.format.DateTimeParseException e) {
            return null;
        }
    }

    static String dateText(java.time.LocalDate d) {
        return d == null ? "" : d.toString();
    }

    static String timeText(java.time.LocalTime t) {
        return t == null ? "" : t.withNano(0).toString();
    }

    static String dateTimeText(java.time.LocalDateTime d) {
        return d == null ? "" : d.withNano(0).toString();
    }

    static java.util.List<String> asList(Object v) {
        java.util.List<String> out = new java.util.ArrayList<>();
        for (String s : asText(v).split(";")) if (!s.isBlank()) out.add(s.trim());
        return out;
    }

    static java.util.List<Double> asNumbers(Object v) {
        java.util.List<Double> out = new java.util.ArrayList<>();
        for (String s : asList(v)) {
            try {
                out.add(Double.parseDouble(s));
            } catch (NumberFormatException e) {
                // o que não é número fica de fora
            }
        }
        return out;
    }

    static java.util.List<Aylis.Color> asColors(Object v) {
        java.util.List<Aylis.Color> out = new java.util.ArrayList<>();
        for (String s : asList(v)) if (s.startsWith("#")) out.add(Aylis.Color.hex(s));
        return out;
    }

    static String listText(java.util.Collection<?> v) {
        java.util.List<String> out = new java.util.ArrayList<>();
        for (Object x : v) out.add(x instanceof Aylis.Color c ? c.toJson() : String.valueOf(x));
        return String.join(";", out);
    }

    /** O intervalo de datas (o on_change do DateRangePicker); null: sem a data. */
    public record DateRange(java.time.LocalDate start, java.time.LocalDate end) {}

    static DateRange asDateRange(Object v) {
        String[] p = (asText(v) + ";").split(";", -1);
        return new DateRange(asDate(p[0].trim()), asDate(p[1].trim()));
    }

    /** Os valores de um enum do acesso tipado. */
    public enum Align {
        NONE("none"),
        TOP("top"),
        BOTTOM("bottom"),
        LEFT("left"),
        RIGHT("right"),
        MOST_TOP("most_top"),
        MOST_BOTTOM("most_bottom"),
        MOST_LEFT("most_left"),
        MOST_RIGHT("most_right"),
        CLIENT("client"),
        CONTENTS("contents"),
        CENTER("center"),
        VERT_CENTER("vert_center"),
        HORZ_CENTER("horz_center"),
        HORIZONTAL("horizontal"),
        VERTICAL("vertical"),
        FIT("fit");

        public final String value;

        Align(String value) {
            this.value = value;
        }

        public static Align of(Object v) {
            for (Align e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Align> listOf(Object v) {
            java.util.List<Align> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Align e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Align> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Align e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum Anchor {
        LEFT("left"),
        TOP("top"),
        RIGHT("right"),
        BOTTOM("bottom");

        public final String value;

        Anchor(String value) {
            this.value = value;
        }

        public static Anchor of(Object v) {
            for (Anchor e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Anchor> listOf(Object v) {
            java.util.List<Anchor> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Anchor e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Anchor> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Anchor e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum BlendMode {
        NORMAL("normal"),
        MULTIPLY("multiply"),
        SCREEN("screen"),
        OVERLAY("overlay"),
        DARKEN("darken"),
        LIGHTEN("lighten"),
        COLOR_DODGE("color_dodge"),
        COLOR_BURN("color_burn"),
        HARD_LIGHT("hard_light"),
        SOFT_LIGHT("soft_light"),
        DIFFERENCE("difference"),
        EXCLUSION("exclusion"),
        HUE("hue"),
        SATURATION("saturation"),
        COLOR("color"),
        LUMINOSITY("luminosity");

        public final String value;

        BlendMode(String value) {
            this.value = value;
        }

        public static BlendMode of(Object v) {
            for (BlendMode e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<BlendMode> listOf(Object v) {
            java.util.List<BlendMode> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (BlendMode e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<BlendMode> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (BlendMode e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum BorderStyle {
        SIZEABLE("sizeable"),
        SINGLE("single"),
        NONE("none"),
        CUSTOM("custom");

        public final String value;

        BorderStyle(String value) {
            this.value = value;
        }

        public static BorderStyle of(Object v) {
            for (BorderStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<BorderStyle> listOf(Object v) {
            java.util.List<BorderStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (BorderStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<BorderStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (BorderStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ButtonStyle {
        DEFAULT(""),
        ACCENT("accent"),
        DANGER("danger"),
        ELEVATED("elevated"),
        FAB("fab"),
        FAB_EXTENDED("fab_extended"),
        FLAT("flat"),
        GHOST("ghost"),
        GLASS("glass"),
        GLASS_PROMINENT("glass_prominent"),
        GLOSSY("glossy"),
        ICON("icon"),
        ICON_FILLED("icon_filled"),
        ICON_OUTLINED("icon_outlined"),
        ICON_TONAL("icon_tonal"),
        INFO("info"),
        LINK("link"),
        NEGATIVE("negative"),
        OUTLINE("outline"),
        PILL("pill"),
        POSITIVE("positive"),
        PUSH("push"),
        REVEAL("reveal"),
        ROUND("round"),
        ROUNDED("rounded"),
        SECONDARY("secondary"),
        SUCCESS("success"),
        TEXT("text"),
        TONAL("tonal"),
        UNELEVATED("unelevated"),
        WARNING("warning");

        public final String value;

        ButtonStyle(String value) {
            this.value = value;
        }

        public static ButtonStyle of(Object v) {
            for (ButtonStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ButtonStyle> listOf(Object v) {
            java.util.List<ButtonStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ButtonStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ButtonStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ButtonStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum CheckBoxStyle {
        DEFAULT(""),
        ROUND("round");

        public final String value;

        CheckBoxStyle(String value) {
            this.value = value;
        }

        public static CheckBoxStyle of(Object v) {
            for (CheckBoxStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CheckBoxStyle> listOf(Object v) {
            java.util.List<CheckBoxStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CheckBoxStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CheckBoxStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CheckBoxStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum CircleStyle {
        DEFAULT("");

        public final String value;

        CircleStyle(String value) {
            this.value = value;
        }

        public static CircleStyle of(Object v) {
            for (CircleStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CircleStyle> listOf(Object v) {
            java.util.List<CircleStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CircleStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CircleStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CircleStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum CloseAction {
        HIDE("hide"),
        FREE("free");

        public final String value;

        CloseAction(String value) {
            this.value = value;
        }

        public static CloseAction of(Object v) {
            for (CloseAction e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CloseAction> listOf(Object v) {
            java.util.List<CloseAction> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CloseAction e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CloseAction> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CloseAction e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ComboArrow {
        CHEVRON("chevron"),
        TRIANGLE("triangle");

        public final String value;

        ComboArrow(String value) {
            this.value = value;
        }

        public static ComboArrow of(Object v) {
            for (ComboArrow e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ComboArrow> listOf(Object v) {
            java.util.List<ComboArrow> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ComboArrow e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ComboArrow> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ComboArrow e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ComboBoxStyle {
        DEFAULT("");

        public final String value;

        ComboBoxStyle(String value) {
            this.value = value;
        }

        public static ComboBoxStyle of(Object v) {
            for (ComboBoxStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ComboBoxStyle> listOf(Object v) {
            java.util.List<ComboBoxStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ComboBoxStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ComboBoxStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ComboBoxStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum CursorShape {
        DEFAULT("default"),
        POINTER("pointer"),
        TEXT("text"),
        WAIT("wait"),
        CROSSHAIR("crosshair"),
        MOVE("move"),
        NOT_ALLOWED("not_allowed"),
        RESIZE_HORIZONTAL("resize_horizontal"),
        RESIZE_VERTICAL("resize_vertical"),
        RESIZE_NE_SW("resize_ne_sw"),
        RESIZE_NW_SE("resize_nw_se"),
        GRAB("grab"),
        GRABBING("grabbing"),
        NONE("none");

        public final String value;

        CursorShape(String value) {
            this.value = value;
        }

        public static CursorShape of(Object v) {
            for (CursorShape e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CursorShape> listOf(Object v) {
            java.util.List<CursorShape> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CursorShape e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CursorShape> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CursorShape e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum DividerStyle {
        DEFAULT("");

        public final String value;

        DividerStyle(String value) {
            this.value = value;
        }

        public static DividerStyle of(Object v) {
            for (DividerStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<DividerStyle> listOf(Object v) {
            java.util.List<DividerStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (DividerStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<DividerStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (DividerStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FontWeight {
        DEFAULT("default"),
        THIN("thin"),
        LIGHT("light"),
        NORMAL("normal"),
        MEDIUM("medium"),
        SEMIBOLD("semibold"),
        BOLD("bold"),
        BLACK("black");

        public final String value;

        FontWeight(String value) {
            this.value = value;
        }

        public static FontWeight of(Object v) {
            for (FontWeight e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FontWeight> listOf(Object v) {
            java.util.List<FontWeight> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FontWeight e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FontWeight> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FontWeight e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FormOrientation {
        PROJECT("project"),
        ANY("any"),
        PORTRAIT("portrait"),
        LANDSCAPE("landscape"),
        SENSOR_PORTRAIT("sensor_portrait"),
        SENSOR_LANDSCAPE("sensor_landscape");

        public final String value;

        FormOrientation(String value) {
            this.value = value;
        }

        public static FormOrientation of(Object v) {
            for (FormOrientation e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FormOrientation> listOf(Object v) {
            java.util.List<FormOrientation> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FormOrientation e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FormOrientation> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FormOrientation e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FormPosition {
        CENTER("center"),
        SCREEN_CENTER("screen_center"),
        DESIGNED("designed"),
        DEFAULT("default"),
        TOP_LEFT("top_left"),
        TOP_RIGHT("top_right"),
        BOTTOM_LEFT("bottom_left"),
        BOTTOM_RIGHT("bottom_right");

        public final String value;

        FormPosition(String value) {
            this.value = value;
        }

        public static FormPosition of(Object v) {
            for (FormPosition e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FormPosition> listOf(Object v) {
            java.util.List<FormPosition> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FormPosition e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FormPosition> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FormPosition e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FormPresent {
        PROJECT("project"),
        WINDOW("window"),
        COMPOSITION("composition"),
        COMPOSED("composed");

        public final String value;

        FormPresent(String value) {
            this.value = value;
        }

        public static FormPresent of(Object v) {
            for (FormPresent e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FormPresent> listOf(Object v) {
            java.util.List<FormPresent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FormPresent e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FormPresent> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FormPresent e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FormStyle {
        DEFAULT("");

        public final String value;

        FormStyle(String value) {
            this.value = value;
        }

        public static FormStyle of(Object v) {
            for (FormStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FormStyle> listOf(Object v) {
            java.util.List<FormStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FormStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FormStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FormStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum GridLayoutStyle {
        DEFAULT("");

        public final String value;

        GridLayoutStyle(String value) {
            this.value = value;
        }

        public static GridLayoutStyle of(Object v) {
            for (GridLayoutStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<GridLayoutStyle> listOf(Object v) {
            java.util.List<GridLayoutStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (GridLayoutStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<GridLayoutStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (GridLayoutStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum IconPosition {
        LEFT("left"),
        RIGHT("right"),
        TOP("top");

        public final String value;

        IconPosition(String value) {
            this.value = value;
        }

        public static IconPosition of(Object v) {
            for (IconPosition e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<IconPosition> listOf(Object v) {
            java.util.List<IconPosition> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (IconPosition e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<IconPosition> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (IconPosition e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum LabelStyle {
        DEFAULT(""),
        CAPTION("caption"),
        ERROR("error"),
        HEADING("heading"),
        KBD("kbd"),
        LINK("link"),
        MONO("mono"),
        MUTED("muted"),
        TITLE("title");

        public final String value;

        LabelStyle(String value) {
            this.value = value;
        }

        public static LabelStyle of(Object v) {
            for (LabelStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<LabelStyle> listOf(Object v) {
            java.util.List<LabelStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (LabelStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<LabelStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (LabelStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum LayerCaching {
        PROJECT("project"),
        ON("on"),
        OFF("off");

        public final String value;

        LayerCaching(String value) {
            this.value = value;
        }

        public static LayerCaching of(Object v) {
            for (LayerCaching e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<LayerCaching> listOf(Object v) {
            java.util.List<LayerCaching> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (LayerCaching e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<LayerCaching> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (LayerCaching e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum LayoutStyle {
        DEFAULT("");

        public final String value;

        LayoutStyle(String value) {
            this.value = value;
        }

        public static LayoutStyle of(Object v) {
            for (LayoutStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<LayoutStyle> listOf(Object v) {
            java.util.List<LayoutStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (LayoutStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<LayoutStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (LayoutStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum MainMenuStyle {
        DEFAULT("");

        public final String value;

        MainMenuStyle(String value) {
            this.value = value;
        }

        public static MainMenuStyle of(Object v) {
            for (MainMenuStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<MainMenuStyle> listOf(Object v) {
            java.util.List<MainMenuStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (MainMenuStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<MainMenuStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (MainMenuStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ModalResult {
        NONE("none"),
        OK("ok"),
        CANCEL("cancel"),
        ABORT("abort"),
        RETRY("retry"),
        IGNORE("ignore"),
        YES("yes"),
        NO("no"),
        CLOSE("close");

        public final String value;

        ModalResult(String value) {
            this.value = value;
        }

        public static ModalResult of(Object v) {
            for (ModalResult e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ModalResult> listOf(Object v) {
            java.util.List<ModalResult> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ModalResult e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ModalResult> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ModalResult e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum Overscroll {
        AUTO("auto"),
        GLOW("glow"),
        STRETCH("stretch"),
        BOUNCE("bounce"),
        NONE("none");

        public final String value;

        Overscroll(String value) {
            this.value = value;
        }

        public static Overscroll of(Object v) {
            for (Overscroll e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Overscroll> listOf(Object v) {
            java.util.List<Overscroll> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Overscroll e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Overscroll> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Overscroll e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum PaintCanvasStyle {
        DEFAULT("");

        public final String value;

        PaintCanvasStyle(String value) {
            this.value = value;
        }

        public static PaintCanvasStyle of(Object v) {
            for (PaintCanvasStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<PaintCanvasStyle> listOf(Object v) {
            java.util.List<PaintCanvasStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (PaintCanvasStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<PaintCanvasStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (PaintCanvasStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum PaintTool {
        PENCIL("pencil"),
        BRUSH("brush"),
        ERASER("eraser"),
        LINE("line"),
        RECTANGLE("rectangle"),
        ELLIPSE("ellipse"),
        FILL("fill"),
        PICKER("picker"),
        SELECT("select");

        public final String value;

        PaintTool(String value) {
            this.value = value;
        }

        public static PaintTool of(Object v) {
            for (PaintTool e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<PaintTool> listOf(Object v) {
            java.util.List<PaintTool> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (PaintTool e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<PaintTool> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (PaintTool e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum PanelStyle {
        DEFAULT(""),
        APPBAR("appbar"),
        CARD("card"),
        FILLED("filled"),
        FLAT("flat"),
        GLASS("glass"),
        OUTLINED("outlined"),
        OVERLAY("overlay"),
        REVEAL("reveal"),
        SIDEBAR("sidebar");

        public final String value;

        PanelStyle(String value) {
            this.value = value;
        }

        public static PanelStyle of(Object v) {
            for (PanelStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<PanelStyle> listOf(Object v) {
            java.util.List<PanelStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (PanelStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<PanelStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (PanelStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ScrollBoxStyle {
        DEFAULT(""),
        HORIZONTAL("horizontal"),
        REFRESH("refresh");

        public final String value;

        ScrollBoxStyle(String value) {
            this.value = value;
        }

        public static ScrollBoxStyle of(Object v) {
            for (ScrollBoxStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ScrollBoxStyle> listOf(Object v) {
            java.util.List<ScrollBoxStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ScrollBoxStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ScrollBoxStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ScrollBoxStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum SelfAlign {
        STRETCH("stretch"),
        START("start"),
        CENTER("center"),
        END("end");

        public final String value;

        SelfAlign(String value) {
            this.value = value;
        }

        public static SelfAlign of(Object v) {
            for (SelfAlign e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<SelfAlign> listOf(Object v) {
            java.util.List<SelfAlign> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (SelfAlign e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<SelfAlign> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (SelfAlign e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum SmoothScroll {
        DEFAULT("default"),
        ON("on"),
        OFF("off");

        public final String value;

        SmoothScroll(String value) {
            this.value = value;
        }

        public static SmoothScroll of(Object v) {
            for (SmoothScroll e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<SmoothScroll> listOf(Object v) {
            java.util.List<SmoothScroll> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (SmoothScroll e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<SmoothScroll> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (SmoothScroll e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum StatusBarStyle {
        DEFAULT("");

        public final String value;

        StatusBarStyle(String value) {
            this.value = value;
        }

        public static StatusBarStyle of(Object v) {
            for (StatusBarStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<StatusBarStyle> listOf(Object v) {
            java.util.List<StatusBarStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (StatusBarStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<StatusBarStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (StatusBarStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum TextAlign {
        LEADING("leading"),
        CENTER("center"),
        TRAILING("trailing"),
        JUSTIFY("justify");

        public final String value;

        TextAlign(String value) {
            this.value = value;
        }

        public static TextAlign of(Object v) {
            for (TextAlign e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TextAlign> listOf(Object v) {
            java.util.List<TextAlign> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TextAlign e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TextAlign> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TextAlign e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum TextCase {
        NONE("none"),
        UPPER("upper"),
        LOWER("lower");

        public final String value;

        TextCase(String value) {
            this.value = value;
        }

        public static TextCase of(Object v) {
            for (TextCase e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TextCase> listOf(Object v) {
            java.util.List<TextCase> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TextCase e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TextCase> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TextCase e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ThemeName {
        DEFAULT(""),
        AYLIS_LIGHT("Aylis Light"),
        AYLIS_DARK("Aylis Dark"),
        MATERIAL_3_LIGHT("Material 3 Light"),
        MATERIAL_3_DARK("Material 3 Dark"),
        MATERIAL_2_LIGHT("Material 2 Light"),
        MATERIAL_2_DARK("Material 2 Dark"),
        ANDROID_7_LIGHT("Android 7 Light"),
        ANDROID_7_DARK("Android 7 Dark"),
        I_OS_26_LIGHT("iOS 26 Light"),
        I_OS_26_DARK("iOS 26 Dark"),
        I_OS_13_LIGHT("iOS 13 Light"),
        I_OS_13_DARK("iOS 13 Dark"),
        I_OS_7_LIGHT("iOS 7 Light"),
        WIN_UI_3_LIGHT("WinUI 3 Light"),
        WIN_UI_3_DARK("WinUI 3 Dark"),
        WINDOWS_11_LIGHT("Windows 11 Light"),
        WINDOWS_11_DARK("Windows 11 Dark"),
        WINDOWS_10_LIGHT("Windows 10 Light"),
        WINDOWS_10_DARK("Windows 10 Dark"),
        WIN32("Win32"),
        WINDOWS_7("Windows 7"),
        WINDOWS_PHONE_DARK("Windows Phone Dark"),
        WINDOWS_PHONE_LIGHT("Windows Phone Light"),
        VUETIFY_3_LIGHT("Vuetify 3 Light"),
        VUETIFY_3_DARK("Vuetify 3 Dark"),
        BOOTSTRAP_5_LIGHT("Bootstrap 5 Light"),
        BOOTSTRAP_5_DARK("Bootstrap 5 Dark"),
        SHADCN_UI_LIGHT("shadcn/ui Light"),
        SHADCN_UI_DARK("shadcn/ui Dark"),
        ANT_DESIGN_5_LIGHT("Ant Design 5 Light"),
        ANT_DESIGN_5_DARK("Ant Design 5 Dark"),
        CHAKRA_UI_LIGHT("Chakra UI Light"),
        CHAKRA_UI_DARK("Chakra UI Dark"),
        QUASAR_2_LIGHT("Quasar 2 Light"),
        QUASAR_2_DARK("Quasar 2 Dark");

        public final String value;

        ThemeName(String value) {
            this.value = value;
        }

        public static ThemeName of(Object v) {
            for (ThemeName e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ThemeName> listOf(Object v) {
            java.util.List<ThemeName> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ThemeName e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ThemeName> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ThemeName e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum Transition {
        NONE("none"),
        FADE("fade"),
        SLIDE_LEFT("slide_left"),
        SLIDE_RIGHT("slide_right"),
        SLIDE_UP("slide_up"),
        SLIDE_DOWN("slide_down");

        public final String value;

        Transition(String value) {
            this.value = value;
        }

        public static Transition of(Object v) {
            for (Transition e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Transition> listOf(Object v) {
            java.util.List<Transition> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Transition e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Transition> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Transition e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum Trimming {
        NONE("none"),
        CHARACTER("character"),
        WORD("word");

        public final String value;

        Trimming(String value) {
            this.value = value;
        }

        public static Trimming of(Object v) {
            for (Trimming e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Trimming> listOf(Object v) {
            java.util.List<Trimming> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Trimming e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Trimming> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Trimming e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum WindowCorners {
        DEFAULT("default"),
        ROUND("round"),
        SMALL("small"),
        SQUARE("square");

        public final String value;

        WindowCorners(String value) {
            this.value = value;
        }

        public static WindowCorners of(Object v) {
            for (WindowCorners e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<WindowCorners> listOf(Object v) {
            java.util.List<WindowCorners> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (WindowCorners e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<WindowCorners> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (WindowCorners e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum WindowShape {
        RECTANGLE("rectangle"),
        ROUNDED("rounded"),
        ELLIPSE("ellipse"),
        PATH("path");

        public final String value;

        WindowShape(String value) {
            this.value = value;
        }

        public static WindowShape of(Object v) {
            for (WindowShape e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<WindowShape> listOf(Object v) {
            java.util.List<WindowShape> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (WindowShape e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<WindowShape> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (WindowShape e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum WindowState {
        NORMAL("normal"),
        MAXIMIZED("maximized"),
        MINIMIZED("minimized"),
        FULL_SCREEN("full_screen");

        public final String value;

        WindowState(String value) {
            this.value = value;
        }

        public static WindowState of(Object v) {
            for (WindowState e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<WindowState> listOf(Object v) {
            java.util.List<WindowState> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (WindowState e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<WindowState> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (WindowState e : values) out.add(e.value);
            return out;
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record GestureEvent(String phase, double x, double y, double scale, double deltaScale, double rotation, double deltaRotation, double panX, double panY) {
        static GestureEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new GestureEvent(asText(m.get("phase")), asNumber(m.get("x")), asNumber(m.get("y")), asNumber(m.get("scale")), asNumber(m.get("delta_scale")), asNumber(m.get("rotation")), asNumber(m.get("delta_rotation")), asNumber(m.get("pan_x")), asNumber(m.get("pan_y")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("phase", phase);
            m.put("x", x);
            m.put("y", y);
            m.put("scale", scale);
            m.put("delta_scale", deltaScale);
            m.put("rotation", rotation);
            m.put("delta_rotation", deltaRotation);
            m.put("pan_x", panX);
            m.put("pan_y", panY);
            return m;
        }

        static java.util.List<GestureEvent> listFromJson(Object v) {
            java.util.List<GestureEvent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static GestureEvent fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<GestureEvent> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(GestureEvent r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<GestureEvent> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code GestureEvent.of().with...(valor)}. */
        public static GestureEvent of() {
            return fromJson(null);
        }

        public GestureEvent withPhase(String v) {
            return new GestureEvent(v, x, y, scale, deltaScale, rotation, deltaRotation, panX, panY);
        }

        public GestureEvent withX(double v) {
            return new GestureEvent(phase, v, y, scale, deltaScale, rotation, deltaRotation, panX, panY);
        }

        public GestureEvent withY(double v) {
            return new GestureEvent(phase, x, v, scale, deltaScale, rotation, deltaRotation, panX, panY);
        }

        public GestureEvent withScale(double v) {
            return new GestureEvent(phase, x, y, v, deltaScale, rotation, deltaRotation, panX, panY);
        }

        public GestureEvent withDeltaScale(double v) {
            return new GestureEvent(phase, x, y, scale, v, rotation, deltaRotation, panX, panY);
        }

        public GestureEvent withRotation(double v) {
            return new GestureEvent(phase, x, y, scale, deltaScale, v, deltaRotation, panX, panY);
        }

        public GestureEvent withDeltaRotation(double v) {
            return new GestureEvent(phase, x, y, scale, deltaScale, rotation, v, panX, panY);
        }

        public GestureEvent withPanX(double v) {
            return new GestureEvent(phase, x, y, scale, deltaScale, rotation, deltaRotation, v, panY);
        }

        public GestureEvent withPanY(double v) {
            return new GestureEvent(phase, x, y, scale, deltaScale, rotation, deltaRotation, panX, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record ImageSize(long width, long height) {
        static ImageSize fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new ImageSize((long) asNumber(m.get("width")), (long) asNumber(m.get("height")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("width", width);
            m.put("height", height);
            return m;
        }

        static java.util.List<ImageSize> listFromJson(Object v) {
            java.util.List<ImageSize> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static ImageSize fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<ImageSize> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(ImageSize r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<ImageSize> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code ImageSize.of().with...(valor)}. */
        public static ImageSize of() {
            return fromJson(null);
        }

        public ImageSize withWidth(long v) {
            return new ImageSize(v, height);
        }

        public ImageSize withHeight(long v) {
            return new ImageSize(width, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record KeyEvent(String key, boolean ctrl, boolean shift, boolean alt) {
        static KeyEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new KeyEvent(asText(m.get("key")), asBool(m.get("ctrl")), asBool(m.get("shift")), asBool(m.get("alt")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("key", key);
            m.put("ctrl", ctrl);
            m.put("shift", shift);
            m.put("alt", alt);
            return m;
        }

        static java.util.List<KeyEvent> listFromJson(Object v) {
            java.util.List<KeyEvent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static KeyEvent fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<KeyEvent> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(KeyEvent r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<KeyEvent> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code KeyEvent.of().with...(valor)}. */
        public static KeyEvent of() {
            return fromJson(null);
        }

        public KeyEvent withKey(String v) {
            return new KeyEvent(v, ctrl, shift, alt);
        }

        public KeyEvent withCtrl(boolean v) {
            return new KeyEvent(key, v, shift, alt);
        }

        public KeyEvent withShift(boolean v) {
            return new KeyEvent(key, ctrl, v, alt);
        }

        public KeyEvent withAlt(boolean v) {
            return new KeyEvent(key, ctrl, shift, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record PointerEvent(double x, double y, String button, long clicks) {
        static PointerEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new PointerEvent(asNumber(m.get("x")), asNumber(m.get("y")), asText(m.get("button")), (long) asNumber(m.get("clicks")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("x", x);
            m.put("y", y);
            m.put("button", button);
            m.put("clicks", clicks);
            return m;
        }

        static java.util.List<PointerEvent> listFromJson(Object v) {
            java.util.List<PointerEvent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static PointerEvent fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<PointerEvent> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(PointerEvent r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<PointerEvent> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code PointerEvent.of().with...(valor)}. */
        public static PointerEvent of() {
            return fromJson(null);
        }

        public PointerEvent withX(double v) {
            return new PointerEvent(v, y, button, clicks);
        }

        public PointerEvent withY(double v) {
            return new PointerEvent(x, v, button, clicks);
        }

        public PointerEvent withButton(String v) {
            return new PointerEvent(x, y, v, clicks);
        }

        public PointerEvent withClicks(long v) {
            return new PointerEvent(x, y, button, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record SwipeEvent(String direction, double x, double y, double dx, double dy, double vx, double vy) {
        static SwipeEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new SwipeEvent(asText(m.get("direction")), asNumber(m.get("x")), asNumber(m.get("y")), asNumber(m.get("dx")), asNumber(m.get("dy")), asNumber(m.get("vx")), asNumber(m.get("vy")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("direction", direction);
            m.put("x", x);
            m.put("y", y);
            m.put("dx", dx);
            m.put("dy", dy);
            m.put("vx", vx);
            m.put("vy", vy);
            return m;
        }

        static java.util.List<SwipeEvent> listFromJson(Object v) {
            java.util.List<SwipeEvent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static SwipeEvent fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<SwipeEvent> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(SwipeEvent r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<SwipeEvent> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code SwipeEvent.of().with...(valor)}. */
        public static SwipeEvent of() {
            return fromJson(null);
        }

        public SwipeEvent withDirection(String v) {
            return new SwipeEvent(v, x, y, dx, dy, vx, vy);
        }

        public SwipeEvent withX(double v) {
            return new SwipeEvent(direction, v, y, dx, dy, vx, vy);
        }

        public SwipeEvent withY(double v) {
            return new SwipeEvent(direction, x, v, dx, dy, vx, vy);
        }

        public SwipeEvent withDx(double v) {
            return new SwipeEvent(direction, x, y, v, dy, vx, vy);
        }

        public SwipeEvent withDy(double v) {
            return new SwipeEvent(direction, x, y, dx, v, vx, vy);
        }

        public SwipeEvent withVx(double v) {
            return new SwipeEvent(direction, x, y, dx, dy, v, vy);
        }

        public SwipeEvent withVy(double v) {
            return new SwipeEvent(direction, x, y, dx, dy, vx, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record WheelEvent(double x, double y, double dx, double dy) {
        static WheelEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new WheelEvent(asNumber(m.get("x")), asNumber(m.get("y")), asNumber(m.get("dx")), asNumber(m.get("dy")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("x", x);
            m.put("y", y);
            m.put("dx", dx);
            m.put("dy", dy);
            return m;
        }

        static java.util.List<WheelEvent> listFromJson(Object v) {
            java.util.List<WheelEvent> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static WheelEvent fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<WheelEvent> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(WheelEvent r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<WheelEvent> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code WheelEvent.of().with...(valor)}. */
        public static WheelEvent of() {
            return fromJson(null);
        }

        public WheelEvent withX(double v) {
            return new WheelEvent(v, y, dx, dy);
        }

        public WheelEvent withY(double v) {
            return new WheelEvent(x, v, dx, dy);
        }

        public WheelEvent withDx(double v) {
            return new WheelEvent(x, y, v, dy);
        }

        public WheelEvent withDy(double v) {
            return new WheelEvent(x, y, dx, v);
        }
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

        public IconPosition getIconPosition() {
            return IconPosition.of(getProp("icon_position"));
        }

        public void setIconPosition(IconPosition value) {
            setProp("icon_position", value.value);
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

        public ModalResult getModalResult() {
            return ModalResult.of(getProp("modal_result"));
        }

        public void setModalResult(ModalResult value) {
            setProp("modal_result", value.value);
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

        public TextAlign getHAlign() {
            return TextAlign.of(getProp("h_align"));
        }

        public void setHAlign(TextAlign value) {
            setProp("h_align", value.value);
        }

        public long getGroupIndex() {
            return (long) asNumber(getProp("group_index"));
        }

        public void setGroupIndex(long value) {
            setProp("group_index", value);
        }

        public Aylis.Bounds getRadius() {
            return Aylis.Bounds.fromJson(getProp("radius"));
        }

        public void setRadius(Aylis.Bounds value) {
            setProp("radius", value.toJson());
        }

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public ButtonStyle getStyle() {
            return ButtonStyle.of(getProp("style"));
        }

        public void setStyle(ButtonStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public CheckBoxStyle getStyle() {
            return CheckBoxStyle.of(getProp("style"));
        }

        public void setStyle(CheckBoxStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
        }
    }

    /** Um Circle de um form. */
    public static class Circle extends Aylis.AylisComponent {
        public Circle(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getStroke() {
            return Aylis.Color.fromJson(getProp("stroke"));
        }

        public void setStroke(Aylis.Color value) {
            setProp("stroke", value.toJson());
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public CircleStyle getStyle() {
            return CircleStyle.of(getProp("style"));
        }

        public void setStyle(CircleStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
        }
    }

    /** Um ComboBox de um form. */
    public static class ComboBox extends Aylis.AylisComponent {
        public ComboBox(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public java.util.List<String> getItems() {
            return asList(getProp("items"));
        }

        public void setItems(java.util.List<String> value) {
            setProp("items", listText(value));
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

        public TextAlign getHAlign() {
            return TextAlign.of(getProp("h_align"));
        }

        public void setHAlign(TextAlign value) {
            setProp("h_align", value.value);
        }

        public Trimming getTrimming() {
            return Trimming.of(getProp("trimming"));
        }

        public void setTrimming(Trimming value) {
            setProp("trimming", value.value);
        }

        public ComboArrow getArrow() {
            return ComboArrow.of(getProp("arrow"));
        }

        public void setArrow(ComboArrow value) {
            setProp("arrow", value.value);
        }

        public boolean isSensitive() {
            return asBool(getProp("sensitive"));
        }

        public void setSensitive(boolean value) {
            setProp("sensitive", value);
        }

        public java.util.List<String> getGroups() {
            return asList(getProp("groups"));
        }

        public void setGroups(java.util.List<String> value) {
            setProp("groups", listText(value));
        }

        public java.util.List<String> getDisabledItems() {
            return asList(getProp("disabled_items"));
        }

        public void setDisabledItems(java.util.List<String> value) {
            setProp("disabled_items", listText(value));
        }

        public long getDropDownCount() {
            return (long) asNumber(getProp("drop_down_count"));
        }

        public void setDropDownCount(long value) {
            setProp("drop_down_count", value);
        }

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public ComboBoxStyle getStyle() {
            return ComboBoxStyle.of(getProp("style"));
        }

        public void setStyle(ComboBoxStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public DividerStyle getStyle() {
            return DividerStyle.of(getProp("style"));
        }

        public void setStyle(DividerStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public FormPosition getPosition() {
            return FormPosition.of(getProp("position"));
        }

        public void setPosition(FormPosition value) {
            setProp("position", value.value);
        }

        public BorderStyle getBorderStyle() {
            return BorderStyle.of(getProp("border_style"));
        }

        public void setBorderStyle(BorderStyle value) {
            setProp("border_style", value.value);
        }

        public WindowCorners getWindowCorners() {
            return WindowCorners.of(getProp("window_corners"));
        }

        public void setWindowCorners(WindowCorners value) {
            setProp("window_corners", value.value);
        }

        public boolean isTransparent() {
            return asBool(getProp("transparent"));
        }

        public void setTransparent(boolean value) {
            setProp("transparent", value);
        }

        public WindowShape getWindowShape() {
            return WindowShape.of(getProp("window_shape"));
        }

        public void setWindowShape(WindowShape value) {
            setProp("window_shape", value.value);
        }

        public double getShapeRadius() {
            return asNumber(getProp("shape_radius"));
        }

        public void setShapeRadius(double value) {
            setProp("shape_radius", value);
        }

        public Aylis.Bounds getShapeRadii() {
            return Aylis.Bounds.fromJson(getProp("shape_radii"));
        }

        public void setShapeRadii(Aylis.Bounds value) {
            setProp("shape_radii", value.toJson());
        }

        public String getShapePath() {
            return asText(getProp("shape_path"));
        }

        public void setShapePath(String value) {
            setProp("shape_path", value);
        }

        public WindowState getWindowState() {
            return WindowState.of(getProp("window_state"));
        }

        public void setWindowState(WindowState value) {
            setProp("window_state", value.value);
        }

        public Transition getTransition() {
            return Transition.of(getProp("transition"));
        }

        public void setTransition(Transition value) {
            setProp("transition", value.value);
        }

        public LayerCaching getLayerCache() {
            return LayerCaching.of(getProp("layer_cache"));
        }

        public void setLayerCache(LayerCaching value) {
            setProp("layer_cache", value.value);
        }

        public long getFrameReuseKb() {
            return (long) asNumber(getProp("frame_reuse_kb"));
        }

        public void setFrameReuseKb(long value) {
            setProp("frame_reuse_kb", value);
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

        public Aylis.AylisComponent getActiveControl() {
            return getRef("active_control");
        }

        public void setActiveControl(Aylis.AylisComponent value) {
            setRef("active_control", value);
        }

        public java.util.List<String> getTabOrder() {
            return asList(getProp("tab_order"));
        }

        public void setTabOrder(java.util.List<String> value) {
            setProp("tab_order", listText(value));
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

        public FormPresent getPresent() {
            return FormPresent.of(getProp("present"));
        }

        public void setPresent(FormPresent value) {
            setProp("present", value.value);
        }

        public boolean isSpanMonitors() {
            return asBool(getProp("span_monitors"));
        }

        public void setSpanMonitors(boolean value) {
            setProp("span_monitors", value);
        }

        public boolean isPhysicalPixels() {
            return asBool(getProp("physical_pixels"));
        }

        public void setPhysicalPixels(boolean value) {
            setProp("physical_pixels", value);
        }

        public boolean isActivateOnShow() {
            return asBool(getProp("activate_on_show"));
        }

        public void setActivateOnShow(boolean value) {
            setProp("activate_on_show", value);
        }

        public boolean isForceForeground() {
            return asBool(getProp("force_foreground"));
        }

        public void setForceForeground(boolean value) {
            setProp("force_foreground", value);
        }

        public CloseAction getCloseAction() {
            return CloseAction.of(getProp("close_action"));
        }

        public void setCloseAction(CloseAction value) {
            setProp("close_action", value.value);
        }

        public double getFrameRate() {
            return asNumber(getProp("frame_rate"));
        }

        public void setFrameRate(double value) {
            setProp("frame_rate", value);
        }

        public FormOrientation getOrientation() {
            return FormOrientation.of(getProp("orientation"));
        }

        public void setOrientation(FormOrientation value) {
            setProp("orientation", value.value);
        }

        public String getOnShow() {
            return asText(getProp("on_show"));
        }

        public void setOnShow(String value) {
            setProp("on_show", value);
        }

        public String getOnWindowCreated() {
            return asText(getProp("on_window_created"));
        }

        public void setOnWindowCreated(String value) {
            setProp("on_window_created", value);
        }

        public String getOnClose() {
            return asText(getProp("on_close"));
        }

        public void setOnClose(String value) {
            setProp("on_close", value);
        }

        public String getOnActivate() {
            return asText(getProp("on_activate"));
        }

        public void setOnActivate(String value) {
            setProp("on_activate", value);
        }

        public String getOnDeactivate() {
            return asText(getProp("on_deactivate"));
        }

        public void setOnDeactivate(String value) {
            setProp("on_deactivate", value);
        }

        public String getOnMinimize() {
            return asText(getProp("on_minimize"));
        }

        public void setOnMinimize(String value) {
            setProp("on_minimize", value);
        }

        public String getOnRestore() {
            return asText(getProp("on_restore"));
        }

        public void setOnRestore(String value) {
            setProp("on_restore", value);
        }

        public String getOnNavigate() {
            return asText(getProp("on_navigate"));
        }

        public void setOnNavigate(String value) {
            setProp("on_navigate", value);
        }

        public String getOnGpuChange() {
            return asText(getProp("on_gpu_change"));
        }

        public void setOnGpuChange(String value) {
            setProp("on_gpu_change", value);
        }

        public double getClientWidth() {
            return asNumber(getProp("client_width"));
        }

        public void setClientWidth(double value) {
            setProp("client_width", value);
        }

        public double getClientHeight() {
            return asNumber(getProp("client_height"));
        }

        public void setClientHeight(double value) {
            setProp("client_height", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public FormStyle getStyle() {
            return FormStyle.of(getProp("style"));
        }

        public void setStyle(FormStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
        }

        public void moveToMonitor(long monitor) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "move_to_monitor", "args", java.util.List.of(monitor)));
        }

        public String gpuAdapter() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapter", "args", java.util.List.of())));
        }

        public java.util.List<String> gpuAdapters() {
            return asList(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapters", "args", java.util.List.of())));
        }

        public String renderRgba(double x, double y, double width, double height, double scale) {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "render_rgba", "args", java.util.List.of(x, y, width, height, scale))));
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public GridLayoutStyle getStyle() {
            return GridLayoutStyle.of(getProp("style"));
        }

        public void setStyle(GridLayoutStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public TextAlign getHAlign() {
            return TextAlign.of(getProp("h_align"));
        }

        public void setHAlign(TextAlign value) {
            setProp("h_align", value.value);
        }

        public TextAlign getVAlign() {
            return TextAlign.of(getProp("v_align"));
        }

        public void setVAlign(TextAlign value) {
            setProp("v_align", value.value);
        }

        public boolean isWrap() {
            return asBool(getProp("wrap"));
        }

        public void setWrap(boolean value) {
            setProp("wrap", value);
        }

        public Trimming getTrimming() {
            return Trimming.of(getProp("trimming"));
        }

        public void setTrimming(Trimming value) {
            setProp("trimming", value.value);
        }

        public boolean isBold() {
            return asBool(getProp("bold"));
        }

        public void setBold(boolean value) {
            setProp("bold", value);
        }

        public Aylis.Color getColor() {
            return Aylis.Color.fromJson(getProp("color"));
        }

        public void setColor(Aylis.Color value) {
            setProp("color", value.toJson());
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

        public TextCase getTextCase() {
            return TextCase.of(getProp("text_case"));
        }

        public void setTextCase(TextCase value) {
            setProp("text_case", value.value);
        }

        public boolean isMarkup() {
            return asBool(getProp("markup"));
        }

        public void setMarkup(boolean value) {
            setProp("markup", value);
        }

        public Aylis.Color getOutline() {
            return Aylis.Color.fromJson(getProp("outline"));
        }

        public void setOutline(Aylis.Color value) {
            setProp("outline", value.toJson());
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

        public Aylis.Shadow getTextShadow() {
            return Aylis.Shadow.fromJson(getProp("text_shadow"));
        }

        public void setTextShadow(Aylis.Shadow value) {
            setProp("text_shadow", (value == null ? "" : value.toJson()));
        }

        public Aylis.AylisComponent getFocusControl() {
            return getRef("focus_control");
        }

        public void setFocusControl(Aylis.AylisComponent value) {
            setRef("focus_control", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public LabelStyle getStyle() {
            return LabelStyle.of(getProp("style"));
        }

        public void setStyle(LabelStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public LayoutStyle getStyle() {
            return LayoutStyle.of(getProp("style"));
        }

        public void setStyle(LayoutStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public MainMenuStyle getStyle() {
            return MainMenuStyle.of(getProp("style"));
        }

        public void setStyle(MainMenuStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public boolean isHeader() {
            return asBool(getProp("header"));
        }

        public void setHeader(boolean value) {
            setProp("header", value);
        }

        public Aylis.AylisComponent getContent() {
            return getRef("content");
        }

        public void setContent(Aylis.AylisComponent value) {
            setRef("content", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
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

        public String getOnSelect() {
            return asText(getProp("on_select"));
        }

        public void setOnSelect(String value) {
            setProp("on_select", value);
        }

        public String getOnCancel() {
            return asText(getProp("on_cancel"));
        }

        public void setOnCancel(String value) {
            setProp("on_cancel", value);
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

        public PaintTool getTool() {
            return PaintTool.of(getProp("tool"));
        }

        public void setTool(PaintTool value) {
            setProp("tool", value.value);
        }

        public Aylis.Color getColor() {
            return Aylis.Color.fromJson(getProp("color"));
        }

        public void setColor(Aylis.Color value) {
            setProp("color", value.toJson());
        }

        public Aylis.Color getBackColor() {
            return Aylis.Color.fromJson(getProp("back_color"));
        }

        public void setBackColor(Aylis.Color value) {
            setProp("back_color", value.toJson());
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

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
        }

        public String getOnPick() {
            return asText(getProp("on_pick"));
        }

        public void setOnPick(String value) {
            setProp("on_pick", value);
        }

        public String getOnCursor() {
            return asText(getProp("on_cursor"));
        }

        public void setOnCursor(String value) {
            setProp("on_cursor", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public PaintCanvasStyle getStyle() {
            return PaintCanvasStyle.of(getProp("style"));
        }

        public void setStyle(PaintCanvasStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public ImageSize size() {
            return ImageSize.fromText(aylis.call("invoke", Aylis.obj("target", id, "method", "size", "args", java.util.List.of())));
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

        public double getGap() {
            return asNumber(getProp("gap"));
        }

        public void setGap(double value) {
            setProp("gap", value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getFill2() {
            return Aylis.Color.fromJson(getProp("fill2"));
        }

        public void setFill2(Aylis.Color value) {
            setProp("fill2", value.toJson());
        }

        public Aylis.Color getFillMid() {
            return Aylis.Color.fromJson(getProp("fill_mid"));
        }

        public void setFillMid(Aylis.Color value) {
            setProp("fill_mid", value.toJson());
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

        public Aylis.Color getStroke() {
            return Aylis.Color.fromJson(getProp("stroke"));
        }

        public void setStroke(Aylis.Color value) {
            setProp("stroke", value.toJson());
        }

        public double getStrokeWidth() {
            return asNumber(getProp("stroke_width"));
        }

        public void setStrokeWidth(double value) {
            setProp("stroke_width", value);
        }

        public Aylis.Bounds getRadius() {
            return Aylis.Bounds.fromJson(getProp("radius"));
        }

        public void setRadius(Aylis.Bounds value) {
            setProp("radius", value.toJson());
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public PanelStyle getStyle() {
            return PanelStyle.of(getProp("style"));
        }

        public void setStyle(PanelStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        public String getOnSelect() {
            return asText(getProp("on_select"));
        }

        public void setOnSelect(String value) {
            setProp("on_select", value);
        }

        public String getOnCancel() {
            return asText(getProp("on_cancel"));
        }

        public void setOnCancel(String value) {
            setProp("on_cancel", value);
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

        public Overscroll getOverscroll() {
            return Overscroll.of(getProp("overscroll"));
        }

        public void setOverscroll(Overscroll value) {
            setProp("overscroll", value.value);
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

        public SmoothScroll getSmooth() {
            return SmoothScroll.of(getProp("smooth"));
        }

        public void setSmooth(SmoothScroll value) {
            setProp("smooth", value.value);
        }

        public boolean isGutter() {
            return asBool(getProp("gutter"));
        }

        public void setGutter(boolean value) {
            setProp("gutter", value);
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

        public String getOnEndReached() {
            return asText(getProp("on_end_reached"));
        }

        public void setOnEndReached(String value) {
            setProp("on_end_reached", value);
        }

        public String getOnRefresh() {
            return asText(getProp("on_refresh"));
        }

        public void setOnRefresh(String value) {
            setProp("on_refresh", value);
        }

        public String getOnScroll() {
            return asText(getProp("on_scroll"));
        }

        public void setOnScroll(String value) {
            setProp("on_scroll", value);
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public ScrollBoxStyle getStyle() {
            return ScrollBoxStyle.of(getProp("style"));
        }

        public void setStyle(ScrollBoxStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
        }

        public void scrollTo(Aylis.AylisComponent component) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "scroll_to", "args", java.util.List.of(component.id)));
        }

        public String visibleComponents(Aylis.AylisComponent list, boolean fully) {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "visible_components", "args", java.util.List.of(list.id, fully))));
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

        public Align getAlign() {
            return Align.of(getProp("align"));
        }

        public void setAlign(Align value) {
            setProp("align", value.value);
        }

        public java.util.List<Anchor> getAnchors() {
            return Anchor.listOf(getProp("anchors"));
        }

        public void setAnchors(java.util.List<Anchor> value) {
            setProp("anchors", Anchor.names(value));
        }

        public Aylis.Bounds getMargins() {
            return Aylis.Bounds.fromJson(getProp("margins"));
        }

        public void setMargins(Aylis.Bounds value) {
            setProp("margins", value.toJson());
        }

        public Aylis.Bounds getPadding() {
            return Aylis.Bounds.fromJson(getProp("padding"));
        }

        public void setPadding(Aylis.Bounds value) {
            setProp("padding", value.toJson());
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

        public boolean isCapturesGesture() {
            return asBool(getProp("captures_gesture"));
        }

        public void setCapturesGesture(boolean value) {
            setProp("captures_gesture", value);
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

        public Aylis.AylisComponent getDragImage() {
            return getRef("drag_image");
        }

        public void setDragImage(Aylis.AylisComponent value) {
            setRef("drag_image", value);
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

        public CursorShape getCursor() {
            return CursorShape.of(getProp("cursor"));
        }

        public void setCursor(CursorShape value) {
            setProp("cursor", value.value);
        }

        public double getOpacity() {
            return asNumber(getProp("opacity"));
        }

        public void setOpacity(double value) {
            setProp("opacity", value);
        }

        public BlendMode getBlendMode() {
            return BlendMode.of(getProp("blend_mode"));
        }

        public void setBlendMode(BlendMode value) {
            setProp("blend_mode", value.value);
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

        public double getScale() {
            return asNumber(getProp("scale"));
        }

        public void setScale(double value) {
            setProp("scale", value);
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

        public SelfAlign getSelfAlign() {
            return SelfAlign.of(getProp("self_align"));
        }

        public void setSelfAlign(SelfAlign value) {
            setProp("self_align", value.value);
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

        public Aylis.Shadow getShadow() {
            return Aylis.Shadow.fromJson(getProp("shadow"));
        }

        public void setShadow(Aylis.Shadow value) {
            setProp("shadow", (value == null ? "" : value.toJson()));
        }

        public StatusBarStyle getStyle() {
            return StatusBarStyle.of(getProp("style"));
        }

        public void setStyle(StatusBarStyle value) {
            setProp("style", value.value);
        }

        public ThemeName getTheme() {
            return ThemeName.of(getProp("theme"));
        }

        public void setTheme(ThemeName value) {
            setProp("theme", value.value);
        }

        public Aylis.Color getFill() {
            return Aylis.Color.fromJson(getProp("fill"));
        }

        public void setFill(Aylis.Color value) {
            setProp("fill", value.toJson());
        }

        public Aylis.Color getBorderColor() {
            return Aylis.Color.fromJson(getProp("border_color"));
        }

        public void setBorderColor(Aylis.Color value) {
            setProp("border_color", value.toJson());
        }

        public Aylis.Color getTextColor() {
            return Aylis.Color.fromJson(getProp("text_color"));
        }

        public void setTextColor(Aylis.Color value) {
            setProp("text_color", value.toJson());
        }

        public Aylis.Color getAccentColor() {
            return Aylis.Color.fromJson(getProp("accent_color"));
        }

        public void setAccentColor(Aylis.Color value) {
            setProp("accent_color", value.toJson());
        }

        public Aylis.Color getSuccessColor() {
            return Aylis.Color.fromJson(getProp("success_color"));
        }

        public void setSuccessColor(Aylis.Color value) {
            setProp("success_color", value.toJson());
        }

        public Aylis.Color getWarningColor() {
            return Aylis.Color.fromJson(getProp("warning_color"));
        }

        public void setWarningColor(Aylis.Color value) {
            setProp("warning_color", value.toJson());
        }

        public Aylis.Color getDangerColor() {
            return Aylis.Color.fromJson(getProp("danger_color"));
        }

        public void setDangerColor(Aylis.Color value) {
            setProp("danger_color", value.toJson());
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

        public FontWeight getFontWeight() {
            return FontWeight.of(getProp("font_weight"));
        }

        public void setFontWeight(FontWeight value) {
            setProp("font_weight", value.value);
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

        public Aylis.AylisComponent getPopupMenu() {
            return getRef("popup_menu");
        }

        public void setPopupMenu(Aylis.AylisComponent value) {
            setRef("popup_menu", value);
        }

        public String getOnClick() {
            return asText(getProp("on_click"));
        }

        public void setOnClick(String value) {
            setProp("on_click", value);
        }

        public String getOnDblClick() {
            return asText(getProp("on_dbl_click"));
        }

        public void setOnDblClick(String value) {
            setProp("on_dbl_click", value);
        }

        public String getOnLongPress() {
            return asText(getProp("on_long_press"));
        }

        public void setOnLongPress(String value) {
            setProp("on_long_press", value);
        }

        public String getOnPointerDown() {
            return asText(getProp("on_pointer_down"));
        }

        public void setOnPointerDown(String value) {
            setProp("on_pointer_down", value);
        }

        public String getOnPointerUp() {
            return asText(getProp("on_pointer_up"));
        }

        public void setOnPointerUp(String value) {
            setProp("on_pointer_up", value);
        }

        public String getOnPointerMove() {
            return asText(getProp("on_pointer_move"));
        }

        public void setOnPointerMove(String value) {
            setProp("on_pointer_move", value);
        }

        public String getOnPointerEnter() {
            return asText(getProp("on_pointer_enter"));
        }

        public void setOnPointerEnter(String value) {
            setProp("on_pointer_enter", value);
        }

        public String getOnPointerLeave() {
            return asText(getProp("on_pointer_leave"));
        }

        public void setOnPointerLeave(String value) {
            setProp("on_pointer_leave", value);
        }

        public String getOnWheel() {
            return asText(getProp("on_wheel"));
        }

        public void setOnWheel(String value) {
            setProp("on_wheel", value);
        }

        public String getOnGesture() {
            return asText(getProp("on_gesture"));
        }

        public void setOnGesture(String value) {
            setProp("on_gesture", value);
        }

        public String getOnSwipe() {
            return asText(getProp("on_swipe"));
        }

        public void setOnSwipe(String value) {
            setProp("on_swipe", value);
        }

        public String getOnKeyUp() {
            return asText(getProp("on_key_up"));
        }

        public void setOnKeyUp(String value) {
            setProp("on_key_up", value);
        }

        public String getOnEnter() {
            return asText(getProp("on_enter"));
        }

        public void setOnEnter(String value) {
            setProp("on_enter", value);
        }

        public String getOnExit() {
            return asText(getProp("on_exit"));
        }

        public void setOnExit(String value) {
            setProp("on_exit", value);
        }

        public String getOnResize() {
            return asText(getProp("on_resize"));
        }

        public void setOnResize(String value) {
            setProp("on_resize", value);
        }

        public String getOnDragMove() {
            return asText(getProp("on_drag_move"));
        }

        public void setOnDragMove(String value) {
            setProp("on_drag_move", value);
        }

        public String getOnDragLeave() {
            return asText(getProp("on_drag_leave"));
        }

        public void setOnDragLeave(String value) {
            setProp("on_drag_leave", value);
        }

        public String getOnDragDrop() {
            return asText(getProp("on_drag_drop"));
        }

        public void setOnDragDrop(String value) {
            setProp("on_drag_drop", value);
        }

        public String getOnDragEnd() {
            return asText(getProp("on_drag_end"));
        }

        public void setOnDragEnd(String value) {
            setProp("on_drag_end", value);
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

        /** O Button deste form com o nome. */
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

        /** O CheckBox deste form com o nome. */
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

        /** O Circle deste form com o nome. */
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

        /** O ComboBox deste form com o nome. */
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

        /** O Divider deste form com o nome. */
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

        /** O GridLayout deste form com o nome. */
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

        /** O Label deste form com o nome. */
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

        /** O Layout deste form com o nome. */
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

        /** O MainMenu deste form com o nome. */
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

        /** O MenuItem deste form com o nome. */
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

        /** O OpenDialog deste form com o nome. */
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

        /** O PaintCanvas deste form com o nome. */
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

        /** O Panel deste form com o nome. */
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

        /** O SaveDialog deste form com o nome. */
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

        /** O ScrollBox deste form com o nome. */
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

        /** O StatusBar deste form com o nome. */
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
