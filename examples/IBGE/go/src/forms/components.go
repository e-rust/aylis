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

// ApplyMode são os valores de um enum do acesso tipado.
type ApplyMode string

const (
	ApplyModeCached ApplyMode = "cached"
	ApplyModeOnPost ApplyMode = "on_post"
)

func enumListApplyMode(v any) []ApplyMode {
	var out []ApplyMode
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ApplyMode(asText(x)))
		}
	}
	return out
}

func enumNamesApplyMode(vs []ApplyMode) []string {
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

// CharCase são os valores de um enum do acesso tipado.
type CharCase string

const (
	CharCaseNormal CharCase = "normal"
	CharCaseUpper CharCase = "upper"
	CharCaseLower CharCase = "lower"
)

func enumListCharCase(v any) []CharCase {
	var out []CharCase
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, CharCase(asText(x)))
		}
	}
	return out
}

func enumNamesCharCase(vs []CharCase) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ChartKind são os valores de um enum do acesso tipado.
type ChartKind string

const (
	ChartKindLine ChartKind = "line"
	ChartKindBar ChartKind = "bar"
	ChartKindArea ChartKind = "area"
	ChartKindPie ChartKind = "pie"
	ChartKindDonut ChartKind = "donut"
	ChartKindSparkline ChartKind = "sparkline"
	ChartKindHeatmap ChartKind = "heatmap"
	ChartKindScatter ChartKind = "scatter"
	ChartKindStackedBar ChartKind = "stacked_bar"
	ChartKindWaterfall ChartKind = "waterfall"
	ChartKindFunnel ChartKind = "funnel"
	ChartKindTreemap ChartKind = "treemap"
	ChartKindRadar ChartKind = "radar"
	ChartKindHorizontalBar ChartKind = "horizontal_bar"
	ChartKindBubble ChartKind = "bubble"
	ChartKindPolarArea ChartKind = "polar_area"
	ChartKindStackedLine ChartKind = "stacked_line"
	ChartKindStackedArea ChartKind = "stacked_area"
	ChartKindMixed ChartKind = "mixed"
	ChartKindSankey ChartKind = "sankey"
	ChartKindWaffle ChartKind = "waffle"
)

func enumListChartKind(v any) []ChartKind {
	var out []ChartKind
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ChartKind(asText(x)))
		}
	}
	return out
}

func enumNamesChartKind(vs []ChartKind) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ChartSeriesFill são os valores de um enum do acesso tipado.
type ChartSeriesFill string

const (
	ChartSeriesFillNone ChartSeriesFill = "none"
	ChartSeriesFillOrigin ChartSeriesFill = "origin"
	ChartSeriesFillStart ChartSeriesFill = "start"
	ChartSeriesFillEnd ChartSeriesFill = "end"
	ChartSeriesFillSeries ChartSeriesFill = "series"
)

func enumListChartSeriesFill(v any) []ChartSeriesFill {
	var out []ChartSeriesFill
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ChartSeriesFill(asText(x)))
		}
	}
	return out
}

func enumNamesChartSeriesFill(vs []ChartSeriesFill) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ChartSeriesKind são os valores de um enum do acesso tipado.
type ChartSeriesKind string

const (
	ChartSeriesKindLine ChartSeriesKind = "line"
	ChartSeriesKindBar ChartSeriesKind = "bar"
	ChartSeriesKindArea ChartSeriesKind = "area"
	ChartSeriesKindScatter ChartSeriesKind = "scatter"
)

func enumListChartSeriesKind(v any) []ChartSeriesKind {
	var out []ChartSeriesKind
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ChartSeriesKind(asText(x)))
		}
	}
	return out
}

func enumNamesChartSeriesKind(vs []ChartSeriesKind) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// ChartSeriesPoint são os valores de um enum do acesso tipado.
type ChartSeriesPoint string

const (
	ChartSeriesPointCircle ChartSeriesPoint = "circle"
	ChartSeriesPointSquare ChartSeriesPoint = "square"
	ChartSeriesPointTriangle ChartSeriesPoint = "triangle"
	ChartSeriesPointDiamond ChartSeriesPoint = "diamond"
	ChartSeriesPointCross ChartSeriesPoint = "cross"
	ChartSeriesPointNone ChartSeriesPoint = "none"
)

func enumListChartSeriesPoint(v any) []ChartSeriesPoint {
	var out []ChartSeriesPoint
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ChartSeriesPoint(asText(x)))
		}
	}
	return out
}

func enumNamesChartSeriesPoint(vs []ChartSeriesPoint) []string {
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

// DBChartStyle são os valores de um enum do acesso tipado.
type DBChartStyle string

const (
	DBChartStyleDefault DBChartStyle = ""
)

func enumListDBChartStyle(v any) []DBChartStyle {
	var out []DBChartStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, DBChartStyle(asText(x)))
		}
	}
	return out
}

func enumNamesDBChartStyle(vs []DBChartStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// DataGridStyle são os valores de um enum do acesso tipado.
type DataGridStyle string

const (
	DataGridStyleDefault DataGridStyle = ""
)

func enumListDataGridStyle(v any) []DataGridStyle {
	var out []DataGridStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, DataGridStyle(asText(x)))
		}
	}
	return out
}

func enumNamesDataGridStyle(vs []DataGridStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FieldKind são os valores de um enum do acesso tipado.
type FieldKind string

const (
	FieldKindData FieldKind = "data"
	FieldKindCalculated FieldKind = "calculated"
	FieldKindLookup FieldKind = "lookup"
)

func enumListFieldKind(v any) []FieldKind {
	var out []FieldKind
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FieldKind(asText(x)))
		}
	}
	return out
}

func enumNamesFieldKind(vs []FieldKind) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// FieldType são os valores de um enum do acesso tipado.
type FieldType string

const (
	FieldTypeString FieldType = "string"
	FieldTypeMemo FieldType = "memo"
	FieldTypeInteger FieldType = "integer"
	FieldTypeFloat FieldType = "float"
	FieldTypeCurrency FieldType = "currency"
	FieldTypeBoolean FieldType = "boolean"
	FieldTypeDate FieldType = "date"
	FieldTypeTime FieldType = "time"
	FieldTypeDatetime FieldType = "datetime"
	FieldTypeBlob FieldType = "blob"
	FieldTypeAutoInc FieldType = "auto_inc"
	FieldTypeTimestamp FieldType = "timestamp"
)

func enumListFieldType(v any) []FieldType {
	var out []FieldType
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, FieldType(asText(x)))
		}
	}
	return out
}

func enumNamesFieldType(vs []FieldType) []string {
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

// GridTotalFunction são os valores de um enum do acesso tipado.
type GridTotalFunction string

const (
	GridTotalFunctionSum GridTotalFunction = "sum"
	GridTotalFunctionAvg GridTotalFunction = "avg"
	GridTotalFunctionMin GridTotalFunction = "min"
	GridTotalFunctionMax GridTotalFunction = "max"
	GridTotalFunctionCount GridTotalFunction = "count"
)

func enumListGridTotalFunction(v any) []GridTotalFunction {
	var out []GridTotalFunction
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, GridTotalFunction(asText(x)))
		}
	}
	return out
}

func enumNamesGridTotalFunction(vs []GridTotalFunction) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// GroupBoxStyle são os valores de um enum do acesso tipado.
type GroupBoxStyle string

const (
	GroupBoxStyleDefault GroupBoxStyle = ""
)

func enumListGroupBoxStyle(v any) []GroupBoxStyle {
	var out []GroupBoxStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, GroupBoxStyle(asText(x)))
		}
	}
	return out
}

func enumNamesGroupBoxStyle(vs []GroupBoxStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// HorizontalWheel são os valores de um enum do acesso tipado.
type HorizontalWheel string

const (
	HorizontalWheelShift HorizontalWheel = "shift"
	HorizontalWheelAlways HorizontalWheel = "always"
	HorizontalWheelNone HorizontalWheel = "none"
)

func enumListHorizontalWheel(v any) []HorizontalWheel {
	var out []HorizontalWheel
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, HorizontalWheel(asText(x)))
		}
	}
	return out
}

func enumNamesHorizontalWheel(vs []HorizontalWheel) []string {
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

// ImageStyle são os valores de um enum do acesso tipado.
type ImageStyle string

const (
	ImageStyleDefault ImageStyle = ""
)

func enumListImageStyle(v any) []ImageStyle {
	var out []ImageStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ImageStyle(asText(x)))
		}
	}
	return out
}

func enumNamesImageStyle(vs []ImageStyle) []string {
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

// LegendPosition são os valores de um enum do acesso tipado.
type LegendPosition string

const (
	LegendPositionTopRight LegendPosition = "top_right"
	LegendPositionTopLeft LegendPosition = "top_left"
	LegendPositionTop LegendPosition = "top"
	LegendPositionBottom LegendPosition = "bottom"
)

func enumListLegendPosition(v any) []LegendPosition {
	var out []LegendPosition
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, LegendPosition(asText(x)))
		}
	}
	return out
}

func enumNamesLegendPosition(vs []LegendPosition) []string {
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

// Rounding são os valores de um enum do acesso tipado.
type Rounding string

const (
	RoundingHalfUp Rounding = "half_up"
	RoundingHalfEven Rounding = "half_even"
	RoundingTruncate Rounding = "truncate"
	RoundingUp Rounding = "up"
)

func enumListRounding(v any) []Rounding {
	var out []Rounding
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, Rounding(asText(x)))
		}
	}
	return out
}

func enumNamesRounding(vs []Rounding) []string {
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

// SplitterStyle são os valores de um enum do acesso tipado.
type SplitterStyle string

const (
	SplitterStyleDefault SplitterStyle = ""
)

func enumListSplitterStyle(v any) []SplitterStyle {
	var out []SplitterStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, SplitterStyle(asText(x)))
		}
	}
	return out
}

func enumNamesSplitterStyle(vs []SplitterStyle) []string {
	out := make([]string, len(vs))
	for i, v := range vs {
		out[i] = string(v)
	}
	return out
}

// SvgStyle são os valores de um enum do acesso tipado.
type SvgStyle string

const (
	SvgStyleDefault SvgStyle = ""
)

func enumListSvgStyle(v any) []SvgStyle {
	var out []SvgStyle
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, SvgStyle(asText(x)))
		}
	}
	return out
}

func enumNamesSvgStyle(vs []SvgStyle) []string {
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

// TitleAlign são os valores de um enum do acesso tipado.
type TitleAlign string

const (
	TitleAlignLeft TitleAlign = "left"
	TitleAlignCenter TitleAlign = "center"
	TitleAlignRight TitleAlign = "right"
)

func enumListTitleAlign(v any) []TitleAlign {
	var out []TitleAlign
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, TitleAlign(asText(x)))
		}
	}
	return out
}

func enumNamesTitleAlign(vs []TitleAlign) []string {
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

// ValueFormat são os valores de um enum do acesso tipado.
type ValueFormat string

const (
	ValueFormatShort ValueFormat = "short"
	ValueFormatFull ValueFormat = "full"
	ValueFormatPercent ValueFormat = "percent"
)

func enumListValueFormat(v any) []ValueFormat {
	var out []ValueFormat
	if l, ok := v.([]any); ok {
		for _, x := range l {
			out = append(out, ValueFormat(asText(x)))
		}
	}
	return out
}

func enumNamesValueFormat(vs []ValueFormat) []string {
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

// ChartSeries é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type ChartSeries struct {
	Name string
	Values []float64
	X []float64
	Radius []float64
	Kind ChartSeriesKind
	RightAxis bool
	Color aylis.Color
	Width float64
	Dash []float64
	Point ChartSeriesPoint
	Fill ChartSeriesFill
	FillSeries int64
	Hidden bool
}

func recordOfChartSeries(v any) ChartSeries {
	m, _ := v.(map[string]any)
	return ChartSeries{
		Name: asText(m["name"]),
		Values: jsonNumbers(m["values"]),
		X: jsonNumbers(m["x"]),
		Radius: jsonNumbers(m["radius"]),
		Kind: ChartSeriesKind(asText(m["kind"])),
		RightAxis: asBool(m["right_axis"]),
		Color: aylis.ColorFromJSON(m["color"]),
		Width: asNumber(m["width"]),
		Dash: jsonNumbers(m["dash"]),
		Point: ChartSeriesPoint(asText(m["point"])),
		Fill: ChartSeriesFill(asText(m["fill"])),
		FillSeries: int64(asNumber(m["fill_series"])),
		Hidden: asBool(m["hidden"]),
	}
}

func (r ChartSeries) jsonValue() map[string]any {
	return map[string]any{
		"name": r.Name,
		"values": r.Values,
		"x": r.X,
		"radius": r.Radius,
		"kind": string(r.Kind),
		"right_axis": r.RightAxis,
		"color": r.Color.JSON(),
		"width": r.Width,
		"dash": r.Dash,
		"point": string(r.Point),
		"fill": string(r.Fill),
		"fill_series": r.FillSeries,
		"hidden": r.Hidden,
	}
}

func jsonListChartSeries(items []ChartSeries) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfChartSeries(v any) []ChartSeries {
	l, _ := v.([]any)
	out := make([]ChartSeries, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfChartSeries(x))
	}
	return out
}

