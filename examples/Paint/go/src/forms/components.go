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

// CheckBoxStyle são os valores de um enum do acesso tipado.
type CheckBoxStyle string

const (
	CheckBoxStyleDefault CheckBoxStyle = ""
	CheckBoxStyleRound CheckBoxStyle = "round"
)

func enumListCheckBoxStyle(v any) []CheckBoxStyle {
	var out []CheckBoxStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CheckBoxStyle(asText(x)))
		}
	}
	return out
}

func enumNamesCheckBoxStyle(vs []CheckBoxStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// CircleStyle são os valores de um enum do acesso tipado.
type CircleStyle string

const (
	CircleStyleDefault CircleStyle = ""
)

func enumListCircleStyle(v any) []CircleStyle {
	var out []CircleStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CircleStyle(asText(x)))
		}
	}
	return out
}

func enumNamesCircleStyle(vs []CircleStyle) []string {
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

// ComboArrow são os valores de um enum do acesso tipado.
type ComboArrow string

const (
	ComboArrowChevron ComboArrow = "chevron"
	ComboArrowTriangle ComboArrow = "triangle"
)

func enumListComboArrow(v any) []ComboArrow {
	var out []ComboArrow
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ComboArrow(asText(x)))
		}
	}
	return out
}

func enumNamesComboArrow(vs []ComboArrow) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ComboBoxStyle são os valores de um enum do acesso tipado.
type ComboBoxStyle string

const (
	ComboBoxStyleDefault ComboBoxStyle = ""
)

func enumListComboBoxStyle(v any) []ComboBoxStyle {
	var out []ComboBoxStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ComboBoxStyle(asText(x)))
		}
	}
	return out
}

func enumNamesComboBoxStyle(vs []ComboBoxStyle) []string {
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

// DividerStyle são os valores de um enum do acesso tipado.
type DividerStyle string

const (
	DividerStyleDefault DividerStyle = ""
)

func enumListDividerStyle(v any) []DividerStyle {
	var out []DividerStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, DividerStyle(asText(x)))
		}
	}
	return out
}

func enumNamesDividerStyle(vs []DividerStyle) []string {
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

// MainMenuStyle são os valores de um enum do acesso tipado.
type MainMenuStyle string

const (
	MainMenuStyleDefault MainMenuStyle = ""
)

func enumListMainMenuStyle(v any) []MainMenuStyle {
	var out []MainMenuStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, MainMenuStyle(asText(x)))
		}
	}
	return out
}

func enumNamesMainMenuStyle(vs []MainMenuStyle) []string {
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

// Overscroll são os valores de um enum do acesso tipado.
type Overscroll string

const (
	OverscrollAuto Overscroll = "auto"
	OverscrollGlow Overscroll = "glow"
	OverscrollStretch Overscroll = "stretch"
	OverscrollBounce Overscroll = "bounce"
	OverscrollNone Overscroll = "none"
)

func enumListOverscroll(v any) []Overscroll {
	var out []Overscroll
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Overscroll(asText(x)))
		}
	}
	return out
}

func enumNamesOverscroll(vs []Overscroll) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// PaintCanvasStyle são os valores de um enum do acesso tipado.
type PaintCanvasStyle string

const (
	PaintCanvasStyleDefault PaintCanvasStyle = ""
)

func enumListPaintCanvasStyle(v any) []PaintCanvasStyle {
	var out []PaintCanvasStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, PaintCanvasStyle(asText(x)))
		}
	}
	return out
}

func enumNamesPaintCanvasStyle(vs []PaintCanvasStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// PaintTool são os valores de um enum do acesso tipado.
type PaintTool string

const (
	PaintToolPencil PaintTool = "pencil"
	PaintToolBrush PaintTool = "brush"
	PaintToolEraser PaintTool = "eraser"
	PaintToolLine PaintTool = "line"
	PaintToolRectangle PaintTool = "rectangle"
	PaintToolEllipse PaintTool = "ellipse"
	PaintToolFill PaintTool = "fill"
	PaintToolPicker PaintTool = "picker"
	PaintToolSelect PaintTool = "select"
)

func enumListPaintTool(v any) []PaintTool {
	var out []PaintTool
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, PaintTool(asText(x)))
		}
	}
	return out
}

func enumNamesPaintTool(vs []PaintTool) []string {
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

// ScrollBoxStyle são os valores de um enum do acesso tipado.
type ScrollBoxStyle string

const (
	ScrollBoxStyleDefault ScrollBoxStyle = ""
	ScrollBoxStyleHorizontal ScrollBoxStyle = "horizontal"
	ScrollBoxStyleRefresh ScrollBoxStyle = "refresh"
)

func enumListScrollBoxStyle(v any) []ScrollBoxStyle {
	var out []ScrollBoxStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ScrollBoxStyle(asText(x)))
		}
	}
	return out
}

func enumNamesScrollBoxStyle(vs []ScrollBoxStyle) []string {
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

// SmoothScroll são os valores de um enum do acesso tipado.
type SmoothScroll string

const (
	SmoothScrollDefault SmoothScroll = "default"
	SmoothScrollOn SmoothScroll = "on"
	SmoothScrollOff SmoothScroll = "off"
)

func enumListSmoothScroll(v any) []SmoothScroll {
	var out []SmoothScroll
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, SmoothScroll(asText(x)))
		}
	}
	return out
}

func enumNamesSmoothScroll(vs []SmoothScroll) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// StatusBarStyle são os valores de um enum do acesso tipado.
type StatusBarStyle string

const (
	StatusBarStyleDefault StatusBarStyle = ""
)

func enumListStatusBarStyle(v any) []StatusBarStyle {
	var out []StatusBarStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, StatusBarStyle(asText(x)))
		}
	}
	return out
}

func enumNamesStatusBarStyle(vs []StatusBarStyle) []string {
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

// ImageSize é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type ImageSize struct {
	Width int64
	Height int64
}

func recordOfImageSize(v any) ImageSize {
	m, _ := v.(map[string]any)
	return ImageSize{
		Width: int64(asNumber(m["width"])),
		Height: int64(asNumber(m["height"])),
	}
}

func (r ImageSize) jsonValue() map[string]any {
	return map[string]any{
		"width": r.Width,
		"height": r.Height,
	}
}

func jsonListImageSize(items []ImageSize) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfImageSize(v any) []ImageSize {
	l, _ := v.([]any)
	out := make([]ImageSize, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfImageSize(x))
	}
	return out
}

func recordImageSize(v any) *ImageSize {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfImageSize(j)
	return &r
}

func recordsImageSize(v any) []ImageSize {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfImageSize(j)
}

func recordTextImageSize(r *ImageSize) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextImageSize(items []ImageSize) string {
	b, _ := json.Marshal(jsonListImageSize(items))
	return string(b)
}

// KeyEvent é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type KeyEvent struct {
	Key string
	Ctrl bool
	Shift bool
	Alt bool
}

func recordOfKeyEvent(v any) KeyEvent {
	m, _ := v.(map[string]any)
	return KeyEvent{
		Key: asText(m["key"]),
		Ctrl: asBool(m["ctrl"]),
		Shift: asBool(m["shift"]),
		Alt: asBool(m["alt"]),
	}
}

func (r KeyEvent) jsonValue() map[string]any {
	return map[string]any{
		"key": r.Key,
		"ctrl": r.Ctrl,
		"shift": r.Shift,
		"alt": r.Alt,
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

// aylisCheckBox é um CheckBox de um form.
type aylisCheckBox struct{ aylis.Component }

func (c aylisCheckBox) Text() string { return asText(c.Get("text")) }
func (c aylisCheckBox) SetText(v string) { c.Set("text", v) }

func (c aylisCheckBox) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisCheckBox) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisCheckBox) BoxSize() float64 { return asNumber(c.Get("box_size")) }
func (c aylisCheckBox) SetBoxSize(v float64) { c.Set("box_size", v) }

func (c aylisCheckBox) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisCheckBox) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisCheckBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisCheckBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisCheckBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisCheckBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisCheckBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisCheckBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisCheckBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisCheckBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisCheckBox) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisCheckBox) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisCheckBox) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisCheckBox) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisCheckBox) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisCheckBox) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisCheckBox) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisCheckBox) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisCheckBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisCheckBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisCheckBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisCheckBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisCheckBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisCheckBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisCheckBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisCheckBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisCheckBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisCheckBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisCheckBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisCheckBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisCheckBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisCheckBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisCheckBox) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisCheckBox) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisCheckBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisCheckBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisCheckBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisCheckBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisCheckBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisCheckBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisCheckBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisCheckBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisCheckBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisCheckBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisCheckBox) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisCheckBox) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisCheckBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisCheckBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisCheckBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisCheckBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisCheckBox) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisCheckBox) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisCheckBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisCheckBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisCheckBox) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisCheckBox) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisCheckBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisCheckBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisCheckBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisCheckBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisCheckBox) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisCheckBox) SetScale(v float64) { c.Set("scale", v) }

