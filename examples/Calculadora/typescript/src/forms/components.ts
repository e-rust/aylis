// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { ComponentInfo, AylisEvent } from "../aylis.ts";
import { AylisComponent, AylisForm, Color, Bounds, Shadow, Easing, Point, registerFormClass } from "../aylis.ts";

/** Uma data do calendário (sem fuso). */
export class LocalDate {
    readonly year: number;
    readonly month: number;
    readonly day: number;
    constructor(year: number, month: number, day: number) {
        this.year = year;
        this.month = month;
        this.day = day;
    }
    /** `2026-10-24` (o que não é data: `null`). */
    static fromJson(value: unknown): LocalDate | null {
        const m = /^(\d{4})-(\d{2})-(\d{2})/.exec(typeof value === "string" ? value : "");
        return m ? new LocalDate(+m[1], +m[2], +m[3]) : null;
    }
    toString(): string {
        return `${String(this.year).padStart(4, "0")}-${String(this.month).padStart(2, "0")}-${String(this.day).padStart(2, "0")}`;
    }
}

/** Uma hora do dia (sem fuso). */
export class LocalTime {
    readonly hour: number;
    readonly minute: number;
    readonly second: number;
    constructor(hour: number, minute: number, second = 0) {
        this.hour = hour;
        this.minute = minute;
        this.second = second;
    }
    /** `14:30` ou `14:30:15` (o que não é hora: `null`). */
    static fromJson(value: unknown): LocalTime | null {
        const m = /^(\d{1,2}):(\d{2})(?::(\d{2}))?/.exec(typeof value === "string" ? value : "");
        return m ? new LocalTime(+m[1], +m[2], m[3] ? +m[3] : 0) : null;
    }
    toString(): string {
        const p = (n: number) => String(n).padStart(2, "0");
        return `${p(this.hour)}:${p(this.minute)}${this.second ? ":" + p(this.second) : ""}`;
    }
}

/** O dia e a hora (sem fuso). */
export class LocalDateTime {
    readonly date: LocalDate;
    readonly time: LocalTime;
    constructor(date: LocalDate, time: LocalTime) {
        this.date = date;
        this.time = time;
    }
    /** `2026-11-12T14:30` (o que não é: `null`). */
    static fromJson(value: unknown): LocalDateTime | null {
        const [d, t] = (typeof value === "string" ? value : "").split(/[T ]/);
        const date = LocalDate.fromJson(d);
        return date ? new LocalDateTime(date, LocalTime.fromJson(t) ?? new LocalTime(0, 0)) : null;
    }
    toString(): string {
        return `${this.date}T${this.time}`;
    }
}

export function listOf(value: unknown): string[] {
    return (typeof value === "string" ? value : "").split(";").map((s) => s.trim()).filter((s) => s !== "");
}

export function numbersOf(value: unknown): number[] {
    return listOf(value).map(Number).filter((n) => !Number.isNaN(n));
}

export function colorsOf(value: unknown): Color[] {
    return listOf(value).filter((s) => s.startsWith("#")).map((s) => Color.hex(s));
}

/** O intervalo de datas (o `on_change` do DateRangePicker); `null`: sem a data. */
export interface DateRange {
    start: LocalDate | null;
    end: LocalDate | null;
}

export function dateRangeOf(value: unknown): DateRange {
    const [s, e] = (String(value ?? "") + ";").split(";");
    return { start: LocalDate.fromJson(s.trim()), end: LocalDate.fromJson(e.trim()) };
}

/** Os valores de um enum do acesso tipado. */
export type Align = "none" | "top" | "bottom" | "left" | "right" | "most_top" | "most_bottom" | "most_left" | "most_right" | "client" | "contents" | "center" | "vert_center" | "horz_center" | "horizontal" | "vertical" | "fit";

/** Os valores de um enum do acesso tipado. */
export type Anchor = "left" | "top" | "right" | "bottom";

/** Os valores de um enum do acesso tipado. */
export type BlendMode = "normal" | "multiply" | "screen" | "overlay" | "darken" | "lighten" | "color_dodge" | "color_burn" | "hard_light" | "soft_light" | "difference" | "exclusion" | "hue" | "saturation" | "color" | "luminosity";

/** Os valores de um enum do acesso tipado. */
export type BorderStyle = "sizeable" | "single" | "none" | "custom";

/** Os valores de um enum do acesso tipado. */
export type ButtonStyle = "" | "accent" | "danger" | "elevated" | "fab" | "fab_extended" | "flat" | "ghost" | "glass" | "glass_prominent" | "glossy" | "icon" | "icon_filled" | "icon_outlined" | "icon_tonal" | "info" | "link" | "negative" | "outline" | "pill" | "positive" | "push" | "reveal" | "round" | "rounded" | "secondary" | "success" | "text" | "tonal" | "unelevated" | "warning";

/** Os valores de um enum do acesso tipado. */
export type CloseAction = "hide" | "free";

/** Os valores de um enum do acesso tipado. */
export type CursorConfine = "none" | "window" | "monitor";

/** Os valores de um enum do acesso tipado. */
export type CursorShape = "default" | "pointer" | "text" | "wait" | "crosshair" | "move" | "not_allowed" | "resize_horizontal" | "resize_vertical" | "resize_ne_sw" | "resize_nw_se" | "grab" | "grabbing" | "none";

/** Os valores de um enum do acesso tipado. */
export type FontWeight = "default" | "thin" | "light" | "normal" | "medium" | "semibold" | "bold" | "black";

/** Os valores de um enum do acesso tipado. */
export type FormBlendCompositor = "project" | "direct2d" | "direct3d";

/** Os valores de um enum do acesso tipado. */
export type FormOrientation = "project" | "any" | "portrait" | "landscape" | "sensor_portrait" | "sensor_landscape";

/** Os valores de um enum do acesso tipado. */
export type FormPosition = "center" | "screen_center" | "designed" | "default" | "top_left" | "top_right" | "bottom_left" | "bottom_right";

/** Os valores de um enum do acesso tipado. */
export type FormPresent = "project" | "window" | "composition" | "composed";

/** Os valores de um enum do acesso tipado. */
export type FormStyle = "";

/** Os valores de um enum do acesso tipado. */
export type FullScreenMode = "borderless" | "exclusive";

/** Os valores de um enum do acesso tipado. */
export type GridLayoutStyle = "";

/** Os valores de um enum do acesso tipado. */
export type IconPosition = "left" | "right" | "top";

/** Os valores de um enum do acesso tipado. */
export type KeepAwake = "off" | "system" | "display";