func recordChartSeries(v any) *ChartSeries {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfChartSeries(j)
	return &r
}

func recordsChartSeries(v any) []ChartSeries {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfChartSeries(j)
}

func recordTextChartSeries(r *ChartSeries) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextChartSeries(items []ChartSeries) string {
	b, _ := json.Marshal(jsonListChartSeries(items))
	return string(b)
}

// ColumnSpec é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type ColumnSpec struct {
	Field string
	Title string
	Width float64
	ReadOnly bool
	Color aylis.Color
	TextColor aylis.Color
}

func recordOfColumnSpec(v any) ColumnSpec {
	m, _ := v.(map[string]any)
	return ColumnSpec{
		Field: asText(m["field"]),
		Title: asText(m["title"]),
		Width: asNumber(m["width"]),
		ReadOnly: asBool(m["read_only"]),
		Color: aylis.ColorFromJSON(m["color"]),
		TextColor: aylis.ColorFromJSON(m["text_color"]),
	}
}

func (r ColumnSpec) jsonValue() map[string]any {
	return map[string]any{
		"field": r.Field,
		"title": r.Title,
		"width": r.Width,
		"read_only": r.ReadOnly,
		"color": r.Color.JSON(),
		"text_color": r.TextColor.JSON(),
	}
}

func jsonListColumnSpec(items []ColumnSpec) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfColumnSpec(v any) []ColumnSpec {
	l, _ := v.([]any)
	out := make([]ColumnSpec, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfColumnSpec(x))
	}
	return out
}

func recordColumnSpec(v any) *ColumnSpec {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfColumnSpec(j)
	return &r
}

func recordsColumnSpec(v any) []ColumnSpec {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfColumnSpec(j)
}

func recordTextColumnSpec(r *ColumnSpec) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextColumnSpec(items []ColumnSpec) string {
	b, _ := json.Marshal(jsonListColumnSpec(items))
	return string(b)
}

// FilterParam é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type FilterParam struct {
	Name string
	Value string
}

func recordOfFilterParam(v any) FilterParam {
	m, _ := v.(map[string]any)
	return FilterParam{
		Name: asText(m["name"]),
		Value: asText(m["value"]),
	}
}

func (r FilterParam) jsonValue() map[string]any {
	return map[string]any{
		"name": r.Name,
		"value": r.Value,
	}
}

func jsonListFilterParam(items []FilterParam) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfFilterParam(v any) []FilterParam {
	l, _ := v.([]any)
	out := make([]FilterParam, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfFilterParam(x))
	}
	return out
}

func recordFilterParam(v any) *FilterParam {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfFilterParam(j)
	return &r
}

func recordsFilterParam(v any) []FilterParam {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfFilterParam(j)
}

func recordTextFilterParam(r *FilterParam) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextFilterParam(items []FilterParam) string {
	b, _ := json.Marshal(jsonListFilterParam(items))
	return string(b)
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

// GridTotal é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type GridTotal struct {
	Field string
	Function GridTotalFunction
}

func recordOfGridTotal(v any) GridTotal {
	m, _ := v.(map[string]any)
	return GridTotal{
		Field: asText(m["field"]),
		Function: GridTotalFunction(asText(m["function"])),
	}
}

func (r GridTotal) jsonValue() map[string]any {
	return map[string]any{
		"field": r.Field,
		"function": string(r.Function),
	}
}

func jsonListGridTotal(items []GridTotal) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfGridTotal(v any) []GridTotal {
	l, _ := v.([]any)
	out := make([]GridTotal, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfGridTotal(x))
	}
	return out
}

func recordGridTotal(v any) *GridTotal {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfGridTotal(j)
	return &r
}

func recordsGridTotal(v any) []GridTotal {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfGridTotal(j)
}

func recordTextGridTotal(r *GridTotal) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextGridTotal(items []GridTotal) string {
	b, _ := json.Marshal(jsonListGridTotal(items))
	return string(b)
}

// IndexDef é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type IndexDef struct {
	Name string
	Fields string
}

func recordOfIndexDef(v any) IndexDef {
	m, _ := v.(map[string]any)
	return IndexDef{
		Name: asText(m["name"]),
		Fields: asText(m["fields"]),
	}
}

func (r IndexDef) jsonValue() map[string]any {
	return map[string]any{
		"name": r.Name,
		"fields": r.Fields,
	}
}

func jsonListIndexDef(items []IndexDef) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfIndexDef(v any) []IndexDef {
	l, _ := v.([]any)
	out := make([]IndexDef, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfIndexDef(x))
	}
	return out
}

func recordIndexDef(v any) *IndexDef {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfIndexDef(j)
	return &r
}

func recordsIndexDef(v any) []IndexDef {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfIndexDef(j)
}

func recordTextIndexDef(r *IndexDef) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextIndexDef(items []IndexDef) string {
	b, _ := json.Marshal(jsonListIndexDef(items))
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

// MasterField é um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
type MasterField struct {
	Field string
	MasterField string
}

func recordOfMasterField(v any) MasterField {
	m, _ := v.(map[string]any)
	return MasterField{
		Field: asText(m["field"]),
		MasterField: asText(m["master_field"]),
	}
}

func (r MasterField) jsonValue() map[string]any {
	return map[string]any{
		"field": r.Field,
		"master_field": r.MasterField,
	}
}

func jsonListMasterField(items []MasterField) []any {
	out := make([]any, len(items))
	for i, r := range items {
		out[i] = r.jsonValue()
	}
	return out
}

func listOfMasterField(v any) []MasterField {
	l, _ := v.([]any)
	out := make([]MasterField, 0, len(l))
	for _, x := range l {
		out = append(out, recordOfMasterField(x))
	}
	return out
}

func recordMasterField(v any) *MasterField {
	var j any
	if json.Unmarshal([]byte(asText(v)), &j) != nil || j == nil {
		return nil
	}
	r := recordOfMasterField(j)
	return &r
}

func recordsMasterField(v any) []MasterField {
	var j any
	_ = json.Unmarshal([]byte(asText(v)), &j)
	return listOfMasterField(j)
}

func recordTextMasterField(r *MasterField) string {
	if r == nil {
		return ""
	}
	b, _ := json.Marshal(r.jsonValue())
	return string(b)
}

func recordsTextMasterField(items []MasterField) string {
	b, _ := json.Marshal(jsonListMasterField(items))
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

// aylisClientDataSet é um ClientDataSet de um form.
type aylisClientDataSet struct{ aylis.Component }

func (c aylisClientDataSet) KeyFields() string { return asText(c.Get("key_fields")) }
func (c aylisClientDataSet) SetKeyFields(v string) { c.Set("key_fields", v) }

func (c aylisClientDataSet) ApplyMode() ApplyMode { return ApplyMode(asText(c.Get("apply_mode"))) }
func (c aylisClientDataSet) SetApplyMode(v ApplyMode) { c.Set("apply_mode", string(v)) }

func (c aylisClientDataSet) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisClientDataSet) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisClientDataSet) Active() bool { return asBool(c.Get("active")) }
func (c aylisClientDataSet) SetActive(v bool) { c.Set("active", v) }

func (c aylisClientDataSet) Filter() string { return asText(c.Get("filter")) }
func (c aylisClientDataSet) SetFilter(v string) { c.Set("filter", v) }

func (c aylisClientDataSet) Filtered() bool { return asBool(c.Get("filtered")) }
func (c aylisClientDataSet) SetFiltered(v bool) { c.Set("filtered", v) }

func (c aylisClientDataSet) IndexFieldNames() string { return asText(c.Get("index_field_names")) }
func (c aylisClientDataSet) SetIndexFieldNames(v string) { c.Set("index_field_names", v) }

func (c aylisClientDataSet) Indexes() []IndexDef { return recordsIndexDef(c.Get("indexes")) }
func (c aylisClientDataSet) SetIndexes(v []IndexDef) { c.Set("indexes", recordsTextIndexDef(v)) }

func (c aylisClientDataSet) IndexName() string { return asText(c.Get("index_name")) }
func (c aylisClientDataSet) SetIndexName(v string) { c.Set("index_name", v) }

func (c aylisClientDataSet) Params() []FilterParam { return recordsFilterParam(c.Get("params")) }
func (c aylisClientDataSet) SetParams(v []FilterParam) { c.Set("params", recordsTextFilterParam(v)) }

func (c aylisClientDataSet) StatusFilter() string { return asText(c.Get("status_filter")) }
func (c aylisClientDataSet) SetStatusFilter(v string) { c.Set("status_filter", v) }

func (c aylisClientDataSet) MasterSource() *aylis.Component { return c.GetRef("master_source") }
func (c aylisClientDataSet) SetMasterSource(v aylis.Ref) { c.SetRef("master_source", v) }

func (c aylisClientDataSet) MasterFields() []MasterField { return recordsMasterField(c.Get("master_fields")) }
func (c aylisClientDataSet) SetMasterFields(v []MasterField) { c.Set("master_fields", recordsTextMasterField(v)) }

func (c aylisClientDataSet) ApplySeparately() bool { return asBool(c.Get("apply_separately")) }
func (c aylisClientDataSet) SetApplySeparately(v bool) { c.Set("apply_separately", v) }

func (c aylisClientDataSet) RecordTotal() int64 { return int64(asNumber(c.Get("record_total"))) }
func (c aylisClientDataSet) SetRecordTotal(v int64) { c.Set("record_total", v) }

func (c aylisClientDataSet) MaxRecords() int64 { return int64(asNumber(c.Get("max_records"))) }
func (c aylisClientDataSet) SetMaxRecords(v int64) { c.Set("max_records", v) }

func (c aylisClientDataSet) PartialNotice() string { return asText(c.Get("partial_notice")) }

func (c aylisClientDataSet) OnMasterChange() string { return asText(c.Get("on_master_change")) }
func (c aylisClientDataSet) SetOnMasterChange(v string) { c.Set("on_master_change", v) }

func (c aylisClientDataSet) AfterScroll() string { return asText(c.Get("after_scroll")) }
func (c aylisClientDataSet) SetAfterScroll(v string) { c.Set("after_scroll", v) }

func (c aylisClientDataSet) AfterPost() string { return asText(c.Get("after_post")) }
func (c aylisClientDataSet) SetAfterPost(v string) { c.Set("after_post", v) }

func (c aylisClientDataSet) OnStateChange() string { return asText(c.Get("on_state_change")) }
func (c aylisClientDataSet) SetOnStateChange(v string) { c.Set("on_state_change", v) }

func (c aylisClientDataSet) OnReconcileError() string { return asText(c.Get("on_reconcile_error")) }
func (c aylisClientDataSet) SetOnReconcileError(v string) { c.Set("on_reconcile_error", v) }

func (c aylisClientDataSet) OnError() string { return asText(c.Get("on_error")) }
func (c aylisClientDataSet) SetOnError(v string) { c.Set("on_error", v) }

func (c aylisClientDataSet) OnFetch() string { return asText(c.Get("on_fetch")) }
func (c aylisClientDataSet) SetOnFetch(v string) { c.Set("on_fetch", v) }

func (c aylisClientDataSet) OnServerQuery() string { return asText(c.Get("on_server_query")) }
func (c aylisClientDataSet) SetOnServerQuery(v string) { c.Set("on_server_query", v) }

// aylisDBChart é um DBChart de um form.
type aylisDBChart struct{ aylis.Component }

func (c aylisDBChart) DataSource() *aylis.Component { return c.GetRef("data_source") }
func (c aylisDBChart) SetDataSource(v aylis.Ref) { c.SetRef("data_source", v) }

func (c aylisDBChart) LabelField() string { return asText(c.Get("label_field")) }
func (c aylisDBChart) SetLabelField(v string) { c.Set("label_field", v) }

func (c aylisDBChart) ValueFields() []string { return asList(c.Get("value_fields")) }
func (c aylisDBChart) SetValueFields(v []string) { c.Set("value_fields", strings.Join(v, ";")) }

func (c aylisDBChart) Kind() ChartKind { return ChartKind(asText(c.Get("kind"))) }
func (c aylisDBChart) SetKind(v ChartKind) { c.Set("kind", string(v)) }

func (c aylisDBChart) Title() string { return asText(c.Get("title")) }
func (c aylisDBChart) SetTitle(v string) { c.Set("title", v) }