func (c aylisCheckBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisCheckBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisCheckBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisCheckBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisCheckBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisCheckBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisCheckBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisCheckBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisCheckBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisCheckBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisCheckBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisCheckBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisCheckBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisCheckBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisCheckBox) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisCheckBox) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisCheckBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisCheckBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisCheckBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisCheckBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisCheckBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisCheckBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisCheckBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisCheckBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisCheckBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisCheckBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisCheckBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisCheckBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisCheckBox) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisCheckBox) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisCheckBox) Style() CheckBoxStyle { return CheckBoxStyle(asText(c.Get("style"))) }
func (c aylisCheckBox) SetStyle(v CheckBoxStyle) { c.Set("style", string(v)) }

func (c aylisCheckBox) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisCheckBox) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisCheckBox) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisCheckBox) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisCheckBox) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisCheckBox) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisCheckBox) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisCheckBox) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisCheckBox) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisCheckBox) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisCheckBox) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisCheckBox) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisCheckBox) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisCheckBox) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisCheckBox) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisCheckBox) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisCheckBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisCheckBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisCheckBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisCheckBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisCheckBox) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisCheckBox) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisCheckBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisCheckBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisCheckBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisCheckBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisCheckBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisCheckBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisCheckBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisCheckBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisCheckBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisCheckBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisCheckBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisCheckBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisCheckBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisCheckBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisCheckBox) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisCheckBox) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisCheckBox) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisCheckBox) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisCheckBox) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisCheckBox) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisCheckBox) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisCheckBox) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisCheckBox) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisCheckBox) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisCheckBox) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisCheckBox) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisCheckBox) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisCheckBox) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisCheckBox) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisCheckBox) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisCheckBox) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisCheckBox) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisCheckBox) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisCheckBox) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisCheckBox) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisCheckBox) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisCheckBox) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisCheckBox) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisCheckBox) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisCheckBox) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisCheckBox) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisCheckBox) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisCheckBox) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisCheckBox) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisCheckBox) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisCheckBox) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisCheckBox) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisCheckBox) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisCheckBox) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisCheckBox) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisCheckBox) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisCheckBox) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisCheckBox) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisCheckBox) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisCheckBox) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisCircle é um Circle de um form.
type aylisCircle struct{ aylis.Component }

func (c aylisCircle) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisCircle) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisCircle) Stroke() aylis.Color { return aylis.ColorFromJSON(c.Get("stroke")) }
func (c aylisCircle) SetStroke(v aylis.Color) { c.Set("stroke", v.JSON()) }

func (c aylisCircle) StrokeWidth() float64 { return asNumber(c.Get("stroke_width")) }
func (c aylisCircle) SetStrokeWidth(v float64) { c.Set("stroke_width", v) }

func (c aylisCircle) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisCircle) SetLeft(v float64) { c.Set("left", v) }

func (c aylisCircle) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisCircle) SetTop(v float64) { c.Set("top", v) }

func (c aylisCircle) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisCircle) SetWidth(v float64) { c.Set("width", v) }

func (c aylisCircle) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisCircle) SetHeight(v float64) { c.Set("height", v) }

func (c aylisCircle) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisCircle) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisCircle) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisCircle) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisCircle) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisCircle) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisCircle) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisCircle) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisCircle) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisCircle) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisCircle) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisCircle) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisCircle) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisCircle) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisCircle) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisCircle) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisCircle) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisCircle) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisCircle) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisCircle) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisCircle) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisCircle) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisCircle) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisCircle) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisCircle) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisCircle) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisCircle) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisCircle) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisCircle) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisCircle) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisCircle) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisCircle) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisCircle) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisCircle) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisCircle) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisCircle) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisCircle) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisCircle) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisCircle) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisCircle) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisCircle) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisCircle) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisCircle) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisCircle) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisCircle) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisCircle) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisCircle) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisCircle) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisCircle) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisCircle) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisCircle) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisCircle) SetScale(v float64) { c.Set("scale", v) }

func (c aylisCircle) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisCircle) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisCircle) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisCircle) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisCircle) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisCircle) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisCircle) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisCircle) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisCircle) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisCircle) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisCircle) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisCircle) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisCircle) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisCircle) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisCircle) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisCircle) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisCircle) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisCircle) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisCircle) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisCircle) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisCircle) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisCircle) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisCircle) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisCircle) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisCircle) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisCircle) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisCircle) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisCircle) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisCircle) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisCircle) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisCircle) Style() CircleStyle { return CircleStyle(asText(c.Get("style"))) }
func (c aylisCircle) SetStyle(v CircleStyle) { c.Set("style", string(v)) }

func (c aylisCircle) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisCircle) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisCircle) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisCircle) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisCircle) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisCircle) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisCircle) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisCircle) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisCircle) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisCircle) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisCircle) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisCircle) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisCircle) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisCircle) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisCircle) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisCircle) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisCircle) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisCircle) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisCircle) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisCircle) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisCircle) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisCircle) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisCircle) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisCircle) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisCircle) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisCircle) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisCircle) Hint() string { return asText(c.Get("hint")) }
func (c aylisCircle) SetHint(v string) { c.Set("hint", v) }

func (c aylisCircle) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisCircle) SetTag(v int64) { c.Set("tag", v) }

func (c aylisCircle) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisCircle) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisCircle) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisCircle) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisCircle) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisCircle) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisCircle) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisCircle) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisCircle) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisCircle) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisCircle) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisCircle) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisCircle) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisCircle) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisCircle) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisCircle) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisCircle) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisCircle) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisCircle) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisCircle) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisCircle) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisCircle) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisCircle) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisCircle) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisCircle) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisCircle) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisCircle) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisCircle) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisCircle) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisCircle) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisCircle) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisCircle) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisCircle) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisCircle) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisCircle) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisCircle) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisCircle) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisCircle) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisCircle) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisCircle) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisCircle) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisCircle) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisCircle) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisCircle) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisComboBox é um ComboBox de um form.
type aylisComboBox struct{ aylis.Component }

func (c aylisComboBox) Items() []string { return asList(c.Get("items")) }
func (c aylisComboBox) SetItems(v []string) { c.Set("items", strings.Join(v, ";")) }

func (c aylisComboBox) Values() string { return asText(c.Get("values")) }
func (c aylisComboBox) SetValues(v string) { c.Set("values", v) }

func (c aylisComboBox) ItemIndex() int64 { return int64(asNumber(c.Get("item_index"))) }
func (c aylisComboBox) SetItemIndex(v int64) { c.Set("item_index", v) }

func (c aylisComboBox) HAlign() TextAlign { return TextAlign(asText(c.Get("h_align"))) }
func (c aylisComboBox) SetHAlign(v TextAlign) { c.Set("h_align", string(v)) }

func (c aylisComboBox) Trimming() Trimming { return Trimming(asText(c.Get("trimming"))) }
func (c aylisComboBox) SetTrimming(v Trimming) { c.Set("trimming", string(v)) }

func (c aylisComboBox) Arrow() ComboArrow { return ComboArrow(asText(c.Get("arrow"))) }
func (c aylisComboBox) SetArrow(v ComboArrow) { c.Set("arrow", string(v)) }

func (c aylisComboBox) Sensitive() bool { return asBool(c.Get("sensitive")) }
func (c aylisComboBox) SetSensitive(v bool) { c.Set("sensitive", v) }

func (c aylisComboBox) Groups() []string { return asList(c.Get("groups")) }
func (c aylisComboBox) SetGroups(v []string) { c.Set("groups", strings.Join(v, ";")) }

func (c aylisComboBox) DisabledItems() []string { return asList(c.Get("disabled_items")) }
func (c aylisComboBox) SetDisabledItems(v []string) { c.Set("disabled_items", strings.Join(v, ";")) }

func (c aylisComboBox) DropDownCount() int64 { return int64(asNumber(c.Get("drop_down_count"))) }
func (c aylisComboBox) SetDropDownCount(v int64) { c.Set("drop_down_count", v) }

func (c aylisComboBox) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisComboBox) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisComboBox) Value() string { return asText(c.Get("value")) }
func (c aylisComboBox) SetValue(v string) { c.Set("value", v) }

func (c aylisComboBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisComboBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisComboBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisComboBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisComboBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisComboBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisComboBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisComboBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisComboBox) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisComboBox) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisComboBox) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisComboBox) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisComboBox) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisComboBox) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisComboBox) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisComboBox) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisComboBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisComboBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisComboBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisComboBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisComboBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisComboBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisComboBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisComboBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisComboBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisComboBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisComboBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisComboBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisComboBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisComboBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisComboBox) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisComboBox) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisComboBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisComboBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisComboBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisComboBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisComboBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisComboBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisComboBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisComboBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisComboBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisComboBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisComboBox) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisComboBox) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisComboBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisComboBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisComboBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisComboBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisComboBox) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisComboBox) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisComboBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisComboBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisComboBox) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisComboBox) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisComboBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisComboBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisComboBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisComboBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisComboBox) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisComboBox) SetScale(v float64) { c.Set("scale", v) }

