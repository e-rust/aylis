// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import (
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
	"time"

	"aylis.dev/cliente/aylis"
)

var _ = fmt.Sprint

func asBool(v any) bool {
	b, _ := v.(bool)
	return b
}

func asNumber(v any) float64 {
	switch n := v.(type) {
	case float64:
		return n
	case int:
		return float64(n)
	case int64:
		return float64(n)
	}
	return 0
}

func asText(v any) string {
	if t, ok := v.(string); ok {
		return t
	}
	if v == nil {
		return ""
	}
	return fmt.Sprint(v)
}

func askCmd(app *aylis.Aylis, cmd string, fields aylis.M) any {
	v, err := app.Call(cmd, fields)
	if err != nil {
		aylis.Log(cmd+":", err)
	}
	return v
}

var _ = json.Marshal

// Date, TimeOfDay e DateTime são o time.Time dos handlers tipados (o código dos forms usa sem
// importar o time; zero é nenhum).
type Date = time.Time
type TimeOfDay = time.Time
type DateTime = time.Time
func asDate(v any) time.Time {
	t, _ := time.ParseInLocation("2006-01-02", asText(v), time.Local)
	return t
}

func asTime(v any) time.Time {
	for _, layout := range []string{"15:04:05", "15:04"} {
		if t, err := time.ParseInLocation(layout, asText(v), time.Local); err == nil {
			return t
		}
	}
	return time.Time{}
}

func asDateTime(v any) time.Time {
	s := strings.Replace(asText(v), " ", "T", 1)
	for _, layout := range []string{"2006-01-02T15:04:05", "2006-01-02T15:04"} {
		if t, err := time.ParseInLocation(layout, s, time.Local); err == nil {
			return t
		}
	}
	return time.Time{}
}

func dateText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	return t.Format("2006-01-02")
}

func timeText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	if t.Second() == 0 {
		return t.Format("15:04")
	}
	return t.Format("15:04:05")
}

func dateTimeText(t time.Time) string {
	if t.IsZero() {
		return ""
	}
	if t.Second() == 0 {
		return t.Format("2006-01-02T15:04")
	}
	return t.Format("2006-01-02T15:04:05")
}

func asList(v any) []string {
	var out []string
	for _, s := range strings.Split(asText(v), ";") {
		if s = strings.TrimSpace(s); s != "" {
			out = append(out, s)
		}
	}
	return out
}

func asNumbers(v any) []float64 {
	var out []float64
	for _, s := range asList(v) {
		if f, err := strconv.ParseFloat(s, 64); err == nil {
			out = append(out, f)
		}
	}
	return out
}

func asColors(v any) []aylis.Color {
	var out []aylis.Color
	for _, s := range asList(v) {
		if c, err := aylis.ColorHex(s); err == nil {
			out = append(out, c)
		}
	}
	return out
}

func numbersText(v []float64) string {
	out := make([]string, len(v))
	for i, f := range v {
		out[i] = strconv.FormatFloat(f, 'g', -1, 64)
	}
	return strings.Join(out, ";")
}

func colorsText(v []aylis.Color) string {
	out := make([]string, len(v))
	for i, c := range v {
		out[i] = c.JSON()
	}
	return strings.Join(out, ";")
}

// DateRange é o intervalo de datas (o on_change do DateRangePicker); a data zero: sem a data.
type DateRange struct {
	Start Date
	End   Date
}

func asDateRange(v any) DateRange {
	p := strings.SplitN(asText(v)+";", ";", 3)
	return DateRange{Start: asDate(strings.TrimSpace(p[0])), End: asDate(strings.TrimSpace(p[1]))}
}

// Align são os valores de um enum do acesso tipado.
type Align string

const (
	AlignNone Align = "none"
	AlignTop Align = "top"
	AlignBottom Align = "bottom"
	AlignLeft Align = "left"
	AlignRight Align = "right"
	AlignMostTop Align = "most_top"
	AlignMostBottom Align = "most_bottom"
	AlignMostLeft Align = "most_left"
	AlignMostRight Align = "most_right"
	AlignClient Align = "client"
	AlignContents Align = "contents"
	AlignCenter Align = "center"
	AlignVertCenter Align = "vert_center"
	AlignHorzCenter Align = "horz_center"
	AlignHorizontal Align = "horizontal"
	AlignVertical Align = "vertical"
	AlignFit Align = "fit"
)

func enumListAlign(v any) []Align {
	var out []Align
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Align(asText(x)))
		}
	}
	return out
}

func enumNamesAlign(vs []Align) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// Anchor são os valores de um enum do acesso tipado.
type Anchor string

const (
	AnchorLeft Anchor = "left"
	AnchorTop Anchor = "top"
	AnchorRight Anchor = "right"
	AnchorBottom Anchor = "bottom"
)

func enumListAnchor(v any) []Anchor {
	var out []Anchor
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Anchor(asText(x)))
		}
	}
	return out
}

func enumNamesAnchor(vs []Anchor) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// BlendMode são os valores de um enum do acesso tipado.
type BlendMode string

const (
	BlendModeNormal BlendMode = "normal"
	BlendModeMultiply BlendMode = "multiply"
	BlendModeScreen BlendMode = "screen"
	BlendModeOverlay BlendMode = "overlay"
	BlendModeDarken BlendMode = "darken"
	BlendModeLighten BlendMode = "lighten"
	BlendModeColorDodge BlendMode = "color_dodge"
	BlendModeColorBurn BlendMode = "color_burn"
	BlendModeHardLight BlendMode = "hard_light"
	BlendModeSoftLight BlendMode = "soft_light"
	BlendModeDifference BlendMode = "difference"
	BlendModeExclusion BlendMode = "exclusion"
	BlendModeHue BlendMode = "hue"
	BlendModeSaturation BlendMode = "saturation"
	BlendModeColor BlendMode = "color"
	BlendModeLuminosity BlendMode = "luminosity"
)

func enumListBlendMode(v any) []BlendMode {
	var out []BlendMode
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, BlendMode(asText(x)))
		}
	}
	return out
}

func enumNamesBlendMode(vs []BlendMode) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// BorderStyle são os valores de um enum do acesso tipado.
type BorderStyle string

const (
	BorderStyleSizeable BorderStyle = "sizeable"
	BorderStyleSingle BorderStyle = "single"
	BorderStyleNone BorderStyle = "none"
	BorderStyleCustom BorderStyle = "custom"
)

func enumListBorderStyle(v any) []BorderStyle {
	var out []BorderStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, BorderStyle(asText(x)))
		}
	}
	return out
}

func enumNamesBorderStyle(vs []BorderStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ButtonStyle são os valores de um enum do acesso tipado.
type ButtonStyle string

const (
	ButtonStyleDefault ButtonStyle = ""
	ButtonStyleAccent ButtonStyle = "accent"
	ButtonStyleDanger ButtonStyle = "danger"
	ButtonStyleElevated ButtonStyle = "elevated"
	ButtonStyleFab ButtonStyle = "fab"
	ButtonStyleFabExtended ButtonStyle = "fab_extended"
	ButtonStyleFlat ButtonStyle = "flat"
	ButtonStyleGhost ButtonStyle = "ghost"
	ButtonStyleGlass ButtonStyle = "glass"
	ButtonStyleGlassProminent ButtonStyle = "glass_prominent"
	ButtonStyleGlossy ButtonStyle = "glossy"
	ButtonStyleIcon ButtonStyle = "icon"
	ButtonStyleIconFilled ButtonStyle = "icon_filled"
	ButtonStyleIconOutlined ButtonStyle = "icon_outlined"
	ButtonStyleIconTonal ButtonStyle = "icon_tonal"
	ButtonStyleInfo ButtonStyle = "info"
	ButtonStyleLink ButtonStyle = "link"
	ButtonStyleNegative ButtonStyle = "negative"
	ButtonStyleOutline ButtonStyle = "outline"
	ButtonStylePill ButtonStyle = "pill"
	ButtonStylePositive ButtonStyle = "positive"
	ButtonStylePush ButtonStyle = "push"
	ButtonStyleReveal ButtonStyle = "reveal"
	ButtonStyleRound ButtonStyle = "round"
	ButtonStyleRounded ButtonStyle = "rounded"
	ButtonStyleSecondary ButtonStyle = "secondary"
	ButtonStyleSuccess ButtonStyle = "success"
	ButtonStyleText ButtonStyle = "text"
	ButtonStyleTonal ButtonStyle = "tonal"
	ButtonStyleUnelevated ButtonStyle = "unelevated"
	ButtonStyleWarning ButtonStyle = "warning"
)

func enumListButtonStyle(v any) []ButtonStyle {
	var out []ButtonStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ButtonStyle(asText(x)))
		}
	}
	return out
}

func enumNamesButtonStyle(vs []ButtonStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// CloseAction são os valores de um enum do acesso tipado.
type CloseAction string

const (
	CloseActionHide CloseAction = "hide"
	CloseActionFree CloseAction = "free"
)

func enumListCloseAction(v any) []CloseAction {
	var out []CloseAction
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CloseAction(asText(x)))
		}
	}
	return out
}

func enumNamesCloseAction(vs []CloseAction) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// CursorConfine são os valores de um enum do acesso tipado.
type CursorConfine string

const (
	CursorConfineNone CursorConfine = "none"
	CursorConfineWindow CursorConfine = "window"
	CursorConfineMonitor CursorConfine = "monitor"
)

func enumListCursorConfine(v any) []CursorConfine {
	var out []CursorConfine
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CursorConfine(asText(x)))
		}
	}
	return out
}

func enumNamesCursorConfine(vs []CursorConfine) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// CursorShape são os valores de um enum do acesso tipado.
type CursorShape string

const (
	CursorShapeDefault CursorShape = "default"
	CursorShapePointer CursorShape = "pointer"
	CursorShapeText CursorShape = "text"
	CursorShapeWait CursorShape = "wait"
	CursorShapeCrosshair CursorShape = "crosshair"
	CursorShapeMove CursorShape = "move"
	CursorShapeNotAllowed CursorShape = "not_allowed"
	CursorShapeResizeHorizontal CursorShape = "resize_horizontal"
	CursorShapeResizeVertical CursorShape = "resize_vertical"
	CursorShapeResizeNeSw CursorShape = "resize_ne_sw"
	CursorShapeResizeNwSe CursorShape = "resize_nw_se"
	CursorShapeGrab CursorShape = "grab"
	CursorShapeGrabbing CursorShape = "grabbing"
	CursorShapeNone CursorShape = "none"
)

func enumListCursorShape(v any) []CursorShape {
	var out []CursorShape
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CursorShape(asText(x)))
		}
	}
	return out
}

func enumNamesCursorShape(vs []CursorShape) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FontWeight são os valores de um enum do acesso tipado.
type FontWeight string

const (
	FontWeightDefault FontWeight = "default"
	FontWeightThin FontWeight = "thin"
	FontWeightLight FontWeight = "light"
	FontWeightNormal FontWeight = "normal"
	FontWeightMedium FontWeight = "medium"
	FontWeightSemibold FontWeight = "semibold"
	FontWeightBold FontWeight = "bold"
	FontWeightBlack FontWeight = "black"
)

func enumListFontWeight(v any) []FontWeight {
	var out []FontWeight
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FontWeight(asText(x)))
		}
	}
	return out
}

func enumNamesFontWeight(vs []FontWeight) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FormBlendCompositor são os valores de um enum do acesso tipado.
type FormBlendCompositor string

const (
	FormBlendCompositorProject FormBlendCompositor = "project"
	FormBlendCompositorDirect2d FormBlendCompositor = "direct2d"
	FormBlendCompositorDirect3d FormBlendCompositor = "direct3d"
)

func enumListFormBlendCompositor(v any) []FormBlendCompositor {
	var out []FormBlendCompositor
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FormBlendCompositor(asText(x)))
		}
	}
	return out
}

func enumNamesFormBlendCompositor(vs []FormBlendCompositor) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FormOrientation são os valores de um enum do acesso tipado.
type FormOrientation string

const (
	FormOrientationProject FormOrientation = "project"
	FormOrientationAny FormOrientation = "any"
	FormOrientationPortrait FormOrientation = "portrait"
	FormOrientationLandscape FormOrientation = "landscape"
	FormOrientationSensorPortrait FormOrientation = "sensor_portrait"
	FormOrientationSensorLandscape FormOrientation = "sensor_landscape"
)

func enumListFormOrientation(v any) []FormOrientation {
	var out []FormOrientation
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FormOrientation(asText(x)))
		}
	}
	return out
}

func enumNamesFormOrientation(vs []FormOrientation) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FormPosition são os valores de um enum do acesso tipado.
type FormPosition string

const (
	FormPositionCenter FormPosition = "center"
	FormPositionScreenCenter FormPosition = "screen_center"
	FormPositionDesigned FormPosition = "designed"
	FormPositionDefault FormPosition = "default"
	FormPositionTopLeft FormPosition = "top_left"
	FormPositionTopRight FormPosition = "top_right"
	FormPositionBottomLeft FormPosition = "bottom_left"
	FormPositionBottomRight FormPosition = "bottom_right"
)

func enumListFormPosition(v any) []FormPosition {
	var out []FormPosition
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FormPosition(asText(x)))
		}
	}
	return out
}

func enumNamesFormPosition(vs []FormPosition) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FormPresent são os valores de um enum do acesso tipado.
type FormPresent string

const (
	FormPresentProject FormPresent = "project"
	FormPresentWindow FormPresent = "window"
	FormPresentComposition FormPresent = "composition"
	FormPresentComposed FormPresent = "composed"
)

func enumListFormPresent(v any) []FormPresent {
	var out []FormPresent
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FormPresent(asText(x)))
		}
	}
	return out
}

func enumNamesFormPresent(vs []FormPresent) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FormStyle são os valores de um enum do acesso tipado.
type FormStyle string

const (
	FormStyleDefault FormStyle = ""
)

func enumListFormStyle(v any) []FormStyle {
	var out []FormStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FormStyle(asText(x)))
		}
	}
	return out
}