/** Os valores de um enum do acesso tipado. */
export type KeyCode = "Unidentified" | "KeyA" | "KeyB" | "KeyC" | "KeyD" | "KeyE" | "KeyF" | "KeyG" | "KeyH" | "KeyI" | "KeyJ" | "KeyK" | "KeyL" | "KeyM" | "KeyN" | "KeyO" | "KeyP" | "KeyQ" | "KeyR" | "KeyS" | "KeyT" | "KeyU" | "KeyV" | "KeyW" | "KeyX" | "KeyY" | "KeyZ" | "Digit0" | "Digit1" | "Digit2" | "Digit3" | "Digit4" | "Digit5" | "Digit6" | "Digit7" | "Digit8" | "Digit9" | "Numpad0" | "Numpad1" | "Numpad2" | "Numpad3" | "Numpad4" | "Numpad5" | "Numpad6" | "Numpad7" | "Numpad8" | "Numpad9" | "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12" | "F13" | "F14" | "F15" | "F16" | "F17" | "F18" | "F19" | "F20" | "F21" | "F22" | "F23" | "F24" | "F25" | "F26" | "F27" | "F28" | "F29" | "F30" | "F31" | "F32" | "F33" | "F34" | "F35" | "Lang1" | "Lang2" | "Lang3" | "Lang4" | "Lang5" | "Backquote" | "Backslash" | "BracketLeft" | "BracketRight" | "Comma" | "Equal" | "IntlBackslash" | "IntlRo" | "IntlYen" | "Minus" | "Period" | "Quote" | "Semicolon" | "Slash" | "AltLeft" | "AltRight" | "Backspace" | "CapsLock" | "ContextMenu" | "ControlLeft" | "ControlRight" | "Enter" | "MetaLeft" | "MetaRight" | "ShiftLeft" | "ShiftRight" | "Space" | "Tab" | "Convert" | "KanaMode" | "NonConvert" | "Delete" | "End" | "Help" | "Home" | "Insert" | "PageDown" | "PageUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "ArrowUp" | "NumLock" | "NumpadAdd" | "NumpadComma" | "NumpadDecimal" | "NumpadDivide" | "NumpadEnter" | "NumpadEqual" | "NumpadMultiply" | "NumpadSubtract" | "Escape" | "Fn" | "PrintScreen" | "ScrollLock" | "Pause" | "AudioVolumeDown" | "AudioVolumeMute" | "AudioVolumeUp" | "MediaPlayPause" | "MediaSelect" | "MediaStop" | "MediaTrackNext" | "MediaTrackPrevious" | "BrowserBack" | "BrowserFavorites" | "BrowserForward" | "BrowserHome" | "BrowserRefresh" | "BrowserSearch" | "BrowserStop" | "LaunchApp1" | "LaunchApp2" | "LaunchMail" | "Power" | "Sleep" | "WakeUp" | "NumpadBackspace" | "NumpadClear" | "NumpadClearEntry" | "NumpadHash" | "NumpadMemoryAdd" | "NumpadMemoryClear" | "NumpadMemoryRecall" | "NumpadMemoryStore" | "NumpadMemorySubtract" | "NumpadParenLeft" | "NumpadParenRight" | "NumpadStar" | "FnLock" | "Hiragana" | "Katakana" | "Again" | "Copy" | "Cut" | "Find" | "Open" | "Paste" | "Props" | "Select" | "Undo" | "Resume" | "Suspend";

/** Os valores de um enum do acesso tipado. */
export type LabelStyle = "" | "caption" | "error" | "heading" | "kbd" | "link" | "mono" | "muted" | "title";

/** Os valores de um enum do acesso tipado. */
export type LayerCaching = "project" | "on" | "off";

/** Os valores de um enum do acesso tipado. */
export type LayoutStyle = "";

/** Os valores de um enum do acesso tipado. */
export type ListViewStyle = "" | "flat";

/** Os valores de um enum do acesso tipado. */
export type ModalResult = "none" | "ok" | "cancel" | "abort" | "retry" | "ignore" | "yes" | "no" | "close";

/** Os valores de um enum do acesso tipado. */
export type PanelStyle = "" | "appbar" | "card" | "filled" | "flat" | "glass" | "outlined" | "overlay" | "reveal" | "sidebar";

/** Os valores de um enum do acesso tipado. */
export type SelfAlign = "stretch" | "start" | "center" | "end";

/** Os valores de um enum do acesso tipado. */
export type TabControlStyle = "";

/** Os valores de um enum do acesso tipado. */
export type TabItemStyle = "";

/** Os valores de um enum do acesso tipado. */
export type TabPosition = "top" | "bottom" | "left" | "right";

/** Os valores de um enum do acesso tipado. */
export type TextAlign = "leading" | "center" | "trailing" | "justify";

/** Os valores de um enum do acesso tipado. */
export type TextCase = "none" | "upper" | "lower";

/** Os valores de um enum do acesso tipado. */
export type ThemeName = "" | "Aylis Light" | "Aylis Dark" | "Material 3 Light" | "Material 3 Dark" | "Material 2 Light" | "Material 2 Dark" | "Android 7 Light" | "Android 7 Dark" | "iOS 26 Light" | "iOS 26 Dark" | "iOS 13 Light" | "iOS 13 Dark" | "iOS 7 Light" | "WinUI 3 Light" | "WinUI 3 Dark" | "Windows 11 Light" | "Windows 11 Dark" | "Windows 10 Light" | "Windows 10 Dark" | "Win32" | "Windows 7" | "Windows Phone Dark" | "Windows Phone Light" | "Vuetify 3 Light" | "Vuetify 3 Dark" | "Bootstrap 5 Light" | "Bootstrap 5 Dark" | "shadcn/ui Light" | "shadcn/ui Dark" | "Ant Design 5 Light" | "Ant Design 5 Dark" | "Chakra UI Light" | "Chakra UI Dark" | "Quasar 2 Light" | "Quasar 2 Dark";

/** Os valores de um enum do acesso tipado. */
export type Transition = "none" | "fade" | "slide_left" | "slide_right" | "slide_up" | "slide_down";

/** Os valores de um enum do acesso tipado. */
export type Trimming = "none" | "character" | "word";

/** Os valores de um enum do acesso tipado. */
export type WindowCorners = "default" | "round" | "small" | "square";

/** Os valores de um enum do acesso tipado. */
export type WindowShape = "rectangle" | "rounded" | "ellipse" | "path";

/** Os valores de um enum do acesso tipado. */
export type WindowState = "normal" | "maximized" | "minimized" | "full_screen";

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface GestureEvent {
    phase: string;
    x: number;
    y: number;
    scale: number;
    deltaScale: number;
    rotation: number;
    deltaRotation: number;
    panX: number;
    panY: number;
}

export function gestureEventFromJson(v: any): GestureEvent {
    return {
        phase: String(v?.phase ?? ""),
        x: Number(v?.x ?? 0),
        y: Number(v?.y ?? 0),
        scale: Number(v?.scale ?? 0),
        deltaScale: Number(v?.delta_scale ?? 0),
        rotation: Number(v?.rotation ?? 0),
        deltaRotation: Number(v?.delta_rotation ?? 0),
        panX: Number(v?.pan_x ?? 0),
        panY: Number(v?.pan_y ?? 0),
    };
}

export function gestureEventToJson(r: GestureEvent): any {
    return {
        phase: r.phase,
        x: r.x,
        y: r.y,
        scale: r.scale,
        delta_scale: r.deltaScale,
        rotation: r.rotation,
        delta_rotation: r.deltaRotation,
        pan_x: r.panX,
        pan_y: r.panY,
    };
}

