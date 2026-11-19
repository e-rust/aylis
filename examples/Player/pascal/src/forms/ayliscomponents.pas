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
  TButtonStyle = (Default, Accent, Danger, Elevated, Fab, FabExtended, Flat, Ghost, Glass, GlassProminent, Glossy, Icon, IconFilled, IconOutlined, IconTonal, Info, Link, Negative, Outline, Pill, Positive, Push, Reveal, Round, Rounded, Secondary, Success, Text, Tonal, Unelevated, Warning);
  {$pop}
  TButtonStyleSet = set of TButtonStyle;

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
  TDividerStyle = (Default);
  {$pop}
  TDividerStyleSet = set of TDividerStyle;

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
  TIconPosition = (Left, Right, Top);
  {$pop}
  TIconPositionSet = set of TIconPosition;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TIconStyle = (Default, Muted);
  {$pop}
  TIconStyleSet = set of TIconStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TImageFit = (Contain, Cover, Stretch, Center, Original);
  {$pop}
  TImageFitSet = set of TImageFit;

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
  TListViewStyle = (Default, Flat);
  {$pop}
  TListViewStyleSet = set of TListViewStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TMainMenuStyle = (Default);
  {$pop}
  TMainMenuStyleSet = set of TMainMenuStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TModalResult = (None, Ok, Cancel, Abort, Retry, Ignore, Yes, No, Close);
  {$pop}
  TModalResultSet = set of TModalResult;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TPanelStyle = (Default, Appbar, Card, Filled, Flat, Glass, Outlined, Overlay, Reveal, Sidebar);
  {$pop}
  TPanelStyleSet = set of TPanelStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSeekBarStyle = (Default);
  {$pop}
  TSeekBarStyleSet = set of TSeekBarStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSelfAlign = (Stretch, Start, Center, &End);
  {$pop}
  TSelfAlignSet = set of TSelfAlign;

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
  TTimerPrecision = (Normal, High, Exact);
  {$pop}
  TTimerPrecisionSet = set of TTimerPrecision;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTrackBarStyle = (Default);
  {$pop}
  TTrackBarStyleSet = set of TTrackBarStyle;

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
  TVideoPlayerStyle = (Default);
  {$pop}
  TVideoPlayerStyleSet = set of TVideoPlayerStyle;

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

  PGestureEvent = ^TGestureEvent;
  PKeyEvent = ^TKeyEvent;
  PListViewItem = ^TListViewItem;
  POutputSpec = ^TOutputSpec;
  PPointerEvent = ^TPointerEvent;
  PSwipeEvent = ^TSwipeEvent;
  PWheelEvent = ^TWheelEvent;
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
  TKeyEvent = record
    Key: string;
    Ctrl: Boolean;
    Shift: Boolean;
    Alt: Boolean;
  end;
  TKeyEventArray = array of TKeyEvent;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TListViewItem = record
    Title: string;
    Detail: string;
  end;
  TListViewItemArray = array of TListViewItem;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TOutputSpec = record
    Name: string;
    Volume: Double;
    DelayMs: Double;
  end;
  TOutputSpecArray = array of TOutputSpec;

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

  { Um Button de um form. }
  TButton = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetIcon: string;
    procedure SetIcon(Value: string);
    function GetCheckedIcon: string;
    procedure SetCheckedIcon(Value: string);
    function GetIconPosition: TIconPosition;
    procedure SetIconPosition(Value: TIconPosition);
    function GetIconSize: Double;
    procedure SetIconSize(Value: Double);
    function GetCheckable: Boolean;
    procedure SetCheckable(Value: Boolean);
    function GetChecked: Boolean;
    procedure SetChecked(Value: Boolean);
    function GetModalResult: TModalResult;
    procedure SetModalResult(Value: TModalResult);
    function GetRepeatEnter: Boolean;
    procedure SetRepeatEnter(Value: Boolean);
    function GetDefaultButton: Boolean;
    procedure SetDefaultButton(Value: Boolean);
    function GetCancelButton: Boolean;
    procedure SetCancelButton(Value: Boolean);
    function GetHAlign: TTextAlign;
    procedure SetHAlign(Value: TTextAlign);
    function GetGroupIndex: Int64;
    procedure SetGroupIndex(Value: Int64);
    function GetRadius: TAylisBounds;
    procedure SetRadius(Value: TAylisBounds);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
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
    function GetStyle: TButtonStyle;
    procedure SetStyle(Value: TButtonStyle);
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
    property Text: string read GetText write SetText;
    property Icon: string read GetIcon write SetIcon;
    property CheckedIcon: string read GetCheckedIcon write SetCheckedIcon;
    property IconPosition: TIconPosition read GetIconPosition write SetIconPosition;
    property IconSize: Double read GetIconSize write SetIconSize;
    property Checkable: Boolean read GetCheckable write SetCheckable;
    property Checked: Boolean read GetChecked write SetChecked;
    property ModalResult: TModalResult read GetModalResult write SetModalResult;
    property RepeatEnter: Boolean read GetRepeatEnter write SetRepeatEnter;
    property DefaultButton: Boolean read GetDefaultButton write SetDefaultButton;
    property CancelButton: Boolean read GetCancelButton write SetCancelButton;
    property HAlign: TTextAlign read GetHAlign write SetHAlign;
    property GroupIndex: Int64 read GetGroupIndex write SetGroupIndex;
    property Radius: TAylisBounds read GetRadius write SetRadius;
    property OnChange: string read GetOnChange write SetOnChange;
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
    property Style: TButtonStyle read GetStyle write SetStyle;
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

  { Um Divider de um form. }
  TDivider = class(TAylisComponent)
  private
    function GetVertical: Boolean;
    procedure SetVertical(Value: Boolean);
    function GetText: string;
    procedure SetText(Value: string);
    function GetThickness: Double;
    procedure SetThickness(Value: Double);
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
    function GetStyle: TDividerStyle;
    procedure SetStyle(Value: TDividerStyle);
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
    property Vertical: Boolean read GetVertical write SetVertical;
    property Text: string read GetText write SetText;
    property Thickness: Double read GetThickness write SetThickness;
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
    property Style: TDividerStyle read GetStyle write SetStyle;
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

  { Um Icon de um form. }
  TIcon = class(TAylisComponent)
  private
    function GetIcon: string;
    procedure SetIcon(Value: string);
    function GetColor: TAylisColor;
    procedure SetColor(Value: TAylisColor);
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
    function GetStyle: TIconStyle;
    procedure SetStyle(Value: TIconStyle);
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
    property Icon: string read GetIcon write SetIcon;
    property Color: TAylisColor read GetColor write SetColor;
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
    property Style: TIconStyle read GetStyle write SetStyle;
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

  { Um ListView de um form. }
  TListView = class(TAylisComponent)
  private
    function GetItems: TListViewItemArray;
    procedure SetItems(Value: TListViewItemArray);
    function GetItemIndex: Int64;
    procedure SetItemIndex(Value: Int64);
    function GetItemHeight: Double;
    procedure SetItemHeight(Value: Double);
    function GetAccessory: Boolean;
    procedure SetAccessory(Value: Boolean);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
    function GetOnItemClick: string;
    procedure SetOnItemClick(Value: string);
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
    function GetStyle: TListViewStyle;
    procedure SetStyle(Value: TListViewStyle);
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
    property Items: TListViewItemArray read GetItems write SetItems;
    property ItemIndex: Int64 read GetItemIndex write SetItemIndex;
    property ItemHeight: Double read GetItemHeight write SetItemHeight;
    property Accessory: Boolean read GetAccessory write SetAccessory;
    property OnChange: string read GetOnChange write SetOnChange;
    property OnItemClick: string read GetOnItemClick write SetOnItemClick;
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
    property Style: TListViewStyle read GetStyle write SetStyle;
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

  { Um MainMenu de um form. }
  TMainMenu = class(TAylisComponent)
  private
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
    function GetStyle: TMainMenuStyle;
    procedure SetStyle(Value: TMainMenuStyle);
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
    procedure Close;
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
    property Style: TMainMenuStyle read GetStyle write SetStyle;
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

  { Um MenuItem de um form. }
  TMenuItem = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetShortcut: string;
    procedure SetShortcut(Value: string);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetChecked: Boolean;
    procedure SetChecked(Value: Boolean);
    function GetAutoCheck: Boolean;
    procedure SetAutoCheck(Value: Boolean);
    function GetIcon: string;
    procedure SetIcon(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetDanger: Boolean;
    procedure SetDanger(Value: Boolean);
    function GetHeader: Boolean;
    procedure SetHeader(Value: Boolean);
    function GetContent: TAylisComponent;
    procedure SetContent(Value: TAylisComponent);
    function GetOnClick: string;
    procedure SetOnClick(Value: string);
  public
    property Text: string read GetText write SetText;
    property Shortcut: string read GetShortcut write SetShortcut;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property Checked: Boolean read GetChecked write SetChecked;
    property AutoCheck: Boolean read GetAutoCheck write SetAutoCheck;
    property Icon: string read GetIcon write SetIcon;
    property Visible: Boolean read GetVisible write SetVisible;
    property Danger: Boolean read GetDanger write SetDanger;
    property Header: Boolean read GetHeader write SetHeader;
    property Content: TAylisComponent read GetContent write SetContent;
    property OnClick: string read GetOnClick write SetOnClick;
  end;

  { Um OpenDialog de um form. }
  TOpenDialog = class(TAylisComponent)
  private
    function GetTitle: string;
    procedure SetTitle(Value: string);
    function GetFilter: string;
    procedure SetFilter(Value: string);
    function GetInitialDir: string;
    procedure SetInitialDir(Value: string);
    function GetFileName: string;
    procedure SetFileName(Value: string);
    function GetMultiple: Boolean;
    procedure SetMultiple(Value: Boolean);
    function GetOnSelect: string;
    procedure SetOnSelect(Value: string);
    function GetOnCancel: string;
    procedure SetOnCancel(Value: string);
  public
    procedure Execute;
    property Title: string read GetTitle write SetTitle;
    property Filter: string read GetFilter write SetFilter;
    property InitialDir: string read GetInitialDir write SetInitialDir;
    property FileName: string read GetFileName write SetFileName;
    property Multiple: Boolean read GetMultiple write SetMultiple;
    property OnSelect: string read GetOnSelect write SetOnSelect;
    property OnCancel: string read GetOnCancel write SetOnCancel;
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

  { Um SeekBar de um form. }
  TSeekBar = class(TAylisComponent)
  private
    function GetPosition: Double;
    procedure SetPosition(Value: Double);
    function GetDuration: Double;
    procedure SetDuration(Value: Double);
    function GetBuffered: Double;
    procedure SetBuffered(Value: Double);
    function GetMarkers: TAylisDoubleArray;
    procedure SetMarkers(Value: TAylisDoubleArray);
    function GetShowTime: Boolean;
    procedure SetShowTime(Value: Boolean);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
    function GetOnSeek: string;
    procedure SetOnSeek(Value: string);
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
    function GetStyle: TSeekBarStyle;
    procedure SetStyle(Value: TSeekBarStyle);
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
    property Position: Double read GetPosition write SetPosition;
    property Duration: Double read GetDuration write SetDuration;
    property Buffered: Double read GetBuffered write SetBuffered;
    property Markers: TAylisDoubleArray read GetMarkers write SetMarkers;
    property ShowTime: Boolean read GetShowTime write SetShowTime;
    property OnChange: string read GetOnChange write SetOnChange;
    property OnSeek: string read GetOnSeek write SetOnSeek;
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
    property Style: TSeekBarStyle read GetStyle write SetStyle;
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

  { Um Timer de um form. }
  TTimer = class(TAylisComponent)
  private
    function GetInterval: Double;
    procedure SetInterval(Value: Double);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetPrecision: TTimerPrecision;
    procedure SetPrecision(Value: TTimerPrecision);
    function GetBackground: Boolean;
    procedure SetBackground(Value: Boolean);
    function GetOneShot: Boolean;
    procedure SetOneShot(Value: Boolean);
    function GetOnTimer: string;
    procedure SetOnTimer(Value: string);
    function GetDrift: Double;
    procedure SetDrift(Value: Double);
  public
    procedure Restart;
    property Interval: Double read GetInterval write SetInterval;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property Precision: TTimerPrecision read GetPrecision write SetPrecision;
    property Background: Boolean read GetBackground write SetBackground;
    property OneShot: Boolean read GetOneShot write SetOneShot;
    property OnTimer: string read GetOnTimer write SetOnTimer;
    property Drift: Double read GetDrift write SetDrift;
  end;

  { Um TrackBar de um form. }
  TTrackBar = class(TAylisComponent)
  private
    function GetValue: Double;
    procedure SetValue(Value: Double);
    function GetMin: Double;
    procedure SetMin(Value: Double);
    function GetMax: Double;
    procedure SetMax(Value: Double);
    function GetFrequency: Double;
    procedure SetFrequency(Value: Double);
    function GetVertical: Boolean;
    procedure SetVertical(Value: Boolean);
    function GetThumbSize: Double;
    procedure SetThumbSize(Value: Double);
    function GetTrackSize: Double;
    procedure SetTrackSize(Value: Double);
    function GetSolidThumb: Boolean;
    procedure SetSolidThumb(Value: Boolean);
    function GetLimit: Double;
    procedure SetLimit(Value: Double);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
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
    function GetStyle: TTrackBarStyle;
    procedure SetStyle(Value: TTrackBarStyle);
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
    property Value: Double read GetValue write SetValue;
    property Min: Double read GetMin write SetMin;
    property Max: Double read GetMax write SetMax;
    property Frequency: Double read GetFrequency write SetFrequency;
    property Vertical: Boolean read GetVertical write SetVertical;
    property ThumbSize: Double read GetThumbSize write SetThumbSize;
    property TrackSize: Double read GetTrackSize write SetTrackSize;
    property SolidThumb: Boolean read GetSolidThumb write SetSolidThumb;
    property Limit: Double read GetLimit write SetLimit;
    property OnChange: string read GetOnChange write SetOnChange;
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
    property Style: TTrackBarStyle read GetStyle write SetStyle;
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

  { Um VideoPlayer de um form. }
  TVideoPlayer = class(TAylisComponent)
  private
    function GetFile: string;
    procedure SetFile(Value: string);
    function GetAutoplay: Boolean;
    procedure SetAutoplay(Value: Boolean);
    function GetLooping: Boolean;
    procedure SetLooping(Value: Boolean);
    function GetVolume: Double;
    procedure SetVolume(Value: Double);
    function GetMuted: Boolean;
    procedure SetMuted(Value: Boolean);
    function GetFit: TImageFit;
    procedure SetFit(Value: TImageFit);
    function GetRadius: Double;
    procedure SetRadius(Value: Double);
    function GetRate: Double;
    procedure SetRate(Value: Double);
    function GetOutput: TOutputSpecArray;
    procedure SetOutput(Value: TOutputSpecArray);
    function GetShowVideo: Boolean;
    procedure SetShowVideo(Value: Boolean);
    function GetFramesAhead: Int64;
    procedure SetFramesAhead(Value: Int64);
    function GetOnReady: string;
    procedure SetOnReady(Value: string);
    function GetOnEnd: string;
    procedure SetOnEnd(Value: string);
    function GetOnError: string;
    procedure SetOnError(Value: string);
    function GetOnFrame: string;
    procedure SetOnFrame(Value: string);
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
    function GetStyle: TVideoPlayerStyle;
    procedure SetStyle(Value: TVideoPlayerStyle);
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
    procedure Play;
    procedure Pause;
    procedure Stop;
    procedure Seek(ASeconds: Double);
    function Position: Double;
    function Duration: Double;
    property &File: string read GetFile write SetFile;
    property Autoplay: Boolean read GetAutoplay write SetAutoplay;
    property Looping: Boolean read GetLooping write SetLooping;
    property Volume: Double read GetVolume write SetVolume;
    property Muted: Boolean read GetMuted write SetMuted;
    property Fit: TImageFit read GetFit write SetFit;
    property Radius: Double read GetRadius write SetRadius;
    property Rate: Double read GetRate write SetRate;
    property Output: TOutputSpecArray read GetOutput write SetOutput;
    property ShowVideo: Boolean read GetShowVideo write SetShowVideo;
    property FramesAhead: Int64 read GetFramesAhead write SetFramesAhead;
    property OnReady: string read GetOnReady write SetOnReady;
    property OnEnd: string read GetOnEnd write SetOnEnd;
    property OnError: string read GetOnError write SetOnError;
    property OnFrame: string read GetOnFrame write SetOnFrame;
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
    property Style: TVideoPlayerStyle read GetStyle write SetStyle;
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

  { O form frmPlayer: a classe base com os componentes. A do form (TFrmPlayer), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmPlayerBase = class(TForm)
  published
    mnuPrincipal: TMainMenu;
    mnuArquivo: TMenuItem;
    mnuAbrir: TMenuItem;
    mnuLimparFila: TMenuItem;
    mnuSep1: TMenuItem;
    mnuSair: TMenuItem;
    mnuReproduzir: TMenuItem;
    mnuReproduzirPausar: TMenuItem;
    mnuParar: TMenuItem;
    mnuAnterior: TMenuItem;
    mnuProxima: TMenuItem;
    mnuSep2: TMenuItem;
    mnuEmbaralhar: TMenuItem;
    mnuRepetir: TMenuItem;
    mnuExibir: TMenuItem;
    mnuAgora: TMenuItem;
    mnuFila: TMenuItem;
    pnlControles: TPanel;
    lytBarra: TLayout;
    lblTempo: TLabel;
    lblDuracao: TLabel;
    sbPosicao: TSeekBar;
    lytInfo: TLayout;
    icoCapa: TIcon;
    lytTextos: TLayout;
    lblTitulo: TLabel;
    lblDetalhe: TLabel;
    lytVolume: TLayout;
    btnMudo: TButton;
    tbVolume: TTrackBar;
    lytBotoes: TLayout;
    lytBotoesCentro: TLayout;
    btnEmbaralhar: TButton;
    btnAnterior: TButton;
    btnReproduzir: TButton;
    btnProxima: TButton;
    btnRepetir: TButton;
    lytNavegacao: TLayout;
    btnAgora: TButton;
    btnFila: TButton;
    divNavegacao: TDivider;
    btnAbrirArquivos: TButton;
    pnlAgora: TPanel;
    vpVideo: TVideoPlayer;
    pnlFila: TPanel;
    lblFila: TLabel;
    lytFilaBotoes: TLayout;
    btnAdicionar: TButton;
    btnLimparFila: TButton;
    lvFila: TListView;
    dlgAbrir: TOpenDialog;
    tmrPosicao: TTimer;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O Button deste form com o nome (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O Divider deste form com o nome (nil se não há). }
    function FindDivider(const Nome: string): TDivider;
    { O Divider que disparou o evento (nil se não é um Divider deste form). }
    function SenderAsDivider(Sender: TAylisComponent): TDivider; overload;
    function SenderAsDivider(Event: TJSONObject): TDivider; overload;
    { O Icon deste form com o nome (nil se não há). }
    function FindIcon(const Nome: string): TIcon;
    { O Icon que disparou o evento (nil se não é um Icon deste form). }
    function SenderAsIcon(Sender: TAylisComponent): TIcon; overload;
    function SenderAsIcon(Event: TJSONObject): TIcon; overload;
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
    { O ListView deste form com o nome (nil se não há). }
    function FindListView(const Nome: string): TListView;
    { O ListView que disparou o evento (nil se não é um ListView deste form). }
    function SenderAsListView(Sender: TAylisComponent): TListView; overload;
    function SenderAsListView(Event: TJSONObject): TListView; overload;
    { O MainMenu deste form com o nome (nil se não há). }
    function FindMainMenu(const Nome: string): TMainMenu;
    { O MainMenu que disparou o evento (nil se não é um MainMenu deste form). }
    function SenderAsMainMenu(Sender: TAylisComponent): TMainMenu; overload;
    function SenderAsMainMenu(Event: TJSONObject): TMainMenu; overload;
    { O MenuItem deste form com o nome (nil se não há). }
    function FindMenuItem(const Nome: string): TMenuItem;
    { O MenuItem que disparou o evento (nil se não é um MenuItem deste form). }
    function SenderAsMenuItem(Sender: TAylisComponent): TMenuItem; overload;
    function SenderAsMenuItem(Event: TJSONObject): TMenuItem; overload;
    { O OpenDialog deste form com o nome (nil se não há). }
    function FindOpenDialog(const Nome: string): TOpenDialog;
    { O OpenDialog que disparou o evento (nil se não é um OpenDialog deste form). }
    function SenderAsOpenDialog(Sender: TAylisComponent): TOpenDialog; overload;
    function SenderAsOpenDialog(Event: TJSONObject): TOpenDialog; overload;
    { O Panel deste form com o nome (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O SeekBar deste form com o nome (nil se não há). }
    function FindSeekBar(const Nome: string): TSeekBar;
    { O SeekBar que disparou o evento (nil se não é um SeekBar deste form). }
    function SenderAsSeekBar(Sender: TAylisComponent): TSeekBar; overload;
    function SenderAsSeekBar(Event: TJSONObject): TSeekBar; overload;
    { O Timer deste form com o nome (nil se não há). }
    function FindTimer(const Nome: string): TTimer;
    { O Timer que disparou o evento (nil se não é um Timer deste form). }
    function SenderAsTimer(Sender: TAylisComponent): TTimer; overload;
    function SenderAsTimer(Event: TJSONObject): TTimer; overload;
    { O TrackBar deste form com o nome (nil se não há). }
    function FindTrackBar(const Nome: string): TTrackBar;
    { O TrackBar que disparou o evento (nil se não é um TrackBar deste form). }
    function SenderAsTrackBar(Sender: TAylisComponent): TTrackBar; overload;
    function SenderAsTrackBar(Event: TJSONObject): TTrackBar; overload;
    { O VideoPlayer deste form com o nome (nil se não há). }
    function FindVideoPlayer(const Nome: string): TVideoPlayer;
    { O VideoPlayer que disparou o evento (nil se não é um VideoPlayer deste form). }
    function SenderAsVideoPlayer(Sender: TAylisComponent): TVideoPlayer; overload;
    function SenderAsVideoPlayer(Event: TJSONObject): TVideoPlayer; overload;
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
function BlendModeOf(Data: TJSONData): TBlendMode;
function BlendModeSetOf(Data: TJSONData): TBlendModeSet;
function BlendModeSetToJSON(const S: TBlendModeSet): TJSONData;
function BorderStyleOf(Data: TJSONData): TBorderStyle;
function BorderStyleSetOf(Data: TJSONData): TBorderStyleSet;
function BorderStyleSetToJSON(const S: TBorderStyleSet): TJSONData;
function ButtonStyleOf(Data: TJSONData): TButtonStyle;
function ButtonStyleSetOf(Data: TJSONData): TButtonStyleSet;
function ButtonStyleSetToJSON(const S: TButtonStyleSet): TJSONData;
function CloseActionOf(Data: TJSONData): TCloseAction;
function CloseActionSetOf(Data: TJSONData): TCloseActionSet;
function CloseActionSetToJSON(const S: TCloseActionSet): TJSONData;
function CursorShapeOf(Data: TJSONData): TCursorShape;
function CursorShapeSetOf(Data: TJSONData): TCursorShapeSet;
function CursorShapeSetToJSON(const S: TCursorShapeSet): TJSONData;
function DividerStyleOf(Data: TJSONData): TDividerStyle;
function DividerStyleSetOf(Data: TJSONData): TDividerStyleSet;
function DividerStyleSetToJSON(const S: TDividerStyleSet): TJSONData;
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
function IconPositionOf(Data: TJSONData): TIconPosition;
function IconPositionSetOf(Data: TJSONData): TIconPositionSet;
function IconPositionSetToJSON(const S: TIconPositionSet): TJSONData;
function IconStyleOf(Data: TJSONData): TIconStyle;
function IconStyleSetOf(Data: TJSONData): TIconStyleSet;
function IconStyleSetToJSON(const S: TIconStyleSet): TJSONData;
function ImageFitOf(Data: TJSONData): TImageFit;
function ImageFitSetOf(Data: TJSONData): TImageFitSet;
function ImageFitSetToJSON(const S: TImageFitSet): TJSONData;
function LabelStyleOf(Data: TJSONData): TLabelStyle;
function LabelStyleSetOf(Data: TJSONData): TLabelStyleSet;
function LabelStyleSetToJSON(const S: TLabelStyleSet): TJSONData;
function LayerCachingOf(Data: TJSONData): TLayerCaching;
function LayerCachingSetOf(Data: TJSONData): TLayerCachingSet;
function LayerCachingSetToJSON(const S: TLayerCachingSet): TJSONData;
function LayoutStyleOf(Data: TJSONData): TLayoutStyle;
function LayoutStyleSetOf(Data: TJSONData): TLayoutStyleSet;
function LayoutStyleSetToJSON(const S: TLayoutStyleSet): TJSONData;
function ListViewStyleOf(Data: TJSONData): TListViewStyle;
function ListViewStyleSetOf(Data: TJSONData): TListViewStyleSet;
function ListViewStyleSetToJSON(const S: TListViewStyleSet): TJSONData;
function MainMenuStyleOf(Data: TJSONData): TMainMenuStyle;
function MainMenuStyleSetOf(Data: TJSONData): TMainMenuStyleSet;
function MainMenuStyleSetToJSON(const S: TMainMenuStyleSet): TJSONData;
function ModalResultOf(Data: TJSONData): TModalResult;
function ModalResultSetOf(Data: TJSONData): TModalResultSet;
function ModalResultSetToJSON(const S: TModalResultSet): TJSONData;
function PanelStyleOf(Data: TJSONData): TPanelStyle;
function PanelStyleSetOf(Data: TJSONData): TPanelStyleSet;
function PanelStyleSetToJSON(const S: TPanelStyleSet): TJSONData;
function SeekBarStyleOf(Data: TJSONData): TSeekBarStyle;
function SeekBarStyleSetOf(Data: TJSONData): TSeekBarStyleSet;
function SeekBarStyleSetToJSON(const S: TSeekBarStyleSet): TJSONData;
function SelfAlignOf(Data: TJSONData): TSelfAlign;
function SelfAlignSetOf(Data: TJSONData): TSelfAlignSet;
function SelfAlignSetToJSON(const S: TSelfAlignSet): TJSONData;
function TextAlignOf(Data: TJSONData): TTextAlign;
function TextAlignSetOf(Data: TJSONData): TTextAlignSet;
function TextAlignSetToJSON(const S: TTextAlignSet): TJSONData;
function TextCaseOf(Data: TJSONData): TTextCase;
function TextCaseSetOf(Data: TJSONData): TTextCaseSet;
function TextCaseSetToJSON(const S: TTextCaseSet): TJSONData;
function ThemeNameOf(Data: TJSONData): TThemeName;
function ThemeNameSetOf(Data: TJSONData): TThemeNameSet;
function ThemeNameSetToJSON(const S: TThemeNameSet): TJSONData;
function TimerPrecisionOf(Data: TJSONData): TTimerPrecision;
function TimerPrecisionSetOf(Data: TJSONData): TTimerPrecisionSet;
function TimerPrecisionSetToJSON(const S: TTimerPrecisionSet): TJSONData;
function TrackBarStyleOf(Data: TJSONData): TTrackBarStyle;
function TrackBarStyleSetOf(Data: TJSONData): TTrackBarStyleSet;
function TrackBarStyleSetToJSON(const S: TTrackBarStyleSet): TJSONData;
function TransitionOf(Data: TJSONData): TTransition;
function TransitionSetOf(Data: TJSONData): TTransitionSet;
function TransitionSetToJSON(const S: TTransitionSet): TJSONData;
function TrimmingOf(Data: TJSONData): TTrimming;
function TrimmingSetOf(Data: TJSONData): TTrimmingSet;
function TrimmingSetToJSON(const S: TTrimmingSet): TJSONData;
function VideoPlayerStyleOf(Data: TJSONData): TVideoPlayerStyle;
function VideoPlayerStyleSetOf(Data: TJSONData): TVideoPlayerStyleSet;
function VideoPlayerStyleSetToJSON(const S: TVideoPlayerStyleSet): TJSONData;
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
function GestureEventFromJSON(Data: TJSONData): TGestureEvent;
function GestureEventListFromJSON(Data: TJSONData): TGestureEventArray;
function GestureEventFromText(Data: TJSONData): TGestureEvent;
function GestureEventListFromText(Data: TJSONData): TGestureEventArray;
function GestureEventToJSON(const R: TGestureEvent): TJSONObject;
function GestureEventListToJSON(const A: array of TGestureEvent): TJSONArray;
function GestureEventText(const R: TGestureEvent): TJSONData;
function GestureEventListText(const A: TGestureEventArray): TJSONData;
function KeyEventFromJSON(Data: TJSONData): TKeyEvent;
function KeyEventListFromJSON(Data: TJSONData): TKeyEventArray;
function KeyEventFromText(Data: TJSONData): TKeyEvent;
function KeyEventListFromText(Data: TJSONData): TKeyEventArray;
function KeyEventToJSON(const R: TKeyEvent): TJSONObject;
function KeyEventListToJSON(const A: array of TKeyEvent): TJSONArray;
function KeyEventText(const R: TKeyEvent): TJSONData;
function KeyEventListText(const A: TKeyEventArray): TJSONData;
function ListViewItemFromJSON(Data: TJSONData): TListViewItem;
function ListViewItemListFromJSON(Data: TJSONData): TListViewItemArray;
function ListViewItemFromText(Data: TJSONData): TListViewItem;
function ListViewItemListFromText(Data: TJSONData): TListViewItemArray;
function ListViewItemToJSON(const R: TListViewItem): TJSONObject;
function ListViewItemListToJSON(const A: array of TListViewItem): TJSONArray;
function ListViewItemText(const R: TListViewItem): TJSONData;
function ListViewItemListText(const A: TListViewItemArray): TJSONData;
function OutputSpecFromJSON(Data: TJSONData): TOutputSpec;
function OutputSpecListFromJSON(Data: TJSONData): TOutputSpecArray;
function OutputSpecFromText(Data: TJSONData): TOutputSpec;
function OutputSpecListFromText(Data: TJSONData): TOutputSpecArray;
function OutputSpecToJSON(const R: TOutputSpec): TJSONObject;
function OutputSpecListToJSON(const A: array of TOutputSpec): TJSONArray;
function OutputSpecText(const R: TOutputSpec): TJSONData;
function OutputSpecListText(const A: TOutputSpecArray): TJSONData;
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
  ButtonStyleNames: array[TButtonStyle] of string = ('', 'accent', 'danger', 'elevated', 'fab', 'fab_extended', 'flat', 'ghost', 'glass', 'glass_prominent', 'glossy', 'icon', 'icon_filled', 'icon_outlined', 'icon_tonal', 'info', 'link', 'negative', 'outline', 'pill', 'positive', 'push', 'reveal', 'round', 'rounded', 'secondary', 'success', 'text', 'tonal', 'unelevated', 'warning');

function ButtonStyleOf(Data: TJSONData): TButtonStyle;
var
  V: TButtonStyle;
begin
  Result := Low(TButtonStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TButtonStyle) to High(TButtonStyle) do
      if ButtonStyleNames[V] = Data.AsString then
        Exit(V);
end;

function ButtonStyleSetOf(Data: TJSONData): TButtonStyleSet;
var
  I: Integer;
  V: TButtonStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TButtonStyle) to High(TButtonStyle) do
      if ButtonStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ButtonStyleSetToJSON(const S: TButtonStyleSet): TJSONData;
var
  V: TButtonStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TButtonStyle) to High(TButtonStyle) do
    if V in S then
      TJSONArray(Result).Add(ButtonStyleNames[V]);
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
  DividerStyleNames: array[TDividerStyle] of string = ('');

function DividerStyleOf(Data: TJSONData): TDividerStyle;
var
  V: TDividerStyle;
begin
  Result := Low(TDividerStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TDividerStyle) to High(TDividerStyle) do
      if DividerStyleNames[V] = Data.AsString then
        Exit(V);
end;

function DividerStyleSetOf(Data: TJSONData): TDividerStyleSet;
var
  I: Integer;
  V: TDividerStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TDividerStyle) to High(TDividerStyle) do
      if DividerStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function DividerStyleSetToJSON(const S: TDividerStyleSet): TJSONData;