func enumNamesFormStyle(vs []FormStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FullScreenMode são os valores de um enum do acesso tipado.
type FullScreenMode string

const (
	FullScreenModeBorderless FullScreenMode = "borderless"
	FullScreenModeExclusive FullScreenMode = "exclusive"
)

func enumListFullScreenMode(v any) []FullScreenMode {
	var out []FullScreenMode
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FullScreenMode(asText(x)))
		}
	}
	return out
}

func enumNamesFullScreenMode(vs []FullScreenMode) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// GridLayoutStyle são os valores de um enum do acesso tipado.
type GridLayoutStyle string

const (
	GridLayoutStyleDefault GridLayoutStyle = ""
)

func enumListGridLayoutStyle(v any) []GridLayoutStyle {
	var out []GridLayoutStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, GridLayoutStyle(asText(x)))
		}
	}
	return out
}

func enumNamesGridLayoutStyle(vs []GridLayoutStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// IconPosition são os valores de um enum do acesso tipado.
type IconPosition string

const (
	IconPositionLeft IconPosition = "left"
	IconPositionRight IconPosition = "right"
	IconPositionTop IconPosition = "top"
)

func enumListIconPosition(v any) []IconPosition {
	var out []IconPosition
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, IconPosition(asText(x)))
		}
	}
	return out
}

func enumNamesIconPosition(vs []IconPosition) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// KeepAwake são os valores de um enum do acesso tipado.
type KeepAwake string

const (
	KeepAwakeOff KeepAwake = "off"
	KeepAwakeSystem KeepAwake = "system"
	KeepAwakeDisplay KeepAwake = "display"
)

func enumListKeepAwake(v any) []KeepAwake {
	var out []KeepAwake
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, KeepAwake(asText(x)))
		}
	}
	return out
}

func enumNamesKeepAwake(vs []KeepAwake) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// KeyCode são os valores de um enum do acesso tipado.
type KeyCode string

const (
	KeyCodeUnidentified KeyCode = "Unidentified"
	KeyCodeKeyA KeyCode = "KeyA"
	KeyCodeKeyB KeyCode = "KeyB"
	KeyCodeKeyC KeyCode = "KeyC"
	KeyCodeKeyD KeyCode = "KeyD"
	KeyCodeKeyE KeyCode = "KeyE"
	KeyCodeKeyF KeyCode = "KeyF"
	KeyCodeKeyG KeyCode = "KeyG"
	KeyCodeKeyH KeyCode = "KeyH"
	KeyCodeKeyI KeyCode = "KeyI"
	KeyCodeKeyJ KeyCode = "KeyJ"
	KeyCodeKeyK KeyCode = "KeyK"
	KeyCodeKeyL KeyCode = "KeyL"
	KeyCodeKeyM KeyCode = "KeyM"
	KeyCodeKeyN KeyCode = "KeyN"
	KeyCodeKeyO KeyCode = "KeyO"
	KeyCodeKeyP KeyCode = "KeyP"
	KeyCodeKeyQ KeyCode = "KeyQ"
	KeyCodeKeyR KeyCode = "KeyR"
	KeyCodeKeyS KeyCode = "KeyS"
	KeyCodeKeyT KeyCode = "KeyT"
	KeyCodeKeyU KeyCode = "KeyU"
	KeyCodeKeyV KeyCode = "KeyV"
	KeyCodeKeyW KeyCode = "KeyW"
	KeyCodeKeyX KeyCode = "KeyX"
	KeyCodeKeyY KeyCode = "KeyY"
	KeyCodeKeyZ KeyCode = "KeyZ"
	KeyCodeDigit0 KeyCode = "Digit0"
	KeyCodeDigit1 KeyCode = "Digit1"
	KeyCodeDigit2 KeyCode = "Digit2"
	KeyCodeDigit3 KeyCode = "Digit3"
	KeyCodeDigit4 KeyCode = "Digit4"
	KeyCodeDigit5 KeyCode = "Digit5"
	KeyCodeDigit6 KeyCode = "Digit6"
	KeyCodeDigit7 KeyCode = "Digit7"
	KeyCodeDigit8 KeyCode = "Digit8"
	KeyCodeDigit9 KeyCode = "Digit9"
	KeyCodeNumpad0 KeyCode = "Numpad0"
	KeyCodeNumpad1 KeyCode = "Numpad1"
	KeyCodeNumpad2 KeyCode = "Numpad2"
	KeyCodeNumpad3 KeyCode = "Numpad3"
	KeyCodeNumpad4 KeyCode = "Numpad4"
	KeyCodeNumpad5 KeyCode = "Numpad5"
	KeyCodeNumpad6 KeyCode = "Numpad6"
	KeyCodeNumpad7 KeyCode = "Numpad7"
	KeyCodeNumpad8 KeyCode = "Numpad8"
	KeyCodeNumpad9 KeyCode = "Numpad9"
	KeyCodeF1 KeyCode = "F1"
	KeyCodeF2 KeyCode = "F2"
	KeyCodeF3 KeyCode = "F3"
	KeyCodeF4 KeyCode = "F4"
	KeyCodeF5 KeyCode = "F5"
	KeyCodeF6 KeyCode = "F6"
	KeyCodeF7 KeyCode = "F7"
	KeyCodeF8 KeyCode = "F8"
	KeyCodeF9 KeyCode = "F9"
	KeyCodeF10 KeyCode = "F10"
	KeyCodeF11 KeyCode = "F11"
	KeyCodeF12 KeyCode = "F12"
	KeyCodeF13 KeyCode = "F13"
	KeyCodeF14 KeyCode = "F14"
	KeyCodeF15 KeyCode = "F15"
	KeyCodeF16 KeyCode = "F16"
	KeyCodeF17 KeyCode = "F17"
	KeyCodeF18 KeyCode = "F18"
	KeyCodeF19 KeyCode = "F19"
	KeyCodeF20 KeyCode = "F20"
	KeyCodeF21 KeyCode = "F21"
	KeyCodeF22 KeyCode = "F22"
	KeyCodeF23 KeyCode = "F23"
	KeyCodeF24 KeyCode = "F24"
	KeyCodeF25 KeyCode = "F25"
	KeyCodeF26 KeyCode = "F26"
	KeyCodeF27 KeyCode = "F27"
	KeyCodeF28 KeyCode = "F28"
	KeyCodeF29 KeyCode = "F29"
	KeyCodeF30 KeyCode = "F30"
	KeyCodeF31 KeyCode = "F31"
	KeyCodeF32 KeyCode = "F32"
	KeyCodeF33 KeyCode = "F33"
	KeyCodeF34 KeyCode = "F34"
	KeyCodeF35 KeyCode = "F35"
	KeyCodeLang1 KeyCode = "Lang1"
	KeyCodeLang2 KeyCode = "Lang2"
	KeyCodeLang3 KeyCode = "Lang3"
	KeyCodeLang4 KeyCode = "Lang4"
	KeyCodeLang5 KeyCode = "Lang5"
	KeyCodeBackquote KeyCode = "Backquote"
	KeyCodeBackslash KeyCode = "Backslash"
	KeyCodeBracketLeft KeyCode = "BracketLeft"
	KeyCodeBracketRight KeyCode = "BracketRight"
	KeyCodeComma KeyCode = "Comma"
	KeyCodeEqual KeyCode = "Equal"
	KeyCodeIntlBackslash KeyCode = "IntlBackslash"
	KeyCodeIntlRo KeyCode = "IntlRo"
	KeyCodeIntlYen KeyCode = "IntlYen"
	KeyCodeMinus KeyCode = "Minus"
	KeyCodePeriod KeyCode = "Period"
	KeyCodeQuote KeyCode = "Quote"
	KeyCodeSemicolon KeyCode = "Semicolon"
	KeyCodeSlash KeyCode = "Slash"
	KeyCodeAltLeft KeyCode = "AltLeft"
	KeyCodeAltRight KeyCode = "AltRight"
	KeyCodeBackspace KeyCode = "Backspace"
	KeyCodeCapsLock KeyCode = "CapsLock"
	KeyCodeContextMenu KeyCode = "ContextMenu"
	KeyCodeControlLeft KeyCode = "ControlLeft"
	KeyCodeControlRight KeyCode = "ControlRight"
	KeyCodeEnter KeyCode = "Enter"
	KeyCodeMetaLeft KeyCode = "MetaLeft"
	KeyCodeMetaRight KeyCode = "MetaRight"
	KeyCodeShiftLeft KeyCode = "ShiftLeft"
	KeyCodeShiftRight KeyCode = "ShiftRight"
	KeyCodeSpace KeyCode = "Space"
	KeyCodeTab KeyCode = "Tab"
	KeyCodeConvert KeyCode = "Convert"
	KeyCodeKanaMode KeyCode = "KanaMode"
	KeyCodeNonConvert KeyCode = "NonConvert"
	KeyCodeDelete KeyCode = "Delete"
	KeyCodeEnd KeyCode = "End"
	KeyCodeHelp KeyCode = "Help"
	KeyCodeHome KeyCode = "Home"
	KeyCodeInsert KeyCode = "Insert"
	KeyCodePageDown KeyCode = "PageDown"
	KeyCodePageUp KeyCode = "PageUp"
	KeyCodeArrowDown KeyCode = "ArrowDown"
	KeyCodeArrowLeft KeyCode = "ArrowLeft"
	KeyCodeArrowRight KeyCode = "ArrowRight"
	KeyCodeArrowUp KeyCode = "ArrowUp"
	KeyCodeNumLock KeyCode = "NumLock"
	KeyCodeNumpadAdd KeyCode = "NumpadAdd"
	KeyCodeNumpadComma KeyCode = "NumpadComma"
	KeyCodeNumpadDecimal KeyCode = "NumpadDecimal"
	KeyCodeNumpadDivide KeyCode = "NumpadDivide"
	KeyCodeNumpadEnter KeyCode = "NumpadEnter"
	KeyCodeNumpadEqual KeyCode = "NumpadEqual"
	KeyCodeNumpadMultiply KeyCode = "NumpadMultiply"
	KeyCodeNumpadSubtract KeyCode = "NumpadSubtract"
	KeyCodeEscape KeyCode = "Escape"
	KeyCodeFn KeyCode = "Fn"
	KeyCodePrintScreen KeyCode = "PrintScreen"
	KeyCodeScrollLock KeyCode = "ScrollLock"
	KeyCodePause KeyCode = "Pause"
	KeyCodeAudioVolumeDown KeyCode = "AudioVolumeDown"
	KeyCodeAudioVolumeMute KeyCode = "AudioVolumeMute"
	KeyCodeAudioVolumeUp KeyCode = "AudioVolumeUp"
	KeyCodeMediaPlayPause KeyCode = "MediaPlayPause"
	KeyCodeMediaSelect KeyCode = "MediaSelect"
	KeyCodeMediaStop KeyCode = "MediaStop"
	KeyCodeMediaTrackNext KeyCode = "MediaTrackNext"
	KeyCodeMediaTrackPrevious KeyCode = "MediaTrackPrevious"
	KeyCodeBrowserBack KeyCode = "BrowserBack"
	KeyCodeBrowserFavorites KeyCode = "BrowserFavorites"
	KeyCodeBrowserForward KeyCode = "BrowserForward"
	KeyCodeBrowserHome KeyCode = "BrowserHome"
	KeyCodeBrowserRefresh KeyCode = "BrowserRefresh"
	KeyCodeBrowserSearch KeyCode = "BrowserSearch"
	KeyCodeBrowserStop KeyCode = "BrowserStop"
	KeyCodeLaunchApp1 KeyCode = "LaunchApp1"
	KeyCodeLaunchApp2 KeyCode = "LaunchApp2"
	KeyCodeLaunchMail KeyCode = "LaunchMail"
	KeyCodePower KeyCode = "Power"
	KeyCodeSleep KeyCode = "Sleep"
	KeyCodeWakeUp KeyCode = "WakeUp"
	KeyCodeNumpadBackspace KeyCode = "NumpadBackspace"
	KeyCodeNumpadClear KeyCode = "NumpadClear"
	KeyCodeNumpadClearEntry KeyCode = "NumpadClearEntry"
	KeyCodeNumpadHash KeyCode = "NumpadHash"
	KeyCodeNumpadMemoryAdd KeyCode = "NumpadMemoryAdd"
	KeyCodeNumpadMemoryClear KeyCode = "NumpadMemoryClear"
	KeyCodeNumpadMemoryRecall KeyCode = "NumpadMemoryRecall"
	KeyCodeNumpadMemoryStore KeyCode = "NumpadMemoryStore"
	KeyCodeNumpadMemorySubtract KeyCode = "NumpadMemorySubtract"
	KeyCodeNumpadParenLeft KeyCode = "NumpadParenLeft"
	KeyCodeNumpadParenRight KeyCode = "NumpadParenRight"
	KeyCodeNumpadStar KeyCode = "NumpadStar"
	KeyCodeFnLock KeyCode = "FnLock"
	KeyCodeHiragana KeyCode = "Hiragana"
	KeyCodeKatakana KeyCode = "Katakana"
	KeyCodeAgain KeyCode = "Again"
	KeyCodeCopy KeyCode = "Copy"
	KeyCodeCut KeyCode = "Cut"
	KeyCodeFind KeyCode = "Find"
	KeyCodeOpen KeyCode = "Open"
	KeyCodePaste KeyCode = "Paste"
	KeyCodeProps KeyCode = "Props"
	KeyCodeSelect KeyCode = "Select"
	KeyCodeUndo KeyCode = "Undo"
	KeyCodeResume KeyCode = "Resume"
	KeyCodeSuspend KeyCode = "Suspend"
)

func enumListKeyCode(v any) []KeyCode {
	var out []KeyCode
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, KeyCode(asText(x)))
		}
	}
	return out
}

func enumNamesKeyCode(vs []KeyCode) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// LabelStyle são os valores de um enum do acesso tipado.
type LabelStyle string

const (
	LabelStyleDefault LabelStyle = ""
	LabelStyleCaption LabelStyle = "caption"
	LabelStyleError LabelStyle = "error"
	LabelStyleHeading LabelStyle = "heading"
	LabelStyleKbd LabelStyle = "kbd"
	LabelStyleLink LabelStyle = "link"
	LabelStyleMono LabelStyle = "mono"
	LabelStyleMuted LabelStyle = "muted"
	LabelStyleTitle LabelStyle = "title"
)

