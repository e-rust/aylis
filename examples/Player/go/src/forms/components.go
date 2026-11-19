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

// IconStyle são os valores de um enum do acesso tipado.
type IconStyle string

const (
	IconStyleDefault IconStyle = ""
	IconStyleMuted IconStyle = "muted"
)

func enumListIconStyle(v any) []IconStyle {
	var out []IconStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, IconStyle(asText(x)))
		}
	}
	return out
}

func enumNamesIconStyle(vs []IconStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ImageFit são os valores de um enum do acesso tipado.
type ImageFit string

const (
	ImageFitContain ImageFit = "contain"
	ImageFitCover ImageFit = "cover"
	ImageFitStretch ImageFit = "stretch"
	ImageFitCenter ImageFit = "center"
	ImageFitOriginal ImageFit = "original"
)

func enumListImageFit(v any) []ImageFit {
	var out []ImageFit
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ImageFit(asText(x)))
		}
	}
	return out
}

func enumNamesImageFit(vs []ImageFit) []string {
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

// SeekBarStyle são os valores de um enum do acesso tipado.
type SeekBarStyle string

const (
	SeekBarStyleDefault SeekBarStyle = ""
)

func enumListSeekBarStyle(v any) []SeekBarStyle {
	var out []SeekBarStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, SeekBarStyle(asText(x)))
		}
	}
	return out
}

func enumNamesSeekBarStyle(vs []SeekBarStyle) []string {
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

// TimerPrecision são os valores de um enum do acesso tipado.
type TimerPrecision string

const (
	TimerPrecisionNormal TimerPrecision = "normal"
	TimerPrecisionHigh TimerPrecision = "high"
	TimerPrecisionExact TimerPrecision = "exact"
)

func enumListTimerPrecision(v any) []TimerPrecision {
	var out []TimerPrecision
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TimerPrecision(asText(x)))
		}
	}
	return out
}

func enumNamesTimerPrecision(vs []TimerPrecision) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// TrackBarStyle são os valores de um enum do acesso tipado.
type TrackBarStyle string

const (
	TrackBarStyleDefault TrackBarStyle = ""
)

func enumListTrackBarStyle(v any) []TrackBarStyle {
	var out []TrackBarStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TrackBarStyle(asText(x)))
		}
	}
	return out
}

func enumNamesTrackBarStyle(vs []TrackBarStyle) []string {
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

// VideoPlayerStyle são os valores de um enum do acesso tipado.
type VideoPlayerStyle string

const (
	VideoPlayerStyleDefault VideoPlayerStyle = ""
)

func enumListVideoPlayerStyle(v any) []VideoPlayerStyle {
	var out []VideoPlayerStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, VideoPlayerStyle(asText(x)))
		}
	}
	return out
}

func enumNamesVideoPlayerStyle(vs []VideoPlayerStyle) []string {
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

// OutputSpec é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type OutputSpec struct {
	Name string
	Volume float64
	DelayMs float64
}

func recordOfOutputSpec(v any) OutputSpec {
	m, _ := v.(map[string]any)
	return OutputSpec{
		Name: asText(m["name"]),
		Volume: asNumber(m["volume"]),
		DelayMs: asNumber(m["delay_ms"]),
	}
}

func (r OutputSpec) jsonValue() map[string]any {
	return map[string]any{
		"name": r.Name,
		"volume": r.Volume,
		"delay_ms": r.DelayMs,
	}
}

func jsonListOutputSpec(items []OutputSpec) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfOutputSpec(v any) []OutputSpec {
	l, _ := v.([]any)
	out := make([]OutputSpec, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfOutputSpec(x))
	}
	return out
}

func recordOutputSpec(v any) *OutputSpec {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfOutputSpec(j)
	return &r
}

func recordsOutputSpec(v any) []OutputSpec {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfOutputSpec(j)
}

func recordTextOutputSpec(r *OutputSpec) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextOutputSpec(items []OutputSpec) string {
	b, _ := json.Marshal(jsonListOutputSpec(items))
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

// aylisIcon é um Icon de um form.
type aylisIcon struct{ aylis.Component }

func (c aylisIcon) Icon() string { return asText(c.Get("icon")) }
func (c aylisIcon) SetIcon(v string) { c.Set("icon", v) }

func (c aylisIcon) Color() aylis.Color { return aylis.ColorFromJSON(c.Get("color")) }
func (c aylisIcon) SetColor(v aylis.Color) { c.Set("color", v.JSON()) }

func (c aylisIcon) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisIcon) SetLeft(v float64) { c.Set("left", v) }

func (c aylisIcon) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisIcon) SetTop(v float64) { c.Set("top", v) }

func (c aylisIcon) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisIcon) SetWidth(v float64) { c.Set("width", v) }

func (c aylisIcon) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisIcon) SetHeight(v float64) { c.Set("height", v) }

func (c aylisIcon) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisIcon) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisIcon) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisIcon) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisIcon) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisIcon) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisIcon) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisIcon) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisIcon) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisIcon) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisIcon) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisIcon) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisIcon) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisIcon) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisIcon) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisIcon) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisIcon) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisIcon) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisIcon) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisIcon) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisIcon) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisIcon) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisIcon) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisIcon) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisIcon) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisIcon) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisIcon) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisIcon) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisIcon) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisIcon) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisIcon) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisIcon) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisIcon) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisIcon) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisIcon) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisIcon) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisIcon) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisIcon) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisIcon) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisIcon) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisIcon) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisIcon) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisIcon) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisIcon) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisIcon) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisIcon) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisIcon) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisIcon) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisIcon) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisIcon) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisIcon) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisIcon) SetScale(v float64) { c.Set("scale", v) }