func (c aylisComboBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisComboBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisComboBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisComboBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisComboBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisComboBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisComboBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisComboBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisComboBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisComboBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisComboBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisComboBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisComboBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisComboBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisComboBox) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisComboBox) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisComboBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisComboBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisComboBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisComboBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisComboBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisComboBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisComboBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisComboBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisComboBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisComboBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisComboBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisComboBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisComboBox) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisComboBox) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisComboBox) Style() ComboBoxStyle { return ComboBoxStyle(asText(c.Get("style"))) }
func (c aylisComboBox) SetStyle(v ComboBoxStyle) { c.Set("style", string(v)) }

func (c aylisComboBox) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisComboBox) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisComboBox) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisComboBox) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisComboBox) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisComboBox) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisComboBox) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisComboBox) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisComboBox) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisComboBox) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisComboBox) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisComboBox) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisComboBox) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisComboBox) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisComboBox) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisComboBox) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisComboBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisComboBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisComboBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisComboBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisComboBox) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisComboBox) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisComboBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisComboBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisComboBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisComboBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisComboBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisComboBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisComboBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisComboBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisComboBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisComboBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisComboBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisComboBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisComboBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisComboBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisComboBox) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisComboBox) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisComboBox) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisComboBox) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisComboBox) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisComboBox) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisComboBox) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisComboBox) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisComboBox) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisComboBox) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisComboBox) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisComboBox) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisComboBox) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisComboBox) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisComboBox) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisComboBox) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisComboBox) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisComboBox) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisComboBox) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisComboBox) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisComboBox) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisComboBox) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisComboBox) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisComboBox) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisComboBox) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisComboBox) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisComboBox) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisComboBox) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisComboBox) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisComboBox) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisComboBox) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisComboBox) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisComboBox) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisComboBox) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisComboBox) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisComboBox) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisComboBox) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisComboBox) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisComboBox) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisComboBox) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisComboBox) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisDivider é um Divider de um form.
type aylisDivider struct{ aylis.Component }

func (c aylisDivider) Vertical() bool { return asBool(c.Get("vertical")) }
func (c aylisDivider) SetVertical(v bool) { c.Set("vertical", v) }

func (c aylisDivider) Text() string { return asText(c.Get("text")) }
func (c aylisDivider) SetText(v string) { c.Set("text", v) }

func (c aylisDivider) Thickness() float64 { return asNumber(c.Get("thickness")) }
func (c aylisDivider) SetThickness(v float64) { c.Set("thickness", v) }

func (c aylisDivider) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDivider) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDivider) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDivider) SetTop(v float64) { c.Set("top", v) }

func (c aylisDivider) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDivider) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDivider) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDivider) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDivider) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisDivider) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisDivider) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisDivider) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisDivider) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisDivider) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisDivider) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisDivider) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisDivider) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDivider) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDivider) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDivider) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDivider) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDivider) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDivider) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDivider) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDivider) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisDivider) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisDivider) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisDivider) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisDivider) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDivider) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDivider) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisDivider) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisDivider) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDivider) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDivider) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDivider) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDivider) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDivider) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDivider) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDivider) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDivider) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDivider) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDivider) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisDivider) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisDivider) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDivider) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDivider) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDivider) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDivider) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisDivider) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisDivider) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDivider) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDivider) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisDivider) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisDivider) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDivider) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDivider) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDivider) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDivider) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisDivider) SetScale(v float64) { c.Set("scale", v) }

func (c aylisDivider) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDivider) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDivider) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDivider) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDivider) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDivider) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDivider) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDivider) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDivider) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDivider) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDivider) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDivider) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDivider) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDivider) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDivider) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisDivider) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisDivider) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDivider) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDivider) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDivider) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDivider) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisDivider) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisDivider) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisDivider) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisDivider) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisDivider) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisDivider) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisDivider) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisDivider) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisDivider) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisDivider) Style() DividerStyle { return DividerStyle(asText(c.Get("style"))) }
func (c aylisDivider) SetStyle(v DividerStyle) { c.Set("style", string(v)) }

func (c aylisDivider) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisDivider) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisDivider) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisDivider) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisDivider) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisDivider) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisDivider) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisDivider) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisDivider) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisDivider) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisDivider) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisDivider) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisDivider) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisDivider) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisDivider) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisDivider) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisDivider) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDivider) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDivider) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDivider) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDivider) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisDivider) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisDivider) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDivider) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDivider) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDivider) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDivider) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDivider) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDivider) Hint() string { return asText(c.Get("hint")) }
func (c aylisDivider) SetHint(v string) { c.Set("hint", v) }

func (c aylisDivider) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDivider) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDivider) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisDivider) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisDivider) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisDivider) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisDivider) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisDivider) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisDivider) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisDivider) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisDivider) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisDivider) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisDivider) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisDivider) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisDivider) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisDivider) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisDivider) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisDivider) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisDivider) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisDivider) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisDivider) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisDivider) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisDivider) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisDivider) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisDivider) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisDivider) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisDivider) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisDivider) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisDivider) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisDivider) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisDivider) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisDivider) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisDivider) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisDivider) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisDivider) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisDivider) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisDivider) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisDivider) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisDivider) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisDivider) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisDivider) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisDivider) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisDivider) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisDivider) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisDivider) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisDivider) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

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
func (c aylisForm) GpuAdapter() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapter", "args": []any{}})) }
func (c aylisForm) GpuAdapters() []string { return asList(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "gpu_adapters", "args": []any{}})) }
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

// aylisMainMenu é um MainMenu de um form.
type aylisMainMenu struct{ aylis.Component }

func (c aylisMainMenu) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisMainMenu) SetLeft(v float64) { c.Set("left", v) }

func (c aylisMainMenu) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisMainMenu) SetTop(v float64) { c.Set("top", v) }

func (c aylisMainMenu) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisMainMenu) SetWidth(v float64) { c.Set("width", v) }

func (c aylisMainMenu) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisMainMenu) SetHeight(v float64) { c.Set("height", v) }

func (c aylisMainMenu) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisMainMenu) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisMainMenu) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisMainMenu) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisMainMenu) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisMainMenu) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisMainMenu) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisMainMenu) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisMainMenu) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisMainMenu) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisMainMenu) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisMainMenu) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisMainMenu) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisMainMenu) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisMainMenu) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisMainMenu) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisMainMenu) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisMainMenu) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisMainMenu) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisMainMenu) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisMainMenu) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisMainMenu) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisMainMenu) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisMainMenu) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisMainMenu) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisMainMenu) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisMainMenu) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisMainMenu) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisMainMenu) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisMainMenu) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisMainMenu) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisMainMenu) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisMainMenu) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisMainMenu) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisMainMenu) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisMainMenu) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisMainMenu) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisMainMenu) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisMainMenu) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisMainMenu) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisMainMenu) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisMainMenu) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisMainMenu) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisMainMenu) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisMainMenu) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisMainMenu) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisMainMenu) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisMainMenu) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisMainMenu) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisMainMenu) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisMainMenu) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisMainMenu) SetScale(v float64) { c.Set("scale", v) }

func (c aylisMainMenu) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisMainMenu) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisMainMenu) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisMainMenu) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisMainMenu) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisMainMenu) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisMainMenu) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisMainMenu) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisMainMenu) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisMainMenu) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisMainMenu) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisMainMenu) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisMainMenu) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisMainMenu) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisMainMenu) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisMainMenu) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisMainMenu) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisMainMenu) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisMainMenu) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisMainMenu) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisMainMenu) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisMainMenu) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisMainMenu) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisMainMenu) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisMainMenu) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisMainMenu) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisMainMenu) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisMainMenu) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisMainMenu) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisMainMenu) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisMainMenu) Style() MainMenuStyle { return MainMenuStyle(asText(c.Get("style"))) }
func (c aylisMainMenu) SetStyle(v MainMenuStyle) { c.Set("style", string(v)) }

func (c aylisMainMenu) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisMainMenu) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisMainMenu) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisMainMenu) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisMainMenu) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisMainMenu) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisMainMenu) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisMainMenu) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisMainMenu) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisMainMenu) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisMainMenu) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisMainMenu) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisMainMenu) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisMainMenu) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisMainMenu) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisMainMenu) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisMainMenu) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisMainMenu) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisMainMenu) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisMainMenu) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisMainMenu) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisMainMenu) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisMainMenu) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisMainMenu) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisMainMenu) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisMainMenu) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisMainMenu) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisMainMenu) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisMainMenu) Hint() string { return asText(c.Get("hint")) }
func (c aylisMainMenu) SetHint(v string) { c.Set("hint", v) }

