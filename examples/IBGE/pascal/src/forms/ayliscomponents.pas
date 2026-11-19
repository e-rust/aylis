{ Os componentes dos forms com as propriedades tipadas e a classe base de cada form (com os
  componentes como campos published, montados quando a tela abre o form). A classe de cada
  form, com os handlers e o estado, fica na unit dele.
  Gerado pela IDE do Aylis a cada "Gerar código": não edite. }
unit ayliscomponents;

{$mode objfpc}{$H+}

interface

uses
  SysUtils, DateUtils, fpjson, jsonparser, aylis;

type
  TAylisDoubleArray = array of Double;
  TAylisColorArray = array of TAylisColor;
  { O intervalo de datas (o on_change do DateRangePicker); 0: sem a data. }
  TAylisDateRange = record
    StartDate, EndDate: TDateTime;
  end;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TAlign = (None, Top, Bottom, Left, Right, MostTop, MostBottom, MostLeft, MostRight, Client, Contents, Center, VertCenter, HorzCenter, Horizontal, Vertical, Fit);
  {$pop}
  TAlignSet = set of TAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TAnchor = (Left, Top, Right, Bottom);
  {$pop}
  TAnchorSet = set of TAnchor;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TApplyMode = (Cached, OnPost);
  {$pop}
  TApplyModeSet = set of TApplyMode;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TBlendMode = (Normal, Multiply, Screen, Overlay, Darken, Lighten, ColorDodge, ColorBurn, HardLight, SoftLight, Difference, Exclusion, Hue, Saturation, Color, Luminosity);
  {$pop}
  TBlendModeSet = set of TBlendMode;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TBorderStyle = (Sizeable, Single, None, Custom);
  {$pop}
  TBorderStyleSet = set of TBorderStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TCharCase = (Normal, Upper, Lower);
  {$pop}
  TCharCaseSet = set of TCharCase;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TChartKind = (Line, Bar, Area, Pie, Donut, Sparkline, Heatmap, Scatter, StackedBar, Waterfall, Funnel, Treemap, Radar, HorizontalBar, Bubble, PolarArea, StackedLine, StackedArea, Mixed, Sankey, Waffle);
  {$pop}
  TChartKindSet = set of TChartKind;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TChartSeriesFill = (None, Origin, Start, &End, Series);
  {$pop}
  TChartSeriesFillSet = set of TChartSeriesFill;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TChartSeriesKind = (Line, Bar, Area, Scatter);
  {$pop}
  TChartSeriesKindSet = set of TChartSeriesKind;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TChartSeriesPoint = (Circle, Square, Triangle, Diamond, Cross, None);
  {$pop}
  TChartSeriesPointSet = set of TChartSeriesPoint;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TCloseAction = (Hide, Free);
  {$pop}
  TCloseActionSet = set of TCloseAction;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TCursorShape = (Default, Pointer, Text, Wait, Crosshair, Move, NotAllowed, ResizeHorizontal, ResizeVertical, ResizeNeSw, ResizeNwSe, Grab, Grabbing, None);
  {$pop}
  TCursorShapeSet = set of TCursorShape;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TDBChartStyle = (Default);
  {$pop}
  TDBChartStyleSet = set of TDBChartStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TDataGridStyle = (Default);
  {$pop}
  TDataGridStyleSet = set of TDataGridStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFieldKind = (Data, Calculated, Lookup);
  {$pop}
  TFieldKindSet = set of TFieldKind;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFieldType = (&String, Memo, Integer, Float, Currency, Boolean, Date, Time, Datetime, Blob, AutoInc, Timestamp);
  {$pop}
  TFieldTypeSet = set of TFieldType;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFontWeight = (Default, Thin, Light, Normal, Medium, Semibold, Bold, Black);
  {$pop}
  TFontWeightSet = set of TFontWeight;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFormOrientation = (Project, Any, Portrait, Landscape, SensorPortrait, SensorLandscape);
  {$pop}
  TFormOrientationSet = set of TFormOrientation;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFormPosition = (Center, ScreenCenter, Designed, Default, TopLeft, TopRight, BottomLeft, BottomRight);
  {$pop}
  TFormPositionSet = set of TFormPosition;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFormPresent = (Project, Window, Composition, Composed);
  {$pop}
  TFormPresentSet = set of TFormPresent;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TFormStyle = (Default);
  {$pop}
  TFormStyleSet = set of TFormStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TGridTotalFunction = (Sum, Avg, Min, Max, Count);
  {$pop}
  TGridTotalFunctionSet = set of TGridTotalFunction;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TGroupBoxStyle = (Default);
  {$pop}
  TGroupBoxStyleSet = set of TGroupBoxStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  THorizontalWheel = (Shift, Always, None);
  {$pop}
  THorizontalWheelSet = set of THorizontalWheel;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TImageFit = (Contain, Cover, Stretch, Center, Original);
  {$pop}
  TImageFitSet = set of TImageFit;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TImageStyle = (Default);
  {$pop}
  TImageStyleSet = set of TImageStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TLabelStyle = (Default, Caption, Error, Heading, Kbd, Link, Mono, Muted, Title);
  {$pop}
  TLabelStyleSet = set of TLabelStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TLayerCaching = (Project, &On, Off);
  {$pop}
  TLayerCachingSet = set of TLayerCaching;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TLayoutStyle = (Default);
  {$pop}
  TLayoutStyleSet = set of TLayoutStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TLegendPosition = (TopRight, TopLeft, Top, Bottom);
  {$pop}
  TLegendPositionSet = set of TLegendPosition;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TOverscroll = (Auto, Glow, Stretch, Bounce, None);
  {$pop}
  TOverscrollSet = set of TOverscroll;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TPanelStyle = (Default, Appbar, Card, Filled, Flat, Glass, Outlined, Overlay, Reveal, Sidebar);
  {$pop}
  TPanelStyleSet = set of TPanelStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TRounding = (HalfUp, HalfEven, Truncate, Up);
  {$pop}
  TRoundingSet = set of TRounding;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSelfAlign = (Stretch, Start, Center, &End);
  {$pop}
  TSelfAlignSet = set of TSelfAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSplitterStyle = (Default);
  {$pop}
  TSplitterStyleSet = set of TSplitterStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSvgStyle = (Default);
  {$pop}
  TSvgStyleSet = set of TSvgStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTextAlign = (Leading, Center, Trailing, Justify);
  {$pop}
  TTextAlignSet = set of TTextAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTextCase = (None, Upper, Lower);
  {$pop}
  TTextCaseSet = set of TTextCase;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TThemeName = (Default, AylisLight, AylisDark, Material3Light, Material3Dark, Material2Light, Material2Dark, Android7Light, Android7Dark, IOS26Light, IOS26Dark, IOS13Light, IOS13Dark, IOS7Light, WinUI3Light, WinUI3Dark, Windows11Light, Windows11Dark, Windows10Light, Windows10Dark, Win32, Windows7, WindowsPhoneDark, WindowsPhoneLight, Vuetify3Light, Vuetify3Dark, Bootstrap5Light, Bootstrap5Dark, ShadcnUiLight, ShadcnUiDark, AntDesign5Light, AntDesign5Dark, ChakraUILight, ChakraUIDark, Quasar2Light, Quasar2Dark);
  {$pop}
  TThemeNameSet = set of TThemeName;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTitleAlign = (Left, Center, Right);
  {$pop}
  TTitleAlignSet = set of TTitleAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTransition = (None, Fade, SlideLeft, SlideRight, SlideUp, SlideDown);
  {$pop}
  TTransitionSet = set of TTransition;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTrimming = (None, Character, Word);
  {$pop}
  TTrimmingSet = set of TTrimming;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TValueFormat = (Short, Full, Percent);
  {$pop}
  TValueFormatSet = set of TValueFormat;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TWindowCorners = (Default, Round, Small, Square);
  {$pop}
  TWindowCornersSet = set of TWindowCorners;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TWindowShape = (Rectangle, Rounded, Ellipse, Path);
  {$pop}
  TWindowShapeSet = set of TWindowShape;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TWindowState = (Normal, Maximized, Minimized, FullScreen);
  {$pop}
  TWindowStateSet = set of TWindowState;

  PChartSeries = ^TChartSeries;
  PColumnSpec = ^TColumnSpec;
  PFilterParam = ^TFilterParam;
  PGestureEvent = ^TGestureEvent;
  PGridTotal = ^TGridTotal;
  PIndexDef = ^TIndexDef;
  PKeyEvent = ^TKeyEvent;
  PMasterField = ^TMasterField;
  PPointerEvent = ^TPointerEvent;
  PSwipeEvent = ^TSwipeEvent;
  PWheelEvent = ^TWheelEvent;
  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TChartSeries = record
    Name: string;
    Values: TAylisDoubleArray;
    X: TAylisDoubleArray;
    Radius: TAylisDoubleArray;
    Kind: TChartSeriesKind;
    RightAxis: Boolean;
    Color: TAylisColor;
    Width: Double;
    Dash: TAylisDoubleArray;
    Point: TChartSeriesPoint;
    Fill: TChartSeriesFill;
    FillSeries: Int64;
    Hidden: Boolean;
  end;
  TChartSeriesArray = array of TChartSeries;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TColumnSpec = record
    Field: string;
    Title: string;
    Width: Double;
    ReadOnly: Boolean;
    Color: TAylisColor;
    TextColor: TAylisColor;
  end;
  TColumnSpecArray = array of TColumnSpec;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TFilterParam = record
    Name: string;
    Value: string;
  end;
  TFilterParamArray = array of TFilterParam;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TGestureEvent = record
    Phase: string;
    X: Double;
    Y: Double;
    Scale: Double;
    DeltaScale: Double;
    Rotation: Double;
    DeltaRotation: Double;
    PanX: Double;
    PanY: Double;
  end;
  TGestureEventArray = array of TGestureEvent;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TGridTotal = record
    Field: string;
    &Function: TGridTotalFunction;
  end;
  TGridTotalArray = array of TGridTotal;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TIndexDef = record
    Name: string;
    Fields: string;
  end;
  TIndexDefArray = array of TIndexDef;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TKeyEvent = record
    Key: string;
    Ctrl: Boolean;
    Shift: Boolean;
    Alt: Boolean;
  end;
  TKeyEventArray = array of TKeyEvent;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TMasterField = record
    Field: string;
    MasterField: string;
  end;
  TMasterFieldArray = array of TMasterField;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TPointerEvent = record
    X: Double;
    Y: Double;
    Button: string;
    Clicks: Int64;
  end;
  TPointerEventArray = array of TPointerEvent;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TSwipeEvent = record
    Direction: string;
    X: Double;
    Y: Double;
    Dx: Double;
    Dy: Double;
    Vx: Double;
    Vy: Double;
  end;
  TSwipeEventArray = array of TSwipeEvent;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TWheelEvent = record
    X: Double;
    Y: Double;
    Dx: Double;
    Dy: Double;
  end;
  TWheelEventArray = array of TWheelEvent;

  { Um ClientDataSet de um form. }
  TClientDataSet = class(TAylisComponent)
  private
    function GetKeyFields: string;
    procedure SetKeyFields(Value: string);
    function GetApplyMode: TApplyMode;
    procedure SetApplyMode(Value: TApplyMode);
    function GetReadOnly: Boolean;
    procedure SetReadOnly(Value: Boolean);
    function GetActive: Boolean;
    procedure SetActive(Value: Boolean);
    function GetFilter: string;
    procedure SetFilter(Value: string);
    function GetFiltered: Boolean;
    procedure SetFiltered(Value: Boolean);
    function GetIndexFieldNames: string;
    procedure SetIndexFieldNames(Value: string);
    function GetIndexes: TIndexDefArray;
    procedure SetIndexes(Value: TIndexDefArray);
    function GetIndexName: string;
    procedure SetIndexName(Value: string);
    function GetParams: TFilterParamArray;
    procedure SetParams(Value: TFilterParamArray);
    function GetStatusFilter: string;
    procedure SetStatusFilter(Value: string);
    function GetMasterSource: TAylisComponent;
    procedure SetMasterSource(Value: TAylisComponent);
    function GetMasterFields: TMasterFieldArray;
    procedure SetMasterFields(Value: TMasterFieldArray);
    function GetApplySeparately: Boolean;
    procedure SetApplySeparately(Value: Boolean);
    function GetRecordTotal: Int64;
    procedure SetRecordTotal(Value: Int64);
    function GetMaxRecords: Int64;
    procedure SetMaxRecords(Value: Int64);
    function GetPartialNotice: string;
    function GetOnMasterChange: string;
    procedure SetOnMasterChange(Value: string);
    function GetAfterScroll: string;
    procedure SetAfterScroll(Value: string);
    function GetAfterPost: string;
    procedure SetAfterPost(Value: string);
    function GetOnStateChange: string;
    procedure SetOnStateChange(Value: string);
    function GetOnReconcileError: string;
    procedure SetOnReconcileError(Value: string);
    function GetOnError: string;
    procedure SetOnError(Value: string);
    function GetOnFetch: string;
    procedure SetOnFetch(Value: string);
    function GetOnServerQuery: string;
    procedure SetOnServerQuery(Value: string);
  public
    property KeyFields: string read GetKeyFields write SetKeyFields;
    property ApplyMode: TApplyMode read GetApplyMode write SetApplyMode;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Active: Boolean read GetActive write SetActive;
    property Filter: string read GetFilter write SetFilter;
    property Filtered: Boolean read GetFiltered write SetFiltered;
    property IndexFieldNames: string read GetIndexFieldNames write SetIndexFieldNames;
    property Indexes: TIndexDefArray read GetIndexes write SetIndexes;
    property IndexName: string read GetIndexName write SetIndexName;
    property Params: TFilterParamArray read GetParams write SetParams;
    property StatusFilter: string read GetStatusFilter write SetStatusFilter;
    property MasterSource: TAylisComponent read GetMasterSource write SetMasterSource;
    property MasterFields: TMasterFieldArray read GetMasterFields write SetMasterFields;
    property ApplySeparately: Boolean read GetApplySeparately write SetApplySeparately;
    property RecordTotal: Int64 read GetRecordTotal write SetRecordTotal;
    property MaxRecords: Int64 read GetMaxRecords write SetMaxRecords;
    property PartialNotice: string read GetPartialNotice;
    property OnMasterChange: string read GetOnMasterChange write SetOnMasterChange;
    property AfterScroll: string read GetAfterScroll write SetAfterScroll;
    property AfterPost: string read GetAfterPost write SetAfterPost;
    property OnStateChange: string read GetOnStateChange write SetOnStateChange;
    property OnReconcileError: string read GetOnReconcileError write SetOnReconcileError;
    property OnError: string read GetOnError write SetOnError;
    property OnFetch: string read GetOnFetch write SetOnFetch;
    property OnServerQuery: string read GetOnServerQuery write SetOnServerQuery;
  end;

  { Um DBChart de um form. }
  TDBChart = class(TAylisComponent)
  private
    function GetDataSource: TAylisComponent;
    procedure SetDataSource(Value: TAylisComponent);
    function GetLabelField: string;
    procedure SetLabelField(Value: string);
    function GetValueFields: TStringArray;
    procedure SetValueFields(Value: TStringArray);
    function GetKind: TChartKind;
    procedure SetKind(Value: TChartKind);
    function GetTitle: string;
    procedure SetTitle(Value: string);
    function GetLabels: TStringArray;
    procedure SetLabels(Value: TStringArray);
    function GetSeries: TChartSeriesArray;
    procedure SetSeries(Value: TChartSeriesArray);
    function GetJson: string;
    procedure SetJson(Value: string);
    function GetLegend: Boolean;
    procedure SetLegend(Value: Boolean);
    function GetStepped: Boolean;
    procedure SetStepped(Value: Boolean);
    function GetTension: Double;
    procedure SetTension(Value: Double);
    function GetAnimate: Boolean;
    procedure SetAnimate(Value: Boolean);
    function GetZoom: Boolean;
    procedure SetZoom(Value: Boolean);
    function GetAutoScale: Boolean;
    procedure SetAutoScale(Value: Boolean);
    function GetYMin: Double;
    procedure SetYMin(Value: Double);
    function GetYMax: Double;
    procedure SetYMax(Value: Double);
    function GetLogScale: Boolean;
    procedure SetLogScale(Value: Boolean);
    function GetReverse: Boolean;
    procedure SetReverse(Value: Boolean);
    function GetTimeAxis: Boolean;
    procedure SetTimeAxis(Value: Boolean);
    function GetClickedSeries: Int64;
    procedure SetClickedSeries(Value: Int64);
    function GetClickedIndex: Int64;
    procedure SetClickedIndex(Value: Int64);
    function GetValueFormat: TValueFormat;
    procedure SetValueFormat(Value: TValueFormat);
    function GetLabelAngle: Double;
    procedure SetLabelAngle(Value: Double);
    function GetTitleAlign: TTitleAlign;
    procedure SetTitleAlign(Value: TTitleAlign);
    function GetTitleBold: Boolean;
    procedure SetTitleBold(Value: Boolean);
    function GetAreaOpacity: Double;
    procedure SetAreaOpacity(Value: Double);
    function GetLegendPosition: TLegendPosition;
    procedure SetLegendPosition(Value: TLegendPosition);
    function GetStacked: Boolean;
    procedure SetStacked(Value: Boolean);
    function GetColors: string;
    procedure SetColors(Value: string);
    function GetCenterText: string;
    procedure SetCenterText(Value: string);
    function GetSliceLabels: string;
    procedure SetSliceLabels(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TDBChartStyle;
    procedure SetStyle(Value: TDBChartStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property DataSource: TAylisComponent read GetDataSource write SetDataSource;
    property LabelField: string read GetLabelField write SetLabelField;
    property ValueFields: TStringArray read GetValueFields write SetValueFields;
    property Kind: TChartKind read GetKind write SetKind;
    property Title: string read GetTitle write SetTitle;
    property Labels: TStringArray read GetLabels write SetLabels;
    property Series: TChartSeriesArray read GetSeries write SetSeries;
    property Json: string read GetJson write SetJson;
    property Legend: Boolean read GetLegend write SetLegend;
    property Stepped: Boolean read GetStepped write SetStepped;
    property Tension: Double read GetTension write SetTension;
    property Animate: Boolean read GetAnimate write SetAnimate;
    property Zoom: Boolean read GetZoom write SetZoom;
    property AutoScale: Boolean read GetAutoScale write SetAutoScale;
    property YMin: Double read GetYMin write SetYMin;
    property YMax: Double read GetYMax write SetYMax;
    property LogScale: Boolean read GetLogScale write SetLogScale;
    property Reverse: Boolean read GetReverse write SetReverse;
    property TimeAxis: Boolean read GetTimeAxis write SetTimeAxis;
    property ClickedSeries: Int64 read GetClickedSeries write SetClickedSeries;
    property ClickedIndex: Int64 read GetClickedIndex write SetClickedIndex;
    property ValueFormat: TValueFormat read GetValueFormat write SetValueFormat;
    property LabelAngle: Double read GetLabelAngle write SetLabelAngle;
    property TitleAlign: TTitleAlign read GetTitleAlign write SetTitleAlign;
    property TitleBold: Boolean read GetTitleBold write SetTitleBold;
    property AreaOpacity: Double read GetAreaOpacity write SetAreaOpacity;
    property LegendPosition: TLegendPosition read GetLegendPosition write SetLegendPosition;
    property Stacked: Boolean read GetStacked write SetStacked;
    property Colors: string read GetColors write SetColors;
    property CenterText: string read GetCenterText write SetCenterText;
    property SliceLabels: string read GetSliceLabels write SetSliceLabels;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TDBChartStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um DataGrid de um form. }
  TDataGrid = class(TAylisComponent)
  private
    function GetOverscroll: TOverscroll;
    procedure SetOverscroll(Value: TOverscroll);
    function GetDataSource: TAylisComponent;
    procedure SetDataSource(Value: TAylisComponent);
    function GetColumns: TColumnSpecArray;
    procedure SetColumns(Value: TColumnSpecArray);
    function GetRowHeight: Double;
    procedure SetRowHeight(Value: Double);
    function GetReadOnly: Boolean;
    procedure SetReadOnly(Value: Boolean);
    function GetSortable: Boolean;
    procedure SetSortable(Value: Boolean);
    function GetColumnMove: Boolean;
    procedure SetColumnMove(Value: Boolean);
    function GetFixedColumns: Int64;
    procedure SetFixedColumns(Value: Int64);
    function GetFilterRow: Boolean;
    procedure SetFilterRow(Value: Boolean);
    function GetMultiSelect: Boolean;
    procedure SetMultiSelect(Value: Boolean);
    function GetFooter: TGridTotalArray;
    procedure SetFooter(Value: TGridTotalArray);
    function GetIncrementalSearch: Boolean;
    procedure SetIncrementalSearch(Value: Boolean);
    function GetCursorColor: TAylisColor;
    procedure SetCursorColor(Value: TAylisColor);
    function GetIndicator: Boolean;
    procedure SetIndicator(Value: Boolean);
    function GetHighlightCurrent: Boolean;
    procedure SetHighlightCurrent(Value: Boolean);
    function GetHorizontalWheel: THorizontalWheel;
    procedure SetHorizontalWheel(Value: THorizontalWheel);
    function GetOnError: string;
    procedure SetOnError(Value: string);
    function GetOnTitleClick: string;
    procedure SetOnTitleClick(Value: string);
    function GetOnSelectionChange: string;
    procedure SetOnSelectionChange(Value: string);
    function GetOnColumnMoved: string;
    procedure SetOnColumnMoved(Value: string);
    function GetScrollLeft: Double;
    procedure SetScrollLeft(Value: Double);
    function GetTopRow: Int64;
    procedure SetTopRow(Value: Int64);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TDataGridStyle;
    procedure SetStyle(Value: TDataGridStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    procedure SetFocus;
    property Overscroll: TOverscroll read GetOverscroll write SetOverscroll;
    property DataSource: TAylisComponent read GetDataSource write SetDataSource;
    property Columns: TColumnSpecArray read GetColumns write SetColumns;
    property RowHeight: Double read GetRowHeight write SetRowHeight;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Sortable: Boolean read GetSortable write SetSortable;
    property ColumnMove: Boolean read GetColumnMove write SetColumnMove;
    property FixedColumns: Int64 read GetFixedColumns write SetFixedColumns;
    property FilterRow: Boolean read GetFilterRow write SetFilterRow;
    property MultiSelect: Boolean read GetMultiSelect write SetMultiSelect;
    property Footer: TGridTotalArray read GetFooter write SetFooter;
    property IncrementalSearch: Boolean read GetIncrementalSearch write SetIncrementalSearch;
    property CursorColor: TAylisColor read GetCursorColor write SetCursorColor;
    property Indicator: Boolean read GetIndicator write SetIndicator;
    property HighlightCurrent: Boolean read GetHighlightCurrent write SetHighlightCurrent;
    property HorizontalWheel: THorizontalWheel read GetHorizontalWheel write SetHorizontalWheel;
    property OnError: string read GetOnError write SetOnError;
    property OnTitleClick: string read GetOnTitleClick write SetOnTitleClick;
    property OnSelectionChange: string read GetOnSelectionChange write SetOnSelectionChange;
    property OnColumnMoved: string read GetOnColumnMoved write SetOnColumnMoved;
    property ScrollLeft: Double read GetScrollLeft write SetScrollLeft;
    property TopRow: Int64 read GetTopRow write SetTopRow;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TDataGridStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um DataSource de um form. }
  TDataSource = class(TAylisComponent)
  private
    function GetDataSet: TAylisComponent;
    procedure SetDataSet(Value: TAylisComponent);
    function GetAutoEdit: Boolean;
    procedure SetAutoEdit(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetOnDataChange: string;
    procedure SetOnDataChange(Value: string);
    function GetOnStateChange: string;
    procedure SetOnStateChange(Value: string);
  public
    property DataSet: TAylisComponent read GetDataSet write SetDataSet;
    property AutoEdit: Boolean read GetAutoEdit write SetAutoEdit;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property OnDataChange: string read GetOnDataChange write SetOnDataChange;
    property OnStateChange: string read GetOnStateChange write SetOnStateChange;
  end;

  { Um Field de um form. }
  TField = class(TAylisComponent)
  private
    function GetFieldName: string;
    procedure SetFieldName(Value: string);
    function GetType: TFieldType;
    procedure SetType(Value: TFieldType);
    function GetSize: Int64;
    procedure SetSize(Value: Int64);
    function GetScale: Int64;
    procedure SetScale(Value: Int64);
    function GetRounding: TRounding;
    procedure SetRounding(Value: TRounding);
    function GetDisplayLabel: string;
    procedure SetDisplayLabel(Value: string);
    function GetDisplayWidth: Double;
    procedure SetDisplayWidth(Value: Double);
    function GetDisplayFormat: string;
    procedure SetDisplayFormat(Value: string);
    function GetEditFormat: string;
    procedure SetEditFormat(Value: string);
    function GetAlignment: TTextAlign;
    procedure SetAlignment(Value: TTextAlign);
    function GetRequired: Boolean;
    procedure SetRequired(Value: Boolean);
    function GetReadOnly: Boolean;
    procedure SetReadOnly(Value: Boolean);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetMin: Double;
    procedure SetMin(Value: Double);
    function GetMax: Double;
    procedure SetMax(Value: Double);
    function GetDefaultValue: string;
    procedure SetDefaultValue(Value: string);
    function GetDisplayValues: string;
    procedure SetDisplayValues(Value: string);
    function GetCharCase: TCharCase;
    procedure SetCharCase(Value: TCharCase);
    function GetEditMask: string;
    procedure SetEditMask(Value: string);
    function GetSensitive: Boolean;
    procedure SetSensitive(Value: Boolean);
    function GetKind: TFieldKind;
    procedure SetKind(Value: TFieldKind);
    function GetExpression: string;
    procedure SetExpression(Value: string);
    function GetLookupDataSet: TAylisComponent;
    procedure SetLookupDataSet(Value: TAylisComponent);
    function GetKeyFields: string;
    procedure SetKeyFields(Value: string);
    function GetLookupKeyFields: string;
    procedure SetLookupKeyFields(Value: string);
    function GetLookupResultField: string;
    procedure SetLookupResultField(Value: string);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
    function GetOnGetText: string;
    procedure SetOnGetText(Value: string);
    function GetOnSetText: string;
    procedure SetOnSetText(Value: string);
  public
    property FieldName: string read GetFieldName write SetFieldName;
    property &Type: TFieldType read GetType write SetType;
    property Size: Int64 read GetSize write SetSize;
    property Scale: Int64 read GetScale write SetScale;
    property Rounding: TRounding read GetRounding write SetRounding;
    property DisplayLabel: string read GetDisplayLabel write SetDisplayLabel;
    property DisplayWidth: Double read GetDisplayWidth write SetDisplayWidth;
    property DisplayFormat: string read GetDisplayFormat write SetDisplayFormat;
    property EditFormat: string read GetEditFormat write SetEditFormat;
    property Alignment: TTextAlign read GetAlignment write SetAlignment;
    property Required: Boolean read GetRequired write SetRequired;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Visible: Boolean read GetVisible write SetVisible;
    property Min: Double read GetMin write SetMin;
    property Max: Double read GetMax write SetMax;
    property DefaultValue: string read GetDefaultValue write SetDefaultValue;
    property DisplayValues: string read GetDisplayValues write SetDisplayValues;
    property CharCase: TCharCase read GetCharCase write SetCharCase;
    property EditMask: string read GetEditMask write SetEditMask;
    property Sensitive: Boolean read GetSensitive write SetSensitive;
    property Kind: TFieldKind read GetKind write SetKind;
    property Expression: string read GetExpression write SetExpression;
    property LookupDataSet: TAylisComponent read GetLookupDataSet write SetLookupDataSet;
    property KeyFields: string read GetKeyFields write SetKeyFields;
    property LookupKeyFields: string read GetLookupKeyFields write SetLookupKeyFields;
    property LookupResultField: string read GetLookupResultField write SetLookupResultField;
    property OnChange: string read GetOnChange write SetOnChange;
    property OnGetText: string read GetOnGetText write SetOnGetText;
    property OnSetText: string read GetOnSetText write SetOnSetText;
  end;

  { Um Form de um form. }
  TForm = class(TAylisForm)
  private
    function GetCaption: string;
    procedure SetCaption(Value: string);
    function GetPosition: TFormPosition;
    procedure SetPosition(Value: TFormPosition);
    function GetBorderStyle: TBorderStyle;
    procedure SetBorderStyle(Value: TBorderStyle);
    function GetWindowCorners: TWindowCorners;
    procedure SetWindowCorners(Value: TWindowCorners);
    function GetTransparent: Boolean;
    procedure SetTransparent(Value: Boolean);
    function GetWindowShape: TWindowShape;
    procedure SetWindowShape(Value: TWindowShape);
    function GetShapeRadius: Double;
    procedure SetShapeRadius(Value: Double);
    function GetShapeRadii: TAylisBounds;
    procedure SetShapeRadii(Value: TAylisBounds);
    function GetShapePath: string;
    procedure SetShapePath(Value: string);
    function GetWindowState: TWindowState;
    procedure SetWindowState(Value: TWindowState);
    function GetTransition: TTransition;
    procedure SetTransition(Value: TTransition);
    function GetLayerCache: TLayerCaching;
    procedure SetLayerCache(Value: TLayerCaching);
    function GetFrameReuseKb: Int64;
    procedure SetFrameReuseKb(Value: Int64);
    function GetTopmost: Boolean;
    procedure SetTopmost(Value: Boolean);
    function GetShowInTaskbar: Boolean;
    procedure SetShowInTaskbar(Value: Boolean);
    function GetActiveControl: TAylisComponent;
    procedure SetActiveControl(Value: TAylisComponent);
    function GetTabOrder: TStringArray;
    procedure SetTabOrder(Value: TStringArray);
    function GetTabKeys: string;
    procedure SetTabKeys(Value: string);
    function GetBackTabKeys: string;
    procedure SetBackTabKeys(Value: string);
    function GetAllowedChars: string;
    procedure SetAllowedChars(Value: string);
    function GetBlockedChars: string;
    procedure SetBlockedChars(Value: string);
    function GetOnlyAllowed: Boolean;
    procedure SetOnlyAllowed(Value: Boolean);
    function GetCharsOverride: Boolean;
    procedure SetCharsOverride(Value: Boolean);
    function GetCharMap: string;
    procedure SetCharMap(Value: string);
    function GetKeyPreview: Boolean;
    procedure SetKeyPreview(Value: Boolean);
    function GetZoom: Double;
    procedure SetZoom(Value: Double);
    function GetMonitor: Int64;
    procedure SetMonitor(Value: Int64);
    function GetRememberPlacement: Boolean;
    procedure SetRememberPlacement(Value: Boolean);
    function GetGpu: string;
    procedure SetGpu(Value: string);
    function GetPresent: TFormPresent;
    procedure SetPresent(Value: TFormPresent);
    function GetSpanMonitors: Boolean;
    procedure SetSpanMonitors(Value: Boolean);
    function GetPhysicalPixels: Boolean;
    procedure SetPhysicalPixels(Value: Boolean);
    function GetActivateOnShow: Boolean;
    procedure SetActivateOnShow(Value: Boolean);
    function GetForceForeground: Boolean;
    procedure SetForceForeground(Value: Boolean);
    function GetCloseAction: TCloseAction;
    procedure SetCloseAction(Value: TCloseAction);
    function GetFrameRate: Double;
    procedure SetFrameRate(Value: Double);
    function GetOrientation: TFormOrientation;
    procedure SetOrientation(Value: TFormOrientation);
    function GetOnShow: string;
    procedure SetOnShow(Value: string);
    function GetOnWindowCreated: string;
    procedure SetOnWindowCreated(Value: string);
    function GetOnClose: string;
    procedure SetOnClose(Value: string);
    function GetOnActivate: string;
    procedure SetOnActivate(Value: string);
    function GetOnDeactivate: string;
    procedure SetOnDeactivate(Value: string);
    function GetOnMinimize: string;
    procedure SetOnMinimize(Value: string);
    function GetOnRestore: string;
    procedure SetOnRestore(Value: string);
    function GetOnNavigate: string;
    procedure SetOnNavigate(Value: string);
    function GetOnGpuChange: string;
    procedure SetOnGpuChange(Value: string);
    function GetClientWidth: Double;
    procedure SetClientWidth(Value: Double);
    function GetClientHeight: Double;
    procedure SetClientHeight(Value: Double);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TFormStyle;
    procedure SetStyle(Value: TFormStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    procedure MoveToMonitor(AMonitor: Int64);
    function GpuAdapter: string;
    function GpuAdapters: TStringArray;
    function RenderRgba(AX: Double; AY: Double; AWidth: Double; AHeight: Double; AScale: Double): string;
    function Focused: string;
    procedure FocusNext;
    procedure FocusPrev;
    property Caption: string read GetCaption write SetCaption;
    property Position: TFormPosition read GetPosition write SetPosition;
    property BorderStyle: TBorderStyle read GetBorderStyle write SetBorderStyle;
    property WindowCorners: TWindowCorners read GetWindowCorners write SetWindowCorners;
    property Transparent: Boolean read GetTransparent write SetTransparent;
    property WindowShape: TWindowShape read GetWindowShape write SetWindowShape;
    property ShapeRadius: Double read GetShapeRadius write SetShapeRadius;
    property ShapeRadii: TAylisBounds read GetShapeRadii write SetShapeRadii;
    property ShapePath: string read GetShapePath write SetShapePath;
    property WindowState: TWindowState read GetWindowState write SetWindowState;
    property Transition: TTransition read GetTransition write SetTransition;
    property LayerCache: TLayerCaching read GetLayerCache write SetLayerCache;
    property FrameReuseKb: Int64 read GetFrameReuseKb write SetFrameReuseKb;
    property Topmost: Boolean read GetTopmost write SetTopmost;
    property ShowInTaskbar: Boolean read GetShowInTaskbar write SetShowInTaskbar;
    property ActiveControl: TAylisComponent read GetActiveControl write SetActiveControl;
    property TabOrder: TStringArray read GetTabOrder write SetTabOrder;
    property TabKeys: string read GetTabKeys write SetTabKeys;
    property BackTabKeys: string read GetBackTabKeys write SetBackTabKeys;
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property KeyPreview: Boolean read GetKeyPreview write SetKeyPreview;
    property Zoom: Double read GetZoom write SetZoom;
    property Monitor: Int64 read GetMonitor write SetMonitor;
    property RememberPlacement: Boolean read GetRememberPlacement write SetRememberPlacement;
    property Gpu: string read GetGpu write SetGpu;
    property Present: TFormPresent read GetPresent write SetPresent;
    property SpanMonitors: Boolean read GetSpanMonitors write SetSpanMonitors;
    property PhysicalPixels: Boolean read GetPhysicalPixels write SetPhysicalPixels;
    property ActivateOnShow: Boolean read GetActivateOnShow write SetActivateOnShow;
    property ForceForeground: Boolean read GetForceForeground write SetForceForeground;
    property CloseAction: TCloseAction read GetCloseAction write SetCloseAction;
    property FrameRate: Double read GetFrameRate write SetFrameRate;
    property Orientation: TFormOrientation read GetOrientation write SetOrientation;
    property OnShow: string read GetOnShow write SetOnShow;
    property OnWindowCreated: string read GetOnWindowCreated write SetOnWindowCreated;
    property OnClose: string read GetOnClose write SetOnClose;
    property OnActivate: string read GetOnActivate write SetOnActivate;
    property OnDeactivate: string read GetOnDeactivate write SetOnDeactivate;
    property OnMinimize: string read GetOnMinimize write SetOnMinimize;
    property OnRestore: string read GetOnRestore write SetOnRestore;
    property OnNavigate: string read GetOnNavigate write SetOnNavigate;
    property OnGpuChange: string read GetOnGpuChange write SetOnGpuChange;
    property ClientWidth: Double read GetClientWidth write SetClientWidth;
    property ClientHeight: Double read GetClientHeight write SetClientHeight;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TFormStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um GroupBox de um form. }
  TGroupBox = class(TAylisComponent)
  private
    function GetCaption: string;
    procedure SetCaption(Value: string);
    function GetAllowedChars: string;
    procedure SetAllowedChars(Value: string);
    function GetBlockedChars: string;
    procedure SetBlockedChars(Value: string);
    function GetOnlyAllowed: Boolean;
    procedure SetOnlyAllowed(Value: Boolean);
    function GetCharsOverride: Boolean;
    procedure SetCharsOverride(Value: Boolean);
    function GetCharMap: string;
    procedure SetCharMap(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TGroupBoxStyle;
    procedure SetStyle(Value: TGroupBoxStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property Caption: string read GetCaption write SetCaption;
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TGroupBoxStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Image de um form. }
  TImage = class(TAylisComponent)
  private
    function GetImage: string;
    procedure SetImage(Value: string);
    function GetFit: TImageFit;
    procedure SetFit(Value: TImageFit);
    function GetRadius: Double;
    procedure SetRadius(Value: Double);
    function GetZoomable: Boolean;
    procedure SetZoomable(Value: Boolean);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TImageStyle;
    procedure SetStyle(Value: TImageStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property Image: string read GetImage write SetImage;
    property Fit: TImageFit read GetFit write SetFit;
    property Radius: Double read GetRadius write SetRadius;
    property Zoomable: Boolean read GetZoomable write SetZoomable;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TImageStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Label de um form. }
  TLabel = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetHAlign: TTextAlign;
    procedure SetHAlign(Value: TTextAlign);
    function GetVAlign: TTextAlign;
    procedure SetVAlign(Value: TTextAlign);
    function GetWrap: Boolean;
    procedure SetWrap(Value: Boolean);
    function GetTrimming: TTrimming;
    procedure SetTrimming(Value: TTrimming);
    function GetBold: Boolean;
    procedure SetBold(Value: Boolean);
    function GetColor: TAylisColor;
    procedure SetColor(Value: TAylisColor);
    function GetHighlight: string;
    procedure SetHighlight(Value: string);
    function GetLetterSpacing: Double;
    procedure SetLetterSpacing(Value: Double);
    function GetTextCase: TTextCase;
    procedure SetTextCase(Value: TTextCase);
    function GetMarkup: Boolean;
    procedure SetMarkup(Value: Boolean);
    function GetOutline: TAylisColor;
    procedure SetOutline(Value: TAylisColor);
    function GetOutlineWidth: Double;
    procedure SetOutlineWidth(Value: Double);
    function GetLineHeight: Double;
    procedure SetLineHeight(Value: Double);
    function GetTextShadow: TAylisShadow;
    procedure SetTextShadow(Value: TAylisShadow);
    function GetFocusControl: TAylisComponent;
    procedure SetFocusControl(Value: TAylisComponent);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TLabelStyle;
    procedure SetStyle(Value: TLabelStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    function MinContentWidth: Double;
    function MaxContentWidth: Double;
    property Text: string read GetText write SetText;
    property HAlign: TTextAlign read GetHAlign write SetHAlign;
    property VAlign: TTextAlign read GetVAlign write SetVAlign;
    property Wrap: Boolean read GetWrap write SetWrap;
    property Trimming: TTrimming read GetTrimming write SetTrimming;
    property Bold: Boolean read GetBold write SetBold;
    property Color: TAylisColor read GetColor write SetColor;
    property Highlight: string read GetHighlight write SetHighlight;
    property LetterSpacing: Double read GetLetterSpacing write SetLetterSpacing;
    property TextCase: TTextCase read GetTextCase write SetTextCase;
    property Markup: Boolean read GetMarkup write SetMarkup;
    property Outline: TAylisColor read GetOutline write SetOutline;
    property OutlineWidth: Double read GetOutlineWidth write SetOutlineWidth;
    property LineHeight: Double read GetLineHeight write SetLineHeight;
    property TextShadow: TAylisShadow read GetTextShadow write SetTextShadow;
    property FocusControl: TAylisComponent read GetFocusControl write SetFocusControl;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TLabelStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Layout de um form. }
  TLayout = class(TAylisComponent)
  private
    function GetAllowedChars: string;
    procedure SetAllowedChars(Value: string);
    function GetBlockedChars: string;
    procedure SetBlockedChars(Value: string);
    function GetOnlyAllowed: Boolean;
    procedure SetOnlyAllowed(Value: Boolean);
    function GetCharsOverride: Boolean;
    procedure SetCharsOverride(Value: Boolean);
    function GetCharMap: string;
    procedure SetCharMap(Value: string);
    function GetGap: Double;
    procedure SetGap(Value: Double);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TLayoutStyle;
    procedure SetStyle(Value: TLayoutStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property Gap: Double read GetGap write SetGap;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TLayoutStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Panel de um form. }
  TPanel = class(TAylisComponent)
  private
    function GetAllowedChars: string;
    procedure SetAllowedChars(Value: string);
    function GetBlockedChars: string;
    procedure SetBlockedChars(Value: string);
    function GetOnlyAllowed: Boolean;
    procedure SetOnlyAllowed(Value: Boolean);
    function GetCharsOverride: Boolean;
    procedure SetCharsOverride(Value: Boolean);
    function GetCharMap: string;
    procedure SetCharMap(Value: string);
    function GetGap: Double;
    procedure SetGap(Value: Double);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetFill2: TAylisColor;
    procedure SetFill2(Value: TAylisColor);
    function GetFillMid: TAylisColor;
    procedure SetFillMid(Value: TAylisColor);
    function GetFillMidAt: Double;
    procedure SetFillMidAt(Value: Double);
    function GetGradientAngle: Double;
    procedure SetGradientAngle(Value: Double);
    function GetGradientRadial: Boolean;
    procedure SetGradientRadial(Value: Boolean);
    function GetStroke: TAylisColor;
    procedure SetStroke(Value: TAylisColor);
    function GetStrokeWidth: Double;
    procedure SetStrokeWidth(Value: Double);
    function GetRadius: TAylisBounds;
    procedure SetRadius(Value: TAylisBounds);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TPanelStyle;
    procedure SetStyle(Value: TPanelStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property Gap: Double read GetGap write SetGap;
    property Fill: TAylisColor read GetFill write SetFill;
    property Fill2: TAylisColor read GetFill2 write SetFill2;
    property FillMid: TAylisColor read GetFillMid write SetFillMid;
    property FillMidAt: Double read GetFillMidAt write SetFillMidAt;
    property GradientAngle: Double read GetGradientAngle write SetGradientAngle;
    property GradientRadial: Boolean read GetGradientRadial write SetGradientRadial;
    property Stroke: TAylisColor read GetStroke write SetStroke;
    property StrokeWidth: Double read GetStrokeWidth write SetStrokeWidth;
    property Radius: TAylisBounds read GetRadius write SetRadius;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TPanelStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Splitter de um form. }
  TSplitter = class(TAylisComponent)
  private
    function GetMinSize: Double;
    procedure SetMinSize(Value: Double);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TSplitterStyle;
    procedure SetStyle(Value: TSplitterStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property MinSize: Double read GetMinSize write SetMinSize;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TSplitterStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { Um Svg de um form. }
  TSvg = class(TAylisComponent)
  private
    function GetSource: string;
    procedure SetSource(Value: string);
    function GetColor: TAylisColor;
    procedure SetColor(Value: TAylisColor);
    function GetFit: TImageFit;
    procedure SetFit(Value: TImageFit);
    function GetHighlight: TStringArray;
    procedure SetHighlight(Value: TStringArray);
    function GetHighlightColor: TAylisColor;
    procedure SetHighlightColor(Value: TAylisColor);
    function GetZoomTo: string;
    procedure SetZoomTo(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: TAlign;
    procedure SetAlign(Value: TAlign);
    function GetAnchors: TAnchorSet;
    procedure SetAnchors(Value: TAnchorSet);
    function GetMargins: TAylisBounds;
    procedure SetMargins(Value: TAylisBounds);
    function GetPadding: TAylisBounds;
    procedure SetPadding(Value: TAylisBounds);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetInheritState: Boolean;
    procedure SetInheritState(Value: Boolean);
    function GetTranslate: Boolean;
    procedure SetTranslate(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
    function GetCapturesGesture: Boolean;
    procedure SetCapturesGesture(Value: Boolean);
    function GetCanFocus: Boolean;
    procedure SetCanFocus(Value: Boolean);
    function GetDraggable: Boolean;
    procedure SetDraggable(Value: Boolean);
    function GetAcceptsDrops: Boolean;
    procedure SetAcceptsDrops(Value: Boolean);
    function GetDragData: string;
    procedure SetDragData(Value: string);
    function GetDragLabel: string;
    procedure SetDragLabel(Value: string);
    function GetDragImage: TAylisComponent;
    procedure SetDragImage(Value: TAylisComponent);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: TCursorShape;
    procedure SetCursor(Value: TCursorShape);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetBlendMode: TBlendMode;
    procedure SetBlendMode(Value: TBlendMode);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
    function GetScale: Double;
    procedure SetScale(Value: Double);
    function GetTranslateX: Double;
    procedure SetTranslateX(Value: Double);
    function GetTranslateY: Double;
    procedure SetTranslateY(Value: Double);
    function GetAspectRatio: Double;
    procedure SetAspectRatio(Value: Double);
    function GetMaxWidth: Double;
    procedure SetMaxWidth(Value: Double);
    function GetMinWidth: Double;
    procedure SetMinWidth(Value: Double);
    function GetMinHeight: Double;
    procedure SetMinHeight(Value: Double);
    function GetMaxHeight: Double;
    procedure SetMaxHeight(Value: Double);
    function GetSelfAlign: TSelfAlign;
    procedure SetSelfAlign(Value: TSelfAlign);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetGridRow: Int64;
    procedure SetGridRow(Value: Int64);
    function GetGridColumn: Int64;
    procedure SetGridColumn(Value: Int64);
    function GetRowSpan: Int64;
    procedure SetRowSpan(Value: Int64);
    function GetGrow: Double;
    procedure SetGrow(Value: Double);
    function GetShadow: TAylisShadow;
    procedure SetShadow(Value: TAylisShadow);
    function GetStyle: TSvgStyle;
    procedure SetStyle(Value: TSvgStyle);
    function GetTheme: TThemeName;
    procedure SetTheme(Value: TThemeName);
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetBorderColor: TAylisColor;
    procedure SetBorderColor(Value: TAylisColor);
    function GetTextColor: TAylisColor;
    procedure SetTextColor(Value: TAylisColor);
    function GetAccentColor: TAylisColor;
    procedure SetAccentColor(Value: TAylisColor);
    function GetSuccessColor: TAylisColor;
    procedure SetSuccessColor(Value: TAylisColor);
    function GetWarningColor: TAylisColor;
    procedure SetWarningColor(Value: TAylisColor);
    function GetDangerColor: TAylisColor;
    procedure SetDangerColor(Value: TAylisColor);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: TFontWeight;
    procedure SetFontWeight(Value: TFontWeight);
    function GetFontItalic: Boolean;
    procedure SetFontItalic(Value: Boolean);
    function GetFontUnderline: Boolean;
    procedure SetFontUnderline(Value: Boolean);
    function GetFontStrikeout: Boolean;
    procedure SetFontStrikeout(Value: Boolean);
    function GetHint: string;
    procedure SetHint(Value: string);
    function GetTag: Int64;
    procedure SetTag(Value: Int64);
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: TAylisComponent;
    procedure SetPopupMenu(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
    function GetOnDblClick: string;
    procedure SetOnDblClick(Value: string);
    function GetOnLongPress: string;
    procedure SetOnLongPress(Value: string);
    function GetOnPointerDown: string;
    procedure SetOnPointerDown(Value: string);
    function GetOnPointerUp: string;
    procedure SetOnPointerUp(Value: string);
    function GetOnPointerMove: string;
    procedure SetOnPointerMove(Value: string);
    function GetOnPointerEnter: string;
    procedure SetOnPointerEnter(Value: string);
    function GetOnPointerLeave: string;
    procedure SetOnPointerLeave(Value: string);
    function GetOnWheel: string;
    procedure SetOnWheel(Value: string);
    function GetOnGesture: string;
    procedure SetOnGesture(Value: string);
    function GetOnSwipe: string;
    procedure SetOnSwipe(Value: string);
    function GetOnKeyUp: string;
    procedure SetOnKeyUp(Value: string);
    function GetOnEnter: string;
    procedure SetOnEnter(Value: string);
    function GetOnExit: string;
    procedure SetOnExit(Value: string);
    function GetOnResize: string;
    procedure SetOnResize(Value: string);
    function GetOnDragMove: string;
    procedure SetOnDragMove(Value: string);
    function GetOnDragLeave: string;
    procedure SetOnDragLeave(Value: string);
    function GetOnDragDrop: string;
    procedure SetOnDragDrop(Value: string);
    function GetOnDragEnd: string;
    procedure SetOnDragEnd(Value: string);
  public
    property Source: string read GetSource write SetSource;
    property Color: TAylisColor read GetColor write SetColor;
    property Fit: TImageFit read GetFit write SetFit;
    property Highlight: TStringArray read GetHighlight write SetHighlight;
    property HighlightColor: TAylisColor read GetHighlightColor write SetHighlightColor;
    property ZoomTo: string read GetZoomTo write SetZoomTo;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: TAlign read GetAlign write SetAlign;
    property Anchors: TAnchorSet read GetAnchors write SetAnchors;
    property Margins: TAylisBounds read GetMargins write SetMargins;
    property Padding: TAylisBounds read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CapturesGesture: Boolean read GetCapturesGesture write SetCapturesGesture;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: TAylisComponent read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: TCursorShape read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property BlendMode: TBlendMode read GetBlendMode write SetBlendMode;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property Scale: Double read GetScale write SetScale;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: TSelfAlign read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
    property Shadow: TAylisShadow read GetShadow write SetShadow;
    property Style: TSvgStyle read GetStyle write SetStyle;
    property Theme: TThemeName read GetTheme write SetTheme;
    property Fill: TAylisColor read GetFill write SetFill;
    property BorderColor: TAylisColor read GetBorderColor write SetBorderColor;
    property TextColor: TAylisColor read GetTextColor write SetTextColor;
    property AccentColor: TAylisColor read GetAccentColor write SetAccentColor;
    property SuccessColor: TAylisColor read GetSuccessColor write SetSuccessColor;
    property WarningColor: TAylisColor read GetWarningColor write SetWarningColor;
    property DangerColor: TAylisColor read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: TFontWeight read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;
    property OnClick: string read GetOnClick write SetOnClick;
    property OnDblClick: string read GetOnDblClick write SetOnDblClick;
    property OnLongPress: string read GetOnLongPress write SetOnLongPress;
    property OnPointerDown: string read GetOnPointerDown write SetOnPointerDown;
    property OnPointerUp: string read GetOnPointerUp write SetOnPointerUp;
    property OnPointerMove: string read GetOnPointerMove write SetOnPointerMove;
    property OnPointerEnter: string read GetOnPointerEnter write SetOnPointerEnter;
    property OnPointerLeave: string read GetOnPointerLeave write SetOnPointerLeave;
    property OnWheel: string read GetOnWheel write SetOnWheel;
    property OnGesture: string read GetOnGesture write SetOnGesture;
    property OnSwipe: string read GetOnSwipe write SetOnSwipe;
    property OnKeyUp: string read GetOnKeyUp write SetOnKeyUp;
    property OnEnter: string read GetOnEnter write SetOnEnter;
    property OnExit: string read GetOnExit write SetOnExit;
    property OnResize: string read GetOnResize write SetOnResize;
    property OnDragMove: string read GetOnDragMove write SetOnDragMove;
    property OnDragLeave: string read GetOnDragLeave write SetOnDragLeave;
    property OnDragDrop: string read GetOnDragDrop write SetOnDragDrop;
    property OnDragEnd: string read GetOnDragEnd write SetOnDragEnd;
  end;

  { O form frmIBGE: a classe base com os componentes. A do form (TFrmIBGE), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmIBGEBase = class(TForm)
  published
    cdsEstados: TClientDataSet;
    cdsEstadosId: TField;
    cdsEstadosSigla: TField;
    cdsEstadosNome: TField;
    cdsEstadosRegiao: TField;
    cdsEstadosPopulacao: TField;
    cdsEstadosArea: TField;
    cdsEstadosDensidade: TField;
    cdsEstadosPib: TField;
    cdsEstadosPibPerCapita: TField;
    cdsEstadosEstimativa: TField;
    cdsEstadosEstimativaAno: TField;
    cdsEstadosMunicipios: TField;
    cdsEstadosBandeira: TField;
    dsEstados: TDataSource;
    cdsMunicipios: TClientDataSet;
    cdsMunicipiosId: TField;
    cdsMunicipiosUfId: TField;
    cdsMunicipiosUf: TField;
    cdsMunicipiosNome: TField;
    cdsMunicipiosMesorregiao: TField;
    cdsMunicipiosMicrorregiao: TField;
    cdsMunicipiosRegiaoIntermediaria: TField;
    cdsMunicipiosRegiaoImediata: TField;
    cdsMunicipiosPopulacao: TField;
    cdsMunicipiosArea: TField;
    cdsMunicipiosDensidade: TField;
    cdsMunicipiosPib: TField;
    cdsMunicipiosPibParticipacao: TField;
    cdsMunicipiosPibPerCapita: TField;
    cdsMunicipiosEstimativa: TField;
    cdsMunicipiosEstimativaAno: TField;
    cdsMunicipiosDistritos: TField;
    cdsMunicipiosBandeira: TField;
    dsMunicipios: TDataSource;
    cdsDistritos: TClientDataSet;
    cdsDistritosId: TField;
    cdsDistritosMunicipioId: TField;
    cdsDistritosNome: TField;
    dsDistritos: TDataSource;
    cdsPopulacao: TClientDataSet;
    cdsPopulacaoAno: TField;
    cdsPopulacaoHabitantes: TField;
    dsPopulacao: TDataSource;
    cdsMaiores: TClientDataSet;
    cdsMaioresNome: TField;
    cdsMaioresHabitantes: TField;
    dsMaiores: TDataSource;
    lblFonte: TLabel;
    lytListas: TLayout;
    gbEstados: TGroupBox;
    grdEstados: TDataGrid;
    gbDistritos: TGroupBox;
    grdDistritos: TDataGrid;
    gbMunicipios: TGroupBox;
    grdMunicipios: TDataGrid;
    splListas: TSplitter;
    lytPainel: TLayout;
    pnlEstado: TPanel;
    svgBrasil: TSvg;
    lytBandeiraEstado: TLayout;
    lblEstado: TLabel;
    lblRegiao: TLabel;
    imgBandeiraEstado: TImage;
    lytNumerosEstado: TLayout;
    dbtEstadoPopulacao: TLabel;
    lblEstadoEstimativa: TLabel;
    dbtEstadoEstimativa: TLabel;
    dbtEstadoArea: TLabel;
    dbtEstadoDensidade: TLabel;
    dbtEstadoPib: TLabel;
    dbtEstadoPibPerCapita: TLabel;
    dbtEstadoMunicipios: TLabel;
    pnlMunicipio: TPanel;
    svgMunicipio: TSvg;
    svgEstado: TSvg;
    lytBandeiraMunicipio: TLayout;
    lblMunicipio: TLabel;
    lblCodigo: TLabel;
    imgBandeiraMunicipio: TImage;
    lblSemBandeira: TLabel;
    lytNumerosMunicipio: TLayout;
    dbtMesorregiao: TLabel;
    dbtMicrorregiao: TLabel;
    dbtRegiaoImediata: TLabel;
    dbtPopulacao: TLabel;
    lblEstimativa: TLabel;
    dbtEstimativa: TLabel;
    dbtArea: TLabel;
    dbtDensidade: TLabel;
    dbtPib: TLabel;
    dbtPibPerCapita: TLabel;
    dbtPibParticipacao: TLabel;
    lytGraficos: TLayout;
    dbcPopulacao: TDBChart;
    dbcMaiores: TDBChart;
    dbcEstados: TDBChart;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O ClientDataSet deste form com o nome (nil se não há). }
    function FindClientDataSet(const Nome: string): TClientDataSet;
    { O ClientDataSet que disparou o evento (nil se não é um ClientDataSet deste form). }
    function SenderAsClientDataSet(Sender: TAylisComponent): TClientDataSet; overload;
    function SenderAsClientDataSet(Event: TJSONObject): TClientDataSet; overload;
    { O DBChart deste form com o nome (nil se não há). }
    function FindDBChart(const Nome: string): TDBChart;
    { O DBChart que disparou o evento (nil se não é um DBChart deste form). }
    function SenderAsDBChart(Sender: TAylisComponent): TDBChart; overload;
    function SenderAsDBChart(Event: TJSONObject): TDBChart; overload;
    { O DataGrid deste form com o nome (nil se não há). }
    function FindDataGrid(const Nome: string): TDataGrid;
    { O DataGrid que disparou o evento (nil se não é um DataGrid deste form). }
    function SenderAsDataGrid(Sender: TAylisComponent): TDataGrid; overload;
    function SenderAsDataGrid(Event: TJSONObject): TDataGrid; overload;
    { O DataSource deste form com o nome (nil se não há). }
    function FindDataSource(const Nome: string): TDataSource;
    { O DataSource que disparou o evento (nil se não é um DataSource deste form). }
    function SenderAsDataSource(Sender: TAylisComponent): TDataSource; overload;
    function SenderAsDataSource(Event: TJSONObject): TDataSource; overload;
    { O Field deste form com o nome (nil se não há). }
    function FindField(const Nome: string): TField;
    { O Field que disparou o evento (nil se não é um Field deste form). }
    function SenderAsField(Sender: TAylisComponent): TField; overload;
    function SenderAsField(Event: TJSONObject): TField; overload;
    { O GroupBox deste form com o nome (nil se não há). }
    function FindGroupBox(const Nome: string): TGroupBox;
    { O GroupBox que disparou o evento (nil se não é um GroupBox deste form). }
    function SenderAsGroupBox(Sender: TAylisComponent): TGroupBox; overload;
    function SenderAsGroupBox(Event: TJSONObject): TGroupBox; overload;
    { O Image deste form com o nome (nil se não há). }
    function FindImage(const Nome: string): TImage;
    { O Image que disparou o evento (nil se não é um Image deste form). }
    function SenderAsImage(Sender: TAylisComponent): TImage; overload;
    function SenderAsImage(Event: TJSONObject): TImage; overload;
    { O Label deste form com o nome (nil se não há). }
    function FindLabel(const Nome: string): TLabel;
    { O Label que disparou o evento (nil se não é um Label deste form). }
    function SenderAsLabel(Sender: TAylisComponent): TLabel; overload;
    function SenderAsLabel(Event: TJSONObject): TLabel; overload;
    { O Layout deste form com o nome (nil se não há). }
    function FindLayout(const Nome: string): TLayout;
    { O Layout que disparou o evento (nil se não é um Layout deste form). }
    function SenderAsLayout(Sender: TAylisComponent): TLayout; overload;
    function SenderAsLayout(Event: TJSONObject): TLayout; overload;
    { O Panel deste form com o nome (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O Splitter deste form com o nome (nil se não há). }
    function FindSplitter(const Nome: string): TSplitter;
    { O Splitter que disparou o evento (nil se não é um Splitter deste form). }
    function SenderAsSplitter(Sender: TAylisComponent): TSplitter; overload;
    function SenderAsSplitter(Event: TJSONObject): TSplitter; overload;
    { O Svg deste form com o nome (nil se não há). }
    function FindSvg(const Nome: string): TSvg;
    { O Svg que disparou o evento (nil se não é um Svg deste form). }
    function SenderAsSvg(Sender: TAylisComponent): TSvg; overload;
    function SenderAsSvg(Event: TJSONObject): TSvg; overload;
  end;

function AsText(Data: TJSONData): string;
function AsDate(Data: TJSONData): TDateTime;
function AsTime(Data: TJSONData): TDateTime;
function AsDateTime(Data: TJSONData): TDateTime;
function AsList(Data: TJSONData): TStringArray;
function AsNumbers(Data: TJSONData): TAylisDoubleArray;
function AsColors(Data: TJSONData): TAylisColorArray;
function AsDateRange(Data: TJSONData): TAylisDateRange;
function AlignOf(Data: TJSONData): TAlign;
function AlignSetOf(Data: TJSONData): TAlignSet;
function AlignSetToJSON(const S: TAlignSet): TJSONData;
function AnchorOf(Data: TJSONData): TAnchor;
function AnchorSetOf(Data: TJSONData): TAnchorSet;
function AnchorSetToJSON(const S: TAnchorSet): TJSONData;
function ApplyModeOf(Data: TJSONData): TApplyMode;
function ApplyModeSetOf(Data: TJSONData): TApplyModeSet;
function ApplyModeSetToJSON(const S: TApplyModeSet): TJSONData;
function BlendModeOf(Data: TJSONData): TBlendMode;
function BlendModeSetOf(Data: TJSONData): TBlendModeSet;
function BlendModeSetToJSON(const S: TBlendModeSet): TJSONData;
function BorderStyleOf(Data: TJSONData): TBorderStyle;
function BorderStyleSetOf(Data: TJSONData): TBorderStyleSet;
function BorderStyleSetToJSON(const S: TBorderStyleSet): TJSONData;
function CharCaseOf(Data: TJSONData): TCharCase;
function CharCaseSetOf(Data: TJSONData): TCharCaseSet;
function CharCaseSetToJSON(const S: TCharCaseSet): TJSONData;
function ChartKindOf(Data: TJSONData): TChartKind;
function ChartKindSetOf(Data: TJSONData): TChartKindSet;
function ChartKindSetToJSON(const S: TChartKindSet): TJSONData;
function ChartSeriesFillOf(Data: TJSONData): TChartSeriesFill;
function ChartSeriesFillSetOf(Data: TJSONData): TChartSeriesFillSet;
function ChartSeriesFillSetToJSON(const S: TChartSeriesFillSet): TJSONData;
function ChartSeriesKindOf(Data: TJSONData): TChartSeriesKind;
function ChartSeriesKindSetOf(Data: TJSONData): TChartSeriesKindSet;
function ChartSeriesKindSetToJSON(const S: TChartSeriesKindSet): TJSONData;
function ChartSeriesPointOf(Data: TJSONData): TChartSeriesPoint;
function ChartSeriesPointSetOf(Data: TJSONData): TChartSeriesPointSet;
function ChartSeriesPointSetToJSON(const S: TChartSeriesPointSet): TJSONData;
function CloseActionOf(Data: TJSONData): TCloseAction;
function CloseActionSetOf(Data: TJSONData): TCloseActionSet;
function CloseActionSetToJSON(const S: TCloseActionSet): TJSONData;
function CursorShapeOf(Data: TJSONData): TCursorShape;
function CursorShapeSetOf(Data: TJSONData): TCursorShapeSet;
function CursorShapeSetToJSON(const S: TCursorShapeSet): TJSONData;
function DBChartStyleOf(Data: TJSONData): TDBChartStyle;
function DBChartStyleSetOf(Data: TJSONData): TDBChartStyleSet;
function DBChartStyleSetToJSON(const S: TDBChartStyleSet): TJSONData;
function DataGridStyleOf(Data: TJSONData): TDataGridStyle;
function DataGridStyleSetOf(Data: TJSONData): TDataGridStyleSet;
function DataGridStyleSetToJSON(const S: TDataGridStyleSet): TJSONData;
function FieldKindOf(Data: TJSONData): TFieldKind;
function FieldKindSetOf(Data: TJSONData): TFieldKindSet;
function FieldKindSetToJSON(const S: TFieldKindSet): TJSONData;
function FieldTypeOf(Data: TJSONData): TFieldType;
function FieldTypeSetOf(Data: TJSONData): TFieldTypeSet;
function FieldTypeSetToJSON(const S: TFieldTypeSet): TJSONData;
function FontWeightOf(Data: TJSONData): TFontWeight;
function FontWeightSetOf(Data: TJSONData): TFontWeightSet;
function FontWeightSetToJSON(const S: TFontWeightSet): TJSONData;
function FormOrientationOf(Data: TJSONData): TFormOrientation;
function FormOrientationSetOf(Data: TJSONData): TFormOrientationSet;
function FormOrientationSetToJSON(const S: TFormOrientationSet): TJSONData;
function FormPositionOf(Data: TJSONData): TFormPosition;
function FormPositionSetOf(Data: TJSONData): TFormPositionSet;
function FormPositionSetToJSON(const S: TFormPositionSet): TJSONData;
function FormPresentOf(Data: TJSONData): TFormPresent;
function FormPresentSetOf(Data: TJSONData): TFormPresentSet;
function FormPresentSetToJSON(const S: TFormPresentSet): TJSONData;
function FormStyleOf(Data: TJSONData): TFormStyle;
function FormStyleSetOf(Data: TJSONData): TFormStyleSet;
function FormStyleSetToJSON(const S: TFormStyleSet): TJSONData;
function GridTotalFunctionOf(Data: TJSONData): TGridTotalFunction;
function GridTotalFunctionSetOf(Data: TJSONData): TGridTotalFunctionSet;
function GridTotalFunctionSetToJSON(const S: TGridTotalFunctionSet): TJSONData;
function GroupBoxStyleOf(Data: TJSONData): TGroupBoxStyle;
function GroupBoxStyleSetOf(Data: TJSONData): TGroupBoxStyleSet;
function GroupBoxStyleSetToJSON(const S: TGroupBoxStyleSet): TJSONData;
function HorizontalWheelOf(Data: TJSONData): THorizontalWheel;
function HorizontalWheelSetOf(Data: TJSONData): THorizontalWheelSet;
function HorizontalWheelSetToJSON(const S: THorizontalWheelSet): TJSONData;
function ImageFitOf(Data: TJSONData): TImageFit;
function ImageFitSetOf(Data: TJSONData): TImageFitSet;
function ImageFitSetToJSON(const S: TImageFitSet): TJSONData;
function ImageStyleOf(Data: TJSONData): TImageStyle;
function ImageStyleSetOf(Data: TJSONData): TImageStyleSet;
function ImageStyleSetToJSON(const S: TImageStyleSet): TJSONData;
function LabelStyleOf(Data: TJSONData): TLabelStyle;
function LabelStyleSetOf(Data: TJSONData): TLabelStyleSet;
function LabelStyleSetToJSON(const S: TLabelStyleSet): TJSONData;
function LayerCachingOf(Data: TJSONData): TLayerCaching;
function LayerCachingSetOf(Data: TJSONData): TLayerCachingSet;
function LayerCachingSetToJSON(const S: TLayerCachingSet): TJSONData;
function LayoutStyleOf(Data: TJSONData): TLayoutStyle;
function LayoutStyleSetOf(Data: TJSONData): TLayoutStyleSet;
function LayoutStyleSetToJSON(const S: TLayoutStyleSet): TJSONData;
function LegendPositionOf(Data: TJSONData): TLegendPosition;
function LegendPositionSetOf(Data: TJSONData): TLegendPositionSet;
function LegendPositionSetToJSON(const S: TLegendPositionSet): TJSONData;
function OverscrollOf(Data: TJSONData): TOverscroll;
function OverscrollSetOf(Data: TJSONData): TOverscrollSet;
function OverscrollSetToJSON(const S: TOverscrollSet): TJSONData;
function PanelStyleOf(Data: TJSONData): TPanelStyle;
function PanelStyleSetOf(Data: TJSONData): TPanelStyleSet;
function PanelStyleSetToJSON(const S: TPanelStyleSet): TJSONData;
function RoundingOf(Data: TJSONData): TRounding;
function RoundingSetOf(Data: TJSONData): TRoundingSet;
function RoundingSetToJSON(const S: TRoundingSet): TJSONData;
function SelfAlignOf(Data: TJSONData): TSelfAlign;
function SelfAlignSetOf(Data: TJSONData): TSelfAlignSet;
function SelfAlignSetToJSON(const S: TSelfAlignSet): TJSONData;
function SplitterStyleOf(Data: TJSONData): TSplitterStyle;
function SplitterStyleSetOf(Data: TJSONData): TSplitterStyleSet;
function SplitterStyleSetToJSON(const S: TSplitterStyleSet): TJSONData;
function SvgStyleOf(Data: TJSONData): TSvgStyle;
function SvgStyleSetOf(Data: TJSONData): TSvgStyleSet;
function SvgStyleSetToJSON(const S: TSvgStyleSet): TJSONData;
function TextAlignOf(Data: TJSONData): TTextAlign;
function TextAlignSetOf(Data: TJSONData): TTextAlignSet;
function TextAlignSetToJSON(const S: TTextAlignSet): TJSONData;
function TextCaseOf(Data: TJSONData): TTextCase;
function TextCaseSetOf(Data: TJSONData): TTextCaseSet;
function TextCaseSetToJSON(const S: TTextCaseSet): TJSONData;
function ThemeNameOf(Data: TJSONData): TThemeName;
function ThemeNameSetOf(Data: TJSONData): TThemeNameSet;
function ThemeNameSetToJSON(const S: TThemeNameSet): TJSONData;
function TitleAlignOf(Data: TJSONData): TTitleAlign;
function TitleAlignSetOf(Data: TJSONData): TTitleAlignSet;
function TitleAlignSetToJSON(const S: TTitleAlignSet): TJSONData;
function TransitionOf(Data: TJSONData): TTransition;
function TransitionSetOf(Data: TJSONData): TTransitionSet;
function TransitionSetToJSON(const S: TTransitionSet): TJSONData;
function TrimmingOf(Data: TJSONData): TTrimming;
function TrimmingSetOf(Data: TJSONData): TTrimmingSet;
function TrimmingSetToJSON(const S: TTrimmingSet): TJSONData;
function ValueFormatOf(Data: TJSONData): TValueFormat;
function ValueFormatSetOf(Data: TJSONData): TValueFormatSet;
function ValueFormatSetToJSON(const S: TValueFormatSet): TJSONData;
function WindowCornersOf(Data: TJSONData): TWindowCorners;
function WindowCornersSetOf(Data: TJSONData): TWindowCornersSet;
function WindowCornersSetToJSON(const S: TWindowCornersSet): TJSONData;
function WindowShapeOf(Data: TJSONData): TWindowShape;
function WindowShapeSetOf(Data: TJSONData): TWindowShapeSet;
function WindowShapeSetToJSON(const S: TWindowShapeSet): TJSONData;
function WindowStateOf(Data: TJSONData): TWindowState;
function WindowStateSetOf(Data: TJSONData): TWindowStateSet;
function WindowStateSetToJSON(const S: TWindowStateSet): TJSONData;
function AylisJSONTexts(Data: TJSONData): TStringArray;
function AylisJSONNumbers(Data: TJSONData): TAylisDoubleArray;
function AylisTextsToJSON(const A: TStringArray): TJSONArray;
function AylisNumbersToJSON(const A: TAylisDoubleArray): TJSONArray;
function ChartSeriesFromJSON(Data: TJSONData): TChartSeries;
function ChartSeriesListFromJSON(Data: TJSONData): TChartSeriesArray;
function ChartSeriesFromText(Data: TJSONData): TChartSeries;
function ChartSeriesListFromText(Data: TJSONData): TChartSeriesArray;
function ChartSeriesToJSON(const R: TChartSeries): TJSONObject;
function ChartSeriesListToJSON(const A: array of TChartSeries): TJSONArray;
function ChartSeriesText(const R: TChartSeries): TJSONData;
function ChartSeriesListText(const A: TChartSeriesArray): TJSONData;
function ColumnSpecFromJSON(Data: TJSONData): TColumnSpec;
function ColumnSpecListFromJSON(Data: TJSONData): TColumnSpecArray;
function ColumnSpecFromText(Data: TJSONData): TColumnSpec;
function ColumnSpecListFromText(Data: TJSONData): TColumnSpecArray;
function ColumnSpecToJSON(const R: TColumnSpec): TJSONObject;
function ColumnSpecListToJSON(const A: array of TColumnSpec): TJSONArray;
function ColumnSpecText(const R: TColumnSpec): TJSONData;
function ColumnSpecListText(const A: TColumnSpecArray): TJSONData;
function FilterParamFromJSON(Data: TJSONData): TFilterParam;
function FilterParamListFromJSON(Data: TJSONData): TFilterParamArray;
function FilterParamFromText(Data: TJSONData): TFilterParam;
function FilterParamListFromText(Data: TJSONData): TFilterParamArray;
function FilterParamToJSON(const R: TFilterParam): TJSONObject;
function FilterParamListToJSON(const A: array of TFilterParam): TJSONArray;
function FilterParamText(const R: TFilterParam): TJSONData;
function FilterParamListText(const A: TFilterParamArray): TJSONData;
function GestureEventFromJSON(Data: TJSONData): TGestureEvent;
function GestureEventListFromJSON(Data: TJSONData): TGestureEventArray;
function GestureEventFromText(Data: TJSONData): TGestureEvent;
function GestureEventListFromText(Data: TJSONData): TGestureEventArray;
function GestureEventToJSON(const R: TGestureEvent): TJSONObject;
function GestureEventListToJSON(const A: array of TGestureEvent): TJSONArray;
function GestureEventText(const R: TGestureEvent): TJSONData;
function GestureEventListText(const A: TGestureEventArray): TJSONData;
function GridTotalFromJSON(Data: TJSONData): TGridTotal;
function GridTotalListFromJSON(Data: TJSONData): TGridTotalArray;
function GridTotalFromText(Data: TJSONData): TGridTotal;
function GridTotalListFromText(Data: TJSONData): TGridTotalArray;
function GridTotalToJSON(const R: TGridTotal): TJSONObject;
function GridTotalListToJSON(const A: array of TGridTotal): TJSONArray;
function GridTotalText(const R: TGridTotal): TJSONData;
function GridTotalListText(const A: TGridTotalArray): TJSONData;
function IndexDefFromJSON(Data: TJSONData): TIndexDef;
function IndexDefListFromJSON(Data: TJSONData): TIndexDefArray;
function IndexDefFromText(Data: TJSONData): TIndexDef;
function IndexDefListFromText(Data: TJSONData): TIndexDefArray;
function IndexDefToJSON(const R: TIndexDef): TJSONObject;
function IndexDefListToJSON(const A: array of TIndexDef): TJSONArray;
function IndexDefText(const R: TIndexDef): TJSONData;
function IndexDefListText(const A: TIndexDefArray): TJSONData;
function KeyEventFromJSON(Data: TJSONData): TKeyEvent;
function KeyEventListFromJSON(Data: TJSONData): TKeyEventArray;
function KeyEventFromText(Data: TJSONData): TKeyEvent;
function KeyEventListFromText(Data: TJSONData): TKeyEventArray;
function KeyEventToJSON(const R: TKeyEvent): TJSONObject;
function KeyEventListToJSON(const A: array of TKeyEvent): TJSONArray;
function KeyEventText(const R: TKeyEvent): TJSONData;
function KeyEventListText(const A: TKeyEventArray): TJSONData;
function MasterFieldFromJSON(Data: TJSONData): TMasterField;
function MasterFieldListFromJSON(Data: TJSONData): TMasterFieldArray;
function MasterFieldFromText(Data: TJSONData): TMasterField;
function MasterFieldListFromText(Data: TJSONData): TMasterFieldArray;
function MasterFieldToJSON(const R: TMasterField): TJSONObject;
function MasterFieldListToJSON(const A: array of TMasterField): TJSONArray;
function MasterFieldText(const R: TMasterField): TJSONData;
function MasterFieldListText(const A: TMasterFieldArray): TJSONData;
function PointerEventFromJSON(Data: TJSONData): TPointerEvent;
function PointerEventListFromJSON(Data: TJSONData): TPointerEventArray;
function PointerEventFromText(Data: TJSONData): TPointerEvent;
function PointerEventListFromText(Data: TJSONData): TPointerEventArray;
function PointerEventToJSON(const R: TPointerEvent): TJSONObject;
function PointerEventListToJSON(const A: array of TPointerEvent): TJSONArray;
function PointerEventText(const R: TPointerEvent): TJSONData;
function PointerEventListText(const A: TPointerEventArray): TJSONData;
function SwipeEventFromJSON(Data: TJSONData): TSwipeEvent;
function SwipeEventListFromJSON(Data: TJSONData): TSwipeEventArray;
function SwipeEventFromText(Data: TJSONData): TSwipeEvent;
function SwipeEventListFromText(Data: TJSONData): TSwipeEventArray;
function SwipeEventToJSON(const R: TSwipeEvent): TJSONObject;
function SwipeEventListToJSON(const A: array of TSwipeEvent): TJSONArray;
function SwipeEventText(const R: TSwipeEvent): TJSONData;
function SwipeEventListText(const A: TSwipeEventArray): TJSONData;
function WheelEventFromJSON(Data: TJSONData): TWheelEvent;
function WheelEventListFromJSON(Data: TJSONData): TWheelEventArray;
function WheelEventFromText(Data: TJSONData): TWheelEvent;
function WheelEventListFromText(Data: TJSONData): TWheelEventArray;
function WheelEventToJSON(const R: TWheelEvent): TJSONObject;
function WheelEventListToJSON(const A: array of TWheelEvent): TJSONArray;
function WheelEventText(const R: TWheelEvent): TJSONData;
function WheelEventListText(const A: TWheelEventArray): TJSONData;

implementation

function AsText(Data: TJSONData): string;
begin
  if (Data = nil) or (Data.JSONType = jtNull) then
    Result := ''
  else
    Result := Data.AsString;
end;

var
  AylisFormat: TFormatSettings;

function AsDate(Data: TJSONData): TDateTime;
var
  S: string;
  Y, M, D: Integer;
begin
  Result := 0;
  S := AsText(Data);
  if (Length(S) >= 10) and TryStrToInt(Copy(S, 1, 4), Y) and TryStrToInt(Copy(S, 6, 2), M) and TryStrToInt(Copy(S, 9, 2), D) then
    TryEncodeDate(Y, M, D, Result);
end;

function AsTime(Data: TJSONData): TDateTime;
var
  S: string;
  H, M, Sec: Integer;
begin
  Result := -1;
  S := AsText(Data);
  Sec := 0;
  if (Length(S) >= 5) and TryStrToInt(Copy(S, 1, 2), H) and TryStrToInt(Copy(S, 4, 2), M) and ((Length(S) < 8) or TryStrToInt(Copy(S, 7, 2), Sec)) then
    if not TryEncodeTime(H, M, Sec, 0, Result) then
      Result := -1;
end;

function AsDateTime(Data: TJSONData): TDateTime;
var
  S: string;
  T: TJSONString;
  Hora: TDateTime;
begin
  S := AsText(Data);
  T := TJSONString.Create(Copy(S, 1, 10));
  try
    Result := AsDate(T);
  finally
    T.Free;
  end;
  if (Result <> 0) and (Length(S) > 11) then
  begin
    T := TJSONString.Create(Copy(S, 12, 8));
    try
      Hora := AsTime(T);
    finally
      T.Free;
    end;
    if Hora >= 0 then
      Result := Result + Hora;
  end;
end;

function DateText(D: TDateTime): TJSONData;
begin
  if D = 0 then
    Result := TJSONString.Create('')
  else
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd', D));
end;

function TimeText(T: TDateTime): TJSONData;
begin
  if T < 0 then
    Result := TJSONString.Create('')
  else if SecondOf(T) = 0 then
    Result := TJSONString.Create(FormatDateTime('hh":"nn', T))
  else
    Result := TJSONString.Create(FormatDateTime('hh":"nn":"ss', T));
end;

function DateTimeText(D: TDateTime): TJSONData;
begin
  if D = 0 then
    Result := TJSONString.Create('')
  else if SecondOf(D) = 0 then
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd"T"hh":"nn', D))
  else
    Result := TJSONString.Create(FormatDateTime('yyyy"-"mm"-"dd"T"hh":"nn":"ss', D));
end;

function AsList(Data: TJSONData): TStringArray;
var
  Partes: TStringArray;
  P: string;
begin
  Result := nil;
  Partes := AsText(Data).Split(';');
  for P in Partes do
    if Trim(P) <> '' then
      Insert(Trim(P), Result, Length(Result));
end;

function AsNumbers(Data: TJSONData): TAylisDoubleArray;
var
  P: string;
  V: Double;
begin
  Result := nil;
  for P in AsList(Data) do
    if TryStrToFloat(P, V, AylisFormat) then
      Insert(V, Result, Length(Result));
end;

function AsColors(Data: TJSONData): TAylisColorArray;
var
  P: string;
begin
  Result := nil;
  for P in AsList(Data) do
    if (P <> '') and (P[1] = '#') then
      Insert(AylisColorHex(P), Result, Length(Result));
end;

function ListText(const A: TStringArray): TJSONData;
begin
  Result := TJSONString.Create(string.Join(';', A));
end;

function NumbersText(const A: TAylisDoubleArray): TJSONData;
var
  I: Integer;
  S: string;
begin
  S := '';
  for I := 0 to High(A) do
  begin
    if I > 0 then
      S := S + ';';
    S := S + FloatToStr(A[I], AylisFormat);
  end;
  Result := TJSONString.Create(S);
end;

function ColorsText(const A: TAylisColorArray): TJSONData;
var
  I: Integer;
  S: string;
  C: TJSONData;
begin
  S := '';
  for I := 0 to High(A) do
  begin
    if I > 0 then
      S := S + ';';
    C := AylisColorToJSON(A[I]);
    try
      S := S + C.AsString;
    finally
      C.Free;
    end;
  end;
  Result := TJSONString.Create(S);
end;


function AsDateRange(Data: TJSONData): TAylisDateRange;
var
  S: string;
  I: Integer;
  T: TJSONString;
begin
  S := AsText(Data);
  I := Pos(';', S);
  if I = 0 then
    I := Length(S) + 1;
  T := TJSONString.Create(Trim(Copy(S, 1, I - 1)));
  try
    Result.StartDate := AsDate(T);
  finally
    T.Free;
  end;
  T := TJSONString.Create(Trim(Copy(S, I + 1, MaxInt)));
  try
    Result.EndDate := AsDate(T);
  finally
    T.Free;
  end;
end;
const
  AlignNames: array[TAlign] of string = ('none', 'top', 'bottom', 'left', 'right', 'most_top', 'most_bottom', 'most_left', 'most_right', 'client', 'contents', 'center', 'vert_center', 'horz_center', 'horizontal', 'vertical', 'fit');

function AlignOf(Data: TJSONData): TAlign;
var
  V: TAlign;
begin
  Result := Low(TAlign);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TAlign) to High(TAlign) do
      if AlignNames[V] = Data.AsString then
        Exit(V);
end;

function AlignSetOf(Data: TJSONData): TAlignSet;
var
  I: Integer;
  V: TAlign;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TAlign) to High(TAlign) do
      if AlignNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function AlignSetToJSON(const S: TAlignSet): TJSONData;
var
  V: TAlign;
begin
  Result := TJSONArray.Create;
  for V := Low(TAlign) to High(TAlign) do
    if V in S then
      TJSONArray(Result).Add(AlignNames[V]);
end;

const
  AnchorNames: array[TAnchor] of string = ('left', 'top', 'right', 'bottom');

function AnchorOf(Data: TJSONData): TAnchor;
var
  V: TAnchor;
begin
  Result := Low(TAnchor);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TAnchor) to High(TAnchor) do
      if AnchorNames[V] = Data.AsString then
        Exit(V);
end;

function AnchorSetOf(Data: TJSONData): TAnchorSet;
var
  I: Integer;
  V: TAnchor;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TAnchor) to High(TAnchor) do
      if AnchorNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function AnchorSetToJSON(const S: TAnchorSet): TJSONData;
var
  V: TAnchor;
begin
  Result := TJSONArray.Create;
  for V := Low(TAnchor) to High(TAnchor) do
    if V in S then
      TJSONArray(Result).Add(AnchorNames[V]);
end;

const
  ApplyModeNames: array[TApplyMode] of string = ('cached', 'on_post');

function ApplyModeOf(Data: TJSONData): TApplyMode;
var
  V: TApplyMode;
begin
  Result := Low(TApplyMode);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TApplyMode) to High(TApplyMode) do
      if ApplyModeNames[V] = Data.AsString then
        Exit(V);
end;

function ApplyModeSetOf(Data: TJSONData): TApplyModeSet;
var
  I: Integer;
  V: TApplyMode;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TApplyMode) to High(TApplyMode) do
      if ApplyModeNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ApplyModeSetToJSON(const S: TApplyModeSet): TJSONData;
var
  V: TApplyMode;
begin
  Result := TJSONArray.Create;
  for V := Low(TApplyMode) to High(TApplyMode) do
    if V in S then
      TJSONArray(Result).Add(ApplyModeNames[V]);
end;

const
  BlendModeNames: array[TBlendMode] of string = ('normal', 'multiply', 'screen', 'overlay', 'darken', 'lighten', 'color_dodge', 'color_burn', 'hard_light', 'soft_light', 'difference', 'exclusion', 'hue', 'saturation', 'color', 'luminosity');

function BlendModeOf(Data: TJSONData): TBlendMode;
var
  V: TBlendMode;
begin
  Result := Low(TBlendMode);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TBlendMode) to High(TBlendMode) do
      if BlendModeNames[V] = Data.AsString then
        Exit(V);
end;

function BlendModeSetOf(Data: TJSONData): TBlendModeSet;
var
  I: Integer;
  V: TBlendMode;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TBlendMode) to High(TBlendMode) do
      if BlendModeNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function BlendModeSetToJSON(const S: TBlendModeSet): TJSONData;
