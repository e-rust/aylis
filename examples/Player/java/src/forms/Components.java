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
    public enum IconStyle {
        DEFAULT(""),
        MUTED("muted");

        public final String value;

        IconStyle(String value) {
            this.value = value;
        }

        public static IconStyle of(Object v) {
            for (IconStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<IconStyle> listOf(Object v) {
            java.util.List<IconStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (IconStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<IconStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (IconStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ImageFit {
        CONTAIN("contain"),
        COVER("cover"),
        STRETCH("stretch"),
        CENTER("center"),
        ORIGINAL("original");

        public final String value;

        ImageFit(String value) {
            this.value = value;
        }

        public static ImageFit of(Object v) {
            for (ImageFit e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ImageFit> listOf(Object v) {
            java.util.List<ImageFit> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ImageFit e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ImageFit> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ImageFit e : values) out.add(e.value);
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
    public enum ListViewStyle {
        DEFAULT(""),
        FLAT("flat");

        public final String value;

        ListViewStyle(String value) {
            this.value = value;
        }

        public static ListViewStyle of(Object v) {
            for (ListViewStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ListViewStyle> listOf(Object v) {
            java.util.List<ListViewStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ListViewStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ListViewStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ListViewStyle e : values) out.add(e.value);
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
    public enum SeekBarStyle {
        DEFAULT("");

        public final String value;

        SeekBarStyle(String value) {
            this.value = value;
        }

        public static SeekBarStyle of(Object v) {
            for (SeekBarStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<SeekBarStyle> listOf(Object v) {
            java.util.List<SeekBarStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (SeekBarStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<SeekBarStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (SeekBarStyle e : values) out.add(e.value);
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
    public enum TimerPrecision {
        NORMAL("normal"),
        HIGH("high"),
        EXACT("exact");

        public final String value;

        TimerPrecision(String value) {
            this.value = value;
        }

        public static TimerPrecision of(Object v) {
            for (TimerPrecision e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TimerPrecision> listOf(Object v) {
            java.util.List<TimerPrecision> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TimerPrecision e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TimerPrecision> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TimerPrecision e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum TrackBarStyle {
        DEFAULT("");

        public final String value;

        TrackBarStyle(String value) {
            this.value = value;
        }

        public static TrackBarStyle of(Object v) {
            for (TrackBarStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TrackBarStyle> listOf(Object v) {
            java.util.List<TrackBarStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TrackBarStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TrackBarStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TrackBarStyle e : values) out.add(e.value);
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
    public enum VideoPlayerStyle {
        DEFAULT("");

        public final String value;

        VideoPlayerStyle(String value) {
            this.value = value;
        }

        public static VideoPlayerStyle of(Object v) {
            for (VideoPlayerStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<VideoPlayerStyle> listOf(Object v) {
            java.util.List<VideoPlayerStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (VideoPlayerStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<VideoPlayerStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (VideoPlayerStyle e : values) out.add(e.value);
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
    public record ListViewItem(String title, String detail) {
        static ListViewItem fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new ListViewItem(asText(m.get("title")), asText(m.get("detail")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("title", title);
            m.put("detail", detail);
            return m;
        }

        static java.util.List<ListViewItem> listFromJson(Object v) {
            java.util.List<ListViewItem> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static ListViewItem fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<ListViewItem> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(ListViewItem r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<ListViewItem> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code ListViewItem.of().with...(valor)}. */
        public static ListViewItem of() {
            return fromJson(null);
        }

        public ListViewItem withTitle(String v) {
            return new ListViewItem(v, detail);
        }

        public ListViewItem withDetail(String v) {
            return new ListViewItem(title, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record OutputSpec(String name, double volume, double delayMs) {
        static OutputSpec fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new OutputSpec(asText(m.get("name")), asNumber(m.get("volume")), asNumber(m.get("delay_ms")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", name);
            m.put("volume", volume);
            m.put("delay_ms", delayMs);
            return m;
        }

        static java.util.List<OutputSpec> listFromJson(Object v) {
            java.util.List<OutputSpec> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static OutputSpec fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<OutputSpec> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(OutputSpec r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<OutputSpec> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code OutputSpec.of().with...(valor)}. */
        public static OutputSpec of() {
            return fromJson(null);
        }

        public OutputSpec withName(String v) {
            return new OutputSpec(v, volume, delayMs);
        }

        public OutputSpec withVolume(double v) {
            return new OutputSpec(name, v, delayMs);
        }

        public OutputSpec withDelayMs(double v) {
            return new OutputSpec(name, volume, v);
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

        public Aylis.Color getColor() {
            return Aylis.Color.fromJson(getProp("color"));
        }

        public void setColor(Aylis.Color value) {
            setProp("color", value.toJson());
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

        public IconStyle getStyle() {
            return IconStyle.of(getProp("style"));
        }

        public void setStyle(IconStyle value) {
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

    /** Um ListView de um form. */
    public static class ListView extends Aylis.AylisComponent {
        public ListView(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public java.util.List<ListViewItem> getItems() {
            return ListViewItem.listFromText(getProp("items"));
        }

        public void setItems(java.util.List<ListViewItem> value) {
            setProp("items", ListViewItem.listToText(value));
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

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
        }

        public String getOnItemClick() {
            return asText(getProp("on_item_click"));
        }

        public void setOnItemClick(String value) {
            setProp("on_item_click", value);
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

        public ListViewStyle getStyle() {
            return ListViewStyle.of(getProp("style"));
        }

        public void setStyle(ListViewStyle value) {
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

        public java.util.List<Double> getMarkers() {
            return asNumbers(getProp("markers"));
        }

        public void setMarkers(java.util.List<Double> value) {
            setProp("markers", listText(value));
        }

        public boolean isShowTime() {
            return asBool(getProp("show_time"));
        }

        public void setShowTime(boolean value) {
            setProp("show_time", value);
        }

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
        }

        public String getOnSeek() {
            return asText(getProp("on_seek"));
        }

        public void setOnSeek(String value) {
            setProp("on_seek", value);
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

        public SeekBarStyle getStyle() {
            return SeekBarStyle.of(getProp("style"));
        }

        public void setStyle(SeekBarStyle value) {
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

        public TimerPrecision getPrecision() {
            return TimerPrecision.of(getProp("precision"));
        }

        public void setPrecision(TimerPrecision value) {
            setProp("precision", value.value);
        }

        public boolean isBackground() {
            return asBool(getProp("background"));
        }

        public void setBackground(boolean value) {
            setProp("background", value);
        }

        public boolean isOneShot() {
            return asBool(getProp("one_shot"));
        }

        public void setOneShot(boolean value) {
            setProp("one_shot", value);
        }

        public String getOnTimer() {
            return asText(getProp("on_timer"));
        }

        public void setOnTimer(String value) {
            setProp("on_timer", value);
        }

        public double getDrift() {
            return asNumber(getProp("drift"));
        }

        public void setDrift(double value) {
            setProp("drift", value);
        }

        public void restart() {
            aylis.send("invoke", Aylis.obj("target", id, "method", "restart", "args", java.util.List.of()));
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

        public TrackBarStyle getStyle() {
            return TrackBarStyle.of(getProp("style"));
        }

        public void setStyle(TrackBarStyle value) {
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

        public ImageFit getFit() {
            return ImageFit.of(getProp("fit"));
        }

        public void setFit(ImageFit value) {
            setProp("fit", value.value);
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

        public java.util.List<OutputSpec> getOutput() {
            return OutputSpec.listFromText(getProp("output"));
        }

        public void setOutput(java.util.List<OutputSpec> value) {
            setProp("output", OutputSpec.listToText(value));
        }

        public boolean isShowVideo() {
            return asBool(getProp("show_video"));
        }

        public void setShowVideo(boolean value) {
            setProp("show_video", value);
        }

        public long getFramesAhead() {
            return (long) asNumber(getProp("frames_ahead"));
        }

        public void setFramesAhead(long value) {
            setProp("frames_ahead", value);
        }

        public String getOnReady() {
            return asText(getProp("on_ready"));
        }

        public void setOnReady(String value) {
            setProp("on_ready", value);
        }

        public String getOnEnd() {
            return asText(getProp("on_end"));
        }

        public void setOnEnd(String value) {
            setProp("on_end", value);
        }

        public String getOnError() {
            return asText(getProp("on_error"));
        }

        public void setOnError(String value) {
            setProp("on_error", value);
        }

        public String getOnFrame() {
            return asText(getProp("on_frame"));
        }

        public void setOnFrame(String value) {
            setProp("on_frame", value);
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

        public VideoPlayerStyle getStyle() {
            return VideoPlayerStyle.of(getProp("style"));
        }

        public void setStyle(VideoPlayerStyle value) {
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

        /** O Button deste form com o nome. */
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

        /** O Divider deste form com o nome. */
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

        /** O Icon deste form com o nome. */
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

        /** O Label deste form com o nome. */
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

        /** O Layout deste form com o nome. */
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

        /** O ListView deste form com o nome. */
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

        /** O Panel deste form com o nome. */
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

        /** O SeekBar deste form com o nome. */
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

        /** O Timer deste form com o nome. */
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

        /** O TrackBar deste form com o nome. */
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

        /** O VideoPlayer deste form com o nome. */
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