/** Um GestureEvent com os campos dados (o resto, o padrão). */
export function newGestureEvent(init: Partial<GestureEvent> = {}): GestureEvent {
    return { ...gestureEventFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface KeyEvent {
    key: string;
    ctrl: boolean;
    shift: boolean;
    alt: boolean;
    meta: boolean;
    capsLock: boolean;
    numLock: boolean;
    scrollLock: boolean;
    code: KeyCode;
    scancode: number;
    form: string;
    formId: number;
}

export function keyEventFromJson(v: any): KeyEvent {
    return {
        key: String(v?.key ?? ""),
        ctrl: Boolean(v?.ctrl),
        shift: Boolean(v?.shift),
        alt: Boolean(v?.alt),
        meta: Boolean(v?.meta),
        capsLock: Boolean(v?.caps_lock),
        numLock: Boolean(v?.num_lock),
        scrollLock: Boolean(v?.scroll_lock),
        code: (v?.code ?? "") as KeyCode,
        scancode: Number(v?.scancode ?? 0),
        form: String(v?.form ?? ""),
        formId: Number(v?.form_id ?? 0),
    };
}

export function keyEventToJson(r: KeyEvent): any {
    return {
        key: r.key,
        ctrl: r.ctrl,
        shift: r.shift,
        alt: r.alt,
        meta: r.meta,
        caps_lock: r.capsLock,
        num_lock: r.numLock,
        scroll_lock: r.scrollLock,
        code: r.code,
        scancode: r.scancode,
        form: r.form,
        form_id: r.formId,
    };
}

/** Um KeyEvent com os campos dados (o resto, o padrão). */
export function newKeyEvent(init: Partial<KeyEvent> = {}): KeyEvent {
    return { ...keyEventFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface ListViewItem {
    title: string;
    detail: string;
}

export function listViewItemFromJson(v: any): ListViewItem {
    return {
        title: String(v?.title ?? ""),
        detail: String(v?.detail ?? ""),
    };
}

export function listViewItemToJson(r: ListViewItem): any {
    return {
        title: r.title,
        detail: r.detail,
    };
}

/** Um ListViewItem com os campos dados (o resto, o padrão). */
export function newListViewItem(init: Partial<ListViewItem> = {}): ListViewItem {
    return { ...listViewItemFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface PointerEvent {
    x: number;
    y: number;
    button: string;
    clicks: number;
}

export function pointerEventFromJson(v: any): PointerEvent {
    return {
        x: Number(v?.x ?? 0),
        y: Number(v?.y ?? 0),
        button: String(v?.button ?? ""),
        clicks: Number(v?.clicks ?? 0),
    };
}

export function pointerEventToJson(r: PointerEvent): any {
    return {
        x: r.x,
        y: r.y,
        button: r.button,
        clicks: r.clicks,
    };
}

/** Um PointerEvent com os campos dados (o resto, o padrão). */
export function newPointerEvent(init: Partial<PointerEvent> = {}): PointerEvent {
    return { ...pointerEventFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface SwipeEvent {
    direction: string;
    x: number;
    y: number;
    dx: number;
    dy: number;
    vx: number;
    vy: number;
}

export function swipeEventFromJson(v: any): SwipeEvent {
    return {
        direction: String(v?.direction ?? ""),
        x: Number(v?.x ?? 0),
        y: Number(v?.y ?? 0),
        dx: Number(v?.dx ?? 0),
        dy: Number(v?.dy ?? 0),
        vx: Number(v?.vx ?? 0),
        vy: Number(v?.vy ?? 0),
    };
}

export function swipeEventToJson(r: SwipeEvent): any {
    return {
        direction: r.direction,
        x: r.x,
        y: r.y,
        dx: r.dx,
        dy: r.dy,
        vx: r.vx,
        vy: r.vy,
    };
}

/** Um SwipeEvent com os campos dados (o resto, o padrão). */
export function newSwipeEvent(init: Partial<SwipeEvent> = {}): SwipeEvent {
    return { ...swipeEventFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface WheelEvent {
    x: number;
    y: number;
    dx: number;
    dy: number;
}

export function wheelEventFromJson(v: any): WheelEvent {
    return {
        x: Number(v?.x ?? 0),
        y: Number(v?.y ?? 0),
        dx: Number(v?.dx ?? 0),
        dy: Number(v?.dy ?? 0),
    };
}

export function wheelEventToJson(r: WheelEvent): any {
    return {
        x: r.x,
        y: r.y,
        dx: r.dx,
        dy: r.dy,
    };
}

/** Um WheelEvent com os campos dados (o resto, o padrão). */
export function newWheelEvent(init: Partial<WheelEvent> = {}): WheelEvent {
    return { ...wheelEventFromJson({}), ...init };
}

/** Um Button de um form. */
export class Button extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text") as Promise<any>;
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get icon(): Promise<string> {
        return this.getProp("icon") as Promise<any>;
    }
    set icon(value: string) {
        this.setProp("icon", value);
    }
    get checkedIcon(): Promise<string> {
        return this.getProp("checked_icon") as Promise<any>;
    }
    set checkedIcon(value: string) {
        this.setProp("checked_icon", value);
    }
    get iconPosition(): Promise<IconPosition> {
        return this.getProp("icon_position") as Promise<any>;
    }
    set iconPosition(value: IconPosition) {
        this.setProp("icon_position", value);
    }
    get iconSize(): Promise<number> {
        return this.getProp("icon_size") as Promise<any>;
    }
    set iconSize(value: number) {
        this.setProp("icon_size", value);
    }
    get checkable(): Promise<boolean> {
        return this.getProp("checkable") as Promise<any>;
    }
    set checkable(value: boolean) {
        this.setProp("checkable", value);
    }
    get checked(): Promise<boolean> {
        return this.getProp("checked") as Promise<any>;
    }
    set checked(value: boolean) {
        this.setProp("checked", value);
    }
    get modalResult(): Promise<ModalResult> {
        return this.getProp("modal_result") as Promise<any>;
    }
    set modalResult(value: ModalResult) {
        this.setProp("modal_result", value);
    }
    get repeatEnter(): Promise<boolean> {
        return this.getProp("repeat_enter") as Promise<any>;
    }
    set repeatEnter(value: boolean) {
        this.setProp("repeat_enter", value);
    }
    get defaultButton(): Promise<boolean> {
        return this.getProp("default_button") as Promise<any>;
    }
    set defaultButton(value: boolean) {
        this.setProp("default_button", value);
    }
    get cancelButton(): Promise<boolean> {
        return this.getProp("cancel_button") as Promise<any>;
    }
    set cancelButton(value: boolean) {
        this.setProp("cancel_button", value);
    }
    get hAlign(): Promise<TextAlign> {
        return this.getProp("h_align") as Promise<any>;
    }
    set hAlign(value: TextAlign) {
        this.setProp("h_align", value);
    }
    get groupIndex(): Promise<number> {
        return this.getProp("group_index") as Promise<any>;
    }
    set groupIndex(value: number) {
        this.setProp("group_index", value);
    }
    get radius(): Promise<Bounds> {
        return this.getProp("radius").then(Bounds.fromJson);
    }
    set radius(value: Bounds) {
        this.setProp("radius", value.toJson());
    }
    get onChange(): Promise<string> {
        return this.getProp("on_change") as Promise<any>;
    }
    set onChange(value: string) {
        this.setProp("on_change", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<ButtonStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: ButtonStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
    setFocus() {
        this.aylis.send("focus", { target: this.id });
    }
}

/** Um Form de um form. */
export class Form extends AylisForm {
    get caption(): Promise<string> {
        return this.getProp("caption") as Promise<any>;
    }
    set caption(value: string) {
        this.setProp("caption", value);
    }
    get position(): Promise<FormPosition> {
        return this.getProp("position") as Promise<any>;
    }
    set position(value: FormPosition) {
        this.setProp("position", value);
    }
    get borderStyle(): Promise<BorderStyle> {
        return this.getProp("border_style") as Promise<any>;
    }
    set borderStyle(value: BorderStyle) {
        this.setProp("border_style", value);
    }
    get windowCorners(): Promise<WindowCorners> {
        return this.getProp("window_corners") as Promise<any>;
    }
    set windowCorners(value: WindowCorners) {
        this.setProp("window_corners", value);
    }
    get transparent(): Promise<boolean> {
        return this.getProp("transparent") as Promise<any>;
    }
    set transparent(value: boolean) {
        this.setProp("transparent", value);
    }
    get windowShape(): Promise<WindowShape> {
        return this.getProp("window_shape") as Promise<any>;
    }
    set windowShape(value: WindowShape) {
        this.setProp("window_shape", value);
    }
    get shapeRadius(): Promise<number> {
        return this.getProp("shape_radius") as Promise<any>;
    }
    set shapeRadius(value: number) {
        this.setProp("shape_radius", value);
    }
    get shapeRadii(): Promise<Bounds> {
        return this.getProp("shape_radii").then(Bounds.fromJson);
    }
    set shapeRadii(value: Bounds) {
        this.setProp("shape_radii", value.toJson());
    }
    get shapePath(): Promise<string> {
        return this.getProp("shape_path") as Promise<any>;
    }
    set shapePath(value: string) {
        this.setProp("shape_path", value);
    }
    get windowState(): Promise<WindowState> {
        return this.getProp("window_state") as Promise<any>;
    }
    set windowState(value: WindowState) {
        this.setProp("window_state", value);
    }
    get transition(): Promise<Transition> {
        return this.getProp("transition") as Promise<any>;
    }
    set transition(value: Transition) {
        this.setProp("transition", value);
    }
    get layerCache(): Promise<LayerCaching> {
        return this.getProp("layer_cache") as Promise<any>;
    }
    set layerCache(value: LayerCaching) {
        this.setProp("layer_cache", value);
    }
    get frameReuseKb(): Promise<number> {
        return this.getProp("frame_reuse_kb") as Promise<any>;
    }
    set frameReuseKb(value: number) {
        this.setProp("frame_reuse_kb", value);
    }
    get topmost(): Promise<boolean> {
        return this.getProp("topmost") as Promise<any>;
    }
    set topmost(value: boolean) {
        this.setProp("topmost", value);
    }
    get bottommost(): Promise<boolean> {
        return this.getProp("bottommost") as Promise<any>;
    }
    set bottommost(value: boolean) {
        this.setProp("bottommost", value);
    }
    get keepAwake(): Promise<KeepAwake> {
        return this.getProp("keep_awake") as Promise<any>;
    }
    set keepAwake(value: KeepAwake) {
        this.setProp("keep_awake", value);
    }
    get confineCursor(): Promise<CursorConfine> {
        return this.getProp("confine_cursor") as Promise<any>;
    }
    set confineCursor(value: CursorConfine) {
        this.setProp("confine_cursor", value);
    }
    get windowOpacity(): Promise<number> {
        return this.getProp("window_opacity") as Promise<any>;
    }
    set windowOpacity(value: number) {
        this.setProp("window_opacity", value);
    }
    get fullScreenMode(): Promise<FullScreenMode> {
        return this.getProp("full_screen_mode") as Promise<any>;
    }
    set fullScreenMode(value: FullScreenMode) {
        this.setProp("full_screen_mode", value);
    }
    get videoWidth(): Promise<number> {
        return this.getProp("video_width") as Promise<any>;
    }
    set videoWidth(value: number) {
        this.setProp("video_width", value);
    }
    get videoHeight(): Promise<number> {
        return this.getProp("video_height") as Promise<any>;
    }
    set videoHeight(value: number) {
        this.setProp("video_height", value);
    }
    get videoRefresh(): Promise<number> {
        return this.getProp("video_refresh") as Promise<any>;
    }
    set videoRefresh(value: number) {
        this.setProp("video_refresh", value);
    }
    get showInTaskbar(): Promise<boolean> {
        return this.getProp("show_in_taskbar") as Promise<any>;
    }
    set showInTaskbar(value: boolean) {
        this.setProp("show_in_taskbar", value);
    }
    get activeControl(): Promise<AylisComponent | null> {
        return this.getRef("active_control");
    }
    set activeControl(value: AylisComponent | null) {
        this.setRef("active_control", value);
    }
    get tabOrder(): Promise<string[]> {
        return this.getProp("tab_order").then(listOf);
    }
    set tabOrder(value: string[]) {
        this.setProp("tab_order", value.join(";"));
    }
    get tabKeys(): Promise<string> {
        return this.getProp("tab_keys") as Promise<any>;
    }
    set tabKeys(value: string) {
        this.setProp("tab_keys", value);
    }
    get backTabKeys(): Promise<string> {
        return this.getProp("back_tab_keys") as Promise<any>;
    }
    set backTabKeys(value: string) {
        this.setProp("back_tab_keys", value);
    }
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars") as Promise<any>;
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars") as Promise<any>;
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed") as Promise<any>;
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override") as Promise<any>;
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map") as Promise<any>;
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get keyPreview(): Promise<boolean> {
        return this.getProp("key_preview") as Promise<any>;
    }
    set keyPreview(value: boolean) {
        this.setProp("key_preview", value);
    }
    get zoom(): Promise<number> {
        return this.getProp("zoom") as Promise<any>;
    }
    set zoom(value: number) {
        this.setProp("zoom", value);
    }
    get monitor(): Promise<number> {
        return this.getProp("monitor") as Promise<any>;
    }
    set monitor(value: number) {
        this.setProp("monitor", value);
    }
    get rememberPlacement(): Promise<boolean> {
        return this.getProp("remember_placement") as Promise<any>;
    }
    set rememberPlacement(value: boolean) {
        this.setProp("remember_placement", value);
    }
    get gpu(): Promise<string> {
        return this.getProp("gpu") as Promise<any>;
    }
    set gpu(value: string) {
        this.setProp("gpu", value);
    }
    get present(): Promise<FormPresent> {
        return this.getProp("present") as Promise<any>;
    }
    set present(value: FormPresent) {
        this.setProp("present", value);
    }
    get blendCompositor(): Promise<FormBlendCompositor> {
        return this.getProp("blend_compositor") as Promise<any>;
    }
    set blendCompositor(value: FormBlendCompositor) {
        this.setProp("blend_compositor", value);
    }
    get spanMonitors(): Promise<boolean> {
        return this.getProp("span_monitors") as Promise<any>;
    }
    set spanMonitors(value: boolean) {
        this.setProp("span_monitors", value);
    }
    get physicalPixels(): Promise<boolean> {
        return this.getProp("physical_pixels") as Promise<any>;
    }
    set physicalPixels(value: boolean) {
        this.setProp("physical_pixels", value);
    }
    get activateOnShow(): Promise<boolean> {
        return this.getProp("activate_on_show") as Promise<any>;
    }
    set activateOnShow(value: boolean) {
        this.setProp("activate_on_show", value);
    }
    get forceForeground(): Promise<boolean> {
        return this.getProp("force_foreground") as Promise<any>;
    }
    set forceForeground(value: boolean) {
        this.setProp("force_foreground", value);
    }
    get closeAction(): Promise<CloseAction> {
        return this.getProp("close_action") as Promise<any>;
    }
    set closeAction(value: CloseAction) {
        this.setProp("close_action", value);
    }
    get frameRate(): Promise<number> {
        return this.getProp("frame_rate") as Promise<any>;
    }
    set frameRate(value: number) {
        this.setProp("frame_rate", value);
    }
    get orientation(): Promise<FormOrientation> {
        return this.getProp("orientation") as Promise<any>;
    }
    set orientation(value: FormOrientation) {
        this.setProp("orientation", value);
    }
    get host(): Promise<AylisComponent | null> {
        return this.getRef("host");
    }
    set host(value: AylisComponent | null) {
        this.setRef("host", value);
    }
    get dockable(): Promise<boolean> {
        return this.getProp("dockable") as Promise<any>;
    }
    set dockable(value: boolean) {
        this.setProp("dockable", value);
    }
    get onShow(): Promise<string> {
        return this.getProp("on_show") as Promise<any>;
    }
    set onShow(value: string) {
        this.setProp("on_show", value);
    }
    get onWindowCreated(): Promise<string> {
        return this.getProp("on_window_created") as Promise<any>;
    }
    set onWindowCreated(value: string) {
        this.setProp("on_window_created", value);
    }
    get onClose(): Promise<string> {
        return this.getProp("on_close") as Promise<any>;
    }
    set onClose(value: string) {
        this.setProp("on_close", value);
    }
    get onActivate(): Promise<string> {
        return this.getProp("on_activate") as Promise<any>;
    }
    set onActivate(value: string) {
        this.setProp("on_activate", value);
    }
    get onDeactivate(): Promise<string> {
        return this.getProp("on_deactivate") as Promise<any>;
    }
    set onDeactivate(value: string) {
        this.setProp("on_deactivate", value);
    }
    get onMinimize(): Promise<string> {
        return this.getProp("on_minimize") as Promise<any>;
    }
    set onMinimize(value: string) {
        this.setProp("on_minimize", value);
    }
    get onRestore(): Promise<string> {
        return this.getProp("on_restore") as Promise<any>;
    }
    set onRestore(value: string) {
        this.setProp("on_restore", value);
    }
    get onNavigate(): Promise<string> {
        return this.getProp("on_navigate") as Promise<any>;
    }
    set onNavigate(value: string) {
        this.setProp("on_navigate", value);
    }
    get onGpuChange(): Promise<string> {
        return this.getProp("on_gpu_change") as Promise<any>;
    }
    set onGpuChange(value: string) {
        this.setProp("on_gpu_change", value);
    }
    get onDock(): Promise<string> {
        return this.getProp("on_dock") as Promise<any>;
    }
    set onDock(value: string) {
        this.setProp("on_dock", value);
    }
    get onUndock(): Promise<string> {
        return this.getProp("on_undock") as Promise<any>;
    }
    set onUndock(value: string) {
        this.setProp("on_undock", value);
    }
    get onFilesDrop(): Promise<string> {
        return this.getProp("on_files_drop") as Promise<any>;
    }
    set onFilesDrop(value: string) {
        this.setProp("on_files_drop", value);
    }
    get onViewChange(): Promise<string> {
        return this.getProp("on_view_change") as Promise<any>;
    }
    set onViewChange(value: string) {
        this.setProp("on_view_change", value);
    }
    get onTopmostLost(): Promise<string> {
        return this.getProp("on_topmost_lost") as Promise<any>;
    }
    set onTopmostLost(value: string) {
        this.setProp("on_topmost_lost", value);
    }
    get onScaleChange(): Promise<string> {
        return this.getProp("on_scale_change") as Promise<any>;
    }
    set onScaleChange(value: string) {
        this.setProp("on_scale_change", value);
    }
    get onResizeEnd(): Promise<string> {
        return this.getProp("on_resize_end") as Promise<any>;
    }
    set onResizeEnd(value: string) {
        this.setProp("on_resize_end", value);
    }
    get clientWidth(): Promise<number> {
        return this.getProp("client_width") as Promise<any>;
    }
    set clientWidth(value: number) {
        this.setProp("client_width", value);
    }
    get clientHeight(): Promise<number> {
        return this.getProp("client_height") as Promise<any>;
    }
    set clientHeight(value: number) {
        this.setProp("client_height", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<FormStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: FormStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
    moveToMonitor(monitor: number) {
        this.aylis.send("invoke", { target: this.id, method: "move_to_monitor", args: [monitor] });
    }
    requestAttention(critical: boolean) {
        this.aylis.send("invoke", { target: this.id, method: "request_attention", args: [critical] });
    }
    bringToFront() {
        this.aylis.send("invoke", { target: this.id, method: "bring_to_front", args: [] });
    }
    setActivationToken(token: string) {
        this.aylis.send("invoke", { target: this.id, method: "set_activation_token", args: [token] });
    }
    windowScale(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "window_scale", args: [] }) as Promise<any>;
    }
    pointerOver(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "pointer_over", args: [] }) as Promise<any>;
    }
    place(monitor: number, x: number, y: number, width: number, height: number, activate: boolean) {
        this.aylis.send("invoke", { target: this.id, method: "place", args: [monitor, x, y, width, height, activate] });
    }
    gpuAdapter(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapter", args: [] }) as Promise<any>;
    }
    gpuAdapters(): Promise<string[]> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapters", args: [] }).then(listOf);
    }
    dropX(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "drop_x", args: [] }) as Promise<any>;
    }
    dropY(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "drop_y", args: [] }) as Promise<any>;
    }
    renderRgba(x: number, y: number, width: number, height: number, scale: number): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "render_rgba", args: [x, y, width, height, scale] }) as Promise<any>;
    }
    focused(): Promise<string> {
        return this.aylis.call("focused", { target: this.id }) as Promise<any>;
    }
    focusNext() {
        this.aylis.send("focus_next", { target: this.id });
    }
    focusPrev() {
        this.aylis.send("focus_prev", { target: this.id });
    }
}

/** Um GridLayout de um form. */
export class GridLayout extends AylisComponent {
    get columns(): Promise<number> {
        return this.getProp("columns") as Promise<any>;
    }
    set columns(value: number) {
        this.setProp("columns", value);
    }
    get minColumnWidth(): Promise<number> {
        return this.getProp("min_column_width") as Promise<any>;
    }
    set minColumnWidth(value: number) {
        this.setProp("min_column_width", value);
    }
    get gap(): Promise<number> {
        return this.getProp("gap") as Promise<any>;
    }
    set gap(value: number) {
        this.setProp("gap", value);
    }
    get rowHeight(): Promise<number> {
        return this.getProp("row_height") as Promise<any>;
    }
    set rowHeight(value: number) {
        this.setProp("row_height", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<GridLayoutStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: GridLayoutStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
}

/** Um Label de um form. */
export class Label extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text") as Promise<any>;
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get hAlign(): Promise<TextAlign> {
        return this.getProp("h_align") as Promise<any>;
    }
    set hAlign(value: TextAlign) {
        this.setProp("h_align", value);
    }
    get vAlign(): Promise<TextAlign> {
        return this.getProp("v_align") as Promise<any>;
    }
    set vAlign(value: TextAlign) {
        this.setProp("v_align", value);
    }
    get wrap(): Promise<boolean> {
        return this.getProp("wrap") as Promise<any>;
    }
    set wrap(value: boolean) {
        this.setProp("wrap", value);
    }
    get trimming(): Promise<Trimming> {
        return this.getProp("trimming") as Promise<any>;
    }
    set trimming(value: Trimming) {
        this.setProp("trimming", value);
    }
    get bold(): Promise<boolean> {
        return this.getProp("bold") as Promise<any>;
    }
    set bold(value: boolean) {
        this.setProp("bold", value);
    }
    get color(): Promise<Color> {
        return this.getProp("color").then(Color.fromJson);
    }
    set color(value: Color) {
        this.setProp("color", value.toJson());
    }
    get highlight(): Promise<string> {
        return this.getProp("highlight") as Promise<any>;
    }
    set highlight(value: string) {
        this.setProp("highlight", value);
    }
    get letterSpacing(): Promise<number> {
        return this.getProp("letter_spacing") as Promise<any>;
    }
    set letterSpacing(value: number) {
        this.setProp("letter_spacing", value);
    }
    get textCase(): Promise<TextCase> {
        return this.getProp("text_case") as Promise<any>;
    }
    set textCase(value: TextCase) {
        this.setProp("text_case", value);
    }
    get markup(): Promise<boolean> {
        return this.getProp("markup") as Promise<any>;
    }
    set markup(value: boolean) {
        this.setProp("markup", value);
    }
    get outline(): Promise<Color> {
        return this.getProp("outline").then(Color.fromJson);
    }
    set outline(value: Color) {
        this.setProp("outline", value.toJson());
    }
    get outlineWidth(): Promise<number> {
        return this.getProp("outline_width") as Promise<any>;
    }
    set outlineWidth(value: number) {
        this.setProp("outline_width", value);
    }
    get lineHeight(): Promise<number> {
        return this.getProp("line_height") as Promise<any>;
    }
    set lineHeight(value: number) {
        this.setProp("line_height", value);
    }
    get textShadow(): Promise<Shadow | null> {
        return this.getProp("text_shadow").then(Shadow.fromJson);
    }
    set textShadow(value: Shadow | null) {
        this.setProp("text_shadow", (value ? value.toJson() : ""));
    }
    get maxLines(): Promise<number> {
        return this.getProp("max_lines") as Promise<any>;
    }
    set maxLines(value: number) {
        this.setProp("max_lines", value);
    }
    get focusControl(): Promise<AylisComponent | null> {
        return this.getRef("focus_control");
    }
    set focusControl(value: AylisComponent | null) {
        this.setRef("focus_control", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<LabelStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: LabelStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
    minContentWidth(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "min_content_width", args: [] }) as Promise<any>;
    }
    maxContentWidth(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "max_content_width", args: [] }) as Promise<any>;
    }
}