func (c aylisIcon) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisIcon) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisIcon) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisIcon) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisIcon) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisIcon) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisIcon) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisIcon) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisIcon) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisIcon) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisIcon) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisIcon) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisIcon) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisIcon) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisIcon) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisIcon) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisIcon) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisIcon) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisIcon) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisIcon) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisIcon) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisIcon) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisIcon) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisIcon) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisIcon) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisIcon) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisIcon) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisIcon) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisIcon) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisIcon) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisIcon) Style() IconStyle { return IconStyle(asText(c.Get("style"))) }
func (c aylisIcon) SetStyle(v IconStyle) { c.Set("style", string(v)) }

func (c aylisIcon) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisIcon) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisIcon) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisIcon) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisIcon) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisIcon) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisIcon) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisIcon) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisIcon) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisIcon) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisIcon) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisIcon) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisIcon) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisIcon) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisIcon) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisIcon) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisIcon) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisIcon) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisIcon) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisIcon) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisIcon) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisIcon) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisIcon) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisIcon) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisIcon) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisIcon) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisIcon) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisIcon) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisIcon) Hint() string { return asText(c.Get("hint")) }
func (c aylisIcon) SetHint(v string) { c.Set("hint", v) }

func (c aylisIcon) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisIcon) SetTag(v int64) { c.Set("tag", v) }

func (c aylisIcon) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisIcon) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisIcon) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisIcon) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisIcon) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisIcon) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisIcon) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisIcon) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisIcon) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisIcon) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisIcon) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisIcon) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisIcon) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisIcon) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisIcon) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisIcon) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisIcon) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisIcon) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisIcon) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisIcon) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisIcon) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisIcon) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisIcon) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisIcon) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisIcon) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisIcon) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisIcon) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisIcon) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisIcon) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisIcon) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisIcon) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisIcon) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisIcon) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisIcon) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisIcon) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisIcon) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisIcon) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisIcon) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisIcon) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisIcon) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisIcon) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisIcon) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisIcon) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisIcon) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

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

// aylisSeekBar é um SeekBar de um form.
type aylisSeekBar struct{ aylis.Component }

func (c aylisSeekBar) Position() float64 { return asNumber(c.Get("position")) }
func (c aylisSeekBar) SetPosition(v float64) { c.Set("position", v) }

func (c aylisSeekBar) Duration() float64 { return asNumber(c.Get("duration")) }
func (c aylisSeekBar) SetDuration(v float64) { c.Set("duration", v) }

func (c aylisSeekBar) Buffered() float64 { return asNumber(c.Get("buffered")) }
func (c aylisSeekBar) SetBuffered(v float64) { c.Set("buffered", v) }

func (c aylisSeekBar) Markers() []float64 { return asNumbers(c.Get("markers")) }
func (c aylisSeekBar) SetMarkers(v []float64) { c.Set("markers", numbersText(v)) }

func (c aylisSeekBar) ShowTime() bool { return asBool(c.Get("show_time")) }
func (c aylisSeekBar) SetShowTime(v bool) { c.Set("show_time", v) }

func (c aylisSeekBar) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisSeekBar) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisSeekBar) OnSeek() string { return asText(c.Get("on_seek")) }
func (c aylisSeekBar) SetOnSeek(v string) { c.Set("on_seek", v) }

func (c aylisSeekBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSeekBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSeekBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSeekBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisSeekBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSeekBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSeekBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSeekBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSeekBar) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisSeekBar) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisSeekBar) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisSeekBar) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisSeekBar) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisSeekBar) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisSeekBar) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisSeekBar) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisSeekBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSeekBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSeekBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSeekBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSeekBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSeekBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSeekBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSeekBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSeekBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisSeekBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisSeekBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisSeekBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisSeekBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSeekBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSeekBar) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisSeekBar) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisSeekBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSeekBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSeekBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSeekBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSeekBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSeekBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSeekBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSeekBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSeekBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSeekBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSeekBar) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisSeekBar) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisSeekBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSeekBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSeekBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSeekBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSeekBar) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisSeekBar) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisSeekBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSeekBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSeekBar) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisSeekBar) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisSeekBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSeekBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSeekBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSeekBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSeekBar) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisSeekBar) SetScale(v float64) { c.Set("scale", v) }

func (c aylisSeekBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSeekBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSeekBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSeekBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSeekBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSeekBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSeekBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSeekBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSeekBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSeekBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSeekBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSeekBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSeekBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSeekBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSeekBar) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisSeekBar) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisSeekBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSeekBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSeekBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSeekBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSeekBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisSeekBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisSeekBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisSeekBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisSeekBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisSeekBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisSeekBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisSeekBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisSeekBar) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisSeekBar) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisSeekBar) Style() SeekBarStyle { return SeekBarStyle(asText(c.Get("style"))) }
func (c aylisSeekBar) SetStyle(v SeekBarStyle) { c.Set("style", string(v)) }