func (c aylisMainMenu) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisMainMenu) SetTag(v int64) { c.Set("tag", v) }

func (c aylisMainMenu) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisMainMenu) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisMainMenu) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisMainMenu) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisMainMenu) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisMainMenu) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisMainMenu) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisMainMenu) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisMainMenu) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisMainMenu) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisMainMenu) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisMainMenu) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisMainMenu) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisMainMenu) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisMainMenu) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisMainMenu) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisMainMenu) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisMainMenu) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisMainMenu) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisMainMenu) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisMainMenu) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisMainMenu) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisMainMenu) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisMainMenu) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisMainMenu) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisMainMenu) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisMainMenu) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisMainMenu) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisMainMenu) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisMainMenu) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisMainMenu) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisMainMenu) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisMainMenu) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisMainMenu) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisMainMenu) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisMainMenu) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisMainMenu) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisMainMenu) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisMainMenu) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisMainMenu) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisMainMenu) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisMainMenu) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisMainMenu) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisMainMenu) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisMenuItem é um MenuItem de um form.
type aylisMenuItem struct{ aylis.Component }

func (c aylisMenuItem) Text() string { return asText(c.Get("text")) }
func (c aylisMenuItem) SetText(v string) { c.Set("text", v) }

func (c aylisMenuItem) Shortcut() string { return asText(c.Get("shortcut")) }
func (c aylisMenuItem) SetShortcut(v string) { c.Set("shortcut", v) }

func (c aylisMenuItem) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisMenuItem) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisMenuItem) Checked() bool { return asBool(c.Get("checked")) }
func (c aylisMenuItem) SetChecked(v bool) { c.Set("checked", v) }

func (c aylisMenuItem) AutoCheck() bool { return asBool(c.Get("auto_check")) }
func (c aylisMenuItem) SetAutoCheck(v bool) { c.Set("auto_check", v) }

func (c aylisMenuItem) Icon() string { return asText(c.Get("icon")) }
func (c aylisMenuItem) SetIcon(v string) { c.Set("icon", v) }

func (c aylisMenuItem) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisMenuItem) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisMenuItem) Danger() bool { return asBool(c.Get("danger")) }
func (c aylisMenuItem) SetDanger(v bool) { c.Set("danger", v) }

func (c aylisMenuItem) Header() bool { return asBool(c.Get("header")) }
func (c aylisMenuItem) SetHeader(v bool) { c.Set("header", v) }

func (c aylisMenuItem) Content() *aylis.Component { return c.GetRef("content") }
func (c aylisMenuItem) SetContent(v aylis.Ref) { c.SetRef("content", v) }

func (c aylisMenuItem) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisMenuItem) SetOnClick(v string) { c.Set("on_click", v) }

// aylisOpenDialog é um OpenDialog de um form.
type aylisOpenDialog struct{ aylis.Component }

func (c aylisOpenDialog) Title() string { return asText(c.Get("title")) }
func (c aylisOpenDialog) SetTitle(v string) { c.Set("title", v) }

func (c aylisOpenDialog) Filter() string { return asText(c.Get("filter")) }
func (c aylisOpenDialog) SetFilter(v string) { c.Set("filter", v) }

func (c aylisOpenDialog) InitialDir() string { return asText(c.Get("initial_dir")) }
func (c aylisOpenDialog) SetInitialDir(v string) { c.Set("initial_dir", v) }

func (c aylisOpenDialog) FileName() string { return asText(c.Get("file_name")) }
func (c aylisOpenDialog) SetFileName(v string) { c.Set("file_name", v) }

func (c aylisOpenDialog) Multiple() bool { return asBool(c.Get("multiple")) }
func (c aylisOpenDialog) SetMultiple(v bool) { c.Set("multiple", v) }

func (c aylisOpenDialog) OnSelect() string { return asText(c.Get("on_select")) }
func (c aylisOpenDialog) SetOnSelect(v string) { c.Set("on_select", v) }

func (c aylisOpenDialog) OnCancel() string { return asText(c.Get("on_cancel")) }
func (c aylisOpenDialog) SetOnCancel(v string) { c.Set("on_cancel", v) }
func (c aylisOpenDialog) Execute() { c.App.Send("execute", aylis.M{"target": c.ID}) }

// aylisPaintCanvas é um PaintCanvas de um form.
type aylisPaintCanvas struct{ aylis.Component }

func (c aylisPaintCanvas) Tool() PaintTool { return PaintTool(asText(c.Get("tool"))) }
func (c aylisPaintCanvas) SetTool(v PaintTool) { c.Set("tool", string(v)) }

func (c aylisPaintCanvas) Color() aylis.Color { return aylis.ColorFromJSON(c.Get("color")) }
func (c aylisPaintCanvas) SetColor(v aylis.Color) { c.Set("color", v.JSON()) }

func (c aylisPaintCanvas) BackColor() aylis.Color { return aylis.ColorFromJSON(c.Get("back_color")) }
func (c aylisPaintCanvas) SetBackColor(v aylis.Color) { c.Set("back_color", v.JSON()) }

func (c aylisPaintCanvas) BrushSize() int64 { return int64(asNumber(c.Get("brush_size"))) }
func (c aylisPaintCanvas) SetBrushSize(v int64) { c.Set("brush_size", v) }

func (c aylisPaintCanvas) Filled() bool { return asBool(c.Get("filled")) }
func (c aylisPaintCanvas) SetFilled(v bool) { c.Set("filled", v) }

func (c aylisPaintCanvas) Zoom() float64 { return asNumber(c.Get("zoom")) }
func (c aylisPaintCanvas) SetZoom(v float64) { c.Set("zoom", v) }

func (c aylisPaintCanvas) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisPaintCanvas) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisPaintCanvas) OnPick() string { return asText(c.Get("on_pick")) }
func (c aylisPaintCanvas) SetOnPick(v string) { c.Set("on_pick", v) }

func (c aylisPaintCanvas) OnCursor() string { return asText(c.Get("on_cursor")) }
func (c aylisPaintCanvas) SetOnCursor(v string) { c.Set("on_cursor", v) }

func (c aylisPaintCanvas) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisPaintCanvas) SetLeft(v float64) { c.Set("left", v) }

func (c aylisPaintCanvas) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisPaintCanvas) SetTop(v float64) { c.Set("top", v) }

func (c aylisPaintCanvas) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisPaintCanvas) SetWidth(v float64) { c.Set("width", v) }

func (c aylisPaintCanvas) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisPaintCanvas) SetHeight(v float64) { c.Set("height", v) }

func (c aylisPaintCanvas) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisPaintCanvas) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisPaintCanvas) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisPaintCanvas) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisPaintCanvas) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisPaintCanvas) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisPaintCanvas) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisPaintCanvas) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisPaintCanvas) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisPaintCanvas) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisPaintCanvas) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisPaintCanvas) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisPaintCanvas) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisPaintCanvas) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisPaintCanvas) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisPaintCanvas) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisPaintCanvas) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisPaintCanvas) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisPaintCanvas) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisPaintCanvas) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisPaintCanvas) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisPaintCanvas) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisPaintCanvas) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisPaintCanvas) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisPaintCanvas) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisPaintCanvas) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisPaintCanvas) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisPaintCanvas) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisPaintCanvas) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisPaintCanvas) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisPaintCanvas) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisPaintCanvas) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisPaintCanvas) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisPaintCanvas) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisPaintCanvas) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisPaintCanvas) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisPaintCanvas) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisPaintCanvas) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisPaintCanvas) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisPaintCanvas) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisPaintCanvas) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisPaintCanvas) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisPaintCanvas) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisPaintCanvas) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisPaintCanvas) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisPaintCanvas) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisPaintCanvas) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisPaintCanvas) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisPaintCanvas) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisPaintCanvas) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisPaintCanvas) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisPaintCanvas) SetScale(v float64) { c.Set("scale", v) }

func (c aylisPaintCanvas) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisPaintCanvas) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisPaintCanvas) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisPaintCanvas) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisPaintCanvas) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisPaintCanvas) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisPaintCanvas) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisPaintCanvas) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisPaintCanvas) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisPaintCanvas) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisPaintCanvas) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisPaintCanvas) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisPaintCanvas) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisPaintCanvas) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisPaintCanvas) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisPaintCanvas) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisPaintCanvas) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisPaintCanvas) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisPaintCanvas) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisPaintCanvas) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisPaintCanvas) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisPaintCanvas) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisPaintCanvas) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisPaintCanvas) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisPaintCanvas) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisPaintCanvas) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisPaintCanvas) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisPaintCanvas) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisPaintCanvas) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisPaintCanvas) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisPaintCanvas) Style() PaintCanvasStyle { return PaintCanvasStyle(asText(c.Get("style"))) }
func (c aylisPaintCanvas) SetStyle(v PaintCanvasStyle) { c.Set("style", string(v)) }

