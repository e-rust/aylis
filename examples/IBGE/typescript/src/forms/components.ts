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
export type ApplyMode = "cached" | "on_post";

/** Os valores de um enum do acesso tipado. */
export type BlendMode = "normal" | "multiply" | "screen" | "overlay" | "darken" | "lighten" | "color_dodge" | "color_burn" | "hard_light" | "soft_light" | "difference" | "exclusion" | "hue" | "saturation" | "color" | "luminosity";

/** Os valores de um enum do acesso tipado. */
export type BorderStyle = "sizeable" | "single" | "none" | "custom";

/** Os valores de um enum do acesso tipado. */
export type CharCase = "normal" | "upper" | "lower";

/** Os valores de um enum do acesso tipado. */
export type ChartKind = "line" | "bar" | "area" | "pie" | "donut" | "sparkline" | "heatmap" | "scatter" | "stacked_bar" | "waterfall" | "funnel" | "treemap" | "radar" | "horizontal_bar" | "bubble" | "polar_area" | "stacked_line" | "stacked_area" | "mixed" | "sankey" | "waffle";

/** Os valores de um enum do acesso tipado. */
export type ChartSeriesFill = "none" | "origin" | "start" | "end" | "series";

/** Os valores de um enum do acesso tipado. */
export type ChartSeriesKind = "line" | "bar" | "area" | "scatter";

/** Os valores de um enum do acesso tipado. */
export type ChartSeriesPoint = "circle" | "square" | "triangle" | "diamond" | "cross" | "none";

/** Os valores de um enum do acesso tipado. */
export type CloseAction = "hide" | "free";

/** Os valores de um enum do acesso tipado. */
export type CursorShape = "default" | "pointer" | "text" | "wait" | "crosshair" | "move" | "not_allowed" | "resize_horizontal" | "resize_vertical" | "resize_ne_sw" | "resize_nw_se" | "grab" | "grabbing" | "none";

/** Os valores de um enum do acesso tipado. */
export type DBChartStyle = "";

/** Os valores de um enum do acesso tipado. */
export type DataGridStyle = "";

/** Os valores de um enum do acesso tipado. */
export type FieldKind = "data" | "calculated" | "lookup";

/** Os valores de um enum do acesso tipado. */
export type FieldType = "string" | "memo" | "integer" | "float" | "currency" | "boolean" | "date" | "time" | "datetime" | "blob" | "auto_inc" | "timestamp";

/** Os valores de um enum do acesso tipado. */
export type FontWeight = "default" | "thin" | "light" | "normal" | "medium" | "semibold" | "bold" | "black";

/** Os valores de um enum do acesso tipado. */
export type FormOrientation = "project" | "any" | "portrait" | "landscape" | "sensor_portrait" | "sensor_landscape";

/** Os valores de um enum do acesso tipado. */
export type FormPosition = "center" | "screen_center" | "designed" | "default" | "top_left" | "top_right" | "bottom_left" | "bottom_right";

/** Os valores de um enum do acesso tipado. */
export type FormPresent = "project" | "window" | "composition" | "composed";

/** Os valores de um enum do acesso tipado. */
export type FormStyle = "";

/** Os valores de um enum do acesso tipado. */
export type GridTotalFunction = "sum" | "avg" | "min" | "max" | "count";

/** Os valores de um enum do acesso tipado. */
export type GroupBoxStyle = "";

/** Os valores de um enum do acesso tipado. */
export type HorizontalWheel = "shift" | "always" | "none";

/** Os valores de um enum do acesso tipado. */
export type ImageFit = "contain" | "cover" | "stretch" | "center" | "original";

/** Os valores de um enum do acesso tipado. */
export type ImageStyle = "";

/** Os valores de um enum do acesso tipado. */
export type LabelStyle = "" | "caption" | "error" | "heading" | "kbd" | "link" | "mono" | "muted" | "title";

/** Os valores de um enum do acesso tipado. */
export type LayerCaching = "project" | "on" | "off";

/** Os valores de um enum do acesso tipado. */
export type LayoutStyle = "";

/** Os valores de um enum do acesso tipado. */
export type LegendPosition = "top_right" | "top_left" | "top" | "bottom";

/** Os valores de um enum do acesso tipado. */
export type Overscroll = "auto" | "glow" | "stretch" | "bounce" | "none";

/** Os valores de um enum do acesso tipado. */
export type PanelStyle = "" | "appbar" | "card" | "filled" | "flat" | "glass" | "outlined" | "overlay" | "reveal" | "sidebar";

/** Os valores de um enum do acesso tipado. */
export type Rounding = "half_up" | "half_even" | "truncate" | "up";

/** Os valores de um enum do acesso tipado. */
export type SelfAlign = "stretch" | "start" | "center" | "end";

/** Os valores de um enum do acesso tipado. */
export type SplitterStyle = "";

/** Os valores de um enum do acesso tipado. */
export type SvgStyle = "";

/** Os valores de um enum do acesso tipado. */
export type TextAlign = "leading" | "center" | "trailing" | "justify";

/** Os valores de um enum do acesso tipado. */
export type TextCase = "none" | "upper" | "lower";

/** Os valores de um enum do acesso tipado. */
export type ThemeName = "" | "Aylis Light" | "Aylis Dark" | "Material 3 Light" | "Material 3 Dark" | "Material 2 Light" | "Material 2 Dark" | "Android 7 Light" | "Android 7 Dark" | "iOS 26 Light" | "iOS 26 Dark" | "iOS 13 Light" | "iOS 13 Dark" | "iOS 7 Light" | "WinUI 3 Light" | "WinUI 3 Dark" | "Windows 11 Light" | "Windows 11 Dark" | "Windows 10 Light" | "Windows 10 Dark" | "Win32" | "Windows 7" | "Windows Phone Dark" | "Windows Phone Light" | "Vuetify 3 Light" | "Vuetify 3 Dark" | "Bootstrap 5 Light" | "Bootstrap 5 Dark" | "shadcn/ui Light" | "shadcn/ui Dark" | "Ant Design 5 Light" | "Ant Design 5 Dark" | "Chakra UI Light" | "Chakra UI Dark" | "Quasar 2 Light" | "Quasar 2 Dark";

/** Os valores de um enum do acesso tipado. */
export type TitleAlign = "left" | "center" | "right";

/** Os valores de um enum do acesso tipado. */
export type Transition = "none" | "fade" | "slide_left" | "slide_right" | "slide_up" | "slide_down";

/** Os valores de um enum do acesso tipado. */
export type Trimming = "none" | "character" | "word";

/** Os valores de um enum do acesso tipado. */
export type ValueFormat = "short" | "full" | "percent";

/** Os valores de um enum do acesso tipado. */
export type WindowCorners = "default" | "round" | "small" | "square";

/** Os valores de um enum do acesso tipado. */
export type WindowShape = "rectangle" | "rounded" | "ellipse" | "path";

/** Os valores de um enum do acesso tipado. */
export type WindowState = "normal" | "maximized" | "minimized" | "full_screen";

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface ChartSeries {
    name: string;
    values: number[];
    x: number[];
    radius: number[];
    kind: ChartSeriesKind;
    rightAxis: boolean;
    color: Color;
    width: number;
    dash: number[];
    point: ChartSeriesPoint;
    fill: ChartSeriesFill;
    fillSeries: number;
    hidden: boolean;
}