func enumListLabelStyle(v any) []LabelStyle {
	var out []LabelStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, LabelStyle(asText(x)))
		}
	}
	return out
}

func enumNamesLabelStyle(vs []LabelStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// LayerCaching são os valores de um enum do acesso tipado.
type LayerCaching string

const (
	LayerCachingProject LayerCaching = "project"
	LayerCachingOn LayerCaching = "on"
	LayerCachingOff LayerCaching = "off"
)

func enumListLayerCaching(v any) []LayerCaching {
	var out []LayerCaching
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, LayerCaching(asText(x)))
		}
	}
	return out
}

func enumNamesLayerCaching(vs []LayerCaching) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// LayoutStyle são os valores de um enum do acesso tipado.
type LayoutStyle string

const (
	LayoutStyleDefault LayoutStyle = ""
)

func enumListLayoutStyle(v any) []LayoutStyle {
	var out []LayoutStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, LayoutStyle(asText(x)))
		}
	}
	return out
}

func enumNamesLayoutStyle(vs []LayoutStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ListViewStyle são os valores de um enum do acesso tipado.
type ListViewStyle string

const (
	ListViewStyleDefault ListViewStyle = ""
	ListViewStyleFlat ListViewStyle = "flat"
)

func enumListListViewStyle(v any) []ListViewStyle {
	var out []ListViewStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ListViewStyle(asText(x)))
		}
	}
	return out
}

func enumNamesListViewStyle(vs []ListViewStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ModalResult são os valores de um enum do acesso tipado.
type ModalResult string

const (
	ModalResultNone ModalResult = "none"
	ModalResultOk ModalResult = "ok"
	ModalResultCancel ModalResult = "cancel"
	ModalResultAbort ModalResult = "abort"
	ModalResultRetry ModalResult = "retry"
	ModalResultIgnore ModalResult = "ignore"
	ModalResultYes ModalResult = "yes"
	ModalResultNo ModalResult = "no"
	ModalResultClose ModalResult = "close"
)

func enumListModalResult(v any) []ModalResult {
	var out []ModalResult
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ModalResult(asText(x)))
		}
	}
	return out
}

func enumNamesModalResult(vs []ModalResult) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// PanelStyle são os valores de um enum do acesso tipado.
type PanelStyle string

const (
	PanelStyleDefault PanelStyle = ""
	PanelStyleAppbar PanelStyle = "appbar"
	PanelStyleCard PanelStyle = "card"
	PanelStyleFilled PanelStyle = "filled"
	PanelStyleFlat PanelStyle = "flat"
	PanelStyleGlass PanelStyle = "glass"
	PanelStyleOutlined PanelStyle = "outlined"
	PanelStyleOverlay PanelStyle = "overlay"
	PanelStyleReveal PanelStyle = "reveal"
	PanelStyleSidebar PanelStyle = "sidebar"
)

func enumListPanelStyle(v any) []PanelStyle {
	var out []PanelStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, PanelStyle(asText(x)))
		}
	}
	return out
}

func enumNamesPanelStyle(vs []PanelStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// SelfAlign são os valores de um enum do acesso tipado.
type SelfAlign string

const (
	SelfAlignStretch SelfAlign = "stretch"
	SelfAlignStart SelfAlign = "start"
	SelfAlignCenter SelfAlign = "center"
	SelfAlignEnd SelfAlign = "end"
)

func enumListSelfAlign(v any) []SelfAlign {
	var out []SelfAlign
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, SelfAlign(asText(x)))
		}
	}
	return out
}

func enumNamesSelfAlign(vs []SelfAlign) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TabControlStyle são os valores de um enum do acesso tipado.
type TabControlStyle string

const (
	TabControlStyleDefault TabControlStyle = ""
)

func enumListTabControlStyle(v any) []TabControlStyle {
	var out []TabControlStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TabControlStyle(asText(x)))
		}
	}
	return out
}

func enumNamesTabControlStyle(vs []TabControlStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TabItemStyle são os valores de um enum do acesso tipado.
type TabItemStyle string

const (
	TabItemStyleDefault TabItemStyle = ""
)

func enumListTabItemStyle(v any) []TabItemStyle {
	var out []TabItemStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TabItemStyle(asText(x)))
		}
	}
	return out
}

func enumNamesTabItemStyle(vs []TabItemStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TabPosition são os valores de um enum do acesso tipado.
type TabPosition string

const (
	TabPositionTop TabPosition = "top"
	TabPositionBottom TabPosition = "bottom"
	TabPositionLeft TabPosition = "left"
	TabPositionRight TabPosition = "right"
)

func enumListTabPosition(v any) []TabPosition {
	var out []TabPosition
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TabPosition(asText(x)))
		}
	}
	return out
}

func enumNamesTabPosition(vs []TabPosition) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TextAlign são os valores de um enum do acesso tipado.
type TextAlign string

const (
	TextAlignLeading TextAlign = "leading"
	TextAlignCenter TextAlign = "center"
	TextAlignTrailing TextAlign = "trailing"
	TextAlignJustify TextAlign = "justify"
)

func enumListTextAlign(v any) []TextAlign {
	var out []TextAlign
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TextAlign(asText(x)))
		}
	}
	return out
}

func enumNamesTextAlign(vs []TextAlign) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TextCase são os valores de um enum do acesso tipado.
type TextCase string

const (
	TextCaseNone TextCase = "none"
	TextCaseUpper TextCase = "upper"
	TextCaseLower TextCase = "lower"
)

func enumListTextCase(v any) []TextCase {
	var out []TextCase
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TextCase(asText(x)))
		}
	}
	return out
}

func enumNamesTextCase(vs []TextCase) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ThemeName são os valores de um enum do acesso tipado.
type ThemeName string

const (
	ThemeNameDefault ThemeName = ""
	ThemeNameAylisLight ThemeName = "Aylis Light"
	ThemeNameAylisDark ThemeName = "Aylis Dark"
	ThemeNameMaterial3Light ThemeName = "Material 3 Light"
	ThemeNameMaterial3Dark ThemeName = "Material 3 Dark"
	ThemeNameMaterial2Light ThemeName = "Material 2 Light"
	ThemeNameMaterial2Dark ThemeName = "Material 2 Dark"
	ThemeNameAndroid7Light ThemeName = "Android 7 Light"
	ThemeNameAndroid7Dark ThemeName = "Android 7 Dark"
	ThemeNameIOS26Light ThemeName = "iOS 26 Light"
	ThemeNameIOS26Dark ThemeName = "iOS 26 Dark"
	ThemeNameIOS13Light ThemeName = "iOS 13 Light"
	ThemeNameIOS13Dark ThemeName = "iOS 13 Dark"
	ThemeNameIOS7Light ThemeName = "iOS 7 Light"
	ThemeNameWinUI3Light ThemeName = "WinUI 3 Light"
	ThemeNameWinUI3Dark ThemeName = "WinUI 3 Dark"
	ThemeNameWindows11Light ThemeName = "Windows 11 Light"
	ThemeNameWindows11Dark ThemeName = "Windows 11 Dark"
	ThemeNameWindows10Light ThemeName = "Windows 10 Light"
	ThemeNameWindows10Dark ThemeName = "Windows 10 Dark"
	ThemeNameWin32 ThemeName = "Win32"
	ThemeNameWindows7 ThemeName = "Windows 7"
	ThemeNameWindowsPhoneDark ThemeName = "Windows Phone Dark"
	ThemeNameWindowsPhoneLight ThemeName = "Windows Phone Light"
	ThemeNameVuetify3Light ThemeName = "Vuetify 3 Light"
	ThemeNameVuetify3Dark ThemeName = "Vuetify 3 Dark"
	ThemeNameBootstrap5Light ThemeName = "Bootstrap 5 Light"
	ThemeNameBootstrap5Dark ThemeName = "Bootstrap 5 Dark"
	ThemeNameShadcnUiLight ThemeName = "shadcn/ui Light"
	ThemeNameShadcnUiDark ThemeName = "shadcn/ui Dark"
	ThemeNameAntDesign5Light ThemeName = "Ant Design 5 Light"
	ThemeNameAntDesign5Dark ThemeName = "Ant Design 5 Dark"
	ThemeNameChakraUILight ThemeName = "Chakra UI Light"
	ThemeNameChakraUIDark ThemeName = "Chakra UI Dark"
	ThemeNameQuasar2Light ThemeName = "Quasar 2 Light"
	ThemeNameQuasar2Dark ThemeName = "Quasar 2 Dark"
)

func enumListThemeName(v any) []ThemeName {
	var out []ThemeName
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ThemeName(asText(x)))
		}
	}
	return out
}

func enumNamesThemeName(vs []ThemeName) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// Transition são os valores de um enum do acesso tipado.
type Transition string

const (
	TransitionNone Transition = "none"
	TransitionFade Transition = "fade"
	TransitionSlideLeft Transition = "slide_left"
	TransitionSlideRight Transition = "slide_right"
	TransitionSlideUp Transition = "slide_up"
	TransitionSlideDown Transition = "slide_down"
)

func enumListTransition(v any) []Transition {
	var out []Transition
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Transition(asText(x)))
		}
	}
	return out
}

func enumNamesTransition(vs []Transition) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// Trimming são os valores de um enum do acesso tipado.
type Trimming string

const (
	TrimmingNone Trimming = "none"
	TrimmingCharacter Trimming = "character"
	TrimmingWord Trimming = "word"
)

func enumListTrimming(v any) []Trimming {
	var out []Trimming
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Trimming(asText(x)))
		}
	}
	return out
}

func enumNamesTrimming(vs []Trimming) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// WindowCorners são os valores de um enum do acesso tipado.
type WindowCorners string

const (
	WindowCornersDefault WindowCorners = "default"
	WindowCornersRound WindowCorners = "round"
	WindowCornersSmall WindowCorners = "small"
	WindowCornersSquare WindowCorners = "square"
)

func enumListWindowCorners(v any) []WindowCorners {
	var out []WindowCorners
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, WindowCorners(asText(x)))
		}
	}
	return out
}

func enumNamesWindowCorners(vs []WindowCorners) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// WindowShape são os valores de um enum do acesso tipado.
type WindowShape string

const (
	WindowShapeRectangle WindowShape = "rectangle"
	WindowShapeRounded WindowShape = "rounded"
	WindowShapeEllipse WindowShape = "ellipse"
	WindowShapePath WindowShape = "path"
)

func enumListWindowShape(v any) []WindowShape {
	var out []WindowShape
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, WindowShape(asText(x)))
		}
	}
	return out
}

func enumNamesWindowShape(vs []WindowShape) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// WindowState são os valores de um enum do acesso tipado.
type WindowState string

const (
	WindowStateNormal WindowState = "normal"
	WindowStateMaximized WindowState = "maximized"
	WindowStateMinimized WindowState = "minimized"
	WindowStateFullScreen WindowState = "full_screen"
)

func enumListWindowState(v any) []WindowState {
	var out []WindowState
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, WindowState(asText(x)))
		}
	}
	return out
}

func enumNamesWindowState(vs []WindowState) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

func jsonTexts(v any) []string {
	l, _ := v.([]any)
	out := make([]string, 0, len(l))
	for _, x := range l {
		out = append(out, asText(x))
	}
	return out
}

func jsonNumbers(v any) []float64 {
	l, _ := v.([]any)
	out := make([]float64, 0, len(l))
	for _, x := range l {
		out = append(out, asNumber(x))
	}
	return out
}

// GestureEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type GestureEvent struct {
	Phase string
	X float64
	Y float64
	Scale float64
	DeltaScale float64
	Rotation float64
	DeltaRotation float64
	PanX float64
	PanY float64
}

func recordOfGestureEvent(v any) GestureEvent {
	m, _ := v.(map[string]any)
	return GestureEvent{
		Phase: asText(m["phase"]),
		X: asNumber(m["x"]),
		Y: asNumber(m["y"]),
		Scale: asNumber(m["scale"]),
		DeltaScale: asNumber(m["delta_scale"]),
		Rotation: asNumber(m["rotation"]),
		DeltaRotation: asNumber(m["delta_rotation"]),
		PanX: asNumber(m["pan_x"]),
		PanY: asNumber(m["pan_y"]),
	}
}

func (r GestureEvent) jsonValue() map[string]any {
	return map[string]any{
		"phase": r.Phase,
		"x": r.X,
		"y": r.Y,
		"scale": r.Scale,
		"delta_scale": r.DeltaScale,
		"rotation": r.Rotation,
		"delta_rotation": r.DeltaRotation,
		"pan_x": r.PanX,
		"pan_y": r.PanY,
	}
}

func jsonListGestureEvent(items []GestureEvent) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfGestureEvent(v any) []GestureEvent {
	l, _ := v.([]any)
	out := make([]GestureEvent, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfGestureEvent(x))
	}
	return out
}

func recordGestureEvent(v any) *GestureEvent {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfGestureEvent(j)
	return &r
}

func recordsGestureEvent(v any) []GestureEvent {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfGestureEvent(j)
}

func recordTextGestureEvent(r *GestureEvent) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextGestureEvent(items []GestureEvent) string {
	b, _ := json.Marshal(jsonListGestureEvent(items))
	return string(b)
}

// KeyEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type KeyEvent struct {
	Key string
	Ctrl bool
	Shift bool
	Alt bool
	Meta bool
	CapsLock bool
	NumLock bool
	ScrollLock bool
	Code KeyCode
	Scancode int64
	Form string
	FormId int64
}

func recordOfKeyEvent(v any) KeyEvent {
	m, _ := v.(map[string]any)
	return KeyEvent{
		Key: asText(m["key"]),
		Ctrl: asBool(m["ctrl"]),
		Shift: asBool(m["shift"]),
		Alt: asBool(m["alt"]),
		Meta: asBool(m["meta"]),
		CapsLock: asBool(m["caps_lock"]),
		NumLock: asBool(m["num_lock"]),
		ScrollLock: asBool(m["scroll_lock"]),
		Code: KeyCode(asText(m["code"])),
		Scancode: int64(asNumber(m["scancode"])),
		Form: asText(m["form"]),
		FormId: int64(asNumber(m["form_id"])),
	}
}