func (c aylisPaintCanvas) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisPaintCanvas) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisPaintCanvas) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisPaintCanvas) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisPaintCanvas) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisPaintCanvas) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisPaintCanvas) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisPaintCanvas) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisPaintCanvas) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisPaintCanvas) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisPaintCanvas) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisPaintCanvas) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisPaintCanvas) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisPaintCanvas) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisPaintCanvas) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisPaintCanvas) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisPaintCanvas) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisPaintCanvas) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisPaintCanvas) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisPaintCanvas) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisPaintCanvas) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisPaintCanvas) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisPaintCanvas) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisPaintCanvas) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisPaintCanvas) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisPaintCanvas) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisPaintCanvas) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisPaintCanvas) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisPaintCanvas) Hint() string { return asText(c.Get("hint")) }
func (c aylisPaintCanvas) SetHint(v string) { c.Set("hint", v) }

func (c aylisPaintCanvas) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisPaintCanvas) SetTag(v int64) { c.Set("tag", v) }

func (c aylisPaintCanvas) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisPaintCanvas) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisPaintCanvas) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisPaintCanvas) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisPaintCanvas) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisPaintCanvas) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisPaintCanvas) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisPaintCanvas) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisPaintCanvas) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisPaintCanvas) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisPaintCanvas) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisPaintCanvas) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisPaintCanvas) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisPaintCanvas) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisPaintCanvas) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisPaintCanvas) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisPaintCanvas) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisPaintCanvas) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisPaintCanvas) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisPaintCanvas) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisPaintCanvas) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisPaintCanvas) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisPaintCanvas) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisPaintCanvas) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisPaintCanvas) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisPaintCanvas) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisPaintCanvas) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisPaintCanvas) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisPaintCanvas) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisPaintCanvas) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisPaintCanvas) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisPaintCanvas) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisPaintCanvas) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisPaintCanvas) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisPaintCanvas) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisPaintCanvas) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisPaintCanvas) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisPaintCanvas) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisPaintCanvas) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisPaintCanvas) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisPaintCanvas) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisPaintCanvas) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisPaintCanvas) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisPaintCanvas) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisPaintCanvas) Undo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "undo", "args": []any{}})) }
func (c aylisPaintCanvas) Redo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "redo", "args": []any{}})) }
func (c aylisPaintCanvas) CanUndo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "can_undo", "args": []any{}})) }
func (c aylisPaintCanvas) CanRedo() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "can_redo", "args": []any{}})) }
func (c aylisPaintCanvas) Size() *ImageSize { return recordImageSize(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "size", "args": []any{}})) }
func (c aylisPaintCanvas) Checksum() string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "checksum", "args": []any{}})) }
func (c aylisPaintCanvas) Clear() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "clear", "args": []any{}})) }
func (c aylisPaintCanvas) Resize(width int64, height int64) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "resize", "args": []any{width, height}})) }
func (c aylisPaintCanvas) New(width int64, height int64) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "new", "args": []any{width, height}})) }
func (c aylisPaintCanvas) Open(path string) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "open", "args": []any{path}})) }
func (c aylisPaintCanvas) Save(path string) bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "save", "args": []any{path}})) }
func (c aylisPaintCanvas) SelectAll() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "select_all", "args": []any{}})) }
func (c aylisPaintCanvas) HasSelection() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "has_selection", "args": []any{}})) }
func (c aylisPaintCanvas) Copy() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "copy", "args": []any{}})) }
func (c aylisPaintCanvas) Cut() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "cut", "args": []any{}})) }
func (c aylisPaintCanvas) Paste() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "paste", "args": []any{}})) }
func (c aylisPaintCanvas) Delete() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "delete", "args": []any{}})) }
func (c aylisPaintCanvas) Deselect() bool { return asBool(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "deselect", "args": []any{}})) }
func (c aylisPaintCanvas) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

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

// aylisSaveDialog é um SaveDialog de um form.
type aylisSaveDialog struct{ aylis.Component }

func (c aylisSaveDialog) Title() string { return asText(c.Get("title")) }
func (c aylisSaveDialog) SetTitle(v string) { c.Set("title", v) }

func (c aylisSaveDialog) Filter() string { return asText(c.Get("filter")) }
func (c aylisSaveDialog) SetFilter(v string) { c.Set("filter", v) }

func (c aylisSaveDialog) InitialDir() string { return asText(c.Get("initial_dir")) }
func (c aylisSaveDialog) SetInitialDir(v string) { c.Set("initial_dir", v) }

func (c aylisSaveDialog) FileName() string { return asText(c.Get("file_name")) }
func (c aylisSaveDialog) SetFileName(v string) { c.Set("file_name", v) }

func (c aylisSaveDialog) Multiple() bool { return asBool(c.Get("multiple")) }
func (c aylisSaveDialog) SetMultiple(v bool) { c.Set("multiple", v) }

func (c aylisSaveDialog) OnSelect() string { return asText(c.Get("on_select")) }
func (c aylisSaveDialog) SetOnSelect(v string) { c.Set("on_select", v) }

func (c aylisSaveDialog) OnCancel() string { return asText(c.Get("on_cancel")) }
func (c aylisSaveDialog) SetOnCancel(v string) { c.Set("on_cancel", v) }
func (c aylisSaveDialog) Execute() { c.App.Send("execute", aylis.M{"target": c.ID}) }

// aylisScrollBox é um ScrollBox de um form.
type aylisScrollBox struct{ aylis.Component }

func (c aylisScrollBox) Overscroll() Overscroll { return Overscroll(asText(c.Get("overscroll"))) }
func (c aylisScrollBox) SetOverscroll(v Overscroll) { c.Set("overscroll", string(v)) }

func (c aylisScrollBox) Horizontal() bool { return asBool(c.Get("horizontal")) }
func (c aylisScrollBox) SetHorizontal(v bool) { c.Set("horizontal", v) }

func (c aylisScrollBox) PullToRefresh() bool { return asBool(c.Get("pull_to_refresh")) }
func (c aylisScrollBox) SetPullToRefresh(v bool) { c.Set("pull_to_refresh", v) }

func (c aylisScrollBox) Refreshing() bool { return asBool(c.Get("refreshing")) }
func (c aylisScrollBox) SetRefreshing(v bool) { c.Set("refreshing", v) }

func (c aylisScrollBox) WheelStep() float64 { return asNumber(c.Get("wheel_step")) }
func (c aylisScrollBox) SetWheelStep(v float64) { c.Set("wheel_step", v) }

func (c aylisScrollBox) Smooth() SmoothScroll { return SmoothScroll(asText(c.Get("smooth"))) }
func (c aylisScrollBox) SetSmooth(v SmoothScroll) { c.Set("smooth", string(v)) }

func (c aylisScrollBox) Gutter() bool { return asBool(c.Get("gutter")) }
func (c aylisScrollBox) SetGutter(v bool) { c.Set("gutter", v) }

func (c aylisScrollBox) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisScrollBox) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisScrollBox) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisScrollBox) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisScrollBox) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisScrollBox) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisScrollBox) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisScrollBox) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisScrollBox) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisScrollBox) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisScrollBox) OnEndReached() string { return asText(c.Get("on_end_reached")) }
func (c aylisScrollBox) SetOnEndReached(v string) { c.Set("on_end_reached", v) }

func (c aylisScrollBox) OnRefresh() string { return asText(c.Get("on_refresh")) }
func (c aylisScrollBox) SetOnRefresh(v string) { c.Set("on_refresh", v) }

func (c aylisScrollBox) OnScroll() string { return asText(c.Get("on_scroll")) }
func (c aylisScrollBox) SetOnScroll(v string) { c.Set("on_scroll", v) }

func (c aylisScrollBox) ScrollPosition() float64 { return asNumber(c.Get("scroll_position")) }
func (c aylisScrollBox) SetScrollPosition(v float64) { c.Set("scroll_position", v) }

func (c aylisScrollBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisScrollBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisScrollBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisScrollBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisScrollBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisScrollBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisScrollBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisScrollBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisScrollBox) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisScrollBox) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisScrollBox) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisScrollBox) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisScrollBox) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisScrollBox) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisScrollBox) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisScrollBox) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisScrollBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisScrollBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisScrollBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisScrollBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisScrollBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisScrollBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisScrollBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisScrollBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisScrollBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisScrollBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisScrollBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisScrollBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisScrollBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisScrollBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisScrollBox) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisScrollBox) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisScrollBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisScrollBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisScrollBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisScrollBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisScrollBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisScrollBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisScrollBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisScrollBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisScrollBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisScrollBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisScrollBox) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisScrollBox) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisScrollBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisScrollBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisScrollBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisScrollBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisScrollBox) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisScrollBox) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisScrollBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisScrollBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisScrollBox) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisScrollBox) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisScrollBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisScrollBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisScrollBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisScrollBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisScrollBox) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisScrollBox) SetScale(v float64) { c.Set("scale", v) }