var
  V: TDividerStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TDividerStyle) to High(TDividerStyle) do
    if V in S then
      TJSONArray(Result).Add(DividerStyleNames[V]);
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
  IconPositionNames: array[TIconPosition] of string = ('left', 'right', 'top');

function IconPositionOf(Data: TJSONData): TIconPosition;
var
  V: TIconPosition;
begin
  Result := Low(TIconPosition);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TIconPosition) to High(TIconPosition) do
      if IconPositionNames[V] = Data.AsString then
        Exit(V);
end;

function IconPositionSetOf(Data: TJSONData): TIconPositionSet;
var
  I: Integer;
  V: TIconPosition;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TIconPosition) to High(TIconPosition) do
      if IconPositionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function IconPositionSetToJSON(const S: TIconPositionSet): TJSONData;
var
  V: TIconPosition;
begin
  Result := TJSONArray.Create;
  for V := Low(TIconPosition) to High(TIconPosition) do
    if V in S then
      TJSONArray(Result).Add(IconPositionNames[V]);
end;

const
  IconStyleNames: array[TIconStyle] of string = ('', 'muted');

function IconStyleOf(Data: TJSONData): TIconStyle;
var
  V: TIconStyle;
begin
  Result := Low(TIconStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TIconStyle) to High(TIconStyle) do
      if IconStyleNames[V] = Data.AsString then
        Exit(V);