func (c aylisSeekBar) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisSeekBar) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisSeekBar) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisSeekBar) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisSeekBar) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisSeekBar) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisSeekBar) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisSeekBar) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisSeekBar) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisSeekBar) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisSeekBar) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisSeekBar) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisSeekBar) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisSeekBar) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisSeekBar) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisSeekBar) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisSeekBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSeekBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSeekBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSeekBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSeekBar) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisSeekBar) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisSeekBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSeekBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSeekBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSeekBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSeekBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSeekBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSeekBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisSeekBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisSeekBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSeekBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSeekBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisSeekBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisSeekBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisSeekBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisSeekBar) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisSeekBar) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisSeekBar) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisSeekBar) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisSeekBar) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisSeekBar) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisSeekBar) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisSeekBar) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisSeekBar) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisSeekBar) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisSeekBar) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisSeekBar) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisSeekBar) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisSeekBar) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisSeekBar) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisSeekBar) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisSeekBar) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisSeekBar) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisSeekBar) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisSeekBar) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisSeekBar) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisSeekBar) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisSeekBar) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisSeekBar) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisSeekBar) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisSeekBar) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisSeekBar) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisSeekBar) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisSeekBar) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisSeekBar) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisSeekBar) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisSeekBar) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisSeekBar) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisSeekBar) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisSeekBar) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisSeekBar) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisSeekBar) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisSeekBar) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisSeekBar) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisSeekBar) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisSeekBar) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisTimer é um Timer de um form.
type aylisTimer struct{ aylis.Component }

func (c aylisTimer) Interval() float64 { return asNumber(c.Get("interval")) }
func (c aylisTimer) SetInterval(v float64) { c.Set("interval", v) }

func (c aylisTimer) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTimer) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTimer) Precision() TimerPrecision { return TimerPrecision(asText(c.Get("precision"))) }
func (c aylisTimer) SetPrecision(v TimerPrecision) { c.Set("precision", string(v)) }

func (c aylisTimer) Background() bool { return asBool(c.Get("background")) }
func (c aylisTimer) SetBackground(v bool) { c.Set("background", v) }

func (c aylisTimer) OneShot() bool { return asBool(c.Get("one_shot")) }
func (c aylisTimer) SetOneShot(v bool) { c.Set("one_shot", v) }

func (c aylisTimer) OnTimer() string { return asText(c.Get("on_timer")) }
func (c aylisTimer) SetOnTimer(v string) { c.Set("on_timer", v) }

func (c aylisTimer) Drift() float64 { return asNumber(c.Get("drift")) }
func (c aylisTimer) SetDrift(v float64) { c.Set("drift", v) }
func (c aylisTimer) Restart() { c.App.Send("invoke", aylis.M{"target": c.ID, "method": "restart", "args": []any{}}) }

// aylisTrackBar é um TrackBar de um form.
type aylisTrackBar struct{ aylis.Component }

func (c aylisTrackBar) Value() float64 { return asNumber(c.Get("value")) }
func (c aylisTrackBar) SetValue(v float64) { c.Set("value", v) }

func (c aylisTrackBar) Min() float64 { return asNumber(c.Get("min")) }
func (c aylisTrackBar) SetMin(v float64) { c.Set("min", v) }

func (c aylisTrackBar) Max() float64 { return asNumber(c.Get("max")) }
func (c aylisTrackBar) SetMax(v float64) { c.Set("max", v) }

func (c aylisTrackBar) Frequency() float64 { return asNumber(c.Get("frequency")) }
func (c aylisTrackBar) SetFrequency(v float64) { c.Set("frequency", v) }

func (c aylisTrackBar) Vertical() bool { return asBool(c.Get("vertical")) }
func (c aylisTrackBar) SetVertical(v bool) { c.Set("vertical", v) }

func (c aylisTrackBar) ThumbSize() float64 { return asNumber(c.Get("thumb_size")) }
func (c aylisTrackBar) SetThumbSize(v float64) { c.Set("thumb_size", v) }

func (c aylisTrackBar) TrackSize() float64 { return asNumber(c.Get("track_size")) }
func (c aylisTrackBar) SetTrackSize(v float64) { c.Set("track_size", v) }

func (c aylisTrackBar) SolidThumb() bool { return asBool(c.Get("solid_thumb")) }
func (c aylisTrackBar) SetSolidThumb(v bool) { c.Set("solid_thumb", v) }

func (c aylisTrackBar) Limit() float64 { return asNumber(c.Get("limit")) }
func (c aylisTrackBar) SetLimit(v float64) { c.Set("limit", v) }

func (c aylisTrackBar) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisTrackBar) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisTrackBar) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisTrackBar) SetLeft(v float64) { c.Set("left", v) }

func (c aylisTrackBar) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisTrackBar) SetTop(v float64) { c.Set("top", v) }

func (c aylisTrackBar) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisTrackBar) SetWidth(v float64) { c.Set("width", v) }

func (c aylisTrackBar) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisTrackBar) SetHeight(v float64) { c.Set("height", v) }

func (c aylisTrackBar) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisTrackBar) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisTrackBar) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisTrackBar) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisTrackBar) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisTrackBar) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisTrackBar) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisTrackBar) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisTrackBar) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisTrackBar) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisTrackBar) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisTrackBar) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisTrackBar) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisTrackBar) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisTrackBar) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisTrackBar) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisTrackBar) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisTrackBar) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisTrackBar) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisTrackBar) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisTrackBar) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisTrackBar) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisTrackBar) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisTrackBar) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisTrackBar) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisTrackBar) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisTrackBar) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisTrackBar) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisTrackBar) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisTrackBar) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisTrackBar) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisTrackBar) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisTrackBar) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisTrackBar) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisTrackBar) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisTrackBar) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisTrackBar) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisTrackBar) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisTrackBar) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisTrackBar) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisTrackBar) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisTrackBar) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisTrackBar) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisTrackBar) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisTrackBar) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisTrackBar) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisTrackBar) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisTrackBar) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisTrackBar) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisTrackBar) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisTrackBar) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisTrackBar) SetScale(v float64) { c.Set("scale", v) }