export function chartSeriesFromJson(v: any): ChartSeries {
    return {
        name: String(v?.name ?? ""),
        values: (v?.values ?? []).map(Number),
        x: (v?.x ?? []).map(Number),
        radius: (v?.radius ?? []).map(Number),
        kind: (v?.kind ?? "") as ChartSeriesKind,
        rightAxis: Boolean(v?.right_axis),
        color: Color.fromJson(v?.color),
        width: Number(v?.width ?? 0),
        dash: (v?.dash ?? []).map(Number),
        point: (v?.point ?? "") as ChartSeriesPoint,
        fill: (v?.fill ?? "") as ChartSeriesFill,
        fillSeries: Number(v?.fill_series ?? 0),
        hidden: Boolean(v?.hidden),
    };
}

export function chartSeriesToJson(r: ChartSeries): any {
    return {
        name: r.name,
        values: r.values,
        x: r.x,
        radius: r.radius,
        kind: r.kind,
        right_axis: r.rightAxis,
        color: r.color.toJson(),
        width: r.width,
        dash: r.dash,
        point: r.point,
        fill: r.fill,
        fill_series: r.fillSeries,
        hidden: r.hidden,
    };
}

/** Um ChartSeries com os campos dados (o resto, o padrão). */
export function newChartSeries(init: Partial<ChartSeries> = {}): ChartSeries {
    return { ...chartSeriesFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface ColumnSpec {
    field: string;
    title: string;
    width: number;
    readOnly: boolean;
    color: Color;
    textColor: Color;
}

export function columnSpecFromJson(v: any): ColumnSpec {
    return {
        field: String(v?.field ?? ""),
        title: String(v?.title ?? ""),
        width: Number(v?.width ?? 0),
        readOnly: Boolean(v?.read_only),
        color: Color.fromJson(v?.color),
        textColor: Color.fromJson(v?.text_color),
    };
}

export function columnSpecToJson(r: ColumnSpec): any {
    return {
        field: r.field,
        title: r.title,
        width: r.width,
        read_only: r.readOnly,
        color: r.color.toJson(),
        text_color: r.textColor.toJson(),
    };
}

/** Um ColumnSpec com os campos dados (o resto, o padrão). */
export function newColumnSpec(init: Partial<ColumnSpec> = {}): ColumnSpec {
    return { ...columnSpecFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface FilterParam {
    name: string;
    value: string;
}

export function filterParamFromJson(v: any): FilterParam {
    return {
        name: String(v?.name ?? ""),
        value: String(v?.value ?? ""),
    };
}

export function filterParamToJson(r: FilterParam): any {
    return {
        name: r.name,
        value: r.value,
    };
}

/** Um FilterParam com os campos dados (o resto, o padrão). */
export function newFilterParam(init: Partial<FilterParam> = {}): FilterParam {
    return { ...filterParamFromJson({}), ...init };
}

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
export interface GridTotal {
    field: string;
    function: GridTotalFunction;
}

export function gridTotalFromJson(v: any): GridTotal {
    return {
        field: String(v?.field ?? ""),
        function: (v?.function ?? "") as GridTotalFunction,
    };
}

export function gridTotalToJson(r: GridTotal): any {
    return {
        field: r.field,
        function: r.function,
    };
}

/** Um GridTotal com os campos dados (o resto, o padrão). */
export function newGridTotal(init: Partial<GridTotal> = {}): GridTotal {
    return { ...gridTotalFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface IndexDef {
    name: string;
    fields: string;
}

export function indexDefFromJson(v: any): IndexDef {
    return {
        name: String(v?.name ?? ""),
        fields: String(v?.fields ?? ""),
    };
}

export function indexDefToJson(r: IndexDef): any {
    return {
        name: r.name,
        fields: r.fields,
    };
}

/** Um IndexDef com os campos dados (o resto, o padrão). */
export function newIndexDef(init: Partial<IndexDef> = {}): IndexDef {
    return { ...indexDefFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface KeyEvent {
    key: string;
    ctrl: boolean;
    shift: boolean;
    alt: boolean;
}

export function keyEventFromJson(v: any): KeyEvent {
    return {
        key: String(v?.key ?? ""),
        ctrl: Boolean(v?.ctrl),
        shift: Boolean(v?.shift),
        alt: Boolean(v?.alt),
    };
}

export function keyEventToJson(r: KeyEvent): any {
    return {
        key: r.key,
        ctrl: r.ctrl,
        shift: r.shift,
        alt: r.alt,
    };
}

/** Um KeyEvent com os campos dados (o resto, o padrão). */
export function newKeyEvent(init: Partial<KeyEvent> = {}): KeyEvent {
    return { ...keyEventFromJson({}), ...init };
}

/** Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). */
export interface MasterField {
    field: string;
    masterField: string;
}

export function masterFieldFromJson(v: any): MasterField {
    return {
        field: String(v?.field ?? ""),
        masterField: String(v?.master_field ?? ""),
    };
}

export function masterFieldToJson(r: MasterField): any {
    return {
        field: r.field,
        master_field: r.masterField,
    };
}

/** Um MasterField com os campos dados (o resto, o padrão). */
export function newMasterField(init: Partial<MasterField> = {}): MasterField {
    return { ...masterFieldFromJson({}), ...init };
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

/** Um ClientDataSet de um form. */
export class ClientDataSet extends AylisComponent {
    get keyFields(): Promise<string> {
        return this.getProp("key_fields") as Promise<any>;
    }
    set keyFields(value: string) {
        this.setProp("key_fields", value);
    }
    get applyMode(): Promise<ApplyMode> {
        return this.getProp("apply_mode") as Promise<any>;
    }
    set applyMode(value: ApplyMode) {
        this.setProp("apply_mode", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only") as Promise<any>;
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get active(): Promise<boolean> {
        return this.getProp("active") as Promise<any>;
    }
    set active(value: boolean) {
        this.setProp("active", value);
    }
    get filter(): Promise<string> {
        return this.getProp("filter") as Promise<any>;
    }
    set filter(value: string) {
        this.setProp("filter", value);
    }
    get filtered(): Promise<boolean> {
        return this.getProp("filtered") as Promise<any>;
    }
    set filtered(value: boolean) {
        this.setProp("filtered", value);
    }
    get indexFieldNames(): Promise<string> {
        return this.getProp("index_field_names") as Promise<any>;
    }
    set indexFieldNames(value: string) {
        this.setProp("index_field_names", value);
    }
    get indexes(): Promise<IndexDef[]> {
        return this.getProp("indexes").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(indexDefFromJson));
    }
    set indexes(value: IndexDef[]) {
        this.setProp("indexes", JSON.stringify(value.map(indexDefToJson)));
    }
    get indexName(): Promise<string> {
        return this.getProp("index_name") as Promise<any>;
    }
    set indexName(value: string) {
        this.setProp("index_name", value);
    }
    get params(): Promise<FilterParam[]> {
        return this.getProp("params").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(filterParamFromJson));
    }
    set params(value: FilterParam[]) {
        this.setProp("params", JSON.stringify(value.map(filterParamToJson)));
    }
    get statusFilter(): Promise<string> {
        return this.getProp("status_filter") as Promise<any>;
    }
    set statusFilter(value: string) {
        this.setProp("status_filter", value);
    }
    get masterSource(): Promise<AylisComponent | null> {
        return this.getRef("master_source");
    }
    set masterSource(value: AylisComponent | null) {
        this.setRef("master_source", value);
    }
    get masterFields(): Promise<MasterField[]> {
        return this.getProp("master_fields").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(masterFieldFromJson));
    }
    set masterFields(value: MasterField[]) {
        this.setProp("master_fields", JSON.stringify(value.map(masterFieldToJson)));
    }
    get applySeparately(): Promise<boolean> {
        return this.getProp("apply_separately") as Promise<any>;
    }
    set applySeparately(value: boolean) {
        this.setProp("apply_separately", value);
    }
    get recordTotal(): Promise<number> {
        return this.getProp("record_total") as Promise<any>;
    }
    set recordTotal(value: number) {
        this.setProp("record_total", value);
    }
    get maxRecords(): Promise<number> {
        return this.getProp("max_records") as Promise<any>;
    }
    set maxRecords(value: number) {
        this.setProp("max_records", value);
    }
    get partialNotice(): Promise<string> {
        return this.getProp("partial_notice") as Promise<any>;
    }
    get onMasterChange(): Promise<string> {
        return this.getProp("on_master_change") as Promise<any>;
    }
    set onMasterChange(value: string) {
        this.setProp("on_master_change", value);
    }
    get afterScroll(): Promise<string> {
        return this.getProp("after_scroll") as Promise<any>;
    }
    set afterScroll(value: string) {
        this.setProp("after_scroll", value);
    }
    get afterPost(): Promise<string> {
        return this.getProp("after_post") as Promise<any>;
    }
    set afterPost(value: string) {
        this.setProp("after_post", value);
    }
    get onStateChange(): Promise<string> {
        return this.getProp("on_state_change") as Promise<any>;
    }
    set onStateChange(value: string) {
        this.setProp("on_state_change", value);
    }
    get onReconcileError(): Promise<string> {
        return this.getProp("on_reconcile_error") as Promise<any>;
    }
    set onReconcileError(value: string) {
        this.setProp("on_reconcile_error", value);
    }
    get onError(): Promise<string> {
        return this.getProp("on_error") as Promise<any>;
    }
    set onError(value: string) {
        this.setProp("on_error", value);
    }
    get onFetch(): Promise<string> {
        return this.getProp("on_fetch") as Promise<any>;
    }
    set onFetch(value: string) {
        this.setProp("on_fetch", value);
    }
    get onServerQuery(): Promise<string> {
        return this.getProp("on_server_query") as Promise<any>;
    }
    set onServerQuery(value: string) {
        this.setProp("on_server_query", value);
    }
}

/** Um DBChart de um form. */
export class DBChart extends AylisComponent {
    get dataSource(): Promise<AylisComponent | null> {
        return this.getRef("data_source");
    }
    set dataSource(value: AylisComponent | null) {
        this.setRef("data_source", value);
    }
    get labelField(): Promise<string> {
        return this.getProp("label_field") as Promise<any>;
    }
    set labelField(value: string) {
        this.setProp("label_field", value);
    }
    get valueFields(): Promise<string[]> {
        return this.getProp("value_fields").then(listOf);
    }
    set valueFields(value: string[]) {
        this.setProp("value_fields", value.join(";"));
    }
    get kind(): Promise<ChartKind> {
        return this.getProp("kind") as Promise<any>;
    }
    set kind(value: ChartKind) {
        this.setProp("kind", value);
    }
    get title(): Promise<string> {
        return this.getProp("title") as Promise<any>;
    }
    set title(value: string) {
        this.setProp("title", value);
    }
    get labels(): Promise<string[]> {
        return this.getProp("labels").then(listOf);
    }
    set labels(value: string[]) {
        this.setProp("labels", value.join(";"));
    }
    get series(): Promise<ChartSeries[]> {
        return this.getProp("series").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(chartSeriesFromJson));
    }
    set series(value: ChartSeries[]) {
        this.setProp("series", JSON.stringify(value.map(chartSeriesToJson)));
    }
    get json(): Promise<string> {
        return this.getProp("json") as Promise<any>;
    }
    set json(value: string) {
        this.setProp("json", value);
    }
    get legend(): Promise<boolean> {
        return this.getProp("legend") as Promise<any>;
    }
    set legend(value: boolean) {
        this.setProp("legend", value);
    }
    get stepped(): Promise<boolean> {
        return this.getProp("stepped") as Promise<any>;
    }
    set stepped(value: boolean) {
        this.setProp("stepped", value);
    }
    get tension(): Promise<number> {
        return this.getProp("tension") as Promise<any>;
    }
    set tension(value: number) {
        this.setProp("tension", value);
    }
    get animate(): Promise<boolean> {
        return this.getProp("animate") as Promise<any>;
    }
    set animate(value: boolean) {
        this.setProp("animate", value);
    }
    get zoom(): Promise<boolean> {
        return this.getProp("zoom") as Promise<any>;
    }
    set zoom(value: boolean) {
        this.setProp("zoom", value);
    }
    get autoScale(): Promise<boolean> {
        return this.getProp("auto_scale") as Promise<any>;
    }
    set autoScale(value: boolean) {
        this.setProp("auto_scale", value);
    }
    get yMin(): Promise<number> {
        return this.getProp("y_min") as Promise<any>;
    }
    set yMin(value: number) {
        this.setProp("y_min", value);
    }
    get yMax(): Promise<number> {
        return this.getProp("y_max") as Promise<any>;
    }
    set yMax(value: number) {
        this.setProp("y_max", value);
    }
    get logScale(): Promise<boolean> {
        return this.getProp("log_scale") as Promise<any>;
    }
    set logScale(value: boolean) {
        this.setProp("log_scale", value);
    }
    get reverse(): Promise<boolean> {
        return this.getProp("reverse") as Promise<any>;
    }
    set reverse(value: boolean) {
        this.setProp("reverse", value);
    }
    get timeAxis(): Promise<boolean> {
        return this.getProp("time_axis") as Promise<any>;
    }
    set timeAxis(value: boolean) {
        this.setProp("time_axis", value);
    }
    get clickedSeries(): Promise<number> {
        return this.getProp("clicked_series") as Promise<any>;
    }
    set clickedSeries(value: number) {
        this.setProp("clicked_series", value);
    }
    get clickedIndex(): Promise<number> {
        return this.getProp("clicked_index") as Promise<any>;
    }
    set clickedIndex(value: number) {
        this.setProp("clicked_index", value);
    }
    get valueFormat(): Promise<ValueFormat> {
        return this.getProp("value_format") as Promise<any>;
    }
    set valueFormat(value: ValueFormat) {
        this.setProp("value_format", value);
    }
    get labelAngle(): Promise<number> {
        return this.getProp("label_angle") as Promise<any>;
    }
    set labelAngle(value: number) {
        this.setProp("label_angle", value);
    }
    get titleAlign(): Promise<TitleAlign> {
        return this.getProp("title_align") as Promise<any>;
    }
    set titleAlign(value: TitleAlign) {
        this.setProp("title_align", value);
    }
    get titleBold(): Promise<boolean> {
        return this.getProp("title_bold") as Promise<any>;
    }
    set titleBold(value: boolean) {
        this.setProp("title_bold", value);
    }
    get areaOpacity(): Promise<number> {
        return this.getProp("area_opacity") as Promise<any>;
    }
    set areaOpacity(value: number) {
        this.setProp("area_opacity", value);
    }
    get legendPosition(): Promise<LegendPosition> {
        return this.getProp("legend_position") as Promise<any>;
    }
    set legendPosition(value: LegendPosition) {
        this.setProp("legend_position", value);
    }
    get stacked(): Promise<boolean> {
        return this.getProp("stacked") as Promise<any>;
    }
    set stacked(value: boolean) {
        this.setProp("stacked", value);
    }
    get colors(): Promise<string> {
        return this.getProp("colors") as Promise<any>;
    }
    set colors(value: string) {
        this.setProp("colors", value);
    }
    get centerText(): Promise<string> {
        return this.getProp("center_text") as Promise<any>;
    }
    set centerText(value: string) {
        this.setProp("center_text", value);
    }
    get sliceLabels(): Promise<string> {
        return this.getProp("slice_labels") as Promise<any>;
    }
    set sliceLabels(value: string) {
        this.setProp("slice_labels", value);
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
    get style(): Promise<DBChartStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: DBChartStyle) {
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

/** Um DataGrid de um form. */
export class DataGrid extends AylisComponent {
    get overscroll(): Promise<Overscroll> {
        return this.getProp("overscroll") as Promise<any>;
    }
    set overscroll(value: Overscroll) {
        this.setProp("overscroll", value);
    }
    get dataSource(): Promise<AylisComponent | null> {
        return this.getRef("data_source");
    }
    set dataSource(value: AylisComponent | null) {
        this.setRef("data_source", value);
    }
    get columns(): Promise<ColumnSpec[]> {
        return this.getProp("columns").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(columnSpecFromJson));
    }
    set columns(value: ColumnSpec[]) {
        this.setProp("columns", JSON.stringify(value.map(columnSpecToJson)));
    }
    get rowHeight(): Promise<number> {
        return this.getProp("row_height") as Promise<any>;
    }
    set rowHeight(value: number) {
        this.setProp("row_height", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only") as Promise<any>;
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get sortable(): Promise<boolean> {
        return this.getProp("sortable") as Promise<any>;
    }
    set sortable(value: boolean) {
        this.setProp("sortable", value);
    }
    get columnMove(): Promise<boolean> {
        return this.getProp("column_move") as Promise<any>;
    }
    set columnMove(value: boolean) {
        this.setProp("column_move", value);
    }
    get fixedColumns(): Promise<number> {
        return this.getProp("fixed_columns") as Promise<any>;
    }
    set fixedColumns(value: number) {
        this.setProp("fixed_columns", value);
    }
    get filterRow(): Promise<boolean> {
        return this.getProp("filter_row") as Promise<any>;
    }
    set filterRow(value: boolean) {
        this.setProp("filter_row", value);
    }
    get multiSelect(): Promise<boolean> {
        return this.getProp("multi_select") as Promise<any>;
    }
    set multiSelect(value: boolean) {
        this.setProp("multi_select", value);
    }
    get footer(): Promise<GridTotal[]> {
        return this.getProp("footer").then((t) => ((typeof t === "string" ? JSON.parse(t || "[]") : t) ?? []).map(gridTotalFromJson));
    }
    set footer(value: GridTotal[]) {
        this.setProp("footer", JSON.stringify(value.map(gridTotalToJson)));
    }
    get incrementalSearch(): Promise<boolean> {
        return this.getProp("incremental_search") as Promise<any>;
    }
    set incrementalSearch(value: boolean) {
        this.setProp("incremental_search", value);
    }
    get cursorColor(): Promise<Color> {
        return this.getProp("cursor_color").then(Color.fromJson);
    }
    set cursorColor(value: Color) {
        this.setProp("cursor_color", value.toJson());
    }
    get indicator(): Promise<boolean> {
        return this.getProp("indicator") as Promise<any>;
    }
    set indicator(value: boolean) {
        this.setProp("indicator", value);
    }
    get highlightCurrent(): Promise<boolean> {
        return this.getProp("highlight_current") as Promise<any>;
    }
    set highlightCurrent(value: boolean) {
        this.setProp("highlight_current", value);
    }
    get horizontalWheel(): Promise<HorizontalWheel> {
        return this.getProp("horizontal_wheel") as Promise<any>;
    }
    set horizontalWheel(value: HorizontalWheel) {
        this.setProp("horizontal_wheel", value);
    }
    get onError(): Promise<string> {
        return this.getProp("on_error") as Promise<any>;
    }
    set onError(value: string) {
        this.setProp("on_error", value);
    }
    get onTitleClick(): Promise<string> {
        return this.getProp("on_title_click") as Promise<any>;
    }
    set onTitleClick(value: string) {
        this.setProp("on_title_click", value);
    }
    get onSelectionChange(): Promise<string> {
        return this.getProp("on_selection_change") as Promise<any>;
    }
    set onSelectionChange(value: string) {
        this.setProp("on_selection_change", value);
    }
    get onColumnMoved(): Promise<string> {
        return this.getProp("on_column_moved") as Promise<any>;
    }
    set onColumnMoved(value: string) {
        this.setProp("on_column_moved", value);
    }
    get scrollLeft(): Promise<number> {
        return this.getProp("scroll_left") as Promise<any>;
    }
    set scrollLeft(value: number) {
        this.setProp("scroll_left", value);
    }
    get topRow(): Promise<number> {
        return this.getProp("top_row") as Promise<any>;
    }
    set topRow(value: number) {
        this.setProp("top_row", value);
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
    get style(): Promise<DataGridStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: DataGridStyle) {
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

/** Um DataSource de um form. */
export class DataSource extends AylisComponent {
    get dataSet(): Promise<AylisComponent | null> {
        return this.getRef("data_set");
    }
    set dataSet(value: AylisComponent | null) {
        this.setRef("data_set", value);
    }
    get autoEdit(): Promise<boolean> {
        return this.getProp("auto_edit") as Promise<any>;
    }
    set autoEdit(value: boolean) {
        this.setProp("auto_edit", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled") as Promise<any>;
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get onDataChange(): Promise<string> {
        return this.getProp("on_data_change") as Promise<any>;
    }
    set onDataChange(value: string) {
        this.setProp("on_data_change", value);
    }
    get onStateChange(): Promise<string> {
        return this.getProp("on_state_change") as Promise<any>;
    }
    set onStateChange(value: string) {
        this.setProp("on_state_change", value);
    }
}

/** Um Field de um form. */
export class Field extends AylisComponent {
    get fieldName(): Promise<string> {
        return this.getProp("field_name") as Promise<any>;
    }
    set fieldName(value: string) {
        this.setProp("field_name", value);
    }
    get type(): Promise<FieldType> {
        return this.getProp("type") as Promise<any>;
    }
    set type(value: FieldType) {
        this.setProp("type", value);
    }
    get size(): Promise<number> {
        return this.getProp("size") as Promise<any>;
    }
    set size(value: number) {
        this.setProp("size", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale") as Promise<any>;
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get rounding(): Promise<Rounding> {
        return this.getProp("rounding") as Promise<any>;
    }
    set rounding(value: Rounding) {
        this.setProp("rounding", value);
    }
    get displayLabel(): Promise<string> {
        return this.getProp("display_label") as Promise<any>;
    }
    set displayLabel(value: string) {
        this.setProp("display_label", value);
    }
    get displayWidth(): Promise<number> {
        return this.getProp("display_width") as Promise<any>;
    }
    set displayWidth(value: number) {
        this.setProp("display_width", value);
    }
    get displayFormat(): Promise<string> {
        return this.getProp("display_format") as Promise<any>;
    }
    set displayFormat(value: string) {
        this.setProp("display_format", value);
    }
    get editFormat(): Promise<string> {
        return this.getProp("edit_format") as Promise<any>;
    }
    set editFormat(value: string) {
        this.setProp("edit_format", value);
    }
    get alignment(): Promise<TextAlign> {
        return this.getProp("alignment") as Promise<any>;
    }
    set alignment(value: TextAlign) {
        this.setProp("alignment", value);
    }
    get required(): Promise<boolean> {
        return this.getProp("required") as Promise<any>;
    }
    set required(value: boolean) {
        this.setProp("required", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only") as Promise<any>;
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible") as Promise<any>;
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get min(): Promise<number> {
        return this.getProp("min") as Promise<any>;
    }
    set min(value: number) {
        this.setProp("min", value);
    }
    get max(): Promise<number> {
        return this.getProp("max") as Promise<any>;
    }
    set max(value: number) {
        this.setProp("max", value);
    }
    get defaultValue(): Promise<string> {
        return this.getProp("default_value") as Promise<any>;
    }
    set defaultValue(value: string) {
        this.setProp("default_value", value);
    }
    get displayValues(): Promise<string> {
        return this.getProp("display_values") as Promise<any>;
    }
    set displayValues(value: string) {
        this.setProp("display_values", value);
    }
    get charCase(): Promise<CharCase> {
        return this.getProp("char_case") as Promise<any>;
    }
    set charCase(value: CharCase) {
        this.setProp("char_case", value);
    }
    get editMask(): Promise<string> {
        return this.getProp("edit_mask") as Promise<any>;
    }
    set editMask(value: string) {
        this.setProp("edit_mask", value);
    }
    get sensitive(): Promise<boolean> {
        return this.getProp("sensitive") as Promise<any>;
    }
    set sensitive(value: boolean) {
        this.setProp("sensitive", value);
    }
    get kind(): Promise<FieldKind> {
        return this.getProp("kind") as Promise<any>;
    }
    set kind(value: FieldKind) {
        this.setProp("kind", value);
    }
    get expression(): Promise<string> {
        return this.getProp("expression") as Promise<any>;
    }
    set expression(value: string) {
        this.setProp("expression", value);
    }
    get lookupDataSet(): Promise<AylisComponent | null> {
        return this.getRef("lookup_data_set");
    }
    set lookupDataSet(value: AylisComponent | null) {
        this.setRef("lookup_data_set", value);
    }
    get keyFields(): Promise<string> {
        return this.getProp("key_fields") as Promise<any>;
    }
    set keyFields(value: string) {
        this.setProp("key_fields", value);
    }
    get lookupKeyFields(): Promise<string> {
        return this.getProp("lookup_key_fields") as Promise<any>;
    }
    set lookupKeyFields(value: string) {
        this.setProp("lookup_key_fields", value);
    }
    get lookupResultField(): Promise<string> {
        return this.getProp("lookup_result_field") as Promise<any>;
    }
    set lookupResultField(value: string) {
        this.setProp("lookup_result_field", value);
    }
    get onChange(): Promise<string> {
        return this.getProp("on_change") as Promise<any>;
    }
    set onChange(value: string) {
        this.setProp("on_change", value);
    }
    get onGetText(): Promise<string> {
        return this.getProp("on_get_text") as Promise<any>;
    }
    set onGetText(value: string) {
        this.setProp("on_get_text", value);
    }
    get onSetText(): Promise<string> {
        return this.getProp("on_set_text") as Promise<any>;
    }
    set onSetText(value: string) {
        this.setProp("on_set_text", value);
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
    gpuAdapter(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapter", args: [] }) as Promise<any>;
    }
    gpuAdapters(): Promise<string[]> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapters", args: [] }).then(listOf);
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

/** Um GroupBox de um form. */
export class GroupBox extends AylisComponent {
    get caption(): Promise<string> {
        return this.getProp("caption") as Promise<any>;
    }
    set caption(value: string) {
        this.setProp("caption", value);
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
    get style(): Promise<GroupBoxStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: GroupBoxStyle) {
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

/** Um Image de um form. */
export class Image extends AylisComponent {
    get image(): Promise<string> {
        return this.getProp("image") as Promise<any>;
    }
    set image(value: string) {
        this.setProp("image", value);
    }
    get fit(): Promise<ImageFit> {
        return this.getProp("fit") as Promise<any>;
    }
    set fit(value: ImageFit) {
        this.setProp("fit", value);
    }
    get radius(): Promise<number> {
        return this.getProp("radius") as Promise<any>;
    }
    set radius(value: number) {
        this.setProp("radius", value);
    }
    get zoomable(): Promise<boolean> {
        return this.getProp("zoomable") as Promise<any>;
    }
    set zoomable(value: boolean) {
        this.setProp("zoomable", value);
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
    get style(): Promise<ImageStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: ImageStyle) {
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

/** Um Splitter de um form. */
export class Splitter extends AylisComponent {
    get minSize(): Promise<number> {
        return this.getProp("min_size") as Promise<any>;
    }
    set minSize(value: number) {
        this.setProp("min_size", value);
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
    get style(): Promise<SplitterStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: SplitterStyle) {
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

/** Um Svg de um form. */
export class Svg extends AylisComponent {
    get source(): Promise<string> {
        return this.getProp("source") as Promise<any>;
    }
    set source(value: string) {
        this.setProp("source", value);
    }
    get color(): Promise<Color> {
        return this.getProp("color").then(Color.fromJson);
    }
    set color(value: Color) {
        this.setProp("color", value.toJson());
    }
    get fit(): Promise<ImageFit> {
        return this.getProp("fit") as Promise<any>;
    }
    set fit(value: ImageFit) {
        this.setProp("fit", value);
    }
    get highlight(): Promise<string[]> {
        return this.getProp("highlight").then(listOf);
    }
    set highlight(value: string[]) {
        this.setProp("highlight", value.join(";"));
    }
    get highlightColor(): Promise<Color> {
        return this.getProp("highlight_color").then(Color.fromJson);
    }
    set highlightColor(value: Color) {
        this.setProp("highlight_color", value.toJson());
    }
    get zoomTo(): Promise<string> {
        return this.getProp("zoom_to") as Promise<any>;
    }
    set zoomTo(value: string) {
        this.setProp("zoom_to", value);
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
    get style(): Promise<SvgStyle> {
        return this.getProp("style") as Promise<any>;
    }
    set style(value: SvgStyle) {
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

/** O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmIBGEComponents extends Form {
    cdsEstados!: ClientDataSet;
    cdsEstadosId!: Field;
    cdsEstadosSigla!: Field;
    cdsEstadosNome!: Field;
    cdsEstadosRegiao!: Field;
    cdsEstadosPopulacao!: Field;
    cdsEstadosArea!: Field;
    cdsEstadosDensidade!: Field;
    cdsEstadosPib!: Field;
    cdsEstadosPibPerCapita!: Field;
    cdsEstadosEstimativa!: Field;
    cdsEstadosEstimativaAno!: Field;
    cdsEstadosMunicipios!: Field;
    cdsEstadosBandeira!: Field;
    dsEstados!: DataSource;
    cdsMunicipios!: ClientDataSet;
    cdsMunicipiosId!: Field;
    cdsMunicipiosUfId!: Field;
    cdsMunicipiosUf!: Field;
    cdsMunicipiosNome!: Field;
    cdsMunicipiosMesorregiao!: Field;
    cdsMunicipiosMicrorregiao!: Field;
    cdsMunicipiosRegiaoIntermediaria!: Field;
    cdsMunicipiosRegiaoImediata!: Field;
    cdsMunicipiosPopulacao!: Field;
    cdsMunicipiosArea!: Field;
    cdsMunicipiosDensidade!: Field;
    cdsMunicipiosPib!: Field;
    cdsMunicipiosPibParticipacao!: Field;
    cdsMunicipiosPibPerCapita!: Field;
    cdsMunicipiosEstimativa!: Field;
    cdsMunicipiosEstimativaAno!: Field;
    cdsMunicipiosDistritos!: Field;
    cdsMunicipiosBandeira!: Field;
    dsMunicipios!: DataSource;
    cdsDistritos!: ClientDataSet;
    cdsDistritosId!: Field;
    cdsDistritosMunicipioId!: Field;
    cdsDistritosNome!: Field;
    dsDistritos!: DataSource;
    cdsPopulacao!: ClientDataSet;
    cdsPopulacaoAno!: Field;
    cdsPopulacaoHabitantes!: Field;
    dsPopulacao!: DataSource;
    cdsMaiores!: ClientDataSet;
    cdsMaioresNome!: Field;
    cdsMaioresHabitantes!: Field;
    dsMaiores!: DataSource;
    lblFonte!: Label;
    lytListas!: Layout;
    gbEstados!: GroupBox;
    grdEstados!: DataGrid;
    gbDistritos!: GroupBox;
    grdDistritos!: DataGrid;
    gbMunicipios!: GroupBox;
    grdMunicipios!: DataGrid;
    splListas!: Splitter;
    lytPainel!: Layout;
    pnlEstado!: Panel;
    svgBrasil!: Svg;
    lytBandeiraEstado!: Layout;
    lblEstado!: Label;
    lblRegiao!: Label;
    imgBandeiraEstado!: Image;
    lytNumerosEstado!: Layout;
    dbtEstadoPopulacao!: Label;
    lblEstadoEstimativa!: Label;
    dbtEstadoEstimativa!: Label;
    dbtEstadoArea!: Label;
    dbtEstadoDensidade!: Label;
    dbtEstadoPib!: Label;
    dbtEstadoPibPerCapita!: Label;
    dbtEstadoMunicipios!: Label;
    pnlMunicipio!: Panel;
    svgMunicipio!: Svg;
    svgEstado!: Svg;
    lytBandeiraMunicipio!: Layout;
    lblMunicipio!: Label;
    lblCodigo!: Label;
    imgBandeiraMunicipio!: Image;
    lblSemBandeira!: Label;
    lytNumerosMunicipio!: Layout;
    dbtMesorregiao!: Label;
    dbtMicrorregiao!: Label;
    dbtRegiaoImediata!: Label;
    dbtPopulacao!: Label;
    lblEstimativa!: Label;
    dbtEstimativa!: Label;
    dbtArea!: Label;
    dbtDensidade!: Label;
    dbtPib!: Label;
    dbtPibPerCapita!: Label;
    dbtPibParticipacao!: Label;
    lytGraficos!: Layout;
    dbcPopulacao!: DBChart;
    dbcMaiores!: DBChart;
    dbcEstados!: DBChart;
    bind(list: ComponentInfo[]) {
        this.cdsEstados = this.resolve(list, "cdsEstados", "ClientDataSet", ClientDataSet);
        this.cdsEstadosId = this.resolve(list, "cdsEstadosId", "Field", Field);
        this.cdsEstadosSigla = this.resolve(list, "cdsEstadosSigla", "Field", Field);
        this.cdsEstadosNome = this.resolve(list, "cdsEstadosNome", "Field", Field);
        this.cdsEstadosRegiao = this.resolve(list, "cdsEstadosRegiao", "Field", Field);
        this.cdsEstadosPopulacao = this.resolve(list, "cdsEstadosPopulacao", "Field", Field);
        this.cdsEstadosArea = this.resolve(list, "cdsEstadosArea", "Field", Field);
        this.cdsEstadosDensidade = this.resolve(list, "cdsEstadosDensidade", "Field", Field);
        this.cdsEstadosPib = this.resolve(list, "cdsEstadosPib", "Field", Field);
        this.cdsEstadosPibPerCapita = this.resolve(list, "cdsEstadosPibPerCapita", "Field", Field);
        this.cdsEstadosEstimativa = this.resolve(list, "cdsEstadosEstimativa", "Field", Field);
        this.cdsEstadosEstimativaAno = this.resolve(list, "cdsEstadosEstimativaAno", "Field", Field);
        this.cdsEstadosMunicipios = this.resolve(list, "cdsEstadosMunicipios", "Field", Field);
        this.cdsEstadosBandeira = this.resolve(list, "cdsEstadosBandeira", "Field", Field);
        this.dsEstados = this.resolve(list, "dsEstados", "DataSource", DataSource);
        this.cdsMunicipios = this.resolve(list, "cdsMunicipios", "ClientDataSet", ClientDataSet);
        this.cdsMunicipiosId = this.resolve(list, "cdsMunicipiosId", "Field", Field);
        this.cdsMunicipiosUfId = this.resolve(list, "cdsMunicipiosUfId", "Field", Field);
        this.cdsMunicipiosUf = this.resolve(list, "cdsMunicipiosUf", "Field", Field);
        this.cdsMunicipiosNome = this.resolve(list, "cdsMunicipiosNome", "Field", Field);
        this.cdsMunicipiosMesorregiao = this.resolve(list, "cdsMunicipiosMesorregiao", "Field", Field);
        this.cdsMunicipiosMicrorregiao = this.resolve(list, "cdsMunicipiosMicrorregiao", "Field", Field);
        this.cdsMunicipiosRegiaoIntermediaria = this.resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field", Field);
        this.cdsMunicipiosRegiaoImediata = this.resolve(list, "cdsMunicipiosRegiaoImediata", "Field", Field);
        this.cdsMunicipiosPopulacao = this.resolve(list, "cdsMunicipiosPopulacao", "Field", Field);
        this.cdsMunicipiosArea = this.resolve(list, "cdsMunicipiosArea", "Field", Field);
        this.cdsMunicipiosDensidade = this.resolve(list, "cdsMunicipiosDensidade", "Field", Field);
        this.cdsMunicipiosPib = this.resolve(list, "cdsMunicipiosPib", "Field", Field);
        this.cdsMunicipiosPibParticipacao = this.resolve(list, "cdsMunicipiosPibParticipacao", "Field", Field);
        this.cdsMunicipiosPibPerCapita = this.resolve(list, "cdsMunicipiosPibPerCapita", "Field", Field);
        this.cdsMunicipiosEstimativa = this.resolve(list, "cdsMunicipiosEstimativa", "Field", Field);
        this.cdsMunicipiosEstimativaAno = this.resolve(list, "cdsMunicipiosEstimativaAno", "Field", Field);
        this.cdsMunicipiosDistritos = this.resolve(list, "cdsMunicipiosDistritos", "Field", Field);
        this.cdsMunicipiosBandeira = this.resolve(list, "cdsMunicipiosBandeira", "Field", Field);
        this.dsMunicipios = this.resolve(list, "dsMunicipios", "DataSource", DataSource);
        this.cdsDistritos = this.resolve(list, "cdsDistritos", "ClientDataSet", ClientDataSet);
        this.cdsDistritosId = this.resolve(list, "cdsDistritosId", "Field", Field);
        this.cdsDistritosMunicipioId = this.resolve(list, "cdsDistritosMunicipioId", "Field", Field);
        this.cdsDistritosNome = this.resolve(list, "cdsDistritosNome", "Field", Field);
        this.dsDistritos = this.resolve(list, "dsDistritos", "DataSource", DataSource);
        this.cdsPopulacao = this.resolve(list, "cdsPopulacao", "ClientDataSet", ClientDataSet);
        this.cdsPopulacaoAno = this.resolve(list, "cdsPopulacaoAno", "Field", Field);
        this.cdsPopulacaoHabitantes = this.resolve(list, "cdsPopulacaoHabitantes", "Field", Field);
        this.dsPopulacao = this.resolve(list, "dsPopulacao", "DataSource", DataSource);
        this.cdsMaiores = this.resolve(list, "cdsMaiores", "ClientDataSet", ClientDataSet);
        this.cdsMaioresNome = this.resolve(list, "cdsMaioresNome", "Field", Field);
        this.cdsMaioresHabitantes = this.resolve(list, "cdsMaioresHabitantes", "Field", Field);
        this.dsMaiores = this.resolve(list, "dsMaiores", "DataSource", DataSource);
        this.lblFonte = this.resolve(list, "lblFonte", "Label", Label);
        this.lytListas = this.resolve(list, "lytListas", "Layout", Layout);
        this.gbEstados = this.resolve(list, "gbEstados", "GroupBox", GroupBox);
        this.grdEstados = this.resolve(list, "grdEstados", "DataGrid", DataGrid);
        this.gbDistritos = this.resolve(list, "gbDistritos", "GroupBox", GroupBox);
        this.grdDistritos = this.resolve(list, "grdDistritos", "DataGrid", DataGrid);
        this.gbMunicipios = this.resolve(list, "gbMunicipios", "GroupBox", GroupBox);
        this.grdMunicipios = this.resolve(list, "grdMunicipios", "DataGrid", DataGrid);
        this.splListas = this.resolve(list, "splListas", "Splitter", Splitter);
        this.lytPainel = this.resolve(list, "lytPainel", "Layout", Layout);
        this.pnlEstado = this.resolve(list, "pnlEstado", "Panel", Panel);
        this.svgBrasil = this.resolve(list, "svgBrasil", "Svg", Svg);
        this.lytBandeiraEstado = this.resolve(list, "lytBandeiraEstado", "Layout", Layout);
        this.lblEstado = this.resolve(list, "lblEstado", "Label", Label);
        this.lblRegiao = this.resolve(list, "lblRegiao", "Label", Label);
        this.imgBandeiraEstado = this.resolve(list, "imgBandeiraEstado", "Image", Image);
        this.lytNumerosEstado = this.resolve(list, "lytNumerosEstado", "Layout", Layout);
        this.dbtEstadoPopulacao = this.resolve(list, "dbtEstadoPopulacao", "Label", Label);
        this.lblEstadoEstimativa = this.resolve(list, "lblEstadoEstimativa", "Label", Label);
        this.dbtEstadoEstimativa = this.resolve(list, "dbtEstadoEstimativa", "Label", Label);
        this.dbtEstadoArea = this.resolve(list, "dbtEstadoArea", "Label", Label);
        this.dbtEstadoDensidade = this.resolve(list, "dbtEstadoDensidade", "Label", Label);
        this.dbtEstadoPib = this.resolve(list, "dbtEstadoPib", "Label", Label);
        this.dbtEstadoPibPerCapita = this.resolve(list, "dbtEstadoPibPerCapita", "Label", Label);
        this.dbtEstadoMunicipios = this.resolve(list, "dbtEstadoMunicipios", "Label", Label);
        this.pnlMunicipio = this.resolve(list, "pnlMunicipio", "Panel", Panel);
        this.svgMunicipio = this.resolve(list, "svgMunicipio", "Svg", Svg);
        this.svgEstado = this.resolve(list, "svgEstado", "Svg", Svg);
        this.lytBandeiraMunicipio = this.resolve(list, "lytBandeiraMunicipio", "Layout", Layout);
        this.lblMunicipio = this.resolve(list, "lblMunicipio", "Label", Label);
        this.lblCodigo = this.resolve(list, "lblCodigo", "Label", Label);
        this.imgBandeiraMunicipio = this.resolve(list, "imgBandeiraMunicipio", "Image", Image);
        this.lblSemBandeira = this.resolve(list, "lblSemBandeira", "Label", Label);
        this.lytNumerosMunicipio = this.resolve(list, "lytNumerosMunicipio", "Layout", Layout);
        this.dbtMesorregiao = this.resolve(list, "dbtMesorregiao", "Label", Label);
        this.dbtMicrorregiao = this.resolve(list, "dbtMicrorregiao", "Label", Label);
        this.dbtRegiaoImediata = this.resolve(list, "dbtRegiaoImediata", "Label", Label);
        this.dbtPopulacao = this.resolve(list, "dbtPopulacao", "Label", Label);
        this.lblEstimativa = this.resolve(list, "lblEstimativa", "Label", Label);
        this.dbtEstimativa = this.resolve(list, "dbtEstimativa", "Label", Label);
        this.dbtArea = this.resolve(list, "dbtArea", "Label", Label);
        this.dbtDensidade = this.resolve(list, "dbtDensidade", "Label", Label);
        this.dbtPib = this.resolve(list, "dbtPib", "Label", Label);
        this.dbtPibPerCapita = this.resolve(list, "dbtPibPerCapita", "Label", Label);
        this.dbtPibParticipacao = this.resolve(list, "dbtPibParticipacao", "Label", Label);
        this.lytGraficos = this.resolve(list, "lytGraficos", "Layout", Layout);
        this.dbcPopulacao = this.resolve(list, "dbcPopulacao", "DBChart", DBChart);
        this.dbcMaiores = this.resolve(list, "dbcMaiores", "DBChart", DBChart);
        this.dbcEstados = this.resolve(list, "dbcEstados", "DBChart", DBChart);
    }
    /** O ClientDataSet deste form com o nome. */
    findClientDataSet(name: string): ClientDataSet | undefined {
        switch (name) {
            case "cdsEstados": return this.cdsEstados;
            case "cdsMunicipios": return this.cdsMunicipios;
            case "cdsDistritos": return this.cdsDistritos;
            case "cdsPopulacao": return this.cdsPopulacao;
            case "cdsMaiores": return this.cdsMaiores;
        }
        return undefined;
    }
    /** O ClientDataSet que disparou o evento. */
    senderAsClientDataSet(ev: AylisEvent): ClientDataSet | undefined {
        return this.findClientDataSet(ev.sender.name);
    }
    /** O DBChart deste form com o nome. */
    findDBChart(name: string): DBChart | undefined {
        switch (name) {
            case "dbcPopulacao": return this.dbcPopulacao;
            case "dbcMaiores": return this.dbcMaiores;
            case "dbcEstados": return this.dbcEstados;
        }
        return undefined;
    }
    /** O DBChart que disparou o evento. */
    senderAsDBChart(ev: AylisEvent): DBChart | undefined {
        return this.findDBChart(ev.sender.name);
    }
    /** O DataGrid deste form com o nome. */
    findDataGrid(name: string): DataGrid | undefined {
        switch (name) {
            case "grdEstados": return this.grdEstados;
            case "grdDistritos": return this.grdDistritos;
            case "grdMunicipios": return this.grdMunicipios;
        }
        return undefined;
    }
    /** O DataGrid que disparou o evento. */
    senderAsDataGrid(ev: AylisEvent): DataGrid | undefined {
        return this.findDataGrid(ev.sender.name);
    }
    /** O DataSource deste form com o nome. */
    findDataSource(name: string): DataSource | undefined {
        switch (name) {
            case "dsEstados": return this.dsEstados;
            case "dsMunicipios": return this.dsMunicipios;
            case "dsDistritos": return this.dsDistritos;
            case "dsPopulacao": return this.dsPopulacao;
            case "dsMaiores": return this.dsMaiores;
        }
        return undefined;
    }
    /** O DataSource que disparou o evento. */
    senderAsDataSource(ev: AylisEvent): DataSource | undefined {
        return this.findDataSource(ev.sender.name);
    }
    /** O Field deste form com o nome. */
    findField(name: string): Field | undefined {
        switch (name) {
            case "cdsEstadosId": return this.cdsEstadosId;
            case "cdsEstadosSigla": return this.cdsEstadosSigla;
            case "cdsEstadosNome": return this.cdsEstadosNome;
            case "cdsEstadosRegiao": return this.cdsEstadosRegiao;
            case "cdsEstadosPopulacao": return this.cdsEstadosPopulacao;
            case "cdsEstadosArea": return this.cdsEstadosArea;
            case "cdsEstadosDensidade": return this.cdsEstadosDensidade;
            case "cdsEstadosPib": return this.cdsEstadosPib;
            case "cdsEstadosPibPerCapita": return this.cdsEstadosPibPerCapita;
            case "cdsEstadosEstimativa": return this.cdsEstadosEstimativa;
            case "cdsEstadosEstimativaAno": return this.cdsEstadosEstimativaAno;
            case "cdsEstadosMunicipios": return this.cdsEstadosMunicipios;
            case "cdsEstadosBandeira": return this.cdsEstadosBandeira;
            case "cdsMunicipiosId": return this.cdsMunicipiosId;
            case "cdsMunicipiosUfId": return this.cdsMunicipiosUfId;
            case "cdsMunicipiosUf": return this.cdsMunicipiosUf;
            case "cdsMunicipiosNome": return this.cdsMunicipiosNome;
            case "cdsMunicipiosMesorregiao": return this.cdsMunicipiosMesorregiao;
            case "cdsMunicipiosMicrorregiao": return this.cdsMunicipiosMicrorregiao;
            case "cdsMunicipiosRegiaoIntermediaria": return this.cdsMunicipiosRegiaoIntermediaria;
            case "cdsMunicipiosRegiaoImediata": return this.cdsMunicipiosRegiaoImediata;
            case "cdsMunicipiosPopulacao": return this.cdsMunicipiosPopulacao;
            case "cdsMunicipiosArea": return this.cdsMunicipiosArea;
            case "cdsMunicipiosDensidade": return this.cdsMunicipiosDensidade;
            case "cdsMunicipiosPib": return this.cdsMunicipiosPib;
            case "cdsMunicipiosPibParticipacao": return this.cdsMunicipiosPibParticipacao;
            case "cdsMunicipiosPibPerCapita": return this.cdsMunicipiosPibPerCapita;
            case "cdsMunicipiosEstimativa": return this.cdsMunicipiosEstimativa;
            case "cdsMunicipiosEstimativaAno": return this.cdsMunicipiosEstimativaAno;
            case "cdsMunicipiosDistritos": return this.cdsMunicipiosDistritos;
            case "cdsMunicipiosBandeira": return this.cdsMunicipiosBandeira;
            case "cdsDistritosId": return this.cdsDistritosId;
            case "cdsDistritosMunicipioId": return this.cdsDistritosMunicipioId;
            case "cdsDistritosNome": return this.cdsDistritosNome;
            case "cdsPopulacaoAno": return this.cdsPopulacaoAno;
            case "cdsPopulacaoHabitantes": return this.cdsPopulacaoHabitantes;
            case "cdsMaioresNome": return this.cdsMaioresNome;
            case "cdsMaioresHabitantes": return this.cdsMaioresHabitantes;
        }
        return undefined;
    }
    /** O Field que disparou o evento. */
    senderAsField(ev: AylisEvent): Field | undefined {
        return this.findField(ev.sender.name);
    }
    /** O GroupBox deste form com o nome. */
    findGroupBox(name: string): GroupBox | undefined {
        switch (name) {
            case "gbEstados": return this.gbEstados;
            case "gbDistritos": return this.gbDistritos;
            case "gbMunicipios": return this.gbMunicipios;
        }
        return undefined;
    }
    /** O GroupBox que disparou o evento. */
    senderAsGroupBox(ev: AylisEvent): GroupBox | undefined {
        return this.findGroupBox(ev.sender.name);
    }
    /** O Image deste form com o nome. */
    findImage(name: string): Image | undefined {
        switch (name) {
            case "imgBandeiraEstado": return this.imgBandeiraEstado;
            case "imgBandeiraMunicipio": return this.imgBandeiraMunicipio;
        }
        return undefined;
    }
    /** O Image que disparou o evento. */
    senderAsImage(ev: AylisEvent): Image | undefined {
        return this.findImage(ev.sender.name);
    }
    /** O Label deste form com o nome. */
    findLabel(name: string): Label | undefined {
        switch (name) {
            case "lblFonte": return this.lblFonte;
            case "lblEstado": return this.lblEstado;
            case "lblRegiao": return this.lblRegiao;
            case "dbtEstadoPopulacao": return this.dbtEstadoPopulacao;
            case "lblEstadoEstimativa": return this.lblEstadoEstimativa;
            case "dbtEstadoEstimativa": return this.dbtEstadoEstimativa;
            case "dbtEstadoArea": return this.dbtEstadoArea;
            case "dbtEstadoDensidade": return this.dbtEstadoDensidade;
            case "dbtEstadoPib": return this.dbtEstadoPib;
            case "dbtEstadoPibPerCapita": return this.dbtEstadoPibPerCapita;
            case "dbtEstadoMunicipios": return this.dbtEstadoMunicipios;
            case "lblMunicipio": return this.lblMunicipio;
            case "lblCodigo": return this.lblCodigo;
            case "lblSemBandeira": return this.lblSemBandeira;
            case "dbtMesorregiao": return this.dbtMesorregiao;
            case "dbtMicrorregiao": return this.dbtMicrorregiao;
            case "dbtRegiaoImediata": return this.dbtRegiaoImediata;
            case "dbtPopulacao": return this.dbtPopulacao;
            case "lblEstimativa": return this.lblEstimativa;
            case "dbtEstimativa": return this.dbtEstimativa;
            case "dbtArea": return this.dbtArea;
            case "dbtDensidade": return this.dbtDensidade;
            case "dbtPib": return this.dbtPib;
            case "dbtPibPerCapita": return this.dbtPibPerCapita;
            case "dbtPibParticipacao": return this.dbtPibParticipacao;
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
            case "lytListas": return this.lytListas;
            case "lytPainel": return this.lytPainel;
            case "lytBandeiraEstado": return this.lytBandeiraEstado;
            case "lytNumerosEstado": return this.lytNumerosEstado;
            case "lytBandeiraMunicipio": return this.lytBandeiraMunicipio;
            case "lytNumerosMunicipio": return this.lytNumerosMunicipio;
            case "lytGraficos": return this.lytGraficos;
        }
        return undefined;
    }
    /** O Layout que disparou o evento. */
    senderAsLayout(ev: AylisEvent): Layout | undefined {
        return this.findLayout(ev.sender.name);
    }
    /** O Panel deste form com o nome. */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlEstado": return this.pnlEstado;
            case "pnlMunicipio": return this.pnlMunicipio;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O Splitter deste form com o nome. */
    findSplitter(name: string): Splitter | undefined {
        switch (name) {
            case "splListas": return this.splListas;
        }
        return undefined;
    }
    /** O Splitter que disparou o evento. */
    senderAsSplitter(ev: AylisEvent): Splitter | undefined {
        return this.findSplitter(ev.sender.name);
    }
    /** O Svg deste form com o nome. */
    findSvg(name: string): Svg | undefined {
        switch (name) {
            case "svgBrasil": return this.svgBrasil;
            case "svgMunicipio": return this.svgMunicipio;
            case "svgEstado": return this.svgEstado;
        }
        return undefined;
    }
    /** O Svg que disparou o evento. */
    senderAsSvg(ev: AylisEvent): Svg | undefined {
        return this.findSvg(ev.sender.name);
    }
}

/** A janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers). */
export let frmIBGE: FrmIBGEComponents;
registerFormClass("frmIBGE", FrmIBGEComponents, (form) => {
    frmIBGE = form as FrmIBGEComponents;
});