func (r KeyEvent) jsonValue() map[string]any {
	return map[string]any{
		"key": r.Key,
		"ctrl": r.Ctrl,
		"shift": r.Shift,
		"alt": r.Alt,
		"meta": r.Meta,
		"caps_lock": r.CapsLock,
		"num_lock": r.NumLock,
		"scroll_lock": r.ScrollLock,
		"code": string(r.Code),
		"scancode": r.Scancode,
		"form": r.Form,
		"form_id": r.FormId,
	}
}

func jsonListKeyEvent(items []KeyEvent) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfKeyEvent(v any) []KeyEvent {
	l, _ := v.([]any)
	out := make([]KeyEvent, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfKeyEvent(x))
	}
	return out
}

func recordKeyEvent(v any) *KeyEvent {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfKeyEvent(j)
	return &r
}

func recordsKeyEvent(v any) []KeyEvent {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfKeyEvent(j)
}

func recordTextKeyEvent(r *KeyEvent) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextKeyEvent(items []KeyEvent) string {
	b, _ := json.Marshal(jsonListKeyEvent(items))
	return string(b)
}

// ListViewItem é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type ListViewItem struct {
	Title string
	Detail string
}

func recordOfListViewItem(v any) ListViewItem {
	m, _ := v.(map[string]any)
	return ListViewItem{
		Title: asText(m["title"]),
		Detail: asText(m["detail"]),
	}
}

func (r ListViewItem) jsonValue() map[string]any {
	return map[string]any{
		"title": r.Title,
		"detail": r.Detail,
	}
}

func jsonListListViewItem(items []ListViewItem) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfListViewItem(v any) []ListViewItem {
	l, _ := v.([]any)
	out := make([]ListViewItem, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfListViewItem(x))
	}
	return out
}

func recordListViewItem(v any) *ListViewItem {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfListViewItem(j)
	return &r
}

func recordsListViewItem(v any) []ListViewItem {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfListViewItem(j)
}

func recordTextListViewItem(r *ListViewItem) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextListViewItem(items []ListViewItem) string {
	b, _ := json.Marshal(jsonListListViewItem(items))
	return string(b)
}

// PointerEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type PointerEvent struct {
	X float64
	Y float64
	Button string
	Clicks int64
}

func recordOfPointerEvent(v any) PointerEvent {
	m, _ := v.(map[string]any)
	return PointerEvent{
		X: asNumber(m["x"]),
		Y: asNumber(m["y"]),
		Button: asText(m["button"]),
		Clicks: int64(asNumber(m["clicks"])),
	}
}

func (r PointerEvent) jsonValue() map[string]any {
	return map[string]any{
		"x": r.X,
		"y": r.Y,
		"button": r.Button,
		"clicks": r.Clicks,
	}
}

func jsonListPointerEvent(items []PointerEvent) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfPointerEvent(v any) []PointerEvent {
	l, _ := v.([]any)
	out := make([]PointerEvent, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfPointerEvent(x))
	}
	return out
}

func recordPointerEvent(v any) *PointerEvent {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfPointerEvent(j)
	return &r
}

func recordsPointerEvent(v any) []PointerEvent {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfPointerEvent(j)
}

func recordTextPointerEvent(r *PointerEvent) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextPointerEvent(items []PointerEvent) string {
	b, _ := json.Marshal(jsonListPointerEvent(items))
	return string(b)
}

// SwipeEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type SwipeEvent struct {
	Direction string
	X float64
	Y float64
	Dx float64
	Dy float64
	Vx float64
	Vy float64
}

func recordOfSwipeEvent(v any) SwipeEvent {
	m, _ := v.(map[string]any)
	return SwipeEvent{
		Direction: asText(m["direction"]),
		X: asNumber(m["x"]),
		Y: asNumber(m["y"]),
		Dx: asNumber(m["dx"]),
		Dy: asNumber(m["dy"]),
		Vx: asNumber(m["vx"]),
		Vy: asNumber(m["vy"]),
	}
}

func (r SwipeEvent) jsonValue() map[string]any {
	return map[string]any{
		"direction": r.Direction,
		"x": r.X,
		"y": r.Y,
		"dx": r.Dx,
		"dy": r.Dy,
		"vx": r.Vx,
		"vy": r.Vy,
	}
}

func jsonListSwipeEvent(items []SwipeEvent) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfSwipeEvent(v any) []SwipeEvent {
	l, _ := v.([]any)
	out := make([]SwipeEvent, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfSwipeEvent(x))
	}
	return out
}

func recordSwipeEvent(v any) *SwipeEvent {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfSwipeEvent(j)
	return &r
}

func recordsSwipeEvent(v any) []SwipeEvent {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfSwipeEvent(j)
}

func recordTextSwipeEvent(r *SwipeEvent) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextSwipeEvent(items []SwipeEvent) string {
	b, _ := json.Marshal(jsonListSwipeEvent(items))
	return string(b)
}

// WheelEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type WheelEvent struct {
	X float64
	Y float64
	Dx float64
	Dy float64
}

func recordOfWheelEvent(v any) WheelEvent {
	m, _ := v.(map[string]any)
	return WheelEvent{
		X: asNumber(m["x"]),
		Y: asNumber(m["y"]),
		Dx: asNumber(m["dx"]),
		Dy: asNumber(m["dy"]),
	}
}

func (r WheelEvent) jsonValue() map[string]any {
	return map[string]any{
		"x": r.X,
		"y": r.Y,
		"dx": r.Dx,
		"dy": r.Dy,
	}
}

func jsonListWheelEvent(items []WheelEvent) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfWheelEvent(v any) []WheelEvent {
	l, _ := v.([]any)
	out := make([]WheelEvent, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfWheelEvent(x))
	}
	return out
}

func recordWheelEvent(v any) *WheelEvent {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfWheelEvent(j)
	return &r
}

func recordsWheelEvent(v any) []WheelEvent {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfWheelEvent(j)
}

func recordTextWheelEvent(r *WheelEvent) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextWheelEvent(items []WheelEvent) string {
	b, _ := json.Marshal(jsonListWheelEvent(items))
	return string(b)
}

// aylisButton é um Button de um form.
type aylisButton struct{ aylis.Component }

func (c aylisButton) Text() string { return asText(c.Get("text")) }
func (c aylisButton) SetText(v string) { c.Set("text", v) }

func (c aylisButton) Icon() string { return asText(c.Get("icon")) }
func (c aylisButton) SetIcon(v string) { c.Set("icon", v) }

func (c aylisButton) CheckedIcon() string { return asText(c.Get("checked_icon")) }
func (c aylisButton) SetCheckedIcon(v string) { c.Set("checked_icon", v) }

func (c aylisButton) IconPosition() IconPosition { return IconPosition(asText(c.Get("icon_position"))) }
func (c aylisButton) SetIconPosition(v IconPosition) { c.Set("icon_position", string(v)) }

func (c aylisButton) IconSize() float64 { return asNumber(c.Get("icon_size")) }
func (c aylisButton) SetIconSize(v float64) { c.Set("icon_size", v) }

func (c aylisButton) Checkable() bool { return asBool(c.Get("checkable")) }
func (c aylisButton) SetCheckable(v bool) { c.Set("checkable", v) }

func (c aylisButton) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisButton) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisButton) ModalResult() ModalResult { return ModalResult(asText(c.Get("modal_result"))) }
func (c aylisButton) SetModalResult(v ModalResult) { c.Set("modal_result", string(v)) }

func (c aylisButton) RepeatEnter() bool { return asBool(c.Get("repeat_enter")) }
func (c aylisButton) SetRepeatEnter(v bool) { c.Set("repeat_enter", v) }

func (c aylisButton) DefaultButton() bool { return asBool(c.Get("default_button")) }
func (c aylisButton) SetDefaultButton(v bool) { c.Set("default_button", v) }

func (c aylisButton) CancelButton() bool { return asBool(c.Get("cancel_button")) }
func (c aylisButton) SetCancelButton(v bool) { c.Set("cancel_button", v) }

func (c aylisButton) HAlign() TextAlign { return TextAlign(asText(c.Get("h_align"))) }
func (c aylisButton) SetHAlign(v TextAlign) { c.Set("h_align", string(v)) }

func (c aylisButton) GroupIndex() int64 { return int64(asNumber(c.Get("group_index"))) }
func (c aylisButton) SetGroupIndex(v int64) { c.Set("group_index", v) }

func (c aylisButton) Radius() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("radius")) }
func (c aylisButton) SetRadius(v aylis.Bounds) { c.Set("radius", v.JSON()) }

func (c aylisButton) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisButton) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisButton) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisButton) SetLeft(v float64) { c.Set("left", v) }

func (c aylisButton) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisButton) SetTop(v float64) { c.Set("top", v) }

func (c aylisButton) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisButton) SetWidth(v float64) { c.Set("width", v) }

func (c aylisButton) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisButton) SetHeight(v float64) { c.Set("height", v) }

func (c aylisButton) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisButton) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisButton) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisButton) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisButton) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisButton) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisButton) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisButton) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisButton) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisButton) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisButton) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisButton) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisButton) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisButton) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisButton) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisButton) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisButton) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisButton) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisButton) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisButton) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisButton) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisButton) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisButton) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisButton) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisButton) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisButton) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisButton) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisButton) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisButton) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisButton) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisButton) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisButton) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisButton) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisButton) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisButton) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisButton) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisButton) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisButton) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisButton) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisButton) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisButton) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisButton) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisButton) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisButton) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisButton) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisButton) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisButton) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisButton) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisButton) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisButton) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisButton) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisButton) SetScale(v float64) { c.Set("scale", v) }

func (c aylisButton) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisButton) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisButton) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisButton) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisButton) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisButton) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisButton) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisButton) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisButton) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisButton) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisButton) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisButton) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisButton) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisButton) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisButton) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisButton) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisButton) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisButton) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisButton) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisButton) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisButton) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisButton) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisButton) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisButton) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisButton) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisButton) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisButton) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisButton) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisButton) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisButton) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisButton) Style() ButtonStyle { return ButtonStyle(asText(c.Get("style"))) }
func (c aylisButton) SetStyle(v ButtonStyle) { c.Set("style", string(v)) }

func (c aylisButton) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisButton) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisButton) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisButton) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisButton) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisButton) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisButton) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisButton) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisButton) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisButton) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisButton) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisButton) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisButton) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisButton) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisButton) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisButton) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisButton) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisButton) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisButton) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisButton) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisButton) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisButton) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisButton) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisButton) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisButton) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisButton) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisButton) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisButton) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisButton) Hint() string { return asText(c.Get("hint")) }
func (c aylisButton) SetHint(v string) { c.Set("hint", v) }

func (c aylisButton) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisButton) SetTag(v int64) { c.Set("tag", v) }

func (c aylisButton) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisButton) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisButton) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisButton) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisButton) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisButton) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisButton) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisButton) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisButton) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisButton) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisButton) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisButton) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisButton) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisButton) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisButton) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisButton) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisButton) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisButton) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisButton) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisButton) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisButton) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisButton) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisButton) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisButton) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisButton) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisButton) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisButton) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisButton) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisButton) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisButton) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisButton) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisButton) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisButton) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisButton) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisButton) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisButton) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisButton) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisButton) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisButton) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisButton) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisButton) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisButton) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisButton) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisButton) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisButton) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisForm é um Form de um form.
type aylisForm struct{ aylis.Form }

func (c aylisForm) Caption() string { return asText(c.Get("caption")) }
func (c aylisForm) SetCaption(v string) { c.Set("caption", v) }

func (c aylisForm) Position() FormPosition { return FormPosition(asText(c.Get("position"))) }
func (c aylisForm) SetPosition(v FormPosition) { c.Set("position", string(v)) }

func (c aylisForm) BorderStyle() BorderStyle { return BorderStyle(asText(c.Get("border_style"))) }
func (c aylisForm) SetBorderStyle(v BorderStyle) { c.Set("border_style", string(v)) }

func (c aylisForm) WindowCorners() WindowCorners { return WindowCorners(asText(c.Get("window_corners"))) }
func (c aylisForm) SetWindowCorners(v WindowCorners) { c.Set("window_corners", string(v)) }

func (c aylisForm) Transparent() bool { return asBool(c.Get("transparent")) }
func (c aylisForm) SetTransparent(v bool) { c.Set("transparent", v) }

func (c aylisForm) WindowShape() WindowShape { return WindowShape(asText(c.Get("window_shape"))) }
func (c aylisForm) SetWindowShape(v WindowShape) { c.Set("window_shape", string(v)) }

func (c aylisForm) ShapeRadius() float64 { return asNumber(c.Get("shape_radius")) }
func (c aylisForm) SetShapeRadius(v float64) { c.Set("shape_radius", v) }

func (c aylisForm) ShapeRadii() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("shape_radii")) }
func (c aylisForm) SetShapeRadii(v aylis.Bounds) { c.Set("shape_radii", v.JSON()) }

func (c aylisForm) ShapePath() string { return asText(c.Get("shape_path")) }
func (c aylisForm) SetShapePath(v string) { c.Set("shape_path", v) }

func (c aylisForm) WindowState() WindowState { return WindowState(asText(c.Get("window_state"))) }
func (c aylisForm) SetWindowState(v WindowState) { c.Set("window_state", string(v)) }

func (c aylisForm) Transition() Transition { return Transition(asText(c.Get("transition"))) }
func (c aylisForm) SetTransition(v Transition) { c.Set("transition", string(v)) }

func (c aylisForm) LayerCache() LayerCaching { return LayerCaching(asText(c.Get("layer_cache"))) }
func (c aylisForm) SetLayerCache(v LayerCaching) { c.Set("layer_cache", string(v)) }

func (c aylisForm) FrameReuseKb() int64 { return int64(asNumber(c.Get("frame_reuse_kb"))) }
func (c aylisForm) SetFrameReuseKb(v int64) { c.Set("frame_reuse_kb", v) }