func (c aylisDBChart) Labels() []string { return asList(c.Get("labels")) }
func (c aylisDBChart) SetLabels(v []string) { c.Set("labels", strings.Join(v, ";")) }

func (c aylisDBChart) Series() []ChartSeries { return recordsChartSeries(c.Get("series")) }
func (c aylisDBChart) SetSeries(v []ChartSeries) { c.Set("series", recordsTextChartSeries(v)) }

func (c aylisDBChart) Json() string { return asText(c.Get("json")) }
func (c aylisDBChart) SetJson(v string) { c.Set("json", v) }

func (c aylisDBChart) Legend() bool { return asBool(c.Get("legend")) }
func (c aylisDBChart) SetLegend(v bool) { c.Set("legend", v) }

func (c aylisDBChart) Stepped() bool { return asBool(c.Get("stepped")) }
func (c aylisDBChart) SetStepped(v bool) { c.Set("stepped", v) }

func (c aylisDBChart) Tension() float64 { return asNumber(c.Get("tension")) }
func (c aylisDBChart) SetTension(v float64) { c.Set("tension", v) }

func (c aylisDBChart) Animate() bool { return asBool(c.Get("animate")) }
func (c aylisDBChart) SetAnimate(v bool) { c.Set("animate", v) }

func (c aylisDBChart) Zoom() bool { return asBool(c.Get("zoom")) }
func (c aylisDBChart) SetZoom(v bool) { c.Set("zoom", v) }

func (c aylisDBChart) AutoScale() bool { return asBool(c.Get("auto_scale")) }
func (c aylisDBChart) SetAutoScale(v bool) { c.Set("auto_scale", v) }

func (c aylisDBChart) YMin() float64 { return asNumber(c.Get("y_min")) }
func (c aylisDBChart) SetYMin(v float64) { c.Set("y_min", v) }

func (c aylisDBChart) YMax() float64 { return asNumber(c.Get("y_max")) }
func (c aylisDBChart) SetYMax(v float64) { c.Set("y_max", v) }

func (c aylisDBChart) LogScale() bool { return asBool(c.Get("log_scale")) }
func (c aylisDBChart) SetLogScale(v bool) { c.Set("log_scale", v) }

func (c aylisDBChart) Reverse() bool { return asBool(c.Get("reverse")) }
func (c aylisDBChart) SetReverse(v bool) { c.Set("reverse", v) }

func (c aylisDBChart) TimeAxis() bool { return asBool(c.Get("time_axis")) }
func (c aylisDBChart) SetTimeAxis(v bool) { c.Set("time_axis", v) }

func (c aylisDBChart) ClickedSeries() int64 { return int64(asNumber(c.Get("clicked_series"))) }
func (c aylisDBChart) SetClickedSeries(v int64) { c.Set("clicked_series", v) }

func (c aylisDBChart) ClickedIndex() int64 { return int64(asNumber(c.Get("clicked_index"))) }
func (c aylisDBChart) SetClickedIndex(v int64) { c.Set("clicked_index", v) }

func (c aylisDBChart) ValueFormat() ValueFormat { return ValueFormat(asText(c.Get("value_format"))) }
func (c aylisDBChart) SetValueFormat(v ValueFormat) { c.Set("value_format", string(v)) }

func (c aylisDBChart) LabelAngle() float64 { return asNumber(c.Get("label_angle")) }
func (c aylisDBChart) SetLabelAngle(v float64) { c.Set("label_angle", v) }

func (c aylisDBChart) TitleAlign() TitleAlign { return TitleAlign(asText(c.Get("title_align"))) }
func (c aylisDBChart) SetTitleAlign(v TitleAlign) { c.Set("title_align", string(v)) }

func (c aylisDBChart) TitleBold() bool { return asBool(c.Get("title_bold")) }
func (c aylisDBChart) SetTitleBold(v bool) { c.Set("title_bold", v) }

func (c aylisDBChart) AreaOpacity() float64 { return asNumber(c.Get("area_opacity")) }
func (c aylisDBChart) SetAreaOpacity(v float64) { c.Set("area_opacity", v) }

func (c aylisDBChart) LegendPosition() LegendPosition { return LegendPosition(asText(c.Get("legend_position"))) }
func (c aylisDBChart) SetLegendPosition(v LegendPosition) { c.Set("legend_position", string(v)) }

func (c aylisDBChart) Stacked() bool { return asBool(c.Get("stacked")) }
func (c aylisDBChart) SetStacked(v bool) { c.Set("stacked", v) }

func (c aylisDBChart) Colors() string { return asText(c.Get("colors")) }
func (c aylisDBChart) SetColors(v string) { c.Set("colors", v) }

func (c aylisDBChart) CenterText() string { return asText(c.Get("center_text")) }
func (c aylisDBChart) SetCenterText(v string) { c.Set("center_text", v) }

func (c aylisDBChart) SliceLabels() string { return asText(c.Get("slice_labels")) }
func (c aylisDBChart) SetSliceLabels(v string) { c.Set("slice_labels", v) }

func (c aylisDBChart) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDBChart) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDBChart) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDBChart) SetTop(v float64) { c.Set("top", v) }

func (c aylisDBChart) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDBChart) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDBChart) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDBChart) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDBChart) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisDBChart) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisDBChart) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisDBChart) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisDBChart) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisDBChart) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisDBChart) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisDBChart) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisDBChart) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDBChart) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDBChart) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDBChart) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDBChart) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDBChart) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDBChart) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDBChart) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDBChart) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisDBChart) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisDBChart) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisDBChart) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisDBChart) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDBChart) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDBChart) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisDBChart) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisDBChart) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDBChart) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDBChart) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDBChart) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDBChart) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDBChart) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDBChart) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDBChart) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDBChart) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDBChart) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDBChart) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisDBChart) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisDBChart) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDBChart) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDBChart) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDBChart) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDBChart) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisDBChart) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisDBChart) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDBChart) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDBChart) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisDBChart) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisDBChart) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDBChart) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDBChart) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDBChart) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDBChart) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisDBChart) SetScale(v float64) { c.Set("scale", v) }

func (c aylisDBChart) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDBChart) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDBChart) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDBChart) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDBChart) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDBChart) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDBChart) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDBChart) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDBChart) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDBChart) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDBChart) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDBChart) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDBChart) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDBChart) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDBChart) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisDBChart) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisDBChart) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDBChart) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDBChart) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDBChart) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDBChart) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisDBChart) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisDBChart) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisDBChart) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisDBChart) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisDBChart) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisDBChart) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisDBChart) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisDBChart) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisDBChart) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisDBChart) Style() DBChartStyle { return DBChartStyle(asText(c.Get("style"))) }
func (c aylisDBChart) SetStyle(v DBChartStyle) { c.Set("style", string(v)) }

func (c aylisDBChart) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisDBChart) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisDBChart) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisDBChart) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisDBChart) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisDBChart) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisDBChart) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisDBChart) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisDBChart) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisDBChart) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisDBChart) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisDBChart) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisDBChart) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisDBChart) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisDBChart) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisDBChart) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisDBChart) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDBChart) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDBChart) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDBChart) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDBChart) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisDBChart) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisDBChart) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDBChart) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDBChart) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDBChart) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDBChart) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDBChart) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDBChart) Hint() string { return asText(c.Get("hint")) }
func (c aylisDBChart) SetHint(v string) { c.Set("hint", v) }

func (c aylisDBChart) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDBChart) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDBChart) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisDBChart) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisDBChart) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisDBChart) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisDBChart) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisDBChart) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisDBChart) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisDBChart) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisDBChart) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisDBChart) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisDBChart) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisDBChart) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisDBChart) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisDBChart) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisDBChart) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisDBChart) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisDBChart) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisDBChart) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisDBChart) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisDBChart) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisDBChart) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisDBChart) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisDBChart) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisDBChart) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisDBChart) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisDBChart) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisDBChart) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisDBChart) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisDBChart) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisDBChart) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisDBChart) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisDBChart) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisDBChart) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisDBChart) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisDBChart) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisDBChart) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisDBChart) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisDBChart) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisDBChart) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisDBChart) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisDBChart) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisDBChart) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisDBChart) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisDBChart) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisDataGrid é um DataGrid de um form.
type aylisDataGrid struct{ aylis.Component }

func (c aylisDataGrid) Overscroll() Overscroll { return Overscroll(asText(c.Get("overscroll"))) }
func (c aylisDataGrid) SetOverscroll(v Overscroll) { c.Set("overscroll", string(v)) }

func (c aylisDataGrid) DataSource() *aylis.Component { return c.GetRef("data_source") }
func (c aylisDataGrid) SetDataSource(v aylis.Ref) { c.SetRef("data_source", v) }

func (c aylisDataGrid) Columns() []ColumnSpec { return recordsColumnSpec(c.Get("columns")) }
func (c aylisDataGrid) SetColumns(v []ColumnSpec) { c.Set("columns", recordsTextColumnSpec(v)) }

func (c aylisDataGrid) RowHeight() float64 { return asNumber(c.Get("row_height")) }
func (c aylisDataGrid) SetRowHeight(v float64) { c.Set("row_height", v) }

func (c aylisDataGrid) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisDataGrid) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisDataGrid) Sortable() bool { return asBool(c.Get("sortable")) }
func (c aylisDataGrid) SetSortable(v bool) { c.Set("sortable", v) }

func (c aylisDataGrid) ColumnMove() bool { return asBool(c.Get("column_move")) }
func (c aylisDataGrid) SetColumnMove(v bool) { c.Set("column_move", v) }

func (c aylisDataGrid) FixedColumns() int64 { return int64(asNumber(c.Get("fixed_columns"))) }
func (c aylisDataGrid) SetFixedColumns(v int64) { c.Set("fixed_columns", v) }

func (c aylisDataGrid) FilterRow() bool { return asBool(c.Get("filter_row")) }
func (c aylisDataGrid) SetFilterRow(v bool) { c.Set("filter_row", v) }

func (c aylisDataGrid) MultiSelect() bool { return asBool(c.Get("multi_select")) }
func (c aylisDataGrid) SetMultiSelect(v bool) { c.Set("multi_select", v) }

func (c aylisDataGrid) Footer() []GridTotal { return recordsGridTotal(c.Get("footer")) }
func (c aylisDataGrid) SetFooter(v []GridTotal) { c.Set("footer", recordsTextGridTotal(v)) }

func (c aylisDataGrid) IncrementalSearch() bool { return asBool(c.Get("incremental_search")) }
func (c aylisDataGrid) SetIncrementalSearch(v bool) { c.Set("incremental_search", v) }

func (c aylisDataGrid) CursorColor() aylis.Color { return aylis.ColorFromJSON(c.Get("cursor_color")) }
func (c aylisDataGrid) SetCursorColor(v aylis.Color) { c.Set("cursor_color", v.JSON()) }

func (c aylisDataGrid) Indicator() bool { return asBool(c.Get("indicator")) }
func (c aylisDataGrid) SetIndicator(v bool) { c.Set("indicator", v) }

func (c aylisDataGrid) HighlightCurrent() bool { return asBool(c.Get("highlight_current")) }
func (c aylisDataGrid) SetHighlightCurrent(v bool) { c.Set("highlight_current", v) }

func (c aylisDataGrid) HorizontalWheel() HorizontalWheel { return HorizontalWheel(asText(c.Get("horizontal_wheel"))) }
func (c aylisDataGrid) SetHorizontalWheel(v HorizontalWheel) { c.Set("horizontal_wheel", string(v)) }

func (c aylisDataGrid) OnError() string { return asText(c.Get("on_error")) }
func (c aylisDataGrid) SetOnError(v string) { c.Set("on_error", v) }

func (c aylisDataGrid) OnTitleClick() string { return asText(c.Get("on_title_click")) }
func (c aylisDataGrid) SetOnTitleClick(v string) { c.Set("on_title_click", v) }

func (c aylisDataGrid) OnSelectionChange() string { return asText(c.Get("on_selection_change")) }
func (c aylisDataGrid) SetOnSelectionChange(v string) { c.Set("on_selection_change", v) }

func (c aylisDataGrid) OnColumnMoved() string { return asText(c.Get("on_column_moved")) }
func (c aylisDataGrid) SetOnColumnMoved(v string) { c.Set("on_column_moved", v) }

func (c aylisDataGrid) ScrollLeft() float64 { return asNumber(c.Get("scroll_left")) }
func (c aylisDataGrid) SetScrollLeft(v float64) { c.Set("scroll_left", v) }

func (c aylisDataGrid) TopRow() int64 { return int64(asNumber(c.Get("top_row"))) }
func (c aylisDataGrid) SetTopRow(v int64) { c.Set("top_row", v) }

func (c aylisDataGrid) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisDataGrid) SetLeft(v float64) { c.Set("left", v) }

func (c aylisDataGrid) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisDataGrid) SetTop(v float64) { c.Set("top", v) }