end;

function IconStyleSetOf(Data: TJSONData): TIconStyleSet;
var
  I: Integer;
  V: TIconStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TIconStyle) to High(TIconStyle) do
      if IconStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function IconStyleSetToJSON(const S: TIconStyleSet): TJSONData;
var
  V: TIconStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TIconStyle) to High(TIconStyle) do
    if V in S then
      TJSONArray(Result).Add(IconStyleNames[V]);
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
  ListViewStyleNames: array[TListViewStyle] of string = ('', 'flat');

function ListViewStyleOf(Data: TJSONData): TListViewStyle;
var
  V: TListViewStyle;
begin
  Result := Low(TListViewStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TListViewStyle) to High(TListViewStyle) do
      if ListViewStyleNames[V] = Data.AsString then
        Exit(V);
end;

function ListViewStyleSetOf(Data: TJSONData): TListViewStyleSet;
var
  I: Integer;
  V: TListViewStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TListViewStyle) to High(TListViewStyle) do
      if ListViewStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ListViewStyleSetToJSON(const S: TListViewStyleSet): TJSONData;
var
  V: TListViewStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TListViewStyle) to High(TListViewStyle) do
    if V in S then
      TJSONArray(Result).Add(ListViewStyleNames[V]);
end;

const
  MainMenuStyleNames: array[TMainMenuStyle] of string = ('');

function MainMenuStyleOf(Data: TJSONData): TMainMenuStyle;
var
  V: TMainMenuStyle;
begin
  Result := Low(TMainMenuStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TMainMenuStyle) to High(TMainMenuStyle) do
      if MainMenuStyleNames[V] = Data.AsString then
        Exit(V);
end;

function MainMenuStyleSetOf(Data: TJSONData): TMainMenuStyleSet;
var
  I: Integer;
  V: TMainMenuStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TMainMenuStyle) to High(TMainMenuStyle) do
      if MainMenuStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function MainMenuStyleSetToJSON(const S: TMainMenuStyleSet): TJSONData;
var
  V: TMainMenuStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TMainMenuStyle) to High(TMainMenuStyle) do
    if V in S then
      TJSONArray(Result).Add(MainMenuStyleNames[V]);
end;