var
  V: TBlendMode;
begin
  Result := TJSONArray.Create;
  for V := Low(TBlendMode) to High(TBlendMode) do
    if V in S then
      TJSONArray(Result).Add(BlendModeNames[V]);
end;

const
  BorderStyleNames: array[TBorderStyle] of string = ('sizeable', 'single', 'none', 'custom');

function BorderStyleOf(Data: TJSONData): TBorderStyle;
var
  V: TBorderStyle;
begin
  Result := Low(TBorderStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TBorderStyle) to High(TBorderStyle) do
      if BorderStyleNames[V] = Data.AsString then
        Exit(V);
end;

function BorderStyleSetOf(Data: TJSONData): TBorderStyleSet;
var
  I: Integer;
  V: TBorderStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TBorderStyle) to High(TBorderStyle) do
      if BorderStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function BorderStyleSetToJSON(const S: TBorderStyleSet): TJSONData;
var
  V: TBorderStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TBorderStyle) to High(TBorderStyle) do
    if V in S then
      TJSONArray(Result).Add(BorderStyleNames[V]);
end;

const
  CharCaseNames: array[TCharCase] of string = ('normal', 'upper', 'lower');

function CharCaseOf(Data: TJSONData): TCharCase;
var
  V: TCharCase;