func (c aylisDataGrid) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisDataGrid) SetWidth(v float64) { c.Set("width", v) }

func (c aylisDataGrid) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisDataGrid) SetHeight(v float64) { c.Set("height", v) }

func (c aylisDataGrid) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisDataGrid) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisDataGrid) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisDataGrid) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisDataGrid) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisDataGrid) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisDataGrid) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisDataGrid) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisDataGrid) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisDataGrid) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisDataGrid) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDataGrid) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDataGrid) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisDataGrid) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisDataGrid) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisDataGrid) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisDataGrid) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisDataGrid) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisDataGrid) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisDataGrid) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisDataGrid) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisDataGrid) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisDataGrid) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisDataGrid) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisDataGrid) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisDataGrid) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisDataGrid) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisDataGrid) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisDataGrid) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisDataGrid) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisDataGrid) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisDataGrid) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisDataGrid) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisDataGrid) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisDataGrid) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisDataGrid) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisDataGrid) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisDataGrid) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisDataGrid) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisDataGrid) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisDataGrid) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisDataGrid) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisDataGrid) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisDataGrid) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisDataGrid) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisDataGrid) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisDataGrid) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisDataGrid) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisDataGrid) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisDataGrid) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisDataGrid) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisDataGrid) SetScale(v float64) { c.Set("scale", v) }

func (c aylisDataGrid) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisDataGrid) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisDataGrid) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisDataGrid) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisDataGrid) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisDataGrid) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisDataGrid) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisDataGrid) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisDataGrid) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisDataGrid) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisDataGrid) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisDataGrid) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisDataGrid) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisDataGrid) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisDataGrid) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisDataGrid) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisDataGrid) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisDataGrid) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisDataGrid) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisDataGrid) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisDataGrid) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisDataGrid) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisDataGrid) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisDataGrid) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisDataGrid) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisDataGrid) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisDataGrid) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisDataGrid) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisDataGrid) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisDataGrid) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisDataGrid) Style() DataGridStyle { return DataGridStyle(asText(c.Get("style"))) }
func (c aylisDataGrid) SetStyle(v DataGridStyle) { c.Set("style", string(v)) }

func (c aylisDataGrid) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisDataGrid) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisDataGrid) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisDataGrid) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisDataGrid) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisDataGrid) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisDataGrid) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisDataGrid) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisDataGrid) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisDataGrid) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisDataGrid) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisDataGrid) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisDataGrid) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisDataGrid) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisDataGrid) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisDataGrid) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisDataGrid) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisDataGrid) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisDataGrid) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisDataGrid) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisDataGrid) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisDataGrid) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisDataGrid) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisDataGrid) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisDataGrid) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisDataGrid) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisDataGrid) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisDataGrid) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisDataGrid) Hint() string { return asText(c.Get("hint")) }
func (c aylisDataGrid) SetHint(v string) { c.Set("hint", v) }

func (c aylisDataGrid) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisDataGrid) SetTag(v int64) { c.Set("tag", v) }

func (c aylisDataGrid) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisDataGrid) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisDataGrid) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisDataGrid) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisDataGrid) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisDataGrid) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisDataGrid) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisDataGrid) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisDataGrid) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisDataGrid) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisDataGrid) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisDataGrid) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisDataGrid) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisDataGrid) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisDataGrid) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisDataGrid) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisDataGrid) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisDataGrid) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisDataGrid) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisDataGrid) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisDataGrid) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisDataGrid) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisDataGrid) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisDataGrid) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisDataGrid) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisDataGrid) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisDataGrid) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisDataGrid) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisDataGrid) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisDataGrid) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisDataGrid) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisDataGrid) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisDataGrid) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisDataGrid) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisDataGrid) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisDataGrid) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisDataGrid) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisDataGrid) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisDataGrid) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisDataGrid) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisDataGrid) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisDataGrid) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisDataGrid) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisDataGrid) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }
func (c aylisDataGrid) SetFocus() { c.App.Send("focus", aylis.M{"target": c.ID}) }

// aylisDataSource é um DataSource de um form.
type aylisDataSource struct{ aylis.Component }

func (c aylisDataSource) DataSet() *aylis.Component { return c.GetRef("data_set") }
func (c aylisDataSource) SetDataSet(v aylis.Ref) { c.SetRef("data_set", v) }

func (c aylisDataSource) AutoEdit() bool { return asBool(c.Get("auto_edit")) }
func (c aylisDataSource) SetAutoEdit(v bool) { c.Set("auto_edit", v) }

func (c aylisDataSource) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisDataSource) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisDataSource) OnDataChange() string { return asText(c.Get("on_data_change")) }
func (c aylisDataSource) SetOnDataChange(v string) { c.Set("on_data_change", v) }

func (c aylisDataSource) OnStateChange() string { return asText(c.Get("on_state_change")) }
func (c aylisDataSource) SetOnStateChange(v string) { c.Set("on_state_change", v) }

// aylisField é um Field de um form.
type aylisField struct{ aylis.Component }

func (c aylisField) FieldName() string { return asText(c.Get("field_name")) }
func (c aylisField) SetFieldName(v string) { c.Set("field_name", v) }

func (c aylisField) Type() FieldType { return FieldType(asText(c.Get("type"))) }
func (c aylisField) SetType(v FieldType) { c.Set("type", string(v)) }

func (c aylisField) Size() int64 { return int64(asNumber(c.Get("size"))) }
func (c aylisField) SetSize(v int64) { c.Set("size", v) }

func (c aylisField) Scale() int64 { return int64(asNumber(c.Get("scale"))) }
func (c aylisField) SetScale(v int64) { c.Set("scale", v) }

func (c aylisField) Rounding() Rounding { return Rounding(asText(c.Get("rounding"))) }
func (c aylisField) SetRounding(v Rounding) { c.Set("rounding", string(v)) }

func (c aylisField) DisplayLabel() string { return asText(c.Get("display_label")) }
func (c aylisField) SetDisplayLabel(v string) { c.Set("display_label", v) }

func (c aylisField) DisplayWidth() float64 { return asNumber(c.Get("display_width")) }
func (c aylisField) SetDisplayWidth(v float64) { c.Set("display_width", v) }

func (c aylisField) DisplayFormat() string { return asText(c.Get("display_format")) }
func (c aylisField) SetDisplayFormat(v string) { c.Set("display_format", v) }

func (c aylisField) EditFormat() string { return asText(c.Get("edit_format")) }
func (c aylisField) SetEditFormat(v string) { c.Set("edit_format", v) }

func (c aylisField) Alignment() TextAlign { return TextAlign(asText(c.Get("alignment"))) }
func (c aylisField) SetAlignment(v TextAlign) { c.Set("alignment", string(v)) }

func (c aylisField) Required() bool { return asBool(c.Get("required")) }
func (c aylisField) SetRequired(v bool) { c.Set("required", v) }

func (c aylisField) ReadOnly() bool { return asBool(c.Get("read_only")) }
func (c aylisField) SetReadOnly(v bool) { c.Set("read_only", v) }

func (c aylisField) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisField) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisField) Min() float64 { return asNumber(c.Get("min")) }
func (c aylisField) SetMin(v float64) { c.Set("min", v) }

func (c aylisField) Max() float64 { return asNumber(c.Get("max")) }
func (c aylisField) SetMax(v float64) { c.Set("max", v) }

func (c aylisField) DefaultValue() string { return asText(c.Get("default_value")) }
func (c aylisField) SetDefaultValue(v string) { c.Set("default_value", v) }

func (c aylisField) DisplayValues() string { return asText(c.Get("display_values")) }
func (c aylisField) SetDisplayValues(v string) { c.Set("display_values", v) }

func (c aylisField) CharCase() CharCase { return CharCase(asText(c.Get("char_case"))) }
func (c aylisField) SetCharCase(v CharCase) { c.Set("char_case", string(v)) }

func (c aylisField) EditMask() string { return asText(c.Get("edit_mask")) }
func (c aylisField) SetEditMask(v string) { c.Set("edit_mask", v) }

func (c aylisField) Sensitive() bool { return asBool(c.Get("sensitive")) }
func (c aylisField) SetSensitive(v bool) { c.Set("sensitive", v) }

func (c aylisField) Kind() FieldKind { return FieldKind(asText(c.Get("kind"))) }
func (c aylisField) SetKind(v FieldKind) { c.Set("kind", string(v)) }

func (c aylisField) Expression() string { return asText(c.Get("expression")) }
func (c aylisField) SetExpression(v string) { c.Set("expression", v) }

func (c aylisField) LookupDataSet() *aylis.Component { return c.GetRef("lookup_data_set") }
func (c aylisField) SetLookupDataSet(v aylis.Ref) { c.SetRef("lookup_data_set", v) }

func (c aylisField) KeyFields() string { return asText(c.Get("key_fields")) }
func (c aylisField) SetKeyFields(v string) { c.Set("key_fields", v) }

func (c aylisField) LookupKeyFields() string { return asText(c.Get("lookup_key_fields")) }
func (c aylisField) SetLookupKeyFields(v string) { c.Set("lookup_key_fields", v) }

func (c aylisField) LookupResultField() string { return asText(c.Get("lookup_result_field")) }
func (c aylisField) SetLookupResultField(v string) { c.Set("lookup_result_field", v) }

func (c aylisField) OnChange() string { return asText(c.Get("on_change")) }
func (c aylisField) SetOnChange(v string) { c.Set("on_change", v) }

func (c aylisField) OnGetText() string { return asText(c.Get("on_get_text")) }
func (c aylisField) SetOnGetText(v string) { c.Set("on_get_text", v) }

func (c aylisField) OnSetText() string { return asText(c.Get("on_set_text")) }
func (c aylisField) SetOnSetText(v string) { c.Set("on_set_text", v) }

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

// aylisGroupBox é um GroupBox de um form.
type aylisGroupBox struct{ aylis.Component }

func (c aylisGroupBox) Caption() string { return asText(c.Get("caption")) }
func (c aylisGroupBox) SetCaption(v string) { c.Set("caption", v) }

func (c aylisGroupBox) AllowedChars() string { return asText(c.Get("allowed_chars")) }
func (c aylisGroupBox) SetAllowedChars(v string) { c.Set("allowed_chars", v) }

func (c aylisGroupBox) BlockedChars() string { return asText(c.Get("blocked_chars")) }
func (c aylisGroupBox) SetBlockedChars(v string) { c.Set("blocked_chars", v) }

func (c aylisGroupBox) OnlyAllowed() bool { return asBool(c.Get("only_allowed")) }
func (c aylisGroupBox) SetOnlyAllowed(v bool) { c.Set("only_allowed", v) }

func (c aylisGroupBox) CharsOverride() bool { return asBool(c.Get("chars_override")) }
func (c aylisGroupBox) SetCharsOverride(v bool) { c.Set("chars_override", v) }

func (c aylisGroupBox) CharMap() string { return asText(c.Get("char_map")) }
func (c aylisGroupBox) SetCharMap(v string) { c.Set("char_map", v) }

func (c aylisGroupBox) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisGroupBox) SetLeft(v float64) { c.Set("left", v) }

func (c aylisGroupBox) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisGroupBox) SetTop(v float64) { c.Set("top", v) }

func (c aylisGroupBox) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisGroupBox) SetWidth(v float64) { c.Set("width", v) }

func (c aylisGroupBox) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisGroupBox) SetHeight(v float64) { c.Set("height", v) }

func (c aylisGroupBox) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisGroupBox) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisGroupBox) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisGroupBox) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisGroupBox) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisGroupBox) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisGroupBox) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisGroupBox) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisGroupBox) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisGroupBox) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisGroupBox) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisGroupBox) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisGroupBox) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisGroupBox) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisGroupBox) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisGroupBox) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisGroupBox) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisGroupBox) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisGroupBox) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisGroupBox) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisGroupBox) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisGroupBox) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisGroupBox) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisGroupBox) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisGroupBox) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisGroupBox) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisGroupBox) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisGroupBox) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisGroupBox) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisGroupBox) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisGroupBox) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisGroupBox) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisGroupBox) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisGroupBox) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisGroupBox) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisGroupBox) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisGroupBox) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisGroupBox) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisGroupBox) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisGroupBox) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisGroupBox) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisGroupBox) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisGroupBox) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisGroupBox) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisGroupBox) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisGroupBox) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisGroupBox) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisGroupBox) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisGroupBox) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisGroupBox) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisGroupBox) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisGroupBox) SetScale(v float64) { c.Set("scale", v) }