func (c aylisForm) Topmost() bool { return asBool(c.Get("topmost")) }
func (c aylisForm) SetTopmost(v bool) { c.Set("topmost", v) }

func (c aylisForm) Bottommost() bool { return asBool(c.Get("bottommost")) }
func (c aylisForm) SetBottommost(v bool) { c.Set("bottommost", v) }

func (c aylisForm) KeepAwake() KeepAwake { return KeepAwake(asText(c.Get("keep_awake"))) }
func (c aylisForm) SetKeepAwake(v KeepAwake) { c.Set("keep_awake", string(v)) }

func (c aylisForm) ConfineCursor() CursorConfine { return CursorConfine(asText(c.Get("confine_cursor"))) }
func (c aylisForm) SetConfineCursor(v CursorConfine) { c.Set("confine_cursor", string(v)) }

func (c aylisForm) WindowOpacity() float64 { return asNumber(c.Get("window_opacity")) }
func (c aylisForm) SetWindowOpacity(v float64) { c.Set("window_opacity", v) }

func (c aylisForm) FullScreenMode() FullScreenMode { return FullScreenMode(asText(c.Get("full_screen_mode"))) }
func (c aylisForm) SetFullScreenMode(v FullScreenMode) { c.Set("full_screen_mode", string(v)) }

func (c aylisForm) VideoWidth() int64 { return int64(asNumber(c.Get("video_width"))) }
func (c aylisForm) SetVideoWidth(v int64) { c.Set("video_width", v) }

func (c aylisForm) VideoHeight() int64 { return int64(asNumber(c.Get("video_height"))) }
func (c aylisForm) SetVideoHeight(v int64) { c.Set("video_height", v) }

func (c aylisForm) VideoRefresh() float64 { return asNumber(c.Get("video_refresh")) }
func (c aylisForm) SetVideoRefresh(v float64) { c.Set("video_refresh", v) }

func (c aylisForm) ShowInTaskbar() bool { return asBool(c.Get("show_in_taskbar")) }
func (c aylisForm) SetShowInTaskbar(v bool) { c.Set("show_in_taskbar", v) }

func (c aylisForm) ActiveControl() *aylis.Component { return c.GetRef("active_control") }
func (c aylisForm) SetActiveControl(v aylis.Ref) { c.SetRef("active_control", v) }

func (c aylisForm) TabOrder() []string { return asList(c.Get("tab_order")) }
func (c aylisForm) SetTabOrder(v []string) { c.Set("tab_order", strings.Join(v, ";")) }

func (c aylisForm) TabKeys() string { return asText(c.Get("tab_keys")) }
func (c aylisForm) SetTabKeys(v string) { c.Set("tab_keys", v) }

func (c aylisForm) BackTabKeys() string { return asText(c.Get("back_tab_keys")) }
func (c aylisForm) SetBackTabKeys(v string) { c.Set("back_tab_keys", v) }

func (c aylisForm) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisForm) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisForm) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisForm) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisForm) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisForm) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisForm) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisForm) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisForm) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisForm) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisForm) KeyPreview() bool { return asBool(c.Get("key_preview")) }
func (c aylisForm) SetKeyPreview(v bool) { c.Set("key_preview", v) }

func (c aylisForm) Zoom() float64 { return asNumber(c.Get("zoom")) }
func (c aylisForm) SetZoom(v float64) { c.Set("zoom", v) }

func (c aylisForm) Monitor() int64 { return int64(asNumber(c.Get("monitor"))) }
func (c aylisForm) SetMonitor(v int64) { c.Set("monitor", v) }

func (c aylisForm) RememberPlacement() bool { return asBool(c.Get("remember_placement")) }
func (c aylisForm) SetRememberPlacement(v bool) { c.Set("remember_placement", v) }

func (c aylisForm) Gpu() string { return asText(c.Get("gpu")) }
func (c aylisForm) SetGpu(v string) { c.Set("gpu", v) }

func (c aylisForm) Present() FormPresent { return FormPresent(asText(c.Get("present"))) }
func (c aylisForm) SetPresent(v FormPresent) { c.Set("present", string(v)) }

func (c aylisForm) BlendCompositor() FormBlendCompositor { return FormBlendCompositor(asText(c.Get("blend_compositor"))) }
func (c aylisForm) SetBlendCompositor(v FormBlendCompositor) { c.Set("blend_compositor", string(v)) }

func (c aylisForm) SpanMonitors() bool { return asBool(c.Get("span_monitors")) }
func (c aylisForm) SetSpanMonitors(v bool) { c.Set("span_monitors", v) }

func (c aylisForm) PhysicalPixels() bool { return asBool(c.Get("physical_pixels")) }
func (c aylisForm) SetPhysicalPixels(v bool) { c.Set("physical_pixels", v) }

func (c aylisForm) ActivateOnShow() bool { return asBool(c.Get("activate_on_show")) }
func (c aylisForm) SetActivateOnShow(v bool) { c.Set("activate_on_show", v) }

func (c aylisForm) ForceForeground() bool { return asBool(c.Get("force_foreground")) }
func (c aylisForm) SetForceForeground(v bool) { c.Set("force_foreground", v) }

func (c aylisForm) CloseAction() CloseAction { return CloseAction(asText(c.Get("close_action"))) }
func (c aylisForm) SetCloseAction(v CloseAction) { c.Set("close_action", string(v)) }

func (c aylisForm) FrameRate() float64 { return asNumber(c.Get("frame_rate")) }
func (c aylisForm) SetFrameRate(v float64) { c.Set("frame_rate", v) }

func (c aylisForm) Orientation() FormOrientation { return FormOrientation(asText(c.Get("orientation"))) }
func (c aylisForm) SetOrientation(v FormOrientation) { c.Set("orientation", string(v)) }

func (c aylisForm) Host() *aylis.Component { return c.GetRef("host") }
func (c aylisForm) SetHost(v aylis.Ref) { c.SetRef("host", v) }

func (c aylisForm) Dockable() bool { return asBool(c.Get("dockable")) }
func (c aylisForm) SetDockable(v bool) { c.Set("dockable", v) }

func (c aylisForm) OnShow() string { return asText(c.Get("on_show")) }
func (c aylisForm) SetOnShow(v string) { c.Set("on_show", v) }

func (c aylisForm) OnWindowCreated() string { return asText(c.Get("on_window_created")) }
func (c aylisForm) SetOnWindowCreated(v string) { c.Set("on_window_created", v) }

func (c aylisForm) OnClose() string { return asText(c.Get("on_close")) }
func (c aylisForm) SetOnClose(v string) { c.Set("on_close", v) }

func (c aylisForm) OnActivate() string { return asText(c.Get("on_activate")) }
func (c aylisForm) SetOnActivate(v string) { c.Set("on_activate", v) }

func (c aylisForm) OnDeactivate() string { return asText(c.Get("on_deactivate")) }
func (c aylisForm) SetOnDeactivate(v string) { c.Set("on_deactivate", v) }

func (c aylisForm) OnMinimize() string { return asText(c.Get("on_minimize")) }
func (c aylisForm) SetOnMinimize(v string) { c.Set("on_minimize", v) }

func (c aylisForm) OnRestore() string { return asText(c.Get("on_restore")) }
func (c aylisForm) SetOnRestore(v string) { c.Set("on_restore", v) }

func (c aylisForm) OnNavigate() string { return asText(c.Get("on_navigate")) }
func (c aylisForm) SetOnNavigate(v string) { c.Set("on_navigate", v) }

func (c aylisForm) OnGpuChange() string { return asText(c.Get("on_gpu_change")) }
func (c aylisForm) SetOnGpuChange(v string) { c.Set("on_gpu_change", v) }

func (c aylisForm) OnDock() string { return asText(c.Get("on_dock")) }
func (c aylisForm) SetOnDock(v string) { c.Set("on_dock", v) }

func (c aylisForm) OnUndock() string { return asText(c.Get("on_undock")) }
func (c aylisForm) SetOnUndock(v string) { c.Set("on_undock", v) }

func (c aylisForm) OnFilesDrop() string { return asText(c.Get("on_files_drop")) }
func (c aylisForm) SetOnFilesDrop(v string) { c.Set("on_files_drop", v) }

func (c aylisForm) OnViewChange() string { return asText(c.Get("on_view_change")) }
func (c aylisForm) SetOnViewChange(v string) { c.Set("on_view_change", v) }

func (c aylisForm) OnTopmostLost() string { return asText(c.Get("on_topmost_lost")) }
func (c aylisForm) SetOnTopmostLost(v string) { c.Set("on_topmost_lost", v) }

func (c aylisForm) OnScaleChange() string { return asText(c.Get("on_scale_change")) }
func (c aylisForm) SetOnScaleChange(v string) { c.Set("on_scale_change", v) }

func (c aylisForm) OnResizeEnd() string { return asText(c.Get("on_resize_end")) }
func (c aylisForm) SetOnResizeEnd(v string) { c.Set("on_resize_end", v) }

func (c aylisForm) ClientWidth() float64 { return asNumber(c.Get("client_width")) }
func (c aylisForm) SetClientWidth(v float64) { c.Set("client_width", v) }

func (c aylisForm) ClientHeight() float64 { return asNumber(c.Get("client_height")) }
func (c aylisForm) SetClientHeight(v float64) { c.Set("client_height", v) }

func (c aylisForm) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisForm) SetLeft(v float64) { c.Set("left", v) }

func (c aylisForm) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisForm) SetTop(v float64) { c.Set("top", v) }

func (c aylisForm) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisForm) SetWidth(v float64) { c.Set("width", v) }

func (c aylisForm) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisForm) SetHeight(v float64) { c.Set("height", v) }

func (c aylisForm) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisForm) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisForm) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisForm) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisForm) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisForm) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisForm) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisForm) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisForm) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisForm) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisForm) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisForm) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisForm) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisForm) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisForm) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisForm) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisForm) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisForm) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisForm) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisForm) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisForm) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisForm) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisForm) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisForm) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisForm) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisForm) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisForm) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisForm) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisForm) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisForm) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisForm) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisForm) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisForm) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisForm) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisForm) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisForm) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisForm) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisForm) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisForm) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisForm) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisForm) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisForm) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisForm) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisForm) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisForm) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisForm) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisForm) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisForm) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisForm) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisForm) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisForm) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisForm) SetScale(v float64) { c.Set("scale", v) }

func (c aylisForm) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisForm) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisForm) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisForm) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisForm) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisForm) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisForm) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisForm) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisForm) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisForm) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisForm) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisForm) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisForm) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisForm) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisForm) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisForm) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisForm) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisForm) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisForm) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisForm) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisForm) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisForm) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisForm) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisForm) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisForm) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisForm) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisForm) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisForm) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisForm) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisForm) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisForm) Style() FormStyle { return FormStyle(asText(c.Get("style"))) }
func (c aylisForm) SetStyle(v FormStyle) { c.Set("style", string(v)) }

func (c aylisForm) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisForm) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisForm) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisForm) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisForm) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisForm) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisForm) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisForm) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisForm) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisForm) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisForm) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisForm) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisForm) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisForm) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisForm) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisForm) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisForm) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisForm) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisForm) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisForm) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisForm) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisForm) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisForm) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisForm) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisForm) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisForm) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisForm) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisForm) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisForm) Hint() string { return asText(c.Get("hint")) }
func (c aylisForm) SetHint(v string) { c.Set("hint", v) }

func (c aylisForm) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisForm) SetTag(v int64) { c.Set("tag", v) }

func (c aylisForm) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisForm) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisForm) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisForm) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisForm) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisForm) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisForm) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisForm) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisForm) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisForm) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisForm) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisForm) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisForm) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisForm) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisForm) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisForm) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisForm) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisForm) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisForm) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisForm) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisForm) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisForm) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisForm) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisForm) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisForm) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisForm) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisForm) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisForm) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisForm) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisForm) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisForm) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisForm) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisForm) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisForm) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisForm) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisForm) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisForm) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisForm) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisForm) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisForm) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisForm) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisForm) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisForm) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisForm) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisForm) MoveToMonitor(monitor int64) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "move_to_monitor", "args": []any{monitor}}) }
func (c aylisForm) RequestAttention(critical bool) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "request_attention", "args": []any{critical}}) }
func (c aylisForm) BringToFront() { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "bring_to_front", "args": []any{}}) }
func (c aylisForm) SetActivationToken(token string) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "set_activation_token", "args": []any{token}}) }
func (c aylisForm) WindowScale() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "window_scale", "args": []any{}})) }
func (c aylisForm) PointerOver() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "pointer_over", "args": []any{}})) }
func (c aylisForm) Place(monitor int64, x int64, y int64, width int64, height int64, activate bool) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "place", "args": []any{monitor, x, y, width, height, activate}}) }
func (c aylisForm) GpuAdapter() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapter", "args": []any{}})) }
func (c aylisForm) GpuAdapters() []string { return asList(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapters", "args": []any{}})) }
func (c aylisForm) DropX() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "drop_x", "args": []any{}})) }
func (c aylisForm) DropY() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "drop_y", "args": []any{}})) }
func (c aylisForm) RenderRgba(x float64, y float64, width float64, height float64, scale float64) string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "render_rgba", "args": []any{x, y, width, height, scale}})) }
func (c aylisForm) Focused() string { return asText(askCmd(c.App, "focused", aylis.M{"target": c.ID})) }
func (c aylisForm) FocusNext() { c.App.Send("focus_next", aylis.M{"target": c.ID}) }
func (c aylisForm) FocusPrev() { c.App.Send("focus_prev", aylis.M{"target": c.ID}) }

// aylisGridLayout é um GridLayout de um form.
type aylisGridLayout struct{ aylis.Component }

func (c aylisGridLayout) Columns() int64 { return int64(asNumber(c.Get("columns"))) }
func (c aylisGridLayout) SetColumns(v int64) { c.Set("columns", v) }

func (c aylisGridLayout) MinColumnWidth() float64 { return asNumber(c.Get("min_column_width")) }
func (c aylisGridLayout) SetMinColumnWidth(v float64) { c.Set("min_column_width", v) }

func (c aylisGridLayout) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisGridLayout) SetGap(v float64) { c.Set("gap", v) }