begin
  Result := Low(TCharCase);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TCharCase) to High(TCharCase) do
      if CharCaseNames[V] = Data.AsString then
        Exit(V);
end;

function CharCaseSetOf(Data: TJSONData): TCharCaseSet;
var
  I: Integer;
  V: TCharCase;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TCharCase) to High(TCharCase) do
      if CharCaseNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function CharCaseSetToJSON(const S: TCharCaseSet): TJSONData;
var
  V: TCharCase;
begin
  Result := TJSONArray.Create;
  for V := Low(TCharCase) to High(TCharCase) do
    if V in S then
      TJSONArray(Result).Add(CharCaseNames[V]);
end;

const
  ChartKindNames: array[TChartKind] of string = ('line', 'bar', 'area', 'pie', 'donut', 'sparkline', 'heatmap', 'scatter', 'stacked_bar', 'waterfall', 'funnel', 'treemap', 'radar', 'horizontal_bar', 'bubble', 'polar_area', 'stacked_line', 'stacked_area', 'mixed', 'sankey', 'waffle');

function ChartKindOf(Data: TJSONData): TChartKind;
var
  V: TChartKind;
begin
  Result := Low(TChartKind);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TChartKind) to High(TChartKind) do
      if ChartKindNames[V] = Data.AsString then
        Exit(V);
end;

function ChartKindSetOf(Data: TJSONData): TChartKindSet;
var
  I: Integer;
  V: TChartKind;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TChartKind) to High(TChartKind) do
      if ChartKindNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ChartKindSetToJSON(const S: TChartKindSet): TJSONData;
var
  V: TChartKind;
begin
  Result := TJSONArray.Create;
  for V := Low(TChartKind) to High(TChartKind) do
    if V in S then
      TJSONArray(Result).Add(ChartKindNames[V]);
end;

const
  ChartSeriesFillNames: array[TChartSeriesFill] of string = ('none', 'origin', 'start', 'end', 'series');

function ChartSeriesFillOf(Data: TJSONData): TChartSeriesFill;
var
  V: TChartSeriesFill;
begin
  Result := Low(TChartSeriesFill);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TChartSeriesFill) to High(TChartSeriesFill) do
      if ChartSeriesFillNames[V] = Data.AsString then
        Exit(V);
end;

function ChartSeriesFillSetOf(Data: TJSONData): TChartSeriesFillSet;
var
  I: Integer;
  V: TChartSeriesFill;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TChartSeriesFill) to High(TChartSeriesFill) do
      if ChartSeriesFillNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ChartSeriesFillSetToJSON(const S: TChartSeriesFillSet): TJSONData;
var
  V: TChartSeriesFill;
begin
  Result := TJSONArray.Create;
  for V := Low(TChartSeriesFill) to High(TChartSeriesFill) do
    if V in S then
      TJSONArray(Result).Add(ChartSeriesFillNames[V]);
end;

const
  ChartSeriesKindNames: array[TChartSeriesKind] of string = ('line', 'bar', 'area', 'scatter');

function ChartSeriesKindOf(Data: TJSONData): TChartSeriesKind;
var
  V: TChartSeriesKind;
begin
  Result := Low(TChartSeriesKind);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TChartSeriesKind) to High(TChartSeriesKind) do
      if ChartSeriesKindNames[V] = Data.AsString then
        Exit(V);
end;

function ChartSeriesKindSetOf(Data: TJSONData): TChartSeriesKindSet;
var
  I: Integer;
  V: TChartSeriesKind;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TChartSeriesKind) to High(TChartSeriesKind) do
      if ChartSeriesKindNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ChartSeriesKindSetToJSON(const S: TChartSeriesKindSet): TJSONData;
var
  V: TChartSeriesKind;
begin
  Result := TJSONArray.Create;
  for V := Low(TChartSeriesKind) to High(TChartSeriesKind) do
    if V in S then
      TJSONArray(Result).Add(ChartSeriesKindNames[V]);