func (c aylisGroupBox) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisGroupBox) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisGroupBox) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisGroupBox) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisGroupBox) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisGroupBox) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisGroupBox) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisGroupBox) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisGroupBox) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisGroupBox) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisGroupBox) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisGroupBox) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisGroupBox) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisGroupBox) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisGroupBox) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisGroupBox) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisGroupBox) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisGroupBox) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisGroupBox) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisGroupBox) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisGroupBox) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisGroupBox) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisGroupBox) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisGroupBox) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisGroupBox) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisGroupBox) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisGroupBox) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisGroupBox) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisGroupBox) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisGroupBox) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisGroupBox) Style() GroupBoxStyle { return GroupBoxStyle(asText(c.Get("style"))) }
func (c aylisGroupBox) SetStyle(v GroupBoxStyle) { c.Set("style", string(v)) }

func (c aylisGroupBox) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisGroupBox) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisGroupBox) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisGroupBox) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisGroupBox) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisGroupBox) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisGroupBox) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisGroupBox) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisGroupBox) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisGroupBox) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisGroupBox) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisGroupBox) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisGroupBox) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisGroupBox) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisGroupBox) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisGroupBox) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisGroupBox) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisGroupBox) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisGroupBox) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisGroupBox) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisGroupBox) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisGroupBox) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisGroupBox) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisGroupBox) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisGroupBox) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisGroupBox) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisGroupBox) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisGroupBox) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisGroupBox) Hint() string { return asText(c.Get("hint")) }
func (c aylisGroupBox) SetHint(v string) { c.Set("hint", v) }

func (c aylisGroupBox) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisGroupBox) SetTag(v int64) { c.Set("tag", v) }

func (c aylisGroupBox) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisGroupBox) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisGroupBox) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisGroupBox) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisGroupBox) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisGroupBox) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisGroupBox) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisGroupBox) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisGroupBox) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisGroupBox) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisGroupBox) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisGroupBox) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisGroupBox) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisGroupBox) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisGroupBox) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisGroupBox) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisGroupBox) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisGroupBox) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisGroupBox) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisGroupBox) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisGroupBox) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisGroupBox) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisGroupBox) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisGroupBox) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisGroupBox) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisGroupBox) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisGroupBox) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisGroupBox) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisGroupBox) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisGroupBox) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisGroupBox) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisGroupBox) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisGroupBox) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisGroupBox) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisGroupBox) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisGroupBox) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisGroupBox) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisGroupBox) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisGroupBox) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisGroupBox) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisGroupBox) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisGroupBox) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisGroupBox) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisGroupBox) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisImage é um Image de um form.
type aylisImage struct{ aylis.Component }

func (c aylisImage) Image() string { return asText(c.Get("image")) }
func (c aylisImage) SetImage(v string) { c.Set("image", v) }

func (c aylisImage) Fit() ImageFit { return ImageFit(asText(c.Get("fit"))) }
func (c aylisImage) SetFit(v ImageFit) { c.Set("fit", string(v)) }

func (c aylisImage) Radius() float64 { return asNumber(c.Get("radius")) }
func (c aylisImage) SetRadius(v float64) { c.Set("radius", v) }

func (c aylisImage) Zoomable() bool { return asBool(c.Get("zoomable")) }
func (c aylisImage) SetZoomable(v bool) { c.Set("zoomable", v) }

func (c aylisImage) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisImage) SetLeft(v float64) { c.Set("left", v) }

func (c aylisImage) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisImage) SetTop(v float64) { c.Set("top", v) }

func (c aylisImage) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisImage) SetWidth(v float64) { c.Set("width", v) }

func (c aylisImage) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisImage) SetHeight(v float64) { c.Set("height", v) }

func (c aylisImage) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisImage) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisImage) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisImage) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisImage) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisImage) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisImage) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisImage) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisImage) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisImage) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisImage) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisImage) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisImage) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisImage) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisImage) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisImage) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisImage) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisImage) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisImage) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisImage) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisImage) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisImage) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisImage) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisImage) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisImage) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisImage) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisImage) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisImage) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisImage) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisImage) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisImage) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisImage) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisImage) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisImage) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisImage) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisImage) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisImage) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisImage) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisImage) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisImage) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisImage) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisImage) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisImage) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisImage) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisImage) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisImage) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisImage) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisImage) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisImage) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisImage) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisImage) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisImage) SetScale(v float64) { c.Set("scale", v) }

func (c aylisImage) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisImage) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisImage) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisImage) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisImage) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisImage) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisImage) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisImage) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisImage) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisImage) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisImage) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisImage) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisImage) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisImage) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisImage) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisImage) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisImage) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisImage) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisImage) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisImage) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisImage) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisImage) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisImage) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisImage) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisImage) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisImage) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisImage) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisImage) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisImage) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisImage) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisImage) Style() ImageStyle { return ImageStyle(asText(c.Get("style"))) }
func (c aylisImage) SetStyle(v ImageStyle) { c.Set("style", string(v)) }

func (c aylisImage) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisImage) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisImage) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisImage) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisImage) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisImage) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisImage) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisImage) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisImage) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisImage) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisImage) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisImage) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisImage) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisImage) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisImage) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisImage) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisImage) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisImage) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisImage) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisImage) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisImage) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisImage) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisImage) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisImage) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisImage) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisImage) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisImage) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisImage) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisImage) Hint() string { return asText(c.Get("hint")) }
func (c aylisImage) SetHint(v string) { c.Set("hint", v) }

func (c aylisImage) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisImage) SetTag(v int64) { c.Set("tag", v) }

func (c aylisImage) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisImage) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisImage) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisImage) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisImage) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisImage) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisImage) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisImage) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisImage) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisImage) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisImage) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisImage) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisImage) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisImage) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisImage) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisImage) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisImage) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisImage) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisImage) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisImage) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisImage) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisImage) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisImage) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisImage) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisImage) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisImage) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisImage) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisImage) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisImage) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisImage) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisImage) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisImage) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisImage) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisImage) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisImage) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisImage) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisImage) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisImage) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisImage) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisImage) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisImage) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisImage) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisImage) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisImage) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

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

// aylisSplitter é um Splitter de um form.
type aylisSplitter struct{ aylis.Component }

func (c aylisSplitter) MinSize() float64 { return asNumber(c.Get("min_size")) }
func (c aylisSplitter) SetMinSize(v float64) { c.Set("min_size", v) }

func (c aylisSplitter) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSplitter) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSplitter) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSplitter) SetTop(v float64) { c.Set("top", v) }

func (c aylisSplitter) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSplitter) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSplitter) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSplitter) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSplitter) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisSplitter) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisSplitter) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisSplitter) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisSplitter) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisSplitter) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisSplitter) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisSplitter) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisSplitter) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSplitter) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSplitter) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSplitter) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSplitter) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSplitter) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSplitter) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSplitter) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSplitter) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisSplitter) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisSplitter) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisSplitter) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisSplitter) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSplitter) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSplitter) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisSplitter) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisSplitter) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSplitter) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSplitter) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSplitter) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSplitter) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSplitter) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSplitter) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSplitter) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSplitter) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSplitter) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSplitter) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisSplitter) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisSplitter) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSplitter) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSplitter) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSplitter) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSplitter) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisSplitter) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisSplitter) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSplitter) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSplitter) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisSplitter) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisSplitter) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSplitter) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSplitter) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSplitter) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSplitter) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisSplitter) SetScale(v float64) { c.Set("scale", v) }

func (c aylisSplitter) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSplitter) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSplitter) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSplitter) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSplitter) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSplitter) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSplitter) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSplitter) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSplitter) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSplitter) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSplitter) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSplitter) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSplitter) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSplitter) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSplitter) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisSplitter) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisSplitter) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSplitter) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSplitter) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSplitter) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSplitter) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisSplitter) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisSplitter) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisSplitter) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisSplitter) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisSplitter) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisSplitter) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisSplitter) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisSplitter) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisSplitter) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisSplitter) Style() SplitterStyle { return SplitterStyle(asText(c.Get("style"))) }
func (c aylisSplitter) SetStyle(v SplitterStyle) { c.Set("style", string(v)) }

func (c aylisSplitter) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisSplitter) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisSplitter) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisSplitter) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisSplitter) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisSplitter) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisSplitter) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisSplitter) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisSplitter) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisSplitter) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisSplitter) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisSplitter) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisSplitter) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisSplitter) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisSplitter) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisSplitter) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisSplitter) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSplitter) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSplitter) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSplitter) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSplitter) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisSplitter) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisSplitter) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSplitter) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSplitter) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSplitter) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSplitter) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSplitter) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSplitter) Hint() string { return asText(c.Get("hint")) }
func (c aylisSplitter) SetHint(v string) { c.Set("hint", v) }

func (c aylisSplitter) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSplitter) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSplitter) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisSplitter) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisSplitter) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisSplitter) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisSplitter) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisSplitter) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisSplitter) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisSplitter) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisSplitter) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisSplitter) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisSplitter) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisSplitter) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisSplitter) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisSplitter) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisSplitter) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisSplitter) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisSplitter) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisSplitter) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisSplitter) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisSplitter) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisSplitter) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisSplitter) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisSplitter) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisSplitter) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisSplitter) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisSplitter) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisSplitter) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisSplitter) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisSplitter) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisSplitter) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisSplitter) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisSplitter) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisSplitter) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisSplitter) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisSplitter) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisSplitter) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisSplitter) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisSplitter) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisSplitter) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisSplitter) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisSplitter) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisSplitter) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisSplitter) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisSplitter) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// aylisSvg é um Svg de um form.
type aylisSvg struct{ aylis.Component }

func (c aylisSvg) Source() string { return asText(c.Get("source")) }
func (c aylisSvg) SetSource(v string) { c.Set("source", v) }

func (c aylisSvg) Color() aylis.Color { return aylis.ColorFromJSON(c.Get("color")) }
func (c aylisSvg) SetColor(v aylis.Color) { c.Set("color", v.JSON()) }

func (c aylisSvg) Fit() ImageFit { return ImageFit(asText(c.Get("fit"))) }
func (c aylisSvg) SetFit(v ImageFit) { c.Set("fit", string(v)) }

func (c aylisSvg) Highlight() []string { return asList(c.Get("highlight")) }
func (c aylisSvg) SetHighlight(v []string) { c.Set("highlight", strings.Join(v, ";")) }

func (c aylisSvg) HighlightColor() aylis.Color { return aylis.ColorFromJSON(c.Get("highlight_color")) }
func (c aylisSvg) SetHighlightColor(v aylis.Color) { c.Set("highlight_color", v.JSON()) }

func (c aylisSvg) ZoomTo() string { return asText(c.Get("zoom_to")) }
func (c aylisSvg) SetZoomTo(v string) { c.Set("zoom_to", v) }

func (c aylisSvg) Left() float64 { return asNumber(c.Get("left")) }
func (c aylisSvg) SetLeft(v float64) { c.Set("left", v) }

func (c aylisSvg) Top() float64 { return asNumber(c.Get("top")) }
func (c aylisSvg) SetTop(v float64) { c.Set("top", v) }

func (c aylisSvg) Width() float64 { return asNumber(c.Get("width")) }
func (c aylisSvg) SetWidth(v float64) { c.Set("width", v) }

func (c aylisSvg) Height() float64 { return asNumber(c.Get("height")) }
func (c aylisSvg) SetHeight(v float64) { c.Set("height", v) }

func (c aylisSvg) Align() Align { return Align(asText(c.Get("align"))) }
func (c aylisSvg) SetAlign(v Align) { c.Set("align", string(v)) }

func (c aylisSvg) Anchors() []Anchor { return enumListAnchor(c.Get("anchors")) }
func (c aylisSvg) SetAnchors(v []Anchor) { c.Set("anchors", enumNamesAnchor(v)) }

func (c aylisSvg) Margins() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("margins")) }
func (c aylisSvg) SetMargins(v aylis.Bounds) { c.Set("margins", v.JSON()) }

func (c aylisSvg) Padding() aylis.Bounds { return aylis.BoundsFromJSON(c.Get("padding")) }
func (c aylisSvg) SetPadding(v aylis.Bounds) { c.Set("padding", v.JSON()) }

func (c aylisSvg) Visible() bool { return asBool(c.Get("visible")) }
func (c aylisSvg) SetVisible(v bool) { c.Set("visible", v) }

func (c aylisSvg) Enabled() bool { return asBool(c.Get("enabled")) }
func (c aylisSvg) SetEnabled(v bool) { c.Set("enabled", v) }

func (c aylisSvg) HitTest() bool { return asBool(c.Get("hit_test")) }
func (c aylisSvg) SetHitTest(v bool) { c.Set("hit_test", v) }

func (c aylisSvg) ClipChildren() bool { return asBool(c.Get("clip_children")) }
func (c aylisSvg) SetClipChildren(v bool) { c.Set("clip_children", v) }

func (c aylisSvg) InheritState() bool { return asBool(c.Get("inherit_state")) }
func (c aylisSvg) SetInheritState(v bool) { c.Set("inherit_state", v) }