func (c aylisGridLayout) RowHeight() float64 { return asNumber(c.Get("row_height")) }
func (c aylisGridLayout) SetRowHeight(v float64) { c.Set("row_height", v) }

func (c aylisGridLayout) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisGridLayout) SetLeft(v float64) { c.Set("left", v) }

func (c aylisGridLayout) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisGridLayout) SetTop(v float64) { c.Set("top", v) }

func (c aylisGridLayout) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisGridLayout) SetWidth(v float64) { c.Set("width", v) }

func (c aylisGridLayout) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisGridLayout) SetHeight(v float64) { c.Set("height", v) }

func (c aylisGridLayout) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisGridLayout) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisGridLayout) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisGridLayout) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisGridLayout) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisGridLayout) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisGridLayout) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisGridLayout) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisGridLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisGridLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisGridLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisGridLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisGridLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisGridLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisGridLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisGridLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisGridLayout) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisGridLayout) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisGridLayout) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisGridLayout) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisGridLayout) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisGridLayout) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisGridLayout) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisGridLayout) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisGridLayout) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisGridLayout) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisGridLayout) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisGridLayout) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisGridLayout) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisGridLayout) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisGridLayout) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisGridLayout) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisGridLayout) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisGridLayout) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisGridLayout) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisGridLayout) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisGridLayout) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisGridLayout) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisGridLayout) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisGridLayout) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisGridLayout) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisGridLayout) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisGridLayout) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisGridLayout) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisGridLayout) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisGridLayout) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisGridLayout) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisGridLayout) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisGridLayout) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisGridLayout) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisGridLayout) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisGridLayout) SetScale(v float64) { c.Set("scale", v) }

func (c aylisGridLayout) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisGridLayout) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisGridLayout) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisGridLayout) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisGridLayout) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisGridLayout) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisGridLayout) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisGridLayout) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisGridLayout) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisGridLayout) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisGridLayout) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisGridLayout) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisGridLayout) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisGridLayout) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisGridLayout) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisGridLayout) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisGridLayout) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisGridLayout) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisGridLayout) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisGridLayout) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisGridLayout) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisGridLayout) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisGridLayout) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisGridLayout) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisGridLayout) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisGridLayout) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisGridLayout) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisGridLayout) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisGridLayout) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisGridLayout) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisGridLayout) Style() GridLayoutStyle { return GridLayoutStyle(asText(c.Get("style"))) }
func (c aylisGridLayout) SetStyle(v GridLayoutStyle) { c.Set("style", string(v)) }

func (c aylisGridLayout) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisGridLayout) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisGridLayout) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisGridLayout) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisGridLayout) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisGridLayout) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisGridLayout) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisGridLayout) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisGridLayout) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisGridLayout) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisGridLayout) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisGridLayout) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisGridLayout) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisGridLayout) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisGridLayout) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisGridLayout) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisGridLayout) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisGridLayout) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisGridLayout) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisGridLayout) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisGridLayout) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisGridLayout) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisGridLayout) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisGridLayout) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisGridLayout) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisGridLayout) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisGridLayout) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisGridLayout) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisGridLayout) Hint() string { return asText(c.Get("hint")) }
func (c aylisGridLayout) SetHint(v string) { c.Set("hint", v) }

func (c aylisGridLayout) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisGridLayout) SetTag(v int64) { c.Set("tag", v) }

func (c aylisGridLayout) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisGridLayout) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisGridLayout) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisGridLayout) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisGridLayout) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisGridLayout) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisGridLayout) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisGridLayout) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisGridLayout) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisGridLayout) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisGridLayout) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisGridLayout) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisGridLayout) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisGridLayout) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisGridLayout) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisGridLayout) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisGridLayout) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisGridLayout) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisGridLayout) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisGridLayout) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisGridLayout) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisGridLayout) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisGridLayout) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisGridLayout) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisGridLayout) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisGridLayout) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisGridLayout) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisGridLayout) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisGridLayout) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisGridLayout) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisGridLayout) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisGridLayout) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisGridLayout) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisGridLayout) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisGridLayout) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisGridLayout) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisGridLayout) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisGridLayout) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisGridLayout) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisGridLayout) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisGridLayout) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisGridLayout) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisGridLayout) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisGridLayout) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisLabel é um Label de um form.
type aylisLabel struct{ aylis.Component }

func (c aylisLabel) Text() string { return asText(c.Get("text")) }
func (c aylisLabel) SetText(v string) { c.Set("text", v) }

func (c aylisLabel) HAlign() TextAlign { return TextAlign(asText(c.Get("h_align"))) }
func (c aylisLabel) SetHAlign(v TextAlign) { c.Set("h_align", string(v)) }

func (c aylisLabel) VAlign() TextAlign { return TextAlign(asText(c.Get("v_align"))) }
func (c aylisLabel) SetVAlign(v TextAlign) { c.Set("v_align", string(v)) }

func (c aylisLabel) Wrap() bool { return asBool(c.Get("wrap")) }
func (c aylisLabel) SetWrap(v bool) { c.Set("wrap", v) }

func (c aylisLabel) Trimming() Trimming { return Trimming(asText(c.Get("trimming"))) }
func (c aylisLabel) SetTrimming(v Trimming) { c.Set("trimming", string(v)) }

func (c aylisLabel) Bold() bool { return asBool(c.Get("bold")) }
func (c aylisLabel) SetBold(v bool) { c.Set("bold", v) }

func (c aylisLabel) Color() aylis.Color { return aylis.ColorFromJSON(c.Get("color")) }
func (c aylisLabel) SetColor(v aylis.Color) { c.Set("color", v.JSON()) }

func (c aylisLabel) Highlight() string { return asText(c.Get("highlight")) }
func (c aylisLabel) SetHighlight(v string) { c.Set("highlight", v) }

func (c aylisLabel) LetterSpacing() float64 { return asNumber(c.Get("letter_spacing")) }
func (c aylisLabel) SetLetterSpacing(v float64) { c.Set("letter_spacing", v) }

func (c aylisLabel) TextCase() TextCase { return TextCase(asText(c.Get("text_case"))) }
func (c aylisLabel) SetTextCase(v TextCase) { c.Set("text_case", string(v)) }

func (c aylisLabel) Markup() bool { return asBool(c.Get("markup")) }
func (c aylisLabel) SetMarkup(v bool) { c.Set("markup", v) }

func (c aylisLabel) Outline() aylis.Color { return aylis.ColorFromJSON(c.Get("outline")) }
func (c aylisLabel) SetOutline(v aylis.Color) { c.Set("outline", v.JSON()) }

func (c aylisLabel) OutlineWidth() float64 { return asNumber(c.Get("outline_width")) }
func (c aylisLabel) SetOutlineWidth(v float64) { c.Set("outline_width", v) }

func (c aylisLabel) LineHeight() float64 { return asNumber(c.Get("line_height")) }
func (c aylisLabel) SetLineHeight(v float64) { c.Set("line_height", v) }

func (c aylisLabel) TextShadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("text_shadow")) }
func (c aylisLabel) SetTextShadow(v *aylis.Shadow) { c.Set("text_shadow", v.JSON()) }

func (c aylisLabel) MaxLines() int64 { return int64(asNumber(c.Get("max_lines"))) }
func (c aylisLabel) SetMaxLines(v int64) { c.Set("max_lines", v) }

func (c aylisLabel) FocusControl() *aylis.Component { return c.GetRef("focus_control") }
func (c aylisLabel) SetFocusControl(v aylis.Ref) { c.SetRef("focus_control", v) }

func (c aylisLabel) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisLabel) SetLeft(v float64) { c.Set("left", v) }

func (c aylisLabel) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisLabel) SetTop(v float64) { c.Set("top", v) }

func (c aylisLabel) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisLabel) SetWidth(v float64) { c.Set("width", v) }

func (c aylisLabel) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisLabel) SetHeight(v float64) { c.Set("height", v) }

func (c aylisLabel) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisLabel) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisLabel) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisLabel) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisLabel) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisLabel) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisLabel) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisLabel) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisLabel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLabel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLabel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLabel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLabel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLabel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLabel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLabel) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisLabel) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisLabel) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisLabel) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisLabel) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisLabel) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisLabel) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisLabel) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisLabel) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisLabel) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisLabel) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisLabel) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisLabel) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisLabel) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisLabel) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisLabel) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisLabel) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisLabel) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisLabel) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisLabel) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisLabel) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisLabel) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisLabel) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisLabel) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisLabel) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisLabel) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisLabel) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisLabel) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisLabel) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisLabel) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisLabel) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisLabel) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisLabel) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisLabel) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisLabel) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisLabel) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisLabel) SetScale(v float64) { c.Set("scale", v) }

func (c aylisLabel) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisLabel) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisLabel) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisLabel) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisLabel) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisLabel) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisLabel) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisLabel) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisLabel) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisLabel) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisLabel) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisLabel) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisLabel) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisLabel) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisLabel) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisLabel) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisLabel) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisLabel) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisLabel) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisLabel) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisLabel) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisLabel) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisLabel) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisLabel) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisLabel) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisLabel) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisLabel) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisLabel) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisLabel) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisLabel) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisLabel) Style() LabelStyle { return LabelStyle(asText(c.Get("style"))) }
func (c aylisLabel) SetStyle(v LabelStyle) { c.Set("style", string(v)) }

func (c aylisLabel) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisLabel) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisLabel) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisLabel) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisLabel) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisLabel) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisLabel) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisLabel) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisLabel) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisLabel) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisLabel) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisLabel) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisLabel) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisLabel) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisLabel) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisLabel) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisLabel) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisLabel) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisLabel) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisLabel) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisLabel) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisLabel) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisLabel) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisLabel) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisLabel) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisLabel) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisLabel) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisLabel) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisLabel) Hint() string { return asText(c.Get("hint")) }
func (c aylisLabel) SetHint(v string) { c.Set("hint", v) }

func (c aylisLabel) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisLabel) SetTag(v int64) { c.Set("tag", v) }

func (c aylisLabel) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisLabel) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisLabel) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisLabel) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisLabel) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisLabel) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisLabel) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisLabel) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisLabel) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisLabel) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisLabel) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisLabel) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisLabel) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisLabel) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisLabel) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisLabel) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisLabel) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisLabel) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisLabel) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisLabel) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisLabel) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisLabel) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisLabel) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisLabel) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisLabel) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisLabel) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisLabel) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisLabel) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisLabel) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisLabel) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisLabel) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisLabel) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisLabel) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisLabel) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisLabel) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisLabel) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisLabel) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisLabel) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisLabel) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisLabel) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisLabel) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisLabel) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisLabel) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisLabel) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisLabel) MinContentWidth() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "min_content_width", "args": []any{}})) }
func (c aylisLabel) MaxContentWidth() float64 { return asNumber(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "max_content_width", "args": []any{}})) }

// aylisLayout é um Layout de um form.
type aylisLayout struct{ aylis.Component }

func (c aylisLayout) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisLayout) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisLayout) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisLayout) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisLayout) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisLayout) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisLayout) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisLayout) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisLayout) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisLayout) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisLayout) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisLayout) SetGap(v float64) { c.Set("gap", v) }

func (c aylisLayout) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisLayout) SetLeft(v float64) { c.Set("left", v) }

func (c aylisLayout) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisLayout) SetTop(v float64) { c.Set("top", v) }

func (c aylisLayout) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisLayout) SetWidth(v float64) { c.Set("width", v) }

func (c aylisLayout) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisLayout) SetHeight(v float64) { c.Set("height", v) }

func (c aylisLayout) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisLayout) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisLayout) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisLayout) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisLayout) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisLayout) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisLayout) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisLayout) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisLayout) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisLayout) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisLayout) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisLayout) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisLayout) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisLayout) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisLayout) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisLayout) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisLayout) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisLayout) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisLayout) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisLayout) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisLayout) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisLayout) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisLayout) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisLayout) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisLayout) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisLayout) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisLayout) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisLayout) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisLayout) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisLayout) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisLayout) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisLayout) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisLayout) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisLayout) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisLayout) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisLayout) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisLayout) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisLayout) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisLayout) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisLayout) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisLayout) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisLayout) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisLayout) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisLayout) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisLayout) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisLayout) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisLayout) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisLayout) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisLayout) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisLayout) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisLayout) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisLayout) SetScale(v float64) { c.Set("scale", v) }

func (c aylisLayout) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisLayout) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisLayout) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisLayout) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisLayout) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisLayout) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisLayout) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisLayout) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisLayout) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisLayout) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisLayout) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisLayout) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisLayout) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisLayout) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisLayout) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisLayout) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisLayout) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisLayout) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisLayout) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisLayout) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisLayout) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisLayout) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisLayout) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisLayout) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisLayout) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisLayout) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisLayout) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisLayout) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisLayout) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisLayout) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisLayout) Style() LayoutStyle { return LayoutStyle(asText(c.Get("style"))) }
func (c aylisLayout) SetStyle(v LayoutStyle) { c.Set("style", string(v)) }

func (c aylisLayout) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisLayout) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisLayout) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisLayout) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisLayout) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisLayout) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisLayout) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisLayout) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisLayout) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisLayout) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisLayout) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisLayout) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisLayout) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisLayout) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisLayout) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisLayout) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisLayout) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisLayout) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisLayout) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisLayout) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisLayout) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisLayout) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisLayout) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisLayout) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisLayout) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisLayout) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisLayout) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisLayout) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisLayout) Hint() string { return asText(c.Get("hint")) }
func (c aylisLayout) SetHint(v string) { c.Set("hint", v) }

func (c aylisLayout) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisLayout) SetTag(v int64) { c.Set("tag", v) }

func (c aylisLayout) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisLayout) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisLayout) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisLayout) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisLayout) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisLayout) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisLayout) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisLayout) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisLayout) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisLayout) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisLayout) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisLayout) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisLayout) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisLayout) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisLayout) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisLayout) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisLayout) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisLayout) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisLayout) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisLayout) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisLayout) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisLayout) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisLayout) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisLayout) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisLayout) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisLayout) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisLayout) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisLayout) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisLayout) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisLayout) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisLayout) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisLayout) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisLayout) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisLayout) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisLayout) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisLayout) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisLayout) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisLayout) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisLayout) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisLayout) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisLayout) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisLayout) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisLayout) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisLayout) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisListView é um ListView de um form.
type aylisListView struct{ aylis.Component }