end;

const
  ChartSeriesPointNames: array[TChartSeriesPoint] of string = ('circle', 'square', 'triangle', 'diamond', 'cross', 'none');

function ChartSeriesPointOf(Data: TJSONData): TChartSeriesPoint;
var
  V: TChartSeriesPoint;
begin
  Result := Low(TChartSeriesPoint);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TChartSeriesPoint) to High(TChartSeriesPoint) do
      if ChartSeriesPointNames[V] = Data.AsString then
        Exit(V);
end;

function ChartSeriesPointSetOf(Data: TJSONData): TChartSeriesPointSet;
var
  I: Integer;
  V: TChartSeriesPoint;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TChartSeriesPoint) to High(TChartSeriesPoint) do
      if ChartSeriesPointNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ChartSeriesPointSetToJSON(const S: TChartSeriesPointSet): TJSONData;
var
  V: TChartSeriesPoint;
begin
  Result := TJSONArray.Create;
  for V := Low(TChartSeriesPoint) to High(TChartSeriesPoint) do
    if V in S then
      TJSONArray(Result).Add(ChartSeriesPointNames[V]);
end;

const
  CloseActionNames: array[TCloseAction] of string = ('hide', 'free');

function CloseActionOf(Data: TJSONData): TCloseAction;
var
  V: TCloseAction;
begin
  Result := Low(TCloseAction);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TCloseAction) to High(TCloseAction) do
      if CloseActionNames[V] = Data.AsString then
        Exit(V);
end;

function CloseActionSetOf(Data: TJSONData): TCloseActionSet;
var
  I: Integer;
  V: TCloseAction;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TCloseAction) to High(TCloseAction) do
      if CloseActionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function CloseActionSetToJSON(const S: TCloseActionSet): TJSONData;
var
  V: TCloseAction;
begin
  Result := TJSONArray.Create;
  for V := Low(TCloseAction) to High(TCloseAction) do
    if V in S then
      TJSONArray(Result).Add(CloseActionNames[V]);
end;

const
  CursorShapeNames: array[TCursorShape] of string = ('default', 'pointer', 'text', 'wait', 'crosshair', 'move', 'not_allowed', 'resize_horizontal', 'resize_vertical', 'resize_ne_sw', 'resize_nw_se', 'grab', 'grabbing', 'none');

function CursorShapeOf(Data: TJSONData): TCursorShape;
var
  V: TCursorShape;
begin
  Result := Low(TCursorShape);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TCursorShape) to High(TCursorShape) do
      if CursorShapeNames[V] = Data.AsString then
        Exit(V);
end;

function CursorShapeSetOf(Data: TJSONData): TCursorShapeSet;
var
  I: Integer;
  V: TCursorShape;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TCursorShape) to High(TCursorShape) do
      if CursorShapeNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function CursorShapeSetToJSON(const S: TCursorShapeSet): TJSONData;
var
  V: TCursorShape;
begin
  Result := TJSONArray.Create;
  for V := Low(TCursorShape) to High(TCursorShape) do
    if V in S then
      TJSONArray(Result).Add(CursorShapeNames[V]);
end;

const
  DBChartStyleNames: array[TDBChartStyle] of string = ('');

function DBChartStyleOf(Data: TJSONData): TDBChartStyle;
var
  V: TDBChartStyle;
begin
  Result := Low(TDBChartStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TDBChartStyle) to High(TDBChartStyle) do
      if DBChartStyleNames[V] = Data.AsString then
        Exit(V);
end;

function DBChartStyleSetOf(Data: TJSONData): TDBChartStyleSet;
var
  I: Integer;
  V: TDBChartStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TDBChartStyle) to High(TDBChartStyle) do
      if DBChartStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function DBChartStyleSetToJSON(const S: TDBChartStyleSet): TJSONData;
var
  V: TDBChartStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TDBChartStyle) to High(TDBChartStyle) do
    if V in S then
      TJSONArray(Result).Add(DBChartStyleNames[V]);
end;

const
  DataGridStyleNames: array[TDataGridStyle] of string = ('');

function DataGridStyleOf(Data: TJSONData): TDataGridStyle;
var
  V: TDataGridStyle;
begin
  Result := Low(TDataGridStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TDataGridStyle) to High(TDataGridStyle) do
      if DataGridStyleNames[V] = Data.AsString then
        Exit(V);
end;

function DataGridStyleSetOf(Data: TJSONData): TDataGridStyleSet;
var
  I: Integer;
  V: TDataGridStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TDataGridStyle) to High(TDataGridStyle) do
      if DataGridStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function DataGridStyleSetToJSON(const S: TDataGridStyleSet): TJSONData;
var
  V: TDataGridStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TDataGridStyle) to High(TDataGridStyle) do
    if V in S then
      TJSONArray(Result).Add(DataGridStyleNames[V]);
end;

const
  FieldKindNames: array[TFieldKind] of string = ('data', 'calculated', 'lookup');

function FieldKindOf(Data: TJSONData): TFieldKind;
var
  V: TFieldKind;
begin
  Result := Low(TFieldKind);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFieldKind) to High(TFieldKind) do
      if FieldKindNames[V] = Data.AsString then
        Exit(V);
end;

function FieldKindSetOf(Data: TJSONData): TFieldKindSet;
var
  I: Integer;
  V: TFieldKind;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFieldKind) to High(TFieldKind) do
      if FieldKindNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FieldKindSetToJSON(const S: TFieldKindSet): TJSONData;
var
  V: TFieldKind;
begin
  Result := TJSONArray.Create;
  for V := Low(TFieldKind) to High(TFieldKind) do
    if V in S then
      TJSONArray(Result).Add(FieldKindNames[V]);
end;

const
  FieldTypeNames: array[TFieldType] of string = ('string', 'memo', 'integer', 'float', 'currency', 'boolean', 'date', 'time', 'datetime', 'blob', 'auto_inc', 'timestamp');

function FieldTypeOf(Data: TJSONData): TFieldType;
var
  V: TFieldType;
begin
  Result := Low(TFieldType);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFieldType) to High(TFieldType) do
      if FieldTypeNames[V] = Data.AsString then
        Exit(V);
end;

function FieldTypeSetOf(Data: TJSONData): TFieldTypeSet;
var
  I: Integer;
  V: TFieldType;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFieldType) to High(TFieldType) do
      if FieldTypeNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FieldTypeSetToJSON(const S: TFieldTypeSet): TJSONData;
var
  V: TFieldType;
begin
  Result := TJSONArray.Create;
  for V := Low(TFieldType) to High(TFieldType) do
    if V in S then
      TJSONArray(Result).Add(FieldTypeNames[V]);
end;

const
  FontWeightNames: array[TFontWeight] of string = ('default', 'thin', 'light', 'normal', 'medium', 'semibold', 'bold', 'black');

function FontWeightOf(Data: TJSONData): TFontWeight;
var
  V: TFontWeight;
begin
  Result := Low(TFontWeight);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFontWeight) to High(TFontWeight) do
      if FontWeightNames[V] = Data.AsString then
        Exit(V);
end;

function FontWeightSetOf(Data: TJSONData): TFontWeightSet;
var
  I: Integer;
  V: TFontWeight;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFontWeight) to High(TFontWeight) do
      if FontWeightNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FontWeightSetToJSON(const S: TFontWeightSet): TJSONData;
var
  V: TFontWeight;
begin
  Result := TJSONArray.Create;
  for V := Low(TFontWeight) to High(TFontWeight) do
    if V in S then
      TJSONArray(Result).Add(FontWeightNames[V]);
end;

const
  FormOrientationNames: array[TFormOrientation] of string = ('project', 'any', 'portrait', 'landscape', 'sensor_portrait', 'sensor_landscape');

function FormOrientationOf(Data: TJSONData): TFormOrientation;
var
  V: TFormOrientation;
begin
  Result := Low(TFormOrientation);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFormOrientation) to High(TFormOrientation) do
      if FormOrientationNames[V] = Data.AsString then
        Exit(V);
end;

function FormOrientationSetOf(Data: TJSONData): TFormOrientationSet;
var
  I: Integer;
  V: TFormOrientation;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFormOrientation) to High(TFormOrientation) do
      if FormOrientationNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FormOrientationSetToJSON(const S: TFormOrientationSet): TJSONData;
var
  V: TFormOrientation;
begin
  Result := TJSONArray.Create;
  for V := Low(TFormOrientation) to High(TFormOrientation) do
    if V in S then
      TJSONArray(Result).Add(FormOrientationNames[V]);
end;

const
  FormPositionNames: array[TFormPosition] of string = ('center', 'screen_center', 'designed', 'default', 'top_left', 'top_right', 'bottom_left', 'bottom_right');

function FormPositionOf(Data: TJSONData): TFormPosition;
var
  V: TFormPosition;
begin
  Result := Low(TFormPosition);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFormPosition) to High(TFormPosition) do
      if FormPositionNames[V] = Data.AsString then
        Exit(V);
end;

function FormPositionSetOf(Data: TJSONData): TFormPositionSet;
var
  I: Integer;
  V: TFormPosition;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFormPosition) to High(TFormPosition) do
      if FormPositionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FormPositionSetToJSON(const S: TFormPositionSet): TJSONData;
var
  V: TFormPosition;
begin
  Result := TJSONArray.Create;
  for V := Low(TFormPosition) to High(TFormPosition) do
    if V in S then
      TJSONArray(Result).Add(FormPositionNames[V]);
end;

const
  FormPresentNames: array[TFormPresent] of string = ('project', 'window', 'composition', 'composed');

function FormPresentOf(Data: TJSONData): TFormPresent;
var
  V: TFormPresent;
begin
  Result := Low(TFormPresent);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFormPresent) to High(TFormPresent) do
      if FormPresentNames[V] = Data.AsString then
        Exit(V);
end;

function FormPresentSetOf(Data: TJSONData): TFormPresentSet;
var
  I: Integer;
  V: TFormPresent;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFormPresent) to High(TFormPresent) do
      if FormPresentNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FormPresentSetToJSON(const S: TFormPresentSet): TJSONData;
var
  V: TFormPresent;
begin
  Result := TJSONArray.Create;
  for V := Low(TFormPresent) to High(TFormPresent) do
    if V in S then
      TJSONArray(Result).Add(FormPresentNames[V]);
end;

const
  FormStyleNames: array[TFormStyle] of string = ('');

function FormStyleOf(Data: TJSONData): TFormStyle;
var
  V: TFormStyle;
begin
  Result := Low(TFormStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TFormStyle) to High(TFormStyle) do
      if FormStyleNames[V] = Data.AsString then
        Exit(V);
end;

function FormStyleSetOf(Data: TJSONData): TFormStyleSet;
var
  I: Integer;
  V: TFormStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TFormStyle) to High(TFormStyle) do
      if FormStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function FormStyleSetToJSON(const S: TFormStyleSet): TJSONData;
var
  V: TFormStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TFormStyle) to High(TFormStyle) do
    if V in S then
      TJSONArray(Result).Add(FormStyleNames[V]);
end;

const
  GridTotalFunctionNames: array[TGridTotalFunction] of string = ('sum', 'avg', 'min', 'max', 'count');

function GridTotalFunctionOf(Data: TJSONData): TGridTotalFunction;
var
  V: TGridTotalFunction;
begin
  Result := Low(TGridTotalFunction);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TGridTotalFunction) to High(TGridTotalFunction) do
      if GridTotalFunctionNames[V] = Data.AsString then
        Exit(V);
end;

function GridTotalFunctionSetOf(Data: TJSONData): TGridTotalFunctionSet;
var
  I: Integer;
  V: TGridTotalFunction;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TGridTotalFunction) to High(TGridTotalFunction) do
      if GridTotalFunctionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function GridTotalFunctionSetToJSON(const S: TGridTotalFunctionSet): TJSONData;
var
  V: TGridTotalFunction;
begin
  Result := TJSONArray.Create;
  for V := Low(TGridTotalFunction) to High(TGridTotalFunction) do
    if V in S then
      TJSONArray(Result).Add(GridTotalFunctionNames[V]);
end;

const
  GroupBoxStyleNames: array[TGroupBoxStyle] of string = ('');

function GroupBoxStyleOf(Data: TJSONData): TGroupBoxStyle;
var
  V: TGroupBoxStyle;
begin
  Result := Low(TGroupBoxStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TGroupBoxStyle) to High(TGroupBoxStyle) do
      if GroupBoxStyleNames[V] = Data.AsString then
        Exit(V);
end;

function GroupBoxStyleSetOf(Data: TJSONData): TGroupBoxStyleSet;
var
  I: Integer;
  V: TGroupBoxStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TGroupBoxStyle) to High(TGroupBoxStyle) do
      if GroupBoxStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function GroupBoxStyleSetToJSON(const S: TGroupBoxStyleSet): TJSONData;
var
  V: TGroupBoxStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TGroupBoxStyle) to High(TGroupBoxStyle) do
    if V in S then
      TJSONArray(Result).Add(GroupBoxStyleNames[V]);
end;

const
  HorizontalWheelNames: array[THorizontalWheel] of string = ('shift', 'always', 'none');

function HorizontalWheelOf(Data: TJSONData): THorizontalWheel;
var
  V: THorizontalWheel;
begin
  Result := Low(THorizontalWheel);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(THorizontalWheel) to High(THorizontalWheel) do
      if HorizontalWheelNames[V] = Data.AsString then
        Exit(V);
end;

function HorizontalWheelSetOf(Data: TJSONData): THorizontalWheelSet;
var
  I: Integer;
  V: THorizontalWheel;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(THorizontalWheel) to High(THorizontalWheel) do
      if HorizontalWheelNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function HorizontalWheelSetToJSON(const S: THorizontalWheelSet): TJSONData;
var
  V: THorizontalWheel;
begin
  Result := TJSONArray.Create;
  for V := Low(THorizontalWheel) to High(THorizontalWheel) do
    if V in S then
      TJSONArray(Result).Add(HorizontalWheelNames[V]);
end;

const
  ImageFitNames: array[TImageFit] of string = ('contain', 'cover', 'stretch', 'center', 'original');

function ImageFitOf(Data: TJSONData): TImageFit;
var
  V: TImageFit;
begin
  Result := Low(TImageFit);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TImageFit) to High(TImageFit) do
      if ImageFitNames[V] = Data.AsString then
        Exit(V);
end;

function ImageFitSetOf(Data: TJSONData): TImageFitSet;
var
  I: Integer;
  V: TImageFit;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TImageFit) to High(TImageFit) do
      if ImageFitNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ImageFitSetToJSON(const S: TImageFitSet): TJSONData;
var
  V: TImageFit;
begin
  Result := TJSONArray.Create;
  for V := Low(TImageFit) to High(TImageFit) do
    if V in S then
      TJSONArray(Result).Add(ImageFitNames[V]);
end;

const
  ImageStyleNames: array[TImageStyle] of string = ('');

function ImageStyleOf(Data: TJSONData): TImageStyle;
var
  V: TImageStyle;
begin
  Result := Low(TImageStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TImageStyle) to High(TImageStyle) do
      if ImageStyleNames[V] = Data.AsString then
        Exit(V);
end;

function ImageStyleSetOf(Data: TJSONData): TImageStyleSet;
var
  I: Integer;
  V: TImageStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TImageStyle) to High(TImageStyle) do
      if ImageStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ImageStyleSetToJSON(const S: TImageStyleSet): TJSONData;
var
  V: TImageStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TImageStyle) to High(TImageStyle) do
    if V in S then
      TJSONArray(Result).Add(ImageStyleNames[V]);
end;

const
  LabelStyleNames: array[TLabelStyle] of string = ('', 'caption', 'error', 'heading', 'kbd', 'link', 'mono', 'muted', 'title');

function LabelStyleOf(Data: TJSONData): TLabelStyle;
var
  V: TLabelStyle;
begin
  Result := Low(TLabelStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TLabelStyle) to High(TLabelStyle) do
      if LabelStyleNames[V] = Data.AsString then
        Exit(V);
end;

function LabelStyleSetOf(Data: TJSONData): TLabelStyleSet;
var
  I: Integer;
  V: TLabelStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TLabelStyle) to High(TLabelStyle) do
      if LabelStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function LabelStyleSetToJSON(const S: TLabelStyleSet): TJSONData;
var
  V: TLabelStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TLabelStyle) to High(TLabelStyle) do
    if V in S then
      TJSONArray(Result).Add(LabelStyleNames[V]);
end;

const
  LayerCachingNames: array[TLayerCaching] of string = ('project', 'on', 'off');

function LayerCachingOf(Data: TJSONData): TLayerCaching;
var
  V: TLayerCaching;
begin
  Result := Low(TLayerCaching);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TLayerCaching) to High(TLayerCaching) do
      if LayerCachingNames[V] = Data.AsString then
        Exit(V);
end;

function LayerCachingSetOf(Data: TJSONData): TLayerCachingSet;
var
  I: Integer;
  V: TLayerCaching;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TLayerCaching) to High(TLayerCaching) do
      if LayerCachingNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function LayerCachingSetToJSON(const S: TLayerCachingSet): TJSONData;
var
  V: TLayerCaching;
begin
  Result := TJSONArray.Create;
  for V := Low(TLayerCaching) to High(TLayerCaching) do
    if V in S then
      TJSONArray(Result).Add(LayerCachingNames[V]);
end;

const
  LayoutStyleNames: array[TLayoutStyle] of string = ('');

function LayoutStyleOf(Data: TJSONData): TLayoutStyle;
var
  V: TLayoutStyle;
begin
  Result := Low(TLayoutStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TLayoutStyle) to High(TLayoutStyle) do
      if LayoutStyleNames[V] = Data.AsString then
        Exit(V);
end;

function LayoutStyleSetOf(Data: TJSONData): TLayoutStyleSet;
var
  I: Integer;
  V: TLayoutStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TLayoutStyle) to High(TLayoutStyle) do
      if LayoutStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function LayoutStyleSetToJSON(const S: TLayoutStyleSet): TJSONData;
var
  V: TLayoutStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TLayoutStyle) to High(TLayoutStyle) do
    if V in S then
      TJSONArray(Result).Add(LayoutStyleNames[V]);
end;

const
  LegendPositionNames: array[TLegendPosition] of string = ('top_right', 'top_left', 'top', 'bottom');

function LegendPositionOf(Data: TJSONData): TLegendPosition;
var
  V: TLegendPosition;
begin
  Result := Low(TLegendPosition);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TLegendPosition) to High(TLegendPosition) do
      if LegendPositionNames[V] = Data.AsString then
        Exit(V);
end;

function LegendPositionSetOf(Data: TJSONData): TLegendPositionSet;
var
  I: Integer;
  V: TLegendPosition;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TLegendPosition) to High(TLegendPosition) do
      if LegendPositionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function LegendPositionSetToJSON(const S: TLegendPositionSet): TJSONData;
var
  V: TLegendPosition;
begin
  Result := TJSONArray.Create;
  for V := Low(TLegendPosition) to High(TLegendPosition) do
    if V in S then
      TJSONArray(Result).Add(LegendPositionNames[V]);
end;

const
  OverscrollNames: array[TOverscroll] of string = ('auto', 'glow', 'stretch', 'bounce', 'none');

function OverscrollOf(Data: TJSONData): TOverscroll;
var
  V: TOverscroll;
begin
  Result := Low(TOverscroll);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TOverscroll) to High(TOverscroll) do
      if OverscrollNames[V] = Data.AsString then
        Exit(V);
end;

function OverscrollSetOf(Data: TJSONData): TOverscrollSet;
var
  I: Integer;
  V: TOverscroll;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TOverscroll) to High(TOverscroll) do
      if OverscrollNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function OverscrollSetToJSON(const S: TOverscrollSet): TJSONData;
var
  V: TOverscroll;
begin
  Result := TJSONArray.Create;
  for V := Low(TOverscroll) to High(TOverscroll) do
    if V in S then
      TJSONArray(Result).Add(OverscrollNames[V]);
end;

const
  PanelStyleNames: array[TPanelStyle] of string = ('', 'appbar', 'card', 'filled', 'flat', 'glass', 'outlined', 'overlay', 'reveal', 'sidebar');

function PanelStyleOf(Data: TJSONData): TPanelStyle;
var
  V: TPanelStyle;
begin
  Result := Low(TPanelStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TPanelStyle) to High(TPanelStyle) do
      if PanelStyleNames[V] = Data.AsString then
        Exit(V);
end;

function PanelStyleSetOf(Data: TJSONData): TPanelStyleSet;
var
  I: Integer;
  V: TPanelStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TPanelStyle) to High(TPanelStyle) do
      if PanelStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function PanelStyleSetToJSON(const S: TPanelStyleSet): TJSONData;
var
  V: TPanelStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TPanelStyle) to High(TPanelStyle) do
    if V in S then
      TJSONArray(Result).Add(PanelStyleNames[V]);
end;

const
  RoundingNames: array[TRounding] of string = ('half_up', 'half_even', 'truncate', 'up');

function RoundingOf(Data: TJSONData): TRounding;
var
  V: TRounding;
begin
  Result := Low(TRounding);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TRounding) to High(TRounding) do
      if RoundingNames[V] = Data.AsString then
        Exit(V);
end;

function RoundingSetOf(Data: TJSONData): TRoundingSet;
var
  I: Integer;
  V: TRounding;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TRounding) to High(TRounding) do
      if RoundingNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function RoundingSetToJSON(const S: TRoundingSet): TJSONData;
var
  V: TRounding;
begin
  Result := TJSONArray.Create;
  for V := Low(TRounding) to High(TRounding) do
    if V in S then
      TJSONArray(Result).Add(RoundingNames[V]);
end;

const
  SelfAlignNames: array[TSelfAlign] of string = ('stretch', 'start', 'center', 'end');

function SelfAlignOf(Data: TJSONData): TSelfAlign;
var
  V: TSelfAlign;
begin
  Result := Low(TSelfAlign);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TSelfAlign) to High(TSelfAlign) do
      if SelfAlignNames[V] = Data.AsString then
        Exit(V);
end;

function SelfAlignSetOf(Data: TJSONData): TSelfAlignSet;
var
  I: Integer;
  V: TSelfAlign;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TSelfAlign) to High(TSelfAlign) do
      if SelfAlignNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function SelfAlignSetToJSON(const S: TSelfAlignSet): TJSONData;
var
  V: TSelfAlign;
begin
  Result := TJSONArray.Create;
  for V := Low(TSelfAlign) to High(TSelfAlign) do
    if V in S then
      TJSONArray(Result).Add(SelfAlignNames[V]);
end;

const
  SplitterStyleNames: array[TSplitterStyle] of string = ('');

function SplitterStyleOf(Data: TJSONData): TSplitterStyle;
var
  V: TSplitterStyle;
begin
  Result := Low(TSplitterStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TSplitterStyle) to High(TSplitterStyle) do
      if SplitterStyleNames[V] = Data.AsString then
        Exit(V);
end;

function SplitterStyleSetOf(Data: TJSONData): TSplitterStyleSet;
var
  I: Integer;
  V: TSplitterStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TSplitterStyle) to High(TSplitterStyle) do
      if SplitterStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function SplitterStyleSetToJSON(const S: TSplitterStyleSet): TJSONData;
var
  V: TSplitterStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TSplitterStyle) to High(TSplitterStyle) do
    if V in S then
      TJSONArray(Result).Add(SplitterStyleNames[V]);
end;

const
  SvgStyleNames: array[TSvgStyle] of string = ('');

function SvgStyleOf(Data: TJSONData): TSvgStyle;
var
  V: TSvgStyle;
begin
  Result := Low(TSvgStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TSvgStyle) to High(TSvgStyle) do
      if SvgStyleNames[V] = Data.AsString then
        Exit(V);
end;

function SvgStyleSetOf(Data: TJSONData): TSvgStyleSet;
var
  I: Integer;
  V: TSvgStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TSvgStyle) to High(TSvgStyle) do
      if SvgStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function SvgStyleSetToJSON(const S: TSvgStyleSet): TJSONData;
var
  V: TSvgStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TSvgStyle) to High(TSvgStyle) do
    if V in S then
      TJSONArray(Result).Add(SvgStyleNames[V]);
end;

const
  TextAlignNames: array[TTextAlign] of string = ('leading', 'center', 'trailing', 'justify');

function TextAlignOf(Data: TJSONData): TTextAlign;
var
  V: TTextAlign;
begin
  Result := Low(TTextAlign);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTextAlign) to High(TTextAlign) do
      if TextAlignNames[V] = Data.AsString then
        Exit(V);
end;

function TextAlignSetOf(Data: TJSONData): TTextAlignSet;
var
  I: Integer;
  V: TTextAlign;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTextAlign) to High(TTextAlign) do
      if TextAlignNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TextAlignSetToJSON(const S: TTextAlignSet): TJSONData;
var
  V: TTextAlign;
begin
  Result := TJSONArray.Create;
  for V := Low(TTextAlign) to High(TTextAlign) do
    if V in S then
      TJSONArray(Result).Add(TextAlignNames[V]);
end;

const
  TextCaseNames: array[TTextCase] of string = ('none', 'upper', 'lower');

function TextCaseOf(Data: TJSONData): TTextCase;
var
  V: TTextCase;
begin
  Result := Low(TTextCase);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTextCase) to High(TTextCase) do
      if TextCaseNames[V] = Data.AsString then
        Exit(V);
end;

function TextCaseSetOf(Data: TJSONData): TTextCaseSet;
var
  I: Integer;
  V: TTextCase;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTextCase) to High(TTextCase) do
      if TextCaseNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TextCaseSetToJSON(const S: TTextCaseSet): TJSONData;
var
  V: TTextCase;
begin
  Result := TJSONArray.Create;
  for V := Low(TTextCase) to High(TTextCase) do
    if V in S then
      TJSONArray(Result).Add(TextCaseNames[V]);
end;

const
  ThemeNameNames: array[TThemeName] of string = ('', 'Aylis Light', 'Aylis Dark', 'Material 3 Light', 'Material 3 Dark', 'Material 2 Light', 'Material 2 Dark', 'Android 7 Light', 'Android 7 Dark', 'iOS 26 Light', 'iOS 26 Dark', 'iOS 13 Light', 'iOS 13 Dark', 'iOS 7 Light', 'WinUI 3 Light', 'WinUI 3 Dark', 'Windows 11 Light', 'Windows 11 Dark', 'Windows 10 Light', 'Windows 10 Dark', 'Win32', 'Windows 7', 'Windows Phone Dark', 'Windows Phone Light', 'Vuetify 3 Light', 'Vuetify 3 Dark', 'Bootstrap 5 Light', 'Bootstrap 5 Dark', 'shadcn/ui Light', 'shadcn/ui Dark', 'Ant Design 5 Light', 'Ant Design 5 Dark', 'Chakra UI Light', 'Chakra UI Dark', 'Quasar 2 Light', 'Quasar 2 Dark');

function ThemeNameOf(Data: TJSONData): TThemeName;
var
  V: TThemeName;
begin
  Result := Low(TThemeName);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TThemeName) to High(TThemeName) do
      if ThemeNameNames[V] = Data.AsString then
        Exit(V);
end;

function ThemeNameSetOf(Data: TJSONData): TThemeNameSet;
var
  I: Integer;
  V: TThemeName;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TThemeName) to High(TThemeName) do
      if ThemeNameNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ThemeNameSetToJSON(const S: TThemeNameSet): TJSONData;
var
  V: TThemeName;
begin
  Result := TJSONArray.Create;
  for V := Low(TThemeName) to High(TThemeName) do
    if V in S then
      TJSONArray(Result).Add(ThemeNameNames[V]);
end;

const
  TitleAlignNames: array[TTitleAlign] of string = ('left', 'center', 'right');

function TitleAlignOf(Data: TJSONData): TTitleAlign;
var
  V: TTitleAlign;
begin
  Result := Low(TTitleAlign);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTitleAlign) to High(TTitleAlign) do
      if TitleAlignNames[V] = Data.AsString then
        Exit(V);
end;

function TitleAlignSetOf(Data: TJSONData): TTitleAlignSet;
var
  I: Integer;
  V: TTitleAlign;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTitleAlign) to High(TTitleAlign) do
      if TitleAlignNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TitleAlignSetToJSON(const S: TTitleAlignSet): TJSONData;
var
  V: TTitleAlign;
begin
  Result := TJSONArray.Create;
  for V := Low(TTitleAlign) to High(TTitleAlign) do
    if V in S then
      TJSONArray(Result).Add(TitleAlignNames[V]);
end;

const
  TransitionNames: array[TTransition] of string = ('none', 'fade', 'slide_left', 'slide_right', 'slide_up', 'slide_down');

function TransitionOf(Data: TJSONData): TTransition;
var
  V: TTransition;
begin
  Result := Low(TTransition);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTransition) to High(TTransition) do
      if TransitionNames[V] = Data.AsString then
        Exit(V);
end;

function TransitionSetOf(Data: TJSONData): TTransitionSet;
var
  I: Integer;
  V: TTransition;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTransition) to High(TTransition) do
      if TransitionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TransitionSetToJSON(const S: TTransitionSet): TJSONData;
var
  V: TTransition;
begin
  Result := TJSONArray.Create;
  for V := Low(TTransition) to High(TTransition) do
    if V in S then
      TJSONArray(Result).Add(TransitionNames[V]);
end;

const
  TrimmingNames: array[TTrimming] of string = ('none', 'character', 'word');

function TrimmingOf(Data: TJSONData): TTrimming;
var
  V: TTrimming;
begin
  Result := Low(TTrimming);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTrimming) to High(TTrimming) do
      if TrimmingNames[V] = Data.AsString then
        Exit(V);
end;

function TrimmingSetOf(Data: TJSONData): TTrimmingSet;
var
  I: Integer;
  V: TTrimming;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTrimming) to High(TTrimming) do
      if TrimmingNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TrimmingSetToJSON(const S: TTrimmingSet): TJSONData;
var
  V: TTrimming;
begin
  Result := TJSONArray.Create;
  for V := Low(TTrimming) to High(TTrimming) do
    if V in S then
      TJSONArray(Result).Add(TrimmingNames[V]);
end;

const
  ValueFormatNames: array[TValueFormat] of string = ('short', 'full', 'percent');