func (c aylisSvg) Translate() bool { return asBool(c.Get("translate")) }
func (c aylisSvg) SetTranslate(v bool) { c.Set("translate", v) }

func (c aylisSvg) CapturesWheel() bool { return asBool(c.Get("captures_wheel")) }
func (c aylisSvg) SetCapturesWheel(v bool) { c.Set("captures_wheel", v) }

func (c aylisSvg) CapturesGesture() bool { return asBool(c.Get("captures_gesture")) }
func (c aylisSvg) SetCapturesGesture(v bool) { c.Set("captures_gesture", v) }

func (c aylisSvg) CanFocus() bool { return asBool(c.Get("can_focus")) }
func (c aylisSvg) SetCanFocus(v bool) { c.Set("can_focus", v) }

func (c aylisSvg) Draggable() bool { return asBool(c.Get("draggable")) }
func (c aylisSvg) SetDraggable(v bool) { c.Set("draggable", v) }

func (c aylisSvg) AcceptsDrops() bool { return asBool(c.Get("accepts_drops")) }
func (c aylisSvg) SetAcceptsDrops(v bool) { c.Set("accepts_drops", v) }

func (c aylisSvg) DragData() string { return asText(c.Get("drag_data")) }
func (c aylisSvg) SetDragData(v string) { c.Set("drag_data", v) }

func (c aylisSvg) DragLabel() string { return asText(c.Get("drag_label")) }
func (c aylisSvg) SetDragLabel(v string) { c.Set("drag_label", v) }

func (c aylisSvg) DragImage() *aylis.Component { return c.GetRef("drag_image") }
func (c aylisSvg) SetDragImage(v aylis.Ref) { c.SetRef("drag_image", v) }

func (c aylisSvg) DropTypes() string { return asText(c.Get("drop_types")) }
func (c aylisSvg) SetDropTypes(v string) { c.Set("drop_types", v) }

func (c aylisSvg) TabStop() bool { return asBool(c.Get("tab_stop")) }
func (c aylisSvg) SetTabStop(v bool) { c.Set("tab_stop", v) }

func (c aylisSvg) Cursor() CursorShape { return CursorShape(asText(c.Get("cursor"))) }
func (c aylisSvg) SetCursor(v CursorShape) { c.Set("cursor", string(v)) }

func (c aylisSvg) Opacity() float64 { return asNumber(c.Get("opacity")) }
func (c aylisSvg) SetOpacity(v float64) { c.Set("opacity", v) }

func (c aylisSvg) BlendMode() BlendMode { return BlendMode(asText(c.Get("blend_mode"))) }
func (c aylisSvg) SetBlendMode(v BlendMode) { c.Set("blend_mode", string(v)) }

func (c aylisSvg) CacheLayer() bool { return asBool(c.Get("cache_layer")) }
func (c aylisSvg) SetCacheLayer(v bool) { c.Set("cache_layer", v) }

func (c aylisSvg) Rotation() float64 { return asNumber(c.Get("rotation")) }
func (c aylisSvg) SetRotation(v float64) { c.Set("rotation", v) }

func (c aylisSvg) Scale() float64 { return asNumber(c.Get("scale")) }
func (c aylisSvg) SetScale(v float64) { c.Set("scale", v) }

func (c aylisSvg) TranslateX() float64 { return asNumber(c.Get("translate_x")) }
func (c aylisSvg) SetTranslateX(v float64) { c.Set("translate_x", v) }

func (c aylisSvg) TranslateY() float64 { return asNumber(c.Get("translate_y")) }
func (c aylisSvg) SetTranslateY(v float64) { c.Set("translate_y", v) }

func (c aylisSvg) AspectRatio() float64 { return asNumber(c.Get("aspect_ratio")) }
func (c aylisSvg) SetAspectRatio(v float64) { c.Set("aspect_ratio", v) }

func (c aylisSvg) MaxWidth() float64 { return asNumber(c.Get("max_width")) }
func (c aylisSvg) SetMaxWidth(v float64) { c.Set("max_width", v) }

func (c aylisSvg) MinWidth() float64 { return asNumber(c.Get("min_width")) }
func (c aylisSvg) SetMinWidth(v float64) { c.Set("min_width", v) }

func (c aylisSvg) MinHeight() float64 { return asNumber(c.Get("min_height")) }
func (c aylisSvg) SetMinHeight(v float64) { c.Set("min_height", v) }

func (c aylisSvg) MaxHeight() float64 { return asNumber(c.Get("max_height")) }
func (c aylisSvg) SetMaxHeight(v float64) { c.Set("max_height", v) }

func (c aylisSvg) SelfAlign() SelfAlign { return SelfAlign(asText(c.Get("self_align"))) }
func (c aylisSvg) SetSelfAlign(v SelfAlign) { c.Set("self_align", string(v)) }

func (c aylisSvg) AutoSize() bool { return asBool(c.Get("auto_size")) }
func (c aylisSvg) SetAutoSize(v bool) { c.Set("auto_size", v) }

func (c aylisSvg) ColumnSpan() int64 { return int64(asNumber(c.Get("column_span"))) }
func (c aylisSvg) SetColumnSpan(v int64) { c.Set("column_span", v) }

func (c aylisSvg) GridRow() int64 { return int64(asNumber(c.Get("grid_row"))) }
func (c aylisSvg) SetGridRow(v int64) { c.Set("grid_row", v) }

func (c aylisSvg) GridColumn() int64 { return int64(asNumber(c.Get("grid_column"))) }
func (c aylisSvg) SetGridColumn(v int64) { c.Set("grid_column", v) }

func (c aylisSvg) RowSpan() int64 { return int64(asNumber(c.Get("row_span"))) }
func (c aylisSvg) SetRowSpan(v int64) { c.Set("row_span", v) }

func (c aylisSvg) Grow() float64 { return asNumber(c.Get("grow")) }
func (c aylisSvg) SetGrow(v float64) { c.Set("grow", v) }

func (c aylisSvg) Shadow() *aylis.Shadow { return aylis.ShadowFromJSON(c.Get("shadow")) }
func (c aylisSvg) SetShadow(v *aylis.Shadow) { c.Set("shadow", v.JSON()) }

func (c aylisSvg) Style() SvgStyle { return SvgStyle(asText(c.Get("style"))) }
func (c aylisSvg) SetStyle(v SvgStyle) { c.Set("style", string(v)) }

func (c aylisSvg) Theme() ThemeName { return ThemeName(asText(c.Get("theme"))) }
func (c aylisSvg) SetTheme(v ThemeName) { c.Set("theme", string(v)) }

func (c aylisSvg) Fill() aylis.Color { return aylis.ColorFromJSON(c.Get("fill")) }
func (c aylisSvg) SetFill(v aylis.Color) { c.Set("fill", v.JSON()) }

func (c aylisSvg) BorderColor() aylis.Color { return aylis.ColorFromJSON(c.Get("border_color")) }
func (c aylisSvg) SetBorderColor(v aylis.Color) { c.Set("border_color", v.JSON()) }

func (c aylisSvg) TextColor() aylis.Color { return aylis.ColorFromJSON(c.Get("text_color")) }
func (c aylisSvg) SetTextColor(v aylis.Color) { c.Set("text_color", v.JSON()) }

func (c aylisSvg) AccentColor() aylis.Color { return aylis.ColorFromJSON(c.Get("accent_color")) }
func (c aylisSvg) SetAccentColor(v aylis.Color) { c.Set("accent_color", v.JSON()) }

func (c aylisSvg) SuccessColor() aylis.Color { return aylis.ColorFromJSON(c.Get("success_color")) }
func (c aylisSvg) SetSuccessColor(v aylis.Color) { c.Set("success_color", v.JSON()) }

func (c aylisSvg) WarningColor() aylis.Color { return aylis.ColorFromJSON(c.Get("warning_color")) }
func (c aylisSvg) SetWarningColor(v aylis.Color) { c.Set("warning_color", v.JSON()) }

func (c aylisSvg) DangerColor() aylis.Color { return aylis.ColorFromJSON(c.Get("danger_color")) }
func (c aylisSvg) SetDangerColor(v aylis.Color) { c.Set("danger_color", v.JSON()) }

func (c aylisSvg) FontFamily() string { return asText(c.Get("font_family")) }
func (c aylisSvg) SetFontFamily(v string) { c.Set("font_family", v) }

func (c aylisSvg) FontSize() float64 { return asNumber(c.Get("font_size")) }
func (c aylisSvg) SetFontSize(v float64) { c.Set("font_size", v) }

func (c aylisSvg) FontWeight() FontWeight { return FontWeight(asText(c.Get("font_weight"))) }
func (c aylisSvg) SetFontWeight(v FontWeight) { c.Set("font_weight", string(v)) }

func (c aylisSvg) FontItalic() bool { return asBool(c.Get("font_italic")) }
func (c aylisSvg) SetFontItalic(v bool) { c.Set("font_italic", v) }

func (c aylisSvg) FontUnderline() bool { return asBool(c.Get("font_underline")) }
func (c aylisSvg) SetFontUnderline(v bool) { c.Set("font_underline", v) }

func (c aylisSvg) FontStrikeout() bool { return asBool(c.Get("font_strikeout")) }
func (c aylisSvg) SetFontStrikeout(v bool) { c.Set("font_strikeout", v) }

func (c aylisSvg) Hint() string { return asText(c.Get("hint")) }
func (c aylisSvg) SetHint(v string) { c.Set("hint", v) }

func (c aylisSvg) Tag() int64 { return int64(asNumber(c.Get("tag"))) }
func (c aylisSvg) SetTag(v int64) { c.Set("tag", v) }

func (c aylisSvg) TagString() string { return asText(c.Get("tag_string")) }
func (c aylisSvg) SetTagString(v string) { c.Set("tag_string", v) }

func (c aylisSvg) AccessibleName() string { return asText(c.Get("accessible_name")) }
func (c aylisSvg) SetAccessibleName(v string) { c.Set("accessible_name", v) }

func (c aylisSvg) PopupMenu() *aylis.Component { return c.GetRef("popup_menu") }
func (c aylisSvg) SetPopupMenu(v aylis.Ref) { c.SetRef("popup_menu", v) }

func (c aylisSvg) OnClick() string { return asText(c.Get("on_click")) }
func (c aylisSvg) SetOnClick(v string) { c.Set("on_click", v) }

func (c aylisSvg) OnDblClick() string { return asText(c.Get("on_dbl_click")) }
func (c aylisSvg) SetOnDblClick(v string) { c.Set("on_dbl_click", v) }

func (c aylisSvg) OnLongPress() string { return asText(c.Get("on_long_press")) }
func (c aylisSvg) SetOnLongPress(v string) { c.Set("on_long_press", v) }

func (c aylisSvg) OnPointerDown() string { return asText(c.Get("on_pointer_down")) }
func (c aylisSvg) SetOnPointerDown(v string) { c.Set("on_pointer_down", v) }

func (c aylisSvg) OnPointerUp() string { return asText(c.Get("on_pointer_up")) }
func (c aylisSvg) SetOnPointerUp(v string) { c.Set("on_pointer_up", v) }

func (c aylisSvg) OnPointerMove() string { return asText(c.Get("on_pointer_move")) }
func (c aylisSvg) SetOnPointerMove(v string) { c.Set("on_pointer_move", v) }

func (c aylisSvg) OnPointerEnter() string { return asText(c.Get("on_pointer_enter")) }
func (c aylisSvg) SetOnPointerEnter(v string) { c.Set("on_pointer_enter", v) }

func (c aylisSvg) OnPointerLeave() string { return asText(c.Get("on_pointer_leave")) }
func (c aylisSvg) SetOnPointerLeave(v string) { c.Set("on_pointer_leave", v) }

func (c aylisSvg) OnWheel() string { return asText(c.Get("on_wheel")) }
func (c aylisSvg) SetOnWheel(v string) { c.Set("on_wheel", v) }

func (c aylisSvg) OnGesture() string { return asText(c.Get("on_gesture")) }
func (c aylisSvg) SetOnGesture(v string) { c.Set("on_gesture", v) }

func (c aylisSvg) OnSwipe() string { return asText(c.Get("on_swipe")) }
func (c aylisSvg) SetOnSwipe(v string) { c.Set("on_swipe", v) }

func (c aylisSvg) OnKeyUp() string { return asText(c.Get("on_key_up")) }
func (c aylisSvg) SetOnKeyUp(v string) { c.Set("on_key_up", v) }

func (c aylisSvg) OnEnter() string { return asText(c.Get("on_enter")) }
func (c aylisSvg) SetOnEnter(v string) { c.Set("on_enter", v) }

func (c aylisSvg) OnExit() string { return asText(c.Get("on_exit")) }
func (c aylisSvg) SetOnExit(v string) { c.Set("on_exit", v) }