const
  ModalResultNames: array[TModalResult] of string = ('none', 'ok', 'cancel', 'abort', 'retry', 'ignore', 'yes', 'no', 'close');

function ModalResultOf(Data: TJSONData): TModalResult;
var
  V: TModalResult;
begin
  Result := Low(TModalResult);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TModalResult) to High(TModalResult) do
      if ModalResultNames[V] = Data.AsString then
        Exit(V);
end;

function ModalResultSetOf(Data: TJSONData): TModalResultSet;
var
  I: Integer;
  V: TModalResult;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TModalResult) to High(TModalResult) do
      if ModalResultNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ModalResultSetToJSON(const S: TModalResultSet): TJSONData;
var
  V: TModalResult;
begin
  Result := TJSONArray.Create;
  for V := Low(TModalResult) to High(TModalResult) do
    if V in S then
      TJSONArray(Result).Add(ModalResultNames[V]);
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
  SeekBarStyleNames: array[TSeekBarStyle] of string = ('');

function SeekBarStyleOf(Data: TJSONData): TSeekBarStyle;
var
  V: TSeekBarStyle;
begin
  Result := Low(TSeekBarStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TSeekBarStyle) to High(TSeekBarStyle) do
      if SeekBarStyleNames[V] = Data.AsString then
        Exit(V);
end;

function SeekBarStyleSetOf(Data: TJSONData): TSeekBarStyleSet;
var
  I: Integer;
  V: TSeekBarStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TSeekBarStyle) to High(TSeekBarStyle) do
      if SeekBarStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function SeekBarStyleSetToJSON(const S: TSeekBarStyleSet): TJSONData;
var
  V: TSeekBarStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TSeekBarStyle) to High(TSeekBarStyle) do
    if V in S then
      TJSONArray(Result).Add(SeekBarStyleNames[V]);
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
  TimerPrecisionNames: array[TTimerPrecision] of string = ('normal', 'high', 'exact');

function TimerPrecisionOf(Data: TJSONData): TTimerPrecision;
var
  V: TTimerPrecision;
begin
  Result := Low(TTimerPrecision);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTimerPrecision) to High(TTimerPrecision) do
      if TimerPrecisionNames[V] = Data.AsString then
        Exit(V);
end;

function TimerPrecisionSetOf(Data: TJSONData): TTimerPrecisionSet;
var
  I: Integer;
  V: TTimerPrecision;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTimerPrecision) to High(TTimerPrecision) do
      if TimerPrecisionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TimerPrecisionSetToJSON(const S: TTimerPrecisionSet): TJSONData;
var
  V: TTimerPrecision;
begin
  Result := TJSONArray.Create;
  for V := Low(TTimerPrecision) to High(TTimerPrecision) do
    if V in S then
      TJSONArray(Result).Add(TimerPrecisionNames[V]);
end;

const
  TrackBarStyleNames: array[TTrackBarStyle] of string = ('');

function TrackBarStyleOf(Data: TJSONData): TTrackBarStyle;
var
  V: TTrackBarStyle;
begin
  Result := Low(TTrackBarStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTrackBarStyle) to High(TTrackBarStyle) do
      if TrackBarStyleNames[V] = Data.AsString then
        Exit(V);
end;

function TrackBarStyleSetOf(Data: TJSONData): TTrackBarStyleSet;
var
  I: Integer;
  V: TTrackBarStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTrackBarStyle) to High(TTrackBarStyle) do
      if TrackBarStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TrackBarStyleSetToJSON(const S: TTrackBarStyleSet): TJSONData;
var
  V: TTrackBarStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TTrackBarStyle) to High(TTrackBarStyle) do
    if V in S then
      TJSONArray(Result).Add(TrackBarStyleNames[V]);
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
  VideoPlayerStyleNames: array[TVideoPlayerStyle] of string = ('');

function VideoPlayerStyleOf(Data: TJSONData): TVideoPlayerStyle;
var
  V: TVideoPlayerStyle;
begin
  Result := Low(TVideoPlayerStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TVideoPlayerStyle) to High(TVideoPlayerStyle) do
      if VideoPlayerStyleNames[V] = Data.AsString then
        Exit(V);
end;

function VideoPlayerStyleSetOf(Data: TJSONData): TVideoPlayerStyleSet;
var
  I: Integer;
  V: TVideoPlayerStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TVideoPlayerStyle) to High(TVideoPlayerStyle) do
      if VideoPlayerStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function VideoPlayerStyleSetToJSON(const S: TVideoPlayerStyleSet): TJSONData;
var
  V: TVideoPlayerStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TVideoPlayerStyle) to High(TVideoPlayerStyle) do
    if V in S then
      TJSONArray(Result).Add(VideoPlayerStyleNames[V]);
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

function ListViewItemFromJSON(Data: TJSONData): TListViewItem;
var
  O: TJSONObject;
begin
  Result := Default(TListViewItem);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Title := O.Get('title', '');
  Result.Detail := O.Get('detail', '');
end;

function ListViewItemToJSON(const R: TListViewItem): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('title', R.Title);
  Result.Add('detail', R.Detail);
end;

function ListViewItemListFromJSON(Data: TJSONData): TListViewItemArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := ListViewItemFromJSON(Data.Items[I]);
end;

function ListViewItemListToJSON(const A: array of TListViewItem): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(ListViewItemToJSON(A[I]));
end;

function ListViewItemText(const R: TListViewItem): TJSONData;
var
  J: TJSONObject;
begin
  J := ListViewItemToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ListViewItemListText(const A: TListViewItemArray): TJSONData;
var
  J: TJSONArray;
begin
  J := ListViewItemListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ListViewItemFromText(Data: TJSONData): TListViewItem;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ListViewItemFromJSON(J);
  finally
    J.Free;
  end;
end;

function ListViewItemListFromText(Data: TJSONData): TListViewItemArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ListViewItemListFromJSON(J);
  finally
    J.Free;
  end;
end;

function OutputSpecFromJSON(Data: TJSONData): TOutputSpec;
var
  O: TJSONObject;
begin
  Result := Default(TOutputSpec);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Name := O.Get('name', '');
  Result.Volume := O.Get('volume', Double(0));
  Result.DelayMs := O.Get('delay_ms', Double(0));
end;

function OutputSpecToJSON(const R: TOutputSpec): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('name', R.Name);
  Result.Add('volume', R.Volume);
  Result.Add('delay_ms', R.DelayMs);
end;

function OutputSpecListFromJSON(Data: TJSONData): TOutputSpecArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := OutputSpecFromJSON(Data.Items[I]);
end;

function OutputSpecListToJSON(const A: array of TOutputSpec): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(OutputSpecToJSON(A[I]));
end;

function OutputSpecText(const R: TOutputSpec): TJSONData;
var
  J: TJSONObject;
begin
  J := OutputSpecToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function OutputSpecListText(const A: TOutputSpecArray): TJSONData;
var
  J: TJSONArray;
begin
  J := OutputSpecListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function OutputSpecFromText(Data: TJSONData): TOutputSpec;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := OutputSpecFromJSON(J);
  finally
    J.Free;
  end;
end;

function OutputSpecListFromText(Data: TJSONData): TOutputSpecArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := OutputSpecListFromJSON(J);
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

function TButton.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TButton.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TButton.GetIcon: string;
begin
  Result := AsText(GetProp('icon'));
end;

procedure TButton.SetIcon(Value: string);
begin
  SetProp('icon', TJSONString.Create(Value));
end;

function TButton.GetCheckedIcon: string;
begin
  Result := AsText(GetProp('checked_icon'));
end;

procedure TButton.SetCheckedIcon(Value: string);
begin
  SetProp('checked_icon', TJSONString.Create(Value));
end;

function TButton.GetIconPosition: TIconPosition;
begin
  Result := IconPositionOf(GetProp('icon_position'));
end;

procedure TButton.SetIconPosition(Value: TIconPosition);
begin
  SetProp('icon_position', TJSONString.Create(IconPositionNames[Value]));
end;

function TButton.GetIconSize: Double;
begin
  Result := GetProp('icon_size').AsFloat;
end;

procedure TButton.SetIconSize(Value: Double);
begin
  SetProp('icon_size', TJSONFloatNumber.Create(Value));
end;

function TButton.GetCheckable: Boolean;
begin
  Result := GetProp('checkable').AsBoolean;
end;

procedure TButton.SetCheckable(Value: Boolean);
begin
  SetProp('checkable', TJSONBoolean.Create(Value));
end;

function TButton.GetChecked: Boolean;
begin
  Result := GetProp('checked').AsBoolean;
end;

procedure TButton.SetChecked(Value: Boolean);
begin
  SetProp('checked', TJSONBoolean.Create(Value));
end;

function TButton.GetModalResult: TModalResult;
begin
  Result := ModalResultOf(GetProp('modal_result'));
end;

procedure TButton.SetModalResult(Value: TModalResult);
begin
  SetProp('modal_result', TJSONString.Create(ModalResultNames[Value]));
end;

function TButton.GetRepeatEnter: Boolean;
begin
  Result := GetProp('repeat_enter').AsBoolean;
end;

procedure TButton.SetRepeatEnter(Value: Boolean);
begin
  SetProp('repeat_enter', TJSONBoolean.Create(Value));
end;

function TButton.GetDefaultButton: Boolean;
begin
  Result := GetProp('default_button').AsBoolean;
end;

procedure TButton.SetDefaultButton(Value: Boolean);
begin
  SetProp('default_button', TJSONBoolean.Create(Value));
end;

function TButton.GetCancelButton: Boolean;
begin
  Result := GetProp('cancel_button').AsBoolean;
end;

procedure TButton.SetCancelButton(Value: Boolean);
begin
  SetProp('cancel_button', TJSONBoolean.Create(Value));
end;

function TButton.GetHAlign: TTextAlign;
begin
  Result := TextAlignOf(GetProp('h_align'));
end;

procedure TButton.SetHAlign(Value: TTextAlign);
begin
  SetProp('h_align', TJSONString.Create(TextAlignNames[Value]));
end;

function TButton.GetGroupIndex: Int64;
begin
  Result := GetProp('group_index').AsInt64;
end;

procedure TButton.SetGroupIndex(Value: Int64);
begin
  SetProp('group_index', TJSONInt64Number.Create(Value));
end;

function TButton.GetRadius: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('radius'));
end;

procedure TButton.SetRadius(Value: TAylisBounds);
begin
  SetProp('radius', AylisBoundsToJSON(Value));
end;

function TButton.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TButton.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TButton.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TButton.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TButton.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TButton.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TButton.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TButton.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TButton.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TButton.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TButton.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TButton.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TButton.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TButton.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TButton.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TButton.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TButton.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TButton.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TButton.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TButton.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TButton.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TButton.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TButton.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TButton.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TButton.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TButton.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TButton.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TButton.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TButton.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TButton.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TButton.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TButton.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TButton.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TButton.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TButton.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TButton.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TButton.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TButton.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TButton.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TButton.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TButton.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TButton.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TButton.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TButton.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TButton.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TButton.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TButton.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TButton.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TButton.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TButton.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TButton.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TButton.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TButton.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TButton.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TButton.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TButton.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TButton.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TButton.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TButton.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TButton.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TButton.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TButton.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TButton.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TButton.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TButton.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TButton.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TButton.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TButton.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TButton.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TButton.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TButton.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TButton.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TButton.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TButton.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TButton.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TButton.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TButton.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TButton.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TButton.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TButton.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TButton.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TButton.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TButton.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TButton.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TButton.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TButton.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TButton.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TButton.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TButton.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TButton.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TButton.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TButton.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TButton.GetStyle: TButtonStyle;
begin
  Result := ButtonStyleOf(GetProp('style'));
end;

procedure TButton.SetStyle(Value: TButtonStyle);
begin
  SetProp('style', TJSONString.Create(ButtonStyleNames[Value]));
end;