func (c aylisListView) Items() []ListViewItem { return recordsListViewItem(c.Get("items")) }
func (c aylisListView) SetItems(v []ListViewItem) { c.Set("items", recordsTextListViewItem(v)) }

func (c aylisListView) ItemIndex() int64 { return int64(asNumber(c.Get("item_index"))) }
func (c aylisListView) SetItemIndex(v int64) { c.Set("item_index", v) }

func (c aylisListView) ItemHeight() float64 { return asNumber(c.Get("item_height")) }
func (c aylisListView) SetItemHeight(v float64) { c.Set("item_height", v) }

func (c aylisListView) Accessory() bool { return asBool(c.Get("accessory")) }
func (c aylisListView) SetAccessory(v bool) { c.Set("accessory", v) }

func (c aylisListView) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisListView) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisListView) OnItemClick() string { return asText(c.Get("on_item_click")) }
func (c aylisListView) SetOnItemClick(v string) { c.Set("on_item_click", v) }

func (c aylisListView) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisListView) SetLeft(v float64) { c.Set("left", v) }

func (c aylisListView) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisListView) SetTop(v float64) { c.Set("top", v) }

func (c aylisListView) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisListView) SetWidth(v float64) { c.Set("width", v) }

func (c aylisListView) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisListView) SetHeight(v float64) { c.Set("height", v) }

func (c aylisListView) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisListView) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisListView) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisListView) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisListView) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisListView) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisListView) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisListView) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisListView) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisListView) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisListView) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisListView) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisListView) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisListView) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisListView) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisListView) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisListView) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisListView) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisListView) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisListView) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisListView) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisListView) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisListView) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisListView) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisListView) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisListView) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisListView) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisListView) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisListView) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisListView) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisListView) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisListView) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisListView) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisListView) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisListView) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisListView) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisListView) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisListView) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisListView) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisListView) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisListView) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisListView) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisListView) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisListView) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisListView) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisListView) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisListView) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisListView) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisListView) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisListView) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisListView) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisListView) SetScale(v float64) { c.Set("scale", v) }

func (c aylisListView) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisListView) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisListView) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisListView) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisListView) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisListView) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisListView) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisListView) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisListView) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisListView) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisListView) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisListView) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisListView) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisListView) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisListView) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisListView) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisListView) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisListView) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisListView) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisListView) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisListView) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisListView) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisListView) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisListView) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisListView) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisListView) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisListView) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisListView) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisListView) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisListView) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisListView) Style() ListViewStyle { return ListViewStyle(asText(c.Get("style"))) }
func (c aylisListView) SetStyle(v ListViewStyle) { c.Set("style", string(v)) }

func (c aylisListView) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisListView) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisListView) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisListView) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisListView) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisListView) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisListView) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisListView) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisListView) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisListView) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisListView) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisListView) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisListView) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisListView) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisListView) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisListView) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisListView) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisListView) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisListView) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisListView) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisListView) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisListView) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisListView) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisListView) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisListView) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisListView) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisListView) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisListView) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisListView) Hint() string { return asText(c.Get("hint")) }
func (c aylisListView) SetHint(v string) { c.Set("hint", v) }

func (c aylisListView) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisListView) SetTag(v int64) { c.Set("tag", v) }

func (c aylisListView) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisListView) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisListView) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisListView) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisListView) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisListView) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisListView) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisListView) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisListView) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisListView) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisListView) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisListView) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisListView) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisListView) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisListView) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisListView) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisListView) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisListView) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisListView) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisListView) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisListView) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisListView) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisListView) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisListView) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisListView) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisListView) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisListView) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisListView) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisListView) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisListView) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisListView) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisListView) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisListView) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisListView) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisListView) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisListView) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisListView) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisListView) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisListView) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisListView) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisListView) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisListView) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisListView) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisListView) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisListView) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisPanel é um Panel de um form.
type aylisPanel struct{ aylis.Component }

func (c aylisPanel) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisPanel) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisPanel) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisPanel) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisPanel) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisPanel) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisPanel) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisPanel) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisPanel) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisPanel) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisPanel) Gap() float64 { return asNumber(c.Get("gap")) }
func (c aylisPanel) SetGap(v float64) { c.Set("gap", v) }

func (c aylisPanel) HoverFill() aylis.Color { return aylis.ColorFromJSON(c.Get("hover_fill")) }
func (c aylisPanel) SetHoverFill(v aylis.Color) { c.Set("hover_fill", v.JSON()) }

func (c aylisPanel) PressedFill() aylis.Color { return aylis.ColorFromJSON(c.Get("pressed_fill")) }
func (c aylisPanel) SetPressedFill(v aylis.Color) { c.Set("pressed_fill", v.JSON()) }

func (c aylisPanel) Interactive() bool { return asBool(c.Get("interactive")) }
func (c aylisPanel) SetInteractive(v bool) { c.Set("interactive", v) }

func (c aylisPanel) SelectedFill() aylis.Color { return aylis.ColorFromJSON(c.Get("selected_fill")) }
func (c aylisPanel) SetSelectedFill(v aylis.Color) { c.Set("selected_fill", v.JSON()) }

func (c aylisPanel) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisPanel) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisPanel) Fill2() aylis.Color { return aylis.ColorFromJSON(c.Get("fill2")) }
func (c aylisPanel) SetFill2(v aylis.Color) { c.Set("fill2", v.JSON()) }

func (c aylisPanel) FillMid() aylis.Color { return aylis.ColorFromJSON(c.Get("fill_mid")) }
func (c aylisPanel) SetFillMid(v aylis.Color) { c.Set("fill_mid", v.JSON()) }

func (c aylisPanel) FillMidAt() float64 { return asNumber(c.Get("fill_mid_at")) }
func (c aylisPanel) SetFillMidAt(v float64) { c.Set("fill_mid_at", v) }

func (c aylisPanel) GradientAngle() float64 { return asNumber(c.Get("gradient_angle")) }
func (c aylisPanel) SetGradientAngle(v float64) { c.Set("gradient_angle", v) }

func (c aylisPanel) GradientRadial() bool { return asBool(c.Get("gradient_radial")) }
func (c aylisPanel) SetGradientRadial(v bool) { c.Set("gradient_radial", v) }

func (c aylisPanel) Stroke() aylis.Color { return aylis.ColorFromJSON(c.Get("stroke")) }
func (c aylisPanel) SetStroke(v aylis.Color) { c.Set("stroke", v.JSON()) }

func (c aylisPanel) StrokeWidth() float64 { return asNumber(c.Get("stroke_width")) }
func (c aylisPanel) SetStrokeWidth(v float64) { c.Set("stroke_width", v) }

func (c aylisPanel) StrokeDash() []float64 { return asNumbers(c.Get("stroke_dash")) }
func (c aylisPanel) SetStrokeDash(v []float64) { c.Set("stroke_dash", numbersText(v)) }

func (c aylisPanel) Radius() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("radius")) }
func (c aylisPanel) SetRadius(v aylis.Bounds) { c.Set("radius", v.JSON()) }

func (c aylisPanel) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisPanel) SetLeft(v float64) { c.Set("left", v) }

func (c aylisPanel) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisPanel) SetTop(v float64) { c.Set("top", v) }

func (c aylisPanel) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisPanel) SetWidth(v float64) { c.Set("width", v) }

func (c aylisPanel) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisPanel) SetHeight(v float64) { c.Set("height", v) }

func (c aylisPanel) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisPanel) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisPanel) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisPanel) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisPanel) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisPanel) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisPanel) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisPanel) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisPanel) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisPanel) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisPanel) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisPanel) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisPanel) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisPanel) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisPanel) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisPanel) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisPanel) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisPanel) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisPanel) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisPanel) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisPanel) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisPanel) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisPanel) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisPanel) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisPanel) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisPanel) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisPanel) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisPanel) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisPanel) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisPanel) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisPanel) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisPanel) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisPanel) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisPanel) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisPanel) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisPanel) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisPanel) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisPanel) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisPanel) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisPanel) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisPanel) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisPanel) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisPanel) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisPanel) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisPanel) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisPanel) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisPanel) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisPanel) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisPanel) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisPanel) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisPanel) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisPanel) SetScale(v float64) { c.Set("scale", v) }

func (c aylisPanel) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisPanel) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisPanel) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisPanel) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisPanel) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisPanel) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisPanel) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisPanel) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisPanel) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisPanel) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisPanel) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisPanel) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisPanel) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisPanel) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisPanel) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisPanel) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisPanel) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisPanel) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisPanel) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisPanel) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisPanel) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisPanel) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisPanel) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisPanel) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisPanel) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisPanel) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisPanel) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisPanel) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisPanel) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisPanel) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisPanel) Style() PanelStyle { return PanelStyle(asText(c.Get("style"))) }
func (c aylisPanel) SetStyle(v PanelStyle) { c.Set("style", string(v)) }

func (c aylisPanel) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisPanel) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisPanel) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisPanel) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisPanel) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisPanel) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisPanel) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisPanel) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisPanel) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisPanel) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisPanel) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisPanel) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisPanel) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisPanel) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisPanel) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisPanel) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisPanel) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisPanel) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisPanel) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisPanel) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisPanel) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisPanel) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisPanel) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisPanel) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisPanel) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisPanel) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisPanel) Hint() string { return asText(c.Get("hint")) }
func (c aylisPanel) SetHint(v string) { c.Set("hint", v) }

func (c aylisPanel) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisPanel) SetTag(v int64) { c.Set("tag", v) }

func (c aylisPanel) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisPanel) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisPanel) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisPanel) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisPanel) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisPanel) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisPanel) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisPanel) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisPanel) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisPanel) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisPanel) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisPanel) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisPanel) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisPanel) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisPanel) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisPanel) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisPanel) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisPanel) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisPanel) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisPanel) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisPanel) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisPanel) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisPanel) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisPanel) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisPanel) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisPanel) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisPanel) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisPanel) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisPanel) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisPanel) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisPanel) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisPanel) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisPanel) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisPanel) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisPanel) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisPanel) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisPanel) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisPanel) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisPanel) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisPanel) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisPanel) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisPanel) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisPanel) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisPanel) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisTabControl é um TabControl de um form.
type aylisTabControl struct{ aylis.Component }

func (c aylisTabControl) TabIndex() int64 { return int64(asNumber(c.Get("tab_index"))) }
func (c aylisTabControl) SetTabIndex(v int64) { c.Set("tab_index", v) }

func (c aylisTabControl) ShowTabs() bool { return asBool(c.Get("show_tabs")) }
func (c aylisTabControl) SetShowTabs(v bool) { c.Set("show_tabs", v) }

func (c aylisTabControl) Transition() Transition { return Transition(asText(c.Get("transition"))) }
func (c aylisTabControl) SetTransition(v Transition) { c.Set("transition", string(v)) }

func (c aylisTabControl) Swipe() bool { return asBool(c.Get("swipe")) }
func (c aylisTabControl) SetSwipe(v bool) { c.Set("swipe", v) }

func (c aylisTabControl) TabPosition() TabPosition { return TabPosition(asText(c.Get("tab_position"))) }
func (c aylisTabControl) SetTabPosition(v TabPosition) { c.Set("tab_position", string(v)) }

func (c aylisTabControl) TabWidth() float64 { return asNumber(c.Get("tab_width")) }
func (c aylisTabControl) SetTabWidth(v float64) { c.Set("tab_width", v) }

func (c aylisTabControl) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisTabControl) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisTabControl) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTabControl) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTabControl) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTabControl) SetTop(v float64) { c.Set("top", v) }

func (c aylisTabControl) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTabControl) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTabControl) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTabControl) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTabControl) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisTabControl) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisTabControl) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisTabControl) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisTabControl) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisTabControl) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisTabControl) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisTabControl) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisTabControl) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTabControl) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTabControl) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTabControl) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTabControl) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTabControl) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTabControl) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTabControl) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTabControl) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisTabControl) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisTabControl) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisTabControl) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisTabControl) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTabControl) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTabControl) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisTabControl) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisTabControl) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTabControl) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTabControl) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTabControl) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTabControl) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTabControl) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTabControl) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTabControl) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTabControl) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTabControl) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTabControl) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisTabControl) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisTabControl) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTabControl) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTabControl) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTabControl) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTabControl) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisTabControl) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisTabControl) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTabControl) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTabControl) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisTabControl) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisTabControl) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTabControl) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTabControl) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTabControl) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTabControl) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisTabControl) SetScale(v float64) { c.Set("scale", v) }

func (c aylisTabControl) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTabControl) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTabControl) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTabControl) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTabControl) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTabControl) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTabControl) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTabControl) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTabControl) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTabControl) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTabControl) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTabControl) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTabControl) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTabControl) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTabControl) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisTabControl) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisTabControl) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTabControl) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTabControl) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTabControl) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTabControl) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisTabControl) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisTabControl) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisTabControl) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisTabControl) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisTabControl) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisTabControl) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisTabControl) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisTabControl) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisTabControl) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisTabControl) Style() TabControlStyle { return TabControlStyle(asText(c.Get("style"))) }
func (c aylisTabControl) SetStyle(v TabControlStyle) { c.Set("style", string(v)) }

func (c aylisTabControl) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisTabControl) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisTabControl) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisTabControl) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisTabControl) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisTabControl) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisTabControl) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisTabControl) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisTabControl) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisTabControl) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisTabControl) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisTabControl) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisTabControl) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisTabControl) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisTabControl) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisTabControl) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisTabControl) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTabControl) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTabControl) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTabControl) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTabControl) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisTabControl) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisTabControl) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTabControl) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTabControl) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTabControl) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTabControl) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTabControl) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTabControl) Hint() string { return asText(c.Get("hint")) }
func (c aylisTabControl) SetHint(v string) { c.Set("hint", v) }