func (c aylisScrollBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisScrollBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisScrollBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisScrollBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisScrollBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisScrollBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisScrollBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisScrollBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisScrollBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisScrollBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisScrollBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisScrollBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisScrollBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisScrollBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisScrollBox) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisScrollBox) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisScrollBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisScrollBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisScrollBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisScrollBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisScrollBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisScrollBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisScrollBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisScrollBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisScrollBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisScrollBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisScrollBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisScrollBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisScrollBox) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisScrollBox) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisScrollBox) Style() ScrollBoxStyle { return ScrollBoxStyle(asText(c.Get("style"))) }
func (c aylisScrollBox) SetStyle(v ScrollBoxStyle) { c.Set("style", string(v)) }

func (c aylisScrollBox) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisScrollBox) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisScrollBox) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisScrollBox) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisScrollBox) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisScrollBox) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisScrollBox) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisScrollBox) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisScrollBox) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisScrollBox) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisScrollBox) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisScrollBox) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisScrollBox) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisScrollBox) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisScrollBox) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisScrollBox) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisScrollBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisScrollBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisScrollBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisScrollBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisScrollBox) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisScrollBox) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisScrollBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisScrollBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisScrollBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisScrollBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisScrollBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisScrollBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisScrollBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisScrollBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisScrollBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisScrollBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisScrollBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisScrollBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisScrollBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisScrollBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisScrollBox) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisScrollBox) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisScrollBox) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisScrollBox) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisScrollBox) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisScrollBox) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisScrollBox) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisScrollBox) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisScrollBox) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisScrollBox) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisScrollBox) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisScrollBox) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisScrollBox) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisScrollBox) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisScrollBox) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisScrollBox) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisScrollBox) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisScrollBox) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisScrollBox) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisScrollBox) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisScrollBox) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisScrollBox) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisScrollBox) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisScrollBox) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisScrollBox) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisScrollBox) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisScrollBox) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisScrollBox) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisScrollBox) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisScrollBox) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisScrollBox) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisScrollBox) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisScrollBox) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisScrollBox) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisScrollBox) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisScrollBox) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisScrollBox) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisScrollBox) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisScrollBox) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisScrollBox) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisScrollBox) ScrollTo(component aylis.Ref) { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "scroll_to", "args": []any{component.ComponentID()}}) }
func (c aylisScrollBox) VisibleComponents(list aylis.Ref, fully bool) string { return asText(askCmd(c.App, "invoke", aylis.M{"target": c.ID, "method": "visible_components", "args": []any{list.ComponentID(), fully}})) }

// aylisStatusBar é um StatusBar de um form.
type aylisStatusBar struct{ aylis.Component }

func (c aylisStatusBar) Text() string { return asText(c.Get("text")) }
func (c aylisStatusBar) SetText(v string) { c.Set("text", v) }

func (c aylisStatusBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisStatusBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisStatusBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisStatusBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisStatusBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisStatusBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisStatusBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisStatusBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisStatusBar) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisStatusBar) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisStatusBar) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisStatusBar) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisStatusBar) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisStatusBar) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisStatusBar) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisStatusBar) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisStatusBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisStatusBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisStatusBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisStatusBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisStatusBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisStatusBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisStatusBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisStatusBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisStatusBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisStatusBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisStatusBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisStatusBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisStatusBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisStatusBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisStatusBar) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisStatusBar) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisStatusBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisStatusBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisStatusBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisStatusBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisStatusBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisStatusBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisStatusBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisStatusBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisStatusBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisStatusBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisStatusBar) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisStatusBar) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisStatusBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisStatusBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisStatusBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisStatusBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisStatusBar) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisStatusBar) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisStatusBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisStatusBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisStatusBar) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisStatusBar) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisStatusBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisStatusBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisStatusBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisStatusBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisStatusBar) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisStatusBar) SetScale(v float64) { c.Set("scale", v) }

func (c aylisStatusBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisStatusBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisStatusBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisStatusBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisStatusBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisStatusBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisStatusBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisStatusBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisStatusBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisStatusBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisStatusBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisStatusBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisStatusBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisStatusBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisStatusBar) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisStatusBar) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisStatusBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisStatusBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisStatusBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisStatusBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisStatusBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisStatusBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisStatusBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisStatusBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisStatusBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisStatusBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisStatusBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisStatusBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisStatusBar) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisStatusBar) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisStatusBar) Style() StatusBarStyle { return StatusBarStyle(asText(c.Get("style"))) }
func (c aylisStatusBar) SetStyle(v StatusBarStyle) { c.Set("style", string(v)) }