function TButton.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TButton.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TButton.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TButton.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TButton.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TButton.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TButton.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TButton.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TButton.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TButton.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TButton.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TButton.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TButton.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TButton.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TButton.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TButton.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TButton.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TButton.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TButton.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TButton.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TButton.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TButton.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TButton.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TButton.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TButton.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TButton.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TButton.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TButton.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TButton.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TButton.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TButton.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TButton.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TButton.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TButton.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TButton.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TButton.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TButton.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TButton.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TButton.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TButton.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TButton.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TButton.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TButton.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TButton.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TButton.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TButton.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TButton.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TButton.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TButton.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TButton.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TButton.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TButton.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TButton.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TButton.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TButton.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TButton.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TButton.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TButton.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TButton.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TButton.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TButton.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TButton.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TButton.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TButton.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TButton.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TButton.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TButton.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TButton.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TButton.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TButton.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TButton.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TButton.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TButton.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TButton.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TButton.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TButton.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TButton.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TDivider.GetVertical: Boolean;
begin
  Result := GetProp('vertical').AsBoolean;
end;

procedure TDivider.SetVertical(Value: Boolean);
begin
  SetProp('vertical', TJSONBoolean.Create(Value));
end;

function TDivider.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TDivider.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TDivider.GetThickness: Double;
begin
  Result := GetProp('thickness').AsFloat;
end;

procedure TDivider.SetThickness(Value: Double);
begin
  SetProp('thickness', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TDivider.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TDivider.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TDivider.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TDivider.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TDivider.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TDivider.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TDivider.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TDivider.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TDivider.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TDivider.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TDivider.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TDivider.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TDivider.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TDivider.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TDivider.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TDivider.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TDivider.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TDivider.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TDivider.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TDivider.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TDivider.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TDivider.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TDivider.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TDivider.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDivider.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TDivider.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TDivider.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TDivider.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TDivider.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TDivider.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TDivider.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TDivider.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TDivider.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TDivider.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TDivider.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TDivider.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TDivider.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TDivider.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TDivider.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TDivider.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TDivider.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TDivider.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TDivider.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TDivider.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TDivider.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TDivider.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDivider.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TDivider.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TDivider.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TDivider.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TDivider.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TDivider.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TDivider.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TDivider.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TDivider.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TDivider.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TDivider.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TDivider.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TDivider.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TDivider.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TDivider.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TDivider.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TDivider.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TDivider.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TDivider.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TDivider.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TDivider.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TDivider.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TDivider.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TDivider.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TDivider.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TDivider.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TDivider.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TDivider.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TDivider.GetStyle: TDividerStyle;
begin
  Result := DividerStyleOf(GetProp('style'));
end;

procedure TDivider.SetStyle(Value: TDividerStyle);
begin
  SetProp('style', TJSONString.Create(DividerStyleNames[Value]));
end;

function TDivider.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TDivider.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TDivider.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TDivider.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TDivider.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TDivider.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TDivider.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TDivider.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TDivider.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TDivider.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TDivider.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TDivider.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TDivider.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TDivider.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TDivider.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TDivider.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TDivider.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TDivider.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TDivider.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TDivider.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TDivider.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TDivider.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TDivider.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TDivider.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TDivider.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TDivider.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TDivider.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TDivider.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TDivider.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TDivider.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TDivider.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TDivider.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TDivider.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TDivider.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TDivider.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TDivider.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TDivider.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TDivider.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TDivider.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TDivider.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TDivider.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TDivider.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TDivider.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TDivider.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TDivider.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TDivider.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TDivider.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TDivider.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TDivider.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TDivider.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TDivider.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TDivider.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TDivider.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TDivider.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TDivider.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TDivider.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TDivider.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TDivider.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TDivider.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TDivider.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TDivider.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TDivider.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TDivider.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TDivider.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TDivider.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TDivider.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TDivider.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TDivider.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TDivider.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TDivider.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TDivider.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TDivider.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TDivider.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TDivider.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TDivider.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TDivider.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
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

function TIcon.GetIcon: string;
begin
  Result := AsText(GetProp('icon'));
end;

procedure TIcon.SetIcon(Value: string);
begin
  SetProp('icon', TJSONString.Create(Value));
end;

function TIcon.GetColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('color'));
end;

procedure TIcon.SetColor(Value: TAylisColor);
begin
  SetProp('color', AylisColorToJSON(Value));
end;

function TIcon.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TIcon.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TIcon.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TIcon.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TIcon.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TIcon.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TIcon.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TIcon.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TIcon.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TIcon.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TIcon.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TIcon.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TIcon.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TIcon.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TIcon.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TIcon.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TIcon.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TIcon.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TIcon.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TIcon.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TIcon.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TIcon.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TIcon.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TIcon.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TIcon.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TIcon.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TIcon.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TIcon.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TIcon.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TIcon.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TIcon.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TIcon.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TIcon.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TIcon.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TIcon.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TIcon.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TIcon.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TIcon.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TIcon.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TIcon.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TIcon.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TIcon.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TIcon.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TIcon.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TIcon.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TIcon.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TIcon.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TIcon.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TIcon.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TIcon.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TIcon.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TIcon.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TIcon.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TIcon.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TIcon.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TIcon.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TIcon.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TIcon.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TIcon.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TIcon.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TIcon.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TIcon.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TIcon.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TIcon.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TIcon.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TIcon.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TIcon.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TIcon.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TIcon.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TIcon.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TIcon.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TIcon.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TIcon.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TIcon.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TIcon.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TIcon.GetStyle: TIconStyle;
begin
  Result := IconStyleOf(GetProp('style'));
end;

procedure TIcon.SetStyle(Value: TIconStyle);
begin
  SetProp('style', TJSONString.Create(IconStyleNames[Value]));
end;

function TIcon.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TIcon.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TIcon.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TIcon.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TIcon.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TIcon.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TIcon.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TIcon.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TIcon.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TIcon.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TIcon.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TIcon.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TIcon.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TIcon.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TIcon.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TIcon.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TIcon.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TIcon.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TIcon.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TIcon.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TIcon.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TIcon.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TIcon.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TIcon.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TIcon.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TIcon.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TIcon.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TIcon.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TIcon.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TIcon.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TIcon.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TIcon.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TIcon.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TIcon.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TIcon.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TIcon.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TIcon.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TIcon.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TIcon.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TIcon.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TIcon.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TIcon.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TIcon.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TIcon.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TIcon.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TIcon.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TIcon.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TIcon.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TIcon.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TIcon.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TIcon.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TIcon.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TIcon.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TIcon.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TIcon.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TIcon.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TIcon.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TIcon.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TIcon.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TIcon.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TIcon.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TIcon.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TIcon.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TIcon.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TIcon.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TIcon.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TIcon.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TIcon.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TIcon.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TIcon.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TIcon.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TIcon.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TIcon.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TIcon.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TIcon.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TIcon.SetOnDragEnd(Value: string);
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

function TListView.GetItems: TListViewItemArray;
begin
  Result := ListViewItemListFromText(GetProp('items'));
end;

procedure TListView.SetItems(Value: TListViewItemArray);
begin
  SetProp('items', ListViewItemListText(Value));
end;

function TListView.GetItemIndex: Int64;
begin
  Result := GetProp('item_index').AsInt64;
end;

procedure TListView.SetItemIndex(Value: Int64);
begin
  SetProp('item_index', TJSONInt64Number.Create(Value));
end;

function TListView.GetItemHeight: Double;
begin
  Result := GetProp('item_height').AsFloat;
end;

procedure TListView.SetItemHeight(Value: Double);
begin
  SetProp('item_height', TJSONFloatNumber.Create(Value));
end;

function TListView.GetAccessory: Boolean;
begin
  Result := GetProp('accessory').AsBoolean;
end;

procedure TListView.SetAccessory(Value: Boolean);
begin
  SetProp('accessory', TJSONBoolean.Create(Value));
end;

function TListView.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TListView.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TListView.GetOnItemClick: string;
begin
  Result := AsText(GetProp('on_item_click'));
end;

procedure TListView.SetOnItemClick(Value: string);
begin
  SetProp('on_item_click', TJSONString.Create(Value));
end;

function TListView.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TListView.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TListView.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TListView.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TListView.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TListView.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TListView.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TListView.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TListView.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TListView.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TListView.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TListView.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TListView.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TListView.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TListView.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TListView.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TListView.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TListView.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TListView.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TListView.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TListView.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TListView.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TListView.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TListView.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TListView.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TListView.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TListView.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TListView.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TListView.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TListView.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TListView.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TListView.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TListView.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TListView.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TListView.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TListView.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TListView.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TListView.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TListView.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TListView.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TListView.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TListView.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TListView.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TListView.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TListView.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TListView.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TListView.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TListView.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TListView.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TListView.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TListView.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TListView.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TListView.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TListView.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TListView.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TListView.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TListView.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TListView.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TListView.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TListView.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TListView.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TListView.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TListView.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TListView.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TListView.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TListView.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TListView.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TListView.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TListView.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TListView.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TListView.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TListView.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TListView.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TListView.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TListView.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TListView.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TListView.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TListView.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TListView.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TListView.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TListView.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TListView.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TListView.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TListView.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TListView.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TListView.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TListView.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TListView.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TListView.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TListView.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TListView.GetStyle: TListViewStyle;
begin
  Result := ListViewStyleOf(GetProp('style'));
end;

procedure TListView.SetStyle(Value: TListViewStyle);
begin
  SetProp('style', TJSONString.Create(ListViewStyleNames[Value]));
end;

function TListView.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TListView.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TListView.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TListView.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TListView.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TListView.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TListView.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TListView.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TListView.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TListView.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TListView.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TListView.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TListView.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TListView.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TListView.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TListView.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TListView.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TListView.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TListView.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TListView.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TListView.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TListView.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TListView.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TListView.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TListView.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TListView.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TListView.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TListView.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TListView.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TListView.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TListView.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TListView.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TListView.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TListView.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TListView.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TListView.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TListView.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TListView.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TListView.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TListView.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TListView.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TListView.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TListView.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TListView.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TListView.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TListView.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TListView.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TListView.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TListView.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TListView.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TListView.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TListView.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TListView.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TListView.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TListView.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TListView.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TListView.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TListView.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TListView.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TListView.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TListView.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TListView.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TListView.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TListView.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TListView.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TListView.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TListView.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TListView.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TListView.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TListView.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TListView.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TListView.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TListView.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TListView.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TListView.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TListView.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TListView.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TMainMenu.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TMainMenu.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TMainMenu.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TMainMenu.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TMainMenu.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TMainMenu.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TMainMenu.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TMainMenu.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TMainMenu.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TMainMenu.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TMainMenu.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TMainMenu.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TMainMenu.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TMainMenu.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TMainMenu.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TMainMenu.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TMainMenu.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TMainMenu.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TMainMenu.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TMainMenu.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TMainMenu.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TMainMenu.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TMainMenu.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TMainMenu.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TMainMenu.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TMainMenu.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TMainMenu.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TMainMenu.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TMainMenu.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TMainMenu.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TMainMenu.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TMainMenu.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TMainMenu.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TMainMenu.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TMainMenu.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TMainMenu.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TMainMenu.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TMainMenu.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TMainMenu.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TMainMenu.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TMainMenu.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TMainMenu.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TMainMenu.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TMainMenu.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TMainMenu.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TMainMenu.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TMainMenu.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TMainMenu.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TMainMenu.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TMainMenu.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TMainMenu.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TMainMenu.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TMainMenu.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TMainMenu.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TMainMenu.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TMainMenu.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TMainMenu.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TMainMenu.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TMainMenu.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TMainMenu.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TMainMenu.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TMainMenu.GetStyle: TMainMenuStyle;
begin
  Result := MainMenuStyleOf(GetProp('style'));
end;

procedure TMainMenu.SetStyle(Value: TMainMenuStyle);
begin
  SetProp('style', TJSONString.Create(MainMenuStyleNames[Value]));
end;

