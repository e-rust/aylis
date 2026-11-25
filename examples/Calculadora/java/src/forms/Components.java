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
    public enum CursorConfine {
        NONE("none"),
        WINDOW("window"),
        MONITOR("monitor");

        public final String value;

        CursorConfine(String value) {
            this.value = value;
        }

        public static CursorConfine of(Object v) {
            for (CursorConfine e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<CursorConfine> listOf(Object v) {
            java.util.List<CursorConfine> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (CursorConfine e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<CursorConfine> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (CursorConfine e : values) out.add(e.value);
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
    public enum FormBlendCompositor {
        PROJECT("project"),
        DIRECT2D("direct2d"),
        DIRECT3D("direct3d");

        public final String value;

        FormBlendCompositor(String value) {
            this.value = value;
        }

        public static FormBlendCompositor of(Object v) {
            for (FormBlendCompositor e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FormBlendCompositor> listOf(Object v) {
            java.util.List<FormBlendCompositor> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FormBlendCompositor e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FormBlendCompositor> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FormBlendCompositor e : values) out.add(e.value);
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
    public enum FullScreenMode {
        BORDERLESS("borderless"),
        EXCLUSIVE("exclusive");

        public final String value;

        FullScreenMode(String value) {
            this.value = value;
        }

        public static FullScreenMode of(Object v) {
            for (FullScreenMode e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<FullScreenMode> listOf(Object v) {
            java.util.List<FullScreenMode> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (FullScreenMode e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<FullScreenMode> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (FullScreenMode e : values) out.add(e.value);
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
    public enum KeepAwake {
        OFF("off"),
        SYSTEM("system"),
        DISPLAY("display");

        public final String value;

        KeepAwake(String value) {
            this.value = value;
        }

        public static KeepAwake of(Object v) {
            for (KeepAwake e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<KeepAwake> listOf(Object v) {
            java.util.List<KeepAwake> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (KeepAwake e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<KeepAwake> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (KeepAwake e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum KeyCode {
        UNIDENTIFIED("Unidentified"),
        KEY_A("KeyA"),
        KEY_B("KeyB"),
        KEY_C("KeyC"),
        KEY_D("KeyD"),
        KEY_E("KeyE"),
        KEY_F("KeyF"),
        KEY_G("KeyG"),
        KEY_H("KeyH"),
        KEY_I("KeyI"),
        KEY_J("KeyJ"),
        KEY_K("KeyK"),
        KEY_L("KeyL"),
        KEY_M("KeyM"),
        KEY_N("KeyN"),
        KEY_O("KeyO"),
        KEY_P("KeyP"),
        KEY_Q("KeyQ"),
        KEY_R("KeyR"),
        KEY_S("KeyS"),
        KEY_T("KeyT"),
        KEY_U("KeyU"),
        KEY_V("KeyV"),
        KEY_W("KeyW"),
        KEY_X("KeyX"),
        KEY_Y("KeyY"),
        KEY_Z("KeyZ"),
        DIGIT0("Digit0"),
        DIGIT1("Digit1"),
        DIGIT2("Digit2"),
        DIGIT3("Digit3"),
        DIGIT4("Digit4"),
        DIGIT5("Digit5"),
        DIGIT6("Digit6"),
        DIGIT7("Digit7"),
        DIGIT8("Digit8"),
        DIGIT9("Digit9"),
        NUMPAD0("Numpad0"),
        NUMPAD1("Numpad1"),
        NUMPAD2("Numpad2"),
        NUMPAD3("Numpad3"),
        NUMPAD4("Numpad4"),
        NUMPAD5("Numpad5"),
        NUMPAD6("Numpad6"),
        NUMPAD7("Numpad7"),
        NUMPAD8("Numpad8"),
        NUMPAD9("Numpad9"),
        F1("F1"),
        F2("F2"),
        F3("F3"),
        F4("F4"),
        F5("F5"),
        F6("F6"),
        F7("F7"),
        F8("F8"),
        F9("F9"),
        F10("F10"),
        F11("F11"),
        F12("F12"),
        F13("F13"),
        F14("F14"),
        F15("F15"),
        F16("F16"),
        F17("F17"),
        F18("F18"),
        F19("F19"),
        F20("F20"),
        F21("F21"),
        F22("F22"),
        F23("F23"),
        F24("F24"),
        F25("F25"),
        F26("F26"),
        F27("F27"),
        F28("F28"),
        F29("F29"),
        F30("F30"),
        F31("F31"),
        F32("F32"),
        F33("F33"),
        F34("F34"),
        F35("F35"),
        LANG1("Lang1"),
        LANG2("Lang2"),
        LANG3("Lang3"),
        LANG4("Lang4"),
        LANG5("Lang5"),
        BACKQUOTE("Backquote"),
        BACKSLASH("Backslash"),
        BRACKET_LEFT("BracketLeft"),
        BRACKET_RIGHT("BracketRight"),
        COMMA("Comma"),
        EQUAL("Equal"),
        INTL_BACKSLASH("IntlBackslash"),
        INTL_RO("IntlRo"),
        INTL_YEN("IntlYen"),
        MINUS("Minus"),
        PERIOD("Period"),
        QUOTE("Quote"),
        SEMICOLON("Semicolon"),
        SLASH("Slash"),
        ALT_LEFT("AltLeft"),
        ALT_RIGHT("AltRight"),
        BACKSPACE("Backspace"),
        CAPS_LOCK("CapsLock"),
        CONTEXT_MENU("ContextMenu"),
        CONTROL_LEFT("ControlLeft"),
        CONTROL_RIGHT("ControlRight"),
        ENTER("Enter"),
        META_LEFT("MetaLeft"),
        META_RIGHT("MetaRight"),
        SHIFT_LEFT("ShiftLeft"),
        SHIFT_RIGHT("ShiftRight"),
        SPACE("Space"),
        TAB("Tab"),
        CONVERT("Convert"),
        KANA_MODE("KanaMode"),
        NON_CONVERT("NonConvert"),
        DELETE("Delete"),
        END("End"),
        HELP("Help"),
        HOME("Home"),
        INSERT("Insert"),
        PAGE_DOWN("PageDown"),
        PAGE_UP("PageUp"),
        ARROW_DOWN("ArrowDown"),
        ARROW_LEFT("ArrowLeft"),
        ARROW_RIGHT("ArrowRight"),
        ARROW_UP("ArrowUp"),
        NUM_LOCK("NumLock"),
        NUMPAD_ADD("NumpadAdd"),
        NUMPAD_COMMA("NumpadComma"),
        NUMPAD_DECIMAL("NumpadDecimal"),
        NUMPAD_DIVIDE("NumpadDivide"),
        NUMPAD_ENTER("NumpadEnter"),
        NUMPAD_EQUAL("NumpadEqual"),
        NUMPAD_MULTIPLY("NumpadMultiply"),
        NUMPAD_SUBTRACT("NumpadSubtract"),
        ESCAPE("Escape"),
        FN("Fn"),
        PRINT_SCREEN("PrintScreen"),
        SCROLL_LOCK("ScrollLock"),
        PAUSE("Pause"),
        AUDIO_VOLUME_DOWN("AudioVolumeDown"),
        AUDIO_VOLUME_MUTE("AudioVolumeMute"),
        AUDIO_VOLUME_UP("AudioVolumeUp"),
        MEDIA_PLAY_PAUSE("MediaPlayPause"),
        MEDIA_SELECT("MediaSelect"),
        MEDIA_STOP("MediaStop"),
        MEDIA_TRACK_NEXT("MediaTrackNext"),
        MEDIA_TRACK_PREVIOUS("MediaTrackPrevious"),
        BROWSER_BACK("BrowserBack"),
        BROWSER_FAVORITES("BrowserFavorites"),
        BROWSER_FORWARD("BrowserForward"),
        BROWSER_HOME("BrowserHome"),
        BROWSER_REFRESH("BrowserRefresh"),
        BROWSER_SEARCH("BrowserSearch"),
        BROWSER_STOP("BrowserStop"),
        LAUNCH_APP1("LaunchApp1"),
        LAUNCH_APP2("LaunchApp2"),
        LAUNCH_MAIL("LaunchMail"),
        POWER("Power"),
        SLEEP("Sleep"),
        WAKE_UP("WakeUp"),
        NUMPAD_BACKSPACE("NumpadBackspace"),
        NUMPAD_CLEAR("NumpadClear"),
        NUMPAD_CLEAR_ENTRY("NumpadClearEntry"),
        NUMPAD_HASH("NumpadHash"),
        NUMPAD_MEMORY_ADD("NumpadMemoryAdd"),
        NUMPAD_MEMORY_CLEAR("NumpadMemoryClear"),
        NUMPAD_MEMORY_RECALL("NumpadMemoryRecall"),
        NUMPAD_MEMORY_STORE("NumpadMemoryStore"),
        NUMPAD_MEMORY_SUBTRACT("NumpadMemorySubtract"),
        NUMPAD_PAREN_LEFT("NumpadParenLeft"),
        NUMPAD_PAREN_RIGHT("NumpadParenRight"),
        NUMPAD_STAR("NumpadStar"),
        FN_LOCK("FnLock"),
        HIRAGANA("Hiragana"),
        KATAKANA("Katakana"),
        AGAIN("Again"),
        COPY("Copy"),
        CUT("Cut"),
        FIND("Find"),
        OPEN("Open"),
        PASTE("Paste"),
        PROPS("Props"),
        SELECT("Select"),
        UNDO("Undo"),
        RESUME("Resume"),
        SUSPEND("Suspend");

        public final String value;

        KeyCode(String value) {
            this.value = value;
        }

        public static KeyCode of(Object v) {
            for (KeyCode e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<KeyCode> listOf(Object v) {
            java.util.List<KeyCode> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (KeyCode e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<KeyCode> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (KeyCode e : values) out.add(e.value);
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
    public enum TabControlStyle {
        DEFAULT("");

        public final String value;

        TabControlStyle(String value) {
            this.value = value;
        }

        public static TabControlStyle of(Object v) {
            for (TabControlStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TabControlStyle> listOf(Object v) {
            java.util.List<TabControlStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TabControlStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TabControlStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TabControlStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum TabItemStyle {
        DEFAULT("");

        public final String value;

        TabItemStyle(String value) {
            this.value = value;
        }

        public static TabItemStyle of(Object v) {
            for (TabItemStyle e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TabItemStyle> listOf(Object v) {
            java.util.List<TabItemStyle> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TabItemStyle e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TabItemStyle> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TabItemStyle e : values) out.add(e.value);
            return out;
        }
    }

    /** Os valores de um enum do acesso tipado. */
    public enum TabPosition {
        TOP("top"),
        BOTTOM("bottom"),
        LEFT("left"),
        RIGHT("right");

        public final String value;

        TabPosition(String value) {
            this.value = value;
        }

        public static TabPosition of(Object v) {
            for (TabPosition e : values()) if (e.value.equals(v)) return e;
            return values()[0];
        }

        public static java.util.List<TabPosition> listOf(Object v) {
            java.util.List<TabPosition> out = new java.util.ArrayList<>();
            if (v instanceof java.util.List<?> l) for (Object x : l) for (TabPosition e : values()) if (e.value.equals(x)) out.add(e);
            return out;
        }

        public static java.util.List<String> names(java.util.Collection<TabPosition> values) {
            java.util.List<String> out = new java.util.ArrayList<>();
            for (TabPosition e : values) out.add(e.value);
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
    public record KeyEvent(String key, boolean ctrl, boolean shift, boolean alt, boolean meta, boolean capsLock, boolean numLock, boolean scrollLock, KeyCode code, long scancode, String form, long formId) {
        static KeyEvent fromJson(Object v) {
            java.util.Map<?, ?> m = v instanceof java.util.Map<?, ?> map ? map : java.util.Map.of();
            return new KeyEvent(asText(m.get("key")), asBool(m.get("ctrl")), asBool(m.get("shift")), asBool(m.get("alt")), asBool(m.get("meta")), asBool(m.get("caps_lock")), asBool(m.get("num_lock")), asBool(m.get("scroll_lock")), KeyCode.of(m.get("code")), (long) asNumber(m.get("scancode")), asText(m.get("form")), (long) asNumber(m.get("form_id")));
        }

        java.util.Map<String, Object> toJson() {
            java.util.Map<String, Object> m = new java.util.LinkedHashMap<>();
            m.put("key", key);
            m.put("ctrl", ctrl);
            m.put("shift", shift);
            m.put("alt", alt);
            m.put("meta", meta);
            m.put("caps_lock", capsLock);
            m.put("num_lock", numLock);
            m.put("scroll_lock", scrollLock);
            m.put("code", code == null ? "" : code.value);
            m.put("scancode", scancode);
            m.put("form", form);
            m.put("form_id", formId);
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
            return new KeyEvent(v, ctrl, shift, alt, meta, capsLock, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withCtrl(boolean v) {
            return new KeyEvent(key, v, shift, alt, meta, capsLock, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withShift(boolean v) {
            return new KeyEvent(key, ctrl, v, alt, meta, capsLock, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withAlt(boolean v) {
            return new KeyEvent(key, ctrl, shift, v, meta, capsLock, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withMeta(boolean v) {
            return new KeyEvent(key, ctrl, shift, alt, v, capsLock, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withCapsLock(boolean v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, v, numLock, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withNumLock(boolean v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, v, scrollLock, code, scancode, form, formId);
        }

        public KeyEvent withScrollLock(boolean v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, numLock, v, code, scancode, form, formId);
        }

        public KeyEvent withCode(KeyCode v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, numLock, scrollLock, v, scancode, form, formId);
        }

        public KeyEvent withScancode(long v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, numLock, scrollLock, code, v, form, formId);
        }

        public KeyEvent withForm(String v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, numLock, scrollLock, code, scancode, v, formId);
        }

        public KeyEvent withFormId(long v) {
            return new KeyEvent(key, ctrl, shift, alt, meta, capsLock, numLock, scrollLock, code, scancode, form, v);
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

        public boolean isBottommost() {
            return asBool(getProp("bottommost"));
        }

        public void setBottommost(boolean value) {
            setProp("bottommost", value);
        }

        public KeepAwake getKeepAwake() {
            return KeepAwake.of(getProp("keep_awake"));
        }

        public void setKeepAwake(KeepAwake value) {
            setProp("keep_awake", value.value);
        }

        public CursorConfine getConfineCursor() {
            return CursorConfine.of(getProp("confine_cursor"));
        }

        public void setConfineCursor(CursorConfine value) {
            setProp("confine_cursor", value.value);
        }

        public double getWindowOpacity() {
            return asNumber(getProp("window_opacity"));
        }

        public void setWindowOpacity(double value) {
            setProp("window_opacity", value);
        }

        public FullScreenMode getFullScreenMode() {
            return FullScreenMode.of(getProp("full_screen_mode"));
        }

        public void setFullScreenMode(FullScreenMode value) {
            setProp("full_screen_mode", value.value);
        }

        public long getVideoWidth() {
            return (long) asNumber(getProp("video_width"));
        }

        public void setVideoWidth(long value) {
            setProp("video_width", value);
        }

        public long getVideoHeight() {
            return (long) asNumber(getProp("video_height"));
        }

        public void setVideoHeight(long value) {
            setProp("video_height", value);
        }

        public double getVideoRefresh() {
            return asNumber(getProp("video_refresh"));
        }

        public void setVideoRefresh(double value) {
            setProp("video_refresh", value);
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

        public FormBlendCompositor getBlendCompositor() {
            return FormBlendCompositor.of(getProp("blend_compositor"));
        }

        public void setBlendCompositor(FormBlendCompositor value) {
            setProp("blend_compositor", value.value);
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

        public Aylis.AylisComponent getHost() {
            return getRef("host");
        }

        public void setHost(Aylis.AylisComponent value) {
            setRef("host", value);
        }

        public boolean isDockable() {
            return asBool(getProp("dockable"));
        }

        public void setDockable(boolean value) {
            setProp("dockable", value);
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

        public String getOnDock() {
            return asText(getProp("on_dock"));
        }

        public void setOnDock(String value) {
            setProp("on_dock", value);
        }

        public String getOnUndock() {
            return asText(getProp("on_undock"));
        }

        public void setOnUndock(String value) {
            setProp("on_undock", value);
        }

        public String getOnFilesDrop() {
            return asText(getProp("on_files_drop"));
        }

        public void setOnFilesDrop(String value) {
            setProp("on_files_drop", value);
        }

        public String getOnViewChange() {
            return asText(getProp("on_view_change"));
        }

        public void setOnViewChange(String value) {
            setProp("on_view_change", value);
        }

        public String getOnTopmostLost() {
            return asText(getProp("on_topmost_lost"));
        }

        public void setOnTopmostLost(String value) {
            setProp("on_topmost_lost", value);
        }

        public String getOnScaleChange() {
            return asText(getProp("on_scale_change"));
        }

        public void setOnScaleChange(String value) {
            setProp("on_scale_change", value);
        }

        public String getOnResizeEnd() {
            return asText(getProp("on_resize_end"));
        }

        public void setOnResizeEnd(String value) {
            setProp("on_resize_end", value);
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

        public void requestAttention(boolean critical) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "request_attention", "args", java.util.List.of(critical)));
        }

        public void bringToFront() {
            aylis.send("invoke", Aylis.obj("target", id, "method", "bring_to_front", "args", java.util.List.of()));
        }

        public void setActivationToken(String token) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "set_activation_token", "args", java.util.List.of(token)));
        }

        public double windowScale() {
            return asNumber(aylis.call("invoke", Aylis.obj("target", id, "method", "window_scale", "args", java.util.List.of())));
        }

        public boolean pointerOver() {
            return asBool(aylis.call("invoke", Aylis.obj("target", id, "method", "pointer_over", "args", java.util.List.of())));
        }

        public void place(long monitor, long x, long y, long width, long height, boolean activate) {
            aylis.send("invoke", Aylis.obj("target", id, "method", "place", "args", java.util.List.of(monitor, x, y, width, height, activate)));
        }

        public String gpuAdapter() {
            return asText(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapter", "args", java.util.List.of())));
        }

        public java.util.List<String> gpuAdapters() {
            return asList(aylis.call("invoke", Aylis.obj("target", id, "method", "gpu_adapters", "args", java.util.List.of())));
        }

        public double dropX() {
            return asNumber(aylis.call("invoke", Aylis.obj("target", id, "method", "drop_x", "args", java.util.List.of())));
        }

        public double dropY() {
            return asNumber(aylis.call("invoke", Aylis.obj("target", id, "method", "drop_y", "args", java.util.List.of())));
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

        public long getMaxLines() {
            return (long) asNumber(getProp("max_lines"));
        }

        public void setMaxLines(long value) {
            setProp("max_lines", value);
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

        public Aylis.Color getHoverFill() {
            return Aylis.Color.fromJson(getProp("hover_fill"));
        }

        public void setHoverFill(Aylis.Color value) {
            setProp("hover_fill", value.toJson());
        }

        public Aylis.Color getPressedFill() {
            return Aylis.Color.fromJson(getProp("pressed_fill"));
        }

        public void setPressedFill(Aylis.Color value) {
            setProp("pressed_fill", value.toJson());
        }

        public boolean isInteractive() {
            return asBool(getProp("interactive"));
        }

        public void setInteractive(boolean value) {
            setProp("interactive", value);
        }

        public Aylis.Color getSelectedFill() {
            return Aylis.Color.fromJson(getProp("selected_fill"));
        }

        public void setSelectedFill(Aylis.Color value) {
            setProp("selected_fill", value.toJson());
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

        public java.util.List<Double> getStrokeDash() {
            return asNumbers(getProp("stroke_dash"));
        }

        public void setStrokeDash(java.util.List<Double> value) {
            setProp("stroke_dash", listText(value));
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

        public Transition getTransition() {
            return Transition.of(getProp("transition"));
        }

        public void setTransition(Transition value) {
            setProp("transition", value.value);
        }

        public boolean isSwipe() {
            return asBool(getProp("swipe"));
        }

        public void setSwipe(boolean value) {
            setProp("swipe", value);
        }

        public TabPosition getTabPosition() {
            return TabPosition.of(getProp("tab_position"));
        }

        public void setTabPosition(TabPosition value) {
            setProp("tab_position", value.value);
        }

        public double getTabWidth() {
            return asNumber(getProp("tab_width"));
        }

        public void setTabWidth(double value) {
            setProp("tab_width", value);
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

        public TabControlStyle getStyle() {
            return TabControlStyle.of(getProp("style"));
        }

        public void setStyle(TabControlStyle value) {
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

        public String getOnShow() {
            return asText(getProp("on_show"));
        }

        public void setOnShow(String value) {
            setProp("on_show", value);
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

        public TabItemStyle getStyle() {
            return TabItemStyle.of(getProp("style"));
        }

        public void setStyle(TabItemStyle value) {
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

        /** O Button deste form com o nome. */
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

        /** O GridLayout deste form com o nome. */
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

        /** O Label deste form com o nome. */
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

        /** O Layout deste form com o nome. */
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

        /** O ListView deste form com o nome. */
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

        /** O Panel deste form com o nome. */
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

        /** O TabControl deste form com o nome. */
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

        /** O TabItem deste form com o nome. */
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