func (c aylisTrackBar) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisTrackBar) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisTrackBar) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisTrackBar) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisTrackBar) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisTrackBar) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisTrackBar) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisTrackBar) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisTrackBar) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisTrackBar) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisTrackBar) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisTrackBar) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisTrackBar) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisTrackBar) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisTrackBar) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisTrackBar) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisTrackBar) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisTrackBar) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisTrackBar) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisTrackBar) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisTrackBar) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisTrackBar) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisTrackBar) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisTrackBar) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisTrackBar) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisTrackBar) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisTrackBar) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisTrackBar) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisTrackBar) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisTrackBar) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisTrackBar) Style() TrackBarStyle { return TrackBarStyle(asText(c.Get("style"))) }
func (c aylisTrackBar) SetStyle(v TrackBarStyle) { c.Set("style", string(v)) }

func (c aylisTrackBar) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisTrackBar) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisTrackBar) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisTrackBar) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisTrackBar) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisTrackBar) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisTrackBar) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisTrackBar) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisTrackBar) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisTrackBar) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisTrackBar) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisTrackBar) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisTrackBar) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisTrackBar) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisTrackBar) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisTrackBar) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisTrackBar) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisTrackBar) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisTrackBar) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisTrackBar) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisTrackBar) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisTrackBar) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisTrackBar) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisTrackBar) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisTrackBar) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisTrackBar) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisTrackBar) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisTrackBar) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisTrackBar) Hint() string { return asText(c.Get("hint")) }
func (c aylisTrackBar) SetHint(v string) { c.Set("hint", v) }

func (c aylisTrackBar) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisTrackBar) SetTag(v int64) { c.Set("tag", v) }

func (c aylisTrackBar) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisTrackBar) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisTrackBar) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisTrackBar) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisTrackBar) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisTrackBar) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisTrackBar) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisTrackBar) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisTrackBar) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisTrackBar) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisTrackBar) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisTrackBar) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisTrackBar) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisTrackBar) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisTrackBar) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisTrackBar) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisTrackBar) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisTrackBar) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisTrackBar) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisTrackBar) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisTrackBar) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisTrackBar) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisTrackBar) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisTrackBar) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisTrackBar) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisTrackBar) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisTrackBar) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisTrackBar) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisTrackBar) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisTrackBar) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisTrackBar) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisTrackBar) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisTrackBar) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisTrackBar) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisTrackBar) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisTrackBar) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisTrackBar) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisTrackBar) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisTrackBar) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisTrackBar) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisTrackBar) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisTrackBar) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisTrackBar) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisTrackBar) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisTrackBar) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisVideoPlayer é um VideoPlayer de um form.
type aylisVideoPlayer struct{ aylis.Component }

func (c aylisVideoPlayer) File() string { return asText(c.Get("file")) }
func (c aylisVideoPlayer) SetFile(v string) { c.Set("file", v) }

func (c aylisVideoPlayer) Autoplay() bool { return asBool(c.Get("autoplay")) }
func (c aylisVideoPlayer) SetAutoplay(v bool) { c.Set("autoplay", v) }

func (c aylisVideoPlayer) Looping() bool { return asBool(c.Get("looping")) }
func (c aylisVideoPlayer) SetLooping(v bool) { c.Set("looping", v) }

func (c aylisVideoPlayer) Volume() float64 { return asNumber(c.Get("volume")) }
func (c aylisVideoPlayer) SetVolume(v float64) { c.Set("volume", v) }

func (c aylisVideoPlayer) Muted() bool { return asBool(c.Get("muted")) }
func (c aylisVideoPlayer) SetMuted(v bool) { c.Set("muted", v) }

func (c aylisVideoPlayer) Fit() ImageFit { return ImageFit(asText(c.Get("fit"))) }
func (c aylisVideoPlayer) SetFit(v ImageFit) { c.Set("fit", string(v)) }

func (c aylisVideoPlayer) Radius() float64 { return asNumber(c.Get("radius")) }
func (c aylisVideoPlayer) SetRadius(v float64) { c.Set("radius", v) }

func (c aylisVideoPlayer) Rate() float64 { return asNumber(c.Get("rate")) }
func (c aylisVideoPlayer) SetRate(v float64) { c.Set("rate", v) }

func (c aylisVideoPlayer) Output() []OutputSpec { return recordsOutputSpec(c.Get("output")) }
func (c aylisVideoPlayer) SetOutput(v []OutputSpec) { c.Set("output", recordsTextOutputSpec(v)) }

func (c aylisVideoPlayer) ShowVideo() bool { return asBool(c.Get("show_video")) }
func (c aylisVideoPlayer) SetShowVideo(v bool) { c.Set("show_video", v) }

func (c aylisVideoPlayer) FramesAhead() int64 { return int64(asNumber(c.Get("frames_ahead"))) }
func (c aylisVideoPlayer) SetFramesAhead(v int64) { c.Set("frames_ahead", v) }