func (c aylisSvg) OnResize() string { return asText(c.Get("on_resize")) }
func (c aylisSvg) SetOnResize(v string) { c.Set("on_resize", v) }

func (c aylisSvg) OnDragMove() string { return asText(c.Get("on_drag_move")) }
func (c aylisSvg) SetOnDragMove(v string) { c.Set("on_drag_move", v) }

func (c aylisSvg) OnDragLeave() string { return asText(c.Get("on_drag_leave")) }
func (c aylisSvg) SetOnDragLeave(v string) { c.Set("on_drag_leave", v) }

func (c aylisSvg) OnDragDrop() string { return asText(c.Get("on_drag_drop")) }
func (c aylisSvg) SetOnDragDrop(v string) { c.Set("on_drag_drop", v) }

func (c aylisSvg) OnDragEnd() string { return asText(c.Get("on_drag_end")) }
func (c aylisSvg) SetOnDragEnd(v string) { c.Set("on_drag_end", v) }

// frmIBGEComponents: o form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).
type frmIBGEComponents struct {
	aylisForm
	cdsEstados aylisClientDataSet
	cdsEstadosId aylisField
	cdsEstadosSigla aylisField
	cdsEstadosNome aylisField
	cdsEstadosRegiao aylisField
	cdsEstadosPopulacao aylisField
	cdsEstadosArea aylisField
	cdsEstadosDensidade aylisField
	cdsEstadosPib aylisField
	cdsEstadosPibPerCapita aylisField
	cdsEstadosEstimativa aylisField
	cdsEstadosEstimativaAno aylisField
	cdsEstadosMunicipios aylisField
	cdsEstadosBandeira aylisField
	dsEstados aylisDataSource
	cdsMunicipios aylisClientDataSet
	cdsMunicipiosId aylisField
	cdsMunicipiosUfId aylisField
	cdsMunicipiosUf aylisField
	cdsMunicipiosNome aylisField
	cdsMunicipiosMesorregiao aylisField
	cdsMunicipiosMicrorregiao aylisField
	cdsMunicipiosRegiaoIntermediaria aylisField
	cdsMunicipiosRegiaoImediata aylisField
	cdsMunicipiosPopulacao aylisField
	cdsMunicipiosArea aylisField
	cdsMunicipiosDensidade aylisField
	cdsMunicipiosPib aylisField
	cdsMunicipiosPibParticipacao aylisField
	cdsMunicipiosPibPerCapita aylisField
	cdsMunicipiosEstimativa aylisField
	cdsMunicipiosEstimativaAno aylisField
	cdsMunicipiosDistritos aylisField
	cdsMunicipiosBandeira aylisField
	dsMunicipios aylisDataSource
	cdsDistritos aylisClientDataSet
	cdsDistritosId aylisField
	cdsDistritosMunicipioId aylisField
	cdsDistritosNome aylisField
	dsDistritos aylisDataSource
	cdsPopulacao aylisClientDataSet
	cdsPopulacaoAno aylisField
	cdsPopulacaoHabitantes aylisField
	dsPopulacao aylisDataSource
	cdsMaiores aylisClientDataSet
	cdsMaioresNome aylisField
	cdsMaioresHabitantes aylisField
	dsMaiores aylisDataSource
	lblFonte aylisLabel
	lytListas aylisLayout
	gbEstados aylisGroupBox
	grdEstados aylisDataGrid
	gbDistritos aylisGroupBox
	grdDistritos aylisDataGrid
	gbMunicipios aylisGroupBox
	grdMunicipios aylisDataGrid
	splListas aylisSplitter
	lytPainel aylisLayout
	pnlEstado aylisPanel
	svgBrasil aylisSvg
	lytBandeiraEstado aylisLayout
	lblEstado aylisLabel
	lblRegiao aylisLabel
	imgBandeiraEstado aylisImage
	lytNumerosEstado aylisLayout
	dbtEstadoPopulacao aylisLabel
	lblEstadoEstimativa aylisLabel
	dbtEstadoEstimativa aylisLabel
	dbtEstadoArea aylisLabel
	dbtEstadoDensidade aylisLabel
	dbtEstadoPib aylisLabel
	dbtEstadoPibPerCapita aylisLabel
	dbtEstadoMunicipios aylisLabel
	pnlMunicipio aylisPanel
	svgMunicipio aylisSvg
	svgEstado aylisSvg
	lytBandeiraMunicipio aylisLayout
	lblMunicipio aylisLabel
	lblCodigo aylisLabel
	imgBandeiraMunicipio aylisImage
	lblSemBandeira aylisLabel
	lytNumerosMunicipio aylisLayout
	dbtMesorregiao aylisLabel
	dbtMicrorregiao aylisLabel
	dbtRegiaoImediata aylisLabel
	dbtPopulacao aylisLabel
	lblEstimativa aylisLabel
	dbtEstimativa aylisLabel
	dbtArea aylisLabel
	dbtDensidade aylisLabel
	dbtPib aylisLabel
	dbtPibPerCapita aylisLabel
	dbtPibParticipacao aylisLabel
	lytGraficos aylisLayout
	dbcPopulacao aylisDBChart
	dbcMaiores aylisDBChart
	dbcEstados aylisDBChart
}

// Bind monta os componentes da janela (o form que não bate com o código é erro).
func (f *frmIBGEComponents) Bind(list []any) error {
	if c, err := f.Resolve(list, "cdsEstados", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsEstados = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosId", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosSigla", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosSigla = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosRegiao", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosRegiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPopulacao", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPopulacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosArea", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosArea = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosDensidade", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosDensidade = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPib", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPib = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosPibPerCapita", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosPibPerCapita = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosEstimativa", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosEstimativa = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosEstimativaAno", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosEstimativaAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosMunicipios", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosMunicipios = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsEstadosBandeira", "Field"); err != nil {
		return err
	} else {
		f.cdsEstadosBandeira = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsEstados", "DataSource"); err != nil {
		return err
	} else {
		f.dsEstados = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipios", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsMunicipios = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosId", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosUfId", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosUfId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosUf", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosUf = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosMesorregiao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosMesorregiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosMicrorregiao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosMicrorregiao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosRegiaoIntermediaria = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosRegiaoImediata", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosRegiaoImediata = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPopulacao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPopulacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosArea", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosArea = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosDensidade", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosDensidade = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPib", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPib = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPibParticipacao", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPibParticipacao = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosPibPerCapita", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosPibPerCapita = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosEstimativa", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosEstimativa = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosEstimativaAno", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosEstimativaAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosDistritos", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosDistritos = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMunicipiosBandeira", "Field"); err != nil {
		return err
	} else {
		f.cdsMunicipiosBandeira = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsMunicipios", "DataSource"); err != nil {
		return err
	} else {
		f.dsMunicipios = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsDistritos", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsDistritos = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosId", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosMunicipioId", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosMunicipioId = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsDistritosNome", "Field"); err != nil {
		return err
	} else {
		f.cdsDistritosNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsDistritos", "DataSource"); err != nil {
		return err
	} else {
		f.dsDistritos = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacao", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsPopulacao = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacaoAno", "Field"); err != nil {
		return err
	} else {
		f.cdsPopulacaoAno = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsPopulacaoHabitantes", "Field"); err != nil {
		return err
	} else {
		f.cdsPopulacaoHabitantes = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsPopulacao", "DataSource"); err != nil {
		return err
	} else {
		f.dsPopulacao = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "cdsMaiores", "ClientDataSet"); err != nil {
		return err
	} else {
		f.cdsMaiores = aylisClientDataSet{c}
	}
	if c, err := f.Resolve(list, "cdsMaioresNome", "Field"); err != nil {
		return err
	} else {
		f.cdsMaioresNome = aylisField{c}
	}
	if c, err := f.Resolve(list, "cdsMaioresHabitantes", "Field"); err != nil {
		return err
	} else {
		f.cdsMaioresHabitantes = aylisField{c}
	}
	if c, err := f.Resolve(list, "dsMaiores", "DataSource"); err != nil {
		return err
	} else {
		f.dsMaiores = aylisDataSource{c}
	}
	if c, err := f.Resolve(list, "lblFonte", "Label"); err != nil {
		return err
	} else {
		f.lblFonte = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytListas", "Layout"); err != nil {
		return err
	} else {
		f.lytListas = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "gbEstados", "GroupBox"); err != nil {
		return err
	} else {
		f.gbEstados = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdEstados", "DataGrid"); err != nil {
		return err
	} else {
		f.grdEstados = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "gbDistritos", "GroupBox"); err != nil {
		return err
	} else {
		f.gbDistritos = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdDistritos", "DataGrid"); err != nil {
		return err
	} else {
		f.grdDistritos = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "gbMunicipios", "GroupBox"); err != nil {
		return err
	} else {
		f.gbMunicipios = aylisGroupBox{c}
	}
	if c, err := f.Resolve(list, "grdMunicipios", "DataGrid"); err != nil {
		return err
	} else {
		f.grdMunicipios = aylisDataGrid{c}
	}
	if c, err := f.Resolve(list, "splListas", "Splitter"); err != nil {
		return err
	} else {
		f.splListas = aylisSplitter{c}
	}
	if c, err := f.Resolve(list, "lytPainel", "Layout"); err != nil {
		return err
	} else {
		f.lytPainel = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "pnlEstado", "Panel"); err != nil {
		return err
	} else {
		f.pnlEstado = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "svgBrasil", "Svg"); err != nil {
		return err
	} else {
		f.svgBrasil = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "lytBandeiraEstado", "Layout"); err != nil {
		return err
	} else {
		f.lytBandeiraEstado = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblEstado", "Label"); err != nil {
		return err
	} else {
		f.lblEstado = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblRegiao", "Label"); err != nil {
		return err
	} else {
		f.lblRegiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "imgBandeiraEstado", "Image"); err != nil {
		return err
	} else {
		f.imgBandeiraEstado = aylisImage{c}
	}
	if c, err := f.Resolve(list, "lytNumerosEstado", "Layout"); err != nil {
		return err
	} else {
		f.lytNumerosEstado = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPopulacao", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPopulacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblEstadoEstimativa", "Label"); err != nil {
		return err
	} else {
		f.lblEstadoEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoEstimativa", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoArea", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoArea = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoDensidade", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoDensidade = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPib", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPib = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoPibPerCapita", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoPibPerCapita = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstadoMunicipios", "Label"); err != nil {
		return err
	} else {
		f.dbtEstadoMunicipios = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "pnlMunicipio", "Panel"); err != nil {
		return err
	} else {
		f.pnlMunicipio = aylisPanel{c}
	}
	if c, err := f.Resolve(list, "svgMunicipio", "Svg"); err != nil {
		return err
	} else {
		f.svgMunicipio = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "svgEstado", "Svg"); err != nil {
		return err
	} else {
		f.svgEstado = aylisSvg{c}
	}
	if c, err := f.Resolve(list, "lytBandeiraMunicipio", "Layout"); err != nil {
		return err
	} else {
		f.lytBandeiraMunicipio = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "lblMunicipio", "Label"); err != nil {
		return err
	} else {
		f.lblMunicipio = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblCodigo", "Label"); err != nil {
		return err
	} else {
		f.lblCodigo = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "imgBandeiraMunicipio", "Image"); err != nil {
		return err
	} else {
		f.imgBandeiraMunicipio = aylisImage{c}
	}
	if c, err := f.Resolve(list, "lblSemBandeira", "Label"); err != nil {
		return err
	} else {
		f.lblSemBandeira = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytNumerosMunicipio", "Layout"); err != nil {
		return err
	} else {
		f.lytNumerosMunicipio = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbtMesorregiao", "Label"); err != nil {
		return err
	} else {
		f.dbtMesorregiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtMicrorregiao", "Label"); err != nil {
		return err
	} else {
		f.dbtMicrorregiao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtRegiaoImediata", "Label"); err != nil {
		return err
	} else {
		f.dbtRegiaoImediata = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPopulacao", "Label"); err != nil {
		return err
	} else {
		f.dbtPopulacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lblEstimativa", "Label"); err != nil {
		return err
	} else {
		f.lblEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtEstimativa", "Label"); err != nil {
		return err
	} else {
		f.dbtEstimativa = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtArea", "Label"); err != nil {
		return err
	} else {
		f.dbtArea = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtDensidade", "Label"); err != nil {
		return err
	} else {
		f.dbtDensidade = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPib", "Label"); err != nil {
		return err
	} else {
		f.dbtPib = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPibPerCapita", "Label"); err != nil {
		return err
	} else {
		f.dbtPibPerCapita = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "dbtPibParticipacao", "Label"); err != nil {
		return err
	} else {
		f.dbtPibParticipacao = aylisLabel{c}
	}
	if c, err := f.Resolve(list, "lytGraficos", "Layout"); err != nil {
		return err
	} else {
		f.lytGraficos = aylisLayout{c}
	}
	if c, err := f.Resolve(list, "dbcPopulacao", "DBChart"); err != nil {
		return err
	} else {
		f.dbcPopulacao = aylisDBChart{c}
	}
	if c, err := f.Resolve(list, "dbcMaiores", "DBChart"); err != nil {
		return err
	} else {
		f.dbcMaiores = aylisDBChart{c}
	}
	if c, err := f.Resolve(list, "dbcEstados", "DBChart"); err != nil {
		return err
	} else {
		f.dbcEstados = aylisDBChart{c}
	}
	return nil
}