function TMainMenu.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TMainMenu.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TMainMenu.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TMainMenu.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TMainMenu.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TMainMenu.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TMainMenu.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TMainMenu.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TMainMenu.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TMainMenu.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TMainMenu.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TMainMenu.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TMainMenu.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TMainMenu.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TMainMenu.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TMainMenu.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TMainMenu.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TMainMenu.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TMainMenu.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TMainMenu.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TMainMenu.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TMainMenu.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TMainMenu.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TMainMenu.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TMainMenu.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TMainMenu.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TMainMenu.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TMainMenu.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TMainMenu.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TMainMenu.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TMainMenu.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TMainMenu.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TMainMenu.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TMainMenu.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TMainMenu.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TMainMenu.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TMainMenu.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TMainMenu.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TMainMenu.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TMainMenu.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TMainMenu.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TMainMenu.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TMainMenu.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TMainMenu.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TMainMenu.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TMainMenu.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TMainMenu.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TMainMenu.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TMainMenu.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TMainMenu.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TMainMenu.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TMainMenu.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TMainMenu.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TMainMenu.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TMainMenu.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TMainMenu.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TMainMenu.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TMainMenu.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TMainMenu.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TMainMenu.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TMainMenu.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TMainMenu.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TMainMenu.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TMainMenu.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TMainMenu.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TMainMenu.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TMainMenu.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TMainMenu.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TMainMenu.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TMainMenu.Close;
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'close', 'args', TJSONArray.Create]));
end;

function TMenuItem.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TMenuItem.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TMenuItem.GetShortcut: string;
begin
  Result := AsText(GetProp('shortcut'));
end;

procedure TMenuItem.SetShortcut(Value: string);
begin
  SetProp('shortcut', TJSONString.Create(Value));
end;

function TMenuItem.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TMenuItem.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetChecked: Boolean;
begin
  Result := GetProp('checked').AsBoolean;
end;