func (c aylisVideoPlayer) OnReady() string { return asText(c.Get("on_ready")) }
func (c aylisVideoPlayer) SetOnReady(v string) { c.Set("on_ready", v) }

func (c aylisVideoPlayer) OnEnd() string { return asText(c.Get("on_end")) }
func (c aylisVideoPlayer) SetOnEnd(v string) { c.Set("on_end", v) }

func (c aylisVideoPlayer) OnError() string { return asText(c.Get("on_error")) }
func (c aylisVideoPlayer) SetOnError(v string) { c.Set("on_error", v) }

func (c aylisVideoPlayer) OnFrame() string { return asText(c.Get("on_frame")) }
func (c aylisVideoPlayer) SetOnFrame(v string) { c.Set("on_frame", v) }

func (c aylisVideoPlayer) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisVideoPlayer) SetLeft(v float64) { c.Set("left", v) }

func (c aylisVideoPlayer) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisVideoPlayer) SetTop(v float64) { c.Set("top", v) }

func (c aylisVideoPlayer) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisVideoPlayer) SetWidth(v float64) { c.Set("width", v) }

func (c aylisVideoPlayer) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisVideoPlayer) SetHeight(v float64) { c.Set("height", v) }

func (c aylisVideoPlayer) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisVideoPlayer) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisVideoPlayer) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisVideoPlayer) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisVideoPlayer) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisVideoPlayer) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisVideoPlayer) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisVideoPlayer) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisVideoPlayer) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisVideoPlayer) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisVideoPlayer) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisVideoPlayer) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisVideoPlayer) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisVideoPlayer) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisVideoPlayer) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisVideoPlayer) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisVideoPlayer) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisVideoPlayer) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisVideoPlayer) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisVideoPlayer) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisVideoPlayer) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisVideoPlayer) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisVideoPlayer) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisVideoPlayer) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisVideoPlayer) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisVideoPlayer) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisVideoPlayer) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisVideoPlayer) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisVideoPlayer) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisVideoPlayer) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisVideoPlayer) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisVideoPlayer) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisVideoPlayer) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisVideoPlayer) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisVideoPlayer) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisVideoPlayer) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisVideoPlayer) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisVideoPlayer) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisVideoPlayer) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisVideoPlayer) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisVideoPlayer) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisVideoPlayer) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisVideoPlayer) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisVideoPlayer) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisVideoPlayer) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisVideoPlayer) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisVideoPlayer) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisVideoPlayer) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisVideoPlayer) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisVideoPlayer) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisVideoPlayer) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisVideoPlayer) SetScale(v float64) { c.Set("scale", v) }

func (c aylisVideoPlayer) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisVideoPlayer) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisVideoPlayer) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisVideoPlayer) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisVideoPlayer) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisVideoPlayer) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisVideoPlayer) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisVideoPlayer) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisVideoPlayer) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisVideoPlayer) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisVideoPlayer) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisVideoPlayer) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisVideoPlayer) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisVideoPlayer) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisVideoPlayer) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisVideoPlayer) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisVideoPlayer) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisVideoPlayer) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisVideoPlayer) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisVideoPlayer) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisVideoPlayer) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisVideoPlayer) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisVideoPlayer) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisVideoPlayer) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisVideoPlayer) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisVideoPlayer) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisVideoPlayer) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisVideoPlayer) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisVideoPlayer) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisVideoPlayer) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisVideoPlayer) Style() VideoPlayerStyle { return VideoPlayerStyle(asText(c.Get("style"))) }
func (c aylisVideoPlayer) SetStyle(v VideoPlayerStyle) { c.Set("style", string(v)) }

func (c aylisVideoPlayer) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisVideoPlayer) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisVideoPlayer) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisVideoPlayer) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisVideoPlayer) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisVideoPlayer) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisVideoPlayer) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisVideoPlayer) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisVideoPlayer) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisVideoPlayer) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisVideoPlayer) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisVideoPlayer) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisVideoPlayer) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisVideoPlayer) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisVideoPlayer) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisVideoPlayer) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisVideoPlayer) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisVideoPlayer) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisVideoPlayer) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisVideoPlayer) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisVideoPlayer) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisVideoPlayer) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisVideoPlayer) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisVideoPlayer) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisVideoPlayer) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisVideoPlayer) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisVideoPlayer) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisVideoPlayer) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisVideoPlayer) Hint() string { return asText(c.Get("hint")) }
func (c aylisVideoPlayer) SetHint(v string) { c.Set("hint", v) }

func (c aylisVideoPlayer) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisVideoPlayer) SetTag(v int64) { c.Set("tag", v) }

