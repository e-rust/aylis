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


class ButtonStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    ACCENT = "accent"
    DANGER = "danger"
    ELEVATED = "elevated"
    FAB = "fab"
    FAB_EXTENDED = "fab_extended"
    FLAT = "flat"
    GHOST = "ghost"
    GLASS = "glass"
    GLASS_PROMINENT = "glass_prominent"
    GLOSSY = "glossy"
    ICON = "icon"
    ICON_FILLED = "icon_filled"
    ICON_OUTLINED = "icon_outlined"
    ICON_TONAL = "icon_tonal"
    INFO = "info"
    LINK = "link"
    NEGATIVE = "negative"
    OUTLINE = "outline"
    PILL = "pill"
    POSITIVE = "positive"
    PUSH = "push"
    REVEAL = "reveal"
    ROUND = "round"
    ROUNDED = "rounded"
    SECONDARY = "secondary"
    SUCCESS = "success"
    TEXT = "text"
    TONAL = "tonal"
    UNELEVATED = "unelevated"
    WARNING = "warning"


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


class DividerStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


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


class IconPosition(str, Enum):
    """Os valores de um enum do acesso tipado."""

    LEFT = "left"
    RIGHT = "right"
    TOP = "top"


class IconStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    MUTED = "muted"


class ImageFit(str, Enum):
    """Os valores de um enum do acesso tipado."""

    CONTAIN = "contain"
    COVER = "cover"
    STRETCH = "stretch"
    CENTER = "center"
    ORIGINAL = "original"


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


class ListViewStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""
    FLAT = "flat"


class MainMenuStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class ModalResult(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NONE = "none"
    OK = "ok"
    CANCEL = "cancel"
    ABORT = "abort"
    RETRY = "retry"
    IGNORE = "ignore"
    YES = "yes"
    NO = "no"
    CLOSE = "close"


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


class SeekBarStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


class SelfAlign(str, Enum):
    """Os valores de um enum do acesso tipado."""

    STRETCH = "stretch"
    START = "start"
    CENTER = "center"
    END = "end"


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


class TimerPrecision(str, Enum):
    """Os valores de um enum do acesso tipado."""

    NORMAL = "normal"
    HIGH = "high"
    EXACT = "exact"


class TrackBarStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


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


class VideoPlayerStyle(str, Enum):
    """Os valores de um enum do acesso tipado."""

    DEFAULT = ""


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
class ListViewItem:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    title: "str" = ""
    detail: "str" = ""

    @staticmethod
    def from_json(data):
        return ListViewItem(
            title=str(data.get("title") or ""),
            detail=str(data.get("detail") or ""),
        )

    def to_json(self):
        return {
            "title": self.title,
            "detail": self.detail,
        }


@dataclass
class OutputSpec:
    """Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos)."""

    name: "str" = ""
    volume: "float" = 0.0
    delay_ms: "float" = 0.0

    @staticmethod
    def from_json(data):
        return OutputSpec(
            name=str(data.get("name") or ""),
            volume=float(data.get("volume") or 0),
            delay_ms=float(data.get("delay_ms") or 0),
        )

    def to_json(self):
        return {
            "name": self.name,
            "volume": self.volume,
            "delay_ms": self.delay_ms,
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


class Button(AylisComponent):
    """Um Button de um form."""

    @property
    def text(self) -> str:
        return self._get("text")

    @text.setter
    def text(self, value: str) -> None:
        self._set("text", value)

    @property
    def icon(self) -> str:
        return self._get("icon")

    @icon.setter
    def icon(self, value: str) -> None:
        self._set("icon", value)

    @property
    def checked_icon(self) -> str:
        return self._get("checked_icon")

    @checked_icon.setter
    def checked_icon(self, value: str) -> None:
        self._set("checked_icon", value)

    @property
    def icon_position(self) -> IconPosition:
        return _enum(IconPosition, self._get("icon_position"))

    @icon_position.setter
    def icon_position(self, value: IconPosition) -> None:
        self._set("icon_position", value.value)

    @property
    def icon_size(self) -> float:
        return self._get("icon_size")

    @icon_size.setter
    def icon_size(self, value: float) -> None:
        self._set("icon_size", value)

    @property
    def checkable(self) -> bool:
        return self._get("checkable")

    @checkable.setter
    def checkable(self, value: bool) -> None:
        self._set("checkable", value)

    @property
    def checked(self) -> bool:
        return self._get("checked")

    @checked.setter
    def checked(self, value: bool) -> None:
        self._set("checked", value)

    @property
    def modal_result(self) -> ModalResult:
        return _enum(ModalResult, self._get("modal_result"))

    @modal_result.setter
    def modal_result(self, value: ModalResult) -> None:
        self._set("modal_result", value.value)

    @property
    def repeat_enter(self) -> bool:
        return self._get("repeat_enter")

    @repeat_enter.setter
    def repeat_enter(self, value: bool) -> None:
        self._set("repeat_enter", value)

    @property
    def default_button(self) -> bool:
        return self._get("default_button")

    @default_button.setter
    def default_button(self, value: bool) -> None:
        self._set("default_button", value)

    @property
    def cancel_button(self) -> bool:
        return self._get("cancel_button")

    @cancel_button.setter
    def cancel_button(self, value: bool) -> None:
        self._set("cancel_button", value)

    @property
    def h_align(self) -> TextAlign:
        return _enum(TextAlign, self._get("h_align"))

    @h_align.setter
    def h_align(self, value: TextAlign) -> None:
        self._set("h_align", value.value)

    @property
    def group_index(self) -> int:
        return self._get("group_index")

    @group_index.setter
    def group_index(self, value: int) -> None:
        self._set("group_index", value)

    @property
    def radius(self) -> Bounds:
        return Bounds.from_json(self._get("radius"))

    @radius.setter
    def radius(self, value: Bounds) -> None:
        self._set("radius", value.to_json())

    @property
    def on_change(self) -> str:
        return self._get("on_change")

    @on_change.setter
    def on_change(self, value: str) -> None:
        self._set("on_change", value)

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
    def style(self) -> ButtonStyle:
        return _enum(ButtonStyle, self._get("style"))

    @style.setter
    def style(self, value: ButtonStyle) -> None:
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


class Divider(AylisComponent):
    """Um Divider de um form."""

    @property
    def vertical(self) -> bool:
        return self._get("vertical")

    @vertical.setter
    def vertical(self, value: bool) -> None:
        self._set("vertical", value)

    @property
    def text(self) -> str:
        return self._get("text")

    @text.setter
    def text(self, value: str) -> None:
        self._set("text", value)

    @property
    def thickness(self) -> float:
        return self._get("thickness")

    @thickness.setter
    def thickness(self, value: float) -> None:
        self._set("thickness", value)

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
    def style(self) -> DividerStyle:
        return _enum(DividerStyle, self._get("style"))

    @style.setter
    def style(self, value: DividerStyle) -> None:
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


class Icon(AylisComponent):
    """Um Icon de um form."""

    @property
    def icon(self) -> str:
        return self._get("icon")

    @icon.setter
    def icon(self, value: str) -> None:
        self._set("icon", value)

    @property
    def color(self) -> Color:
        return Color.from_json(self._get("color"))

    @color.setter
    def color(self, value: Color) -> None:
        self._set("color", value.to_json())

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
    def style(self) -> IconStyle:
        return _enum(IconStyle, self._get("style"))

    @style.setter
    def style(self, value: IconStyle) -> None:
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


class ListView(AylisComponent):
    """Um ListView de um form."""

    @property
    def items(self) -> list[ListViewItem]:
        return _records(ListViewItem, self._get("items"))

    @items.setter
    def items(self, value: list[ListViewItem]) -> None:
        self._set("items", _records_text(value))

    @property
    def item_index(self) -> int:
        return self._get("item_index")

    @item_index.setter
    def item_index(self, value: int) -> None:
        self._set("item_index", value)

    @property
    def item_height(self) -> float:
        return self._get("item_height")

    @item_height.setter
    def item_height(self, value: float) -> None:
        self._set("item_height", value)

    @property
    def accessory(self) -> bool:
        return self._get("accessory")

    @accessory.setter
    def accessory(self, value: bool) -> None:
        self._set("accessory", value)

    @property
    def on_change(self) -> str:
        return self._get("on_change")

    @on_change.setter
    def on_change(self, value: str) -> None:
        self._set("on_change", value)

    @property
    def on_item_click(self) -> str:
        return self._get("on_item_click")

    @on_item_click.setter
    def on_item_click(self, value: str) -> None:
        self._set("on_item_click", value)

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
    def style(self) -> ListViewStyle:
        return _enum(ListViewStyle, self._get("style"))

    @style.setter
    def style(self, value: ListViewStyle) -> None:
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


class MainMenu(AylisComponent):
    """Um MainMenu de um form."""

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
    def style(self) -> MainMenuStyle:
        return _enum(MainMenuStyle, self._get("style"))

    @style.setter
    def style(self, value: MainMenuStyle) -> None:
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

    def close(self) -> None:
        self.aylis.send("invoke", target=self.id, method="close", args=[])


class MenuItem(AylisComponent):
    """Um MenuItem de um form."""

    @property
    def text(self) -> str:
        return self._get("text")

    @text.setter
    def text(self, value: str) -> None:
        self._set("text", value)

    @property
    def shortcut(self) -> str:
        return self._get("shortcut")

    @shortcut.setter
    def shortcut(self, value: str) -> None:
        self._set("shortcut", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def checked(self) -> bool:
        return self._get("checked")

    @checked.setter
    def checked(self, value: bool) -> None:
        self._set("checked", value)

    @property
    def auto_check(self) -> bool:
        return self._get("auto_check")

    @auto_check.setter
    def auto_check(self, value: bool) -> None:
        self._set("auto_check", value)

    @property
    def icon(self) -> str:
        return self._get("icon")

    @icon.setter
    def icon(self, value: str) -> None:
        self._set("icon", value)

    @property
    def visible(self) -> bool:
        return self._get("visible")

    @visible.setter
    def visible(self, value: bool) -> None:
        self._set("visible", value)

    @property
    def danger(self) -> bool:
        return self._get("danger")

    @danger.setter
    def danger(self, value: bool) -> None:
        self._set("danger", value)

    @property
    def header(self) -> bool:
        return self._get("header")

    @header.setter
    def header(self, value: bool) -> None:
        self._set("header", value)

    @property
    def content(self) -> AylisComponent | None:
        return self._get_ref("content")

    @content.setter
    def content(self, value: AylisComponent | None) -> None:
        self._set_ref("content", value)

    @property
    def on_click(self) -> str:
        return self._get("on_click")

    @on_click.setter
    def on_click(self, value: str) -> None:
        self._set("on_click", value)


class OpenDialog(AylisComponent):
    """Um OpenDialog de um form."""

    @property
    def title(self) -> str:
        return self._get("title")

    @title.setter
    def title(self, value: str) -> None:
        self._set("title", value)

    @property
    def filter(self) -> str:
        return self._get("filter")

    @filter.setter
    def filter(self, value: str) -> None:
        self._set("filter", value)

    @property
    def initial_dir(self) -> str:
        return self._get("initial_dir")

    @initial_dir.setter
    def initial_dir(self, value: str) -> None:
        self._set("initial_dir", value)

    @property
    def file_name(self) -> str:
        return self._get("file_name")

    @file_name.setter
    def file_name(self, value: str) -> None:
        self._set("file_name", value)

    @property
    def multiple(self) -> bool:
        return self._get("multiple")

    @multiple.setter
    def multiple(self, value: bool) -> None:
        self._set("multiple", value)

    @property
    def on_select(self) -> str:
        return self._get("on_select")

    @on_select.setter
    def on_select(self, value: str) -> None:
        self._set("on_select", value)

    @property
    def on_cancel(self) -> str:
        return self._get("on_cancel")

    @on_cancel.setter
    def on_cancel(self, value: str) -> None:
        self._set("on_cancel", value)

    def execute(self) -> None:
        self.aylis.send("execute", target=self.id)


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


class SeekBar(AylisComponent):
    """Um SeekBar de um form."""

    @property
    def position(self) -> float:
        return self._get("position")

    @position.setter
    def position(self, value: float) -> None:
        self._set("position", value)

    @property
    def duration(self) -> float:
        return self._get("duration")

    @duration.setter
    def duration(self, value: float) -> None:
        self._set("duration", value)

    @property
    def buffered(self) -> float:
        return self._get("buffered")

    @buffered.setter
    def buffered(self, value: float) -> None:
        self._set("buffered", value)

    @property
    def markers(self) -> list[float]:
        return _numbers(self._get("markers"))

    @markers.setter
    def markers(self, value: list[float]) -> None:
        self._set("markers", ";".join(repr(float(x)) for x in value))

    @property
    def show_time(self) -> bool:
        return self._get("show_time")

    @show_time.setter
    def show_time(self, value: bool) -> None:
        self._set("show_time", value)

    @property
    def on_change(self) -> str:
        return self._get("on_change")

    @on_change.setter
    def on_change(self, value: str) -> None:
        self._set("on_change", value)

    @property
    def on_seek(self) -> str:
        return self._get("on_seek")

    @on_seek.setter
    def on_seek(self, value: str) -> None:
        self._set("on_seek", value)

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
    def style(self) -> SeekBarStyle:
        return _enum(SeekBarStyle, self._get("style"))

    @style.setter
    def style(self, value: SeekBarStyle) -> None:
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


class Timer(AylisComponent):
    """Um Timer de um form."""

    @property
    def interval(self) -> float:
        return self._get("interval")

    @interval.setter
    def interval(self, value: float) -> None:
        self._set("interval", value)

    @property
    def enabled(self) -> bool:
        return self._get("enabled")

    @enabled.setter
    def enabled(self, value: bool) -> None:
        self._set("enabled", value)

    @property
    def precision(self) -> TimerPrecision:
        return _enum(TimerPrecision, self._get("precision"))

    @precision.setter
    def precision(self, value: TimerPrecision) -> None:
        self._set("precision", value.value)

    @property
    def background(self) -> bool:
        return self._get("background")

    @background.setter
    def background(self, value: bool) -> None:
        self._set("background", value)

    @property
    def one_shot(self) -> bool:
        return self._get("one_shot")

    @one_shot.setter
    def one_shot(self, value: bool) -> None:
        self._set("one_shot", value)

    @property
    def on_timer(self) -> str:
        return self._get("on_timer")

    @on_timer.setter
    def on_timer(self, value: str) -> None:
        self._set("on_timer", value)

    @property
    def drift(self) -> float:
        return self._get("drift")

    @drift.setter
    def drift(self, value: float) -> None:
        self._set("drift", value)

    def restart(self) -> None:
        self.aylis.send("invoke", target=self.id, method="restart", args=[])


class TrackBar(AylisComponent):
    """Um TrackBar de um form."""

    @property
    def value(self) -> float:
        return self._get("value")

    @value.setter
    def value(self, value: float) -> None:
        self._set("value", value)

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
    def frequency(self) -> float:
        return self._get("frequency")

    @frequency.setter
    def frequency(self, value: float) -> None:
        self._set("frequency", value)

    @property
    def vertical(self) -> bool:
        return self._get("vertical")

    @vertical.setter
    def vertical(self, value: bool) -> None:
        self._set("vertical", value)

    @property
    def thumb_size(self) -> float:
        return self._get("thumb_size")

    @thumb_size.setter
    def thumb_size(self, value: float) -> None:
        self._set("thumb_size", value)

    @property
    def track_size(self) -> float:
        return self._get("track_size")

    @track_size.setter
    def track_size(self, value: float) -> None:
        self._set("track_size", value)

    @property
    def solid_thumb(self) -> bool:
        return self._get("solid_thumb")

    @solid_thumb.setter
    def solid_thumb(self, value: bool) -> None:
        self._set("solid_thumb", value)

    @property
    def limit(self) -> float:
        return self._get("limit")

    @limit.setter
    def limit(self, value: float) -> None:
        self._set("limit", value)

    @property
    def on_change(self) -> str:
        return self._get("on_change")

    @on_change.setter
    def on_change(self, value: str) -> None:
        self._set("on_change", value)

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
    def style(self) -> TrackBarStyle:
        return _enum(TrackBarStyle, self._get("style"))

    @style.setter
    def style(self, value: TrackBarStyle) -> None:
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


class VideoPlayer(AylisComponent):
    """Um VideoPlayer de um form."""

    @property
    def file(self) -> str:
        return self._get("file")

    @file.setter
    def file(self, value: str) -> None:
        self._set("file", value)

    @property
    def autoplay(self) -> bool:
        return self._get("autoplay")

    @autoplay.setter
    def autoplay(self, value: bool) -> None:
        self._set("autoplay", value)

    @property
    def looping(self) -> bool:
        return self._get("looping")

    @looping.setter
    def looping(self, value: bool) -> None:
        self._set("looping", value)

    @property
    def volume(self) -> float:
        return self._get("volume")

    @volume.setter
    def volume(self, value: float) -> None:
        self._set("volume", value)

    @property
    def muted(self) -> bool:
        return self._get("muted")

    @muted.setter
    def muted(self, value: bool) -> None:
        self._set("muted", value)

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
    def rate(self) -> float:
        return self._get("rate")

    @rate.setter
    def rate(self, value: float) -> None:
        self._set("rate", value)

    @property
    def output(self) -> list[OutputSpec]:
        return _records(OutputSpec, self._get("output"))

    @output.setter
    def output(self, value: list[OutputSpec]) -> None:
        self._set("output", _records_text(value))

    @property
    def show_video(self) -> bool:
        return self._get("show_video")

    @show_video.setter
    def show_video(self, value: bool) -> None:
        self._set("show_video", value)

    @property
    def frames_ahead(self) -> int:
        return self._get("frames_ahead")

    @frames_ahead.setter
    def frames_ahead(self, value: int) -> None:
        self._set("frames_ahead", value)

    @property
    def on_ready(self) -> str:
        return self._get("on_ready")

    @on_ready.setter
    def on_ready(self, value: str) -> None:
        self._set("on_ready", value)

    @property
    def on_end(self) -> str:
        return self._get("on_end")

    @on_end.setter
    def on_end(self, value: str) -> None:
        self._set("on_end", value)

    @property
    def on_error(self) -> str:
        return self._get("on_error")

    @on_error.setter
    def on_error(self, value: str) -> None:
        self._set("on_error", value)

    @property
    def on_frame(self) -> str:
        return self._get("on_frame")

    @on_frame.setter
    def on_frame(self, value: str) -> None:
        self._set("on_frame", value)

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
    def style(self) -> VideoPlayerStyle:
        return _enum(VideoPlayerStyle, self._get("style"))

    @style.setter
    def style(self, value: VideoPlayerStyle) -> None:
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

    def play(self) -> None:
        self.aylis.send("media", target=self.id, op="play")

    def pause(self) -> None:
        self.aylis.send("media", target=self.id, op="pause")

    def stop(self) -> None:
        self.aylis.send("media", target=self.id, op="stop")

    def seek(self, seconds: float) -> None:
        self.aylis.send("media", target=self.id, op="seek", seconds=seconds)

    def position(self) -> float:
        return self.aylis.call("media", target=self.id, op="position")

    def duration(self) -> float:
        return self.aylis.call("media", target=self.id, op="duration")


class FrmPlayerComponents(Form):
    """O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância)."""

    def bind(self, components):
        self.mnuPrincipal = self._resolve(components, "mnuPrincipal", "MainMenu", MainMenu)
        self.mnuArquivo = self._resolve(components, "mnuArquivo", "MenuItem", MenuItem)
        self.mnuAbrir = self._resolve(components, "mnuAbrir", "MenuItem", MenuItem)
        self.mnuLimparFila = self._resolve(components, "mnuLimparFila", "MenuItem", MenuItem)
        self.mnuSep1 = self._resolve(components, "mnuSep1", "MenuItem", MenuItem)
        self.mnuSair = self._resolve(components, "mnuSair", "MenuItem", MenuItem)
        self.mnuReproduzir = self._resolve(components, "mnuReproduzir", "MenuItem", MenuItem)
        self.mnuReproduzirPausar = self._resolve(components, "mnuReproduzirPausar", "MenuItem", MenuItem)
        self.mnuParar = self._resolve(components, "mnuParar", "MenuItem", MenuItem)
        self.mnuAnterior = self._resolve(components, "mnuAnterior", "MenuItem", MenuItem)
        self.mnuProxima = self._resolve(components, "mnuProxima", "MenuItem", MenuItem)
        self.mnuSep2 = self._resolve(components, "mnuSep2", "MenuItem", MenuItem)
        self.mnuEmbaralhar = self._resolve(components, "mnuEmbaralhar", "MenuItem", MenuItem)
        self.mnuRepetir = self._resolve(components, "mnuRepetir", "MenuItem", MenuItem)
        self.mnuExibir = self._resolve(components, "mnuExibir", "MenuItem", MenuItem)
        self.mnuAgora = self._resolve(components, "mnuAgora", "MenuItem", MenuItem)
        self.mnuFila = self._resolve(components, "mnuFila", "MenuItem", MenuItem)
        self.pnlControles = self._resolve(components, "pnlControles", "Panel", Panel)
        self.lytBarra = self._resolve(components, "lytBarra", "Layout", Layout)
        self.lblTempo = self._resolve(components, "lblTempo", "Label", Label)
        self.lblDuracao = self._resolve(components, "lblDuracao", "Label", Label)
        self.sbPosicao = self._resolve(components, "sbPosicao", "SeekBar", SeekBar)
        self.lytInfo = self._resolve(components, "lytInfo", "Layout", Layout)
        self.icoCapa = self._resolve(components, "icoCapa", "Icon", Icon)
        self.lytTextos = self._resolve(components, "lytTextos", "Layout", Layout)
        self.lblTitulo = self._resolve(components, "lblTitulo", "Label", Label)
        self.lblDetalhe = self._resolve(components, "lblDetalhe", "Label", Label)
        self.lytVolume = self._resolve(components, "lytVolume", "Layout", Layout)
        self.btnMudo = self._resolve(components, "btnMudo", "Button", Button)
        self.tbVolume = self._resolve(components, "tbVolume", "TrackBar", TrackBar)
        self.lytBotoes = self._resolve(components, "lytBotoes", "Layout", Layout)
        self.lytBotoesCentro = self._resolve(components, "lytBotoesCentro", "Layout", Layout)
        self.btnEmbaralhar = self._resolve(components, "btnEmbaralhar", "Button", Button)
        self.btnAnterior = self._resolve(components, "btnAnterior", "Button", Button)
        self.btnReproduzir = self._resolve(components, "btnReproduzir", "Button", Button)
        self.btnProxima = self._resolve(components, "btnProxima", "Button", Button)
        self.btnRepetir = self._resolve(components, "btnRepetir", "Button", Button)
        self.lytNavegacao = self._resolve(components, "lytNavegacao", "Layout", Layout)
        self.btnAgora = self._resolve(components, "btnAgora", "Button", Button)
        self.btnFila = self._resolve(components, "btnFila", "Button", Button)
        self.divNavegacao = self._resolve(components, "divNavegacao", "Divider", Divider)
        self.btnAbrirArquivos = self._resolve(components, "btnAbrirArquivos", "Button", Button)
        self.pnlAgora = self._resolve(components, "pnlAgora", "Panel", Panel)
        self.vpVideo = self._resolve(components, "vpVideo", "VideoPlayer", VideoPlayer)
        self.pnlFila = self._resolve(components, "pnlFila", "Panel", Panel)
        self.lblFila = self._resolve(components, "lblFila", "Label", Label)
        self.lytFilaBotoes = self._resolve(components, "lytFilaBotoes", "Layout", Layout)
        self.btnAdicionar = self._resolve(components, "btnAdicionar", "Button", Button)
        self.btnLimparFila = self._resolve(components, "btnLimparFila", "Button", Button)
        self.lvFila = self._resolve(components, "lvFila", "ListView", ListView)
        self.dlgAbrir = self._resolve(components, "dlgAbrir", "OpenDialog", OpenDialog)
        self.tmrPosicao = self._resolve(components, "tmrPosicao", "Timer", Timer)

    def find_button(self, name):
        """O Button deste form com o nome."""
        if name == "btnMudo":
            return self.btnMudo
        if name == "btnEmbaralhar":
            return self.btnEmbaralhar
        if name == "btnAnterior":
            return self.btnAnterior
        if name == "btnReproduzir":
            return self.btnReproduzir
        if name == "btnProxima":
            return self.btnProxima
        if name == "btnRepetir":
            return self.btnRepetir
        if name == "btnAgora":
            return self.btnAgora
        if name == "btnFila":
            return self.btnFila
        if name == "btnAbrirArquivos":
            return self.btnAbrirArquivos
        if name == "btnAdicionar":
            return self.btnAdicionar
        if name == "btnLimparFila":
            return self.btnLimparFila
        return None

    def sender_as_button(self, event):
        """O Button que disparou o evento."""
        return self.find_button(event["sender"]["name"])

    def find_divider(self, name):
        """O Divider deste form com o nome."""
        if name == "divNavegacao":
            return self.divNavegacao
        return None

    def sender_as_divider(self, event):
        """O Divider que disparou o evento."""
        return self.find_divider(event["sender"]["name"])

    def find_icon(self, name):
        """O Icon deste form com o nome."""
        if name == "icoCapa":
            return self.icoCapa
        return None

    def sender_as_icon(self, event):
        """O Icon que disparou o evento."""
        return self.find_icon(event["sender"]["name"])

    def find_label(self, name):
        """O Label deste form com o nome."""
        if name == "lblTempo":
            return self.lblTempo
        if name == "lblDuracao":
            return self.lblDuracao
        if name == "lblTitulo":
            return self.lblTitulo
        if name == "lblDetalhe":
            return self.lblDetalhe
        if name == "lblFila":
            return self.lblFila
        return None

    def sender_as_label(self, event):
        """O Label que disparou o evento."""
        return self.find_label(event["sender"]["name"])

    def find_layout(self, name):
        """O Layout deste form com o nome."""
        if name == "lytBarra":
            return self.lytBarra
        if name == "lytInfo":
            return self.lytInfo
        if name == "lytTextos":
            return self.lytTextos
        if name == "lytVolume":
            return self.lytVolume
        if name == "lytBotoes":
            return self.lytBotoes
        if name == "lytBotoesCentro":
            return self.lytBotoesCentro
        if name == "lytNavegacao":
            return self.lytNavegacao
        if name == "lytFilaBotoes":
            return self.lytFilaBotoes
        return None

    def sender_as_layout(self, event):
        """O Layout que disparou o evento."""
        return self.find_layout(event["sender"]["name"])

    def find_list_view(self, name):
        """O ListView deste form com o nome."""
        if name == "lvFila":
            return self.lvFila
        return None

    def sender_as_list_view(self, event):
        """O ListView que disparou o evento."""
        return self.find_list_view(event["sender"]["name"])

    def find_main_menu(self, name):
        """O MainMenu deste form com o nome."""
        if name == "mnuPrincipal":
            return self.mnuPrincipal
        return None

    def sender_as_main_menu(self, event):
        """O MainMenu que disparou o evento."""
        return self.find_main_menu(event["sender"]["name"])

    def find_menu_item(self, name):
        """O MenuItem deste form com o nome."""
        if name == "mnuArquivo":
            return self.mnuArquivo
        if name == "mnuAbrir":
            return self.mnuAbrir
        if name == "mnuLimparFila":
            return self.mnuLimparFila
        if name == "mnuSep1":
            return self.mnuSep1
        if name == "mnuSair":
            return self.mnuSair
        if name == "mnuReproduzir":
            return self.mnuReproduzir
        if name == "mnuReproduzirPausar":
            return self.mnuReproduzirPausar
        if name == "mnuParar":
            return self.mnuParar
        if name == "mnuAnterior":
            return self.mnuAnterior
        if name == "mnuProxima":
            return self.mnuProxima
        if name == "mnuSep2":
            return self.mnuSep2
        if name == "mnuEmbaralhar":
            return self.mnuEmbaralhar
        if name == "mnuRepetir":
            return self.mnuRepetir
        if name == "mnuExibir":
            return self.mnuExibir
        if name == "mnuAgora":
            return self.mnuAgora
        if name == "mnuFila":
            return self.mnuFila
        return None

    def sender_as_menu_item(self, event):
        """O MenuItem que disparou o evento."""
        return self.find_menu_item(event["sender"]["name"])

    def find_open_dialog(self, name):
        """O OpenDialog deste form com o nome."""
        if name == "dlgAbrir":
            return self.dlgAbrir
        return None

    def sender_as_open_dialog(self, event):
        """O OpenDialog que disparou o evento."""
        return self.find_open_dialog(event["sender"]["name"])

    def find_panel(self, name):
        """O Panel deste form com o nome."""
        if name == "pnlControles":
            return self.pnlControles
        if name == "pnlAgora":
            return self.pnlAgora
        if name == "pnlFila":
            return self.pnlFila
        return None

    def sender_as_panel(self, event):
        """O Panel que disparou o evento."""
        return self.find_panel(event["sender"]["name"])

    def find_seek_bar(self, name):
        """O SeekBar deste form com o nome."""
        if name == "sbPosicao":
            return self.sbPosicao
        return None

    def sender_as_seek_bar(self, event):
        """O SeekBar que disparou o evento."""
        return self.find_seek_bar(event["sender"]["name"])

    def find_timer(self, name):
        """O Timer deste form com o nome."""
        if name == "tmrPosicao":
            return self.tmrPosicao
        return None

    def sender_as_timer(self, event):
        """O Timer que disparou o evento."""
        return self.find_timer(event["sender"]["name"])

    def find_track_bar(self, name):
        """O TrackBar deste form com o nome."""
        if name == "tbVolume":
            return self.tbVolume
        return None

    def sender_as_track_bar(self, event):
        """O TrackBar que disparou o evento."""
        return self.find_track_bar(event["sender"]["name"])

    def find_video_player(self, name):
        """O VideoPlayer deste form com o nome."""
        if name == "vpVideo":
            return self.vpVideo
        return None

    def sender_as_video_player(self, event):
        """O VideoPlayer que disparou o evento."""
        return self.find_video_player(event["sender"]["name"])


# A janela de frmPlayer cujo evento está rodando (a primeira, fora dos handlers).
frmPlayer = CurrentForm("frmPlayer")
register_form_class("frmPlayer", FrmPlayerComponents, frmPlayer._activate)