procedure TMenuItem.SetChecked(Value: Boolean);
begin
  SetProp('checked', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetAutoCheck: Boolean;
begin
  Result := GetProp('auto_check').AsBoolean;
end;

procedure TMenuItem.SetAutoCheck(Value: Boolean);
begin
  SetProp('auto_check', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetIcon: string;
begin
  Result := AsText(GetProp('icon'));
end;

procedure TMenuItem.SetIcon(Value: string);
begin
  SetProp('icon', TJSONString.Create(Value));
end;

function TMenuItem.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TMenuItem.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetDanger: Boolean;
begin
  Result := GetProp('danger').AsBoolean;
end;

procedure TMenuItem.SetDanger(Value: Boolean);
begin
  SetProp('danger', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetHeader: Boolean;
begin
  Result := GetProp('header').AsBoolean;
end;

procedure TMenuItem.SetHeader(Value: Boolean);
begin
  SetProp('header', TJSONBoolean.Create(Value));
end;

function TMenuItem.GetContent: TAylisComponent;
begin
  Result := GetRef('content');
end;

procedure TMenuItem.SetContent(Value: TAylisComponent);
begin
  SetRef('content', Value);
end;

function TMenuItem.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TMenuItem.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TOpenDialog.GetTitle: string;
begin
  Result := AsText(GetProp('title'));
end;

procedure TOpenDialog.SetTitle(Value: string);
begin
  SetProp('title', TJSONString.Create(Value));
end;

function TOpenDialog.GetFilter: string;
begin
  Result := AsText(GetProp('filter'));
end;

procedure TOpenDialog.SetFilter(Value: string);
begin
  SetProp('filter', TJSONString.Create(Value));
end;

function TOpenDialog.GetInitialDir: string;
begin
  Result := AsText(GetProp('initial_dir'));
end;

procedure TOpenDialog.SetInitialDir(Value: string);
begin
  SetProp('initial_dir', TJSONString.Create(Value));
end;

function TOpenDialog.GetFileName: string;
begin
  Result := AsText(GetProp('file_name'));
end;

procedure TOpenDialog.SetFileName(Value: string);
begin
  SetProp('file_name', TJSONString.Create(Value));
end;

function TOpenDialog.GetMultiple: Boolean;
begin
  Result := GetProp('multiple').AsBoolean;
end;

procedure TOpenDialog.SetMultiple(Value: Boolean);
begin
  SetProp('multiple', TJSONBoolean.Create(Value));
end;

function TOpenDialog.GetOnSelect: string;
begin
  Result := AsText(GetProp('on_select'));
end;

procedure TOpenDialog.SetOnSelect(Value: string);
begin
  SetProp('on_select', TJSONString.Create(Value));
end;

function TOpenDialog.GetOnCancel: string;
begin
  Result := AsText(GetProp('on_cancel'));
end;

procedure TOpenDialog.SetOnCancel(Value: string);
begin
  SetProp('on_cancel', TJSONString.Create(Value));
end;

procedure TOpenDialog.Execute;
begin
  Aylis.Send('execute', TJSONObject.Create(['target', Id]));
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

function TSeekBar.GetPosition: Double;
begin
  Result := GetProp('position').AsFloat;
end;

procedure TSeekBar.SetPosition(Value: Double);
begin
  SetProp('position', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetDuration: Double;
begin
  Result := GetProp('duration').AsFloat;
end;

procedure TSeekBar.SetDuration(Value: Double);
begin
  SetProp('duration', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetBuffered: Double;
begin
  Result := GetProp('buffered').AsFloat;
end;

procedure TSeekBar.SetBuffered(Value: Double);
begin
  SetProp('buffered', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetMarkers: TAylisDoubleArray;
begin
  Result := AsNumbers(GetProp('markers'));
end;

procedure TSeekBar.SetMarkers(Value: TAylisDoubleArray);
begin
  SetProp('markers', NumbersText(Value));
end;

function TSeekBar.GetShowTime: Boolean;
begin
  Result := GetProp('show_time').AsBoolean;
end;

procedure TSeekBar.SetShowTime(Value: Boolean);
begin
  SetProp('show_time', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TSeekBar.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TSeekBar.GetOnSeek: string;
begin
  Result := AsText(GetProp('on_seek'));
end;

procedure TSeekBar.SetOnSeek(Value: string);
begin
  SetProp('on_seek', TJSONString.Create(Value));
end;

function TSeekBar.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TSeekBar.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TSeekBar.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TSeekBar.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TSeekBar.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TSeekBar.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TSeekBar.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TSeekBar.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TSeekBar.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TSeekBar.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TSeekBar.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TSeekBar.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TSeekBar.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TSeekBar.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TSeekBar.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TSeekBar.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TSeekBar.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TSeekBar.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TSeekBar.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSeekBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TSeekBar.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TSeekBar.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TSeekBar.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TSeekBar.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TSeekBar.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TSeekBar.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TSeekBar.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TSeekBar.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TSeekBar.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TSeekBar.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TSeekBar.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TSeekBar.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TSeekBar.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TSeekBar.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TSeekBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSeekBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TSeekBar.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TSeekBar.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TSeekBar.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TSeekBar.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TSeekBar.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TSeekBar.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TSeekBar.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TSeekBar.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TSeekBar.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TSeekBar.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TSeekBar.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TSeekBar.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TSeekBar.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TSeekBar.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TSeekBar.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TSeekBar.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TSeekBar.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TSeekBar.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TSeekBar.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TSeekBar.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TSeekBar.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TSeekBar.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TSeekBar.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TSeekBar.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TSeekBar.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TSeekBar.GetStyle: TSeekBarStyle;
begin
  Result := SeekBarStyleOf(GetProp('style'));
end;

procedure TSeekBar.SetStyle(Value: TSeekBarStyle);
begin
  SetProp('style', TJSONString.Create(SeekBarStyleNames[Value]));
end;

function TSeekBar.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TSeekBar.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TSeekBar.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TSeekBar.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TSeekBar.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TSeekBar.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TSeekBar.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TSeekBar.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TSeekBar.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TSeekBar.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TSeekBar.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TSeekBar.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TSeekBar.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TSeekBar.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TSeekBar.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TSeekBar.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TSeekBar.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TSeekBar.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TSeekBar.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TSeekBar.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TSeekBar.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TSeekBar.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TSeekBar.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TSeekBar.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TSeekBar.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TSeekBar.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TSeekBar.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TSeekBar.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TSeekBar.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TSeekBar.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TSeekBar.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TSeekBar.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TSeekBar.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TSeekBar.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TSeekBar.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TSeekBar.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TSeekBar.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TSeekBar.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TSeekBar.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TSeekBar.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TSeekBar.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TSeekBar.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TSeekBar.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TSeekBar.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TSeekBar.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TSeekBar.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TSeekBar.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TSeekBar.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TSeekBar.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TSeekBar.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TSeekBar.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TSeekBar.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TSeekBar.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TSeekBar.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TSeekBar.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TSeekBar.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TSeekBar.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TSeekBar.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TSeekBar.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TSeekBar.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TSeekBar.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TSeekBar.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TSeekBar.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TSeekBar.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TSeekBar.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TSeekBar.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TSeekBar.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TSeekBar.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TSeekBar.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TSeekBar.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TTimer.GetInterval: Double;
begin
  Result := GetProp('interval').AsFloat;
end;

procedure TTimer.SetInterval(Value: Double);
begin
  SetProp('interval', TJSONFloatNumber.Create(Value));
end;

function TTimer.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TTimer.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TTimer.GetPrecision: TTimerPrecision;
begin
  Result := TimerPrecisionOf(GetProp('precision'));
end;

procedure TTimer.SetPrecision(Value: TTimerPrecision);
begin
  SetProp('precision', TJSONString.Create(TimerPrecisionNames[Value]));
end;

function TTimer.GetBackground: Boolean;
begin
  Result := GetProp('background').AsBoolean;
end;

procedure TTimer.SetBackground(Value: Boolean);
begin
  SetProp('background', TJSONBoolean.Create(Value));
end;

function TTimer.GetOneShot: Boolean;
begin
  Result := GetProp('one_shot').AsBoolean;
end;

procedure TTimer.SetOneShot(Value: Boolean);
begin
  SetProp('one_shot', TJSONBoolean.Create(Value));
end;

function TTimer.GetOnTimer: string;
begin
  Result := AsText(GetProp('on_timer'));
end;

procedure TTimer.SetOnTimer(Value: string);
begin
  SetProp('on_timer', TJSONString.Create(Value));
end;

function TTimer.GetDrift: Double;
begin
  Result := GetProp('drift').AsFloat;
end;

procedure TTimer.SetDrift(Value: Double);
begin
  SetProp('drift', TJSONFloatNumber.Create(Value));
end;

procedure TTimer.Restart;
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'restart', 'args', TJSONArray.Create]));
end;

function TTrackBar.GetValue: Double;
begin
  Result := GetProp('value').AsFloat;
end;

procedure TTrackBar.SetValue(Value: Double);
begin
  SetProp('value', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMin: Double;
begin
  Result := GetProp('min').AsFloat;
end;

procedure TTrackBar.SetMin(Value: Double);
begin
  SetProp('min', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMax: Double;
begin
  Result := GetProp('max').AsFloat;
end;

procedure TTrackBar.SetMax(Value: Double);
begin
  SetProp('max', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetFrequency: Double;
begin
  Result := GetProp('frequency').AsFloat;
end;

procedure TTrackBar.SetFrequency(Value: Double);
begin
  SetProp('frequency', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetVertical: Boolean;
begin
  Result := GetProp('vertical').AsBoolean;
end;

procedure TTrackBar.SetVertical(Value: Boolean);
begin
  SetProp('vertical', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetThumbSize: Double;
begin
  Result := GetProp('thumb_size').AsFloat;
end;

procedure TTrackBar.SetThumbSize(Value: Double);
begin
  SetProp('thumb_size', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetTrackSize: Double;
begin
  Result := GetProp('track_size').AsFloat;
end;

procedure TTrackBar.SetTrackSize(Value: Double);
begin
  SetProp('track_size', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetSolidThumb: Boolean;
begin
  Result := GetProp('solid_thumb').AsBoolean;
end;

procedure TTrackBar.SetSolidThumb(Value: Boolean);
begin
  SetProp('solid_thumb', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetLimit: Double;
begin
  Result := GetProp('limit').AsFloat;
end;

procedure TTrackBar.SetLimit(Value: Double);
begin
  SetProp('limit', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TTrackBar.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TTrackBar.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TTrackBar.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TTrackBar.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TTrackBar.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TTrackBar.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TTrackBar.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TTrackBar.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TTrackBar.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TTrackBar.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TTrackBar.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TTrackBar.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TTrackBar.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TTrackBar.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TTrackBar.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TTrackBar.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TTrackBar.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TTrackBar.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TTrackBar.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TTrackBar.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTrackBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TTrackBar.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TTrackBar.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TTrackBar.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TTrackBar.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TTrackBar.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TTrackBar.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TTrackBar.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TTrackBar.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TTrackBar.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TTrackBar.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TTrackBar.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TTrackBar.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TTrackBar.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TTrackBar.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TTrackBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTrackBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TTrackBar.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TTrackBar.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TTrackBar.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TTrackBar.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TTrackBar.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TTrackBar.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TTrackBar.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TTrackBar.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TTrackBar.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TTrackBar.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TTrackBar.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TTrackBar.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TTrackBar.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TTrackBar.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TTrackBar.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TTrackBar.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TTrackBar.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TTrackBar.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TTrackBar.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TTrackBar.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TTrackBar.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TTrackBar.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TTrackBar.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TTrackBar.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TTrackBar.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TTrackBar.GetStyle: TTrackBarStyle;
begin
  Result := TrackBarStyleOf(GetProp('style'));
end;

procedure TTrackBar.SetStyle(Value: TTrackBarStyle);
begin
  SetProp('style', TJSONString.Create(TrackBarStyleNames[Value]));
end;

function TTrackBar.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TTrackBar.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TTrackBar.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TTrackBar.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TTrackBar.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TTrackBar.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TTrackBar.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TTrackBar.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TTrackBar.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TTrackBar.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TTrackBar.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TTrackBar.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TTrackBar.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TTrackBar.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TTrackBar.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TTrackBar.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TTrackBar.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TTrackBar.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TTrackBar.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TTrackBar.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TTrackBar.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TTrackBar.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TTrackBar.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TTrackBar.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TTrackBar.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TTrackBar.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TTrackBar.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TTrackBar.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TTrackBar.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TTrackBar.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TTrackBar.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TTrackBar.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TTrackBar.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TTrackBar.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TTrackBar.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TTrackBar.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TTrackBar.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TTrackBar.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TTrackBar.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TTrackBar.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TTrackBar.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TTrackBar.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TTrackBar.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TTrackBar.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TTrackBar.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TTrackBar.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TTrackBar.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TTrackBar.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TTrackBar.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TTrackBar.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TTrackBar.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TTrackBar.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TTrackBar.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TTrackBar.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TTrackBar.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TTrackBar.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TTrackBar.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TTrackBar.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TTrackBar.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TTrackBar.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TTrackBar.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TTrackBar.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TTrackBar.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TTrackBar.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TTrackBar.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TTrackBar.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TTrackBar.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TTrackBar.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TTrackBar.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TTrackBar.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TVideoPlayer.GetFile: string;
begin
  Result := AsText(GetProp('file'));
end;

procedure TVideoPlayer.SetFile(Value: string);
begin
  SetProp('file', TJSONString.Create(Value));
end;

function TVideoPlayer.GetAutoplay: Boolean;
begin
  Result := GetProp('autoplay').AsBoolean;
end;

procedure TVideoPlayer.SetAutoplay(Value: Boolean);
begin
  SetProp('autoplay', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetLooping: Boolean;
begin
  Result := GetProp('looping').AsBoolean;
end;

procedure TVideoPlayer.SetLooping(Value: Boolean);
begin
  SetProp('looping', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetVolume: Double;
begin
  Result := GetProp('volume').AsFloat;
end;

procedure TVideoPlayer.SetVolume(Value: Double);
begin
  SetProp('volume', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetMuted: Boolean;
begin
  Result := GetProp('muted').AsBoolean;
end;

procedure TVideoPlayer.SetMuted(Value: Boolean);
begin
  SetProp('muted', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetFit: TImageFit;
begin
  Result := ImageFitOf(GetProp('fit'));
end;

procedure TVideoPlayer.SetFit(Value: TImageFit);
begin
  SetProp('fit', TJSONString.Create(ImageFitNames[Value]));
end;

function TVideoPlayer.GetRadius: Double;
begin
  Result := GetProp('radius').AsFloat;
end;

procedure TVideoPlayer.SetRadius(Value: Double);
begin
  SetProp('radius', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetRate: Double;
begin
  Result := GetProp('rate').AsFloat;
end;

procedure TVideoPlayer.SetRate(Value: Double);
begin
  SetProp('rate', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetOutput: TOutputSpecArray;
begin
  Result := OutputSpecListFromText(GetProp('output'));
end;

procedure TVideoPlayer.SetOutput(Value: TOutputSpecArray);
begin
  SetProp('output', OutputSpecListText(Value));
end;

function TVideoPlayer.GetShowVideo: Boolean;
begin
  Result := GetProp('show_video').AsBoolean;
end;

procedure TVideoPlayer.SetShowVideo(Value: Boolean);
begin
  SetProp('show_video', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetFramesAhead: Int64;
begin
  Result := GetProp('frames_ahead').AsInt64;
end;

procedure TVideoPlayer.SetFramesAhead(Value: Int64);
begin
  SetProp('frames_ahead', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetOnReady: string;
begin
  Result := AsText(GetProp('on_ready'));
end;

procedure TVideoPlayer.SetOnReady(Value: string);
begin
  SetProp('on_ready', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnEnd: string;
begin
  Result := AsText(GetProp('on_end'));
end;

procedure TVideoPlayer.SetOnEnd(Value: string);
begin
  SetProp('on_end', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnError: string;
begin
  Result := AsText(GetProp('on_error'));
end;

procedure TVideoPlayer.SetOnError(Value: string);
begin
  SetProp('on_error', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnFrame: string;
begin
  Result := AsText(GetProp('on_frame'));
end;

procedure TVideoPlayer.SetOnFrame(Value: string);
begin
  SetProp('on_frame', TJSONString.Create(Value));
end;

function TVideoPlayer.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TVideoPlayer.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TVideoPlayer.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TVideoPlayer.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TVideoPlayer.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TVideoPlayer.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TVideoPlayer.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TVideoPlayer.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TVideoPlayer.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TVideoPlayer.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TVideoPlayer.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TVideoPlayer.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TVideoPlayer.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TVideoPlayer.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TVideoPlayer.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TVideoPlayer.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TVideoPlayer.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TVideoPlayer.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TVideoPlayer.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TVideoPlayer.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TVideoPlayer.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TVideoPlayer.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TVideoPlayer.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TVideoPlayer.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TVideoPlayer.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TVideoPlayer.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TVideoPlayer.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TVideoPlayer.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TVideoPlayer.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TVideoPlayer.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TVideoPlayer.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TVideoPlayer.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TVideoPlayer.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TVideoPlayer.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TVideoPlayer.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TVideoPlayer.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TVideoPlayer.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TVideoPlayer.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TVideoPlayer.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TVideoPlayer.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TVideoPlayer.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TVideoPlayer.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TVideoPlayer.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TVideoPlayer.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TVideoPlayer.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TVideoPlayer.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TVideoPlayer.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TVideoPlayer.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TVideoPlayer.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TVideoPlayer.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TVideoPlayer.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TVideoPlayer.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TVideoPlayer.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TVideoPlayer.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TVideoPlayer.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TVideoPlayer.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TVideoPlayer.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TVideoPlayer.GetStyle: TVideoPlayerStyle;
begin
  Result := VideoPlayerStyleOf(GetProp('style'));
end;

procedure TVideoPlayer.SetStyle(Value: TVideoPlayerStyle);
begin
  SetProp('style', TJSONString.Create(VideoPlayerStyleNames[Value]));
end;

function TVideoPlayer.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TVideoPlayer.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TVideoPlayer.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TVideoPlayer.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TVideoPlayer.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TVideoPlayer.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TVideoPlayer.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TVideoPlayer.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TVideoPlayer.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TVideoPlayer.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TVideoPlayer.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TVideoPlayer.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TVideoPlayer.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TVideoPlayer.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TVideoPlayer.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TVideoPlayer.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TVideoPlayer.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TVideoPlayer.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TVideoPlayer.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TVideoPlayer.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TVideoPlayer.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TVideoPlayer.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TVideoPlayer.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TVideoPlayer.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TVideoPlayer.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TVideoPlayer.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TVideoPlayer.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TVideoPlayer.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TVideoPlayer.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TVideoPlayer.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TVideoPlayer.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TVideoPlayer.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TVideoPlayer.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TVideoPlayer.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TVideoPlayer.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TVideoPlayer.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TVideoPlayer.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TVideoPlayer.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TVideoPlayer.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TVideoPlayer.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TVideoPlayer.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TVideoPlayer.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TVideoPlayer.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TVideoPlayer.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TVideoPlayer.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TVideoPlayer.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TVideoPlayer.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TVideoPlayer.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TVideoPlayer.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TVideoPlayer.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TVideoPlayer.Play;
begin
  Aylis.Send('media', TJSONObject.Create(['target', Id, 'op', 'play']));
end;

procedure TVideoPlayer.Pause;
begin
  Aylis.Send('media', TJSONObject.Create(['target', Id, 'op', 'pause']));
end;

procedure TVideoPlayer.Stop;
begin
  Aylis.Send('media', TJSONObject.Create(['target', Id, 'op', 'stop']));
end;

procedure TVideoPlayer.Seek(ASeconds: Double);
begin
  Aylis.Send('media', TJSONObject.Create(['target', Id, 'op', 'seek', 'seconds', ASeconds]));
end;

function TVideoPlayer.Position: Double;
begin
  Result := Aylis.Call('media', TJSONObject.Create(['target', Id, 'op', 'position'])).AsFloat;
end;

function TVideoPlayer.Duration: Double;
begin
  Result := Aylis.Call('media', TJSONObject.Create(['target', Id, 'op', 'duration'])).AsFloat;
end;

procedure TFrmPlayerBase.BindComponents(List: TJSONArray);
begin
  mnuPrincipal := TMainMenu(Resolve(List, 'mnuPrincipal', 'MainMenu', TMainMenu));
  mnuArquivo := TMenuItem(Resolve(List, 'mnuArquivo', 'MenuItem', TMenuItem));
  mnuAbrir := TMenuItem(Resolve(List, 'mnuAbrir', 'MenuItem', TMenuItem));
  mnuLimparFila := TMenuItem(Resolve(List, 'mnuLimparFila', 'MenuItem', TMenuItem));
  mnuSep1 := TMenuItem(Resolve(List, 'mnuSep1', 'MenuItem', TMenuItem));
  mnuSair := TMenuItem(Resolve(List, 'mnuSair', 'MenuItem', TMenuItem));
  mnuReproduzir := TMenuItem(Resolve(List, 'mnuReproduzir', 'MenuItem', TMenuItem));
  mnuReproduzirPausar := TMenuItem(Resolve(List, 'mnuReproduzirPausar', 'MenuItem', TMenuItem));
  mnuParar := TMenuItem(Resolve(List, 'mnuParar', 'MenuItem', TMenuItem));
  mnuAnterior := TMenuItem(Resolve(List, 'mnuAnterior', 'MenuItem', TMenuItem));
  mnuProxima := TMenuItem(Resolve(List, 'mnuProxima', 'MenuItem', TMenuItem));
  mnuSep2 := TMenuItem(Resolve(List, 'mnuSep2', 'MenuItem', TMenuItem));
  mnuEmbaralhar := TMenuItem(Resolve(List, 'mnuEmbaralhar', 'MenuItem', TMenuItem));
  mnuRepetir := TMenuItem(Resolve(List, 'mnuRepetir', 'MenuItem', TMenuItem));
  mnuExibir := TMenuItem(Resolve(List, 'mnuExibir', 'MenuItem', TMenuItem));
  mnuAgora := TMenuItem(Resolve(List, 'mnuAgora', 'MenuItem', TMenuItem));
  mnuFila := TMenuItem(Resolve(List, 'mnuFila', 'MenuItem', TMenuItem));
  pnlControles := TPanel(Resolve(List, 'pnlControles', 'Panel', TPanel));
  lytBarra := TLayout(Resolve(List, 'lytBarra', 'Layout', TLayout));
  lblTempo := TLabel(Resolve(List, 'lblTempo', 'Label', TLabel));
  lblDuracao := TLabel(Resolve(List, 'lblDuracao', 'Label', TLabel));
  sbPosicao := TSeekBar(Resolve(List, 'sbPosicao', 'SeekBar', TSeekBar));
  lytInfo := TLayout(Resolve(List, 'lytInfo', 'Layout', TLayout));
  icoCapa := TIcon(Resolve(List, 'icoCapa', 'Icon', TIcon));
  lytTextos := TLayout(Resolve(List, 'lytTextos', 'Layout', TLayout));
  lblTitulo := TLabel(Resolve(List, 'lblTitulo', 'Label', TLabel));
  lblDetalhe := TLabel(Resolve(List, 'lblDetalhe', 'Label', TLabel));
  lytVolume := TLayout(Resolve(List, 'lytVolume', 'Layout', TLayout));
  btnMudo := TButton(Resolve(List, 'btnMudo', 'Button', TButton));
  tbVolume := TTrackBar(Resolve(List, 'tbVolume', 'TrackBar', TTrackBar));
  lytBotoes := TLayout(Resolve(List, 'lytBotoes', 'Layout', TLayout));
  lytBotoesCentro := TLayout(Resolve(List, 'lytBotoesCentro', 'Layout', TLayout));
  btnEmbaralhar := TButton(Resolve(List, 'btnEmbaralhar', 'Button', TButton));
  btnAnterior := TButton(Resolve(List, 'btnAnterior', 'Button', TButton));
  btnReproduzir := TButton(Resolve(List, 'btnReproduzir', 'Button', TButton));
  btnProxima := TButton(Resolve(List, 'btnProxima', 'Button', TButton));
  btnRepetir := TButton(Resolve(List, 'btnRepetir', 'Button', TButton));
  lytNavegacao := TLayout(Resolve(List, 'lytNavegacao', 'Layout', TLayout));
  btnAgora := TButton(Resolve(List, 'btnAgora', 'Button', TButton));
  btnFila := TButton(Resolve(List, 'btnFila', 'Button', TButton));
  divNavegacao := TDivider(Resolve(List, 'divNavegacao', 'Divider', TDivider));
  btnAbrirArquivos := TButton(Resolve(List, 'btnAbrirArquivos', 'Button', TButton));
  pnlAgora := TPanel(Resolve(List, 'pnlAgora', 'Panel', TPanel));
  vpVideo := TVideoPlayer(Resolve(List, 'vpVideo', 'VideoPlayer', TVideoPlayer));
  pnlFila := TPanel(Resolve(List, 'pnlFila', 'Panel', TPanel));
  lblFila := TLabel(Resolve(List, 'lblFila', 'Label', TLabel));
  lytFilaBotoes := TLayout(Resolve(List, 'lytFilaBotoes', 'Layout', TLayout));
  btnAdicionar := TButton(Resolve(List, 'btnAdicionar', 'Button', TButton));
  btnLimparFila := TButton(Resolve(List, 'btnLimparFila', 'Button', TButton));
  lvFila := TListView(Resolve(List, 'lvFila', 'ListView', TListView));
  dlgAbrir := TOpenDialog(Resolve(List, 'dlgAbrir', 'OpenDialog', TOpenDialog));
  tmrPosicao := TTimer(Resolve(List, 'tmrPosicao', 'Timer', TTimer));
end;

function TFrmPlayerBase.FindButton(const Nome: string): TButton;
begin
  Result := nil;
  if Nome = 'btnMudo' then
    Result := btnMudo;
  if Nome = 'btnEmbaralhar' then
    Result := btnEmbaralhar;
  if Nome = 'btnAnterior' then
    Result := btnAnterior;
  if Nome = 'btnReproduzir' then
    Result := btnReproduzir;
  if Nome = 'btnProxima' then
    Result := btnProxima;
  if Nome = 'btnRepetir' then
    Result := btnRepetir;
  if Nome = 'btnAgora' then
    Result := btnAgora;
  if Nome = 'btnFila' then
    Result := btnFila;
  if Nome = 'btnAbrirArquivos' then
    Result := btnAbrirArquivos;
  if Nome = 'btnAdicionar' then
    Result := btnAdicionar;
  if Nome = 'btnLimparFila' then
    Result := btnLimparFila;
end;

function TFrmPlayerBase.SenderAsButton(Sender: TAylisComponent): TButton;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindButton(Sender.Name);
end;

function TFrmPlayerBase.SenderAsButton(Event: TJSONObject): TButton;
begin
  Result := FindButton(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindDivider(const Nome: string): TDivider;
begin
  Result := nil;
  if Nome = 'divNavegacao' then
    Result := divNavegacao;
end;

function TFrmPlayerBase.SenderAsDivider(Sender: TAylisComponent): TDivider;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindDivider(Sender.Name);
end;

function TFrmPlayerBase.SenderAsDivider(Event: TJSONObject): TDivider;
begin
  Result := FindDivider(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindIcon(const Nome: string): TIcon;
begin
  Result := nil;
  if Nome = 'icoCapa' then
    Result := icoCapa;
end;

function TFrmPlayerBase.SenderAsIcon(Sender: TAylisComponent): TIcon;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindIcon(Sender.Name);
end;

function TFrmPlayerBase.SenderAsIcon(Event: TJSONObject): TIcon;
begin
  Result := FindIcon(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblTempo' then
    Result := lblTempo;
  if Nome = 'lblDuracao' then
    Result := lblDuracao;
  if Nome = 'lblTitulo' then
    Result := lblTitulo;
  if Nome = 'lblDetalhe' then
    Result := lblDetalhe;
  if Nome = 'lblFila' then
    Result := lblFila;
end;

function TFrmPlayerBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmPlayerBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindLayout(const Nome: string): TLayout;
begin
  Result := nil;
  if Nome = 'lytBarra' then
    Result := lytBarra;
  if Nome = 'lytInfo' then
    Result := lytInfo;
  if Nome = 'lytTextos' then
    Result := lytTextos;
  if Nome = 'lytVolume' then
    Result := lytVolume;
  if Nome = 'lytBotoes' then
    Result := lytBotoes;
  if Nome = 'lytBotoesCentro' then
    Result := lytBotoesCentro;
  if Nome = 'lytNavegacao' then
    Result := lytNavegacao;
  if Nome = 'lytFilaBotoes' then
    Result := lytFilaBotoes;
end;

function TFrmPlayerBase.SenderAsLayout(Sender: TAylisComponent): TLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLayout(Sender.Name);
end;

function TFrmPlayerBase.SenderAsLayout(Event: TJSONObject): TLayout;
begin
  Result := FindLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindListView(const Nome: string): TListView;
begin
  Result := nil;
  if Nome = 'lvFila' then
    Result := lvFila;
end;

function TFrmPlayerBase.SenderAsListView(Sender: TAylisComponent): TListView;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindListView(Sender.Name);
end;

function TFrmPlayerBase.SenderAsListView(Event: TJSONObject): TListView;
begin
  Result := FindListView(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindMainMenu(const Nome: string): TMainMenu;
begin
  Result := nil;
  if Nome = 'mnuPrincipal' then
    Result := mnuPrincipal;
end;

function TFrmPlayerBase.SenderAsMainMenu(Sender: TAylisComponent): TMainMenu;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindMainMenu(Sender.Name);
end;

function TFrmPlayerBase.SenderAsMainMenu(Event: TJSONObject): TMainMenu;
begin
  Result := FindMainMenu(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindMenuItem(const Nome: string): TMenuItem;
begin
  Result := nil;
  if Nome = 'mnuArquivo' then
    Result := mnuArquivo;
  if Nome = 'mnuAbrir' then
    Result := mnuAbrir;
  if Nome = 'mnuLimparFila' then
    Result := mnuLimparFila;
  if Nome = 'mnuSep1' then
    Result := mnuSep1;
  if Nome = 'mnuSair' then
    Result := mnuSair;
  if Nome = 'mnuReproduzir' then
    Result := mnuReproduzir;
  if Nome = 'mnuReproduzirPausar' then
    Result := mnuReproduzirPausar;
  if Nome = 'mnuParar' then
    Result := mnuParar;
  if Nome = 'mnuAnterior' then
    Result := mnuAnterior;
  if Nome = 'mnuProxima' then
    Result := mnuProxima;
  if Nome = 'mnuSep2' then
    Result := mnuSep2;
  if Nome = 'mnuEmbaralhar' then
    Result := mnuEmbaralhar;
  if Nome = 'mnuRepetir' then
    Result := mnuRepetir;
  if Nome = 'mnuExibir' then
    Result := mnuExibir;
  if Nome = 'mnuAgora' then
    Result := mnuAgora;
  if Nome = 'mnuFila' then
    Result := mnuFila;
end;

function TFrmPlayerBase.SenderAsMenuItem(Sender: TAylisComponent): TMenuItem;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindMenuItem(Sender.Name);
end;

function TFrmPlayerBase.SenderAsMenuItem(Event: TJSONObject): TMenuItem;
begin
  Result := FindMenuItem(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindOpenDialog(const Nome: string): TOpenDialog;
begin
  Result := nil;
  if Nome = 'dlgAbrir' then
    Result := dlgAbrir;
end;

function TFrmPlayerBase.SenderAsOpenDialog(Sender: TAylisComponent): TOpenDialog;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindOpenDialog(Sender.Name);
end;

function TFrmPlayerBase.SenderAsOpenDialog(Event: TJSONObject): TOpenDialog;
begin
  Result := FindOpenDialog(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindPanel(const Nome: string): TPanel;
begin
  Result := nil;
  if Nome = 'pnlControles' then
    Result := pnlControles;
  if Nome = 'pnlAgora' then
    Result := pnlAgora;
  if Nome = 'pnlFila' then
    Result := pnlFila;
end;

function TFrmPlayerBase.SenderAsPanel(Sender: TAylisComponent): TPanel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindPanel(Sender.Name);
end;

function TFrmPlayerBase.SenderAsPanel(Event: TJSONObject): TPanel;
begin
  Result := FindPanel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindSeekBar(const Nome: string): TSeekBar;
begin
  Result := nil;
  if Nome = 'sbPosicao' then
    Result := sbPosicao;
end;

function TFrmPlayerBase.SenderAsSeekBar(Sender: TAylisComponent): TSeekBar;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindSeekBar(Sender.Name);
end;

function TFrmPlayerBase.SenderAsSeekBar(Event: TJSONObject): TSeekBar;
begin
  Result := FindSeekBar(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindTimer(const Nome: string): TTimer;
begin
  Result := nil;
  if Nome = 'tmrPosicao' then
    Result := tmrPosicao;
end;

function TFrmPlayerBase.SenderAsTimer(Sender: TAylisComponent): TTimer;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindTimer(Sender.Name);
end;

function TFrmPlayerBase.SenderAsTimer(Event: TJSONObject): TTimer;
begin
  Result := FindTimer(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindTrackBar(const Nome: string): TTrackBar;
begin
  Result := nil;
  if Nome = 'tbVolume' then
    Result := tbVolume;
end;

function TFrmPlayerBase.SenderAsTrackBar(Sender: TAylisComponent): TTrackBar;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindTrackBar(Sender.Name);
end;

function TFrmPlayerBase.SenderAsTrackBar(Event: TJSONObject): TTrackBar;
begin
  Result := FindTrackBar(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPlayerBase.FindVideoPlayer(const Nome: string): TVideoPlayer;
begin
  Result := nil;
  if Nome = 'vpVideo' then
    Result := vpVideo;
end;

function TFrmPlayerBase.SenderAsVideoPlayer(Sender: TAylisComponent): TVideoPlayer;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindVideoPlayer(Sender.Name);
end;

function TFrmPlayerBase.SenderAsVideoPlayer(Event: TJSONObject): TVideoPlayer;
begin
  Result := FindVideoPlayer(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

initialization
  AylisFormat := DefaultFormatSettings;
  AylisFormat.DecimalSeparator := '.';
end.