func (c aylisVideoPlayer) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisVideoPlayer) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisVideoPlayer) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisVideoPlayer) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisVideoPlayer) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisVideoPlayer) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisVideoPlayer) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisVideoPlayer) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisVideoPlayer) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisVideoPlayer) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisVideoPlayer) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisVideoPlayer) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisVideoPlayer) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisVideoPlayer) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisVideoPlayer) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisVideoPlayer) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisVideoPlayer) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisVideoPlayer) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisVideoPlayer) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisVideoPlayer) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisVideoPlayer) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisVideoPlayer) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisVideoPlayer) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisVideoPlayer) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisVideoPlayer) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisVideoPlayer) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisVideoPlayer) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisVideoPlayer) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisVideoPlayer) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisVideoPlayer) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisVideoPlayer) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisVideoPlayer) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisVideoPlayer) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisVideoPlayer) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisVideoPlayer) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisVideoPlayer) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisVideoPlayer) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisVideoPlayer) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisVideoPlayer) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisVideoPlayer) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisVideoPlayer) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisVideoPlayer) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisVideoPlayer) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisVideoPlayer) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisVideoPlayer) Play() { c.App.Send("media", aylis.M{"target": c.ID, "op": "play"}) }
func (c aylisVideoPlayer) Pause() { c.App.Send("media", aylis.M{"target": c.ID, "op": "pause"}) }
func (c aylisVideoPlayer) Stop() { c.App.Send("media", aylis.M{"target": c.ID, "op": "stop"}) }
func (c aylisVideoPlayer) Seek(seconds float64) { c.App.Send("media", aylis.M{"target": c.ID, "op": "seek", "seconds": seconds}) }
func (c aylisVideoPlayer) Position() float64 { return asNumber(askCmd(c.App, "media", aylis.M{"target": c.ID, "op": "position"})) }
func (c aylisVideoPlayer) Duration() float64 { return asNumber(askCmd(c.App, "media", aylis.M{"target": c.ID, "op": "duration"})) }

// frmPlayerComponents: o form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmPlayerComponents struct {
	aylisForm
	mnuPrincipal aylisMainMenu
	mnuArquivo aylisMenuItem
	mnuAbrir aylisMenuItem
	mnuLimparFila aylisMenuItem
	mnuSep1 aylisMenuItem
	mnuSair aylisMenuItem
	mnuReproduzir aylisMenuItem
	mnuReproduzirPausar aylisMenuItem
	mnuParar aylisMenuItem
	mnuAnterior aylisMenuItem
	mnuProxima aylisMenuItem
	mnuSep2 aylisMenuItem
	mnuEmbaralhar aylisMenuItem
	mnuRepetir aylisMenuItem
	mnuExibir aylisMenuItem
	mnuAgora aylisMenuItem
	mnuFila aylisMenuItem
	pnlControles aylisPanel
	lytBarra aylisLayout
	lblTempo aylisLabel
	lblDuracao aylisLabel
	sbPosicao aylisSeekBar
	lytInfo aylisLayout
	icoCapa aylisIcon
	lytTextos aylisLayout
	lblTitulo aylisLabel
	lblDetalhe aylisLabel
	lytVolume aylisLayout
	btnMudo aylisButton
	tbVolume aylisTrackBar
	lytBotoes aylisLayout
	lytBotoesCentro aylisLayout
	btnEmbaralhar aylisButton
	btnAnterior aylisButton
	btnReproduzir aylisButton
	btnProxima aylisButton
	btnRepetir aylisButton
	lytNavegacao aylisLayout
	btnAgora aylisButton
	btnFila aylisButton
	divNavegacao aylisDivider
	btnAbrirArquivos aylisButton
	pnlAgora aylisPanel
	vpVideo aylisVideoPlayer
	pnlFila aylisPanel
	lblFila aylisLabel
	lytFilaBotoes aylisLayout
	btnAdicionar aylisButton
	btnLimparFila aylisButton
	lvFila aylisListView
	dlgAbrir aylisOpenDialog
	tmrPosicao aylisTimer
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmPlayerComponents) Bind(list []any) error {
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
	if c, err := f.Resolve(list, "mnuAbrir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAbrir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuLimparFila", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuLimparFila = aylisMenuItem{c}
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
	if c, err := f.Resolve(list, "mnuReproduzir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReproduzir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuReproduzirPausar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuReproduzirPausar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuParar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuParar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAnterior", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAnterior = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuProxima", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuProxima = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuSep2", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuSep2 = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuEmbaralhar", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuEmbaralhar = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuRepetir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuRepetir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuExibir", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuExibir = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuAgora", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuAgora = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "mnuFila", "MenuItem"); err != nil {
		return err
	} else {
		f.mnuFila = aylisMenuItem{c}
	}
	if c, err := f.Resolve(list, "pnlControles", "Panel"); err != nil {
		return err
	} else {
		f.pnlControles = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lytBarra", "Layout"); err != nil {
		return err
	} else {
		f.lytBarra = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTempo", "Label"); err != nil {
		return err
	} else {
		f.lblTempo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblDuracao", "Label"); err != nil {
		return err
	} else {
		f.lblDuracao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "sbPosicao", "SeekBar"); err != nil {
		return err
	} else {
		f.sbPosicao = aylisSeekBar{c}
	}
	if c, err := f.Resolve(list, "lytInfo", "Layout"); err != nil {
		return err
	} else {
		f.lytInfo = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "icoCapa", "Icon"); err != nil {
		return err
	} else {
		f.icoCapa = aylisIcon{c}
	}
	if c, err := f.Resolve(list, "lytTextos", "Layout"); err != nil {
		return err
	} else {
		f.lytTextos = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblTitulo", "Label"); err != nil {
		return err
	} else {
		f.lblTitulo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblDetalhe", "Label"); err != nil {
		return err
	} else {
		f.lblDetalhe = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytVolume", "Layout"); err != nil {
		return err
	} else {
		f.lytVolume = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnMudo", "Button"); err != nil {
		return err
	} else {
		f.btnMudo = aylisButton{c}
	}
	if c, err := f.Resolve(list, "tbVolume", "TrackBar"); err != nil {
		return err
	} else {
		f.tbVolume = aylisTrackBar{c}
	}
	if c, err := f.Resolve(list, "lytBotoes", "Layout"); err != nil {
		return err
	} else {
		f.lytBotoes = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lytBotoesCentro", "Layout"); err != nil {
		return err
	} else {
		f.lytBotoesCentro = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnEmbaralhar", "Button"); err != nil {
		return err
	} else {
		f.btnEmbaralhar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnAnterior", "Button"); err != nil {
		return err
	} else {
		f.btnAnterior = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnReproduzir", "Button"); err != nil {
		return err
	} else {
		f.btnReproduzir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnProxima", "Button"); err != nil {
		return err
	} else {
		f.btnProxima = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnRepetir", "Button"); err != nil {
		return err
	} else {
		f.btnRepetir = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lytNavegacao", "Layout"); err != nil {
		return err
	} else {
		f.lytNavegacao = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnAgora", "Button"); err != nil {
		return err
	} else {
		f.btnAgora = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnFila", "Button"); err != nil {
		return err
	} else {
		f.btnFila = aylisButton{c}
	}
	if c, err := f.Resolve(list, "divNavegacao", "Divider"); err != nil {
		return err
	} else {
		f.divNavegacao = aylisDivider{c}
	}
	if c, err := f.Resolve(list, "btnAbrirArquivos", "Button"); err != nil {
		return err
	} else {
		f.btnAbrirArquivos = aylisButton{c}
	}
	if c, err := f.Resolve(list, "pnlAgora", "Panel"); err != nil {
		return err
	} else {
		f.pnlAgora = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "vpVideo", "VideoPlayer"); err != nil {
		return err
	} else {
		f.vpVideo = aylisVideoPlayer{c}
	}
	if c, err := f.Resolve(list, "pnlFila", "Panel"); err != nil {
		return err
	} else {
		f.pnlFila = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "lblFila", "Label"); err != nil {
		return err
	} else {
		f.lblFila = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytFilaBotoes", "Layout"); err != nil {
		return err
	} else {
		f.lytFilaBotoes = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "btnAdicionar", "Button"); err != nil {
		return err
	} else {
		f.btnAdicionar = aylisButton{c}
	}
	if c, err := f.Resolve(list, "btnLimparFila", "Button"); err != nil {
		return err
	} else {
		f.btnLimparFila = aylisButton{c}
	}
	if c, err := f.Resolve(list, "lvFila", "ListView"); err != nil {
		return err
	} else {
		f.lvFila = aylisListView{c}
	}
	if c, err := f.Resolve(list, "dlgAbrir", "OpenDialog"); err != nil {
		return err
	} else {
		f.dlgAbrir = aylisOpenDialog{c}
	}
	if c, err := f.Resolve(list, "tmrPosicao", "Timer"); err != nil {
		return err
	} else {
		f.tmrPosicao = aylisTimer{c}
	}
	return nil
}

// frmPlayer é a janela de frmPlayer cujo evento está rodando (a primeira, fora dos handlers).
var frmPlayer *frmPlayerComponents

func init() {
	aylis.RegisterFormClass("frmPlayer", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmPlayerComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmPlayer = form.(*frmPlayerComponents) })
}