func (c aylisStatusBar) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisStatusBar) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisStatusBar) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisStatusBar) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisStatusBar) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisStatusBar) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisStatusBar) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisStatusBar) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisStatusBar) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisStatusBar) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisStatusBar) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisStatusBar) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisStatusBar) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisStatusBar) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisStatusBar) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisStatusBar) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisStatusBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisStatusBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisStatusBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisStatusBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisStatusBar) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisStatusBar) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisStatusBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisStatusBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisStatusBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisStatusBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisStatusBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisStatusBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisStatusBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisStatusBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisStatusBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisStatusBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisStatusBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisStatusBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisStatusBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisStatusBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisStatusBar) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisStatusBar) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisStatusBar) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisStatusBar) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisStatusBar) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisStatusBar) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisStatusBar) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisStatusBar) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisStatusBar) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisStatusBar) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisStatusBar) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisStatusBar) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisStatusBar) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisStatusBar) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisStatusBar) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisStatusBar) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisStatusBar) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisStatusBar) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisStatusBar) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisStatusBar) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisStatusBar) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisStatusBar) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisStatusBar) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisStatusBar) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisStatusBar) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisStatusBar) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisStatusBar) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisStatusBar) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisStatusBar) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisStatusBar) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisStatusBar) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisStatusBar) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisStatusBar) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisStatusBar) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisStatusBar) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisStatusBar) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisStatusBar) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisStatusBar) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisStatusBar) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisStatusBar) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// frmPaintComponents: o form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmPaintComponents struct {
	aylisForm
	lytTopo aylisLayout
	mnuPrincipal aylisMainMenu
	mnuArquivo aylisMenuItem
	mnuNovo aylisMenuItem
	mnuAbrir aylisMenuItem
	mnuSalvar aylisMenuItem
	mnuSalvarComo aylisMenuItem
	mnuSep1 aylisMenuItem
	mnuSair aylisMenuItem
	mnuEditar aylisMenuItem
	mnuDesfazer aylisMenuItem
	mnuRefazer aylisMenuItem
	mnuSep2 aylisMenuItem
	mnuRecortar aylisMenuItem
	mnuCopiar aylisMenuItem
	mnuColar aylisMenuItem
	mnuExcluir aylisMenuItem
	mnuSelecionarTudo aylisMenuItem
	mnuSep3 aylisMenuItem
	mnuLimpar aylisMenuItem
	mnuExibir aylisMenuItem
	mnuAmpliar aylisMenuItem
	mnuReduzir aylisMenuItem
	mnuTamanhoReal aylisMenuItem
	divTopo aylisDivider
	btnSalvar aylisButton
	btnDesfazer aylisButton
	btnRefazer aylisButton
	pnlFaixa aylisPanel
	lytSelecao aylisLayout
	lblSelecaoGrupo aylisLabel
	btnSelecionar aylisButton
	divSelecao aylisDivider
	lytFerramentas aylisLayout
	lblFerramentasGrupo aylisLabel
	grdFerramentas aylisGridLayout
	btnLapis aylisButton
	btnBalde aylisButton
	btnBorracha aylisButton
	btnContaGotas aylisButton
	divFerramentas aylisDivider
	lytPinceis aylisLayout
	lblPinceis aylisLabel
	btnPincel aylisButton
	divPinceis aylisDivider
	lytFormas aylisLayout
	lblFormas aylisLabel
	chkPreencher aylisCheckBox
	grdFormas aylisGridLayout
	btnLinha aylisButton
	btnRetangulo aylisButton
	btnElipse aylisButton
	divFormas aylisDivider
	lytTamanho aylisLayout
	lblTamanhoGrupo aylisLabel
	cbEspessura aylisComboBox
	divTamanho aylisDivider
	lytCores aylisLayout
	lblCores aylisLabel
	btnCor1 aylisCircle
	btnCor2 aylisCircle
	grdPaleta aylisGridLayout
	btnPaleta01 aylisCircle
	btnPaleta02 aylisCircle
	btnPaleta03 aylisCircle
	btnPaleta04 aylisCircle
	btnPaleta05 aylisCircle
	btnPaleta06 aylisCircle
	btnPaleta07 aylisCircle
	btnPaleta08 aylisCircle
	btnPaleta09 aylisCircle
	btnPaleta10 aylisCircle
	btnPaleta11 aylisCircle
	btnPaleta12 aylisCircle
	btnPaleta13 aylisCircle
	btnPaleta14 aylisCircle
	btnPaleta15 aylisCircle
	btnPaleta16 aylisCircle
	btnPaleta17 aylisCircle
	btnPaleta18 aylisCircle
	btnPaleta19 aylisCircle
	btnPaleta20 aylisCircle
	stbStatus aylisStatusBar
	lblPosicao aylisLabel
	lblTamanho aylisLabel
	lblZoom aylisLabel
	sbxArea aylisScrollBox
	pcDesenho aylisPaintCanvas
	dlgAbrir aylisOpenDialog
	dlgSalvar aylisSaveDialog
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmPaintComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "lytTopo", "Layout"); err != nil {
		return err
	} else {
		f.lytTopo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "mnuPrincipal", "MainMenu"); err != nil {
		return err
	} else {
		f.mnuPrincipal = aylisMainMenu{c}
	}
	if c, err := f.Resolve(list, "mnuArquivo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuArquivo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuNovo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuNovo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAbrir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAbrir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSalvar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSalvar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSalvarComo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSalvarComo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep1", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep1 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSair", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSair = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuEditar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuEditar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuDesfazer", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuDesfazer = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRefazer", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRefazer = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep2", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep2 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRecortar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRecortar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuCopiar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuCopiar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuColar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuColar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExcluir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExcluir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSelecionarTudo", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSelecionarTudo = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep3", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep3 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuLimpar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuLimpar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExibir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExibir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAmpliar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAmpliar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuReduzir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReduzir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuTamanhoReal", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuTamanhoReal = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "divTopo", "Divider"); err != nil {
		return err
	} else {
		f.divTopo = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "btnSalvar", "Button"); err != nil {
		return err
	} else {
		f.btnSalvar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnDesfazer", "Button"); err != nil {
		return err
	} else {
		f.btnDesfazer = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRefazer", "Button"); err != nil {
		return err
	} else {
		f.btnRefazer = aylisButton{c}
	}
	if c, err := f.Resolve(list, "pnlFaixa", "Panel"); err != nil {
		return err
	} else {
		f.pnlFaixa = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lytSelecao", "Layout"); err != nil {
		return err
	} else {
		f.lytSelecao = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblSelecaoGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblSelecaoGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnSelecionar", "Button"); err != nil {
		return err
	} else {
		f.btnSelecionar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divSelecao", "Divider"); err != nil {
		return err
	} else {
		f.divSelecao = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytFerramentas", "Layout"); err != nil {
		return err
	} else {
		f.lytFerramentas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblFerramentasGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblFerramentasGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "grdFerramentas", "GridLayout"); err != nil {
		return err
	} else {
		f.grdFerramentas = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnLapis", "Button"); err != nil {
		return err
	} else {
		f.btnLapis = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnBalde", "Button"); err != nil {
		return err
	} else {
		f.btnBalde = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnBorracha", "Button"); err != nil {
		return err
	} else {
		f.btnBorracha = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnContaGotas", "Button"); err != nil {
		return err
	} else {
		f.btnContaGotas = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divFerramentas", "Divider"); err != nil {
		return err
	} else {
		f.divFerramentas = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytPinceis", "Layout"); err != nil {
		return err
	} else {
		f.lytPinceis = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblPinceis", "Label"); err != nil {
		return err
	} else {
		f.lblPinceis = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnPincel", "Button"); err != nil {
		return err
	} else {
		f.btnPincel = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divPinceis", "Divider"); err != nil {
		return err
	} else {
		f.divPinceis = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytFormas", "Layout"); err != nil {
		return err
	} else {
		f.lytFormas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblFormas", "Label"); err != nil {
		return err
	} else {
		f.lblFormas = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "chkPreencher", "CheckBox"); err != nil {
		return err
	} else {
		f.chkPreencher = aylisCheckBox{c}
	}
	if c, err := f.Resolve(list, "grdFormas", "GridLayout"); err != nil {
		return err
	} else {
		f.grdFormas = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnLinha", "Button"); err != nil {
		return err
	} else {
		f.btnLinha = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRetangulo", "Button"); err != nil {
		return err
	} else {
		f.btnRetangulo = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnElipse", "Button"); err != nil {
		return err
	} else {
		f.btnElipse = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divFormas", "Divider"); err != nil {
		return err
	} else {
		f.divFormas = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytTamanho", "Layout"); err != nil {
		return err
	} else {
		f.lytTamanho = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTamanhoGrupo", "Label"); err != nil {
		return err
	} else {
		f.lblTamanhoGrupo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "cbEspessura", "ComboBox"); err != nil {
		return err
	} else {
		f.cbEspessura = aylisComboBox{c}
	}
	if c, err := f.Resolve(list, "divTamanho", "Divider"); err != nil {
		return err
	} else {
		f.divTamanho = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "lytCores", "Layout"); err != nil {
		return err
	} else {
		f.lytCores = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblCores", "Label"); err != nil {
		return err
	} else {
		f.lblCores = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "btnCor1", "Circle"); err != nil {
		return err
	} else {
		f.btnCor1 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnCor2", "Circle"); err != nil {
		return err
	} else {
		f.btnCor2 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "grdPaleta", "GridLayout"); err != nil {
		return err
	} else {
		f.grdPaleta = aylisGridLayout{c}
	}
	if c, err := f.Resolve(list, "btnPaleta01", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta01 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta02", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta02 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta03", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta03 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta04", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta04 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta05", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta05 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta06", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta06 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta07", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta07 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta08", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta08 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta09", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta09 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta10", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta10 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta11", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta11 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta12", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta12 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta13", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta13 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta14", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta14 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta15", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta15 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta16", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta16 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta17", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta17 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta18", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta18 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta19", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta19 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "btnPaleta20", "Circle"); err != nil {
		return err
	} else {
		f.btnPaleta20 = aylisCircle{c}
	}
	if c, err := f.Resolve(list, "stbStatus", "StatusBar"); err != nil {
		return err
	} else {
		f.stbStatus = aylisStatusBar{c}
	}
	if c, err := f.Resolve(list, "lblPosicao", "Label"); err != nil {
		return err
	} else {
		f.lblPosicao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblTamanho", "Label"); err != nil {
		return err
	} else {
		f.lblTamanho = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblZoom", "Label"); err != nil {
		return err
	} else {
		f.lblZoom = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "sbxArea", "ScrollBox"); err != nil {
		return err
	} else {
		f.sbxArea = aylisScrollBox{c}
	}
	if c, err := f.Resolve(list, "pcDesenho", "PaintCanvas"); err != nil {
		return err
	} else {
		f.pcDesenho = aylisPaintCanvas{c}
	}
	if c, err := f.Resolve(list, "dlgAbrir", "OpenDialog"); err != nil {
		return err
	} else {
		f.dlgAbrir = aylisOpenDialog{c}
	}
	if c, err := f.Resolve(list, "dlgSalvar", "SaveDialog"); err != nil {
		return err
	} else {
		f.dlgSalvar = aylisSaveDialog{c}
	}
	return nil
}

// frmPaint é a janela de frmPaint cujo evento está rodando (a primeira, fora dos handlers).
var frmPaint *frmPaintComponents

func init() {
	aylis.RegisterFormClass("frmPaint", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmPaintComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmPaint = form.(*frmPaintComponents) })
}

