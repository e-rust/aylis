"""Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from __future__ import annotations

import datetime
import json
import dataclasses
from dataclasses import dataclass
from enum import Enum

from aylis_client import CurrentForm, AylisComponent, AylisForm, Color, Bounds, Shadow, Easing, Point, register_form_class


def _enum(cls, value):
    """O valor da tela como o enum (o primeiro, se não for um dos nomes)."""
    try:
        return cls(value)
    except ValueError:
        return next(iter(cls))


def _date(value):
    try:
        return datetime.date.fromisoformat(value) if value else None
    except ValueError:
        return None


def _time(value):
    try:
        return datetime.time.fromisoformat(value) if value else None
    except ValueError:
        return None


def _datetime(value):
    try:
        return datetime.datetime.fromisoformat(value) if value else None
    except ValueError:
        return None


def _time_text(t):
    if t is None:
        return ""
    return t.strftime("%H:%M" if t.second == 0 else "%H:%M:%S")


def _datetime_text(d):
    if d is None:
        return ""
    return d.strftime("%Y-%m-%dT%H:%M" if d.second == 0 else "%Y-%m-%dT%H:%M:%S")


def _list(value):
    return [s.strip() for s in (value or "").split(";") if s.strip()]


def _numbers(value):
    out = []
    for s in _list(value):
        try:
            out.append(float(s))
        except ValueError:
            pass
    return out


def _colors(value):
    return [Color.hex(s) for s in _list(value) if s.startswith("#")]


@dataclass
class DateRange:
    """O intervalo de datas (o `on_change` do DateRangePicker); `None`: sem a data."""

    start: "datetime.date | None" = None
    end: "datetime.date | None" = None


def _date_range(value):
    parts = (str(value or "") + ";").split(";")
    return DateRange(_date(parts[0].strip()), _date(parts[1].strip()))


class Align(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    TOP = "top"
    BOTTOM = "bottom"
    LEFT = "left"
    RIGHT = "right"
    MOST_TOP = "most_top"
    MOST_BOTTOM = "most_bottom"
    MOST_LEFT = "most_left"
    MOST_RIGHT = "most_right"
    CLIENT = "client"
    CONTENTS = "contents"
    CENTER = "center"
    VERT_CENTER = "vert_center"
    HORZ_CENTER = "horz_center"
    HORIZONTAL = "horizontal"
    VERTICAL = "vertical"
    FIT = "fit"


class Anchor(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LEFT = "left"
    TOP = "top"
    RIGHT = "right"
    BOTTOM = "bottom"


class ApplyMode(str, Enum):
    """Os valores de um enum do acesso tipado."""

    CACHED = "cached"
    ON_POST = "on_post"


class BlendMode(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NORMAL = "normal"
    MULTIPLY = "multiply"
    SCREEN = "screen"
    OVERLAY = "overlay"
    DARKEN = "darken"
    LIGHTEN = "lighten"
    COLOR_DODGE = "color_dodge"
    COLOR_BURN = "color_burn"
    HARD_LIGHT = "hard_light"
    SOFT_LIGHT = "soft_light"
    DIFFERENCE = "difference"
    EXCLUSION = "exclusion"
    HUE = "hue"
    SATURATION = "saturation"
    COLOR = "color"
    LUMINOSITY = "luminosity"


class BorderStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    SIZEABLE = "sizeable"
    SINGLE = "single"
    NONE = "none"
    CUSTOM = "custom"


class CharCase(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NORMAL = "normal"
    UPPER = "upper"
    LOWER = "lower"


class ChartKind(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LINE = "line"
    BAR = "bar"
    AREA = "area"
    PIE = "pie"
    DONUT = "donut"
    SPARKLINE = "sparkline"
    HEATMAP = "heatmap"
    SCATTER = "scatter"
    STACKED_BAR = "stacked_bar"
    WATERFALL = "waterfall"
    FUNNEL = "funnel"
    TREEMAP = "treemap"
    RADAR = "radar"
    HORIZONTAL_BAR = "horizontal_bar"
    BUBBLE = "bubble"
    POLAR_AREA = "polar_area"
    STACKED_LINE = "stacked_line"
    STACKED_AREA = "stacked_area"
    MIXED = "mixed"
    SANKEY = "sankey"
    WAFFLE = "waffle"


class ChartSeriesFill(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    ORIGIN = "origin"
    START = "start"
    END = "end"
    SERIES = "series"


class ChartSeriesKind(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LINE = "line"
    BAR = "bar"
    AREA = "area"
    SCATTER = "scatter"


class ChartSeriesPoint(str, Enum):
    """Os valores de um enum do acesso tipado."""

    CIRCLE = "circle"
    SQUARE = "square"
    TRIANGLE = "triangle"
    DIAMOND = "diamond"
    CROSS = "cross"
    NONE = "none"


class CloseAction(str, Enum):
    """Os valores de um enum do acesso tipado."""

    HIDE = "hide"
    FREE = "free"


class CursorShape(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = "default"
    POINTER = "pointer"
    TEXT = "text"
    WAIT = "wait"
    CROSSHAIR = "crosshair"
    MOVE = "move"
    NOT_ALLOWED = "not_allowed"
    RESIZE_HORIZONTAL = "resize_horizontal"
    RESIZE_VERTICAL = "resize_vertical"
    RESIZE_NE_SW = "resize_ne_sw"
    RESIZE_NW_SE = "resize_nw_se"
    GRAB = "grab"
    GRABBING = "grabbing"
    NONE = "none"


class DBChartStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class DataGridStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class FieldKind(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DATA = "data"
    CALCULATED = "calculated"
    LOOKUP = "lookup"


class FieldType(str, Enum):
    """Os valores de um enum do acesso tipado."""

    STRING = "string"
    MEMO = "memo"
    INTEGER = "integer"
    FLOAT = "float"
    CURRENCY = "currency"
    BOOLEAN = "boolean"
    DATE = "date"
    TIME = "time"
    DATETIME = "datetime"
    BLOB = "blob"
    AUTO_INC = "auto_inc"
    TIMESTAMP = "timestamp"


class FontWeight(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = "default"
    THIN = "thin"
    LIGHT = "light"
    NORMAL = "normal"
    MEDIUM = "medium"
    SEMIBOLD = "semibold"
    BOLD = "bold"
    BLACK = "black"


class FormOrientation(str, Enum):
    """Os valores de um enum do acesso tipado."""

    PROJECT = "project"
    ANY = "any"
    PORTRAIT = "portrait"
    LANDSCAPE = "landscape"
    SENSOR_PORTRAIT = "sensor_portrait"
    SENSOR_LANDSCAPE = "sensor_landscape"


class FormPosition(str, Enum):
    """Os valores de um enum do acesso tipado."""

    CENTER = "center"
    SCREEN_CENTER = "screen_center"
    DESIGNED = "designed"
    DEFAULT = "default"
    TOP_LEFT = "top_left"
    TOP_RIGHT = "top_right"
    BOTTOM_LEFT = "bottom_left"
    BOTTOM_RIGHT = "bottom_right"


class FormPresent(str, Enum):
    """Os valores de um enum do acesso tipado."""

    PROJECT = "project"
    WINDOW = "window"
    COMPOSITION = "composition"
    COMPOSED = "composed"


class FormStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class GridTotalFunction(str, Enum):
    """Os valores de um enum do acesso tipado."""

    SUM = "sum"
    AVG = "avg"
    MIN = "min"
    MAX = "max"
    COUNT = "count"


class GroupBoxStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class HorizontalWheel(str, Enum):
    """Os valores de um enum do acesso tipado."""

    SHIFT = "shift"
    ALWAYS = "always"
    NONE = "none"


class ImageFit(str, Enum):
    """Os valores de um enum do acesso tipado."""

    CONTAIN = "contain"
    COVER = "cover"
    STRETCH = "stretch"
    CENTER = "center"
    ORIGINAL = "original"


class ImageStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class LabelStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    CAPTION = "caption"
    ERROR = "error"
    HEADING = "heading"
    KBD = "kbd"
    LINK = "link"
    MONO = "mono"
    MUTED = "muted"
    TITLE = "title"


class LayerCaching(str, Enum):
    """Os valores de um enum do acesso tipado."""

    PROJECT = "project"
    ON = "on"
    OFF = "off"


class LayoutStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class LegendPosition(str, Enum):
    """Os valores de um enum do acesso tipado."""

    TOP_RIGHT = "top_right"
    TOP_LEFT = "top_left"
    TOP = "top"
    BOTTOM = "bottom"


class Overscroll(str, Enum):
    """Os valores de um enum do acesso tipado."""

    AUTO = "auto"
    GLOW = "glow"
    STRETCH = "stretch"
    BOUNCE = "bounce"
    NONE = "none"


class PanelStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    APPBAR = "appbar"
    CARD = "card"
    FILLED = "filled"
    FLAT = "flat"
    GLASS = "glass"
    OUTLINED = "outlined"
    OVERLAY = "overlay"
    REVEAL = "reveal"
    SIDEBAR = "sidebar"


class Rounding(str, Enum):
    """Os valores de um enum do acesso tipado."""

    HALF_UP = "half_up"
    HALF_EVEN = "half_even"
    TRUNCATE = "truncate"
    UP = "up"


class SelfAlign(str, Enum):
    """Os valores de um enum do acesso tipado."""

    STRETCH = "stretch"
    START = "start"
    CENTER = "center"
    END = "end"


class SplitterStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class SvgStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class TextAlign(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LEADING = "leading"
    CENTER = "center"
    TRAILING = "trailing"
    JUSTIFY = "justify"


class TextCase(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    UPPER = "upper"
    LOWER = "lower"


class ThemeName(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    AYLIS_LIGHT = "Aylis Light"
    AYLIS_DARK = "Aylis Dark"
    MATERIAL_3_LIGHT = "Material 3 Light"
    MATERIAL_3_DARK = "Material 3 Dark"
    MATERIAL_2_LIGHT = "Material 2 Light"
    MATERIAL_2_DARK = "Material 2 Dark"
    ANDROID_7_LIGHT = "Android 7 Light"
    ANDROID_7_DARK = "Android 7 Dark"
    I_OS_26_LIGHT = "iOS 26 Light"
    I_OS_26_DARK = "iOS 26 Dark"
    I_OS_13_LIGHT = "iOS 13 Light"
    I_OS_13_DARK = "iOS 13 Dark"
    I_OS_7_LIGHT = "iOS 7 Light"
    WIN_UI_3_LIGHT = "WinUI 3 Light"
    WIN_UI_3_DARK = "WinUI 3 Dark"
    WINDOWS_11_LIGHT = "Windows 11 Light"
    WINDOWS_11_DARK = "Windows 11 Dark"
    WINDOWS_10_LIGHT = "Windows 10 Light"
    WINDOWS_10_DARK = "Windows 10 Dark"
    WIN32 = "Win32"
    WINDOWS_7 = "Windows 7"
    WINDOWS_PHONE_DARK = "Windows Phone Dark"
    WINDOWS_PHONE_LIGHT = "Windows Phone Light"
    VUETIFY_3_LIGHT = "Vuetify 3 Light"
    VUETIFY_3_DARK = "Vuetify 3 Dark"
    BOOTSTRAP_5_LIGHT = "Bootstrap 5 Light"
    BOOTSTRAP_5_DARK = "Bootstrap 5 Dark"
    SHADCN_UI_LIGHT = "shadcn/ui Light"
    SHADCN_UI_DARK = "shadcn/ui Dark"
    ANT_DESIGN_5_LIGHT = "Ant Design 5 Light"
    ANT_DESIGN_5_DARK = "Ant Design 5 Dark"
    CHAKRA_UI_LIGHT = "Chakra UI Light"
    CHAKRA_UI_DARK = "Chakra UI Dark"
    QUASAR_2_LIGHT = "Quasar 2 Light"
    QUASAR_2_DARK = "Quasar 2 Dark"


class TitleAlign(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LEFT = "left"
    CENTER = "center"
    RIGHT = "right"


class Transition(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    FADE = "fade"
    SLIDE_LEFT = "slide_left"
    SLIDE_RIGHT = "slide_right"
    SLIDE_UP = "slide_up"
    SLIDE_DOWN = "slide_down"


class Trimming(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    CHARACTER = "character"
    WORD = "word"


class ValueFormat(str, Enum):
    """Os valores de um enum do acesso tipado."""

    SHORT = "short"
    FULL = "full"
    PERCENT = "percent"


class WindowCorners(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = "default"
    ROUND = "round"
    SMALL = "small"
    SQUARE = "square"


class WindowShape(str, Enum):
    """Os valores de um enum do acesso tipado."""

    RECTANGLE = "rectangle"
    ROUNDED = "rounded"
    ELLIPSE = "ellipse"
    PATH = "path"


class WindowState(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NORMAL = "normal"
    MAXIMIZED = "maximized"
    MINIMIZED = "minimized"
    FULL_SCREEN = "full_screen"


def _record(cls, value):
    """O registro do JSON da tela (None se não há)."""
    data = json.loads(value) if isinstance(value, str) and value else value
    return cls.from_json(data) if isinstance(data, dict) else None


def _records(cls, value):
    data = json.loads(value) if isinstance(value, str) and value else value
    return [cls.from_json(x) for x in data] if isinstance(data, list) else []


def _record_text(r):
    return json.dumps(r.to_json(), ensure_ascii=False) if r is not None else ""


def _records_text(items):
    return json.dumps([r.to_json() for r in items], ensure_ascii=False)


@dataclass
class ChartSeries:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    name: "str" = ""
    values: "list[float]" = dataclasses.field(default_factory=list)
    x: "list[float]" = dataclasses.field(default_factory=list)
    radius: "list[float]" = dataclasses.field(default_factory=list)
    kind: "ChartSeriesKind" = dataclasses.field(default_factory=lambda: next(iter(ChartSeriesKind)))
    right_axis: "bool" = False
    color: "Color" = dataclasses.field(default_factory=Color)
    width: "float" = 0.0
    dash: "list[float]" = dataclasses.field(default_factory=list)
    point: "ChartSeriesPoint" = dataclasses.field(default_factory=lambda: next(iter(ChartSeriesPoint)))
    fill: "ChartSeriesFill" = dataclasses.field(default_factory=lambda: next(iter(ChartSeriesFill)))
    fill_series: "int" = 0
    hidden: "bool" = False

    @staticmethod
    def from_json(data):
        return ChartSeries(
            name=str(data.get("name") or ""),
            values=[float(x) for x in data.get("values") or []],
            x=[float(x) for x in data.get("x") or []],
            radius=[float(x) for x in data.get("radius") or []],
            kind=_enum(ChartSeriesKind, data.get("kind")),
            right_axis=bool(data.get("right_axis") or False),
            color=Color.from_json(data.get("color")) if data.get("color") else Color(),
            width=float(data.get("width") or 0),
            dash=[float(x) for x in data.get("dash") or []],
            point=_enum(ChartSeriesPoint, data.get("point")),
            fill=_enum(ChartSeriesFill, data.get("fill")),
            fill_series=int(data.get("fill_series") or 0),
            hidden=bool(data.get("hidden") or False),
        )

    def to_json(self):
        return {
            "name": self.name,
            "values": list(self.values),
            "x": list(self.x),
            "radius": list(self.radius),
            "kind": self.kind.value,
            "right_axis": self.right_axis,
            "color": self.color.to_json(),
            "width": self.width,
            "dash": list(self.dash),
            "point": self.point.value,
            "fill": self.fill.value,
            "fill_series": self.fill_series,
            "hidden": self.hidden,
        }


@dataclass
class ColumnSpec:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    field: "str" = ""
    title: "str" = ""
    width: "float" = 0.0
    read_only: "bool" = False
    color: "Color" = dataclasses.field(default_factory=Color)
    text_color: "Color" = dataclasses.field(default_factory=Color)

    @staticmethod
    def from_json(data):
        return ColumnSpec(
            field=str(data.get("field") or ""),
            title=str(data.get("title") or ""),
            width=float(data.get("width") or 0),
            read_only=bool(data.get("read_only") or False),
            color=Color.from_json(data.get("color")) if data.get("color") else Color(),
            text_color=Color.from_json(data.get("text_color")) if data.get("text_color") else Color(),
        )

    def to_json(self):
        return {
            "field": self.field,
            "title": self.title,
            "width": self.width,
            "read_only": self.read_only,
            "color": self.color.to_json(),
            "text_color": self.text_color.to_json(),
        }


@dataclass
class FilterParam:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    name: "str" = ""
    value: "str" = ""

    @staticmethod
    def from_json(data):
        return FilterParam(
            name=str(data.get("name") or ""),
            value=str(data.get("value") or ""),
        )

    def to_json(self):
        return {
            "name": self.name,
            "value": self.value,
        }


@dataclass
class GestureEvent:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    phase: "str" = ""
    x: "float" = 0.0
    y: "float" = 0.0
    scale: "float" = 0.0
    delta_scale: "float" = 0.0
    rotation: "float" = 0.0
    delta_rotation: "float" = 0.0
    pan_x: "float" = 0.0
    pan_y: "float" = 0.0

    @staticmethod
    def from_json(data):
        return GestureEvent(
            phase=str(data.get("phase") or ""),
            x=float(data.get("x") or 0),
            y=float(data.get("y") or 0),
            scale=float(data.get("scale") or 0),
            delta_scale=float(data.get("delta_scale") or 0),
            rotation=float(data.get("rotation") or 0),
            delta_rotation=float(data.get("delta_rotation") or 0),
            pan_x=float(data.get("pan_x") or 0),
            pan_y=float(data.get("pan_y") or 0),
        )

    def to_json(self):
        return {
            "phase": self.phase,
            "x": self.x,
            "y": self.y,
            "scale": self.scale,
            "delta_scale": self.delta_scale,
            "rotation": self.rotation,
            "delta_rotation": self.delta_rotation,
            "pan_x": self.pan_x,
            "pan_y": self.pan_y,
        }


@dataclass
class GridTotal:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    field: "str" = ""
    function: "GridTotalFunction" = dataclasses.field(default_factory=lambda: next(iter(GridTotalFunction)))

    @staticmethod
    def from_json(data):
        return GridTotal(
            field=str(data.get("field") or ""),
            function=_enum(GridTotalFunction, data.get("function")),
        )

    def to_json(self):
        return {
            "field": self.field,
            "function": self.function.value,
        }


@dataclass
class IndexDef:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    name: "str" = ""
    fields: "str" = ""

    @staticmethod
    def from_json(data):
        return IndexDef(
            name=str(data.get("name") or ""),
            fields=str(data.get("fields") or ""),
        )

    def to_json(self):
        return {
            "name": self.name,
            "fields": self.fields,
        }


@dataclass
class KeyEvent:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    key: "str" = ""
    ctrl: "bool" = False
    shift: "bool" = False
    alt: "bool" = False

    @staticmethod
    def from_json(data):
        return KeyEvent(
            key=str(data.get("key") or ""),
            ctrl=bool(data.get("ctrl") or False),
            shift=bool(data.get("shift") or False),
            alt=bool(data.get("alt") or False),
        )

    def to_json(self):
        return {
            "key": self.key,
            "ctrl": self.ctrl,
            "shift": self.shift,
            "alt": self.alt,
        }


@dataclass
class MasterField:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    field: "str" = ""
    master_field: "str" = ""

    @staticmethod
    def from_json(data):
        return MasterField(
            field=str(data.get("field") or ""),
            master_field=str(data.get("master_field") or ""),
        )

    def to_json(self):
        return {
            "field": self.field,
            "master_field": self.master_field,
        }


@dataclass
class PointerEvent:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    x: "float" = 0.0
    y: "float" = 0.0
    button: "str" = ""
    clicks: "int" = 0

    @staticmethod
    def from_json(data):
        return PointerEvent(
            x=float(data.get("x") or 0),
            y=float(data.get("y") or 0),
            button=str(data.get("button") or ""),
            clicks=int(data.get("clicks") or 0),
        )

    def to_json(self):
        return {
            "x": self.x,
            "y": self.y,
            "button": self.button,
            "clicks": self.clicks,
        }


@dataclass
class SwipeEvent:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    direction: "str" = ""
    x: "float" = 0.0
    y: "float" = 0.0
    dx: "float" = 0.0
    dy: "float" = 0.0
    vx: "float" = 0.0
    vy: "float" = 0.0

    @staticmethod
    def from_json(data):
        return SwipeEvent(
            direction=str(data.get("direction") or ""),
            x=float(data.get("x") or 0),
            y=float(data.get("y") or 0),
            dx=float(data.get("dx") or 0),
            dy=float(data.get("dy") or 0),
            vx=float(data.get("vx") or 0),
            vy=float(data.get("vy") or 0),
        )

    def to_json(self):
        return {
            "direction": self.direction,
            "x": self.x,
            "y": self.y,
            "dx": self.dx,
            "dy": self.dy,
            "vx": self.vx,
            "vy": self.vy,
        }


@dataclass
class WheelEvent:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    x: "float" = 0.0
    y: "float" = 0.0
    dx: "float" = 0.0
    dy: "float" = 0.0

    @staticmethod
    def from_json(data):
        return WheelEvent(
            x=float(data.get("x") or 0),
            y=float(data.get("y") or 0),
            dx=float(data.get("dx") or 0),
            dy=float(data.get("dy") or 0),
        )

    def to_json(self):
        return {
            "x": self.x,
            "y": self.y,
            "dx": self.dx,
            "dy": self.dy,
        }


class ClientDataSet(AylisComponent):
    """Um ClientDataSet de um form."""

    @property
    def key_fields(self) -> str:
        return self._get("key_fields")

    @key_fields.setter
    def key_fields(self, value: str) -> None:
        self._set("key_fields", value)

    @property
    def apply_mode(self) -> ApplyMode:
        return _enum(ApplyMode, self._get("apply_mode"))

    @apply_mode.setter
    def apply_mode(self, value: ApplyMode) -> None:
        self._set("apply_mode", value.value)

    @property
    def read_only(self) -> bool:
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value: bool) -> None:
        self._set("read_only", value)

    @property
    def active(self) -> bool:
        return self._get("active")

    @active.setter
    def active(self, value: bool) -> None:
        self._set("active", value)

    @property
    def filter(self) -> str:
        return self._get("filter")

    @filter.setter
    def filter(self, value: str) -> None:
        self._set("filter", value)

    @property
    def filtered(self) -> bool:
        return self._get("filtered")

    @filtered.setter
    def filtered(self, value: bool) -> None:
        self._set("filtered", value)

    @property
    def index_field_names(self) -> str:
        return self._get("index_field_names")

    @index_field_names.setter
    def index_field_names(self, value: str) -> None:
        self._set("index_field_names", value)

    @property
    def indexes(self) -> list[IndexDef]:
        return _records(IndexDef, self._get("indexes"))

    @indexes.setter
    def indexes(self, value: list[IndexDef]) -> None:
        self._set("indexes", _records_text(value))

    @property
    def index_name(self) -> str:
        return self._get("index_name")

    @index_name.setter
    def index_name(self, value: str) -> None:
        self._set("index_name", value)

    @property
    def params(self) -> list[FilterParam]:
        return _records(FilterParam, self._get("params"))

    @params.setter
    def params(self, value: list[FilterParam]) -> None:
        self._set("params", _records_text(value))

    @property
    def status_filter(self) -> str:
        return self._get("status_filter")

    @status_filter.setter
    def status_filter(self, value: str) -> None:
        self._set("status_filter", value)

    @property
    def master_source(self) -> AylisComponent | None:
        return self._get_ref("master_source")

    @master_source.setter
    def master_source(self, value: AylisComponent | None) -> None:
        self._set_ref("master_source", value)

    @property
    def master_fields(self) -> list[MasterField]:
        return _records(MasterField, self._get("master_fields"))

    @master_fields.setter
    def master_fields(self, value: list[MasterField]) -> None:
        self._set("master_fields", _records_text(value))

    @property
    def apply_separately(self) -> bool:
        return self._get("apply_separately")

    @apply_separately.setter
    def apply_separately(self, value: bool) -> None:
        self._set("apply_separately", value)

    @property
    def record_total(self) -> int:
        return self._get("record_total")

    @record_total.setter
    def record_total(self, value: int) -> None:
        self._set("record_total", value)

    @property
    def max_records(self) -> int:
        return self._get("max_records")

    @max_records.setter
    def max_records(self, value: int) -> None:
        self._set("max_records", value)

    @property
    def partial_notice(self) -> str:
        return self._get("partial_notice")

    @property
    def on_master_change(self) -> str:
        return self._get("on_master_change")

    @on_master_change.setter
    def on_master_change(self, value: str) -> None:
        self._set("on_master_change", value)

    @property
    def after_scroll(self) -> str:
        return self._get("after_scroll")

    @after_scroll.setter
    def after_scroll(self, value: str) -> None:
        self._set("after_scroll", value)

    @property
    def after_post(self) -> str:
        return self._get("after_post")

    @after_post.setter
    def after_post(self, value: str) -> None:
        self._set("after_post", value)

    @property
    def on_state_change(self) -> str:
        return self._get("on_state_change")

    @on_state_change.setter
    def on_state_change(self, value: str) -> None:
        self._set("on_state_change", value)

    @property
    def on_reconcile_error(self) -> str:
        return self._get("on_reconcile_error")

    @on_reconcile_error.setter
    def on_reconcile_error(self, value: str) -> None:
        self._set("on_reconcile_error", value)

    @property
    def on_error(self) -> str:
        return self._get("on_error")

    @on_error.setter
    def on_error(self, value: str) -> None:
        self._set("on_error", value)

    @property
    def on_fetch(self) -> str:
        return self._get("on_fetch")

    @on_fetch.setter
    def on_fetch(self, value: str) -> None:
        self._set("on_fetch", value)

    @property
    def on_server_query(self) -> str:
        return self._get("on_server_query")

    @on_server_query.setter
    def on_server_query(self, value: str) -> None:
        self._set("on_server_query", value)


class DBChart(AylisComponent):
    """Um DBChart de um form."""

    @property
    def data_source(self) -> AylisComponent | None:
        return self._get_ref("data_source")

    @data_source.setter
    def data_source(self, value: AylisComponent | None) -> None:
        self._set_ref("data_source", value)

    @property
    def label_field(self) -> str:
        return self._get("label_field")

    @label_field.setter
    def label_field(self, value: str) -> None:
        self._set("label_field", value)

    @property
    def value_fields(self) -> list[str]:
        return _list(self._get("value_fields"))

    @value_fields.setter
    def value_fields(self, value: list[str]) -> None:
        self._set("value_fields", ";".join(value))

    @property
    def kind(self) -> ChartKind:
        return _enum(ChartKind, self._get("kind"))

    @kind.setter
    def kind(self, value: ChartKind) -> None:
        self._set("kind", value.value)

    @property
    def title(self) -> str:
        return self._get("title")

    @title.setter
    def title(self, value: str) -> None:
        self._set("title", value)

    @property
    def labels(self) -> list[str]:
        return _list(self._get("labels"))

    @labels.setter
    def labels(self, value: list[str]) -> None:
        self._set("labels", ";".join(value))

    @property
    def series(self) -> list[ChartSeries]:
        return _records(ChartSeries, self._get("series"))

    @series.setter
    def series(self, value: list[ChartSeries]) -> None:
        self._set("series", _records_text(value))

    @property
    def json(self) -> str:
        return self._get("json")

    @json.setter
    def json(self, value: str) -> None:
        self._set("json", value)

    @property
    def legend(self) -> bool:
        return self._get("legend")

    @legend.setter
    def legend(self, value: bool) -> None:
        self._set("legend", value)

    @property
    def stepped(self) -> bool:
        return self._get("stepped")

    @stepped.setter
    def stepped(self, value: bool) -> None:
        self._set("stepped", value)

    @property
    def tension(self) -> float:
        return self._get("tension")

    @tension.setter
    def tension(self, value: float) -> None:
        self._set("tension", value)

    @property
    def animate(self) -> bool:
        return self._get("animate")

    @animate.setter
    def animate(self, value: bool) -> None:
        self._set("animate", value)

    @property
    def zoom(self) -> bool:
        return self._get("zoom")

    @zoom.setter
    def zoom(self, value: bool) -> None:
        self._set("zoom", value)

    @property
    def auto_scale(self) -> bool:
        return self._get("auto_scale")

    @auto_scale.setter
    def auto_scale(self, value: bool) -> None:
        self._set("auto_scale", value)

    @property
    def y_min(self) -> float:
        return self._get("y_min")

    @y_min.setter
    def y_min(self, value: float) -> None:
        self._set("y_min", value)

    @property
    def y_max(self) -> float:
        return self._get("y_max")

    @y_max.setter
    def y_max(self, value: float) -> None:
        self._set("y_max", value)

    @property
    def log_scale(self) -> bool:
        return self._get("log_scale")

    @log_scale.setter
    def log_scale(self, value: bool) -> None:
        self._set("log_scale", value)

    @property
    def reverse(self) -> bool:
        return self._get("reverse")

    @reverse.setter
    def reverse(self, value: bool) -> None:
        self._set("reverse", value)

    @property
    def time_axis(self) -> bool:
        return self._get("time_axis")

    @time_axis.setter
    def time_axis(self, value: bool) -> None:
        self._set("time_axis", value)

    @property
    def clicked_series(self) -> int:
        return self._get("clicked_series")

    @clicked_series.setter
    def clicked_series(self, value: int) -> None:
        self._set("clicked_series", value)

    @property
    def clicked_index(self) -> int:
        return self._get("clicked_index")

    @clicked_index.setter
    def clicked_index(self, value: int) -> None:
        self._set("clicked_index", value)

    @property
    def value_format(self) -> ValueFormat:
        return _enum(ValueFormat, self._get("value_format"))

    @value_format.setter
    def value_format(self, value: ValueFormat) -> None:
        self._set("value_format", value.value)

    @property
    def label_angle(self) -> float:
        return self._get("label_angle")

    @label_angle.setter
    def label_angle(self, value: float) -> None:
        self._set("label_angle", value)

    @property
    def title_align(self) -> TitleAlign:
        return _enum(TitleAlign, self._get("title_align"))

    @title_align.setter
    def title_align(self, value: TitleAlign) -> None:
        self._set("title_align", value.value)

    @property
    def title_bold(self) -> bool:
        return self._get("title_bold")

    @title_bold.setter
    def title_bold(self, value: bool) -> None:
        self._set("title_bold", value)

    @property
    def area_opacity(self) -> float:
        return self._get("area_opacity")

    @area_opacity.setter
    def area_opacity(self, value: float) -> None:
        self._set("area_opacity", value)

    @property
    def legend_position(self) -> LegendPosition:
        return _enum(LegendPosition, self._get("legend_position"))

    @legend_position.setter
    def legend_position(self, value: LegendPosition) -> None:
        self._set("legend_position", value.value)

    @property
    def stacked(self) -> bool:
        return self._get("stacked")

    @stacked.setter
    def stacked(self, value: bool) -> None:
        self._set("stacked", value)

    @property
    def colors(self) -> str:
        return self._get("colors")

    @colors.setter
    def colors(self, value: str) -> None:
        self._set("colors", value)

    @property
    def center_text(self) -> str:
        return self._get("center_text")

    @center_text.setter
    def center_text(self, value: str) -> None:
        self._set("center_text", value)

    @property
    def slice_labels(self) -> str:
        return self._get("slice_labels")

    @slice_labels.setter
    def slice_labels(self, value: str) -> None:
        self._set("slice_labels", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> DBChartStyle:
        return _enum(DBChartStyle, self._get("style"))

    @style.setter
    def style(self, value: DBChartStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class DataGrid(AylisComponent):
    """Um DataGrid de um form."""

    @property
    def overscroll(self) -> Overscroll:
        return _enum(Overscroll, self._get("overscroll"))

    @overscroll.setter
    def overscroll(self, value: Overscroll) -> None:
        self._set("overscroll", value.value)

    @property
    def data_source(self) -> AylisComponent | None:
        return self._get_ref("data_source")

    @data_source.setter
    def data_source(self, value: AylisComponent | None) -> None:
        self._set_ref("data_source", value)

    @property
    def columns(self) -> list[ColumnSpec]:
        return _records(ColumnSpec, self._get("columns"))

    @columns.setter
    def columns(self, value: list[ColumnSpec]) -> None:
        self._set("columns", _records_text(value))

    @property
    def row_height(self) -> float:
        return self._get("row_height")

    @row_height.setter
    def row_height(self, value: float) -> None:
        self._set("row_height", value)

    @property
    def read_only(self) -> bool:
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value: bool) -> None:
        self._set("read_only", value)

    @property
    def sortable(self) -> bool:
        return self._get("sortable")

    @sortable.setter
    def sortable(self, value: bool) -> None:
        self._set("sortable", value)

    @property
    def column_move(self) -> bool:
        return self._get("column_move")

    @column_move.setter
    def column_move(self, value: bool) -> None:
        self._set("column_move", value)

    @property
    def fixed_columns(self) -> int:
        return self._get("fixed_columns")

    @fixed_columns.setter
    def fixed_columns(self, value: int) -> None:
        self._set("fixed_columns", value)

    @property
    def filter_row(self) -> bool:
        return self._get("filter_row")

    @filter_row.setter
    def filter_row(self, value: bool) -> None:
        self._set("filter_row", value)

    @property
    def multi_select(self) -> bool:
        return self._get("multi_select")

    @multi_select.setter
    def multi_select(self, value: bool) -> None:
        self._set("multi_select", value)

    @property
    def footer(self) -> list[GridTotal]:
        return _records(GridTotal, self._get("footer"))

    @footer.setter
    def footer(self, value: list[GridTotal]) -> None:
        self._set("footer", _records_text(value))

    @property
    def incremental_search(self) -> bool:
        return self._get("incremental_search")

    @incremental_search.setter
    def incremental_search(self, value: bool) -> None:
        self._set("incremental_search", value)

    @property
    def cursor_color(self) -> Color:
        return Color.from_json(self._get("cursor_color"))

    @cursor_color.setter
    def cursor_color(self, value: Color) -> None:
        self._set("cursor_color", value.to_json())

    @property
    def indicator(self) -> bool:
        return self._get("indicator")

    @indicator.setter
    def indicator(self, value: bool) -> None:
        self._set("indicator", value)

    @property
    def highlight_current(self) -> bool:
        return self._get("highlight_current")

    @highlight_current.setter
    def highlight_current(self, value: bool) -> None:
        self._set("highlight_current", value)

    @property
    def horizontal_wheel(self) -> HorizontalWheel:
        return _enum(HorizontalWheel, self._get("horizontal_wheel"))

    @horizontal_wheel.setter
    def horizontal_wheel(self, value: HorizontalWheel) -> None:
        self._set("horizontal_wheel", value.value)

    @property
    def on_error(self) -> str:
        return self._get("on_error")

    @on_error.setter
    def on_error(self, value: str) -> None:
        self._set("on_error", value)

    @property
    def on_title_click(self) -> str:
        return self._get("on_title_click")

    @on_title_click.setter
    def on_title_click(self, value: str) -> None:
        self._set("on_title_click", value)

    @property
    def on_selection_change(self) -> str:
        return self._get("on_selection_change")

    @on_selection_change.setter
    def on_selection_change(self, value: str) -> None:
        self._set("on_selection_change", value)

    @property
    def on_column_moved(self) -> str:
        return self._get("on_column_moved")

    @on_column_moved.setter
    def on_column_moved(self, value: str) -> None:
        self._set("on_column_moved", value)

    @property
    def scroll_left(self) -> float:
        return self._get("scroll_left")

    @scroll_left.setter
    def scroll_left(self, value: float) -> None:
        self._set("scroll_left", value)

    @property
    def top_row(self) -> int:
        return self._get("top_row")

    @top_row.setter
    def top_row(self, value: int) -> None:
        self._set("top_row", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> DataGridStyle:
        return _enum(DataGridStyle, self._get("style"))

    @style.setter
    def style(self, value: DataGridStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)

    def set_focus(self) -> None:
        self.aylis.send("focus", target=self.id)


class DataSource(AylisComponent):
    """Um DataSource de um form."""

    @property
    def data_set(self) -> AylisComponent | None:
        return self._get_ref("data_set")

    @data_set.setter
    def data_set(self, value: AylisComponent | None) -> None:
        self._set_ref("data_set", value)

    @property
    def auto_edit(self) -> bool:
        return self._get("auto_edit")

    @auto_edit.setter
    def auto_edit(self, value: bool) -> None:
        self._set("auto_edit", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def on_data_change(self) -> str:
        return self._get("on_data_change")

    @on_data_change.setter
    def on_data_change(self, value: str) -> None:
        self._set("on_data_change", value)

    @property
    def on_state_change(self) -> str:
        return self._get("on_state_change")

    @on_state_change.setter
    def on_state_change(self, value: str) -> None:
        self._set("on_state_change", value)


class Field(AylisComponent):
    """Um Field de um form."""

    @property
    def field_name(self) -> str:
        return self._get("field_name")

    @field_name.setter
    def field_name(self, value: str) -> None:
        self._set("field_name", value)

    @property
    def type(self) -> FieldType:
        return _enum(FieldType, self._get("type"))

    @type.setter
    def type(self, value: FieldType) -> None:
        self._set("type", value.value)

    @property
    def size(self) -> int:
        return self._get("size")

    @size.setter
    def size(self, value: int) -> None:
        self._set("size", value)

    @property
    def scale(self) -> int:
        return self._get("scale")

    @scale.setter
    def scale(self, value: int) -> None:
        self._set("scale", value)

    @property
    def rounding(self) -> Rounding:
        return _enum(Rounding, self._get("rounding"))

    @rounding.setter
    def rounding(self, value: Rounding) -> None:
        self._set("rounding", value.value)

    @property
    def display_label(self) -> str:
        return self._get("display_label")

    @display_label.setter
    def display_label(self, value: str) -> None:
        self._set("display_label", value)

    @property
    def display_width(self) -> float:
        return self._get("display_width")

    @display_width.setter
    def display_width(self, value: float) -> None:
        self._set("display_width", value)

    @property
    def display_format(self) -> str:
        return self._get("display_format")

    @display_format.setter
    def display_format(self, value: str) -> None:
        self._set("display_format", value)

    @property
    def edit_format(self) -> str:
        return self._get("edit_format")

    @edit_format.setter
    def edit_format(self, value: str) -> None:
        self._set("edit_format", value)

    @property
    def alignment(self) -> TextAlign:
        return _enum(TextAlign, self._get("alignment"))

    @alignment.setter
    def alignment(self, value: TextAlign) -> None:
        self._set("alignment", value.value)

    @property
    def required(self) -> bool:
        return self._get("required")

    @required.setter
    def required(self, value: bool) -> None:
        self._set("required", value)

    @property
    def read_only(self) -> bool:
        return self._get("read_only")

    @read_only.setter
    def read_only(self, value: bool) -> None:
        self._set("read_only", value)

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def min(self) -> float:
        return self._get("min")

    @min.setter
    def min(self, value: float) -> None:
        self._set("min", value)

    @property
    def max(self) -> float:
        return self._get("max")

    @max.setter
    def max(self, value: float) -> None:
        self._set("max", value)

    @property
    def default_value(self) -> str:
        return self._get("default_value")

    @default_value.setter
    def default_value(self, value: str) -> None:
        self._set("default_value", value)

    @property
    def display_values(self) -> str:
        return self._get("display_values")

    @display_values.setter
    def display_values(self, value: str) -> None:
        self._set("display_values", value)

    @property
    def char_case(self) -> CharCase:
        return _enum(CharCase, self._get("char_case"))

    @char_case.setter
    def char_case(self, value: CharCase) -> None:
        self._set("char_case", value.value)

    @property
    def edit_mask(self) -> str:
        return self._get("edit_mask")

    @edit_mask.setter
    def edit_mask(self, value: str) -> None:
        self._set("edit_mask", value)

    @property
    def sensitive(self) -> bool:
        return self._get("sensitive")

    @sensitive.setter
    def sensitive(self, value: bool) -> None:
        self._set("sensitive", value)

    @property
    def kind(self) -> FieldKind:
        return _enum(FieldKind, self._get("kind"))

    @kind.setter
    def kind(self, value: FieldKind) -> None:
        self._set("kind", value.value)

    @property
    def expression(self) -> str:
        return self._get("expression")

    @expression.setter
    def expression(self, value: str) -> None:
        self._set("expression", value)

    @property
    def lookup_data_set(self) -> AylisComponent | None:
        return self._get_ref("lookup_data_set")

    @lookup_data_set.setter
    def lookup_data_set(self, value: AylisComponent | None) -> None:
        self._set_ref("lookup_data_set", value)

    @property
    def key_fields(self) -> str:
        return self._get("key_fields")

    @key_fields.setter
    def key_fields(self, value: str) -> None:
        self._set("key_fields", value)

    @property
    def lookup_key_fields(self) -> str:
        return self._get("lookup_key_fields")

    @lookup_key_fields.setter
    def lookup_key_fields(self, value: str) -> None:
        self._set("lookup_key_fields", value)

    @property
    def lookup_result_field(self) -> str:
        return self._get("lookup_result_field")

    @lookup_result_field.setter
    def lookup_result_field(self, value: str) -> None:
        self._set("lookup_result_field", value)

    @property
    def on_change(self) -> str:
        return self._get("on_change")

    @on_change.setter
    def on_change(self, value: str) -> None:
        self._set("on_change", value)

    @property
    def on_get_text(self) -> str:
        return self._get("on_get_text")

    @on_get_text.setter
    def on_get_text(self, value: str) -> None:
        self._set("on_get_text", value)

    @property
    def on_set_text(self) -> str:
        return self._get("on_set_text")

    @on_set_text.setter
    def on_set_text(self, value: str) -> None:
        self._set("on_set_text", value)


class Form(AylisForm):
    """Um Form de um form."""

    @property
    def caption(self) -> str:
        return self._get("caption")

    @caption.setter
    def caption(self, value: str) -> None:
        self._set("caption", value)

    @property
    def position(self) -> FormPosition:
        return _enum(FormPosition, self._get("position"))

    @position.setter
    def position(self, value: FormPosition) -> None:
        self._set("position", value.value)

    @property
    def border_style(self) -> BorderStyle:
        return _enum(BorderStyle, self._get("border_style"))

    @border_style.setter
    def border_style(self, value: BorderStyle) -> None:
        self._set("border_style", value.value)

    @property
    def window_corners(self) -> WindowCorners:
        return _enum(WindowCorners, self._get("window_corners"))

    @window_corners.setter
    def window_corners(self, value: WindowCorners) -> None:
        self._set("window_corners", value.value)

    @property
    def transparent(self) -> bool:
        return self._get("transparent")

    @transparent.setter
    def transparent(self, value: bool) -> None:
        self._set("transparent", value)

    @property
    def window_shape(self) -> WindowShape:
        return _enum(WindowShape, self._get("window_shape"))

    @window_shape.setter
    def window_shape(self, value: WindowShape) -> None:
        self._set("window_shape", value.value)

    @property
    def shape_radius(self) -> float:
        return self._get("shape_radius")

    @shape_radius.setter
    def shape_radius(self, value: float) -> None:
        self._set("shape_radius", value)

    @property
    def shape_radii(self) -> Bounds:
        return Bounds.from_json(self._get("shape_radii"))

    @shape_radii.setter
    def shape_radii(self, value: Bounds) -> None:
        self._set("shape_radii", value.to_json())

    @property
    def shape_path(self) -> str:
        return self._get("shape_path")

    @shape_path.setter
    def shape_path(self, value: str) -> None:
        self._set("shape_path", value)

    @property
    def window_state(self) -> WindowState:
        return _enum(WindowState, self._get("window_state"))

    @window_state.setter
    def window_state(self, value: WindowState) -> None:
        self._set("window_state", value.value)

    @property
    def transition(self) -> Transition:
        return _enum(Transition, self._get("transition"))

    @transition.setter
    def transition(self, value: Transition) -> None:
        self._set("transition", value.value)

    @property
    def layer_cache(self) -> LayerCaching:
        return _enum(LayerCaching, self._get("layer_cache"))

    @layer_cache.setter
    def layer_cache(self, value: LayerCaching) -> None:
        self._set("layer_cache", value.value)

    @property
    def frame_reuse_kb(self) -> int:
        return self._get("frame_reuse_kb")

    @frame_reuse_kb.setter
    def frame_reuse_kb(self, value: int) -> None:
        self._set("frame_reuse_kb", value)

    @property
    def topmost(self) -> bool:
        return self._get("topmost")

    @topmost.setter
    def topmost(self, value: bool) -> None:
        self._set("topmost", value)

    @property
    def show_in_taskbar(self) -> bool:
        return self._get("show_in_taskbar")

    @show_in_taskbar.setter
    def show_in_taskbar(self, value: bool) -> None:
        self._set("show_in_taskbar", value)

    @property
    def active_control(self) -> AylisComponent | None:
        return self._get_ref("active_control")

    @active_control.setter
    def active_control(self, value: AylisComponent | None) -> None:
        self._set_ref("active_control", value)

    @property
    def tab_order(self) -> list[str]:
        return _list(self._get("tab_order"))

    @tab_order.setter
    def tab_order(self, value: list[str]) -> None:
        self._set("tab_order", ";".join(value))

    @property
    def tab_keys(self) -> str:
        return self._get("tab_keys")

    @tab_keys.setter
    def tab_keys(self, value: str) -> None:
        self._set("tab_keys", value)

    @property
    def back_tab_keys(self) -> str:
        return self._get("back_tab_keys")

    @back_tab_keys.setter
    def back_tab_keys(self, value: str) -> None:
        self._set("back_tab_keys", value)

    @property
    def allowed_chars(self) -> str:
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value: str) -> None:
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self) -> str:
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value: str) -> None:
        self._set("blocked_chars", value)

    @property
    def only_allowed(self) -> bool:
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value: bool) -> None:
        self._set("only_allowed", value)

    @property
    def chars_override(self) -> bool:
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value: bool) -> None:
        self._set("chars_override", value)

    @property
    def char_map(self) -> str:
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value: str) -> None:
        self._set("char_map", value)

    @property
    def key_preview(self) -> bool:
        return self._get("key_preview")

    @key_preview.setter
    def key_preview(self, value: bool) -> None:
        self._set("key_preview", value)

    @property
    def zoom(self) -> float:
        return self._get("zoom")

    @zoom.setter
    def zoom(self, value: float) -> None:
        self._set("zoom", value)

    @property
    def monitor(self) -> int:
        return self._get("monitor")

    @monitor.setter
    def monitor(self, value: int) -> None:
        self._set("monitor", value)

    @property
    def remember_placement(self) -> bool:
        return self._get("remember_placement")

    @remember_placement.setter
    def remember_placement(self, value: bool) -> None:
        self._set("remember_placement", value)

    @property
    def gpu(self) -> str:
        return self._get("gpu")

    @gpu.setter
    def gpu(self, value: str) -> None:
        self._set("gpu", value)

    @property
    def present(self) -> FormPresent:
        return _enum(FormPresent, self._get("present"))

    @present.setter
    def present(self, value: FormPresent) -> None:
        self._set("present", value.value)

    @property
    def span_monitors(self) -> bool:
        return self._get("span_monitors")

    @span_monitors.setter
    def span_monitors(self, value: bool) -> None:
        self._set("span_monitors", value)

    @property
    def physical_pixels(self) -> bool:
        return self._get("physical_pixels")

    @physical_pixels.setter
    def physical_pixels(self, value: bool) -> None:
        self._set("physical_pixels", value)

    @property
    def activate_on_show(self) -> bool:
        return self._get("activate_on_show")

    @activate_on_show.setter
    def activate_on_show(self, value: bool) -> None:
        self._set("activate_on_show", value)

    @property
    def force_foreground(self) -> bool:
        return self._get("force_foreground")

    @force_foreground.setter
    def force_foreground(self, value: bool) -> None:
        self._set("force_foreground", value)

    @property
    def close_action(self) -> CloseAction:
        return _enum(CloseAction, self._get("close_action"))

    @close_action.setter
    def close_action(self, value: CloseAction) -> None:
        self._set("close_action", value.value)

    @property
    def frame_rate(self) -> float:
        return self._get("frame_rate")

    @frame_rate.setter
    def frame_rate(self, value: float) -> None:
        self._set("frame_rate", value)

    @property
    def orientation(self) -> FormOrientation:
        return _enum(FormOrientation, self._get("orientation"))

    @orientation.setter
    def orientation(self, value: FormOrientation) -> None:
        self._set("orientation", value.value)

    @property
    def on_show(self) -> str:
        return self._get("on_show")

    @on_show.setter
    def on_show(self, value: str) -> None:
        self._set("on_show", value)

    @property
    def on_window_created(self) -> str:
        return self._get("on_window_created")

    @on_window_created.setter
    def on_window_created(self, value: str) -> None:
        self._set("on_window_created", value)

    @property
    def on_close(self) -> str:
        return self._get("on_close")

    @on_close.setter
    def on_close(self, value: str) -> None:
        self._set("on_close", value)

    @property
    def on_activate(self) -> str:
        return self._get("on_activate")

    @on_activate.setter
    def on_activate(self, value: str) -> None:
        self._set("on_activate", value)

    @property
    def on_deactivate(self) -> str:
        return self._get("on_deactivate")

    @on_deactivate.setter
    def on_deactivate(self, value: str) -> None:
        self._set("on_deactivate", value)

    @property
    def on_minimize(self) -> str:
        return self._get("on_minimize")

    @on_minimize.setter
    def on_minimize(self, value: str) -> None:
        self._set("on_minimize", value)

    @property
    def on_restore(self) -> str:
        return self._get("on_restore")

    @on_restore.setter
    def on_restore(self, value: str) -> None:
        self._set("on_restore", value)

    @property
    def on_navigate(self) -> str:
        return self._get("on_navigate")

    @on_navigate.setter
    def on_navigate(self, value: str) -> None:
        self._set("on_navigate", value)

    @property
    def on_gpu_change(self) -> str:
        return self._get("on_gpu_change")

    @on_gpu_change.setter
    def on_gpu_change(self, value: str) -> None:
        self._set("on_gpu_change", value)

    @property
    def client_width(self) -> float:
        return self._get("client_width")

    @client_width.setter
    def client_width(self, value: float) -> None:
        self._set("client_width", value)

    @property
    def client_height(self) -> float:
        return self._get("client_height")

    @client_height.setter
    def client_height(self, value: float) -> None:
        self._set("client_height", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> FormStyle:
        return _enum(FormStyle, self._get("style"))

    @style.setter
    def style(self, value: FormStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)

    def move_to_monitor(self, monitor: int) -> None:
        self.aylis.send("invoke", target=self.id, method="move_to_monitor", args=[monitor])

    def gpu_adapter(self) -> str:
        return self.aylis.call("invoke", target=self.id, method="gpu_adapter", args=[])

    def gpu_adapters(self) -> list[str]:
        return _list(self.aylis.call("invoke", target=self.id, method="gpu_adapters", args=[]))

    def render_rgba(self, x: float, y: float, width: float, height: float, scale: float) -> str:
        return self.aylis.call("invoke", target=self.id, method="render_rgba", args=[x, y, width, height, scale])

    def focused(self) -> str:
        return self.aylis.call("focused", target=self.id)

    def focus_next(self) -> None:
        self.aylis.send("focus_next", target=self.id)

    def focus_prev(self) -> None:
        self.aylis.send("focus_prev", target=self.id)


class GroupBox(AylisComponent):
    """Um GroupBox de um form."""

    @property
    def caption(self) -> str:
        return self._get("caption")

    @caption.setter
    def caption(self, value: str) -> None:
        self._set("caption", value)

    @property
    def allowed_chars(self) -> str:
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value: str) -> None:
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self) -> str:
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value: str) -> None:
        self._set("blocked_chars", value)

    @property
    def only_allowed(self) -> bool:
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value: bool) -> None:
        self._set("only_allowed", value)

    @property
    def chars_override(self) -> bool:
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value: bool) -> None:
        self._set("chars_override", value)

    @property
    def char_map(self) -> str:
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value: str) -> None:
        self._set("char_map", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> GroupBoxStyle:
        return _enum(GroupBoxStyle, self._get("style"))

    @style.setter
    def style(self, value: GroupBoxStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class Image(AylisComponent):
    """Um Image de um form."""

    @property
    def image(self) -> str:
        return self._get("image")

    @image.setter
    def image(self, value: str) -> None:
        self._set("image", value)

    @property
    def fit(self) -> ImageFit:
        return _enum(ImageFit, self._get("fit"))

    @fit.setter
    def fit(self, value: ImageFit) -> None:
        self._set("fit", value.value)

    @property
    def radius(self) -> float:
        return self._get("radius")

    @radius.setter
    def radius(self, value: float) -> None:
        self._set("radius", value)

    @property
    def zoomable(self) -> bool:
        return self._get("zoomable")

    @zoomable.setter
    def zoomable(self, value: bool) -> None:
        self._set("zoomable", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> ImageStyle:
        return _enum(ImageStyle, self._get("style"))

    @style.setter
    def style(self, value: ImageStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class Label(AylisComponent):
    """Um Label de um form."""

    @property
    def text(self) -> str:
        return self._get("text")

    @text.setter
    def text(self, value: str) -> None:
        self._set("text", value)

    @property
    def h_align(self) -> TextAlign:
        return _enum(TextAlign, self._get("h_align"))

    @h_align.setter
    def h_align(self, value: TextAlign) -> None:
        self._set("h_align", value.value)

    @property
    def v_align(self) -> TextAlign:
        return _enum(TextAlign, self._get("v_align"))

    @v_align.setter
    def v_align(self, value: TextAlign) -> None:
        self._set("v_align", value.value)

    @property
    def wrap(self) -> bool:
        return self._get("wrap")

    @wrap.setter
    def wrap(self, value: bool) -> None:
        self._set("wrap", value)

    @property
    def trimming(self) -> Trimming:
        return _enum(Trimming, self._get("trimming"))

    @trimming.setter
    def trimming(self, value: Trimming) -> None:
        self._set("trimming", value.value)

    @property
    def bold(self) -> bool:
        return self._get("bold")

    @bold.setter
    def bold(self, value: bool) -> None:
        self._set("bold", value)

    @property
    def color(self) -> Color:
        return Color.from_json(self._get("color"))

    @color.setter
    def color(self, value: Color) -> None:
        self._set("color", value.to_json())

    @property
    def highlight(self) -> str:
        return self._get("highlight")

    @highlight.setter
    def highlight(self, value: str) -> None:
        self._set("highlight", value)

    @property
    def letter_spacing(self) -> float:
        return self._get("letter_spacing")

    @letter_spacing.setter
    def letter_spacing(self, value: float) -> None:
        self._set("letter_spacing", value)

    @property
    def text_case(self) -> TextCase:
        return _enum(TextCase, self._get("text_case"))

    @text_case.setter
    def text_case(self, value: TextCase) -> None:
        self._set("text_case", value.value)

    @property
    def markup(self) -> bool:
        return self._get("markup")

    @markup.setter
    def markup(self, value: bool) -> None:
        self._set("markup", value)

    @property
    def outline(self) -> Color:
        return Color.from_json(self._get("outline"))

    @outline.setter
    def outline(self, value: Color) -> None:
        self._set("outline", value.to_json())

    @property
    def outline_width(self) -> float:
        return self._get("outline_width")

    @outline_width.setter
    def outline_width(self, value: float) -> None:
        self._set("outline_width", value)

    @property
    def line_height(self) -> float:
        return self._get("line_height")

    @line_height.setter
    def line_height(self, value: float) -> None:
        self._set("line_height", value)

    @property
    def text_shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("text_shadow"))

    @text_shadow.setter
    def text_shadow(self, value: Shadow | None) -> None:
        self._set("text_shadow", (value.to_json() if value else ""))

    @property
    def focus_control(self) -> AylisComponent | None:
        return self._get_ref("focus_control")

    @focus_control.setter
    def focus_control(self, value: AylisComponent | None) -> None:
        self._set_ref("focus_control", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> LabelStyle:
        return _enum(LabelStyle, self._get("style"))

    @style.setter
    def style(self, value: LabelStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)

    def min_content_width(self) -> float:
        return self.aylis.call("invoke", target=self.id, method="min_content_width", args=[])

    def max_content_width(self) -> float:
        return self.aylis.call("invoke", target=self.id, method="max_content_width", args=[])


class Layout(AylisComponent):
    """Um Layout de um form."""

    @property
    def allowed_chars(self) -> str:
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value: str) -> None:
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self) -> str:
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value: str) -> None:
        self._set("blocked_chars", value)

    @property
    def only_allowed(self) -> bool:
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value: bool) -> None:
        self._set("only_allowed", value)

    @property
    def chars_override(self) -> bool:
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value: bool) -> None:
        self._set("chars_override", value)

    @property
    def char_map(self) -> str:
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value: str) -> None:
        self._set("char_map", value)

    @property
    def gap(self) -> float:
        return self._get("gap")

    @gap.setter
    def gap(self, value: float) -> None:
        self._set("gap", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> LayoutStyle:
        return _enum(LayoutStyle, self._get("style"))

    @style.setter
    def style(self, value: LayoutStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class Panel(AylisComponent):
    """Um Panel de um form."""

    @property
    def allowed_chars(self) -> str:
        return self._get("allowed_chars")

    @allowed_chars.setter
    def allowed_chars(self, value: str) -> None:
        self._set("allowed_chars", value)

    @property
    def blocked_chars(self) -> str:
        return self._get("blocked_chars")

    @blocked_chars.setter
    def blocked_chars(self, value: str) -> None:
        self._set("blocked_chars", value)

    @property
    def only_allowed(self) -> bool:
        return self._get("only_allowed")

    @only_allowed.setter
    def only_allowed(self, value: bool) -> None:
        self._set("only_allowed", value)

    @property
    def chars_override(self) -> bool:
        return self._get("chars_override")

    @chars_override.setter
    def chars_override(self, value: bool) -> None:
        self._set("chars_override", value)

    @property
    def char_map(self) -> str:
        return self._get("char_map")

    @char_map.setter
    def char_map(self, value: str) -> None:
        self._set("char_map", value)

    @property
    def gap(self) -> float:
        return self._get("gap")

    @gap.setter
    def gap(self, value: float) -> None:
        self._set("gap", value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def fill2(self) -> Color:
        return Color.from_json(self._get("fill2"))

    @fill2.setter
    def fill2(self, value: Color) -> None:
        self._set("fill2", value.to_json())

    @property
    def fill_mid(self) -> Color:
        return Color.from_json(self._get("fill_mid"))

    @fill_mid.setter
    def fill_mid(self, value: Color) -> None:
        self._set("fill_mid", value.to_json())

    @property
    def fill_mid_at(self) -> float:
        return self._get("fill_mid_at")

    @fill_mid_at.setter
    def fill_mid_at(self, value: float) -> None:
        self._set("fill_mid_at", value)

    @property
    def gradient_angle(self) -> float:
        return self._get("gradient_angle")

    @gradient_angle.setter
    def gradient_angle(self, value: float) -> None:
        self._set("gradient_angle", value)

    @property
    def gradient_radial(self) -> bool:
        return self._get("gradient_radial")

    @gradient_radial.setter
    def gradient_radial(self, value: bool) -> None:
        self._set("gradient_radial", value)

    @property
    def stroke(self) -> Color:
        return Color.from_json(self._get("stroke"))

    @stroke.setter
    def stroke(self, value: Color) -> None:
        self._set("stroke", value.to_json())

    @property
    def stroke_width(self) -> float:
        return self._get("stroke_width")

    @stroke_width.setter
    def stroke_width(self, value: float) -> None:
        self._set("stroke_width", value)

    @property
    def radius(self) -> Bounds:
        return Bounds.from_json(self._get("radius"))

    @radius.setter
    def radius(self, value: Bounds) -> None:
        self._set("radius", value.to_json())

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> PanelStyle:
        return _enum(PanelStyle, self._get("style"))

    @style.setter
    def style(self, value: PanelStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class Splitter(AylisComponent):
    """Um Splitter de um form."""

    @property
    def min_size(self) -> float:
        return self._get("min_size")

    @min_size.setter
    def min_size(self, value: float) -> None:
        self._set("min_size", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> SplitterStyle:
        return _enum(SplitterStyle, self._get("style"))

    @style.setter
    def style(self, value: SplitterStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class Svg(AylisComponent):
    """Um Svg de um form."""

    @property
    def source(self) -> str:
        return self._get("source")

    @source.setter
    def source(self, value: str) -> None:
        self._set("source", value)

    @property
    def color(self) -> Color:
        return Color.from_json(self._get("color"))

    @color.setter
    def color(self, value: Color) -> None:
        self._set("color", value.to_json())

    @property
    def fit(self) -> ImageFit:
        return _enum(ImageFit, self._get("fit"))

    @fit.setter
    def fit(self, value: ImageFit) -> None:
        self._set("fit", value.value)

    @property
    def highlight(self) -> list[str]:
        return _list(self._get("highlight"))

    @highlight.setter
    def highlight(self, value: list[str]) -> None:
        self._set("highlight", ";".join(value))

    @property
    def highlight_color(self) -> Color:
        return Color.from_json(self._get("highlight_color"))

    @highlight_color.setter
    def highlight_color(self, value: Color) -> None:
        self._set("highlight_color", value.to_json())

    @property
    def zoom_to(self) -> str:
        return self._get("zoom_to")

    @zoom_to.setter
    def zoom_to(self, value: str) -> None:
        self._set("zoom_to", value)

    @property
    def left(self) -> float:
        return self._get("left")

    @left.setter
    def left(self, value: float) -> None:
        self._set("left", value)

    @property
    def top(self) -> float:
        return self._get("top")

    @top.setter
    def top(self, value: float) -> None:
        self._set("top", value)

    @property
    def width(self) -> float:
        return self._get("width")

    @width.setter
    def width(self, value: float) -> None:
        self._set("width", value)

    @property
    def height(self) -> float:
        return self._get("height")

    @height.setter
    def height(self, value: float) -> None:
        self._set("height", value)

    @property
    def align(self) -> Align:
        return _enum(Align, self._get("align"))

    @align.setter
    def align(self, value: Align) -> None:
        self._set("align", value.value)

    @property
    def anchors(self) -> list[Anchor]:
        return [_enum(Anchor, x) for x in (self._get("anchors") or [])]

    @anchors.setter
    def anchors(self, value: list[Anchor]) -> None:
        self._set("anchors", [x.value for x in value])

    @property
    def margins(self) -> Bounds:
        return Bounds.from_json(self._get("margins"))

    @margins.setter
    def margins(self, value: Bounds) -> None:
        self._set("margins", value.to_json())

    @property
    def padding(self) -> Bounds:
        return Bounds.from_json(self._get("padding"))

    @padding.setter
    def padding(self, value: Bounds) -> None:
        self._set("padding", value.to_json())

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def hit_test(self) -> bool:
        return self._get("hit_test")

    @hit_test.setter
    def hit_test(self, value: bool) -> None:
        self._set("hit_test", value)

    @property
    def clip_children(self) -> bool:
        return self._get("clip_children")

    @clip_children.setter
    def clip_children(self, value: bool) -> None:
        self._set("clip_children", value)

    @property
    def inherit_state(self) -> bool:
        return self._get("inherit_state")

    @inherit_state.setter
    def inherit_state(self, value: bool) -> None:
        self._set("inherit_state", value)

    @property
    def translate(self) -> bool:
        return self._get("translate")

    @translate.setter
    def translate(self, value: bool) -> None:
        self._set("translate", value)

    @property
    def captures_wheel(self) -> bool:
        return self._get("captures_wheel")

    @captures_wheel.setter
    def captures_wheel(self, value: bool) -> None:
        self._set("captures_wheel", value)

    @property
    def captures_gesture(self) -> bool:
        return self._get("captures_gesture")

    @captures_gesture.setter
    def captures_gesture(self, value: bool) -> None:
        self._set("captures_gesture", value)

    @property
    def can_focus(self) -> bool:
        return self._get("can_focus")

    @can_focus.setter
    def can_focus(self, value: bool) -> None:
        self._set("can_focus", value)

    @property
    def draggable(self) -> bool:
        return self._get("draggable")

    @draggable.setter
    def draggable(self, value: bool) -> None:
        self._set("draggable", value)

    @property
    def accepts_drops(self) -> bool:
        return self._get("accepts_drops")

    @accepts_drops.setter
    def accepts_drops(self, value: bool) -> None:
        self._set("accepts_drops", value)

    @property
    def drag_data(self) -> str:
        return self._get("drag_data")

    @drag_data.setter
    def drag_data(self, value: str) -> None:
        self._set("drag_data", value)

    @property
    def drag_label(self) -> str:
        return self._get("drag_label")

    @drag_label.setter
    def drag_label(self, value: str) -> None:
        self._set("drag_label", value)

    @property
    def drag_image(self) -> AylisComponent | None:
        return self._get_ref("drag_image")

    @drag_image.setter
    def drag_image(self, value: AylisComponent | None) -> None:
        self._set_ref("drag_image", value)

    @property
    def drop_types(self) -> str:
        return self._get("drop_types")

    @drop_types.setter
    def drop_types(self, value: str) -> None:
        self._set("drop_types", value)

    @property
    def tab_stop(self) -> bool:
        return self._get("tab_stop")

    @tab_stop.setter
    def tab_stop(self, value: bool) -> None:
        self._set("tab_stop", value)

    @property
    def cursor(self) -> CursorShape:
        return _enum(CursorShape, self._get("cursor"))

    @cursor.setter
    def cursor(self, value: CursorShape) -> None:
        self._set("cursor", value.value)

    @property
    def opacity(self) -> float:
        return self._get("opacity")

    @opacity.setter
    def opacity(self, value: float) -> None:
        self._set("opacity", value)

    @property
    def blend_mode(self) -> BlendMode:
        return _enum(BlendMode, self._get("blend_mode"))

    @blend_mode.setter
    def blend_mode(self, value: BlendMode) -> None:
        self._set("blend_mode", value.value)

    @property
    def cache_layer(self) -> bool:
        return self._get("cache_layer")

    @cache_layer.setter
    def cache_layer(self, value: bool) -> None:
        self._set("cache_layer", value)

    @property
    def rotation(self) -> float:
        return self._get("rotation")

    @rotation.setter
    def rotation(self, value: float) -> None:
        self._set("rotation", value)

    @property
    def scale(self) -> float:
        return self._get("scale")

    @scale.setter
    def scale(self, value: float) -> None:
        self._set("scale", value)

    @property
    def translate_x(self) -> float:
        return self._get("translate_x")

    @translate_x.setter
    def translate_x(self, value: float) -> None:
        self._set("translate_x", value)

    @property
    def translate_y(self) -> float:
        return self._get("translate_y")

    @translate_y.setter
    def translate_y(self, value: float) -> None:
        self._set("translate_y", value)

    @property
    def aspect_ratio(self) -> float:
        return self._get("aspect_ratio")

    @aspect_ratio.setter
    def aspect_ratio(self, value: float) -> None:
        self._set("aspect_ratio", value)

    @property
    def max_width(self) -> float:
        return self._get("max_width")

    @max_width.setter
    def max_width(self, value: float) -> None:
        self._set("max_width", value)

    @property
    def min_width(self) -> float:
        return self._get("min_width")

    @min_width.setter
    def min_width(self, value: float) -> None:
        self._set("min_width", value)

    @property
    def min_height(self) -> float:
        return self._get("min_height")

    @min_height.setter
    def min_height(self, value: float) -> None:
        self._set("min_height", value)

    @property
    def max_height(self) -> float:
        return self._get("max_height")

    @max_height.setter
    def max_height(self, value: float) -> None:
        self._set("max_height", value)

    @property
    def self_align(self) -> SelfAlign:
        return _enum(SelfAlign, self._get("self_align"))

    @self_align.setter
    def self_align(self, value: SelfAlign) -> None:
        self._set("self_align", value.value)

    @property
    def auto_size(self) -> bool:
        return self._get("auto_size")

    @auto_size.setter
    def auto_size(self, value: bool) -> None:
        self._set("auto_size", value)

    @property
    def column_span(self) -> int:
        return self._get("column_span")

    @column_span.setter
    def column_span(self, value: int) -> None:
        self._set("column_span", value)

    @property
    def grid_row(self) -> int:
        return self._get("grid_row")

    @grid_row.setter
    def grid_row(self, value: int) -> None:
        self._set("grid_row", value)

    @property
    def grid_column(self) -> int:
        return self._get("grid_column")

    @grid_column.setter
    def grid_column(self, value: int) -> None:
        self._set("grid_column", value)

    @property
    def row_span(self) -> int:
        return self._get("row_span")

    @row_span.setter
    def row_span(self, value: int) -> None:
        self._set("row_span", value)

    @property
    def grow(self) -> float:
        return self._get("grow")

    @grow.setter
    def grow(self, value: float) -> None:
        self._set("grow", value)

    @property
    def shadow(self) -> Shadow | None:
        return Shadow.from_json(self._get("shadow"))

    @shadow.setter
    def shadow(self, value: Shadow | None) -> None:
        self._set("shadow", (value.to_json() if value else ""))

    @property
    def style(self) -> SvgStyle:
        return _enum(SvgStyle, self._get("style"))

    @style.setter
    def style(self, value: SvgStyle) -> None:
        self._set("style", value.value)

    @property
    def theme(self) -> ThemeName:
        return _enum(ThemeName, self._get("theme"))

    @theme.setter
    def theme(self, value: ThemeName) -> None:
        self._set("theme", value.value)

    @property
    def fill(self) -> Color:
        return Color.from_json(self._get("fill"))

    @fill.setter
    def fill(self, value: Color) -> None:
        self._set("fill", value.to_json())

    @property
    def border_color(self) -> Color:
        return Color.from_json(self._get("border_color"))

    @border_color.setter
    def border_color(self, value: Color) -> None:
        self._set("border_color", value.to_json())

    @property
    def text_color(self) -> Color:
        return Color.from_json(self._get("text_color"))

    @text_color.setter
    def text_color(self, value: Color) -> None:
        self._set("text_color", value.to_json())

    @property
    def accent_color(self) -> Color:
        return Color.from_json(self._get("accent_color"))

    @accent_color.setter
    def accent_color(self, value: Color) -> None:
        self._set("accent_color", value.to_json())

    @property
    def success_color(self) -> Color:
        return Color.from_json(self._get("success_color"))

    @success_color.setter
    def success_color(self, value: Color) -> None:
        self._set("success_color", value.to_json())

    @property
    def warning_color(self) -> Color:
        return Color.from_json(self._get("warning_color"))

    @warning_color.setter
    def warning_color(self, value: Color) -> None:
        self._set("warning_color", value.to_json())

    @property
    def danger_color(self) -> Color:
        return Color.from_json(self._get("danger_color"))

    @danger_color.setter
    def danger_color(self, value: Color) -> None:
        self._set("danger_color", value.to_json())

    @property
    def font_family(self) -> str:
        return self._get("font_family")

    @font_family.setter
    def font_family(self, value: str) -> None:
        self._set("font_family", value)

    @property
    def font_size(self) -> float:
        return self._get("font_size")

    @font_size.setter
    def font_size(self, value: float) -> None:
        self._set("font_size", value)

    @property
    def font_weight(self) -> FontWeight:
        return _enum(FontWeight, self._get("font_weight"))

    @font_weight.setter
    def font_weight(self, value: FontWeight) -> None:
        self._set("font_weight", value.value)

    @property
    def font_italic(self) -> bool:
        return self._get("font_italic")

    @font_italic.setter
    def font_italic(self, value: bool) -> None:
        self._set("font_italic", value)

    @property
    def font_underline(self) -> bool:
        return self._get("font_underline")

    @font_underline.setter
    def font_underline(self, value: bool) -> None:
        self._set("font_underline", value)

    @property
    def font_strikeout(self) -> bool:
        return self._get("font_strikeout")

    @font_strikeout.setter
    def font_strikeout(self, value: bool) -> None:
        self._set("font_strikeout", value)

    @property
    def hint(self) -> str:
        return self._get("hint")

    @hint.setter
    def hint(self, value: str) -> None:
        self._set("hint", value)

    @property
    def tag(self) -> int:
        return self._get("tag")

    @tag.setter
    def tag(self, value: int) -> None:
        self._set("tag", value)

    @property
    def tag_string(self) -> str:
        return self._get("tag_string")

    @tag_string.setter
    def tag_string(self, value: str) -> None:
        self._set("tag_string", value)

    @property
    def accessible_name(self) -> str:
        return self._get("accessible_name")

    @accessible_name.setter
    def accessible_name(self, value: str) -> None:
        self._set("accessible_name", value)

    @property
    def popup_menu(self) -> AylisComponent | None:
        return self._get_ref("popup_menu")

    @popup_menu.setter
    def popup_menu(self, value: AylisComponent | None) -> None:
        self._set_ref("popup_menu", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)

    @property
    def on_dbl_click(self) -> str:
        return self._get("on_dbl_click")

    @on_dbl_click.setter
    def on_dbl_click(self, value: str) -> None:
        self._set("on_dbl_click", value)

    @property
    def on_long_press(self) -> str:
        return self._get("on_long_press")

    @on_long_press.setter
    def on_long_press(self, value: str) -> None:
        self._set("on_long_press", value)

    @property
    def on_pointer_down(self) -> str:
        return self._get("on_pointer_down")

    @on_pointer_down.setter
    def on_pointer_down(self, value: str) -> None:
        self._set("on_pointer_down", value)

    @property
    def on_pointer_up(self) -> str:
        return self._get("on_pointer_up")

    @on_pointer_up.setter
    def on_pointer_up(self, value: str) -> None:
        self._set("on_pointer_up", value)

    @property
    def on_pointer_move(self) -> str:
        return self._get("on_pointer_move")

    @on_pointer_move.setter
    def on_pointer_move(self, value: str) -> None:
        self._set("on_pointer_move", value)

    @property
    def on_pointer_enter(self) -> str:
        return self._get("on_pointer_enter")

    @on_pointer_enter.setter
    def on_pointer_enter(self, value: str) -> None:
        self._set("on_pointer_enter", value)

    @property
    def on_pointer_leave(self) -> str:
        return self._get("on_pointer_leave")

    @on_pointer_leave.setter
    def on_pointer_leave(self, value: str) -> None:
        self._set("on_pointer_leave", value)

    @property
    def on_wheel(self) -> str:
        return self._get("on_wheel")

    @on_wheel.setter
    def on_wheel(self, value: str) -> None:
        self._set("on_wheel", value)

    @property
    def on_gesture(self) -> str:
        return self._get("on_gesture")

    @on_gesture.setter
    def on_gesture(self, value: str) -> None:
        self._set("on_gesture", value)

    @property
    def on_swipe(self) -> str:
        return self._get("on_swipe")

    @on_swipe.setter
    def on_swipe(self, value: str) -> None:
        self._set("on_swipe", value)

    @property
    def on_key_up(self) -> str:
        return self._get("on_key_up")

    @on_key_up.setter
    def on_key_up(self, value: str) -> None:
        self._set("on_key_up", value)

    @property
    def on_enter(self) -> str:
        return self._get("on_enter")

    @on_enter.setter
    def on_enter(self, value: str) -> None:
        self._set("on_enter", value)

    @property
    def on_exit(self) -> str:
        return self._get("on_exit")

    @on_exit.setter
    def on_exit(self, value: str) -> None:
        self._set("on_exit", value)

    @property
    def on_resize(self) -> str:
        return self._get("on_resize")

    @on_resize.setter
    def on_resize(self, value: str) -> None:
        self._set("on_resize", value)

    @property
    def on_drag_move(self) -> str:
        return self._get("on_drag_move")

    @on_drag_move.setter
    def on_drag_move(self, value: str) -> None:
        self._set("on_drag_move", value)

    @property
    def on_drag_leave(self) -> str:
        return self._get("on_drag_leave")

    @on_drag_leave.setter
    def on_drag_leave(self, value: str) -> None:
        self._set("on_drag_leave", value)

    @property
    def on_drag_drop(self) -> str:
        return self._get("on_drag_drop")

    @on_drag_drop.setter
    def on_drag_drop(self, value: str) -> None:
        self._set("on_drag_drop", value)

    @property
    def on_drag_end(self) -> str:
        return self._get("on_drag_end")

    @on_drag_end.setter
    def on_drag_end(self, value: str) -> None:
        self._set("on_drag_end", value)


class FrmIBGEComponents(Form):
    """O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância)."""

    def bind(self, components):
        self.cdsEstados = self._resolve(components, "cdsEstados", "ClientDataSet", ClientDataSet)
        self.cdsEstadosId = self._resolve(components, "cdsEstadosId", "Field", Field)
        self.cdsEstadosSigla = self._resolve(components, "cdsEstadosSigla", "Field", Field)
        self.cdsEstadosNome = self._resolve(components, "cdsEstadosNome", "Field", Field)
        self.cdsEstadosRegiao = self._resolve(components, "cdsEstadosRegiao", "Field", Field)
        self.cdsEstadosPopulacao = self._resolve(components, "cdsEstadosPopulacao", "Field", Field)
        self.cdsEstadosArea = self._resolve(components, "cdsEstadosArea", "Field", Field)
        self.cdsEstadosDensidade = self._resolve(components, "cdsEstadosDensidade", "Field", Field)
        self.cdsEstadosPib = self._resolve(components, "cdsEstadosPib", "Field", Field)
        self.cdsEstadosPibPerCapita = self._resolve(components, "cdsEstadosPibPerCapita", "Field", Field)
        self.cdsEstadosEstimativa = self._resolve(components, "cdsEstadosEstimativa", "Field", Field)
        self.cdsEstadosEstimativaAno = self._resolve(components, "cdsEstadosEstimativaAno", "Field", Field)
        self.cdsEstadosMunicipios = self._resolve(components, "cdsEstadosMunicipios", "Field", Field)
        self.cdsEstadosBandeira = self._resolve(components, "cdsEstadosBandeira", "Field", Field)
        self.dsEstados = self._resolve(components, "dsEstados", "DataSource", DataSource)
        self.cdsMunicipios = self._resolve(components, "cdsMunicipios", "ClientDataSet", ClientDataSet)
        self.cdsMunicipiosId = self._resolve(components, "cdsMunicipiosId", "Field", Field)
        self.cdsMunicipiosUfId = self._resolve(components, "cdsMunicipiosUfId", "Field", Field)
        self.cdsMunicipiosUf = self._resolve(components, "cdsMunicipiosUf", "Field", Field)
        self.cdsMunicipiosNome = self._resolve(components, "cdsMunicipiosNome", "Field", Field)
        self.cdsMunicipiosMesorregiao = self._resolve(components, "cdsMunicipiosMesorregiao", "Field", Field)
        self.cdsMunicipiosMicrorregiao = self._resolve(components, "cdsMunicipiosMicrorregiao", "Field", Field)
        self.cdsMunicipiosRegiaoIntermediaria = self._resolve(components, "cdsMunicipiosRegiaoIntermediaria", "Field", Field)
        self.cdsMunicipiosRegiaoImediata = self._resolve(components, "cdsMunicipiosRegiaoImediata", "Field", Field)
        self.cdsMunicipiosPopulacao = self._resolve(components, "cdsMunicipiosPopulacao", "Field", Field)
        self.cdsMunicipiosArea = self._resolve(components, "cdsMunicipiosArea", "Field", Field)
        self.cdsMunicipiosDensidade = self._resolve(components, "cdsMunicipiosDensidade", "Field", Field)
        self.cdsMunicipiosPib = self._resolve(components, "cdsMunicipiosPib", "Field", Field)
        self.cdsMunicipiosPibParticipacao = self._resolve(components, "cdsMunicipiosPibParticipacao", "Field", Field)
        self.cdsMunicipiosPibPerCapita = self._resolve(components, "cdsMunicipiosPibPerCapita", "Field", Field)
        self.cdsMunicipiosEstimativa = self._resolve(components, "cdsMunicipiosEstimativa", "Field", Field)
        self.cdsMunicipiosEstimativaAno = self._resolve(components, "cdsMunicipiosEstimativaAno", "Field", Field)
        self.cdsMunicipiosDistritos = self._resolve(components, "cdsMunicipiosDistritos", "Field", Field)
        self.cdsMunicipiosBandeira = self._resolve(components, "cdsMunicipiosBandeira", "Field", Field)
        self.dsMunicipios = self._resolve(components, "dsMunicipios", "DataSource", DataSource)
        self.cdsDistritos = self._resolve(components, "cdsDistritos", "ClientDataSet", ClientDataSet)
        self.cdsDistritosId = self._resolve(components, "cdsDistritosId", "Field", Field)
        self.cdsDistritosMunicipioId = self._resolve(components, "cdsDistritosMunicipioId", "Field", Field)
        self.cdsDistritosNome = self._resolve(components, "cdsDistritosNome", "Field", Field)
        self.dsDistritos = self._resolve(components, "dsDistritos", "DataSource", DataSource)
        self.cdsPopulacao = self._resolve(components, "cdsPopulacao", "ClientDataSet", ClientDataSet)
        self.cdsPopulacaoAno = self._resolve(components, "cdsPopulacaoAno", "Field", Field)
        self.cdsPopulacaoHabitantes = self._resolve(components, "cdsPopulacaoHabitantes", "Field", Field)
        self.dsPopulacao = self._resolve(components, "dsPopulacao", "DataSource", DataSource)
        self.cdsMaiores = self._resolve(components, "cdsMaiores", "ClientDataSet", ClientDataSet)
        self.cdsMaioresNome = self._resolve(components, "cdsMaioresNome", "Field", Field)
        self.cdsMaioresHabitantes = self._resolve(components, "cdsMaioresHabitantes", "Field", Field)
        self.dsMaiores = self._resolve(components, "dsMaiores", "DataSource", DataSource)
        self.lblFonte = self._resolve(components, "lblFonte", "Label", Label)
        self.lytListas = self._resolve(components, "lytListas", "Layout", Layout)
        self.gbEstados = self._resolve(components, "gbEstados", "GroupBox", GroupBox)
        self.grdEstados = self._resolve(components, "grdEstados", "DataGrid", DataGrid)
        self.gbDistritos = self._resolve(components, "gbDistritos", "GroupBox", GroupBox)
        self.grdDistritos = self._resolve(components, "grdDistritos", "DataGrid", DataGrid)
        self.gbMunicipios = self._resolve(components, "gbMunicipios", "GroupBox", GroupBox)
        self.grdMunicipios = self._resolve(components, "grdMunicipios", "DataGrid", DataGrid)
        self.splListas = self._resolve(components, "splListas", "Splitter", Splitter)
        self.lytPainel = self._resolve(components, "lytPainel", "Layout", Layout)
        self.pnlEstado = self._resolve(components, "pnlEstado", "Panel", Panel)
        self.svgBrasil = self._resolve(components, "svgBrasil", "Svg", Svg)
        self.lytBandeiraEstado = self._resolve(components, "lytBandeiraEstado", "Layout", Layout)
        self.lblEstado = self._resolve(components, "lblEstado", "Label", Label)
        self.lblRegiao = self._resolve(components, "lblRegiao", "Label", Label)
        self.imgBandeiraEstado = self._resolve(components, "imgBandeiraEstado", "Image", Image)
        self.lytNumerosEstado = self._resolve(components, "lytNumerosEstado", "Layout", Layout)
        self.dbtEstadoPopulacao = self._resolve(components, "dbtEstadoPopulacao", "Label", Label)
        self.lblEstadoEstimativa = self._resolve(components, "lblEstadoEstimativa", "Label", Label)
        self.dbtEstadoEstimativa = self._resolve(components, "dbtEstadoEstimativa", "Label", Label)
        self.dbtEstadoArea = self._resolve(components, "dbtEstadoArea", "Label", Label)
        self.dbtEstadoDensidade = self._resolve(components, "dbtEstadoDensidade", "Label", Label)
        self.dbtEstadoPib = self._resolve(components, "dbtEstadoPib", "Label", Label)
        self.dbtEstadoPibPerCapita = self._resolve(components, "dbtEstadoPibPerCapita", "Label", Label)
        self.dbtEstadoMunicipios = self._resolve(components, "dbtEstadoMunicipios", "Label", Label)
        self.pnlMunicipio = self._resolve(components, "pnlMunicipio", "Panel", Panel)
        self.svgMunicipio = self._resolve(components, "svgMunicipio", "Svg", Svg)
        self.svgEstado = self._resolve(components, "svgEstado", "Svg", Svg)
        self.lytBandeiraMunicipio = self._resolve(components, "lytBandeiraMunicipio", "Layout", Layout)
        self.lblMunicipio = self._resolve(components, "lblMunicipio", "Label", Label)
        self.lblCodigo = self._resolve(components, "lblCodigo", "Label", Label)
        self.imgBandeiraMunicipio = self._resolve(components, "imgBandeiraMunicipio", "Image", Image)
        self.lblSemBandeira = self._resolve(components, "lblSemBandeira", "Label", Label)
        self.lytNumerosMunicipio = self._resolve(components, "lytNumerosMunicipio", "Layout", Layout)
        self.dbtMesorregiao = self._resolve(components, "dbtMesorregiao", "Label", Label)
        self.dbtMicrorregiao = self._resolve(components, "dbtMicrorregiao", "Label", Label)
        self.dbtRegiaoImediata = self._resolve(components, "dbtRegiaoImediata", "Label", Label)
        self.dbtPopulacao = self._resolve(components, "dbtPopulacao", "Label", Label)
        self.lblEstimativa = self._resolve(components, "lblEstimativa", "Label", Label)
        self.dbtEstimativa = self._resolve(components, "dbtEstimativa", "Label", Label)
        self.dbtArea = self._resolve(components, "dbtArea", "Label", Label)
        self.dbtDensidade = self._resolve(components, "dbtDensidade", "Label", Label)
        self.dbtPib = self._resolve(components, "dbtPib", "Label", Label)
        self.dbtPibPerCapita = self._resolve(components, "dbtPibPerCapita", "Label", Label)
        self.dbtPibParticipacao = self._resolve(components, "dbtPibParticipacao", "Label", Label)
        self.lytGraficos = self._resolve(components, "lytGraficos", "Layout", Layout)
        self.dbcPopulacao = self._resolve(components, "dbcPopulacao", "DBChart", DBChart)
        self.dbcMaiores = self._resolve(components, "dbcMaiores", "DBChart", DBChart)
        self.dbcEstados = self._resolve(components, "dbcEstados", "DBChart", DBChart)

    def find_client_data_set(self, name):
        """O ClientDataSet deste form com o nome."""
        if name == "cdsEstados":
            return self.cdsEstados
        if name == "cdsMunicipios":
            return self.cdsMunicipios
        if name == "cdsDistritos":
            return self.cdsDistritos
        if name == "cdsPopulacao":
            return self.cdsPopulacao
        if name == "cdsMaiores":
            return self.cdsMaiores
        return None

    def sender_as_client_data_set(self, event):
        """O ClientDataSet que disparou o evento."""
        return self.find_client_data_set(event["sender"]["name"])

    def find_d_b_chart(self, name):
        """O DBChart deste form com o nome."""
        if name == "dbcPopulacao":
            return self.dbcPopulacao
        if name == "dbcMaiores":
            return self.dbcMaiores
        if name == "dbcEstados":
            return self.dbcEstados
        return None

    def sender_as_d_b_chart(self, event):
        """O DBChart que disparou o evento."""
        return self.find_d_b_chart(event["sender"]["name"])

    def find_data_grid(self, name):
        """O DataGrid deste form com o nome."""
        if name == "grdEstados":
            return self.grdEstados
        if name == "grdDistritos":
            return self.grdDistritos
        if name == "grdMunicipios":
            return self.grdMunicipios
        return None

    def sender_as_data_grid(self, event):
        """O DataGrid que disparou o evento."""
        return self.find_data_grid(event["sender"]["name"])

    def find_data_source(self, name):
        """O DataSource deste form com o nome."""
        if name == "dsEstados":
            return self.dsEstados
        if name == "dsMunicipios":
            return self.dsMunicipios
        if name == "dsDistritos":
            return self.dsDistritos
        if name == "dsPopulacao":
            return self.dsPopulacao
        if name == "dsMaiores":
            return self.dsMaiores
        return None

    def sender_as_data_source(self, event):
        """O DataSource que disparou o evento."""
        return self.find_data_source(event["sender"]["name"])

    def find_field(self, name):
        """O Field deste form com o nome."""
        if name == "cdsEstadosId":
            return self.cdsEstadosId
        if name == "cdsEstadosSigla":
            return self.cdsEstadosSigla
        if name == "cdsEstadosNome":
            return self.cdsEstadosNome
        if name == "cdsEstadosRegiao":
            return self.cdsEstadosRegiao
        if name == "cdsEstadosPopulacao":
            return self.cdsEstadosPopulacao
        if name == "cdsEstadosArea":
            return self.cdsEstadosArea
        if name == "cdsEstadosDensidade":
            return self.cdsEstadosDensidade
        if name == "cdsEstadosPib":
            return self.cdsEstadosPib
        if name == "cdsEstadosPibPerCapita":
            return self.cdsEstadosPibPerCapita
        if name == "cdsEstadosEstimativa":
            return self.cdsEstadosEstimativa
        if name == "cdsEstadosEstimativaAno":
            return self.cdsEstadosEstimativaAno
        if name == "cdsEstadosMunicipios":
            return self.cdsEstadosMunicipios
        if name == "cdsEstadosBandeira":
            return self.cdsEstadosBandeira
        if name == "cdsMunicipiosId":
            return self.cdsMunicipiosId
        if name == "cdsMunicipiosUfId":
            return self.cdsMunicipiosUfId
        if name == "cdsMunicipiosUf":
            return self.cdsMunicipiosUf
        if name == "cdsMunicipiosNome":
            return self.cdsMunicipiosNome
        if name == "cdsMunicipiosMesorregiao":
            return self.cdsMunicipiosMesorregiao
        if name == "cdsMunicipiosMicrorregiao":
            return self.cdsMunicipiosMicrorregiao
        if name == "cdsMunicipiosRegiaoIntermediaria":
            return self.cdsMunicipiosRegiaoIntermediaria
        if name == "cdsMunicipiosRegiaoImediata":
            return self.cdsMunicipiosRegiaoImediata
        if name == "cdsMunicipiosPopulacao":
            return self.cdsMunicipiosPopulacao
        if name == "cdsMunicipiosArea":
            return self.cdsMunicipiosArea
        if name == "cdsMunicipiosDensidade":
            return self.cdsMunicipiosDensidade
        if name == "cdsMunicipiosPib":
            return self.cdsMunicipiosPib
        if name == "cdsMunicipiosPibParticipacao":
            return self.cdsMunicipiosPibParticipacao
        if name == "cdsMunicipiosPibPerCapita":
            return self.cdsMunicipiosPibPerCapita
        if name == "cdsMunicipiosEstimativa":
            return self.cdsMunicipiosEstimativa
        if name == "cdsMunicipiosEstimativaAno":
            return self.cdsMunicipiosEstimativaAno
        if name == "cdsMunicipiosDistritos":
            return self.cdsMunicipiosDistritos
        if name == "cdsMunicipiosBandeira":
            return self.cdsMunicipiosBandeira
        if name == "cdsDistritosId":
            return self.cdsDistritosId
        if name == "cdsDistritosMunicipioId":
            return self.cdsDistritosMunicipioId
        if name == "cdsDistritosNome":
            return self.cdsDistritosNome
        if name == "cdsPopulacaoAno":
            return self.cdsPopulacaoAno
        if name == "cdsPopulacaoHabitantes":
            return self.cdsPopulacaoHabitantes
        if name == "cdsMaioresNome":
            return self.cdsMaioresNome
        if name == "cdsMaioresHabitantes":
            return self.cdsMaioresHabitantes
        return None

    def sender_as_field(self, event):
        """O Field que disparou o evento."""
        return self.find_field(event["sender"]["name"])

    def find_group_box(self, name):
        """O GroupBox deste form com o nome."""
        if name == "gbEstados":
            return self.gbEstados
        if name == "gbDistritos":
            return self.gbDistritos
        if name == "gbMunicipios":
            return self.gbMunicipios
        return None

    def sender_as_group_box(self, event):
        """O GroupBox que disparou o evento."""
        return self.find_group_box(event["sender"]["name"])

    def find_image(self, name):
        """O Image deste form com o nome."""
        if name == "imgBandeiraEstado":
            return self.imgBandeiraEstado
        if name == "imgBandeiraMunicipio":
            return self.imgBandeiraMunicipio
        return None

    def sender_as_image(self, event):
        """O Image que disparou o evento."""
        return self.find_image(event["sender"]["name"])

    def find_label(self, name):
        """O Label deste form com o nome."""
        if name == "lblFonte":
            return self.lblFonte
        if name == "lblEstado":
            return self.lblEstado
        if name == "lblRegiao":
            return self.lblRegiao
        if name == "dbtEstadoPopulacao":
            return self.dbtEstadoPopulacao
        if name == "lblEstadoEstimativa":
            return self.lblEstadoEstimativa
        if name == "dbtEstadoEstimativa":
            return self.dbtEstadoEstimativa
        if name == "dbtEstadoArea":
            return self.dbtEstadoArea
        if name == "dbtEstadoDensidade":
            return self.dbtEstadoDensidade
        if name == "dbtEstadoPib":
            return self.dbtEstadoPib
        if name == "dbtEstadoPibPerCapita":
            return self.dbtEstadoPibPerCapita
        if name == "dbtEstadoMunicipios":
            return self.dbtEstadoMunicipios
        if name == "lblMunicipio":
            return self.lblMunicipio
        if name == "lblCodigo":
            return self.lblCodigo
        if name == "lblSemBandeira":
            return self.lblSemBandeira
        if name == "dbtMesorregiao":
            return self.dbtMesorregiao
        if name == "dbtMicrorregiao":
            return self.dbtMicrorregiao
        if name == "dbtRegiaoImediata":
            return self.dbtRegiaoImediata
        if name == "dbtPopulacao":
            return self.dbtPopulacao
        if name == "lblEstimativa":
            return self.lblEstimativa
        if name == "dbtEstimativa":
            return self.dbtEstimativa
        if name == "dbtArea":
            return self.dbtArea
        if name == "dbtDensidade":
            return self.dbtDensidade
        if name == "dbtPib":
            return self.dbtPib
        if name == "dbtPibPerCapita":
            return self.dbtPibPerCapita
        if name == "dbtPibParticipacao":
            return self.dbtPibParticipacao
        return None

    def sender_as_label(self, event):
        """O Label que disparou o evento."""
        return self.find_label(event["sender"]["name"])

    def find_layout(self, name):
        """O Layout deste form com o nome."""
        if name == "lytListas":
            return self.lytListas
        if name == "lytPainel":
            return self.lytPainel
        if name == "lytBandeiraEstado":
            return self.lytBandeiraEstado
        if name == "lytNumerosEstado":
            return self.lytNumerosEstado
        if name == "lytBandeiraMunicipio":
            return self.lytBandeiraMunicipio
        if name == "lytNumerosMunicipio":
            return self.lytNumerosMunicipio
        if name == "lytGraficos":
            return self.lytGraficos
        return None

    def sender_as_layout(self, event):
        """O Layout que disparou o evento."""
        return self.find_layout(event["sender"]["name"])

    def find_panel(self, name):
        """O Panel deste form com o nome."""
        if name == "pnlEstado":
            return self.pnlEstado
        if name == "pnlMunicipio":
            return self.pnlMunicipio
        return None

    def sender_as_panel(self, event):
        """O Panel que disparou o evento."""
        return self.find_panel(event["sender"]["name"])

    def find_splitter(self, name):
        """O Splitter deste form com o nome."""
        if name == "splListas":
            return self.splListas
        return None

    def sender_as_splitter(self, event):
        """O Splitter que disparou o evento."""
        return self.find_splitter(event["sender"]["name"])

    def find_svg(self, name):
        """O Svg deste form com o nome."""
        if name == "svgBrasil":
            return self.svgBrasil
        if name == "svgMunicipio":
            return self.svgMunicipio
        if name == "svgEstado":
            return self.svgEstado
        return None

    def sender_as_svg(self, event):
        """O Svg que disparou o evento."""
        return self.find_svg(event["sender"]["name"])


# A janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers).
frmIBGE = CurrentForm("frmIBGE")
register_form_class("frmIBGE", FrmIBGEComponents, frmIBGE._activate)