// FindButton é o Button deste form com o nome.
func (f *frmPlayerComponents) FindButton(name string) aylisButton {
	switch name {
	case "btnMudo":
		return f.btnMudo
	case "btnEmbaralhar":
		return f.btnEmbaralhar
	case "btnAnterior":
		return f.btnAnterior
	case "btnReproduzir":
		return f.btnReproduzir
	case "btnProxima":
		return f.btnProxima
	case "btnRepetir":
		return f.btnRepetir
	case "btnAgora":
		return f.btnAgora
	case "btnFila":
		return f.btnFila
	case "btnAbrirArquivos":
		return f.btnAbrirArquivos
	case "btnAdicionar":
		return f.btnAdicionar
	case "btnLimparFila":
		return f.btnLimparFila
	}
	return aylisButton{}
}

// SenderAsButton é o Button que disparou o evento.
func (f *frmPlayerComponents) SenderAsButton(ev aylis.Event) aylisButton { return f.FindButton(ev.Sender.Name) }

// FindDivider é o Divider deste form com o nome.
func (f *frmPlayerComponents) FindDivider(name string) aylisDivider {
	switch name {
	case "divNavegacao":
		return f.divNavegacao
	}
	return aylisDivider{}
}

// SenderAsDivider é o Divider que disparou o evento.
func (f *frmPlayerComponents) SenderAsDivider(ev aylis.Event) aylisDivider { return f.FindDivider(ev.Sender.Name) }

// FindIcon é o Icon deste form com o nome.
func (f *frmPlayerComponents) FindIcon(name string) aylisIcon {
	switch name {
	case "icoCapa":
		return f.icoCapa
	}
	return aylisIcon{}
}