function ValueFormatOf(Data: TJSONData): TValueFormat;
var
  V: TValueFormat;
begin
  Result := Low(TValueFormat);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TValueFormat) to High(TValueFormat) do
      if ValueFormatNames[V] = Data.AsString then
        Exit(V);
end;

function ValueFormatSetOf(Data: TJSONData): TValueFormatSet;
var
  I: Integer;
  V: TValueFormat;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TValueFormat) to High(TValueFormat) do
      if ValueFormatNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ValueFormatSetToJSON(const S: TValueFormatSet): TJSONData;
var
  V: TValueFormat;
begin
  Result := TJSONArray.Create;
  for V := Low(TValueFormat) to High(TValueFormat) do
    if V in S then
      TJSONArray(Result).Add(ValueFormatNames[V]);
end;

const
  WindowCornersNames: array[TWindowCorners] of string = ('default', 'round', 'small', 'square');

function WindowCornersOf(Data: TJSONData): TWindowCorners;
var
  V: TWindowCorners;
begin
  Result := Low(TWindowCorners);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TWindowCorners) to High(TWindowCorners) do
      if WindowCornersNames[V] = Data.AsString then
        Exit(V);
end;

function WindowCornersSetOf(Data: TJSONData): TWindowCornersSet;
var
  I: Integer;
  V: TWindowCorners;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TWindowCorners) to High(TWindowCorners) do
      if WindowCornersNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function WindowCornersSetToJSON(const S: TWindowCornersSet): TJSONData;
var
  V: TWindowCorners;
begin
  Result := TJSONArray.Create;
  for V := Low(TWindowCorners) to High(TWindowCorners) do
    if V in S then
      TJSONArray(Result).Add(WindowCornersNames[V]);
end;

const
  WindowShapeNames: array[TWindowShape] of string = ('rectangle', 'rounded', 'ellipse', 'path');

function WindowShapeOf(Data: TJSONData): TWindowShape;
var
  V: TWindowShape;
begin
  Result := Low(TWindowShape);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TWindowShape) to High(TWindowShape) do
      if WindowShapeNames[V] = Data.AsString then
        Exit(V);
end;

function WindowShapeSetOf(Data: TJSONData): TWindowShapeSet;
var
  I: Integer;
  V: TWindowShape;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TWindowShape) to High(TWindowShape) do
      if WindowShapeNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function WindowShapeSetToJSON(const S: TWindowShapeSet): TJSONData;
var
  V: TWindowShape;
begin
  Result := TJSONArray.Create;
  for V := Low(TWindowShape) to High(TWindowShape) do
    if V in S then
      TJSONArray(Result).Add(WindowShapeNames[V]);
end;

const
  WindowStateNames: array[TWindowState] of string = ('normal', 'maximized', 'minimized', 'full_screen');

function WindowStateOf(Data: TJSONData): TWindowState;
var
  V: TWindowState;
begin
  Result := Low(TWindowState);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TWindowState) to High(TWindowState) do
      if WindowStateNames[V] = Data.AsString then
        Exit(V);
end;

function WindowStateSetOf(Data: TJSONData): TWindowStateSet;
var
  I: Integer;
  V: TWindowState;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TWindowState) to High(TWindowState) do
      if WindowStateNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function WindowStateSetToJSON(const S: TWindowStateSet): TJSONData;
var
  V: TWindowState;
begin
  Result := TJSONArray.Create;
  for V := Low(TWindowState) to High(TWindowState) do
    if V in S then
      TJSONArray(Result).Add(WindowStateNames[V]);
end;

function AylisJSONTexts(Data: TJSONData): TStringArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := AsText(Data.Items[I]);
end;

function AylisJSONNumbers(Data: TJSONData): TAylisDoubleArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := Data.Items[I].AsFloat;
end;

function AylisTextsToJSON(const A: TStringArray): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(A[I]);
end;

function AylisNumbersToJSON(const A: TAylisDoubleArray): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(A[I]);
end;

function ChartSeriesFromJSON(Data: TJSONData): TChartSeries;
var
  O: TJSONObject;
begin
  Result := Default(TChartSeries);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Name := O.Get('name', '');
  Result.Values := AylisJSONNumbers(O.Find('values'));
  Result.X := AylisJSONNumbers(O.Find('x'));
  Result.Radius := AylisJSONNumbers(O.Find('radius'));
  Result.Kind := ChartSeriesKindOf(O.Find('kind'));
  Result.RightAxis := O.Get('right_axis', False);
  Result.Color := AylisColorFromJSON(O.Find('color'));
  Result.Width := O.Get('width', Double(0));
  Result.Dash := AylisJSONNumbers(O.Find('dash'));
  Result.Point := ChartSeriesPointOf(O.Find('point'));
  Result.Fill := ChartSeriesFillOf(O.Find('fill'));
  Result.FillSeries := O.Get('fill_series', Int64(0));
  Result.Hidden := O.Get('hidden', False);
end;

function ChartSeriesToJSON(const R: TChartSeries): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('name', R.Name);
  Result.Add('values', AylisNumbersToJSON(R.Values));
  Result.Add('x', AylisNumbersToJSON(R.X));
  Result.Add('radius', AylisNumbersToJSON(R.Radius));
  Result.Add('kind', ChartSeriesKindNames[R.Kind]);
  Result.Add('right_axis', R.RightAxis);
  Result.Add('color', AylisColorToJSON(R.Color));
  Result.Add('width', R.Width);
  Result.Add('dash', AylisNumbersToJSON(R.Dash));
  Result.Add('point', ChartSeriesPointNames[R.Point]);
  Result.Add('fill', ChartSeriesFillNames[R.Fill]);
  Result.Add('fill_series', R.FillSeries);
  Result.Add('hidden', R.Hidden);
end;

function ChartSeriesListFromJSON(Data: TJSONData): TChartSeriesArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := ChartSeriesFromJSON(Data.Items[I]);
end;

function ChartSeriesListToJSON(const A: array of TChartSeries): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(ChartSeriesToJSON(A[I]));
end;

function ChartSeriesText(const R: TChartSeries): TJSONData;
var
  J: TJSONObject;
begin
  J := ChartSeriesToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ChartSeriesListText(const A: TChartSeriesArray): TJSONData;
var
  J: TJSONArray;
begin
  J := ChartSeriesListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ChartSeriesFromText(Data: TJSONData): TChartSeries;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ChartSeriesFromJSON(J);
  finally
    J.Free;
  end;
end;

function ChartSeriesListFromText(Data: TJSONData): TChartSeriesArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ChartSeriesListFromJSON(J);
  finally
    J.Free;
  end;
end;

function ColumnSpecFromJSON(Data: TJSONData): TColumnSpec;
var
  O: TJSONObject;
begin
  Result := Default(TColumnSpec);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Field := O.Get('field', '');
  Result.Title := O.Get('title', '');
  Result.Width := O.Get('width', Double(0));
  Result.ReadOnly := O.Get('read_only', False);
  Result.Color := AylisColorFromJSON(O.Find('color'));
  Result.TextColor := AylisColorFromJSON(O.Find('text_color'));
end;

function ColumnSpecToJSON(const R: TColumnSpec): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('field', R.Field);
  Result.Add('title', R.Title);
  Result.Add('width', R.Width);
  Result.Add('read_only', R.ReadOnly);
  Result.Add('color', AylisColorToJSON(R.Color));
  Result.Add('text_color', AylisColorToJSON(R.TextColor));
end;

function ColumnSpecListFromJSON(Data: TJSONData): TColumnSpecArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := ColumnSpecFromJSON(Data.Items[I]);
end;

function ColumnSpecListToJSON(const A: array of TColumnSpec): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(ColumnSpecToJSON(A[I]));
end;

function ColumnSpecText(const R: TColumnSpec): TJSONData;
var
  J: TJSONObject;
begin
  J := ColumnSpecToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ColumnSpecListText(const A: TColumnSpecArray): TJSONData;
var
  J: TJSONArray;
begin
  J := ColumnSpecListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ColumnSpecFromText(Data: TJSONData): TColumnSpec;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ColumnSpecFromJSON(J);
  finally
    J.Free;
  end;
end;

function ColumnSpecListFromText(Data: TJSONData): TColumnSpecArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ColumnSpecListFromJSON(J);
  finally
    J.Free;
  end;
end;

function FilterParamFromJSON(Data: TJSONData): TFilterParam;
var
  O: TJSONObject;
begin
  Result := Default(TFilterParam);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Name := O.Get('name', '');
  Result.Value := O.Get('value', '');
end;

function FilterParamToJSON(const R: TFilterParam): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('name', R.Name);
  Result.Add('value', R.Value);
end;

function FilterParamListFromJSON(Data: TJSONData): TFilterParamArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := FilterParamFromJSON(Data.Items[I]);
end;

function FilterParamListToJSON(const A: array of TFilterParam): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(FilterParamToJSON(A[I]));
end;

function FilterParamText(const R: TFilterParam): TJSONData;
var
  J: TJSONObject;
begin
  J := FilterParamToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function FilterParamListText(const A: TFilterParamArray): TJSONData;
var
  J: TJSONArray;
begin
  J := FilterParamListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function FilterParamFromText(Data: TJSONData): TFilterParam;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := FilterParamFromJSON(J);
  finally
    J.Free;
  end;
end;

function FilterParamListFromText(Data: TJSONData): TFilterParamArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := FilterParamListFromJSON(J);
  finally
    J.Free;
  end;
end;

function GestureEventFromJSON(Data: TJSONData): TGestureEvent;
var
  O: TJSONObject;
begin
  Result := Default(TGestureEvent);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Phase := O.Get('phase', '');
  Result.X := O.Get('x', Double(0));
  Result.Y := O.Get('y', Double(0));
  Result.Scale := O.Get('scale', Double(0));
  Result.DeltaScale := O.Get('delta_scale', Double(0));
  Result.Rotation := O.Get('rotation', Double(0));
  Result.DeltaRotation := O.Get('delta_rotation', Double(0));
  Result.PanX := O.Get('pan_x', Double(0));
  Result.PanY := O.Get('pan_y', Double(0));
end;

function GestureEventToJSON(const R: TGestureEvent): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('phase', R.Phase);
  Result.Add('x', R.X);
  Result.Add('y', R.Y);
  Result.Add('scale', R.Scale);
  Result.Add('delta_scale', R.DeltaScale);
  Result.Add('rotation', R.Rotation);
  Result.Add('delta_rotation', R.DeltaRotation);
  Result.Add('pan_x', R.PanX);
  Result.Add('pan_y', R.PanY);
end;

function GestureEventListFromJSON(Data: TJSONData): TGestureEventArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := GestureEventFromJSON(Data.Items[I]);
end;

function GestureEventListToJSON(const A: array of TGestureEvent): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(GestureEventToJSON(A[I]));
end;

function GestureEventText(const R: TGestureEvent): TJSONData;
var
  J: TJSONObject;
begin
  J := GestureEventToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function GestureEventListText(const A: TGestureEventArray): TJSONData;
var
  J: TJSONArray;
begin
  J := GestureEventListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function GestureEventFromText(Data: TJSONData): TGestureEvent;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := GestureEventFromJSON(J);
  finally
    J.Free;
  end;
end;

function GestureEventListFromText(Data: TJSONData): TGestureEventArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := GestureEventListFromJSON(J);
  finally
    J.Free;
  end;
end;

function GridTotalFromJSON(Data: TJSONData): TGridTotal;
var
  O: TJSONObject;
begin
  Result := Default(TGridTotal);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Field := O.Get('field', '');
  Result.&Function := GridTotalFunctionOf(O.Find('function'));
end;

function GridTotalToJSON(const R: TGridTotal): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('field', R.Field);
  Result.Add('function', GridTotalFunctionNames[R.&Function]);
end;

function GridTotalListFromJSON(Data: TJSONData): TGridTotalArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := GridTotalFromJSON(Data.Items[I]);
end;

function GridTotalListToJSON(const A: array of TGridTotal): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(GridTotalToJSON(A[I]));
end;

function GridTotalText(const R: TGridTotal): TJSONData;
var
  J: TJSONObject;
begin
  J := GridTotalToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function GridTotalListText(const A: TGridTotalArray): TJSONData;
var
  J: TJSONArray;
begin
  J := GridTotalListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function GridTotalFromText(Data: TJSONData): TGridTotal;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := GridTotalFromJSON(J);
  finally
    J.Free;
  end;
end;

function GridTotalListFromText(Data: TJSONData): TGridTotalArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := GridTotalListFromJSON(J);
  finally
    J.Free;
  end;
end;

function IndexDefFromJSON(Data: TJSONData): TIndexDef;
var
  O: TJSONObject;
begin
  Result := Default(TIndexDef);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Name := O.Get('name', '');
  Result.Fields := O.Get('fields', '');
end;

function IndexDefToJSON(const R: TIndexDef): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('name', R.Name);
  Result.Add('fields', R.Fields);
end;

function IndexDefListFromJSON(Data: TJSONData): TIndexDefArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := IndexDefFromJSON(Data.Items[I]);
end;

function IndexDefListToJSON(const A: array of TIndexDef): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(IndexDefToJSON(A[I]));
end;

function IndexDefText(const R: TIndexDef): TJSONData;
var
  J: TJSONObject;
begin
  J := IndexDefToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function IndexDefListText(const A: TIndexDefArray): TJSONData;
var
  J: TJSONArray;
begin
  J := IndexDefListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function IndexDefFromText(Data: TJSONData): TIndexDef;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := IndexDefFromJSON(J);
  finally
    J.Free;
  end;
end;

function IndexDefListFromText(Data: TJSONData): TIndexDefArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := IndexDefListFromJSON(J);
  finally
    J.Free;
  end;
end;

function KeyEventFromJSON(Data: TJSONData): TKeyEvent;
var
  O: TJSONObject;
begin
  Result := Default(TKeyEvent);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Key := O.Get('key', '');
  Result.Ctrl := O.Get('ctrl', False);
  Result.Shift := O.Get('shift', False);
  Result.Alt := O.Get('alt', False);
end;

function KeyEventToJSON(const R: TKeyEvent): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('key', R.Key);
  Result.Add('ctrl', R.Ctrl);
  Result.Add('shift', R.Shift);
  Result.Add('alt', R.Alt);
end;

function KeyEventListFromJSON(Data: TJSONData): TKeyEventArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := KeyEventFromJSON(Data.Items[I]);
end;

function KeyEventListToJSON(const A: array of TKeyEvent): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(KeyEventToJSON(A[I]));
end;

function KeyEventText(const R: TKeyEvent): TJSONData;
var
  J: TJSONObject;
begin
  J := KeyEventToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function KeyEventListText(const A: TKeyEventArray): TJSONData;
var
  J: TJSONArray;
begin
  J := KeyEventListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function KeyEventFromText(Data: TJSONData): TKeyEvent;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := KeyEventFromJSON(J);
  finally
    J.Free;
  end;
end;

function KeyEventListFromText(Data: TJSONData): TKeyEventArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := KeyEventListFromJSON(J);
  finally
    J.Free;
  end;
end;

function MasterFieldFromJSON(Data: TJSONData): TMasterField;
var
  O: TJSONObject;
begin
  Result := Default(TMasterField);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Field := O.Get('field', '');
  Result.MasterField := O.Get('master_field', '');
end;

function MasterFieldToJSON(const R: TMasterField): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('field', R.Field);
  Result.Add('master_field', R.MasterField);
end;

function MasterFieldListFromJSON(Data: TJSONData): TMasterFieldArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := MasterFieldFromJSON(Data.Items[I]);
end;

function MasterFieldListToJSON(const A: array of TMasterField): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(MasterFieldToJSON(A[I]));
end;

function MasterFieldText(const R: TMasterField): TJSONData;
var
  J: TJSONObject;
begin
  J := MasterFieldToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function MasterFieldListText(const A: TMasterFieldArray): TJSONData;
var
  J: TJSONArray;
begin
  J := MasterFieldListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function MasterFieldFromText(Data: TJSONData): TMasterField;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := MasterFieldFromJSON(J);
  finally
    J.Free;
  end;
end;

function MasterFieldListFromText(Data: TJSONData): TMasterFieldArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := MasterFieldListFromJSON(J);
  finally
    J.Free;
  end;
end;

function PointerEventFromJSON(Data: TJSONData): TPointerEvent;
var
  O: TJSONObject;
begin
  Result := Default(TPointerEvent);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.X := O.Get('x', Double(0));
  Result.Y := O.Get('y', Double(0));
  Result.Button := O.Get('button', '');
  Result.Clicks := O.Get('clicks', Int64(0));
end;

function PointerEventToJSON(const R: TPointerEvent): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('x', R.X);
  Result.Add('y', R.Y);
  Result.Add('button', R.Button);
  Result.Add('clicks', R.Clicks);
end;

function PointerEventListFromJSON(Data: TJSONData): TPointerEventArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := PointerEventFromJSON(Data.Items[I]);
end;

function PointerEventListToJSON(const A: array of TPointerEvent): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(PointerEventToJSON(A[I]));
end;

function PointerEventText(const R: TPointerEvent): TJSONData;
var
  J: TJSONObject;
begin
  J := PointerEventToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function PointerEventListText(const A: TPointerEventArray): TJSONData;
var
  J: TJSONArray;
begin
  J := PointerEventListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function PointerEventFromText(Data: TJSONData): TPointerEvent;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := PointerEventFromJSON(J);
  finally
    J.Free;
  end;
end;

function PointerEventListFromText(Data: TJSONData): TPointerEventArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := PointerEventListFromJSON(J);
  finally
    J.Free;
  end;
end;

function SwipeEventFromJSON(Data: TJSONData): TSwipeEvent;
var
  O: TJSONObject;
begin
  Result := Default(TSwipeEvent);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Direction := O.Get('direction', '');
  Result.X := O.Get('x', Double(0));
  Result.Y := O.Get('y', Double(0));
  Result.Dx := O.Get('dx', Double(0));
  Result.Dy := O.Get('dy', Double(0));
  Result.Vx := O.Get('vx', Double(0));
  Result.Vy := O.Get('vy', Double(0));
end;

function SwipeEventToJSON(const R: TSwipeEvent): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('direction', R.Direction);
  Result.Add('x', R.X);
  Result.Add('y', R.Y);
  Result.Add('dx', R.Dx);
  Result.Add('dy', R.Dy);
  Result.Add('vx', R.Vx);
  Result.Add('vy', R.Vy);
end;

function SwipeEventListFromJSON(Data: TJSONData): TSwipeEventArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := SwipeEventFromJSON(Data.Items[I]);
end;

function SwipeEventListToJSON(const A: array of TSwipeEvent): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(SwipeEventToJSON(A[I]));
end;

function SwipeEventText(const R: TSwipeEvent): TJSONData;
var
  J: TJSONObject;
begin
  J := SwipeEventToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function SwipeEventListText(const A: TSwipeEventArray): TJSONData;
var
  J: TJSONArray;
begin
  J := SwipeEventListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function SwipeEventFromText(Data: TJSONData): TSwipeEvent;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := SwipeEventFromJSON(J);
  finally
    J.Free;
  end;
end;

function SwipeEventListFromText(Data: TJSONData): TSwipeEventArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := SwipeEventListFromJSON(J);
  finally
    J.Free;
  end;
end;

function WheelEventFromJSON(Data: TJSONData): TWheelEvent;
var
  O: TJSONObject;
begin
  Result := Default(TWheelEvent);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.X := O.Get('x', Double(0));
  Result.Y := O.Get('y', Double(0));
  Result.Dx := O.Get('dx', Double(0));
  Result.Dy := O.Get('dy', Double(0));
end;

function WheelEventToJSON(const R: TWheelEvent): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('x', R.X);
  Result.Add('y', R.Y);
  Result.Add('dx', R.Dx);
  Result.Add('dy', R.Dy);
end;

function WheelEventListFromJSON(Data: TJSONData): TWheelEventArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := WheelEventFromJSON(Data.Items[I]);
end;

function WheelEventListToJSON(const A: array of TWheelEvent): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(WheelEventToJSON(A[I]));
end;

function WheelEventText(const R: TWheelEvent): TJSONData;
var
  J: TJSONObject;
begin
  J := WheelEventToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function WheelEventListText(const A: TWheelEventArray): TJSONData;
var
  J: TJSONArray;
begin
  J := WheelEventListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function WheelEventFromText(Data: TJSONData): TWheelEvent;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := WheelEventFromJSON(J);
  finally
    J.Free;
  end;
end;

function WheelEventListFromText(Data: TJSONData): TWheelEventArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := WheelEventListFromJSON(J);
  finally
    J.Free;
  end;
end;

function TClientDataSet.GetKeyFields: string;
begin
  Result := AsText(GetProp('key_fields'));
end;

procedure TClientDataSet.SetKeyFields(Value: string);
begin
  SetProp('key_fields', TJSONString.Create(Value));
end;

function TClientDataSet.GetApplyMode: TApplyMode;
begin
  Result := ApplyModeOf(GetProp('apply_mode'));
end;

procedure TClientDataSet.SetApplyMode(Value: TApplyMode);
begin
  SetProp('apply_mode', TJSONString.Create(ApplyModeNames[Value]));
end;

function TClientDataSet.GetReadOnly: Boolean;
begin
  Result := GetProp('read_only').AsBoolean;
end;

procedure TClientDataSet.SetReadOnly(Value: Boolean);
begin
  SetProp('read_only', TJSONBoolean.Create(Value));
end;

function TClientDataSet.GetActive: Boolean;
begin
  Result := GetProp('active').AsBoolean;
end;

procedure TClientDataSet.SetActive(Value: Boolean);
begin
  SetProp('active', TJSONBoolean.Create(Value));
end;

function TClientDataSet.GetFilter: string;
begin
  Result := AsText(GetProp('filter'));
end;

procedure TClientDataSet.SetFilter(Value: string);
begin
  SetProp('filter', TJSONString.Create(Value));
end;

function TClientDataSet.GetFiltered: Boolean;
begin
  Result := GetProp('filtered').AsBoolean;
end;

procedure TClientDataSet.SetFiltered(Value: Boolean);
begin
  SetProp('filtered', TJSONBoolean.Create(Value));
end;

function TClientDataSet.GetIndexFieldNames: string;
begin
  Result := AsText(GetProp('index_field_names'));
end;

procedure TClientDataSet.SetIndexFieldNames(Value: string);
begin
  SetProp('index_field_names', TJSONString.Create(Value));
end;

function TClientDataSet.GetIndexes: TIndexDefArray;
begin
  Result := IndexDefListFromText(GetProp('indexes'));
end;

procedure TClientDataSet.SetIndexes(Value: TIndexDefArray);
begin
  SetProp('indexes', IndexDefListText(Value));
end;

function TClientDataSet.GetIndexName: string;
begin
  Result := AsText(GetProp('index_name'));
end;

procedure TClientDataSet.SetIndexName(Value: string);
begin
  SetProp('index_name', TJSONString.Create(Value));
end;

function TClientDataSet.GetParams: TFilterParamArray;
begin
  Result := FilterParamListFromText(GetProp('params'));
end;

procedure TClientDataSet.SetParams(Value: TFilterParamArray);
begin
  SetProp('params', FilterParamListText(Value));
end;

function TClientDataSet.GetStatusFilter: string;
begin
  Result := AsText(GetProp('status_filter'));
end;

procedure TClientDataSet.SetStatusFilter(Value: string);
begin
  SetProp('status_filter', TJSONString.Create(Value));
end;

function TClientDataSet.GetMasterSource: TAylisComponent;
begin
  Result := GetRef('master_source');
end;

procedure TClientDataSet.SetMasterSource(Value: TAylisComponent);
begin
  SetRef('master_source', Value);
end;

function TClientDataSet.GetMasterFields: TMasterFieldArray;
begin
  Result := MasterFieldListFromText(GetProp('master_fields'));
end;

procedure TClientDataSet.SetMasterFields(Value: TMasterFieldArray);
begin
  SetProp('master_fields', MasterFieldListText(Value));
end;

function TClientDataSet.GetApplySeparately: Boolean;
begin
  Result := GetProp('apply_separately').AsBoolean;
end;

procedure TClientDataSet.SetApplySeparately(Value: Boolean);
begin
  SetProp('apply_separately', TJSONBoolean.Create(Value));
end;

function TClientDataSet.GetRecordTotal: Int64;
begin
  Result := GetProp('record_total').AsInt64;
end;

procedure TClientDataSet.SetRecordTotal(Value: Int64);
begin
  SetProp('record_total', TJSONInt64Number.Create(Value));
end;

function TClientDataSet.GetMaxRecords: Int64;
begin
  Result := GetProp('max_records').AsInt64;
end;

procedure TClientDataSet.SetMaxRecords(Value: Int64);
begin
  SetProp('max_records', TJSONInt64Number.Create(Value));
end;

function TClientDataSet.GetPartialNotice: string;
begin
  Result := AsText(GetProp('partial_notice'));
end;

function TClientDataSet.GetOnMasterChange: string;
begin
  Result := AsText(GetProp('on_master_change'));
end;

procedure TClientDataSet.SetOnMasterChange(Value: string);
begin
  SetProp('on_master_change', TJSONString.Create(Value));
end;

function TClientDataSet.GetAfterScroll: string;
begin
  Result := AsText(GetProp('after_scroll'));
end;

procedure TClientDataSet.SetAfterScroll(Value: string);
begin
  SetProp('after_scroll', TJSONString.Create(Value));
end;

function TClientDataSet.GetAfterPost: string;
begin
  Result := AsText(GetProp('after_post'));
end;

procedure TClientDataSet.SetAfterPost(Value: string);
begin
  SetProp('after_post', TJSONString.Create(Value));
end;

function TClientDataSet.GetOnStateChange: string;
begin
  Result := AsText(GetProp('on_state_change'));
end;

procedure TClientDataSet.SetOnStateChange(Value: string);
begin
  SetProp('on_state_change', TJSONString.Create(Value));
end;

function TClientDataSet.GetOnReconcileError: string;
begin
  Result := AsText(GetProp('on_reconcile_error'));
end;

procedure TClientDataSet.SetOnReconcileError(Value: string);
begin
  SetProp('on_reconcile_error', TJSONString.Create(Value));
end;

function TClientDataSet.GetOnError: string;
begin
  Result := AsText(GetProp('on_error'));
end;

procedure TClientDataSet.SetOnError(Value: string);
begin
  SetProp('on_error', TJSONString.Create(Value));
end;

function TClientDataSet.GetOnFetch: string;
begin
  Result := AsText(GetProp('on_fetch'));
end;

procedure TClientDataSet.SetOnFetch(Value: string);
begin
  SetProp('on_fetch', TJSONString.Create(Value));
end;

function TClientDataSet.GetOnServerQuery: string;
begin
  Result := AsText(GetProp('on_server_query'));
end;

procedure TClientDataSet.SetOnServerQuery(Value: string);
begin
  SetProp('on_server_query', TJSONString.Create(Value));
end;

function TDBChart.GetDataSource: TAylisComponent;
begin
  Result := GetRef('data_source');
end;

procedure TDBChart.SetDataSource(Value: TAylisComponent);
begin
  SetRef('data_source', Value);
end;

function TDBChart.GetLabelField: string;
begin
  Result := AsText(GetProp('label_field'));
end;

procedure TDBChart.SetLabelField(Value: string);
begin
  SetProp('label_field', TJSONString.Create(Value));
end;

function TDBChart.GetValueFields: TStringArray;
begin
  Result := AsList(GetProp('value_fields'));
end;

procedure TDBChart.SetValueFields(Value: TStringArray);
begin
  SetProp('value_fields', ListText(Value));
end;

function TDBChart.GetKind: TChartKind;
begin
  Result := ChartKindOf(GetProp('kind'));
end;

procedure TDBChart.SetKind(Value: TChartKind);
begin
  SetProp('kind', TJSONString.Create(ChartKindNames[Value]));
end;

function TDBChart.GetTitle: string;
begin
  Result := AsText(GetProp('title'));
end;

procedure TDBChart.SetTitle(Value: string);
begin
  SetProp('title', TJSONString.Create(Value));
end;

function TDBChart.GetLabels: TStringArray;
begin
  Result := AsList(GetProp('labels'));
end;

procedure TDBChart.SetLabels(Value: TStringArray);
begin
  SetProp('labels', ListText(Value));
end;

function TDBChart.GetSeries: TChartSeriesArray;
begin
  Result := ChartSeriesListFromText(GetProp('series'));
end;

procedure TDBChart.SetSeries(Value: TChartSeriesArray);
begin
  SetProp('series', ChartSeriesListText(Value));
end;

function TDBChart.GetJson: string;
begin
  Result := AsText(GetProp('json'));
end;

procedure TDBChart.SetJson(Value: string);
begin
  SetProp('json', TJSONString.Create(Value));
end;

function TDBChart.GetLegend: Boolean;
begin
  Result := GetProp('legend').AsBoolean;
end;

procedure TDBChart.SetLegend(Value: Boolean);
begin
  SetProp('legend', TJSONBoolean.Create(Value));
end;

function TDBChart.GetStepped: Boolean;
begin
  Result := GetProp('stepped').AsBoolean;
end;

procedure TDBChart.SetStepped(Value: Boolean);
begin
  SetProp('stepped', TJSONBoolean.Create(Value));
end;

function TDBChart.GetTension: Double;
begin
  Result := GetProp('tension').AsFloat;
end;