// FindButton é o Button deste form com o nome.
func (f *frmPaintComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnSalvar":
		return f.btnSalvar
	case "btnDesfazer":
		return f.btnDesfazer
	case "btnRefazer":
		return f.btnRefazer
	case "btnSelecionar":
		return f.btnSelecionar
	case "btnLapis":
		return f.btnLapis
	case "btnBalde":
		return f.btnBalde
	case "btnBorracha":
		return f.btnBorracha
	case "btnContaGotas":
		return f.btnContaGotas
	case "btnPincel":
		return f.btnPincel
	case "btnLinha":
		return f.btnLinha
	case "btnRetangulo":
		return f.btnRetangulo
	case "btnElipse":
		return f.btnElipse
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmPaintComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindCheckBox é o CheckBox deste form com o nome.
func (f *frmPaintComponents) FindCheckBox(name string) aylisCheckBox {
	switch name {
	case "chkPreencher":
		return f.chkPreencher
	}
	return aylisCheckBox{}
}

// SenderAsCheckBox é o CheckBox que disparou o evento.
func (f *frmPaintComponents) SenderAsCheckBox(ev aylis.Event) aylisCheckBox { return f.FindCheckBox(ev.Sender.Name) }

// FindCircle é o Circle deste form com o nome.
func (f *frmPaintComponents) FindCircle(name string) aylisCircle {
	switch name {
	case "btnCor1":
		return f.btnCor1
	case "btnCor2":
		return f.btnCor2
	case "btnPaleta01":
		return f.btnPaleta01
	case "btnPaleta02":
		return f.btnPaleta02
	case "btnPaleta03":
		return f.btnPaleta03
	case "btnPaleta04":
		return f.btnPaleta04
	case "btnPaleta05":
		return f.btnPaleta05
	case "btnPaleta06":
		return f.btnPaleta06
	case "btnPaleta07":
		return f.btnPaleta07
	case "btnPaleta08":
		return f.btnPaleta08
	case "btnPaleta09":
		return f.btnPaleta09
	case "btnPaleta10":
		return f.btnPaleta10
	case "btnPaleta11":
		return f.btnPaleta11
	case "btnPaleta12":
		return f.btnPaleta12
	case "btnPaleta13":
		return f.btnPaleta13
	case "btnPaleta14":
		return f.btnPaleta14
	case "btnPaleta15":
		return f.btnPaleta15
	case "btnPaleta16":
		return f.btnPaleta16
	case "btnPaleta17":
		return f.btnPaleta17
	case "btnPaleta18":
		return f.btnPaleta18
	case "btnPaleta19":
		return f.btnPaleta19
	case "btnPaleta20":
		return f.btnPaleta20
	}
	return aylisCircle{}
}

// SenderAsCircle é o Circle que disparou o evento.
func (f *frmPaintComponents) SenderAsCircle(ev aylis.Event) aylisCircle { return f.FindCircle(ev.Sender.Name) }

// FindComboBox é o ComboBox deste form com o nome.
func (f *frmPaintComponents) FindComboBox(name string) aylisComboBox {
	switch name {
	case "cbEspessura":
		return f.cbEspessura
	}
	return aylisComboBox{}
}

// SenderAsComboBox é o ComboBox que disparou o evento.
func (f *frmPaintComponents) SenderAsComboBox(ev aylis.Event) aylisComboBox { return f.FindComboBox(ev.Sender.Name) }

// FindDivider é o Divider deste form com o nome.
func (f *frmPaintComponents) FindDivider(name string) aylisDivider {
	switch name {
	case "divTopo":
		return f.divTopo
	case "divSelecao":
		return f.divSelecao
	case "divFerramentas":
		return f.divFerramentas
	case "divPinceis":
		return f.divPinceis
	case "divFormas":
		return f.divFormas
	case "divTamanho":
		return f.divTamanho
	}
	return aylisDivider{}
}

// SenderAsDivider é o Divider que disparou o evento.
func (f *frmPaintComponents) SenderAsDivider(ev aylis.Event) aylisDivider { return f.FindDivider(ev.Sender.Name) }

// FindGridLayout é o GridLayout deste form com o nome.
func (f *frmPaintComponents) FindGridLayout(name string) aylisGridLayout {
	switch name {
	case "grdFerramentas":
		return f.grdFerramentas
	case "grdFormas":
		return f.grdFormas
	case "grdPaleta":
		return f.grdPaleta
	}
	return aylisGridLayout{}
}

// SenderAsGridLayout é o GridLayout que disparou o evento.
func (f *frmPaintComponents) SenderAsGridLayout(ev aylis.Event) aylisGridLayout { return f.FindGridLayout(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome.
func (f *frmPaintComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblSelecaoGrupo":
		return f.lblSelecaoGrupo
	case "lblFerramentasGrupo":
		return f.lblFerramentasGrupo
	case "lblPinceis":
		return f.lblPinceis
	case "lblFormas":
		return f.lblFormas
	case "lblTamanhoGrupo":
		return f.lblTamanhoGrupo
	case "lblCores":
		return f.lblCores
	case "lblPosicao":
		return f.lblPosicao
	case "lblTamanho":
		return f.lblTamanho
	case "lblZoom":
		return f.lblZoom
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmPaintComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome.
func (f *frmPaintComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytTopo":
		return f.lytTopo
	case "lytSelecao":
		return f.lytSelecao
	case "lytFerramentas":
		return f.lytFerramentas
	case "lytPinceis":
		return f.lytPinceis
	case "lytFormas":
		return f.lytFormas
	case "lytTamanho":
		return f.lytTamanho
	case "lytCores":
		return f.lytCores
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmPaintComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindMainMenu é o MainMenu deste form com o nome.
func (f *frmPaintComponents) FindMainMenu(name string) aylisMainMenu {
	switch name {
	case "mnuPrincipal":
		return f.mnuPrincipal
	}
	return aylisMainMenu{}
}

// SenderAsMainMenu é o MainMenu que disparou o evento.
func (f *frmPaintComponents) SenderAsMainMenu(ev aylis.Event) aylisMainMenu { return f.FindMainMenu(ev.Sender.Name) }

// FindMenuItem é o MenuItem deste form com o nome.
func (f *frmPaintComponents) FindMenuItem(name string) aylisMenuItem {
	switch name {
	case "mnuArquivo":
		return f.mnuArquivo
	case "mnuNovo":
		return f.mnuNovo
	case "mnuAbrir":
		return f.mnuAbrir
	case "mnuSalvar":
		return f.mnuSalvar
	case "mnuSalvarComo":
		return f.mnuSalvarComo
	case "mnuSep1":
		return f.mnuSep1
	case "mnuSair":
		return f.mnuSair
	case "mnuEditar":
		return f.mnuEditar
	case "mnuDesfazer":
		return f.mnuDesfazer
	case "mnuRefazer":
		return f.mnuRefazer
	case "mnuSep2":
		return f.mnuSep2
	case "mnuRecortar":
		return f.mnuRecortar
	case "mnuCopiar":
		return f.mnuCopiar
	case "mnuColar":
		return f.mnuColar
	case "mnuExcluir":
		return f.mnuExcluir
	case "mnuSelecionarTudo":
		return f.mnuSelecionarTudo
	case "mnuSep3":
		return f.mnuSep3
	case "mnuLimpar":
		return f.mnuLimpar
	case "mnuExibir":
		return f.mnuExibir
	case "mnuAmpliar":
		return f.mnuAmpliar
	case "mnuReduzir":
		return f.mnuReduzir
	case "mnuTamanhoReal":
		return f.mnuTamanhoReal
	}
	return aylisMenuItem{}
}

// SenderAsMenuItem é o MenuItem que disparou o evento.
func (f *frmPaintComponents) SenderAsMenuItem(ev aylis.Event) aylisMenuItem { return f.FindMenuItem(ev.Sender.Name) }

// FindOpenDialog é o OpenDialog deste form com o nome.
func (f *frmPaintComponents) FindOpenDialog(name string) aylisOpenDialog {
	switch name {
	case "dlgAbrir":
		return f.dlgAbrir
	}
	return aylisOpenDialog{}
}

// SenderAsOpenDialog é o OpenDialog que disparou o evento.
func (f *frmPaintComponents) SenderAsOpenDialog(ev aylis.Event) aylisOpenDialog { return f.FindOpenDialog(ev.Sender.Name) }

// FindPaintCanvas é o PaintCanvas deste form com o nome.
func (f *frmPaintComponents) FindPaintCanvas(name string) aylisPaintCanvas {
	switch name {
	case "pcDesenho":
		return f.pcDesenho
	}
	return aylisPaintCanvas{}
}

// SenderAsPaintCanvas é o PaintCanvas que disparou o evento.
func (f *frmPaintComponents) SenderAsPaintCanvas(ev aylis.Event) aylisPaintCanvas { return f.FindPaintCanvas(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome.
func (f *frmPaintComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlFaixa":
		return f.pnlFaixa
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmPaintComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSaveDialog é o SaveDialog deste form com o nome.
func (f *frmPaintComponents) FindSaveDialog(name string) aylisSaveDialog {
	switch name {
	case "dlgSalvar":
		return f.dlgSalvar
	}
	return aylisSaveDialog{}
}

// SenderAsSaveDialog é o SaveDialog que disparou o evento.
func (f *frmPaintComponents) SenderAsSaveDialog(ev aylis.Event) aylisSaveDialog { return f.FindSaveDialog(ev.Sender.Name) }

// FindScrollBox é o ScrollBox deste form com o nome.
func (f *frmPaintComponents) FindScrollBox(name string) aylisScrollBox {
	switch name {
	case "sbxArea":
		return f.sbxArea
	}
	return aylisScrollBox{}
}

// SenderAsScrollBox é o ScrollBox que disparou o evento.
func (f *frmPaintComponents) SenderAsScrollBox(ev aylis.Event) aylisScrollBox { return f.FindScrollBox(ev.Sender.Name) }

// FindStatusBar é o StatusBar deste form com o nome.
func (f *frmPaintComponents) FindStatusBar(name string) aylisStatusBar {
	switch name {
	case "stbStatus":
		return f.stbStatus
	}
	return aylisStatusBar{}
}

// SenderAsStatusBar é o StatusBar que disparou o evento.
func (f *frmPaintComponents) SenderAsStatusBar(ev aylis.Event) aylisStatusBar { return f.FindStatusBar(ev.Sender.Name) }
