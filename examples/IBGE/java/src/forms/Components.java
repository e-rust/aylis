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
    public enum ApplyMode {
        CACHED("cached"),
        ON_POST("on_post");

        public final String value;

        ApplyMode(String value) {
            this.value = value;
        }

        public static ApplyMode of(Object v) {
            for (ApplyMode e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ApplyMode> listOf(Object v) {
            java.util.List<ApplyMode> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ApplyMode e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ApplyMode> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ApplyMode e : values) out.add(e.value);
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
    public enum CharCase {
        NORMAL("normal"),
        UPPER("upper"),
        LOWER("lower");

        public final String value;

        CharCase(String value) {
            this.value = value;
        }

        public static CharCase of(Object v) {
            for (CharCase e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CharCase> listOf(Object v) {
            java.util.List<CharCase> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CharCase e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CharCase> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CharCase e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ChartKind {
        LINE("line"),
        BAR("bar"),
        AREA("area"),
        PIE("pie"),
        DONUT("donut"),
        SPARKLINE("sparkline"),
        HEATMAP("heatmap"),
        SCATTER("scatter"),
        STACKED_BAR("stacked_bar"),
        WATERFALL("waterfall"),
        FUNNEL("funnel"),
        TREEMAP("treemap"),
        RADAR("radar"),
        HORIZONTAL_BAR("horizontal_bar"),
        BUBBLE("bubble"),
        POLAR_AREA("polar_area"),
        STACKED_LINE("stacked_line"),
        STACKED_AREA("stacked_area"),
        MIXED("mixed"),
        SANKEY("sankey"),
        WAFFLE("waffle");

        public final String value;

        ChartKind(String value) {
            this.value = value;
        }

        public static ChartKind of(Object v) {
            for (ChartKind e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ChartKind> listOf(Object v) {
            java.util.List<ChartKind> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ChartKind e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ChartKind> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ChartKind e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ChartSeriesFill {
        NONE("none"),
        ORIGIN("origin"),
        START("start"),
        END("end"),
        SERIES("series");

        public final String value;

        ChartSeriesFill(String value) {
            this.value = value;
        }

        public static ChartSeriesFill of(Object v) {
            for (ChartSeriesFill e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ChartSeriesFill> listOf(Object v) {
            java.util.List<ChartSeriesFill> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ChartSeriesFill e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ChartSeriesFill> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ChartSeriesFill e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ChartSeriesKind {
        LINE("line"),
        BAR("bar"),
        AREA("area"),
        SCATTER("scatter");

        public final String value;

        ChartSeriesKind(String value) {
            this.value = value;
        }

        public static ChartSeriesKind of(Object v) {
            for (ChartSeriesKind e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ChartSeriesKind> listOf(Object v) {
            java.util.List<ChartSeriesKind> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ChartSeriesKind e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ChartSeriesKind> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ChartSeriesKind e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum ChartSeriesPoint {
        CIRCLE("circle"),
        SQUARE("square"),
        TRIANGLE("triangle"),
        DIAMOND("diamond"),
        CROSS("cross"),
        NONE("none");

        public final String value;

        ChartSeriesPoint(String value) {
            this.value = value;
        }

        public static ChartSeriesPoint of(Object v) {
            for (ChartSeriesPoint e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ChartSeriesPoint> listOf(Object v) {
            java.util.List<ChartSeriesPoint> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ChartSeriesPoint e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ChartSeriesPoint> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ChartSeriesPoint e : values) out.add(e.value);
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
    public enum DBChartStyle {
        DEFAULT("");

        public final String value;

        DBChartStyle(String value) {
            this.value = value;
        }

        public static DBChartStyle of(Object v) {
            for (DBChartStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<DBChartStyle> listOf(Object v) {
            java.util.List<DBChartStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (DBChartStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<DBChartStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (DBChartStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum DataGridStyle {
        DEFAULT("");

        public final String value;

        DataGridStyle(String value) {
            this.value = value;
        }

        public static DataGridStyle of(Object v) {
            for (DataGridStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<DataGridStyle> listOf(Object v) {
            java.util.List<DataGridStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (DataGridStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<DataGridStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (DataGridStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FieldKind {
        DATA("data"),
        CALCULATED("calculated"),
        LOOKUP("lookup");

        public final String value;

        FieldKind(String value) {
            this.value = value;
        }

        public static FieldKind of(Object v) {
            for (FieldKind e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FieldKind> listOf(Object v) {
            java.util.List<FieldKind> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FieldKind e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FieldKind> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FieldKind e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum FieldType {
        STRING("string"),
        MEMO("memo"),
        INTEGER("integer"),
        FLOAT("float"),
        CURRENCY("currency"),
        BOOLEAN("boolean"),
        DATE("date"),
        TIME("time"),
        DATETIME("datetime"),
        BLOB("blob"),
        AUTO_INC("auto_inc"),
        TIMESTAMP("timestamp");

        public final String value;

        FieldType(String value) {
            this.value = value;
        }

        public static FieldType of(Object v) {
            for (FieldType e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FieldType> listOf(Object v) {
            java.util.List<FieldType> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FieldType e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FieldType> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FieldType e : values) out.add(e.value);
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
    public enum GridTotalFunction {
        SUM("sum"),
        AVG("avg"),
        MIN("min"),
        MAX("max"),
        COUNT("count");

        public final String value;

        GridTotalFunction(String value) {
            this.value = value;
        }

        public static GridTotalFunction of(Object v) {
            for (GridTotalFunction e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<GridTotalFunction> listOf(Object v) {
            java.util.List<GridTotalFunction> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (GridTotalFunction e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<GridTotalFunction> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (GridTotalFunction e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum GroupBoxStyle {
        DEFAULT("");

        public final String value;

        GroupBoxStyle(String value) {
            this.value = value;
        }

        public static GroupBoxStyle of(Object v) {
            for (GroupBoxStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<GroupBoxStyle> listOf(Object v) {
            java.util.List<GroupBoxStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (GroupBoxStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<GroupBoxStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (GroupBoxStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum HorizontalWheel {
        SHIFT("shift"),
        ALWAYS("always"),
        NONE("none");

        public final String value;

        HorizontalWheel(String value) {
            this.value = value;
        }

        public static HorizontalWheel of(Object v) {
            for (HorizontalWheel e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<HorizontalWheel> listOf(Object v) {
            java.util.List<HorizontalWheel> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (HorizontalWheel e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<HorizontalWheel> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (HorizontalWheel e : values) out.add(e.value);
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
    public enum ImageStyle {
        DEFAULT("");

        public final String value;

        ImageStyle(String value) {
            this.value = value;
        }

        public static ImageStyle of(Object v) {
            for (ImageStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ImageStyle> listOf(Object v) {
            java.util.List<ImageStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ImageStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ImageStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ImageStyle e : values) out.add(e.value);
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
    public enum LegendPosition {
        TOP_RIGHT("top_right"),
        TOP_LEFT("top_left"),
        TOP("top"),
        BOTTOM("bottom");

        public final String value;

        LegendPosition(String value) {
            this.value = value;
        }

        public static LegendPosition of(Object v) {
            for (LegendPosition e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<LegendPosition> listOf(Object v) {
            java.util.List<LegendPosition> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (LegendPosition e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<LegendPosition> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (LegendPosition e : values) out.add(e.value);
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
    public enum Rounding {
        HALF_UP("half_up"),
        HALF_EVEN("half_even"),
        TRUNCATE("truncate"),
        UP("up");

        public final String value;

        Rounding(String value) {
            this.value = value;
        }

        public static Rounding of(Object v) {
            for (Rounding e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<Rounding> listOf(Object v) {
            java.util.List<Rounding> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (Rounding e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<Rounding> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (Rounding e : values) out.add(e.value);
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
    public enum SplitterStyle {
        DEFAULT("");

        public final String value;

        SplitterStyle(String value) {
            this.value = value;
        }

        public static SplitterStyle of(Object v) {
            for (SplitterStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<SplitterStyle> listOf(Object v) {
            java.util.List<SplitterStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (SplitterStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<SplitterStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (SplitterStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum SvgStyle {
        DEFAULT("");

        public final String value;

        SvgStyle(String value) {
            this.value = value;
        }

        public static SvgStyle of(Object v) {
            for (SvgStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<SvgStyle> listOf(Object v) {
            java.util.List<SvgStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (SvgStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<SvgStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (SvgStyle e : values) out.add(e.value);
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
    public enum TitleAlign {
        LEFT("left"),
        CENTER("center"),
        RIGHT("right");

        public final String value;

        TitleAlign(String value) {
            this.value = value;
        }

        public static TitleAlign of(Object v) {
            for (TitleAlign e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TitleAlign> listOf(Object v) {
            java.util.List<TitleAlign> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TitleAlign e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TitleAlign> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TitleAlign e : values) out.add(e.value);
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
    public enum ValueFormat {
        SHORT("short"),
        FULL("full"),
        PERCENT("percent");

        public final String value;

        ValueFormat(String value) {
            this.value = value;
        }

        public static ValueFormat of(Object v) {
            for (ValueFormat e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<ValueFormat> listOf(Object v) {
            java.util.List<ValueFormat> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (ValueFormat e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<ValueFormat> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (ValueFormat e : values) out.add(e.value);
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
    public record ChartSeries(String name, java.util.List<Double> values, java.util.List<Double> x, java.util.List<Double> radius, ChartSeriesKind kind, boolean rightAxis, Aylis.Color color, double width, java.util.List<Double> dash, ChartSeriesPoint point, ChartSeriesFill fill, long fillSeries, boolean hidden) {
        static ChartSeries fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new ChartSeries(asText(m.get("name")), m.get("values") instanceof java.util.List<?> lvalues ? lvalues.stream().map(x -> asNumber(x)).toList() : java.util.List.<Double>of(), m.get("x") instanceof java.util.List<?> lx ? lx.stream().map(x -> asNumber(x)).toList() : java.util.List.<Double>of(), m.get("radius") instanceof java.util.List<?> lradius ? lradius.stream().map(x -> asNumber(x)).toList() : java.util.List.<Double>of(), ChartSeriesKind.of(m.get("kind")), asBool(m.get("right_axis")), Aylis.Color.fromJson(m.get("color")), asNumber(m.get("width")), m.get("dash") instanceof java.util.List<?> ldash ? ldash.stream().map(x -> asNumber(x)).toList() : java.util.List.<Double>of(), ChartSeriesPoint.of(m.get("point")), ChartSeriesFill.of(m.get("fill")), (long) asNumber(m.get("fill_series")), asBool(m.get("hidden")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", name);
            m.put("values", values);
            m.put("x", x);
            m.put("radius", radius);
            m.put("kind", kind == null ? "" : kind.value);
            m.put("right_axis", rightAxis);
            m.put("color", color == null ? "" : color.toJson());
            m.put("width", width);
            m.put("dash", dash);
            m.put("point", point == null ? "" : point.value);
            m.put("fill", fill == null ? "" : fill.value);
            m.put("fill_series", fillSeries);
            m.put("hidden", hidden);
            return m;
        }

        static java.util.List<ChartSeries> listFromJson(Object v) {
            java.util.List<ChartSeries> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static ChartSeries fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<ChartSeries> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(ChartSeries r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<ChartSeries> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code ChartSeries.of().with...(valor)}. */
        public static ChartSeries of() {
            return fromJson(null);
        }

        public ChartSeries withName(String v) {
            return new ChartSeries(v, values, x, radius, kind, rightAxis, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withValues(java.util.List<Double> v) {
            return new ChartSeries(name, v, x, radius, kind, rightAxis, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withX(java.util.List<Double> v) {
            return new ChartSeries(name, values, v, radius, kind, rightAxis, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withRadius(java.util.List<Double> v) {
            return new ChartSeries(name, values, x, v, kind, rightAxis, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withKind(ChartSeriesKind v) {
            return new ChartSeries(name, values, x, radius, v, rightAxis, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withRightAxis(boolean v) {
            return new ChartSeries(name, values, x, radius, kind, v, color, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withColor(Aylis.Color v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, v, width, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withWidth(double v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, v, dash, point, fill, fillSeries, hidden);
        }

        public ChartSeries withDash(java.util.List<Double> v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, width, v, point, fill, fillSeries, hidden);
        }

        public ChartSeries withPoint(ChartSeriesPoint v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, width, dash, v, fill, fillSeries, hidden);
        }

        public ChartSeries withFill(ChartSeriesFill v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, width, dash, point, v, fillSeries, hidden);
        }

        public ChartSeries withFillSeries(long v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, width, dash, point, fill, v, hidden);
        }

        public ChartSeries withHidden(boolean v) {
            return new ChartSeries(name, values, x, radius, kind, rightAxis, color, width, dash, point, fill, fillSeries, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record ColumnSpec(String field, String title, double width, boolean readOnly, Aylis.Color color, Aylis.Color textColor) {
        static ColumnSpec fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new ColumnSpec(asText(m.get("field")), asText(m.get("title")), asNumber(m.get("width")), asBool(m.get("read_only")), Aylis.Color.fromJson(m.get("color")), Aylis.Color.fromJson(m.get("text_color")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("field", field);
            m.put("title", title);
            m.put("width", width);
            m.put("read_only", readOnly);
            m.put("color", color == null ? "" : color.toJson());
            m.put("text_color", textColor == null ? "" : textColor.toJson());
            return m;
        }

        static java.util.List<ColumnSpec> listFromJson(Object v) {
            java.util.List<ColumnSpec> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static ColumnSpec fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<ColumnSpec> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(ColumnSpec r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<ColumnSpec> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code ColumnSpec.of().with...(valor)}. */
        public static ColumnSpec of() {
            return fromJson(null);
        }

        public ColumnSpec withField(String v) {
            return new ColumnSpec(v, title, width, readOnly, color, textColor);
        }

        public ColumnSpec withTitle(String v) {
            return new ColumnSpec(field, v, width, readOnly, color, textColor);
        }

        public ColumnSpec withWidth(double v) {
            return new ColumnSpec(field, title, v, readOnly, color, textColor);
        }

        public ColumnSpec withReadOnly(boolean v) {
            return new ColumnSpec(field, title, width, v, color, textColor);
        }

        public ColumnSpec withColor(Aylis.Color v) {
            return new ColumnSpec(field, title, width, readOnly, v, textColor);
        }

        public ColumnSpec withTextColor(Aylis.Color v) {
            return new ColumnSpec(field, title, width, readOnly, color, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record FilterParam(String name, String value) {
        static FilterParam fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new FilterParam(asText(m.get("name")), asText(m.get("value")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", name);
            m.put("value", value);
            return m;
        }

        static java.util.List<FilterParam> listFromJson(Object v) {
            java.util.List<FilterParam> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static FilterParam fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<FilterParam> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(FilterParam r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<FilterParam> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code FilterParam.of().with...(valor)}. */
        public static FilterParam of() {
            return fromJson(null);
        }

        public FilterParam withName(String v) {
            return new FilterParam(v, value);
        }

        public FilterParam withValue(String v) {
            return new FilterParam(name, v);
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
    public record GridTotal(String field, GridTotalFunction function) {
        static GridTotal fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new GridTotal(asText(m.get("field")), GridTotalFunction.of(m.get("function")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("field", field);
            m.put("function", function == null ? "" : function.value);
            return m;
        }

        static java.util.List<GridTotal> listFromJson(Object v) {
            java.util.List<GridTotal> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static GridTotal fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<GridTotal> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(GridTotal r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<GridTotal> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code GridTotal.of().with...(valor)}. */
        public static GridTotal of() {
            return fromJson(null);
        }

        public GridTotal withField(String v) {
            return new GridTotal(v, function);
        }

        public GridTotal withFunction(GridTotalFunction v) {
            return new GridTotal(field, v);
        }
    }

    /** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
    public record IndexDef(String name, String fields) {
        static IndexDef fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new IndexDef(asText(m.get("name")), asText(m.get("fields")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("name", name);
            m.put("fields", fields);
            return m;
        }

        static java.util.List<IndexDef> listFromJson(Object v) {
            java.util.List<IndexDef> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static IndexDef fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<IndexDef> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(IndexDef r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<IndexDef> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code IndexDef.of().with...(valor)}. */
        public static IndexDef of() {
            return fromJson(null);
        }

        public IndexDef withName(String v) {
            return new IndexDef(v, fields);
        }

        public IndexDef withFields(String v) {
            return new IndexDef(name, v);
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
    public record MasterField(String field, String masterField) {
        static MasterField fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new MasterField(asText(m.get("field")), asText(m.get("master_field")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("field", field);
            m.put("master_field", masterField);
            return m;
        }

        static java.util.List<MasterField> listFromJson(Object v) {
            java.util.List<MasterField> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) out.add(fromJson(x));
            return out;
        }

        static MasterField fromText(Object v) {
            String t = asText(v);
            Object j = t.isEmpty() ? null : Aylis.Json.parse(t);
            return j instanceof java.util.Map<?, ?> ? fromJson(j) : null;
        }

        static java.util.List<MasterField> listFromText(Object v) {
            String t = asText(v);
            return listFromJson(t.isEmpty() ? null : Aylis.Json.parse(t));
        }

        static String toText(MasterField r) {
            return r == null ? "" : Aylis.Json.write(r.toJson());
        }

        static String listToText(java.util.List<MasterField> items) {
            return Aylis.Json.write(items.stream().map(x -> (Object) x.toJson()).toList());
        }

        /** Com os padrões; os {@code with...} trocam um campo: {@code MasterField.of().with...(valor)}. */
        public static MasterField of() {
            return fromJson(null);
        }

        public MasterField withField(String v) {
            return new MasterField(v, masterField);
        }

        public MasterField withMasterField(String v) {
            return new MasterField(field, v);
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

        public ApplyMode getApplyMode() {
            return ApplyMode.of(getProp("apply_mode"));
        }

        public void setApplyMode(ApplyMode value) {
            setProp("apply_mode", value.value);
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

        public java.util.List<IndexDef> getIndexes() {
            return IndexDef.listFromText(getProp("indexes"));
        }

        public void setIndexes(java.util.List<IndexDef> value) {
            setProp("indexes", IndexDef.listToText(value));
        }

        public String getIndexName() {
            return asText(getProp("index_name"));
        }

        public void setIndexName(String value) {
            setProp("index_name", value);
        }

        public java.util.List<FilterParam> getParams() {
            return FilterParam.listFromText(getProp("params"));
        }

        public void setParams(java.util.List<FilterParam> value) {
            setProp("params", FilterParam.listToText(value));
        }

        public String getStatusFilter() {
            return asText(getProp("status_filter"));
        }

        public void setStatusFilter(String value) {
            setProp("status_filter", value);
        }

        public Aylis.AylisComponent getMasterSource() {
            return getRef("master_source");
        }

        public void setMasterSource(Aylis.AylisComponent value) {
            setRef("master_source", value);
        }

        public java.util.List<MasterField> getMasterFields() {
            return MasterField.listFromText(getProp("master_fields"));
        }

        public void setMasterFields(java.util.List<MasterField> value) {
            setProp("master_fields", MasterField.listToText(value));
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

        public String getOnMasterChange() {
            return asText(getProp("on_master_change"));
        }

        public void setOnMasterChange(String value) {
            setProp("on_master_change", value);
        }

        public String getAfterScroll() {
            return asText(getProp("after_scroll"));
        }

        public void setAfterScroll(String value) {
            setProp("after_scroll", value);
        }

        public String getAfterPost() {
            return asText(getProp("after_post"));
        }

        public void setAfterPost(String value) {
            setProp("after_post", value);
        }

        public String getOnStateChange() {
            return asText(getProp("on_state_change"));
        }

        public void setOnStateChange(String value) {
            setProp("on_state_change", value);
        }

        public String getOnReconcileError() {
            return asText(getProp("on_reconcile_error"));
        }

        public void setOnReconcileError(String value) {
            setProp("on_reconcile_error", value);
        }

        public String getOnError() {
            return asText(getProp("on_error"));
        }

        public void setOnError(String value) {
            setProp("on_error", value);
        }

        public String getOnFetch() {
            return asText(getProp("on_fetch"));
        }

        public void setOnFetch(String value) {
            setProp("on_fetch", value);
        }

        public String getOnServerQuery() {
            return asText(getProp("on_server_query"));
        }

        public void setOnServerQuery(String value) {
            setProp("on_server_query", value);
        }
    }

    /** Um DBChart de um form. */
    public static class DBChart extends Aylis.AylisComponent {
        public DBChart(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public Aylis.AylisComponent getDataSource() {
            return getRef("data_source");
        }

        public void setDataSource(Aylis.AylisComponent value) {
            setRef("data_source", value);
        }

        public String getLabelField() {
            return asText(getProp("label_field"));
        }

        public void setLabelField(String value) {
            setProp("label_field", value);
        }

        public java.util.List<String> getValueFields() {
            return asList(getProp("value_fields"));
        }

        public void setValueFields(java.util.List<String> value) {
            setProp("value_fields", listText(value));
        }

        public ChartKind getKind() {
            return ChartKind.of(getProp("kind"));
        }

        public void setKind(ChartKind value) {
            setProp("kind", value.value);
        }

        public String getTitle() {
            return asText(getProp("title"));
        }

        public void setTitle(String value) {
            setProp("title", value);
        }

        public java.util.List<String> getLabels() {
            return asList(getProp("labels"));
        }

        public void setLabels(java.util.List<String> value) {
            setProp("labels", listText(value));
        }

        public java.util.List<ChartSeries> getSeries() {
            return ChartSeries.listFromText(getProp("series"));
        }

        public void setSeries(java.util.List<ChartSeries> value) {
            setProp("series", ChartSeries.listToText(value));
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

        public ValueFormat getValueFormat() {
            return ValueFormat.of(getProp("value_format"));
        }

        public void setValueFormat(ValueFormat value) {
            setProp("value_format", value.value);
        }

        public double getLabelAngle() {
            return asNumber(getProp("label_angle"));
        }

        public void setLabelAngle(double value) {
            setProp("label_angle", value);
        }

        public TitleAlign getTitleAlign() {
            return TitleAlign.of(getProp("title_align"));
        }

        public void setTitleAlign(TitleAlign value) {
            setProp("title_align", value.value);
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

        public LegendPosition getLegendPosition() {
            return LegendPosition.of(getProp("legend_position"));
        }

        public void setLegendPosition(LegendPosition value) {
            setProp("legend_position", value.value);
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

        public DBChartStyle getStyle() {
            return DBChartStyle.of(getProp("style"));
        }

        public void setStyle(DBChartStyle value) {
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

    /** Um DataGrid de um form. */
    public static class DataGrid extends Aylis.AylisComponent {
        public DataGrid(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public Overscroll getOverscroll() {
            return Overscroll.of(getProp("overscroll"));
        }

        public void setOverscroll(Overscroll value) {
            setProp("overscroll", value.value);
        }

        public Aylis.AylisComponent getDataSource() {
            return getRef("data_source");
        }

        public void setDataSource(Aylis.AylisComponent value) {
            setRef("data_source", value);
        }

        public java.util.List<ColumnSpec> getColumns() {
            return ColumnSpec.listFromText(getProp("columns"));
        }

        public void setColumns(java.util.List<ColumnSpec> value) {
            setProp("columns", ColumnSpec.listToText(value));
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

        public java.util.List<GridTotal> getFooter() {
            return GridTotal.listFromText(getProp("footer"));
        }

        public void setFooter(java.util.List<GridTotal> value) {
            setProp("footer", GridTotal.listToText(value));
        }

        public boolean isIncrementalSearch() {
            return asBool(getProp("incremental_search"));
        }

        public void setIncrementalSearch(boolean value) {
            setProp("incremental_search", value);
        }

        public Aylis.Color getCursorColor() {
            return Aylis.Color.fromJson(getProp("cursor_color"));
        }

        public void setCursorColor(Aylis.Color value) {
            setProp("cursor_color", value.toJson());
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

        public HorizontalWheel getHorizontalWheel() {
            return HorizontalWheel.of(getProp("horizontal_wheel"));
        }

        public void setHorizontalWheel(HorizontalWheel value) {
            setProp("horizontal_wheel", value.value);
        }

        public String getOnError() {
            return asText(getProp("on_error"));
        }

        public void setOnError(String value) {
            setProp("on_error", value);
        }

        public String getOnTitleClick() {
            return asText(getProp("on_title_click"));
        }

        public void setOnTitleClick(String value) {
            setProp("on_title_click", value);
        }

        public String getOnSelectionChange() {
            return asText(getProp("on_selection_change"));
        }

        public void setOnSelectionChange(String value) {
            setProp("on_selection_change", value);
        }

        public String getOnColumnMoved() {
            return asText(getProp("on_column_moved"));
        }

        public void setOnColumnMoved(String value) {
            setProp("on_column_moved", value);
        }

        public double getScrollLeft() {
            return asNumber(getProp("scroll_left"));
        }

        public void setScrollLeft(double value) {
            setProp("scroll_left", value);
        }

        public long getTopRow() {
            return (long) asNumber(getProp("top_row"));
        }

        public void setTopRow(long value) {
            setProp("top_row", value);
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

        public DataGridStyle getStyle() {
            return DataGridStyle.of(getProp("style"));
        }

        public void setStyle(DataGridStyle value) {
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

    /** Um DataSource de um form. */
    public static class DataSource extends Aylis.AylisComponent {
        public DataSource(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        public Aylis.AylisComponent getDataSet() {
            return getRef("data_set");
        }

        public void setDataSet(Aylis.AylisComponent value) {
            setRef("data_set", value);
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

        public String getOnDataChange() {
            return asText(getProp("on_data_change"));
        }

        public void setOnDataChange(String value) {
            setProp("on_data_change", value);
        }

        public String getOnStateChange() {
            return asText(getProp("on_state_change"));
        }

        public void setOnStateChange(String value) {
            setProp("on_state_change", value);
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

        public FieldType getType() {
            return FieldType.of(getProp("type"));
        }

        public void setType(FieldType value) {
            setProp("type", value.value);
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

        public Rounding getRounding() {
            return Rounding.of(getProp("rounding"));
        }

        public void setRounding(Rounding value) {
            setProp("rounding", value.value);
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

        public TextAlign getAlignment() {
            return TextAlign.of(getProp("alignment"));
        }

        public void setAlignment(TextAlign value) {
            setProp("alignment", value.value);
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

        public CharCase getCharCase() {
            return CharCase.of(getProp("char_case"));
        }

        public void setCharCase(CharCase value) {
            setProp("char_case", value.value);
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

        public FieldKind getKind() {
            return FieldKind.of(getProp("kind"));
        }

        public void setKind(FieldKind value) {
            setProp("kind", value.value);
        }

        public String getExpression() {
            return asText(getProp("expression"));
        }

        public void setExpression(String value) {
            setProp("expression", value);
        }

        public Aylis.AylisComponent getLookupDataSet() {
            return getRef("lookup_data_set");
        }

        public void setLookupDataSet(Aylis.AylisComponent value) {
            setRef("lookup_data_set", value);
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

        public String getOnChange() {
            return asText(getProp("on_change"));
        }

        public void setOnChange(String value) {
            setProp("on_change", value);
        }

        public String getOnGetText() {
            return asText(getProp("on_get_text"));
        }

        public void setOnGetText(String value) {
            setProp("on_get_text", value);
        }

        public String getOnSetText() {
            return asText(getProp("on_set_text"));
        }

        public void setOnSetText(String value) {
            setProp("on_set_text", value);
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

        public GroupBoxStyle getStyle() {
            return GroupBoxStyle.of(getProp("style"));
        }

        public void setStyle(GroupBoxStyle value) {
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

        public boolean isZoomable() {
            return asBool(getProp("zoomable"));
        }

        public void setZoomable(boolean value) {
            setProp("zoomable", value);
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

        public ImageStyle getStyle() {
            return ImageStyle.of(getProp("style"));
        }

        public void setStyle(ImageStyle value) {
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

        public SplitterStyle getStyle() {
            return SplitterStyle.of(getProp("style"));
        }

        public void setStyle(SplitterStyle value) {
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

        public Aylis.Color getColor() {
            return Aylis.Color.fromJson(getProp("color"));
        }

        public void setColor(Aylis.Color value) {
            setProp("color", value.toJson());
        }

        public ImageFit getFit() {
            return ImageFit.of(getProp("fit"));
        }

        public void setFit(ImageFit value) {
            setProp("fit", value.value);
        }

        public java.util.List<String> getHighlight() {
            return asList(getProp("highlight"));
        }

        public void setHighlight(java.util.List<String> value) {
            setProp("highlight", listText(value));
        }

        public Aylis.Color getHighlightColor() {
            return Aylis.Color.fromJson(getProp("highlight_color"));
        }

        public void setHighlightColor(Aylis.Color value) {
            setProp("highlight_color", value.toJson());
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

        public SvgStyle getStyle() {
            return SvgStyle.of(getProp("style"));
        }

        public void setStyle(SvgStyle value) {
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

        /** O ClientDataSet deste form com o nome. */
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

        /** O DBChart deste form com o nome. */
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

        /** O DataGrid deste form com o nome. */
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

        /** O DataSource deste form com o nome. */
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

        /** O Field deste form com o nome. */
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

        /** O GroupBox deste form com o nome. */
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

        /** O Image deste form com o nome. */
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

        /** O Label deste form com o nome. */
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

        /** O Layout deste form com o nome. */
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

        /** O Panel deste form com o nome. */
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

        /** O Splitter deste form com o nome. */
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

        /** O Svg deste form com o nome. */
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