procedure TDBChart.SetTension(Value: Double);
begin
  SetProp('tension', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetAnimate: Boolean;
begin
  Result := GetProp('animate').AsBoolean;
end;

procedure TDBChart.SetAnimate(Value: Boolean);
begin
  SetProp('animate', TJSONBoolean.Create(Value));
end;

function TDBChart.GetZoom: Boolean;
begin
  Result := GetProp('zoom').AsBoolean;
end;

procedure TDBChart.SetZoom(Value: Boolean);
begin
  SetProp('zoom', TJSONBoolean.Create(Value));
end;

function TDBChart.GetAutoScale: Boolean;
begin
  Result := GetProp('auto_scale').AsBoolean;
end;

procedure TDBChart.SetAutoScale(Value: Boolean);
begin
  SetProp('auto_scale', TJSONBoolean.Create(Value));
end;

function TDBChart.GetYMin: Double;
begin
  Result := GetProp('y_min').AsFloat;
end;

procedure TDBChart.SetYMin(Value: Double);
begin
  SetProp('y_min', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetYMax: Double;
begin
  Result := GetProp('y_max').AsFloat;
end;

procedure TDBChart.SetYMax(Value: Double);
begin
  SetProp('y_max', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetLogScale: Boolean;
begin
  Result := GetProp('log_scale').AsBoolean;
end;

procedure TDBChart.SetLogScale(Value: Boolean);
begin
  SetProp('log_scale', TJSONBoolean.Create(Value));
end;

function TDBChart.GetReverse: Boolean;
begin
  Result := GetProp('reverse').AsBoolean;
end;

procedure TDBChart.SetReverse(Value: Boolean);
begin
  SetProp('reverse', TJSONBoolean.Create(Value));
end;

function TDBChart.GetTimeAxis: Boolean;
begin
  Result := GetProp('time_axis').AsBoolean;
end;

procedure TDBChart.SetTimeAxis(Value: Boolean);
begin
  SetProp('time_axis', TJSONBoolean.Create(Value));
end;

function TDBChart.GetClickedSeries: Int64;
begin
  Result := GetProp('clicked_series').AsInt64;
end;

procedure TDBChart.SetClickedSeries(Value: Int64);
begin
  SetProp('clicked_series', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetClickedIndex: Int64;
begin
  Result := GetProp('clicked_index').AsInt64;
end;

procedure TDBChart.SetClickedIndex(Value: Int64);
begin
  SetProp('clicked_index', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetValueFormat: TValueFormat;
begin
  Result := ValueFormatOf(GetProp('value_format'));
end;

procedure TDBChart.SetValueFormat(Value: TValueFormat);
begin
  SetProp('value_format', TJSONString.Create(ValueFormatNames[Value]));
end;

function TDBChart.GetLabelAngle: Double;
begin
  Result := GetProp('label_angle').AsFloat;
end;

procedure TDBChart.SetLabelAngle(Value: Double);
begin
  SetProp('label_angle', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetTitleAlign: TTitleAlign;
begin
  Result := TitleAlignOf(GetProp('title_align'));
end;

procedure TDBChart.SetTitleAlign(Value: TTitleAlign);
begin
  SetProp('title_align', TJSONString.Create(TitleAlignNames[Value]));
end;

function TDBChart.GetTitleBold: Boolean;
begin
  Result := GetProp('title_bold').AsBoolean;
end;

procedure TDBChart.SetTitleBold(Value: Boolean);
begin
  SetProp('title_bold', TJSONBoolean.Create(Value));
end;

function TDBChart.GetAreaOpacity: Double;
begin
  Result := GetProp('area_opacity').AsFloat;
end;

procedure TDBChart.SetAreaOpacity(Value: Double);
begin
  SetProp('area_opacity', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetLegendPosition: TLegendPosition;
begin
  Result := LegendPositionOf(GetProp('legend_position'));
end;

procedure TDBChart.SetLegendPosition(Value: TLegendPosition);
begin
  SetProp('legend_position', TJSONString.Create(LegendPositionNames[Value]));
end;

function TDBChart.GetStacked: Boolean;
begin
  Result := GetProp('stacked').AsBoolean;
end;

procedure TDBChart.SetStacked(Value: Boolean);
begin
  SetProp('stacked', TJSONBoolean.Create(Value));
end;

function TDBChart.GetColors: string;
begin
  Result := AsText(GetProp('colors'));
end;

procedure TDBChart.SetColors(Value: string);
begin
  SetProp('colors', TJSONString.Create(Value));
end;

function TDBChart.GetCenterText: string;
begin
  Result := AsText(GetProp('center_text'));
end;

procedure TDBChart.SetCenterText(Value: string);
begin
  SetProp('center_text', TJSONString.Create(Value));
end;

function TDBChart.GetSliceLabels: string;
begin
  Result := AsText(GetProp('slice_labels'));
end;

procedure TDBChart.SetSliceLabels(Value: string);
begin
  SetProp('slice_labels', TJSONString.Create(Value));
end;

function TDBChart.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TDBChart.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TDBChart.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TDBChart.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TDBChart.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TDBChart.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TDBChart.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TDBChart.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TDBChart.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TDBChart.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TDBChart.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TDBChart.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TDBChart.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TDBChart.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TDBChart.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TDBChart.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TDBChart.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TDBChart.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TDBChart.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TDBChart.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TDBChart.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TDBChart.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TDBChart.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TDBChart.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TDBChart.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDBChart.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TDBChart.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TDBChart.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TDBChart.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TDBChart.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TDBChart.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TDBChart.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TDBChart.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TDBChart.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TDBChart.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TDBChart.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TDBChart.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TDBChart.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TDBChart.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TDBChart.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TDBChart.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TDBChart.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TDBChart.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TDBChart.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TDBChart.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TDBChart.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TDBChart.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDBChart.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TDBChart.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TDBChart.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TDBChart.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TDBChart.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TDBChart.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TDBChart.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TDBChart.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TDBChart.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TDBChart.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TDBChart.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TDBChart.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TDBChart.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TDBChart.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TDBChart.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TDBChart.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TDBChart.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TDBChart.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TDBChart.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TDBChart.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TDBChart.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TDBChart.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TDBChart.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TDBChart.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TDBChart.GetStyle: TDBChartStyle;
begin
  Result := DBChartStyleOf(GetProp('style'));
end;

procedure TDBChart.SetStyle(Value: TDBChartStyle);
begin
  SetProp('style', TJSONString.Create(DBChartStyleNames[Value]));
end;

function TDBChart.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TDBChart.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TDBChart.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TDBChart.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TDBChart.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TDBChart.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TDBChart.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TDBChart.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TDBChart.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TDBChart.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TDBChart.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TDBChart.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TDBChart.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TDBChart.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TDBChart.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TDBChart.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TDBChart.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TDBChart.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TDBChart.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TDBChart.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TDBChart.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TDBChart.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TDBChart.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TDBChart.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TDBChart.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TDBChart.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TDBChart.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TDBChart.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TDBChart.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TDBChart.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TDBChart.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TDBChart.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TDBChart.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TDBChart.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TDBChart.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TDBChart.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TDBChart.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TDBChart.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TDBChart.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TDBChart.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TDBChart.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TDBChart.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TDBChart.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TDBChart.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TDBChart.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TDBChart.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TDBChart.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TDBChart.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TDBChart.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TDBChart.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TDBChart.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TDBChart.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TDBChart.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TDBChart.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TDBChart.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TDBChart.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TDBChart.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TDBChart.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TDBChart.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TDBChart.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TDBChart.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TDBChart.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TDBChart.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TDBChart.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TDBChart.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TDBChart.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TDBChart.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TDBChart.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TDBChart.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TDBChart.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TDBChart.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TDBChart.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TDBChart.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TDBChart.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TDBChart.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TDBChart.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TDataGrid.GetOverscroll: TOverscroll;
begin
  Result := OverscrollOf(GetProp('overscroll'));
end;

procedure TDataGrid.SetOverscroll(Value: TOverscroll);
begin
  SetProp('overscroll', TJSONString.Create(OverscrollNames[Value]));
end;

function TDataGrid.GetDataSource: TAylisComponent;
begin
  Result := GetRef('data_source');
end;

procedure TDataGrid.SetDataSource(Value: TAylisComponent);
begin
  SetRef('data_source', Value);
end;

function TDataGrid.GetColumns: TColumnSpecArray;
begin
  Result := ColumnSpecListFromText(GetProp('columns'));
end;

procedure TDataGrid.SetColumns(Value: TColumnSpecArray);
begin
  SetProp('columns', ColumnSpecListText(Value));
end;

function TDataGrid.GetRowHeight: Double;
begin
  Result := GetProp('row_height').AsFloat;
end;

procedure TDataGrid.SetRowHeight(Value: Double);
begin
  SetProp('row_height', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetReadOnly: Boolean;
begin
  Result := GetProp('read_only').AsBoolean;
end;

procedure TDataGrid.SetReadOnly(Value: Boolean);
begin
  SetProp('read_only', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetSortable: Boolean;
begin
  Result := GetProp('sortable').AsBoolean;
end;

procedure TDataGrid.SetSortable(Value: Boolean);
begin
  SetProp('sortable', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetColumnMove: Boolean;
begin
  Result := GetProp('column_move').AsBoolean;
end;

procedure TDataGrid.SetColumnMove(Value: Boolean);
begin
  SetProp('column_move', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetFixedColumns: Int64;
begin
  Result := GetProp('fixed_columns').AsInt64;
end;

procedure TDataGrid.SetFixedColumns(Value: Int64);
begin
  SetProp('fixed_columns', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetFilterRow: Boolean;
begin
  Result := GetProp('filter_row').AsBoolean;
end;

procedure TDataGrid.SetFilterRow(Value: Boolean);
begin
  SetProp('filter_row', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetMultiSelect: Boolean;
begin
  Result := GetProp('multi_select').AsBoolean;
end;

procedure TDataGrid.SetMultiSelect(Value: Boolean);
begin
  SetProp('multi_select', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetFooter: TGridTotalArray;
begin
  Result := GridTotalListFromText(GetProp('footer'));
end;

procedure TDataGrid.SetFooter(Value: TGridTotalArray);
begin
  SetProp('footer', GridTotalListText(Value));
end;

function TDataGrid.GetIncrementalSearch: Boolean;
begin
  Result := GetProp('incremental_search').AsBoolean;
end;

procedure TDataGrid.SetIncrementalSearch(Value: Boolean);
begin
  SetProp('incremental_search', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCursorColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('cursor_color'));
end;

procedure TDataGrid.SetCursorColor(Value: TAylisColor);
begin
  SetProp('cursor_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetIndicator: Boolean;
begin
  Result := GetProp('indicator').AsBoolean;
end;

procedure TDataGrid.SetIndicator(Value: Boolean);
begin
  SetProp('indicator', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetHighlightCurrent: Boolean;
begin
  Result := GetProp('highlight_current').AsBoolean;
end;

procedure TDataGrid.SetHighlightCurrent(Value: Boolean);
begin
  SetProp('highlight_current', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetHorizontalWheel: THorizontalWheel;
begin
  Result := HorizontalWheelOf(GetProp('horizontal_wheel'));
end;

procedure TDataGrid.SetHorizontalWheel(Value: THorizontalWheel);
begin
  SetProp('horizontal_wheel', TJSONString.Create(HorizontalWheelNames[Value]));
end;

function TDataGrid.GetOnError: string;
begin
  Result := AsText(GetProp('on_error'));
end;

procedure TDataGrid.SetOnError(Value: string);
begin
  SetProp('on_error', TJSONString.Create(Value));
end;

function TDataGrid.GetOnTitleClick: string;
begin
  Result := AsText(GetProp('on_title_click'));
end;

procedure TDataGrid.SetOnTitleClick(Value: string);
begin
  SetProp('on_title_click', TJSONString.Create(Value));
end;

function TDataGrid.GetOnSelectionChange: string;
begin
  Result := AsText(GetProp('on_selection_change'));
end;

procedure TDataGrid.SetOnSelectionChange(Value: string);
begin
  SetProp('on_selection_change', TJSONString.Create(Value));
end;

function TDataGrid.GetOnColumnMoved: string;
begin
  Result := AsText(GetProp('on_column_moved'));
end;

procedure TDataGrid.SetOnColumnMoved(Value: string);
begin
  SetProp('on_column_moved', TJSONString.Create(Value));
end;

function TDataGrid.GetScrollLeft: Double;
begin
  Result := GetProp('scroll_left').AsFloat;
end;

procedure TDataGrid.SetScrollLeft(Value: Double);
begin
  SetProp('scroll_left', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetTopRow: Int64;
begin
  Result := GetProp('top_row').AsInt64;
end;

procedure TDataGrid.SetTopRow(Value: Int64);
begin
  SetProp('top_row', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TDataGrid.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TDataGrid.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TDataGrid.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TDataGrid.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TDataGrid.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TDataGrid.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TDataGrid.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TDataGrid.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TDataGrid.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TDataGrid.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TDataGrid.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TDataGrid.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TDataGrid.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TDataGrid.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TDataGrid.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TDataGrid.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TDataGrid.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TDataGrid.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDataGrid.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TDataGrid.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TDataGrid.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TDataGrid.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TDataGrid.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TDataGrid.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TDataGrid.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TDataGrid.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TDataGrid.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TDataGrid.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TDataGrid.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TDataGrid.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TDataGrid.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TDataGrid.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TDataGrid.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TDataGrid.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDataGrid.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TDataGrid.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TDataGrid.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TDataGrid.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TDataGrid.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TDataGrid.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TDataGrid.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TDataGrid.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TDataGrid.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TDataGrid.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TDataGrid.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TDataGrid.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TDataGrid.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TDataGrid.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TDataGrid.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TDataGrid.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TDataGrid.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TDataGrid.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TDataGrid.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TDataGrid.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TDataGrid.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TDataGrid.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TDataGrid.GetStyle: TDataGridStyle;
begin
  Result := DataGridStyleOf(GetProp('style'));
end;

procedure TDataGrid.SetStyle(Value: TDataGridStyle);
begin
  SetProp('style', TJSONString.Create(DataGridStyleNames[Value]));
end;

function TDataGrid.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TDataGrid.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TDataGrid.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TDataGrid.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TDataGrid.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TDataGrid.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TDataGrid.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TDataGrid.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TDataGrid.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TDataGrid.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TDataGrid.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TDataGrid.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TDataGrid.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TDataGrid.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TDataGrid.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TDataGrid.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TDataGrid.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TDataGrid.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TDataGrid.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TDataGrid.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TDataGrid.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TDataGrid.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TDataGrid.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TDataGrid.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TDataGrid.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TDataGrid.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TDataGrid.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TDataGrid.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TDataGrid.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TDataGrid.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TDataGrid.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TDataGrid.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TDataGrid.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TDataGrid.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TDataGrid.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TDataGrid.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TDataGrid.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TDataGrid.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TDataGrid.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TDataGrid.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TDataGrid.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TDataGrid.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TDataGrid.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TDataGrid.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TDataGrid.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TDataGrid.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TDataGrid.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TDataGrid.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TDataGrid.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TDataGrid.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TDataGrid.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TDataGrid.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TDataGrid.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TDataGrid.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TDataGrid.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TDataGrid.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TDataGrid.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TDataGrid.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TDataGrid.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TDataGrid.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TDataGrid.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TDataGrid.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TDataGrid.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TDataGrid.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TDataGrid.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TDataGrid.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TDataGrid.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TDataGrid.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TDataGrid.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TDataSource.GetDataSet: TAylisComponent;
begin
  Result := GetRef('data_set');
end;

procedure TDataSource.SetDataSet(Value: TAylisComponent);
begin
  SetRef('data_set', Value);
end;

function TDataSource.GetAutoEdit: Boolean;
begin
  Result := GetProp('auto_edit').AsBoolean;
end;

procedure TDataSource.SetAutoEdit(Value: Boolean);
begin
  SetProp('auto_edit', TJSONBoolean.Create(Value));
end;

function TDataSource.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TDataSource.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TDataSource.GetOnDataChange: string;
begin
  Result := AsText(GetProp('on_data_change'));
end;

procedure TDataSource.SetOnDataChange(Value: string);
begin
  SetProp('on_data_change', TJSONString.Create(Value));
end;

function TDataSource.GetOnStateChange: string;
begin
  Result := AsText(GetProp('on_state_change'));
end;

procedure TDataSource.SetOnStateChange(Value: string);
begin
  SetProp('on_state_change', TJSONString.Create(Value));
end;

function TField.GetFieldName: string;
begin
  Result := AsText(GetProp('field_name'));
end;

procedure TField.SetFieldName(Value: string);
begin
  SetProp('field_name', TJSONString.Create(Value));
end;

function TField.GetType: TFieldType;
begin
  Result := FieldTypeOf(GetProp('type'));
end;

procedure TField.SetType(Value: TFieldType);
begin
  SetProp('type', TJSONString.Create(FieldTypeNames[Value]));
end;

function TField.GetSize: Int64;
begin
  Result := GetProp('size').AsInt64;
end;

procedure TField.SetSize(Value: Int64);
begin
  SetProp('size', TJSONInt64Number.Create(Value));
end;

function TField.GetScale: Int64;
begin
  Result := GetProp('scale').AsInt64;
end;

procedure TField.SetScale(Value: Int64);
begin
  SetProp('scale', TJSONInt64Number.Create(Value));
end;

function TField.GetRounding: TRounding;
begin
  Result := RoundingOf(GetProp('rounding'));
end;

procedure TField.SetRounding(Value: TRounding);
begin
  SetProp('rounding', TJSONString.Create(RoundingNames[Value]));
end;

function TField.GetDisplayLabel: string;
begin
  Result := AsText(GetProp('display_label'));
end;

procedure TField.SetDisplayLabel(Value: string);
begin
  SetProp('display_label', TJSONString.Create(Value));
end;

function TField.GetDisplayWidth: Double;
begin
  Result := GetProp('display_width').AsFloat;
end;

procedure TField.SetDisplayWidth(Value: Double);
begin
  SetProp('display_width', TJSONFloatNumber.Create(Value));
end;

function TField.GetDisplayFormat: string;
begin
  Result := AsText(GetProp('display_format'));
end;

procedure TField.SetDisplayFormat(Value: string);
begin
  SetProp('display_format', TJSONString.Create(Value));
end;

function TField.GetEditFormat: string;
begin
  Result := AsText(GetProp('edit_format'));
end;

procedure TField.SetEditFormat(Value: string);
begin
  SetProp('edit_format', TJSONString.Create(Value));
end;

function TField.GetAlignment: TTextAlign;
begin
  Result := TextAlignOf(GetProp('alignment'));
end;

procedure TField.SetAlignment(Value: TTextAlign);
begin
  SetProp('alignment', TJSONString.Create(TextAlignNames[Value]));
end;

function TField.GetRequired: Boolean;
begin
  Result := GetProp('required').AsBoolean;
end;

procedure TField.SetRequired(Value: Boolean);
begin
  SetProp('required', TJSONBoolean.Create(Value));
end;

function TField.GetReadOnly: Boolean;
begin
  Result := GetProp('read_only').AsBoolean;
end;

procedure TField.SetReadOnly(Value: Boolean);
begin
  SetProp('read_only', TJSONBoolean.Create(Value));
end;

function TField.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TField.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TField.GetMin: Double;
begin
  Result := GetProp('min').AsFloat;
end;

procedure TField.SetMin(Value: Double);
begin
  SetProp('min', TJSONFloatNumber.Create(Value));
end;

function TField.GetMax: Double;
begin
  Result := GetProp('max').AsFloat;
end;

procedure TField.SetMax(Value: Double);
begin
  SetProp('max', TJSONFloatNumber.Create(Value));
end;

function TField.GetDefaultValue: string;
begin
  Result := AsText(GetProp('default_value'));
end;

procedure TField.SetDefaultValue(Value: string);
begin
  SetProp('default_value', TJSONString.Create(Value));
end;

function TField.GetDisplayValues: string;
begin
  Result := AsText(GetProp('display_values'));
end;

procedure TField.SetDisplayValues(Value: string);
begin
  SetProp('display_values', TJSONString.Create(Value));
end;

function TField.GetCharCase: TCharCase;
begin
  Result := CharCaseOf(GetProp('char_case'));
end;

procedure TField.SetCharCase(Value: TCharCase);
begin
  SetProp('char_case', TJSONString.Create(CharCaseNames[Value]));
end;

function TField.GetEditMask: string;
begin
  Result := AsText(GetProp('edit_mask'));
end;

procedure TField.SetEditMask(Value: string);
begin
  SetProp('edit_mask', TJSONString.Create(Value));
end;

function TField.GetSensitive: Boolean;
begin
  Result := GetProp('sensitive').AsBoolean;
end;

procedure TField.SetSensitive(Value: Boolean);
begin
  SetProp('sensitive', TJSONBoolean.Create(Value));
end;

function TField.GetKind: TFieldKind;
begin
  Result := FieldKindOf(GetProp('kind'));
end;

procedure TField.SetKind(Value: TFieldKind);
begin
  SetProp('kind', TJSONString.Create(FieldKindNames[Value]));
end;

function TField.GetExpression: string;
begin
  Result := AsText(GetProp('expression'));
end;

procedure TField.SetExpression(Value: string);
begin
  SetProp('expression', TJSONString.Create(Value));
end;

function TField.GetLookupDataSet: TAylisComponent;
begin
  Result := GetRef('lookup_data_set');
end;

procedure TField.SetLookupDataSet(Value: TAylisComponent);
begin
  SetRef('lookup_data_set', Value);
end;

function TField.GetKeyFields: string;
begin
  Result := AsText(GetProp('key_fields'));
end;

procedure TField.SetKeyFields(Value: string);
begin
  SetProp('key_fields', TJSONString.Create(Value));
end;

function TField.GetLookupKeyFields: string;
begin
  Result := AsText(GetProp('lookup_key_fields'));
end;

procedure TField.SetLookupKeyFields(Value: string);
begin
  SetProp('lookup_key_fields', TJSONString.Create(Value));
end;

function TField.GetLookupResultField: string;
begin
  Result := AsText(GetProp('lookup_result_field'));
end;

procedure TField.SetLookupResultField(Value: string);
begin
  SetProp('lookup_result_field', TJSONString.Create(Value));
end;

function TField.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TField.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TField.GetOnGetText: string;
begin
  Result := AsText(GetProp('on_get_text'));
end;

procedure TField.SetOnGetText(Value: string);
begin
  SetProp('on_get_text', TJSONString.Create(Value));
end;

function TField.GetOnSetText: string;
begin
  Result := AsText(GetProp('on_set_text'));
end;

procedure TField.SetOnSetText(Value: string);
begin
  SetProp('on_set_text', TJSONString.Create(Value));
end;

function TForm.GetCaption: string;
begin
  Result := AsText(GetProp('caption'));
end;

procedure TForm.SetCaption(Value: string);
begin
  SetProp('caption', TJSONString.Create(Value));
end;

function TForm.GetPosition: TFormPosition;
begin
  Result := FormPositionOf(GetProp('position'));
end;

procedure TForm.SetPosition(Value: TFormPosition);
begin
  SetProp('position', TJSONString.Create(FormPositionNames[Value]));
end;

function TForm.GetBorderStyle: TBorderStyle;
begin
  Result := BorderStyleOf(GetProp('border_style'));
end;

procedure TForm.SetBorderStyle(Value: TBorderStyle);
begin
  SetProp('border_style', TJSONString.Create(BorderStyleNames[Value]));
end;

function TForm.GetWindowCorners: TWindowCorners;
begin
  Result := WindowCornersOf(GetProp('window_corners'));
end;

procedure TForm.SetWindowCorners(Value: TWindowCorners);
begin
  SetProp('window_corners', TJSONString.Create(WindowCornersNames[Value]));
end;

function TForm.GetTransparent: Boolean;
begin
  Result := GetProp('transparent').AsBoolean;
end;

procedure TForm.SetTransparent(Value: Boolean);
begin
  SetProp('transparent', TJSONBoolean.Create(Value));
end;

function TForm.GetWindowShape: TWindowShape;
begin
  Result := WindowShapeOf(GetProp('window_shape'));
end;

procedure TForm.SetWindowShape(Value: TWindowShape);
begin
  SetProp('window_shape', TJSONString.Create(WindowShapeNames[Value]));
end;

function TForm.GetShapeRadius: Double;
begin
  Result := GetProp('shape_radius').AsFloat;
end;

procedure TForm.SetShapeRadius(Value: Double);
begin
  SetProp('shape_radius', TJSONFloatNumber.Create(Value));
end;

function TForm.GetShapeRadii: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('shape_radii'));
end;

procedure TForm.SetShapeRadii(Value: TAylisBounds);
begin
  SetProp('shape_radii', AylisBoundsToJSON(Value));
end;

function TForm.GetShapePath: string;
begin
  Result := AsText(GetProp('shape_path'));
end;

procedure TForm.SetShapePath(Value: string);
begin
  SetProp('shape_path', TJSONString.Create(Value));
end;

function TForm.GetWindowState: TWindowState;
begin
  Result := WindowStateOf(GetProp('window_state'));
end;

procedure TForm.SetWindowState(Value: TWindowState);
begin
  SetProp('window_state', TJSONString.Create(WindowStateNames[Value]));
end;

function TForm.GetTransition: TTransition;
begin
  Result := TransitionOf(GetProp('transition'));
end;

procedure TForm.SetTransition(Value: TTransition);
begin
  SetProp('transition', TJSONString.Create(TransitionNames[Value]));
end;

function TForm.GetLayerCache: TLayerCaching;
begin
  Result := LayerCachingOf(GetProp('layer_cache'));
end;

procedure TForm.SetLayerCache(Value: TLayerCaching);
begin
  SetProp('layer_cache', TJSONString.Create(LayerCachingNames[Value]));
end;

function TForm.GetFrameReuseKb: Int64;
begin
  Result := GetProp('frame_reuse_kb').AsInt64;
end;

procedure TForm.SetFrameReuseKb(Value: Int64);
begin
  SetProp('frame_reuse_kb', TJSONInt64Number.Create(Value));
end;

function TForm.GetTopmost: Boolean;
begin
  Result := GetProp('topmost').AsBoolean;
end;

procedure TForm.SetTopmost(Value: Boolean);
begin
  SetProp('topmost', TJSONBoolean.Create(Value));
end;

function TForm.GetShowInTaskbar: Boolean;
begin
  Result := GetProp('show_in_taskbar').AsBoolean;
end;

procedure TForm.SetShowInTaskbar(Value: Boolean);
begin
  SetProp('show_in_taskbar', TJSONBoolean.Create(Value));
end;

function TForm.GetActiveControl: TAylisComponent;
begin
  Result := GetRef('active_control');
end;

procedure TForm.SetActiveControl(Value: TAylisComponent);
begin
  SetRef('active_control', Value);
end;

function TForm.GetTabOrder: TStringArray;
begin
  Result := AsList(GetProp('tab_order'));
end;

procedure TForm.SetTabOrder(Value: TStringArray);
begin
  SetProp('tab_order', ListText(Value));
end;

function TForm.GetTabKeys: string;
begin
  Result := AsText(GetProp('tab_keys'));
end;

procedure TForm.SetTabKeys(Value: string);
begin
  SetProp('tab_keys', TJSONString.Create(Value));
end;

function TForm.GetBackTabKeys: string;
begin
  Result := AsText(GetProp('back_tab_keys'));
end;

procedure TForm.SetBackTabKeys(Value: string);
begin
  SetProp('back_tab_keys', TJSONString.Create(Value));
end;

function TForm.GetAllowedChars: string;
begin
  Result := AsText(GetProp('allowed_chars'));
end;

procedure TForm.SetAllowedChars(Value: string);
begin
  SetProp('allowed_chars', TJSONString.Create(Value));
end;

function TForm.GetBlockedChars: string;
begin
  Result := AsText(GetProp('blocked_chars'));
end;

procedure TForm.SetBlockedChars(Value: string);
begin
  SetProp('blocked_chars', TJSONString.Create(Value));
end;

function TForm.GetOnlyAllowed: Boolean;
begin
  Result := GetProp('only_allowed').AsBoolean;
end;

procedure TForm.SetOnlyAllowed(Value: Boolean);
begin
  SetProp('only_allowed', TJSONBoolean.Create(Value));
end;

function TForm.GetCharsOverride: Boolean;
begin
  Result := GetProp('chars_override').AsBoolean;
end;

procedure TForm.SetCharsOverride(Value: Boolean);
begin
  SetProp('chars_override', TJSONBoolean.Create(Value));
end;

function TForm.GetCharMap: string;
begin
  Result := AsText(GetProp('char_map'));
end;

procedure TForm.SetCharMap(Value: string);
begin
  SetProp('char_map', TJSONString.Create(Value));
end;

function TForm.GetKeyPreview: Boolean;
begin
  Result := GetProp('key_preview').AsBoolean;
end;

procedure TForm.SetKeyPreview(Value: Boolean);
begin
  SetProp('key_preview', TJSONBoolean.Create(Value));
end;

function TForm.GetZoom: Double;
begin
  Result := GetProp('zoom').AsFloat;
end;

procedure TForm.SetZoom(Value: Double);
begin
  SetProp('zoom', TJSONFloatNumber.Create(Value));
end;

function TForm.GetMonitor: Int64;
begin
  Result := GetProp('monitor').AsInt64;
end;

procedure TForm.SetMonitor(Value: Int64);
begin
  SetProp('monitor', TJSONInt64Number.Create(Value));
end;

function TForm.GetRememberPlacement: Boolean;
begin
  Result := GetProp('remember_placement').AsBoolean;
end;

procedure TForm.SetRememberPlacement(Value: Boolean);
begin
  SetProp('remember_placement', TJSONBoolean.Create(Value));
end;

function TForm.GetGpu: string;
begin
  Result := AsText(GetProp('gpu'));
end;

procedure TForm.SetGpu(Value: string);
begin
  SetProp('gpu', TJSONString.Create(Value));
end;

function TForm.GetPresent: TFormPresent;
begin
  Result := FormPresentOf(GetProp('present'));
end;

procedure TForm.SetPresent(Value: TFormPresent);
begin
  SetProp('present', TJSONString.Create(FormPresentNames[Value]));
end;

function TForm.GetSpanMonitors: Boolean;
begin
  Result := GetProp('span_monitors').AsBoolean;
end;

procedure TForm.SetSpanMonitors(Value: Boolean);
begin
  SetProp('span_monitors', TJSONBoolean.Create(Value));
end;

function TForm.GetPhysicalPixels: Boolean;
begin
  Result := GetProp('physical_pixels').AsBoolean;
end;

procedure TForm.SetPhysicalPixels(Value: Boolean);
begin
  SetProp('physical_pixels', TJSONBoolean.Create(Value));
end;

function TForm.GetActivateOnShow: Boolean;
begin
  Result := GetProp('activate_on_show').AsBoolean;
end;

procedure TForm.SetActivateOnShow(Value: Boolean);
begin
  SetProp('activate_on_show', TJSONBoolean.Create(Value));
end;

function TForm.GetForceForeground: Boolean;
begin
  Result := GetProp('force_foreground').AsBoolean;
end;

procedure TForm.SetForceForeground(Value: Boolean);
begin
  SetProp('force_foreground', TJSONBoolean.Create(Value));
end;

function TForm.GetCloseAction: TCloseAction;
begin
  Result := CloseActionOf(GetProp('close_action'));
end;

procedure TForm.SetCloseAction(Value: TCloseAction);
begin
  SetProp('close_action', TJSONString.Create(CloseActionNames[Value]));
end;

function TForm.GetFrameRate: Double;
begin
  Result := GetProp('frame_rate').AsFloat;
end;

procedure TForm.SetFrameRate(Value: Double);
begin
  SetProp('frame_rate', TJSONFloatNumber.Create(Value));
end;

function TForm.GetOrientation: TFormOrientation;
begin
  Result := FormOrientationOf(GetProp('orientation'));
end;

procedure TForm.SetOrientation(Value: TFormOrientation);
begin
  SetProp('orientation', TJSONString.Create(FormOrientationNames[Value]));
end;

function TForm.GetOnShow: string;
begin
  Result := AsText(GetProp('on_show'));
end;

procedure TForm.SetOnShow(Value: string);
begin
  SetProp('on_show', TJSONString.Create(Value));
end;

function TForm.GetOnWindowCreated: string;
begin
  Result := AsText(GetProp('on_window_created'));
end;

procedure TForm.SetOnWindowCreated(Value: string);
begin
  SetProp('on_window_created', TJSONString.Create(Value));
end;

function TForm.GetOnClose: string;
begin
  Result := AsText(GetProp('on_close'));
end;

procedure TForm.SetOnClose(Value: string);
begin
  SetProp('on_close', TJSONString.Create(Value));
end;

function TForm.GetOnActivate: string;
begin
  Result := AsText(GetProp('on_activate'));
end;

procedure TForm.SetOnActivate(Value: string);
begin
  SetProp('on_activate', TJSONString.Create(Value));
end;

function TForm.GetOnDeactivate: string;
begin
  Result := AsText(GetProp('on_deactivate'));
end;

procedure TForm.SetOnDeactivate(Value: string);
begin
  SetProp('on_deactivate', TJSONString.Create(Value));
end;

function TForm.GetOnMinimize: string;
begin
  Result := AsText(GetProp('on_minimize'));
end;

procedure TForm.SetOnMinimize(Value: string);
begin
  SetProp('on_minimize', TJSONString.Create(Value));
end;

function TForm.GetOnRestore: string;
begin
  Result := AsText(GetProp('on_restore'));
end;

procedure TForm.SetOnRestore(Value: string);
begin
  SetProp('on_restore', TJSONString.Create(Value));
end;

function TForm.GetOnNavigate: string;
begin
  Result := AsText(GetProp('on_navigate'));
end;

procedure TForm.SetOnNavigate(Value: string);
begin
  SetProp('on_navigate', TJSONString.Create(Value));
end;

function TForm.GetOnGpuChange: string;
begin
  Result := AsText(GetProp('on_gpu_change'));
end;

procedure TForm.SetOnGpuChange(Value: string);
begin
  SetProp('on_gpu_change', TJSONString.Create(Value));
end;

function TForm.GetClientWidth: Double;
begin
  Result := GetProp('client_width').AsFloat;
end;

procedure TForm.SetClientWidth(Value: Double);
begin
  SetProp('client_width', TJSONFloatNumber.Create(Value));
end;

function TForm.GetClientHeight: Double;
begin
  Result := GetProp('client_height').AsFloat;
end;

procedure TForm.SetClientHeight(Value: Double);
begin
  SetProp('client_height', TJSONFloatNumber.Create(Value));
end;

function TForm.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TForm.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TForm.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TForm.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TForm.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TForm.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TForm.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TForm.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TForm.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TForm.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TForm.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TForm.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TForm.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TForm.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TForm.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TForm.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TForm.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TForm.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TForm.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TForm.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TForm.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TForm.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TForm.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TForm.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TForm.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TForm.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TForm.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TForm.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TForm.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TForm.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TForm.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TForm.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TForm.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TForm.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TForm.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TForm.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TForm.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TForm.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TForm.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TForm.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TForm.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TForm.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TForm.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TForm.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TForm.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TForm.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TForm.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TForm.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TForm.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TForm.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TForm.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TForm.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TForm.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TForm.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TForm.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TForm.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TForm.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TForm.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TForm.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TForm.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TForm.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TForm.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TForm.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TForm.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TForm.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TForm.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TForm.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TForm.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TForm.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TForm.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TForm.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TForm.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TForm.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TForm.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TForm.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TForm.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TForm.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TForm.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TForm.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TForm.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TForm.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TForm.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TForm.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TForm.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TForm.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TForm.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TForm.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TForm.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TForm.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TForm.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TForm.GetStyle: TFormStyle;
begin
  Result := FormStyleOf(GetProp('style'));
end;

procedure TForm.SetStyle(Value: TFormStyle);
begin
  SetProp('style', TJSONString.Create(FormStyleNames[Value]));
end;

function TForm.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TForm.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TForm.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TForm.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TForm.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TForm.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TForm.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TForm.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TForm.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TForm.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TForm.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TForm.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TForm.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TForm.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TForm.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TForm.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TForm.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TForm.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TForm.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TForm.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TForm.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TForm.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TForm.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TForm.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TForm.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TForm.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TForm.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TForm.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TForm.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TForm.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TForm.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TForm.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TForm.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TForm.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TForm.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TForm.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TForm.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TForm.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TForm.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TForm.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TForm.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TForm.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TForm.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TForm.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TForm.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TForm.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TForm.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TForm.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TForm.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TForm.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TForm.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TForm.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TForm.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TForm.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TForm.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TForm.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TForm.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TForm.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TForm.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TForm.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TForm.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TForm.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TForm.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TForm.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TForm.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TForm.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TForm.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TForm.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TForm.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TForm.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TForm.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TForm.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TForm.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TForm.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TForm.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TForm.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TForm.MoveToMonitor(AMonitor: Int64);
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'move_to_monitor', 'args', TJSONArray.Create([AMonitor])]));
end;

function TForm.GpuAdapter: string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'gpu_adapter', 'args', TJSONArray.Create])));
end;

function TForm.GpuAdapters: TStringArray;
begin
  Result := AsList(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'gpu_adapters', 'args', TJSONArray.Create])));
end;

function TForm.RenderRgba(AX: Double; AY: Double; AWidth: Double; AHeight: Double; AScale: Double): string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'render_rgba', 'args', TJSONArray.Create([AX, AY, AWidth, AHeight, AScale])])));
end;

function TForm.Focused: string;
begin
  Result := AsText(Aylis.Call('focused', TJSONObject.Create(['target', Id])));
end;

procedure TForm.FocusNext;
begin
  Aylis.Send('focus_next', TJSONObject.Create(['target', Id]));
end;

procedure TForm.FocusPrev;
begin
  Aylis.Send('focus_prev', TJSONObject.Create(['target', Id]));
end;

function TGroupBox.GetCaption: string;
begin
  Result := AsText(GetProp('caption'));
end;

procedure TGroupBox.SetCaption(Value: string);
begin
  SetProp('caption', TJSONString.Create(Value));
end;

function TGroupBox.GetAllowedChars: string;
begin
  Result := AsText(GetProp('allowed_chars'));
end;

procedure TGroupBox.SetAllowedChars(Value: string);
begin
  SetProp('allowed_chars', TJSONString.Create(Value));
end;

function TGroupBox.GetBlockedChars: string;
begin
  Result := AsText(GetProp('blocked_chars'));
end;

procedure TGroupBox.SetBlockedChars(Value: string);
begin
  SetProp('blocked_chars', TJSONString.Create(Value));
end;

function TGroupBox.GetOnlyAllowed: Boolean;
begin
  Result := GetProp('only_allowed').AsBoolean;
end;

procedure TGroupBox.SetOnlyAllowed(Value: Boolean);
begin
  SetProp('only_allowed', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCharsOverride: Boolean;
begin
  Result := GetProp('chars_override').AsBoolean;
end;

procedure TGroupBox.SetCharsOverride(Value: Boolean);
begin
  SetProp('chars_override', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCharMap: string;
begin
  Result := AsText(GetProp('char_map'));
end;

procedure TGroupBox.SetCharMap(Value: string);
begin
  SetProp('char_map', TJSONString.Create(Value));
end;

function TGroupBox.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TGroupBox.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TGroupBox.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TGroupBox.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TGroupBox.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TGroupBox.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TGroupBox.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TGroupBox.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TGroupBox.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TGroupBox.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TGroupBox.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TGroupBox.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TGroupBox.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TGroupBox.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TGroupBox.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TGroupBox.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TGroupBox.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TGroupBox.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TGroupBox.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TGroupBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TGroupBox.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TGroupBox.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TGroupBox.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TGroupBox.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TGroupBox.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TGroupBox.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TGroupBox.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TGroupBox.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TGroupBox.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TGroupBox.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TGroupBox.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TGroupBox.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TGroupBox.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TGroupBox.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TGroupBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TGroupBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TGroupBox.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TGroupBox.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TGroupBox.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TGroupBox.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TGroupBox.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TGroupBox.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TGroupBox.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TGroupBox.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TGroupBox.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TGroupBox.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TGroupBox.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TGroupBox.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TGroupBox.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TGroupBox.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TGroupBox.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TGroupBox.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TGroupBox.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TGroupBox.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TGroupBox.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TGroupBox.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TGroupBox.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TGroupBox.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TGroupBox.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TGroupBox.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TGroupBox.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TGroupBox.GetStyle: TGroupBoxStyle;
begin
  Result := GroupBoxStyleOf(GetProp('style'));
end;

procedure TGroupBox.SetStyle(Value: TGroupBoxStyle);
begin
  SetProp('style', TJSONString.Create(GroupBoxStyleNames[Value]));
end;

function TGroupBox.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TGroupBox.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TGroupBox.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TGroupBox.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TGroupBox.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TGroupBox.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TGroupBox.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TGroupBox.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TGroupBox.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TGroupBox.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TGroupBox.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TGroupBox.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TGroupBox.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TGroupBox.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TGroupBox.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TGroupBox.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TGroupBox.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TGroupBox.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TGroupBox.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TGroupBox.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TGroupBox.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TGroupBox.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TGroupBox.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TGroupBox.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TGroupBox.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TGroupBox.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TGroupBox.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TGroupBox.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TGroupBox.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TGroupBox.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TGroupBox.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TGroupBox.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TGroupBox.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TGroupBox.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TGroupBox.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TGroupBox.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TGroupBox.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TGroupBox.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TGroupBox.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TGroupBox.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TGroupBox.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TGroupBox.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TGroupBox.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TGroupBox.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TGroupBox.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TGroupBox.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TGroupBox.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TGroupBox.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TGroupBox.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TGroupBox.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TGroupBox.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TGroupBox.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TGroupBox.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TGroupBox.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TGroupBox.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TGroupBox.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TGroupBox.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TGroupBox.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TGroupBox.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TGroupBox.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TGroupBox.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TGroupBox.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TGroupBox.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TGroupBox.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TGroupBox.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TGroupBox.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TGroupBox.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TGroupBox.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TGroupBox.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TImage.GetImage: string;
begin
  Result := AsText(GetProp('image'));
end;

procedure TImage.SetImage(Value: string);
begin
  SetProp('image', TJSONString.Create(Value));
end;

function TImage.GetFit: TImageFit;
begin
  Result := ImageFitOf(GetProp('fit'));
end;

procedure TImage.SetFit(Value: TImageFit);
begin
  SetProp('fit', TJSONString.Create(ImageFitNames[Value]));
end;

function TImage.GetRadius: Double;
begin
  Result := GetProp('radius').AsFloat;
end;

procedure TImage.SetRadius(Value: Double);
begin
  SetProp('radius', TJSONFloatNumber.Create(Value));
end;

function TImage.GetZoomable: Boolean;
begin
  Result := GetProp('zoomable').AsBoolean;
end;

procedure TImage.SetZoomable(Value: Boolean);
begin
  SetProp('zoomable', TJSONBoolean.Create(Value));
end;

function TImage.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TImage.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TImage.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TImage.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TImage.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TImage.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TImage.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TImage.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TImage.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TImage.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TImage.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TImage.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TImage.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TImage.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TImage.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TImage.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TImage.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TImage.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TImage.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TImage.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TImage.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TImage.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TImage.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TImage.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TImage.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TImage.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TImage.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TImage.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TImage.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TImage.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TImage.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TImage.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TImage.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TImage.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TImage.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TImage.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TImage.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TImage.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TImage.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TImage.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TImage.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TImage.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TImage.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TImage.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TImage.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TImage.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TImage.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TImage.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TImage.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TImage.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TImage.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TImage.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TImage.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TImage.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TImage.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TImage.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TImage.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TImage.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TImage.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TImage.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TImage.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TImage.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TImage.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TImage.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TImage.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TImage.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TImage.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TImage.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TImage.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TImage.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TImage.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TImage.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TImage.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TImage.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TImage.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TImage.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TImage.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TImage.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TImage.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TImage.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TImage.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TImage.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TImage.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TImage.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TImage.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TImage.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TImage.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TImage.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TImage.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TImage.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TImage.GetStyle: TImageStyle;
begin
  Result := ImageStyleOf(GetProp('style'));
end;

procedure TImage.SetStyle(Value: TImageStyle);
begin
  SetProp('style', TJSONString.Create(ImageStyleNames[Value]));
end;

function TImage.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TImage.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TImage.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TImage.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TImage.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TImage.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TImage.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TImage.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TImage.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TImage.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TImage.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TImage.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TImage.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TImage.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TImage.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TImage.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TImage.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TImage.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TImage.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TImage.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TImage.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TImage.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TImage.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TImage.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TImage.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TImage.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TImage.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TImage.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TImage.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TImage.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TImage.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TImage.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TImage.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TImage.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TImage.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TImage.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TImage.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TImage.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TImage.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TImage.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TImage.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TImage.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TImage.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TImage.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TImage.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TImage.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TImage.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TImage.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TImage.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TImage.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TImage.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TImage.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TImage.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TImage.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TImage.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TImage.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TImage.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TImage.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TImage.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TImage.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TImage.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TImage.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TImage.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TImage.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TImage.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TImage.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TImage.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TImage.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TImage.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TImage.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TImage.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TImage.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TImage.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TImage.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TImage.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TImage.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TLabel.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TLabel.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TLabel.GetHAlign: TTextAlign;
begin
  Result := TextAlignOf(GetProp('h_align'));
end;

procedure TLabel.SetHAlign(Value: TTextAlign);
begin
  SetProp('h_align', TJSONString.Create(TextAlignNames[Value]));
end;

function TLabel.GetVAlign: TTextAlign;
begin
  Result := TextAlignOf(GetProp('v_align'));
end;

procedure TLabel.SetVAlign(Value: TTextAlign);
begin
  SetProp('v_align', TJSONString.Create(TextAlignNames[Value]));
end;

function TLabel.GetWrap: Boolean;
begin
  Result := GetProp('wrap').AsBoolean;
end;

procedure TLabel.SetWrap(Value: Boolean);
begin
  SetProp('wrap', TJSONBoolean.Create(Value));
end;

function TLabel.GetTrimming: TTrimming;
begin
  Result := TrimmingOf(GetProp('trimming'));
end;

procedure TLabel.SetTrimming(Value: TTrimming);
begin
  SetProp('trimming', TJSONString.Create(TrimmingNames[Value]));
end;

function TLabel.GetBold: Boolean;
begin
  Result := GetProp('bold').AsBoolean;
end;

procedure TLabel.SetBold(Value: Boolean);
begin
  SetProp('bold', TJSONBoolean.Create(Value));
end;

function TLabel.GetColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('color'));
end;

procedure TLabel.SetColor(Value: TAylisColor);
begin
  SetProp('color', AylisColorToJSON(Value));
end;

function TLabel.GetHighlight: string;
begin
  Result := AsText(GetProp('highlight'));
end;

procedure TLabel.SetHighlight(Value: string);
begin
  SetProp('highlight', TJSONString.Create(Value));
end;

function TLabel.GetLetterSpacing: Double;
begin
  Result := GetProp('letter_spacing').AsFloat;
end;

procedure TLabel.SetLetterSpacing(Value: Double);
begin
  SetProp('letter_spacing', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTextCase: TTextCase;
begin
  Result := TextCaseOf(GetProp('text_case'));
end;

procedure TLabel.SetTextCase(Value: TTextCase);
begin
  SetProp('text_case', TJSONString.Create(TextCaseNames[Value]));
end;

function TLabel.GetMarkup: Boolean;
begin
  Result := GetProp('markup').AsBoolean;
end;

procedure TLabel.SetMarkup(Value: Boolean);
begin
  SetProp('markup', TJSONBoolean.Create(Value));
end;

function TLabel.GetOutline: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('outline'));
end;

procedure TLabel.SetOutline(Value: TAylisColor);
begin
  SetProp('outline', AylisColorToJSON(Value));
end;

function TLabel.GetOutlineWidth: Double;
begin
  Result := GetProp('outline_width').AsFloat;
end;

procedure TLabel.SetOutlineWidth(Value: Double);
begin
  SetProp('outline_width', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetLineHeight: Double;
begin
  Result := GetProp('line_height').AsFloat;
end;

procedure TLabel.SetLineHeight(Value: Double);
begin
  SetProp('line_height', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTextShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('text_shadow'));
end;

procedure TLabel.SetTextShadow(Value: TAylisShadow);
begin
  SetProp('text_shadow', AylisShadowToJSON(Value));
end;

function TLabel.GetFocusControl: TAylisComponent;
begin
  Result := GetRef('focus_control');
end;

procedure TLabel.SetFocusControl(Value: TAylisComponent);
begin
  SetRef('focus_control', Value);
end;

function TLabel.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TLabel.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TLabel.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TLabel.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TLabel.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TLabel.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TLabel.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TLabel.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TLabel.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TLabel.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TLabel.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TLabel.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TLabel.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TLabel.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TLabel.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TLabel.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TLabel.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TLabel.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TLabel.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TLabel.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TLabel.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TLabel.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TLabel.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TLabel.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TLabel.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TLabel.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TLabel.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TLabel.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TLabel.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TLabel.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TLabel.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TLabel.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TLabel.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TLabel.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TLabel.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TLabel.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TLabel.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TLabel.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TLabel.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TLabel.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TLabel.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TLabel.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TLabel.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TLabel.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TLabel.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TLabel.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TLabel.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TLabel.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TLabel.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TLabel.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TLabel.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TLabel.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TLabel.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TLabel.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TLabel.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TLabel.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TLabel.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TLabel.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TLabel.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TLabel.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TLabel.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TLabel.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TLabel.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TLabel.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TLabel.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TLabel.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TLabel.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TLabel.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TLabel.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TLabel.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TLabel.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TLabel.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TLabel.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TLabel.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TLabel.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TLabel.GetStyle: TLabelStyle;
begin
  Result := LabelStyleOf(GetProp('style'));
end;

procedure TLabel.SetStyle(Value: TLabelStyle);
begin
  SetProp('style', TJSONString.Create(LabelStyleNames[Value]));
end;

function TLabel.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TLabel.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TLabel.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TLabel.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TLabel.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TLabel.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TLabel.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TLabel.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TLabel.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TLabel.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TLabel.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TLabel.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TLabel.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TLabel.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TLabel.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TLabel.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TLabel.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TLabel.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TLabel.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TLabel.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TLabel.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TLabel.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TLabel.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TLabel.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TLabel.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TLabel.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TLabel.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TLabel.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TLabel.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TLabel.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TLabel.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TLabel.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TLabel.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TLabel.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TLabel.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TLabel.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TLabel.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TLabel.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TLabel.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TLabel.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TLabel.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TLabel.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TLabel.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TLabel.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TLabel.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TLabel.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TLabel.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TLabel.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TLabel.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TLabel.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TLabel.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TLabel.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TLabel.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TLabel.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TLabel.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TLabel.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TLabel.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TLabel.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TLabel.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TLabel.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TLabel.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TLabel.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TLabel.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TLabel.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TLabel.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TLabel.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TLabel.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TLabel.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TLabel.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TLabel.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TLabel.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TLabel.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TLabel.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TLabel.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TLabel.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TLabel.MinContentWidth: Double;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'min_content_width', 'args', TJSONArray.Create])).AsFloat;
end;

function TLabel.MaxContentWidth: Double;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'max_content_width', 'args', TJSONArray.Create])).AsFloat;
end;