// SenderAsIcon é o Icon que disparou o evento.
func (f *frmPlayerComponents) SenderAsIcon(ev aylis.Event) aylisIcon { return f.FindIcon(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome.
func (f *frmPlayerComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblTempo":
		return f.lblTempo
	case "lblDuracao":
		return f.lblDuracao
	case "lblTitulo":
		return f.lblTitulo
	case "lblDetalhe":
		return f.lblDetalhe
	case "lblFila":
		return f.lblFila
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmPlayerComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome.
func (f *frmPlayerComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytBarra":
		return f.lytBarra
	case "lytInfo":
		return f.lytInfo
	case "lytTextos":
		return f.lytTextos
	case "lytVolume":
		return f.lytVolume
	case "lytBotoes":
		return f.lytBotoes
	case "lytBotoesCentro":
		return f.lytBotoesCentro
	case "lytNavegacao":
		return f.lytNavegacao
	case "lytFilaBotoes":
		return f.lytFilaBotoes
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmPlayerComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindListView é o ListView deste form com o nome.
func (f *frmPlayerComponents) FindListView(name string) aylisListView {
	switch name {
	case "lvFila":
		return f.lvFila
	}
	return aylisListView{}
}

// SenderAsListView é o ListView que disparou o evento.
func (f *frmPlayerComponents) SenderAsListView(ev aylis.Event) aylisListView { return f.FindListView(ev.Sender.Name) }

// FindMainMenu é o MainMenu deste form com o nome.
func (f *frmPlayerComponents) FindMainMenu(name string) aylisMainMenu {
	switch name {
	case "mnuPrincipal":
		return f.mnuPrincipal
	}
	return aylisMainMenu{}
}

// SenderAsMainMenu é o MainMenu que disparou o evento.
func (f *frmPlayerComponents) SenderAsMainMenu(ev aylis.Event) aylisMainMenu { return f.FindMainMenu(ev.Sender.Name) }

// FindMenuItem é o MenuItem deste form com o nome.
func (f *frmPlayerComponents) FindMenuItem(name string) aylisMenuItem {
	switch name {
	case "mnuArquivo":
		return f.mnuArquivo
	case "mnuAbrir":
		return f.mnuAbrir
	case "mnuLimparFila":
		return f.mnuLimparFila
	case "mnuSep1":
		return f.mnuSep1
	case "mnuSair":
		return f.mnuSair
	case "mnuReproduzir":
		return f.mnuReproduzir
	case "mnuReproduzirPausar":
		return f.mnuReproduzirPausar
	case "mnuParar":
		return f.mnuParar
	case "mnuAnterior":
		return f.mnuAnterior
	case "mnuProxima":
		return f.mnuProxima
	case "mnuSep2":
		return f.mnuSep2
	case "mnuEmbaralhar":
		return f.mnuEmbaralhar
	case "mnuRepetir":
		return f.mnuRepetir
	case "mnuExibir":
		return f.mnuExibir
	case "mnuAgora":
		return f.mnuAgora
	case "mnuFila":
		return f.mnuFila
	}
	return aylisMenuItem{}
}

// SenderAsMenuItem é o MenuItem que disparou o evento.
func (f *frmPlayerComponents) SenderAsMenuItem(ev aylis.Event) aylisMenuItem { return f.FindMenuItem(ev.Sender.Name) }

// FindOpenDialog é o OpenDialog deste form com o nome.
func (f *frmPlayerComponents) FindOpenDialog(name string) aylisOpenDialog {
	switch name {
	case "dlgAbrir":
		return f.dlgAbrir
	}
	return aylisOpenDialog{}
}

// SenderAsOpenDialog é o OpenDialog que disparou o evento.
func (f *frmPlayerComponents) SenderAsOpenDialog(ev aylis.Event) aylisOpenDialog { return f.FindOpenDialog(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome.
func (f *frmPlayerComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlControles":
		return f.pnlControles
	case "pnlAgora":
		return f.pnlAgora
	case "pnlFila":
		return f.pnlFila
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmPlayerComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSeekBar é o SeekBar deste form com o nome.
func (f *frmPlayerComponents) FindSeekBar(name string) aylisSeekBar {
	switch name {
	case "sbPosicao":
		return f.sbPosicao
	}
	return aylisSeekBar{}
}

// SenderAsSeekBar é o SeekBar que disparou o evento.
func (f *frmPlayerComponents) SenderAsSeekBar(ev aylis.Event) aylisSeekBar { return f.FindSeekBar(ev.Sender.Name) }

// FindTimer é o Timer deste form com o nome.
func (f *frmPlayerComponents) FindTimer(name string) aylisTimer {
	switch name {
	case "tmrPosicao":
		return f.tmrPosicao
	}
	return aylisTimer{}
}

// SenderAsTimer é o Timer que disparou o evento.
func (f *frmPlayerComponents) SenderAsTimer(ev aylis.Event) aylisTimer { return f.FindTimer(ev.Sender.Name) }

// FindTrackBar é o TrackBar deste form com o nome.
func (f *frmPlayerComponents) FindTrackBar(name string) aylisTrackBar {
	switch name {
	case "tbVolume":
		return f.tbVolume
	}
	return aylisTrackBar{}
}

// SenderAsTrackBar é o TrackBar que disparou o evento.
func (f *frmPlayerComponents) SenderAsTrackBar(ev aylis.Event) aylisTrackBar { return f.FindTrackBar(ev.Sender.Name) }

// FindVideoPlayer é o VideoPlayer deste form com o nome.
func (f *frmPlayerComponents) FindVideoPlayer(name string) aylisVideoPlayer {
	switch name {
	case "vpVideo":
		return f.vpVideo
	}
	return aylisVideoPlayer{}
}

// SenderAsVideoPlayer é o VideoPlayer que disparou o evento.
func (f *frmPlayerComponents) SenderAsVideoPlayer(ev aylis.Event) aylisVideoPlayer { return f.FindVideoPlayer(ev.Sender.Name) }