/** Um Layout de um form. */
export class Layout extends AylisComponent {
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars") as Promise<any>;
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars") as Promise<any>;
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed") as Promise<any>;
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override") as Promise<any>;
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map") as Promise<any>;
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get gap(): Promise<number> {
        return this.getProp("gap") as Promise<any>;
    }
    set gap(value: number) {
        this.setProp("gap", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<LayoutStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: LayoutStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
}

/** Um ListView de um form. */
export class ListView extends AylisComponent {
    get items(): Promise<ListViewItem[]> {
        return this.getProp("items").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(listViewItemFromJson));
    }
    set items(value: ListViewItem[]) {
        this.setProp("items", JSON.stringify(value.map(listViewItemToJson)));
    }
    get itemIndex(): Promise<number> {
        return this.getProp("item_index") as Promise<any>;
    }
    set itemIndex(value: number) {
        this.setProp("item_index", value);
    }
    get itemHeight(): Promise<number> {
        return this.getProp("item_height") as Promise<any>;
    }
    set itemHeight(value: number) {
        this.setProp("item_height", value);
    }
    get accessory(): Promise<boolean> {
        return this.getProp("accessory") as Promise<any>;
    }
    set accessory(value: boolean) {
        this.setProp("accessory", value);
    }
    get onChange(): Promise<string> {
        return this.getProp("on_change") as Promise<any>;
    }
    set onChange(value: string) {
        this.setProp("on_change", value);
    }
    get onItemClick(): Promise<string> {
        return this.getProp("on_item_click") as Promise<any>;
    }
    set onItemClick(value: string) {
        this.setProp("on_item_click", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<ListViewStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: ListViewStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
    setFocus() {
        this.aylis.send("focus", { target: this.id });
    }
}

/** Um Panel de um form. */
export class Panel extends AylisComponent {
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars") as Promise<any>;
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars") as Promise<any>;
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed") as Promise<any>;
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override") as Promise<any>;
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map") as Promise<any>;
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get gap(): Promise<number> {
        return this.getProp("gap") as Promise<any>;
    }
    set gap(value: number) {
        this.setProp("gap", value);
    }
    get hoverFill(): Promise<Color> {
        return this.getProp("hover_fill").then(Color.fromJson);
    }
    set hoverFill(value: Color) {
        this.setProp("hover_fill", value.toJson());
    }
    get pressedFill(): Promise<Color> {
        return this.getProp("pressed_fill").then(Color.fromJson);
    }
    set pressedFill(value: Color) {
        this.setProp("pressed_fill", value.toJson());
    }
    get interactive(): Promise<boolean> {
        return this.getProp("interactive") as Promise<any>;
    }
    set interactive(value: boolean) {
        this.setProp("interactive", value);
    }
    get selectedFill(): Promise<Color> {
        return this.getProp("selected_fill").then(Color.fromJson);
    }
    set selectedFill(value: Color) {
        this.setProp("selected_fill", value.toJson());
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get fill2(): Promise<Color> {
        return this.getProp("fill2").then(Color.fromJson);
    }
    set fill2(value: Color) {
        this.setProp("fill2", value.toJson());
    }
    get fillMid(): Promise<Color> {
        return this.getProp("fill_mid").then(Color.fromJson);
    }
    set fillMid(value: Color) {
        this.setProp("fill_mid", value.toJson());
    }
    get fillMidAt(): Promise<number> {
        return this.getProp("fill_mid_at") as Promise<any>;
    }
    set fillMidAt(value: number) {
        this.setProp("fill_mid_at", value);
    }
    get gradientAngle(): Promise<number> {
        return this.getProp("gradient_angle") as Promise<any>;
    }
    set gradientAngle(value: number) {
        this.setProp("gradient_angle", value);
    }
    get gradientRadial(): Promise<boolean> {
        return this.getProp("gradient_radial") as Promise<any>;
    }
    set gradientRadial(value: boolean) {
        this.setProp("gradient_radial", value);
    }
    get stroke(): Promise<Color> {
        return this.getProp("stroke").then(Color.fromJson);
    }
    set stroke(value: Color) {
        this.setProp("stroke", value.toJson());
    }
    get strokeWidth(): Promise<number> {
        return this.getProp("stroke_width") as Promise<any>;
    }
    set strokeWidth(value: number) {
        this.setProp("stroke_width", value);
    }
    get strokeDash(): Promise<number[]> {
        return this.getProp("stroke_dash").then(numbersOf);
    }
    set strokeDash(value: number[]) {
        this.setProp("stroke_dash", value.join(";"));
    }
    get radius(): Promise<Bounds> {
        return this.getProp("radius").then(Bounds.fromJson);
    }
    set radius(value: Bounds) {
        this.setProp("radius", value.toJson());
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<PanelStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: PanelStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
}

/** Um TabControl de um form. */
export class TabControl extends AylisComponent {
    get tabIndex(): Promise<number> {
        return this.getProp("tab_index") as Promise<any>;
    }
    set tabIndex(value: number) {
        this.setProp("tab_index", value);
    }
    get showTabs(): Promise<boolean> {
        return this.getProp("show_tabs") as Promise<any>;
    }
    set showTabs(value: boolean) {
        this.setProp("show_tabs", value);
    }
    get transition(): Promise<Transition> {
        return this.getProp("transition") as Promise<any>;
    }
    set transition(value: Transition) {
        this.setProp("transition", value);
    }
    get swipe(): Promise<boolean> {
        return this.getProp("swipe") as Promise<any>;
    }
    set swipe(value: boolean) {
        this.setProp("swipe", value);
    }
    get tabPosition(): Promise<TabPosition> {
        return this.getProp("tab_position") as Promise<any>;
    }
    set tabPosition(value: TabPosition) {
        this.setProp("tab_position", value);
    }
    get tabWidth(): Promise<number> {
        return this.getProp("tab_width") as Promise<any>;
    }
    set tabWidth(value: number) {
        this.setProp("tab_width", value);
    }
    get onChange(): Promise<string> {
        return this.getProp("on_change") as Promise<any>;
    }
    set onChange(value: string) {
        this.setProp("on_change", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<TabControlStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: TabControlStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
    setFocus() {
        this.aylis.send("focus", { target: this.id });
    }
}

/** Um TabItem de um form. */
export class TabItem extends AylisComponent {
    get caption(): Promise<string> {
        return this.getProp("caption") as Promise<any>;
    }
    set caption(value: string) {
        this.setProp("caption", value);
    }
    get onShow(): Promise<string> {
        return this.getProp("on_show") as Promise<any>;
    }
    set onShow(value: string) {
        this.setProp("on_show", value);
    }
    get left(): Promise<number> {
        return this.getProp("left") as Promise<any>;
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top") as Promise<any>;
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width") as Promise<any>;
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height") as Promise<any>;
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<Align> {
        return this.getProp("align") as Promise<any>;
    }
    set align(value: Align) {
        this.setProp("align", value);
    }
    get anchors(): Promise<Anchor[]> {
        return this.getProp("anchors").then((v) => (v ?? []) as any);
    }
    set anchors(value: Anchor[]) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<Bounds> {
        return this.getProp("margins").then(Bounds.fromJson);
    }
    set margins(value: Bounds) {
        this.setProp("margins", value.toJson());
    }
    get padding(): Promise<Bounds> {
        return this.getProp("padding").then(Bounds.fromJson);
    }
    set padding(value: Bounds) {
        this.setProp("padding", value.toJson());
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test") as Promise<any>;
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children") as Promise<any>;
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state") as Promise<any>;
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate") as Promise<any>;
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel") as Promise<any>;
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get capturesGesture(): Promise<boolean> {
        return this.getProp("captures_gesture") as Promise<any>;
    }
    set capturesGesture(value: boolean) {
        this.setProp("captures_gesture", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus") as Promise<any>;
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable") as Promise<any>;
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops") as Promise<any>;
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data") as Promise<any>;
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label") as Promise<any>;
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<AylisComponent | null> {
        return this.getRef("drag_image");
    }
    set dragImage(value: AylisComponent | null) {
        this.setRef("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types") as Promise<any>;
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop") as Promise<any>;
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<CursorShape> {
        return this.getProp("cursor") as Promise<any>;
    }
    set cursor(value: CursorShape) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity") as Promise<any>;
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get blendMode(): Promise<BlendMode> {
        return this.getProp("blend_mode") as Promise<any>;
    }
    set blendMode(value: BlendMode) {
        this.setProp("blend_mode", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer") as Promise<any>;
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation") as Promise<any>;
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x") as Promise<any>;
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y") as Promise<any>;
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio") as Promise<any>;
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width") as Promise<any>;
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width") as Promise<any>;
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height") as Promise<any>;
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height") as Promise<any>;
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<SelfAlign> {
        return this.getProp("self_align") as Promise<any>;
    }
    set selfAlign(value: SelfAlign) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size") as Promise<any>;
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span") as Promise<any>;
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get gridRow(): Promise<number> {
        return this.getProp("grid_row") as Promise<any>;
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column") as Promise<any>;
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span") as Promise<any>;
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow") as Promise<any>;
    }
    set grow(value: number) {
        this.setProp("grow", value);
    }
    get shadow(): Promise<Shadow | null> {
        return this.getProp("shadow").then(Shadow.fromJson);
    }
    set shadow(value: Shadow | null) {
        this.setProp("shadow", (value ? value.toJson() : ""));
    }
    get style(): Promise<TabItemStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: TabItemStyle) {
        this.setProp("style", value);
    }
    get theme(): Promise<ThemeName> {
        return this.getProp("theme") as Promise<any>;
    }
    set theme(value: ThemeName) {
        this.setProp("theme", value);
    }
    get fill(): Promise<Color> {
        return this.getProp("fill").then(Color.fromJson);
    }
    set fill(value: Color) {
        this.setProp("fill", value.toJson());
    }
    get borderColor(): Promise<Color> {
        return this.getProp("border_color").then(Color.fromJson);
    }
    set borderColor(value: Color) {
        this.setProp("border_color", value.toJson());
    }
    get textColor(): Promise<Color> {
        return this.getProp("text_color").then(Color.fromJson);
    }
    set textColor(value: Color) {
        this.setProp("text_color", value.toJson());
    }
    get accentColor(): Promise<Color> {
        return this.getProp("accent_color").then(Color.fromJson);
    }
    set accentColor(value: Color) {
        this.setProp("accent_color", value.toJson());
    }
    get successColor(): Promise<Color> {
        return this.getProp("success_color").then(Color.fromJson);
    }
    set successColor(value: Color) {
        this.setProp("success_color", value.toJson());
    }
    get warningColor(): Promise<Color> {
        return this.getProp("warning_color").then(Color.fromJson);
    }
    set warningColor(value: Color) {
        this.setProp("warning_color", value.toJson());
    }
    get dangerColor(): Promise<Color> {
        return this.getProp("danger_color").then(Color.fromJson);
    }
    set dangerColor(value: Color) {
        this.setProp("danger_color", value.toJson());
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family") as Promise<any>;
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size") as Promise<any>;
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<FontWeight> {
        return this.getProp("font_weight") as Promise<any>;
    }
    set fontWeight(value: FontWeight) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic") as Promise<any>;
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline") as Promise<any>;
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout") as Promise<any>;
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint") as Promise<any>;
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag") as Promise<any>;
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get tagString(): Promise<string> {
        return this.getProp("tag_string") as Promise<any>;
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name") as Promise<any>;
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<AylisComponent | null> {
        return this.getRef("popup_menu");
    }
    set popupMenu(value: AylisComponent | null) {
        this.setRef("popup_menu", value);
    }
    get onClick(): Promise<string> {
        return this.getProp("on_click") as Promise<any>;
    }
    set onClick(value: string) {
        this.setProp("on_click", value);
    }
    get onDblClick(): Promise<string> {
        return this.getProp("on_dbl_click") as Promise<any>;
    }
    set onDblClick(value: string) {
        this.setProp("on_dbl_click", value);
    }
    get onLongPress(): Promise<string> {
        return this.getProp("on_long_press") as Promise<any>;
    }
    set onLongPress(value: string) {
        this.setProp("on_long_press", value);
    }
    get onPointerDown(): Promise<string> {
        return this.getProp("on_pointer_down") as Promise<any>;
    }
    set onPointerDown(value: string) {
        this.setProp("on_pointer_down", value);
    }
    get onPointerUp(): Promise<string> {
        return this.getProp("on_pointer_up") as Promise<any>;
    }
    set onPointerUp(value: string) {
        this.setProp("on_pointer_up", value);
    }
    get onPointerMove(): Promise<string> {
        return this.getProp("on_pointer_move") as Promise<any>;
    }
    set onPointerMove(value: string) {
        this.setProp("on_pointer_move", value);
    }
    get onPointerEnter(): Promise<string> {
        return this.getProp("on_pointer_enter") as Promise<any>;
    }
    set onPointerEnter(value: string) {
        this.setProp("on_pointer_enter", value);
    }
    get onPointerLeave(): Promise<string> {
        return this.getProp("on_pointer_leave") as Promise<any>;
    }
    set onPointerLeave(value: string) {
        this.setProp("on_pointer_leave", value);
    }
    get onWheel(): Promise<string> {
        return this.getProp("on_wheel") as Promise<any>;
    }
    set onWheel(value: string) {
        this.setProp("on_wheel", value);
    }
    get onGesture(): Promise<string> {
        return this.getProp("on_gesture") as Promise<any>;
    }
    set onGesture(value: string) {
        this.setProp("on_gesture", value);
    }
    get onSwipe(): Promise<string> {
        return this.getProp("on_swipe") as Promise<any>;
    }
    set onSwipe(value: string) {
        this.setProp("on_swipe", value);
    }
    get onKeyUp(): Promise<string> {
        return this.getProp("on_key_up") as Promise<any>;
    }
    set onKeyUp(value: string) {
        this.setProp("on_key_up", value);
    }
    get onEnter(): Promise<string> {
        return this.getProp("on_enter") as Promise<any>;
    }
    set onEnter(value: string) {
        this.setProp("on_enter", value);
    }
    get onExit(): Promise<string> {
        return this.getProp("on_exit") as Promise<any>;
    }
    set onExit(value: string) {
        this.setProp("on_exit", value);
    }
    get onResize(): Promise<string> {
        return this.getProp("on_resize") as Promise<any>;
    }
    set onResize(value: string) {
        this.setProp("on_resize", value);
    }
    get onDragMove(): Promise<string> {
        return this.getProp("on_drag_move") as Promise<any>;
    }
    set onDragMove(value: string) {
        this.setProp("on_drag_move", value);
    }
    get onDragLeave(): Promise<string> {
        return this.getProp("on_drag_leave") as Promise<any>;
    }
    set onDragLeave(value: string) {
        this.setProp("on_drag_leave", value);
    }
    get onDragDrop(): Promise<string> {
        return this.getProp("on_drag_drop") as Promise<any>;
    }
    set onDragDrop(value: string) {
        this.setProp("on_drag_drop", value);
    }
    get onDragEnd(): Promise<string> {
        return this.getProp("on_drag_end") as Promise<any>;
    }
    set onDragEnd(value: string) {
        this.setProp("on_drag_end", value);
    }
}

/** O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmCalculadoraComponents extends Form {
    pnlLateral!: Panel;
    tabLateral!: TabControl;
    tabHistorico!: TabItem;
    btnLimparHistorico!: Button;
    lvHistorico!: ListView;
    tabMemoria!: TabItem;
    lvMemoria!: ListView;
    lytCorpo!: Layout;
    lytTopo!: Layout;
    btnHistorico!: Button;
    lblTitulo!: Label;
    lblExpressao!: Label;
    lblVisor!: Label;
    grdMemoria!: GridLayout;
    btnMC!: Button;
    btnMR!: Button;
    btnMMais!: Button;
    btnMMenos!: Button;
    btnMS!: Button;
    grdTeclado!: GridLayout;
    btnPorcento!: Button;
    btnCE!: Button;
    btnC!: Button;
    btnApagar!: Button;
    btnInverso!: Button;
    btnQuadrado!: Button;
    btnRaiz!: Button;
    btnDividir!: Button;
    btn7!: Button;
    btn8!: Button;
    btn9!: Button;
    btnMultiplicar!: Button;
    btn4!: Button;
    btn5!: Button;
    btn6!: Button;
    btnSubtrair!: Button;
    btn1!: Button;
    btn2!: Button;
    btn3!: Button;
    btnSomar!: Button;
    btnSinal!: Button;
    btn0!: Button;
    btnVirgula!: Button;
    btnIgual!: Button;
    bind(list: ComponentInfo[]) {
        this.pnlLateral = this.resolve(list, "pnlLateral", "Panel", Panel);
        this.tabLateral = this.resolve(list, "tabLateral", "TabControl", TabControl);
        this.tabHistorico = this.resolve(list, "tabHistorico", "TabItem", TabItem);
        this.btnLimparHistorico = this.resolve(list, "btnLimparHistorico", "Button", Button);
        this.lvHistorico = this.resolve(list, "lvHistorico", "ListView", ListView);
        this.tabMemoria = this.resolve(list, "tabMemoria", "TabItem", TabItem);
        this.lvMemoria = this.resolve(list, "lvMemoria", "ListView", ListView);
        this.lytCorpo = this.resolve(list, "lytCorpo", "Layout", Layout);
        this.lytTopo = this.resolve(list, "lytTopo", "Layout", Layout);
        this.btnHistorico = this.resolve(list, "btnHistorico", "Button", Button);
        this.lblTitulo = this.resolve(list, "lblTitulo", "Label", Label);
        this.lblExpressao = this.resolve(list, "lblExpressao", "Label", Label);
        this.lblVisor = this.resolve(list, "lblVisor", "Label", Label);
        this.grdMemoria = this.resolve(list, "grdMemoria", "GridLayout", GridLayout);
        this.btnMC = this.resolve(list, "btnMC", "Button", Button);
        this.btnMR = this.resolve(list, "btnMR", "Button", Button);
        this.btnMMais = this.resolve(list, "btnMMais", "Button", Button);
        this.btnMMenos = this.resolve(list, "btnMMenos", "Button", Button);
        this.btnMS = this.resolve(list, "btnMS", "Button", Button);
        this.grdTeclado = this.resolve(list, "grdTeclado", "GridLayout", GridLayout);
        this.btnPorcento = this.resolve(list, "btnPorcento", "Button", Button);
        this.btnCE = this.resolve(list, "btnCE", "Button", Button);
        this.btnC = this.resolve(list, "btnC", "Button", Button);
        this.btnApagar = this.resolve(list, "btnApagar", "Button", Button);
        this.btnInverso = this.resolve(list, "btnInverso", "Button", Button);
        this.btnQuadrado = this.resolve(list, "btnQuadrado", "Button", Button);
        this.btnRaiz = this.resolve(list, "btnRaiz", "Button", Button);
        this.btnDividir = this.resolve(list, "btnDividir", "Button", Button);
        this.btn7 = this.resolve(list, "btn7", "Button", Button);
        this.btn8 = this.resolve(list, "btn8", "Button", Button);
        this.btn9 = this.resolve(list, "btn9", "Button", Button);
        this.btnMultiplicar = this.resolve(list, "btnMultiplicar", "Button", Button);
        this.btn4 = this.resolve(list, "btn4", "Button", Button);
        this.btn5 = this.resolve(list, "btn5", "Button", Button);
        this.btn6 = this.resolve(list, "btn6", "Button", Button);
        this.btnSubtrair = this.resolve(list, "btnSubtrair", "Button", Button);
        this.btn1 = this.resolve(list, "btn1", "Button", Button);
        this.btn2 = this.resolve(list, "btn2", "Button", Button);
        this.btn3 = this.resolve(list, "btn3", "Button", Button);
        this.btnSomar = this.resolve(list, "btnSomar", "Button", Button);
        this.btnSinal = this.resolve(list, "btnSinal", "Button", Button);
        this.btn0 = this.resolve(list, "btn0", "Button", Button);
        this.btnVirgula = this.resolve(list, "btnVirgula", "Button", Button);
        this.btnIgual = this.resolve(list, "btnIgual", "Button", Button);
    }
    /** O Button deste form com o nome. */
    findButton(name: string): Button | undefined {
        switch (name) {
            case "btnLimparHistorico": return this.btnLimparHistorico;
            case "btnHistorico": return this.btnHistorico;
            case "btnMC": return this.btnMC;
            case "btnMR": return this.btnMR;
            case "btnMMais": return this.btnMMais;
            case "btnMMenos": return this.btnMMenos;
            case "btnMS": return this.btnMS;
            case "btnPorcento": return this.btnPorcento;
            case "btnCE": return this.btnCE;
            case "btnC": return this.btnC;
            case "btnApagar": return this.btnApagar;
            case "btnInverso": return this.btnInverso;
            case "btnQuadrado": return this.btnQuadrado;
            case "btnRaiz": return this.btnRaiz;
            case "btnDividir": return this.btnDividir;
            case "btn7": return this.btn7;
            case "btn8": return this.btn8;
            case "btn9": return this.btn9;
            case "btnMultiplicar": return this.btnMultiplicar;
            case "btn4": return this.btn4;
            case "btn5": return this.btn5;
            case "btn6": return this.btn6;
            case "btnSubtrair": return this.btnSubtrair;
            case "btn1": return this.btn1;
            case "btn2": return this.btn2;
            case "btn3": return this.btn3;
            case "btnSomar": return this.btnSomar;
            case "btnSinal": return this.btnSinal;
            case "btn0": return this.btn0;
            case "btnVirgula": return this.btnVirgula;
            case "btnIgual": return this.btnIgual;
        }
        return undefined;
    }
    /** O Button que disparou o evento. */
    senderAsButton(ev: AylisEvent): Button | undefined {
        return this.findButton(ev.sender.name);
    }
    /** O GridLayout deste form com o nome. */
    findGridLayout(name: string): GridLayout | undefined {
        switch (name) {
            case "grdMemoria": return this.grdMemoria;
            case "grdTeclado": return this.grdTeclado;
        }
        return undefined;
    }
    /** O GridLayout que disparou o evento. */
    senderAsGridLayout(ev: AylisEvent): GridLayout | undefined {
        return this.findGridLayout(ev.sender.name);
    }
    /** O Label deste form com o nome. */
    findLabel(name: string): Label | undefined {
        switch (name) {
            case "lblTitulo": return this.lblTitulo;
            case "lblExpressao": return this.lblExpressao;
            case "lblVisor": return this.lblVisor;
        }
        return undefined;
    }
    /** O Label que disparou o evento. */
    senderAsLabel(ev: AylisEvent): Label | undefined {
        return this.findLabel(ev.sender.name);
    }
    /** O Layout deste form com o nome. */
    findLayout(name: string): Layout | undefined {
        switch (name) {
            case "lytCorpo": return this.lytCorpo;
            case "lytTopo": return this.lytTopo;
        }
        return undefined;
    }
    /** O Layout que disparou o evento. */
    senderAsLayout(ev: AylisEvent): Layout | undefined {
        return this.findLayout(ev.sender.name);
    }
    /** O ListView deste form com o nome. */
    findListView(name: string): ListView | undefined {
        switch (name) {
            case "lvHistorico": return this.lvHistorico;
            case "lvMemoria": return this.lvMemoria;
        }
        return undefined;
    }
    /** O ListView que disparou o evento. */
    senderAsListView(ev: AylisEvent): ListView | undefined {
        return this.findListView(ev.sender.name);
    }
    /** O Panel deste form com o nome. */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlLateral": return this.pnlLateral;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O TabControl deste form com o nome. */
    findTabControl(name: string): TabControl | undefined {
        switch (name) {
            case "tabLateral": return this.tabLateral;
        }
        return undefined;
    }
    /** O TabControl que disparou o evento. */
    senderAsTabControl(ev: AylisEvent): TabControl | undefined {
        return this.findTabControl(ev.sender.name);
    }
    /** O TabItem deste form com o nome. */
    findTabItem(name: string): TabItem | undefined {
        switch (name) {
            case "tabHistorico": return this.tabHistorico;
            case "tabMemoria": return this.tabMemoria;
        }
        return undefined;
    }
    /** O TabItem que disparou o evento. */
    senderAsTabItem(ev: AylisEvent): TabItem | undefined {
        return this.findTabItem(ev.sender.name);
    }
}

/** A janela de frmCalculadora cujo evento está rodando (a primeira, fora dos handlers). */
export let frmCalculadora: FrmCalculadoraComponents;
registerFormClass("frmCalculadora", FrmCalculadoraComponents, (form) => {
    frmCalculadora = form as FrmCalculadoraComponents;
});