function TLayout.GetAllowedChars: string;
begin
  Result := AsText(GetProp('allowed_chars'));
end;

procedure TLayout.SetAllowedChars(Value: string);
begin
  SetProp('allowed_chars', TJSONString.Create(Value));
end;

function TLayout.GetBlockedChars: string;
begin
  Result := AsText(GetProp('blocked_chars'));
end;

procedure TLayout.SetBlockedChars(Value: string);
begin
  SetProp('blocked_chars', TJSONString.Create(Value));
end;

function TLayout.GetOnlyAllowed: Boolean;
begin
  Result := GetProp('only_allowed').AsBoolean;
end;

procedure TLayout.SetOnlyAllowed(Value: Boolean);
begin
  SetProp('only_allowed', TJSONBoolean.Create(Value));
end;

function TLayout.GetCharsOverride: Boolean;
begin
  Result := GetProp('chars_override').AsBoolean;
end;

procedure TLayout.SetCharsOverride(Value: Boolean);
begin
  SetProp('chars_override', TJSONBoolean.Create(Value));
end;

function TLayout.GetCharMap: string;
begin
  Result := AsText(GetProp('char_map'));
end;

procedure TLayout.SetCharMap(Value: string);
begin
  SetProp('char_map', TJSONString.Create(Value));
end;

function TLayout.GetGap: Double;
begin
  Result := GetProp('gap').AsFloat;
end;