// frmIBGE é a janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers).
var frmIBGE *frmIBGEComponents

func init() {
	aylis.RegisterFormClass("frmIBGE", func(app *aylis.Aylis, id int64, name string) aylis.FormInstance {
		return &frmIBGEComponents{aylisForm: aylisForm{aylis.Form{Component: aylis.Component{App: app, ID: id, Name: name}}}}
	}, func(form aylis.FormInstance) { frmIBGE = form.(*frmIBGEComponents) })
}

// FindClientDataSet é o ClientDataSet deste form com o nome.
func (f *frmIBGEComponents) FindClientDataSet(name string) aylisClientDataSet {
	switch name {
	case "cdsEstados":
		return f.cdsEstados
	case "cdsMunicipios":
		return f.cdsMunicipios
	case "cdsDistritos":
		return f.cdsDistritos
	case "cdsPopulacao":
		return f.cdsPopulacao
	case "cdsMaiores":
		return f.cdsMaiores
	}
	return aylisClientDataSet{}
}

// SenderAsClientDataSet é o ClientDataSet que disparou o evento.
func (f *frmIBGEComponents) SenderAsClientDataSet(ev aylis.Event) aylisClientDataSet { return f.FindClientDataSet(ev.Sender.Name) }

// FindDBChart é o DBChart deste form com o nome.
func (f *frmIBGEComponents) FindDBChart(name string) aylisDBChart {
	switch name {
	case "dbcPopulacao":
		return f.dbcPopulacao
	case "dbcMaiores":
		return f.dbcMaiores
	case "dbcEstados":
		return f.dbcEstados
	}
	return aylisDBChart{}
}

// SenderAsDBChart é o DBChart que disparou o evento.
func (f *frmIBGEComponents) SenderAsDBChart(ev aylis.Event) aylisDBChart { return f.FindDBChart(ev.Sender.Name) }

// FindDataGrid é o DataGrid deste form com o nome.
func (f *frmIBGEComponents) FindDataGrid(name string) aylisDataGrid {
	switch name {
	case "grdEstados":
		return f.grdEstados
	case "grdDistritos":
		return f.grdDistritos
	case "grdMunicipios":
		return f.grdMunicipios
	}
	return aylisDataGrid{}
}

// SenderAsDataGrid é o DataGrid que disparou o evento.
func (f *frmIBGEComponents) SenderAsDataGrid(ev aylis.Event) aylisDataGrid { return f.FindDataGrid(ev.Sender.Name) }

// FindDataSource é o DataSource deste form com o nome.
func (f *frmIBGEComponents) FindDataSource(name string) aylisDataSource {
	switch name {
	case "dsEstados":
		return f.dsEstados
	case "dsMunicipios":
		return f.dsMunicipios
	case "dsDistritos":
		return f.dsDistritos
	case "dsPopulacao":
		return f.dsPopulacao
	case "dsMaiores":
		return f.dsMaiores
	}
	return aylisDataSource{}
}

// SenderAsDataSource é o DataSource que disparou o evento.
func (f *frmIBGEComponents) SenderAsDataSource(ev aylis.Event) aylisDataSource { return f.FindDataSource(ev.Sender.Name) }

// FindField é o Field deste form com o nome.
func (f *frmIBGEComponents) FindField(name string) aylisField {
	switch name {
	case "cdsEstadosId":
		return f.cdsEstadosId
	case "cdsEstadosSigla":
		return f.cdsEstadosSigla
	case "cdsEstadosNome":
		return f.cdsEstadosNome
	case "cdsEstadosRegiao":
		return f.cdsEstadosRegiao
	case "cdsEstadosPopulacao":
		return f.cdsEstadosPopulacao
	case "cdsEstadosArea":
		return f.cdsEstadosArea
	case "cdsEstadosDensidade":
		return f.cdsEstadosDensidade
	case "cdsEstadosPib":
		return f.cdsEstadosPib
	case "cdsEstadosPibPerCapita":
		return f.cdsEstadosPibPerCapita
	case "cdsEstadosEstimativa":
		return f.cdsEstadosEstimativa
	case "cdsEstadosEstimativaAno":
		return f.cdsEstadosEstimativaAno
	case "cdsEstadosMunicipios":
		return f.cdsEstadosMunicipios
	case "cdsEstadosBandeira":
		return f.cdsEstadosBandeira
	case "cdsMunicipiosId":
		return f.cdsMunicipiosId
	case "cdsMunicipiosUfId":
		return f.cdsMunicipiosUfId
	case "cdsMunicipiosUf":
		return f.cdsMunicipiosUf
	case "cdsMunicipiosNome":
		return f.cdsMunicipiosNome
	case "cdsMunicipiosMesorregiao":
		return f.cdsMunicipiosMesorregiao
	case "cdsMunicipiosMicrorregiao":
		return f.cdsMunicipiosMicrorregiao
	case "cdsMunicipiosRegiaoIntermediaria":
		return f.cdsMunicipiosRegiaoIntermediaria
	case "cdsMunicipiosRegiaoImediata":
		return f.cdsMunicipiosRegiaoImediata
	case "cdsMunicipiosPopulacao":
		return f.cdsMunicipiosPopulacao
	case "cdsMunicipiosArea":
		return f.cdsMunicipiosArea
	case "cdsMunicipiosDensidade":
		return f.cdsMunicipiosDensidade
	case "cdsMunicipiosPib":
		return f.cdsMunicipiosPib
	case "cdsMunicipiosPibParticipacao":
		return f.cdsMunicipiosPibParticipacao
	case "cdsMunicipiosPibPerCapita":
		return f.cdsMunicipiosPibPerCapita
	case "cdsMunicipiosEstimativa":
		return f.cdsMunicipiosEstimativa
	case "cdsMunicipiosEstimativaAno":
		return f.cdsMunicipiosEstimativaAno
	case "cdsMunicipiosDistritos":
		return f.cdsMunicipiosDistritos
	case "cdsMunicipiosBandeira":
		return f.cdsMunicipiosBandeira
	case "cdsDistritosId":
		return f.cdsDistritosId
	case "cdsDistritosMunicipioId":
		return f.cdsDistritosMunicipioId
	case "cdsDistritosNome":
		return f.cdsDistritosNome
	case "cdsPopulacaoAno":
		return f.cdsPopulacaoAno
	case "cdsPopulacaoHabitantes":
		return f.cdsPopulacaoHabitantes
	case "cdsMaioresNome":
		return f.cdsMaioresNome
	case "cdsMaioresHabitantes":
		return f.cdsMaioresHabitantes
	}
	return aylisField{}
}

// SenderAsField é o Field que disparou o evento.
func (f *frmIBGEComponents) SenderAsField(ev aylis.Event) aylisField { return f.FindField(ev.Sender.Name) }

// FindGroupBox é o GroupBox deste form com o nome.
func (f *frmIBGEComponents) FindGroupBox(name string) aylisGroupBox {
	switch name {
	case "gbEstados":
		return f.gbEstados
	case "gbDistritos":
		return f.gbDistritos
	case "gbMunicipios":
		return f.gbMunicipios
	}
	return aylisGroupBox{}
}

// SenderAsGroupBox é o GroupBox que disparou o evento.
func (f *frmIBGEComponents) SenderAsGroupBox(ev aylis.Event) aylisGroupBox { return f.FindGroupBox(ev.Sender.Name) }

// FindImage é o Image deste form com o nome.
func (f *frmIBGEComponents) FindImage(name string) aylisImage {
	switch name {
	case "imgBandeiraEstado":
		return f.imgBandeiraEstado
	case "imgBandeiraMunicipio":
		return f.imgBandeiraMunicipio
	}
	return aylisImage{}
}

// SenderAsImage é o Image que disparou o evento.
func (f *frmIBGEComponents) SenderAsImage(ev aylis.Event) aylisImage { return f.FindImage(ev.Sender.Name) }

// FindLabel é o Label deste form com o nome.
func (f *frmIBGEComponents) FindLabel(name string) aylisLabel {
	switch name {
	case "lblFonte":
		return f.lblFonte
	case "lblEstado":
		return f.lblEstado
	case "lblRegiao":
		return f.lblRegiao
	case "dbtEstadoPopulacao":
		return f.dbtEstadoPopulacao
	case "lblEstadoEstimativa":
		return f.lblEstadoEstimativa
	case "dbtEstadoEstimativa":
		return f.dbtEstadoEstimativa
	case "dbtEstadoArea":
		return f.dbtEstadoArea
	case "dbtEstadoDensidade":
		return f.dbtEstadoDensidade
	case "dbtEstadoPib":
		return f.dbtEstadoPib
	case "dbtEstadoPibPerCapita":
		return f.dbtEstadoPibPerCapita
	case "dbtEstadoMunicipios":
		return f.dbtEstadoMunicipios
	case "lblMunicipio":
		return f.lblMunicipio
	case "lblCodigo":
		return f.lblCodigo
	case "lblSemBandeira":
		return f.lblSemBandeira
	case "dbtMesorregiao":
		return f.dbtMesorregiao
	case "dbtMicrorregiao":
		return f.dbtMicrorregiao
	case "dbtRegiaoImediata":
		return f.dbtRegiaoImediata
	case "dbtPopulacao":
		return f.dbtPopulacao
	case "lblEstimativa":
		return f.lblEstimativa
	case "dbtEstimativa":
		return f.dbtEstimativa
	case "dbtArea":
		return f.dbtArea
	case "dbtDensidade":
		return f.dbtDensidade
	case "dbtPib":
		return f.dbtPib
	case "dbtPibPerCapita":
		return f.dbtPibPerCapita
	case "dbtPibParticipacao":
		return f.dbtPibParticipacao
	}
	return aylisLabel{}
}

// SenderAsLabel é o Label que disparou o evento.
func (f *frmIBGEComponents) SenderAsLabel(ev aylis.Event) aylisLabel { return f.FindLabel(ev.Sender.Name) }

// FindLayout é o Layout deste form com o nome.
func (f *frmIBGEComponents) FindLayout(name string) aylisLayout {
	switch name {
	case "lytListas":
		return f.lytListas
	case "lytPainel":
		return f.lytPainel
	case "lytBandeiraEstado":
		return f.lytBandeiraEstado
	case "lytNumerosEstado":
		return f.lytNumerosEstado
	case "lytBandeiraMunicipio":
		return f.lytBandeiraMunicipio
	case "lytNumerosMunicipio":
		return f.lytNumerosMunicipio
	case "lytGraficos":
		return f.lytGraficos
	}
	return aylisLayout{}
}

// SenderAsLayout é o Layout que disparou o evento.
func (f *frmIBGEComponents) SenderAsLayout(ev aylis.Event) aylisLayout { return f.FindLayout(ev.Sender.Name) }

// FindPanel é o Panel deste form com o nome.
func (f *frmIBGEComponents) FindPanel(name string) aylisPanel {
	switch name {
	case "pnlEstado":
		return f.pnlEstado
	case "pnlMunicipio":
		return f.pnlMunicipio
	}
	return aylisPanel{}
}

// SenderAsPanel é o Panel que disparou o evento.
func (f *frmIBGEComponents) SenderAsPanel(ev aylis.Event) aylisPanel { return f.FindPanel(ev.Sender.Name) }

// FindSplitter é o Splitter deste form com o nome.
func (f *frmIBGEComponents) FindSplitter(name string) aylisSplitter {
	switch name {
	case "splListas":
		return f.splListas
	}
	return aylisSplitter{}
}

// SenderAsSplitter é o Splitter que disparou o evento.
func (f *frmIBGEComponents) SenderAsSplitter(ev aylis.Event) aylisSplitter { return f.FindSplitter(ev.Sender.Name) }

// FindSvg é o Svg deste form com o nome.
func (f *frmIBGEComponents) FindSvg(name string) aylisSvg {
	switch name {
	case "svgBrasil":
		return f.svgBrasil
	case "svgMunicipio":
		return f.svgMunicipio
	case "svgEstado":
		return f.svgEstado
	}
	return aylisSvg{}
}

// SenderAsSvg é o Svg que disparou o evento.
func (f *frmIBGEComponents) SenderAsSvg(ev aylis.Event) aylisSvg { return f.FindSvg(ev.Sender.Name) }