func (c aylisTabControl) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTabControl) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTabControl) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisTabControl) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisTabControl) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisTabControl) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisTabControl) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisTabControl) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisTabControl) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisTabControl) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisTabControl) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisTabControl) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisTabControl) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisTabControl) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisTabControl) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisTabControl) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisTabControl) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisTabControl) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisTabControl) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisTabControl) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisTabControl) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisTabControl) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisTabControl) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisTabControl) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisTabControl) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisTabControl) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisTabControl) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisTabControl) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisTabControl) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisTabControl) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisTabControl) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisTabControl) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisTabControl) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisTabControl) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisTabControl) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisTabControl) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisTabControl) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisTabControl) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisTabControl) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisTabControl) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisTabControl) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisTabControl) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisTabControl) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisTabControl) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisTabControl) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisTabControl) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisTabControl) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisTabItem é um TabItem de um form.
type aylisTabItem struct{ aylis.Component }

func (c aylisTabItem) Caption() string { return asText(c.Get("caption")) }
func (c aylisTabItem) SetCaption(v string) { c.Set("caption", v) }

func (c aylisTabItem) OnShow() string { return asText(c.Get("on_show")) }
func (c aylisTabItem) SetOnShow(v string) { c.Set("on_show", v) }

func (c aylisTabItem) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTabItem) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTabItem) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTabItem) SetTop(v float64) { c.Set("top", v) }

func (c aylisTabItem) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTabItem) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTabItem) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTabItem) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTabItem) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisTabItem) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisTabItem) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisTabItem) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisTabItem) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisTabItem) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisTabItem) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisTabItem) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisTabItem) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTabItem) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTabItem) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTabItem) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTabItem) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTabItem) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTabItem) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTabItem) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTabItem) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisTabItem) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisTabItem) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisTabItem) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisTabItem) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTabItem) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTabItem) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisTabItem) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisTabItem) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTabItem) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTabItem) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTabItem) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTabItem) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTabItem) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTabItem) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTabItem) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTabItem) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTabItem) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTabItem) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisTabItem) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisTabItem) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTabItem) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTabItem) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTabItem) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTabItem) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisTabItem) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisTabItem) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTabItem) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTabItem) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisTabItem) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisTabItem) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTabItem) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTabItem) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTabItem) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTabItem) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisTabItem) SetScale(v float64) { c.Set("scale", v) }

func (c aylisTabItem) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTabItem) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTabItem) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTabItem) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTabItem) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTabItem) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTabItem) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTabItem) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTabItem) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTabItem) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTabItem) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTabItem) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTabItem) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTabItem) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTabItem) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisTabItem) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisTabItem) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTabItem) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTabItem) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTabItem) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTabItem) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisTabItem) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisTabItem) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisTabItem) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisTabItem) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisTabItem) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisTabItem) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisTabItem) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisTabItem) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisTabItem) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisTabItem) Style() TabItemStyle { return TabItemStyle(asText(c.Get("style"))) }
func (c aylisTabItem) SetStyle(v TabItemStyle) { c.Set("style", string(v)) }

func (c aylisTabItem) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisTabItem) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisTabItem) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisTabItem) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisTabItem) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisTabItem) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisTabItem) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisTabItem) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisTabItem) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisTabItem) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisTabItem) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisTabItem) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisTabItem) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisTabItem) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisTabItem) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisTabItem) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisTabItem) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTabItem) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTabItem) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTabItem) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTabItem) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisTabItem) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisTabItem) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTabItem) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTabItem) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTabItem) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTabItem) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTabItem) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTabItem) Hint() string { return asText(c.Get("hint")) }
func (c aylisTabItem) SetHint(v string) { c.Set("hint", v) }

func (c aylisTabItem) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTabItem) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTabItem) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisTabItem) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisTabItem) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisTabItem) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisTabItem) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisTabItem) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisTabItem) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisTabItem) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisTabItem) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisTabItem) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisTabItem) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisTabItem) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisTabItem) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisTabItem) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisTabItem) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisTabItem) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisTabItem) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisTabItem) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisTabItem) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisTabItem) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisTabItem) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisTabItem) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisTabItem) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisTabItem) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisTabItem) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisTabItem) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisTabItem) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisTabItem) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisTabItem) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisTabItem) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisTabItem) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisTabItem) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisTabItem) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisTabItem) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisTabItem) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisTabItem) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisTabItem) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisTabItem) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisTabItem) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisTabItem) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisTabItem) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisTabItem) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisTabItem) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisTabItem) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// frmCalculadoraComponents: o form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmCalculadoraComponents struct {
	aylisForm
	pnlLateral aylisPanel
	tabLateral aylisTabControl
	tabHistorico aylisTabItem
	btnLimparHistorico aylisButton
	lvHistorico aylisListView
	tabMemoria aylisTabItem
	lvMemoria aylisListView
	lytCorpo aylisLayout
	lytTopo aylisLayout
	btnHistorico aylisButton
	lblTitulo aylisLabel
	lblExpressao aylisLabel
	lblVisor aylisLabel
	grdMemoria aylisGridLayout
	btnMC aylisButton
	btnMR aylisButton
	btnMMais aylisButton
	btnMMenos aylisButton
	btnMS aylisButton
	grdTeclado aylisGridLayout
	btnPorcento aylisButton
	btnCE aylisButton
	btnC aylisButton
	btnApagar aylisButton
	btnInverso aylisButton
	btnQuadrado aylisButton
	btnRaiz aylisButton
	btnDividir aylisButton
	btn7 aylisButton
	btn8 aylisButton
	btn9 aylisButton
	btnMultiplicar aylisButton
	btn4 aylisButton
	btn5 aylisButton
	btn6 aylisButton
	btnSubtrair aylisButton
	btn1 aylisButton
	btn2 aylisButton
	btn3 aylisButton
	btnSomar aylisButton
	btnSinal aylisButton
	btn0 aylisButton
	btnVirgula aylisButton
	btnIgual aylisButton
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmCalculadoraComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "pnlLateral", "Panel"); err != nil {
		return err
	} else {
		f.pnlLateral = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "tabLateral", "TabControl"); err != nil {
		return err
	} else {
		f.tabLateral = aylisTabControl{c}
	}
	if c, err := f.Resolve(list, "tabHistorico", "TabItem"); err != nil {
		return err
	} else {
		f.tabHistorico = aylisTabItem{c}
	}
	if c, err := f.Resolve(list, "btnLimparHistorico", "Button"); err != nil {
		return err
	} else {
		f.btnLimparHistorico = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lvHistorico", "ListView"); err != nil {
		return err
	} else {
		f.lvHistorico = aylisListView{c}
	}
	if c, err := f.Resolve(list, "tabMemoria", "TabItem"); err != nil {
		return err
	} else {
		f.tabMemoria = aylisTabItem{c}
	}
	if c, err := f.Resolve(list, "lvMemoria", "ListView"); err != nil {
		return err
	} else {
		f.lvMemoria = aylisListView{c}
	}
	if c, err := f.Resolve(list, "lytCorpo", "Layout"); err != nil {
		return err
	} else {
		f.lytCorpo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lytTopo", "Layout"); err != nil {
		return err
	} else {
		f.lytTopo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnHistorico", "Button"); err != nil {
		return err
	} else {
		f.btnHistorico = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lblTitulo", "Label"); err != nil {
		return err
	} else {
		f.lblTitulo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblExpressao", "Label"); err != nil {
		return err
	} else {
		f.lblExpressao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblVisor", "Label"); err != nil {
		return err
	} else {
		f.lblVisor = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "grdMemoria", "GridLayout"); err != nil {
		return err
	} else {
		f.grdMemoria = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnMC", "Button"); err != nil {
		return err
	} else {
		f.btnMC = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMR", "Button"); err != nil {
		return err
	} else {
		f.btnMR = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMMais", "Button"); err != nil {
		return err
	} else {
		f.btnMMais = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMMenos", "Button"); err != nil {
		return err
	} else {
		f.btnMMenos = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMS", "Button"); err != nil {
		return err
	} else {
		f.btnMS = aylisButton{c}
	}
	if c, err := f.Resolve(list, "grdTeclado", "GridLayout"); err != nil {
		return err
	} else {
		f.grdTeclado = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnPorcento", "Button"); err != nil {
		return err
	} else {
		f.btnPorcento = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnCE", "Button"); err != nil {
		return err
	} else {
		f.btnCE = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnC", "Button"); err != nil {
		return err
	} else {
		f.btnC = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnApagar", "Button"); err != nil {
		return err
	} else {
		f.btnApagar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnInverso", "Button"); err != nil {
		return err
	} else {
		f.btnInverso = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnQuadrado", "Button"); err != nil {
		return err
	} else {
		f.btnQuadrado = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRaiz", "Button"); err != nil {
		return err
	} else {
		f.btnRaiz = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnDividir", "Button"); err != nil {
		return err
	} else {
		f.btnDividir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn7", "Button"); err != nil {
		return err
	} else {
		f.btn7 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn8", "Button"); err != nil {
		return err
	} else {
		f.btn8 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn9", "Button"); err != nil {
		return err
	} else {
		f.btn9 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnMultiplicar", "Button"); err != nil {
		return err
	} else {
		f.btnMultiplicar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn4", "Button"); err != nil {
		return err
	} else {
		f.btn4 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn5", "Button"); err != nil {
		return err
	} else {
		f.btn5 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn6", "Button"); err != nil {
		return err
	} else {
		f.btn6 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSubtrair", "Button"); err != nil {
		return err
	} else {
		f.btnSubtrair = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn1", "Button"); err != nil {
		return err
	} else {
		f.btn1 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn2", "Button"); err != nil {
		return err
	} else {
		f.btn2 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn3", "Button"); err != nil {
		return err
	} else {
		f.btn3 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSomar", "Button"); err != nil {
		return err
	} else {
		f.btnSomar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnSinal", "Button"); err != nil {
		return err
	} else {
		f.btnSinal = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btn0", "Button"); err != nil {
		return err
	} else {
		f.btn0 = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnVirgula", "Button"); err != nil {
		return err
	} else {
		f.btnVirgula = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnIgual", "Button"); err != nil {
		return err
	} else {
		f.btnIgual = aylisButton{c}
	}
	return nil
}

// frmCalculadora é a janela de frmCalculadora cujo evento está rodando (a primeira, fora dos handlers).
var frmCalculadora *frmCalculadoraComponents

func init() {
	aylis.RegisterFormClass("frmCalculadora", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmCalculadoraComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmCalculadora = form.(*frmCalculadoraComponents) })
}

// FindButton é o Button deste form com o nome.
func (f *frmCalculadoraComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnLimparHistorico":
		return f.btnLimparHistorico
	case "btnHistorico":
		return f.btnHistorico
	case "btnMC":
		return f.btnMC
	case "btnMR":
		return f.btnMR
	case "btnMMais":
		return f.btnMMais
	case "btnMMenos":
		return f.btnMMenos
	case "btnMS":
		return f.btnMS
	case "btnPorcento":
		return f.btnPorcento
	case "btnCE":
		return f.btnCE
	case "btnC":
		return f.btnC
	case "btnApagar":
		return f.btnApagar
	case "btnInverso":
		return f.btnInverso
	case "btnQuadrado":
		return f.btnQuadrado
	case "btnRaiz":
		return f.btnRaiz
	case "btnDividir":
		return f.btnDividir
	case "btn7":
		return f.btn7
	case "btn8":
		return f.btn8
	case "btn9":
		return f.btn9
	case "btnMultiplicar":
		return f.btnMultiplicar
	case "btn4":
		return f.btn4
	case "btn5":
		return f.btn5
	case "btn6":
		return f.btn6
	case "btnSubtrair":
		return f.btnSubtrair
	case "btn1":
		return f.btn1
	case "btn2":
		return f.btn2
	case "btn3":
		return f.btn3
	case "btnSomar":
		return f.btnSomar
	case "btnSinal":
		return f.btnSinal
	case "btn0":
		return f.btn0
	case "btnVirgula":
		return f.btnVirgula
	case "btnIgual":
		return f.btnIgual
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindGridLayout é o GridLayout deste form com o nome.
func (f *frmCalculadoraComponents) FindGridLayout(name string) aylisGridLayout {
	switch name {
	case "grdMemoria":
		return f.grdMemoria
	case "grdTeclado":
		return f.grdTeclado
	}
	return aylisGridLayout{}
}

// SenderAsGridLayout é o GridLayout que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsGridLayout(ev aylis.Event) aylisGridLayout { return f.FindGridLayout(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome.
func (f *frmCalculadoraComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblTitulo":
		return f.lblTitulo
	case "lblExpressao":
		return f.lblExpressao
	case "lblVisor":
		return f.lblVisor
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome.
func (f *frmCalculadoraComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytCorpo":
		return f.lytCorpo
	case "lytTopo":
		return f.lytTopo
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindListView é o ListView deste form com o nome.
func (f *frmCalculadoraComponents) FindListView(name string) aylisListView {
	switch name {
	case "lvHistorico":
		return f.lvHistorico
	case "lvMemoria":
		return f.lvMemoria
	}
	return aylisListView{}
}

// SenderAsListView é o ListView que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsListView(ev aylis.Event) aylisListView { return f.FindListView(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome.
func (f *frmCalculadoraComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlLateral":
		return f.pnlLateral
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindTabControl é o TabControl deste form com o nome.
func (f *frmCalculadoraComponents) FindTabControl(name string) aylisTabControl {
	switch name {
	case "tabLateral":
		return f.tabLateral
	}
	return aylisTabControl{}
}

// SenderAsTabControl é o TabControl que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsTabControl(ev aylis.Event) aylisTabControl { return f.FindTabControl(ev.Sender.Name) }

// FindTabItem é o TabItem deste form com o nome.
func (f *frmCalculadoraComponents) FindTabItem(name string) aylisTabItem {
	switch name {
	case "tabHistorico":
		return f.tabHistorico
	case "tabMemoria":
		return f.tabMemoria
	}
	return aylisTabItem{}
}

// SenderAsTabItem é o TabItem que disparou o evento.
func (f *frmCalculadoraComponents) SenderAsTabItem(ev aylis.Event) aylisTabItem { return f.FindTabItem(ev.Sender.Name) }