procedure TLayout.SetGap(Value: Double);
begin
  SetProp('gap', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TLayout.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TLayout.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TLayout.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TLayout.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TLayout.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TLayout.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TLayout.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TLayout.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TLayout.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TLayout.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TLayout.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TLayout.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TLayout.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TLayout.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TLayout.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TLayout.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TLayout.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TLayout.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TLayout.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TLayout.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TLayout.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TLayout.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TLayout.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TLayout.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TLayout.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TLayout.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TLayout.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TLayout.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TLayout.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TLayout.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TLayout.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TLayout.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TLayout.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TLayout.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TLayout.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TLayout.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TLayout.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TLayout.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TLayout.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TLayout.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TLayout.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TLayout.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TLayout.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TLayout.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TLayout.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TLayout.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TLayout.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TLayout.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TLayout.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TLayout.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TLayout.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TLayout.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TLayout.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TLayout.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TLayout.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TLayout.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TLayout.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TLayout.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TLayout.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TLayout.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TLayout.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TLayout.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TLayout.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TLayout.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TLayout.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TLayout.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TLayout.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TLayout.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TLayout.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TLayout.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TLayout.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TLayout.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TLayout.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TLayout.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TLayout.GetStyle: TLayoutStyle;
begin
  Result := LayoutStyleOf(GetProp('style'));
end;

procedure TLayout.SetStyle(Value: TLayoutStyle);
begin
  SetProp('style', TJSONString.Create(LayoutStyleNames[Value]));
end;

function TLayout.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TLayout.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TLayout.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TLayout.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TLayout.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TLayout.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TLayout.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TLayout.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TLayout.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TLayout.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TLayout.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TLayout.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TLayout.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TLayout.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TLayout.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TLayout.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TLayout.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TLayout.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TLayout.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TLayout.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TLayout.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TLayout.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TLayout.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TLayout.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TLayout.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TLayout.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TLayout.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TLayout.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TLayout.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TLayout.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TLayout.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TLayout.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TLayout.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TLayout.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TLayout.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TLayout.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TLayout.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TLayout.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TLayout.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TLayout.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TLayout.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TLayout.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TLayout.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TLayout.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TLayout.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TLayout.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TLayout.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TLayout.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TLayout.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TLayout.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TLayout.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TLayout.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TLayout.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TLayout.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TLayout.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TLayout.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TLayout.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TLayout.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TLayout.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TLayout.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TLayout.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TLayout.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TLayout.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TLayout.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TLayout.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TLayout.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TLayout.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TLayout.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TLayout.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TLayout.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TLayout.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TLayout.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TLayout.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TLayout.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TLayout.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TLayout.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TPanel.GetAllowedChars: string;
begin
  Result := AsText(GetProp('allowed_chars'));
end;

procedure TPanel.SetAllowedChars(Value: string);
begin
  SetProp('allowed_chars', TJSONString.Create(Value));
end;

function TPanel.GetBlockedChars: string;
begin
  Result := AsText(GetProp('blocked_chars'));
end;

procedure TPanel.SetBlockedChars(Value: string);
begin
  SetProp('blocked_chars', TJSONString.Create(Value));
end;

function TPanel.GetOnlyAllowed: Boolean;
begin
  Result := GetProp('only_allowed').AsBoolean;
end;

procedure TPanel.SetOnlyAllowed(Value: Boolean);
begin
  SetProp('only_allowed', TJSONBoolean.Create(Value));
end;

function TPanel.GetCharsOverride: Boolean;
begin
  Result := GetProp('chars_override').AsBoolean;
end;

procedure TPanel.SetCharsOverride(Value: Boolean);
begin
  SetProp('chars_override', TJSONBoolean.Create(Value));
end;

function TPanel.GetCharMap: string;
begin
  Result := AsText(GetProp('char_map'));
end;

procedure TPanel.SetCharMap(Value: string);
begin
  SetProp('char_map', TJSONString.Create(Value));
end;

function TPanel.GetGap: Double;
begin
  Result := GetProp('gap').AsFloat;
end;

procedure TPanel.SetGap(Value: Double);
begin
  SetProp('gap', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TPanel.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TPanel.GetFill2: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill2'));
end;

procedure TPanel.SetFill2(Value: TAylisColor);
begin
  SetProp('fill2', AylisColorToJSON(Value));
end;

function TPanel.GetFillMid: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill_mid'));
end;

procedure TPanel.SetFillMid(Value: TAylisColor);
begin
  SetProp('fill_mid', AylisColorToJSON(Value));
end;

function TPanel.GetFillMidAt: Double;
begin
  Result := GetProp('fill_mid_at').AsFloat;
end;

procedure TPanel.SetFillMidAt(Value: Double);
begin
  SetProp('fill_mid_at', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetGradientAngle: Double;
begin
  Result := GetProp('gradient_angle').AsFloat;
end;

procedure TPanel.SetGradientAngle(Value: Double);
begin
  SetProp('gradient_angle', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetGradientRadial: Boolean;
begin
  Result := GetProp('gradient_radial').AsBoolean;
end;

procedure TPanel.SetGradientRadial(Value: Boolean);
begin
  SetProp('gradient_radial', TJSONBoolean.Create(Value));
end;

function TPanel.GetStroke: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('stroke'));
end;

procedure TPanel.SetStroke(Value: TAylisColor);
begin
  SetProp('stroke', AylisColorToJSON(Value));
end;

function TPanel.GetStrokeWidth: Double;
begin
  Result := GetProp('stroke_width').AsFloat;
end;

procedure TPanel.SetStrokeWidth(Value: Double);
begin
  SetProp('stroke_width', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetRadius: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('radius'));
end;

procedure TPanel.SetRadius(Value: TAylisBounds);
begin
  SetProp('radius', AylisBoundsToJSON(Value));
end;

function TPanel.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TPanel.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TPanel.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TPanel.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TPanel.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TPanel.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TPanel.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TPanel.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TPanel.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TPanel.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TPanel.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TPanel.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TPanel.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TPanel.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TPanel.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TPanel.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TPanel.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TPanel.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TPanel.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TPanel.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TPanel.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TPanel.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TPanel.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TPanel.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TPanel.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TPanel.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TPanel.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TPanel.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TPanel.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TPanel.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TPanel.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TPanel.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TPanel.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TPanel.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TPanel.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TPanel.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TPanel.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TPanel.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TPanel.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TPanel.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TPanel.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TPanel.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TPanel.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TPanel.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TPanel.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TPanel.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TPanel.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TPanel.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TPanel.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TPanel.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TPanel.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TPanel.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TPanel.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TPanel.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TPanel.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TPanel.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TPanel.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TPanel.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TPanel.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TPanel.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TPanel.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TPanel.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TPanel.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TPanel.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TPanel.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TPanel.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TPanel.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TPanel.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TPanel.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TPanel.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TPanel.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TPanel.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TPanel.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TPanel.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TPanel.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TPanel.GetStyle: TPanelStyle;
begin
  Result := PanelStyleOf(GetProp('style'));
end;

procedure TPanel.SetStyle(Value: TPanelStyle);
begin
  SetProp('style', TJSONString.Create(PanelStyleNames[Value]));
end;

function TPanel.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TPanel.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TPanel.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TPanel.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TPanel.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TPanel.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TPanel.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TPanel.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TPanel.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TPanel.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TPanel.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TPanel.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TPanel.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TPanel.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TPanel.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TPanel.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TPanel.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TPanel.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TPanel.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TPanel.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TPanel.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TPanel.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TPanel.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TPanel.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TPanel.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TPanel.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TPanel.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TPanel.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TPanel.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TPanel.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TPanel.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TPanel.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TPanel.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TPanel.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TPanel.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TPanel.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TPanel.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TPanel.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TPanel.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TPanel.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TPanel.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TPanel.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TPanel.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TPanel.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TPanel.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TPanel.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TPanel.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TPanel.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TPanel.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TPanel.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TPanel.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TPanel.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TPanel.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TPanel.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TPanel.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TPanel.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TPanel.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TPanel.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TPanel.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TPanel.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TPanel.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TPanel.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TPanel.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TPanel.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TPanel.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TPanel.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TPanel.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TPanel.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TPanel.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TPanel.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TPanel.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TPanel.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TPanel.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TSplitter.GetMinSize: Double;
begin
  Result := GetProp('min_size').AsFloat;
end;

procedure TSplitter.SetMinSize(Value: Double);
begin
  SetProp('min_size', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TSplitter.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TSplitter.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TSplitter.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TSplitter.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TSplitter.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TSplitter.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TSplitter.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TSplitter.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TSplitter.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TSplitter.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TSplitter.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TSplitter.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TSplitter.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TSplitter.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TSplitter.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TSplitter.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TSplitter.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TSplitter.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TSplitter.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TSplitter.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TSplitter.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TSplitter.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TSplitter.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TSplitter.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSplitter.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TSplitter.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TSplitter.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TSplitter.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TSplitter.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TSplitter.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TSplitter.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TSplitter.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TSplitter.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TSplitter.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TSplitter.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TSplitter.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TSplitter.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TSplitter.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TSplitter.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TSplitter.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TSplitter.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TSplitter.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TSplitter.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TSplitter.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TSplitter.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TSplitter.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSplitter.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TSplitter.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TSplitter.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TSplitter.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TSplitter.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TSplitter.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TSplitter.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TSplitter.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TSplitter.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TSplitter.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TSplitter.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TSplitter.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TSplitter.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TSplitter.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TSplitter.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TSplitter.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TSplitter.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TSplitter.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TSplitter.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TSplitter.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TSplitter.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TSplitter.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TSplitter.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TSplitter.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TSplitter.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TSplitter.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TSplitter.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TSplitter.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TSplitter.GetStyle: TSplitterStyle;
begin
  Result := SplitterStyleOf(GetProp('style'));
end;

procedure TSplitter.SetStyle(Value: TSplitterStyle);
begin
  SetProp('style', TJSONString.Create(SplitterStyleNames[Value]));
end;

function TSplitter.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TSplitter.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TSplitter.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TSplitter.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TSplitter.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TSplitter.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TSplitter.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TSplitter.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TSplitter.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TSplitter.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TSplitter.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TSplitter.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TSplitter.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TSplitter.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TSplitter.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TSplitter.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TSplitter.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TSplitter.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TSplitter.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TSplitter.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TSplitter.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TSplitter.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TSplitter.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TSplitter.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TSplitter.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TSplitter.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TSplitter.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TSplitter.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TSplitter.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TSplitter.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TSplitter.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TSplitter.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TSplitter.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TSplitter.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TSplitter.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TSplitter.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TSplitter.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TSplitter.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TSplitter.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TSplitter.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TSplitter.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TSplitter.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TSplitter.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TSplitter.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TSplitter.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TSplitter.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TSplitter.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TSplitter.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TSplitter.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TSplitter.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TSplitter.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TSplitter.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TSplitter.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TSplitter.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TSplitter.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TSplitter.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TSplitter.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TSplitter.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TSplitter.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TSplitter.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TSplitter.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TSplitter.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TSplitter.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TSplitter.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TSplitter.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TSplitter.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TSplitter.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TSplitter.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TSplitter.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TSplitter.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TSplitter.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TSplitter.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TSplitter.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TSplitter.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TSplitter.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TSplitter.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TSvg.GetSource: string;
begin
  Result := AsText(GetProp('source'));
end;

procedure TSvg.SetSource(Value: string);
begin
  SetProp('source', TJSONString.Create(Value));
end;

function TSvg.GetColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('color'));
end;

procedure TSvg.SetColor(Value: TAylisColor);
begin
  SetProp('color', AylisColorToJSON(Value));
end;

function TSvg.GetFit: TImageFit;
begin
  Result := ImageFitOf(GetProp('fit'));
end;

procedure TSvg.SetFit(Value: TImageFit);
begin
  SetProp('fit', TJSONString.Create(ImageFitNames[Value]));
end;

function TSvg.GetHighlight: TStringArray;
begin
  Result := AsList(GetProp('highlight'));
end;

procedure TSvg.SetHighlight(Value: TStringArray);
begin
  SetProp('highlight', ListText(Value));
end;

function TSvg.GetHighlightColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('highlight_color'));
end;

procedure TSvg.SetHighlightColor(Value: TAylisColor);
begin
  SetProp('highlight_color', AylisColorToJSON(Value));
end;

function TSvg.GetZoomTo: string;
begin
  Result := AsText(GetProp('zoom_to'));
end;

procedure TSvg.SetZoomTo(Value: string);
begin
  SetProp('zoom_to', TJSONString.Create(Value));
end;

function TSvg.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TSvg.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TSvg.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TSvg.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TSvg.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TSvg.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TSvg.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TSvg.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TSvg.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TSvg.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TSvg.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TSvg.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TSvg.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TSvg.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TSvg.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TSvg.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TSvg.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TSvg.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TSvg.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TSvg.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TSvg.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TSvg.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TSvg.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TSvg.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TSvg.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSvg.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TSvg.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TSvg.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TSvg.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TSvg.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TSvg.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TSvg.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TSvg.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TSvg.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TSvg.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TSvg.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TSvg.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TSvg.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TSvg.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TSvg.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TSvg.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TSvg.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TSvg.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TSvg.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TSvg.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TSvg.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TSvg.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSvg.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TSvg.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TSvg.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TSvg.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TSvg.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TSvg.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TSvg.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TSvg.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TSvg.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TSvg.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TSvg.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TSvg.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TSvg.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TSvg.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TSvg.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TSvg.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TSvg.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TSvg.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TSvg.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TSvg.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TSvg.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TSvg.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TSvg.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TSvg.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TSvg.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TSvg.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TSvg.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TSvg.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TSvg.GetStyle: TSvgStyle;
begin
  Result := SvgStyleOf(GetProp('style'));
end;

procedure TSvg.SetStyle(Value: TSvgStyle);
begin
  SetProp('style', TJSONString.Create(SvgStyleNames[Value]));
end;

function TSvg.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TSvg.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TSvg.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TSvg.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TSvg.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TSvg.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TSvg.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TSvg.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TSvg.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TSvg.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TSvg.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TSvg.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TSvg.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TSvg.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TSvg.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TSvg.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TSvg.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TSvg.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TSvg.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TSvg.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TSvg.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TSvg.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TSvg.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TSvg.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TSvg.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TSvg.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TSvg.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TSvg.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TSvg.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TSvg.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TSvg.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TSvg.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TSvg.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TSvg.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TSvg.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TSvg.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TSvg.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TSvg.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TSvg.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TSvg.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TSvg.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TSvg.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TSvg.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TSvg.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TSvg.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TSvg.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TSvg.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TSvg.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TSvg.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TSvg.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TSvg.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TSvg.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TSvg.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TSvg.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TSvg.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TSvg.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TSvg.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TSvg.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TSvg.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TSvg.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TSvg.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TSvg.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TSvg.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TSvg.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TSvg.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TSvg.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TSvg.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TSvg.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TSvg.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TSvg.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TSvg.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TSvg.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TSvg.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TSvg.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TSvg.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TSvg.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TFrmIBGEBase.BindComponents(List: TJSONArray);
begin
  cdsEstados := TClientDataSet(Resolve(List, 'cdsEstados', 'ClientDataSet', TClientDataSet));
  cdsEstadosId := TField(Resolve(List, 'cdsEstadosId', 'Field', TField));
  cdsEstadosSigla := TField(Resolve(List, 'cdsEstadosSigla', 'Field', TField));
  cdsEstadosNome := TField(Resolve(List, 'cdsEstadosNome', 'Field', TField));
  cdsEstadosRegiao := TField(Resolve(List, 'cdsEstadosRegiao', 'Field', TField));
  cdsEstadosPopulacao := TField(Resolve(List, 'cdsEstadosPopulacao', 'Field', TField));
  cdsEstadosArea := TField(Resolve(List, 'cdsEstadosArea', 'Field', TField));
  cdsEstadosDensidade := TField(Resolve(List, 'cdsEstadosDensidade', 'Field', TField));
  cdsEstadosPib := TField(Resolve(List, 'cdsEstadosPib', 'Field', TField));
  cdsEstadosPibPerCapita := TField(Resolve(List, 'cdsEstadosPibPerCapita', 'Field', TField));
  cdsEstadosEstimativa := TField(Resolve(List, 'cdsEstadosEstimativa', 'Field', TField));
  cdsEstadosEstimativaAno := TField(Resolve(List, 'cdsEstadosEstimativaAno', 'Field', TField));
  cdsEstadosMunicipios := TField(Resolve(List, 'cdsEstadosMunicipios', 'Field', TField));
  cdsEstadosBandeira := TField(Resolve(List, 'cdsEstadosBandeira', 'Field', TField));
  dsEstados := TDataSource(Resolve(List, 'dsEstados', 'DataSource', TDataSource));
  cdsMunicipios := TClientDataSet(Resolve(List, 'cdsMunicipios', 'ClientDataSet', TClientDataSet));
  cdsMunicipiosId := TField(Resolve(List, 'cdsMunicipiosId', 'Field', TField));
  cdsMunicipiosUfId := TField(Resolve(List, 'cdsMunicipiosUfId', 'Field', TField));
  cdsMunicipiosUf := TField(Resolve(List, 'cdsMunicipiosUf', 'Field', TField));
  cdsMunicipiosNome := TField(Resolve(List, 'cdsMunicipiosNome', 'Field', TField));
  cdsMunicipiosMesorregiao := TField(Resolve(List, 'cdsMunicipiosMesorregiao', 'Field', TField));
  cdsMunicipiosMicrorregiao := TField(Resolve(List, 'cdsMunicipiosMicrorregiao', 'Field', TField));
  cdsMunicipiosRegiaoIntermediaria := TField(Resolve(List, 'cdsMunicipiosRegiaoIntermediaria', 'Field', TField));
  cdsMunicipiosRegiaoImediata := TField(Resolve(List, 'cdsMunicipiosRegiaoImediata', 'Field', TField));
  cdsMunicipiosPopulacao := TField(Resolve(List, 'cdsMunicipiosPopulacao', 'Field', TField));
  cdsMunicipiosArea := TField(Resolve(List, 'cdsMunicipiosArea', 'Field', TField));
  cdsMunicipiosDensidade := TField(Resolve(List, 'cdsMunicipiosDensidade', 'Field', TField));
  cdsMunicipiosPib := TField(Resolve(List, 'cdsMunicipiosPib', 'Field', TField));
  cdsMunicipiosPibParticipacao := TField(Resolve(List, 'cdsMunicipiosPibParticipacao', 'Field', TField));
  cdsMunicipiosPibPerCapita := TField(Resolve(List, 'cdsMunicipiosPibPerCapita', 'Field', TField));
  cdsMunicipiosEstimativa := TField(Resolve(List, 'cdsMunicipiosEstimativa', 'Field', TField));
  cdsMunicipiosEstimativaAno := TField(Resolve(List, 'cdsMunicipiosEstimativaAno', 'Field', TField));
  cdsMunicipiosDistritos := TField(Resolve(List, 'cdsMunicipiosDistritos', 'Field', TField));
  cdsMunicipiosBandeira := TField(Resolve(List, 'cdsMunicipiosBandeira', 'Field', TField));
  dsMunicipios := TDataSource(Resolve(List, 'dsMunicipios', 'DataSource', TDataSource));
  cdsDistritos := TClientDataSet(Resolve(List, 'cdsDistritos', 'ClientDataSet', TClientDataSet));
  cdsDistritosId := TField(Resolve(List, 'cdsDistritosId', 'Field', TField));
  cdsDistritosMunicipioId := TField(Resolve(List, 'cdsDistritosMunicipioId', 'Field', TField));
  cdsDistritosNome := TField(Resolve(List, 'cdsDistritosNome', 'Field', TField));
  dsDistritos := TDataSource(Resolve(List, 'dsDistritos', 'DataSource', TDataSource));
  cdsPopulacao := TClientDataSet(Resolve(List, 'cdsPopulacao', 'ClientDataSet', TClientDataSet));
  cdsPopulacaoAno := TField(Resolve(List, 'cdsPopulacaoAno', 'Field', TField));
  cdsPopulacaoHabitantes := TField(Resolve(List, 'cdsPopulacaoHabitantes', 'Field', TField));
  dsPopulacao := TDataSource(Resolve(List, 'dsPopulacao', 'DataSource', TDataSource));
  cdsMaiores := TClientDataSet(Resolve(List, 'cdsMaiores', 'ClientDataSet', TClientDataSet));
  cdsMaioresNome := TField(Resolve(List, 'cdsMaioresNome', 'Field', TField));
  cdsMaioresHabitantes := TField(Resolve(List, 'cdsMaioresHabitantes', 'Field', TField));
  dsMaiores := TDataSource(Resolve(List, 'dsMaiores', 'DataSource', TDataSource));
  lblFonte := TLabel(Resolve(List, 'lblFonte', 'Label', TLabel));
  lytListas := TLayout(Resolve(List, 'lytListas', 'Layout', TLayout));
  gbEstados := TGroupBox(Resolve(List, 'gbEstados', 'GroupBox', TGroupBox));
  grdEstados := TDataGrid(Resolve(List, 'grdEstados', 'DataGrid', TDataGrid));
  gbDistritos := TGroupBox(Resolve(List, 'gbDistritos', 'GroupBox', TGroupBox));
  grdDistritos := TDataGrid(Resolve(List, 'grdDistritos', 'DataGrid', TDataGrid));
  gbMunicipios := TGroupBox(Resolve(List, 'gbMunicipios', 'GroupBox', TGroupBox));
  grdMunicipios := TDataGrid(Resolve(List, 'grdMunicipios', 'DataGrid', TDataGrid));
  splListas := TSplitter(Resolve(List, 'splListas', 'Splitter', TSplitter));
  lytPainel := TLayout(Resolve(List, 'lytPainel', 'Layout', TLayout));
  pnlEstado := TPanel(Resolve(List, 'pnlEstado', 'Panel', TPanel));
  svgBrasil := TSvg(Resolve(List, 'svgBrasil', 'Svg', TSvg));
  lytBandeiraEstado := TLayout(Resolve(List, 'lytBandeiraEstado', 'Layout', TLayout));
  lblEstado := TLabel(Resolve(List, 'lblEstado', 'Label', TLabel));
  lblRegiao := TLabel(Resolve(List, 'lblRegiao', 'Label', TLabel));
  imgBandeiraEstado := TImage(Resolve(List, 'imgBandeiraEstado', 'Image', TImage));
  lytNumerosEstado := TLayout(Resolve(List, 'lytNumerosEstado', 'Layout', TLayout));
  dbtEstadoPopulacao := TLabel(Resolve(List, 'dbtEstadoPopulacao', 'Label', TLabel));
  lblEstadoEstimativa := TLabel(Resolve(List, 'lblEstadoEstimativa', 'Label', TLabel));
  dbtEstadoEstimativa := TLabel(Resolve(List, 'dbtEstadoEstimativa', 'Label', TLabel));
  dbtEstadoArea := TLabel(Resolve(List, 'dbtEstadoArea', 'Label', TLabel));
  dbtEstadoDensidade := TLabel(Resolve(List, 'dbtEstadoDensidade', 'Label', TLabel));
  dbtEstadoPib := TLabel(Resolve(List, 'dbtEstadoPib', 'Label', TLabel));
  dbtEstadoPibPerCapita := TLabel(Resolve(List, 'dbtEstadoPibPerCapita', 'Label', TLabel));
  dbtEstadoMunicipios := TLabel(Resolve(List, 'dbtEstadoMunicipios', 'Label', TLabel));
  pnlMunicipio := TPanel(Resolve(List, 'pnlMunicipio', 'Panel', TPanel));
  svgMunicipio := TSvg(Resolve(List, 'svgMunicipio', 'Svg', TSvg));
  svgEstado := TSvg(Resolve(List, 'svgEstado', 'Svg', TSvg));
  lytBandeiraMunicipio := TLayout(Resolve(List, 'lytBandeiraMunicipio', 'Layout', TLayout));
  lblMunicipio := TLabel(Resolve(List, 'lblMunicipio', 'Label', TLabel));
  lblCodigo := TLabel(Resolve(List, 'lblCodigo', 'Label', TLabel));
  imgBandeiraMunicipio := TImage(Resolve(List, 'imgBandeiraMunicipio', 'Image', TImage));
  lblSemBandeira := TLabel(Resolve(List, 'lblSemBandeira', 'Label', TLabel));
  lytNumerosMunicipio := TLayout(Resolve(List, 'lytNumerosMunicipio', 'Layout', TLayout));
  dbtMesorregiao := TLabel(Resolve(List, 'dbtMesorregiao', 'Label', TLabel));
  dbtMicrorregiao := TLabel(Resolve(List, 'dbtMicrorregiao', 'Label', TLabel));
  dbtRegiaoImediata := TLabel(Resolve(List, 'dbtRegiaoImediata', 'Label', TLabel));
  dbtPopulacao := TLabel(Resolve(List, 'dbtPopulacao', 'Label', TLabel));
  lblEstimativa := TLabel(Resolve(List, 'lblEstimativa', 'Label', TLabel));
  dbtEstimativa := TLabel(Resolve(List, 'dbtEstimativa', 'Label', TLabel));
  dbtArea := TLabel(Resolve(List, 'dbtArea', 'Label', TLabel));
  dbtDensidade := TLabel(Resolve(List, 'dbtDensidade', 'Label', TLabel));
  dbtPib := TLabel(Resolve(List, 'dbtPib', 'Label', TLabel));
  dbtPibPerCapita := TLabel(Resolve(List, 'dbtPibPerCapita', 'Label', TLabel));
  dbtPibParticipacao := TLabel(Resolve(List, 'dbtPibParticipacao', 'Label', TLabel));
  lytGraficos := TLayout(Resolve(List, 'lytGraficos', 'Layout', TLayout));
  dbcPopulacao := TDBChart(Resolve(List, 'dbcPopulacao', 'DBChart', TDBChart));
  dbcMaiores := TDBChart(Resolve(List, 'dbcMaiores', 'DBChart', TDBChart));
  dbcEstados := TDBChart(Resolve(List, 'dbcEstados', 'DBChart', TDBChart));
end;

function TFrmIBGEBase.FindClientDataSet(const Nome: string): TClientDataSet;
begin
  Result := nil;
  if Nome = 'cdsEstados' then
    Result := cdsEstados;
  if Nome = 'cdsMunicipios' then
    Result := cdsMunicipios;
  if Nome = 'cdsDistritos' then
    Result := cdsDistritos;
  if Nome = 'cdsPopulacao' then
    Result := cdsPopulacao;
  if Nome = 'cdsMaiores' then
    Result := cdsMaiores;
end;

function TFrmIBGEBase.SenderAsClientDataSet(Sender: TAylisComponent): TClientDataSet;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindClientDataSet(Sender.Name);
end;

function TFrmIBGEBase.SenderAsClientDataSet(Event: TJSONObject): TClientDataSet;
begin
  Result := FindClientDataSet(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindDBChart(const Nome: string): TDBChart;
begin
  Result := nil;
  if Nome = 'dbcPopulacao' then
    Result := dbcPopulacao;
  if Nome = 'dbcMaiores' then
    Result := dbcMaiores;
  if Nome = 'dbcEstados' then
    Result := dbcEstados;
end;

function TFrmIBGEBase.SenderAsDBChart(Sender: TAylisComponent): TDBChart;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindDBChart(Sender.Name);
end;

function TFrmIBGEBase.SenderAsDBChart(Event: TJSONObject): TDBChart;
begin
  Result := FindDBChart(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindDataGrid(const Nome: string): TDataGrid;
begin
  Result := nil;
  if Nome = 'grdEstados' then
    Result := grdEstados;
  if Nome = 'grdDistritos' then
    Result := grdDistritos;
  if Nome = 'grdMunicipios' then
    Result := grdMunicipios;
end;

function TFrmIBGEBase.SenderAsDataGrid(Sender: TAylisComponent): TDataGrid;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindDataGrid(Sender.Name);
end;

function TFrmIBGEBase.SenderAsDataGrid(Event: TJSONObject): TDataGrid;
begin
  Result := FindDataGrid(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindDataSource(const Nome: string): TDataSource;
begin
  Result := nil;
  if Nome = 'dsEstados' then
    Result := dsEstados;
  if Nome = 'dsMunicipios' then
    Result := dsMunicipios;
  if Nome = 'dsDistritos' then
    Result := dsDistritos;
  if Nome = 'dsPopulacao' then
    Result := dsPopulacao;
  if Nome = 'dsMaiores' then
    Result := dsMaiores;
end;

function TFrmIBGEBase.SenderAsDataSource(Sender: TAylisComponent): TDataSource;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindDataSource(Sender.Name);
end;

function TFrmIBGEBase.SenderAsDataSource(Event: TJSONObject): TDataSource;
begin
  Result := FindDataSource(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindField(const Nome: string): TField;
begin
  Result := nil;
  if Nome = 'cdsEstadosId' then
    Result := cdsEstadosId;
  if Nome = 'cdsEstadosSigla' then
    Result := cdsEstadosSigla;
  if Nome = 'cdsEstadosNome' then
    Result := cdsEstadosNome;
  if Nome = 'cdsEstadosRegiao' then
    Result := cdsEstadosRegiao;
  if Nome = 'cdsEstadosPopulacao' then
    Result := cdsEstadosPopulacao;
  if Nome = 'cdsEstadosArea' then
    Result := cdsEstadosArea;
  if Nome = 'cdsEstadosDensidade' then
    Result := cdsEstadosDensidade;
  if Nome = 'cdsEstadosPib' then
    Result := cdsEstadosPib;
  if Nome = 'cdsEstadosPibPerCapita' then
    Result := cdsEstadosPibPerCapita;
  if Nome = 'cdsEstadosEstimativa' then
    Result := cdsEstadosEstimativa;
  if Nome = 'cdsEstadosEstimativaAno' then
    Result := cdsEstadosEstimativaAno;
  if Nome = 'cdsEstadosMunicipios' then
    Result := cdsEstadosMunicipios;
  if Nome = 'cdsEstadosBandeira' then
    Result := cdsEstadosBandeira;
  if Nome = 'cdsMunicipiosId' then
    Result := cdsMunicipiosId;
  if Nome = 'cdsMunicipiosUfId' then
    Result := cdsMunicipiosUfId;
  if Nome = 'cdsMunicipiosUf' then
    Result := cdsMunicipiosUf;
  if Nome = 'cdsMunicipiosNome' then
    Result := cdsMunicipiosNome;
  if Nome = 'cdsMunicipiosMesorregiao' then
    Result := cdsMunicipiosMesorregiao;
  if Nome = 'cdsMunicipiosMicrorregiao' then
    Result := cdsMunicipiosMicrorregiao;
  if Nome = 'cdsMunicipiosRegiaoIntermediaria' then
    Result := cdsMunicipiosRegiaoIntermediaria;
  if Nome = 'cdsMunicipiosRegiaoImediata' then
    Result := cdsMunicipiosRegiaoImediata;
  if Nome = 'cdsMunicipiosPopulacao' then
    Result := cdsMunicipiosPopulacao;
  if Nome = 'cdsMunicipiosArea' then
    Result := cdsMunicipiosArea;
  if Nome = 'cdsMunicipiosDensidade' then
    Result := cdsMunicipiosDensidade;
  if Nome = 'cdsMunicipiosPib' then
    Result := cdsMunicipiosPib;
  if Nome = 'cdsMunicipiosPibParticipacao' then
    Result := cdsMunicipiosPibParticipacao;
  if Nome = 'cdsMunicipiosPibPerCapita' then
    Result := cdsMunicipiosPibPerCapita;
  if Nome = 'cdsMunicipiosEstimativa' then
    Result := cdsMunicipiosEstimativa;
  if Nome = 'cdsMunicipiosEstimativaAno' then
    Result := cdsMunicipiosEstimativaAno;
  if Nome = 'cdsMunicipiosDistritos' then
    Result := cdsMunicipiosDistritos;
  if Nome = 'cdsMunicipiosBandeira' then
    Result := cdsMunicipiosBandeira;
  if Nome = 'cdsDistritosId' then
    Result := cdsDistritosId;
  if Nome = 'cdsDistritosMunicipioId' then
    Result := cdsDistritosMunicipioId;
  if Nome = 'cdsDistritosNome' then
    Result := cdsDistritosNome;
  if Nome = 'cdsPopulacaoAno' then
    Result := cdsPopulacaoAno;
  if Nome = 'cdsPopulacaoHabitantes' then
    Result := cdsPopulacaoHabitantes;
  if Nome = 'cdsMaioresNome' then
    Result := cdsMaioresNome;
  if Nome = 'cdsMaioresHabitantes' then
    Result := cdsMaioresHabitantes;
end;

function TFrmIBGEBase.SenderAsField(Sender: TAylisComponent): TField;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindField(Sender.Name);
end;

function TFrmIBGEBase.SenderAsField(Event: TJSONObject): TField;
begin
  Result := FindField(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindGroupBox(const Nome: string): TGroupBox;
begin
  Result := nil;
  if Nome = 'gbEstados' then
    Result := gbEstados;
  if Nome = 'gbDistritos' then
    Result := gbDistritos;
  if Nome = 'gbMunicipios' then
    Result := gbMunicipios;
end;

function TFrmIBGEBase.SenderAsGroupBox(Sender: TAylisComponent): TGroupBox;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindGroupBox(Sender.Name);
end;

function TFrmIBGEBase.SenderAsGroupBox(Event: TJSONObject): TGroupBox;
begin
  Result := FindGroupBox(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindImage(const Nome: string): TImage;
begin
  Result := nil;
  if Nome = 'imgBandeiraEstado' then
    Result := imgBandeiraEstado;
  if Nome = 'imgBandeiraMunicipio' then
    Result := imgBandeiraMunicipio;
end;

function TFrmIBGEBase.SenderAsImage(Sender: TAylisComponent): TImage;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindImage(Sender.Name);
end;

function TFrmIBGEBase.SenderAsImage(Event: TJSONObject): TImage;
begin
  Result := FindImage(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblFonte' then
    Result := lblFonte;
  if Nome = 'lblEstado' then
    Result := lblEstado;
  if Nome = 'lblRegiao' then
    Result := lblRegiao;
  if Nome = 'dbtEstadoPopulacao' then
    Result := dbtEstadoPopulacao;
  if Nome = 'lblEstadoEstimativa' then
    Result := lblEstadoEstimativa;
  if Nome = 'dbtEstadoEstimativa' then
    Result := dbtEstadoEstimativa;
  if Nome = 'dbtEstadoArea' then
    Result := dbtEstadoArea;
  if Nome = 'dbtEstadoDensidade' then
    Result := dbtEstadoDensidade;
  if Nome = 'dbtEstadoPib' then
    Result := dbtEstadoPib;
  if Nome = 'dbtEstadoPibPerCapita' then
    Result := dbtEstadoPibPerCapita;
  if Nome = 'dbtEstadoMunicipios' then
    Result := dbtEstadoMunicipios;
  if Nome = 'lblMunicipio' then
    Result := lblMunicipio;
  if Nome = 'lblCodigo' then
    Result := lblCodigo;
  if Nome = 'lblSemBandeira' then
    Result := lblSemBandeira;
  if Nome = 'dbtMesorregiao' then
    Result := dbtMesorregiao;
  if Nome = 'dbtMicrorregiao' then
    Result := dbtMicrorregiao;
  if Nome = 'dbtRegiaoImediata' then
    Result := dbtRegiaoImediata;
  if Nome = 'dbtPopulacao' then
    Result := dbtPopulacao;
  if Nome = 'lblEstimativa' then
    Result := lblEstimativa;
  if Nome = 'dbtEstimativa' then
    Result := dbtEstimativa;
  if Nome = 'dbtArea' then
    Result := dbtArea;
  if Nome = 'dbtDensidade' then
    Result := dbtDensidade;
  if Nome = 'dbtPib' then
    Result := dbtPib;
  if Nome = 'dbtPibPerCapita' then
    Result := dbtPibPerCapita;
  if Nome = 'dbtPibParticipacao' then
    Result := dbtPibParticipacao;
end;

function TFrmIBGEBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmIBGEBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindLayout(const Nome: string): TLayout;
begin
  Result := nil;
  if Nome = 'lytListas' then
    Result := lytListas;
  if Nome = 'lytPainel' then
    Result := lytPainel;
  if Nome = 'lytBandeiraEstado' then
    Result := lytBandeiraEstado;
  if Nome = 'lytNumerosEstado' then
    Result := lytNumerosEstado;
  if Nome = 'lytBandeiraMunicipio' then
    Result := lytBandeiraMunicipio;
  if Nome = 'lytNumerosMunicipio' then
    Result := lytNumerosMunicipio;
  if Nome = 'lytGraficos' then
    Result := lytGraficos;
end;

function TFrmIBGEBase.SenderAsLayout(Sender: TAylisComponent): TLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLayout(Sender.Name);
end;

function TFrmIBGEBase.SenderAsLayout(Event: TJSONObject): TLayout;
begin
  Result := FindLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindPanel(const Nome: string): TPanel;
begin
  Result := nil;
  if Nome = 'pnlEstado' then
    Result := pnlEstado;
  if Nome = 'pnlMunicipio' then
    Result := pnlMunicipio;
end;

function TFrmIBGEBase.SenderAsPanel(Sender: TAylisComponent): TPanel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindPanel(Sender.Name);
end;

function TFrmIBGEBase.SenderAsPanel(Event: TJSONObject): TPanel;
begin
  Result := FindPanel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindSplitter(const Nome: string): TSplitter;
begin
  Result := nil;
  if Nome = 'splListas' then
    Result := splListas;
end;

function TFrmIBGEBase.SenderAsSplitter(Sender: TAylisComponent): TSplitter;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindSplitter(Sender.Name);
end;

function TFrmIBGEBase.SenderAsSplitter(Event: TJSONObject): TSplitter;
begin
  Result := FindSplitter(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmIBGEBase.FindSvg(const Nome: string): TSvg;
begin
  Result := nil;
  if Nome = 'svgBrasil' then
    Result := svgBrasil;
  if Nome = 'svgMunicipio' then
    Result := svgMunicipio;
  if Nome = 'svgEstado' then
    Result := svgEstado;
end;

function TFrmIBGEBase.SenderAsSvg(Sender: TAylisComponent): TSvg;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindSvg(Sender.Name);
end;

function TFrmIBGEBase.SenderAsSvg(Event: TJSONObject): TSvg;
begin
  Result := FindSvg(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

initialization
  AylisFormat := DefaultFormatSettings;
  AylisFormat.DecimalSeparator := '.';
end.
